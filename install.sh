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

# url0="https://github.com/xwxb/camdict-cli/releases/download/${LATEST_RELEASE_TAG}/${FILE_NAME}"
# url1="https://kkgithub.com/xwxb/camdict-cli/releases/download/${LATEST_RELEASE_TAG}/${FILE_NAME}"
url0="https://ghproxy.net/https://github.com/xwxb/camdict-cli/releases/download/${LATEST_RELEASE_TAG}/${FILE_NAME}"

# # Test response time for each URL
# time0=$(curl -L -o /dev/null -s -w '%{time_starttransfer}\n' "$url0")
# time1=$(curl -L -o /dev/null -s -w '%{time_starttransfer}\n' "$url1")
# time2=$(curl -L -o /dev/null -s -w '%{time_starttransfer}\n' "$url2")

# # Choose the URL with the fastest response time
# FASTEST_URL="$url0"
# FASTEST_TIME="$time0"
# if (( $(echo "$time1 < $FASTEST_TIME" | bc -l) )); then
#     FASTEST_URL="$url1"
#     FASTEST_TIME="$time1"
# fi
# if (( $(echo "$time2 < $FASTEST_TIME" | bc -l) )); then
#     FASTEST_URL="$url2"
#     FASTEST_TIME="$time2"
# fi

# # Download the file using the fastest URL
# curl -L -o fcd "$FASTEST_URL"
curl -L -o fcd "$url0"

# Make the file executable
chmod +x fcd

# Move the file to /usr/local/bin
sudo mv fcd /usr/local/bin

echo "Installation completed successfully."