use anyhow::Result;
use inquire::{Select, Text, Confirm};
use crate::git::{branches::Branch, worktree::{BranchMode, WorktreeInfo}};
use crate::utils::validation::validate_branch_name;

pub struct Prompts;

impl Prompts {
    pub fn select_branch_mode() -> Result<BranchMode> {
        let options = vec!["Create new branch", "Use existing branch"];
        let selection = Select::new("Would you like to create a new branch or use an existing one?", options)
            .with_help_message("Use arrow keys to navigate, Enter to select")
            .prompt()?;

        match selection {
            "Create new branch" => Ok(BranchMode::NewBranch),
            "Use existing branch" => Ok(BranchMode::ExistingBranch),
            _ => unreachable!(),
        }
    }

    pub fn input_branch_name() -> Result<String> {
        let branch_name = Text::new("Enter new branch name:")
            .with_help_message("Branch name should not contain spaces or special characters")
            .with_validator(|input: &str| {
                match validate_branch_name(input) {
                    Ok(_) => Ok(inquire::validator::Validation::Valid),
                    Err(e) => Ok(inquire::validator::Validation::Invalid(
                        inquire::validator::ErrorMessage::Custom(e.to_string())
                    )),
                }
            })
            .prompt()?;

        Ok(branch_name)
    }

    pub fn select_base_branch(branches: Vec<Branch>) -> Result<Branch> {
        if branches.is_empty() {
            return Err(anyhow::anyhow!("No branches available"));
        }

        let branch_names: Vec<String> = branches.iter().map(|b| {
            if b.is_remote {
                format!("{} (remote)", b.name)
            } else {
                b.name.clone()
            }
        }).collect();

        let selection = Select::new("Select base branch:", branch_names.clone())
            .with_help_message("This will be the starting point for your new branch")
            .prompt()?;

        // 找到對應的分支
        let index = branch_names.iter().position(|name| name == &selection)
            .ok_or_else(|| anyhow::anyhow!("Selected branch not found"))?;

        Ok(branches[index].clone())
    }

    pub fn select_existing_branch(branches: Vec<Branch>) -> Result<Branch> {
        if branches.is_empty() {
            return Err(anyhow::anyhow!("No branches available"));
        }

        let branch_names: Vec<String> = branches.iter().map(|b| {
            if b.is_remote {
                format!("{} (remote)", b.name)
            } else {
                b.name.clone()
            }
        }).collect();

        let selection = Select::new("Select existing branch:", branch_names.clone())
            .with_help_message("Choose the branch to create a worktree for")
            .prompt()?;

        // 找到對應的分支
        let index = branch_names.iter().position(|name| name == &selection)
            .ok_or_else(|| anyhow::anyhow!("Selected branch not found"))?;

        Ok(branches[index].clone())
    }

    pub fn select_worktree(worktrees: Vec<WorktreeInfo>) -> Result<WorktreeInfo> {
        if worktrees.is_empty() {
            return Err(anyhow::anyhow!("No worktrees available"));
        }

        let worktree_descriptions: Vec<String> = worktrees.iter().map(|wt| {
            format!("{} ({})", wt.branch, wt.path.display())
        }).collect();

        let selection = Select::new("Select worktree:", worktree_descriptions.clone())
            .with_help_message("Choose the worktree to work with")
            .prompt()?;

        // 找到對應的 worktree
        let index = worktree_descriptions.iter().position(|desc| desc == &selection)
            .ok_or_else(|| anyhow::anyhow!("Selected worktree not found"))?;

        Ok(worktrees[index].clone())
    }

    pub fn confirm_removal(worktree: &WorktreeInfo) -> Result<bool> {
        let message = format!(
            "Are you sure you want to remove the worktree '{}'?\nPath: {}",
            worktree.branch,
            worktree.path.display()
        );

        let confirmed = Confirm::new(&message)
            .with_default(false)
            .with_help_message("This action cannot be undone")
            .prompt()?;

        Ok(confirmed)
    }

    pub fn confirm_vscode_open(path: &str) -> Result<bool> {
        let message = format!("Open worktree in VSCode?\nPath: {}", path);

        let confirmed = Confirm::new(&message)
            .with_default(true)
            .with_help_message("This will launch VSCode with the worktree directory")
            .prompt()?;

        Ok(confirmed)
    }
}