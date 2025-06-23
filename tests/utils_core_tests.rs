use image_metadata_extractor::utils_core::format_file_size;

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
