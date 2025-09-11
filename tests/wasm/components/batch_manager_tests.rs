#![cfg(target_arch = "wasm32")]

use image_metadata_extractor::components::batch_manager::{BatchManager, BatchManagerProps};
use wasm_bindgen_test::*;
use yew::prelude::*;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_batch_manager_hidden_when_inactive() {
    let props = BatchManagerProps {
        in_progress: false,
        processed: 0,
        total: 0,
        last_file: None,
    };

    let rendered = html! { <BatchManager ..props.clone() /> };
    // Rendering should not panic; hidden state returns empty html
    let _ = rendered;
}

#[wasm_bindgen_test]
fn test_batch_manager_renders_when_active() {
    let props = BatchManagerProps {
        in_progress: true,
        processed: 1,
        total: 3,
        last_file: None,
    };

    let rendered = html! { <BatchManager ..props.clone() /> };
    let _ = rendered; // Just ensure it renders without panic
}
