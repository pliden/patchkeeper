use crate::meta::HIDDEN;
use crate::meta::Metadata;
use crate::print;
use crate::repo::BranchUtils;
use crate::stdout;
use anyhow::Result;
use colored::Colorize;
use git2::BranchType;
use git2::Repository;
use immargs::immargs;
use std::path::Path;

immargs! {
    BlistArgs,
    -r --remote   "show remote branches",
    -x --hidden   "show hidden branches",
    -h --help     "print help message",
}

fn is_branch_hidden(meta: &Metadata, name: &str) -> Result<bool> {
    let branch = meta.branches.acquire(name);
    let hidden = branch.properties.get_flag(HIDDEN)?;
    meta.branches.release(branch);
    Ok(hidden)
}

fn list(repo: &Repository, meta: &Metadata, remote: bool, hidden: bool) -> Result<()> {
    for branch_and_type in repo.branches(None)? {
        let (branch, branch_type) = branch_and_type?;
        if (branch_type == BranchType::Remote) != remote {
            continue;
        }

        let name = branch.short_name()?;

        if is_branch_hidden(meta, &name)? == hidden {
            let marker = print::marker(branch.is_head());
            stdout!("{} {}\n", marker.bold().red(), name.bold().yellow());
        }
    }

    Ok(())
}

pub fn main(path: &Path, args: BlistArgs) -> Result<()> {
    let repo = Repository::discover(path)?;
    let meta = Metadata::open(&repo)?;

    list(&repo, &meta, args.remote, args.hidden)
}
