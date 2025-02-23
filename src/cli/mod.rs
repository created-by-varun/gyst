use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "gyst")]
#[command(about = "AI-powered Git commit message generator", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Generate commit message for staged changes
    Commit {
        /// Use quick mode (skip confirmation)
        #[arg(short, long)]
        quick: bool,
    },
    /// Get multiple commit message suggestions
    Suggest {
        /// Number of suggestions to generate
        #[arg(short, long, default_value_t = 3)]
        count: u8,
    },
    /// Configure Gyst settings
    Config {
        /// Set API key
        #[arg(long)]
        api_key: Option<String>,
        /// Show current configuration
        #[arg(long)]
        show: bool,
    },
    /// Show analyzed diff before commit
    Diff,
}