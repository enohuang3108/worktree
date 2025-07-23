use anyhow::{Result, anyhow};
use git2::Repository as Git2Repository;
use std::env;
use std::path::{Path, PathBuf};

pub struct Repository {
    pub inner: Git2Repository,
}

impl Repository {
    pub fn open_current() -> Result<Self> {
        let current_dir = env::current_dir()?;
        let repo = Git2Repository::discover(&current_dir)
            .map_err(|_| anyhow!("Not in a Git repository"))?;
        
        Ok(Repository { inner: repo })
    }

    pub fn get_project_name(&self) -> Result<String> {
        // 嘗試從 remote URL 提取專案名稱
        if let Ok(remote) = self.inner.find_remote("origin") {
            if let Some(url) = remote.url() {
                if let Some(name) = extract_project_name_from_url(url) {
                    return Ok(name);
                }
            }
        }

        // 如果沒有 remote，則使用目錄名稱
        let workdir = self.inner.workdir()
            .ok_or_else(|| anyhow!("Repository has no working directory"))?;
        
        let dir_name = workdir.file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| anyhow!("Invalid directory name"))?;
        
        Ok(dir_name.to_string())
    }

    pub fn is_git_repository() -> bool {
        match env::current_dir() {
            Ok(current_dir) => {
                Git2Repository::discover(&current_dir).is_ok()
            }
            Err(_) => false,
        }
    }
}

fn extract_project_name_from_url(url: &str) -> Option<String> {
    // 處理 HTTPS URL: https://github.com/user/repo.git
    if url.starts_with("https://") {
        if let Some(last_part) = url.split('/').last() {
            let name = last_part.trim_end_matches(".git");
            return Some(name.to_string());
        }
    }
    
    // 處理 SSH URL: git@github.com:user/repo.git
    if url.starts_with("git@") {
        if let Some(colon_pos) = url.find(':') {
            let after_colon = &url[colon_pos + 1..];
            if let Some(last_part) = after_colon.split('/').last() {
                let name = last_part.trim_end_matches(".git");
                return Some(name.to_string());
            }
        }
    }
    
    None
}