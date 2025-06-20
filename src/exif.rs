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

pub async fn file_bytes(file: &File) -> Result<Vec<u8>, JsValue> {
    let array_buffer = JsFuture::from(file.array_buffer()).await?;
    let uint8_array = Uint8Array::new(&array_buffer);
    Ok(uint8_array.to_vec())
}

pub fn determine_mime_type(name: &str, file_type: &str, bytes: &[u8]) -> String {
    if !file_type.is_empty() {
        return file_type.to_string();
    }

    match image::guess_format(bytes) {
        Ok(image::ImageFormat::Png) => "image/png".into(),
        Ok(image::ImageFormat::Jpeg) => "image/jpeg".into(),
        Ok(image::ImageFormat::Gif) => "image/gif".into(),
        Ok(image::ImageFormat::WebP) => "image/webp".into(),
        _ => {
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
    }
}

pub fn is_supported_mime_type(mime: &str) -> bool {
    const SUPPORTED: &[&str] = &[
        "image/png",
        "image/jpeg",
        "image/gif",
        "image/webp",
        "application/pdf",
        "image/svg+xml",
        "image/tiff",
        "image/heif",
        "image/avif",
        "image/jxl",
    ];
    SUPPORTED.contains(&mime)
}

pub fn create_data_url(mime: &str, bytes: &[u8]) -> String {
    format!("data:{};base64,{}", mime, base64_encode(bytes))
}

pub fn get_dimensions(mime: &str, bytes: &[u8]) -> (Option<u32>, Option<u32>) {
    if mime.starts_with("image/") && mime != "image/svg+xml" {
        match get_image_dimensions(bytes) {
            Ok(dims) => (Some(dims.0), Some(dims.1)),
            Err(_) => (None, None),
        }
    } else {
        (None, None)
    }
}
pub async fn process_file(file: File) -> Result<ImageData, JsValue> {
    let name = file.name();
    let size = file.size() as u64;

    let bytes = file_bytes(&file).await?;
    let mime_type = determine_mime_type(&name, &file.type_(), &bytes);
    if !is_supported_mime_type(&mime_type) {
        return Err(JsValue::from_str("Unsupported file type"));
    }

    let data_url = create_data_url(&mime_type, &bytes);
    let (width, height) = get_dimensions(&mime_type, &bytes);
    let (exif_data, gps_coords) = extract_exif_data(&bytes);

    Ok(ImageData {
        name,
        size,
        mime_type,
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

pub fn extract_exif_data(bytes: &[u8]) -> (HashMap<String, String>, Option<(f64, f64)>) {
    let mut exif_map = HashMap::new();
    let mut gps_coords = None;

    if let Ok(exifreader) = Reader::new().read_from_container(&mut Cursor::new(bytes)) {
        for f in exifreader.fields() {
            let tag_name = format!("{}", f.tag);
            let value = format!("{}", f.display_value().with_unit(&exifreader));
            exif_map.insert(tag_name, value);
            update_gps_coords(&mut gps_coords, f, &exifreader);
        }

        if let Some(mut coords) = gps_coords {
            apply_gps_ref(&exifreader, &mut coords);
            gps_coords = Some(coords);
        }
    }

    (exif_map, gps_coords)
}

fn update_gps_coords(coords: &mut Option<(f64, f64)>, field: &Field, reader: &Exif) {
    match field.tag {
        Tag::GPSLatitude => {
            if let Some(lat) = parse_gps_coordinate(field, reader) {
                if let Some((_, lon)) = *coords {
                    *coords = Some((lat, lon));
                } else {
                    *coords = Some((lat, 0.0));
                }
            }
        }
        Tag::GPSLongitude => {
            if let Some(lon) = parse_gps_coordinate(field, reader) {
                if let Some((lat, _)) = *coords {
                    *coords = Some((lat, lon));
                } else {
                    *coords = Some((0.0, lon));
                }
            }
        }
        _ => {}
    }
}

fn apply_gps_ref(exif: &Exif, coords: &mut (f64, f64)) {
    let (lat_ref, lon_ref) = (
        exif.get_field(Tag::GPSLatitudeRef, In::PRIMARY),
        exif.get_field(Tag::GPSLongitudeRef, In::PRIMARY),
    );

    if let Some(field) = lat_ref {
        if let Value::Ascii(ref vec) = field.value {
            if let Some(val) = vec.first() {
                if !val.is_empty() && val[0] == b'S' {
                    coords.0 = -coords.0;
                }
            }
        }
    }

    if let Some(field) = lon_ref {
        if let Value::Ascii(ref vec) = field.value {
            if let Some(val) = vec.first() {
                if !val.is_empty() && val[0] == b'W' {
                    coords.1 = -coords.1;
                }
            }
        }
    }
}

pub fn parse_gps_coordinate(field: &Field, _exifreader: &Exif) -> Option<f64> {
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
