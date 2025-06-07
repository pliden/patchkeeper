use cmdline::CmdLine;
use std::ffi::OsString;
use std::path::PathBuf;

macro_rules! from_args {
    ($($args:tt)*) => {{
        let args = [$($args)*].iter().map(OsString::from).collect::<Vec<_>>();
        Args::from_args("test", args)
    }};
}

#[test]
fn type_bool() {
    #[derive(CmdLine)]
    struct Args {
        value: bool,
    }

    let a = from_args!("-v", "true");
    assert!(a.value == true);
}

#[test]
fn type_char() {
    #[derive(CmdLine)]
    struct Args {
        value: char,
    }

    let a = from_args!("-v", "X");
    assert!(a.value == 'X');
}

#[test]
fn type_u8() {
    #[derive(CmdLine)]
    struct Args {
        value: u8,
    }

    let a = from_args!("-v", "47");
    assert!(a.value == 47);
}

#[test]
fn type_u16() {
    #[derive(CmdLine)]
    struct Args {
        value: u16,
    }

    let a = from_args!("-v", "47");
    assert!(a.value == 47);
}

#[test]
fn type_u32() {
    #[derive(CmdLine)]
    struct Args {
        value: u32,
    }

    let a = from_args!("-v", "47");
    assert!(a.value == 47);
}

#[test]
fn type_u64() {
    #[derive(CmdLine)]
    struct Args {
        value: u64,
    }

    let a = from_args!("-v", "47");
    assert!(a.value == 47);
}

#[test]
fn type_u128() {
    #[derive(CmdLine)]
    struct Args {
        value: u128,
    }

    let a = from_args!("-v", "47");
    assert!(a.value == 47);
}

#[test]
fn type_i8() {
    #[derive(CmdLine)]
    struct Args {
        value: i8,
    }

    let a = from_args!("-v", "-47");
    assert!(a.value == -47);
}

#[test]
fn type_i16() {
    #[derive(CmdLine)]
    struct Args {
        value: i16,
    }

    let a = from_args!("-v", "-47");
    assert!(a.value == -47);
}

#[test]
fn type_i32() {
    #[derive(CmdLine)]
    struct Args {
        value: i32,
    }

    let a = from_args!("-v", "-47");
    assert!(a.value == -47);
}

#[test]
fn type_i64() {
    #[derive(CmdLine)]
    struct Args {
        value: i64,
    }

    let a = from_args!("-v", "-47");
    assert!(a.value == -47);
}

#[test]
fn type_i128() {
    #[derive(CmdLine)]
    struct Args {
        value: i128,
    }

    let a = from_args!("-v", "-47");
    assert!(a.value == -47);
}

#[test]
fn type_f32() {
    #[derive(CmdLine)]
    struct Args {
        value: f32,
    }

    let a = from_args!("-v", "1.25");
    assert!(a.value == 1.25);
}

#[test]
fn type_f64() {
    #[derive(CmdLine)]
    struct Args {
        value: f64,
    }

    let a = from_args!("-v", "1.25");
    assert!(a.value == 1.25);
}

#[test]
fn type_string() {
    #[derive(CmdLine)]
    struct Args {
        value: String,
    }

    let a = from_args!("-v", "hello");
    assert!(a.value == "hello");
}

#[test]
fn type_os_string() {
    #[derive(CmdLine)]
    struct Args {
        value: OsString,
    }

    let a = from_args!("-v", "hello");
    assert!(a.value == "hello");
}

#[test]
fn type_pathbuf() {
    #[derive(CmdLine)]
    struct Args {
        value: PathBuf,
    }

    let a = from_args!("-v", "hello");
    assert!(a.value.to_str().unwrap() == "hello");
}

#[test]
fn type_option_unit() {
    #[derive(CmdLine)]
    struct Args {
        value: Option<()>,
    }

    let a = from_args!("-v");
    assert!(a.value.is_some());
}

#[test]
fn type_option_string() {
    #[derive(CmdLine)]
    struct Args {
        value: Option<String>,
    }

    let a = from_args!("-v", "hello");
    assert!(a.value.unwrap() == "hello");
}

#[test]
fn type_vec_string() {
    #[derive(CmdLine)]
    struct Args {
        value: Vec<String>,
    }

    let a = from_args!("-v", "hello", "-v", "world");
    let value = a.value;
    assert!(value[0] == "hello");
    assert!(value[1] == "world");
}

#[test]
fn type_option_vec_string() {
    #[derive(CmdLine)]
    struct Args {
        value: Option<Vec<String>>,
    }

    let a = from_args!("-v", "hello", "-v", "world");
    let value = a.value.unwrap();
    assert!(value[0] == "hello");
    assert!(value[1] == "world");
}

#[test]
fn type_positional_option_vec_string() {
    #[derive(CmdLine)]
    struct Args {
        #[cmdline(positional)]
        value: Option<Vec<String>>,
    }

    let a = from_args!("hello", "world");
    let value = a.value.unwrap();
    assert!(value[0] == "hello");
    assert!(value[1] == "world");
}

#[test]
fn type_positional_vec_string() {
    #[derive(CmdLine)]
    struct Args {
        #[cmdline(positional)]
        value: Vec<String>,
    }

    let a = from_args!("hello", "world");
    assert!(a.value[0] == "hello");
    assert!(a.value[1] == "world");
}
