use crate::exif::process_file;
use crate::export::{generate_csv, generate_txt};
use crate::types::ImageData;
use crate::utils::{download_file, format_file_size};
use web_sys::{Event, HtmlInputElement};
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let image_data = use_state(|| None::<ImageData>);
    let is_expanded = use_state(|| false);

    let on_file_change = {
        let image_data = image_data.clone();
        let is_expanded = is_expanded.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if let Some(files) = input.files() {
                if let Some(file) = files.get(0) {
                    let image_data = image_data.clone();
                    let is_expanded = is_expanded.clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        if let Ok(data) = process_file(file).await {
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
        Callback::from(move |_| {
            if let Some(ref data) = *image_data {
                if let Ok(json) = serde_json::to_string_pretty(data) {
                    download_file(
                        &json,
                        &format!("{}_metadata.json", data.name),
                        "application/json",
                    );
                }
            }
        })
    };

    let export_csv = {
        let image_data = image_data.clone();
        Callback::from(move |_| {
            if let Some(ref data) = *image_data {
                let csv = generate_csv(data);
                download_file(&csv, &format!("{}_metadata.csv", data.name), "text/csv");
            }
        })
    };

    let export_txt = {
        let image_data = image_data.clone();
        Callback::from(move |_| {
            if let Some(ref data) = *image_data {
                let txt = generate_txt(data);
                download_file(&txt, &format!("{}_metadata.txt", data.name), "text/plain");
            }
        })
    };

    html! {
        <div style="max-width: 800px; margin: 0 auto; padding: 20px;">
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
                                    html! {
                                        <div style="background: #f0f8ff; padding: 15px; border-radius: 4px;">
                                            <h3>{"EXIF Metadata"}</h3>
                                            <div style="max-height: 400px; overflow-y: auto;">
                                                {
                                                    data.exif_data.iter().map(|(key, value)| {
                                                        html! {
                                                            <p key={key.clone()}>
                                                                <strong>{format!("{}: ", key)}</strong>
                                                                {value}
                                                            </p>
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

                            <div style="background: #fff3cd; padding: 15px; border-radius: 4px; margin-top: 20px; border: 1px solid #ffeaa7;">
                                <h3>{"Export Metadata"}</h3>
                                <p style="margin-bottom: 15px; color: #856404;">{"Download the extracted metadata in your preferred format:"}</p>
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
            
            <footer style="margin-top: 40px; padding-top: 20px; border-top: 1px solid #ddd; text-align: center; color: #666; font-size: 14px;">
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
