use crate::print;
use crate::repo::BranchUtils;
use crate::repo::RepositoryUtils;
use anyhow::Result;
use git2::BranchType;
use git2::Repository;
use gumdrop::Options;
use std::path::Path;

#[derive(Options)]
pub struct Args {
    #[options(help = "Print help message")]
    help: bool,

    #[options(free, required, help = "<name>")]
    name: String,
}

fn set(repo: &Repository, name: &str) -> Result<()> {
    let branch = repo.find_branch(name, BranchType::Local)?;
    let full_name = branch.full_name()?;

    print::branch_action(name, "branch");
    repo.set_head(&full_name)?;

    Ok(())
}

pub fn main(path: &Path, args: Args) -> Result<()> {
    let repo = Repository::discover(path)?;

    repo.ensure_no_unresolved()?;
    repo.ensure_no_unrefreshed()?;

    set(&repo, &args.name)
}
