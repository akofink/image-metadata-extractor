use image_metadata_extractor::utils::format_file_size;

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
