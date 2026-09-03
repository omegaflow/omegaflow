use std::collections::HashMap;

use crate::archivar::{
    body_barycenter_position, body_barycenter_velocity, body_fixed_to_icrs_smooth, BodyEphemeris,
};

pub const C: f64 = 299792458.0;
pub const SUN_MU: f64 = 1.32712440018e20;
pub const EARTH: &str = "earth";

pub fn dsn_station(id: i64) -> Option<(f64, f64, f64)> {
    match id {
        11 => Some((35.3892806, -116.8561972, 900.0)),
        12 => Some((35.1638472, -116.7898472, 1006.5)),
        14 => Some((35.4268333, -116.8900000, 1001.5)),
        42 => Some((-35.4009889, 148.9772778, 702.0)),
        43 => Some((-35.4014889, 148.9816167, 692.5)),
        44 => Some((-35.5836, 148.977, 1116.0)),
        61 => Some((40.4318611, -4.2486111, 812.0)),
        63 => Some((40.4312500, -4.2487778, 865.0)),
        _ => None,
    }
}

fn dist(a: [f64; 3], b: [f64; 3]) -> f64 {
    let dx = a[0] - b[0];
    let dy = a[1] - b[1];
    let dz = a[2] - b[2];
    (dx * dx + dy * dy + dz * dz).sqrt()
}

fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

pub fn station_velocity(
    t: f64,
    lat: f64,
    lon: f64,
    alt: f64,
    eph: &HashMap<String, BodyEphemeris>,
) -> Option<[f64; 3]> {
    let dt = 60.0;
    let r_plus = body_fixed_to_icrs_smooth(EARTH, lat, lon, alt, t + dt, eph)?;
    let r_minus = body_fixed_to_icrs_smooth(EARTH, lat, lon, alt, t - dt, eph)?;
    let e_plus = body_barycenter_position(EARTH, t + dt, eph)?;
    let e_minus = body_barycenter_position(EARTH, t - dt, eph)?;
    let v_earth = body_barycenter_velocity(EARTH, t, eph)?;
    let mut v = [0.0; 3];
    for k in 0..3 {
        v[k] = v_earth[k] + ((r_plus[k] - e_plus[k]) - (r_minus[k] - e_minus[k])) / (2.0 * dt);
    }
    Some(v)
}

pub fn downlink_rate(
    t1: f64,
    lat: f64,
    lon: f64,
    alt: f64,
    eph: &HashMap<String, BodyEphemeris>,
    sc: &dyn Fn(f64) -> Option<([f64; 3], [f64; 3])>,
) -> Option<f64> {
    let r_st1 = body_fixed_to_icrs_smooth(EARTH, lat, lon, alt, t1, eph)?;
    let v_st1 = station_velocity(t1, lat, lon, alt, eph)?;
    downlink_rate_core(t1, r_st1, v_st1, sc)
}

pub fn downlink_rate_core(
    t1: f64,
    r_st1: [f64; 3],
    v_st1: [f64; 3],
    sc: &dyn Fn(f64) -> Option<([f64; 3], [f64; 3])>,
) -> Option<f64> {
    let mut t3 = t1;
    for _ in 0..6 {
        let (r_sc3, _) = sc(t3)?;
        let rho_down = dist(r_st1, r_sc3);
        let t3_new = t1 - rho_down / C;
        if (t3_new - t3).abs() < 1e-9 {
            t3 = t3_new;
            break;
        }
        t3 = t3_new;
    }
    let (r_sc3, v_sc3) = sc(t3)?;
    let rho_down = dist(r_st1, r_sc3);
    if rho_down <= 0.0 {
        return None;
    }
    let mut d_down = [0.0; 3];
    let mut v_down = [0.0; 3];
    for k in 0..3 {
        d_down[k] = r_st1[k] - r_sc3[k];
        v_down[k] = v_st1[k] - v_sc3[k];
    }
    Some(dot(d_down, v_down) / rho_down)
}

pub fn sun_accel(r: [f64; 3]) -> [f64; 3] {
    let r2 = r[0] * r[0] + r[1] * r[1] + r[2] * r[2];
    let rn = r2.sqrt();
    let k = -SUN_MU / (r2 * rn);
    [k * r[0], k * r[1], k * r[2]]
}

pub fn accel(r: [f64; 3], a_p: f64) -> [f64; 3] {
    let r2 = r[0] * r[0] + r[1] * r[1] + r[2] * r[2];
    let rn = r2.sqrt();
    let k = -(SUN_MU / (r2 * rn) + a_p / rn);
    [k * r[0], k * r[1], k * r[2]]
}

pub fn rk4_step(s: [f64; 6], dt: f64, a_p: f64) -> [f64; 6] {
    let f = |sv: [f64; 6]| -> [f64; 6] {
        let a = accel([sv[0], sv[1], sv[2]], a_p);
        [sv[3], sv[4], sv[5], a[0], a[1], a[2]]
    };
    let k1 = f(s);
    let s2 = [
        s[0] + 0.5 * dt * k1[0],
        s[1] + 0.5 * dt * k1[1],
        s[2] + 0.5 * dt * k1[2],
        s[3] + 0.5 * dt * k1[3],
        s[4] + 0.5 * dt * k1[4],
        s[5] + 0.5 * dt * k1[5],
    ];
    let k2 = f(s2);
    let s3 = [
        s[0] + 0.5 * dt * k2[0],
        s[1] + 0.5 * dt * k2[1],
        s[2] + 0.5 * dt * k2[2],
        s[3] + 0.5 * dt * k2[3],
        s[4] + 0.5 * dt * k2[4],
        s[5] + 0.5 * dt * k2[5],
    ];
    let k3 = f(s3);
    let s4 = [
        s[0] + dt * k3[0],
        s[1] + dt * k3[1],
        s[2] + dt * k3[2],
        s[3] + dt * k3[3],
        s[4] + dt * k3[4],
        s[5] + dt * k3[5],
    ];
    let k4 = f(s4);
    let mut out = [0.0; 6];
    for i in 0..6 {
        out[i] = s[i] + dt / 6.0 * (k1[i] + 2.0 * k2[i] + 2.0 * k3[i] + k4[i]);
    }
    out
}

pub fn propagate_grid(
    state0: [f64; 6],
    t0: f64,
    t1: f64,
    a_p: f64,
    dt: f64,
) -> Vec<(f64, [f64; 6])> {
    let dir = if t1 >= t0 { 1.0 } else { -1.0 };
    let step = dt * dir;
    let mut grid = Vec::new();
    let mut t = t0;
    let mut s = state0;
    grid.push((t, s));
    while (t - t1) * dir < 0.0 {
        let dt_eff = if (t + step - t1) * dir > 0.0 {
            t1 - t
        } else {
            step
        };
        s = rk4_step(s, dt_eff, a_p);
        t += dt_eff;
        grid.push((t, s));
    }
    grid
}

pub fn rk4_accel_step(
    s: [f64; 6],
    t: f64,
    dt: f64,
    a: &dyn Fn(f64, [f64; 3]) -> [f64; 3],
) -> [f64; 6] {
    let f = |tt: f64, sv: [f64; 6]| -> [f64; 6] {
        let acc = a(tt, [sv[0], sv[1], sv[2]]);
        [sv[3], sv[4], sv[5], acc[0], acc[1], acc[2]]
    };
    let k1 = f(t, s);
    let s2 = [
        s[0] + 0.5 * dt * k1[0],
        s[1] + 0.5 * dt * k1[1],
        s[2] + 0.5 * dt * k1[2],
        s[3] + 0.5 * dt * k1[3],
        s[4] + 0.5 * dt * k1[4],
        s[5] + 0.5 * dt * k1[5],
    ];
    let k2 = f(t + 0.5 * dt, s2);
    let s3 = [
        s[0] + 0.5 * dt * k2[0],
        s[1] + 0.5 * dt * k2[1],
        s[2] + 0.5 * dt * k2[2],
        s[3] + 0.5 * dt * k2[3],
        s[4] + 0.5 * dt * k2[4],
        s[5] + 0.5 * dt * k2[5],
    ];
    let k3 = f(t + 0.5 * dt, s3);
    let s4 = [
        s[0] + dt * k3[0],
        s[1] + dt * k3[1],
        s[2] + dt * k3[2],
        s[3] + dt * k3[3],
        s[4] + dt * k3[4],
        s[5] + dt * k3[5],
    ];
    let k4 = f(t + dt, s4);
    let mut out = [0.0; 6];
    for i in 0..6 {
        out[i] = s[i] + dt / 6.0 * (k1[i] + 2.0 * k2[i] + 2.0 * k3[i] + k4[i]);
    }
    out
}

pub fn propagate_accel(
    state0: [f64; 6],
    t0: f64,
    t1: f64,
    dt: f64,
    a: &dyn Fn(f64, [f64; 3]) -> [f64; 3],
) -> Vec<(f64, [f64; 6])> {
    let dir = if t1 >= t0 { 1.0 } else { -1.0 };
    let step = dt * dir;
    let mut grid = Vec::new();
    let mut t = t0;
    let mut s = state0;
    grid.push((t, s));
    while (t - t1) * dir < 0.0 {
        let dt_eff = if (t + step - t1) * dir > 0.0 {
            t1 - t
        } else {
            step
        };
        s = rk4_accel_step(s, t, dt_eff, a);
        t += dt_eff;
        grid.push((t, s));
    }
    grid
}

pub fn interp(grid: &[(f64, [f64; 6])], t: f64) -> Option<[f64; 6]> {
    if grid.len() < 2 {
        return None;
    }
    if t <= grid[0].0 {
        return Some(grid[0].1);
    }
    let last = grid[grid.len() - 1].0;
    if t >= last {
        return Some(grid[grid.len() - 1].1);
    }
    let idx = grid.partition_point(|(tt, _)| *tt < t);
    let (t0, s0) = grid[idx - 1];
    let (t1, s1) = grid[idx];
    let w = (t - t0) / (t1 - t0);
    let mut out = [0.0; 6];
    for i in 0..6 {
        out[i] = s0[i] + w * (s1[i] - s0[i]);
    }
    Some(out)
}
