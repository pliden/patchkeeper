use crate::safe_println;
use anyhow::anyhow;
use anyhow::bail;
use anyhow::Result;
use gumdrop::Options;
use gumdrop::ParsingStyle;
use itertools::Itertools;
use std::env;
use std::path::PathBuf;

mod bdelete;
mod bhide;
mod blist;
mod bnew;
mod brename;
mod bset;
mod bunhide;
mod delete;
mod finalize;
mod fold;
mod hide;
mod init;
mod list;
mod new;
mod pop;
mod push;
mod refresh;
mod reset;
mod resolve;
mod show;
mod unhide;
mod version;

#[derive(Options)]
struct Args {
    #[options(meta = "<path>", help = "Path to repository")]
    repo: Option<PathBuf>,

    #[options(help = "Print help message")]
    help: bool,

    #[options(command, required)]
    command: Option<Cmd>,
}

#[derive(Options)]
enum Cmd {
    #[options(help = "Initialize repository")]
    Init(init::Args),

    #[options(help = "New branch")]
    Bnew(bnew::Args),
    Bn(bnew::Args),

    #[options(help = "Set branch")]
    Bset(bset::Args),
    B(bset::Args),

    #[options(help = "Rename branch")]
    Brename(brename::Args),
    Br(brename::Args),

    #[options(help = "Delete branch")]
    Bdelete(bdelete::Args),
    Bd(bdelete::Args),

    #[options(help = "List branches")]
    Blist(blist::Args),
    Bls(blist::Args),
    Bl(blist::Args),

    #[options(help = "Hide branch")]
    Bhide(bhide::Args),

    #[options(help = "Unhide branch")]
    Bunhide(bunhide::Args),

    #[options(help = "New commit")]
    New(new::Args),
    N(new::Args),

    #[options(help = "Delete commit")]
    Delete(delete::Args),
    Del(delete::Args),

    #[options(help = "Refresh commit")]
    Refresh(refresh::Args),
    R(refresh::Args),

    #[options(help = "Finalize commit(s)")]
    Finalize(finalize::Args),
    Fin(finalize::Args),

    #[options(help = "Push commit")]
    Push(push::Args),
    Pu(push::Args),

    #[options(help = "Pop commit")]
    Pop(pop::Args),
    Po(pop::Args),

    #[options(help = "Fold commit")]
    Fold(fold::Args),

    #[options(help = "Hide commit")]
    Hide(hide::Args),

    #[options(help = "Unhide commit")]
    Unhide(unhide::Args),

    #[options(help = "List commits")]
    List(list::Args),
    Ls(list::Args),
    L(list::Args),

    #[options(help = "Resolve merge conflict")]
    Resolve(resolve::Args),
    Res(resolve::Args),

    #[options(help = "Reset head")]
    Reset(reset::Args),

    #[options(help = "Show commit")]
    Show(show::Args),
    S(show::Args),

    #[options(help = "Show version")]
    Version(version::Args),
}

fn parse_args() -> Result<Args> {
    let args = env::args().collect::<Vec<_>>();
    Ok(Args::parse_args(&args[1..], ParsingStyle::default())?)
}

fn print_usage(args: Args) -> Result<()> {
    fn format_cmd_list(command_list: &str) -> Vec<(String, String)> {
        let mut names_and_help: Vec<(String, String)> = vec![];

        for line in command_list.lines() {
            let name_or_alias = line.split_whitespace().next().unwrap().to_string();
            let help = line.split_whitespace().skip(1).join(" ");
            if help.is_empty() {
                let (mut names, help) = names_and_help.pop().unwrap();
                names.push_str(", ");
                names.push_str(name_or_alias.as_str());
                names_and_help.push((names, help));
            } else {
                names_and_help.push((name_or_alias, help));
            }
        }

        names_and_help
    }

    fn format_options(usage: &str) -> Vec<(String, String)> {
        let mut opts: Vec<(String, String)> = vec![];

        let mut found = false;
        for line in usage.lines() {
            if !found {
                found = line == "Optional arguments:";
                continue;
            }

            let mut option: Vec<String> = vec![];
            let mut help: Vec<String> = vec![];

            for word in line.split_whitespace() {
                if word.chars().next().unwrap().is_alphabetic() {
                    help.push(word.to_string());
                } else {
                    option.push(word.to_string());
                }
            }

            opts.push((option.join(" "), help.join(" ")));
        }

        opts
    }

    fn format_arguments(usage: &str) -> String {
        let mut args: Vec<String> = vec![];

        let mut found = false;
        for line in usage.lines() {
            if !found {
                found = line == "Positional arguments:";
                continue;
            }

            if line == "Optional arguments:" {
                break;
            }

            let arg = line.split_whitespace().skip(1).join(" ");
            args.push(arg);
        }

        args.join(" ")
    }

    let width = 23;

    if let Some(cmd_list) = args.self_command_list() {
        let cmds = format_cmd_list(cmd_list);
        let opts = format_options(args.self_usage());

        safe_println!("Usage: pk [options] <command> [command options]");
        safe_println!();
        safe_println!("Options:");
        for (option, help) in opts {
            safe_println!("  {option:width$} {help}");
        }
        safe_println!();
        safe_println!("Commands:");
        for (names, help) in cmds {
            safe_println!("  {names:width$} {help}");
        }
    } else if let Some(cmd) = args.command() {
        let cmd_name = cmd.command_name().unwrap_or("?");
        let cmd_args = format_arguments(cmd.self_usage());
        let cmd_opts = format_options(cmd.self_usage());

        if args.help {
            bail!("unexpected argument `{cmd_name}`");
        }

        safe_println!("Usage: pk {cmd_name} [options] {cmd_args}");
        safe_println!();
        safe_println!("Options:");
        for (option, help) in cmd_opts {
            safe_println!("  {option:width$} {help}");
        }
    }

    safe_println!();
    Ok(())
}

fn enabled_options<'a>(options: &[(&'a str, bool)]) -> Vec<&'a str> {
    options
        .iter()
        .filter_map(|(name, enabled)| if *enabled { Some(*name) } else { None })
        .collect::<Vec<_>>()
}

pub fn missing_option(options: &[(&str, bool)]) -> Result<()> {
    let enabled = enabled_options(options);
    if enabled.is_empty() {
        let options = options.iter().map(|(name, _)| *name).join(" or ");
        bail!("missing option: {options}");
    }

    Ok(())
}

pub fn conflicting_options(options: &[(&str, bool)]) -> Result<()> {
    let enabled = enabled_options(options);
    if enabled.len() > 1 {
        let options = enabled.join(" and ");
        bail!("conflicting options: {options}");
    }

    Ok(())
}

pub fn missing_or_conflicting_options(options: &[(&str, bool)]) -> Result<()> {
    missing_option(options)?;
    conflicting_options(options)
}

fn format_error(result: Result<()>) -> Result<()> {
    match result {
        Err(error) => match error.downcast_ref::<git2::Error>() {
            Some(error) => Err(anyhow!(error.message().trim_end_matches('.').to_string())),
            _ => Err(error),
        },
        _ => result,
    }
}

pub fn main() -> Result<()> {
    let args = parse_args()?;
    if args.help_requested() {
        print_usage(args)?;
        return Ok(());
    }

    let path = args.repo.unwrap_or(env::current_dir()?);

    format_error(match args.command.unwrap() {
        Cmd::Init(args) => init::main(&path, args),
        Cmd::Bnew(args) | Cmd::Bn(args) => bnew::main(&path, args),
        Cmd::Bset(args) | Cmd::B(args) => bset::main(&path, args),
        Cmd::Brename(args) | Cmd::Br(args) => brename::main(&path, args),
        Cmd::Bdelete(args) | Cmd::Bd(args) => bdelete::main(&path, args),
        Cmd::Blist(args) | Cmd::Bls(args) | Cmd::Bl(args) => blist::main(&path, args),
        Cmd::Bhide(args) => bhide::main(&path, args),
        Cmd::Bunhide(args) => bunhide::main(&path, args),
        Cmd::New(args) | Cmd::N(args) => new::main(&path, args),
        Cmd::Delete(args) | Cmd::Del(args) => delete::main(&path, args),
        Cmd::Refresh(args) | Cmd::R(args) => refresh::main(&path, args),
        Cmd::Finalize(args) | Cmd::Fin(args) => finalize::main(&path, args),
        Cmd::Push(args) | Cmd::Pu(args) => push::main(&path, args),
        Cmd::Pop(args) | Cmd::Po(args) => pop::main(&path, args),
        Cmd::Fold(args) => fold::main(&path, args),
        Cmd::Hide(args) => hide::main(&path, args),
        Cmd::Unhide(args) => unhide::main(&path, args),
        Cmd::List(args) | Cmd::Ls(args) | Cmd::L(args) => list::main(&path, args),
        Cmd::Resolve(args) | Cmd::Res(args) => resolve::main(&path, args),
        Cmd::Reset(args) => reset::main(&path, args),
        Cmd::Show(args) | Cmd::S(args) => show::main(&path, args),
        Cmd::Version(args) => version::main(args),
    })
}
