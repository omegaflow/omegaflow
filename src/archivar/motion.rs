use super::*;

pub const CHEBYSHEV_N: usize = 18;


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


pub fn chebyshev_evaluate_deriv(coeffs: &[f64; CHEBYSHEV_N], tau: f64) -> f64 {
    let mut dc = [0.0_f64; CHEBYSHEV_N];
    for k in (1..CHEBYSHEV_N).rev() {
        if k + 2 < CHEBYSHEV_N {
            dc[k] += dc[k + 2];
        }
        if k + 1 < CHEBYSHEV_N {
            dc[k] += 2.0 * (k as f64 + 1.0) * coeffs[k + 1];
        }
    }
    dc[0] += coeffs[1];
    if CHEBYSHEV_N >= 3 {
        dc[0] += 0.5 * dc[2];
    }
    chebyshev_evaluate(&dc, tau)
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
    if let Some(orbit) = &e.orbit {
        return crate::wind_orbit::position_at(orbit, tdb).map(|(p, _)| p);
    }
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


pub fn body_barycenter_velocity(
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
            let scale = 1.0 / (g.dt_jd * 86400.0);
            return Some([
                chebyshev_evaluate_deriv(&g.cx, tau) * scale,
                chebyshev_evaluate_deriv(&g.cy, tau) * scale,
                chebyshev_evaluate_deriv(&g.cz, tau) * scale,
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


pub fn finite_pos(p: [f64; 3]) -> Option<[f64; 3]> {
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
    pub orbit: Option<std::sync::Arc<crate::wind_orbit::OrbitRec>>,
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
            orbit: None,
        })
    }
}
