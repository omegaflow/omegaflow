//! Die Ernte-Maschinen: Solar + ENSO. Jede Maschine erntet ihre Serien
//! über die vorhandenen Archivar-Fetch-Pfade und sendet Zellen über einen
//! mpsc-Kanal an die Mathematikerin — ein Kanal je Maschine, kein
//! Sonderpfad im Kern.
pub(crate) use std::collections::{HashMap, HashSet};
pub(crate) use std::sync::atomic::{AtomicBool, Ordering};
pub(crate) use std::sync::mpsc;
pub(crate) use std::sync::{Arc, Mutex};
pub(crate) use std::thread;

pub(crate) use crate::archivar::{
    extract_series, fetch_raw, fetch_raw_bytes, Extract, LeapSeconds, SourceConfig, Φ,
};

pub mod enso;
pub mod solar;
#[cfg(test)]
mod tests;
pub mod verdict;

pub use enso::*;
pub use solar::*;
pub(crate) use verdict::*;
