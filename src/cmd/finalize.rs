use crate::meta::Metadata;
use crate::print;
use crate::repo::RepositoryUtils;
use anyhow::Result;
use anyhow::bail;
use git2::Repository;
use immargs::args;
use std::path::Path;

args! {
    FinalizeArgs,
    -h --help   "print help message",
}

fn finalize(repo: &Repository, meta: &Metadata) -> Result<()> {
    let name = repo.head_name()?;
    let mut branch = meta.branches.acquire(&name);

    if branch.pushed.is_empty() {
        bail!("nothing to finalize");
    }

    for oid in branch.pushed.all() {
        let commit = repo.find_commit(oid)?;
        print::commit(&commit, "finalize")?;
    }

    branch.pushed.remove_all();
    meta.branches.release(branch);

    meta.commit(repo, "finalize")
}

pub fn main(path: &Path, _args: FinalizeArgs) -> Result<()> {
    let repo = Repository::discover(path)?;
    let meta = Metadata::open(&repo)?;

    repo.ensure_no_unresolved()?;
    repo.ensure_no_unrefreshed()?;

    finalize(&repo, &meta)
}
