# Image Metadata Extractor

A browser-based image metadata extraction tool built entirely in Rust using Yew framework.

## Project Overview

This application allows users to upload images and extract comprehensive metadata including:
- EXIF data (camera settings, timestamps, etc.)
- GPS coordinates (if present)
- Image dimensions and technical details
- File size and format information
- Any additional embedded metadata

**Key Features:**
- Runs completely in the browser (no server required)
- Client-side processing for privacy
- Fast performance via WebAssembly
- Support for various image formats

## Technology Choices

### Why Rust?
- **Performance**: Compiles to efficient WebAssembly for fast image processing
- **Safety**: Memory-safe binary data parsing
- **Ecosystem**: Rich crates for image processing (`image`, `kamadak-exif`)
- **Single Language**: Consistent development experience across entire application

### Why Yew?
- **Mature Framework**: Stable API with extensive documentation
- **React-like**: Familiar component-based architecture
- **Battle-tested**: Proven in production applications
- **Rich Ecosystem**: Good selection of components and examples
- **File Upload Support**: Well-documented patterns for handling file inputs

### Architecture
- **Frontend**: Yew framework compiled to WebAssembly
- **Image Processing**: Rust crates for metadata extraction
- **Deployment**: Static files served from any web server

## Development Setup

### Prerequisites

- **Rust**: Install via [rustup.rs](https://rustup.rs/)
- **wasm-pack**: Install with `cargo install wasm-pack`
- **Basic HTTP Server**: Any static file server (Python's `http.server`, Node's `serve`, etc.)

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
   # Option 1: Python
   python -m http.server 8000
   
   # Option 2: Node.js (install with: npm install -g serve)
   serve -s . -p 8000
   
   # Option 3: Rust (install with: cargo install basic-http-server)
   basic-http-server -a 0.0.0.0:8000
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
│   ├── lib.rs          # Main application entry point
│   └── components/     # Yew components (to be created)
├── pkg/                # Generated WebAssembly files (git-ignored)
├── Cargo.toml          # Rust dependencies and configuration
└── README.md           # This file
```

### Debugging

- Use browser developer tools for runtime debugging
- `console.log!()` macro from `web-sys` for logging
- `wee_alloc` can be added for smaller WASM binary sizes

## Supported Formats

(To be determined based on crate capabilities)