use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn jd_now() -> f64 {
    js_sys::Date::now() / 86400000.0 + 2440587.5
}
