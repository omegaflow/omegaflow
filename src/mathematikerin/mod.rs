#[cfg(all(not(target_arch = "wasm32"), feature = "membrane"))]
pub(crate) use crate::archivar::{Buffer, CurveSet, PARSEC_M, SampleRecord};
#[cfg(all(not(target_arch = "wasm32"), feature = "membrane"))]
pub(crate) use crate::archivar::{LeapSeconds, MembraneCtx, Radiator, sense_membrane, system_now};
#[cfg(all(not(target_arch = "wasm32"), feature = "membrane"))]
pub(crate) use crate::machines::{
    MatrixMachine, SolarCell, SolarMachine, TE_KSG_K_PROD, TE_SERIES_BYTES, TE_SERIES_STRIDE,
    le_bytes_f32, te_absence_word, te_read_verdict, te_verdict_bytes,
};

#[cfg(all(not(target_arch = "wasm32"), feature = "membrane"))]
pub mod actuators;
pub mod channel;
pub mod dispersion;
pub mod doppler;
pub mod equilibrium;
pub mod force;
pub mod healpix;
pub mod ksg_k;
pub mod least_squares;
#[cfg(all(not(target_arch = "wasm32"), feature = "membrane"))]
pub mod machines;
pub mod mat;
pub mod mci;
pub mod media;
pub mod newell;
#[cfg(all(not(target_arch = "wasm32"), feature = "membrane"))]
pub mod omega;
pub mod orientation;
pub mod ozzy;
pub mod parcorr;
pub mod pc;
pub mod receiver;
pub mod s2;
#[cfg(all(not(target_arch = "wasm32"), feature = "membrane"))]
pub mod scalar_te_gpu;
pub mod shaders;
pub mod te;
#[cfg(test)]
mod tests;
pub mod tropo;
pub mod wy_max_t;

#[cfg(all(not(target_arch = "wasm32"), feature = "membrane"))]
pub use actuators::*;
#[cfg(all(not(target_arch = "wasm32"), feature = "membrane"))]
pub use omega::*;
pub use orientation::*;
pub use s2::*;
#[cfg(all(not(target_arch = "wasm32"), feature = "membrane"))]
pub use scalar_te_gpu::*;
pub use shaders::*;

#[cfg(all(not(target_arch = "wasm32"), feature = "membrane"))]
pub(crate) use crate::force::kernel_id_for_force;
#[cfg(all(not(target_arch = "wasm32"), feature = "membrane"))]
pub(crate) use std::sync::atomic::{AtomicBool, Ordering};
#[cfg(all(not(target_arch = "wasm32"), feature = "membrane"))]
pub(crate) use std::sync::{Arc, mpsc};
#[cfg(all(not(target_arch = "wasm32"), feature = "membrane"))]
pub(crate) use std::sync::{Mutex, RwLock};
#[cfg(all(not(target_arch = "wasm32"), feature = "membrane"))]
pub(crate) use std::thread;
