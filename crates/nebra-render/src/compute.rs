use glam::DVec3;
use rayon::prelude::*;

pub fn field(jd: f64, center: DVec3, scale: f64, rx: usize, ry: usize) -> Vec<f32> {
    let masses = nebra_core::masses_at(jd);
    let half_x = (rx as f64) * scale * 0.5;
    let half_y = (ry as f64) * scale * 0.5;

    (0..rx * ry)
        .into_par_iter()
        .flat_map(|idx| {
            let px = idx % rx;
            let py = idx / rx;
            let x = center.x - half_x + (px as f64) * scale;
            let y = center.y + half_y - (py as f64) * scale;
            let pos = DVec3::new(x, y, center.z);
            let mut omega = 0.0_f64;
            let mut flow = DVec3::ZERO;
            for m in &masses {
                let delta = m.pos - pos;
                let dist = delta.length();
                if dist < 1.0 { continue; }
                let g = m.gm / (dist * dist);
                omega += g;
                flow += delta.normalize() * g;
            }
            [omega as f32, flow.x as f32, flow.y as f32, flow.z as f32]
        })
        .collect()
}
