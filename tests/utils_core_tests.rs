use image_metadata_extractor::utils_core::{base64_encode_native, format_file_size};

#[test]
fn test_format_file_size() {
    assert_eq!(format_file_size(0), "0.0 B");
    assert_eq!(format_file_size(1), "1.0 B");
    assert_eq!(format_file_size(1023), "1023.0 B");
    assert_eq!(format_file_size(1024), "1.0 KB");
    assert_eq!(format_file_size(1536), "1.5 KB");
    assert_eq!(format_file_size(1048576), "1.0 MB");
    assert_eq!(format_file_size(1073741824), "1.0 GB");
}

// Removed base64 tests as they test external library functionality, not our application logic

#[test]
fn test_format_file_size_edge_cases() {
    // Test edge cases for size boundary transitions
    assert_eq!(format_file_size(1023), "1023.0 B");
    assert_eq!(format_file_size(1024), "1.0 KB");
    assert_eq!(format_file_size(1048575), "1024.0 KB");
    assert_eq!(format_file_size(1048576), "1.0 MB");
    assert_eq!(format_file_size(1073741823), "1024.0 MB");
    assert_eq!(format_file_size(1073741824), "1.0 GB");
}

#[test]
fn test_format_file_size_precision() {
    // Test precision and rounding behavior
    assert_eq!(format_file_size(1536), "1.5 KB"); // 1.5 KB exactly
    assert_eq!(format_file_size(1588), "1.6 KB"); // Rounds to 1.6
    assert_eq!(format_file_size(1638), "1.6 KB"); // Rounds to 1.6
    assert_eq!(format_file_size(1689), "1.6 KB"); // Rounds to 1.6
    assert_eq!(format_file_size(1740), "1.7 KB"); // Rounds to 1.7
}

#[test]
fn test_base64_encode_native_empty() {
    // Test empty input
    assert_eq!(base64_encode_native(&[]), "");
}

#[test]
fn test_base64_encode_native_basic() {
    // Test basic text encoding
    assert_eq!(base64_encode_native(b"hello"), "aGVsbG8=");
    assert_eq!(base64_encode_native(b"world"), "d29ybGQ=");
    assert_eq!(
        base64_encode_native(b"Hello, World!"),
        "SGVsbG8sIFdvcmxkIQ=="
    );
}

#[test]
fn test_base64_encode_native_binary_data() {
    // Test binary data encoding
    let binary_data = vec![0u8, 1u8, 255u8, 128u8, 64u8];
    let encoded = base64_encode_native(&binary_data);
    assert!(!encoded.is_empty());
    assert!(
        encoded
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '+' || c == '/' || c == '=')
    );
}

#[test]
fn test_base64_encode_native_all_bytes() {
    // Test with various byte values
    let test_data = vec![0, 127, 128, 255];
    let encoded = base64_encode_native(&test_data);
    assert_eq!(encoded, "AH+A/w==");
}

#[test]
fn test_base64_encode_native_lengths() {
    // Test different input lengths to ensure padding works correctly
    assert_eq!(base64_encode_native(b"A"), "QQ=="); // 1 byte -> 2 padding chars
    assert_eq!(base64_encode_native(b"AB"), "QUI="); // 2 bytes -> 1 padding char  
    assert_eq!(base64_encode_native(b"ABC"), "QUJD"); // 3 bytes -> no padding
    assert_eq!(base64_encode_native(b"ABCD"), "QUJDRA=="); // 4 bytes -> 2 padding chars
}

#[test]
fn test_base64_encode_native_unicode() {
    // Test UTF-8 encoded strings
    assert_eq!(base64_encode_native("café".as_bytes()), "Y2Fmw6k=");
    assert_eq!(base64_encode_native("🦀".as_bytes()), "8J+mgA=="); // Rust crab emoji
}

#[test]
fn test_base64_encode_native_long_input() {
    // Test with longer input to ensure it handles larger data
    let long_input = "a".repeat(1000).into_bytes();
    let encoded = base64_encode_native(&long_input);

    // Should be roughly 4/3 the length of input
    assert!(encoded.len() > 1000);
    assert!(encoded.len() < 1500);

    // Should only contain valid base64 characters
    assert!(
        encoded
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '+' || c == '/' || c == '=')
    );
}
