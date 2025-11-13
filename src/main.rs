use std::process::ExitCode;

mod cmd;
mod error;
mod macros;
mod meta;
mod print;
mod repo;
mod ui;

fn main() -> ExitCode {
    match error::format(cmd::main()) {
        Err(error) => {
            stderr!("error: {}\n", error);
            ExitCode::FAILURE
        }
        Ok(()) => ExitCode::SUCCESS,
    }
}
