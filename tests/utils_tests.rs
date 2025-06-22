use image_metadata_extractor::utils::format_file_size;

#[cfg(target_arch = "wasm32")]
use image_metadata_extractor::utils::{base64_encode, download_binary_file, download_file};

#[test]
fn test_format_file_size_bytes() {
    assert_eq!(format_file_size(0), "0.0 B");
    assert_eq!(format_file_size(1), "1.0 B");
    assert_eq!(format_file_size(512), "512.0 B");
    assert_eq!(format_file_size(1023), "1023.0 B");
}

#[test]
fn test_format_file_size_kilobytes() {
    assert_eq!(format_file_size(1024), "1.0 KB");
    assert_eq!(format_file_size(1536), "1.5 KB");
    assert_eq!(format_file_size(2048), "2.0 KB");
    assert_eq!(format_file_size(1024 * 1023), "1023.0 KB");
}

#[test]
fn test_format_file_size_megabytes() {
    assert_eq!(format_file_size(1024 * 1024), "1.0 MB");
    assert_eq!(format_file_size(1024 * 1024 + 512 * 1024), "1.5 MB");
    assert_eq!(format_file_size(5 * 1024 * 1024), "5.0 MB");
    assert_eq!(format_file_size(1024 * 1024 * 1023), "1023.0 MB");
}

#[test]
fn test_format_file_size_gigabytes() {
    assert_eq!(format_file_size(1024 * 1024 * 1024), "1.0 GB");
    assert_eq!(format_file_size(2 * 1024 * 1024 * 1024), "2.0 GB");
    assert_eq!(
        format_file_size(1024_u64.pow(3) + 512 * 1024_u64.pow(2)),
        "1.5 GB"
    );
}

#[test]
fn test_format_file_size_max_unit() {
    // Test that we don't go beyond GB
    let huge_size = 2048 * 1024 * 1024 * 1024; // 2TB in bytes
    let result = format_file_size(huge_size);
    assert!(result.ends_with(" GB"));
    assert!(result.starts_with("2048.0"));
}

#[test]
fn test_format_file_size_edge_cases() {
    // Test boundary conditions
    assert_eq!(format_file_size(1024 - 1), "1023.0 B");
    assert_eq!(format_file_size(1024), "1.0 KB");
    assert_eq!(format_file_size(1024 * 1024 - 1), "1024.0 KB");
    assert_eq!(format_file_size(1024 * 1024), "1.0 MB");
    assert_eq!(format_file_size(1024 * 1024 * 1024 - 1), "1024.0 MB");
    assert_eq!(format_file_size(1024 * 1024 * 1024), "1.0 GB");
}

#[test]
fn test_format_file_size_precision() {
    // Test decimal precision
    assert_eq!(format_file_size(1024 + 100), "1.1 KB"); // 1124 bytes
    assert_eq!(format_file_size(1024 + 200), "1.2 KB"); // 1224 bytes
    assert_eq!(format_file_size(1024 + 500), "1.5 KB"); // 1524 bytes
}

// WASM-only tests for browser API functions
#[cfg(target_arch = "wasm32")]
#[test]
fn test_base64_encode() {
    // Test base64 encoding of simple data
    let data = b"hello world";
    let encoded = base64_encode(data);

    // Should be a valid base64 string
    assert!(encoded.len() > 0);
    assert!(
        encoded
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '+' || c == '/' || c == '=')
    );

    // Test with empty data
    let empty_encoded = base64_encode(&[]);
    assert_eq!(empty_encoded, "");
}

// Note: download_file and download_binary_file can't be easily tested
// without a full browser environment as they manipulate the DOM
// and create actual downloads. They would require integration testing
// with a headless browser or similar setup.
