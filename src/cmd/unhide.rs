use crate::meta::Metadata;
use crate::print;
use crate::repo::RepositoryUtils;
use anyhow::bail;
use anyhow::Result;
use git2::Commit;
use git2::Repository;
use immargs::ImmArgs;
use std::path::Path;

#[derive(ImmArgs)]
pub struct Args {
    #[cmdline(choice = "0", help = "Unhide all commits")]
    all: Option<()>,

    #[cmdline(positional, choice = "0")]
    revspec: Option<Vec<String>>,
}

fn unhide(repo: &Repository, meta: &Metadata, commits: &[Commit]) -> Result<()> {
    if commits.is_empty() {
        bail!("nothing to unhide");
    }

    let name = repo.head_name()?;

    for commit in commits {
        let mut branch = meta.branches.acquire(&name);

        if !branch.hidden.remove(commit.id()) {
            bail!("cannot unhide non-hidden commit");
        }

        branch.popped.add_top(commit.id());
        meta.branches.release(branch);

        print::commit(commit, "unhide")?;

        meta.commit(repo, "unhide")?;
    }

    Ok(())
}

fn unhide_revspecs(repo: &Repository, meta: &Metadata, revspecs: &[String]) -> Result<()> {
    let commits = repo.find_commits_by_revspecs(revspecs)?;
    unhide(repo, meta, &commits)
}

fn unhide_all(repo: &Repository, meta: &Metadata) -> Result<()> {
    let name = repo.head_name()?;
    let branch = meta.branches.acquire(&name);
    let oids = branch.hidden.all_reversed();
    let commits = repo.find_commits(&oids)?;
    meta.branches.release(branch);
    unhide(repo, meta, &commits)
}

pub fn main(path: &Path, args: Args) -> Result<()> {
    let repo = Repository::discover(path)?;
    let meta = Metadata::open(&repo)?;

    if let Some(revspec) = args.revspec {
        unhide_revspecs(&repo, &meta, &revspec)
    } else if args.all.is_some() {
        unhide_all(&repo, &meta)
    } else {
        panic!();
    }
}
