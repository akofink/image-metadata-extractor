//! Provide human readable explanations and categories for metadata keys.
//!
//! This module uses data-driven approach with static collections for better
//! testability and maintainability compared to large match statements.

use std::collections::HashMap;

/// Metadata explanation entry
#[derive(Debug, Clone)]
pub struct MetadataInfo {
    pub category: &'static str,
    pub explanation: &'static str,
}

/// Static metadata information database
/// This approach allows for 100% test coverage without testing every individual key
pub static METADATA_DB: &[(&str, MetadataInfo)] = &[
    // Camera Information
    (
        "Camera make",
        MetadataInfo {
            category: "📷 Camera",
            explanation: "Camera manufacturer (e.g., Canon, Nikon, Apple)",
        },
    ),
    (
        "Make",
        MetadataInfo {
            category: "📷 Camera",
            explanation: "Camera manufacturer (e.g., Canon, Nikon, Apple)",
        },
    ),
    (
        "Camera model",
        MetadataInfo {
            category: "📷 Camera",
            explanation: "Specific camera or device model",
        },
    ),
    (
        "Model",
        MetadataInfo {
            category: "📷 Camera",
            explanation: "Specific camera or device model",
        },
    ),
    (
        "Software",
        MetadataInfo {
            category: "📷 Camera",
            explanation: "Camera firmware version or photo editing software used",
        },
    ),
    // Photography Settings
    (
        "F-number",
        MetadataInfo {
            category: "⚙️ Settings",
            explanation: "Aperture setting - lower numbers = wider aperture, shallower depth of field",
        },
    ),
    (
        "FNumber",
        MetadataInfo {
            category: "⚙️ Settings",
            explanation: "Aperture setting - lower numbers = wider aperture, shallower depth of field",
        },
    ),
    (
        "ISO speed",
        MetadataInfo {
            category: "⚙️ Settings",
            explanation: "Sensor sensitivity - higher ISO = more sensitive but potentially more noise",
        },
    ),
    (
        "Exposure time",
        MetadataInfo {
            category: "⚙️ Settings",
            explanation: "Shutter speed - how long the sensor was exposed to light",
        },
    ),
    (
        "ExposureTime",
        MetadataInfo {
            category: "⚙️ Settings",
            explanation: "Shutter speed - how long the sensor was exposed to light",
        },
    ),
    (
        "ISOSpeedRatings",
        MetadataInfo {
            category: "⚙️ Settings",
            explanation: "Sensor sensitivity - higher ISO = more sensitive but potentially more noise",
        },
    ),
    (
        "PhotographicSensitivity",
        MetadataInfo {
            category: "⚙️ Settings",
            explanation: "Sensor sensitivity - higher ISO = more sensitive but potentially more noise",
        },
    ),
    (
        "Focal length",
        MetadataInfo {
            category: "⚙️ Settings",
            explanation: "Lens focal length in millimeters - affects field of view and magnification",
        },
    ),
    (
        "FocalLength",
        MetadataInfo {
            category: "⚙️ Settings",
            explanation: "Lens focal length in millimeters - affects field of view and magnification",
        },
    ),
    (
        "Exposure bias",
        MetadataInfo {
            category: "⚙️ Settings",
            explanation: "Exposure compensation applied by photographer (+/- EV adjustments)",
        },
    ),
    (
        "ExposureBiasValue",
        MetadataInfo {
            category: "⚙️ Settings",
            explanation: "Exposure compensation applied by photographer (+/- EV adjustments)",
        },
    ),
    (
        "Exposure mode",
        MetadataInfo {
            category: "⚙️ Settings",
            explanation: "How exposure was determined (manual, auto, aperture priority, etc.)",
        },
    ),
    (
        "ExposureMode",
        MetadataInfo {
            category: "⚙️ Settings",
            explanation: "How exposure was determined (manual, auto, aperture priority, etc.)",
        },
    ),
    (
        "Exposure program",
        MetadataInfo {
            category: "⚙️ Settings",
            explanation: "Camera's automatic exposure mode setting",
        },
    ),
    (
        "ExposureProgram",
        MetadataInfo {
            category: "⚙️ Settings",
            explanation: "Camera's automatic exposure mode setting",
        },
    ),
    (
        "Metering mode",
        MetadataInfo {
            category: "⚙️ Settings",
            explanation: "How the camera measured light (spot, center-weighted, matrix, etc.)",
        },
    ),
    (
        "MeteringMode",
        MetadataInfo {
            category: "⚙️ Settings",
            explanation: "How the camera measured light (spot, center-weighted, matrix, etc.)",
        },
    ),
    (
        "Flash",
        MetadataInfo {
            category: "⚙️ Settings",
            explanation: "Flash settings and whether flash fired",
        },
    ),
    (
        "White balance",
        MetadataInfo {
            category: "⚙️ Settings",
            explanation: "Color temperature adjustment setting",
        },
    ),
    (
        "WhiteBalance",
        MetadataInfo {
            category: "⚙️ Settings",
            explanation: "Color temperature adjustment setting",
        },
    ),
    (
        "Scene capture type",
        MetadataInfo {
            category: "⚙️ Settings",
            explanation: "Type of scene (standard, landscape, portrait, night, etc.)",
        },
    ),
    (
        "SceneCaptureType",
        MetadataInfo {
            category: "⚙️ Settings",
            explanation: "Type of scene (standard, landscape, portrait, night, etc.)",
        },
    ),
    (
        "Subject distance range",
        MetadataInfo {
            category: "⚙️ Settings",
            explanation: "Distance category to the main subject",
        },
    ),
    (
        "SubjectDistanceRange",
        MetadataInfo {
            category: "⚙️ Settings",
            explanation: "Distance category to the main subject",
        },
    ),
    (
        "Digital zoom ratio",
        MetadataInfo {
            category: "⚙️ Settings",
            explanation: "Amount of digital zoom applied",
        },
    ),
    (
        "DigitalZoomRatio",
        MetadataInfo {
            category: "⚙️ Settings",
            explanation: "Amount of digital zoom applied",
        },
    ),
    (
        "Contrast",
        MetadataInfo {
            category: "⚙️ Settings",
            explanation: "Contrast adjustment applied by camera",
        },
    ),
    (
        "Saturation",
        MetadataInfo {
            category: "⚙️ Settings",
            explanation: "Color saturation adjustment applied by camera",
        },
    ),
    (
        "Sharpness",
        MetadataInfo {
            category: "⚙️ Settings",
            explanation: "Sharpness adjustment applied by camera",
        },
    ),
    (
        "Scene type",
        MetadataInfo {
            category: "⚙️ Settings",
            explanation: "Whether image was directly photographed or processed",
        },
    ),
    (
        "SceneType",
        MetadataInfo {
            category: "⚙️ Settings",
            explanation: "Whether image was directly photographed or processed",
        },
    ),
    (
        "Custom rendered",
        MetadataInfo {
            category: "⚙️ Settings",
            explanation: "Special processing applied to the image",
        },
    ),
    (
        "CustomRendered",
        MetadataInfo {
            category: "⚙️ Settings",
            explanation: "Special processing applied to the image",
        },
    ),
    // Technical Details
    (
        "Orientation",
        MetadataInfo {
            category: "🖼️ Technical",
            explanation: "Image rotation/orientation when photo was taken",
        },
    ),
    (
        "Resolution unit",
        MetadataInfo {
            category: "🖼️ Technical",
            explanation: "Units for image resolution (inches or centimeters)",
        },
    ),
    (
        "ResolutionUnit",
        MetadataInfo {
            category: "🖼️ Technical",
            explanation: "Units for image resolution (inches or centimeters)",
        },
    ),
    (
        "X resolution",
        MetadataInfo {
            category: "🖼️ Technical",
            explanation: "Horizontal resolution in pixels per unit",
        },
    ),
    (
        "XResolution",
        MetadataInfo {
            category: "🖼️ Technical",
            explanation: "Horizontal resolution in pixels per unit",
        },
    ),
    (
        "Y resolution",
        MetadataInfo {
            category: "🖼️ Technical",
            explanation: "Vertical resolution in pixels per unit",
        },
    ),
    (
        "YResolution",
        MetadataInfo {
            category: "🖼️ Technical",
            explanation: "Vertical resolution in pixels per unit",
        },
    ),
    (
        "Color space",
        MetadataInfo {
            category: "🖼️ Technical",
            explanation: "Color encoding standard used (sRGB, Adobe RGB, etc.)",
        },
    ),
    (
        "ColorSpace",
        MetadataInfo {
            category: "🖼️ Technical",
            explanation: "Color encoding standard used (sRGB, Adobe RGB, etc.)",
        },
    ),
    (
        "Bits per sample",
        MetadataInfo {
            category: "🖼️ Technical",
            explanation: "Color depth - bits used per color channel",
        },
    ),
    (
        "BitsPerSample",
        MetadataInfo {
            category: "🖼️ Technical",
            explanation: "Color depth - bits used per color channel",
        },
    ),
    (
        "Samples per pixel",
        MetadataInfo {
            category: "🖼️ Technical",
            explanation: "Number of color channels (3 for RGB, 4 for CMYK)",
        },
    ),
    (
        "SamplesPerPixel",
        MetadataInfo {
            category: "🖼️ Technical",
            explanation: "Number of color channels (3 for RGB, 4 for CMYK)",
        },
    ),
    (
        "Compression",
        MetadataInfo {
            category: "🖼️ Technical",
            explanation: "Image compression method used",
        },
    ),
    (
        "Photometric interpretation",
        MetadataInfo {
            category: "🖼️ Technical",
            explanation: "How pixel values should be interpreted",
        },
    ),
    (
        "PhotometricInterpretation",
        MetadataInfo {
            category: "🖼️ Technical",
            explanation: "How pixel values should be interpreted",
        },
    ),
    (
        "Image width",
        MetadataInfo {
            category: "🖼️ Technical",
            explanation: "Width of the image in pixels",
        },
    ),
    (
        "ImageWidth",
        MetadataInfo {
            category: "🖼️ Technical",
            explanation: "Width of the image in pixels",
        },
    ),
    (
        "PixelXDimension",
        MetadataInfo {
            category: "🖼️ Technical",
            explanation: "Width of the image in pixels",
        },
    ),
    (
        "Image height",
        MetadataInfo {
            category: "🖼️ Technical",
            explanation: "Height of the image in pixels",
        },
    ),
    (
        "ImageLength",
        MetadataInfo {
            category: "🖼️ Technical",
            explanation: "Height of the image in pixels",
        },
    ),
    (
        "PixelYDimension",
        MetadataInfo {
            category: "🖼️ Technical",
            explanation: "Height of the image in pixels",
        },
    ),
    (
        "Thumbnail offset",
        MetadataInfo {
            category: "🖼️ Technical",
            explanation: "Location of embedded thumbnail image",
        },
    ),
    (
        "ThumbnailOffset",
        MetadataInfo {
            category: "🖼️ Technical",
            explanation: "Location of embedded thumbnail image",
        },
    ),
    (
        "Thumbnail length",
        MetadataInfo {
            category: "🖼️ Technical",
            explanation: "Size of embedded thumbnail image",
        },
    ),
    (
        "ThumbnailLength",
        MetadataInfo {
            category: "🖼️ Technical",
            explanation: "Size of embedded thumbnail image",
        },
    ),
    // Date and Time
    (
        "Date and time",
        MetadataInfo {
            category: "🕒 Date & Time",
            explanation: "When the photo was taken (camera's clock setting)",
        },
    ),
    (
        "DateTime",
        MetadataInfo {
            category: "🕒 Date & Time",
            explanation: "When the photo was taken (camera's clock setting)",
        },
    ),
    (
        "Date and time (original)",
        MetadataInfo {
            category: "🕒 Date & Time",
            explanation: "Original capture time (usually same as DateTime)",
        },
    ),
    (
        "DateTimeOriginal",
        MetadataInfo {
            category: "🕒 Date & Time",
            explanation: "Original capture time (usually same as DateTime)",
        },
    ),
    (
        "Date and time (digitized)",
        MetadataInfo {
            category: "🕒 Date & Time",
            explanation: "When image was digitized (for scanned photos)",
        },
    ),
    (
        "DateTimeDigitized",
        MetadataInfo {
            category: "🕒 Date & Time",
            explanation: "When image was digitized (for scanned photos)",
        },
    ),
    (
        "Sub-second time",
        MetadataInfo {
            category: "🕒 Date & Time",
            explanation: "Fractional seconds for more precise timestamps",
        },
    ),
    (
        "SubSecTime",
        MetadataInfo {
            category: "🕒 Date & Time",
            explanation: "Fractional seconds for more precise timestamps",
        },
    ),
    // Location
    (
        "GPS latitude",
        MetadataInfo {
            category: "📍 Location",
            explanation: "Geographic latitude coordinate where photo was taken",
        },
    ),
    (
        "GPSLatitude",
        MetadataInfo {
            category: "📍 Location",
            explanation: "Geographic latitude coordinate where photo was taken",
        },
    ),
    (
        "GPS longitude",
        MetadataInfo {
            category: "📍 Location",
            explanation: "Geographic longitude coordinate where photo was taken",
        },
    ),
    (
        "GPSLongitude",
        MetadataInfo {
            category: "📍 Location",
            explanation: "Geographic longitude coordinate where photo was taken",
        },
    ),
    (
        "GPS altitude",
        MetadataInfo {
            category: "📍 Location",
            explanation: "Elevation above sea level where photo was taken",
        },
    ),
    (
        "GPSAltitude",
        MetadataInfo {
            category: "📍 Location",
            explanation: "Elevation above sea level where photo was taken",
        },
    ),
    (
        "GPS direction",
        MetadataInfo {
            category: "📍 Location",
            explanation: "Compass direction the camera was pointing",
        },
    ),
    (
        "GPSImgDirection",
        MetadataInfo {
            category: "📍 Location",
            explanation: "Compass direction the camera was pointing",
        },
    ),
    (
        "GPS speed",
        MetadataInfo {
            category: "📍 Location",
            explanation: "Speed of device when photo was taken",
        },
    ),
    (
        "GPSSpeed",
        MetadataInfo {
            category: "📍 Location",
            explanation: "Speed of device when photo was taken",
        },
    ),
    (
        "GPS date stamp",
        MetadataInfo {
            category: "📍 Location",
            explanation: "Date when GPS coordinates were recorded",
        },
    ),
    (
        "GPSDateStamp",
        MetadataInfo {
            category: "📍 Location",
            explanation: "Date when GPS coordinates were recorded",
        },
    ),
    (
        "GPS time stamp",
        MetadataInfo {
            category: "📍 Location",
            explanation: "UTC time when GPS coordinates were recorded",
        },
    ),
    (
        "GPSTimeStamp",
        MetadataInfo {
            category: "📍 Location",
            explanation: "UTC time when GPS coordinates were recorded",
        },
    ),
    // Lens Information
    (
        "Lens make",
        MetadataInfo {
            category: "🔍 Lens",
            explanation: "Manufacturer of the lens used",
        },
    ),
    (
        "LensMake",
        MetadataInfo {
            category: "🔍 Lens",
            explanation: "Manufacturer of the lens used",
        },
    ),
    (
        "Lens model",
        MetadataInfo {
            category: "🔍 Lens",
            explanation: "Specific lens model used",
        },
    ),
    (
        "LensModel",
        MetadataInfo {
            category: "🔍 Lens",
            explanation: "Specific lens model used",
        },
    ),
    (
        "Lens specification",
        MetadataInfo {
            category: "🔍 Lens",
            explanation: "Technical specifications of the lens",
        },
    ),
    (
        "LensSpecification",
        MetadataInfo {
            category: "🔍 Lens",
            explanation: "Technical specifications of the lens",
        },
    ),
    (
        "Max aperture",
        MetadataInfo {
            category: "🔍 Lens",
            explanation: "Maximum aperture (lowest f-number) the lens can achieve",
        },
    ),
    (
        "MaxApertureValue",
        MetadataInfo {
            category: "🔍 Lens",
            explanation: "Maximum aperture (lowest f-number) the lens can achieve",
        },
    ),
];

/// Build lookup map from static data
fn build_lookup_map() -> HashMap<&'static str, &'static MetadataInfo> {
    METADATA_DB.iter().map(|(k, v)| (*k, v)).collect()
}

/// Return an explanatory string for a given metadata key.
pub fn get_metadata_explanation(key: &str) -> Option<&'static str> {
    static LOOKUP: std::sync::OnceLock<HashMap<&'static str, &'static MetadataInfo>> =
        std::sync::OnceLock::new();
    let map = LOOKUP.get_or_init(build_lookup_map);
    map.get(key).map(|info| info.explanation)
}

/// Return the category for a given metadata key.
pub fn get_metadata_category(key: &str) -> &'static str {
    static LOOKUP: std::sync::OnceLock<HashMap<&'static str, &'static MetadataInfo>> =
        std::sync::OnceLock::new();
    let map = LOOKUP.get_or_init(build_lookup_map);
    map.get(key).map(|info| info.category).unwrap_or("📊 Other")
}
