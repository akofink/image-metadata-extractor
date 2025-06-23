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
    // This test validates that every database entry is properly accessible
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
    // Ensure all categories have our specific emoji prefixes
    let expected_emojis = ["📷", "⚙️", "🖼️", "🕒", "📍", "🔍", "📊"];

    for (_, info) in METADATA_DB {
        let has_expected_emoji = expected_emojis
            .iter()
            .any(|emoji| info.category.starts_with(emoji));
        assert!(
            has_expected_emoji,
            "Category should start with one of our expected emojis: {}",
            info.category
        );
    }
}
