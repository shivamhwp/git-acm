#!/bin/bash

set -e

# Function to show progress
progress() {
    echo "=> $1"
}

# Detect platform
PLATFORM="unknown"
case "$(uname -s)" in
    Linux*)  PLATFORM="linux";;
    Darwin*) PLATFORM="darwin";;
    MSYS*|MINGW*) PLATFORM="windows";;
    *)
        echo "Unsupported platform: $(uname -s)"
        exit 1
        ;;
esac

ARCH="x86_64"  # Add more architectures as needed

# Construct binary name
if [ "$PLATFORM" = "windows" ]; then
    BINARY="git-acm-windows-x86_64.exe"
else
    BINARY="git-acm-${PLATFORM}-${ARCH}"
fi

progress "Detected platform: $PLATFORM-$ARCH"

# Determine install location with fallbacks
if [ "$PLATFORM" = "windows" ]; then
    INSTALL_DIR="$HOME/bin"
    mkdir -p "$INSTALL_DIR"
else
    # Try /usr/local/bin first
    if [ -w "/usr/local/bin" ]; then
        INSTALL_DIR="/usr/local/bin"
    # Then try ~/.local/bin
    elif [ -d "$HOME/.local/bin" ] || mkdir -p "$HOME/.local/bin"; then
        INSTALL_DIR="$HOME/.local/bin"
        # Add to PATH if not already there
        if [[ ":$PATH:" != *":$HOME/.local/bin:"* ]]; then
            echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$HOME/.bashrc"
            echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$HOME/.zshrc" 2>/dev/null || true
        fi
    # Finally, try ~/bin
    else
        INSTALL_DIR="$HOME/bin"
        mkdir -p "$INSTALL_DIR"
        # Add to PATH if not already there
        if [[ ":$PATH:" != *":$HOME/bin:"* ]]; then
            echo 'export PATH="$HOME/bin:$PATH"' >> "$HOME/.bashrc"
            echo 'export PATH="$HOME/bin:$PATH"' >> "$HOME/.zshrc" 2>/dev/null || true
        fi
    fi
fi

progress "Installing to: $INSTALL_DIR"

# Use specific version tag instead of latest
VERSION="v1.0.0"  # Update this to match your latest version
DOWNLOAD_URL="https://github.com/YOUR_USERNAME/YOUR_REPO/releases/download/${VERSION}/${BINARY}"

progress "Downloading version ${VERSION}..."

# Download and install
curl -sL "$DOWNLOAD_URL" -o "$INSTALL_DIR/git-acm"
chmod +x "$INSTALL_DIR/git-acm"

progress "Installation complete! Try running: git acm"
progress "Note: You may need to restart your terminal or run 'source ~/.bashrc' to update your PATH"