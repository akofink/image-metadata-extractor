use image_metadata_extractor::metadata_info::{
    METADATA_DB, get_metadata_category, get_metadata_explanation,
};

#[test]
fn test_metadata_db_not_empty() {
    assert!(!METADATA_DB.is_empty());
}

#[test]
fn test_known_key_returns_explanation() {
    // Test known keys from our database
    assert_eq!(
        get_metadata_explanation("Camera make"),
        Some("Camera manufacturer (e.g., Canon, Nikon, Apple)")
    );
    assert_eq!(
        get_metadata_explanation("GPS latitude"),
        Some("Geographic latitude coordinate where photo was taken")
    );
    assert_eq!(
        get_metadata_explanation("F-number"),
        Some("Aperture setting - lower numbers = wider aperture, shallower depth of field")
    );
}

#[test]
fn test_unknown_key_returns_none() {
    assert_eq!(get_metadata_explanation("NonexistentKey"), None);
}

#[test]
fn test_metadata_categories() {
    assert_eq!(get_metadata_category("GPSLatitude"), "📍 Location");
    assert_eq!(get_metadata_category("DateTimeOriginal"), "🕒 Date & Time");
    assert_eq!(get_metadata_category("Camera model"), "📷 Camera");
    assert_eq!(get_metadata_category("Lens model"), "🔍 Lens");
    assert_eq!(get_metadata_category("F-number"), "⚙️ Settings");
    assert_eq!(get_metadata_category("ColorSpace"), "🖼️ Technical");
    assert_eq!(get_metadata_category("UnknownKey"), "📊 Other");
}

#[test]
fn test_all_database_entries_accessible() {
    // This test ensures 100% coverage of the database
    // and validates that every entry is properly accessible
    for (key, expected_info) in METADATA_DB {
        let explanation = get_metadata_explanation(key);
        let category = get_metadata_category(key);

        assert_eq!(
            explanation,
            Some(expected_info.explanation),
            "Failed to get explanation for key: {}",
            key
        );
        assert_eq!(
            category, expected_info.category,
            "Failed to get category for key: {}",
            key
        );
    }
}

#[test]
fn test_database_consistency() {
    // Ensure no duplicate keys in database
    let mut seen_keys = std::collections::HashSet::new();
    for (key, _) in METADATA_DB {
        assert!(seen_keys.insert(key), "Duplicate key found: {}", key);
    }
}

#[test]
fn test_category_emojis_present() {
    // Ensure all categories have proper emoji prefixes
    for (_, info) in METADATA_DB {
        assert!(
            info.category
                .chars()
                .next()
                .unwrap()
                .is_emoji_modifier_base()
                || info
                    .category
                    .chars()
                    .next()
                    .unwrap()
                    .is_emoji_presentation()
                || info.category.starts_with("📷")
                || info.category.starts_with("⚙️")
                || info.category.starts_with("🖼️")
                || info.category.starts_with("🕒")
                || info.category.starts_with("📍")
                || info.category.starts_with("🔍")
                || info.category.starts_with("📊"),
            "Category missing emoji: {}",
            info.category
        );
    }
}

trait EmojiCheck {
    fn is_emoji_modifier_base(&self) -> bool;
    fn is_emoji_presentation(&self) -> bool;
}

impl EmojiCheck for char {
    fn is_emoji_modifier_base(&self) -> bool {
        // Basic emoji detection - this is simplified
        matches!(*self as u32, 0x1F600..=0x1F64F | 0x1F300..=0x1F5FF | 0x1F680..=0x1F6FF | 0x1F1E6..=0x1F1FF)
    }

    fn is_emoji_presentation(&self) -> bool {
        // This checks for common emoji ranges
        matches!(*self as u32, 0x231A..=0x231B | 0x23E9..=0x23EC | 0x23F0 | 0x23F3 | 0x25FD..=0x25FE | 0x2614..=0x2615)
    }
}
