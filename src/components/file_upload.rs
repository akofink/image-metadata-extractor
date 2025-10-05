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
    /// Optional callback to emit when multiple files are processed sequentially.
    #[prop_or_default]
    pub on_files_loaded: Option<Callback<Vec<ImageData>>>,
    /// Optional progress callback: (processed_count, total_count).
    #[prop_or_default]
    pub on_progress: Option<Callback<(usize, usize)>>,
}

/// File input element that parses the selected file and emits [`ImageData`].
#[function_component(FileUpload)]
pub fn file_upload(props: &FileUploadProps) -> Html {
    let input_ref = use_node_ref();

    let on_file_change = {
        let on_file_loaded = props.on_file_loaded.clone();
        let on_error = props.on_error.clone();
        let on_files_loaded = props.on_files_loaded.clone();
        let on_progress = props.on_progress.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if let Some(file_list) = input.files() {
                let count = file_list.length() as usize;
                let on_file_loaded = on_file_loaded.clone();
                let on_error = on_error.clone();
                let on_files_loaded = on_files_loaded.clone();
                let on_progress = on_progress.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    if let Some(cb) = &on_progress {
                        cb.emit((0, count));
                    }
                    let mut acc = Vec::with_capacity(count);
                    for idx in 0..count {
                        if let Some(file) = file_list.get(idx as u32) {
                            match process_file(file).await {
                                Ok(data) => {
                                    // Emit first file immediately for legacy UX
                                    if idx == 0 {
                                        on_file_loaded.emit(data.clone());
                                    }
                                    acc.push(data);
                                }
                                Err(e) => {
                                    let msg = e
                                        .as_string()
                                        .unwrap_or_else(|| "Unknown error".to_string());
                                    on_error.emit(msg);
                                }
                            }
                        }
                        if let Some(cb) = &on_progress {
                            cb.emit((idx + 1, count));
                        }
                    }
                    if let Some(cb) = on_files_loaded {
                        cb.emit(acc);
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
            multiple=true
            accept="image/*,application/pdf,image/svg+xml,image/tiff,image/heif,image/avif,image/jxl,.tiff,.tif,.heif,.heic,.avif,.jxl,.pdf,.svg"
            onchange={on_file_change}
            style="display: none;"
            data-testid="file-input"
        />
    }
}
