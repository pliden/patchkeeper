use crate::repo::RepositoryUtils;
use anyhow::Result;
use anyhow::bail;
use git2::IndexAddOption;
use git2::Repository;
use immargs::immargs;
use std::path::Path;
use std::path::PathBuf;

immargs! {
    AddArgs,
    -a --all            ! "add all untracked files",
    -h --help             "print help message",
    [<path>...] PathBuf !,
}

fn add(repo: &Repository, paths: &[PathBuf]) -> Result<()> {
    let mut index = repo.index()?;
    index.add_all(paths, IndexAddOption::CHECK_PATHSPEC, None)?;
    Ok(index.write()?)
}

fn add_all(repo: &Repository) -> Result<()> {
    add(repo, &[PathBuf::from("*")])
}

fn add_paths(repo: &Repository, paths: &[PathBuf]) -> Result<()> {
    if paths.is_empty() {
        bail!("nothing to add");
    }

    let relative_paths = repo.paths_relative_to_workdir(paths)?;
    add(repo, &relative_paths)
}

pub fn main(path: &Path, args: AddArgs) -> Result<()> {
    let repo = Repository::discover(path)?;

    repo.ensure_no_unresolved()?;

    if args.all {
        add_all(&repo)
    } else {
        add_paths(&repo, &args.path)
    }
}
