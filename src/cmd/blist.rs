use crate::meta::Metadata;
use crate::meta::HIDDEN;
use crate::print;
use crate::repo::BranchUtils;
use crate::stdout;
use anyhow::Result;
use colored::Colorize;
use git2::BranchType;
use git2::Repository;
use immargs::ImmArgs;
use std::path::Path;

#[derive(ImmArgs)]
pub struct Args {
    #[arg(help = "Show remote branches")]
    remote: Option<()>,

    #[arg(short = 'x', help = "Show hidden branches")]
    hidden: Option<()>,
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

pub fn main(path: &Path, args: Args) -> Result<()> {
    let repo = Repository::discover(path)?;
    let meta = Metadata::open(&repo)?;

    list(&repo, &meta, args.remote.is_some(), args.hidden.is_some())
}
