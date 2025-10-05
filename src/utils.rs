//! Miscellaneous helpers for encoding, formatting and downloading data.
//!
//! This module re-exports functionality from both the testable core module and WASM-specific module.

// Re-export testable core functionality
pub use crate::utils_core::{base64_encode_native, format_file_size};

// Re-export WASM-specific functionality
pub use crate::utils_wasm::{
    copy_to_clipboard, download_binary_file, download_binary_file_with_picker, download_file,
    download_file_with_picker,
};
