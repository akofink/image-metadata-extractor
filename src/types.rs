use serde::Serialize;
use std::collections::{HashMap, HashSet};

#[derive(Clone, PartialEq, Serialize)]
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
}

fn is_zero(value: &u64) -> bool {
    *value == 0
}

impl ImageData {
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
        }
    }
}
