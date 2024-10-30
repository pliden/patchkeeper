use anyhow::anyhow;
use anyhow::bail;
use anyhow::Result;
use chrono::DateTime;
use chrono::Local;
use git2::Branch;
use git2::Commit;
use git2::Config;
use git2::Index;
use git2::Oid;
use git2::Repository;
use git2::RepositoryInitOptions;
use git2::ResetType;
use git2::Time;
use std::path::Path;

pub const HEAD: &str = "HEAD";

// Override git2's default branch name 'master' with 'main'
const CONFIG_DEFAULT_BRANCH: &str = "init.defaultBranch";
const DEFAULT_BRANCH: &str = "main";

pub trait RepositoryUtils {
    fn initialize(path: &Path) -> Result<Repository>;
    fn ensure_no_unrefreshed(&self) -> Result<()>;
    fn ensure_no_unresolved(&self) -> Result<()>;
    fn head_name(&self) -> Result<String>;
    fn find_commits(&self, oids: &[Oid]) -> Result<Vec<Commit>>;
    fn find_commit_by_revspec(&self, revspec: &str) -> Result<Commit>;
    fn find_commits_by_revspecs(&self, revspecs: &[String]) -> Result<Vec<Commit>>;
    fn reset_hard(&self, head: &Commit) -> Result<()>;
    fn amend_head(&self, index: &mut Index) -> Result<Oid>;
}

impl RepositoryUtils for Repository {
    fn initialize(path: &Path) -> Result<Repository> {
        let default_branch_name = Config::open_default()?
            .get_string(CONFIG_DEFAULT_BRANCH)
            .unwrap_or(DEFAULT_BRANCH.to_string());

        Ok(Repository::init_opts(
            path,
            RepositoryInitOptions::new()
                .no_reinit(true)
                .mkdir(true)
                .mkpath(true)
                .initial_head(&default_branch_name),
        )?)
    }

    fn ensure_no_unrefreshed(&self) -> Result<()> {
        let tree = self.head()?.peel_to_commit()?.tree()?;
        let diff = self.diff_tree_to_workdir_with_index(Some(&tree), None)?;
        if diff.stats()?.files_changed() > 0 {
            bail!("unrefreshed changes found");
        }

        Ok(())
    }

    fn ensure_no_unresolved(&self) -> Result<()> {
        let index = self.index()?;
        if index.has_conflicts() {
            bail!("unresolved merge conflicts found");
        }

        Ok(())
    }

    fn head_name(&self) -> Result<String> {
        let reference = self.head()?;
        if !reference.is_branch() {
            bail!("'HEAD' is not a branch");
        }

        Branch::wrap(reference).short_name()
    }

    fn find_commits(&self, oids: &[Oid]) -> Result<Vec<Commit>> {
        let mut commits = vec![];
        for oid in oids.iter().copied() {
            commits.push(self.find_commit(oid)?);
        }
        Ok(commits)
    }

    fn find_commit_by_revspec(&self, revspec: &str) -> Result<Commit> {
        Ok(self.revparse_single(revspec)?.peel_to_commit()?)
    }

    fn find_commits_by_revspecs(&self, revspecs: &[String]) -> Result<Vec<Commit>> {
        let mut commits = vec![];
        for revspec in revspecs {
            commits.push(self.find_commit_by_revspec(revspec)?);
        }
        Ok(commits)
    }

    fn reset_hard(&self, head: &Commit) -> Result<()> {
        Ok(self.reset(head.as_object(), ResetType::Hard, None)?)
    }

    fn amend_head(&self, index: &mut Index) -> Result<Oid> {
        let head = self.head()?.peel_to_commit()?;
        let update_ref = Some(HEAD);
        let author = None;
        let committer = None;
        let encoding = None;
        let message = None;
        let tree_id = index.write_tree()?;
        let tree = self.find_tree(tree_id)?;
        let tree = Some(&tree);
        Ok(head.amend(update_ref, author, committer, encoding, message, tree)?)
    }
}

pub trait IndexUtils {
    fn unresolved_conflicts(&self) -> Result<Vec<String>>;
}

impl IndexUtils for Index {
    fn unresolved_conflicts(&self) -> Result<Vec<String>> {
        let mut paths = vec![];
        if self.has_conflicts() {
            for conflict in self.conflicts()? {
                let index_conflict = conflict?;
                let index_entry = index_conflict
                    .our
                    .ok_or(anyhow!("cannot access merge conflict in index"))?;
                let path = String::from_utf8(index_entry.path)?;
                paths.push(path);
            }
        }

        Ok(paths)
    }
}

pub trait BranchUtils {
    fn short_name(&self) -> Result<String>;
    fn full_name(&self) -> Result<String>;
}

impl BranchUtils for Branch<'_> {
    fn short_name(&self) -> Result<String> {
        match self.name()? {
            Some(name) => Ok(name.to_string()),
            _ => bail!("branch name is not a valid UTF-8 string"),
        }
    }

    fn full_name(&self) -> Result<String> {
        match self.get().name() {
            Some(name) => Ok(name.to_string()),
            _ => bail!("branch name is not a valid UTF-8 string"),
        }
    }
}

pub trait CommitUtils {
    fn short_id(&self) -> Result<String>;
    fn time_local(&self) -> Result<String>;
}

fn utc_to_local(time: Time) -> Result<DateTime<Local>> {
    Ok(DateTime::from_timestamp(time.seconds(), 0)
        .ok_or(anyhow!("failed to resolve timestemp"))?
        .with_timezone(&Local))
}

impl CommitUtils for Commit<'_> {
    fn short_id(&self) -> Result<String> {
        match self.as_object().short_id()?.as_str() {
            Some(short_id) => Ok(short_id.to_string()),
            _ => bail!("short id is not a valid UTF-8 string"),
        }
    }

    fn time_local(&self) -> Result<String> {
        Ok(utc_to_local(self.time())?.to_string())
    }

    // pub fn author_time_local(commit: &Commit) -> Result<String> {
    //     Ok(utc_to_local(commit.author().when())?.to_string())
    // }
}
