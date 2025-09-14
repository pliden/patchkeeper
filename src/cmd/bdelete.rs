use crate::meta::Metadata;
use crate::print;
use crate::repo::RepositoryUtils;
use anyhow::Result;
use anyhow::bail;
use git2::BranchType;
use git2::Repository;
use immargs::immargs;
use std::path::Path;

immargs! {
    BdeleteArgs,
    -f --force   "force delete",
    -h --help    "print help message",
    <branch> String,
}

fn delete(repo: &Repository, meta: &Metadata, name: &str, force: bool) -> Result<()> {
    let mut branch = repo.find_branch(name, BranchType::Local)?;

    if name == repo.head_name()? {
        bail!("cannot delete current branch");
    }

    if !meta.branches.acquire(name).is_empty() && !force {
        bail!("branch has patches and/or properties (use --force to delete)");
    }

    print::branch(name, "delete");
    branch.delete()?;

    meta.commit(repo, "bdelete")
}

pub fn main(path: &Path, args: BdeleteArgs) -> Result<()> {
    let repo = Repository::discover(path)?;
    let meta = Metadata::open(&repo)?;

    delete(&repo, &meta, &args.branch, args.force)
}
