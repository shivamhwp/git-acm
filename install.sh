#!/bin/bash

# Detect OS
OS="$(uname -s)"
case "${OS}" in
    Linux*)     MACHINE=linux;;
    Darwin*)    MACHINE=macos;;
    CYGWIN*|MINGW*|MSYS*) MACHINE=windows;;
    *)          echo "Unsupported operating system: ${OS}" && exit 1;;
esac

# Download latest release for detected OS
echo "Downloading git-acm for ${MACHINE}..."
curl -L "https://github.com/shivamhwp/git-acm/releases/latest/download/git-acm-${MACHINE}" -o git-acm
if [ $? -ne 0 ]; then
    echo "Error downloading git-acm for ${MACHINE}. Exiting."
    exit 1
fi

# Make executable and install
chmod +x git-acm

# Check if sudo is available, if not, notify the user
if ! command -v sudo &> /dev/null; then
    echo "sudo is required to install git-acm to /usr/local/bin. Please install sudo and try again."
    exit 1
fi

# Check if git-acm already exists in /usr/local/bin
if command -v git-acm &> /dev/null; then
    echo "git-acm is already installed. Overwriting the existing version."
fi

if [ "${MACHINE}" = "windows" ]; then
    # For Windows, we move the file with .exe extension
    mv git-acm git-acm.exe
    # No need to use sudo, just place it in a directory in PATH
    mv git-acm.exe /c/Program\ Files/git-acm/
else
    sudo mv git-acm /usr/local/bin/
fi

# Create config directory
mkdir -p ~/.config/git-acm

# Create default config if it doesn't exist
if [ ! -f ~/.config/git-acm/model.txt ]; then
    echo "openai" > ~/.config/git-acm/model.txt
fi

echo "git-acm installed successfully!"
