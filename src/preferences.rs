//! User preferences management with localStorage persistence.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use web_sys::window;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct UserPreferences {
    pub show_explanations: bool,
    pub include_basic_info: bool,
    pub include_gps: bool,
}

/// Export profile representing a saved metadata selection pattern
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ExportProfile {
    pub name: String,
    pub description: String,
    pub selected_fields: HashSet<String>,
    pub include_basic_info: bool,
    pub include_gps: bool,
}

impl Default for UserPreferences {
    fn default() -> Self {
        Self {
            show_explanations: false,
            include_basic_info: true,
            include_gps: true,
        }
    }
}

const PREFERENCES_KEY: &str = "file_metadata_extractor_preferences";

impl UserPreferences {
    /// Load preferences from localStorage, falling back to defaults
    pub fn load() -> Self {
        if let Some(storage) = window().and_then(|w| w.local_storage().ok().flatten())
            && let Ok(Some(json)) = storage.get_item(PREFERENCES_KEY)
            && let Ok(prefs) = serde_json::from_str(&json)
        {
            return prefs;
        }
        Self::default()
    }

    /// Save preferences to localStorage
    pub fn save(&self) {
        if let Some(storage) = window().and_then(|w| w.local_storage().ok().flatten())
            && let Ok(json) = serde_json::to_string(self)
        {
            let _ = storage.set_item(PREFERENCES_KEY, &json);
        }
    }

    /// Update and save a single preference field
    pub fn update_and_save(&mut self, update_fn: impl FnOnce(&mut Self)) {
        update_fn(self);
        self.save();
    }
}

const EXPORT_PROFILES_KEY: &str = "file_metadata_extractor_export_profiles";

impl ExportProfile {
    /// Create a new export profile
    pub fn new(
        name: String,
        description: String,
        selected_fields: HashSet<String>,
        include_basic_info: bool,
        include_gps: bool,
    ) -> Self {
        Self {
            name,
            description,
            selected_fields,
            include_basic_info,
            include_gps,
        }
    }

    /// Load all saved profiles from localStorage
    pub fn load_all() -> HashMap<String, ExportProfile> {
        if let Some(storage) = window().and_then(|w| w.local_storage().ok().flatten())
            && let Ok(Some(json)) = storage.get_item(EXPORT_PROFILES_KEY)
            && let Ok(profiles) = serde_json::from_str(&json)
        {
            return profiles;
        }
        HashMap::new()
    }

    /// Save a profile to localStorage
    pub fn save(profile: &ExportProfile) -> Result<(), String> {
        let mut profiles = Self::load_all();
        profiles.insert(profile.name.clone(), profile.clone());

        if let Some(storage) = window().and_then(|w| w.local_storage().ok().flatten())
            && let Ok(json) = serde_json::to_string(&profiles)
        {
            storage
                .set_item(EXPORT_PROFILES_KEY, &json)
                .map_err(|_| "Failed to save profile".to_string())?;
            Ok(())
        } else {
            Err("Failed to access localStorage".to_string())
        }
    }

    /// Delete a profile from localStorage
    pub fn delete(profile_name: &str) -> Result<(), String> {
        let mut profiles = Self::load_all();
        profiles.remove(profile_name);

        if let Some(storage) = window().and_then(|w| w.local_storage().ok().flatten())
            && let Ok(json) = serde_json::to_string(&profiles)
        {
            storage
                .set_item(EXPORT_PROFILES_KEY, &json)
                .map_err(|_| "Failed to delete profile".to_string())?;
            Ok(())
        } else {
            Err("Failed to access localStorage".to_string())
        }
    }

    /// Get preset profiles for common use cases
    pub fn get_presets() -> Vec<ExportProfile> {
        vec![
            // Journalism: Location, camera settings, and timestamps
            ExportProfile::new(
                "Journalism".to_string(),
                "Essential metadata for news and photojournalism".to_string(),
                [
                    "DateTime",
                    "DateTimeOriginal",
                    "Make",
                    "Model",
                    "LensModel",
                    "FocalLength",
                    "FNumber",
                    "ISOSpeedRatings",
                    "ExposureTime",
                    "Software",
                ]
                .iter()
                .map(|s| s.to_string())
                .collect(),
                true,
                true,
            ),
            // Real Estate: Minimal technical details without location
            ExportProfile::new(
                "Real Estate".to_string(),
                "Basic image info without sensitive location data".to_string(),
                ["DateTime", "DateTimeOriginal", "Orientation", "ColorSpace"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
                true,
                false,
            ),
            // Forensics: Everything for analysis
            ExportProfile::new(
                "Forensics".to_string(),
                "Complete metadata for forensic analysis and verification".to_string(),
                HashSet::new(), // Will select all fields
                true,
                true,
            ),
            // Privacy: No location, no camera details
            ExportProfile::new(
                "Privacy-Safe".to_string(),
                "Only basic file information without identifying metadata".to_string(),
                ["DateTime", "Orientation", "ColorSpace"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
                true,
                false,
            ),
            // Research: Technical camera settings
            ExportProfile::new(
                "Research/Technical".to_string(),
                "Camera settings and technical specifications".to_string(),
                [
                    "Make",
                    "Model",
                    "LensModel",
                    "FocalLength",
                    "FocalLengthIn35mmFilm",
                    "FNumber",
                    "ISOSpeedRatings",
                    "ExposureTime",
                    "ExposureProgram",
                    "MeteringMode",
                    "Flash",
                    "WhiteBalance",
                    "ColorSpace",
                    "PixelXDimension",
                    "PixelYDimension",
                ]
                .iter()
                .map(|s| s.to_string())
                .collect(),
                true,
                false,
            ),
        ]
    }
}
