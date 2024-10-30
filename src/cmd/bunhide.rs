use crate::cmd;
use crate::meta::Metadata;
use crate::meta::HIDDEN;
use crate::print;
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

fn unhide(repo: &Repository, meta: &Metadata, names: &[String]) -> Result<()> {
    for name in names {
        let _ = repo.find_branch(name, BranchType::Local)?;
        let branch = meta.branches.acquire(name);

        if !branch.properties.get_flag(HIDDEN)? {
            bail!("branch '{}' is not hidden", name);
        }

        branch.properties.remove(HIDDEN);
        meta.branches.release(branch);

        print::branch_action(name, "unhide");

        meta.commit(repo, "bunhide")?;
    }

    Ok(())
}

pub fn main(path: &Path, args: Args) -> Result<()> {
    cmd::missing_option(&[("<branch>", !args.names.is_empty())])?;

    let repo = Repository::discover(path)?;
    let meta = Metadata::open(&repo)?;

    unhide(&repo, &meta, &args.names)
}
