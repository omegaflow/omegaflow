use anise::constants::frames::SSB_J2000;
use anise::prelude::*;
use glam::DVec3;
use hifitime::Epoch;
use std::sync::OnceLock;
use bytes::BytesMut;

static ALMANAC: OnceLock<Almanac> = OnceLock::new();
static MASS_IDS: OnceLock<Vec<i32>> = OnceLock::new();

pub struct Mass { pub pos: DVec3, pub gm: f64 }

pub fn init() {
    let alm = Almanac::new("data/de440s.bsp").and_then(|a| a.load("data/pck08.pca"));
    if let Ok(alm) = alm {
        let ids: Vec<i32> = (0..1000).filter(|&id| { let frame = Frame::from_ephem_j2000(id); alm.frame_info(frame).ok().and_then(|f| f.mu_km3_s2).is_some() }).collect();
        let _ = MASS_IDS.set(ids);
        let _ = ALMANAC.set(alm);
    }
}

pub fn init_from_bytes(de440s: &[u8], pck08: &[u8]) -> bool {
    let alm = Almanac::default()
        .load_from_bytes(BytesMut::from(de440s))
        .and_then(|a| a.load_from_bytes(BytesMut::from(pck08)));
    match alm {
        Ok(alm) => {
            let ids: Vec<i32> = (0..1000).filter(|&id| { let frame = Frame::from_ephem_j2000(id); alm.frame_info(frame).ok().and_then(|f| f.mu_km3_s2).is_some() }).collect();
            let _ = MASS_IDS.set(ids);
            let _ = ALMANAC.set(alm);
            true
        }
        Err(_) => false
    }
}

pub fn almanac() -> Option<&'static Almanac> { ALMANAC.get() }

pub fn masses_at(t: f64, cx: f64, cy: f64, cz: f64, scale: f64) -> Vec<Mass> {
    let Some(alm) = ALMANAC.get() else { return Vec::new() };
    let Some(ids) = MASS_IDS.get() else { return Vec::new() };
    let epoch = Epoch::from_tdb_seconds(t);
    let viewport_center = DVec3::new(cx, cy, cz);
    let mut out = Vec::new();
    for &id in ids {
        let frame = Frame::from_ephem_j2000(id);
        let gm = match alm.frame_info(frame).ok().and_then(|f| f.mu_km3_s2) { Some(gm) => gm * 1e9, None => continue };
        let Ok(state) = alm.translate(frame, SSB_J2000, epoch, None) else { continue };
        let pos = DVec3::new(state.radius_km.x * 1e3, state.radius_km.y * 1e3, state.radius_km.z * 1e3);
        let dist = (pos - viewport_center).length();
        let influence_radius = scale * 10.0; 
        if dist > influence_radius && id != 10 { continue; } 
        out.push(Mass { pos, gm });
    }
    out
}

