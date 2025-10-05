//! WASM-specific utility functions that require browser APIs.
//! This module contains all browser-dependent utility functions that cannot be tested in native environment.

use wasm_bindgen::prelude::*;
use web_sys::{HtmlAnchorElement, Url};

/// Check if the File System Access API (showSaveFilePicker) is available.
fn has_save_file_picker_support() -> bool {
    if let Some(window) = web_sys::window() {
        js_sys::Reflect::has(&window, &JsValue::from_str("showSaveFilePicker")).unwrap_or(false)
    } else {
        false
    }
}

/// Download a file using the modern File System Access API with showSaveFilePicker.
/// This allows users to choose the download location and filename.
async fn download_with_picker(blob: &web_sys::Blob, filename: &str) -> Result<(), JsValue> {
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("No window"))?;

    // Create options for the file picker
    let options = js_sys::Object::new();
    js_sys::Reflect::set(
        &options,
        &JsValue::from_str("suggestedName"),
        &JsValue::from_str(filename),
    )?;

    // Call showSaveFilePicker
    let picker_fn = js_sys::Reflect::get(&window, &JsValue::from_str("showSaveFilePicker"))?;
    let picker_fn: js_sys::Function = picker_fn.dyn_into()?;
    let file_handle_promise = picker_fn.call1(&window, &options)?;
    let file_handle =
        wasm_bindgen_futures::JsFuture::from(js_sys::Promise::from(file_handle_promise)).await?;

    // Create a writable stream
    let create_writable_fn =
        js_sys::Reflect::get(&file_handle, &JsValue::from_str("createWritable"))?;
    let create_writable_fn: js_sys::Function = create_writable_fn.dyn_into()?;
    let writable_promise = create_writable_fn.call0(&file_handle)?;
    let writable =
        wasm_bindgen_futures::JsFuture::from(js_sys::Promise::from(writable_promise)).await?;

    // Write the blob to the stream
    let write_fn = js_sys::Reflect::get(&writable, &JsValue::from_str("write"))?;
    let write_fn: js_sys::Function = write_fn.dyn_into()?;
    let write_promise = write_fn.call1(&writable, blob)?;
    wasm_bindgen_futures::JsFuture::from(js_sys::Promise::from(write_promise)).await?;

    // Close the stream
    let close_fn = js_sys::Reflect::get(&writable, &JsValue::from_str("close"))?;
    let close_fn: js_sys::Function = close_fn.dyn_into()?;
    let close_promise = close_fn.call0(&writable)?;
    wasm_bindgen_futures::JsFuture::from(js_sys::Promise::from(close_promise)).await?;

    Ok(())
}

/// Trigger a text file download in the browser (legacy fallback method).
pub fn download_file(content: &str, filename: &str, mime_type: &str) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    // Create blob and URL
    let blob_parts = js_sys::Array::new();
    blob_parts.push(&JsValue::from_str(content));

    let blob_options = web_sys::BlobPropertyBag::new();
    blob_options.set_type(mime_type);

    let blob =
        web_sys::Blob::new_with_str_sequence_and_options(&blob_parts, &blob_options).unwrap();

    let url = Url::create_object_url_with_blob(&blob).unwrap();

    // Create download link
    let link = document.create_element("a").unwrap();
    let link = link.dyn_into::<HtmlAnchorElement>().unwrap();

    link.set_href(&url);
    link.set_download(filename);
    link.style().set_property("display", "none").unwrap();

    // Trigger download
    document.body().unwrap().append_child(&link).unwrap();
    link.click();
    document.body().unwrap().remove_child(&link).unwrap();

    // Clean up URL
    Url::revoke_object_url(&url).unwrap();
}

/// Copy a string to the clipboard using the async Clipboard API, with a fallback to a hidden textarea.
pub fn copy_to_clipboard(text: &str) {
    if let Some(window) = web_sys::window() {
        // Use async Clipboard API; if unavailable at runtime, nothing happens.
        let clipboard = window.navigator().clipboard();
        let s = text.to_string();
        wasm_bindgen_futures::spawn_local(async move {
            let _ = wasm_bindgen_futures::JsFuture::from(clipboard.write_text(&s)).await;
        });
    }
}

/// Trigger a binary file download in the browser (legacy fallback method).
pub fn download_binary_file(bytes: &[u8], filename: &str, mime_type: &str) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    // Create Uint8Array from bytes
    let uint8_array = js_sys::Uint8Array::new_with_length(bytes.len() as u32);
    uint8_array.copy_from(bytes);

    // Create blob and URL
    let blob_parts = js_sys::Array::new();
    blob_parts.push(&uint8_array);

    let blob_options = web_sys::BlobPropertyBag::new();
    blob_options.set_type(mime_type);

    let blob =
        web_sys::Blob::new_with_u8_array_sequence_and_options(&blob_parts, &blob_options).unwrap();

    let url = Url::create_object_url_with_blob(&blob).unwrap();

    // Create download link
    let link = document.create_element("a").unwrap();
    let link = link.dyn_into::<HtmlAnchorElement>().unwrap();

    link.set_href(&url);
    link.set_download(filename);
    link.style().set_property("display", "none").unwrap();

    // Trigger download
    document.body().unwrap().append_child(&link).unwrap();
    link.click();
    document.body().unwrap().remove_child(&link).unwrap();

    // Clean up URL
    Url::revoke_object_url(&url).unwrap();
}

/// Download a text file using the modern File System Access API when available,
/// falling back to the legacy anchor element method on unsupported browsers.
///
/// This provides a better user experience on Chrome/Edge by allowing users to:
/// - Choose the exact download location
/// - See a native save dialog
/// - Overwrite existing files without duplicate naming
pub fn download_file_with_picker(content: &str, filename: &str, mime_type: &str) {
    let content = content.to_string();
    let filename = filename.to_string();
    let mime_type = mime_type.to_string();

    wasm_bindgen_futures::spawn_local(async move {
        // Create blob
        let blob_parts = js_sys::Array::new();
        blob_parts.push(&JsValue::from_str(&content));

        let blob_options = web_sys::BlobPropertyBag::new();
        blob_options.set_type(&mime_type);

        let blob =
            match web_sys::Blob::new_with_str_sequence_and_options(&blob_parts, &blob_options) {
                Ok(b) => b,
                Err(_) => {
                    // Fallback on blob creation failure
                    download_file(&content, &filename, &mime_type);
                    return;
                }
            };

        // Try modern API first
        if has_save_file_picker_support() {
            if download_with_picker(&blob, &filename).await.is_err() {
                // User cancelled or error occurred, fall back to legacy method
                download_file(&content, &filename, &mime_type);
            }
        } else {
            // Browser doesn't support File System Access API, use legacy method
            download_file(&content, &filename, &mime_type);
        }
    });
}

/// Download a binary file using the modern File System Access API when available,
/// falling back to the legacy anchor element method on unsupported browsers.
///
/// This provides a better user experience on Chrome/Edge by allowing users to:
/// - Choose the exact download location
/// - See a native save dialog
/// - Overwrite existing files without duplicate naming
pub fn download_binary_file_with_picker(bytes: &[u8], filename: &str, mime_type: &str) {
    let bytes = bytes.to_vec();
    let filename = filename.to_string();
    let mime_type = mime_type.to_string();

    wasm_bindgen_futures::spawn_local(async move {
        // Create Uint8Array from bytes
        let uint8_array = js_sys::Uint8Array::new_with_length(bytes.len() as u32);
        uint8_array.copy_from(&bytes);

        // Create blob
        let blob_parts = js_sys::Array::new();
        blob_parts.push(&uint8_array);

        let blob_options = web_sys::BlobPropertyBag::new();
        blob_options.set_type(&mime_type);

        let blob =
            match web_sys::Blob::new_with_u8_array_sequence_and_options(&blob_parts, &blob_options)
            {
                Ok(b) => b,
                Err(_) => {
                    // Fallback on blob creation failure
                    download_binary_file(&bytes, &filename, &mime_type);
                    return;
                }
            };

        // Try modern API first
        if has_save_file_picker_support() {
            if download_with_picker(&blob, &filename).await.is_err() {
                // User cancelled or error occurred, fall back to legacy method
                download_binary_file(&bytes, &filename, &mime_type);
            }
        } else {
            // Browser doesn't support File System Access API, use legacy method
            download_binary_file(&bytes, &filename, &mime_type);
        }
    });
}
