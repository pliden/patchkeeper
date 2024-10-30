use crate::repo::BranchUtils;
use crate::repo::CommitUtils;
use crate::repo::RepositoryUtils;
use crate::repo::HEAD;
use crate::safe_print;
use crate::safe_println;
use anyhow::Result;
use colored::Colorize;
use git2::Commit;
use git2::DiffDelta;
use git2::DiffFormat;
use git2::DiffHunk;
use git2::DiffLine;
use git2::DiffLineType;
use git2::Oid;
use git2::Repository;
use gumdrop::Options;
use std::collections::HashMap;
use std::path::Path;

#[derive(Options)]
pub struct Args {
    #[options(help = "Show files only")]
    files: bool,

    #[options(help = "Print help message")]
    help: bool,

    #[options(free, help = "<revspec>")]
    revspec: Option<String>,
}

pub type BranchMap = HashMap<Oid, BranchInfo>;

pub struct BranchInfo {
    pub head: Option<String>,
    pub non_heads: Vec<String>,
}

pub fn branch_map(repo: &Repository) -> Result<BranchMap> {
    let mut map = BranchMap::default();

    for branch_and_type in repo.branches(None)? {
        let (branch, _) = branch_and_type?;
        let oid = branch.get().peel_to_commit()?.id();
        let name = branch.short_name()?;

        match map.get_mut(&oid) {
            Some(info) => match branch.is_head() {
                true => info.head = Some(name),
                false => info.non_heads.push(name),
            },
            _ => {
                map.insert(
                    oid,
                    match branch.is_head() {
                        true => BranchInfo {
                            head: Some(name),
                            non_heads: vec![],
                        },
                        false => BranchInfo {
                            head: None,
                            non_heads: vec![name],
                        },
                    },
                );
            }
        }
    }

    Ok(map)
}

fn branch_heads_on_commit(commit: &Commit, branch_map: &BranchMap) -> String {
    match branch_map.get(&commit.id()) {
        Some(info) => match (&info.head, info.non_heads.is_empty()) {
            (Some(head), false) => format!("({} -> {}, {})", HEAD, head, info.non_heads.join(", ")),
            (Some(head), true) => format!("({} -> {})", HEAD, head),
            (None, _) => format!("({})", info.non_heads.join(", ")),
        },
        _ => String::new(),
    }
}

fn print_header(repo: &Repository, commit: &Commit) -> Result<()> {
    let branch_map = branch_map(repo)?;
    let short_oid = commit.short_id()?;
    let oid = commit.id();
    let branches = branch_heads_on_commit(commit, &branch_map);
    let author = commit.author().to_string();
    let time = commit.time_local()?;
    let message = commit.message().unwrap_or_default();

    safe_println!(
        "{} {} / {} {}",
        "Commit:".bold().green(),
        short_oid.bold().red(),
        oid.to_string().bold().red(),
        branches.bold().yellow()
    );
    safe_println!("{} {}", "Author:".bold().green(), author);
    safe_println!("{} {}", "Date:  ".bold().green(), time);
    safe_println!("{}\n", message);

    Ok(())
}

fn print_diff_line(_delta: DiffDelta, _hunk: Option<DiffHunk>, line: DiffLine) -> bool {
    let origin = line.origin_value();
    let marker = match origin {
        DiffLineType::Addition => "+",
        DiffLineType::Deletion => "-",
        DiffLineType::Context => " ",
        _ => "",
    };
    let content = std::str::from_utf8(line.content()).unwrap_or("<Unable to decode content>");
    let line = format!("{marker}{content}");
    let colored_line = match origin {
        DiffLineType::Addition => line.green(),
        DiffLineType::Deletion => line.red(),
        DiffLineType::AddEOFNL => line.green(),
        DiffLineType::DeleteEOFNL => line.red(),
        DiffLineType::FileHeader => line.bold(),
        DiffLineType::HunkHeader => line.cyan(),
        _ => line.normal(),
    };

    safe_print!("{}", colored_line);

    true
}

fn print_diff(repo: &Repository, commit: &Commit, format: DiffFormat) -> Result<()> {
    let (old_tree, new_tree) = match commit.parent_count() {
        0 => (None, Some(commit.tree()?)),
        1 => (Some(commit.parent(0)?.tree()?), Some(commit.tree()?)),
        _ => (None, None),
    };

    let diff = repo.diff_tree_to_tree(old_tree.as_ref(), new_tree.as_ref(), None)?;
    diff.print(format, print_diff_line)?;

    Ok(())
}

pub fn main(path: &Path, args: Args) -> Result<()> {
    let repo = Repository::discover(path)?;
    let revspec = args.revspec.unwrap_or(String::from(HEAD));
    let commit = repo.find_commit_by_revspec(&revspec)?;

    let format = match args.files {
        true => DiffFormat::NameOnly,
        false => DiffFormat::Patch,
    };

    print_header(&repo, &commit)?;
    print_diff(&repo, &commit, format)
}
