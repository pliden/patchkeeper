#[macro_export]
macro_rules! stdout {
    ($($arg:tt)*) => {{
        use std::io::Write;
        let _ = write!(std::io::stdout(), $($arg)*);
    }};
}

#[macro_export]
macro_rules! stderr {
    ($($arg:tt)*) => {{
        use std::io::Write;
        let _ = write!(std::io::stderr(), $($arg)*);
    }};
}
