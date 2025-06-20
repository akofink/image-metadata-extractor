use crate::types::ImageData;
use crate::utils::format_file_size;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ImageDisplayProps {
    pub image_data: ImageData,
    pub is_expanded: bool,
    pub on_image_click: Callback<web_sys::MouseEvent>,
}

#[function_component(ImageDisplay)]
pub fn image_display(props: &ImageDisplayProps) -> Html {
    let data = &props.image_data;
    let is_expanded = props.is_expanded;
    let on_image_click = props.on_image_click.clone();

    html! {
        <div>
            <div style="margin: 20px 0;">
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
                        style={format!("width: 100%; max-width: 300px; max-height: 200px; object-fit: contain; border: 1px solid #ddd; border-radius: 4px; cursor: pointer; transition: transform 0.2s ease; {}",
                            if is_expanded { "" } else { "box-shadow: 0 2px 8px rgba(0,0,0,0.1);" })}
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
        </div>
    }
}
