mod app;
mod binary_cleaner;
mod components;
mod exif;
mod export;
mod metadata_info;
mod types;
mod utils;

use app::App;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<App>::new().render();
}
