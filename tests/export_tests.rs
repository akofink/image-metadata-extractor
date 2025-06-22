use image_metadata_extractor::export::{generate_csv, generate_txt};
use image_metadata_extractor::types::ImageData;
use std::collections::HashMap;

#[test]
fn test_generate_csv_minimal_data() {
    // Test CSV generation with minimal data (no dimensions, GPS, or EXIF)
    let data = ImageData {
        name: "simple.jpg".to_string(),
        size: 1024,
        mime_type: "image/jpeg".to_string(),
        data_url: String::new(),
        width: None,
        height: None,
        exif_data: HashMap::new(),
        gps_coords: None,
    };

    let csv = generate_csv(&data);

    // Should include header and basic file info
    assert!(csv.contains("Property,Value"));
    assert!(csv.contains("Filename,\"simple.jpg\""));
    assert!(csv.contains("File Size,1.0 KB"));

    // Should not include dimensions or GPS since they're None
    assert!(!csv.contains("Width,"));
    assert!(!csv.contains("Height,"));
    assert!(!csv.contains("GPS Latitude,"));
    assert!(!csv.contains("GPS Longitude,"));
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

#[test]
fn test_generate_csv_dimensions_only() {
    // Test CSV generation with dimensions but no GPS
    let data = ImageData {
        name: "sized.png".to_string(),
        size: 500,
        mime_type: "image/png".to_string(),
        data_url: String::new(),
        width: Some(1920),
        height: Some(1080),
        exif_data: HashMap::new(),
        gps_coords: None,
    };

    let csv = generate_csv(&data);

    // Should include dimensions
    assert!(csv.contains("Width,1920"));
    assert!(csv.contains("Height,1080"));
    assert!(csv.contains("Dimensions,\"1920x1080 pixels\""));

    // Should not include GPS
    assert!(!csv.contains("GPS Latitude,"));
    assert!(!csv.contains("GPS Longitude,"));
}

#[test]
fn test_generate_csv_gps_only() {
    // Test CSV generation with GPS but no dimensions
    let data = ImageData {
        name: "located.jpg".to_string(),
        size: 750,
        mime_type: "image/jpeg".to_string(),
        data_url: String::new(),
        width: None,
        height: None,
        exif_data: HashMap::new(),
        gps_coords: Some((37.7749, -122.4194)),
    };

    let csv = generate_csv(&data);

    // Should include GPS
    assert!(csv.contains("GPS Latitude,37.7749"));
    assert!(csv.contains("GPS Longitude,-122.4194"));

    // Should not include dimensions
    assert!(!csv.contains("Width,"));
    assert!(!csv.contains("Height,"));
    assert!(!csv.contains("Dimensions,"));
}

#[test]
fn test_generate_csv_quote_escaping() {
    // Test CSV quote escaping in EXIF data
    let mut exif = HashMap::new();
    exif.insert("Description".to_string(), "Photo \"test\"".to_string());
    exif.insert(
        "Comment".to_string(),
        "Multiple \"quotes\" in \"text\"".to_string(),
    );

    let data = ImageData {
        name: "quotes.jpg".to_string(),
        size: 1000,
        mime_type: "image/jpeg".to_string(),
        data_url: String::new(),
        width: None,
        height: None,
        exif_data: exif,
        gps_coords: None,
    };

    let csv = generate_csv(&data);

    // Should properly escape quotes in both keys and values
    assert!(csv.contains("\"Description\",\"Photo \"\"test\"\"\""));
    assert!(csv.contains("\"Comment\",\"Multiple \"\"quotes\"\" in \"\"text\"\"\""));
}

#[test]
fn test_generate_txt_with_metadata() {
    // Test TXT generation with EXIF metadata (covers lines 77-82)
    let mut exif = HashMap::new();
    exif.insert("Camera".to_string(), "Canon EOS".to_string());
    exif.insert("ISO".to_string(), "200".to_string());

    let data = ImageData {
        name: "meta.jpg".to_string(),
        size: 2000,
        mime_type: "image/jpeg".to_string(),
        data_url: String::new(),
        width: Some(800),
        height: Some(600),
        exif_data: exif,
        gps_coords: None,
    };

    let txt = generate_txt(&data);

    // Should include file info section
    assert!(txt.contains("FILE INFORMATION"));
    assert!(txt.contains("Filename: meta.jpg"));
    assert!(txt.contains("File Size: 2.0 KB"));
    assert!(txt.contains("Dimensions: 800x600 pixels"));

    // Should include metadata section with EXIF data
    assert!(txt.contains("METADATA"));
    assert!(txt.contains("Camera: Canon EOS"));
    assert!(txt.contains("ISO: 200"));

    // Should not include GPS section
    assert!(!txt.contains("GPS LOCATION"));
}

#[test]
fn test_generate_txt_no_dimensions() {
    // Test TXT generation without dimensions
    let data = ImageData {
        name: "nodims.pdf".to_string(),
        size: 5000,
        mime_type: "application/pdf".to_string(),
        data_url: String::new(),
        width: None,
        height: None,
        exif_data: HashMap::new(),
        gps_coords: None,
    };

    let txt = generate_txt(&data);

    // Should include basic file info
    assert!(txt.contains("Filename: nodims.pdf"));
    assert!(txt.contains("File Size: 4.9 KB"));

    // Should not include dimensions line
    assert!(!txt.contains("Dimensions:"));
}

#[test]
fn test_generate_txt_complete_report() {
    // Test TXT generation with all sections (file info, GPS, metadata)
    let mut exif = HashMap::new();
    exif.insert("Make".to_string(), "Apple".to_string());
    exif.insert("Model".to_string(), "iPhone 12".to_string());

    let data = ImageData {
        name: "complete.heic".to_string(),
        size: 3500,
        mime_type: "image/heif".to_string(),
        data_url: String::new(),
        width: Some(4032),
        height: Some(3024),
        exif_data: exif,
        gps_coords: Some((40.7128, -74.0060)),
    };

    let txt = generate_txt(&data);

    // Should include all sections
    assert!(txt.contains("FILE INFORMATION"));
    assert!(txt.contains("GPS LOCATION"));
    assert!(txt.contains("METADATA"));

    // File info
    assert!(txt.contains("Filename: complete.heic"));
    assert!(txt.contains("File Size: 3.4 KB"));
    assert!(txt.contains("Dimensions: 4032x3024 pixels"));

    // GPS info with all map links
    assert!(txt.contains("Latitude: 40.7128"));
    assert!(txt.contains("Longitude: -74.006"));
    assert!(txt.contains("Google Maps: https://maps.google.com/maps?q=40.7128,-74.006"));
    assert!(txt.contains("Apple Maps: https://maps.apple.com/?ll=40.7128,-74.006"));
    assert!(
        txt.contains("OpenStreetMap: https://www.openstreetmap.org/?mlat=40.7128&mlon=-74.006")
    );

    // EXIF data
    assert!(txt.contains("Make: Apple"));
    assert!(txt.contains("Model: iPhone 12"));
}
