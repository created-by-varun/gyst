use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "gyst")]
#[command(author = "Varun V")]
#[command(version = "0.1.0")]
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
}