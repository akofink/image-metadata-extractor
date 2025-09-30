//! WebAssembly front-end for extracting and cleaning metadata from images.
//!
//! The crate exposes a Yew application that runs entirely in the browser and
//! provides utilities for parsing metadata, presenting it to the user and
//! exporting or cleaning the underlying file.

pub mod app;
pub mod binary_cleaner;
pub mod components;
pub mod exif;
pub mod exif_core;
pub mod exif_wasm;
pub mod export;
pub mod gps_privacy;
pub mod metadata_info;
pub mod preferences;
pub mod types;
pub mod utils;
pub mod utils_core;
pub mod utils_hash;
pub mod utils_wasm;

use app::App;
use wasm_bindgen::prelude::*;

/// Entry point invoked by the generated JavaScript to start the Yew
/// application.
#[wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<App>::new().render();
}
