use crate::types::ImageData;
use crate::utils::format_file_size;

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

    // EXIF data
    for (key, value) in &data.exif_data {
        csv.push_str(&format!(
            "\"{}\",\"{}\"\n",
            key.replace("\"", "\"\""),
            value.replace("\"", "\"\"")
        ));
    }

    csv
}

pub fn generate_txt(data: &ImageData) -> String {
    let mut txt = String::new();
    txt.push_str("IMAGE METADATA REPORT\n");
    txt.push_str("=====================\n\n");

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
            "Google Maps: https://maps.google.com/maps?q={},{}\n\n",
            lat, lon
        ));
    }

    // EXIF data
    if !data.exif_data.is_empty() {
        txt.push_str("EXIF METADATA\n");
        txt.push_str("-------------\n");
        for (key, value) in &data.exif_data {
            txt.push_str(&format!("{}: {}\n", key, value));
        }
    } else {
        txt.push_str("EXIF METADATA\n");
        txt.push_str("-------------\n");
        txt.push_str("No EXIF data found in this image\n");
    }

    txt
}
