//! Download a copy of the file with all metadata stripped.

use crate::binary_cleaner::BinaryCleaner;
use crate::types::{ImageData, Theme};
use crate::utils::download_binary_file;
use base64::Engine as _;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use yew::prelude::*;

/// Fetch bytes from a blob URL
async fn fetch_blob_bytes(blob_url: &str) -> Result<Vec<u8>, JsValue> {
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("No window"))?;
    let response = JsFuture::from(window.fetch_with_str(blob_url)).await?;
    let response: web_sys::Response = response.dyn_into()?;
    let array_buffer = JsFuture::from(response.array_buffer()?).await?;
    let uint8_array = js_sys::Uint8Array::new(&array_buffer);
    Ok(uint8_array.to_vec())
}

struct CleanerColors {
    background: &'static str,
    text: &'static str,
    border: &'static str,
    info_bg: &'static str,
    info_text: &'static str,
    button_bg: &'static str,
}

const LIGHT_CLEANER_COLORS: CleanerColors = CleanerColors {
    background: "#d1ecf1",
    text: "#0c5460",
    border: "#bee5eb",
    info_bg: "rgba(255,255,255,0.7)",
    info_text: "#666",
    button_bg: "#17a2b8",
};

const DARK_CLEANER_COLORS: CleanerColors = CleanerColors {
    background: "#1a4548",
    text: "#b8dce1",
    border: "#2d5a5f",
    info_bg: "rgba(255,255,255,0.1)",
    info_text: "#aaa",
    button_bg: "#20c997",
};

/// Properties for [`ImageCleaner`].
#[derive(Properties, PartialEq)]
pub struct ImageCleanerProps {
    pub image_data: ImageData,
    pub theme: Theme,
}

/// Button that performs binary metadata removal and triggers a download.
#[function_component(ImageCleaner)]
pub fn image_cleaner(props: &ImageCleanerProps) -> Html {
    let colors = match props.theme {
        Theme::Light => LIGHT_CLEANER_COLORS,
        Theme::Dark => DARK_CLEANER_COLORS,
    };

    let download_cleaned_image_cb = {
        let data = props.image_data.clone();

        Callback::from(move |_| {
            let data_url = data.data_url.clone();
            let filename = data.name.clone();

            wasm_bindgen_futures::spawn_local(async move {
                if let Some(file_extension) = filename.split('.').next_back() {
                    // Fetch the blob data from the object URL
                    let file_bytes = if data_url.starts_with("blob:") {
                        // Fetch blob content
                        match fetch_blob_bytes(&data_url).await {
                            Ok(bytes) => bytes,
                            Err(e) => {
                                web_sys::console::log_1(
                                    &format!("Failed to fetch blob data: {:?}", e).into(),
                                );
                                return;
                            }
                        }
                    } else if let Some(base64_data) = data_url.strip_prefix("data:image/")
                        && let Some(comma_pos) = base64_data.find(',')
                    {
                        // Legacy base64 data URL support
                        match base64::engine::general_purpose::STANDARD
                            .decode(&base64_data[comma_pos + 1..])
                        {
                            Ok(bytes) => bytes,
                            Err(e) => {
                                web_sys::console::log_1(
                                    &format!("Failed to decode base64 data: {}", e).into(),
                                );
                                return;
                            }
                        }
                    } else {
                        web_sys::console::log_1(&"Unsupported data URL format".into());
                        return;
                    };

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
                            download_binary_file(&cleaned_bytes, &cleaned_filename, &mime_type);
                        }
                        Err(e) => {
                            web_sys::console::log_1(
                                &format!("Binary cleaning failed: {}", e).into(),
                            );
                        }
                    }
                }
            });
        })
    };

    html! {
        <div style={format!("background: {}; padding: 15px; border-radius: 4px; margin-top: 20px; border: 1px solid {}; color: {};", colors.background, colors.border, colors.text)}>
            <h3>{"🧹 Download Cleaned File"}</h3>
            <p style={format!("margin-bottom: 15px; color: {};", colors.text)}>
                {"Download your file with all metadata removed for privacy:"}
            </p>

            <div style={format!("margin-bottom: 15px; padding: 10px; background: {}; border-radius: 4px;", colors.info_bg)}>
                <div style={format!("font-size: 14px; color: {}; margin-bottom: 8px;", colors.info_text)}>
                    {"High-performance binary metadata removal preserves original file quality"}
                </div>
                <div style={format!("font-size: 12px; color: {};", colors.info_text)}>
                    {"Supports JPEG, PNG, WebP, GIF, TIFF, HEIF, PDF, SVG and more"}
                </div>
            </div>

            <button
                onclick={download_cleaned_image_cb}
                style={format!("background: {}; color: white; border: none; padding: 10px 20px; border-radius: 4px; cursor: pointer; font-weight: bold; font-size: 14px;", colors.button_bg)}
                data-testid="clean-button"
            >
                {"🧹 Download Privacy-Safe File"}
            </button>
        </div>
    }
}
