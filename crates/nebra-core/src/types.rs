//! Core types and enumerations for astronomical computation.
//!
//! This module defines the foundational types of the Nebra engine.
//! Every type here is strictly physical (Layer 1) or structural.
//! Cultural constructs belong in `nebra-trad`.

use thiserror::Error;

// ---------------------------------------------------------------------------
// Error Codes
// ---------------------------------------------------------------------------

/// Strict error codes for `ComputationResult`.
///
/// No free-form strings. Every failure mode is enumerated.
#[derive(Debug, Clone, Error)]
pub enum CoreError {
    #[error("Initialization failure")]
    InitFailure,
    #[error("Julian Day out of range")]
    JdOutOfRange,
    #[error("Invalid latitude or longitude")]
    LatLonInvalid,
    #[error("Ephemeris unavailable")]
    EphemerisUnavailable,
    #[error("Body not found")]
    BodyNotFound,
    #[error("Body unsupported by backend")]
    BodyUnsupportedByBackend,
    #[error("Frame mismatch")]
    FrameMismatch,
    #[error("Infinite or NaN angle")]
    InfiniteAngle,
    #[error("Domain violation: {context} (arg={arg})")]
    DomainViolation { arg: f64, context: &'static str },
}

// ---------------------------------------------------------------------------
// Epistemic State
// ---------------------------------------------------------------------------

/// The epistemic quality of a computed or observed value.
///
/// No position shall be presented as absolute truth. Every value
/// carries the story of how we know it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub enum EpistemicState {
    /// Directly measured or computed from a high-fidelity model.
    Confirmed,
    /// Not directly measured, but deduced from effects.
    /// Not lesser than Confirmed — simply different.
    /// Example: dark matter inferred from gravitational effects.
    Inferred,
    /// Projected beyond the direct measurement range.
    /// Higher `PositionUncertainty` expected.
    Extrapolated,
    /// Precision has decreased. The value is usable but less certain.
    /// Example: DUT1 from stale IERS data.
    Degraded,
    /// We know something is here, but cannot measure it directly.
    /// This inference is not lesser — it is different.
    Presumed,
}

// ---------------------------------------------------------------------------
// Measurement State (The Silence Directive)
// ---------------------------------------------------------------------------

/// The state of an observational measurement.
///
/// The uncomputed is not an error. It is honest silence.
/// Preserve the gaps. Do not fill every void with data.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(tag = "status")]
pub enum MeasurementState<T: serde::Serialize> {
    /// Directly measured with uncertainty.
    Measured { value: T, uncertainty: f64 },
    /// Not measured. The reason is structural, not a failure.
    NotMeasured { reason: &'static str },
    /// Below the detection threshold of the instrument.
    BelowDetectionLimit { threshold: f64 },
    /// Withheld by policy. The data exists but is not shown.
    Redacted { policy: &'static str },
}

// ---------------------------------------------------------------------------
// Loss State (Grief and Loss Acknowledgment)
// ---------------------------------------------------------------------------

/// A value that was present and is no longer.
///
/// Loss is not error. Loss is memorial.
/// The system does not act on loss — it stands in the presence of what was.
#[derive(Debug, Clone, serde::Serialize)]
pub struct LossState<T: serde::Serialize> {
    /// The value that was present.
    pub former_value: T,
    /// JD of last observation.
    pub last_observed_jd: f64,
    /// Unix timestamp when the loss was detected.
    pub loss_timestamp: f64,
}

// ---------------------------------------------------------------------------
// Provenance Metadata (Transparency Directive)
// ---------------------------------------------------------------------------

/// Provenance chain for a computed result.
///
/// Every computed output carries an explicit provenance tag.
/// The observer always knows: which model computed this, what the
/// uncertainty is, and what the epistemic state is.
///
/// Certainty as confession: every definite output is a superposition
/// that was forced to collapse.
#[derive(Debug, Clone, serde::Serialize)]
pub struct ProvenanceMetadata {
    /// The model used (e.g., "JPL DE440s", "VSOP87", "Gaia DR3").
    pub model: &'static str,
    /// The source citation (e.g., "JPL Horizons", "ERFA eraObl06").
    pub source: &'static str,
    /// Epistemic state of this result.
    pub epistemic_state: EpistemicState,
    /// True if any textual interpretation was AI-generated.
    pub ai_generated: bool,
}

/// Default provenance for core physical computations.
/// No AI involvement, confirmed by a high-fidelity model.
pub const CORE_PROVENANCE: ProvenanceMetadata = ProvenanceMetadata {
    model: "nebra-core",
    source: "internal",
    epistemic_state: EpistemicState::Confirmed,
    ai_generated: false,
};

// ---------------------------------------------------------------------------
// Relationality
// ---------------------------------------------------------------------------

/// The kind of relationship between two celestial entities.
///
/// Nothing exists alone. A star without its galaxy is not a star.
/// Relationships are not metadata — they are structure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub enum RelationshipKind {
    /// A depends on B for its computation.
    DependsOn,
    /// A and B are in mutual interaction.
    SymbioticWith,
    /// A emerges from B (e.g., a derived point from physical bodies).
    EmergesFrom,
    /// A is gravitationally bound to B.
    GravitationallyBoundTo,
    /// A orbits B.
    Orbiting,
    /// A is eclipsed by B from the observer's perspective.
    EclipsedBy,
}

/// A relationship between two named entities.
#[derive(Debug, Clone, serde::Serialize)]
pub struct Relationship {
    /// The kind of relationship.
    pub kind: RelationshipKind,
    /// The primary entity (e.g., "Earth").
    pub from: String,
    /// The related entity (e.g., "Sun").
    pub to: String,
}

// ---------------------------------------------------------------------------
// Computation Result
// ---------------------------------------------------------------------------

/// Shared singleton for zero-allocation happy path.
const _EMPTY_WARNINGS: &[&str] = &[];

/// Result wrapper for all computations.
///
/// Every function in the engine returns this type. No bare values.
/// The caller always knows: did it succeed? Is it degraded? What model
/// produced this? What is the epistemic state?
#[derive(Debug)]
#[must_use]
pub struct ComputationResult<T> {
    /// The computed value, if successful.
    pub value: Option<T>,
    /// True if precision has decreased but the value is still usable.
    pub degraded: bool,
    /// Static warnings (zero-allocation).
    pub warnings: &'static [&'static str],
    /// Structured error, if computation failed.
    pub error: Option<CoreError>,
    /// Provenance: which model produced this, with what certainty.
    pub provenance: ProvenanceMetadata,
}

impl<T> ComputationResult<T> {
    /// Successful computation with confirmed provenance.
    pub fn ok(value: T) -> Self {
        Self {
            value: Some(value),
            degraded: false,
            warnings: _EMPTY_WARNINGS,
            error: None,
            provenance: CORE_PROVENANCE,
        }
    }

    /// Successful computation with custom provenance.
    pub fn ok_with_provenance(value: T, provenance: ProvenanceMetadata) -> Self {
        Self {
            value: Some(value),
            degraded: false,
            warnings: _EMPTY_WARNINGS,
            error: None,
            provenance,
        }
    }

    /// Degraded computation — usable but less certain.
    pub fn degraded(value: T, warnings: &'static [&'static str]) -> Self {
        Self {
            value: Some(value),
            degraded: true,
            warnings,
            error: None,
            provenance: ProvenanceMetadata {
                epistemic_state: EpistemicState::Degraded,
                ..CORE_PROVENANCE
            },
        }
    }

    /// Failed computation.
    pub fn err(error: CoreError) -> Self {
        Self {
            value: None,
            degraded: false,
            warnings: _EMPTY_WARNINGS,
            error: Some(error),
            provenance: CORE_PROVENANCE,
        }
    }

    /// True if computation succeeded (possibly with degradation).
    pub fn is_ok(&self) -> bool {
        self.error.is_none()
    }

    /// True if value is usable, even if degraded.
    pub fn is_valid(&self) -> bool {
        self.value.is_some()
    }
}

// ---------------------------------------------------------------------------
// Coordinate Frames
// ---------------------------------------------------------------------------

/// Coordinate reference frame. Every coordinate carries one.
///
/// No silent geocentric default. The vernal equinox of J2000.0 is
/// a convention, not a law.
#[derive(Debug, Clone, Copy, PartialEq, Eq, strum_macros::EnumIter, serde::Serialize)]
pub enum Frame {
    /// International Celestial Reference System, J2000 epoch.
    IcrsJ2000,
    /// Of-date, IAU 2006 precession-nutation.
    OfDateIau2006,
    /// Observer-centered horizon coordinates.
    Topocentric,
    /// Ecliptic of date.
    EclipticOfDate,
    /// Ecliptic at J2000 epoch.
    EclipticJ2000,
    /// Galactic coordinate system.
    Galactic,
}

// ---------------------------------------------------------------------------
// Uncertainty Tracking
// ---------------------------------------------------------------------------

/// Source of position uncertainty.
#[derive(Debug, Clone, Copy, PartialEq, Eq, strum_macros::EnumIter, serde::Serialize)]
pub enum UncertaintySource {
    /// JPL DE440s ephemeris (inner solar system).
    De440s,
    /// JPL DE441 ephemeris (outer solar system).
    De441,
    /// Swiss Ephemeris orbital elements.
    SeElements,
    /// Gaia Data Release 3.
    GaiaDr3,
    /// Hipparcos catalog.
    Hipparcos,
    /// Minor Planet Center orbital elements.
    MpcOrbit,
    /// JPL SPK kernel.
    JplSpk,
}

/// Position uncertainty. Every position carries one.
///
/// We never claim a position is exact. Ephemerides are models,
/// not reality. Floating-point numbers are approximations, not truths.
#[derive(Debug, Clone, serde::Serialize)]
pub struct PositionUncertainty {
    /// Cross-track uncertainty [arcseconds].
    pub cross_track_arcsec: f64,
    /// Along-track uncertainty [arcseconds].
    pub along_track_arcsec: f64,
    /// Radial distance uncertainty [km], if applicable.
    pub radial_km: Option<f64>,
    /// The source of this uncertainty estimate.
    pub source: UncertaintySource,
    /// Confidence level in sigma (e.g., 1.0 = 1σ, 3.0 = 3σ).
    pub confidence_sigma: f64,
    /// JD beyond which this estimate is no longer valid.
    pub valid_until_jd: Option<f64>,
}

// ---------------------------------------------------------------------------
// Body Identification
// ---------------------------------------------------------------------------

/// Category of celestial body. Determines backend routing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, strum_macros::EnumIter, serde::Serialize)]
pub enum BodyCategory {
    /// Naked-eye planet (Sun, Moon, Mercury...Saturn).
    Planet,
    /// Trans-Neptunian object (Pluto, Eris, Sedna).
    Tno,
    /// Minor body (asteroids, Chiron, Ceres).
    Minor,
    /// Fixed star.
    Star,
    /// Comet.
    Comet,
    /// Derived lunar point (Lilith, True/Mean Node).
    DerivedLunar,
    /// Hypothetical or calculated point.
    Hypothetical,
}

/// Unique identifier for a celestial body.
///
/// The BodyId contains physical properties only. No cultural fields.
#[derive(Debug, Clone, serde::Serialize)]
pub struct BodyId {
    name: String,
    pub se_id: Option<i32>,
    pub category: BodyCategory,
}

impl BodyId {
    /// Create a new BodyId. Fails if name is empty.
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

    /// The canonical name of this body.
    pub fn name(&self) -> &str {
        &self.name
    }
}

// ---------------------------------------------------------------------------
// Planet Model
// ---------------------------------------------------------------------------

/// Planet model for observer position.
///
/// Not hardcoded to Earth. Supports Mars, ISS, and orbital platforms.
#[derive(Debug, Clone, serde::Serialize)]
pub struct PlanetModel {
    /// The name of this planet (e.g., "Earth").
    pub name: &'static str,
    /// Equatorial radius [km].
    pub equatorial_radius_km: f64,
    /// Flattening factor (dimensionless).
    pub flattening: f64,
    /// Gravitational constant [km³/s²].
    pub gm_km3_s2: f64,
    /// Rotation rate [rad/s].
    pub rotation_rate_rad_s: f64,
}

/// WGS84 ellipsoid — Earth.
///
/// Source: NIMA TR8350.2, "Department of Defense World Geodetic System 1984".
/// GM source: NASA/GSFC EGM2008.
/// Rotation source: IERS Conventions 2010, eq. 1.5.
pub const EARTH: PlanetModel = PlanetModel {
    name: "Earth",
    equatorial_radius_km: 6378.137,
    flattening: 1.0 / 298.257223563,
    gm_km3_s2: 398600.4418,
    rotation_rate_rad_s: 7.2921159e-5,
};

// ---------------------------------------------------------------------------
// Physical Constants
// ---------------------------------------------------------------------------

/// Astronomical Unit [km].
/// Source: IAU 2012 Resolution B2.
pub const AU_KM: f64 = 149_597_870.700;

/// Speed of light [AU/day].
/// Source: IAU 2012 Resolution B2.
pub const C_AU_DAY: f64 = 173.144_632_684_869_3;

/// Mean obliquity of the ecliptic at J2000.0 [degrees].
/// 84381.406 arcsec / 3600 = 23.4392794°.
/// Source: IAU 2006 Resolution B1; Capitaine, Wallace & Chapront (2003).
pub const J2000_OBLIQUITY_DEG: f64 = 84381.406 / 3600.0;

/// Standard atmospheric refraction at the horizon [degrees].
/// 34.0 arcmin = 0.56667°.
/// Source: Saemundsson (1986).
pub const REFRACTION_DEG: f64 = 34.0 / 60.0;

/// Sun apparent semi-diameter at mean distance [degrees].
/// 16.0 arcmin.
/// Source: Meeus Ch.15.
pub const SUN_SEMIDIAM_DEG: f64 = 16.0 / 60.0;

/// Moon mean apparent semi-diameter [degrees].
/// 15.53 arcmin.
/// Source: Meeus Ch.53.
pub const MOON_SEMIDIAM_DEG: f64 = 15.53 / 60.0;

/// Moon mean horizontal parallax [degrees].
/// 57.042 arcmin.
/// Source: Meeus Ch.15.
pub const MOON_PARALLAX_DEG: f64 = 57.042 / 60.0;

/// Ratio of Moon mean radius to Earth mean radius.
/// R_moon = 1737.4 km (IAU 2009); R_earth = 6371.0 km.
/// Source: Meeus Ch.53.
pub const MOON_EARTH_RADIUS_RATIO: f64 = 0.2727;

/// Geometric altitude of Sun upper limb at apparent sunrise [degrees].
/// h₀ = -(ρ + SD) = -(34' + 16') = -50' = -0.8333°.
/// Source: Meeus Ch.15.
pub const SUN_H0: f64 = -(REFRACTION_DEG + SUN_SEMIDIAM_DEG);

/// Geometric altitude of Moon center at apparent moonrise [degrees].
/// h₀ = HP - ρ - SD.
/// Source: Meeus Ch.15.
pub const MOON_H0: f64 = MOON_PARALLAX_DEG - REFRACTION_DEG - MOON_SEMIDIAM_DEG;

/// Geometric altitude of a point source at apparent rise [degrees].
/// h₀ = -ρ = -34' = -0.5667°.
/// Source: Meeus Ch.15.
pub const STAR_H0: f64 = -REFRACTION_DEG;

// ---------------------------------------------------------------------------
// Raw Body
// ---------------------------------------------------------------------------

/// Model-independent astronomical body position.
///
/// Strictly physical observables. No cultural constructs.
/// A star is not a waypoint. A planet is not a destination.
#[derive(Debug, Clone, serde::Serialize)]
pub struct RawBody {
    /// Canonical body name.
    pub name: String,
    /// Ecliptic longitude [degrees].
    pub ecl_lon: f64,
    /// Ecliptic latitude [degrees].
    pub ecl_lat: f64,
    /// Angular speed [degrees/day].
    pub speed: f64,
    /// Geocentric distance [AU].
    pub distance_au: f64,
    /// Right ascension [degrees].
    pub ra: f64,
    /// Declination [degrees].
    pub dec: f64,
    /// Azimuth [degrees, North=0, East=90].
    pub az: f64,
    /// Altitude above horizon [degrees]. Negative if below.
    pub alt: f64,
    /// Heliocentric longitude [degrees], if available.
    pub helio_lon: Option<f64>,
    /// Heliocentric latitude [degrees], if available.
    pub helio_lat: Option<f64>,
    /// Heliocentric distance [AU], if available.
    pub helio_dist: Option<f64>,
    /// The coordinate frame of this position.
    pub frame: Frame,
    /// Position uncertainty. Every position carries one.
    pub uncertainty: PositionUncertainty,
    /// Epistemic state of this body's position.
    pub epistemic_state: EpistemicState,
}

// ---------------------------------------------------------------------------
// Precision Budget
// ---------------------------------------------------------------------------

/// Precision budget for an astronomical snapshot [arcseconds].
///
/// Tracks the contribution of each error source to the total
/// positional uncertainty.
#[derive(Debug, Clone, serde::Serialize)]
pub struct PrecisionBudget {
    /// Ephemeris model uncertainty [arcseconds].
    pub ephemeris_arcsec: f64,
    /// Time scale uncertainty (e.g., DUT1 staleness) [arcseconds].
    pub time_scale_arcsec: f64,
    /// Cache-induced uncertainty [arcseconds].
    pub cache_arcsec: f64,
    /// Numerical method uncertainty [arcseconds].
    pub numerical_arcsec: f64,
}

// ---------------------------------------------------------------------------
// Astronomical Snapshot
// ---------------------------------------------------------------------------

/// Model-independent astronomical state at a moment in time.
///
/// The central output of `compute_positions()`. Contains the physical
/// state of all bodies at a single `jd`.
///
/// All times use high-precision epochs internally.
/// Data flows strictly: jd → positions → snapshot → rendering.
/// jd is a parameter, not a timeline.
#[derive(Debug, Clone, serde::Serialize)]
pub struct AstronomicalSnapshot {
    /// Julian Day in UTC [days].
    pub jd_utc: f64,
    /// Observer geodetic latitude [degrees].
    pub lat: f64,
    /// Observer geodetic longitude [degrees].
    pub lon: f64,
    /// Observer altitude above ellipsoid [meters].
    pub alt: f64,
    /// All computed body positions.
    pub bodies: Vec<RawBody>,
    /// Obliquity of the ecliptic [degrees].
    pub obliquity: f64,
    /// Greenwich Apparent Sidereal Time [degrees].
    pub gast: f64,
    /// Obliquity (alias for API compatibility).
    pub epsilon: f64,
    /// Precision budget for this snapshot.
    pub budget: PrecisionBudget,
    /// Epistemic state of the snapshot.
    pub epistemic_state: EpistemicState,
    /// Relationships between bodies.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub relationships: Vec<Relationship>,
}
