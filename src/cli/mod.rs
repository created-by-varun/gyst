use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "gyst")]
#[command(author = "Varun V")]
#[command(version = "0.1.1")]
#[command(about = "AI-powered Git assistant for commits, branch management, and more")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Generate and create a commit with an AI-generated message
    ///
    /// Analyzes your staged changes and generates a meaningful commit message.
    /// In interactive mode (default), you can review, edit, or reject the message.
    /// Use --quick to skip confirmation and use the message directly.
    Commit {
        /// Skip confirmation and use the generated message directly
        #[arg(short, long)]
        quick: bool,
    },

    /// Get multiple commit message suggestions
    ///
    /// Generates three different commit message options for you to choose from.
    /// If no changes are staged, offers to stage all changes first.
    Suggest,

    /// Get AI-powered suggestions for Git commands
    ///
    /// Provides step-by-step instructions and explanations for Git operations
    /// based on your natural language description of what you want to do.
    Explain {
        /// Description of what you want to do (e.g., "undo last commit")
        #[arg(value_name = "DESCRIPTION")]
        description: String,
    },

    /// Configure gyst settings
    ///
    /// Manage configuration settings including API keys and preferences.
    /// Use --show to view current settings, --api-key to set API key.
    Config {
        /// Set the OpenAI API key
        #[arg(long)]
        api_key: Option<String>,

        /// Show current configuration
        #[arg(short, long)]
        show: bool,

        /// Enable or disable using the server
        #[arg(long)]
        use_server: Option<bool>,
    },

    /// Show detailed analysis of staged changes
    ///
    /// Displays a comprehensive diff view including:
    /// - Summary of changes (files, insertions, deletions)
    /// - List of added, modified, deleted, and renamed files
    /// - Detailed changes with syntax highlighting
    Diff,

    /// Analyze and manage git branches
    ///
    /// Tools for branch maintenance and health monitoring.
    /// Use 'gyst branch health' to analyze branch status.
    Branch {
        #[command(subcommand)]
        command: BranchCommands,
    },
}

#[derive(Subcommand)]
pub enum BranchCommands {
    /// Analyze and report branch health status
    ///
    /// Monitors branch health metrics including:
    /// - Age and last activity time
    /// - Commit frequency and contributors
    /// - Distance from main branch
    /// - Overall health status (healthy, needs attention, stale)
    Health {
        /// Include all branches (local and remote)
        #[arg(long)]
        all: bool,

        /// Only remote branches
        #[arg(long)]
        remote: bool,

        /// Only local branches (default)
        #[arg(long)]
        local: bool,

        /// Consider activity within last N days
        #[arg(long)]
        days: Option<u32>,

        /// Filter by author name
        #[arg(long)]
        author: Option<String>,

        /// Output format: text (default), json, or markdown
        #[arg(long, default_value = "text")]
        format: String,
    },
}
