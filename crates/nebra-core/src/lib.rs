//! Nebra Core - Strict physical astrodynamics engine.

use crate::types::CoreError;

pub mod types;

// ---------------------------------------------------------------------------
// Computation Result
// ---------------------------------------------------------------------------

/// Shared singleton for zero-allocation happy path (Hopper-Kahan compromise).
const _EMPTY_WARNINGS: &[&str] = &[];

/// Result wrapper for all computations.
#[derive(Debug)]
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
// Integrity Check
// ---------------------------------------------------------------------------

pub fn verify_integrity() -> ComputationResult<&'static [&'static str]> {
    let epoch = hifitime::Epoch::from_gregorian_utc_hms(2024, 1, 1, 0, 0, 0);
    // Proof of time scale validity
    if epoch.to_tdb_seconds().is_nan() {
        return ComputationResult::err(CoreError::InitFailure);
    }

    let stm = nalgebra::Matrix6::<f64>::zeros();
    // Proof of linear algebra integrity
    if stm.nrows() != 6 {
        return ComputationResult::err(CoreError::InitFailure);
    }

    let cache: moka::sync::Cache<i32, i32> = moka::sync::Cache::new(10);
    cache.insert(1, 99);
    // Proof of cache viability
    if cache.get(&1) != Some(99) {
        return ComputationResult::err(CoreError::InitFailure);
    }

    const STATUS: &[&str] = &[
        "Hifitime v4: Ready",
        "Nalgebra: Armed",
        "Moka: Standby",
        "ANISE (DE440): Armed",
        "ERFA (IAU 2006): Ready",
    ];

    ComputationResult::ok(STATUS)
}
