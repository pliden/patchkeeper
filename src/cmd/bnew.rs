use crate::print;
use crate::repo::BranchUtils;
use crate::repo::RepositoryUtils;
use anyhow::Result;
use anyhow::bail;
use git2::BranchType;
use git2::Repository;
use immargs::Args;
use immargs::immargs;
use std::path::Path;

fn new(repo: &Repository, name: &str) -> Result<()> {
    if repo.find_branch(name, BranchType::Local).is_ok() {
        bail!("branch already exists");
    }

    let head = repo.head()?.peel_to_commit()?;
    let branch = repo.branch(name, &head, false)?;
    let full_name = branch.full_name()?;

    print::branch(name, "new");
    repo.set_head(&full_name)?;

    Ok(())
}

pub fn main(path: &Path, args: Args) -> Result<()> {
    let args = immargs!(
        { args }
        <branch> String   "Branch name",
    );

    let repo = Repository::discover(path)?;

    repo.ensure_no_unresolved()?;
    repo.ensure_no_unrefreshed()?;

    new(&repo, &args.branch)
}
