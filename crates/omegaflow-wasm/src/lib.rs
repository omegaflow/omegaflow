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
pub fn jd_now() -> f64 {
    js_sys::Date::now() / 86400000.0 + 2440587.5
}
