use std::path::Path;

use crate::bsp_reader::daf::{DafError, DafFile};
use crate::fk::FkFile;
use crate::lsk::days_from_civil;
use crate::mat::matmul;

#[derive(Debug, Clone, Copy)]
pub struct SclkCoeff {
    pub clock: f64,
    pub time: f64,
    pub rate: f64,
}

struct SclkBlock {
    msf: f64,
    coeffs: Vec<SclkCoeff>,
}

pub struct SclkFile {
    blocks: Vec<(i32, SclkBlock)>,
}

fn month_of(name: &str) -> Option<i64> {
    match name {
        "JAN" => Some(1),
        "FEB" => Some(2),
        "MAR" => Some(3),
        "APR" => Some(4),
        "MAY" => Some(5),
        "JUN" => Some(6),
        "JUL" => Some(7),
        "AUG" => Some(8),
        "SEP" => Some(9),
        "OCT" => Some(10),
        "NOV" => Some(11),
        "DEC" => Some(12),
        _ => None,
    }
}

fn parse_time(token: &str) -> Option<f64> {
    if let Some(date) = token.strip_prefix('@') {
        let parts: Vec<&str> = date.split('-').collect();
        let year: i64 = parts.first()?.parse().ok()?;
        let month = month_of(parts.get(1)?)?;
        let day: i64 = parts.get(2)?.parse().ok()?;
        let days = days_from_civil(year, month, day)?;
        let mut secs_of_day = 0.0;
        if let Some(hms) = parts.get(3) {
            let mut it = hms.split(':');
            let h: f64 = it.next()?.parse().ok()?;
            let m: f64 = it.next()?.parse().ok()?;
            let s: f64 = it.next().unwrap_or("0").parse().ok()?;
            secs_of_day = h * 3600.0 + m * 60.0 + s;
        }
        return Some((days as f64 - 10957.5) * 86400.0 + secs_of_day);
    }
    token.parse::<f64>().ok()
}

fn msf_of(moduli: &[f64]) -> f64 {
    moduli.iter().skip(1).fold(1.0, |acc, m| acc * m)
}

impl SclkFile {
    pub fn parse(text: &str) -> SclkFile {
        let mut coeff_blocks: Vec<(i32, Vec<SclkCoeff>)> = Vec::new();
        let mut moduli: Vec<(i32, Vec<f64>)> = Vec::new();
        let mut lines = text.lines().peekable();
        while let Some(line) = lines.next() {
            let trimmed = line.trim();
            if let Some(idx) = trimmed.find("SCLK01_MODULI_") {
                let after = &trimmed[idx + "SCLK01_MODULI_".len()..];
                let digits: String = after.chars().take_while(|c| c.is_ascii_digit()).collect();
                let Ok(id) = digits.parse::<i32>() else {
                    continue;
                };
                let Some(lparen) = trimmed.rfind('(') else {
                    continue;
                };
                let start = lparen + 1;
                let mut buf = String::from(&trimmed[start..]);
                loop {
                    if buf.contains(')') {
                        break;
                    }
                    match lines.next() {
                        Some(next) => {
                            buf.push(' ');
                            buf.push_str(next);
                        }
                        None => break,
                    }
                }
                let end = buf.find(')').unwrap_or(buf.len());
                let mut m: Vec<f64> = Vec::new();
                for tok in buf[..end].split_whitespace() {
                    if let Ok(v) = tok.parse::<f64>() {
                        m.push(v);
                    }
                }
                if !m.is_empty() {
                    moduli.push((id, m));
                }
                continue;
            }
            let Some(idx) = trimmed.find("SCLK01_COEFFICIENTS_") else {
                continue;
            };
            let after = &trimmed[idx + "SCLK01_COEFFICIENTS_".len()..];
            let digits: String = after.chars().take_while(|c| c.is_ascii_digit()).collect();
            let Ok(id) = digits.parse::<i32>() else {
                continue;
            };
            let Some(lparen) = trimmed.rfind('(') else {
                continue;
            };
            let start = lparen + 1;
            let mut buf = String::from(&trimmed[start..]);
            loop {
                if buf.contains(')') {
                    break;
                }
                match lines.next() {
                    Some(next) => {
                        buf.push(' ');
                        buf.push_str(next);
                    }
                    None => break,
                }
            }
            let end = buf.find(')').unwrap_or(buf.len());
            let mut coeffs: Vec<SclkCoeff> = Vec::new();
            for chunk in buf[..end]
                .split_whitespace()
                .collect::<Vec<&str>>()
                .chunks_exact(3)
            {
                let clock = chunk[0].parse::<f64>();
                let time = parse_time(chunk[1]);
                let rate = chunk[2].parse::<f64>();
                if let (Ok(c), Some(t), Ok(r)) = (clock, time, rate) {
                    coeffs.push(SclkCoeff {
                        clock: c,
                        time: t,
                        rate: r,
                    });
                }
            }
            if !coeffs.is_empty() {
                coeffs.sort_by(|a, b| a.clock.partial_cmp(&b.clock).unwrap());
                coeff_blocks.push((id, coeffs));
            }
        }
        let blocks: Vec<(i32, SclkBlock)> = coeff_blocks
            .into_iter()
            .map(|(id, coeffs)| {
                let msf = moduli
                    .iter()
                    .find(|(mid, _)| (*mid as u32) == id.unsigned_abs())
                    .map(|(_, m)| msf_of(m))
                    .unwrap_or(1.0);
                (id, SclkBlock { msf, coeffs })
            })
            .collect();
        SclkFile { blocks }
    }

    pub fn open<P: AsRef<Path>>(path: P) -> Result<SclkFile, String> {
        std::fs::read_to_string(path.as_ref())
            .map(|t| SclkFile::parse(&t))
            .map_err(|e| e.to_string())
    }

    fn block(&self, scid: i32) -> Option<&SclkBlock> {
        let key = scid.unsigned_abs();
        self.blocks
            .iter()
            .find(|(id, _)| (*id as u32) == key)
            .map(|(_, b)| b)
    }

    pub fn tick_to_et(&self, scid: i32, tick: f64) -> Option<f64> {
        let block = self.block(scid)?;
        let coeffs = &block.coeffs;
        let idx = upper_bound(coeffs.len(), |i| coeffs[i].clock <= tick);
        if idx == 0 {
            return None;
        }
        let c = &coeffs[idx - 1];
        Some(c.time + (tick - c.clock) * c.rate / block.msf)
    }

    pub fn et_to_tick(&self, scid: i32, et: f64) -> Option<f64> {
        let block = self.block(scid)?;
        let coeffs = &block.coeffs;
        let idx = upper_bound(coeffs.len(), |i| coeffs[i].time <= et);
        if idx == 0 {
            return None;
        }
        let c = &coeffs[idx - 1];
        Some(c.clock + (et - c.time) * block.msf / c.rate)
    }
}

#[derive(Debug)]
pub enum CkError {
    Daf(DafError),
    NoCoverage {
        instrument: i32,
        reference: i32,
        tick: f64,
    },
    UnsupportedType(i32),
    BadSegment(&'static str),
}

impl std::fmt::Display for CkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CkError::Daf(e) => write!(f, "{e}"),
            CkError::NoCoverage {
                instrument,
                reference,
                tick,
            } => write!(
                f,
                "no CK segment covers instrument {instrument} wrt {reference} at tick {tick}"
            ),
            CkError::UnsupportedType(t) => write!(f, "unsupported CK data type {t}"),
            CkError::BadSegment(msg) => write!(f, "malformed CK segment: {msg}"),
        }
    }
}

impl From<DafError> for CkError {
    fn from(e: DafError) -> Self {
        CkError::Daf(e)
    }
}

impl std::error::Error for CkError {}

#[derive(Debug, Clone)]
pub struct CkState {
    pub matrix: [f64; 9],
    pub angular_velocity: Option<[f64; 3]>,
}

pub struct CkSegment {
    pub instrument: i32,
    pub reference: i32,
    pub data_type: i32,
    pub av_flag: i32,
    pub start_tick: f64,
    pub end_tick: f64,
    pub start_addr: u32,
    pub end_addr: u32,
    pub name: String,
    payload: CkPayload,
}

enum CkPayload {
    Discrete(Discrete),
    Unsupported,
}

struct Discrete {
    file: DafFile,
    dtype: i32,
    reclen: usize,
    n_records: usize,
    av: bool,
    start_addr: u32,
    times_addr: u32,
    stop_times_addr: u32,
    numint: usize,
}

impl Discrete {
    fn from_segment(
        file: &DafFile,
        dtype: i32,
        av_flag: i32,
        start_addr: u32,
        end_addr: u32,
    ) -> Result<Discrete, CkError> {
        let dbls = file.read_doubles(start_addr, end_addr)?;
        let total = dbls.len();
        match dtype {
            1 => {
                if total < 2 {
                    return Err(CkError::BadSegment("type 1 too short"));
                }
                let nprec = dbls[total - 1].round() as usize;
                let reclen = if av_flag == 1 { 7 } else { 4 };
                let dir = (nprec.saturating_sub(1)) / 100;
                if nprec == 0 || nprec * reclen + nprec + dir + 1 != total {
                    return Err(CkError::BadSegment("type 1 geometry unresolved"));
                }
                Ok(Discrete {
                    file: file.clone(),
                    dtype,
                    reclen,
                    n_records: nprec,
                    av: av_flag == 1,
                    start_addr,
                    times_addr: start_addr + (nprec * reclen) as u32,
                    stop_times_addr: 0,
                    numint: 0,
                })
            }
            2 => {
                let reclen = 8usize;
                let nprec = solve_type2_records(total)?;
                let dir = (nprec.saturating_sub(1)) / 100;
                if nprec == 0 || nprec * reclen + nprec + nprec + dir != total {
                    return Err(CkError::BadSegment("type 2 geometry unresolved"));
                }
                Ok(Discrete {
                    file: file.clone(),
                    dtype,
                    reclen,
                    n_records: nprec,
                    av: true,
                    start_addr,
                    times_addr: start_addr + (nprec * reclen) as u32,
                    stop_times_addr: start_addr + (nprec * reclen + nprec) as u32,
                    numint: 0,
                })
            }
            3 => {
                if total < 3 {
                    return Err(CkError::BadSegment("type 3 too short"));
                }
                let nprec = dbls[total - 1].round() as usize;
                let numint = dbls[total - 2].round() as usize;
                let reclen = if av_flag == 1 { 7 } else { 4 };
                let dir = (nprec.saturating_sub(1)) / 100;
                let sdir = (numint.saturating_sub(1)) / 100;
                if nprec == 0
                    || numint == 0
                    || nprec * reclen + nprec + dir + numint + sdir + 2 != total
                {
                    return Err(CkError::BadSegment("type 3 geometry unresolved"));
                }
                Ok(Discrete {
                    file: file.clone(),
                    dtype,
                    reclen,
                    n_records: nprec,
                    av: av_flag == 1,
                    start_addr,
                    times_addr: start_addr + (nprec * reclen) as u32,
                    stop_times_addr: 0,
                    numint,
                })
            }
            _ => Err(CkError::UnsupportedType(dtype)),
        }
    }

    fn record(&self, idx: usize) -> Result<[f64; 8], CkError> {
        let base = self.start_addr + (idx * self.reclen) as u32;
        let d = self
            .file
            .doubles_native(base, base + self.reclen as u32 - 1)?;
        let mut out = [0.0_f64; 8];
        out[..self.reclen].copy_from_slice(&d);
        Ok(out)
    }

    fn times(&self) -> Result<Vec<f64>, CkError> {
        Ok(self
            .file
            .doubles_native(self.times_addr, self.times_addr + self.n_records as u32 - 1)?)
    }

    fn state_from_record(&self, rec: &[f64; 8]) -> CkState {
        let q = normalize_quat([rec[0], rec[1], rec[2], rec[3]]);
        let matrix = quat_to_matrix(q);
        let angular_velocity = if self.av {
            Some([rec[4], rec[5], rec[6]])
        } else {
            None
        };
        CkState {
            matrix,
            angular_velocity,
        }
    }

    fn state_at(&self, idx: usize) -> Result<CkState, CkError> {
        let rec = self.record(idx)?;
        Ok(self.state_from_record(&rec))
    }

    fn evaluate(&self, tick: f64) -> Result<CkState, CkError> {
        match self.dtype {
            1 => self.eval_type1(tick),
            2 => self.eval_type2(tick),
            3 => self.eval_type3(tick),
            _ => Err(CkError::UnsupportedType(self.dtype)),
        }
    }

    fn eval_type1(&self, tick: f64) -> Result<CkState, CkError> {
        let times = self.times()?;
        let idx = upper_bound(self.n_records, |i| times[i] <= tick);
        let nearest = if idx == 0 {
            0
        } else if idx == self.n_records {
            self.n_records - 1
        } else if (tick - times[idx - 1]) <= (times[idx] - tick) {
            idx - 1
        } else {
            idx
        };
        self.state_at(nearest)
    }

    fn eval_type2(&self, tick: f64) -> Result<CkState, CkError> {
        let starts = self.times()?;
        let stops = self.file.doubles_native(
            self.stop_times_addr,
            self.stop_times_addr + self.n_records as u32 - 1,
        )?;
        let idx = upper_bound(self.n_records, |i| starts[i] <= tick);
        if idx == 0 {
            return self.state_at(0);
        }
        let k = idx - 1;
        let rec = self.record(k)?;
        let q = normalize_quat([rec[0], rec[1], rec[2], rec[3]]);
        let m0 = quat_to_matrix(q);
        let av = [rec[4], rec[5], rec[6]];
        let rate = rec[7];
        let dt = (tick - starts[k]) * rate;
        let mag = (av[0] * av[0] + av[1] * av[1] + av[2] * av[2]).sqrt();
        let matrix = if mag == 0.0 || dt == 0.0 {
            m0
        } else {
            let axis = [av[0] / mag, av[1] / mag, av[2] / mag];
            let rot = axis_angle_matrix(axis, mag * dt);
            matmul(&rot, &m0)
        };
        let _ = &stops;
        Ok(CkState {
            matrix,
            angular_velocity: Some(av),
        })
    }

    fn interval_starts(&self) -> Result<Vec<f64>, CkError> {
        let dir = (self.n_records.saturating_sub(1)) / 100;
        let base = self.times_addr + self.n_records as u32 + dir as u32;
        Ok(self
            .file
            .doubles_native(base, base + self.numint as u32 - 1)?)
    }

    fn eval_type3(&self, tick: f64) -> Result<CkState, CkError> {
        let times = self.times()?;
        let idx = upper_bound(self.n_records, |i| times[i] <= tick);
        if idx == 0 {
            return self.state_at(0);
        }
        if idx == self.n_records {
            return self.state_at(self.n_records - 1);
        }
        let lo = idx - 1;
        let hi = idx;
        let t_lo = times[lo];
        let t_hi = times[hi];
        let starts = self.interval_starts()?;
        let hi_is_start = upper_bound(starts.len(), |i| starts[i] <= t_hi) > 0
            && starts[upper_bound(starts.len(), |i| starts[i] <= t_hi) - 1] == t_hi;
        if hi_is_start {
            return if (tick - t_lo) <= (t_hi - tick) {
                self.state_at(lo)
            } else {
                self.state_at(hi)
            };
        }
        let w = (tick - t_lo) / (t_hi - t_lo);
        let rec_lo = self.record(lo)?;
        let rec_hi = self.record(hi)?;
        let q_lo = [rec_lo[0], rec_lo[1], rec_lo[2], rec_lo[3]];
        let q_hi = [rec_hi[0], rec_hi[1], rec_hi[2], rec_hi[3]];
        let q = slerp(q_lo, q_hi, w);
        let matrix = quat_to_matrix(q);
        let angular_velocity = if self.av {
            Some([
                rec_lo[4] + (rec_hi[4] - rec_lo[4]) * w,
                rec_lo[5] + (rec_hi[5] - rec_lo[5]) * w,
                rec_lo[6] + (rec_hi[6] - rec_lo[6]) * w,
            ])
        } else {
            None
        };
        Ok(CkState {
            matrix,
            angular_velocity,
        })
    }
}

fn solve_type2_records(total: usize) -> Result<usize, CkError> {
    let mut n = total / 10;
    while n > 0 {
        let dir = (n - 1) / 100;
        let computed = n * 10 + dir;
        if computed == total {
            return Ok(n);
        }
        if computed < total {
            return Err(CkError::BadSegment("type 2 record count unresolved"));
        }
        n -= 1;
    }
    Err(CkError::BadSegment("type 2 record count unresolved"))
}

pub struct CkFile {
    segments: Vec<CkSegment>,
}

impl CkFile {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, CkError> {
        let daf = DafFile::open(path)?;
        Self::from_daf(daf)
    }

    pub fn from_daf(daf: DafFile) -> Result<Self, CkError> {
        let mut segments = Vec::new();
        for summary in daf.summaries()? {
            if summary.doubles.len() < 2 || summary.integers.len() < 6 {
                continue;
            }
            let start_tick = summary.doubles[0];
            let end_tick = summary.doubles[1];
            let instrument = summary.integers[0];
            let reference = summary.integers[1];
            let data_type = summary.integers[2];
            let av_flag = summary.integers[3];
            let start_addr = summary.integers[4] as u32;
            let end_addr = summary.integers[5] as u32;

            let payload = match data_type {
                1 | 2 | 3 => {
                    match Discrete::from_segment(&daf, data_type, av_flag, start_addr, end_addr) {
                        Ok(d) => CkPayload::Discrete(d),
                        Err(_) => CkPayload::Unsupported,
                    }
                }
                _ => CkPayload::Unsupported,
            };

            segments.push(CkSegment {
                instrument,
                reference,
                data_type,
                av_flag,
                start_tick,
                end_tick,
                start_addr,
                end_addr,
                name: summary.name,
                payload,
            });
        }
        Ok(CkFile { segments })
    }

    pub fn segments(&self) -> &[CkSegment] {
        &self.segments
    }

    pub fn attitude(&self, instrument: i32, reference: i32, tick: f64) -> Result<CkState, CkError> {
        for seg in &self.segments {
            if seg.instrument != instrument || seg.reference != reference {
                continue;
            }
            if tick < seg.start_tick || tick > seg.end_tick {
                continue;
            }
            return match &seg.payload {
                CkPayload::Discrete(d) => d.evaluate(tick),
                CkPayload::Unsupported => Err(CkError::UnsupportedType(seg.data_type)),
            };
        }
        Err(CkError::NoCoverage {
            instrument,
            reference,
            tick,
        })
    }
}

pub struct CkFrameRef {
    pub id: i32,
    pub name: String,
    pub sclk_id: i32,
    pub spk_id: i32,
}

pub fn switch_resolution(fk: &FkFile, switch_frame: i32) -> Option<Vec<CkFrameRef>> {
    let f = fk.frame(switch_frame)?;
    if f.class != Some(6) {
        return None;
    }
    let aligned = f.aligned_with.as_ref()?;
    let mut out = Vec::new();
    for name in aligned {
        let child = fk.frame_by_name(name)?;
        let sclk_id = child.sclk?;
        let spk_id = child.spk?;
        out.push(CkFrameRef {
            id: child.id,
            name: child.name.clone(),
            sclk_id,
            spk_id,
        });
    }
    if out.is_empty() {
        None
    } else {
        Some(out)
    }
}

fn upper_bound(n: usize, pred: impl Fn(usize) -> bool) -> usize {
    let mut lo = 0usize;
    let mut hi = n;
    while lo < hi {
        let mid = (lo + hi) / 2;
        if pred(mid) {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    lo
}

fn normalize_quat(q: [f64; 4]) -> [f64; 4] {
    let n = (q[0] * q[0] + q[1] * q[1] + q[2] * q[2] + q[3] * q[3]).sqrt();
    if n == 0.0 {
        return [1.0, 0.0, 0.0, 0.0];
    }
    [q[0] / n, q[1] / n, q[2] / n, q[3] / n]
}

fn quat_to_matrix(q: [f64; 4]) -> [f64; 9] {
    let [q0, q1, q2, q3] = q;
    let n2 = q0 * q0 + q1 * q1 + q2 * q2 + q3 * q3;
    if n2 == 0.0 {
        return [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0];
    }
    let s = 2.0 / n2;
    [
        1.0 - s * (q2 * q2 + q3 * q3),
        s * (q1 * q2 - q0 * q3),
        s * (q1 * q3 + q0 * q2),
        s * (q1 * q2 + q0 * q3),
        1.0 - s * (q1 * q1 + q3 * q3),
        s * (q2 * q3 - q0 * q1),
        s * (q1 * q3 - q0 * q2),
        s * (q2 * q3 + q0 * q1),
        1.0 - s * (q1 * q1 + q2 * q2),
    ]
}

fn slerp(qa: [f64; 4], qb: [f64; 4], t: f64) -> [f64; 4] {
    let mut dot = qa[0] * qb[0] + qa[1] * qb[1] + qa[2] * qb[2] + qa[3] * qb[3];
    let mut qb = qb;
    if dot < 0.0 {
        dot = -dot;
        qb = [-qb[0], -qb[1], -qb[2], -qb[3]];
    }
    if dot > 0.9995 {
        let mut q = [0.0_f64; 4];
        for i in 0..4 {
            q[i] = qa[i] + t * (qb[i] - qa[i]);
        }
        return normalize_quat(q);
    }
    let theta = dot.acos();
    let sin_theta = theta.sin();
    let s0 = ((1.0 - t) * theta).sin() / sin_theta;
    let s1 = (t * theta).sin() / sin_theta;
    let mut q = [0.0_f64; 4];
    for i in 0..4 {
        q[i] = qa[i] * s0 + qb[i] * s1;
    }
    normalize_quat(q)
}

fn axis_angle_matrix(axis: [f64; 3], angle: f64) -> [f64; 9] {
    let [x, y, z] = axis;
    let (s, c) = angle.sin_cos();
    let t = 1.0 - c;
    [
        c + x * x * t,
        x * y * t - z * s,
        x * z * t + y * s,
        y * x * t + z * s,
        c + y * y * t,
        y * z * t - x * s,
        z * x * t - y * s,
        z * y * t + x * s,
        c + z * z * t,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bsp_reader::daf::{DOUBLE_BYTES, RECORD_BYTES};

    const DATA_START_ADDR: u32 = 3 * (RECORD_BYTES as u32) / (DOUBLE_BYTES as u32) + 1;

    fn synthetic_ck_daf(dtype: i32, av_flag: i32) -> Vec<u8> {
        let nprec = 2usize;
        let numint = 1usize;
        let reclen = if dtype == 2 {
            8
        } else if av_flag == 1 {
            7
        } else {
            4
        };
        let start_addr = DATA_START_ADDR;
        let total = match dtype {
            1 => nprec * reclen + nprec + 0 + 1,
            2 => nprec * reclen + nprec + nprec + 0,
            3 => nprec * reclen + nprec + 0 + numint + 0 + 2,
            _ => return Vec::new(),
        };
        let end_addr = start_addr + total as u32 - 1;
        let mut buf = vec![0u8; end_addr as usize * DOUBLE_BYTES];

        buf[0..8].copy_from_slice(b"DAF/CK  ");
        let nd: u32 = 2;
        let ni: u32 = 6;
        buf[8..12].copy_from_slice(&nd.to_le_bytes());
        buf[12..16].copy_from_slice(&ni.to_le_bytes());
        let fward: u32 = 2;
        buf[76..80].copy_from_slice(&fward.to_le_bytes());
        buf[88..96].copy_from_slice(b"LTL-IEEE");

        let sum_rec = RECORD_BYTES;
        let nsum: f64 = 1.0;
        buf[sum_rec + 16..sum_rec + 24].copy_from_slice(&nsum.to_le_bytes());
        let t0: f64 = 100.0;
        let t1: f64 = 200.0;
        buf[sum_rec + 24..sum_rec + 32].copy_from_slice(&t0.to_le_bytes());
        buf[sum_rec + 32..sum_rec + 40].copy_from_slice(&t1.to_le_bytes());
        let ints: [i32; 6] = [
            -28002,
            1,
            dtype,
            av_flag,
            start_addr as i32,
            end_addr as i32,
        ];
        for (k, v) in ints.iter().enumerate() {
            let off = sum_rec + 24 + 2 * DOUBLE_BYTES + k * 4;
            buf[off..off + 4].copy_from_slice(&v.to_le_bytes());
        }

        let name_rec = 2 * RECORD_BYTES;
        buf[name_rec..name_rec + 4].copy_from_slice(b"TEST");

        let base = (start_addr as usize - 1) * DOUBLE_BYTES;
        let s = std::f64::consts::FRAC_1_SQRT_2;
        let q0 = [1.0, 0.0, 0.0, 0.0];
        let q1 = [s, 0.0, 0.0, s];
        let av = [0.1_f64, 0.2_f64, 0.3_f64];
        let mut off = base;
        for q in [q0, q1] {
            for i in 0..4 {
                buf[off..off + 8].copy_from_slice(&q[i].to_le_bytes());
                off += 8;
            }
            if av_flag == 1 {
                for i in 0..3 {
                    buf[off..off + 8].copy_from_slice(&av[i].to_le_bytes());
                    off += 8;
                }
            }
            if dtype == 2 {
                buf[off..off + 8].copy_from_slice(&1.0_f64.to_le_bytes());
                off += 8;
            }
        }
        for t in [t0, t1] {
            buf[off..off + 8].copy_from_slice(&t.to_le_bytes());
            off += 8;
        }
        if dtype == 2 {
            for t in [t1, t1 + 100.0] {
                buf[off..off + 8].copy_from_slice(&t.to_le_bytes());
                off += 8;
            }
        }
        if dtype == 3 {
            buf[off..off + 8].copy_from_slice(&t0.to_le_bytes());
            off += 8;
            buf[off..off + 8].copy_from_slice(&(numint as f64).to_le_bytes());
            off += 8;
            buf[off..off + 8].copy_from_slice(&(nprec as f64).to_le_bytes());
            off += 8;
        }
        if dtype == 1 {
            buf[off..off + 8].copy_from_slice(&(nprec as f64).to_le_bytes());
        }
        buf
    }

    fn ck_from_synthetic(dtype: i32, av_flag: i32) -> CkFile {
        let data = synthetic_ck_daf(dtype, av_flag);
        let daf = DafFile::from_data(data).expect("synthetic DAF parses");
        CkFile::from_daf(daf).expect("synthetic CK parses")
    }

    #[test]
    fn type3_slerp_halfway_is_45deg_about_z() {
        let ck = ck_from_synthetic(3, 1);
        let seg = &ck.segments()[0];
        assert_eq!(seg.data_type, 3);
        assert_eq!(seg.instrument, -28002);
        assert_eq!(seg.reference, 1);
        let state = ck.attitude(-28002, 1, 150.0).expect("attitude");
        let c = std::f64::consts::FRAC_1_SQRT_2;
        let expect = [c, -c, 0.0, c, c, 0.0, 0.0, 0.0, 1.0];
        for i in 0..9 {
            assert!(
                (state.matrix[i] - expect[i]).abs() < 1e-9,
                "matrix[{i}] was {}",
                state.matrix[i]
            );
        }
        assert!(state.angular_velocity.is_some());
    }

    #[test]
    fn type1_velocity_is_absent() {
        let ck = ck_from_synthetic(1, 0);
        let seg = &ck.segments()[0];
        assert_eq!(seg.data_type, 1);
        assert_eq!(seg.av_flag, 0);
        let state = ck.attitude(-28002, 1, 100.0).expect("attitude");
        assert_eq!(state.matrix, [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]);
        assert!(state.angular_velocity.is_none());
    }

    #[test]
    fn type2_constant_rotation() {
        let ck = ck_from_synthetic(2, 1);
        let seg = &ck.segments()[0];
        assert_eq!(seg.data_type, 2);
        let state = ck.attitude(-28002, 1, 100.0).expect("attitude");
        assert_eq!(state.matrix, [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]);
        assert!(state.angular_velocity.is_some());
    }

    #[test]
    fn attitude_without_coverage_is_no_coverage() {
        let ck = ck_from_synthetic(3, 1);
        assert!(matches!(
            ck.attitude(-28002, 1, 999.0),
            Err(CkError::NoCoverage { .. })
        ));
    }

    const TSC: &str = "\\begindata\n\
SCLK_DATA_TYPE_28 = ( 1 )\n\
SCLK01_COEFFICIENTS_28 = (\n\
  0.0        1000.0   1.0\n\
  1000.0     2000.0   2.0\n\
)\n";

    #[test]
    fn sclk_tick_to_et_and_back() {
        let sclk = SclkFile::parse(TSC);
        assert_eq!(sclk.tick_to_et(28, 500.0), Some(1500.0));
        assert_eq!(sclk.tick_to_et(28, 1500.0), Some(3000.0));
        assert_eq!(sclk.tick_to_et(-28, 500.0), Some(1500.0));
        assert_eq!(sclk.et_to_tick(28, 1500.0), Some(500.0));
        assert_eq!(sclk.et_to_tick(28, 3000.0), Some(1500.0));
        assert_eq!(sclk.tick_to_et(28, -1.0), None);
    }

    const JUICE_FICT_TSC: &str = "\\begindata\n\
SCLK_DATA_TYPE_28999 = ( 1 )\n\
SCLK01_MODULI_28999 = ( 4294967296 65536 )\n\
SCLK01_COEFFICIENTS_28999 = (\n\
  0.0    @2022-MAY-31-23:59:59.999    1\n\
)\n";

    #[test]
    fn sclk_juice_moduli_scale_the_tick_rate() {
        let sclk = SclkFile::parse(JUICE_FICT_TSC);
        let expected_t0 = (crate::lsk::days_from_civil(2022, 5, 31).unwrap() as f64 - 10957.5)
            * 86400.0
            + 86399.999;
        let t0 = sclk.tick_to_et(-28999, 0.0).unwrap();
        assert!((t0 - expected_t0).abs() < 1e-6);
        let et1 = sclk.tick_to_et(-28999, 65536.0).unwrap();
        assert!((et1 - (t0 + 1.0)).abs() < 1e-6);
        let tick2 = sclk.et_to_tick(-28999, t0 + 2.0).unwrap();
        assert!((tick2 - 131072.0).abs() < 1e-6);
    }

    const FK_TF: &str = "\\begindata\n\
FRAME_JUICE_SPACECRAFT = -28000\n\
FRAME_-28000_NAME = 'JUICE_SPACECRAFT'\n\
FRAME_-28000_CLASS = 6\n\
FRAME_-28000_ALIGNED_WITH = (\n\
   'JUICE_SPACECRAFT_PLAN'\n\
   'JUICE_SPACECRAFT_MEAS'\n\
)\n\
\\begindata\n\
FRAME_JUICE_SPACECRAFT_PLAN = -28001\n\
FRAME_-28001_CLASS = 3\n\
CK_-28001_SCLK = -28999\n\
CK_-28001_SPK = -28\n\
\\begindata\n\
FRAME_JUICE_SPACECRAFT_MEAS = -28002\n\
FRAME_-28002_CLASS = 3\n\
CK_-28002_SCLK = -28\n\
CK_-28002_SPK = -28\n";

    #[test]
    fn switch_frame_resolves_to_aligned_ck_frames() {
        let fk = FkFile::parse(FK_TF);
        let resolved = switch_resolution(&fk, -28000).expect("switch resolves");
        assert_eq!(resolved.len(), 2);
        assert_eq!(resolved[0].id, -28001);
        assert_eq!(resolved[0].sclk_id, -28999);
        assert_eq!(resolved[1].id, -28002);
        assert_eq!(resolved[1].sclk_id, -28);
        assert_eq!(resolved[1].spk_id, -28);
    }

    #[test]
    fn switch_resolution_of_non_switch_frame_is_none() {
        let fk = FkFile::parse(FK_TF);
        assert!(switch_resolution(&fk, -28001).is_none());
    }
}
