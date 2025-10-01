//! Allows users to download selected metadata in various formats.

use crate::export::{
    generate_csv, generate_csv_batch, generate_json_batch, generate_md, generate_txt,
    generate_txt_batch, generate_xml, generate_yaml,
};
use crate::preferences::{ExportProfile, UserPreferences};
use crate::types::{ImageData, Theme};
use crate::utils::{copy_to_clipboard, download_file};
use std::collections::HashSet;
use yew::prelude::*;

struct ExportColors {
    background: &'static str,
    text: &'static str,
    border: &'static str,
    checkbox_bg: &'static str,
    help_text: &'static str,
}

const LIGHT_EXPORT_COLORS: ExportColors = ExportColors {
    background: "#fff3cd",
    text: "#856404",
    border: "#ffeaa7",
    checkbox_bg: "rgba(255,255,255,0.7)",
    help_text: "#666",
};

const DARK_EXPORT_COLORS: ExportColors = ExportColors {
    background: "#3d3d0a",
    text: "#fff3cd",
    border: "#666600",
    checkbox_bg: "rgba(255,255,255,0.1)",
    help_text: "#aaa",
};

/// Properties for [`MetadataExport`].
#[derive(Properties, PartialEq)]
pub struct MetadataExportProps {
    pub image_data: ImageData,
    pub selected_metadata: HashSet<String>,
    pub theme: Theme,
    pub preferences: UserPreferences,
    pub on_preferences_change: Callback<UserPreferences>,
    pub on_metadata_selection_change: Callback<HashSet<String>>,
    #[prop_or_default]
    pub batch_items: Option<Vec<ImageData>>, // When provided, enable combined batch export
}

/// Controls for exporting chosen metadata fields to JSON, CSV or text.
#[function_component(MetadataExport)]
pub fn metadata_export(props: &MetadataExportProps) -> Html {
    let data = &props.image_data;
    let selected_metadata = &props.selected_metadata;
    let preferences = &props.preferences;

    let colors = match props.theme {
        Theme::Light => LIGHT_EXPORT_COLORS,
        Theme::Dark => DARK_EXPORT_COLORS,
    };

    let export_json = {
        let data = data.clone();
        let selected_metadata = selected_metadata.clone();
        let preferences = preferences.clone();

        Callback::from(move |_| {
            let filtered_data = data.filter_metadata(
                &selected_metadata,
                preferences.include_basic_info,
                preferences.include_gps,
            );
            if let Ok(json) = serde_json::to_string_pretty(&filtered_data) {
                download_file(
                    &json,
                    &format!("{}_filtered_metadata.json", data.name),
                    "application/json",
                );
            }
        })
    };

    let export_csv = {
        let data = data.clone();
        let selected_metadata = selected_metadata.clone();
        let preferences = preferences.clone();

        Callback::from(move |_| {
            let filtered_data = data.filter_metadata(
                &selected_metadata,
                preferences.include_basic_info,
                preferences.include_gps,
            );
            let csv = generate_csv(&filtered_data);
            download_file(
                &csv,
                &format!("{}_filtered_metadata.csv", data.name),
                "text/csv",
            );
        })
    };

    let export_txt = {
        let data = data.clone();
        let selected_metadata = selected_metadata.clone();
        let preferences = preferences.clone();

        Callback::from(move |_| {
            let filtered_data = data.filter_metadata(
                &selected_metadata,
                preferences.include_basic_info,
                preferences.include_gps,
            );
            let txt = generate_txt(&filtered_data);
            download_file(
                &txt,
                &format!("{}_filtered_metadata.txt", data.name),
                "text/plain",
            );
        })
    };

    let export_md = {
        let data = data.clone();
        let selected_metadata = selected_metadata.clone();
        let preferences = preferences.clone();

        Callback::from(move |_| {
            let filtered_data = data.filter_metadata(
                &selected_metadata,
                preferences.include_basic_info,
                preferences.include_gps,
            );
            let md = generate_md(&filtered_data);
            download_file(
                &md,
                &format!("{}_filtered_metadata.md", data.name),
                "text/markdown",
            );
        })
    };

    let export_yaml = {
        let data = data.clone();
        let selected_metadata = selected_metadata.clone();
        let preferences = preferences.clone();

        Callback::from(move |_| {
            let filtered_data = data.filter_metadata(
                &selected_metadata,
                preferences.include_basic_info,
                preferences.include_gps,
            );
            let yaml = generate_yaml(&filtered_data);
            download_file(
                &yaml,
                &format!("{}_filtered_metadata.yaml", data.name),
                "text/yaml",
            );
        })
    };

    let export_xml = {
        let data = data.clone();
        let selected_metadata = selected_metadata.clone();
        let preferences = preferences.clone();

        Callback::from(move |_| {
            let filtered_data = data.filter_metadata(
                &selected_metadata,
                preferences.include_basic_info,
                preferences.include_gps,
            );
            let xml = generate_xml(&filtered_data);
            download_file(
                &xml,
                &format!("{}_filtered_metadata.xml", data.name),
                "application/xml",
            );
        })
    };

    let copy_json = {
        let data = data.clone();
        let selected_metadata = selected_metadata.clone();
        let preferences = preferences.clone();

        Callback::from(move |_| {
            let filtered_data = data.filter_metadata(
                &selected_metadata,
                preferences.include_basic_info,
                preferences.include_gps,
            );
            if let Ok(json) = serde_json::to_string_pretty(&filtered_data) {
                copy_to_clipboard(&json);
            }
        })
    };

    let copy_csv = {
        let data = data.clone();
        let selected_metadata = selected_metadata.clone();
        let preferences = preferences.clone();

        Callback::from(move |_| {
            let filtered_data = data.filter_metadata(
                &selected_metadata,
                preferences.include_basic_info,
                preferences.include_gps,
            );
            let csv = generate_csv(&filtered_data);
            copy_to_clipboard(&csv);
        })
    };

    let copy_txt = {
        let data = data.clone();
        let selected_metadata = selected_metadata.clone();
        let preferences = preferences.clone();

        Callback::from(move |_| {
            let filtered_data = data.filter_metadata(
                &selected_metadata,
                preferences.include_basic_info,
                preferences.include_gps,
            );
            let txt = generate_txt(&filtered_data);
            copy_to_clipboard(&txt);
        })
    };

    let copy_md = {
        let data = data.clone();
        let selected_metadata = selected_metadata.clone();
        let preferences = preferences.clone();

        Callback::from(move |_| {
            let filtered_data = data.filter_metadata(
                &selected_metadata,
                preferences.include_basic_info,
                preferences.include_gps,
            );
            let md = generate_md(&filtered_data);
            copy_to_clipboard(&md);
        })
    };

    let copy_yaml = {
        let data = data.clone();
        let selected_metadata = selected_metadata.clone();
        let preferences = preferences.clone();

        Callback::from(move |_| {
            let filtered_data = data.filter_metadata(
                &selected_metadata,
                preferences.include_basic_info,
                preferences.include_gps,
            );
            let yaml = generate_yaml(&filtered_data);
            copy_to_clipboard(&yaml);
        })
    };

    let copy_xml = {
        let data = data.clone();
        let selected_metadata = selected_metadata.clone();
        let preferences = preferences.clone();

        Callback::from(move |_| {
            let filtered_data = data.filter_metadata(
                &selected_metadata,
                preferences.include_basic_info,
                preferences.include_gps,
            );
            let xml = generate_xml(&filtered_data);
            copy_to_clipboard(&xml);
        })
    };

    // Only show export section if there's metadata to export
    if data.exif_data.is_empty()
        && data.gps_coords.is_none()
        && data.width.is_none()
        && data.height.is_none()
    {
        return html! {};
    }

    // Calculate if there's anything to export
    let has_metadata = !selected_metadata.is_empty();
    let has_file_info = preferences.include_basic_info; // File info always includes at least name/size
    let has_gps = preferences.include_gps && data.gps_coords.is_some();
    let has_anything_to_export = has_metadata || has_file_info || has_gps;

    // Profile management state
    let profiles = use_state(ExportProfile::load_all);
    let show_profile_save = use_state(|| false);
    let profile_name_input = use_state(String::new);
    let profile_desc_input = use_state(String::new);
    let profile_error = use_state(|| None::<String>);

    // Load a profile
    let on_load_profile = {
        let on_metadata_selection_change = props.on_metadata_selection_change.clone();
        let on_preferences_change = props.on_preferences_change.clone();
        let preferences = preferences.clone();
        let data = data.clone();
        Callback::from(move |profile: ExportProfile| {
            // Update metadata selection
            // For "Forensics" profile with empty fields, select all
            let new_selection = if profile.selected_fields.is_empty() {
                data.exif_data.keys().cloned().collect()
            } else {
                // Only select fields that exist in current image
                profile
                    .selected_fields
                    .iter()
                    .filter(|f| data.exif_data.contains_key(*f))
                    .cloned()
                    .collect()
            };
            on_metadata_selection_change.emit(new_selection);

            // Update preferences
            let mut new_prefs = preferences.clone();
            new_prefs.include_basic_info = profile.include_basic_info;
            new_prefs.include_gps = profile.include_gps;
            new_prefs.save();
            on_preferences_change.emit(new_prefs);
        })
    };

    // Save current selection as a new profile
    let on_save_profile_click = {
        let show_profile_save = show_profile_save.clone();
        Callback::from(move |_| {
            show_profile_save.set(!*show_profile_save);
        })
    };

    let on_profile_name_input = {
        let profile_name_input = profile_name_input.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_dyn_into::<web_sys::HtmlInputElement>();
            if let Some(input) = input {
                profile_name_input.set(input.value());
            }
        })
    };

    let on_profile_desc_input = {
        let profile_desc_input = profile_desc_input.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_dyn_into::<web_sys::HtmlInputElement>();
            if let Some(input) = input {
                profile_desc_input.set(input.value());
            }
        })
    };

    let on_save_profile_submit = {
        let profile_name_input = profile_name_input.clone();
        let profile_desc_input = profile_desc_input.clone();
        let selected_metadata = selected_metadata.clone();
        let preferences = preferences.clone();
        let profiles = profiles.clone();
        let show_profile_save = show_profile_save.clone();
        let profile_error = profile_error.clone();
        Callback::from(move |_| {
            let name = (*profile_name_input).trim().to_string();
            if name.is_empty() {
                profile_error.set(Some("Profile name is required".to_string()));
                return;
            }

            let profile = ExportProfile::new(
                name,
                (*profile_desc_input).clone(),
                selected_metadata.clone(),
                preferences.include_basic_info,
                preferences.include_gps,
            );

            match ExportProfile::save(&profile) {
                Ok(()) => {
                    profiles.set(ExportProfile::load_all());
                    profile_name_input.set(String::new());
                    profile_desc_input.set(String::new());
                    profile_error.set(None);
                    show_profile_save.set(false);
                }
                Err(e) => {
                    profile_error.set(Some(e));
                }
            }
        })
    };

    let on_delete_profile = {
        let profiles = profiles.clone();
        Callback::from(move |profile_name: String| {
            let _ = ExportProfile::delete(&profile_name);
            profiles.set(ExportProfile::load_all());
        })
    };

    html! {
        <div style={format!("background: {}; padding: 15px; border-radius: 4px; margin-top: 20px; border: 1px solid {}; color: {};", colors.background, colors.border, colors.text)}>
            <h3>{"📊 Export Metadata"}</h3>
            <p style={format!("margin-bottom: 15px; color: {};", colors.text)}>
                {"Download selected metadata in your preferred format:"}
            </p>

            // Profile management section
            <div style={format!("margin-bottom: 15px; padding: 10px; background: {}; border-radius: 4px;", colors.checkbox_bg)}>
                <h4 style="margin: 0 0 10px 0; font-size: 14px;">{"⚡ Quick Profiles:"}</h4>

                // Preset profiles
                <div style="display: flex; gap: 6px; flex-wrap: wrap; margin-bottom: 10px;">
                    {
                        ExportProfile::get_presets().into_iter().map(|profile| {
                            let on_load = on_load_profile.clone();
                            let profile_clone = profile.clone();
                            html! {
                                <button
                                    title={profile.description.clone()}
                                    onclick={Callback::from(move |_| on_load.emit(profile_clone.clone()))}
                                    style="border: none; padding: 5px 10px; border-radius: 4px; background: #6c757d; color: white; cursor: pointer; font-size: 12px;"
                                >
                                    {&profile.name}
                                </button>
                            }
                        }).collect::<Html>()
                    }
                </div>

                // Saved custom profiles
                {
                    if !profiles.is_empty() {
                        html! {
                            <>
                                <h4 style="margin: 10px 0 8px 0; font-size: 13px;">{"💾 Saved Profiles:"}</h4>
                                <div style="display: flex; gap: 6px; flex-wrap: wrap; margin-bottom: 10px;">
                                    {
                                        profiles.iter().map(|(name, profile)| {
                                            let on_load = on_load_profile.clone();
                                            let on_delete = on_delete_profile.clone();
                                            let profile_clone = profile.clone();
                                            let name_clone = name.clone();
                                            html! {
                                                <div style="display: flex; gap: 2px;">
                                                    <button
                                                        title={profile.description.clone()}
                                                        onclick={Callback::from(move |_| on_load.emit(profile_clone.clone()))}
                                                        style="border: none; padding: 5px 10px; border-radius: 4px 0 0 4px; background: #0d6efd; color: white; cursor: pointer; font-size: 12px;"
                                                    >
                                                        {name}
                                                    </button>
                                                    <button
                                                        title="Delete profile"
                                                        onclick={Callback::from(move |_| on_delete.emit(name_clone.clone()))}
                                                        style="border: none; padding: 5px 8px; border-radius: 0 4px 4px 0; background: #dc3545; color: white; cursor: pointer; font-size: 12px;"
                                                    >
                                                        {"×"}
                                                    </button>
                                                </div>
                                            }
                                        }).collect::<Html>()
                                    }
                                </div>
                            </>
                        }
                    } else {
                        html! {}
                    }
                }

                // Save profile button and form
                <div>
                    <button
                        onclick={on_save_profile_click}
                        style="border: none; padding: 5px 10px; border-radius: 4px; background: #28a745; color: white; cursor: pointer; font-size: 12px;"
                    >
                        {if *show_profile_save { "✕ Cancel" } else { "💾 Save Current Selection" }}
                    </button>

                    {
                        if *show_profile_save {
                            html! {
                                <div style="margin-top: 10px; padding: 10px; background: rgba(255,255,255,0.1); border-radius: 4px;">
                                    <input
                                        type="text"
                                        placeholder="Profile name (e.g., 'My Custom Profile')"
                                        value={(*profile_name_input).clone()}
                                        oninput={on_profile_name_input}
                                        style="width: 100%; padding: 6px; margin-bottom: 8px; border: 1px solid #ccc; border-radius: 4px;"
                                    />
                                    <input
                                        type="text"
                                        placeholder="Description (optional)"
                                        value={(*profile_desc_input).clone()}
                                        oninput={on_profile_desc_input}
                                        style="width: 100%; padding: 6px; margin-bottom: 8px; border: 1px solid #ccc; border-radius: 4px;"
                                    />
                                    {
                                        if let Some(error) = &*profile_error {
                                            html! { <p style="color: #dc3545; font-size: 12px; margin-bottom: 8px;">{error}</p> }
                                        } else {
                                            html! {}
                                        }
                                    }
                                    <button
                                        onclick={on_save_profile_submit}
                                        style="border: none; padding: 6px 12px; border-radius: 4px; background: #28a745; color: white; cursor: pointer; font-size: 12px;"
                                    >
                                        {"💾 Save Profile"}
                                    </button>
                                </div>
                            }
                        } else {
                            html! {}
                        }
                    }
                </div>
            </div>

            {
                if let Some(items) = &props.batch_items {
                    if items.len() > 1 {
                        let items = items.clone();
                        let on_download_json = {
                            let items = items.clone();
                            Callback::from(move |_| {
                                let json = generate_json_batch(&items);
                                let name = if items.len() == 1 { items[0].name.clone() } else { format!("{}-items", items.len()) };
                                download_file(&json, &format!("batch_{}_metadata.json", name), "application/json");
                            })
                        };
                        let on_download_csv = {
                            let items = items.clone();
                            Callback::from(move |_| {
                                let csv = generate_csv_batch(&items);
                                let name = if items.len() == 1 { items[0].name.clone() } else { format!("{}-items", items.len()) };
                                download_file(&csv, &format!("batch_{}_metadata.csv", name), "text/csv");
                            })
                        };
                        let on_download_txt = {
                            let items = items.clone();
                            Callback::from(move |_| {
                                let txt = generate_txt_batch(&items);
                                let name = if items.len() == 1 { items[0].name.clone() } else { format!("{}-items", items.len()) };
                                download_file(&txt, &format!("batch_{}_metadata.txt", name), "text/plain");
                            })
                        };
                        html! {
                            <div style={format!("margin-bottom: 12px; padding: 10px; border: 1px dashed {}; border-radius: 4px;", colors.border)}>
                                <div style="display:flex; align-items:center; justify-content:space-between; gap:8px; flex-wrap:wrap;">
                                    <div style={format!("font-size: 13px; color: {};", colors.text)}>
                                        {format!("Batch items loaded: {} • Combined export:", items.len())}
                                    </div>
                                    <div style="display:flex; gap: 8px; flex-wrap: wrap;">
                                        <button style="border:none; padding:6px 10px; border-radius:4px; background:#0d6efd; color:white; cursor:pointer; font-size:12px;" onclick={on_download_json}>{"⬇ JSON (combined)"}</button>
                                        <button style="border:none; padding:6px 10px; border-radius:4px; background:#28a745; color:white; cursor:pointer; font-size:12px;" onclick={on_download_csv}>{"⬇ CSV (table)"}</button>
                                        <button style="border:none; padding:6px 10px; border-radius:4px; background:#6c757d; color:white; cursor:pointer; font-size:12px;" onclick={on_download_txt}>{"⬇ TXT (concat)"}</button>
                                    </div>
                                </div>
                            </div>
                        }
                    } else { html!{} }
                } else { html!{} }
            }

            <div style={format!("margin-bottom: 15px; padding: 10px; background: {}; border-radius: 4px;", colors.checkbox_bg)}>
                <h4 style="margin: 0 0 10px 0; font-size: 14px;">{"Include in Export:"}</h4>
                <div style="display: flex; gap: 15px; flex-wrap: wrap;">
                    {
                        // Always show file info checkbox since we always have name and size
                        html! {
                            <label style="display: flex; align-items: center; gap: 5px; cursor: pointer;">
                                <input
                                    type="checkbox"
                                    checked={preferences.include_basic_info}
                                    onchange={{
                                        let preferences = preferences.clone();
                                        let on_change = props.on_preferences_change.clone();
                                        Callback::from(move |_| {
                                            let mut new_prefs = preferences.clone();
                                            new_prefs.update_and_save(|prefs| {
                                                prefs.include_basic_info = !prefs.include_basic_info;
                                            });
                                            on_change.emit(new_prefs);
                                        })
                                    }}
                                />
                                {
                                    if data.width.is_some() && data.height.is_some() {
                                        "File Info (name, size, dimensions)"
                                    } else {
                                        "File Info (name, size)"
                                    }
                                }
                            </label>
                        }
                    }
                    {
                        // Only show GPS checkbox if GPS data exists
                        if data.gps_coords.is_some() {
                            html! {
                                <label style="display: flex; align-items: center; gap: 5px; cursor: pointer;">
                                    <input
                                        type="checkbox"
                                        checked={preferences.include_gps}
                                        onchange={{
                                            let preferences = preferences.clone();
                                            let on_change = props.on_preferences_change.clone();
                                            Callback::from(move |_| {
                                                let mut new_prefs = preferences.clone();
                                                new_prefs.update_and_save(|prefs| {
                                                    prefs.include_gps = !prefs.include_gps;
                                                });
                                                on_change.emit(new_prefs);
                                            })
                                        }}
                                    />
                                    {"GPS Location"}
                                </label>
                            }
                        } else {
                            html! {}
                        }
                    }
                </div>
                <div style={format!("margin-top: 10px; font-size: 12px; color: {};", colors.help_text)}>
                    {format!("{} metadata fields selected", selected_metadata.len())}
                    {" • Use checkboxes above to select specific metadata"}
                </div>
            </div>

            <div style="display: flex; gap: 8px; flex-wrap: wrap;">
                // JSON format
                <div style="display: flex; gap: 2px;">
                    <button title="Download JSON"
                        onclick={if has_anything_to_export { export_json.clone() } else { Callback::noop() }}
                        disabled={!has_anything_to_export}
                        style={format!("border: none; padding: 6px 12px; border-radius: 4px 0 0 4px; font-weight: bold; font-size: 12px; {}",
                            if has_anything_to_export {
                                "background: #007bff; color: white; cursor: pointer;"
                            } else {
                                "background: #6c757d; color: #aaa; cursor: not-allowed;"
                            }
                        )}
                    >
                        {"📄 JSON"}
                    </button>
                    <button title="Copy JSON to clipboard"
                        onclick={if has_anything_to_export { copy_json.clone() } else { Callback::noop() }}
                        disabled={!has_anything_to_export}
                        style={format!("border: 1px solid #007bff; padding: 6px 8px; border-radius: 0 4px 4px 0; font-weight: bold; background: white; color: #007bff; font-size: 14px; {}",
                            if has_anything_to_export { "cursor: pointer;" } else { "cursor: not-allowed; color: #aaa; border-color: #6c757d;" }
                        )}
                    >
                        {"⧉"}
                    </button>
                </div>

                // CSV format
                <div style="display: flex; gap: 2px;">
                    <button title="Download CSV"
                        onclick={if has_anything_to_export { export_csv.clone() } else { Callback::noop() }}
                        disabled={!has_anything_to_export}
                        style={format!("border: none; padding: 6px 12px; border-radius: 4px 0 0 4px; font-weight: bold; font-size: 12px; {}",
                            if has_anything_to_export {
                                "background: #28a745; color: white; cursor: pointer;"
                            } else {
                                "background: #6c757d; color: #aaa; cursor: not-allowed;"
                            }
                        )}
                    >
                        {"📊 CSV"}
                    </button>
                    <button title="Copy CSV to clipboard"
                        onclick={if has_anything_to_export { copy_csv.clone() } else { Callback::noop() }}
                        disabled={!has_anything_to_export}
                        style={format!("border: 1px solid #28a745; padding: 6px 8px; border-radius: 0 4px 4px 0; font-weight: bold; background: white; color: #28a745; font-size: 14px; {}",
                            if has_anything_to_export { "cursor: pointer;" } else { "cursor: not-allowed; color: #aaa; border-color: #6c757d;" }
                        )}
                    >
                        {"⧉"}
                    </button>
                </div>

                // Text format
                <div style="display: flex; gap: 2px;">
                    <button title="Download Text"
                        onclick={if has_anything_to_export { export_txt.clone() } else { Callback::noop() }}
                        disabled={!has_anything_to_export}
                        style={format!("border: none; padding: 6px 12px; border-radius: 4px 0 0 4px; font-weight: bold; font-size: 12px; {}",
                            if has_anything_to_export {
                                "background: #6c757d; color: white; cursor: pointer;"
                            } else {
                                "background: #6c757d; color: #aaa; cursor: not-allowed;"
                            }
                        )}
                    >
                        {"📝 Text"}
                    </button>
                    <button title="Copy Text to clipboard"
                        onclick={if has_anything_to_export { copy_txt.clone() } else { Callback::noop() }}
                        disabled={!has_anything_to_export}
                        style={format!("border: 1px solid #6c757d; padding: 6px 8px; border-radius: 0 4px 4px 0; font-weight: bold; background: white; color: #6c757d; font-size: 14px; {}",
                            if has_anything_to_export { "cursor: pointer;" } else { "cursor: not-allowed; color: #aaa; border-color: #6c757d;" }
                        )}
                    >
                        {"⧉"}
                    </button>
                </div>

                // Markdown format
                <div style="display: flex; gap: 2px;">
                    <button title="Download Markdown"
                        onclick={if has_anything_to_export { export_md.clone() } else { Callback::noop() }}
                        disabled={!has_anything_to_export}
                        style={format!("border: none; padding: 6px 12px; border-radius: 4px 0 0 4px; font-weight: bold; font-size: 12px; {}",
                            if has_anything_to_export { "background: #6f42c1; color: white; cursor: pointer;" } else { "background: #6c757d; color: #aaa; cursor: not-allowed;" }
                        )}
                    >
                        {"🗒️ MD"}
                    </button>
                    <button title="Copy Markdown to clipboard"
                        onclick={if has_anything_to_export { copy_md.clone() } else { Callback::noop() }}
                        disabled={!has_anything_to_export}
                        style={format!("border: 1px solid #6f42c1; padding: 6px 8px; border-radius: 0 4px 4px 0; font-weight: bold; background: white; color: #6f42c1; font-size: 14px; {}",
                            if has_anything_to_export { "cursor: pointer;" } else { "cursor: not-allowed; color: #aaa; border-color: #6c757d;" }
                        )}
                    >
                        {"⧉"}
                    </button>
                </div>

                // YAML format
                <div style="display: flex; gap: 2px;">
                    <button title="Download YAML"
                        onclick={if has_anything_to_export { export_yaml.clone() } else { Callback::noop() }}
                        disabled={!has_anything_to_export}
                        style={format!("border: none; padding: 6px 12px; border-radius: 4px 0 0 4px; font-weight: bold; font-size: 12px; {}",
                            if has_anything_to_export { "background: #20c997; color: white; cursor: pointer;" } else { "background: #6c757d; color: #aaa; cursor: not-allowed;" }
                        )}
                    >
                        {"🧾 YAML"}
                    </button>
                    <button title="Copy YAML to clipboard"
                        onclick={if has_anything_to_export { copy_yaml.clone() } else { Callback::noop() }}
                        disabled={!has_anything_to_export}
                        style={format!("border: 1px solid #20c997; padding: 6px 8px; border-radius: 0 4px 4px 0; font-weight: bold; background: white; color: #20c997; font-size: 14px; {}",
                            if has_anything_to_export { "cursor: pointer;" } else { "cursor: not-allowed; color: #aaa; border-color: #6c757d;" }
                        )}
                    >
                        {"⧉"}
                    </button>
                </div>

                // XML format
                <div style="display: flex; gap: 2px;">
                    <button title="Download XML"
                        onclick={if has_anything_to_export { export_xml.clone() } else { Callback::noop() }}
                        disabled={!has_anything_to_export}
                        style={format!("border: none; padding: 6px 12px; border-radius: 4px 0 0 4px; font-weight: bold; font-size: 12px; {}",
                            if has_anything_to_export { "background: #17a2b8; color: white; cursor: pointer;" } else { "background: #6c757d; color: #aaa; cursor: not-allowed;" }
                        )}
                    >
                        {"🧩 XML"}
                    </button>
                    <button title="Copy XML to clipboard"
                        onclick={if has_anything_to_export { copy_xml.clone() } else { Callback::noop() }}
                        disabled={!has_anything_to_export}
                        style={format!("border: 1px solid #17a2b8; padding: 6px 8px; border-radius: 0 4px 4px 0; font-weight: bold; background: white; color: #17a2b8; font-size: 14px; {}",
                            if has_anything_to_export { "cursor: pointer;" } else { "cursor: not-allowed; color: #aaa; border-color: #6c757d;" }
                        )}
                    >
                        {"⧉"}
                    </button>
                </div>
            </div>
        </div>
    }
}
