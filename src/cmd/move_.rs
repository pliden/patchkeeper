use crate::cmd::remove::remove_empty_parent_dirs;
use crate::repo::RepositoryUtils;
use anyhow::Result;
use anyhow::anyhow;
use anyhow::bail;
use git2::Repository;
use immargs::args;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

args! {
    MoveArgs,
    -h --help "print help message",
    <src>... PathBuf,
    <dest> PathBuf,
}

fn move_to_file(repo: &Repository, src_file: &Path, dest_file: &Path) -> Result<()> {
    let relative_from_path = repo.path_relative_to_workdir(src_file)?;

    fs::rename(src_file, dest_file)?;
    remove_empty_parent_dirs(src_file);

    let relative_to_path = repo.path_relative_to_workdir(dest_file)?;

    let mut index = repo.index()?;
    index.remove_path(&relative_from_path)?;
    index.add_path(&relative_to_path)?;
    Ok(index.write()?)
}

fn move_to_dir(repo: &Repository, src_file: &Path, to_dir: &Path) -> Result<()> {
    let file_name = src_file
        .file_name()
        .ok_or(anyhow!("file not found: {}", src_file.display()))?;
    let mut to_file = to_dir.to_path_buf();
    to_file.push(file_name);
    move_to_file(repo, src_file, &to_file)
}

fn move_(repo: &Repository, src: &[PathBuf], dest: &Path) -> Result<()> {
    if src.len() > 1 && !dest.is_dir() {
        bail!("destination must be a directory");
    }

    for src_file in src {
        if dest.is_dir() {
            move_to_dir(repo, src_file, dest)?;
        } else {
            move_to_file(repo, src_file, dest)?;
        }
    }

    Ok(())
}

pub fn main(path: &Path, args: MoveArgs) -> Result<()> {
    let repo = Repository::discover(path)?;

    repo.ensure_no_unresolved()?;

    move_(&repo, &args.src, &args.dest)
}
