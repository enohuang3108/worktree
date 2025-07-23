use clap::Args;
use anyhow::Result;
use colored::Colorize;
use dialoguer::{Select, Input, Confirm};
use console::style;

use crate::utils::{
    git::{get_project_name, get_branches, add_worktree},
    fs::{create_worktree_path, ensure_worktree_dir},
    vscode::open_in_vscode,
};

#[derive(Debug, Args)]
pub struct AddCommand {}

impl AddCommand {
    pub async fn execute(&self) -> Result<()> {
        println!("{} Creating a new worktree...", "🌳".bright_green());
        
        // Get project name
        let project_name = get_project_name()?;
        println!("Project: {}", style(&project_name).cyan());
        
        // Get branches
        let branches = get_branches().await?;
        if branches.is_empty() {
            eprintln!("{}", "No branches found".red());
            return Ok(());
        }
        
        // Ask for branch type
        let branch_types = vec!["Create new branch", "Use existing branch"];
        let branch_type_selection = Select::new()
            .with_prompt("Would you like to create a new branch or use an existing one?")
            .items(&branch_types)
            .default(0)
            .interact()?;
        
        let (branch_name, is_new_branch, base_branch) = if branch_type_selection == 0 {
            // New branch
            let branch_name: String = Input::new()
                .with_prompt("Enter new branch name")
                .validate_with(|input: &String| -> Result<(), &str> {
                    if input.trim().is_empty() {
                        Err("Branch name cannot be empty")
                    } else if input.contains(' ') {
                        Err("Branch name cannot contain spaces")
                    } else {
                        Ok(())
                    }
                })
                .interact_text()?;
            
            // Select base branch
            let base_selection = Select::new()
                .with_prompt("Select base branch")
                .items(&branches)
                .default(0)
                .interact()?;
            
            let base_branch = branches[base_selection].clone();
            (branch_name, true, Some(base_branch))
        } else {
            // Existing branch
            let branch_selection = Select::new()
                .with_prompt("Select existing branch")
                .items(&branches)
                .default(0)
                .interact()?;
            
            let branch_name = branches[branch_selection].clone();
            (branch_name, false, None)
        };
        
        // Create worktree path
        let worktree_path = create_worktree_path(&project_name, &branch_name)?;
        println!("Worktree path: {}", style(&worktree_path).yellow());
        
        // Ensure directory exists
        ensure_worktree_dir(&worktree_path).await?;
        
        // Add worktree
        println!("{} Creating worktree...", "⚙️".bright_blue());
        add_worktree(&worktree_path, &branch_name, is_new_branch, base_branch.as_deref()).await?;
        
        println!("{} Worktree created successfully!", "✅".bright_green());
        println!("Path: {}", style(&worktree_path).green());
        
        // Ask to open in VSCode
        let open_vscode = Confirm::new()
            .with_prompt("Open in VSCode?")
            .default(true)
            .interact()?;
        
        if open_vscode {
            println!("{} Opening VSCode...", "🖥️".bright_blue());
            if let Err(e) = open_in_vscode(&worktree_path).await {
                eprintln!("{} Failed to open VSCode: {}", "❌".red(), e);
            } else {
                println!("{} VSCode opened successfully!", "✅".bright_green());
            }
        }
        
        Ok(())
    }
}