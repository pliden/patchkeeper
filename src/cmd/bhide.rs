use crate::meta::Metadata;
use crate::meta::HIDDEN;
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
    #[options(help = "Print help message")]
    help: bool,

    #[options(free, help = "[<branch>...]")]
    names: Vec<String>,
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

        print::branch_action(name, "hide");

        meta.commit(repo, "bhide")?;
    }

    Ok(())
}

fn hide_current(repo: &Repository, meta: &Metadata) -> Result<()> {
    let names = [repo.head_name()?];
    hide(repo, meta, &names)
}

pub fn main(path: &Path, args: Args) -> Result<()> {
    let repo = Repository::discover(path)?;
    let meta = Metadata::open(&repo)?;

    if !args.names.is_empty() {
        hide(&repo, &meta, &args.names)
    } else {
        hide_current(&repo, &meta)
    }
}
