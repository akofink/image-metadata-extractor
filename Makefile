# Image Metadata Extractor - Makefile
# Convenient commands for development and deployment

.PHONY: help build build-dev build-release clean check test lint format serve install

# Default target
all: build

# Show available commands
help:
	@echo "Image Metadata Extractor - Available Commands:"
	@echo ""
	@echo "  make build       - Build for development (default)"
	@echo "  make build-release - Build optimized for production"
	@echo "  make serve       - Start local development server"
	@echo "  make check       - Check code compilation"
	@echo "  make test        - Run tests"
	@echo "  make lint        - Run clippy linting"
	@echo "  make format      - Format code with cargo fmt"
	@echo "  make clean       - Clean build artifacts"
	@echo "  make install     - Install wasm-pack if missing"
	@echo ""

# Development build (fast, with debug info)
build: build-dev

build-dev:
	@echo "🔨 Building for development..."
	wasm-pack build --target web --dev
	@echo "✅ Development build complete!"

# Production build (optimized, smaller size)
build-release:
	@echo "🚀 Building for production..."
	wasm-pack build --target web --release
	@echo "✅ Production build complete!"

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
test:
	@echo "🧪 Running tests..."
	cargo test
	@echo "✅ Tests complete!"

# Run clippy linting
lint:
	@echo "🔍 Running clippy linting..."
	cargo clippy
	@echo "✅ Linting complete!"

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
dev: check format lint build-dev

# Production workflow - full checks and optimized build  
prod: check test lint format build-release

# Quick deployment check
deploy-check: prod
	@echo "🚀 Ready for deployment!"
	@echo "   • Code checked and tested"
	@echo "   • Production build complete"
	@echo "   • Files ready in pkg/ directory"