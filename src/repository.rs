use std::{ops::Deref, str::from_utf8};

use anyhow::{Result, bail};
use git2::{BranchType, DiffFormat};

pub struct Repository {
    inner: git2::Repository,
}

impl Repository {
    /// Create a new repository instance from a path
    pub fn discover(path: &str) -> Result<Self> {
        Ok(Self { inner: git2::Repository::discover(path)? })
    }

    /// Get the current branch name
    pub fn get_current_branch(&self) -> Result<String> {
        match self.head()?.shorthand() {
            Some(name) => Ok(name.to_string()),
            None => bail!("Not currently on a branch (detached HEAD)"),
        }
    }

    /// Get the default branch name (develop, main, or master)
    pub fn get_default_branch(&self) -> Result<String> {
        // Check common default branches
        for &branch_name in &["develop", "main", "master"] {
            if self.find_branch(branch_name, BranchType::Local).is_ok() {
                return Ok(branch_name.to_string());
            }
        }

        // Check remote HEAD reference
        let reference = self.find_reference("refs/remotes/origin/HEAD")?;
        if let Some(target) = reference.symbolic_target()
            && let Some(branch_name) = target.rsplit('/').next()
        {
            return Ok(branch_name.to_string());
        }

        bail!("Could not determine default branch")
    }

    /// Generate a diff between two branches (committed changes only)
    pub fn get_branch_diff(&self, source: &str, target: &str) -> Result<String> {
        let source_tree = self
            .find_branch(source, BranchType::Local)?
            .get()
            .peel_to_commit()?
            .tree()?;
        let target_tree = self
            .find_branch(target, BranchType::Local)?
            .get()
            .peel_to_commit()?
            .tree()?;

        let diff = self.diff_tree_to_tree(Some(&target_tree), Some(&source_tree), None)?;

        let mut diff_text = String::new();
        diff.print(DiffFormat::Patch, |_, _, line| {
            if let Ok(content) = from_utf8(line.content()) {
                diff_text.push_str(&format!("{}{content}", line.origin()));
            }
            true
        })?;

        let diff_text = diff_text.trim();
        if diff_text.is_empty() {
            bail!("No committed changes found between {source} and {target}");
        }

        Ok(if diff_text.len() > 5000 {
            format!("{}\n\n[... truncated ...]", &diff_text[..5000])
        } else {
            diff_text.to_string()
        })
    }

    /// Rename a branch
    pub fn rename_branch(&self, old: &str, new: &str) -> Result<()> {
        self.find_branch(old, BranchType::Local)?.rename(new, false)?;
        Ok(())
    }
}

impl Deref for Repository {
    type Target = git2::Repository;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
