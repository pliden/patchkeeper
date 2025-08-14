use crate::repo::RepositoryUtils;
use anyhow::Result;
use git2::Repository;
use immargs::ImmArgs;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::str;

#[derive(ImmArgs)]
pub struct Args {
    #[arg(positional)]
    path: Vec<PathBuf>,
}

pub fn remove_empty_parent_dirs(path: &Path) {
    let mut parent = path.parent();
    while parent.is_some() {
        let dir = parent.unwrap();
        parent = if fs::remove_dir(dir).is_ok() {
            dir.parent()
        } else {
            None
        }
    }
}

fn remove_paths(repo: &Repository, paths: &[PathBuf]) -> Result<()> {
    for path in paths {
        let relative_path = repo.path_relative_to_workdir(path)?;

        fs::remove_file(path)?;
        remove_empty_parent_dirs(path);

        let mut index = repo.index()?;
        index.remove_path(&relative_path)?;
        index.write()?;
    }

    Ok(())
}

pub fn main(path: &Path, args: Args) -> Result<()> {
    let repo = Repository::discover(path)?;

    repo.ensure_no_unresolved()?;

    remove_paths(&repo, &args.path)
}
