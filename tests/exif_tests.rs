use exif::Rational;
use exif::{Field, In, Tag, Value};
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
    let img = base64::decode(JPG_B64).unwrap();
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
    let bytes = base64::decode(JPG_B64).unwrap();
    let (map, gps) = extract_exif_data(&bytes);
    assert!(!map.is_empty());
    let (lat, lon) = gps.unwrap();
    assert!((lat - 1.0).abs() < 1e-6);
    assert!((lon + 2.5).abs() < 1e-6);
    let (map2, gps2) = extract_exif_data(b"not exif");
    assert!(map2.is_empty());
    assert!(gps2.is_none());
}
