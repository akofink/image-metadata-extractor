pub mod app;
mod binary_cleaner;
pub mod components;
pub mod exif;
pub mod export;
pub mod image_cleaner;
mod metadata_info;
pub mod types;
pub mod utils;

use app::App;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<App>::new().render();
}
