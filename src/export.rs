//! Helpers for exporting selected metadata in various formats.

use crate::types::ImageData;
use crate::utils::format_file_size;
use std::fmt::Write as _;

fn sorted_exif_pairs(data: &ImageData) -> Vec<(String, String)> {
    let mut v: Vec<(String, String)> = data
        .exif_data
        .iter()
        .map(|(k, val)| (k.clone(), val.clone()))
        .collect();
    v.sort_by(|a, b| a.0.cmp(&b.0));
    v
}

/// Create a CSV representation of the provided [`ImageData`].
pub fn generate_csv(data: &ImageData) -> String {
    let mut csv = String::new();
    csv.push_str("Property,Value\n");

    // Basic file info
    csv.push_str(&format!("Filename,\"{}\"\n", data.name));
    csv.push_str(&format!("File Size,{}\n", format_file_size(data.size)));

    if let (Some(width), Some(height)) = (data.width, data.height) {
        csv.push_str(&format!("Width,{}\n", width));
        csv.push_str(&format!("Height,{}\n", height));
        csv.push_str(&format!("Dimensions,\"{}x{} pixels\"\n", width, height));
    }

    // GPS data
    if let Some((lat, lon)) = data.gps_coords {
        csv.push_str(&format!("GPS Latitude,{}\n", lat));
        csv.push_str(&format!("GPS Longitude,{}\n", lon));
    }

    // EXIF data (sorted for deterministic output)
    for (key, value) in sorted_exif_pairs(data) {
        let mut esc_key = String::new();
        let mut esc_val = String::new();
        for ch in key.chars() {
            if ch == '"' {
                esc_key.push('"');
            }
            esc_key.push(ch);
        }
        for ch in value.chars() {
            if ch == '"' {
                esc_val.push('"');
            }
            esc_val.push(ch);
        }
        let _ = writeln!(csv, "\"{}\",\"{}\"", esc_key, esc_val);
    }

    csv
}

/// Produce a human readable text report for the selected metadata.
pub fn generate_txt(data: &ImageData) -> String {
    let mut txt = String::new();
    txt.push_str("FILE METADATA REPORT\n");
    txt.push_str("====================\n\n");

    // Basic file info
    txt.push_str("FILE INFORMATION\n");
    txt.push_str("----------------\n");
    txt.push_str(&format!("Filename: {}\n", data.name));
    txt.push_str(&format!("File Size: {}\n", format_file_size(data.size)));

    if let (Some(width), Some(height)) = (data.width, data.height) {
        txt.push_str(&format!("Dimensions: {}x{} pixels\n", width, height));
    }
    txt.push('\n');

    // GPS data
    if let Some((lat, lon)) = data.gps_coords {
        txt.push_str("GPS LOCATION\n");
        txt.push_str("------------\n");
        txt.push_str(&format!("Latitude: {}\n", lat));
        txt.push_str(&format!("Longitude: {}\n", lon));
        txt.push_str(&format!(
            "Google Maps: https://maps.google.com/maps?q={},{}\n",
            lat, lon
        ));
        txt.push_str(&format!(
            "Apple Maps: https://maps.apple.com/?ll={},{}\n",
            lat, lon
        ));
        txt.push_str(&format!(
            "OpenStreetMap: https://www.openstreetmap.org/?mlat={}&mlon={}\n\n",
            lat, lon
        ));
    }

    // EXIF data
    if !data.exif_data.is_empty() {
        txt.push_str("METADATA\n");
        txt.push_str("--------\n");
        for (key, value) in sorted_exif_pairs(data) {
            txt.push_str(&format!("{}: {}\n", key, value));
        }
    } else {
        txt.push_str("METADATA\n");
        txt.push_str("--------\n");
        txt.push_str("No metadata found in this file\n");
    }

    txt
}
