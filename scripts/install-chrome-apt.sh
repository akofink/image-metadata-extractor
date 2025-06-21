#!/bin/bash
# Install Chrome and ChromeDriver via APT on Ubuntu 24.04

set -e

echo "🔧 Installing Google Chrome and ChromeDriver via APT..."

# Update package list
sudo apt-get update

# Install dependencies
sudo apt-get install -y wget gnupg software-properties-common unzip curl

# Add Google's GPG key
wget -q -O - https://dl.google.com/linux/linux_signing_key.pub | sudo apt-key add -

# Add Google Chrome repository
echo "deb [arch=amd64] http://dl.google.com/linux/chrome/deb/ stable main" | sudo tee /etc/apt/sources.list.d/google-chrome.list

# Update package list again
sudo apt-get update

# Install Google Chrome
sudo apt-get install -y google-chrome-stable

# Get Chrome version and download matching ChromeDriver
CHROME_VERSION=$(google-chrome --version | grep -oE "[0-9]+\.[0-9]+\.[0-9]+\.[0-9]+")
CHROME_MAJOR_VERSION=$(echo $CHROME_VERSION | cut -d. -f1)

echo "Chrome version: $CHROME_VERSION"
echo "Chrome major version: $CHROME_MAJOR_VERSION"

# Download ChromeDriver for the specific Chrome version
# Try the new Chrome for Testing endpoints first
CHROMEDRIVER_URL="https://edgedl.me.gvt1.com/edgedl/chrome/chrome-for-testing/${CHROME_VERSION}/linux64/chromedriver-linux64.zip"

echo "Attempting to download ChromeDriver from: $CHROMEDRIVER_URL"

# Try to download ChromeDriver
if wget -q --spider "$CHROMEDRIVER_URL"; then
    echo "✅ Found ChromeDriver for exact version"
    wget -O /tmp/chromedriver.zip "$CHROMEDRIVER_URL"
    sudo unzip -o /tmp/chromedriver.zip -d /tmp/
    sudo mv /tmp/chromedriver-linux64/chromedriver /usr/local/bin/
    sudo chmod +x /usr/local/bin/chromedriver
    rm -rf /tmp/chromedriver.zip /tmp/chromedriver-linux64/
else
    echo "⚠️  Exact version not found, trying latest stable..."
    # Fallback to a known working version
    FALLBACK_VERSION="119.0.6045.105"
    FALLBACK_URL="https://edgedl.me.gvt1.com/edgedl/chrome/chrome-for-testing/${FALLBACK_VERSION}/linux64/chromedriver-linux64.zip"
    
    if wget -q --spider "$FALLBACK_URL"; then
        echo "✅ Using fallback ChromeDriver version: $FALLBACK_VERSION"
        wget -O /tmp/chromedriver.zip "$FALLBACK_URL"
        sudo unzip -o /tmp/chromedriver.zip -d /tmp/
        sudo mv /tmp/chromedriver-linux64/chromedriver /usr/local/bin/
        sudo chmod +x /usr/local/bin/chromedriver
        rm -rf /tmp/chromedriver.zip /tmp/chromedriver-linux64/
    else
        echo "❌ Could not download ChromeDriver. You may need to install it manually."
        exit 1
    fi
fi

# Verify installations
echo "✅ Chrome version:"
google-chrome --version

echo "✅ ChromeDriver version:"
/usr/local/bin/chromedriver --version

# Add ChromeDriver to PATH if not already there
if ! grep -q "/usr/local/bin" ~/.bashrc; then
    echo 'export PATH="/usr/local/bin:$PATH"' >> ~/.bashrc
    export PATH="/usr/local/bin:$PATH"
fi

echo "✅ Installation complete!"
echo "💡 You may need to run 'source ~/.bashrc' or restart your shell."