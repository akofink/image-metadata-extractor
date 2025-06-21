use image_metadata_extractor::export::generate_txt;
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
