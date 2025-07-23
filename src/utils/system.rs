use anyhow::{Result, anyhow};
use std::path::Path;
use std::process::Command;

pub fn open_vscode(path: &Path) -> Result<()> {
    if !check_vscode_available() {
        return Err(anyhow!("VSCode is not available. Make sure 'code' command is in your PATH."));
    }

    let output = Command::new("code")
        .arg(path)
        .output()?;

    if !output.status.success() {
        let error_message = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("Failed to open VSCode: {}", error_message));
    }

    Ok(())
}

pub fn check_vscode_available() -> bool {
    Command::new("code")
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}