//! Root Yew component orchestrating file upload, display, cleaning and export.
//!
//! This module wires together the high level components to form the single page
//! application.

use crate::components::{
    batch_manager::BatchManager,
    command_palette::{CommandAction, CommandPalette},
    duplicate_detector::DuplicateDetector,
    file_upload::FileUpload,
    image_cleaner::ImageCleaner,
    image_display::ImageDisplay,
    metadata_display::MetadataDisplay,
    metadata_export::MetadataExport,
};
use crate::preferences::UserPreferences;
use crate::types::{ImageData, Theme};
use std::collections::HashSet;
use wasm_bindgen::JsCast;
use web_sys::window;
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
    let preferences = use_state(UserPreferences::load);
    let file_input_trigger = use_state(|| None::<Callback<()>>);
    let error_message = use_state(|| None::<String>);
    let theme = use_state(|| Theme::Light);
    let command_palette_visible = use_state(|| false);

    // Effect to save theme to localStorage
    {
        let theme = theme.clone();
        use_effect_with(theme.clone(), move |theme| {
            if let Some(storage) = window().and_then(|w| w.local_storage().ok().flatten()) {
                let theme_str = match **theme {
                    Theme::Light => "light",
                    Theme::Dark => "dark",
                };
                storage.set_item("theme", theme_str).ok();
            }
            || ()
        });
    }

    // Global keyboard shortcuts
    {
        let command_palette_visible = command_palette_visible.clone();
        let is_expanded = is_expanded.clone();
        let file_input_trigger = file_input_trigger.clone();
        let selected_metadata = selected_metadata.clone();
        let preferences = preferences.clone();
        let image_data = image_data.clone();

        use_effect_with((), move |_| {
            let command_palette_visible = command_palette_visible.clone();
            let is_expanded = is_expanded.clone();
            let file_input_trigger = file_input_trigger.clone();
            let selected_metadata = selected_metadata.clone();
            let preferences = preferences.clone();
            let image_data = image_data.clone();

            let keydown_handler = {
                let command_palette_visible = command_palette_visible.clone();
                let is_expanded = is_expanded.clone();
                let file_input_trigger = file_input_trigger.clone();
                let selected_metadata = selected_metadata.clone();
                let preferences = preferences.clone();
                let image_data = image_data.clone();

                gloo::events::EventListener::new(
                    &web_sys::window().unwrap(),
                    "keydown",
                    move |event| {
                        let event = event.dyn_ref::<web_sys::KeyboardEvent>().unwrap();
                        let ctrl_or_cmd = event.ctrl_key() || event.meta_key();

                        match event.key().as_str() {
                            "k" | "K" if ctrl_or_cmd => {
                                event.prevent_default();
                                command_palette_visible.set(true);
                            }
                            " " if !*command_palette_visible && image_data.is_some() => {
                                // Only handle space if not typing in an input and we have an image
                                if let Some(active_element) = web_sys::window()
                                    .and_then(|w| w.document())
                                    .and_then(|d| d.active_element())
                                {
                                    let tag_name = active_element.tag_name().to_lowercase();
                                    if tag_name != "input" && tag_name != "textarea" {
                                        event.prevent_default();
                                        is_expanded.set(!*is_expanded);
                                    }
                                }
                            }
                            "o" if ctrl_or_cmd => {
                                event.prevent_default();
                                if let Some(ref trigger) = *file_input_trigger {
                                    trigger.emit(());
                                }
                            }
                            "a" if ctrl_or_cmd && image_data.is_some() => {
                                event.prevent_default();
                                if let Some(data) = &*image_data {
                                    let all_keys: HashSet<String> =
                                        data.exif_data.keys().cloned().collect();
                                    selected_metadata.set(all_keys);
                                }
                            }
                            "d" if ctrl_or_cmd && image_data.is_some() => {
                                event.prevent_default();
                                selected_metadata.set(HashSet::new());
                            }
                            "?" if image_data.is_some() => {
                                event.prevent_default();
                                let mut new_prefs = (*preferences).clone();
                                new_prefs.update_and_save(|prefs| {
                                    prefs.show_explanations = !prefs.show_explanations;
                                });
                                preferences.set(new_prefs);
                            }
                            _ => {}
                        }
                    },
                )
            };

            move || drop(keydown_handler)
        });
    }

    // Effect for initial theme detection
    {
        let theme = theme.clone();
        use_effect_with((), move |_| {
            let storage = window().and_then(|w| w.local_storage().ok().flatten());
            let initial_theme = if let Some(storage) = storage {
                storage
                    .get_item("theme")
                    .ok()
                    .flatten()
                    .and_then(|t| match t.as_str() {
                        "light" => Some(Theme::Light),
                        "dark" => Some(Theme::Dark),
                        _ => None,
                    })
            } else {
                None
            };

            if let Some(initial_theme) = initial_theme {
                theme.set(initial_theme);
            } else if let Some(window) = window()
                && let Ok(Some(media_query_list)) =
                    window.match_media("(prefers-color-scheme: dark)")
                && media_query_list.matches()
            {
                theme.set(Theme::Dark);
            }

            || ()
        });
    }

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
        let preferences = preferences.clone();
        Callback::from(move |_: web_sys::MouseEvent| {
            let mut new_prefs = (*preferences).clone();
            new_prefs.update_and_save(|prefs| {
                prefs.show_explanations = !prefs.show_explanations;
            });
            preferences.set(new_prefs);
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

    let on_preferences_change = {
        let preferences = preferences.clone();
        Callback::from(move |new_prefs: UserPreferences| {
            preferences.set(new_prefs);
        })
    };

    let on_command_palette_close = {
        let command_palette_visible = command_palette_visible.clone();
        Callback::from(move |_| {
            command_palette_visible.set(false);
        })
    };

    let on_command = {
        let theme = theme.clone();
        let preferences = preferences.clone();
        let is_expanded = is_expanded.clone();
        let file_input_trigger = file_input_trigger.clone();
        let selected_metadata = selected_metadata.clone();
        let image_data = image_data.clone();
        let on_preferences_change = on_preferences_change.clone();

        Callback::from(move |action: CommandAction| match action {
            CommandAction::ToggleTheme => {
                let current_theme = *theme;
                theme.set(match current_theme {
                    Theme::Light => Theme::Dark,
                    Theme::Dark => Theme::Light,
                });
            }
            CommandAction::ToggleExplanations => {
                let mut new_prefs = (*preferences).clone();
                new_prefs.update_and_save(|prefs| {
                    prefs.show_explanations = !prefs.show_explanations;
                });
                on_preferences_change.emit(new_prefs);
            }
            CommandAction::ToggleFileInfo => {
                let mut new_prefs = (*preferences).clone();
                new_prefs.update_and_save(|prefs| {
                    prefs.include_basic_info = !prefs.include_basic_info;
                });
                on_preferences_change.emit(new_prefs);
            }
            CommandAction::ToggleGps => {
                let mut new_prefs = (*preferences).clone();
                new_prefs.update_and_save(|prefs| {
                    prefs.include_gps = !prefs.include_gps;
                });
                on_preferences_change.emit(new_prefs);
            }
            CommandAction::ExpandImage => {
                is_expanded.set(!*is_expanded);
            }
            CommandAction::UploadNew => {
                if let Some(ref trigger) = *file_input_trigger {
                    trigger.emit(());
                }
            }
            CommandAction::SelectAllMetadata => {
                if let Some(data) = &*image_data {
                    let all_keys: HashSet<String> = data.exif_data.keys().cloned().collect();
                    selected_metadata.set(all_keys);
                }
            }
            CommandAction::DeselectAllMetadata => {
                selected_metadata.set(HashSet::new());
            }
            CommandAction::ExportJson => {
                if let Some(data) = &*image_data {
                    let filtered_data = data.filter_metadata(
                        &selected_metadata,
                        preferences.include_basic_info,
                        preferences.include_gps,
                    );
                    if let Ok(json) = serde_json::to_string_pretty(&filtered_data) {
                        crate::utils::download_file(
                            &json,
                            &format!("{}_filtered_metadata.json", data.name),
                            "application/json",
                        );
                    }
                }
            }
            CommandAction::ExportCsv => {
                if let Some(data) = &*image_data {
                    let filtered_data = data.filter_metadata(
                        &selected_metadata,
                        preferences.include_basic_info,
                        preferences.include_gps,
                    );
                    let csv = crate::export::generate_csv(&filtered_data);
                    crate::utils::download_file(
                        &csv,
                        &format!("{}_filtered_metadata.csv", data.name),
                        "text/csv",
                    );
                }
            }
            CommandAction::ExportTxt => {
                if let Some(data) = &*image_data {
                    let filtered_data = data.filter_metadata(
                        &selected_metadata,
                        preferences.include_basic_info,
                        preferences.include_gps,
                    );
                    let txt = crate::export::generate_txt(&filtered_data);
                    crate::utils::download_file(
                        &txt,
                        &format!("{}_filtered_metadata.txt", data.name),
                        "text/plain",
                    );
                }
            }
            CommandAction::CopyJson => {
                if let Some(data) = &*image_data {
                    let filtered_data = data.filter_metadata(
                        &selected_metadata,
                        preferences.include_basic_info,
                        preferences.include_gps,
                    );
                    if let Ok(json) = serde_json::to_string_pretty(&filtered_data) {
                        crate::utils::copy_to_clipboard(&json);
                    }
                }
            }
            CommandAction::CopyCsv => {
                if let Some(data) = &*image_data {
                    let filtered_data = data.filter_metadata(
                        &selected_metadata,
                        preferences.include_basic_info,
                        preferences.include_gps,
                    );
                    let csv = crate::export::generate_csv(&filtered_data);
                    crate::utils::copy_to_clipboard(&csv);
                }
            }
            CommandAction::CopyTxt => {
                if let Some(data) = &*image_data {
                    let filtered_data = data.filter_metadata(
                        &selected_metadata,
                        preferences.include_basic_info,
                        preferences.include_gps,
                    );
                    let txt = crate::export::generate_txt(&filtered_data);
                    crate::utils::copy_to_clipboard(&txt);
                }
            }
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

    let theme_button_style = format!(
        "padding: 8px; border-radius: 4px; cursor: pointer; border: 1px solid {}; background-color: {}; color: {}; font-size: 16px; width: 36px; height: 36px; display: flex; align-items: center; justify-content: center;",
        colors.border, colors.background, colors.text
    );

    html! {
        <div style={main_div_style}>
            <div style="max-width: 800px; margin: 0 auto; padding: 16px; flex: 1;">
                <div style="position: relative; margin-bottom: 20px;">
                    <h1 style="text-align: center; margin: 0; padding-right: 50px;">{"File Metadata Extractor"}</h1>
                    <button
                        onclick={on_theme_toggle}
                        style={format!("{} position: absolute; top: 50%; right: 0; transform: translateY(-50%);", theme_button_style)}
                        title={if *theme == Theme::Light { "Switch to dark mode" } else { "Switch to light mode" }}
                    >
                        { if *theme == Theme::Light { "🌙" } else { "☀️" } }
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

                                           let button_text_color = if *theme == Theme::Light { "white" } else { "black" };

                                           let prev_style = format!(
                                                "border: none; padding: 6px 12px; border-radius: 4px; font-weight: bold; background: {}; color: {}; cursor: {};",
                                                if has_prev { colors.primary } else { colors.secondary },
                                                button_text_color,
                                                if has_prev { "pointer" } else { "not-allowed" }
                                           );
                                           let next_style = format!(
                                                "border: none; padding: 6px 12px; border-radius: 4px; font-weight: bold; background: {}; color: {}; cursor: {};",
                                                if has_next { colors.primary } else { colors.secondary },
                                                button_text_color,
                                                if has_next { "pointer" } else { "not-allowed" }
                                           );

                                           html! {
                                               <div style="display:flex;gap:12px;align-items:center;justify-content:flex-end;margin:8px 0 12px 0;">
                                                   <div style={format!("font-size:12px;color:{};", colors.text)}>{ format!("Image {} of {}", *batch_index + 1, batch_items.len()) }</div>
                                                   <button onclick={on_prev} disabled={!has_prev} style={prev_style}>{"⬅ Previous"}</button>
                                                   <button onclick={on_next} disabled={!has_next} style={next_style}>{"Next ➡"}</button>
                                               </div>
                                           }
                                       } else { html!{} }
                                   }

                                   {
                                       // Show duplicate detector if we have multiple batch items
                                       if batch_items.len() > 1 {
                                           html! {
                                               <DuplicateDetector
                                                   batch_items={(*batch_items).clone()}
                                                   theme={*theme}
                                               />
                                           }
                                       } else {
                                           html! {}
                                       }
                                   }

                                    <ImageDisplay
                                        image_data={data.clone()}
                                        is_expanded={*is_expanded}
                                        on_image_click={on_image_click}
                                        on_upload_new={Some(on_upload_new)}
                                        theme={*theme}
                                    />

                                    <MetadataDisplay
                                        image_data={data.clone()}
                                        selected_metadata={(*selected_metadata).clone()}
                                        show_explanations={preferences.show_explanations}
                                        on_metadata_selection_change={on_metadata_selection_change.clone()}
                                        on_toggle_explanations={on_toggle_explanations}
                                        theme={*theme}
                                    />

                                    <ImageCleaner image_data={data.clone()} theme={*theme} />

                                   <MetadataExport
                                       image_data={data.clone()}
                                       selected_metadata={(*selected_metadata).clone()}
                                       theme={*theme}
                                       preferences={(*preferences).clone()}
                                       on_preferences_change={on_preferences_change}
                                       on_metadata_selection_change={on_metadata_selection_change.clone()}
                                       batch_items={if batch_items.is_empty() { None } else { Some((*batch_items).clone()) }}
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

            <CommandPalette
                visible={*command_palette_visible}
                theme={*theme}
                on_close={on_command_palette_close}
                on_command={on_command}
                has_image={image_data.is_some()}
                is_expanded={*is_expanded}
                show_explanations={preferences.show_explanations}
                include_file_info={preferences.include_basic_info}
                include_gps={preferences.include_gps}
            />
        </div>
    }
}
