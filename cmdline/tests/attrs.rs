use cmdline::CmdLine;
use std::ffi::OsString;

macro_rules! from_args {
    () => {{
        Args::from_args("test", Vec::<OsString>::new())
    }};
    ($($args:tt)*) => {{
        let args = [$($args)*].iter().map(OsString::from).collect::<Vec<_>>();
        Args::from_args("test", args)
    }};
}

#[test]
fn attr_field_short() {
    #[derive(CmdLine)]
    struct Args {
        #[cmdline(short = 'x')]
        value: String,
    }

    let a = from_args!("-x", "hello");
    assert!(a.value == "hello");
}

#[test]
fn attr_field_long() {
    #[derive(CmdLine)]
    struct Args {
        #[cmdline(long = "xxx")]
        value: String,
    }

    let a = from_args!("--xxx", "hello");
    assert!(a.value == "hello");
}

#[test]
fn attr_field_positional() {
    #[derive(CmdLine)]
    struct Args {
        #[cmdline(positional)]
        value: String,
    }

    let a = from_args!("hello");
    assert!(a.value == "hello");
}

#[test]
fn attr_field_conflict() {
    #[derive(CmdLine)]
    struct Args {
        #[cmdline(conflict = "days_or_weeks")]
        days: Option<()>,

        #[cmdline(conflict = "days_or_weeks")]
        weeks: Option<()>,

        #[cmdline(conflict = "fast_or_slow")]
        fast: Option<()>,

        #[cmdline(conflict = "fast_or_slow")]
        slow: Option<()>,
    }

    let a = from_args!();
    assert!(a.days.is_none());
    assert!(a.weeks.is_none());
    assert!(a.fast.is_none());
    assert!(a.slow.is_none());

    let a = from_args!("-d");
    assert!(a.days.is_some());
    assert!(a.weeks.is_none());
    assert!(a.fast.is_none());
    assert!(a.slow.is_none());

    let a = from_args!("-f");
    assert!(a.days.is_none());
    assert!(a.weeks.is_none());
    assert!(a.fast.is_some());
    assert!(a.slow.is_none());

    let a = from_args!("-d", "-s");
    assert!(a.days.is_some());
    assert!(a.weeks.is_none());
    assert!(a.fast.is_none());
    assert!(a.slow.is_some());

    let a = from_args!("-w", "-f");
    assert!(a.days.is_none());
    assert!(a.weeks.is_some());
    assert!(a.fast.is_some());
    assert!(a.slow.is_none());
}

#[test]
fn attr_field_choice() {
    #[derive(CmdLine)]
    struct Args {
        #[cmdline(choice = "days_or_weeks")]
        days: Option<()>,

        #[cmdline(choice = "days_or_weeks")]
        weeks: Option<()>,

        #[cmdline(choice = "fast_or_slow")]
        fast: Option<()>,

        #[cmdline(choice = "fast_or_slow")]
        slow: Option<()>,
    }

    let a = from_args!("-d", "-s");
    assert!(a.days.is_some());
    assert!(a.weeks.is_none());
    assert!(a.fast.is_none());
    assert!(a.slow.is_some());

    let a = from_args!("-w", "-f");
    assert!(a.days.is_none());
    assert!(a.weeks.is_some());
    assert!(a.fast.is_some());
    assert!(a.slow.is_none());
}

#[test]
fn attr_variant_name() {
    #[derive(CmdLine)]
    enum Color {
        #[cmdline(name = "RED")]
        Red,

        #[cmdline(name = "GREEN")]
        Green,

        #[cmdline(name = "BLUE")]
        Blue,
    }

    #[derive(CmdLine)]
    struct Args {
        color: Color,
    }

    let a = from_args!("-c", "RED");
    assert!(matches!(a.color, Color::Red));

    let a = from_args!("-c", "GREEN");
    assert!(matches!(a.color, Color::Green));

    let a = from_args!("-c", "BLUE");
    assert!(matches!(a.color, Color::Blue));
}

#[test]
fn attr_variant_alias() {
    #[derive(CmdLine)]
    enum Color {
        #[cmdline(alias = "RED0", alias = "RED1", alias = "RED2")]
        Red,

        #[cmdline(alias = "GREEN0", alias = "GREEN1")]
        Green,

        #[cmdline(alias = "BLUE0")]
        Blue,
    }

    #[derive(CmdLine)]
    struct Args {
        color: Color,
    }

    let a = from_args!("-c", "RED0");
    assert!(matches!(a.color, Color::Red));

    let a = from_args!("-c", "RED1");
    assert!(matches!(a.color, Color::Red));

    let a = from_args!("-c", "RED2");
    assert!(matches!(a.color, Color::Red));

    let a = from_args!("-c", "GREEN0");
    assert!(matches!(a.color, Color::Green));

    let a = from_args!("-c", "GREEN1");
    assert!(matches!(a.color, Color::Green));

    let a = from_args!("-c", "BLUE0");
    assert!(matches!(a.color, Color::Blue));
}
