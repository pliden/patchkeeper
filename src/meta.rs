use crate::safe_println;
use anyhow::bail;
use anyhow::Result;
use git2::Commit;
use git2::Oid;
use git2::Repository;
use itertools::chain;
use itertools::Itertools;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::str;

mod branch;
mod branches;
mod patches;
mod properties;
mod serialize;
mod source;

const MAGIC: &str = "PatchKeeper";
const REFERENCE: &str = "refs/patchkeeper_0";

pub const REVISION: &str = "revision";
pub const HIDDEN: &str = "hidden";
pub const UNDO: &str = "undo";

#[derive(Debug, Default)]
pub struct Properties(RefCell<HashMap<String, Option<String>>>);

#[derive(Debug, Default)]
pub struct Patches(VecDeque<Oid>);

#[derive(Debug, Default)]
struct Source(RefCell<Option<Oid>>);

#[derive(Debug, Default)]
pub struct Branch {
    name: String,
    pub properties: Properties,
    pub hidden: Patches,
    pub popped: Patches,
    pub pushed: Patches,
}

#[derive(Debug, Default)]
pub struct Branches(RefCell<HashMap<String, Branch>>);

#[derive(Debug, Default)]
pub struct Metadata {
    pub properties: Properties,
    pub branches: Branches,
    source: Source,
}

impl Metadata {
    pub fn open(repo: &Repository) -> Result<Self> {
        let commit = match repo.find_reference(REFERENCE) {
            Ok(reference) => reference.peel_to_commit()?,
            _ => return Ok(Self::default()),
        };

        let meta = Self::from_commit(&commit)?;
        meta.verify(repo)?;
        Ok(meta)
    }

    fn from_commit(commit: &Commit) -> Result<Self> {
        let summary = commit.summary().unwrap_or_default();
        let body = commit.body().unwrap_or_default();

        if summary != MAGIC {
            bail!("metadata format not recognized");
        }

        let meta = body.parse::<Metadata>()?;
        meta.source.set_id(commit.id());
        Ok(meta)
    }

    fn verify(&self, repo: &Repository) -> Result<()> {
        let Branches(branches) = &self.branches;
        for (name, branch) in branches.borrow().iter() {
            let mut next_commit = match repo.resolve_reference_from_short_name(name) {
                Ok(reference) if reference.is_branch() => Some(reference.peel_to_commit()?),
                _ => {
                    safe_println!("warning: metadata references to non-existing branch '{name}'");
                    continue;
                }
            };

            for id in branch.pushed.all() {
                let commit = match next_commit {
                    Some(commit) if id == commit.id() => commit,
                    _ => bail!("metadata out of sync (use 'pk fsck' to repair)"),
                };

                next_commit = if commit.parent_count() == 1 {
                    Some(commit.parent(0)?)
                } else {
                    None
                };
            }
        }

        Ok(())
    }

    fn parents<'a>(&self, repo: &'a Repository) -> Result<Vec<Commit<'a>>> {
        let Branches(branches) = &self.branches;
        let mut commits = vec![];

        for (_, branch) in branches.borrow().iter().sorted_by_key(|(name, _)| *name) {
            for oid in chain!(
                branch.hidden.all(),
                branch.popped.all(),
                branch.pushed.all()
            ) {
                let commit = repo.find_commit(oid)?;
                commits.push(commit);
            }
        }

        if let Some(oid) = self.properties.get(UNDO)? {
            let commit = repo.find_commit(oid)?;
            commits.push(commit);
        };

        Ok(commits)
    }

    fn commit_inner(&self, repo: &Repository, log_message: &str) -> Result<()> {
        let revision = self.properties.get(REVISION)?.unwrap_or(0u64);
        self.properties.set(REVISION, revision + 1);

        let update_ref = None;
        let author = repo.signature()?;
        let tree_id = repo.treebuilder(None)?.write()?;
        let tree = repo.find_tree(tree_id)?;
        let parents = self.parents(repo)?;
        let parents = parents.iter().collect::<Vec<_>>();
        let message = format!("{MAGIC}\n\n{self}");
        let oid = repo.commit(update_ref, &author, &author, &message, &tree, &parents)?;

        let reference = REFERENCE;
        repo.reference_ensure_log(reference)?;

        // FIXME! Is there an issue with undo here?
        if let Some(current_oid) = self.source.id() {
            repo.reference_matching(reference, oid, true, current_oid, log_message)?;
        } else {
            repo.reference(reference, oid, false, log_message)?;
        }

        self.source.set_id(oid);

        Ok(())
    }

    pub fn commit(&self, repo: &Repository, log_message: &str) -> Result<()> {
        self.properties.remove(UNDO);
        self.commit_inner(repo, log_message)
    }

    pub fn commit_with_undo(&self, repo: &Repository, log_message: &str) -> Result<()> {
        if let Some(oid) = self.source.id() {
            self.properties.set(UNDO, oid);
        }
        self.commit_inner(repo, log_message)
    }

    pub fn undo(&self, repo: &Repository) -> Result<Option<Metadata>> {
        let commit = match self.properties.get(UNDO)? {
            Some(oid) => repo.find_commit(oid)?,
            _ => return Ok(None),
        };

        let meta = Metadata::from_commit(&commit)?;

        let revision = meta.properties.get(REVISION)?.unwrap_or(0u64);
        meta.properties.set(REVISION, revision + 1);

        Ok(Some(meta))
    }
}
