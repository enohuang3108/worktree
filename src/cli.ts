#!/usr/bin/env node

import { Command } from 'commander';
import chalk from 'chalk';
import { isGitRepository } from './utils.js';
import { addWorktreeCommand } from './commands/add.js';
import { removeWorktreeCommand } from './commands/remove.js';
import { openWorktreeCommand } from './commands/open.js';

const program = new Command();

program
  .name('wt')
  .description('Git Worktree CLI Tool - Easily manage Git worktrees with interactive CLI prompts')
  .version('1.0.0');

// Check if in git repository before running commands
program.hook('preAction', () => {
  if (!isGitRepository()) {
    console.error(chalk.red('Error: Not in a Git repository'));
    process.exit(1);
  }
});

// Add command
program
  .command('add')
  .description('Create a new worktree')
  .action(addWorktreeCommand);

// Remove command (with aliases)
program
  .command('remove')
  .alias('rm')
  .alias('delete')
  .description('Remove an existing worktree')
  .action(removeWorktreeCommand);

// Open command
program
  .command('open')
  .description('Open an existing worktree in VSCode')
  .action(openWorktreeCommand);

program.parse();