pub const GM_SUN_M3_S2: f64 = 1.32712440018e20;
pub const AU_M: f64 = 1.495978707e11;
const OBLIQUITY_DEG: f64 = 23.4392911;
const TAU: f64 = 2.0 * std::f64::consts::PI;

pub fn solve_kepler_ecc(mean_anomaly_rad: f64, e: f64) -> f64 {
    if !e.is_finite() || e <= 0.0 {
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
    elements_to_icrs_state(
        a_au, e, incl_deg, node_deg, peri_deg, ma_deg, epoch_jd, t_jd,
    )
    .map(|(p, _)| p)
}

pub fn elements_to_icrs_state(
    a_au: f64,
    e: f64,
    incl_deg: f64,
    node_deg: f64,
    peri_deg: f64,
    ma_deg: f64,
    epoch_jd: f64,
    t_jd: f64,
) -> Option<([f64; 3], [f64; 3])> {
    if !a_au.is_finite() || a_au <= 0.0 {
        return None;
    }
    if !e.is_finite() || e < 0.0 || e >= 1.0 {
        return None;
    }
    let a_m = a_au * AU_M;
    let n = (GM_SUN_M3_S2 / a_m.powi(3)).sqrt();
    let dt = (t_jd - epoch_jd) * 86400.0;
    let m = (ma_deg.to_radians() + n * dt).rem_euclid(TAU);
    let ecc = solve_kepler_ecc(m, e);
    let cos_e = ecc.cos();
    let sin_e = ecc.sin();
    let r_m = a_m * (1.0 - e * cos_e);
    let x_orb = a_m * (cos_e - e);
    let y_orb = a_m * (1.0 - e * e).sqrt() * sin_e;
    let fac = (GM_SUN_M3_S2 * a_m).sqrt() / r_m;
    let vx_orb = -fac * sin_e;
    let vy_orb = fac * (1.0 - e * e).sqrt() * cos_e;
    let (sp, cp) = peri_deg.to_radians().sin_cos();
    let (so, co) = node_deg.to_radians().sin_cos();
    let (si, ci) = incl_deg.to_radians().sin_cos();
    let r11 = cp * co - sp * so * ci;
    let r12 = -sp * co - cp * so * ci;
    let r21 = cp * so + sp * co * ci;
    let r22 = -sp * so + cp * co * ci;
    let r31 = sp * si;
    let r32 = cp * si;
    let xe = r11 * x_orb + r12 * y_orb;
    let ye = r21 * x_orb + r22 * y_orb;
    let ze = r31 * x_orb + r32 * y_orb;
    let vxe = r11 * vx_orb + r12 * vy_orb;
    let vye = r21 * vx_orb + r22 * vy_orb;
    let vze = r31 * vx_orb + r32 * vy_orb;
    let (se, ce) = OBLIQUITY_DEG.to_radians().sin_cos();
    Some((
        [xe, ye * ce - ze * se, ye * se + ze * ce],
        [vxe, vye * ce - vze * se, vye * se + vze * ce],
    ))
}

#[cfg(test)]
mod tests {
    use super::{
        elements_to_icrs, elements_to_icrs_state, solve_kepler_ecc, AU_M, GM_SUN_M3_S2, TAU,
    };

    const J2000: f64 = 2451545.0;

    fn norm(p: [f64; 3]) -> f64 {
        (p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt()
    }

    #[test]
    fn circular_orbital_speed() {
        let (_, v) = elements_to_icrs_state(1.0, 0.0, 0.0, 0.0, 0.0, 0.0, J2000, J2000).unwrap();
        let expect = (GM_SUN_M3_S2 / AU_M).sqrt();
        assert!((norm(v) - expect).abs() < 1e-3);
    }

    #[test]
    fn perihelion_vis_viva() {
        let a = 2.0;
        let e = 0.5;
        let (_, v) = elements_to_icrs_state(a, e, 0.0, 0.0, 0.0, 0.0, J2000, J2000).unwrap();
        let expect = (GM_SUN_M3_S2 * (1.0 + e) / (a * AU_M * (1.0 - e))).sqrt();
        assert!((norm(v) - expect).abs() < 1e-3);
    }

    #[test]
    fn state_position_matches_position_only() {
        let (p, _) =
            elements_to_icrs_state(1.3, 0.2, 11.0, 80.0, 73.0, 10.0, J2000, J2000 + 37.0).unwrap();
        let q = elements_to_icrs(1.3, 0.2, 11.0, 80.0, 73.0, 10.0, J2000, J2000 + 37.0).unwrap();
        for k in 0..3 {
            assert!((p[k] - q[k]).abs() < 1e-6);
        }
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
