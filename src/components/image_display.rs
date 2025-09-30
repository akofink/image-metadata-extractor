//! Renders the uploaded image and basic information.

use crate::gps_privacy::{GpsPrecisionLevel, fuzz_coordinates};
use crate::types::{ImageData, Theme};
use crate::utils::{copy_to_clipboard, format_file_size};
use yew::prelude::*;

struct ImageDisplayColors {
    background: &'static str,
    text: &'static str,
    primary: &'static str,
    border: &'static str,
    hash_bg: &'static str,
    secondary_text: &'static str,
    gps_bg: &'static str,
}

const LIGHT_IMAGE_COLORS: ImageDisplayColors = ImageDisplayColors {
    background: "#f5f5f5",
    text: "#333333",
    primary: "#007bff",
    border: "#ddd",
    hash_bg: "#f8f9fa",
    secondary_text: "#666",
    gps_bg: "#e8f5e8",
};

const DARK_IMAGE_COLORS: ImageDisplayColors = ImageDisplayColors {
    background: "#1e1e1e",
    text: "#e0e0e0",
    primary: "#bb86fc",
    border: "#444",
    hash_bg: "#2d2d2d",
    secondary_text: "#aaa",
    gps_bg: "#1a3d1a",
};

/// Properties for [`ImageDisplay`].
#[derive(Properties, PartialEq)]
pub struct ImageDisplayProps {
    pub image_data: ImageData,
    pub is_expanded: bool,
    pub on_image_click: Callback<web_sys::MouseEvent>,
    pub on_upload_new: Option<Callback<web_sys::MouseEvent>>,
    pub theme: Theme,
}

/// Preview component showing the file and optional GPS information.
#[function_component(ImageDisplay)]
pub fn image_display(props: &ImageDisplayProps) -> Html {
    let data = &props.image_data;
    let is_expanded = props.is_expanded;
    let on_image_click = props.on_image_click.clone();

    // GPS privacy precision state
    let gps_precision = use_state(|| GpsPrecisionLevel::Exact);

    let colors = match props.theme {
        Theme::Light => LIGHT_IMAGE_COLORS,
        Theme::Dark => DARK_IMAGE_COLORS,
    };

    html! {
        <div>
            <div style="margin: 20px 0;">
                {
                    if data.mime_type.starts_with("image/") && data.mime_type != "image/svg+xml" {
                        html! {
                            <>
                                {
                                    if is_expanded {
                                        html! {
                                            <div
                                                style="position: fixed; top: 0; left: 0; width: 100%; height: 100%; background: rgba(0,0,0,0.8); z-index: 1000; display: flex; align-items: center; justify-content: center;"
                                                onclick={on_image_click.clone()}
                                            >
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
                                        style={format!("max-width: 300px; height: auto; border-radius: 4px; cursor: pointer; transition: transform 0.2s ease; {}",
                                            if is_expanded { "" } else { "box-shadow: 0 2px 8px rgba(0,0,0,0.1);" })}
                                        onclick={on_image_click}
                                    />
                                    <p style="margin: 10px 0 0 0; color: #666; font-size: 14px;">{"Click image to expand"}</p>
                                </div>
                            </>
                        }
                    } else {
                        html! {
                            <div style="text-align: center; padding: 40px 20px; background: #f8f9fa; border-radius: 8px; border: 2px dashed #dee2e6;">
                                <div style="font-size: 48px; margin-bottom: 16px;">
                                    {
                                        if data.mime_type == "application/pdf" {
                                            "📄"
                                        } else if data.mime_type == "image/svg+xml" {
                                            "🖼️"
                                        } else {
                                            "📎"
                                        }
                                    }
                                </div>
                                <p style="font-size: 16px; margin: 0; color: #666;">{format!("{} file", data.mime_type)}</p>
                                <p style="font-size: 14px; margin: 8px 0 0 0; color: #999;">{"Preview not available for this file type"}</p>
                            </div>
                        }
                    }
                }
            </div>

            <div style={format!("background: {}; padding: 15px; border-radius: 4px; margin-bottom: 20px; border: 1px solid {}; color: {};", colors.background, colors.border, colors.text)}>
                <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 15px;">
                    <h3 style="margin: 0;">{"File Information"}</h3>
                    {
                        if let Some(on_upload_new) = &props.on_upload_new {
                            html! {
                                <button
                                    onclick={on_upload_new.clone()}
                                    style={format!("background: {}; color: white; border: none; padding: 8px 16px; border-radius: 4px; cursor: pointer; font-size: 14px; transition: background-color 0.2s ease;", colors.primary)}
                                    class="upload-new-button"
                                >
                                    {"📁 Upload New File"}
                                </button>
                            }
                        } else {
                            html! {}
                        }
                    }
                </div>
                <p><strong>{"Name: "}</strong>{&data.name}</p>
                <p><strong>{"Size: "}</strong>{format_file_size(data.size)}</p>
                {
                    if let (Some(width), Some(height)) = (data.width, data.height) {
                        html! {
                            <p><strong>{"Dimensions: "}</strong>{format!("{}x{} pixels", width, height)}</p>
                        }
                    } else {
                        html! {}
                    }
                }
                {
                    if let Some(hash) = &data.sha256_hash {
                        let copy_hash = {
                            let hash = hash.clone();
                            Callback::from(move |_| {
                                copy_to_clipboard(&hash);
                            })
                        };

                        html! {
                            <div>
                                <p><strong>{"SHA-256 Hash: "}</strong></p>
                                <div style={format!("font-family: monospace; font-size: 12px; background: {}; padding: 8px; border-radius: 4px; word-break: break-all; margin: 4px 0; border: 1px solid {}; display: flex; justify-content: space-between; align-items: flex-start; gap: 8px;", colors.hash_bg, colors.border)}>
                                    <span style="flex: 1; line-height: 1.4;">{hash}</span>
                                    <button
                                        onclick={copy_hash}
                                        title="Copy hash to clipboard"
                                        style={format!("border: 1px solid {}; padding: 2px 4px; border-radius: 2px; font-weight: bold; background: {}; color: {}; font-size: 12px; cursor: pointer; flex-shrink: 0; line-height: 1;", colors.border, colors.background, colors.primary)}
                                    >
                                        {"⧉"}
                                    </button>
                                </div>
                                <p style={format!("font-size: 11px; color: {}; margin: 4px 0 0 0;", colors.secondary_text)}>{"Cryptographic fingerprint for forensics and deduplication"}</p>
                            </div>
                        }
                    } else {
                        html! {}
                    }
                }
            </div>

            {
                if let Some((lat, lon)) = data.gps_coords {
                    let (display_lat, display_lon) = fuzz_coordinates(lat, lon, *gps_precision);

                    let google_url = format!("https://maps.google.com/maps?q={},{}", display_lat, display_lon);
                    let apple_url = format!("https://maps.apple.com/?ll={},{}", display_lat, display_lon);
                    let osm_url = format!("https://www.openstreetmap.org/?mlat={}&mlon={}", display_lat, display_lon);

                    let copy_google = {
                        let url = google_url.clone();
                        Callback::from(move |_| copy_to_clipboard(&url))
                    };
                    let copy_apple = {
                        let url = apple_url.clone();
                        Callback::from(move |_| copy_to_clipboard(&url))
                    };
                    let copy_osm = {
                        let url = osm_url.clone();
                        Callback::from(move |_| copy_to_clipboard(&url))
                    };

                    let on_precision_change = {
                        let gps_precision = gps_precision.clone();
                        Callback::from(move |e: Event| {
                            let target = e.target_dyn_into::<web_sys::HtmlSelectElement>();
                            if let Some(select) = target {
                                let new_level = match select.value().as_str() {
                                    "street" => GpsPrecisionLevel::Street,
                                    "neighborhood" => GpsPrecisionLevel::Neighborhood,
                                    "city" => GpsPrecisionLevel::City,
                                    "region" => GpsPrecisionLevel::Region,
                                    _ => GpsPrecisionLevel::Exact,
                                };
                                gps_precision.set(new_level);
                            }
                        })
                    };

                    html! {
                        <div style={format!("background: {}; padding: 15px; border-radius: 4px; margin-bottom: 20px; border: 1px solid {}; color: {};", colors.gps_bg, colors.border, colors.text)}>
                            <h3>{"GPS Location"}</h3>

                            <div style="margin: 12px 0; padding: 10px; background: rgba(255, 193, 7, 0.1); border-left: 3px solid #ffc107; border-radius: 3px;">
                                <div style="margin-bottom: 8px;">
                                    <strong>{"🔒 Privacy Control:"}</strong>
                                </div>
                                <select
                                    onchange={on_precision_change}
                                    style={format!("width: 100%; padding: 6px; border: 1px solid {}; border-radius: 3px; background: {}; color: {};", colors.border, colors.background, colors.text)}
                                >
                                    <option value="exact" selected={*gps_precision == GpsPrecisionLevel::Exact}>{"Exact location (~1 meter)"}</option>
                                    <option value="street" selected={*gps_precision == GpsPrecisionLevel::Street}>{"Street level (~100 meters)"}</option>
                                    <option value="neighborhood" selected={*gps_precision == GpsPrecisionLevel::Neighborhood}>{"Neighborhood (~1 km)"}</option>
                                    <option value="city" selected={*gps_precision == GpsPrecisionLevel::City}>{"City level (~10 km)"}</option>
                                    <option value="region" selected={*gps_precision == GpsPrecisionLevel::Region}>{"Region level (~100 km)"}</option>
                                </select>
                                <p style="font-size: 11px; margin: 6px 0 0 0; color: #856404;">
                                    {"Adjust precision to protect your privacy when sharing location data."}
                                </p>
                            </div>

                            <p><strong>{"Latitude: "}</strong>{display_lat}</p>
                            <p><strong>{"Longitude: "}</strong>{display_lon}</p>
                            {
                                if *gps_precision != GpsPrecisionLevel::Exact {
                                    html! {
                                        <p style="font-size: 11px; font-style: italic; color: #888;">
                                            {format!("Original: {:.6}, {:.6}", lat, lon)}
                                        </p>
                                    }
                                } else {
                                    html! {}
                                }
                            }
                            <div style="margin-top: 12px;">
                                <p style="margin-bottom: 8px; font-weight: bold;">{"Map Links:"}</p>
                                <div style="display: flex; flex-direction: column; gap: 6px;">
                                    <div style="display: flex; align-items: center; gap: 8px;">
                                        <a href={google_url.clone()} target="_blank" style={format!("color: {}; text-decoration: none;", colors.text)}>{"🌍 Google Maps"}</a>
                                        <button
                                            onclick={copy_google}
                                            style={format!("background: {}; color: white; border: none; padding: 2px 8px; border-radius: 3px; cursor: pointer; font-size: 11px;", colors.primary)}
                                            title="Copy Google Maps link"
                                        >
                                            {"📋 Copy"}
                                        </button>
                                    </div>
                                    <div style="display: flex; align-items: center; gap: 8px;">
                                        <a href={apple_url.clone()} target="_blank" style={format!("color: {}; text-decoration: none;", colors.text)}>{"🍎 Apple Maps"}</a>
                                        <button
                                            onclick={copy_apple}
                                            style={format!("background: {}; color: white; border: none; padding: 2px 8px; border-radius: 3px; cursor: pointer; font-size: 11px;", colors.primary)}
                                            title="Copy Apple Maps link"
                                        >
                                            {"📋 Copy"}
                                        </button>
                                    </div>
                                    <div style="display: flex; align-items: center; gap: 8px;">
                                        <a href={osm_url.clone()} target="_blank" style={format!("color: {}; text-decoration: none;", colors.text)}>{"🗺️ OpenStreetMap"}</a>
                                        <button
                                            onclick={copy_osm}
                                            style={format!("background: {}; color: white; border: none; padding: 2px 8px; border-radius: 3px; cursor: pointer; font-size: 11px;", colors.primary)}
                                            title="Copy OpenStreetMap link"
                                        >
                                            {"📋 Copy"}
                                        </button>
                                    </div>
                                </div>
                            </div>
                        </div>
                    }
                } else {
                    html! {}
                }
            }
        </div>
    }
}
