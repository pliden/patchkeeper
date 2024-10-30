use crate::meta::Metadata;
use crate::repo::RepositoryUtils;
use anyhow::Result;
use git2::Repository;
use gumdrop::Options;
use std::path::Path;

#[derive(Options)]
pub struct Args {
    #[options(help = "Print help message")]
    help: bool,
}

pub fn main(path: &Path, _args: Args) -> Result<()> {
    let repo = Repository::initialize(path)?;
    let meta = Metadata::open(&repo)?;

    meta.commit(&repo, "init")
}
