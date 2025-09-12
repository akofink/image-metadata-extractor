//! Root Yew component orchestrating file upload, display, cleaning and export.
//!
//! This module wires together the high level components to form the single page
//! application.

use crate::components::{
    batch_manager::BatchManager, file_upload::FileUpload, image_cleaner::ImageCleaner,
    image_display::ImageDisplay, metadata_display::MetadataDisplay,
    metadata_export::MetadataExport,
};
use crate::types::{ImageData, Theme};
use std::collections::HashSet;
use yew::prelude::*;

struct AppColors {
    background: &'static str,
    text: &'static str,
    primary: &'static str,
    secondary: &'static str,
    border: &'static str,
    placeholder_bg: &'static str,
    placeholder_border: &'static str,
}

const LIGHT_THEME: AppColors = AppColors {
    background: "#ffffff",
    text: "#333333",
    primary: "#007bff",
    secondary: "#6c757d",
    border: "#ddd",
    placeholder_bg: "#f8f9fa",
    placeholder_border: "#dee2e6",
};

const DARK_THEME: AppColors = AppColors {
    background: "#121212",
    text: "#e0e0e0",
    primary: "#bb86fc",
    secondary: "#03dac6",
    border: "#333",
    placeholder_bg: "#1e1e1e",
    placeholder_border: "#444",
};

#[function_component(App)]
pub fn app() -> Html {
    let image_data = use_state(|| None::<ImageData>);
    let is_expanded = use_state(|| false);
    let selected_metadata = use_state(HashSet::<String>::new);
    let show_explanations = use_state(|| false);
    let file_input_trigger = use_state(|| None::<Callback<()>>);
    let error_message = use_state(|| None::<String>);
    let theme = use_state(|| Theme::Light);
    // Batch browsing state
    let batch_items = use_state(Vec::<ImageData>::new);
    let batch_index = use_state(|| 0usize);

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

    // Batch progress handlers
    let on_batch_progress = {
        let batch_in_progress = batch_in_progress.clone();
        let batch_processed = batch_processed.clone();
        let batch_total = batch_total.clone();
        Callback::from(move |(processed, total): (usize, usize)| {
            batch_in_progress.set(total > 0 && processed < total);
            batch_processed.set(processed);
            batch_total.set(total);
        })
    };

    let on_files_loaded = {
        let batch_in_progress = batch_in_progress.clone();
        let batch_processed = batch_processed.clone();
        let batch_total = batch_total.clone();
        let batch_items = batch_items.clone();
        let batch_index = batch_index.clone();
        let image_data = image_data.clone();
        let selected_metadata = selected_metadata.clone();
        let is_expanded = is_expanded.clone();
        Callback::from(move |datas: Vec<ImageData>| {
            batch_in_progress.set(false);
            // Update progress to complete
            if *batch_total > 0 {
                batch_processed.set(*batch_total);
            }
            // Save items and reset index
            batch_items.set(datas.clone());
            batch_index.set(0);
            // Ensure first item is visible (already emitted earlier, but keep consistent)
            if let Some(first) = datas.first() {
                let all_keys: HashSet<String> = first.exif_data.keys().cloned().collect();
                selected_metadata.set(all_keys);
                image_data.set(Some(first.clone()));
                is_expanded.set(false);
            }
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

    let on_theme_toggle = {
        let theme = theme.clone();
        Callback::from(move |_: MouseEvent| {
            let current_theme = *theme;
            theme.set(match current_theme {
                Theme::Light => Theme::Dark,
                Theme::Dark => Theme::Light,
            });
        })
    };

    let colors = match *theme {
        Theme::Light => LIGHT_THEME,
        Theme::Dark => DARK_THEME,
    };

    let main_div_style = format!(
        "min-height: 100vh; display: flex; flex-direction: column; background-color: {}; color: {};",
        colors.background, colors.text
    );

    let footer_style = format!(
        "margin-top: auto; padding: 20px 0; border-top: 1px solid {}; text-align: center; color: {}; font-size: 14px; background-color: {};",
        colors.border, colors.secondary, colors.placeholder_bg
    );

    let link_style = format!("color: {}; text-decoration: none;", colors.primary);

    html! {
        <div style={main_div_style}>
            <div style="max-width: 800px; margin: 0 auto; padding: 16px; flex: 1;">
                <div style="display: flex; justify-content: space-between; align-items: center;">
                    <h1>{"File Metadata Extractor"}</h1>
                    <button onclick={on_theme_toggle} style="padding: 8px 12px; border-radius: 4px; cursor: pointer;">
                        { match *theme { Theme::Light => "Switch to Dark Mode", Theme::Dark => "Switch to Light Mode" } }
                    </button>
                </div>

                {
                    if let Some(msg) = &*error_message {
                        html! { <p style="color: red;">{msg}</p> }
                    } else { html!{} }
                }

                <FileUpload
                    on_file_loaded={on_file_loaded}
                    trigger_file_input={on_trigger_file_input}
                    on_error={on_file_error}
                    on_files_loaded={Some(on_files_loaded)}
                    on_progress={Some(on_batch_progress)}
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
                                   {
                                       if !batch_items.is_empty() && batch_items.len() > 1 {
                                           let has_prev = *batch_index > 0;
                                           let has_next = *batch_index + 1 < batch_items.len();

                                           let on_prev = {
                                               let batch_index = batch_index.clone();
                                               let batch_items = batch_items.clone();
                                               let selected_metadata = selected_metadata.clone();
                                               let image_state = image_data.clone();
                                               Callback::from(move |_| {
                                                   if *batch_index > 0 {
                                                       let new_idx = *batch_index - 1;
                                                       batch_index.set(new_idx);
                                                       if let Some(item) = batch_items.get(new_idx) {
                                                           let keys: HashSet<String> = item.exif_data.keys().cloned().collect();
                                                           selected_metadata.set(keys);
                                                           image_state.set(Some(item.clone()));
                                                       }
                                                   }
                                               })
                                           };

                                           let on_next = {
                                               let batch_index = batch_index.clone();
                                               let batch_items = batch_items.clone();
                                               let selected_metadata = selected_metadata.clone();
                                               let image_state = image_data.clone();
                                               Callback::from(move |_| {
                                                   if *batch_index + 1 < batch_items.len() {
                                                       let new_idx = *batch_index + 1;
                                                       batch_index.set(new_idx);
                                                       if let Some(item) = batch_items.get(new_idx) {
                                                           let keys: HashSet<String> = item.exif_data.keys().cloned().collect();
                                                           selected_metadata.set(keys);
                                                           image_state.set(Some(item.clone()));
                                                       }
                                                   }
                                               })
                                           };

                                           let prev_style = if has_prev {
                                               "border: none; padding: 6px 12px; border-radius: 4px; font-weight: bold; background: #007bff; color: white; cursor: pointer;"
                                           } else {
                                               "border: none; padding: 6px 12px; border-radius: 4px; font-weight: bold; background: #6c757d; color: #aaa; cursor: not-allowed;"
                                           };
                                           let next_style = if has_next {
                                               "border: none; padding: 6px 12px; border-radius: 4px; font-weight: bold; background: #007bff; color: white; cursor: pointer;"
                                           } else {
                                               "border: none; padding: 6px 12px; border-radius: 4px; font-weight: bold; background: #6c757d; color: #aaa; cursor: not-allowed;"
                                           };

                                           html! {
                                               <div style="display:flex;gap:12px;align-items:center;justify-content:flex-end;margin:8px 0 12px 0;">
                                                   <div style="font-size:12px;color:#666;">{ format!("Image {} of {}", *batch_index + 1, batch_items.len()) }</div>
                                                   <button onclick={on_prev} disabled={!has_prev} style={prev_style}>{"⬅ Previous"}</button>
                                                   <button onclick={on_next} disabled={!has_next} style={next_style}>{"Next ➡"}</button>
                                               </div>
                                           }
                                       } else { html!{} }
                                   }

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

                                   {
                                       if !batch_items.is_empty() && batch_items.len() > 1 {
                                           let has_prev = *batch_index > 0;
                                           let has_next = *batch_index + 1 < batch_items.len();

                                           let on_prev = {
                                               let batch_index = batch_index.clone();
                                               let batch_items = batch_items.clone();
                                               let selected_metadata = selected_metadata.clone();
                                               let image_state = image_data.clone();
                                               Callback::from(move |_| {
                                                   if *batch_index > 0 {
                                                       let new_idx = *batch_index - 1;
                                                       batch_index.set(new_idx);
                                                       if let Some(item) = batch_items.get(new_idx) {
                                                           let keys: HashSet<String> = item.exif_data.keys().cloned().collect();
                                                           selected_metadata.set(keys);
                                                           image_state.set(Some(item.clone()));
                                                       }
                                                   }
                                               })
                                           };

                                           let on_next = {
                                               let batch_index = batch_index.clone();
                                               let batch_items = batch_items.clone();
                                               let selected_metadata = selected_metadata.clone();
                                               let image_state = image_data.clone();
                                               Callback::from(move |_| {
                                                   if *batch_index + 1 < batch_items.len() {
                                                       let new_idx = *batch_index + 1;
                                                       batch_index.set(new_idx);
                                                       if let Some(item) = batch_items.get(new_idx) {
                                                           let keys: HashSet<String> = item.exif_data.keys().cloned().collect();
                                                           selected_metadata.set(keys);
                                                           image_state.set(Some(item.clone()));
                                                       }
                                                   }
                                               })
                                           };

                                           html! {
                                               <div style="display:flex;gap:8px;align-items:center;margin:12px 0;">
                                                   <button onclick={on_prev} disabled={!has_prev} style="padding:6px 10px;border-radius:4px;">{"⬅ Previous"}</button>
                                                   <div style="font-size:12px;color:#666;">{ format!("Image {} of {}", *batch_index + 1, batch_items.len()) }</div>
                                                   <button onclick={on_next} disabled={!has_next} style="padding:6px 10px;border-radius:4px;">{"Next ➡"}</button>
                                               </div>
                                           }
                                       } else { html!{} }
                                   }

                                   <MetadataExport
                                       image_data={data.clone()}
                                       selected_metadata={(*selected_metadata).clone()}
                                   />
                                </div>
                            }
                        } else {
                            let placeholder_style = format!(
                                "text-align: center; padding: 40px 20px; color: {}; background: {}; border-radius: 8px; border: 2px dashed {}; cursor: pointer; transition: all 0.2s ease;",
                                colors.text,
                                colors.placeholder_bg,
                                colors.placeholder_border
                            );
                            html! {
                                <div
                                    onclick={on_placeholder_click}
                                    style={placeholder_style}
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

            <footer style={footer_style}>
                <p>
                    {"Built with ❤️ using Rust + WebAssembly • "}
                    <a href="https://github.com/akofink/image-metadata-extractor" target="_blank" style={link_style.clone()}>
                        {"Open Source"}
                    </a>
                    {" • Privacy-First (No Server Uploads)"}
                </p>
                <p style="margin-top: 8px; font-size: 12px;">
                    {"© 2024 File Metadata Extractor • "}
                    <a href="mailto:contact@image-metadata-extractor.com" style={link_style.clone()}>
                        {"Contact"}
                    </a>
                </p>
            </footer>
        </div>
    }
}
