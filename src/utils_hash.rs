//! SHA-256 file hashing utilities using Web Crypto API.

use wasm_bindgen::prelude::*;

/// Calculate SHA-256 hash of file data using Web Crypto API.
/// This is the core async function that performs the actual hash calculation.
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

/// Calculate SHA-256 hash during browser idle time for better perceived performance.
/// Uses `requestIdleCallback` when available, falls back to immediate execution.
/// This prevents hash calculation from blocking UI rendering and user interactions.
pub async fn calculate_sha256_hash_idle(data: &[u8]) -> Result<String, JsValue> {
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("No window"))?;

    // Check if requestIdleCallback is available
    let has_idle_callback =
        js_sys::Reflect::has(&window, &JsValue::from_str("requestIdleCallback")).unwrap_or(false);

    if has_idle_callback {
        // Use idle callback for better UX - wait for browser idle time
        let (sender, receiver) = futures::channel::oneshot::channel();
        let data_clone = data.to_vec();

        let closure = Closure::once(Box::new(move || {
            wasm_bindgen_futures::spawn_local(async move {
                let result = calculate_sha256_hash(&data_clone).await;
                let _ = sender.send(result);
            });
        }) as Box<dyn FnOnce()>);

        // Call requestIdleCallback
        let request_idle_fn =
            js_sys::Reflect::get(&window, &JsValue::from_str("requestIdleCallback"))?;
        let request_idle_fn: js_sys::Function = request_idle_fn.dyn_into()?;
        request_idle_fn.call1(&window, closure.as_ref().unchecked_ref())?;
        closure.forget();

        // Wait for the result
        receiver
            .await
            .map_err(|_| JsValue::from_str("Channel canceled"))?
    } else {
        // Fallback: calculate immediately if requestIdleCallback not available
        calculate_sha256_hash(data).await
    }
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
