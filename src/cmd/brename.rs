use crate::meta::Metadata;
use crate::repo::RepositoryUtils;
use anyhow::Result;
use git2::BranchType;
use git2::Repository;
use immargs::args;
use std::path::Path;

args! {
    BrenameArgs,
    -h --help   "print help message",
    [<from>] String,
    <to> String,
}

fn rename(repo: &Repository, meta: &Metadata, from: &str, to: &str) -> Result<()> {
    let mut branch = repo.find_branch(from, BranchType::Local)?;
    branch.rename(to, false)?;

    meta.branches.rename(from, to);
    meta.commit(repo, "brename")
}

fn rename_current(repo: &Repository, meta: &Metadata, to: &str) -> Result<()> {
    let from = repo.head_name()?;
    rename(repo, meta, &from, to)
}

pub fn main(path: &Path, args: BrenameArgs) -> Result<()> {
    let repo = Repository::discover(path)?;
    let meta = Metadata::open(&repo)?;

    repo.ensure_no_unresolved()?;
    repo.ensure_no_unrefreshed()?;

    if let Some(from) = &args.from {
        rename(&repo, &meta, from, &args.to)
    } else {
        rename_current(&repo, &meta, &args.to)
    }
}
