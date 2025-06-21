// Unit tests for component logic that can be tested without WASM/UI framework
use image_metadata_extractor::types::ImageData;
use std::collections::{HashMap, HashSet};

#[test]
fn test_image_data_has_metadata() {
    let mut exif = HashMap::new();
    exif.insert("Make".to_string(), "Canon".to_string());

    let data = ImageData {
        name: "test.jpg".to_string(),
        size: 1024,
        mime_type: "image/jpeg".to_string(),
        data_url: String::new(),
        width: Some(800),
        height: Some(600),
        exif_data: exif,
        gps_coords: Some((1.0, 2.0)),
    };

    // Test logic that would be used in MetadataDisplay component
    assert!(!data.exif_data.is_empty());
    assert!(data.gps_coords.is_some());
    assert_eq!(data.width, Some(800));
    assert_eq!(data.height, Some(600));
}

#[test]
fn test_image_data_no_metadata() {
    let data = ImageData {
        name: "plain.png".to_string(),
        size: 512,
        mime_type: "image/png".to_string(),
        data_url: String::new(),
        width: Some(100),
        height: Some(200),
        exif_data: HashMap::new(),
        gps_coords: None,
    };

    // Test logic for empty metadata case in components
    assert!(data.exif_data.is_empty());
    assert!(data.gps_coords.is_none());
}

#[test]
fn test_metadata_selection_logic() {
    let mut exif = HashMap::new();
    exif.insert("Make".to_string(), "Canon".to_string());
    exif.insert("Model".to_string(), "EOS R5".to_string());
    exif.insert("ISO".to_string(), "100".to_string());

    let data = ImageData {
        name: "photo.jpg".to_string(),
        size: 2048,
        mime_type: "image/jpeg".to_string(),
        data_url: String::new(),
        width: Some(4000),
        height: Some(3000),
        exif_data: exif.clone(),
        gps_coords: Some((40.7128, -74.0060)),
    };

    // Test selection logic that would be used in MetadataExport component
    let mut selected = HashSet::new();
    selected.insert("Make".to_string());
    selected.insert("Model".to_string());

    let filtered = data.filter_metadata(&selected, true, true);

    // Should include selected metadata plus file info and GPS
    assert!(!filtered.name.is_empty());
    assert_eq!(filtered.size, 2048);
    assert!(filtered.gps_coords.is_some());
    assert_eq!(filtered.exif_data.len(), 2);
    assert!(filtered.exif_data.contains_key("Make"));
    assert!(filtered.exif_data.contains_key("Model"));
    assert!(!filtered.exif_data.contains_key("ISO"));
}

#[test]
fn test_component_props_equality() {
    // Test that component props implement PartialEq correctly
    let exif = HashMap::new();

    let data1 = ImageData {
        name: "test.jpg".to_string(),
        size: 1024,
        mime_type: "image/jpeg".to_string(),
        data_url: "data:image/jpeg;base64,abc".to_string(),
        width: Some(800),
        height: Some(600),
        exif_data: exif.clone(),
        gps_coords: None,
    };

    let data2 = ImageData {
        name: "test.jpg".to_string(),
        size: 1024,
        mime_type: "image/jpeg".to_string(),
        data_url: "data:image/jpeg;base64,abc".to_string(),
        width: Some(800),
        height: Some(600),
        exif_data: exif,
        gps_coords: None,
    };

    // Test equality logic used in component prop comparison
    assert!(data1 == data2);
}
