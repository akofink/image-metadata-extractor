use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement};

pub async fn create_cleaned_image(
    image_data_url: &str,
    filename: &str,
    quality: f64,
    format: &str,
) -> Result<(String, String), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    // Create an image element
    let img: HtmlImageElement = document
        .create_element("img")?
        .dyn_into::<HtmlImageElement>()?;

    // Create a promise to wait for image load
    let promise = js_sys::Promise::new(&mut |resolve, _reject| {
        let img_clone = img.clone();
        let resolve_clone = resolve.clone();
        
        let onload = Closure::wrap(Box::new(move || {
            resolve_clone.call0(&JsValue::NULL).unwrap();
        }) as Box<dyn FnMut()>);
        
        img_clone.set_onload(Some(onload.as_ref().unchecked_ref()));
        onload.forget(); // Keep the closure alive
    });

    // Set the image source and wait for it to load
    img.set_src(image_data_url);
    wasm_bindgen_futures::JsFuture::from(promise).await?;

    // Get image dimensions
    let width = img.natural_width();
    let height = img.natural_height();

    // Create a canvas
    let canvas: HtmlCanvasElement = document
        .create_element("canvas")?
        .dyn_into::<HtmlCanvasElement>()?;
    
    canvas.set_width(width);
    canvas.set_height(height);

    // Get 2D context
    let context: CanvasRenderingContext2d = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;

    // Draw the image to canvas (this strips all metadata)
    context.draw_image_with_html_image_element(&img, 0.0, 0.0)?;

    // Determine output format and extension based on user selection
    let (mime_type, extension) = match format {
        "png" => ("image/png", "png"),
        "jpeg" | _ => ("image/jpeg", "jpg"),
    };

    // Get the data URL for download
    let data_url = if mime_type == "image/jpeg" {
        canvas.to_data_url_with_type_and_encoder_options(mime_type, &JsValue::from_f64(quality))?
    } else {
        canvas.to_data_url_with_type(mime_type)?
    };

    // Create filename for cleaned image
    let base_name = filename
        .rfind('.')
        .map(|i| &filename[..i])
        .unwrap_or(filename);
    let cleaned_filename = format!("{}_cleaned.{}", base_name, extension);

    Ok((data_url, cleaned_filename))
}

pub fn download_cleaned_image(data_url: &str, filename: &str) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    // Create download link
    let link = document.create_element("a").unwrap();
    let link = link.dyn_into::<web_sys::HtmlAnchorElement>().unwrap();

    link.set_href(data_url);
    link.set_download(filename);
    link.style().set_property("display", "none").unwrap();

    // Trigger download
    document.body().unwrap().append_child(&link).unwrap();
    link.click();
    document.body().unwrap().remove_child(&link).unwrap();
}