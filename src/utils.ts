import { execSync } from 'child_process';
import { execa } from 'execa';
import fs from 'fs/promises';
import path from 'path';

export interface WorktreeInfo {
  path: string;
  branch: string;
  commit: string;
}

/**
 * Get current project name from git repository
 */
export function getProjectName(): string {
  try {
    const remoteUrl = execSync('git config --get remote.origin.url', { encoding: 'utf-8' }).trim();
    const match = remoteUrl.match(/\/([^\/]+?)(?:\.git)?$/);
    if (match) {
      return match[1];
    }
  } catch {
    // Fallback to current directory name
  }
  return path.basename(process.cwd());
}

/**
 * Get all existing Git branches
 */
export async function getBranches(): Promise<string[]> {
  try {
    const { stdout } = await execa('git', ['branch', '-a']);
    return stdout
      .split('\n')
      .map(line => line.replace(/^\*?\s+/, '').replace(/^remotes\/origin\//, ''))
      .filter(line => line && !line.includes('HEAD'))
      .filter((branch, index, self) => self.indexOf(branch) === index);
  } catch (error) {
    throw new Error('Failed to get Git branches. Make sure you are in a Git repository.');
  }
}

/**
 * Get all existing worktrees
 */
export async function getWorktrees(): Promise<WorktreeInfo[]> {
  try {
    const { stdout } = await execa('git', ['worktree', 'list', '--porcelain']);
    const worktrees: WorktreeInfo[] = [];
    const lines = stdout.split('\n');
    
    let currentWorktree: Partial<WorktreeInfo> = {};
    
    for (const line of lines) {
      if (line.startsWith('worktree ')) {
        currentWorktree.path = line.replace('worktree ', '');
      } else if (line.startsWith('branch ')) {
        currentWorktree.branch = line.replace('branch refs/heads/', '');
      } else if (line.startsWith('HEAD ')) {
        currentWorktree.commit = line.replace('HEAD ', '');
      } else if (line === '' && currentWorktree.path) {
        worktrees.push(currentWorktree as WorktreeInfo);
        currentWorktree = {};
      }
    }
    
    if (currentWorktree.path) {
      worktrees.push(currentWorktree as WorktreeInfo);
    }
    
    return worktrees.filter(wt => !wt.path.endsWith(process.cwd()));
  } catch (error) {
    throw new Error('Failed to get worktrees. Make sure you are in a Git repository.');
  }
}

/**
 * Create worktree directory path
 */
export function createWorktreePath(projectName: string, branchName: string): string {
  const parentDir = path.dirname(process.cwd());
  const worktreeDir = `${projectName}-worktree`;
  const worktreeName = `${projectName}-${branchName}-worktree`;
  return path.join(parentDir, worktreeDir, worktreeName);
}

/**
 * Ensure worktree parent directory exists
 */
export async function ensureWorktreeDir(worktreePath: string): Promise<void> {
  const parentDir = path.dirname(worktreePath);
  try {
    await fs.access(parentDir);
  } catch {
    await fs.mkdir(parentDir, { recursive: true });
  }
}

/**
 * Add a new worktree
 */
export async function addWorktree(worktreePath: string, branchName: string, isNewBranch: boolean, baseBranch?: string): Promise<void> {
  try {
    await ensureWorktreeDir(worktreePath);
    
    if (isNewBranch && baseBranch) {
      await execa('git', ['worktree', 'add', '-b', branchName, worktreePath, baseBranch]);
    } else {
      await execa('git', ['worktree', 'add', worktreePath, branchName]);
    }
  } catch (error) {
    throw new Error(`Failed to add worktree: ${error}`);
  }
}

/**
 * Remove a worktree
 */
export async function removeWorktree(worktreePath: string): Promise<void> {
  try {
    await execa('git', ['worktree', 'remove', worktreePath]);
  } catch (error) {
    throw new Error(`Failed to remove worktree: ${error}`);
  }
}

/**
 * Open directory in VSCode
 */
export async function openInVSCode(dirPath: string): Promise<void> {
  try {
    await execa('code', [dirPath]);
  } catch (error) {
    throw new Error(`Failed to open VSCode. Make sure 'code' command is available in PATH.`);
  }
}

/**
 * Check if we are in a Git repository
 */
export function isGitRepository(): boolean {
  try {
    execSync('git rev-parse --git-dir', { stdio: 'ignore' });
    return true;
  } catch {
    return false;
  }
}
