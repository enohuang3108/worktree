use anyhow::Result;
use std::path::{Path, PathBuf};
use std::env;
use std::fs;

pub fn generate_worktree_path(project_name: &str, branch_name: &str) -> Result<PathBuf> {
    // 路徑格式：../<project_name>-worktree/<project_name>-<branch_name>-worktree
    let current_dir = env::current_dir()?;
    let parent_dir = current_dir.parent()
        .ok_or_else(|| anyhow::anyhow!("Cannot find parent directory"))?;
    
    let clean_branch = clean_branch_name(branch_name);
    let worktree_parent = parent_dir.join(format!("{}-worktree", project_name));
    let worktree_path = worktree_parent.join(format!("{}-{}-worktree", project_name, clean_branch));
    
    Ok(worktree_path)
}

pub fn ensure_parent_directory(path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    Ok(())
}

pub fn normalize_path(path: &Path) -> PathBuf {
    let mut components = Vec::new();
    
    for component in path.components() {
        match component {
            std::path::Component::ParentDir => {
                if !components.is_empty() {
                    components.pop();
                }
            }
            std::path::Component::CurDir => {
                // 忽略 "."
            }
            _ => {
                components.push(component);
            }
        }
    }
    
    components.iter().collect()
}

pub fn clean_branch_name(branch_name: &str) -> String {
    branch_name
        .replace(' ', "-")
        .replace('/', "-")
        .replace('\\', "-")
        .replace(':', "-")
        .replace('?', "")
        .replace('*', "")
        .replace('<', "")
        .replace('>', "")
        .replace('|', "")
        .replace('"', "")
}