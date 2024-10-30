use crate::repo::CommitUtils;
use crate::repo::IndexUtils;
use crate::safe_println;
use anyhow::Result;
use colored::Colorize;
use git2::Commit;
use git2::Index;

pub fn branch_action(name: &str, action: &str) {
    safe_println!("{}: {}", action, name.bold().yellow());
}

pub fn commit_action(commit: &Commit, action: &str) -> Result<()> {
    let commit_short_id = commit.short_id()?;
    let commit_summary = commit.summary().unwrap_or_default();

    safe_println!(
        "{}: {} {}",
        action,
        commit_short_id.bold().green(),
        commit_summary
    );

    Ok(())
}

pub fn conflicts(index: &Index) -> Result<()> {
    safe_println!("{}", "merge conflict(s):".bold().red());
    for path in index.unresolved_conflicts()? {
        safe_println!("{}", path);
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
