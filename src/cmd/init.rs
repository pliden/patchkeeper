use crate::meta::Metadata;
use crate::repo::RepositoryUtils;
use anyhow::Result;
use cmdline::CmdLine;
use git2::Repository;
use std::path::Path;

#[derive(CmdLine)]
pub struct Args {}

pub fn main(path: &Path, _args: Args) -> Result<()> {
    let repo = Repository::initialize(path)?;
    let meta = Metadata::open(&repo)?;

    meta.commit(&repo, "init")
}
