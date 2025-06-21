#!/bin/bash
# Manual ChromeDriver installation for restricted environments

set -e

echo "🔧 Manual ChromeDriver installation for Ubuntu 24.04..."

# Create directory for ChromeDriver
sudo mkdir -p /usr/local/bin

# Download ChromeDriver manually (you may need to update the version)
CHROMEDRIVER_VERSION="114.0.5735.90"
wget -O /tmp/chromedriver.zip "https://storage.googleapis.com/chrome-for-testing-public/${CHROMEDRIVER_VERSION}/linux64/chromedriver-linux64.zip"

# Extract and install
sudo unzip /tmp/chromedriver.zip -d /tmp/
sudo mv /tmp/chromedriver-linux64/chromedriver /usr/local/bin/
sudo chmod +x /usr/local/bin/chromedriver

# Clean up
rm -rf /tmp/chromedriver.zip /tmp/chromedriver-linux64/

# Verify installation
echo "✅ ChromeDriver installed:"
/usr/local/bin/chromedriver --version

echo "✅ Manual installation complete!"