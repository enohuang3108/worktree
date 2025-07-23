use anyhow::{Result, Context};

/// Open a path in VSCode
pub async fn open_in_vscode<P: AsRef<str>>(path: P) -> Result<()> {
    let path = path.as_ref();
    
    let output = tokio::process::Command::new("code")
        .arg(path)
        .output()
        .await
        .context("Failed to execute code command. Make sure VSCode is installed and 'code' is in your PATH")?;
    
    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Failed to open VSCode: {}", error_msg);
    }
    
    Ok(())
}