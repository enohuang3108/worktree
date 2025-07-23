use clap::Args;
use anyhow::Result;
use crate::git::repository::Repository;
use crate::git::branches::BranchManager;
use crate::git::worktree::{WorktreeManager, BranchMode};
use crate::ui::prompts::Prompts;
use crate::ui::display::Display;
use crate::utils::paths::generate_worktree_path;
use crate::utils::system::open_vscode;

#[derive(Debug, Args)]
pub struct AddCommand {}

impl AddCommand {
    pub fn execute(&self) -> Result<()> {
        Display::show_info("Creating a new worktree...");
        
        let repo = Repository::open_current()?;
        let project_name = repo.get_project_name()?;
        let branch_manager = BranchManager::new(&repo);
        
        // 選擇分支模式
        let branch_mode = Prompts::select_branch_mode()?;
        
        let (branch_name, _base_branch) = match branch_mode {
            BranchMode::NewBranch => {
                let branch_name = Prompts::input_branch_name()?;
                let branches = branch_manager.list_local_branches()?;
                let base_branch = Prompts::select_base_branch(branches)?;
                (branch_name, Some(base_branch))
            }
            BranchMode::ExistingBranch => {
                let branches = branch_manager.list_all_branches()?;
                let selected_branch = Prompts::select_existing_branch(branches)?;
                (selected_branch.name, None)
            }
        };
        
        // 生成 worktree 路徑
        let worktree_path = generate_worktree_path(&project_name, &branch_name)?;
        
        // 創建 worktree
        let worktree_manager = WorktreeManager::new(repo.inner.path().parent().unwrap().to_path_buf());
        worktree_manager.add_worktree(&worktree_path, &branch_name)?;
        
        Display::show_success(&format!("Worktree created at: {}", worktree_path.display()));
        
        // 詢問是否在 VSCode 中開啟
        if Prompts::confirm_vscode_open(&worktree_path.to_string_lossy())? {
            open_vscode(&worktree_path)?;
            Display::show_success("VSCode opened successfully!");
        }
        
        Ok(())
    }
}