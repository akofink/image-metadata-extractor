use image_metadata_extractor::types::ImageData;
use image_metadata_extractor::utils::format_file_size;
use std::collections::{HashMap, HashSet};

#[test]
fn test_filter_metadata_selective() {
    let mut exif = HashMap::new();
    exif.insert("ISO".to_string(), "100".to_string());
    exif.insert("Aperture".to_string(), "f/1.8".to_string());

    let data = ImageData {
        name: "photo.jpg".to_string(),
        size: 1234,
        mime_type: "image/jpeg".to_string(),
        data_url: String::new(),
        width: Some(800),
        height: Some(600),
        exif_data: exif.clone(),
        gps_coords: Some((1.0, 2.0)),
    };

    let mut keys = HashSet::new();
    keys.insert("ISO".to_string());

    let filtered = data.filter_metadata(&keys, false, false);
    assert_eq!(filtered.name, "");
    assert_eq!(filtered.size, 0);
    assert!(filtered.width.is_none());
    assert!(filtered.height.is_none());
    assert!(filtered.gps_coords.is_none());
    assert_eq!(filtered.exif_data.len(), 1);
    assert_eq!(filtered.exif_data.get("ISO"), Some(&"100".to_string()));
}

#[test]
fn test_format_file_size_various_units() {
    assert_eq!(format_file_size(0), "0.0 B");
    assert_eq!(format_file_size(1023), "1023.0 B");
    assert_eq!(format_file_size(1024), "1.0 KB");
    assert_eq!(format_file_size(1024 * 1024), "1.0 MB");
    assert_eq!(format_file_size(1024 * 1024 * 1024), "1.0 GB");
}
