# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Repository Overview

This is a Rust-based image metadata extraction tool that runs entirely in the browser using WebAssembly. The application is built with the Yew framework and provides a complete client-side solution for extracting EXIF data, GPS coordinates, and technical information from uploaded images with advanced export capabilities and privacy-focused image cleaning.

## Development Commands

### Pre-commit Hooks (Recommended Setup)
For consistent code quality, install git pre-commit hooks:
```bash
# Easy setup using Makefile
make setup-hooks
```

This installs a git hook that automatically runs `make check`, `make test`, `make format`, and `make lint` on every commit, ensuring code quality and consistent formatting. The hooks will prevent commits if any checks fail.

### Makefile Commands (Recommended)
- **Default build**: `make` or `make build`
- **Production build**: `make build-release`
- **Start dev server**: `make serve`
- **Setup hooks**: `make setup-hooks`
- **Check code**: `make check`
- **Run tests**: `make test`
- **Run WebAssembly tests**: `make test-wasm`
- **Run all tests**: `make test-all`
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

### Modular Component Architecture
The application uses a modern component-based architecture for maintainability and reusability:

#### **Main Application** (`src/app.rs`)
- Central state management and component coordination
- File upload handling and state transitions
- Layout and mobile-responsive design

#### **Component Modules** (`src/components/`)
- **`file_upload.rs`**: File selection, validation, and processing pipeline
- **`image_display.rs`**: Image thumbnail/modal view, file information display
- **`metadata_display.rs`**: Categorized EXIF data with sorting, filtering, and explanations
- **`image_cleaner.rs`**: Privacy-safe image downloads with format conversion and quality controls
- **`metadata_export.rs`**: Multi-format export (JSON, CSV, TXT) with selective field inclusion

#### **Core Logic Modules** (`src/`)
- **`exif.rs`**: EXIF metadata extraction and GPS coordinate parsing
- **`export.rs`**: CSV and text export generation functions
- **`metadata_info.rs`**: Field explanations, categorization, and help text
- **`types.rs`**: Data structures, filtering logic, and serialization
- **`utils.rs`**: File downloads, size formatting, and utility functions

### Key Data Structures
- **ImageData** (`src/types.rs`): Central data structure containing file info, dimensions, EXIF data, and GPS coordinates
- **HashMap<String, String>**: EXIF data storage for flexible metadata handling
- **GPS Coordinates**: Optional (latitude, longitude) tuple with proper N/S/E/W reference handling
- **Component Props**: Strongly-typed interfaces for component communication

### Browser Integration
- **File API**: Advanced file handling with drag-and-drop support
- **Binary Processing**: Direct byte-stream metadata removal preserving image quality
- **Blob API**: Dynamic file generation for downloads
- **Local Storage**: No data persistence (privacy-first design)
- **Responsive Design**: Mobile-optimized layouts and interactions

## Testing

### Testing Framework
The project uses `wasm-bindgen-test` for WebAssembly component testing:
- **Unit Tests**: Component logic and state management
- **Integration Tests**: Component interactions and data flow  
- **Regression Tests**: Prevent infinite render loops and side effect bugs

### Running Tests
- **Standard Rust tests**: `make test` or `cargo test`
- **WebAssembly tests**: `make test-wasm` (runs in browser via wasm-pack)
- **All tests**: `make test-all` (combines both test suites)

### Test Categories
- **Component Tests**: FileUpload component render loop prevention
- **Callback Tests**: Proper use_effect_with patterns and one-time setup
- **Lifecycle Tests**: Memory safety and component creation/destruction
- **Props Tests**: Component interface and equality implementations

### Pre-commit Testing
The git hooks automatically run tests on every commit:
```bash
make check && make test && make format && make lint
```

## Build Output
The build process generates the `pkg/` directory containing:
- `image_metadata_extractor.js`: JavaScript bindings
- `image_metadata_extractor_bg.wasm`: Compiled WebAssembly module
- `image_metadata_extractor.d.ts`: TypeScript type definitions
- `package.json`: NPM package metadata

## Supported Features

### Image Processing
- **Image Formats**: JPEG (full EXIF), PNG, GIF, WebP, TIFF, HEIF, SVG, PDF
- **Metadata Extraction**: Camera settings, timestamps, GPS location, technical specifications
- **Privacy Cleaning**: Complete metadata removal via binary stream processing (little_exif library)
- **Format Detection**: Automatic MIME type detection and format validation
- **Lossless Processing**: Direct byte-stream manipulation preserves original image quality

### Data Export
- **Export Formats**: JSON (structured), CSV (spreadsheet), TXT (human-readable)
- **Selective Export**: Choose specific metadata fields via checkboxes
- **Smart Filtering**: Include/exclude file info and GPS data independently
- **Auto-naming**: Descriptive filenames based on original image name
- **Select/Deselect All**: Quickly toggle all fields globally or per category
- **Disabled When Empty**: Export buttons remain inactive until a field or option is chosen
- **File Info Toggle**: Control inclusion of filename, size and dimensions
- **Clean JSON Output**: Empty values are omitted from exports

### User Interface
- **Mobile-Optimized**: Responsive design with touch-friendly interactions
- **Component Architecture**: Modular, maintainable codebase
- **Categorized Display**: Organized metadata sections with alphabetical sorting
- **Smart Explanations**: Toggle-able field descriptions and help text
- **Stable Layout**: No content jumping or jarring transitions
- **Accessibility**: Keyboard navigation and screen reader support
- **Auto-select on Upload**: All metadata fields start selected for quick export

### Privacy & Performance
- **Client-Side Only**: No server communication - complete privacy
- **Fast Processing**: Rust + WebAssembly optimization
- **Memory Efficient**: Handles large images without performance issues
- **Instant Results**: Real-time metadata extraction and processing

## Development Guidelines

### Code Quality Standards
- All commits automatically run pre-commit hooks (`make setup-hooks`)
- Code must pass `cargo check`, `cargo fmt`, and `cargo clippy`
- Use Makefile commands for consistent development workflows
- Follow the modular component architecture established in `src/components/`

### Component Design Principles
- **Single Responsibility**: Each component handles one specific concern
- **Props Interface**: Well-defined, typed interfaces for component communication
- **State Management**: Local state in components, shared state in main app
- **Mobile-First**: Responsive design with touch-friendly interactions
- **Accessibility**: Semantic HTML and keyboard navigation support

### Mobile UX Considerations
- **No Nested Scrolling**: Avoid `max-height` with `overflow-y: auto`
- **Natural Page Flow**: Let content expand naturally on mobile
- **Touch Targets**: Minimum 44px touch targets for interactive elements
- **Layout Stability**: Prevent content jumping during state changes
- **Performance**: Optimize for mobile rendering and interaction

### Testing & Quality Assurance
- Use `make dev` for full development workflow
- Test on mobile devices and different screen sizes
- Verify accessibility with screen readers
- Check performance with large image files
- Validate exports in different formats

## Recent Architectural Improvements

### Component Refactoring (Latest)
- Broke down 400+ line monolithic `app.rs` into focused, reusable components
- Implemented proper separation of concerns with typed interfaces
- Improved code maintainability and testing capabilities

### Mobile UX Enhancements (Latest)
- Removed problematic nested scrolling containers
- Added smooth layout transitions and stable content areas
- Improved responsive design for touch interactions
- Enhanced spacing and sizing for mobile devices

### Privacy Features (Latest)
- Binary stream-based image cleaning for complete metadata removal using little_exif library
- Lossless metadata stripping preserving original image quality
- Support for multiple image formats (JPEG, PNG, GIF, WebP, TIFF, HEIF, SVG, PDF)
- Browser-native downloads with cleaned filenames

The codebase maintains high standards through automated tooling, comprehensive pre-commit checks, and a focus on user experience across all device types.