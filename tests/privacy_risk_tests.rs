//! Tests for privacy risk calculation and assessment

use image_metadata_extractor::types::{ImageData, PrivacyRiskLevel};
use std::collections::HashMap;

#[test]
fn test_privacy_risk_no_sensitive_data() {
    let data = ImageData {
        name: "test.jpg".to_string(),
        size: 1024,
        mime_type: "image/jpeg".to_string(),
        data_url: String::new(),
        width: Some(800),
        height: Some(600),
        exif_data: HashMap::new(),
        gps_coords: None,
        sha256_hash: None,
    };

    let risk = data.calculate_privacy_risk();
    assert_eq!(risk.level, PrivacyRiskLevel::Low);
    assert_eq!(risk.score, 0);
    assert!(risk.warnings.is_empty());
    assert!(risk.sensitive_fields.is_empty());
    assert!(risk.consistency_issues.is_empty());
}

#[test]
fn test_privacy_risk_gps_critical() {
    let data = ImageData {
        name: "test.jpg".to_string(),
        size: 1024,
        mime_type: "image/jpeg".to_string(),
        data_url: String::new(),
        width: Some(800),
        height: Some(600),
        exif_data: HashMap::new(),
        gps_coords: Some((40.7128, -74.0060)), // NYC coordinates
        sha256_hash: None,
    };

    let risk = data.calculate_privacy_risk();
    assert_eq!(risk.level, PrivacyRiskLevel::High); // 40 points = High
    assert_eq!(risk.score, 40);
    assert_eq!(risk.warnings.len(), 1);
    assert_eq!(risk.sensitive_fields.len(), 1);
    assert!(risk.warnings[0].contains("GPS coordinates"));
    assert_eq!(risk.sensitive_fields[0], "GPS Location");
}

#[test]
fn test_privacy_risk_camera_serial_number() {
    let mut exif_data = HashMap::new();
    exif_data.insert("BodySerialNumber".to_string(), "12345678".to_string());

    let data = ImageData {
        name: "test.jpg".to_string(),
        size: 1024,
        mime_type: "image/jpeg".to_string(),
        data_url: String::new(),
        width: Some(800),
        height: Some(600),
        exif_data,
        gps_coords: None,
        sha256_hash: None,
    };

    let risk = data.calculate_privacy_risk();
    assert_eq!(risk.level, PrivacyRiskLevel::Medium); // 25 points = Medium
    assert_eq!(risk.score, 25);
    assert!(risk.warnings.iter().any(|w| w.contains("serial number")));
    assert!(
        risk.sensitive_fields
            .contains(&"Camera Serial Number".to_string())
    );
}

#[test]
fn test_privacy_risk_owner_name() {
    let mut exif_data = HashMap::new();
    exif_data.insert("Artist".to_string(), "John Doe".to_string());

    let data = ImageData {
        name: "test.jpg".to_string(),
        size: 1024,
        mime_type: "image/jpeg".to_string(),
        data_url: String::new(),
        width: Some(800),
        height: Some(600),
        exif_data,
        gps_coords: None,
        sha256_hash: None,
    };

    let risk = data.calculate_privacy_risk();
    assert_eq!(risk.level, PrivacyRiskLevel::Medium);
    assert_eq!(risk.score, 25);
    assert!(risk.warnings.iter().any(|w| w.contains("artist name")));
}

#[test]
fn test_privacy_risk_copyright() {
    let mut exif_data = HashMap::new();
    exif_data.insert("Copyright".to_string(), "© 2024 John Doe".to_string());

    let data = ImageData {
        name: "test.jpg".to_string(),
        size: 1024,
        mime_type: "image/jpeg".to_string(),
        data_url: String::new(),
        width: Some(800),
        height: Some(600),
        exif_data,
        gps_coords: None,
        sha256_hash: None,
    };

    let risk = data.calculate_privacy_risk();
    assert_eq!(risk.score, 25);
    assert!(
        risk.sensitive_fields
            .contains(&"Owner/Artist Name".to_string())
    );
}

#[test]
fn test_privacy_risk_software() {
    let mut exif_data = HashMap::new();
    exif_data.insert("Software".to_string(), "Adobe Photoshop".to_string());

    let data = ImageData {
        name: "test.jpg".to_string(),
        size: 1024,
        mime_type: "image/jpeg".to_string(),
        data_url: String::new(),
        width: Some(800),
        height: Some(600),
        exif_data,
        gps_coords: None,
        sha256_hash: None,
    };

    let risk = data.calculate_privacy_risk();
    assert_eq!(risk.score, 10);
    assert!(risk.warnings.iter().any(|w| w.contains("Software")));
}

#[test]
fn test_privacy_risk_timestamps() {
    let mut exif_data = HashMap::new();
    exif_data.insert("DateTime".to_string(), "2024:01:01 12:00:00".to_string());

    let data = ImageData {
        name: "test.jpg".to_string(),
        size: 1024,
        mime_type: "image/jpeg".to_string(),
        data_url: String::new(),
        width: Some(800),
        height: Some(600),
        exif_data,
        gps_coords: None,
        sha256_hash: None,
    };

    let risk = data.calculate_privacy_risk();
    assert_eq!(risk.score, 15);
    assert!(risk.warnings.iter().any(|w| w.contains("Timestamps")));
}

#[test]
fn test_privacy_risk_camera_make_model() {
    let mut exif_data = HashMap::new();
    exif_data.insert("Make".to_string(), "Canon".to_string());
    exif_data.insert("Model".to_string(), "EOS 5D Mark IV".to_string());

    let data = ImageData {
        name: "test.jpg".to_string(),
        size: 1024,
        mime_type: "image/jpeg".to_string(),
        data_url: String::new(),
        width: Some(800),
        height: Some(600),
        exif_data,
        gps_coords: None,
        sha256_hash: None,
    };

    let risk = data.calculate_privacy_risk();
    assert_eq!(risk.score, 10);
    assert!(risk.warnings.iter().any(|w| w.contains("Camera make")));
}

#[test]
fn test_privacy_risk_lens_info() {
    let mut exif_data = HashMap::new();
    exif_data.insert("LensModel".to_string(), "EF 24-70mm f/2.8L".to_string());

    let data = ImageData {
        name: "test.jpg".to_string(),
        size: 1024,
        mime_type: "image/jpeg".to_string(),
        data_url: String::new(),
        width: Some(800),
        height: Some(600),
        exif_data,
        gps_coords: None,
        sha256_hash: None,
    };

    let risk = data.calculate_privacy_risk();
    assert_eq!(risk.score, 5);
    assert!(risk.warnings.iter().any(|w| w.contains("Lens")));
}

#[test]
fn test_privacy_risk_critical_combined() {
    let mut exif_data = HashMap::new();
    exif_data.insert("Artist".to_string(), "John Doe".to_string());
    exif_data.insert("BodySerialNumber".to_string(), "12345678".to_string());

    let data = ImageData {
        name: "test.jpg".to_string(),
        size: 1024,
        mime_type: "image/jpeg".to_string(),
        data_url: String::new(),
        width: Some(800),
        height: Some(600),
        exif_data,
        gps_coords: Some((40.7128, -74.0060)),
        sha256_hash: None,
    };

    let risk = data.calculate_privacy_risk();
    assert_eq!(risk.level, PrivacyRiskLevel::Critical); // 40 + 25 + 25 = 90
    assert_eq!(risk.score, 90);
    assert_eq!(risk.sensitive_fields.len(), 3);
}

#[test]
fn test_privacy_risk_all_fields() {
    let mut exif_data = HashMap::new();
    exif_data.insert("Artist".to_string(), "John Doe".to_string());
    exif_data.insert("BodySerialNumber".to_string(), "12345678".to_string());
    exif_data.insert("Software".to_string(), "Lightroom".to_string());
    exif_data.insert("DateTime".to_string(), "2024:01:01 12:00:00".to_string());
    exif_data.insert("Make".to_string(), "Canon".to_string());
    exif_data.insert("Model".to_string(), "EOS R5".to_string());
    exif_data.insert("LensModel".to_string(), "RF 24-70mm".to_string());

    let data = ImageData {
        name: "test.jpg".to_string(),
        size: 1024,
        mime_type: "image/jpeg".to_string(),
        data_url: String::new(),
        width: Some(800),
        height: Some(600),
        exif_data,
        gps_coords: Some((40.7128, -74.0060)),
        sha256_hash: None,
    };

    let risk = data.calculate_privacy_risk();
    assert_eq!(risk.level, PrivacyRiskLevel::Critical);
    // 40 (GPS) + 25 (Artist) + 25 (Serial) + 10 (Software) + 15 (DateTime) + 10 (Make/Model) + 5 (Lens) = 130
    assert_eq!(risk.score, 130);
    assert_eq!(risk.sensitive_fields.len(), 7);
}

#[test]
fn test_consistency_gps_missing_ref() {
    let exif_data = HashMap::new();
    // GPS coordinates but no reference fields should trigger warning

    let data = ImageData {
        name: "test.jpg".to_string(),
        size: 1024,
        mime_type: "image/jpeg".to_string(),
        data_url: String::new(),
        width: Some(800),
        height: Some(600),
        exif_data,
        gps_coords: Some((40.7128, -74.0060)),
        sha256_hash: None,
    };

    let risk = data.calculate_privacy_risk();
    assert!(!risk.consistency_issues.is_empty());
    assert!(risk.consistency_issues.iter().any(|i| i.contains("GPS")));
}

#[test]
fn test_consistency_timestamp_mismatch() {
    let mut exif_data = HashMap::new();
    exif_data.insert("DateTime".to_string(), "2024:01:01 12:00:00".to_string());
    exif_data.insert(
        "DateTimeOriginal".to_string(),
        "2024:01:01 10:00:00".to_string(),
    );

    let data = ImageData {
        name: "test.jpg".to_string(),
        size: 1024,
        mime_type: "image/jpeg".to_string(),
        data_url: String::new(),
        width: Some(800),
        height: Some(600),
        exif_data,
        gps_coords: None,
        sha256_hash: None,
    };

    let risk = data.calculate_privacy_risk();
    assert!(!risk.consistency_issues.is_empty());
    assert!(
        risk.consistency_issues
            .iter()
            .any(|i| i.contains("DateTime"))
    );
}

#[test]
fn test_consistency_orientation_without_dimensions() {
    let mut exif_data = HashMap::new();
    exif_data.insert("Orientation".to_string(), "1".to_string());

    let data = ImageData {
        name: "test.jpg".to_string(),
        size: 1024,
        mime_type: "image/jpeg".to_string(),
        data_url: String::new(),
        width: None,
        height: None,
        exif_data,
        gps_coords: None,
        sha256_hash: None,
    };

    let risk = data.calculate_privacy_risk();
    assert!(!risk.consistency_issues.is_empty());
    assert!(
        risk.consistency_issues
            .iter()
            .any(|i| i.contains("Orientation"))
    );
}

#[test]
fn test_consistency_dimension_mismatch() {
    let mut exif_data = HashMap::new();
    exif_data.insert("Make".to_string(), "Canon".to_string());
    exif_data.insert("LensModel".to_string(), "RF 24-70mm".to_string());
    exif_data.insert("PixelXDimension".to_string(), "4000".to_string());
    exif_data.insert("ExifImageWidth".to_string(), "3000".to_string());

    let data = ImageData {
        name: "test.jpg".to_string(),
        size: 1024,
        mime_type: "image/jpeg".to_string(),
        data_url: String::new(),
        width: Some(800),
        height: Some(600),
        exif_data,
        gps_coords: None,
        sha256_hash: None,
    };

    let risk = data.calculate_privacy_risk();
    assert!(!risk.consistency_issues.is_empty());
    assert!(
        risk.consistency_issues
            .iter()
            .any(|i| i.contains("dimension"))
    );
}

#[test]
fn test_consistency_software_without_timestamp() {
    let mut exif_data = HashMap::new();
    exif_data.insert("Software".to_string(), "Photoshop".to_string());

    let data = ImageData {
        name: "test.jpg".to_string(),
        size: 1024,
        mime_type: "image/jpeg".to_string(),
        data_url: String::new(),
        width: Some(800),
        height: Some(600),
        exif_data,
        gps_coords: None,
        sha256_hash: None,
    };

    let risk = data.calculate_privacy_risk();
    assert!(!risk.consistency_issues.is_empty());
    assert!(
        risk.consistency_issues
            .iter()
            .any(|i| i.contains("Software"))
    );
}

#[test]
fn test_consistency_timestamp_match_no_issue() {
    let mut exif_data = HashMap::new();
    exif_data.insert("DateTime".to_string(), "2024:01:01 12:00:00".to_string());
    exif_data.insert(
        "DateTimeOriginal".to_string(),
        "2024:01:01 12:00:00".to_string(),
    );

    let data = ImageData {
        name: "test.jpg".to_string(),
        size: 1024,
        mime_type: "image/jpeg".to_string(),
        data_url: String::new(),
        width: Some(800),
        height: Some(600),
        exif_data,
        gps_coords: None,
        sha256_hash: None,
    };

    let risk = data.calculate_privacy_risk();
    // Should not have timestamp inconsistency issue
    assert!(
        !risk
            .consistency_issues
            .iter()
            .any(|i| i.contains("DateTime") && i.contains("differ"))
    );
}

#[test]
fn test_risk_level_boundaries() {
    // Test Low (0-19)
    let data_low = ImageData {
        name: "test.jpg".to_string(),
        size: 1024,
        mime_type: "image/jpeg".to_string(),
        data_url: String::new(),
        width: Some(800),
        height: Some(600),
        exif_data: HashMap::new(),
        gps_coords: None,
        sha256_hash: None,
    };
    assert_eq!(
        data_low.calculate_privacy_risk().level,
        PrivacyRiskLevel::Low
    );

    // Test Medium (20-39)
    let mut exif_medium = HashMap::new();
    exif_medium.insert("DateTime".to_string(), "2024:01:01 12:00:00".to_string());
    exif_medium.insert("Make".to_string(), "Canon".to_string());
    exif_medium.insert("Model".to_string(), "EOS".to_string());
    let data_medium = ImageData {
        name: "test.jpg".to_string(),
        size: 1024,
        mime_type: "image/jpeg".to_string(),
        data_url: String::new(),
        width: Some(800),
        height: Some(600),
        exif_data: exif_medium,
        gps_coords: None,
        sha256_hash: None,
    };
    assert_eq!(
        data_medium.calculate_privacy_risk().level,
        PrivacyRiskLevel::Medium
    );

    // Test High (40-59)
    let data_high = ImageData {
        name: "test.jpg".to_string(),
        size: 1024,
        mime_type: "image/jpeg".to_string(),
        data_url: String::new(),
        width: Some(800),
        height: Some(600),
        exif_data: HashMap::new(),
        gps_coords: Some((40.0, -74.0)),
        sha256_hash: None,
    };
    assert_eq!(
        data_high.calculate_privacy_risk().level,
        PrivacyRiskLevel::High
    );

    // Test Critical (60+)
    let mut exif_critical = HashMap::new();
    exif_critical.insert("Artist".to_string(), "John".to_string());
    exif_critical.insert("BodySerialNumber".to_string(), "123".to_string());
    let data_critical = ImageData {
        name: "test.jpg".to_string(),
        size: 1024,
        mime_type: "image/jpeg".to_string(),
        data_url: String::new(),
        width: Some(800),
        height: Some(600),
        exif_data: exif_critical,
        gps_coords: Some((40.0, -74.0)),
        sha256_hash: None,
    };
    assert_eq!(
        data_critical.calculate_privacy_risk().level,
        PrivacyRiskLevel::Critical
    );
}
