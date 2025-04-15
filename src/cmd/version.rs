use crate::stdout;
use anyhow::Result;
use gumdrop::Options;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Options)]
pub struct Args {
    #[options(help = "Print help message")]
    help: bool,
}

pub fn main(_args: Args) -> Result<()> {
    stdout!("patchkeeper {VERSION}\n");
    Ok(())
}
