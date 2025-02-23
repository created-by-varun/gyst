mod cli;
mod git;
mod ai;
mod config;
mod utils;
mod command_suggest;

use clap::Parser;
use cli::{Cli, Commands};
use colored::*;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Commit { quick } => {
            let repo = git::GitRepo::open(".")?;

            // Check if there are any staged changes
            if !repo.has_staged_changes()? {
                println!("\n{}", "No staged changes found.".yellow());
                print!("Would you like to stage all changes? [y/N] ");
                io::stdout().flush()?;

                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                
                if input.trim().to_lowercase() == "y" {
                    repo.stage_all()?;
                    println!("All changes have been staged.");
                } else {
                    println!("No changes to commit. Stage your changes using 'git add' first.");
                    return Ok(());
                }
            }

            let changes = repo.get_staged_changes()?;
            let hunks = repo.get_structured_diff()?;
            
            // Convert hunks to a single diff string
            let mut diff = String::new();
            for hunk in &hunks {
                diff.push_str(&hunk.header);
                for line in &hunk.lines {
                    diff.push_str(&line.content);
                }
            }

            // Load config and create AI client
            let config = config::Config::load()?;
            let generator = ai::CommitMessageGenerator::new(config);

            println!("{}", "Generating commit message...".bold());
            let message = generator.generate_message(&changes, &diff).await?;

            if quick {
                // Use the message directly in quick mode
                repo.create_commit(&message)?;
                println!("\n{}", "Commit created successfully!".green().bold());
                println!("Message: {}", message);
            } else {
                // Show the message and ask for confirmation
                println!("\nProposed commit message:");
                println!("{}", message.green());
                print!("\nUse this message? [Y/n/e(edit)] ");
                io::stdout().flush()?;

                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                
                let message = match input.trim().to_lowercase().as_str() {
                    "n" | "no" => {
                        println!("Commit aborted");
                        return Ok(());
                    }
                    "e" | "edit" => {
                        // Create a temporary file with the message
                        let mut temp = tempfile::NamedTempFile::new()?;
                        writeln!(temp, "{}", message)?;
                        
                        // Get the path before the file is closed
                        let temp_path = temp.path().to_path_buf();
                        
                        // Open in the default editor
                        let status = std::process::Command::new(std::env::var("EDITOR").unwrap_or_else(|_| "vim".to_string()))
                            .arg(&temp_path)
                            .status()?;
                        
                        if !status.success() {
                            println!("Editor returned with error");
                            return Ok(());
                        }
                        
                        // Read back the edited message
                        let edited = std::fs::read_to_string(&temp_path)?;
                        edited.trim().to_string()
                    }
                    _ => message,
                };

                repo.create_commit(&message)?;
                println!("\n{}", "Commit created successfully!".green().bold());
            }
        }
        Commands::Suggest => {
            let repo = git::GitRepo::open(".")?;
            
            // Check if there are any staged changes
            if !repo.has_staged_changes()? {
                println!("\n{}", "No staged changes found.".yellow());
                print!("Would you like to stage all changes? [y/N] ");
                io::stdout().flush()?;

                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                
                if input.trim().to_lowercase() == "y" {
                    repo.stage_all()?;
                    println!("All changes have been staged.");
                } else {
                    println!("No changes to commit. Stage your changes using 'git add' first.");
                    return Ok(());
                }
            }

            let changes = repo.get_staged_changes()?;
            let hunks = repo.get_structured_diff()?;
            
            // Convert hunks to a single diff string
            let mut diff = String::new();
            for hunk in &hunks {
                diff.push_str(&hunk.header);
                for line in &hunk.lines {
                    diff.push_str(&line.content);
                }
            }

            // Load config and create AI client
            let config = config::Config::load()?;
            let generator = ai::CommitMessageGenerator::new(config);

            println!("Generating commit message suggestions...");
            let suggestions = generator.generate_suggestions(&changes, &diff, 3).await?;

            println!("\n{}", "Suggested commit messages:".bold());
            for (i, message) in suggestions.iter().enumerate() {
                println!("\n{}. {}", (i + 1).to_string().bold(), message.green());
            }

            print!("\nSelect a message to use (1-3) or press Enter to skip: ");
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            
            if let Ok(choice) = input.trim().parse::<usize>() {
                if choice > 0 && choice <= suggestions.len() {
                    let message = &suggestions[choice - 1];
                    repo.create_commit(message)?;
                    println!("\n{}", "Commit created successfully!".green().bold());
                }
            } else {
                println!("No message selected. You can still create a commit manually.");
            }
        }
        Commands::Config { api_key, show } => {
            let mut config = config::Config::load()?;
            
            if let Some(ref key) = api_key {
                println!("Setting API key...");
                config.set_api_key(key.clone())?;
                println!("{}", "API key saved successfully!".green());
            }

            if show || api_key.is_none() {
                println!("{}", config.display());
            }
        }
        Commands::Explain { description } => {
            println!("{}", "Analyzing your request...".bold());
            
            let config = config::Config::load()?;
            let suggester = command_suggest::CommandSuggester::new(config);
            
            match suggester.suggest(&description).await {
                Ok(suggestion) => {
                    println!("\n{}", "Here's what you can do:".bold());
                    println!("{}", suggestion);
                }
                Err(e) => {
                    println!("{}", format!("Error getting suggestion: {}", e).red());
                }
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