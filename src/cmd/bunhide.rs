use crate::meta::HIDDEN;
use crate::meta::Metadata;
use crate::print;
use anyhow::Result;
use anyhow::bail;
use git2::BranchType;
use git2::Repository;
use immargs::args;
use std::path::Path;

args! {
    BunhideArgs,
    -h --help   "print help message",
    <branch>... String,
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

        print::branch(name, "unhide");

        meta.commit(repo, "bunhide")?;
    }

    Ok(())
}

pub fn main(path: &Path, args: BunhideArgs) -> Result<()> {
    let repo = Repository::discover(path)?;
    let meta = Metadata::open(&repo)?;

    unhide(&repo, &meta, &args.branch)
}
