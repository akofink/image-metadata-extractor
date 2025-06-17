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

(To be filled in as project develops)

## Supported Formats

(To be determined based on crate capabilities)