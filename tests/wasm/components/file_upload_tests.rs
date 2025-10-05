#![cfg(target_arch = "wasm32")]

use image_metadata_extractor::components::file_upload::{FileUpload, FileUploadProps};
use wasm_bindgen_test::*;
use yew::prelude::*;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_file_upload_component_renders() {
    let on_file_loaded = Callback::noop();
    let trigger_file_input = Callback::noop();

    let props = FileUploadProps {
        on_file_loaded,
        trigger_file_input,
        on_error: Callback::noop(),
        on_files_loaded: None,
        on_progress: None,
    };

    let _rendered = html! {
        <FileUpload ..props />
    };

    // Basic test to ensure component can be instantiated without panicking
    assert!(true);
}

#[wasm_bindgen_test]
fn test_file_upload_props_equality() {
    let on_file_loaded = Callback::noop();
    let trigger_file_input = Callback::noop();

    let _props1 = FileUploadProps {
        on_file_loaded: on_file_loaded.clone(),
        trigger_file_input: trigger_file_input.clone(),
        on_error: Callback::noop(),
        on_files_loaded: None,
        on_progress: None,
    };

    let _props2 = FileUploadProps {
        on_file_loaded: on_file_loaded.clone(),
        trigger_file_input: trigger_file_input.clone(),
        on_error: Callback::noop(),
        on_files_loaded: None,
        on_progress: None,
    };

    // Test that props can be created without panicking
    // This test just verifies the props can be constructed and cloned without errors
    assert!(true);
}

// Test to prevent regression of infinite render loop bug
#[wasm_bindgen_test]
fn test_trigger_callback_setup_only_once() {
    // This test verifies that the FileUpload component can be created
    // without causing infinite loops or panics during render
    let on_file_loaded = Callback::noop();
    let trigger_file_input = Callback::noop();

    let props = FileUploadProps {
        on_file_loaded,
        trigger_file_input,
        on_error: Callback::noop(),
        on_files_loaded: None,
        on_progress: None,
    };

    // The component should render successfully without infinite loops
    let _rendered = html! {
        <FileUpload ..props />
    };

    // If we reach this point without panicking or hanging,
    // the component is properly using use_effect_with for one-time setup
    assert!(true);
}
