#!/bin/bash

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Functions
print_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Configuration
REPO_URL="https://github.com/enohuang3108/worktree"
BINARY_NAME="wt"
INSTALL_DIR="$HOME/.local/bin"

# Detect OS and architecture
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case $ARCH in
    x86_64) ARCH="x64" ;;
    arm64|aarch64) ARCH="arm64" ;;
    *)
        print_error "Unsupported architecture: $ARCH"
        exit 1
        ;;
esac

case $OS in
    linux) PLATFORM="linux" ;;
    darwin) PLATFORM="macos" ;;
    *)
        print_error "Unsupported OS: $OS"
        exit 1
        ;;
esac

BINARY_URL="${REPO_URL}/releases/latest/download/wt-${PLATFORM}-${ARCH}"

print_info "🌳 Git Worktree CLI Tool Installer"
print_info "Detected platform: ${PLATFORM}-${ARCH}"

# Check if curl is available
if ! command -v curl &> /dev/null; then
    print_error "curl is required but not installed. Please install curl and try again."
    exit 1
fi

# Create temporary directory
TEMP_DIR=$(mktemp -d)
TEMP_BINARY="$TEMP_DIR/$BINARY_NAME"

print_info "📥 Downloading wt binary..."

# Download binary
if curl -L --fail --silent --show-error -o "$TEMP_BINARY" "$BINARY_URL"; then
    print_success "Binary downloaded successfully"
else
    print_error "Failed to download binary from $BINARY_URL"
    print_info "Please check if the release exists for your platform"
    exit 1
fi

# Make binary executable
chmod +x "$TEMP_BINARY"

# Create install directory if it doesn't exist
mkdir -p "$INSTALL_DIR"

print_info "📦 Installing wt to $INSTALL_DIR..."

# Install binary
if mv "$TEMP_BINARY" "$INSTALL_DIR/$BINARY_NAME"; then
    print_success "wt installed successfully!"
else
    print_error "Failed to install wt to $INSTALL_DIR"
    exit 1
fi

# Cleanup
rm -rf "$TEMP_DIR"

# Verify installation
if command -v wt &> /dev/null; then
    print_success "✅ Installation verified! You can now use 'wt' command."
    print_info ""
    print_info "📖 Usage examples:"
    print_info "  wt add        # Create new worktree"
    print_info "  wt remove     # Remove worktree"
    print_info "  wt open       # Open worktree in VSCode"
    print_info "  wt --help     # Show help"
else
    print_warning "wt command not found in PATH. Adding $INSTALL_DIR to PATH..."
    
    # Detect shell and add to appropriate profile
    SHELL_NAME=$(basename "$SHELL")
    case $SHELL_NAME in
        bash)
            PROFILE_FILE="$HOME/.bashrc"
            ;;
        zsh)
            PROFILE_FILE="$HOME/.zshrc"
            ;;
        fish)
            PROFILE_FILE="$HOME/.config/fish/config.fish"
            ;;
        *)
            PROFILE_FILE="$HOME/.profile"
            ;;
    esac
    
    # Check if PATH export already exists
    if ! grep -q "$INSTALL_DIR" "$PROFILE_FILE" 2>/dev/null; then
        echo "" >> "$PROFILE_FILE"
        echo "# Added by wt installer" >> "$PROFILE_FILE"
        if [[ $SHELL_NAME == "fish" ]]; then
            echo "set -gx PATH $INSTALL_DIR \$PATH" >> "$PROFILE_FILE"
        else
            echo "export PATH=\"$INSTALL_DIR:\$PATH\"" >> "$PROFILE_FILE"
        fi
        print_success "Added $INSTALL_DIR to PATH in $PROFILE_FILE"
        print_info "Please restart your terminal or run: source $PROFILE_FILE"
    else
        print_info "$INSTALL_DIR already in PATH configuration"
    fi
fi

print_info ""
print_success "🎉 Installation complete!"
