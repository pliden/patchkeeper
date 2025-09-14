use crate::meta::Metadata;
use crate::repo::HEAD;
use crate::repo::RepositoryUtils;
use anyhow::Result;
use anyhow::bail;
use git2::Repository;
use git2::Signature;
use immargs::immargs;
use std::path::Path;

immargs! {
    RefreshArgs,
    -r --resolve     "mark all merge conflicts as resolved",
    -a --author      "update author",
    -c --committer   "update comitter",
    -h --help        "print help message",
}

fn refresh(repo: &Repository, meta: &Metadata, author: bool, committer: bool) -> Result<()> {
    let name = repo.head_name()?;
    let mut branch = meta.branches.acquire(&name);

    if branch.pushed.is_empty() {
        bail!("nothing to refresh");
    }

    let commit = repo.head()?.peel_to_commit()?;
    let update_ref = Some(HEAD);
    let me = repo.signature()?;
    let author_now = Signature::now(
        commit.author().name().unwrap_or("<unknown>"),
        commit.author().email().unwrap_or("<unknown>"),
    )?;
    let committer_now = Signature::now(
        commit.committer().name().unwrap_or("<unknown>"),
        commit.committer().email().unwrap_or("<unknown>"),
    )?;
    let author = Some(if author { &me } else { &author_now });
    let committer = Some(if committer { &me } else { &committer_now });
    let encoding = None;
    let message = None;
    let mut index = repo.index()?;
    index.update_all(["*"].iter(), None)?;
    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;
    let tree = Some(&tree);
    let oid = commit.amend(update_ref, author, committer, encoding, message, tree)?;
    index.write()?;

    branch.pushed.replace_top(oid);

    meta.branches.release(branch);
    meta.commit(repo, "refresh")
}

pub fn main(path: &Path, args: RefreshArgs) -> Result<()> {
    let repo = Repository::discover(path)?;
    let meta = Metadata::open(&repo)?;

    if !args.resolve {
        repo.ensure_no_unresolved()?;
    }

    refresh(&repo, &meta, args.author, args.committer)
}
