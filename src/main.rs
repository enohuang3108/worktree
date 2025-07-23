use clap::Parser;
use anyhow::Result;

mod cli;
mod commands;
mod git;
mod ui;
mod utils;
mod error;

use cli::{Cli, Commands};
use git::repository::Repository;
use ui::display::Display;

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Check if we're in a git repository
    if !Repository::is_git_repository() {
        Display::show_error("Not in a Git repository");
        std::process::exit(1);
    }

    match cli.command {
        Commands::Add(cmd) => cmd.execute(),
        Commands::Remove(cmd) => cmd.execute(),
        Commands::Open(cmd) => cmd.execute(),
    }
}