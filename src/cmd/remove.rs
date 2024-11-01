use crate::repo::RepositoryUtils;
use crate::safe_println;
use anyhow::bail;
use anyhow::Result;
use git2::Repository;
use git2::Status;
use gumdrop::Options;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::str;

#[derive(Options)]
pub struct Args {
    #[options(help = "Print help message")]
    help: bool,

    #[options(free, required, help = "[<path>...]")]
    paths: Vec<String>,
}

fn remove(repo: &Repository, pathspecs: &[String]) -> Result<()> {
    let mut removed = 0;

    let print_remove = &mut |path: &Path, _matched_spec: &[u8]| -> i32 {
        match repo.status_file(path) {
            Ok(status) if status.contains(Status::WT_DELETED) => {
                safe_println!("remove: {}", path.display());
                removed += 1;
                0
            }
            Ok(_) => 1,
            Err(error) => {
                safe_println!("error: {}: {}", path.display(), error);
                -1
            }
        }
    };

    let callback = Some(print_remove as &mut git2::IndexMatchedPath);

    let mut index = repo.index()?;
    index.remove_all(pathspecs, callback)?;
    index.write()?;

    if removed == 0 {
        bail!("nothing to remove");
    }

    Ok(())
}

fn remove_files_and_empty_dirs(paths: &[String]) -> Result<()> {
    for path in paths {
        let path = PathBuf::from(path);
        fs::remove_file(path.as_path())?;
        if let Some(parent) = path.parent() {
            let _ = fs::remove_dir(parent);
        }
    }

    Ok(())
}

fn remove_paths(repo: &Repository, paths: &[String]) -> Result<()> {
    let relative_paths = repo.paths_relative_to_workdir(paths)?;
    remove_files_and_empty_dirs(paths)?;
    remove(repo, &relative_paths)
}

pub fn main(path: &Path, args: Args) -> Result<()> {
    let repo = Repository::discover(path)?;

    repo.ensure_no_unresolved()?;

    remove_paths(&repo, &args.paths)
}
