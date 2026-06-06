//! Numerical safety primitives for astronomical computation.

use crate::types::{ComputationResult, CoreError};

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Tightest upper bound for inverse trigonometry.
/// Equivalent to nextafter(1.0, 0.0).
pub const TRIG_ARG_MAX: f64 = 1.0 - f64::EPSILON;

/// Tightest lower bound for inverse trigonometry.
/// Equivalent to nextafter(-1.0, 0.0).
pub const TRIG_ARG_MIN: f64 = -1.0 + f64::EPSILON;

/// 1e5× above machine epsilon, 6× above 1 mas signal.
/// Prevents false zero-division without swallowing physical signals.
pub const DENOM_EPSILON: f64 = 1e-10;

// ---------------------------------------------------------------------------
// Domain Guards
// ---------------------------------------------------------------------------

/// Clip argument to safe domain for inverse trigonometry.
pub fn clip_trig_arg(x: f64) -> f64 {
    x.clamp(TRIG_ARG_MIN, TRIG_ARG_MAX)
}

/// Arccos with mandatory domain enforcement.
/// Returns [radians].
pub fn safe_acos(x: f64) -> ComputationResult<f64> {
    if x.is_nan() || x.is_infinite() {
        return ComputationResult::err(CoreError::DomainViolation {
            arg: x,
            context: "safe_acos input",
        });
    }
    if !(-1.0..=1.0).contains(&x) {
        return ComputationResult::err(CoreError::DomainViolation {
            arg: x,
            context: "safe_acos domain",
        });
    }
    ComputationResult::ok(x.acos())
}

/// Arcsin with mandatory domain enforcement.
/// Returns [radians].
pub fn safe_asin(x: f64) -> ComputationResult<f64> {
    if x.is_nan() || x.is_infinite() {
        return ComputationResult::err(CoreError::DomainViolation {
            arg: x,
            context: "safe_asin input",
        });
    }
    if !(-1.0..=1.0).contains(&x) {
        return ComputationResult::err(CoreError::DomainViolation {
            arg: x,
            context: "safe_asin domain",
        });
    }
    ComputationResult::ok(x.asin())
}

// ---------------------------------------------------------------------------
// Angle Normalization
// ---------------------------------------------------------------------------

/// Wrap angle to [0, 360) range. Normalizes -0.0 to 0.0.
/// Source: ERFA function eraAnp.
pub fn wrap_360(x: f64) -> ComputationResult<f64> {
    if x.is_nan() || x.is_infinite() {
        return ComputationResult::err(CoreError::DomainViolation {
            arg: x,
            context: "wrap_360 input",
        });
    }
    let mut w = x % 360.0;
    if w < 0.0 {
        w += 360.0;
    }
    // Honor of Zero: -0.0 becomes 0.0
    if w == 0.0 {
        w = 0.0;
    }
    ComputationResult::ok(w)
}

/// Wrap angle to (-180, 180] range.
/// Source: ERFA function eraAnpm.
pub fn wrap_180(x: f64) -> ComputationResult<f64> {
    if x.is_nan() || x.is_infinite() {
        return ComputationResult::err(CoreError::DomainViolation {
            arg: x,
            context: "wrap_180 input",
        });
    }
    let mut w = x % 360.0;
    if w <= -180.0 {
        w += 360.0;
    }
    if w > 180.0 {
        w -= 360.0;
    }
    ComputationResult::ok(w)
}

// ---------------------------------------------------------------------------
// Compensated Arithmetic
// ---------------------------------------------------------------------------

/// Compensated subtraction.
/// Source: Kahan 1965, "Further Remarks on Reducing Truncation Errors".
pub fn compensated_subtract(a: f64, b: f64) -> (f64, f64) {
    let x = a - b;
    let z = x - (a - b - x);
    (x, z)
}

// ---------------------------------------------------------------------------
// Angular Separation
// ---------------------------------------------------------------------------

/// Angular separation using Vincenty formula.
/// Source: Meeus Ch.17 eq.17.1.
///
/// # Arguments
/// * `lon1` - Longitude of first point [radians].
/// * `lat1` - Latitude of first point [radians].
/// * `lon2` - Longitude of second point [radians].
/// * `lat2` - Latitude of second point [radians].
///
/// # Returns
/// Angular separation [radians].
pub fn angular_separation(lon1: f64, lat1: f64, lon2: f64, lat2: f64) -> ComputationResult<f64> {
    if lon1.is_nan() || lat1.is_nan() || lon2.is_nan() || lat2.is_nan() {
        return ComputationResult::err(CoreError::DomainViolation {
            arg: 0.0,
            context: "angular_separation NaN input",
        });
    }
    let d_lon = lon2 - lon1;
    let sd_lon = d_lon.sin();
    let cd_lon = d_lon.cos();
    let s_lat1 = lat1.sin();
    let c_lat1 = lat1.cos();
    let s_lat2 = lat2.sin();
    let c_lat2 = lat2.cos();

    let num_y = (c_lat2 * sd_lon).hypot(c_lat1 * s_lat2 - s_lat1 * c_lat2 * cd_lon);
    let num_x = s_lat1 * s_lat2 + c_lat1 * c_lat2 * cd_lon;

    ComputationResult::ok(num_y.atan2(num_x))
}
