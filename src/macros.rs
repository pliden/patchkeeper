#[macro_export]
macro_rules! stdout {
    ($($args:tt)*) => {{
        use std::io::Write;
        let mut stdout = std::io::stdout().lock();
        let _ = write!(stdout, $($args)*);
    }};
}

#[macro_export]
macro_rules! stderr {
    ($($args:tt)*) => {{
        use std::io::Write;
        let mut stderr = std::io::stderr().lock();
        let _ = write!(stderr, $($args)*);
    }};
}
