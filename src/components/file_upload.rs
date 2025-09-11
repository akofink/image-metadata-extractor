//! Component for uploading files and initiating metadata extraction.

use crate::exif::process_file;
use crate::types::ImageData;
use web_sys::{Event, HtmlInputElement};
use yew::prelude::*;

/// Properties for [`FileUpload`].
#[derive(Properties, PartialEq, Debug)]
pub struct FileUploadProps {
    pub on_file_loaded: Callback<ImageData>,
    pub trigger_file_input: Callback<Callback<()>>,
    pub on_error: Callback<String>,
}

/// File input element that parses the selected file and emits [`ImageData`].
#[function_component(FileUpload)]
pub fn file_upload(props: &FileUploadProps) -> Html {
    let input_ref = use_node_ref();

    let on_file_change = {
        let on_file_loaded = props.on_file_loaded.clone();
        let on_error = props.on_error.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if let Some(files) = input.files()
                && let Some(file) = files.get(0)
            {
                let on_file_loaded = on_file_loaded.clone();
                let on_error = on_error.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    match process_file(file).await {
                        Ok(data) => on_file_loaded.emit(data),
                        Err(e) => {
                            let msg = e.as_string().unwrap_or_else(|| "Unknown error".to_string());
                            on_error.emit(msg);
                        }
                    }
                });
            }
        })
    };

    // Set up trigger callback for external clicking (only once)
    {
        let input_ref = input_ref.clone();
        let trigger_file_input = props.trigger_file_input.clone();
        use_effect_with((), move |_| {
            let trigger_callback = Callback::from(move |_| {
                if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                    input.click();
                }
            });
            trigger_file_input.emit(trigger_callback);
            || ()
        });
    }

    html! {
        <input
            ref={input_ref}
            type="file"
            accept="image/*,application/pdf,image/svg+xml,image/tiff,image/heif,image/avif,image/jxl,.tiff,.tif,.heif,.heic,.avif,.jxl,.pdf,.svg"
            onchange={on_file_change}
            style="display: none;"
        />
    }
}
