use cmdline::CmdLine;
use cmdline::Error;
use indoc::indoc;
use std::ffi::OsString;

macro_rules! from_args {
    ($($args:tt)*) => {{
        let args = [$($args)*].iter().map(OsString::from).collect::<Vec<_>>();
        match Args::from_args_no_exit("test", args) {
            Err(Error::Help{ commands: _, message }) => {
                println!("###\n{}###", message);
                message
            },
            _ => panic!("should return a help error")
        }
    }}
}

#[test]
fn help_empty() {
    #[derive(CmdLine)]
    struct Args {}

    let help = indoc! {"
        usage: test [options]

        options:
           -h, --help     Print available options

    "};

    assert!(from_args!("-h") == help);
}

#[test]
fn help_field_attr_short() {
    #[allow(dead_code)]
    #[derive(CmdLine)]
    struct Args {
        #[cmdline(no_short)]
        aaa: Option<()>,

        #[cmdline(short = 'x')]
        bbb: Option<u64>,
    }

    let help = indoc! {"
        usage: test [options]

        options:
           --aaa
           -x, --bbb <bbb>
           -h, --help          Print available options

    "};

    assert!(from_args!("-h") == help);
}

#[test]
fn help_field_attr_long() {
    #[allow(dead_code)]
    #[derive(CmdLine)]
    struct Args {
        #[cmdline(no_long)]
        aaa: Option<()>,

        #[cmdline(long = "xxx")]
        bbb: Option<u64>,
    }

    let help = indoc! {"
        usage: test [options]

        options:
           -a
           -b, --xxx <bbb>
           -h, --help          Print available options

    "};

    assert!(from_args!("-h") == help);
}

#[test]
fn help_field_attr_positional() {
    #[allow(dead_code)]
    #[derive(CmdLine)]
    struct Args {
        #[cmdline(positional)]
        aaa: Option<u64>,

        #[cmdline(positional)]
        bbb: String,
    }

    let help = indoc! {"
        usage: test [options] [aaa] <bbb>

        options:
           -h, --help     Print available options

    "};

    assert!(from_args!("-h") == help);
}

#[test]
fn help_field_attr_positional_vec() {
    #[allow(dead_code)]
    #[derive(CmdLine)]
    struct Args {
        #[cmdline(positional)]
        aaa: Vec<String>,
    }

    let help = indoc! {"
        usage: test [options] <aaa...>

        options:
           -h, --help     Print available options

    "};

    assert!(from_args!("-h") == help);
}

#[test]
fn help_field_attr_positional_option_vec() {
    #[allow(dead_code)]
    #[derive(CmdLine)]
    struct Args {
        #[cmdline(positional)]
        aaa: Option<Vec<String>>,
    }

    let help = indoc! {"
        usage: test [options] [aaa...]

        options:
           -h, --help     Print available options

    "};

    assert!(from_args!("-h") == help);
}

#[test]
fn help_field_attr_meta_option_value() {
    #[allow(dead_code)]
    #[derive(CmdLine)]
    struct Args {
        #[cmdline(meta = "number")]
        aaa: Option<u64>,

        #[cmdline(meta = "text")]
        bbb: Option<String>,
    }

    let help = indoc! {"
        usage: test [options]

        options:
           -a, --aaa <number>
           -b, --bbb <text>
           -h, --help             Print available options

    "};

    assert!(from_args!("-h") == help);
}

#[test]
fn help_field_attr_meta_positional() {
    #[allow(dead_code)]
    #[derive(CmdLine)]
    struct Args {
        #[cmdline(positional, meta = "number")]
        aaa: u64,

        #[cmdline(positional, meta = "file")]
        bbb: String,

        #[cmdline(positional, meta = "text")]
        ccc: Option<Vec<String>>,
    }

    let help = indoc! {"
        usage: test [options] <number> <file> [text...]

        options:
           -h, --help     Print available options

    "};

    assert!(from_args!("-h") == help);
}

#[test]
fn help_field_attr_help() {
    #[allow(dead_code)]
    #[derive(CmdLine)]
    struct Args {
        #[cmdline(help = "Help A")]
        aaa: u64,

        #[cmdline(help = "Help B")]
        bbb: String,
    }

    let help = indoc! {"
        usage: test [options]

        options:
           -a, --aaa <aaa>     Help A
           -b, --bbb <bbb>     Help B
           -h, --help          Print available options

    "};

    assert!(from_args!("-h") == help);
}

#[test]
fn help_variant_option() {
    #[derive(CmdLine)]
    enum Color {
        Red,
        Green,
        Blue,
    }

    #[allow(dead_code)]
    #[derive(CmdLine)]
    struct Args {
        #[cmdline(variants = "colors")]
        aaa: Color,
        bbb: Color,
    }

    let help = indoc! {"
        usage: test [options]

        options:
           -a, --aaa <aaa>
           -b, --bbb <bbb>
           -h, --help          Print available options

        colors:
           red
           green
           blue

    "};

    assert!(from_args!("-h") == help);
}

#[test]
fn help_variant_attr_positional() {
    #[derive(CmdLine)]
    enum Color {
        Red,
        Green,
        Blue,
    }

    #[allow(dead_code)]
    #[derive(CmdLine)]
    struct Args {
        #[cmdline(positional, variants = "colors")]
        aaa: Color,

        #[cmdline(positional, variants = "colors")]
        bbb: Option<Color>,
    }

    let help = indoc! {"
        usage: test [options] <aaa> [bbb]

        options:
           -h, --help     Print available options

        colors:
           red
           green
           blue

    "};

    assert!(from_args!("-h") == help);
}

#[test]
fn help_variant_attr_positional_vec() {
    #[derive(CmdLine)]
    enum Color {
        Red,
        Green,
        Blue,
    }

    #[allow(dead_code)]
    #[derive(CmdLine)]
    struct Args {
        #[cmdline(positional, variants = "colors")]
        color: Vec<Color>,
    }

    let help = indoc! {"
        usage: test [options] <color...>

        options:
           -h, --help     Print available options

        colors:
           red
           green
           blue

    "};

    assert!(from_args!("-h") == help);
}

#[test]
fn help_variant_attr_name() {
    #[derive(CmdLine)]
    enum Color {
        #[cmdline(name = "RED")]
        Red,

        #[cmdline(name = "green")]
        Green,

        #[cmdline(name = "Blue")]
        Blue,
    }

    #[allow(dead_code)]
    #[derive(CmdLine)]
    struct Args {
        #[cmdline(variants = "colors")]
        color: Color,
    }

    let help = indoc! {"
        usage: test [options]

        options:
           -c, --color <color>
           -h, --help              Print available options

        colors:
           RED
           green
           Blue

    "};

    assert!(from_args!("-h") == help);
}

#[test]
fn help_variant_attr_alias() {
    #[derive(CmdLine)]
    enum Color {
        #[cmdline(alias = "RED")]
        Red,

        #[cmdline(alias = "GREEN", alias = "G")]
        Green,

        #[cmdline(alias = "BLUE", alias = "BL", alias = "B")]
        Blue,
    }

    #[allow(dead_code)]
    #[derive(CmdLine)]
    struct Args {
        #[cmdline(variants = "colors")]
        color: Color,
    }

    let help = indoc! {"
        usage: test [options]

        options:
           -c, --color <color>
           -h, --help              Print available options

        colors:
           red, RED
           green, GREEN, G
           blue, BLUE, BL, B

    "};

    assert!(from_args!("-h") == help);
}

#[test]
fn help_variant_attr_help() {
    #[derive(CmdLine)]
    enum Color {
        #[cmdline(help = "This is red")]
        Red,

        #[cmdline(help = "This is green")]
        Green,

        #[cmdline(help = "This is blue")]
        Blue,
    }

    #[allow(dead_code)]
    #[derive(CmdLine)]
    struct Args {
        #[cmdline(variants = "colors")]
        color: Color,
    }

    let help = indoc! {"
        usage: test [options]

        options:
           -c, --color <color>
           -h, --help              Print available options

        colors:
           red       This is red
           green     This is green
           blue      This is blue

    "};

    assert!(from_args!("-h") == help);
}
