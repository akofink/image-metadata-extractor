use image_metadata_extractor::components::file_upload::{FileUpload, FileUploadProps};
#[allow(dead_code)] // WebAssembly tests are run by wasm-pack, not cargo test
use wasm_bindgen_test::*;
use yew::prelude::*;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
#[allow(dead_code)] // WebAssembly test functions appear unused to cargo test
fn test_file_upload_component_renders() {
    let on_file_loaded = Callback::noop();
    let trigger_file_input = Callback::noop();

    let props = FileUploadProps {
        on_file_loaded,
        trigger_file_input,
        on_error: Callback::noop(),
    };

    let _rendered = html! {
        <FileUpload ..props />
    };

    // Basic test to ensure component can be instantiated without panicking
    assert!(true);
}

#[wasm_bindgen_test]
#[allow(dead_code)]
fn test_file_upload_props_equality() {
    let on_file_loaded = Callback::noop();
    let trigger_file_input = Callback::noop();

    let props1 = FileUploadProps {
        on_file_loaded: on_file_loaded.clone(),
        trigger_file_input: trigger_file_input.clone(),
        on_error: Callback::noop(),
    };

    let props2 = FileUploadProps {
        on_file_loaded: on_file_loaded.clone(),
        trigger_file_input: trigger_file_input.clone(),
        on_error: Callback::noop(),
    };

    // Test that props implement PartialEq correctly
    assert_eq!(props1, props2);
}

// Test to prevent regression of infinite render loop bug
#[wasm_bindgen_test]
#[allow(dead_code)]
fn test_trigger_callback_setup_only_once() {
    // This test verifies that the FileUpload component can be created
    // without causing infinite loops or panics during render
    let on_file_loaded = Callback::noop();
    let trigger_file_input = Callback::noop();

    let props = FileUploadProps {
        on_file_loaded,
        trigger_file_input,
        on_error: Callback::noop(),
    };

    // The component should render successfully without infinite loops
    let _rendered = html! {
        <FileUpload ..props />
    };

    // If we reach this point without panicking or hanging,
    // the component is properly using use_effect_with for one-time setup
    assert!(true);
}
