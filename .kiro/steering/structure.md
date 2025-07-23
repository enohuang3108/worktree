# Project Structure

## Root Directory Organization

```
worktree/
├── .kiro/                    # Spec-driven development files
│   └── steering/            # Project steering documents
├── build/                   # Compiled output directory
│   ├── index.js            # Main bundled executable (ncc)
│   └── wt.js              # Alternative executable (rolldown)
├── dist/                   # TypeScript compilation output
│   ├── cli.js             # Main entry point
│   ├── utils.js           # Utility functions
│   └── commands/          # Command implementations
├── src/                    # Source code directory
│   ├── cli.ts             # Main CLI entry point and command setup
│   ├── utils.ts           # Core utility functions and Git operations
│   └── commands/          # Individual command implementations
├── package.json            # Project configuration and dependencies
├── tsconfig.json          # TypeScript compiler configuration
├── rolldown.config.mjs    # Rolldown bundler configuration (alternative)
├── README.md              # Project documentation
└── CLAUDE.md              # AI assistant instructions
```

## Subdirectory Structures

### `/src` - Source Code
```
src/
├── cli.ts                 # Commander.js setup, command registration, hooks
├── utils.ts               # Git operations, filesystem utilities, interfaces
└── commands/
    ├── add.ts            # Worktree creation command
    ├── remove.ts         # Worktree removal command
    └── open.ts           # VSCode integration command
```

### `/build` - Distribution Files
```
build/
├── index.js              # Single-file executable (ncc bundled)
└── wt.js                # Alternative single-file executable (rolldown)
```

### `/dist` - TypeScript Output
```
dist/
├── cli.js               # Compiled main entry point
├── utils.js             # Compiled utilities
└── commands/
    ├── add.js          # Compiled add command
    ├── remove.js       # Compiled remove command
    └── open.js         # Compiled open command
```

## Code Organization Patterns

### Command Pattern
- **Main CLI (cli.ts)**: Command registration, global hooks, and routing
- **Command Files (commands/)**: Individual command implementations with dedicated files
- **Utils (utils.ts)**: Shared functionality for Git operations and filesystem management

### Separation of Concerns
- **CLI Interface**: Commander.js setup and command definitions
- **Business Logic**: Git worktree operations and validation
- **User Interaction**: Inquirer.js prompts and user input handling
- **External Integration**: Git command execution and VSCode launching

## File Naming Conventions

### TypeScript Files
- **Kebab-case**: Not used in this project
- **Camel-case**: Used for multi-word concepts (`addWorktree`, `openInVSCode`)
- **Command files**: Named after CLI commands (`add.ts`, `remove.ts`, `open.ts`)
- **Utility files**: Descriptive names (`utils.ts`, `cli.ts`)

### Function Naming
- **Command functions**: `[verb]Command` pattern (`addWorktreeCommand`)
- **Utility functions**: Descriptive verbs (`getProjectName`, `createWorktreePath`)
- **Git operations**: `[action][Target]` pattern (`addWorktree`, `removeWorktree`)

## Import Organization

### Import Order (established pattern)
1. **Node.js built-ins**: `child_process`, `fs/promises`, `path`
2. **External dependencies**: `commander`, `inquirer`, `chalk`, `execa`
3. **Internal modules**: `./utils.js`, `./commands/*.js`

### Import Style
- **ES6 imports**: Used throughout TypeScript source
- **Named imports**: Preferred for utilities (`import { execSync } from 'child_process'`)
- **Default imports**: Used for primary exports (`import inquirer from 'inquirer'`)
- **File extensions**: `.js` extension required in imports (TypeScript ES module setup)

## Key Architectural Principles

### Single Responsibility
- Each command file handles one primary user action
- Utils file contains only shared functionality
- Clear separation between CLI setup and command logic

### Git-First Approach
- All operations use native Git commands via `execa`
- No Git library dependencies - direct command execution
- Git repository validation before any operations

### Interactive UX Design
- All user inputs go through Inquirer.js prompts
- Consistent error handling with colored output
- Confirmation prompts for destructive operations

### Build Flexibility
- Multiple build targets (ncc, rolldown) for different deployment needs
- TypeScript compilation separate from bundling
- Single-file executables for easy distribution

### Error Handling Philosophy
- Graceful error messages with colored output
- Process exit on critical errors
- User-friendly error descriptions over technical details