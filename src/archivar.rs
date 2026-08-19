use crate::force::{force_id_of, kernel_id_for_force};
use crate::inflate::unzip;
pub use crate::json::{jnum, jpath, jpath_val, json_num, jstr, parse_json, scalar_of, JsonVal};
use crate::lsk::LeapSeconds;
use std::collections::HashMap;
use std::process::Command;

pub const Φ: f64 = 1.618033988749895;
pub const CHEBYSHEV_N: usize = 18;

#[derive(Clone)]
pub enum Motion {
    Surface {
        body_name: String,
        lat: f64,
        lon: f64,
        alt: f64,
    },
    Barycenter {
        body_name: String,
        scale: f64,
    },
    Linear {
        p: [f64; 3],
        v: [f64; 3],
    },
}

#[derive(Clone)]
pub enum OscillatorSource {
    Source(u32),
    Sensor,
    Body,
}

#[derive(Clone)]
pub struct Oscillator {
    pub source: OscillatorSource,
    pub epoch: f64,
    pub ttl: f64,
    pub extent: f64,
    pub tau: f64,
    pub kernel_id: f64,
    pub force_type: f64,
    pub absorption: f64,
    pub advection: f64,
    pub vmax: f64,
    pub amax: f64,
    pub p0f: [f64; 3],
    pub motion: Motion,
    pub val: f64,
    pub name: String,
    pub z: f64,
    pub freq: f64,
    pub bin_width: f64,
}

#[derive(Clone, Debug)]
pub enum Position {
    Source,
    Surface {
        body_name: String,
        lat: f64,
        lon: f64,
        alt: f64,
    },
    SurfaceFlow {
        body_name: String,
        lat: f64,
        lon: f64,
        alt: f64,
        speed: f64,
        track: f64,
        vrate: Option<f64>,
    },
    StateVector {
        p: [f64; 3],
        v: [f64; 3],
        track: bool,
    },
}

#[derive(Clone)]
pub struct DeclaredBody {
    pub body_name: String,
    pub lat: f64,
    pub lon: f64,
    pub alt: Option<f64>,
}

#[derive(Clone)]
pub struct Channel {
    pub name: String,
    pub value: f64,
    pub position: Position,
    pub epoch: f64,
    pub z: f64,
    pub freq: f64,
    pub bin_width: f64,
}

#[derive(Clone)]
pub enum Extract {
    Field(FieldConfig),
    First(FieldConfig),
    Last(FieldConfig),
    Count(FieldConfig),
    LastRow(FieldConfig),
    LastObj(String, String, String, String),
    LastLine(String),
    ObjLast(FieldConfig),
    GeojsonEvents {
        mag_key: String,
        min_mag: f64,
        outputs: Vec<String>,
        tau: f64,
        absorption: f64,
        advection: f64,
        mag_type_key: String,
    },
    Path(FieldConfig),
    Deep(FieldConfig),
    Regex(FieldConfig),

    Map {
        arr_path: String,
        lat_key: String,
        lon_key: String,
        alt_key: String,
        epoch_key: String,
        val_key: String,
        alt_scale: f64,
        vel_key: String,
        vel_scale: f64,
        trk_key: String,
        vr_key: String,
        fields: Vec<FieldConfig>,
        lat_sign: Option<String>,
        lon_sign: Option<String>,
        epoch_scale: f64,
        tau_key: String,
        mag_type_key: String,
    },
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
        fields: Vec<FieldConfig>,
        tau_key: String,
    },
    ProfileMap {
        arr_path: String,
        lat_key: String,
        lon_key: String,
        epoch_key: String,
        pressure_var: String,
        pressure_scale: f64,
        fields: Vec<FieldConfig>,
    },
    Rows {
        last_line: bool,
        fields: Vec<FieldConfig>,
        tau_key: String,
    },
    Flatten {
        arr_path: String,
        geom_path: String,
        epoch_key: String,
        fields: Vec<FieldConfig>,
    },
    CmrPolygon {
        arr_path: String,
        fields: Vec<FieldConfig>,
        epoch_key: String,
        alt_key: String,
        val_key: String,
    },
    CelestialPolygon {
        arr_path: String,
        radius: f64,
        fields: Vec<FieldConfig>,
        epoch_key: String,
        val_key: String,
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
        q_key: String,
        tp_key: String,
        fields: Vec<FieldConfig>,
    },
    TransitMap {
        arr_path: String,
        name_key: String,
        ra_key: String,
        dec_key: String,
        dist_key: String,
        dist_scale: f64,
        a_key: String,
        e_key: String,
        i_key: String,
        w_key: String,
        tranmid_key: String,
        period_key: String,
        rp_key: String,
        rs_key: String,
    },
    Hapi(Vec<(String, String)>),
    Alerce(String),
    XmlCount(String, String),
}

#[derive(Clone)]
pub struct FieldConfig {
    pub key: String,
    pub name: String,
    pub kernel: u8,
    pub force: u8,
    pub tau: f64,
    pub absorption: f64,
    pub advection: f64,
    pub unit: String,
    pub fold: Option<(u8, String)>,
}

pub fn convert_to_si(value: f64, unit: &str) -> Option<f64> {
    match unit.trim() {
        "MW" => return Some(value * 1.0e6),
        "Mw" => return Some(10.0f64.powf(1.5 * value + 9.1)),
        "M" => return None,
        _ => {}
    }
    match unit.trim().to_lowercase().as_str() {
        "" | "m" | "s" | "k" | "kg" | "pa" | "w" | "w/m2" | "w/m²" | "t" | "hz" | "v" | "a"
        | "rad" | "m/s" | "m/s2" | "m/s²" | "j" | "v/m" | "s/m" | "ntu" => Some(value),
        "km" | "km/s" => Some(value * 1e3),
        "cm" => Some(value * 1e-2),
        "mm" | "ms" => Some(value * 1e-3),
        "d" => Some(value * 86400.0),
        "hpa" | "mb" => Some(value * 100.0),
        "decibar" => Some(value * 1e4),
        "npa" => Some(value * 1e-9),
        "nt" => Some(value * 1e-9),
        "gal" => Some(value * 1e-2),
        "mgal" => Some(value * 1e-5),
        "km/h" | "kmh" => Some(value / 3.6),
        "knot" | "kt" => Some(value * 0.514444),
        "c" | "°c" => Some(value + 273.15),
        "ppm" => Some(value * 1e-6),
        "ppb" => Some(value * 1e-9),
        "pct" | "%" => Some(value * 1e-2),
        "psu" => Some(value * 1e-3),
        "jy" => Some(value * 1e-26),
        "mjy" => Some(value * 1e-29),
        "sfu" => Some(value * 1e-22),
        "au" => Some(value * 1.495978707e11),
        "pc" => Some(value * 3.085677581e16),
        "pc/cm3" => Some(value * 3.085677581e22),
        "ev" => Some(value * 1.602176634e-19),
        "ft" => Some(value * 0.3048),
        "deg" => Some(value * std::f64::consts::PI / 180.0),
        "arcsec" => Some(value * 4.84813681109536e-6),
        "m_sun" => Some(value * 1.98847e30),
        "m_earth" => Some(value * 5.9722e24),
        "r_earth" => Some(value * 6.371e6),
        "mg/m3" | "mg/m³" | "mg/kg" => Some(value * 1e-6),
        "ug/m3" | "ug/m³" | "µg/m3" | "µg/m³" => Some(value * 1e-9),
        "ua/m2" | "ua/m²" | "µa/m2" | "µa/m²" => Some(value * 1e-6),
        "mv/m" => Some(value * 1e-3),
        "us/cm" => Some(value * 1e-4),
        "uatm" => Some(value * 0.101325),
        "erg/cm2" => Some(value * 1e-3),
        "m3/s" | "m³/s" => Some(value),
        "cfs" => Some(value * 0.028316846592),
        "n/cc" | "cm-3" | "1/cm3" => Some(value * 1e6),
        "du" => Some(value * 2.6867e20),
        "jy_km/s" => Some(value * 1e-23),
        "crab" => Some(value * 2.4e-14),
        "logg" => Some(10.0f64.powf(value) * 0.01),
        "cpm" => Some(value * 1.0e-6 / (334.0 * 3600.0)),
        "e10j" => Some(value * 1.0e10),
        "kt_tnt" => Some(value * 4.184e12),
        _ => None,
    }
}

pub fn register_unconverted_unit(unit: &str, name: &str) {
    static REPORTED: std::sync::Mutex<Option<std::collections::HashSet<String>>> =
        std::sync::Mutex::new(None);
    let mut guard = match REPORTED.lock() {
        Ok(g) => g,
        Err(_) => return,
    };
    let set = guard.get_or_insert_with(std::collections::HashSet::new);
    if set.insert(unit.to_string()) {
        eprintln!(
            "unit \"{}\" unconverted — SI absent; oscillators like \"{}\" stay unmanifested (pending curation)",
            unit, name
        );
    }
}

pub fn fold_value(a: Option<f64>, b: Option<f64>, op: u8) -> Option<f64> {
    let (a, b) = (a?, b?);
    Some(match op {
        1 => (a + b) * 0.5,
        2 => a - b,
        _ => a + b,
    })
}

pub fn is_moment_magnitude(t: &str) -> bool {
    matches!(
        t.trim().to_ascii_lowercase().as_str(),
        "mw" | "mww" | "mwc" | "mwb" | "mwr" | "mwp" | "mwpd" | "mi"
    )
}

#[derive(Clone)]
pub struct Anomaly {
    pub category: &'static str,
    pub url: String,
    pub details: String,
}

static ANOMALIES: std::sync::Mutex<Vec<Anomaly>> = std::sync::Mutex::new(Vec::new());

thread_local! {
    pub static ANOMALY_COLLECT: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

pub fn report_anomaly(category: &'static str, url: &str, details: &str) {
    if !ANOMALY_COLLECT.with(|c| c.get()) {
        return;
    }
    if let Ok(mut v) = ANOMALIES.lock() {
        v.push(Anomaly {
            category,
            url: url.to_string(),
            details: details.to_string(),
        });
    }
}

pub fn take_anomalies() -> Vec<Anomaly> {
    match ANOMALIES.lock() {
        Ok(mut v) => std::mem::take(&mut *v),
        Err(_) => Vec::new(),
    }
}

pub fn anomaly_issue_body(anomalies: &[Anomaly]) -> String {
    let mut body = String::from("| Category | URL | Details |\n|---|---|---|\n");
    for a in anomalies {
        body.push_str(&format!("| {} | {} | {} |\n", a.category, a.url, a.details));
    }
    body
}

pub fn normalize_unit(unit: &str) -> String {
    unit.trim()
        .to_lowercase()
        .replace('\u{b2}', "2")
        .replace('\u{b3}', "3")
        .replace('\u{b5}', "u")
        .replace('\u{3bc}', "u")
}

pub fn allowed_units_for_force(force: u8) -> &'static [&'static str] {
    match force {
        0 => &[
            "w", "w/m2", "t", "nt", "ev", "jy", "mjy", "jy_km/s", "hz", "m", "km", "mag", "pc/cm3",
            "erg/cm2", "crab", "cpm", "e10j", "kt_tnt", "sfu", "1/cm3",
        ],
        1 => &[
            "m/s2", "gal", "mgal", "kg", "m_sun", "m_earth", "au", "pc", "t", "nt", "m", "r_earth",
            "logg",
        ],
        2 => &["pa", "hpa", "m", "mm", "hz"],
        3 => &["m", "mm", "km", "m/s2", "gal", "pa", "hz", "mw"],
        4 => &["m", "mm", "cm", "km", "pa", "m/s", "mw"],
        5 => &["k", "c", "w/m2", "w", "j", "mw"],
        6 => &[
            "ppm", "ppb", "mg/m3", "ug/m3", "mg/kg", "psu", "ntu", "%", "pct", "hpa", "uatm", "du",
            "cm-3",
        ],
        7 => &[
            "m/s", "km/h", "km/s", "knot", "kt", "m3/s", "cfs", "pa", "hpa", "mb", "m", "decibar",
            "npa",
        ],
        8 => &["v/m", "v", "a", "s/m", "ua/m2", "mv/m", "us/cm"],
        _ => &[],
    }
}

pub fn report_physics_mismatch(force: u8, unit: &str, key: &str, url: &str) {
    if !allowed_units_for_force(force).contains(&normalize_unit(unit).as_str()) {
        report_anomaly(
            "Physics Mismatch",
            url,
            &format!("field {}: unit \"{}\" not in force registry", key, unit),
        );
    }
}

pub struct BrowserSensor {
    pub key: String,
    pub force: u8,
    pub kernel: u8,
    pub ttl: f64,
}

#[derive(Clone)]
pub enum Frame {
    Surface {
        body_name: String,
        lat: f64,
        lon: f64,
        alt: f64,
    },
    Barycenter {
        body_name: String,
        scale: f64,
    },
    Manifest,
}

#[derive(Clone)]
pub struct SourceConfig {
    pub ttl: u64,
    pub url: String,
    pub frame: Frame,
    pub format: String,
    pub extracts: Vec<Extract>,
    pub headers: Vec<(String, String)>,
    pub post_body: Option<String>,
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
    pub catalog_epoch: Option<f64>,
    pub repeat_ra_bins: u32,
    pub fanout_cap: u32,
    pub stations_flatten: String,
    pub stations_filter: Option<(String, String)>,
    pub fanout_delay: u64,
}

pub const J2000_EPOCH: f64 = 2451545.0;
pub const PARSEC_M: f64 = 3.085677581e16;
pub const C_LIGHT: f64 = 299792458.0;
pub const HUBBLE_H0: f64 = 70000.0 / (PARSEC_M * 1.0e6);
pub const MAS_YR_TO_RAD_S: f64 = 4.84813681109536e-9 / 31557600.0;
const GAUSS_K: f64 = 0.01720209895;

pub type OscRecord = (
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
);

pub fn chebyshev_evaluate(coeffs: &[f64; CHEBYSHEV_N], tau: f64) -> f64 {
    let mut b0 = 0.0;
    let mut b1 = 0.0;
    for i in (0..CHEBYSHEV_N).rev() {
        let b2 = b1;
        b1 = b0;
        b0 = 2.0 * tau * b1 - b2 + coeffs[i];
    }
    b0 - tau * b1
}

pub fn chebyshev_eval_slice(coeffs: &[f64], tau: f64) -> f64 {
    let mut b0 = 0.0;
    let mut b1 = 0.0;
    for i in (0..coeffs.len()).rev() {
        let b2 = b1;
        b1 = b0;
        b0 = 2.0 * tau * b1 - b2 + coeffs[i];
    }
    b0 - tau * b1
}

pub fn nutation_deltas_at(props: &BodyProperties, jd: f64) -> Option<(f64, f64, f64)> {
    let records = props.nutation.as_ref()?;
    let rec = records
        .iter()
        .find(|r| (jd - r.mid_jd).abs() <= r.half_jd)?;
    let tau = ((jd - rec.mid_jd) / rec.half_jd).clamp(-1.0, 1.0);
    Some((
        chebyshev_eval_slice(&rec.ra, tau),
        chebyshev_eval_slice(&rec.dec, tau),
        chebyshev_eval_slice(&rec.pm, tau),
    ))
}

pub fn nutation_sum(terms: &[[f64; 3]], t: f64) -> f64 {
    terms
        .iter()
        .map(|&[amplitude, frequency, phase]| amplitude * (frequency * t + phase).sin())
        .sum()
}

pub fn orientation_angles_at(bp: &BodyProperties, jd: f64) -> (f64, f64, f64) {
    let tc = (jd - J2000_EPOCH) / 36525.0;
    let (d_ra, d_dec, d_pm) = match nutation_deltas_at(bp, jd) {
        Some(d) => d,
        None => {
            if bp.nutation.is_some() {
                eprintln!(
                    "nutation: no granule covers jd {:.3} — deltas carry zero for this interval",
                    jd
                );
            }
            (0.0, 0.0, 0.0)
        }
    };
    let nut_ra = match &bp.nut_ra {
        Some(terms) => nutation_sum(terms, tc),
        None => 0.0,
    };
    let nut_dec = match &bp.nut_dec {
        Some(terms) => nutation_sum(terms, tc),
        None => 0.0,
    };
    let ra = bp.α0_deg + bp.dα0_dt_deg_per_century * tc + nut_ra + d_ra;
    let dec = bp.δ0_deg + bp.dδ0_dt_deg_per_century * tc + nut_dec + d_dec;
    let pm = bp.w0_deg + bp.dw_dt_deg_per_day * (jd - J2000_EPOCH) + d_pm;
    (ra, dec, pm)
}

pub fn body_barycenter_position(
    name: &str,
    tdb: f64,
    eph: &HashMap<String, BodyEphemeris>,
) -> Option<[f64; 3]> {
    let e = eph.get(name)?;
    let jd = tdb / 86400.0 + J2000_EPOCH;
    for g in &e.granules {
        let tau = (jd - g.t0_jd) / g.dt_jd;
        if tau >= -1.0 && tau <= 1.0 {
            return Some([
                chebyshev_evaluate(&g.cx, tau),
                chebyshev_evaluate(&g.cy, tau),
                chebyshev_evaluate(&g.cz, tau),
            ]);
        }
    }
    None
}

pub fn body_fixed_to_icrs(
    name: &str,
    lat: f64,
    lon: f64,
    alt: f64,
    tdb: f64,
    eph: &HashMap<String, BodyEphemeris>,
) -> Option<[f64; 3]> {
    let e = eph.get(name)?;
    let bp = e.props.as_ref()?;
    let [bx, by, bz] = body_barycenter_position(name, tdb, eph)?;
    let lr = lat.to_radians();
    let nr = lon.to_radians();
    let f = match bp.radii_c {
        Some(rc) if bp.radius_m > 0.0 => (bp.radius_m - rc) / bp.radius_m,
        Some(_) | None => bp.flattening?,
    };
    let e2 = f * (2.0 - f);
    let sl = lr.sin();
    let n = bp.radius_m / (1.0 - e2 * sl * sl).sqrt();
    let rb_scale = match bp.radii_b {
        Some(v) if bp.radius_m > 0.0 => v / bp.radius_m,
        _ => 1.0,
    };
    let xb = (n + alt) * lr.cos() * nr.cos();
    let yb = (n + alt) * lr.cos() * nr.sin() * rb_scale;
    let zb = (n * (1.0 - e2) + alt) * sl;
    if !e.rotation_matrices.is_empty() {
        let jd = tdb / 86400.0 + J2000_EPOCH;
        let rot_m = e
            .rotation_matrices
            .iter()
            .filter(|(t, _)| t.is_finite())
            .min_by(|a, b| (jd - a.0).abs().total_cmp(&(jd - b.0).abs()))
            .map(|(_, m)| m)?;
        let xi = rot_m[0] * xb + rot_m[1] * yb + rot_m[2] * zb;
        let yi = rot_m[3] * xb + rot_m[4] * yb + rot_m[5] * zb;
        let zi = rot_m[6] * xb + rot_m[7] * yb + rot_m[8] * zb;
        return Some([xi + bx, yi + by, zi + bz]);
    }
    let jd = tdb / 86400.0 + J2000_EPOCH;
    let (ra, dec, pm) = orientation_angles_at(bp, jd);
    let a = ra.to_radians();
    let d = dec.to_radians();
    let w = (pm - ra).to_radians();
    let xt = xb * w.cos() + yb * w.sin();
    let yt = -xb * w.sin() + yb * w.cos();
    let zt = zb;
    let (sa, ca) = a.sin_cos();
    let (sd, cd) = d.sin_cos();
    let xi = xt * cd * ca - yt * sa - zt * sd * ca;
    let yi = xt * cd * sa + yt * ca - zt * sd * sa;
    let zi = xt * sd + zt * cd;
    Some([xi + bx, yi + by, zi + bz])
}

pub fn icrs_to_body_surface(
    x: f64,
    y: f64,
    z: f64,
    tdb_secs: f64,
    body_name: &str,
    eph: &HashMap<String, BodyEphemeris>,
) -> Option<(f64, f64)> {
    let e = eph.get(body_name)?;
    let [bx, by, bz] = body_barycenter_position(body_name, tdb_secs, eph)?;
    let rx = x - bx;
    let ry = y - by;
    let rz = z - bz;
    let (xb, yb, zb) = if !e.rotation_matrices.is_empty() {
        let jd = tdb_secs / 86400.0 + J2000_EPOCH;
        let rot_m = e
            .rotation_matrices
            .iter()
            .filter(|(t, _)| t.is_finite())
            .min_by(|a, b| (jd - a.0).abs().total_cmp(&(jd - b.0).abs()))
            .map(|(_, m)| m)?;
        let xt = rot_m[0] * rx + rot_m[3] * ry + rot_m[6] * rz;
        let yt = rot_m[1] * rx + rot_m[4] * ry + rot_m[7] * rz;
        let zt = rot_m[2] * rx + rot_m[5] * ry + rot_m[8] * rz;
        (xt, yt, zt)
    } else {
        let bp = e.props.as_ref()?;
        let jd = tdb_secs / 86400.0 + J2000_EPOCH;
        let (ra, dec, pm) = orientation_angles_at(bp, jd);
        let a = ra.to_radians();
        let d = dec.to_radians();
        let w = (pm - ra).to_radians();
        let (sw, cw) = w.sin_cos();
        let (sa, ca) = a.sin_cos();
        let (sd, cd) = d.sin_cos();
        let xt = cd * ca * rx + cd * sa * ry + sd * rz;
        let yt = -sa * rx + ca * ry;
        let zt = -sd * ca * rx - sd * sa * ry + cd * rz;
        let xb = xt * cw - yt * sw;
        let yb = xt * sw + yt * cw;
        (xb, yb, zt)
    };
    let bp = e.props.as_ref()?;
    let lon = yb.atan2(xb);
    let p = (xb * xb + yb * yb).sqrt();
    let f = bp.flattening?;
    let e2 = f * (2.0 - f);
    let r = bp.radius_m;
    let mut lat = zb.atan2(p * (1.0 - e2));
    for _ in 0..3 {
        let sl = lat.sin();
        let n = r / (1.0 - e2 * sl * sl).sqrt();
        let h = p / lat.cos() - n;
        lat = zb.atan2(p * (1.0 - e2 * n / (n + h)));
    }
    Some((lat.to_degrees(), lon.to_degrees()))
}

impl Motion {
    pub fn at(&self, t: f64, epoch: f64, eph: &HashMap<String, BodyEphemeris>) -> Option<[f64; 3]> {
        match self {
            Motion::Surface {
                body_name,
                lat,
                lon,
                alt,
            } => body_fixed_to_icrs(body_name, *lat, *lon, *alt, t, eph),
            Motion::Barycenter { body_name, scale } => body_barycenter_position(body_name, t, eph)
                .map(|[x, y, z]| [x * scale, y * scale, z * scale]),
            Motion::Linear { p, v } => {
                let dt = t - epoch;
                Some([p[0] + v[0] * dt, p[1] + v[1] * dt, p[2] + v[2] * dt])
            }
        }
    }

    pub fn anchor_body(&self) -> Option<&str> {
        match self {
            Motion::Surface { body_name, .. } | Motion::Barycenter { body_name, .. } => {
                Some(body_name)
            }
            Motion::Linear { .. } => None,
        }
    }
}

pub fn jlast(json: &JsonVal, key: &str) -> Option<f64> {
    if let Some((target_path, final_key)) = key.rsplit_once('.') {
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

pub fn jfirst(json: &JsonVal, key: &str) -> Option<f64> {
    if let Some((prefix, final_key)) = key.rsplit_once('.') {
        let parent = if prefix.is_empty() {
            json
        } else {
            jpath_val(json, prefix)?
        };
        if let JsonVal::Arr(arr) = parent {
            return arr.first().and_then(|v| match v {
                JsonVal::Obj(o) => o.get(final_key).and_then(scalar_of),
                other => scalar_of(other),
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

pub fn kernel_id_of(name: &str) -> Option<u8> {
    match name {
        "inverse-square" => Some(0),
        "gaussian-inverse-square" => Some(1),
        "gaussian-inverse" => Some(2),
        "erfc" => Some(3),
        "exponential-decay" => Some(4),
        "patch-levy" => Some(5),
        "inverse-linear" => Some(6),
        _ => None,
    }
}

pub fn extract_fields(ext: &Extract) -> &[FieldConfig] {
    match ext {
        Extract::Map { fields, .. }
        | Extract::CelestialMap { fields, .. }
        | Extract::Rows { fields, .. }
        | Extract::Flatten { fields, .. }
        | Extract::CmrPolygon { fields, .. }
        | Extract::CelestialPolygon { fields, .. }
        | Extract::KeplerMap { fields, .. } => fields,
        Extract::TransitMap { .. } => &[],
        Extract::Field(fc)
        | Extract::First(fc)
        | Extract::Last(fc)
        | Extract::Count(fc)
        | Extract::LastRow(fc)
        | Extract::ObjLast(fc)
        | Extract::Path(fc)
        | Extract::Deep(fc)
        | Extract::Regex(fc) => std::slice::from_ref(fc),
        _ => &[],
    }
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

pub fn fetch_raw(
    url: &str,
    body: Option<&str>,
    headers: &[(String, String)],
    ttl: u64,
) -> Option<String> {
    let connect_t = ((ttl as f64) / (Φ * Φ * Φ)).ceil() as u64;
    let max_t = ((ttl as f64) / (Φ * Φ)).ceil() as u64;
    let mut cmd = Command::new("curl");
    cmd.arg("-s")
        .arg("-S")
        .arg("-f")
        .arg("-L")
        .arg("--retry")
        .arg("3")
        .arg("--retry-all-errors")
        .arg("--retry-delay")
        .arg("2")
        .arg("-m")
        .arg(max_t.to_string())
        .arg("--connect-timeout")
        .arg(connect_t.to_string());
    if let Some(b) = body {
        cmd.arg("-X").arg("POST");
        cmd.arg("-d").arg(b);
    }
    for (k, v) in headers {
        cmd.arg("-H").arg(format!("{}: {}", k, v));
    }
    cmd.arg(url);
    let output = cmd.output().ok()?;
    if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!(
            "\r\x1b[Kfetch returned ({}): {} {}",
            output.status,
            url,
            stderr.trim()
        );
        None
    }
}

pub fn curl_base(ttl: u64, parallel_max: u8) -> Command {
    let connect_t = ((ttl as f64) / (Φ * Φ * Φ)).ceil() as u64;
    let max_t = ((ttl as f64) / (Φ * Φ)).ceil() as u64;
    let mut cmd = Command::new("curl");
    cmd.arg("-s")
        .arg("-S")
        .arg("-f")
        .arg("-L")
        .arg("--retry")
        .arg("5")
        .arg("--retry-all-errors")
        .arg("--retry-delay")
        .arg("2");
    if parallel_max > 0 {
        cmd.arg("--parallel")
            .arg("--parallel-max")
            .arg(parallel_max.to_string());
    }
    cmd.arg("-m")
        .arg(max_t.to_string())
        .arg("--connect-timeout")
        .arg(connect_t.to_string());
    cmd
}

pub fn fetch_raw_bytes(url: &str, ttl: u64) -> Option<Vec<u8>> {
    let mut cmd = curl_base(ttl, 0);
    cmd.arg(url);
    let output = cmd.output().ok()?;
    if output.status.success() {
        Some(output.stdout)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!(
            "\r\x1b[Kfetch_bytes returned ({}): {} {}",
            output.status,
            url,
            stderr.trim()
        );
        None
    }
}

pub fn fetch_raw_probe(
    url: &str,
    body: Option<&str>,
    headers: &[(String, String)],
) -> Option<String> {
    let mut cmd = Command::new("curl");
    cmd.arg("-s")
        .arg("-S")
        .arg("-f")
        .arg("-L")
        .arg("--retry")
        .arg("1")
        .arg("--retry-all-errors")
        .arg("--retry-delay")
        .arg("1")
        .arg("-m")
        .arg("10")
        .arg("--connect-timeout")
        .arg("5");
    if let Some(b) = body {
        cmd.arg("-X")
            .arg("POST")
            .arg("-H")
            .arg("Content-Type: application/json")
            .arg("-d")
            .arg(b);
    }
    for (k, v) in headers {
        cmd.arg("-H").arg(format!("{}: {}", k, v));
    }
    cmd.arg(url);
    let output = cmd.output().ok()?;
    if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).into_owned())
    } else {
        None
    }
}

pub fn fetch_raw_bytes_post(
    url: &str,
    body: Option<&str>,
    headers: &[(String, String)],
    ttl: u64,
) -> Option<Vec<u8>> {
    let connect_t = ((ttl as f64) / (Φ * Φ * Φ)).ceil() as u64;
    let max_t = ((ttl as f64) / (Φ * Φ)).ceil() as u64;
    let mut cmd = Command::new("curl");
    cmd.arg("-s")
        .arg("-S")
        .arg("-f")
        .arg("-L")
        .arg("--retry")
        .arg("5")
        .arg("--retry-all-errors")
        .arg("--retry-delay")
        .arg("2")
        .arg("-m")
        .arg(max_t.to_string())
        .arg("--connect-timeout")
        .arg(connect_t.to_string())
        .arg("-X")
        .arg("POST");
    if let Some(b) = body {
        cmd.arg("-H")
            .arg("Content-Type: application/json")
            .arg("-d")
            .arg(b);
    }
    for (k, v) in headers {
        cmd.arg("-H").arg(format!("{}: {}", k, v));
    }
    cmd.arg(url);
    let output = cmd.output().ok()?;
    if output.status.success() {
        Some(output.stdout)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!(
            "fetch_bytes_post returned ({}): {} {}",
            output.status,
            url,
            stderr.trim()
        );
        None
    }
}

pub fn load_sources() -> Vec<SourceConfig> {
    let content = match std::fs::read_to_string("phi/sources.φ") {
        Ok(c) => c,
        Err(_) => return Vec::new(),
    };
    parse_sources(&content)
}

pub fn parse_sources(content: &str) -> Vec<SourceConfig> {
    let mut sources = Vec::new();

    let mut cur_ttl: u64 = 0;
    let mut cur_url = String::new();
    let mut cur_format = String::new();
    let mut cur_extracts: Vec<Extract> = Vec::new();
    let mut cur_headers: Vec<(String, String)> = Vec::new();
    let mut cur_target: Option<String> = None;
    let mut cur_catalog: Option<String> = None;
    let mut cur_max_freq: Option<f64> = None;
    let mut cur_min_freq: Option<f64> = None;
    let mut cur_body: Option<String> = None;
    let mut cur_post_body: Option<String> = None;

    let mut cur_stations_url: Option<String> = None;
    let mut cur_stations_path = String::from("stations");
    let mut cur_stations_lat = String::from("lat");
    let mut cur_stations_lon = String::from("lng");
    let mut cur_stations_id = String::from("id");
    let mut cur_flux_from_mag: Option<String> = None;
    let mut cur_abs_mag_from: Option<String> = None;
    let mut cur_catalog_epoch: Option<f64> = None;
    let mut cur_repeat_ra_bins: u32 = 0;
    let mut cur_fanout_cap: u32 = 0;
    let mut cur_stations_flatten = String::new();
    let mut cur_stations_filter: Option<(String, String)> = None;
    let mut cur_fanout_delay: u64 = 0;
    let mut cur_frame: Option<Frame> = None;
    let mut active = false;

    macro_rules! flush {
        () => {
            if active && cur_ttl > 0 && !cur_url.is_empty() {
                if cur_format == "kernel_text" || cur_frame.is_some() {
                    if cur_flux_from_mag.is_some() && cur_abs_mag_from.is_some() {
                        eprintln!(
                            "source refused: flux_from_mag + abs_mag_from conflict at {}",
                            cur_url
                        );
                    } else {
                        sources.push(SourceConfig {
                            ttl: cur_ttl,
                            url: std::mem::take(&mut cur_url),
                            frame: match &cur_frame {
                                Some(f) => f.clone(),
                                None => Frame::Manifest,
                            },
                            format: std::mem::take(&mut cur_format),
                            extracts: std::mem::take(&mut cur_extracts),
                            headers: std::mem::take(&mut cur_headers),
                            post_body: cur_post_body.clone(),
                            target: cur_target.clone(),
                            catalog: cur_catalog.clone(),
                            max_freq: cur_max_freq,
                            min_freq: cur_min_freq,
                            body: cur_body.clone(),
                            stations_url: cur_stations_url.clone(),
                            stations_path: std::mem::take(&mut cur_stations_path),
                            stations_lat: std::mem::take(&mut cur_stations_lat),
                            stations_lon: std::mem::take(&mut cur_stations_lon),
                            stations_id: std::mem::take(&mut cur_stations_id),
                            flux_from_mag: cur_flux_from_mag.clone(),
                            abs_mag_from: cur_abs_mag_from.clone(),
                            catalog_epoch: cur_catalog_epoch,
                            repeat_ra_bins: cur_repeat_ra_bins,
                            fanout_cap: cur_fanout_cap,
                            stations_flatten: std::mem::take(&mut cur_stations_flatten),
                            stations_filter: cur_stations_filter.take(),
                            fanout_delay: cur_fanout_delay,
                        });
                    }
                }
            }
        };
    }

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "url" if parts.len() >= 2 => {
                flush!();
                cur_url = parts[1].to_string();
                cur_format.clear();
                cur_extracts.clear();
                cur_headers.clear();
                cur_ttl = 0;
                cur_target = None;
                cur_catalog = None;
                cur_max_freq = None;
                cur_min_freq = None;
                cur_body = None;
                cur_post_body = None;
                cur_stations_url = None;
                cur_stations_path = String::from("stations");
                cur_stations_lat = String::from("lat");
                cur_stations_lon = String::from("lng");
                cur_stations_id = String::from("id");
                cur_flux_from_mag = None;
                cur_abs_mag_from = None;
                cur_catalog_epoch = None;
                cur_repeat_ra_bins = 0;
                cur_fanout_cap = 0;
                cur_stations_flatten = String::new();
                cur_stations_filter = None;
                cur_fanout_delay = 0;
                cur_frame = None;
                active = true;
            }
            "ttl" if parts.len() >= 2 => {
                if let Ok(v) = parts[1].parse::<u64>() {
                    cur_ttl = v;
                } else {
                    report_anomaly(
                        "Invalid Syntax",
                        &cur_url,
                        &format!("ttl non-numeric: {}", line),
                    );
                }
            }
            "at" if parts.len() >= 2 => {
                let body = parts[1].to_string();
                cur_body = Some(body.clone());
                cur_frame = Some(Frame::Barycenter {
                    body_name: body,
                    scale: 1.0,
                });
            }
            "on" if parts.len() >= 4 => {
                let body = parts[1].to_string();
                cur_body = Some(body.clone());
                let lat: f64 = match parts[2].parse() {
                    Ok(v) => v,
                    Err(_) => {
                        report_anomaly(
                            "Invalid Syntax",
                            &cur_url,
                            &format!("on lat non-numeric: {}", line),
                        );
                        continue;
                    }
                };
                let lon: f64 = match parts[3].parse() {
                    Ok(v) => v,
                    Err(_) => {
                        report_anomaly(
                            "Invalid Syntax",
                            &cur_url,
                            &format!("on lon non-numeric: {}", line),
                        );
                        continue;
                    }
                };
                let alt: f64 = match parts.get(4) {
                    Some(s) => match s.parse() {
                        Ok(v) => v,
                        Err(_) => {
                            report_anomaly(
                                "Invalid Syntax",
                                &cur_url,
                                &format!("on alt non-numeric: {}", line),
                            );
                            continue;
                        }
                    },
                    None => {
                        report_anomaly(
                            "Invalid Syntax",
                            &cur_url,
                            &format!("on without alt refused — declare alt: {}", line),
                        );
                        continue;
                    }
                };
                cur_frame = Some(Frame::Surface {
                    body_name: body,
                    lat,
                    lon,
                    alt,
                });
            }
            "on" => {
                report_anomaly(
                    "Invalid Syntax",
                    &cur_url,
                    &format!("on needs <body> <lat> <lon> [alt]: {}", line),
                );
            }
            "map" if parts.len() >= 2 => {
                cur_extracts.push(Extract::Map {
                    arr_path: parts[1].to_string(),
                    lat_key: String::new(),
                    lon_key: String::new(),
                    alt_key: String::new(),
                    epoch_key: String::new(),
                    val_key: String::new(),
                    alt_scale: 1.0,
                    vel_key: String::new(),
                    vel_scale: 1.0,
                    trk_key: String::new(),
                    vr_key: String::new(),
                    fields: Vec::new(),
                    lat_sign: None,
                    lon_sign: None,
                    epoch_scale: 1.0,
                    tau_key: String::new(),
                    mag_type_key: String::new(),
                });
            }
            "cmap" if parts.len() >= 2 => {
                cur_extracts.push(Extract::CelestialMap {
                    arr_path: parts[1].to_string(),
                    ra_key: String::new(),
                    dec_key: String::new(),
                    dist_key: String::new(),
                    dist_scale: 1.0,
                    plx_key: String::new(),
                    z_key: String::new(),
                    pmra_key: String::new(),
                    pmdec_key: String::new(),
                    rv_key: String::new(),
                    rv_scale: 1.0,
                    epoch_key: String::new(),
                    fields: Vec::new(),
                    tau_key: String::new(),
                });
            }
            "profile" if parts.len() >= 2 => {
                cur_extracts.push(Extract::ProfileMap {
                    arr_path: parts[1].to_string(),
                    lat_key: String::new(),
                    lon_key: String::new(),
                    epoch_key: String::new(),
                    pressure_var: String::new(),
                    pressure_scale: 1.0,
                    fields: Vec::new(),
                });
            }
            "rows" => {
                cur_extracts.push(Extract::Rows {
                    last_line: false,
                    fields: Vec::new(),
                    tau_key: String::new(),
                });
            }
            "first" if parts.len() >= 9 => {
                let (k, f, tau, absorption, advection) = match parse_field_config(&parts) {
                    Some(v) => v,
                    None => continue,
                };
                let fc = FieldConfig {
                    key: parts[1].to_string(),
                    name: parts[2].to_string(),
                    kernel: k,
                    force: f,
                    tau,
                    absorption,
                    advection,
                    unit: String::new(),
                    fold: None,
                };
                cur_extracts.push(Extract::First(fc));
            }
            "last" if parts.len() >= 9 => {
                let (k, f, tau, absorption, advection) = match parse_field_config(&parts) {
                    Some(v) => v,
                    None => continue,
                };
                let fc = FieldConfig {
                    key: parts[1].to_string(),
                    name: parts[2].to_string(),
                    kernel: k,
                    force: f,
                    tau,
                    absorption,
                    advection,
                    unit: String::new(),
                    fold: None,
                };
                cur_extracts.push(Extract::Last(fc));
            }
            "count" if parts.len() >= 2 => {
                cur_extracts.push(Extract::Count(FieldConfig {
                    key: parts[1].to_string(),
                    name: if parts.len() >= 3 {
                        parts[2].to_string()
                    } else {
                        parts[1].to_string()
                    },
                    kernel: 0,
                    force: 0,
                    tau: 0.0,
                    absorption: 0.0,
                    advection: 0.0,
                    unit: String::new(),
                    fold: None,
                }));
            }
            "lastrow" if parts.len() >= 9 => {
                let (k, f, tau, absorption, advection) = match parse_field_config(&parts) {
                    Some(v) => v,
                    None => continue,
                };
                let fc = FieldConfig {
                    key: parts[1].to_string(),
                    name: parts[2].to_string(),
                    kernel: k,
                    force: f,
                    tau,
                    absorption,
                    advection,
                    unit: String::new(),
                    fold: None,
                };
                cur_extracts.push(Extract::LastRow(fc));
            }
            "lastobj" if parts.len() >= 5 => {
                cur_extracts.push(Extract::LastObj(
                    parts[1].to_string(),
                    parts[2].to_string(),
                    parts[3].to_string(),
                    parts[4].to_string(),
                ));
            }
            "lastline" if parts.len() >= 2 => {
                cur_extracts.push(Extract::LastLine(parts[1].to_string()));
            }
            "objlast" if parts.len() >= 9 => {
                let (k, f, tau, absorption, advection) = match parse_field_config(&parts) {
                    Some(v) => v,
                    None => continue,
                };
                let fc = FieldConfig {
                    key: parts[1].to_string(),
                    name: parts[2].to_string(),
                    kernel: k,
                    force: f,
                    tau,
                    absorption,
                    advection,
                    unit: String::new(),
                    fold: None,
                };
                cur_extracts.push(Extract::ObjLast(fc));
            }
            "geojson" if parts.len() >= 6 => {
                let mag_key = parts[1].to_string();
                let min_mag: f64 = match parts[2].parse() {
                    Ok(v) => v,
                    Err(_) => continue,
                };
                let mut outputs = Vec::new();
                for i in 3..parts.len().min(5) {
                    outputs.push(parts[i].to_string());
                }
                let tau: f64 = match parts.get(5).and_then(|s| s.parse().ok()) {
                    Some(v) if v > 0.0 => v,
                    _ => continue,
                };
                let absorption: f64 = match parts.get(6).and_then(|s| s.parse().ok()) {
                    Some(v) => v,
                    _ => continue,
                };
                let advection: f64 = match parts.get(7).and_then(|s| s.parse().ok()) {
                    Some(v) => v,
                    _ => continue,
                };
                cur_extracts.push(Extract::GeojsonEvents {
                    mag_key,
                    min_mag,
                    outputs,
                    tau,
                    absorption,
                    advection,
                    mag_type_key: String::new(),
                });
            }
            "path" if parts.len() >= 9 => {
                let (k, f, tau, absorption, advection) = match parse_field_config(&parts) {
                    Some(v) => v,
                    None => continue,
                };
                let fc = FieldConfig {
                    key: parts[1].to_string(),
                    name: parts[2].to_string(),
                    kernel: k,
                    force: f,
                    tau,
                    absorption,
                    advection,
                    unit: String::new(),
                    fold: None,
                };
                cur_extracts.push(Extract::Path(fc));
            }
            "deep" if parts.len() >= 9 => {
                let (k, f, tau, absorption, advection) = match parse_field_config(&parts) {
                    Some(v) => v,
                    None => continue,
                };
                let fc = FieldConfig {
                    key: parts[1].to_string(),
                    name: parts[2].to_string(),
                    kernel: k,
                    force: f,
                    tau,
                    absorption,
                    advection,
                    unit: String::new(),
                    fold: None,
                };
                cur_extracts.push(Extract::Deep(fc));
            }
            "regex" if parts.len() >= 9 => {
                let (k, f, tau, absorption, advection) = match parse_field_config(&parts) {
                    Some(v) => v,
                    None => continue,
                };
                let fc = FieldConfig {
                    key: parts[1].to_string(),
                    name: parts[2].to_string(),
                    kernel: k,
                    force: f,
                    tau,
                    absorption,
                    advection,
                    unit: String::new(),
                    fold: None,
                };
                cur_extracts.push(Extract::Regex(fc));
            }
            "flatten" if parts.len() >= 2 => {
                cur_extracts.push(Extract::Flatten {
                    arr_path: parts[1].to_string(),
                    geom_path: if parts.len() >= 3 {
                        parts[2].to_string()
                    } else {
                        String::new()
                    },
                    epoch_key: if parts.len() >= 4 {
                        parts[3].to_string()
                    } else {
                        String::new()
                    },
                    fields: Vec::new(),
                });
            }
            "cmrpolygon" if parts.len() >= 2 => {
                cur_extracts.push(Extract::CmrPolygon {
                    arr_path: parts[1].to_string(),
                    fields: Vec::new(),
                    epoch_key: if parts.len() >= 3 {
                        parts[2].to_string()
                    } else {
                        String::new()
                    },
                    alt_key: if parts.len() >= 4 {
                        parts[3].to_string()
                    } else {
                        String::new()
                    },
                    val_key: if parts.len() >= 5 {
                        parts[4].to_string()
                    } else {
                        String::new()
                    },
                });
            }
            "celestialpolygon" if parts.len() >= 2 => {
                let radius: f64 = match parts.get(2).and_then(|s| s.parse().ok()) {
                    Some(v) => v,
                    None => {
                        eprintln!(
                            "celestialpolygon radius parse returned void: {:?}",
                            parts.get(2)
                        );
                        continue;
                    }
                };
                cur_extracts.push(Extract::CelestialPolygon {
                    arr_path: parts[1].to_string(),
                    radius,
                    fields: Vec::new(),
                    epoch_key: if parts.len() >= 4 {
                        parts[3].to_string()
                    } else {
                        String::new()
                    },
                    val_key: if parts.len() >= 5 {
                        parts[4].to_string()
                    } else {
                        String::new()
                    },
                });
            }
            "keplermap" if parts.len() >= 2 => {
                cur_extracts.push(Extract::KeplerMap {
                    arr_path: parts[1].to_string(),
                    a_key: if parts.len() >= 3 {
                        parts[2].to_string()
                    } else {
                        String::new()
                    },
                    e_key: if parts.len() >= 4 {
                        parts[3].to_string()
                    } else {
                        String::new()
                    },
                    i_key: if parts.len() >= 5 {
                        parts[4].to_string()
                    } else {
                        String::new()
                    },
                    om_key: String::new(),
                    w_key: String::new(),
                    ma_key: String::new(),
                    epoch_key: String::new(),
                    q_key: String::new(),
                    tp_key: String::new(),
                    fields: Vec::new(),
                });
            }
            "transitmap" if parts.len() >= 2 => {
                cur_extracts.push(Extract::TransitMap {
                    arr_path: parts[1].to_string(),
                    name_key: String::new(),
                    ra_key: String::new(),
                    dec_key: String::new(),
                    dist_key: String::new(),
                    dist_scale: 3.085677581e16,
                    a_key: String::new(),
                    e_key: String::new(),
                    i_key: String::new(),
                    w_key: String::new(),
                    tranmid_key: String::new(),
                    period_key: String::new(),
                    rp_key: String::new(),
                    rs_key: String::new(),
                });
            }
            "hapi" if parts.len() >= 2 => {
                let mut params = Vec::new();
                for s in &parts[1..] {
                    if let Some((k, v)) = s.split_once('=') {
                        params.push((k.to_string(), v.to_string()));
                    }
                }
                cur_extracts.push(Extract::Hapi(params));
            }
            "alerce" if parts.len() >= 2 => {
                cur_extracts.push(Extract::Alerce(parts[1].to_string()));
            }
            "xmlcount" if parts.len() >= 3 => {
                cur_extracts.push(Extract::XmlCount(
                    parts[1].to_string(),
                    parts[2].to_string(),
                ));
            }
            "ephemeris" | "vectors" => {
                eprintln!(
                    "{} refused: Horizons-text extract is superseded by format ephemeris_binary + body channels (value would be a fabricated range)",
                    parts[0]
                );
            }
            "field" if parts.len() == 6 => {
                let f = match force_id_of(parts[2]) {
                    Some(f) => f,
                    None => {
                        report_anomaly(
                            "Invalid Syntax",
                            &cur_url,
                            &format!("unknown force \"{}\": {}", parts[2], line),
                        );
                        continue;
                    }
                };
                report_physics_mismatch(f, parts[3], parts[1], &cur_url);
                let tau: f64 = match parts[4].parse() {
                    Ok(v) if v > 0.0 => v,
                    _ => continue,
                };
                let k = match kernel_id_of(parts[5]) {
                    Some(k) => k,
                    None => match kernel_id_for_force(f) {
                        Some(k) => k,
                        None => continue,
                    },
                };
                let fc = FieldConfig {
                    key: parts[1].to_string(),
                    name: parts[1].to_string(),
                    kernel: k,
                    force: f,
                    tau,
                    absorption: 0.0,
                    advection: 0.0,
                    unit: parts[3].to_string(),
                    fold: None,
                };
                if let Some(ext) = cur_extracts.last_mut() {
                    let fields: Option<&mut Vec<FieldConfig>> = match ext {
                        Extract::Map { fields, .. } => Some(fields),
                        Extract::CelestialMap { fields, .. } => Some(fields),
                        Extract::Rows { fields, .. } => Some(fields),
                        Extract::Flatten { fields, .. } => Some(fields),
                        Extract::CmrPolygon { fields, .. } => Some(fields),
                        Extract::CelestialPolygon { fields, .. } => Some(fields),
                        Extract::KeplerMap { fields, .. } => Some(fields),
                        Extract::ProfileMap { .. } => {
                            eprintln!(
                                "field refused at {}: 5/6-token field inside a profile block is an orphan — the 9-token form carries the pressure arm",
                                parts[1]
                            );
                            continue;
                        }
                        _ => None,
                    };
                    if let Some(flds) = fields {
                        flds.push(fc);
                    } else {
                        cur_extracts.push(Extract::Field(fc.clone()));
                    }
                } else {
                    cur_extracts.push(Extract::Field(fc.clone()));
                }
            }
            "field" if parts.len() == 5 => {
                let f = match force_id_of(parts[2]) {
                    Some(f) => f,
                    None => {
                        report_anomaly(
                            "Invalid Syntax",
                            &cur_url,
                            &format!("unknown force \"{}\": {}", parts[2], line),
                        );
                        continue;
                    }
                };
                report_physics_mismatch(f, parts[3], parts[1], &cur_url);
                let tau: f64 = match parts[4].parse() {
                    Ok(v) if v > 0.0 => v,
                    _ => continue,
                };
                let fc = FieldConfig {
                    key: parts[1].to_string(),
                    name: parts[1].to_string(),
                    kernel: match kernel_id_for_force(f) {
                        Some(k) => k,
                        None => continue,
                    },
                    force: f,
                    tau,
                    absorption: 0.0,
                    advection: 0.0,
                    unit: parts[3].to_string(),
                    fold: None,
                };
                if let Some(ext) = cur_extracts.last_mut() {
                    let fields: Option<&mut Vec<FieldConfig>> = match ext {
                        Extract::Map { fields, .. } => Some(fields),
                        Extract::CelestialMap { fields, .. } => Some(fields),
                        Extract::Rows { fields, .. } => Some(fields),
                        Extract::Flatten { fields, .. } => Some(fields),
                        Extract::CmrPolygon { fields, .. } => Some(fields),
                        Extract::CelestialPolygon { fields, .. } => Some(fields),
                        Extract::KeplerMap { fields, .. } => Some(fields),
                        Extract::ProfileMap { .. } => {
                            eprintln!(
                                "field refused at {}: 5/6-token field inside a profile block is an orphan — the 9-token form carries the pressure arm",
                                parts[1]
                            );
                            continue;
                        }
                        _ => None,
                    };
                    if let Some(flds) = fields {
                        flds.push(fc);
                    } else {
                        cur_extracts.push(Extract::Field(fc.clone()));
                    }
                } else {
                    cur_extracts.push(Extract::Field(fc.clone()));
                }
            }
            "field" if parts.len() == 3 => {
                eprintln!(
                    "field refused at {}: 3-token field carries no tau (τ-Gate)",
                    parts[1]
                );
            }
            "field_in" if parts.len() >= 3 => {
                eprintln!(
                    "field_in refused at {}: legacy directive — the --gold port migrates field_in to field (τ-Gate)",
                    parts[1]
                );
            }
            "field" if parts.len() >= 9 => {
                let k = match kernel_id_of(parts[3]) {
                    Some(k) => k,
                    None => continue,
                };
                let f = match force_id_of(parts[4]) {
                    Some(f) => f,
                    None => {
                        report_anomaly(
                            "Invalid Syntax",
                            &cur_url,
                            &format!("unknown force \"{}\": {}", parts[4], line),
                        );
                        continue;
                    }
                };
                report_physics_mismatch(f, parts[5], parts[1], &cur_url);
                let tau: f64 = match parts[6].parse() {
                    Ok(v) if v > 0.0 => v,
                    _ => {
                        eprintln!(
                            "field refused at {}: tau absent or not positive (τ-Gate)",
                            parts[1]
                        );
                        continue;
                    }
                };
                let absorption: f64 = match parts[7].parse() {
                    Ok(v) => v,
                    Err(_) => continue,
                };
                let advection: f64 = match parts[8].parse() {
                    Ok(v) => v,
                    Err(_) => continue,
                };
                let fc = FieldConfig {
                    key: parts[1].to_string(),
                    name: parts[2].to_string(),
                    kernel: k,
                    force: f,

                    tau,
                    absorption,
                    advection,
                    unit: parts[5].to_string(),
                    fold: None,
                };
                if let Some(Extract::Map { fields, .. }) = cur_extracts.last_mut() {
                    fields.push(fc);
                } else if let Some(Extract::CelestialMap { fields, .. }) = cur_extracts.last_mut() {
                    fields.push(fc);
                } else if let Some(Extract::Rows { fields, .. }) = cur_extracts.last_mut() {
                    fields.push(fc);
                } else if let Some(Extract::Flatten { fields, .. }) = cur_extracts.last_mut() {
                    fields.push(fc);
                } else if let Some(Extract::CmrPolygon { fields, .. }) = cur_extracts.last_mut() {
                    fields.push(fc);
                } else if let Some(Extract::CelestialPolygon { fields, .. }) =
                    cur_extracts.last_mut()
                {
                    fields.push(fc);
                } else if let Some(Extract::KeplerMap { fields, .. }) = cur_extracts.last_mut() {
                    fields.push(fc);
                } else if let Some(Extract::ProfileMap { fields, .. }) = cur_extracts.last_mut() {
                    fields.push(fc);
                } else {
                    cur_extracts.push(Extract::Field(fc.clone()));
                }
            }
            "field" => {
                report_anomaly(
                    "Invalid Syntax",
                    &cur_url,
                    &format!("field arity {}: {}", parts.len(), line),
                );
            }
            "lat" if parts.len() >= 2 => match cur_extracts.last_mut() {
                Some(Extract::Map { lat_key, .. }) | Some(Extract::ProfileMap { lat_key, .. }) => {
                    *lat_key = parts[1].to_string();
                }
                _ => {}
            },
            "lon" if parts.len() >= 2 => match cur_extracts.last_mut() {
                Some(Extract::Map { lon_key, .. }) | Some(Extract::ProfileMap { lon_key, .. }) => {
                    *lon_key = parts[1].to_string();
                }
                _ => {}
            },
            "lat_sign" if parts.len() >= 2 => {
                if let Some(Extract::Map { lat_sign, .. }) = cur_extracts.last_mut() {
                    *lat_sign = Some(parts[1].to_string());
                }
            }
            "lon_sign" if parts.len() >= 2 => {
                if let Some(Extract::Map { lon_sign, .. }) = cur_extracts.last_mut() {
                    *lon_sign = Some(parts[1].to_string());
                }
            }
            "epoch_scale" if parts.len() >= 2 => {
                if let Ok(s) = parts[1].parse::<f64>() {
                    if let Some(Extract::Map { epoch_scale, .. }) = cur_extracts.last_mut() {
                        *epoch_scale = s;
                    }
                }
            }
            "alt" if parts.len() >= 2 => {
                let scale = match parts.get(2) {
                    None => 1.0,
                    Some(&"m") => 1.0,
                    Some(&"km") => 1000.0,
                    Some(&"ft") => 0.3048,
                    Some(&"cm") => 0.01,
                    Some(&"mm") => 0.001,
                    Some(&"-m") => -1.0,
                    Some(&"-km") => -1000.0,
                    Some(&"decibar") => 1.0,
                    Some(_) => continue,
                };
                if let Some(Extract::Map {
                    alt_key, alt_scale, ..
                }) = cur_extracts.last_mut()
                {
                    *alt_key = parts[1].to_string();
                    *alt_scale = scale;
                } else if let Some(Extract::ProfileMap {
                    pressure_var,
                    pressure_scale,
                    ..
                }) = cur_extracts.last_mut()
                {
                    *pressure_var = parts[1].to_string();
                    *pressure_scale = scale;
                }
            }
            "epoch" if parts.len() >= 2 => match cur_extracts.last_mut() {
                Some(Extract::Map { epoch_key, .. })
                | Some(Extract::KeplerMap { epoch_key, .. })
                | Some(Extract::ProfileMap { epoch_key, .. }) => {
                    *epoch_key = parts[1].to_string();
                }
                _ => {}
            },
            "pressure" if parts.len() >= 2 => {
                if let Some(Extract::ProfileMap {
                    pressure_var,
                    pressure_scale,
                    ..
                }) = cur_extracts.last_mut()
                {
                    *pressure_var = parts[1].to_string();
                    if parts.len() >= 3 {
                        if let Ok(s) = parts[2].parse::<f64>() {
                            *pressure_scale = s;
                        }
                    }
                }
            }
            "a" if parts.len() >= 2 => {
                if let Some(Extract::KeplerMap { a_key, .. }) = cur_extracts.last_mut() {
                    *a_key = parts[1].to_string();
                }
            }
            "e" if parts.len() >= 2 => {
                if let Some(Extract::KeplerMap { e_key, .. }) = cur_extracts.last_mut() {
                    *e_key = parts[1].to_string();
                }
            }
            "i" if parts.len() >= 2 => {
                if let Some(Extract::KeplerMap { i_key, .. }) = cur_extracts.last_mut() {
                    *i_key = parts[1].to_string();
                }
            }
            "om" if parts.len() >= 2 => {
                if let Some(Extract::KeplerMap { om_key, .. }) = cur_extracts.last_mut() {
                    *om_key = parts[1].to_string();
                }
            }
            "w" if parts.len() >= 2 => {
                if let Some(Extract::KeplerMap { w_key, .. }) = cur_extracts.last_mut() {
                    *w_key = parts[1].to_string();
                }
            }
            "ma" if parts.len() >= 2 => {
                if let Some(Extract::KeplerMap { ma_key, .. }) = cur_extracts.last_mut() {
                    *ma_key = parts[1].to_string();
                }
            }
            "qr" if parts.len() >= 2 => {
                if let Some(Extract::KeplerMap { q_key, .. }) = cur_extracts.last_mut() {
                    *q_key = parts[1].to_string();
                }
            }
            "tp" if parts.len() >= 2 => {
                if let Some(Extract::KeplerMap { tp_key, .. }) = cur_extracts.last_mut() {
                    *tp_key = parts[1].to_string();
                }
            }
            "vel" if parts.len() >= 2 => {
                if let Some(Extract::Map {
                    vel_key, vel_scale, ..
                }) = cur_extracts.last_mut()
                {
                    if parts.len() >= 3 {
                        match convert_to_si(1.0, parts[2]) {
                            Some(scale) if scale > 0.0 => {
                                *vel_scale = scale;
                                *vel_key = parts[1].to_string();
                            }
                            _ => {
                                eprintln!(
                                    "vel refused: unit \"{}\" unconverted — SI absent (pending curation)",
                                    parts[2]
                                );
                            }
                        }
                    } else {
                        *vel_key = parts[1].to_string();
                    }
                }
            }
            "tau_key" if parts.len() >= 2 => {
                let target = match cur_extracts.last_mut() {
                    Some(Extract::Map { tau_key, .. })
                    | Some(Extract::CelestialMap { tau_key, .. })
                    | Some(Extract::Rows { tau_key, .. }) => Some(tau_key),
                    _ => None,
                };
                if let Some(tk) = target {
                    *tk = parts[1].to_string();
                }
            }
            "mag_type_key" if parts.len() >= 2 => {
                let target = match cur_extracts.last_mut() {
                    Some(Extract::Map { mag_type_key, .. })
                    | Some(Extract::GeojsonEvents { mag_type_key, .. }) => Some(mag_type_key),
                    _ => None,
                };
                if let Some(mt) = target {
                    *mt = parts[1].to_string();
                }
            }
            "fold" if parts.len() == 7 => {
                let op = match parts[1] {
                    "mean" => 1u8,
                    "diff" => 2,
                    "sum" => 3,
                    other => {
                        eprintln!("fold refused: op \"{}\" unknown (mean|diff|sum)", other);
                        continue;
                    }
                };
                let f = match force_id_of(parts[4]) {
                    Some(f) => f,
                    None => continue,
                };
                let tau: f64 = match parts[6].parse() {
                    Ok(v) if v > 0.0 => v,
                    _ => continue,
                };
                let k = match kernel_id_for_force(f) {
                    Some(k) => k,
                    None => continue,
                };
                let fc = FieldConfig {
                    key: parts[2].to_string(),
                    name: format!("fold_{}_{}_{}", parts[1], parts[2], parts[3]),
                    kernel: k,
                    force: f,
                    tau,
                    absorption: 0.0,
                    advection: 0.0,
                    unit: parts[5].to_string(),
                    fold: Some((op, parts[3].to_string())),
                };
                let holder = match cur_extracts.last_mut() {
                    Some(Extract::Map { fields, .. })
                    | Some(Extract::CelestialMap { fields, .. })
                    | Some(Extract::Flatten { fields, .. })
                    | Some(Extract::Rows { fields, .. }) => Some(fields),
                    _ => None,
                };
                match holder {
                    Some(flds) => flds.push(fc),
                    None => {
                        eprintln!(
                            "fold refused: no map/cmap/flatten/rows holder at {} {}",
                            parts[2], parts[3]
                        );
                    }
                }
            }
            "trk" if parts.len() >= 2 => {
                if let Some(Extract::Map { trk_key, .. }) = cur_extracts.last_mut() {
                    *trk_key = parts[1].to_string();
                }
            }
            "vr" if parts.len() >= 2 => {
                if let Some(Extract::Map { vr_key, .. }) = cur_extracts.last_mut() {
                    *vr_key = parts[1].to_string();
                }
            }
            "val" if parts.len() >= 2 => {
                if let Some(Extract::Map { val_key, .. }) = cur_extracts.last_mut() {
                    *val_key = parts[1].to_string();
                }
            }
            "ra" if parts.len() >= 2 => {
                if let Some(Extract::CelestialMap { ra_key, .. }) = cur_extracts.last_mut() {
                    *ra_key = parts[1].to_string();
                }
            }
            "dec" if parts.len() >= 2 => {
                if let Some(Extract::CelestialMap { dec_key, .. }) = cur_extracts.last_mut() {
                    *dec_key = parts[1].to_string();
                }
            }
            "z" if parts.len() >= 2 => {
                if let Some(Extract::CelestialMap { z_key, .. }) = cur_extracts.last_mut() {
                    *z_key = parts[1].to_string();
                }
            }
            "plx" if parts.len() >= 2 => {
                if let Some(Extract::CelestialMap { plx_key, .. }) = cur_extracts.last_mut() {
                    *plx_key = parts[1].to_string();
                }
            }
            "pmra" if parts.len() >= 2 => {
                if let Some(Extract::CelestialMap { pmra_key, .. }) = cur_extracts.last_mut() {
                    *pmra_key = parts[1].to_string();
                }
            }
            "pmdec" if parts.len() >= 2 => {
                if let Some(Extract::CelestialMap { pmdec_key, .. }) = cur_extracts.last_mut() {
                    *pmdec_key = parts[1].to_string();
                }
            }
            "radvel" if parts.len() >= 2 => {
                if let Some(Extract::CelestialMap { rv_key, .. }) = cur_extracts.last_mut() {
                    *rv_key = parts[1].to_string();
                }
            }
            "dist" if parts.len() >= 2 => {
                if let Some(Extract::CelestialMap { dist_key, .. }) = cur_extracts.last_mut() {
                    *dist_key = parts[1].to_string();
                }
            }
            "dist_scale" if parts.len() >= 2 => {
                if let Ok(v) = parts[1].parse::<f64>() {
                    if let Some(Extract::CelestialMap { dist_scale, .. }) = cur_extracts.last_mut()
                    {
                        *dist_scale = v;
                    }
                }
            }
            "tname" if parts.len() >= 2 => {
                if let Some(Extract::TransitMap { name_key, .. }) = cur_extracts.last_mut() {
                    *name_key = parts[1].to_string();
                }
            }
            "tra" if parts.len() >= 2 => {
                if let Some(Extract::TransitMap { ra_key, .. }) = cur_extracts.last_mut() {
                    *ra_key = parts[1].to_string();
                }
            }
            "tdec" if parts.len() >= 2 => {
                if let Some(Extract::TransitMap { dec_key, .. }) = cur_extracts.last_mut() {
                    *dec_key = parts[1].to_string();
                }
            }
            "tdist" if parts.len() >= 2 => {
                if let Some(Extract::TransitMap { dist_key, .. }) = cur_extracts.last_mut() {
                    *dist_key = parts[1].to_string();
                }
            }
            "tdist_scale" if parts.len() >= 2 => {
                if let Ok(v) = parts[1].parse::<f64>() {
                    if let Some(Extract::TransitMap { dist_scale, .. }) = cur_extracts.last_mut() {
                        *dist_scale = v;
                    }
                }
            }
            "ta" if parts.len() >= 2 => {
                if let Some(Extract::TransitMap { a_key, .. }) = cur_extracts.last_mut() {
                    *a_key = parts[1].to_string();
                }
            }
            "te" if parts.len() >= 2 => {
                if let Some(Extract::TransitMap { e_key, .. }) = cur_extracts.last_mut() {
                    *e_key = parts[1].to_string();
                }
            }
            "ti" if parts.len() >= 2 => {
                if let Some(Extract::TransitMap { i_key, .. }) = cur_extracts.last_mut() {
                    *i_key = parts[1].to_string();
                }
            }
            "tw" if parts.len() >= 2 => {
                if let Some(Extract::TransitMap { w_key, .. }) = cur_extracts.last_mut() {
                    *w_key = parts[1].to_string();
                }
            }
            "ttranmid" if parts.len() >= 2 => {
                if let Some(Extract::TransitMap { tranmid_key, .. }) = cur_extracts.last_mut() {
                    *tranmid_key = parts[1].to_string();
                }
            }
            "tperiod" if parts.len() >= 2 => {
                if let Some(Extract::TransitMap { period_key, .. }) = cur_extracts.last_mut() {
                    *period_key = parts[1].to_string();
                }
            }
            "trp" if parts.len() >= 2 => {
                if let Some(Extract::TransitMap { rp_key, .. }) = cur_extracts.last_mut() {
                    *rp_key = parts[1].to_string();
                }
            }
            "trs" if parts.len() >= 2 => {
                if let Some(Extract::TransitMap { rs_key, .. }) = cur_extracts.last_mut() {
                    *rs_key = parts[1].to_string();
                }
            }
            "format" if parts.len() >= 2 => cur_format = parts[1..].join(" "),
            "body" if parts.len() >= 2 => {
                cur_body = Some(parts[1].to_string());
            }
            "force" if parts.len() >= 2 => {
                eprintln!(
                "force directive refused at {}: force is a field token, not a standalone directive",
                parts[1]
            );
            }
            "header" if parts.len() >= 3 => {
                cur_headers.push((parts[1].to_string(), parts[2].to_string()));
            }
            "post_body" if parts.len() >= 2 => cur_post_body = Some(parts[1].to_string()),
            "target" if parts.len() >= 2 => cur_target = Some(parts[1].to_string()),
            "catalog" if parts.len() >= 2 => cur_catalog = Some(parts[1].to_string()),
            "max_freq" if parts.len() >= 2 => {
                if let Ok(v) = parts[1].parse::<f64>() {
                    cur_max_freq = Some(v);
                }
            }
            "min_freq" if parts.len() >= 2 => {
                if let Ok(v) = parts[1].parse::<f64>() {
                    cur_min_freq = Some(v);
                }
            }
            "stations" if parts.len() >= 2 => cur_stations_url = Some(parts[1].to_string()),
            "stations_path" if parts.len() >= 2 => cur_stations_path = parts[1].to_string(),
            "stations_lat" if parts.len() >= 2 => cur_stations_lat = parts[1].to_string(),
            "stations_lon" if parts.len() >= 2 => cur_stations_lon = parts[1].to_string(),
            "stations_id" if parts.len() >= 2 => cur_stations_id = parts[1].to_string(),
            "stations_flatten" if parts.len() >= 2 => cur_stations_flatten = parts[1].to_string(),
            "stations_filter" if parts.len() >= 3 => {
                cur_stations_filter = Some((parts[1].to_string(), parts[2].to_string()));
            }
            "fanout_delay" if parts.len() >= 2 => {
                if let Ok(v) = parts[1].parse::<u64>() {
                    cur_fanout_delay = v;
                }
            }
            "fanout" if parts.len() >= 2 => {
                if let Ok(v) = parts[1].parse::<u32>() {
                    cur_fanout_cap = v;
                }
            }
            "flux_from_mag" if parts.len() >= 2 => cur_flux_from_mag = Some(parts[1].to_string()),
            "abs_mag_from" if parts.len() >= 2 => cur_abs_mag_from = Some(parts[1].to_string()),
            "catalog_epoch" if parts.len() >= 2 => {
                if let Ok(v) = parts[1].parse::<f64>() {
                    cur_catalog_epoch = Some(v);
                }
            }
            "repeat" if parts.len() >= 2 => {
                if parts[1] == "ra" && parts.len() >= 5 {
                    if let Ok(v) = parts[4].parse::<u32>() {
                        cur_repeat_ra_bins = v;
                    }
                } else if let Ok(v) = parts[1].parse::<u32>() {
                    cur_repeat_ra_bins = v;
                }
            }
            _ => {}
        }
    }
    flush!();
    sources
}

#[cfg(feature = "browser_relay")]
pub fn parse_path(s: &str) -> String {
    let Some(fl) = s.lines().next() else {
        return "/".to_string();
    };
    let p: Vec<&str> = fl.split_whitespace().collect();
    if p.len() >= 2 {
        p[1].to_string()
    } else {
        "/".to_string()
    }
}

#[derive(Clone)]
pub struct BodyProperties {
    pub α0_deg: f64,
    pub dα0_dt_deg_per_century: f64,
    pub δ0_deg: f64,
    pub dδ0_dt_deg_per_century: f64,
    pub w0_deg: f64,
    pub dw_dt_deg_per_day: f64,
    pub radius_m: f64,
    pub flattening: Option<f64>,
    pub gaussian_inverse_square: f64,
    pub gaussian_inverse: f64,
    pub erfc: f64,
    pub exponential_decay: f64,
    pub patch_levy: f64,
    pub gm: Option<f64>,
    pub j2: Option<f64>,
    pub j4: Option<f64>,
    pub radii_b: Option<f64>,
    pub radii_c: Option<f64>,
    pub nut_ra: Option<Vec<[f64; 3]>>,
    pub nut_dec: Option<Vec<[f64; 3]>>,
    pub nutation: Option<Vec<NutationRecord>>,
}

#[derive(Clone)]
pub struct NutationRecord {
    pub mid_jd: f64,
    pub half_jd: f64,
    pub ra: Vec<f64>,
    pub dec: Vec<f64>,
    pub pm: Vec<f64>,
}

#[derive(Clone)]
pub struct ChebyshevGranule {
    pub t0_jd: f64,
    pub dt_jd: f64,
    pub cx: [f64; CHEBYSHEV_N],
    pub cy: [f64; CHEBYSHEV_N],
    pub cz: [f64; CHEBYSHEV_N],
}

#[derive(Clone)]
pub struct BodyEphemeris {
    pub granules: Vec<ChebyshevGranule>,
    pub rotation_matrices: Vec<(f64, [f64; 9])>,
    pub props: Option<BodyProperties>,
}

pub fn parse_ephemeris_binary(data: &[u8]) -> Option<BodyEphemeris> {
    if data.len() < 24 || data[0] != 0xCF || data[1] != 0x86 || (data[2] != 0x01 && data[2] != 0x02)
    {
        return None;
    }
    let version = data[2];
    let section_count = u32::from_le_bytes(data[4..8].try_into().ok()?) as usize;
    let mut pos = 8usize;
    let mut granules = Vec::new();
    let mut rotation_matrices = Vec::new();
    let mut props = None;
    let mut has_stype2 = false;
    for _ in 0..section_count {
        if pos + 24 > data.len() {
            break;
        }
        let stype = u32::from_le_bytes(data[pos..pos + 4].try_into().ok()?);
        pos += 4;
        let gcount = u32::from_le_bytes(data[pos..pos + 4].try_into().ok()?) as usize;
        pos += 4;
        let degree = u32::from_le_bytes(data[pos..pos + 4].try_into().ok()?) as usize;
        pos += 4;
        pos += 4;
        if stype == 1 {
            let f = |i: usize| -> f64 {
                let mut buf = [0u8; 8];
                buf.copy_from_slice(&data[pos + i * 8..pos + i * 8 + 8]);
                f64::from_le_bytes(buf)
            };
            if gcount != 12 {
                return None;
            }
            let mask: u16 = if version == 0x02 {
                if pos + 96 + 2 > data.len() {
                    return None;
                }
                u16::from_le_bytes(data[pos + 96..pos + 98].try_into().ok()?)
            } else {
                0xFFFF
            };
            let slot = |v: f64, bit: usize| -> Option<f64> {
                if version == 0x02 {
                    if mask & (1u16 << (bit as u16)) != 0 {
                        Some(v)
                    } else {
                        None
                    }
                } else if v != 0.0 {
                    Some(v)
                } else {
                    None
                }
            };
            let radius_m = f(6);
            let radii_b = slot(f(7), 7);
            let radii_c = slot(f(8), 8);
            props = Some(BodyProperties {
                α0_deg: f(0),
                dα0_dt_deg_per_century: f(1),
                δ0_deg: f(2),
                dδ0_dt_deg_per_century: f(3),
                w0_deg: f(4),
                dw_dt_deg_per_day: f(5),
                radius_m,
                flattening: match radii_c {
                    Some(c) if radius_m > 0.0 => Some((radius_m - c) / radius_m),
                    _ => None,
                },
                gaussian_inverse_square: 0.0,
                gaussian_inverse: 0.0,
                erfc: 0.0,
                exponential_decay: 0.0,
                patch_levy: 0.0,
                gm: slot(f(11), 11),
                j2: slot(f(9), 9),
                j4: slot(f(10), 10),
                radii_b,
                radii_c,
                nut_ra: None,
                nut_dec: None,
                nutation: None,
            });
            pos += if version == 0x02 { 104 } else { 96 };
            continue;
        }
        if stype == 2 {
            has_stype2 = true;
            let f = |i: usize| -> f64 {
                let mut buf = [0u8; 8];
                buf.copy_from_slice(&data[pos + i * 8..pos + i * 8 + 8]);
                f64::from_le_bytes(buf)
            };
            let gis = f(0);
            let giv = f(1);
            let ert = f(2);
            let ed = f(3);
            let pl = f(4);
            if let Some(ref mut p) = props {
                p.gaussian_inverse_square = gis;
                p.gaussian_inverse = giv;
                p.erfc = ert;
                p.exponential_decay = ed;
                p.patch_levy = pl;
            }
            pos += 5 * 8;
            continue;
        }
        if stype == 3 {
            for _ in 0..gcount {
                if pos + 80 > data.len() {
                    break;
                }
                let f = |i: usize| -> f64 {
                    let mut buf = [0u8; 8];
                    buf.copy_from_slice(&data[pos + i * 8..pos + i * 8 + 8]);
                    f64::from_le_bytes(buf)
                };
                let t0_jd = f(0);
                let m: [f64; 9] = [f(1), f(2), f(3), f(4), f(5), f(6), f(7), f(8), f(9)];
                rotation_matrices.push((t0_jd, m));
                pos += 80;
            }
            continue;
        }
        if stype == 4 {
            let n = degree + 1;
            let gs = 2 + 3 * n;
            let mut records = Vec::new();
            for _ in 0..gcount {
                if pos + gs * 8 > data.len() {
                    break;
                }
                let f = |i: usize| -> f64 {
                    let mut buf = [0u8; 8];
                    buf.copy_from_slice(&data[pos + i * 8..pos + i * 8 + 8]);
                    f64::from_le_bytes(buf)
                };
                let mid_jd = f(0);
                let half_jd = f(1);
                let mut ra = Vec::with_capacity(n);
                let mut dec = Vec::with_capacity(n);
                let mut pm = Vec::with_capacity(n);
                for k in 0..n {
                    ra.push(f(2 + k));
                    dec.push(f(2 + n + k));
                    pm.push(f(2 + 2 * n + k));
                }
                records.push(NutationRecord {
                    mid_jd,
                    half_jd,
                    ra,
                    dec,
                    pm,
                });
                pos += gs * 8;
            }
            if let Some(ref mut p) = props {
                p.nutation = Some(records);
            }
            continue;
        }
        if stype != 0 {
            pos += gcount * (2 + 3 * (degree + 1)) * 8;
            continue;
        }
        let n = degree + 1;
        let gs = 2 + 3 * n;
        for _ in 0..gcount {
            if pos + gs * 8 > data.len() {
                break;
            }
            let f = |i: usize| -> f64 {
                let mut buf = [0u8; 8];
                buf.copy_from_slice(&data[pos + i * 8..pos + i * 8 + 8]);
                f64::from_le_bytes(buf)
            };
            let mut cx = [0f64; CHEBYSHEV_N];
            let mut cy = [0f64; CHEBYSHEV_N];
            let mut cz = [0f64; CHEBYSHEV_N];
            for k in 0..n.min(CHEBYSHEV_N) {
                cx[k] = f(2 + k);
                cy[k] = f(2 + n + k);
                cz[k] = f(2 + 2 * n + k);
            }
            granules.push(ChebyshevGranule {
                t0_jd: f(0),
                dt_jd: f(1),
                cx,
                cy,
                cz,
            });
            pos += gs * 8;
        }
    }
    if props.is_some() && !has_stype2 {
        return None;
    }
    if granules.is_empty() {
        None
    } else {
        Some(BodyEphemeris {
            granules,
            rotation_matrices,
            props,
        })
    }
}

pub fn frame_body_name(frame: &Frame) -> String {
    match frame {
        Frame::Surface { body_name, .. } | Frame::Barycenter { body_name, .. } => body_name.clone(),
        Frame::Manifest => String::new(),
    }
}

pub fn parse_iso_tdb(s: &str, lsk: &LeapSeconds) -> Option<f64> {
    let s = s.trim();
    let (date, time) = if let Some((d, t)) = s.split_once('T') {
        (d, t)
    } else if let Some((d, t)) = s.split_once(' ') {
        (d, t)
    } else {
        return None;
    };
    let mut dp = date.split('-');
    let y: i64 = dp.next()?.parse().ok()?;
    let m: u32 = dp.next()?.parse().ok()?;
    let d: u32 = dp.next()?.parse().ok()?;
    let t = match time
        .split(|c: char| c == '.' || c == 'Z' || c == 'z')
        .next()
    {
        Some(t) => t,
        None => return None,
    };
    let mut tp = t.split(':');
    let hh: u32 = tp.next()?.parse().ok()?;
    let mm: u32 = tp.next()?.parse().ok()?;
    let ss: u32 = match tp.next() {
        Some(s) => s,
        None => return None,
    }
    .parse()
    .ok()?;
    let days = ymd_to_days(y, m, d)? as i64;
    let unix = days * 86400 + (hh as i64) * 3600 + (mm as i64) * 60 + ss as i64;
    lsk.unix_to_tdb(unix as f64)
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

fn split_csv_line(line: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut cur = String::new();
    let mut in_quotes = false;
    let mut chars = line.chars().peekable();
    while let Some(c) = chars.next() {
        if in_quotes {
            if c == '"' {
                if chars.peek() == Some(&'"') {
                    chars.next();
                    cur.push('"');
                } else {
                    in_quotes = false;
                }
            } else {
                cur.push(c);
            }
        } else if c == '"' {
            in_quotes = true;
        } else if c == ',' {
            fields.push(std::mem::take(&mut cur));
        } else {
            cur.push(c);
        }
    }
    fields.push(cur);
    fields
}

pub fn csv_to_json(text: &str) -> Option<JsonVal> {
    let mut lines = text
        .lines()
        .filter(|l| !l.trim().is_empty() && !l.trim().starts_with('#'));
    let header_line = lines.find(|l| l.contains(','))?;
    let headers = split_csv_line(header_line);
    if headers.len() < 2 {
        return None;
    }
    let mut rows = Vec::new();
    for line in lines {
        if !line.contains(',') {
            continue;
        }
        let fields = split_csv_line(line);
        if fields.len() != headers.len() {
            continue;
        }
        let mut obj = HashMap::new();
        for (h, f) in headers.iter().zip(fields.iter()) {
            obj.insert(h.clone(), JsonVal::Str(f.clone()));
        }
        rows.push(JsonVal::Obj(obj));
    }
    Some(JsonVal::Arr(rows))
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
            fields.push(FieldConfig {
                key: "val".into(),
                name: "val".into(),
                kernel: 0,
                force: 0,
                tau: 0.0,
                absorption: 0.0,
                advection: 0.0,
                unit: String::new(),
                fold: None,
            });
        }
        if first.contains_key("extent") {
            fields.push(FieldConfig {
                key: "extent".into(),
                name: "extent".into(),
                kernel: 0,
                force: 0,
                tau: 0.0,
                absorption: 0.0,
                advection: 0.0,
                unit: String::new(),
                fold: None,
            });
        }
        if first.contains_key("tau") {
            fields.push(FieldConfig {
                key: "tau".into(),
                name: "tau".into(),
                kernel: 0,
                force: 0,
                tau: 0.0,
                absorption: 0.0,
                advection: 0.0,
                unit: String::new(),
                fold: None,
            });
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
            tau_key: String::new(),
        }]
    } else if has_lat && has_lon {
        let alt_key = if first.contains_key("alt") { "alt" } else { "" };
        let epoch_key = if first.contains_key("t") { "t" } else { "" };
        let vel_key = if first.contains_key("vel") { "vel" } else { "" };
        let trk_key = if first.contains_key("trk") { "trk" } else { "" };
        let vr_key = if first.contains_key("vr") { "vr" } else { "" };
        let mut fields = vec![];
        if first.contains_key("val") {
            fields.push(FieldConfig {
                key: "val".into(),
                name: "val".into(),
                kernel: 0,
                force: 0,
                tau: 0.0,
                absorption: 0.0,
                advection: 0.0,
                unit: String::new(),
                fold: None,
            });
        }
        if first.contains_key("extent") {
            fields.push(FieldConfig {
                key: "extent".into(),
                name: "extent".into(),
                kernel: 0,
                force: 0,
                tau: 0.0,
                absorption: 0.0,
                advection: 0.0,
                unit: String::new(),
                fold: None,
            });
        }
        vec![Extract::Map {
            arr_path: "data".into(),
            lat_key: "lat".into(),
            lon_key: "lon".into(),
            alt_key: alt_key.into(),
            epoch_key: epoch_key.into(),
            val_key: String::new(),
            alt_scale: -1.0,
            vel_key: vel_key.into(),
            vel_scale: 1.0,
            trk_key: trk_key.into(),
            vr_key: vr_key.into(),
            fields,
            lat_sign: None,
            lon_sign: None,
            epoch_scale: 1.0,
            tau_key: String::new(),
            mag_type_key: String::new(),
        }]
    } else {
        vec![]
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
        let stripped = (if let Some(s) = trimmed.strip_prefix('#') {
            s
        } else {
            trimmed
        })
        .trim();
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
            || trimmed.chars().next().is_some_and(|c| c.is_alphabetic())
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
            match r.find(|c: char| c.is_whitespace() || c == '<' || c == '"') {
                Some(pos) => pos,
                None => r.len(),
            }
        } else {
            match r.find(suffix) {
                Some(pos) => pos,
                None => r.len(),
            }
        };
        return r[..e].trim().parse::<f64>().ok();
    }

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
                    let void_matches = matches!(esc, b'D' | b'S' | b'W');
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
                        let ok = match bc() {
                            Some(c) => check(c),
                            None => void_matches,
                        };
                        if !ok {
                            return None;
                        }
                        bi += 1;
                    }
                    if max == usize::MAX {
                        while b.get(bi).map_or(false, |&c| check(c)) {
                            bi += 1;
                        }
                    } else if min == 0 && max == 1 {
                        if b.get(bi).map_or(false, |&c| check(c)) {
                            bi += 1;
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
                    if let Some(new_bi) = match_re(0, &p[pi + 1..end], bi, b, cap) {
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
                        } else if p.get(pi + 1).map_or(false, |&c| c == b'-')
                            && p.get(pi + 2).is_some()
                        {
                            let lo = p[pi];
                            let hi = p[pi + 2];
                            for c in lo..=hi {
                                cls.push(c);
                            }
                            pi += 3;
                        } else {
                            cls.push(p[pi]);
                            pi += 1;
                        }
                    }
                    pi += 1;
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
                        let in_cls = bc().map_or(false, |c| cls.contains(&c));
                        if neg == in_cls {
                            return None;
                        }
                        bi += 1;
                    }
                    if max == usize::MAX {
                        while b.get(bi).map_or(false, |c| cls.contains(c) != neg) {
                            bi += 1;
                        }
                    } else if min == 0 && max == 1 {
                        if b.get(bi).map_or(false, |c| cls.contains(c) != neg) {
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

    for start in 0..=body_bytes.len() {
        let mut cap: Option<f64> = None;
        if match_re(0, pat_bytes, start, body_bytes, &mut cap).is_some() {
            return cap;
        }
    }
    None
}

pub fn is_drop_key(key: &str) -> bool {
    let kl = key.to_lowercase();
    kl == "id"
        || kl == "hex"
        || kl == "flight"
        || kl == "callsign"
        || kl == "icao24"
        || kl == "origin_country"
        || kl == "evid"
        || kl == "publicid"
        || kl == "locality"
        || kl == "place"
        || kl == "region"
        || kl == "flynn_region"
        || kl == "satellite"
        || kl == "net"
        || kl == "source"
        || kl == "station"
        || kl == "name"
        || kl == "stid"
        || kl == "icao"
        || kl == "station_name"
        || kl == "country"
        || kl == "sitename"
        || kl == "variablename"
        || kl == "hypocenter"
        || kl == "code"
        || kl == "wmo"
        || kl == "wban"
        || kl == "usaf"
        || kl == "buoy_id"
        || kl == "platform"
        || kl == "sensor"
        || kl == "catalog"
        || is_time_key(key)
        || kl == "timestamp_utc"
        || kl == "observed_date"
        || kl == "generated"
        || kl == "local_date_time"
        || kl == "datetime"
        || kl == "timezone"
        || kl == "origintime"
        || kl == "obstime"
        || kl == "lastupdated"
        || kl == "begintime"
        || kl == "peaktime"
        || kl == "endtime"
        || kl == "announcedtime"
        || kl == "daynum"
        || kl == "type"
        || kl == "status"
        || kl == "alert"
        || kl == "magtype"
        || kl == "evtype"
        || kl == "auth"
        || kl == "iscancel"
        || kl == "isfinal"
        || kl == "domestictsunami"
        || kl == "issea"
        || kl == "istraining"
        || kl == "active"
        || kl == "count"
        || kl == "total"
        || kl == "number_spots"
        || kl == "station_count"
        || kl == "event_count"
        || kl == "multiplicity"
        || kl.starts_with("count_")
        || kl.starts_with("number_")
        || (kl.starts_with("n_") && kl.len() <= 5)
        || kl.ends_with("_index")
        || kl.ends_with("_scale")
        || kl.ends_with("_code")
        || kl.ends_with("_pct")
        || kl == "ssn"
        || kl == "kp_index"
        || kl == "estimated_kp"
        || kl == "kp"
        || kl == "a_running"
        || kl == "uv_index"
        || kl == "weather_code"
        || kl == "cdi"
        || kl == "mmi"
        || kl == "sig"
        || kl == "felt"
        || kl == "tsunami"
        || kl == "confidence"
        || kl == "dmin"
        || kl == "nst"
        || kl == "rms"
        || kl == "gap"
        || kl == "flare_index"
        || kl == "storm_level"
        || kl == "noaa_scale"
        || kl == "class"
        || kl == "classtype"
        || kl == "sample_size"
        || kl.ends_with("_size")
}

pub fn is_unit_name(name: &str) -> bool {
    let kl = name.to_lowercase();
    kl == "degt"
        || kl == "m/s"
        || kl == "sec"
        || kl == "hpa"
        || kl == "degc"
        || kl == "nmi"
        || kl == "ft"
        || kl == "m"
        || kl == "s"
        || kl == "cm"
        || kl == "mm"
        || kl == "km"
        || kl == "in"
        || kl == "inhg"
        || kl == "mb"
        || kl == "mbar"
        || kl == "kt"
        || kl == "mph"
        || kl == "knots"
        || kl == "m/sec"
        || kl == "deg"
}

pub fn text_to_json(text: &str) -> Option<JsonVal> {
    let header = text.lines().find_map(|line| {
        let t = line.trim();
        if !t.starts_with('#') {
            return None;
        }
        let stripped = t.trim_start_matches('#').trim();
        if stripped.is_empty() {
            return None;
        }
        let cols: Vec<String> = stripped.split_whitespace().map(|s| s.to_string()).collect();
        if cols.len() > 5 {
            Some(cols)
        } else {
            None
        }
    })?;
    let data = text.lines().find_map(|line| {
        let t = line.trim();
        if t.starts_with('#') {
            return None;
        }
        let cols: Vec<String> = t.split_whitespace().map(|s| s.to_string()).collect();
        if cols.len() >= header.len() {
            Some(cols)
        } else {
            None
        }
    })?;
    let mut obj = HashMap::new();
    for (name, value) in header[5..].iter().zip(data[5..].iter()) {
        let lower = name.to_lowercase();
        if lower == "yy" || lower == "mm" || lower == "dd" || lower == "hh" || lower == "min" {
            continue;
        }
        if is_unit_name(&lower) || is_drop_key(&lower) {
            continue;
        }
        if let Ok(n) = value.parse::<f64>() {
            obj.insert(name.clone(), JsonVal::Num(n));
        }
    }
    if obj.is_empty() {
        None
    } else {
        Some(JsonVal::Obj(obj))
    }
}

pub fn tap_to_json(val: &JsonVal) -> Option<JsonVal> {
    let obj = match val {
        JsonVal::Obj(m) => m,
        _ => return None,
    };
    let metadata = match obj.get("metadata") {
        Some(JsonVal::Arr(a)) => a,
        _ => return None,
    };
    let data = match obj.get("data") {
        Some(JsonVal::Arr(a)) => a,
        _ => return None,
    };
    let mut names: Vec<String> = Vec::new();
    for m in metadata {
        if let JsonVal::Obj(mo) = m {
            if let Some(JsonVal::Str(name)) = mo.get("name") {
                names.push(name.clone());
            }
        }
    }
    if names.is_empty() {
        return None;
    }
    let mut rows: Vec<JsonVal> = Vec::new();
    for d in data {
        if let JsonVal::Arr(row) = d {
            let mut row_map = HashMap::new();
            for (name, cell) in names.iter().zip(row.iter()) {
                row_map.insert(name.clone(), cell.clone());
            }
            rows.push(JsonVal::Obj(row_map));
        }
    }
    if rows.is_empty() {
        None
    } else {
        Some(JsonVal::Arr(rows))
    }
}

pub fn tdb_to_jd(tdb_secs: f64) -> f64 {
    tdb_secs / 86400.0 + J2000_EPOCH
}

pub fn flatten_geojson_coords(val: &[JsonVal]) -> Vec<(f64, f64, Option<f64>)> {
    if let Some(JsonVal::Num(_)) = val.first() {
        if val.len() >= 2 {
            if let (Some(lon), Some(lat)) = (scalar_of(&val[0]), scalar_of(&val[1])) {
                let z = if val.len() >= 3 {
                    scalar_of(&val[2])
                } else {
                    None
                };
                return vec![(lon, lat, z)];
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

pub enum ExtractResult {
    Measurements(Vec<(Channel, FieldConfig)>),
    WithEphemeris(Vec<(Channel, FieldConfig)>, BodyEphemeris),
}

pub fn extract(src: &SourceConfig, body: &str, now: f64, lsk: &LeapSeconds) -> ExtractResult {
    if src.format == "ephemeris_binary" {
        let mut buf = Vec::new();
        if let Ok(mut f) = std::fs::File::open(body) {
            use std::io::Read;
            f.read_to_end(&mut buf).ok();
        }
        if let Some(eph) = parse_ephemeris_binary(&buf) {
            return ExtractResult::WithEphemeris(vec![], eph);
        }
        return ExtractResult::Measurements(vec![]);
    }
    let mut channels: Vec<(Channel, FieldConfig)> = Vec::new();
    let mut extracted: HashMap<String, f64> = HashMap::new();
    let parsed_json = if src.format == "csv_zip" {
        std::fs::read(body)
            .ok()
            .and_then(|b| unzip(&b))
            .map(|bytes| String::from_utf8_lossy(&bytes).into_owned())
            .as_deref()
            .and_then(csv_to_json)
    } else if src.format == "csv" {
        csv_to_json(body)
    } else if src.format == "free text" {
        text_to_json(body)
    } else if src.format == "tap" {
        parse_json(body).and_then(|j| tap_to_json(&j))
    } else if src.format == "json" || src.format.is_empty() || src.format == "universal" {
        let body = body
            .strip_prefix("OK")
            .and_then(|r| r.strip_prefix('\n').or_else(|| r.strip_prefix("\r\n")))
            .unwrap_or(body);
        parse_json(body)
    } else {
        None
    };
    let auto_extracts: Option<Vec<Extract>>;
    let effective_extracts: &[Extract] = if src.format == "universal" && src.extracts.is_empty() {
        if let Some(ref j) = parsed_json {
            auto_extracts = Some(universal_auto_detect(j));
            if let Some(ref auto) = auto_extracts {
                auto.as_slice()
            } else {
                return ExtractResult::Measurements(vec![]);
            }
        } else {
            return ExtractResult::Measurements(vec![]);
        }
    } else {
        &src.extracts
    };
    for ext in effective_extracts {
        match ext {
            Extract::Field(fc) => {
                if let Some(ref j) = parsed_json {
                    if let Some(v) = jnum(j, &fc.key) {
                        extracted.insert(fc.name.clone(), v);
                    }
                }
            }
            Extract::First(fc) => {
                if let Some(ref j) = parsed_json {
                    if let Some(v) = jfirst(j, &fc.key) {
                        extracted.insert(fc.name.clone(), v);
                    }
                }
            }
            Extract::Last(fc) => {
                if let Some(ref j) = parsed_json {
                    if let Some(v) = jlast(j, &fc.key) {
                        extracted.insert(fc.name.clone(), v);
                    }
                } else if fc.key == "line" {
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
                        extracted.insert(fc.name.clone(), v);
                    }
                }
            }
            Extract::Count(fc) => {
                let v = if src.format == "csv" || fc.key == "lines" {
                    Some(
                        body.lines()
                            .filter(|l| !l.trim().is_empty() && !l.trim().starts_with('#'))
                            .count() as f64,
                    )
                } else {
                    parsed_json.as_ref().and_then(|j| jcount(j, &fc.key))
                };
                if let Some(v) = v {
                    extracted.insert(fc.name.clone(), v);
                }
            }
            Extract::LastRow(fc) => {
                if src.format == "csv" {
                    if let Some(v) = text_last_col(body, &fc.key) {
                        extracted.insert(fc.name.clone(), v);
                    }
                } else if let Some(ref j) = parsed_json {
                    if let Some(v) = j2d_last_row(j, &fc.key) {
                        extracted.insert(fc.name.clone(), v);
                    }
                } else if let Some(v) = text_last_col(body, &fc.key) {
                    extracted.insert(fc.name.clone(), v);
                }
            }
            Extract::Path(fc) => {
                if let Some(ref j) = parsed_json {
                    if let Some(v) = jpath(j, &fc.key) {
                        extracted.insert(fc.name.clone(), v);
                    }
                }
            }
            Extract::Deep(fc) => {
                if let Some(ref j) = parsed_json {
                    if let Some(v) = jdeep_find_num(j, &fc.key) {
                        extracted.insert(fc.name.clone(), v);
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
            Extract::ObjLast(fc) => {
                if let Some(ref j) = parsed_json {
                    if let Some(obj) = jpath_val(j, &fc.key) {
                        if let JsonVal::Obj(m) = obj {
                            if let Some(last_key) = m.keys().max_by(|a, b| {
                                if let (Ok(ka), Ok(kb)) = (a.parse::<i64>(), b.parse::<i64>()) {
                                    ka.cmp(&kb)
                                } else {
                                    a.cmp(b)
                                }
                            }) {
                                if let Some(val) = m.get(last_key).and_then(scalar_of) {
                                    extracted.insert(fc.name.clone(), val);
                                }
                            }
                        }
                    }
                }
            }
            Extract::Regex(fc) => {
                if let Some(v) = extract_regex_val(body, &fc.key) {
                    extracted.insert(fc.name.clone(), v);
                }
            }
            Extract::XmlCount(tag, n) => {
                let count = body.matches(&format!("<{}>", tag)).count() as f64;
                extracted.insert(n.clone(), count);
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
                alt_scale,
                vel_key,
                vel_scale,
                trk_key,
                vr_key,
                fields,
                lat_sign,
                lon_sign,
                epoch_scale,
                tau_key,
                mag_type_key,
            } => {
                let eff_lat_key = lat_key.clone();
                let eff_lon_key = lon_key.clone();
                let eff_epoch_key = epoch_key.clone();
                if let Some(ref j) = parsed_json {
                    let rows: Vec<&JsonVal> = match jpath_val(j, arr_path) {
                        Some(JsonVal::Arr(arr)) => arr.iter().collect(),
                        Some(obj @ JsonVal::Obj(_)) => vec![obj],
                        _ => Vec::new(),
                    };
                    {
                        for v in rows {
                            let lat = jpath(v, &eff_lat_key);
                            let lon = jpath(v, &eff_lon_key);
                            let alt = if alt_key.is_empty() {
                                Some(0.0)
                            } else {
                                jpath(v, alt_key).map(|a| a * alt_scale)
                            };
                            if let (Some(la), Some(lo), Some(al)) = (lat, lon, alt) {
                                let mut lat_val = la;
                                if let Some(sign_key) = lat_sign {
                                    if let Some(vv) = jpath_val(v, sign_key) {
                                        if let JsonVal::Str(s) = vv {
                                            if s.contains('S') || s.contains('s') {
                                                lat_val = -la;
                                            }
                                        }
                                    }
                                }
                                let mut lon_val = lo;
                                if let Some(sign_key) = lon_sign {
                                    if let Some(vv) = jpath_val(v, sign_key) {
                                        if let JsonVal::Str(s) = vv {
                                            if s.contains('W') || s.contains('w') {
                                                lon_val = -lo;
                                            }
                                        }
                                    }
                                }
                                let speed = if vel_key.is_empty() {
                                    None
                                } else {
                                    jpath(v, vel_key).map(|s| s * vel_scale)
                                };
                                let track = if trk_key.is_empty() {
                                    None
                                } else {
                                    jpath(v, trk_key)
                                };
                                let vrate = if vr_key.is_empty() {
                                    None
                                } else {
                                    jpath(v, vr_key).map(|s| s * vel_scale)
                                };
                                let position = if let (Some(sp), Some(tr)) = (speed, track) {
                                    Position::SurfaceFlow {
                                        body_name: frame_body_name(&src.frame),
                                        lat: lat_val,
                                        lon: lon_val,
                                        alt: al,
                                        speed: sp,
                                        track: tr,
                                        vrate,
                                    }
                                } else {
                                    Position::Surface {
                                        body_name: frame_body_name(&src.frame),
                                        lat: lat_val,
                                        lon: lon_val,
                                        alt: al,
                                    }
                                };
                                let epoch = if eff_epoch_key.is_empty() {
                                    now
                                } else if let Some(ev) = jpath_val(v, &eff_epoch_key) {
                                    match ev {
                                        JsonVal::Str(s) => {
                                            if let Some(t) = parse_iso_tdb(s, lsk) {
                                                t
                                            } else {
                                                continue;
                                            }
                                        }
                                        JsonVal::Num(n) => {
                                            match lsk.unix_to_tdb(*n * epoch_scale) {
                                                Some(t) => t,
                                                None => continue,
                                            }
                                        }
                                        _ => continue,
                                    }
                                } else {
                                    continue;
                                };
                                let row_tau: Option<f64> = if tau_key.is_empty() {
                                    None
                                } else {
                                    match jpath(v, tau_key) {
                                        Some(t) if t > 0.0 => Some(t),
                                        Some(_) => continue,
                                        None => None,
                                    }
                                };
                                for fc in fields {
                                    if !val_key.is_empty() && fc.name != *val_key {
                                        continue;
                                    }
                                    let mut raw = jpath(v, &fc.key);
                                    if !mag_type_key.is_empty()
                                        && fc.unit.eq_ignore_ascii_case("mw")
                                    {
                                        if let Some(t) = jstr(v, &mag_type_key) {
                                            if !is_moment_magnitude(&t) {
                                                continue;
                                            }
                                        }
                                    }
                                    let mut transformed = false;
                                    if let Some((op, key_b)) = &fc.fold {
                                        raw = fold_value(raw, jpath(v, key_b), *op);
                                    } else if let Some(ref mag_key) = src.flux_from_mag {
                                        if fc.key == *mag_key {
                                            raw = raw.map(|r| 10.0f64.powf(-0.4 * r));
                                            transformed = true;
                                        }
                                    }
                                    let val = match raw {
                                        Some(vv) => vv,
                                        None => continue,
                                    };
                                    if !val.is_finite() {
                                        continue;
                                    }
                                    let mut eff_fc = (*fc).clone();
                                    if transformed {
                                        eff_fc.unit.clear();
                                    }
                                    if let Some(t) = row_tau {
                                        eff_fc.tau = t;
                                    }
                                    channels.push((
                                        Channel {
                                            z: 0.0,
                                            freq: 0.0,
                                            bin_width: 0.0,
                                            epoch,
                                            position: position.clone(),
                                            name: fc.name.clone(),
                                            value: val,
                                        },
                                        eff_fc,
                                    ));
                                }
                            }
                        }
                    }
                }
            }
            Extract::ProfileMap {
                arr_path,
                lat_key,
                lon_key,
                epoch_key,
                pressure_var,
                pressure_scale,
                fields,
            } => {
                if let Some(ref j) = parsed_json {
                    let rows: Vec<&JsonVal> = match jpath_val(j, arr_path) {
                        Some(JsonVal::Arr(arr)) => arr.iter().collect(),
                        Some(obj @ JsonVal::Obj(_)) => vec![obj],
                        _ => Vec::new(),
                    };
                    for v in rows {
                        let lat = jpath(v, &lat_key);
                        let lon = jpath(v, &lon_key);
                        if let (Some(la), Some(lo)) = (lat, lon) {
                            let epoch = if epoch_key.is_empty() {
                                now
                            } else if let Some(ev) = jpath_val(v, &epoch_key) {
                                match ev {
                                    JsonVal::Str(s) => {
                                        if let Some(t) = parse_iso_tdb(s, lsk) {
                                            t
                                        } else {
                                            continue;
                                        }
                                    }
                                    JsonVal::Num(n) => match lsk.unix_to_tdb(*n) {
                                        Some(t) => t,
                                        None => continue,
                                    },
                                    _ => continue,
                                }
                            } else {
                                continue;
                            };
                            let data = match jpath_val(v, "data") {
                                Some(JsonVal::Arr(d)) => d,
                                _ => continue,
                            };
                            let (var_names, pressure_idx) = match jpath_val(v, "data_info") {
                                Some(JsonVal::Arr(info)) => {
                                    let names: Vec<String> = match info.first() {
                                        Some(JsonVal::Arr(n)) => n
                                            .iter()
                                            .filter_map(|e| match e {
                                                JsonVal::Str(s) => Some(s.clone()),
                                                _ => None,
                                            })
                                            .collect(),
                                        _ => Vec::new(),
                                    };
                                    let pidx = names.iter().position(|n| n == pressure_var);
                                    (names, pidx)
                                }
                                _ => (Vec::new(), None),
                            };
                            let pidx = match pressure_idx {
                                Some(i) => i,
                                None => continue,
                            };
                            let pressure = match data.get(pidx) {
                                Some(JsonVal::Arr(p)) => p,
                                _ => continue,
                            };
                            let n_levels = pressure.len();
                            for fc in fields {
                                let vidx = match var_names.iter().position(|n| *n == fc.key) {
                                    Some(i) => i,
                                    None => continue,
                                };
                                let values = match data.get(vidx) {
                                    Some(JsonVal::Arr(a)) => a,
                                    _ => continue,
                                };
                                for k in 0..n_levels {
                                    let p = match pressure.get(k) {
                                        Some(JsonVal::Num(x)) => *x,
                                        _ => continue,
                                    };
                                    let val = match values.get(k) {
                                        Some(JsonVal::Num(x)) => *x,
                                        _ => continue,
                                    };
                                    if !val.is_finite() {
                                        continue;
                                    }
                                    let position = Position::Surface {
                                        body_name: frame_body_name(&src.frame),
                                        lat: la,
                                        lon: lo,
                                        alt: -p * pressure_scale,
                                    };
                                    channels.push((
                                        Channel {
                                            z: 0.0,
                                            freq: 0.0,
                                            bin_width: 0.0,
                                            epoch,
                                            position,
                                            name: fc.name.clone(),
                                            value: val,
                                        },
                                        fc.clone(),
                                    ));
                                }
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
                if let Some(ref j) = parsed_json {
                    if let Some(JsonVal::Arr(arr)) = jpath_val(j, arr_path) {
                        for v in arr.iter() {
                            let coords = if geom_path.is_empty() {
                                match jpath_val(v, "coordinates") {
                                    Some(JsonVal::Arr(c)) => c,
                                    _ => match v {
                                        JsonVal::Arr(c) => c,
                                        _ => continue,
                                    },
                                }
                            } else {
                                let geom = match jpath_val(v, geom_path) {
                                    Some(g) => g,
                                    None => continue,
                                };
                                match jpath_val(geom, "coordinates") {
                                    Some(JsonVal::Arr(c)) => c,
                                    _ => match geom {
                                        JsonVal::Arr(c) => c,
                                        _ => continue,
                                    },
                                }
                            };
                            let vertices = flatten_geojson_coords(coords);
                            if vertices.is_empty() {
                                continue;
                            }
                            let row_epoch = if !epoch_key.is_empty() {
                                match jpath(v, epoch_key) {
                                    Some(ev) => ev,
                                    None => continue,
                                }
                            } else {
                                continue;
                            };
                            for (lon, lat, z) in vertices {
                                let position = Position::Surface {
                                    body_name: frame_body_name(&src.frame),
                                    lat,
                                    lon,
                                    alt: match z {
                                        Some(a) => a,
                                        None => continue,
                                    },
                                };
                                for fc in fields {
                                    let mut raw = jpath(v, &fc.key);
                                    let mut transformed = false;
                                    if let Some((op, key_b)) = &fc.fold {
                                        raw = fold_value(raw, jpath(v, key_b), *op);
                                    } else if let Some(ref mag_key) = src.flux_from_mag {
                                        if fc.key == *mag_key {
                                            raw = raw.map(|r| 10.0f64.powf(-0.4 * r));
                                            transformed = true;
                                        }
                                    }
                                    let val = match raw {
                                        Some(vv) => vv,
                                        None => continue,
                                    };
                                    if !val.is_finite() {
                                        continue;
                                    }
                                    let mut eff_fc = (*fc).clone();
                                    if transformed {
                                        eff_fc.unit.clear();
                                    }
                                    channels.push((
                                        Channel {
                                            z: 0.0,
                                            freq: 0.0,
                                            bin_width: 0.0,
                                            epoch: row_epoch,
                                            position: position.clone(),
                                            name: fc.name.clone(),
                                            value: val,
                                        },
                                        eff_fc,
                                    ));
                                }
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
                        for v in arr.iter() {
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
                                continue;
                            } else if let Some(ev) = jpath_val(v, epoch_key) {
                                match ev {
                                    JsonVal::Str(s) => {
                                        if let Some(t) = parse_iso_tdb(s, lsk) {
                                            t
                                        } else {
                                            continue;
                                        }
                                    }
                                    JsonVal::Num(n) => match lsk.unix_to_tdb(*n) {
                                        Some(t) => t,
                                        None => continue,
                                    },
                                    _ => continue,
                                }
                            } else {
                                continue;
                            };
                            let alt = match alt_key {
                                k if k.is_empty() => continue,
                                _ => match jpath(v, alt_key) {
                                    Some(a) => a,
                                    None => continue,
                                },
                            };
                            for fc in fields {
                                if !val_key.is_empty() && fc.name != *val_key {
                                    continue;
                                }
                                let mut raw = jpath(v, &fc.key);
                                let mut transformed = false;
                                if let Some(ref mag_key) = src.flux_from_mag {
                                    if fc.key == *mag_key {
                                        raw = raw.map(|r| 10.0f64.powf(-0.4 * r));
                                        transformed = true;
                                    }
                                }
                                let val = match raw {
                                    Some(vv) => vv,
                                    None => continue,
                                };
                                if !val.is_finite() {
                                    continue;
                                }
                                let mut eff_fc = (*fc).clone();
                                if transformed {
                                    eff_fc.unit.clear();
                                }
                                for (lon, lat) in vertices.iter() {
                                    channels.push((
                                        Channel {
                                            z: 0.0,
                                            freq: 0.0,
                                            bin_width: 0.0,
                                            epoch,
                                            position: Position::Surface {
                                                body_name: frame_body_name(&src.frame),
                                                lat: *lat,
                                                lon: *lon,
                                                alt,
                                            },
                                            name: fc.name.clone(),
                                            value: val,
                                        },
                                        eff_fc.clone(),
                                    ));
                                }
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
                        for v in arr.iter() {
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
                            let row_epoch = if !epoch_key.is_empty() {
                                match jpath(v, epoch_key) {
                                    Some(ev) => ev,
                                    None => continue,
                                }
                            } else {
                                continue;
                            };
                            for fc in fields {
                                if !val_key.is_empty() && fc.name != *val_key {
                                    continue;
                                }
                                let mut raw = jpath(v, &fc.key);
                                let mut transformed = false;
                                if let Some(ref mag_key) = src.flux_from_mag {
                                    if fc.key == *mag_key {
                                        raw = raw.map(|r| 10.0f64.powf(-0.4 * r));
                                        transformed = true;
                                    }
                                }
                                let val = match raw {
                                    Some(vv) => vv,
                                    None => continue,
                                };
                                if !val.is_finite() {
                                    continue;
                                }
                                let mut eff_fc = (*fc).clone();
                                if transformed {
                                    eff_fc.unit.clear();
                                }
                                for (ra_deg, dec_deg, _z) in &vertices {
                                    let ra = ra_deg.to_radians();
                                    let dec = dec_deg.to_radians();
                                    let (sa, ca) = ra.sin_cos();
                                    let (sd, cd) = dec.sin_cos();
                                    let p = [cd * ca * radius, cd * sa * radius, sd * radius];
                                    channels.push((
                                        Channel {
                                            z: 0.0,
                                            freq: 0.0,
                                            bin_width: 0.0,
                                            epoch: row_epoch,
                                            position: Position::StateVector {
                                                p,
                                                v: [0.0, 0.0, 0.0],
                                                track: false,
                                            },
                                            name: fc.name.clone(),
                                            value: val,
                                        },
                                        eff_fc.clone(),
                                    ));
                                }
                            }
                        }
                    }
                }
            }
            Extract::Rows {
                last_line,
                fields,
                tau_key,
            } => {
                if let Frame::Surface { lat, lon, alt, .. } = src.frame {
                    let position = Position::Surface {
                        body_name: frame_body_name(&src.frame),
                        lat,
                        lon,
                        alt,
                    };
                    let resolve_col = |key: &str| -> Option<usize> {
                        if let Ok(idx) = key.parse::<usize>() {
                            return Some(idx);
                        }
                        body.lines().find_map(|line| {
                            let t = line.trim();
                            if t.is_empty() {
                                return None;
                            }
                            let s = t.strip_prefix('#').unwrap_or(t).trim();
                            split_data_line(s)
                                .iter()
                                .position(|c| c.eq_ignore_ascii_case(key) || c.starts_with(key))
                        })
                    };
                    let col_fcs: Vec<(usize, Option<usize>, &FieldConfig)> = fields
                        .iter()
                        .filter_map(|fc| {
                            let idx = resolve_col(&fc.key)?;
                            let idx_b = match &fc.fold {
                                Some((_, kb)) => Some(resolve_col(kb)?),
                                None => None,
                            };
                            Some((idx, idx_b, fc))
                        })
                        .collect();
                    let tau_col = if tau_key.is_empty() {
                        None
                    } else {
                        resolve_col(&tau_key)
                    };
                    let lines: Vec<&str> = if *last_line {
                        body.lines()
                            .rev()
                            .find(|l| {
                                let t = l.trim();
                                !t.is_empty() && !t.starts_with('#')
                            })
                            .into_iter()
                            .collect()
                    } else {
                        body.lines()
                            .filter(|l| {
                                let t = l.trim();
                                !t.is_empty() && !t.starts_with('#')
                            })
                            .collect()
                    };
                    for line in lines {
                        let cols = split_data_line(line.trim());
                        let row_tau: Option<f64> = match tau_col {
                            None => None,
                            Some(idx) => match cols
                                .get(idx)
                                .and_then(|s| s.trim().trim_matches('"').parse::<f64>().ok())
                            {
                                Some(t) if t > 0.0 => Some(t),
                                Some(_) => continue,
                                None => None,
                            },
                        };
                        for (idx, idx_b, fc) in &col_fcs {
                            let raw = cols
                                .get(*idx)
                                .and_then(|s| s.trim().trim_matches('"').parse::<f64>().ok());
                            let val = match (&fc.fold, idx_b) {
                                (Some((op, _)), Some(bi)) => fold_value(
                                    raw,
                                    cols.get(*bi).and_then(|s| {
                                        s.trim().trim_matches('"').parse::<f64>().ok()
                                    }),
                                    *op,
                                ),
                                _ => raw,
                            };
                            let val = match val {
                                Some(v) => v,
                                None => continue,
                            };
                            if !val.is_finite() {
                                continue;
                            }
                            let mut eff_fc = (*fc).clone();
                            if let Some(t) = row_tau {
                                eff_fc.tau = t;
                            }
                            channels.push((
                                Channel {
                                    z: 0.0,
                                    freq: 0.0,
                                    bin_width: 0.0,
                                    epoch: now,
                                    position: position.clone(),
                                    name: fc.name.clone(),
                                    value: val,
                                },
                                eff_fc,
                            ));
                        }
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
                q_key,
                tp_key,
                fields,
            } => {
                if let Some(ref j) = parsed_json {
                    if let Some(JsonVal::Arr(arr)) = jpath_val(j, arr_path) {
                        let jd_now = tdb_to_jd(now);
                        for v in arr.iter() {
                            let (Some(e_val), Some(i_val), Some(om_val), Some(w_val)) = (
                                jpath(v, e_key),
                                jpath(v, i_key),
                                jpath(v, om_key),
                                jpath(v, w_key),
                            ) else {
                                continue;
                            };
                            if !(0.0..1.0).contains(&e_val) {
                                continue;
                            }
                            let (Some(epoch_val),) = (jpath(v, epoch_key),) else {
                                continue;
                            };
                            let a_au = if !a_key.is_empty() {
                                match jpath(v, a_key) {
                                    Some(a) if a > 0.0 => a,
                                    _ => continue,
                                }
                            } else if !q_key.is_empty() {
                                match jpath(v, q_key) {
                                    Some(q) if q > 0.0 => q / (1.0 - e_val),
                                    _ => continue,
                                }
                            } else {
                                continue;
                            };
                            let ma_deg = if !ma_key.is_empty() {
                                match jpath(v, ma_key) {
                                    Some(m) => m,
                                    None => continue,
                                }
                            } else if !tp_key.is_empty() {
                                let Some(tp) = jpath(v, tp_key) else {
                                    continue;
                                };
                                let n_deg_day = GAUSS_K / (a_au * a_au * a_au).sqrt()
                                    * (180.0 / std::f64::consts::PI);
                                n_deg_day * (epoch_val - tp)
                            } else {
                                continue;
                            };
                            let (p, vel) = match crate::kepler::elements_to_icrs_state(
                                a_au, e_val, i_val, om_val, w_val, ma_deg, epoch_val, jd_now,
                            ) {
                                Some(st) => st,
                                None => continue,
                            };
                            for fc in fields {
                                let mut raw = jpath(v, &fc.key);
                                let mut transformed = false;
                                if let Some(ref mag_key) = src.flux_from_mag {
                                    if fc.key == *mag_key {
                                        raw = raw.map(|r| 10.0f64.powf(-0.4 * r));
                                        transformed = true;
                                    }
                                }
                                let val = match raw {
                                    Some(vv) => vv,
                                    None => continue,
                                };
                                if !val.is_finite() {
                                    continue;
                                }
                                let mut eff_fc = (*fc).clone();
                                if transformed {
                                    eff_fc.unit.clear();
                                }
                                channels.push((
                                    Channel {
                                        z: 0.0,
                                        freq: 0.0,
                                        bin_width: 0.0,
                                        epoch: now,
                                        position: Position::StateVector {
                                            p,
                                            v: vel,
                                            track: false,
                                        },
                                        name: fc.name.clone(),
                                        value: val,
                                    },
                                    eff_fc,
                                ));
                            }
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
                tau_key,
            } => {
                let default_epoch = if let Some(e) = src.catalog_epoch {
                    e
                } else {
                    now
                };
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
                                    Some(plx) if plx.is_finite() && plx > 0.0 => {
                                        PARSEC_M * 1000.0 / plx
                                    }
                                    _ => {
                                        if !z_key.is_empty() {
                                            match jpath(v, z_key) {
                                                Some(z) if z.is_finite() && z > 0.0 => {
                                                    z * C_LIGHT / HUBBLE_H0
                                                }
                                                _ => {
                                                    if !dist_key.is_empty() {
                                                        match jpath(v, dist_key) {
                                                            Some(dd)
                                                                if dd.is_finite() && dd > 0.0 =>
                                                            {
                                                                dd * dist_scale
                                                            }
                                                            _ => continue,
                                                        }
                                                    } else {
                                                        continue;
                                                    }
                                                }
                                            }
                                        } else if !dist_key.is_empty() {
                                            match jpath(v, dist_key) {
                                                Some(dd) if dd.is_finite() && dd > 0.0 => {
                                                    dd * dist_scale
                                                }
                                                _ => continue,
                                            }
                                        } else {
                                            continue;
                                        }
                                    }
                                }
                            } else if !dist_key.is_empty() {
                                match jpath(v, dist_key) {
                                    Some(dd) if dd.is_finite() && dd > 0.0 => dd * dist_scale,
                                    _ => {
                                        if !z_key.is_empty() {
                                            match jpath(v, z_key) {
                                                Some(z) if z.is_finite() && z > 0.0 => {
                                                    z * C_LIGHT / HUBBLE_H0
                                                }
                                                _ => continue,
                                            }
                                        } else {
                                            continue;
                                        }
                                    }
                                }
                            } else if !z_key.is_empty() {
                                match jpath(v, z_key) {
                                    Some(z) if z.is_finite() && z > 0.0 => z * C_LIGHT / HUBBLE_H0,
                                    _ => continue,
                                }
                            } else {
                                continue;
                            };
                            let zval = if !z_key.is_empty() {
                                jpath(v, z_key).filter(|z| z.is_finite() && *z > 0.0)
                            } else {
                                None
                            };
                            let ra = ra_deg.to_radians();
                            let dec = dec_deg.to_radians();
                            let (sa, ca) = ra.sin_cos();
                            let (sd, cd) = dec.sin_cos();
                            let p_hat = [cd * ca, cd * sa, sd];
                            let p = [p_hat[0] * d, p_hat[1] * d, p_hat[2] * d];
                            let mu_a = if pmra_key.is_empty() {
                                None
                            } else {
                                jpath(v, pmra_key)
                                    .filter(|x| x.is_finite())
                                    .map(|v| v * MAS_YR_TO_RAD_S)
                            };
                            let mu_d = if pmdec_key.is_empty() {
                                None
                            } else {
                                jpath(v, pmdec_key)
                                    .filter(|x| x.is_finite())
                                    .map(|v| v * MAS_YR_TO_RAD_S)
                            };
                            let vr = if rv_key.is_empty() {
                                None
                            } else {
                                jpath(v, rv_key)
                                    .filter(|x| x.is_finite())
                                    .map(|v| v * rv_scale)
                            };
                            let a_hat = [-sa, ca, 0.0];
                            let d_hat = [-sd * ca, -sd * sa, cd];
                            let vel = [
                                d * (mu_a.map_or(0.0, |m| m * a_hat[0])
                                    + mu_d.map_or(0.0, |m| m * d_hat[0]))
                                    + vr.map_or(0.0, |v| v * p_hat[0]),
                                d * (mu_a.map_or(0.0, |m| m * a_hat[1])
                                    + mu_d.map_or(0.0, |m| m * d_hat[1]))
                                    + vr.map_or(0.0, |v| v * p_hat[1]),
                                d * (mu_a.map_or(0.0, |m| m * a_hat[2])
                                    + mu_d.map_or(0.0, |m| m * d_hat[2]))
                                    + vr.map_or(0.0, |v| v * p_hat[2]),
                            ];
                            let sample_epoch = if !epoch_key.is_empty() {
                                if let Some(v) = jpath(v, epoch_key) {
                                    v
                                } else {
                                    continue;
                                }
                            } else {
                                default_epoch
                            };
                            let row_tau: Option<f64> = if tau_key.is_empty() {
                                None
                            } else {
                                match jpath(v, tau_key) {
                                    Some(t) if t > 0.0 => Some(t),
                                    Some(_) => continue,
                                    None => None,
                                }
                            };
                            for fc in fields {
                                let mut raw: Option<f64> = jpath(v, &fc.key);
                                let mut transformed = false;
                                if let Some((op, key_b)) = &fc.fold {
                                    raw = fold_value(raw, jpath(v, key_b), *op);
                                } else if let Some(ref mag_field) = src.abs_mag_from {
                                    if fc.name == *mag_field {
                                        raw = raw.map(|v| {
                                            let dist_pc = d / PARSEC_M;
                                            let abs_m = v - 5.0 * (dist_pc / 10.0).log10();
                                            10.0f64.powf(-0.4 * abs_m)
                                        });
                                        transformed = true;
                                    }
                                } else if let Some(ref mag_key) = src.flux_from_mag {
                                    if fc.key == *mag_key {
                                        raw = raw.map(|r| 10.0f64.powf(-0.4 * r));
                                        transformed = true;
                                    }
                                }
                                let val = match raw {
                                    Some(vv) => vv,
                                    None => continue,
                                };
                                if !val.is_finite() {
                                    continue;
                                }
                                let mut eff_fc = (*fc).clone();
                                if transformed {
                                    eff_fc.unit.clear();
                                }
                                if let Some(t) = row_tau {
                                    eff_fc.tau = t;
                                }
                                channels.push((
                                    Channel {
                                        z: zval.unwrap_or(0.0),
                                        freq: 0.0,
                                        bin_width: 0.0,
                                        epoch: sample_epoch,
                                        position: Position::StateVector {
                                            p,
                                            v: vel,
                                            track: false,
                                        },
                                        name: fc.name.clone(),
                                        value: val,
                                    },
                                    eff_fc,
                                ));
                            }
                        }
                    }
                }
            }
            Extract::GeojsonEvents {
                mag_key,
                min_mag,
                outputs,
                tau,
                absorption,
                advection,
                mag_type_key,
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
                                        let mut mag: Option<f64> = None;
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
                                                    if m.is_finite() {
                                                        mag = Some(m);
                                                    }
                                                }
                                                if !mag_type_key.is_empty() {
                                                    if let Some(t) = jstr(props, &mag_type_key) {
                                                        if !is_moment_magnitude(&t) {
                                                            continue;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        if let Some(mag) = mag {
                                            if mag >= *min_mag {
                                                channels.push((
                                                    Channel {
                                                        z: 0.0,
                                                        freq: 0.0,
                                                        bin_width: 0.0,
                                                        epoch: now,
                                                        position: Position::Surface {
                                                            body_name: frame_body_name(&src.frame),
                                                            lat: ela,
                                                            lon: elo,
                                                            alt: -ed * 1000.0,
                                                        },
                                                        name: outputs[0].clone(),
                                                        value: mag,
                                                    },
                                                    FieldConfig {
                                                        key: outputs[0].clone(),
                                                        name: outputs[0].clone(),
                                                        kernel: 0,
                                                        force: 3,
                                                        tau: *tau,
                                                        absorption: *absorption,
                                                        advection: *advection,
                                                        unit: "Mw".to_string(),
                                                        fold: None,
                                                    },
                                                ));
                                                channels.push((
                                                    Channel {
                                                        z: 0.0,
                                                        freq: 0.0,
                                                        bin_width: 0.0,
                                                        epoch: now,
                                                        position: Position::Surface {
                                                            body_name: frame_body_name(&src.frame),
                                                            lat: ela,
                                                            lon: elo,
                                                            alt: -ed * 1000.0,
                                                        },
                                                        name: outputs[1].clone(),
                                                        value: ed * 1000.0,
                                                    },
                                                    FieldConfig {
                                                        key: outputs[1].clone(),
                                                        name: outputs[1].clone(),
                                                        kernel: 0,
                                                        force: 3,
                                                        tau: *tau,
                                                        absorption: *absorption,
                                                        advection: *advection,
                                                        unit: String::new(),
                                                        fold: None,
                                                    },
                                                ));
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
                        if let Some(JsonVal::Arr(data)) = root.get("data") {
                            let mut col: HashMap<String, usize> = HashMap::new();
                            let mut fill_of: HashMap<String, f64> = HashMap::new();
                            let mut has_params = false;
                            if let Some(JsonVal::Arr(params)) = root.get("parameters") {
                                for (i, p) in params.iter().enumerate() {
                                    if let JsonVal::Obj(po) = p {
                                        if let Some(JsonVal::Str(nn)) = po.get("name") {
                                            col.insert(nn.clone(), i);
                                            has_params = true;
                                            if let Some(fv) = po.get("fill").and_then(scalar_of) {
                                                fill_of.insert(nn.clone(), fv);
                                            }
                                        }
                                    }
                                }
                            }
                            if !has_params {
                                if pairs.len() == 1 {
                                    if let Some(JsonVal::Arr(row)) = data.last() {
                                        if let Some(val) = row.last().and_then(scalar_of) {
                                            extracted.insert(pairs[0].1.clone(), val);
                                        }
                                    }
                                    continue;
                                }
                                for (i, (param, _)) in pairs.iter().enumerate() {
                                    col.insert(param.clone(), i + 1);
                                }
                            }
                            if let Some(last_row) = data.last() {
                                if let JsonVal::Arr(row) = last_row {
                                    for (param, name) in pairs {
                                        let (base, comp) = match param.rfind('.') {
                                            Some(dot)
                                                if param[dot + 1..]
                                                    .chars()
                                                    .all(|c| c.is_ascii_digit()) =>
                                            {
                                                (
                                                    &param[..dot],
                                                    param[dot + 1..].parse::<usize>().ok(),
                                                )
                                            }
                                            _ => (param.as_str(), None),
                                        };
                                        if let Some(&idx) = col.get(base) {
                                            let v = match comp {
                                                Some(i) => row.get(idx).and_then(|cell| {
                                                    if let JsonVal::Arr(a) = cell {
                                                        a.get(i).and_then(scalar_of)
                                                    } else {
                                                        None
                                                    }
                                                }),
                                                None => row.get(idx).and_then(scalar_of),
                                            };
                                            if let Some(val) = v {
                                                if fill_of.get(base).map_or(true, |&f| val != f) {
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
            Extract::TransitMap { .. } => {}
            Extract::Alerce(_) => {}
        }
    }
    if !extracted.is_empty() {
        for (name, val) in &extracted {
            let fc = effective_extracts.iter().find_map(|ext| match ext {
                Extract::Field(fc)
                | Extract::First(fc)
                | Extract::Last(fc)
                | Extract::Count(fc)
                | Extract::LastRow(fc)
                | Extract::ObjLast(fc)
                | Extract::Path(fc)
                | Extract::Deep(fc)
                | Extract::Regex(fc) => {
                    if fc.name == *name && fc.tau > 0.0 {
                        Some(fc)
                    } else {
                        None
                    }
                }
                _ => None,
            });
            if let Some(fc) = fc {
                let mut raw = Some(*val);
                let mut transformed = false;
                if let Some(ref mag_key) = src.flux_from_mag {
                    if fc.key == *mag_key {
                        raw = raw.map(|v| 10.0f64.powf(-0.4 * v));
                        transformed = true;
                    }
                }
                let val = match raw {
                    Some(v) => v,
                    None => continue,
                };
                if !val.is_finite() {
                    continue;
                }
                let mut eff_fc = fc.clone();
                if transformed {
                    eff_fc.unit.clear();
                }
                channels.push((
                    Channel {
                        z: 0.0,
                        freq: 0.0,
                        bin_width: 0.0,
                        epoch: now,
                        position: Position::Source,
                        name: fc.name.clone(),
                        value: val,
                    },
                    eff_fc,
                ));
            }
        }
    }
    channels.retain(|(c, _)| c.value.is_finite());
    ExtractResult::Measurements(channels)
}

fn parse_field_config(parts: &[&str]) -> Option<(u8, u8, f64, f64, f64)> {
    let kernel = match kernel_id_of(parts[3]) {
        Some(k) => k,
        None => return None,
    };
    let force = match force_id_of(parts[4]) {
        Some(f) => f,
        None => return None,
    };
    let tau: f64 = match parts[6].parse() {
        Ok(v) if v > 0.0 => v,
        _ => return None,
    };
    let absorption: f64 = match parts[7].parse() {
        Ok(v) => v,
        Err(_) => return None,
    };
    let advection: f64 = match parts[8].parse() {
        Ok(v) => v,
        Err(_) => return None,
    };
    Some((kernel, force, tau, absorption, advection))
}

fn series_epoch_of(el: &JsonVal, lsk: &LeapSeconds) -> Option<f64> {
    match el {
        JsonVal::Obj(map) => {
            for (k, v) in map {
                if !is_time_key(k) {
                    continue;
                }
                match v {
                    JsonVal::Str(s) => {
                        if let Some(t) = parse_iso_tdb(s, lsk) {
                            return Some(t);
                        }
                    }
                    JsonVal::Num(n) => {
                        if let Some(t) = lsk.unix_to_tdb(*n) {
                            return Some(t);
                        }
                    }
                    _ => {}
                }
            }
            None
        }
        JsonVal::Arr(row) => {
            let first = row.first()?;
            match first {
                JsonVal::Str(s) => parse_iso_tdb(s, lsk),
                JsonVal::Num(n) => lsk.unix_to_tdb(*n),
                _ => None,
            }
        }
        _ => None,
    }
}

pub fn is_time_key(k: &str) -> bool {
    let kl = k.to_lowercase();
    kl == "time"
        || kl == "time_tag"
        || kl == "timestamp"
        || kl == "epoch"
        || kl == "t"
        || kl == "date"
        || kl.contains("time")
        || kl.contains("date")
}

pub fn extract_series(src: &SourceConfig, body: &str, lsk: &LeapSeconds) -> Vec<(f64, f64)> {
    let parsed = parse_json(body);
    let Some(ref j) = parsed else {
        return Vec::new();
    };
    let mut out: Vec<(f64, f64)> = Vec::new();
    for ext in &src.extracts {
        match ext {
            Extract::Last(fc) => {
                let JsonVal::Arr(elements) = j else {
                    continue;
                };
                for el in elements {
                    let raw = match el {
                        JsonVal::Obj(map) => map.get(&fc.key).and_then(scalar_of),
                        JsonVal::Arr(row) => {
                            if let Ok(idx) = fc.key.parse::<usize>() {
                                row.get(idx).and_then(scalar_of)
                            } else {
                                None
                            }
                        }
                        _ => None,
                    };
                    let (Some(raw), Some(epoch)) = (raw, series_epoch_of(el, lsk)) else {
                        continue;
                    };
                    let Some(val) = convert_to_si(raw, &fc.unit) else {
                        register_unconverted_unit(&fc.unit, &fc.name);
                        continue;
                    };
                    if val.is_finite() {
                        out.push((epoch, val));
                    }
                }
            }
            Extract::Path(fc) => {
                let JsonVal::Arr(elements) = j else {
                    continue;
                };
                for el in elements {
                    let raw = jpath(el, &fc.key);
                    let (Some(raw), Some(epoch)) = (raw, series_epoch_of(el, lsk)) else {
                        continue;
                    };
                    let Some(val) = convert_to_si(raw, &fc.unit) else {
                        register_unconverted_unit(&fc.unit, &fc.name);
                        continue;
                    };
                    if val.is_finite() {
                        out.push((epoch, val));
                    }
                }
            }
            Extract::Hapi(pairs) => {
                let JsonVal::Obj(root) = j else {
                    continue;
                };
                let Some(JsonVal::Arr(data)) = root.get("data") else {
                    continue;
                };
                let mut col: HashMap<String, usize> = HashMap::new();
                let mut fill_of: HashMap<String, f64> = HashMap::new();
                if let Some(JsonVal::Arr(params)) = root.get("parameters") {
                    for (i, p) in params.iter().enumerate() {
                        if let JsonVal::Obj(po) = p {
                            if let Some(JsonVal::Str(nn)) = po.get("name") {
                                col.insert(nn.clone(), i);
                                if let Some(fv) = po.get("fill").and_then(scalar_of) {
                                    fill_of.insert(nn.clone(), fv);
                                }
                            }
                        }
                    }
                }
                if col.is_empty() {
                    for (i, (param, _)) in pairs.iter().enumerate() {
                        col.insert(param.clone(), i + 1);
                    }
                }
                for row in data {
                    let JsonVal::Arr(cells) = row else {
                        continue;
                    };
                    let Some(epoch) = series_epoch_of(row, lsk) else {
                        continue;
                    };
                    for (param, name) in pairs {
                        let (base, comp) = match param.rfind('.') {
                            Some(dot) if param[dot + 1..].chars().all(|c| c.is_ascii_digit()) => {
                                (&param[..dot], param[dot + 1..].parse::<usize>().ok())
                            }
                            _ => (param.as_str(), None),
                        };
                        let Some(&idx) = col.get(base) else {
                            continue;
                        };
                        let v = match comp {
                            Some(i) => cells.get(idx).and_then(|cell| {
                                if let JsonVal::Arr(a) = cell {
                                    a.get(i).and_then(scalar_of)
                                } else {
                                    None
                                }
                            }),
                            None => cells.get(idx).and_then(scalar_of),
                        };
                        let Some(raw) = v else {
                            continue;
                        };
                        if fill_of.get(base).map_or(false, |&f| raw == f) {
                            continue;
                        }
                        let Some(fc) = src.extracts.iter().find_map(|e| match e {
                            Extract::Field(fc) if fc.name == *name => Some(fc),
                            _ => None,
                        }) else {
                            continue;
                        };
                        let Some(val) = convert_to_si(raw, &fc.unit) else {
                            register_unconverted_unit(&fc.unit, &fc.name);
                            continue;
                        };
                        if val.is_finite() {
                            out.push((epoch, val));
                        }
                    }
                }
            }
            _ => {}
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    static ANOMALY_TEST_GATE: std::sync::Mutex<()> = std::sync::Mutex::new(());

    fn field_fixture(name: &str, tau: f64) -> FieldConfig {
        FieldConfig {
            key: name.into(),
            name: name.into(),
            kernel: 0,
            force: 0,
            tau,
            absorption: 0.0,
            advection: 0.0,
            unit: String::new(),
            fold: None,
        }
    }

    fn source_fixture(format: &str, extracts: Vec<Extract>) -> SourceConfig {
        SourceConfig {
            ttl: 3600,
            url: "https://example.com/x".into(),
            frame: Frame::Barycenter {
                body_name: "sun".into(),
                scale: 1.0,
            },
            format: format.into(),
            extracts,
            headers: vec![],
            post_body: None,
            target: None,
            catalog: None,
            max_freq: None,
            min_freq: None,
            body: None,
            stations_url: None,
            stations_path: String::new(),
            stations_lat: String::new(),
            stations_lon: String::new(),
            stations_id: String::new(),
            flux_from_mag: None,
            abs_mag_from: None,
            catalog_epoch: None,
            repeat_ra_bins: 0,
            fanout_cap: 0,
            stations_flatten: String::new(),
            stations_filter: None,
            fanout_delay: 0,
        }
    }

    fn fixture_lsk() -> LeapSeconds {
        LeapSeconds {
            delta_t_a: 32.184,
            deltas: vec![(37.0, 1483228800.0)],
        }
    }

    #[test]
    fn test_convert_to_si() {
        let close = |a: Option<f64>, b: f64| {
            let a = a.unwrap_or_else(|| panic!("conversion returned None"));
            assert!((a - b).abs() < 1e-9 * b.abs().max(1e-12), "{a} vs {b}");
        };
        close(convert_to_si(5.0, "km"), 5000.0);
        close(convert_to_si(1000.0, "hPa"), 100000.0);
        close(convert_to_si(30.0, "nT"), 30e-9);
        close(convert_to_si(20.0, "C"), 293.15);
        close(convert_to_si(2.0, "mgal"), 2e-5);
        close(convert_to_si(3.0, "ppm"), 3e-6);
        close(convert_to_si(72.0, "km/h"), 20.0);
        close(convert_to_si(7.0, "m"), 7.0);
        close(convert_to_si(-1.0, "km"), -1000.0);
        close(convert_to_si(90.0, "deg"), std::f64::consts::PI / 2.0);
        close(convert_to_si(1.0, "M_sun"), 1.98847e30);
        close(convert_to_si(1.0, "MW"), 1e6);
        close(convert_to_si(2.0, "d"), 172800.0);
        close(convert_to_si(1.0, "uatm"), 0.101325);
        close(convert_to_si(1.0, "cfs"), 0.028316846592);
        close(convert_to_si(1.0, "%"), 0.01);
        close(convert_to_si(1.0, "pc/cm3"), 3.085677581e22);
        close(convert_to_si(1.0, "knot"), 0.514444);
        close(convert_to_si(1.0, "Jy_km/s"), 1e-23);
        close(convert_to_si(1.0, "Crab"), 2.4e-14);
        close(convert_to_si(4.4, "logg"), 251.188643150958);
        close(convert_to_si(7.2, "Mw"), 10.0f64.powf(1.5 * 7.2 + 9.1));
        close(convert_to_si(334.0, "cpm"), 1e-6 / 3600.0);
        close(convert_to_si(1.0, "decibar"), 1e4);
        close(convert_to_si(1.0, "mV/m"), 1e-3);
        close(convert_to_si(1.0, "nPa"), 1e-9);
        close(convert_to_si(1.0, "sfu"), 1e-22);
        assert!(convert_to_si(9.0, "weird").is_none());
        assert!(convert_to_si(7.2, "M").is_none());
        assert!(convert_to_si(5.0, "mag").is_none());
        assert!(convert_to_si(1.0, "dex").is_none());
        assert!(convert_to_si(0.0, "").is_some());
    }

    #[test]
    fn test_anomaly_reporter() {
        let _gate = ANOMALY_TEST_GATE.lock();
        ANOMALY_COLLECT.with(|c| c.set(true));
        report_anomaly(
            "API Unreachable",
            "https://example.org/x",
            "fetch returned void",
        );
        report_anomaly("Malformed Data", "https://example.org/y", "JSON parse void");
        let _ = parse_sources(
            "url https://example.org/em\nttl 60\nformat json\nfield mag mag inverse-square em K 60 0 0\n",
        );
        let _ = parse_sources(
            "url https://example.org/syn\nttl 60\nformat json\non earth\nfield p p inverse-square electroweak K 60 0 0\n",
        );
        let anomalies = take_anomalies();
        assert_eq!(anomalies.len(), 5);
        assert_eq!(anomalies[0].category, "API Unreachable");
        assert_eq!(anomalies[0].url, "https://example.org/x");
        assert_eq!(anomalies[1].category, "Malformed Data");
        assert_eq!(anomalies[2].category, "Physics Mismatch");
        assert_eq!(anomalies[3].category, "Invalid Syntax");
        assert_eq!(anomalies[4].category, "Invalid Syntax");
        let body = anomaly_issue_body(&anomalies);
        assert!(body.starts_with("| Category | URL | Details |\n|---|---|---|\n"));
        assert!(body.contains("| API Unreachable | https://example.org/x | fetch returned void |"));
        assert!(body.contains("| Malformed Data | https://example.org/y | JSON parse void |"));
        assert!(body.contains(
            "| Physics Mismatch | https://example.org/em | field mag: unit \"K\" not in force registry |"
        ));
        assert!(body.contains(
            "| Invalid Syntax | https://example.org/syn | on needs <body> <lat> <lon> [alt]: on earth |"
        ));
        assert!(take_anomalies().is_empty());
        ANOMALY_COLLECT.with(|c| c.set(false));
    }

    #[test]
    fn test_allowed_units_for_force() {
        assert!(allowed_units_for_force(0).contains(&"nt"));
        assert!(allowed_units_for_force(5).contains(&"k"));
        assert!(allowed_units_for_force(7).contains(&"m/s"));
        assert!(allowed_units_for_force(9).is_empty());
        assert!(allowed_units_for_force(0).contains(&"mag"));
        assert!(allowed_units_for_force(0).contains(&"jy_km/s"));
        assert!(allowed_units_for_force(1).contains(&"logg"));
        assert!(allowed_units_for_force(1).contains(&"m_sun"));
        assert!(allowed_units_for_force(3).contains(&"mw"));
        assert!(allowed_units_for_force(5).contains(&"mw"));
        assert!(allowed_units_for_force(6).contains(&"du"));
        assert!(allowed_units_for_force(6).contains(&"ug/m3"));
        assert!(allowed_units_for_force(7).contains(&"cfs"));
        assert!(allowed_units_for_force(8).contains(&"ua/m2"));
        assert!(!allowed_units_for_force(0).contains(&"k"));
        assert!(!allowed_units_for_force(2).contains(&"c"));
        assert!(!allowed_units_for_force(6).contains(&"kt"));
    }

    #[test]
    fn test_normalize_unit() {
        assert_eq!(normalize_unit("nT"), "nt");
        assert_eq!(normalize_unit(" M_sun "), "m_sun");
        assert_eq!(normalize_unit("µg/m3"), "ug/m3");
        assert_eq!(normalize_unit("m/s²"), "m/s2");
        assert_eq!(normalize_unit("K"), "k");
        assert_eq!(normalize_unit("Pa"), "pa");
    }

    #[test]
    fn test_hapi_fill_skipped_and_component_index() {
        let json = r#"{
            "parameters": [
                {"name": "Time"},
                {"name": "VEC", "fill": "-1.0e31"},
                {"name": "SCAL", "fill": "-1.0e31"}
            ],
            "data": [
                ["2026-08-18T00:00:00Z", [-1.0e31, -1.0e31, -1.0e31], 9.0],
                ["2026-08-18T01:00:00Z", [1.1, 2.2, 3.3], 7.5]
            ]
        }"#;
        let src = source_fixture(
            "json",
            vec![
                Extract::Field(field_fixture("x", 3600.0)),
                Extract::Field(field_fixture("y", 3600.0)),
                Extract::Field(field_fixture("z", 3600.0)),
                Extract::Field(field_fixture("s", 3600.0)),
                Extract::Hapi(vec![
                    ("VEC.0".into(), "x".into()),
                    ("VEC.1".into(), "y".into()),
                    ("VEC.2".into(), "z".into()),
                    ("SCAL".into(), "s".into()),
                ]),
            ],
        );
        match extract(&src, json, 8.0e8, &fixture_lsk()) {
            ExtractResult::Measurements(channels) => {
                assert_eq!(channels.len(), 4);
                let vals: Vec<(&str, f64)> = channels
                    .iter()
                    .map(|(c, fc)| (fc.name.as_str(), c.value))
                    .collect();
                assert!(vals.contains(&("x", 1.1)));
                assert!(vals.contains(&("y", 2.2)));
                assert!(vals.contains(&("z", 3.3)));
                assert!(vals.contains(&("s", 7.5)));
            }
            _ => panic!("expected measurements"),
        }
        let fill_only = r#"{
            "parameters": [{"name": "Time"}, {"name": "SCAL", "fill": "-1.0e31"}],
            "data": [["2026-08-18T01:00:00Z", -1.0e31]]
        }"#;
        let src_fill = source_fixture(
            "json",
            vec![
                Extract::Field(field_fixture("s", 3600.0)),
                Extract::Hapi(vec![("SCAL".into(), "s".into())]),
            ],
        );
        match extract(&src_fill, fill_only, 8.0e8, &fixture_lsk()) {
            ExtractResult::Measurements(channels) => {
                assert!(channels.is_empty(), "fill must not be ingested");
            }
            _ => panic!("expected measurements"),
        }
    }
}
