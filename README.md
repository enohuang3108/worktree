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
curl -fsSL https://github.com/enohuang3108/worktree/releases/latest/download/install.sh | bash
```

### Manual Installation

1. Download the appropriate binary for your platform from [releases](https://github.com/enohuang3108/worktree/releases)
2. Make it executable: `chmod +x wt`
3. Move to your PATH: `sudo mv wt /usr/local/bin/`

### Build from Source (WSL/Older Linux compatibility)

If you encounter GLIBC compatibility issues on older systems or WSL:

```bash
# Install Rust if not already installed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone and build
git clone https://github.com/enohuang3108/worktree.git
cd worktree
cargo build --release

# Install
sudo cp target/release/wt /usr/local/bin/
```

## Usage

### Commands

- `wt add` - Create a new worktree
- `wt remove` (aliases: `rm`, `delete`) - Remove an existing worktree
- `wt open` - Open a worktree in VSCode
- `wt --help` - Show help information
