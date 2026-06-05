//! WGS84 coordinate projections.

use crate::types::{ComputationResult, ErrorCode};

// ---------------------------------------------------------------------------
// Web Mercator Projection
// ---------------------------------------------------------------------------

const EARTH_EQUATORIAL_RADIUS_M: f64 = 6_378_137.0;
const ORIGIN_SHIFT_M: f64 = 2.0 * std::f64::consts::PI * EARTH_EQUATORIAL_RADIUS_M / 2.0;

/// Transform WGS84 to Web Mercator.
///
/// Source: Snyder, J.P. "Map Projections - A Working Manual" (1987), USGS Paper 1395, Eq. 7.7.
///
/// # Arguments
/// * `lat` - Latitude [degrees]. Bounds: [-85.051, 85.051].
/// * `lon` - Longitude [degrees]. Bounds: [-180.0, 180.0].
///
/// # Returns
/// `(x, y)` in meters.
pub fn wgs84_to_mercator(lat: f64, lon: f64) -> ComputationResult<(f64, f64)> {
    if lat.abs() > 85.0511287798 || lon.abs() > 180.0 {
        return ComputationResult {
            value: None,
            error: Some(ErrorCode::LAT_LON_INVALID),
            ..Default::default()
        };
    }

    let lat_rad = lat.to_radians();
    let x = (lon.to_radians() * EARTH_EQUATORIAL_RADIUS_M).clamp(-ORIGIN_SHIFT_M, ORIGIN_SHIFT_M);
    let y = (lat_rad.tan().ln() * EARTH_EQUATORIAL_RADIUS_M).clamp(-ORIGIN_SHIFT_M, ORIGIN_SHIFT_M);

    ComputationResult {
        value: Some((x, y)),
        error: None,
        ..Default::default()
    }
}

