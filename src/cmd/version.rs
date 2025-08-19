use crate::stdout;
use anyhow::Result;
use immargs::Args;
use immargs::immargs;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn main(args: Args) -> Result<()> {
    let _ = immargs!({ args });
    stdout!("patchkeeper {VERSION}\n");
    Ok(())
}
