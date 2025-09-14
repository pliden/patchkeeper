use crate::meta::Metadata;
use crate::repo::RepositoryUtils;
use anyhow::Result;
use anyhow::bail;
use git2::Repository;
use immargs::immargs;
use std::path::Path;

immargs! {
    ResetArgs,
    -h --help "print help message",
    <revspec> String,
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

pub fn main(path: &Path, args: ResetArgs) -> Result<()> {
    let repo = Repository::discover(path)?;
    let meta = Metadata::open(&repo)?;

    repo.ensure_no_unresolved()?;
    repo.ensure_no_unrefreshed()?;

    reset(&repo, &meta, &args.revspec)
}
