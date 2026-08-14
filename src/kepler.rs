pub const GM_SUN_M3_S2: f64 = 1.32712440018e20;
pub const AU_M: f64 = 1.495978707e11;
const OBLIQUITY_DEG: f64 = 23.4392911;
const TAU: f64 = 2.0 * std::f64::consts::PI;

pub fn solve_kepler_ecc(mean_anomaly_rad: f64, e: f64) -> f64 {
    if e <= 0.0 {
        return mean_anomaly_rad;
    }
    let m = mean_anomaly_rad.rem_euclid(TAU);
    let mut ecc = m;
    for _ in 0..60 {
        let f = ecc - e * ecc.sin() - m;
        let fp = 1.0 - e * ecc.cos();
        let d = f / fp;
        ecc -= d;
        if d.abs() < 1e-14 {
            break;
        }
    }
    ecc
}

pub fn elements_to_icrs(
    a_au: f64,
    e: f64,
    incl_deg: f64,
    node_deg: f64,
    peri_deg: f64,
    ma_deg: f64,
    epoch_jd: f64,
    t_jd: f64,
) -> Option<[f64; 3]> {
    if !a_au.is_finite() || a_au <= 0.0 {
        return None;
    }
    if !e.is_finite() || e < 0.0 || e >= 1.0 {
        return None;
    }
    let n = (GM_SUN_M3_S2 / (a_au * AU_M).powi(3)).sqrt();
    let dt = (t_jd - epoch_jd) * 86400.0;
    let m = (ma_deg.to_radians() + n * dt).rem_euclid(TAU);
    let ecc = solve_kepler_ecc(m, e);
    let cos_e = ecc.cos();
    let x_orb = a_au * (cos_e - e);
    let y_orb = a_au * (1.0 - e * e).sqrt() * ecc.sin();
    let (sp, cp) = peri_deg.to_radians().sin_cos();
    let (so, co) = node_deg.to_radians().sin_cos();
    let (si, ci) = incl_deg.to_radians().sin_cos();
    let xe = (cp * co - sp * so * ci) * x_orb + (-sp * co - cp * so * ci) * y_orb;
    let ye = (cp * so + sp * co * ci) * x_orb + (-sp * so + cp * co * ci) * y_orb;
    let ze = (sp * si) * x_orb + (cp * si) * y_orb;
    let (se, ce) = OBLIQUITY_DEG.to_radians().sin_cos();
    Some([
        xe * AU_M,
        (ye * ce - ze * se) * AU_M,
        (ye * se + ze * ce) * AU_M,
    ])
}

#[cfg(test)]
mod tests {
    use super::{elements_to_icrs, solve_kepler_ecc, AU_M, GM_SUN_M3_S2, TAU};

    const J2000: f64 = 2451545.0;

    fn norm(p: [f64; 3]) -> f64 {
        (p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt()
    }

    #[test]
    fn kepler_converges_circular() {
        let m = 1.2;
        let ecc = solve_kepler_ecc(m, 0.0);
        assert!((ecc - m).abs() < 1e-12);
    }

    #[test]
    fn kepler_converges_eccentric() {
        let m = 0.9;
        let e = 0.21;
        let ecc = solve_kepler_ecc(m, e);
        assert!((ecc - e * ecc.sin() - m).abs() < 1e-10);
    }

    #[test]
    fn circular_orbit_constant_radius() {
        let r0 = elements_to_icrs(1.0, 0.0, 0.0, 0.0, 0.0, 0.0, J2000, J2000).unwrap();
        let r1 = elements_to_icrs(1.0, 0.0, 0.0, 0.0, 0.0, 120.0, J2000, J2000).unwrap();
        assert!((norm(r0) - AU_M).abs() < 1e-6);
        assert!((norm(r1) - AU_M).abs() < 1e-6);
    }

    #[test]
    fn perihelion_distance() {
        let p = elements_to_icrs(2.0, 0.5, 0.0, 0.0, 0.0, 0.0, J2000, J2000).unwrap();
        assert!((norm(p) - AU_M).abs() < 1e-6);
    }

    #[test]
    fn aphelion_distance() {
        let p = elements_to_icrs(2.0, 0.5, 0.0, 0.0, 0.0, 180.0, J2000, J2000).unwrap();
        assert!((norm(p) - 3.0 * AU_M).abs() < 1e-6);
    }

    #[test]
    fn orbital_period_closed() {
        let a_au: f64 = 1.3;
        let p_days = TAU / (GM_SUN_M3_S2 / (a_au * AU_M).powi(3)).sqrt() / 86400.0;
        let r0 = elements_to_icrs(a_au, 0.2, 11.0, 80.0, 73.0, 10.0, J2000, J2000).unwrap();
        let r1 =
            elements_to_icrs(a_au, 0.2, 11.0, 80.0, 73.0, 10.0, J2000, J2000 + p_days).unwrap();
        for k in 0..3 {
            assert!(
                (r0[k] - r1[k]).abs() < 10.0,
                "component {}: {} vs {}",
                k,
                r0[k],
                r1[k]
            );
        }
    }

    #[test]
    fn mean_anomaly_wraps() {
        let m = 3.0 * TAU + 0.7;
        let ecc = solve_kepler_ecc(m, 0.3);
        assert!((ecc - solve_kepler_ecc(0.7, 0.3)).abs() < 1e-12);
    }
}
