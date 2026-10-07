use super::*;

pub type CellKey = (i64, i64, i64);

const PHASE_PAD: f64 = 0.0;

pub const STAR_SPAN_M: f64 = 1.798012e21;
pub const STAR_CATALOG_COUNT: usize = 1_704_587;
const STAR_OCCUPANCY_TARGET: f64 = 5.0;

type StarFields = (f64, f64, f64, f64, f64, f64, f64, f64, f64);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct StarCellKey {
    pub level: u8,
    pub cell: (i64, i64, i64),
}

const STAR_LEAF_TARGET: usize = 32;
const STAR_MAX_LEVEL: u8 = 40;

pub struct SpatialHash {
    pub cell_size: f64,
    pub anchor_vmax: f64,
    pub anchor_amax: f64,
    pub epoch_min: f64,
    pub cell_lo: CellKey,
    pub cell_hi: CellKey,
    pub cells: HashMap<CellKey, Vec<Arc<Sample>>>,
    pub cell_size_star: f64,
    pub star_cells: HashMap<StarCellKey, Vec<Arc<Sample>>>,
    pub star_lo: StarCellKey,
    pub star_hi: StarCellKey,
    pub star_epoch_min: f64,
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
    pub redshift: f64,
    pub z_kind: u8,
    pub bins: Vec<(f64, f64, f64)>,
}

pub struct Buffer {
    pub cache: SpatialHash,
    pub eph: Arc<HashMap<String, BodyEphemeris>>,
    pub curves: Option<Arc<CurveSet>>,
    pub spectral: Vec<SpectralHash>,
    pub volumes: Vec<crate::archivar::volume::Volume>,
    pub bayestar: Option<Arc<crate::archivar::bayestar::BayestarMap>>,
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
    pub sigma_plx_mas: Option<f64>,
    pub sigma_pm_ra_masyr: Option<f64>,
    pub sigma_pm_de_masyr: Option<f64>,
}

pub struct CurveStar {
    pub ra_deg: f64,
    pub dec_deg: f64,
    pub plx_mas: f64,
    pub cadence: f64,
    pub freq: f64,
    pub bin_width: f64,
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

fn star_cell_at(p: [f64; 3], s: f64, level: u8) -> StarCellKey {
    StarCellKey {
        level,
        cell: (
            (p[0] / s).floor() as i64,
            (p[1] / s).floor() as i64,
            (p[2] / s).floor() as i64,
        ),
    }
}

pub fn star_cell_of(p: [f64; 3], s: f64) -> StarCellKey {
    star_cell_at(p, s, 0)
}

fn star_cell_key(p: [f64; 3], base: f64, level: u8) -> StarCellKey {
    star_cell_at(p, base / (1u64 << level.min(40)) as f64, level)
}

fn star_cell_bounds(ci: i64, base: f64, level: u8) -> (f64, f64) {
    let s = base / (1u64 << level.min(40)) as f64;
    (ci as f64 * s, (ci as f64 + 1.0) * s)
}

fn build_star_leaves(
    stars: Vec<Arc<Sample>>,
    base: f64,
    out: &mut HashMap<StarCellKey, Vec<Arc<Sample>>>,
) {
    let mut by_cell: HashMap<StarCellKey, Vec<Arc<Sample>>> = HashMap::new();
    for s in stars {
        by_cell
            .entry(star_cell_key(s.anchor_p0, base, 0))
            .or_default()
            .push(s);
    }
    for (key, group) in by_cell {
        subdivide_star(group, base, 0, key.cell, out);
    }
}

fn subdivide_star(
    group: Vec<Arc<Sample>>,
    base: f64,
    level: u8,
    cell: (i64, i64, i64),
    out: &mut HashMap<StarCellKey, Vec<Arc<Sample>>>,
) {
    if group.len() <= STAR_LEAF_TARGET || level >= STAR_MAX_LEVEL {
        out.insert(StarCellKey { level, cell }, group);
        return;
    }
    let mut buckets: [Vec<Arc<Sample>>; 8] = std::array::from_fn(|_| Vec::new());
    for s in group {
        let c = star_cell_key(s.anchor_p0, base, level + 1).cell;
        let di = (c.0 - 2 * cell.0) as usize;
        let dj = (c.1 - 2 * cell.1) as usize;
        let dk = (c.2 - 2 * cell.2) as usize;
        buckets[di + 2 * dj + 4 * dk].push(s);
    }
    for (idx, b) in buckets.into_iter().enumerate() {
        if b.is_empty() {
            continue;
        }
        let child = (
            2 * cell.0 + (idx & 1) as i64,
            2 * cell.1 + ((idx >> 1) & 1) as i64,
            2 * cell.2 + ((idx >> 2) & 1) as i64,
        );
        subdivide_star(b, base, level + 1, child, out);
    }
}

fn descend_star_cells<F: FnMut(&Vec<Arc<Sample>>)>(
    hash: &SpatialHash,
    level: u8,
    cell: [i64; 3],
    qlo: [f64; 3],
    qhi: [f64; 3],
    emit: &mut F,
) {
    let [ci, cj, ck] = cell;
    let key = StarCellKey {
        level,
        cell: (ci, cj, ck),
    };
    if let Some(v) = hash.star_cells.get(&key) {
        emit(v);
        return;
    }
    if level >= STAR_MAX_LEVEL {
        return;
    }
    let base = hash.cell_size_star;
    let child = level + 1;
    for di in 0..2i64 {
        for dj in 0..2i64 {
            for dk in 0..2i64 {
                let (ci2, cj2, ck2) = (2 * ci + di, 2 * cj + dj, 2 * ck + dk);
                let (xlo, xhi) = star_cell_bounds(ci2, base, child);
                if xhi < qlo[0] || xlo > qhi[0] {
                    continue;
                }
                let (ylo, yhi) = star_cell_bounds(cj2, base, child);
                if yhi < qlo[1] || ylo > qhi[1] {
                    continue;
                }
                let (zlo, zhi) = star_cell_bounds(ck2, base, child);
                if zhi < qlo[2] || zlo > qhi[2] {
                    continue;
                }
                descend_star_cells(hash, child, [ci2, cj2, ck2], qlo, qhi, emit);
            }
        }
    }
}

pub fn star_cell_size(stars: &[Arc<Sample>]) -> f64 {
    let committed = STAR_SPAN_M * (STAR_OCCUPANCY_TARGET / STAR_CATALOG_COUNT as f64).cbrt();
    if stars.is_empty() {
        return committed;
    }
    let mut span = 0.0f64;
    for k in 0..3 {
        let mut lo = f64::INFINITY;
        let mut hi = f64::NEG_INFINITY;
        for s in stars {
            lo = lo.min(s.anchor_p0[k]);
            hi = hi.max(s.anchor_p0[k]);
        }
        span = span.max(hi - lo);
    }
    if span <= 2.0 * STAR_SPAN_M {
        return committed;
    }
    eprintln!(
        "star grid rehash: the live span {:.3e} m exceeds 2× the committed {:.3e} m — cell_size_star follows the live span for this build; the register carries a new catalog generation",
        span, STAR_SPAN_M
    );
    span * (STAR_OCCUPANCY_TARGET / stars.len() as f64).cbrt()
}

pub fn law_bounds(
    motion: &Motion,
    epoch: f64,
    resid_ema: f64,
    eph: &HashMap<String, BodyEphemeris>,
) -> Option<(f64, f64, [f64; 3])> {
    let p0 = motion.at(epoch, epoch, eph)?;
    if let Motion::Spherical { rec } = motion {
        if !(rec.plx_mas.is_finite() && rec.plx_mas > 0.0) {
            return None;
        }
        let d = ((1000.0 / rec.plx_mas) * PARSEC_M).abs();
        let mu_a = rec.pm_ra_masyr * MAS_YR_TO_RAD_S;
        let mu_dec = rec.pm_de_masyr * MAS_YR_TO_RAD_S;
        let speed = (d * d * (mu_a * mu_a + mu_dec * mu_dec) + rec.rv_m_s * rec.rv_m_s).sqrt();
        let year_s = 86400.0 * 365.25;
        let cos_dec = rec.dec_deg.to_radians().cos().max(1e-6);
        let om_ra = rec.pm_ra_masyr * std::f64::consts::PI / (180.0 * 3.6e6 * cos_dec * year_s);
        let om_dec = rec.pm_de_masyr * std::f64::consts::PI / (180.0 * 3.6e6 * year_s);
        let om = om_ra.abs() + om_dec.abs();
        let accel = 2.0 * rec.rv_m_s.abs() * om + d * om * om;
        return Some((Φ * (speed + resid_ema), Φ * accel, p0));
    }
    let p1 = motion.at(epoch + 1.0, epoch, eph)?;
    let p2 = motion.at(epoch + 2.0, epoch, eph)?;
    let v = ((p1[0] - p0[0]).powi(2) + (p1[1] - p0[1]).powi(2) + (p1[2] - p0[2]).powi(2)).sqrt();
    let a = ((p2[0] - 2.0 * p1[0] + p0[0]).powi(2)
        + (p2[1] - 2.0 * p1[1] + p0[1]).powi(2)
        + (p2[2] - 2.0 * p1[2] + p0[2]).powi(2))
    .sqrt();
    Some((Φ * (v + resid_ema), Φ * a, p0))
}

pub fn law_bounds_over_span(
    motion: &Motion,
    epoch: f64,
    resid_ema: f64,
    span: f64,
    eph: &HashMap<String, BodyEphemeris>,
) -> Option<(f64, f64, [f64; 3])> {
    let p0 = motion.at(epoch, epoch, eph)?;
    if let Motion::Spherical { .. } = motion {
        return law_bounds(motion, epoch, resid_ema, eph);
    }
    let Motion::Kepler { rec } = motion else {
        return law_bounds(motion, epoch, resid_ema, eph);
    };
    let a_m = rec.a_au * crate::kepler::AU_M;
    let e = rec.e;
    if !(a_m.is_finite() && a_m > 0.0) || !(e.is_finite() && (0.0..1.0).contains(&e)) {
        return None;
    }
    let t = std::f64::consts::TAU;
    let n = (crate::kepler::GM_SUN_M3_S2 / a_m.powi(3)).sqrt();
    let period = t / n;
    let m0 = rec.ma_deg.to_radians().rem_euclid(t);
    let to_peri = ((t - m0) % t) / n;
    let d = to_peri.rem_euclid(period);
    let peri_distance = d.min(period - d);
    let peri_in_span = span.is_finite() && peri_distance <= span;
    let mut r_min = if peri_in_span {
        a_m * (1.0 - e)
    } else {
        f64::INFINITY
    };
    for t_end in [epoch - span, epoch + span] {
        let p = motion.at(t_end, epoch, eph)?;
        let r = (p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt();
        if r.is_finite() && r < r_min {
            r_min = r;
        }
    }
    if !(r_min.is_finite() && r_min > 0.0) {
        return None;
    }
    let v_span = (crate::kepler::GM_SUN_M3_S2 * (2.0 / r_min - 1.0 / a_m)).sqrt();
    let a_span = crate::kepler::GM_SUN_M3_S2 / (r_min * r_min);
    Some((Φ * (v_span + resid_ema), Φ * a_span, p0))
}

pub fn build_spatial_hash(samples: Vec<Arc<Sample>>, cadence: f64) -> SpatialHash {
    let mut bounded = Vec::new();
    let mut stars = Vec::new();
    for s in samples {
        if s.extent.is_finite() {
            bounded.push(s);
        } else {
            stars.push(s);
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
    let rho_cad = enclosure_rho(anchor_vmax, anchor_amax, cadence, 0.0);
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
    let mut cells: HashMap<CellKey, Vec<Arc<Sample>>> = HashMap::new();
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
    let mut star_epoch_min = f64::MAX;
    for s in &stars {
        star_epoch_min = star_epoch_min.min(s.epoch);
    }
    let cell_size_star = star_cell_size(&stars);
    let mut star_lo = (i64::MAX, i64::MAX, i64::MAX);
    let mut star_hi = (i64::MIN, i64::MIN, i64::MIN);
    for s in &stars {
        let c = star_cell_of(s.anchor_p0, cell_size_star).cell;
        star_lo.0 = star_lo.0.min(c.0);
        star_lo.1 = star_lo.1.min(c.1);
        star_lo.2 = star_lo.2.min(c.2);
        star_hi.0 = star_hi.0.max(c.0);
        star_hi.1 = star_hi.1.max(c.1);
        star_hi.2 = star_hi.2.max(c.2);
    }
    let mut star_cells: HashMap<StarCellKey, Vec<Arc<Sample>>> = HashMap::new();
    build_star_leaves(stars, cell_size_star, &mut star_cells);
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
        cell_size_star,
        star_cells,
        star_lo: StarCellKey {
            level: 0,
            cell: star_lo,
        },
        star_hi: StarCellKey {
            level: 0,
            cell: star_hi,
        },
        star_epoch_min,
    }
}

pub fn build_buffer(
    samples: Vec<Arc<Sample>>,
    cadence: f64,
    eph: Arc<HashMap<String, BodyEphemeris>>,
    curves: Option<Arc<CurveSet>>,
    spectral: Vec<SpectralHash>,
    volumes: Vec<crate::archivar::volume::Volume>,
    bayestar: Option<Arc<crate::archivar::bayestar::BayestarMap>>,
) -> Buffer {
    Buffer {
        cache: build_spatial_hash(samples, cadence),
        eph,
        curves,
        spectral,
        volumes,
        bayestar,
    }
}

pub fn build_asteroid_samples(bytes: &[u8], ttl: u64) -> Vec<Sample> {
    let eph: HashMap<String, BodyEphemeris> = HashMap::new();
    let mut samples: Vec<Sample> = Vec::new();
    for chunk in bytes.as_chunks::<RECORD_STRIDE>().0 {
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
        let span = 64.0 * ttl as f64;
        let Some((anchor_vmax, anchor_amax, anchor_p0)) =
            law_bounds_over_span(&motion, epoch_secs, 0.0, span, &eph)
        else {
            continue;
        };
        let gm = rec.gm_km3_s2 as f64 * 1.0e9;
        let body_radius_m = rec.radius_km as f64 * 1000.0;
        samples.push(Sample {
            source: SampleSource::Ephemeris,
            epoch: epoch_secs,
            ttl: ttl as f64,
            extent: body_radius_m,
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
            z_flux: SLOT_ABSENT,
            freq: crate::spectral::SPECTRAL_NO_BAND,
            bin_width: crate::spectral::SPECTRAL_NO_BAND,
            color_index: 0.0,
            phase: None,
        });
        if rec.radius_km > 0.0 {
            samples.push(Sample {
                source: SampleSource::Ephemeris,
                epoch: epoch_secs,
                ttl: ttl as f64,
                extent: body_radius_m,
                tau: f64::INFINITY,
                kernel_id: 1.0,
                force_type: 1.0,
                absorption: 0.0,
                advection: 0.0,
                anchor_vmax,
                anchor_amax,
                anchor_p0,
                motion,
                val: body_radius_m,
                name: "dastcom.radius".to_string(),
                z_flux: SLOT_ABSENT,
                freq: crate::spectral::SPECTRAL_NO_BAND,
                bin_width: crate::spectral::SPECTRAL_NO_BAND,
                color_index: 0.0,
                phase: None,
            });
        }
    }
    samples
}

pub const STAR_RECORD_BYTES: usize = 56;
pub const LEGACY_STAR_RECORD_BYTES: usize = 44;

pub fn star_stride(bytes: &[u8]) -> Option<usize> {
    if bytes.is_empty() {
        return None;
    }
    let new = bytes.len().is_multiple_of(STAR_RECORD_BYTES);
    let legacy = bytes.len().is_multiple_of(LEGACY_STAR_RECORD_BYTES);
    match (new, legacy) {
        (true, true) => {
            eprintln!(
                "star bin {} bytes: the size divides both the {} B and the legacy {} B stride — the record width is ambiguous, refused (a legacy asset stays pending recompilation; the compilers withhold the trailing record when the count is a multiple of 11)",
                bytes.len(),
                STAR_RECORD_BYTES,
                LEGACY_STAR_RECORD_BYTES
            );
            None
        }
        (true, false) => Some(STAR_RECORD_BYTES),
        (false, true) => Some(LEGACY_STAR_RECORD_BYTES),
        (false, false) => None,
    }
}

fn star_fields(b: &[u8]) -> Option<StarFields> {
    let ra = f64::from_le_bytes(b[0..8].try_into().ok()?);
    let dec = f64::from_le_bytes(b[8..16].try_into().ok()?);
    let pm_ra = f32::from_le_bytes(b[16..20].try_into().ok()?) as f64;
    let pm_de = f32::from_le_bytes(b[20..24].try_into().ok()?) as f64;
    let plx = f32::from_le_bytes(b[24..28].try_into().ok()?) as f64;
    let mag = f32::from_le_bytes(b[28..32].try_into().ok()?) as f64;
    let flux = f32::from_le_bytes(b[32..36].try_into().ok()?) as f64;
    let color = f32::from_le_bytes(b[36..40].try_into().ok()?) as f64;
    let rv = f32::from_le_bytes(b[40..44].try_into().ok()?) as f64;
    if !ra.is_finite()
        || !dec.is_finite()
        || plx <= 0.0
        || plx.is_nan()
        || !mag.is_finite()
        || !rv.is_finite()
        || !color.is_finite()
    {
        return None;
    }
    Some((ra, dec, pm_ra, pm_de, plx, mag, flux, color, rv))
}

fn sigma_slot(b: &[u8]) -> Option<Option<f64>> {
    let v = f32::from_le_bytes(b.try_into().ok()?);
    if v == 0.0 {
        Some(None)
    } else if v.is_finite() && v > 0.0 {
        Some(Some(v as f64))
    } else {
        None
    }
}

pub fn parse_star_record(b: &[u8]) -> Option<StarRec> {
    match b.len() {
        LEGACY_STAR_RECORD_BYTES => star_fields(b).map(|f| StarRec {
            ra_deg: f.0,
            dec_deg: f.1,
            pm_ra_masyr: f.2,
            pm_de_masyr: f.3,
            plx_mas: f.4,
            mag: f.5,
            flux: f.6,
            color_index: f.7,
            rv_m_s: f.8,
            tau: 0.0,
            sigma_plx_mas: None,
            sigma_pm_ra_masyr: None,
            sigma_pm_de_masyr: None,
        }),
        STAR_RECORD_BYTES => {
            let f = star_fields(b)?;
            let sigma_plx = sigma_slot(b.get(44..48)?)?;
            let sigma_pm_ra = sigma_slot(b.get(48..52)?)?;
            let sigma_pm_de = sigma_slot(b.get(52..56)?)?;
            Some(StarRec {
                ra_deg: f.0,
                dec_deg: f.1,
                pm_ra_masyr: f.2,
                pm_de_masyr: f.3,
                plx_mas: f.4,
                mag: f.5,
                flux: f.6,
                color_index: f.7,
                rv_m_s: f.8,
                tau: 0.0,
                sigma_plx_mas: sigma_plx,
                sigma_pm_ra_masyr: sigma_pm_ra,
                sigma_pm_de_masyr: sigma_pm_de,
            })
        }
        _ => None,
    }
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

pub fn build_star_samples(bytes: &[u8], catalog_epoch_yr: Option<f64>) -> Vec<Sample> {
    let eph: HashMap<String, BodyEphemeris> = HashMap::new();
    let mut samples: Vec<Sample> = Vec::new();
    let Some(epoch_yr) = catalog_epoch_yr else {
        eprintln!(
            "star bin: the block carries no catalog_epoch — stars stay dark (register duty: catalog_epoch <yr>)"
        );
        return samples;
    };
    if !epoch_yr.is_finite() {
        eprintln!(
            "star bin: catalog_epoch reads non-finite — stars stay dark (register duty: catalog_epoch <yr>)"
        );
        return samples;
    }
    let epoch = (epoch_yr - 2000.0) * 86400.0 * 365.25;
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
        let Some((anchor_vmax, anchor_amax, anchor_p0)) = law_bounds(&motion, epoch, 0.0, &eph)
        else {
            continue;
        };
        samples.push(Sample {
            source: SampleSource::Ephemeris,
            epoch,
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
            z_flux: SLOT_ABSENT,
            freq: crate::spectral::SPECTRAL_NO_BAND,
            bin_width: crate::spectral::SPECTRAL_NO_BAND,
            color_index: rec.color_index,
            phase: None,
        });
    }
    samples
}

#[derive(Clone, Copy)]
pub struct MembraneCtx<'a> {
    pub center: [f64; 3],
    pub t2: f64,
    pub pad: f64,
    pub delta_t_cache: f64,
    pub floor: &'a [f64; 9],
    pub softening: f64,
    pub forward: [f64; 3],
    pub eph: &'a HashMap<String, BodyEphemeris>,
}

pub fn query_hash(hash: &SpatialHash, ctx: MembraneCtx<'_>, records: &mut Vec<SampleRecord>) {
    let MembraneCtx {
        center,
        t2,
        pad,
        delta_t_cache,
        floor,
        softening,
        forward,
        eph,
    } = ctx;
    let riss: HashSet<String> = current_riss_names(Some(t2));
    let star_rho = C_LIGHT * ((t2 - hash.star_epoch_min).abs() + delta_t_cache) + pad;
    let mut emit_star = |samples: &Vec<Arc<Sample>>| {
        for sample in samples {
            let ax = sample.anchor_p0[0] - center[0];
            let ay = sample.anchor_p0[1] - center[1];
            let az = sample.anchor_p0[2] - center[2];
            if ax * ax + ay * ay + az * az > star_rho * star_rho {
                continue;
            }
            if riss.contains(&sample.name) {
                continue;
            }
            let age = (t2 - sample.epoch).abs();
            if age > sample.ttl * 64.0 {
                continue;
            }
            if signal_reach(
                sample.force_type,
                sample.advection,
                age,
                sample.freq,
                sample.bin_width,
            )
            .is_none()
            {
                continue;
            }
            let v_prop = match propagation_speed(
                sample.force_type,
                sample.advection,
                sample.freq,
                sample.bin_width,
            ) {
                Some(v) => v,
                None => continue,
            };
            let ft = sample.force_type as u8;
            let floor_ft = if ft < 9 { floor[ft as usize] } else { f64::NAN };
            if !(floor_ft.is_finite() && floor_ft > 0.0) {
                continue;
            }
            let z_aperture = if sample.force_type == 0.0 && slot_measured(sample.z_flux) {
                let z1 = 1.0 + sample.z_flux;
                1.0 / (z1 * z1)
            } else {
                1.0
            };
            let val_max = sample.val.abs() * z_aperture;
            let scale2 = softening * softening;
            let plausible = val_max >= 0.0;
            if !plausible || val_max < floor_ft * scale2 {
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
            if sample.ttl <= 0.0 || sample.ttl.is_nan() {
                continue;
            }
            let retarded = if v_prop > 0.0 && d > 0.0 {
                (age - d / v_prop).max(0.0)
            } else {
                age
            };
            let val_eff = sample.val * (-retarded / sample.ttl).exp() * z_aperture;
            if val_eff.abs() / (transverse2 + scale2) < floor_ft {
                continue;
            }
            let v = match sample.motion.velocity_at(t2) {
                Some(v) => v,
                None => {
                    let p_dt = match sample.motion.at(t2 + 1e-3, sample.epoch, eph) {
                        Some(pd) => pd,
                        None => continue,
                    };
                    [
                        (p_dt[0] - p[0]) / 1e-3,
                        (p_dt[1] - p[1]) / 1e-3,
                        (p_dt[2] - p[2]) / 1e-3,
                    ]
                }
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
                slot_or_pad(sample.absorption),
                slot_or_pad(sample.advection),
                v[0],
                v[1],
                v[2],
                if sample.force_type == 0.0 && slot_measured(sample.z_flux) {
                    sample.z_flux
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
                sample.phase.unwrap_or(PHASE_PAD),
                presence_flags(
                    sample.phase,
                    sample.absorption,
                    sample.advection,
                    sample.z_flux,
                ) + if sample.force_type == FORCE_TYPE_QUANTITY as f64 {
                    PRESENCE_FLAG_QUANTITY
                } else {
                    0.0
                },
            ));
        }
    };
    if !hash.star_cells.is_empty() {
        let s = hash.cell_size_star;
        let qlo = [
            center[0] - star_rho,
            center[1] - star_rho,
            center[2] - star_rho,
        ];
        let qhi = [
            center[0] + star_rho,
            center[1] + star_rho,
            center[2] + star_rho,
        ];
        let lo = (
            ((qlo[0] / s).floor() as i64).max(hash.star_lo.cell.0),
            ((qlo[1] / s).floor() as i64).max(hash.star_lo.cell.1),
            ((qlo[2] / s).floor() as i64).max(hash.star_lo.cell.2),
        );
        let hi = (
            ((qhi[0] / s).floor() as i64).min(hash.star_hi.cell.0),
            ((qhi[1] / s).floor() as i64).min(hash.star_hi.cell.1),
            ((qhi[2] / s).floor() as i64).min(hash.star_hi.cell.2),
        );
        if lo.0 <= hi.0 && lo.1 <= hi.1 && lo.2 <= hi.2 {
            for ci in lo.0..=hi.0 {
                for cj in lo.1..=hi.1 {
                    for ck in lo.2..=hi.2 {
                        descend_star_cells(hash, 0, [ci, cj, ck], qlo, qhi, &mut emit_star);
                    }
                }
            }
        }
    }
    if hash.cells.is_empty() {
        return;
    }
    let qf = center;
    let dt = (t2 - hash.epoch_min).abs() + delta_t_cache;
    let rho = enclosure_rho(hash.anchor_vmax, hash.anchor_amax, dt, pad);
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
    let mut emit = |samples: &Vec<Arc<Sample>>| {
        for sample in samples {
            if riss.contains(&sample.name) {
                continue;
            }
            let age = (t2 - sample.epoch).abs();
            if age > sample.ttl * 64.0 {
                continue;
            }
            let reach_signal = match signal_reach(
                sample.force_type,
                sample.advection,
                age,
                sample.freq,
                sample.bin_width,
            ) {
                Some(r) => r,
                None => continue,
            };
            let future_age = age + delta_t_cache;
            let reach = reach_signal
                + sample.extent
                + enclosure_rho(sample.anchor_vmax, sample.anchor_amax, future_age, pad);
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
            let dist2 = ddx * ddx + ddy * ddy + ddz * ddz;
            if dist2 > reach * reach {
                continue;
            }
            let v = match sample.motion.velocity_at(t2) {
                Some(v) => v,
                None => {
                    let p_dt = match sample.motion.at(t2 + 1e-3, sample.epoch, eph) {
                        Some(pd) => pd,
                        None => continue,
                    };
                    [
                        (p_dt[0] - p[0]) / 1e-3,
                        (p_dt[1] - p[1]) / 1e-3,
                        (p_dt[2] - p[2]) / 1e-3,
                    ]
                }
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
                slot_or_pad(sample.absorption),
                slot_or_pad(sample.advection),
                v[0],
                v[1],
                v[2],
                if sample.force_type == 0.0 && slot_measured(sample.z_flux) {
                    sample.z_flux
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
                sample.phase.unwrap_or(PHASE_PAD),
                presence_flags(
                    sample.phase,
                    sample.absorption,
                    sample.advection,
                    sample.z_flux,
                ) + if sample.force_type == FORCE_TYPE_QUANTITY as f64 {
                    PRESENCE_FLAG_QUANTITY
                } else {
                    0.0
                },
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
    if bytes.len() < 8 {
        return CurveSet { stars };
    }
    let ztf = &bytes[0..4] == b"ZTF1";
    if &bytes[0..4] != b"TSS1" && !ztf {
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
        let (freq, bin_width) = if ztf {
            let Some(f) = take_f64(bytes, &mut off) else {
                return CurveSet { stars };
            };
            let Some(bw) = take_f64(bytes, &mut off) else {
                return CurveSet { stars };
            };
            (f, bw)
        } else {
            (0.0, 0.0)
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
            freq,
            bin_width,
            samples,
        });
    }
    CurveSet { stars }
}

#[cfg(test)]
mod curve_set_tests {
    use super::*;

    #[test]
    fn build_curve_set_parses_ztf1_bands() {
        let curves = vec![crate::ztf::ZtfCurve {
            ra_deg: 210.0,
            dec_deg: 30.0,
            plx_mas: 0.0,
            freq: 6.0e14,
            bin_width: 2.0e14,
            samples: vec![(8.0e8, 1.0e-6), (8.1e8, 2.0e-6), (8.2e8, 3.0e-6)],
        }];
        let bytes = crate::ztf::write_ztf_bin(&curves).unwrap();
        let set = build_curve_set(&bytes);
        assert_eq!(set.stars.len(), 1);
        let s = &set.stars[0];
        assert_eq!(s.plx_mas, 0.0);
        assert_eq!(s.freq, 6.0e14);
        assert_eq!(s.bin_width, 2.0e14);
        assert_eq!(s.samples.len(), 3);
        assert!(s.cadence > 0.0);
    }

    #[test]
    fn build_curve_set_tss1_stays_band_void() {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(b"TSS1");
        bytes.extend_from_slice(&1u32.to_le_bytes());
        bytes.extend_from_slice(&210.0f64.to_le_bytes());
        bytes.extend_from_slice(&30.0f64.to_le_bytes());
        bytes.extend_from_slice(&5.0f64.to_le_bytes());
        bytes.extend_from_slice(&3u32.to_le_bytes());
        for t in [8.0e8f64, 8.1e8, 8.2e8] {
            bytes.extend_from_slice(&t.to_le_bytes());
            bytes.extend_from_slice(&1.0f32.to_le_bytes());
        }
        let set = build_curve_set(&bytes);
        assert_eq!(set.stars.len(), 1);
        assert_eq!(set.stars[0].freq, 0.0);
        assert_eq!(set.stars[0].bin_width, 0.0);
        assert_eq!(set.stars[0].plx_mas, 5.0);
    }
}
