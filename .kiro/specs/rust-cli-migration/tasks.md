# 實作計劃

## 進度狀況
- 建立時間: 2025-07-23
- 狀態: 準備開始實作
- 總任務數: 25
- 已完成: 0
- 剩餘: 25

## 實作任務

### 1. 專案設置和核心結構
- [ ] 1.1 初始化 Rust 專案結構
  - 建立 Cargo.toml 配置檔案，定義專案元數據和依賴項目
  - 設置核心依賴：clap, inquire, owo-colors, anyhow, git2, serde
  - 建立 src/ 目錄結構（main.rs, cli.rs, commands/, git/, ui/, utils/, error.rs）
  - 配置編譯優化設定和跨平台目標
  - _需求: 技術需求.核心技術棧, 建置和分發_

- [ ] 1.2 建立基礎模組框架
  - 實作 main.rs 程式入口點和錯誤處理
  - 定義 cli.rs 中的 CLI 命令結構和路由
  - 建立各模組的 mod.rs 檔案和基本模組導出
  - 設置專案級別的 use 語句和模組可見性
  - _需求: 模組架構設計_

### 2. 錯誤處理和核心資料結構
- [ ] 2.1 實作錯誤處理系統
  - 定義 error.rs 中的自定義錯誤類型 WtError
  - 實作使用者友善的錯誤訊息和建議提示
  - 整合 anyhow 錯誤處理和錯誤鏈顯示
  - 建立錯誤處理函數和 main.rs 中的錯誤攔截
  - _需求: 非功能需求.可靠性, 錯誤處理_

- [ ] 2.2 定義核心資料結構
  - 實作 WorktreeInfo 結構體（路徑、分支、提交、當前狀態）
  - 定義 BranchMode 枚舉（新分支、現有分支）
  - 建立 Branch 結構體（名稱、是否遠程、upstream）
  - 實作必要的 trait（Debug, Clone, PartialEq）
  - _需求: API 介面設計.核心資料結構_

### 3. Git 操作模組實作
- [ ] 3.1 倉庫檢測和基本操作
  - 實作 git/repository.rs 中的 Repository 結構體
  - 建立 open_current() 方法檢測當前 Git 倉庫
  - 實作 get_project_name() 從 remote URL 或目錄名提取專案名稱
  - 添加 is_git_repository() 靜態方法驗證 Git 倉庫
  - _需求: 核心功能模組.Git 操作.倉庫檢測, 專案名稱_

- [ ] 3.2 分支管理功能
  - 實作 git/branches.rs 中的 BranchManager 結構體
  - 建立 list_local_branches() 獲取本地分支列表
  - 實作 list_remote_branches() 獲取遠程分支列表
  - 添加 list_all_branches() 合併本地和遠程分支
  - _需求: 核心功能模組.Git 操作.分支管理_

- [ ] 3.3 Worktree 管理核心
  - 實作 git/worktree.rs 中的 WorktreeManager 結構體
  - 建立 list_worktrees() 解析現有 worktree 列表
  - 實作 add_worktree() 創建新的 worktree
  - 添加 remove_worktree() 安全移除 worktree
  - _需求: 核心功能模組.Git 操作.Worktree 管理_

### 4. 使用者介面模組實作
- [ ] 4.1 交互式提示系統
  - 實作 ui/prompts.rs 中的 Prompts 結構體
  - 建立 select_branch_mode() 分支模式選擇提示
  - 實作 input_branch_name() 和 select_base_branch() 新分支處理
  - 添加 select_existing_branch() 和 select_worktree() 選擇提示
  - 實作 confirm_removal() 和 confirm_vscode_open() 確認對話框
  - _需求: 核心功能模組.使用者介面.交互式選單_

- [ ] 4.2 顯示格式化和彩色輸出
  - 實作 ui/display.rs 中的 Display 結構體
  - 建立 show_project_info() 顯示專案資訊
  - 實作 show_success(), show_error(), show_info() 彩色訊息輸出
  - 配置 ui/colors.rs 智慧終端偵測和顏色配置
  - _需求: 核心功能模組.使用者介面.彩色輸出_

### 5. 工具函數和系統整合
- [ ] 5.1 路徑操作和驗證
  - 實作 utils/paths.rs 路徑生成和操作函數
  - 建立 worktree 路徑命名規則：`../<專案名>-worktree/<專案名>-<分支名>-worktree`
  - 實作目錄創建和父目錄檢查功能
  - 添加路徑清理和正規化函數
  - _需求: 核心功能模組.檔案系統操作.路徑生成, 目錄管理_

- [ ] 5.2 輸入驗證和系統整合
  - 實作 utils/validation.rs 輸入驗證函數
  - 建立分支名稱驗證（防止空格、特殊字符）
  - 實作 utils/system.rs 系統整合功能
  - 添加 VSCode 啟動功能（code 命令執行）
  - _需求: 核心功能模組.檔案系統操作.權限檢查, VSCode 整合_

### 6. CLI 命令實作
- [ ] 6.1 Add 命令完整實作
  - 實作 commands/add.rs 中的 AddCommand 結構體
  - 建立完整的 execute() 方法流程
  - 實作 handle_new_branch() 和 handle_existing_branch() 輔助方法
  - 添加 generate_worktree_path() 路徑生成邏輯
  - 整合所有模組：Git 操作、UI 提示、系統整合
  - _需求: 主要 CLI 命令.wt add, 使用者故事 1, 2_

- [ ] 6.2 Remove 命令完整實作
  - 實作 commands/remove.rs 中的 RemoveCommand 結構體
  - 建立 worktree 列表顯示和選擇邏輯
  - 實作安全確認機制和刪除操作
  - 添加空列表處理和錯誤回饋
  - _需求: 主要 CLI 命令.wt remove, 使用者故事 3_

- [ ] 6.3 Open 命令完整實作
  - 實作 commands/open.rs 中的 OpenCommand 結構體
  - 建立 worktree 選擇和 VSCode 啟動邏輯
  - 實作錯誤處理和使用者回饋
  - 添加空列表檢查和操作確認
  - _需求: 主要 CLI 命令.wt open, 使用者故事 4_

### 7. CLI 框架整合和路由
- [ ] 7.1 完善 CLI 命令定義
  - 完善 cli.rs 中的 clap 配置和命令定義
  - 實作命令別名：remove -> rm, delete
  - 添加 --help 和 --version 支援
  - 配置命令描述和使用範例
  - _需求: CLI 介面.命令名稱, 相容性需求_

- [ ] 7.2 主程式流程整合
  - 完善 main.rs 中的程式入口點和錯誤處理
  - 實作 Git 倉庫驗證和早期錯誤檢查
  - 建立命令路由和執行邏輯
  - 添加程式級別的錯誤攔截和使用者友善訊息
  - _需求: 錯誤處理策略, 主程式錯誤處理_

### 8. 建置配置和最佳化
- [ ] 8.1 Cargo 專案配置完善
  - 完善 Cargo.toml 專案元數據和依賴版本
  - 配置 release profile 優化設定（LTO, codegen-units, panic, strip）
  - 設置跨平台編譯目標配置
  - 添加二進制檔案命名和特性配置
  - _需求: 建置和分發.編譯目標, 效能需求_

- [ ] 8.2 跨平台編譯腳本
  - 建立 build.sh 跨平台編譯腳本
  - 配置目標平台：Linux (x64/arm64), macOS (x64/arm64), Windows (x64)
  - 實作編譯最佳化環境變數設定
  - 添加輸出目錄組織和檔案複製邏輯
  - _需求: 建置和分發.跨平台支援_

### 9. 自動化部署和發布
- [ ] 9.1 GitHub Actions 工作流程
  - 建立 .github/workflows/release.yml 發布工作流程
  - 配置跨平台編譯矩陣（Ubuntu, macOS, Windows）
  - 實作 cross 工具安裝和交叉編譯
  - 添加二進制檔案打包和 Release 上傳
  - _需求: 建置和分發.分發機制_

- [ ] 9.2 安裝腳本實作
  - 建立 install.sh 自動安裝腳本
  - 實作系統和架構自動偵測
  - 添加最新版本下載和安裝邏輯
  - 配置 PATH 環境變數和 shell 整合
  - _需求: 建置和分發.安裝方法, 使用者故事 5_

### 10. 測試框架和驗證
- [ ] 10.1 單元測試實作
  - 為核心模組建立單元測試（git/, ui/, utils/）
  - 實作 WorktreeInfo, Repository, BranchManager 測試
  - 添加路徑生成和驗證函數測試
  - 配置 test 目錄結構和測試工具
  - _需求: 非功能需求.可維護性.測試覆蓋_

- [ ] 10.2 整合測試和 CLI 測試
  - 建立 CLI 命令整合測試
  - 實作 --help 和 --version 輸出驗證
  - 添加錯誤場景和邊界條件測試
  - 配置測試用的模擬 Git 倉庫環境
  - _需求: 測試策略.整合測試_

### 11. 沙盒安裝測試
- [ ] 11.1 隔離式安裝測試環境
  - 建立 tests/sandbox_install_test.sh 測試腳本
  - 實作隔離的 HOME 和 PATH 環境變數設定
  - 添加安裝腳本執行和二進制檔案驗證
  - 配置測試 Git 倉庫建立和基本功能測試
  - _需求: 沙盒安裝測試.隔離式安裝測試環境_

- [ ] 11.2 CI/CD 整合測試
  - 建立 .github/workflows/install-test.yml 測試工作流程
  - 配置 Linux 和 macOS 平台的安裝測試
  - 實作 sandbox_install_test.sh 腳本整合
  - 添加功能驗證和自動清理邏輯
  - _需求: 沙盒安裝測試.CI/CD 整合測試_

### 12. 功能驗證和相容性測試
- [ ] 12.1 功能驗證測試腳本
  - 建立 tests/functional_test.sh 功能測試腳本
  - 實作 CLI 命令可用性和基本功能測試
  - 添加專案名稱檢測和 Git 操作測試
  - 配置非互動模式測試和回歸測試
  - _需求: 沙盒安裝測試.功能驗證測試腳本_

- [ ] 12.2 相容性驗證和效能測試
  - 驗證與 TypeScript 版本的 CLI 介面相容性
  - 測試 worktree 路徑生成規則一致性
  - 驗證 Git 命令和參數使用一致性
  - 測試效能需求：啟動時間 <100ms, 記憶體 <10MB, 大小 <5MB
  - _需求: 相容性需求, 效能需求_

### 13. 文檔和最終整合
- [ ] 13.1 使用者文檔
  - 更新 README.md 包含 Rust 版本安裝和使用說明
  - 建立命令使用範例和常見問題解答
  - 添加從 TypeScript 版本遷移的說明
  - 配置專案描述和貢獻指南
  - _需求: 非功能需求.可用性.文檔_

- [ ] 13.2 最終測試和發布準備
  - 執行完整的測試套件和相容性驗證
  - 驗證所有平台的編譯和安裝流程
  - 測試端到端的使用者工作流程
  - 準備第一個正式版本的發布標籤和說明
  - _需求: 所有需求的完整覆蓋驗證_

## 依賴關係
- 1.1 → 1.2 → 2.1, 2.2
- 2.1, 2.2 → 3.1 → 3.2, 3.3
- 3.1, 3.2, 3.3 → 4.1, 4.2
- 4.1, 4.2 → 5.1, 5.2
- 3.1-5.2 → 6.1, 6.2, 6.3
- 6.1, 6.2, 6.3 → 7.1, 7.2
- 7.1, 7.2 → 8.1, 8.2
- 8.1, 8.2 → 9.1, 9.2
- 所有核心功能 → 10.1, 10.2, 11.1, 11.2
- 10.1-11.2 → 12.1, 12.2
- 12.1, 12.2 → 13.1, 13.2

## 驗收標準
每個任務完成時必須滿足：
1. 程式碼編譯無錯誤和警告
2. 相關單元測試通過
3. 功能符合需求文件規格
4. 程式碼遵循 Rust 最佳實踐
5. 與設計文件中的 API 一致

## 里程碑
- **里程碑 1**: 基礎架構完成（任務 1-2）
- **里程碑 2**: 核心模組實作完成（任務 3-5）
- **里程碑 3**: CLI 命令功能完成（任務 6-7）
- **里程碑 4**: 建置和部署配置完成（任務 8-9）
- **里程碑 5**: 測試和驗證完成（任務 10-12）
- **里程碑 6**: 文檔和發布準備完成（任務 13）