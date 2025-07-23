use anyhow::Result;
use git2::BranchType;
use crate::git::repository::Repository;

#[derive(Debug, Clone, PartialEq)]
pub struct Branch {
    pub name: String,
    pub is_remote: bool,
    pub upstream: Option<String>,
}

pub struct BranchManager<'a> {
    repo: &'a Repository,
}

impl<'a> BranchManager<'a> {
    pub fn new(repo: &'a Repository) -> Self {
        Self { repo }
    }

    pub fn list_local_branches(&self) -> Result<Vec<Branch>> {
        let mut branches = Vec::new();
        let branch_iter = self.repo.inner.branches(Some(BranchType::Local))?;
        
        for branch_result in branch_iter {
            let (branch, _branch_type) = branch_result?;
            if let Some(name) = branch.name()? {
                // 跳過當前分支的 HEAD 指向
                if name == "HEAD" {
                    continue;
                }
                
                let upstream = branch.upstream()
                    .ok()
                    .and_then(|upstream_branch| {
                        upstream_branch.name().ok().flatten().map(|s| s.to_string())
                    });

                branches.push(Branch {
                    name: name.to_string(),
                    is_remote: false,
                    upstream,
                });
            }
        }
        
        Ok(branches)
    }

    pub fn list_remote_branches(&self) -> Result<Vec<Branch>> {
        let mut branches = Vec::new();
        let branch_iter = self.repo.inner.branches(Some(BranchType::Remote))?;
        
        for branch_result in branch_iter {
            let (branch, _branch_type) = branch_result?;
            if let Some(name) = branch.name()? {
                // 跳過 HEAD 指向
                if name.ends_with("/HEAD") {
                    continue;
                }
                
                // 移除 origin/ 前綴
                let display_name = if let Some(stripped) = name.strip_prefix("origin/") {
                    stripped.to_string()
                } else {
                    name.to_string()
                };

                branches.push(Branch {
                    name: display_name,
                    is_remote: true,
                    upstream: None,
                });
            }
        }
        
        Ok(branches)
    }

    pub fn list_all_branches(&self) -> Result<Vec<Branch>> {
        let mut all_branches = self.list_local_branches()?;
        let remote_branches = self.list_remote_branches()?;
        
        // 添加遠程分支，但跳過已經存在的本地分支
        for remote_branch in remote_branches {
            if !all_branches.iter().any(|local| local.name == remote_branch.name) {
                all_branches.push(remote_branch);
            }
        }
        
        Ok(all_branches)
    }
}