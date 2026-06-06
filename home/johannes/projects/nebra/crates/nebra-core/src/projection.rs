//! WGS84 coordinate projections.
//!
//! Source: Snyder, J.P. "Map Projections - A Working Manual" (1987),
//! USGS Paper 1395.

use crate::types::{ComputationResult, CoreError};

// ---------------------------------------------------------------------------
// Web Mercator Projection
// ---------------------------------------------------------------------------

/// Earth equatorial radius (WGS84) [meters].
const EARTH_EQUATORIAL_RADIUS_M: f64 = 6_378_137.0;

/// Half-circumference at equator [meters].
const ORIGIN_SHIFT_M: f64 = std::f64::consts::PI * EARTH_EQUATORIAL_RADIUS_M;

/// Transform WGS84 geodetic coordinates to Web Mercator (EPSG:3857).
///
/// # Arguments
/// * `lat` - Latitude [degrees]. Bounds: [-85.05112878, 85.05112878].
/// * `lon` - Longitude [degrees]. Bounds: [-180.0, 180.0].
///
/// # Returns
/// `(x, y)` in meters. Clamped to valid Mercator bounds.
///
/// # Source
/// Snyder, J.P. "Map Projections - A Working Manual" (1987), Eq. 7.7.
pub fn wgs84_to_mercator(lat: f64, lon: f64) -> ComputationResult<(f64, f64)> {
    if lat.abs() > 85.051_128_78 || lon.abs() > 180.0 {
        return ComputationResult::err(CoreError::LatLonInvalid);
    }

    let lat_rad = lat.to_radians();
    let x = (lon.to_radians() * EARTH_EQUATORIAL_RADIUS_M)
        .clamp(-ORIGIN_SHIFT_M, ORIGIN_SHIFT_M);
    let y = (lat_rad.tan().ln() * EARTH_EQUATORIAL_RADIUS_M)
        .clamp(-ORIGIN_SHIFT_M, ORIGIN_SHIFT_M);

    ComputationResult::ok((x, y))
}
