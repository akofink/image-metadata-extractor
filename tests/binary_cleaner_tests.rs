use image_metadata_extractor::binary_cleaner::BinaryCleaner;

// JPEG Tests
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
fn clean_jpeg_multiple_app_segments() {
    let mut data = vec![0xFF, 0xD8]; // SOI
    // APP0 segment
    data.extend_from_slice(&[0xFF, 0xE0, 0x00, 0x10]);
    data.extend_from_slice(b"JFIF\0\x01\x01\x01\0\x48\0\x48\0\0");
    // APP1 segment (EXIF)
    data.extend_from_slice(&[0xFF, 0xE1, 0x00, 0x06]);
    data.extend_from_slice(b"EXIF");
    // Start of scan
    data.extend_from_slice(&[0xFF, 0xDA, 0x00, 0x04, 0x01, 0x02]);

    let cleaned = BinaryCleaner::clean_metadata(&data, "jpeg").unwrap();
    assert_eq!(&cleaned[0..2], &[0xFF, 0xD8]); // SOI preserved
    assert_eq!(&cleaned[2..4], &[0xFF, 0xDA]); // SOS preserved
    assert!(cleaned.len() < data.len()); // APP segments removed
}

#[test]
fn clean_jpeg_invalid_soi_marker() {
    // Missing SOI marker - our validation catches this
    let result = BinaryCleaner::clean_metadata(&[0x00, 0x01, 0x02, 0x03], "jpg");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("missing SOI marker"));
}

#[test]
fn clean_jpeg_with_proper_segments() {
    // Test a properly formed JPEG with segments that can be cleanly removed
    let mut data = vec![0xFF, 0xD8]; // SOI
    // Add a properly formed APP0 segment
    data.extend_from_slice(&[0xFF, 0xE0, 0x00, 0x10]); // APP0, length 16
    data.extend_from_slice(&[0x4A, 0x46, 0x49, 0x46, 0x00]); // "JFIF\0"
    data.extend_from_slice(&[
        0x01, 0x01, 0x01, 0x00, 0x48, 0x00, 0x48, 0x00, 0x00, 0x00, 0x00,
    ]); // JFIF data
    // Add SOS to end parsing
    data.extend_from_slice(&[0xFF, 0xDA, 0x00, 0x02, 0x01, 0x02]); // Start of scan

    let result = BinaryCleaner::clean_metadata(&data, "jpg");
    assert!(result.is_ok());
    let cleaned = result.unwrap();
    assert!(cleaned.len() < data.len()); // APP0 should be removed
}

// PNG Tests
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
fn clean_png_metadata_removes_all_metadata_chunks() {
    let mut png = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
    // IHDR (critical)
    png.extend_from_slice(&[0x00, 0x00, 0x00, 0x0D, b'I', b'H', b'D', b'R']);
    png.extend_from_slice(&[
        0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x02, 0x00, 0x00, 0x00,
    ]);
    png.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // CRC

    // Add metadata chunks that should be removed
    let metadata_chunks = [
        (b"tIME", 7u32),
        (b"pHYs", 9u32),
        (b"gAMA", 4u32),
        (b"cHRM", 32u32),
        (b"sRGB", 1u32),
        (b"iCCP", 10u32),
        (b"zTXt", 8u32),
        (b"iTXt", 12u32),
    ];

    for (chunk_name, size) in metadata_chunks.iter() {
        png.extend_from_slice(&size.to_be_bytes());
        png.extend_from_slice(*chunk_name);
        png.extend_from_slice(&vec![0u8; *size as usize]);
        png.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // CRC
    }

    // IEND (critical)
    png.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, b'I', b'E', b'N', b'D', 0, 0, 0, 0]);

    let cleaned = BinaryCleaner::clean_metadata(&png, "png").unwrap();
    assert!(cleaned.windows(4).any(|w| w == b"IHDR"));
    assert!(cleaned.windows(4).any(|w| w == b"IEND"));

    // Verify metadata chunks are removed
    for (chunk_name, _) in metadata_chunks.iter() {
        assert!(
            !cleaned.windows(4).any(|w| w == *chunk_name),
            "Chunk {:?} should be removed",
            std::str::from_utf8(*chunk_name)
        );
    }
}

#[test]
fn clean_png_invalid_files() {
    // Too short
    let result = BinaryCleaner::clean_metadata(&[0x89, 0x50], "png");
    assert!(result.is_err());

    // Invalid signature
    let bad_png = vec![0x00, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
    let result = BinaryCleaner::clean_metadata(&bad_png, "png");
    assert!(result.is_err());
}

// WebP Tests
#[test]
fn clean_webp_metadata_removes_exif_xmp() {
    let mut webp = b"RIFF".to_vec();
    webp.extend_from_slice(&[0x20, 0x00, 0x00, 0x00]); // File size (little-endian)
    webp.extend_from_slice(b"WEBP");

    // VP8 chunk (image data)
    webp.extend_from_slice(b"VP8 ");
    webp.extend_from_slice(&[0x08, 0x00, 0x00, 0x00]); // Chunk size
    webp.extend_from_slice(&[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]); // Data

    // EXIF chunk (metadata)
    webp.extend_from_slice(b"EXIF");
    webp.extend_from_slice(&[0x04, 0x00, 0x00, 0x00]); // Chunk size
    webp.extend_from_slice(&[0xAA, 0xBB, 0xCC, 0xDD]); // Metadata

    let cleaned = BinaryCleaner::clean_metadata(&webp, "webp").unwrap();
    assert!(cleaned.windows(4).any(|w| w == b"VP8 "));
    assert!(!cleaned.windows(4).any(|w| w == b"EXIF"));
}

#[test]
fn clean_webp_invalid_files() {
    // Too short
    let result = BinaryCleaner::clean_metadata(&[b'R', b'I'], "webp");
    assert!(result.is_err());

    // Invalid RIFF header
    let bad_webp = b"JUNK1234WEBP".to_vec();
    let result = BinaryCleaner::clean_metadata(&bad_webp, "webp");
    assert!(result.is_err());

    // Missing WEBP signature
    let bad_webp = b"RIFF\x10\x00\x00\x00JUNK".to_vec();
    let result = BinaryCleaner::clean_metadata(&bad_webp, "webp");
    assert!(result.is_err());
}

// GIF Tests
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

#[test]
fn clean_gif_metadata_removes_application_extension() {
    let mut gif = b"GIF89a".to_vec();
    gif.extend_from_slice(&[0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00]); // Screen descriptor
    // Application extension with XMP data
    gif.extend_from_slice(&[0x21, 0xFF, 0x0B]); // Application extension + block size
    gif.extend_from_slice(b"XMP DataXMP"); // Application identifier
    gif.extend_from_slice(&[0x05, b'h', b'e', b'l', b'l', b'o']); // Sub-block
    gif.extend_from_slice(&[0x00]); // Block terminator
    gif.push(0x3B); // Trailer

    let cleaned = BinaryCleaner::clean_metadata(&gif, "gif").unwrap();
    assert!(!cleaned.windows(2).any(|w| w == [0x21, 0xFF]));
    assert_eq!(cleaned.last(), Some(&0x3B));
}

#[test]
fn clean_gif_invalid_files() {
    // Too short
    let result = BinaryCleaner::clean_metadata(&[b'G', b'I'], "gif");
    assert!(result.is_err());

    // Invalid signature
    let bad_gif = b"NOTGIF".to_vec();
    let result = BinaryCleaner::clean_metadata(&bad_gif, "gif");
    assert!(result.is_err());

    // Invalid version
    let bad_gif = b"GIF99a".to_vec();
    let result = BinaryCleaner::clean_metadata(&bad_gif, "gif");
    assert!(result.is_err());
}

// TIFF Tests
#[test]
fn clean_tiff_metadata_basic() {
    // Create minimal TIFF header
    let mut tiff = vec![0x49, 0x49, 0x2A, 0x00]; // Little-endian TIFF header
    tiff.extend_from_slice(&[0x08, 0x00, 0x00, 0x00]); // IFD offset

    let cleaned = BinaryCleaner::clean_metadata(&tiff, "tiff").unwrap();
    // Should return the data (little_exif clears metadata)
    assert_eq!(cleaned.len(), tiff.len());
}

#[test]
fn clean_tiff_alternative_extension() {
    let tiff = vec![0x4D, 0x4D, 0x00, 0x2A, 0x00, 0x00, 0x00, 0x08]; // Big-endian TIFF
    let cleaned = BinaryCleaner::clean_metadata(&tiff, "tif").unwrap();
    assert_eq!(cleaned.len(), tiff.len());
}

// HEIF Tests
#[test]
fn clean_heif_metadata_basic() {
    // Create minimal HEIF data
    let heif = vec![0x00, 0x00, 0x00, 0x20, b'f', b't', b'y', b'p']; // HEIF box header
    let cleaned = BinaryCleaner::clean_metadata(&heif, "heif").unwrap();
    assert_eq!(cleaned.len(), heif.len());
}

#[test]
fn clean_heic_metadata_basic() {
    let heic = vec![0x00, 0x00, 0x00, 0x20, b'f', b't', b'y', b'p'];
    let cleaned = BinaryCleaner::clean_metadata(&heic, "heic").unwrap();
    assert_eq!(cleaned.len(), heic.len());
}

// SVG Tests
#[test]
fn clean_svg_metadata_removes_metadata_elements() {
    let svg = b"<svg>\n<metadata>secret</metadata>\n<rect width='1' height='1'/>\n</svg>";
    let cleaned = BinaryCleaner::clean_metadata(svg, "svg").unwrap();
    let cleaned_str = String::from_utf8(cleaned).unwrap();
    assert!(!cleaned_str.contains("<metadata"));
    assert!(cleaned_str.contains("<rect"));
}

#[test]
fn clean_svg_metadata_removes_rdf_elements() {
    let svg = b"<svg xmlns:rdf=\"test\">\n<rdf:Description>data</rdf:Description>\n<circle r='5'/>\n</svg>";
    let cleaned = BinaryCleaner::clean_metadata(svg, "svg").unwrap();
    let cleaned_str = String::from_utf8(cleaned).unwrap();
    assert!(!cleaned_str.contains("xmlns:rdf"));
    assert!(!cleaned_str.contains("<rdf:"));
    assert!(cleaned_str.contains("<circle"));
}

#[test]
fn clean_svg_metadata_invalid_input() {
    let result = BinaryCleaner::clean_metadata(b"not svg", "svg");
    assert!(result.is_err());
}

// PDF Tests
#[test]
fn clean_pdf_metadata_basic_validation() {
    let pdf = b"%PDF-1.4\n%1234";
    let cleaned = BinaryCleaner::clean_metadata(pdf, "pdf").unwrap();
    assert_eq!(cleaned, pdf);
    let bad = BinaryCleaner::clean_metadata(b"not a pdf", "pdf");
    assert!(bad.is_err());
}

// Unimplemented format tests
#[test]
fn clean_avif_not_implemented() {
    let result = BinaryCleaner::clean_metadata(&[0x01, 0x02], "avif");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("not fully implemented"));
}

#[test]
fn clean_jxl_not_implemented() {
    let result = BinaryCleaner::clean_metadata(&[0x01, 0x02], "jxl");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("not fully implemented"));
}

// Unsupported format test
#[test]
fn clean_unsupported_format() {
    let result = BinaryCleaner::clean_metadata(&[0x01, 0x02], "xyz");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Unsupported format"));
}

// Test case sensitivity
#[test]
fn clean_metadata_case_insensitive() {
    let jpeg = vec![0xFF, 0xD8, 0xFF, 0xDA, 0x00, 0x04, 0x01, 0x02];
    let cleaned_upper = BinaryCleaner::clean_metadata(&jpeg, "JPEG").unwrap();
    let cleaned_lower = BinaryCleaner::clean_metadata(&jpeg, "jpeg").unwrap();
    assert_eq!(cleaned_upper, cleaned_lower);
}

// JPEG Edge Cases - High Priority Coverage Improvements
#[test]
fn clean_jpeg_truncated_at_segment_boundary() {
    // Test JPEG truncated during segment length reading
    let mut data = vec![0xFF, 0xD8]; // SOI
    data.extend_from_slice(&[0xFF, 0xE1, 0x00]); // APP1 with incomplete length

    let result = BinaryCleaner::clean_metadata(&data, "jpg");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Truncated JPEG file"));
}

#[test]
fn clean_jpeg_segment_length_extends_beyond_file() {
    let mut data = vec![0xFF, 0xD8]; // SOI
    data.extend_from_slice(&[0xFF, 0xE1, 0x00, 0x20]); // APP1 with length 32
    data.extend_from_slice(&[0x01, 0x02, 0x03]); // Only 3 bytes of data (need 30 more)

    let result = BinaryCleaner::clean_metadata(&data, "jpg");
    // The cleaner gracefully handles incomplete segments by preserving what it can
    assert!(result.is_ok());
    let cleaned = result.unwrap();
    // Should preserve SOI marker and handle truncated segment gracefully
    assert_eq!(cleaned, vec![0xFF, 0xD8]);
    assert!(cleaned.len() < data.len());
}

#[test]
fn clean_jpeg_segment_with_maximum_length() {
    let mut data = vec![0xFF, 0xD8]; // SOI
    data.extend_from_slice(&[0xFF, 0xE1, 0xFF, 0xFF]); // APP1 with max length 65535
    // Don't add the full data - test boundary condition

    let result = BinaryCleaner::clean_metadata(&data, "jpg");
    // The cleaner gracefully handles incomplete segments by preserving what it can
    assert!(result.is_ok());
    let cleaned = result.unwrap();
    // Should preserve SOI marker and handle truncated segment gracefully
    assert_eq!(cleaned, vec![0xFF, 0xD8]);
    assert!(cleaned.len() < data.len());
}

#[test]
fn clean_jpeg_zero_length_segment() {
    let mut data = vec![0xFF, 0xD8]; // SOI
    data.extend_from_slice(&[0xFF, 0xE1, 0x00, 0x02]); // APP1 with length 2 (just length field)
    data.extend_from_slice(&[0xFF, 0xDA, 0x00, 0x04, 0x01, 0x02]); // SOS

    let result = BinaryCleaner::clean_metadata(&data, "jpg");
    assert!(result.is_ok());
    let cleaned = result.unwrap();
    assert!(cleaned.len() < data.len());
}

#[test]
fn clean_jpeg_multiple_consecutive_app_segments() {
    let mut data = vec![0xFF, 0xD8]; // SOI

    // Multiple APP segments back-to-back
    for app_num in 0..16 {
        data.extend_from_slice(&[0xFF, 0xE0 + app_num, 0x00, 0x04]); // APPx with length 4
        data.extend_from_slice(&[0x00, 0x00]); // 2 bytes of data
    }

    data.extend_from_slice(&[0xFF, 0xDA, 0x00, 0x04, 0x01, 0x02]); // SOS

    let result = BinaryCleaner::clean_metadata(&data, "jpg");
    assert!(result.is_ok());
    let cleaned = result.unwrap();
    // Should have removed all APP segments
    assert!(cleaned.len() < data.len());
    assert!(cleaned.windows(2).any(|w| w == [0xFF, 0xDA])); // SOS preserved
}

// PNG Edge Cases
#[test]
fn clean_png_chunk_with_zero_length() {
    let mut png = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]; // PNG signature

    // IHDR chunk
    png.extend_from_slice(&[0x00, 0x00, 0x00, 0x0D, b'I', b'H', b'D', b'R']);
    png.extend_from_slice(&[
        0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x02, 0x00, 0x00, 0x00,
    ]);
    png.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // CRC

    // Zero-length metadata chunk
    png.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, b't', b'E', b'X', b't']);
    png.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // CRC

    // IEND chunk
    png.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, b'I', b'E', b'N', b'D', 0, 0, 0, 0]);

    let result = BinaryCleaner::clean_metadata(&png, "png");
    assert!(result.is_ok());
    let cleaned = result.unwrap();
    assert!(!cleaned.windows(4).any(|w| w == b"tEXt"));
}

#[test]
fn clean_png_chunk_length_extends_beyond_file() {
    let mut png = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]; // PNG signature

    // IHDR chunk
    png.extend_from_slice(&[0x00, 0x00, 0x00, 0x0D, b'I', b'H', b'D', b'R']);
    png.extend_from_slice(&[
        0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x02, 0x00, 0x00, 0x00,
    ]);
    png.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // CRC

    // Chunk with length that extends beyond file
    png.extend_from_slice(&[0xFF, 0xFF, 0xFF, 0xFF, b't', b'E', b'X', b't']); // Huge length
    png.extend_from_slice(&[0x01, 0x02]); // Only 2 bytes of data

    let result = BinaryCleaner::clean_metadata(&png, "png");
    // PNG cleaner gracefully handles malformed chunks by breaking out of parsing
    assert!(result.is_ok());
    let cleaned = result.unwrap();
    // Should preserve PNG signature and IHDR but stop parsing at malformed chunk
    assert!(cleaned.len() < png.len());
}

#[test]
fn clean_png_truncated_at_chunk_header() {
    let mut png = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]; // PNG signature

    // IHDR chunk
    png.extend_from_slice(&[0x00, 0x00, 0x00, 0x0D, b'I', b'H', b'D', b'R']);
    png.extend_from_slice(&[
        0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x02, 0x00, 0x00, 0x00,
    ]);
    png.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // CRC

    // Truncated chunk (only length, no type)
    png.extend_from_slice(&[0x00, 0x00, 0x00, 0x10]); // Length but no chunk type

    let result = BinaryCleaner::clean_metadata(&png, "png");
    // PNG cleaner gracefully handles truncated chunks by breaking out of parsing
    assert!(result.is_ok());
    let cleaned = result.unwrap();
    // Should preserve PNG signature and IHDR
    assert!(cleaned.windows(4).any(|w| w == b"IHDR"));
}

// WebP Edge Cases
#[test]
fn clean_webp_file_size_update_verification() {
    let mut webp = b"RIFF".to_vec();
    webp.extend_from_slice(&[0x30, 0x00, 0x00, 0x00]); // Initial file size (48 bytes)
    webp.extend_from_slice(b"WEBP");

    // VP8 chunk (image data)
    webp.extend_from_slice(b"VP8 ");
    webp.extend_from_slice(&[0x08, 0x00, 0x00, 0x00]); // Chunk size
    webp.extend_from_slice(&[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]); // Data

    // EXIF chunk (metadata to be removed)
    webp.extend_from_slice(b"EXIF");
    webp.extend_from_slice(&[0x10, 0x00, 0x00, 0x00]); // Chunk size (16 bytes)
    webp.extend_from_slice(&[0x00; 16]); // 16 bytes of EXIF data

    let original_size = webp.len();
    let cleaned = BinaryCleaner::clean_metadata(&webp, "webp").unwrap();

    // Verify EXIF chunk was removed
    assert!(!cleaned.windows(4).any(|w| w == b"EXIF"));

    // Verify RIFF file size was updated correctly
    let new_file_size = u32::from_le_bytes([cleaned[4], cleaned[5], cleaned[6], cleaned[7]]);
    let expected_size = (cleaned.len() - 8) as u32; // File size excluding RIFF header
    assert_eq!(new_file_size, expected_size);

    // Verify file is smaller
    assert!(cleaned.len() < original_size);
}

#[test]
fn clean_webp_chunk_with_odd_size_padding() {
    let mut webp = b"RIFF".to_vec();
    webp.extend_from_slice(&[0x24, 0x00, 0x00, 0x00]); // File size
    webp.extend_from_slice(b"WEBP");

    // VP8 chunk
    webp.extend_from_slice(b"VP8 ");
    webp.extend_from_slice(&[0x08, 0x00, 0x00, 0x00]); // Chunk size
    webp.extend_from_slice(&[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]); // Data

    // EXIF chunk with odd size (requires padding)
    webp.extend_from_slice(b"EXIF");
    webp.extend_from_slice(&[0x05, 0x00, 0x00, 0x00]); // Odd chunk size (5 bytes)
    webp.extend_from_slice(&[0xAA, 0xBB, 0xCC, 0xDD, 0xEE]); // 5 bytes of data
    webp.push(0x00); // Padding byte for alignment

    let cleaned = BinaryCleaner::clean_metadata(&webp, "webp").unwrap();

    // Should handle padding correctly and remove EXIF chunk
    assert!(!cleaned.windows(4).any(|w| w == b"EXIF"));
    assert!(cleaned.windows(4).any(|w| w == b"VP8 "));
}

// GIF Edge Cases
#[test]
fn clean_gif_truncated_extension_block() {
    let mut gif = b"GIF89a".to_vec();
    gif.extend_from_slice(&[0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00]); // Screen descriptor

    // Truncated comment extension (missing terminator)
    gif.extend_from_slice(&[0x21, 0xFE, 0x05]); // Comment extension, 5 bytes
    gif.extend_from_slice(b"hello"); // 5 bytes of comment
    // Missing 0x00 terminator

    let result = BinaryCleaner::clean_metadata(&gif, "gif");
    // GIF cleaner gracefully handles truncated extensions by breaking out of parsing
    assert!(result.is_ok());
    let cleaned = result.unwrap();
    // Should preserve GIF header and screen descriptor
    assert!(cleaned.starts_with(b"GIF89a"));
}

#[test]
fn clean_gif_application_extension_with_sub_blocks() {
    let mut gif = b"GIF89a".to_vec();
    gif.extend_from_slice(&[0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00]); // Screen descriptor

    // Application extension with multiple sub-blocks
    gif.extend_from_slice(&[0x21, 0xFF, 0x0B]); // Application extension + block size
    gif.extend_from_slice(b"NETSCAPE2.0"); // Application identifier

    // Sub-block 1
    gif.extend_from_slice(&[0x03, 0x01, 0x00, 0x00]); // 3 bytes: loop forever

    // Sub-block 2
    gif.extend_from_slice(&[0x05, b'h', b'e', b'l', b'l', b'o']); // 5 bytes of data

    // Block terminator
    gif.extend_from_slice(&[0x00]);

    // Trailer
    gif.push(0x3B);

    let result = BinaryCleaner::clean_metadata(&gif, "gif");
    assert!(result.is_ok());
    let cleaned = result.unwrap();

    // Should remove the entire application extension
    assert!(!cleaned.windows(2).any(|w| w == [0x21, 0xFF]));
    assert_eq!(cleaned.last(), Some(&0x3B)); // Trailer preserved
}
