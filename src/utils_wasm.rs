//! WASM-specific utility functions that require browser APIs.
//! This module contains all browser-dependent utility functions that cannot be tested in native environment.

use wasm_bindgen::prelude::*;
use web_sys::{HtmlAnchorElement, Url};

/// Trigger a text file download in the browser.
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

/// Trigger a binary file download in the browser.
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
