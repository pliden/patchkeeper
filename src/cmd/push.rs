use crate::cmd;
use crate::meta::Metadata;
use crate::print;
use crate::repo::RepositoryUtils;
use crate::repo::HEAD;
use anyhow::bail;
use anyhow::Result;
use git2::Commit;
use git2::Repository;
use git2::Signature;
use gumdrop::Options;
use std::path::Path;
use std::str;

#[derive(Options)]
pub struct Args {
    #[options(help = "Push all commits")]
    all: bool,

    #[options(long = "move", meta = "<revspec>", help = "Move commit")]
    move_: Option<String>,

    #[options(meta = "<revspec>", help = "Graft commit")]
    graft: Option<String>,

    #[options(meta = "<revspec>", help = "Backout commit")]
    backout: Option<String>,

    #[options(help = "Print help message")]
    help: bool,

    #[options(free, help = "[<revspec>]")]
    revspec: Option<String>,
}

#[derive(Copy, Clone)]
enum PushOp {
    Normal,
    Graft,
    Backout,
}

fn fast_forward(repo: &Repository, meta: &Metadata, commit: &Commit, op: PushOp) -> Result<bool> {
    if !matches!(op, PushOp::Normal) {
        return Ok(false);
    }

    let head = repo.head()?.peel_to_commit()?;
    let parent = commit.parent(0).unwrap();

    if head.id() != parent.id() {
        return Ok(false);
    }

    print::commit_action(commit, "push")?;
    repo.reset_hard(commit)?;

    let name = repo.head_name()?;
    let mut branch = meta.branches.acquire(&name);
    branch.hidden.remove(commit.id());
    branch.popped.remove(commit.id());
    branch.pushed.add_top(commit.id());
    meta.branches.release(branch);

    meta.commit(repo, "push (fast-forward)")?;

    Ok(true)
}

fn new(repo: &Repository, meta: &Metadata, commit: &Commit, op: PushOp) -> Result<()> {
    let update_ref = Some(HEAD);
    let author = Signature::now(
        commit.author().name().unwrap_or("<unknown>"),
        commit.author().email().unwrap_or("<unknown>"),
    )?;
    let committer = Signature::now(
        commit.committer().name().unwrap_or("<unknown>"),
        commit.committer().email().unwrap_or("<unknown>"),
    )?;
    let message = commit.message().unwrap_or("<empty>");
    let message = match op {
        PushOp::Backout => format!("Backout: {message}"),
        _ => message.to_string(),
    };
    let mut index = repo.index()?;
    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;
    let parent = repo.head()?.peel_to_commit()?;
    let parents = [&parent];
    let oid = repo.commit(update_ref, &author, &committer, &message, &tree, &parents)?;

    let name = repo.head_name()?;
    let mut branch = meta.branches.acquire(&name);
    if matches!(op, PushOp::Normal) {
        branch.hidden.remove(commit.id());
        branch.popped.remove(commit.id());
    }
    branch.pushed.add_top(oid);
    meta.branches.release(branch);

    meta.commit_with_undo(repo, "push (new)")
}

fn refresh(repo: &Repository, meta: &Metadata) -> Result<()> {
    let mut index = repo.index()?;
    let oid = repo.amend_head(&mut index)?;
    let commit = repo.find_commit(oid)?;
    print::commit_action(&commit, "push")?;

    let name = repo.head_name()?;
    let mut branch = meta.branches.acquire(&name);
    branch.pushed.replace_top(oid);
    meta.branches.release(branch);

    meta.commit(repo, "push (refresh)")
}

fn not_fast_forward(
    repo: &Repository,
    meta: &Metadata,
    commit: &Commit,
    op: PushOp,
) -> Result<bool> {
    new(repo, meta, commit, op)?;

    if matches!(op, PushOp::Backout) {
        repo.revert(commit, None)?;
    } else {
        repo.cherrypick(commit, None)?;
    }

    let index = repo.index()?;
    if index.has_conflicts() {
        print::commit_action(commit, "push")?;
        print::conflicts(&index)?;
        Ok(false)
    } else {
        refresh(repo, meta)?;
        Ok(true)
    }
}

fn push(repo: &Repository, meta: &Metadata, commits: &[Commit], op: PushOp) -> Result<()> {
    if commits.is_empty() {
        bail!("nothing to push");
    }

    for commit in commits {
        if commit.parent_count() == 0 {
            bail!("cannot pushed initial commit");
        } else if commit.parent_count() > 1 {
            bail!("cannot push merge commit");
        }

        if fast_forward(repo, meta, commit, op)? {
            continue;
        }

        if !not_fast_forward(repo, meta, commit, op)? {
            break;
        }
    }

    Ok(())
}

fn push_revspec(repo: &Repository, meta: &Metadata, revspec: &str) -> Result<()> {
    let name = repo.head_name()?;
    let branch = meta.branches.acquire(&name);
    let commit = repo.find_commit_by_revspec(revspec)?;
    if !branch.popped.contains(commit.id()) {
        bail!("cannot push non-popped commit");
    }
    let oids = branch.popped.range_reversed(commit.id());
    let commits = repo.find_commits(&oids)?;
    meta.branches.release(branch);
    push(repo, meta, &commits, PushOp::Normal)
}

fn push_all(repo: &Repository, meta: &Metadata) -> Result<()> {
    let name = repo.head_name()?;
    let branch = meta.branches.acquire(&name);
    let oids = branch.popped.all_reversed();
    let commits = repo.find_commits(&oids)?;
    meta.branches.release(branch);
    push(repo, meta, &commits, PushOp::Normal)
}

fn push_move(repo: &Repository, meta: &Metadata, revspec: &str) -> Result<()> {
    let name = repo.head_name()?;
    let branch = meta.branches.acquire(&name);
    let commit = repo.find_commit_by_revspec(revspec)?;
    if !branch.hidden.contains(commit.id()) && !branch.popped.contains(commit.id()) {
        bail!("cannot move non-popped commit (use --graft)");
    }
    let commits = [commit];
    meta.branches.release(branch);
    push(repo, meta, &commits, PushOp::Normal)
}

fn push_graft(repo: &Repository, meta: &Metadata, revspec: &str) -> Result<()> {
    let commit = repo.find_commit_by_revspec(revspec)?;
    let commits = [commit];
    push(repo, meta, &commits, PushOp::Graft)
}

fn push_backout(repo: &Repository, meta: &Metadata, revspec: &str) -> Result<()> {
    let commit = repo.find_commit_by_revspec(revspec)?;
    let commits = [commit];
    push(repo, meta, &commits, PushOp::Backout)
}

fn push_next(repo: &Repository, meta: &Metadata) -> Result<()> {
    let name = repo.head_name()?;
    let branch = meta.branches.acquire(&name);
    let oids = branch.popped.bottom_as_vec();
    let commits = repo.find_commits(&oids)?;
    meta.branches.release(branch);
    push(repo, meta, &commits, PushOp::Normal)
}

pub fn main(path: &Path, args: Args) -> Result<()> {
    cmd::conflicting_options(&[
        ("-a", args.all),
        ("-m", args.move_.is_some()),
        ("-g", args.graft.is_some()),
        ("-b", args.backout.is_some()),
        ("<revspec>", args.revspec.is_some()),
    ])?;

    let repo = Repository::discover(path)?;
    let meta = Metadata::open(&repo)?;

    repo.ensure_no_unresolved()?;
    repo.ensure_no_unrefreshed()?;

    if let Some(revspec) = args.revspec {
        push_revspec(&repo, &meta, &revspec)
    } else if args.all {
        push_all(&repo, &meta)
    } else if let Some(revspec) = args.move_ {
        push_move(&repo, &meta, &revspec)
    } else if let Some(revspec) = args.graft {
        push_graft(&repo, &meta, &revspec)
    } else if let Some(revspec) = args.backout {
        push_backout(&repo, &meta, &revspec)
    } else {
        push_next(&repo, &meta)
    }
}
