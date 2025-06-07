pub use cmdline_derive::CmdLine;
use lexopt;
pub use lexopt::Arg::Long;
pub use lexopt::Arg::Short;
pub use lexopt::Arg::Value;
use std::ffi::OsString;

pub enum Error {
    InvalidOption {
        option: String,
    },
    InvalidArgument {
        arg: OsString,
    },
    InvalidVariant {
        enum_name: String,
        value: String,
    },
    MissingArgument {
        alternatives: Vec<String>,
    },
    ConflictingArguments {
        arg0: String,
        arg1: String,
    },
    MissingValue {
        option: String,
    },
    UnexpectedValue {
        option: String,
        value: OsString,
    },
    NonUnicodeValue {
        value: OsString,
    },
    ParsingFailed {
        value: String,
        error: Box<dyn std::error::Error + Send + Sync + 'static>,
    },
    Help {
        commands: Vec<String>,
        message: String,
    },
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::Error::*;

        match self {
            InvalidOption { option } => {
                write!(f, "invalid option '{}'", option)
            }
            InvalidArgument { arg } => {
                write!(f, "invalid argument '{}'", arg.display())
            }
            InvalidVariant { enum_name, value } => {
                write!(f, "invalid {} '{}'", enum_name, value)
            }
            MissingArgument { alternatives } => {
                let args = alternatives
                    .iter()
                    .map(|arg| format!("'{arg}'"))
                    .collect::<Vec<_>>()
                    .join(" or ");
                write!(f, "missing argument {}", args)
            }
            ConflictingArguments { arg0, arg1 } => {
                write!(f, "conflicting arguments '{}' and '{}'", arg0, arg1)
            }
            MissingValue { option } => {
                write!(f, "missing argument for option '{}'", option)
            }
            UnexpectedValue { option, value } => {
                write!(
                    f,
                    "unexpected argument for option '{}': {}",
                    option,
                    value.display()
                )
            }
            NonUnicodeValue { value } => {
                write!(f, "argument is invalid unicode: '{}'", value.display())
            }
            ParsingFailed { value, error } => {
                write!(f, "cannot parse argument '{}': {}", value, error)
            }
            Help {
                commands: _,
                message,
            } => {
                write!(f, "{}", message)
            }
        }
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl From<lexopt::Error> for Error {
    fn from(error: lexopt::Error) -> Self {
        match error {
            lexopt::Error::MissingValue { option: None } => panic!("missing value not expected"),
            lexopt::Error::MissingValue {
                option: Some(option),
            } => Error::MissingValue { option },
            lexopt::Error::UnexpectedOption(option) => Error::InvalidOption { option },
            lexopt::Error::UnexpectedArgument(arg) => Error::InvalidArgument { arg },
            lexopt::Error::UnexpectedValue { option, value } => {
                Error::UnexpectedValue { option, value }
            }
            lexopt::Error::ParsingFailed { value, error } => Error::ParsingFailed { value, error },
            lexopt::Error::NonUnicodeValue(value) => Error::NonUnicodeValue { value },
            lexopt::Error::Custom(_) => panic!("custom lexopt error not expected"),
        }
    }
}

pub struct Context<'a> {
    commands: Vec<&'a str>,
    parser: lexopt::Parser,
}

impl<'a> Context<'a> {
    fn new<I: IntoIterator<Item = OsString>>(bin_name: &'a str, args: I) -> Self {
        Self {
            parser: lexopt::Parser::from_args(args),
            commands: vec![bin_name],
        }
    }

    pub fn add_command(&mut self, command: &'a str) {
        self.commands.push(command);
    }

    pub fn commands(&self) -> Vec<String> {
        self.commands
            .iter()
            .map(std::string::ToString::to_string)
            .collect::<Vec<_>>()
    }

    pub fn parser_next(&mut self) -> Result<Option<lexopt::Arg>, Error> {
        self.parser.next().map_err(Error::from)
    }

    pub fn parser_value(&mut self) -> Result<OsString, Error> {
        self.parser.value().map_err(Error::from)
    }
}

pub trait CmdLine: Sized {
    fn parse(context: &mut Context, value: OsString) -> Result<Self, Error>;
    fn help_usage(_buf: &mut String, _meta: &str) {}
    fn help_options(_buf: &mut String, _meta: &str) {}
}

impl<T: std::str::FromStr> CmdLine for T
where
    T::Err: Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
{
    fn parse(context: &mut Context, value: OsString) -> Result<Self, Error> {
        let _ = context;
        match value.to_str() {
            Some(value) => match std::str::FromStr::from_str(value) {
                Ok(value) => Ok(value),
                Err(error) => Err(Error::ParsingFailed {
                    value: value.to_string(),
                    error: error.into(),
                }),
            },
            None => Err(Error::NonUnicodeValue { value }),
        }
    }
}

pub fn no_value() -> OsString {
    OsString::new()
}

pub fn value_to_string(value: OsString) -> Result<String, Error> {
    match value.into_string() {
        Ok(string) => Ok(string),
        Err(value) => Err(Error::NonUnicodeValue { value }),
    }
}

pub fn parse<T: CmdLine>(context: &mut Context, value: OsString) -> Result<T, Error> {
    T::parse(context, value)
}

pub fn help<T: CmdLine>(commands: Vec<String>) -> Result<T, Error> {
    let mut buf = String::new();
    let meta = commands.join(" ");
    help_usage::<T>(&mut buf, &meta);
    help_options::<T>(&mut buf, "NONE");
    Err(Error::Help {
        commands,
        message: buf,
    })
}

pub fn help_usage<T: CmdLine>(buf: &mut String, meta: &str) {
    T::help_usage(buf, meta);
}

pub fn help_options<T: CmdLine>(buf: &mut String, meta: &str) {
    T::help_options(buf, meta);
}

pub fn make_arg<T>(value: T) -> Option<T> {
    Some(value)
}

pub fn make_option_arg<T>(value: T) -> Option<Option<T>> {
    Some(Some(value))
}

pub fn make_vec_arg<T>(vec: Option<Vec<T>>, value: T) -> Option<Vec<T>> {
    let mut vec = vec.unwrap_or_default();
    vec.push(value);
    Some(vec)
}

pub fn make_option_vec_arg<T>(vec: Option<Option<Vec<T>>>, value: T) -> Option<Option<Vec<T>>> {
    let mut vec = vec.flatten().unwrap_or_default();
    vec.push(value);
    Some(Some(vec))
}

pub fn unwrap_arg<T>(arg: Option<T>, help0: &str) -> Result<T, Error> {
    arg.ok_or(missing_argument(&[help0]))
}

pub fn unwrap_option_arg<T>(arg: Option<Option<T>>) -> Option<T> {
    arg.unwrap_or_default()
}

pub fn invalid_short(short: char) -> Error {
    Error::InvalidOption {
        option: format!("-{short}"),
    }
}

pub fn invalid_long(long: &str) -> Error {
    Error::InvalidOption {
        option: format!("--{long}"),
    }
}

pub fn invalid_argument(arg: OsString) -> Error {
    Error::InvalidArgument { arg }
}

pub fn invalid_variant(enum_name: &str, value: &str) -> Error {
    Error::InvalidVariant {
        enum_name: enum_name.to_string(),
        value: value.to_string(),
    }
}

pub fn missing_argument(args: &[&str]) -> Error {
    let alternatives = args.iter().map(|arg| arg.to_string()).collect::<Vec<_>>();
    Error::MissingArgument { alternatives }
}

pub fn conflicting_arguments(arg0: &str, arg1: &str) -> Error {
    Error::ConflictingArguments {
        arg0: arg0.to_string(),
        arg1: arg1.to_string(),
    }
}

fn exit_on_error<T: CmdLine>(value: Result<T, Error>) -> T {
    match value {
        Ok(args) => args,
        Err(error) => {
            use std::io::Write;
            let (prefix, postfix, exit_code) = match error {
                Error::Help {
                    commands: _,
                    message: _,
                } => ("", "", 0),
                _ => ("error: ", "\n", 1),
            };
            let _ = write!(std::io::stdout(), "{}{}{}", prefix, error, postfix);
            std::process::exit(exit_code);
        }
    }
}

pub fn from_args<T: CmdLine, I: IntoIterator<Item = OsString>>(bin_name: &str, args: I) -> T {
    let args = from_args_no_exit(bin_name, args);
    exit_on_error(args)
}

pub fn from_args_no_exit<T: CmdLine, I: IntoIterator<Item = OsString>>(
    bin_name: &str,
    args: I,
) -> Result<T, Error> {
    let mut context = Context::new(bin_name, args);
    parse(&mut context, no_value())
}

pub fn from_env<T: CmdLine>(bin_name: &str) -> T {
    let args = from_env_no_exit(bin_name);
    exit_on_error(args)
}

pub fn from_env_no_exit<T: CmdLine>(bin_name: &str) -> Result<T, Error> {
    let args = std::env::args_os().skip(1);
    from_args_no_exit(bin_name, args)
}
