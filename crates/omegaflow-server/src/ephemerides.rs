use glam::DVec3;
use std::sync::OnceLock;
use anise::prelude::*;
use anise::constants::frames::SSB_J2000;
use hifitime::Epoch;

static ALMANAC: OnceLock<Almanac> = OnceLock::new();
static MASS_IDS: OnceLock<Vec<i32>> = OnceLock::new();

pub struct Mass {
    pub pos: DVec3,
    pub gm: f64,
}

pub fn init() {
    let alm = Almanac::new("data/de440s.bsp")
        .and_then(|a| a.load("data/pck08.pca"));
    if let Ok(alm) = alm {
        let ids: Vec<i32> = (0..1000)
            .filter(|&id| {
                let frame = Frame::from_ephem_j2000(id);
                alm.frame_info(frame).ok().and_then(|f| f.mu_km3_s2).is_some()
            })
            .collect();
        let _ = MASS_IDS.set(ids);
        let _ = ALMANAC.set(alm);
    }
}

pub fn almanac() -> Option<&'static Almanac> {
    ALMANAC.get()
}

pub fn masses_at(t: f64, cx: f64, cy: f64, cz: f64, scale: f64, observer_tier: i32) -> Vec<Mass> {
    let Some(alm) = ALMANAC.get() else { return Vec::new() };
    let Some(ids) = MASS_IDS.get() else { return Vec::new() };
    let epoch = Epoch::from_tdb_seconds(t);
    let viewport_center = DVec3::new(cx, cy, cz);
    let mut out = Vec::new();
    for &id in ids {
        let frame = Frame::from_ephem_j2000(id);
        let gm = match alm.frame_info(frame).ok().and_then(|f| f.mu_km3_s2) {
            Some(gm) => gm * 1e9,
            None => continue,
        };
        let Ok(state) = alm.translate(frame, SSB_J2000, epoch, None) else { continue };
        let pos = DVec3::new(state.radius_km.x * 1e3, state.radius_km.y * 1e3, state.radius_km.z * 1e3);
        
        let dist = (pos - viewport_center).length();
        let influence_radius = scale * 10.0; 
        if dist > influence_radius && id != 10 { continue; } 
        
        out.push(Mass { pos, gm });
    }
    out.sort_by(|a, b| b.gm.partial_cmp(&a.gm).unwrap_or(std::cmp::Ordering::Equal));
    
    let max_masses = if observer_tier > 0 { 15 } else { 5 };
    out.truncate(max_masses);
    
    out
}
