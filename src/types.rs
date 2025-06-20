use serde::Serialize;
use std::collections::{HashMap, HashSet};

#[derive(Clone, PartialEq, Serialize)]
pub struct ImageData {
    pub name: String,
    pub size: u64,
    #[serde(skip)] // Don't include data URL in exports
    pub data_url: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub exif_data: HashMap<String, String>,
    pub gps_coords: Option<(f64, f64)>, // (latitude, longitude)
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
            name: self.name.clone(),
            size: if include_basic_info { self.size } else { 0 },
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
