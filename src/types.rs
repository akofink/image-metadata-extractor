use serde::Serialize;
use std::collections::HashMap;

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
