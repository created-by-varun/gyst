use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "gyst")]
#[command(author = "Varun V")]
#[command(version = "0.1.1")]
#[command(about = "AI-powered Git commit assistant", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Generate and create a commit with an AI-generated message
    Commit {
        /// Skip confirmation and use the generated message directly
        #[arg(short, long)]
        quick: bool,
    },
    
    /// Get multiple commit message suggestions
    Suggest,
    
    /// Get suggestions for Git commands based on what you want to do
    Explain {
        /// Description of what you want to do
        #[arg(value_name = "DESCRIPTION")]
        description: String,
    },
    
    /// Configure gyst settings
    Config {
        /// Set the OpenAI API key
        #[arg(long)]
        api_key: Option<String>,
        
        /// Show current configuration
        #[arg(short, long)]
        show: bool,
    },
    
    /// Show staged changes with detailed diff
    Diff,
    
    /// Analyze and manage git branches
    Branch {
        #[command(subcommand)]
        command: BranchCommands,
    },
}

#[derive(Subcommand)]
pub enum BranchCommands {
    /// Analyze and report branch health status
    Health {
        /// Include all branches (local and remote)
        #[arg(long)]
        all: bool,

        /// Only remote branches
        #[arg(long)]
        remote: bool,

        /// Only local branches
        #[arg(long)]
        local: bool,

        /// Consider activity within last N days
        #[arg(long)]
        days: Option<u32>,

        /// Filter by author
        #[arg(long)]
        author: Option<String>,

        /// Output format (text, json, markdown)
        #[arg(long, default_value = "text")]
        format: String,
    },
}