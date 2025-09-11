//! Root Yew component orchestrating file upload, display, cleaning and export.
//!
//! This module wires together the high level components to form the single page
//! application.

use crate::components::{
    batch_manager::BatchManager, file_upload::FileUpload, image_cleaner::ImageCleaner,
    image_display::ImageDisplay, metadata_display::MetadataDisplay,
    metadata_export::MetadataExport,
};
use crate::types::ImageData;
use std::collections::HashSet;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let image_data = use_state(|| None::<ImageData>);
    let is_expanded = use_state(|| false);
    let selected_metadata = use_state(HashSet::<String>::new);
    let show_explanations = use_state(|| false);
    let file_input_trigger = use_state(|| None::<Callback<()>>);
    let error_message = use_state(|| None::<String>);

    let on_file_loaded = {
        let image_data = image_data.clone();
        let is_expanded = is_expanded.clone();
        let selected_metadata = selected_metadata.clone();
        let error_message = error_message.clone();

        Callback::from(move |data: ImageData| {
            // Auto-select all metadata by default
            let all_keys: HashSet<String> = data.exif_data.keys().cloned().collect();
            selected_metadata.set(all_keys);
            image_data.set(Some(data));
            is_expanded.set(false); // Reset to thumbnail view
            error_message.set(None);
        })
    };

    // Batch state for progress visualization
    let batch_in_progress = use_state(|| false);
    let batch_processed = use_state(|| 0usize);
    let batch_total = use_state(|| 0usize);

    let on_file_error = {
        let error_message = error_message.clone();
        Callback::from(move |msg: String| {
            error_message.set(Some(msg));
        })
    };

    let on_image_click = {
        let is_expanded = is_expanded.clone();
        Callback::from(move |_: web_sys::MouseEvent| {
            is_expanded.set(!*is_expanded);
        })
    };

    let on_metadata_selection_change = {
        let selected_metadata = selected_metadata.clone();
        Callback::from(move |new_selection: HashSet<String>| {
            selected_metadata.set(new_selection);
        })
    };

    let on_toggle_explanations = {
        let show_explanations = show_explanations.clone();
        Callback::from(move |_: web_sys::MouseEvent| {
            show_explanations.set(!*show_explanations);
        })
    };

    let on_trigger_file_input = {
        let file_input_trigger = file_input_trigger.clone();
        Callback::from(move |trigger: Callback<()>| {
            file_input_trigger.set(Some(trigger));
        })
    };

    let on_placeholder_click = {
        let file_input_trigger = file_input_trigger.clone();
        Callback::from(move |_: web_sys::MouseEvent| {
            if let Some(ref trigger) = *file_input_trigger {
                trigger.emit(());
            }
        })
    };

    let on_upload_new = {
        let file_input_trigger = file_input_trigger.clone();
        Callback::from(move |_: web_sys::MouseEvent| {
            if let Some(ref trigger) = *file_input_trigger {
                trigger.emit(());
            }
        })
    };

    html! {
        <div style="min-height: 100vh; display: flex; flex-direction: column;">
            <div style="max-width: 800px; margin: 0 auto; padding: 16px; flex: 1;">
                <h1>{"File Metadata Extractor"}</h1>
                {
                    if let Some(msg) = &*error_message {
                        html! { <p style="color: red;">{msg}</p> }
                    } else { html!{} }
                }

                <FileUpload
                    on_file_loaded={on_file_loaded}
                    trigger_file_input={on_trigger_file_input}
                    on_error={on_file_error}
                />

                <BatchManager
                    in_progress={*batch_in_progress}
                    processed={*batch_processed}
                    total={*batch_total}
                />

                // Main content area with consistent layout
                <div style="margin-top: 20px;">
                    {
                        if let Some(ref data) = *image_data {
                            html! {
                                <div style="opacity: 1; transition: opacity 0.3s ease-in-out;">
                                    <ImageDisplay
                                        image_data={data.clone()}
                                        is_expanded={*is_expanded}
                                        on_image_click={on_image_click}
                                        on_upload_new={Some(on_upload_new)}
                                    />

                                    <MetadataDisplay
                                        image_data={data.clone()}
                                        selected_metadata={(*selected_metadata).clone()}
                                        show_explanations={*show_explanations}
                                        on_metadata_selection_change={on_metadata_selection_change}
                                        on_toggle_explanations={on_toggle_explanations}
                                    />

                                    <ImageCleaner image_data={data.clone()} />

                                    <MetadataExport
                                        image_data={data.clone()}
                                        selected_metadata={(*selected_metadata).clone()}
                                    />
                                </div>
                            }
                        } else {
                            html! {
                                <div
                                    onclick={on_placeholder_click}
                                    style="text-align: center; padding: 40px 20px; color: #666; background: #f8f9fa; border-radius: 8px; border: 2px dashed #dee2e6; cursor: pointer; transition: all 0.2s ease; hover:background-color: #e9ecef; hover:border-color: #007bff;"
                                >
                                    <div style="font-size: 48px; margin-bottom: 16px;">{"📁"}</div>
                                    <p style="font-size: 18px; margin-bottom: 8px; font-weight: 500;">{"Click here to select a file"}</p>
                                    <p style="font-size: 14px; margin: 0;">{"Upload images, PDFs, SVGs, and more to extract metadata"}</p>
                                </div>
                            }
                        }
                    }
                </div>
            </div>

            <footer style="margin-top: auto; padding: 20px 0; border-top: 1px solid #ddd; text-align: center; color: #666; font-size: 14px; background-color: #f8f9fa;">
                <p>
                    {"Built with ❤️ using Rust + WebAssembly • "}
                    <a href="https://github.com/akofink/image-metadata-extractor" target="_blank" style="color: #007bff; text-decoration: none;">
                        {"Open Source"}
                    </a>
                    {" • Privacy-First (No Server Uploads)"}
                </p>
                <p style="margin-top: 8px; font-size: 12px;">
                    {"© 2024 File Metadata Extractor • "}
                    <a href="mailto:contact@image-metadata-extractor.com" style="color: #007bff; text-decoration: none;">
                        {"Contact"}
                    </a>
                </p>
            </footer>
        </div>
    }
}
