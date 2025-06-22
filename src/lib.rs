//! WebAssembly front-end for extracting and cleaning metadata from images.
//!
//! The crate exposes a Yew application that runs entirely in the browser and
//! provides utilities for parsing metadata, presenting it to the user and
//! exporting or cleaning the underlying file.

pub mod app;
pub mod binary_cleaner;
pub mod components;
pub mod exif;
pub mod export;
pub mod metadata_info;
pub mod types;
pub mod utils;

use app::App;
use wasm_bindgen::prelude::*;

/// Entry point invoked by the generated JavaScript to start the Yew
/// application.
#[wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<App>::new().render();
}
