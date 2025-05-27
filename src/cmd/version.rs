use crate::stdout;
use anyhow::Result;
use cmdline::CmdLine;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(CmdLine)]
pub struct Args {}

pub fn main(_args: Args) -> Result<()> {
    stdout!("patchkeeper {VERSION}\n");
    Ok(())
}
