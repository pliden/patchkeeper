use crate::cmd;
use crate::meta::Metadata;
use crate::print;
use crate::repo::IndexUtils;
use crate::repo::RepositoryUtils;
use crate::safe_println;
use anyhow::bail;
use anyhow::Result;
use colored::Colorize;
use git2::Repository;
use gumdrop::Options;
use std::path::Path;
use std::str;

#[derive(Options)]
pub struct Args {
    #[options(help = "Mark all merge conflicts as resolved")]
    all: bool,

    #[options(help = "List unresolved merge conflicts")]
    list: bool,

    #[options(help = "Undo push causing current merge conflict")]
    undo: bool,

    #[options(help = "Print help message")]
    help: bool,

    #[options(free, help = "[<path>...]")]
    paths: Vec<String>,
}

fn resolve(repo: &Repository, paths: Option<&Vec<String>>) -> Result<()> {
    let mut index = repo.index()?;
    let conflicts = index.unresolved_conflicts()?;
    let paths = match paths {
        Some(paths) => paths,
        _ => &conflicts,
    };

    let unresolved_paths = paths
        .iter()
        .filter(|path| conflicts.contains(path))
        .collect::<Vec<_>>();

    if unresolved_paths.is_empty() {
        bail!("nothing to resolve");
    }

    for path in unresolved_paths {
        safe_println!("resolved: {}", path.bold().green());
        index.add_path(Path::new(&path))?;
    }

    Ok(index.write()?)
}

fn list(repo: &Repository) -> Result<()> {
    let index = repo.index()?;
    let conflicts = index.unresolved_conflicts()?;

    if conflicts.is_empty() {
        safe_println!("no merge conflicts");
    } else {
        for path in conflicts {
            safe_println!("{}", path);
        }
    }

    Ok(())
}

fn undo(repo: &Repository, meta: &Metadata) -> Result<()> {
    let meta = match meta.undo(repo)? {
        Some(meta) => meta,
        _ => bail!("nothing to undo"),
    };

    let name = repo.head_name()?;
    let branch = meta.branches.acquire(&name);
    let head = repo.find_commit(branch.pushed.top())?;
    meta.branches.release(branch);

    print::commit_action(&head, "restore")?;
    repo.reset_hard(&head)?;

    meta.commit(repo, "undo")
}

pub fn main(path: &Path, args: Args) -> Result<()> {
    cmd::missing_or_conflicting_options(&[
        ("-a", args.all),
        ("-l", args.list),
        ("-u", args.undo),
        ("<path>", !args.paths.is_empty()),
    ])?;

    let repo = Repository::discover(path)?;
    let meta = Metadata::open(&repo)?;

    if !args.paths.is_empty() {
        resolve(&repo, Some(&args.paths))
    } else if args.all {
        resolve(&repo, None)
    } else if args.list {
        list(&repo)
    } else {
        undo(&repo, &meta)
    }
}
