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
    #[options(help = "Unhide all commits")]
    all: bool,

    #[options(help = "Print help message")]
    help: bool,

    #[options(free, help = "[<revspec>...]")]
    revspecs: Vec<String>,
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

        print::commit_action(commit, "unhide")?;

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
    cmd::missing_or_conflicting_options(&[
        ("-a", args.all),
        ("<revspec>", !args.revspecs.is_empty()),
    ])?;

    let repo = Repository::discover(path)?;
    let meta = Metadata::open(&repo)?;

    if args.all {
        unhide_all(&repo, &meta)
    } else {
        unhide_revspecs(&repo, &meta, &args.revspecs)
    }
}
