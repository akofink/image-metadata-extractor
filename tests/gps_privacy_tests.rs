use image_metadata_extractor::gps_privacy::{GpsPrecisionLevel, fuzz_coordinates};

#[test]
fn test_fuzz_exact() {
    let (lat, lon) = fuzz_coordinates(37.123456, -122.654321, GpsPrecisionLevel::Exact);
    assert_eq!(lat, 37.123456);
    assert_eq!(lon, -122.654321);
}

#[test]
fn test_fuzz_street() {
    let (lat, lon) = fuzz_coordinates(37.123456, -122.654321, GpsPrecisionLevel::Street);
    assert_eq!(lat, 37.123);
    assert_eq!(lon, -122.654);
}

#[test]
fn test_fuzz_neighborhood() {
    let (lat, lon) = fuzz_coordinates(37.123456, -122.654321, GpsPrecisionLevel::Neighborhood);
    assert_eq!(lat, 37.12);
    assert_eq!(lon, -122.65);
}

#[test]
fn test_fuzz_city() {
    let (lat, lon) = fuzz_coordinates(37.123456, -122.654321, GpsPrecisionLevel::City);
    assert_eq!(lat, 37.1);
    assert_eq!(lon, -122.7);
}

#[test]
fn test_fuzz_region() {
    let (lat, lon) = fuzz_coordinates(37.123456, -122.654321, GpsPrecisionLevel::Region);
    assert_eq!(lat, 37.0);
    assert_eq!(lon, -123.0);
}
