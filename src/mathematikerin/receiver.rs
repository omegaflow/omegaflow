use crate::archivar::types::C_LIGHT;

pub const TWO_WAY_DOPPLER_CONVENTION: &str = "t_bounce from classical downlink light-time (<=10 steps, 1e-12 s); t_tx from classical uplink light-time against the bounce state; roundtrip_s = t_rx - t_tx + shapiro_s; range_rate_m_s is the downlink line-of-sight rate (positive receding), m/s; downlink_hz = f_ref * turn_ratio * (1 - rho_dot_dn/c); uplink_hz = f_ref * (1 - rho_dot_up/c); observable_hz = f_ref * turn_ratio * (1 - rho_dot_up/c) * (1 - rho_dot_dn/c); the gravitating body (its ICRS position and GM) is an explicit parameter of the reduction, a declared worldline, never the ICRS origin";

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TwoWayDoppler {
    pub t_rx_tdb: f64,
    pub t_bounce_tdb: f64,
    pub t_tx_tdb: f64,
    pub roundtrip_s: f64,
    pub range_rate_m_s: f64,
    pub downlink_hz: f64,
    pub uplink_hz: f64,
    pub shapiro_s: f64,
    pub observable_hz: f64,
}

fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn norm(a: [f64; 3]) -> f64 {
    (a[0] * a[0] + a[1] * a[1] + a[2] * a[2]).sqrt()
}

fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn finite3(a: [f64; 3]) -> bool {
    a[0].is_finite() && a[1].is_finite() && a[2].is_finite()
}

fn downlink_bounce<S>(
    t_rx_tdb: f64,
    station_rx_icrs_km: [f64; 3],
    target_state: &S,
) -> Option<(f64, [f64; 3], [f64; 3])>
where
    S: Fn(f64) -> Option<([f64; 3], [f64; 3])>,
{
    let c_km_s = C_LIGHT / 1000.0;
    let mut t_bounce = t_rx_tdb;
    for _ in 0..10 {
        let (pos, _vel) = target_state(t_bounce)?;
        if !finite3(pos) {
            return None;
        }
        let d = norm(sub(pos, station_rx_icrs_km));
        if !d.is_finite() {
            return None;
        }
        let t_new = t_rx_tdb - d / c_km_s;
        if !t_new.is_finite() {
            return None;
        }
        let settled = (t_new - t_bounce).abs() < 1.0e-12;
        t_bounce = t_new;
        if settled {
            break;
        }
    }
    let (pos, vel) = target_state(t_bounce)?;
    if !finite3(pos) || !finite3(vel) {
        return None;
    }
    Some((t_bounce, pos, vel))
}

pub fn shapiro_leg_s(
    station_icrs_km: [f64; 3],
    target_icrs_km: [f64; 3],
    gravitator_icrs_km: [f64; 3],
    gm_m3_s2: f64,
) -> Option<f64> {
    if !finite3(station_icrs_km) || !finite3(target_icrs_km) || !finite3(gravitator_icrs_km) {
        return None;
    }
    if !(gm_m3_s2.is_finite() && gm_m3_s2 > 0.0) {
        return None;
    }
    let r1 = norm(sub(station_icrs_km, gravitator_icrs_km)) * 1000.0;
    let r2 = norm(sub(target_icrs_km, gravitator_icrs_km)) * 1000.0;
    let r12 = norm(sub(target_icrs_km, station_icrs_km)) * 1000.0;
    if r1 <= 0.0 || r2 <= 0.0 || r12 <= 0.0 {
        return None;
    }
    let denom = r1 + r2 - r12;
    if !denom.is_finite() || denom <= 0.0 {
        return None;
    }
    let arg = (r1 + r2 + r12) / denom;
    if !arg.is_finite() || arg <= 1.0 {
        return None;
    }
    let dt = 2.0 * gm_m3_s2 / (C_LIGHT * C_LIGHT * C_LIGHT) * arg.ln();
    if dt.is_finite() && dt > 0.0 {
        Some(dt)
    } else {
        None
    }
}

pub fn two_way_doppler<S>(
    t_rx_tdb: f64,
    station_rx_icrs_km: [f64; 3],
    station_tx_icrs_km: [f64; 3],
    f_ref_hz: f64,
    turn_ratio: f64,
    gravitator_icrs_km: [f64; 3],
    gm_gravitator_m3_s2: f64,
    target_state: S,
) -> Option<TwoWayDoppler>
where
    S: Fn(f64) -> Option<([f64; 3], [f64; 3])>,
{
    if !t_rx_tdb.is_finite()
        || !f_ref_hz.is_finite()
        || f_ref_hz <= 0.0
        || !turn_ratio.is_finite()
        || turn_ratio <= 0.0
        || !finite3(station_rx_icrs_km)
        || !finite3(station_tx_icrs_km)
    {
        return None;
    }
    let c_km_s = C_LIGHT / 1000.0;
    let (t_bounce, pos, vel) = downlink_bounce(t_rx_tdb, station_rx_icrs_km, &target_state)?;
    let r_dn = sub(pos, station_rx_icrs_km);
    let d_dn = norm(r_dn);
    let r_up = sub(pos, station_tx_icrs_km);
    let d_up = norm(r_up);
    if d_dn <= 0.0 || d_up <= 0.0 || !d_dn.is_finite() || !d_up.is_finite() {
        return None;
    }
    let t_tx = t_bounce - d_up / c_km_s;
    let shapiro_s = shapiro_leg_s(
        station_rx_icrs_km,
        pos,
        gravitator_icrs_km,
        gm_gravitator_m3_s2,
    )? + shapiro_leg_s(
        station_tx_icrs_km,
        pos,
        gravitator_icrs_km,
        gm_gravitator_m3_s2,
    )?;
    let roundtrip_s = (t_rx_tdb - t_tx) + shapiro_s;
    let rho_dot_dn = dot(r_dn, vel) / d_dn * 1000.0;
    let rho_dot_up = dot(r_up, vel) / d_up * 1000.0;
    let downlink_hz = f_ref_hz * turn_ratio * (1.0 - rho_dot_dn / C_LIGHT);
    let uplink_hz = f_ref_hz * (1.0 - rho_dot_up / C_LIGHT);
    let observable_hz =
        f_ref_hz * turn_ratio * (1.0 - rho_dot_up / C_LIGHT) * (1.0 - rho_dot_dn / C_LIGHT);
    let fields = [
        t_bounce,
        t_tx,
        roundtrip_s,
        rho_dot_dn,
        downlink_hz,
        uplink_hz,
        shapiro_s,
        observable_hz,
    ];
    if fields.iter().any(|v| !v.is_finite()) {
        return None;
    }
    Some(TwoWayDoppler {
        t_rx_tdb,
        t_bounce_tdb: t_bounce,
        t_tx_tdb: t_tx,
        roundtrip_s,
        range_rate_m_s: rho_dot_dn,
        downlink_hz,
        uplink_hz,
        shapiro_s,
        observable_hz,
    })
}

pub const ITRF_TO_CIRS_CONVENTION: &str = "ITRS -> CIRS = R3(-ERA) * W(t), IERS Conventions 2010 (TN36 ch. 5.4.2); W(t) = R3(-s') * R2(x_p) * R1(y_p); R(t) = R3(-ERA); R1(t)=[[1,0,0],[0,cos t,sin t],[0,-sin t,cos t]], R2(t)=[[cos t,0,-sin t],[0,1,0],[sin t,0,cos t]], R3(t)=[[cos t,sin t,0],[-sin t,cos t,0],[0,0,1]]; x_p along the 0-degree meridian, y_p along 90-degree west; s' ~ 0 neglected (secular drift near -47 uas/century, SOFA iauSp00); the transpose hand is verified against ERFA eraPom00 (V(TRS) = rpom * V(CIP), i.e. CIRS -> ITRS), so this carries W = rpom^T; the next step Q(t) = IAU-2006 precession * IAU-2000A nutation (SOFA iauPnm06a/iauC2i06a) is absent, so this is itrf_to_cirs, never itrf_to_icrs";

pub const EARTH_ROTATION_ANGLE_CONVENTION: &str = "IAU 2000 ERA from a UT1 Julian date (SOFA iauEra00 linear part): ERA = 2*pi * frac(0.7790572732640 + 1.00273781191135448 * (UT1_JD - 2451545.0)), result in [0, 2*pi); non-finite input is absent -> None; 1 arcsec = pi/648000 rad (IERS Conventions 2010, Table 1.1)";

const ARCSEC_TO_RAD: f64 = std::f64::consts::PI / 648000.0;

pub struct EopSample {
    pub ut1_utc_s: f64,
    pub pm_x_arcsec: f64,
    pub pm_y_arcsec: f64,
}

pub fn earth_rotation_angle_rad(ut1_jd: f64) -> Option<f64> {
    if !ut1_jd.is_finite() {
        return None;
    }
    let d = ut1_jd - 2451545.0;
    let turn = 0.7790572732640 + 1.002_737_811_911_354_6 * d;
    if !turn.is_finite() {
        return None;
    }
    let frac = turn - turn.floor();
    Some(2.0 * std::f64::consts::PI * frac)
}

fn itrf_to_cirs_at(
    era_rad: f64,
    xp_rad: f64,
    yp_rad: f64,
    r_itrf_km: [f64; 3],
) -> Option<[f64; 3]> {
    if !finite3(r_itrf_km) || !era_rad.is_finite() || !xp_rad.is_finite() || !yp_rad.is_finite() {
        return None;
    }
    let (cx, sx) = (xp_rad.cos(), xp_rad.sin());
    let (cy, sy) = (yp_rad.cos(), yp_rad.sin());
    let t0 = cx * r_itrf_km[0] + sx * sy * r_itrf_km[1] - sx * cy * r_itrf_km[2];
    let t1 = cy * r_itrf_km[1] + sy * r_itrf_km[2];
    let t2 = sx * r_itrf_km[0] - cx * sy * r_itrf_km[1] + cx * cy * r_itrf_km[2];
    let (ce, se) = (era_rad.cos(), era_rad.sin());
    let out = [ce * t0 - se * t1, se * t0 + ce * t1, t2];
    if finite3(out) { Some(out) } else { None }
}

pub fn itrf_to_cirs(r_itrf_km: [f64; 3], ut1_jd: f64, eop: &EopSample) -> Option<[f64; 3]> {
    let era = earth_rotation_angle_rad(ut1_jd)?;
    itrf_to_cirs_at(
        era,
        eop.pm_x_arcsec * ARCSEC_TO_RAD,
        eop.pm_y_arcsec * ARCSEC_TO_RAD,
        r_itrf_km,
    )
}

pub const CIRS_TO_GCRS_CONVENTION: &str = "CIRS -> GCRS = R(t)^T * V(CIRS), SOFA iauC2i06a/iauPnm06a; R(t) = P(t) * N(t); P(t) = Rx(-eps_a) * Rz(-psi_b) * Rx(phi_b) * Rz(gamma_b), the IAU 2006 Fukushima-Williams angles (SOFA iauPfw06/iauFw2m), closed-form, no table; P maps GCRS -> mean equator/equinox of date, so its transpose carries CIRS -> GCRS; R1(t)=[[1,0,0],[0,cos t,sin t],[0,-sin t,cos t]], R2(t)=[[cos t,0,-sin t],[0,1,0],[sin t,0,cos t]], R3(t)=[[cos t,sin t,0],[-sin t,cos t,0],[0,0,1]]; N(t) = IAU 2000A nutation (SOFA iauNut06a) is a NAMED ABSENT step (its 1365-term luni-solar/planetary series exceeds one bounded step), so N = I and this is precession-only; non-finite input or angle is absent -> None; this is cirs_to_gcrs, never cirs_to_icrs";

type Mat3 = [[f64; 3]; 3];

fn rx(phi: f64) -> Mat3 {
    let (s, c) = (phi.sin(), phi.cos());
    [[c, 0.0, -s], [0.0, 1.0, 0.0], [s, 0.0, c]]
}

fn rz(psi: f64) -> Mat3 {
    let (s, c) = (psi.sin(), psi.cos());
    [[c, s, 0.0], [-s, c, 0.0], [0.0, 0.0, 1.0]]
}

fn mat_mul(a: &Mat3, b: &Mat3) -> Mat3 {
    let mut out = [[0.0_f64; 3]; 3];
    for (i, row) in out.iter_mut().enumerate() {
        for (j, cell) in row.iter_mut().enumerate() {
            *cell = a[i][0] * b[0][j] + a[i][1] * b[1][j] + a[i][2] * b[2][j];
        }
    }
    out
}

fn mat_apply(m: &Mat3, v: [f64; 3]) -> [f64; 3] {
    [
        m[0][0] * v[0] + m[0][1] * v[1] + m[0][2] * v[2],
        m[1][0] * v[0] + m[1][1] * v[1] + m[1][2] * v[2],
        m[2][0] * v[0] + m[2][1] * v[1] + m[2][2] * v[2],
    ]
}

fn mat_transpose(m: &Mat3) -> Mat3 {
    let mut out = [[0.0_f64; 3]; 3];
    for (i, row) in out.iter_mut().enumerate() {
        for (j, cell) in row.iter_mut().enumerate() {
            *cell = m[j][i];
        }
    }
    out
}

fn precession_matrix_fw06(tdb_jd: f64) -> Option<Mat3> {
    if !tdb_jd.is_finite() {
        return None;
    }
    let t = (tdb_jd - 2451545.0) / 36525.0;
    let gamb = (-0.052928
        + (10.556378
            + (0.4932044 + (-0.00031238 + (-0.000002788 + 0.0000000260 * t) * t) * t) * t)
            * t)
        * ARCSEC_TO_RAD;
    let phib = (84381.448 + (-46.815758 + (-0.000590 + 0.001813 * t) * t) * t) * ARCSEC_TO_RAD;
    let psib = (-0.041775
        + (5038.481484
            + (1.5584175 + (-0.00018522 + (-0.000026452 + (-0.0000000148) * t) * t) * t) * t)
            * t)
        * ARCSEC_TO_RAD;
    let epsa = (84381.406
        + (-46.836769
            + (-0.0001831 + (0.00200340 + (-0.000000576 + (-0.0000000434) * t) * t) * t) * t)
            * t)
        * ARCSEC_TO_RAD;
    if !gamb.is_finite() || !phib.is_finite() || !psib.is_finite() || !epsa.is_finite() {
        return None;
    }
    let p = mat_mul(
        &rx(-epsa),
        &mat_mul(&rz(-psib), &mat_mul(&rx(phib), &rz(gamb))),
    );
    if p.iter().all(|row| row.iter().all(|c| c.is_finite())) {
        Some(p)
    } else {
        None
    }
}

pub fn cirs_to_gcrs(r_cirs_km: [f64; 3], tdb_jd: f64) -> Option<[f64; 3]> {
    if !finite3(r_cirs_km) {
        return None;
    }
    let p = precession_matrix_fw06(tdb_jd)?;
    let out = mat_apply(&mat_transpose(&p), r_cirs_km);
    if finite3(out) { Some(out) } else { None }
}

pub fn itrf_to_icrs(
    r_itrf_km: [f64; 3],
    ut1_jd: f64,
    tdb_jd: f64,
    eop: &EopSample,
) -> Option<[f64; 3]> {
    let r_cirs = itrf_to_cirs(r_itrf_km, ut1_jd, eop)?;
    cirs_to_gcrs(r_cirs, tdb_jd)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kepler::GM_SUN_M3_S2;

    const AU_KM: f64 = 1.495978707e8;

    fn circular_orbit() -> impl Fn(f64) -> Option<([f64; 3], [f64; 3])> {
        let r_m = AU_KM * 1000.0;
        let n = (GM_SUN_M3_S2 / r_m.powi(3)).sqrt();
        move |t: f64| {
            let theta = n * t;
            let pos = [AU_KM * theta.cos(), AU_KM * theta.sin(), 0.0];
            let vel = [-AU_KM * n * theta.sin(), AU_KM * n * theta.cos(), 0.0];
            Some((pos, vel))
        }
    }

    #[test]
    fn two_way_roundtrip_reads_twice_the_light_time() {
        let orbit = circular_orbit();
        let station = [0.0, -AU_KM, 0.0];
        let got = two_way_doppler(
            0.0,
            station,
            station,
            2.0e9,
            880.0 / 749.0,
            [0.0, 0.0, 0.0],
            GM_SUN_M3_S2,
            &orbit,
        )
        .unwrap();
        let (pos, _vel) = orbit(got.t_bounce_tdb).unwrap();
        let d_km = norm(sub(pos, station));
        let light_time = d_km * 1000.0 / C_LIGHT;
        assert!(
            (got.roundtrip_s - 2.0 * light_time).abs() < 1.0,
            "roundtrip {} vs 2*c tau {}",
            got.roundtrip_s,
            2.0 * light_time
        );
        assert!(got.roundtrip_s > 1300.0 && got.roundtrip_s < 1500.0);
    }

    #[test]
    fn range_rate_reads_the_line_of_sight_projection() {
        let orbit = circular_orbit();
        let station = [0.0, -AU_KM, 0.0];
        let got = two_way_doppler(
            0.0,
            station,
            station,
            2.0e9,
            880.0 / 749.0,
            [0.0, 0.0, 0.0],
            GM_SUN_M3_S2,
            orbit,
        )
        .unwrap();
        assert!(
            got.range_rate_m_s.abs() > 5000.0 && got.range_rate_m_s.abs() < 40000.0,
            "range rate {}",
            got.range_rate_m_s
        );
    }

    #[test]
    fn shapiro_reads_a_positive_small_delay() {
        let station = [AU_KM, 0.0, 0.0];
        let target = [-AU_KM, 1.0e6, 0.0];
        let dt = shapiro_leg_s(station, target, [0.0, 0.0, 0.0], GM_SUN_M3_S2).unwrap();
        assert!(dt > 0.0, "shapiro {} positive", dt);
        assert!(dt < 1.0e-3, "shapiro {} below a millisecond", dt);
        assert!(dt > 1.0e-6, "shapiro {} above a microsecond", dt);
    }

    #[test]
    fn shapiro_cuts_the_superior_conjunction() {
        let station = [AU_KM, 0.0, 0.0];
        let target = [-AU_KM, 0.0, 0.0];
        assert!(shapiro_leg_s(station, target, [0.0, 0.0, 0.0], GM_SUN_M3_S2).is_none());
    }

    #[test]
    fn two_way_refuses_absent_target_state() {
        let absent = |_t: f64| -> Option<([f64; 3], [f64; 3])> { None };
        let station = [0.0, -AU_KM, 0.0];
        assert!(
            two_way_doppler(
                0.0,
                station,
                station,
                2.0e9,
                880.0 / 749.0,
                [0.0, 0.0, 0.0],
                GM_SUN_M3_S2,
                absent
            )
            .is_none()
        );
    }

    #[test]
    fn light_time_solution_is_ordered() {
        let orbit = circular_orbit();
        let station = [0.0, -AU_KM, 0.0];
        let got = two_way_doppler(
            0.0,
            station,
            station,
            2.0e9,
            880.0 / 749.0,
            [0.0, 0.0, 0.0],
            GM_SUN_M3_S2,
            orbit,
        )
        .unwrap();
        assert!(
            got.t_tx_tdb < got.t_bounce_tdb && got.t_bounce_tdb < got.t_rx_tdb,
            "tx {} < bounce {} < rx {}",
            got.t_tx_tdb,
            got.t_bounce_tdb,
            got.t_rx_tdb
        );
    }

    #[test]
    fn two_way_reads_the_doppler_shift() {
        let orbit = circular_orbit();
        let station = [0.0, -AU_KM, 0.0];
        let got = two_way_doppler(
            0.0,
            station,
            station,
            2.0e9,
            880.0 / 749.0,
            [0.0, 0.0, 0.0],
            GM_SUN_M3_S2,
            orbit,
        )
        .unwrap();
        let nominal = 2.0e9 * 880.0 / 749.0;
        let shift = got.observable_hz - nominal;
        assert!(shift.abs() > 1.0e5 && shift.abs() < 1.0e8, "shift {shift}");
    }

    #[test]
    fn itrf_to_cirs_preserves_the_norm() {
        let station = [6378.137, 1000.0, -2000.0];
        let eop = EopSample {
            ut1_utc_s: 0.1,
            pm_x_arcsec: 0.2,
            pm_y_arcsec: -0.1,
        };
        let out = itrf_to_cirs(station, 2451545.0 + 0.5, &eop).unwrap();
        let d = (norm(out) - norm(station)).abs();
        assert!(d < 1.0e-9, "norm drift {d}");
    }

    #[test]
    fn itrf_to_cirs_identity_at_zero_polar_motion_and_zero_era() {
        let station = [6378.137, 1000.0, -2000.0];
        let out = itrf_to_cirs_at(0.0, 0.0, 0.0, station).unwrap();
        assert_eq!(out, station);
    }

    #[test]
    fn earth_rotation_angle_reads_the_j2000_constant() {
        let era = earth_rotation_angle_rad(2451545.0).unwrap();
        let want = 2.0 * std::f64::consts::PI * 0.7790572732640;
        assert!((era - want).abs() < 1.0e-15, "era {era} vs {want}");
        assert!((0.0..2.0 * std::f64::consts::PI).contains(&era));
    }

    #[test]
    fn itrf_to_cirs_refuses_absent_input() {
        let station = [6378.137, 1000.0, -2000.0];
        let eop = EopSample {
            ut1_utc_s: 0.0,
            pm_x_arcsec: 0.0,
            pm_y_arcsec: 0.0,
        };
        assert!(itrf_to_cirs(station, f64::NAN, &eop).is_none());
        assert!(earth_rotation_angle_rad(f64::INFINITY).is_none());
        let bad = EopSample {
            ut1_utc_s: 0.0,
            pm_x_arcsec: f64::NAN,
            pm_y_arcsec: 0.0,
        };
        assert!(itrf_to_cirs(station, 2451545.0, &bad).is_none());
        assert!(itrf_to_cirs([f64::NAN, 0.0, 0.0], 2451545.0, &eop).is_none());
    }

    #[test]
    fn cirs_to_gcrs_preserves_the_norm() {
        let station = [6378.137, 1000.0, -2000.0];
        let out = cirs_to_gcrs(station, 2460000.5).unwrap();
        let d = (norm(out) - norm(station)).abs();
        assert!(d < 1.0e-9, "norm drift {d}");
    }

    #[test]
    fn cirs_to_gcrs_is_the_identity_at_j2000_up_to_frame_bias() {
        let station = [6378.137, 1000.0, -2000.0];
        let out = cirs_to_gcrs(station, 2451545.0).unwrap();
        let sep = norm(sub(out, station));
        assert!(sep < 1.0e-2, "frame-bias separation {sep} km");
    }

    #[test]
    fn precession_matrix_is_orthogonal() {
        let p = precession_matrix_fw06(2460000.5).unwrap();
        let should_be_identity = mat_mul(&mat_transpose(&p), &p);
        for (i, row) in should_be_identity.iter().enumerate() {
            for (j, c) in row.iter().enumerate() {
                let want = if i == j { 1.0 } else { 0.0 };
                assert!((c - want).abs() < 1.0e-12, "P^T P [{i}][{j}] = {c}");
            }
        }
    }

    #[test]
    fn cirs_to_gcrs_refuses_absent_input() {
        assert!(cirs_to_gcrs([f64::NAN, 0.0, 0.0], 2451545.0).is_none());
        assert!(cirs_to_gcrs([0.0, f64::INFINITY, 0.0], 2451545.0).is_none());
        assert!(cirs_to_gcrs([6378.137, 1000.0, -2000.0], f64::NAN).is_none());
        assert!(cirs_to_gcrs([6378.137, 1000.0, -2000.0], f64::INFINITY).is_none());
    }

    #[test]
    fn itrf_to_icrs_composes_through_cirs() {
        let station = [6378.137, 1000.0, -2000.0];
        let eop = EopSample {
            ut1_utc_s: 0.1,
            pm_x_arcsec: 0.2,
            pm_y_arcsec: -0.1,
        };
        let out = itrf_to_icrs(station, 2451545.0 + 0.5, 2451545.0 + 0.5, &eop).unwrap();
        let d = (norm(out) - norm(station)).abs();
        assert!(d < 1.0e-9, "norm drift {d}");
        assert!(itrf_to_icrs(station, f64::NAN, 2451545.0, &eop).is_none());
        assert!(itrf_to_icrs(station, 2451545.0, f64::NAN, &eop).is_none());
    }
}
