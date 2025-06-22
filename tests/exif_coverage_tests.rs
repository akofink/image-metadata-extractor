// Tests specifically targeting uncovered lines and regions in exif.rs
// Focus on GPS reference handling, image dimensions, and format detection edge cases

use base64::Engine;
use base64::engine::general_purpose;
use image_metadata_extractor::exif_core::{determine_mime_type, extract_exif_data};
use image_metadata_extractor::exif_wasm::get_dimensions;

const JPG_B64: &str = "/9j/4QCMRXhpZgAASUkqAAgAAAABACWIBAABAAAAHAAAAAAAAAAAAAQAAQACAAIAAABOAAAAAgAFAAMAAABUAAAAAwACAAIAAABXAAAABAAFAAMAAABsAAAAAAAAAAAAAQAAAAEAAAAAAAAAAQAAAAAAAAABAAAAAgAAAAEAAAAeAAAAAQAAAAAAAAABAAAA/9k=";

#[test]
fn test_get_dimensions_non_image_mime_comprehensive() {
    // Target line 84: Non-image mime type path
    let dummy_data = &[0x01, 0x02, 0x03, 0x04];

    let non_image_types = [
        "application/pdf",
        "text/plain",
        "application/json",
        "video/mp4",
        "audio/wav",
    ];

    for mime_type in non_image_types.iter() {
        let (w, h) = get_dimensions(mime_type, dummy_data);
        assert_eq!(
            (w, h),
            (None, None),
            "Non-image type {} should return None",
            mime_type
        );
    }
}

#[test]
fn test_determine_mime_type_format_detection_failure() {
    // Target lines 34-50: File extension fallback when image::guess_format() fails
    let unrecognized_header = &[0x12, 0x34, 0x56, 0x78]; // Not a known image format

    // Should fall back to extension-based detection
    assert_eq!(
        determine_mime_type("document.pdf", "", unrecognized_header),
        "application/pdf"
    );
    assert_eq!(
        determine_mime_type("vector.svg", "", unrecognized_header),
        "image/svg+xml"
    );
    assert_eq!(
        determine_mime_type("photo.tiff", "", unrecognized_header),
        "image/tiff"
    );
    assert_eq!(
        determine_mime_type("photo.tif", "", unrecognized_header),
        "image/tiff"
    );
    assert_eq!(
        determine_mime_type("shot.heif", "", unrecognized_header),
        "image/heif"
    );
    assert_eq!(
        determine_mime_type("shot.heic", "", unrecognized_header),
        "image/heif"
    );
    assert_eq!(
        determine_mime_type("modern.avif", "", unrecognized_header),
        "image/avif"
    );
    assert_eq!(
        determine_mime_type("nextgen.jxl", "", unrecognized_header),
        "image/jxl"
    );
    assert_eq!(
        determine_mime_type("unknown.xyz", "", unrecognized_header),
        "application/octet-stream"
    );
}

#[test]
fn test_determine_mime_type_edge_case_extensions() {
    // Test less common extension variations
    let data = &[];

    // Test case variations
    assert_eq!(determine_mime_type("FILE.TIF", "", data), "image/tiff");
    assert_eq!(determine_mime_type("IMAGE.HEIF", "", data), "image/heif");
    assert_eq!(determine_mime_type("PHOTO.HEIC", "", data), "image/heif");
    assert_eq!(determine_mime_type("DOC.PDF", "", data), "application/pdf");
    assert_eq!(determine_mime_type("VECTOR.SVG", "", data), "image/svg+xml");
}

#[test]
fn test_extract_exif_data_gps_coordinate_building() {
    // Target lines 130-145: GPS coordinate building and finalization
    let bytes = general_purpose::STANDARD.decode(JPG_B64).unwrap();
    let (exif_map, gps_coords) = extract_exif_data(&bytes);

    // Should extract GPS coordinates with proper reference application
    assert!(!exif_map.is_empty(), "Should extract EXIF fields");
    assert!(gps_coords.is_some(), "Should extract GPS coordinates");

    if let Some((lat, lon)) = gps_coords {
        // Test that coordinates are within valid ranges after reference application
        assert!(
            lat >= -90.0 && lat <= 90.0,
            "Latitude should be valid after reference application"
        );
        assert!(
            lon >= -180.0 && lon <= 180.0,
            "Longitude should be valid after reference application"
        );
    }
}

#[test]
fn test_extract_exif_data_empty_gps_handling() {
    // Test extract_exif_data when GPS coordinates cannot be parsed
    let data_without_gps = &[0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10]; // Minimal JPEG without GPS
    let (_exif_map, gps_coords) = extract_exif_data(data_without_gps);

    // Should handle absence of GPS data gracefully
    assert!(
        gps_coords.is_none(),
        "Should return None for missing GPS data"
    );
    // May or may not have other EXIF data depending on parsing success
}

#[test]
fn test_extract_exif_data_malformed_container() {
    // Test extract_exif_data with data that fails EXIF container parsing
    let malformed_data = &[0x00, 0x01, 0x02, 0x03, 0x04, 0x05]; // Not a valid EXIF container
    let (exif_map, gps_coords) = extract_exif_data(malformed_data);

    // Should handle malformed EXIF container gracefully
    assert!(
        exif_map.is_empty(),
        "Malformed data should return empty EXIF map"
    );
    assert!(
        gps_coords.is_none(),
        "Malformed data should return no GPS coordinates"
    );
}

#[test]
fn test_extract_exif_data_with_minimal_jpeg() {
    // Test with minimal valid JPEG structure
    let minimal_jpeg = &[
        0xFF, 0xD8, // SOI
        0xFF, 0xE0, 0x00, 0x10, // APP0 segment
        0x4A, 0x46, 0x49, 0x46, 0x00, 0x01, 0x01, 0x01, 0x00, 0x48, 0x00, 0x48, 0x00, 0x00, 0xFF,
        0xD9, // EOI
    ];

    let (_exif_map, gps_coords) = extract_exif_data(minimal_jpeg);

    // Minimal JPEG may not have extractable EXIF data
    assert!(
        gps_coords.is_none(),
        "Minimal JPEG should not have GPS data"
    );
}

#[test]
fn test_get_dimensions_svg_exclusion() {
    // Test that SVG is properly excluded from dimension parsing (line 78)
    let svg_data = b"<svg width='100' height='200'></svg>";
    let (w, h) = get_dimensions("image/svg+xml", svg_data);
    assert_eq!(
        (w, h),
        (None, None),
        "SVG should be excluded from dimension parsing"
    );
}

#[test]
fn test_determine_mime_type_comprehensive_format_detection() {
    // Test format detection with actual format headers

    // Test PNG detection
    let png_header = &[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
    assert_eq!(determine_mime_type("test", "", png_header), "image/png");

    // Test JPEG detection
    let jpeg_header = &[0xFF, 0xD8, 0xFF, 0xE0];
    assert_eq!(determine_mime_type("test", "", jpeg_header), "image/jpeg");

    // Test GIF detection
    let gif_header = b"GIF89a";
    assert_eq!(determine_mime_type("test", "", gif_header), "image/gif");

    // Test WebP detection
    let webp_header = b"RIFF\x20\x00\x00\x00WEBP";
    assert_eq!(determine_mime_type("test", "", webp_header), "image/webp");

    // Test fallback when no format detected
    let unknown_header = &[0x00, 0x00, 0x00, 0x00];
    assert_eq!(
        determine_mime_type("test.unknown", "", unknown_header),
        "application/octet-stream"
    );
}

#[test]
fn test_extract_exif_data_field_iteration() {
    // Target lines 131-136: EXIF field iteration and GPS building
    let bytes = general_purpose::STANDARD.decode(JPG_B64).unwrap();
    let (exif_map, gps_coords) = extract_exif_data(&bytes);

    // Should process multiple EXIF fields
    assert!(
        !exif_map.is_empty(),
        "Should extract EXIF fields through iteration"
    );

    // Check that field processing worked correctly
    let has_gps_fields = exif_map
        .keys()
        .any(|key| key.to_lowercase().contains("gps"));
    if gps_coords.is_some() {
        assert!(
            has_gps_fields,
            "If GPS coordinates exist, should have GPS fields in map"
        );
    }
}
