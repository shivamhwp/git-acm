#!/bin/bash

set -euo pipefail

progress() {
    echo "=> $1"
}

die() {
    echo "error: $1" >&2
    exit 1
}

require_cmd() {
    command -v "$1" >/dev/null 2>&1 || die "required tool '$1' is not installed"
}

# Cleanup temp files on exit
TMP_DIR=""
cleanup() {
    if [ -n "${TMP_DIR:-}" ] && [ -d "$TMP_DIR" ]; then
        rm -rf "$TMP_DIR"
    fi
}
trap cleanup EXIT

# Verify required tools early
require_cmd curl
if command -v sha256sum >/dev/null 2>&1; then
    :
elif command -v shasum >/dev/null 2>&1; then
    :
else
    die "no sha256 tool found (install 'sha256sum' or 'shasum')"
fi
require_cmd tar
require_cmd install

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

ARCH_RAW="$(uname -m)"
case "$ARCH_RAW" in
    x86_64|amd64)
        ARCH="x86_64";;
    arm64|aarch64)
        if [ "$PLATFORM" = "linux" ]; then
            ARCH="aarch64"
        else
            ARCH="arm64"
        fi;;
    *)
        echo "unsupported architecture: $ARCH_RAW"
        exit 1;;
esac
VERSION=$(curl -s https://api.github.com/repos/shivamhwp/git-acm/releases/latest | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
# gets the latest version 

# Construct asset name and URLs (archives + aggregated checksums)
if [ "$PLATFORM" = "windows" ]; then
    die "windows is not supported by this installer; use install.ps1 instead"
fi

ASSET="git-acm-${PLATFORM}-${ARCH}.tar.gz"
ASSET_URL="https://github.com/shivamhwp/git-acm/releases/download/${VERSION}/${ASSET}"
CHECKSUMS_URL="https://github.com/shivamhwp/git-acm/releases/download/${VERSION}/checksums.txt"

# Create temporary directory
TMP_DIR=$(mktemp -d)
TMP_ASSET="${TMP_DIR}/${ASSET}"
TMP_CHECKSUMS="${TMP_DIR}/checksums.txt"
TMP_EXPECTED_CHECKSUM="${TMP_DIR}/expected.sha256"

progress "version: $VERSION"
progress "asset:   $ASSET"

# Download checksums and asset
progress "downloading checksums..."
curl -sLf "$CHECKSUMS_URL" -o "$TMP_CHECKSUMS"

progress "downloading asset..."
curl -sLf "$ASSET_URL" -o "$TMP_ASSET"

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

EXPECTED_CHECKSUM=$(grep -E "[[:space:]]${ASSET}$" "$TMP_CHECKSUMS" | head -n1 | awk '{print $1}' | tr -d ' \r\n')
if [ -z "$EXPECTED_CHECKSUM" ]; then
    echo "error: could not find expected checksum for $ASSET in checksums.txt"
    echo "looked in: $CHECKSUMS_URL"
    exit 1
fi

progress "verifying checksum..."
LOCAL_CHECKSUM=$($SHA256_CMD "$TMP_ASSET" | awk '{print $1}' | tr -d ' \r\n')
if [ "${LOCAL_CHECKSUM,,}" != "${EXPECTED_CHECKSUM,,}" ]; then
    echo "error: checksum verification failed for $ASSET"
    echo "expected: $EXPECTED_CHECKSUM"
    echo "actual:   $LOCAL_CHECKSUM"
    echo "url:      $ASSET_URL"
    exit 1
fi


# Determine install location (prefer /usr/local/bin if writable; fallback to ~/.local/bin)
if [ -w "/usr/local/bin" ]; then
    INSTALL_DIR="/usr/local/bin"
else
    INSTALL_DIR="$HOME/.local/bin"
    mkdir -p "$INSTALL_DIR"
fi

# Extract and install binary
EXTRACT_DIR="$TMP_DIR/extract"
mkdir -p "$EXTRACT_DIR"
tar -xzf "$TMP_ASSET" -C "$EXTRACT_DIR"
# expect structure: git-acm-<plat>/git-acm
EX_BIN=$(find "$EXTRACT_DIR" -type f -name git-acm | head -n1)
if [ -z "$EX_BIN" ]; then
    echo "error: could not locate git-acm binary in archive"
    exit 1
fi
install -m 0755 "$EX_BIN" "$INSTALL_DIR/git-acm"

# Handle macOS specific security
if [ "$PLATFORM" = "darwin" ]; then
    progress "talking to the macos gods"
    xattr -d com.apple.quarantine "$INSTALL_DIR/git-acm" 2>/dev/null || true
    # If using newer macOS versions, might need to add to security list
    if [ -x "/usr/bin/spctl" ]; then
        sudo spctl --add "$INSTALL_DIR/git-acm" 2>/dev/null || true
    fi
fi

# PATH configuration (idempotent, shell-aware; skip in non-interactive/CI)
SKIP_PATH_EDITS=0
if [ -n "${CI:-}" ] || [ ! -t 1 ]; then
    SKIP_PATH_EDITS=1
fi

SHELL_NAME=$(basename "${SHELL:-}")
PROFILE_FILE=""
ADD_LINE=""

if [ "$SKIP_PATH_EDITS" -eq 0 ] && [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    case "$SHELL_NAME" in
        zsh)
            if [ "$PLATFORM" = "darwin" ]; then
                PROFILE_FILE="$HOME/.zprofile"
            else
                PROFILE_FILE="$HOME/.zshrc"
            fi
            ADD_LINE="export PATH=\"$INSTALL_DIR:\$PATH\""
            ;;
        bash)
            if [ "$PLATFORM" = "darwin" ]; then
                PROFILE_FILE="$HOME/.bash_profile"
            else
                PROFILE_FILE="$HOME/.bashrc"
            fi
            ADD_LINE="export PATH=\"$INSTALL_DIR:\$PATH\""
            ;;
        fish)
            mkdir -p "$HOME/.config/fish/conf.d"
            PROFILE_FILE="$HOME/.config/fish/conf.d/git-acm.fish"
            ADD_LINE="set -gx PATH $INSTALL_DIR \$PATH"
            ;;
        *)
            PROFILE_FILE="$HOME/.profile"
            ADD_LINE="export PATH=\"$INSTALL_DIR:\$PATH\""
            ;;
    esac

    if [ -n "$PROFILE_FILE" ]; then
        mkdir -p "$(dirname "$PROFILE_FILE")"
        if [ ! -f "$PROFILE_FILE" ] || ! grep -qsF "$INSTALL_DIR" "$PROFILE_FILE"; then
            printf '%s\n' "$ADD_LINE" >> "$PROFILE_FILE"
            progress "added $INSTALL_DIR to PATH in $(basename "$PROFILE_FILE")"
        else
            progress "PATH already configured in $(basename "$PROFILE_FILE")"
        fi
    fi
else
    if [ "$SKIP_PATH_EDITS" -eq 1 ] && [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
        progress "non-interactive shell detected; skipping PATH edits"
    fi
fi

progress "installed git-acm $VERSION"
progress "asset: $ASSET"
progress "path: $INSTALL_DIR/git-acm"
progress "try it: git-acm --help"
progress "all done 🎉 !!!"

# Do not source shell configs; print instructions instead
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
  case "$SHELL_NAME" in
    zsh)
      if [ "$PLATFORM" = "darwin" ]; then hint_file="~/.zprofile"; else hint_file="~/.zshrc"; fi ;;
    bash)
      if [ "$PLATFORM" = "darwin" ]; then hint_file="~/.bash_profile"; else hint_file="~/.bashrc"; fi ;;
    fish)
      hint_file="~/.config/fish/conf.d/git-acm.fish" ;;
    *)
      hint_file="~/.profile" ;;
  esac
  progress "Add to PATH manually if needed: $INSTALL_DIR"
  progress "or append the PATH update to $hint_file and restart your shell"
fi
