use crate::meta::Metadata;
use crate::print;
use crate::repo::RepositoryUtils;
use crate::ui;
use anyhow::bail;
use anyhow::Result;
use git2::Commit;
use git2::Repository;
use immargs::ImmArgs;
use std::path::Path;

#[derive(ImmArgs)]
pub struct Args {
    #[arg(choice = "0", help = "Interactive mode")]
    interactive: Option<()>,

    #[arg(choice = "0", help = "Delete next commit")]
    next: Option<()>,

    #[arg(positional, choice = "0")]
    revspec: Option<Vec<String>>,
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

        print::commit(commit, "delete")?;

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
        print::commit(&commits[selected], "selected")?;
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
    let oids = branch.popped.bottom_vec();
    let commits = repo.find_commits(&oids)?;
    meta.branches.release(branch);
    delete(repo, meta, &commits)
}

pub fn main(path: &Path, args: Args) -> Result<()> {
    let repo = Repository::discover(path)?;
    let meta = Metadata::open(&repo)?;

    if args.interactive.is_some() {
        delete_interactive(&repo, &meta)
    } else if let Some(revspec) = &args.revspec {
        delete_revspecs(&repo, &meta, revspec)
    } else if args.next.is_some() {
        delete_next(&repo, &meta)
    } else {
        panic!();
    }
}
