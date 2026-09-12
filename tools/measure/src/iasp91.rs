const R_EARTH_KM: f64 = 6371.0;
const MAX_DELTA_DEG: f64 = 98.0;
const PI: f64 = std::f64::consts::PI;
const RAD_TO_DEG: f64 = 180.0 / PI;
const DEG_TO_RAD: f64 = PI / 180.0;

const IASP91: &[(f64, f64, f64, f64)] = &[
    (0.00, 5.8000, 3.3600, 2.7200),
    (20.00, 5.8000, 3.3600, 2.7200),
    (20.00, 6.5000, 3.7500, 2.9200),
    (35.00, 6.5000, 3.7500, 2.9200),
    (35.00, 8.0400, 4.4700, 3.3198),
    (77.50, 8.0450, 4.4850, 3.3455),
    (120.00, 8.0500, 4.5000, 3.3713),
    (165.00, 8.1750, 4.5090, 3.3985),
    (210.00, 8.3000, 4.5180, 3.4258),
    (260.00, 8.4825, 4.6090, 3.4561),
    (310.00, 8.6650, 4.6964, 3.4864),
    (360.00, 8.8475, 4.7832, 3.5167),
    (410.00, 9.0300, 4.8700, 3.5470),
    (410.00, 9.3600, 5.0800, 3.5470),
    (460.00, 9.5200, 5.1864, 3.5800),
    (510.00, 9.6900, 5.2922, 3.6100),
    (560.00, 9.8600, 5.3989, 3.6400),
    (610.00, 10.0320, 5.5047, 3.6700),
    (660.00, 10.2000, 5.6104, 3.7000),
    (660.00, 10.7900, 5.9600, 3.7000),
    (710.00, 10.9220, 6.0898, 3.7300),
    (760.00, 11.0550, 6.2100, 3.7600),
    (809.50, 11.1350, 6.2420, 3.7900),
    (859.00, 11.2220, 6.2799, 3.8200),
    (908.50, 11.3060, 6.3164, 3.8500),
    (958.00, 11.3890, 6.3519, 3.8800),
    (1007.50, 11.4700, 6.3860, 3.9100),
    (1057.00, 11.5490, 6.4182, 3.9400),
    (1106.50, 11.6260, 6.4514, 3.9700),
    (1156.00, 11.7020, 6.4822, 4.0000),
    (1205.50, 11.7760, 6.5131, 4.0300),
    (1255.00, 11.8490, 6.5431, 4.0600),
    (1304.50, 11.9200, 6.5728, 4.0900),
    (1354.00, 11.9890, 6.6009, 4.1200),
    (1403.50, 12.0570, 6.6285, 4.1500),
    (1453.00, 12.1240, 6.6554, 4.1800),
    (1502.50, 12.1910, 6.6813, 4.2100),
    (1552.00, 12.2560, 6.7070, 4.2400),
    (1601.50, 12.3200, 6.7323, 4.2700),
    (1651.00, 12.3840, 6.7570, 4.3000),
    (1700.50, 12.4460, 6.7810, 4.3300),
    (1750.00, 12.5070, 6.8044, 4.3600),
    (1799.50, 12.5670, 6.8274, 4.3900),
    (1849.00, 12.6250, 6.8498, 4.4200),
    (1898.50, 12.6830, 6.8715, 4.4500),
    (1948.00, 12.7390, 6.8928, 4.4800),
    (1997.50, 12.7940, 6.9134, 4.5100),
    (2047.00, 12.8480, 6.9338, 4.5400),
    (2096.50, 12.9010, 6.9537, 4.5700),
    (2146.00, 12.9530, 6.9732, 4.6000),
    (2195.50, 13.0040, 6.9922, 4.6300),
    (2245.00, 13.0550, 7.0111, 4.6600),
    (2294.50, 13.1040, 7.0292, 4.6900),
    (2344.00, 13.1530, 7.0469, 4.7200),
    (2393.50, 13.2010, 7.0640, 4.7500),
    (2443.00, 13.2480, 7.0808, 4.7800),
    (2492.50, 13.2940, 7.0971, 4.8100),
    (2542.00, 13.3390, 7.1129, 4.8400),
    (2591.50, 13.3840, 7.1286, 4.8700),
    (2641.00, 13.4270, 7.1438, 4.9000),
    (2690.50, 13.4700, 7.1585, 4.9300),
    (2740.00, 13.5120, 7.1727, 4.9600),
    (2740.00, 13.5120, 7.1727, 4.9600),
    (2789.67, 13.5360, 7.1865, 4.9900),
    (2839.33, 13.5900, 7.2140, 5.0200),
    (2889.00, 13.6450, 7.2400, 5.0500),
];

pub fn iasp91_nodes() -> &'static [(f64, f64, f64, f64)] {
    IASP91
}

pub fn surface_p_velocity() -> f64 {
    IASP91[0].1
}

pub fn surface_s_velocity() -> f64 {
    IASP91[0].2
}

fn p_nodes() -> Vec<(f64, f64)> {
    IASP91
        .iter()
        .map(|&(d, vp, _, _)| (R_EARTH_KM - d, vp))
        .collect()
}

fn vp_at(nodes: &[(f64, f64)], r: f64) -> f64 {
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

fn vp_outer(nodes: &[(f64, f64)], r: f64) -> f64 {
    for i in 0..nodes.len() {
        if nodes[i].0 == r {
            return nodes[i].1;
        }
    }
    vp_at(nodes, r)
}

fn eta_of(nodes: &[(f64, f64)], r: f64) -> f64 {
    r / vp_at(nodes, r)
}

fn turning_radius(nodes: &[(f64, f64)], p: f64) -> Option<f64> {
    for w in nodes.windows(2) {
        let (rt, vt) = w[0];
        let (rb, vb) = w[1];
        if rt == rb {
            let eta_above = rt / vt;
            let eta_below = rb / vb;
            if p >= eta_below && p <= eta_above {
                return Some(rt);
            }
            continue;
        }
        let eta_top = rt / vt;
        let eta_bot = rb / vb;
        let (lo, hi) = (eta_bot.min(eta_top), eta_bot.max(eta_top));
        if p > lo && p < hi {
            let mut a = rb;
            let mut b = rt;
            let ga = eta_bot - p;
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

fn leg(nodes: &[(f64, f64)], p: f64, rt: f64) -> Option<(f64, f64)> {
    if rt >= R_EARTH_KM {
        return Some((0.0, 0.0));
    }
    let eta_outer = rt / vp_outer(nodes, rt);
    if eta_outer < p - 1e-9 {
        return None;
    }
    let xmax = (R_EARTH_KM - rt).sqrt();
    if !xmax.is_finite() || xmax <= 0.0 {
        return None;
    }
    let n = 2048usize;
    let dx = xmax / n as f64;
    let f0 = if eta_outer - p > 1e-9 {
        0.0
    } else {
        let h = 0.5;
        let eta_prime = (eta_of(nodes, rt + h) - eta_outer) / h;
        if eta_prime <= 0.0 || !eta_prime.is_finite() {
            return None;
        }
        2.0 / (rt * (2.0 * p * eta_prime).sqrt())
    };
    let mut sum_d = 0.0;
    let mut sum_t = 0.0;
    for k in 0..=n {
        let x = k as f64 * dx;
        let r = rt + x * x;
        let w = if k == 0 || k == n { 0.5 } else { 1.0 };
        if k == 0 {
            sum_d += w * p * f0;
            sum_t += w * eta_outer * eta_outer * f0;
        } else {
            let e = eta_of(nodes, r);
            let sq = e * e - p * p;
            if sq <= 0.0 {
                return None;
            }
            let f = 2.0 * x / (r * sq.sqrt());
            sum_d += w * p * f;
            sum_t += w * e * e * f;
        }
    }
    Some((sum_d * dx, sum_t * dx))
}

fn ray(nodes: &[(f64, f64)], p: f64) -> Option<(f64, f64)> {
    let rt = turning_radius(nodes, p)?;
    let (d, t) = leg(nodes, p, rt)?;
    Some((2.0 * d, 2.0 * t))
}

struct TravelTable {
    pts: Vec<(f64, f64)>,
}

fn build_table() -> TravelTable {
    let nodes = p_nodes();
    let p_min = nodes[nodes.len() - 1].0 / nodes[nodes.len() - 1].1;
    let p_max = R_EARTH_KM / surface_p_velocity();
    let n = 60000;
    let mut pts: Vec<(f64, f64)> = vec![(0.0, 0.0)];
    for k in 0..=n {
        let p = p_min + (p_max - p_min) * (k as f64 / n as f64);
        if let Some((d, t)) = ray(&nodes, p) {
            let delta_deg = d * RAD_TO_DEG;
            if delta_deg.is_finite() && delta_deg > 0.0 && delta_deg <= MAX_DELTA_DEG {
                pts.push((delta_deg, t));
            }
        }
    }
    pts.sort_by(|a, b| a.0.total_cmp(&b.0));
    TravelTable { pts }
}

fn travel_table() -> &'static TravelTable {
    use std::sync::OnceLock;
    static T: OnceLock<TravelTable> = OnceLock::new();
    T.get_or_init(build_table)
}

pub fn p_travel(delta_deg: f64) -> Option<f64> {
    if !delta_deg.is_finite() || delta_deg < 0.0 || delta_deg > MAX_DELTA_DEG {
        return None;
    }
    let pts = &travel_table().pts;
    let first = pts[0];
    let last = pts[pts.len() - 1];
    if delta_deg <= first.0 {
        return Some(first.1);
    }
    if delta_deg >= last.0 {
        return Some(last.1);
    }
    let mut lo = 0usize;
    let mut hi = pts.len() - 1;
    while hi - lo > 1 {
        let mid = (lo + hi) / 2;
        if pts[mid].0 < delta_deg {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    let (d0, t0) = pts[lo];
    let (d1, t1) = pts[hi];
    let f = (delta_deg - d0) / (d1 - d0);
    Some(t0 + (t1 - t0) * f)
}

pub struct Triplication {
    pub depth_km: f64,
    pub delta_lo_deg: f64,
    pub delta_hi_deg: f64,
    pub slowness_lo_s_deg: f64,
    pub slowness_hi_s_deg: f64,
}

fn discontinuity_depths() -> Vec<(f64, f64, f64)> {
    let mut out = Vec::new();
    for w in IASP91.windows(2) {
        if w[0].0 == w[1].0 && (w[0].1 - w[1].1).abs() > 1e-9 {
            out.push((w[0].0, w[0].1, w[1].1));
        }
    }
    out
}

pub fn triplications() -> Vec<Triplication> {
    let nodes = p_nodes();
    discontinuity_depths()
        .into_iter()
        .filter_map(|(depth_km, vp_above, vp_below)| {
            let rd = R_EARTH_KM - depth_km;
            let eta_above = rd / vp_above;
            let eta_below = rd / vp_below;
            let (d_lo, _) = leg(&nodes, eta_below, rd)?;
            let (d_hi, _) = leg(&nodes, eta_above, rd)?;
            let delta_lo = 2.0 * d_lo * RAD_TO_DEG;
            let delta_hi = 2.0 * d_hi * RAD_TO_DEG;
            if !delta_lo.is_finite() || !delta_hi.is_finite() {
                return None;
            }
            Some(Triplication {
                depth_km,
                delta_lo_deg: delta_lo.min(delta_hi),
                delta_hi_deg: delta_lo.max(delta_hi),
                slowness_lo_s_deg: eta_below * DEG_TO_RAD,
                slowness_hi_s_deg: eta_above * DEG_TO_RAD,
            })
        })
        .collect()
}

pub fn delta_sweep() -> Vec<(f64, f64, f64)> {
    let nodes = p_nodes();
    let p_min = nodes[nodes.len() - 1].0 / nodes[nodes.len() - 1].1;
    let p_max = R_EARTH_KM / surface_p_velocity();
    let n = 60000;
    let mut out = Vec::new();
    for k in 0..=n {
        let p = p_min + (p_max - p_min) * (k as f64 / n as f64);
        if let Some((d, t)) = ray(&nodes, p) {
            let delta_deg = d * RAD_TO_DEG;
            if delta_deg.is_finite() && delta_deg >= 0.0 && delta_deg <= MAX_DELTA_DEG {
                out.push((p * DEG_TO_RAD, delta_deg, t));
            }
        }
    }
    out.sort_by(|a, b| a.0.total_cmp(&b.0));
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn surface_velocities_are_the_published_iasp91_values() {
        assert!((surface_p_velocity() - 5.80).abs() < 1e-12);
        assert!((surface_s_velocity() - 3.36).abs() < 1e-12);
    }

    #[test]
    fn constant_velocity_matches_chord() {
        let nodes = vec![(R_EARTH_KM, 10.0), (0.0, 10.0)];
        for p in [100.0, 300.0, 500.0, 620.0] {
            let (d, t) = ray(&nodes, p).unwrap();
            let delta_deg = d.to_degrees();
            let chord = 2.0 * R_EARTH_KM * (delta_deg.to_radians() / 2.0).sin();
            let t_exact = chord / 10.0;
            assert!((t - t_exact).abs() < 0.02, "p={p}: T={t} vs {t_exact}");
        }
    }

    #[test]
    fn travel_time_is_monotone_in_distance() {
        let mut prev = 0.0;
        for d in [2.0, 5.0, 20.0, 40.0, 70.0, 90.0] {
            let t = p_travel(d).unwrap();
            assert!(t > prev, "T({d}) = {t} not above {prev}");
            prev = t;
        }
    }

    #[test]
    fn triplication_brackets_are_finite_and_ordered() {
        let trips = triplications();
        let by_depth = |d: f64| trips.iter().find(|t| (t.depth_km - d).abs() < 1e-6);
        let t410 = by_depth(410.0).unwrap();
        let t660 = by_depth(660.0).unwrap();
        assert!(t410.delta_lo_deg > 0.0 && t410.delta_hi_deg > t410.delta_lo_deg);
        assert!(t660.delta_lo_deg > 0.0 && t660.delta_hi_deg > t660.delta_lo_deg);
        assert!(t410.delta_hi_deg < 30.0);
        assert!(t660.delta_hi_deg < 40.0);
        assert!(t410.slowness_lo_s_deg < t410.slowness_hi_s_deg);
        assert!(t660.slowness_lo_s_deg < t660.slowness_hi_s_deg);
        assert!(t410.slowness_lo_s_deg > t660.slowness_hi_s_deg);
    }

    #[test]
    fn p_waves_arrive_after_zero_distance() {
        assert!((p_travel(0.0).unwrap() - 0.0).abs() < 1e-9);
        for d in [10.0, 30.0, 60.0, 90.0] {
            assert!(p_travel(d).unwrap() > 0.0);
        }
    }
}
