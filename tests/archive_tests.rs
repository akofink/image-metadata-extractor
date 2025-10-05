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

// Note: ArchivedFile struct tests are skipped because they require WebAssembly APIs
// The struct is tested indirectly through the extract_images_from_zip function
// in integration tests

#[test]
fn test_is_image_file_mixed_case_and_dots() {
    // Mixed case with paths
    assert!(is_image_file("folder/image.JPG"));
    assert!(is_image_file("path/to/photo.Jpeg"));
    assert!(is_image_file("nested/dir/pic.GiF"));

    // Complex filenames
    assert!(is_image_file("IMG_2023-12-25_vacation.jpeg"));
    assert!(is_image_file("Screenshot 2024-01-01 at 10.30.45.png"));
    assert!(is_image_file("Document_Scan_01.pdf"));

    // Multiple extensions (only last one matters)
    assert!(is_image_file("backup.tar.gz.jpg")); // Should be true (ends in .jpg)
    assert!(!is_image_file("image.jpg.backup")); // Should be false (ends in .backup)
}

#[test]
fn test_get_mime_type_with_paths() {
    // Test with full paths
    assert_eq!(get_mime_type("folder/image.jpg"), "image/jpeg");
    assert_eq!(get_mime_type("/absolute/path/photo.png"), "image/png");
    assert_eq!(get_mime_type("relative/path/doc.pdf"), "application/pdf");

    // Test with complex filenames
    assert_eq!(get_mime_type("IMG_2023-12-25_vacation.jpeg"), "image/jpeg");
    assert_eq!(
        get_mime_type("Screenshot 2024-01-01 at 10.30.45.png"),
        "image/png"
    );
    assert_eq!(get_mime_type("Document Scan 01.tiff"), "image/tiff");
}

#[test]
fn test_is_image_file_all_supported_formats() {
    // Test all supported image formats mentioned in the function
    let image_files = [
        "test.jpg",
        "test.jpeg",
        "test.png",
        "test.gif",
        "test.webp",
        "test.tiff",
        "test.tif",
        "test.heif",
        "test.heic",
        "test.avif",
        "test.jxl",
        "test.svg",
        "test.pdf",
    ];

    for filename in &image_files {
        assert!(is_image_file(filename), "Failed for {}", filename);
    }

    // Test non-image files
    let non_image_files = [
        "test.txt",
        "test.doc",
        "test.zip",
        "test.rar",
        "test.mp4",
        "test.mp3",
        "test.html",
        "test.css",
        "test.js",
        "test.json",
        "test.xml",
        "test.yaml",
        "test.toml",
        "test.rs",
        "test.py",
    ];

    for filename in &non_image_files {
        assert!(!is_image_file(filename), "Should be false for {}", filename);
    }
}

#[test]
fn test_get_mime_type_all_supported_formats() {
    // Test all supported MIME types
    let format_tests = [
        ("test.jpg", "image/jpeg"),
        ("test.jpeg", "image/jpeg"),
        ("test.png", "image/png"),
        ("test.gif", "image/gif"),
        ("test.webp", "image/webp"),
        ("test.tiff", "image/tiff"),
        ("test.tif", "image/tiff"),
        ("test.heif", "image/heif"),
        ("test.heic", "image/heif"),
        ("test.avif", "image/avif"),
        ("test.jxl", "image/jxl"),
        ("test.svg", "image/svg+xml"),
        ("test.pdf", "application/pdf"),
        ("test.unknown", "application/octet-stream"),
    ];

    for (filename, expected_mime) in &format_tests {
        assert_eq!(
            get_mime_type(filename),
            *expected_mime,
            "Failed for {}",
            filename
        );
    }
}

#[test]
fn test_filename_parsing_edge_cases() {
    // Test various edge cases that might occur in real ZIP files

    // Windows-style paths
    assert!(is_image_file("C:\\Users\\Documents\\photo.jpg"));
    assert_eq!(
        get_mime_type("C:\\Users\\Documents\\photo.jpg"),
        "image/jpeg"
    );

    // Unix-style paths
    assert!(is_image_file("/home/user/pictures/image.png"));
    assert_eq!(get_mime_type("/home/user/pictures/image.png"), "image/png");

    // Spaces and special characters
    assert!(is_image_file("My Photo Collection/IMG (1).jpeg"));
    assert_eq!(
        get_mime_type("My Photo Collection/IMG (1).jpeg"),
        "image/jpeg"
    );

    // Unicode characters
    assert!(is_image_file("фото.jpg")); // Cyrillic
    assert!(is_image_file("写真.png")); // Japanese
    assert!(is_image_file("ñoño.gif")); // Spanish

    // Very long filenames
    let long_name = "a".repeat(200) + ".jpg";
    assert!(is_image_file(&long_name));
    assert_eq!(get_mime_type(&long_name), "image/jpeg");
}

#[test]
fn test_case_sensitivity_comprehensive() {
    // Test all possible case combinations for popular formats
    let case_variants = [
        ("image.jpg", "image.JPG", "image.Jpg", "image.jPg"),
        ("photo.png", "photo.PNG", "photo.Png", "photo.pNg"),
        ("doc.pdf", "doc.PDF", "doc.Pdf", "doc.pDf"),
        ("vector.svg", "vector.SVG", "vector.Svg", "vector.sVg"),
    ];

    for variants in &case_variants {
        let variant_slice = [variants.0, variants.1, variants.2, variants.3];
        for variant in &variant_slice {
            assert!(
                is_image_file(variant),
                "Failed for case variant: {}",
                variant
            );
        }
    }
}

#[test]
fn test_empty_and_null_like_inputs() {
    // Empty string
    assert!(!is_image_file(""));
    assert_eq!(get_mime_type(""), "application/octet-stream");

    // Just dots
    assert!(!is_image_file("."));
    assert!(!is_image_file(".."));
    assert!(!is_image_file("..."));

    // Whitespace-only (not valid filenames but test robustness)
    assert!(!is_image_file(" "));
    assert!(!is_image_file("\t"));
    assert!(!is_image_file("\n"));

    // Single characters
    assert!(!is_image_file("a"));
    assert!(!is_image_file("1"));
    assert!(!is_image_file("/"));
}

// Note: ArchivedFile blob tests are skipped because they require WebAssembly APIs
// These are tested in WASM integration tests where the full browser environment is available
