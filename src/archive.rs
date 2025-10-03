//! Archive processing module for extracting images from ZIP files.
//!
//! This module provides client-side ZIP extraction functionality, enabling
//! users to upload entire archives of images for batch metadata processing
//! without requiring server-side processing.

use gloo_file::Blob;
use std::io::Cursor;
use wasm_bindgen::JsValue;
use web_sys::File;
use zip::ZipArchive;

/// Represents a file extracted from an archive
#[derive(Clone, Debug)]
pub struct ArchivedFile {
    pub name: String,
    pub blob: Blob,
}

/// Extract image files from a ZIP archive
///
/// # Arguments
/// * `file` - The ZIP file to extract from
///
/// # Returns
/// A vector of [`ArchivedFile`] entries containing image files only
///
/// # Errors
/// Returns JsValue error if the ZIP is corrupted or cannot be read
pub async fn extract_images_from_zip(file: File) -> Result<Vec<ArchivedFile>, JsValue> {
    // Read the file as bytes
    let array_buffer = gloo_file::futures::read_as_bytes(&file.into())
        .await
        .map_err(|e| JsValue::from_str(&format!("Failed to read ZIP file: {:?}", e)))?;

    // Create a cursor for the zip reader
    let cursor = Cursor::new(array_buffer);

    // Open the ZIP archive
    let mut archive = ZipArchive::new(cursor)
        .map_err(|e| JsValue::from_str(&format!("Invalid ZIP file: {}", e)))?;

    let mut results = Vec::new();

    // Iterate through all entries in the archive
    for i in 0..archive.len() {
        let mut entry = archive
            .by_index(i)
            .map_err(|e| JsValue::from_str(&format!("Failed to read ZIP entry: {}", e)))?;

        // Skip directories
        if entry.is_dir() {
            continue;
        }

        let name = entry.name().to_string();

        // Only process image files based on extension
        if !is_image_file(&name) {
            continue;
        }

        // Read the file contents
        let mut contents = Vec::new();
        std::io::copy(&mut entry, &mut contents)
            .map_err(|e| JsValue::from_str(&format!("Failed to extract {}: {}", name, e)))?;

        // Determine MIME type from extension
        let mime_type = get_mime_type(&name);

        // Create a Blob from the contents
        let blob = Blob::new_with_options(contents.as_slice(), Some(mime_type));

        results.push(ArchivedFile { name, blob });
    }

    Ok(results)
}

/// Check if a filename represents an image file
pub fn is_image_file(filename: &str) -> bool {
    let lower = filename.to_lowercase();
    lower.ends_with(".jpg")
        || lower.ends_with(".jpeg")
        || lower.ends_with(".png")
        || lower.ends_with(".gif")
        || lower.ends_with(".webp")
        || lower.ends_with(".tiff")
        || lower.ends_with(".tif")
        || lower.ends_with(".heif")
        || lower.ends_with(".heic")
        || lower.ends_with(".avif")
        || lower.ends_with(".jxl")
        || lower.ends_with(".svg")
        || lower.ends_with(".pdf")
}

/// Get MIME type from filename extension
pub fn get_mime_type(filename: &str) -> &'static str {
    let lower = filename.to_lowercase();
    if lower.ends_with(".jpg") || lower.ends_with(".jpeg") {
        "image/jpeg"
    } else if lower.ends_with(".png") {
        "image/png"
    } else if lower.ends_with(".gif") {
        "image/gif"
    } else if lower.ends_with(".webp") {
        "image/webp"
    } else if lower.ends_with(".tiff") || lower.ends_with(".tif") {
        "image/tiff"
    } else if lower.ends_with(".heif") || lower.ends_with(".heic") {
        "image/heif"
    } else if lower.ends_with(".avif") {
        "image/avif"
    } else if lower.ends_with(".jxl") {
        "image/jxl"
    } else if lower.ends_with(".svg") {
        "image/svg+xml"
    } else if lower.ends_with(".pdf") {
        "application/pdf"
    } else {
        "application/octet-stream"
    }
}
