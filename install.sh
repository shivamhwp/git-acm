#!/bin/bash

# Detect OS
OS="$(uname -s)"
case "${OS}" in
    Linux*)     MACHINE=linux;;
    Darwin*)    MACHINE=macos;;
    *)          echo "Unsupported operating system: ${OS}" && exit 1;;
esac

# Download latest release for detected OS
echo "Downloading git-acm for ${MACHINE}..."
curl -L "https://github.com/shivamhwp/git-acm/releases/latest/download/git-acm-${MACHINE}" -o git-acm

# Make executable and install
chmod +x git-acm
sudo mv git-acm /usr/local/bin/

# Create config directory
mkdir -p ~/.config/git-acm

# Create default config if it doesn't exist
if [ ! -f ~/.config/git-acm/model.txt ]; then
    echo "openai" > ~/.config/git-acm/model.txt
fi

echo "git-acm installed successfully!"