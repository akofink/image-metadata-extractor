use crate::image_cleaner::{create_cleaned_image, download_cleaned_image};
use crate::types::ImageData;
use web_sys::{Event, HtmlSelectElement};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ImageCleanerProps {
    pub image_data: ImageData,
}

#[function_component(ImageCleaner)]
pub fn image_cleaner(props: &ImageCleanerProps) -> Html {
    let image_quality = use_state(|| 0.9);
    let initial_format = if props.image_data.mime_type.starts_with("image/png") {
        "png".to_string()
    } else {
        "jpeg".to_string()
    };
    let selected_format = use_state(|| initial_format);

    {
        let selected_format = selected_format.clone();
        use_effect_with(props.image_data.mime_type.clone(), move |mime_type| {
            if mime_type.starts_with("image/png") {
                selected_format.set("png".to_string());
            } else {
                selected_format.set("jpeg".to_string());
            }
            || ()
        });
    }

    let download_cleaned_image_cb = {
        let data = props.image_data.clone();
        let image_quality = image_quality.clone();
        let selected_format = selected_format.clone();

        Callback::from(move |_| {
            let data_url = data.data_url.clone();
            let filename = data.name.clone();
            let quality = *image_quality;
            let format = (*selected_format).clone();

            wasm_bindgen_futures::spawn_local(async move {
                if let Ok((cleaned_data_url, cleaned_filename)) =
                    create_cleaned_image(&data_url, &filename, quality, &format).await
                {
                    download_cleaned_image(&cleaned_data_url, &cleaned_filename);
                }
            });
        })
    };

    html! {
        <div style="background: #d1ecf1; padding: 15px; border-radius: 4px; margin-top: 20px; border: 1px solid #bee5eb;">
            <h3>{"🖼️ Download Cleaned Image"}</h3>
            <p style="margin-bottom: 15px; color: #0c5460;">
                {"Download your image with all metadata removed for privacy:"}
            </p>

            <div style="margin-bottom: 15px; padding: 10px; background: rgba(255,255,255,0.7); border-radius: 4px;">
                <div style="display: flex; align-items: center; gap: 15px; flex-wrap: wrap; margin-bottom: 10px;">
                    <FormatSelector
                        selected_format={(*selected_format).clone()}
                        on_format_change={{
                            let selected_format = selected_format.clone();
                            Callback::from(move |format: String| {
                                selected_format.set(format);
                            })
                        }}
                    />

                    {
                        if *selected_format == "jpeg" {
                            html! {
                                <QualitySlider
                                    quality={*image_quality}
                                    on_quality_change={{
                                        let image_quality = image_quality.clone();
                                        Callback::from(move |quality: f64| {
                                            image_quality.set(quality);
                                        })
                                    }}
                                />
                            }
                        } else {
                            html! {}
                        }
                    }
                </div>
                <div style="font-size: 12px; color: #666;">
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
    }
}

#[derive(Properties, PartialEq)]
struct FormatSelectorProps {
    pub selected_format: String,
    pub on_format_change: Callback<String>,
}

#[function_component(FormatSelector)]
fn format_selector(props: &FormatSelectorProps) -> Html {
    let on_change = {
        let on_format_change = props.on_format_change.clone();
        Callback::from(move |e: Event| {
            let select: HtmlSelectElement = e.target_unchecked_into();
            on_format_change.emit(select.value());
        })
    };

    html! {
        <label style="display: flex; align-items: center; gap: 5px;">
            {"Output Format:"}
            <select
                onchange={on_change}
                style="margin-left: 5px; padding: 4px 8px; border: 1px solid #ccc; border-radius: 3px;"
            >
                <option value="jpeg" selected={props.selected_format == "jpeg"}>{"JPEG (smaller file)"}</option>
                <option value="png" selected={props.selected_format == "png"}>{"PNG (lossless)"}</option>
            </select>
        </label>
    }
}

#[derive(Properties, PartialEq)]
struct QualitySliderProps {
    pub quality: f64,
    pub on_quality_change: Callback<f64>,
}

#[function_component(QualitySlider)]
fn quality_slider(props: &QualitySliderProps) -> Html {
    let on_input = {
        let on_quality_change = props.on_quality_change.clone();
        Callback::from(move |e: web_sys::InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            if let Ok(val) = input.value().parse::<f64>() {
                on_quality_change.emit(val);
            }
        })
    };

    html! {
        <label style="display: flex; align-items: center; gap: 5px;">
            {"Quality:"}
            <input
                type="range"
                min="0.3"
                max="1.0"
                step="0.1"
                value={props.quality.to_string()}
                oninput={on_input}
                style="margin: 0 5px;"
            />
            <span style="font-size: 12px; color: #666;">
                {format!("{}%", (props.quality * 100.0) as u8)}
            </span>
        </label>
    }
}
