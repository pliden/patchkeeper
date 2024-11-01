use crate::cmd;
use crate::repo::RepositoryUtils;
use crate::safe_println;
use anyhow::bail;
use anyhow::Result;
use git2::IndexAddOption;
use git2::Repository;
use gumdrop::Options;
use std::path::Path;
use std::str;

#[derive(Options)]
pub struct Args {
    #[options(help = "Add all untracked files")]
    all: bool,

    #[options(help = "Print help message")]
    help: bool,

    #[options(free, help = "[<path>...]")]
    paths: Vec<String>,
}

fn add(repo: &Repository, pathspecs: &[String]) -> Result<()> {
    let mut added = 0;

    let print_add = &mut |path: &Path, _matched_spec: &[u8]| -> i32 {
        match repo.status_file(path) {
            Ok(status) if status.contains(git2::Status::WT_NEW) => {
                safe_println!("add: {}", path.display());
                added += 1;
                0
            }
            Ok(_) => 1,
            Err(error) => {
                safe_println!("error: {}: {}", path.display(), error);
                -1
            }
        }
    };

    let callback = Some(print_add as &mut git2::IndexMatchedPath);

    let mut index = repo.index()?;
    index.add_all(pathspecs, IndexAddOption::CHECK_PATHSPEC, callback)?;
    index.write()?;

    if added == 0 {
        bail!("nothing to add");
    }

    Ok(())
}

fn add_paths(repo: &Repository, paths: &[String]) -> Result<()> {
    let relative_paths = repo.paths_relative_to_workdir(paths)?;
    add(repo, &relative_paths)
}

fn add_all(repo: &Repository) -> Result<()> {
    let all_paths = [String::from("*")];
    add(repo, &all_paths)
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
