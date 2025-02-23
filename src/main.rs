mod cli;
mod git;
mod ai;
mod config;
mod utils;

use clap::Parser;
use cli::{Cli, Commands};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Commit { quick } => {
            println!("Generating commit message... Quick mode: {}", quick);
            // TODO: Implement commit logic
        }
        Commands::Suggest { count } => {
            println!("Generating {} suggestions...", count);
            // TODO: Implement suggest logic
        }
        Commands::Config { api_key, show } => {
            if let Some(key) = api_key {
                println!("Setting API key: {}", key);
                // TODO: Implement config setting
            }
            if show {
                println!("Showing configuration");
                // TODO: Implement config display
            }
        }
        Commands::Diff => {
            println!("Analyzing diff...");
            // TODO: Implement diff analysis
        }
    }

    Ok(())
}