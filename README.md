# Image Metadata Extractor

A comprehensive browser-based image metadata extraction tool built entirely in Rust using Yew framework. See [ROADMAP.md](./ROADMAP.md) for the planned feature roadmap.

## Project Overview

This application allows users to upload images and extract comprehensive metadata with advanced visualization, export capabilities, and privacy-focused image cleaning:

### 📋 **Metadata Extraction**
- **EXIF data**: Camera settings, timestamps, camera model, lens info, etc.
- **GPS coordinates**: Location data with Google Maps, Apple Maps, and OpenStreetMap links
- **Image dimensions**: Width, height, and technical specifications
- **File information**: Size, format, and basic properties
- **Categorized display**: Organized by Camera Settings, GPS, Technical, etc.
- **Comprehensive coverage**: All standard EXIF tags and values
- **Smart explanations**: Toggle-able descriptions for each metadata field
- **Auto-select on upload**: All metadata fields are selected by default
- **Batch processing**: Upload multiple files with progress tracking and navigation

### 🖼️ **Image Display**
- **Smart thumbnails**: Compact 300x200px display for better page layout
- **Click-to-expand**: Full-screen modal view for detailed inspection
- **Responsive design**: Works seamlessly across different screen sizes

### 🧹 **Privacy-Safe Image Cleaning**
- **Metadata removal**: Strip ALL EXIF data, GPS, and camera information
- **Format conversion**: Convert between JPEG and PNG regardless of input format
- **Quality control**: Adjustable JPEG compression (30%-100%)
- **Binary metadata removal**: Lossless cleaning for JPEG, PNG, WebP, GIF and more
- **One-click download**: Privacy-safe images with zero metadata

### 📊 **Advanced Export Capabilities**
- **Selective export**: Choose exactly which metadata fields to include
- **Multiple formats**: JSON, CSV, and human-readable text
- **File info options**: Toggle filename, size, and dimensions
- **Smart filtering**: Visual checkboxes for granular control
- **Auto-generated filenames**: Convenient downloads with descriptive names
- **Select/deselect all**: Quickly toggle entire metadata sets
- **Disabled export when empty**: Buttons stay inactive until something is selected

### 📱 **Enhanced User Experience**
- **Organized metadata**: Alphabetically sorted categories and fields
- **Stable rendering**: No more jumping sections during UI updates
- **Modular components**: Clean, maintainable component architecture
- **Professional styling**: Color-coded sections with intuitive icons
- **Dark mode support**: Automatic system theme detection and manual toggle
- **Batch navigation**: Previous/Next buttons for switching between uploaded files

**Key Features:**
- 🔒 **Complete privacy**: Runs entirely in the browser (no server required)
- 🧹 **Metadata cleaning**: Remove all tracking data for privacy
- ⚡ **High performance**: Fast processing via WebAssembly
- 🌐 **Universal compatibility**: Works in any modern web browser
- 📱 **Responsive design**: Mobile and desktop friendly
- 🎨 **Professional UI**: Clean, intuitive interface with visual hierarchy
- 🔧 **Format flexibility**: Convert between image formats while cleaning

## Technology Stack

### Core Technologies
- **🦀 Rust**: Memory-safe systems programming language
- **🕸️ WebAssembly**: High-performance web execution
- **⚛️ Yew**: Modern React-like framework for Rust
- **📦 wasm-pack**: Rust-generated WebAssembly packaging

### Key Dependencies
- **`kamadak-exif`**: Comprehensive EXIF metadata parsing
- **`image`**: Image format support and dimension extraction
- **`serde`**: Serialization for JSON export functionality
- **`web-sys`**: Browser API bindings for file handling and canvas operations

### Architecture Benefits
- **🔧 Single Language**: Rust throughout the entire application
- **⚡ Performance**: Near-native speed via WebAssembly
- **🛡️ Safety**: Memory-safe image parsing prevents crashes
- **📦 Small Bundle**: Optimized WASM output
- **🌐 Universal**: Runs in any modern web browser
- **🧩 Modular**: Component-based architecture for maintainability

### Why This Stack?
- **Rust + WASM**: Combines safety with performance for binary data processing
- **Client-Side**: Complete privacy - no server communication required
- **Modern Web**: Leverages cutting-edge web technologies
- **Developer Experience**: Type safety and excellent tooling throughout

## Development Setup

### Prerequisites

- **Rust**: Install via [rustup.rs](https://rustup.rs/)
- **Rustfmt & Clippy**: Automatically installed via `rust-toolchain.toml` on first `cargo` run
- **wasm-pack**: Install with `cargo install wasm-pack`
- **HTTP Server**: Node.js for `npx` (recommended) or Python for `http.server`

### Getting Started

1. **Clone the repository:**
   ```bash
   git clone https://github.com/akofink/image-metadata-extractor.git
   cd image-metadata-extractor
   ```

2. **Set up development environment:**
   ```bash
   # Install pre-commit hooks for code quality
   make setup-hooks

   # Check dependencies (installs clippy/rustfmt on first run)
   make check
   ```

3. **Build and serve:**
   ```bash
   # Development build and serve
   make && make serve
   
   # Or production build
   make build-release
   ```

4. **Open in browser:**
   Navigate to `http://localhost:8000`

### Development Commands (Makefile)

The project includes a comprehensive Makefile for streamlined development:

#### **Essential Commands**
- **`make` or `make build`** - Development build (fast, with debug info)
- **`make build-release`** - Production build (optimized, smaller size)
- **`make serve`** - Start local development server on port 8000
- **`make setup-hooks`** - Install git pre-commit hooks for code quality

#### **Code Quality**
- **`make check`** - Check code compilation
- **`make test`** - Run all tests
- **`make coverage`** - Generate HTML coverage report
- **`make format`** - Format code with cargo fmt
- **`make lint`** - Run clippy linting
- **`make clean`** - Clean build artifacts

#### **Workflow Commands**
- **`make dev`** - Full development workflow (check + format + lint + build)
- **`make prod`** - Production workflow (check + test + lint + format + build-release)
- **`make help`** - Show all available commands

### Pre-commit Hooks and Quality Gate

For consistent code quality, install git pre-commit hooks:

```bash
make setup-hooks
```

This automatically runs code checks, formatting, linting, test separation checks, an enforced coverage threshold (configurable via COVERAGE_MIN, default 60%), and security/dependency policy checks (cargo-audit, cargo-deny) on every commit.

In CI, the Quality Gate workflow repeats these checks for pull requests, ensuring a consistent bar before merge.

### Project Structure

```
├── src/
│   ├── app.rs              # Main application component
│   ├── components/         # Modular UI components
│   │   ├── file_upload.rs     # File selection and processing
│   │   ├── image_display.rs   # Image viewing and file info
│   │   ├── metadata_display.rs # EXIF data with categorization
│   │   ├── image_cleaner.rs   # Privacy-safe image downloads
│   │   └── metadata_export.rs # Export functionality
│   ├── exif.rs            # EXIF metadata extraction logic
│   ├── export.rs          # CSV and text export functions
│   ├── image_cleaner.rs   # Image metadata removal via canvas
│   ├── metadata_info.rs   # Field explanations and categorization
│   ├── types.rs           # Data structures and filtering
│   ├── utils.rs           # File downloads and utilities
│   └── lib.rs             # WebAssembly exports and entry point
├── pkg/                   # Generated WebAssembly files (git-ignored)
├── index.html             # Web application entry point
├── Makefile              # Development commands and workflows
├── Cargo.toml            # Rust dependencies and configuration
├── CLAUDE.md             # Development documentation for AI assistance
└── README.md             # This documentation
```

## Quick Start

1. **Clone and setup:**
   ```bash
   git clone https://github.com/akofink/image-metadata-extractor.git
   cd image-metadata-extractor
   make setup-hooks  # Install code quality hooks
   make check        # Verify toolchain and dependencies
   ```

2. **Build and serve:**
   ```bash
   make && make serve
   ```

3. **Open browser:**
   Navigate to `http://localhost:8000`

4. **Upload and explore:**
   - Upload an image using the file input
   - View extracted metadata organized by category
   - Toggle field explanations for detailed information
   - Select specific metadata fields for export
   - Download privacy-safe cleaned images
   - Export metadata in JSON, CSV, or text format

## Supported Formats

### Image Formats
- **JPEG/JPG**: Full EXIF support including GPS data
- **PNG**: Basic metadata and dimensions
- **GIF**: Dimensions and file information
- **WebP**: Modern format with metadata support

### Export Formats
- **JSON**: Structured data with nested objects for complex metadata
- **CSV**: Tabular format perfect for spreadsheet analysis
- **TXT**: Human-readable reports with organized sections

### Cleaned Image Formats
- **JPEG**: Adjustable quality (30%-100%) for size optimization
- **PNG**: Lossless format for maximum quality retention
- **WebP**: Modern format for smaller file sizes
- **GIF**: Basic format for simple animations or compatibility
- **Format conversion**: Input JPEG/PNG/GIF/WebP → Output JPEG/PNG/GIF/WebP

## Features in Detail

### EXIF Metadata Support
- **Camera Information**: Make, model, lens details
- **Shooting Parameters**: ISO, aperture, shutter speed, focal length
- **Timestamps**: Creation date, modification date
- **GPS Location**: Latitude, longitude with Google Maps, Apple Maps, and OpenStreetMap links
- **Technical Details**: Color space, orientation, resolution
- **Software Information**: Camera firmware, editing software
- **Organized categories**: Alphabetically sorted for consistent display

### Privacy-Safe Image Cleaning
- **Complete metadata removal**: Strips ALL EXIF data, GPS coordinates, and camera information
- **Canvas-based processing**: Uses HTML5 Canvas API for reliable metadata removal
- **Format flexibility**: Convert between JPEG and PNG during cleaning
- **Quality control**: Adjustable JPEG compression for size vs. quality balance
- **Binary metadata removal**: Lossless cleaning for JPEG, PNG, WebP, GIF and more
- **One-click downloads**: Browser-native downloads with cleaned filenames

### Advanced Export System
- **Granular selection**: Choose individual metadata fields with checkboxes
- **Smart filtering**: Include/exclude file info and GPS data separately
- **Multiple formats**: JSON for developers, CSV for analysis, TXT for reports
- **Real-time preview**: See field count and selection status
- **Auto-generated filenames**: Descriptive names based on original filename
- **Select/deselect all**: Toggle all fields globally or by category
- **Disabled export when empty**: Buttons enable only when something is selected

### User Interface Excellence
- **Component architecture**: Modular, maintainable codebase
- **Responsive design**: Works seamlessly on mobile and desktop
- **Professional styling**: Color-coded sections with intuitive icons
- **Stable rendering**: Alphabetically sorted categories prevent UI jumping
- **Accessibility**: Keyboard navigation and screen reader friendly

### Performance & Privacy
- **Client-Side Only**: No data leaves your browser - complete privacy
- **Fast Processing**: Rust + WebAssembly optimization for near-native speed
- **Memory Efficient**: Handles large images smoothly without crashes
- **Instant Results**: Real-time metadata extraction and processing
- **Canvas optimization**: Efficient image processing for metadata removal

## Contributing

### Code Quality Standards
- All commits automatically run pre-commit hooks (install with `make setup-hooks`)
- Code must pass `cargo check`, `cargo fmt`, and `cargo clippy`
- Use the Makefile commands for consistent development workflows
- CI publishes an HTML coverage report for each pull request
- Follow the modular component architecture established in `src/components/`

### Development Workflow
1. Install pre-commit hooks: `make setup-hooks`
2. Use `make dev` for the full development workflow
3. Use `make prod` before submitting pull requests
4. All changes are automatically formatted and linted on commit

The project maintains high code quality standards through automated tooling and comprehensive pre-commit checks.