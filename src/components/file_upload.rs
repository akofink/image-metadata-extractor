use crate::exif::process_file;
use crate::types::ImageData;
use web_sys::{Event, HtmlInputElement};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct FileUploadProps {
    pub on_file_loaded: Callback<ImageData>,
}

#[function_component(FileUpload)]
pub fn file_upload(props: &FileUploadProps) -> Html {
    let on_file_change = {
        let on_file_loaded = props.on_file_loaded.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if let Some(files) = input.files() {
                if let Some(file) = files.get(0) {
                    let on_file_loaded = on_file_loaded.clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        if let Ok(data) = process_file(file).await {
                            on_file_loaded.emit(data);
                        }
                    });
                }
            }
        })
    };

    html! {
        <div style="margin: 20px 0;">
            <input
                type="file"
                accept="image/*"
                onchange={on_file_change}
                style="margin-bottom: 20px;"
            />
        </div>
    }
}
