//! Displays extracted metadata with selection controls.

use crate::metadata_info::{get_metadata_category, get_metadata_explanation};
use crate::types::{ImageData, Theme};
use std::collections::{HashMap, HashSet};
use yew::prelude::*;

struct MetadataColors {
    background: &'static str,
    text: &'static str,
    primary: &'static str,
    secondary: &'static str,
    border: &'static str,
    section_bg: &'static str,
}

const LIGHT_METADATA_COLORS: MetadataColors = MetadataColors {
    background: "#f0f8ff",
    text: "#333333",
    primary: "#007bff",
    secondary: "#28a745",
    border: "#ddd",
    section_bg: "#f9f9f9",
};

const DARK_METADATA_COLORS: MetadataColors = MetadataColors {
    background: "#1e1e1e",
    text: "#e0e0e0",
    primary: "#bb86fc",
    secondary: "#03dac6",
    border: "#444",
    section_bg: "#2d2d2d",
};

/// Properties for [`MetadataDisplay`].
#[derive(Properties)]
pub struct MetadataDisplayProps {
    pub image_data: ImageData,
    pub selected_metadata: HashSet<String>,
    pub show_explanations: bool,
    pub on_metadata_selection_change: Callback<HashSet<String>>,
    pub on_toggle_explanations: Callback<web_sys::MouseEvent>,
    pub theme: Theme,
}

impl PartialEq for MetadataDisplayProps {
    fn eq(&self, other: &Self) -> bool {
        self.image_data == other.image_data
            && self.selected_metadata == other.selected_metadata
            && self.show_explanations == other.show_explanations
            && self.theme == other.theme
    }
}

/// Shows metadata grouped by category with checkboxes and explanations.
#[function_component(MetadataDisplay)]
pub fn metadata_display(props: &MetadataDisplayProps) -> Html {
    let data = &props.image_data;
    let selected_metadata = &props.selected_metadata;
    let show_explanations = props.show_explanations;

    let colors = match props.theme {
        Theme::Light => LIGHT_METADATA_COLORS,
        Theme::Dark => DARK_METADATA_COLORS,
    };

    if data.exif_data.is_empty() {
        return html! {
            <div style={format!("background: {}; padding: 15px; border-radius: 4px; color: {}; border: 1px solid {};", colors.section_bg, colors.text, colors.border)}>
                <h3>{"Metadata"}</h3>
                <p>{"No metadata found in this file"}</p>
            </div>
        };
    }

    // Memoize the expensive categorization and sorting
    let sorted_categories = use_memo(data.exif_data.clone(), |exif_data| {
        // Group metadata by category
        let mut categorized: HashMap<&str, Vec<(String, String)>> = HashMap::new();
        for (key, value) in exif_data {
            let category = get_metadata_category(key);
            categorized
                .entry(category)
                .or_default()
                .push((key.clone(), value.clone()));
        }

        // Sort categories alphabetically and items within each category
        let mut sorted: Vec<_> = categorized.into_iter().collect();
        sorted.sort_by_key(|(category, _)| *category);
        for (_, items) in &mut sorted {
            items.sort_by(|(a, _), (b, _)| a.cmp(b));
        }
        sorted
    });

    // Calculate global select/deselect state
    let all_keys: HashSet<String> = data.exif_data.keys().cloned().collect();
    let _all_selected = selected_metadata.len() == all_keys.len() && !all_keys.is_empty();

    html! {
        <div style={format!("background: {}; padding: 15px; border-radius: 4px; border: 1px solid {}; color: {};", colors.background, colors.border, colors.text)}>
            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 15px;">
                <h3 style="margin: 0;">{"Metadata"}</h3>
                <div style="display: flex; gap: 10px;">
                    <div style="display: flex; gap: 5px;">
                        <button
                            onclick={{
                                let on_change = props.on_metadata_selection_change.clone();
                                let all_keys = all_keys.clone();
                                Callback::from(move |_| {
                                    on_change.emit(all_keys.clone());
                                })
                            }}
                            style={format!("background: {}; color: white; border: none; padding: 3px 8px; border-radius: 3px; cursor: pointer; font-size: 11px;", colors.secondary)}
                        >
                            {"Select All"}
                        </button>
                        <button
                            onclick={{
                                let on_change = props.on_metadata_selection_change.clone();
                                Callback::from(move |_| {
                                    on_change.emit(HashSet::new());
                                })
                            }}
                            style="background: #dc3545; color: white; border: none; padding: 3px 8px; border-radius: 3px; cursor: pointer; font-size: 11px;"
                        >
                            {"Deselect All"}
                        </button>
                    </div>
                    <button
                        onclick={props.on_toggle_explanations.clone()}
                        title={if show_explanations { "Hide explanations for metadata fields" } else { "Show helpful explanations for each metadata field" }}
                        style={format!("background: {}; color: white; border: none; padding: 5px 10px; border-radius: 3px; cursor: pointer; font-size: 12px;", colors.primary)}
                    >
                        {if show_explanations { "Hide Info" } else { "Show Info" }}
                    </button>
                </div>
            </div>

            <div>
                {
                    sorted_categories.iter().map(|(category, items)| {
                        // Calculate per-category select/deselect state
                        let category_keys: HashSet<String> = items.iter().map(|(key, _)| key.clone()).collect();
                        let category_selected_count = category_keys.iter().filter(|key| selected_metadata.contains(*key)).count();
                        let _all_category_selected = category_selected_count == category_keys.len();

                        html! {
                            <div key={*category} style="margin-bottom: 20px;">
                                <div style={format!("display: flex; justify-content: space-between; align-items: center; margin-bottom: 10px; border-bottom: 1px solid {}; padding-bottom: 5px;", colors.border)}>
                                    <h4 style={format!("margin: 0; color: {}; font-size: 14px;", colors.text)}>
                                        {*category}
                                    </h4>
                                    <div style="display: flex; gap: 3px;">
                                        <button
                                            onclick={{
                                                let on_change = props.on_metadata_selection_change.clone();
                                                let selected_metadata = props.selected_metadata.clone();
                                                let category_keys = category_keys.clone();
                                                Callback::from(move |_| {
                                                    // Add all category keys to current selection
                                                    let mut new_selection = selected_metadata.clone();
                                                    for key in &category_keys {
                                                        new_selection.insert(key.clone());
                                                    }
                                                    on_change.emit(new_selection);
                                                })
                                            }}
                                            style={format!("background: {}; color: white; border: none; padding: 2px 6px; border-radius: 2px; cursor: pointer; font-size: 10px;", colors.secondary)}
                                        >
                                            {"All"}
                                        </button>
                                        <button
                                            onclick={{
                                                let on_change = props.on_metadata_selection_change.clone();
                                                let selected_metadata = props.selected_metadata.clone();
                                                let category_keys = category_keys.clone();
                                                Callback::from(move |_| {
                                                    // Remove all category keys from current selection
                                                    let mut new_selection = selected_metadata.clone();
                                                    for key in &category_keys {
                                                        new_selection.remove(key);
                                                    }
                                                    on_change.emit(new_selection);
                                                })
                                            }}
                                            style="background: #dc3545; color: white; border: none; padding: 2px 6px; border-radius: 2px; cursor: pointer; font-size: 10px;"
                                        >
                                            {"None"}
                                        </button>
                                    </div>
                                </div>
                                {
                                    items.iter().map(|(key, value)| {
                                        let is_selected = selected_metadata.contains(key);
                                        let key_str = key.clone();

                                        let on_checkbox_change = {
                                            let on_change = props.on_metadata_selection_change.clone();
                                            let selected_metadata = props.selected_metadata.clone();
                                            let key = key_str.clone();

                                            Callback::from(move |_| {
                                                let mut current = selected_metadata.clone();
                                                if current.contains(&key) {
                                                    current.remove(&key);
                                                } else {
                                                    current.insert(key.clone());
                                                }
                                                on_change.emit(current);
                                            })
                                        };

                                        html! {
                                            <div key={key_str.clone()} style={format!("margin-bottom: 12px; padding: 8px; border-radius: 4px; background: {}; border: 1px solid {};", colors.section_bg, colors.border)}>
                                                <div style="display: flex; align-items: flex-start; gap: 12px;">
                                                    <input
                                                        type="checkbox"
                                                        checked={is_selected}
                                                        onchange={on_checkbox_change}
                                                        style="margin-top: 2px;"
                                                    />
                                                    <div style="flex: 1;">
                                                        <div style={format!("margin-bottom: 2px; color: {};", colors.text)}>
                                                            <strong>{format!("{}: ", key)}</strong>
                                                            <span style="word-break: break-all; overflow-wrap: break-word;">{value}</span>
                                                        </div>
                                                        {
                                                            if show_explanations {
                                                                if let Some(explanation) = get_metadata_explanation(key) {
                                                                    html! {
                                                                        <div style={format!("font-size: 11px; color: {}; font-style: italic; margin-top: 2px;", if props.theme == Theme::Light { "#666" } else { "#aaa" })}>
                                                                            {explanation}
                                                                        </div>
                                                                    }
                                                                } else {
                                                                    html! {}
                                                                }
                                                            } else {
                                                                html! {}
                                                            }
                                                        }
                                                    </div>
                                                </div>
                                            </div>
                                        }
                                    }).collect::<Html>()
                                }
                            </div>
                        }
                    }).collect::<Html>()
                }
            </div>
        </div>
    }
}
