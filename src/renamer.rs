use std::process::Command;

use anyhow::{Result, anyhow, bail};

use crate::{config::CONFIG, repository::Repository};

pub struct Renamer {
    repo: Repository,
    current_branch: String,
    default_branch: String,
}

impl Renamer {
    /// Create a new renamer instance from a path. Bails if the current branch is protected, or if
    /// the current branch is the same as the default branch.
    pub fn try_new(path: &str) -> Result<Self> {
        let repo = Repository::discover(path)?;
        let current_branch = repo.get_current_branch()?;
        if matches!(current_branch.to_lowercase().as_str(), "main" | "master" | "develop") {
            bail!("Cannot rename protected branch '{current_branch}'");
        }

        let default_branch = repo.get_default_branch()?;
        if current_branch == default_branch {
            bail!("Current branch '{current_branch}' is the same as default branch");
        }

        Ok(Self { repo, current_branch, default_branch })
    }

    /// Rename the current branch to an AI-generated name
    pub fn name_auto(&self) -> Result<(String, String)> {
        let diff = &self
            .repo
            .get_branch_diff(&self.current_branch, &self.default_branch)?;
        let suggested_name = Self::generate(diff)?;
        if suggested_name == self.current_branch {
            bail!("Generated name '{suggested_name}' is the same as current branch name");
        }

        self.repo.rename_branch(&self.current_branch, &suggested_name)?;

        Ok((self.current_branch.clone(), suggested_name))
    }

    // Generate a branch name using Claude CLI
    fn generate(diff: &str) -> Result<String> {
        Command::new(&CONFIG.generator.command)
            .args(&CONFIG.generator.args)
            .arg(CONFIG.prompt.format_with_diff(diff))
            .output()
            .ok()
            .filter(|output| output.status.success())
            .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
            .filter(|result| !result.is_empty())
            .ok_or_else(|| anyhow!("Failed to generate branch name using Claude Code"))
    }
}
