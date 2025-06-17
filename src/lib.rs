use yew::prelude::*;
use wasm_bindgen::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            <h1>{"Image Metadata Extractor"}</h1>
            <p>{"Upload an image to extract its metadata"}</p>
        </div>
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<App>::new().render();
}
