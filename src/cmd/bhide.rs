use crate::meta::HIDDEN;
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
    BhideArgs,
    -h --help   "print help message",
    [<branch>...] String,
}

fn hide(repo: &Repository, meta: &Metadata, names: &[String]) -> Result<()> {
    for name in names {
        let _ = repo.find_branch(name, BranchType::Local)?;
        let branch = meta.branches.acquire(name);

        if branch.properties.get_flag(HIDDEN)? {
            bail!("branch '{}' already hidden", name);
        }

        branch.properties.set_flag(HIDDEN);
        meta.branches.release(branch);

        print::branch(name, "hide");

        meta.commit(repo, "bhide")?;
    }

    Ok(())
}

fn hide_current(repo: &Repository, meta: &Metadata) -> Result<()> {
    let names = [repo.head_name()?];
    hide(repo, meta, &names)
}

pub fn main(path: &Path, args: BhideArgs) -> Result<()> {
    let repo = Repository::discover(path)?;
    let meta = Metadata::open(&repo)?;

    if !args.branch.is_empty() {
        hide(&repo, &meta, &args.branch)
    } else {
        hide_current(&repo, &meta)
    }
}
