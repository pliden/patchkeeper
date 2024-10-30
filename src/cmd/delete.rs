use crate::cmd;
use crate::meta::Metadata;
use crate::print;
use crate::repo::RepositoryUtils;
use crate::ui;
use anyhow::bail;
use anyhow::Result;
use git2::Commit;
use git2::Repository;
use gumdrop::Options;
use std::path::Path;

#[derive(Options)]
pub struct Args {
    #[options(help = "Interactive mode")]
    interactive: bool,

    #[options(help = "Delete next commit")]
    next: bool,

    #[options(help = "Print help message")]
    help: bool,

    #[options(free, help = "[<revspec>...]")]
    revspecs: Vec<String>,
}

fn delete(repo: &Repository, meta: &Metadata, commits: &[Commit]) -> Result<()> {
    if commits.is_empty() {
        bail!("nothing to delete");
    }

    let name = repo.head_name()?;

    for commit in commits {
        let mut branch = meta.branches.acquire(&name);

        if !branch.hidden.remove(commit.id()) && !branch.popped.remove(commit.id()) {
            bail!("cannot delete non-popped commit");
        }

        meta.branches.release(branch);

        print::commit_action(commit, "delete")?;

        meta.commit(repo, "delete")?;
    }

    Ok(())
}

fn delete_interactive(repo: &Repository, meta: &Metadata) -> Result<()> {
    let name = repo.head_name()?;
    let branch = meta.branches.acquire(&name);
    let oids = branch.popped.all();
    let commits = repo.find_commits(&oids)?;
    meta.branches.release(branch);

    if commits.is_empty() {
        bail!("nothing to delete");
    }

    let selected = ui::select_commit("DELETE", &commits, false)?;
    if let Some(selected) = selected {
        print::commit_action(&commits[selected], "selected")?;
        // delete(repo, meta, &commits[selected..=selected])?;
    }

    Ok(())
}

fn delete_revspecs(repo: &Repository, meta: &Metadata, revspecs: &[String]) -> Result<()> {
    let commits = repo.find_commits_by_revspecs(revspecs)?;
    delete(repo, meta, &commits)
}

fn delete_next(repo: &Repository, meta: &Metadata) -> Result<()> {
    let name = repo.head_name()?;
    let branch = meta.branches.acquire(&name);
    let oids = branch.popped.bottom_as_vec();
    let commits = repo.find_commits(&oids)?;
    meta.branches.release(branch);
    delete(repo, meta, &commits)
}

pub fn main(path: &Path, args: Args) -> Result<()> {
    cmd::missing_or_conflicting_options(&[
        ("-i", args.interactive),
        ("-n", args.next),
        ("<revspec>", !args.revspecs.is_empty()),
    ])?;

    let repo = Repository::discover(path)?;
    let meta = Metadata::open(&repo)?;

    if args.interactive {
        delete_interactive(&repo, &meta)
    } else if !args.revspecs.is_empty() {
        delete_revspecs(&repo, &meta, &args.revspecs)
    } else {
        delete_next(&repo, &meta)
    }
}
