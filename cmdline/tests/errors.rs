use cmdline::CmdLine;
use cmdline::Error;
use std::ffi::OsString;

macro_rules! from_args {
    () => {{
        let error = Args::from_args_no_exit("test", Vec::<OsString>::new())
            .err().expect("should be an error");
        println!("###\n{}\n###", error);
        error
    }};
    ($($args:tt)*) => {{
        let args = [$($args)*].iter().map(OsString::from).collect::<Vec<_>>();
        let error = Args::from_args_no_exit("test", args)
            .err().expect("should be an error");
        println!("###\n{}\n###", error);
        error
    }};
}

#[test]
fn error_invalid_option() {
    #[derive(CmdLine)]
    struct Args {}

    let error = from_args!("--invalid");
    assert!(match &error {
        Error::InvalidOption { option } => {
            option == "--invalid"
        }
        _ => false,
    });
    assert!(error.to_string() == "invalid option '--invalid'");
}

#[test]
fn error_invalid_argument() {
    #[derive(CmdLine)]
    struct Args {}

    let error = from_args!("invalid");
    assert!(match &error {
        Error::InvalidArgument { arg } => {
            arg == "invalid"
        }
        _ => false,
    });
    assert!(error.to_string() == "invalid argument 'invalid'");
}

#[test]
fn error_invalid_variant() {
    #[derive(CmdLine)]
    enum Color {
        Red,
        Blue,
        Green,
    }

    #[derive(CmdLine)]
    struct Args {
        #[allow(dead_code)]
        #[cmdline(positional)]
        color: Color,
    }

    let error = from_args!("black");
    assert!(match &error {
        Error::InvalidVariant { enum_name, value } => {
            enum_name == "color" && value == "black"
        }
        _ => false,
    });
    assert!(error.to_string() == "invalid color 'black'");
}

#[test]
fn error_missing_argument() {
    #[allow(dead_code)]
    #[derive(CmdLine)]
    struct Args {
        value: String,
    }

    let error = from_args!();
    assert!(match &error {
        Error::MissingArgument { alternatives } => {
            alternatives.len() == 1 && alternatives[0] == "--value <value>"
        }
        _ => false,
    });
    assert!(error.to_string() == "missing argument '--value <value>'");
}

#[test]
fn error_conflicting_arguments() {
    #[allow(dead_code)]
    #[derive(CmdLine)]
    struct Args {
        #[cmdline(conflict = "fast_or_slow")]
        fast: Option<()>,

        #[cmdline(conflict = "fast_or_slow")]
        slow: Option<()>,
    }

    let error = from_args!("-f", "-s");
    assert!(match &error {
        Error::ConflictingArguments { arg0, arg1 } => {
            arg0 == "--fast" && arg1 == "--slow"
        }
        _ => false,
    });
    assert!(error.to_string() == "conflicting arguments '--fast' and '--slow'");
}

#[test]
fn error_choice_missing_argument() {
    #[allow(dead_code)]
    #[derive(CmdLine)]
    struct Args {
        #[cmdline(choice = "fast_or_slow")]
        fast: Option<()>,

        #[cmdline(choice = "fast_or_slow")]
        slow: Option<()>,
    }

    let error = from_args!();
    assert!(match &error {
        Error::MissingArgument { alternatives } => {
            alternatives.len() == 2 && alternatives[0] == "--fast" && alternatives[1] == "--slow"
        }
        _ => false,
    });
    assert!(error.to_string() == "missing argument '--fast' or '--slow'");

    let error = from_args!("-f", "-s");
    assert!(match &error {
        Error::ConflictingArguments { arg0, arg1 } => {
            arg0 == "--fast" && arg1 == "--slow"
        }
        _ => false,
    });
    assert!(error.to_string() == "conflicting arguments '--fast' and '--slow'");
}
