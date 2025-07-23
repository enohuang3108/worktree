use clap::{Parser, Subcommand};

use crate::commands::{add::AddCommand, remove::RemoveCommand, open::OpenCommand};

#[derive(Parser)]
#[command(name = "wt")]
#[command(about = "Git Worktree CLI Tool - Easily manage Git worktrees with interactive CLI prompts")]
#[command(version = "1.0.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new worktree
    Add(AddCommand),
    /// Remove an existing worktree
    #[command(alias = "rm")]
    #[command(alias = "delete")]
    Remove(RemoveCommand),
    /// Open an existing worktree in VSCode
    Open(OpenCommand),
}