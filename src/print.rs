use crate::repo::CommitUtils;
use crate::repo::IndexUtils;
use crate::stdout;
use anyhow::Result;
use colored::Colorize;
use git2::Commit;
use git2::Index;

pub fn branch(name: &str, action: &str) {
    stdout!("{}: {}\n", action, name.bold().yellow());
}

pub fn commit(commit: &Commit, action: &str) -> Result<()> {
    let commit_short_id = commit.short_id()?;
    let commit_summary = commit.summary().unwrap_or_default();

    stdout!(
        "{}: {} {}\n",
        action,
        commit_short_id.bold().green(),
        commit_summary
    );

    Ok(())
}

pub fn conflicts(index: &Index) -> Result<()> {
    stdout!("{}\n", "merge conflict(s):".bold().red());
    for path in &index.unresolved_conflicts()? {
        stdout!("{}\n", path.display());
    }

    Ok(())
}

pub fn marker(enabled: bool) -> String {
    match enabled {
        true => "*",
        false => " ",
    }
    .to_string()
}
