use crate::meta::Metadata;
use crate::print;
use crate::repo::RepositoryUtils;
use anyhow::Result;
use anyhow::bail;
use git2::Commit;
use git2::Repository;
use immargs::args;
use std::path::Path;

args! {
    HideArgs,
    -a --all              ! "hide all commits",
    -h --help               "print help message",
    [<revspec>...] String !,

}

fn hide(repo: &Repository, meta: &Metadata, commits: &[Commit]) -> Result<()> {
    if commits.is_empty() {
        bail!("nothing to hide");
    }

    let name = repo.head_name()?;

    for commit in commits {
        let mut branch = meta.branches.acquire(&name);

        if !branch.popped.remove(commit.id()) {
            bail!("cannot hide non-popped commit");
        }

        branch.hidden.add_top(commit.id());
        meta.branches.release(branch);

        print::commit(commit, "hide")?;

        meta.commit(repo, "hide")?;
    }

    Ok(())
}

fn hide_revspecs(repo: &Repository, meta: &Metadata, revspecs: &[String]) -> Result<()> {
    let commits = repo.find_commits_by_revspecs(revspecs)?;
    hide(repo, meta, &commits)
}

fn hide_all(repo: &Repository, meta: &Metadata) -> Result<()> {
    let name = repo.head_name()?;
    let branch = meta.branches.acquire(&name);
    let oids = branch.popped.all_reversed();
    let commits = repo.find_commits(&oids)?;
    meta.branches.release(branch);
    hide(repo, meta, &commits)
}

fn hide_next(repo: &Repository, meta: &Metadata) -> Result<()> {
    let name = repo.head_name()?;
    let branch = meta.branches.acquire(&name);
    let oids = branch.popped.bottom_vec();
    let commits = repo.find_commits(&oids)?;
    meta.branches.release(branch);
    hide(repo, meta, &commits)
}

pub fn main(path: &Path, args: HideArgs) -> Result<()> {
    let repo = Repository::discover(path)?;
    let meta = Metadata::open(&repo)?;

    if args.all {
        hide_all(&repo, &meta)
    } else if !args.revspec.is_empty() {
        hide_revspecs(&repo, &meta, &args.revspec)
    } else {
        hide_next(&repo, &meta)
    }
}
