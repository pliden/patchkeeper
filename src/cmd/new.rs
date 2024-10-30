use crate::meta::Metadata;
use crate::print;
use crate::repo::RepositoryUtils;
use crate::repo::HEAD;
use anyhow::bail;
use anyhow::Result;
use git2::Commit;
use git2::Error;
use git2::ErrorClass;
use git2::ErrorCode;
use git2::Repository;
use gumdrop::Options;
use std::path::Path;

#[derive(Options)]
pub struct Args {
    #[options(help = "Print help message")]
    help: bool,

    #[options(free, required, help = "<message>")]
    message: Vec<String>,
}

fn resolve_head(repo: &Repository) -> Result<Option<Commit>> {
    fn is_reference_unborn_branch(error: &Error) -> bool {
        error.class() == ErrorClass::Reference && error.code() == ErrorCode::UnbornBranch
    }

    // Failure to resolve HEAD with a "reference unborn branch" error is a special
    // case when creating a commit. This error indicates that this is a repository
    // that doesn't have any commits yet. Commits must have at least one parent,
    // except for the initial commit, which is parentless.
    match repo.head() {
        Ok(head) => Ok(Some(head.peel_to_commit()?)),
        Err(error) if is_reference_unborn_branch(&error) => Ok(None),
        Err(error) => bail!(error),
    }
}

fn new(repo: &Repository, meta: &Metadata, message: &[String]) -> Result<()> {
    let update_ref = Some(HEAD);
    let signature = repo.signature()?;
    let author = &signature;
    let committer = &signature;
    let message = message.join(" ");
    let mut index = repo.index()?;
    index.update_all(["*"].iter(), None)?;
    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;
    let parent = resolve_head(repo)?;
    let parents = parent.iter().collect::<Vec<_>>();
    let oid = repo.commit(update_ref, author, committer, &message, &tree, &parents)?;
    index.write()?;

    let name = repo.head_name()?;
    let mut branch = meta.branches.acquire(&name);

    branch.pushed.add_top(oid);

    let commit = repo.find_commit(oid)?;
    print::commit_action(&commit, "new")?;

    meta.branches.release(branch);
    meta.commit(repo, "new")
}

pub fn main(path: &Path, args: Args) -> Result<()> {
    let repo = Repository::discover(path)?;
    let meta = Metadata::open(&repo)?;

    repo.ensure_no_unresolved()?;

    new(&repo, &meta, &args.message)
}
