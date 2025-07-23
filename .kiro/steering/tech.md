# Technology Stack

## Architecture

**Single Binary CLI Application** - Monolithic TypeScript application compiled to a standalone executable that wraps Git worktree functionality with an interactive user interface.

## Frontend

Not applicable - this is a command-line interface application without a traditional frontend.

## Backend

- **Language**: TypeScript 5.3.3
- **Runtime**: Node.js 18+ (ES2020 target)
- **CLI Framework**: Commander.js 11.0.0 for command parsing and routing
- **Interactive Interface**: Inquirer.js 9.2.0 for user prompts and input validation
- **Process Execution**: Execa 8.0.0 for Git command execution
- **Output Styling**: Chalk 5.3.0 for colored terminal output
- **Module System**: CommonJS (compiled from TypeScript ES modules)

## Development Environment

### Required Tools
- **Node.js 18+** - Runtime environment
- **pnpm** - Package manager (preferred over npm/yarn)
- **TypeScript 5.3.3** - Language compiler
- **Git** - Required for worktree operations
- **VSCode** - Optional, for opening worktrees (runtime dependency)

### Build Tools
- **TypeScript Compiler (tsc)** - Primary compilation
- **@vercel/ncc 0.38.3** - Bundle optimizer for single-file output
- **ts-node 10.9.2** - Development TypeScript execution

## Common Commands

```bash
# Development
pnpm install           # Install dependencies
pnpm run dev          # Run in development mode with ts-node
pnpm run build        # Compile TypeScript and bundle with ncc
pnpm run start        # Run compiled JavaScript version

# Testing (manual)
./build/index.js --help    # Test built binary
wt add                     # Test add command
wt remove                  # Test remove command
wt open                    # Test open command
```

## Environment Variables

**None currently used** - The application relies on Git configuration and file system detection rather than environment variables.

## Port Configuration

**Not applicable** - CLI application does not use network ports.

## Build Output Structure

```
build/
├── index.js          # Main bundled executable (ncc output)
└── wt.js            # Alternative executable name (rolldown output)
```

## Git Integration

- **Git Commands Used**: `git worktree`, `git branch`, `git config`, `git rev-parse`
- **Repository Detection**: Uses `git rev-parse --git-dir` to validate Git repository
- **Branch Operations**: Reads local and remote branches via `git branch -a`
- **Worktree Management**: Full Git worktree lifecycle through native Git commands

## Dependencies Philosophy

- **Minimal Dependencies** - Only essential packages for CLI functionality
- **Well-maintained Libraries** - All dependencies are actively maintained with good track records
- **TypeScript First** - All dependencies have TypeScript support or type definitions
- **Node.js Native APIs** - Preference for built-in Node.js modules where possible (fs, path, child_process)