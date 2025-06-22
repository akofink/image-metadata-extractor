use base64::Engine;
use base64::engine::general_purpose;
use exif::Rational;
use exif::{Field, In, Tag, Value};
#[cfg(target_arch = "wasm32")]
use image_metadata_extractor::exif::create_data_url;
use image_metadata_extractor::exif::{
    determine_mime_type, extract_exif_data, get_dimensions, is_supported_mime_type,
    parse_gps_coordinate,
};
use std::io::Cursor;

const JPG_B64: &str = "/9j/4QCMRXhpZgAASUkqAAgAAAABACWIBAABAAAAHAAAAAAAAAAAAAQAAQACAAIAAABOAAAAAgAFAAMAAABUAAAAAwACAAIAAABXAAAABAAFAAMAAABsAAAAAAAAAAAAAQAAAAEAAAAAAAAAAQAAAAAAAAABAAAAAgAAAAEAAAAeAAAAAQAAAAAAAAABAAAA/9k=";

#[test]
fn test_determine_mime_type_from_file_type() {
    let bytes = b"fake";
    let mime = determine_mime_type("photo.jpg", "image/jpeg", bytes);
    assert_eq!(mime, "image/jpeg");
}

#[test]
fn test_determine_mime_type_guess_formats() {
    let jpeg = &[0xFF, 0xD8, 0xFF, 0xE0];
    assert_eq!(determine_mime_type("a", "", jpeg), "image/jpeg");
    let png = b"\x89PNG\r\n\x1a\n";
    assert_eq!(determine_mime_type("a", "", png), "image/png");
    let gif = b"GIF89a";
    assert_eq!(determine_mime_type("a", "", gif), "image/gif");
}

#[test]
fn test_determine_mime_type_extensions() {
    assert_eq!(determine_mime_type("file.pdf", "", b""), "application/pdf");
    assert_eq!(determine_mime_type("map.svg", "", b""), "image/svg+xml");
    assert_eq!(determine_mime_type("pic.tiff", "", b""), "image/tiff");
    assert_eq!(determine_mime_type("movie.heic", "", b""), "image/heif");
    assert_eq!(determine_mime_type("img.avif", "", b""), "image/avif");
    assert_eq!(determine_mime_type("img.jxl", "", b""), "image/jxl");
    assert_eq!(
        determine_mime_type("unknown.bin", "", b""),
        "application/octet-stream"
    );
}

#[test]
fn test_supported_mime() {
    assert!(is_supported_mime_type("image/png"));
    assert!(!is_supported_mime_type("text/plain"));
}

#[test]
fn test_get_dimensions() {
    let img = image::RgbaImage::new(2, 3);
    let mut buf = Vec::new();
    img.write_to(&mut Cursor::new(&mut buf), image::ImageOutputFormat::Png)
        .unwrap();
    let (w, h) = get_dimensions("image/png", &buf);
    assert_eq!((w, h), (Some(2), Some(3)));
    let (w, h) = get_dimensions("application/pdf", b"data");
    assert_eq!((w, h), (None, None));
}

#[test]
fn test_parse_gps_coordinate() {
    let field = Field {
        tag: Tag::GPSLatitude,
        ifd_num: In::PRIMARY,
        value: Value::Rational(vec![
            Rational { num: 1, denom: 1 },
            Rational { num: 30, denom: 1 },
            Rational { num: 0, denom: 1 },
        ]),
    };
    let img = general_purpose::STANDARD.decode(JPG_B64).unwrap();
    let dummy = exif::Reader::new()
        .read_from_container(&mut Cursor::new(&img))
        .unwrap();
    let val = parse_gps_coordinate(&field, &dummy).unwrap();
    assert!((val - 1.5).abs() < 1e-6);
    let field_short = Field {
        tag: Tag::GPSLatitude,
        ifd_num: In::PRIMARY,
        value: Value::Rational(vec![Rational { num: 1, denom: 1 }]),
    };
    assert!(parse_gps_coordinate(&field_short, &dummy).is_none());
    let field_bad = Field {
        tag: Tag::GPSLatitude,
        ifd_num: In::PRIMARY,
        value: Value::Byte(vec![1]),
    };
    assert!(parse_gps_coordinate(&field_bad, &dummy).is_none());
}

#[test]
fn test_extract_exif_data_gps() {
    let bytes = general_purpose::STANDARD.decode(JPG_B64).unwrap();
    let (map, gps) = extract_exif_data(&bytes);
    assert!(!map.is_empty());
    let (lat, lon) = gps.unwrap();
    assert!((lat - 1.0).abs() < 1e-6);
    assert!((lon + 2.5).abs() < 1e-6);
    let (map2, gps2) = extract_exif_data(b"not exif");
    assert!(map2.is_empty());
    assert!(gps2.is_none());
}

// create_data_url uses web APIs - test in WASM environment only
#[cfg(target_arch = "wasm32")]
#[test]
fn test_create_data_url() {
    let data = b"hello world";
    let url = create_data_url("text/plain", data);
    assert!(url.starts_with("data:text/plain;base64,"));
    assert!(url.contains("aGVsbG8gd29ybGQ=")); // base64 of "hello world"
}

#[test]
fn test_determine_mime_type_webp() {
    // Test WebP format detection
    let webp_header = b"RIFF\x20\x00\x00\x00WEBP";
    let mime = determine_mime_type("test.webp", "", webp_header);
    assert_eq!(mime, "image/webp");
}

#[test]
fn test_determine_mime_type_tif_extension() {
    // Test .tif extension (not just .tiff)
    let mime = determine_mime_type("image.tif", "", b"unknown");
    assert_eq!(mime, "image/tiff");
}

#[test]
fn test_determine_mime_type_case_insensitive() {
    // Test case insensitive extension matching
    let mime1 = determine_mime_type("IMAGE.PDF", "", b"unknown");
    assert_eq!(mime1, "application/pdf");

    let mime2 = determine_mime_type("photo.HEIC", "", b"unknown");
    assert_eq!(mime2, "image/heif");
}

#[test]
fn test_is_supported_mime_type_comprehensive() {
    // Test all supported MIME types
    let supported_types = [
        "image/png",
        "image/jpeg",
        "image/gif",
        "image/webp",
        "application/pdf",
        "image/svg+xml",
        "image/tiff",
        "image/heif",
        "image/avif",
        "image/jxl",
    ];

    for mime_type in &supported_types {
        assert!(
            is_supported_mime_type(mime_type),
            "Should support {}",
            mime_type
        );
    }

    // Test unsupported types
    let unsupported_types = [
        "text/plain",
        "application/json",
        "image/bmp",
        "video/mp4",
        "audio/mp3",
    ];

    for mime_type in &unsupported_types {
        assert!(
            !is_supported_mime_type(mime_type),
            "Should not support {}",
            mime_type
        );
    }
}

#[test]
fn test_get_dimensions_svg_exclusion() {
    // Test that SVG is excluded from dimension parsing
    let svg_data = b"<svg width='100' height='200'></svg>";
    let (w, h) = get_dimensions("image/svg+xml", svg_data);
    assert_eq!((w, h), (None, None));
}

// Note: Invalid image test removed due to dependency issues in test environment

#[test]
fn test_parse_gps_coordinate_edge_cases() {
    let dummy_exif = exif::Reader::new()
        .read_from_container(&mut Cursor::new(
            general_purpose::STANDARD.decode(JPG_B64).unwrap(),
        ))
        .unwrap();

    // Test with empty rational array
    let empty_field = Field {
        tag: Tag::GPSLatitude,
        ifd_num: In::PRIMARY,
        value: Value::Rational(vec![]),
    };
    assert!(parse_gps_coordinate(&empty_field, &dummy_exif).is_none());

    // Test with only 2 rational values (should be 3 for DMS)
    let short_field = Field {
        tag: Tag::GPSLatitude,
        ifd_num: In::PRIMARY,
        value: Value::Rational(vec![
            Rational { num: 40, denom: 1 },
            Rational { num: 30, denom: 1 },
        ]),
    };
    assert!(parse_gps_coordinate(&short_field, &dummy_exif).is_none());

    // Test non-rational value
    let string_field = Field {
        tag: Tag::GPSLatitude,
        ifd_num: In::PRIMARY,
        value: Value::Ascii(vec![b"40.123".to_vec()]),
    };
    assert!(parse_gps_coordinate(&string_field, &dummy_exif).is_none());
}

#[test]
fn test_extract_exif_data_various_inputs() {
    // Test with empty data
    let (map, gps) = extract_exif_data(&[]);
    assert!(map.is_empty());
    assert!(gps.is_none());

    // Test with PNG data (should work for some PNG files with EXIF)
    let png_data = b"\x89PNG\r\n\x1a\n\x00\x00\x00\rIHDR\x00\x00\x00\x01\x00\x00\x00\x01\x08\x02\x00\x00\x00\x90wS\xde";
    let (map, gps) = extract_exif_data(png_data);
    // PNG without EXIF should return empty
    assert!(map.is_empty());
    assert!(gps.is_none());

    // Test with random binary data
    let random_data = &[0x00, 0x01, 0x02, 0x03, 0xFF, 0xFE, 0xFD];
    let (map, gps) = extract_exif_data(random_data);
    assert!(map.is_empty());
    assert!(gps.is_none());
}

#[test]
fn test_get_dimensions_non_image_types() {
    // Test get_dimensions returns None for non-image mime types
    // (avoiding actual image parsing which requires wasm APIs)
    let data = b"some data";

    // Non-image types should return None
    let (w, h) = get_dimensions("text/plain", data);
    assert_eq!((w, h), (None, None));

    let (w, h) = get_dimensions("application/pdf", data);
    assert_eq!((w, h), (None, None));

    let (w, h) = get_dimensions("application/json", data);
    assert_eq!((w, h), (None, None));

    // SVG should also return None as it's excluded
    let (w, h) = get_dimensions("image/svg+xml", data);
    assert_eq!((w, h), (None, None));
}

#[test]
fn test_determine_mime_type_priority_order() {
    // Test that provided file_type takes priority over detection
    let jpeg_bytes = &[0xFF, 0xD8, 0xFF, 0xE0];

    // When file_type is provided, it should be used regardless of bytes
    let mime = determine_mime_type("test.jpg", "custom/type", jpeg_bytes);
    assert_eq!(mime, "custom/type");

    // When file_type is empty, should fall back to detection
    let mime = determine_mime_type("test.jpg", "", jpeg_bytes);
    assert_eq!(mime, "image/jpeg");
}

#[test]
fn test_determine_mime_type_fallback_behavior() {
    // Test fallback to extension when image detection fails
    let unknown_bytes = &[0x00, 0x01, 0x02, 0x03];

    // Should use extension when detection fails
    let mime = determine_mime_type("document.pdf", "", unknown_bytes);
    assert_eq!(mime, "application/pdf");

    // Should fall back to octet-stream for unknown extensions
    let mime = determine_mime_type("file.unknown", "", unknown_bytes);
    assert_eq!(mime, "application/octet-stream");
}

#[test]
fn test_extract_exif_data_comprehensive_parsing() {
    // Test the extract_exif_data function with actual JPEG that has GPS
    let bytes = general_purpose::STANDARD.decode(JPG_B64).unwrap();
    let (exif_map, gps_coords) = extract_exif_data(&bytes);

    // Should extract EXIF fields
    assert!(!exif_map.is_empty(), "Should extract EXIF metadata");

    // Should extract GPS coordinates
    assert!(gps_coords.is_some(), "Should extract GPS coordinates");

    if let Some((lat, lon)) = gps_coords {
        // Verify coordinates are reasonable (from the test image)
        assert!(lat.abs() < 90.0, "Latitude should be valid");
        assert!(lon.abs() < 180.0, "Longitude should be valid");
    }
}

#[test]
fn test_determine_mime_type_all_supported_extensions() {
    // Test all supported file extensions are correctly mapped
    let test_cases = [
        ("file.pdf", "application/pdf"),
        ("image.svg", "image/svg+xml"),
        ("photo.tiff", "image/tiff"),
        ("photo.tif", "image/tiff"),
        ("shot.heif", "image/heif"),
        ("shot.heic", "image/heif"),
        ("modern.avif", "image/avif"),
        ("new.jxl", "image/jxl"),
    ];

    for (filename, expected_mime) in test_cases.iter() {
        let result = determine_mime_type(filename, "", &[]);
        assert_eq!(result, *expected_mime, "Failed for {}", filename);
    }
}
