use crate::meta::Metadata;
use crate::repo::RepositoryUtils;
use anyhow::bail;
use anyhow::Result;
use git2::Repository;
use immargs::ImmArgs;
use std::path::Path;

#[derive(ImmArgs)]
pub struct Args {
    #[arg(positional)]
    revspec: String,
}

fn reset(repo: &Repository, meta: &Metadata, revspec: &str) -> Result<()> {
    let name = repo.head_name()?;
    let branch = meta.branches.acquire(&name);

    if !branch.pushed.is_empty() {
        bail!("cannot reset with pushed commits");
    }

    let commit = repo.find_commit_by_revspec(revspec)?;
    repo.reset_hard(&commit)
}

pub fn main(path: &Path, args: Args) -> Result<()> {
    let repo = Repository::discover(path)?;
    let meta = Metadata::open(&repo)?;

    repo.ensure_no_unresolved()?;
    repo.ensure_no_unrefreshed()?;

    reset(&repo, &meta, &args.revspec)
}
