use crate::metadata_info::{get_metadata_category, get_metadata_explanation};
use crate::types::ImageData;
use std::collections::{HashMap, HashSet};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MetadataDisplayProps {
    pub image_data: ImageData,
    pub selected_metadata: HashSet<String>,
    pub show_explanations: bool,
    pub on_metadata_selection_change: Callback<HashSet<String>>,
    pub on_toggle_explanations: Callback<web_sys::MouseEvent>,
}

#[function_component(MetadataDisplay)]
pub fn metadata_display(props: &MetadataDisplayProps) -> Html {
    let data = &props.image_data;
    let selected_metadata = &props.selected_metadata;
    let show_explanations = props.show_explanations;

    if data.exif_data.is_empty() {
        return html! {
            <div style="background: #f9f9f9; padding: 15px; border-radius: 4px; color: #666;">
                <h3>{"EXIF Metadata"}</h3>
                <p>{"No EXIF data found in this image"}</p>
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

    html! {
        <div style="background: #f0f8ff; padding: 15px; border-radius: 4px;">
            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 15px;">
                <h3 style="margin: 0;">{"EXIF Metadata"}</h3>
                <button
                    onclick={props.on_toggle_explanations.clone()}
                    style="background: #007bff; color: white; border: none; padding: 5px 10px; border-radius: 3px; cursor: pointer; font-size: 12px;"
                >
                    {if show_explanations { "Hide Info" } else { "Show Info" }}
                </button>
            </div>

            <div style="max-height: 400px; overflow-y: auto;">
                {
                    sorted_categories.iter().map(|(category, items)| {
                        html! {
                            <div key={*category} style="margin-bottom: 20px;">
                                <h4 style="margin: 0 0 10px 0; color: #555; font-size: 14px; border-bottom: 1px solid #ddd; padding-bottom: 5px;">
                                    {*category}
                                </h4>
                                {
                                    items.iter().map(|(key, value)| {
                                        let is_selected = selected_metadata.contains(*key);
                                        let key_clone = (*key).clone();
                                        let selected_metadata_clone = selected_metadata.clone();
                                        let on_change = props.on_metadata_selection_change.clone();

                                        html! {
                                            <div key={(*key).clone()} style="margin-bottom: 8px; padding: 5px; border-radius: 3px; background: rgba(255,255,255,0.5);">
                                                <div style="display: flex; align-items: flex-start; gap: 8px;">
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
                                                            <span>{*value}</span>
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
