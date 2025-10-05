//! Batch cleaning component that downloads multiple cleaned images as a ZIP file.

use crate::binary_cleaner::BinaryCleaner;
use crate::types::{ImageData, Theme};
use crate::utils::download_binary_file;
use base64::{Engine as _, engine::general_purpose};
use std::io::{Cursor, Write as _};
use std::rc::Rc;
use yew::prelude::*;
use zip::ZipWriter;
use zip::write::FileOptions;

struct BatchCleanerColors {
    background: &'static str,
    text: &'static str,
    border: &'static str,
    info_bg: &'static str,
    info_text: &'static str,
    button_bg: &'static str,
}

const LIGHT_BATCH_CLEANER_COLORS: BatchCleanerColors = BatchCleanerColors {
    background: "#d1ecf1",
    text: "#0c5460",
    border: "#bee5eb",
    info_bg: "rgba(255,255,255,0.7)",
    info_text: "#666",
    button_bg: "#17a2b8",
};

const DARK_BATCH_CLEANER_COLORS: BatchCleanerColors = BatchCleanerColors {
    background: "#1a4548",
    text: "#b8dce1",
    border: "#2d5a5f",
    info_bg: "rgba(255,255,255,0.1)",
    info_text: "#aaa",
    button_bg: "#20c997",
};

/// Properties for [`BatchCleaner`].
#[derive(Properties, PartialEq)]
pub struct BatchCleanerProps {
    pub batch_items: Vec<Rc<ImageData>>,
    pub theme: Theme,
}

/// Creates a ZIP archive containing cleaned versions of all uploaded images.
#[function_component(BatchCleaner)]
pub fn batch_cleaner(props: &BatchCleanerProps) -> Html {
    let colors = match props.theme {
        Theme::Light => LIGHT_BATCH_CLEANER_COLORS,
        Theme::Dark => DARK_BATCH_CLEANER_COLORS,
    };

    let is_processing = use_state(|| false);

    let download_batch_cleaned_cb = {
        let batch_items = props.batch_items.clone();
        let is_processing = is_processing.clone();

        Callback::from(move |_| {
            let batch_items = batch_items.clone();
            let is_processing = is_processing.clone();

            is_processing.set(true);

            wasm_bindgen_futures::spawn_local(async move {
                // Create a buffer to hold the ZIP file
                let buffer = Cursor::new(Vec::new());
                let mut zip = ZipWriter::new(buffer);
                let options = FileOptions::default()
                    .compression_method(zip::CompressionMethod::Deflated)
                    .compression_level(Some(6));

                let mut success_count = 0;
                let mut error_count = 0;

                for image_data in &batch_items {
                    let data_url = &image_data.data_url;
                    let filename = &image_data.name;

                    // Extract file extension
                    if let Some(file_extension) = filename.split('.').next_back()
                        && let Some(base64_data) = data_url.strip_prefix("data:image/")
                        && let Some(comma_pos) = base64_data.find(',')
                        && let Ok(file_bytes) =
                            general_purpose::STANDARD.decode(&base64_data[comma_pos + 1..])
                    {
                        match BinaryCleaner::clean_metadata(&file_bytes, file_extension) {
                            Ok(cleaned_bytes) => {
                                // Create cleaned filename
                                let cleaned_filename = filename
                                    .strip_suffix(&format!(".{}", file_extension))
                                    .unwrap_or(filename)
                                    .to_string()
                                    + "_cleaned."
                                    + file_extension;

                                // Add file to ZIP
                                if zip.start_file(cleaned_filename, options).is_ok() {
                                    if zip.write_all(&cleaned_bytes).is_ok() {
                                        success_count += 1;
                                    } else {
                                        error_count += 1;
                                        web_sys::console::log_1(
                                            &format!("Failed to write {} to ZIP", filename).into(),
                                        );
                                    }
                                } else {
                                    error_count += 1;
                                    web_sys::console::log_1(
                                        &format!("Failed to add {} to ZIP", filename).into(),
                                    );
                                }
                            }
                            Err(e) => {
                                error_count += 1;
                                web_sys::console::log_1(
                                    &format!("Failed to clean {}: {}", filename, e).into(),
                                );
                            }
                        }
                    } else {
                        error_count += 1;
                        web_sys::console::log_1(
                            &format!("Failed to process {} for cleaning", filename).into(),
                        );
                    }
                }

                // Finalize the ZIP
                if let Ok(buffer) = zip.finish() {
                    let zip_bytes = buffer.into_inner();

                    if success_count > 0 {
                        // Download the ZIP file
                        download_binary_file(&zip_bytes, "cleaned_images.zip", "application/zip");

                        web_sys::console::log_1(
                            &format!(
                                "Successfully cleaned {} files, {} errors",
                                success_count, error_count
                            )
                            .into(),
                        );
                    } else {
                        web_sys::console::log_1(&"No files were successfully cleaned".into());
                    }
                } else {
                    web_sys::console::log_1(&"Failed to finalize ZIP archive".into());
                }

                is_processing.set(false);
            });
        })
    };

    // Don't show component if there are no batch items
    if props.batch_items.is_empty() {
        return html! {};
    }

    let button_text = if *is_processing {
        "⏳ Processing..."
    } else {
        "📦 Download All as ZIP"
    };

    let button_disabled = *is_processing;

    html! {
        <div style={format!(
            "background: {}; padding: 15px; border-radius: 4px; margin-top: 20px; border: 1px solid {}; color: {};",
            colors.background, colors.border, colors.text
        )}>
            <h3>{"📦 Batch Clean & Download"}</h3>
            <p style={format!("margin-bottom: 15px; color: {};", colors.text)}>
                {format!("Download all {} images with metadata removed as a single ZIP file:", props.batch_items.len())}
            </p>

            <div style={format!(
                "margin-bottom: 15px; padding: 10px; background: {}; border-radius: 4px;",
                colors.info_bg
            )}>
                <div style={format!("font-size: 14px; color: {}; margin-bottom: 8px;", colors.info_text)}>
                    {"All images will be cleaned using binary metadata removal"}
                </div>
                <div style={format!("font-size: 12px; color: {};", colors.info_text)}>
                    {"Original file formats and quality preserved"}
                </div>
            </div>

            <button
                onclick={download_batch_cleaned_cb}
                disabled={button_disabled}
                style={format!(
                    "background: {}; color: white; border: none; padding: 10px 20px; border-radius: 4px; cursor: {}; font-weight: bold; font-size: 14px; opacity: {};",
                    colors.button_bg,
                    if button_disabled { "not-allowed" } else { "pointer" },
                    if button_disabled { "0.6" } else { "1" }
                )}
            >
                {button_text}
            </button>
        </div>
    }
}
