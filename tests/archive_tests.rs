//! Tests for the archive module

use image_metadata_extractor::archive::{get_mime_type, is_image_file};

#[test]
fn test_is_image_file() {
    // JPEG variations
    assert!(is_image_file("photo.jpg"));
    assert!(is_image_file("photo.JPEG"));
    assert!(is_image_file("photo.jpeg"));
    assert!(is_image_file("photo.JPG"));

    // PNG
    assert!(is_image_file("image.png"));
    assert!(is_image_file("image.PNG"));

    // GIF
    assert!(is_image_file("pic.gif"));
    assert!(is_image_file("pic.GIF"));

    // WebP
    assert!(is_image_file("modern.webp"));
    assert!(is_image_file("modern.WEBP"));

    // HEIF/HEIC
    assert!(is_image_file("raw.heif"));
    assert!(is_image_file("raw.heic"));
    assert!(is_image_file("raw.HEIF"));
    assert!(is_image_file("raw.HEIC"));

    // TIFF
    assert!(is_image_file("scan.tiff"));
    assert!(is_image_file("scan.tif"));
    assert!(is_image_file("scan.TIFF"));
    assert!(is_image_file("scan.TIF"));

    // AVIF
    assert!(is_image_file("next.avif"));
    assert!(is_image_file("next.AVIF"));

    // JPEG XL
    assert!(is_image_file("modern.jxl"));
    assert!(is_image_file("modern.JXL"));

    // SVG
    assert!(is_image_file("vector.svg"));
    assert!(is_image_file("vector.SVG"));

    // PDF
    assert!(is_image_file("document.pdf"));
    assert!(is_image_file("document.PDF"));

    // Non-image files
    assert!(!is_image_file("document.txt"));
    assert!(!is_image_file("archive.zip"));
    assert!(!is_image_file("README.md"));
    assert!(!is_image_file("data.json"));
    assert!(!is_image_file("code.rs"));
}

#[test]
fn test_get_mime_type_comprehensive() {
    // JPEG
    assert_eq!(get_mime_type("photo.jpg"), "image/jpeg");
    assert_eq!(get_mime_type("photo.JPEG"), "image/jpeg");
    assert_eq!(get_mime_type("photo.jpeg"), "image/jpeg");
    assert_eq!(get_mime_type("photo.JPG"), "image/jpeg");

    // PNG
    assert_eq!(get_mime_type("image.png"), "image/png");
    assert_eq!(get_mime_type("image.PNG"), "image/png");

    // GIF
    assert_eq!(get_mime_type("pic.gif"), "image/gif");
    assert_eq!(get_mime_type("pic.GIF"), "image/gif");

    // WebP
    assert_eq!(get_mime_type("modern.webp"), "image/webp");
    assert_eq!(get_mime_type("modern.WEBP"), "image/webp");

    // TIFF
    assert_eq!(get_mime_type("scan.tiff"), "image/tiff");
    assert_eq!(get_mime_type("scan.tif"), "image/tiff");
    assert_eq!(get_mime_type("scan.TIFF"), "image/tiff");
    assert_eq!(get_mime_type("scan.TIF"), "image/tiff");

    // HEIF/HEIC
    assert_eq!(get_mime_type("raw.heif"), "image/heif");
    assert_eq!(get_mime_type("raw.heic"), "image/heif");
    assert_eq!(get_mime_type("raw.HEIF"), "image/heif");
    assert_eq!(get_mime_type("raw.HEIC"), "image/heif");

    // AVIF
    assert_eq!(get_mime_type("next.avif"), "image/avif");
    assert_eq!(get_mime_type("next.AVIF"), "image/avif");

    // JPEG XL
    assert_eq!(get_mime_type("modern.jxl"), "image/jxl");
    assert_eq!(get_mime_type("modern.JXL"), "image/jxl");

    // SVG
    assert_eq!(get_mime_type("vector.svg"), "image/svg+xml");
    assert_eq!(get_mime_type("vector.SVG"), "image/svg+xml");

    // PDF
    assert_eq!(get_mime_type("document.pdf"), "application/pdf");
    assert_eq!(get_mime_type("document.PDF"), "application/pdf");

    // Unknown
    assert_eq!(get_mime_type("unknown.xyz"), "application/octet-stream");
    assert_eq!(get_mime_type("data.json"), "application/octet-stream");
}

#[test]
fn test_is_image_file_edge_cases() {
    // Empty filename
    assert!(!is_image_file(""));

    // Just extension (still matches because it ends with the extension)
    assert!(is_image_file(".jpg"));
    assert!(is_image_file(".png"));

    // Multiple dots
    assert!(is_image_file("my.photo.jpg"));
    assert!(is_image_file("archive.backup.png"));

    // No extension
    assert!(!is_image_file("photo"));
    assert!(!is_image_file("document"));
}

#[test]
fn test_get_mime_type_edge_cases() {
    // Empty filename
    assert_eq!(get_mime_type(""), "application/octet-stream");

    // Just extension
    assert_eq!(get_mime_type(".jpg"), "image/jpeg");

    // Multiple dots
    assert_eq!(get_mime_type("my.photo.jpg"), "image/jpeg");

    // No extension
    assert_eq!(get_mime_type("photo"), "application/octet-stream");
}
