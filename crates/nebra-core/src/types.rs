//! Core types and enumerations.

use thiserror::Error;

// ---------------------------------------------------------------------------
// Error Codes
// ---------------------------------------------------------------------------

/// Strict error codes for ComputationResult. No free-form strings.
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
}
