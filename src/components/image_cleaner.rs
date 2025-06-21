//! Download a copy of the file with all metadata stripped.

use crate::binary_cleaner::BinaryCleaner;
use crate::types::ImageData;
use crate::utils::download_binary_file;
use base64::{Engine as _, engine::general_purpose};
use yew::prelude::*;

/// Properties for [`ImageCleaner`].
#[derive(Properties, PartialEq)]
pub struct ImageCleanerProps {
    pub image_data: ImageData,
}

/// Button that performs binary metadata removal and triggers a download.
#[function_component(ImageCleaner)]
pub fn image_cleaner(props: &ImageCleanerProps) -> Html {
    let download_cleaned_image_cb = {
        let data = props.image_data.clone();

        Callback::from(move |_| {
            let data_url = data.data_url.clone();
            let filename = data.name.clone();

            wasm_bindgen_futures::spawn_local(async move {
                if let Some(file_extension) = filename.split('.').next_back() {
                    // Convert data URL to bytes
                    if let Some(base64_data) = data_url.strip_prefix("data:image/") {
                        if let Some(comma_pos) = base64_data.find(',') {
                            let base64_content = &base64_data[comma_pos + 1..];
                            if let Ok(file_bytes) = general_purpose::STANDARD.decode(base64_content)
                            {
                                match BinaryCleaner::clean_metadata(&file_bytes, file_extension) {
                                    Ok(cleaned_bytes) => {
                                        // Create cleaned filename
                                        let cleaned_filename = filename
                                            .strip_suffix(&format!(".{}", file_extension))
                                            .unwrap_or(&filename)
                                            .to_string()
                                            + "_cleaned."
                                            + file_extension;

                                        // Download cleaned file
                                        let mime_type = format!("image/{}", file_extension);
                                        download_binary_file(
                                            &cleaned_bytes,
                                            &cleaned_filename,
                                            &mime_type,
                                        );
                                        return;
                                    }
                                    Err(e) => {
                                        web_sys::console::log_1(
                                            &format!("Binary cleaning failed: {}", e).into(),
                                        );
                                        return;
                                    }
                                }
                            }
                        }
                    }
                }

                web_sys::console::log_1(&"Failed to process file for binary cleaning".into());
            });
        })
    };

    html! {
        <div style="background: #d1ecf1; padding: 15px; border-radius: 4px; margin-top: 20px; border: 1px solid #bee5eb;">
            <h3>{"🧹 Download Cleaned File"}</h3>
            <p style="margin-bottom: 15px; color: #0c5460;">
                {"Download your file with all metadata removed for privacy:"}
            </p>

            <div style="margin-bottom: 15px; padding: 10px; background: rgba(255,255,255,0.7); border-radius: 4px;">
                <div style="font-size: 14px; color: #666; margin-bottom: 8px;">
                    {"High-performance binary metadata removal preserves original file quality"}
                </div>
                <div style="font-size: 12px; color: #666;">
                    {"Supports JPEG, PNG, WebP, GIF, TIFF, HEIF, PDF, SVG and more"}
                </div>
            </div>

            <button
                onclick={download_cleaned_image_cb}
                style="background: #17a2b8; color: white; border: none; padding: 10px 20px; border-radius: 4px; cursor: pointer; font-weight: bold; font-size: 14px;"
            >
                {"🧹 Download Privacy-Safe File"}
            </button>
        </div>
    }
}
