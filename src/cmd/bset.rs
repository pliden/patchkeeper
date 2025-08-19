use crate::print;
use crate::repo::BranchUtils;
use crate::repo::RepositoryUtils;
use anyhow::Result;
use git2::BranchType;
use git2::Repository;
use immargs::Args;
use immargs::immargs;
use std::path::Path;

fn set(repo: &Repository, name: &str) -> Result<()> {
    let branch = repo.find_branch(name, BranchType::Local)?;
    let full_name = branch.full_name()?;

    print::branch(name, "branch");
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

    set(&repo, &args.branch)
}
