use anyhow::{Result, Context};
use git2::Repository;
use std::process::Command;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorktreeInfo {
    pub path: String,
    pub branch: String,
    pub commit: Option<String>,
}

/// Check if current directory is a git repository
pub fn is_git_repository() -> Result<bool> {
    match Repository::open(".") {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

/// Get project name from git remote URL or current directory
pub fn get_project_name() -> Result<String> {
    // Try to get from git remote
    if let Ok(output) = Command::new("git")
        .args(&["config", "--get", "remote.origin.url"])
        .output()
    {
        if output.status.success() {
            let url_bytes = String::from_utf8_lossy(&output.stdout);
            let url = url_bytes.trim();
            if let Some(name) = extract_project_name_from_url(url) {
                return Ok(name);
            }
        }
    }
    
    // Fallback to current directory name
    let current_dir = std::env::current_dir()
        .context("Failed to get current directory")?;
    let project_name = current_dir
        .file_name()
        .context("Failed to get directory name")?
        .to_string_lossy()
        .to_string();
    
    Ok(project_name)
}

/// Extract project name from git URL
fn extract_project_name_from_url(url: &str) -> Option<String> {
    let url = url.trim();
    
    // Handle SSH URLs like git@github.com:user/repo.git
    if url.starts_with("git@") {
        if let Some(colon_pos) = url.find(':') {
            let path_part = &url[colon_pos + 1..];
            if let Some(last_slash) = path_part.rfind('/') {
                let repo_name = &path_part[last_slash + 1..];
                return Some(repo_name.trim_end_matches(".git").to_string());
            }
        }
    }
    
    // Handle HTTPS URLs like https://github.com/user/repo.git
    if url.starts_with("http") {
        if let Some(last_slash) = url.rfind('/') {
            let repo_name = &url[last_slash + 1..];
            return Some(repo_name.trim_end_matches(".git").to_string());
        }
    }
    
    None
}

/// Get all branches (local and remote)
pub async fn get_branches() -> Result<Vec<String>> {
    let output = tokio::process::Command::new("git")
        .args(&["branch", "-a"])
        .output()
        .await
        .context("Failed to execute git branch command")?;
    
    if !output.status.success() {
        anyhow::bail!("Git branch command failed");
    }
    
    let branches_text = String::from_utf8_lossy(&output.stdout);
    let mut branches: Vec<String> = branches_text
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty() && !line.contains("HEAD"))
        .map(|line| {
            // Remove current branch marker and whitespace
            let cleaned = line.trim_start_matches('*').trim();
            // Remove remotes/origin/ prefix
            if cleaned.starts_with("remotes/origin/") {
                cleaned.strip_prefix("remotes/origin/").unwrap_or(cleaned)
            } else {
                cleaned
            }
        })
        .map(String::from)
        .collect();
    
    // Remove duplicates and sort
    branches.sort();
    branches.dedup();
    
    Ok(branches)
}

/// Get all worktrees
pub async fn get_worktrees() -> Result<Vec<WorktreeInfo>> {
    let output = tokio::process::Command::new("git")
        .args(&["worktree", "list", "--porcelain"])
        .output()
        .await
        .context("Failed to execute git worktree list command")?;
    
    if !output.status.success() {
        anyhow::bail!("Git worktree list command failed");
    }
    
    let worktree_text = String::from_utf8_lossy(&output.stdout);
    let mut worktrees = Vec::new();
    let mut current_worktree: Option<WorktreeInfo> = None;
    
    for line in worktree_text.lines() {
        let line = line.trim();
        if line.is_empty() {
            if let Some(wt) = current_worktree.take() {
                worktrees.push(wt);
            }
            continue;
        }
        
        if line.starts_with("worktree ") {
            current_worktree = Some(WorktreeInfo {
                path: line.strip_prefix("worktree ").unwrap_or("").to_string(),
                branch: String::new(),
                commit: None,
            });
        } else if line.starts_with("branch ") {
            if let Some(ref mut wt) = current_worktree {
                wt.branch = line.strip_prefix("branch refs/heads/").unwrap_or(
                    line.strip_prefix("branch ").unwrap_or("")
                ).to_string();
            }
        } else if line.starts_with("HEAD ") {
            if let Some(ref mut wt) = current_worktree {
                wt.commit = Some(line.strip_prefix("HEAD ").unwrap_or("").to_string());
            }
        }
    }
    
    // Add the last worktree if exists
    if let Some(wt) = current_worktree {
        worktrees.push(wt);
    }
    
    // Filter out the current working directory
    let current_dir = std::env::current_dir()?;
    let current_path = current_dir.to_string_lossy();
    
    Ok(worktrees.into_iter()
        .filter(|wt| !wt.path.ends_with(&*current_path))
        .collect())
}

/// Add a new worktree
pub async fn add_worktree(
    worktree_path: &str,
    branch_name: &str,
    is_new_branch: bool,
    base_branch: Option<&str>,
) -> Result<()> {
    let mut args = vec!["worktree", "add"];
    
    if is_new_branch {
        args.push("-b");
        args.push(branch_name);
        args.push(worktree_path);
        if let Some(base) = base_branch {
            args.push(base);
        }
    } else {
        args.push(worktree_path);
        args.push(branch_name);
    }
    
    let output = tokio::process::Command::new("git")
        .args(&args)
        .output()
        .await
        .context("Failed to execute git worktree add command")?;
    
    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Failed to add worktree: {}", error_msg);
    }
    
    Ok(())
}

/// Remove a worktree
pub async fn remove_worktree(worktree_path: &str) -> Result<()> {
    let output = tokio::process::Command::new("git")
        .args(&["worktree", "remove", worktree_path])
        .output()
        .await
        .context("Failed to execute git worktree remove command")?;
    
    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Failed to remove worktree: {}", error_msg);
    }
    
    Ok(())
}