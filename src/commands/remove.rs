use clap::Args;
use anyhow::Result;
use colored::Colorize;
use dialoguer::{Select, Confirm};
use console::style;

use crate::utils::git::{get_worktrees, remove_worktree};

#[derive(Debug, Args)]
pub struct RemoveCommand {}

impl RemoveCommand {
    pub async fn execute(&self) -> Result<()> {
        println!("{} Removing a worktree...", "🗑️".bright_red());
        
        // Get worktrees
        let worktrees = get_worktrees().await?;
        
        if worktrees.is_empty() {
            println!("{} No worktrees found to remove.", "ℹ️".bright_blue());
            return Ok(());
        }
        
        // Create choices for selection
        let choices: Vec<String> = worktrees
            .iter()
            .map(|wt| format!("{} ({})", wt.branch, wt.path))
            .collect();
        
        // Select worktree to remove
        let selection = Select::new()
            .with_prompt("Select worktree to remove")
            .items(&choices)
            .interact()?;
        
        let selected_worktree = &worktrees[selection];
        
        println!();
        println!("Selected worktree:");
        println!("  Branch: {}", style(&selected_worktree.branch).cyan());
        println!("  Path: {}", style(&selected_worktree.path).yellow());
        if let Some(ref commit) = selected_worktree.commit {
            println!("  Commit: {}", style(commit).dim());
        }
        println!();
        
        // Confirm deletion
        let confirm = Confirm::new()
            .with_prompt(&format!(
                "Are you sure you want to remove the worktree '{}'?",
                selected_worktree.branch
            ))
            .default(false)
            .interact()?;
        
        if confirm {
            println!("{} Removing worktree...", "⚙️".bright_blue());
            
            match remove_worktree(&selected_worktree.path).await {
                Ok(_) => {
                    println!(
                        "{} Worktree '{}' removed successfully!",
                        "✅".bright_green(),
                        selected_worktree.branch
                    );
                }
                Err(e) => {
                    eprintln!(
                        "{} Failed to remove worktree '{}': {}",
                        "❌".red(),
                        selected_worktree.branch,
                        e
                    );
                }
            }
        } else {
            println!("{} Operation cancelled.", "ℹ️".bright_blue());
        }
        
        Ok(())
    }
}