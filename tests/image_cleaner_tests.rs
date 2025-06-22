use image_metadata_extractor::image_cleaner::output_format;

#[test]
fn test_output_format_webp() {
    let (mime, ext) = output_format("webp");
    assert_eq!(mime, "image/webp");
    assert_eq!(ext, "webp");
}

#[test]
fn test_output_format_gif() {
    let (mime, ext) = output_format("gif");
    assert_eq!(mime, "image/gif");
    assert_eq!(ext, "gif");
}

#[test]
fn test_output_format_png() {
    let (mime, ext) = output_format("png");
    assert_eq!(mime, "image/png");
    assert_eq!(ext, "png");
}

#[test]
fn test_output_format_jpeg_default() {
    // Test that any unrecognized format defaults to JPEG
    let (mime, ext) = output_format("jpg");
    assert_eq!(mime, "image/jpeg");
    assert_eq!(ext, "jpg");

    let (mime, ext) = output_format("jpeg");
    assert_eq!(mime, "image/jpeg");
    assert_eq!(ext, "jpg");
}

#[test]
fn test_output_format_case_insensitive() {
    // Test uppercase formats
    let (mime, ext) = output_format("WEBP");
    assert_eq!(mime, "image/webp");
    assert_eq!(ext, "webp");

    let (mime, ext) = output_format("GIF");
    assert_eq!(mime, "image/gif");
    assert_eq!(ext, "gif");

    let (mime, ext) = output_format("PNG");
    assert_eq!(mime, "image/png");
    assert_eq!(ext, "png");

    // Mixed case
    let (mime, ext) = output_format("WebP");
    assert_eq!(mime, "image/webp");
    assert_eq!(ext, "webp");
}

#[test]
fn test_output_format_unknown_formats() {
    // Test various unknown formats default to JPEG
    let test_cases = [
        "bmp",
        "tiff",
        "svg",
        "avif",
        "jxl",
        "unknown",
        "",
        "123",
        "image/png",
        "file.png",
        "random",
    ];

    for format in test_cases.iter() {
        let (mime, ext) = output_format(format);
        assert_eq!(
            mime, "image/jpeg",
            "Format '{}' should default to JPEG",
            format
        );
        assert_eq!(
            ext, "jpg",
            "Format '{}' should default to jpg extension",
            format
        );
    }
}

#[test]
fn test_output_format_whitespace() {
    // Test formats with whitespace (should not match and default to JPEG)
    let (mime, ext) = output_format(" png ");
    assert_eq!(mime, "image/jpeg");
    assert_eq!(ext, "jpg");

    let (mime, ext) = output_format("png ");
    assert_eq!(mime, "image/jpeg");
    assert_eq!(ext, "jpg");

    let (mime, ext) = output_format(" png");
    assert_eq!(mime, "image/jpeg");
    assert_eq!(ext, "jpg");
}

#[test]
fn test_output_format_edge_cases() {
    // Test empty string
    let (mime, ext) = output_format("");
    assert_eq!(mime, "image/jpeg");
    assert_eq!(ext, "jpg");

    // Test single characters
    let (mime, ext) = output_format("p");
    assert_eq!(mime, "image/jpeg");
    assert_eq!(ext, "jpg");

    // Test numbers
    let (mime, ext) = output_format("123");
    assert_eq!(mime, "image/jpeg");
    assert_eq!(ext, "jpg");

    // Test special characters
    let (mime, ext) = output_format("@#$");
    assert_eq!(mime, "image/jpeg");
    assert_eq!(ext, "jpg");
}

#[test]
fn test_output_format_return_types() {
    // Verify the return types are static string references
    let (mime, ext) = output_format("png");

    // These should be static references that live for the entire program
    assert_eq!(std::mem::size_of_val(&mime), std::mem::size_of::<&str>());
    assert_eq!(std::mem::size_of_val(&ext), std::mem::size_of::<&str>());

    // Verify the strings are not empty
    assert!(!mime.is_empty());
    assert!(!ext.is_empty());
}

#[test]
fn test_output_format_all_supported_formats() {
    // Test that all supported formats are properly handled
    let supported_formats = vec![
        ("webp", "image/webp", "webp"),
        ("gif", "image/gif", "gif"),
        ("png", "image/png", "png"),
    ];

    for (input, expected_mime, expected_ext) in supported_formats {
        let (mime, ext) = output_format(input);
        assert_eq!(mime, expected_mime, "MIME type mismatch for {}", input);
        assert_eq!(ext, expected_ext, "Extension mismatch for {}", input);
    }
}
