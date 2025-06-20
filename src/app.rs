use crate::components::{
    file_upload::FileUpload, image_cleaner::ImageCleaner, image_display::ImageDisplay,
    metadata_display::MetadataDisplay, metadata_export::MetadataExport,
};
use crate::types::ImageData;
use std::collections::HashSet;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let image_data = use_state(|| None::<ImageData>);
    let is_expanded = use_state(|| false);
    let selected_metadata = use_state(HashSet::<String>::new);
    let show_explanations = use_state(|| false);

    let on_file_loaded = {
        let image_data = image_data.clone();
        let is_expanded = is_expanded.clone();
        let selected_metadata = selected_metadata.clone();

        Callback::from(move |data: ImageData| {
            // Auto-select all metadata by default
            let all_keys: HashSet<String> = data.exif_data.keys().cloned().collect();
            selected_metadata.set(all_keys);
            image_data.set(Some(data));
            is_expanded.set(false); // Reset to thumbnail view
        })
    };

    let on_image_click = {
        let is_expanded = is_expanded.clone();
        Callback::from(move |_: web_sys::MouseEvent| {
            is_expanded.set(!*is_expanded);
        })
    };

    let on_metadata_selection_change = {
        let selected_metadata = selected_metadata.clone();
        Callback::from(move |new_selection: HashSet<String>| {
            selected_metadata.set(new_selection);
        })
    };

    let on_toggle_explanations = {
        let show_explanations = show_explanations.clone();
        Callback::from(move |_: web_sys::MouseEvent| {
            show_explanations.set(!*show_explanations);
        })
    };

    html! {
        <div style="min-height: 100vh; display: flex; flex-direction: column;">
            <div style="max-width: 800px; margin: 0 auto; padding: 16px; flex: 1;">
                <h1>{"Image Metadata Extractor"}</h1>

                <FileUpload on_file_loaded={on_file_loaded} />

                // Main content area with consistent layout
                <div style="margin-top: 20px;">
                    {
                        if let Some(ref data) = *image_data {
                            html! {
                                <div style="opacity: 1; transition: opacity 0.3s ease-in-out;">
                                    <ImageDisplay
                                        image_data={data.clone()}
                                        is_expanded={*is_expanded}
                                        on_image_click={on_image_click}
                                    />

                                    <MetadataDisplay
                                        image_data={data.clone()}
                                        selected_metadata={(*selected_metadata).clone()}
                                        show_explanations={*show_explanations}
                                        on_metadata_selection_change={on_metadata_selection_change}
                                        on_toggle_explanations={on_toggle_explanations}
                                    />

                                    <ImageCleaner image_data={data.clone()} />

                                    <MetadataExport
                                        image_data={data.clone()}
                                        selected_metadata={(*selected_metadata).clone()}
                                    />
                                </div>
                            }
                        } else {
                            html! {
                                <div style="text-align: center; padding: 40px 20px; color: #666; background: #f8f9fa; border-radius: 8px; border: 2px dashed #dee2e6;">
                                    <div style="font-size: 48px; margin-bottom: 16px;">{"📷"}</div>
                                    <p style="font-size: 18px; margin-bottom: 8px;">{"Select an image to get started"}</p>
                                    <p style="font-size: 14px; margin: 0;">{"Upload JPEG, PNG, GIF, or WebP files to extract metadata"}</p>
                                </div>
                            }
                        }
                    }
                </div>
            </div>

            <footer style="margin-top: auto; padding: 20px 0; border-top: 1px solid #ddd; text-align: center; color: #666; font-size: 14px; background-color: #f8f9fa;">
                <p>
                    {"Built with ❤️ using Rust + WebAssembly • "}
                    <a href="https://github.com/akofink/image-metadata-extractor" target="_blank" style="color: #007bff; text-decoration: none;">
                        {"Open Source"}
                    </a>
                    {" • Privacy-First (No Server Uploads)"}
                </p>
                <p style="margin-top: 8px; font-size: 12px;">
                    {"© 2024 Image Metadata Extractor • "}
                    <a href="mailto:contact@image-metadata-extractor.com" style="color: #007bff; text-decoration: none;">
                        {"Contact"}
                    </a>
                </p>
            </footer>
        </div>
    }
}
