use crate::meta::HIDDEN;
use crate::meta::Metadata;
use crate::print;
use crate::repo::BranchUtils;
use crate::repo::CommitUtils;
use crate::repo::RepositoryUtils;
use crate::stdout;
use anyhow::Result;
use colored::Color;
use colored::Colorize;
use git2::BranchType;
use git2::Oid;
use git2::Repository;
use immargs::args;
use std::path::Path;

args! {
    ListArgs,
    -a --all             ! "show all branches",
    -x --hidden          ! "show hidden commit",
    -h --help              "print help message",
    [<branch>...] String !,
}

fn print_branch(name: &str, is_hidden: bool, color: Color) {
    if is_hidden {
        stdout!("{} (hidden)\n", name.bold().color(color));
    } else {
        stdout!("{}\n", name.bold().color(color));
    }
}

fn print_commit(repo: &Repository, oid: Oid, color: Color, marker: bool) -> Result<()> {
    let commit = repo.find_commit(oid)?;
    let short_oid = commit.short_id()?;
    let summary = commit.summary().unwrap_or_default();
    let marker = print::marker(marker);

    stdout!(
        "{} {} {}\n",
        marker.bold().red(),
        short_oid.bold().color(color),
        summary
    );

    Ok(())
}

fn list(repo: &Repository, meta: &Metadata, names: &[String], hidden: bool) -> Result<()> {
    let current_name = repo.head_name()?;

    for name in names {
        let branch = repo.find_branch(name, BranchType::Local)?;
        let branch_oid = branch.get().peel_to_commit()?.id();

        let branch = meta.branches.acquire(name);
        let branch_is_hidden = branch.properties.get_flag(HIDDEN)?;

        if name != &current_name && branch_is_hidden && !hidden {
            continue;
        }

        print_branch(name, branch_is_hidden, Color::Yellow);

        if hidden {
            for oid in branch.hidden.all() {
                print_commit(repo, oid, Color::Cyan, false)?;
            }
        } else {
            for oid in branch.popped.all() {
                print_commit(repo, oid, Color::White, false)?;
            }
            for oid in branch.pushed.all() {
                let marker = oid == branch_oid;
                print_commit(repo, oid, Color::Green, marker)?;
            }
        }
    }

    Ok(())
}

fn list_all(repo: &Repository, meta: &Metadata, hidden: bool) -> Result<()> {
    let mut names = vec![];
    for branch_and_type in repo.branches(Some(BranchType::Local))? {
        let (branch, _) = branch_and_type?;
        let name = branch.short_name()?;
        names.push(name);
    }

    list(repo, meta, &names, hidden)
}

fn list_current(repo: &Repository, meta: &Metadata, hidden: bool) -> Result<()> {
    let names = [repo.head_name()?];
    list(repo, meta, &names, hidden)
}

pub fn main(path: &Path, args: ListArgs) -> Result<()> {
    let repo = Repository::discover(path)?;
    let meta = Metadata::open(&repo)?;

    if !args.branch.is_empty() {
        list(&repo, &meta, &args.branch, args.hidden)
    } else if args.all {
        list_all(&repo, &meta, args.hidden)
    } else {
        list_current(&repo, &meta, args.hidden)
    }
}
