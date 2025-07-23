use anyhow::Result;
use std::fmt;

#[derive(Debug)]
pub enum WtError {
    NotGitRepository,
    GitError(String),
    IOError(String),
    ValidationError(String),
    UserCancelled,
}

impl fmt::Display for WtError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WtError::NotGitRepository => write!(f, "Not in a Git repository"),
            WtError::GitError(msg) => write!(f, "Git error: {}", msg),
            WtError::IOError(msg) => write!(f, "IO error: {}", msg),
            WtError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            WtError::UserCancelled => write!(f, "Operation cancelled by user"),
        }
    }
}

impl std::error::Error for WtError {}

pub type WtResult<T> = Result<T, WtError>;