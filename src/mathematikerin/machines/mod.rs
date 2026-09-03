pub(crate) use std::collections::HashMap;
pub(crate) use std::sync::atomic::{AtomicBool, Ordering};
pub(crate) use std::sync::mpsc;
pub(crate) use std::sync::{Arc, Mutex};
pub(crate) use std::thread;

pub(crate) use crate::archivar::{
    extract_series, fetch_raw, Extract, LeapSeconds, SourceConfig, Φ,
};

pub mod matrix;
pub mod solar;
#[cfg(test)]
mod tests;
pub mod verdict;

pub use matrix::*;
pub use solar::*;
pub(crate) use verdict::*;
