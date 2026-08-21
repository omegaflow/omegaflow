use crate::dastcom::AsteroidRec;
use crate::force::{force_id_of, kernel_id_for_force};
use crate::inflate::unzip;
pub use crate::json::{jnum, jpath, jpath_val, json_num, jstr, parse_json, scalar_of, JsonVal};
pub use crate::lsk::LeapSeconds;
use std::collections::HashMap;
use std::process::Command;
use std::sync::{Arc, OnceLock};

pub mod goes;
pub mod omni2;

fn series_parse_bin(format: &str, bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    match format {
        "rpw_efield" => crate::rpw::parse_bin(bytes),
        "goes_xrs" => goes::parse_bin(bytes),
        "omni2_serie" => omni2::parse_bin(bytes),
        _ => None,
    }
}

fn series_component_name(format: &str, comp: u32) -> Option<&'static str> {
    match format {
        "rpw_efield" => match comp {
            crate::rpw::COMP_EY => Some("rpw_e_y"),
            crate::rpw::COMP_EZ => Some("rpw_e_z"),
            _ => None,
        },
        "goes_xrs" => match comp {
            goes::COMP_XRSA => Some("goes_xrs_xrsa"),
            goes::COMP_XRSB => Some("goes_xrs_xrsb"),
            _ => None,
        },
        "omni2_serie" => match comp {
            omni2::COMP_V1800 => Some("omni_solarwind_flow_speed_kms"),
            omni2::COMP_N1800 => Some("omni_solarwind_density_percc"),
            omni2::COMP_T1800 => Some("omni_solarwind_temp_k"),
            omni2::COMP_BX => Some("omni_imf_bx_gse_nt"),
            omni2::COMP_BY => Some("omni_imf_by_gsm_nt"),
            omni2::COMP_BZ => Some("omni_imf_bz_gsm_nt"),
            omni2::COMP_PRESSURE => Some("omni_solarwind_pressure_npa"),
            _ => None,
        },
        _ => None,
    }
}

pub type CellKey = (i64, i64, i64);

pub struct SpatialHash {
    pub cell_size: f64,
    pub anchor_vmax: f64,
    pub anchor_amax: f64,
    pub epoch_min: f64,
    pub cell_lo: CellKey,
    pub cell_hi: CellKey,
    pub cells: HashMap<CellKey, Vec<Sample>>,
    pub unbounded: Vec<Sample>,
}

#[derive(Clone)]
pub struct SpectralHash {
    pub name: String,
    pub motion: Motion,
    pub epoch: f64,
    pub ttl: f64,
    pub tau: f64,
    pub kernel_id: f64,
    pub force_type: f64,
    pub absorption: f64,
    pub advection: f64,
    pub bins: Vec<(f64, f64, f64)>,
}

pub struct Buffer {
    pub cache: SpatialHash,
    pub eph: Arc<HashMap<String, BodyEphemeris>>,
    pub curves: Option<Arc<CurveSet>>,
    pub spectral: Vec<SpectralHash>,
}
#[derive(Clone)]
pub struct StarRec {
    pub ra_deg: f64,
    pub dec_deg: f64,
    pub pm_ra_masyr: f64,
    pub pm_de_masyr: f64,
    pub plx_mas: f64,
    pub flux: f64,
    pub mag: f64,
    pub tau: f64,
    pub color_index: f64,
    pub rv_m_s: f64,
}
pub struct CurveStar {
    pub ra_deg: f64,
    pub dec_deg: f64,
    pub plx_mas: f64,
    pub cadence: f64,
    pub samples: Vec<(f64, f32)>,
}
pub struct CurveSet {
    pub stars: Vec<CurveStar>,
}

pub trait Radiator: Send + Sync {
    fn accept(&mut self, field: Arc<Buffer>);
}
pub const Φ: f64 = 1.618033988749895;
const FETCH_BUDGET: usize = 1 << 3;
const FETCH_VOID_CAP: u32 = 1 << 2;
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
    Kepler {
        rec: Arc<AsteroidRec>,
    },
    Spherical {
        rec: Arc<StarRec>,
    },
}

#[derive(Clone)]
pub enum SampleSource {
    Source(u32),
    Sensor,
    Ephemeris,
}

#[derive(Clone)]
pub struct Sample {
    pub source: SampleSource,
    pub epoch: f64,
    pub ttl: f64,
    pub extent: f64,
    pub tau: f64,
    pub kernel_id: f64,
    pub force_type: f64,
    pub absorption: f64,
    pub advection: f64,
    pub anchor_vmax: f64,
    pub anchor_amax: f64,
    pub anchor_p0: [f64; 3],
    pub motion: Motion,
    pub val: f64,
    pub name: String,
    pub z: f64,
    pub freq: f64,
    pub bin_width: f64,
    pub color_index: f64,
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
    First(FieldConfig, Option<(String, String)>),
    Last(FieldConfig, Option<(String, String)>),
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
        | "rad" | "m/s" | "m/s2" | "m/s²" | "j" | "v/m" | "s/m" | "ntu" | "1" => Some(value),
        "wm2_1au" => Some(value * 1.495978707e11 * 1.495978707e11),
        "1e-4w/m2" => Some(value * 1e-4),
        "pfu" => Some(value * 1e4),
        "pfu/mev" => Some(value * 6.241509074e16),
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
            "unit \"{}\" unconverted — SI absent; samples like \"{}\" stay unmanifested (pending curation)",
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
            "erg/cm2", "crab", "cpm", "e10j", "kt_tnt", "sfu", "1/cm3", "wm2_1au", "1e-4w/m2", "1",
            "pfu", "pfu/mev",
        ],
        1 => &[
            "m/s2", "gal", "mgal", "kg", "m_sun", "m_earth", "au", "pc", "t", "nt", "m", "r_earth",
            "logg", "1",
        ],
        2 => &["pa", "hpa", "m", "mm", "hz"],
        3 => &["m", "mm", "km", "m/s2", "gal", "pa", "hz", "mw"],
        4 => &["m", "mm", "cm", "km", "pa", "m/s", "mw"],
        5 => &["k", "c", "w/m2", "w", "j", "mw"],
        6 => &[
            "ppm", "ppb", "mg/m3", "ug/m3", "mg/kg", "psu", "ntu", "%", "pct", "hpa", "uatm", "du",
            "cm-3", "1/cm3",
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

pub type SampleRecord = (
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
    let idx = e.granules.partition_point(|g| g.t0_jd < jd);
    for i in [idx.saturating_sub(1), idx] {
        if i >= e.granules.len() {
            continue;
        }
        let g = &e.granules[i];
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
        let idx = e.rotation_matrices.partition_point(|(t, _)| *t < jd);
        let mut best: Option<&[f64; 9]> = None;
        let mut best_d = f64::INFINITY;
        let lo = idx.saturating_sub(2);
        let hi = (idx + 2).min(e.rotation_matrices.len().saturating_sub(1));
        for i in lo..=hi {
            let (t, m) = &e.rotation_matrices[i];
            if !t.is_finite() {
                continue;
            }
            let d = (jd - t).abs();
            if d < best_d {
                best_d = d;
                best = Some(m);
            }
        }
        let rot_m = best?;
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

fn finite_pos(p: [f64; 3]) -> Option<[f64; 3]> {
    if p[0].is_finite() && p[1].is_finite() && p[2].is_finite() {
        Some(p)
    } else {
        None
    }
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
            Motion::Kepler { rec } => {
                let t_jd = t / 86400.0 + J2000_EPOCH;
                match state_at(rec, t_jd) {
                    Some((p, _)) => finite_pos(p),
                    None => None,
                }
            }
            Motion::Spherical { rec } => finite_pos(star_position_at(rec, t).0),
        }
    }

    pub fn anchor_body(&self) -> Option<&str> {
        match self {
            Motion::Surface { body_name, .. } | Motion::Barycenter { body_name, .. } => {
                Some(body_name)
            }
            Motion::Linear { .. } | Motion::Kepler { .. } | Motion::Spherical { .. } => None,
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

fn row_matches(el: &JsonVal, fk: &str, fv: &str) -> bool {
    let JsonVal::Obj(map) = el else {
        return false;
    };
    match map.get(fk) {
        Some(JsonVal::Str(s)) => s == fv,
        Some(JsonVal::Num(n)) => fv.parse::<f64>().map_or(false, |f| f == *n),
        _ => false,
    }
}

fn row_value(el: &JsonVal, key: &str) -> Option<f64> {
    match el {
        JsonVal::Obj(o) => o.get(key).and_then(scalar_of),
        other => scalar_of(other),
    }
}

pub fn jfirst_where(json: &JsonVal, key: &str, filter: Option<&(String, String)>) -> Option<f64> {
    let Some((fk, fv)) = filter else {
        return jfirst(json, key);
    };
    if let Some((prefix, final_key)) = key.rsplit_once('.') {
        let parent = if prefix.is_empty() {
            json
        } else {
            jpath_val(json, prefix)?
        };
        let JsonVal::Arr(arr) = parent else {
            return None;
        };
        return arr
            .iter()
            .find(|v| row_matches(v, fk, fv))
            .and_then(|v| row_value(v, final_key));
    }
    let JsonVal::Arr(arr) = json else {
        return None;
    };
    arr.iter()
        .find(|v| row_matches(v, fk, fv))
        .and_then(|v| row_value(v, key))
}

pub fn jlast_where(json: &JsonVal, key: &str, filter: Option<&(String, String)>) -> Option<f64> {
    let Some((fk, fv)) = filter else {
        return jlast(json, key);
    };
    if let Some((prefix, final_key)) = key.rsplit_once('.') {
        let parent = if prefix.is_empty() {
            json
        } else {
            jpath_val(json, prefix)?
        };
        let JsonVal::Arr(arr) = parent else {
            return None;
        };
        return arr
            .iter()
            .rev()
            .find(|v| row_matches(v, fk, fv))
            .and_then(|v| row_value(v, final_key));
    }
    let JsonVal::Arr(arr) = json else {
        return None;
    };
    arr.iter()
        .rev()
        .find(|v| row_matches(v, fk, fv))
        .and_then(|v| row_value(v, key))
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

pub fn extract_fields(ext: &Extract) -> Vec<FieldConfig> {
    match ext {
        Extract::Map { fields, .. }
        | Extract::CelestialMap { fields, .. }
        | Extract::Rows { fields, .. }
        | Extract::Flatten { fields, .. }
        | Extract::CmrPolygon { fields, .. }
        | Extract::CelestialPolygon { fields, .. }
        | Extract::KeplerMap { fields, .. }
        | Extract::ProfileMap { fields, .. } => fields.clone(),
        Extract::Field(fc)
        | Extract::First(fc, _)
        | Extract::Last(fc, _)
        | Extract::Count(fc)
        | Extract::LastRow(fc)
        | Extract::ObjLast(fc)
        | Extract::Path(fc)
        | Extract::Deep(fc)
        | Extract::Regex(fc) => vec![fc.clone()],
        Extract::GeojsonEvents {
            outputs,
            tau,
            absorption,
            advection,
            ..
        } => {
            if outputs.len() < 2 {
                return Vec::new();
            }
            vec![
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
            ]
        }
        _ => Vec::new(),
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
        .arg("-g")
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
        .arg("-g")
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
        .arg("-g")
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
                let filter = match parse_where(&parts) {
                    Ok(f) => f,
                    Err(()) => continue,
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
                cur_extracts.push(Extract::First(fc, filter));
            }
            "last" if parts.len() >= 9 => {
                let (k, f, tau, absorption, advection) = match parse_field_config(&parts) {
                    Some(v) => v,
                    None => continue,
                };
                let filter = match parse_where(&parts) {
                    Ok(f) => f,
                    Err(()) => continue,
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
                cur_extracts.push(Extract::Last(fc, filter));
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
                    unit: parts[5].to_string(),
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
                    unit: parts[5].to_string(),
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
                    unit: parts[5].to_string(),
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
                    unit: parts[5].to_string(),
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
                    unit: parts[5].to_string(),
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
                if parts.len() >= 10 && parts[9] == "where" {
                    eprintln!(
                        "where refused at {}: the row filter lives on first/last, not on field",
                        parts[1]
                    );
                    continue;
                }
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
            "tname" if parts.len() >= 2 => {}
            "tra" if parts.len() >= 2 => {}
            "tdec" if parts.len() >= 2 => {}
            "tdist" if parts.len() >= 2 => {}
            "tdist_scale" if parts.len() >= 2 => {}
            "ta" if parts.len() >= 2 => {}
            "te" if parts.len() >= 2 => {}
            "ti" if parts.len() >= 2 => {}
            "tw" if parts.len() >= 2 => {}
            "ttranmid" if parts.len() >= 2 => {}
            "tperiod" if parts.len() >= 2 => {}
            "trp" if parts.len() >= 2 => {}
            "trs" if parts.len() >= 2 => {}
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
    pub omega_g: Option<(f64, f64)>,
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
            return None;
        }
        let stype = u32::from_le_bytes(data[pos..pos + 4].try_into().ok()?);
        pos += 4;
        let gcount = u32::from_le_bytes(data[pos..pos + 4].try_into().ok()?) as usize;
        pos += 4;
        let degree = u32::from_le_bytes(data[pos..pos + 4].try_into().ok()?) as usize;
        pos += 4;
        pos += 4;
        if stype == 1 {
            if pos + 96 > data.len() {
                return None;
            }
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
                omega_g: None,
            });
            pos += if version == 0x02 { 104 } else { 96 };
            continue;
        }
        if stype == 7 {
            let n = degree + 1;
            let gs = 2 + 3 * n;
            for _ in 0..gcount {
                if pos + gs * 8 > data.len() {
                    return None;
                }
                let f = |i: usize| -> f64 {
                    let mut buf = [0u8; 8];
                    buf.copy_from_slice(&data[pos + i * 8..pos + i * 8 + 8]);
                    f64::from_le_bytes(buf)
                };
                let value = f(0);
                let sigma = f(1);
                if value > 0.0 && value.is_finite() && sigma.is_finite() {
                    if let Some(ref mut p) = props {
                        p.omega_g = Some((value, sigma));
                    }
                }
                pos += gs * 8;
            }
            continue;
        }
        if stype == 2 {
            if pos + 40 > data.len() {
                return None;
            }
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
                    return None;
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
                    return None;
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
            let skip = gcount * (2 + 3 * (degree + 1)) * 8;
            if pos + skip > data.len() {
                return None;
            }
            pos += skip;
            continue;
        }
        let n = degree + 1;
        let gs = 2 + 3 * n;
        for _ in 0..gcount {
            if pos + gs * 8 > data.len() {
                return None;
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
            Extract::First(fc, filter) => {
                if let Some(ref j) = parsed_json {
                    if let Some(v) = jfirst_where(j, &fc.key, filter.as_ref()) {
                        extracted.insert(fc.name.clone(), v);
                    }
                }
            }
            Extract::Last(fc, filter) => {
                if let Some(ref j) = parsed_json {
                    if let Some(v) = jlast_where(j, &fc.key, filter.as_ref()) {
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
            Extract::Alerce(_) => {}
        }
    }
    if !extracted.is_empty() {
        for (name, val) in &extracted {
            let fc = effective_extracts.iter().find_map(|ext| match ext {
                Extract::Field(fc)
                | Extract::First(fc, _)
                | Extract::Last(fc, _)
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

fn parse_where(parts: &[&str]) -> Result<Option<(String, String)>, ()> {
    if parts.len() < 10 || parts[9] != "where" {
        return Ok(None);
    }
    if parts.len() != 12 {
        eprintln!(
            "where refused at {}: the filter clause carries exactly `where <key> <value>`",
            parts.get(1).copied().unwrap_or("?")
        );
        return Err(());
    }
    Ok(Some((parts[10].to_string(), parts[11].to_string())))
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
            Extract::First(fc, filter) => {
                let JsonVal::Arr(elements) = j else {
                    continue;
                };
                for el in elements {
                    if let Some((fk, fv)) = filter {
                        if !row_matches(el, fk, fv) {
                            continue;
                        }
                    }
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
            Extract::Last(fc, filter) => {
                let JsonVal::Arr(elements) = j else {
                    continue;
                };
                for el in elements {
                    if let Some((fk, fv)) = filter {
                        if !row_matches(el, fk, fv) {
                            continue;
                        }
                    }
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

    fn euvs_json() -> &'static str {
        r#"[
            {"time_tag": "2026-08-20T08:50:00Z", "line": "304", "value": 1.1e-4},
            {"time_tag": "2026-08-20T08:50:00Z", "line": "284", "value": 2.2e-4},
            {"time_tag": "2026-08-20T08:51:00Z", "line": "304", "value": 3.3e-4},
            {"time_tag": "2026-08-20T08:51:00Z", "line": "284", "value": 4.4e-4},
            {"time_tag": "2026-08-20T08:51:00Z", "line": "mgii_index", "value": 0.278}
        ]"#
    }

    fn last_where_fixture(name: &str, key: &str, fk: &str, fv: &str) -> Extract {
        let mut fc = field_fixture(name, 60.0);
        fc.key = key.into();
        Extract::Last(fc, Some((fk.into(), fv.into())))
    }

    #[test]
    fn test_last_where_picks_matching_row() {
        let src = source_fixture(
            "json",
            vec![
                last_where_fixture("euv304", "value", "line", "304"),
                last_where_fixture("euv284", "value", "line", "284"),
            ],
        );
        match extract(&src, euvs_json(), 8.0e8, &fixture_lsk()) {
            ExtractResult::Measurements(channels) => {
                let vals: HashMap<&str, f64> = channels
                    .iter()
                    .map(|(c, fc)| (fc.name.as_str(), c.value))
                    .collect();
                assert_eq!(vals.get("euv304"), Some(&3.3e-4));
                assert_eq!(vals.get("euv284"), Some(&4.4e-4));
            }
            _ => panic!("expected measurements"),
        }
    }

    #[test]
    fn test_last_where_no_match_absent() {
        let src = source_fixture(
            "json",
            vec![last_where_fixture("euv999", "value", "line", "999")],
        );
        match extract(&src, euvs_json(), 8.0e8, &fixture_lsk()) {
            ExtractResult::Measurements(channels) => {
                assert!(channels.is_empty(), "no matching row → fehlt, never 0.0")
            }
            _ => panic!("expected measurements"),
        }
    }

    #[test]
    fn test_first_where_picks_first_matching_row() {
        let mut fc = field_fixture("euv304_first", 60.0);
        fc.key = "value".into();
        let src = source_fixture(
            "json",
            vec![Extract::First(fc, Some(("line".into(), "304".into())))],
        );
        match extract(&src, euvs_json(), 8.0e8, &fixture_lsk()) {
            ExtractResult::Measurements(channels) => {
                assert_eq!(channels.len(), 1);
                assert_eq!(channels[0].0.value, 1.1e-4);
            }
            _ => panic!("expected measurements"),
        }
    }

    #[test]
    fn test_last_where_numeric_filter_value() {
        let json = r#"[
            {"time_tag": "2026-08-20T08:50:00Z", "satellite": 18, "flux": 7.0},
            {"time_tag": "2026-08-20T08:51:00Z", "satellite": 19, "flux": 9.0}
        ]"#;
        let src = source_fixture(
            "json",
            vec![last_where_fixture("g18", "flux", "satellite", "18")],
        );
        match extract(&src, json, 8.0e8, &fixture_lsk()) {
            ExtractResult::Measurements(channels) => {
                assert_eq!(channels.len(), 1);
                assert_eq!(channels[0].0.value, 7.0);
            }
            _ => panic!("expected measurements"),
        }
    }

    #[test]
    fn test_extract_series_where_filters_rows() {
        let src = source_fixture(
            "json",
            vec![last_where_fixture("euv304", "value", "line", "304")],
        );
        let series = extract_series(&src, euvs_json(), &fixture_lsk());
        assert_eq!(series.len(), 2);
        assert_eq!(series[0].1, 1.1e-4);
        assert_eq!(series[1].1, 3.3e-4);
    }

    #[test]
    fn test_parse_where_clause() {
        let content = "url https://example.com/euvs.json\nttl 600\nat sun\nlast value euv304 inverse-square em W/m2 60.0 0.0 0.0 where line 304\n";
        let sources = parse_sources(content);
        assert_eq!(sources.len(), 1);
        match &sources[0].extracts[0] {
            Extract::Last(fc, Some((fk, fv))) => {
                assert_eq!(fc.name, "euv304");
                assert_eq!(fk, "line");
                assert_eq!(fv, "304");
            }
            _ => panic!("expected filtered last extract"),
        }
    }

    #[test]
    fn test_parse_where_malformed_refused() {
        let content = "url https://example.com/euvs.json\nttl 600\nat sun\nlast value euv304 inverse-square em W/m2 60.0 0.0 0.0 where line\n";
        let sources = parse_sources(content);
        assert_eq!(sources.len(), 1);
        assert!(
            sources[0].extracts.is_empty(),
            "malformed where must refuse the line loudly"
        );
    }

    #[test]
    fn test_parse_where_refused_on_field() {
        let content = "url https://example.com/euvs.json\nttl 600\nat sun\nfield value euv304 inverse-square em W/m2 60.0 0.0 0.0 where line 304\n";
        let sources = parse_sources(content);
        assert_eq!(sources.len(), 1);
        assert!(sources[0].extracts.is_empty());
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
    use std::collections::HashMap;

    fn full_fixture_lsk() -> super::LeapSeconds {
        super::LeapSeconds {
            delta_t_a: 32.184,
            deltas: vec![
                (10.0, 63072000.0),
                (11.0, 78796800.0),
                (12.0, 94694400.0),
                (13.0, 126230400.0),
                (14.0, 157766400.0),
                (15.0, 189302400.0),
                (16.0, 220924800.0),
                (17.0, 252460800.0),
                (18.0, 283996800.0),
                (19.0, 315532800.0),
                (20.0, 362793600.0),
                (21.0, 394329600.0),
                (22.0, 425865600.0),
                (23.0, 489024000.0),
                (24.0, 567993600.0),
                (25.0, 631152000.0),
                (26.0, 662688000.0),
                (27.0, 709948800.0),
                (28.0, 741484800.0),
                (29.0, 773020800.0),
                (30.0, 820454400.0),
                (31.0, 867715200.0),
                (32.0, 915148800.0),
                (33.0, 1136073600.0),
                (34.0, 1230768000.0),
                (35.0, 1341100800.0),
                (36.0, 1435708800.0),
                (37.0, 1483228800.0),
            ],
        }
    }

    #[test]
    fn test_parse_json_skips_jina_header() {
        let s = "Title: \n\n\nURL Source: http://api.wheretheiss.at/v1/satellites/25544\n\n\nMarkdown Content:\n{\"name\":\"iss\",\"id\":25544,\"latitude\":-39.79}";
        let v = parse_json(s).unwrap();
        let obj = match v {
            super::JsonVal::Obj(m) => m,
            other => panic!("root is {:?}", other),
        };
        assert!(matches!(obj.get("name"), Some(super::JsonVal::Str(s)) if s == "iss"));
        assert!(
            matches!(obj.get("id"), Some(super::JsonVal::Num(n)) if (n - 25544.0).abs() < 1e-9)
        );
    }

    #[test]
    fn test_render_source_url_substitutions() {
        let src = SourceConfig {
            ttl: 100,
            url: "https://example.com?sstr={target}&from={catalog}".into(),
            frame: super::Frame::Surface {
                body_name: "body_test".into(),
                lat: 0.0,
                lon: 0.0,
                alt: 0.0,
            },
            format: "json".into(),
            extracts: vec![],
            headers: vec![],
            post_body: None,
            target: Some("Ceres".into()),
            catalog: Some("fp_psc".into()),
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
        };
        let fixture_lsk = super::LeapSeconds {
            delta_t_a: 32.184,
            deltas: vec![(37.0, 1483228800.0)],
        };
        let url = render_source_url(
            &src,
            0.0,
            0.0,
            0.0,
            8.0e8,
            1000.0,
            &HashMap::new(),
            &HashMap::new(),
            &fixture_lsk,
        );
        let url = url.unwrap();
        assert!(url.contains("Ceres"));
        assert!(url.contains("fp_psc"));
        assert!(!url.contains("{target}"));
        assert!(!url.contains("{catalog}"));
    }

    #[test]
    fn test_post_body_rendering() {
        let src = SourceConfig {
            ttl: 100,
            url: "https://earth-search.aws.element84.com/v0/search".into(),
            frame: super::Frame::Surface { body_name: "body_test".into(), lat: 0.0, lon: 0.0, alt: 0.0 },
            format: "json".into(),
            extracts: vec![],
            headers: vec![("Content-Type".into(), "application/stac+json".into())],
            post_body: Some("{\"bbox\":[{lon_min},{lat_min},{lon_max},{lat_max}],\"datetime\":\"{today}/{today}\"}".into()),
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
        };
        let fixture_lsk = super::LeapSeconds {
            delta_t_a: 32.184,
            deltas: vec![(37.0, 1483228800.0)],
        };
        let body = render_source_body(
            &src,
            0.0,
            0.0,
            0.0,
            8.0e8,
            100000.0,
            &HashMap::new(),
            &fixture_lsk,
        );
        assert!(body.is_some());
        let b = body.unwrap();
        assert!(b.contains("bbox"));
        assert!(b.contains("{lon_min}"));
        assert!(!b.contains("{today}"));
    }

    #[test]
    fn test_csv_to_json_tns_shape() {
        let csv = "2026-08-13 00:00:00 - 23:59:59\n\"objid\",\"ra\",\"declination\",\"redshift\",\"discoverymag\"\n\"1\",\"89.8\",\"53.6\",\"0.027\",\"19.8\"\n\"2\",\"35.0\",\"-24.4\",\"\",\"19.4\"\n";
        let j = csv_to_json(csv).unwrap();
        let arr = match j {
            JsonVal::Arr(a) => a,
            _ => panic!("expected array"),
        };
        assert_eq!(arr.len(), 2);
        match &arr[0] {
            JsonVal::Obj(m) => {
                assert_eq!(scalar_of(m.get("ra").unwrap()), Some(89.8));
                assert_eq!(scalar_of(m.get("redshift").unwrap()), Some(0.027));
            }
            _ => panic!("expected object"),
        }
        match &arr[1] {
            JsonVal::Obj(m) => {
                assert_eq!(scalar_of(m.get("redshift").unwrap()), None);
            }
            _ => panic!("expected object"),
        }
    }

    #[test]
    fn test_celestial_map_redshift_distance() {
        let src = SourceConfig {
            ttl: 100,
            url: "https://example.com/x".into(),
            frame: Frame::Barycenter {
                body_name: "sun".into(),
                scale: 1.0,
            },
            format: "json".into(),
            extracts: vec![Extract::CelestialMap {
                arr_path: ".".into(),
                ra_key: "ra".into(),
                dec_key: "declination".into(),
                dist_key: String::new(),
                dist_scale: 1.0,
                plx_key: String::new(),
                z_key: "redshift".into(),
                pmra_key: String::new(),
                pmdec_key: String::new(),
                rv_key: String::new(),
                rv_scale: 1.0,
                epoch_key: String::new(),
                fields: vec![FieldConfig {
                    key: "discoverymag".into(),
                    name: "tns_transient_flux".into(),
                    kernel: 0,
                    force: 0,
                    tau: 3600.0,
                    absorption: 0.0,
                    advection: 0.0,
                    unit: String::new(),
                    fold: None,
                }],
                tau_key: String::new(),
            }],
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
            abs_mag_from: Some("tns_transient_flux".into()),
            catalog_epoch: None,
            repeat_ra_bins: 0,
            fanout_cap: 0,
            stations_flatten: String::new(),
            stations_filter: None,
            fanout_delay: 0,
        };
        let fixture_lsk = LeapSeconds {
            delta_t_a: 32.184,
            deltas: vec![(37.0, 1483228800.0)],
        };
        let body = r#"[{"ra":89.8,"declination":53.6,"redshift":0.027,"discoverymag":19.8},{"ra":35.0,"declination":-24.4,"redshift":0.0,"discoverymag":19.4}]"#;
        match extract(&src, body, 8.0e8, &fixture_lsk) {
            ExtractResult::Measurements(channels) => {
                assert_eq!(channels.len(), 1);
                assert_eq!(channels[0].1.name, "tns_transient_flux");
                assert!((channels[0].0.z - 0.027).abs() < 1e-9);
            }
            ExtractResult::WithEphemeris(_, _) => panic!("unexpected ephemeris"),
        }
    }

    #[test]
    fn test_extract_csv_zip_end_to_end() {
        let csv = "2026-08-13 00:00:00 - 23:59:59\n\"objid\",\"ra\",\"declination\",\"redshift\",\"discoverymag\"\n\"1\",\"89.8\",\"53.6\",\"0.027\",\"19.8\"\n\"2\",\"35.0\",\"-24.4\",\"\",\"19.4\"\n";
        let mut zip = Vec::new();
        zip.extend_from_slice(b"PK\x03\x04");
        zip.extend_from_slice(&20u16.to_le_bytes());
        zip.extend_from_slice(&0u16.to_le_bytes());
        zip.extend_from_slice(&0u16.to_le_bytes());
        zip.extend_from_slice(&0u16.to_le_bytes());
        zip.extend_from_slice(&0u16.to_le_bytes());
        zip.extend_from_slice(&0u32.to_le_bytes());
        zip.extend_from_slice(&(csv.len() as u32).to_le_bytes());
        zip.extend_from_slice(&(csv.len() as u32).to_le_bytes());
        zip.extend_from_slice(&0u16.to_le_bytes());
        zip.extend_from_slice(&0u16.to_le_bytes());
        zip.extend_from_slice(csv.as_bytes());
        let path = std::env::temp_dir().join("omegaflow_test_tns.zip");
        std::fs::write(&path, &zip).unwrap();
        let src = SourceConfig {
            ttl: 100,
            url: "https://example.com/x".into(),
            frame: Frame::Barycenter {
                body_name: "sun".into(),
                scale: 1.0,
            },
            format: "csv_zip".into(),
            extracts: vec![Extract::CelestialMap {
                arr_path: ".".into(),
                ra_key: "ra".into(),
                dec_key: "declination".into(),
                dist_key: String::new(),
                dist_scale: 1.0,
                plx_key: String::new(),
                z_key: "redshift".into(),
                pmra_key: String::new(),
                pmdec_key: String::new(),
                rv_key: String::new(),
                rv_scale: 1.0,
                epoch_key: String::new(),
                fields: vec![FieldConfig {
                    key: "discoverymag".into(),
                    name: "tns_transient_flux".into(),
                    kernel: 0,
                    force: 0,
                    tau: 3600.0,
                    absorption: 0.0,
                    advection: 0.0,
                    unit: String::new(),
                    fold: None,
                }],
                tau_key: String::new(),
            }],
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
            abs_mag_from: Some("tns_transient_flux".into()),
            catalog_epoch: None,
            repeat_ra_bins: 0,
            fanout_cap: 0,
            stations_flatten: String::new(),
            stations_filter: None,
            fanout_delay: 0,
        };
        let fixture_lsk = LeapSeconds {
            delta_t_a: 32.184,
            deltas: vec![(37.0, 1483228800.0)],
        };
        match extract(&src, path.to_str().unwrap(), 8.0e8, &fixture_lsk) {
            ExtractResult::Measurements(channels) => {
                assert_eq!(channels.len(), 1);
                assert_eq!(channels[0].1.name, "tns_transient_flux");
            }
            ExtractResult::WithEphemeris(_, _) => panic!("unexpected ephemeris"),
        }
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_parse_sources_dist_scale() {
        let phi = "url https://example.com/comets.json\n\
ttl 604800\n\
at sun\n\
cmap .\n\
ra ra\n\
dec dec\n\
dist dist_au\n\
dist_scale 3.085677581e19\n\
field H comet_h_mag gaussian-inverse-square em mag 604800 0.0 0.0\n";
        let sources = parse_sources(phi);
        assert_eq!(sources.len(), 1);
        match &sources[0].extracts[0] {
            Extract::CelestialMap { dist_scale, .. } => {
                assert!((dist_scale - 3.085677581e19).abs() < 1e6);
            }
            _ => panic!("expected CelestialMap extract"),
        }
    }

    #[test]
    fn test_dead_grammar_refused() {
        let phi = "url https://example.com/dead.json\n\
ttl 60\n\
on earth 0.0 0.0 0.0\n\
cmap .\n\
force em\n\
field temp temp_c\n";
        let sources = parse_sources(phi);
        assert_eq!(sources.len(), 1);
        match &sources[0].extracts[0] {
            Extract::CelestialMap { fields, .. } => assert!(fields.is_empty()),
            _ => panic!("expected CelestialMap extract"),
        }
    }

    #[test]
    fn test_extract_cmap_dist_scale_kpc() {
        let json = r#"[{"ra":0.0,"dec":0.0,"dist_kpc":1.0,"H":5.5}]"#;
        let src = SourceConfig {
            ttl: 604800,
            url: "https://example.com/x".into(),
            frame: Frame::Barycenter {
                body_name: "sun".into(),
                scale: 1.0,
            },
            format: "json".into(),
            extracts: vec![Extract::CelestialMap {
                arr_path: ".".into(),
                ra_key: "ra".into(),
                dec_key: "dec".into(),
                dist_key: "dist_kpc".into(),
                dist_scale: 3.085677581e19,
                plx_key: String::new(),
                z_key: String::new(),
                pmra_key: String::new(),
                pmdec_key: String::new(),
                rv_key: String::new(),
                rv_scale: 1.0,
                epoch_key: String::new(),
                fields: vec![FieldConfig {
                    key: "H".into(),
                    name: "comet_h_mag".into(),
                    kernel: 0,
                    force: 0,
                    tau: 604800.0,
                    absorption: 0.0,
                    advection: 0.0,
                    unit: String::new(),
                    fold: None,
                }],
                tau_key: String::new(),
            }],
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
        };
        let fixture_lsk = LeapSeconds {
            delta_t_a: 32.184,
            deltas: vec![(37.0, 1483228800.0)],
        };
        match extract(&src, json, 8.0e8, &fixture_lsk) {
            ExtractResult::Measurements(channels) => {
                assert_eq!(channels.len(), 1);
                assert_eq!(channels[0].1.name, "comet_h_mag");
                assert_eq!(channels[0].0.value, 5.5);
                if let Position::StateVector { p, .. } = channels[0].0.position {
                    let expect = 3.085677581e19;
                    assert!((p[0] - expect).abs() / expect < 1e-12);
                    assert!(p[1].abs() / expect < 1e-12);
                    assert!(p[2].abs() / expect < 1e-12);
                } else {
                    panic!("expected StateVector position");
                }
            }
            ExtractResult::WithEphemeris(_, _) => panic!("unexpected ephemeris"),
        }
    }

    #[test]
    fn test_extract_cmap_pm_radvel_plx() {
        let json =
            r#"[{"ra":0.0,"dec":0.0,"plx":100.0,"pmra":1000.0,"pmdec":2000.0,"rv":50.0,"H":5.5}]"#;
        let src = SourceConfig {
            ttl: 604800,
            url: "https://example.com/x".into(),
            frame: Frame::Barycenter {
                body_name: "sun".into(),
                scale: 1.0,
            },
            format: "json".into(),
            extracts: vec![Extract::CelestialMap {
                arr_path: ".".into(),
                ra_key: "ra".into(),
                dec_key: "dec".into(),
                dist_key: String::new(),
                dist_scale: 1.0,
                plx_key: "plx".into(),
                z_key: String::new(),
                pmra_key: "pmra".into(),
                pmdec_key: "pmdec".into(),
                rv_key: "rv".into(),
                rv_scale: 1.0,
                epoch_key: String::new(),
                fields: vec![FieldConfig {
                    key: "H".into(),
                    name: "comet_h_mag".into(),
                    kernel: 0,
                    force: 0,
                    tau: 604800.0,
                    absorption: 0.0,
                    advection: 0.0,
                    unit: String::new(),
                    fold: None,
                }],
                tau_key: String::new(),
            }],
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
        };
        let fixture_lsk = LeapSeconds {
            delta_t_a: 32.184,
            deltas: vec![(37.0, 1483228800.0)],
        };
        match extract(&src, json, 8.0e8, &fixture_lsk) {
            ExtractResult::Measurements(channels) => {
                assert_eq!(channels.len(), 1);
                if let Position::StateVector { p, v, .. } = channels[0].0.position {
                    let d = super::PARSEC_M * 1000.0 / 100.0;
                    assert!((p[0] - d).abs() / d < 1e-12);
                    assert!(p[1].abs() < 1.0);
                    assert!(p[2].abs() < 1.0);
                    let mu_a = 1000.0 * super::MAS_YR_TO_RAD_S;
                    let mu_d = 2000.0 * super::MAS_YR_TO_RAD_S;
                    let expect_v = [50.0, d * mu_a, d * mu_d];
                    assert!((v[0] - expect_v[0]).abs() < 1e-6);
                    assert!((v[1] - expect_v[1]).abs() < 1e-6);
                    assert!((v[2] - expect_v[2]).abs() < 1e-6);
                } else {
                    panic!("expected StateVector position");
                }
            }
            ExtractResult::WithEphemeris(_, _) => panic!("unexpected ephemeris"),
        }
    }

    #[test]
    fn test_empty_data_anomaly() {
        let _gate = ANOMALY_TEST_GATE.lock();
        ANOMALY_COLLECT.with(|c| c.set(true));
        let sources = parse_sources(
                "url https://example.org/e\nttl 3600\nformat json\nat earth\nmap features\nlat lat\nlon lon\nfield magnitude mag gaussian-inverse-square em mag 3600 0 0\n",
            );
        assert_eq!(sources.len(), 1);
        let src = &sources[0];
        let lsk = full_fixture_lsk();
        let _ = take_anomalies();
        check_empty_data(src, r#"{"features":[]}"#, 0.0, &lsk);
        let anomalies = take_anomalies();
        assert!(anomalies
            .iter()
            .any(|a| a.category == "Empty Data" && a.url == "https://example.org/e"));
        check_empty_data(
            src,
            r#"{"features":[{"lat":10.0,"lon":20.0,"magnitude":5.0}]}"#,
            0.0,
            &lsk,
        );
        let anomalies = take_anomalies();
        assert!(!anomalies.iter().any(|a| a.category == "Empty Data"));
        ANOMALY_COLLECT.with(|c| c.set(false));
    }

    #[test]
    fn test_extract_cmap_no_distance_skipped() {
        let json = r#"[{"ra":0.0,"dec":0.0,"H":5.5}]"#;
        let src = SourceConfig {
            ttl: 604800,
            url: "https://example.com/x".into(),
            frame: Frame::Barycenter {
                body_name: "sun".into(),
                scale: 1.0,
            },
            format: "json".into(),
            extracts: vec![Extract::CelestialMap {
                arr_path: ".".into(),
                ra_key: "ra".into(),
                dec_key: "dec".into(),
                dist_key: String::new(),
                dist_scale: 1.0,
                plx_key: String::new(),
                z_key: String::new(),
                pmra_key: String::new(),
                pmdec_key: String::new(),
                rv_key: String::new(),
                rv_scale: 1.0,
                epoch_key: String::new(),
                fields: vec![FieldConfig {
                    key: "H".into(),
                    name: "comet_h_mag".into(),
                    kernel: 0,
                    force: 0,
                    tau: 604800.0,
                    absorption: 0.0,
                    advection: 0.0,
                    unit: String::new(),
                    fold: None,
                }],
                tau_key: String::new(),
            }],
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
        };
        let fixture_lsk = LeapSeconds {
            delta_t_a: 32.184,
            deltas: vec![(37.0, 1483228800.0)],
        };
        match extract(&src, json, 8.0e8, &fixture_lsk) {
            ExtractResult::Measurements(channels) => {
                assert_eq!(channels.len(), 0);
            }
            ExtractResult::WithEphemeris(_, _) => panic!("unexpected ephemeris"),
        }
    }

    #[test]
    fn test_extract_cmap_null_dist_skipped() {
        let json = r#"[{"ra":0.0,"dec":0.0,"dist_pc":null,"H":5.5}]"#;
        let src = SourceConfig {
            ttl: 604800,
            url: "https://example.com/x".into(),
            frame: Frame::Barycenter {
                body_name: "sun".into(),
                scale: 1.0,
            },
            format: "json".into(),
            extracts: vec![Extract::CelestialMap {
                arr_path: ".".into(),
                ra_key: "ra".into(),
                dec_key: "dec".into(),
                dist_key: "dist_pc".into(),
                dist_scale: 3.085677581e16,
                plx_key: String::new(),
                z_key: String::new(),
                pmra_key: String::new(),
                pmdec_key: String::new(),
                rv_key: String::new(),
                rv_scale: 1.0,
                epoch_key: String::new(),
                fields: vec![FieldConfig {
                    key: "H".into(),
                    name: "comet_h_mag".into(),
                    kernel: 0,
                    force: 0,
                    tau: 604800.0,
                    absorption: 0.0,
                    advection: 0.0,
                    unit: String::new(),
                    fold: None,
                }],
                tau_key: String::new(),
            }],
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
        };
        let fixture_lsk = LeapSeconds {
            delta_t_a: 32.184,
            deltas: vec![(37.0, 1483228800.0)],
        };
        match extract(&src, json, 8.0e8, &fixture_lsk) {
            ExtractResult::Measurements(channels) => {
                assert_eq!(channels.len(), 0);
            }
            ExtractResult::WithEphemeris(_, _) => panic!("unexpected ephemeris"),
        }
    }

    #[test]
    fn test_extract_cmap_csv_dist_scale_mpc() {
        let csv = "AGCNr,Name,RAdeg_HI,Decdeg_HI,RAdeg_OC,DECdeg_OC,Vhelio,W50,errW50,HIflux,errflux,SNR,RMS,Dist,logMsun,HIcode,OCcode,NoteFlag\n\
331061,456-013,0.01042,15.87222,0.00875,15.88167,6007,260,45,1.13,0.09,6.5,2.40,85.2,9.29,1,I,\"\"\n\
331405,\"\",0.01375,26.01639,0.01458,26.01389,10409,315,8,2.62,0.09,16.1,2.05,143.8,10.11,1,I,\"\"\n";
        let src = SourceConfig {
            ttl: 604800,
            url: "https://example.com/x".into(),
            frame: Frame::Barycenter {
                body_name: "sun".into(),
                scale: 1.0,
            },
            format: "csv".into(),
            extracts: vec![Extract::CelestialMap {
                arr_path: ".".into(),
                ra_key: "RAdeg_HI".into(),
                dec_key: "Decdeg_HI".into(),
                dist_key: "Dist".into(),
                dist_scale: 3.085677581e22,
                plx_key: String::new(),
                z_key: String::new(),
                pmra_key: String::new(),
                pmdec_key: String::new(),
                rv_key: String::new(),
                rv_scale: 1.0,
                epoch_key: String::new(),
                fields: vec![FieldConfig {
                    key: "HIflux".into(),
                    name: "alfalfa_hi_flux".into(),
                    kernel: 0,
                    force: 0,
                    tau: 604800.0,
                    absorption: 0.0,
                    advection: 0.0,
                    unit: String::new(),
                    fold: None,
                }],
                tau_key: String::new(),
            }],
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
        };
        let fixture_lsk = LeapSeconds {
            delta_t_a: 32.184,
            deltas: vec![(37.0, 1483228800.0)],
        };
        match extract(&src, csv, 8.0e8, &fixture_lsk) {
            ExtractResult::Measurements(channels) => {
                assert_eq!(channels.len(), 2);
                assert_eq!(channels[0].1.name, "alfalfa_hi_flux");
                assert_eq!(channels[0].0.value, 1.13);
                assert_eq!(channels[1].0.value, 2.62);
                if let Position::StateVector { p, .. } = channels[0].0.position {
                    let expect = 85.2 * 3.085677581e22;
                    let r = (p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt();
                    assert!((r - expect).abs() / expect < 1e-12);
                } else {
                    panic!("expected StateVector position");
                }
            }
            ExtractResult::WithEphemeris(_, _) => panic!("unexpected ephemeris"),
        }
    }

    #[test]
    fn test_star_samples_build_tau() {
        let mut bin = Vec::new();
        bin.extend_from_slice(&0f64.to_le_bytes());
        bin.extend_from_slice(&0f64.to_le_bytes());
        bin.extend_from_slice(&0f32.to_le_bytes());
        bin.extend_from_slice(&0f32.to_le_bytes());
        bin.extend_from_slice(&100f32.to_le_bytes());
        bin.extend_from_slice(&0f32.to_le_bytes());
        bin.extend_from_slice(&1f32.to_le_bytes());
        bin.extend_from_slice(&1.2f32.to_le_bytes());
        bin.extend_from_slice(&30000f32.to_le_bytes());
        let samples = build_star_samples(&bin);
        assert_eq!(samples.len(), 1);
        assert!(samples[0].tau > 0.0);
        assert_eq!(samples[0].val, 1.0);
        assert_eq!(samples[0].force_type, 0.0);
        assert_eq!(samples[0].kernel_id, 0.0);
        assert_eq!(samples[0].ttl, samples[0].tau);
        assert_eq!(samples[0].epoch, 0.0);
        assert!(samples[0].extent.is_infinite());
        assert!((samples[0].color_index - 1.2).abs() < 1e-4);
        let Motion::Spherical { rec } = &samples[0].motion else {
            panic!("spherical motion");
        };
        assert!((rec.plx_mas - 100.0).abs() < 1e-6);
        assert!((rec.rv_m_s - 30000.0).abs() < 1e-4);
        assert!((rec.color_index - 1.2).abs() < 1e-4);
        let (p, _) = star_position_at(rec, 0.0);
        let d = 10.0 * PARSEC_M;
        assert!((p[0] - d).abs() / d < 1e-9);
        assert!((samples[0].anchor_p0[0] - d).abs() / d < 1e-9);
        let short = [0u8; 36];
        assert!(parse_star_record(&short).is_none());
        let legacy = [0u8; 40];
        assert!(parse_star_record(&legacy).is_none());
        assert_eq!(build_star_samples(&bin[..40]).len(), 0);
    }

    #[test]
    fn test_star_samples_diode() {
        let mut bin = Vec::new();
        bin.extend_from_slice(&0f64.to_le_bytes());
        bin.extend_from_slice(&0f64.to_le_bytes());
        bin.extend_from_slice(&0f32.to_le_bytes());
        bin.extend_from_slice(&0f32.to_le_bytes());
        bin.extend_from_slice(&100f32.to_le_bytes());
        bin.extend_from_slice(&0f32.to_le_bytes());
        bin.extend_from_slice(&1f32.to_le_bytes());
        bin.extend_from_slice(&1.2f32.to_le_bytes());
        bin.extend_from_slice(&12000f32.to_le_bytes());
        let samples = build_star_samples(&bin);
        assert_eq!(samples.len(), 1);
        let d = 10.0 * PARSEC_M;
        assert!((samples[0].anchor_p0[0] - d).abs() / d < 1e-9);
        let eph: HashMap<String, BodyEphemeris> = HashMap::new();
        let buf = build_buffer(samples, 1.0, Arc::new(eph.clone()), None, Vec::new());
        let query = |floor: [f64; 9], forward: [f64; 3]| {
            let mut out: Vec<SampleRecord> = Vec::new();
            query_hash(
                &buf.cache,
                [0.0, 0.0, 0.0],
                0.0,
                1.0,
                0.0,
                &floor,
                1.0,
                forward,
                &mut out,
                &eph,
            );
            out
        };
        let dark = query([0.0; 9], [1.0, 0.0, 0.0]);
        assert_eq!(dark.len(), 0);
        let loud = query([1e9; 9], [1.0, 0.0, 0.0]);
        assert_eq!(loud.len(), 0);
        let off_axis = query([0.5; 9], [0.0, 0.0, 1.0]);
        assert_eq!(off_axis.len(), 0);
        let on_axis = query([0.5; 9], [1.0, 0.0, 0.0]);
        assert_eq!(on_axis.len(), 1);
        assert!((on_axis[0].0 - d).abs() / d < 1e-9);
        assert_eq!(on_axis[0].3, 1.0);
        assert_eq!(on_axis[0].7, 0.0);
        assert_eq!(on_axis[0].8, 0.0);
        assert_eq!(on_axis[0].9, 0.0);
        assert!((on_axis[0].21 - 1.2).abs() < 1e-4);
    }

    fn kepler_rec_fixture() -> AsteroidRec {
        AsteroidRec {
            number: 1,
            epoch_jd: J2000_EPOCH,
            a_au: 1.0,
            e: 0.0,
            incl_deg: 0.0,
            node_deg: 0.0,
            peri_deg: 0.0,
            ma_deg: 0.0,
            h: 0.0,
            g: 0.0,
            albedo: 0.0,
            rot_period_h: 0.0,
            radius_km: 0.0,
            gm_km3_s2: 0.0,
            sptype: [0u8; 5],
        }
    }

    #[test]
    fn test_motion_kepler_at_anchor_body_and_law_bounds() {
        let au_m = crate::kepler::AU_M;
        let gm_sun = crate::kepler::GM_SUN_M3_S2;
        let rec = kepler_rec_fixture();
        let motion = Motion::Kepler {
            rec: Arc::new(rec.clone()),
        };
        assert!(motion.anchor_body().is_none());
        let eph: HashMap<String, BodyEphemeris> = HashMap::new();
        let p0 = motion.at(0.0, 0.0, &eph).expect("kepler position at epoch");
        let r0 = (p0[0] * p0[0] + p0[1] * p0[1] + p0[2] * p0[2]).sqrt();
        assert!((r0 - au_m).abs() / au_m < 1e-12);
        let p_dt = motion
            .at(1e-1, 0.0, &eph)
            .expect("kepler position at epoch+dt");
        let v_fd = [
            (p_dt[0] - p0[0]) / 1e-1,
            (p_dt[1] - p0[1]) / 1e-1,
            (p_dt[2] - p0[2]) / 1e-1,
        ];
        let speed = (v_fd[0] * v_fd[0] + v_fd[1] * v_fd[1] + v_fd[2] * v_fd[2]).sqrt();
        let v_circ = (gm_sun / au_m).sqrt();
        assert!((speed - v_circ).abs() / v_circ < 1e-3);
        let (vmax, amax, p_anchor) =
            law_bounds(&motion, 0.0, 0.0, &eph).expect("kepler law bounds");
        assert!((p_anchor[0] - au_m).abs() / au_m < 1e-12);
        assert!((vmax / Φ - v_circ).abs() / v_circ < 1e-4);
        assert!(amax > 0.0 && amax.is_finite());
        let mut unbound = rec;
        unbound.e = 1.5;
        assert!(Motion::Kepler {
            rec: Arc::new(unbound)
        }
        .at(0.0, 0.0, &eph)
        .is_none());
    }

    #[test]
    fn test_build_asteroid_samples_gm_radius_and_query() {
        let mut bin: Vec<u8> = Vec::new();
        let mut with_radius = kepler_rec_fixture();
        with_radius.number = 1;
        with_radius.gm_km3_s2 = 0.5;
        with_radius.radius_km = 3.0;
        crate::dastcom::encode_record(&with_radius, &mut bin);
        let mut far = kepler_rec_fixture();
        far.number = 2;
        far.a_au = 2.0;
        far.gm_km3_s2 = 0.25;
        crate::dastcom::encode_record(&far, &mut bin);
        let mut unbound = kepler_rec_fixture();
        unbound.number = 3;
        unbound.e = 1.5;
        unbound.gm_km3_s2 = 0.5;
        crate::dastcom::encode_record(&unbound, &mut bin);

        let samples = build_asteroid_samples(&bin, 86400);
        assert_eq!(samples.len(), 3);
        let gm = &samples[0];
        let radius = &samples[1];
        let far_gm = &samples[2];
        assert_eq!(gm.name, "dastcom.mass");
        assert_eq!(radius.name, "dastcom.radius");
        assert_eq!(gm.val, 5.0e8);
        assert_eq!(radius.val, 3000.0);
        assert_eq!(far_gm.val, 2.5e8);
        assert_eq!(gm.kernel_id, 0.0);
        assert_eq!(radius.kernel_id, 1.0);
        assert_eq!(gm.force_type, 1.0);
        assert_eq!(radius.force_type, 1.0);
        assert!(gm.extent == 0.0 && gm.tau.is_infinite());
        assert!(radius.extent == 0.0 && radius.tau.is_infinite());
        let Motion::Kepler { rec: rec_gm } = &gm.motion else {
            panic!("kepler motion");
        };
        let Motion::Kepler { rec: rec_radius } = &radius.motion else {
            panic!("kepler motion");
        };
        assert!(Arc::ptr_eq(rec_gm, rec_radius));
        let au_m = crate::kepler::AU_M;
        assert!((gm.anchor_p0[0] - au_m).abs() / au_m < 1e-9);
        let anchor_p0 = gm.anchor_p0;

        let eph: HashMap<String, BodyEphemeris> = HashMap::new();
        let buf = build_buffer(samples, 1.0, Arc::new(eph.clone()), None, Vec::new());
        let mut records: Vec<SampleRecord> = Vec::new();
        query_hash(
            &buf.cache,
            anchor_p0,
            0.0,
            1.0,
            0.0,
            &[0.0; 9],
            1.0,
            [1.0, 0.0, 0.0],
            &mut records,
            &eph,
        );
        assert_eq!(records.len(), 2);
        assert_eq!(records[0].3, 5.0e8);
        assert_eq!(records[1].3, 3000.0);
        assert_eq!(records[0].9, 1.0);
    }

    #[test]
    fn test_motion_spherical_at_anchor_body_and_law_bounds() {
        let rec = StarRec {
            ra_deg: 0.0,
            dec_deg: 0.0,
            pm_ra_masyr: 1000.0,
            pm_de_masyr: 0.0,
            plx_mas: 100.0,
            flux: 1.0,
            mag: 0.0,
            tau: 0.0,
            color_index: 0.0,
            rv_m_s: 0.0,
        };
        let motion = Motion::Spherical {
            rec: Arc::new(rec.clone()),
        };
        assert!(motion.anchor_body().is_none());
        let eph: HashMap<String, BodyEphemeris> = HashMap::new();
        let t_yr = 86400.0 * 365.25;
        let p = motion.at(t_yr, 0.0, &eph).expect("spherical position");
        let (p_ref, v_ref) = star_position_at(&rec, t_yr);
        let d = 10.0 * PARSEC_M;
        for k in 0..3 {
            assert!((p[k] - p_ref[k]).abs() < 1e-9 * d);
        }
        let (vmax, amax, p0) = law_bounds(&motion, 0.0, 0.0, &eph).expect("spherical law bounds");
        assert!((p0[0] - d).abs() / d < 1e-9);
        let speed = (v_ref[0] * v_ref[0] + v_ref[1] * v_ref[1] + v_ref[2] * v_ref[2]).sqrt();
        assert!((vmax / Φ - speed).abs() / speed < 1e-2);
        assert!(amax.is_finite());
    }

    #[test]
    fn test_source_name_flat_and_collision_overrides() {
        let q1 = source_name_from_url("https://h/api?station=1");
        let q2 = source_name_from_url("https://h/api?station=2");
        assert_ne!(q1, q2);
        let again = source_name_from_url("https://h/api?station=1");
        assert_eq!(q1, again);
        let buoy1 = source_name_from_url("https://www.ndbc.noaa.gov/data/realtime/41009.txt");
        let buoy2 = source_name_from_url("https://www.ndbc.noaa.gov/data/realtime/41010.txt");
        assert_ne!(buoy1, buoy2);
        let slash = source_name_from_url("https://h/api/a/b");
        let dash = source_name_from_url("https://h/api/a-b");
        assert_eq!(slash, dash);
        let map = cdn_manifest_for(
            ["https://h/api/a/b", "https://h/api/a-b"]
                .into_iter()
                .map(|s| s.to_string()),
        );
        assert_eq!(
            map.get("https://h/api/a-b").unwrap(),
            &format!("{}-2", slash)
        );
        assert!(!map.contains_key("https://h/api/a/b"));
    }

    #[test]
    fn test_render_headers_secret_substitution() {
        let mut env = HashMap::new();
        env.insert("PURPLEAIR_KEY".to_string(), "secret123".to_string());
        let headers = vec![
            ("X-API-Key".to_string(), "{PURPLEAIR_KEY}".to_string()),
            ("User-Agent".to_string(), "plain".to_string()),
        ];
        let rendered = render_headers(&headers, &env);
        assert_eq!(rendered[0].1, "secret123");
        assert_eq!(rendered[1].1, "plain");
    }

    #[test]
    fn test_parse_station_entries() {
        let j = parse_json(
            r#"{"results":[{"id":"GHCND:AA1","latitude":17.1,"longitude":-61.8,"elevation":10.0},{"id":"GHCND:BB2","latitude":40.9,"longitude":-74.0},{"id":7,"latitude":52.5,"longitude":13.4}]}"#,
        )
        .unwrap();
        let src = SourceConfig {
            ttl: 300,
            url: "https://example.com/x".into(),
            frame: Frame::Surface {
                body_name: "earth".into(),
                lat: 0.0,
                lon: 0.0,
                alt: 0.0,
            },
            format: "json".into(),
            extracts: vec![],
            headers: vec![],
            post_body: None,
            target: None,
            catalog: None,
            max_freq: None,
            min_freq: None,
            body: None,
            stations_url: None,
            stations_path: "results".into(),
            stations_lat: "latitude".into(),
            stations_lon: "longitude".into(),
            stations_id: "id".into(),
            flux_from_mag: None,
            abs_mag_from: None,
            catalog_epoch: None,
            repeat_ra_bins: 0,
            fanout_cap: 0,
            stations_flatten: String::new(),
            stations_filter: None,
            fanout_delay: 0,
        };
        let stations = parse_station_entries(&j, &src);
        assert_eq!(stations.len(), 3);
        assert_eq!(stations[0].id, "GHCND:AA1");
        assert_eq!(stations[0].lat, 17.1);
        assert_eq!(stations[0].lon, -61.8);
        assert_eq!(stations[2].id, "7");
        assert_eq!(stations[2].lat, 52.5);
    }

    #[test]
    fn test_parse_station_entries_flatten_filter() {
        let j = parse_json(
            r#"{"results":[{"coordinates":{"latitude":40.8,"longitude":-73.9},"sensors":[{"id":671,"parameter":{"name":"o3"}},{"id":673,"parameter":{"name":"pm25"}}]},{"coordinates":{"latitude":40.9,"longitude":-74.0},"sensors":[{"id":1097,"parameter":{"name":"pm25"}}]}]}"#,
        )
        .unwrap();
        let src = SourceConfig {
            ttl: 300,
            url: "https://example.com/x".into(),
            frame: Frame::Surface {
                body_name: "earth".into(),
                lat: 0.0,
                lon: 0.0,
                alt: 0.0,
            },
            format: "json".into(),
            extracts: vec![],
            headers: vec![],
            post_body: None,
            target: None,
            catalog: None,
            max_freq: None,
            min_freq: None,
            body: None,
            stations_url: None,
            stations_path: "results".into(),
            stations_lat: "coordinates.latitude".into(),
            stations_lon: "coordinates.longitude".into(),
            stations_id: "id".into(),
            flux_from_mag: None,
            abs_mag_from: None,
            catalog_epoch: None,
            repeat_ra_bins: 0,
            fanout_cap: 0,
            stations_flatten: "sensors".into(),
            stations_filter: Some(("parameter.name".into(), "pm25".into())),
            fanout_delay: 0,
        };
        let stations = parse_station_entries(&j, &src);
        assert_eq!(stations.len(), 2);
        assert_eq!(stations[0].id, "673");
        assert_eq!(stations[0].lat, 40.8);
        assert_eq!(stations[0].lon, -73.9);
        assert_eq!(stations[1].id, "1097");
        assert_eq!(stations[1].lat, 40.9);
        assert_eq!(stations[1].lon, -74.0);
    }

    #[test]
    fn temp_port_convert_check() {
        let content = std::fs::read_to_string("phi/pipeline/queue/master.φ").unwrap();
        let mut blocks = 0usize;
        let mut parsed = 0usize;
        let mut with_extracts = 0usize;
        let mut block = String::new();
        for line in content.lines() {
            let t = line.trim_start();
            if (t.starts_with("url ") || t.starts_with("source ")) && !block.is_empty() {
                blocks += 1;
                let conv = super::port_block(&block);
                let srcs = super::parse_sources(&conv);
                if !srcs.is_empty() {
                    parsed += 1;
                    with_extracts += srcs.iter().filter(|s| !s.extracts.is_empty()).count();
                }
                block = String::new();
            }
            block.push_str(line);
            block.push('\n');
        }
        eprintln!(
            "port convert: {} blocks, {} parsed, {} with extracts",
            blocks, parsed, with_extracts
        );
    }

    #[test]
    fn test_profile_map_parse() {
        let block = "url https://argovis-api.colorado.edu/argo?data=temperature,salinity,pressure\nttl 86400\non earth 0 0 0\nprofile .\nlat geolocation.coordinates.1\nlon geolocation.coordinates.0\nepoch timestamp\npressure pressure\nfield temperature argo_temperature_c erfc thermal C 86400 0.0 0.0\nfield salinity argo_salinity_psu erfc diffusion psu 86400 0.0 0.0\n";
        let srcs = super::parse_sources(block);
        assert_eq!(srcs.len(), 1);
        let prof = srcs[0].extracts.iter().find_map(|e| match e {
            super::Extract::ProfileMap {
                pressure_var,
                fields,
                lat_key,
                lon_key,
                ..
            } => Some((
                pressure_var.clone(),
                fields,
                lat_key.clone(),
                lon_key.clone(),
            )),
            _ => None,
        });
        let (pv, fields, lk, ok) = prof.expect("ProfileMap extract missing");
        assert_eq!(pv, "pressure");
        assert_eq!(lk, "geolocation.coordinates.1");
        assert_eq!(ok, "geolocation.coordinates.0");
        assert_eq!(fields.len(), 2);
        assert_eq!(fields[0].key, "temperature");
        assert_eq!(fields[1].key, "salinity");
    }

    #[test]
    fn test_netcdf_grammar_alt_decibar() {
        let block = "url https://data-argo.ifremer.fr/dac/aoml/1901843/profiles/R1901843_357.nc\nttl 604800\non earth 0 0 0\nformat netcdf\nprofile .\nlat LATITUDE\nlon LONGITUDE\nepoch JULD\nalt PRES decibar\nfield TEMP argo_dac_temp_c erfc thermal C 604800 0.0 0.0\nfield PSAL argo_dac_salinity_psu erfc diffusion psu 604800 0.0 0.0\n";
        let srcs = super::parse_sources(block);
        assert_eq!(srcs.len(), 1);
        assert_eq!(srcs[0].format, "netcdf");
        let prof = srcs[0].extracts.iter().find_map(|e| match e {
            super::Extract::ProfileMap {
                pressure_var,
                pressure_scale,
                fields,
                lat_key,
                lon_key,
                epoch_key,
                ..
            } => Some((
                pressure_var.clone(),
                *pressure_scale,
                fields,
                lat_key.clone(),
                lon_key.clone(),
                epoch_key.clone(),
            )),
            _ => None,
        });
        let (pv, ps, fields, lk, ok, ek) = prof.expect("ProfileMap extract missing");
        assert_eq!(pv, "PRES");
        assert_eq!(ps, 1.0);
        assert_eq!(lk, "LATITUDE");
        assert_eq!(ok, "LONGITUDE");
        assert_eq!(ek, "JULD");
        assert_eq!(fields.len(), 2);
        assert_eq!(fields[0].key, "TEMP");
        assert_eq!(fields[1].key, "PSAL");
    }

    #[test]
    fn test_build_netcdf_channels() {
        let block = "url https://data-argo.ifremer.fr/dac/aoml/1901843/profiles/R1901843_357.nc\nttl 604800\non earth 0 0 0\nformat netcdf\nprofile .\nlat LATITUDE\nlon LONGITUDE\nepoch JULD\nalt PRES decibar\nfield TEMP argo_dac_temp_c erfc thermal C 604800 0.0 0.0\nfield PSAL argo_dac_salinity_psu erfc diffusion psu 604800 0.0 0.0\n";
        let srcs = super::parse_sources(block);
        let u32b = |x: u32| x.to_be_bytes().to_vec();
        let f64b = |x: f64| x.to_bits().to_be_bytes().to_vec();
        let f32b = |x: f32| x.to_bits().to_be_bytes().to_vec();
        let name = |s: &str| {
            let mut b = u32b(s.len() as u32);
            b.extend_from_slice(s.as_bytes());
            while b.len() % 4 != 0 {
                b.push(0);
            }
            b
        };
        let mut b = Vec::new();
        b.extend([0x43, 0x44, 0x46, 0x01]);
        b.extend(u32b(0));
        b.extend(u32b(0x0A));
        b.extend(u32b(2));
        b.extend(name("N_PROF"));
        b.extend(u32b(1));
        b.extend(name("N_LEVELS"));
        b.extend(u32b(3));
        b.extend(u32b(0));
        b.extend(u32b(0));
        b.extend(u32b(0x0B));
        b.extend(u32b(6));
        let var = |b: &mut Vec<u8>,
                   nm: &str,
                   rank: u32,
                   dims: &[u32],
                   fill: Option<f32>,
                   t: u32,
                   vsize: u32|
         -> usize {
            b.extend(name(nm));
            b.extend(u32b(rank));
            for &d in dims {
                b.extend(u32b(d));
            }
            match fill {
                Some(fv) => {
                    b.extend(u32b(0x0C));
                    b.extend(u32b(1));
                    b.extend(name("_FillValue"));
                    b.extend(u32b(5));
                    b.extend(u32b(1));
                    b.extend(f32b(fv));
                }
                None => {
                    b.extend(u32b(0));
                    b.extend(u32b(0));
                }
            }
            b.extend(u32b(t));
            b.extend(u32b(vsize));
            let slot = b.len();
            b.extend(u32b(0));
            slot
        };
        let slots = vec![
            var(&mut b, "LATITUDE", 1, &[0], None, 6, 8),
            var(&mut b, "LONGITUDE", 1, &[0], None, 6, 8),
            var(&mut b, "JULD", 1, &[0], None, 6, 8),
            var(&mut b, "PRES", 2, &[0, 1], Some(99999.0), 5, 4),
            var(&mut b, "TEMP", 2, &[0, 1], Some(99999.0), 5, 4),
            var(&mut b, "PSAL", 2, &[0, 1], Some(99999.0), 5, 4),
        ];
        let data_start = b.len() as u64;
        let begins = [
            data_start,
            data_start + 8,
            data_start + 16,
            data_start + 24,
            data_start + 36,
            data_start + 48,
        ];
        for (slot, beg) in slots.iter().zip(begins.iter()) {
            b[*slot..*slot + 4].copy_from_slice(&(*beg as u32).to_be_bytes());
        }
        b.extend(f64b(-15.77751));
        b.extend(f64b(57.37286));
        b.extend(f64b(27965.773125022904));
        for p in [1.08f32, 2.0, 99999.0] {
            b.extend(f32b(p));
        }
        for t in [25.862f32, f32::NAN, 10.0] {
            b.extend(f32b(t));
        }
        for s in [35.0314f32, 35.0, 34.9] {
            b.extend(f32b(s));
        }
        let lsk = full_fixture_lsk();
        let expected_epoch = lsk
            .unix_to_tdb((27965.773125022904f64 - 7305.0) * 86400.0)
            .unwrap();
        let channels = super::build_netcdf_channels(&srcs[0], &b, &lsk);
        assert_eq!(channels.len(), 3);
        assert_eq!(channels[0].0.name, "argo_dac_temp_c");
        assert_eq!(channels[1].0.name, "argo_dac_salinity_psu");
        assert_eq!(channels[2].0.name, "argo_dac_salinity_psu");
        assert!((channels[0].0.value - 25.862).abs() < 1e-3);
        assert!((channels[1].0.value - 35.0314).abs() < 1e-3);
        assert!((channels[2].0.value - 35.0).abs() < 1e-6);
        assert!((channels[0].0.epoch - expected_epoch).abs() < 1e-6);
        assert!((channels[1].0.epoch - expected_epoch).abs() < 1e-6);
        assert!((channels[2].0.epoch - expected_epoch).abs() < 1e-6);
        let alts: Vec<f64> = channels
            .iter()
            .map(|(c, _)| match &c.position {
                super::Position::Surface { alt, lat, lon, .. } => {
                    assert!((*lat + 15.77751).abs() < 1e-4);
                    assert!((*lon - 57.37286).abs() < 1e-4);
                    *alt
                }
                _ => panic!("position is not Surface"),
            })
            .collect();
        assert!((alts[0] + 1.08).abs() < 1e-2);
        assert!((alts[1] + 1.08).abs() < 1e-2);
        assert!((alts[2] + 2.0).abs() < 1e-2);
    }

    #[test]
    fn test_port_convert_celestial_and_post() {
        let celestial = "source oac\nttl 86400\nforce em\nurl https://api.example.org/{target}/\nverify false\ntarget SN2014J\nmap .\nlat_key ra\nlon_key dec\nfield name name\n";
        let conv = super::port_block(celestial);
        assert!(conv.contains("ttl 86400\n"));
        assert!(conv.contains("at sun\n"));
        assert!(conv.contains("url https://api.example.org/{target}/\n"));
        let srcs = super::parse_sources(&conv);
        for s in &srcs {
            for e in &s.extracts {
                match e {
                    super::Extract::CelestialMap { fields, .. }
                    | super::Extract::Map { fields, .. } => assert!(fields.is_empty()),
                    _ => {}
                }
            }
        }
        let post = "source stac\nttl 86400\nforce em\nurl https://example.org/search\nmethod post\nbody {\"collections\":[\"x\"],\"bbox\":[{lon_min},{lat_min},{lon_max},{lat_max}]}\nmap features\nlat_key properties.centroid.lat\nlon_key properties.centroid.lon\nfield id scene\n";
        let conv = super::port_block(post);
        assert!(conv.contains(
                "post_body {\"collections\":[\"x\"],\"bbox\":[{lon_min},{lat_min},{lon_max},{lat_max}]}\n"
            ));
        let srcs = super::parse_sources(&conv);
        assert!(
            srcs.is_empty()
                || srcs.iter().all(|s| s.extracts.iter().all(|e| match e {
                    super::Extract::Map { fields, .. } => fields.is_empty(),
                    _ => true,
                }))
        );
        let array_post = "source arr\nttl 86400\nforce em\nurl https://example.org/search\nmethod POST\nbody [1,2,3]\nmap features\nlat_key properties.centroid.lat\nlon_key properties.centroid.lon\nfield id scene\n";
        let conv = super::port_block(array_post);
        assert!(conv.contains("post_body [1,2,3]\n"));
        let srcs = super::parse_sources(&conv);
        assert!(
            srcs.is_empty()
                || srcs.iter().all(|s| s.extracts.iter().all(|e| match e {
                    super::Extract::Map { fields, .. } => fields.is_empty(),
                    _ => true,
                }))
        );
        let form_post = "source form\nttl 86400\nforce em\nurl https://example.org/search\nmethod POST\nbody collection=landsat&limit=10\nmap features\nlat_key lat\nlon_key lon\nfield id scene\n";
        let conv = super::port_block(form_post);
        assert!(conv.contains("post_body collection=landsat&limit=10\n"));
        let srcs = super::parse_sources(&conv);
        assert!(
            srcs.is_empty()
                || srcs.iter().all(|s| s.extracts.iter().all(|e| match e {
                    super::Extract::Map { fields, .. } => fields.is_empty(),
                    _ => true,
                }))
        );
        let no_method = "source kt\nttl 86400\nforce em\nurl https://example.org/x\nformat kernel_text\nbody 399\n";
        let conv = super::port_block(no_method);
        assert!(conv.contains("body 399\n"));
        let srcs = super::parse_sources(&conv);
        assert!(
            srcs.is_empty()
                || srcs.iter().all(|s| s.extracts.iter().all(|e| match e {
                    super::Extract::Map { fields, .. } => fields.is_empty(),
                    _ => true,
                }))
        );
    }
    #[test]
    fn test_walk_celestial_cmap() {
        let j =
            super::parse_json("{\"results\":[{\"ra\":1.5,\"dec\":-2.5,\"mag\":12.3}]}").unwrap();
        let mut fields = String::new();
        let mut coords = String::new();
        let mut map_path: Option<String> = None;
        let mut budget = 48usize;
        super::walk_json_probe(&j, "", &mut fields, &mut coords, &mut map_path, &mut budget);
        assert!(coords.contains("ra "));
        assert!(coords.contains("dec "));
        assert_eq!(map_path.as_deref(), Some("results"));
        assert!(fields.contains("field"));
        let (frame, _) = super::derive_frame(&j, &coords);
        assert!(frame.starts_with("at sun"));
    }

    #[test]
    fn test_tap_to_json_rows() {
        let j = super::parse_json(
                "{\"metadata\":[{\"name\":\"RAJ2000\"},{\"name\":\"DEJ2000\"},{\"name\":\"Ksmag\"}],\"data\":[[1.5,-2.5,12.3],[3.0,4.0,10.1]]}",
            )
            .unwrap();
        let flat = super::tap_to_json(&j).unwrap();
        let mut fields = String::new();
        let mut coords = String::new();
        let mut map_path: Option<String> = None;
        let mut budget = 48usize;
        super::walk_json_probe(
            &flat,
            "",
            &mut fields,
            &mut coords,
            &mut map_path,
            &mut budget,
        );
        assert!(coords.contains("ra RAJ2000"));
        assert!(coords.contains("dec DEJ2000"));
        assert_eq!(map_path.as_deref(), Some("."));
        let (frame, _) = super::derive_frame(&flat, &coords);
        assert!(frame.starts_with("at sun"));
    }

    #[test]
    fn test_parse_stations_xml() {
        let xml = "<?xml version=\"1.0\" ?><GINServices>\n <ObservatoryList>\n  <Observatory>\n   <Code>AAE</Code>\n   <Name>Addis Ababa</Name>\n   <Latitude>9.035</Latitude>   <Longitude>38.770</Longitude>   <Elevation>2441</Elevation>\n  </Observatory>\n  <Observatory>\n   <Code>YKC</Code>\n   <Latitude>62.48</Latitude>   <Longitude>-114.48</Longitude>   <Elevation>181</Elevation>\n  </Observatory>\n </ObservatoryList>\n</GINServices>";
        let st = parse_stations_xml(xml);
        assert_eq!(st.len(), 2);
        assert_eq!(st[0].id, "aae");
        assert_eq!(st[0].lat, 9.035);
        assert_eq!(st[0].lon, 38.770);
        assert_eq!(st[1].id, "ykc");
        assert_eq!(st[1].lat, 62.48);
    }

    #[test]
    fn test_backlog_batches_verify() {
        fn substitute_test_templates(url: &str) -> String {
            let mut u = url.to_string();
            for (k, v) in [
                ("{today}", "2026-08-07"),
                ("{yesterday}", "2026-08-06"),
                ("{tomorrow}", "2026-08-08"),
                ("{now}", "2026-08-07T12:00:00Z"),
                ("{year}", "2026"),
                ("{month}", "08"),
                ("{day}", "07"),
                ("{lat}", "29.5"),
                ("{lon}", "-95.0"),
                ("{ra}", "0.0"),
                ("{dec}", "0.0"),
                ("{target}", "Ceres"),
                ("{week_ago}", "2026-07-31"),
                ("{hour_ago}", "2026-08-07T11:00:00Z"),
                ("{body}", "ISS"),
                ("{lon_min}", "-95.0"),
                ("{lon_max}", "-94.0"),
                ("{lat_min}", "29.0"),
                ("{lat_max}", "30.0"),
                ("{grid}", "29.5,-95.0|29.6,-95.0"),
                ("{nearest_station}", "8518750"),
            ] {
                u = u.replace(k, v);
            }
            u
        }
        let live: std::collections::HashSet<String> = super::load_sources()
            .iter()
            .map(|s| s.url.clone())
            .collect();
        let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();
        let mut ok_text = String::from("# staging: backlog blocks verified with samples\n");
        if let Ok(existing) = std::fs::read_to_string("phi/pipeline/stage/staging_verified.φ") {
            for l in existing.lines() {
                let t = l.trim_start();
                if t.starts_with("url ") {
                    seen.insert(t[4..].trim().to_string());
                }
            }
            ok_text = existing;
        }
        let mut void_text = String::new();
        if let Ok(existing) = std::fs::read_to_string("phi/pipeline/stage/staging_void_ledger.txt")
        {
            for l in existing.lines() {
                if let Some(u) = l.strip_prefix("void ") {
                    if let Some(end) = u.find(' ') {
                        seen.insert(u[..end].to_string());
                    }
                }
            }
            void_text = existing;
        }
        let fixture_lsk = full_fixture_lsk();
        let now = fixture_lsk.system_now_tdb().unwrap();
        let env = super::load_env();
        let mut limit = 300usize;
        let mut ok = 0usize;
        let mut empty = 0usize;
        for e in std::fs::read_dir("phi/pipeline/stage").unwrap().flatten() {
            let fname = e.file_name().to_string_lossy().to_string();
            if !fname.ends_with("_converted.φ") {
                continue;
            }
            let content = std::fs::read_to_string(e.path()).unwrap();
            let mut block = String::new();
            for line in content.lines().chain(std::iter::once("url __eof__")) {
                let t = line.trim_start();
                if t.starts_with("url ") && !block.is_empty() {
                    if limit == 0 {
                        break;
                    }
                    let srcs = super::parse_sources(&block);
                    for s in &srcs {
                        if s.fanout_cap > 0 || s.format == "csv_zip" || s.format == "kernel_text" {
                            break;
                        }
                        let mut url = substitute_test_templates(&s.url);
                        url = super::resolve_secret(&url, &env);
                        url = url.replace(' ', "%20");
                        if live.contains(&s.url) || !seen.insert(s.url.clone()) {
                            break;
                        }
                        limit -= 1;
                        let headers = super::render_headers(&s.headers, &env);
                        let post_body = match &s.post_body {
                            Some(pb) => Some(substitute_test_templates(pb)),
                            None => None,
                        };
                        let post = post_body.as_deref();
                        let body = match super::fetch_raw_probe(&url, post, &headers) {
                            Some(b) => b,
                            None => {
                                empty += 1;
                                void_text.push_str(&format!(
                                    "void {} {}\n",
                                    s.url, "fetch returned empty"
                                ));
                                break;
                            }
                        };
                        let n_samples = match super::extract(s, &body, now, &fixture_lsk) {
                            super::ExtractResult::Measurements(v) => v.len(),
                            super::ExtractResult::WithEphemeris(v, _) => v.len(),
                        };
                        if n_samples == 0 {
                            empty += 1;
                            void_text.push_str(&format!(
                                "void {} {}\n",
                                s.url,
                                super::diagnose_no_samples(s, &body)
                            ));
                        } else {
                            ok += 1;
                            ok_text.push_str(&format!("# from {}\n", fname));
                            ok_text.push_str(&block);
                            ok_text.push('\n');
                        }
                    }
                    block = String::new();
                }
                block.push_str(line);
                block.push('\n');
            }
        }
        eprintln!("=== BACKLOG VERIFY: {} ok, {} empty ===", ok, empty);
        let ok_path = "phi/pipeline/stage/staging_verified.φ";
        let void_path = "phi/pipeline/stage/staging_void_ledger.txt";
        std::fs::write(ok_path, &ok_text).unwrap();
        std::fs::write(void_path, &void_text).unwrap();
        eprintln!("staged: {} and {}", ok_path, void_path);
    }

    #[test]
    fn test_erddap_argo_map_extract() {
        let src = SourceConfig {
            ttl: 43200,
            url: "https://erddap.ifremer.fr/erddap/tabledap/ArgoFloats.json".into(),
            frame: super::Frame::Surface {
                body_name: "body_test".into(),
                lat: 0.0,
                lon: 0.0,
                alt: 0.0,
            },
            format: "json".into(),
            extracts: vec![super::Extract::Map {
                arr_path: "table.rows".into(),
                lat_key: "2".into(),
                lon_key: "1".into(),
                alt_key: "3".into(),
                epoch_key: "0".into(),
                val_key: String::new(),
                alt_scale: -1.0,
                vel_key: String::new(),
                vel_scale: 1.0,
                trk_key: String::new(),
                vr_key: String::new(),
                fields: vec![FieldConfig {
                    key: "4".into(),
                    name: "argo_temp_c".into(),
                    kernel: 0,
                    force: 0,
                    tau: 0.0,
                    absorption: 0.0,
                    advection: 0.0,
                    unit: String::new(),
                    fold: None,
                }],
                lat_sign: None,
                lon_sign: None,
                epoch_scale: 1.0,
                tau_key: String::new(),
                mag_type_key: String::new(),
            }],
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
        };
        let body = r#"{"table":{"columnNames":["time","longitude","latitude","pres","temp"],"columnTypes":["String","double","double","float","float"],"rows":[["2026-07-30T21:40:30Z",-14.408395,34.49025,3.1,23.478],["2026-07-30T22:00:00Z",-12.5,35.0,1000.0,4.681]]}}"#;
        let fixture_lsk = super::LeapSeconds {
            delta_t_a: 32.184,
            deltas: vec![(37.0, 1483228800.0)],
        };
        let test_epoch = super::parse_iso_tdb("2026-07-30T21:40:30Z", &fixture_lsk).unwrap();
        let now = test_epoch + 86400.0;
        eprintln!("now={} test_epoch={}", now, test_epoch);
        let result = super::extract(&src, body, now, &fixture_lsk);
        let channels = match result {
            super::ExtractResult::Measurements(v) => v,
            _ => {
                panic!("ExtractResult is WithEphemeris, 0 Channels");
            }
        };
        assert_eq!(channels.len(), 2);
        let (p0, _f0) = &channels[0];
        assert!(p0.epoch < now);
        let test_epoch = super::parse_iso_tdb("2026-07-30T21:40:30Z", &fixture_lsk).unwrap();
        assert!((p0.epoch - test_epoch).abs() < 1e-6);
        match p0.position {
            super::Position::Surface {
                lat,
                lon,
                alt,
                body_name: _,
            } => {
                assert!((lat - 34.49025).abs() < 1e-6);
                assert!((lon - -14.408395).abs() < 1e-6);
                assert!((alt - -3.1).abs() < 1e-6);
            }
            _ => panic!("position is {:?}", p0.position),
        }
        assert_eq!(p0.name, "argo_temp_c");
        assert!((p0.value - 23.478).abs() < 1e-6);
        let (p1, _) = &channels[1];
        match p1.position {
            super::Position::Surface { alt, .. } => {
                assert!((alt - -1000.0).abs() < 1e-6);
            }
            _ => panic!("position is {:?}", p1.position),
        }
    }

    #[test]
    fn test_ymd_days_roundtrip() {
        for (y, m, d) in [
            (1970, 1, 1),
            (2000, 2, 29),
            (2026, 7, 31),
            (2026, 12, 31),
            (2026, 1, 1),
        ] {
            let days = super::ymd_to_days(y, m, d).unwrap();
            let (y2, m2, d2) = super::days_to_ymd(days);
            assert_eq!(
                (y2 as i64, m2, d2),
                (y, m, d),
                "roundtrip {} {}-{}-{}",
                days,
                y,
                m,
                d
            );
        }
        assert_eq!(super::ymd_to_days(1970, 1, 1).unwrap(), 0);
        assert!(super::ymd_to_days(1900, 1, 1).is_none());
        assert_eq!(super::ymd_to_days(1970, 1, 1).unwrap(), 0);
    }

    #[test]
    fn test_kernel_id_of() {
        assert_eq!(super::kernel_id_of("inverse-square"), Some(0));
        assert_eq!(super::kernel_id_of("gaussian-inverse-square"), Some(1));
        assert_eq!(super::kernel_id_of("gaussian-inverse"), Some(2));
        assert_eq!(super::kernel_id_of("erfc"), Some(3));
        assert_eq!(super::kernel_id_of("exponential-decay"), Some(4));
        assert_eq!(super::kernel_id_of("patch-levy"), Some(5));
        assert_eq!(super::kernel_id_of("inverse-linear"), Some(6));
        assert_eq!(super::kernel_id_of("nonexistent"), None);
    }

    #[test]
    fn test_universal_auto_detect_celestial() {
        let body = r#"{"data":[{"ra":83.63,"dec":22.01,"plx":3.14,"val":14.2,"t":2457389.5}]}"#;
        let j = super::parse_json(body).unwrap();
        let extracts = super::universal_auto_detect(&j);
        assert_eq!(extracts.len(), 1);
        match &extracts[0] {
            super::Extract::CelestialMap {
                ra_key,
                dec_key,
                plx_key,
                pmra_key,
                pmdec_key,
                arr_path,
                fields,
                ..
            } => {
                assert_eq!(ra_key, "ra");
                assert_eq!(dec_key, "dec");
                assert_eq!(plx_key, "plx");
                assert_eq!(pmra_key, "");
                assert_eq!(pmdec_key, "");
                assert_eq!(arr_path, "data");
                assert!(!fields.is_empty());
            }
            _ => panic!("auto_detect returned Map/Rows/Field, CelestialMap absent"),
        }
    }

    #[test]
    fn test_universal_auto_detect_terrestrial() {
        let body = r#"{"data":[{"lat":52.5,"lon":13.4,"val":28.5,"alt":50.0}]}"#;
        let j = super::parse_json(body).unwrap();
        let extracts = super::universal_auto_detect(&j);
        assert_eq!(extracts.len(), 1);
        match &extracts[0] {
            super::Extract::Map {
                lat_key,
                lon_key,
                alt_key,
                arr_path,
                ..
            } => {
                assert_eq!(lat_key, "lat");
                assert_eq!(lon_key, "lon");
                assert_eq!(alt_key, "alt");
                assert_eq!(arr_path, "data");
            }
            _ => panic!("auto_detect returned CelestialMap/Rows/Field, Map absent"),
        }
    }

    #[test]
    fn test_wgccre_roundtrip() {
        use std::collections::HashMap;
        let tdb = 3.0 * 86400.0;
        let mut cx: [f64; super::CHEBYSHEV_N] = [0.0; super::CHEBYSHEV_N];
        cx[0] = 1.5e9;
        let props = super::BodyProperties {
            α0_deg: 317.68143,
            dα0_dt_deg_per_century: -0.1061,
            δ0_deg: 52.88650,
            dδ0_dt_deg_per_century: -0.0609,
            w0_deg: 176.630,
            dw_dt_deg_per_day: 350.89198226,
            radius_m: 3389500.0,
            flattening: Some(0.00589),
            gaussian_inverse_square: 0.0,
            gaussian_inverse: 0.0,
            erfc: 0.0,
            patch_levy: 0.0,
            exponential_decay: 0.0,
            gm: None,
            j2: None,
            j4: None,
            radii_b: None,
            radii_c: None,
            nut_ra: None,
            nut_dec: None,
            nutation: None,
            omega_g: None,
        };
        let granule = super::ChebyshevGranule {
            t0_jd: super::J2000_EPOCH,
            dt_jd: 32.0,
            cx,
            cy: [0.0; super::CHEBYSHEV_N],
            cz: [0.0; super::CHEBYSHEV_N],
        };
        let eph = super::BodyEphemeris {
            granules: vec![granule],
            rotation_matrices: vec![],
            props: Some(props),
        };
        let mut map = HashMap::new();
        map.insert("mars".to_string(), eph);
        let cases = [
            (35.0, -15.0, 0.0),
            (0.0, 90.0, 0.0),
            (89.9, 45.0, 0.0),
            (-60.0, 170.0, 5000.0),
        ];
        for (lat, lon, alt) in cases {
            let p = super::body_fixed_to_icrs("mars", lat, lon, alt, tdb, &map).unwrap();
            let (lat2, lon2) =
                super::icrs_to_body_surface(p[0], p[1], p[2], tdb, "mars", &map).unwrap();
            assert!((lat2 - lat).abs() < 1e-6, "lat {} vs {}", lat2, lat);
            assert!((lon2 - lon).abs() < 1e-6, "lon {} vs {}", lon2, lon);
        }
    }

    #[test]
    fn test_rotation_matrix_roundtrip() {
        use std::collections::HashMap;
        let tdb = 3.0 * 86400.0;
        let jd = super::J2000_EPOCH + 3.0;
        let tc = (jd - super::J2000_EPOCH) / 36525.0;
        let a = (317.68143f64 - 0.1061 * tc).to_radians();
        let d = (52.88650f64 - 0.0609 * tc).to_radians();
        let w = (176.630f64 + 350.89198226 * (jd - super::J2000_EPOCH)
            - (317.68143f64 - 0.1061 * tc))
            .to_radians();
        let (sa, ca) = a.sin_cos();
        let (sd, cd) = d.sin_cos();
        let (sw, cw) = w.sin_cos();
        let m: [f64; 9] = [
            cd * ca * cw + sa * sw,
            cd * ca * sw - sa * cw,
            -sd * ca,
            cd * sa * cw - ca * sw,
            cd * sa * sw + ca * cw,
            -sd * sa,
            sd * cw,
            sd * sw,
            cd,
        ];
        let mut cx: [f64; super::CHEBYSHEV_N] = [0.0; super::CHEBYSHEV_N];
        cx[0] = 1.5e9;
        let props = super::BodyProperties {
            α0_deg: 317.68143,
            dα0_dt_deg_per_century: -0.1061,
            δ0_deg: 52.88650,
            dδ0_dt_deg_per_century: -0.0609,
            w0_deg: 176.630,
            dw_dt_deg_per_day: 350.89198226,
            radius_m: 3389500.0,
            flattening: Some(0.00589),
            gaussian_inverse_square: 0.0,
            gaussian_inverse: 0.0,
            erfc: 0.0,
            patch_levy: 0.0,
            exponential_decay: 0.0,
            gm: None,
            j2: None,
            j4: None,
            radii_b: None,
            radii_c: None,
            nut_ra: None,
            nut_dec: None,
            nutation: None,
            omega_g: None,
        };
        let granule = super::ChebyshevGranule {
            t0_jd: super::J2000_EPOCH,
            dt_jd: 32.0,
            cx,
            cy: [0.0; super::CHEBYSHEV_N],
            cz: [0.0; super::CHEBYSHEV_N],
        };
        let eph = super::BodyEphemeris {
            granules: vec![granule],
            rotation_matrices: vec![(jd, m)],
            props: Some(props),
        };
        let mut map = HashMap::new();
        map.insert("mars".to_string(), eph);
        let cases = [
            (35.0, -15.0, 0.0),
            (0.0, 90.0, 0.0),
            (89.9, 45.0, 0.0),
            (-60.0, 170.0, 5000.0),
        ];
        for (lat, lon, alt) in cases {
            let p = super::body_fixed_to_icrs("mars", lat, lon, alt, tdb, &map).unwrap();
            let (lat2, lon2) =
                super::icrs_to_body_surface(p[0], p[1], p[2], tdb, "mars", &map).unwrap();
            assert!((lat2 - lat).abs() < 1e-6, "lat {} vs {}", lat2, lat);
            assert!((lon2 - lon).abs() < 1e-6, "lon {} vs {}", lon2, lon);
        }
    }

    #[test]
    fn test_matrix_vs_wgccre_agreement() {
        use std::collections::HashMap;
        let tdb = 3.0 * 86400.0;
        let jd = super::J2000_EPOCH + 3.0;
        let tc = (jd - super::J2000_EPOCH) / 36525.0;
        let a = (317.68143f64 - 0.1061 * tc).to_radians();
        let d = (52.88650f64 - 0.0609 * tc).to_radians();
        let w = (176.630f64 + 350.89198226 * (jd - super::J2000_EPOCH)
            - (317.68143f64 - 0.1061 * tc))
            .to_radians();
        let (sa, ca) = a.sin_cos();
        let (sd, cd) = d.sin_cos();
        let (sw, cw) = w.sin_cos();
        let m: [f64; 9] = [
            cd * ca * cw + sa * sw,
            cd * ca * sw - sa * cw,
            -sd * ca,
            cd * sa * cw - ca * sw,
            cd * sa * sw + ca * cw,
            -sd * sa,
            sd * cw,
            sd * sw,
            cd,
        ];
        let mut cx: [f64; super::CHEBYSHEV_N] = [0.0; super::CHEBYSHEV_N];
        cx[0] = 1.5e9;
        let props = super::BodyProperties {
            α0_deg: 317.68143,
            dα0_dt_deg_per_century: -0.1061,
            δ0_deg: 52.88650,
            dδ0_dt_deg_per_century: -0.0609,
            w0_deg: 176.630,
            dw_dt_deg_per_day: 350.89198226,
            radius_m: 3389500.0,
            flattening: Some(0.00589),
            gaussian_inverse_square: 0.0,
            gaussian_inverse: 0.0,
            erfc: 0.0,
            patch_levy: 0.0,
            exponential_decay: 0.0,
            gm: None,
            j2: None,
            j4: None,
            radii_b: None,
            radii_c: None,
            nut_ra: None,
            nut_dec: None,
            nutation: None,
            omega_g: None,
        };
        let granule = super::ChebyshevGranule {
            t0_jd: super::J2000_EPOCH,
            dt_jd: 32.0,
            cx,
            cy: [0.0; super::CHEBYSHEV_N],
            cz: [0.0; super::CHEBYSHEV_N],
        };
        let eph_matrix = super::BodyEphemeris {
            granules: vec![granule.clone()],
            rotation_matrices: vec![(jd, m)],
            props: Some(props.clone()),
        };
        let eph_test = super::BodyEphemeris {
            granules: vec![granule],
            rotation_matrices: vec![],
            props: Some(props),
        };
        let mut map_matrix = HashMap::new();
        map_matrix.insert("mars".to_string(), eph_matrix);
        let mut map_props = HashMap::new();
        map_props.insert("mars".to_string(), eph_test);
        let cases = [(35.0, -15.0, 0.0), (0.0, 90.0, 0.0), (-60.0, 170.0, 5000.0)];
        for (lat, lon, alt) in cases {
            let pm = super::body_fixed_to_icrs("mars", lat, lon, alt, tdb, &map_matrix).unwrap();
            let pf = super::body_fixed_to_icrs("mars", lat, lon, alt, tdb, &map_props).unwrap();
            let d2 = (pm[0] - pf[0]).powi(2) + (pm[1] - pf[1]).powi(2) + (pm[2] - pf[2]).powi(2);
            assert!(
                d2 < 1.0,
                "matrix vs wgccre disagree at ({}, {}): d2={}",
                lat,
                lon,
                d2
            );
        }
    }

    #[test]
    fn test_rotation_matrix_empty_props() {
        use std::collections::HashMap;
        let tdb = 3.0 * 86400.0;
        let mut cx: [f64; super::CHEBYSHEV_N] = [0.0; super::CHEBYSHEV_N];
        cx[0] = 1.5e9;
        let props = super::BodyProperties {
            α0_deg: 317.68143,
            dα0_dt_deg_per_century: -0.1061,
            δ0_deg: 52.88650,
            dδ0_dt_deg_per_century: -0.0609,
            w0_deg: 176.630,
            dw_dt_deg_per_day: 350.89198226,
            radius_m: 3389500.0,
            flattening: Some(0.00589),
            gaussian_inverse_square: 0.0,
            gaussian_inverse: 0.0,
            erfc: 0.0,
            patch_levy: 0.0,
            exponential_decay: 0.0,
            gm: None,
            j2: None,
            j4: None,
            radii_b: None,
            radii_c: None,
            nut_ra: None,
            nut_dec: None,
            nutation: None,
            omega_g: None,
        };
        let granule = super::ChebyshevGranule {
            t0_jd: super::J2000_EPOCH,
            dt_jd: 32.0,
            cx,
            cy: [0.0; super::CHEBYSHEV_N],
            cz: [0.0; super::CHEBYSHEV_N],
        };
        let eph = super::BodyEphemeris {
            granules: vec![granule],
            rotation_matrices: vec![],
            props: Some(props),
        };
        let mut map = HashMap::new();
        map.insert("mars".to_string(), eph);
        let p = super::body_fixed_to_icrs("mars", 35.0, -15.0, 0.0, tdb, &map).unwrap();
        assert!(p[0] > 1.0e9);
    }

    #[test]
    fn test_restored_extract_variants() {
        let j = super::parse_json(
            r#"{"data":[{"a":1,"nested":{"b":9}},{"a":2},{"a":3}],"x":[10,20,30]}"#,
        )
        .unwrap();
        assert_eq!(super::jfirst(&j, "data.a"), Some(1.0));
        assert_eq!(super::jlast(&j, "data.a"), Some(3.0));
        assert_eq!(super::jdeep_find_num(&j, "b"), Some(9.0));
        assert_eq!(super::jcount(&j, "data"), Some(3.0));
        assert_eq!(super::jpath(&j, "x.-1"), Some(30.0));
        let j2 = super::parse_json(r#"[["t","a"],["x",1],["y",2]]"#).unwrap();
        assert_eq!(super::j2d_last_row(&j2, "a"), Some(2.0));
        let csv = "# time temp\n1 10\n2 20\n";
        assert_eq!(super::text_last_col(csv, "temp"), Some(20.0));
        assert_eq!(
            super::extract_regex_val(r#"{"totalItems":5,"x":1}"#, r#"("totalItems":...,)"#),
            Some(5.0)
        );
        assert_eq!(
            super::extract_regex_val("<Count>5</Count>", "<Count>([0-9]+)</Count>"),
            Some(5.0)
        );
        assert_eq!(
            super::jcount(&super::parse_json(r"[1,2,3]").unwrap(), "."),
            Some(3.0)
        );
    }

    #[test]
    fn test_anchor_body_agnostic() {
        use std::collections::HashMap;
        let frame = super::Frame::Surface {
            body_name: "mars".into(),
            lat: 0.0,
            lon: 0.0,
            alt: 0.0,
        };
        let src = super::SourceConfig {
            ttl: 3600,
            url: "https://example.com".into(),
            frame,
            format: "json".into(),
            extracts: vec![Extract::Field(FieldConfig {
                key: "v".into(),
                name: "v".into(),
                kernel: 1,
                force: 0,
                tau: 0.0,
                absorption: 0.0,
                advection: 0.0,
                unit: String::new(),
                fold: None,
            })],
            headers: vec![],
            post_body: None,
            target: None,
            catalog: None,
            max_freq: None,
            min_freq: None,
            body: Some("mars".into()),
            stations_url: None,
            stations_path: "stations".into(),
            stations_lat: "lat".into(),
            stations_lon: "lon".into(),
            stations_id: "id".into(),
            flux_from_mag: None,
            abs_mag_from: None,
            catalog_epoch: None,
            repeat_ra_bins: 0,
            fanout_cap: 0,
            stations_flatten: String::new(),
            stations_filter: None,
            fanout_delay: 0,
        };
        let channel = super::Channel {
            z: 0.0,
            freq: 0.0,
            bin_width: 0.0,
            epoch: 0.0,
            position: super::Position::Surface {
                body_name: "mars".into(),
                lat: 14.0,
                lon: 90.0,
                alt: 0.0,
            },
            name: "v".into(),
            value: 1.0,
        };
        let sensor = super::FieldConfig {
            key: "v".into(),
            name: "v".into(),
            kernel: 1,
            force: 0,
            tau: 60.0,
            absorption: 0.0,
            advection: 0.0,
            unit: String::new(),
            fold: None,
        };
        let mut cx: [f64; super::CHEBYSHEV_N] = [0.0; super::CHEBYSHEV_N];
        cx[0] = 1.5e9;
        let props = super::BodyProperties {
            α0_deg: 317.68143,
            dα0_dt_deg_per_century: -0.1061,
            δ0_deg: 52.88650,
            dδ0_dt_deg_per_century: -0.0609,
            w0_deg: 176.630,
            dw_dt_deg_per_day: 350.89198226,
            radius_m: 3389500.0,
            flattening: Some(0.00589),
            gaussian_inverse_square: 0.0,
            gaussian_inverse: 0.0,
            erfc: 0.0,
            patch_levy: 0.0,
            exponential_decay: 0.0,
            gm: None,
            j2: None,
            j4: None,
            radii_b: None,
            radii_c: None,
            nut_ra: None,
            nut_dec: None,
            nutation: None,
            omega_g: None,
        };
        let granule = super::ChebyshevGranule {
            t0_jd: super::J2000_EPOCH,
            dt_jd: 32.0,
            cx,
            cy: [0.0; super::CHEBYSHEV_N],
            cz: [0.0; super::CHEBYSHEV_N],
        };
        let mars_eph = super::BodyEphemeris {
            granules: vec![granule],
            rotation_matrices: vec![],
            props: Some(props),
        };
        let mut eph = HashMap::new();
        eph.insert("mars".to_string(), mars_eph);
        let mut origin_state = super::OriginState {
            fetched: 0.0,
            prev_epoch: 0.0,
            prev_abs: [0.0, 0.0, 0.0],
            prev_motion: None,
            resid_ema: 0.0,
            has_prev: false,
            failures: 0,
            in_flight: false,
        };
        let sample = super::anchor(
            &channel,
            &sensor,
            3600.0,
            Some(0),
            Some(&src.frame),
            Some(&mut origin_state),
            &eph,
        );
        assert!(sample.is_some(), "sample is None");
        let sample = sample.unwrap();
        if let super::Motion::Surface { body_name, .. } = &sample.motion {
            assert_eq!(
                body_name,
                "mars",
                "body name: {}",
                sample
                    .motion
                    .anchor_body()
                    .unwrap_or_else(|| "absent".into())
            );
        } else {
            panic!("motion is Barycenter or Linear, Surface absent");
        }
    }

    #[test]
    fn test_anchor_applies_declared_unit() {
        use std::collections::HashMap;
        let frame = super::Frame::Surface {
            body_name: "mars".into(),
            lat: 0.0,
            lon: 0.0,
            alt: 0.0,
        };
        let src = super::SourceConfig {
            ttl: 3600,
            url: "https://example.com".into(),
            frame,
            format: "json".into(),
            extracts: vec![],
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
        };
        let channel = super::Channel {
            z: 0.0,
            freq: 0.0,
            bin_width: 0.0,
            epoch: 0.0,
            position: super::Position::Surface {
                body_name: "mars".into(),
                lat: 14.0,
                lon: 90.0,
                alt: 0.0,
            },
            name: "imf".into(),
            value: 7.0,
        };
        let sensor = super::FieldConfig {
            key: "bz".into(),
            name: "imf".into(),
            kernel: 1,
            force: 0,
            tau: 60.0,
            absorption: 0.0,
            advection: 0.0,
            unit: "nT".into(),
            fold: None,
        };
        let mut cx: [f64; super::CHEBYSHEV_N] = [0.0; super::CHEBYSHEV_N];
        cx[0] = 1.5e9;
        let props = super::BodyProperties {
            α0_deg: 317.68143,
            dα0_dt_deg_per_century: -0.1061,
            δ0_deg: 52.88650,
            dδ0_dt_deg_per_century: -0.0609,
            w0_deg: 176.630,
            dw_dt_deg_per_day: 350.89198226,
            radius_m: 3389500.0,
            flattening: Some(0.00589),
            gaussian_inverse_square: 0.0,
            gaussian_inverse: 0.0,
            erfc: 0.0,
            patch_levy: 0.0,
            exponential_decay: 0.0,
            gm: None,
            j2: None,
            j4: None,
            radii_b: None,
            radii_c: None,
            nut_ra: None,
            nut_dec: None,
            nutation: None,
            omega_g: None,
        };
        let granule = super::ChebyshevGranule {
            t0_jd: super::J2000_EPOCH,
            dt_jd: 32.0,
            cx,
            cy: [0.0; super::CHEBYSHEV_N],
            cz: [0.0; super::CHEBYSHEV_N],
        };
        let mars_eph = super::BodyEphemeris {
            granules: vec![granule],
            rotation_matrices: vec![],
            props: Some(props),
        };
        let mut eph = HashMap::new();
        eph.insert("mars".to_string(), mars_eph);
        let mut origin_state = super::OriginState {
            fetched: 0.0,
            prev_epoch: 0.0,
            prev_abs: [0.0, 0.0, 0.0],
            prev_motion: None,
            resid_ema: 0.0,
            has_prev: false,
            failures: 0,
            in_flight: false,
        };
        let sample = super::anchor(
            &channel,
            &sensor,
            3600.0,
            Some(0),
            Some(&src.frame),
            Some(&mut origin_state),
            &eph,
        );
        assert!(sample.is_some(), "sample is None");
        let sample = sample.unwrap();
        assert!(
            (sample.val - 7e-9).abs() < 1e-20,
            "nT must convert to Tesla at the anchor, was {}",
            sample.val
        );
    }

    #[test]
    fn test_last_form_captures_unit() {
        let content = "url https://example.com/mag.json\nttl 60\nat sun\nlast bz_gsm imf_bz inverse-square em nT 6.0 0.0 0.0 where satellite 18\n";
        let sources = parse_sources(content);
        assert_eq!(sources.len(), 1);
        match &sources[0].extracts[0] {
            Extract::Last(fc, Some((fk, fv))) => {
                assert_eq!(fc.unit, "nT");
                assert_eq!(fk, "satellite");
                assert_eq!(fv, "18");
            }
            _ => panic!("expected filtered last extract"),
        }
    }

    #[test]
    fn test_convert_luminosity_and_particle_units() {
        let au2 = 1.495978707e11 * 1.495978707e11;
        let v = convert_to_si(1.8e-6, "wm2_1au").unwrap();
        assert!((v - 1.8e-6 * au2).abs() < 1e-6 * 1.8e-6 * au2);
        assert!((convert_to_si(2.0, "pfu").unwrap() - 2.0e4).abs() < 1e-9);
        assert!((convert_to_si(1.0, "1").unwrap() - 1.0).abs() < 1e-12);
        assert!((convert_to_si(1.5, "1e-4w/m2").unwrap() - 1.5e-4).abs() < 1e-16);
    }

    #[test]
    fn test_embedded_lsk_parses_and_covers_now() {
        let lsk = embedded_lsk().expect("the embedded kernel must parse");
        let now_unix = 1.78e9;
        assert_eq!(lsk.leap_at(now_unix), Some(37.0));
        assert!(
            lsk.system_now_tdb().is_some(),
            "the time base exists without any fetch"
        );
    }

    #[test]
    fn test_sense_membrane_delivers_sun_sample_with_zero_floor() {
        use std::collections::HashMap;
        let t = 8.0e8;
        let sample = super::Sample {
            source: super::SampleSource::Source(0),
            epoch: t,
            ttl: 60.0,
            extent: 0.0,
            tau: 6.0,
            kernel_id: 0.0,
            force_type: 0.0,
            absorption: 0.0,
            advection: 0.0,
            anchor_vmax: 0.0,
            anchor_amax: 0.0,
            anchor_p0: [0.0, 0.0, 0.0],
            motion: super::Motion::Linear {
                p: [0.0, 0.0, 0.0],
                v: [0.0, 0.0, 0.0],
            },
            val: 4.0e16,
            name: "sun_xray".into(),
            z: 0.0,
            freq: 0.0,
            bin_width: 0.0,
            color_index: 0.0,
        };
        let cache = super::build_spatial_hash(vec![sample], 1.0);
        let eph: HashMap<String, super::BodyEphemeris> = HashMap::new();
        let buf = super::Buffer {
            cache,
            eph: Arc::new(eph),
            curves: None,
            spectral: Vec::new(),
        };
        let mut records: Vec<super::SampleRecord> = Vec::new();
        super::sense_membrane(
            &buf,
            [0.0, 0.0, 0.0],
            t + 1.0,
            3.0e12,
            1.0,
            &[0.0; 9],
            2.0e9,
            [0.0, 0.0, 1.0],
            &mut records,
            &HashMap::new(),
        );
        assert_eq!(
            records.len(),
            1,
            "the sun sample must reach the window with a zero floor"
        );
        assert_eq!(records[0].3, 4.0e16);
    }

    #[test]
    fn test_parse_ephemeris_binary_v2() {
        let mut buf = Vec::new();
        buf.extend_from_slice(&[0xCF, 0x86, 0x01, 0x00]);
        buf.extend_from_slice(&4u32.to_le_bytes());
        buf.extend_from_slice(&0u32.to_le_bytes());
        buf.extend_from_slice(&1u32.to_le_bytes());
        buf.extend_from_slice(&17u32.to_le_bytes());
        buf.extend_from_slice(&0u32.to_le_bytes());
        for _ in 0..56 {
            buf.extend_from_slice(&0.0_f64.to_le_bytes());
        }
        buf.extend_from_slice(&1u32.to_le_bytes());
        buf.extend_from_slice(&12u32.to_le_bytes());
        buf.extend_from_slice(&17u32.to_le_bytes());
        buf.extend_from_slice(&0u32.to_le_bytes());
        let params: [f64; 12] = [
            270.0,
            0.003,
            66.54,
            0.013,
            38.31,
            14460.0,
            6378136.6,
            6378136.6,
            6356751.9,
            1.08262668e-3,
            -1.6196e-6,
            3.9860043543609598e14,
        ];
        for p in params {
            buf.extend_from_slice(&p.to_le_bytes());
        }
        buf.extend_from_slice(&2u32.to_le_bytes());
        buf.extend_from_slice(&0u32.to_le_bytes());
        buf.extend_from_slice(&17u32.to_le_bytes());
        buf.extend_from_slice(&0u32.to_le_bytes());
        for _ in 0..5 {
            buf.extend_from_slice(&0.0_f64.to_le_bytes());
        }
        buf.extend_from_slice(&4u32.to_le_bytes());
        buf.extend_from_slice(&1u32.to_le_bytes());
        buf.extend_from_slice(&1u32.to_le_bytes());
        buf.extend_from_slice(&0u32.to_le_bytes());
        buf.extend_from_slice(&2451545.0_f64.to_le_bytes());
        buf.extend_from_slice(&16.0_f64.to_le_bytes());
        for c in [1.0_f64, 0.5, 2.0, 0.25, 0.5, 0.125] {
            buf.extend_from_slice(&c.to_le_bytes());
        }
        let eph = super::parse_ephemeris_binary(&buf).unwrap();
        let props = eph.props.unwrap();
        assert_eq!(props.gm, Some(3.9860043543609598e14));
        assert_eq!(props.j2, Some(1.08262668e-3));
        assert_eq!(props.j4, Some(-1.6196e-6));
        assert_eq!(props.radii_b, Some(6378136.6));
        assert_eq!(props.radii_c, Some(6356751.9));
        assert!((props.flattening.unwrap() - (6378136.6 - 6356751.9) / 6378136.6).abs() < 1e-15);
        assert_eq!(eph.granules.len(), 1);
        let deltas = super::nutation_deltas_at(&props, 2451545.0).unwrap();
        assert!((deltas.0 - 1.0).abs() < 1e-12);
        assert!((deltas.1 - 2.0).abs() < 1e-12);
        assert!((deltas.2 - 0.5).abs() < 1e-12);
    }

    #[test]
    fn test_parse_ephemeris_binary_v3_mask() {
        let mut buf = Vec::new();
        buf.extend_from_slice(&[0xCF, 0x86, 0x02, 0x00]);
        buf.extend_from_slice(&3u32.to_le_bytes());
        buf.extend_from_slice(&0u32.to_le_bytes());
        buf.extend_from_slice(&1u32.to_le_bytes());
        buf.extend_from_slice(&17u32.to_le_bytes());
        buf.extend_from_slice(&0u32.to_le_bytes());
        for _ in 0..56 {
            buf.extend_from_slice(&0.0_f64.to_le_bytes());
        }
        buf.extend_from_slice(&1u32.to_le_bytes());
        buf.extend_from_slice(&12u32.to_le_bytes());
        buf.extend_from_slice(&17u32.to_le_bytes());
        buf.extend_from_slice(&0u32.to_le_bytes());
        let mut params: [f64; 12] = [
            270.0,
            0.003,
            66.54,
            0.013,
            38.31,
            14460.0,
            6378136.6,
            6378136.6,
            6356751.9,
            1.08262668e-3,
            -1.6196e-6,
            3.9860043543609598e14,
        ];
        let mask: u16 = 0xFFFF ^ (1 << 9);
        params[9] = 0.0;
        for p in params {
            buf.extend_from_slice(&p.to_le_bytes());
        }
        buf.extend_from_slice(&mask.to_le_bytes());
        buf.extend_from_slice(&[0u8; 6]);
        buf.extend_from_slice(&2u32.to_le_bytes());
        buf.extend_from_slice(&0u32.to_le_bytes());
        buf.extend_from_slice(&17u32.to_le_bytes());
        buf.extend_from_slice(&0u32.to_le_bytes());
        for _ in 0..5 {
            buf.extend_from_slice(&0.0_f64.to_le_bytes());
        }
        let eph = super::parse_ephemeris_binary(&buf).unwrap();
        let props = eph.props.unwrap();
        assert_eq!(props.gm, Some(3.9860043543609598e14));
        assert_eq!(props.j2, None);
        assert_eq!(props.j4, Some(-1.6196e-6));
        assert_eq!(props.radii_c, Some(6356751.9));
        assert!((props.flattening.unwrap() - (6378136.6 - 6356751.9) / 6378136.6).abs() < 1e-15);
        assert_eq!(eph.granules.len(), 1);
    }

    #[test]
    fn test_parse_ephemeris_binary_stype2_medium_constants() {
        let mut buf = Vec::new();
        buf.extend_from_slice(&[0xCF, 0x86, 0x01, 0x00]);
        buf.extend_from_slice(&3u32.to_le_bytes());
        buf.extend_from_slice(&0u32.to_le_bytes());
        buf.extend_from_slice(&1u32.to_le_bytes());
        buf.extend_from_slice(&17u32.to_le_bytes());
        buf.extend_from_slice(&0u32.to_le_bytes());
        for _ in 0..56 {
            buf.extend_from_slice(&0.0_f64.to_le_bytes());
        }
        buf.extend_from_slice(&1u32.to_le_bytes());
        buf.extend_from_slice(&12u32.to_le_bytes());
        buf.extend_from_slice(&17u32.to_le_bytes());
        buf.extend_from_slice(&0u32.to_le_bytes());
        for _ in 0..12 {
            buf.extend_from_slice(&0.0_f64.to_le_bytes());
        }
        buf.extend_from_slice(&2u32.to_le_bytes());
        buf.extend_from_slice(&0u32.to_le_bytes());
        buf.extend_from_slice(&17u32.to_le_bytes());
        buf.extend_from_slice(&0u32.to_le_bytes());
        for &p in &crate::media::medium_params_of("earth")
            .expect("earth row")
            .wire()
        {
            buf.extend_from_slice(&p.to_le_bytes());
        }
        let eph = super::parse_ephemeris_binary(&buf).unwrap();
        let props = eph.props.unwrap();
        assert_eq!(props.gaussian_inverse_square, 340.2);
        assert_eq!(props.gaussian_inverse, 5950.0);
        assert_eq!(props.erfc, 3630.0);
        assert_eq!(props.exponential_decay, 2.18e-5);
        assert_eq!(props.patch_levy, 2.00e-5);
    }

    #[test]
    fn test_parse_ephemeris_binary_stype7_omega_g() {
        let mut buf = Vec::new();
        buf.extend_from_slice(&[0xCF, 0x86, 0x02, 0x00]);
        buf.extend_from_slice(&4u32.to_le_bytes());
        buf.extend_from_slice(&0u32.to_le_bytes());
        buf.extend_from_slice(&1u32.to_le_bytes());
        buf.extend_from_slice(&17u32.to_le_bytes());
        buf.extend_from_slice(&0u32.to_le_bytes());
        for _ in 0..56 {
            buf.extend_from_slice(&0.0_f64.to_le_bytes());
        }
        buf.extend_from_slice(&1u32.to_le_bytes());
        buf.extend_from_slice(&12u32.to_le_bytes());
        buf.extend_from_slice(&17u32.to_le_bytes());
        buf.extend_from_slice(&0u32.to_le_bytes());
        for _ in 0..12 {
            buf.extend_from_slice(&0.0_f64.to_le_bytes());
        }
        buf.extend_from_slice(&(0xFFFFu16).to_le_bytes());
        buf.extend_from_slice(&[0u8; 6]);
        buf.extend_from_slice(&2u32.to_le_bytes());
        buf.extend_from_slice(&0u32.to_le_bytes());
        buf.extend_from_slice(&17u32.to_le_bytes());
        buf.extend_from_slice(&0u32.to_le_bytes());
        for _ in 0..5 {
            buf.extend_from_slice(&0.0_f64.to_le_bytes());
        }
        buf.extend_from_slice(&7u32.to_le_bytes());
        buf.extend_from_slice(&1u32.to_le_bytes());
        buf.extend_from_slice(&0u32.to_le_bytes());
        buf.extend_from_slice(&0u32.to_le_bytes());
        buf.extend_from_slice(&1.277e-6_f64.to_le_bytes());
        buf.extend_from_slice(&2.0e-8_f64.to_le_bytes());
        buf.extend_from_slice(&0.0_f64.to_le_bytes());
        buf.extend_from_slice(&0.0_f64.to_le_bytes());
        buf.extend_from_slice(&0.0_f64.to_le_bytes());
        let eph = super::parse_ephemeris_binary(&buf).unwrap();
        let props = eph.props.unwrap();
        let (omega_g, sigma) = props.omega_g.unwrap();
        assert!((omega_g - 1.277e-6).abs() < 1e-18);
        assert!((sigma - 2.0e-8).abs() < 1e-18);
    }

    #[test]
    fn test_fetch_dispatch_gate_admits_em_source() {
        let fc = FieldConfig {
            key: "flux".into(),
            name: "flux".into(),
            kernel: 0,
            force: 0,
            tau: 60.0,
            absorption: 0.0,
            advection: 0.0,
            unit: String::new(),
            fold: None,
        };
        let reach = super::dispatch_reach(&[fc], 60.0).expect("em carries a propagation law");
        assert_eq!(reach, C_LIGHT * 60.0 * 64.0);
        let presences: Vec<(f64, f64, f64, f64, f64, f64, f64, f64, f64)> =
            vec![(8.0e8, 0.0, 0.0, 0.0, 1.0e12, 0.0, 0.0, 0.0, 0.0)];
        assert!(
            super::presence_gate(&presences, (0.0, 0.0, 0.0), reach),
            "the em source at the presence anchor must be fetched"
        );
    }

    #[test]
    fn test_fetch_dispatch_gate_window_anchor_passes_with_zero_reach() {
        let presences: Vec<(f64, f64, f64, f64, f64, f64, f64, f64, f64)> =
            vec![(8.0e8, 0.0, 0.0, 0.0, 100.0, 0.0, 0.0, 0.0, 0.0)];
        assert!(
            super::presence_gate(&presences, (50.0, 0.0, 0.0), 0.0),
            "an anchor inside the presence window passes on the window range alone"
        );
        assert!(
            !super::presence_gate(&presences, (1.0e6, 0.0, 0.0), 0.0),
            "an anchor outside the window stays refused without a physical reach"
        );
    }

    #[test]
    fn test_fetch_dispatch_gate_thermal_reach_governs_geometry() {
        let fc = FieldConfig {
            key: "temp".into(),
            name: "temp".into(),
            kernel: 3,
            force: 5,
            tau: 60.0,
            absorption: 0.0,
            advection: 0.0,
            unit: String::new(),
            fold: None,
        };
        let reach = super::dispatch_reach(&[fc], 60.0).expect("thermal carries a propagation law");
        assert_eq!(reach, (2.0 * DIFFUSIVITY_THERMAL * 60.0 * 64.0).sqrt());
        let presences: Vec<(f64, f64, f64, f64, f64, f64, f64, f64, f64)> =
            vec![(8.0e8, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0)];
        assert!(
            super::presence_gate(&presences, (10.0, 0.0, 0.0), reach),
            "the thermal front over the sample lifetime reaches 10 m"
        );
        assert!(
            !super::presence_gate(&presences, (1.0e6, 0.0, 0.0), reach),
            "a thermal anchor 1000 km away is out of physical reach"
        );
    }

    #[test]
    fn test_fetch_dispatch_gate_forceless_field_refused() {
        let fc = FieldConfig {
            key: "x".into(),
            name: "x".into(),
            kernel: 0,
            force: 9,
            tau: 60.0,
            absorption: 0.0,
            advection: 0.0,
            unit: String::new(),
            fold: None,
        };
        assert!(
            super::dispatch_reach(&[fc], 60.0).is_none(),
            "a force without a propagation law is refused, never 0.0"
        );
    }

    #[test]
    fn test_extract_fields_reads_profile_map() {
        let fc = field_fixture("temp", 60.0);
        let ext = Extract::ProfileMap {
            arr_path: ".".into(),
            lat_key: "lat".into(),
            lon_key: "lon".into(),
            epoch_key: "t".into(),
            pressure_var: "pressure".into(),
            pressure_scale: 1.0,
            fields: vec![fc.clone()],
        };
        let fields = super::extract_fields(&ext);
        assert_eq!(fields.len(), 1);
        assert_eq!(fields[0].key, "temp");
        let reach = super::dispatch_reach(&fields, 60.0).expect("the profile field gates");
        assert_eq!(reach, C_LIGHT * 60.0 * 64.0);
    }

    #[test]
    fn test_extract_fields_reads_geojson_events() {
        let ext = Extract::GeojsonEvents {
            mag_key: "mag".into(),
            min_mag: 0.0,
            outputs: vec!["seismic_magnitude_mw".into(), "seismic_depth_km".into()],
            tau: 6.0,
            absorption: 0.0,
            advection: 0.0,
            mag_type_key: String::new(),
        };
        let fields = super::extract_fields(&ext);
        assert_eq!(fields.len(), 2);
        assert_eq!(fields[0].name, "seismic_magnitude_mw");
        assert_eq!(fields[0].force, 3);
        assert_eq!(fields[1].name, "seismic_depth_km");
        let reach = super::dispatch_reach(&fields, 60.0).expect("the geojson fields gate");
        assert_eq!(reach, SEISMIC_BODY_SPEED * 60.0 * 64.0);
        let single = Extract::GeojsonEvents {
            mag_key: "mag".into(),
            min_mag: 0.0,
            outputs: vec!["seismic_magnitude_mw".into()],
            tau: 6.0,
            absorption: 0.0,
            advection: 0.0,
            mag_type_key: String::new(),
        };
        assert!(
            super::extract_fields(&single).is_empty(),
            "a geojson extract with one output emits nothing"
        );
    }

    #[test]
    fn test_fetch_dispatch_gate_advective_uses_field_advection() {
        let fc = FieldConfig {
            key: "wind".into(),
            name: "wind".into(),
            kernel: 5,
            force: 7,
            tau: 60.0,
            absorption: 0.0,
            advection: 400000.0,
            unit: String::new(),
            fold: None,
        };
        let reach =
            super::dispatch_reach(&[fc], 60.0).expect("advective carries a propagation law");
        assert_eq!(reach, 400000.0 * 60.0 * 64.0);
    }

    fn origin_fixture(failures: u32, in_flight: bool) -> super::OriginState {
        super::OriginState {
            fetched: 0.0,
            prev_epoch: 0.0,
            prev_abs: [0.0, 0.0, 0.0],
            prev_motion: None,
            resid_ema: 0.0,
            has_prev: false,
            failures,
            in_flight,
        }
    }

    #[test]
    fn test_origin_stale_holds_while_fetch_in_flight() {
        let mut origins = std::collections::HashMap::new();
        origins.insert(0u32, origin_fixture(0, true));
        assert!(
            !super::origin_stale(&origins, 0, 60, 1.0e9),
            "a running fetch blocks the re-dispatch, however stale the origin"
        );
        origins.insert(0u32, origin_fixture(0, false));
        assert!(
            super::origin_stale(&origins, 0, 60, 1.0e9),
            "a settled stale origin dispatches again"
        );
    }

    #[test]
    fn test_fetch_void_backoff_grows_power_of_two_and_caps() {
        for failures in 0..=super::FETCH_VOID_CAP + 2 {
            let mut origins = std::collections::HashMap::new();
            origins.insert(0u32, origin_fixture(failures, false));
            let factor = 2f64.powi(failures.min(super::FETCH_VOID_CAP) as i32);
            let backoff = 60.0 / super::Φ * factor;
            assert!(
                !super::origin_stale(&origins, 0, 60, backoff - 0.5),
                "failures {}: fresh inside the backoff stays held",
                failures
            );
            assert!(
                super::origin_stale(&origins, 0, 60, backoff + 0.5),
                "failures {}: the backoff ttl/Φ·2ⁿ expires",
                failures
            );
        }
    }

    #[test]
    fn test_settle_fetch_resets_voids_on_ok_and_caps_on_void() {
        let mut st = origin_fixture(3, true);
        super::settle_fetch(&mut st, true, 100.0);
        assert_eq!(st.failures, 0, "a delivered fetch resets the void count");
        assert!(!st.in_flight);
        assert_eq!(st.fetched, 100.0);
        super::settle_fetch(&mut st, false, 200.0);
        assert_eq!(st.failures, 1, "a fetch void counts one failure");
        st.failures = super::FETCH_VOID_CAP;
        super::settle_fetch(&mut st, false, 300.0);
        assert_eq!(
            st.failures,
            super::FETCH_VOID_CAP,
            "the void count caps at the power-of-2 ceiling"
        );
    }

    #[test]
    fn test_anchor_bodies_have_ephemeris_sources() {
        let content = std::fs::read_to_string("phi/sources.φ").unwrap();
        let sources = super::parse_sources(&content);
        let uses = super::anchor_uses(&sources);
        let eph_bodies: std::collections::HashSet<String> = sources
            .iter()
            .filter(|s| s.format == "ephemeris_binary")
            .filter_map(|s| s.body.clone())
            .collect();
        let missing: Vec<&String> = uses.keys().filter(|b| !eph_bodies.contains(*b)).collect();
        assert!(
            missing.is_empty(),
            "anchor bodies without an ephemeris source would starve the load gate: {:?}",
            missing
        );
    }

    #[test]
    fn test_query_admits_surface_sample_within_window() {
        let now = 840511523.88;
        let jd_now = super::J2000_EPOCH + now / 86400.0;
        let props = super::BodyProperties {
            α0_deg: 270.0,
            dα0_dt_deg_per_century: 0.003,
            δ0_deg: 66.54,
            dδ0_dt_deg_per_century: 0.013,
            w0_deg: 190.147,
            dw_dt_deg_per_day: 360.9856235,
            radius_m: 6378136.6,
            flattening: Some((6378136.6 - 6356751.9) / 6378136.6),
            gaussian_inverse_square: 340.2,
            gaussian_inverse: 5950.0,
            erfc: 3630.0,
            exponential_decay: 2.18e-5,
            patch_levy: 2.00e-5,
            gm: Some(3.986004418e14),
            j2: Some(1.08262668e-3),
            j4: Some(-1.619e-6),
            radii_b: Some(6378136.6),
            radii_c: Some(6356751.9),
            nut_ra: None,
            nut_dec: None,
            nutation: None,
            omega_g: None,
        };
        let mut eph = super::BodyEphemeris {
            granules: Vec::new(),
            rotation_matrices: Vec::new(),
            props: Some(props),
        };
        for i in -1..=1 {
            let t0 = jd_now + i as f64 * 16.0;
            let mut cx = [0.0_f64; super::CHEBYSHEV_N];
            cx[0] = 1.5e11;
            eph.granules.push(super::ChebyshevGranule {
                t0_jd: t0,
                dt_jd: 16.0,
                cx,
                cy: [0.0; super::CHEBYSHEV_N],
                cz: [0.0; super::CHEBYSHEV_N],
            });
        }
        let mut eph_map = std::collections::HashMap::new();
        eph_map.insert("earth".to_string(), eph);
        let pos = super::body_barycenter_position("earth", now, &eph_map).expect("earth pos");
        let channel = Channel {
            z: 0.0,
            freq: 0.0,
            bin_width: 0.0,
            name: "argo_dac_temp_c".into(),
            value: 25.0,
            position: Position::Surface {
                body_name: "earth".into(),
                lat: 0.0,
                lon: 0.0,
                alt: 0.0,
            },
            epoch: now - 2.15e6,
        };
        let sensor = FieldConfig {
            key: "TEMP".into(),
            name: "argo_dac_temp_c".into(),
            kernel: 3,
            force: 5,
            tau: 604800.0,
            absorption: 0.0,
            advection: 0.0,
            unit: "C".into(),
            fold: None,
        };
        let frame = Frame::Surface {
            body_name: "earth".into(),
            lat: 0.0,
            lon: 0.0,
            alt: 0.0,
        };
        let sample = super::anchor(
            &channel,
            &sensor,
            604800.0,
            Some(157),
            Some(&frame),
            None,
            &eph_map,
        )
        .expect("argo sample anchors");
        let hash = super::build_spatial_hash(vec![sample.clone()], 1.0);
        let mut recs = Vec::new();
        super::query_hash(
            &hash,
            pos,
            now,
            8.0e6,
            0.0,
            &[1.0e-300_f64; 9],
            1.0,
            [0.0, 0.0, 0.0],
            &mut recs,
            &eph_map,
        );
        assert!(
            !recs.is_empty(),
            "the surface sample within the window pad must reach the membrane, got {} records",
            recs.len()
        );
        let mut recs_ssb = Vec::new();
        super::query_hash(
            &hash,
            [0.0, 0.0, 0.0],
            now,
            2.0e12,
            0.0,
            &[1.0e-300_f64; 9],
            1.0,
            [0.0, 0.0, 0.0],
            &mut recs_ssb,
            &eph_map,
        );
        assert!(
            !recs_ssb.is_empty(),
            "the earth surface sample within the boot window must reach the SSB presence"
        );
    }

    #[test]
    fn test_parse_ephemeris_binary_rejects_non_v2_props() {
        let mut buf = Vec::new();
        buf.extend_from_slice(&[0xCF, 0x86, 0x01, 0x00]);
        buf.extend_from_slice(&3u32.to_le_bytes());
        buf.extend_from_slice(&0u32.to_le_bytes());
        buf.extend_from_slice(&1u32.to_le_bytes());
        buf.extend_from_slice(&17u32.to_le_bytes());
        buf.extend_from_slice(&0u32.to_le_bytes());
        for _ in 0..56 {
            buf.extend_from_slice(&0.0_f64.to_le_bytes());
        }
        buf.extend_from_slice(&1u32.to_le_bytes());
        buf.extend_from_slice(&0u32.to_le_bytes());
        buf.extend_from_slice(&17u32.to_le_bytes());
        buf.extend_from_slice(&0u32.to_le_bytes());
        for _ in 0..8 {
            buf.extend_from_slice(&0.0_f64.to_le_bytes());
        }
        assert!(super::parse_ephemeris_binary(&buf).is_none());
    }

    #[test]
    fn test_parse_ephemeris_binary_rejects_truncated_granules() {
        let mut buf = Vec::new();
        buf.extend_from_slice(&[0xCF, 0x86, 0x01, 0x00]);
        buf.extend_from_slice(&2u32.to_le_bytes());
        buf.extend_from_slice(&0u32.to_le_bytes());
        buf.extend_from_slice(&2u32.to_le_bytes());
        buf.extend_from_slice(&17u32.to_le_bytes());
        buf.extend_from_slice(&0u32.to_le_bytes());
        for _ in 0..56 {
            buf.extend_from_slice(&0.0_f64.to_le_bytes());
        }
        assert!(super::parse_ephemeris_binary(&buf).is_none());
    }

    #[test]
    fn test_parse_ephemeris_binary_rejects_truncated_props() {
        let mut buf = Vec::new();
        buf.extend_from_slice(&[0xCF, 0x86, 0x01, 0x00]);
        buf.extend_from_slice(&2u32.to_le_bytes());
        buf.extend_from_slice(&0u32.to_le_bytes());
        buf.extend_from_slice(&1u32.to_le_bytes());
        buf.extend_from_slice(&17u32.to_le_bytes());
        buf.extend_from_slice(&0u32.to_le_bytes());
        for _ in 0..56 {
            buf.extend_from_slice(&0.0_f64.to_le_bytes());
        }
        buf.extend_from_slice(&1u32.to_le_bytes());
        buf.extend_from_slice(&12u32.to_le_bytes());
        buf.extend_from_slice(&17u32.to_le_bytes());
        buf.extend_from_slice(&0u32.to_le_bytes());
        for _ in 0..6 {
            buf.extend_from_slice(&0.0_f64.to_le_bytes());
        }
        assert!(super::parse_ephemeris_binary(&buf).is_none());
    }

    #[test]
    fn test_live_sources_extract() {
        let srcs = super::load_sources();
        eprintln!("load_sources returned {} sources", srcs.len());
        let fixture_lsk = full_fixture_lsk();
        let now = fixture_lsk.system_now_tdb().unwrap();
        let env = super::load_env();
        let (ok, findings) = super::live_sweep(&env, now, &fixture_lsk, 600);
        eprintln!(
            "\n=== LIVE SOURCE EXTRACTION: {} ok, {} void (of {} tested) ===",
            ok,
            findings.len(),
            ok + findings.len()
        );
        for f in findings.iter() {
            eprintln!("  void {}  {}  {}", f.class.as_str(), f.url, f.detail);
        }
    }

    #[test]
    fn test_diagnose_no_samples() {
        let base = super::SourceConfig {
            ttl: 60,
            url: "https://example.com/q".into(),
            frame: super::Frame::Surface {
                body_name: "earth".into(),
                lat: 0.0,
                lon: 0.0,
                alt: 0.0,
            },
            format: "json".into(),
            extracts: vec![super::Extract::Map {
                arr_path: "features".into(),
                lat_key: "geometry.coordinates.1".into(),
                lon_key: "geometry.coordinates.0".into(),
                alt_key: String::new(),
                epoch_key: String::new(),
                val_key: String::new(),
                alt_scale: 1.0,
                vel_key: String::new(),
                vel_scale: 1.0,
                trk_key: String::new(),
                vr_key: String::new(),
                fields: vec![FieldConfig {
                    key: "properties.mag".into(),
                    name: "mag".into(),
                    kernel: 0,
                    force: 0,
                    tau: 0.0,
                    absorption: 0.0,
                    advection: 0.0,
                    unit: String::new(),
                    fold: None,
                }],
                lat_sign: None,
                lon_sign: None,
                epoch_scale: 1.0,
                tau_key: String::new(),
                mag_type_key: String::new(),
            }],
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
        };
        let empty_geojson =
            r#"{"type":"FeatureCollection","metadata":{"api":"2.7","count":0},"features":[]}"#;
        let d_empty = super::diagnose_no_samples(&base, empty_geojson);
        eprintln!("empty geojson -> {}", d_empty);
        assert!(d_empty.contains("empty-response"), "got: {}", d_empty);

        let filled_geojson = r#"{"type":"FeatureCollection","features":[{"geometry":{"coordinates":[-104.0,39.5,10.0]},"properties":{"mag":3.2}}]}"#;
        let d_filled = super::diagnose_no_samples(&base, filled_geojson);
        eprintln!("filled geojson -> {}", d_filled);
        assert!(d_filled.contains("data-present"), "got: {}", d_filled);

        let html = "<html>GraceDB down</html>";
        let d_html = super::diagnose_no_samples(&base, html);
        eprintln!("html -> {}", d_html);
        assert!(d_html.contains("non-JSON"), "got: {}", d_html);
    }

    #[test]
    fn test_refusal_ledger_dedup_and_reload() {
        let path =
            std::env::temp_dir().join(format!("omegaflow_refusal_ledger_{}.φ", std::process::id()));
        let _ = std::fs::remove_file(&path);
        {
            let mut ledger = super::RefusalLedger::new(path.to_str().unwrap());
            ledger.register("https://a.example/q", "extract-void");
            ledger.register("https://a.example/q", "extract-void");
            ledger.register("https://b.example/q", "fetch-void");
        }
        let content = std::fs::read_to_string(&path).unwrap();
        assert_eq!(content.lines().count(), 2, "one entry per class+url");
        assert!(content.contains("extract-void https://a.example/q"));
        assert!(content.contains("fetch-void https://b.example/q"));
        {
            let mut ledger = super::RefusalLedger::new(path.to_str().unwrap());
            ledger.register("https://a.example/q", "extract-void");
        }
        let reloaded = std::fs::read_to_string(&path).unwrap();
        assert_eq!(
            reloaded.lines().count(),
            2,
            "a reloaded ledger never repeats an entry"
        );
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_map_single_object_alt_scale_epoch_default() {
        let src = super::SourceConfig {
            ttl: 10,
            url: "https://api.wheretheiss.at/v1/satellites/25544".into(),
            frame: super::Frame::Barycenter {
                body_name: "earth".into(),
                scale: 1.0,
            },
            format: "json".into(),
            extracts: vec![super::Extract::Map {
                arr_path: ".".into(),
                lat_key: "latitude".into(),
                lon_key: "longitude".into(),
                alt_key: "altitude".into(),
                epoch_key: String::new(),
                val_key: String::new(),
                alt_scale: 1000.0,
                vel_key: String::new(),
                vel_scale: 1.0,
                trk_key: String::new(),
                vr_key: String::new(),
                fields: vec![FieldConfig {
                    key: "velocity".into(),
                    name: "velocity".into(),
                    kernel: 0,
                    force: 0,
                    tau: 1.0,
                    absorption: 0.0,
                    advection: 0.0,
                    unit: String::new(),
                    fold: None,
                }],
                lat_sign: None,
                lon_sign: None,
                epoch_scale: 1.0,
                tau_key: String::new(),
                mag_type_key: String::new(),
            }],
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
        };
        let body = r#"{"latitude":-47.75,"longitude":78.87,"altitude":438.28,"velocity":27528.0}"#;
        let now = 8.4e8;
        let fixture_lsk = super::LeapSeconds {
            delta_t_a: 32.184,
            deltas: vec![(37.0, 1483228800.0)],
        };
        match super::extract(&src, body, now, &fixture_lsk) {
            super::ExtractResult::Measurements(v) => {
                assert_eq!(v.len(), 1);
                let (ch, fc) = &v[0];
                assert_eq!(ch.epoch, now);
                assert_eq!(ch.value, 27528.0);
                assert_eq!(fc.tau, 1.0);
                match &ch.position {
                    super::Position::Surface { lat, lon, alt, .. } => {
                        assert!((lat - -47.75).abs() < 1e-9);
                        assert!((lon - 78.87).abs() < 1e-9);
                        assert!((alt - 438280.0).abs() < 1e-6);
                    }
                    other => panic!("position variant: {:?} unexpected", other),
                }
            }
            _ => panic!("extract variant unexpected"),
        }
    }

    #[test]
    fn test_map_vel_unit_and_tau_key_override() {
        let src = super::SourceConfig {
            ttl: 10,
            url: "https://example.org/flow".into(),
            frame: super::Frame::Surface {
                body_name: "earth".into(),
                lat: 0.0,
                lon: 0.0,
                alt: 0.0,
            },
            format: "json".into(),
            extracts: vec![super::Extract::Map {
                arr_path: "data".into(),
                lat_key: "lat".into(),
                lon_key: "lon".into(),
                alt_key: "alt".into(),
                epoch_key: String::new(),
                val_key: String::new(),
                alt_scale: 1.0,
                vel_key: "spd".into(),
                vel_scale: 1.0 / 3.6,
                trk_key: "hdg".into(),
                vr_key: "vr".into(),
                fields: vec![FieldConfig {
                    key: "v".into(),
                    name: "flow_value".into(),
                    kernel: 0,
                    force: 0,
                    tau: 7.0,
                    absorption: 0.0,
                    advection: 0.0,
                    unit: String::new(),
                    fold: None,
                }],
                lat_sign: None,
                lon_sign: None,
                epoch_scale: 1.0,
                tau_key: "row_tau".into(),
                mag_type_key: String::new(),
            }],
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
        };
        let body = r#"{"data":[
                {"lat":10.0,"lon":20.0,"alt":0.0,"spd":72.0,"hdg":90.0,"vr":3.6,"row_tau":60.0,"v":5.0},
                {"lat":11.0,"lon":21.0,"alt":0.0,"spd":36.0,"hdg":0.0,"vr":1.8,"row_tau":0.0,"v":5.0},
                {"lat":12.0,"lon":22.0,"alt":0.0,"spd":18.0,"hdg":270.0,"vr":1.0,"v":5.0}
            ]}"#;
        let now = 8.4e8;
        let fixture_lsk = super::LeapSeconds {
            delta_t_a: 32.184,
            deltas: vec![(37.0, 1483228800.0)],
        };
        match super::extract(&src, body, now, &fixture_lsk) {
            super::ExtractResult::Measurements(v) => {
                assert_eq!(v.len(), 2);
                let (ch0, fc0) = &v[0];
                match &ch0.position {
                    super::Position::SurfaceFlow {
                        speed,
                        track,
                        vrate,
                        ..
                    } => {
                        assert!((speed - 20.0).abs() < 1e-9);
                        assert!((track - 90.0).abs() < 1e-9);
                        assert!((vrate.unwrap() - 1.0).abs() < 1e-9);
                    }
                    other => panic!("expected SurfaceFlow, got {:?}", other),
                }
                assert!((fc0.tau - 60.0).abs() < 1e-9);
                let (ch1, fc1) = &v[1];
                match &ch1.position {
                    super::Position::SurfaceFlow { speed, vrate, .. } => {
                        assert!((speed - 5.0).abs() < 1e-9);
                        assert!((vrate.unwrap() - 1.0 / 3.6).abs() < 1e-9);
                    }
                    other => panic!("expected SurfaceFlow, got {:?}", other),
                }
                assert!((fc1.tau - 7.0).abs() < 1e-9);
            }
            _ => panic!("extract variant unexpected"),
        }
    }

    #[test]
    fn test_parse_spectral_block() {
        let block = "url https://github.com/omegaflow/sources/releases/download/ssd.jpl.nasa.gov/spectra.bin\nformat spectral\non earth 19.82 -155.47 0\nttl 86400\nfield irradiance spectral_irradiance_W_m2_Hz inverse-square em W/m2/Hz 2628000 0.0 0.0\n";
        let srcs = super::parse_sources(block);
        assert_eq!(srcs.len(), 1);
        assert_eq!(srcs[0].format, "spectral");
        match &srcs[0].frame {
            super::Frame::Surface {
                body_name,
                lat,
                lon,
                alt,
            } => {
                assert_eq!(body_name, "earth");
                assert!((*lat - 19.82).abs() < 1e-12);
                assert!((*lon + 155.47).abs() < 1e-12);
                assert_eq!(*alt, 0.0);
            }
            other => {
                let _ = other;
                panic!("expected Surface frame")
            }
        }
        match &srcs[0].extracts[0] {
            super::Extract::Field(fc) => {
                assert_eq!(fc.name, "spectral_irradiance_W_m2_Hz");
                assert_eq!(fc.unit, "W/m2/Hz");
                assert!((fc.tau - 2628000.0).abs() < 1e-9);
                assert_eq!(fc.force as u32, 0);
                assert_eq!(fc.kernel as u32, 0);
            }
            other => {
                let _ = other;
                panic!("expected Field extract")
            }
        }
    }

    #[test]
    fn test_parse_vel_unit_and_tau_key_directives() {
        let block = "url https://example.org/flow\nttl 3600\nformat json\non earth 0 0 0\nmap data\nlat lat\nlon lon\nvel spd km/h\ntau_key row_tau\nfield v flow_value inverse-square thermal W 10 0.0 0.0\n";
        let srcs = super::parse_sources(block);
        assert_eq!(srcs.len(), 1);
        match &srcs[0].extracts[0] {
            super::Extract::Map {
                vel_key,
                vel_scale,
                tau_key,
                fields,
                ..
            } => {
                assert_eq!(vel_key, "spd");
                assert!((*vel_scale - 1.0 / 3.6).abs() < 1e-12);
                assert_eq!(tau_key, "row_tau");
                assert_eq!(fields.len(), 1);
                assert_eq!(fields[0].unit, "W");
            }
            other => {
                let _ = other;
                panic!("expected Map extract")
            }
        }
        let cmap_block =
                "url https://example.org/c\nttl 3600\nformat json\nat sun\ncmap data\nra ra\ndec dec\ntau_key tkey\n";
        let csrcs = super::parse_sources(cmap_block);
        assert_eq!(csrcs.len(), 1);
        match &csrcs[0].extracts[0] {
            super::Extract::CelestialMap { tau_key, .. } => assert_eq!(tau_key, "tkey"),
            other => {
                let _ = other;
                panic!("expected CelestialMap extract")
            }
        }
        let rows_block = "url https://example.org/r\nttl 3600\nformat text\non earth 1 2 0\nrows\nlast_line true\ntau_key rtau\nlastrow T val thermal K 10 0 0\n";
        let rsrcs = super::parse_sources(rows_block);
        assert_eq!(rsrcs.len(), 1);
        match &rsrcs[0].extracts[0] {
            super::Extract::Rows { tau_key, .. } => assert_eq!(tau_key, "rtau"),
            other => {
                let _ = other;
                panic!("expected Rows extract")
            }
        }
    }

    #[test]
    fn test_fold_directive_parse_and_extract() {
        let block = "url https://example.org/f\nttl 3600\nformat json\non earth 0 0 0\nmap data\nlat lat\nlon lon\nfold mean nh sh diffusion ppm 100\nfold diff nh sh diffusion ppm 100\n";
        let srcs = super::parse_sources(block);
        assert_eq!(srcs.len(), 1);
        let fields = match &srcs[0].extracts[0] {
            super::Extract::Map { fields, .. } => fields,
            other => {
                let _ = other;
                panic!("expected Map extract")
            }
        };
        assert_eq!(fields.len(), 2);
        assert_eq!(fields[0].name, "fold_mean_nh_sh");
        assert!(matches!(&fields[0].fold, Some((1, b)) if b == "sh"));
        assert!(matches!(&fields[1].fold, Some((2, b)) if b == "sh"));
        assert!((fields[0].tau - 100.0).abs() < 1e-12);
        assert_eq!(fields[0].unit, "ppm");
        let refused =
            "url https://example.org/f\nttl 3600\nformat json\nat sun\nfold mean a b em mag 100\n";
        let rsrcs = super::parse_sources(refused);
        assert_eq!(rsrcs.len(), 1);
        assert!(rsrcs[0].extracts.is_empty());

        let src = super::SourceConfig {
            ttl: 10,
            url: "https://example.org/f".into(),
            frame: super::Frame::Surface {
                body_name: "earth".into(),
                lat: 0.0,
                lon: 0.0,
                alt: 0.0,
            },
            format: "json".into(),
            extracts: vec![super::Extract::Map {
                arr_path: "data".into(),
                lat_key: "lat".into(),
                lon_key: "lon".into(),
                alt_key: "alt".into(),
                epoch_key: String::new(),
                val_key: String::new(),
                alt_scale: 1.0,
                vel_key: String::new(),
                vel_scale: 1.0,
                trk_key: String::new(),
                vr_key: String::new(),
                fields: vec![
                    FieldConfig {
                        key: "nh".into(),
                        name: "fold_mean_nh_sh".into(),
                        kernel: 0,
                        force: 6,
                        tau: 100.0,
                        absorption: 0.0,
                        advection: 0.0,
                        unit: "ppm".into(),
                        fold: Some((1, "sh".into())),
                    },
                    FieldConfig {
                        key: "nh".into(),
                        name: "fold_diff_nh_sh".into(),
                        kernel: 0,
                        force: 6,
                        tau: 100.0,
                        absorption: 0.0,
                        advection: 0.0,
                        unit: "ppm".into(),
                        fold: Some((2, "sh".into())),
                    },
                ],
                lat_sign: None,
                lon_sign: None,
                epoch_scale: 1.0,
                tau_key: String::new(),
                mag_type_key: String::new(),
            }],
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
        };
        let body = r#"{"data":[
                {"lat":1.0,"lon":2.0,"alt":0.0,"nh":420.0,"sh":410.0},
                {"lat":3.0,"lon":4.0,"alt":0.0,"nh":420.0}
            ]}"#;
        let now = 8.4e8;
        let fixture_lsk = super::LeapSeconds {
            delta_t_a: 32.184,
            deltas: vec![(37.0, 1483228800.0)],
        };
        match super::extract(&src, body, now, &fixture_lsk) {
            super::ExtractResult::Measurements(v) => {
                assert_eq!(v.len(), 2);
                assert!((v[0].0.value - 415.0).abs() < 1e-9);
                assert!((v[1].0.value - 10.0).abs() < 1e-9);
            }
            _ => panic!("extract variant unexpected"),
        }
    }

    #[test]
    fn test_keplermap_elements_to_icrs() {
        let block = "url https://example.org/k\nttl 3600\nformat json\nat sun\nkeplermap data a e i\nom om\nw w\nma ma\nepoch epoch\nqr q\ntp tp\nfield H abs_mag inverse-square em mag 100 0 0\n";
        let srcs = super::parse_sources(block);
        assert_eq!(srcs.len(), 1);
        match &srcs[0].extracts[0] {
            super::Extract::KeplerMap {
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
                ..
            } => {
                assert_eq!(a_key, "a");
                assert_eq!(e_key, "e");
                assert_eq!(i_key, "i");
                assert_eq!(om_key, "om");
                assert_eq!(w_key, "w");
                assert_eq!(ma_key, "ma");
                assert_eq!(epoch_key, "epoch");
                assert_eq!(q_key, "q");
                assert_eq!(tp_key, "tp");
                assert_eq!(fields.len(), 1);
            }
            other => {
                let _ = other;
                panic!("expected KeplerMap extract")
            }
        }
        let mk_src = |a_key: &str, ma_key: &str, q_key: &str, tp_key: &str| super::SourceConfig {
            ttl: 10,
            url: "https://example.org/k".into(),
            frame: super::Frame::Barycenter {
                body_name: "sun".into(),
                scale: 1.0,
            },
            format: "json".into(),
            extracts: vec![super::Extract::KeplerMap {
                arr_path: "data".into(),
                a_key: a_key.into(),
                e_key: "e".into(),
                i_key: "i".into(),
                om_key: "om".into(),
                w_key: "w".into(),
                ma_key: ma_key.into(),
                epoch_key: "epoch".into(),
                q_key: q_key.into(),
                tp_key: tp_key.into(),
                fields: vec![FieldConfig {
                    key: "H".into(),
                    name: "abs_mag".into(),
                    kernel: 0,
                    force: 0,
                    tau: 100.0,
                    absorption: 0.0,
                    advection: 0.0,
                    unit: String::new(),
                    fold: None,
                }],
            }],
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
        };
        let au = 1.495978707e11;
        let expect_v = (1.32712440018e20_f64 / au).sqrt();
        let now = 8.4e8;
        let fixture_lsk = super::LeapSeconds {
            delta_t_a: 32.184,
            deltas: vec![(37.0, 1483228800.0)],
        };
        let body_ma = r#"{"data":[{"a":1.0,"e":0.0,"i":0.0,"om":0.0,"w":0.0,"ma":0.0,"epoch":2451545.0,"H":12.0}]}"#;
        let src_ma = mk_src("a", "ma", "", "");
        match super::extract(&src_ma, body_ma, now, &fixture_lsk) {
            super::ExtractResult::Measurements(v) => {
                assert_eq!(v.len(), 1);
                match &v[0].0.position {
                    super::Position::StateVector { p, v: vel, .. } => {
                        let r = (p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt();
                        let sp = (vel[0] * vel[0] + vel[1] * vel[1] + vel[2] * vel[2]).sqrt();
                        assert!((r - au).abs() < au * 1e-6);
                        assert!((sp - expect_v).abs() < 1.0);
                    }
                    other => {
                        let _ = other;
                        panic!("expected StateVector position")
                    }
                }
                assert!((v[0].0.value - 12.0).abs() < 1e-9);
            }
            _ => panic!("extract variant unexpected"),
        }
        let body_tp = r#"{"data":[{"q":1.0,"e":0.0,"i":0.0,"om":0.0,"w":0.0,"tp":2451545.0,"epoch":2451545.0,"H":12.0}]}"#;
        let src_tp = mk_src("", "", "q", "tp");
        match super::extract(&src_tp, body_tp, now, &fixture_lsk) {
            super::ExtractResult::Measurements(v) => {
                assert_eq!(v.len(), 1);
                match &v[0].0.position {
                    super::Position::StateVector { p, .. } => {
                        let r = (p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt();
                        assert!((r - au).abs() < au * 1e-6);
                    }
                    other => {
                        let _ = other;
                        panic!("expected StateVector position")
                    }
                }
            }
            _ => panic!("extract variant unexpected"),
        }
    }

    #[test]
    fn test_field_in_nested_port_and_flatten_generic() {
        let legacy = "source geosphere\nttl 86400\nforce seismic-body\nurl https://example.org/g\nmap data\nlat_key lat\nlon_key lon\nfield_in geometry.coordinates.2 quake_depth\nfield_in properties.mag quake_mag\n";
        let conv = super::port_block(legacy);
        let srcs = super::parse_sources(&conv);
        assert_eq!(srcs.len(), 0);

        let src = super::SourceConfig {
            ttl: 10,
            url: "https://example.org/f".into(),
            frame: super::Frame::Surface {
                body_name: "earth".into(),
                lat: 0.0,
                lon: 0.0,
                alt: 0.0,
            },
            format: "json".into(),
            extracts: vec![super::Extract::Flatten {
                arr_path: "rows".into(),
                geom_path: "pts".into(),
                epoch_key: "t".into(),
                fields: vec![FieldConfig {
                    key: "v".into(),
                    name: "v".into(),
                    kernel: 0,
                    force: 0,
                    tau: 10.0,
                    absorption: 0.0,
                    advection: 0.0,
                    unit: String::new(),
                    fold: None,
                }],
            }],
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
        };
        let body = r#"{"rows":[
                {"t":1000000.0,"pts":[[[10.0,20.0,5.0],[11.0,21.0,6.0]],[[12.0,22.0,7.0]]],"v":3.5},
                {"t":2000000.0,"pts":[30.0,40.0,8.0],"v":2.5}
            ]}"#;
        let now = 8.4e8;
        let fixture_lsk = super::LeapSeconds {
            delta_t_a: 32.184,
            deltas: vec![(37.0, 1483228800.0)],
        };
        match super::extract(&src, body, now, &fixture_lsk) {
            super::ExtractResult::Measurements(v) => {
                assert_eq!(v.len(), 4);
                for (ch, _) in &v {
                    match &ch.position {
                        super::Position::Surface { lat, lon, .. } => {
                            assert!(*lat >= 20.0 && *lat <= 40.0);
                            assert!(*lon >= 10.0 && *lon <= 30.0);
                        }
                        other => {
                            let _ = other;
                            panic!("expected Surface position")
                        }
                    }
                }
            }
            _ => panic!("extract variant unexpected"),
        }
    }

    #[test]
    fn test_flux_from_mag_manifests() {
        let src = super::SourceConfig {
            ttl: 10,
            url: "https://example.org/cat".into(),
            frame: super::Frame::Barycenter {
                body_name: "sun".into(),
                scale: 1.0,
            },
            format: "json".into(),
            extracts: vec![super::Extract::CelestialMap {
                arr_path: ".".into(),
                ra_key: "ra".into(),
                dec_key: "dec".into(),
                dist_key: String::new(),
                dist_scale: 1.0,
                plx_key: "plx".into(),
                z_key: String::new(),
                pmra_key: String::new(),
                pmdec_key: String::new(),
                rv_key: String::new(),
                rv_scale: 1.0,
                epoch_key: String::new(),
                fields: vec![FieldConfig {
                    key: "mag".into(),
                    name: "cat_vmag".into(),
                    kernel: 0,
                    force: 0,
                    tau: 100.0,
                    absorption: 0.0,
                    advection: 0.0,
                    unit: "mag".into(),
                    fold: None,
                }],
                tau_key: String::new(),
            }],
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
            flux_from_mag: Some("mag".into()),
            abs_mag_from: None,
            catalog_epoch: None,
            repeat_ra_bins: 0,
            fanout_cap: 0,
            stations_flatten: String::new(),
            stations_filter: None,
            fanout_delay: 0,
        };
        let body = r#"[{"ra":89.8,"dec":53.6,"mag":12.0,"plx":10.0}]"#;
        let now = 8.0e8;
        let fixture_lsk = super::LeapSeconds {
            delta_t_a: 32.184,
            deltas: vec![(37.0, 1483228800.0)],
        };
        match super::extract(&src, body, now, &fixture_lsk) {
            super::ExtractResult::Measurements(v) => {
                assert_eq!(v.len(), 1);
                let expect = 10.0f64.powf(-0.4 * 12.0);
                assert!((v[0].0.value - expect).abs() < 1e-12);
                assert_eq!(v[0].1.unit, "");
            }
            _ => panic!("extract variant unexpected"),
        }
    }

    #[test]
    fn test_map_lat_sign_lon_sign() {
        let src = SourceConfig {
            ttl: 3600,
            url: "https://example.com/fireball".into(),
            frame: super::Frame::Surface {
                body_name: "earth".into(),
                lat: 0.0,
                lon: 0.0,
                alt: 0.0,
            },
            format: "json".into(),
            extracts: vec![Extract::Map {
                arr_path: "data".into(),
                lat_key: "3".into(),
                lon_key: "5".into(),
                alt_key: "7".into(),
                epoch_key: "0".into(),
                val_key: String::new(),
                alt_scale: 1000.0,
                vel_key: String::new(),
                vel_scale: 1.0,
                trk_key: String::new(),
                vr_key: String::new(),
                fields: vec![FieldConfig {
                    key: "1".into(),
                    name: "fireball_energy_e10j".into(),
                    kernel: 0,
                    force: 0,
                    tau: 3600.0,
                    absorption: 0.0,
                    advection: 0.0,
                    unit: "e10j".into(),
                    fold: None,
                }],
                lat_sign: Some("4".into()),
                lon_sign: Some("6".into()),
                epoch_scale: 1.0,
                tau_key: String::new(),
                mag_type_key: String::new(),
            }],
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
        };
        let body = r#"{"data":[["2026-08-01 17:43:48","2.9","0.1","19.5","S","176.2","E","45.0",null],["2026-07-21 01:14:45","3.2","0.11","9.4","N","57.4","W","31.5",null]]}"#;
        let fixture_lsk = super::LeapSeconds {
            delta_t_a: 32.184,
            deltas: vec![(37.0, 1483228800.0)],
        };
        match super::extract(&src, body, 8.0e8, &fixture_lsk) {
            super::ExtractResult::Measurements(v) => {
                assert_eq!(v.len(), 2);
                match (&v[0].0.position, &v[1].0.position) {
                    (
                        super::Position::Surface {
                            lat: la0, lon: lo0, ..
                        },
                        super::Position::Surface {
                            lat: la1, lon: lo1, ..
                        },
                    ) => {
                        assert!((la0 - (-19.5)).abs() < 1e-9, "lat S: {}", la0);
                        assert!((lo0 - 176.2).abs() < 1e-9, "lon E: {}", lo0);
                        assert!((la1 - 9.4).abs() < 1e-9, "lat N: {}", la1);
                        assert!((lo1 - (-57.4)).abs() < 1e-9, "lon W: {}", lo1);
                    }
                    _ => panic!("position variant unexpected"),
                }
            }
            _ => panic!("extract variant unexpected"),
        }
    }

    #[test]
    fn test_mag_type_gating() {
        assert!(is_moment_magnitude("mww"));
        assert!(is_moment_magnitude("Mw"));
        assert!(is_moment_magnitude("MWP"));
        assert!(is_moment_magnitude("mwpd"));
        assert!(!is_moment_magnitude("ml"));
        assert!(!is_moment_magnitude("md"));
        assert!(!is_moment_magnitude("mb"));
        assert!(!is_moment_magnitude("m"));
        assert!(!is_moment_magnitude("Mj"));

        let src = super::SourceConfig {
            ttl: 60,
            url: "https://example.org/quake".into(),
            frame: super::Frame::Surface {
                body_name: "earth".into(),
                lat: 0.0,
                lon: 0.0,
                alt: 0.0,
            },
            format: "json".into(),
            extracts: vec![super::Extract::Map {
                arr_path: "data".into(),
                lat_key: "lat".into(),
                lon_key: "lon".into(),
                alt_key: "alt".into(),
                epoch_key: String::new(),
                val_key: String::new(),
                alt_scale: 1.0,
                vel_key: String::new(),
                vel_scale: 1.0,
                trk_key: String::new(),
                vr_key: String::new(),
                fields: vec![FieldConfig {
                    key: "mag".into(),
                    name: "quake_moment".into(),
                    kernel: 0,
                    force: 3,
                    tau: 6.0,
                    absorption: 0.0,
                    advection: 0.0,
                    unit: "Mw".into(),
                    fold: None,
                }],
                lat_sign: None,
                lon_sign: None,
                epoch_scale: 1.0,
                tau_key: String::new(),
                mag_type_key: "magType".into(),
            }],
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
        };
        let body = r#"{"data":[
                {"lat":1.0,"lon":2.0,"alt":0.0,"magType":"mww","mag":5.5},
                {"lat":3.0,"lon":4.0,"alt":0.0,"magType":"ml","mag":3.0},
                {"lat":5.0,"lon":6.0,"alt":0.0,"magType":"Mw","mag":6.0}
            ]}"#;
        let now = 8.4e8;
        let fixture_lsk = super::LeapSeconds {
            delta_t_a: 32.184,
            deltas: vec![(37.0, 1483228800.0)],
        };
        match super::extract(&src, body, now, &fixture_lsk) {
            super::ExtractResult::Measurements(v) => {
                assert_eq!(v.len(), 2);
                assert!((v[0].0.value - 5.5).abs() < 1e-12);
                assert!((v[1].0.value - 6.0).abs() < 1e-12);
            }
            _ => panic!("extract variant unexpected"),
        }
    }

    #[test]
    fn test_force_id_electric() {
        assert_eq!(crate::force::force_id_of("electric"), Some(8));
        assert_eq!(crate::force::force_id_of("biotic"), None);
        assert_eq!(crate::force::kernel_id_for_force(8), Some(1));
    }

    #[test]
    fn test_route_key_strips_query_and_www() {
        assert_eq!(
            route_key("https://earthquake.usgs.gov/fdsnws/event/1/query?format=geojson&limit=2"),
            Some("earthquake.usgs.gov/fdsnws/event/1/query".to_string())
        );
        assert_eq!(
            route_key("https://www.example.com/"),
            Some("example.com".to_string())
        );
    }

    #[test]
    fn test_route_key_normalizes_template() {
        assert_eq!(
            route_key("https://api.example.com/{lat}/{lon}"),
            Some("api.example.com/*/*".to_string())
        );
    }

    #[test]
    fn test_route_prefix_keys_most_specific_first() {
        let keys =
            route_prefix_keys("https://earthquake.usgs.gov/fdsnws/event/1/query?format=geojson");
        assert_eq!(
            keys,
            vec![
                "earthquake.usgs.gov/fdsnws/event/1/query".to_string(),
                "earthquake.usgs.gov/fdsnws/event/1".to_string(),
                "earthquake.usgs.gov/fdsnws/event".to_string(),
                "earthquake.usgs.gov/fdsnws".to_string(),
                "earthquake.usgs.gov".to_string(),
            ]
        );
    }

    #[test]
    fn test_frame_registry_distinguishes_routes_on_one_host() {
        let mut reg: HashMap<String, String> = HashMap::new();
        reg.insert(
            "api.example.com/weather".to_string(),
            "on earth".to_string(),
        );
        reg.insert(
            "api.example.com/asteroids".to_string(),
            "at sun".to_string(),
        );
        let (weather, _) =
            draft_frame_guess("https://api.example.com/weather?city=berlin", "", &reg);
        let (asteroids, _) = draft_frame_guess("https://api.example.com/asteroids/433", "", &reg);
        assert_eq!(weather, "on earth\n");
        assert_eq!(asteroids, "at sun\n");
    }

    #[test]
    fn test_frame_registry_prefix_match() {
        let mut reg: HashMap<String, String> = HashMap::new();
        reg.insert(
            "api.example.com/weather".to_string(),
            "on earth".to_string(),
        );
        let (frame, _) = draft_frame_guess("https://api.example.com/weather/current", "", &reg);
        assert_eq!(frame, "on earth\n");
    }

    #[test]
    fn test_ci_classification_plain_secret_template_fanout() {
        assert!(!url_has_template("https://example.com/a/b.json"));
        assert!(!url_has_template(
            "https://firms.modaps.eosdis.nasa.gov/api/area/csv/{FIRMS_MAP_KEY}/MODIS_NRT/world/1"
        ));
        assert!(url_has_template("https://example.com/{lat}/{lon}"));
        assert!(url_has_template(
            "https://earthquake.usgs.gov/fdsnws/event/1/query?starttime={hour_ago}&latitude={lat}"
        ));
        assert!(url_is_fanout(
            "https://example.com/stations/{station}/readings"
        ));
        assert!(url_is_fanout(
            "https://example.com/?station={nearest_station}"
        ));
        assert!(!url_is_fanout("https://example.com/{lat}/{lon}"));
    }

    #[test]
    fn test_ci_probe_render_resolves_templates_and_secrets() {
        let mut env = HashMap::new();
        env.insert("FIRMS_MAP_KEY".to_string(), "ABC123".to_string());
        let url = ci_probe_render(
            "https://example.com/?lat={lat}&lon={lon}&key={FIRMS_MAP_KEY}",
            (52.5, 13.4),
            &env,
        )
        .unwrap();
        assert!(url.contains("lat=52.500000"), "got {}", url);
        assert!(url.contains("lon=13.400000"), "got {}", url);
        assert!(url.contains("key=ABC123"), "got {}", url);
        assert!(!url.contains('{'), "unresolved marker in {}", url);
    }

    #[test]
    fn test_ci_probe_render_bbox_and_temporal() {
        let env = HashMap::new();
        let url = ci_probe_render(
                "https://example.com/?bBox={lon_min},{lat_min},{lon_max},{lat_max}&start={week_ago}&end={today}",
                (0.0, 0.0),
                &env,
            )
            .unwrap();
        assert!(!url.contains('{'), "unresolved marker in {}", url);
        assert!(url.contains("start=20"), "missing week_ago in {}", url);
        assert!(url.contains("end=20"), "missing today in {}", url);
    }

    #[test]
    fn test_secret_resolves_void_distinguishes_absent_and_empty() {
        let mut env = HashMap::new();
        env.insert("SET_KEY".to_string(), "v".to_string());
        env.insert("EMPTY_KEY".to_string(), String::new());
        assert!(secret_resolves_void("{ABSENT_KEY}", &env));
        assert!(secret_resolves_void("{EMPTY_KEY}", &env));
        assert!(!secret_resolves_void("{SET_KEY}", &env));
    }

    #[test]
    fn test_alerce_object_and_detection_parse() {
        let list = r#"{"total":null,"items":[{"oid":"ZTF17aaaaaal","meanra":210.5,"meandec":-12.25,"firstmjd":58000.0},{"oid":"ZTF18bbbbbbb","meanra":null,"meandec":null}]}"#;
        let objs = alerce_objects(&parse_json(list).unwrap());
        assert_eq!(objs.len(), 1);
        assert_eq!(objs[0].0, "ZTF17aaaaaal");
        assert!((objs[0].1 - 210.5).abs() < 1e-12);
        assert!((objs[0].2 + 12.25).abs() < 1e-12);
        let det = r#"[{"ra":210.5,"dec":-12.25,"mjd":60123.4,"magpsf":18.1,"magap":18.4},{"ra":"absent","dec":0.0,"mjd":60123.4,"magpsf":19.0,"magap":19.0}]"#;
        let rows = alerce_detection_rows(&parse_json(det).unwrap());
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0], (210.5, -12.25, 60123.4, 18.1, 18.4));
    }

    #[test]
    fn test_finals_channels_last_occupied_line() {
        let line = format!(
            "{:>2}{:>2}{:>2} {:8.2} {:1} {:9.6}{:10.6} {:9.6}{:10.6} {:1} {:10.7}",
            25, 8, 21, 61638.00, "P", 0.269050, 0.000012, 0.372959, 0.000014, "P", -0.0683654
        );
        let text = format!("{}\n{:>2}{:>2}{:>2} {:8.2}\n", line, 25, 8, 22, 61639.00);
        let src = source_fixture(
            "finals",
            vec![
                Extract::Field(field_fixture("ut1_utc", 86400.0)),
                Extract::Field(field_fixture("pmx", 86400.0)),
                Extract::Field(field_fixture("pmy", 86400.0)),
            ],
        );
        let lsk = fixture_lsk();
        let channels = build_finals_channels(&src, &text, &lsk);
        assert_eq!(channels.len(), 3);
        let expect_epoch = lsk.unix_to_tdb((61638.0 - 40587.0) * 86400.0).unwrap();
        for (c, fc) in &channels {
            assert_eq!(c.epoch, expect_epoch);
            match fc.name.as_str() {
                "ut1_utc" => assert_eq!(c.value, -0.0683654),
                "pmx" => assert_eq!(c.value, 0.269050),
                "pmy" => assert_eq!(c.value, 0.372959),
                other => panic!("unexpected field {}", other),
            }
        }
    }

    #[test]
    fn test_ionex_channels_two_lat_five_lon() {
        let mut text = String::new();
        text.push_str("     1            CODEX                       IONEX VERSION / TYPE\n");
        text.push_str(
            "    -1                                                              EXPONENT\n",
        );
        text.push_str("     4 START OF TEC MAP\n");
        text.push_str(
            "  2026     8    18    12     0     0                        EPOCH OF CURRENT MAP\n",
        );
        text.push_str(
            "    87.5-180.0 180.0  90.0 450.0                            LAT/LON1/LON2/DLON/H\n",
        );
        let mut line = String::from("    87.5-180.0 180.0  90.0 450.0");
        for w in 101..=105 {
            line.push_str(&format!("{:>5}", w));
        }
        text.push_str(&line);
        text.push('\n');
        let mut line2 = String::from("    72.5-180.0 180.0  90.0 450.0");
        for w in 201..=205 {
            line2.push_str(&format!("{:>5}", w));
        }
        text.push_str(&line2);
        text.push('\n');
        text.push_str("     8 END OF TEC MAP\n");
        let src = source_fixture("ionex", vec![Extract::Field(field_fixture("tec", 7200.0))]);
        let lsk = fixture_lsk();
        let now = 1786968000.0 + 69.184 + 3600.0;
        let channels = build_ionex_channels(&src, &text, now, &lsk);
        assert_eq!(channels.len(), 10);
        let mut seen_lat = [false; 2];
        for (c, _) in &channels {
            if let Position::Surface { lat, lon, alt, .. } = &c.position {
                assert!(*alt > 400_000.0, "ionex H must set the shell altitude");
                assert!((lon + 180.0) % 90.0 == 0.0, "lon grid mismatch");
                if *lat == 87.5 {
                    seen_lat[0] = true;
                    assert!(c.value >= 10.1 && c.value <= 10.5);
                } else if *lat == 72.5 {
                    seen_lat[1] = true;
                    assert!(c.value >= 20.1 && c.value <= 20.5);
                } else {
                    panic!("unexpected lat {}", lat);
                }
            }
        }
        assert!(seen_lat[0] && seen_lat[1], "both lat rows present");
    }
}
use crate::dastcom::{hill_radius_m, parse_record, state_at, RECORD_STRIDE};
use crate::force::default_kernel_for;
use crate::inflate::gunzip;
use crate::netcdf::NetcdfFile;
use crate::pck::PckBody;
use std::io::IsTerminal;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Mutex};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};
const NAIF_LSK_TTL_SECS: u64 = 86400;
const NAIF_LSK_EMBEDDED: &str = include_str!("kernels/naif0012.tls");

pub fn embedded_lsk() -> Option<LeapSeconds> {
    crate::lsk::parse(NAIF_LSK_EMBEDDED)
}

pub fn resolve_asset(rel: &str) -> std::path::PathBuf {
    let cwd_candidate = std::path::PathBuf::from(rel);
    if cwd_candidate.exists() {
        return cwd_candidate;
    }
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            let exe_candidate = dir.join(rel);
            if exe_candidate.exists() {
                return exe_candidate;
            }
        }
    }
    if !rel.starts_with('.') {
        eprintln!("asset {} absent (CWD {:?})", rel, std::env::current_dir());
    }
    cwd_candidate
}

const SURFACE_MOTION_DT: f64 = 0.01;
const MAX_SAMPLES: usize = 1 << 22;

type Origin = u32;

fn cell_of(p: [f64; 3], s: f64) -> CellKey {
    (
        (p[0] / s).floor() as i64,
        (p[1] / s).floor() as i64,
        (p[2] / s).floor() as i64,
    )
}

pub fn law_bounds(
    motion: &Motion,
    epoch: f64,
    resid_ema: f64,
    eph: &HashMap<String, BodyEphemeris>,
) -> Option<(f64, f64, [f64; 3])> {
    let p0 = motion.at(epoch, epoch, eph)?;
    let p1 = motion.at(epoch + 1.0, epoch, eph)?;
    let p2 = motion.at(epoch + 2.0, epoch, eph)?;
    let v = ((p1[0] - p0[0]).powi(2) + (p1[1] - p0[1]).powi(2) + (p1[2] - p0[2]).powi(2)).sqrt();
    let a = ((p2[0] - 2.0 * p1[0] + p0[0]).powi(2)
        + (p2[1] - 2.0 * p1[1] + p0[1]).powi(2)
        + (p2[2] - 2.0 * p1[2] + p0[2]).powi(2))
    .sqrt();
    Some((Φ * (v + resid_ema), Φ * a, p0))
}

fn build_spatial_hash(samples: Vec<Sample>, cadence: f64) -> SpatialHash {
    let mut bounded = Vec::new();
    let mut unbounded = Vec::new();
    for s in samples {
        if s.extent.is_finite() {
            bounded.push(s);
        } else {
            unbounded.push(s);
        }
    }
    let mut anchor_vmax = 0.0f64;
    let mut anchor_amax = 0.0f64;
    let mut epoch_min = f64::MAX;
    for s in &bounded {
        anchor_vmax = anchor_vmax.max(s.anchor_vmax);
        anchor_amax = anchor_amax.max(s.anchor_amax);
        epoch_min = epoch_min.min(s.epoch);
    }
    let rho_cad = anchor_vmax * cadence + 0.5 * anchor_amax * cadence * cadence;
    let shift = (2.0 * rho_cad).log2().ceil().clamp(0.0, 63.0) as i32;
    let motion_cell = 2f64.powi(shift);
    let mut span = 1.0f64;
    for k in 0..3 {
        let mut lo = f64::INFINITY;
        let mut hi = f64::NEG_INFINITY;
        for s in &bounded {
            lo = lo.min(s.anchor_p0[k]);
            hi = hi.max(s.anchor_p0[k]);
        }
        span = span.max(hi - lo);
    }
    let cell_size = motion_cell.max(span / 1024.0);
    let mut cells: HashMap<CellKey, Vec<Sample>> = HashMap::new();
    let mut cell_lo = (i64::MAX, i64::MAX, i64::MAX);
    let mut cell_hi = (i64::MIN, i64::MIN, i64::MIN);
    for s in bounded {
        let c = cell_of(s.anchor_p0, cell_size);
        cell_lo.0 = cell_lo.0.min(c.0);
        cell_lo.1 = cell_lo.1.min(c.1);
        cell_lo.2 = cell_lo.2.min(c.2);
        cell_hi.0 = cell_hi.0.max(c.0);
        cell_hi.1 = cell_hi.1.max(c.1);
        cell_hi.2 = cell_hi.2.max(c.2);
        cells.entry(c).or_default().push(s);
    }
    SpatialHash {
        cell_size,
        anchor_vmax,
        anchor_amax,
        epoch_min: if epoch_min == f64::MAX {
            0.0
        } else {
            epoch_min
        },
        cell_lo,
        cell_hi,
        cells,
        unbounded,
    }
}

pub fn build_buffer(
    samples: Vec<Sample>,
    cadence: f64,
    eph: Arc<HashMap<String, BodyEphemeris>>,
    curves: Option<Arc<CurveSet>>,
    spectral: Vec<SpectralHash>,
) -> Buffer {
    Buffer {
        cache: build_spatial_hash(samples, cadence),
        eph,
        curves,
        spectral,
    }
}

fn build_asteroid_samples(bytes: &[u8], ttl: u64) -> Vec<Sample> {
    let eph: HashMap<String, BodyEphemeris> = HashMap::new();
    let mut samples: Vec<Sample> = Vec::new();
    for chunk in bytes.chunks_exact(RECORD_STRIDE) {
        let rec = match parse_record(chunk) {
            Some(r) => r,
            None => continue,
        };
        if rec.number == 0 || rec.a_au <= 0.0 || rec.e >= 1.0 {
            continue;
        }
        if hill_radius_m(&rec).is_none() {
            continue;
        }
        let epoch_secs = (rec.epoch_jd - J2000_EPOCH) * 86400.0;
        let motion = Motion::Kepler {
            rec: Arc::new(rec.clone()),
        };
        let Some((anchor_vmax, anchor_amax, anchor_p0)) =
            law_bounds(&motion, epoch_secs, 0.0, &eph)
        else {
            continue;
        };
        let gm = rec.gm_km3_s2 as f64 * 1.0e9;
        samples.push(Sample {
            source: SampleSource::Ephemeris,
            epoch: epoch_secs,
            ttl: ttl as f64,
            extent: 0.0,
            tau: f64::INFINITY,
            kernel_id: 0.0,
            force_type: 1.0,
            absorption: 0.0,
            advection: 0.0,
            anchor_vmax,
            anchor_amax,
            anchor_p0,
            motion: motion.clone(),
            val: gm,
            name: "dastcom.mass".to_string(),
            z: 0.0,
            freq: 0.0,
            bin_width: 0.0,
            color_index: 0.0,
        });
        if rec.radius_km > 0.0 {
            samples.push(Sample {
                source: SampleSource::Ephemeris,
                epoch: epoch_secs,
                ttl: ttl as f64,
                extent: 0.0,
                tau: f64::INFINITY,
                kernel_id: 1.0,
                force_type: 1.0,
                absorption: 0.0,
                advection: 0.0,
                anchor_vmax,
                anchor_amax,
                anchor_p0,
                motion,
                val: rec.radius_km as f64 * 1000.0,
                name: "dastcom.radius".to_string(),
                z: 0.0,
                freq: 0.0,
                bin_width: 0.0,
                color_index: 0.0,
            });
        }
    }
    samples
}

const STAR_RECORD_BYTES: usize = 44;

fn star_stride(bytes: &[u8]) -> Option<usize> {
    if bytes.len() > 0 && bytes.len() % STAR_RECORD_BYTES == 0 {
        Some(STAR_RECORD_BYTES)
    } else {
        None
    }
}

fn parse_star_record(b: &[u8]) -> Option<StarRec> {
    if b.len() != STAR_RECORD_BYTES {
        return None;
    }
    let ra = f64::from_le_bytes(b[0..8].try_into().ok()?);
    let dec = f64::from_le_bytes(b[8..16].try_into().ok()?);
    let pm_ra = f32::from_le_bytes(b[16..20].try_into().ok()?) as f64;
    let pm_de = f32::from_le_bytes(b[20..24].try_into().ok()?) as f64;
    let plx = f32::from_le_bytes(b[24..28].try_into().ok()?) as f64;
    let mag = f32::from_le_bytes(b[28..32].try_into().ok()?) as f64;
    let flux = f32::from_le_bytes(b[32..36].try_into().ok()?) as f64;
    let color = f32::from_le_bytes(b[36..40].try_into().ok()?) as f64;
    let rv = f32::from_le_bytes(b[40..44].try_into().ok()?) as f64;
    if !ra.is_finite() || !dec.is_finite() || !(plx > 0.0) || !mag.is_finite() || !rv.is_finite() {
        return None;
    }
    Some(StarRec {
        ra_deg: ra,
        dec_deg: dec,
        pm_ra_masyr: pm_ra,
        pm_de_masyr: pm_de,
        plx_mas: plx,
        flux,
        mag,
        tau: 0.0,
        color_index: if color.is_finite() { color } else { 0.0 },
        rv_m_s: rv,
    })
}

pub fn star_position_at(rec: &StarRec, t2: f64) -> ([f64; 3], [f64; 3]) {
    let dt_yr = t2 / (86400.0 * 365.25);
    let dec_rad = rec.dec_deg.to_radians();
    let ra = rec.ra_deg + rec.pm_ra_masyr / (3.6e6 * dec_rad.cos().max(1e-6)) * dt_yr;
    let dec = rec.dec_deg + rec.pm_de_masyr / 3.6e6 * dt_yr;
    let (sa, ca) = ra.to_radians().sin_cos();
    let (sd, cd) = dec.to_radians().sin_cos();
    let p_hat = [cd * ca, cd * sa, sd];
    let d = (1000.0 / rec.plx_mas) * PARSEC_M;
    let p = [p_hat[0] * d, p_hat[1] * d, p_hat[2] * d];
    let mu_a = rec.pm_ra_masyr * MAS_YR_TO_RAD_S;
    let mu_d = rec.pm_de_masyr * MAS_YR_TO_RAD_S;
    let a_hat = [-sa, ca, 0.0];
    let d_hat = [-sd * ca, -sd * sa, cd];
    let vr = rec.rv_m_s;
    let vel = [
        d * (mu_a * a_hat[0] + mu_d * d_hat[0]) + vr * p_hat[0],
        d * (mu_a * a_hat[1] + mu_d * d_hat[1]) + vr * p_hat[1],
        d * (mu_a * a_hat[2] + mu_d * d_hat[2]) + vr * p_hat[2],
    ];
    (p, vel)
}

fn build_star_samples(bytes: &[u8]) -> Vec<Sample> {
    let eph: HashMap<String, BodyEphemeris> = HashMap::new();
    let mut samples: Vec<Sample> = Vec::new();
    let Some(stride) = star_stride(bytes) else {
        eprintln!(
            "star bin {} bytes: no {}-byte records — pending recompilation, stars stay dark",
            bytes.len(),
            STAR_RECORD_BYTES
        );
        return samples;
    };
    for chunk in bytes.chunks_exact(stride) {
        let Some(mut rec) = parse_star_record(chunk) else {
            continue;
        };
        let m_abs = rec.mag + 5.0 * (rec.plx_mas / 100.0).log10();
        let lum = 10f64.powf(-0.4 * (m_abs - 4.83));
        rec.tau = 1e10 * 365.25 * 86400.0 * lum.powf(-5.0 / 7.0);
        let motion = Motion::Spherical {
            rec: Arc::new(rec.clone()),
        };
        let Some((anchor_vmax, anchor_amax, anchor_p0)) = law_bounds(&motion, 0.0, 0.0, &eph)
        else {
            continue;
        };
        samples.push(Sample {
            source: SampleSource::Ephemeris,
            epoch: 0.0,
            ttl: rec.tau,
            extent: f64::INFINITY,
            tau: rec.tau,
            kernel_id: 0.0,
            force_type: 0.0,
            absorption: 0.0,
            advection: 0.0,
            anchor_vmax,
            anchor_amax,
            anchor_p0,
            motion,
            val: rec.flux,
            name: "dr3_stars.flux".to_string(),
            z: 0.0,
            freq: 0.0,
            bin_width: 0.0,
            color_index: rec.color_index,
        });
    }
    samples
}

pub fn query_hash(
    hash: &SpatialHash,
    center: [f64; 3],
    t2: f64,
    pad: f64,
    delta_t_cache: f64,
    floor: &[f64; 9],
    softening: f64,
    forward: [f64; 3],
    records: &mut Vec<SampleRecord>,
    eph: &HashMap<String, BodyEphemeris>,
) {
    for sample in &hash.unbounded {
        let age = (t2 - sample.epoch).abs();
        if age > sample.ttl * 64.0 {
            continue;
        }
        if signal_reach(sample.force_type, sample.advection, age).is_none() {
            continue;
        }
        let v_prop = match propagation_speed(sample.force_type, sample.advection) {
            Some(v) => v,
            None => continue,
        };
        let ft = sample.force_type as u8;
        let floor_ft = if ft < 9 { floor[ft as usize] } else { f64::NAN };
        if !(floor_ft.is_finite() && floor_ft > 0.0) {
            continue;
        }
        let tolman = if sample.force_type == 0.0 && sample.z > 0.0 {
            let z1 = 1.0 + sample.z;
            1.0 / (z1 * z1 * z1 * z1)
        } else {
            1.0
        };
        let val_max = sample.val.abs() * tolman;
        let scale2 = softening * softening;
        if !val_max.is_finite() || val_max < floor_ft * scale2 {
            continue;
        }
        let p = match sample.motion.at(t2, sample.epoch, eph) {
            Some(p) => p,
            None => continue,
        };
        let ddx = p[0] - center[0];
        let ddy = p[1] - center[1];
        let ddz = p[2] - center[2];
        let d2 = ddx * ddx + ddy * ddy + ddz * ddz;
        let d = d2.sqrt();
        let sd = ddx * forward[0] + ddy * forward[1] + ddz * forward[2];
        let transverse2 = (d2 - sd * sd).max(0.0);
        if !(sample.ttl > 0.0) {
            continue;
        }
        let retarded = if v_prop > 0.0 && d > 0.0 {
            (age - d / v_prop).max(0.0)
        } else {
            age
        };
        let val_eff = sample.val * (-retarded / sample.ttl).exp() * tolman;
        if val_eff.abs() / (transverse2 + scale2) < floor_ft {
            continue;
        }
        let v = if let Motion::Linear { v, .. } = &sample.motion {
            [v[0], v[1], v[2]]
        } else {
            let p_dt = match sample.motion.at(t2 + 1e-3, sample.epoch, eph) {
                Some(pd) => pd,
                None => continue,
            };
            [
                (p_dt[0] - p[0]) / 1e-3,
                (p_dt[1] - p[1]) / 1e-3,
                (p_dt[2] - p[2]) / 1e-3,
            ]
        };
        records.push((
            p[0],
            p[1],
            p[2],
            sample.val,
            sample.epoch,
            sample.ttl,
            sample.tau,
            wire_extent(sample.extent),
            sample.kernel_id,
            sample.force_type,
            sample.absorption,
            sample.advection,
            v[0],
            v[1],
            v[2],
            if sample.force_type == 0.0 {
                sample.z
            } else {
                0.0
            },
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            sample.color_index,
            sample.freq,
            sample.bin_width,
        ));
    }
    if hash.cells.is_empty() {
        return;
    }
    let qf = center;
    let dt = (t2 - hash.epoch_min).abs() + delta_t_cache;
    let rho = hash.anchor_vmax * dt + 0.5 * hash.anchor_amax * dt * dt + pad;
    let s = hash.cell_size;
    let qlo = cell_of([qf[0] - rho, qf[1] - rho, qf[2] - rho], s);
    let qhi = cell_of([qf[0] + rho, qf[1] + rho, qf[2] + rho], s);
    let lo = (
        qlo.0.max(hash.cell_lo.0),
        qlo.1.max(hash.cell_lo.1),
        qlo.2.max(hash.cell_lo.2),
    );
    let hi = (
        qhi.0.min(hash.cell_hi.0),
        qhi.1.min(hash.cell_hi.1),
        qhi.2.min(hash.cell_hi.2),
    );
    if lo.0 > hi.0 || lo.1 > hi.1 || lo.2 > hi.2 {
        return;
    }
    let span = (hi.0.saturating_sub(lo.0).saturating_add(1) as u64)
        .saturating_mul(hi.1.saturating_sub(lo.1).saturating_add(1) as u64)
        .saturating_mul(hi.2.saturating_sub(lo.2).saturating_add(1) as u64);
    let in_box = |ck: &CellKey| {
        ck.0 >= lo.0 && ck.0 <= hi.0 && ck.1 >= lo.1 && ck.1 <= hi.1 && ck.2 >= lo.2 && ck.2 <= hi.2
    };
    let mut emit = |samples: &Vec<Sample>| {
        for sample in samples {
            let age = (t2 - sample.epoch).abs();
            if age > sample.ttl * 64.0 {
                continue;
            }
            let reach_signal = match signal_reach(sample.force_type, sample.advection, age) {
                Some(r) => r,
                None => continue,
            };
            let future_age = age + delta_t_cache;
            let reach = reach_signal
                + sample.extent
                + sample.anchor_vmax * future_age
                + 0.5 * sample.anchor_amax * future_age * future_age
                + pad;
            let dx = sample.anchor_p0[0] - qf[0];
            let dy = sample.anchor_p0[1] - qf[1];
            let dz = sample.anchor_p0[2] - qf[2];
            let dist2_anchor_p0 = dx * dx + dy * dy + dz * dz;
            if dist2_anchor_p0 > reach * reach {
                continue;
            }
            let p = match sample.motion.at(t2, sample.epoch, eph) {
                Some(p) => p,
                None => continue,
            };
            let ddx = p[0] - center[0];
            let ddy = p[1] - center[1];
            let ddz = p[2] - center[2];
            let exact = sample.extent + pad;
            let dist2 = ddx * ddx + ddy * ddy + ddz * ddz;
            if dist2 > exact * exact {
                continue;
            }
            let v = if let Motion::Linear { v, .. } = &sample.motion {
                [v[0], v[1], v[2]]
            } else {
                let p_dt = match sample.motion.at(t2 + 1e-3, sample.epoch, eph) {
                    Some(pd) => pd,
                    None => continue,
                };
                [
                    (p_dt[0] - p[0]) / 1e-3,
                    (p_dt[1] - p[1]) / 1e-3,
                    (p_dt[2] - p[2]) / 1e-3,
                ]
            };
            records.push((
                p[0],
                p[1],
                p[2],
                sample.val,
                sample.epoch,
                sample.ttl,
                sample.tau,
                wire_extent(sample.extent),
                sample.kernel_id,
                sample.force_type,
                sample.absorption,
                sample.advection,
                v[0],
                v[1],
                v[2],
                if sample.force_type == 0.0 {
                    sample.z
                } else {
                    0.0
                },
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                sample.color_index,
                sample.freq,
                sample.bin_width,
            ));
        }
    };
    if span > hash.cells.len() as u64 * 4 {
        for (ck, v) in &hash.cells {
            if in_box(ck) {
                emit(v);
            }
        }
    } else {
        for cx in lo.0..=hi.0 {
            for cy in lo.1..=hi.1 {
                for cz in lo.2..=hi.2 {
                    if let Some(v) = hash.cells.get(&(cx, cy, cz)) {
                        emit(v);
                    }
                }
            }
        }
    }
}

pub fn sense_membrane(
    buf: &Buffer,
    center: [f64; 3],
    t2: f64,
    pad: f64,
    delta_t_cache: f64,
    floor: &[f64; 9],
    softening: f64,
    forward: [f64; 3],
    records: &mut Vec<SampleRecord>,
    eph: &HashMap<String, BodyEphemeris>,
) {
    query_hash(
        &buf.cache,
        center,
        t2,
        pad,
        delta_t_cache,
        floor,
        softening,
        forward,
        records,
        eph,
    );
    for sh in &buf.spectral {
        let Some(p) = sh.motion.at(t2, sh.epoch, eph) else {
            continue;
        };
        let Some(p2) = sh.motion.at(t2 + 1e-3, sh.epoch, eph) else {
            continue;
        };
        let ddx = p[0] - center[0];
        let ddy = p[1] - center[1];
        let ddz = p[2] - center[2];
        if ddx * ddx + ddy * ddy + ddz * ddz > pad * pad {
            continue;
        }
        let vx = (p2[0] - p[0]) / 1e-3;
        let vy = (p2[1] - p[1]) / 1e-3;
        let vz = (p2[2] - p[2]) / 1e-3;
        for &(freq, bin_width, val) in &sh.bins {
            records.push((
                p[0],
                p[1],
                p[2],
                val,
                sh.epoch,
                sh.ttl,
                sh.tau,
                0.0,
                sh.kernel_id,
                sh.force_type,
                sh.absorption,
                sh.advection,
                vx,
                vy,
                vz,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                freq,
                bin_width,
            ));
        }
    }
}

pub fn surface_motion(
    body_name: &str,
    lat: f64,
    lon: f64,
    alt: f64,
    speed: f64,
    track: f64,
    vrate: f64,
    t: f64,
    eph: &HashMap<String, BodyEphemeris>,
) -> Option<Motion> {
    let p0 = body_fixed_to_icrs(body_name, lat, lon, alt, t, eph)?;
    let p1 = body_fixed_to_icrs(body_name, lat, lon, alt, t + 1.0, eph)?;
    let v_frame = [p1[0] - p0[0], p1[1] - p0[1], p1[2] - p0[2]];
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
    let r = eph
        .get(body_name)
        .and_then(|e| e.props.as_ref())
        .map(|p| p.radius_m)?;
    let cl = latr.cos();
    let dt = SURFACE_MOTION_DT;
    let pp = body_fixed_to_icrs(
        body_name,
        lat + v_ecef[1] * dt / r,
        lon + v_ecef[0] * dt / (r * cl),
        alt + v_ecef[2] * dt,
        t,
        eph,
    )?;
    let v_rot = [
        (pp[0] - p0[0]) / dt,
        (pp[1] - p0[1]) / dt,
        (pp[2] - p0[2]) / dt,
    ];
    Some(Motion::Linear {
        p: p0,
        v: [
            v_frame[0] + v_rot[0],
            v_frame[1] + v_rot[1],
            v_frame[2] + v_rot[2],
        ],
    })
}

#[cfg(feature = "browser_relay")]
pub fn body_id_to_name(bodies: &[String], id: u32) -> Option<String> {
    if id == 0 {
        return None;
    }
    bodies.get((id - 1) as usize).cloned()
}

pub fn frame_motion(
    frame: &Frame,
    spd: Option<f64>,
    hdg: Option<f64>,
    t: f64,
    eph: &HashMap<String, BodyEphemeris>,
) -> Option<Motion> {
    match frame {
        Frame::Surface {
            body_name,
            lat,
            lon,
            alt,
            ..
        } => match (spd, hdg) {
            (Some(s), Some(h)) if s > 0.0 => {
                surface_motion(body_name, *lat, *lon, *alt, s, h, 0.0, t, eph)
            }
            _ => {
                if eph.get(body_name).and_then(|e| e.props.as_ref()).is_some() {
                    Some(Motion::Surface {
                        body_name: body_name.clone(),
                        lat: *lat,
                        lon: *lon,
                        alt: *alt,
                    })
                } else {
                    None
                }
            }
        },
        Frame::Barycenter {
            body_name, scale, ..
        } => {
            if eph.get(body_name).is_some() {
                Some(Motion::Barycenter {
                    body_name: body_name.clone(),
                    scale: *scale,
                })
            } else {
                None
            }
        }
        Frame::Manifest => None,
    }
}

pub fn leap_seconds(time: &Arc<Mutex<Option<LeapSeconds>>>) -> Option<LeapSeconds> {
    match time.lock() {
        Ok(guard) => guard.clone(),
        Err(_) => None,
    }
}

pub fn system_now(time: &Arc<Mutex<Option<LeapSeconds>>>) -> Option<f64> {
    match leap_seconds(time) {
        Some(lsk) => lsk.system_now_tdb(),
        None => None,
    }
}

#[derive(Clone)]
pub struct OriginState {
    fetched: f64,
    prev_epoch: f64,
    prev_abs: [f64; 3],
    prev_motion: Option<Motion>,
    resid_ema: f64,
    has_prev: bool,
    failures: u32,
    in_flight: bool,
}

fn origin_stale(
    origins: &HashMap<Origin, OriginState>,
    origin: Origin,
    ttl: u64,
    now: f64,
) -> bool {
    match origins.get(&origin) {
        Some(o) => {
            let backoff = (ttl as f64 / Φ) * (2f64).powi(o.failures.min(FETCH_VOID_CAP) as i32);
            !o.in_flight && now - o.fetched >= backoff
        }
        None => true,
    }
}

fn begin_fetch(origins: &mut HashMap<Origin, OriginState>, origin: Origin, now: f64) {
    let st = origins.entry(origin).or_insert(OriginState {
        fetched: now,
        prev_epoch: now,
        prev_abs: [0.0, 0.0, 0.0],
        prev_motion: None,
        resid_ema: 0.0,
        has_prev: false,
        failures: 0,
        in_flight: false,
    });
    st.in_flight = true;
}

fn settle_fetch(st: &mut OriginState, ok: bool, now: f64) {
    st.fetched = now;
    st.in_flight = false;
    if ok {
        st.failures = 0;
    } else {
        st.failures = (st.failures + 1).min(FETCH_VOID_CAP);
    }
}

fn presence_gate(
    presences: &[(f64, f64, f64, f64, f64, f64, f64, f64, f64)],
    pos: (f64, f64, f64),
    extent: f64,
) -> bool {
    presences.iter().any(|&(_, x, y, z, range, ..)| {
        let reach = extent * Φ + range;
        let dx = x - pos.0;
        let dy = y - pos.1;
        let dz = z - pos.2;
        dx * dx + dy * dy + dz * dz <= reach * reach
    })
}

fn json_has_content(v: &JsonVal) -> bool {
    match v {
        JsonVal::Arr(arr) => !arr.is_empty() || arr.iter().any(json_has_content),
        JsonVal::Obj(map) => map.values().any(json_has_content),
        JsonVal::Null | JsonVal::Bool(_) | JsonVal::Str(_) | JsonVal::Num(_) => false,
    }
}

fn diagnose_no_samples(src: &SourceConfig, body: &str) -> String {
    let parsed = parse_json(body);
    match parsed {
        None => {
            let trimmed = body.trim();
            if trimmed.is_empty() {
                "empty-response (empty body)".to_string()
            } else {
                "data-present (non-JSON body: HTML/XML/text)".to_string()
            }
        }
        Some(j) => {
            let mut arr_has_rows = false;
            let mut key_found = false;
            for ext in &src.extracts {
                match ext {
                    Extract::Map {
                        arr_path,
                        lat_key,
                        lon_key,
                        fields,
                        ..
                    } => {
                        if let Some(JsonVal::Arr(arr)) = jpath_val(&j, arr_path) {
                            if !arr.is_empty() {
                                arr_has_rows = true;
                            }
                        }
                        for fk in [lat_key.as_str(), lon_key.as_str()] {
                            if jpath_val(&j, fk).is_some() {
                                key_found = true;
                            }
                        }
                        for fc in fields {
                            if jpath_val(&j, &fc.key).is_some() {
                                key_found = true;
                            }
                        }
                    }
                    Extract::CelestialMap {
                        arr_path, fields, ..
                    }
                    | Extract::Flatten {
                        arr_path, fields, ..
                    }
                    | Extract::CmrPolygon {
                        arr_path, fields, ..
                    }
                    | Extract::CelestialPolygon {
                        arr_path, fields, ..
                    }
                    | Extract::KeplerMap {
                        arr_path, fields, ..
                    }
                    | Extract::ProfileMap {
                        arr_path, fields, ..
                    } => {
                        if let Some(JsonVal::Arr(arr)) = jpath_val(&j, arr_path) {
                            if !arr.is_empty() {
                                arr_has_rows = true;
                            }
                        }
                        for fc in fields {
                            if jpath_val(&j, &fc.key).is_some() {
                                key_found = true;
                            }
                        }
                    }
                    Extract::Rows { .. } | Extract::GeojsonEvents { .. } | Extract::Hapi(_) => {
                        if json_has_content(&j) {
                            arr_has_rows = true;
                        }
                    }
                    Extract::Field(FieldConfig { key, .. })
                    | Extract::First(FieldConfig { key, .. }, _)
                    | Extract::Last(FieldConfig { key, .. }, _)
                    | Extract::Count(FieldConfig { key, .. })
                    | Extract::Path(FieldConfig { key, .. })
                    | Extract::Deep(FieldConfig { key, .. })
                    | Extract::LastRow(FieldConfig { key, .. })
                    | Extract::ObjLast(FieldConfig { key, .. })
                    | Extract::Regex(FieldConfig { key, .. }) => {
                        if jpath_val(&j, key).is_some()
                            || jpath_val(
                                &j,
                                if let Some((p, _)) = key.rsplit_once('.') {
                                    p
                                } else {
                                    key
                                },
                            )
                            .is_some()
                        {
                            key_found = true;
                        }
                    }
                    Extract::LastObj(_, _, _, _)
                    | Extract::LastLine(_)
                    | Extract::XmlCount(_, _) => {
                        if json_has_content(&j) {
                            key_found = true;
                        }
                    }
                    Extract::Alerce(_) => {
                        if json_has_content(&j) {
                            key_found = true;
                        }
                    }
                }
            }
            if arr_has_rows {
                "data-present (container array has rows but extract yielded nothing)".to_string()
            } else if key_found {
                "data-present (keys exist but no rows extracted)".to_string()
            } else if json_has_content(&j) {
                "data-present (JSON has content but declared keys absent)".to_string()
            } else {
                "empty-response (JSON parsed but all containers empty)".to_string()
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum VoidClass {
    Key,
    Drift,
    Quiet,
    Kaputt,
}

impl VoidClass {
    fn as_str(&self) -> &'static str {
        match self {
            VoidClass::Key => "key-void",
            VoidClass::Drift => "drift-void",
            VoidClass::Quiet => "ruhig-void",
            VoidClass::Kaputt => "kaputt",
        }
    }
}

struct VoidFinding {
    url: String,
    class: VoidClass,
    detail: String,
}

fn civil_date(unix: u64) -> (i64, u32, u32) {
    let days = (unix / 86400) as i64;
    let z = days + 719468;
    let era = (if z >= 0 { z } else { z - 146096 }) / 146097;
    let doe = (z - era * 146097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = (if mp < 10 { mp + 3 } else { mp - 9 }) as u32;
    (if m <= 2 { y + 1 } else { y }, m, d)
}

fn date_str(unix: u64) -> String {
    let (y, m, d) = civil_date(unix);
    format!("{:04}-{:02}-{:02}", y, m, d)
}

fn hour_str(unix: u64) -> String {
    format!(
        "{}T{:02}:{:02}:{:02}Z",
        date_str(unix),
        (unix / 3600) % 24,
        (unix / 60) % 60,
        unix % 60
    )
}

fn live_markers() -> Vec<(String, String)> {
    let unix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let (y, m, d) = civil_date(unix);
    let jd = unix as f64 / 86400.0 + 2440587.5;
    vec![
        ("{today}".into(), date_str(unix)),
        ("{yesterday}".into(), date_str(unix - 86400)),
        ("{tomorrow}".into(), date_str(unix + 86400)),
        ("{now}".into(), hour_str(unix)),
        ("{year}".into(), format!("{:04}", y)),
        ("{month}".into(), format!("{:02}", m)),
        ("{day}".into(), format!("{:02}", d)),
        ("{lat}".into(), "29.5".into()),
        ("{lon}".into(), "-95.0".into()),
        ("{ra}".into(), "0.0".into()),
        ("{dec}".into(), "0.0".into()),
        ("{target}".into(), "Ceres".into()),
        ("{week_ago}".into(), date_str(unix - 7 * 86400)),
        ("{hour_ago}".into(), hour_str(unix - 3600)),
        ("{body}".into(), "ISS".into()),
        ("{lon_min}".into(), "-95.0".into()),
        ("{lon_max}".into(), "-94.0".into()),
        ("{lat_min}".into(), "29.0".into()),
        ("{lat_max}".into(), "30.0".into()),
        ("{grid}".into(), "29.5,-95.0|29.6,-95.0".into()),
        ("{nearest_station}".into(), "8518750".into()),
        ("{jd_now}".into(), format!("{:.2}", jd)),
        ("{jd_start}".into(), format!("{:.2}", jd - 1.0)),
        ("{jd_end}".into(), format!("{:.2}", jd)),
    ]
}

fn unresolved_key(template: &str, env: &HashMap<String, String>) -> Option<String> {
    let mut rest = template;
    while let Some(start) = rest.find('{') {
        rest = &rest[start + 1..];
        let Some(end) = rest.find('}') else {
            return None;
        };
        let key = &rest[..end];
        let upper = key.to_uppercase();
        match env.get(key).or_else(|| env.get(&upper)) {
            Some(v) if !v.is_empty() => {}
            _ => return Some(key.to_string()),
        }
        rest = &rest[end + 1..];
    }
    None
}

fn live_sweep(
    env: &HashMap<String, String>,
    now: f64,
    lsk: &LeapSeconds,
    limit: usize,
) -> (usize, Vec<VoidFinding>) {
    let srcs = load_sources();
    let markers = live_markers();
    let mut ok = 0usize;
    let mut findings: Vec<VoidFinding> = Vec::new();
    let mut budget = limit;
    for s in srcs.iter() {
        if s.url.starts_with("https://github.com/omegaflow/sources") {
            continue;
        }
        if s.fanout_cap > 0 || s.format == "csv_zip" || s.format == "kernel_text" {
            continue;
        }
        if matches!(
            s.format.as_str(),
            "ephemeris_binary"
                | "catalog_dastcom"
                | "netcdf"
                | "finals"
                | "ionex"
                | "alerce"
                | "catalog_tycho"
                | "spectral"
                | "lightcurve"
                | "rpw_efield"
                | "goes_xrs"
                | "gong_modes"
        ) {
            continue;
        }
        if budget == 0 {
            break;
        }
        budget -= 1;
        let mut url = s.url.clone();
        for (k, v) in &markers {
            url = url.replace(k, v);
        }
        if let Some(key) = unresolved_key(&url, env) {
            findings.push(VoidFinding {
                url: s.url.clone(),
                class: VoidClass::Key,
                detail: format!("marker {{{}}} absent in .secrets.local", key),
            });
            continue;
        }
        let mut header_void = None;
        for (_, v) in &s.headers {
            if let Some(key) = unresolved_key(v, env) {
                header_void = Some(key);
                break;
            }
        }
        if let Some(key) = header_void {
            findings.push(VoidFinding {
                url: s.url.clone(),
                class: VoidClass::Key,
                detail: format!("header marker {{{}}} absent in .secrets.local", key),
            });
            continue;
        }
        let url = resolve_secret(&url, env);
        let headers = render_headers(&s.headers, env);
        let body = match fetch_one(&url, None, &headers, s.ttl, Some(now)) {
            Some(b) => b,
            None => {
                findings.push(VoidFinding {
                    url: s.url.clone(),
                    class: VoidClass::Kaputt,
                    detail: "fetch void".into(),
                });
                continue;
            }
        };
        match extract(s, &body, now, lsk) {
            ExtractResult::Measurements(v) | ExtractResult::WithEphemeris(v, _) => {
                if v.is_empty() {
                    let diag = diagnose_no_samples(s, &body);
                    let class = if diag.contains("all containers empty")
                        || diag.contains("no rows extracted")
                    {
                        VoidClass::Quiet
                    } else {
                        VoidClass::Drift
                    };
                    findings.push(VoidFinding {
                        url: s.url.clone(),
                        class,
                        detail: diag,
                    });
                } else {
                    ok += 1;
                }
            }
        }
    }
    (ok, findings)
}

pub fn kernel_extent(
    force_type: u8,
    kernel_id: u8,
    body_props: Option<&BodyProperties>,
    tau: f64,
) -> f64 {
    if tau == 0.0 {
        return 0.0;
    }
    let p = match body_props {
        Some(p) => p,
        None => return 0.0,
    };
    if force_type == 1 {
        return p.radius_m;
    }
    let reach_time = tau;
    if kernel_id == 1 {
        return p.gaussian_inverse_square * reach_time;
    }
    if kernel_id == 2 {
        return p.gaussian_inverse * reach_time;
    }
    if kernel_id == 3 {
        return (2.0 * p.erfc * reach_time).sqrt();
    }
    if kernel_id == 4 {
        return p.exponential_decay;
    }
    if kernel_id == 5 {
        return p.patch_levy * reach_time;
    }
    0.0
}

const AUDIO_SPEED_AIR: f64 = 343.0;
const SEISMIC_BODY_SPEED: f64 = 6000.0;
const SEISMIC_SURFACE_SPEED: f64 = 3000.0;
const ADVECTIVE_BASE_SPEED: f64 = 1.0;
const DIFFUSIVITY_THERMAL: f64 = 0.3;
const DIFFUSIVITY_MOLECULAR: f64 = 0.05;

fn signal_reach(force_type: f64, advection: f64, age: f64) -> Option<f64> {
    match force_type as u8 {
        0 | 1 | 8 => Some(C_LIGHT * age),
        2 => Some(AUDIO_SPEED_AIR * age),
        3 => Some(SEISMIC_BODY_SPEED * age),
        4 => Some(SEISMIC_SURFACE_SPEED * age),
        7 => {
            if advection > 0.0 {
                Some(advection * age)
            } else {
                Some(ADVECTIVE_BASE_SPEED * age)
            }
        }
        5 => Some((2.0 * DIFFUSIVITY_THERMAL * age).sqrt()),
        6 => Some((2.0 * DIFFUSIVITY_MOLECULAR * age).sqrt()),
        _ => None,
    }
}

fn dispatch_reach(fields: &[FieldConfig], src_ttl: f64) -> Option<f64> {
    let mut reach: Option<f64> = None;
    for fc in fields {
        if let Some(rr) = signal_reach(fc.force as f64, fc.advection, src_ttl * 64.0) {
            reach = Some(reach.map_or(rr, |prev| prev.max(rr)));
        }
    }
    reach
}

fn propagation_speed(force_type: f64, advection: f64) -> Option<f64> {
    match force_type as u8 {
        0 | 1 | 8 => Some(C_LIGHT),
        2 => Some(AUDIO_SPEED_AIR),
        3 => Some(SEISMIC_BODY_SPEED),
        4 => Some(SEISMIC_SURFACE_SPEED),
        5 => Some(DIFFUSIVITY_THERMAL),
        6 => Some(DIFFUSIVITY_MOLECULAR),
        7 => {
            if advection > 0.0 {
                Some(advection)
            } else {
                Some(ADVECTIVE_BASE_SPEED)
            }
        }
        _ => None,
    }
}

fn wire_extent(extent: f64) -> f64 {
    if extent.is_finite() {
        extent
    } else {
        0.0
    }
}

pub fn sensor_config(name: &str) -> Option<BrowserSensor> {
    let kl = name.to_lowercase();
    let (force, kernel, ttl) = if kl.contains("temperature")
        || kl.contains("temp")
        || kl == "thermistor"
    {
        (5, 3, 60.0)
    } else if kl.contains("pressure") || kl.contains("baro") || kl == "pres" {
        (6, 3, 60.0)
    } else if kl.contains("humidity") || kl.contains("humid") || kl == "rh" || kl == "moisture" {
        (5, 3, 300.0)
    } else if kl.contains("wind") && kl.contains("speed") || kl == "windspeed" || kl == "anemometer"
    {
        (6, 3, 10.0)
    } else if (kl.contains("wind") && kl.contains("dir"))
        || kl == "winddirection"
        || kl == "winddir"
        || kl == "vane"
    {
        (6, 3, 10.0)
    } else if kl.contains("mic")
        || kl.contains("audio")
        || kl.contains("sound")
        || kl.contains("noise")
        || kl == "spl"
    {
        (2, 1, 0.01)
    } else if kl.contains("light")
        || kl.contains("lux")
        || kl.contains("lumin")
        || kl.contains("irradiance")
    {
        (0, 0, 10.0)
    } else if kl.contains("battery")
        && (kl.contains("level") || kl.contains("pct") || kl.contains("soc"))
    {
        (5, 3, 60.0)
    } else if kl.contains("battery") && (kl.contains("volt") || kl == "voltage") {
        (8, 5, 60.0)
    } else if kl.contains("battery") && kl.contains("current") {
        (8, 5, 10.0)
    } else if kl.contains("co2")
        || kl.contains("voc")
        || kl.contains("pm2")
        || kl.contains("pm10")
        || kl.contains("gas")
    {
        (5, 3, 300.0)
    } else if kl.contains("magnet") || kl.contains("compass") || kl.contains("b_field") {
        (0, 0, 10.0)
    } else if kl.contains("accelerometer") || kl.contains("acc") || kl.contains("vibration") {
        (3, 1, 1.0)
    } else if kl.contains("gyro") {
        (3, 1, 1.0)
    } else if kl.contains("gravity") {
        (1, 0, 10.0)
    } else if kl.contains("camera") || kl.contains("video") {
        (0, 0, 1.0 / 30.0)
    } else if kl.contains("battery") && kl.contains("charging") {
        (8, 5, 60.0)
    } else if kl.contains("gps") || kl.contains("gnss") {
        return None;
    } else if kl.starts_with("event.") {
        (0, 0, 10.0)
    } else {
        return None;
    };
    Some(BrowserSensor {
        key: name.into(),
        force,
        kernel,
        ttl,
    })
}

struct StationEntry {
    id: String,
    lat: f64,
    lon: f64,
}

struct FetchResult {
    source_idx: usize,
    channels: Vec<(Channel, FieldConfig)>,
    eph_update: Option<(String, BodyEphemeris)>,
    asteroid_samples: Vec<Sample>,
    star_samples: Vec<Sample>,
    curves: Option<Arc<CurveSet>>,
    spectral: Option<SpectralHash>,
    fetch_ok: bool,
}

struct StderrRadiator {
    last_line: String,
    interactive: bool,
}

impl Radiator for StderrRadiator {
    fn accept(&mut self, field: Arc<Buffer>) {
        let mut body_samples = 0usize;
        let mut api_samples = 0usize;
        let mut sensor_samples = 0usize;
        let mut body_src: std::collections::HashSet<String> = std::collections::HashSet::new();
        let mut api_src: std::collections::HashSet<u32> = std::collections::HashSet::new();
        for cell in field
            .cache
            .cells
            .values()
            .chain(std::iter::once(&field.cache.unbounded))
        {
            for sample in cell {
                match sample.source {
                    SampleSource::Sensor => sensor_samples += 1,
                    SampleSource::Source(idx) => {
                        api_samples += 1;
                        api_src.insert(idx);
                    }
                    SampleSource::Ephemeris => {
                        body_samples += 1;
                        body_src.insert(sample.name.split('.').next().unwrap_or("").to_string());
                    }
                }
            }
        }
        let line = format!(
            "omegaflow v{} | φ v7 | body: {} sources, {} samples | api: {} sources, {} samples | sensor: {} samples",
            env!("CARGO_PKG_VERSION"),
            body_src.len(),
            body_samples,
            api_src.len(),
            api_samples,
            sensor_samples,
        );
        let prev_len = self.last_line.chars().count();
        if self.interactive {
            let pad = " ".repeat(prev_len.saturating_sub(line.chars().count()));
            eprint!("\r{}{}", line, pad);
        } else if line != self.last_line {
            eprintln!("{}", line);
        }
        self.last_line = line;
    }
}

struct Archive {
    sources: Vec<SourceConfig>,
    body_ephemerides: Arc<HashMap<String, BodyEphemeris>>,
    field: Arc<Buffer>,
    presence: HashMap<String, (f64, f64, f64, f64, f64, f64, f64, f64, f64)>,
    declared_body: Option<DeclaredBody>,
    origins: HashMap<Origin, OriginState>,
    pck_bodies: HashMap<i32, PckBody>,
    time: Arc<Mutex<Option<LeapSeconds>>>,
    asteroid_samples: Vec<Sample>,
    star_samples: Vec<Sample>,
    curves: Option<Arc<CurveSet>>,
    spectral: Vec<SpectralHash>,
    pending_channels: Vec<(Channel, FieldConfig, u32)>,
}

fn days_to_ymd(total_days: u64) -> (u32, u32, u32) {
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

fn anchor_uses(sources: &[SourceConfig]) -> std::collections::HashMap<String, usize> {
    let mut uses = std::collections::HashMap::new();
    for s in sources {
        if s.format == "ephemeris_binary" || s.format == "kernel_text" {
            continue;
        }
        match &s.frame {
            Frame::Surface { body_name, .. } | Frame::Barycenter { body_name, .. } => {
                *uses.entry(body_name.clone()).or_insert(0) += 1;
            }
            Frame::Manifest => {}
        }
    }
    uses
}

fn spawn_ephemeris_bootstrap(
    sources: &[SourceConfig],
    guard: &std::sync::Arc<std::sync::atomic::AtomicBool>,
) {
    if guard.swap(true, std::sync::atomic::Ordering::SeqCst) {
        return;
    }
    let anchor_uses = anchor_uses(sources);
    let mut anchor_items: Vec<(usize, SourceConfig, String)> = Vec::new();
    let mut rest_items: Vec<(usize, SourceConfig, String)> = Vec::new();
    for (i, s) in sources.iter().enumerate() {
        if s.format != "ephemeris_binary" {
            continue;
        }
        let Some(body) = &s.body else {
            continue;
        };
        let tmp_path = format!("/tmp/omegaflow_eph_{}.bin", body);
        if cache_fresh(&tmp_path, s.ttl) {
            continue;
        }
        if anchor_uses.contains_key(body) {
            anchor_items.push((i, s.clone(), tmp_path));
        } else {
            rest_items.push((i, s.clone(), tmp_path));
        }
    }
    anchor_items.sort_by_key(|(_, s, _)| {
        std::cmp::Reverse(
            anchor_uses
                .get(s.body.as_deref().unwrap_or(""))
                .copied()
                .unwrap_or(0),
        )
    });
    if !anchor_items.is_empty() || !rest_items.is_empty() {
        let guard = guard.clone();
        thread::spawn(move || {
            let mut items = anchor_items;
            items.extend(rest_items);
            download_ephemeris_batch(&items);
            guard.store(false, std::sync::atomic::Ordering::SeqCst);
        });
    } else {
        guard.store(false, std::sync::atomic::Ordering::SeqCst);
    }
}

fn download_ephemeris_batch(items: &[(usize, SourceConfig, String)]) {
    if items.is_empty() {
        return;
    }
    let ttl = items[0].1.ttl;
    let parts: Vec<String> = items
        .iter()
        .map(|(_, _, p)| format!("{}.part", p))
        .collect();
    let mut pending: Vec<(usize, String, String)> = Vec::new();
    for (i, (_, _, tmp_path)) in items.iter().enumerate() {
        if let Ok(data) = std::fs::read(&parts[i]) {
            if parse_ephemeris_binary(&data).is_some() {
                let _ = std::fs::rename(&parts[i], tmp_path);
                continue;
            }
        }
        let _ = std::fs::remove_file(&parts[i]);
        pending.push((i, parts[i].clone(), tmp_path.clone()));
    }
    if pending.is_empty() {
        return;
    }
    let mut cmd = curl_base(ttl, 8);
    for (i, part, _) in &pending {
        cmd.arg("-o").arg(part).arg(&items[*i].1.url);
    }
    let output = match cmd.output() {
        Ok(o) => o,
        Err(_) => {
            eprintln!("ephemeris batch ({} files): curl void", pending.len());
            for (_, part, _) in &pending {
                let _ = std::fs::remove_file(part);
            }
            return;
        }
    };
    if output.status.success() {
        for (_, part, tmp_path) in &pending {
            if std::fs::rename(part, tmp_path).is_err() {
                let _ = std::fs::remove_file(part);
            }
        }
    } else {
        eprintln!(
            "ephemeris batch ({} files): curl returned {}: {}",
            pending.len(),
            output.status,
            String::from_utf8_lossy(&output.stderr).trim()
        );
        for (_, part, _) in &pending {
            let _ = std::fs::remove_file(part);
        }
    }
}

fn rfc1123_to_unix(s: &str) -> Option<u64> {
    let parts: Vec<&str> = s.split_whitespace().collect();
    if parts.len() < 6 {
        return None;
    }
    let day: u32 = parts[1].parse().ok()?;
    let month = match parts[2] {
        "Jan" => 1,
        "Feb" => 2,
        "Mar" => 3,
        "Apr" => 4,
        "May" => 5,
        "Jun" => 6,
        "Jul" => 7,
        "Aug" => 8,
        "Sep" => 9,
        "Oct" => 10,
        "Nov" => 11,
        "Dec" => 12,
        _ => return None,
    };
    let year: i64 = parts[3].parse().ok()?;
    let mut hms = parts[4].split(':');
    let hh: u64 = hms.next()?.parse().ok()?;
    let mm: u64 = hms.next()?.parse().ok()?;
    let ss: u64 = hms.next()?.parse().ok()?;
    let days = ymd_to_days(year, month, day)?;
    Some(days * 86400 + hh * 3600 + mm * 60 + ss)
}

fn cdn_fresh(cdn_url: &str, ttl: u64) -> bool {
    const CI_REFRESH_S: u64 = 300;
    let connect_t = ((ttl as f64) / (Φ * Φ * Φ)).ceil() as u64;
    let mut cmd = Command::new("curl");
    cmd.arg("-s")
        .arg("-I")
        .arg("-L")
        .arg("-m")
        .arg(connect_t.to_string())
        .arg(cdn_url);
    let output = match cmd.output() {
        Ok(o) => o,
        Err(_) => return false,
    };
    if !output.status.success() {
        return false;
    }
    let head = String::from_utf8_lossy(&output.stdout).to_string();
    let lm = match extract_header(&head, "last-modified") {
        Some(v) => v,
        None => return false,
    };
    let asset_ts = match rfc1123_to_unix(&lm) {
        Some(t) => t,
        None => return false,
    };
    let now = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(d) => d.as_secs(),
        Err(_) => return false,
    };
    now.saturating_sub(asset_ts) < ttl.max(CI_REFRESH_S)
}

fn fetch_one(
    url: &str,
    body: Option<&str>,
    headers: &[(String, String)],
    ttl: u64,
    now: Option<f64>,
) -> Option<String> {
    let manifest = cdn_manifest_map();
    let asset_name = |u: &str| -> String {
        manifest
            .get(u)
            .cloned()
            .unwrap_or_else(|| source_name_from_url(u))
    };
    if !url.starts_with("https://github.com/omegaflow/sources") {
        if let Some(netloc) = extract_netloc(url) {
            let name = asset_name(url);
            if !name.is_empty() {
                let cache_path = format!("/tmp/archivar_cache/{}/{}.json", netloc, name);
                if now.map_or(false, |n| cache_fresh_at(&cache_path, ttl, n)) {
                    if let Some(cached) = std::fs::read_to_string(&cache_path).ok() {
                        return Some(cached);
                    }
                }
                let cdn_url = format!("{}/{}/{}.json", crate::cdn::CDN_BASE, netloc, name);
                if cdn_fresh(&cdn_url, ttl) {
                    if let Some(cdn_body) = fetch_raw(&cdn_url, None, &[], ttl) {
                        if let Some(parent) = std::path::Path::new(&cache_path).parent() {
                            let _ = std::fs::create_dir_all(parent);
                        }
                        match std::fs::write(&cache_path, cdn_body.as_bytes()) {
                            Ok(()) => {
                                if let Some(n) = now {
                                    write_epoch_stamp(&cache_path, n);
                                }
                            }
                            Err(_) => {
                                eprintln!("cache {}: write void — refetch next cycle", cache_path)
                            }
                        }
                        return Some(cdn_body);
                    }
                }
            }
        }
    }
    let live = fetch_raw(url, body, headers, ttl);
    if let Some(ref r) = live {
        if let Some(netloc) = extract_netloc(url) {
            let name = asset_name(url);
            if !name.is_empty() {
                let cache_path = format!("/tmp/archivar_cache/{}/{}.json", netloc, name);
                if let Some(parent) = std::path::Path::new(&cache_path).parent() {
                    let _ = std::fs::create_dir_all(parent);
                }
                match std::fs::write(&cache_path, r.as_bytes()) {
                    Ok(()) => {
                        if let Some(n) = now {
                            write_epoch_stamp(&cache_path, n);
                        }
                    }
                    Err(_) => {
                        eprintln!("cache {}: write void — refetch next cycle", cache_path)
                    }
                }
            }
        }
    }
    live
}

fn cache_fresh(path: &str, ttl: u64) -> bool {
    let meta = match std::fs::metadata(path) {
        Ok(m) => m,
        Err(_) => return false,
    };
    let modified = match meta.modified() {
        Ok(m) => m,
        Err(_) => return false,
    };
    match std::time::SystemTime::now().duration_since(modified) {
        Ok(age) => age.as_secs() < ttl,
        Err(_) => false,
    }
}

fn cache_fresh_at(path: &str, ttl: u64, t_presence: f64) -> bool {
    match read_epoch_stamp(path) {
        Some(epoch) => (t_presence - epoch).abs() < ttl as f64,
        None => false,
    }
}

fn read_epoch_stamp(path: &str) -> Option<f64> {
    let stamp_path = format!("{}.epoch", path);
    std::fs::read_to_string(stamp_path)
        .ok()
        .and_then(|t| t.trim().parse::<f64>().ok())
}

fn write_epoch_stamp(path: &str, epoch: f64) {
    let stamp_path = format!("{}.epoch", path);
    if std::fs::write(&stamp_path, epoch.to_string()).is_err() {
        eprintln!(
            "cache {}: epoch stamp write void — refetch next cycle",
            path
        );
    }
}

fn machine_now_tdb() -> Option<f64> {
    embedded_lsk().and_then(|l| l.system_now_tdb())
}

fn is_leap(y: u32) -> bool {
    (y % 4 == 0 && y % 100 != 0) || y % 400 == 0
}

fn load_env() -> HashMap<String, String> {
    let mut env: HashMap<String, String> = std::env::vars().collect();
    for name in &[".env", ".secrets.local"] {
        if let Ok(content) = std::fs::read_to_string(resolve_asset(name)) {
            for line in content.lines() {
                let line = line.trim();
                if line.is_empty() || line.starts_with('#') {
                    continue;
                }
                if let Some(eq) = line.find('=') {
                    let key = line[..eq].trim().to_string();
                    let val = line[eq + 1..].trim().to_string();
                    if !env.contains_key(&key) {
                        env.insert(key, val);
                    }
                }
            }
        }
    }
    env
}

fn resolve_secret(url: &str, env: &HashMap<String, String>) -> String {
    let mut result = String::with_capacity(url.len());
    let mut rest = url;
    while let Some(start) = rest.find('{') {
        result.push_str(&rest[..start]);
        rest = &rest[start + 1..];
        if let Some(end) = rest.find('}') {
            let key = &rest[..end];
            let upper = key.to_uppercase();
            match env.get(key).or_else(|| env.get(&upper)) {
                Some(val) => result.push_str(val),
                None => eprintln!("env marker {{{}}} absent — substituting void", key),
            }
            rest = &rest[end + 1..];
        } else {
            result.push('{');
        }
    }
    result.push_str(rest);
    result
}

fn secret_resolves_void(template: &str, env: &HashMap<String, String>) -> bool {
    let mut rest = template;
    while let Some(start) = rest.find('{') {
        rest = &rest[start + 1..];
        let Some(end) = rest.find('}') else {
            return false;
        };
        let key = &rest[..end];
        let upper = key.to_uppercase();
        match env.get(key).or_else(|| env.get(&upper)) {
            Some(v) if !v.is_empty() => {}
            _ => return true,
        }
        rest = &rest[end + 1..];
    }
    false
}

fn url_has_template(url: &str) -> bool {
    let mut rest = url;
    while let Some(start) = rest.find('{') {
        rest = &rest[start + 1..];
        let Some(end) = rest.find('}') else {
            return false;
        };
        if rest[..end].chars().any(|c| c.is_ascii_lowercase()) {
            return true;
        }
        rest = &rest[end + 1..];
    }
    false
}

fn url_is_fanout(url: &str) -> bool {
    url.contains("{station}") || url.contains("{nearest_station}")
}

fn frame_anchor(frame: &Frame) -> (f64, f64) {
    match frame {
        Frame::Surface { lat, lon, .. } => (*lat, *lon),
        _ => (0.0, 0.0),
    }
}

fn ci_probe_render(
    template: &str,
    anchor: (f64, f64),
    env: &HashMap<String, String>,
) -> Option<String> {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .ok()?;
    let days = secs / 86400;
    let (ty, tm, td) = days_to_ymd(days);
    let (yy, ym, yd) = days_to_ymd(days - 1);
    let (wy, wm, wd) = days_to_ymd(days - 7);
    let hour = (secs % 86400) / 3600;
    let minute = (secs % 3600) / 60;
    let now_iso = format!("{}-{:02}-{:02}T{:02}:{:02}:00", ty, tm, td, hour, minute);
    let hour_ago_iso = {
        let dt = secs.saturating_sub(3600);
        let (h_y, h_m, h_d) = days_to_ymd(dt / 86400);
        let h_h = (dt % 86400) / 3600;
        let h_min = (dt % 3600) / 60;
        format!("{}-{:02}-{:02}T{:02}:{:02}:00", h_y, h_m, h_d, h_h, h_min)
    };
    let half = 0.5f64;
    let jd_now = 2440587.5 + secs as f64 / 86400.0;
    let url = template
        .replace("{today}", &format!("{}-{:02}-{:02}", ty, tm, td))
        .replace("{yesterday}", &format!("{}-{:02}-{:02}", yy, ym, yd))
        .replace("{week_ago}", &format!("{}-{:02}-{:02}", wy, wm, wd))
        .replace("{now}", &now_iso)
        .replace("{hour_ago}", &hour_ago_iso)
        .replace("{year}", &ty.to_string())
        .replace("{jd_now}", &format!("{:.6}", jd_now))
        .replace("{jd_start}", &format!("{:.6}", jd_now - 1.0))
        .replace("{jd_end}", &format!("{:.6}", jd_now))
        .replace("{lat}", &format!("{:.6}", anchor.0))
        .replace("{lon}", &format!("{:.6}", anchor.1))
        .replace("{lat_int}", &format!("{:.0}", anchor.0))
        .replace("{lon_int}", &format!("{:.0}", anchor.1))
        .replace("{lat_min}", &format!("{:.6}", anchor.0 - half))
        .replace("{lat_max}", &format!("{:.6}", anchor.0 + half))
        .replace("{lon_min}", &format!("{:.6}", anchor.1 - half))
        .replace("{lon_max}", &format!("{:.6}", anchor.1 + half));
    Some(resolve_secret(&url, env))
}

fn render_headers(
    headers: &[(String, String)],
    env: &HashMap<String, String>,
) -> Vec<(String, String)> {
    headers
        .iter()
        .map(|(k, v)| (k.clone(), resolve_secret(v, env)))
        .collect()
}

fn render_url(
    template: &str,
    x: f64,
    y: f64,
    z: f64,
    tdb_secs: f64,
    extent: f64,
    body_name: &str,
    eph: &HashMap<String, BodyEphemeris>,
    lsk: &LeapSeconds,
) -> Option<String> {
    let unix = match lsk.tdb_to_unix(tdb_secs) {
        Some(u) => u,
        None => return None,
    };
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
    let jd_now = format!("{:.6}", tdb_to_jd(tdb_secs));
    let jd_start = format!("{:.6}", tdb_to_jd(tdb_secs - 86400.0));

    let mut url = template
        .replace("{x}", &format!("{}", x))
        .replace("{y}", &format!("{}", y))
        .replace("{z}", &format!("{}", z))
        .replace("{jd_now}", &jd_now)
        .replace("{jd_start}", &jd_start)
        .replace("{jd_end}", &jd_now)
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
        .replace("{hour_ago}", &hour_ago)
        .replace("{year}", &ty.to_string())
        .replace("{year2}", &format!("{:02}", year2))
        .replace("{month}", &tm.to_string())
        .replace("{day}", &td.to_string())
        .replace("{yday}", &format!("{:03}", yday))
        .replace("{hour}", &format!("{:02}", q_hour))
        .replace("{minute}", &format!("{:02}", q_minute))
        .replace("{unix_now}", &unix_now)
        .replace("{unix_now_plus_3600}", &unix_now_plus_3600);

    if let Some((lat, lon)) = icrs_to_body_surface(x, y, z, tdb_secs, body_name, eph) {
        let radius_m = match eph.get(body_name).and_then(|e| e.props.as_ref()) {
            Some(p) => p.radius_m,
            None => 0.0,
        };
        let lat_str = format!("{:.6}", lat);
        let lon_str = format!("{:.6}", lon);
        url = url
            .replace("{lat}", &lat_str)
            .replace("{lon}", &lon_str)
            .replace("{lat_int}", &format!("{:.0}", lat))
            .replace("{lon_int}", &format!("{:.0}", lon));
        if radius_m > 0.0 {
            let m_per_deg =
                std::f64::consts::PI * radius_m / 180.0 * lat.to_radians().cos().max(0.0);
            if m_per_deg > 0.0 {
                let half_deg = extent / m_per_deg;
                let res = 6usize;
                url = url
                    .replace("{lat_min}", &format!("{:.*}", res, lat - half_deg))
                    .replace("{lat_max}", &format!("{:.*}", res, lat + half_deg))
                    .replace("{lon_min}", &format!("{:.*}", res, lon - half_deg))
                    .replace("{lon_max}", &format!("{:.*}", res, lon + half_deg));
                let step = half_deg * 0.5;
                let mut grid = Vec::with_capacity(16);
                let mut gla = Vec::with_capacity(4);
                let mut glo = Vec::with_capacity(4);
                for i in 0..4 {
                    for j in 0..4 {
                        grid.push(format!(
                            "{:.*},{:.*}",
                            res,
                            lat + (i as f64 - 1.5) * step,
                            res,
                            lon + (j as f64 - 1.5) * step
                        ));
                    }
                    gla.push(format!("{:.*}", res, lat + (i as f64 - 1.5) * step));
                    glo.push(format!("{:.*}", res, lon + (i as f64 - 1.5) * step));
                }
                url = url
                    .replace("{grid}", &grid.join("|"))
                    .replace("{grid_lat}", &gla.join(","))
                    .replace("{grid_lon}", &glo.join(","));
            }
        }
    }

    Some(url)
}

fn render_source_url(
    src: &SourceConfig,
    x: f64,
    y: f64,
    z: f64,
    tdb: f64,
    r: f64,
    eph: &HashMap<String, BodyEphemeris>,
    env: &HashMap<String, String>,
    lsk: &LeapSeconds,
) -> Option<String> {
    let mut url = match render_url(
        &src.url,
        x,
        y,
        z,
        tdb,
        r,
        &frame_body_name(&src.frame),
        eph,
        lsk,
    ) {
        Some(u) => u,
        None => return None,
    };
    if let Some(ref t) = src.target {
        url = url.replace("{target}", t);
    }
    if let Some(ref c) = src.catalog {
        url = url.replace("{catalog}", c);
    }
    if let Some(f) = src.max_freq {
        url = url.replace("{max_freq}", &f.to_string());
    }
    if let Some(f) = src.min_freq {
        url = url.replace("{min_freq}", &f.to_string());
    }
    if src.repeat_ra_bins > 0 {
        let ra_deg = f64::atan2(y, x).to_degrees();
        let ra_norm = ((ra_deg % 360.0) + 360.0) % 360.0;
        let bin = ((ra_norm / 360.0) * (src.repeat_ra_bins as f64)) as u32;
        let bin_str = format!("{:02}", bin);
        url = url
            .replace("{repeat_bin}", &bin_str)
            .replace("{bin}", &bin_str);
    }
    if url.contains("{nearest_station}") {
        if let Some(ref st_url) = src.stations_url {
            let stations = if let Some(body) = fetch_one(st_url, None, &[], 86400, Some(tdb)) {
                if let Some(j) = parse_json(&body) {
                    let arr = jpath_val(&j, &src.stations_path).and_then(|v| {
                        if let JsonVal::Arr(a) = v {
                            Some(a)
                        } else {
                            None
                        }
                    });
                    if let Some(arr) = arr {
                        let entries: Vec<StationEntry> = arr
                            .iter()
                            .filter_map(|s| {
                                let id = match jpath_val(s, &src.stations_id)? {
                                    JsonVal::Str(st) => st.clone(),
                                    JsonVal::Num(n) => n.to_string(),
                                    _ => return None,
                                };
                                let lat = scalar_of(jpath_val(s, &src.stations_lat)?)?;
                                let lon = scalar_of(jpath_val(s, &src.stations_lon)?)?;
                                Some(StationEntry { id, lat, lon })
                            })
                            .collect();
                        Arc::new(entries)
                    } else {
                        Arc::new(Vec::new())
                    }
                } else {
                    Arc::new(Vec::new())
                }
            } else {
                Arc::new(Vec::new())
            };
            if !stations.is_empty() {
                let (lat, lon) =
                    match icrs_to_body_surface(x, y, z, tdb, &frame_body_name(&src.frame), eph) {
                        Some(ll) => ll,
                        None => return Some(url),
                    };
                let mut best = 0usize;
                let mut best_d = f64::MAX;
                for (i, st) in stations.iter().enumerate() {
                    let d2 = (st.lat - lat).powi(2) + (st.lon - lon).powi(2);
                    if d2 < best_d {
                        best_d = d2;
                        best = i;
                    }
                }
                url = url.replace("{nearest_station}", &stations[best].id);
            }
        }
    }
    Some(resolve_secret(&url, env))
}

fn render_source_body(
    src: &SourceConfig,
    x: f64,
    y: f64,
    z: f64,
    tdb: f64,
    r: f64,
    eph: &HashMap<String, BodyEphemeris>,
    lsk: &LeapSeconds,
) -> Option<String> {
    let tmpl = src.post_body.as_ref()?;
    let mut body = match render_url(
        tmpl,
        x,
        y,
        z,
        tdb,
        r,
        &frame_body_name(&src.frame),
        eph,
        lsk,
    ) {
        Some(b) => b,
        None => return None,
    };
    if let Some(ref t) = src.target {
        body = body.replace("{target}", t);
    }
    if let Some(ref c) = src.catalog {
        body = body.replace("{catalog}", c);
    }
    if let Some(f) = src.max_freq {
        body = body.replace("{max_freq}", &f.to_string());
    }
    if let Some(f) = src.min_freq {
        body = body.replace("{min_freq}", &f.to_string());
    }
    Some(body)
}
fn angular_distance_deg(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let r1 = lat1.to_radians();
    let r2 = lat2.to_radians();
    let dlon = (lon2 - lon1).to_radians();
    let a = ((r2 - r1) * 0.5).sin().powi(2) + r1.cos() * r2.cos() * (dlon * 0.5).sin().powi(2);
    (2.0 * a.sqrt().asin()).to_degrees()
}

fn port_field_synth(
    directive: &str,
    force: &str,
    key: &str,
    name: &str,
    tau: Option<f64>,
) -> Option<String> {
    let (kernel, f) = default_kernel_for(force)?;
    let tau = tau.filter(|t| t.is_finite() && *t > 0.0)?;
    Some(format!(
        "{} {} {} {} {} 1 {} 0.0 0.0\n",
        directive, key, name, kernel, f, tau
    ))
}

fn port_block(block: &str) -> String {
    let mut head: Vec<String> = Vec::new();
    let mut force = String::new();
    let mut ttl: u64 = 0;
    let mut frame_line: Option<String> = None;
    let mut lat: Option<f64> = None;
    let mut lon: Option<f64> = None;
    let mut alt: Option<f64> = None;
    let mut map_line: Option<String> = None;
    let mut lat_key: Option<String> = None;
    let mut lon_key: Option<String> = None;
    let mut alt_key: Option<String> = None;
    let mut epoch_key: Option<String> = None;
    let mut post_body: Option<String> = None;
    let mut method_post = false;
    let mut body_target: Option<String> = None;
    let mut raw_extracts: Vec<String> = Vec::new();
    for line in block.lines() {
        let t = line.trim();
        if t.is_empty() || t.starts_with('#') {
            continue;
        }
        let parts: Vec<&str> = t.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        match parts[0] {
            "url" | "format" | "header" | "target" | "catalog" | "flux_from_mag"
            | "abs_mag_from" | "catalog_epoch" | "max_freq" | "min_freq" => {
                head.push(t.to_string());
            }
            "ttl" => {
                head.push(t.to_string());
                if let Ok(v) = parts[1].parse::<u64>() {
                    ttl = v;
                }
            }
            "on" | "at" => frame_line = Some(t.to_string()),
            "lat" if parts.len() >= 2 => {
                if let Ok(v) = parts[1].parse::<f64>() {
                    lat = Some(v);
                }
            }
            "lon" if parts.len() >= 2 => {
                if let Ok(v) = parts[1].parse::<f64>() {
                    lon = Some(v);
                }
            }
            "alt" if parts.len() >= 2 => {
                if let Ok(v) = parts[1].parse::<f64>() {
                    alt = Some(v);
                }
            }
            "force" if parts.len() >= 2 => force = parts[1].to_string(),
            "method" if parts.len() >= 2 => {
                method_post = parts[1].eq_ignore_ascii_case("post");
            }
            "body" if parts.len() >= 2 => {
                if parts[1].starts_with('{') || method_post {
                    post_body = Some(parts[1].to_string());
                } else {
                    body_target = Some(parts[1].to_string());
                }
            }
            "map" | "cmap" | "rows" => {
                let arg = parts.get(1).copied().unwrap_or(".");
                map_line = Some(format!("{} {}", parts[0], arg));
            }
            "lat_key" if parts.len() >= 2 => lat_key = Some(parts[1].to_string()),
            "lon_key" if parts.len() >= 2 => lon_key = Some(parts[1].to_string()),
            "alt_key" if parts.len() >= 2 => alt_key = Some(parts[1].to_string()),
            "epoch_key" if parts.len() >= 2 => epoch_key = Some(parts[1].to_string()),
            "field" | "field_in" | "first" | "last" | "count" | "path" | "deep" | "last_row"
            | "last_line" | "last_obj" | "geojson" | "regex" => {
                raw_extracts.push(t.to_string());
            }
            _ => {}
        }
    }

    let celestial = matches!(
        (lat_key.as_deref(), lon_key.as_deref()),
        (Some(k1), Some(k2))
            if (k1.eq_ignore_ascii_case("ra") || k1.eq_ignore_ascii_case("s_ra"))
                && (k2.eq_ignore_ascii_case("dec") || k2.eq_ignore_ascii_case("s_dec"))
    );
    let named_keys = lat_key
        .as_deref()
        .is_some_and(|k| k.parse::<f64>().is_err())
        && lon_key
            .as_deref()
            .is_some_and(|k| k.parse::<f64>().is_err());

    let mut out = String::new();
    for h in head.iter().filter(|h| h.starts_with("url ")) {
        out.push_str(h);
        out.push('\n');
    }
    if ttl > 0 {
        out.push_str(&format!("ttl {}\n", ttl));
    }
    for h in head.iter().filter(|h| !h.starts_with("url ")) {
        out.push_str(h);
        out.push('\n');
    }
    if let Some(b) = &post_body {
        out.push_str("post_body ");
        out.push_str(b);
        out.push('\n');
    }
    if let Some(b) = &body_target {
        out.push_str("body ");
        out.push_str(b);
        out.push('\n');
    }
    if let Some(f) = &frame_line {
        out.push_str(f);
        out.push('\n');
    } else if celestial && map_line.is_some() {
        out.push_str("at sun\n");
    } else if named_keys && map_line.is_some() {
        out.push_str("on earth 0 0\n");
    } else if let (Some(lat), Some(lon)) = (lat, lon) {
        match alt {
            Some(a) => out.push_str(&format!("on earth {} {} {}\n", lat, lon, a)),
            None => out.push_str(&format!("on earth {} {}\n", lat, lon)),
        }
    }
    if let Some(m) = &map_line {
        if celestial {
            let arg = m.splitn(2, ' ').nth(1).unwrap_or(".");
            out.push_str("cmap ");
            out.push_str(arg);
            out.push('\n');
        } else {
            out.push_str(m);
            out.push('\n');
        }
    }
    if celestial {
        if let Some(k) = &lat_key {
            out.push_str("ra ");
            out.push_str(k);
            out.push('\n');
        }
        if let Some(k) = &lon_key {
            out.push_str("dec ");
            out.push_str(k);
            out.push('\n');
        }
    } else {
        if let Some(k) = &lat_key {
            out.push_str("lat ");
            out.push_str(k);
            out.push('\n');
        }
        if let Some(k) = &lon_key {
            out.push_str("lon ");
            out.push_str(k);
            out.push('\n');
        }
        if let Some(k) = &alt_key {
            out.push_str("alt ");
            out.push_str(k);
            out.push('\n');
        }
        if let Some(k) = &epoch_key {
            out.push_str("epoch ");
            out.push_str(k);
            out.push('\n');
        }
    }
    for r in &raw_extracts {
        let parts: Vec<&str> = r.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        let s = match parts[0] {
            "field" | "field_in" if parts.len() >= 3 => {
                port_field_synth("field", &force, parts[1], parts[2], None)
            }
            "first" | "last" | "count" | "path" | "deep" if parts.len() >= 3 => {
                port_field_synth(parts[0], &force, parts[1], parts[2], None)
            }
            "last_row" if parts.len() >= 3 => {
                port_field_synth("lastrow", &force, parts[1], parts[2], None)
            }
            "last_line" if parts.len() >= 2 => Some(format!("lastline {}", parts[1])),
            "last_obj" if parts.len() >= 5 => {
                let name = parts[parts.len() - 1];
                let key = parts[parts.len() - 2];
                let parent = parts[1];
                let m = parts[2..parts.len() - 2].join(" ");
                Some(format!("lastobj {} {} {} {}", parent, m, key, name))
            }
            "geojson" if parts.len() >= 5 => None,
            "regex" if parts.len() >= 3 => {
                let name = parts[parts.len() - 1];
                let pat = parts[1..parts.len() - 1].join(" ");
                port_field_synth("regex", &force, &pat, name, None)
            }
            _ => None,
        };
        if let Some(s) = s {
            out.push_str(s.trim_end());
            out.push('\n');
        }
    }
    out
}

fn flush_port_block(block: &str, converted: &mut String, total: &mut usize, parsed: &mut usize) {
    *total += 1;
    let conv = port_block(block);
    if !parse_sources(&conv).is_empty() {
        *parsed += 1;
        converted.push_str(&conv);
        converted.push('\n');
    }
}

fn port_mode(input: &str, output: &str) -> i32 {
    let content = match std::fs::read_to_string(input) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("--port: input unreadable: {}", input);
            return 1;
        }
    };
    let mut converted =
        String::from("# port conversion (source grammar → canonical grammar, mechanical)\n");
    let mut block = String::new();
    let mut total = 0usize;
    let mut parsed = 0usize;
    let mut in_source = false;
    for line in content.lines() {
        let t = line.trim_start();
        if t.starts_with("source ") {
            if !block.is_empty() {
                flush_port_block(&block, &mut converted, &mut total, &mut parsed);
                block = String::new();
            }
            in_source = true;
            block.push_str(line);
            block.push('\n');
            continue;
        }
        if t.starts_with("url ") && !in_source {
            if !block.is_empty() {
                flush_port_block(&block, &mut converted, &mut total, &mut parsed);
                block = String::new();
            }
            block.push_str(line);
            block.push('\n');
            continue;
        }
        block.push_str(line);
        block.push('\n');
    }
    if !block.is_empty() {
        flush_port_block(&block, &mut converted, &mut total, &mut parsed);
    }
    if std::fs::write(output, &converted).is_err() {
        eprintln!("--port: output unwritable: {}", output);
        return 1;
    }
    eprintln!(
        "--port: {} blocks converted, {} parse in the current parser → {}",
        total, parsed, output
    );
    0
}

fn parse_station_entries(j: &JsonVal, src: &SourceConfig) -> Vec<StationEntry> {
    let arr = match jpath_val(j, &src.stations_path) {
        Some(JsonVal::Arr(a)) => a.iter().collect::<Vec<_>>(),
        _ => return Vec::new(),
    };
    let mut stations: Vec<StationEntry> = Vec::new();
    let mut push_entry = |id_ref: &JsonVal, lat: f64, lon: f64| {
        let id = match id_ref {
            JsonVal::Str(s) => s.clone(),
            JsonVal::Num(n) => n.to_string(),
            _ => return,
        };
        stations.push(StationEntry { id, lat, lon });
    };
    let filter_ok = |v: &JsonVal| -> bool {
        match &src.stations_filter {
            Some((k, want)) => match jpath_val(v, k) {
                Some(JsonVal::Str(s)) => s == want,
                _ => false,
            },
            None => true,
        }
    };
    for v in arr {
        let lat = match jpath(v, &src.stations_lat) {
            Some(l) => l,
            None => continue,
        };
        let lon = match jpath(v, &src.stations_lon) {
            Some(l) => l,
            None => continue,
        };
        if src.stations_flatten.is_empty() {
            if filter_ok(v) {
                match jpath_val(v, &src.stations_id) {
                    Some(id_ref) => push_entry(id_ref, lat, lon),
                    None => {}
                }
            }
        } else if let Some(JsonVal::Arr(elems)) = jpath_val(v, &src.stations_flatten) {
            for e in elems {
                if !filter_ok(e) {
                    continue;
                }
                match jpath_val(e, &src.stations_id) {
                    Some(id_ref) => push_entry(id_ref, lat, lon),
                    None => push_entry(v, lat, lon),
                }
            }
        }
    }
    stations
}

fn parse_stations_xml(body: &str) -> Vec<StationEntry> {
    let mut out = Vec::new();
    for obs in body.split("<Observatory>").skip(1) {
        let tag = |name: &str| -> Option<&str> {
            let open = format!("<{}>", name);
            let start = obs.find(&open)? + open.len();
            let end = obs[start..].find(&format!("</{}>", name))? + start;
            Some(&obs[start..end])
        };
        let code = match tag("Code") {
            Some(c) => c.trim().to_lowercase(),
            None => continue,
        };
        let lat = match tag("Latitude").and_then(|s| s.trim().parse::<f64>().ok()) {
            Some(v) => v,
            None => continue,
        };
        let lon = match tag("Longitude").and_then(|s| s.trim().parse::<f64>().ok()) {
            Some(v) => v,
            None => continue,
        };
        out.push(StationEntry { id: code, lat, lon });
    }
    out
}

fn fanout_fetch(
    src: &SourceConfig,
    stations_url_tmpl: &str,
    x: f64,
    y: f64,
    z: f64,
    presence: Option<(f64, f64, f64)>,
    now: f64,
    r: f64,
    eph: &HashMap<String, BodyEphemeris>,
    env: &HashMap<String, String>,
    lsk: &LeapSeconds,
) -> Vec<(Channel, FieldConfig)> {
    let mut channels = Vec::new();
    let body_name = frame_body_name(&src.frame);
    let (ux, uy, uz) = presence.unwrap_or((x, y, z));
    let stations_url = match render_url(stations_url_tmpl, ux, uy, uz, now, r, &body_name, eph, lsk)
    {
        Some(u) => resolve_secret(&u, env),
        None => return channels,
    };
    let headers = render_headers(&src.headers, env);
    let raw = match fetch_raw(&stations_url, None, &headers, src.ttl) {
        Some(v) => v,
        None => return channels,
    };
    let mut stations = match parse_json(&raw) {
        Some(j) => parse_station_entries(&j, src),
        None => parse_stations_xml(&raw),
    };
    let sort_center = match presence {
        Some((px, py, pz)) => icrs_to_body_surface(px, py, pz, now, &body_name, eph),
        None => None,
    }
    .or_else(|| {
        if let Frame::Surface { lat, lon, .. } = src.frame {
            Some((lat, lon))
        } else {
            None
        }
    });
    if let Some((clat, clon)) = sort_center {
        stations.sort_by(|a, b| {
            angular_distance_deg(a.lat, a.lon, clat, clon)
                .partial_cmp(&angular_distance_deg(b.lat, b.lon, clat, clon))
                .unwrap_or(std::cmp::Ordering::Equal)
        });
    }
    let cap = src.fanout_cap as usize;
    let base_url = match render_url(&src.url, ux, uy, uz, now, r, &body_name, eph, lsk) {
        Some(u) => u,
        None => return channels,
    };
    let body = render_source_body(src, ux, uy, uz, now, r, eph, lsk);
    let window = 3usize;
    let chunks: Vec<&StationEntry> = stations.iter().take(cap).collect();
    for (wi, chunk) in chunks.chunks(window).enumerate() {
        if wi > 0 && src.fanout_delay > 0 {
            thread::sleep(std::time::Duration::from_secs(src.fanout_delay));
        }
        thread::scope(|s| {
            let handles: Vec<_> = chunk
                .iter()
                .map(|st| {
                    let base_url = base_url.clone();
                    let body = body.clone();
                    let headers = headers.clone();
                    let body_name = body_name.clone();
                    let st = *st;
                    s.spawn(move || -> Vec<(Channel, FieldConfig)> {
                        let url = resolve_secret(&base_url.replace("{station}", &st.id), env);
                        let post = body.as_deref().map(|b| b.replace("{station}", &st.id));
                        let raw = match fetch_raw(&url, post.as_deref(), &headers, src.ttl) {
                            Some(v) => v,
                            None => {
                                eprintln!("station {}: fetch void — retry in ttl/Φ", st.id);
                                return Vec::new();
                            }
                        };
                        let mut out = Vec::new();
                        if let ExtractResult::Measurements(mut cs) = extract(src, &raw, now, lsk) {
                            if cs.is_empty() {
                                eprintln!("station {}: extract returned no measurements", st.id);
                            }
                            for (mut ch, fc) in cs.drain(..) {
                                ch.position = Position::Surface {
                                    body_name: body_name.clone(),
                                    lat: st.lat,
                                    lon: st.lon,
                                    alt: 0.0,
                                };
                                out.push((ch, fc));
                            }
                        }
                        out
                    })
                })
                .collect();
            for h in handles {
                if let Ok(v) = h.join() {
                    channels.extend(v);
                }
            }
        });
    }
    channels
}

fn build_netcdf_channels(
    src: &SourceConfig,
    bytes: &[u8],
    lsk: &LeapSeconds,
) -> Vec<(Channel, FieldConfig)> {
    let bytes = if bytes.starts_with(&[0x1f, 0x8b]) {
        match gunzip(bytes) {
            Some(b) => b,
            None => return Vec::new(),
        }
    } else {
        bytes.to_vec()
    };
    let nc = match NetcdfFile::parse(&bytes) {
        Ok(f) => f,
        Err(note) => {
            eprintln!("netcdf {}: {:?}", src.url, note);
            return Vec::new();
        }
    };
    let mut channels = Vec::new();
    for ext in &src.extracts {
        let Extract::ProfileMap {
            lat_key,
            lon_key,
            epoch_key,
            pressure_var,
            pressure_scale,
            fields,
            ..
        } = ext
        else {
            continue;
        };
        let Some(lat_v) = nc.values_f64(&bytes, lat_key) else {
            continue;
        };
        let Some(lon_v) = nc.values_f64(&bytes, lon_key) else {
            continue;
        };
        let Some(juld_v) = nc.values_f64(&bytes, epoch_key) else {
            continue;
        };
        let Some(pres_v) = nc.values_f32(&bytes, pressure_var) else {
            continue;
        };
        let n_prof = lat_v.len().min(lon_v.len()).min(juld_v.len());
        let n_levels = match nc.var(pressure_var).and_then(|v| nc.var_shape(v).ok()) {
            Some(shape) => match shape.get(1) {
                Some(&n) => n as usize,
                None => continue,
            },
            None => continue,
        };
        if n_levels == 0 || pres_v.len() < n_prof * n_levels {
            continue;
        }
        let pres_fill = nc
            .var(pressure_var)
            .and_then(|v| v.attrs.iter().find(|a| a.name == "_FillValue"))
            .and_then(|a| nc.attr_num(a));
        let lat_fill = nc
            .var(lat_key)
            .and_then(|v| v.attrs.iter().find(|a| a.name == "_FillValue"))
            .and_then(|a| nc.attr_num(a));
        let lon_fill = nc
            .var(lon_key)
            .and_then(|v| v.attrs.iter().find(|a| a.name == "_FillValue"))
            .and_then(|a| nc.attr_num(a));
        let juld_fill = nc
            .var(epoch_key)
            .and_then(|v| v.attrs.iter().find(|a| a.name == "_FillValue"))
            .and_then(|a| nc.attr_num(a));
        for p in 0..n_prof {
            let lat = lat_v[p];
            let lon = lon_v[p];
            let juld = juld_v[p];
            if !lat.is_finite()
                || !lon.is_finite()
                || !juld.is_finite()
                || lat_fill.map_or(false, |f| lat == f)
                || lon_fill.map_or(false, |f| lon == f)
                || juld_fill.map_or(false, |f| juld == f)
            {
                continue;
            }
            let unix = (juld - 7305.0) * 86400.0;
            let Some(epoch) = lsk.unix_to_tdb(unix) else {
                continue;
            };
            for fc in fields {
                let Some(vals) = nc.values_f32(&bytes, &fc.key) else {
                    continue;
                };
                if vals.len() < n_prof * n_levels {
                    continue;
                }
                let fill = nc
                    .var(&fc.key)
                    .and_then(|v| v.attrs.iter().find(|a| a.name == "_FillValue"))
                    .and_then(|a| nc.attr_num(a));
                for k in 0..n_levels {
                    let pres = pres_v[p * n_levels + k];
                    let val = vals[p * n_levels + k];
                    if !val.is_finite()
                        || !pres.is_finite()
                        || fill.map_or(false, |f| (val as f64) == f)
                        || pres_fill.map_or(false, |f| (pres as f64) == f)
                    {
                        continue;
                    }
                    let position = Position::Surface {
                        body_name: frame_body_name(&src.frame),
                        lat,
                        lon,
                        alt: -(pres as f64) * pressure_scale,
                    };
                    channels.push((
                        Channel {
                            z: 0.0,
                            freq: 0.0,
                            bin_width: 0.0,
                            epoch,
                            position,
                            name: fc.name.clone(),
                            value: val as f64,
                        },
                        fc.clone(),
                    ));
                }
            }
        }
    }
    channels
}

fn build_finals_channels(
    src: &SourceConfig,
    text: &str,
    lsk: &LeapSeconds,
) -> Vec<(Channel, FieldConfig)> {
    let mut channels = Vec::new();
    let mut fields: Vec<&FieldConfig> = Vec::new();
    for ext in &src.extracts {
        if let Extract::Field(fc) = ext {
            fields.push(fc);
        }
    }
    if fields.is_empty() {
        return channels;
    }
    for line in text.lines().rev() {
        if line.len() < 58 {
            continue;
        }
        let col = |a: usize, b: usize| -> Option<f64> {
            line.get(a..b).unwrap_or("").trim().parse::<f64>().ok()
        };
        let (Some(mjd), Some(pmx), Some(pmy), Some(ut1)) =
            (col(7, 15), col(18, 27), col(38, 47), col(60, 70))
        else {
            continue;
        };
        let unix = (mjd - 40587.0) * 86400.0;
        let Some(epoch) = lsk.unix_to_tdb(unix) else {
            continue;
        };
        let position = Position::Source;
        for fc in &fields {
            let value = match fc.key.as_str() {
                "ut1_utc" => ut1,
                "pmx" => pmx,
                "pmy" => pmy,
                _ => continue,
            };
            channels.push((
                Channel {
                    z: 0.0,
                    freq: 0.0,
                    bin_width: 0.0,
                    epoch,
                    position: position.clone(),
                    name: fc.name.clone(),
                    value,
                },
                (*fc).clone(),
            ));
        }
        break;
    }
    channels
}

fn build_ionex_channels(
    src: &SourceConfig,
    text: &str,
    now: f64,
    lsk: &LeapSeconds,
) -> Vec<(Channel, FieldConfig)> {
    let mut channels = Vec::new();
    let mut tec_field: Option<&FieldConfig> = None;
    for ext in &src.extracts {
        if let Extract::Field(fc) = ext {
            if fc.key == "tec" {
                tec_field = Some(fc);
            }
        }
    }
    let Some(fc) = tec_field else {
        return channels;
    };
    let mut exponent: Option<f64> = None;
    for line in text.lines() {
        if line.ends_with("EXPONENT") {
            exponent = line
                .split_whitespace()
                .next()
                .and_then(|t| t.parse::<f64>().ok());
            break;
        }
        if line.contains("START OF TEC MAP") {
            break;
        }
    }
    let Some(exp) = exponent else {
        return channels;
    };
    let mut lines = text.lines().peekable();
    let mut best: Option<(f64, f64, Vec<(f64, f64, f64)>)> = None;
    while let Some(l) = lines.next() {
        if !l.trim_end().ends_with("START OF TEC MAP") {
            continue;
        }
        let Some(ep_line) = lines.next() else { break };
        let t: Vec<f64> = ep_line
            .split_whitespace()
            .filter_map(|t| t.parse::<f64>().ok())
            .collect();
        if t.len() < 6 {
            continue;
        }
        let (y, mo, d) = (t[0] as i64, t[1] as u32, t[2] as u32);
        let Some(days) = ymd_to_days(y, mo, d) else {
            continue;
        };
        let unix = days as f64 * 86400.0 + t[3] * 3600.0 + t[4] * 60.0 + t[5];
        let Some(epoch) = lsk.unix_to_tdb(unix) else {
            continue;
        };
        if epoch > now {
            continue;
        }
        let Some(hdr) = lines.next() else { break };
        let getp = |a: usize, b: usize| -> Option<f64> {
            hdr.get(a..b).unwrap_or("").trim().parse::<f64>().ok()
        };
        let (Some(lon1), Some(lon2), Some(dlon), Some(h)) =
            (getp(8, 14), getp(14, 20), getp(20, 26), getp(26, 32))
        else {
            continue;
        };
        if dlon == 0.0 {
            continue;
        }
        let nlon = ((lon2 - lon1) / dlon).round() as i64 + 1;
        let mut pts: Vec<(f64, f64, f64)> = Vec::new();
        loop {
            let Some(cur) = lines.next() else { break };
            if cur.trim_end().ends_with("END OF TEC MAP") {
                break;
            }
            let Some(lat) = cur.get(2..8).and_then(|s| s.trim().parse::<f64>().ok()) else {
                break;
            };
            let mut remaining = nlon as usize;
            let mut row: Option<&str> = Some(cur);
            loop {
                let Some(r) = row else { break };
                let take = remaining.min(16);
                for k in 0..take {
                    let idx = nlon as usize - remaining + k;
                    if let Some(v) = r
                        .get(32 + 5 * k..32 + 5 * (k + 1))
                        .and_then(|s| s.trim().parse::<f64>().ok())
                    {
                        let tec = v * 10f64.powf(exp);
                        if tec >= 0.0 {
                            pts.push((lat, lon1 + idx as f64 * dlon, tec));
                        }
                    }
                }
                remaining -= take;
                if remaining == 0 {
                    break;
                }
                row = lines.next();
            }
        }
        if best.as_ref().map_or(true, |(be, _, _)| epoch > *be) {
            best = Some((epoch, h * 1000.0, pts));
        }
    }
    if let Some((epoch, alt, pts)) = best {
        let body = frame_body_name(&src.frame);
        for (lat, lon, tec) in pts {
            channels.push((
                Channel {
                    z: 0.0,
                    freq: 0.0,
                    bin_width: 0.0,
                    epoch,
                    position: Position::Surface {
                        body_name: body.clone(),
                        lat,
                        lon,
                        alt,
                    },
                    name: fc.name.clone(),
                    value: tec,
                },
                fc.clone(),
            ));
        }
    }
    channels
}

fn alerce_objects(json: &JsonVal) -> Vec<(String, f64, f64)> {
    let mut out = Vec::new();
    let JsonVal::Obj(root) = json else {
        return out;
    };
    let Some(JsonVal::Arr(items)) = root.get("items") else {
        return out;
    };
    for it in items {
        let JsonVal::Obj(o) = it else { continue };
        let (Some(JsonVal::Str(oid)), Some(ra), Some(dec)) = (
            o.get("oid"),
            o.get("meanra").and_then(scalar_of),
            o.get("meandec").and_then(scalar_of),
        ) else {
            continue;
        };
        if ra.is_finite() && dec.is_finite() {
            out.push((oid.clone(), ra, dec));
        }
    }
    out
}

fn alerce_detection_rows(json: &JsonVal) -> Vec<(f64, f64, f64, f64, f64)> {
    let mut out = Vec::new();
    let JsonVal::Arr(rows) = json else {
        return out;
    };
    for r in rows {
        let JsonVal::Obj(o) = r else { continue };
        let (Some(ra), Some(dec), Some(mjd), Some(magpsf), Some(magap)) = (
            o.get("ra").and_then(scalar_of),
            o.get("dec").and_then(scalar_of),
            o.get("mjd").and_then(scalar_of),
            o.get("magpsf").and_then(scalar_of),
            o.get("magap").and_then(scalar_of),
        ) else {
            continue;
        };
        if ra.is_finite() && dec.is_finite() && mjd.is_finite() {
            out.push((ra, dec, mjd, magpsf, magap));
        }
    }
    out
}

fn build_alerce_channels(
    src: &SourceConfig,
    cap: usize,
    delay: u64,
) -> Vec<(Channel, FieldConfig)> {
    let channels = Vec::new();
    let Some(Extract::Alerce(detail)) = src
        .extracts
        .iter()
        .find(|e| matches!(e, Extract::Alerce(_)))
    else {
        return channels;
    };
    let Some(list_bytes) = fetch_raw_bytes(&src.url, src.ttl) else {
        return channels;
    };
    let Some(list_json) = parse_json(&String::from_utf8_lossy(&list_bytes)) else {
        return channels;
    };
    let objects = alerce_objects(&list_json);
    for (wi, (oid, _, _)) in objects.iter().take(cap).enumerate() {
        if wi > 0 && delay > 0 {
            thread::sleep(std::time::Duration::from_secs(delay));
        }
        let url = detail.replace("{oid}", oid);
        let Some(det_bytes) = fetch_raw_bytes(&url, src.ttl) else {
            continue;
        };
        let Some(det_json) = parse_json(&String::from_utf8_lossy(&det_bytes)) else {
            continue;
        };
        let detections = alerce_detection_rows(&det_json);
        if !detections.is_empty() {
            eprintln!(
                    "alerce {}: {} detections without distance — dark until a distance channel exists (pending)",
                    oid,
                    detections.len()
                );
        }
    }
    channels
}

pub fn anchor(
    channel: &Channel,
    sensor: &FieldConfig,
    source_ttl: f64,
    source_idx: Option<u32>,
    frame: Option<&Frame>,
    mut origin_state: Option<&mut OriginState>,
    eph: &HashMap<String, BodyEphemeris>,
) -> Option<Sample> {
    if sensor.tau <= 0.0 {
        return None;
    }
    let motion = match &channel.position {
        Position::StateVector { p, v, .. } => Motion::Linear { p: *p, v: *v },
        Position::Surface {
            body_name,
            lat,
            lon,
            alt,
        } => {
            if eph
                .get(body_name.as_str())
                .and_then(|e| e.props.as_ref())
                .is_none()
            {
                return None;
            }
            Motion::Surface {
                body_name: body_name.clone(),
                lat: *lat,
                lon: *lon,
                alt: *alt,
            }
        }
        Position::SurfaceFlow {
            body_name,
            lat,
            lon,
            alt,
            speed,
            track,
            vrate,
        } => {
            if eph
                .get(body_name.as_str())
                .and_then(|e| e.props.as_ref())
                .is_none()
            {
                return None;
            }
            let v = match vrate {
                Some(v) => *v,
                None => return None,
            };
            match surface_motion(
                body_name,
                *lat,
                *lon,
                *alt,
                *speed,
                *track,
                v,
                channel.epoch,
                eph,
            ) {
                Some(m) => m,
                None => return None,
            }
        }
        Position::Source => match frame {
            Some(f) => match frame_motion(f, None, None, channel.epoch, eph) {
                Some(m) => m,
                None => return None,
            },
            None => return None,
        },
    };
    let abs = match motion.at(channel.epoch, channel.epoch, eph) {
        Some(p) => p,
        None => return None,
    };
    if !abs[0].is_finite()
        || !abs[1].is_finite()
        || !abs[2].is_finite()
        || !channel.epoch.is_finite()
    {
        return None;
    }
    let mut resid_ema = 0.0;
    if let Some(ref mut st) = origin_state {
        if st.has_prev {
            let dt_raw = (channel.epoch - st.prev_epoch).abs();
            if dt_raw > 0.0 && source_ttl > 0.0 {
                let dt = dt_raw;
                if let Some(pm) = &st.prev_motion {
                    if let Some(pred) = pm.at(channel.epoch, st.prev_epoch, eph) {
                        let resid = ((pred[0] - abs[0]).powi(2)
                            + (pred[1] - abs[1]).powi(2)
                            + (pred[2] - abs[2]).powi(2))
                        .sqrt();
                        let alpha = 1.0 - (-dt / source_ttl).exp();
                        st.resid_ema += (resid / dt - st.resid_ema) * alpha;
                    }
                }
            }
        }
        resid_ema = st.resid_ema;
        st.prev_epoch = channel.epoch;
        st.prev_abs = abs;
        st.prev_motion = Some(motion.clone());
        st.has_prev = true;
    }
    let (anchor_vmax, anchor_amax, anchor_p0) =
        match law_bounds(&motion, channel.epoch, resid_ema, eph) {
            Some(b) => b,
            None => return None,
        };
    if !anchor_p0[0].is_finite()
        || !anchor_p0[1].is_finite()
        || !anchor_p0[2].is_finite()
        || !anchor_vmax.is_finite()
        || !anchor_amax.is_finite()
    {
        return None;
    }
    let body_props = motion
        .anchor_body()
        .and_then(|name| eph.get(name))
        .and_then(|e| e.props.as_ref());
    let extent = kernel_extent(sensor.force, sensor.kernel, body_props, sensor.tau);
    if !extent.is_finite() {
        return None;
    }
    Some(Sample {
        source: match source_idx {
            Some(idx) => SampleSource::Source(idx),
            None => SampleSource::Sensor,
        },
        epoch: channel.epoch,
        ttl: source_ttl,
        extent,
        tau: sensor.tau,
        kernel_id: sensor.kernel as f64,
        force_type: sensor.force as f64,
        absorption: sensor.absorption,
        advection: sensor.advection,
        anchor_vmax,
        anchor_amax,
        anchor_p0,
        motion: motion.clone(),
        val: match convert_to_si(channel.value, &sensor.unit) {
            Some(v) => v,
            None => {
                register_unconverted_unit(&sensor.unit, &channel.name);
                return None;
            }
        },
        name: channel.name.clone(),
        z: channel.z,
        freq: channel.freq,
        bin_width: channel.bin_width,
        color_index: 0.0,
    })
}

fn body_channels(name: &str, props: &BodyProperties, now: f64) -> Vec<(Channel, FieldConfig)> {
    let mut out = Vec::new();
    if let Some(gm) = props.gm {
        out.push((
            Channel {
                z: 0.0,
                freq: 0.0,
                bin_width: 0.0,
                name: format!("{}.mass", name),
                value: gm,
                position: Position::Source,
                epoch: now,
            },
            FieldConfig {
                key: format!("{}.mass", name),
                name: format!("{}.mass", name),
                kernel: 0,
                force: 1,
                tau: f64::INFINITY,
                absorption: 0.0,
                advection: 0.0,
                unit: String::new(),
                fold: None,
            },
        ));
    }
    if let Some((omega_g, sigma)) = props.omega_g {
        let tau = if omega_g > 0.0 { 1.0 / omega_g } else { 0.0 };
        out.push((
            Channel {
                z: 0.0,
                freq: omega_g,
                bin_width: sigma,
                name: format!("{}.omega_g", name),
                value: omega_g,
                position: Position::Source,
                epoch: now,
            },
            FieldConfig {
                key: format!("{}.omega_g", name),
                name: format!("{}.omega_g", name),
                kernel: 0,
                force: 1,
                tau,
                absorption: 0.0,
                advection: 0.0,
                unit: String::new(),
                fold: None,
            },
        ));
    }
    out
}

fn extract_netloc(url: &str) -> Option<&str> {
    let after = url
        .strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"))?;
    let netloc = after.split('/').next()?;
    Some(if let Some(s) = netloc.strip_prefix("www.") {
        s
    } else {
        netloc
    })
}

fn route_segments(url: &str) -> Option<(String, Vec<String>)> {
    let after = url
        .strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"))?;
    let (netloc, rest) = match after.split_once('/') {
        Some((n, r)) => (n, r),
        None => (after, ""),
    };
    let host = netloc.strip_prefix("www.").unwrap_or(netloc);
    let path = rest.split(|c| c == '?' || c == '#').next().unwrap_or("");
    let mut segs: Vec<String> = Vec::new();
    for s in path.split('/') {
        if s.is_empty() {
            continue;
        }
        let seg = if s.starts_with('{') && s.ends_with('}') {
            "*".to_string()
        } else {
            s.to_string()
        };
        segs.push(seg);
    }
    Some((host.to_string(), segs))
}

fn route_key(url: &str) -> Option<String> {
    let (host, segs) = route_segments(url)?;
    if segs.is_empty() {
        Some(host)
    } else {
        Some(format!("{}/{}", host, segs.join("/")))
    }
}

fn route_prefix_keys(url: &str) -> Vec<String> {
    let Some((host, segs)) = route_segments(url) else {
        return Vec::new();
    };
    let mut keys = vec![host.clone()];
    let mut acc = host;
    for s in segs {
        acc.push('/');
        acc.push_str(&s);
        keys.push(acc.clone());
    }
    keys.reverse();
    keys
}

fn source_name_from_url(url: &str) -> String {
    let s1 = match url.strip_prefix("https://") {
        Some(s) => s,
        None => url,
    };
    let s2 = match s1.strip_prefix("http://") {
        Some(s) => s,
        None => s1,
    };
    let without_scheme = match s2.strip_prefix("www.") {
        Some(s) => s,
        None => s2,
    };
    let after_domain: Vec<&str> = without_scheme.splitn(2, '/').collect();
    if after_domain.len() < 2 {
        return "index.json".to_string();
    }
    let path_and_query = after_domain[1];
    let cleaned = path_and_query
        .chars()
        .map(|c| match c {
            c if c.is_ascii_alphanumeric()
                || c == '-'
                || c == '.'
                || c == '_'
                || c == '{'
                || c == '}' =>
            {
                c
            }
            '/' | '?' | '&' | '=' => '-',
            _ => '_',
        })
        .collect::<String>();
    if cleaned.is_empty() {
        "index".to_string()
    } else {
        cleaned.trim_matches('-').to_string()
    }
}

fn probe_one(
    src: &SourceConfig,
    now: f64,
    lsk_ref: &LeapSeconds,
    void_eph: &HashMap<String, BodyEphemeris>,
    env: &HashMap<String, String>,
    fetchone: bool,
    precise: bool,
    lat: f64,
    lon: f64,
) -> (bool, String) {
    let url = match render_url(&src.url, 0.0, 0.0, 0.0, now, 0.0, "", void_eph, lsk_ref) {
        Some(u) => u,
        None => return (false, "# declined: time absent\n".to_string()),
    }
    .replace("{lat}", &format!("{:.6}", lat))
    .replace("{lon}", &format!("{:.6}", lon));
    let url = resolve_secret(&url, env);
    let url = url.replace("ZZ", "Z").replace("  ", " ");
    let headers = render_headers(&src.headers, env);
    let raw = if fetchone {
        fetch_one(&url, None, &headers, src.ttl, Some(now))
    } else {
        fetch_raw_probe(&url, None, &headers)
    };
    let parsed = raw.as_ref().and_then(|r| parse_json(r));
    let auto_ttl = raw.as_ref().and_then(|r| probe_ttl(r));
    let mut block = String::new();
    block.push_str(&format!("url {}\n", src.url));
    let ttl = match auto_ttl {
        Some(t) => t,
        None => src.ttl,
    };
    block.push_str(&format!("ttl {}\n", ttl));
    match &src.frame {
        Frame::Surface {
            body_name,
            lat,
            lon,
            alt,
        } => {
            block.push_str(&format!("on {} {:?} {:?} {:?}\n", body_name, lat, lon, alt));
        }
        Frame::Barycenter { body_name, scale } if *scale == 1.0 => {
            block.push_str(&format!("at {}\n", body_name));
        }
        Frame::Barycenter { body_name, scale } => {
            block.push_str(&format!("at {} {}\n", body_name, scale));
        }
        Frame::Manifest => {}
    }
    if let Some(p) = parsed {
        let mut fields = String::new();
        let mut coords = String::new();
        let mut map_path: Option<String> = None;
        let mut budget = 48usize;
        walk_json_probe(&p, "", &mut fields, &mut coords, &mut map_path, &mut budget);
        if map_path.is_none() && !coords.is_empty() {
            map_path = Some(".".to_string());
        }
        let precision_lines = measure_precision(&p);
        if !precision_lines.is_empty() {
            block.push_str(&precision_lines);
        }
        if let Some(ref mp) = map_path {
            if !coords.is_empty() {
                let container = if coords.contains("ra ") || coords.contains("dec ") {
                    "cmap"
                } else {
                    "map"
                };
                block.push_str(&format!("{} {}\n", container, mp));
            }
        }
        if !coords.is_empty() {
            block.push_str(&coords);
        }
        if !fields.is_empty() {
            block.push_str(&fields);
        }
    } else if let Some(ref r) = raw {
        if let Some(csv) = probe_csv(r) {
            block.push_str("format free text\n");
            block.push_str(&csv);
        }
    } else {
        block.push_str("# fetch returned void\n");
    }
    if precise && raw.is_some() {
        block.push_str(&bruteforce_precision(&url, &src.url, ttl));
    }
    let verdict = match (&raw, parse_sources(&block).first()) {
        (Some(r), Some(candidate)) => match extract(candidate, r, now, lsk_ref) {
            ExtractResult::Measurements(v) | ExtractResult::WithEphemeris(v, _) => {
                if v.is_empty() {
                    Err(diagnose_no_samples(candidate, r))
                } else {
                    Ok(v.len())
                }
            }
        },
        (Some(_), None) => Err("block refused at parse (frame/ttl/field gate)".into()),
        (None, _) => Err("fetch returned void".into()),
    };
    match verdict {
        Ok(n) => {
            let mut b = format!("# verified {} samples\n", n);
            b.push_str(&block);
            b.push('\n');
            (true, b)
        }
        Err(why) => {
            let mut b = format!("# declined: {}\n", why);
            b.push_str(&block);
            b.push('\n');
            (false, b)
        }
    }
}

fn reverify_mode(env: &HashMap<String, String>) -> i32 {
    let Some(lsk) = embedded_lsk() else {
        eprintln!("reverify: the time base is absent — no sweep without a clock");
        return 1;
    };
    let Some(now) = lsk.system_now_tdb() else {
        eprintln!("reverify: TDB absent — no sweep without a clock");
        return 1;
    };
    let (ok, findings) = live_sweep(env, now, &lsk, 600);
    eprintln!(
        "\n=== REVERIFY: {} ok, {} void (of {} tested) ===",
        ok,
        findings.len(),
        ok + findings.len()
    );
    let mut lines: Vec<String> = vec![
        format!(
            "# recheck-live {} — mechanischer Re-Verifikations-Sweep über phi/sources.φ (lebende Quellen)",
            date_str(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .map(|d| d.as_secs())
                    .unwrap_or(0)
            )
        ),
        "# Klassen: key-void (Key-Marker ohne .secrets.local) | drift-void (API-Drift — Kurationsauftrag) | ruhig-void (leer = Wahrheit) | kaputt (fetch void)".into(),
    ];
    for f in findings.iter() {
        let line = format!("recheck {} {} — {}", f.url, f.class.as_str(), f.detail);
        println!("{}", line);
        lines.push(line);
    }
    if findings.is_empty() {
        lines.push(format!(
            "recheck-live {}: 0 Befunde — alle {} getesteten Quellen extrahierten Samples",
            date_str(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .map(|d| d.as_secs())
                    .unwrap_or(0)
            ),
            ok
        ));
    }
    match std::fs::write("phi/pipeline/stage/recheck_live.φ", lines.join("\n") + "\n") {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("reverify: write phi/pipeline/stage/recheck_live.φ: {}", e);
            1
        }
    }
}

fn probe_mode(
    path: &str,
    precise: bool,
    lat: f64,
    lon: f64,
    env: &HashMap<String, String>,
    fetchone: bool,
) -> i32 {
    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("probe: read {}: {}", path, e);
            return 1;
        }
    };
    let sources = load_sources_from(&content);
    eprintln!(
        "probe: {} source blocks loaded from {}",
        sources.len(),
        path
    );
    let mut lsk: Option<LeapSeconds> = None;
    for src in sources.iter().filter(|s| s.format == "kernel_text") {
        if src.body.as_deref() != Some("naif0012") {
            continue;
        }
        if let Some(text) = fetch_one(&src.url, None, &[], src.ttl, machine_now_tdb()) {
            lsk = crate::lsk::parse(&text);
        }
    }
    if lsk.is_none() {
        if let Some(text) = fetch_one(
            "https://naif.jpl.nasa.gov/pub/naif/generic_kernels/lsk/naif0012.tls",
            None,
            &[],
            NAIF_LSK_TTL_SECS,
            machine_now_tdb(),
        ) {
            lsk = crate::lsk::parse(&text);
        }
    }
    let time_pair: Option<(f64, LeapSeconds)> = match lsk {
        Some(l) => match l.system_now_tdb() {
            Some(t) => Some((t, l)),
            None => None,
        },
        None => None,
    };
    let void_eph: HashMap<String, BodyEphemeris> = HashMap::new();
    let accepted = std::sync::atomic::AtomicUsize::new(0);
    let declined = std::sync::atomic::AtomicUsize::new(0);
    let out_lock = std::sync::Mutex::new(String::new());
    let dead_lock = std::sync::Mutex::new(String::new());
    let next = std::sync::atomic::AtomicUsize::new(0);
    let non_kernel: Vec<&SourceConfig> = sources
        .iter()
        .filter(|s| s.format != "kernel_text")
        .collect();
    match &time_pair {
        Some((now, lsk_ref)) => {
            let now = *now;
            let workers = 8.min(non_kernel.len().max(1));
            std::thread::scope(|scope| {
                for _ in 0..workers {
                    scope.spawn(|| loop {
                        let i = next.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                        if i >= non_kernel.len() {
                            break;
                        }
                        let (ok, text) = probe_one(
                            non_kernel[i],
                            now,
                            lsk_ref,
                            &void_eph,
                            env,
                            fetchone,
                            precise,
                            lat,
                            lon,
                        );
                        if ok {
                            accepted.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                            out_lock.lock().unwrap().push_str(&text);
                        } else {
                            declined.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                            dead_lock.lock().unwrap().push_str(&text);
                        }
                        std::thread::sleep(std::time::Duration::from_millis(300));
                    });
                }
            });
        }
        None => {
            for _ in &non_kernel {
                declined.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                dead_lock
                    .lock()
                    .unwrap()
                    .push_str("# declined: time absent\n");
            }
        }
    }
    let accepted = accepted.load(std::sync::atomic::Ordering::Relaxed);
    let declined = declined.load(std::sync::atomic::Ordering::Relaxed);
    let out = out_lock.into_inner().unwrap();
    let dead = dead_lock.into_inner().unwrap();
    std::fs::create_dir_all("phi/pipeline").ok();
    if std::fs::write("phi/pipeline/probe_survivors.φ", &out).is_err() {
        eprintln!("write phi/pipeline/probe_survivors.φ: the register does not remember");
    }
    if std::fs::write("phi/pipeline/probe_void.txt", &dead).is_err() {
        eprintln!("write phi/pipeline/probe_void.txt: the register does not remember");
    }
    eprintln!(
        "probe: wrote phi/pipeline/probe_survivors.φ ({} verified) and phi/pipeline/probe_void.txt ({} declined)",
        accepted, declined
    );
    0
}

fn extract_all_template_values(
    substituted_url: &str,
    template_url: &str,
) -> HashMap<String, String> {
    let mut values = HashMap::new();
    let bytes = template_url.as_bytes();
    let mut markers: Vec<(usize, usize, &str)> = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'{' {
            let start = i;
            while i < bytes.len() && bytes[i] != b'}' {
                i += 1;
            }
            if i < bytes.len() {
                i += 1;
                if let Ok(marker) = std::str::from_utf8(&bytes[start..i]) {
                    markers.push((start, i, marker));
                }
            }
        } else {
            i += 1;
        }
    }
    let mut parts: Vec<&str> = Vec::new();
    let mut prev_end = 0;
    for &(start, end, _) in &markers {
        parts.push(&template_url[prev_end..start]);
        prev_end = end;
    }
    parts.push(&template_url[prev_end..]);
    let sub = substituted_url;
    let mut pos = 0;
    for idx in 0..markers.len() {
        let part = parts[idx];
        if !part.is_empty() {
            match sub[pos..].find(part) {
                Some(offset) => pos += offset + part.len(),
                None => break,
            }
        }
        let marker = markers[idx].2;
        let next_part = parts[idx + 1];
        let val_end = if next_part.is_empty() {
            let next_const = parts[idx + 2..].iter().find(|p| !p.is_empty());
            match next_const {
                Some(nc) => match sub[pos..].find(nc) {
                    Some(p) => pos + p,
                    None => sub.len(),
                },
                None => sub.len(),
            }
        } else {
            match sub[pos..].find(next_part) {
                Some(p) => pos + p,
                None => sub.len(),
            }
        };
        let val_str = &sub[pos..val_end];
        values.insert(marker.to_string(), val_str.to_string());
        pos = val_end;
    }
    values
}

fn bruteforce_precision(substituted_url: &str, template_url: &str, ttl: u64) -> String {
    let spatial: &[&str] = &["{lat}", "{lon}", "{x}", "{y}", "{z}"];
    let has_spatial = spatial.iter().any(|v| template_url.contains(v));
    if !has_spatial {
        return String::new();
    }
    let all_values = extract_all_template_values(substituted_url, template_url);
    let baseline = fetch_raw(substituted_url, None, &[], ttl);
    let mut effective_dp: usize = 0;
    for dp in 0..=15 {
        let mut test_url = template_url.to_string();
        for (marker, value_str) in &all_values {
            let replacement: String = if spatial.contains(&marker.as_str()) {
                match marker.as_str() {
                    "{lat}" => format!("{:.lat_dp$}", 35.0, lat_dp = dp),
                    "{lon}" => format!("{:.lon_dp$}", 139.0, lon_dp = dp),
                    "{x}" => format!("{:.x_dp$}", 1.495978707e11, x_dp = dp),
                    "{y}" => format!("{:.y_dp$}", 0.0, y_dp = dp),
                    "{z}" => format!("{:.z_dp$}", 0.0, z_dp = dp),
                    _ => format!("{:.prec$}", 0.0, prec = dp),
                }
            } else {
                value_str.clone()
            };
            test_url = test_url.replace(marker, &replacement);
        }
        let body = fetch_raw(&test_url, None, &[], ttl);
        if let (Some(b), Some(base)) = (&body, &baseline) {
            if b != base {
                effective_dp = dp;
            }
        }
    }
    format!("# template_precision {}dp\n", effective_dp)
}

fn probe_ttl(body: &str) -> Option<u64> {
    let val = parse_json(body)?;
    match val {
        JsonVal::Arr(ref arr) if arr.len() >= 2 => {
            let t0 = find_timestamp(&arr[0]);
            let t1 = find_timestamp(&arr[1]);
            match (t0, t1) {
                (Some(a), Some(b)) if (a - b).abs() >= 1.0 => Some((a - b).abs() as u64),
                _ => None,
            }
        }
        JsonVal::Obj(ref map) => {
            for (k, v) in map {
                if is_time_key(k) {
                    if json_num(v).is_some() {
                        return Some(60);
                    }
                }
            }
            None
        }
        _ => None,
    }
}

fn find_timestamp(val: &JsonVal) -> Option<f64> {
    if let JsonVal::Obj(map) = val {
        for (k, v) in map {
            if is_time_key(k) {
                if let Some(n) = json_num(v) {
                    return Some(n);
                }
            }
        }
    }
    None
}

fn is_coord_key(key: &str) -> bool {
    let kl = key.to_lowercase();
    kl == "latitude"
        || kl == "lat"
        || kl == "longitude"
        || kl == "lon"
        || kl == "lng"
        || kl == "altitude"
        || kl == "alt"
        || kl == "depth"
        || kl == "solar_lat"
        || kl == "solar_lon"
        || kl == "ra"
        || kl == "dec"
        || kl.contains("raj2000")
        || kl.contains("dej2000")
}

fn draft_field_line(key: &str, force: &str, unit: &str, tau: f64) -> Option<String> {
    let fid = force_id_of(force)?;
    let kid = kernel_id_for_force(fid)?;
    Some(format!(
        "field {} {} {} {} {} {} 0.0 0.0\n",
        key, key, kid, force, unit, tau
    ))
}

fn probe_csv(raw: &str) -> Option<String> {
    let first_header = raw.lines().find_map(|line| {
        let trimmed = line.trim();
        if trimmed.starts_with('#') {
            let stripped = (if let Some(s) = trimmed.strip_prefix('#') {
                s
            } else {
                trimmed
            })
            .trim();
            if !stripped.is_empty() {
                return Some(stripped);
            }
        }
        None
    })?;
    let cols: Vec<&str> = first_header.split_whitespace().collect();
    if cols.len() <= 5 {
        return None;
    }
    let mut out = String::new();
    for col in &cols[5..] {
        let lower = col.to_lowercase();
        if lower == "yy" || lower == "mm" || lower == "dd" || lower == "hh" || lower == "min" {
            continue;
        }
        if is_unit_name(&lower) {
            continue;
        }
        if is_drop_key(&lower) {
            continue;
        }
        out.push_str(&format!("# {}\n", col));
        let (force, unit, tau) = probe_classify(col);
        if force != "DROP" {
            if let Some(line) = draft_field_line(col, &force, &unit, tau) {
                out.push_str(&line);
            }
        }
    }
    if out.is_empty() {
        None
    } else {
        Some(out)
    }
}

fn probe_classify(key: &str) -> (&str, &str, f64) {
    let kl = key.to_lowercase();
    if kl.contains("temp") || kl.contains("atmp") || kl.contains("wtmp") || kl.contains("dewp") {
        ("thermal", "C", 3600.0)
    } else if kl.contains("pres") || kl.contains("baro") {
        ("advective", "hPa", 60.0)
    } else if kl.contains("spd")
        || kl.contains("gust")
        || (kl.contains("wind") && !kl.contains("dir"))
    {
        ("advective", "m/s", 60.0)
    } else if kl.contains("dir") || kl.contains("heading") {
        ("advective", "deg", 60.0)
    } else if kl.contains("wave") || kl.contains("wvht") || kl.contains("swell") {
        ("acoustic", "m", 10.0)
    } else if kl.contains("depth") {
        ("seismic-body", "km", 10.0)
    } else if kl.contains("flux") {
        ("em", "W/m2", 3600.0)
    } else if kl == "bx"
        || kl == "by"
        || kl == "bz"
        || kl == "bt"
        || kl == "dst"
        || kl.contains("mag_")
        || kl.contains("_b_")
    {
        ("em", "nT", 60.0)
    } else if kl.contains("hum") || kl.contains("rh") || kl == "rel_hum" {
        ("diffusion", "%", 86400.0)
    } else if kl.contains("rain") || kl.contains("prcp") {
        ("acoustic", "mm", 60.0)
    } else if kl.contains("vis") {
        ("em", "km", 60.0)
    } else if kl.contains("co2")
        || kl.contains("ch4")
        || kl.contains("o3")
        || kl.contains("no2")
        || kl.contains("so2")
    {
        ("diffusion", "ppm", 86400.0)
    } else if kl.contains("vel") || kl.contains("vlct") {
        ("advective", "km/s", 60.0)
    } else if kl.contains("freq") || kl.ends_with("_hz") {
        ("em", "Hz", 60.0)
    } else if kl.contains("dens") {
        ("diffusion", "p/cm3", 3600.0)
    } else if kl.contains("conc") || kl.contains("salinity") {
        ("diffusion", "PSU", 86400.0)
    } else if kl == "db" || kl.ends_with("_db") {
        ("acoustic", "dB", 60.0)
    } else if kl.contains("discharge") {
        ("advective", "m3/s", 60.0)
    } else if kl == "v" || kl == "s" {
        ("gravity", "m", 3600.0)
    } else if kl.contains("footprint") {
        ("em", "km", 60.0)
    } else if kl.contains("volt") || kl.contains("efield") || kl.contains("potential") {
        ("electric", "V", 60.0)
    } else if kl.contains("current") && !kl.contains("ocean") {
        ("electric", "A", 60.0)
    } else if kl.contains("conduct") {
        ("electric", "S/m", 3600.0)
    } else if kl.contains("sample") || kl.contains("sort") || kl.contains("order") {
        ("DROP", "", 0.0)
    } else if kl == "mag" || kl.contains("magnitude") {
        ("seismic-body", "M", 3600.0)
    } else if kl.contains("bbox") {
        ("DROP", "", 0.0)
    } else {
        ("UNCERTAIN", "", 0.0)
    }
}

fn walk_json_probe(
    val: &JsonVal,
    prefix: &str,
    out: &mut String,
    coords: &mut String,
    map_path: &mut Option<String>,
    budget: &mut usize,
) {
    match val {
        JsonVal::Obj(map) => {
            for (k, v) in map {
                let path = if prefix.is_empty() {
                    k.clone()
                } else {
                    format!("{}.{}", prefix, k)
                };
                if is_coord_key(k) {
                    let unit = coord_unit(k);
                    let directive = coord_directive(k);
                    let exact = matches!(
                        k.to_lowercase().as_str(),
                        "lat"
                            | "latitude"
                            | "lon"
                            | "lng"
                            | "longitude"
                            | "alt"
                            | "altitude"
                            | "depth"
                    );
                    let line = format!("{} {} {}\n", directive, path, unit);
                    let marker = format!("{} ", directive);
                    let existing: Option<String> = coords
                        .lines()
                        .find(|l| l.starts_with(&marker))
                        .map(|l| l.to_string());
                    match existing {
                        None => coords.push_str(&line),
                        Some(old) => {
                            let old_key = old.split_whitespace().nth(1);
                            let old_exact = old_key.map_or(false, |ok_key| {
                                let tail = match ok_key.rfind('.') {
                                    Some(p) => &ok_key[p + 1..],
                                    None => ok_key,
                                };
                                matches!(
                                    tail.to_lowercase().as_str(),
                                    "lat"
                                        | "latitude"
                                        | "lon"
                                        | "lng"
                                        | "longitude"
                                        | "alt"
                                        | "altitude"
                                        | "depth"
                                )
                            });
                            if exact && !old_exact {
                                *coords = coords.replace(&format!("{}\n", old), &line);
                            }
                        }
                    }
                    if k == "depth" {
                        let (force, unit, tau) = probe_classify("depth");
                        if force != "DROP" {
                            if let Some(line) = draft_field_line(&path, &force, &unit, tau) {
                                out.push_str(&line);
                            }
                        }
                    }
                } else {
                    walk_json_probe(v, &path, out, coords, map_path, budget);
                }
            }
        }
        JsonVal::Arr(arr) => {
            if arr.is_empty() {
                return;
            }
            if prefix.ends_with(".coordinates") || prefix == "coordinates" {
                let lon_path = if prefix.is_empty() {
                    "coordinates.0".to_string()
                } else {
                    format!("{}.0", prefix)
                };
                let lat_path = if prefix.is_empty() {
                    "coordinates.1".to_string()
                } else {
                    format!("{}.1", prefix)
                };
                let alt_path = if prefix.is_empty() {
                    "coordinates.2".to_string()
                } else {
                    format!("{}.2", prefix)
                };
                let lon_line = format!("lon {} deg\n", lon_path);
                if !coords.contains(&lon_line) {
                    coords.push_str(&lon_line);
                }
                let lat_line = format!("lat {} deg\n", lat_path);
                if !coords.contains(&lat_line) {
                    coords.push_str(&lat_line);
                }
                let alt_line = format!("alt {} km\n", alt_path);
                if !coords.contains(&alt_line) {
                    coords.push_str(&alt_line);
                }
                return;
            }
            let first = &arr[0];
            if matches!(first, JsonVal::Obj(_)) {
                if map_path.is_none() {
                    *map_path = Some(if prefix.is_empty() {
                        ".".to_string()
                    } else {
                        prefix.to_string()
                    });
                    walk_json_probe(first, "", out, coords, map_path, budget);
                } else {
                    walk_json_probe(first, prefix, out, coords, map_path, budget);
                }
            } else {
                for (i, v) in arr.iter().enumerate() {
                    walk_json_probe(
                        v,
                        &format!("{}.{}", prefix, i),
                        out,
                        coords,
                        map_path,
                        budget,
                    );
                }
            }
        }
        JsonVal::Num(n) => {
            let key = match prefix.rfind('.') {
                Some(pos) => &prefix[pos + 1..],
                None => prefix,
            };
            if is_drop_key(key) || is_coord_key(key) {
                return;
            }
            if *budget == 0 {
                return;
            }
            *budget -= 1;
            out.push_str(&format!("# {} = {:?}\n", prefix, n));
            let (force, unit, tau) = probe_classify(key);
            if force == "UNCERTAIN" {
                out.push_str(&format!(
                    "# uncertain field {} — force/unit undetermined, review\n",
                    prefix
                ));
            } else if force != "DROP" {
                let unit_lc = unit.to_lowercase();
                let in_registry = force_id_of(&force)
                    .map(|fid| allowed_units_for_force(fid).contains(&unit_lc.as_str()))
                    .unwrap_or(false);
                if !in_registry {
                    out.push_str(&format!("# unit {} not in force registry — review\n", unit));
                }
                if let Some(line) = draft_field_line(prefix, &force, &unit, tau) {
                    out.push_str(&line);
                }
            }
        }
        JsonVal::Str(s) => {
            if let Ok(n) = s.parse::<f64>() {
                let key = match prefix.rfind('.') {
                    Some(pos) => &prefix[pos + 1..],
                    None => prefix,
                };
                if is_drop_key(key) || is_coord_key(key) {
                    return;
                }
                if *budget == 0 {
                    return;
                }
                *budget -= 1;
                out.push_str(&format!("# {} = {:?} (str)\n", prefix, n));
                let (force, unit, tau) = probe_classify(key);
                if force == "UNCERTAIN" {
                    out.push_str(&format!(
                        "# uncertain field {} — force/unit undetermined, review\n",
                        prefix
                    ));
                } else if force != "DROP" {
                    let unit_lc = unit.to_lowercase();
                    let in_registry = force_id_of(&force)
                        .map(|fid| allowed_units_for_force(fid).contains(&unit_lc.as_str()))
                        .unwrap_or(false);
                    if !in_registry {
                        out.push_str(&format!("# unit {} not in force registry — review\n", unit));
                    }
                    if let Some(line) = draft_field_line(prefix, &force, &unit, tau) {
                        out.push_str(&line);
                    }
                }
            }
        }
        _ => {}
    }
}

fn coord_unit(key: &str) -> &'static str {
    let kl = key.to_lowercase();
    if kl == "altitude" || kl == "alt" || kl.contains("depth") {
        "km"
    } else {
        "deg"
    }
}

fn coord_directive(key: &str) -> &'static str {
    let kl = key.to_lowercase();
    if kl == "altitude" || kl == "alt" || kl.contains("depth") {
        "alt"
    } else if kl == "ra" || kl.contains("raj2000") {
        "ra"
    } else if kl == "dec" || kl.contains("dej2000") {
        "dec"
    } else if kl.contains("lon") || kl == "lng" {
        "lon"
    } else {
        "lat"
    }
}

fn coord_precision(a: f64, b: f64) -> usize {
    let diff = (a - b).abs();
    if diff == 0.0 {
        return 15;
    }
    let mut p = 0;
    let mut d = diff;
    while d < 1.0 && p < 15 {
        d *= 10.0;
        p += 1;
    }
    p
}

fn measure_precision(val: &JsonVal) -> String {
    match val {
        JsonVal::Arr(arr) if arr.len() >= 2 => {
            let a = &arr[0];
            let b = &arr[1];
            let mut out = String::new();
            find_coord_precisions(a, b, "", &mut out);
            if !out.is_empty() {
                format!("# precision {}\n", out.trim())
            } else {
                String::new()
            }
        }
        JsonVal::Obj(map) => {
            if let Some(features) = map.get("features") {
                if let JsonVal::Arr(features_arr) = features {
                    if features_arr.len() >= 2 {
                        let a = &features_arr[0];
                        let b = &features_arr[1];
                        let mut out = String::new();
                        find_coord_precisions(a, b, "", &mut out);
                        if !out.is_empty() {
                            format!("# precision {}\n", out.trim())
                        } else {
                            String::new()
                        }
                    } else {
                        String::new()
                    }
                } else {
                    String::new()
                }
            } else {
                String::new()
            }
        }
        _ => String::new(),
    }
}

fn find_coord_precisions(a: &JsonVal, b: &JsonVal, prefix: &str, out: &mut String) {
    match (a, b) {
        (JsonVal::Obj(ma), JsonVal::Obj(mb)) => {
            for (k, va) in ma {
                if let Some(vb) = mb.get(k) {
                    let path = if prefix.is_empty() {
                        k.clone()
                    } else {
                        format!("{}.{}", prefix, k)
                    };
                    find_coord_precisions(va, vb, &path, out);
                }
            }
        }
        (JsonVal::Arr(aa), JsonVal::Arr(ab)) => {
            if aa.len() >= 2 && ab.len() >= 2 && prefix.ends_with("coordinates") {
                for i in 0..3.min(aa.len()).min(ab.len()) {
                    if let (JsonVal::Num(na), JsonVal::Num(nb)) = (&aa[i], &ab[i]) {
                        let p = coord_precision(*na, *nb);
                        let label = ["lon", "lat", "alt"][i.min(2)];
                        out.push_str(&format!("{}={}dp ", label, p));
                    }
                }
            }
        }
        (JsonVal::Num(na), JsonVal::Num(nb)) => {
            let p = coord_precision(*na, *nb);
            if p < 15 {
                let key = match prefix.rfind('.') {
                    Some(pos) => &prefix[pos + 1..],
                    None => prefix,
                };
                if is_drop_key(key) || is_coord_key(key) {
                    return;
                }
                out.push_str(&format!("{}={}dp ", prefix, p));
            }
        }
        (JsonVal::Str(sa), JsonVal::Str(sb)) => {
            if let (Ok(na), Ok(nb)) = (sa.parse::<f64>(), sb.parse::<f64>()) {
                let p = coord_precision(na, nb);
                if p < 15 {
                    let key = match prefix.rfind('.') {
                        Some(pos) => &prefix[pos + 1..],
                        None => prefix,
                    };
                    if is_drop_key(key) || is_coord_key(key) {
                        return;
                    }
                    out.push_str(&format!("{}={}dp ", prefix, p));
                }
            }
        }
        _ => {}
    }
}
fn load_sources_from(content: &str) -> Vec<SourceConfig> {
    parse_sources(content)
}

fn load_all_sources(dir: &str) -> Vec<SourceConfig> {
    let mut sources = Vec::new();
    let dir_path = std::path::Path::new(dir);
    if let Ok(entries) = std::fs::read_dir(dir_path) {
        for e in entries.flatten() {
            let p = e.path();
            if p.is_dir() {
                let is_fetch_only = p.file_name().is_some_and(|n| {
                    let n = n.to_string_lossy();
                    n == "research" || n == "port"
                });
                if is_fetch_only {
                    continue;
                }
                let path_str = p.to_string_lossy().to_string();
                sources.extend(load_all_sources(&path_str));
            } else if p.extension().is_some_and(|x| x == "φ") {
                if let Ok(content) = std::fs::read_to_string(&p) {
                    sources.extend(load_sources_from(&content));
                }
            }
        }
    }
    sources
}

fn check_empty_data(src: &SourceConfig, raw: &str, now: f64, lsk: &LeapSeconds) {
    if let ExtractResult::Measurements(channels) = extract(src, raw, now, lsk) {
        if channels.is_empty() {
            report_anomaly("Empty Data", &src.url, "extract returned no measurements");
        }
    }
}

fn ci_mode(dir: &str) -> i32 {
    let env = load_env();
    let sources = if dir == "phi" {
        match std::fs::read_to_string("phi/sources.φ") {
            Ok(content) => load_sources_from(&content),
            Err(e) => {
                eprintln!("ci-mode: read phi/sources.φ: {}", e);
                return 1;
            }
        }
    } else {
        load_all_sources(dir)
    };
    let mirror_enabled = dir == "phi";
    let mut lsk: Option<LeapSeconds> = None;
    for src in sources.iter().filter(|s| s.format == "kernel_text") {
        if src.body.as_deref() != Some("naif0012") {
            continue;
        }
        if let Some(text) = fetch_one(&src.url, None, &[], src.ttl, machine_now_tdb()) {
            lsk = crate::lsk::parse(&text);
        }
    }
    if lsk.is_none() {
        if let Some(text) = fetch_one(
            "https://naif.jpl.nasa.gov/pub/naif/generic_kernels/lsk/naif0012.tls",
            None,
            &[],
            NAIF_LSK_TTL_SECS,
            machine_now_tdb(),
        ) {
            lsk = crate::lsk::parse(&text);
        }
    }
    let now_tdb: Option<f64> = lsk.as_ref().and_then(|l| l.system_now_tdb());
    let total = sources.len();
    let mut reachable = 0usize;
    let mut dead = 0usize;
    let mut pending = 0usize;
    let mut mirrored = 0u32;
    let mut fresh = 0u32;
    for src in &sources {
        if src.url.starts_with("https://github.com/omegaflow/sources")
            || src.format == "ephemeris_binary"
            || src.format == "catalog_dastcom"
            || src.format == "csv_zip"
            || src.format == "kernel_text"
        {
            continue;
        }
        let headers = render_headers(&src.headers, &env);
        if headers.iter().any(|(_, v)| secret_resolves_void(v, &env)) {
            eprintln!("ci-mode: {} header secret void — pending", src.url);
            pending += 1;
            continue;
        }
        if url_is_fanout(&src.url) {
            mirror_stations(src, &headers, &mut mirrored, &mut reachable, &mut dead);
            probe_fanout(
                src,
                &headers,
                &env,
                &mut reachable,
                &mut dead,
                &mut mirrored,
                &mut pending,
            );
            continue;
        }
        if url_has_template(&src.url) {
            probe_template(
                src,
                &headers,
                &env,
                &mut reachable,
                &mut dead,
                &mut mirrored,
            );
            continue;
        }
        if secret_resolves_void(&src.url, &env) {
            eprintln!("ci-mode: {} secret void — pending", src.url);
            pending += 1;
            continue;
        }
        if src.url.contains('{') {
            let resolved = resolve_secret(&src.url, &env);
            match fetch_raw(&resolved, None, &headers, src.ttl) {
                Some(r) if parse_json(&r).is_some() => {
                    reachable += 1;
                    eprintln!("ci-mode: {} JSON ok (live-only, secret in URL)", src.url);
                }
                Some(_) => {
                    eprintln!("ci-mode: {} JSON parse void", src.url);
                    dead += 1;
                }
                None => {
                    eprintln!("ci-mode: fetch returned void for {}", src.url);
                    dead += 1;
                }
            }
            continue;
        }
        let netloc = extract_netloc(&src.url);
        let manifest = cdn_manifest_map();
        let name = manifest
            .get(&src.url)
            .cloned()
            .unwrap_or_else(|| source_name_from_url(&src.url));
        let cache_path = match (&netloc, &name) {
            (Some(nl), nm) if !nm.is_empty() => {
                Some(format!("/tmp/archivar_cache/{}/{}.json", nl, nm))
            }
            _ => None,
        };
        if let Some(cp) = &cache_path {
            if cache_fresh(cp, src.ttl) {
                fresh += 1;
                continue;
            }
        }
        let raw = match fetch_raw(&src.url, None, &headers, src.ttl) {
            Some(r) => r,
            None => {
                eprintln!("ci-mode: fetch returned void for {}", src.url);
                report_anomaly("API Unreachable", &src.url, "fetch returned void");
                dead += 1;
                continue;
            }
        };
        if parse_json(&raw).is_some() {
            reachable += 1;
            if let (Some(l), Some(now)) = (&lsk, now_tdb) {
                check_empty_data(src, &raw, now, l);
            }
            eprintln!("ci-mode: {} JSON ok", src.url);
            if let Some(cp) = &cache_path {
                if let Some(parent) = std::path::Path::new(cp).parent() {
                    let _ = std::fs::create_dir_all(parent);
                }
                let _ = std::fs::write(cp, &raw);
            }
            if mirror_enabled {
                if let Some(netloc) = extract_netloc(&src.url) {
                    let name = manifest
                        .get(&src.url)
                        .cloned()
                        .unwrap_or_else(|| source_name_from_url(&src.url));
                    let tmp_path = format!("/tmp/archivar_cache/{}/{}.json", netloc, name);
                    if std::fs::write(&tmp_path, &raw).is_ok()
                        && crate::cdn::upload_release(netloc, &tmp_path)
                    {
                        mirrored += 1;
                    }
                }
            }
        } else {
            eprintln!("ci-mode: {} JSON parse void", src.url);
            report_anomaly("Malformed Data", &src.url, "JSON parse void");
            dead += 1;
        }
    }
    eprintln!(
        "ci-mode: {}/{} reachable, {} dead, {} pending (secret void), {} mirrored to CDN, {} fresh (local TTL), mirror={}",
        reachable, total, dead, pending, mirrored, fresh, mirror_enabled
    );
    let anomalies = take_anomalies();
    if !anomalies.is_empty() {
        if std::env::var("GH_TOKEN").is_ok() {
            let date = match SystemTime::now().duration_since(UNIX_EPOCH) {
                Ok(d) => {
                    let (y, m, d) = days_to_ymd(d.as_secs() / 86400);
                    format!("{}-{:02}-{:02}", y, m, d)
                }
                Err(_) => "clock-unavailable".to_string(),
            };
            let title = format!("[Automated CI Report] Omegaflow Anomalies ({})", date);
            let body = anomaly_issue_body(&anomalies);
            match Command::new("gh")
                .arg("issue")
                .arg("create")
                .arg("--repo")
                .arg("omegaflow/omegaflow")
                .arg("--title")
                .arg(&title)
                .arg("--label")
                .arg("anomaly-report")
                .arg("--body")
                .arg(&body)
                .output()
            {
                Ok(o) if o.status.success() => {
                    eprintln!(
                        "ci-mode: anomaly issue created ({} anomalies)",
                        anomalies.len()
                    )
                }
                Ok(o) => eprintln!("ci-mode: gh issue create exited {:?}", o.status.code()),
                Err(e) => eprintln!("ci-mode: gh issue create: {}", e),
            }
        } else {
            eprintln!(
                    "ci-mode: {} anomalies, GH_TOKEN absent — the report goes to the console (no issue register)",
                    anomalies.len()
                );
            for a in &anomalies {
                eprintln!("anomaly: {} | {} | {}", a.category, a.url, a.details);
            }
        }
    }
    if dead == 0 {
        0
    } else {
        1
    }
}

fn mirror_stations(
    src: &SourceConfig,
    headers: &[(String, String)],
    mirrored: &mut u32,
    reachable: &mut usize,
    dead: &mut usize,
) {
    let Some(stations_url) = &src.stations_url else {
        return;
    };
    if url_has_template(stations_url) {
        return;
    }
    let Some(netloc) = extract_netloc(stations_url) else {
        return;
    };
    let name = source_name_from_url(stations_url);
    let cache_path = format!("/tmp/archivar_cache/{}/{}.json", netloc, name);
    if cache_fresh(&cache_path, src.ttl) {
        return;
    }
    match fetch_raw(stations_url, None, headers, src.ttl) {
        Some(raw) => {
            if parse_json(&raw).is_some() {
                *reachable += 1;
                eprintln!("ci-mode: stations {} JSON ok", stations_url);
                if let Some(parent) = std::path::Path::new(&cache_path).parent() {
                    let _ = std::fs::create_dir_all(parent);
                }
                if std::fs::write(&cache_path, &raw).is_ok()
                    && crate::cdn::upload_release(netloc, &cache_path)
                {
                    *mirrored += 1;
                }
            } else {
                eprintln!("ci-mode: stations {} JSON parse void", stations_url);
                *dead += 1;
            }
        }
        None => {
            eprintln!("ci-mode: stations fetch void {}", stations_url);
            *dead += 1;
        }
    }
}

fn probe_fanout(
    src: &SourceConfig,
    headers: &[(String, String)],
    env: &HashMap<String, String>,
    reachable: &mut usize,
    dead: &mut usize,
    mirrored: &mut u32,
    pending: &mut usize,
) {
    let Some(stations_url) = &src.stations_url else {
        *pending += 1;
        return;
    };
    let stations_url = if url_has_template(stations_url) {
        match ci_probe_render(stations_url, frame_anchor(&src.frame), env) {
            Some(u) => u,
            None => {
                *pending += 1;
                return;
            }
        }
    } else {
        resolve_secret(stations_url, env)
    };
    let raw = match fetch_raw(&stations_url, None, headers, 86400) {
        Some(r) => r,
        None => {
            eprintln!("ci-mode: fanout stations void {}", stations_url);
            *dead += 1;
            return;
        }
    };
    let stations = match parse_json(&raw) {
        Some(j) => parse_station_entries(&j, src),
        None => parse_stations_xml(&raw),
    };
    let Some(first) = stations.first() else {
        eprintln!("ci-mode: fanout no stations {}", stations_url);
        *dead += 1;
        return;
    };
    let probe_url = resolve_secret(&src.url.replace("{station}", &first.id), env)
        .replace("{nearest_station}", &first.id);
    let Some(netloc) = extract_netloc(&src.url) else {
        return;
    };
    let tag = format!("{}-template", netloc);
    let name = source_name_from_url(&src.url);
    let cache_path = format!("/tmp/archivar_cache/{}/{}.json", tag, name);
    if cache_fresh(&cache_path, src.ttl) {
        *reachable += 1;
        return;
    }
    match fetch_raw(&probe_url, None, headers, src.ttl) {
        Some(body) => {
            if parse_json(&body).is_some() {
                *reachable += 1;
                eprintln!("ci-mode: fanout probe {} JSON ok", probe_url);
                if let Some(parent) = std::path::Path::new(&cache_path).parent() {
                    let _ = std::fs::create_dir_all(parent);
                }
                if std::fs::write(&cache_path, &body).is_ok()
                    && crate::cdn::upload_release(&tag, &cache_path)
                {
                    *mirrored += 1;
                }
            } else {
                eprintln!("ci-mode: fanout probe {} JSON parse void", probe_url);
                *dead += 1;
            }
        }
        None => {
            eprintln!("ci-mode: fanout probe void {}", probe_url);
            *dead += 1;
        }
    }
}

fn probe_template(
    src: &SourceConfig,
    headers: &[(String, String)],
    env: &HashMap<String, String>,
    reachable: &mut usize,
    dead: &mut usize,
    mirrored: &mut u32,
) {
    let anchor = frame_anchor(&src.frame);
    let probe_url = match ci_probe_render(&src.url, anchor, env) {
        Some(u) => u,
        None => return,
    };
    if secret_resolves_void(&probe_url, env) {
        return;
    }
    let Some(netloc) = extract_netloc(&src.url) else {
        return;
    };
    let tag = format!("{}-template", netloc);
    let name = source_name_from_url(&src.url);
    let cache_path = format!("/tmp/archivar_cache/{}/{}.json", tag, name);
    if cache_fresh(&cache_path, src.ttl) {
        *reachable += 1;
        return;
    }
    match fetch_raw(&probe_url, None, headers, src.ttl) {
        Some(body) => {
            if parse_json(&body).is_some() {
                *reachable += 1;
                eprintln!("ci-mode: template probe {} JSON ok", probe_url);
                if let Some(parent) = std::path::Path::new(&cache_path).parent() {
                    let _ = std::fs::create_dir_all(parent);
                }
                if std::fs::write(&cache_path, &body).is_ok()
                    && crate::cdn::upload_release(&tag, &cache_path)
                {
                    *mirrored += 1;
                }
            } else {
                eprintln!("ci-mode: template probe {} JSON parse void", probe_url);
                *dead += 1;
            }
        }
        None => {
            eprintln!("ci-mode: template probe void {}", probe_url);
            *dead += 1;
        }
    }
}

fn cdn_manifest_for(urls: impl Iterator<Item = String>) -> HashMap<String, String> {
    let mut map = HashMap::new();
    let mut seen: HashMap<String, u32> = HashMap::new();
    for url in urls {
        if url.starts_with("https://github.com/omegaflow/sources") {
            continue;
        }
        let name = source_name_from_url(&url);
        let k = match seen.get_mut(&name) {
            Some(n) => {
                *n += 1;
                *n
            }
            None => {
                seen.insert(name, 1);
                continue;
            }
        };
        map.insert(url, format!("{}-{}", name, k));
    }
    map
}

fn cdn_manifest_map() -> &'static HashMap<String, String> {
    static MANIFEST: OnceLock<HashMap<String, String>> = OnceLock::new();
    MANIFEST.get_or_init(|| {
        if let Ok(content) = std::fs::read_to_string("phi/sources.φ") {
            let sources = load_sources_from(&content);
            cdn_manifest_for(sources.iter().map(|s| s.url.clone()))
        } else {
            HashMap::new()
        }
    })
}

fn json_has_key_ci(val: &JsonVal, target: &str) -> bool {
    match val {
        JsonVal::Obj(map) => {
            map.keys().any(|k| k.eq_ignore_ascii_case(target))
                || map.values().any(|v| json_has_key_ci(v, target))
        }
        JsonVal::Arr(arr) => arr.iter().any(|v| json_has_key_ci(v, target)),
        _ => false,
    }
}

fn derive_frame(parsed: &JsonVal, coords: &str) -> (String, String) {
    if coords.contains("lat ") || coords.contains("lon ") {
        (
            "on earth 0 0\n".to_string(),
            "geographic coords".to_string(),
        )
    } else if coords.contains("ra ") || coords.contains("dec ") {
        ("at sun\n".to_string(), "celestial coords".to_string())
    } else if json_has_key_ci(parsed, "ra") && json_has_key_ci(parsed, "dec") {
        ("at sun\n".to_string(), "celestial ra/dec".to_string())
    } else {
        ("".to_string(), "frame pending".to_string())
    }
}

fn draft_url_mode(path: &str, env: &HashMap<String, String>, fetchone: bool) -> i32 {
    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("--draft: read {}: {}", path, e);
            return 1;
        }
    };
    let urls: Vec<String> = content
        .lines()
        .map(|l| {
            let t = l.trim();
            let u = if t.starts_with("live ") || t.starts_with("candidate ") {
                t.split_whitespace()
                    .find(|w| w.starts_with("http"))
                    .unwrap_or("")
            } else {
                t
            };
            u.to_string()
        })
        .filter(|u| u.starts_with("http"))
        .collect();
    let total = urls.len();
    let out_lock = std::sync::Mutex::new(String::new());
    let learned_lock = std::sync::Mutex::new(HashMap::<String, String>::new());
    let drafted = std::sync::atomic::AtomicUsize::new(0);
    let next = std::sync::atomic::AtomicUsize::new(0);
    let workers = 8.min(total.max(1));
    std::thread::scope(|scope| {
        for _ in 0..workers {
            scope.spawn(|| loop {
                let i = next.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if i >= total {
                    break;
                }
                let url = resolve_secret(&urls[i], env);
                let raw = if fetchone {
                    fetch_one(&url, None, &[], 3600, machine_now_tdb())
                } else {
                    fetch_raw_probe(&url, None, &[])
                };
                if let Some(body) = raw {
                    if let Some(parsed) = parse_json(&body) {
                        let tap_flat = tap_to_json(&parsed);
                        let effective = tap_flat.as_ref().unwrap_or(&parsed);
                        let mut fields = String::new();
                        let mut coords = String::new();
                        let mut map_path: Option<String> = None;
                        let mut budget = 48usize;
                        walk_json_probe(
                            effective,
                            "",
                            &mut fields,
                            &mut coords,
                            &mut map_path,
                            &mut budget,
                        );
                        let ttl = probe_ttl(&body);
                        let (frame, reason) = derive_frame(effective, &coords);
                        if !frame.is_empty() {
                            if let Some(rk) = route_key(&urls[i]) {
                                learned_lock
                                    .lock()
                                    .unwrap()
                                    .entry(rk)
                                    .or_insert_with(|| frame.trim_end().to_string());
                            }
                        }
                        let mut block = format!("url {}\n", urls[i]);
                        if let Some(t) = ttl {
                            block.push_str(&format!("ttl {}\n", t));
                        }
                        if tap_flat.is_some() {
                            block.push_str("format tap\n");
                        }
                        block.push_str(&frame);
                        if let Some(ref mp) = map_path {
                            if !coords.is_empty() {
                                let container = if coords.contains("ra ") || coords.contains("dec ")
                                {
                                    "cmap"
                                } else {
                                    "map"
                                };
                                block.push_str(&format!("{} {}\n", container, mp));
                            }
                        }
                        if !coords.is_empty() {
                            block.push_str(&coords);
                        }
                        if !fields.is_empty() {
                            block.push_str(&fields);
                        }
                        let mut out = format!("# frame: {}\n", reason);
                        out.push_str(&block);
                        out.push('\n');
                        out_lock.lock().unwrap().push_str(&out);
                        drafted.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    }
                }
                std::thread::sleep(std::time::Duration::from_millis(300));
            });
        }
    });
    let drafted = drafted.load(std::sync::atomic::Ordering::Relaxed);
    std::fs::create_dir_all("phi/pipeline").ok();
    std::fs::write(
        "phi/pipeline/probe_drafts.φ",
        out_lock.into_inner().unwrap(),
    )
    .ok();
    let learned = learned_lock.into_inner().unwrap();
    eprintln!(
        "--draft: {} candidates, {} blocks drafted, {} frames learned → phi/pipeline/probe_drafts.φ + phi/pipeline/frame_learned.φ",
        total,
        drafted,
        learned.len()
    );
    learn_frames(&learned);
    0
}

const CELESTIAL_NETLOCS: &[&str] = &[
    "tapvizier.cds.unistra.fr",
    "vizier.cds.unistra.fr",
    "cds.unistra.fr",
    "irsa.ipac.caltech.edu",
    "dc.g-vo.org",
    "gaia.ari.uni-heidelberg.de",
    "exoplanetarchive.ipac.caltech.edu",
    "heasarc.gsfc.nasa.gov",
    "simbad.u-strasbg.fr",
    "gea.esac.esa.int",
    "wis-tns.org",
    "ssd.jpl.nasa.gov",
    "ssd-api.jpl.nasa.gov",
    "naif.jpl.nasa.gov",
    "archive.stsci.edu",
    "mast.stsci.edu",
    "archive.gemini.edu",
    "archive.nrao.edu",
    "skyserver.sdss.org",
    "atnf.csiro.au",
    "noirlab.edu",
    "eso.org",
    "astrocats.space",
];

fn draft_frame_guess(
    url: &str,
    context: &str,
    registry: &HashMap<String, String>,
) -> (String, String) {
    let netloc = extract_netloc(url).unwrap_or_default();
    for key in route_prefix_keys(url) {
        if let Some(f) = registry.get(&key) {
            return (format!("{}\n", f), format!("route-registry: {}", f));
        }
    }
    for n in CELESTIAL_NETLOCS {
        if netloc == *n || netloc.ends_with(n) {
            return ("at sun\n".to_string(), "celestial netloc".to_string());
        }
    }
    let lower = context.to_lowercase();
    for w in [
        "station",
        "buoy",
        "quake",
        "earthquake",
        "weather",
        "wind",
        "temperature",
        "water",
        "tide",
        "sea ",
        "ocean",
        "snow",
        "rain",
        "seismic",
        "metar",
        "airport",
        "pegel",
        "air quality",
        "hurricane",
    ] {
        if lower.contains(w) {
            return (
                "on earth 0 0\n".to_string(),
                format!("terrestrial vocab: {}", w.trim()),
            );
        }
    }
    ("".to_string(), "frame pending".to_string())
}

fn build_frame_registry() -> HashMap<String, String> {
    let mut map: HashMap<String, String> = HashMap::new();
    for path in [
        "phi/sources.φ",
        "phi/dead_sources.φ",
        "phi/blocked_sources.φ",
    ] {
        let Ok(content) = std::fs::read_to_string(path) else {
            continue;
        };
        let mut cur_url: Option<String> = None;
        for line in content.lines() {
            let t = line.trim();
            if let Some(rest) = t.strip_prefix("url ") {
                cur_url = Some(rest.trim().to_string());
            } else if let Some(url) = &cur_url {
                if t.starts_with("on ") {
                    if let Some(rk) = route_key(url) {
                        map.entry(rk).or_insert_with(|| "on earth".to_string());
                    }
                } else if let Some(rest) = t.strip_prefix("at ") {
                    let body = rest.split_whitespace().next().unwrap_or("sun");
                    if let Some(rk) = route_key(url) {
                        map.entry(rk).or_insert_with(|| format!("at {}", body));
                    }
                }
            }
        }
    }
    if let Ok(content) = std::fs::read_to_string("phi/pipeline/frame_learned.φ") {
        for line in content.lines() {
            let t = line.trim();
            if t.is_empty() || t.starts_with('#') {
                continue;
            }
            if let Some((nl, frame)) = t.split_once('|') {
                let nl = nl.trim();
                let frame = frame.trim();
                if !nl.is_empty() && !frame.is_empty() {
                    map.entry(nl.to_string())
                        .or_insert_with(|| frame.to_string());
                }
            }
        }
    }
    map
}

fn learn_frames(new: &HashMap<String, String>) {
    let mut map: HashMap<String, String> = HashMap::new();
    if let Ok(content) = std::fs::read_to_string("phi/pipeline/frame_learned.φ") {
        for line in content.lines() {
            let t = line.trim();
            if t.is_empty() || t.starts_with('#') {
                continue;
            }
            if let Some((nl, frame)) = t.split_once('|') {
                map.insert(nl.trim().to_string(), frame.trim().to_string());
            }
        }
    }
    for (nl, frame) in new {
        map.entry(nl.to_string())
            .or_insert_with(|| frame.to_string());
    }
    let mut out = String::from(
            "# frame-learned — route (host/path, query stripped) → frame, self-learning from probe responses (--draft)\n",
        );
    let mut keys: Vec<(&String, &String)> = map.iter().collect();
    keys.sort();
    for (nl, frame) in keys {
        out.push_str(&format!("{} | {}\n", nl, frame));
    }
    std::fs::create_dir_all("phi/pipeline").ok();
    if std::fs::write("phi/pipeline/frame_learned.φ", out).is_err() {
        eprintln!("write phi/pipeline/frame_learned.φ: the register does not remember");
    }
}

fn draft_context_mode(path: &str) -> i32 {
    let drafts = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("--draft-context: read {}: {}", path, e);
            return 1;
        }
    };
    let mut context_map: HashMap<String, String> = HashMap::new();
    for dir in ["phi/pipeline/katalog", "phi/pipeline"] {
        if let Ok(rd) = std::fs::read_dir(dir) {
            for e in rd.flatten() {
                let name = e.file_name().to_string_lossy().to_string();
                let relevant = (dir == "phi/pipeline/katalog" && name.ends_with(".φ"))
                    || (dir == "phi/pipeline"
                        && name.starts_with("weights_")
                        && name.ends_with(".txt"));
                if !relevant {
                    continue;
                }
                if let Ok(c) = std::fs::read_to_string(e.path()) {
                    for l in c.lines() {
                        let t = l.trim();
                        if let Some(pos) = t.find("http") {
                            let u = t[pos..]
                                .split_whitespace()
                                .next()
                                .unwrap_or("")
                                .trim_end_matches(|ch| ch == ',' || ch == '|' || ch == ';');
                            if u.starts_with("http") {
                                context_map
                                    .entry(u.to_string())
                                    .or_insert_with(|| t.to_string());
                            }
                        }
                    }
                }
            }
        }
    }
    let registry = build_frame_registry();
    let mut reg = String::from(
            "# frame-registry — route (host/path, query stripped) → frame, self-learning from sources.φ + dead_sources.φ + blocked_sources.φ + frame_learned.φ\n",
        );
    let mut reg_keys: Vec<(&String, &String)> = registry.iter().collect();
    reg_keys.sort();
    for (nl, f) in reg_keys {
        reg.push_str(&format!("{} | {}\n", nl, f));
    }
    if std::fs::write("phi/pipeline/frame_registry.φ", reg).is_err() {
        eprintln!("write phi/pipeline/frame_registry.φ: the register does not remember");
    }
    let mut out = String::new();
    let mut celestial = 0usize;
    let mut terrestrial = 0usize;
    let mut pending = 0usize;
    for block in drafts.split("\n\n") {
        let b = block.trim();
        if b.is_empty() {
            continue;
        }
        let mut url = "";
        let mut is_pending = false;
        for l in b.lines() {
            if l.starts_with("url ") {
                url = l.trim_start_matches("url ").trim();
            }
            if l == "# frame: frame pending" {
                is_pending = true;
            }
        }
        if !is_pending || url.is_empty() {
            out.push_str(b);
            out.push_str("\n\n");
            continue;
        }
        let context = context_map.get(url).cloned().unwrap_or_default();
        let (frame, reason) = draft_frame_guess(url, &context, &registry);
        if frame.is_empty() {
            pending += 1;
            out.push_str(b);
            out.push_str("\n\n");
            continue;
        }
        let mut lines: Vec<String> = Vec::new();
        for l in b.lines() {
            if l == "# frame: frame pending" {
                lines.push(format!("# frame: {}", reason));
                continue;
            }
            lines.push(l.to_string());
            if l.starts_with("ttl ") {
                lines.push(frame.trim_end().to_string());
            }
        }
        if frame.starts_with("at sun") {
            celestial += 1;
        } else {
            terrestrial += 1;
        }
        out.push_str(&lines.join("\n"));
        out.push_str("\n\n");
    }
    std::fs::create_dir_all("phi/pipeline").ok();
    if std::fs::write("phi/pipeline/probe_drafts_enriched.φ", out).is_err() {
        eprintln!("write phi/pipeline/probe_drafts_enriched.φ: the register does not remember");
    }
    eprintln!(
            "--draft-context: {} pending → {} celestial, {} terrestrial, {} stay pending → phi/pipeline/probe_drafts_enriched.φ",
            celestial + terrestrial + pending,
            celestial,
            terrestrial,
            pending
        );
    0
}

fn gate_learn_mode() -> i32 {
    let mut delta: Vec<(i32, String, String)> = Vec::new();
    let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();
    if let Ok(content) = std::fs::read_to_string("phi/sources.φ") {
        for line in content.lines() {
            let t = line.trim();
            if let Some(rest) = t.strip_prefix("url ") {
                if let Some(nl) = extract_netloc(rest.trim()) {
                    if seen.insert(format!("+{}", nl)) {
                        delta.push((4, "-".to_string(), nl.to_string()));
                    }
                }
            }
        }
    }
    if let Ok(content) = std::fs::read_to_string("phi/dead_sources.φ") {
        for line in content.lines() {
            let t = line.trim();
            if let Some(rest) = t.strip_prefix("url ") {
                if let Some(nl) = extract_netloc(rest.trim()) {
                    if seen.insert(format!("-{}", nl)) {
                        delta.push((-4, "-".to_string(), nl.to_string()));
                    }
                }
            }
        }
    }
    if let Ok(content) = std::fs::read_to_string("phi/blocked_sources.φ") {
        for line in content.lines() {
            let t = line.trim();
            if let Some(rest) = t.strip_prefix("url ") {
                if let Some(nl) = extract_netloc(rest.trim()) {
                    if seen.insert(format!("b{}", nl)) {
                        delta.push((-2, "b".to_string(), nl.to_string()));
                    }
                }
            }
        }
    }
    let (mut pos, mut neg) = (0usize, 0usize);
    for (w, _, _) in &delta {
        if *w > 0 {
            pos += 1;
        } else {
            neg += 1;
        }
    }
    let mut d = String::from(
            "# gate-delta — netloc weights, self-learning from sources.φ (+) + dead_sources.φ (−) + blocked_sources.φ (b)\n",
        );
    for (w, f, tag) in &delta {
        d.push_str(&format!("{} {} {}\n", w, f, tag));
    }
    if std::fs::write("phi/pipeline/library_gate_delta.φ", d).is_err() {
        eprintln!("write phi/pipeline/library_gate_delta.φ: the register does not remember");
    }
    let library = std::fs::read_to_string("phi/pipeline/library.φ").unwrap_or_default();
    let delta_lines: Vec<String> = delta
        .iter()
        .map(|(w, f, tag)| format!("{} {} {}", w, f, tag))
        .collect();
    let mut seen2: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut out = String::new();
    for line in library
        .lines()
        .chain(delta_lines.iter().map(|s| s.as_str()))
    {
        if line.starts_with('#') {
            out.push_str(line);
            out.push('\n');
            continue;
        }
        let parts: Vec<&str> = line.splitn(3, char::is_whitespace).collect();
        if parts.len() < 3 {
            continue;
        }
        if seen2.insert(parts[2].to_string()) {
            out.push_str(line);
            out.push('\n');
        }
    }
    if std::fs::write("phi/pipeline/library.φ", out).is_err() {
        eprintln!("write phi/pipeline/library.φ: the register does not remember");
    }
    eprintln!(
            "--learn-gate: {} netloc-Gewichte ({} positiv, {} negativ) → library.φ + library_gate_delta.φ",
            delta.len(),
            pos,
            neg
        );
    0
}

fn url_probe_mode(path: &str, env: &HashMap<String, String>, fetchone: bool, jina: bool) -> i32 {
    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("--urls: read {}: {}", path, e);
            return 1;
        }
    };
    let urls: Vec<String> = content
        .lines()
        .map(|l| {
            let t = l.trim();
            let u = if t.starts_with("candidate ") {
                t.trim_start_matches("candidate ")
            } else {
                t
            };
            u.split_whitespace().next().unwrap_or("").to_string()
        })
        .filter(|u| u.starts_with("http"))
        .collect();
    let total = urls.len();
    let live = std::sync::atomic::AtomicUsize::new(0);
    let void = std::sync::atomic::AtomicUsize::new(0);
    let live_lock = std::sync::Mutex::new(String::new());
    let void_lock = std::sync::Mutex::new(String::new());
    let jina_lock = std::sync::Mutex::new(Vec::<String>::new());
    let next = std::sync::atomic::AtomicUsize::new(0);
    let workers = 8.min(total.max(1));
    std::thread::scope(|scope| {
        for _ in 0..workers {
            scope.spawn(|| loop {
                let i = next.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if i >= total {
                    break;
                }
                let url = resolve_secret(&urls[i], env);
                let raw = if fetchone {
                    fetch_one(&url, None, &[], 3600, machine_now_tdb())
                } else {
                    fetch_raw_probe(&url, None, &[])
                };
                match raw {
                    Some(body) => {
                        let kind = if parse_json(&body).is_some() {
                            "json"
                        } else if body.trim_start().starts_with('<') {
                            "html"
                        } else {
                            "text"
                        };
                        live.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                        live_lock
                            .lock()
                            .unwrap()
                            .push_str(&format!("live {} | {}\n", kind, urls[i]));
                        if jina && kind != "json" {
                            jina_lock.lock().unwrap().push(urls[i].clone());
                        }
                    }
                    None => {
                        void.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                        void_lock
                            .lock()
                            .unwrap()
                            .push_str(&format!("void {}\n", urls[i]));
                    }
                }
                std::thread::sleep(std::time::Duration::from_millis(300));
            });
        }
    });
    let live = live.load(std::sync::atomic::Ordering::Relaxed);
    let void = void.load(std::sync::atomic::Ordering::Relaxed);
    std::fs::create_dir_all("phi/pipeline").ok();
    std::fs::write(
        "phi/pipeline/probe_live.txt",
        live_lock.into_inner().unwrap(),
    )
    .ok();
    std::fs::write(
        "phi/pipeline/probe_url_void.txt",
        void_lock.into_inner().unwrap(),
    )
    .ok();
    let mut jina_report = 0usize;
    if jina {
        let candidates = jina_lock.into_inner().unwrap();
        let jina_out = std::sync::Mutex::new(String::new());
        let jn = std::sync::atomic::AtomicUsize::new(0);
        let n_cand = candidates.len();
        let jina_key = env.get("JINA_API_KEY").cloned().unwrap_or_default();
        let jina_headers: Vec<(String, String)> = if jina_key.is_empty() {
            Vec::new()
        } else {
            vec![("Authorization".to_string(), format!("Bearer {}", jina_key))]
        };
        let j_workers = 4.min(n_cand.max(1));
        let j_pacing = if jina_key.is_empty() { 4000u64 } else { 500u64 };
        std::thread::scope(|scope| {
            for _ in 0..j_workers {
                scope.spawn(|| loop {
                    let i = jn.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    if i >= n_cand {
                        break;
                    }
                    let wrapped = format!("https://r.jina.ai/{}", candidates[i]);
                    let body = fetch_raw_probe(&wrapped, None, &jina_headers);
                    if let Some(b) = body {
                        if parse_json(&b).is_some() {
                            jina_out
                                .lock()
                                .unwrap()
                                .push_str(&format!("jina-json | {}\n", candidates[i]));
                        }
                    }
                    std::thread::sleep(std::time::Duration::from_millis(j_pacing));
                });
            }
        });
        let out = jina_out.into_inner().unwrap();
        jina_report = out.lines().count();
        std::fs::write("phi/pipeline/probe_jina.txt", out).ok();
    }
    eprintln!(
        "--urls: {} checked, {} live, {} void, {} jina-json → phi/pipeline/probe_live.txt + probe_url_void.txt + probe_jina.txt",
        total, live, void, jina_report
    );
    0
}

fn serial_ports() -> Vec<String> {
    let mut out = Vec::new();
    let Ok(entries) = std::fs::read_dir("/dev") else {
        return out;
    };
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().to_string();
        if name.starts_with("ttyACM") || name.starts_with("ttyUSB") {
            out.push(format!("/dev/{}", name));
        }
    }
    out
}

fn serial_ingress(tx: mpsc::Sender<Vec<(String, f64, f64)>>) {
    loop {
        for name in serial_ports() {
            let mut port = match serialport::new(&name, 115_200)
                .timeout(std::time::Duration::from_millis(50))
                .open()
            {
                Ok(p) => p,
                Err(_) => continue,
            };
            let mut line = String::new();
            let mut buf = [0u8; 256];
            let mut batch: Vec<(String, f64, f64)> = Vec::new();
            loop {
                let n = match port.read(&mut buf) {
                    Ok(n) if n > 0 => n,
                    _ => break,
                };
                for b in &buf[..n] {
                    if *b == b'\n' {
                        if let Some((k, v)) = line.split_once('=') {
                            if let Ok(val) = v.trim().parse::<f64>() {
                                batch.push((k.trim().to_string(), val, 0.0));
                            }
                        }
                        line.clear();
                        if batch.len() >= 64 {
                            let _ = tx.send(batch);
                            batch = Vec::new();
                        }
                    } else {
                        line.push(*b as char);
                    }
                    if line.len() > 256 {
                        line.clear();
                    }
                }
            }
            if !batch.is_empty() {
                let _ = tx.send(batch);
            }
        }
        thread::sleep(std::time::Duration::from_secs(5));
    }
}

fn battery_ingress(tx: mpsc::Sender<Vec<(String, f64, f64)>>) {
    loop {
        let mut batch: Vec<(String, f64, f64)> = Vec::new();
        if let Ok(entries) = std::fs::read_dir("/sys/class/power_supply") {
            for entry in entries.flatten() {
                let path = entry.path();
                let read_num = |name: &str| -> Option<f64> {
                    std::fs::read_to_string(path.join(name))
                        .ok()
                        .and_then(|s| s.trim().parse::<f64>().ok())
                };
                let capacity = read_num("capacity");
                let voltage = read_num("voltage_now").map(|v| v / 1e6);
                let current = read_num("current_now").map(|a| a / 1e6);
                let status = std::fs::read_to_string(path.join("status")).unwrap_or_default();
                if let Some(c) = capacity {
                    batch.push(("battery.level".to_string(), c, 60.0));
                }
                if let Some(v) = voltage {
                    batch.push(("battery.voltage".to_string(), v, 60.0));
                }
                if let Some(a) = current {
                    batch.push(("battery.current".to_string(), a, 10.0));
                }
                if status.trim() == "Charging" {
                    batch.push(("battery.charging".to_string(), 1.0, 60.0));
                }
            }
        }
        if !batch.is_empty() {
            let _ = tx.send(batch);
        }
        thread::sleep(std::time::Duration::from_secs(5));
    }
}

pub const SOLAR_FAST_GRID: u32 = 60;
pub const SOLAR_COARSE_GRID: u32 = 43200;
const SOLAR_GOES_SYNC_S: f64 = 149_597_870_700.0 / 299_792_458.0;
const SOLAR_L1_SUN_M: f64 = 1.481e11;
const SOLAR_RING_MAX: usize = 256;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum SolarChannel {
    Xray,
    Euv304,
    Euv284,
    BzGsm,
    Density,
    F107,
}

impl SolarChannel {
    pub fn name(self) -> &'static str {
        match self {
            SolarChannel::Xray => "xray",
            SolarChannel::Euv304 => "euv304",
            SolarChannel::Euv284 => "euv284",
            SolarChannel::BzGsm => "bz",
            SolarChannel::Density => "density",
            SolarChannel::F107 => "f107",
        }
    }

    pub fn idx(self) -> usize {
        match self {
            SolarChannel::Xray => 0,
            SolarChannel::Euv304 => 1,
            SolarChannel::Euv284 => 2,
            SolarChannel::BzGsm => 3,
            SolarChannel::Density => 4,
            SolarChannel::F107 => 5,
        }
    }
}

#[derive(Clone, Copy)]
pub struct SolarCell {
    pub grid: u32,
    pub channel: SolarChannel,
    pub bin: u64,
    pub value: f32,
}

fn solar_find_block<'a>(sources: &'a [SourceConfig], field_name: &str) -> Option<&'a SourceConfig> {
    sources.iter().find(|s| {
        s.extracts.iter().any(|e| match e {
            Extract::Field(fc)
            | Extract::First(fc, _)
            | Extract::Last(fc, _)
            | Extract::Path(fc) => fc.name == field_name,
            _ => false,
        })
    })
}

fn solar_series(
    block: &SourceConfig,
    body: &str,
    field_name: &str,
    lsk: &LeapSeconds,
) -> Vec<(f64, f64)> {
    let mut series_src = block.clone();
    series_src.extracts.retain(|e| {
        matches!(
            e,
            Extract::First(fc, _) | Extract::Last(fc, _) | Extract::Path(fc)
                if fc.name == field_name
        )
    });
    extract_series(&series_src, body, lsk)
}

fn solar_l1_sync(series: &[(f64, f64)], wind: &[(f64, f64)], tolerance_s: f64) -> Vec<(f64, f64)> {
    let mut out = Vec::new();
    for &(t, v) in series {
        let mut best: Option<(f64, f64)> = None;
        for &(tw, vw) in wind {
            let dt = (tw - t).abs();
            if dt <= tolerance_s && best.map_or(true, |(b, _)| dt < b) {
                best = Some((dt, vw));
            }
        }
        let Some((_, v_ms)) = best else {
            continue;
        };
        if v_ms <= 0.0 {
            continue;
        }
        out.push((t - SOLAR_L1_SUN_M / v_ms, v));
    }
    out.sort_by(|a, b| a.0.total_cmp(&b.0));
    out
}

fn solar_send_bins(
    series: &[(f64, f64)],
    grid: u32,
    channel: SolarChannel,
    last_sent: &mut std::collections::HashMap<(u32, SolarChannel), u64>,
    tx: &mpsc::Sender<SolarCell>,
) {
    let dt = grid as f64;
    let mut sorted: Vec<&(f64, f64)> = series
        .iter()
        .filter(|&&(t, _)| t.is_finite() && t > 0.0)
        .collect();
    sorted.sort_by(|a, b| a.0.total_cmp(&b.0));
    let gate = last_sent.get(&(grid, channel)).copied().unwrap_or(0);
    let mut cells: Vec<SolarCell> = Vec::new();
    let mut cur_bin: i64 = i64::MIN;
    let mut sum: f64 = 0.0;
    let mut cnt: u32 = 0;
    for &&(t, v) in &sorted {
        let bin = (t / dt).floor() as i64;
        if bin != cur_bin {
            if cur_bin != i64::MIN && cnt > 0 {
                let b = cur_bin as u64;
                if b > gate {
                    cells.push(SolarCell {
                        grid,
                        channel,
                        bin: b,
                        value: (sum / cnt as f64) as f32,
                    });
                }
            }
            cur_bin = bin;
            sum = 0.0;
            cnt = 0;
        }
        sum += v;
        cnt += 1;
    }
    if cur_bin != i64::MIN && cnt > 0 {
        let b = cur_bin as u64;
        if b > gate {
            cells.push(SolarCell {
                grid,
                channel,
                bin: b,
                value: (sum / cnt as f64) as f32,
            });
        }
    }
    if gate == 0 && cells.len() > SOLAR_RING_MAX {
        let drop = cells.len() - SOLAR_RING_MAX;
        cells.drain(..drop);
    }
    for cell in &cells {
        let _ = tx.send(*cell);
    }
    if let Some(last) = cells.last() {
        last_sent.insert((grid, channel), last.bin);
    }
}

fn solar_harvest(
    tx: mpsc::Sender<SolarCell>,
    sources: Vec<SourceConfig>,
    time: Arc<Mutex<Option<LeapSeconds>>>,
) {
    let mut last_sent: std::collections::HashMap<(u32, SolarChannel), u64> =
        std::collections::HashMap::new();
    loop {
        let lock = match time.lock() {
            Ok(l) => l,
            Err(_) => break,
        };
        let Some(lsk) = lock.as_ref() else {
            drop(lock);
            thread::sleep(std::time::Duration::from_secs(SOLAR_FAST_GRID as u64));
            continue;
        };
        let goes_shift = |series: Vec<(f64, f64)>| -> Vec<(f64, f64)> {
            series
                .into_iter()
                .map(|(t, v)| (t - SOLAR_GOES_SYNC_S, v))
                .collect()
        };
        if let Some(block) = solar_find_block(&sources, "noaa_goes_xray_flux_w_m2") {
            if let Some(body) = fetch_raw(&block.url, None, &block.headers, block.ttl) {
                let raw = solar_series(block, &body, "noaa_goes_xray_flux_w_m2", lsk);
                let shifted = goes_shift(raw);
                solar_send_bins(
                    &shifted,
                    SOLAR_FAST_GRID,
                    SolarChannel::Xray,
                    &mut last_sent,
                    &tx,
                );
                solar_send_bins(
                    &shifted,
                    SOLAR_COARSE_GRID,
                    SolarChannel::Xray,
                    &mut last_sent,
                    &tx,
                );
            }
        }
        if let Some(block) = solar_find_block(&sources, "solar_euv_flux_304_wm2") {
            if let Some(body) = fetch_raw(&block.url, None, &block.headers, block.ttl) {
                for (field, channel) in [
                    ("solar_euv_flux_304_wm2", SolarChannel::Euv304),
                    ("solar_euv_flux_284_wm2", SolarChannel::Euv284),
                ] {
                    let raw = solar_series(block, &body, field, lsk);
                    let shifted = goes_shift(raw);
                    solar_send_bins(&shifted, SOLAR_FAST_GRID, channel, &mut last_sent, &tx);
                    solar_send_bins(&shifted, SOLAR_COARSE_GRID, channel, &mut last_sent, &tx);
                }
            }
        }
        if let Some(block) = solar_find_block(&sources, "solar_f107_flux_sfu") {
            if let Some(body) = fetch_raw(&block.url, None, &block.headers, block.ttl) {
                let raw = solar_series(block, &body, "solar_f107_flux_sfu", lsk);
                let shifted = goes_shift(raw);
                solar_send_bins(
                    &shifted,
                    SOLAR_COARSE_GRID,
                    SolarChannel::F107,
                    &mut last_sent,
                    &tx,
                );
            }
        }
        let mut wind: Vec<(f64, f64)> = Vec::new();
        if let Some(block) = solar_find_block(&sources, "solar_wind_speed_km_s") {
            if let Some(body) = fetch_raw(&block.url, None, &block.headers, block.ttl) {
                wind = solar_series(block, &body, "solar_wind_speed_km_s", lsk);
                let dens = solar_series(block, &body, "solar_wind_density_cm3", lsk);
                let synced = solar_l1_sync(&dens, &wind, 60.0);
                solar_send_bins(
                    &synced,
                    SOLAR_FAST_GRID,
                    SolarChannel::Density,
                    &mut last_sent,
                    &tx,
                );
            }
        }
        if let Some(block) = solar_find_block(&sources, "magnetosphere_imf_bz_nt") {
            if let Some(body) = fetch_raw(&block.url, None, &block.headers, block.ttl) {
                let bz = solar_series(block, &body, "magnetosphere_imf_bz_nt", lsk);
                let synced = solar_l1_sync(&bz, &wind, 60.0);
                solar_send_bins(
                    &synced,
                    SOLAR_FAST_GRID,
                    SolarChannel::BzGsm,
                    &mut last_sent,
                    &tx,
                );
            }
        }
        drop(lock);
        thread::sleep(std::time::Duration::from_secs(SOLAR_FAST_GRID as u64));
    }
}

struct RefusalLedger {
    path: std::path::PathBuf,
    seen: std::collections::HashSet<String>,
}

impl RefusalLedger {
    fn new(path: &str) -> RefusalLedger {
        let mut seen = std::collections::HashSet::new();
        if let Ok(content) = std::fs::read_to_string(path) {
            for line in content.lines() {
                let mut parts = line.splitn(4, ' ');
                if parts.next() == Some("refused") {
                    let _unix = parts.next();
                    let class = parts.next().unwrap_or("");
                    let url = parts.next().unwrap_or("");
                    if !class.is_empty() && !url.is_empty() {
                        seen.insert(format!("{}|{}", class, url));
                    }
                }
            }
        }
        RefusalLedger {
            path: std::path::PathBuf::from(path),
            seen,
        }
    }

    fn register(&mut self, url: &str, class: &str) {
        let key = format!("{}|{}", class, url);
        if !self.seen.insert(key) {
            return;
        }
        let unix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        let line = format!("refused {} {} {}\n", unix, class, url);
        if let Ok(mut f) = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
        {
            use std::io::Write;
            let _ = f.write_all(line.as_bytes());
        }
    }
}

pub fn main_flow() {
    let env = Arc::new(load_env());
    {
        let args: Vec<String> = std::env::args().collect();
        if args.len() > 1 && args[1] == "--verify" {
            let dir = match args.get(2) {
                Some(s) => s.as_str(),
                None => {
                    eprintln!("--verify: directory argument absent");
                    std::process::exit(1);
                }
            };
            ANOMALY_COLLECT.with(|c| c.set(true));
            std::process::exit(ci_mode(dir));
        }
        if args.len() > 1 && args[1] == "--port" {
            let input = match args.get(2) {
                Some(s) => s.as_str(),
                None => {
                    eprintln!("--port: input file argument absent");
                    std::process::exit(1);
                }
            };
            let output = match args.get(3) {
                Some(s) => s.as_str(),
                None => {
                    eprintln!("--port: output file argument absent");
                    std::process::exit(1);
                }
            };
            std::process::exit(port_mode(input, output));
        }
        if args.len() > 1 && args[1] == "--learn-gate" {
            std::process::exit(gate_learn_mode());
        }
        if args.len() > 1 && args[1] == "--draft-context" {
            let path = match args.get(2) {
                Some(s) => s.as_str(),
                None => {
                    eprintln!("--draft-context: file argument absent");
                    std::process::exit(1);
                }
            };
            std::process::exit(draft_context_mode(path));
        }
        if args.len() > 1 && args[1] == "--draft" {
            let path = match args.get(2) {
                Some(s) => s.as_str(),
                None => {
                    eprintln!("--draft: file argument absent");
                    std::process::exit(1);
                }
            };
            let fetchone = args.iter().any(|a| a == "--fetchone");
            std::process::exit(draft_url_mode(path, &env, fetchone));
        }
        if args.len() > 1 && args[1] == "--urls" {
            let path = match args.get(2) {
                Some(s) => s.as_str(),
                None => {
                    eprintln!("--urls: file argument absent");
                    std::process::exit(1);
                }
            };
            let fetchone = args.iter().any(|a| a == "--fetchone");
            let jina = args.iter().any(|a| a == "--jina");
            std::process::exit(url_probe_mode(path, &env, fetchone, jina));
        }
        if args.len() > 1 && args[1] == "--probe" {
            let path = match args.get(2) {
                Some(s) => s.as_str(),
                None => {
                    eprintln!("--probe: file argument absent");
                    std::process::exit(1);
                }
            };
            let precise = args.iter().any(|a| a == "--precise");
            let fetchone = args.iter().any(|a| a == "--fetchone");
            let mut lat = 0.0;
            let mut lon = 0.0;
            let mut i = 2;
            while i < args.len() {
                if args[i] == "--lat" {
                    if let Some(v) = args.get(i + 1).and_then(|s| s.parse::<f64>().ok()) {
                        lat = v;
                        i += 1;
                    }
                } else if args[i] == "--lon" {
                    if let Some(v) = args.get(i + 1).and_then(|s| s.parse::<f64>().ok()) {
                        lon = v;
                        i += 1;
                    }
                }
                i += 1;
            }
            std::process::exit(probe_mode(path, precise, lat, lon, &env, fetchone));
        }
        if args.len() > 1 && args[1] == "--reverify" {
            std::process::exit(reverify_mode(&env));
        }
    }
    let declared_body: Option<DeclaredBody> = std::env::args().skip(1).find_map(|a| {
        let rest = a.strip_prefix("#body=")?;
        let parts: Vec<&str> = rest.split(',').collect();
        if parts.len() < 3 {
            return None;
        }
        let lat: f64 = parts[1].parse().ok()?;
        let lon: f64 = parts[2].parse().ok()?;
        let alt: Option<f64> = parts.get(3).and_then(|s| s.parse().ok());
        Some(DeclaredBody {
            body_name: parts[0].to_string(),
            lat,
            lon,
            alt,
        })
    });
    if declared_body.is_none() {
        eprintln!(
                "native body undeclared — station samples refused (declare via #body=<body>,<lat>,<lon>,<alt>)"
            );
    }
    let loaded = load_sources();
    let refusal_ledger = Arc::new(Mutex::new(RefusalLedger::new(
        "phi/pipeline/refusal_ledger.φ",
    )));
    let (sensor_tx, sensor_rx) = mpsc::channel::<Vec<(String, f64, f64)>>();
    let consent = Arc::new(AtomicBool::new(false));
    eprintln!("record consent: press Y (record) or N (silent) in the membrane window");
    let serial_tx = sensor_tx.clone();
    thread::spawn(move || serial_ingress(serial_tx));
    let battery_tx = sensor_tx.clone();
    thread::spawn(move || battery_ingress(battery_tx));
    #[cfg(feature = "browser_relay")]
    let port: u16 = match std::env::var("PORT").ok().and_then(|s| s.parse().ok()) {
        Some(p) => p,
        None => crate::relay::PORT_CONST,
    };
    let (fetch_tx, fetch_rx) = mpsc::channel::<FetchResult>();
    #[cfg(feature = "browser_relay")]
    let (sample_tx, sample_rx) = mpsc::channel::<Vec<Sample>>();
    #[cfg(not(feature = "browser_relay"))]
    let sample_rx = mpsc::channel::<Vec<Sample>>().1;
    let (presence_tx, presence_rx) =
        mpsc::channel::<(String, f64, f64, f64, f64, f64, f64, f64, f64, f64)>();
    let body_ephemerides = Arc::new(HashMap::new());
    #[cfg(feature = "browser_relay")]
    let index_html = match std::fs::read(resolve_asset("static/index.html")) {
        Ok(v) => v,
        Err(_) => {
            eprintln!("static/index.html absent — serving 0 bytes");
            Vec::new()
        }
    };
    #[cfg(feature = "browser_relay")]
    let constants_js = match std::fs::read(resolve_asset("static/constants.js")) {
        Ok(v) => v,
        Err(_) => {
            eprintln!("static/constants.js absent — browser protocol empty");
            Vec::new()
        }
    };
    let time: Arc<Mutex<Option<LeapSeconds>>> = Arc::new(Mutex::new(embedded_lsk()));
    let (solar_tx, solar_rx) = mpsc::channel::<SolarCell>();
    let solar_sources = loaded.clone();
    let solar_time = time.clone();
    thread::spawn(move || solar_harvest(solar_tx, solar_sources, solar_time));
    let mut archive = Archive {
        sources: loaded,
        body_ephemerides: body_ephemerides.clone(),
        field: Arc::new(build_buffer(
            Vec::new(),
            1.0,
            body_ephemerides.clone(),
            None,
            Vec::new(),
        )),
        presence: HashMap::new(),
        declared_body,
        origins: HashMap::new(),
        pck_bodies: HashMap::new(),
        time: time.clone(),
        asteroid_samples: Vec::new(),
        star_samples: Vec::new(),
        curves: None,
        spectral: Vec::new(),
        pending_channels: Vec::new(),
    };
    let body_names: Arc<Vec<String>> = {
        let mut names: Vec<String> = archive
            .sources
            .iter()
            .filter(|s| s.format != "kernel_text")
            .filter_map(|s| s.body.clone())
            .collect();
        names.sort();
        names.dedup();
        Arc::new(names)
    };
    let mut radiators: Vec<Box<dyn Radiator>> = Vec::new();
    #[cfg(feature = "browser_relay")]
    let presence_relay_tx = presence_tx.clone();
    let hidden = std::env::var("OMEGAFLOW_HIDDEN").is_ok();
    let (acoustic_tx, acoustic_rx) = mpsc::channel::<crate::mathematikerin::PresenceFrame>();
    let (seismic_tx, seismic_rx) = mpsc::channel::<crate::mathematikerin::PresenceFrame>();
    if !hidden {
        let mut kinetic: Vec<Box<dyn crate::mathematikerin::KineticRadiator>> = Vec::new();
        if let Ok(port) = std::env::var("OMEGAFLOW_SERIAL_OUT") {
            kinetic.push(Box::new(crate::mathematikerin::SeismicOscillator::new(
                &port,
            )));
        }
        thread::spawn(move || {
            while let Ok(frame) = seismic_rx.recv() {
                for s in kinetic.iter_mut() {
                    s.vibrate(&frame);
                }
            }
        });
    }
    let boot_pt = time
        .lock()
        .ok()
        .and_then(|l| l.as_ref().and_then(|l| l.system_now_tdb()));
    if let Some(pt) = boot_pt {
        let _ = presence_tx.send((
            "native".to_string(),
            pt,
            0.0,
            0.0,
            0.0,
            1280.0_f64 * crate::mathematikerin::GRID_INIT * 2.0,
            0.0,
            0.0,
            0.0,
            0.0,
        ));
    }
    let em_shutdown = if std::env::var("OMEGAFLOW_HEADLESS").is_ok() {
        std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false))
    } else {
        let em = crate::mathematikerin::EMOscillator::new(
            presence_tx,
            sensor_tx.clone(),
            body_names.clone(),
            time.clone(),
            consent.clone(),
            acoustic_tx,
            seismic_tx,
            solar_rx,
        );
        let em_shutdown = em.shutdown_flag();
        radiators.push(Box::new(em));
        em_shutdown
    };
    #[cfg(feature = "browser_relay")]
    {
        if !hidden {
            let sr = crate::relay::TcpRadiator::new(
                port,
                body_names.clone(),
                archive.field.clone(),
                index_html.clone(),
                constants_js.clone(),
                sample_tx.clone(),
                presence_relay_tx,
                time.clone(),
                consent.clone(),
            );
            radiators.push(Box::new(sr));
        }
    }
    if !hidden {
        let _acoustic = crate::mathematikerin::AcousticOscillator::new(acoustic_rx);
    }
    radiators.push(Box::new(StderrRadiator {
        last_line: String::new(),
        interactive: std::io::stderr().is_terminal(),
    }));
    let cadence = 1.0;
    let mut gm_text: Option<String> = None;
    let mut pck_text: Option<String> = None;
    let bootstrap_running: std::sync::Arc<std::sync::atomic::AtomicBool> =
        std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    spawn_ephemeris_bootstrap(&archive.sources, &bootstrap_running);
    let mut last_bootstrap: f64 = 0.0;
    let mut tick: u64 = 0;
    loop {
        tick += 1;
        if em_shutdown.load(Ordering::SeqCst) {
            eprintln!("the window closed — the ω-loop ends");
            break;
        }
        while let Ok((name, pt, px, py, pz, pr, vx, vy, vz, tt)) = presence_rx.try_recv() {
            archive
                .presence
                .insert(name, (pt, px, py, pz, pr, vx, vy, vz, tt));
        }
        let now = match archive.presence.get("native").map(|p| p.0) {
            Some(t) if t.is_finite() && t > 0.0 => t,
            _ => match system_now(&archive.time) {
                Some(t) => t,
                None => {
                    thread::sleep(std::time::Duration::from_secs_f64(cadence));
                    continue;
                }
            },
        };
        for i in 0..archive.sources.len() {
            if archive.sources[i].format != "kernel_text" {
                continue;
            }
            let Some(kernel_body) = archive.sources[i].body.clone() else {
                continue;
            };
            let url = archive.sources[i].url.clone();
            let ttl = archive.sources[i].ttl;
            let cache_path = format!("/tmp/omegaflow_kernel_{}.txt", kernel_body);
            if !cache_fresh(&cache_path, ttl) {
                let Some(text) = fetch_one(&url, None, &[], ttl, Some(now)) else {
                    continue;
                };
                if std::fs::write(&cache_path, text.as_bytes()).is_err() {
                    continue;
                }
            }
            let Ok(text) = std::fs::read_to_string(&cache_path) else {
                continue;
            };
            match kernel_body.as_str() {
                "gm_de440" => gm_text = Some(text),
                "pck00010" | "geophysical" => {
                    pck_text = Some(match pck_text {
                        Some(prev) => prev + "\n" + &text,
                        None => text,
                    });
                }
                "naif0012" => {
                    if let Some(l) = crate::lsk::parse(&text) {
                        if let Ok(mut guard) = archive.time.lock() {
                            *guard = Some(l);
                        }
                    }
                }
                _ => {}
            }
            archive.pck_bodies = crate::pck::parse(gm_text.as_deref(), pck_text.as_deref());
        }
        let lsk = match leap_seconds(&archive.time) {
            Some(l) => l,
            None => {
                eprintln!(
                    "the time base is absent — the process refuses to fabricate a dead field"
                );
                thread::sleep(std::time::Duration::from_secs_f64(cadence));
                continue;
            }
        };
        let wall_entered = match lsk.system_now_tdb() {
            Some(t) => t,
            None => {
                thread::sleep(std::time::Duration::from_secs_f64(cadence));
                continue;
            }
        };
        if tick & 63 == 0 {
            let mut missing_ttl: Option<f64> = None;
            for s in &archive.sources {
                if s.format != "ephemeris_binary" {
                    continue;
                }
                let Some(body) = &s.body else {
                    continue;
                };
                let tmp_path = format!("/tmp/omegaflow_eph_{}.bin", body);
                if cache_fresh(&tmp_path, s.ttl)
                    || archive
                        .body_ephemerides
                        .get(body)
                        .and_then(|e| e.props.as_ref())
                        .is_some()
                {
                    continue;
                }
                missing_ttl = Some(s.ttl as f64);
                break;
            }
            if let Some(ttl) = missing_ttl {
                if now - last_bootstrap >= ttl / (Φ * Φ) {
                    last_bootstrap = now;
                    spawn_ephemeris_bootstrap(&archive.sources, &bootstrap_running);
                }
            }
        }
        let mut fetched_samples: Vec<Sample> = Vec::new();
        {
            let mut still_pending: Vec<(Channel, FieldConfig, u32)> = Vec::new();
            for (channel, sensor, idx) in archive.pending_channels.drain(..) {
                let src = &archive.sources[idx as usize];
                match anchor(
                    &channel,
                    &sensor,
                    src.ttl as f64,
                    Some(idx),
                    Some(&src.frame),
                    None,
                    &archive.body_ephemerides,
                ) {
                    Some(sample) => fetched_samples.push(sample),
                    None => still_pending.push((channel, sensor, idx)),
                }
            }
            if !still_pending.is_empty() {
                eprintln!(
                    "{} channels waiting for body ephemerides — anchored on arrival",
                    still_pending.len()
                );
            }
            archive.pending_channels = still_pending;
        }
        let mut dropped_channels: Vec<(Channel, FieldConfig, u32)> = Vec::new();
        while let Ok(res) = fetch_rx.try_recv() {
            let st = archive
                .origins
                .entry(res.source_idx as u32)
                .or_insert(OriginState {
                    fetched: now,
                    prev_epoch: now,
                    prev_abs: [0.0, 0.0, 0.0],
                    prev_motion: None,
                    resid_ema: 0.0,
                    has_prev: false,
                    failures: 0,
                    in_flight: false,
                });
            settle_fetch(st, res.fetch_ok, now);
            if let Some((name, eph)) = res.eph_update {
                let mut eph_map = (*archive.body_ephemerides).clone();
                eph_map.insert(name, eph);
                archive.body_ephemerides = Arc::new(eph_map);
            }
            if !res.asteroid_samples.is_empty() {
                archive.asteroid_samples = res.asteroid_samples;
            }
            if !res.star_samples.is_empty() {
                archive.star_samples = res.star_samples;
            }
            if let Some(curves) = res.curves {
                archive.curves = Some(curves);
            }
            if let Some(hash) = res.spectral {
                archive.spectral.retain(|h| h.name != hash.name);
                archive.spectral.push(hash);
            }
            let src = &archive.sources[res.source_idx];
            for (channel, sensor) in &res.channels {
                let track_origin = matches!(channel.position, Position::Source)
                    || matches!(channel.position, Position::StateVector { track: true, .. });
                if let Some(sample) = anchor(
                    channel,
                    sensor,
                    src.ttl as f64,
                    Some(res.source_idx as u32),
                    Some(&src.frame),
                    if track_origin {
                        archive.origins.get_mut(&(res.source_idx as u32))
                    } else {
                        None
                    },
                    &archive.body_ephemerides,
                ) {
                    fetched_samples.push(sample);
                } else {
                    let eph_missing = match &channel.position {
                        Position::Surface { body_name, .. }
                        | Position::SurfaceFlow { body_name, .. } => archive
                            .body_ephemerides
                            .get(body_name.as_str())
                            .and_then(|e| e.props.as_ref())
                            .is_none(),
                        Position::Source => match &src.frame {
                            Frame::Surface { body_name, .. }
                            | Frame::Barycenter { body_name, .. } => archive
                                .body_ephemerides
                                .get(body_name.as_str())
                                .and_then(|e| e.props.as_ref())
                                .is_none(),
                            Frame::Manifest => false,
                        },
                        Position::StateVector { .. } => false,
                    };
                    if eph_missing {
                        dropped_channels.push((
                            channel.clone(),
                            sensor.clone(),
                            res.source_idx as u32,
                        ));
                    }
                }
            }
        }
        archive.pending_channels.extend(dropped_channels);
        while let Ok(samples) = sample_rx.try_recv() {
            fetched_samples.extend(samples);
        }
        while let Ok(samples) = sensor_rx.try_recv() {
            if !consent.load(Ordering::SeqCst) {
                continue;
            }
            let Some(declared_body) = archive.declared_body.clone() else {
                continue;
            };
            for (name, value, tau) in samples {
                let Some(bs) = sensor_config(&name) else {
                    continue;
                };
                let effective_tau = if tau > 0.0 {
                    tau
                } else {
                    continue;
                };
                if !value.is_finite() {
                    continue;
                }
                let fc = FieldConfig {
                    key: bs.key.clone(),
                    name: bs.key.clone(),
                    kernel: bs.kernel,
                    force: bs.force,
                    tau: effective_tau,
                    absorption: 0.0,
                    advection: 0.0,
                    unit: String::new(),
                    fold: None,
                };
                let channel = Channel {
                    z: 0.0,
                    freq: 0.0,
                    bin_width: 0.0,
                    epoch: now,
                    position: Position::Surface {
                        body_name: declared_body.body_name.clone(),
                        lat: declared_body.lat,
                        lon: declared_body.lon,
                        alt: match declared_body.alt {
                            Some(a) => a,
                            None => {
                                eprintln!(
                                        "sensor alt undeclared — samples refused (declare #body=<body>,<lat>,<lon>,<alt>)"
                                    );
                                continue;
                            }
                        },
                    },
                    name: fc.name.clone(),
                    value,
                };
                if let Some(sample) = anchor(
                    &channel,
                    &fc,
                    bs.ttl,
                    None,
                    None,
                    None,
                    &archive.body_ephemerides,
                ) {
                    fetched_samples.push(sample);
                }
            }
        }
        for i in 0..archive.sources.len() {
            let origin = i as u32;
            if !origin_stale(&archive.origins, origin, archive.sources[i].ttl, now) {
                continue;
            }
            if archive.sources[i].format == "kernel_text" {
                continue;
            }
            if archive.origins.values().filter(|o| o.in_flight).count() >= FETCH_BUDGET {
                break;
            }
            if archive.sources[i].format == "ephemeris_binary" {
                let src_idx = i;
                let src_clone = archive.sources[i].clone();
                let tmp_path = match &src_clone.body {
                    Some(b) => format!("/tmp/omegaflow_eph_{}.bin", b),
                    None => continue,
                };
                if cache_fresh(&tmp_path, src_clone.ttl) {
                    begin_fetch(&mut archive.origins, i as u32, now);
                    let ftx = fetch_tx.clone();
                    let lsk_c = lsk.clone();
                    let now_c = now;
                    let tmp_path_c = tmp_path.clone();
                    thread::spawn(move || {
                        if let ExtractResult::WithEphemeris(_, eph) =
                            extract(&src_clone, &tmp_path_c, now_c, &lsk_c)
                        {
                            let _ = ftx.send(FetchResult {
                                source_idx: src_idx,
                                channels: Vec::new(),
                                eph_update: src_clone.body.clone().map(|b| (b, eph)),
                                asteroid_samples: Vec::new(),
                                star_samples: Vec::new(),
                                curves: None,
                                spectral: None,
                                fetch_ok: true,
                            });
                        } else {
                            eprintln!(
                                    "ephemeris {}: extract void — cache dropped, refetch next bootstrap",
                                    src_clone.body.as_deref().unwrap_or("?")
                                );
                            let _ = std::fs::remove_file(&tmp_path_c);
                            let _ = ftx.send(FetchResult {
                                source_idx: src_idx,
                                channels: Vec::new(),
                                eph_update: None,
                                asteroid_samples: Vec::new(),
                                star_samples: Vec::new(),
                                curves: None,
                                spectral: None,
                                fetch_ok: true,
                            });
                        }
                    });
                }
                continue;
            }
            if archive.sources[i].format == "catalog_dastcom" {
                let url = archive.sources[i].url.clone();
                begin_fetch(&mut archive.origins, i as u32, now);
                let ftx = fetch_tx.clone();
                let src_clone = archive.sources[i].clone();
                let src_idx = i;
                let src_ttl = src_clone.ttl;
                thread::spawn(move || {
                    let name = url.rsplit('/').next().unwrap_or("catalog").to_string();
                    let tmp_path = format!("/tmp/omegaflow_catalog_{}", name);
                    let mut fetched = true;
                    if !cache_fresh(&tmp_path, src_ttl) {
                        fetched = match fetch_raw_bytes(&url, src_ttl) {
                            Some(bytes) => std::fs::write(&tmp_path, &bytes).is_ok(),
                            None => false,
                        };
                    }
                    if !fetched {
                        eprintln!(
                            "catalog {}: fetch void — the catalog stays absent, retry in ttl/Φ·2ⁿ",
                            url
                        );
                        let _ = ftx.send(FetchResult {
                            source_idx: src_idx,
                            channels: Vec::new(),
                            eph_update: None,
                            asteroid_samples: Vec::new(),
                            star_samples: Vec::new(),
                            curves: None,
                            spectral: None,
                            fetch_ok: false,
                        });
                        return;
                    }
                    let bytes = match std::fs::read(&tmp_path) {
                        Ok(b) => b,
                        Err(_) => {
                            eprintln!("catalog {}: read void — retry in ttl/Φ", url);
                            let _ = ftx.send(FetchResult {
                                source_idx: src_idx,
                                channels: Vec::new(),
                                eph_update: None,
                                asteroid_samples: Vec::new(),
                                star_samples: Vec::new(),
                                curves: None,
                                spectral: None,
                                fetch_ok: true,
                            });
                            return;
                        }
                    };
                    let samples = build_asteroid_samples(&bytes, src_ttl);
                    eprintln!("\r\x1b[Kcatalog_dastcom: {} samples", samples.len());
                    let _ = ftx.send(FetchResult {
                        source_idx: src_idx,
                        channels: Vec::new(),
                        eph_update: None,
                        asteroid_samples: samples,
                        star_samples: Vec::new(),
                        curves: None,
                        spectral: None,
                        fetch_ok: true,
                    });
                });
                continue;
            }
            if archive.sources[i].format == "netcdf" {
                let src_clone = archive.sources[i].clone();
                let pos = match &src_clone.frame {
                    Frame::Surface {
                        lat,
                        lon,
                        alt,
                        body_name,
                    } => body_fixed_to_icrs(
                        body_name,
                        *lat,
                        *lon,
                        *alt,
                        now,
                        &archive.body_ephemerides,
                    )
                    .map(|p| (p[0], p[1], p[2])),
                    Frame::Barycenter { body_name, scale } => {
                        body_barycenter_position(body_name, now, &archive.body_ephemerides)
                            .map(|p| (p[0] * scale, p[1] * scale, p[2] * scale))
                    }
                    Frame::Manifest => None,
                };
                let url = match pos {
                    Some((x, y, z)) => render_source_url(
                        &src_clone,
                        x,
                        y,
                        z,
                        now,
                        0.0,
                        &archive.body_ephemerides,
                        &env,
                        &lsk,
                    ),
                    None => render_source_url(
                        &src_clone,
                        0.0,
                        0.0,
                        0.0,
                        now,
                        0.0,
                        &archive.body_ephemerides,
                        &env,
                        &lsk,
                    ),
                };
                let Some(url) = url else {
                    continue;
                };
                begin_fetch(&mut archive.origins, i as u32, now);
                let ftx = fetch_tx.clone();
                let src_idx = i;
                let src_ttl = src_clone.ttl;
                let lsk_c = lsk.clone();
                thread::spawn(move || {
                    let name = url.rsplit('/').next().unwrap_or("netcdf").to_string();
                    let tmp_path = format!("/tmp/omegaflow_netcdf_{}", name);
                    if !cache_fresh(&tmp_path, src_ttl) {
                        let bytes = match fetch_raw_bytes(&url, src_ttl) {
                            Some(b) => b,
                            None => {
                                eprintln!("netcdf {}: fetch void — retry in ttl/Φ·2ⁿ", url);
                                let _ = ftx.send(FetchResult {
                                    source_idx: src_idx,
                                    channels: Vec::new(),
                                    eph_update: None,
                                    asteroid_samples: Vec::new(),
                                    star_samples: Vec::new(),
                                    curves: None,
                                    spectral: None,
                                    fetch_ok: false,
                                });
                                return;
                            }
                        };
                        if std::fs::write(&tmp_path, &bytes).is_err() {
                            eprintln!("netcdf {}: write void — retry in ttl/Φ", url);
                            let _ = ftx.send(FetchResult {
                                source_idx: src_idx,
                                channels: Vec::new(),
                                eph_update: None,
                                asteroid_samples: Vec::new(),
                                star_samples: Vec::new(),
                                curves: None,
                                spectral: None,
                                fetch_ok: true,
                            });
                            return;
                        }
                    }
                    let bytes = match std::fs::read(&tmp_path) {
                        Ok(b) => b,
                        Err(_) => {
                            eprintln!("netcdf {}: read void — retry in ttl/Φ", url);
                            let _ = ftx.send(FetchResult {
                                source_idx: src_idx,
                                channels: Vec::new(),
                                eph_update: None,
                                asteroid_samples: Vec::new(),
                                star_samples: Vec::new(),
                                curves: None,
                                spectral: None,
                                fetch_ok: true,
                            });
                            return;
                        }
                    };
                    let channels = build_netcdf_channels(&src_clone, &bytes, &lsk_c);
                    eprintln!("\r\x1b[Knetcdf {}: {} samples", name, channels.len());
                    let _ = ftx.send(FetchResult {
                        source_idx: src_idx,
                        channels,
                        eph_update: None,
                        asteroid_samples: Vec::new(),
                        star_samples: Vec::new(),
                        curves: None,
                        spectral: None,
                        fetch_ok: true,
                    });
                });
                continue;
            }
            if archive.sources[i].format == "finals" || archive.sources[i].format == "ionex" {
                let url = archive.sources[i].url.clone();
                begin_fetch(&mut archive.origins, i as u32, now);
                let ftx = fetch_tx.clone();
                let src_clone = archive.sources[i].clone();
                let src_idx = i;
                let src_ttl = src_clone.ttl;
                let lsk_c = lsk.clone();
                let now_c = now;
                let is_ionex = archive.sources[i].format == "ionex";
                thread::spawn(move || {
                    let bytes = match fetch_raw_bytes(&url, src_ttl) {
                        Some(b) => b,
                        None => {
                            eprintln!("finals {}: fetch void — retry in ttl/Φ·2ⁿ", url);
                            let _ = ftx.send(FetchResult {
                                source_idx: src_idx,
                                channels: Vec::new(),
                                eph_update: None,
                                asteroid_samples: Vec::new(),
                                star_samples: Vec::new(),
                                curves: None,
                                spectral: None,
                                fetch_ok: false,
                            });
                            return;
                        }
                    };
                    let text = String::from_utf8_lossy(&bytes).into_owned();
                    let channels = if is_ionex {
                        build_ionex_channels(&src_clone, &text, now_c, &lsk_c)
                    } else {
                        build_finals_channels(&src_clone, &text, &lsk_c)
                    };
                    let _ = ftx.send(FetchResult {
                        source_idx: src_idx,
                        channels,
                        eph_update: None,
                        asteroid_samples: Vec::new(),
                        star_samples: Vec::new(),
                        curves: None,
                        spectral: None,
                        fetch_ok: true,
                    });
                });
                continue;
            }
            if archive.sources[i].format == "alerce" {
                begin_fetch(&mut archive.origins, i as u32, now);
                let ftx = fetch_tx.clone();
                let src_clone = archive.sources[i].clone();
                let src_idx = i;
                let cap = src_clone.fanout_cap.max(1) as usize;
                let delay = src_clone.fanout_delay;
                thread::spawn(move || {
                    let channels = build_alerce_channels(&src_clone, cap, delay);
                    let _ = ftx.send(FetchResult {
                        source_idx: src_idx,
                        channels,
                        eph_update: None,
                        asteroid_samples: Vec::new(),
                        star_samples: Vec::new(),
                        curves: None,
                        spectral: None,
                        fetch_ok: true,
                    });
                });
                continue;
            }
            if archive.sources[i].format == "catalog_tycho" {
                let url = archive.sources[i].url.clone();
                begin_fetch(&mut archive.origins, i as u32, now);
                let ftx = fetch_tx.clone();
                let src_clone = archive.sources[i].clone();
                let src_idx = i;
                let src_ttl = src_clone.ttl;
                thread::spawn(move || {
                    let name = url.rsplit('/').next().unwrap_or("stars").to_string();
                    let tmp_path = format!("/tmp/omegaflow_catalog_{}", name);
                    if !cache_fresh(&tmp_path, src_ttl) {
                        let bytes = match fetch_raw_bytes(&url, src_ttl) {
                            Some(b) => b,
                            None => {
                                eprintln!("catalog_tycho {}: fetch void — retry in ttl/Φ·2ⁿ", url);
                                let _ = ftx.send(FetchResult {
                                    source_idx: src_idx,
                                    channels: Vec::new(),
                                    eph_update: None,
                                    asteroid_samples: Vec::new(),
                                    star_samples: Vec::new(),
                                    curves: None,
                                    spectral: None,
                                    fetch_ok: false,
                                });
                                return;
                            }
                        };
                        if std::fs::write(&tmp_path, &bytes).is_err() {
                            eprintln!("catalog_tycho {}: write void — retry in ttl/Φ", url);
                            let _ = ftx.send(FetchResult {
                                source_idx: src_idx,
                                channels: Vec::new(),
                                eph_update: None,
                                asteroid_samples: Vec::new(),
                                star_samples: Vec::new(),
                                curves: None,
                                spectral: None,
                                fetch_ok: true,
                            });
                            return;
                        }
                    }
                    let bytes = match std::fs::read(&tmp_path) {
                        Ok(b) => b,
                        Err(_) => {
                            eprintln!("catalog_tycho {}: read void — retry in ttl/Φ", url);
                            let _ = ftx.send(FetchResult {
                                source_idx: src_idx,
                                channels: Vec::new(),
                                eph_update: None,
                                asteroid_samples: Vec::new(),
                                star_samples: Vec::new(),
                                curves: None,
                                spectral: None,
                                fetch_ok: true,
                            });
                            return;
                        }
                    };
                    let star_samples = build_star_samples(&bytes);
                    eprintln!("\r\x1b[Kcatalog_tycho: {} stars", star_samples.len());
                    let _ = ftx.send(FetchResult {
                        source_idx: src_idx,
                        channels: Vec::new(),
                        eph_update: None,
                        asteroid_samples: Vec::new(),
                        star_samples,
                        curves: None,
                        spectral: None,
                        fetch_ok: true,
                    });
                });
                continue;
            }
            if archive.sources[i].format == "spectral" {
                let src = archive.sources[i].clone();
                begin_fetch(&mut archive.origins, i as u32, now);
                let ftx = fetch_tx.clone();
                let src_idx = i;
                let src_ttl = src.ttl;
                thread::spawn(move || {
                    let empty = |fetch_ok: bool| FetchResult {
                        source_idx: src_idx,
                        channels: Vec::new(),
                        eph_update: None,
                        asteroid_samples: Vec::new(),
                        star_samples: Vec::new(),
                        curves: None,
                        spectral: None,
                        fetch_ok,
                    };
                    let url = src.url.clone();
                    let name = url.rsplit('/').next().unwrap_or("spectra").to_string();
                    let tmp_path = format!("/tmp/omegaflow_spectral_{}", name);
                    if !cache_fresh(&tmp_path, src_ttl) {
                        let bytes = match fetch_raw_bytes(&url, src_ttl) {
                            Some(b) => b,
                            None => {
                                eprintln!("spectral {}: fetch void — retry in ttl/Φ·2ⁿ", url);
                                let _ = ftx.send(empty(false));
                                return;
                            }
                        };
                        if std::fs::write(&tmp_path, &bytes).is_err() {
                            eprintln!("spectral {}: write void — retry in ttl/Φ", url);
                            let _ = ftx.send(empty(true));
                            return;
                        }
                    }
                    let bytes = match std::fs::read(&tmp_path) {
                        Ok(b) => b,
                        Err(_) => {
                            eprintln!("spectral {}: read void — retry in ttl/Φ", url);
                            let _ = ftx.send(empty(true));
                            return;
                        }
                    };
                    let (epoch, bins) = match crate::spectral::parse_spectral_bin(&bytes) {
                        Some(x) => x,
                        None => {
                            eprintln!(
                                "spectral {}: bin reads void — {} B carry no spectra.bin contract",
                                url,
                                bytes.len()
                            );
                            let _ = ftx.send(empty(true));
                            return;
                        }
                    };
                    let motion = match &src.frame {
                        Frame::Surface {
                            body_name,
                            lat,
                            lon,
                            alt,
                        } => Motion::Surface {
                            body_name: body_name.clone(),
                            lat: *lat,
                            lon: *lon,
                            alt: *alt,
                        },
                        Frame::Barycenter { body_name, scale } => Motion::Barycenter {
                            body_name: body_name.clone(),
                            scale: *scale,
                        },
                        Frame::Manifest => {
                            eprintln!(
                                "spectral {}: frameless — the block declares no position",
                                url
                            );
                            let _ = ftx.send(empty(true));
                            return;
                        }
                    };
                    let field = match src.extracts.first() {
                        Some(Extract::Field(fc)) => fc.clone(),
                        _ => {
                            eprintln!(
                                "spectral {}: field undeclared — the block carries no field line",
                                url
                            );
                            let _ = ftx.send(empty(true));
                            return;
                        }
                    };
                    let hash = SpectralHash {
                        name: field.name.clone(),
                        motion,
                        epoch,
                        ttl: src_ttl as f64,
                        tau: field.tau,
                        kernel_id: field.kernel as f64,
                        force_type: field.force as f64,
                        absorption: field.absorption,
                        advection: field.advection,
                        bins,
                    };
                    eprintln!(
                        "\r\x1b[Kspectral {}: {} bins, epoch_tdb {}",
                        field.name,
                        hash.bins.len(),
                        hash.epoch
                    );
                    let _ = ftx.send(FetchResult {
                        source_idx: src_idx,
                        channels: Vec::new(),
                        eph_update: None,
                        asteroid_samples: Vec::new(),
                        star_samples: Vec::new(),
                        curves: None,
                        spectral: Some(hash),
                        fetch_ok: true,
                    });
                });
                continue;
            }
            if archive.sources[i].format == "lightcurve" {
                let url = archive.sources[i].url.clone();
                begin_fetch(&mut archive.origins, i as u32, now);
                let ftx = fetch_tx.clone();
                let src_idx = i;
                let src_ttl = archive.sources[i].ttl;
                thread::spawn(move || {
                    let name = url.rsplit('/').next().unwrap_or("curves").to_string();
                    let tmp_path = format!("/tmp/omegaflow_catalog_{}", name);
                    if !cache_fresh(&tmp_path, src_ttl) {
                        let bytes = match fetch_raw_bytes(&url, src_ttl) {
                            Some(b) => b,
                            None => {
                                eprintln!("lightcurve {}: fetch void — retry in ttl/Φ·2ⁿ", url);
                                let _ = ftx.send(FetchResult {
                                    source_idx: src_idx,
                                    channels: Vec::new(),
                                    eph_update: None,
                                    asteroid_samples: Vec::new(),
                                    star_samples: Vec::new(),
                                    curves: None,
                                    spectral: None,
                                    fetch_ok: false,
                                });
                                return;
                            }
                        };
                        if std::fs::write(&tmp_path, &bytes).is_err() {
                            eprintln!("lightcurve {}: write void — retry in ttl/Φ", url);
                            let _ = ftx.send(FetchResult {
                                source_idx: src_idx,
                                channels: Vec::new(),
                                eph_update: None,
                                asteroid_samples: Vec::new(),
                                star_samples: Vec::new(),
                                curves: None,
                                spectral: None,
                                fetch_ok: true,
                            });
                            return;
                        }
                    }
                    let bytes = match std::fs::read(&tmp_path) {
                        Ok(b) => b,
                        Err(_) => {
                            eprintln!("lightcurve {}: read void — retry in ttl/Φ", url);
                            let _ = ftx.send(FetchResult {
                                source_idx: src_idx,
                                channels: Vec::new(),
                                eph_update: None,
                                asteroid_samples: Vec::new(),
                                star_samples: Vec::new(),
                                curves: None,
                                spectral: None,
                                fetch_ok: true,
                            });
                            return;
                        }
                    };
                    let curves = build_curve_set(&bytes);
                    eprintln!("\r\x1b[Klightcurve: {} stars", curves.stars.len());
                    let _ = ftx.send(FetchResult {
                        source_idx: src_idx,
                        channels: Vec::new(),
                        eph_update: None,
                        asteroid_samples: Vec::new(),
                        star_samples: Vec::new(),
                        curves: Some(Arc::new(curves)),
                        spectral: None,
                        fetch_ok: true,
                    });
                });
                continue;
            }
            if matches!(
                archive.sources[i].format.as_str(),
                "rpw_efield" | "goes_xrs" | "omni2_serie"
            ) {
                let url = archive.sources[i].url.clone();
                let src = archive.sources[i].clone();
                let fmt = archive.sources[i].format.clone();
                begin_fetch(&mut archive.origins, i as u32, now);
                let ftx = fetch_tx.clone();
                let src_idx = i;
                let src_ttl = src.ttl;
                thread::spawn(move || {
                    let empty = |fetch_ok: bool| FetchResult {
                        source_idx: src_idx,
                        channels: Vec::new(),
                        eph_update: None,
                        asteroid_samples: Vec::new(),
                        star_samples: Vec::new(),
                        curves: None,
                        spectral: None,
                        fetch_ok,
                    };
                    let name = url.rsplit('/').next().unwrap_or("series").to_string();
                    let tmp_path = format!("/tmp/omegaflow_series_{}", name);
                    if !cache_fresh(&tmp_path, src_ttl) {
                        let bytes = match fetch_raw_bytes(&url, src_ttl) {
                            Some(b) => b,
                            None => {
                                eprintln!("{} {}: fetch void — retry in ttl/Φ·2ⁿ", fmt, url);
                                let _ = ftx.send(empty(false));
                                return;
                            }
                        };
                        if std::fs::write(&tmp_path, &bytes).is_err() {
                            eprintln!("{} {}: write void — retry in ttl/Φ", fmt, url);
                            let _ = ftx.send(empty(true));
                            return;
                        }
                    }
                    let bytes = match std::fs::read(&tmp_path) {
                        Ok(b) => b,
                        Err(_) => {
                            eprintln!("{} {}: read void — retry in ttl/Φ", fmt, url);
                            let _ = ftx.send(empty(true));
                            return;
                        }
                    };
                    let records = match series_parse_bin(&fmt, &bytes) {
                        Some(r) => r,
                        None => {
                            eprintln!(
                                "{} {}: bin reads void — {} B carry no {} contract",
                                fmt,
                                url,
                                bytes.len(),
                                fmt
                            );
                            let _ = ftx.send(empty(true));
                            return;
                        }
                    };
                    let fields: Vec<FieldConfig> = src
                        .extracts
                        .iter()
                        .filter_map(|e| match e {
                            Extract::Field(fc) => Some(fc.clone()),
                            _ => None,
                        })
                        .collect();
                    if fields.is_empty() {
                        eprintln!(
                            "{} {}: field undeclared — the block carries no field line",
                            fmt, url
                        );
                        let _ = ftx.send(empty(true));
                        return;
                    }
                    let mut channels = Vec::with_capacity(records.len());
                    for (t, val, comp) in records {
                        let Some(name) = series_component_name(&fmt, comp) else {
                            continue;
                        };
                        let Some(fc) = fields.iter().find(|fc| fc.name == name) else {
                            continue;
                        };
                        channels.push((
                            Channel {
                                z: 0.0,
                                freq: 0.0,
                                bin_width: 0.0,
                                epoch: t,
                                position: Position::Source,
                                name: fc.name.clone(),
                                value: val,
                            },
                            fc.clone(),
                        ));
                    }
                    eprintln!("\r\x1b[K{} {}: {} oscillators", fmt, url, channels.len());
                    let _ = ftx.send(FetchResult {
                        source_idx: src_idx,
                        channels,
                        eph_update: None,
                        asteroid_samples: Vec::new(),
                        star_samples: Vec::new(),
                        curves: None,
                        spectral: None,
                        fetch_ok: true,
                    });
                });
                continue;
            }
            if archive.sources[i].format == "gong_modes" {
                let url = archive.sources[i].url.clone();
                let src = archive.sources[i].clone();
                begin_fetch(&mut archive.origins, i as u32, now);
                let ftx = fetch_tx.clone();
                let src_idx = i;
                let src_ttl = src.ttl;
                thread::spawn(move || {
                    let empty = |fetch_ok: bool| FetchResult {
                        source_idx: src_idx,
                        channels: Vec::new(),
                        eph_update: None,
                        asteroid_samples: Vec::new(),
                        star_samples: Vec::new(),
                        curves: None,
                        spectral: None,
                        fetch_ok,
                    };
                    let name = url.rsplit('/').next().unwrap_or("gong").to_string();
                    let tmp_path = format!("/tmp/omegaflow_gong_{}", name);
                    if !cache_fresh(&tmp_path, src_ttl) {
                        let bytes = match fetch_raw_bytes(&url, src_ttl) {
                            Some(b) => b,
                            None => {
                                eprintln!("gong {}: fetch void — retry in ttl/Φ·2ⁿ", url);
                                let _ = ftx.send(empty(false));
                                return;
                            }
                        };
                        if std::fs::write(&tmp_path, &bytes).is_err() {
                            eprintln!("gong {}: write void — retry in ttl/Φ", url);
                            let _ = ftx.send(empty(true));
                            return;
                        }
                    }
                    let bytes = match std::fs::read(&tmp_path) {
                        Ok(b) => b,
                        Err(_) => {
                            eprintln!("gong {}: read void — retry in ttl/Φ", url);
                            let _ = ftx.send(empty(true));
                            return;
                        }
                    };
                    let modes = match crate::gong::parse_bin(&bytes) {
                        Some(m) => m,
                        None => {
                            eprintln!(
                                "gong {}: bin reads void — {} B carry no gong_modes.bin contract",
                                url,
                                bytes.len()
                            );
                            let _ = ftx.send(empty(true));
                            return;
                        }
                    };
                    let fields: Vec<FieldConfig> = src
                        .extracts
                        .iter()
                        .filter_map(|e| match e {
                            Extract::Field(fc) => Some(fc.clone()),
                            _ => None,
                        })
                        .collect();
                    let Some(fc) = fields.first().cloned() else {
                        eprintln!(
                            "gong {}: field undeclared — the block carries no field line",
                            url
                        );
                        let _ = ftx.send(empty(true));
                        return;
                    };
                    let mut channels = Vec::with_capacity(modes.len());
                    for (_, _, t, rms) in modes {
                        channels.push((
                            Channel {
                                z: 0.0,
                                freq: 0.0,
                                bin_width: 0.0,
                                epoch: t,
                                position: Position::Source,
                                name: fc.name.clone(),
                                value: rms,
                            },
                            fc.clone(),
                        ));
                    }
                    eprintln!("\r\x1b[Kgong {}: {} modes", url, channels.len());
                    let _ = ftx.send(FetchResult {
                        source_idx: src_idx,
                        channels,
                        eph_update: None,
                        asteroid_samples: Vec::new(),
                        star_samples: Vec::new(),
                        curves: None,
                        spectral: None,
                        fetch_ok: true,
                    });
                });
                continue;
            }
            if archive.sources[i].format == "csv_zip" {
                begin_fetch(&mut archive.origins, i as u32, now);
                let ftx = fetch_tx.clone();
                let src_clone = archive.sources[i].clone();
                let src_idx = i;
                let eph_arc = archive.body_ephemerides.clone();
                let e = env.clone();
                let lsk_c = lsk.clone();
                thread::spawn(move || {
                    let url = match render_source_url(
                        &src_clone, 0.0, 0.0, 0.0, now, 0.0, &eph_arc, &e, &lsk_c,
                    ) {
                        Some(u) => u,
                        None => {
                            eprintln!("csv_zip {}: url render void — retry in ttl/Φ", src_idx);
                            let _ = ftx.send(FetchResult {
                                source_idx: src_idx,
                                channels: Vec::new(),
                                eph_update: None,
                                asteroid_samples: Vec::new(),
                                star_samples: Vec::new(),
                                curves: None,
                                spectral: None,
                                fetch_ok: true,
                            });
                            return;
                        }
                    };
                    let tmp_path = format!("/tmp/omegaflow_csv_{}.zip", src_idx);
                    if !cache_fresh(&tmp_path, src_clone.ttl) {
                        let headers = render_headers(&src_clone.headers, &e);
                        let bytes = match fetch_raw_bytes_post(&url, None, &headers, src_clone.ttl)
                        {
                            Some(b) => b,
                            None => {
                                eprintln!("csv_zip {}: fetch void — retry in ttl/Φ·2ⁿ", src_idx);
                                let _ = ftx.send(FetchResult {
                                    source_idx: src_idx,
                                    channels: Vec::new(),
                                    eph_update: None,
                                    asteroid_samples: Vec::new(),
                                    star_samples: Vec::new(),
                                    curves: None,
                                    spectral: None,
                                    fetch_ok: false,
                                });
                                return;
                            }
                        };
                        if std::fs::write(&tmp_path, &bytes).is_err() {
                            eprintln!("csv_zip {}: write void — retry in ttl/Φ", src_idx);
                            let _ = ftx.send(FetchResult {
                                source_idx: src_idx,
                                channels: Vec::new(),
                                eph_update: None,
                                asteroid_samples: Vec::new(),
                                star_samples: Vec::new(),
                                curves: None,
                                spectral: None,
                                fetch_ok: true,
                            });
                            return;
                        }
                    }
                    if let ExtractResult::Measurements(channels) =
                        extract(&src_clone, &tmp_path, now, &lsk_c)
                    {
                        let _ = ftx.send(FetchResult {
                            source_idx: src_idx,
                            channels,
                            eph_update: None,
                            asteroid_samples: Vec::new(),
                            star_samples: Vec::new(),
                            curves: None,
                            spectral: None,
                            fetch_ok: true,
                        });
                    } else {
                        eprintln!("csv_zip {}: extract void — retry in ttl/Φ", src_idx);
                        let _ = ftx.send(FetchResult {
                            source_idx: src_idx,
                            channels: Vec::new(),
                            eph_update: None,
                            asteroid_samples: Vec::new(),
                            star_samples: Vec::new(),
                            curves: None,
                            spectral: None,
                            fetch_ok: true,
                        });
                    }
                });
                continue;
            }
            let mut fields: Vec<FieldConfig> = Vec::new();
            for ext in &archive.sources[i].extracts {
                fields.extend(extract_fields(ext));
            }
            let Some(r) = dispatch_reach(&fields, archive.sources[i].ttl as f64) else {
                if fields.is_empty() {
                    eprintln!(
                        "source {}: carries no field lines — refused, retry in ttl/Φ",
                        i
                    );
                    if let Ok(mut ledger) = refusal_ledger.lock() {
                        ledger.register(&archive.sources[i].url, "gate-no-field-lines");
                    }
                } else {
                    eprintln!(
                        "source {}: no field carries a propagation law — refused, retry in ttl/Φ",
                        i
                    );
                    if let Ok(mut ledger) = refusal_ledger.lock() {
                        ledger.register(&archive.sources[i].url, "gate-no-propagation");
                    }
                }
                continue;
            };
            let pos = match &archive.sources[i].frame {
                Frame::Surface {
                    lat,
                    lon,
                    alt,
                    body_name,
                } => {
                    if let Some(p) = body_fixed_to_icrs(
                        body_name,
                        *lat,
                        *lon,
                        *alt,
                        now,
                        &archive.body_ephemerides,
                    ) {
                        (p[0], p[1], p[2])
                    } else {
                        continue;
                    }
                }
                Frame::Barycenter { body_name, scale } => {
                    if let Some(bp) =
                        body_barycenter_position(body_name, now, &archive.body_ephemerides)
                    {
                        (bp[0] * scale, bp[1] * scale, bp[2] * scale)
                    } else {
                        continue;
                    }
                }
                Frame::Manifest => continue,
            };
            let presences: Vec<(f64, f64, f64, f64, f64, f64, f64, f64, f64)> =
                archive.presence.values().cloned().collect();
            if !presence_gate(&presences, pos, r) {
                continue;
            }
            begin_fetch(&mut archive.origins, i as u32, now);
            let ftx = fetch_tx.clone();
            let src_clone = archive.sources[i].clone();
            let eph_arc = archive.body_ephemerides.clone();
            let e = env.clone();
            let src_idx = i;
            let lsk_c = lsk.clone();
            let rl = refusal_ledger.clone();
            let presence_center = presences.first().map(|p| (p.2, p.3, p.4));
            thread::spawn(move || {
                if src_clone.fanout_cap > 0 {
                    if let Some(ref su) = src_clone.stations_url {
                        let channels = fanout_fetch(
                            &src_clone,
                            su,
                            pos.0,
                            pos.1,
                            pos.2,
                            presence_center,
                            now,
                            r,
                            &eph_arc,
                            &e,
                            &lsk_c,
                        );
                        let _ = ftx.send(FetchResult {
                            source_idx: src_idx,
                            channels,
                            eph_update: None,
                            asteroid_samples: Vec::new(),
                            star_samples: Vec::new(),
                            curves: None,
                            spectral: None,
                            fetch_ok: true,
                        });
                    } else {
                        eprintln!("fanout {}: stations_url absent — retry in ttl/Φ", src_idx);
                        let _ = ftx.send(FetchResult {
                            source_idx: src_idx,
                            channels: Vec::new(),
                            eph_update: None,
                            asteroid_samples: Vec::new(),
                            star_samples: Vec::new(),
                            curves: None,
                            spectral: None,
                            fetch_ok: true,
                        });
                    }
                    return;
                }
                let url = match render_source_url(
                    &src_clone, pos.0, pos.1, pos.2, now, r, &eph_arc, &e, &lsk_c,
                ) {
                    Some(u) => u,
                    None => {
                        eprintln!("source {}: url render void — retry in ttl/Φ", src_idx);
                        if let Ok(mut ledger) = rl.lock() {
                            ledger.register(&src_clone.url, "url-render-void");
                        }
                        let _ = ftx.send(FetchResult {
                            source_idx: src_idx,
                            channels: Vec::new(),
                            eph_update: None,
                            asteroid_samples: Vec::new(),
                            star_samples: Vec::new(),
                            curves: None,
                            spectral: None,
                            fetch_ok: true,
                        });
                        return;
                    }
                };
                let body =
                    render_source_body(&src_clone, pos.0, pos.1, pos.2, now, r, &eph_arc, &lsk_c);
                let headers = render_headers(&src_clone.headers, &e);
                let raw = fetch_one(&url, body.as_deref(), &headers, src_clone.ttl, Some(now));
                let fetch_ok = raw.is_some();
                let channels = match raw {
                    Some(ref r) => match extract(&src_clone, r, now, &lsk_c) {
                        ExtractResult::Measurements(v) => {
                            if v.is_empty() {
                                eprintln!("source {}: extract returned no measurements", src_idx);
                                if let Ok(mut ledger) = rl.lock() {
                                    ledger.register(&src_clone.url, "extract-void");
                                }
                            }
                            v
                        }
                        ExtractResult::WithEphemeris(v, eph) => {
                            let _ = ftx.send(FetchResult {
                                source_idx: src_idx,
                                channels: Vec::new(),
                                eph_update: src_clone.body.clone().map(|b| (b, eph)),
                                asteroid_samples: Vec::new(),
                                star_samples: Vec::new(),
                                curves: None,
                                spectral: None,
                                fetch_ok: true,
                            });
                            v
                        }
                    },
                    None => {
                        eprintln!("source {}: fetch void — retry in ttl/Φ·2ⁿ", src_idx);
                        if let Ok(mut ledger) = rl.lock() {
                            ledger.register(&src_clone.url, "fetch-void");
                        }
                        Vec::new()
                    }
                };
                let _ = ftx.send(FetchResult {
                    source_idx: src_idx,
                    channels,
                    eph_update: None,
                    asteroid_samples: Vec::new(),
                    star_samples: Vec::new(),

                    curves: None,
                    spectral: None,
                    fetch_ok,
                });
            });
        }
        {
            let old = archive.field.clone();
            let retained_estimate: usize = old.cache.cells.values().map(|v| v.len()).sum::<usize>()
                + old.cache.unbounded.len();
            let mut all: Vec<Sample> = Vec::with_capacity(
                fetched_samples.len()
                    + retained_estimate
                    + archive.body_ephemerides.len() * 2
                    + archive.asteroid_samples.len(),
            );
            all.append(&mut fetched_samples);
            for v in old
                .cache
                .cells
                .values()
                .chain(std::iter::once(&old.cache.unbounded))
            {
                for s in v {
                    if matches!(s.source, SampleSource::Ephemeris) {
                        continue;
                    }
                    if (now - s.epoch).abs() <= s.ttl * 64.0 {
                        all.push(s.clone());
                    }
                }
            }
            for (name, eph) in archive.body_ephemerides.iter() {
                if let Some(props) = &eph.props {
                    if props.radius_m > 0.0 {
                        let Some(body_ttl) = archive
                            .sources
                            .iter()
                            .find(|s| s.body.as_deref() == Some(name.as_str()))
                            .map(|s| s.ttl as f64)
                        else {
                            continue;
                        };
                        let frame = Frame::Barycenter {
                            body_name: name.clone(),
                            scale: 1.0,
                        };
                        for (channel, sensor) in body_channels(name, props, now) {
                            if let Some(mut sample) = anchor(
                                &channel,
                                &sensor,
                                body_ttl,
                                Some(archive.sources.len() as u32),
                                Some(&frame),
                                None,
                                &archive.body_ephemerides,
                            ) {
                                sample.source = SampleSource::Ephemeris;
                                all.push(sample);
                            }
                        }
                    }
                }
            }
            all.extend(archive.asteroid_samples.iter().cloned());
            all.extend(archive.star_samples.iter().cloned());
            if all.len() > MAX_SAMPLES {
                let dropped = all.len() - MAX_SAMPLES;
                all.sort_by(|a, b| b.epoch.total_cmp(&a.epoch));
                all.truncate(MAX_SAMPLES);
                eprintln!(
                    "sample cap {} reached — {} dropped (newest kept)",
                    MAX_SAMPLES, dropped
                );
            }
            archive.field = Arc::new(build_buffer(
                all,
                cadence,
                archive.body_ephemerides.clone(),
                archive.curves.clone(),
                archive.spectral.clone(),
            ));
        }
        let f = archive.field.clone();
        for r in &mut radiators {
            r.accept(f.clone());
        }
        let elapsed = match lsk.system_now_tdb() {
            Some(t) => t - wall_entered,
            None => cadence,
        };
        if elapsed < cadence {
            thread::sleep(std::time::Duration::from_secs_f64(cadence - elapsed));
        }
    }
}

fn take_u32(bytes: &[u8], off: &mut usize) -> Option<u32> {
    let raw: [u8; 4] = bytes.get(*off..*off + 4)?.try_into().ok()?;
    *off += 4;
    Some(u32::from_le_bytes(raw))
}

fn take_f64(bytes: &[u8], off: &mut usize) -> Option<f64> {
    let raw: [u8; 8] = bytes.get(*off..*off + 8)?.try_into().ok()?;
    *off += 8;
    Some(f64::from_le_bytes(raw))
}

fn take_f32(bytes: &[u8], off: &mut usize) -> Option<f32> {
    let raw: [u8; 4] = bytes.get(*off..*off + 4)?.try_into().ok()?;
    *off += 4;
    Some(f32::from_le_bytes(raw))
}

pub fn build_curve_set(bytes: &[u8]) -> CurveSet {
    let mut stars = Vec::new();
    if bytes.len() < 8 || &bytes[0..4] != b"TSS1" {
        return CurveSet { stars };
    }
    let mut off = 4usize;
    let Some(n_stars) = take_u32(bytes, &mut off) else {
        return CurveSet { stars };
    };
    for _ in 0..n_stars {
        let Some(ra_deg) = take_f64(bytes, &mut off) else {
            return CurveSet { stars };
        };
        let Some(dec_deg) = take_f64(bytes, &mut off) else {
            return CurveSet { stars };
        };
        let Some(plx_mas) = take_f64(bytes, &mut off) else {
            return CurveSet { stars };
        };
        let Some(n_samples) = take_u32(bytes, &mut off) else {
            return CurveSet { stars };
        };
        let mut samples = Vec::with_capacity(n_samples as usize);
        for _ in 0..n_samples {
            let Some(t) = take_f64(bytes, &mut off) else {
                return CurveSet { stars };
            };
            let Some(f) = take_f32(bytes, &mut off) else {
                return CurveSet { stars };
            };
            samples.push((t, f));
        }
        if samples.len() < 2 {
            continue;
        }
        let mut gaps: Vec<f64> = samples
            .windows(2)
            .map(|w| w[1].0 - w[0].0)
            .filter(|g| *g > 0.0)
            .collect();
        if gaps.is_empty() {
            continue;
        }
        gaps.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let cadence = gaps[gaps.len() / 2];
        stars.push(CurveStar {
            ra_deg,
            dec_deg,
            plx_mas,
            cadence,
            samples,
        });
    }
    CurveSet { stars }
}
