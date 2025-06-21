use image_metadata_extractor::export::{generate_csv, generate_txt};
use image_metadata_extractor::types::ImageData;
use std::collections::HashMap;

#[test]
fn test_output_format_placeholder() {
    assert_eq!(1 + 1, 2);
}

#[test]
fn test_generate_txt_includes_map_links() {
    let data = ImageData {
        name: "test.jpg".to_string(),
        size: 123,
        mime_type: "image/jpeg".to_string(),
        data_url: String::new(),
        width: None,
        height: None,
        exif_data: HashMap::new(),
        gps_coords: Some((10.0, 20.0)),
    };

    let txt = generate_txt(&data);
    assert!(txt.contains("Google Maps"));
    assert!(txt.contains("Apple Maps"));
    assert!(txt.contains("OpenStreetMap"));
}

#[test]
fn test_generate_txt_no_metadata_message() {
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

    let txt = generate_txt(&data);
    assert!(txt.contains("Dimensions: 100x200 pixels"));
    assert!(txt.contains("No metadata found in this file"));
}

#[test]
fn test_generate_csv_includes_all_fields() {
    let mut exif = HashMap::new();
    exif.insert("Make".to_string(), "Canon".to_string());
    exif.insert("Model".to_string(), "EOS \"M\"".to_string());

    let data = ImageData {
        name: "pic.jpg".to_string(),
        size: 2048,
        mime_type: "image/jpeg".to_string(),
        data_url: String::new(),
        width: Some(400),
        height: Some(300),
        exif_data: exif,
        gps_coords: Some((12.3, 45.6)),
    };

    let csv = generate_csv(&data);

    assert!(csv.contains("Filename,\"pic.jpg\""));
    assert!(csv.contains("File Size,2.0 KB"));
    assert!(csv.contains("Width,400"));
    assert!(csv.contains("Height,300"));
    assert!(csv.contains("Dimensions,\"400x300 pixels\""));
    assert!(csv.contains("GPS Latitude,12.3"));
    assert!(csv.contains("GPS Longitude,45.6"));
    assert!(csv.contains("\"Make\",\"Canon\""));
    assert!(csv.contains("\"Model\",\"EOS \"\"M\"\"\""));
}
