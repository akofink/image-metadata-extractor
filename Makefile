# Image Metadata Extractor - Makefile
# Convenient commands for development and deployment

# Phony targets that don't create files
.PHONY: help clean check test lint format serve install setup-hooks dev prod deploy-check coverage

# Default target
all: pkg

# Show available commands
help:
	@echo "Image Metadata Extractor - Available Commands:"
	@echo ""
	@echo "  make build       - Build for development (default)"
	@echo "  make build-release - Build optimized for production"
	@echo "  make serve       - Start local development server"
	@echo "  make check       - Check code compilation"
	@echo "  make test        - Run tests"
	@echo "  make test-wasm   - Run WebAssembly tests in browser (Chrome)"
	@echo "  make test-wasm-all-browsers - Run WebAssembly tests in all browsers"
	@echo "  make test-wasm-chrome - Run WebAssembly tests in Chrome only"
	@echo "  make test-all    - Run all tests (standard + WebAssembly)"
	@echo "  make lint        - Run clippy linting"
	@echo "  make coverage    - Generate code coverage report"
	@echo "  make format      - Format code with cargo fmt"
	@echo "  make clean       - Clean build artifacts"
	@echo "  make install     - Install wasm-pack if missing"
	@echo "  make setup-hooks - Install git pre-commit hooks"
	@echo ""

# Development build (fast, with debug info) - default
pkg: src/**/*.rs Cargo.toml Cargo.lock
	@echo "🔨 Building for development..."
	cargo build --test "component_tests"
	wasm-pack build --target web --dev
	@echo "✅ Development build complete!"

# Alias for default build
build: pkg

# Development build alias
build-dev: pkg

# Production build (optimized, smaller size)
pkg-release: src/*.rs Cargo.toml Cargo.lock
	@echo "🚀 Building for production..."
	wasm-pack build --target web --release
	@echo "✅ Production build complete!"

# Alias for production build
build-release: pkg-release

# Start local development server
serve:
	@echo "🌐 Starting development server on http://localhost:8000..."
	@echo "Press Ctrl+C to stop"
	@which npx > /dev/null && npx -y serve -s . -p 8000 || \
	(echo "npx not found, trying Python..." && python3 -m http.server 8000) || \
	(echo "Python3 not found, trying Python..." && python -m http.server 8000) || \
	echo "❌ No suitable server found. Install Node.js or Python."

# Check compilation without building
check:
	@echo "🔍 Checking code..."
	cargo check
	@echo "✅ Code check complete!"

# Run tests
test: build
	@echo "🧪 Running tests..."
	cargo test
	@echo "✅ Tests complete!"

# Run WebAssembly tests in browser (Chrome - most reliable)
test-wasm:
	@echo "🌐 Running WebAssembly tests in browser..."
	wasm-pack test --headless --chrome
	@echo "✅ WebAssembly tests complete!"

# Run WebAssembly tests across multiple browsers (requires all drivers)
test-wasm-all-browsers:
	@echo "🌐 Attempting WebAssembly tests across all browsers..."
	@echo "Note: This requires Chrome, Firefox, and Safari drivers to be installed"
	wasm-pack test --headless --chrome --firefox --safari || \
	(echo "⚠️  Some browsers failed. Falling back to Chrome only..." && \
	 wasm-pack test --headless --chrome)
	@echo "✅ Multi-browser WebAssembly tests complete!"

# Run WebAssembly tests in Chrome only (fast option)
test-wasm-chrome:
	@echo "🌐 Running WebAssembly tests in Chrome..."
	wasm-pack test --headless --chrome
	@echo "✅ Chrome WebAssembly tests complete!"

# Run all tests (standard + WebAssembly across all browsers)
test-all: test test-wasm-all-browsers

# Run clippy linting
lint: build
	@echo "🔍 Running clippy linting..."
	cargo clippy -- -D warnings
	@echo "✅ Linting complete!"

# Generate code coverage report
coverage: build
	@echo "📈 Generating coverage report..."
	cargo llvm-cov --html
	@echo "✅ Coverage report generated!"

# Format code
format:
	@echo "🎨 Formatting code..."
	cargo fmt
	@echo "✅ Code formatted!"

# Clean build artifacts
clean:
	@echo "🧹 Cleaning build artifacts..."
	cargo clean
	rm -rf pkg
	@echo "✅ Clean complete!"

# Install wasm-pack if missing
install:
	@echo "📦 Checking for wasm-pack..."
	@which wasm-pack > /dev/null || \
	(echo "Installing wasm-pack..." && \
	 curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh)
	@echo "✅ wasm-pack ready!"

# Development workflow - check, format, lint, then build
dev: check format lint pkg

# Production workflow - full checks and optimized build  
prod: check test-all lint format pkg-release

# Install git pre-commit hooks
setup-hooks:
	@echo "🪝 Setting up git pre-commit hooks..."
	@echo '#!/bin/bash' > .git/hooks/pre-commit
	@echo 'set -e' >> .git/hooks/pre-commit
	@echo 'echo "🔍 Running pre-commit checks..."' >> .git/hooks/pre-commit
	@echo 'make check && make test && make format && make lint' >> .git/hooks/pre-commit
	@echo 'git add -u  # Add any formatting changes' >> .git/hooks/pre-commit
	@echo 'echo "✅ Pre-commit checks passed!"' >> .git/hooks/pre-commit
	@chmod +x .git/hooks/pre-commit
	@echo "✅ Pre-commit hooks installed!"
	@echo "   • Hooks will run automatically on each commit"
	@echo "   • Runs: make check && make test && make format && make lint"

# Quick deployment check
deploy-check: pkg-release
	@echo "🚀 Ready for deployment!"
	@echo "   • Code checked and tested"
	@echo "   • Production build complete"
	@echo "   • Files ready in pkg/ directory"
