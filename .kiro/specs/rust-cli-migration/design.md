# 設計文件

## 專案概覽

本設計文件詳細說明如何將現有的 TypeScript Git worktree CLI 工具完全遷移至 Rust，提供高效能、跨平台的單一可執行檔案，保持 100% 的功能相容性。

## 技術架構

### 核心技術棧

```toml
[dependencies]
clap = { version = "4.0", features = ["derive"] }       # CLI 框架
inquire = "0.6"                                         # 交互式介面
owo-colors = { version = "3.5", features = ["supports-colors"] }  # 彩色輸出
anyhow = "1.0"                                          # 錯誤處理
git2 = "0.18"                                           # Git 操作
serde = { version = "1.0", features = ["derive"] }     # 序列化
```

**技術選擇理由**:
- **clap**: 現代、功能完整的 CLI 框架，替代 commander.js
- **inquire**: 與 inquirer.js 功能對等的交互式介面庫
- **owo-colors**: 零分配、智慧終端偵測的彩色輸出，替代 chalk
- **anyhow**: CLI 應用最佳的錯誤處理解決方案
- **git2**: 型別安全的 Git 操作，部分替代 subprocess 調用

## 模組架構設計

### 專案結構
```
src/
├── main.rs              # 程式入口點
├── cli.rs               # CLI 命令定義和路由
├── commands/            # 命令實作模組
│   ├── mod.rs
│   ├── add.rs          # wt add 命令
│   ├── remove.rs       # wt remove 命令
│   └── open.rs         # wt open 命令
├── git/                 # Git 操作模組
│   ├── mod.rs
│   ├── repository.rs   # 倉庫檢測和基本操作
│   ├── branches.rs     # 分支管理
│   └── worktree.rs     # Worktree 管理
├── ui/                  # 使用者介面模組
│   ├── mod.rs
│   ├── prompts.rs      # 交互式提示
│   ├── display.rs      # 顯示格式化
│   └── colors.rs       # 彩色輸出配置
├── utils/               # 工具函數模組
│   ├── mod.rs
│   ├── paths.rs        # 路徑操作
│   ├── validation.rs   # 輸入驗證
│   └── system.rs       # 系統整合
└── error.rs            # 錯誤定義
```

### 模組職責分離

#### 1. CLI 模組 (`cli.rs`)
```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(name = "wt")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new worktree
    Add,
    /// Remove an existing worktree
    #[command(alias = "rm", alias = "delete")]
    Remove,
    /// Open a worktree in VSCode
    Open,
}
```

#### 2. Git 模組架構
```rust
// git/repository.rs
pub struct Repository {
    repo: git2::Repository,
    project_name: String,
}

impl Repository {
    pub fn open_current() -> Result<Self>;
    pub fn get_project_name(&self) -> &str;
    pub fn is_git_repository() -> bool;
}

// git/branches.rs
pub struct BranchManager<'repo> {
    repo: &'repo git2::Repository,
}

impl<'repo> BranchManager<'repo> {
    pub fn list_local_branches(&self) -> Result<Vec<String>>;
    pub fn list_remote_branches(&self) -> Result<Vec<String>>;
    pub fn list_all_branches(&self) -> Result<Vec<Branch>>;
}

// git/worktree.rs
pub struct WorktreeManager<'repo> {
    repo: &'repo git2::Repository,
}

impl<'repo> WorktreeManager<'repo> {
    pub fn list_worktrees(&self) -> Result<Vec<WorktreeInfo>>;
    pub fn add_worktree(&self, path: &Path, branch: &str) -> Result<()>;
    pub fn remove_worktree(&self, path: &Path) -> Result<()>;
}
```

#### 3. UI 模組架構
```rust
// ui/prompts.rs
pub struct Prompts;

impl Prompts {
    pub fn select_branch_mode() -> Result<BranchMode>;
    pub fn input_branch_name() -> Result<String>;
    pub fn select_base_branch(branches: &[String]) -> Result<String>;
    pub fn select_existing_branch(branches: &[String]) -> Result<String>;
    pub fn select_worktree(worktrees: &[WorktreeInfo]) -> Result<WorktreeInfo>;
    pub fn confirm_removal(worktree: &WorktreeInfo) -> Result<bool>;
    pub fn confirm_vscode_open() -> Result<bool>;
}

// ui/display.rs
pub struct Display;

impl Display {
    pub fn show_project_info(project_name: &str);
    pub fn show_success(message: &str);
    pub fn show_error(message: &str);
    pub fn show_info(message: &str);
}
```

## API 介面設計

### 核心資料結構

```rust
#[derive(Debug, Clone)]
pub struct WorktreeInfo {
    pub path: PathBuf,
    pub branch: String,
    pub commit: String,
    pub is_current: bool,
}

#[derive(Debug)]
pub enum BranchMode {
    NewBranch,
    ExistingBranch,
}

#[derive(Debug, Clone)]
pub struct Branch {
    pub name: String,
    pub is_remote: bool,
    pub upstream: Option<String>,
}
```

### 命令介面設計

#### 1. Add 命令流程
```rust
// commands/add.rs
pub struct AddCommand<'repo> {
    repo: &'repo Repository,
    worktree_manager: WorktreeManager<'repo>,
    branch_manager: BranchManager<'repo>,
}

impl<'repo> AddCommand<'repo> {
    pub fn execute(&self) -> Result<()> {
        // 1. 顯示專案資訊
        Display::show_project_info(self.repo.get_project_name());

        // 2. 選擇分支模式
        let mode = Prompts::select_branch_mode()?;

        // 3. 根據模式處理分支
        let (branch_name, base_branch) = match mode {
            BranchMode::NewBranch => self.handle_new_branch()?,
            BranchMode::ExistingBranch => self.handle_existing_branch()?,
        };

        // 4. 生成 worktree 路徑
        let worktree_path = self.generate_worktree_path(&branch_name)?;

        // 5. 創建 worktree
        self.worktree_manager.add_worktree(&worktree_path, &branch_name)?;

        // 6. 詢問是否在 VSCode 中開啟
        if Prompts::confirm_vscode_open()? {
            SystemIntegration::open_in_vscode(&worktree_path)?;
        }

        Display::show_success(&format!("Worktree created at: {}", worktree_path.display()));
        Ok(())
    }
}
```

#### 2. Remove 命令流程
```rust
// commands/remove.rs
pub struct RemoveCommand<'repo> {
    repo: &'repo Repository,
    worktree_manager: WorktreeManager<'repo>,
}

impl<'repo> RemoveCommand<'repo> {
    pub fn execute(&self) -> Result<()> {
        // 1. 顯示專案資訊
        Display::show_project_info(self.repo.get_project_name());

        // 2. 獲取 worktree 列表
        let worktrees = self.worktree_manager.list_worktrees()?;
        if worktrees.is_empty() {
            Display::show_info("No worktrees found.");
            return Ok(());
        }

        // 3. 使用者選擇要移除的 worktree
        let selected = Prompts::select_worktree(&worktrees)?;

        // 4. 確認刪除
        if !Prompts::confirm_removal(&selected)? {
            Display::show_info("Operation cancelled.");
            return Ok(());
        }

        // 5. 執行移除
        self.worktree_manager.remove_worktree(&selected.path)?;
        Display::show_success(&format!("Worktree removed: {}", selected.path.display()));
        Ok(())
    }
}
```

#### 3. Open 命令流程
```rust
// commands/open.rs
pub struct OpenCommand<'repo> {
    repo: &'repo Repository,
    worktree_manager: WorktreeManager<'repo>,
}

impl<'repo> OpenCommand<'repo> {
    pub fn execute(&self) -> Result<()> {
        // 1. 顯示專案資訊
        Display::show_project_info(self.repo.get_project_name());

        // 2. 獲取 worktree 列表
        let worktrees = self.worktree_manager.list_worktrees()?;
        if worktrees.is_empty() {
            Display::show_info("No worktrees found.");
            return Ok(());
        }

        // 3. 使用者選擇 worktree
        let selected = Prompts::select_worktree(&worktrees)?;

        // 4. 在 VSCode 中開啟
        SystemIntegration::open_in_vscode(&selected.path)?;
        Display::show_success(&format!("Opened worktree: {}", selected.path.display()));
        Ok(())
    }
}
```

## 建置和部署設計

### Cargo 配置

#### Cargo.toml
```toml
[package]
name = "wt"
version = "1.0.0"
edition = "2021"
description = "Git Worktree management CLI tool"
authors = ["Your Name <your.email@example.com>"]
license = "MIT"
repository = "https://github.com/your-username/wt"
readme = "README.md"
keywords = ["cli", "git", "worktree", "development"]
categories = ["command-line-utilities", "development-tools"]

[[bin]]
name = "wt"
path = "src/main.rs"

[dependencies]
clap = { version = "4.0", features = ["derive"] }
inquire = "0.6"
owo-colors = { version = "3.5", features = ["supports-colors"] }
anyhow = "1.0"
git2 = "0.18"
serde = { version = "1.0", features = ["derive"] }

[profile.release]
lto = true                # 鏈接時優化
codegen-units = 1        # 單個程式碼生成單元
panic = "abort"          # 減少二進制大小
strip = true             # 移除符號資訊
opt-level = "z"          # 優化二進制大小

[profile.dev]
debug = true
opt-level = 0
```

### 跨平台編譯配置

#### 支援的目標平台
```bash
# Tier 1 - 主要支援
x86_64-unknown-linux-gnu      # Linux x64
x86_64-pc-windows-msvc        # Windows x64
x86_64-apple-darwin           # macOS Intel

# Tier 2 - 次要支援
aarch64-apple-darwin          # macOS Apple Silicon
x86_64-unknown-linux-musl     # Linux x64 靜態連結
aarch64-unknown-linux-musl    # Linux ARM64 靜態連結
```

#### 建置腳本 (build.sh)
```bash
#!/bin/bash

# 設定 Rust 優化環境變數
export RUSTFLAGS="-C target-feature=+crt-static -C link-self-contained=yes"

# 建置所有目標平台
targets=(
    "x86_64-unknown-linux-gnu"
    "x86_64-pc-windows-msvc"
    "x86_64-apple-darwin"
    "aarch64-apple-darwin"
    "x86_64-unknown-linux-musl"
    "aarch64-unknown-linux-musl"
)

for target in "${targets[@]}"; do
    echo "Building for $target..."
    cross build --target $target --release

    # 建立輸出目錄
    mkdir -p "dist/$target"

    # 複製二進制檔案
    if [[ $target == *"windows"* ]]; then
        cp "target/$target/release/wt.exe" "dist/$target/"
    else
        cp "target/$target/release/wt" "dist/$target/"
    fi
done
```

### GitHub Actions 自動化部署

#### .github/workflows/release.yml
```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  build:
    name: Build for ${{ matrix.target }}
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        include:
          - target: x86_64-unknown-linux-gnu
            os: ubuntu-latest
          - target: x86_64-pc-windows-msvc
            os: windows-latest
          - target: x86_64-apple-darwin
            os: macos-latest
          - target: aarch64-apple-darwin
            os: macos-latest
          - target: x86_64-unknown-linux-musl
            os: ubuntu-latest
          - target: aarch64-unknown-linux-musl
            os: ubuntu-latest

    steps:
    - uses: actions/checkout@v3

    - name: Install Rust
      uses: dtolnay/rust-toolchain@stable
      with:
        targets: ${{ matrix.target }}

    - name: Install cross
      run: cargo install cross

    - name: Build
      run: cross build --target ${{ matrix.target }} --release

    - name: Package
      shell: bash
      run: |
        if [[ "${{ matrix.target }}" == *"windows"* ]]; then
          EXE_SUFFIX=".exe"
        else
          EXE_SUFFIX=""
        fi

        BINARY_NAME="wt${EXE_SUFFIX}"
        ARCHIVE_NAME="wt-${{ github.ref_name }}-${{ matrix.target }}"

        mv "target/${{ matrix.target }}/release/${BINARY_NAME}" "${BINARY_NAME}"

        if [[ "${{ runner.os }}" == "Windows" ]]; then
          7z a "${ARCHIVE_NAME}.zip" "${BINARY_NAME}"
          echo "ASSET=${ARCHIVE_NAME}.zip" >> $GITHUB_ENV
        else
          tar czf "${ARCHIVE_NAME}.tar.gz" "${BINARY_NAME}"
          echo "ASSET=${ARCHIVE_NAME}.tar.gz" >> $GITHUB_ENV
        fi

    - name: Upload Release Asset
      uses: softprops/action-gh-release@v1
      with:
        files: ${{ env.ASSET }}
```

### 安裝腳本設計

#### install.sh (Linux/macOS)
```bash
#!/bin/bash

set -e

# 偵測系統架構
case "$(uname -s)" in
    Darwin) OS="apple-darwin" ;;
    Linux) OS="unknown-linux-musl" ;;
    *) echo "Unsupported OS"; exit 1 ;;
esac

case "$(uname -m)" in
    x86_64) ARCH="x86_64" ;;
    arm64|aarch64) ARCH="aarch64" ;;
    *) echo "Unsupported architecture"; exit 1 ;;
esac

TARGET="${ARCH}-${OS}"
BINARY_NAME="wt"
INSTALL_DIR="$HOME/.local/bin"

# 創建安裝目錄
mkdir -p "$INSTALL_DIR"

# 下載最新版本
LATEST_VERSION=$(curl -s https://api.github.com/repos/your-username/wt/releases/latest | grep '"tag_name"' | cut -d'"' -f4)
DOWNLOAD_URL="https://github.com/your-username/wt/releases/download/${LATEST_VERSION}/wt-${LATEST_VERSION}-${TARGET}.tar.gz"

echo "Downloading wt ${LATEST_VERSION} for ${TARGET}..."
curl -L "$DOWNLOAD_URL" | tar xz -C "$INSTALL_DIR"

# 設定執行權限
chmod +x "$INSTALL_DIR/$BINARY_NAME"

# 檢查 PATH
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo "Adding $INSTALL_DIR to PATH..."

    # 偵測 shell 並添加到相應的配置檔案
    if [[ -n "$ZSH_VERSION" ]]; then
        echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc
        echo "Please restart your terminal or run: source ~/.zshrc"
    elif [[ -n "$BASH_VERSION" ]]; then
        echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
        echo "Please restart your terminal or run: source ~/.bashrc"
    fi
fi

echo "wt installed successfully!"
echo "Run 'wt --help' to get started."
```

## 錯誤處理策略

### 錯誤類型定義
```rust
// error.rs
use anyhow::{Context, Result};
use std::fmt;

#[derive(Debug)]
pub enum WtError {
    GitError(String),
    IoError(String),
    UserCancelled,
    InvalidInput(String),
    SystemError(String),
}

impl fmt::Display for WtError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WtError::GitError(msg) => write!(f, "Git error: {}", msg),
            WtError::IoError(msg) => write!(f, "IO error: {}", msg),
            WtError::UserCancelled => write!(f, "Operation cancelled by user"),
            WtError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            WtError::SystemError(msg) => write!(f, "System error: {}", msg),
        }
    }
}

impl std::error::Error for WtError {}

// 使用者友善錯誤處理
pub fn handle_error(err: anyhow::Error) {
    use owo_colors::OwoColorize;

    eprintln!("{}: {}", "Error".red().bold(), err);

    // 顯示錯誤鏈
    for cause in err.chain().skip(1) {
        eprintln!("  {}: {}", "Caused by".yellow(), cause);
    }

    // 提供建議或幫助資訊
    if let Some(wt_error) = err.downcast_ref::<WtError>() {
        match wt_error {
            WtError::GitError(_) => {
                eprintln!("  {}: Make sure you're in a Git repository", "Hint".blue());
            }
            WtError::InvalidInput(_) => {
                eprintln!("  {}: Please check your input and try again", "Hint".blue());
            }
            _ => {}
        }
    }

    std::process::exit(1);
}
```

### 主程式錯誤處理
```rust
// main.rs
use anyhow::Result;

fn main() {
    if let Err(err) = run() {
        crate::error::handle_error(err);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    // 檢查是否在 Git 倉庫中
    let repo = Repository::open_current()
        .context("Failed to open Git repository. Make sure you're in a Git project directory.")?;

    match cli.command {
        Commands::Add => commands::add::execute(&repo),
        Commands::Remove => commands::remove::execute(&repo),
        Commands::Open => commands::open::execute(&repo),
    }
}
```

## 效能優化策略

### 編譯時優化
1. **LTO (Link Time Optimization)**: 啟用全程式最佳化
2. **單程式碼生成單元**: 減少二進制大小
3. **Strip 符號**: 移除除錯符號
4. **最小化優化等級**: 使用 `opt-level = "z"`

### 執行時效能
1. **Git2 整合**: 減少 subprocess 調用開銷
2. **快取分支資訊**: 避免重複的 Git 查詢
3. **延遲載入**: 僅在需要時載入大型資料結構
4. **並行處理**: 使用 rayon 進行並行操作（如需要）

### 記憶體管理
1. **字串切片優先**: 使用 `&str` 而非 `String`
2. **借用檢查器**: Rust 的零成本抽象
3. **栈分配**: 避免不必要的堆分配
4. **生命週期管理**: 明確的記憶體生命週期

## 測試策略

### 單元測試架構
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_worktree_path_generation() {
        let temp_dir = TempDir::new().unwrap();
        // 測試邏輯
    }

    #[test]
    fn test_branch_selection() {
        // 測試分支選擇邏輯
    }
}
```

### 整合測試
```rust
// tests/integration_test.rs
use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("wt").unwrap();
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Git Worktree management"));
}
```

### 沙盒安裝測試

#### 隔離式安裝測試環境
為確保安裝腳本不會影響開發人員的本地環境，設計沙盒測試方案：

```bash
# tests/sandbox_install_test.sh
#!/bin/bash

# 創建隔離的測試環境
TEST_HOME=$(mktemp -d)
export HOME="$TEST_HOME"
export PATH="$TEST_HOME/.local/bin:$PATH"

echo "🧪 Testing installation in sandbox: $TEST_HOME"

# 執行安裝腳本
curl -fsSL https://github.com/enohuang3108/worktree/releases/latest/download/install.sh | bash

# 驗證安裝結果
if [ -f "$TEST_HOME/.local/bin/wt" ]; then
    echo "✅ Binary installed successfully"

    # 測試基本功能
    echo "🔧 Testing basic functionality..."

    # 創建測試 Git 倉庫
    TEST_REPO="$TEST_HOME/test-repo"
    mkdir -p "$TEST_REPO"
    cd "$TEST_REPO"

    git init
    git config user.name "Test User"
    git config user.email "test@example.com"
    echo "# Test Repo" > README.md
    git add README.md
    git commit -m "Initial commit"

    # 測試 wt 命令
    echo "📋 Testing wt --help..."
    "$TEST_HOME/.local/bin/wt" --help

    echo "📋 Testing wt add --help..."
    "$TEST_HOME/.local/bin/wt" add --help

    echo "📋 Testing wt remove --help..."
    "$TEST_HOME/.local/bin/wt" remove --help

    echo "📋 Testing wt open --help..."
    "$TEST_HOME/.local/bin/wt" open --help

    echo "✅ All tests passed!"
else
    echo "❌ Installation failed"
    exit 1
fi

# 清理測試環境
rm -rf "$TEST_HOME"
echo "🧹 Sandbox cleaned up"
```

#### CI/CD 整合測試
```yaml
# .github/workflows/install-test.yml
name: Installation Test

on:
  release:
    types: [published]
  workflow_dispatch:

jobs:
  test-install:
    name: Test Installation on ${{ matrix.os }}
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest]

    steps:
    - name: Checkout
      uses: actions/checkout@v3

    - name: Run Sandbox Installation Test
      run: |
        # 創建隔離測試環境
        TEST_HOME=$(mktemp -d)
        export HOME="$TEST_HOME"
        export PATH="$TEST_HOME/.local/bin:$PATH"

        echo "Testing installation in: $TEST_HOME"

        # 執行安裝
        curl -fsSL https://github.com/enohuang3108/worktree/releases/latest/download/install.sh | bash

        # 驗證安裝
        if [ -f "$TEST_HOME/.local/bin/wt" ]; then
          echo "✅ Installation successful"

          # 測試基本命令
          "$TEST_HOME/.local/bin/wt" --version
          "$TEST_HOME/.local/bin/wt" --help

          # 創建測試倉庫並測試功能
          mkdir -p "$TEST_HOME/test-repo"
          cd "$TEST_HOME/test-repo"
          git init
          git config user.name "CI Test"
          git config user.email "ci@test.com"
          echo "test" > test.txt
          git add test.txt
          git commit -m "test commit"

          # 測試 wt 命令（非互動模式）
          "$TEST_HOME/.local/bin/wt" --help

          echo "✅ All tests passed"
        else
          echo "❌ Installation failed"
          exit 1
        fi

        # 清理
        rm -rf "$TEST_HOME"

  test-install-docker:
    name: Test Installation in Docker
    runs-on: ubuntu-latest

    steps:
    - name: Checkout
      uses: actions/checkout@v3

    - name: Build Test Container
      run: |
        cat > Dockerfile.test << 'EOF'
        FROM ubuntu:22.04

        RUN apt-get update && apt-get install -y \
            curl git bash && \
            rm -rf /var/lib/apt/lists/*

        RUN useradd -m testuser
        USER testuser
        WORKDIR /home/testuser

        RUN curl -fsSL https://github.com/enohuang3108/worktree/releases/latest/download/install.sh | bash

        # 測試安裝結果
        RUN test -f /home/testuser/.local/bin/wt
        RUN /home/testuser/.local/bin/wt --version
        RUN /home/testuser/.local/bin/wt --help

        # 測試基本 Git 操作
        RUN git init test-repo && cd test-repo && \
            git config user.name "Test" && \
            git config user.email "test@test.com" && \
            echo "test" > README.md && \
            git add README.md && \
            git commit -m "test"
        EOF

        docker build -f Dockerfile.test -t wt-install-test .
        echo "✅ Docker installation test passed"
```

#### 功能驗證測試腳本
```bash
# tests/functional_test.sh
#!/bin/bash

set -e

WT_BINARY="$1"
if [ -z "$WT_BINARY" ]; then
    echo "Usage: $0 <path-to-wt-binary>"
    exit 1
fi

echo "🧪 Running functional tests for: $WT_BINARY"

# 創建測試環境
TEST_DIR=$(mktemp -d)
cd "$TEST_DIR"

echo "📁 Test directory: $TEST_DIR"

# 初始化 Git 倉庫
git init
git config user.name "Test User"
git config user.email "test@example.com"

# 創建初始提交
echo "# Test Project" > README.md
git add README.md
git commit -m "Initial commit"

# 創建一些分支
git checkout -b feature-a
echo "Feature A" >> README.md
git add README.md
git commit -m "Add feature A"

git checkout main
git checkout -b feature-b
echo "Feature B" >> README.md
git add README.md
git commit -m "Add feature B"

git checkout main

echo "✅ Test repository prepared"

# 測試命令存在性和基本功能
echo "🔍 Testing command availability..."
"$WT_BINARY" --version
"$WT_BINARY" --help

echo "🔍 Testing subcommands help..."
"$WT_BINARY" add --help
"$WT_BINARY" remove --help
"$WT_BINARY" open --help

# 測試專案名稱檢測
echo "🔍 Testing project name detection..."
# 這裡需要模擬非互動式測試，實際實作時可能需要特殊的測試模式

echo "✅ All functional tests passed!"

# 清理
cd /
rm -rf "$TEST_DIR"
echo "🧹 Test environment cleaned up"
```

#### 自動化測試整合
```yaml
# 在主要的 release workflow 中整合
- name: Run Installation and Functional Tests
  run: |
    # 等待 releases 檔案可用
    sleep 30

    # 執行沙盒安裝測試
    bash tests/sandbox_install_test.sh

    # 執行功能測試
    bash tests/functional_test.sh ~/.local/bin/wt
```

#### 優化後的 CI/CD 整合測試
是的，CI/CD 整合測試可以直接使用 `tests/sandbox_install_test.sh` 檔案，這樣更簡潔且避免重複代碼：

```yaml
# .github/workflows/install-test.yml (優化版本)
name: Installation Test

on:
  release:
    types: [published]
  workflow_dispatch:

jobs:
  test-install:
    name: Test Installation on ${{ matrix.os }}
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest]
    
    steps:
    - name: Checkout
      uses: actions/checkout@v3
    
    - name: Run Sandbox Installation Test
      run: |
        # 直接執行沙盒測試腳本，避免重複代碼
        chmod +x tests/sandbox_install_test.sh
        bash tests/sandbox_install_test.sh
    
    - name: Run Additional Functional Tests (if needed)
      run: |
        # 如果需要額外的功能測試
        if [ -f tests/functional_test.sh ]; then
          chmod +x tests/functional_test.sh
          bash tests/functional_test.sh ~/.local/bin/wt
        fi
```

#### 使用統一腳本的優勢
1. **代碼復用**: 本地開發和 CI/CD 使用相同的測試邏輯
2. **維護簡化**: 只需要維護一份測試腳本
3. **一致性保證**: 確保本地和 CI 環境的測試行為一致
4. **除錯容易**: 本地可以直接執行相同的測試腳本進行除錯
5. **版本控制**: 測試邏輯的變更可以通過 Git 追蹤

#### 測試文檔
```markdown
# 沙盒測試說明

## 目的
確保安裝腳本和發布的二進制檔案在乾淨環境中正常工作，不影響開發人員的本地環境。

## 測試範圍
1. **安裝腳本測試**: 驗證 `install.sh` 正確下載和安裝二進制檔案
2. **基本功能測試**: 驗證 CLI 命令正常運作
3. **Git 整合測試**: 驗證 Git 倉庫檢測和基本操作
4. **跨平台測試**: 在 Linux、macOS 環境中測試

## 運行方式

### 本地測試
```bash
# 執行沙盒測試
bash tests/sandbox_install_test.sh

# 執行功能測試（需要已安裝的 wt）
bash tests/functional_test.sh /path/to/wt
```


### CI/CD 自動測試
每次發布時自動觸發，確保發布品質。

## 注意事項
- 所有測試都在隔離環境中進行
- 測試完成後自動清理臨時檔案
- 不會影響開發環境的配置
```

## 相容性保證

### CLI 介面相容性
- **命令名稱**: 完全相同 (`wt add`, `wt remove`, `wt open`)
- **互動流程**: 相同的提示和選項
- **輸出格式**: 保持相同的成功/錯誤訊息格式
- **路徑結構**: 維持相同的 worktree 命名規則

### 行為一致性檢查清單
- [ ] 專案名稱提取邏輯相同
- [ ] Worktree 路徑生成規則相同
- [ ] 分支列表顯示格式相同
- [ ] 錯誤處理行為相同
- [ ] VSCode 整合機制相同

---
**狀態**: 設計已完成，等待審核
**下一步**: 審核設計並更新 spec.json 為已生成狀態
