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

# Use specific version tag
VERSION="v1.0.0"  # Update this to match your latest version
DOWNLOAD_URL="https://github.com/shivamhwp/git-acp/releases/download/${VERSION}/${BINARY}"

progress "Downloading version ${VERSION}..."

# Create temporary directory for download
TMP_DIR=$(mktemp -d)
TMP_FILE="${TMP_DIR}/${BINARY}"

# Download the binary to temporary location
curl -sL "$DOWNLOAD_URL" -o "$TMP_FILE"

# Make it executable
chmod +x "$TMP_FILE"

# Move to final location with correct permissions
mv "$TMP_FILE" "$INSTALL_DIR/git-acm"
chmod 755 "$INSTALL_DIR/git-acm"

# Clean up
rm -rf "$TMP_DIR"

progress "Installation complete! Try running: git acm"
progress "Note: You may need to restart your terminal or run 'source ~/.bashrc' to update your PATH"