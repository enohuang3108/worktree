use anyhow::Result;
use crate::error::{WtError, WtResult};

pub fn validate_branch_name(name: &str) -> WtResult<()> {
    if name.trim().is_empty() {
        return Err(WtError::ValidationError("Branch name cannot be empty".to_string()));
    }

    if name.contains(' ') {
        return Err(WtError::ValidationError("Branch name cannot contain spaces".to_string()));
    }

    todo!("完成分支名稱驗證")
}

pub fn validate_path(path: &str) -> WtResult<()> {
    todo!("實作路徑驗證")
}