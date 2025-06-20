# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Repository Overview

This is a Rust-based image metadata extraction tool that runs entirely in the browser using WebAssembly. The application is built with the Yew framework and provides a complete client-side solution for extracting EXIF data, GPS coordinates, and technical information from uploaded images with export capabilities in multiple formats.

## Development Commands

### Pre-commit Hooks (Recommended Setup)
For consistent code quality, install git pre-commit hooks:
```bash
# Easy setup using Makefile
make setup-hooks
```

This installs a git hook that automatically runs `make check`, `make format`, and `make lint` on every commit, ensuring code quality and consistent formatting. The hooks will prevent commits if any checks fail.

### Makefile Commands (Recommended)
- **Default build**: `make` or `make build`
- **Production build**: `make build-release`
- **Start dev server**: `make serve`
- **Check code**: `make check`
- **Run tests**: `make test`
- **Format code**: `make format`
- **Lint code**: `make lint`
- **Clean artifacts**: `make clean`
- **Full dev workflow**: `make dev` (check + format + lint + build)
- **Production workflow**: `make prod` (check + test + lint + format + build-release)
- **Show all commands**: `make help`

### Direct Commands (Alternative)
- **Development build**: `wasm-pack build --target web --dev`
- **Production build**: `wasm-pack build --target web --release`
- **Check code**: `cargo check`
- **Clean build artifacts**: `cargo clean`
- **Run tests**: `cargo test`
- **Format code**: `cargo fmt`
- **Lint code**: `cargo clippy`

### Local Development Server
The Makefile automatically detects available servers:
- **Using Makefile**: `make serve` (tries npx, python3, python in order)
- **Manual options**:
  - **Node.js**: `npx -y serve -s . -p 8000`
  - **Python**: `python -m http.server 8000`
  - **Rust**: `cargo install basic-http-server && basic-http-server -a 0.0.0.0:8000`

After starting the server, open `http://localhost:8000` in your browser.

## Architecture

### Core Technologies
- **Rust + WebAssembly**: Main application logic compiled to WASM for browser execution
- **Yew Framework**: React-like component framework for Rust web applications
- **kamadak-exif**: EXIF metadata parsing library
- **image crate**: Image format support and dimension extraction
- **web-sys**: Browser API bindings for file handling and DOM manipulation

### Application Structure
The entire application is contained in `src/lib.rs` with a single-file architecture:

- **App Component** (`src/lib.rs:24-229`): Main Yew component handling UI state and user interactions
- **File Processing** (`src/lib.rs:231-259`): Async file upload and metadata extraction pipeline
- **EXIF Extraction** (`src/lib.rs:272-354`): Core metadata parsing with GPS coordinate handling
- **Export Functions** (`src/lib.rs:415-489`): Multi-format data export (JSON, CSV, TXT)
- **Utility Functions**: Base64 encoding, file size formatting, browser download handling

### Key Data Structures
- **ImageData** (`src/lib.rs:12-22`): Central data structure containing file info, dimensions, EXIF data, and GPS coordinates
- **HashMap<String, String>**: EXIF data storage for flexible metadata handling
- **GPS Coordinates**: Optional (latitude, longitude) tuple with proper N/S/E/W reference handling

### Browser Integration
- Uses `web-sys` for DOM manipulation and file API access
- Implements drag-and-drop file upload with async processing
- Creates downloadable files using Blob API and temporary object URLs
- Handles image display with modal expansion and thumbnail views

## Build Output
The build process generates the `pkg/` directory containing:
- `image_metadata_extractor.js`: JavaScript bindings
- `image_metadata_extractor_bg.wasm`: Compiled WebAssembly module
- `image_metadata_extractor.d.ts`: TypeScript type definitions
- `package.json`: NPM package metadata

## Supported Features
- **Image Formats**: JPEG (full EXIF), PNG, GIF, WebP
- **Metadata Types**: Camera settings, timestamps, GPS location, technical specifications
- **Export Formats**: JSON (structured), CSV (spreadsheet), TXT (human-readable)
- **UI Features**: Thumbnail/modal image viewing, drag-and-drop upload, one-click exports