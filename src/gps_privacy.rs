//! GPS coordinate privacy utilities for fuzzing location precision.

/// Precision level for GPS coordinate fuzzing.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GpsPrecisionLevel {
    /// Exact coordinates (no fuzzing) - ~1 meter precision
    Exact,
    /// Street level - ~100 meters precision (3 decimal places)
    Street,
    /// Neighborhood level - ~1 km precision (2 decimal places)
    Neighborhood,
    /// City level - ~10 km precision (1 decimal place)
    City,
    /// Region level - ~100 km precision (0 decimal places)
    Region,
}

impl GpsPrecisionLevel {
    /// Get the number of decimal places for this precision level.
    pub fn decimal_places(&self) -> u32 {
        match self {
            GpsPrecisionLevel::Exact => 6,
            GpsPrecisionLevel::Street => 3,
            GpsPrecisionLevel::Neighborhood => 2,
            GpsPrecisionLevel::City => 1,
            GpsPrecisionLevel::Region => 0,
        }
    }

    /// Get a human-readable description of this precision level.
    pub fn description(&self) -> &'static str {
        match self {
            GpsPrecisionLevel::Exact => "Exact location (~1 meter)",
            GpsPrecisionLevel::Street => "Street level (~100 meters)",
            GpsPrecisionLevel::Neighborhood => "Neighborhood (~1 kilometer)",
            GpsPrecisionLevel::City => "City level (~10 kilometers)",
            GpsPrecisionLevel::Region => "Region level (~100 kilometers)",
        }
    }
}

/// Fuzz GPS coordinates to a specified precision level.
///
/// This reduces the precision of coordinates by rounding to fewer decimal places,
/// making it harder to identify exact locations while maintaining general area information.
pub fn fuzz_coordinates(lat: f64, lon: f64, precision: GpsPrecisionLevel) -> (f64, f64) {
    let places = precision.decimal_places();
    let multiplier = 10_f64.powi(places as i32);

    let fuzzed_lat = (lat * multiplier).round() / multiplier;
    let fuzzed_lon = (lon * multiplier).round() / multiplier;

    (fuzzed_lat, fuzzed_lon)
}
