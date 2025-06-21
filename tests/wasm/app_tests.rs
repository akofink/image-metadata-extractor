use gloo::utils::document;
use image_metadata_extractor::app::App;
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;
use yew::prelude::*;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn test_app_renders_heading_and_placeholder() {
    let div = document().create_element("div").unwrap();
    document().body().unwrap().append_child(&div).unwrap();

    yew::Renderer::<App>::with_root(div.clone().unchecked_into()).render();
    gloo::timers::future::TimeoutFuture::new(0).await;

    let html = div.inner_html();
    assert!(html.contains("File Metadata Extractor"));
    assert!(html.contains("Click here to select a file"));
}

#[wasm_bindgen_test]
async fn test_app_footer_present() {
    let div = document().create_element("div").unwrap();
    document().body().unwrap().append_child(&div).unwrap();

    yew::Renderer::<App>::with_root(div.clone().unchecked_into()).render();
    gloo::timers::future::TimeoutFuture::new(0).await;

    let html = div.inner_html();
    assert!(html.contains("Privacy-First"));
    assert!(html.contains("Open Source"));
}
