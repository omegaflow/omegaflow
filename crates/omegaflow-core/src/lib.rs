pub mod ephemerides;
#[cfg(not(target_arch = "wasm32"))]
pub mod magnetic;
#[cfg(not(target_arch = "wasm32"))]
pub mod terrain;

pub use ephemerides::Mass;
pub use ephemerides::almanac;
pub use ephemerides::init;
pub use ephemerides::init_from_bytes;
pub use ephemerides::masses_at;
#[cfg(not(target_arch = "wasm32"))]
pub use magnetic::WmmData;
#[cfg(not(target_arch = "wasm32"))]
pub use magnetic::wmm_at;
#[cfg(not(target_arch = "wasm32"))]
pub use terrain::raw_egm96;
#[cfg(not(target_arch = "wasm32"))]
pub use terrain::raw_hgt_tile;

