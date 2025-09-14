use crate::stdout;
use anyhow::Result;
use immargs::immargs;

immargs! {
    VersionArgs,
    -h --help "print help message",
}

pub fn main(_args: VersionArgs) -> Result<()> {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    stdout!("patchkeeper {VERSION}\n");
    Ok(())
}
