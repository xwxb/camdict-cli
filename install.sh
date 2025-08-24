#!/bin/bash

# Detect the OS
OS="$(uname)"

# Set the file name based on the OS
if [ "$OS" == "Darwin" ]; then
    FILE_NAME="fcd_M1"
elif [ "$OS" == "Linux" ]; then
    FILE_NAME="fcd_x86_64"
else
    echo "Unsupported OS: $OS"
    exit 1
fi

# Get the latest release tag from GitHub
LATEST_RELEASE_TAG=$(curl --silent "https://api.github.com/repos/xwxb/camdict-cli/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

# Check if we got a valid release tag
if [ -z "$LATEST_RELEASE_TAG" ]; then
    echo "Error: Could not fetch the latest release tag from GitHub"
    exit 1
fi

echo "Found latest release: $LATEST_RELEASE_TAG"

# Define URLs with official GitHub as fallback
PROXY_URL="https://ghproxy.net/https://github.com/xwxb/camdict-cli/releases/download/${LATEST_RELEASE_TAG}/${FILE_NAME}"
OFFICIAL_URL="https://github.com/xwxb/camdict-cli/releases/download/${LATEST_RELEASE_TAG}/${FILE_NAME}"

# Try downloading from proxy first (with 10 second timeout)
echo "Downloading from proxy URL..."
if ! curl --connect-timeout 10 --max-time 30 -L -o fcd "$PROXY_URL"; then
    echo "Proxy download failed, trying official GitHub URL..."
    # If proxy fails, try the official GitHub URL
    if ! curl --connect-timeout 10 --max-time 60 -L -o fcd "$OFFICIAL_URL"; then
        echo "Error: Failed to download from both proxy and official GitHub URLs"
        exit 1
    fi
    echo "Downloaded successfully from official GitHub URL"
else
    echo "Downloaded successfully from proxy URL"
fi

# Verify the downloaded file is not empty and appears to be a binary
if [ ! -f "fcd" ]; then
    echo "Error: Download file not found"
    exit 1
fi

# Check if file is not empty and has reasonable size (should be at least 1MB for a Rust binary)
file_size=$(stat -c%s fcd 2>/dev/null || stat -f%z fcd 2>/dev/null || echo "0")
if [ "$file_size" -lt 1000000 ]; then
    echo "Error: Downloaded file appears to be invalid (size: ${file_size} bytes)"
    echo "This might indicate the release assets are not available yet."
    rm -f fcd
    exit 1
fi

echo "Downloaded binary file (${file_size} bytes)"

# Make the file executable
chmod +x fcd

# Move the file to /usr/local/bin
sudo mv fcd /usr/local/bin

echo "Installation completed successfully."