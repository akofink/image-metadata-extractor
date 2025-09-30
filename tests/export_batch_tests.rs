use image_metadata_extractor::export::{
    generate_csv_batch, generate_json_batch, generate_txt_batch,
};
use image_metadata_extractor::types::ImageData;
use std::collections::HashMap;

fn sample(
    name: &str,
    size: u64,
    w: Option<u32>,
    h: Option<u32>,
    gps: Option<(f64, f64)>,
    exif: &[(&str, &str)],
) -> ImageData {
    let mut map = HashMap::new();
    for (k, v) in exif {
        map.insert((*k).to_string(), (*v).to_string());
    }
    ImageData {
        name: name.to_string(),
        size,
        mime_type: "image/jpeg".to_string(),
        data_url: String::new(),
        width: w,
        height: h,
        exif_data: map,
        gps_coords: gps,
        sha256_hash: None,
    }
}

#[test]
fn test_generate_json_batch_array() {
    let items = vec![
        sample(
            "a.jpg",
            1000,
            Some(10),
            Some(20),
            None,
            &[("Make", "Canon")],
        ),
        sample(
            "b.jpg",
            2000,
            None,
            None,
            Some((1.1, 2.2)),
            &[("Model", "X")],
        ),
    ];
    let json = generate_json_batch(&items);
    assert!(json.starts_with("["));
    assert!(json.contains("\"name\": \"a.jpg\""));
    assert!(json.contains("\"name\": \"b.jpg\""));
}

#[test]
fn test_generate_csv_batch_header_and_rows() {
    let items = vec![
        sample(
            "one.jpg",
            1024,
            Some(100),
            Some(200),
            Some((10.0, 20.0)),
            &[("ISO", "100"), ("Make", "A")],
        ),
        sample(
            "two.jpg",
            2048,
            None,
            None,
            None,
            &[("Model", "B"), ("ISO", "200")],
        ),
    ];
    let csv = generate_csv_batch(&items);
    // Header must include union of EXIF keys in sorted order
    assert!(csv.lines().next().unwrap().contains("Filename"));
    assert!(csv.contains("\"ISO\""));
    assert!(csv.contains("\"Make\""));
    assert!(csv.contains("\"Model\""));
    // Two data rows
    let rows: Vec<&str> = csv.lines().collect();
    assert_eq!(rows.len(), 1 + items.len());
}

#[test]
fn test_generate_txt_batch_contains_each_report() {
    let items = vec![
        sample("x.png", 500, Some(1), Some(2), None, &[]),
        sample("y.png", 1500, None, None, Some((0.0, 0.0)), &[]),
    ];
    let txt = generate_txt_batch(&items);
    assert!(txt.contains("BATCH FILE METADATA REPORT"));
    assert!(txt.contains("# x.png"));
    assert!(txt.contains("# y.png"));
}
