use clap::Args;
use anyhow::Result;
use colored::Colorize;
use dialoguer::Select;
use console::style;

use crate::utils::{
    git::get_worktrees,
    vscode::open_in_vscode,
};

#[derive(Debug, Args)]
pub struct OpenCommand {}

impl OpenCommand {
    pub async fn execute(&self) -> Result<()> {
        println!("{} Opening a worktree in VSCode...", "🖥️".bright_blue());
        
        // Get worktrees
        let worktrees = get_worktrees().await?;
        
        if worktrees.is_empty() {
            println!("{} No worktrees found to open.", "ℹ️".bright_blue());
            return Ok(());
        }
        
        // Create choices for selection
        let choices: Vec<String> = worktrees
            .iter()
            .map(|wt| format!("{} ({})", wt.branch, wt.path))
            .collect();
        
        // Select worktree to open
        let selection = Select::new()
            .with_prompt("Select worktree to open in VSCode")
            .items(&choices)
            .interact()?;
        
        let selected_worktree = &worktrees[selection];
        
        println!();
        println!("Opening worktree:");
        println!("  Branch: {}", style(&selected_worktree.branch).cyan());
        println!("  Path: {}", style(&selected_worktree.path).yellow());
        if let Some(ref commit) = selected_worktree.commit {
            println!("  Commit: {}", style(commit).dim());
        }
        println!();
        
        println!("{} Opening VSCode...", "⚙️".bright_blue());
        
        match open_in_vscode(&selected_worktree.path).await {
            Ok(_) => {
                println!(
                    "{} VSCode opened successfully for worktree '{}'!",
                    "✅".bright_green(),
                    selected_worktree.branch
                );
            }
            Err(e) => {
                eprintln!(
                    "{} Failed to open VSCode for worktree '{}': {}",
                    "❌".red(),
                    selected_worktree.branch,
                    e
                );
            }
        }
        
        Ok(())
    }
}