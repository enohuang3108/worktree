use clap::Args;
use anyhow::Result;
use crate::git::repository::Repository;
use crate::git::worktree::WorktreeManager;
use crate::ui::prompts::Prompts;
use crate::ui::display::Display;

#[derive(Debug, Args)]
pub struct RemoveCommand {}

impl RemoveCommand {
    pub fn execute(&self) -> Result<()> {
        Display::show_info("Removing a worktree...");
        
        let repo = Repository::open_current()?;
        let worktree_manager = WorktreeManager::new(repo.inner.path().parent().unwrap().to_path_buf());
        
        // 獲取 worktree 列表
        let worktrees = worktree_manager.list_worktrees()?;
        
        if worktrees.is_empty() {
            Display::show_info("No worktrees found to remove.");
            return Ok(());
        }
        
        // 選擇要移除的 worktree
        let selected_worktree = Prompts::select_worktree(worktrees)?;
        
        // 確認移除
        if Prompts::confirm_removal(&selected_worktree)? {
            Display::show_info("Removing worktree...");
            worktree_manager.remove_worktree(&selected_worktree.path)?;
            Display::show_success(&format!("Worktree '{}' removed successfully!", selected_worktree.branch));
        } else {
            Display::show_info("Operation cancelled.");
        }
        
        Ok(())
    }
}