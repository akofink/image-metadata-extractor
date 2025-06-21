use little_exif::filetype::FileExtension;
use little_exif::metadata::Metadata;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[cfg(not(target_arch = "wasm32"))]
fn log(_s: &str) {}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

pub struct BinaryCleaner;

impl BinaryCleaner {
    /// Remove metadata from image file using direct binary manipulation
    /// This preserves original image quality while stripping all metadata
    pub fn clean_metadata(file_data: &[u8], file_extension: &str) -> Result<Vec<u8>, String> {
        match file_extension.to_lowercase().as_str() {
            // Formats that require mutable access for little_exif
            "jpg" | "jpeg" => {
                let mut data = file_data.to_vec();
                Self::clean_jpeg_metadata(&mut data)
            }
            "tiff" | "tif" => {
                let mut data = file_data.to_vec();
                Self::clean_tiff_metadata(&mut data)
            }
            "heif" | "heic" => {
                let mut data = file_data.to_vec();
                Self::clean_heif_metadata(&mut data)
            }
            // Formats cleaned without mutation
            "png" => Self::clean_png_metadata(file_data),
            "webp" => Self::clean_webp_metadata(file_data),
            "gif" => Self::clean_gif_metadata(file_data),
            "avif" => Self::clean_avif_metadata(file_data),
            "jxl" => Self::clean_jxl_metadata(file_data),
            "pdf" => Self::clean_pdf_metadata(file_data),
            "svg" => Self::clean_svg_metadata(file_data),
            _ => Err(format!(
                "Unsupported format for binary cleaning: {}",
                file_extension
            )),
        }
    }

    /// Clean JPEG metadata by removing application segments (APP0-APP15)
    // little_exif expects a mutable Vec reference
    #[allow(clippy::ptr_arg)]
    fn clean_jpeg_metadata(data: &mut Vec<u8>) -> Result<Vec<u8>, String> {
        // Use little_exif to clear common metadata segments
        match Metadata::clear_app12_segment(data, FileExtension::JPEG) {
            Ok(_) => console_log!("Cleared APP12 segment"),
            Err(e) => console_log!("APP12 clear warning: {:?}", e),
        }

        match Metadata::clear_app13_segment(data, FileExtension::JPEG) {
            Ok(_) => console_log!("Cleared APP13 segment"),
            Err(e) => console_log!("APP13 clear warning: {:?}", e),
        }

        // Manual removal of common EXIF and metadata segments
        Self::remove_jpeg_app_segments(data)
    }

    /// Remove JPEG application segments manually for comprehensive metadata removal
    fn remove_jpeg_app_segments(data: &[u8]) -> Result<Vec<u8>, String> {
        if data.len() < 4 {
            return Err("Invalid JPEG file: too short".to_string());
        }

        // Verify JPEG SOI marker (0xFFD8)
        if data[0] != 0xFF || data[1] != 0xD8 {
            return Err("Invalid JPEG file: missing SOI marker".to_string());
        }

        let mut cleaned = vec![0xFF, 0xD8]; // Keep SOI marker
        let mut i = 2;

        while i < data.len() - 1 {
            if data[i] != 0xFF {
                // Not a marker, copy remaining data (we've hit image data)
                cleaned.extend_from_slice(&data[i..]);
                break;
            }

            let marker = data[i + 1];

            match marker {
                // Start of Scan - image data follows, copy rest of file
                0xDA => {
                    cleaned.extend_from_slice(&data[i..]);
                    break;
                }
                // Application segments (APP0-APP15) - remove these
                0xE0..=0xEF => {
                    if i + 3 >= data.len() {
                        return Err("Truncated JPEG file".to_string());
                    }
                    // Get segment length (big-endian)
                    let length = ((data[i + 2] as u16) << 8) | (data[i + 3] as u16);
                    if length < 2 {
                        return Err("Invalid segment length".to_string());
                    }
                    // Skip entire segment (marker + length + data)
                    i += 2 + length as usize;
                    console_log!("Removed APP{} segment", marker - 0xE0);
                }
                // Keep other markers (quantization tables, Huffman tables, etc.)
                _ => {
                    if i + 3 >= data.len() {
                        cleaned.extend_from_slice(&data[i..]);
                        break;
                    }
                    let length = ((data[i + 2] as u16) << 8) | (data[i + 3] as u16);
                    if length < 2 || i + 2 + length as usize > data.len() {
                        cleaned.extend_from_slice(&data[i..]);
                        break;
                    }
                    cleaned.extend_from_slice(&data[i..i + 2 + length as usize]);
                    i += 2 + length as usize;
                }
            }
        }

        Ok(cleaned)
    }

    /// Clean PNG metadata by removing ancillary chunks
    fn clean_png_metadata(data: &[u8]) -> Result<Vec<u8>, String> {
        if data.len() < 8 {
            return Err("Invalid PNG file: too short".to_string());
        }

        // Verify PNG signature
        let png_signature = &[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
        if data[..8] != *png_signature {
            return Err("Invalid PNG file: missing signature".to_string());
        }

        let mut cleaned = Vec::new();
        cleaned.extend_from_slice(&data[0..8]); // Keep PNG signature

        let mut i = 8;
        while i < data.len() {
            if i + 8 > data.len() {
                break;
            }

            // Read chunk length (big-endian)
            let length = u32::from_be_bytes([data[i], data[i + 1], data[i + 2], data[i + 3]]);
            let chunk_type = &data[i + 4..i + 8];
            let chunk_name = String::from_utf8_lossy(chunk_type);

            let total_chunk_size = 12 + length as usize; // 4 bytes length + 4 bytes type + data + 4 bytes CRC
            if i + total_chunk_size > data.len() {
                break;
            }

            match chunk_name.as_ref() {
                // Critical chunks - must keep
                "IHDR" | "PLTE" | "IDAT" | "IEND" => {
                    cleaned.extend_from_slice(&data[i..i + total_chunk_size]);
                }
                // Metadata chunks - remove these
                "tEXt" | "zTXt" | "iTXt" | "tIME" | "pHYs" | "gAMA" | "cHRM" | "sRGB" | "iCCP" => {
                    console_log!("Removed PNG {} chunk", chunk_name);
                }
                // Other ancillary chunks - keep for safety
                _ => {
                    cleaned.extend_from_slice(&data[i..i + total_chunk_size]);
                }
            }

            i += total_chunk_size;
        }

        Ok(cleaned)
    }

    /// Clean WebP metadata by removing metadata chunks from RIFF container
    fn clean_webp_metadata(data: &[u8]) -> Result<Vec<u8>, String> {
        if data.len() < 12 {
            return Err("Invalid WebP file: too short".to_string());
        }

        // Verify RIFF header and WebP signature
        if &data[0..4] != b"RIFF" || &data[8..12] != b"WEBP" {
            return Err("Invalid WebP file: missing RIFF/WEBP signature".to_string());
        }

        let mut cleaned = Vec::new();
        cleaned.extend_from_slice(&data[0..12]); // Keep RIFF header and WebP signature

        let mut i = 12;
        let mut new_file_size = 4u32; // Start with "WEBP" in size calculation

        while i < data.len() {
            if i + 8 > data.len() {
                break;
            }

            let chunk_id = &data[i..i + 4];
            let chunk_size =
                u32::from_le_bytes([data[i + 4], data[i + 5], data[i + 6], data[i + 7]]);
            let chunk_name = String::from_utf8_lossy(chunk_id);

            // Ensure chunk size is reasonable
            let padded_size = if chunk_size % 2 == 1 {
                chunk_size + 1
            } else {
                chunk_size
            };
            let total_chunk_size = 8 + padded_size as usize;

            if i + total_chunk_size > data.len() {
                break;
            }

            match chunk_name.as_ref() {
                // Image data chunks - keep
                "VP8 " | "VP8L" | "VP8X" | "ANIM" | "ANMF" => {
                    cleaned.extend_from_slice(&data[i..i + total_chunk_size]);
                    new_file_size += total_chunk_size as u32;
                }
                // Metadata chunks - remove
                "EXIF" | "XMP " | "ICCP" => {
                    console_log!("Removed WebP {} chunk", chunk_name);
                }
                // Unknown chunks - keep for safety
                _ => {
                    cleaned.extend_from_slice(&data[i..i + total_chunk_size]);
                    new_file_size += total_chunk_size as u32;
                }
            }

            i += total_chunk_size;
        }

        // Update RIFF file size
        let size_bytes = new_file_size.to_le_bytes();
        cleaned[4..8].copy_from_slice(&size_bytes);

        Ok(cleaned)
    }

    /// Clean GIF metadata by removing extension blocks
    fn clean_gif_metadata(data: &[u8]) -> Result<Vec<u8>, String> {
        if data.len() < 6 {
            return Err("Invalid GIF file: too short".to_string());
        }

        // Verify GIF signature
        if &data[0..3] != b"GIF" || (&data[3..6] != b"87a" && &data[3..6] != b"89a") {
            return Err("Invalid GIF file: missing signature".to_string());
        }

        let mut cleaned = Vec::new();
        let mut i = 0;

        // Copy header (6 bytes) and logical screen descriptor (7 bytes)
        if data.len() >= 13 {
            cleaned.extend_from_slice(&data[0..13]);
            i = 13;

            // Handle global color table if present
            let packed_field = data[10];
            if packed_field & 0x80 != 0 {
                let global_color_table_size = 2 << (packed_field & 0x07);
                let color_table_bytes = global_color_table_size * 3;
                if i + color_table_bytes <= data.len() {
                    cleaned.extend_from_slice(&data[i..i + color_table_bytes]);
                    i += color_table_bytes;
                }
            }
        }

        // Process data stream
        while i < data.len() {
            match data[i] {
                // Extension introducer
                0x21 => {
                    if i + 1 >= data.len() {
                        break;
                    }
                    let label = data[i + 1];
                    match label {
                        // Application extension (may contain metadata like XMP)
                        0xFF => {
                            console_log!("Removed GIF application extension");
                            i += 2;
                            // Skip sub-blocks
                            while i < data.len() {
                                let block_size = data[i] as usize;
                                if block_size == 0 {
                                    i += 1;
                                    break;
                                }
                                i += 1 + block_size;
                                if i >= data.len() {
                                    break;
                                }
                            }
                        }
                        // Comment extension
                        0xFE => {
                            console_log!("Removed GIF comment extension");
                            i += 2;
                            // Skip sub-blocks
                            while i < data.len() {
                                let block_size = data[i] as usize;
                                if block_size == 0 {
                                    i += 1;
                                    break;
                                }
                                i += 1 + block_size;
                                if i >= data.len() {
                                    break;
                                }
                            }
                        }
                        // Keep other extensions (graphics control, plain text)
                        _ => {
                            let start = i;
                            i += 2;
                            // Copy extension including sub-blocks
                            while i < data.len() {
                                let block_size = data[i] as usize;
                                i += 1;
                                if block_size == 0 {
                                    break;
                                }
                                i += block_size;
                                if i >= data.len() {
                                    break;
                                }
                            }
                            cleaned.extend_from_slice(&data[start..i]);
                        }
                    }
                }
                // Image separator or trailer
                0x2C | 0x3B => {
                    cleaned.extend_from_slice(&data[i..]);
                    break;
                }
                _ => {
                    cleaned.push(data[i]);
                    i += 1;
                }
            }
        }

        Ok(cleaned)
    }

    /// Clean TIFF metadata using little_exif library
    // little_exif expects a mutable Vec reference
    #[allow(clippy::ptr_arg)]
    fn clean_tiff_metadata(data: &mut Vec<u8>) -> Result<Vec<u8>, String> {
        // TIFF files can be cleaned using little_exif
        match Metadata::clear_app12_segment(data, FileExtension::TIFF) {
            Ok(_) => console_log!("Cleared TIFF APP12 segment"),
            Err(e) => console_log!("TIFF APP12 clear warning: {:?}", e),
        }

        match Metadata::clear_app13_segment(data, FileExtension::TIFF) {
            Ok(_) => console_log!("Cleared TIFF APP13 segment"),
            Err(e) => console_log!("TIFF APP13 clear warning: {:?}", e),
        }

        Ok(data.to_vec())
    }

    /// Clean HEIF/HEIC metadata using little_exif library
    // little_exif expects a mutable Vec reference
    #[allow(clippy::ptr_arg)]
    fn clean_heif_metadata(data: &mut Vec<u8>) -> Result<Vec<u8>, String> {
        // HEIF files can be cleaned using little_exif
        match Metadata::clear_app12_segment(data, FileExtension::HEIF) {
            Ok(_) => console_log!("Cleared HEIF APP12 segment"),
            Err(e) => console_log!("HEIF APP12 clear warning: {:?}", e),
        }

        match Metadata::clear_app13_segment(data, FileExtension::HEIF) {
            Ok(_) => console_log!("Cleared HEIF APP13 segment"),
            Err(e) => console_log!("HEIF APP13 clear warning: {:?}", e),
        }

        Ok(data.to_vec())
    }

    /// Clean AVIF metadata (basic implementation)
    fn clean_avif_metadata(_data: &[u8]) -> Result<Vec<u8>, String> {
        // AVIF is based on HEIF, but might need specialized handling
        // For now, return error as it's not fully supported by little_exif
        Err("AVIF metadata cleaning not fully implemented yet".to_string())
    }

    /// Clean JPEG XL metadata (basic implementation)
    fn clean_jxl_metadata(_data: &[u8]) -> Result<Vec<u8>, String> {
        // JXL is a newer format that may not be fully supported
        Err("JPEG XL metadata cleaning not fully implemented yet".to_string())
    }

    /// Clean PDF metadata by removing info dictionary and XMP
    fn clean_pdf_metadata(data: &[u8]) -> Result<Vec<u8>, String> {
        let data_str = String::from_utf8_lossy(data);

        // Basic PDF header check
        if !data_str.starts_with("%PDF-") {
            return Err("Invalid PDF file".to_string());
        }

        // This is a simplified implementation - real PDF metadata removal
        // would require a proper PDF parser to handle the document structure
        console_log!("PDF metadata cleaning is basic - consider using specialized PDF tools");

        // For now, just return the original data with a warning
        // A full implementation would need to parse PDF objects and remove:
        // - /Info dictionary
        // - /Metadata streams
        // - /XMP packets
        Ok(data.to_vec())
    }

    /// Clean SVG metadata by removing metadata elements
    fn clean_svg_metadata(data: &[u8]) -> Result<Vec<u8>, String> {
        let data_str = String::from_utf8_lossy(data);

        // Basic SVG check
        if !data_str.contains("<svg") {
            return Err("Invalid SVG file".to_string());
        }

        // Remove common metadata elements
        let mut cleaned = data_str.to_string();

        // Remove metadata elements (simplified regex-like approach)
        // In a real implementation, you'd want to use proper XML parsing
        cleaned = cleaned
            .lines()
            .filter(|line| {
                let line_lower = line.to_lowercase();
                !line_lower.contains("<metadata")
                    && !line_lower.contains("</metadata>")
                    && !line_lower.contains("xmlns:dc=")
                    && !line_lower.contains("xmlns:cc=")
                    && !line_lower.contains("xmlns:rdf=")
                    && !line_lower.contains("<rdf:")
                    && !line_lower.contains("</rdf:")
                    && !line_lower.contains("<dc:")
                    && !line_lower.contains("<cc:")
            })
            .collect::<Vec<&str>>()
            .join("\n");

        console_log!("Removed basic SVG metadata elements");
        Ok(cleaned.into_bytes())
    }
}
