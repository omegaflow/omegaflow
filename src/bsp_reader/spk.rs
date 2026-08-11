
use std::collections::HashMap;
use std::path::Path;

use super::daf::{DafError, DafFile};

#[derive(Debug)]
pub enum SpkError {
    Daf(DafError),
    NoCoverage { target: i32, center: i32, et: f64 },
    UnsupportedType(i32),
    BadType2(&'static str),
}

impl std::fmt::Display for SpkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpkError::Daf(e) => write!(f, "{e}"),
            SpkError::NoCoverage { target, center, et } => {
                write!(
                    f,
                    "no segment covers target {target} wrt center {center} at et {et}"
                )
            }
            SpkError::UnsupportedType(t) => write!(f, "unsupported SPK data type {t}"),
            SpkError::BadType2(msg) => write!(f, "malformed Type 2 segment: {msg}"),
        }
    }
}

impl From<DafError> for SpkError {
    fn from(e: DafError) -> Self {
        SpkError::Daf(e)
    }
}

impl std::error::Error for SpkError {}

#[derive(Clone)]
pub struct SpkSegment {
    pub target: i32,
    pub center: i32,
    pub frame: i32,
    pub data_type: i32,
    pub start_et: f64,
    pub end_et: f64,
    pub start_addr: u32,
    pub end_addr: u32,
    pub name: String,
    payload: SpkPayload,
}

#[derive(Clone)]
enum SpkPayload {
    Type2(SpkType2),
    Type3(SpkType3),
    Type9(SpkType9),
    Type13(SpkType13),
    Unsupported,
}

#[derive(Clone)]
struct SpkType2 {
    file: DafFile,
    init: f64,
    intlen: f64,
    rsize: usize,
    n_records: usize,
    n_coef: usize,
    start_addr: u32,
}

impl SpkType2 {
    fn from_segment(file: &DafFile, start_addr: u32, end_addr: u32) -> Result<Self, SpkError> {
        let trailer = file.read_doubles(end_addr - 3, end_addr)?;
        let init = trailer[0];
        let intlen = trailer[1];
        let rsize = trailer[2] as usize;
        let n_records = trailer[3] as usize;
        if rsize < 2 || (rsize - 2) % 3 != 0 {
            return Err(SpkError::BadType2("RSIZE not 2 + 3N"));
        }
        let n_coef = (rsize - 2) / 3;
        if n_coef == 0 {
            return Err(SpkError::BadType2("degree < 0"));
        }
        if intlen <= 0.0 {
            return Err(SpkError::BadType2("INTLEN <= 0"));
        }
        Ok(SpkType2 {
            file: file.clone(),
            init,
            intlen,
            rsize,
            n_records,
            n_coef,
            start_addr,
        })
    }

    fn evaluate(&self, et: f64) -> Result<[f64; 6], SpkError> {
        let raw_idx = ((et - self.init) / self.intlen).floor() as isize;
        let idx = raw_idx.clamp(0, self.n_records as isize - 1) as usize;
        let rec_start = self.start_addr + (idx * self.rsize) as u32;
        let rec_end = rec_start + self.rsize as u32 - 1;
        let rec = self.file.doubles_native(rec_start, rec_end)?;

        let mid = rec[0];
        let radius = rec[1];
        if radius == 0.0 {
            return Err(SpkError::BadType2("RADIUS == 0"));
        }
        let s = (et - mid) / radius;

        let n = self.n_coef;
        let xc = &rec[2..2 + n];
        let yc = &rec[2 + n..2 + 2 * n];
        let zc = &rec[2 + 2 * n..2 + 3 * n];

        let (pos, vel) = cheby3_val_and_deriv(xc, yc, zc, s);
        let inv_r = 1.0 / radius;
        Ok([
            pos[0],
            pos[1],
            pos[2],
            vel[0] * inv_r,
            vel[1] * inv_r,
            vel[2] * inv_r,
        ])
    }
}

#[derive(Clone)]
struct SpkType3 {
    file: DafFile,
    init: f64,
    intlen: f64,
    rsize: usize,
    n_records: usize,
    n_coef: usize,
    start_addr: u32,
}

impl SpkType3 {
    fn from_segment(file: &DafFile, start_addr: u32, end_addr: u32) -> Result<Self, SpkError> {
        let trailer = file.read_doubles(end_addr - 3, end_addr)?;
        let init = trailer[0];
        let intlen = trailer[1];
        let rsize = trailer[2] as usize;
        let n_records = trailer[3] as usize;
        if rsize < 2 || (rsize - 2) % 6 != 0 {
            return Err(SpkError::BadType2("RSIZE not 2 + 6N"));
        }
        let n_coef = (rsize - 2) / 6;
        if n_coef == 0 || intlen <= 0.0 {
            return Err(SpkError::BadType2("degree<0 or INTLEN<=0"));
        }
        Ok(SpkType3 {
            file: file.clone(),
            init,
            intlen,
            rsize,
            n_records,
            n_coef,
            start_addr,
        })
    }

    fn evaluate(&self, et: f64) -> Result<[f64; 6], SpkError> {
        let raw_idx = ((et - self.init) / self.intlen).floor() as isize;
        let idx = raw_idx.clamp(0, self.n_records as isize - 1) as usize;
        let rec_start = self.start_addr + (idx * self.rsize) as u32;
        let rec_end = rec_start + self.rsize as u32 - 1;
        let rec = self.file.doubles_native(rec_start, rec_end)?;

        let mid = rec[0];
        let radius = rec[1];
        let s = (et - mid) / radius;
        let n = self.n_coef;
        let xc = &rec[2..2 + n];
        debug_assert_eq!(rec.len(), self.rsize);
        let yc = &rec[2 + n..2 + 2 * n];
        let zc = &rec[2 + 2 * n..2 + 3 * n];
        let vxc = &rec[2 + 3 * n..2 + 4 * n];
        let vyc = &rec[2 + 4 * n..2 + 5 * n];
        let vzc = &rec[2 + 5 * n..2 + 6 * n];
        let pos = cheby3_val_only(xc, yc, zc, s);
        let vel = cheby3_val_only(vxc, vyc, vzc, s);
        Ok([pos[0], pos[1], pos[2], vel[0], vel[1], vel[2]])
    }
}

struct DiscreteMeta {
    window_or_degree: usize,
    n_states: usize,
    states_start: u32,
    epochs_start: u32,
}

impl DiscreteMeta {
    fn from_segment(file: &DafFile, start_addr: u32, end_addr: u32) -> Result<Self, SpkError> {
        let tail = file.read_doubles(end_addr - 1, end_addr)?;
        let window_or_degree = tail[0] as usize;
        let n_states = tail[1] as usize;
        if n_states == 0 {
            return Err(SpkError::BadType2("empty discrete-state segment"));
        }
        let n_dir = n_states / 100;
        let states_start = start_addr;
        let epochs_start = start_addr + (6 * n_states) as u32;
        let expected_end = epochs_start + n_states as u32 + n_dir as u32 + 2 - 1;
        if expected_end != end_addr {
            return Err(SpkError::BadType2("segment size does not match trailer"));
        }
        Ok(DiscreteMeta {
            window_or_degree,
            n_states,
            states_start,
            epochs_start,
        })
    }
}

#[derive(Clone)]
struct SpkType9 {
    file: DafFile,
    meta_degree: usize,
    n_states: usize,
    states_start: u32,
    epochs_start: u32,
}

impl SpkType9 {
    fn from_segment(file: &DafFile, start_addr: u32, end_addr: u32) -> Result<Self, SpkError> {
        let meta = DiscreteMeta::from_segment(file, start_addr, end_addr)?;
        Ok(SpkType9 {
            file: file.clone(),
            meta_degree: meta.window_or_degree,
            n_states: meta.n_states,
            states_start: meta.states_start,
            epochs_start: meta.epochs_start,
        })
    }

    fn evaluate(&self, et: f64) -> Result<[f64; 6], SpkError> {
        let window = self.meta_degree + 1;
        let (i0, count) = pick_window(&self.file, self.epochs_start, self.n_states, window, et)?;
        let epochs = self.file.doubles_native(
            self.epochs_start + i0 as u32,
            self.epochs_start + (i0 + count - 1) as u32,
        )?;
        let states = self.file.doubles_native(
            self.states_start + (6 * i0) as u32,
            self.states_start + (6 * (i0 + count) - 1) as u32,
        )?;
        let mut out = [0.0_f64; 6];
        let mut comp = vec![0.0_f64; count];
        for k in 0..6 {
            for j in 0..count {
                comp[j] = states[6 * j + k];
            }
            out[k] = lagrange_eval(&epochs, &comp, et);
        }
        Ok(out)
    }
}

#[derive(Clone)]
struct SpkType13 {
    file: DafFile,
    window_size: usize,
    n_states: usize,
    states_start: u32,
    epochs_start: u32,
}

impl SpkType13 {
    fn from_segment(file: &DafFile, start_addr: u32, end_addr: u32) -> Result<Self, SpkError> {
        let meta = DiscreteMeta::from_segment(file, start_addr, end_addr)?;
        let window_size = meta.window_or_degree + 1;
        if window_size < 2 {
            return Err(SpkError::BadType2("Type 13 window size < 2"));
        }
        Ok(SpkType13 {
            file: file.clone(),
            window_size,
            n_states: meta.n_states,
            states_start: meta.states_start,
            epochs_start: meta.epochs_start,
        })
    }

    fn evaluate(&self, et: f64) -> Result<[f64; 6], SpkError> {
        let (i0, count) = pick_window(
            &self.file,
            self.epochs_start,
            self.n_states,
            self.window_size,
            et,
        )?;
        let epochs = self.file.doubles_native(
            self.epochs_start + i0 as u32,
            self.epochs_start + (i0 + count - 1) as u32,
        )?;
        let states = self.file.doubles_native(
            self.states_start + (6 * i0) as u32,
            self.states_start + (6 * (i0 + count) - 1) as u32,
        )?;
        let mut out = [0.0_f64; 6];
        let mut pos_vals = vec![0.0_f64; count];
        let mut vel_vals = vec![0.0_f64; count];
        for axis in 0..3 {
            for j in 0..count {
                pos_vals[j] = states[6 * j + axis];
                vel_vals[j] = states[6 * j + 3 + axis];
            }
            let (p, v) = hermite_eval(&epochs, &pos_vals, &vel_vals, et);
            out[axis] = p;
            out[3 + axis] = v;
        }
        Ok(out)
    }
}

fn pick_window(
    file: &DafFile,
    epochs_start: u32,
    n_states: usize,
    window: usize,
    et: f64,
) -> Result<(usize, usize), SpkError> {
    if window == 0 {
        return Err(SpkError::BadType2("window size 0"));
    }
    let count = window.min(n_states);
    let epochs = file.doubles_native(epochs_start, epochs_start + n_states as u32 - 1)?;
    let mut lo = 0usize;
    let mut hi = n_states;
    while lo < hi {
        let mid = (lo + hi) / 2;
        if epochs[mid] <= et {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    let left_idx = lo.saturating_sub(1);
    let half = (count - 1) / 2;
    let start = left_idx.saturating_sub(half);
    let start = start.min(n_states - count);
    Ok((start, count))
}

fn lagrange_eval(xs: &[f64], ys: &[f64], x: f64) -> f64 {
    let n = xs.len();
    for i in 0..n {
        if xs[i] == x {
            return ys[i];
        }
    }
    let mut w = vec![1.0_f64; n];
    for i in 0..n {
        for j in 0..n {
            if i != j {
                w[i] *= xs[i] - xs[j];
            }
        }
        w[i] = 1.0 / w[i];
    }
    let mut num = 0.0_f64;
    let mut den = 0.0_f64;
    for i in 0..n {
        let term = w[i] / (x - xs[i]);
        num += term * ys[i];
        den += term;
    }
    num / den
}

fn hermite_eval(xs: &[f64], ys: &[f64], dys: &[f64], x: f64) -> (f64, f64) {
    let n = xs.len();
    debug_assert_eq!(ys.len(), n);
    debug_assert_eq!(dys.len(), n);
    let m = 2 * n;
    let mut z = vec![0.0_f64; m];
    let mut f = vec![0.0_f64; m];
    for i in 0..n {
        z[2 * i] = xs[i];
        z[2 * i + 1] = xs[i];
        f[2 * i] = ys[i];
        f[2 * i + 1] = ys[i];
    }
    let prev = f.clone();
    let mut col = vec![0.0_f64; m];
    for i in 0..m - 1 {
        if (i % 2) == 0 {
            col[i] = dys[i / 2];
        } else {
            col[i] = (prev[i + 1] - prev[i]) / (z[i + 1] - z[i]);
        }
    }
    let mut coeffs = vec![0.0_f64; m];
    coeffs[0] = f[0];
    coeffs[1] = col[0];
    let mut cur = col.clone();
    for k in 2..m {
        let mut next = vec![0.0_f64; m - k];
        for i in 0..m - k {
            next[i] = (cur[i + 1] - cur[i]) / (z[i + k] - z[i]);
        }
        coeffs[k] = next[0];
        cur = next;
    }
    let mut val = coeffs[0];
    let mut der = 0.0_f64;
    let mut prod = 1.0_f64;
    let mut dprod = 0.0_f64;
    for k in 1..m {
        let dprod_new = prod + (x - z[k - 1]) * dprod;
        let prod_new = (x - z[k - 1]) * prod;
        val += coeffs[k] * prod_new;
        der += coeffs[k] * dprod_new;
        prod = prod_new;
        dprod = dprod_new;
    }
    (val, der)
}

#[inline]
pub(crate) fn cheby3_val_and_deriv(
    cx: &[f64],
    cy: &[f64],
    cz: &[f64],
    s: f64,
) -> ([f64; 3], [f64; 3]) {
    let n = cx.len();
    debug_assert_eq!(cy.len(), n);
    debug_assert_eq!(cz.len(), n);
    if n == 0 {
        return ([0.0; 3], [0.0; 3]);
    }
    if n == 1 {
        return ([cx[0], cy[0], cz[0]], [0.0; 3]);
    }
    let mut t_prev = 1.0;
    let mut t_curr = s;
    let mut dt_prev = 0.0_f64;
    let mut dt_curr = 1.0;
    let mut val = [
        cx[0] * t_prev + cx[1] * t_curr,
        cy[0] * t_prev + cy[1] * t_curr,
        cz[0] * t_prev + cz[1] * t_curr,
    ];
    let mut der = [cx[1] * dt_curr, cy[1] * dt_curr, cz[1] * dt_curr];
    let two_s = 2.0 * s;
    for k in 2..n {
        let t_next = two_s * t_curr - t_prev;
        let dt_next = 2.0 * t_curr + two_s * dt_curr - dt_prev;
        val[0] += cx[k] * t_next;
        val[1] += cy[k] * t_next;
        val[2] += cz[k] * t_next;
        der[0] += cx[k] * dt_next;
        der[1] += cy[k] * dt_next;
        der[2] += cz[k] * dt_next;
        t_prev = t_curr;
        t_curr = t_next;
        dt_prev = dt_curr;
        dt_curr = dt_next;
    }
    (val, der)
}

#[inline]
pub(crate) fn cheby3_val_only(cx: &[f64], cy: &[f64], cz: &[f64], s: f64) -> [f64; 3] {
    let n = cx.len();
    debug_assert_eq!(cy.len(), n);
    debug_assert_eq!(cz.len(), n);
    if n == 0 {
        return [0.0; 3];
    }
    if n == 1 {
        return [cx[0], cy[0], cz[0]];
    }
    let mut t_prev = 1.0;
    let mut t_curr = s;
    let mut val = [
        cx[0] * t_prev + cx[1] * t_curr,
        cy[0] * t_prev + cy[1] * t_curr,
        cz[0] * t_prev + cz[1] * t_curr,
    ];
    let two_s = 2.0 * s;
    for k in 2..n {
        let t_next = two_s * t_curr - t_prev;
        val[0] += cx[k] * t_next;
        val[1] += cy[k] * t_next;
        val[2] += cz[k] * t_next;
        t_prev = t_curr;
        t_curr = t_next;
    }
    val
}

pub struct SpkFile {
    segments: Vec<SpkSegment>,
    index: HashMap<(i32, i32), Vec<usize>>,
}

impl SpkFile {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, SpkError> {
        let daf = DafFile::open(path)?;
        Self::from_daf(daf)
    }

    pub fn from_daf(daf: DafFile) -> Result<Self, SpkError> {
        let mut segments = Vec::new();
        let mut index: HashMap<(i32, i32), Vec<usize>> = HashMap::new();
        for summary in daf.summaries()? {
            if summary.doubles.len() < 2 || summary.integers.len() < 6 {
                continue;
            }
            let start_et = summary.doubles[0];
            let end_et = summary.doubles[1];
            let target = summary.integers[0];
            let center = summary.integers[1];
            let frame = summary.integers[2];
            let data_type = summary.integers[3];
            let start_addr = summary.integers[4] as u32;
            let end_addr = summary.integers[5] as u32;

            let payload = match data_type {
                2 => SpkPayload::Type2(SpkType2::from_segment(&daf, start_addr, end_addr)?),
                3 => SpkPayload::Type3(SpkType3::from_segment(&daf, start_addr, end_addr)?),
                9 => SpkPayload::Type9(SpkType9::from_segment(&daf, start_addr, end_addr)?),
                13 => SpkPayload::Type13(SpkType13::from_segment(&daf, start_addr, end_addr)?),
                _ => SpkPayload::Unsupported,
            };

            index
                .entry((target, center))
                .or_default()
                .push(segments.len());
            segments.push(SpkSegment {
                target,
                center,
                frame,
                data_type,
                start_et,
                end_et,
                start_addr,
                end_addr,
                name: summary.name,
                payload,
            });
        }

        Ok(SpkFile { segments, index })
    }

    pub fn segments(&self) -> &[SpkSegment] {
        &self.segments
    }

    #[inline]
    pub fn state(&self, target: i32, center: i32, et: f64) -> Result<[f64; 6], SpkError> {
        if target == center {
            return Ok([0.0; 6]);
        }

        if let Some(s) = self.try_direct(target, center, et)? {
            return Ok(s);
        }
        self.state_via_ssb_chain(target, center, et)
    }

    #[cold]
    #[inline(never)]
    fn state_via_ssb_chain(&self, target: i32, center: i32, et: f64) -> Result<[f64; 6], SpkError> {
        let t_ssb = self.state_wrt_ssb(target, et)?;
        let c_ssb = self.state_wrt_ssb(center, et)?;
        let mut out = [0.0_f64; 6];
        for i in 0..6 {
            out[i] = t_ssb[i] - c_ssb[i];
        }
        Ok(out)
    }

    #[inline]
    fn try_direct(&self, target: i32, center: i32, et: f64) -> Result<Option<[f64; 6]>, SpkError> {
        if let Some(indices) = self.index.get(&(target, center)) {
            for &i in indices {
                let seg = &self.segments[i];
                if et >= seg.start_et && et <= seg.end_et {
                    return Ok(Some(Self::eval_segment(seg, et)?));
                }
            }
        }
        if let Some(indices) = self.index.get(&(center, target)) {
            for &i in indices {
                let seg = &self.segments[i];
                if et >= seg.start_et && et <= seg.end_et {
                    let mut s = Self::eval_segment(seg, et)?;
                    for v in s.iter_mut() {
                        *v = -*v;
                    }
                    return Ok(Some(s));
                }
            }
        }
        Ok(None)
    }

    #[inline]
    fn eval_segment(seg: &SpkSegment, et: f64) -> Result<[f64; 6], SpkError> {
        match &seg.payload {
            SpkPayload::Type2(t) => t.evaluate(et),
            SpkPayload::Type3(t) => t.evaluate(et),
            SpkPayload::Type9(t) => t.evaluate(et),
            SpkPayload::Type13(t) => t.evaluate(et),
            SpkPayload::Unsupported => Err(SpkError::UnsupportedType(seg.data_type)),
        }
    }

    #[cold]
    #[inline(never)]
    fn state_wrt_ssb(&self, body: i32, et: f64) -> Result<[f64; 6], SpkError> {
        if body == 0 {
            return Ok([0.0; 6]);
        }
        let mut total = [0.0_f64; 6];
        let mut cur = body;
        for _ in 0..32 {
            let (delta, next_center) = self.step_toward_ssb(cur, et)?;
            for i in 0..6 {
                total[i] += delta[i];
            }
            if next_center == 0 {
                return Ok(total);
            }
            if next_center == body {
                return Err(SpkError::NoCoverage {
                    target: body,
                    center: 0,
                    et,
                });
            }
            cur = next_center;
        }
        Err(SpkError::NoCoverage {
            target: body,
            center: 0,
            et,
        })
    }

    #[cold]
    #[inline(never)]
    fn step_toward_ssb(&self, body: i32, et: f64) -> Result<([f64; 6], i32), SpkError> {
        let mut preferred: Option<(&SpkSegment, [f64; 6])> = None;
        for seg in &self.segments {
            if seg.target != body {
                continue;
            }
            if et < seg.start_et || et > seg.end_et {
                continue;
            }
            let s = Self::eval_segment(seg, et)?;
            if seg.center == 0 {
                return Ok((s, 0));
            }
            if preferred.is_none() {
                preferred = Some((seg, s));
            }
        }
        match preferred {
            Some((seg, s)) => Ok((s, seg.center)),
            None => Err(SpkError::NoCoverage {
                target: body,
                center: 0,
                et,
            }),
        }
    }
}
