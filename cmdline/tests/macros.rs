use cmdline::CmdLine;
use std::ffi::OsString;

macro_rules! cmdline_from_env {
    ($($fields:tt)*) => {{
        #[derive(CmdLine)]
        struct Args {
            $($fields)*
        }

        let args = ["-x"].iter().map(OsString::from).collect::<Vec<_>>();
        Args::from_args("test", args)
    }};
}

macro_rules! cmdline_from_args {
    ($args:ident, $($fields:tt)*) => {{
        #[derive(CmdLine)]
        struct Args {
            $($fields)*
        }

        Args::from_args("test", $args)
    }};
}

#[test]
fn macro_cmdline_from_env() {
    let a = cmdline_from_env!(
        #[cmdline(short = 'x')]
        value: Option<()>,
    );

    assert!(a.value.is_some());
}

#[test]
fn macro_cmdline_from_args() {
    let args = [
        OsString::from("-x"),
        OsString::from("--length"),
        OsString::from("47"),
    ];

    let a = cmdline_from_args!(args,
        #[cmdline(short = 'x')]
        flag: Option<()>,

        #[cmdline(long = "length")]
        value: Option<u64>,
    );

    assert!(a.flag.is_some());
    assert!(a.value == Some(47));
}
