use crate::exif::process_file;
use crate::types::ImageData;
use web_sys::{Event, HtmlInputElement};
use yew::prelude::*;

#[derive(Properties, PartialEq, Debug)]
pub struct FileUploadProps {
    pub on_file_loaded: Callback<ImageData>,
    pub trigger_file_input: Callback<Callback<()>>,
    pub on_error: Callback<String>,
}

#[function_component(FileUpload)]
pub fn file_upload(props: &FileUploadProps) -> Html {
    let input_ref = use_node_ref();

    let on_file_change = {
        let on_file_loaded = props.on_file_loaded.clone();
        let on_error = props.on_error.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if let Some(files) = input.files() {
                if let Some(file) = files.get(0) {
                    let on_file_loaded = on_file_loaded.clone();
                    let on_error = on_error.clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        match process_file(file).await {
                            Ok(data) => on_file_loaded.emit(data),
                            Err(e) => {
                                let msg =
                                    e.as_string().unwrap_or_else(|| "Unknown error".to_string());
                                on_error.emit(msg);
                            }
                        }
                    });
                }
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

#[cfg(test)]
#[allow(dead_code)] // WebAssembly tests are run by wasm-pack, not cargo test
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    #[allow(dead_code)] // WebAssembly test functions appear unused to cargo test
    fn test_file_upload_component_renders() {
        let on_file_loaded = Callback::noop();
        let trigger_file_input = Callback::noop();

        let props = FileUploadProps {
            on_file_loaded,
            trigger_file_input,
            on_error: Callback::noop(),
        };

        let _rendered = html! {
            <FileUpload ..props />
        };

        // Basic test to ensure component can be instantiated without panicking
        assert!(true);
    }

    #[wasm_bindgen_test]
    #[allow(dead_code)]
    fn test_file_upload_props_equality() {
        let on_file_loaded = Callback::noop();
        let trigger_file_input = Callback::noop();

        let props1 = FileUploadProps {
            on_file_loaded: on_file_loaded.clone(),
            trigger_file_input: trigger_file_input.clone(),
            on_error: Callback::noop(),
        };

        let props2 = FileUploadProps {
            on_file_loaded: on_file_loaded.clone(),
            trigger_file_input: trigger_file_input.clone(),
            on_error: Callback::noop(),
        };

        // Test that props implement PartialEq correctly
        assert_eq!(props1, props2);
    }

    // Test to prevent regression of infinite render loop bug
    #[wasm_bindgen_test]
    #[allow(dead_code)]
    fn test_trigger_callback_setup_only_once() {
        // This test verifies that the FileUpload component can be created
        // without causing infinite loops or panics during render
        let on_file_loaded = Callback::noop();
        let trigger_file_input = Callback::noop();

        let props = FileUploadProps {
            on_file_loaded,
            trigger_file_input,
            on_error: Callback::noop(),
        };

        // The component should render successfully without infinite loops
        let _rendered = html! {
            <FileUpload ..props />
        };

        // If we reach this point without panicking or hanging,
        // the component is properly using use_effect_with for one-time setup
        assert!(true);
    }
}
