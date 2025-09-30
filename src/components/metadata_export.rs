//! Allows users to download selected metadata in various formats.

use crate::export::{
    generate_csv, generate_csv_batch, generate_json_batch, generate_md, generate_txt,
    generate_txt_batch, generate_xml, generate_yaml,
};
use crate::preferences::UserPreferences;
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

    html! {
        <div style={format!("background: {}; padding: 15px; border-radius: 4px; margin-top: 20px; border: 1px solid {}; color: {};", colors.background, colors.border, colors.text)}>
            <h3>{"📊 Export Metadata"}</h3>
            <p style={format!("margin-bottom: 15px; color: {};", colors.text)}>
                {"Download selected metadata in your preferred format:"}
            </p>
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
