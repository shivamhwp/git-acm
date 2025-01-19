#!/bin/bash

set -e

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
        echo "unsupported platform: $(uname -s)"
        exit 1
        ;;
esac

ARCH="x86_64"
VERSION=$(curl -s https://api.github.com/repos/shivamhwp/git-acm/releases/latest | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
# gets the latest version 

# Construct binary name and URLs
if [ "$PLATFORM" = "windows" ]; then
    BINARY="git-acm-windows-x86_64.exe"
else
    BINARY="git-acm-${PLATFORM}-${ARCH}"
fi

DOWNLOAD_URL="https://github.com/shivamhwp/git-acm/releases/download/${VERSION}/${BINARY}"
CHECKSUM_URL="${DOWNLOAD_URL}.sha256"

# Create temporary directory
TMP_DIR=$(mktemp -d)
TMP_FILE="${TMP_DIR}/${BINARY}"
TMP_CHECKSUM="${TMP_DIR}/${BINARY}.sha256"

# Download files
progress "downloading binary and checksum..."
curl -sL "$DOWNLOAD_URL" -o "$TMP_FILE"
curl -sL "$CHECKSUM_URL" -o "$TMP_CHECKSUM"

# Verify checksum
progress "verifying checksum..."
if command -v sha256sum >/dev/null; then
    SHA256_CMD="sha256sum"
elif command -v shasum >/dev/null; then
    SHA256_CMD="shasum -a 256"
else
    echo "error: No sha256sum or shasum command found"
    exit 1
fi

if ! (cd "$TMP_DIR" && $SHA256_CMD -c "${BINARY}.sha256"); then
    echo "error: checksum verification failed"
    rm -rf "$TMP_DIR"
    exit 1
fi

# Determine install location
if [ "$PLATFORM" = "darwin" ]; then
    INSTALL_DIR="$HOME/.local/bin"
    mkdir -p "$INSTALL_DIR"
elif [ -w "/usr/local/bin" ]; then
    INSTALL_DIR="/usr/local/bin"
else
    INSTALL_DIR="$HOME/.local/bin"
    mkdir -p "$INSTALL_DIR"
fi

# Install binary
progress "installing to $INSTALL_DIR..."
mv "$TMP_FILE" "$INSTALL_DIR/git-acm"
chmod 755 "$INSTALL_DIR/git-acm"

# Handle macOS specific security
if [ "$PLATFORM" = "darwin" ]; then
    progress "handling macOS security..."
    xattr -d com.apple.quarantine "$INSTALL_DIR/git-acm" 2>/dev/null || true
    # If using newer macOS versions, might need to add to security list
    if [ -x "/usr/bin/spctl" ]; then
        sudo spctl --add "$INSTALL_DIR/git-acm" 2>/dev/null || true
    fi
fi

# Add to PATH if needed
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    if [ -f "$HOME/.zshrc" ]; then
        echo "export PATH=\"$INSTALL_DIR:\$PATH\"" >> "$HOME/.zshrc"
    fi
    if [ -f "$HOME/.bashrc" ]; then
        echo "export PATH=\"$INSTALL_DIR:\$PATH\"" >> "$HOME/.bashrc"
    fi
fi

# Cleanup
rm -rf "$TMP_DIR"

progress "installation complete! you may need to:"
progress "1. run 'source ~/.zshrc' or restart your terminal"
if [ "$PLATFORM" = "darwin" ]; then
    progress "2. go to system preferences -> security & privacy and allow the binary if prompted"
fi
progress "try running: git-acm"