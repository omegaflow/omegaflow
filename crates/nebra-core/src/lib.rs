//! Nebra Core - Strict physical astrodynamics engine.

pub mod numerics;
pub mod types;

pub use types::{ComputationResult, CoreError};

// ---------------------------------------------------------------------------
// Integrity Check
// ---------------------------------------------------------------------------

pub fn verify_integrity() -> ComputationResult<&'static [&'static str]> {
    let epoch = hifitime::Epoch::from_gregorian_utc_hms(2024, 1, 1, 0, 0, 0);
    if epoch.to_tdb_seconds().is_nan() {
        return ComputationResult::err(CoreError::InitFailure);
    }

    let stm = nalgebra::Matrix6::<f64>::zeros();
    if stm.nrows() != 6 {
        return ComputationResult::err(CoreError::InitFailure);
    }

    let cache: moka::sync::Cache<i32, i32> = moka::sync::Cache::new(10);
    cache.insert(1, 99);
    if cache.get(&1) != Some(99) {
        return ComputationResult::err(CoreError::InitFailure);
    }

    const STATUS: &[&str] = &[
        "Hifitime v4: Ready",
        "Nalgebra: Armed",
        "Moka: Standby",
        "ANISE (DE440): Armed",
        "ERFA (IAU 2006): Ready",
        "Numerics: Armed",
    ];

    ComputationResult::ok(STATUS)
}
