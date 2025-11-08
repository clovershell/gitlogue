use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use git2::{Commit as Git2Commit, Oid, Repository};
use rand::seq::SliceRandom;
use std::path::Path;

pub struct GitRepository {
    repo: Repository,
}

#[derive(Debug, Clone)]
pub struct CommitMetadata {
    pub hash: String,
    pub author: String,
    pub date: DateTime<Utc>,
    pub message: String,
}

impl GitRepository {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let repo = Repository::open(path)
            .context("Failed to open Git repository")?;
        Ok(Self { repo })
    }

    pub fn get_commit(&self, hash: &str) -> Result<CommitMetadata> {
        let obj = self.repo
            .revparse_single(hash)
            .context("Invalid commit hash or commit not found")?;

        let commit = obj
            .peel_to_commit()
            .context("Object is not a commit")?;

        Ok(Self::extract_metadata(&commit))
    }

    pub fn random_commit(&self) -> Result<CommitMetadata> {
        let mut revwalk = self.repo.revwalk()?;
        revwalk.push_head()?;

        let non_merge_commits: Vec<Oid> = revwalk
            .filter_map(|oid| oid.ok())
            .filter(|oid| {
                self.repo
                    .find_commit(*oid)
                    .map(|c| c.parent_count() <= 1)
                    .unwrap_or(false)
            })
            .collect();

        if non_merge_commits.is_empty() {
            anyhow::bail!("No non-merge commits found in repository");
        }

        let oid = non_merge_commits
            .choose(&mut rand::thread_rng())
            .context("Failed to select random commit")?;

        let commit = self.repo.find_commit(*oid)?;
        Ok(Self::extract_metadata(&commit))
    }

    fn extract_metadata(commit: &Git2Commit) -> CommitMetadata {
        let hash = commit.id().to_string();
        let author = commit.author();
        let author_name = author.name().unwrap_or("Unknown").to_string();
        let timestamp = author.when().seconds();
        let date = DateTime::from_timestamp(timestamp, 0)
            .unwrap_or_else(|| Utc::now());
        let message = commit.message().unwrap_or("").trim().to_string();

        CommitMetadata {
            hash,
            author: author_name,
            date,
            message,
        }
    }
}
