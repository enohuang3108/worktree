use anyhow::{Result, anyhow};
use std::path::PathBuf;
use std::process::Command;
use std::fs;

#[derive(Debug, Clone, PartialEq)]
pub struct WorktreeInfo {
    pub path: PathBuf,
    pub branch: String,
    pub commit: String,
    pub is_current: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BranchMode {
    NewBranch,
    ExistingBranch,
}

pub struct WorktreeManager {
    repo_path: PathBuf,
}

impl WorktreeManager {
    pub fn new(repo_path: PathBuf) -> Self {
        Self { repo_path }
    }

    pub fn list_worktrees(&self) -> Result<Vec<WorktreeInfo>> {
        // 使用 git worktree list --porcelain 來獲取 worktree 資訊
        let output = Command::new("git")
            .args(&["worktree", "list", "--porcelain"])
            .current_dir(&self.repo_path)
            .output()?;

        if !output.status.success() {
            return Err(anyhow!("Failed to list worktrees: {}", 
                String::from_utf8_lossy(&output.stderr)));
        }

        let output_str = String::from_utf8_lossy(&output.stdout);
        let mut worktrees = Vec::new();
        let mut current_worktree: Option<WorktreeInfo> = None;
        
        for line in output_str.lines() {
            if line.starts_with("worktree ") {
                // 如果有之前的 worktree，先添加它
                if let Some(wt) = current_worktree.take() {
                    worktrees.push(wt);
                }
                
                // 開始新的 worktree
                let path = PathBuf::from(&line[9..]);
                current_worktree = Some(WorktreeInfo {
                    path,
                    branch: String::new(),
                    commit: String::new(),
                    is_current: false,
                });
            } else if line.starts_with("HEAD ") && current_worktree.is_some() {
                if let Some(ref mut wt) = current_worktree {
                    wt.commit = line[5..].to_string();
                }
            } else if line.starts_with("branch ") && current_worktree.is_some() {
                if let Some(ref mut wt) = current_worktree {
                    let branch_ref = &line[7..];
                    // 提取分支名稱 (移除 refs/heads/ 前綴)
                    wt.branch = branch_ref.strip_prefix("refs/heads/")
                        .unwrap_or(branch_ref)
                        .to_string();
                }
            } else if line == "bare" && current_worktree.is_some() {
                // 跳過 bare repository
                if let Some(ref mut wt) = current_worktree {
                    wt.branch = "(bare)".to_string();
                }
            }
        }
        
        // 添加最後一個 worktree
        if let Some(wt) = current_worktree {
            worktrees.push(wt);
        }

        // 移除主倉庫 (通常是第一個)
        worktrees.retain(|wt| wt.branch != "(bare)" && !wt.path.ends_with(".git"));
        
        Ok(worktrees)
    }

    pub fn add_worktree(&self, path: &PathBuf, branch: &str) -> Result<()> {
        // 確保父目錄存在
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        // 使用 git worktree add 命令
        let output = Command::new("git")
            .args(&["worktree", "add", &path.to_string_lossy(), branch])
            .current_dir(&self.repo_path)
            .output()?;

        if !output.status.success() {
            let error_message = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow!("Failed to add worktree: {}", error_message));
        }

        Ok(())
    }

    pub fn remove_worktree(&self, path: &PathBuf) -> Result<()> {
        // 使用 git worktree remove 命令
        let output = Command::new("git")
            .args(&["worktree", "remove", &path.to_string_lossy()])
            .current_dir(&self.repo_path)
            .output()?;

        if !output.status.success() {
            let error_message = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow!("Failed to remove worktree: {}", error_message));
        }

        Ok(())
    }
}