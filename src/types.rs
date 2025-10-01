//! Shared data structures used throughout the application.

use serde::Serialize;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Theme {
    Light,
    Dark,
}

/// Privacy risk level for metadata
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum PrivacyRiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

/// Privacy risk assessment for image metadata
#[derive(Debug, Clone, Serialize)]
pub struct PrivacyRisk {
    pub level: PrivacyRiskLevel,
    pub score: u32,
    pub warnings: Vec<String>,
    pub sensitive_fields: Vec<String>,
}

/// Metadata extracted from an uploaded file.
#[derive(Clone, PartialEq, Serialize, Debug)]
pub struct ImageData {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub name: String,
    #[serde(skip_serializing_if = "is_zero")]
    pub size: u64,
    #[serde(skip)]
    pub mime_type: String,
    #[serde(skip)] // Don't include data URL in exports
    pub data_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub exif_data: HashMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gps_coords: Option<(f64, f64)>, // (latitude, longitude)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha256_hash: Option<String>, // SHA-256 file hash for forensics and deduplication
}

fn is_zero(value: &u64) -> bool {
    *value == 0
}

impl ImageData {
    /// Return a new `ImageData` containing only the selected metadata fields.
    pub fn filter_metadata(
        &self,
        selected_keys: &HashSet<String>,
        include_basic_info: bool,
        include_gps: bool,
    ) -> Self {
        let mut filtered_exif = HashMap::new();

        // Include selected EXIF fields
        for (key, value) in &self.exif_data {
            if selected_keys.contains(key) {
                filtered_exif.insert(key.clone(), value.clone());
            }
        }

        Self {
            name: if include_basic_info {
                self.name.clone()
            } else {
                String::new()
            },
            size: if include_basic_info { self.size } else { 0 },
            mime_type: self.mime_type.clone(),
            data_url: self.data_url.clone(), // Always keep for display
            width: if include_basic_info { self.width } else { None },
            height: if include_basic_info {
                self.height
            } else {
                None
            },
            exif_data: filtered_exif,
            gps_coords: if include_gps { self.gps_coords } else { None },
            sha256_hash: if include_basic_info {
                self.sha256_hash.clone()
            } else {
                None
            },
        }
    }

    /// Calculate privacy risk based on metadata content
    pub fn calculate_privacy_risk(&self) -> PrivacyRisk {
        let mut score = 0u32;
        let mut warnings = Vec::new();
        let mut sensitive_fields = Vec::new();

        // Critical: GPS location data (40 points)
        if self.gps_coords.is_some() {
            score += 40;
            warnings
                .push("GPS coordinates reveal exact location where photo was taken".to_string());
            sensitive_fields.push("GPS Location".to_string());
        }

        // High: Camera serial number and owner name (25 points each)
        if self.exif_data.contains_key("BodySerialNumber")
            || self.exif_data.contains_key("InternalSerialNumber")
        {
            score += 25;
            warnings.push(
                "Camera serial number can identify specific device and link photos to owner"
                    .to_string(),
            );
            sensitive_fields.push("Camera Serial Number".to_string());
        }

        if self.exif_data.contains_key("Artist")
            || self.exif_data.contains_key("Copyright")
            || self.exif_data.contains_key("OwnerName")
        {
            score += 25;
            warnings.push("Owner or artist name directly identifies the photographer".to_string());
            sensitive_fields.push("Owner/Artist Name".to_string());
        }

        // Medium: Software and timestamps (15 points each)
        if self.exif_data.contains_key("Software") {
            score += 10;
            warnings.push("Software information may reveal editing tools and workflow".to_string());
            sensitive_fields.push("Software".to_string());
        }

        if self.exif_data.contains_key("DateTimeOriginal")
            || self.exif_data.contains_key("DateTime")
        {
            score += 15;
            warnings
                .push("Timestamps reveal when and potentially where photo was taken".to_string());
            sensitive_fields.push("Timestamps".to_string());
        }

        // Medium: Unique camera identifiers (15 points)
        if self.exif_data.contains_key("Make") && self.exif_data.contains_key("Model") {
            score += 10;
            warnings.push(
                "Camera make and model combined with other metadata can identify photographer"
                    .to_string(),
            );
            sensitive_fields.push("Camera Make/Model".to_string());
        }

        // Low: Lens information (5 points)
        if self.exif_data.contains_key("LensModel") || self.exif_data.contains_key("LensMake") {
            score += 5;
            warnings
                .push("Lens information may help identify photographer's equipment".to_string());
            sensitive_fields.push("Lens Information".to_string());
        }

        // Determine risk level based on score
        let level = if score >= 60 {
            PrivacyRiskLevel::Critical
        } else if score >= 40 {
            PrivacyRiskLevel::High
        } else if score >= 20 {
            PrivacyRiskLevel::Medium
        } else {
            PrivacyRiskLevel::Low
        };

        PrivacyRisk {
            level,
            score,
            warnings,
            sensitive_fields,
        }
    }
}
