use std::sync::OnceLock;

const MODEL_RAW: &str = include_str!("kernels/ak135.dat");
const R_EARTH_KM: f64 = 6371.0;
const DR: f64 = 0.5;
const MAX_DELTA_DEG: f64 = 98.0;
const MAX_DEPTH_KM: f64 = 250.0;
const DEPTH_KM: [f64; 9] = [10.0, 15.0, 20.0, 35.0, 50.0, 100.0, 150.0, 200.0, 250.0];
const P_SLOWNESS_CORE_GRAZE: f64 = 255.0;
const S_SLOWNESS_CORE_GRAZE: f64 = 478.0;

struct Model {
    vp: Vec<(f64, f64)>,
    vs: Vec<(f64, f64)>,
    grid: Vec<f64>,
    sgrid: Vec<f64>,
}

fn parse_nodes() -> (Vec<(f64, f64)>, Vec<(f64, f64)>) {
    let mut vp = Vec::new();
    let mut vs = Vec::new();
    for line in MODEL_RAW.lines() {
        if line.starts_with('#') {
            continue;
        }
        let mut it = line.split_whitespace();
        let (Some(depth), Some(v), Some(s)) = (
            it.next().and_then(|t| t.parse::<f64>().ok()),
            it.next().and_then(|t| t.parse::<f64>().ok()),
            it.next().and_then(|t| t.parse::<f64>().ok()),
        ) else {
            continue;
        };
        let r = R_EARTH_KM - depth;
        vp.push((r, v));
        if s <= 0.0 {
            break;
        }
        vs.push((r, s));
    }
    (vp, vs)
}

fn vel_nodes(nodes: &[(f64, f64)], r: f64) -> f64 {
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
            r / vel_nodes(nodes, r)
        })
        .collect()
}

fn model() -> &'static Model {
    static M: OnceLock<Model> = OnceLock::new();
    M.get_or_init(|| {
        let (vp, vs) = parse_nodes();
        let grid = build_grid(&vp);
        let sgrid = build_grid(&vs);
        Model {
            vp,
            vs,
            grid,
            sgrid,
        }
    })
}

fn eta_grid(grid: &[f64], r: f64) -> f64 {
    if r <= 0.0 {
        return grid[0];
    }
    if r >= R_EARTH_KM {
        return grid[grid.len() - 1];
    }
    let idx = (r / DR).floor() as usize;
    let idx = idx.min(grid.len() - 2);
    let t = r / DR - idx as f64;
    grid[idx] + (grid[idx + 1] - grid[idx]) * t
}

fn turning_radius(nodes: &[(f64, f64)], p: f64) -> Option<f64> {
    for w in nodes.windows(2) {
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

fn integrate(nodes: &[(f64, f64)], grid: &[f64], p: f64, rp: f64, r_end: f64) -> (f64, f64) {
    let xmax = (r_end - rp).sqrt();
    let n = 1024;
    let dx = xmax / n as f64;
    let h = 0.01;
    let eta_exact = |r: f64| r / vel_nodes(nodes, r);
    let eta_prime = (eta_exact(rp + h) - eta_exact(rp - h)) / (2.0 * h);
    let f0 = 2.0 / (rp * (2.0 * p * eta_prime).sqrt());
    let mut sum_d = 0.0;
    let mut sum_t = 0.0;
    for k in 0..=n {
        let x = k as f64 * dx;
        let r = rp + x * x;
        let e = eta_grid(grid, r);
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

fn ray(nodes: &[(f64, f64)], grid: &[f64], p: f64, source_radius: f64) -> Vec<(f64, f64)> {
    let Some(rp) = turning_radius(nodes, p) else {
        return Vec::new();
    };
    if rp >= source_radius {
        return Vec::new();
    }
    let (ud, ut) = integrate(nodes, grid, p, rp, R_EARTH_KM);
    let (dd, dt) = integrate(nodes, grid, p, rp, source_radius);
    if source_radius >= R_EARTH_KM {
        vec![(2.0 * ud, 2.0 * ut)]
    } else {
        vec![(ud - dd, ut - dt), (ud + dd, ut + dt)]
    }
}

fn up_leg(nodes: &[(f64, f64)], grid: &[f64], p: f64, rs: f64) -> Option<(f64, f64)> {
    if rs >= R_EARTH_KM {
        return Some((0.0, 0.0));
    }
    if p >= rs / vel_nodes(nodes, rs) {
        return None;
    }
    let xmax = (R_EARTH_KM - rs).sqrt();
    let n = 512;
    let dx = xmax / n as f64;
    let mut sum_d = 0.0;
    let mut sum_t = 0.0;
    for k in 0..=n {
        let x = k as f64 * dx;
        let r = rs + x * x;
        let e = eta_grid(grid, r);
        let sq = (e * e - p * p).sqrt();
        let w = if k == 0 || k == n { 0.5 } else { 1.0 };
        sum_d += w * 2.0 * x * p / (r * sq);
        sum_t += w * 2.0 * x * e * e / (r * sq);
    }
    Some((sum_d * dx, sum_t * dx))
}

fn surface_leg(nodes: &[(f64, f64)], grid: &[f64], p: f64) -> Option<(f64, f64)> {
    ray(nodes, grid, p, R_EARTH_KM).into_iter().next()
}

fn table() -> &'static Vec<(f64, f64)> {
    static T: OnceLock<Vec<(f64, f64)>> = OnceLock::new();
    T.get_or_init(|| {
        let m = model();
        let p_min = P_SLOWNESS_CORE_GRAZE;
        let p_max = R_EARTH_KM / vel_nodes(&m.vp, R_EARTH_KM);
        let n = 30000;
        let mut pts: Vec<(f64, f64)> = Vec::new();
        for k in 0..=n {
            let p = p_min + (p_max - p_min) * (k as f64 / n as f64);
            for (d, t) in ray(&m.vp, &m.grid, p, R_EARTH_KM) {
                pts.push((d.to_degrees(), t));
            }
        }
        pts.sort_by(|a, b| a.0.total_cmp(&b.0));
        pts
    })
}

fn s_table() -> &'static Vec<(f64, f64)> {
    static T: OnceLock<Vec<(f64, f64)>> = OnceLock::new();
    T.get_or_init(|| {
        let m = model();
        let p_min = S_SLOWNESS_CORE_GRAZE;
        let p_max = R_EARTH_KM / vel_nodes(&m.vs, R_EARTH_KM);
        let n = 30000;
        let mut pts: Vec<(f64, f64)> = Vec::new();
        for k in 0..=n {
            let p = p_min + (p_max - p_min) * (k as f64 / n as f64);
            for (d, t) in ray(&m.vs, &m.sgrid, p, R_EARTH_KM) {
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
    let p_min = P_SLOWNESS_CORE_GRAZE;
    let p_max = rs / vel_nodes(&m.vp, rs);
    let n = 6000;
    let mut pts: Vec<(f64, f64)> = Vec::new();
    for k in 0..=n {
        let p = p_min + (p_max - p_min) * (k as f64 / n as f64);
        for (d, t) in ray(&m.vp, &m.grid, p, rs) {
            pts.push((d.to_degrees(), t));
        }
    }
    pts.sort_by(|a, b| a.0.total_cmp(&b.0));
    pts
}

fn build_p_p_table(depth_km: f64) -> Vec<(f64, f64)> {
    let m = model();
    let rs = R_EARTH_KM - depth_km;
    let p_min = P_SLOWNESS_CORE_GRAZE;
    let p_max = (rs / vel_nodes(&m.vp, rs)).min(R_EARTH_KM / vel_nodes(&m.vp, R_EARTH_KM));
    let n = 6000;
    let mut pts: Vec<(f64, f64)> = Vec::new();
    for k in 0..=n {
        let p = p_min + (p_max - p_min) * (k as f64 / n as f64);
        let Some((du, tu)) = up_leg(&m.vp, &m.grid, p, rs) else {
            continue;
        };
        let Some((dd, td)) = surface_leg(&m.vp, &m.grid, p) else {
            continue;
        };
        pts.push(((du + dd).to_degrees(), tu + td));
    }
    pts.sort_by(|a, b| a.0.total_cmp(&b.0));
    pts
}

fn build_s_p_table(depth_km: f64) -> Vec<(f64, f64)> {
    let m = model();
    let rs = R_EARTH_KM - depth_km;
    let p_min = P_SLOWNESS_CORE_GRAZE;
    let p_max = (rs / vel_nodes(&m.vs, rs)).min(R_EARTH_KM / vel_nodes(&m.vp, R_EARTH_KM));
    let n = 6000;
    let mut pts: Vec<(f64, f64)> = Vec::new();
    for k in 0..=n {
        let p = p_min + (p_max - p_min) * (k as f64 / n as f64);
        let Some((du, tu)) = up_leg(&m.vs, &m.sgrid, p, rs) else {
            continue;
        };
        let Some((dd, td)) = surface_leg(&m.vp, &m.grid, p) else {
            continue;
        };
        pts.push(((du + dd).to_degrees(), tu + td));
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

fn p_p_grid() -> &'static Vec<(f64, Vec<(f64, f64)>)> {
    static G: OnceLock<Vec<(f64, Vec<(f64, f64)>)>> = OnceLock::new();
    G.get_or_init(|| {
        let mut v = vec![(0.0f64, table().clone())];
        for &d in DEPTH_KM.iter() {
            v.push((d, build_p_p_table(d)));
        }
        v
    })
}

fn s_p_grid() -> &'static Vec<(f64, Vec<(f64, f64)>)> {
    static G: OnceLock<Vec<(f64, Vec<(f64, f64)>)>> = OnceLock::new();
    G.get_or_init(|| {
        let mut v = vec![(0.0f64, table().clone())];
        for &d in DEPTH_KM.iter() {
            v.push((d, build_s_p_table(d)));
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

fn interp_depth_grid(
    grid: &[(f64, Vec<(f64, f64)>)],
    delta_deg: f64,
    depth_km: f64,
) -> Option<f64> {
    if depth_km <= grid[0].0 {
        return interp_delta(&grid[0].1, delta_deg);
    }
    for i in 0..grid.len() - 1 {
        if depth_km >= grid[i].0 && depth_km <= grid[i + 1].0 {
            let t_lo = interp_delta(&grid[i].1, delta_deg)?;
            let t_hi = interp_delta(&grid[i + 1].1, delta_deg)?;
            let f = (depth_km - grid[i].0) / (grid[i + 1].0 - grid[i].0);
            return Some(t_lo + (t_hi - t_lo) * f);
        }
    }
    None
}

pub fn p_travel(delta_deg: f64) -> Option<f64> {
    if !delta_deg.is_finite() || delta_deg < 0.0 || delta_deg > MAX_DELTA_DEG {
        return None;
    }
    interp_delta(table(), delta_deg)
}

pub fn s_travel(delta_deg: f64) -> Option<f64> {
    if !delta_deg.is_finite() || delta_deg < 0.0 || delta_deg > MAX_DELTA_DEG {
        return None;
    }
    interp_delta(s_table(), delta_deg)
}

pub fn p_travel_depth(delta_deg: f64, depth_km: f64) -> Option<f64> {
    if !delta_deg.is_finite() || delta_deg < 0.0 || delta_deg > MAX_DELTA_DEG {
        return None;
    }
    if !depth_km.is_finite() || depth_km < 0.0 || depth_km > MAX_DEPTH_KM {
        return None;
    }
    interp_depth_grid(depth_grid(), delta_deg, depth_km)
}

pub fn p_p_travel(delta_deg: f64, depth_km: f64) -> Option<f64> {
    if !delta_deg.is_finite() || delta_deg < 0.0 || delta_deg > MAX_DELTA_DEG {
        return None;
    }
    if !depth_km.is_finite() || depth_km < 0.0 || depth_km > MAX_DEPTH_KM {
        return None;
    }
    interp_depth_grid(p_p_grid(), delta_deg, depth_km)
}

pub fn s_p_travel(delta_deg: f64, depth_km: f64) -> Option<f64> {
    if !delta_deg.is_finite() || delta_deg < 0.0 || delta_deg > MAX_DELTA_DEG {
        return None;
    }
    if !depth_km.is_finite() || depth_km < 0.0 || depth_km > MAX_DEPTH_KM {
        return None;
    }
    interp_depth_grid(s_p_grid(), delta_deg, depth_km)
}

pub fn surface_p_velocity() -> Option<f64> {
    model().vp.first().map(|&(_, v)| v)
}

pub fn surface_s_velocity() -> Option<f64> {
    model().vs.first().map(|&(_, v)| v)
}

const DEG_TO_KM: f64 = 111.1949;

fn rayparam_deg(
    travel: impl Fn(f64, f64) -> Option<f64>,
    delta_deg: f64,
    depth_km: f64,
) -> Option<f64> {
    let d = 0.5;
    let t_lo = travel(delta_deg - d, depth_km)?;
    let t_hi = travel(delta_deg + d, depth_km)?;
    Some((t_hi - t_lo) / (2.0 * d) / DEG_TO_KM)
}

pub fn p_p_rayparam(delta_deg: f64, depth_km: f64) -> Option<f64> {
    rayparam_deg(p_p_travel, delta_deg, depth_km)
}

pub fn s_p_rayparam(delta_deg: f64, depth_km: f64) -> Option<f64> {
    rayparam_deg(s_p_travel, delta_deg, depth_km)
}

pub fn surface_incidence_deg(p_s_km: f64) -> Option<f64> {
    let alpha = surface_p_velocity()?;
    let s = p_s_km * alpha;
    if !s.is_finite() || s <= 0.0 || s >= 1.0 {
        return None;
    }
    Some(s.asin().to_degrees())
}

pub fn free_surface_pp(p_s_km: f64) -> Option<f64> {
    let alpha = surface_p_velocity()?;
    let beta = surface_s_velocity()?;
    free_surface_pp_ab(p_s_km, alpha, beta)
}

fn free_surface_pp_ab(p: f64, alpha: f64, beta: f64) -> Option<f64> {
    let eta = (1.0 / (alpha * alpha) - p * p).sqrt();
    let xi = (1.0 / (beta * beta) - p * p).sqrt();
    if !eta.is_finite() || !xi.is_finite() || eta <= 0.0 || xi <= 0.0 {
        return None;
    }
    let d = 1.0 / (beta * beta) - 2.0 * p * p;
    let num = 4.0 * p * p * eta * xi - d * d;
    let den = 4.0 * p * p * eta * xi + d * d;
    if den.abs() < 1e-15 {
        return None;
    }
    Some(num / den)
}

pub fn free_surface_sp(p_s_km: f64) -> Option<f64> {
    let alpha = surface_p_velocity()?;
    let beta = surface_s_velocity()?;
    let eta = (1.0 / (alpha * alpha) - p_s_km * p_s_km).sqrt();
    let xi = (1.0 / (beta * beta) - p_s_km * p_s_km).sqrt();
    if !eta.is_finite() || !xi.is_finite() || eta <= 0.0 || xi <= 0.0 {
        return None;
    }
    let d = 1.0 / (beta * beta) - 2.0 * p_s_km * p_s_km;
    let num = -4.0 * xi * p_s_km * d;
    let den = 4.0 * p_s_km * p_s_km * eta * xi + d * d;
    if den.abs() < 1e-15 {
        return None;
    }
    Some(num / den)
}

#[cfg(test)]
fn free_surface_ps(p: f64, alpha: f64, beta: f64) -> Option<f64> {
    let eta = (1.0 / (alpha * alpha) - p * p).sqrt();
    let xi = (1.0 / (beta * beta) - p * p).sqrt();
    if !eta.is_finite() || !xi.is_finite() || eta <= 0.0 || xi <= 0.0 {
        return None;
    }
    let d = 1.0 / (beta * beta) - 2.0 * p * p;
    let r_pp = free_surface_pp_ab(p, alpha, beta)?;
    Some(2.0 * eta * p * (1.0 - r_pp) / d)
}

#[cfg(test)]
fn free_surface_ss(p: f64, alpha: f64, beta: f64) -> Option<f64> {
    free_surface_pp_ab(p, alpha, beta)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn free_surface_pp_reverses_at_normal_incidence() {
        let alpha = surface_p_velocity().unwrap();
        let beta = surface_s_velocity().unwrap();
        let r = free_surface_pp_ab(0.0, alpha, beta).unwrap();
        assert!(
            (r + 1.0).abs() < 1e-9,
            "at normal incidence the free surface reflects P inverted, got {r}"
        );
        assert!(
            free_surface_sp(0.0).unwrap().abs() < 1e-12,
            "no conversion at normal incidence"
        );
    }

    #[test]
    fn free_surface_reflection_conserves_energy() {
        let alpha = surface_p_velocity().unwrap();
        let beta = surface_s_velocity().unwrap();
        let inv_a2 = 1.0 / (alpha * alpha);
        let inv_b2 = 1.0 / (beta * beta);
        let mut p = 0.01;
        while p < 1.0 / alpha - 0.005 {
            let eta = (inv_a2 - p * p).sqrt();
            let xi = (inv_b2 - p * p).sqrt();
            let r_pp = free_surface_pp_ab(p, alpha, beta).unwrap();
            let r_ps = free_surface_ps(p, alpha, beta).unwrap();
            let r_ss = free_surface_ss(p, alpha, beta).unwrap();
            let r_sp = free_surface_sp(p).unwrap();
            let flux_p = eta * r_pp * r_pp + xi * r_ps * r_ps;
            let flux_s = xi * r_ss * r_ss + eta * r_sp * r_sp;
            assert!(
                (flux_p - eta).abs() < 1e-6,
                "P flux {flux_p} vs {eta} at p={p}"
            );
            assert!(
                (flux_s - xi).abs() < 1e-6,
                "S flux {flux_s} vs {xi} at p={p}"
            );
            assert!(r_pp.abs() <= 1.0 + 1e-9, "|R_pp| {r_pp} exceeds 1 at p={p}");
            assert!(r_ss.abs() <= 1.0 + 1e-9, "|R_ss| {r_ss} exceeds 1 at p={p}");
            p += 0.005;
        }
    }

    #[test]
    fn free_surface_pp_crosses_zero_at_an_oblique_angle() {
        let alpha = surface_p_velocity().unwrap();
        let beta = surface_s_velocity().unwrap();
        let r0 = free_surface_pp_ab(0.0, alpha, beta).unwrap();
        assert!(r0 < 0.0, "near-normal incidence must be inverted");
        let mut prev = r0;
        let mut crossed = false;
        let mut p = 0.005;
        while p < 1.0 / alpha {
            let r = free_surface_pp_ab(p, alpha, beta).unwrap();
            if prev < 0.0 && r > 0.0 {
                crossed = true;
            }
            prev = r;
            p += 0.005;
        }
        assert!(
            crossed,
            "R_pp must cross zero (the positive-polarity band) at an oblique incidence angle"
        );
    }

    #[test]
    fn rayparam_of_the_deep_phases_is_a_finite_positive_slowness() {
        for d in [30.0, 60.0, 90.0] {
            let p = p_p_rayparam(d, 231.0).unwrap();
            assert!(p.is_finite() && p > 0.0, "pP ray param at {d} deg: {p}");
            let sp = s_p_rayparam(d, 231.0).unwrap();
            assert!(sp.is_finite() && sp > 0.0, "sP ray param at {d} deg: {sp}");
        }
    }

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
        for p in [100.0, 300.0, 500.0, 620.0] {
            let pts = ray(&nodes, &grid, p, R_EARTH_KM);
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
        assert!(s_travel(-1.0).is_none());
        assert!(s_travel(99.0).is_none());
        assert!(p_p_travel(-1.0, 20.0).is_none());
        assert!(p_p_travel(99.0, 20.0).is_none());
        assert!(p_p_travel(60.0, 300.0).is_none());
        assert!(s_p_travel(60.0, -1.0).is_none());
        assert!(s_p_travel(99.0, 20.0).is_none());
        assert!(s_p_travel(60.0, 300.0).is_none());
    }

    #[test]
    fn s_waves_arrive_after_p() {
        for d in [10.0, 30.0, 60.0, 90.0] {
            let tp = p_travel(d).unwrap();
            let ts = s_travel(d).unwrap();
            assert!(ts > tp, "S({d})={ts} not after P({d})={tp}");
        }
    }

    #[test]
    fn depth_phases_reduce_to_p_at_zero_depth() {
        for d in [20.0, 40.0, 60.0, 80.0] {
            let tp = p_travel(d).unwrap();
            assert_eq!(p_p_travel(d, 0.0).unwrap(), tp);
            assert_eq!(s_p_travel(d, 0.0).unwrap(), tp);
        }
    }

    #[test]
    fn p_p_arrives_after_direct_p() {
        for (d, h) in [(30.0, 20.0), (60.0, 20.0), (60.0, 100.0), (90.0, 20.0)] {
            let direct = p_travel_depth(d, h).unwrap();
            let pp = p_p_travel(d, h).unwrap();
            assert!(pp > direct, "pP({d},{h})={pp} not after P={direct}");
        }
    }

    #[test]
    fn s_p_arrives_after_p_p() {
        for (d, h) in [(40.0, 20.0), (60.0, 20.0), (90.0, 20.0), (60.0, 100.0)] {
            let pp = p_p_travel(d, h).unwrap();
            let sp = s_p_travel(d, h).unwrap();
            assert!(sp > pp, "sP({d},{h})={sp} not after pP={pp}");
        }
    }

    #[test]
    fn p_p_is_monotone_in_distance() {
        let mut prev = 0.0;
        for d in [30.0, 40.0, 50.0, 60.0, 70.0, 80.0] {
            let t = p_p_travel(d, 20.0).unwrap();
            assert!(t > prev, "pP({d},20)={t} not above {prev}");
            prev = t;
        }
    }

    #[test]
    fn direct_up_leg_matches_ray_branch() {
        let m = model();
        let rs = R_EARTH_KM - 20.0;
        for p in [300.0, 400.0, 500.0, 700.0] {
            let (du, tu) = up_leg(&m.vp, &m.grid, p, rs).unwrap();
            let (dr, tr) = ray(&m.vp, &m.grid, p, rs)[0];
            assert!((du - dr).abs() < 1e-1, "p={p}: up d {du} vs ray {dr}");
            assert!((tu - tr).abs() < 1e-1, "p={p}: up t {tu} vs ray {tr}");
        }
    }

    #[test]
    fn p_p_equals_direct_plus_twice_up_leg() {
        let m = model();
        let rs = R_EARTH_KM - 20.0;
        for p in [300.0, 400.0, 500.0] {
            let (du, tu) = ray(&m.vp, &m.grid, p, rs)[0];
            let (dd, td) = ray(&m.vp, &m.grid, p, rs)[1];
            let (ds, ts) = surface_leg(&m.vp, &m.grid, p).unwrap();
            let t_pp = tu + ts;
            let t_identity = td + 2.0 * tu;
            let d_pp = du + ds;
            let d_identity = dd + 2.0 * du;
            assert!(
                (t_pp - t_identity).abs() < 1e-6,
                "p={p}: t_pp={t_pp} vs {t_identity}"
            );
            assert!(
                (d_pp - d_identity).abs() < 1e-6,
                "p={p}: d_pp={d_pp} vs {d_identity}"
            );
        }
    }

    #[test]
    fn s_p_equals_direct_p_plus_both_up_legs() {
        let m = model();
        let rs = R_EARTH_KM - 20.0;
        for p in [480.0, 500.0, 520.0] {
            let (du_s, tu_s) = ray(&m.vs, &m.sgrid, p, rs)[0];
            let (du_p, tu_p) = ray(&m.vp, &m.grid, p, rs)[0];
            let (_, td) = ray(&m.vp, &m.grid, p, rs)[1];
            let (ds, ts) = surface_leg(&m.vp, &m.grid, p).unwrap();
            let t_sp = tu_s + ts;
            let t_identity = td + tu_p + tu_s;
            assert!(
                (t_sp - t_identity).abs() < 1e-6,
                "p={p}: t_sp={t_sp} vs {t_identity}"
            );
            assert!((du_s + du_p + ds).is_finite());
        }
    }
}
