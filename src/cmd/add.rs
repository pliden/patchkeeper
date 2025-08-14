use crate::repo::RepositoryUtils;
use anyhow::Result;
use git2::IndexAddOption;
use git2::Repository;
use immargs::ImmArgs;
use std::path::Path;
use std::path::PathBuf;

#[derive(ImmArgs)]
pub struct Args {
    #[arg(choice = "0", help = "Add all untracked files")]
    all: Option<()>,

    #[arg(positional, choice = "0")]
    path: Option<Vec<PathBuf>>,
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
    let repo = Repository::discover(path)?;

    repo.ensure_no_unresolved()?;

    if let Some(path) = args.path {
        add_paths(&repo, &path)
    } else if args.all.is_some() {
        add_all(&repo)
    } else {
        panic!();
    }
}
