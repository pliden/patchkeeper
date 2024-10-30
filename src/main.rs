use std::fmt;
use std::io::Write;
use std::process::ExitCode;

mod cmd;
mod meta;
mod print;
mod repo;
mod ui;

fn safe_eprint_fmt(args: fmt::Arguments<'_>) {
    let _ = std::io::stderr().write_fmt(args);
}

fn safe_print_fmt(args: fmt::Arguments<'_>) {
    let _ = std::io::stdout().write_fmt(args);
}

#[macro_export]
macro_rules! safe_eprint {
    ($($arg:tt)*) => {{
        $crate::safe_eprint_fmt(std::format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! safe_eprintln {
    () => {
        $crate::safe_eprint!("\n")
    };
    ($($arg:tt)*) => {
        $crate::safe_eprint!("{}\n", std::format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! safe_print {
    ($($arg:tt)*) => {{
        $crate::safe_print_fmt(std::format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! safe_println {
    () => {
        $crate::safe_print!("\n")
    };
    ($($arg:tt)*) => {
        $crate::safe_print!("{}\n", std::format_args!($($arg)*))
    };
}

fn main() -> ExitCode {
    match cmd::main() {
        Err(error) => {
            safe_eprintln!("error: {}", error);
            ExitCode::FAILURE
        }
        Ok(()) => ExitCode::SUCCESS,
    }
}
