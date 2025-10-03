//! Component for importing and processing ZIP archives of images.

use crate::archive::extract_images_from_zip;
use crate::exif::process_blob;
use crate::types::ImageData;
use web_sys::{Event, HtmlInputElement};
use yew::prelude::*;

/// Properties for [`ArchiveImport`].
#[derive(Properties, PartialEq, Debug)]
pub struct ArchiveImportProps {
    /// Callback when archive processing is complete with all extracted images.
    pub on_files_loaded: Callback<Vec<ImageData>>,
    /// Callback for handling errors.
    pub on_error: Callback<String>,
    /// Optional progress callback: (processed_count, total_count).
    #[prop_or_default]
    pub on_progress: Option<Callback<(usize, usize)>>,
    /// Trigger callback to allow external components to open the file dialog.
    pub trigger_archive_input: Callback<Callback<()>>,
}

/// ZIP archive input component that extracts and processes all image files.
#[function_component(ArchiveImport)]
pub fn archive_import(props: &ArchiveImportProps) -> Html {
    let input_ref = use_node_ref();

    let on_archive_change = {
        let on_files_loaded = props.on_files_loaded.clone();
        let on_error = props.on_error.clone();
        let on_progress = props.on_progress.clone();

        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if let Some(file_list) = input.files()
                && let Some(zip_file) = file_list.get(0)
            {
                let on_files_loaded = on_files_loaded.clone();
                let on_error = on_error.clone();
                let on_progress = on_progress.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    // Extract images from the ZIP
                    match extract_images_from_zip(zip_file).await {
                        Ok(archived_files) => {
                            let total = archived_files.len();

                            if total == 0 {
                                on_error.emit("No image files found in the archive.".to_string());
                                return;
                            }

                            if let Some(cb) = &on_progress {
                                cb.emit((0, total));
                            }

                            let mut results = Vec::with_capacity(total);

                            // Process each extracted image
                            for (idx, archived_file) in archived_files.into_iter().enumerate() {
                                match process_blob(
                                    archived_file.blob.clone(),
                                    archived_file.name.clone(),
                                )
                                .await
                                {
                                    Ok(data) => {
                                        results.push(data);
                                    }
                                    Err(e) => {
                                        let msg = e.as_string().unwrap_or_else(|| {
                                            format!(
                                                "Failed to process {}: Unknown error",
                                                archived_file.name
                                            )
                                        });
                                        on_error.emit(msg);
                                    }
                                }

                                if let Some(cb) = &on_progress {
                                    cb.emit((idx + 1, total));
                                }
                            }

                            on_files_loaded.emit(results);
                        }
                        Err(e) => {
                            let msg = e
                                .as_string()
                                .unwrap_or_else(|| "Failed to extract ZIP archive".to_string());
                            on_error.emit(msg);
                        }
                    }
                });
            }
        })
    };

    // Set up trigger callback for external clicking
    {
        let input_ref = input_ref.clone();
        let trigger_archive_input = props.trigger_archive_input.clone();
        use_effect_with((), move |_| {
            let trigger_callback = Callback::from(move |_| {
                if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                    input.click();
                }
            });
            trigger_archive_input.emit(trigger_callback);
            || ()
        });
    }

    html! {
        <input
            ref={input_ref}
            type="file"
            accept=".zip,application/zip,application/x-zip-compressed"
            onchange={on_archive_change}
            style="display: none;"
        />
    }
}
