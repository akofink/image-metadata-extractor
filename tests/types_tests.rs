use image_metadata_extractor::types::ImageData;
use serde_json;
use std::collections::{HashMap, HashSet};

#[test]
fn test_image_data_creation() {
    let mut exif = HashMap::new();
    exif.insert("ISO".to_string(), "100".to_string());
    exif.insert("F-number".to_string(), "f/1.8".to_string());

    let data = ImageData {
        name: "test.jpg".to_string(),
        size: 1024,
        mime_type: "image/jpeg".to_string(),
        data_url: "data:image/jpeg;base64,abc123".to_string(),
        width: Some(800),
        height: Some(600),
        exif_data: exif.clone(),
        gps_coords: Some((37.7749, -122.4194)),
        sha256_hash: None,
    };

    assert_eq!(data.name, "test.jpg");
    assert_eq!(data.size, 1024);
    assert_eq!(data.mime_type, "image/jpeg");
    assert_eq!(data.width, Some(800));
    assert_eq!(data.height, Some(600));
    assert_eq!(data.exif_data.len(), 2);
    assert_eq!(data.gps_coords, Some((37.7749, -122.4194)));
}

#[test]
fn test_filter_metadata_all_excluded() {
    let mut exif = HashMap::new();
    exif.insert("ISO".to_string(), "100".to_string());
    exif.insert("Aperture".to_string(), "f/1.8".to_string());

    let data = ImageData {
        name: "photo.jpg".to_string(),
        size: 1234,
        mime_type: "image/jpeg".to_string(),
        data_url: "data:image/jpeg;base64,xyz".to_string(),
        width: Some(800),
        height: Some(600),
        exif_data: exif,
        gps_coords: Some((1.0, 2.0)),
        sha256_hash: None,
    };

    let keys = HashSet::new(); // No keys selected

    let filtered = data.filter_metadata(&keys, false, false);
    assert_eq!(filtered.name, "");
    assert_eq!(filtered.size, 0);
    assert_eq!(filtered.mime_type, "image/jpeg"); // Always preserved
    assert_eq!(filtered.data_url, "data:image/jpeg;base64,xyz"); // Always preserved
    assert!(filtered.width.is_none());
    assert!(filtered.height.is_none());
    assert!(filtered.exif_data.is_empty());
    assert!(filtered.gps_coords.is_none());
}

#[test]
fn test_filter_metadata_selective_exif() {
    let mut exif = HashMap::new();
    exif.insert("ISO".to_string(), "100".to_string());
    exif.insert("Aperture".to_string(), "f/1.8".to_string());
    exif.insert("Camera".to_string(), "Canon".to_string());

    let data = ImageData {
        name: "photo.jpg".to_string(),
        size: 1234,
        mime_type: "image/jpeg".to_string(),
        data_url: String::new(),
        width: Some(800),
        height: Some(600),
        exif_data: exif,
        gps_coords: Some((1.0, 2.0)),
        sha256_hash: None,
    };

    let mut keys = HashSet::new();
    keys.insert("ISO".to_string());
    keys.insert("Camera".to_string());

    let filtered = data.filter_metadata(&keys, false, false);
    assert_eq!(filtered.exif_data.len(), 2);
    assert_eq!(filtered.exif_data.get("ISO"), Some(&"100".to_string()));
    assert_eq!(filtered.exif_data.get("Camera"), Some(&"Canon".to_string()));
    assert!(filtered.exif_data.get("Aperture").is_none());
}

#[test]
fn test_filter_metadata_include_basic_info() {
    let data = ImageData {
        name: "photo.jpg".to_string(),
        size: 1234,
        mime_type: "image/jpeg".to_string(),
        data_url: "data:...".to_string(),
        width: Some(800),
        height: Some(600),
        exif_data: HashMap::new(),
        gps_coords: Some((1.0, 2.0)),
        sha256_hash: None,
    };

    let keys = HashSet::new();

    let filtered = data.filter_metadata(&keys, true, false);
    assert_eq!(filtered.name, "photo.jpg");
    assert_eq!(filtered.size, 1234);
    assert_eq!(filtered.width, Some(800));
    assert_eq!(filtered.height, Some(600));
    assert!(filtered.gps_coords.is_none()); // GPS not included
}

#[test]
fn test_filter_metadata_include_gps() {
    let data = ImageData {
        name: "photo.jpg".to_string(),
        size: 1234,
        mime_type: "image/jpeg".to_string(),
        data_url: String::new(),
        width: Some(800),
        height: Some(600),
        exif_data: HashMap::new(),
        gps_coords: Some((37.7749, -122.4194)),
        sha256_hash: None,
    };

    let keys = HashSet::new();

    let filtered = data.filter_metadata(&keys, false, true);
    assert_eq!(filtered.name, "");
    assert_eq!(filtered.size, 0);
    assert!(filtered.width.is_none());
    assert!(filtered.height.is_none());
    assert_eq!(filtered.gps_coords, Some((37.7749, -122.4194))); // GPS included
}

#[test]
fn test_filter_metadata_include_all() {
    let mut exif = HashMap::new();
    exif.insert("ISO".to_string(), "200".to_string());

    let data = ImageData {
        name: "test.jpg".to_string(),
        size: 5678,
        mime_type: "image/jpeg".to_string(),
        data_url: "data:...".to_string(),
        width: Some(1920),
        height: Some(1080),
        exif_data: exif,
        gps_coords: Some((40.7128, -74.0060)),
        sha256_hash: None,
    };

    let mut keys = HashSet::new();
    keys.insert("ISO".to_string());

    let filtered = data.filter_metadata(&keys, true, true);
    assert_eq!(filtered.name, "test.jpg");
    assert_eq!(filtered.size, 5678);
    assert_eq!(filtered.width, Some(1920));
    assert_eq!(filtered.height, Some(1080));
    assert_eq!(filtered.exif_data.len(), 1);
    assert_eq!(filtered.exif_data.get("ISO"), Some(&"200".to_string()));
    assert_eq!(filtered.gps_coords, Some((40.7128, -74.0060)));
}

#[test]
fn test_filter_metadata_nonexistent_keys() {
    let mut exif = HashMap::new();
    exif.insert("ISO".to_string(), "100".to_string());

    let data = ImageData {
        name: "photo.jpg".to_string(),
        size: 1000,
        mime_type: "image/jpeg".to_string(),
        data_url: String::new(),
        width: Some(640),
        height: Some(480),
        exif_data: exif,
        gps_coords: None,
        sha256_hash: None,
    };

    let mut keys = HashSet::new();
    keys.insert("NonexistentKey".to_string());
    keys.insert("AnotherMissingKey".to_string());

    let filtered = data.filter_metadata(&keys, false, false);
    assert!(filtered.exif_data.is_empty());
}

#[test]
fn test_filter_metadata_empty_exif() {
    let data = ImageData {
        name: "empty.jpg".to_string(),
        size: 500,
        mime_type: "image/jpeg".to_string(),
        data_url: String::new(),
        width: Some(100),
        height: Some(100),
        exif_data: HashMap::new(),
        gps_coords: None,
        sha256_hash: None,
    };

    let mut keys = HashSet::new();
    keys.insert("ISO".to_string());

    let filtered = data.filter_metadata(&keys, true, true);
    assert_eq!(filtered.name, "empty.jpg");
    assert_eq!(filtered.size, 500);
    assert!(filtered.exif_data.is_empty());
    assert!(filtered.gps_coords.is_none());
}

// Removed tests for derived traits (Clone, PartialEq) as they test compiler-guaranteed functionality

#[test]
fn test_image_data_serialization() {
    let mut exif = HashMap::new();
    exif.insert("ISO".to_string(), "200".to_string());

    let data = ImageData {
        name: "test.jpg".to_string(),
        size: 1024,
        mime_type: "image/jpeg".to_string(), // Should be skipped
        data_url: "data:...".to_string(),    // Should be skipped
        width: Some(800),
        height: Some(600),
        exif_data: exif,
        gps_coords: Some((37.7749, -122.4194)),
        sha256_hash: None,
    };

    let json = serde_json::to_string(&data).unwrap();
    assert!(json.contains("\"name\":\"test.jpg\""));
    assert!(json.contains("\"size\":1024"));
    assert!(json.contains("\"width\":800"));
    assert!(json.contains("\"height\":600"));
    assert!(json.contains("\"ISO\":\"200\""));
    assert!(json.contains("\"gps_coords\":[37.7749,-122.4194]"));

    // These should be skipped in serialization
    assert!(!json.contains("mime_type"));
    assert!(!json.contains("data_url"));
}

#[test]
fn test_image_data_serialization_empty_values() {
    // Test the serde skip conditions
    let data = ImageData {
        name: "".to_string(), // Should be skipped (empty string)
        size: 0,              // Should be skipped (zero)
        mime_type: "image/jpeg".to_string(),
        data_url: "data:...".to_string(),
        width: None,               // Should be skipped (None)
        height: None,              // Should be skipped (None)
        exif_data: HashMap::new(), // Should be skipped (empty)
        gps_coords: None,          // Should be skipped (None)
        sha256_hash: None,
    };

    let json = serde_json::to_string(&data).unwrap();
    // Should only have empty object since everything gets skipped
    assert_eq!(json, "{}");
}

#[test]
fn test_image_data_serialization_partial_skip() {
    let mut exif = HashMap::new();
    exif.insert("Camera".to_string(), "Canon".to_string());

    let data = ImageData {
        name: "photo.jpg".to_string(),         // Included (non-empty)
        size: 0,                               // Skipped (zero)
        mime_type: "image/jpeg".to_string(),   // Always skipped
        data_url: "data:...".to_string(),      // Always skipped
        width: Some(1920),                     // Included (Some value)
        height: None,                          // Skipped (None)
        exif_data: exif,                       // Included (non-empty)
        gps_coords: Some((40.7128, -74.0060)), // Included (Some value)
        sha256_hash: None,
    };

    let json = serde_json::to_string(&data).unwrap();
    assert!(json.contains("\"name\":\"photo.jpg\""));
    assert!(json.contains("\"width\":1920"));
    assert!(json.contains("\"Camera\":\"Canon\""));
    assert!(json.contains("\"gps_coords\"")); // Just check field exists

    // These should be skipped
    assert!(!json.contains("size"));
    assert!(!json.contains("height"));
    assert!(!json.contains("mime_type"));
    assert!(!json.contains("data_url"));
}
