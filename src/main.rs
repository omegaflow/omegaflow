#![allow(mixed_script_confusables)]
use omegaflow::dastcom::{
    accel_at_epoch, hill_radius_m, parse_record, speed_at_epoch, state_at, AsteroidRec,
    RECORD_STRIDE,
};
use omegaflow::inflate::unzip;
use omegaflow::lsk::LeapSeconds;
use omegaflow::pck::PckBody;
use std::{
    collections::HashMap,
    io::{Cursor, IsTerminal, Read, Write},
    net::{TcpListener, TcpStream},
    process::Command,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc, Arc, Mutex,
    },
    thread,
    time::{SystemTime, UNIX_EPOCH},
};

const Φ: f64 = 1.618033988749895;
const J2000_EPOCH: f64 = 2451545.0;
const PARSEC_M: f64 = 3.085677581e16;
const C_LIGHT: f64 = 299792458.0;
const HUBBLE_H0: f64 = 70000.0 / (PARSEC_M * 1.0e6);
const MAS_YR_TO_RAD_S: f64 = 4.84813681109536e-9 / 31557600.0;

fn resolve_asset(rel: &str) -> std::path::PathBuf {
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

const CHEBYSHEV_N: usize = 18;
const ECLIPTIC_OBLIQUITY: f64 = 0.409092804;
const AU: f64 = 1.495978707e11;
const GAUSS_K: f64 = 0.01720209895;
const PORT_CONST: u16 = 1618;
const SONIFICATION_ROOT_FREQ: f32 = 220.0;
const SONIFICATION_KERNEL_STEP: f32 = 110.0;
const SONIFICATION_FORCE_STEP: f32 = 55.0;
const SURFACE_MOTION_DT: f64 = 0.01;

#[derive(Clone)]
struct BodyProperties {
    α0_deg: f64,
    dα0_dt_deg_per_century: f64,
    δ0_deg: f64,
    dδ0_dt_deg_per_century: f64,
    w0_deg: f64,
    dw_dt_deg_per_day: f64,
    radius_m: f64,
    flattening: f64,
    gaussian_inverse_square: f64,
    gaussian_inverse: f64,
    erfc: f64,
    exponential_decay: f64,
    patch_levy: f64,
    gm: Option<f64>,
    j2: Option<f64>,
    j4: Option<f64>,
    radii_b: Option<f64>,
    radii_c: Option<f64>,
    nut_ra: Option<Vec<[f64; 3]>>,
    nut_dec: Option<Vec<[f64; 3]>>,
    nutation: Option<Vec<NutationRecord>>,
}

#[derive(Clone)]
struct NutationRecord {
    mid_jd: f64,
    half_jd: f64,
    ra: Vec<f64>,
    dec: Vec<f64>,
    pm: Vec<f64>,
}

#[derive(Clone)]
struct ChebyshevGranule {
    t0_jd: f64,
    dt_jd: f64,
    cx: [f64; CHEBYSHEV_N],
    cy: [f64; CHEBYSHEV_N],
    cz: [f64; CHEBYSHEV_N],
}

#[derive(Clone)]
struct BodyEphemeris {
    granules: Vec<ChebyshevGranule>,
    rotation_matrices: Vec<(f64, [f64; 9])>,
    props: Option<BodyProperties>,
}

fn chebyshev_evaluate(coeffs: &[f64; CHEBYSHEV_N], tau: f64) -> f64 {
    let mut b0 = 0.0;
    let mut b1 = 0.0;
    for i in (0..CHEBYSHEV_N).rev() {
        let b2 = b1;
        b1 = b0;
        b0 = 2.0 * tau * b1 - b2 + coeffs[i];
    }
    b0 - tau * b1
}

fn chebyshev_eval_slice(coeffs: &[f64], tau: f64) -> f64 {
    let mut b0 = 0.0;
    let mut b1 = 0.0;
    for i in (0..coeffs.len()).rev() {
        let b2 = b1;
        b1 = b0;
        b0 = 2.0 * tau * b1 - b2 + coeffs[i];
    }
    b0 - tau * b1
}

fn nutation_deltas_at(props: &BodyProperties, jd: f64) -> Option<(f64, f64, f64)> {
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

fn orientation_angles_at(bp: &BodyProperties, jd: f64) -> (f64, f64, f64) {
    let tc = (jd - J2000_EPOCH) / 36525.0;
    let (d_ra, d_dec, d_pm) = nutation_deltas_at(bp, jd).unwrap_or((0.0, 0.0, 0.0));
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

fn measured(v: f64) -> Option<f64> {
    if v != 0.0 {
        Some(v)
    } else {
        None
    }
}

fn parse_ephemeris_binary(data: &[u8]) -> Option<BodyEphemeris> {
    if data.len() < 24 || data[0] != 0xCF || data[1] != 0x86 || data[2] != 0x01 {
        return None;
    }
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
            let radius_m = f(6);
            let radii_b = measured(f(7));
            let radii_c = measured(f(8));
            props = Some(BodyProperties {
                α0_deg: f(0),
                dα0_dt_deg_per_century: f(1),
                δ0_deg: f(2),
                dδ0_dt_deg_per_century: f(3),
                w0_deg: f(4),
                dw_dt_deg_per_day: f(5),
                radius_m,
                flattening: match radii_c {
                    Some(c) if radius_m > 0.0 => (radius_m - c) / radius_m,
                    _ => 0.0,
                },
                gaussian_inverse_square: 0.0,
                gaussian_inverse: 0.0,
                erfc: 0.0,
                exponential_decay: 0.0,
                patch_levy: 0.0,
                gm: measured(f(11)),
                j2: measured(f(9)),
                j4: measured(f(10)),
                radii_b,
                radii_c,
                nut_ra: None,
                nut_dec: None,
                nutation: None,
            });
            pos += 96;
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

fn body_barycenter_position(
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

fn body_fixed_to_icrs(
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
        Some(_) | None => bp.flattening,
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

fn icrs_to_body_surface(
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
    let f = bp.flattening;
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

type CellKey = (i64, i64, i64);
type Origin = u32;

#[derive(Clone)]
enum Motion {
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

impl Motion {
    fn at(&self, t: f64, epoch: f64, eph: &HashMap<String, BodyEphemeris>) -> Option<[f64; 3]> {
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
    fn anchor_body(&self) -> Option<&str> {
        match self {
            Motion::Surface { body_name, .. } | Motion::Barycenter { body_name, .. } => {
                Some(body_name)
            }
            Motion::Linear { .. } => None,
        }
    }
}

#[derive(Clone)]
enum OscillatorSource {
    Api(u32),
    Device,
    Body,
}

#[derive(Clone)]
struct Oscillator {
    source: OscillatorSource,
    epoch: f64,
    ttl: f64,
    extent: f64,
    tau: f64,
    kernel_id: f64,
    force_type: f64,
    absorption: f64,
    advection: f64,
    vmax: f64,
    amax: f64,
    p0f: [f64; 3],
    motion: Motion,
    val: f64,
    name: String,
}

struct SpatialHash {
    cell_size: f64,
    vmax: f64,
    amax: f64,
    rmax: f64,
    epoch_min: f64,
    cell_lo: CellKey,
    cell_hi: CellKey,
    cells: HashMap<CellKey, Vec<Oscillator>>,
    unbounded: Vec<Oscillator>,
}

type OscRecord = (
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

struct Buffer {
    bodies: HashMap<String, SpatialHash>,
    inertial: SpatialHash,
    eph: Arc<HashMap<String, BodyEphemeris>>,
    asteroids: Option<Arc<AsteroidHash>>,
    stars: Option<Arc<StarHash>>,
}

fn cell_of(p: [f64; 3], s: f64) -> CellKey {
    (
        (p[0] / s).floor() as i64,
        (p[1] / s).floor() as i64,
        (p[2] / s).floor() as i64,
    )
}

fn relative_frame_position(
    motion: &Motion,
    t: f64,
    epoch: f64,
    eph: &HashMap<String, BodyEphemeris>,
) -> Option<[f64; 3]> {
    let p = motion.at(t, epoch, eph)?;
    match motion.anchor_body() {
        Some(body) => {
            let b = body_barycenter_position(body, t, eph)?;
            Some([p[0] - b[0], p[1] - b[1], p[2] - b[2]])
        }
        None => Some(p),
    }
}

fn law_bounds(
    motion: &Motion,
    epoch: f64,
    resid_ema: f64,
    eph: &HashMap<String, BodyEphemeris>,
) -> Option<(f64, f64, [f64; 3])> {
    let p0 = relative_frame_position(motion, epoch, epoch, eph)?;
    let p1 = relative_frame_position(motion, epoch + 1.0, epoch, eph)?;
    let p2 = relative_frame_position(motion, epoch + 2.0, epoch, eph)?;
    let v = ((p1[0] - p0[0]).powi(2) + (p1[1] - p0[1]).powi(2) + (p1[2] - p0[2]).powi(2)).sqrt();
    let a = ((p2[0] - 2.0 * p1[0] + p0[0]).powi(2)
        + (p2[1] - 2.0 * p1[1] + p0[1]).powi(2)
        + (p2[2] - 2.0 * p1[2] + p0[2]).powi(2))
    .sqrt();
    Some((Φ * (v + resid_ema), Φ * a, p0))
}

fn build_spatial_hash(samples: Vec<Oscillator>, cadence: f64) -> SpatialHash {
    let mut bounded = Vec::new();
    let mut unbounded = Vec::new();
    for s in samples {
        if s.extent.is_finite() {
            bounded.push(s);
        } else {
            unbounded.push(s);
        }
    }
    let mut vmax = 0.0f64;
    let mut amax = 0.0f64;
    let mut rmax = 0.0f64;
    let mut epoch_min = f64::MAX;
    for s in &bounded {
        vmax = vmax.max(s.vmax);
        amax = amax.max(s.amax);
        rmax = rmax.max(s.extent);
        epoch_min = epoch_min.min(s.epoch);
    }
    let rho_cad = rmax + vmax * cadence + 0.5 * amax * cadence * cadence;
    let shift = (2.0 * rho_cad).log2().ceil().clamp(0.0, 63.0) as i32;
    let cell_size = 2f64.powi(shift);
    let mut cells: HashMap<CellKey, Vec<Oscillator>> = HashMap::new();
    let mut cell_lo = (i64::MAX, i64::MAX, i64::MAX);
    let mut cell_hi = (i64::MIN, i64::MIN, i64::MIN);
    for s in bounded {
        let c = cell_of(s.p0f, cell_size);
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
        unbounded,
    }
}

fn build_buffer(
    samples: Vec<Oscillator>,
    cadence: f64,
    eph: Arc<HashMap<String, BodyEphemeris>>,
    asteroids: Option<Arc<AsteroidHash>>,
    stars: Option<Arc<StarHash>>,
) -> Buffer {
    let mut body_samps: HashMap<String, Vec<Oscillator>> = HashMap::new();
    let mut inertial_samps = Vec::new();
    for s in samples {
        if let Some(body) = s.motion.anchor_body() {
            body_samps.entry(body.to_string()).or_default().push(s);
        } else {
            inertial_samps.push(s);
        }
    }
    let mut bodies = HashMap::new();
    for (name, oscs) in body_samps {
        bodies.insert(name, build_spatial_hash(oscs, cadence));
    }
    Buffer {
        bodies,
        inertial: build_spatial_hash(inertial_samps, cadence),
        eph,
        asteroids,
        stars,
    }
}

struct AsteroidHash {
    cell_size: f64,
    vmax: f64,
    amax: f64,
    rmax: f64,
    epoch_min_secs: f64,
    ttl: f64,
    cell_lo: CellKey,
    cell_hi: CellKey,
    cells: HashMap<CellKey, Vec<u32>>,
    records: Vec<AsteroidRec>,
    p0: Vec<[f64; 3]>,
}

fn build_asteroid_hash(bytes: &[u8], cadence: f64, ttl: u64) -> AsteroidHash {
    let mut records: Vec<AsteroidRec> = Vec::new();
    let mut p0: Vec<[f64; 3]> = Vec::new();
    let mut vmax = 0.0f64;
    let mut amax = 0.0f64;
    let mut rmax = 0.0f64;
    let mut epoch_min = f64::MAX;
    for chunk in bytes.chunks_exact(RECORD_STRIDE) {
        let rec = match parse_record(chunk) {
            Some(r) => r,
            None => continue,
        };
        if rec.number == 0 || rec.a_au <= 0.0 || rec.e >= 1.0 {
            continue;
        }
        let hill = match hill_radius_m(&rec) {
            Some(h) => h,
            None => continue,
        };
        let (pos, vel) = match state_at(&rec, rec.epoch_jd) {
            Some(s) => s,
            None => continue,
        };
        let speed = (vel[0] * vel[0] + vel[1] * vel[1] + vel[2] * vel[2]).sqrt();
        let accel = accel_at_epoch(&rec).unwrap_or(0.0);
        let epoch_secs = (rec.epoch_jd - J2000_EPOCH) * 86400.0;
        vmax = vmax.max(speed);
        amax = amax.max(accel);
        rmax = rmax.max(hill);
        epoch_min = epoch_min.min(epoch_secs);
        records.push(rec);
        p0.push(pos);
    }
    vmax *= Φ;
    amax *= Φ;
    let rho_cad = rmax + vmax * cadence + 0.5 * amax * cadence * cadence;
    let shift = (2.0 * rho_cad).log2().ceil().clamp(0.0, 63.0) as i32;
    let cell_size = 2f64.powi(shift);
    let mut cells: HashMap<CellKey, Vec<u32>> = HashMap::new();
    let mut cell_lo = (i64::MAX, i64::MAX, i64::MAX);
    let mut cell_hi = (i64::MIN, i64::MIN, i64::MIN);
    for (i, pos) in p0.iter().enumerate() {
        let c = cell_of(*pos, cell_size);
        cell_lo.0 = cell_lo.0.min(c.0);
        cell_lo.1 = cell_lo.1.min(c.1);
        cell_lo.2 = cell_lo.2.min(c.2);
        cell_hi.0 = cell_hi.0.max(c.0);
        cell_hi.1 = cell_hi.1.max(c.1);
        cell_hi.2 = cell_hi.2.max(c.2);
        cells.entry(c).or_default().push(i as u32);
    }
    AsteroidHash {
        cell_size,
        vmax,
        amax,
        rmax,
        epoch_min_secs: if epoch_min == f64::MAX {
            0.0
        } else {
            epoch_min
        },
        ttl: ttl as f64,
        cell_lo,
        cell_hi,
        cells,
        records,
        p0,
    }
}

fn query_asteroid_hash(
    hash: &AsteroidHash,
    center: [f64; 3],
    t2: f64,
    pad: f64,
    delta_t_cache: f64,
    records: &mut Vec<OscRecord>,
) {
    if hash.cells.is_empty() {
        return;
    }
    let dt = (t2 - hash.epoch_min_secs).abs() + delta_t_cache;
    let rho = hash.rmax + hash.vmax * dt + 0.5 * hash.amax * dt * dt + pad;
    let s = hash.cell_size;
    let qlo = cell_of([center[0] - rho, center[1] - rho, center[2] - rho], s);
    let qhi = cell_of([center[0] + rho, center[1] + rho, center[2] + rho], s);
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
    let span = ((hi.0 - lo.0 + 1) as u64)
        .saturating_mul((hi.1 - lo.1 + 1) as u64)
        .saturating_mul((hi.2 - lo.2 + 1) as u64);
    let visit: Vec<&Vec<u32>> = if span > hash.cells.len() as u64 * 4 {
        hash.cells
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
                    if let Some(indices) = hash.cells.get(&(cx, cy, cz)) {
                        out.push(indices);
                    }
                }
            }
        }
        out
    };
    let t_jd = t2 / 86400.0 + J2000_EPOCH;
    for indices in visit {
        for &i in indices {
            let rec = &hash.records[i as usize];
            let hill = match hill_radius_m(rec) {
                Some(h) => h,
                None => continue,
            };
            let epoch_secs = (rec.epoch_jd - J2000_EPOCH) * 86400.0;
            let age = (t2 - epoch_secs).abs();
            let future_age = age + delta_t_cache;
            let speed = speed_at_epoch(rec).unwrap_or(0.0);
            let accel = accel_at_epoch(rec).unwrap_or(0.0);
            let reach = hill + speed * future_age + 0.5 * accel * future_age * future_age + pad;
            let p0 = hash.p0[i as usize];
            let dx = p0[0] - center[0];
            let dy = p0[1] - center[1];
            let dz = p0[2] - center[2];
            let dist2_p0 = dx * dx + dy * dy + dz * dz;
            if dist2_p0 > reach * reach {
                continue;
            }
            let (p, v) = match state_at(rec, t_jd) {
                Some(s) => s,
                None => continue,
            };
            let ddx = p[0] - center[0];
            let ddy = p[1] - center[1];
            let ddz = p[2] - center[2];
            let exact = hill + pad;
            let dist2 = ddx * ddx + ddy * ddy + ddz * ddz;
            if dist2 > exact * exact {
                continue;
            }
            let gm = rec.gm_km3_s2 as f64 * 1.0e9;
            records.push((
                p[0],
                p[1],
                p[2],
                gm,
                epoch_secs,
                hash.ttl,
                f64::INFINITY,
                hill,
                0.0,
                1.0,
                0.0,
                0.0,
                v[0],
                v[1],
                v[2],
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
            ));
            if rec.radius_km > 0.0 {
                records.push((
                    p[0],
                    p[1],
                    p[2],
                    rec.radius_km as f64 * 1000.0,
                    epoch_secs,
                    hash.ttl,
                    f64::INFINITY,
                    hill,
                    0.0,
                    1.0,
                    0.0,
                    0.0,
                    v[0],
                    v[1],
                    v[2],
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                ));
            }
        }
    }
}

const STAR_RECORD_STRIDE: usize = 36;

struct StarRec {
    ra_deg: f64,
    dec_deg: f64,
    pm_ra_masyr: f64,
    pm_de_masyr: f64,
    plx_mas: f64,
    flux: f64,
}

fn parse_star_record(b: &[u8]) -> Option<StarRec> {
    if b.len() < STAR_RECORD_STRIDE {
        return None;
    }
    let ra = f64::from_le_bytes(b[0..8].try_into().ok()?);
    let dec = f64::from_le_bytes(b[8..16].try_into().ok()?);
    let pm_ra = f32::from_le_bytes(b[16..20].try_into().ok()?) as f64;
    let pm_de = f32::from_le_bytes(b[20..24].try_into().ok()?) as f64;
    let plx = f32::from_le_bytes(b[24..28].try_into().ok()?) as f64;
    let vt = f32::from_le_bytes(b[28..32].try_into().ok()?) as f64;
    let flux = f32::from_le_bytes(b[32..36].try_into().ok()?) as f64;
    if !ra.is_finite() || !dec.is_finite() || !(plx > 0.0) || !vt.is_finite() {
        return None;
    }
    Some(StarRec {
        ra_deg: ra,
        dec_deg: dec,
        pm_ra_masyr: pm_ra,
        pm_de_masyr: pm_de,
        plx_mas: plx,
        flux,
    })
}

fn star_position_at(rec: &StarRec, t2: f64) -> ([f64; 3], [f64; 3]) {
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
    let vel = [
        d * (mu_a * a_hat[0] + mu_d * d_hat[0]),
        d * (mu_a * a_hat[1] + mu_d * d_hat[1]),
        d * (mu_a * a_hat[2] + mu_d * d_hat[2]),
    ];
    (p, vel)
}

struct StarHash {
    cell_size: f64,
    vmax: f64,
    ttl: f64,
    build_epoch: f64,
    cell_lo: CellKey,
    cell_hi: CellKey,
    cells: HashMap<CellKey, Vec<u32>>,
    records: Vec<StarRec>,
    p0: Vec<[f64; 3]>,
}

fn build_star_hash(bytes: &[u8], build_epoch: f64, cadence: f64, ttl: u64) -> StarHash {
    let mut records: Vec<StarRec> = Vec::new();
    let mut p0: Vec<[f64; 3]> = Vec::new();
    let mut vmax = 0.0f64;
    for chunk in bytes.chunks_exact(STAR_RECORD_STRIDE) {
        let Some(rec) = parse_star_record(chunk) else {
            continue;
        };
        let (p, v) = star_position_at(&rec, build_epoch);
        let speed = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
        vmax = vmax.max(speed);
        records.push(rec);
        p0.push(p);
    }
    vmax *= Φ;
    let rho_cad = vmax * cadence;
    let shift = (2.0 * rho_cad).log2().ceil().clamp(0.0, 63.0) as i32;
    let cell_size = 2f64.powi(shift);
    let mut cells: HashMap<CellKey, Vec<u32>> = HashMap::new();
    let mut cell_lo = (i64::MAX, i64::MAX, i64::MAX);
    let mut cell_hi = (i64::MIN, i64::MIN, i64::MIN);
    for (i, pos) in p0.iter().enumerate() {
        let c = cell_of(*pos, cell_size);
        cell_lo.0 = cell_lo.0.min(c.0);
        cell_lo.1 = cell_lo.1.min(c.1);
        cell_lo.2 = cell_lo.2.min(c.2);
        cell_hi.0 = cell_hi.0.max(c.0);
        cell_hi.1 = cell_hi.1.max(c.1);
        cell_hi.2 = cell_hi.2.max(c.2);
        cells.entry(c).or_default().push(i as u32);
    }
    StarHash {
        cell_size,
        vmax,
        ttl: ttl as f64,
        build_epoch,
        cell_lo,
        cell_hi,
        cells,
        records,
        p0,
    }
}

fn query_star_hash(
    hash: &StarHash,
    center: [f64; 3],
    t2: f64,
    pad: f64,
    delta_t_cache: f64,
    records: &mut Vec<OscRecord>,
) {
    if hash.cells.is_empty() {
        return;
    }
    let dt = (t2 - hash.build_epoch).abs() + delta_t_cache;
    let rho = hash.vmax * dt + pad;
    let s = hash.cell_size;
    let qlo = cell_of([center[0] - rho, center[1] - rho, center[2] - rho], s);
    let qhi = cell_of([center[0] + rho, center[1] + rho, center[2] + rho], s);
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
    let span = ((hi.0 - lo.0 + 1) as u64)
        .saturating_mul((hi.1 - lo.1 + 1) as u64)
        .saturating_mul((hi.2 - lo.2 + 1) as u64);
    let visit: Vec<&Vec<u32>> = if span > hash.cells.len() as u64 * 4 {
        hash.cells
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
                    if let Some(indices) = hash.cells.get(&(cx, cy, cz)) {
                        out.push(indices);
                    }
                }
            }
        }
        out
    };
    for indices in visit {
        for &i in indices {
            let rec = &hash.records[i as usize];
            let d = (1000.0 / rec.plx_mas) * PARSEC_M;
            let mu_a = rec.pm_ra_masyr * MAS_YR_TO_RAD_S;
            let mu_d = rec.pm_de_masyr * MAS_YR_TO_RAD_S;
            let v_lin = d * mu_a.hypot(mu_d);
            let reach = v_lin * dt + pad;
            let p0 = hash.p0[i as usize];
            let dx = p0[0] - center[0];
            let dy = p0[1] - center[1];
            let dz = p0[2] - center[2];
            let dist2_p0 = dx * dx + dy * dy + dz * dz;
            if dist2_p0 > reach * reach {
                continue;
            }
            let (p, v) = star_position_at(rec, t2);
            let ddx = p[0] - center[0];
            let ddy = p[1] - center[1];
            let ddz = p[2] - center[2];
            let dist2 = ddx * ddx + ddy * ddy + ddz * ddz;
            if dist2 > pad * pad {
                continue;
            }
            records.push((
                p[0], p[1], p[2], rec.flux, t2, hash.ttl, hash.ttl, 0.0, 0.0, 0.0, 0.0, 0.0, v[0],
                v[1], v[2], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ));
        }
    }
}

fn query_hash(
    hash: &SpatialHash,
    anchor: [f64; 3],
    center: [f64; 3],
    t2: f64,
    pad: f64,
    delta_t_cache: f64,
    records: &mut Vec<(
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
    )>,
    body_props: Option<&BodyProperties>,
    eph: &HashMap<String, BodyEphemeris>,
) {
    for osc in &hash.unbounded {
        let p = match osc.motion.at(t2, osc.epoch, eph) {
            Some(p) => p,
            None => continue,
        };
        let v = if let Motion::Linear { v, .. } = &osc.motion {
            [v[0], v[1], v[2]]
        } else {
            let p_dt = match osc.motion.at(t2 + 1e-3, osc.epoch, eph) {
                Some(pd) => pd,
                None => continue,
            };
            [
                (p_dt[0] - p[0]) / 1e-3,
                (p_dt[1] - p[1]) / 1e-3,
                (p_dt[2] - p[2]) / 1e-3,
            ]
        };
        let (pole, j2, j4, r_eq) = gravity_manifest(osc, t2, eph);
        records.push((
            p[0],
            p[1],
            p[2],
            osc.val,
            osc.epoch,
            osc.ttl,
            osc.tau,
            osc.extent,
            osc.kernel_id,
            osc.force_type,
            osc.absorption,
            osc.advection,
            v[0],
            v[1],
            v[2],
            pole[0],
            pole[1],
            pole[2],
            j2,
            j4,
            r_eq,
        ));
    }
    if hash.cells.is_empty() {
        return;
    }
    let qf = [
        center[0] - anchor[0],
        center[1] - anchor[1],
        center[2] - anchor[2],
    ];
    let dt = (t2 - hash.epoch_min).abs() + delta_t_cache;
    let rho = hash.rmax + hash.vmax * dt + 0.5 * hash.amax * dt * dt + pad;
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
    let span = ((hi.0 - lo.0 + 1) as u64)
        .saturating_mul((hi.1 - lo.1 + 1) as u64)
        .saturating_mul((hi.2 - lo.2 + 1) as u64);
    let visit: Vec<&Vec<Oscillator>> = if span > hash.cells.len() as u64 * 4 {
        hash.cells
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
                    if let Some(samples) = hash.cells.get(&(cx, cy, cz)) {
                        out.push(samples);
                    }
                }
            }
        }
        out
    };
    for samples in visit {
        for osc in samples {
            let age = (t2 - osc.epoch).abs();
            let causal_reach = kernel_extent(osc.kernel_id as u8, body_props, osc.tau);
            let future_age = age + delta_t_cache;
            let reach = osc.extent.max(causal_reach)
                + osc.vmax * future_age
                + 0.5 * osc.amax * future_age * future_age
                + pad;
            let dx = osc.p0f[0] - qf[0];
            let dy = osc.p0f[1] - qf[1];
            let dz = osc.p0f[2] - qf[2];
            let dist2_p0f = dx * dx + dy * dy + dz * dz;
            if dist2_p0f > reach * reach {
                continue;
            }
            let p = match osc.motion.at(t2, osc.epoch, eph) {
                Some(p) => p,
                None => continue,
            };
            let ddx = p[0] - center[0];
            let ddy = p[1] - center[1];
            let ddz = p[2] - center[2];
            let exact = osc.extent + pad;
            let dist2 = ddx * ddx + ddy * ddy + ddz * ddz;
            if dist2 > exact * exact {
                continue;
            }
            let v = if let Motion::Linear { v, .. } = &osc.motion {
                [v[0], v[1], v[2]]
            } else {
                let p_dt = match osc.motion.at(t2 + 1e-3, osc.epoch, eph) {
                    Some(pd) => pd,
                    None => continue,
                };
                [
                    (p_dt[0] - p[0]) / 1e-3,
                    (p_dt[1] - p[1]) / 1e-3,
                    (p_dt[2] - p[2]) / 1e-3,
                ]
            };
            let (pole, j2, j4, r_eq) = gravity_manifest(osc, t2, eph);
            records.push((
                p[0],
                p[1],
                p[2],
                osc.val,
                osc.epoch,
                osc.ttl,
                osc.tau,
                osc.extent,
                osc.kernel_id,
                osc.force_type,
                osc.absorption,
                osc.advection,
                v[0],
                v[1],
                v[2],
                pole[0],
                pole[1],
                pole[2],
                j2,
                j4,
                r_eq,
            ));
        }
    }
}

fn sense_buffer(
    buf: &Buffer,
    center: [f64; 3],
    t2: f64,
    pad: f64,
    delta_t_cache: f64,
    records: &mut Vec<(
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
    )>,
    eph: &HashMap<String, BodyEphemeris>,
) {
    for (body_name, hash_cell) in &buf.bodies {
        let anchor = match body_barycenter_position(body_name, t2, eph) {
            Some(a) => a,
            None => continue,
        };
        let body_props = eph.get(body_name).and_then(|e| e.props.as_ref());
        query_hash(
            hash_cell,
            anchor,
            center,
            t2,
            pad,
            delta_t_cache,
            records,
            body_props,
            eph,
        );
    }
    query_hash(
        &buf.inertial,
        [0.0, 0.0, 0.0],
        center,
        t2,
        pad,
        delta_t_cache,
        records,
        None,
        eph,
    );
    if let Some(ash) = &buf.asteroids {
        query_asteroid_hash(ash, center, t2, pad, delta_t_cache, records);
    }
    if let Some(sh) = &buf.stars {
        query_star_hash(sh, center, t2, pad, delta_t_cache, records);
    }
}

fn surface_motion(
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

fn frame_body_name(frame: &Frame) -> String {
    match frame {
        Frame::Surface { body_name, .. } | Frame::Barycenter { body_name, .. } => body_name.clone(),
        Frame::Manifest => String::new(),
    }
}

fn body_id_to_name(bodies: &[String], id: u32) -> Option<String> {
    if id == 0 {
        return None;
    }
    bodies.get((id - 1) as usize).cloned()
}

fn frame_motion(
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

fn leap_seconds(time: &Arc<Mutex<Option<LeapSeconds>>>) -> Option<LeapSeconds> {
    match time.lock() {
        Ok(guard) => guard.clone(),
        Err(_) => None,
    }
}

fn system_now(time: &Arc<Mutex<Option<LeapSeconds>>>) -> Option<f64> {
    match leap_seconds(time) {
        Some(lsk) => lsk.system_now_tdb(),
        None => None,
    }
}

fn nutation_sum(terms: &[[f64; 3]], t: f64) -> f64 {
    terms
        .iter()
        .map(|&[amplitude, frequency, phase]| amplitude * (frequency * t + phase).sin())
        .sum()
}

fn body_pole_at(props: &BodyProperties, tdb: f64) -> [f64; 3] {
    let jd = tdb / 86400.0 + J2000_EPOCH;
    let (ra, dec, _) = orientation_angles_at(props, jd);
    let (sd, cd) = dec.to_radians().sin_cos();
    let (sr, cr) = ra.to_radians().sin_cos();
    [cd * cr, cd * sr, sd]
}

fn gravity_manifest(
    osc: &Oscillator,
    t: f64,
    eph: &HashMap<String, BodyEphemeris>,
) -> ([f64; 3], f64, f64, f64) {
    let name = match osc.motion.anchor_body() {
        Some(n) => n,
        None => return ([0.0; 3], 0.0, 0.0, 0.0),
    };
    let props = match eph.get(name).and_then(|e| e.props.as_ref()) {
        Some(p) => p,
        None => return ([0.0; 3], 0.0, 0.0, 0.0),
    };
    let pole = body_pole_at(props, t);
    let j2 = match props.j2 {
        Some(v) => v,
        None => 0.0,
    };
    let j4 = match props.j4 {
        Some(v) => v,
        None => 0.0,
    };
    (pole, j2, j4, props.radius_m)
}

fn parse_iso_tdb(s: &str, lsk: &LeapSeconds) -> Option<f64> {
    let s = s.trim();
    let (date, time) = s.split_once('T')?;
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

fn ymd_to_days(year: i64, month: u32, day: u32) -> Option<u64> {
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

#[derive(Clone)]
struct OriginState {
    fetched: f64,
    prev_epoch: f64,
    prev_abs: [f64; 3],
    prev_motion: Option<Motion>,
    resid_ema: f64,
    has_prev: bool,
}

#[derive(Clone, Debug)]
enum Position {
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
struct Channel {
    name: String,
    value: f64,
    position: Position,
    epoch: f64,
}

fn origin_stale(
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

fn presence_gate(
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
    let bytes = s.as_bytes();
    let start = (0..bytes.len()).find(|&i| bytes[i] == b'{' || bytes[i] == b'[')?;
    let mut p = JsonParser {
        chars: bytes,
        pos: start,
    };
    p.skip_ws();
    p.parse_value()
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

fn csv_to_json(text: &str) -> Option<JsonVal> {
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

struct JsonParser<'a> {
    chars: &'a [u8],
    pos: usize,
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
                if self.chars[self.pos..].starts_with(b"true") {
                    self.pos += 4;
                    Some(JsonVal::Bool(true))
                } else {
                    None
                }
            }
            b'f' => {
                if self.chars[self.pos..].starts_with(b"false") {
                    self.pos += 5;
                    Some(JsonVal::Bool(false))
                } else {
                    None
                }
            }
            b'n' => {
                if self.chars[self.pos..].starts_with(b"null") {
                    self.pos += 4;
                    Some(JsonVal::Null)
                } else if self.chars[self.pos..].starts_with(b"nan") {
                    self.pos += 3;
                    Some(JsonVal::Num(f64::NAN))
                } else {
                    None
                }
            }
            b'N' => {
                if self.chars[self.pos..].starts_with(b"NaN") {
                    self.pos += 3;
                    Some(JsonVal::Num(f64::NAN))
                } else {
                    None
                }
            }
            b'I' => {
                if self.chars[self.pos..].starts_with(b"Infinity") {
                    self.pos += 8;
                    Some(JsonVal::Num(f64::INFINITY))
                } else {
                    None
                }
            }
            b'i' => {
                if self.chars[self.pos..].starts_with(b"inf") {
                    self.pos += 3;
                    Some(JsonVal::Num(f64::INFINITY))
                } else {
                    None
                }
            }
            b'-' if self.chars[self.pos..].starts_with(b"-Infinity") => {
                self.pos += 9;
                Some(JsonVal::Num(f64::NEG_INFINITY))
            }
            b'-' if self.chars[self.pos..].starts_with(b"-inf") => {
                self.pos += 4;
                Some(JsonVal::Num(f64::NEG_INFINITY))
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
                                    self.pos += 4;
                                    if (0xD800..=0xDBFF).contains(&cp)
                                        && self.pos + 6 <= self.chars.len()
                                        && self.chars[self.pos] == b'\\'
                                        && self.chars[self.pos + 1] == b'u'
                                    {
                                        if let Ok(hex_lo) = std::str::from_utf8(
                                            &self.chars[self.pos + 2..self.pos + 6],
                                        ) {
                                            if let Ok(lo) = u32::from_str_radix(hex_lo, 16) {
                                                if (0xDC00..=0xDFFF).contains(&lo) {
                                                    self.pos += 6;
                                                    let combined = 0x10000
                                                        + ((cp - 0xD800) << 10)
                                                        + (lo - 0xDC00);
                                                    if let Some(ch) = char::from_u32(combined) {
                                                        s.push(ch);
                                                    }
                                                    continue;
                                                }
                                            }
                                        }
                                    } else if let Some(ch) = char::from_u32(cp) {
                                        s.push(ch);
                                    }
                                } else {
                                    self.pos += 4;
                                }
                            } else {
                                self.pos += 4;
                            }
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
                let run_start = self.pos;
                while self.pos < self.chars.len()
                    && self.chars[self.pos] != b'"'
                    && self.chars[self.pos] != b'\\'
                {
                    self.pos += 1;
                }
                match std::str::from_utf8(&self.chars[run_start..self.pos]) {
                    Ok(t) => s.push_str(t),
                    Err(_) => {
                        s.push_str(&String::from_utf8_lossy(&self.chars[run_start..self.pos]))
                    }
                }
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

fn scalar_of(v: &JsonVal) -> Option<f64> {
    match v {
        JsonVal::Num(n) => Some(*n),
        JsonVal::Str(s) => s.parse().ok(),
        _ => None,
    }
}

fn universal_auto_detect(j: &JsonVal) -> Vec<Extract> {
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
            trk_key: trk_key.into(),
            vr_key: vr_key.into(),
            fields,
            lon_sign: None,
        }]
    } else {
        vec![]
    }
}

fn jpath_val<'a>(json: &'a JsonVal, path: &str) -> Option<&'a JsonVal> {
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
                    | Extract::First(FieldConfig { key, .. })
                    | Extract::Last(FieldConfig { key, .. })
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
                    | Extract::Ephemeris(_)
                    | Extract::Vectors(_)
                    | Extract::XmlCount(_, _) => {
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

fn jnum(json: &JsonVal, key: &str) -> Option<f64> {
    if key.contains('.') {
        return jpath_val(json, key).and_then(scalar_of);
    }
    match json {
        JsonVal::Obj(map) => map.get(key).and_then(scalar_of),
        _ => None,
    }
}

fn jpath(json: &JsonVal, path: &str) -> Option<f64> {
    if path == "." || path.is_empty() {
        return scalar_of(json);
    }
    jpath_val(json, path).and_then(scalar_of)
}

fn jcount(json: &JsonVal, path: &str) -> Option<f64> {
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

fn jlast(json: &JsonVal, key: &str) -> Option<f64> {
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

fn jfirst(json: &JsonVal, key: &str) -> Option<f64> {
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

fn jdeep_find_num(json: &JsonVal, key: &str) -> Option<f64> {
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

fn j2d_last_row(json: &JsonVal, col: &str) -> Option<f64> {
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
fn text_last_col(data: &str, col: &str) -> Option<f64> {
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
fn extract_regex_val(body: &str, pat: &str) -> Option<f64> {
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

#[derive(Clone)]
enum Extract {
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
        trk_key: String,
        vr_key: String,
        fields: Vec<FieldConfig>,
        lon_sign: Option<String>,
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
    },
    Rows {
        last_line: bool,
        fields: Vec<FieldConfig>,
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
        fields: Vec<FieldConfig>,
    },
    Hapi(Vec<(String, String)>),
    XmlCount(String, String),
    Ephemeris(String),
    Vectors(String),
}

fn kernel_id_of(name: &str) -> Option<u8> {
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

fn kernel_for_force(force: u8) -> Option<u8> {
    match force {
        0 | 1 => Some(0),
        2 | 3 | 4 | 7 | 8 => Some(1),
        5 | 6 => Some(3),
        _ => None,
    }
}

fn extract_fields(ext: &Extract) -> &[FieldConfig] {
    match ext {
        Extract::Map { fields, .. }
        | Extract::CelestialMap { fields, .. }
        | Extract::Rows { fields, .. }
        | Extract::Flatten { fields, .. }
        | Extract::CmrPolygon { fields, .. }
        | Extract::CelestialPolygon { fields, .. }
        | Extract::KeplerMap { fields, .. } => fields,
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

fn kernel_extent(kernel_id: u8, body_props: Option<&BodyProperties>, tau: f64) -> f64 {
    if tau == 0.0 {
        return 0.0;
    }
    if kernel_id == 0 || kernel_id == 6 {
        return f64::INFINITY;
    }
    let p = match body_props {
        Some(p) => p,
        None => return 0.0,
    };
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

#[derive(Clone)]
struct FieldConfig {
    key: String,
    name: String,
    kernel: u8,
    force: u8,
    tau: f64,
    absorption: f64,
    advection: f64,
}

struct BrowserSensor {
    key: String,
    force: u8,
    kernel: u8,
    ttl: f64,
}

fn force_id_of(name: &str) -> Option<u8> {
    match name {
        "em" => Some(0),
        "gravity" => Some(1),
        "acoustic" => Some(2),
        "seismic-body" => Some(3),
        "seismic-surface" => Some(4),
        "thermal" => Some(5),
        "diffusion" => Some(6),
        "advective" => Some(7),
        "electric" => Some(8),
        _ => None,
    }
}

fn sensor_config(name: &str) -> Option<BrowserSensor> {
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

#[derive(Clone)]
enum Frame {
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

struct StationEntry {
    id: String,
    lat: f64,
    lon: f64,
}

#[derive(Clone)]
struct SourceConfig {
    ttl: u64,
    url: String,
    frame: Frame,
    format: String,
    extracts: Vec<Extract>,
    headers: Vec<(String, String)>,
    post_body: Option<String>,
    target: Option<String>,
    catalog: Option<String>,
    max_freq: Option<f64>,
    min_freq: Option<f64>,
    body: Option<String>,
    stations_url: Option<String>,
    stations_path: String,
    stations_lat: String,
    stations_lon: String,
    stations_id: String,
    flux_from_mag: Option<String>,
    abs_mag_from: Option<String>,
    catalog_epoch: Option<f64>,
    repeat_ra_bins: u32,
    fanout_cap: u32,
    stations_flatten: String,
    stations_filter: Option<(String, String)>,
    fanout_delay: u64,
}

struct FetchResult {
    source_idx: usize,
    channels: Vec<(Channel, FieldConfig)>,
    eph_update: Option<(String, BodyEphemeris)>,
    asteroid_hash: Option<Arc<AsteroidHash>>,
    star_hash: Option<Arc<StarHash>>,
}

struct WsConfig {
    bodies: Arc<Vec<String>>,
    index_html: Vec<u8>,
    constants_js: Vec<u8>,
    field_rx: mpsc::Receiver<Arc<Buffer>>,
    osc_tx: mpsc::Sender<Vec<Oscillator>>,
    presence_tx: mpsc::Sender<(f64, f64, f64, f64, f64)>,
    time: Arc<Mutex<Option<LeapSeconds>>>,
}

trait Radiator: Send + Sync {
    fn accept(&mut self, field: Arc<Buffer>);
}

struct TcpRadiator {
    shutdown: Arc<AtomicBool>,
    field_tx: mpsc::Sender<Arc<Buffer>>,
    _thread: Option<thread::JoinHandle<()>>,
}

impl TcpRadiator {
    fn new(
        port: u16,
        bodies: Arc<Vec<String>>,
        initial_field: Arc<Buffer>,
        index_html: Vec<u8>,
        constants_js: Vec<u8>,
        osc_tx: mpsc::Sender<Vec<Oscillator>>,
        presence_tx: mpsc::Sender<(f64, f64, f64, f64, f64)>,
        time: Arc<Mutex<Option<LeapSeconds>>>,
    ) -> Self {
        let (field_tx, field_rx) = mpsc::channel::<Arc<Buffer>>();
        let listener = match TcpListener::bind(format!("127.0.0.1:{}", port)) {
            Ok(l) => {
                eprintln!("serving on http://127.0.0.1:{}", port);
                l
            }
            Err(e) => {
                eprintln!("TCP bind to 127.0.0.1:{} returned {:?}", port, e.kind());
                std::process::exit(1);
            }
        };
        match listener.set_nonblocking(true) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("TCP set_nonblocking returned {:?}", e.kind());
                std::process::exit(1);
            }
        }
        let shutdown = Arc::new(AtomicBool::new(false));
        let shutdown_clone = shutdown.clone();
        let handle = thread::spawn(move || {
            let mut field_txs: Vec<mpsc::SyncSender<Arc<Buffer>>> = Vec::new();
            let mut latest: Arc<Buffer> = initial_field;
            loop {
                if shutdown_clone.load(Ordering::Relaxed) {
                    break;
                }
                while let Ok((stream, _)) = listener.accept() {
                    let (ftx, frx) = mpsc::sync_channel::<Arc<Buffer>>(2);
                    let _ = ftx.try_send(latest.clone());
                    field_txs.push(ftx);
                    let cfg = WsConfig {
                        bodies: bodies.clone(),
                        index_html: index_html.clone(),
                        constants_js: constants_js.clone(),
                        field_rx: frx,
                        osc_tx: osc_tx.clone(),
                        presence_tx: presence_tx.clone(),
                        time: time.clone(),
                    };
                    thread::spawn(move || handle_ingress(stream, cfg));
                }
                match field_rx.recv_timeout(std::time::Duration::from_secs_f64(2f64.powi(-8))) {
                    Ok(field) => {
                        latest = field;
                        field_txs.retain(|tx| match tx.try_send(latest.clone()) {
                            Ok(_) => true,
                            Err(mpsc::TrySendError::Full(_)) => true,
                            Err(mpsc::TrySendError::Disconnected(_)) => false,
                        });
                    }
                    Err(mpsc::RecvTimeoutError::Timeout) => {}
                    Err(mpsc::RecvTimeoutError::Disconnected) => break,
                }
            }
        });
        Self {
            shutdown,
            field_tx,
            _thread: Some(handle),
        }
    }
}

impl Radiator for TcpRadiator {
    fn accept(&mut self, field: Arc<Buffer>) {
        let _ = self.field_tx.send(field);
    }
}

impl Drop for TcpRadiator {
    fn drop(&mut self) {
        self.shutdown.store(true, Ordering::SeqCst);
    }
}
const AUDIO_SAMPLE_RATE: u32 = 44100;

struct AudioRadiator {
    sample_rate: u32,
}

impl AudioRadiator {
    fn new(sample_rate: u32) -> Self {
        Self { sample_rate }
    }
}

impl Radiator for AudioRadiator {
    fn accept(&mut self, field: Arc<Buffer>) {
        if std::io::stdout().is_terminal() {
            return;
        }
        let mut out = std::io::stdout();
        let mut samples: Vec<f32> = Vec::new();
        for hash in field
            .bodies
            .values()
            .chain(std::iter::once(&field.inertial))
        {
            for v in hash.cells.values().chain(std::iter::once(&hash.unbounded)) {
                for s in v {
                    if !s.tau.is_finite() {
                        continue;
                    }
                    let amp = (s.val as f32).clamp(0.0, 1.0);
                    let dur = (s.tau as f32 * self.sample_rate as f32).max(1.0) as u32;
                    let freq = SONIFICATION_ROOT_FREQ
                        + s.kernel_id as f32 * SONIFICATION_KERNEL_STEP
                        + s.force_type as f32 * SONIFICATION_FORCE_STEP;
                    let tau_samples = (s.tau as f32 * self.sample_rate as f32) as u32;
                    let envelope_samples = if tau_samples > 0 {
                        tau_samples.min(dur)
                    } else {
                        dur
                    };
                    for t in 0..dur {
                        let phase =
                            2.0 * std::f32::consts::PI * freq * t as f32 / self.sample_rate as f32;
                        let decay = (-(t as f32) / envelope_samples as f32).exp();
                        samples.push(amp * phase.sin() * decay);
                    }
                }
            }
        }
        if !samples.is_empty() {
            let mut bytes = Vec::with_capacity(samples.len() * 4);
            for s in &samples {
                bytes.extend_from_slice(&s.to_le_bytes());
            }
            let _ = out.write_all(&bytes);
            let _ = out.flush();
        }
    }
}

struct StderrRadiator {
    last_line: String,
    interactive: bool,
}

impl Radiator for StderrRadiator {
    fn accept(&mut self, field: Arc<Buffer>) {
        let mut body_osc = 0usize;
        let mut api_osc = 0usize;
        let mut dev_osc = 0usize;
        let mut body_src: std::collections::HashSet<String> = std::collections::HashSet::new();
        let mut api_src: std::collections::HashSet<u32> = std::collections::HashSet::new();
        for (_body_name, hash) in &field.bodies {
            for cell in hash.cells.values().chain(std::iter::once(&hash.unbounded)) {
                for osc in cell {
                    match osc.source {
                        OscillatorSource::Device => dev_osc += 1,
                        OscillatorSource::Api(idx) => {
                            api_osc += 1;
                            api_src.insert(idx);
                        }
                        OscillatorSource::Body => {
                            body_osc += 1;
                            body_src.insert(osc.name.split('.').next().unwrap_or("").to_string());
                        }
                    }
                }
            }
        }
        for cell in field
            .inertial
            .cells
            .values()
            .chain(std::iter::once(&field.inertial.unbounded))
        {
            for osc in cell {
                match osc.source {
                    OscillatorSource::Device => dev_osc += 1,
                    OscillatorSource::Api(idx) => {
                        api_osc += 1;
                        api_src.insert(idx);
                    }
                    OscillatorSource::Body => {
                        body_osc += 1;
                        body_src.insert(osc.name.split('.').next().unwrap_or("").to_string());
                    }
                }
            }
        }
        let line = format!(
            "omegaflow v{} | φ v6 | body: {} sources, {} oscillators | api: {} sources, {} oscillators | device: {} oscillators",
            env!("CARGO_PKG_VERSION"),
            body_src.len(),
            body_osc,
            api_src.len(),
            api_osc,
            dev_osc,
        );
        if line == self.last_line {
            return;
        }
        let prev_len = self.last_line.chars().count();
        if self.interactive {
            let pad = " ".repeat(prev_len.saturating_sub(line.chars().count()));
            eprint!("\r{}{}\r", line, pad);
        } else {
            eprintln!("{}", line);
        }
        self.last_line = line;
    }
}

struct Archive {
    sources: Vec<SourceConfig>,
    body_ephemerides: Arc<HashMap<String, BodyEphemeris>>,
    field: Arc<Buffer>,
    presence: HashMap<String, (f64, f64, f64, f64, f64)>,
    origins: HashMap<Origin, OriginState>,
    pck_bodies: HashMap<i32, PckBody>,
    time: Arc<Mutex<Option<LeapSeconds>>>,
    asteroids: Option<Arc<AsteroidHash>>,
    stars: Option<Arc<StarHash>>,
}
struct WsFrame {
    opcode: u8,
    payload: Vec<u8>,
}

fn base64_encode(data: &[u8]) -> String {
    const T: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut r = String::new();
    for c in data.chunks(3) {
        r.push(T[(c[0] >> 2) as usize] as char);
        if c.len() == 1 {
            r.push(T[((c[0] & 0x03) << 4) as usize] as char);
            r.push('=');
            r.push('=');
        } else {
            r.push(T[(((c[0] & 0x03) << 4) | (c[1] >> 4)) as usize] as char);
            if c.len() == 2 {
                r.push(T[((c[1] & 0x0f) << 2) as usize] as char);
                r.push('=');
            } else {
                r.push(T[(((c[1] & 0x0f) << 2) | (c[2] >> 6)) as usize] as char);
                r.push(T[(c[2] & 0x3f) as usize] as char);
            }
        }
    }
    r
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

fn emit(s: &mut TcpStream, st: &str, ct: &str, b: &[u8]) {
    let _=s.write_all(format!("HTTP/1.1 {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nCache-Control: no-store, no-cache, must-revalidate\r\nPragma: no-cache\r\nExpires: 0\r\nConnection: keep-alive\r\n\r\n",st,ct,b.len()).as_bytes());
    let _ = s.write_all(b);
}
fn emit_void(s: &mut TcpStream) {
    let _ =
        s.write_all(b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n");
}
fn extract_header(s: &str, n: &str) -> Option<String> {
    for l in s.lines() {
        if let Some(c) = l.find(':') {
            if l[..c].trim().eq_ignore_ascii_case(n) {
                return Some(l[c + 1..].trim().to_string());
            }
        }
    }
    None
}

fn fetch_raw(
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
            "fetch returned ({}): {} {}",
            output.status,
            url,
            stderr.trim()
        );
        None
    }
}

fn fetch_raw_bytes(url: &str, ttl: u64) -> Option<Vec<u8>> {
    let connect_t = ((ttl as f64) / (Φ * Φ * Φ)).ceil() as u64;
    let max_t = ((ttl as f64) / (Φ * Φ)).ceil() as u64;
    let mut cmd = Command::new("curl");
    cmd.arg("-s")
        .arg("-S")
        .arg("-f")
        .arg("-L")
        .arg("--retry")
        .arg("5")
        .arg("--retry-delay")
        .arg("2")
        .arg("--http1.1")
        .arg("-m")
        .arg(max_t.to_string())
        .arg("--connect-timeout")
        .arg(connect_t.to_string());
    cmd.arg(url);
    let output = cmd.output().ok()?;
    if output.status.success() {
        Some(output.stdout)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!(
            "fetch_bytes returned ({}): {} {}",
            output.status,
            url,
            stderr.trim()
        );
        None
    }
}

fn fetch_raw_probe(url: &str, body: Option<&str>, headers: &[(String, String)]) -> Option<String> {
    let mut cmd = Command::new("curl");
    cmd.arg("-s")
        .arg("-S")
        .arg("-f")
        .arg("-L")
        .arg("--retry")
        .arg("1")
        .arg("--retry-delay")
        .arg("1")
        .arg("--http1.1")
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

fn fetch_raw_bytes_post(
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
        .arg("--retry-delay")
        .arg("2")
        .arg("--http1.1")
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
    now.saturating_sub(asset_ts) < ttl
}

fn fetch_one(
    url: &str,
    body: Option<&str>,
    headers: &[(String, String)],
    ttl: u64,
) -> Option<String> {
    if !url.starts_with("https://github.com/omegaflow/sources") {
        if let Some(netloc) = extract_netloc(url) {
            let name = source_name_from_url(url);
            if !name.is_empty() {
                let cache_path = format!("/tmp/archivar_cache/{}/{}.json", netloc, name);
                if let Some(cached) = read_cache_if_fresh(&cache_path, ttl) {
                    return Some(cached);
                }
                let cdn_url = format!(
                    "https://github.com/omegaflow/sources/releases/download/{}/{}.json",
                    netloc, name
                );
                if cdn_fresh(&cdn_url, ttl) {
                    if let Some(cdn_body) = fetch_raw(&cdn_url, None, &[], ttl) {
                        if let Some(parent) = std::path::Path::new(&cache_path).parent() {
                            let _ = std::fs::create_dir_all(parent);
                        }
                        let _ = std::fs::write(&cache_path, cdn_body.as_bytes());
                        return Some(cdn_body);
                    }
                }
            }
        }
    }
    let live = fetch_raw(url, body, headers, ttl);
    if let Some(ref r) = live {
        if let Some(netloc) = extract_netloc(url) {
            let name = source_name_from_url(url);
            if !name.is_empty() {
                let cache_path = format!("/tmp/archivar_cache/{}/{}.json", netloc, name);
                if let Some(parent) = std::path::Path::new(&cache_path).parent() {
                    let _ = std::fs::create_dir_all(parent);
                }
                let _ = std::fs::write(&cache_path, r.as_bytes());
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

fn read_cache_if_fresh(path: &str, ttl: u64) -> Option<String> {
    if cache_fresh(path, ttl) {
        std::fs::read_to_string(path).ok()
    } else {
        None
    }
}

fn handle_ingress(stream: TcpStream, cfg: WsConfig) {
    let mut s = stream;
    s.set_nodelay(true).ok();
    let signal = match read_signal(&mut s) {
        Some(r) => r,
        None => return,
    };
    if signal.to_lowercase().contains("upgrade: websocket") {
        resonance(s, &signal, cfg);
    } else {
        let mut last_field: Option<Arc<Buffer>> = None;
        let mut cur = signal;
        loop {
            let path = parse_path(&cur);
            match path.as_str() {
                "/" => {
                    let page = match std::fs::read(resolve_asset("static/index.html")) {
                        Ok(v) => v,
                        Err(_) => cfg.index_html.clone(),
                    };
                    emit(&mut s, "200 OK", "text/html", &page);
                }
                "/time" => match system_now(&cfg.time) {
                    Some(tdb) => {
                        emit(&mut s, "200 OK", "text/plain", tdb.to_string().as_bytes());
                    }
                    None => {
                        emit_void(&mut s);
                        break;
                    }
                },
                "/device" => {
                    let now = match system_now(&cfg.time) {
                        Some(t) => t,
                        None => {
                            emit_void(&mut s);
                            break;
                        }
                    };
                    let result = {
                        let buf = {
                            if let Ok(f) = cfg.field_rx.try_recv() {
                                last_field = Some(f);
                            }
                            last_field.clone().unwrap_or_else(|| {
                                Arc::new(build_buffer(
                                    Vec::new(),
                                    1.0,
                                    Arc::new(HashMap::new()),
                                    None,
                                    None,
                                ))
                            })
                        };
                        let eph_map = buf.eph.clone();
                        let mut device_sample: Option<Oscillator> = None;
                        for hash in buf.bodies.values().chain(std::iter::once(&buf.inertial)) {
                            for v in hash.cells.values().chain(std::iter::once(&hash.unbounded)) {
                                for osc in v {
                                    if matches!(osc.source, OscillatorSource::Device) {
                                        let newer = match &device_sample {
                                            Some(cur) => osc.epoch > cur.epoch,
                                            None => true,
                                        };
                                        if newer {
                                            device_sample = Some(osc.clone());
                                        }
                                    }
                                }
                            }
                        }
                        device_sample.and_then(|osc| {
                            let p0 = osc.motion.at(now, osc.epoch, &eph_map)?;
                            let p1 = osc.motion.at(now + 1.0, osc.epoch, &eph_map)?;
                            Some((p0, [p1[0] - p0[0], p1[1] - p0[1], p1[2] - p0[2]]))
                        })
                    };
                    match result {
                        Some((p, v)) => emit(
                            &mut s,
                            "200 OK",
                            "text/plain",
                            format!("{} {} {} {} {} {}", p[0], p[1], p[2], v[0], v[1], v[2])
                                .as_bytes(),
                        ),
                        None => {
                            emit_void(&mut s);
                            break;
                        }
                    }
                }
                _ if path.starts_with("/jump/") => {
                    let body: &str = &path[6..];
                    let eph = {
                        if let Ok(f) = cfg.field_rx.try_recv() {
                            last_field = Some(f);
                        }
                        last_field
                            .as_ref()
                            .map(|b| b.eph.clone())
                            .unwrap_or_else(|| Arc::new(HashMap::new()))
                    };
                    let now = match system_now(&cfg.time) {
                        Some(t) => t,
                        None => {
                            emit_void(&mut s);
                            break;
                        }
                    };
                    match body_barycenter_position(body, now, &eph) {
                        Some([x, y, z]) => emit(
                            &mut s,
                            "200 OK",
                            "text/plain",
                            format!("{} {} {}", x, y, z).as_bytes(),
                        ),
                        None => {
                            emit_void(&mut s);
                            break;
                        }
                    }
                }
                "/field" => {
                    let buf = {
                        if let Ok(f) = cfg.field_rx.try_recv() {
                            last_field = Some(f);
                        }
                        last_field.clone().unwrap_or_else(|| {
                            Arc::new(build_buffer(
                                Vec::new(),
                                1.0,
                                Arc::new(HashMap::new()),
                                None,
                                None,
                            ))
                        })
                    };
                    let mut report = String::new();
                    let mut hashes: Vec<(&str, &SpatialHash)> =
                        buf.bodies.iter().map(|(k, v)| (k.as_str(), v)).collect();
                    hashes.push(("inertial", &buf.inertial));
                    for (fname, hash) in hashes {
                        let mut n = 0usize;
                        let mut field_names: std::collections::HashSet<&str> =
                            std::collections::HashSet::new();
                        let mut src_ids: std::collections::HashSet<u32> =
                            std::collections::HashSet::new();
                        for v in hash.cells.values().chain(std::iter::once(&hash.unbounded)) {
                            for osc in v {
                                n += 1;
                                field_names.insert(osc.name.as_str());
                                match osc.source {
                                    OscillatorSource::Api(id) => {
                                        src_ids.insert(id);
                                    }
                                    _ => {}
                                };
                            }
                        }
                        report.push_str(&format!(
                            "{} samples={} cells={} unbounded={} rmax={:.3e} vmax={:.3e} epoch_min={:.1} origins={}\n",
                            fname,
                            n,
                            hash.cells.len(),
                            hash.unbounded.len(),
                            hash.rmax,
                            hash.vmax,
                            hash.epoch_min,
                            src_ids.len()
                        ));
                        let mut names: Vec<&str> = field_names.into_iter().collect();
                        names.sort();
                        report.push_str(&format!("{} fields: {}\n", fname, names.len()));
                        for nm in names {
                            report.push_str(&format!("  {}\n", nm));
                        }
                    }
                    report.push_str(&format!("ephemerides={}\n", buf.eph.len()));
                    emit(&mut s, "200 OK", "text/plain", report.as_bytes());
                }
                "/constants.js" => {
                    let mut page = match std::fs::read(resolve_asset("static/constants.js")) {
                        Ok(v) => v,
                        Err(_) => cfg.constants_js.clone(),
                    };
                    let mut extra = String::from("\nexport const BODY_REGISTRY = {");
                    for (i, name) in cfg.bodies.iter().enumerate() {
                        extra.push_str(&format!("{}:\"{}\",", i + 1, name));
                    }
                    extra.push_str("};\n");
                    page.extend_from_slice(extra.as_bytes());
                    emit(&mut s, "200 OK", "application/javascript", &page);
                }
                "/crash" => {
                    emit(&mut s, "200 OK", "text/plain", &[]);
                    break;
                }
                _ => {
                    emit_void(&mut s);
                    break;
                }
            }
            match read_signal(&mut s) {
                Some(r) => cur = r,
                None => break,
            }
        }
    }
}

fn resonance(mut stream: TcpStream, signal: &str, cfg: WsConfig) {
    let key = match extract_header(signal, "Sec-WebSocket-Key") {
        Some(k) => k,
        None => return,
    };
    let encoded = base64_encode(&sha1(
        &format!("{}{}", key, "258EAFA5-E914-47DA-95CA-C5AB0DC85B11").into_bytes(),
    ));
    if stream.write_all(format!("HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Accept: {}\r\n\r\n", encoded).as_bytes()).is_err() { return; }
    let mut last_field_r: Option<Arc<Buffer>> = None;
    let _ = stream.set_nodelay(true);
    while let Some(frame) = read_ws_frame_raw(&mut stream) {
        if frame.opcode == 0x8 {
            break;
        }
        if frame.opcode == 0x9 {
            let mut h = [0u8; 2];
            h[0] = 0x8A;
            h[1] = frame.payload.len() as u8;
            if stream.write_all(&h).is_err() {
                break;
            }
            if stream.write_all(&frame.payload).is_err() {
                break;
            }
            continue;
        }
        if frame.opcode == 0x2 {
            if frame.payload.len() < 12 {
                continue;
            }

            let mut cursor = Cursor::new(&frame.payload);
            let mut buf4 = [0u8; 4];

            if cursor.read_exact(&mut buf4).is_err() {
                continue;
            }
            let id = u32::from_le_bytes(buf4);
            if cursor.read_exact(&mut buf4).is_err() {
                continue;
            }
            let oscillator_count = u32::from_le_bytes(buf4) as usize;

            let mut browser: Vec<(String, f64, f64)> = Vec::with_capacity(oscillator_count);
            {
                for _ in 0..oscillator_count {
                    let mut val_buf = [0u8; 8];
                    if cursor.read_exact(&mut val_buf).is_err() {
                        break;
                    }
                    let value = f64::from_le_bytes(val_buf);

                    let mut name_len_buf = [0u8; 1];
                    if cursor.read_exact(&mut name_len_buf).is_err() {
                        break;
                    }
                    let name_len = name_len_buf[0] as usize;
                    let mut name_bytes = vec![0u8; name_len];
                    if cursor.read_exact(&mut name_bytes).is_err() {
                        break;
                    }
                    let name = String::from_utf8_lossy(&name_bytes).to_string();
                    let mut tau_buf = [0u8; 8];
                    let tau = if cursor.read_exact(&mut tau_buf).is_ok() {
                        f64::from_le_bytes(tau_buf)
                    } else {
                        0.0
                    };

                    browser.push((name, value, tau));
                }
            }

            if cursor.read_exact(&mut buf4).is_err() {
                continue;
            }
            let query_count = u32::from_le_bytes(buf4) as usize;
            let mut queries: Vec<(f64, f64, f64, f64)> = Vec::with_capacity(query_count);
            for _ in 0..query_count {
                let mut t_buf = [0u8; 8];
                if cursor.read_exact(&mut t_buf).is_err() {
                    break;
                }
                let qt = f64::from_le_bytes(t_buf);
                if cursor.read_exact(&mut t_buf).is_err() {
                    break;
                }
                let qx = f64::from_le_bytes(t_buf);
                if cursor.read_exact(&mut t_buf).is_err() {
                    break;
                }
                let qy = f64::from_le_bytes(t_buf);
                if cursor.read_exact(&mut t_buf).is_err() {
                    break;
                }
                let qz = f64::from_le_bytes(t_buf);
                queries.push((qt, qx, qy, qz));
            }
            let mut delta_t_cache = 0.0f64;
            {
                let mut pb8 = [0u8; 8];
                if cursor.read_exact(&mut pb8).is_ok() {
                    let px = f64::from_le_bytes(pb8);
                    if cursor.read_exact(&mut pb8).is_ok() {
                        let py = f64::from_le_bytes(pb8);
                        if cursor.read_exact(&mut pb8).is_ok() {
                            let pz = f64::from_le_bytes(pb8);
                            if cursor.read_exact(&mut pb8).is_ok() {
                                let pt = f64::from_le_bytes(pb8);
                                if cursor.read_exact(&mut pb8).is_ok() {
                                    let pr = f64::from_le_bytes(pb8);
                                    let _ = cfg.presence_tx.send((pt, px, py, pz, pr));
                                    if cursor.read_exact(&mut pb8).is_ok() {
                                        delta_t_cache = f64::from_le_bytes(pb8);
                                    }
                                }
                            }
                        }
                    }
                }
            }

            let field = {
                if let Ok(f) = cfg.field_rx.try_recv() {
                    last_field_r = Some(f);
                }
                last_field_r.clone().unwrap_or_else(|| {
                    Arc::new(build_buffer(
                        Vec::new(),
                        1.0,
                        Arc::new(HashMap::new()),
                        None,
                        None,
                    ))
                })
            };
            let eph_map = field.eph.clone();
            let now = match system_now(&cfg.time) {
                Some(t) => t,
                None => continue,
            };
            let (mut st_lat, mut st_lon, mut st_alt, mut st_body) = (None, None, None, None::<u32>);
            let mut field_values: Vec<(String, f64, f64)> = Vec::new();
            for (name, value, tau) in &browser {
                let tau = *tau;
                match name.as_str() {
                    "lat" => st_lat = Some(*value),
                    "lon" => st_lon = Some(*value),
                    "alt" => st_alt = Some(*value),
                    "body" => {
                        st_body = if *value > 0.0 && *value <= u32::MAX as f64 {
                            Some(*value as u32)
                        } else {
                            None
                        }
                    }
                    _ => {
                        field_values.push((name.clone(), *value, tau));
                    }
                }
            }
            if let (Some(lat), Some(lon), Some(alt)) = (st_lat, st_lon, st_alt) {
                let body_name = match st_body.and_then(|id| body_id_to_name(&cfg.bodies, id)) {
                    Some(n) => n,
                    None => String::new(),
                };
                if eph_map
                    .get(&body_name)
                    .and_then(|e| e.props.as_ref())
                    .is_some()
                {
                    let pos = Position::Surface {
                        body_name: body_name.clone(),
                        lat,
                        lon,
                        alt,
                    };
                    let mut channels: Vec<(Channel, FieldConfig, f64)> = Vec::new();
                    for (name, value, tau) in &field_values {
                        if let Some(bs) = sensor_config(name) {
                            let sensor_ttl = bs.ttl;
                            let effective_tau = if *tau > 0.0 { *tau } else { bs.ttl };
                            let fc = FieldConfig {
                                key: bs.key.clone(),
                                name: bs.key.clone(),
                                kernel: bs.kernel,
                                force: bs.force,
                                tau: effective_tau,
                                absorption: 0.0,
                                advection: 0.0,
                            };
                            if value.is_finite() {
                                channels.push((
                                    Channel {
                                        epoch: now,
                                        position: pos.clone(),
                                        name: fc.name.clone(),
                                        value: *value,
                                    },
                                    fc,
                                    sensor_ttl,
                                ));
                            }
                        }
                    }
                    let mut oscillators = Vec::new();
                    for (channel, sensor, sensor_ttl) in channels {
                        if let Some(osc) =
                            anchor(&channel, &sensor, sensor_ttl, None, None, None, &eph_map)
                        {
                            oscillators.push(osc);
                        }
                    }
                    if !oscillators.is_empty() {
                        let _ = cfg.osc_tx.send(oscillators);
                    }
                }
            } else if let Some(body_id) = st_body {
                let body_name = match body_id_to_name(&cfg.bodies, body_id) {
                    Some(n) => n,
                    None => String::new(),
                };
                let frame = Frame::Barycenter {
                    body_name: body_name.clone(),
                    scale: 1.0,
                };
                let pos = Position::Source;
                let mut channels: Vec<(Channel, FieldConfig, f64)> = Vec::new();
                for (name, value, tau) in &field_values {
                    if let Some(bs) = sensor_config(name) {
                        let sensor_ttl = bs.ttl;
                        let effective_tau = if *tau > 0.0 { *tau } else { bs.ttl };
                        let fc = FieldConfig {
                            key: bs.key.clone(),
                            name: bs.key.clone(),
                            kernel: bs.kernel,
                            force: bs.force,
                            tau: effective_tau,
                            absorption: 0.0,
                            advection: 0.0,
                        };
                        if value.is_finite() {
                            channels.push((
                                Channel {
                                    epoch: now,
                                    position: pos.clone(),
                                    name: fc.name.clone(),
                                    value: *value,
                                },
                                fc,
                                sensor_ttl,
                            ));
                        }
                    }
                }
                let mut oscillators = Vec::new();
                for (channel, sensor, sensor_ttl) in channels {
                    if let Some(osc) = anchor(
                        &channel,
                        &sensor,
                        sensor_ttl,
                        None,
                        Some(&frame),
                        None,
                        &eph_map,
                    ) {
                        oscillators.push(osc);
                    }
                }
                if !oscillators.is_empty() {
                    let _ = cfg.osc_tx.send(oscillators);
                }
            }
            let mut records: Vec<(
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
            )> = Vec::new();
            let response_epoch;
            if !queries.is_empty() {
                let (t0, x0, y0, z0) = queries[0];
                let mut extent = 0.0f64;
                for &(_, qx, qy, qz) in &queries[1..] {
                    let d = ((qx - x0).powi(2) + (qy - y0).powi(2) + (qz - z0).powi(2)).sqrt();
                    if d > extent {
                        extent = d;
                    }
                }
                let center = [x0, y0, z0];
                sense_buffer(
                    &field,
                    center,
                    t0,
                    extent,
                    delta_t_cache,
                    &mut records,
                    &eph_map,
                );
                response_epoch = t0;
            } else {
                response_epoch = now;
            }

            let mut out = Vec::with_capacity(19 + records.len() * 168);
            out.extend_from_slice(&[0xCF, 0x86]);
            out.push(6u8);
            out.extend_from_slice(&response_epoch.to_le_bytes());
            out.extend_from_slice(&id.to_le_bytes());
            out.extend_from_slice(&(records.len() as u32).to_le_bytes());
            for &(
                x,
                y,
                z,
                val,
                epoch,
                ttl,
                tau,
                extent,
                kernel_id,
                force_type,
                absorption,
                advection,
                vx,
                vy,
                vz,
                pole_x,
                pole_y,
                pole_z,
                j2,
                j4,
                r_eq,
            ) in &records
            {
                out.extend_from_slice(&x.to_le_bytes());
                out.extend_from_slice(&y.to_le_bytes());
                out.extend_from_slice(&z.to_le_bytes());
                out.extend_from_slice(&val.to_le_bytes());
                out.extend_from_slice(&epoch.to_le_bytes());
                out.extend_from_slice(&ttl.to_le_bytes());
                out.extend_from_slice(&tau.to_le_bytes());
                out.extend_from_slice(&extent.to_le_bytes());
                out.extend_from_slice(&kernel_id.to_le_bytes());
                out.extend_from_slice(&force_type.to_le_bytes());
                out.extend_from_slice(&absorption.to_le_bytes());
                out.extend_from_slice(&advection.to_le_bytes());
                out.extend_from_slice(&vx.to_le_bytes());
                out.extend_from_slice(&vy.to_le_bytes());
                out.extend_from_slice(&vz.to_le_bytes());
                out.extend_from_slice(&pole_x.to_le_bytes());
                out.extend_from_slice(&pole_y.to_le_bytes());
                out.extend_from_slice(&pole_z.to_le_bytes());
                out.extend_from_slice(&j2.to_le_bytes());
                out.extend_from_slice(&j4.to_le_bytes());
                out.extend_from_slice(&r_eq.to_le_bytes());
            }
            write_ws_binary(&mut stream, &out);
        }
    }
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

fn render_headers(
    headers: &[(String, String)],
    env: &HashMap<String, String>,
) -> Vec<(String, String)> {
    headers
        .iter()
        .map(|(k, v)| (k.clone(), resolve_secret(v, env)))
        .collect()
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

fn load_sources() -> Vec<SourceConfig> {
    let content = match std::fs::read_to_string("phi/sources.φ") {
        Ok(c) => c,
        Err(_) => return Vec::new(),
    };
    parse_sources(&content)
}

fn parse_sources(content: &str) -> Vec<SourceConfig> {
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
                    Err(_) => continue,
                };
                let lon: f64 = match parts[3].parse() {
                    Ok(v) => v,
                    Err(_) => continue,
                };
                let alt: f64 = match parts.get(4) {
                    Some(s) => match s.parse() {
                        Ok(v) => v,
                        Err(_) => continue,
                    },
                    None => 0.0,
                };
                cur_frame = Some(Frame::Surface {
                    body_name: body,
                    lat,
                    lon,
                    alt,
                });
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
                    trk_key: String::new(),
                    vr_key: String::new(),
                    fields: Vec::new(),
                    lon_sign: None,
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
                });
            }
            "rows" => {
                cur_extracts.push(Extract::Rows {
                    last_line: false,
                    fields: Vec::new(),
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
            "xmlcount" if parts.len() >= 3 => {
                cur_extracts.push(Extract::XmlCount(
                    parts[1].to_string(),
                    parts[2].to_string(),
                ));
            }
            "ephemeris" if parts.len() >= 2 => {
                cur_extracts.push(Extract::Ephemeris(parts[1].to_string()));
            }
            "vectors" if parts.len() >= 2 => {
                cur_extracts.push(Extract::Vectors(parts[1].to_string()));
            }
            "field" if parts.len() == 6 => {
                let f = match force_id_of(parts[2]) {
                    Some(f) => f,
                    None => continue,
                };
                let tau: f64 = match parts[4].parse() {
                    Ok(v) if v > 0.0 => v,
                    _ => continue,
                };
                let k = match kernel_id_of(parts[5]) {
                    Some(k) => k,
                    None => match kernel_for_force(f) {
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
                    None => continue,
                };
                let tau: f64 = match parts[4].parse() {
                    Ok(v) if v > 0.0 => v,
                    _ => continue,
                };
                let fc = FieldConfig {
                    key: parts[1].to_string(),
                    name: parts[1].to_string(),
                    kernel: match kernel_for_force(f) {
                        Some(k) => k,
                        None => continue,
                    },
                    force: f,
                    tau,
                    absorption: 0.0,
                    advection: 0.0,
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
            "field" if parts.len() >= 9 => {
                let k = match kernel_id_of(parts[3]) {
                    Some(k) => k,
                    None => continue,
                };
                let f = match force_id_of(parts[4]) {
                    Some(f) => f,
                    None => continue,
                };
                let tau: f64 = match parts[6].parse() {
                    Ok(v) if v > 0.0 => v,
                    _ => continue,
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
                } else {
                    cur_extracts.push(Extract::Field(fc.clone()));
                }
            }
            "lat" if parts.len() >= 2 => {
                if let Some(Extract::Map { lat_key, .. }) = cur_extracts.last_mut() {
                    *lat_key = parts[1].to_string();
                }
            }
            "lon" if parts.len() >= 2 => {
                if let Some(Extract::Map { lon_key, .. }) = cur_extracts.last_mut() {
                    *lon_key = parts[1].to_string();
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
                    Some(_) => continue,
                };
                if let Some(Extract::Map {
                    alt_key, alt_scale, ..
                }) = cur_extracts.last_mut()
                {
                    *alt_key = parts[1].to_string();
                    *alt_scale = scale;
                }
            }
            "epoch" if parts.len() >= 2 => {
                if let Some(Extract::Map { epoch_key, .. }) = cur_extracts.last_mut() {
                    *epoch_key = parts[1].to_string();
                }
            }
            "vel" if parts.len() >= 2 => {
                if let Some(Extract::Map { vel_key, .. }) = cur_extracts.last_mut() {
                    *vel_key = parts[1].to_string();
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
            "format" if parts.len() >= 2 => cur_format = parts[1].to_string(),
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

fn parse_path(s: &str) -> String {
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

fn read_signal(s: &mut TcpStream) -> Option<String> {
    let mut buf = [0u8; 8192];
    let mut acc = Vec::new();
    loop {
        match s.read(&mut buf) {
            Ok(0) => return None,
            Ok(n) => {
                acc.extend_from_slice(&buf[..n]);
                if acc.windows(4).any(|w| w == b"\r\n\r\n") {
                    return Some(String::from_utf8_lossy(&acc).to_string());
                }
                if acc.len() > 65536 {
                    return None;
                }
            }
            Err(_) => return None,
        }
    }
}

fn read_ws_frame_part(stream: &mut TcpStream) -> Option<(u8, bool, Vec<u8>)> {
    let mut header = [0u8; 2];
    stream.read_exact(&mut header).ok()?;
    let fin = (header[0] & 0x80) != 0;
    let opcode = header[0] & 0x0f;
    let masked = (header[1] & 0x80) != 0;
    let mut plen = (header[1] & 0x7f) as usize;
    if plen == 126 {
        let mut e = [0u8; 2];
        stream.read_exact(&mut e).ok()?;
        plen = u16::from_be_bytes(e) as usize;
    } else if plen == 127 {
        let mut e = [0u8; 8];
        stream.read_exact(&mut e).ok()?;
        plen = u64::from_be_bytes(e) as usize;
    }
    if plen > 1 << 24 {
        return None;
    }
    let mut mk = [0u8; 4];
    if masked {
        stream.read_exact(&mut mk).ok()?;
    }
    let mut payload = vec![0u8; plen];
    stream.read_exact(&mut payload).ok()?;
    if masked {
        for i in 0..payload.len() {
            payload[i] ^= mk[i % 4];
        }
    }
    Some((opcode, fin, payload))
}

fn read_ws_frame_raw(stream: &mut TcpStream) -> Option<WsFrame> {
    let (opcode, mut fin, mut payload) = read_ws_frame_part(stream)?;
    while !fin {
        let (next_op, next_fin, mut next_payload) = read_ws_frame_part(stream)?;
        match next_op {
            0x0 => {
                if payload.len() + next_payload.len() > 1 << 24 {
                    return None;
                }
                payload.append(&mut next_payload);
                fin = next_fin;
            }
            0x8 => {
                return Some(WsFrame {
                    opcode: 0x8,
                    payload: next_payload,
                });
            }
            _ => {}
        }
    }
    Some(WsFrame { opcode, payload })
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

    let mut url = template
        .replace("{x}", &format!("{}", x))
        .replace("{y}", &format!("{}", y))
        .replace("{z}", &format!("{}", z))
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
            let stations = if let Some(body) = fetch_one(st_url, None, &[], 86400) {
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

fn gold_kernel_for(force: &str) -> Option<(&'static str, &'static str)> {
    match force {
        "em" => Some(("inverse-square", "em")),
        "gravity" => Some(("inverse-square", "gravity")),
        "acoustic" => Some(("gaussian-inverse-square", "acoustic")),
        "seismic-body" => Some(("gaussian-inverse-square", "seismic-body")),
        "seismic-surface" => Some(("erfc", "seismic-surface")),
        "thermal" => Some(("exponential-decay", "thermal")),
        "diffusion" => Some(("gaussian-inverse-square", "diffusion")),
        "advective" => Some(("patch-levy", "advective")),
        _ => None,
    }
}

fn gold_synth(directive: &str, force: &str, key: &str, name: &str, ttl: u64) -> Option<String> {
    let (kernel, f) = gold_kernel_for(force)?;
    let tau = if ttl > 0 {
        ((ttl as f64) / 10.0).max(1.0)
    } else {
        1.0
    };
    Some(format!(
        "{} {} {} {} {} 1 {} 0.0 0.0\n",
        directive, key, name, kernel, f, tau
    ))
}

fn convert_gold_block(block: &str) -> String {
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
            "method" => {}
            "body" if parts.len() >= 2 => {
                if parts[1].starts_with('{') {
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
    } else if lat.is_some() && lon.is_some() {
        out.push_str(&format!(
            "on earth {} {} {}\n",
            lat.unwrap_or(0.0),
            lon.unwrap_or(0.0),
            alt.unwrap_or(0.0)
        ));
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
                gold_synth("field", &force, parts[1], parts[2], ttl)
            }
            "first" | "last" | "count" | "path" | "deep" if parts.len() >= 3 => {
                gold_synth(parts[0], &force, parts[1], parts[2], ttl)
            }
            "last_row" if parts.len() >= 3 => {
                gold_synth("lastrow", &force, parts[1], parts[2], ttl)
            }
            "last_line" if parts.len() >= 2 => Some(format!("lastline {}", parts[1])),
            "last_obj" if parts.len() >= 5 => {
                let name = parts[parts.len() - 1];
                let key = parts[parts.len() - 2];
                let parent = parts[1];
                let m = parts[2..parts.len() - 2].join(" ");
                Some(format!("lastobj {} {} {} {}", parent, m, key, name))
            }
            "geojson" if parts.len() >= 5 => {
                let tau = if ttl > 0 {
                    ((ttl as f64) / 10.0).max(1.0)
                } else {
                    1.0
                };
                Some(format!(
                    "geojson {} 0.0 {} {} {} 0.0 0.0",
                    parts[2], parts[3], parts[4], tau
                ))
            }
            "regex" if parts.len() >= 3 => {
                let name = parts[parts.len() - 1];
                let pat = parts[1..parts.len() - 1].join(" ");
                gold_synth("regex", &force, &pat, name, ttl)
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

fn gold_flush_block(block: &str, converted: &mut String, total: &mut usize, parsed: &mut usize) {
    *total += 1;
    let conv = convert_gold_block(block);
    if !parse_sources(&conv).is_empty() {
        *parsed += 1;
        converted.push_str(&conv);
        converted.push('\n');
    }
}

fn gold_convert_mode(input: &str, output: &str) -> i32 {
    let content = match std::fs::read_to_string(input) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("--gold: input unreadable: {}", input);
            return 1;
        }
    };
    let mut converted = String::from("# gold conversion (old → new grammar, mechanical)\n");
    let mut block = String::new();
    let mut total = 0usize;
    let mut parsed = 0usize;
    let mut in_source = false;
    for line in content.lines() {
        let t = line.trim_start();
        if t.starts_with("source ") {
            if !block.is_empty() {
                gold_flush_block(&block, &mut converted, &mut total, &mut parsed);
                block = String::new();
            }
            in_source = true;
            block.push_str(line);
            block.push('\n');
            continue;
        }
        if t.starts_with("url ") && !in_source {
            if !block.is_empty() {
                gold_flush_block(&block, &mut converted, &mut total, &mut parsed);
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
        gold_flush_block(&block, &mut converted, &mut total, &mut parsed);
    }
    if std::fs::write(output, &converted).is_err() {
        eprintln!("--gold: output unwritable: {}", output);
        return 1;
    }
    eprintln!(
        "--gold: {} blocks converted, {} parse in the current parser → {}",
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
                            None => return Vec::new(),
                        };
                        let mut out = Vec::new();
                        if let ExtractResult::Measurements(mut cs) = extract(src, &raw, now, lsk) {
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

fn sha1(data: &[u8]) -> [u8; 20] {
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

fn split_data_line(line: &str) -> Vec<&str> {
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

fn write_ws_binary(stream: &mut TcpStream, data: &[u8]) {
    let mut h = [0u8; 10];
    h[0] = 0x82;
    if data.len() <= 125 {
        h[1] = data.len() as u8;
        let _ = stream.write_all(&h[..2]);
    } else if data.len() <= 65535 {
        h[1] = 126;
        let e = (data.len() as u16).to_be_bytes();
        h[2] = e[0];
        h[3] = e[1];
        let _ = stream.write_all(&h[..4]);
    } else {
        h[1] = 127;
        let e = (data.len() as u64).to_be_bytes();
        h[2..10].copy_from_slice(&e);
        let _ = stream.write_all(&h);
    }
    let _ = stream.write_all(data);
}

enum ExtractResult {
    Measurements(Vec<(Channel, FieldConfig)>),
    WithEphemeris(Vec<(Channel, FieldConfig)>, BodyEphemeris),
}

fn extract(src: &SourceConfig, body: &str, now: f64, lsk: &LeapSeconds) -> ExtractResult {
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
    } else if src.format == "json" || src.format.is_empty() || src.format == "universal" {
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
            Extract::Ephemeris(n) => {
                let ht = if let Some(ref j) = parsed_json {
                    if let JsonVal::Obj(m) = j {
                        if let Some(JsonVal::Str(s)) = m.get("result") {
                            s.clone()
                        } else {
                            eprintln!("EPH result key absent, fmt={}", src.format);
                            body.to_string()
                        }
                    } else {
                        eprintln!("EPH root not object, fmt={}", src.format);
                        body.to_string()
                    }
                } else {
                    eprintln!(
                        "EPH body not JSON, fmt={} len={} lead={:?}",
                        src.format,
                        body.len(),
                        &body[..body.len().min(120)]
                    );
                    body.to_string()
                };
                if let Some(soe) = ht.find("$$SOE") {
                    let a = &ht[soe + 5..];
                    let e = match a.find("$EOE") {
                        Some(i) => i,
                        None => continue,
                    };
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
                        let c0 = match t.chars().next() {
                            Some(c) => c,
                            None => continue,
                        };
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
                            let &(jd1, p1k, _, _) = match rows.last() {
                                Some(r) => r,
                                None => return ExtractResult::Measurements(Vec::new()),
                            };
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
                        if let Some(rg) = rg0 {
                            channels.push((
                                Channel {
                                    epoch: now,
                                    position: Position::StateVector {
                                        p: p_now,
                                        v,
                                        track: true,
                                    },
                                    name: n.clone(),
                                    value: rg * 1000.0,
                                },
                                FieldConfig {
                                    key: n.clone(),
                                    name: n.clone(),
                                    kernel: 0,
                                    force: 1,
                                    tau: 86400.0 * 365.0,
                                    absorption: 0.0,
                                    advection: 0.0,
                                },
                            ));
                        }
                    }
                }
            }
            Extract::Vectors(n) => {
                let ht = if let Some(ref j) = parsed_json {
                    if let JsonVal::Obj(m) = j {
                        if let Some(JsonVal::Str(s)) = m.get("result") {
                            s.clone()
                        } else {
                            eprintln!("VEC result key absent, fmt={}", src.format);
                            body.to_string()
                        }
                    } else {
                        eprintln!("VEC root not object, fmt={}", src.format);
                        body.to_string()
                    }
                } else {
                    eprintln!(
                        "VEC body not JSON, fmt={} len={} lead={:?}",
                        src.format,
                        body.len(),
                        body.get(..40.min(body.len()))
                    );
                    body.to_string()
                };
                if let Some(soe) = ht.find("$$SOE") {
                    let a = &ht[soe + 5..];
                    let e = match a.find("$EOE") {
                        Some(i) => i,
                        None => continue,
                    };
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
                        let c0 = match t.chars().next() {
                            Some(c) => c,
                            None => continue,
                        };
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
                                if let Some(rg) = cur_rg {
                                    channels.push((
                                        Channel {
                                            epoch: row_epoch,
                                            position: Position::StateVector {
                                                p: p_f,
                                                v: v_f,
                                                track: true,
                                            },
                                            name: n.clone(),
                                            value: rg * 1000.0,
                                        },
                                        FieldConfig {
                                            key: n.clone(),
                                            name: n.clone(),
                                            kernel: 0,
                                            force: 1,
                                            tau: 86400.0 * 365.0,
                                            absorption: 0.0,
                                            advection: 0.0,
                                        },
                                    ));
                                }
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
                        if let Some(rg) = cur_rg {
                            channels.push((
                                Channel {
                                    epoch: row_epoch,
                                    position: Position::StateVector {
                                        p: p_f,
                                        v: v_f,
                                        track: true,
                                    },
                                    name: n.clone(),
                                    value: rg * 1000.0,
                                },
                                FieldConfig {
                                    key: n.clone(),
                                    name: n.clone(),
                                    kernel: 0,
                                    force: 1,
                                    tau: 86400.0 * 365.0,
                                    absorption: 0.0,
                                    advection: 0.0,
                                },
                            ));
                        }
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
                alt_scale,
                vel_key,
                trk_key,
                vr_key,
                fields,
                lon_sign,
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
                                    Position::SurfaceFlow {
                                        body_name: frame_body_name(&src.frame),
                                        lat: la,
                                        lon: lon_val,
                                        alt: al,
                                        speed: sp,
                                        track: tr,
                                        vrate,
                                    }
                                } else {
                                    Position::Surface {
                                        body_name: frame_body_name(&src.frame),
                                        lat: la,
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
                                        JsonVal::Num(n) => match lsk.unix_to_tdb(*n) {
                                            Some(t) => t,
                                            None => continue,
                                        },
                                        _ => continue,
                                    }
                                } else {
                                    continue;
                                };
                                for fc in fields {
                                    if !val_key.is_empty() && fc.name != *val_key {
                                        continue;
                                    }
                                    let mut raw = jpath(v, &fc.key);
                                    if let Some(ref mag_key) = src.flux_from_mag {
                                        if fc.key == *mag_key {
                                            raw = raw.map(|r| 10.0f64.powf(-0.4 * r));
                                        }
                                    }
                                    let val = match raw {
                                        Some(vv) => vv,
                                        None => continue,
                                    };
                                    if val.is_nan() {
                                        continue;
                                    }
                                    channels.push((
                                        Channel {
                                            epoch,
                                            position: position.clone(),
                                            name: fc.name.clone(),
                                            value: val,
                                        },
                                        (*fc).clone(),
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
                                    if let Some(ref mag_key) = src.flux_from_mag {
                                        if fc.key == *mag_key {
                                            raw = raw.map(|r| 10.0f64.powf(-0.4 * r));
                                        }
                                    }
                                    let val = match raw {
                                        Some(vv) => vv,
                                        None => continue,
                                    };
                                    if val.is_nan() {
                                        continue;
                                    }
                                    channels.push((
                                        Channel {
                                            epoch: row_epoch,
                                            position: position.clone(),
                                            name: fc.name.clone(),
                                            value: val,
                                        },
                                        (*fc).clone(),
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
                                if let Some(ref mag_key) = src.flux_from_mag {
                                    if fc.key == *mag_key {
                                        raw = raw.map(|r| 10.0f64.powf(-0.4 * r));
                                    }
                                }
                                let val = match raw {
                                    Some(vv) => vv,
                                    None => continue,
                                };
                                if val.is_nan() {
                                    continue;
                                }
                                for (lon, lat) in vertices.iter() {
                                    channels.push((
                                        Channel {
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
                                        (*fc).clone(),
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
                                if let Some(ref mag_key) = src.flux_from_mag {
                                    if fc.key == *mag_key {
                                        raw = raw.map(|r| 10.0f64.powf(-0.4 * r));
                                    }
                                }
                                let val = match raw {
                                    Some(vv) => vv,
                                    None => continue,
                                };
                                if val.is_nan() {
                                    continue;
                                }
                                for (ra_deg, dec_deg, _z) in &vertices {
                                    let ra = ra_deg.to_radians();
                                    let dec = dec_deg.to_radians();
                                    let (sa, ca) = ra.sin_cos();
                                    let (sd, cd) = dec.sin_cos();
                                    let p = [cd * ca * radius, cd * sa * radius, sd * radius];
                                    channels.push((
                                        Channel {
                                            epoch: row_epoch,
                                            position: Position::StateVector {
                                                p,
                                                v: [0.0, 0.0, 0.0],
                                                track: false,
                                            },
                                            name: fc.name.clone(),
                                            value: val,
                                        },
                                        (*fc).clone(),
                                    ));
                                }
                            }
                        }
                    }
                }
            }
            Extract::Rows { last_line, fields } => {
                if let Frame::Surface { lat, lon, alt, .. } = src.frame {
                    let col_fcs: Vec<(usize, &FieldConfig)> = fields
                        .iter()
                        .filter_map(|fc| {
                            if let Ok(idx) = fc.key.parse::<usize>() {
                                Some((idx, fc))
                            } else {
                                for line in body.lines() {
                                    let t = line.trim();
                                    if t.is_empty() {
                                        continue;
                                    }
                                    let s = (if let Some(u) = t.strip_prefix('#') {
                                        u
                                    } else {
                                        t
                                    })
                                    .trim();
                                    if let Some(idx) = split_data_line(s).iter().position(|c| {
                                        c.eq_ignore_ascii_case(&fc.key) || c.starts_with(&fc.key)
                                    }) {
                                        return Some((idx, fc));
                                    }
                                }
                                None
                            }
                        })
                        .collect();
                    let position = Position::Surface {
                        body_name: frame_body_name(&src.frame),
                        lat,
                        lon,
                        alt,
                    };
                    if *last_line {
                        let last_data_line = body.lines().rev().find(|line| {
                            let t = line.trim();
                            !t.is_empty() && !t.starts_with('#')
                        });
                        if let Some(line) = last_data_line {
                            let cols = split_data_line(line.trim());
                            for (idx, fc) in &col_fcs {
                                let raw = cols
                                    .get(*idx)
                                    .and_then(|s| s.trim().trim_matches('"').parse::<f64>().ok());
                                let val = match raw {
                                    Some(v) => v,
                                    None => continue,
                                };
                                if val.is_nan() {
                                    continue;
                                }
                                channels.push((
                                    Channel {
                                        epoch: now,
                                        position: position.clone(),
                                        name: fc.name.clone(),
                                        value: val,
                                    },
                                    (*fc).clone(),
                                ));
                            }
                        }
                    } else {
                        for line in body.lines() {
                            let trimmed = line.trim();
                            if trimmed.is_empty() || trimmed.starts_with('#') {
                                continue;
                            }
                            let cols = split_data_line(trimmed);
                            for (idx, fc) in &col_fcs {
                                let raw = cols
                                    .get(*idx)
                                    .and_then(|s| s.trim().trim_matches('"').parse::<f64>().ok());
                                let val = match raw {
                                    Some(v) => v,
                                    None => continue,
                                };
                                if val.is_nan() {
                                    continue;
                                }
                                channels.push((
                                    Channel {
                                        epoch: now,
                                        position: position.clone(),
                                        name: fc.name.clone(),
                                        value: val,
                                    },
                                    (*fc).clone(),
                                ));
                            }
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
                            if a_val <= 0.0 {
                                continue;
                            }
                            let a_au = a_val;
                            let n = GAUSS_K * (1.0 / (a_au * a_au * a_au)).sqrt();
                            let m = ma_val.to_radians() + n * (jd_now - epoch_val);
                            let e = e_val;
                            if e >= 1.0 || e < 0.0 {
                                continue;
                            }
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
                            for fc in fields {
                                let mut raw = jpath(v, &fc.key);
                                if let Some(ref mag_key) = src.flux_from_mag {
                                    if fc.key == *mag_key {
                                        raw = raw.map(|r| 10.0f64.powf(-0.4 * r));
                                    }
                                }
                                let val = match raw {
                                    Some(vv) => vv,
                                    None => continue,
                                };
                                if val.is_nan() {
                                    continue;
                                }
                                channels.push((
                                    Channel {
                                        epoch: now,
                                        position: Position::StateVector {
                                            p,
                                            v: vel,
                                            track: false,
                                        },
                                        name: fc.name.clone(),
                                        value: val,
                                    },
                                    (*fc).clone(),
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
                            let ra = ra_deg.to_radians();
                            let dec = dec_deg.to_radians();
                            let (sa, ca) = ra.sin_cos();
                            let (sd, cd) = dec.sin_cos();
                            let p_hat = [cd * ca, cd * sa, sd];
                            let p = [p_hat[0] * d, p_hat[1] * d, p_hat[2] * d];
                            let mu_a = if pmra_key.is_empty() {
                                0.0
                            } else if let Some(v) = jpath(v, pmra_key) {
                                v * MAS_YR_TO_RAD_S
                            } else {
                                0.0
                            };
                            let mu_d = if pmdec_key.is_empty() {
                                0.0
                            } else if let Some(v) = jpath(v, pmdec_key) {
                                v * MAS_YR_TO_RAD_S
                            } else {
                                0.0
                            };
                            let vr = if rv_key.is_empty() {
                                0.0
                            } else if let Some(v) = jpath(v, rv_key) {
                                v * rv_scale
                            } else {
                                0.0
                            };
                            let a_hat = [-sa, ca, 0.0];
                            let d_hat = [-sd * ca, -sd * sa, cd];
                            let vel = [
                                d * (mu_a * a_hat[0] + mu_d * d_hat[0]) + vr * p_hat[0],
                                d * (mu_a * a_hat[1] + mu_d * d_hat[1]) + vr * p_hat[1],
                                d * (mu_a * a_hat[2] + mu_d * d_hat[2]) + vr * p_hat[2],
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
                            for fc in fields {
                                let mut raw: Option<f64> = jpath(v, &fc.key);
                                if let Some(ref mag_field) = src.abs_mag_from {
                                    if fc.name == *mag_field {
                                        raw = raw.map(|v| {
                                            let dist_pc = d / PARSEC_M;
                                            let abs_m = v - 5.0 * (dist_pc / 10.0).log10();
                                            10.0f64.powf(-0.4 * abs_m)
                                        });
                                    }
                                }
                                if let Some(ref mag_key) = src.flux_from_mag {
                                    if fc.key == *mag_key {
                                        raw = raw.map(|r| 10.0f64.powf(-0.4 * r));
                                    }
                                }
                                let val = match raw {
                                    Some(vv) => vv,
                                    None => continue,
                                };
                                if val.is_nan() {
                                    continue;
                                }
                                channels.push((
                                    Channel {
                                        epoch: sample_epoch,
                                        position: Position::StateVector {
                                            p,
                                            v: vel,
                                            track: false,
                                        },
                                        name: fc.name.clone(),
                                        value: val,
                                    },
                                    (*fc).clone(),
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
                                                channels.push((
                                                    Channel {
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
                                                    },
                                                ));
                                                channels.push((
                                                    Channel {
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
                            let mut has_params = false;
                            if let Some(JsonVal::Arr(params)) = root.get("parameters") {
                                for (i, p) in params.iter().enumerate() {
                                    if let JsonVal::Obj(po) = p {
                                        if let Some(JsonVal::Str(nn)) = po.get("name") {
                                            col.insert(nn.clone(), i);
                                            has_params = true;
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
        for (name, val) in &extracted {
            let hapi_found = effective_extracts
                .iter()
                .any(|ext| matches!(ext, Extract::Hapi(_)));
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
            let fc: Option<&FieldConfig> = if hapi_found {
                Some(&FieldConfig {
                    key: name.clone(),
                    name: name.clone(),
                    kernel: 0,
                    force: 0,
                    tau: 0.0,
                    absorption: 0.0,
                    advection: 0.0,
                })
            } else {
                fc
            };
            if let Some(fc) = fc {
                let mut raw = Some(*val);
                if let Some(ref mag_key) = src.flux_from_mag {
                    if fc.key == *mag_key {
                        raw = raw.map(|v| 10.0f64.powf(-0.4 * v));
                    }
                }
                let val = match raw {
                    Some(v) => v,
                    None => continue,
                };
                if val.is_nan() {
                    continue;
                }
                channels.push((
                    Channel {
                        epoch: now,
                        position: Position::Source,
                        name: fc.name.clone(),
                        value: val,
                    },
                    fc.clone(),
                ));
            }
        }
    }
    channels.retain(|(c, _)| c.value.is_finite());
    ExtractResult::Measurements(channels)
}

fn anchor(
    channel: &Channel,
    sensor: &FieldConfig,
    source_ttl: f64,
    source_idx: Option<u32>,
    frame: Option<&Frame>,
    mut origin_state: Option<&mut OriginState>,
    eph: &HashMap<String, BodyEphemeris>,
) -> Option<Oscillator> {
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
    if abs[0].is_nan() || abs[1].is_nan() || abs[2].is_nan() || channel.epoch.is_nan() {
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
    let (vmax, amax, p0f) = match law_bounds(&motion, channel.epoch, resid_ema, eph) {
        Some(b) => b,
        None => return None,
    };
    if p0f[0].is_nan() || p0f[1].is_nan() || p0f[2].is_nan() || vmax.is_nan() || amax.is_nan() {
        return None;
    }
    let body_props = motion
        .anchor_body()
        .and_then(|name| eph.get(name))
        .and_then(|e| e.props.as_ref());
    let extent = kernel_extent(sensor.kernel, body_props, sensor.tau);
    if extent.is_nan() {
        return None;
    }
    Some(Oscillator {
        source: match source_idx {
            Some(idx) => OscillatorSource::Api(idx),
            None => OscillatorSource::Device,
        },
        epoch: channel.epoch,
        ttl: source_ttl,
        extent,
        tau: sensor.tau,
        kernel_id: sensor.kernel as f64,
        force_type: sensor.force as f64,
        absorption: sensor.absorption,
        advection: sensor.advection,
        vmax,
        amax,
        p0f,
        motion: motion.clone(),
        val: channel.value,
        name: channel.name.clone(),
    })
}

fn body_channels(name: &str, props: &BodyProperties, now: f64) -> Vec<(Channel, FieldConfig)> {
    let mut out = Vec::new();
    out.push((
        Channel {
            name: format!("{}.radius", name),
            value: props.radius_m,
            position: Position::Source,
            epoch: now,
        },
        FieldConfig {
            key: format!("{}.radius", name),
            name: format!("{}.radius", name),
            kernel: 0,
            force: 1,
            tau: f64::INFINITY,
            absorption: 0.0,
            advection: 0.0,
        },
    ));
    if let Some(gm) = props.gm {
        out.push((
            Channel {
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

fn probe_mode(path: &str, precise: bool, lat: f64, lon: f64, env: &HashMap<String, String>) -> i32 {
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
    let mut out = String::new();
    let mut dead = String::new();
    let mut lsk: Option<LeapSeconds> = None;
    for src in sources.iter().filter(|s| s.format == "kernel_text") {
        if src.body.as_deref() != Some("naif0012") {
            continue;
        }
        if let Some(text) = fetch_one(&src.url, None, &[], src.ttl) {
            lsk = omegaflow::lsk::parse(&text);
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
    let mut accepted = 0usize;
    let mut declined = 0usize;
    for src in &sources {
        if src.format == "kernel_text" {
            continue;
        }
        let (now, lsk_ref) = match &time_pair {
            Some((t, l)) => (*t, l),
            None => {
                declined += 1;
                dead.push_str("# declined: time absent\n");
                continue;
            }
        };
        let url = match render_url(&src.url, 0.0, 0.0, 0.0, now, 0.0, "", &void_eph, lsk_ref) {
            Some(u) => u,
            None => {
                declined += 1;
                dead.push_str("# declined: time absent\n");
                continue;
            }
        }
        .replace("{lat}", &format!("{:.6}", lat))
        .replace("{lon}", &format!("{:.6}", lon));
        let url = resolve_secret(&url, env);
        let url = url.replace("ZZ", "Z").replace("  ", " ");
        let raw = fetch_raw_probe(&url, None, &[]);
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
            walk_json_probe(&p, "", &mut fields, &mut coords, &mut map_path);
            if map_path.is_none() && !coords.is_empty() {
                map_path = Some(".".to_string());
            }
            let precision_lines = measure_precision(&p);
            if !precision_lines.is_empty() {
                block.push_str(&precision_lines);
            }
            if let Some(ref mp) = map_path {
                if !coords.is_empty() {
                    block.push_str(&format!("map {}\n", mp));
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
                accepted += 1;
                out.push_str(&format!("# verified {} samples\n", n));
                out.push_str(&block);
                out.push('\n');
            }
            Err(why) => {
                declined += 1;
                dead.push_str(&format!("# declined: {}\n", why));
                dead.push_str(&block);
                dead.push('\n');
            }
        }
    }
    std::fs::write("probe_output.φ", &out).ok();
    std::fs::write("probe_dead.φ", &dead).ok();
    eprintln!(
        "probe: wrote probe_output.φ ({} verified) and probe_dead.φ ({} declined)",
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

fn json_num(val: &JsonVal) -> Option<f64> {
    match val {
        JsonVal::Num(n) => Some(*n),
        JsonVal::Str(s) => s.parse().ok(),
        _ => None,
    }
}

fn is_time_key(k: &str) -> bool {
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

fn is_drop_key(key: &str) -> bool {
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
        || kl == "mag"
        || kl == "magnitude"
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
        || kl == "ra"
        || kl == "dec"
        || kl == "sample_size"
        || kl.ends_with("_size")
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
            out.push_str(&format!("field {} {} {} {}\n", col, force, unit, tau));
        }
    }
    if out.is_empty() {
        None
    } else {
        Some(out)
    }
}

fn is_unit_name(name: &str) -> bool {
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
                        out.push_str(&format!("field {} seismic-body {} 3600\n", path, unit));
                    }
                } else {
                    walk_json_probe(v, &path, out, coords, map_path);
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
                    walk_json_probe(first, "", out, coords, map_path);
                } else {
                    walk_json_probe(first, prefix, out, coords, map_path);
                }
            } else {
                for (i, v) in arr.iter().enumerate() {
                    walk_json_probe(v, &format!("{}.{}", prefix, i), out, coords, map_path);
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
            out.push_str(&format!("# {} = {:?}\n", prefix, n));
            let (force, unit, tau) = probe_classify(key);
            if force == "UNCERTAIN" {
                out.push_str(&format!(
                    "# uncertain field {} — force/unit undetermined, review\n",
                    prefix
                ));
            } else if force != "DROP" {
                out.push_str(&format!("field {} {} {} {}\n", prefix, force, unit, tau));
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
                out.push_str(&format!("# {} = {:?} (str)\n", prefix, n));
                let (force, unit, tau) = probe_classify(key);
                if force == "UNCERTAIN" {
                    out.push_str(&format!(
                        "# uncertain field {} — force/unit undetermined, review\n",
                        prefix
                    ));
                } else if force != "DROP" {
                    out.push_str(&format!("field {} {} {} {}\n", prefix, force, unit, tau));
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
    let mut sources = Vec::new();
    let mut cur_ttl: u64 = 0;
    let mut cur_url = String::new();
    let mut cur_body: Option<String> = None;
    let mut cur_frame: Option<Frame> = None;
    let mut cur_format = String::new();
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
                if cur_ttl > 0 && !cur_url.is_empty() {
                    if cur_format == "kernel_text" || cur_frame.is_some() {
                        sources.push(SourceConfig {
                            ttl: cur_ttl,
                            url: std::mem::take(&mut cur_url),
                            frame: match cur_frame.clone() {
                                Some(f) => f,
                                None => Frame::Manifest,
                            },
                            format: std::mem::take(&mut cur_format),
                            extracts: Vec::new(),
                            headers: Vec::new(),
                            post_body: None,
                            target: None,
                            catalog: None,
                            max_freq: None,
                            min_freq: None,
                            body: cur_body.clone(),
                            stations_url: None,
                            stations_path: String::from("stations"),
                            stations_lat: String::from("lat"),
                            stations_lon: String::from("lng"),
                            stations_id: String::from("id"),
                            flux_from_mag: None,
                            abs_mag_from: None,
                            catalog_epoch: None,
                            repeat_ra_bins: 0,
                            fanout_cap: 0,
                            stations_flatten: String::new(),
                            stations_filter: None,
                            fanout_delay: 0,
                        });
                    }
                }
                cur_url = parts[1].to_string();
                cur_frame = None;
                cur_body = None;
                cur_ttl = 0;
                cur_format.clear();
            }
            "ttl" if parts.len() >= 2 => {
                if let Ok(v) = parts[1].parse::<u64>() {
                    cur_ttl = v;
                }
            }
            "format" if parts.len() >= 2 => cur_format = parts[1].to_string(),
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
                let Ok(lat) = parts[2].parse::<f64>() else {
                    continue;
                };
                let Ok(lon) = parts[3].parse::<f64>() else {
                    continue;
                };
                let alt: f64 = match parts.get(4) {
                    Some(s) => match s.parse::<f64>() {
                        Ok(a) => a,
                        Err(_) => continue,
                    },
                    None => 0.0,
                };
                cur_body = Some(body.clone());
                cur_frame = Some(Frame::Surface {
                    body_name: body,
                    lat,
                    lon,
                    alt,
                });
            }
            "body" if parts.len() >= 2 => {
                cur_body = Some(parts[1].to_string());
            }
            _ => {}
        }
    }
    if cur_ttl > 0 && !cur_url.is_empty() {
        if cur_format == "kernel_text" || cur_frame.is_some() {
            sources.push(SourceConfig {
                ttl: cur_ttl,
                url: cur_url,
                frame: match cur_frame.clone() {
                    Some(f) => f,
                    None => Frame::Manifest,
                },
                format: std::mem::take(&mut cur_format),
                extracts: Vec::new(),
                headers: Vec::new(),
                post_body: None,
                target: None,
                catalog: None,
                max_freq: None,
                min_freq: None,
                body: cur_body,
                stations_url: None,
                stations_path: String::from("stations"),
                stations_lat: String::from("lat"),
                stations_lon: String::from("lng"),
                stations_id: String::from("id"),
                flux_from_mag: None,
                abs_mag_from: None,
                catalog_epoch: None,
                repeat_ra_bins: 0,
                fanout_cap: 0,
                stations_flatten: String::new(),
                stations_filter: None,
                fanout_delay: 0,
            });
        }
    }
    sources
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

fn ci_mode(dir: &str) -> i32 {
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
    let total = sources.len();
    let mut reachable = 0usize;
    let mut dead = 0usize;
    let mut mirrored = 0u32;
    let mut fresh = 0u32;
    let mut template_skipped = 0u32;
    for src in &sources {
        if src.url.starts_with("https://github.com/omegaflow/sources")
            || src.format == "ephemeris_binary"
            || src.format == "catalog_dastcom"
            || src.format == "csv_zip"
            || src.format == "kernel_text"
        {
            continue;
        }
        if src.url.contains('{') {
            template_skipped += 1;
            continue;
        }
        let netloc = extract_netloc(&src.url);
        let name = source_name_from_url(&src.url);
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
        let raw = match fetch_raw(&src.url, None, &[], src.ttl) {
            Some(r) => r,
            None => {
                eprintln!("ci-mode: fetch returned void for {}", src.url);
                dead += 1;
                continue;
            }
        };
        if parse_json(&raw).is_some() {
            reachable += 1;
            eprintln!("ci-mode: {} JSON ok", src.url);
            if let Some(cp) = &cache_path {
                if let Some(parent) = std::path::Path::new(cp).parent() {
                    let _ = std::fs::create_dir_all(parent);
                }
                let _ = std::fs::write(cp, &raw);
            }
            if mirror_enabled {
                if let Some(netloc) = extract_netloc(&src.url) {
                    let name = source_name_from_url(&src.url);
                    let tmp_path = format!("/tmp/archivar_cache/{}/{}.json", netloc, name);
                    if std::fs::write(&tmp_path, &raw).is_ok() {
                        if let Ok(_gh) = std::env::var("GH_TOKEN") {
                            let status = Command::new("gh")
                                .arg("release")
                                .arg("upload")
                                .arg(netloc)
                                .arg(&tmp_path)
                                .arg("--clobber")
                                .arg("--repo")
                                .arg("omegaflow/sources")
                                .output();
                            if let Ok(o) = status {
                                if o.status.success() {
                                    mirrored += 1;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            eprintln!("ci-mode: {} JSON parse void", src.url);
            dead += 1;
        }
    }
    eprintln!(
        "ci-mode: {}/{} reachable, {} dead, {} mirrored to CDN, {} fresh (local TTL), {} template-skipped, mirror={}",
        reachable, total, dead, mirrored, fresh, template_skipped, mirror_enabled
    );
    if dead == 0 {
        0
    } else {
        1
    }
}

fn main() {
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
            std::process::exit(ci_mode(dir));
        }
        if args.len() > 1 && args[1] == "--gold" {
            let input = match args.get(2) {
                Some(s) => s.as_str(),
                None => {
                    eprintln!("--gold: input file argument absent");
                    std::process::exit(1);
                }
            };
            let output = match args.get(3) {
                Some(s) => s.as_str(),
                None => {
                    eprintln!("--gold: output file argument absent");
                    std::process::exit(1);
                }
            };
            std::process::exit(gold_convert_mode(input, output));
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
            std::process::exit(probe_mode(path, precise, lat, lon, &env));
        }
    }
    let loaded = load_sources();
    let port: u16 = match std::env::var("PORT").ok().and_then(|s| s.parse().ok()) {
        Some(p) => p,
        None => PORT_CONST,
    };
    let (fetch_tx, fetch_rx) = mpsc::channel::<FetchResult>();
    let (osc_tx, osc_rx) = mpsc::channel::<Vec<Oscillator>>();
    let (presence_tx, presence_rx) = mpsc::channel::<(f64, f64, f64, f64, f64)>();
    let body_ephemerides = Arc::new(HashMap::new());
    let index_html = match std::fs::read(resolve_asset("static/index.html")) {
        Ok(v) => v,
        Err(_) => {
            eprintln!("static/index.html absent — serving 0 bytes");
            Vec::new()
        }
    };
    let constants_js = match std::fs::read(resolve_asset("static/constants.js")) {
        Ok(v) => v,
        Err(_) => {
            eprintln!("static/constants.js absent — browser protocol empty");
            Vec::new()
        }
    };
    let time: Arc<Mutex<Option<LeapSeconds>>> = Arc::new(Mutex::new(None));
    let mut archive = Archive {
        sources: loaded,
        body_ephemerides: body_ephemerides.clone(),
        field: Arc::new(build_buffer(
            Vec::new(),
            1.0,
            body_ephemerides.clone(),
            None,
            None,
        )),
        presence: HashMap::new(),
        origins: HashMap::new(),
        pck_bodies: HashMap::new(),
        time: time.clone(),
        asteroids: None,
        stars: None,
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
    let sr = TcpRadiator::new(
        port,
        body_names.clone(),
        archive.field.clone(),
        index_html.clone(),
        constants_js.clone(),
        osc_tx.clone(),
        presence_tx,
        time.clone(),
    );
    radiators.push(Box::new(sr));
    radiators.push(Box::new(AudioRadiator::new(AUDIO_SAMPLE_RATE)));
    radiators.push(Box::new(StderrRadiator {
        last_line: String::new(),
        interactive: std::io::stderr().is_terminal(),
    }));
    let cadence = 1.0;
    let mut gm_text: Option<String> = None;
    let mut pck_text: Option<String> = None;
    loop {
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
                let Some(text) = fetch_one(&url, None, &[], ttl) else {
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
                    if let Some(l) = omegaflow::lsk::parse(&text) {
                        if let Ok(mut guard) = archive.time.lock() {
                            *guard = Some(l);
                        }
                    }
                }
                _ => {}
            }
            archive.pck_bodies = omegaflow::pck::parse(gm_text.as_deref(), pck_text.as_deref());
        }
        let lsk = match leap_seconds(&archive.time) {
            Some(l) => l,
            None => {
                thread::sleep(std::time::Duration::from_secs_f64(cadence));
                continue;
            }
        };
        let now = match lsk.system_now_tdb() {
            Some(t) => t,
            None => {
                thread::sleep(std::time::Duration::from_secs_f64(cadence));
                continue;
            }
        };
        let mut fetched_oscillators: Vec<Oscillator> = Vec::new();
        while let Ok(res) = fetch_rx.try_recv() {
            archive
                .origins
                .entry(res.source_idx as u32)
                .or_insert(OriginState {
                    fetched: now,
                    prev_epoch: now,
                    prev_abs: [0.0, 0.0, 0.0],
                    prev_motion: None,
                    resid_ema: 0.0,
                    has_prev: false,
                })
                .fetched = now;
            if let Some((name, eph)) = res.eph_update {
                let mut eph_map = (*archive.body_ephemerides).clone();
                eph_map.insert(name, eph);
                archive.body_ephemerides = Arc::new(eph_map);
            }
            if let Some(hash) = res.asteroid_hash {
                archive.asteroids = Some(hash);
            }
            if let Some(hash) = res.star_hash {
                archive.stars = Some(hash);
            }
            let src = &archive.sources[res.source_idx];
            for (channel, sensor) in &res.channels {
                let track_origin = matches!(channel.position, Position::Source)
                    || matches!(channel.position, Position::StateVector { track: true, .. });
                if let Some(osc) = anchor(
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
                    fetched_oscillators.push(osc);
                }
            }
        }
        while let Ok(oscs) = osc_rx.try_recv() {
            fetched_oscillators.extend(oscs);
        }
        while let Ok((pt, px, py, pz, pr)) = presence_rx.try_recv() {
            archive
                .presence
                .insert("device".to_string(), (pt, px, py, pz, pr));
        }
        for i in 0..archive.sources.len() {
            let origin = i as u32;
            if !origin_stale(&archive.origins, origin, archive.sources[i].ttl, now) {
                continue;
            }
            if archive.sources[i].format == "ephemeris_binary" {
                let url = archive.sources[i].url.clone();
                let ftx = fetch_tx.clone();
                let src_clone = archive.sources[i].clone();
                let src_idx = i;
                let _body_log = match &src_clone.body {
                    Some(b) => b.as_str(),
                    None => "",
                };
                archive.origins.entry(origin).or_insert(OriginState {
                    fetched: now,
                    prev_epoch: now,
                    prev_abs: [0.0; 3],
                    prev_motion: None,
                    resid_ema: 0.0,
                    has_prev: false,
                });
                let lsk_c = lsk.clone();
                thread::spawn(move || {
                    let tmp_path = match &src_clone.body {
                        Some(b) => format!("/tmp/omegaflow_eph_{}.bin", b),
                        None => return,
                    };
                    if !cache_fresh(&tmp_path, src_clone.ttl) {
                        let bytes = match fetch_raw_bytes(&url, src_clone.ttl) {
                            Some(b) => b,
                            None => return,
                        };
                        if std::fs::write(&tmp_path, &bytes).is_err() {
                            return;
                        }
                    }
                    let _body_log = match &src_clone.body {
                        Some(b) => b.as_str(),
                        None => "",
                    };
                    if let ExtractResult::WithEphemeris(_, eph) =
                        extract(&src_clone, &tmp_path, now, &lsk_c)
                    {
                        let _ = ftx.send(FetchResult {
                            source_idx: src_idx,
                            channels: Vec::new(),
                            eph_update: src_clone.body.clone().map(|b| (b, eph)),
                            asteroid_hash: None,
                            star_hash: None,
                        });
                    }
                });
                continue;
            }
            if archive.sources[i].format == "catalog_dastcom" {
                let url = archive.sources[i].url.clone();
                let ftx = fetch_tx.clone();
                let src_clone = archive.sources[i].clone();
                let src_idx = i;
                let src_ttl = src_clone.ttl;
                let cadence_c = cadence;
                archive.origins.entry(origin).or_insert(OriginState {
                    fetched: now,
                    prev_epoch: now,
                    prev_abs: [0.0; 3],
                    prev_motion: None,
                    resid_ema: 0.0,
                    has_prev: false,
                });
                thread::spawn(move || {
                    let name = url.rsplit('/').next().unwrap_or("catalog").to_string();
                    let tmp_path = format!("/tmp/omegaflow_catalog_{}", name);
                    if !cache_fresh(&tmp_path, src_ttl) {
                        let bytes = match fetch_raw_bytes(&url, src_ttl) {
                            Some(b) => b,
                            None => return,
                        };
                        if std::fs::write(&tmp_path, &bytes).is_err() {
                            return;
                        }
                    }
                    let bytes = match std::fs::read(&tmp_path) {
                        Ok(b) => b,
                        Err(_) => return,
                    };
                    let hash = build_asteroid_hash(&bytes, cadence_c, src_ttl);
                    eprintln!(
                        "catalog_dastcom: {} records, cell_size {:.3e} m, vmax {:.1} m/s, rmax {:.3e} m",
                        hash.records.len(),
                        hash.cell_size,
                        hash.vmax,
                        hash.rmax
                    );
                    let _ = ftx.send(FetchResult {
                        source_idx: src_idx,
                        channels: Vec::new(),
                        eph_update: None,
                        asteroid_hash: Some(Arc::new(hash)),
                        star_hash: None,
                    });
                });
                continue;
            }
            if archive.sources[i].format == "catalog_tycho" {
                let url = archive.sources[i].url.clone();
                let ftx = fetch_tx.clone();
                let src_clone = archive.sources[i].clone();
                let src_idx = i;
                let src_ttl = src_clone.ttl;
                let cadence_c = cadence;
                let build_epoch = now;
                archive.origins.entry(origin).or_insert(OriginState {
                    fetched: now,
                    prev_epoch: now,
                    prev_abs: [0.0, 0.0, 0.0],
                    prev_motion: None,
                    resid_ema: 0.0,
                    has_prev: false,
                });
                thread::spawn(move || {
                    let name = url.rsplit('/').next().unwrap_or("stars").to_string();
                    let tmp_path = format!("/tmp/omegaflow_catalog_{}", name);
                    if !cache_fresh(&tmp_path, src_ttl) {
                        let bytes = match fetch_raw_bytes(&url, src_ttl) {
                            Some(b) => b,
                            None => return,
                        };
                        if std::fs::write(&tmp_path, &bytes).is_err() {
                            return;
                        }
                    }
                    let bytes = match std::fs::read(&tmp_path) {
                        Ok(b) => b,
                        Err(_) => return,
                    };
                    let hash = build_star_hash(&bytes, build_epoch, cadence_c, src_ttl);
                    eprintln!(
                        "catalog_tycho: {} stars, cell_size {:.3e} m, vmax {:.1} m/s",
                        hash.records.len(),
                        hash.cell_size,
                        hash.vmax
                    );
                    let _ = ftx.send(FetchResult {
                        source_idx: src_idx,
                        channels: Vec::new(),
                        eph_update: None,
                        asteroid_hash: None,
                        star_hash: Some(Arc::new(hash)),
                    });
                });
                continue;
            }
            if archive.sources[i].format == "csv_zip" {
                let ftx = fetch_tx.clone();
                let src_clone = archive.sources[i].clone();
                let src_idx = i;
                let eph_arc = archive.body_ephemerides.clone();
                let e = env.clone();
                archive.origins.entry(origin).or_insert(OriginState {
                    fetched: now,
                    prev_epoch: now,
                    prev_abs: [0.0; 3],
                    prev_motion: None,
                    resid_ema: 0.0,
                    has_prev: false,
                });
                let lsk_c = lsk.clone();
                thread::spawn(move || {
                    let url = match render_source_url(
                        &src_clone, 0.0, 0.0, 0.0, now, 0.0, &eph_arc, &e, &lsk_c,
                    ) {
                        Some(u) => u,
                        None => return,
                    };
                    let tmp_path = format!("/tmp/omegaflow_csv_{}.zip", src_idx);
                    if !cache_fresh(&tmp_path, src_clone.ttl) {
                        let headers = render_headers(&src_clone.headers, &e);
                        let bytes = match fetch_raw_bytes_post(&url, None, &headers, src_clone.ttl)
                        {
                            Some(b) => b,
                            None => return,
                        };
                        if std::fs::write(&tmp_path, &bytes).is_err() {
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
                            asteroid_hash: None,
                            star_hash: None,
                        });
                    }
                });
                continue;
            }
            let body_name = match &archive.sources[i].frame {
                Frame::Surface { body_name, .. } | Frame::Barycenter { body_name, .. } => {
                    body_name.as_str()
                }
                Frame::Manifest => continue,
            };
            let body_props = archive
                .body_ephemerides
                .get(body_name)
                .and_then(|e| e.props.as_ref());
            let mut r = 0.0_f64;
            for ext in &archive.sources[i].extracts {
                for fc in extract_fields(ext) {
                    r = f64::max(r, kernel_extent(fc.kernel, body_props, fc.tau));
                }
            }
            if r == 0.0 {
                continue;
            }
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
            let presences: Vec<(f64, f64, f64, f64, f64)> =
                archive.presence.values().cloned().collect();
            if !presence_gate(&presences, pos, r) {
                continue;
            }
            let ftx = fetch_tx.clone();
            let src_clone = archive.sources[i].clone();
            let eph_arc = archive.body_ephemerides.clone();
            let e = env.clone();
            let src_idx = i;
            archive.origins.entry(origin).or_insert(OriginState {
                fetched: now,
                prev_epoch: now,
                prev_abs: [0.0; 3],
                prev_motion: None,
                resid_ema: 0.0,
                has_prev: false,
            });
            let lsk_c = lsk.clone();
            let presence_center = presences.first().map(|p| (p.1, p.2, p.3));
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
                            asteroid_hash: None,
                            star_hash: None,
                        });
                    }
                    return;
                }
                let url = match render_source_url(
                    &src_clone, pos.0, pos.1, pos.2, now, r, &eph_arc, &e, &lsk_c,
                ) {
                    Some(u) => u,
                    None => return,
                };
                let body =
                    render_source_body(&src_clone, pos.0, pos.1, pos.2, now, r, &eph_arc, &lsk_c);
                let headers = render_headers(&src_clone.headers, &e);
                let raw = fetch_one(&url, body.as_deref(), &headers, src_clone.ttl);
                let channels = match raw {
                    Some(ref r) => match extract(&src_clone, r, now, &lsk_c) {
                        ExtractResult::Measurements(v) => v,
                        ExtractResult::WithEphemeris(v, eph) => {
                            let _ = ftx.send(FetchResult {
                                source_idx: src_idx,
                                channels: Vec::new(),
                                eph_update: src_clone.body.clone().map(|b| (b, eph)),
                                asteroid_hash: None,
                                star_hash: None,
                            });
                            v
                        }
                    },
                    None => Vec::new(),
                };
                let _ = ftx.send(FetchResult {
                    source_idx: src_idx,
                    channels,
                    eph_update: None,
                    asteroid_hash: None,
                    star_hash: None,
                });
            });
        }
        {
            let mut all: Vec<Oscillator> = Vec::new();
            all.append(&mut fetched_oscillators);
            let old = archive.field.clone();
            let mut hashes: Vec<&SpatialHash> = old.bodies.values().collect();
            hashes.push(&old.inertial);
            for hash in hashes {
                for v in hash.cells.values().chain(std::iter::once(&hash.unbounded)) {
                    for s in v {
                        if matches!(s.source, OscillatorSource::Body) {
                            continue;
                        }
                        if (now - s.epoch).abs() <= s.ttl * 64.0 {
                            all.push(s.clone());
                        }
                    }
                }
            }
            for (name, eph) in archive.body_ephemerides.iter() {
                if let Some(props) = &eph.props {
                    if props.radius_m > 0.0 {
                        let frame = Frame::Barycenter {
                            body_name: name.clone(),
                            scale: 1.0,
                        };
                        for (channel, sensor) in body_channels(name, props, now) {
                            if let Some(mut osc) = anchor(
                                &channel,
                                &sensor,
                                86400.0,
                                Some(archive.sources.len() as u32),
                                Some(&frame),
                                None,
                                &archive.body_ephemerides,
                            ) {
                                osc.source = OscillatorSource::Body;
                                all.push(osc);
                            }
                        }
                    }
                }
            }
            archive.field = Arc::new(build_buffer(
                all,
                cadence,
                archive.body_ephemerides.clone(),
                archive.asteroids.clone(),
                archive.stars.clone(),
            ));
        }
        let f = archive.field.clone();
        for r in &mut radiators {
            r.accept(f.clone());
        }
        let elapsed = match lsk.system_now_tdb() {
            Some(t) => t - now,
            None => cadence,
        };
        if elapsed < cadence {
            thread::sleep(std::time::Duration::from_secs_f64(cadence - elapsed));
        }
    }
}
fn tdb_to_jd(tdb_secs: f64) -> f64 {
    tdb_secs / 86400.0 + J2000_EPOCH
}

fn horizons_nums(line: &str, keys: [&str; 3]) -> Option<[f64; 3]> {
    let mut out = [0.0; 3];
    for (i, k) in keys.iter().enumerate() {
        let p = line.find(k)?;
        let r = line[p + k.len()..].trim_start_matches(|c: char| c == '=' || c == ' ' || c == '\t');
        let end = match r.find(|c: char| c.is_whitespace()) {
            Some(pos) => pos,
            None => r.len(),
        };
        out[i] = r[..end].parse().ok()?;
    }
    Some(out)
}

fn ecliptic_to_field(v: [f64; 3]) -> [f64; 3] {
    let (c, s) = (ECLIPTIC_OBLIQUITY.cos(), ECLIPTIC_OBLIQUITY.sin());
    [v[0], v[1] * c - v[2] * s, v[1] * s + v[2] * c]
}

fn flatten_geojson_coords(val: &[JsonVal]) -> Vec<(f64, f64, Option<f64>)> {
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

#[cfg(test)]
mod tests {
    use super::*;
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
on earth 0.0 0.0\n\
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
    fn test_star_hash_build_and_query() {
        let mut bin = Vec::new();
        bin.extend_from_slice(&0f64.to_le_bytes());
        bin.extend_from_slice(&0f64.to_le_bytes());
        bin.extend_from_slice(&0f32.to_le_bytes());
        bin.extend_from_slice(&0f32.to_le_bytes());
        bin.extend_from_slice(&100f32.to_le_bytes());
        bin.extend_from_slice(&0f32.to_le_bytes());
        bin.extend_from_slice(&1f32.to_le_bytes());
        let hash = build_star_hash(&bin, 0.0, 1.0, 604800);
        assert_eq!(hash.records.len(), 1);
        let d = 10.0 * PARSEC_M;
        let mut records: Vec<OscRecord> = Vec::new();
        query_star_hash(&hash, [0.0, 0.0, 0.0], 0.0, d * 1.001, 0.0, &mut records);
        assert_eq!(records.len(), 1);
        assert!((records[0].0 - d).abs() / d < 1e-9);
        assert_eq!(records[0].3, 1.0);
        assert_eq!(records[0].6, 604800.0);
        let mut outside: Vec<OscRecord> = Vec::new();
        query_star_hash(&hash, [2.0 * d, 0.0, 0.0], 0.0, d * 0.5, 0.0, &mut outside);
        assert!(outside.is_empty());
        let mut short = [0u8; 4];
        short.copy_from_slice(&0f32.to_le_bytes());
        assert!(parse_star_record(&short).is_none());
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
    fn temp_gold_convert_check() {
        let content =
            std::fs::read_to_string("phi/port/queue/sources_gold_pre-cdn_27k_359-domains.φ")
                .unwrap();
        let mut blocks = 0usize;
        let mut parsed = 0usize;
        let mut with_extracts = 0usize;
        let mut block = String::new();
        for line in content.lines() {
            let t = line.trim_start();
            if t.starts_with("url ") && !block.is_empty() {
                blocks += 1;
                let conv = super::convert_gold_block(&block);
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
            "gold convert: {} blocks, {} parsed, {} with extracts",
            blocks, parsed, with_extracts
        );
    }

    #[test]
    fn test_gold_convert_celestial_and_post() {
        let has_field = |src: &super::SourceConfig| {
            src.extracts.iter().any(|e| match e {
                super::Extract::CelestialMap { fields, .. } => !fields.is_empty(),
                super::Extract::Map { fields, .. } => !fields.is_empty(),
                _ => false,
            })
        };
        let celestial = "source oac\nttl 86400\nforce em\nurl https://api.example.org/{target}/\nverify false\ntarget SN2014J\nmap .\nlat_key ra\nlon_key dec\nfield name name\n";
        let conv = super::convert_gold_block(celestial);
        let srcs = super::parse_sources(&conv);
        assert_eq!(srcs.len(), 1);
        assert!(matches!(&srcs[0].frame, super::Frame::Barycenter { .. }));
        assert!(srcs[0]
            .extracts
            .iter()
            .any(|e| matches!(e, super::Extract::CelestialMap { .. })));
        assert!(has_field(&srcs[0]));
        let post = "source stac\nttl 86400\nforce em\nurl https://example.org/search\nmethod post\nbody {\"collections\":[\"x\"],\"bbox\":[{lon_min},{lat_min},{lon_max},{lat_max}]}\nmap features\nlat_key properties.centroid.lat\nlon_key properties.centroid.lon\nfield id scene\n";
        let conv = super::convert_gold_block(post);
        let srcs = super::parse_sources(&conv);
        assert_eq!(srcs.len(), 1);
        assert!(matches!(&srcs[0].frame, super::Frame::Surface { .. }));
        assert_eq!(
            srcs[0].post_body.as_deref(),
            Some("{\"collections\":[\"x\"],\"bbox\":[{lon_min},{lat_min},{lon_max},{lat_max}]}")
        );
        assert!(srcs[0]
            .extracts
            .iter()
            .any(|e| matches!(e, super::Extract::Map { .. })));
        assert!(has_field(&srcs[0]));
        let mast = "source mast\nttl 3600\nforce em\nurl https://example.org/tap\nformat votable\nmap data\nfield_in s_ra ra_deg\nfield_in s_dec dec_deg\nlat_key s_ra\nlon_key s_dec\n";
        let conv = super::convert_gold_block(mast);
        let srcs = super::parse_sources(&conv);
        assert_eq!(srcs.len(), 1);
        assert!(matches!(&srcs[0].frame, super::Frame::Barycenter { .. }));
        assert!(srcs[0]
            .extracts
            .iter()
            .any(|e| matches!(e, super::Extract::CelestialMap { .. })));
        assert!(has_field(&srcs[0]));
    }
    #[test]
    fn test_parse_stations_xml() {
        let xml = "<?xml version=\"1.0\" ?><GINServices>\n <ObservatoryList>\n  <Observatory>\n   <Code>AAE</Code>\n   <Name>Addis Ababa</Name>\n   <Latitude>9.035</Latitude>   <Longitude>38.770</Longitude>   <Elevation>2441</Elevation>\n  </Observatory>\n  <Observatory>\n   <Code>YKC</Code>\n   <Latitude>62.48</Latitude>   <Longitude>-114.48</Longitude>   <Elevation>181</Elevation>\n  </Observatory>\n </ObservatoryList>\n</GINServices>";
        let st = parse_stations_xml(xml);
        assert_eq!(st.len(), 2);
        assert_eq!(st[0].id, "AAE");
        assert_eq!(st[0].lat, 9.035);
        assert_eq!(st[0].lon, 38.770);
        assert_eq!(st[1].id, "YKC");
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
        if let Ok(existing) = std::fs::read_to_string("phi/port/stage/staging_verified.φ") {
            for l in existing.lines() {
                let t = l.trim_start();
                if t.starts_with("url ") {
                    seen.insert(t[4..].trim().to_string());
                }
            }
            ok_text = existing;
        }
        let mut void_text = String::new();
        if let Ok(existing) = std::fs::read_to_string("phi/port/stage/staging_empty.txt") {
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
        for e in std::fs::read_dir("phi/port/stage").unwrap().flatten() {
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
        let ok_path = "phi/port/stage/staging_verified.φ";
        let void_path = "phi/port/stage/staging_empty.txt";
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
                }],
                lon_sign: None,
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
            flattening: 0.00589,
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
            flattening: 0.00589,
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
            flattening: 0.00589,
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
            flattening: 0.00589,
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
            flattening: 0.00589,
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
        assert!((props.flattening - (6378136.6 - 6356751.9) / 6378136.6).abs() < 1e-15);
        assert_eq!(eph.granules.len(), 1);
        let deltas = super::nutation_deltas_at(&props, 2451545.0).unwrap();
        assert!((deltas.0 - 1.0).abs() < 1e-12);
        assert!((deltas.1 - 2.0).abs() < 1e-12);
        assert!((deltas.2 - 0.5).abs() < 1e-12);
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
    fn test_live_sources_extract() {
        let srcs = super::load_sources();
        eprintln!("load_sources returned {} sources", srcs.len());
        let fixture_lsk = full_fixture_lsk();
        let now = fixture_lsk.system_now_tdb().unwrap();
        let mut ok = 0usize;
        let mut empty: Vec<(String, String)> = Vec::new();
        let mut limit = 600usize;
        let env = super::load_env();
        for s in srcs.iter() {
            let mut url = s.url.clone();
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
                url = url.replace(k, v);
            }
            url = super::resolve_secret(&url, &env);
            if url.starts_with("https://github.com/omegaflow/sources") {
                continue;
            }
            if s.fanout_cap > 0 {
                eprintln!("skip fanout (needs fanout_fetch): {}", s.url);
                continue;
            }
            if s.format == "csv_zip" {
                eprintln!("skip csv_zip (byte path): {}", s.url);
                continue;
            }
            if s.format == "kernel_text" {
                eprintln!(
                    "skip kernel_text (data file, not a field source): {}",
                    s.url
                );
                continue;
            }
            if limit == 0 {
                break;
            }
            limit -= 1;
            let headers = super::render_headers(&s.headers, &env);
            let body = match super::fetch_one(&url, None, &headers, s.ttl) {
                Some(b) => b,
                None => {
                    empty.push((s.url.clone(), "fetch returned empty".into()));
                    continue;
                }
            };
            match super::extract(s, &body, now, &fixture_lsk) {
                super::ExtractResult::Measurements(v) => {
                    if v.is_empty() {
                        let diag = super::diagnose_no_samples(s, &body);
                        empty.push((s.url.clone(), format!("no samples ({})", diag)));
                    } else {
                        ok += 1;
                    }
                }
                super::ExtractResult::WithEphemeris(v, _) => {
                    if v.is_empty() {
                        let diag = super::diagnose_no_samples(s, &body);
                        empty.push((s.url.clone(), format!("no samples ({})", diag)));
                    } else {
                        ok += 1;
                    }
                }
            }
        }
        eprintln!(
            "\n=== LIVE SOURCE EXTRACTION: {} ok, {} empty (of {} tested) ===",
            ok,
            empty.len(),
            ok + empty.len()
        );
        for (u, why) in empty.iter() {
            eprintln!("  void {}  {}", u, why);
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
                }],
                lon_sign: None,
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
                }],
                lon_sign: None,
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
    fn test_force_id_electric() {
        assert_eq!(super::force_id_of("electric"), Some(8));
        assert_eq!(super::force_id_of("biotic"), None);
        assert_eq!(super::kernel_for_force(8), Some(1));
    }
}
