# wt – Git Worktree CLI Tool

Easily manage Git worktrees with interactive CLI prompts.

## Features

- 📁 **Add a new worktree** - Create worktrees from new or existing branches with interactive prompts
- ❌ **Remove existing worktrees** - Safely remove worktrees with confirmation
- 🖥️ **Open worktree in VSCode** - Quickly open any worktree in your editor
- 🎯 **Interactive interface** - User-friendly prompts for all operations
- 🚀 **Cross-platform** - Works on Linux, macOS, and Windows

## Installation

### Quick Install (Recommended)

```bash
curl -fsSL https://raw.githubusercontent.com/yourname/wt-cli/main/install.sh | bash
```

### Manual Installation

1. Download the appropriate binary for your platform from [releases](https://github.com/yourname/wt-cli/releases)
2. Make it executable: `chmod +x wt`
3. Move to your PATH: `sudo mv wt /usr/local/bin/`

### Development Installation

```bash
# Clone the repository
git clone https://github.com/yourname/wt-cli.git
cd wt-cli

# Install dependencies
pnpm install

# Build the project
pnpm run build

# Link globally
pnpm link --global
```

## Usage

### Commands

- `wt add` - Create a new worktree
- `wt remove` (aliases: `rm`, `delete`) - Remove an existing worktree
- `wt open` - Open a worktree in VSCode
- `wt --help` - Show help information

### Examples

```bash
# Create a new worktree
wt add

# Remove a worktree
wt remove

# Open a worktree in VSCode
wt open

# Show help
wt --help
```

## How it Works

### Adding a Worktree (`wt add`)

1. Choose to create a new branch or use an existing one
2. If new branch: enter branch name and select base branch
3. If existing: select from available branches
4. Creates worktree in `../<project-name>-worktree/<project-name>-<branch>-worktree`
5. Optionally opens in VSCode

### Removing a Worktree (`wt remove`)

1. Lists all existing worktrees
2. Select which one to remove
3. Confirms the deletion
4. Removes the worktree and cleans up

### Opening a Worktree (`wt open`)

1. Lists all existing worktrees
2. Select which one to open
3. Opens the selected worktree in VSCode

## Requirements

- Git (with worktree support)
- Node.js 18+ (for development)
- VSCode (optional, for opening worktrees)

## Development

This project uses:
- TypeScript
- Commander.js for CLI interface
- Inquirer.js for interactive prompts
- Chalk for colored output
- Execa for process execution
- pnpm as package manager

### Development Commands

```bash
# Install dependencies
pnpm install

# Run in development mode
pnpm run dev

# Build the project
pnpm run build

# Create executable binaries
pnpm run pkg
```
