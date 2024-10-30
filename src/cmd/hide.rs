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

#[derive(Options)]
pub struct Args {
    #[options(help = "Hide all commits")]
    all: bool,

    #[options(help = "Hide next commit")]
    next: bool,

    #[options(help = "Print help message")]
    help: bool,

    #[options(free, help = "[<revspec>...]")]
    revspecs: Vec<String>,
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

        print::commit_action(commit, "hide")?;

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
    let oids = branch.popped.bottom_as_vec();
    let commits = repo.find_commits(&oids)?;
    meta.branches.release(branch);
    hide(repo, meta, &commits)
}

pub fn main(path: &Path, args: Args) -> Result<()> {
    cmd::missing_or_conflicting_options(&[
        ("-a", args.all),
        ("-n", args.next),
        ("<revspec>", !args.revspecs.is_empty()),
    ])?;

    let repo = Repository::discover(path)?;
    let meta = Metadata::open(&repo)?;

    if !args.revspecs.is_empty() {
        hide_revspecs(&repo, &meta, &args.revspecs)
    } else if args.all {
        hide_all(&repo, &meta)
    } else {
        hide_next(&repo, &meta)
    }
}
