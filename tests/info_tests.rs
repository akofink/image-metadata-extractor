use image_metadata_extractor::image_cleaner::output_format;
use image_metadata_extractor::metadata_info::{get_metadata_category, get_metadata_explanation};

#[test]
fn test_get_metadata_explanation_known() {
    assert_eq!(
        get_metadata_explanation("Camera make"),
        Some("Camera manufacturer (e.g., Canon, Nikon, Apple)")
    );
    assert_eq!(
        get_metadata_explanation("GPS latitude"),
        Some("Geographic latitude coordinate where photo was taken")
    );
}

#[test]
fn test_get_metadata_explanation_unknown() {
    assert!(get_metadata_explanation("UnknownKey").is_none());
}

#[test]
fn test_get_metadata_category_groups() {
    assert_eq!(get_metadata_category("GPSLatitude"), "📍 Location");
    assert_eq!(get_metadata_category("DateTimeOriginal"), "🕒 Date & Time");
    assert_eq!(get_metadata_category("Camera model"), "📷 Camera");
    assert_eq!(get_metadata_category("Lens model"), "🔍 Lens");
    assert_eq!(get_metadata_category("F-number"), "⚙️ Settings");
    assert_eq!(get_metadata_category("ColorSpace"), "🖼️ Technical");
    assert_eq!(get_metadata_category("SomethingElse"), "📊 Other");
}

#[test]
fn test_output_format_cases() {
    assert_eq!(output_format("webp"), ("image/webp", "webp"));
    assert_eq!(output_format("GIF"), ("image/gif", "gif"));
    assert_eq!(output_format("Png"), ("image/png", "png"));
    assert_eq!(output_format("unknown"), ("image/jpeg", "jpg"));
}
