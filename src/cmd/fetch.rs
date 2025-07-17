use crate::meta::Metadata;
use crate::repo;
use crate::repo::BranchUtils;
use crate::repo::CommitUtils;
use crate::repo::RepositoryUtils;
use crate::stdout;
use anyhow::bail;
use anyhow::Result;
use colored::Colorize;
use git2::Branch;
use git2::BranchType;
use git2::FetchOptions;
use git2::RemoteCallbacks;
use git2::Repository;
use immargs::ImmArgs;
use std::io;
use std::io::Write;
use std::path::Path;
use std::str;

#[derive(ImmArgs)]
pub struct Args {
    #[cmdline(positional)]
    remote: Option<String>,

    #[cmdline(positional)]
    refspecs: Option<Vec<String>>,
}

fn fetch(repo: &Repository, remote: &str, refspecs: &[String]) -> Result<()> {
    let mut remote = repo
        .find_remote(remote)
        .or_else(|_| repo.remote_anonymous(remote))?;

    stdout!("fetch: {}\n", remote.url().unwrap_or("<unknown>"));

    let mut remote_messages = vec![];
    let mut downloading = true;
    let mut updated_references = 0;
    let mut remote_cb = RemoteCallbacks::new();

    remote_cb.sideband_progress(|data| {
        let message = str::from_utf8(data).unwrap_or("???\n").to_string();
        remote_messages.push(message);
        true
    });

    remote_cb.transfer_progress(|stats| {
        if downloading {
            let received = stats.received_objects();
            let total = stats.total_objects();
            let percent = (100 * received) / total;
            stdout!("downloading..... {}%\r", percent);
            if received == total {
                downloading = false;
                stdout!("\n");
            }
        } else {
            let indexed = stats.indexed_objects();
            let total = stats.total_objects();
            let percent = (100 * indexed) / total;
            stdout!("indexing..... {}%\r", percent);
            if indexed == total {
                stdout!("\n");
            }
        }

        let _ = io::stdout().flush();
        true
    });

    remote_cb.update_tips(|refname, from_oid, to_oid| {
        let reference = repo.find_reference(refname).unwrap();
        let branch = Branch::wrap(reference);
        let name = branch.short_name().unwrap();

        let to = repo.find_commit(to_oid).unwrap().short_id().unwrap();

        if from_oid.is_zero() {
            stdout!("{} {}\n", name.bold().yellow(), to);
        } else {
            let from = repo.find_commit(from_oid).unwrap().short_id().unwrap();
            stdout!("{} {} -> {}\n", name.bold().yellow(), from, to);
        }

        updated_references += 1;
        true
    });

    let result = remote.fetch(
        &refspecs.iter().map(String::as_str).collect::<Vec<_>>(),
        Some(FetchOptions::new().remote_callbacks(remote_cb)),
        Some("fetch"),
    );

    if let Err(error) = result {
        for message in remote_messages {
            stdout!("remote: {}", message);
        }

        bail!(error);
    }

    if remote.stats().total_objects() == 0 && updated_references == 0 {
        stdout!("nothing to fetch\n");
    }

    Ok(())
}

pub fn main(path: &Path, args: Args) -> Result<()> {
    let repo = Repository::discover(path)?;
    let remote = args.remote.as_deref().unwrap_or(repo::ORIGIN);

    fetch(&repo, remote, &args.refspecs.unwrap_or_default())
}

fn pull(repo: &Repository, meta: &Metadata, remote: &str, refspecs: &[String]) -> Result<()> {
    let name = repo.head_name()?;
    let branch = meta.branches.acquire(&name);

    if !branch.pushed.is_empty() {
        bail!("cannot have pushed commits");
    }

    fetch(repo, remote, refspecs)?;

    let branch = repo.find_branch(&name, BranchType::Local)?;
    let head = branch.get().peel_to_commit()?;
    let remote_head = branch.upstream()?.get().peel_to_commit()?;

    if head.id() == remote_head.id() {
        stdout!("nothing to pull\n");
    } else {
        stdout!(
            "{} {} -> {}\n",
            name.bold().yellow(),
            head.short_id()?,
            remote_head.short_id()?
        );

        repo.reset_hard(&remote_head)?;
    }

    Ok(())
}

pub fn pull_main(path: &Path, args: Args) -> Result<()> {
    let repo = Repository::discover(path)?;
    let meta = Metadata::open(&repo)?;
    let remote = args.remote.as_deref().unwrap_or(repo::ORIGIN);

    repo.ensure_no_unresolved()?;
    repo.ensure_no_unrefreshed()?;

    pull(&repo, &meta, remote, &args.refspecs.unwrap_or_default())
}
