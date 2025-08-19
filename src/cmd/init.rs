use crate::meta::Metadata;
use crate::repo::RepositoryUtils;
use anyhow::Result;
use git2::Repository;
use immargs::Args;
use immargs::immargs;
use std::path::Path;

pub fn main(path: &Path, args: Args) -> Result<()> {
    let _ = immargs!({ args });
    let repo = Repository::initialize(path)?;
    let meta = Metadata::open(&repo)?;

    meta.commit(&repo, "init")
}
