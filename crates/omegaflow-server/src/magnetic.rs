use glam::DVec3;
use anise::prelude::*;
use anise::constants::frames::SSB_J2000;
use hifitime::Epoch;
use world_magnetic_model::wmm_models::select_models;
use world_magnetic_model::time::Date;

pub struct WmmData {
    pub earth_pos: DVec3,
    pub time_delta: f32,
    pub n_max: i32,
    pub g_mfc: Vec<f32>,
    pub h_mfc: Vec<f32>,
    pub g_svc: Vec<f32>,
    pub h_svc: Vec<f32>,
}

pub fn wmm_at(t: f64, alm: &Almanac) -> Option<WmmData> {
    let epoch = Epoch::from_tdb_seconds(t);
    let year = epoch.year();
    let day_of_year = epoch.day_of_year() as u16;
    let date = Date::from_ordinal_date(year, day_of_year).ok()?;
    
    let (model, _error_model) = select_models(date).ok()?;
    
    let earth_frame = Frame::from_ephem_j2000(3);
    let state = alm.translate(earth_frame, SSB_J2000, epoch, None).ok()?;
    let earth_pos = DVec3::new(state.radius_km.x * 1e3, state.radius_km.y * 1e3, state.radius_km.z * 1e3);
    
    let n_max = (((8 * model.g_mfc.len() as i32 + 9) as f64).sqrt() as i32 - 3) / 2;
    let g_mfc: Vec<f32> = model.g_mfc.iter().map(|&x| x as f32).collect();
    let h_mfc: Vec<f32> = model.h_mfc.iter().map(|&x| x as f32).collect();
    let g_svc: Vec<f32> = model.g_svc.iter().map(|&x| x as f32).collect();
    let h_svc: Vec<f32> = model.h_svc.iter().map(|&x| x as f32).collect();
    
    let time_delta = ((year - 2020) as f32) + (day_of_year as f32 / 365.0);
    
    Some(WmmData {
        earth_pos,
        time_delta,
        n_max,
        g_mfc,
        h_mfc,
        g_svc,
        h_svc,
    })
}
