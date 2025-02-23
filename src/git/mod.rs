use anyhow::{Context, Result};
use git2::{Repository, StatusOptions, Delta};
use std::path::Path;

#[derive(Debug)]
pub struct StagedChanges {
    pub added: Vec<String>,
    pub modified: Vec<String>,
    pub deleted: Vec<String>,
    pub renamed: Vec<(String, String)>, // (old_path, new_path)
    pub stats: DiffStats,
}

#[derive(Debug, Default)]
pub struct DiffStats {
    pub files_changed: usize,
    pub insertions: usize,
    pub deletions: usize,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct DiffHunk {
    pub old_start: u32,
    pub old_lines: u32,
    pub new_start: u32,
    pub new_lines: u32,
    pub header: String,
    pub lines: Vec<DiffLine>,
}

#[derive(Debug)]
pub struct DiffLine {
    pub origin: char,
    pub content: String,
}

pub struct GitRepo {
    repo: Repository,
}

impl GitRepo {
    /// Open a git repository at the given path or search parent directories
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let repo = Repository::discover(path)
            .context("Failed to find git repository")?;
        Ok(Self { repo })
    }

    /// Stage all changes in the repository
    pub fn stage_all(&self) -> Result<()> {
        let mut index = self.repo.index()?;
        
        // Add all changes to the index
        index.add_all(
            ["*"].iter(),
            git2::IndexAddOption::DEFAULT,
            None,
        )?;
        
        // Write the index to disk
        index.write()?;
        
        Ok(())
    }

    /// Check if there are any staged changes in the repository
    pub fn has_staged_changes(&self) -> Result<bool> {
        let mut opts = StatusOptions::new();
        opts.include_untracked(false)
            .include_ignored(false)
            .include_unmodified(false)
            .exclude_submodules(true);

        let statuses = self.repo
            .statuses(Some(&mut opts))
            .context("Failed to get repository status")?;

        Ok(statuses.iter().any(|entry| {
            let status = entry.status();
            status.is_index_new() 
                || status.is_index_modified() 
                || status.is_index_deleted() 
                || status.is_index_renamed() 
                || status.is_index_typechange()
        }))
    }

    /// Check if there are any changes (staged or unstaged) in the repository
    pub fn has_any_changes(&self) -> Result<bool> {
        let mut opts = StatusOptions::new();
        opts.include_untracked(true)
            .include_ignored(false)
            .include_unmodified(false)
            .exclude_submodules(true);

        let statuses = self.repo
            .statuses(Some(&mut opts))
            .context("Failed to get repository status")?;

        Ok(!statuses.is_empty())
    }

    /// Get a summary of staged changes
    pub fn get_staged_changes(&self) -> Result<StagedChanges> {
        let mut changes = StagedChanges {
            added: Vec::new(),
            modified: Vec::new(),
            deleted: Vec::new(),
            renamed: Vec::new(),
            stats: DiffStats::default(),
        };

        let mut opts = StatusOptions::new();
        opts.include_untracked(false)
            .include_ignored(false)
            .include_unmodified(false)
            .exclude_submodules(true);

        let statuses = self.repo
            .statuses(Some(&mut opts))
            .context("Failed to get repository status")?;

        for entry in statuses.iter() {
            let status = entry.status();
            let path = entry.path().unwrap_or("").to_string();

            if status.is_index_new() {
                changes.added.push(path);
                changes.stats.files_changed += 1;
            } else if status.is_index_modified() {
                changes.modified.push(path);
                changes.stats.files_changed += 1;
            } else if status.is_index_deleted() {
                changes.deleted.push(path);
                changes.stats.files_changed += 1;
            } else if status.is_index_renamed() {
                if let Some(head_to_index) = entry.head_to_index() {
                    let old_path = head_to_index.old_file().path()
                        .map(|p| p.to_string_lossy().to_string())
                        .unwrap_or_else(|| "unknown".to_string());
                    let new_path = head_to_index.new_file().path()
                        .map(|p| p.to_string_lossy().to_string())
                        .unwrap_or_else(|| "unknown".to_string());
                    changes.renamed.push((old_path, new_path));
                    changes.stats.files_changed += 1;
                }
            }
        }

        // Get detailed stats from the diff
        if let Ok(diff) = self.get_diff() {
            let stats = diff.stats()?;
            changes.stats.insertions = stats.insertions();
            changes.stats.deletions = stats.deletions();
        }

        Ok(changes)
    }

    /// Get the raw diff object for staged changes
    fn get_diff(&self) -> Result<git2::Diff> {
        let mut diff_opts = git2::DiffOptions::new();
        
        // Get the current index (staged changes)
        let index = self.repo.index()?;
        
        // Get the diff between HEAD and index (staged changes)
        if let Ok(head) = self.repo.head() {
            let tree = head.peel_to_tree()?;
            self.repo.diff_tree_to_index(Some(&tree), Some(&index), Some(&mut diff_opts))
        } else {
            // If there's no HEAD (initial commit), diff against an empty tree
            let empty_tree = self.repo.find_tree(git2::Oid::zero())?;
            self.repo.diff_tree_to_index(Some(&empty_tree), Some(&index), Some(&mut diff_opts))
        }.context("Failed to generate diff")
    }

    /// Get structured diff information
    pub fn get_structured_diff(&self) -> Result<Vec<DiffHunk>> {
        let diff = self.get_diff()?;
        let mut hunks = Vec::new();
        let mut current_hunk: Option<DiffHunk> = None;

        diff.print(git2::DiffFormat::Patch, |delta, hunk, line| {
            if let Some(hunk) = hunk {
                if current_hunk.is_none() {
                    current_hunk = Some(DiffHunk {
                        old_start: hunk.old_start(),
                        old_lines: hunk.old_lines(),
                        new_start: hunk.new_start(),
                        new_lines: hunk.new_lines(),
                        header: String::from_utf8_lossy(hunk.header()).to_string(),
                        lines: Vec::new(),
                    });
                }
                
                if let Some(hunk) = &mut current_hunk {
                    let origin = line.origin();
                    let content = String::from_utf8_lossy(line.content()).to_string();
                    hunk.lines.push(DiffLine { origin, content });
                }
            } else if delta.status() == Delta::Renamed {
                // Handle renamed files
                if let Some(hunk) = current_hunk.take() {
                    hunks.push(hunk);
                }
            }
            true
        })?;

        if let Some(hunk) = current_hunk.take() {
            hunks.push(hunk);
        }

        Ok(hunks)
    }

    /// Create a commit with the given message
    pub fn create_commit(&self, message: &str) -> Result<git2::Oid> {
        let signature = self.repo.signature()
            .context("Failed to get signature")?;
        
        let tree_id = self.repo.index()?
            .write_tree()
            .context("Failed to write tree")?;
        
        let tree = self.repo.find_tree(tree_id)
            .context("Failed to find tree")?;

        let parent = match self.repo.head() {
            Ok(head) => Some(head.peel_to_commit()?),
            Err(_) => None,
        };

        let parents: Vec<&git2::Commit> = match &parent {
            Some(commit) => vec![commit],
            None => vec![],
        };

        self.repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            message,
            &tree,
            &parents,
        ).context("Failed to create commit")
    }
}