mod cli;
mod git;
mod ai;
mod config;
mod command_suggest;

use clap::Parser;
use cli::{Cli, Commands};
use colored::*;
use std::io::{self, Write};
use spinners::{Spinner, Spinners};
use console::{style, Emoji};
use dialoguer::{theme::ColorfulTheme, Select};

static CHECKMARK: Emoji<'_, '_> = Emoji("✓", "√");
static CROSS: Emoji<'_, '_> = Emoji("✗", "x");
static SPARKLE: Emoji<'_, '_> = Emoji("✨", "*");
static PENCIL: Emoji<'_, '_> = Emoji("✏️ ", ">");

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Commit { quick } => {
            let repo = git::GitRepo::open(".")?;

            // Check if there are any staged changes
            if !repo.has_staged_changes()? {
                println!("\n{} {}", CROSS, style("No staged changes found.").yellow());
                print!("\n{} Would you like to stage all changes? [y/N] ", PENCIL);
                io::stdout().flush()?;

                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                
                if input.trim().to_lowercase() == "y" {
                    let mut sp = Spinner::new(Spinners::Dots9, "Staging all changes...".into());
                    repo.stage_all()?;
                    sp.stop_with_message(format!("{} {} {}\n", CHECKMARK, style("All changes have been staged").green(), SPARKLE));
                } else {
                    println!("\n{} {}", CROSS, style("No changes to commit. Stage your changes using 'git add' first.").yellow());
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

            let mut sp = Spinner::new(Spinners::Dots12, "Analyzing changes and generating commit message...".into());
            let message = generator.generate_message(&changes, &diff).await?;
            sp.stop_with_message(format!("{} {}\n", CHECKMARK, style("Commit message generated!").green()));

            if quick {
                // Use the message directly in quick mode
                let mut sp = Spinner::new(Spinners::Dots9, "Creating commit...".into());
                repo.create_commit(&message)?;
                sp.stop_with_message(format!("{} {} {}\n", CHECKMARK, style("Commit created successfully!").green().bold(), SPARKLE));
                println!("\n{} {}\n{}\n", PENCIL, style("Commit Message:").cyan().bold(), message);
            } else {
                // Show the message and ask for confirmation
                println!("\n{} {}", SPARKLE, style("Proposed commit message:").cyan().bold());
                println!("{}\n", style(message.as_str()).green());
                print!("\n{} Use this message? [Y/n/e(edit)] ", PENCIL);
                io::stdout().flush()?;

                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                
                let message = match input.trim().to_lowercase().as_str() {
                    "n" | "no" => {
                        println!("\n{} {}", CROSS, style("Commit aborted").yellow());
                        return Ok(());
                    }
                    "e" | "edit" => {
                        println!("\n{} {}", PENCIL, style("Opening in editor...").cyan());
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
                            println!("{} {}", CROSS, style("Editor returned with error").red());
                            return Ok(());
                        }
                        
                        // Read back the edited message
                        let edited = std::fs::read_to_string(&temp_path)?;
                        edited.trim().to_string()
                    }
                    _ => message,
                };

                // Create the commit
                let mut sp = Spinner::new(Spinners::Dots9, "Creating commit...".into());
                repo.create_commit(&message)?;
                sp.stop_with_message(format!("{} {} {}\n", CHECKMARK, style("Commit created successfully!").green().bold(), SPARKLE));
                println!("\n{} {}\n{}\n", PENCIL, style("Final Commit Message:").cyan().bold(), message);
            }
        }
        Commands::Suggest => {
            let repo = git::GitRepo::open(".")?;
            
            // Check if there are any staged changes
            if !repo.has_staged_changes()? {
                println!("\n{} {}", CROSS, style("No staged changes found.").yellow());
                print!("\n{} Would you like to stage all changes? [y/N] ", PENCIL);
                io::stdout().flush()?;

                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                
                if input.trim().to_lowercase() == "y" {
                    let mut sp = Spinner::new(Spinners::Dots9, "Staging all changes...".into());
                    repo.stage_all()?;
                    sp.stop_with_message(format!("{} {} {}\n", CHECKMARK, style("All changes have been staged").green(), SPARKLE));
                } else {
                    println!("\n{} {}", CROSS, style("No changes to commit. Stage your changes using 'git add' first.").yellow());
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

            let config = config::Config::load()?;
            let generator = ai::CommitMessageGenerator::new(config);

            let mut sp = Spinner::new(Spinners::Dots12, "Generating commit message suggestions...".into());
            let suggestions = generator.generate_suggestions(&changes, &diff, 3).await?;
            sp.stop_with_message(format!("{} {} {}\n", CHECKMARK, style("Suggestions generated!").green(), SPARKLE));

            // Create selection items with numbers
            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select a commit message")
                .default(0)
                .items(&suggestions)
                .interact_opt()?;

            match selection {
                Some(index) => {
                    let message = &suggestions[index];
                    let mut sp = Spinner::new(Spinners::Dots9, "Creating commit...".into());
                    repo.create_commit(message)?;
                    sp.stop_with_message(format!("{} {} {}\n", CHECKMARK, style("Commit created successfully!").green().bold(), SPARKLE));
                    println!("\n{} {}\n{}\n", PENCIL, style("Final Commit Message:").cyan().bold(), message);
                }
                None => {
                    println!("\n{} {}", CROSS, style("No message selected. You can still create a commit manually.").yellow());
                }
            }
        }
        Commands::Explain { description } => {
            let mut sp = Spinner::new(Spinners::Dots12, format!("{} {}", SPARKLE, style("Analyzing your request...").cyan().bold()).into());
            
            let config = config::Config::load()?;
            let suggester = command_suggest::CommandSuggester::new(config);
            
            match suggester.suggest(&description).await {
                Ok(suggestion) => {
                    sp.stop_with_message(format!("{} {}\n", CHECKMARK, style("Analysis complete!").green()));

                    // Parse the suggestion into sections
                    let sections: Vec<&str> = suggestion.split("\nCOMMAND:").collect();
                    
                    if sections.len() > 1 {
                        // First section is the introduction
                        if !sections[0].trim().is_empty() {
                            println!("\n{}", style(sections[0].trim()).white());
                        }

                        // Process each command section
                        for section in sections[1..].iter() {
                            let parts: Vec<&str> = section.split("\nEXPLANATION:").collect();
                            if parts.len() == 2 {
                                // Command with special formatting
                                println!("\n{} {}", PENCIL, style(parts[0].trim()).green().bold());

                                // Split explanation and note if present
                                let explanation_parts: Vec<&str> = parts[1].split("\nNOTE:").collect();
                                println!("   {}", style(explanation_parts[0].trim()).white());

                                // Print note if present, but only if it's important
                                if explanation_parts.len() > 1 {
                                    let note = explanation_parts[1].trim();
                                    if note.contains("CAREFUL") || note.contains("WARNING") || 
                                       note.contains("IMPORTANT") || note.contains("DO NOT") {
                                        println!("   {} {}", CROSS, style(note).yellow());
                                    }
                                }
                            }
                        }

                        // Print additional tip if present and important
                        if let Some(tip_start) = suggestion.find("\nADDITIONAL TIP:") {
                            let tip = suggestion[tip_start..].trim().replace("ADDITIONAL TIP:", "").trim().to_string();
                            if tip.contains("CAREFUL") || tip.contains("WARNING") || 
                               tip.contains("IMPORTANT") || tip.contains("caution") {
                                println!("\n{} {}", SPARKLE, style(tip).yellow().italic());
                            }
                        }
                    } else {
                        // Simple output for single-line suggestions
                        println!("\n{} {}", PENCIL, style(suggestion).green());
                    }
                }
                Err(e) => {
                    sp.stop_with_message(format!("{} {}\n", CROSS, style("Analysis failed").red()));
                    println!("{} {}", CROSS, style(format!("Error: {}", e)).red());
                }
            }
        }
        Commands::Config { api_key, show } => {
            let mut config = config::Config::load()?;
            
            if let Some(ref key) = api_key {
                println!("{} {}", PENCIL, style("Setting API key...").cyan());
                config.set_api_key(key.clone())?;
                println!("{} {}", CHECKMARK, style("API key saved successfully!").green());
            }

            if show || api_key.is_none() {
                println!("{}", config.display());
            }
        }
        Commands::Diff => {
            println!("{} {}", PENCIL, style("Analyzing diff...").cyan().bold());
            let repo = git::GitRepo::open(".")?;
            
            if !repo.has_staged_changes()? {
                println!("\n{} {}", CROSS, style("No staged changes found. Stage some changes first with 'git add'").yellow());
                return Ok(());
            }

            let changes = repo.get_staged_changes()?;
            
            // Print summary statistics
            println!("\n{} {}", SPARKLE, style("Summary").cyan().bold().underlined());
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
                println!("\n{} {}", SPARKLE, style("Added files:").cyan().bold());
                for file in changes.added {
                    println!("  {} {}", "+".green().bold(), style(file).green());
                }
            }
            
            if !changes.modified.is_empty() {
                println!("\n{} {}", SPARKLE, style("Modified files:").cyan().bold());
                for file in changes.modified {
                    println!("  {} {}", "*".yellow().bold(), style(file).yellow());
                }
            }
            
            if !changes.deleted.is_empty() {
                println!("\n{} {}", SPARKLE, style("Deleted files:").cyan().bold());
                for file in changes.deleted {
                    println!("  {} {}", "-".red().bold(), style(file).red());
                }
            }
            
            if !changes.renamed.is_empty() {
                println!("\n{} {}", SPARKLE, style("Renamed files:").cyan().bold());
                for (old, new) in changes.renamed {
                    println!("  {} {} {} {}", 
                        "→".blue().bold(),
                        style(old).strikethrough(),
                        "→".blue().bold(),
                        style(new).blue()
                    );
                }
            }

            // Print detailed diff
            println!("\n{} {}", SPARKLE, style("Detailed changes:").cyan().bold().underlined());
            let hunks = repo.get_structured_diff()?;
            for hunk in hunks {
                println!("\n{}", style(hunk.header).cyan());
                for line in hunk.lines {
                    match line.origin {
                        '+' => print!("{}", style(line.content).green()),
                        '-' => print!("{}", style(line.content).red()),
                        _ => print!("{}", style(line.content).dim()),
                    }
                }
            }
        }
    }

    Ok(())
}