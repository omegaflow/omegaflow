//! Core types and enumerations for astronomical computation.

use thiserror::Error;

// ---------------------------------------------------------------------------
// Error Codes
// ---------------------------------------------------------------------------

/// Strict error codes for ComputationResult.
#[derive(Debug, Error)]
pub enum CoreError {
    #[error("Initialization failure")]
    InitFailure,
    #[error("Julian Day out of range")]
    JdOutOfRange,
    #[error("Invalid latitude or longitude")]
    LatLonInvalid,
    #[error("Ephemeris unavailable")]
    EphemerisUnavailable,
    #[error("Domain violation: {context} (arg={arg})")]
    DomainViolation { arg: f64, context: &'static str },
}

// ---------------------------------------------------------------------------
// Computation Result
// ---------------------------------------------------------------------------

/// Shared singleton for zero-allocation happy path (Hopper-Kahan compromise).
const _EMPTY_WARNINGS: &[&str] = &[];

/// Result wrapper for all computations.
#[derive(Debug)]
#[must_use]
pub struct ComputationResult<T> {
    pub value: Option<T>,
    pub degraded: bool,
    pub warnings: &'static [&'static str],
    pub error: Option<CoreError>,
}

impl<T> ComputationResult<T> {
    pub fn ok(value: T) -> Self {
        Self {
            value: Some(value),
            degraded: false,
            warnings: _EMPTY_WARNINGS,
            error: None,
        }
    }

    pub fn err(error: CoreError) -> Self {
        Self {
            value: None,
            degraded: false,
            warnings: _EMPTY_WARNINGS,
            error: Some(error),
        }
    }
}

// ---------------------------------------------------------------------------
// Coordinate Frames
// ---------------------------------------------------------------------------

/// Coordinate reference frame. Every coordinate carries one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, strum_macros::EnumIter, serde::Serialize)]
pub enum Frame {
    IcrsJ2000,
    OfDateIau2006,
    Topocentric,
    EclipticOfDate,
    EclipticJ2000,
    Galactic,
}

// ---------------------------------------------------------------------------
// Uncertainty Tracking
// ---------------------------------------------------------------------------

/// Source of position uncertainty.
#[derive(Debug, Clone, Copy, PartialEq, Eq, strum_macros::EnumIter, serde::Serialize)]
pub enum UncertaintySource {
    De440s,
    De441,
    SeElements,
    GaiaDr3,
    Hipparcos,
    MpcOrbit,
    JplSpk,
}

/// Position uncertainty. Every position carries one.
#[derive(Debug, Clone, serde::Serialize)]
pub struct PositionUncertainty {
    pub cross_track_arcsec: f64,
    pub along_track_arcsec: f64,
    pub radial_km: Option<f64>,
    pub source: UncertaintySource,
    pub confidence_sigma: f64,
    pub valid_until_jd: Option<f64>,
}

// ---------------------------------------------------------------------------
// Body Identification
// ---------------------------------------------------------------------------

/// Category of celestial body.
#[derive(Debug, Clone, Copy, PartialEq, Eq, strum_macros::EnumIter, serde::Serialize)]
pub enum BodyCategory {
    Planet,
    Tno,
    Minor,
    Star,
    Comet,
    DerivedLunar,
    Hypothetical,
}

/// Unique identifier for a celestial body.
#[derive(Debug, Clone, serde::Serialize)]
pub struct BodyId {
    name: String,
    pub se_id: Option<i32>,
    pub category: BodyCategory,
}

impl BodyId {
    pub fn new(
        name: String,
        se_id: Option<i32>,
        category: BodyCategory,
    ) -> ComputationResult<Self> {
        if name.is_empty() {
            return ComputationResult::err(CoreError::InitFailure);
        }
        ComputationResult::ok(Self {
            name,
            se_id,
            category,
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

// ---------------------------------------------------------------------------
// Planet Model
// ---------------------------------------------------------------------------

/// Planet model for observer position.
#[derive(Debug, Clone, serde::Serialize)]
pub struct PlanetModel {
    pub name: String,
    pub equatorial_radius_km: f64,
    pub flattening: f64,
    pub gm_km3_s2: f64,
    pub rotation_rate_rad_s: f64,
}

/// WGS84 ellipsoid.
/// Source: NIMA TR8350.2, "Department of Defense World Geodetic System 1984".
pub const EARTH: PlanetModel = PlanetModel {
    name: String::new(),
    equatorial_radius_km: 6378.137,
    flattening: 1.0 / 298.257223563,
    gm_km3_s2: 398600.4418,
    rotation_rate_rad_s: 7.2921159e-5,
};

// ---------------------------------------------------------------------------
// Raw Body
// ---------------------------------------------------------------------------

/// Model-independent astronomical body position.
/// Strictly physical observables.
#[derive(Debug, Clone, serde::Serialize)]
pub struct RawBody {
    pub name: String,
    pub ecl_lon: f64,
    pub ecl_lat: f64,
    pub speed: f64,
    pub distance_au: f64,
    pub ra: f64,
    pub dec: f64,
    pub az: f64,
    pub alt: f64,
    pub helio_lon: Option<f64>,
    pub helio_lat: Option<f64>,
    pub helio_dist: Option<f64>,
    pub frame: Frame,
    pub uncertainty: PositionUncertainty,
}

// ---------------------------------------------------------------------------
// Precision Budget
// ---------------------------------------------------------------------------

/// Precision budget for an astronomical snapshot [arcseconds].
#[derive(Debug, Clone, serde::Serialize)]
pub struct PrecisionBudget {
    pub ephemeris_arcsec: f64,
    pub time_scale_arcsec: f64,
    pub cache_arcsec: f64,
    pub numerical_arcsec: f64,
}

// ---------------------------------------------------------------------------
// Astronomical Snapshot
// ---------------------------------------------------------------------------

/// Model-independent astronomical state at a moment in time.
#[derive(Debug, Clone, serde::Serialize)]
pub struct AstronomicalSnapshot {
    pub jd_utc: f64,
    pub lat: f64,
    pub lon: f64,
    pub alt: f64,
    pub bodies: Vec<RawBody>,
    pub obliquity: f64,
    pub gast: f64,
    pub epsilon: f64,
    pub budget: PrecisionBudget,
}
