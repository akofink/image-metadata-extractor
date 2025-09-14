//! SHA-256 file hashing utilities using Web Crypto API.

use wasm_bindgen::prelude::*;

/// Calculate SHA-256 hash of file data using Web Crypto API.
pub async fn calculate_sha256_hash(data: &[u8]) -> Result<String, JsValue> {
    // Access crypto through the global crypto object
    let global = js_sys::global();
    let crypto = js_sys::Reflect::get(&global, &JsValue::from_str("crypto"))?;
    let crypto: web_sys::Crypto = crypto.dyn_into()?;
    let subtle = crypto.subtle();

    // Calculate SHA-256 hash directly from the byte slice
    let hash_promise = subtle.digest_with_str_and_u8_array("SHA-256", data)?;
    let hash_result = wasm_bindgen_futures::JsFuture::from(hash_promise).await?;
    let hash_array = js_sys::Uint8Array::new(&hash_result);
    let hash_bytes = hash_array.to_vec();

    // Convert to hex string
    let hex_string = hash_bytes
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect::<String>();

    Ok(hex_string)
}

/// Synchronous wrapper for SHA-256 hashing that spawns an async task.
/// Calls the provided callback with the hash string when complete.
pub fn calculate_sha256_hash_sync(data: Vec<u8>, callback: js_sys::Function) {
    wasm_bindgen_futures::spawn_local(async move {
        match calculate_sha256_hash(&data).await {
            Ok(hash) => {
                let _ = callback.call1(&JsValue::NULL, &JsValue::from_str(&hash));
            }
            Err(_) => {
                let _ = callback.call1(&JsValue::NULL, &JsValue::from_str(""));
            }
        }
    });
}
