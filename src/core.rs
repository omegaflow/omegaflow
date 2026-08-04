use std::collections::HashMap;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

pub const Φ: f64 = 1.618033988749895;
pub const AU: f64 = 1.495978707e11;
pub const EARTH_RADIUS: f64 = 6378137.0;
pub const MARS_RADIUS: f64 = 3396190.0;
pub const EARTH_ECC: f64 = 0.0167086;
pub const ECLIPTIC_OBLIQUITY: f64 = 0.409092804;
pub const J2000_EPOCH: f64 = 2451545.0;
pub const UNIX_J2000_OFFSET: f64 = 946728000.0;
pub const GAUSS_K: f64 = 0.01720209895;
pub const PARSEC_M: f64 = 3.085677581e16;
pub const C_LIGHT: f64 = 299792458.0;
pub const HUBBLE_H0: f64 = 70000.0 / (PARSEC_M * 1.0e6);
pub const MAS_YR_TO_RAD_S: f64 = 4.84813681109536e-9 / 31557600.0;

pub const V_SOUND_288: f64 = 343.0;
pub const V_P_GRANITE: f64 = 5900.0;
pub const V_S_GRANITE: f64 = 2930.0;
pub const D_AIR: f64 = 2.0e-5;
pub const ALPHA_AIR: f64 = 2.18e-5;

pub fn compute_gmst(tdb_secs: f64) -> f64 {
    let jd = tdb_secs / 86400.0 + J2000_EPOCH;
    let t = (jd - J2000_EPOCH) / 36525.0;
    let gmst = 280.46061837 + 360.98564736629 * (jd - J2000_EPOCH) + 0.000387933 * t * t
        - t * t * t / 38710000.0;
    (gmst % 360.0) * std::f64::consts::PI / 180.0
}

pub fn geodetic_to_ecef(lat: f64, lon: f64, alt: f64) -> (f64, f64, f64) {
    let lat_r = lat * std::f64::consts::PI / 180.0;
    let lon_r = lon * std::f64::consts::PI / 180.0;
    const WGS84_F: f64 = 1.0 / 298.257223563;
    let e2 = WGS84_F * (2.0 - WGS84_F);
    let sin_lat = lat_r.sin();
    let n = EARTH_RADIUS / (1.0 - e2 * sin_lat * sin_lat).sqrt();
    (
        (n + alt) * lat_r.cos() * lon_r.cos(),
        (n + alt) * lat_r.cos() * lon_r.sin(),
        (n * (1.0 - e2) + alt) * sin_lat,
    )
}

pub fn tdb_to_jd(tdb_secs: f64) -> f64 {
    tdb_secs / 86400.0 + J2000_EPOCH
}

pub fn earth_position_icrs(tdb_secs: f64) -> (f64, f64, f64) {
    let jd = tdb_to_jd(tdb_secs);
    let t = (jd - J2000_EPOCH) / 36525.0;
    let m = 6.239996 + 0.017201969 * t * 36525.0;
    let e = EARTH_ECC;
    let mut e_anom = m;
    for _ in 0..5 {
        e_anom = e_anom - (e_anom - e * e_anom.sin() - m) / (1.0 - e * e_anom.cos());
    }
    let x_orb = AU * (e_anom.cos() - e);
    let y_orb = AU * (1.0 - e * e).sqrt() * e_anom.sin();
    let omega: f64 = -0.113;
    let x_ecl = x_orb * omega.cos() - y_orb * omega.sin();
    let y_ecl = x_orb * omega.sin() + y_orb * omega.cos();
    let x_icrs = x_ecl;
    let y_icrs = y_ecl * ECLIPTIC_OBLIQUITY.cos();
    let z_icrs = y_ecl * ECLIPTIC_OBLIQUITY.sin();
    (x_icrs, y_icrs, z_icrs)
}

pub fn mars_position_icrs(tdb_secs: f64) -> (f64, f64, f64) {
    let jd = tdb_to_jd(tdb_secs);
    let m = 0.338403 + 0.00914587 * (jd - J2000_EPOCH);
    let e = 0.09340062;
    let mut e_anom = m;
    for _ in 0..5 {
        e_anom = e_anom - (e_anom - e * e_anom.sin() - m) / (1.0 - e * e_anom.cos());
    }
    let a = AU * 1.523679;
    let x_orb = a * (e_anom.cos() - e);
    let y_orb = a * (1.0 - e * e).sqrt() * e_anom.sin();
    let omega: f64 = 5.0019;
    let x_ecl = x_orb * omega.cos() - y_orb * omega.sin();
    let y_ecl = x_orb * omega.sin() + y_orb * omega.cos();
    let x_icrs = x_ecl;
    let y_icrs = y_ecl * ECLIPTIC_OBLIQUITY.cos();
    let z_icrs = y_ecl * ECLIPTIC_OBLIQUITY.sin();
    (x_icrs, y_icrs, z_icrs)
}

pub fn geodetic_to_icrs(lat: f64, lon: f64, alt: f64, tdb_secs: f64) -> (f64, f64, f64) {
    let (x_ecef, y_ecef, z_ecef) = geodetic_to_ecef(lat, lon, alt);
    let gmst_rad = compute_gmst(tdb_secs);
    let x_eci = x_ecef * gmst_rad.cos() + y_ecef * gmst_rad.sin();
    let y_eci = -x_ecef * gmst_rad.sin() + y_ecef * gmst_rad.cos();
    let z_eci = z_ecef;
    let x_ecl = x_eci;
    let y_ecl = y_eci * ECLIPTIC_OBLIQUITY.cos() + z_eci * ECLIPTIC_OBLIQUITY.sin();
    let z_ecl = -y_eci * ECLIPTIC_OBLIQUITY.sin() + z_eci * ECLIPTIC_OBLIQUITY.cos();
    let (ex, ey, ez) = earth_position_icrs(tdb_secs);
    (x_ecl + ex, y_ecl + ey, z_ecl + ez)
}

pub fn iau2000_to_icrs(lat: f64, lon: f64, alt: f64, tdb_secs: f64) -> (f64, f64, f64) {
    let lat_rad = lat.to_radians();
    let lon_rad = lon.to_radians();
    let r = MARS_RADIUS + alt;
    let x_body = r * lat_rad.cos() * lon_rad.cos();
    let y_body = r * lat_rad.cos() * lon_rad.sin();
    let z_body = r * lat_rad.sin();
    let mars_day = 88642.66;
    let rot = (tdb_secs / mars_day).fract() * std::f64::consts::TAU;
    let x_eci = x_body * rot.cos() + y_body * rot.sin();
    let y_eci = -x_body * rot.sin() + y_body * rot.cos();
    let z_eci = z_body;
    let x_ecl = x_eci;
    let y_ecl = y_eci * ECLIPTIC_OBLIQUITY.cos() + z_eci * ECLIPTIC_OBLIQUITY.sin();
    let z_ecl = -y_eci * ECLIPTIC_OBLIQUITY.sin() + z_eci * ECLIPTIC_OBLIQUITY.cos();
    let (mx, my, mz) = mars_position_icrs(tdb_secs);
    (x_ecl + mx, y_ecl + my, z_ecl + mz)
}

pub fn icrs_to_geodetic(x: f64, y: f64, z: f64, tdb_secs: f64) -> (f64, f64) {
    let (ex, ey, ez) = earth_position_icrs(tdb_secs);
    let x_ecl = x - ex;
    let y_ecl = y - ey;
    let z_ecl = z - ez;
    let x_eci = x_ecl;
    let y_eci = y_ecl * ECLIPTIC_OBLIQUITY.cos() - z_ecl * ECLIPTIC_OBLIQUITY.sin();
    let z_eci = y_ecl * ECLIPTIC_OBLIQUITY.sin() + z_ecl * ECLIPTIC_OBLIQUITY.cos();
    let jd = tdb_to_jd(tdb_secs);
    let t = (jd - J2000_EPOCH) / 36525.0;
    let gmst = 280.46061837 + 360.98564736629 * (jd - J2000_EPOCH) + 0.000387933 * t * t
        - t * t * t / 38710000.0;
    let gmst_rad = (gmst % 360.0) * std::f64::consts::PI / 180.0;
    let x_ecef = x_eci * gmst_rad.cos() - y_eci * gmst_rad.sin();
    let y_ecef = x_eci * gmst_rad.sin() + y_eci * gmst_rad.cos();
    let z_ecef = z_eci;
    let lon = y_ecef.atan2(x_ecef).to_degrees();
    let lat = z_ecef
        .atan2((x_ecef * x_ecef + y_ecef * y_ecef).sqrt())
        .to_degrees();
    (lat, lon)
}

pub type CellKey = (i64, i64, i64);
pub type Origin = (u32, i32, i32);

#[derive(Clone, Copy)]
pub enum Motion {
    Ecliptic { scale: f64 },
    Areocentric { scale: f64 },
    WGS84 { lat: f64, lon: f64, alt: f64 },
    IAU2000 { lat: f64, lon: f64, alt: f64 },
    Linear { p: [f64; 3], v: [f64; 3] },
}

impl Motion {
    pub fn at(&self, t: f64, epoch: f64) -> [f64; 3] {
        match self {
            Motion::WGS84 { lat, lon, alt } => {
                let (x, y, z) = geodetic_to_icrs(*lat, *lon, *alt, t);
                [x, y, z]
            }
            Motion::IAU2000 { lat, lon, alt } => {
                let (x, y, z) = iau2000_to_icrs(*lat, *lon, *alt, t);
                [x, y, z]
            }
            Motion::Linear { p, v } => {
                let dt = t - epoch;
                [p[0] + v[0] * dt, p[1] + v[1] * dt, p[2] + v[2] * dt]
            }
            Motion::Ecliptic { scale } => {
                let (x, y, z) = earth_position_icrs(t);
                [x * scale, y * scale, z * scale]
            }
            Motion::Areocentric { scale } => {
                let (x, y, z) = mars_position_icrs(t);
                [x * scale, y * scale, z * scale]
            }
        }
    }
    pub fn planet_bound(&self) -> bool {
        matches!(
            self,
            Motion::WGS84 { .. }
                | Motion::IAU2000 { .. }
                | Motion::Ecliptic { .. }
                | Motion::Areocentric { .. }
        )
    }
}

#[derive(Clone)]
pub struct Sample {
    pub origin: Origin,
    pub epoch: f64,
    pub ttl: f64,
    pub extent: f64,
    pub tau: f64,
    pub force_type: f64,
    pub vmax: f64,
    pub amax: f64,
    pub p0f: [f64; 3],
    pub motion: Motion,
    pub fields: Vec<(String, f64)>,
}

pub struct Family {
    pub cell_size: f64,
    pub vmax: f64,
    pub amax: f64,
    pub rmax: f64,
    pub epoch_min: f64,
    pub cell_lo: CellKey,
    pub cell_hi: CellKey,
    pub cells: HashMap<CellKey, Vec<Sample>>,
}

pub struct Buffer {
    pub planet: Family,
    pub inertial: Family,
}

pub fn region_quantize(deg: f64, extent: f64) -> i32 {
    (deg * 111319.0 / extent).round() as i32
}

pub fn cell_of(p: [f64; 3], s: f64) -> CellKey {
    (
        (p[0] / s).floor() as i64,
        (p[1] / s).floor() as i64,
        (p[2] / s).floor() as i64,
    )
}

pub fn relative_frame_position(motion: &Motion, t: f64, epoch: f64) -> [f64; 3] {
    let p = motion.at(t, epoch);
    if motion.planet_bound() {
        let (ex, ey, ez) = earth_position_icrs(t);
        [p[0] - ex, p[1] - ey, p[2] - ez]
    } else {
        p
    }
}

pub fn law_bounds(motion: &Motion, epoch: f64, resid_ema: f64) -> (f64, f64, [f64; 3]) {
    let p0 = relative_frame_position(motion, epoch, epoch);
    let p1 = relative_frame_position(motion, epoch + 1.0, epoch);
    let p2 = relative_frame_position(motion, epoch + 2.0, epoch);
    let v = ((p1[0] - p0[0]).powi(2) + (p1[1] - p0[1]).powi(2) + (p1[2] - p0[2]).powi(2)).sqrt();
    let a = ((p2[0] - 2.0 * p1[0] + p0[0]).powi(2)
        + (p2[1] - 2.0 * p1[1] + p0[1]).powi(2)
        + (p2[2] - 2.0 * p1[2] + p0[2]).powi(2))
    .sqrt();
    (Φ * (v + resid_ema), Φ * a, p0)
}

pub fn build_family(samples: Vec<Sample>, cadence: f64) -> Family {
    let mut vmax = 0.0f64;
    let mut amax = 0.0f64;
    let mut rmax = 0.0f64;
    let mut epoch_min = f64::MAX;
    for s in &samples {
        vmax = vmax.max(s.vmax);
        amax = amax.max(s.amax);
        rmax = rmax.max(s.extent);
        epoch_min = epoch_min.min(s.epoch);
    }
    let rho_cad = rmax + vmax * cadence + 0.5 * amax * cadence * cadence;
    let shift = (2.0 * rho_cad).log2().ceil().clamp(0.0, 63.0) as i32;
    let cell_size = 2f64.powi(shift);
    let mut cells: HashMap<CellKey, Vec<Sample>> = HashMap::new();
    let mut cell_lo = (i64::MAX, i64::MAX, i64::MAX);
    let mut cell_hi = (i64::MIN, i64::MIN, i64::MIN);
    for s in samples {
        let c = cell_of(s.p0f, cell_size);
        cell_lo.0 = cell_lo.0.min(c.0);
        cell_lo.1 = cell_lo.1.min(c.1);
        cell_lo.2 = cell_lo.2.min(c.2);
        cell_hi.0 = cell_hi.0.max(c.0);
        cell_hi.1 = cell_hi.1.max(c.1);
        cell_hi.2 = cell_hi.2.max(c.2);
        cells.entry(c).or_default().push(s);
    }
    Family {
        cell_size,
        vmax,
        amax,
        rmax,
        epoch_min: if epoch_min == f64::MAX {
            0.0
        } else {
            epoch_min
        },
        cell_lo,
        cell_hi,
        cells,
    }
}

pub fn build_buffer(samples: Vec<Sample>, cadence: f64) -> Buffer {
    let (planet_samples, inertial): (Vec<Sample>, Vec<Sample>) =
        samples.into_iter().partition(|s| s.motion.planet_bound());
    Buffer {
        planet: build_family(planet_samples, cadence),
        inertial: build_family(inertial, cadence),
    }
}

pub fn force_constants_by_id(id: f64) -> Option<(f64, bool)> {
    match id as u8 {
        0 => Some((C_LIGHT, false)),
        1 => Some((C_LIGHT, false)),
        2 => Some((V_SOUND_288, false)),
        3 => Some((V_P_GRANITE, false)),
        4 => Some((V_S_GRANITE, false)),
        5 => Some((ALPHA_AIR, true)),
        6 => Some((D_AIR, true)),
        7 => Some((10.0, false)),
        _ => None,
    }
}

pub fn enclose_family(
    fam: &Family,
    anchor: [f64; 3],
    center: [f64; 3],
    t2: f64,
    pad: f64,
    records: &mut Vec<(f64, f64, f64, f64, f64, f64, f64, f64, f64)>,
    _frustum: Option<([f64; 3], [f64; 3], [f64; 3], f64, f64)>,
) {
    if fam.cells.is_empty() {
        return;
    }
    let qf = [
        center[0] - anchor[0],
        center[1] - anchor[1],
        center[2] - anchor[2],
    ];
    let dt = (t2 - fam.epoch_min).abs();
    let rho = fam.rmax + fam.vmax * dt + 0.5 * fam.amax * dt * dt + pad;
    let s = fam.cell_size;
    let qlo = cell_of([qf[0] - rho, qf[1] - rho, qf[2] - rho], s);
    let qhi = cell_of([qf[0] + rho, qf[1] + rho, qf[2] + rho], s);
    let lo = (
        qlo.0.max(fam.cell_lo.0),
        qlo.1.max(fam.cell_lo.1),
        qlo.2.max(fam.cell_lo.2),
    );
    let hi = (
        qhi.0.min(fam.cell_hi.0),
        qhi.1.min(fam.cell_hi.1),
        qhi.2.min(fam.cell_hi.2),
    );
    if lo.0 > hi.0 || lo.1 > hi.1 || lo.2 > hi.2 {
        return;
    }
    let span = ((hi.0 - lo.0 + 1) as u64)
        .saturating_mul((hi.1 - lo.1 + 1) as u64)
        .saturating_mul((hi.2 - lo.2 + 1) as u64);
    let visit: Vec<&Vec<Sample>> = if span > fam.cells.len() as u64 * 4 {
        fam.cells
            .iter()
            .filter(|(ck, _)| {
                ck.0 >= lo.0
                    && ck.0 <= hi.0
                    && ck.1 >= lo.1
                    && ck.1 <= hi.1
                    && ck.2 >= lo.2
                    && ck.2 <= hi.2
            })
            .map(|(_, v)| v)
            .collect()
    } else {
        let mut out = Vec::new();
        for cx in lo.0..=hi.0 {
            for cy in lo.1..=hi.1 {
                for cz in lo.2..=hi.2 {
                    if let Some(samples) = fam.cells.get(&(cx, cy, cz)) {
                        out.push(samples);
                    }
                }
            }
        }
        out
    };
    for samples in visit {
        for smp in samples {
            let age = (t2 - smp.epoch).abs();
            let causal_reach =
                if let Some((v_or_d, is_diff)) = force_constants_by_id(smp.force_type) {
                    let lifetime = smp.ttl;
                    if is_diff {
                        (2.0 * v_or_d * lifetime).sqrt()
                    } else {
                        v_or_d * lifetime
                    }
                } else {
                    0.0
                };
            let reach =
                smp.extent.max(causal_reach) + smp.vmax * age + 0.5 * smp.amax * age * age + pad;
            let dx = smp.p0f[0] - qf[0];
            let dy = smp.p0f[1] - qf[1];
            let dz = smp.p0f[2] - qf[2];
            let dist2_p0f = dx * dx + dy * dy + dz * dz;
            if dist2_p0f > reach * reach {
                continue;
            }
            if let Some((_v_or_d, _is_diff)) = force_constants_by_id(smp.force_type) {
                if smp.tau > 0.0 && age > smp.tau * 64.0 {
                    continue;
                }
            }
            let p = smp.motion.at(t2, smp.epoch);
            let ddx = p[0] - center[0];
            let ddy = p[1] - center[1];
            let ddz = p[2] - center[2];
            let exact = smp.extent + pad;
            let dist2 = ddx * ddx + ddy * ddy + ddz * ddz;
            if dist2 > exact * exact {
                continue;
            }
            if let Some((_, val)) = smp.fields.iter().find(|(n, _)| !n.starts_with('_')) {
                records.push((
                    p[0],
                    p[1],
                    p[2],
                    *val,
                    smp.extent,
                    smp.epoch,
                    smp.ttl,
                    smp.tau,
                    smp.force_type,
                ));
            }
        }
    }
}

pub fn sense_buffer(
    buf: &Buffer,
    center: [f64; 3],
    t2: f64,
    pad: f64,
    records: &mut Vec<(f64, f64, f64, f64, f64, f64, f64, f64, f64)>,
    _frustum: Option<([f64; 3], [f64; 3], [f64; 3], f64, f64)>,
) {
    let (ex, ey, ez) = earth_position_icrs(t2);
    enclose_family(
        &buf.planet,
        [ex, ey, ez],
        center,
        t2,
        pad,
        records,
        _frustum,
    );
    enclose_family(
        &buf.inertial,
        [0.0, 0.0, 0.0],
        center,
        t2,
        pad,
        records,
        _frustum,
    );
}

pub fn ecliptic_to_field(v: [f64; 3]) -> [f64; 3] {
    let (c, s) = (ECLIPTIC_OBLIQUITY.cos(), ECLIPTIC_OBLIQUITY.sin());
    [v[0], v[1] * c - v[2] * s, v[1] * s + v[2] * c]
}

pub fn ecef_vec_to_field(v: [f64; 3], t: f64) -> [f64; 3] {
    let g = compute_gmst(t);
    let (cg, sg) = (g.cos(), g.sin());
    let x = v[0] * cg + v[1] * sg;
    let y = -v[0] * sg + v[1] * cg;
    let (c, s) = (ECLIPTIC_OBLIQUITY.cos(), ECLIPTIC_OBLIQUITY.sin());
    [x, y * c + v[2] * s, -y * s + v[2] * c]
}

pub fn flow_motion(
    lat: f64,
    lon: f64,
    alt: f64,
    speed: f64,
    track: f64,
    vrate: f64,
    t: f64,
) -> Motion {
    let p = geodetic_to_icrs(lat, lon, alt, t);
    let pn = geodetic_to_icrs(lat, lon, alt, t + 1.0);
    let v_frame = [pn.0 - p.0, pn.1 - p.1, pn.2 - p.2];
    let latr = lat.to_radians();
    let lonr = lon.to_radians();
    let trk = track.to_radians();
    let v_e = speed * trk.sin();
    let v_n = speed * trk.cos();
    let v_ecef = [
        -v_e * lonr.sin() - v_n * latr.sin() * lonr.cos() + vrate * latr.cos() * lonr.cos(),
        v_e * lonr.cos() - v_n * latr.sin() * lonr.sin() + vrate * latr.cos() * lonr.sin(),
        v_n * latr.cos() + vrate * latr.sin(),
    ];
    let v_rot = ecef_vec_to_field(v_ecef, t);
    Motion::Linear {
        p: [p.0, p.1, p.2],
        v: [
            v_frame[0] + v_rot[0],
            v_frame[1] + v_rot[1],
            v_frame[2] + v_rot[2],
        ],
    }
}

pub fn horizons_nums(line: &str, keys: [&str; 3]) -> Option<[f64; 3]> {
    let mut out = [0.0; 3];
    for (i, k) in keys.iter().enumerate() {
        let p = line.find(k)?;
        let r = line[p + k.len()..].trim_start_matches(|c: char| c == '=' || c == ' ' || c == '\t');
        let end = r.find(|c: char| c.is_whitespace()).unwrap_or(r.len());
        out[i] = r[..end].parse().ok()?;
    }
    Some(out)
}

pub fn tdb_now() -> f64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs_f64()
        - UNIX_J2000_OFFSET
}

pub fn parse_iso_tdb(s: &str) -> Option<f64> {
    let s = s.trim();
    let (date, time) = s.split_once('T')?;
    let mut dp = date.split('-');
    let y: i64 = dp.next()?.parse().ok()?;
    let m: u32 = dp.next()?.parse().ok()?;
    let d: u32 = dp.next()?.parse().ok()?;
    let t = time
        .split(|c: char| c == '.' || c == 'Z' || c == 'z')
        .next()
        .unwrap_or(time);
    let mut tp = t.split(':');
    let hh: u32 = tp.next()?.parse().ok()?;
    let mm: u32 = tp.next()?.parse().ok()?;
    let ss: u32 = tp.next().unwrap_or("0").parse().ok()?;
    let days = ymd_to_days(y, m, d)? as i64;
    let unix = days * 86400 + (hh as i64) * 3600 + (mm as i64) * 60 + ss as i64;
    Some(unix as f64 - UNIX_J2000_OFFSET)
}

pub fn ymd_to_days(year: i64, month: u32, day: u32) -> Option<u64> {
    let (y, m) = if month <= 2 {
        (year - 1, month + 12)
    } else {
        (year, month)
    };
    let a = y / 100;
    let b = 2 - a + a / 4;
    let jdn =
        (365.25 * (y + 4716) as f64) as i64 + (30.6001 * (m + 1) as f64) as i64 + day as i64 + b
            - 1524;
    let days = jdn - 2440588;
    if days < 0 {
        None
    } else {
        Some(days as u64)
    }
}

#[derive(Clone, Copy, Default)]
pub struct OriginState {
    pub fetched: f64,
    pub ttl: f64,
    pub prev_epoch: f64,
    pub prev_abs: [f64; 3],
    pub prev_motion: Option<Motion>,
    pub resid_ema: f64,
    pub has_prev: bool,
    pub zero_yield: u32,
}

pub struct StationState {
    pub sample: Option<Sample>,
    pub buffer: Arc<Buffer>,
    pub ema_interval: f64,
    pub last_seen: f64,
}

pub enum PendingPosition {
    Source,
    Geodetic {
        lat: f64,
        lon: f64,
        alt: f64,
    },
    GeodeticFlow {
        lat: f64,
        lon: f64,
        alt: f64,
        speed: f64,
        track: f64,
        vrate: f64,
    },
    StateVector {
        p: [f64; 3],
        v: [f64; 3],
        track: bool,
    },
}

pub struct PendingSample {
    pub epoch: f64,
    pub position: PendingPosition,
    pub fields: Vec<(String, f64)>,
}

pub fn origin_stale(
    origins: &HashMap<Origin, OriginState>,
    origin: Origin,
    ttl: u64,
    now: f64,
) -> bool {
    match origins.get(&origin) {
        Some(o) => now - o.fetched >= ttl as f64 / Φ,
        None => true,
    }
}

pub fn presence_gate(
    presences: &[(f64, f64, f64, f64, f64)],
    pos: (f64, f64, f64),
    extent: f64,
) -> bool {
    presences.iter().any(|&(_, x, y, z, range)| {
        let reach = extent * Φ + range;
        let dx = x - pos.0;
        let dy = y - pos.1;
        let dz = z - pos.2;
        dx * dx + dy * dy + dz * dz <= reach * reach
    })
}

#[derive(Clone, Debug)]
pub enum JsonVal {
    Null,
    Bool(bool),
    Num(f64),
    Str(String),
    Arr(Vec<JsonVal>),
    Obj(HashMap<String, JsonVal>),
}

pub fn parse_json(s: &str) -> Option<JsonVal> {
    let mut p = JsonParser {
        chars: s.as_bytes(),
        pos: 0,
    };
    p.skip_ws();
    p.parse_value()
}

pub struct JsonParser<'a> {
    pub chars: &'a [u8],
    pub pos: usize,
}

impl<'a> JsonParser<'a> {
    fn skip_ws(&mut self) {
        while self.pos < self.chars.len() && (self.chars[self.pos] as char).is_whitespace() {
            self.pos += 1;
        }
    }
    fn parse_value(&mut self) -> Option<JsonVal> {
        self.skip_ws();
        if self.pos >= self.chars.len() {
            return None;
        }
        match self.chars[self.pos] {
            b'{' => self.parse_obj(),
            b'[' => self.parse_arr(),
            b'"' => self.parse_str().map(JsonVal::Str),
            b't' => {
                self.pos += 4;
                Some(JsonVal::Bool(true))
            }
            b'f' => {
                self.pos += 5;
                Some(JsonVal::Bool(false))
            }
            b'n' => {
                self.pos += 4;
                Some(JsonVal::Null)
            }
            _ => self.parse_num(),
        }
    }
    fn parse_obj(&mut self) -> Option<JsonVal> {
        self.pos += 1;
        self.skip_ws();
        let mut map = HashMap::new();
        if self.pos < self.chars.len() && self.chars[self.pos] == b'}' {
            self.pos += 1;
            return Some(JsonVal::Obj(map));
        }
        loop {
            self.skip_ws();
            let key = self.parse_str()?;
            self.skip_ws();
            if self.pos >= self.chars.len() || self.chars[self.pos] != b':' {
                return None;
            }
            self.pos += 1;
            let val = self.parse_value()?;
            map.insert(key, val);
            self.skip_ws();
            if self.pos >= self.chars.len() {
                return None;
            }
            match self.chars[self.pos] {
                b',' => {
                    self.pos += 1;
                }
                b'}' => {
                    self.pos += 1;
                    break;
                }
                _ => return None,
            }
        }
        Some(JsonVal::Obj(map))
    }
    fn parse_arr(&mut self) -> Option<JsonVal> {
        self.pos += 1;
        self.skip_ws();
        let mut arr = Vec::new();
        if self.pos < self.chars.len() && self.chars[self.pos] == b']' {
            self.pos += 1;
            return Some(JsonVal::Arr(arr));
        }
        loop {
            let val = self.parse_value()?;
            arr.push(val);
            self.skip_ws();
            if self.pos >= self.chars.len() {
                return None;
            }
            match self.chars[self.pos] {
                b',' => {
                    self.pos += 1;
                }
                b']' => {
                    self.pos += 1;
                    break;
                }
                _ => return None,
            }
        }
        Some(JsonVal::Arr(arr))
    }
    fn parse_str(&mut self) -> Option<String> {
        if self.pos >= self.chars.len() || self.chars[self.pos] != b'"' {
            return None;
        }
        self.pos += 1;
        let mut s = String::new();
        while self.pos < self.chars.len() {
            let c = self.chars[self.pos];
            if c == b'\\' && self.pos + 1 < self.chars.len() {
                self.pos += 1;
                match self.chars[self.pos] {
                    b'"' => {
                        s.push('"');
                        self.pos += 1;
                    }
                    b'\\' => {
                        s.push('\\');
                        self.pos += 1;
                    }
                    b'/' => {
                        s.push('/');
                        self.pos += 1;
                    }
                    b'n' => {
                        s.push('\n');
                        self.pos += 1;
                    }
                    b't' => {
                        s.push('\t');
                        self.pos += 1;
                    }
                    b'r' => {
                        s.push('\r');
                        self.pos += 1;
                    }
                    b'u' => {
                        self.pos += 1;
                        if self.pos + 4 <= self.chars.len() {
                            if let Ok(hex) =
                                std::str::from_utf8(&self.chars[self.pos..self.pos + 4])
                            {
                                if let Ok(cp) = u32::from_str_radix(hex, 16) {
                                    if let Some(ch) = char::from_u32(cp) {
                                        s.push(ch);
                                    }
                                }
                            }
                            self.pos += 4;
                        }
                    }
                    _ => {
                        self.pos += 1;
                    }
                }
            } else if c == b'"' {
                self.pos += 1;
                return Some(s);
            } else {
                s.push(c as char);
                self.pos += 1;
            }
        }
        None
    }
    fn parse_num(&mut self) -> Option<JsonVal> {
        let start = self.pos;
        while self.pos < self.chars.len() {
            let c = self.chars[self.pos];
            if c.is_ascii_digit() || c == b'-' || c == b'+' || c == b'.' || c == b'e' || c == b'E' {
                self.pos += 1;
            } else {
                break;
            }
        }
        let s = std::str::from_utf8(&self.chars[start..self.pos]).ok()?;
        s.parse::<f64>().ok().map(JsonVal::Num)
    }
}

pub fn scalar_of(v: &JsonVal) -> Option<f64> {
    match v {
        JsonVal::Num(n) => Some(*n),
        JsonVal::Str(s) => s.parse().ok(),
        _ => None,
    }
}

pub fn votable_to_json(votable: &str) -> Option<String> {
    let b = votable;
    let mut cols: Vec<&str> = Vec::new();
    let mut pos = 0;
    while let Some(f_start) = b[pos..].find("<FIELD") {
        let tag_start = pos + f_start;
        let rest = &b[tag_start + 6..];
        if let Some(na) = rest.find("name=\"") {
            let vs = na + 6;
            if let Some(ve) = rest[vs..].find('"') {
                cols.push(&rest[vs..vs + ve]);
            }
        }
        pos = tag_start + 6 + 1;
    }
    let tds = b.find("<TABLEDATA>")?;
    let tde = b[tds + 11..].find("</TABLEDATA>")?;
    let td = &b[tds + 11..tds + 11 + tde];
    let mut rows: Vec<String> = Vec::new();
    for tr in td.split("</TR>") {
        let t = tr.trim();
        if !t.starts_with("<TR>") {
            continue;
        }
        let inner = &t[4..];
        let mut vals: Vec<&str> = Vec::new();
        for td_cell in inner.split("</TD>") {
            let c = td_cell.trim();
            if c.is_empty() {
                continue;
            }
            if let Some(s) = c.find("<TD>") {
                vals.push(c[s + 4..].trim());
            }
        }
        if vals.is_empty() {
            continue;
        }
        let mut o = String::from("{");
        for (i, v) in vals.iter().enumerate() {
            if i > 0 {
                o.push(',');
            }
            if i < cols.len() {
                o.push('"');
                o.push_str(cols[i]);
                o.push('"');
                o.push(':');
                if v.parse::<f64>().is_ok() {
                    o.push_str(v);
                } else {
                    o.push('"');
                    o.push_str(v);
                    o.push('"');
                }
            }
        }
        o.push('}');
        rows.push(o);
    }
    if rows.is_empty() {
        return None;
    }
    Some(format!("{{\"data\":[{}]}}", rows.join(",")))
}

pub fn csv_to_json(csv: &str) -> Option<String> {
    let lines: Vec<&str> = csv
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect();
    if lines.is_empty() {
        return None;
    }
    let header_idx = match lines.iter().position(|l| !l.starts_with('#')) {
        Some(0) => 0,
        Some(i) => {
            if lines[0].starts_with('#')
                && lines[0].trim_start_matches('#').split_whitespace().count() > 1
                && lines[i]
                    .split_whitespace()
                    .next()
                    .map(|t| t.parse::<f64>().is_ok())
                    == Some(true)
            {
                0
            } else {
                i
            }
        }
        None => 0,
    };
    let header_line = lines[header_idx].trim_start_matches('#');
    let delim = if header_line.contains('\t') {
        '\t'
    } else if header_line.contains('|') && header_line.split('|').count() > 1 {
        '|'
    } else if header_line.contains(',') && header_line.split(',').count() > 1 {
        ','
    } else {
        ' '
    };
    let cols: Vec<String> = if delim == ' ' {
        header_line
            .split_whitespace()
            .map(|c| c.trim_matches('"').to_string())
            .collect()
    } else {
        split_csv_line(header_line, delim)
            .into_iter()
            .map(|c| c.trim_matches('"').to_string())
            .collect()
    };
    if cols.is_empty() {
        return None;
    }
    let mut rows: Vec<String> = Vec::new();
    for line in lines.iter().skip(header_idx + 1) {
        if line.starts_with('#') {
            continue;
        }
        let vals: Vec<String> = if delim == ' ' {
            line.split_whitespace().map(|s| s.to_string()).collect()
        } else {
            split_csv_line(line, delim)
        };
        if vals.is_empty() {
            continue;
        }
        let mut o = String::from("{");
        for (i, v) in vals.iter().enumerate() {
            if i > 0 {
                o.push(',');
            }
            let col_name = if i < cols.len() {
                &cols[i]
            } else {
                &format!("col{}", i)
            };
            o.push('"');
            o.push_str(col_name);
            o.push('"');
            o.push(':');
            let cleaned = v.trim_matches('"');
            if cleaned.parse::<f64>().is_ok() {
                o.push_str(cleaned);
            } else {
                o.push('"');
                o.push_str(cleaned);
                o.push('"');
            }
        }
        o.push('}');
        rows.push(o);
    }
    if rows.is_empty() {
        return None;
    }
    Some(format!("{{\"data\":[{}]}}", rows.join(",")))
}

pub fn split_csv_line(line: &str, delim: char) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut chars = line.chars();
    while let Some(c) = chars.next() {
        if in_quotes {
            if c == '"' {
                if let Some('"') = chars.clone().next() {
                    current.push('"');
                    chars.next();
                } else {
                    in_quotes = false;
                }
            } else {
                current.push(c);
            }
        } else if c == '"' {
            in_quotes = true;
        } else if c == delim {
            result.push(current.trim().to_string());
            current = String::new();
        } else {
            current.push(c);
        }
    }
    result.push(current.trim().to_string());
    result
}

pub fn erddap_csv_to_json(csv: &str) -> Option<String> {
    let lines: Vec<&str> = csv
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect();
    if lines.len() < 2 {
        return None;
    }
    let header_idx = match lines.iter().position(|l| !l.starts_with('#')) {
        Some(0) => 0,
        Some(i) => {
            if lines[0].starts_with('#')
                && lines[0].trim_start_matches('#').split_whitespace().count() > 1
                && lines[i]
                    .split_whitespace()
                    .next()
                    .map(|t| t.parse::<f64>().is_ok())
                    == Some(true)
            {
                0
            } else {
                i
            }
        }
        None => 0,
    };
    let header_line = lines[header_idx].trim_start_matches('#');
    let delim = if header_line.contains('\t') {
        '\t'
    } else if header_line.contains('|') && header_line.split('|').count() > 1 {
        '|'
    } else if header_line.contains(',') && header_line.split(',').count() > 1 {
        ','
    } else {
        ' '
    };
    let cols: Vec<String> = if delim == ' ' {
        header_line
            .split_whitespace()
            .map(|s| s.trim().to_string())
            .collect()
    } else {
        header_line
            .split(delim)
            .map(|s| s.trim().trim_matches('"').to_string())
            .collect()
    };
    if cols.is_empty() || cols.iter().all(|c| c.is_empty()) {
        return None;
    }
    let mut rows: Vec<String> = Vec::new();
    let mut skip = 0u32;
    for (_li, line) in lines.iter().enumerate().skip(header_idx + 1) {
        if line.starts_with('#') {
            continue;
        }
        if skip < 1 {
            let lower = line.to_lowercase();
            let is_unit_row = lower.contains("degrees_")
                || lower.contains("degree_")
                || lower.contains("seconds since")
                || lower.trim() == "utc"
                || lower.contains(" utc")
                || lower.contains(" utc,")
                || lower.starts_with("utc,")
                || lower.contains("1e-3")
                || lower.contains("db")
                || lower.contains("m/s")
                || lower.contains("knots")
                || lower.contains("ug/l")
                || lower.contains("μmol");
            if is_unit_row {
                skip += 1;
                continue;
            }
            skip += 1;
        }
        let vals: Vec<String> = if delim == ' ' {
            line.split_whitespace()
                .map(|s| s.trim().to_string())
                .collect()
        } else {
            split_csv_line(line, delim)
        };
        if vals.is_empty() {
            continue;
        }
        let mut o = String::from("{");
        for (i, v) in vals.iter().enumerate() {
            if i > 0 {
                o.push(',');
            }
            if i < cols.len() {
                o.push('"');
                o.push_str(&cols[i]);
                o.push('"');
                o.push(':');
            } else {
                o.push('"');
                o.push_str(&format!("col{}", i));
                o.push('"');
                o.push(':');
            }
            let cleaned = v.trim().trim_matches('"');
            if cleaned.parse::<f64>().is_ok() {
                o.push_str(cleaned);
            } else {
                o.push('"');
                for ch in cleaned.chars() {
                    match ch {
                        '"' => o.push_str("\\\""),
                        '\\' => o.push_str("\\\\"),
                        _ => o.push(ch),
                    }
                }
                o.push('"');
            }
        }
        o.push('}');
        rows.push(o);
    }
    if rows.is_empty() {
        return None;
    }
    Some(format!("{{\"data\":[{}]}}", rows.join(",")))
}

pub fn stac_to_json(body: &str) -> Option<String> {
    let parsed = parse_json(body)?;
    let features = jpath_val(&parsed, "features")?;
    let arr = match features {
        JsonVal::Arr(a) => a,
        _ => return None,
    };
    let mut rows: Vec<String> = Vec::new();
    for feature in arr.iter() {
        let bbox = jpath_val(feature, "bbox");
        let geometry = jpath_val(feature, "geometry");
        let props = jpath_val(feature, "properties");

        let (lat, lon) = match (bbox, geometry) {
            (Some(JsonVal::Arr(b)), _) if b.len() >= 4 => {
                let minlon = scalar_of(&b[0]).unwrap_or(0.0);
                let minlat = scalar_of(&b[1]).unwrap_or(0.0);
                let maxlon = scalar_of(&b[2]).unwrap_or(0.0);
                let maxlat = scalar_of(&b[3]).unwrap_or(0.0);
                ((minlat + maxlat) / 2.0, (minlon + maxlon) / 2.0)
            }
            (_, Some(JsonVal::Obj(g))) => {
                if let Some(coords) = g.get("coordinates") {
                    let (mut clat, mut clon, mut count) = (0.0f64, 0.0f64, 0u32);
                    fn sum_ring(ring: &[JsonVal], lat: &mut f64, lon: &mut f64, count: &mut u32) {
                        for point in ring.iter() {
                            if let JsonVal::Arr(p) = point {
                                if p.len() >= 2 {
                                    *lon += scalar_of(&p[0]).unwrap_or(0.0);
                                    *lat += scalar_of(&p[1]).unwrap_or(0.0);
                                    *count += 1;
                                }
                            }
                        }
                    }
                    if let JsonVal::Arr(outer) = coords {
                        if let Some(first) = outer.first() {
                            if matches!(first, JsonVal::Arr(inner) if inner.first().map_or(false, |v| matches!(v, JsonVal::Num(_))))
                            {
                                for ring in outer.iter() {
                                    if let JsonVal::Arr(r) = ring {
                                        sum_ring(r, &mut clat, &mut clon, &mut count);
                                    }
                                }
                            } else {
                                sum_ring(outer, &mut clat, &mut clon, &mut count);
                            }
                        }
                    }
                    if count > 0 {
                        (clat / count as f64, clon / count as f64)
                    } else {
                        (0.0, 0.0)
                    }
                } else {
                    continue;
                }
            }
            _ => continue,
        };

        let dt = props
            .and_then(|p| jpath_val(p, "datetime"))
            .and_then(|v| match v {
                JsonVal::Str(s) => Some(s.clone()),
                _ => None,
            })
            .unwrap_or_default();

        let collection = props
            .and_then(|p| jpath_val(p, "collection"))
            .and_then(|v| match v {
                JsonVal::Str(s) => Some(s.clone()),
                _ => None,
            })
            .unwrap_or_default();

        let mut o = String::from("{\"lat\":");
        o.push_str(&format!("{}", lat));
        o.push_str(",\"lon\":");
        o.push_str(&format!("{}", lon));
        o.push_str(",\"datetime\":\"");
        o.push_str(&dt);
        o.push('"');
        if !collection.is_empty() {
            o.push_str(",\"collection\":\"");
            o.push_str(&collection);
            o.push('"');
        }
        if let Some(JsonVal::Obj(pmap)) = props {
            for (k, v) in pmap.iter() {
                if k == "datetime" || k == "collection" {
                    continue;
                }
                let clean_key = k.replace(':', "_").replace('.', "_");
                o.push(',');
                o.push('"');
                o.push_str("stac_");
                o.push_str(&clean_key);
                o.push('"');
                o.push(':');
                match v {
                    JsonVal::Num(n) => o.push_str(&format!("{}", n)),
                    JsonVal::Str(s) => {
                        o.push('"');
                        for ch in s.chars() {
                            match ch {
                                '"' => o.push_str("\\\""),
                                '\\' => o.push_str("\\\\"),
                                _ => o.push(ch),
                            }
                        }
                        o.push('"');
                    }
                    JsonVal::Bool(b) => o.push_str(if *b { "true" } else { "false" }),
                    _ => o.push_str("null"),
                }
            }
        }
        o.push('}');
        rows.push(o);
    }
    if rows.is_empty() {
        return None;
    }
    Some(format!("{{\"data\":[{}]}}", rows.join(",")))
}

pub fn sanitize_col_name(name: &str) -> String {
    let mut out = String::new();
    for c in name.chars() {
        match c {
            '[' | ']' | '(' | ')' | '/' | '\\' | '.' | '%' => out.push('_'),
            ' ' | '\t' | '\n' | '\r' => {
                if !out.ends_with('_') {
                    out.push('_')
                }
            }
            _ => out.push(c),
        }
    }
    out.trim_matches('_').to_string()
}

pub fn html_table_to_json(html: &str, target_idx: usize) -> Option<String> {
    let lower = html.to_lowercase();
    let mut table_pos = 0usize;
    let mut current_idx = 0usize;
    while current_idx <= target_idx {
        let search = &lower[table_pos..];
        let found = search.find("<table")?;
        let abs = table_pos + found;
        let rest = &html[abs..];
        let end = find_closing_tag(rest, "table")?;
        if current_idx == target_idx {
            let table = &rest[..end];
            return parse_html_table(table);
        }
        table_pos = abs + end;
        current_idx += 1;
    }
    None
}

pub fn parse_html_table(table: &str) -> Option<String> {
    let mut header: Vec<String> = Vec::new();
    let mut rows: Vec<String> = Vec::new();

    let mut pos = 0;
    while let Some(tr_start) = table[pos..].to_lowercase().find("<tr") {
        let tr_abs = pos + tr_start;
        let tr_rest = &table[tr_abs..];
        let tr_end = find_closing_tag(tr_rest, "tr")?;
        let tr_content = &tr_rest[..tr_end];
        pos = tr_abs + tr_end;

        let mut cells: Vec<String> = Vec::new();
        let mut cp = 0;
        while cp < tr_content.len() {
            let rest = &tr_content[cp..];
            let lower_rest = rest.to_lowercase();
            if let Some(td_start) = lower_rest.find("<td") {
                let td_abs = cp + td_start;
                let td_rest = &tr_content[td_abs..];
                if let Some(td_end) = find_closing_tag(td_rest, "td") {
                    let inner = &td_rest[..td_end];
                    let gt = inner.find('>').unwrap_or(0);
                    let val = strip_html_tags(&inner[gt + 1..]);
                    cells.push(val.trim().to_string());
                    cp = td_abs + td_end;
                    continue;
                }
            }
            if let Some(th_start) = lower_rest.find("<th") {
                let th_abs = cp + th_start;
                let th_rest = &tr_content[th_abs..];
                if let Some(th_end) = find_closing_tag(th_rest, "th") {
                    let inner = &th_rest[..th_end];
                    let gt = inner.find('>').unwrap_or(0);
                    let val = strip_html_tags(&inner[gt + 1..]);
                    cells.push(val.trim().to_string());
                    cp = th_abs + th_end;
                    continue;
                }
            }
            break;
        }

        if cells.is_empty() {
            continue;
        }

        let is_header = tr_content.to_lowercase().contains("<th");
        let has_colspan = tr_content.to_lowercase().contains("colspan");

        if has_colspan && is_header {
            continue;
        }

        if header.is_empty() && (is_header || cells.iter().any(|c| c.parse::<f64>().is_err())) {
            header = cells.into_iter().map(|c| sanitize_col_name(&c)).collect();
            if header.iter().all(|h| h.is_empty()) {
                header.clear();
            }
            continue;
        }

        let mut o = String::from("{");
        for (i, v) in cells.iter().enumerate() {
            if i > 0 {
                o.push(',');
            }
            let col = if i < header.len() && !header[i].is_empty() {
                &header[i]
            } else if i < header.len() {
                "val"
            } else {
                "col"
            };
            o.push('"');
            o.push_str(col);
            o.push('"');
            o.push(':');
            let cleaned = v.trim();
            if cleaned.parse::<f64>().is_ok() {
                o.push_str(cleaned);
            } else {
                o.push('"');
                for ch in cleaned.chars() {
                    match ch {
                        '"' => o.push_str("\\\""),
                        '\\' => o.push_str("\\\\"),
                        _ => o.push(ch),
                    }
                }
                o.push('"');
            }
        }
        o.push('}');
        rows.push(o);
    }

    if rows.is_empty() {
        return None;
    }
    Some(format!("{{\"data\":[{}]}}", rows.join(",")))
}

pub fn find_closing_tag(html: &str, tag: &str) -> Option<usize> {
    let open = format!("<{}", tag);
    let close = format!("</{}>", tag);
    let mut depth = 1u32;
    let mut pos = html.find(&open)? + open.len();
    while depth > 0 {
        let rest = &html[pos..];
        let lower = rest.to_lowercase();
        let next_open = lower.find(&open);
        let next_close = lower.find(&close);
        match (next_open, next_close) {
            (_, Some(ci)) if next_open.map_or(true, |oi| ci < oi) => {
                depth -= 1;
                pos += ci + close.len();
            }
            (Some(oi), _) => {
                depth += 1;
                pos += oi + open.len();
            }
            _ => return None,
        }
    }
    Some(pos)
}

pub fn strip_html_tags(s: &str) -> String {
    let mut out = String::new();
    let mut in_tag = false;
    for c in s.chars() {
        if c == '<' {
            in_tag = true;
        } else if c == '>' {
            in_tag = false;
        } else if !in_tag {
            out.push(c);
        }
    }
    out
}

pub fn universal_auto_detect(j: &JsonVal) -> Vec<Extract> {
    let arr = match jpath_val(j, "data").and_then(|v| {
        if let JsonVal::Arr(a) = v {
            Some(a)
        } else {
            None
        }
    }) {
        Some(a) => a,
        None => return vec![],
    };
    let first = match arr.first() {
        Some(JsonVal::Obj(m)) => m,
        _ => return vec![],
    };
    let has_ra = first.contains_key("ra");
    let has_dec = first.contains_key("dec");
    let has_lat = first.contains_key("lat");
    let has_lon = first.contains_key("lon");
    if has_ra && has_dec {
        let plx_key = if first.contains_key("plx") { "plx" } else { "" };
        let pmra_key = if first.contains_key("pmra") {
            "pmra"
        } else {
            ""
        };
        let pmdec_key = if first.contains_key("pmdec") {
            "pmdec"
        } else {
            ""
        };
        let rv_key = if first.contains_key("radvel") {
            "radvel"
        } else {
            ""
        };
        let dist_key = if first.contains_key("dist") {
            "dist"
        } else {
            ""
        };
        let z_key = if first.contains_key("z") { "z" } else { "" };
        let epoch_key = if first.contains_key("t") { "t" } else { "" };
        let mut fields = vec![];
        if first.contains_key("val") {
            fields.push(("val".into(), "val".into()));
        }
        if first.contains_key("extent") {
            fields.push(("extent".into(), "extent".into()));
        }
        if first.contains_key("tau") {
            fields.push(("tau".into(), "tau".into()));
        }
        vec![Extract::CelestialMap {
            arr_path: "data".into(),
            ra_key: "ra".into(),
            dec_key: "dec".into(),
            dist_key: dist_key.into(),
            dist_scale: 1.0,
            plx_key: plx_key.into(),
            z_key: z_key.into(),
            pmra_key: pmra_key.into(),
            pmdec_key: pmdec_key.into(),
            rv_key: rv_key.into(),
            rv_scale: 1.0,
            epoch_key: epoch_key.into(),
            fields,
        }]
    } else if has_lat && has_lon {
        let alt_key = if first.contains_key("alt") { "alt" } else { "" };
        let epoch_key = if first.contains_key("t") { "t" } else { "" };
        let vel_key = if first.contains_key("vel") { "vel" } else { "" };
        let trk_key = if first.contains_key("trk") { "trk" } else { "" };
        let vr_key = if first.contains_key("vr") { "vr" } else { "" };
        let mut fields = vec![];
        if first.contains_key("val") {
            fields.push(("val".into(), "val".into()));
        }
        if first.contains_key("extent") {
            fields.push(("extent".into(), "extent".into()));
        }
        vec![Extract::Map {
            arr_path: "data".into(),
            lat_key: "lat".into(),
            lon_key: "lon".into(),
            alt_key: alt_key.into(),
            epoch_key: epoch_key.into(),
            val_key: String::new(),
            alt_sign: -1.0,
            vel_key: vel_key.into(),
            trk_key: trk_key.into(),
            vr_key: vr_key.into(),
            fields,
            lon_sign: None,
        }]
    } else {
        vec![]
    }
}

pub fn jpath_val<'a>(json: &'a JsonVal, path: &str) -> Option<&'a JsonVal> {
    if path.is_empty() || path == "." {
        return Some(json);
    }
    let mut current = json;
    for part in path.split('.') {
        if let JsonVal::Obj(map) = current {
            current = map.get(part)?;
        } else if let JsonVal::Arr(arr) = current {
            let raw_idx: i64 = part.parse().ok()?;
            let len = arr.len() as i64;
            let idx = if raw_idx < 0 {
                let actual = len + raw_idx;
                if actual < 0 {
                    return None;
                }
                actual as usize
            } else {
                raw_idx as usize
            };
            current = arr.get(idx)?;
        } else {
            return None;
        }
    }
    Some(current)
}

pub fn jnum(json: &JsonVal, key: &str) -> Option<f64> {
    if key.contains('.') {
        return jpath_val(json, key).and_then(scalar_of);
    }
    match json {
        JsonVal::Obj(map) => map.get(key).and_then(scalar_of),
        _ => None,
    }
}

pub fn jpath(json: &JsonVal, path: &str) -> Option<f64> {
    if path == "." || path.is_empty() {
        return scalar_of(json);
    }
    jpath_val(json, path).and_then(scalar_of)
}

pub fn flatten_geojson_coords(val: &[JsonVal]) -> Vec<(f64, f64)> {
    if let Some(JsonVal::Num(_)) = val.first() {
        if val.len() >= 2 {
            if let (Some(lon), Some(lat)) = (scalar_of(&val[0]), scalar_of(&val[1])) {
                return vec![(lon, lat)];
            }
        }
        return Vec::new();
    }
    let mut result = Vec::new();
    for v in val {
        if let JsonVal::Arr(inner) = v {
            result.extend(flatten_geojson_coords(inner));
        }
    }
    result
}

pub fn jdeep_find_num(json: &JsonVal, key: &str) -> Option<f64> {
    match json {
        JsonVal::Obj(map) => {
            if let Some(v) = map.get(key) {
                if let Some(n) = scalar_of(v) {
                    return Some(n);
                }
            }
            for v in map.values() {
                if let Some(n) = jdeep_find_num(v, key) {
                    return Some(n);
                }
            }
            None
        }
        JsonVal::Arr(arr) => {
            for v in arr {
                if let Some(n) = jdeep_find_num(v, key) {
                    return Some(n);
                }
            }
            None
        }
        _ => None,
    }
}

pub fn jcount(json: &JsonVal, path: &str) -> Option<f64> {
    if path == "." || path.is_empty() {
        if let JsonVal::Arr(arr) = json {
            return Some(arr.len() as f64);
        }
        return None;
    }
    if path.contains('.') {
        let target = jpath_val(json, path)?;
        if let JsonVal::Arr(arr) = target {
            return Some(arr.len() as f64);
        }
        return None;
    }
    match json {
        JsonVal::Obj(map) => {
            if let Some(JsonVal::Arr(arr)) = map.get(path) {
                Some(arr.len() as f64)
            } else {
                None
            }
        }
        _ => None,
    }
}

pub fn jfirst(json: &JsonVal, key: &str) -> Option<f64> {
    if key.contains('.') {
        let target_path = key.rsplit_once('.').map(|(p, _)| p).unwrap_or("");
        let final_key = key.rsplit_once('.').map(|(_, k)| k).unwrap_or(key);
        let parent = if target_path.is_empty() {
            json
        } else {
            jpath_val(json, target_path)?
        };
        if let JsonVal::Arr(arr) = parent {
            return arr.first().and_then(|v| {
                if let JsonVal::Obj(o) = v {
                    o.get(final_key).and_then(scalar_of)
                } else {
                    scalar_of(v)
                }
            });
        }
        return None;
    }
    match json {
        JsonVal::Arr(arr) => arr.first().and_then(|v| match v {
            JsonVal::Obj(o) => o.get(key).and_then(scalar_of),
            other => scalar_of(other),
        }),
        JsonVal::Obj(map) => map.get(key).and_then(|v| {
            if let JsonVal::Arr(a) = v {
                a.first().and_then(scalar_of)
            } else {
                None
            }
        }),
        _ => None,
    }
}

pub fn jlast(json: &JsonVal, key: &str) -> Option<f64> {
    if key.contains('.') {
        let target_path = key.rsplit_once('.').map(|(p, _)| p).unwrap_or("");
        let final_key = key.rsplit_once('.').map(|(_, k)| k).unwrap_or(key);
        let parent = if target_path.is_empty() {
            json
        } else {
            jpath_val(json, target_path)?
        };
        if let JsonVal::Arr(arr) = parent {
            return arr.last().and_then(|v| {
                if let JsonVal::Obj(o) = v {
                    o.get(final_key).and_then(scalar_of)
                } else {
                    scalar_of(v)
                }
            });
        }
        return None;
    }
    match json {
        JsonVal::Arr(arr) => arr.last().and_then(|v| match v {
            JsonVal::Obj(o) => o.get(key).and_then(scalar_of),
            other => scalar_of(other),
        }),
        JsonVal::Obj(map) => map.get(key).and_then(|v| {
            if let JsonVal::Arr(a) = v {
                a.last().and_then(scalar_of)
            } else {
                None
            }
        }),
        _ => None,
    }
}

pub fn extract_regex_val(body: &str, pat: &str) -> Option<f64> {
    let pat_bytes = pat.as_bytes();
    let body_bytes = body.as_bytes();

    let first = pat.find('(')?;
    let last = pat.rfind(')')?;
    if first >= last {
        return None;
    }
    let inner = &pat[first + 1..last];

    if inner.contains("...") {
        let (prefix, suffix) = inner.split_once("...")?;
        let p = body.find(prefix)?;
        let r = &body[p + prefix.len()..];
        let e = if suffix.is_empty() {
            r.find(|c: char| c.is_whitespace() || c == '<' || c == '"')
                .unwrap_or(r.len())
        } else {
            r.find(suffix).unwrap_or(r.len())
        };
        return r[..e].trim().parse::<f64>().ok();
    }

    let mut capture: Option<f64> = None;

    fn match_re(
        mut pi: usize,
        p: &[u8],
        mut bi: usize,
        b: &[u8],
        cap: &mut Option<f64>,
    ) -> Option<usize> {
        while pi < p.len() {
            let bc = || b.get(bi).copied();
            match p[pi] {
                b'\\' => {
                    pi += 1;
                    let esc = p.get(pi).copied()?;
                    let ok = match esc {
                        b'd' => bc().map_or(false, |c| c.is_ascii_digit()),
                        b's' => bc().map_or(false, |c| c.is_ascii_whitespace()),
                        b'w' => bc().map_or(false, |c| c.is_ascii_alphanumeric() || c == b'_'),
                        b'D' => bc().map_or(true, |c| !c.is_ascii_digit()),
                        b'S' => bc().map_or(true, |c| !c.is_ascii_whitespace()),
                        b'W' => bc().map_or(true, |c| !(c.is_ascii_alphanumeric() || c == b'_')),
                        _ => bc() == Some(esc),
                    };
                    if !ok {
                        return None;
                    }
                    bi += 1;
                    pi += 1;
                    let check = |c: u8| -> bool {
                        match esc {
                            b'd' => c.is_ascii_digit(),
                            b's' => c.is_ascii_whitespace(),
                            b'w' => c.is_ascii_alphanumeric() || c == b'_',
                            b'D' => !c.is_ascii_digit(),
                            b'S' => !c.is_ascii_whitespace(),
                            b'W' => !(c.is_ascii_alphanumeric() || c == b'_'),
                            _ => c == esc,
                        }
                    };
                    if pi < p.len() {
                        match p[pi] {
                            b'+' => {
                                pi += 1;
                                while b.get(bi).map_or(false, |&c| check(c)) {
                                    bi += 1;
                                }
                            }
                            b'*' => {
                                pi += 1;
                                while b.get(bi).map_or(false, |&c| check(c)) {
                                    bi += 1;
                                }
                            }
                            b'?' => {
                                pi += 1;
                            }
                            _ => {}
                        }
                    }
                }
                b'.' => {
                    pi += 1;
                    if bc().is_none() || bc() == Some(b'\n') {
                        return None;
                    }
                    let (min, max, greedy) = if pi < p.len() {
                        match p[pi] {
                            b'+' => {
                                pi += 1;
                                (1, usize::MAX, true)
                            }
                            b'*' => {
                                pi += 1;
                                (0, usize::MAX, true)
                            }
                            b'?' => {
                                pi += 1;
                                (0, 1, true)
                            }
                            _ => (1, 1, true),
                        }
                    } else {
                        (1, 1, true)
                    };

                    if greedy {
                        let mut best: Option<usize> = None;
                        for len in (min..=max).rev() {
                            let end = bi + len;
                            if end > b.len() {
                                continue;
                            }
                            if b[bi..end].iter().any(|&c| c == b'\n') {
                                continue;
                            }
                            if let Some(res) = match_re(pi, p, end, b, cap) {
                                best = Some(res);
                                break;
                            }
                        }
                        if let Some(res) = best {
                            bi = res;
                        } else {
                            return None;
                        }
                    } else {
                        if let Some(res) = match_re(pi, p, bi + 1, b, cap) {
                            bi = res;
                        } else {
                            return None;
                        }
                    }
                }
                b'(' => {
                    let mut depth = 1;
                    let mut end = pi + 1;
                    while end < p.len() && depth > 0 {
                        if p[end] == b'\\' {
                            end += 2;
                            continue;
                        }
                        if p[end] == b'(' {
                            depth += 1;
                        }
                        if p[end] == b')' {
                            depth -= 1;
                            if depth == 0 {
                                break;
                            }
                        }
                        end += 1;
                    }
                    if depth != 0 {
                        return None;
                    }
                    let save = bi;
                    if let Some(new_bi) = match_re(pi + 1, &p[pi + 1..end], bi, b, cap) {
                        if cap.is_none() {
                            if let Ok(s) = std::str::from_utf8(&b[save..new_bi]) {
                                if let Ok(v) = s.parse::<f64>() {
                                    *cap = Some(v);
                                }
                            }
                        }
                        bi = new_bi;
                        pi = end + 1;
                    } else {
                        return None;
                    }
                }
                b'[' => {
                    pi += 1;
                    let neg = pi < p.len() && p[pi] == b'^';
                    if neg {
                        pi += 1;
                    }
                    let mut cls = Vec::new();
                    while pi < p.len() && p[pi] != b']' {
                        if p[pi] == b'\\' {
                            cls.push(p[pi + 1]);
                            pi += 2;
                        } else {
                            cls.push(p[pi]);
                            pi += 1;
                        }
                    }
                    pi += 1;
                    let in_cls = bc().map_or(false, |c| cls.contains(&c));
                    if neg == in_cls {
                        return None;
                    }
                    let (min, max) = if pi < p.len() {
                        match p[pi] {
                            b'+' => {
                                pi += 1;
                                (1, usize::MAX)
                            }
                            b'*' => {
                                pi += 1;
                                (0, usize::MAX)
                            }
                            b'?' => {
                                pi += 1;
                                (0, 1)
                            }
                            _ => (1, 1),
                        }
                    } else {
                        (1, 1)
                    };
                    if min > 0 {
                        bi += 1;
                    }
                    if max == usize::MAX {
                        while b.get(bi).map_or(false, |c| cls.contains(c) != neg) {
                            bi += 1;
                        }
                    }
                }
                c => {
                    if bc().map_or(false, |bc| bc == c) {
                        bi += 1;
                        pi += 1;
                    } else {
                        return None;
                    }
                }
            }
        }
        Some(bi)
    }

    if let Some(_) = match_re(0, pat_bytes, 0, body_bytes, &mut capture) {
        capture
    } else {
        None
    }
}

#[derive(Clone)]
pub enum Extract {
    Field(String, String),
    First(String, String),
    Last(String, String),
    Count(String, String),
    LastRow(String, String),
    LastObj(String, String, String, String),
    LastLine(String),
    ObjLast(String, String),
    GeojsonEvents {
        mag_key: String,
        min_mag: f64,
        outputs: Vec<String>,
    },
    Path(String, String),
    Deep(String, String),
    Map {
        arr_path: String,
        lat_key: String,
        lon_key: String,
        alt_key: String,
        epoch_key: String,
        val_key: String,
        alt_sign: f64,
        vel_key: String,
        trk_key: String,
        vr_key: String,
        fields: Vec<(String, String)>,
        lon_sign: Option<String>,
    },
    Regex(String, String),
    XmlCount(String, String),
    Ephemeris(String),
    Vectors(String),
    CelestialMap {
        arr_path: String,
        ra_key: String,
        dec_key: String,
        dist_key: String,
        dist_scale: f64,
        plx_key: String,
        z_key: String,
        pmra_key: String,
        pmdec_key: String,
        rv_key: String,
        rv_scale: f64,
        epoch_key: String,
        fields: Vec<(String, String)>,
    },
    Flatten {
        arr_path: String,
        geom_path: String,
        epoch_key: String,
        fields: Vec<(String, String)>,
    },
    CmrPolygon {
        arr_path: String,
        fields: Vec<(String, String)>,
        epoch_key: String,
        alt_key: String,
        val_key: String,
    },
    CelestialPolygon {
        arr_path: String,
        radius: f64,
        fields: Vec<(String, String)>,
        epoch_key: String,
        val_key: String,
    },
    Rows {
        fields: Vec<(String, String)>,
    },
    KeplerMap {
        arr_path: String,
        a_key: String,
        e_key: String,
        i_key: String,
        om_key: String,
        w_key: String,
        ma_key: String,
        epoch_key: String,
        fields: Vec<(String, String)>,
    },
    Hapi(Vec<(String, String)>),
}

pub fn force_constants(force: &str) -> Option<(f64, f64, bool, u8)> {
    match force {
        "em" => Some((C_LIGHT, (1.0 / C_LIGHT) * 1024.0, false, 0)),
        "gravity" => Some((C_LIGHT, (AU / C_LIGHT) / 1024.0, false, 1)),
        "acoustic" => Some((V_SOUND_288, 1.0 / V_SOUND_288, false, 2)),
        "seismic-body" => Some((V_P_GRANITE, 1.0 / V_P_GRANITE, false, 3)),
        "seismic-surface" => Some((V_S_GRANITE, 1.0 / V_S_GRANITE, false, 4)),
        "thermal" => Some((ALPHA_AIR, 1.0 / ALPHA_AIR, true, 5)),
        "diffusion" => Some((D_AIR, 1.0 / D_AIR, true, 6)),
        "advective" => Some((10.0, 0.1, false, 7)),
        _ => None,
    }
}

pub fn force_extent(force: &str) -> f64 {
    match force {
        "em" | "gravity" => f64::INFINITY,
        "seismic-body" => 1e5,
        "seismic-surface" => 1e4,
        "acoustic" => 1e3,
        "advective" => 1e1,
        "thermal" | "diffusion" => 1e0,
        _ => 0.0,
    }
}

pub fn force_type_of(force: &str) -> f64 {
    force_constants(force)
        .map(|(_, _, _, id)| id as f64)
        .unwrap_or(0.0)
}

pub enum Frame {
    WGS84 { lat: f64, lon: f64, alt: f64 },
    Ecliptic { scale: f64 },
    Areocentric { scale: f64 },
    IAU2000 { lat: f64, lon: f64, alt: f64 },
    Data,
    Query,
}

pub struct StationEntry {
    pub id: String,
    pub lat: f64,
    pub lon: f64,
}

pub struct SourceConfig {
    #[allow(dead_code)]
    pub name: String,
    pub ttl: u64,
    pub url: String,
    pub frame: Frame,
    pub force: String,
    pub tau: Option<f64>,
    pub tau_key: Option<String>,
    pub format: String,
    pub extracts: Vec<Extract>,
    pub headers: Vec<(String, String)>,
    pub pos_fields: Option<(String, String, Option<String>, f64)>,
    pub target: Option<String>,
    pub catalog: Option<String>,
    pub max_freq: Option<f64>,
    pub min_freq: Option<f64>,
    pub body: Option<String>,
    pub stations_url: Option<String>,
    pub stations_path: String,
    pub stations_lat: String,
    pub stations_lon: String,
    pub stations_id: String,
    pub flux_from_mag: Option<String>,
    pub abs_mag_from: Option<String>,
    pub reach_ttl: Option<u64>,
    pub catalog_epoch: Option<f64>,
    pub repeat_ra_bins: u32,
}

pub fn base64_encode(data: &[u8]) -> String {
    const T: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut r = String::new();
    for c in data.chunks(3) {
        let b = [c[0], *c.get(1).unwrap_or(&0), *c.get(2).unwrap_or(&0)];
        r.push(T[(b[0] >> 2) as usize] as char);
        r.push(T[(((b[0] & 0x03) << 4) | (b[1] >> 4)) as usize] as char);
        r.push(if c.len() > 1 {
            T[(((b[1] & 0x0f) << 2) | (b[2] >> 6)) as usize] as char
        } else {
            '='
        });
        r.push(if c.len() > 2 {
            T[(b[2] & 0x3f) as usize] as char
        } else {
            '='
        });
    }
    r
}

pub fn days_to_ymd(total_days: u64) -> (u32, u32, u32) {
    let mut d = total_days as u32;
    let mut y = 1970u32;
    loop {
        let yd = if is_leap(y) { 366 } else { 365 };
        if d < yd {
            break;
        }
        d -= yd;
        y += 1;
    }
    let months: [u32; 12] = if is_leap(y) {
        [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    };
    let mut m = 0u32;
    while d >= months[m as usize] {
        d -= months[m as usize];
        m += 1;
    }
    (y, m + 1, d + 1)
}

pub fn extract_header(s: &str, n: &str) -> Option<String> {
    for l in s.lines() {
        if let Some(c) = l.find(':') {
            if l[..c].trim().eq_ignore_ascii_case(n) {
                return Some(l[c + 1..].trim().to_string());
            }
        }
    }
    None
}

pub fn is_leap(y: u32) -> bool {
    (y % 4 == 0 && y % 100 != 0) || y % 400 == 0
}

pub fn j2d_last_row(json: &JsonVal, col: &str) -> Option<f64> {
    if let JsonVal::Arr(arr) = json {
        if arr.len() < 2 {
            return None;
        }
        if let JsonVal::Arr(headers) = &arr[0] {
            let col_idx = headers.iter().position(|h| {
                if let JsonVal::Str(s) = h {
                    s.eq_ignore_ascii_case(col) || s.starts_with(col)
                } else {
                    false
                }
            })?;
            if let Some(JsonVal::Arr(last_row)) = arr.last() {
                return last_row.get(col_idx).and_then(scalar_of);
            }
        }
    }
    None
}

pub fn text_last_col(data: &str, col: &str) -> Option<f64> {
    let mut header_idx: Option<usize> = None;
    for line in data.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let stripped = trimmed.strip_prefix('#').unwrap_or(trimmed).trim();
        let cols = split_data_line(stripped);
        if header_idx.is_none() {
            if let Some(idx) = cols
                .iter()
                .position(|c| c.eq_ignore_ascii_case(col) || c.starts_with(col))
            {
                header_idx = Some(idx);
                break;
            }
            continue;
        }
    }
    let idx = header_idx?;
    for line in data.lines().rev() {
        let trimmed = line.trim();
        if trimmed.is_empty()
            || trimmed.starts_with('#')
            || trimmed
                .chars()
                .next()
                .map(|c| c.is_alphabetic())
                .unwrap_or(false)
        {
            continue;
        }
        let cols = split_data_line(trimmed);
        if let Some(v) = cols.get(idx) {
            if let Ok(f) = v.trim_matches('"').parse::<f64>() {
                return Some(f);
            }
        }
    }
    None
}

pub fn parse_path(s: &str) -> String {
    let fl = s.lines().next().unwrap_or("");
    let p: Vec<&str> = fl.split_whitespace().collect();
    if p.len() >= 2 {
        p[1].to_string()
    } else {
        "/".to_string()
    }
}
pub fn parse_quoted_args(s: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut chars = s.chars().peekable();
    while chars.peek().is_some() {
        while chars.peek().map(|c| c.is_whitespace()).unwrap_or(false) {
            chars.next();
        }
        if chars.peek().is_none() {
            break;
        }
        if *chars.peek().unwrap() == '"' {
            chars.next();
            let mut val = String::new();
            while let Some(&c) = chars.peek() {
                if c == '"' {
                    chars.next();
                    break;
                }
                val.push(c);
                chars.next();
            }
            result.push(val);
        } else {
            let mut val = String::new();
            while let Some(&c) = chars.peek() {
                if c.is_whitespace() {
                    break;
                }
                val.push(c);
                chars.next();
            }
            result.push(val);
        }
    }
    result
}

pub fn render_url(template: &str, x: f64, y: f64, z: f64, tdb_secs: f64, extent: f64) -> String {
    let unix = tdb_secs + UNIX_J2000_OFFSET;
    let secs = unix as u64;
    let days = secs / 86400;
    let (ty, tm, td) = days_to_ymd(days);
    let yday = {
        let cum = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
        let leap = (ty % 4 == 0 && ty % 100 != 0) || ty % 400 == 0;
        let base = if tm > 0 { cum[(tm - 1) as usize] } else { 0 };
        base + td + if leap && tm > 2 { 1 } else { 0 }
    };
    let year2 = ty % 100;
    let today = format!("{}-{:02}-{:02}", ty, tm, td);
    let (yy, ym, yd) = days_to_ymd(days - 1);
    let yesterday = format!("{}-{:02}-{:02}", yy, ym, yd);
    let (tmy, tmm, tmd) = days_to_ymd(days + 1);
    let tomorrow = format!("{}-{:02}-{:02}", tmy, tmm, tmd);
    let today_yyyymmdd = format!("{}_{:02}_{:02}", ty, tm, td);
    let today_nodashes = format!("{}{:02}{:02}", ty, tm, td);
    let yesterday_nodashes = format!("{}{:02}{:02}", yy, ym, yd);
    let tomorrow_nodashes = format!("{}{:02}{:02}", tmy, tmm, tmd);
    let hour_ago = {
        let dt = secs.saturating_sub(3600);
        let (h_y, h_m, h_d) = days_to_ymd(dt / 86400);
        let h_h = (dt % 86400) / 3600;
        let h_min = (dt % 3600) / 60;
        format!("{}-{:02}-{:02}T{:02}:{:02}:00", h_y, h_m, h_d, h_h, h_min)
    };
    let now_iso = {
        let n_h = (secs % 86400) / 3600;
        let n_min = (secs % 3600) / 60;
        format!("{}-{:02}-{:02}T{:02}:{:02}:00", ty, tm, td, n_h, n_min)
    };
    let now_minus_1 = {
        let dt = secs.saturating_sub(60);
        let (n1_y, n1_m, n1_d) = days_to_ymd(dt / 86400);
        let n1_h = (dt % 86400) / 3600;
        let n1_min = (dt % 3600) / 60;
        format!(
            "{}-{:02}-{:02}T{:02}:{:02}:00",
            n1_y, n1_m, n1_d, n1_h, n1_min
        )
    };
    let now_minus_2 = {
        let dt = secs.saturating_sub(120);
        let (n2_y, n2_m, n2_d) = days_to_ymd(dt / 86400);
        let n2_h = (dt % 86400) / 3600;
        let n2_min = (dt % 3600) / 60;
        format!(
            "{}-{:02}-{:02}T{:02}:{:02}:00",
            n2_y, n2_m, n2_d, n2_h, n2_min
        )
    };
    let week_ago = {
        let dt = secs.saturating_sub(604800);
        let (w_y, w_m, w_d) = days_to_ymd(dt / 86400);
        format!("{}-{:02}-{:02}", w_y, w_m, w_d)
    };
    let week_ago_nodashes = {
        let dt = secs.saturating_sub(604800);
        let (w_y, w_m, w_d) = days_to_ymd(dt / 86400);
        format!("{}{:02}{:02}", w_y, w_m, w_d)
    };
    let q_hour = (secs % 86400) / 3600;
    let q_minute = (secs % 3600) / 60;
    let unix_now = secs.to_string();
    let unix_now_plus_3600 = (secs + 3600).to_string();

    let (lat, lon) = icrs_to_geodetic(x, y, z, tdb_secs);
    let res_usize = 6usize;
    let lat_str = format!("{:.6}", lat);
    let lon_str = format!("{:.6}", lon);
    let half_deg = extent / 111319.0;
    let lat_min_str = format!("{:.*}", res_usize, lat - half_deg);
    let lat_max_str = format!("{:.*}", res_usize, lat + half_deg);
    let lon_min_str = format!("{:.*}", res_usize, lon - half_deg);
    let lon_max_str = format!("{:.*}", res_usize, lon + half_deg);
    let (grid_str, grid_lat_str, grid_lon_str) = {
        let step = half_deg * 0.5;
        let mut g = Vec::with_capacity(16);
        let mut gla = Vec::with_capacity(4);
        let mut glo = Vec::with_capacity(4);
        for i in 0..4 {
            for j in 0..4 {
                g.push(format!(
                    "{:.*},{:.*}",
                    res_usize,
                    lat + (i as f64 - 1.5) * step,
                    res_usize,
                    lon + (j as f64 - 1.5) * step
                ));
            }
            gla.push(format!("{:.*}", res_usize, lat + (i as f64 - 1.5) * step));
            glo.push(format!("{:.*}", res_usize, lon + (i as f64 - 1.5) * step));
        }
        (g.join("|"), gla.join(","), glo.join(","))
    };

    template
        .replace("{x}", &format!("{}", x))
        .replace("{y}", &format!("{}", y))
        .replace("{z}", &format!("{}", z))
        .replace("{grid_lat}", &grid_lat_str)
        .replace("{grid_lon}", &grid_lon_str)
        .replace("{grid}", &grid_str)
        .replace("{lat}", &lat_str)
        .replace("{lon}", &lon_str)
        .replace("{lat_min}", &lat_min_str)
        .replace("{lat_max}", &lat_max_str)
        .replace("{lon_min}", &lon_min_str)
        .replace("{lon_max}", &lon_max_str)
        .replace("{today}", &today)
        .replace("{yesterday}", &yesterday)
        .replace("{tomorrow}", &tomorrow)
        .replace("{today_yyyymmdd}", &today_yyyymmdd)
        .replace("{today_ymd}", &today_yyyymmdd)
        .replace("{today_nodashes}", &today_nodashes)
        .replace("{yesterday_nodashes}", &yesterday_nodashes)
        .replace("{tomorrow_nodashes}", &tomorrow_nodashes)
        .replace("{t_start}", &yesterday)
        .replace("{t_end}", &today)
        .replace("{now}", &now_iso)
        .replace("{now_minus_1}", &now_minus_1)
        .replace("{now_minus_2}", &now_minus_2)
        .replace("{week_ago}", &week_ago)
        .replace("{week_ago_nodashes}", &week_ago_nodashes)
        .replace(
            "{today_plus_365}",
            &format!("{}-{:02}-{:02}", ty + 1, tm, td),
        )
        .replace("{lat_int}", &format!("{}", lat as i32))
        .replace("{lon_int}", &format!("{}", lon as i32))
        .replace("{hour_ago}", &hour_ago)
        .replace("{year}", &ty.to_string())
        .replace("{year2}", &format!("{:02}", year2))
        .replace("{month}", &tm.to_string())
        .replace("{day}", &td.to_string())
        .replace("{yday}", &format!("{:03}", yday))
        .replace("{hour}", &format!("{:02}", q_hour))
        .replace("{minute}", &format!("{:02}", q_minute))
        .replace("{unix_now}", &unix_now)
        .replace("{unix_now_plus_3600}", &unix_now_plus_3600)
        .replace("{nasa_key}", "DEMO_KEY")
}

pub fn sha1(data: &[u8]) -> [u8; 20] {
    let mut h: [u32; 5] = [0x67452301, 0xEFCDAB89, 0x98BADCFE, 0x10325476, 0xC3D2E1F0];
    let bl = (data.len() as u64) * 8;
    let mut m = data.to_vec();
    m.push(0x80);
    while m.len() % 64 != 56 {
        m.push(0);
    }
    m.extend_from_slice(&bl.to_be_bytes());
    for chunk in m.chunks(64) {
        let mut w = [0u32; 80];
        for i in 0..16 {
            w[i] = u32::from_be_bytes([
                chunk[i * 4],
                chunk[i * 4 + 1],
                chunk[i * 4 + 2],
                chunk[i * 4 + 3],
            ]);
        }
        for i in 16..80 {
            w[i] = (w[i - 3] ^ w[i - 8] ^ w[i - 14] ^ w[i - 16]).rotate_left(1);
        }
        let (mut a, mut b, mut c, mut d, mut e) = (h[0], h[1], h[2], h[3], h[4]);
        for i in 0..80 {
            let (f, k) = match i {
                0..=19 => ((b & c) | ((!b) & d), 0x5A827999),
                20..=39 => (b ^ c ^ d, 0x6ED9EBA1),
                40..=59 => ((b & c) | (b & d) | (c & d), 0x8F1BBCDC),
                _ => (b ^ c ^ d, 0xCA62C1D6),
            };
            let temp = a
                .rotate_left(5)
                .wrapping_add(f)
                .wrapping_add(e)
                .wrapping_add(k)
                .wrapping_add(w[i]);
            e = d;
            d = c;
            c = b.rotate_left(30);
            b = a;
            a = temp;
        }
        h[0] = h[0].wrapping_add(a);
        h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(c);
        h[3] = h[3].wrapping_add(d);
        h[4] = h[4].wrapping_add(e);
    }
    let mut r = [0u8; 20];
    for i in 0..5 {
        r[i * 4..i * 4 + 4].copy_from_slice(&h[i].to_be_bytes());
    }
    r
}

pub fn split_data_line(line: &str) -> Vec<&str> {
    if line.contains('|') && line.split('|').count() > 2 {
        line.split('|')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect()
    } else if line.contains('\t') && line.split('\t').count() > 2 {
        line.split('\t')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect()
    } else if line.contains(';') {
        line.split(';')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect()
    } else if line.contains(',') && line.split(',').count() > 2 {
        line.split(',')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect()
    } else {
        line.split_whitespace().collect()
    }
}

pub fn extract_pending(src: &SourceConfig, body: &str, now: f64) -> Vec<PendingSample> {
    let mut pending: Vec<PendingSample> = Vec::new();
    let mut extracted: HashMap<String, f64> = HashMap::new();
    let parsed_json = if src.format == "json" || src.format.is_empty() {
        parse_json(body)
    } else if src.format == "votable" {
        votable_to_json(body).and_then(|j| parse_json(&j))
    } else if src.format == "csv" {
        csv_to_json(body).and_then(|j| parse_json(&j))
    } else if src.format.starts_with("html_table") {
        let table_idx: usize = if src.format.len() > 10 {
            src.format[11..].parse().unwrap_or(0)
        } else {
            0
        };
        html_table_to_json(body, table_idx).and_then(|j| parse_json(&j))
    } else if src.format == "erddap" {
        erddap_csv_to_json(body).and_then(|j| parse_json(&j))
    } else if src.format == "stac" {
        stac_to_json(body).and_then(|j| parse_json(&j))
    } else if src.format == "universal" {
        parse_json(body)
    } else {
        None
    };
    let auto_extracts: Option<Vec<Extract>>;
    let effective_extracts: &[Extract] = if src.format == "universal" && src.extracts.is_empty() {
        if let Some(ref j) = parsed_json {
            auto_extracts = Some(universal_auto_detect(j));
            auto_extracts.as_ref().map(|v| v.as_slice()).unwrap_or(&[])
        } else {
            &[]
        }
    } else {
        &src.extracts
    };
    for ext in effective_extracts {
        match ext {
            Extract::Field(k, n) => {
                if let Some(ref j) = parsed_json {
                    if let Some(v) = jnum(j, k) {
                        extracted.insert(n.clone(), v);
                    }
                }
            }
            Extract::First(k, n) => {
                if let Some(ref j) = parsed_json {
                    if let Some(v) = jfirst(j, k) {
                        extracted.insert(n.clone(), v);
                    }
                }
            }
            Extract::Last(k, n) => {
                if let Some(ref j) = parsed_json {
                    if let Some(v) = jlast(j, k) {
                        extracted.insert(n.clone(), v);
                    }
                } else if k == "line" {
                    if let Some(v) = body
                        .lines()
                        .rev()
                        .filter(|l| {
                            let t = l.trim();
                            !t.is_empty() && !t.starts_with('#')
                        })
                        .find_map(|l| {
                            split_data_line(l)
                                .last()
                                .and_then(|c| c.trim_matches('"').parse::<f64>().ok())
                        })
                    {
                        extracted.insert(n.clone(), v);
                    }
                }
            }
            Extract::Count(k, n) => {
                let v = if src.format == "csv" || k == "lines" {
                    Some(
                        body.lines()
                            .filter(|l| !l.trim().is_empty() && !l.trim().starts_with('#'))
                            .count() as f64,
                    )
                } else {
                    parsed_json.as_ref().and_then(|j| jcount(j, k))
                };
                if let Some(v) = v {
                    extracted.insert(n.clone(), v);
                }
            }
            Extract::LastRow(k, n) => {
                if src.format == "csv" {
                    if let Some(v) = text_last_col(body, k) {
                        extracted.insert(n.clone(), v);
                    }
                } else if let Some(ref j) = parsed_json {
                    if let Some(v) = j2d_last_row(j, k) {
                        extracted.insert(n.clone(), v);
                    }
                } else if let Some(v) = text_last_col(body, k) {
                    extracted.insert(n.clone(), v);
                }
            }
            Extract::Path(k, n) => {
                if let Some(ref j) = parsed_json {
                    if let Some(v) = jpath(j, k) {
                        extracted.insert(n.clone(), v);
                    }
                }
            }
            Extract::Deep(k, n) => {
                if let Some(ref j) = parsed_json {
                    if let Some(v) = jdeep_find_num(j, k) {
                        extracted.insert(n.clone(), v);
                    }
                }
            }
            Extract::LastLine(n) => {
                if let Some(v) = body
                    .lines()
                    .filter(|l| {
                        let t = l.trim();
                        !t.is_empty() && !t.starts_with('#')
                    })
                    .last()
                    .and_then(|line| {
                        split_data_line(line)
                            .into_iter()
                            .filter_map(|t| t.parse::<f64>().ok())
                            .last()
                    })
                {
                    extracted.insert(n.clone(), v);
                }
            }
            Extract::ObjLast(k, n) => {
                if let Some(ref j) = parsed_json {
                    if let Some(obj) = jpath_val(j, k) {
                        if let JsonVal::Obj(m) = obj {
                            if let Some(last_key) = m.keys().max_by(|a, b| {
                                let ka = a.parse::<i64>().unwrap_or(0);
                                let kb = b.parse::<i64>().unwrap_or(0);
                                ka.cmp(&kb)
                            }) {
                                if let Some(val) = m.get(last_key).and_then(scalar_of) {
                                    extracted.insert(n.clone(), val);
                                }
                            }
                        }
                    }
                }
            }
            Extract::Regex(pat, n) => {
                if let Some(v) = extract_regex_val(body, pat) {
                    extracted.insert(n.clone(), v);
                }
            }
            Extract::XmlCount(tag, n) => {
                let count = body.matches(&format!("<{}>", tag)).count() as f64;
                extracted.insert(n.clone(), count);
            }
            Extract::Ephemeris(n) => {
                let ht = if let Some(ref j) = parsed_json {
                    if let JsonVal::Obj(m) = j {
                        if let Some(JsonVal::Str(s)) = m.get("result") {
                            s.clone()
                        } else {
                            eprintln!("EPH no result key fmt={}", src.format);
                            body.to_string()
                        }
                    } else {
                        eprintln!("EPH not obj fmt={}", src.format);
                        body.to_string()
                    }
                } else {
                    eprintln!(
                        "EPH parse_json failed fmt={} len={} head={:?}",
                        src.format,
                        body.len(),
                        body.get(..40.min(body.len()))
                    );
                    body.to_string()
                };
                if let Some(soe) = ht.find("$$SOE") {
                    let a = &ht[soe + 5..];
                    let e = a.find("$$EOE").unwrap_or(a.len());
                    let blk = &a[..e];
                    let mut rows: Vec<(f64, [f64; 3], [f64; 3], Option<f64>)> = Vec::new();
                    let mut cur_jd: Option<f64> = None;
                    let mut cur_p: Option<[f64; 3]> = None;
                    let mut cur_v: Option<[f64; 3]> = None;
                    let mut cur_rg: Option<f64> = None;
                    for line in blk.lines() {
                        let t = line.trim();
                        if t.is_empty() {
                            continue;
                        }
                        let c0 = t.chars().next().unwrap_or(' ');
                        if c0.is_ascii_digit() {
                            if let (Some(jd), Some(p), Some(v)) = (cur_jd, cur_p, cur_v) {
                                rows.push((jd, p, v, cur_rg));
                            }
                            cur_jd = t
                                .split('=')
                                .next()
                                .and_then(|s| s.trim().parse::<f64>().ok());
                            cur_p = None;
                            cur_v = None;
                            cur_rg = None;
                        } else if t.starts_with("VX") {
                            cur_v = horizons_nums(t, ["VX", "VY", "VZ"]);
                        } else if t.starts_with('X') {
                            cur_p = horizons_nums(t, ["X", "Y", "Z"]);
                        } else if t.starts_with("LT") {
                            if let Some(r) = horizons_nums(t, ["RG", "LT", "RR"]) {
                                cur_rg = Some(r[0]);
                            }
                        }
                    }
                    if let (Some(jd), Some(p), Some(v)) = (cur_jd, cur_p, cur_v) {
                        rows.push((jd, p, v, cur_rg));
                    }
                    if let Some(&(jd0, p0k, v0k, rg0)) = rows.first() {
                        let p0 =
                            ecliptic_to_field([p0k[0] * 1000.0, p0k[1] * 1000.0, p0k[2] * 1000.0]);
                        let row_epoch = (jd0 - J2000_EPOCH) * 86400.0;
                        let v = if rows.len() > 1 {
                            let &(jd1, p1k, _, _) = rows.last().unwrap_or(&rows[0]);
                            let p1 = ecliptic_to_field([
                                p1k[0] * 1000.0,
                                p1k[1] * 1000.0,
                                p1k[2] * 1000.0,
                            ]);
                            let dt = (jd1 - jd0) * 86400.0;
                            if dt > 0.0 {
                                [
                                    (p1[0] - p0[0]) / dt,
                                    (p1[1] - p0[1]) / dt,
                                    (p1[2] - p0[2]) / dt,
                                ]
                            } else {
                                ecliptic_to_field([
                                    v0k[0] * 1000.0,
                                    v0k[1] * 1000.0,
                                    v0k[2] * 1000.0,
                                ])
                            }
                        } else {
                            ecliptic_to_field([v0k[0] * 1000.0, v0k[1] * 1000.0, v0k[2] * 1000.0])
                        };
                        let shift = now - row_epoch;
                        let p_now = [
                            p0[0] + v[0] * shift,
                            p0[1] + v[1] * shift,
                            p0[2] + v[2] * shift,
                        ];
                        let mut fields = Vec::new();
                        if let Some(rg) = rg0 {
                            fields.push((n.clone(), rg * 1000.0));
                        }
                        pending.push(PendingSample {
                            epoch: now,
                            position: PendingPosition::StateVector {
                                p: p_now,
                                v,
                                track: true,
                            },
                            fields,
                        });
                    }
                }
            }
            Extract::Vectors(n) => {
                let ht = if let Some(ref j) = parsed_json {
                    if let JsonVal::Obj(m) = j {
                        if let Some(JsonVal::Str(s)) = m.get("result") {
                            s.clone()
                        } else {
                            eprintln!("VEC no result key fmt={}", src.format);
                            body.to_string()
                        }
                    } else {
                        eprintln!("VEC not obj fmt={}", src.format);
                        body.to_string()
                    }
                } else {
                    eprintln!(
                        "VEC parse_json failed fmt={} len={} head={:?}",
                        src.format,
                        body.len(),
                        body.get(..40.min(body.len()))
                    );
                    body.to_string()
                };
                if let Some(soe) = ht.find("$$SOE") {
                    let a = &ht[soe + 5..];
                    let e = a.find("$$EOE").unwrap_or(a.len());
                    let blk = &a[..e];
                    let mut cur_jd: Option<f64> = None;
                    let mut cur_p: Option<[f64; 3]> = None;
                    let mut cur_v: Option<[f64; 3]> = None;
                    let mut cur_rg: Option<f64> = None;
                    for line in blk.lines() {
                        let t = line.trim();
                        if t.is_empty() {
                            continue;
                        }
                        let c0 = t.chars().next().unwrap_or(' ');
                        if c0.is_ascii_digit() {
                            if let (Some(jd), Some(p), Some(v)) = (cur_jd, cur_p, cur_v) {
                                let p_f = ecliptic_to_field([
                                    p[0] * 1000.0,
                                    p[1] * 1000.0,
                                    p[2] * 1000.0,
                                ]);
                                let v_f = ecliptic_to_field([
                                    v[0] * 1000.0,
                                    v[1] * 1000.0,
                                    v[2] * 1000.0,
                                ]);
                                let row_epoch = (jd - J2000_EPOCH) * 86400.0;
                                let mut fields = Vec::new();
                                if let Some(rg) = cur_rg {
                                    fields.push((n.clone(), rg * 1000.0));
                                }
                                pending.push(PendingSample {
                                    epoch: row_epoch,
                                    position: PendingPosition::StateVector {
                                        p: p_f,
                                        v: v_f,
                                        track: true,
                                    },
                                    fields,
                                });
                            }
                            cur_jd = t
                                .split('=')
                                .next()
                                .and_then(|s| s.trim().parse::<f64>().ok());
                            cur_p = None;
                            cur_v = None;
                            cur_rg = None;
                        } else if t.starts_with("VX") {
                            cur_v = horizons_nums(t, ["VX", "VY", "VZ"]);
                        } else if t.starts_with('X') {
                            cur_p = horizons_nums(t, ["X", "Y", "Z"]);
                        } else if t.starts_with("LT") {
                            if let Some(r) = horizons_nums(t, ["RG", "LT", "RR"]) {
                                cur_rg = Some(r[0]);
                            }
                        }
                    }
                    if let (Some(jd), Some(p), Some(v)) = (cur_jd, cur_p, cur_v) {
                        let p_f = ecliptic_to_field([p[0] * 1000.0, p[1] * 1000.0, p[2] * 1000.0]);
                        let v_f = ecliptic_to_field([v[0] * 1000.0, v[1] * 1000.0, v[2] * 1000.0]);
                        let row_epoch = (jd - J2000_EPOCH) * 86400.0;
                        let mut fields = Vec::new();
                        if let Some(rg) = cur_rg {
                            fields.push((n.clone(), rg * 1000.0));
                        }
                        pending.push(PendingSample {
                            epoch: row_epoch,
                            position: PendingPosition::StateVector {
                                p: p_f,
                                v: v_f,
                                track: true,
                            },
                            fields,
                        });
                    }
                }
            }
            Extract::LastObj(fk, fv, ek, n) => {
                if let Some(ref j) = parsed_json {
                    if let JsonVal::Arr(arr) = j {
                        for v in arr.iter().rev() {
                            if let JsonVal::Obj(o) = v {
                                if let Some(JsonVal::Str(s)) = o.get(fk) {
                                    if s == fv {
                                        if let Some(val) = jnum(v, ek) {
                                            extracted.insert(n.clone(), val);
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Extract::Map {
                arr_path,
                lat_key,
                lon_key,
                alt_key,
                epoch_key,
                val_key,
                alt_sign,
                vel_key,
                trk_key,
                vr_key,
                fields,
                lon_sign,
            } => {
                let (eff_lat_key, eff_lon_key, eff_epoch_key) = if src.format == "erddap"
                    && (lat_key.is_empty() || lon_key.is_empty())
                {
                    if let Some(ref j) = parsed_json {
                        if let Some(JsonVal::Arr(arr)) = jpath_val(j, arr_path) {
                            if let Some(first) = arr.first() {
                                if let JsonVal::Obj(map) = first {
                                    let detect = |candidates: &[&str]| -> String {
                                        for c in candidates {
                                            if map.contains_key(*c) {
                                                return c.to_string();
                                            }
                                        }
                                        for (k, _) in map.iter() {
                                            let kl = k.to_lowercase();
                                            if candidates.iter().any(|c| kl.contains(c)) {
                                                return k.clone();
                                            }
                                        }
                                        String::new()
                                    };
                                    let lk = if lat_key.is_empty() {
                                        detect(&["latitude", "lat", "LAT", "LATITUDE"])
                                    } else {
                                        lat_key.clone()
                                    };
                                    let lnk = if lon_key.is_empty() {
                                        detect(&["longitude", "lon", "LON", "LONGITUDE", "long"])
                                    } else {
                                        lon_key.clone()
                                    };
                                    let ek = if epoch_key.is_empty() {
                                        detect(&["time", "TIME", "Time", "date", "DATE"])
                                    } else {
                                        epoch_key.clone()
                                    };
                                    (lk, lnk, ek)
                                } else {
                                    (lat_key.clone(), lon_key.clone(), epoch_key.clone())
                                }
                            } else {
                                (lat_key.clone(), lon_key.clone(), epoch_key.clone())
                            }
                        } else {
                            (lat_key.clone(), lon_key.clone(), epoch_key.clone())
                        }
                    } else {
                        (lat_key.clone(), lon_key.clone(), epoch_key.clone())
                    }
                } else {
                    (lat_key.clone(), lon_key.clone(), epoch_key.clone())
                };
                if let Some(ref j) = parsed_json {
                    if let Some(JsonVal::Arr(arr)) = jpath_val(j, arr_path) {
                        for v in arr.iter() {
                            let lat = jpath(v, &eff_lat_key);
                            let lon = jpath(v, &eff_lon_key);
                            let alt = if alt_key.is_empty() {
                                Some(0.0)
                            } else {
                                jpath(v, alt_key).map(|a| a * alt_sign)
                            };
                            if let (Some(la), Some(lo), Some(al)) = (lat, lon, alt) {
                                let mut ev_fields: Vec<(String, f64)> = Vec::new();
                                for (fk, fn_) in fields {
                                    if let Some(val) = jpath(v, fk) {
                                        ev_fields.push((fn_.clone(), val));
                                    }
                                }
                                if val_key.is_empty() {
                                    if ev_fields.is_empty() {
                                        continue;
                                    }
                                } else {
                                    ev_fields.retain(|(fn_, _)| *fn_ == *val_key);
                                    if ev_fields.is_empty() {
                                        continue;
                                    }
                                }
                                let mut lon_val = lo;
                                if let Some(sign_key) = lon_sign {
                                    if let Some(vv) = jpath_val(v, sign_key) {
                                        if let JsonVal::Str(s) = vv {
                                            if s.contains('W')
                                                || s.contains('w')
                                                || s.contains('S')
                                                || s.contains('s')
                                            {
                                                lon_val = -lo;
                                            }
                                        }
                                    }
                                }
                                let speed = if vel_key.is_empty() {
                                    None
                                } else {
                                    jpath(v, vel_key)
                                };
                                let track = if trk_key.is_empty() {
                                    None
                                } else {
                                    jpath(v, trk_key)
                                };
                                let vrate = if vr_key.is_empty() {
                                    None
                                } else {
                                    jpath(v, vr_key)
                                };
                                let position = if let (Some(sp), Some(tr)) = (speed, track) {
                                    PendingPosition::GeodeticFlow {
                                        lat: la,
                                        lon: lon_val,
                                        alt: al,
                                        speed: sp,
                                        track: tr,
                                        vrate: vrate.unwrap_or(0.0),
                                    }
                                } else {
                                    PendingPosition::Geodetic {
                                        lat: la,
                                        lon: lon_val,
                                        alt: al,
                                    }
                                };
                                let epoch = if eff_epoch_key.is_empty() {
                                    now
                                } else {
                                    jpath_val(v, &eff_epoch_key)
                                        .and_then(|ev| match ev {
                                            JsonVal::Str(s) => parse_iso_tdb(s),
                                            JsonVal::Num(n) => Some(*n - UNIX_J2000_OFFSET),
                                            _ => None,
                                        })
                                        .unwrap_or(now)
                                };
                                pending.push(PendingSample {
                                    epoch,
                                    position,
                                    fields: ev_fields,
                                });
                            }
                        }
                    }
                }
            }
            Extract::Flatten {
                arr_path,
                geom_path,
                epoch_key,
                fields,
            } => {
                let _default_epoch = src.catalog_epoch.unwrap_or(now);
                if let Some(ref j) = parsed_json {
                    if let Some(JsonVal::Arr(arr)) = jpath_val(j, arr_path) {
                        for (idx, v) in arr.iter().enumerate() {
                            let geom = match jpath_val(v, geom_path) {
                                Some(g) => g,
                                None => continue,
                            };
                            let coords = match jpath_val(geom, "coordinates") {
                                Some(JsonVal::Arr(c)) => c,
                                _ => continue,
                            };
                            let vertices = flatten_geojson_coords(coords);
                            if vertices.is_empty() {
                                continue;
                            }
                            let mut ev_fields: Vec<(String, f64)> = Vec::new();
                            for (fk, fn_) in fields {
                                if let Some(val) = jpath(v, fk) {
                                    ev_fields.push((fn_.clone(), val));
                                }
                            }
                            ev_fields.push(("_flatten_id".to_string(), idx as f64));
                            for (lon, lat) in vertices {
                                let row_epoch = if !epoch_key.is_empty() {
                                    jpath(v, &epoch_key).unwrap_or(now)
                                } else {
                                    now
                                };
                                pending.push(PendingSample {
                                    epoch: row_epoch,
                                    position: PendingPosition::Geodetic { lat, lon, alt: 0.0 },
                                    fields: ev_fields.clone(),
                                });
                            }
                        }
                    }
                }
            }
            Extract::CmrPolygon {
                arr_path,
                fields,
                epoch_key,
                alt_key,
                val_key,
            } => {
                if let Some(ref j) = parsed_json {
                    if let Some(JsonVal::Arr(arr)) = jpath_val(j, arr_path) {
                        for (idx, v) in arr.iter().enumerate() {
                            let polys = match jpath_val(v, "polygons") {
                                Some(JsonVal::Arr(p)) => p,
                                _ => continue,
                            };
                            let mut vertices: Vec<(f64, f64)> = Vec::new();
                            for ring_list in polys {
                                if let JsonVal::Arr(rings) = ring_list {
                                    for ring_str_val in rings {
                                        if let JsonVal::Str(s) = ring_str_val {
                                            let nums: Vec<f64> = s
                                                .split_whitespace()
                                                .filter_map(|n| n.parse().ok())
                                                .collect();
                                            for pair in nums.chunks(2) {
                                                if pair.len() == 2 {
                                                    vertices.push((pair[1], pair[0]));
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            if vertices.is_empty() {
                                continue;
                            }
                            let epoch = if epoch_key.is_empty() {
                                now
                            } else {
                                jpath_val(v, epoch_key)
                                    .and_then(|ev| match ev {
                                        JsonVal::Str(s) => parse_iso_tdb(s),
                                        JsonVal::Num(n) => Some(*n - UNIX_J2000_OFFSET),
                                        _ => None,
                                    })
                                    .unwrap_or(now)
                            };
                            let mut ev_fields: Vec<(String, f64)> = Vec::new();
                            for (fk, fn_) in fields {
                                if let Some(val) = jpath(v, fk) {
                                    if val_key.is_empty() || fk == val_key {
                                        ev_fields.push((fn_.clone(), val));
                                    }
                                }
                            }
                            if ev_fields.is_empty() {
                                ev_fields.push(("_cmr_id".to_string(), idx as f64));
                            }
                            let alt = if alt_key.is_empty() {
                                0.0
                            } else {
                                jpath(v, alt_key).unwrap_or(0.0)
                            };
                            for (lon, lat) in vertices {
                                pending.push(PendingSample {
                                    epoch,
                                    position: PendingPosition::Geodetic { lat, lon, alt },
                                    fields: ev_fields.clone(),
                                });
                            }
                        }
                    }
                }
            }
            Extract::CelestialPolygon {
                arr_path,
                radius,
                fields,
                epoch_key,
                val_key,
            } => {
                if let Some(ref j) = parsed_json {
                    if let Some(JsonVal::Arr(arr)) = jpath_val(j, arr_path) {
                        for (idx, v) in arr.iter().enumerate() {
                            let geom = match jpath_val(v, "geometry") {
                                Some(g) => g,
                                None => continue,
                            };
                            let coords = match jpath_val(geom, "coordinates") {
                                Some(JsonVal::Arr(c)) => c,
                                _ => continue,
                            };
                            let vertices = flatten_geojson_coords(coords);
                            if vertices.is_empty() || *radius <= 0.0 {
                                continue;
                            }
                            let _epoch = if epoch_key.is_empty() {
                                now
                            } else {
                                jpath_val(v, epoch_key)
                                    .and_then(|ev| match ev {
                                        JsonVal::Str(s) => parse_iso_tdb(s),
                                        JsonVal::Num(n) => Some(*n - UNIX_J2000_OFFSET),
                                        _ => None,
                                    })
                                    .unwrap_or(now)
                            };
                            let mut ev_fields: Vec<(String, f64)> = Vec::new();
                            for (fk, fn_) in fields {
                                if let Some(val) = jpath(v, fk) {
                                    if val_key.is_empty() || fk == val_key {
                                        ev_fields.push((fn_.clone(), val));
                                    }
                                }
                            }
                            if ev_fields.is_empty() {
                                ev_fields.push(("_cpoly_id".to_string(), idx as f64));
                            }
                            for (ra_deg, dec_deg) in vertices {
                                let ra = ra_deg.to_radians();
                                let dec = dec_deg.to_radians();
                                let (sa, ca) = ra.sin_cos();
                                let (sd, cd) = dec.sin_cos();
                                let p = [cd * ca * radius, cd * sa * radius, sd * radius];
                                let row_epoch = if !epoch_key.is_empty() {
                                    jpath(v, &epoch_key).unwrap_or(now)
                                } else {
                                    now
                                };
                                pending.push(PendingSample {
                                    epoch: row_epoch,
                                    position: PendingPosition::StateVector {
                                        p,
                                        v: [0.0, 0.0, 0.0],
                                        track: false,
                                    },
                                    fields: ev_fields.clone(),
                                });
                            }
                        }
                    }
                }
            }
            Extract::Rows { fields } => {
                if let Frame::WGS84 { lat, lon, alt } | Frame::IAU2000 { lat, lon, alt } = src.frame
                {
                    let col_indices: Vec<(usize, &String)> = fields
                        .iter()
                        .filter_map(|(fk, fn_)| {
                            if let Ok(idx) = fk.parse::<usize>() {
                                Some((idx, fn_))
                            } else {
                                for line in body.lines() {
                                    let t = line.trim();
                                    if t.is_empty() {
                                        continue;
                                    }
                                    let s = t.strip_prefix('#').unwrap_or(t).trim();
                                    if let Some(idx) = split_data_line(s).iter().position(|c| {
                                        c.eq_ignore_ascii_case(fk) || c.starts_with(fk)
                                    }) {
                                        return Some((idx, fn_));
                                    }
                                }
                                None
                            }
                        })
                        .collect();
                    for line in body.lines() {
                        let trimmed = line.trim();
                        if trimmed.is_empty() || trimmed.starts_with('#') {
                            continue;
                        }
                        let cols = split_data_line(trimmed);
                        let mut ev_fields: Vec<(String, f64)> = Vec::new();
                        for (idx, fn_) in &col_indices {
                            if let Some(val) = cols
                                .get(*idx)
                                .and_then(|s| s.trim().trim_matches('"').parse::<f64>().ok())
                            {
                                ev_fields.push((fn_.to_string(), val));
                            }
                        }
                        if ev_fields.is_empty() {
                            continue;
                        }
                        pending.push(PendingSample {
                            epoch: now,
                            position: PendingPosition::Geodetic { lat, lon, alt },
                            fields: ev_fields,
                        });
                    }
                }
            }
            Extract::KeplerMap {
                arr_path,
                a_key,
                e_key,
                i_key,
                om_key,
                w_key,
                ma_key,
                epoch_key,
                fields,
            } => {
                if let Some(ref j) = parsed_json {
                    if let Some(JsonVal::Arr(arr)) = jpath_val(j, arr_path) {
                        let jd_now = tdb_to_jd(now);
                        for v in arr.iter() {
                            let (
                                Some(a_val),
                                Some(e_val),
                                Some(i_val),
                                Some(om_val),
                                Some(w_val),
                                Some(ma_val),
                                Some(epoch_val),
                            ) = (
                                jpath(v, a_key),
                                jpath(v, e_key),
                                jpath(v, i_key),
                                jpath(v, om_key),
                                jpath(v, w_key),
                                jpath(v, ma_key),
                                jpath(v, epoch_key),
                            )
                            else {
                                continue;
                            };
                            let a_au = a_val.max(1e-10);
                            let n = GAUSS_K * (1.0 / (a_au * a_au * a_au)).sqrt();
                            let m = ma_val.to_radians() + n * (jd_now - epoch_val);
                            let e = e_val.clamp(0.0, 0.999);
                            let mut e_anom = m;
                            for _ in 0..5 {
                                e_anom = e_anom
                                    - (e_anom - e * e_anom.sin() - m) / (1.0 - e * e_anom.cos());
                            }
                            let (cos_e, sin_e) = (e_anom.cos(), e_anom.sin());
                            let sqrt_1me2 = (1.0 - e * e).sqrt();
                            let x_orb = a_au * (cos_e - e);
                            let y_orb = a_au * sqrt_1me2 * sin_e;
                            let r = a_au * (1.0 - e * cos_e);
                            let n_rad_s = n / 86400.0;
                            let vx_orb = -a_au * sin_e * n_rad_s / r;
                            let vy_orb = a_au * sqrt_1me2 * cos_e * n_rad_s / r;
                            let (sin_om, cos_om) = om_val.to_radians().sin_cos();
                            let (sin_i, cos_i) = i_val.to_radians().sin_cos();
                            let (sin_w, cos_w) = w_val.to_radians().sin_cos();
                            let x1 = cos_w * x_orb - sin_w * y_orb;
                            let y1 = sin_w * x_orb + cos_w * y_orb;
                            let vx1 = cos_w * vx_orb - sin_w * vy_orb;
                            let vy1 = sin_w * vx_orb + cos_w * vy_orb;
                            let p = [
                                (cos_om * x1 - sin_om * cos_i * y1) * AU,
                                (sin_om * x1 + cos_om * cos_i * y1) * AU,
                                sin_i * y1 * AU,
                            ];
                            let vel = [
                                (cos_om * vx1 - sin_om * cos_i * vy1) * AU,
                                (sin_om * vx1 + cos_om * cos_i * vy1) * AU,
                                sin_i * vy1 * AU,
                            ];
                            let mut ev_fields: Vec<(String, f64)> = Vec::new();
                            for (fk, fn_) in fields {
                                if let Some(val) = jpath(v, fk) {
                                    ev_fields.push((fn_.clone(), val));
                                }
                            }
                            pending.push(PendingSample {
                                epoch: now,
                                position: PendingPosition::StateVector {
                                    p,
                                    v: vel,
                                    track: false,
                                },
                                fields: ev_fields,
                            });
                        }
                    }
                }
            }
            Extract::CelestialMap {
                arr_path,
                ra_key,
                dec_key,
                dist_key,
                dist_scale,
                plx_key,
                z_key,
                pmra_key,
                pmdec_key,
                rv_key,
                rv_scale,
                epoch_key,
                fields,
            } => {
                let default_epoch = src.catalog_epoch.unwrap_or(now);
                if let Some(ref j) = parsed_json {
                    if let Some(JsonVal::Arr(arr)) = jpath_val(j, arr_path) {
                        for v in arr.iter() {
                            let (Some(ra_deg), Some(dec_deg)) =
                                (jpath(v, ra_key), jpath(v, dec_key))
                            else {
                                continue;
                            };
                            let d = if !plx_key.is_empty() {
                                match jpath(v, plx_key) {
                                    Some(plx) if plx > 0.0 => PARSEC_M * 1000.0 / plx,
                                    _ => {
                                        if !z_key.is_empty() {
                                            match jpath(v, z_key) {
                                                Some(z) if z > 0.0 => z * C_LIGHT / HUBBLE_H0,
                                                _ => continue,
                                            }
                                        } else if !dist_key.is_empty() {
                                            match jpath(v, dist_key) {
                                                Some(dd) if dd > 0.0 => dd * dist_scale,
                                                _ => continue,
                                            }
                                        } else {
                                            continue;
                                        }
                                    }
                                }
                            } else if !dist_key.is_empty() {
                                match jpath(v, dist_key) {
                                    Some(dd) if dd > 0.0 => dd * dist_scale,
                                    _ => {
                                        if !z_key.is_empty() {
                                            match jpath(v, z_key) {
                                                Some(z) if z > 0.0 => z * C_LIGHT / HUBBLE_H0,
                                                _ => continue,
                                            }
                                        } else {
                                            continue;
                                        }
                                    }
                                }
                            } else if !z_key.is_empty() {
                                match jpath(v, z_key) {
                                    Some(z) if z > 0.0 => z * C_LIGHT / HUBBLE_H0,
                                    _ => continue,
                                }
                            } else {
                                continue;
                            };
                            let mut ev_fields: Vec<(String, f64)> = Vec::new();
                            for (fk, fn_) in fields {
                                if let Some(val) = jpath(v, fk) {
                                    ev_fields.push((fn_.clone(), val));
                                }
                            }
                            ev_fields.push(("_dist_m".to_string(), d));
                            if let Some(ref mag_field) = src.abs_mag_from {
                                if let Some(idx) =
                                    ev_fields.iter().position(|(n, _)| n == mag_field)
                                {
                                    let mag = ev_fields[idx].1;
                                    let dist_pc = d / PARSEC_M;
                                    let abs_m = mag - 5.0 * (dist_pc / 10.0).log10();
                                    ev_fields[idx].1 = 10.0f64.powf(-0.4 * abs_m);
                                }
                            }
                            if ev_fields.is_empty() {
                                continue;
                            }
                            let ra = ra_deg.to_radians();
                            let dec = dec_deg.to_radians();
                            let (sa, ca) = ra.sin_cos();
                            let (sd, cd) = dec.sin_cos();
                            let p_hat = [cd * ca, cd * sa, sd];
                            let p = [p_hat[0] * d, p_hat[1] * d, p_hat[2] * d];
                            let mu_a = if pmra_key.is_empty() {
                                0.0
                            } else {
                                jpath(v, pmra_key).unwrap_or(0.0) * MAS_YR_TO_RAD_S
                            };
                            let mu_d = if pmdec_key.is_empty() {
                                0.0
                            } else {
                                jpath(v, pmdec_key).unwrap_or(0.0) * MAS_YR_TO_RAD_S
                            };
                            let vr = if rv_key.is_empty() {
                                0.0
                            } else {
                                jpath(v, rv_key).unwrap_or(0.0) * rv_scale
                            };
                            let a_hat = [-sa, ca, 0.0];
                            let d_hat = [-sd * ca, -sd * sa, cd];
                            let vel = [
                                d * (mu_a * a_hat[0] + mu_d * d_hat[0]) + vr * p_hat[0],
                                d * (mu_a * a_hat[1] + mu_d * d_hat[1]) + vr * p_hat[1],
                                d * (mu_a * a_hat[2] + mu_d * d_hat[2]) + vr * p_hat[2],
                            ];
                            let sample_epoch = if !epoch_key.is_empty() {
                                jpath(v, epoch_key).unwrap_or(default_epoch)
                            } else {
                                default_epoch
                            };
                            pending.push(PendingSample {
                                epoch: sample_epoch,
                                position: PendingPosition::StateVector {
                                    p,
                                    v: vel,
                                    track: false,
                                },
                                fields: ev_fields,
                            });
                        }
                    }
                }
            }
            Extract::GeojsonEvents {
                mag_key,
                min_mag,
                outputs,
            } => {
                if outputs.len() >= 2 {
                    if let Some(ref j) = parsed_json {
                        if let JsonVal::Obj(root) = j {
                            if let Some(JsonVal::Arr(features)) = root.get("features") {
                                for feat in features {
                                    if let JsonVal::Obj(f) = feat {
                                        let mut elo = 0.0;
                                        let mut ela = 0.0;
                                        let mut ed = 0.0;
                                        let mut mag = 0.0;
                                        let mut valid = false;
                                        if let Some(JsonVal::Obj(geom)) = f.get("geometry") {
                                            if let Some(JsonVal::Arr(c)) = geom.get("coordinates") {
                                                if c.len() >= 3 {
                                                    if let JsonVal::Num(n) = c[0] {
                                                        elo = n;
                                                    }
                                                    if let JsonVal::Num(n) = c[1] {
                                                        ela = n;
                                                    }
                                                    if let JsonVal::Num(n) = c[2] {
                                                        ed = n;
                                                    }
                                                    valid = true;
                                                }
                                            }
                                        }
                                        if valid {
                                            if let Some(props) = f.get("properties") {
                                                if let Some(m) = jnum(props, mag_key) {
                                                    mag = m;
                                                }
                                            }
                                            if mag >= *min_mag {
                                                pending.push(PendingSample {
                                                    epoch: now,
                                                    position: PendingPosition::Geodetic {
                                                        lat: ela,
                                                        lon: elo,
                                                        alt: -ed * 1000.0,
                                                    },
                                                    fields: vec![
                                                        (outputs[0].clone(), mag),
                                                        (outputs[1].clone(), ed * 1000.0),
                                                    ],
                                                });
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Extract::Hapi(pairs) => {
                if let Some(ref j) = parsed_json {
                    if let JsonVal::Obj(root) = j {
                        if let (Some(JsonVal::Arr(data)), Some(JsonVal::Arr(params))) =
                            (root.get("data"), root.get("parameters"))
                        {
                            let mut col: HashMap<String, usize> = HashMap::new();
                            for (i, p) in params.iter().enumerate() {
                                if let JsonVal::Obj(po) = p {
                                    if let Some(JsonVal::Str(nn)) = po.get("name") {
                                        col.insert(nn.clone(), i);
                                    }
                                }
                            }
                            if let Some(last_row) = data.last() {
                                if let JsonVal::Arr(row) = last_row {
                                    for (param, name) in pairs {
                                        if let Some(&idx) = col.get(param) {
                                            if let Some(val) = row.get(idx).and_then(scalar_of) {
                                                extracted.insert(name.clone(), val);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if !extracted.is_empty() {
        pending.push(PendingSample {
            epoch: now,
            position: PendingPosition::Source,
            fields: extracted.into_iter().collect(),
        });
    }
    for p in &mut pending {
        p.fields.retain(|(_, v)| v.is_finite());
        if let Some(ref mag_key) = src.flux_from_mag {
            if let Some((_, v)) = p
                .fields
                .iter_mut()
                .find(|(n, _)| n.as_str() == mag_key.as_str())
            {
                *v = 10.0f64.powf(-0.4 * *v);
            }
        }
        if let Some(ce) = src.catalog_epoch {
            p.epoch = ce;
        }
    }
    pending.retain(|p| !p.fields.is_empty());
    pending
}

pub fn materialize(
    src: &SourceConfig,
    origin: Origin,
    region: Option<(f64, f64)>,
    pend: PendingSample,
    origins: &mut HashMap<Origin, OriginState>,
) -> Vec<Sample> {
    if pend.fields.is_empty() {
        return vec![];
    }
    let vmax_floor = 0.0f64;
    let motion = match &pend.position {
        PendingPosition::StateVector { p, v, .. } => Motion::Linear { p: *p, v: *v },
        PendingPosition::Geodetic { lat, lon, alt } => Motion::WGS84 {
            lat: *lat,
            lon: *lon,
            alt: *alt,
        },
        PendingPosition::GeodeticFlow {
            lat,
            lon,
            alt,
            speed,
            track,
            vrate,
        } => flow_motion(*lat, *lon, *alt, *speed, *track, *vrate, pend.epoch),
        PendingPosition::Source => match &src.frame {
            Frame::WGS84 { lat, lon, alt } => Motion::WGS84 {
                lat: *lat,
                lon: *lon,
                alt: *alt,
            },
            Frame::IAU2000 { lat, lon, alt } => Motion::IAU2000 {
                lat: *lat,
                lon: *lon,
                alt: *alt,
            },
            Frame::Ecliptic { scale } => Motion::Ecliptic { scale: *scale },
            Frame::Areocentric { scale } => Motion::Areocentric { scale: *scale },
            Frame::Query => {
                let Some((lat, lon)) = region else {
                    return vec![];
                };
                Motion::WGS84 { lat, lon, alt: 0.0 }
            }
            Frame::Data => {
                let Some((latf, lonf, altf, alt_scale)) = src.pos_fields.as_ref() else {
                    return vec![];
                };
                let find = |k: &str| pend.fields.iter().find(|(n, _)| n == k).map(|(_, v)| *v);
                let Some(lat) = find(latf) else { return vec![] };
                let Some(lon) = find(lonf) else { return vec![] };
                let alt = match altf {
                    Some(k) => {
                        let Some(v) = find(k) else { return vec![] };
                        v * alt_scale
                    }
                    None => 0.0,
                };
                Motion::WGS84 { lat, lon, alt }
            }
        },
    };
    let abs = motion.at(pend.epoch, pend.epoch);
    if abs[0].is_nan() || abs[1].is_nan() || abs[2].is_nan() || pend.epoch.is_nan() {
        return vec![];
    }
    let mut resid_ema = 0.0;
    let track_origin = matches!(pend.position, PendingPosition::Source)
        || matches!(
            pend.position,
            PendingPosition::StateVector { track: true, .. }
        );
    if track_origin {
        let entry = origins.entry(origin).or_default();
        if entry.has_prev {
            let dt = (pend.epoch - entry.prev_epoch).abs().max(1.0);
            if let Some(pm) = &entry.prev_motion {
                let pred = pm.at(pend.epoch, entry.prev_epoch);
                let resid = ((pred[0] - abs[0]).powi(2)
                    + (pred[1] - abs[1]).powi(2)
                    + (pred[2] - abs[2]).powi(2))
                .sqrt();
                let alpha = 1.0 - (-dt / src.ttl.max(1) as f64).exp();
                entry.resid_ema += (resid / dt - entry.resid_ema) * alpha;
            }
        }
        resid_ema = entry.resid_ema;
        entry.prev_epoch = pend.epoch;
        entry.prev_abs = abs;
        entry.prev_motion = Some(motion);
        entry.has_prev = true;
    }
    let (mut vmax, amax, p0f) = law_bounds(&motion, pend.epoch, resid_ema);
    if p0f[0].is_nan() || p0f[1].is_nan() || p0f[2].is_nan() || vmax.is_nan() || amax.is_nan() {
        return vec![];
    }
    if vmax_floor > vmax {
        vmax = vmax_floor;
    }
    let forces: Vec<(f64, f64, bool, u8)> = src
        .force
        .split_whitespace()
        .filter_map(|f| force_constants(f))
        .collect();
    if forces.is_empty() {
        return vec![];
    }
    let clean_fields: Vec<(String, f64)> = pend
        .fields
        .iter()
        .filter(|(_, v)| !v.is_nan() && v.is_finite())
        .cloned()
        .collect();
    forces
        .into_iter()
        .filter_map(|(v_or_d, tau_default, is_diff, force_type)| {
            let tau = src
                .tau
                .or_else(|| {
                    src.tau_key.as_ref().and_then(|k| {
                        clean_fields.iter().find(|(n, _)| n == k).map(|(_, v)| {
                            if is_diff {
                                *v / v_or_d
                            } else {
                                *v / v_or_d
                            }
                        })
                    })
                })
                .unwrap_or(tau_default);
            let effective_ttl = src.reach_ttl.unwrap_or(src.ttl) as f64;
            let reach_time = effective_ttl + tau;
            let extent = if is_diff {
                (2.0 * v_or_d * reach_time).sqrt()
            } else {
                v_or_d * reach_time
            };
            if extent.is_nan() || tau.is_nan() {
                return None;
            }
            Some(Sample {
                origin,
                epoch: pend.epoch,
                ttl: src.ttl.max(1) as f64,
                extent,
                tau,
                force_type: force_type as f64,
                vmax,
                amax,
                p0f,
                motion,
                fields: clean_fields.clone(),
            })
        })
        .collect()
}

pub fn render_headers(
    src: &SourceConfig,
    x: f64,
    y: f64,
    z: f64,
    now: f64,
    extent: f64,
) -> Vec<(String, String)> {
    src.headers
        .iter()
        .map(|(k, v)| (k.clone(), render_url(v, x, y, z, now, extent)))
        .collect()
}

pub fn fetch_priority(
    url: &str,
    pos: (f64, f64, f64),
    r: f64,
    is_new: bool,
    ttl: u64,
    presences: &[(f64, f64, f64, f64, f64)],
) -> u8 {
    if is_new {
        // First run: materialize near the presence first (First Light in seconds).
        // Proximity-weighted, short-TTL live sources first; big CDN catalogs last.
        let mut min_d = f64::INFINITY;
        for &(_, px, py, pz, _) in presences {
            let d = ((px - pos.0).powi(2) + (py - pos.1).powi(2) + (pz - pos.2).powi(2)).sqrt();
            if d < min_d {
                min_d = d;
            }
        }
        let proximity = if r > 0.0 {
            1.0 - (min_d / (r + min_d)).min(1.0)
        } else {
            0.0
        };
        let urgency = 1.0 - (ttl.max(1) as f64).log10().min(5.0) / 5.0;
        if url.contains("omegaflow/sources") {
            let x = (ttl.max(1) as f64).log2() / Φ;
            return ((255.0 * (1.0 - 1.0 / (1.0 + x))).max(128.0)) as u8;
        }
        return ((proximity * 0.7 + urgency * 0.3) * 240.0) as u8;
    }
    // Steady state: hybrid of proximity (how relevant to the observer)
    // and refresh urgency (short TTL = must refresh sooner).
    let mut min_d = f64::INFINITY;
    for &(_, px, py, pz, _) in presences {
        let d = ((px - pos.0).powi(2) + (py - pos.1).powi(2) + (pz - pos.2).powi(2)).sqrt();
        if d < min_d {
            min_d = d;
        }
    }
    let proximity = if r > 0.0 {
        1.0 - (min_d / (r + min_d)).min(1.0)
    } else {
        0.0
    };
    let urgency = 1.0 - (ttl.max(1) as f64).log10().min(5.0) / 5.0;
    if url.contains("omegaflow/sources") {
        let x = (ttl.max(1) as f64).log2() / Φ;
        return ((255.0 * (1.0 - 1.0 / (1.0 + x))).max(128.0)) as u8;
    }
    // Offset 32 keeps stale refreshes below new-source priorities (0..256).
    (32.0 + (proximity * 0.7 + urgency * 0.3) * 200.0) as u8
}
