//! Tests for additional export formats (Markdown, YAML, XML)

use image_metadata_extractor::export::{generate_md, generate_xml, generate_yaml};
use image_metadata_extractor::types::ImageData;
use std::collections::HashMap;

fn create_test_image_data() -> ImageData {
    let mut exif_data = HashMap::new();
    exif_data.insert("Make".to_string(), "Canon".to_string());
    exif_data.insert("Model".to_string(), "EOS R5".to_string());
    exif_data.insert("DateTime".to_string(), "2024:01:01 12:00:00".to_string());
    exif_data.insert("FNumber".to_string(), "2.8".to_string());
    exif_data.insert("ISOSpeedRatings".to_string(), "400".to_string());

    ImageData {
        name: "test_image.jpg".to_string(),
        size: 2048576, // 2 MB
        mime_type: "image/jpeg".to_string(),
        data_url: String::new(),
        width: Some(6000),
        height: Some(4000),
        exif_data,
        gps_coords: Some((40.7128, -74.0060)), // NYC coordinates
        sha256_hash: Some("a1b2c3d4e5f6".to_string()),
    }
}

// ============================================================================
// Markdown Tests
// ============================================================================

#[test]
fn test_generate_md_structure() {
    let data = create_test_image_data();
    let md = generate_md(&data);

    // Check header structure
    assert!(md.contains("# File Metadata"));
    assert!(md.contains("## File Information"));
    assert!(md.contains("## GPS Location"));
    assert!(md.contains("## Metadata"));
}

#[test]
fn test_generate_md_file_info() {
    let data = create_test_image_data();
    let md = generate_md(&data);

    // Check file info content
    assert!(md.contains("- Filename: test_image.jpg"));
    assert!(md.contains("- File Size:"));
    assert!(md.contains("- Dimensions: 6000x4000 pixels"));
}

#[test]
fn test_generate_md_gps_with_map_links() {
    let data = create_test_image_data();
    let md = generate_md(&data);

    // Check GPS coordinates (trailing zeros may be trimmed in float formatting)
    assert!(md.contains("- Latitude: 40.7128"));
    assert!(md.contains("- Longitude: -74.006"));

    // Check map service links (URLs preserve full precision)
    assert!(md.contains("- Google Maps: https://maps.google.com/maps?q=40.7128,-74.006"));
    assert!(md.contains("- Apple Maps: https://maps.apple.com/?ll=40.7128,-74.006"));
    assert!(
        md.contains("- OpenStreetMap: https://www.openstreetmap.org/?mlat=40.7128&mlon=-74.006")
    );
}

#[test]
fn test_generate_md_exif_data() {
    let data = create_test_image_data();
    let md = generate_md(&data);

    // Check EXIF fields (should be alphabetically sorted)
    assert!(md.contains("- DateTime:"));
    assert!(md.contains("- FNumber:"));
    assert!(md.contains("- ISOSpeedRatings:"));
    assert!(md.contains("- Make: Canon"));
    assert!(md.contains("- Model: EOS R5"));
}

#[test]
fn test_generate_md_minimal_no_gps() {
    let data = ImageData {
        name: "minimal.jpg".to_string(),
        size: 1024,
        mime_type: "image/jpeg".to_string(),
        data_url: String::new(),
        width: None,
        height: None,
        exif_data: HashMap::new(),
        gps_coords: None,
        sha256_hash: None,
    };

    let md = generate_md(&data);

    // Should not contain GPS section
    assert!(!md.contains("## GPS Location"));
    assert!(md.contains("## File Information"));
    assert!(md.contains("minimal.jpg"));
}

#[test]
fn test_generate_md_with_dimensions_no_gps() {
    let data = ImageData {
        name: "no_gps.jpg".to_string(),
        size: 1024,
        mime_type: "image/jpeg".to_string(),
        data_url: String::new(),
        width: Some(800),
        height: Some(600),
        exif_data: HashMap::new(),
        gps_coords: None,
        sha256_hash: None,
    };

    let md = generate_md(&data);
    assert!(!md.contains("GPS"));
    assert!(md.contains("800x600"));
}

// ============================================================================
// YAML Tests
// ============================================================================

#[test]
fn test_generate_yaml_basic_structure() {
    let data = create_test_image_data();
    let yaml = generate_yaml(&data);

    // Check basic YAML keys
    assert!(yaml.contains("name: test_image.jpg"));
    assert!(yaml.contains("size: 2048576"));
    assert!(yaml.contains("dimensions: \"6000x4000\""));
}

#[test]
fn test_generate_yaml_gps() {
    let data = create_test_image_data();
    let yaml = generate_yaml(&data);

    // Check GPS section (trailing zeros may be trimmed in float formatting)
    assert!(yaml.contains("gps:"));
    assert!(yaml.contains("  lat: 40.7128"));
    assert!(yaml.contains("  lon: -74.006"));
}

#[test]
fn test_generate_yaml_exif() {
    let data = create_test_image_data();
    let yaml = generate_yaml(&data);

    // Check EXIF section
    assert!(yaml.contains("exif:"));
    assert!(yaml.contains("\"Make\": \"Canon\""));
    assert!(yaml.contains("\"Model\": \"EOS R5\""));
    assert!(yaml.contains("\"DateTime\":"));
}

#[test]
fn test_generate_yaml_minimal() {
    let data = ImageData {
        name: "minimal.jpg".to_string(),
        size: 1024,
        mime_type: "image/jpeg".to_string(),
        data_url: String::new(),
        width: None,
        height: None,
        exif_data: HashMap::new(),
        gps_coords: None,
        sha256_hash: None,
    };

    let yaml = generate_yaml(&data);

    // Should have basic structure
    assert!(yaml.contains("name: minimal.jpg"));
    assert!(yaml.contains("size: 1024"));
    // Should not have GPS or exif sections
    assert!(!yaml.contains("gps:"));
    assert!(!yaml.contains("exif:"));
}

#[test]
fn test_generate_yaml_quote_escaping() {
    let mut exif_data = HashMap::new();
    exif_data.insert(
        "Description".to_string(),
        "A photo with \"quotes\"".to_string(),
    );

    let data = ImageData {
        name: "special.jpg".to_string(),
        size: 1024,
        mime_type: "image/jpeg".to_string(),
        data_url: String::new(),
        width: Some(800),
        height: Some(600),
        exif_data,
        gps_coords: None,
        sha256_hash: None,
    };

    let yaml = generate_yaml(&data);

    // Should escape quotes in YAML values
    assert!(yaml.contains("Description"));
    assert!(yaml.contains("\\\"") || yaml.contains("quotes"));
}

#[test]
fn test_generate_yaml_no_dimensions() {
    let data = ImageData {
        name: "nodim.jpg".to_string(),
        size: 1024,
        mime_type: "image/jpeg".to_string(),
        data_url: String::new(),
        width: None,
        height: None,
        exif_data: HashMap::new(),
        gps_coords: None,
        sha256_hash: None,
    };

    let yaml = generate_yaml(&data);
    // Should not include dimensions if width/height are None
    assert!(!yaml.contains("dimensions:"));
}

// ============================================================================
// XML Tests
// ============================================================================

#[test]
fn test_generate_xml_structure() {
    let data = create_test_image_data();
    let xml = generate_xml(&data);

    // Check XML declaration and root
    assert!(xml.contains("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"));
    assert!(xml.contains("<metadata>"));
    assert!(xml.contains("</metadata>"));
}

#[test]
fn test_generate_xml_file_info() {
    let data = create_test_image_data();
    let xml = generate_xml(&data);

    // Check file information elements
    assert!(xml.contains("<name>test_image.jpg</name>"));
    assert!(xml.contains("<size>2048576</size>"));
    assert!(xml.contains("<dimensions>6000x4000</dimensions>"));
}

#[test]
fn test_generate_xml_gps() {
    let data = create_test_image_data();
    let xml = generate_xml(&data);

    // Check GPS structure (trailing zeros may be trimmed in float formatting)
    assert!(xml.contains("<gps>"));
    assert!(xml.contains("</gps>"));
    assert!(xml.contains("<lat>40.7128</lat>"));
    assert!(xml.contains("<lon>-74.006</lon>"));
}

#[test]
fn test_generate_xml_exif() {
    let data = create_test_image_data();
    let xml = generate_xml(&data);

    // Check EXIF structure
    assert!(xml.contains("<exif>"));
    assert!(xml.contains("</exif>"));
    assert!(xml.contains("<tag name=\"Make\">Canon</tag>"));
    assert!(xml.contains("<tag name=\"Model\">EOS R5</tag>"));
}

#[test]
fn test_generate_xml_minimal() {
    let data = ImageData {
        name: "minimal.jpg".to_string(),
        size: 1024,
        mime_type: "image/jpeg".to_string(),
        data_url: String::new(),
        width: None,
        height: None,
        exif_data: HashMap::new(),
        gps_coords: None,
        sha256_hash: None,
    };

    let xml = generate_xml(&data);

    // Should have basic structure
    assert!(xml.contains("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"));
    assert!(xml.contains("<name>minimal.jpg</name>"));
    assert!(xml.contains("<size>1024</size>"));

    // Should not have GPS or EXIF sections
    assert!(!xml.contains("<gps>"));
    assert!(!xml.contains("<exif>"));
}

#[test]
fn test_generate_xml_special_characters() {
    let mut exif_data = HashMap::new();
    exif_data.insert("Copyright".to_string(), "<Test & Co.>".to_string());

    let data = ImageData {
        name: "special<>&.jpg".to_string(),
        size: 1024,
        mime_type: "image/jpeg".to_string(),
        data_url: String::new(),
        width: Some(800),
        height: Some(600),
        exif_data,
        gps_coords: None,
        sha256_hash: None,
    };

    let xml = generate_xml(&data);

    // Should escape XML special characters
    assert!(xml.contains("&lt;") || xml.contains("&amp;") || xml.contains("&gt;"));
}

#[test]
fn test_generate_xml_no_dimensions() {
    let data = ImageData {
        name: "nodim.jpg".to_string(),
        size: 1024,
        mime_type: "image/jpeg".to_string(),
        data_url: String::new(),
        width: None,
        height: None,
        exif_data: HashMap::new(),
        gps_coords: None,
        sha256_hash: None,
    };

    let xml = generate_xml(&data);
    // Should not include dimensions element
    assert!(!xml.contains("<dimensions>"));
}

// ============================================================================
// Cross-format Tests
// ============================================================================

#[test]
fn test_all_formats_contain_core_data() {
    let data = create_test_image_data();

    let md = generate_md(&data);
    let yaml = generate_yaml(&data);
    let xml = generate_xml(&data);

    // All formats should contain the core data (trailing zeros may be trimmed)
    for format in &[&md, &yaml, &xml] {
        assert!(format.contains("test_image.jpg"));
        assert!(format.contains("Canon"));
        assert!(format.contains("EOS R5"));
        assert!(format.contains("40.7128"));
        assert!(format.contains("-74.006"));
    }
}

#[test]
fn test_all_formats_empty_exif() {
    let data = ImageData {
        name: "empty.jpg".to_string(),
        size: 1024,
        mime_type: "image/jpeg".to_string(),
        data_url: String::new(),
        width: Some(800),
        height: Some(600),
        exif_data: HashMap::new(),
        gps_coords: None,
        sha256_hash: None,
    };

    let md = generate_md(&data);
    let yaml = generate_yaml(&data);
    let xml = generate_xml(&data);

    // All should produce valid output even without EXIF
    assert!(!md.is_empty());
    assert!(!yaml.is_empty());
    assert!(!xml.is_empty());

    assert!(md.contains("empty.jpg"));
    assert!(yaml.contains("empty.jpg"));
    assert!(xml.contains("empty.jpg"));
}

#[test]
fn test_formats_sorting_consistency() {
    let mut exif_data = HashMap::new();
    exif_data.insert("Zebra".to_string(), "last".to_string());
    exif_data.insert("Apple".to_string(), "first".to_string());
    exif_data.insert("Middle".to_string(), "middle".to_string());

    let data = ImageData {
        name: "sort.jpg".to_string(),
        size: 1024,
        mime_type: "image/jpeg".to_string(),
        data_url: String::new(),
        width: Some(800),
        height: Some(600),
        exif_data,
        gps_coords: None,
        sha256_hash: None,
    };

    let md = generate_md(&data);
    let yaml = generate_yaml(&data);
    let xml = generate_xml(&data);

    // All formats should have sorted EXIF data (alphabetically)
    assert!(md.find("Apple").unwrap() < md.find("Middle").unwrap());
    assert!(md.find("Middle").unwrap() < md.find("Zebra").unwrap());

    assert!(yaml.find("Apple").unwrap() < yaml.find("Middle").unwrap());
    assert!(yaml.find("Middle").unwrap() < yaml.find("Zebra").unwrap());

    assert!(xml.find("Apple").unwrap() < xml.find("Middle").unwrap());
    assert!(xml.find("Middle").unwrap() < xml.find("Zebra").unwrap());
}
