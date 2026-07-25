use crate::meta::Metadata;
use crate::repo::RepositoryUtils;
use anyhow::Result;
use git2::Repository;
use immargs::args;
use std::path::Path;

args! {
    InitArgs,
    -h --help   "print help message",
}

pub fn main(path: &Path, _args: InitArgs) -> Result<()> {
    let repo = Repository::initialize(path)?;
    let meta = Metadata::open(&repo)?;

    meta.commit(&repo, "init")
}
