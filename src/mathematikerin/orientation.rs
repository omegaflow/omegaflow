use super::*;

pub fn q_mul(a: [f64; 4], b: [f64; 4]) -> [f64; 4] {
    [
        a[0] * b[0] - a[1] * b[1] - a[2] * b[2] - a[3] * b[3],
        a[0] * b[1] + a[1] * b[0] + a[2] * b[3] - a[3] * b[2],
        a[0] * b[2] - a[1] * b[3] + a[2] * b[0] + a[3] * b[1],
        a[0] * b[3] + a[1] * b[2] - a[2] * b[1] + a[3] * b[0],
    ]
}


pub fn q_norm(q: [f64; 4]) -> [f64; 4] {
    let n = (q[0] * q[0] + q[1] * q[1] + q[2] * q[2] + q[3] * q[3])
        .sqrt()
        .max(1e-12);
    [q[0] / n, q[1] / n, q[2] / n, q[3] / n]
}


pub fn q_rotate(q: [f64; 4], v: [f64; 3]) -> [f64; 3] {
    let p = [0.0, v[0], v[1], v[2]];
    let c = [q[0], -q[1], -q[2], -q[3]];
    let r = q_mul(q_mul(q, p), c);
    [r[1], r[2], r[3]]
}


pub fn q_axis_angle(axis: [f64; 3], angle: f64) -> [f64; 4] {
    let s = (angle / 2.0).sin();
    [(angle / 2.0).cos(), axis[0] * s, axis[1] * s, axis[2] * s]
}


pub const WINDOW_STATE_PATH: &str = "/tmp/omegaflow_window_state.φ";


pub fn window_state_load(path: &str) -> (f64, [f64; 3], [f64; 4]) {
    let mut grid = GRID_INIT;
    let mut p = [0.0f64; 3];
    let mut q = [1.0, 0.0, 0.0, 0.0];
    let Ok(text) = std::fs::read_to_string(path) else {
        return (grid, p, q);
    };
    for line in text.lines() {
        let mut toks = line.split_whitespace();
        match toks.next() {
            Some("grid_step") => {
                if let Some(v) = toks.next().and_then(|s| s.parse::<f64>().ok()) {
                    if v.is_finite() && v > 0.0 {
                        grid = v;
                    }
                }
            }
            Some("p") => {
                let vals: Option<Vec<f64>> = toks.map(|s| s.parse::<f64>().ok()).collect();
                if let Some(vals) = vals {
                    if vals.len() == 3 && vals.iter().all(|v| v.is_finite()) {
                        p = [vals[0], vals[1], vals[2]];
                    }
                }
            }
            Some("q") => {
                let vals: Option<Vec<f64>> = toks.map(|s| s.parse::<f64>().ok()).collect();
                if let Some(vals) = vals {
                    if vals.len() == 4
                        && vals.iter().all(|v| v.is_finite())
                        && vals.iter().any(|v| *v != 0.0)
                    {
                        q = q_norm([vals[0], vals[1], vals[2], vals[3]]);
                    }
                }
            }
            _ => {}
        }
    }
    (grid, p, q)
}


pub fn window_state_save(path: &str, grid_step: f64, p: [f64; 3], q: [f64; 4]) {
    let mut text = String::new();
    text.push_str(&format!("grid_step {:.17e}\n", grid_step));
    text.push_str(&format!("p {:.17e} {:.17e} {:.17e}\n", p[0], p[1], p[2]));
    text.push_str(&format!(
        "q {:.17e} {:.17e} {:.17e} {:.17e}\n",
        q[0], q[1], q[2], q[3]
    ));
    let _ = std::fs::write(path, text);
}
