use crate::exif::process_file;
use crate::export::{generate_csv, generate_txt};
use crate::image_cleaner::{create_cleaned_image, download_cleaned_image};
use crate::metadata_info::{get_metadata_explanation, get_metadata_category};
use crate::types::ImageData;
use crate::utils::{download_file, format_file_size};
use std::collections::{HashMap, HashSet};
use web_sys::{Event, HtmlInputElement};
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let image_data = use_state(|| None::<ImageData>);
    let is_expanded = use_state(|| false);
    let selected_metadata = use_state(|| HashSet::<String>::new());
    let include_basic_info = use_state(|| true);
    let include_gps = use_state(|| true);
    let show_explanations = use_state(|| false);
    let image_quality = use_state(|| 0.9); // JPEG quality for cleaned images

    let on_file_change = {
        let image_data = image_data.clone();
        let is_expanded = is_expanded.clone();
        let selected_metadata = selected_metadata.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if let Some(files) = input.files() {
                if let Some(file) = files.get(0) {
                    let image_data = image_data.clone();
                    let is_expanded = is_expanded.clone();
                    let selected_metadata = selected_metadata.clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        if let Ok(data) = process_file(file).await {
                            // Auto-select all metadata by default
                            let all_keys: HashSet<String> = data.exif_data.keys().cloned().collect();
                            selected_metadata.set(all_keys);
                            image_data.set(Some(data));
                            is_expanded.set(false); // Reset to thumbnail view
                        }
                    });
                }
            }
        })
    };

    let on_image_click = {
        let is_expanded = is_expanded.clone();
        Callback::from(move |_| {
            is_expanded.set(!*is_expanded);
        })
    };

    let export_json = {
        let image_data = image_data.clone();
        let selected_metadata = selected_metadata.clone();
        let include_basic_info = include_basic_info.clone();
        let include_gps = include_gps.clone();
        Callback::from(move |_| {
            if let Some(ref data) = *image_data {
                let filtered_data = data.filter_metadata(&*selected_metadata, *include_basic_info, *include_gps);
                if let Ok(json) = serde_json::to_string_pretty(&filtered_data) {
                    download_file(
                        &json,
                        &format!("{}_filtered_metadata.json", data.name),
                        "application/json",
                    );
                }
            }
        })
    };

    let export_csv = {
        let image_data = image_data.clone();
        let selected_metadata = selected_metadata.clone();
        let include_basic_info = include_basic_info.clone();
        let include_gps = include_gps.clone();
        Callback::from(move |_| {
            if let Some(ref data) = *image_data {
                let filtered_data = data.filter_metadata(&*selected_metadata, *include_basic_info, *include_gps);
                let csv = generate_csv(&filtered_data);
                download_file(&csv, &format!("{}_filtered_metadata.csv", data.name), "text/csv");
            }
        })
    };

    let export_txt = {
        let image_data = image_data.clone();
        let selected_metadata = selected_metadata.clone();
        let include_basic_info = include_basic_info.clone();
        let include_gps = include_gps.clone();
        Callback::from(move |_| {
            if let Some(ref data) = *image_data {
                let filtered_data = data.filter_metadata(&*selected_metadata, *include_basic_info, *include_gps);
                let txt = generate_txt(&filtered_data);
                download_file(&txt, &format!("{}_filtered_metadata.txt", data.name), "text/plain");
            }
        })
    };

    let download_cleaned_image_cb = {
        let image_data = image_data.clone();
        let image_quality = image_quality.clone();
        Callback::from(move |_| {
            if let Some(ref data) = *image_data {
                let data_url = data.data_url.clone();
                let filename = data.name.clone();
                let quality = *image_quality;
                
                wasm_bindgen_futures::spawn_local(async move {
                    if let Ok((cleaned_data_url, cleaned_filename)) = 
                        create_cleaned_image(&data_url, &filename, quality).await 
                    {
                        download_cleaned_image(&cleaned_data_url, &cleaned_filename);
                    }
                });
            }
        })
    };

    html! {
        <div style="min-height: 100vh; display: flex; flex-direction: column;">
            <div style="max-width: 800px; margin: 0 auto; padding: 20px; flex: 1;">
            <h1>{"Image Metadata Extractor"}</h1>

            <div style="margin: 20px 0;">
                <input
                    type="file"
                    accept="image/*"
                    onchange={on_file_change}
                    style="margin-bottom: 20px;"
                />
            </div>

            {
                if let Some(ref data) = *image_data {
                    html! {
                        <div>
                            <div style="margin: 20px 0;">
                                {
                                    if *is_expanded {
                                        html! {
                                            <div style="position: fixed; top: 0; left: 0; width: 100%; height: 100%; background: rgba(0,0,0,0.8); z-index: 1000; display: flex; align-items: center; justify-content: center;" onclick={on_image_click.clone()}>
                                                <img
                                                    src={data.data_url.clone()}
                                                    alt={data.name.clone()}
                                                    style="max-width: 90%; max-height: 90%; object-fit: contain; border-radius: 4px;"
                                                />
                                            </div>
                                        }
                                    } else {
                                        html! {}
                                    }
                                }
                                <div style="text-align: center;">
                                    <img
                                        src={data.data_url.clone()}
                                        alt={data.name.clone()}
                                        style={format!("max-width: 300px; max-height: 200px; object-fit: contain; border: 1px solid #ddd; border-radius: 4px; cursor: pointer; transition: transform 0.2s ease; {}",
                                            if *is_expanded { "" } else { "box-shadow: 0 2px 8px rgba(0,0,0,0.1);" })}
                                        onclick={on_image_click}
                                    />
                                    <p style="margin: 10px 0 0 0; color: #666; font-size: 14px;">{"Click image to expand"}</p>
                                </div>
                            </div>

                            <div style="background: #f5f5f5; padding: 15px; border-radius: 4px; margin-bottom: 20px;">
                                <h3>{"File Information"}</h3>
                                <p><strong>{"Name: "}</strong>{&data.name}</p>
                                <p><strong>{"Size: "}</strong>{format_file_size(data.size)}</p>
                                {
                                    if let (Some(width), Some(height)) = (data.width, data.height) {
                                        html! {
                                            <p><strong>{"Dimensions: "}</strong>{format!("{}x{} pixels", width, height)}</p>
                                        }
                                    } else {
                                        html! { <p><strong>{"Dimensions: "}</strong>{"Loading..."}</p> }
                                    }
                                }
                            </div>

                            {
                                if let Some((lat, lon)) = data.gps_coords {
                                    html! {
                                        <div style="background: #e8f5e8; padding: 15px; border-radius: 4px; margin-bottom: 20px;">
                                            <h3>{"GPS Location"}</h3>
                                            <p><strong>{"Latitude: "}</strong>{lat}</p>
                                            <p><strong>{"Longitude: "}</strong>{lon}</p>
                                            <p><a href={format!("https://maps.google.com/maps?q={},{}", lat, lon)} target="_blank">{"View on Google Maps"}</a></p>
                                        </div>
                                    }
                                } else {
                                    html! {}
                                }
                            }

                            {
                                if !data.exif_data.is_empty() {
                                    // Group metadata by category
                                    let mut categorized: HashMap<&str, Vec<(&String, &String)>> = HashMap::new();
                                    for (key, value) in &data.exif_data {
                                        let category = get_metadata_category(key);
                                        categorized.entry(category).or_insert_with(Vec::new).push((key, value));
                                    }

                                    html! {
                                        <div style="background: #f0f8ff; padding: 15px; border-radius: 4px;">
                                            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 15px;">
                                                <h3 style="margin: 0;">{"EXIF Metadata"}</h3>
                                                <button 
                                                    onclick={{
                                                        let show_explanations = show_explanations.clone();
                                                        Callback::from(move |_| show_explanations.set(!*show_explanations))
                                                    }}
                                                    style="background: #007bff; color: white; border: none; padding: 5px 10px; border-radius: 3px; cursor: pointer; font-size: 12px;"
                                                >
                                                    {if *show_explanations { "Hide Info" } else { "Show Info" }}
                                                </button>
                                            </div>
                                            
                                            <div style="max-height: 400px; overflow-y: auto;">
                                                {
                                                    categorized.iter().map(|(category, items)| {
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
                                                                        
                                                                        html! {
                                                                            <div key={(*key).clone()} style="margin-bottom: 8px; padding: 5px; border-radius: 3px; background: rgba(255,255,255,0.5);">
                                                                                <div style="display: flex; align-items: flex-start; gap: 8px;">
                                                                                    <input 
                                                                                        type="checkbox"
                                                                                        checked={is_selected}
                                                                                        onchange={Callback::from(move |_| {
                                                                                            let mut current = (*selected_metadata_clone).clone();
                                                                                            if current.contains(&key_clone) {
                                                                                                current.remove(&key_clone);
                                                                                            } else {
                                                                                                current.insert(key_clone.clone());
                                                                                            }
                                                                                            selected_metadata_clone.set(current);
                                                                                        })}
                                                                                        style="margin-top: 2px;"
                                                                                    />
                                                                                    <div style="flex: 1;">
                                                                                        <div style="margin-bottom: 2px;">
                                                                                            <strong>{format!("{}: ", key)}</strong>
                                                                                            <span>{*value}</span>
                                                                                        </div>
                                                                                        {
                                                                                            if *show_explanations {
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
                                } else {
                                    html! {
                                        <div style="background: #f9f9f9; padding: 15px; border-radius: 4px; color: #666;">
                                            <h3>{"EXIF Metadata"}</h3>
                                            <p>{"No EXIF data found in this image"}</p>
                                        </div>
                                    }
                                }
                            }

                            <div style="background: #d1ecf1; padding: 15px; border-radius: 4px; margin-top: 20px; border: 1px solid #bee5eb;">
                                <h3>{"🖼️ Download Cleaned Image"}</h3>
                                <p style="margin-bottom: 15px; color: #0c5460;">{"Download your image with all metadata removed for privacy:"}</p>
                                
                                <div style="margin-bottom: 15px; padding: 10px; background: rgba(255,255,255,0.7); border-radius: 4px;">
                                    <div style="display: flex; align-items: center; gap: 15px; flex-wrap: wrap;">
                                        <label style="display: flex; align-items: center; gap: 5px;">
                                            {"JPEG Quality:"}
                                            <input 
                                                type="range"
                                                min="0.3"
                                                max="1.0"
                                                step="0.1"
                                                value={image_quality.to_string()}
                                                oninput={{
                                                    let image_quality = image_quality.clone();
                                                    Callback::from(move |e: web_sys::InputEvent| {
                                                        let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                                        if let Ok(val) = input.value().parse::<f64>() {
                                                            image_quality.set(val);
                                                        }
                                                    })
                                                }}
                                                style="margin: 0 5px;"
                                            />
                                            <span style="font-size: 12px; color: #666;">
                                                {format!("{}%", (*image_quality * 100.0) as u8)}
                                            </span>
                                        </label>
                                    </div>
                                    <div style="margin-top: 8px; font-size: 12px; color: #666;">
                                        {"Removes ALL metadata including GPS, camera info, and EXIF data"}
                                    </div>
                                </div>
                                
                                <button
                                    onclick={download_cleaned_image_cb}
                                    style="background: #17a2b8; color: white; border: none; padding: 10px 20px; border-radius: 4px; cursor: pointer; font-weight: bold; font-size: 14px;"
                                >
                                    {"🧹 Download Privacy-Safe Image"}
                                </button>
                            </div>

                            <div style="background: #fff3cd; padding: 15px; border-radius: 4px; margin-top: 20px; border: 1px solid #ffeaa7;">
                                <h3>{"📊 Export Metadata"}</h3>
                                <p style="margin-bottom: 15px; color: #856404;">{"Download selected metadata in your preferred format:"}</p>
                                
                                <div style="margin-bottom: 15px; padding: 10px; background: rgba(255,255,255,0.7); border-radius: 4px;">
                                    <h4 style="margin: 0 0 10px 0; font-size: 14px;">{"Include in Export:"}</h4>
                                    <div style="display: flex; gap: 15px; flex-wrap: wrap;">
                                        <label style="display: flex; align-items: center; gap: 5px; cursor: pointer;">
                                            <input 
                                                type="checkbox"
                                                checked={*include_basic_info}
                                                onchange={{
                                                    let include_basic_info = include_basic_info.clone();
                                                    Callback::from(move |_| include_basic_info.set(!*include_basic_info))
                                                }}
                                            />
                                            {"File Info (name, size, dimensions)"}
                                        </label>
                                        <label style="display: flex; align-items: center; gap: 5px; cursor: pointer;">
                                            <input 
                                                type="checkbox"
                                                checked={*include_gps}
                                                onchange={{
                                                    let include_gps = include_gps.clone();
                                                    Callback::from(move |_| include_gps.set(!*include_gps))
                                                }}
                                            />
                                            {"GPS Location"}
                                        </label>
                                    </div>
                                    <div style="margin-top: 10px; font-size: 12px; color: #666;">
                                        {format!("{} EXIF fields selected", selected_metadata.len())}
                                        {" • Use checkboxes above to select specific metadata"}
                                    </div>
                                </div>
                                
                                <div style="display: flex; gap: 10px; flex-wrap: wrap;">
                                    <button
                                        onclick={export_json}
                                        style="background: #007bff; color: white; border: none; padding: 8px 16px; border-radius: 4px; cursor: pointer; font-weight: bold;"
                                    >
                                        {"📄 JSON"}
                                    </button>
                                    <button
                                        onclick={export_csv}
                                        style="background: #28a745; color: white; border: none; padding: 8px 16px; border-radius: 4px; cursor: pointer; font-weight: bold;"
                                    >
                                        {"📊 CSV"}
                                    </button>
                                    <button
                                        onclick={export_txt}
                                        style="background: #6c757d; color: white; border: none; padding: 8px 16px; border-radius: 4px; cursor: pointer; font-weight: bold;"
                                    >
                                        {"📝 Text"}
                                    </button>
                                </div>
                            </div>
                        </div>
                    }
                } else {
                    html! {
                        <p style="color: #666;">{"Select an image file to view its metadata"}</p>
                    }
                }
            }
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
                    {"© 2024 Image Metadata Extractor • "} 
                    <a href="mailto:contact@image-metadata-extractor.com" style="color: #007bff; text-decoration: none;">
                        {"Contact"}
                    </a>
                </p>
            </footer>
        </div>
    }
}
