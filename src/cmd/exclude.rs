use crate::meta::Metadata;
use crate::repo::HEAD;
use crate::repo::RepositoryUtils;
use anyhow::Result;
use anyhow::bail;
use git2::IndexAddOption;
use git2::Repository;
use immargs::immargs;
use std::path::Path;
use std::path::PathBuf;

immargs! {
    ExcludeArgs,
    -h --help "print help message",
    <path>... PathBuf,
}

fn exclude(repo: &Repository, meta: &Metadata, paths: &[PathBuf]) -> Result<()> {
    let name = repo.head_name()?;
    let mut branch = meta.branches.acquire(&name);

    if branch.pushed.is_empty() {
        bail!("nothing pushed");
    }

    let relative_paths = repo.paths_relative_to_workdir(paths)?;

    let commit = repo.head()?.peel_to_commit()?;
    let parent = if commit.parent_count() > 0 {
        Some(commit.parent(0)?.into_object())
    } else {
        None
    };

    repo.reset_default(parent.as_ref(), &relative_paths)?;

    let update_ref = Some(HEAD);
    let author = None;
    let committer = None;
    let encoding = None;
    let message = None;
    let mut index = repo.index()?;
    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;
    let tree = Some(&tree);
    let oid = commit.amend(update_ref, author, committer, encoding, message, tree)?;

    index.add_all(relative_paths, IndexAddOption::CHECK_PATHSPEC, None)?;
    index.write()?;

    branch.pushed.replace_top(oid);
    meta.branches.release(branch);

    meta.commit(repo, "exclude")
}

pub fn main(path: &Path, args: ExcludeArgs) -> Result<()> {
    let repo = Repository::discover(path)?;
    let meta = Metadata::open(&repo)?;

    repo.ensure_no_unresolved()?;

    exclude(&repo, &meta, &args.path)
}
