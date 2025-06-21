use image_metadata_extractor::binary_cleaner::BinaryCleaner;

#[test]
fn remove_jpeg_app_segments_strips_app_data() {
    let mut data = vec![0xFF, 0xD8];
    data.extend_from_slice(&[
        0xFF, 0xE1, 0x00, 0x0A, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88,
    ]);
    data.extend_from_slice(&[
        0xFF, 0xDA, 0x00, 0x08, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0xFF, 0xD9,
    ]);
    let cleaned = BinaryCleaner::clean_metadata(&data, "jpg").unwrap();
    assert_eq!(&cleaned[0..2], &[0xFF, 0xD8]);
    assert_eq!(&cleaned[2..4], &[0xFF, 0xDA]);
    assert!(cleaned.len() < data.len());
}

#[test]
fn clean_png_metadata_drops_text_chunks() {
    let mut png = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
    png.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, b'I', b'H', b'D', b'R', 0, 0, 0, 0]);
    png.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, b't', b'E', b'X', b't', 0, 0, 0, 0]);
    png.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, b'I', b'E', b'N', b'D', 0, 0, 0, 0]);
    let cleaned = BinaryCleaner::clean_metadata(&png, "png").unwrap();
    assert!(cleaned.windows(4).any(|w| w == b"IHDR"));
    assert!(cleaned.windows(4).any(|w| w == b"IEND"));
    assert!(!cleaned.windows(4).any(|w| w == b"tEXt"));
}

#[test]
fn clean_gif_metadata_removes_comment_extension() {
    let mut gif = b"GIF89a".to_vec();
    gif.extend_from_slice(&[0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00]);
    gif.extend_from_slice(&[0x21, 0xFE, 0x03, b'A', b'B', b'C', 0x00]);
    gif.push(0x3B);
    let cleaned = BinaryCleaner::clean_metadata(&gif, "gif").unwrap();
    assert!(!cleaned.windows(2).any(|w| w == [0x21, 0xFE]));
    assert_eq!(cleaned.last(), Some(&0x3B));
}
