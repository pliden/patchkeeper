use crate::meta::Metadata;
use crate::print;
use crate::repo::RepositoryUtils;
use anyhow::Result;
use anyhow::bail;
use git2::Commit;
use git2::Repository;
use immargs::immargs;
use std::path::Path;

immargs! {
    UnhideArgs,
    -a --all              ! "unhide all commits",
    -h --help               "print help message",
    [<revspec>...] String !,
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
    if revspecs.is_empty() {
        bail!("nothing to unhide");
    }

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

pub fn main(path: &Path, args: UnhideArgs) -> Result<()> {
    let repo = Repository::discover(path)?;
    let meta = Metadata::open(&repo)?;

    if args.all {
        unhide_all(&repo, &meta)
    } else {
        unhide_revspecs(&repo, &meta, &args.revspec)
    }
}
