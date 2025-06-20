use crate::types::ImageData;
use crate::utils::base64_encode;
use exif::{Exif, Field, In, Reader, Tag, Value};
use image::GenericImageView;
use js_sys::Uint8Array;
use std::collections::HashMap;
use std::io::Cursor;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::File;

pub async fn process_file(file: File) -> Result<ImageData, JsValue> {
    let name = file.name();
    let size = file.size() as u64;

    // Create data URL for image display
    let array_buffer = JsFuture::from(file.array_buffer()).await?;
    let uint8_array = Uint8Array::new(&array_buffer);
    let bytes = uint8_array.to_vec();

    // Determine MIME type from file or guess from extension/content
    let mut mime_type = file.type_();
    if mime_type.is_empty() {
        // Try image format detection first
        mime_type = match image::guess_format(&bytes) {
            Ok(image::ImageFormat::Png) => "image/png".into(),
            Ok(image::ImageFormat::Jpeg) => "image/jpeg".into(),
            Ok(image::ImageFormat::Gif) => "image/gif".into(),
            Ok(image::ImageFormat::WebP) => "image/webp".into(),
            _ => {
                // Guess from file extension for non-image files
                let name_lower = name.to_lowercase();
                if name_lower.ends_with(".pdf") {
                    "application/pdf".into()
                } else if name_lower.ends_with(".svg") {
                    "image/svg+xml".into()
                } else if name_lower.ends_with(".tiff") || name_lower.ends_with(".tif") {
                    "image/tiff".into()
                } else if name_lower.ends_with(".heif") || name_lower.ends_with(".heic") {
                    "image/heif".into()
                } else if name_lower.ends_with(".avif") {
                    "image/avif".into()
                } else if name_lower.ends_with(".jxl") {
                    "image/jxl".into()
                } else {
                    "application/octet-stream".into()
                }
            }
        };
    }

    // Create data URL
    let data_url = format!("data:{};base64,{}", mime_type, base64_encode(&bytes));

    // Get dimensions (only for image files)
    let (width, height) = if mime_type.starts_with("image/") && mime_type != "image/svg+xml" {
        match get_image_dimensions(&bytes) {
            Ok(dims) => (Some(dims.0), Some(dims.1)),
            Err(_) => (None, None), // Non-image or unsupported format
        }
    } else {
        (None, None) // Non-image files don't have pixel dimensions
    };

    // Extract EXIF data
    let (exif_data, gps_coords) = extract_exif_data(&bytes);

    Ok(ImageData {
        name,
        size,
        mime_type: mime_type.clone(),
        data_url,
        width,
        height,
        exif_data,
        gps_coords,
    })
}

fn get_image_dimensions(bytes: &[u8]) -> Result<(u32, u32), JsValue> {
    // Use the image crate to get dimensions
    match image::load_from_memory(bytes) {
        Ok(img) => {
            let dimensions = img.dimensions();
            Ok(dimensions)
        }
        Err(_) => Err(JsValue::from_str("Failed to parse image")),
    }
}

fn extract_exif_data(bytes: &[u8]) -> (HashMap<String, String>, Option<(f64, f64)>) {
    let mut exif_map = HashMap::new();
    let mut gps_coords = None;

    // Try to parse EXIF data
    if let Ok(exifreader) = Reader::new().read_from_container(&mut Cursor::new(bytes)) {
        for f in exifreader.fields() {
            let tag_name = format!("{}", f.tag);
            let value = format!("{}", f.display_value().with_unit(&exifreader));

            // Store the EXIF field
            exif_map.insert(tag_name.clone(), value);

            // Check for GPS coordinates
            match f.tag {
                Tag::GPSLatitude => {
                    if let Some(lat) = parse_gps_coordinate(f, &exifreader) {
                        if let Some((_, lon)) = gps_coords {
                            gps_coords = Some((lat, lon));
                        } else {
                            gps_coords = Some((lat, 0.0));
                        }
                    }
                }
                Tag::GPSLongitude => {
                    if let Some(lon) = parse_gps_coordinate(f, &exifreader) {
                        if let Some((lat, _)) = gps_coords {
                            gps_coords = Some((lat, lon));
                        } else {
                            gps_coords = Some((0.0, lon));
                        }
                    }
                }
                _ => {}
            }
        }

        // Apply GPS reference directions
        if let Some((mut lat, mut lon)) = gps_coords {
            // Check latitude reference (N/S)
            if let Some(lat_ref_field) = exifreader.get_field(Tag::GPSLatitudeRef, In::PRIMARY) {
                if let Value::Ascii(ref vec) = lat_ref_field.value {
                    if let Some(lat_ref) = vec.first() {
                        if !lat_ref.is_empty() && lat_ref[0] == b'S' {
                            lat = -lat;
                        }
                    }
                }
            }

            // Check longitude reference (E/W)
            if let Some(lon_ref_field) = exifreader.get_field(Tag::GPSLongitudeRef, In::PRIMARY) {
                if let Value::Ascii(ref vec) = lon_ref_field.value {
                    if let Some(lon_ref) = vec.first() {
                        if !lon_ref.is_empty() && lon_ref[0] == b'W' {
                            lon = -lon;
                        }
                    }
                }
            }

            gps_coords = Some((lat, lon));
        }
    }

    (exif_map, gps_coords)
}

fn parse_gps_coordinate(field: &Field, _exifreader: &Exif) -> Option<f64> {
    if let Value::Rational(ref rationals) = field.value {
        if rationals.len() >= 3 {
            let degrees = rationals[0].to_f64();
            let minutes = rationals[1].to_f64();
            let seconds = rationals[2].to_f64();

            Some(degrees + minutes / 60.0 + seconds / 3600.0)
        } else {
            None
        }
    } else {
        None
    }
}
