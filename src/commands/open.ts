import inquirer from 'inquirer';
import chalk from 'chalk';
import { getWorktrees, openInVSCode } from '../utils.js';

export async function openWorktreeCommand(): Promise<void> {
  try {
    console.log(chalk.blue('🖥️  Opening a Git worktree in VSCode\n'));

    const worktrees = await getWorktrees();

    if (worktrees.length === 0) {
      console.log(chalk.yellow('No worktrees found to open.'));
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
        message: 'Select a worktree to open:',
        choices
      }
    ]);

    console.log(chalk.cyan(`\n🖥️  Opening worktree in VSCode: ${selectedWorktree}`));

    // Open in VSCode
    await openInVSCode(selectedWorktree);
    
    console.log(chalk.green('✅ Worktree opened in VSCode!'));
    
  } catch (error) {
    console.error(chalk.red(`❌ Error: ${error}`));
    process.exit(1);
  }
}