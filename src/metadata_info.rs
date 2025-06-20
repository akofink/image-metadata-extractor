pub fn get_metadata_explanation(key: &str) -> Option<&'static str> {
    match key {
        // Camera Information
        "Camera make" | "Make" => Some("Camera manufacturer (e.g., Canon, Nikon, Apple)"),
        "Camera model" | "Model" => Some("Specific camera or device model"),
        "Software" => Some("Camera firmware version or photo editing software used"),

        // Photography Settings
        "F-number" | "FNumber" => {
            Some("Aperture setting - lower numbers = wider aperture, shallower depth of field")
        }
        "Exposure time" | "ExposureTime" => {
            Some("Shutter speed - how long the sensor was exposed to light")
        }
        "ISO speed" | "ISOSpeedRatings" | "PhotographicSensitivity" => {
            Some("Sensor sensitivity - higher ISO = more sensitive but potentially more noise")
        }
        "Focal length" | "FocalLength" => {
            Some("Lens focal length in millimeters - affects field of view and magnification")
        }
        "Exposure bias" | "ExposureBiasValue" => {
            Some("Exposure compensation applied by photographer (+/- EV adjustments)")
        }
        "Exposure mode" | "ExposureMode" => {
            Some("How exposure was determined (manual, auto, aperture priority, etc.)")
        }
        "Exposure program" | "ExposureProgram" => Some("Camera's automatic exposure mode setting"),
        "Metering mode" | "MeteringMode" => {
            Some("How the camera measured light (spot, center-weighted, matrix, etc.)")
        }
        "Flash" => Some("Flash settings and whether flash fired"),
        "White balance" | "WhiteBalance" => Some("Color temperature adjustment setting"),

        // Technical Details
        "Orientation" => Some("Image rotation/orientation when photo was taken"),
        "Resolution unit" | "ResolutionUnit" => {
            Some("Units for image resolution (inches or centimeters)")
        }
        "X resolution" | "XResolution" => Some("Horizontal resolution in pixels per unit"),
        "Y resolution" | "YResolution" => Some("Vertical resolution in pixels per unit"),
        "Color space" | "ColorSpace" => {
            Some("Color encoding standard used (sRGB, Adobe RGB, etc.)")
        }
        "Bits per sample" | "BitsPerSample" => Some("Color depth - bits used per color channel"),
        "Samples per pixel" | "SamplesPerPixel" => {
            Some("Number of color channels (3 for RGB, 4 for CMYK)")
        }
        "Compression" => Some("Image compression method used"),
        "Photometric interpretation" | "PhotometricInterpretation" => {
            Some("How pixel values should be interpreted")
        }

        // Date and Time
        "Date and time" | "DateTime" => Some("When the photo was taken (camera's clock setting)"),
        "Date and time (original)" | "DateTimeOriginal" => {
            Some("Original capture time (usually same as DateTime)")
        }
        "Date and time (digitized)" | "DateTimeDigitized" => {
            Some("When image was digitized (for scanned photos)")
        }
        "Sub-second time" | "SubSecTime" => Some("Fractional seconds for more precise timestamps"),

        // GPS Information
        "GPS latitude" | "GPSLatitude" => {
            Some("Geographic latitude coordinate where photo was taken")
        }
        "GPS longitude" | "GPSLongitude" => {
            Some("Geographic longitude coordinate where photo was taken")
        }
        "GPS altitude" | "GPSAltitude" => Some("Elevation above sea level where photo was taken"),
        "GPS direction" | "GPSImgDirection" => Some("Compass direction the camera was pointing"),
        "GPS speed" | "GPSSpeed" => Some("Speed of device when photo was taken"),
        "GPS date stamp" | "GPSDateStamp" => Some("Date when GPS coordinates were recorded"),
        "GPS time stamp" | "GPSTimeStamp" => Some("UTC time when GPS coordinates were recorded"),

        // Image Characteristics
        "Image width" | "ImageWidth" | "PixelXDimension" => Some("Width of the image in pixels"),
        "Image height" | "ImageLength" | "PixelYDimension" => Some("Height of the image in pixels"),
        "Thumbnail offset" | "ThumbnailOffset" => Some("Location of embedded thumbnail image"),
        "Thumbnail length" | "ThumbnailLength" => Some("Size of embedded thumbnail image"),

        // Lens Information
        "Lens make" | "LensMake" => Some("Manufacturer of the lens used"),
        "Lens model" | "LensModel" => Some("Specific lens model used"),
        "Lens specification" | "LensSpecification" => Some("Technical specifications of the lens"),
        "Max aperture" | "MaxApertureValue" => {
            Some("Maximum aperture (lowest f-number) the lens can achieve")
        }

        // Advanced Settings
        "Scene capture type" | "SceneCaptureType" => {
            Some("Type of scene (standard, landscape, portrait, night, etc.)")
        }
        "Subject distance range" | "SubjectDistanceRange" => {
            Some("Distance category to the main subject")
        }
        "Digital zoom ratio" | "DigitalZoomRatio" => Some("Amount of digital zoom applied"),
        "Contrast" => Some("Contrast adjustment applied by camera"),
        "Saturation" => Some("Color saturation adjustment applied by camera"),
        "Sharpness" => Some("Sharpness adjustment applied by camera"),
        "Scene type" | "SceneType" => Some("Whether image was directly photographed or processed"),
        "Custom rendered" | "CustomRendered" => Some("Special processing applied to the image"),

        _ => None,
    }
}

pub fn get_metadata_category(key: &str) -> &'static str {
    match key {
        k if k.contains("GPS") => "📍 Location",
        k if k.contains("Date") || k.contains("Time") => "🕒 Date & Time",
        k if k.contains("Camera")
            || k.contains("Make")
            || k.contains("Model")
            || k.contains("Software") =>
        {
            "📷 Camera"
        }
        k if k.contains("Lens") => "🔍 Lens",
        k if k.contains("F-number")
            || k.contains("Exposure")
            || k.contains("ISO")
            || k.contains("Focal")
            || k.contains("Flash")
            || k.contains("White")
            || k.contains("Metering") =>
        {
            "⚙️ Settings"
        }
        k if k.contains("width")
            || k.contains("height")
            || k.contains("Resolution")
            || k.contains("Color")
            || k.contains("Bits") =>
        {
            "🖼️ Technical"
        }
        _ => "📊 Other",
    }
}
