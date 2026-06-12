use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn init(de440s: &[u8], pck08: &[u8]) -> bool {
    omegaflow_core::init_from_bytes(de440s, pck08)
}

#[wasm_bindgen]
pub fn masses_at(t: f64, cx: f64, cy: f64, cz: f64, scale: f64) -> Vec<f32> {
    let masses = omegaflow_core::masses_at(t, cx, cy, cz, scale);
    masses.iter().flat_map(|m| {
        [m.pos.x as f32, m.pos.y as f32, m.pos.z as f32, m.gm as f32]
    }).collect()
}

#[wasm_bindgen]
pub fn wmm_at(t: f64) -> Vec<f32> {
    let Some(alm) = omegaflow_core::almanac() else { return Vec::new() };
    let Some(data) = omegaflow_core::wmm_at(t, alm) else { return Vec::new() };
    let n_max = data.n_max;
    let wmm_coeffs = (n_max * (n_max + 3)) / 2;
    let mut out = vec![data.earth_pos.x as f32, data.earth_pos.y as f32, data.earth_pos.z as f32, data.time_delta, n_max as f32];
    for i in 0..wmm_coeffs as usize {
        out.push(*data.g_mfc.get(i).unwrap_or(&0.0));
        out.push(*data.h_mfc.get(i).unwrap_or(&0.0));
        out.push(*data.g_svc.get(i).unwrap_or(&0.0));
        out.push(*data.h_svc.get(i).unwrap_or(&0.0));
    }
    out
}

#[wasm_bindgen]
pub fn jd_now() -> f64 {
    js_sys::Date::now() / 86400000.0 + 2440587.5
}
