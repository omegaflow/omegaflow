use crate::archivar::types::C_LIGHT;
use crate::kepler::GM_SUN_M3_S2;

pub const SUN_ICRS_KM: [f64; 3] = [0.0, 0.0, 0.0];

pub const TWO_WAY_DOPPLER_CONVENTION: &str = "t_bounce from classical downlink light-time (<=10 steps, 1e-12 s); t_tx from classical uplink light-time against the bounce state; roundtrip_s = t_rx - t_tx + shapiro_s; range_rate_m_s is the downlink line-of-sight rate (positive receding), m/s; downlink_hz = f_ref * turn_ratio * (1 - rho_dot_dn/c); uplink_hz = f_ref * (1 - rho_dot_up/c); observable_hz = f_ref * turn_ratio * (1 - rho_dot_up/c) * (1 - rho_dot_dn/c); the Sun rests at the ICRS origin";

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

pub fn shapiro_sun_leg_s(station_icrs_km: [f64; 3], target_icrs_km: [f64; 3]) -> Option<f64> {
    if !finite3(station_icrs_km) || !finite3(target_icrs_km) {
        return None;
    }
    let r1 = norm(sub(station_icrs_km, SUN_ICRS_KM)) * 1000.0;
    let r2 = norm(sub(target_icrs_km, SUN_ICRS_KM)) * 1000.0;
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
    let dt = 2.0 * GM_SUN_M3_S2 / (C_LIGHT * C_LIGHT * C_LIGHT) * arg.ln();
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
    let shapiro_s =
        shapiro_sun_leg_s(station_rx_icrs_km, pos)? + shapiro_sun_leg_s(station_tx_icrs_km, pos)?;
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

#[cfg(test)]
mod tests {
    use super::*;

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
        let got = two_way_doppler(0.0, station, station, 2.0e9, 880.0 / 749.0, orbit).unwrap();
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
        let got = two_way_doppler(0.0, station, station, 2.0e9, 880.0 / 749.0, orbit).unwrap();
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
        let dt = shapiro_sun_leg_s(station, target).unwrap();
        assert!(dt > 0.0, "shapiro {} positive", dt);
        assert!(dt < 1.0e-3, "shapiro {} below a millisecond", dt);
        assert!(dt > 1.0e-6, "shapiro {} above a microsecond", dt);
    }

    #[test]
    fn shapiro_cuts_the_superior_conjunction() {
        let station = [AU_KM, 0.0, 0.0];
        let target = [-AU_KM, 0.0, 0.0];
        assert!(shapiro_sun_leg_s(station, target).is_none());
    }

    #[test]
    fn two_way_refuses_absent_target_state() {
        let absent = |_t: f64| -> Option<([f64; 3], [f64; 3])> { None };
        let station = [0.0, -AU_KM, 0.0];
        assert!(two_way_doppler(0.0, station, station, 2.0e9, 880.0 / 749.0, absent).is_none());
    }

    #[test]
    fn light_time_solution_is_ordered() {
        let orbit = circular_orbit();
        let station = [0.0, -AU_KM, 0.0];
        let got = two_way_doppler(0.0, station, station, 2.0e9, 880.0 / 749.0, orbit).unwrap();
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
        let got = two_way_doppler(0.0, station, station, 2.0e9, 880.0 / 749.0, orbit).unwrap();
        let nominal = 2.0e9 * 880.0 / 749.0;
        let shift = got.observable_hz - nominal;
        assert!(shift.abs() > 1.0e5 && shift.abs() < 1.0e8, "shift {shift}");
    }
}
