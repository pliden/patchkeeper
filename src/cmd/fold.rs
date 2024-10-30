use crate::cmd;
use crate::meta::Metadata;
use crate::print;
use crate::repo::RepositoryUtils;
use anyhow::bail;
use anyhow::Result;
use git2::Commit;
use git2::Repository;
use gumdrop::Options;
use std::path::Path;
use std::str;

#[derive(Options)]
pub struct Args {
    #[options(help = "Fold next commit")]
    next: bool,

    #[options(help = "Print help message")]
    help: bool,

    #[options(free, help = "[<revspec>...]")]
    revspecs: Vec<String>,
}

fn fold(repo: &Repository, meta: &Metadata, commits: &[Commit]) -> Result<()> {
    if commits.is_empty() {
        bail!("nothing to fold");
    }

    let name = repo.head_name()?;

    for commit in commits {
        let mut branch = meta.branches.acquire(&name);

        if !branch.popped.remove(commit.id()) {
            bail!("cannot fold non-popped commit");
        }

        print::commit_action(commit, "fold")?;
        repo.cherrypick(commit, None)?;

        let mut index = repo.index()?;
        if index.has_conflicts() {
            meta.branches.release(branch);
            meta.commit_with_undo(repo, "fold")?;
            print::conflicts(&index)?;
        } else {
            let oid = repo.amend_head(&mut index)?;
            branch.pushed.replace_top(oid);
            meta.branches.release(branch);
            meta.commit(repo, "fold")?;
        }
    }

    Ok(())
}

fn fold_revspecs(repo: &Repository, meta: &Metadata, revspecs: &[String]) -> Result<()> {
    let commits = repo.find_commits_by_revspecs(revspecs)?;
    fold(repo, meta, &commits)
}

fn fold_next(repo: &Repository, meta: &Metadata) -> Result<()> {
    let name = repo.head_name()?;
    let branch = meta.branches.acquire(&name);
    let oids = branch.popped.bottom_as_vec();
    let commits = repo.find_commits(&oids)?;
    meta.branches.release(branch);
    fold(repo, meta, &commits)
}

pub fn main(path: &Path, args: Args) -> Result<()> {
    cmd::conflicting_options(&[("-n", args.next), ("<revspec>", !args.revspecs.is_empty())])?;

    let repo = Repository::discover(path)?;
    let meta = Metadata::open(&repo)?;

    repo.ensure_no_unresolved()?;
    repo.ensure_no_unrefreshed()?;

    if !args.revspecs.is_empty() {
        fold_revspecs(&repo, &meta, &args.revspecs)
    } else {
        fold_next(&repo, &meta)
    }
}
