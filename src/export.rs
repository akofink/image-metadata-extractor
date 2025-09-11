//! Helpers for exporting selected metadata in various formats.

use crate::types::ImageData;
use crate::utils::format_file_size;
use std::fmt::Write as _;

fn xml_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&apos;"),
            _ => out.push(ch),
        }
    }
    out
}

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

/// Generate a Markdown report (deterministic ordering).
pub fn generate_md(data: &ImageData) -> String {
    let mut out = String::new();
    out.push_str("# File Metadata\n\n");
    out.push_str("## File Information\n");
    let _ = writeln!(out, "- Filename: {}", data.name);
    let _ = writeln!(out, "- File Size: {}", format_file_size(data.size));
    if let (Some(w), Some(h)) = (data.width, data.height) {
        let _ = writeln!(out, "- Dimensions: {}x{} pixels", w, h);
    }
    out.push('\n');

    if let Some((lat, lon)) = data.gps_coords {
        out.push_str("## GPS Location\n");
        let _ = writeln!(out, "- Latitude: {}", lat);
        let _ = writeln!(out, "- Longitude: {}", lon);
        let _ = writeln!(
            out,
            "- Google Maps: https://maps.google.com/maps?q={},{}",
            lat, lon
        );
        let _ = writeln!(
            out,
            "- Apple Maps: https://maps.apple.com/?ll={},{}",
            lat, lon
        );
        let _ = writeln!(
            out,
            "- OpenStreetMap: https://www.openstreetmap.org/?mlat={}&mlon={}",
            lat, lon
        );
        out.push('\n');
    }

    out.push_str("## Metadata\n");
    for (k, v) in sorted_exif_pairs(data) {
        let _ = writeln!(out, "- {}: {}", k, v);
    }
    out
}

/// Generate a YAML document (deterministic ordering).
pub fn generate_yaml(data: &ImageData) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "name: {}", data.name);
    let _ = writeln!(out, "size: {}", data.size);
    if let (Some(w), Some(h)) = (data.width, data.height) {
        let _ = writeln!(out, "dimensions: \"{}x{}\"", w, h);
    }
    if let Some((lat, lon)) = data.gps_coords {
        let _ = writeln!(out, "gps:");
        let _ = writeln!(out, "  lat: {}", lat);
        let _ = writeln!(out, "  lon: {}", lon);
    }
    if !data.exif_data.is_empty() {
        out.push_str("exif:\n");
        for (k, v) in sorted_exif_pairs(data) {
            let _ = writeln!(
                out,
                "  \"{}\": \"{}\"",
                k.replace('"', "\\\""),
                v.replace('"', "\\\"")
            );
        }
    }
    out
}

/// Generate a minimal XML export (deterministic ordering).
pub fn generate_xml(data: &ImageData) -> String {
    let mut out = String::new();
    out.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    out.push_str("<metadata>\n");
    let _ = writeln!(out, "  <name>{}</name>", xml_escape(&data.name));
    let _ = writeln!(out, "  <size>{}</size>", data.size);
    if let (Some(w), Some(h)) = (data.width, data.height) {
        let _ = writeln!(out, "  <dimensions>{}x{}</dimensions>", w, h);
    }
    if let Some((lat, lon)) = data.gps_coords {
        out.push_str("  <gps>\n");
        let _ = writeln!(out, "    <lat>{}</lat>", lat);
        let _ = writeln!(out, "    <lon>{}</lon>", lon);
        out.push_str("  </gps>\n");
    }
    if !data.exif_data.is_empty() {
        out.push_str("  <exif>\n");
        for (k, v) in sorted_exif_pairs(data) {
            let _ = writeln!(
                out,
                "    <tag name=\"{}\">{}</tag>",
                xml_escape(&k),
                xml_escape(&v)
            );
        }
        out.push_str("  </exif>\n");
    }
    out.push_str("</metadata>\n");
    out
}
