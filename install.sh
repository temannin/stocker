#!/usr/bin/env bash

# Function to display an error message and exit
function error_exit {
    echo "$1"
    exit 1
}


# Extract the download URL for the binary (assuming the binary is for Linux x86_64)
DOWNLOAD_URL=https://github.com/temannin/stocker/releases/latest/download/stocker

# Step 2: Download the release binary
echo "Downloading stocker binary..."
curl -L -o stocker "$DOWNLOAD_URL" || error_exit "Failed to download the binary."

# Step 3: Make the binary executable
chmod +x stocker || error_exit "Failed to make the binary executable."

# Step 4: Move the binary to a directory in the PATH (e.g., /usr/local/bin)
echo "Installing stocker to /usr/local/bin..."
sudo mv stocker /usr/local/bin/ || error_exit "Failed to move the binary to /usr/local/bin."

echo "Stocker installed successfully!"
