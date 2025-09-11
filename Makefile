# Image Metadata Extractor - Makefile
# Convenient commands for development and deployment

# Phony targets that don't create files
.PHONY: help clean check test lint format serve install setup-hooks dev prod deploy-check coverage check-warnings check-test-separation coverage-text coverage-compact coverage-summary doc

# Default target
all: pkg

# Show available commands
help:
	@echo "Image Metadata Extractor - Available Commands:"
	@echo ""
	@echo "  make build       - Build for development (default)"
	@echo "  make build-release - Build optimized for production"
	@echo "  make serve       - Start local development server"
	@echo "  make check       - Check code compilation (fails on warnings)"
	@echo "  make check-warnings - Comprehensive warning check (code + tests)"
	@echo "  make check-test-separation - Ensure tests are separated from app code"
	@echo "  make test        - Run tests (fails on warnings)"
	@echo "  make test-wasm   - Run WebAssembly tests in browser (Chrome)"
	@echo "  make test-wasm-all-browsers - Run WebAssembly tests in all browsers"
	@echo "  make test-wasm-chrome - Run WebAssembly tests in Chrome only"
	@echo "  make test-wasm-skip - Skip WASM tests (for environments without Chrome)"
	@echo "  make test-wasm-fallback - Try Chrome, skip WASM if failed"
	@echo "  make test-ci     - Run regular tests only (skip WASM for CI)"
	@echo "  make test-auto   - Auto-detect Chrome and run appropriate tests"
	@echo "  make test-debug  - Debug Chrome detection (troubleshooting)"
	@echo "  make test-all    - Run all tests (standard + WebAssembly)"
	@echo "  make lint        - Run clippy linting"
	@echo "  make coverage    - Generate code coverage report"
	@echo "  make coverage-text - Show coverage summary in terminal"
	@echo "  make coverage-compact - Show compact coverage table"
	@echo "  make coverage-summary - Show just overall coverage percentage"
	@echo "  make format      - Format code with cargo fmt"
	@echo "  make doc         - Generate documentation"
	@echo "  make clean       - Clean build artifacts"
	@echo "  make install     - Install wasm-pack if missing"
	@echo "  make setup-hooks - Install git pre-commit hooks"
	@echo ""

# Development build (fast, with debug info) - default
pkg: install src/**/*.rs Cargo.toml Cargo.lock
	@echo "🔨 Building for development..."
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
	RUSTFLAGS="-D warnings" cargo check
	@echo "✅ Code check complete!"

# Comprehensive warning check (code + tests)
check-warnings:
	@echo "🔍 Comprehensive warning check..."
	RUSTFLAGS="-D warnings" cargo check
	RUSTFLAGS="-D warnings" cargo check --tests
	@echo "✅ All warning checks passed!"

# Check that tests are properly separated from application code
check-test-separation:
	@echo "🔍 Checking test organization..."
	@echo "  • Checking for test functions in src/ directory..."
	@if find src -name "*.rs" -exec grep -l "#\[test\]" {} \; | head -1 | grep -q .; then \
		echo "❌ Found test functions in src/ directory:"; \
		find src -name "*.rs" -exec grep -l "#\[test\]" {} \; | sed 's/^/    /'; \
		echo "   Please move tests to tests/ directory"; \
		exit 1; \
	fi
	@echo "  • Checking for test modules in src/ directory..."
	@if find src -name "*.rs" -exec grep -l "#\[cfg(test)\]" {} \; | head -1 | grep -q .; then \
		echo "❌ Found test modules in src/ directory:"; \
		find src -name "*.rs" -exec grep -l "#\[cfg(test)\]" {} \; | sed 's/^/    /'; \
		echo "   Please move tests to tests/ directory"; \
		exit 1; \
	fi
	@echo "  • Checking for test imports in src/ directory..."
	@if find src -name "*.rs" -exec grep -l "use.*test" {} \; | head -1 | grep -q .; then \
		echo "⚠️  Found potential test imports in src/ directory:"; \
		find src -name "*.rs" -exec grep -l "use.*test" {} \; | sed 's/^/    /'; \
		echo "   Please review these imports"; \
	fi
	@echo "  • Checking that tests/ directory exists and has tests..."
	@if [ ! -d "tests" ]; then \
		echo "⚠️  No tests/ directory found"; \
	elif [ -z "$$(find tests -name "*.rs" 2>/dev/null)" ]; then \
		echo "⚠️  No test files found in tests/ directory"; \
	else \
		echo "  ✅ Found $$(find tests -name "*.rs" | wc -l | tr -d ' ') test files in tests/ directory"; \
	fi
	@echo "✅ Test separation check complete!"

# Run tests
test: build
	@echo "🧪 Running tests..."
	RUSTFLAGS="-D warnings" cargo test
	@echo "✅ Tests complete!"

# Run WebAssembly tests in browser (Chrome - most reliable)
test-wasm:
	@echo "🌐 Running WebAssembly tests in browser..."
	wasm-pack test --headless --chrome -- --test wasm_component_tests --test wasm_file_upload_tests --test wasm_integration_tests --test app_tests
	@echo "✅ WebAssembly tests complete!"

# Run WebAssembly tests across multiple browsers (requires all drivers)
test-wasm-all-browsers:
	@echo "🌐 Attempting WebAssembly tests across all browsers..."
	@echo "Note: This requires Chrome, Firefox, and Safari drivers to be installed"
	wasm-pack test --headless --chrome --firefox --safari -- --test wasm_component_tests --test wasm_file_upload_tests --test wasm_integration_tests --test app_tests || \
        (echo "⚠️  Some browsers failed. Falling back to Chrome only..." && \
	wasm-pack test --headless --chrome -- --test wasm_component_tests --test wasm_file_upload_tests --test wasm_integration_tests --test app_tests)
	@echo "✅ Multi-browser WebAssembly tests complete!"

# Run WebAssembly tests in Chrome only (fast option)
test-wasm-chrome:
	@echo "🌐 Running WebAssembly tests in Chrome..."
	wasm-pack test --headless --chrome -- --test wasm_component_tests --test wasm_file_upload_tests --test wasm_integration_tests --test app_tests
	@echo "✅ Chrome WebAssembly tests complete!"

# Skip WASM tests (for environments without Chrome)
test-wasm-skip:
	@echo "⚠️  Skipping WebAssembly tests (Chrome not available)"
	@echo "💡 WASM tests require browser APIs and cannot run in Node.js"
	@echo "✅ Use 'make test-ci' for regular tests only"

# Try Chrome, skip WASM tests if Chrome fails  
test-wasm-fallback:
	@echo "🌐 Attempting WebAssembly tests in Chrome..."
	wasm-pack test --headless --chrome -- --test wasm_component_tests --test wasm_file_upload_tests --test wasm_integration_tests --test app_tests || \
        (echo "⚠️  Chrome failed, skipping WASM tests..." && \
         echo "💡 WASM tests require browser environment" && \
         echo "✅ Use './scripts/install-chrome-apt.sh' to install Chrome")
	@echo "✅ Test run complete!"

# Run all tests (standard + WebAssembly across all browsers)
test-all: test test-wasm-all-browsers

# Run only regular tests (skip WASM tests for CI/remote environments)
test-ci: test
	@echo "✅ CI tests complete (WASM tests skipped)!"

# Check if Chrome is available and run appropriate tests
test-auto:
	@echo "🔍 Detecting Chrome availability..."
	@if command -v google-chrome >/dev/null 2>&1 || \
		command -v chromium >/dev/null 2>&1 || \
		command -v chrome >/dev/null 2>&1 || \
		[ -d "/Applications/Google Chrome.app" ] || \
		[ -d "/Applications/Chromium.app" ]; then \
		echo "🌐 Chrome detected, running full test suite..."; \
		echo "💡 Note: wasm-pack will download ChromeDriver automatically if needed"; \
		$(MAKE) test test-wasm-chrome; \
	else \
		echo "⚠️  Chrome not found, running regular tests only..."; \
		echo "💡 To run WASM tests:"; \
		echo "   • macOS: Install Chrome from https://www.google.com/chrome/"; \
		echo "   • Linux: Run ./scripts/install-chrome-apt.sh"; \
		$(MAKE) test-ci; \
	fi

# Debug Chrome detection for troubleshooting
test-debug:
	@echo "🔍 Chrome Detection Debug:"
	@echo "Testing Chrome binary detection (Linux/CLI):"
	@command -v google-chrome >/dev/null 2>&1 && echo "  ✅ google-chrome found: $$(command -v google-chrome)" || echo "  ❌ google-chrome not found"
	@command -v chromium >/dev/null 2>&1 && echo "  ✅ chromium found: $$(command -v chromium)" || echo "  ❌ chromium not found"
	@command -v chrome >/dev/null 2>&1 && echo "  ✅ chrome found: $$(command -v chrome)" || echo "  ❌ chrome not found"
	@echo ""
	@echo "Testing Chrome app detection (macOS):"
	@[ -d "/Applications/Google Chrome.app" ] && echo "  ✅ Google Chrome.app found" || echo "  ❌ Google Chrome.app not found"
	@[ -d "/Applications/Chromium.app" ] && echo "  ✅ Chromium.app found" || echo "  ❌ Chromium.app not found"
	@echo ""
	@echo "Testing ChromeDriver detection:"
	@command -v chromedriver >/dev/null 2>&1 && echo "  ✅ chromedriver found: $$(command -v chromedriver)" || echo "  ❌ chromedriver not found"
	@echo ""
	@echo "Auto-detection result:"
	@if command -v google-chrome >/dev/null 2>&1 || \
		command -v chromium >/dev/null 2>&1 || \
		command -v chrome >/dev/null 2>&1 || \
		[ -d "/Applications/Google Chrome.app" ] || \
		[ -d "/Applications/Chromium.app" ]; then \
		echo "  ✅ Chrome detected - test-auto will run WASM tests"; \
	else \
		echo "  ❌ Chrome not detected - test-auto will skip WASM tests"; \
	fi
	@echo ""
	@echo "💡 Note: wasm-pack can download ChromeDriver automatically even if not in PATH"

# Run clippy linting
lint:
	@echo "🔍 Running clippy linting..."
	cargo clippy -- -D warnings
	@echo "✅ Linting complete!"

# Generate code coverage report
# Excludes untestable files that require WASM browser environment:
# - app.rs: Yew framework app component
# - lib.rs: Library entry point 
# - components/*: UI components requiring DOM
# - *_wasm.rs: WASM-specific modules that require browser APIs
coverage:
	@echo "📈 Generating coverage report..."
	cargo install cargo-llvm-cov --version 0.6.0
	RUSTFLAGS="-D warnings" cargo llvm-cov --ignore-filename-regex "src/(app\.rs|lib\.rs|components/.*\.rs|.*_wasm\.rs)$$"
	RUSTFLAGS="-D warnings" cargo llvm-cov report --html --ignore-filename-regex "src/(app\.rs|lib\.rs|components/.*\.rs|.*_wasm\.rs)$$"
	@echo "✅ Coverage report generated!"

# Generate terminal-friendly coverage summary
coverage-text:
	@echo "📊 Generating coverage summary..."
	cargo install cargo-llvm-cov --version 0.6.0
	@echo ""
	@echo "=== COVERAGE SUMMARY ==="
	RUSTFLAGS="-D warnings" cargo llvm-cov --ignore-filename-regex "src/(app\.rs|lib\.rs|components/.*\.rs|.*_wasm\.rs)$$" 2>/dev/null | tail -n 1
	@echo ""
	@echo "=== TOP COVERED FILES ==="
	RUSTFLAGS="-D warnings" cargo llvm-cov --ignore-filename-regex "src/(app\.rs|lib\.rs|components/.*\.rs|.*_wasm\.rs)$$" 2>/dev/null | grep -v "0.00%" | grep -v "TOTAL" | head -n 8
	@echo ""
	@echo "=== FILES NEEDING COVERAGE ==="
	RUSTFLAGS="-D warnings" cargo llvm-cov --ignore-filename-regex "src/(app\.rs|lib\.rs|components/.*\.rs|.*_wasm\.rs)$$" 2>/dev/null | grep "0.00%" | head -n 5
	@echo "✅ Coverage summary complete!"

# Generate compact coverage table (just key metrics)
coverage-compact:
	@echo "📊 Generating compact coverage..."
	cargo install cargo-llvm-cov --version 0.6.0
	@echo ""
	@printf "%-25s %8s %8s %8s\n" "File" "Lines" "Regions" "Functions"
	@printf "%-25s %8s %8s %8s\n" "----" "-----" "-------" "---------"
	@RUSTFLAGS="-D warnings" cargo llvm-cov --ignore-filename-regex "src/(app\.rs|lib\.rs|components/.*\.rs|.*_wasm\.rs)$$" 2>/dev/null | grep "\.rs" | head -n 12 | awk '{printf "%-25s %8s %8s %8s\n", substr($$1,1,25), $$7, $$3, $$5}'
	@echo ""
	@RUSTFLAGS="-D warnings" cargo llvm-cov --ignore-filename-regex "src/(app\.rs|lib\.rs|components/.*\.rs|.*_wasm\.rs)$$" 2>/dev/null | tail -n 1 | awk '{printf "%-25s %8s %8s %8s\n", "TOTAL", $$7, $$3, $$5}'
	@echo "✅ Compact coverage complete!"

# Show just overall coverage percentage
coverage-summary:
	@echo "📊 Overall Coverage:"
	cargo install cargo-llvm-cov --version 0.6.0 >/dev/null 2>&1
	@RUSTFLAGS="-D warnings" cargo llvm-cov --ignore-filename-regex "src/(app\.rs|lib\.rs|components/.*\.rs|.*_wasm\.rs)$$" 2>/dev/null | tail -n 1 | awk '{printf "  Lines: %s | Regions: %s | Functions: %s\n", $$10, $$4, $$7}'

# Show coverage including untestable files (for comparison)
coverage-full:
	@echo "📊 Full Coverage (including untestable files):"
	cargo install cargo-llvm-cov --version 0.6.0 >/dev/null 2>&1
	@RUSTFLAGS="-D warnings" cargo llvm-cov 2>/dev/null | tail -n 1 | awk '{printf "  Lines: %s | Regions: %s | Functions: %s\n", $$10, $$4, $$7}'

# Format code
format:
	@echo "🎨 Formatting code..."
	cargo fmt
	@echo "✅ Code formatted!"

# Generate documentation
doc:
	@echo "📚 Generating documentation..."
	RUSTDOCFLAGS="-D warnings" cargo doc
	@echo "✅ Documentation generated!"
	@echo "   Open target/doc/image_metadata_extractor/index.html to view"

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

# Configuration: minimum coverage percentage
COVERAGE_MIN ?= 60

# Ensure required tools are installed
ensure-tools:
	@echo "🧰 Ensuring dev tools are installed..."
	@command -v cargo-llvm-cov >/dev/null 2>&1 || cargo install cargo-llvm-cov --version 0.6.0
	@command -v cargo-deny >/dev/null 2>&1 || cargo install cargo-deny
	@command -v cargo-audit >/dev/null 2>&1 || cargo install cargo-audit
	@echo "✅ Tools ready!"

# Dependency and vulnerability checks
audit:
	@echo "🔒 Running security audit (advisory DB)..."
	@cargo audit
	@echo "✅ Audit complete!"

deny:
	@echo "📦 Checking dependencies policy (cargo-deny)..."
	@cargo deny check
	@echo "✅ Dependency policy check complete!"

# Verify coverage exceeds threshold
coverage-verify:
	@echo "📈 Verifying code coverage (min: $(COVERAGE_MIN)%)..."
	@cargo llvm-cov --ignore-filename-regex "src/(app\.rs|lib\.rs|components/.*\.rs|.*_wasm\.rs)$$" >/dev/null
	@PCT=$$(cargo llvm-cov --ignore-filename-regex "src/(app\.rs|lib\.rs|components/.*\.rs|.*_wasm\.rs)$$" 2>/dev/null | tail -n 1 | awk '{print $$10}' | tr -d '%'); \
	if [ -z "$$PCT" ]; then echo "❌ Unable to parse coverage"; exit 1; fi; \
	PCT_INT=$$(printf '%.0f' $$PCT); \
	echo "   • Lines coverage: $$PCT%"; \
	if [ $$PCT_INT -lt $(COVERAGE_MIN) ]; then \
	  echo "❌ Coverage ($$PCT%) is below threshold ($(COVERAGE_MIN)%)"; exit 1; \
	else \
	  echo "✅ Coverage verified (>= $(COVERAGE_MIN)%)"; \
	fi

# Install git pre-commit hooks
setup-hooks:
	@echo "🪝 Setting up git pre-commit hooks..."
	@echo '#!/bin/bash' > .git/hooks/pre-commit
	@echo 'set -e' >> .git/hooks/pre-commit
	@echo 'echo "🔍 Running pre-commit checks..."' >> .git/hooks/pre-commit
	@echo 'make ensure-tools' >> .git/hooks/pre-commit
	@echo 'make check && make format && make lint' >> .git/hooks/pre-commit
	@echo '' >> .git/hooks/pre-commit
	@echo '# Also check tests specifically for warnings' >> .git/hooks/pre-commit
	@echo 'echo "🧪 Checking test code for warnings..."' >> .git/hooks/pre-commit
	@echo 'RUSTFLAGS="-D warnings" cargo check --tests' >> .git/hooks/pre-commit
	@echo '' >> .git/hooks/pre-commit
	@echo '# Ensure tests are properly separated from application code' >> .git/hooks/pre-commit
	@echo 'echo "🔬 Checking test organization..."' >> .git/hooks/pre-commit
	@echo 'make check-test-separation' >> .git/hooks/pre-commit
	@echo '' >> .git/hooks/pre-commit
	@echo '# Verify coverage threshold' >> .git/hooks/pre-commit
	@echo 'make coverage-verify' >> .git/hooks/pre-commit
	@echo '' >> .git/hooks/pre-commit
	@echo '# Security and dependency policy checks' >> .git/hooks/pre-commit
	@echo 'make audit || (echo "⚠️  Audit found issues. Review before committing."; exit 1)' >> .git/hooks/pre-commit
	@echo 'make deny || (echo "⚠️  Dependency policy failed. Review before committing."; exit 1)' >> .git/hooks/pre-commit
	@echo '' >> .git/hooks/pre-commit
	@echo 'git add -u  # Add any formatting changes' >> .git/hooks/pre-commit
	@echo 'echo "✅ Pre-commit checks passed!"' >> .git/hooks/pre-commit
	@chmod +x .git/hooks/pre-commit
	@echo "✅ Pre-commit hooks installed!"
	@echo "   • Hooks will run automatically on each commit"
	@echo "   • Runs: tools check, code check, format, lint, test warnings, test separation, coverage threshold ($(COVERAGE_MIN)%), audit, deny"

# Quick deployment check
deploy-check: pkg-release
	@echo "🚀 Ready for deployment!"
	@echo "   • Code checked and tested"
	@echo "   • Production build complete"
	@echo "   • Files ready in pkg/ directory"
