use image_metadata_extractor::preferences::*;
use std::collections::HashSet;

#[test]
fn test_user_preferences_default() {
    let prefs = UserPreferences::default();

    // Test default values based on actual structure
    assert!(
        !prefs.show_explanations,
        "Should not show explanations by default"
    );
    assert!(
        prefs.include_basic_info,
        "Should include basic info by default"
    );
    assert!(prefs.include_gps, "Should include GPS by default");
}

#[test]
fn test_user_preferences_creation() {
    let prefs = UserPreferences {
        show_explanations: true,
        include_basic_info: false,
        include_gps: true,
    };

    assert!(prefs.show_explanations);
    assert!(!prefs.include_basic_info);
    assert!(prefs.include_gps);
}

#[test]
fn test_user_preferences_clone_and_equality() {
    let prefs1 = UserPreferences {
        show_explanations: true,
        include_basic_info: false,
        include_gps: true,
    };

    let prefs2 = prefs1.clone();
    assert_eq!(prefs1, prefs2);

    let prefs3 = UserPreferences {
        show_explanations: false,
        include_basic_info: false,
        include_gps: true,
    };

    assert_ne!(prefs1, prefs3);
}

#[test]
fn test_export_profile_creation() {
    let mut selected_fields = HashSet::new();
    selected_fields.insert("Make".to_string());
    selected_fields.insert("Model".to_string());
    selected_fields.insert("DateTime".to_string());

    let profile = ExportProfile::new(
        "Test Profile".to_string(),
        "A test profile".to_string(),
        selected_fields.clone(),
        true,
        false,
    );

    assert_eq!(profile.name, "Test Profile");
    assert_eq!(profile.description, "A test profile");
    assert_eq!(profile.selected_fields, selected_fields);
    assert!(profile.include_basic_info);
    assert!(!profile.include_gps);
}

#[test]
fn test_export_profile_fields() {
    let mut selected_fields = HashSet::new();
    selected_fields.insert("Camera_Make".to_string());
    selected_fields.insert("Camera_Model".to_string());

    let profile = ExportProfile {
        name: "Camera Profile".to_string(),
        description: "Camera metadata only".to_string(),
        selected_fields: selected_fields.clone(),
        include_basic_info: false,
        include_gps: true,
    };

    assert_eq!(profile.selected_fields.len(), 2);
    assert!(profile.selected_fields.contains("Camera_Make"));
    assert!(profile.selected_fields.contains("Camera_Model"));
    assert!(!profile.selected_fields.contains("GPS_Latitude"));
    assert!(!profile.include_basic_info);
    assert!(profile.include_gps);
}

#[test]
fn test_export_profile_clone_and_equality() {
    let mut fields = HashSet::new();
    fields.insert("TestField".to_string());

    let profile1 = ExportProfile {
        name: "Profile1".to_string(),
        description: "Description1".to_string(),
        selected_fields: fields.clone(),
        include_basic_info: true,
        include_gps: false,
    };

    let profile2 = profile1.clone();
    assert_eq!(profile1, profile2);

    let profile3 = ExportProfile {
        name: "Profile3".to_string(),
        description: "Description1".to_string(),
        selected_fields: fields,
        include_basic_info: true,
        include_gps: false,
    };

    assert_ne!(profile1, profile3); // Different names
}

#[test]
fn test_export_profile_get_presets() {
    let presets = ExportProfile::get_presets();

    // Should have multiple preset profiles
    assert!(!presets.is_empty(), "Should have preset profiles");

    // Check that each preset has required properties
    for preset in &presets {
        assert!(!preset.name.is_empty(), "Preset should have a name");
        assert!(
            !preset.description.is_empty(),
            "Preset should have a description"
        );
        // Selected fields can be empty for some presets
    }

    // Check for expected preset names
    let preset_names: Vec<&String> = presets.iter().map(|p| &p.name).collect();
    assert!(
        preset_names.contains(&&"Journalism".to_string()),
        "Should have Journalism preset"
    );
}

#[test]
fn test_export_profile_journalism_preset() {
    let presets = ExportProfile::get_presets();
    let journalism = presets.iter().find(|p| p.name == "Journalism");

    assert!(journalism.is_some(), "Should have Journalism preset");

    if let Some(profile) = journalism {
        assert_eq!(profile.name, "Journalism");
        assert!(!profile.description.is_empty());
        assert!(
            !profile.selected_fields.is_empty(),
            "Journalism preset should have selected fields"
        );

        // Check for expected journalism fields
        assert!(profile.selected_fields.contains("DateTime"));
        assert!(profile.selected_fields.contains("Make"));
        assert!(profile.selected_fields.contains("Model"));
        assert!(profile.include_basic_info);
        assert!(profile.include_gps);
    }
}

#[test]
fn test_user_preferences_serde() {
    let prefs = UserPreferences {
        show_explanations: true,
        include_basic_info: false,
        include_gps: true,
    };

    // Test serialization
    let json = serde_json::to_string(&prefs).expect("Should serialize to JSON");
    assert!(!json.is_empty());
    assert!(json.contains("show_explanations"));
    assert!(json.contains("include_basic_info"));
    assert!(json.contains("include_gps"));

    // Test deserialization
    let deserialized: UserPreferences =
        serde_json::from_str(&json).expect("Should deserialize from JSON");
    assert_eq!(prefs, deserialized);
}

#[test]
fn test_export_profile_serde() {
    let mut fields = HashSet::new();
    fields.insert("TestField1".to_string());
    fields.insert("TestField2".to_string());

    let profile = ExportProfile {
        name: "Test Profile".to_string(),
        description: "Test Description".to_string(),
        selected_fields: fields.clone(),
        include_basic_info: true,
        include_gps: false,
    };

    // Test serialization
    let json = serde_json::to_string(&profile).expect("Should serialize to JSON");
    assert!(!json.is_empty());
    assert!(json.contains("Test Profile"));
    assert!(json.contains("Test Description"));
    assert!(json.contains("TestField1"));
    assert!(json.contains("TestField2"));

    // Test deserialization
    let deserialized: ExportProfile =
        serde_json::from_str(&json).expect("Should deserialize from JSON");
    assert_eq!(profile, deserialized);
}

#[test]
fn test_export_profile_empty_fields() {
    let profile = ExportProfile::new(
        "Empty Profile".to_string(),
        "Profile with no selected fields".to_string(),
        HashSet::new(),
        false,
        false,
    );

    assert_eq!(profile.name, "Empty Profile");
    assert!(profile.selected_fields.is_empty());
    assert!(!profile.include_basic_info);
    assert!(!profile.include_gps);
}

// Note: load_all and update_and_save methods use localStorage APIs
// which are only available in WebAssembly/browser environments
// These are tested in the WASM integration tests instead

#[test]
fn test_export_profile_preset_varieties() {
    let presets = ExportProfile::get_presets();

    // Should have multiple different presets
    assert!(presets.len() >= 2, "Should have at least 2 preset profiles");

    // Check that preset names are unique
    let mut names = HashSet::new();
    for preset in &presets {
        assert!(
            names.insert(&preset.name),
            "Preset names should be unique: {}",
            preset.name
        );
    }

    // Should have profiles with different field configurations
    let has_minimal = presets.iter().any(|p| p.selected_fields.is_empty());
    let has_comprehensive = presets.iter().any(|p| p.selected_fields.len() > 5);

    // At least one preset should be minimal or comprehensive
    assert!(
        has_minimal || has_comprehensive,
        "Should have variety in preset field counts"
    );
}

// Note: save() and load() methods use localStorage APIs
// which are only available in WebAssembly/browser environments
// These are tested in the WASM integration tests instead
