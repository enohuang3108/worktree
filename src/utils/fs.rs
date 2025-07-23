use anyhow::{Result, Context};
use std::path::Path;

/// Create worktree path based on project name and branch name
pub fn create_worktree_path(project_name: &str, branch_name: &str) -> Result<String> {
    let current_dir = std::env::current_dir()
        .context("Failed to get current directory")?;
    
    let parent_dir = current_dir.parent()
        .context("Failed to get parent directory")?;
    
    let worktree_dir = format!("{}-worktree", project_name);
    let worktree_name = format!("{}-{}-worktree", project_name, branch_name);
    
    let worktree_path = parent_dir
        .join(&worktree_dir)
        .join(&worktree_name);
    
    Ok(worktree_path.to_string_lossy().to_string())
}

/// Ensure worktree directory exists
pub async fn ensure_worktree_dir<P: AsRef<Path>>(worktree_path: P) -> Result<()> {
    let path = worktree_path.as_ref();
    
    if let Some(parent) = path.parent() {
        tokio::fs::create_dir_all(parent).await
            .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
    }
    
    Ok(())
}