use exif::{Exif, Field, In, Reader, Tag, Value};
use image::GenericImageView;
use js_sys::Uint8Array;
use serde::Serialize;
use std::collections::HashMap;
use std::io::Cursor;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Event, File, HtmlAnchorElement, HtmlInputElement, Url, window};
use yew::prelude::*;

#[derive(Clone, PartialEq, Serialize)]
struct ImageData {
    name: String,
    size: u64,
    #[serde(skip)] // Don't include data URL in exports
    data_url: String,
    width: Option<u32>,
    height: Option<u32>,
    exif_data: HashMap<String, String>,
    gps_coords: Option<(f64, f64)>, // (latitude, longitude)
}

#[function_component(App)]
fn app() -> Html {
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
        </div>
    }
}

async fn process_file(file: File) -> Result<ImageData, JsValue> {
    let name = file.name();
    let size = file.size() as u64;

    // Create data URL for image display
    let array_buffer = JsFuture::from(file.array_buffer()).await?;
    let uint8_array = Uint8Array::new(&array_buffer);
    let bytes = uint8_array.to_vec();

    // Create data URL
    let mime_type = file.type_();
    let data_url = format!("data:{};base64,{}", mime_type, base64_encode(&bytes));

    // Get image dimensions
    let (width, height) = get_image_dimensions(&bytes)?;

    // Extract EXIF data
    let (exif_data, gps_coords) = extract_exif_data(&bytes);

    Ok(ImageData {
        name,
        size,
        data_url,
        width: Some(width),
        height: Some(height),
        exif_data,
        gps_coords,
    })
}

fn get_image_dimensions(bytes: &[u8]) -> Result<(u32, u32), JsValue> {
    // Use the image crate to get dimensions
    match image::load_from_memory(bytes) {
        Ok(img) => {
            let dimensions = img.dimensions();
            Ok(dimensions)
        }
        Err(_) => Err(JsValue::from_str("Failed to parse image")),
    }
}

fn extract_exif_data(bytes: &[u8]) -> (HashMap<String, String>, Option<(f64, f64)>) {
    let mut exif_map = HashMap::new();
    let mut gps_coords = None;

    // Try to parse EXIF data
    if let Ok(exifreader) = Reader::new().read_from_container(&mut Cursor::new(bytes)) {
        for f in exifreader.fields() {
            let tag_name = format!("{}", f.tag);
            let value = format!("{}", f.display_value().with_unit(&exifreader));

            // Store the EXIF field
            exif_map.insert(tag_name.clone(), value);

            // Check for GPS coordinates
            match f.tag {
                Tag::GPSLatitude => {
                    if let Some(lat) = parse_gps_coordinate(f, &exifreader) {
                        if let Some((_, lon)) = gps_coords {
                            gps_coords = Some((lat, lon));
                        } else {
                            gps_coords = Some((lat, 0.0));
                        }
                    }
                }
                Tag::GPSLongitude => {
                    if let Some(lon) = parse_gps_coordinate(f, &exifreader) {
                        if let Some((lat, _)) = gps_coords {
                            gps_coords = Some((lat, lon));
                        } else {
                            gps_coords = Some((0.0, lon));
                        }
                    }
                }
                _ => {}
            }
        }

        // Apply GPS reference directions
        if let Some((mut lat, mut lon)) = gps_coords {
            // Check latitude reference (N/S)
            if let Some(lat_ref_field) = exifreader.get_field(Tag::GPSLatitudeRef, In::PRIMARY) {
                if let Value::Ascii(ref vec) = lat_ref_field.value {
                    if let Some(lat_ref) = vec.first() {
                        if !lat_ref.is_empty() && lat_ref[0] == b'S' {
                            lat = -lat;
                        }
                    }
                }
            }

            // Check longitude reference (E/W)
            if let Some(lon_ref_field) = exifreader.get_field(Tag::GPSLongitudeRef, In::PRIMARY) {
                if let Value::Ascii(ref vec) = lon_ref_field.value {
                    if let Some(lon_ref) = vec.first() {
                        if !lon_ref.is_empty() && lon_ref[0] == b'W' {
                            lon = -lon;
                        }
                    }
                }
            }

            gps_coords = Some((lat, lon));
        }
    }

    (exif_map, gps_coords)
}

fn parse_gps_coordinate(field: &Field, _exifreader: &Exif) -> Option<f64> {
    if let Value::Rational(ref rationals) = field.value {
        if rationals.len() >= 3 {
            let degrees = rationals[0].to_f64();
            let minutes = rationals[1].to_f64();
            let seconds = rationals[2].to_f64();

            Some(degrees + minutes / 60.0 + seconds / 3600.0)
        } else {
            None
        }
    } else {
        None
    }
}

fn base64_encode(bytes: &[u8]) -> String {
    let window = web_sys::window().unwrap();
    let btoa = js_sys::Reflect::get(&window, &JsValue::from_str("btoa")).unwrap();
    let btoa_fn: js_sys::Function = btoa.unchecked_into();

    // Convert bytes to string for btoa
    let binary_string = bytes.iter().map(|b| *b as char).collect::<String>();
    let result = btoa_fn
        .call1(&window, &JsValue::from_str(&binary_string))
        .unwrap();
    result.as_string().unwrap()
}

fn format_file_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    format!("{:.1} {}", size, UNITS[unit_index])
}

fn download_file(content: &str, filename: &str, mime_type: &str) {
    let window = window().unwrap();
    let document = window.document().unwrap();

    // Create blob and URL
    let blob_parts = js_sys::Array::new();
    blob_parts.push(&JsValue::from_str(content));

    let blob_options = web_sys::BlobPropertyBag::new();
    blob_options.set_type(mime_type);

    let blob =
        web_sys::Blob::new_with_str_sequence_and_options(&blob_parts, &blob_options).unwrap();

    let url = Url::create_object_url_with_blob(&blob).unwrap();

    // Create download link
    let link = document.create_element("a").unwrap();
    let link = link.dyn_into::<HtmlAnchorElement>().unwrap();

    link.set_href(&url);
    link.set_download(filename);
    link.style().set_property("display", "none").unwrap();

    // Trigger download
    document.body().unwrap().append_child(&link).unwrap();
    link.click();
    document.body().unwrap().remove_child(&link).unwrap();

    // Clean up URL
    Url::revoke_object_url(&url).unwrap();
}

fn generate_csv(data: &ImageData) -> String {
    let mut csv = String::new();
    csv.push_str("Property,Value\n");

    // Basic file info
    csv.push_str(&format!("Filename,\"{}\"\n", data.name));
    csv.push_str(&format!("File Size,{}\n", format_file_size(data.size)));

    if let (Some(width), Some(height)) = (data.width, data.height) {
        csv.push_str(&format!("Width,{}\n", width));
        csv.push_str(&format!("Height,{}\n", height));
        csv.push_str(&format!("Dimensions,\"{}x{} pixels\"\n", width, height));
    }

    // GPS data
    if let Some((lat, lon)) = data.gps_coords {
        csv.push_str(&format!("GPS Latitude,{}\n", lat));
        csv.push_str(&format!("GPS Longitude,{}\n", lon));
    }

    // EXIF data
    for (key, value) in &data.exif_data {
        csv.push_str(&format!(
            "\"{}\",\"{}\"\n",
            key.replace("\"", "\"\""),
            value.replace("\"", "\"\"")
        ));
    }

    csv
}

fn generate_txt(data: &ImageData) -> String {
    let mut txt = String::new();
    txt.push_str("IMAGE METADATA REPORT\n");
    txt.push_str("=====================\n\n");

    // Basic file info
    txt.push_str("FILE INFORMATION\n");
    txt.push_str("----------------\n");
    txt.push_str(&format!("Filename: {}\n", data.name));
    txt.push_str(&format!("File Size: {}\n", format_file_size(data.size)));

    if let (Some(width), Some(height)) = (data.width, data.height) {
        txt.push_str(&format!("Dimensions: {}x{} pixels\n", width, height));
    }
    txt.push('\n');

    // GPS data
    if let Some((lat, lon)) = data.gps_coords {
        txt.push_str("GPS LOCATION\n");
        txt.push_str("------------\n");
        txt.push_str(&format!("Latitude: {}\n", lat));
        txt.push_str(&format!("Longitude: {}\n", lon));
        txt.push_str(&format!(
            "Google Maps: https://maps.google.com/maps?q={},{}\n\n",
            lat, lon
        ));
    }

    // EXIF data
    if !data.exif_data.is_empty() {
        txt.push_str("EXIF METADATA\n");
        txt.push_str("-------------\n");
        for (key, value) in &data.exif_data {
            txt.push_str(&format!("{}: {}\n", key, value));
        }
    } else {
        txt.push_str("EXIF METADATA\n");
        txt.push_str("-------------\n");
        txt.push_str("No EXIF data found in this image\n");
    }

    txt
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<App>::new().render();
}
