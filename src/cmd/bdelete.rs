use crate::meta::Metadata;
use crate::print;
use crate::repo::RepositoryUtils;
use anyhow::bail;
use anyhow::Result;
use git2::BranchType;
use git2::Repository;
use gumdrop::Options;
use std::path::Path;

#[derive(Options)]
pub struct Args {
    #[options(help = "Force delete")]
    force: bool,

    #[options(help = "Print help message")]
    help: bool,

    #[options(free, required, help = "<name>")]
    name: String,
}

fn delete(repo: &Repository, meta: &Metadata, name: &str, force: bool) -> Result<()> {
    let mut branch = repo.find_branch(name, BranchType::Local)?;

    if name == repo.head_name()? {
        bail!("cannot delete current branch");
    }

    if !meta.branches.acquire(name).is_empty() && !force {
        bail!("branch has patches and/or properties (use --force to delete)");
    }

    print::branch_action(name, "delete");
    branch.delete()?;

    meta.commit(repo, "bdelete")
}

pub fn main(path: &Path, args: Args) -> Result<()> {
    let repo = Repository::discover(path)?;
    let meta = Metadata::open(&repo)?;

    delete(&repo, &meta, &args.name, args.force)
}
