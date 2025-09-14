use anyhow::Result;
use immargs::immargs;
use std::env::current_dir;
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

immargs! {
    MainArgs,
    -r --repo <path> PathBuf   "path to repository",
    -v --version               "print version information",
    -h --help                  "print help message",
    <command> Command {
        init                   "initialize repository",
        clone                  "clone repository",
        fetch                  "fetch remote commit(s)",
        pull                   "pull remote commit(s)",
        bnew bn                "new branch",
        bset b                 "set branch",
        brename br             "rename branch",
        bdelete bd             "delete branch",
        blist bls bl           "list branches",
        bhide                  "hide branch",
        bunhide                "unhide branch",
        new n                  "new commit",
        delete del             "delete commit",
        refresh r              "refresh commit",
        message msg m          "set commit message",
        finalize fin           "finalize commit(s)",
        add a                  "add file(s)",
        remove rm              "remove file(s)",
        move_ mv               "move file(s)",
        include i              "include file(s) in commit",
        exclude x              "exclude file(s) from commit",
        push pu                "push commit",
        pop po                 "pop commit",
        fold                   "fold commit",
        hide                   "hide commit",
        unhide                 "unhide commit",
        list ls l              "list commits",
        resolve res            "resolve merge conflict",
        reset                  "reset head",
        show s                 "show commit",
    }
}

pub fn main() -> Result<()> {
    let args = MainArgs::from_env();
    let path = args.repo.unwrap_or(current_dir()?);

    match args.command {
        Command::Init(args) => init::main(&path, args.into()),
        Command::Clone(args) => clone::main(args.into()),
        Command::Fetch(args) => fetch::main(&path, args.into()),
        Command::Pull(args) => fetch::pull_main(&path, args.into()),
        Command::Bnew(args) => bnew::main(&path, args.into()),
        Command::Bset(args) => bset::main(&path, args.into()),
        Command::Brename(args) => brename::main(&path, args.into()),
        Command::Bdelete(args) => bdelete::main(&path, args.into()),
        Command::Blist(args) => blist::main(&path, args.into()),
        Command::Bhide(args) => bhide::main(&path, args.into()),
        Command::Bunhide(args) => bunhide::main(&path, args.into()),
        Command::New(args) => new::main(&path, args.into()),
        Command::Delete(args) => delete::main(&path, args.into()),
        Command::Refresh(args) => refresh::main(&path, args.into()),
        Command::Message(args) => message::main(&path, args.into()),
        Command::Finalize(args) => finalize::main(&path, args.into()),
        Command::Add(args) => add::main(&path, args.into()),
        Command::Remove(args) => remove::main(&path, args.into()),
        Command::Move(args) => move_::main(&path, args.into()),
        Command::Include(args) => include::main(&path, args.into()),
        Command::Exclude(args) => exclude::main(&path, args.into()),
        Command::Push(args) => push::main(&path, args.into()),
        Command::Pop(args) => pop::main(&path, args.into()),
        Command::Fold(args) => fold::main(&path, args.into()),
        Command::Hide(args) => hide::main(&path, args.into()),
        Command::Unhide(args) => unhide::main(&path, args.into()),
        Command::List(args) => list::main(&path, args.into()),
        Command::Resolve(args) => resolve::main(&path, args.into()),
        Command::Reset(args) => reset::main(&path, args.into()),
        Command::Show(args) => show::main(&path, args.into()),
    }
}
