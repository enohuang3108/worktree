# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust CLI tool called `wt` (worktree-cli) that provides an interactive interface for managing Git worktrees. The project migrates from a previous Markdown-based command system to a native executable built with Rust.

### Key Dependencies
- `clap` with derive features for CLI argument parsing
- `inquire` for interactive prompts
- `owo-colors` for terminal colors  
- `git2` with vendored-openssl for Git operations
- `anyhow` for error handling

## Architecture

The codebase follows a modular structure:

- `src/main.rs` - Entry point with basic git repository validation
- `src/cli.rs` - Command-line interface definition using clap
- `src/commands/` - Command implementations (add, remove, open)
- `src/git/` - Git operations (repository, branches, worktree management)
- `src/ui/` - User interface components (prompts, display, colors)
- `src/utils/` - Utility functions (paths, system, validation)
- `src/error.rs` - Error handling

### Command Structure
Each command follows the pattern:
1. Defined in `src/cli.rs` as enum variants
2. Implemented in separate modules under `src/commands/`
3. Commands use interactive prompts via the `inquire` crate
4. Git operations are abstracted through the `src/git/` modules

## Development Commands

### Build and Run
```bash
cargo build                    # Build debug version
cargo build --release         # Build optimized release version
cargo run -- <command>        # Run with arguments
cargo run -- add             # Example: run the add command
```

### Testing and Quality
```bash
cargo test                    # Run all tests
cargo clippy                  # Run linter
cargo fmt                    # Format code
```

### Installation Testing
```bash
./install.sh                 # Test installation script
```

## Important Notes

- The binary name is `wt` (defined in Cargo.toml)
- All commands require being run from within a Git repository
- The project uses `git2` with vendored OpenSSL for cross-platform compatibility
- Interactive prompts are handled through the `inquire` crate
- Terminal color detection is implemented for better user experience

## Active Specifications
- **rust-cli-migration**: Rust CLI 重構專案，將現有 Markdown 命令系統遷移至原生可執行檔案