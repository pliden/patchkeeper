use anyhow::Result;
use anyhow::anyhow;
use immargs::immargs;
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
    let args = immargs!(
        { bin: "pk" }
        -r --repo <path> PathBuf   "Path to repository",
        <command> [...]            "Command" {
            init                   "Initialize repository",
            clone                  "Clone repository",
            fetch                  "Fetch remote commit(s)",
            pull                   "Pull remote commit(s)",
            bnew bn                "New branch",
            bset b                 "Set branch",
            brename br             "Rename branch",
            bdelete bd             "Delete branch",
            blist bls bl           "List branches",
            bhide                  "Hide branch",
            bunhde                 "Unhide branch",
            new n                  "New commit",
            delete del             "Delete commit",
            refresh r              "Refresh commit",
            message msg m          "Set commit message",
            finalize fin           "Finalize commit(s)",
            add a                  "Add file(s)",
            remove rm              "Remove file(s)",
            move_ mv               "Move file(s)",
            include i              "Include file(s) in commit",
            exclude x              "Exclude file(s) from commit",
            push pu                "Push commit",
            pop po                 "Pop commit",
            fold                   "Fold commit",
            hide                   "Hide commit",
            unhide                 "Unhide commit",
            list ls l              "List commits",
            resolve res            "Resolve merge conflict",
            reset                  "Reset head",
            show s                 "Show commit",
            version                "Show version",
        }
    );

    let path = args.repo.unwrap_or(env::current_dir()?);

    format_error(match args.command {
        ("init", args) => init::main(&path, args),
        ("clone", args) => clone::main(&path, args),
        ("fetch", args) => fetch::main(&path, args),
        ("pull", args) => fetch::pull_main(&path, args),
        ("bnew", args) => bnew::main(&path, args),
        ("bset", args) => bset::main(&path, args),
        ("brename", args) => brename::main(&path, args),
        ("bdelete", args) => bdelete::main(&path, args),
        ("blist", args) => blist::main(&path, args),
        ("bhide", args) => bhide::main(&path, args),
        ("bunhide", args) => bunhide::main(&path, args),
        ("new", args) => new::main(&path, args),
        ("delete", args) => delete::main(&path, args),
        ("refresh", args) => refresh::main(&path, args),
        ("message", args) => message::main(&path, args),
        ("finalize", args) => finalize::main(&path, args),
        ("add", args) => add::main(&path, args),
        ("remove", args) => remove::main(&path, args),
        ("move", args) => move_::main(&path, args),
        ("include", args) => include::main(&path, args),
        ("exclude", args) => exclude::main(&path, args),
        ("push", args) => push::main(&path, args),
        ("pop", args) => pop::main(&path, args),
        ("fold", args) => fold::main(&path, args),
        ("hide", args) => hide::main(&path, args),
        ("unhide", args) => unhide::main(&path, args),
        ("list", args) => list::main(&path, args),
        ("resolve", args) => resolve::main(&path, args),
        ("reset", args) => reset::main(&path, args),
        ("show", args) => show::main(&path, args),
        ("version", args) => version::main(args),
        _ => unreachable!(),
    })
}
