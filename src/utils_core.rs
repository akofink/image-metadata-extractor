//! Pure Rust utility functions that can be thoroughly tested in native environment.
//! This module contains all testable utility functionality without browser dependencies.

/// Format a raw byte count into a human readable string.
pub fn format_file_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    format!("{:.1} {}", size, UNITS[unit_index])
}

/// Convert a byte slice into a base64 encoded string using native Rust.
/// This is a fallback implementation for testing environments.
pub fn base64_encode_native(bytes: &[u8]) -> String {
    use base64::{Engine as _, engine::general_purpose};
    general_purpose::STANDARD.encode(bytes)
}
