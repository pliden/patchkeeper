use crate::safe_println;
use anyhow::Result;
use gumdrop::Options;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Options)]
pub struct Args {
    #[options(help = "Print help message")]
    help: bool,
}

pub fn main(_args: Args) -> Result<()> {
    safe_println!("patchkeeper {VERSION}");
    Ok(())
}
