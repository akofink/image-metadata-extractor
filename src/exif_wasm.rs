//! WASM-specific EXIF functionality that requires browser APIs.
//! This module contains all browser-dependent functions that cannot be tested in native environment.

use crate::exif_core;
use crate::types::ImageData;
use crate::utils_hash::calculate_sha256_hash;
use image::GenericImageView;
use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::File;

/// Read the contents of a [`File`] into a byte vector.
pub async fn file_bytes(file: &File) -> Result<Vec<u8>, JsValue> {
    let array_buffer = JsFuture::from(file.array_buffer()).await?;
    let uint8_array = Uint8Array::new(&array_buffer);
    Ok(uint8_array.to_vec())
}

/// Create a data URL from raw bytes for preview in the browser.
pub fn create_data_url(mime: &str, bytes: &[u8]) -> String {
    use crate::utils_wasm::base64_encode;
    format!("data:{};base64,{}", mime, base64_encode(bytes))
}

/// Attempt to read image width and height from the byte stream.
/// This function requires WASM APIs and cannot be tested in native environment.
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

/// Convert an uploaded [`File`] into [`ImageData`].
pub async fn process_file(file: File) -> Result<ImageData, JsValue> {
    let name = file.name();
    let size = file.size() as u64;

    let bytes = file_bytes(&file).await?;
    let mime_type = exif_core::determine_mime_type(&name, &file.type_(), &bytes);
    if !exif_core::is_supported_mime_type(&mime_type) {
        return Err(JsValue::from_str("Unsupported file type"));
    }

    let data_url = create_data_url(&mime_type, &bytes);
    let (width, height) = get_dimensions(&mime_type, &bytes);
    let (exif_data, gps_coords) = exif_core::extract_exif_data(&bytes);

    // Calculate SHA-256 hash for forensics and deduplication
    let sha256_hash = calculate_sha256_hash(&bytes).await.ok();

    Ok(ImageData {
        name,
        size,
        mime_type,
        data_url,
        width,
        height,
        exif_data,
        gps_coords,
        sha256_hash,
    })
}
