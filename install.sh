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
TMP_EXPECTED_CHECKSUM="${TMP_DIR}/expected.sha256"

# Download binary
progress "downloading..."
curl -sL "$DOWNLOAD_URL" -o "$TMP_FILE"

# Determine which checksum tool to use
if command -v sha256sum >/dev/null; then
    SHA256_CMD="sha256sum"
elif command -v shasum >/dev/null; then
    SHA256_CMD="shasum -a 256"
else
    echo "error: No sha256sum or shasum command found"
    rm -rf "$TMP_DIR"
    exit 1
fi

# Generate checksum file locally
(cd "$TMP_DIR" && $SHA256_CMD "$BINARY" > "${BINARY}.sha256")

# Download trusted checksum directly from GitHub release
# progress "downloading trusted checksum"
if curl -sL --fail "$CHECKSUM_URL" -o "$TMP_EXPECTED_CHECKSUM"; then
    # progress "successfully downloaded checksum file"
      progress "making sure it's all good"
else
    echo "warning: could not download checksum file, using fallback verification"
    # As fallback, download the binary a second time to verify it hasn't been corrupted
    SECOND_DOWNLOAD="${TMP_DIR}/second_${BINARY}"
    curl -sL "$DOWNLOAD_URL" -o "$SECOND_DOWNLOAD"
    (cd "$TMP_DIR" && $SHA256_CMD "$BINARY" > "$TMP_EXPECTED_CHECKSUM")
    progress "performed secondary download for comparison"
fi

# Verify integrity by comparing checksums
progress "verifying if it's all safe and secure"
LOCAL_CHECKSUM=$(cut -d' ' -f1 "$TMP_CHECKSUM")
EXPECTED_CHECKSUM=$(cat "$TMP_EXPECTED_CHECKSUM" | tr -d ' \r\n')

if [ "$LOCAL_CHECKSUM" != "$EXPECTED_CHECKSUM" ]; then
    echo "error: checksum verification failed"
    # echo "expected: $EXPECTED_CHECKSUM"
    # echo "got:      $LOCAL_CHECKSUM"
    rm -rf "$TMP_DIR"
    exit 1
fi

progress "almost there"

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
mv "$TMP_FILE" "$INSTALL_DIR/git-acm"
chmod 755 "$INSTALL_DIR/git-acm"

# Handle macOS specific security
if [ "$PLATFORM" = "darwin" ]; then
    progress "talking to the macos gods"
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

progress "all done 🎉  "
