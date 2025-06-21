//! Displays extracted metadata with selection controls.

use crate::metadata_info::{get_metadata_category, get_metadata_explanation};
use crate::types::ImageData;
use std::collections::{HashMap, HashSet};
use yew::prelude::*;

/// Properties for [`MetadataDisplay`].
#[derive(Properties, PartialEq)]
pub struct MetadataDisplayProps {
    pub image_data: ImageData,
    pub selected_metadata: HashSet<String>,
    pub show_explanations: bool,
    pub on_metadata_selection_change: Callback<HashSet<String>>,
    pub on_toggle_explanations: Callback<web_sys::MouseEvent>,
}

/// Shows metadata grouped by category with checkboxes and explanations.
#[function_component(MetadataDisplay)]
pub fn metadata_display(props: &MetadataDisplayProps) -> Html {
    let data = &props.image_data;
    let selected_metadata = &props.selected_metadata;
    let show_explanations = props.show_explanations;

    if data.exif_data.is_empty() {
        return html! {
            <div style="background: #f9f9f9; padding: 15px; border-radius: 4px; color: #666;">
                <h3>{"Metadata"}</h3>
                <p>{"No metadata found in this file"}</p>
            </div>
        };
    }

    // Group metadata by category
    let mut categorized: HashMap<&str, Vec<(&String, &String)>> = HashMap::new();
    for (key, value) in &data.exif_data {
        let category = get_metadata_category(key);
        categorized.entry(category).or_default().push((key, value));
    }

    // Sort categories alphabetically and items within each category
    let mut sorted_categories: Vec<_> = categorized.into_iter().collect();
    sorted_categories.sort_by_key(|(category, _)| *category);
    for (_, items) in &mut sorted_categories {
        items.sort_by_key(|(key, _)| key.as_str());
    }

    // Calculate global select/deselect state
    let all_keys: HashSet<String> = data.exif_data.keys().cloned().collect();
    let _all_selected = selected_metadata.len() == all_keys.len() && !all_keys.is_empty();

    html! {
        <div style="background: #f0f8ff; padding: 15px; border-radius: 4px;">
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
                            style="background: #28a745; color: white; border: none; padding: 3px 8px; border-radius: 3px; cursor: pointer; font-size: 11px;"
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
                        style="background: #007bff; color: white; border: none; padding: 5px 10px; border-radius: 3px; cursor: pointer; font-size: 12px;"
                    >
                        {if show_explanations { "Hide Info" } else { "Show Info" }}
                    </button>
                </div>
            </div>

            <div>
                {
                    sorted_categories.iter().map(|(category, items)| {
                        // Calculate per-category select/deselect state
                        let category_keys: HashSet<String> = items.iter().map(|(key, _)| (*key).clone()).collect();
                        let category_selected_count = category_keys.iter().filter(|key| selected_metadata.contains(*key)).count();
                        let _all_category_selected = category_selected_count == category_keys.len();

                        html! {
                            <div key={*category} style="margin-bottom: 20px;">
                                <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 10px; border-bottom: 1px solid #ddd; padding-bottom: 5px;">
                                    <h4 style="margin: 0; color: #555; font-size: 14px;">
                                        {*category}
                                    </h4>
                                    <div style="display: flex; gap: 3px;">
                                        <button
                                            onclick={{
                                                let on_change = props.on_metadata_selection_change.clone();
                                                let selected_metadata = selected_metadata.clone();
                                                let category_keys = category_keys.clone();
                                                Callback::from(move |_| {
                                                    let mut current = selected_metadata.clone();
                                                    for key in &category_keys {
                                                        current.insert(key.clone());
                                                    }
                                                    on_change.emit(current);
                                                })
                                            }}
                                            style="background: #28a745; color: white; border: none; padding: 2px 6px; border-radius: 2px; cursor: pointer; font-size: 10px;"
                                        >
                                            {"All"}
                                        </button>
                                        <button
                                            onclick={{
                                                let on_change = props.on_metadata_selection_change.clone();
                                                let selected_metadata = selected_metadata.clone();
                                                let category_keys = category_keys.clone();
                                                Callback::from(move |_| {
                                                    let mut current = selected_metadata.clone();
                                                    for key in &category_keys {
                                                        current.remove(key);
                                                    }
                                                    on_change.emit(current);
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
                                        let is_selected = selected_metadata.contains(*key);
                                        let key_clone = (*key).clone();
                                        let selected_metadata_clone = selected_metadata.clone();
                                        let on_change = props.on_metadata_selection_change.clone();

                                        html! {
                                            <div key={(*key).clone()} style="margin-bottom: 12px; padding: 8px; border-radius: 4px; background: rgba(255,255,255,0.5);">
                                                <div style="display: flex; align-items: flex-start; gap: 12px;">
                                                    <input
                                                        type="checkbox"
                                                        checked={is_selected}
                                                        onchange={Callback::from(move |_| {
                                                            let mut current = selected_metadata_clone.clone();
                                                            if current.contains(&key_clone) {
                                                                current.remove(&key_clone);
                                                            } else {
                                                                current.insert(key_clone.clone());
                                                            }
                                                            on_change.emit(current);
                                                        })}
                                                        style="margin-top: 2px;"
                                                    />
                                                    <div style="flex: 1;">
                                                        <div style="margin-bottom: 2px;">
                                                            <strong>{format!("{}: ", key)}</strong>
                                                            <span style="word-break: break-all; overflow-wrap: break-word;">{*value}</span>
                                                        </div>
                                                        {
                                                            if show_explanations {
                                                                if let Some(explanation) = get_metadata_explanation(key) {
                                                                    html! {
                                                                        <div style="font-size: 11px; color: #666; font-style: italic; margin-top: 2px;">
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
