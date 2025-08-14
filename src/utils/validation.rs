use crate::error::{WtError, WtResult};

pub fn validate_branch_name(name: &str) -> WtResult<()> {
    if name.trim().is_empty() {
        return Err(WtError::ValidationError("Branch name cannot be empty".to_string()));
    }

    // Check for leading/trailing whitespace
    if name != name.trim() {
        return Err(WtError::ValidationError("Branch name cannot have leading or trailing whitespace".to_string()));
    }

    if name.contains(' ') {
        return Err(WtError::ValidationError("Branch name cannot contain spaces".to_string()));
    }

    let trimmed = name;

    // Git branch naming rules
    if trimmed.starts_with('/') || trimmed.ends_with('/') {
        return Err(WtError::ValidationError("Branch name cannot start or end with '/'".to_string()));
    }

    if trimmed.contains("//") {
        return Err(WtError::ValidationError("Branch name cannot contain consecutive slashes".to_string()));
    }

    if trimmed.starts_with('.') || trimmed.ends_with('.') {
        return Err(WtError::ValidationError("Branch name cannot start or end with '.'".to_string()));
    }

    if trimmed.ends_with(".lock") {
        return Err(WtError::ValidationError("Branch name cannot end with '.lock'".to_string()));
    }

    // Check for invalid characters
    let invalid_chars = ['~', '^', ':', '?', '*', '[', '\\', ' ', '\t', '\n'];
    for &ch in &invalid_chars {
        if trimmed.contains(ch) {
            return Err(WtError::ValidationError(format!("Branch name cannot contain '{}'", ch)));
        }
    }

    // Check for ASCII control characters
    if trimmed.chars().any(|c| c.is_control()) {
        return Err(WtError::ValidationError("Branch name cannot contain control characters".to_string()));
    }

    // Check against reserved names
    if trimmed == "HEAD" {
        return Err(WtError::ValidationError("'HEAD' is a reserved name".to_string()));
    }

    if trimmed.starts_with("refs/") {
        return Err(WtError::ValidationError("Branch name cannot start with 'refs/'".to_string()));
    }

    Ok(())
}

pub fn validate_path(path: &str) -> WtResult<()> {
    if path.trim().is_empty() {
        return Err(WtError::ValidationError("Path cannot be empty".to_string()));
    }

    // Check for leading/trailing whitespace
    if path != path.trim() {
        return Err(WtError::ValidationError("Path cannot have leading or trailing whitespace".to_string()));
    }

    // Check for null bytes (invalid in filesystem paths)
    if path.contains('\0') {
        return Err(WtError::ValidationError("Path cannot contain null bytes".to_string()));
    }

    // Check for invalid characters on Windows (also good practice on Unix)
    let invalid_chars = ['<', '>', ':', '"', '|', '?', '*'];
    for &ch in &invalid_chars {
        if path.contains(ch) {
            return Err(WtError::ValidationError(format!("Path cannot contain '{}'", ch)));
        }
    }

    // Check for control characters
    if path.chars().any(|c| c.is_control() && c != '\t') {
        return Err(WtError::ValidationError("Path cannot contain control characters".to_string()));
    }

    // Check for reserved names on Windows
    let reserved_names = ["CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4", 
                          "COM5", "COM6", "COM7", "COM8", "COM9", "LPT1", "LPT2", 
                          "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9"];
    
    for component in path.split('/').chain(path.split('\\')) {
        let component_upper = component.to_uppercase();
        if reserved_names.contains(&component_upper.as_str()) {
            return Err(WtError::ValidationError(format!("'{}' is a reserved name", component)));
        }
        
        // Check for names ending with dot (invalid on Windows)
        if component.ends_with('.') && !component.is_empty() {
            return Err(WtError::ValidationError("Path components cannot end with '.'".to_string()));
        }
    }

    // Check path length (typical filesystem limits)
    if path.len() > 4096 {
        return Err(WtError::ValidationError("Path is too long (maximum 4096 characters)".to_string()));
    }

    // Check for potentially dangerous patterns
    if path.contains("..") {
        return Err(WtError::ValidationError("Path cannot contain '..' (parent directory references)".to_string()));
    }

    Ok(())
}