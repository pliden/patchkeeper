use crate::cmd::remove::remove_empty_parent_dirs;
use crate::repo::RepositoryUtils;
use anyhow::anyhow;
use anyhow::bail;
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

fn move_to_file(repo: &Repository, from_file: &Path, to_file: &Path) -> Result<()> {
    let relative_from_path = repo.path_relative_to_workdir(from_file)?;

    fs::rename(from_file, to_file)?;
    remove_empty_parent_dirs(from_file);

    let relative_to_path = repo.path_relative_to_workdir(to_file)?;

    let mut index = repo.index()?;
    index.remove_path(&relative_from_path)?;
    index.add_path(&relative_to_path)?;
    Ok(index.write()?)
}

fn move_to_dir(repo: &Repository, from_file: &Path, to_dir: &Path) -> Result<()> {
    let file_name = from_file
        .file_name()
        .ok_or(anyhow!("file not found: {}", from_file.display()))?;
    let mut to_file = to_dir.to_path_buf();
    to_file.push(file_name);
    move_to_file(repo, from_file, &to_file)
}

fn move_paths(repo: &Repository, paths: &[PathBuf]) -> Result<()> {
    let (to, from) = paths.split_last().unwrap();

    if from.is_empty() {
        bail!("must specify at least two paths");
    }

    if from.len() > 1 && !to.is_dir() {
        bail!("last path must be a directory");
    }

    for path in from {
        if to.is_dir() {
            move_to_dir(repo, path, to)?;
        } else {
            move_to_file(repo, path, to)?;
        }
    }

    Ok(())
}

pub fn main(path: &Path, args: Args) -> Result<()> {
    let repo = Repository::discover(path)?;

    repo.ensure_no_unresolved()?;

    move_paths(&repo, &args.path)
}
