use anyhow::{Result, Context};
use git2::{Repository, Branch, BranchType, Time};
use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::Local;

#[derive(Debug, Serialize)]
pub struct TimeAgo {
    pub days: u32,
    pub hours: u32,
    pub minutes: u32,
}

impl TimeAgo {
    pub fn to_string(&self) -> String {
        if self.days > 0 {
            format!("{} days", self.days)
        } else if self.hours > 0 {
            format!("{} hours", self.hours)
        } else {
            format!("{} minutes", self.minutes)
        }
    }
}

#[derive(Debug, Serialize)]
pub struct BranchHealth {
    pub name: String,
    pub status: BranchStatus,
    #[serde(skip)]
    pub last_activity: TimeAgo,
    #[serde(rename = "last_activity")]
    pub last_activity_display: String,
    #[serde(rename = "age")]
    pub age_display: String,
    pub author: String,
    pub commit_count: u32,
    pub ahead_count: u32,
    pub behind_count: u32,
}

#[derive(Debug, Serialize)]
pub enum BranchStatus {
    Healthy,
    NeedsAttention,
    Stale,
}

pub struct BranchAnalyzer {
    repo: Repository,
    stale_days: u32,
    inactive_days: u32,
}

impl BranchAnalyzer {
    pub fn new(repo_path: &str) -> Result<Self> {
        let repo = Repository::discover(repo_path)
            .context("Failed to find git repository")?;
        
        Ok(Self {
            repo,
            stale_days: 30,
            inactive_days: 7,
        })
    }

    fn calculate_time_ago(&self, git_time: Time) -> Result<TimeAgo> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .context("Failed to get current time")?
            .as_secs() as i64;
        
        let diff_secs = now - git_time.seconds();
        let days = diff_secs / (24 * 60 * 60);
        let remaining_secs = diff_secs % (24 * 60 * 60);
        let hours = remaining_secs / (60 * 60);
        let minutes = (remaining_secs % (60 * 60)) / 60;
        
        Ok(TimeAgo {
            days: days as u32,
            hours: hours as u32,
            minutes: minutes as u32,
        })
    }

    pub fn analyze_branch(&self, branch: &Branch) -> Result<BranchHealth> {
        let branch_ref = branch.get();
        let branch_name = match branch.name()? {
            Some(name) => name.to_string(),
            None => "unknown".to_string(),
        };
        
        let commit = branch_ref.peel_to_commit()
            .context("Failed to get branch commit")?;
        
        let last_activity = self.calculate_time_ago(commit.time())?;
        
        let mut revwalk = self.repo.revwalk()?;
        revwalk.push(commit.id())?;
        let commit_count = revwalk.count() as u32;
        
        let (ahead, behind) = self.get_distance_from_main(&branch)?;

        let main_branch = self.repo.find_branch("master", BranchType::Local)
            .or_else(|_| self.repo.find_branch("main", BranchType::Local))
            .context("Failed to find main branch")?;
        let main_commit = main_branch.get().peel_to_commit()?;
        let merge_base = self.repo.merge_base(commit.id(), main_commit.id())?;

        let mut revwalk = self.repo.revwalk()?;
        revwalk.push(commit.id())?;
        revwalk.hide(merge_base)?;
        let age_time = if let Some(Ok(commit_id)) = revwalk.next() {
            self.repo.find_commit(commit_id)?.time()
        } else {
            commit.time()
        };

        let age = self.calculate_time_ago(age_time)?;

        let status = if last_activity.days >= self.stale_days {
            BranchStatus::Stale
        } else if last_activity.days >= self.inactive_days {
            BranchStatus::NeedsAttention
        } else {
            BranchStatus::Healthy
        };

        Ok(BranchHealth {
            name: branch_name,
            status,
            age_display: age.to_string(),
            last_activity_display: last_activity.to_string(),
            last_activity,
            author: commit.author().name().unwrap_or("unknown").to_string(),
            commit_count,
            ahead_count: ahead as u32,
            behind_count: behind as u32,
        })
    }

    fn get_distance_from_main(&self, branch: &Branch) -> Result<(usize, usize)> {
        let main_branch = self.repo.find_branch("main", BranchType::Local)
            .or_else(|_| self.repo.find_branch("master", BranchType::Local))
            .context("Failed to find main or master branch")?;
        
        let branch_commit = branch.get().peel_to_commit()
            .context("Failed to get branch commit")?;
        let main_commit = main_branch.get().peel_to_commit()
            .context("Failed to get main branch commit")?;
        
        self.repo.graph_ahead_behind(branch_commit.id(), main_commit.id())
            .context("Failed to calculate ahead/behind counts")
    }

    pub fn analyze_branches(&self, filter: BranchFilter, days: Option<u32>, author: Option<String>) -> Result<Vec<BranchHealth>> {
        let mut results = Vec::new();
        
        let branch_types = match filter {
            BranchFilter::All => vec![BranchType::Local, BranchType::Remote],
            BranchFilter::Local => vec![BranchType::Local],
            BranchFilter::Remote => vec![BranchType::Remote],
        };
        
        for branch_type in branch_types {
            let branches = self.repo.branches(Some(branch_type))
                .context("Failed to get repository branches")?;
            
            for branch_result in branches {
                let (branch, _) = branch_result?;
                if let Ok(health) = self.analyze_branch(&branch) {
                    if let Some(max_days) = days {
                        if health.last_activity.days > max_days {
                            continue;
                        }
                    }
                    
                    if let Some(ref target_author) = author {
                        if health.author != *target_author {
                            continue;
                        }
                    }
                    
                    results.push(health);
                }
            }
        }
        
        Ok(results)
    }
}

#[derive(Debug)]
pub enum BranchFilter {
    All,
    Local,
    Remote,
}

pub enum OutputFormat {
    Text,
    Json,
    Markdown,
}

impl From<&str> for OutputFormat {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "json" => OutputFormat::Json,
            "markdown" => OutputFormat::Markdown,
            _ => OutputFormat::Text,
        }
    }
}

pub fn format_output(results: &[BranchHealth], format: OutputFormat) -> Result<String> {
    match format {
        OutputFormat::Json => Ok(serde_json::to_string_pretty(results)?),
        OutputFormat::Markdown => format_markdown(results),
        OutputFormat::Text => format_text(results),
    }
}

fn format_text(results: &[BranchHealth]) -> Result<String> {
    let mut output = String::from("Branch Health Report\n");
    output.push_str(&format!("Last updated: {}\n\n", Local::now().format("%Y-%m-%d %H:%M:%S")));

    for health in results {
        let status_emoji = match health.status {
            BranchStatus::Healthy => "🟢",
            BranchStatus::NeedsAttention => "🟡",
            BranchStatus::Stale => "🔴",
        };

        output.push_str(&format!("{}\n", health.name));
        output.push_str(&format!("├── Status: {} {}\n", status_emoji, format!("{:?}", health.status)));
        output.push_str(&format!("├── Age: {}\n", health.age_display));
        output.push_str(&format!("├── Last Activity: {}\n", health.last_activity_display));
        output.push_str(&format!("├── Author: {}\n", health.author));
        output.push_str(&format!("├── Commits: {}\n", health.commit_count));
        output.push_str(&format!("└── Main Distance: {} ahead, {} behind\n\n", health.ahead_count, health.behind_count));
    }

    Ok(output)
}

fn format_markdown(results: &[BranchHealth]) -> Result<String> {
    let mut output = String::from("# Branch Health Report\n\n");
    output.push_str(&format!("*Last updated: {}*\n\n", Local::now().format("%Y-%m-%d %H:%M:%S")));

    for health in results {
        let status_emoji = match health.status {
            BranchStatus::Healthy => "🟢",
            BranchStatus::NeedsAttention => "🟡",
            BranchStatus::Stale => "🔴",
        };

        output.push_str(&format!("## {}\n\n", health.name));
        output.push_str("| Metric | Value |\n");
        output.push_str("|--------|-------|\n");
        output.push_str(&format!("| Status | {} {} |\n", status_emoji, format!("{:?}", health.status)));
        output.push_str(&format!("| Age | {} |\n", health.age_display));
        output.push_str(&format!("| Last Activity | {} |\n", health.last_activity_display));
        output.push_str(&format!("| Author | {} |\n", health.author));
        output.push_str(&format!("| Commits | {} |\n", health.commit_count));
        output.push_str(&format!("| Main Distance | {} ahead, {} behind |\n\n", health.ahead_count, health.behind_count));
    }

    Ok(output)
}
