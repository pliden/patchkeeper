use anyhow::anyhow;
use anyhow::Result;
use cmdline::CmdLine;
use std::env;
use std::io;
use std::path::PathBuf;

mod add;
mod bdelete;
mod bhide;
mod blist;
mod bnew;
mod brename;
mod bset;
mod bunhide;
mod clone;
mod delete;
mod exclude;
mod fetch;
mod finalize;
mod fold;
mod hide;
mod include;
mod init;
mod list;
mod message;
mod move_;
mod new;
mod pop;
mod push;
mod refresh;
mod remove;
mod reset;
mod resolve;
mod show;
mod unhide;
mod version;

#[derive(CmdLine)]
struct Args {
    #[cmdline(meta = "path", help = "Path to repository")]
    repo: Option<PathBuf>,

    #[cmdline(positional, variants = "commands")]
    command: Command,
}

#[derive(CmdLine)]
enum Command {
    #[cmdline(help = "Initialize repository")]
    Init(init::Args),

    #[cmdline(help = "Clone repository")]
    Clone(clone::Args),

    #[cmdline(help = "Fetch remote commit(s)")]
    Fetch(fetch::Args),

    #[cmdline(help = "Pull remote commit(s)")]
    Pull(fetch::Args),

    #[cmdline(alias = "bn", help = "New branch")]
    Bnew(bnew::Args),

    #[cmdline(alias = "b", help = "Set branch")]
    Bset(bset::Args),

    #[cmdline(alias = "br", help = "Rename branch")]
    Brename(brename::Args),

    #[cmdline(alias = "bd", help = "Delete branch")]
    Bdelete(bdelete::Args),

    #[cmdline(alias = "bls", alias = "bl", help = "List branches")]
    Blist(blist::Args),

    #[cmdline(help = "Hide branch")]
    Bhide(bhide::Args),

    #[cmdline(help = "Unhide branch")]
    Bunhide(bunhide::Args),

    #[cmdline(alias = "n", help = "New commit")]
    New(new::Args),

    #[cmdline(alias = "del", help = "Delete commit")]
    Delete(delete::Args),

    #[cmdline(alias = "r", help = "Refresh commit")]
    Refresh(refresh::Args),

    #[cmdline(alias = "msg", alias = "m", help = "Set commit message")]
    Message(message::Args),

    #[cmdline(alias = "fin", help = "Finalize commit(s)")]
    Finalize(finalize::Args),

    #[cmdline(alias = "a", help = "Add file(s)")]
    Add(add::Args),

    #[cmdline(alias = "rm", help = "Remove file(s)")]
    Remove(remove::Args),

    #[cmdline(alias = "mv", help = "Move file(s)")]
    Move(move_::Args),

    #[cmdline(alias = "i", help = "Include file(s) in commit")]
    Include(include::Args),

    #[cmdline(alias = "x", help = "Exclude file(s) from commit")]
    Exclude(exclude::Args),

    #[cmdline(alias = "pu", help = "Push commit")]
    Push(push::Args),

    #[cmdline(alias = "po", help = "Pop commit")]
    Pop(pop::Args),

    #[cmdline(help = "Fold commit")]
    Fold(fold::Args),

    #[cmdline(help = "Hide commit")]
    Hide(hide::Args),

    #[cmdline(help = "Unhide commit")]
    Unhide(unhide::Args),

    #[cmdline(alias = "ls", alias = "l", help = "List commits")]
    List(list::Args),

    #[cmdline(alias = "res", help = "Resolve merge conflict")]
    Resolve(resolve::Args),

    #[cmdline(help = "Reset head")]
    Reset(reset::Args),

    #[cmdline(alias = "s", help = "Show commit")]
    Show(show::Args),

    #[cmdline(help = "Show version")]
    Version(version::Args),
}

fn format_error(result: Result<()>) -> Result<()> {
    match result {
        Err(error) => {
            if let Some(error) = error.downcast_ref::<io::Error>() {
                let message = error.to_string();
                let trimmed = message
                    .split_once(" (os error")
                    .map(|(first, _)| first)
                    .unwrap_or(&message)
                    .to_lowercase();
                Err(anyhow!(trimmed))
            } else if let Some(error) = error.downcast_ref::<git2::Error>() {
                let trimmed = error.message().trim_end_matches('.').to_string();
                Err(anyhow!(trimmed))
            } else {
                Err(error)
            }
        }
        _ => result,
    }
}

pub fn main() -> Result<()> {
    let args = Args::from_env("pk");

    let path = args.repo.unwrap_or(env::current_dir()?);

    format_error(match args.command {
        Command::Init(args) => init::main(&path, args),
        Command::Clone(args) => clone::main(&path, args),
        Command::Fetch(args) => fetch::main(&path, args),
        Command::Pull(args) => fetch::pull_main(&path, args),
        Command::Bnew(args) => bnew::main(&path, args),
        Command::Bset(args) => bset::main(&path, args),
        Command::Brename(args) => brename::main(&path, args),
        Command::Bdelete(args) => bdelete::main(&path, args),
        Command::Blist(args) => blist::main(&path, args),
        Command::Bhide(args) => bhide::main(&path, args),
        Command::Bunhide(args) => bunhide::main(&path, args),
        Command::New(args) => new::main(&path, args),
        Command::Delete(args) => delete::main(&path, args),
        Command::Refresh(args) => refresh::main(&path, args),
        Command::Message(args) => message::main(&path, args),
        Command::Finalize(args) => finalize::main(&path, args),
        Command::Add(args) => add::main(&path, args),
        Command::Remove(args) => remove::main(&path, args),
        Command::Move(args) => move_::main(&path, args),
        Command::Include(args) => include::main(&path, args),
        Command::Exclude(args) => exclude::main(&path, args),
        Command::Push(args) => push::main(&path, args),
        Command::Pop(args) => pop::main(&path, args),
        Command::Fold(args) => fold::main(&path, args),
        Command::Hide(args) => hide::main(&path, args),
        Command::Unhide(args) => unhide::main(&path, args),
        Command::List(args) => list::main(&path, args),
        Command::Resolve(args) => resolve::main(&path, args),
        Command::Reset(args) => reset::main(&path, args),
        Command::Show(args) => show::main(&path, args),
        Command::Version(args) => version::main(args),
    })
}
