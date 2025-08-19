use crate::meta::Metadata;
use crate::repo::RepositoryUtils;
use anyhow::Result;
use anyhow::bail;
use git2::BranchType;
use git2::Repository;
use immargs::Args;
use immargs::immargs;
use std::path::Path;

fn rename(repo: &Repository, meta: &Metadata, old_name: &str, new_name: &str) -> Result<()> {
    let mut branch = repo.find_branch(old_name, BranchType::Local)?;
    branch.rename(new_name, false)?;

    meta.branches.rename(old_name, new_name);
    meta.commit(repo, "brename")
}

fn rename_current(repo: &Repository, meta: &Metadata, new_name: &str) -> Result<()> {
    let old_name = repo.head_name()?;
    rename(repo, meta, &old_name, new_name)
}

pub fn main(path: &Path, args: Args) -> Result<()> {
    let args = immargs!(
        [from] String   "From branch name",
        <to> String     "To branch name",
        // FIXME! Add support for optional before required....
    );

    if args.branch.len() > 2 {
        bail!("too many arguments");
    }

    let repo = Repository::discover(path)?;
    let meta = Metadata::open(&repo)?;

    repo.ensure_no_unresolved()?;
    repo.ensure_no_unrefreshed()?;

    if args.branch.len() == 1 {
        rename_current(&repo, &meta, &args.branch[0])
    } else {
        rename(&repo, &meta, &args.branch[0], &args.branch[1])
    }
}
