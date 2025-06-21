# Installation Scripts

This directory contains installation scripts for setting up Chrome and ChromeDriver on Ubuntu systems.

## Scripts

### `install-chrome-ubuntu.sh`
Comprehensive installation script for Ubuntu 24.04:
- Installs Google Chrome from official repository
- Installs ChromeDriver via snap (most reliable)
- Includes fallback manual installation method

**Usage:**
```bash
chmod +x scripts/install-chrome-ubuntu.sh
./scripts/install-chrome-ubuntu.sh
```

### `manual-chromedriver-install.sh` 
Manual ChromeDriver installation for restricted environments:
- Downloads ChromeDriver directly from Google
- Useful when package managers are unavailable
- May need version updates for compatibility

**Usage:**
```bash
chmod +x scripts/manual-chromedriver-install.sh
./scripts/manual-chromedriver-install.sh
```

## Alternative Testing Methods

If Chrome installation fails, use these Makefile targets:

- `make test-wasm-node` - Run tests in Node.js instead of Chrome
- `make test-wasm-fallback` - Try Chrome first, fallback to Node.js
- `make test` - Run only regular Rust tests (no WASM)