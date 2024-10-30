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
    #[options(help = "Pop all commits")]
    all: bool,

    #[options(help = "Pop finalized commit")]
    finalized: bool,

    #[options(help = "Print help message")]
    help: bool,

    #[options(free, help = "[<revspec>]")]
    revspec: Option<String>,
}

fn pop(repo: &Repository, meta: &Metadata, commits: &[Commit]) -> Result<()> {
    if commits.is_empty() {
        bail!("nothing to pop");
    }

    let name = repo.head_name()?;

    for commit in commits {
        let head = repo.head()?.peel_to_commit()?;

        if commit.id() != head.id() {
            bail!("cannot pop non-head commit");
        } else if commit.parent_count() == 0 {
            bail!("cannot pop initial commit");
        } else if commit.parent_count() > 1 {
            bail!("cannot pop merge commit");
        }

        let mut branch = meta.branches.acquire(&name);
        branch.pushed.remove(commit.id());
        branch.popped.add_bottom(commit.id());
        meta.branches.release(branch);

        print::commit_action(commit, "pop")?;

        let parent = commit.parent(0).unwrap();
        repo.reset_hard(&parent)?;

        meta.commit(repo, "pop")?;
    }

    Ok(())
}

fn pop_revspec(repo: &Repository, meta: &Metadata, revspec: &str) -> Result<()> {
    let name = repo.head_name()?;
    let branch = meta.branches.acquire(&name);
    let commit = repo.find_commit_by_revspec(revspec)?;
    if !branch.pushed.contains(commit.id()) {
        bail!("cannot pop non-pushed commit");
    }
    let oids = branch.pushed.range(commit.id());
    let commits = repo.find_commits(&oids)?;
    meta.branches.release(branch);
    pop(repo, meta, &commits)
}

fn pop_all(repo: &Repository, meta: &Metadata) -> Result<()> {
    let name = repo.head_name()?;
    let branch = meta.branches.acquire(&name);
    let oids = branch.pushed.all();
    let commits = repo.find_commits(&oids)?;
    meta.branches.release(branch);
    pop(repo, meta, &commits)
}

fn pop_finalized(repo: &Repository, meta: &Metadata) -> Result<()> {
    let name = repo.head_name()?;
    let mut branch = meta.branches.acquire(&name);
    if !branch.pushed.is_empty() {
        bail!("cannot have pushed commits");
    }
    let commit = repo.head()?.peel_to_commit()?;
    branch.pushed.add_top(commit.id());
    let commits = [commit];
    meta.branches.release(branch);
    pop(repo, meta, &commits)
}

fn pop_next(repo: &Repository, meta: &Metadata) -> Result<()> {
    let name = repo.head_name()?;
    let branch = meta.branches.acquire(&name);
    let oids = branch.pushed.top_as_vec();
    let commits = repo.find_commits(&oids)?;
    meta.branches.release(branch);
    pop(repo, meta, &commits)
}

pub fn main(path: &Path, args: Args) -> Result<()> {
    cmd::conflicting_options(&[
        ("-a", args.all),
        ("-f", args.finalized),
        ("<revspec>", args.revspec.is_some()),
    ])?;

    let repo = Repository::discover(path)?;
    let meta = Metadata::open(&repo)?;

    repo.ensure_no_unresolved()?;
    repo.ensure_no_unrefreshed()?;

    if let Some(revspec) = args.revspec {
        pop_revspec(&repo, &meta, &revspec)
    } else if args.all {
        pop_all(&repo, &meta)
    } else if args.finalized {
        pop_finalized(&repo, &meta)
    } else {
        pop_next(&repo, &meta)
    }
}
