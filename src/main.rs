#![allow(mixed_script_confusables)]
use std::collections::HashMap;
use std::io::{Cursor, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process::Command;
use std::sync::{Arc, Condvar, Mutex, RwLock};
use std::thread;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

const Φ: f64 = 1.618033988749895;
const J2000_EPOCH: f64 = 2451545.0;
const UNIX_J2000_OFFSET: f64 = 946728000.0;
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
    eprintln!(
        "asset '{}' not found (CWD={:?})",
        rel,
        std::env::current_dir()
    );
    cwd_candidate
}

const CHEBYSHEV_N: usize = 18;
const ECLIPTIC_OBLIQUITY: f64 = 0.409092804;
const AU: f64 = 1.495978707e11;
const GAUSS_K: f64 = 0.01720209895;

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
    v_sound: Option<f64>,
    v_seismic_p: Option<f64>,
    v_seismic_s: Option<f64>,
    alpha_thermal: Option<f64>,
    d_diffusion: Option<f64>,
    v_advective: Option<f64>,
}

#[derive(Clone, Default)]
struct ChebyshevGranule {
    t0_jd: f64,
    dt_jd: f64,
    cx: [f64; CHEBYSHEV_N],
    cy: [f64; CHEBYSHEV_N],
    cz: [f64; CHEBYSHEV_N],
}

#[derive(Clone, Default)]
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

fn parse_ephemeris_binary(data: &[u8]) -> Option<BodyEphemeris> {
    if data.len() < 24 || data[0] != 0xCF || data[1] != 0x86 || data[2] != 0x01 {
        return None;
    }
    let section_count = u32::from_le_bytes(data[4..8].try_into().ok()?) as usize;
    let mut pos = 8usize;
    let mut granules = Vec::new();
    let mut rotation_matrices = Vec::new();
    let mut props = None;
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
                f64::from_le_bytes(
                    data[pos + i * 8..pos + (i + 1) * 8]
                        .try_into()
                        .unwrap_or([0; 8]),
                )
            };
            props = Some(BodyProperties {
                α0_deg: f(0),
                dα0_dt_deg_per_century: f(1),
                δ0_deg: f(2),
                dδ0_dt_deg_per_century: f(3),
                w0_deg: f(4),
                dw_dt_deg_per_day: f(5),
                radius_m: f(6),
                flattening: f(7),
                v_sound: None,
                v_seismic_p: None,
                v_seismic_s: None,
                alpha_thermal: None,
                d_diffusion: None,
                v_advective: None,
            });
            pos += 64;
            continue;
        }
        if stype == 2 {
            let f = |i: usize| -> f64 {
                f64::from_le_bytes(
                    data[pos + i * 8..pos + (i + 1) * 8]
                        .try_into()
                        .unwrap_or([0; 8]),
                )
            };
            let vs = f(0);
            let vp = f(1);
            let vss = f(2);
            let ath = f(3);
            let dd = f(4);
            let vad = f(5);
            props = Some(match props {
                Some(mut p) => {
                    if vs > 0.0 {
                        p.v_sound = Some(vs);
                    }
                    if vp > 0.0 {
                        p.v_seismic_p = Some(vp);
                    }
                    if vss > 0.0 {
                        p.v_seismic_s = Some(vss);
                    }
                    if ath > 0.0 {
                        p.alpha_thermal = Some(ath);
                    }
                    if dd > 0.0 {
                        p.d_diffusion = Some(dd);
                    }
                    if vad > 0.0 {
                        p.v_advective = Some(vad);
                    }
                    p
                }
                None => BodyProperties {
                    α0_deg: 0.0,
                    dα0_dt_deg_per_century: 0.0,
                    δ0_deg: 0.0,
                    dδ0_dt_deg_per_century: 0.0,
                    w0_deg: 0.0,
                    dw_dt_deg_per_day: 0.0,
                    radius_m: 0.0,
                    flattening: 0.0,
                    v_sound: if vs > 0.0 { Some(vs) } else { None },
                    v_seismic_p: if vp > 0.0 { Some(vp) } else { None },
                    v_seismic_s: if vss > 0.0 { Some(vss) } else { None },
                    alpha_thermal: if ath > 0.0 { Some(ath) } else { None },
                    d_diffusion: if dd > 0.0 { Some(dd) } else { None },
                    v_advective: if vad > 0.0 { Some(vad) } else { None },
                },
            });
            pos += 48;
            continue;
        }
        if stype == 3 {
            for _ in 0..gcount {
                if pos + 80 > data.len() {
                    break;
                }
                let f = |i: usize| -> f64 {
                    f64::from_le_bytes(
                        data[pos + i * 8..pos + (i + 1) * 8]
                            .try_into()
                            .unwrap_or([0; 8]),
                    )
                };
                let t0_jd = f(0);
                let m: [f64; 9] = [f(1), f(2), f(3), f(4), f(5), f(6), f(7), f(8), f(9)];
                rotation_matrices.push((t0_jd, m));
                pos += 80;
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
                f64::from_le_bytes(
                    data[pos + i * 8..pos + (i + 1) * 8]
                        .try_into()
                        .unwrap_or([0; 8]),
                )
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
    let f = bp.flattening;
    let e2 = f * (2.0 - f);
    let sl = lr.sin();
    let n = bp.radius_m / (1.0 - e2 * sl * sl).sqrt();
    let xb = (n + alt) * lr.cos() * nr.cos();
    let yb = (n + alt) * lr.cos() * nr.sin();
    let zb = (n * (1.0 - e2) + alt) * sl;
    if !e.rotation_matrices.is_empty() {
        let jd = tdb / 86400.0 + J2000_EPOCH;
        let rot_m = e
            .rotation_matrices
            .iter()
            .min_by(|a, b| {
                (jd - a.0)
                    .abs()
                    .partial_cmp(&(jd - b.0).abs())
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(_, m)| m)?;
        let xi = rot_m[0] * xb + rot_m[1] * yb + rot_m[2] * zb;
        let yi = rot_m[3] * xb + rot_m[4] * yb + rot_m[5] * zb;
        let zi = rot_m[6] * xb + rot_m[7] * yb + rot_m[8] * zb;
        return Some([xi + bx, yi + by, zi + bz]);
    }
    let jd = tdb / 86400.0 + J2000_EPOCH;
    let tc = (jd - J2000_EPOCH) / 36525.0;
    let a = (bp.α0_deg + bp.dα0_dt_deg_per_century * tc).to_radians();
    let d = (bp.δ0_deg + bp.dδ0_dt_deg_per_century * tc).to_radians();
    let w = ((bp.w0_deg + bp.dw_dt_deg_per_day * (jd - J2000_EPOCH))
        - (bp.α0_deg + bp.dα0_dt_deg_per_century * tc))
        .to_radians();
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
            .min_by(|a, b| {
                (jd - a.0)
                    .abs()
                    .partial_cmp(&(jd - b.0).abs())
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(_, m)| m)?;
        let xt = rot_m[0] * rx + rot_m[3] * ry + rot_m[6] * rz;
        let yt = rot_m[1] * rx + rot_m[4] * ry + rot_m[7] * rz;
        let zt = rot_m[2] * rx + rot_m[5] * ry + rot_m[8] * rz;
        (xt, yt, zt)
    } else {
        let bp = e.props.as_ref()?;
        let jd = tdb_secs / 86400.0 + J2000_EPOCH;
        let tc = (jd - J2000_EPOCH) / 36525.0;
        let a = (bp.α0_deg + bp.dα0_dt_deg_per_century * tc).to_radians();
        let d = (bp.δ0_deg + bp.dδ0_dt_deg_per_century * tc).to_radians();
        let w = ((bp.w0_deg + bp.dw_dt_deg_per_day * (jd - J2000_EPOCH))
            - (bp.α0_deg + bp.dα0_dt_deg_per_century * tc))
            .to_radians();
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
type Origin = (u32, i32, i32);

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
struct Sample {
    origin: Origin,
    epoch: f64,
    ttl: f64,
    extent: f64,
    tau: f64,
    force_type: f64,
    vmax: f64,
    amax: f64,
    p0f: [f64; 3],
    motion: Motion,
    fields: Vec<(String, f64)>,
}

struct Family {
    cell_size: f64,
    vmax: f64,
    amax: f64,
    rmax: f64,
    epoch_min: f64,
    cell_lo: CellKey,
    cell_hi: CellKey,
    cells: HashMap<CellKey, Vec<Sample>>,
}

struct Buffer {
    bodies: HashMap<String, Family>,
    inertial: Family,
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

fn build_family(samples: Vec<Sample>, cadence: f64) -> Family {
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

fn build_buffer(samples: Vec<Sample>, cadence: f64) -> Buffer {
    let mut body_samps: HashMap<String, Vec<Sample>> = HashMap::new();
    let mut inertial_samps = Vec::new();
    for s in samples {
        if let Some(body) = s.motion.anchor_body() {
            body_samps.entry(body.to_string()).or_default().push(s);
        } else {
            inertial_samps.push(s);
        }
    }
    let mut bodies = HashMap::new();
    for (name, smps) in body_samps {
        bodies.insert(name, build_family(smps, cadence));
    }
    Buffer {
        bodies,
        inertial: build_family(inertial_samps, cadence),
    }
}

fn force_constants_by_id(id: f64, body_props: Option<&BodyProperties>) -> Option<(f64, bool)> {
    match id as u8 {
        0 | 1 => Some((C_LIGHT, false)),
        2 => body_props.and_then(|p| p.v_sound).map(|v| (v, false)),
        3 => body_props.and_then(|p| p.v_seismic_p).map(|v| (v, false)),
        4 => body_props.and_then(|p| p.v_seismic_s).map(|v| (v, false)),
        5 => body_props.and_then(|p| p.alpha_thermal).map(|v| (v, true)),
        6 => body_props.and_then(|p| p.d_diffusion).map(|v| (v, true)),
        7 => body_props.and_then(|p| p.v_advective).map(|v| (v, false)),
        8 => Some((0.0, false)),
        _ => None,
    }
}

fn enclose_family(
    fam: &Family,
    anchor: [f64; 3],
    center: [f64; 3],
    t2: f64,
    pad: f64,
    records: &mut Vec<(f64, f64, f64, f64, f64, f64, f64, f64, f64, f64)>,
    _frustum: Option<([f64; 3], [f64; 3], [f64; 3], f64, f64)>,
    body_props: Option<&BodyProperties>,
    eph: &HashMap<String, BodyEphemeris>,
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
            let causal_reach = if let Some((v_or_d, is_diff)) =
                force_constants_by_id(smp.force_type, body_props)
            {
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
            if force_constants_by_id(smp.force_type, body_props).is_some() {
                if smp.tau > 0.0 && age > smp.tau * 64.0 {
                    continue;
                }
            }
            let p = match smp.motion.at(t2, smp.epoch, eph) {
                Some(p) => p,
                None => continue,
            };
            let ddx = p[0] - center[0];
            let ddy = p[1] - center[1];
            let ddz = p[2] - center[2];
            let exact = smp.extent + pad;
            let dist2 = ddx * ddx + ddy * ddy + ddz * ddz;
            if dist2 > exact * exact {
                continue;
            }
            if let Some((_, val)) = smp.fields.iter().find(|(n, _)| !n.starts_with('_')) {
                let absorption = force_constants_by_id(smp.force_type, body_props)
                    .map(|(v, _)| if v > 0.0 { 1.0 / v } else { 0.0 })
                    .unwrap_or(0.0);
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
                    absorption,
                ));
            }
        }
    }
}

fn sense_buffer(
    buf: &Buffer,
    center: [f64; 3],
    t2: f64,
    pad: f64,
    records: &mut Vec<(f64, f64, f64, f64, f64, f64, f64, f64, f64, f64)>,
    _frustum: Option<([f64; 3], [f64; 3], [f64; 3], f64, f64)>,
    eph: &HashMap<String, BodyEphemeris>,
) {
    for (body_name, fam) in &buf.bodies {
        let anchor = match body_barycenter_position(body_name, t2, eph) {
            Some(a) => a,
            None => continue,
        };
        let body_props = eph.get(body_name).and_then(|e| e.props.as_ref());
        enclose_family(
            fam, anchor, center, t2, pad, records, _frustum, body_props, eph,
        );
    }
    enclose_family(
        &buf.inertial,
        [0.0, 0.0, 0.0],
        center,
        t2,
        pad,
        records,
        _frustum,
        None,
        eph,
    );
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
    let dt = 0.01;
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
    }
}

fn body_id_to_name(eph: &HashMap<String, BodyEphemeris>, id: u32) -> Option<String> {
    if id == 0 {
        return None;
    }
    let mut keys: Vec<&String> = eph.keys().collect();
    keys.sort();
    keys.get((id - 1) as usize).map(|s| (*s).clone())
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
    }
}

fn tdb_now() -> f64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs_f64()
        - UNIX_J2000_OFFSET
}

fn parse_iso_tdb(s: &str) -> Option<f64> {
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

#[derive(Clone, Default)]
struct OriginState {
    fetched: f64,
    ttl: f64,
    prev_epoch: f64,
    prev_abs: [f64; 3],
    prev_motion: Option<Motion>,
    resid_ema: f64,
    has_prev: bool,
    zero_yield: u32,
    last_body_hash: [u8; 20],
}

struct StationState {
    sample: Option<Sample>,
    buffer: Arc<Buffer>,
    ema_interval: f64,
    last_seen: f64,
}

enum PendingPosition {
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
        vrate: f64,
    },
    StateVector {
        p: [f64; 3],
        v: [f64; 3],
        track: bool,
    },
}

struct PendingSample {
    epoch: f64,
    position: PendingPosition,
    fields: Vec<(String, f64)>,
    extent: Option<f64>,
    ttl: Option<f64>,
    tau: Option<f64>,
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
    // Jina-style metadata headers (Title:, URL Source:, Markdown Content:) precede
    // the payload — jump to the first structural character.
    let start = (0..bytes.len()).find(|&i| bytes[i] == b'{' || bytes[i] == b'[')?;
    let mut p = JsonParser {
        chars: bytes,
        pos: start,
    };
    p.skip_ws();
    p.parse_value()
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
            lat_sign: None,
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

fn jfirst(json: &JsonVal, key: &str) -> Option<f64> {
    if key.contains('.') {
        let (prefix, final_key) = key.rsplit_once('.').unwrap_or(("", key));
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
                            // expand ranges like 0-9, a-z
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
fn parse_quoted_args(s: &str) -> Vec<String> {
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

#[derive(Clone)]
enum Extract {
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
    Regex(String, String),

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
        lat_sign: Option<String>,
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
        fields: Vec<(String, String)>,
    },
    Rows {
        last_line: bool,
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
    XmlCount(String, String),
    Ephemeris(String),
    Vectors(String),
}

fn force_id_of(force: &str) -> Option<u8> {
    match force {
        "em" => Some(0),
        "gravity" => Some(1),
        "acoustic" => Some(2),
        "seismic-body" => Some(3),
        "seismic-surface" => Some(4),
        "thermal" => Some(5),
        "diffusion" => Some(6),
        "advective" => Some(7),
        "biotic" => Some(8),
        _ => None,
    }
}

fn force_type_val(force: &str) -> f64 {
    force_id_of(force).map(|id| id as f64).unwrap_or(0.0)
}

fn force_extent(force: &str) -> f64 {
    match force {
        "em" | "gravity" => f64::INFINITY,
        "seismic-body" => 1e5,
        "seismic-surface" => 1e4,
        "acoustic" => 1e3,
        "advective" => 1e1,
        "biotic" => 1e2,
        "thermal" | "diffusion" => 1e0,
        _ => 0.0,
    }
}

fn force_tau_of_id(id: u8) -> Option<f64> {
    match id {
        8 => Some(86400.0),
        _ => None,
    }
}

fn force_extent_of_id(id: u8) -> Option<f64> {
    match id {
        8 => Some(1e2),
        _ => None,
    }
}

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
}

struct StationEntry {
    id: String,
    lat: f64,
    lon: f64,
}

struct SourceConfig {
    ttl: u64,
    url: String,
    frame: Frame,
    force: String,
    tau: Option<f64>,
    tau_key: Option<String>,
    format: String,
    extracts: Vec<Extract>,
    headers: Vec<(String, String)>,
    post_body: Option<String>,
    method: String,
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
    reach_ttl: Option<u64>,
    catalog_epoch: Option<f64>,
    repeat_ra_bins: u32,
}

struct Archive {
    sources: Vec<SourceConfig>,
    index_html: Vec<u8>,
    constants_js: Vec<u8>,
    body_ephemerides: RwLock<HashMap<String, BodyEphemeris>>,
    field: RwLock<Arc<Buffer>>,
    station: Mutex<StationState>,
    presence: Mutex<HashMap<String, (f64, f64, f64, f64, f64)>>,
    origins: Mutex<HashMap<Origin, OriginState>>,
    ttl_eff: Mutex<HashMap<String, f64>>,
    stations_cache: Mutex<HashMap<String, (Instant, Arc<Vec<StationEntry>>)>>,
    warm_cache_mutex: Mutex<()>,
    warm_cache_cv: Condvar,
}
struct WsFrame {
    opcode: u8,
    payload: Vec<u8>,
}

fn base64_encode(data: &[u8]) -> String {
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
    let _=s.write_all(format!("HTTP/1.1 {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nCache-Control: no-store\r\nConnection: keep-alive\r\n\r\n",st,ct,b.len()).as_bytes());
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

fn fetch_one(
    url: &str,
    body: Option<&str>,
    headers: &[(String, String)],
    ttl: u64,
) -> Option<String> {
    let connect_t = (((ttl as f64) / (Φ * Φ * Φ)).max(1.0) as u64).min(5);
    let max_t = (((ttl as f64) / (Φ * Φ)).max(1.0) as u64).min(120);
    let mut cmd = Command::new("curl");
    cmd.arg("-s")
        .arg("-L")
        .arg("--retry")
        .arg("3")
        .arg("--remove-on-error")
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
        None
    }
}

fn spawn_task_curl(
    batch_dir: &std::path::Path,
    n: usize,
    task: &(
        usize,
        Origin,
        Option<(f64, f64)>,
        String,
        Option<String>,
        Vec<(String, String)>,
        u64,
        String,
    ),
) -> Option<std::process::Child> {
    let (_i, _origin, _region, url, body, headers, ttl, method) = task;
    let mut cmd = Command::new("curl");
    cmd.arg("--remove-on-error");
    cmd.arg("--retry").arg("1");
    cmd.arg("-s");
    cmd.arg("--location");
    cmd.arg("-o")
        .arg(format!("{}/b_{}", batch_dir.display(), n));
    cmd.arg("-D")
        .arg(format!("{}/h_{}", batch_dir.display(), n));

    let connect_t = (((*ttl as f64) / (Φ * Φ * Φ)).max(1.0) as u64).min(5);
    let max_t = (((*ttl as f64) / (Φ * Φ)).max(1.0) as u64).min(45);
    cmd.arg("-m").arg(max_t.to_string());
    cmd.arg("--connect-timeout").arg(connect_t.to_string());

    if let Some(b) = body {
        if method.as_str() == "POST" || method.as_str() == "PUT" {
            cmd.arg("-X").arg(method.as_str());
        } else {
            cmd.arg("-X").arg("POST");
        }
        cmd.arg("-d").arg(b);
    } else if method.as_str() != "GET" && !method.is_empty() {
        cmd.arg("-X").arg(method.as_str());
    }
    for (k, v) in headers {
        cmd.arg("-H").arg(format!("{}: {}", k, v));
    }
    cmd.arg(url);
    cmd.stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .ok()
}

fn handle_ingress(stream: TcpStream, archive: Arc<Archive>) {
    let mut s = stream;
    s.set_nodelay(true).ok();
    let signal = match read_signal(&mut s) {
        Some(r) => r,
        None => return,
    };
    if signal.to_lowercase().contains("upgrade: websocket") {
        resonance(s, &signal, archive);
    } else {
        let mut cur = signal;
        loop {
            let path = parse_path(&cur);
            if path.starts_with("/crash") {
                let body_start = cur.find("\r\n\r\n").map(|i| &cur[i + 4..]).unwrap_or("");
                let log = format!(
                    "[{}] ASYNC_LOG: {}\n",
                    SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs(),
                    body_start.trim()
                );
                println!("{}", log.trim());
                let _ = std::fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open("crash.log")
                    .and_then(|mut f| f.write_all(log.as_bytes()));
                emit(&mut s, "200 OK", "text/plain", b"ok");
            } else {
                match path.as_str() {
                    "/" => {
                        let page = std::fs::read(resolve_asset("static/index.html"))
                            .unwrap_or_else(|_| archive.index_html.clone());
                        emit(&mut s, "200 OK", "text/html", &page);
                    }
                    "/time" => {
                        let unix = SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap_or_default()
                            .as_secs_f64();
                        let tdb = unix - UNIX_J2000_OFFSET;
                        emit(&mut s, "200 OK", "text/plain", tdb.to_string().as_bytes());
                    }
                    "/station" => {
                        let now = tdb_now();
                        let eph_map = archive.body_ephemerides.read().unwrap();
                        let result = archive
                            .station
                            .lock()
                            .unwrap_or_else(|e| e.into_inner())
                            .sample
                            .as_ref()
                            .and_then(|smp| {
                                let p0 = smp.motion.at(now, smp.epoch, &eph_map)?;
                                let p1 = smp.motion.at(now + 1.0, smp.epoch, &eph_map)?;
                                Some((p0, [p1[0] - p0[0], p1[1] - p0[1], p1[2] - p0[2]]))
                            });
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
                        let eph = archive.body_ephemerides.read().unwrap();
                        match body_barycenter_position(body, tdb_now(), &eph) {
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
                        let buf = archive
                            .field
                            .read()
                            .unwrap_or_else(|e| e.into_inner())
                            .clone();
                        let mut report = String::new();
                        let mut families: Vec<(&str, &Family)> =
                            buf.bodies.iter().map(|(k, v)| (k.as_str(), v)).collect();
                        families.push(("inertial", &buf.inertial));
                        for (fname, fam) in families {
                            let mut n = 0usize;
                            let mut field_names: std::collections::HashSet<&str> =
                                std::collections::HashSet::new();
                            for v in fam.cells.values() {
                                for smp in v {
                                    n += 1;
                                    for (k, _) in &smp.fields {
                                        field_names.insert(k.as_str());
                                    }
                                }
                            }
                            report.push_str(&format!(
                                "{} samples={} cells={} rmax={:.3e} vmax={:.3e} epoch_min={:.1}\n",
                                fname,
                                n,
                                fam.cells.len(),
                                fam.rmax,
                                fam.vmax,
                                fam.epoch_min
                            ));
                            let mut names: Vec<&str> = field_names.into_iter().collect();
                            names.sort();
                            report.push_str(&format!("{} fields: {}\n", fname, names.len()));
                            for nm in names {
                                report.push_str(&format!("  {}\n", nm));
                            }
                        }
                        let origins_n = archive
                            .origins
                            .lock()
                            .unwrap_or_else(|e| e.into_inner())
                            .len();
                        let presence_n = archive
                            .presence
                            .lock()
                            .unwrap_or_else(|e| e.into_inner())
                            .len();
                        report
                            .push_str(&format!("origins={} presence={}\n", origins_n, presence_n));
                        emit(&mut s, "200 OK", "text/plain", report.as_bytes());
                    }
                    "/constants.js" => {
                        let mut page = std::fs::read(resolve_asset("static/constants.js"))
                            .unwrap_or_else(|_| archive.constants_js.clone());
                        let mut extra = String::from("\nexport const BODY_REGISTRY = {");
                        let eph = archive.body_ephemerides.read().unwrap();
                        let mut keys: Vec<&String> = eph.keys().collect();
                        keys.sort();
                        for (i, name) in keys.iter().enumerate() {
                            extra.push_str(&format!("{}:\"{}\",", i + 1, name));
                        }
                        extra.push_str("};\n");
                        page.extend_from_slice(extra.as_bytes());
                        emit(&mut s, "200 OK", "application/javascript", &page);
                    }
                    _ => {
                        emit_void(&mut s);
                        break;
                    }
                }
            }
            match read_signal(&mut s) {
                Some(r) => cur = r,
                None => break,
            }
        }
    }
}

fn resonance(mut stream: TcpStream, signal: &str, archive: Arc<Archive>) {
    let key = match extract_header(signal, "Sec-WebSocket-Key") {
        Some(k) => k,
        None => return,
    };
    let encoded = base64_encode(&sha1(
        &format!("{}{}", key, "258EAFA5-E914-47DA-95CA-C5AB0DC85B11").into_bytes(),
    ));
    if stream.write_all(format!("HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Accept: {}\r\n\r\n", encoded).as_bytes()).is_err() { return; }
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

            let mut source_oscillators: Vec<(String, f64)> = Vec::with_capacity(oscillator_count);
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

                    source_oscillators.push((name, value));
                }
            }

            if cursor.read_exact(&mut buf4).is_err() {
                continue;
            }
            let query_count = u32::from_le_bytes(buf4) as usize;

            let now = tdb_now();
            let mut station_fields: Vec<(String, f64)> =
                Vec::with_capacity(source_oscillators.len());
            let (
                mut st_lat,
                mut st_lon,
                mut st_alt,
                mut st_acc,
                mut st_spd,
                mut st_hdg,
                mut st_body,
            ) = (None, None, None, None, None, None, None::<u32>);
            for (name, value) in &source_oscillators {
                match name.as_str() {
                    "lat" => st_lat = Some(*value),
                    "lon" => st_lon = Some(*value),
                    "alt" => st_alt = Some(*value),
                    "acc" => st_acc = Some(*value),
                    "spd" => st_spd = Some(*value),
                    "hdg" => st_hdg = Some(*value),
                    "body" => {
                        st_body = if *value > 0.0 {
                            Some(*value as u32)
                        } else {
                            None
                        }
                    }
                    _ => {}
                }
                station_fields.push((name.clone(), *value));
            }
            let eph_map = archive.body_ephemerides.read().unwrap();
            let body_name = st_body.and_then(|id| body_id_to_name(&eph_map, id));
            let station_buf = {
                let mut station = archive.station.lock().unwrap_or_else(|e| e.into_inner());
                if let (Some(lat), Some(lon), Some(acc), Some(body_name)) =
                    (st_lat, st_lon, st_acc, body_name)
                {
                    if acc > 0.0 {
                        let body_radius_m = eph_map
                            .get(&body_name)
                            .and_then(|e| e.props.as_ref())
                            .map(|p| p.radius_m);
                        if let Some(body_radius_m) = body_radius_m {
                            let dt = if station.last_seen > 0.0 {
                                (now - station.last_seen).abs()
                            } else {
                                0.0
                            };
                            if dt > 0.0 {
                                if station.ema_interval <= 0.0 {
                                    station.ema_interval = dt;
                                } else {
                                    let tau = station.ema_interval;
                                    station.ema_interval += (dt - tau) * (1.0 - (-dt / tau).exp());
                                }
                            }
                            station.last_seen = now;
                            let acc_extent = if acc > 0.0 {
                                2f64.powf(
                                    (acc / (body_radius_m * std::f64::consts::PI / 180.0))
                                        .log2()
                                        .ceil(),
                                )
                            } else {
                                1.0
                            };
                            let alt = st_alt.unwrap_or(0.0);
                            let motion = match (st_spd, st_hdg) {
                                (Some(spd), Some(hdg)) if spd > 0.0 => surface_motion(
                                    &body_name, lat, lon, alt, spd, hdg, 0.0, now, &eph_map,
                                ),
                                _ => Some(Motion::Surface {
                                    body_name,
                                    lat,
                                    lon,
                                    alt,
                                }),
                            };
                            if let Some(motion) = motion {
                                if let Some((vmax, amax, p0f)) =
                                    law_bounds(&motion, now, 0.0, &eph_map)
                                {
                                    let sample = Sample {
                                        origin: (u32::MAX, 0, 0),
                                        epoch: now,
                                        ttl: Φ * Φ * station.ema_interval,
                                        extent: acc_extent,
                                        tau: station.ema_interval,
                                        force_type: force_type_val("em"),
                                        vmax,
                                        amax,
                                        p0f,
                                        motion: motion.clone(),
                                        fields: station_fields,
                                    };
                                    station.buffer = Arc::new(build_buffer(
                                        vec![sample.clone()],
                                        station.ema_interval,
                                    ));
                                    station.sample = Some(sample);
                                }
                            }
                        }
                    }
                }
                Arc::clone(&station.buffer)
            };
            let field = archive
                .field
                .read()
                .unwrap_or_else(|e| e.into_inner())
                .clone();

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
            let mut records: Vec<(f64, f64, f64, f64, f64, f64, f64, f64, f64, f64)> = Vec::new();
            if !queries.is_empty() {
                let (t0, x0, y0, z0) = queries[0];
                let mut extent = 0.0f64;
                for &(_, qx, qy, qz) in &queries[1..] {
                    let d = ((qx - x0).powi(2) + (qy - y0).powi(2) + (qz - z0).powi(2)).sqrt();
                    if d > extent {
                        extent = d;
                    }
                }
                archive
                    .presence
                    .lock()
                    .unwrap_or_else(|e| e.into_inner())
                    .insert(
                        format!("{}_{}_{}", x0 as i64, y0 as i64, z0 as i64),
                        (t0, x0, y0, z0, extent),
                    );
                archive.warm_cache_cv.notify_one();
                let eph_map = archive.body_ephemerides.read().unwrap();
                let center = [x0, y0, z0];
                sense_buffer(&field, center, t0, extent, &mut records, None, &eph_map);
                sense_buffer(
                    &station_buf,
                    center,
                    t0,
                    extent,
                    &mut records,
                    None,
                    &eph_map,
                );
            }

            let mut out = Vec::with_capacity(11 + records.len() * 80);
            out.extend_from_slice(&[0xCF, 0x86]);
            out.push(2u8);
            out.extend_from_slice(&id.to_le_bytes());
            out.extend_from_slice(&(records.len() as u32).to_le_bytes());
            for &(x, y, z, val, extent, epoch, ttl, tau, force_type, absorption) in &records {
                out.extend_from_slice(&x.to_le_bytes());
                out.extend_from_slice(&y.to_le_bytes());
                out.extend_from_slice(&z.to_le_bytes());
                out.extend_from_slice(&val.to_le_bytes());
                out.extend_from_slice(&extent.to_le_bytes());
                out.extend_from_slice(&epoch.to_le_bytes());
                out.extend_from_slice(&ttl.to_le_bytes());
                out.extend_from_slice(&tau.to_le_bytes());
                out.extend_from_slice(&force_type.to_le_bytes());
                out.extend_from_slice(&absorption.to_le_bytes());
            }
            write_ws_binary(&mut stream, &out);
        }
    }
}

fn is_leap(y: u32) -> bool {
    (y % 4 == 0 && y % 100 != 0) || y % 400 == 0
}

fn load_env() {
    for name in &[".env", ".secrets.local"] {
        if let Ok(content) = std::fs::read_to_string(resolve_asset(name)) {
            for line in content.lines() {
                let line = line.trim();
                if line.is_empty() || line.starts_with('#') {
                    continue;
                }
                if let Some(eq) = line.find('=') {
                    let key = line[..eq].trim();
                    let val = line[eq + 1..].trim();
                    if std::env::var(key).is_err() {
                        unsafe {
                            std::env::set_var(key, val);
                        }
                    }
                }
            }
        }
    }
}

fn resolve_secret(url: &str) -> String {
    let mut result = String::with_capacity(url.len());
    let mut rest = url;
    while let Some(start) = rest.find('{') {
        result.push_str(&rest[..start]);
        rest = &rest[start + 1..];
        if let Some(end) = rest.find('}') {
            let key = &rest[..end];
            let val = std::env::var(key).unwrap_or_default();
            result.push_str(&val);
            rest = &rest[end + 1..];
        } else {
            result.push('{');
        }
    }
    result.push_str(rest);
    result
}

fn load_sources() -> Vec<SourceConfig> {
    let mut sources = Vec::new();
    for path in &["phi/sources_cdn.φ", "phi/sources_live.φ"] {
        let content = match std::fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => continue,
        };
        let mut cur_ttl: u64 = 0;
        let mut cur_force = String::new();
        let mut cur_tau: Option<f64> = None;
        let mut cur_tau_key: Option<String> = None;
        let mut cur_url = String::new();
        let mut cur_lat: Option<f64> = None;
        let mut cur_lon: Option<f64> = None;
        let mut cur_alt: f64 = 0.0;
        let mut cur_scale: Option<f64> = None;
        let mut cur_pos: Option<(String, String, Option<String>, f64)> = None;
        let mut cur_format = String::new();
        let mut cur_extracts: Vec<Extract> = Vec::new();
        let mut cur_headers: Vec<(String, String)> = Vec::new();
        let mut cur_target: Option<String> = None;
        let mut cur_catalog: Option<String> = None;
        let mut cur_max_freq: Option<f64> = None;
        let mut cur_min_freq: Option<f64> = None;
        let mut cur_body: Option<String> = None;
        let mut cur_post_body: Option<String> = None;
        let mut cur_method: Option<String> = None;
        let mut cur_stations_url: Option<String> = None;
        let mut cur_stations_path = String::from("stations");
        let mut cur_stations_lat = String::from("lat");
        let mut cur_stations_lon = String::from("lng");
        let mut cur_stations_id = String::from("id");
        let mut cur_flux_from_mag: Option<String> = None;
        let mut cur_abs_mag_from: Option<String> = None;
        let mut cur_reach_ttl: Option<u64> = None;
        let mut cur_catalog_epoch: Option<f64> = None;
        let mut cur_repeat_ra_bins: u32 = 0;
        let mut active = false;

        macro_rules! flush {
        () => {
            if active {
                if cur_force.is_empty() {
                    eprintln!("source refused (no force): {}", cur_url);
                } else if force_id_of(&cur_force).is_none() {
                    eprintln!(
                        "source refused (unknown force '{}'): {}",
                        cur_force, cur_url
                    );
                } else if cur_flux_from_mag.is_some() && cur_abs_mag_from.is_some() {
                    eprintln!(
                        "source refused (flux_from_mag and abs_mag_from are mutually exclusive): {}",
                        cur_url
                    );
                } else {
                    let has_data_position = cur_pos.is_some()
                        || cur_extracts.iter().any(|e| {
                            matches!(
                                e,
                                Extract::Map { .. }
                                    | Extract::GeojsonEvents { .. }
                                    | Extract::CelestialMap { .. }
                                    | Extract::Rows { .. }
                            )
                        });
                    let frame = if let (Some(lat), Some(lon)) = (cur_lat, cur_lon) {
                        let body = cur_body.clone().unwrap_or_else(|| {
                            eprintln!("source refused (on without body): {}", cur_url);
                            String::new()
                        });
                        if body.is_empty() {
                            None
                        } else {
                            Some(Frame::Surface { body_name: body, lat, lon, alt: cur_alt })
                        }
                    } else if let Some(scale) = cur_scale {
                        let body = cur_body.clone().unwrap_or_else(|| {
                            eprintln!("source refused (at without body): {}", cur_url);
                            String::new()
                        });
                        if body.is_empty() {
                            None
                        } else {
                            Some(Frame::Barycenter { body_name: body, scale })
                        }
                    } else if has_data_position {
                        match cur_body.clone() {
                            Some(body) => Some(Frame::Surface {
                                body_name: body,
                                lat: 0.0, lon: 0.0, alt: 0.0,
                            }),
                            None => {
                                eprintln!("source refused (pos without body directive): {}", cur_url);
                                None
                            }
                        }
                    } else if cur_url.contains("{lat}")
                        || cur_url.contains("{lon}")
                        || cur_url.contains("{x}")
                        || cur_url.contains("{y}")
                        || cur_url.contains("{z}")
                        || cur_url.contains("{grid")
                    {
                        match cur_body.clone() {
                            Some(body) => Some(Frame::Surface {
                                body_name: body,
                                lat: 0.0, lon: 0.0, alt: 0.0,
                            }),
                            None => {
                                eprintln!("source refused (template URL without body directive): {}", cur_url);
                                None
                            }
                        }
                    } else {
                        eprintln!("source refused (no reference frame): {}", cur_url);
                        None
                    };
                    if let Some(frame) = frame {
                        if !matches!(frame, Frame::Surface { .. })
                            && cur_extracts
                                .iter()
                                .any(|e| matches!(e, Extract::Map { .. }))
                        {
                            eprintln!(
                                "warning: map extract with non-surface frame: {}",
                                cur_url
                            );
                        }
                        sources.push(SourceConfig {
                            ttl: cur_ttl,
                            url: cur_url.clone(),
                            frame,
                            force: cur_force.clone(),
                            tau: cur_tau,
                            tau_key: cur_tau_key.clone(),
                            format: cur_format.clone(),
                            extracts: cur_extracts.clone(),
                            headers: cur_headers.clone(),
                            post_body: cur_post_body.clone(),
                            method: cur_method.clone().unwrap_or_else(|| "GET".to_string()),
                            target: cur_target.clone(),
                            catalog: cur_catalog.clone(),
                            max_freq: cur_max_freq,
                            min_freq: cur_min_freq,
                            body: cur_body.clone(),
                            stations_url: cur_stations_url.clone(),
                            stations_path: cur_stations_path.clone(),
                            stations_lat: cur_stations_lat.clone(),
                            stations_lon: cur_stations_lon.clone(),
                            stations_id: cur_stations_id.clone(),
                            flux_from_mag: cur_flux_from_mag.clone(),
                            abs_mag_from: cur_abs_mag_from.clone(),
                            reach_ttl: cur_reach_ttl,
                            catalog_epoch: cur_catalog_epoch,
                            repeat_ra_bins: cur_repeat_ra_bins,
                        });
                    }
                }
            }
        };
    }

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') || line.starts_with("```") {
                continue;
            }
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.is_empty() {
                continue;
            }
            match parts[0] {
                "url" => {
                    flush!();
                    cur_url = line.get(4..).unwrap_or("").trim().to_string();
                    cur_ttl = 0;
                    cur_force.clear();
                    cur_tau = None;
                    cur_tau_key = None;
                    cur_lat = None;
                    cur_lon = None;
                    cur_alt = 0.0;
                    cur_scale = None;
                    cur_format.clear();
                    cur_target = None;
                    cur_catalog = None;
                    cur_max_freq = None;
                    cur_min_freq = None;
                    cur_body = None;
                    cur_post_body = None;
                    cur_method = None;
                    cur_stations_url = None;
                    cur_stations_path = String::from("stations");
                    cur_stations_lat = String::from("lat");
                    cur_stations_lon = String::from("lng");
                    cur_stations_id = String::from("id");
                    cur_extracts.clear();
                    cur_pos = None;
                    cur_headers.clear();
                    cur_flux_from_mag = None;
                    cur_abs_mag_from = None;
                    cur_reach_ttl = None;
                    cur_catalog_epoch = None;
                    cur_repeat_ra_bins = 0;
                    active = true;
                }
                "ttl" => cur_ttl = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0),
                "force" => cur_force = parts.get(1).unwrap_or(&"").to_string(),
                "tau" => cur_tau = parts.get(1).and_then(|s| s.parse().ok()),
                "tau_key" => cur_tau_key = parts.get(1).map(|s| s.to_string()),
                "format" => cur_format = parts.get(1).unwrap_or(&"json").to_string(),
                "stations" => cur_stations_url = parts.get(1).map(|s| s.to_string()),
                "stations_path" => {
                    cur_stations_path = parts.get(1).unwrap_or(&"stations").to_string()
                }
                "stations_lat" => cur_stations_lat = parts.get(1).unwrap_or(&"lat").to_string(),
                "stations_lon" => cur_stations_lon = parts.get(1).unwrap_or(&"lng").to_string(),
                "stations_id" => cur_stations_id = parts.get(1).unwrap_or(&"id").to_string(),
                "flux_from_mag" => cur_flux_from_mag = parts.get(1).map(|s| s.to_string()),
                "abs_mag_from" => cur_abs_mag_from = parts.get(1).map(|s| s.to_string()),
                "reach_ttl" => cur_reach_ttl = parts.get(1).and_then(|s| s.parse().ok()),
                "catalog_epoch" => cur_catalog_epoch = parts.get(1).and_then(|s| s.parse().ok()),
                "repeat" => {
                    if parts.len() >= 5 {
                        cur_repeat_ra_bins = parts.get(4).and_then(|s| s.parse().ok()).unwrap_or(0);
                    }
                }
                "target" => cur_target = parts.get(1).map(|s| s.to_string()),
                "catalog" => cur_catalog = parts.get(1).map(|s| s.to_string()),
                "max_freq" => cur_max_freq = parts.get(1).and_then(|s| s.parse().ok()),
                "min_freq" => cur_min_freq = parts.get(1).and_then(|s| s.parse().ok()),
                "body" => cur_body = Some(line.get(5..).unwrap_or("").trim().to_string()),
                "post_body" => {
                    cur_post_body = Some(line.get(10..).unwrap_or("").trim().to_string())
                }
                "method" => cur_method = parts.get(1).map(|s| s.to_string()),
                "verify" => {}
                "on" => {
                    cur_body = Some(parts.get(1).unwrap_or(&"").to_string());
                    cur_lat = parts.get(2).and_then(|s| s.parse().ok());
                    cur_lon = parts.get(3).and_then(|s| s.parse().ok());
                    cur_alt = parts.get(4).and_then(|s| s.parse().ok()).unwrap_or(0.0);
                }
                "at" => {
                    cur_body = Some(parts.get(1).unwrap_or(&"").to_string());
                    cur_scale = parts.get(2).and_then(|s| s.parse().ok());
                }
                "pos" => {
                    if parts.len() >= 3 {
                        cur_pos = Some((
                            parts[1].to_string(),
                            parts[2].to_string(),
                            parts.get(3).map(|s| s.to_string()),
                            parts.get(4).and_then(|s| s.parse().ok()).unwrap_or(1.0),
                        ));
                    }
                }
                "header" => {
                    let rest = line.get(7..).unwrap_or("").trim();
                    if let Some(sp) = rest.find(' ') {
                        cur_headers.push((
                            rest[..sp].to_string(),
                            rest[sp + 1..].trim_matches('"').to_string(),
                        ));
                    }
                }
                "field" => {
                    if parts.len() >= 3 {
                        cur_extracts
                            .push(Extract::Field(parts[1].to_string(), parts[2].to_string()));
                    }
                }
                "first" => {
                    if parts.len() >= 3 {
                        cur_extracts
                            .push(Extract::First(parts[1].to_string(), parts[2].to_string()));
                    }
                }
                "last" => {
                    if parts.len() >= 3 {
                        cur_extracts
                            .push(Extract::Last(parts[1].to_string(), parts[2].to_string()));
                    }
                }
                "count" => {
                    if parts.len() >= 3 {
                        cur_extracts
                            .push(Extract::Count(parts[1].to_string(), parts[2].to_string()));
                    }
                }
                "last_row" => {
                    if parts.len() >= 3 {
                        cur_extracts
                            .push(Extract::LastRow(parts[1].to_string(), parts[2].to_string()));
                    }
                }
                "path" => {
                    if parts.len() >= 3 {
                        cur_extracts
                            .push(Extract::Path(parts[1].to_string(), parts[2].to_string()));
                    }
                }
                "deep" => {
                    if parts.len() >= 3 {
                        cur_extracts
                            .push(Extract::Deep(parts[1].to_string(), parts[2].to_string()));
                    }
                }
                "regex" => {
                    if parts.len() >= 3 {
                        cur_extracts
                            .push(Extract::Regex(parts[1].to_string(), parts[2].to_string()));
                    }
                }
                "last_line" => {
                    if parts.len() >= 2 {
                        cur_extracts.push(Extract::LastLine(parts[1].to_string()));
                    }
                }
                "obj_last" => {
                    if parts.len() >= 3 {
                        cur_extracts
                            .push(Extract::ObjLast(parts[1].to_string(), parts[2].to_string()));
                    }
                }
                "last_obj" => {
                    let quoted = parse_quoted_args(line.get(9..).unwrap_or(""));
                    if quoted.len() >= 4 {
                        cur_extracts.push(Extract::LastObj(
                            quoted[0].clone(),
                            quoted[1].clone(),
                            quoted[2].clone(),
                            quoted[3].clone(),
                        ));
                    }
                }
                "geojson" => {
                    if parts.len() >= 5 && parts[1] == "events" {
                        cur_extracts.push(Extract::GeojsonEvents {
                            mag_key: parts.get(2).unwrap_or(&"mag").to_string(),
                            min_mag: parts.get(3).and_then(|s| s.parse().ok()).unwrap_or(0.0),
                            outputs: parts[4..].iter().map(|s| s.to_string()).collect(),
                        });
                    }
                }
                "map" => {
                    if parts.len() >= 2 {
                        cur_extracts.push(Extract::Map {
                            arr_path: parts[1].to_string(),
                            lat_key: String::new(),
                            lon_key: String::new(),
                            alt_key: String::new(),
                            epoch_key: String::new(),
                            val_key: String::new(),
                            alt_sign: 1.0,
                            vel_key: String::new(),
                            trk_key: String::new(),
                            vr_key: String::new(),
                            fields: Vec::new(),
                            lon_sign: None,
                            lat_sign: None,
                        });
                    }
                }
                "lat_key" => {
                    if let Some(Extract::Map { lat_key, .. }) = cur_extracts.last_mut() {
                        *lat_key = parts.get(1).unwrap_or(&"").to_string();
                    }
                }
                "lon_key" => {
                    if let Some(Extract::Map { lon_key, .. }) = cur_extracts.last_mut() {
                        *lon_key = parts.get(1).unwrap_or(&"").to_string();
                    }
                }
                "alt_key" => {
                    if let Some(e) = cur_extracts.last_mut() {
                        match e {
                            Extract::Map { alt_key, .. } => {
                                *alt_key = parts.get(1).unwrap_or(&"").to_string();
                            }
                            _ => {}
                        }
                    }
                }
                "alt_sign" => {
                    if let Some(Extract::Map { alt_sign, .. }) = cur_extracts.last_mut() {
                        *alt_sign = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(1.0);
                    }
                }
                "vel_key" => {
                    if let Some(Extract::Map { vel_key, .. }) = cur_extracts.last_mut() {
                        *vel_key = parts.get(1).unwrap_or(&"").to_string();
                    }
                }
                "trk_key" => {
                    if let Some(Extract::Map { trk_key, .. }) = cur_extracts.last_mut() {
                        *trk_key = parts.get(1).unwrap_or(&"").to_string();
                    }
                }
                "vr_key" => {
                    if let Some(Extract::Map { vr_key, .. }) = cur_extracts.last_mut() {
                        *vr_key = parts.get(1).unwrap_or(&"").to_string();
                    }
                }
                "lon_sign" => {
                    if let Some(Extract::Map { lon_sign, .. }) = cur_extracts.last_mut() {
                        *lon_sign = parts.get(1).map(|s| s.to_string());
                    }
                }
                "lat_sign" => {
                    if let Some(Extract::Map { lat_sign, .. }) = cur_extracts.last_mut() {
                        *lat_sign = parts.get(1).map(|s| s.to_string());
                    }
                }
                "epoch_key" => {
                    if let Some(e) = cur_extracts.last_mut() {
                        match e {
                            Extract::Map { epoch_key, .. }
                            | Extract::CelestialMap { epoch_key, .. } => {
                                *epoch_key = parts.get(1).unwrap_or(&"").to_string();
                            }
                            _ => {}
                        }
                    }
                }
                "val_key" => {
                    if let Some(e) = cur_extracts.last_mut() {
                        match e {
                            Extract::Map { val_key, .. } => {
                                *val_key = parts.get(1).unwrap_or(&"").to_string();
                            }
                            _ => {}
                        }
                    }
                }
                "tail" => {
                    cur_extracts.push(Extract::Rows {
                        last_line: true,
                        fields: Vec::new(),
                    });
                }
                "rows" => {
                    cur_extracts.push(Extract::Rows {
                        last_line: false,
                        fields: Vec::new(),
                    });
                }
                "field_in" => {
                    if parts.len() >= 3 {
                        match cur_extracts.last_mut() {
                            Some(Extract::Map { fields, .. })
                            | Some(Extract::CelestialMap { fields, .. })
                            | Some(Extract::Rows { fields, .. }) => {
                                fields.push((parts[1].to_string(), parts[2].to_string()));
                            }
                            _ => {}
                        }
                    }
                }
                "cmap" => {
                    if parts.len() >= 2 {
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
                }
                "ra_key" => {
                    if let Some(Extract::CelestialMap { ra_key, .. }) = cur_extracts.last_mut() {
                        *ra_key = parts.get(1).unwrap_or(&"").to_string();
                    }
                }
                "dec_key" => {
                    if let Some(Extract::CelestialMap { dec_key, .. }) = cur_extracts.last_mut() {
                        *dec_key = parts.get(1).unwrap_or(&"").to_string();
                    }
                }
                "dist_key" => {
                    if let Some(Extract::CelestialMap {
                        dist_key,
                        dist_scale,
                        ..
                    }) = cur_extracts.last_mut()
                    {
                        *dist_key = parts.get(1).unwrap_or(&"").to_string();
                        *dist_scale = parts.get(2).and_then(|s| s.parse().ok()).unwrap_or(1.0);
                    }
                }
                "plx_key" => {
                    if let Some(Extract::CelestialMap { plx_key, .. }) = cur_extracts.last_mut() {
                        *plx_key = parts.get(1).unwrap_or(&"").to_string();
                    }
                }
                "z_key" => {
                    if let Some(Extract::CelestialMap { z_key, .. }) = cur_extracts.last_mut() {
                        *z_key = parts.get(1).unwrap_or(&"").to_string();
                    }
                }
                "pmra_key" => {
                    if let Some(Extract::CelestialMap { pmra_key, .. }) = cur_extracts.last_mut() {
                        *pmra_key = parts.get(1).unwrap_or(&"").to_string();
                    }
                }
                "pmdec_key" => {
                    if let Some(Extract::CelestialMap { pmdec_key, .. }) = cur_extracts.last_mut() {
                        *pmdec_key = parts.get(1).unwrap_or(&"").to_string();
                    }
                }
                "radvel_key" => {
                    if let Some(Extract::CelestialMap {
                        rv_key, rv_scale, ..
                    }) = cur_extracts.last_mut()
                    {
                        *rv_key = parts.get(1).unwrap_or(&"").to_string();
                        *rv_scale = parts.get(2).and_then(|s| s.parse().ok()).unwrap_or(1.0);
                    }
                }
                _ => {}
            }
        }
        flush!();
    }
    sources
}

fn parse_path(s: &str) -> String {
    let fl = s.lines().next().unwrap_or("");
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

fn read_ws_frame_raw(stream: &mut TcpStream) -> Option<WsFrame> {
    let mut header = [0u8; 2];
    stream.read_exact(&mut header).ok()?;
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
) -> String {
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

    let (lat, lon) = icrs_to_body_surface(x, y, z, tdb_secs, body_name, eph).unwrap_or((0.0, 0.0));
    let radius_m = eph
        .get(body_name)
        .and_then(|e| e.props.as_ref())
        .map(|p| p.radius_m)
        .unwrap_or(0.0);
    let m_per_deg = if radius_m > 0.0 {
        std::f64::consts::PI * radius_m / 180.0 * lat.to_radians().cos().max(0.0)
    } else {
        0.0
    };
    let res_usize = 6usize;
    let lat_str = format!("{:.6}", lat);
    let lon_str = format!("{:.6}", lon);
    let half_deg = if m_per_deg > 0.0 {
        extent / m_per_deg
    } else {
        0.0
    };
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

    let url = template
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
        .replace("{unix_now_plus_3600}", &unix_now_plus_3600);
    url
}

fn render_source_url(
    src: &SourceConfig,
    x: f64,
    y: f64,
    z: f64,
    tdb: f64,
    r: f64,
    archive: Option<&Archive>,
    eph: &HashMap<String, BodyEphemeris>,
) -> String {
    let mut url = render_url(&src.url, x, y, z, tdb, r, &frame_body_name(&src.frame), eph);
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
    if let Some(ar) = archive {
        if let Some(ref st_url) = src.stations_url {
            let cache_key = st_url.clone();
            let stations = {
                let cache = ar.stations_cache.lock().unwrap_or_else(|e| e.into_inner());
                if let Some((ts, st)) = cache.get(&cache_key) {
                    if ts.elapsed().as_secs() < 86400 {
                        Some(st.clone())
                    } else {
                        None
                    }
                } else {
                    None
                }
            };
            let stations = stations.unwrap_or_else(|| {
                if let Some(body) = fetch_one(st_url, None, &[], 86400) {
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
                            let arc = Arc::new(entries);
                            let mut cache =
                                ar.stations_cache.lock().unwrap_or_else(|e| e.into_inner());
                            cache.insert(cache_key, (Instant::now(), arc.clone()));
                            arc
                        } else {
                            Arc::new(Vec::new())
                        }
                    } else {
                        Arc::new(Vec::new())
                    }
                } else {
                    Arc::new(Vec::new())
                }
            });
            if !stations.is_empty() {
                let (lat, lon) =
                    icrs_to_body_surface(x, y, z, tdb, &frame_body_name(&src.frame), eph)
                        .unwrap_or((0.0, 0.0));
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
    resolve_secret(&url)
}

fn render_source_body(
    src: &SourceConfig,
    x: f64,
    y: f64,
    z: f64,
    tdb: f64,
    r: f64,
    eph: &HashMap<String, BodyEphemeris>,
) -> Option<String> {
    let tmpl = src.post_body.as_ref()?;
    let mut body = render_url(tmpl, x, y, z, tdb, r, &frame_body_name(&src.frame), eph);
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
    Samples(Vec<PendingSample>),
    WithEphemeris(Vec<PendingSample>, BodyEphemeris),
}

fn extract_pending(src: &SourceConfig, body: &str, body_bytes: &[u8], now: f64) -> ExtractResult {
    if src.format == "ephemeris_binary" {
        let mut buf = body_bytes.to_vec();
        if buf.is_empty() {
            if let Ok(mut f) = std::fs::File::open(&src.url) {
                use std::io::Read;
                f.read_to_end(&mut buf).ok();
            }
        }
        if let Some(eph) = parse_ephemeris_binary(&buf) {
            return ExtractResult::WithEphemeris(vec![], eph);
        }
        return ExtractResult::Samples(vec![]);
    }
    let mut pending: Vec<PendingSample> = Vec::new();
    let mut extracted: HashMap<String, f64> = HashMap::new();
    let parsed_json = if src.format == "json" || src.format.is_empty() || src.format == "universal"
    {
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
                let v = if k == "lines" {
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
                if let Some(ref j) = parsed_json {
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
                lat_sign,
            } => {
                let eff_lat_key = lat_key.clone();
                let eff_lon_key = lon_key.clone();
                let eff_epoch_key = epoch_key.clone();
                let body_name = frame_body_name(&src.frame);
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
                                    PendingPosition::SurfaceFlow {
                                        body_name: body_name.clone(),
                                        lat: lat_val,
                                        lon: lon_val,
                                        alt: al,
                                        speed: sp,
                                        track: tr,
                                        vrate: vrate.unwrap_or(0.0),
                                    }
                                } else {
                                    PendingPosition::Surface {
                                        body_name: body_name.clone(),
                                        lat: lat_val,
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
                                    extent: None,
                                    ttl: None,
                                    tau: None,
                                });
                            }
                        }
                    }
                }
            }
            Extract::Rows { last_line, fields } => {
                if let Frame::Surface {
                    ref body_name,
                    lat,
                    lon,
                    alt,
                    ..
                } = src.frame
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
                    if *last_line {
                        let last_data_line = body.lines().rev().find(|line| {
                            let t = line.trim();
                            !t.is_empty() && !t.starts_with('#')
                        });
                        if let Some(line) = last_data_line {
                            let cols = split_data_line(line.trim());
                            let mut ev_fields: Vec<(String, f64)> = Vec::new();
                            for (idx, fn_) in &col_indices {
                                if let Some(val) = cols
                                    .get(*idx)
                                    .and_then(|s| s.trim().trim_matches('"').parse::<f64>().ok())
                                {
                                    ev_fields.push((fn_.to_string(), val));
                                }
                            }
                            if !ev_fields.is_empty() {
                                pending.push(PendingSample {
                                    epoch: now,
                                    position: PendingPosition::Surface {
                                        body_name: body_name.clone(),
                                        lat,
                                        lon,
                                        alt,
                                    },
                                    fields: ev_fields,
                                    extent: None,
                                    ttl: None,
                                    tau: None,
                                });
                            }
                        }
                    } else {
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
                                position: PendingPosition::Surface {
                                    body_name: body_name.clone(),
                                    lat,
                                    lon,
                                    alt,
                                },
                                fields: ev_fields,
                                extent: None,
                                ttl: None,
                                tau: None,
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
                                extent: None,
                                ttl: None,
                                tau: None,
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
                let body_name = frame_body_name(&src.frame);
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
                                                    position: PendingPosition::Surface {
                                                        body_name: body_name.clone(),
                                                        lat: ela,
                                                        lon: elo,
                                                        alt: -ed * 1000.0,
                                                    },
                                                    fields: vec![
                                                        (outputs[0].clone(), mag),
                                                        (outputs[1].clone(), ed * 1000.0),
                                                    ],
                                                    extent: None,
                                                    ttl: None,
                                                    tau: None,
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
            _ => {}
        }
    }
    if !extracted.is_empty() {
        pending.push(PendingSample {
            epoch: now,
            position: PendingPosition::Source,
            fields: extracted.into_iter().collect(),
            extent: None,
            ttl: None,
            tau: None,
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
    ExtractResult::Samples(pending)
}

fn materialize(
    src: &SourceConfig,
    origin: Origin,
    _region: Option<(f64, f64)>,
    pend: PendingSample,
    origins: &mut HashMap<Origin, OriginState>,
    eph: &HashMap<String, BodyEphemeris>,
) -> Vec<Sample> {
    if pend.fields.is_empty() {
        return vec![];
    }
    let vmax_floor = 0.0f64;
    let motion = match &pend.position {
        PendingPosition::StateVector { p, v, .. } => Motion::Linear { p: *p, v: *v },
        PendingPosition::Surface {
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
                return vec![];
            }
            Motion::Surface {
                body_name: body_name.clone(),
                lat: *lat,
                lon: *lon,
                alt: *alt,
            }
        }
        PendingPosition::SurfaceFlow {
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
                return vec![];
            }
            match surface_motion(
                body_name, *lat, *lon, *alt, *speed, *track, *vrate, pend.epoch, eph,
            ) {
                Some(m) => m,
                None => return vec![],
            }
        }
        PendingPosition::Geodetic { lat, lon, alt } => Motion::Surface {
            body_name: "earth".into(),
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
        } => {
            match surface_motion(
                "earth", *lat, *lon, *alt, *speed, *track, *vrate, pend.epoch, eph,
            ) {
                Some(m) => m,
                None => return vec![],
            }
        }
        PendingPosition::Source => match frame_motion(&src.frame, None, None, pend.epoch, eph) {
            Some(m) => m,
            None => return vec![],
        },
    };
    let abs = match motion.at(pend.epoch, pend.epoch, eph) {
        Some(p) => p,
        None => return vec![],
    };
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
                if let Some(pred) = pm.at(pend.epoch, entry.prev_epoch, eph) {
                    let resid = ((pred[0] - abs[0]).powi(2)
                        + (pred[1] - abs[1]).powi(2)
                        + (pred[2] - abs[2]).powi(2))
                    .sqrt();
                    let alpha = 1.0 - (-dt / src.ttl.max(1) as f64).exp();
                    entry.resid_ema += (resid / dt - entry.resid_ema) * alpha;
                }
            }
        }
        resid_ema = entry.resid_ema;
        entry.prev_epoch = pend.epoch;
        entry.prev_abs = abs;
        entry.prev_motion = Some(motion.clone());
        entry.has_prev = true;
    }
    let (mut vmax, amax, p0f) = match law_bounds(&motion, pend.epoch, resid_ema, eph) {
        Some(b) => b,
        None => return vec![],
    };
    if p0f[0].is_nan() || p0f[1].is_nan() || p0f[2].is_nan() || vmax.is_nan() || amax.is_nan() {
        return vec![];
    }
    if vmax_floor > vmax {
        vmax = vmax_floor;
    }
    let body_props = motion
        .anchor_body()
        .and_then(|name| eph.get(name))
        .and_then(|e| e.props.as_ref());
    let forces: Vec<(f64, f64, bool, u8)> = src
        .force
        .split_whitespace()
        .filter_map(|f| {
            let id = force_id_of(f)?;
            let (v_or_d, is_diff) = match id {
                0 | 1 => (C_LIGHT, false),
                _ => force_constants_by_id(id as f64, body_props)?,
            };
            let tau_default = force_tau_of_id(id).unwrap_or(1.0 / v_or_d);
            Some((v_or_d, tau_default, is_diff, id))
        })
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
            let tau = pend.tau.unwrap_or_else(|| {
                src.tau
                    .or_else(|| {
                        src.tau_key.as_ref().and_then(|k| {
                            clean_fields
                                .iter()
                                .find(|(n, _)| n == k)
                                .map(|(_, v)| *v / v_or_d)
                        })
                    })
                    .unwrap_or(tau_default)
            });
            let effective_ttl = pend.ttl.unwrap_or(src.reach_ttl.unwrap_or(src.ttl) as f64);
            let extent = pend.extent.unwrap_or_else(|| {
                force_extent_of_id(force_type).unwrap_or_else(|| {
                    let reach_time = effective_ttl + tau;
                    if is_diff {
                        (2.0 * v_or_d * reach_time).sqrt()
                    } else {
                        v_or_d * reach_time
                    }
                })
            });
            if extent.is_nan() || tau.is_nan() {
                return None;
            }
            Some(Sample {
                origin,
                epoch: pend.epoch,
                ttl: pend.ttl.unwrap_or(src.ttl.max(1) as f64),
                extent,
                tau,
                force_type: force_type as f64,
                vmax,
                amax,
                p0f,
                motion: motion.clone(),
                fields: clean_fields.clone(),
            })
        })
        .collect()
}

fn render_headers(
    src: &SourceConfig,
    x: f64,
    y: f64,
    z: f64,
    now: f64,
    extent: f64,
    eph: &HashMap<String, BodyEphemeris>,
) -> Vec<(String, String)> {
    src.headers
        .iter()
        .map(|(k, v)| {
            (
                k.clone(),
                render_url(v, x, y, z, now, extent, &frame_body_name(&src.frame), eph),
            )
        })
        .collect()
}

fn fetch_priority(
    url: &str,
    pos: (f64, f64, f64),
    r: f64,
    is_new: bool,
    ttl: u64,
    presences: &[(f64, f64, f64, f64, f64)],
) -> u8 {
    if is_new {
        if url.contains("omegaflow/sources") {
            return 0;
        }
        let log_ttl = (ttl.max(1) as f64).log10().min(5.0) / 5.0;
        return 1 + (255.0 * (1.0 - log_ttl)) as u8;
    }
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
    (32.0 + (proximity * 0.7 + urgency * 0.3) * 200.0) as u8
}

fn per_origin_sleep(archive: &Archive, fallback: f64) -> f64 {
    let origins = archive.origins.lock().unwrap_or_else(|e| e.into_inner());
    if origins.is_empty() {
        return fallback;
    }
    let now = tdb_now();
    origins
        .values()
        .map(|o| o.fetched + o.ttl / Φ - now)
        .fold(fallback, f64::min)
        .max(0.1)
}

fn warm_cache(archive: Arc<Archive>) {
    loop {
        let min_ttl = archive.sources.iter().map(|s| s.ttl).min().unwrap_or(60);
        let cadence = ((min_ttl as f64) / Φ).max(1.0);
        let now = tdb_now();
        archive
            .presence
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .retain(|_, (t, _, _, _, _)| (now - *t).abs() < min_ttl as f64 * 64.0);
        let presences: Vec<(f64, f64, f64, f64, f64)> = archive
            .presence
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .values()
            .cloned()
            .collect();
        if presences.is_empty() {
            let lock = archive
                .warm_cache_mutex
                .lock()
                .unwrap_or_else(|e| e.into_inner());
            let still_empty = archive
                .presence
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .is_empty();
            if still_empty {
                let sleep_secs = per_origin_sleep(&archive, cadence);
                let _ = archive
                    .warm_cache_cv
                    .wait_timeout(lock, std::time::Duration::from_secs_f64(sleep_secs));
            }
            continue;
        }

        let mut tasks: Vec<(
            usize,
            Origin,
            Option<(f64, f64)>,
            String,
            Option<String>,
            Vec<(String, String)>,
            u64,
            String,
        )> = Vec::new();
        let mut task_prios: Vec<u8> = Vec::new();
        {
            let ttl_snapshot: HashMap<String, f64> = archive
                .ttl_eff
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .clone();
            let origins_snap: HashMap<Origin, OriginState> = archive
                .origins
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .clone();
            let mut src_order: Vec<(usize, f64, u8)> = archive
                .sources
                .iter()
                .enumerate()
                .map(|(i, s)| {
                    let (fx, fi) = s
                        .force
                        .split_whitespace()
                        .filter_map(|f| force_id_of(f).map(|id| (force_extent(f), id)))
                        .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal))
                        .unwrap_or((0.0, 0));
                    (i, -fx, fi)
                })
                .collect();
            src_order.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap().then(a.2.cmp(&b.2)));
            for (i, _fx, _fi) in src_order {
                let src = &archive.sources[i];
                let eph_map = archive.body_ephemerides.read().unwrap();
                let r = {
                    let mut max_r = 0.0f64;
                    let body_props = eph_map
                        .get(&frame_body_name(&src.frame))
                        .and_then(|e| e.props.as_ref());
                    for f in src.force.split_whitespace() {
                        if let Some(id) = force_id_of(f) {
                            if let Some((v_or_d, is_diff)) =
                                force_constants_by_id(id as f64, body_props)
                            {
                                let tau = src.tau.unwrap_or(1.0 / v_or_d);
                                let effective_ttl = src.reach_ttl.unwrap_or(src.ttl) as f64;
                                let reach_time = effective_ttl + tau;
                                let this_r = if is_diff {
                                    (2.0 * v_or_d * reach_time).sqrt()
                                } else {
                                    v_or_d * reach_time
                                };
                                if this_r > max_r {
                                    max_r = this_r;
                                }
                            }
                        }
                    }
                    max_r
                };
                if r == 0.0 {
                    continue;
                }
                match &src.frame {
                    Frame::Surface { lat, lon, alt, .. } => {
                        let has_template = src.url.contains("{lat}")
                            || src.url.contains("{lon}")
                            || src.url.contains("{x}")
                            || src.url.contains("{y}")
                            || src.url.contains("{z}")
                            || src.url.contains("{grid");
                        let eph_map = archive.body_ephemerides.read().unwrap();
                        if has_template {
                            let cell_size = 2f64.powi((r * 2.0).max(1.0).log2().ceil() as i32);
                            for &(_, px, py, pz, _) in &presences {
                                if !presence_gate(&presences, (px, py, pz), r) {
                                    continue;
                                }
                                let region = icrs_to_body_surface(
                                    px,
                                    py,
                                    pz,
                                    now,
                                    &frame_body_name(&src.frame),
                                    &eph_map,
                                );
                                let (cx, cy, _) = cell_of([px, py, pz], cell_size);
                                let origin = (i as u32, cx as i32, cy as i32);
                                if !origin_stale(
                                    &origins_snap,
                                    origin,
                                    ttl_snapshot
                                        .get(&src.url.split('/').nth(2).unwrap_or("").to_string())
                                        .copied()
                                        .map(|t| t as u64)
                                        .unwrap_or(src.ttl),
                                    now,
                                ) {
                                    continue;
                                }
                                let is_new = origins_snap.get(&origin).is_none();
                                task_prios.push(fetch_priority(
                                    &src.url,
                                    (px, py, pz),
                                    r,
                                    is_new,
                                    src.ttl,
                                    &presences,
                                ));
                                tasks.push((
                                    i,
                                    origin,
                                    region,
                                    render_source_url(
                                        src,
                                        px,
                                        py,
                                        pz,
                                        now,
                                        r,
                                        Some(&*archive),
                                        &eph_map,
                                    ),
                                    render_source_body(src, px, py, pz, now, r, &eph_map),
                                    render_headers(src, px, py, pz, now, r, &eph_map),
                                    src.ttl,
                                    src.method.clone(),
                                ));
                            }
                            continue;
                        }
                        let origin = (i as u32, 0, 0);
                        let eff_ttl = ttl_snapshot
                            .get(&src.url.split('/').nth(2).unwrap_or("").to_string())
                            .copied()
                            .map(|t| t as u64)
                            .unwrap_or(src.ttl);
                        if !origin_stale(&origins_snap, origin, eff_ttl, now) {
                            continue;
                        }
                        let pa = match body_fixed_to_icrs(
                            &frame_body_name(&src.frame),
                            *lat,
                            *lon,
                            *alt,
                            now,
                            &eph_map,
                        ) {
                            Some(p) => p,
                            None => continue,
                        };
                        let pos = (pa[0], pa[1], pa[2]);
                        if !presence_gate(&presences, pos, r) {
                            continue;
                        }
                        let is_new = origins_snap.get(&origin).is_none();
                        task_prios.push(fetch_priority(
                            &src.url, pos, r, is_new, src.ttl, &presences,
                        ));
                        tasks.push((
                            i,
                            origin,
                            None,
                            render_source_url(
                                src,
                                pos.0,
                                pos.1,
                                pos.2,
                                now,
                                r,
                                Some(&*archive),
                                &eph_map,
                            ),
                            render_source_body(src, pos.0, pos.1, pos.2, now, r, &eph_map),
                            render_headers(src, pos.0, pos.1, pos.2, now, r, &eph_map),
                            src.ttl,
                            src.method.clone(),
                        ));
                    }
                    Frame::Barycenter { body_name, scale } => {
                        let origin = (i as u32, 0, 0);
                        if !origin_stale(
                            &origins_snap,
                            origin,
                            ttl_snapshot
                                .get(&src.url.split('/').nth(2).unwrap_or("").to_string())
                                .copied()
                                .map(|t| t as u64)
                                .unwrap_or(src.ttl),
                            now,
                        ) {
                            continue;
                        }
                        let eph_map = archive.body_ephemerides.read().unwrap();
                        let bp = match body_barycenter_position(body_name, now, &eph_map) {
                            Some(b) => b,
                            None => continue,
                        };
                        let pos = (bp[0] * scale, bp[1] * scale, bp[2] * scale);
                        if !presence_gate(&presences, pos, r) {
                            continue;
                        }
                        let is_new = origins_snap.get(&origin).is_none();
                        task_prios.push(fetch_priority(
                            &src.url, pos, r, is_new, src.ttl, &presences,
                        ));
                        tasks.push((
                            i,
                            origin,
                            None,
                            render_source_url(
                                src,
                                pos.0,
                                pos.1,
                                pos.2,
                                now,
                                r,
                                Some(&*archive),
                                &eph_map,
                            ),
                            render_source_body(src, pos.0, pos.1, pos.2, now, r, &eph_map),
                            render_headers(src, pos.0, pos.1, pos.2, now, r, &eph_map),
                            src.ttl,
                            src.method.clone(),
                        ));
                    }
                }
            }
        }

        let (tx, rx) = std::sync::mpsc::sync_channel::<
            Vec<(
                usize,
                Origin,
                Option<(f64, f64)>,
                String,
                Option<String>,
                Vec<(String, String)>,
                u64,
                String,
            )>,
        >(2);
        eprintln!("warm_cache: {} tasks via parallel pool", tasks.len());
        let ar = Arc::clone(&archive);
        let consumer = std::thread::spawn(move || {
            let mut new_samples: Vec<Sample> = Vec::new();
            let mut refreshed: std::collections::HashSet<Origin> = std::collections::HashSet::new();
            let mut swap_acc: usize = 0;
            let mut last_swap_len: usize = 0;
            let pid = std::process::id();
            while let Ok(chunk_tasks) = rx.recv() {
                let batch_dir =
                    std::path::PathBuf::from(format!("/tmp/of_pool_{}_{}", pid, swap_acc));
                std::fs::create_dir_all(&batch_dir).ok();
                let mut children: Vec<(usize, std::process::Child)> = Vec::new();
                let mut next = 0usize;
                while next < chunk_tasks.len() || !children.is_empty() {
                    while children.len() < 32 && next < chunk_tasks.len() {
                        if let Some(c) = spawn_task_curl(&batch_dir, next, &chunk_tasks[next]) {
                            children.push((next, c));
                        }
                        next += 1;
                    }
                    let mut done: Option<(usize, usize)> = None;
                    for (ci, (n, child)) in children.iter_mut().enumerate() {
                        if let Ok(Some(_)) = child.try_wait() {
                            done = Some((ci, *n));
                            break;
                        }
                    }
                    if let Some((ci, n)) = done {
                        let (src_idx, origin, region, url, _body, _headers, ttl, _method) =
                            chunk_tasks[n].clone();
                        let body_file = batch_dir.join(format!("b_{}", n));
                        let hdr_file = batch_dir.join(format!("h_{}", n));
                        let body_raw = std::fs::read_to_string(&body_file).ok();
                        let body_bytes_raw = std::fs::read(&body_file).ok();
                        if let Some(ref hdr) = std::fs::read_to_string(&hdr_file).ok() {
                            for line in hdr.lines() {
                                let line_lower = line.to_lowercase();
                                if let Some(v) = line_lower.strip_prefix("cache-control:") {
                                    for part in v.split(';') {
                                        let p = part.trim();
                                        if let Some(ma) = p.strip_prefix("max-age=") {
                                            if let Ok(max_age) = ma.trim().parse::<f64>() {
                                                let host =
                                                    url.split('/').nth(2).unwrap_or("").to_string();
                                                ar.ttl_eff
                                                    .lock()
                                                    .unwrap_or_else(|e| e.into_inner())
                                                    .insert(host, max_age.max(ttl as f64));
                                            }
                                        }
                                    }
                                }
                                if line_lower.starts_with("retry-after:") {
                                    let val = line.splitn(2, ':').nth(1).unwrap_or("").trim();
                                    if let Ok(ra) = val.parse::<f64>() {
                                        let host = url.split('/').nth(2).unwrap_or("").to_string();
                                        ar.ttl_eff
                                            .lock()
                                            .unwrap_or_else(|e| e.into_inner())
                                            .insert(host, ra.max(ttl as f64));
                                    }
                                }
                            }
                        }
                        let _ = std::fs::remove_file(&body_file);
                        let _ = std::fs::remove_file(&hdr_file);
                        let src = &ar.sources[src_idx];
                        let mut origins = ar.origins.lock().unwrap_or_else(|e| e.into_inner());
                        if body_raw.is_some() || src.format == "ephemeris_binary" {
                            let body = body_raw.as_deref().unwrap_or("");
                            let entry = origins.entry(origin).or_default();
                            entry.fetched = now;
                            entry.ttl = src.ttl as f64;
                            let body_bytes = body.as_bytes();
                            let mut pendings: Option<Vec<PendingSample>> = None;
                            if body_bytes.len() < 50 && src.format != "ephemeris_binary" {
                                entry.zero_yield += 1;
                            } else {
                                let body_hash = sha1(body_bytes);
                                if entry.last_body_hash != [0u8; 20]
                                    && entry.last_body_hash == body_hash
                                {
                                } else {
                                    entry.last_body_hash = body_hash;
                                    let result = extract_pending(
                                        src,
                                        body,
                                        body_bytes_raw.as_deref().unwrap_or(b""),
                                        now,
                                    );
                                    match result {
                                        ExtractResult::WithEphemeris(samples, eph) => {
                                            if let Some(ref bname) = src.body {
                                                ar.body_ephemerides
                                                    .write()
                                                    .unwrap()
                                                    .insert(bname.clone(), eph);
                                            }
                                            pendings = Some(samples);
                                        }
                                        ExtractResult::Samples(samples) => {
                                            pendings = Some(samples);
                                        }
                                    }
                                }
                            }
                            if let Some(pendings) = pendings {
                                let may_be_empty = src.extracts.iter().any(|e| {
                                    matches!(
                                        e,
                                        Extract::Map { .. }
                                            | Extract::GeojsonEvents { .. }
                                            | Extract::CelestialMap { .. }
                                            | Extract::Rows { .. }
                                    )
                                });
                                if pendings.is_empty() {
                                    if !may_be_empty {
                                        let entry = origins.entry(origin).or_default();
                                        entry.zero_yield += 1;
                                        if entry.zero_yield >= 4
                                            && entry.zero_yield & (entry.zero_yield - 1) == 0
                                        {
                                            eprintln!(
                                                "zero_yield x{} {}",
                                                entry.zero_yield,
                                                src.url.split('/').nth(2).unwrap_or("?")
                                            );
                                        }
                                    }
                                } else {
                                    origins.entry(origin).or_default().zero_yield = 0;
                                    let before = new_samples.len();
                                    let eph_map = ar.body_ephemerides.read().unwrap();
                                    for pend in pendings {
                                        for smp in materialize(
                                            src,
                                            origin,
                                            region,
                                            pend,
                                            &mut origins,
                                            &eph_map,
                                        ) {
                                            new_samples.push(smp);
                                        }
                                    }
                                    if new_samples.len() > before {
                                        refreshed.insert(origin);
                                    }
                                }
                            }
                        }
                        drop(origins);
                        children.remove(ci);
                        swap_acc += 1;
                        if new_samples.len() != last_swap_len {
                            last_swap_len = new_samples.len();
                            let station_sample = ar
                                .station
                                .lock()
                                .unwrap_or_else(|e| e.into_inner())
                                .sample
                                .clone()
                                .filter(|s| (now - s.epoch).abs() <= s.ttl * 64.0);
                            let mut all: Vec<Sample> = Vec::new();
                            {
                                let old =
                                    ar.field.read().unwrap_or_else(|e| e.into_inner()).clone();
                                let mut families: Vec<&Family> = old.bodies.values().collect();
                                families.push(&old.inertial);
                                for fam in families {
                                    for v in fam.cells.values() {
                                        for s in v {
                                            if (now - s.epoch).abs() <= s.ttl * 64.0
                                                && !refreshed.contains(&s.origin)
                                            {
                                                all.push(s.clone());
                                            }
                                        }
                                    }
                                }
                            }
                            all.extend(new_samples.iter().cloned());
                            if let Some(ref s) = station_sample {
                                all.push(s.clone());
                            }
                            *ar.field.write().unwrap_or_else(|e| e.into_inner()) =
                                Arc::new(build_buffer(all, cadence));
                        }
                    } else {
                        std::thread::sleep(std::time::Duration::from_millis(15));
                    }
                }
                std::fs::remove_dir_all(&batch_dir).ok();
            }
            (new_samples, refreshed)
        });
        let _ = tx.send(tasks);
        drop(tx);
        let (new_samples, refreshed) = match consumer.join() {
            Ok(v) => v,
            Err(e) => {
                let msg = if let Some(s) = e.downcast_ref::<&str>() {
                    s.to_string()
                } else if let Some(s) = e.downcast_ref::<String>() {
                    s.clone()
                } else {
                    "unknown panic".to_string()
                };
                eprintln!("warm_cache consumer panicked: {}", msg);
                (Vec::new(), std::collections::HashSet::new())
            }
        };

        let station_sample = archive
            .station
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .sample
            .clone()
            .filter(|s| (now - s.epoch).abs() <= s.ttl * 64.0);
        let mut all: Vec<Sample> = Vec::new();
        {
            let old = archive
                .field
                .read()
                .unwrap_or_else(|e| e.into_inner())
                .clone();
            let mut families: Vec<&Family> = old.bodies.values().collect();
            families.push(&old.inertial);
            for fam in families {
                for v in fam.cells.values() {
                    for s in v {
                        if (now - s.epoch).abs() <= s.ttl * 64.0 && !refreshed.contains(&s.origin) {
                            all.push(s.clone());
                        }
                    }
                }
            }
        }
        all.extend(new_samples.iter().cloned());
        if let Some(ref s) = station_sample {
            all.push(s.clone());
        }
        let partial = Arc::new(build_buffer(all, cadence));
        *archive.field.write().unwrap_or_else(|e| e.into_inner()) = partial;

        archive
            .origins
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .retain(|_, o| (now - o.fetched).abs() < o.ttl.max(1.0) * 64.0);
        let sleep_secs = per_origin_sleep(&archive, cadence);
        let lock = archive
            .warm_cache_mutex
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        let _ = archive
            .warm_cache_cv
            .wait_timeout(lock, std::time::Duration::from_secs_f64(sleep_secs));
    }
}

fn main() {
    load_env();
    // reference pre-CDN constants/variants reserved for extraction arms
    {
        let _ = (ECLIPTIC_OBLIQUITY, AU, GAUSS_K);
        let _ = PendingPosition::Geodetic {
            lat: 0.0,
            lon: 0.0,
            alt: 0.0,
        };
        let _ = PendingPosition::GeodeticFlow {
            lat: 0.0,
            lon: 0.0,
            alt: 0.0,
            speed: 0.0,
            track: 0.0,
            vrate: 0.0,
        };
    }
    let loaded = load_sources();
    eprintln!("loaded {} sources from phi/sources_cdn.φ", loaded.len());
    if loaded.is_empty() {
        eprintln!(
            "FATAL: zero sources loaded. Is phi/sources_cdn.φ present? cwd={:?}",
            std::env::current_dir()
        );
    }
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(1111);
    let archive = Arc::new(Archive {
        sources: loaded,
        index_html: std::fs::read(resolve_asset("static/index.html")).unwrap_or_default(),
        constants_js: std::fs::read(resolve_asset("static/constants.js")).unwrap_or_default(),
        body_ephemerides: RwLock::new(HashMap::new()),
        field: RwLock::new(Arc::new(build_buffer(Vec::new(), 1.0))),
        station: Mutex::new(StationState {
            sample: None,
            buffer: Arc::new(build_buffer(Vec::new(), 1.0)),
            ema_interval: 0.0,
            last_seen: 0.0,
        }),
        presence: Mutex::new(HashMap::new()),
        origins: Mutex::new(HashMap::new()),
        ttl_eff: Mutex::new(HashMap::new()),
        stations_cache: Mutex::new(HashMap::new()),
        warm_cache_mutex: Mutex::new(()),
        warm_cache_cv: Condvar::new(),
    });
    {
        archive
            .presence
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .insert("0_0_0".to_string(), (tdb_now(), 0.0, 0.0, 0.0, 1e11));
        let ar = Arc::clone(&archive);
        thread::spawn(move || warm_cache(ar));
    }
    let listener = match TcpListener::bind(format!("127.0.0.1:{}", port)) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("bind 127.0.0.1:{} failed: {}", port, e);
            std::process::exit(1);
        }
    };
    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            let ar = Arc::clone(&archive);
            thread::spawn(move || handle_ingress(stream, ar));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_json, render_source_body, render_source_url, SourceConfig};
    use std::collections::HashMap;

    #[test]
    fn test_parse_json_skips_jina_header() {
        let s = "Title: \n\n\nURL Source: http://api.wheretheiss.at/v1/satellites/25544\n\n\nMarkdown Content:\n{\"name\":\"iss\",\"id\":25544,\"latitude\":-39.79}";
        let v = parse_json(s).unwrap();
        let obj = match v {
            super::JsonVal::Obj(m) => m,
            _ => panic!("expected object"),
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
            force: "em".into(),
            tau: None,
            tau_key: None,
            format: "json".into(),
            extracts: vec![],
            headers: vec![],
            post_body: None,
            method: "GET".to_string(),
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
            reach_ttl: None,
            catalog_epoch: None,
            repeat_ra_bins: 0,
        };
        let url = render_source_url(&src, 0.0, 0.0, 0.0, 0.0, 1000.0, None, &HashMap::new());
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
            force: "em".into(),
            tau: None,
            tau_key: None,
            format: "json".into(),
            extracts: vec![],
            headers: vec![("Content-Type".into(), "application/stac+json".into())],
            post_body: Some("{\"bbox\":[{lon_min},{lat_min},{lon_max},{lat_max}],\"datetime\":\"{today}/{today}\"}".into()),
            method: "GET".to_string(),
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
            reach_ttl: None,
            catalog_epoch: None,
            repeat_ra_bins: 0,
        };
        let body = render_source_body(&src, 0.0, 0.0, 0.0, 0.0, 100000.0, &HashMap::new());
        assert!(body.is_some());
        let b = body.unwrap();
        assert!(b.contains("bbox"));
        assert!(!b.contains("{lon_min}"));
        assert!(!b.contains("{today}"));
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
            force: "thermal".into(),
            tau: None,
            tau_key: None,
            format: "json".into(),
            extracts: vec![super::Extract::Map {
                arr_path: "table.rows".into(),
                lat_key: "2".into(),
                lon_key: "1".into(),
                alt_key: "3".into(),
                epoch_key: "0".into(),
                val_key: String::new(),
                alt_sign: -1.0,
                vel_key: String::new(),
                trk_key: String::new(),
                vr_key: String::new(),
                fields: vec![("4".into(), "argo_temp_c".into())],
                lon_sign: None,
                lat_sign: None,
            }],
            headers: vec![],
            post_body: None,
            method: "GET".into(),
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
            reach_ttl: None,
            catalog_epoch: None,
            repeat_ra_bins: 0,
        };
        let body = r#"{"table":{"columnNames":["time","longitude","latitude","pres","temp"],"columnTypes":["String","double","double","float","float"],"rows":[["2026-07-30T21:40:30Z",-14.408395,34.49025,3.1,23.478],["2026-07-30T22:00:00Z",-12.5,35.0,1000.0,4.681]]}}"#;
        let now = super::tdb_now();
        let expected_epoch = super::parse_iso_tdb("2026-07-30T21:40:30Z").unwrap();
        eprintln!("now={} expected_epoch={}", now, expected_epoch);
        let result = super::extract_pending(&src, body, b"", now);
        let pending = match result {
            super::ExtractResult::Samples(v) => v,
            _ => {
                panic!("expected Samples");
            }
        };
        assert_eq!(pending.len(), 2);
        let p0 = &pending[0];
        assert!(p0.epoch < now);
        let expected_epoch = super::parse_iso_tdb("2026-07-30T21:40:30Z").unwrap();
        assert!((p0.epoch - expected_epoch).abs() < 1e-6);
        match p0.position {
            super::PendingPosition::Surface { lat, lon, alt, .. } => {
                assert!((lat - 34.49025).abs() < 1e-6);
                assert!((lon - -14.408395).abs() < 1e-6);
                assert!((alt - -3.1).abs() < 1e-6);
            }
            _ => panic!("expected Surface position"),
        }
        assert_eq!(p0.fields, vec![("argo_temp_c".to_string(), 23.478)]);
        let p1 = &pending[1];
        match p1.position {
            super::PendingPosition::Surface { alt, .. } => {
                assert!((alt - -1000.0).abs() < 1e-6);
            }
            _ => panic!("expected Surface position"),
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
            _ => panic!("expected CelestialMap"),
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
            _ => panic!("expected Map"),
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
            v_sound: None,
            v_seismic_p: None,
            v_seismic_s: None,
            alpha_thermal: None,
            d_diffusion: None,
            v_advective: None,
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
            v_sound: None,
            v_seismic_p: None,
            v_seismic_s: None,
            alpha_thermal: None,
            d_diffusion: None,
            v_advective: None,
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
            v_sound: None,
            v_seismic_p: None,
            v_seismic_s: None,
            alpha_thermal: None,
            d_diffusion: None,
            v_advective: None,
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
        let eph_fallback = super::BodyEphemeris {
            granules: vec![granule],
            rotation_matrices: vec![],
            props: Some(props),
        };
        let mut map_matrix = HashMap::new();
        map_matrix.insert("mars".to_string(), eph_matrix);
        let mut map_fallback = HashMap::new();
        map_fallback.insert("mars".to_string(), eph_fallback);
        let cases = [(35.0, -15.0, 0.0), (0.0, 90.0, 0.0), (-60.0, 170.0, 5000.0)];
        for (lat, lon, alt) in cases {
            let pm = super::body_fixed_to_icrs("mars", lat, lon, alt, tdb, &map_matrix).unwrap();
            let pf = super::body_fixed_to_icrs("mars", lat, lon, alt, tdb, &map_fallback).unwrap();
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
    fn test_rotation_matrix_empty_fallback() {
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
            v_sound: None,
            v_seismic_p: None,
            v_seismic_s: None,
            alpha_thermal: None,
            d_diffusion: None,
            v_advective: None,
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
        // jfirst / jlast / jdeep / jcount / jpath(negative idx)
        let j = super::parse_json(
            r#"{"data":[{"a":1,"nested":{"b":9}},{"a":2},{"a":3}],"x":[10,20,30]}"#,
        )
        .unwrap();
        assert_eq!(super::jfirst(&j, "data.a"), Some(1.0));
        assert_eq!(super::jlast(&j, "data.a"), Some(3.0));
        assert_eq!(super::jdeep_find_num(&j, "b"), Some(9.0));
        assert_eq!(super::jcount(&j, "data"), Some(3.0));
        assert_eq!(super::jpath(&j, "x.-1"), Some(30.0));
        // 2D array with header row: last row's value for column "a"
        let j2 = super::parse_json(r#"[["t","a"],["x",1],["y",2]]"#).unwrap();
        assert_eq!(super::j2d_last_row(&j2, "a"), Some(2.0));
        // text_last_col: header matched by name, last data row
        let csv = "# time temp\n1 10\n2 20\n";
        assert_eq!(super::text_last_col(csv, "temp"), Some(20.0));
        // std regex engine: '...' shorthand and literal-group patterns work
        assert_eq!(
            super::extract_regex_val(r#"{"totalItems":5,"x":1}"#, r#"("totalItems":...,)"#),
            Some(5.0)
        );
        assert_eq!(
            super::extract_regex_val("<Count>5</Count>", "<Count>([0-9]+)</Count>"),
            Some(5.0)
        );
        // LastLine: last numeric column of last non-comment line
        assert_eq!(
            super::jcount(&super::parse_json(r"[1,2,3]").unwrap(), "."),
            Some(3.0)
        );
    }
}
