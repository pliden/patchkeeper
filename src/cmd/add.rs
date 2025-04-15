use crate::cmd;
use crate::repo::RepositoryUtils;
use anyhow::Result;
use git2::IndexAddOption;
use git2::Repository;
use gumdrop::Options;
use std::path::Path;
use std::path::PathBuf;
use std::str;

#[derive(Options)]
pub struct Args {
    #[options(help = "Add all untracked files")]
    all: bool,

    #[options(help = "Print help message")]
    help: bool,

    #[options(free, help = "[<path>...]")]
    paths: Vec<PathBuf>,
}

fn add(repo: &Repository, paths: &[PathBuf]) -> Result<()> {
    let mut index = repo.index()?;
    index.add_all(paths, IndexAddOption::CHECK_PATHSPEC, None)?;
    Ok(index.write()?)
}

fn add_paths(repo: &Repository, paths: &[PathBuf]) -> Result<()> {
    let relative_paths = repo.paths_relative_to_workdir(paths)?;
    add(repo, &relative_paths)
}

fn add_all(repo: &Repository) -> Result<()> {
    add(repo, &[PathBuf::from("*")])
}

pub fn main(path: &Path, args: Args) -> Result<()> {
    cmd::missing_or_conflicting_options(&[("-a", args.all), ("<path>", !args.paths.is_empty())])?;

    let repo = Repository::discover(path)?;

    repo.ensure_no_unresolved()?;

    if !args.paths.is_empty() {
        add_paths(&repo, &args.paths)
    } else {
        add_all(&repo)
    }
}
