import inquirer from 'inquirer';
import chalk from 'chalk';
import { getWorktrees, removeWorktree } from '../utils.js';

export async function removeWorktreeCommand(): Promise<void> {
  try {
    console.log(chalk.blue('🗑️  Removing a Git worktree\n'));

    const worktrees = await getWorktrees();

    if (worktrees.length === 0) {
      console.log(chalk.yellow('No worktrees found to remove.'));
      return;
    }

    // Create choices for inquirer
    const choices = worktrees.map(wt => ({
      name: `${wt.branch} (${wt.path})`,
      value: wt.path
    }));

    const { selectedWorktree } = await inquirer.prompt([
      {
        type: 'list',
        name: 'selectedWorktree',
        message: 'Select a worktree to remove:',
        choices
      }
    ]);

    // Confirm deletion
    const { confirmDelete } = await inquirer.prompt([
      {
        type: 'confirm',
        name: 'confirmDelete',
        message: `Are you sure you want to remove this worktree?\n  ${selectedWorktree}`,
        default: false
      }
    ]);

    if (!confirmDelete) {
      console.log(chalk.yellow('Operation cancelled.'));
      return;
    }

    console.log(chalk.cyan(`\n🗑️  Removing worktree: ${selectedWorktree}`));

    // Remove the worktree
    await removeWorktree(selectedWorktree);
    
    console.log(chalk.green('✅ Worktree removed successfully!'));
    
  } catch (error) {
    console.error(chalk.red(`❌ Error: ${error}`));
    process.exit(1);
  }
}