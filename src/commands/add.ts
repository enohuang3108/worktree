import inquirer from 'inquirer';
import chalk from 'chalk';
import { 
  getProjectName, 
  getBranches, 
  createWorktreePath, 
  addWorktree, 
  openInVSCode 
} from '../utils.js';

export async function addWorktreeCommand(): Promise<void> {
  try {
    console.log(chalk.blue('🌳 Creating a new Git worktree\n'));

    const projectName = getProjectName();
    const branches = await getBranches();

    // Ask user to choose between new branch or existing branch
    const { branchType } = await inquirer.prompt([
      {
        type: 'list',
        name: 'branchType',
        message: 'What would you like to do?',
        choices: [
          { name: 'Create a new branch', value: 'new' },
          { name: 'Use an existing branch', value: 'existing' }
        ]
      }
    ]);

    let branchName: string;
    let baseBranch: string | undefined;
    let isNewBranch: boolean;

    if (branchType === 'new') {
      // Create new branch
      isNewBranch = true;
      
      const newBranchAnswers = await inquirer.prompt([
        {
          type: 'input',
          name: 'branchName',
          message: 'Enter the new branch name:',
          validate: (input: string) => {
            if (!input.trim()) return 'Branch name cannot be empty';
            if (input.includes(' ')) return 'Branch name cannot contain spaces';
            return true;
          }
        },
        {
          type: 'list',
          name: 'baseBranch',
          message: 'Select the base branch:',
          choices: branches
        }
      ]);
      
      branchName = newBranchAnswers.branchName.trim();
      baseBranch = newBranchAnswers.baseBranch;
    } else {
      // Use existing branch
      isNewBranch = false;
      
      const { selectedBranch } = await inquirer.prompt([
        {
          type: 'list',
          name: 'selectedBranch',
          message: 'Select a branch:',
          choices: branches
        }
      ]);
      
      branchName = selectedBranch;
    }

    // Create worktree path
    const worktreePath = createWorktreePath(projectName, branchName);
    
    console.log(chalk.cyan(`\n📁 Creating worktree at: ${worktreePath}`));

    // Create the worktree
    await addWorktree(worktreePath, branchName, isNewBranch, baseBranch);
    
    console.log(chalk.green('✅ Worktree created successfully!'));

    // Ask if user wants to open in VSCode
    const { openVSCode } = await inquirer.prompt([
      {
        type: 'confirm',
        name: 'openVSCode',
        message: 'Open the worktree in VSCode?',
        default: true
      }
    ]);

    if (openVSCode) {
      console.log(chalk.cyan('🖥️  Opening in VSCode...'));
      await openInVSCode(worktreePath);
    }

    console.log(chalk.green('\n🎉 All done!'));
    
  } catch (error) {
    console.error(chalk.red(`❌ Error: ${error}`));
    process.exit(1);
  }
}