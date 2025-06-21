use image_metadata_extractor::exif::process_file;
use js_sys::{Array, Uint8Array};
use wasm_bindgen_test::*;
use web_sys::File;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn test_unsupported_file_error() {
    let bytes = Uint8Array::new_with_length(4);
    bytes.copy_from(&[1, 2, 3, 4]);
    let parts = Array::new();
    parts.push(&bytes.buffer());
    let bag = web_sys::FilePropertyBag::new();
    bag.set_type("application/octet-stream");
    let file = File::new_with_u8_array_sequence_and_options(&parts, "test.bin", &bag).unwrap();

    let result = process_file(file).await;
    assert!(result.is_err());
}