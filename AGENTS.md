# AGENTS.md - Development Guide

Welcome to the Image Metadata Extractor repository. This file summarizes the key information for developers and AI assistants contributing to this project.

## Repository Overview
- **Source code**: `src/` contains the Yew front end and core logic implemented in Rust and WebAssembly.
- **Tests**: `tests/` holds WebAssembly tests using `wasm-bindgen-test`.
- **Build scripts**: `Makefile` provides commands for building, serving and checking code.
- **Documentation**: `README.md` and `CLAUDE.md` describe features and workflows.

The application is a browser-based image metadata extractor with privacy-safe image cleaning, advanced export formats, and responsive UI.

## Shared Vision
This project delivers a fully client-side tool to inspect and clean image metadata. Contributions should maintain privacy by avoiding server-side dependencies and keep performance high for large images.

## Local Workflow
1. **Install dependencies**: ensure Rust and `wasm-pack` are installed.
2. **Setup hooks**: `make setup-hooks` installs pre-commit hooks that run `make check`, `make test`, `make format` and `make lint`.
3. **Check and build**:
   ```bash
   make check     # cargo check
   make format    # cargo fmt
   make lint      # cargo clippy
   make build     # development build
   ```
4. **Serve locally**: `make serve` launches a local server at http://localhost:8000.
5. **Run tests**: `make test` for standard Rust tests and `make test-wasm` for WebAssembly tests.
6. Use `make dev` for the full workflow or `make prod` before submitting pull requests.

## Testing Guidelines
- Cover new functionality and edge cases using `wasm-bindgen-test`.
- WebAssembly tests run in the browser; use `make test-wasm` or `make test-all`.
- Maintain existing pre-commit quality checks so tests pass before each commit.

## Style Notes
- Write comments as full sentences ending with a period.
- Use meaningful names for variables, functions and components.
- Keep functions small and focused on a single responsibility.
- Follow the modular component architecture in `src/components/`.

## Commit Message Format
Use the conventional commit style:
```
type(scope): description
```
Examples:
```
feat(app): add new metadata export option
fix(cleaner): prevent panic on malformed input
```

## Pull Request Expectations
Include a concise summary, test plan and any related issues. Ensure:
- [ ] New tests cover added functionality.
- [ ] Documentation is updated as needed.
- [ ] `make check`, `make test`, `make format` and `make lint` pass.
- [ ] Commit messages follow the conventional format.

## What Reviewers Look For
- Proper test coverage with both Rust and WebAssembly tests.
- Consistent formatting and clear, maintainable code.
- Updates to documentation when public behavior changes.
- Consideration for privacy and performance in the browser.

## Architecture Guidelines
- Maintain separation of concerns between components and core logic.
- Favor composition over inheritance and keep modules focused.
- Use typed props and interfaces for component communication.
- Keep the codebase Rust-only with Yew for the web front end.

## Security Considerations
- Validate all input files and avoid storing user data.
- Never commit secrets or tokens to the repository.
- Follow safe Rust practices to avoid memory issues.

## Performance Guidelines
- Optimize canvas-based image processing for speed.
- Use efficient data structures for metadata storage.
- Keep bundle size small through release builds (`make build-release`).

## Known Limitations
- WebAssembly tests require Chrome (or other browsers) installed locally.
- Large image files may still cause memory pressure on low-end devices.

