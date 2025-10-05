//! Functions for reading EXIF metadata and converting uploaded files into
//! structured [`ImageData`].
//!
//! This module re-exports functionality from both the testable core module and WASM-specific module.

// Re-export testable core functionality
pub use crate::exif_core::{
    determine_mime_type, extract_exif_data, is_supported_mime_type, parse_gps_coordinate,
};

// Re-export WASM-specific functionality
pub use crate::exif_wasm::{
    create_object_url, file_bytes, get_dimensions, process_blob, process_file,
};
