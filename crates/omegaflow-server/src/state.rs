pub use crate::ephemerides::{self, masses_at, almanac, Mass};
pub use crate::terrain::{self as terrain_mod, terrain_height};
pub use crate::magnetic::{self as magnetic_mod, wmm_at, WmmData};

pub fn init() {
    ephemerides::init();
    terrain_mod::init();
}

