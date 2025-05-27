use crate::meta::Metadata;
use crate::repo::RepositoryUtils;
use crate::repo::HEAD;
use anyhow::Result;
use cmdline::CmdLine;
use git2::Repository;
use std::path::Path;

#[derive(CmdLine)]
pub struct Args {
    #[cmdline(positional)]
    message: Vec<String>,
}

fn message(repo: &Repository, meta: &Metadata, message: &[String]) -> Result<()> {
    let name = repo.head_name()?;
    let mut branch = meta.branches.acquire(&name);

    let commit = repo.head()?.peel_to_commit()?;
    let update_ref = Some(HEAD);
    let author = None;
    let committer = None;
    let encoding = None;
    let joined_message = message.join(" ");
    let message = Some(joined_message.as_str());
    let tree = None;
    let oid = commit.amend(update_ref, author, committer, encoding, message, tree)?;

    branch.pushed.replace_top(oid);

    meta.branches.release(branch);
    meta.commit(repo, "message")
}

pub fn main(path: &Path, args: Args) -> Result<()> {
    let repo = Repository::discover(path)?;
    let meta = Metadata::open(&repo)?;

    repo.ensure_no_unresolved()?;

    message(&repo, &meta, &args.message)
}
