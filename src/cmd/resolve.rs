use crate::meta::Metadata;
use crate::print;
use crate::repo::IndexUtils;
use crate::repo::RepositoryUtils;
use crate::stdout;
use anyhow::bail;
use anyhow::Result;
use git2::Repository;
use immargs::ImmArgs;
use std::path::Path;
use std::path::PathBuf;

#[derive(ImmArgs)]
pub struct Args {
    #[arg(choice = "0", help = "Mark all merge conflicts as resolved")]
    all: Option<()>,

    #[arg(choice = "0", help = "List unresolved merge conflicts")]
    list: Option<()>,

    #[arg(choice = "0", help = "Undo push causing current merge conflict")]
    undo: Option<()>,

    #[arg(positional, choice = "0")]
    path: Option<Vec<PathBuf>>,
}

fn resolve(repo: &Repository, paths: Option<&Vec<PathBuf>>) -> Result<()> {
    let mut index = repo.index()?;
    let conflicts = index.unresolved_conflicts()?;

    if conflicts.is_empty() {
        bail!("nothing to resolve");
    }

    let relative_paths = match paths {
        Some(paths) => Some(repo.paths_relative_to_workdir(paths)?),
        _ => None,
    };

    let unresolved_paths = match relative_paths {
        Some(paths) => paths
            .into_iter()
            .filter(|path| conflicts.contains(path))
            .collect::<Vec<_>>(),
        _ => conflicts,
    };

    for path in unresolved_paths {
        index.add_path(Path::new(&path))?;
    }

    Ok(index.write()?)
}

fn list(repo: &Repository) -> Result<()> {
    let index = repo.index()?;
    let conflicts = index.unresolved_conflicts()?;

    if conflicts.is_empty() {
        stdout!("no merge conflicts\n");
    } else {
        for path in &conflicts {
            stdout!("{}\n", path.display());
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

    print::commit(&head, "restore")?;
    repo.reset_hard(&head)?;

    meta.commit(repo, "undo")
}

pub fn main(path: &Path, args: Args) -> Result<()> {
    let repo = Repository::discover(path)?;
    let meta = Metadata::open(&repo)?;

    if let Some(path) = args.path {
        resolve(&repo, Some(&path))
    } else if args.all.is_some() {
        resolve(&repo, None)
    } else if args.list.is_some() {
        list(&repo)
    } else if args.undo.is_some() {
        undo(&repo, &meta)
    } else {
        panic!();
    }
}
