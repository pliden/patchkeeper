use crate::meta::Metadata;
use crate::repo::RepositoryUtils;
use crate::repo::HEAD;
use anyhow::bail;
use anyhow::Result;
use cmdline::CmdLine;
use git2::IndexAddOption;
use git2::Repository;
use std::path::Path;
use std::path::PathBuf;

#[derive(CmdLine)]
pub struct Args {
    #[cmdline(positional)]
    path: Vec<PathBuf>,
}

fn include(repo: &Repository, meta: &Metadata, paths: &[PathBuf]) -> Result<()> {
    let name = repo.head_name()?;
    let mut branch = meta.branches.acquire(&name);

    if branch.pushed.is_empty() {
        bail!("nothing pushed");
    }

    let relative_paths = repo.paths_relative_to_workdir(paths)?;

    let mut index = repo.index()?;
    let commit = repo.head()?.peel_to_commit()?;
    if commit.parent_count() > 0 {
        let parent_tree = commit.parent(0)?.tree()?;
        index.read_tree(&parent_tree)?;
    } else {
        index.clear()?;
    }

    for path in relative_paths {
        index.add_path(&path)?;
    }

    let update_ref = Some(HEAD);
    let author = None;
    let committer = None;
    let encoding = None;
    let message = None;
    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;
    let tree = Some(&tree);
    let oid = commit.amend(update_ref, author, committer, encoding, message, tree)?;

    index.add_all(["*"], IndexAddOption::CHECK_PATHSPEC, None)?;
    index.write()?;

    branch.pushed.replace_top(oid);
    meta.branches.release(branch);

    meta.commit(repo, "include")
}

pub fn main(path: &Path, args: Args) -> Result<()> {
    let repo = Repository::discover(path)?;
    let meta = Metadata::open(&repo)?;

    repo.ensure_no_unresolved()?;

    include(&repo, &meta, &args.path)
}
