use crate::exif::process_file;
use crate::types::ImageData;
use web_sys::{Event, HtmlInputElement};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct FileUploadProps {
    pub on_file_loaded: Callback<ImageData>,
    pub trigger_file_input: Callback<Callback<()>>,
}

#[function_component(FileUpload)]
pub fn file_upload(props: &FileUploadProps) -> Html {
    let input_ref = use_node_ref();

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

    // Set up trigger callback for external clicking
    {
        let input_ref = input_ref.clone();
        let trigger_callback = Callback::from(move |_| {
            if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                input.click();
            }
        });
        props.trigger_file_input.emit(trigger_callback);
    }

    html! {
        <div style="margin: 20px 0;">
            <input
                ref={input_ref}
                type="file"
                accept="image/*"
                onchange={on_file_change}
                style="margin-bottom: 20px;"
            />
        </div>
    }
}
