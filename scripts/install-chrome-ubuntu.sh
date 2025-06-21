#!/bin/bash
# Install Chrome and ChromeDriver on Ubuntu 24.04

set -e

echo "🔧 Installing Google Chrome and ChromeDriver on Ubuntu 24.04..."

# Update package list
sudo apt-get update

# Install dependencies
sudo apt-get install -y wget gnupg software-properties-common

# Add Google's GPG key
wget -q -O - https://dl.google.com/linux/linux_signing_key.pub | sudo apt-key add -

# Add Google Chrome repository
echo "deb [arch=amd64] http://dl.google.com/linux/chrome/deb/ stable main" | sudo tee /etc/apt/sources.list.d/google-chrome.list

# Update package list again
sudo apt-get update

# Install Google Chrome
sudo apt-get install -y google-chrome-stable

# Install ChromeDriver via snap (usually more reliable)
sudo snap install chromium-chromedriver

# Alternative: Install ChromeDriver directly
# CHROME_VERSION=$(google-chrome --version | grep -oE "[0-9]+\.[0-9]+\.[0-9]+")
# CHROMEDRIVER_VERSION=$(curl -sS https://chromedriver.storage.googleapis.com/LATEST_RELEASE_${CHROME_VERSION%.*})
# wget -O /tmp/chromedriver.zip https://chromedriver.storage.googleapis.com/${CHROMEDRIVER_VERSION}/chromedriver_linux64.zip
# sudo unzip /tmp/chromedriver.zip -d /usr/local/bin/
# sudo chmod +x /usr/local/bin/chromedriver

# Verify installations
echo "✅ Chrome version:"
google-chrome --version

echo "✅ ChromeDriver version:"
if command -v chromium.chromedriver &> /dev/null; then
    chromium.chromedriver --version
else
    chromedriver --version 2>/dev/null || echo "ChromeDriver not found in PATH"
fi

echo "✅ Installation complete!"