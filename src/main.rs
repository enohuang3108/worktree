use clap::{Parser, Subcommand};
use anyhow::Result;

mod commands;
mod utils;

use commands::{add::AddCommand, remove::RemoveCommand, open::OpenCommand};
use utils::git::is_git_repository;

#[derive(Parser)]
#[command(name = "wt")]
#[command(about = "Git Worktree CLI Tool - Easily manage Git worktrees with interactive CLI prompts")]
#[command(version = "1.0.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new worktree
    Add(AddCommand),
    /// Remove an existing worktree
    #[command(alias = "rm")]
    #[command(alias = "delete")]
    Remove(RemoveCommand),
    /// Open an existing worktree in VSCode
    Open(OpenCommand),
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Check if we're in a git repository
    if !is_git_repository()? {
        eprintln!("{}", colored::Colorize::red("Error: Not in a Git repository"));
        std::process::exit(1);
    }

    match cli.command {
        Commands::Add(cmd) => cmd.execute().await,
        Commands::Remove(cmd) => cmd.execute().await,
        Commands::Open(cmd) => cmd.execute().await,
    }
}