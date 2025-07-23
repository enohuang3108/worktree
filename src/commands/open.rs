use clap::Args;
use anyhow::Result;
use crate::git::repository::Repository;
use crate::git::worktree::WorktreeManager;
use crate::ui::prompts::Prompts;
use crate::ui::display::Display;
use crate::utils::system::open_vscode;

#[derive(Debug, Args)]
pub struct OpenCommand {}

impl OpenCommand {
    pub fn execute(&self) -> Result<()> {
        Display::show_info("Opening a worktree in VSCode...");
        
        let repo = Repository::open_current()?;
        let worktree_manager = WorktreeManager::new(repo.inner.path().parent().unwrap().to_path_buf());
        
        // 獲取 worktree 列表
        let worktrees = worktree_manager.list_worktrees()?;
        
        if worktrees.is_empty() {
            Display::show_info("No worktrees found to open.");
            return Ok(());
        }
        
        // 選擇要開啟的 worktree
        let selected_worktree = Prompts::select_worktree(worktrees)?;
        
        Display::show_info("Opening VSCode...");
        open_vscode(&selected_worktree.path)?;
        Display::show_success(&format!("VSCode opened for worktree '{}'!", selected_worktree.branch));
        
        Ok(())
    }
}