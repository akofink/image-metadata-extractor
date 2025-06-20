pub fn output_format(format: &str) -> (&'static str, &'static str) {
    match format.to_lowercase().as_str() {
        "webp" => ("image/webp", "webp"),
        "gif" => ("image/gif", "gif"),
        "png" => ("image/png", "png"),
        _ => ("image/jpeg", "jpg"),
    }
}
