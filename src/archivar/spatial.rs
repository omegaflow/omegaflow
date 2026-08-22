use super::*;

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

pub fn cell_of(p: [f64; 3], s: f64) -> CellKey {
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

pub fn build_spatial_hash(samples: Vec<Sample>, cadence: f64) -> SpatialHash {
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

pub fn build_asteroid_samples(bytes: &[u8], ttl: u64) -> Vec<Sample> {
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

pub const STAR_RECORD_BYTES: usize = 44;

pub fn star_stride(bytes: &[u8]) -> Option<usize> {
    if bytes.len() > 0 && bytes.len() % STAR_RECORD_BYTES == 0 {
        Some(STAR_RECORD_BYTES)
    } else {
        None
    }
}

pub fn parse_star_record(b: &[u8]) -> Option<StarRec> {
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

pub fn build_star_samples(bytes: &[u8]) -> Vec<Sample> {
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

pub fn take_u32(bytes: &[u8], off: &mut usize) -> Option<u32> {
    let raw: [u8; 4] = bytes.get(*off..*off + 4)?.try_into().ok()?;
    *off += 4;
    Some(u32::from_le_bytes(raw))
}

pub fn take_f64(bytes: &[u8], off: &mut usize) -> Option<f64> {
    let raw: [u8; 8] = bytes.get(*off..*off + 8)?.try_into().ok()?;
    *off += 8;
    Some(f64::from_le_bytes(raw))
}

pub fn take_f32(bytes: &[u8], off: &mut usize) -> Option<f32> {
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
