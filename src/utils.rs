use wasm_bindgen::prelude::*;
use web_sys::{HtmlAnchorElement, Url};

pub fn base64_encode(bytes: &[u8]) -> String {
    let window = web_sys::window().unwrap();
    let btoa = js_sys::Reflect::get(&window, &JsValue::from_str("btoa")).unwrap();
    let btoa_fn: js_sys::Function = btoa.unchecked_into();

    // Convert bytes to string for btoa
    let binary_string = bytes.iter().map(|b| *b as char).collect::<String>();
    let result = btoa_fn
        .call1(&window, &JsValue::from_str(&binary_string))
        .unwrap();
    result.as_string().unwrap()
}

pub fn format_file_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    format!("{:.1} {}", size, UNITS[unit_index])
}

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
