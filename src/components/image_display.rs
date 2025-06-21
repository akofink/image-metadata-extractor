//! Renders the uploaded image and basic information.

use crate::types::ImageData;
use crate::utils::format_file_size;
use yew::prelude::*;

/// Properties for [`ImageDisplay`].
#[derive(Properties, PartialEq)]
pub struct ImageDisplayProps {
    pub image_data: ImageData,
    pub is_expanded: bool,
    pub on_image_click: Callback<web_sys::MouseEvent>,
    pub on_upload_new: Option<Callback<web_sys::MouseEvent>>,
}

/// Preview component showing the file and optional GPS information.
#[function_component(ImageDisplay)]
pub fn image_display(props: &ImageDisplayProps) -> Html {
    let data = &props.image_data;
    let is_expanded = props.is_expanded;
    let on_image_click = props.on_image_click.clone();

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

            <div style="background: #f5f5f5; padding: 15px; border-radius: 4px; margin-bottom: 20px;">
                <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 15px;">
                    <h3 style="margin: 0;">{"File Information"}</h3>
                    {
                        if let Some(on_upload_new) = &props.on_upload_new {
                            html! {
                                <button
                                    onclick={on_upload_new.clone()}
                                    style="background: #007bff; color: white; border: none; padding: 8px 16px; border-radius: 4px; cursor: pointer; font-size: 14px; transition: background-color 0.2s ease;"
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
            </div>

            {
                if let Some((lat, lon)) = data.gps_coords {
                    html! {
                        <div style="background: #e8f5e8; padding: 15px; border-radius: 4px; margin-bottom: 20px;">
                            <h3>{"GPS Location"}</h3>
                            <p><strong>{"Latitude: "}</strong>{lat}</p>
                            <p><strong>{"Longitude: "}</strong>{lon}</p>
                            <p>
                                <a href={format!("https://maps.google.com/maps?q={},{}", lat, lon)} target="_blank">{"Google Maps"}</a>
                                {" | "}
                                <a href={format!("https://maps.apple.com/?ll={},{}", lat, lon)} target="_blank">{"Apple Maps"}</a>
                                {" | "}
                                <a href={format!("https://www.openstreetmap.org/?mlat={}&mlon={}", lat, lon)} target="_blank">{"OpenStreetMap"}</a>
                            </p>
                        </div>
                    }
                } else {
                    html! {}
                }
            }
        </div>
    }
}
