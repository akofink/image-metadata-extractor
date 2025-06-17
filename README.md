# Image Metadata Extractor

A comprehensive browser-based image metadata extraction tool built entirely in Rust using Yew framework.

## Project Overview

This application allows users to upload images and extract comprehensive metadata with advanced visualization and export capabilities:

### 📋 **Metadata Extraction**
- **EXIF data**: Camera settings, timestamps, camera model, lens info, etc.
- **GPS coordinates**: Location data with Google Maps integration
- **Image dimensions**: Width, height, and technical specifications
- **File information**: Size, format, and basic properties
- **Comprehensive coverage**: All standard EXIF tags and values

### 🖼️ **Image Display**
- **Smart thumbnails**: Compact 300x200px display for better page layout
- **Click-to-expand**: Full-screen modal view for detailed inspection
- **Responsive design**: Works seamlessly across different screen sizes

### 📊 **Export Capabilities**
- **JSON format**: Pretty-formatted structured data for developers
- **CSV format**: Spreadsheet-ready with proper escaping for analysis
- **Text format**: Human-readable reports with organized sections
- **Smart downloads**: Browser-native downloads with auto-generated filenames

**Key Features:**
- 🔒 **Complete privacy**: Runs entirely in the browser (no server required)
- ⚡ **High performance**: Fast processing via WebAssembly
- 🌐 **Universal compatibility**: Works in any modern web browser
- 📱 **Responsive design**: Mobile and desktop friendly
- 🎨 **Professional UI**: Clean, intuitive interface with visual hierarchy

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
- **`web-sys`**: Browser API bindings for file handling

### Architecture Benefits
- **🔧 Single Language**: Rust throughout the entire application
- **⚡ Performance**: Near-native speed via WebAssembly
- **🛡️ Safety**: Memory-safe image parsing prevents crashes
- **📦 Small Bundle**: Optimized WASM output
- **🌐 Universal**: Runs in any modern web browser

### Why This Stack?
- **Rust + WASM**: Combines safety with performance for binary data processing
- **Client-Side**: Complete privacy - no server communication required
- **Modern Web**: Leverages cutting-edge web technologies
- **Developer Experience**: Type safety and excellent tooling throughout

## Development Setup

### Prerequisites

- **Rust**: Install via [rustup.rs](https://rustup.rs/)
- **wasm-pack**: Install with `cargo install wasm-pack`
- **HTTP Server**: Node.js for `npx` (recommended) or Python for `http.server`

### Getting Started

1. **Clone the repository:**
   ```bash
   git clone <repository-url>
   cd image-metadata
   ```

2. **Install dependencies:**
   ```bash
   cargo check
   ```

3. **Build for development:**
   ```bash
   wasm-pack build --target web --dev
   ```

4. **Build for production:**
   ```bash
   wasm-pack build --target web --release
   ```

5. **Serve locally:**
   ```bash
   # Option 1: Node.js (npx - installs automatically)
   npx -y serve -s . -p 8000
   
   # Option 2: Python (usually pre-installed)
   python -m http.server 8000
   
   # Option 3: Rust (install once)
   cargo install basic-http-server && basic-http-server -a 0.0.0.0:8000
   ```

6. **Open in browser:**
   Navigate to `http://localhost:8000`

### Development Commands

- **Check code:** `cargo check`
- **Run tests:** `cargo test`
- **Format code:** `cargo fmt`
- **Lint code:** `cargo clippy`
- **Clean build artifacts:** `cargo clean`

### Project Structure

```
├── src/
│   └── lib.rs          # Complete application with EXIF extraction and UI
├── pkg/                # Generated WebAssembly files (git-ignored)
├── index.html          # Web application entry point
├── Cargo.toml          # Rust dependencies and configuration
└── README.md           # This documentation
```

## Quick Start

1. **Clone and build:**
   ```bash
   git clone <repository-url>
   cd image-metadata
   wasm-pack build --target web --dev
   ```

2. **Serve locally:**
   ```bash
   npx -y serve -s . -p 8000
   ```

3. **Open browser:**
   Navigate to `http://localhost:8000`

4. **Upload an image:**
   - Click the file input or drag & drop an image
   - View extracted metadata instantly
   - Click thumbnail to expand image
   - Export data in your preferred format

### Debugging

- Use browser developer tools for runtime debugging
- `console.log!()` macro from `web-sys` for logging
- `wee_alloc` can be added for smaller WASM binary sizes

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

## Features in Detail

### EXIF Metadata Support
- **Camera Information**: Make, model, lens details
- **Shooting Parameters**: ISO, aperture, shutter speed, focal length
- **Timestamps**: Creation date, modification date
- **GPS Location**: Latitude, longitude with direction references
- **Technical Details**: Color space, orientation, resolution
- **Software Information**: Camera firmware, editing software

### User Interface
- **Drag & Drop**: Easy file upload interface
- **Live Preview**: Immediate thumbnail display
- **Organized Display**: Categorized metadata sections
- **Export Panel**: One-click downloads in multiple formats
- **Modal Gallery**: Full-screen image viewing

### Performance & Privacy
- **Client-Side Only**: No data leaves your browser
- **Fast Processing**: Rust + WebAssembly optimization
- **Memory Efficient**: Handles large images smoothly
- **Instant Results**: Real-time metadata extraction