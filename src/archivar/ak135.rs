use std::sync::OnceLock;

const MODEL_RAW: &str = include_str!("kernels/ak135.dat");
const R_EARTH_KM: f64 = 6371.0;
const DR: f64 = 0.5;
const MAX_DELTA_DEG: f64 = 98.0;
const MAX_DEPTH_KM: f64 = 250.0;
const DEPTH_KM: [f64; 9] = [10.0, 15.0, 20.0, 35.0, 50.0, 100.0, 150.0, 200.0, 250.0];

struct Model {
    nodes: Vec<(f64, f64)>,
    grid: Vec<f64>,
}

fn parse_nodes() -> Vec<(f64, f64)> {
    let mut out = Vec::new();
    for line in MODEL_RAW.lines() {
        if line.starts_with('#') {
            continue;
        }
        let mut it = line.split_whitespace();
        let (Some(depth), Some(vp)) = (
            it.next().and_then(|s| s.parse::<f64>().ok()),
            it.next().and_then(|s| s.parse::<f64>().ok()),
        ) else {
            continue;
        };
        out.push((R_EARTH_KM - depth, vp));
    }
    out
}

fn vp_nodes(nodes: &[(f64, f64)], r: f64) -> f64 {
    if r <= nodes[nodes.len() - 1].0 {
        return nodes[nodes.len() - 1].1;
    }
    if r >= nodes[0].0 {
        return nodes[0].1;
    }
    for i in 0..nodes.len() - 1 {
        let (rt, vt) = nodes[i];
        let (rb, vb) = nodes[i + 1];
        if r <= rt && r >= rb {
            if rt == rb {
                return vb;
            }
            let t = (r - rb) / (rt - rb);
            return vb + (vt - vb) * t;
        }
    }
    nodes[0].1
}

fn build_grid(nodes: &[(f64, f64)]) -> Vec<f64> {
    let n = (R_EARTH_KM / DR).ceil() as usize + 1;
    (0..n)
        .map(|k| {
            let r = k as f64 * DR;
            r / vp_nodes(nodes, r)
        })
        .collect()
}

fn model() -> &'static Model {
    static M: OnceLock<Model> = OnceLock::new();
    M.get_or_init(|| {
        let nodes = parse_nodes();
        let grid = build_grid(&nodes);
        Model { nodes, grid }
    })
}

fn eta_grid(m: &Model, r: f64) -> f64 {
    if r <= 0.0 {
        return m.grid[0];
    }
    if r >= R_EARTH_KM {
        return m.grid[m.grid.len() - 1];
    }
    let idx = (r / DR).floor() as usize;
    let idx = idx.min(m.grid.len() - 2);
    let t = r / DR - idx as f64;
    m.grid[idx] + (m.grid[idx + 1] - m.grid[idx]) * t
}

fn turning_radius(m: &Model, p: f64) -> Option<f64> {
    for w in m.nodes.windows(2) {
        let (rt, vt) = w[0];
        let (rb, vb) = w[1];
        if rt <= rb {
            continue;
        }
        let eb = rb / vb;
        let et = rt / vt;
        let (lo, hi) = (eb.min(et), eb.max(et));
        if p > lo && p < hi {
            let mut a = rb;
            let mut b = rt;
            let ga = eb - p;
            for _ in 0..80 {
                let mid = 0.5 * (a + b);
                let t = (mid - rb) / (rt - rb);
                let v = vb + (vt - vb) * t;
                let gm = mid / v - p;
                if ga * gm > 0.0 {
                    a = mid;
                } else {
                    b = mid;
                }
            }
            return Some(0.5 * (a + b));
        }
    }
    None
}

fn integrate(m: &Model, p: f64, rp: f64, r_end: f64) -> (f64, f64) {
    let xmax = (r_end - rp).sqrt();
    let n = 1024;
    let dx = xmax / n as f64;
    let h = 0.01;
    let eta_exact = |r: f64| r / vp_nodes(&m.nodes, r);
    let eta_prime = (eta_exact(rp + h) - eta_exact(rp - h)) / (2.0 * h);
    let f0 = 2.0 / (rp * (2.0 * p * eta_prime).sqrt());
    let mut sum_d = 0.0;
    let mut sum_t = 0.0;
    for k in 0..=n {
        let x = k as f64 * dx;
        let r = rp + x * x;
        let e = eta_grid(m, r);
        let f = if k == 0 {
            f0
        } else {
            2.0 * x / (r * (e * e - p * p).sqrt())
        };
        let w = if k == 0 || k == n { 0.5 } else { 1.0 };
        sum_d += w * p * f;
        sum_t += w * e * e * f;
    }
    (sum_d * dx, sum_t * dx)
}

fn ray(m: &Model, p: f64, source_radius: f64) -> Vec<(f64, f64)> {
    let Some(rp) = turning_radius(m, p) else {
        return Vec::new();
    };
    if rp >= source_radius {
        return Vec::new();
    }
    let (ud, ut) = integrate(m, p, rp, R_EARTH_KM);
    let (dd, dt) = integrate(m, p, rp, source_radius);
    if source_radius >= R_EARTH_KM {
        vec![(2.0 * ud, 2.0 * ut)]
    } else {
        vec![(ud - dd, ut - dt), (ud + dd, ut + dt)]
    }
}

fn table() -> &'static Vec<(f64, f64)> {
    static T: OnceLock<Vec<(f64, f64)>> = OnceLock::new();
    T.get_or_init(|| {
        let m = model();
        let p_min = 255.0;
        let p_max = 1098.0;
        let n = 30000;
        let mut pts: Vec<(f64, f64)> = Vec::new();
        for k in 0..=n {
            let p = p_min + (p_max - p_min) * (k as f64 / n as f64);
            for (d, t) in ray(m, p, R_EARTH_KM) {
                pts.push((d.to_degrees(), t));
            }
        }
        pts.sort_by(|a, b| a.0.total_cmp(&b.0));
        pts
    })
}

fn build_depth_table(depth_km: f64) -> Vec<(f64, f64)> {
    let m = model();
    let rs = R_EARTH_KM - depth_km;
    let p_min = 255.0;
    let p_max = rs / vp_nodes(&m.nodes, rs);
    let n = 6000;
    let mut pts: Vec<(f64, f64)> = Vec::new();
    for k in 0..=n {
        let p = p_min + (p_max - p_min) * (k as f64 / n as f64);
        for (d, t) in ray(m, p, rs) {
            pts.push((d.to_degrees(), t));
        }
    }
    pts.sort_by(|a, b| a.0.total_cmp(&b.0));
    pts
}

fn depth_grid() -> &'static Vec<(f64, Vec<(f64, f64)>)> {
    static G: OnceLock<Vec<(f64, Vec<(f64, f64)>)>> = OnceLock::new();
    G.get_or_init(|| {
        let mut v = vec![(0.0f64, table().clone())];
        for &d in DEPTH_KM.iter() {
            v.push((d, build_depth_table(d)));
        }
        v
    })
}

fn interp_delta(table: &[(f64, f64)], delta_deg: f64) -> Option<f64> {
    if table.is_empty() {
        return None;
    }
    let first = table[0];
    let last = table[table.len() - 1];
    if delta_deg < first.0 || delta_deg > last.0 {
        return None;
    }
    if delta_deg <= first.0 {
        return Some(first.1);
    }
    if delta_deg >= last.0 {
        return Some(last.1);
    }
    let mut lo = 0usize;
    let mut hi = table.len() - 1;
    while hi - lo > 1 {
        let mid = (lo + hi) / 2;
        if table[mid].0 < delta_deg {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    let (d0, t0) = table[lo];
    let (d1, t1) = table[hi];
    let f = (delta_deg - d0) / (d1 - d0);
    Some(t0 + (t1 - t0) * f)
}

pub fn p_travel(delta_deg: f64) -> Option<f64> {
    if !delta_deg.is_finite() || delta_deg < 0.0 || delta_deg > MAX_DELTA_DEG {
        return None;
    }
    interp_delta(table(), delta_deg)
}

pub fn p_travel_depth(delta_deg: f64, depth_km: f64) -> Option<f64> {
    if !delta_deg.is_finite() || delta_deg < 0.0 || delta_deg > MAX_DELTA_DEG {
        return None;
    }
    if !depth_km.is_finite() || depth_km < 0.0 || depth_km > MAX_DEPTH_KM {
        return None;
    }
    let g = depth_grid();
    if depth_km <= g[0].0 {
        return interp_delta(&g[0].1, delta_deg);
    }
    for i in 0..g.len() - 1 {
        if depth_km >= g[i].0 && depth_km <= g[i + 1].0 {
            let t_lo = interp_delta(&g[i].1, delta_deg)?;
            let t_hi = interp_delta(&g[i + 1].1, delta_deg)?;
            let f = (depth_km - g[i].0) / (g[i + 1].0 - g[i].0);
            return Some(t_lo + (t_hi - t_lo) * f);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn surface_p_times_match_published_ak135() {
        let t30 = p_travel(30.0).unwrap();
        let t60 = p_travel(60.0).unwrap();
        let t90 = p_travel(90.0).unwrap();
        assert!((t30 - 370.27).abs() < 0.5, "T(30) = {t30}");
        assert!((t60 - 608.34).abs() < 0.5, "T(60) = {t60}");
        assert!((t90 - 781.40).abs() < 0.5, "T(90) = {t90}");
    }

    #[test]
    fn depth_times_match_published_ak135() {
        let cases = [
            (30.0, 367.98, 365.24, 363.82, 359.08, 354.39, 349.86, 345.50),
            (60.0, 605.93, 603.01, 601.39, 596.01, 590.67, 585.45, 580.37),
            (90.0, 778.89, 775.83, 774.08, 768.24, 762.43, 756.72, 751.15),
        ];
        let depths = [15.0, 35.0, 50.0, 100.0, 150.0, 200.0, 250.0];
        for (delta, t15, t35, t50, t100, t150, t200, t250) in cases {
            let expected = [t15, t35, t50, t100, t150, t200, t250];
            for (i, &d) in depths.iter().enumerate() {
                let t = p_travel_depth(delta, d).unwrap();
                assert!(
                    (t - expected[i]).abs() < 0.5,
                    "T({delta}, {d} km) = {t} vs {}",
                    expected[i]
                );
            }
        }
    }

    #[test]
    fn travel_time_is_monotone() {
        let mut prev = 0.0;
        for d in [2.0, 5.0, 20.0, 40.0, 70.0, 90.0] {
            let t = p_travel(d).unwrap();
            assert!(t > prev, "T({d}) = {t} not above {prev}");
            prev = t;
        }
    }

    #[test]
    fn a_deep_source_shortens_the_arrival() {
        let t0 = p_travel(60.0).unwrap();
        let t100 = p_travel_depth(60.0, 100.0).unwrap();
        assert!(t100 < t0, "T(60,100)={t100} not below T(60,0)={t0}");
    }

    #[test]
    fn constant_velocity_matches_chord() {
        let v = 10.0;
        let nodes = vec![(R_EARTH_KM, v), (0.0, v)];
        let grid = build_grid(&nodes);
        let m = Model { nodes, grid };
        for p in [100.0, 300.0, 500.0, 620.0] {
            let pts = ray(&m, p, R_EARTH_KM);
            assert!(!pts.is_empty(), "p={p} carries no turning ray");
            let (d, t) = pts[0];
            let delta_deg = d.to_degrees();
            let chord = 2.0 * R_EARTH_KM * (delta_deg.to_radians() / 2.0).sin();
            let t_exact = chord / v;
            assert!(
                (t - t_exact).abs() < 0.01,
                "p={p}: T={t} vs chord/v={t_exact}"
            );
        }
    }

    #[test]
    fn out_of_range_carries_no_value() {
        assert!(p_travel(-1.0).is_none());
        assert!(p_travel(99.0).is_none());
        assert!(p_travel(f64::NAN).is_none());
        assert!(p_travel_depth(60.0, -1.0).is_none());
        assert!(p_travel_depth(60.0, 300.0).is_none());
    }
}
