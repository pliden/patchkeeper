use crate::stdout;
use anyhow::Result;
use immargs::ImmArgs;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(ImmArgs)]
pub struct Args {}

pub fn main(_args: Args) -> Result<()> {
    stdout!("patchkeeper {VERSION}\n");
    Ok(())
}
