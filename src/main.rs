mod cli;
mod git;
mod ai;
mod config;
mod utils;

use clap::Parser;
use cli::{Cli, Commands};
use colored::*;

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
            println!("{}", "Analyzing diff...".bold());
            let repo = git::GitRepo::open(".")?;
            
            if !repo.has_staged_changes()? {
                println!("{}", "\nNo staged changes found. Stage some changes first with 'git add'".yellow());
                return Ok(());
            }

            let changes = repo.get_staged_changes()?;
            
            // Print summary statistics
            println!("\n{}", "Summary".bold().underline());
            println!("{} {}, {} {}, {} {}",
                changes.stats.files_changed.to_string().bold(),
                if changes.stats.files_changed == 1 { "file" } else { "files" },
                changes.stats.insertions.to_string().green().bold(),
                if changes.stats.insertions == 1 { "insertion(+)" } else { "insertions(+)" },
                changes.stats.deletions.to_string().red().bold(),
                if changes.stats.deletions == 1 { "deletion(-)" } else { "deletions(-)" }
            );
            
            // Print file changes summary
            if !changes.added.is_empty() {
                println!("\n{}", "Added files:".green().bold());
                for file in changes.added {
                    println!("  {} {}", "+".green().bold(), file.green());
                }
            }
            
            if !changes.modified.is_empty() {
                println!("\n{}", "Modified files:".yellow().bold());
                for file in changes.modified {
                    println!("  {} {}", "*".yellow().bold(), file.yellow());
                }
            }
            
            if !changes.deleted.is_empty() {
                println!("\n{}", "Deleted files:".red().bold());
                for file in changes.deleted {
                    println!("  {} {}", "-".red().bold(), file.red());
                }
            }
            
            if !changes.renamed.is_empty() {
                println!("\n{}", "Renamed files:".blue().bold());
                for (old, new) in changes.renamed {
                    println!("  {} {} {} {}", 
                        "→".blue().bold(),
                        old.strikethrough(),
                        "→".blue().bold(),
                        new.blue()
                    );
                }
            }

            // Print detailed diff
            println!("\n{}", "Detailed changes:".bold().underline());
            let hunks = repo.get_structured_diff()?;
            for hunk in hunks {
                println!("\n{}", hunk.header.cyan());
                for line in hunk.lines {
                    match line.origin {
                        '+' => print!("{}", line.content.green()),
                        '-' => print!("{}", line.content.red()),
                        _ => print!("{}", line.content.dimmed()),
                    }
                }
            }
        }
    }

    Ok(())
}