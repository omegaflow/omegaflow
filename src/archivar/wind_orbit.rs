pub const MAGIC: [u8; 4] = *b"WOB1";

pub const RECORD_BYTES: usize = 56;

pub struct OrbitRec {
    pub times: Vec<f64>,
    pub pos: Vec<[f64; 3]>,
    pub vel: Vec<[f64; 3]>,
    pub median_stride: f64,
    pub hint: std::sync::Arc<std::sync::atomic::AtomicUsize>,
}

pub fn write_bin(records: &[(f64, [f64; 3], [f64; 3])]) -> Vec<u8> {
    let mut buf = Vec::with_capacity(8 + records.len() * RECORD_BYTES);
    buf.extend_from_slice(&MAGIC);
    buf.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for &(t, p, v) in records {
        buf.extend_from_slice(&t.to_le_bytes());
        buf.extend_from_slice(&p[0].to_le_bytes());
        buf.extend_from_slice(&p[1].to_le_bytes());
        buf.extend_from_slice(&p[2].to_le_bytes());
        buf.extend_from_slice(&v[0].to_le_bytes());
        buf.extend_from_slice(&v[1].to_le_bytes());
        buf.extend_from_slice(&v[2].to_le_bytes());
    }
    buf
}

fn f64_at(bytes: &[u8], off: usize) -> Option<f64> {
    Some(f64::from_le_bytes(
        bytes.get(off..off + 8)?.try_into().ok()?,
    ))
}

pub fn parse_bin(bytes: &[u8]) -> Option<Vec<(f64, [f64; 3], [f64; 3])>> {
    if bytes.len() < 8 || bytes[0..4] != MAGIC {
        return None;
    }
    let n = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    if n > (bytes.len() - 8) / RECORD_BYTES {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    let mut off = 8usize;
    for _ in 0..n {
        let t = f64_at(bytes, off)?;
        let x = f64_at(bytes, off + 8)?;
        let y = f64_at(bytes, off + 16)?;
        let z = f64_at(bytes, off + 24)?;
        let vx = f64_at(bytes, off + 32)?;
        let vy = f64_at(bytes, off + 40)?;
        let vz = f64_at(bytes, off + 48)?;
        off += RECORD_BYTES;
        out.push((t, [x, y, z], [vx, vy, vz]));
    }
    Some(out)
}

pub fn orbit_rec(records: &[(f64, [f64; 3], [f64; 3])]) -> OrbitRec {
    let mut times = Vec::with_capacity(records.len());
    let mut pos = Vec::with_capacity(records.len());
    let mut vel = Vec::with_capacity(records.len());
    for &(t, p, v) in records {
        times.push(t);
        pos.push(p);
        vel.push(v);
    }
    let mut strides: Vec<f64> = times.windows(2).map(|w| w[1] - w[0]).collect();
    strides.sort_by(|a, b| a.total_cmp(b));
    let median_stride = if strides.is_empty() {
        600.0
    } else {
        strides[strides.len() / 2]
    };
    OrbitRec {
        times,
        pos,
        vel,
        median_stride,
        hint: std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0)),
    }
}

pub fn position_at(rec: &OrbitRec, t: f64) -> Option<([f64; 3], [f64; 3])> {
    let n = rec.times.len();
    if n == 0 {
        return None;
    }
    let mut idx = rec
        .hint
        .load(std::sync::atomic::Ordering::Relaxed)
        .min(n - 1);
    while idx < n && rec.times[idx] < t {
        idx += 1;
    }
    while idx > 0 && rec.times[idx - 1] >= t {
        idx -= 1;
    }
    rec.hint
        .store(idx.min(n - 1), std::sync::atomic::Ordering::Relaxed);
    if idx == 0 {
        if (rec.times[0] - t).abs() < 1.0e-6 {
            return Some((rec.pos[0], rec.vel[0]));
        }
        return None;
    }
    if idx >= n {
        if (rec.times[n - 1] - t).abs() < 1.0e-6 {
            return Some((rec.pos[n - 1], rec.vel[n - 1]));
        }
        return None;
    }
    let t0 = rec.times[idx - 1];
    let t1 = rec.times[idx];
    let stride = t1 - t0;
    if stride <= 0.0 || stride > 2.5 * rec.median_stride {
        return None;
    }
    let w = (t - t0) / stride;
    let p0 = rec.pos[idx - 1];
    let p1 = rec.pos[idx];
    let v0 = rec.vel[idx - 1];
    let v1 = rec.vel[idx];
    let mix = |a: f64, b: f64| a + (b - a) * w;
    Some((
        [mix(p0[0], p1[0]), mix(p0[1], p1[1]), mix(p0[2], p1[2])],
        [mix(v0[0], v1[0]), mix(v0[1], v1[1]), mix(v0[2], v1[2])],
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let records = vec![
            (100.0, [1.0, 2.0, 3.0], [0.1, 0.2, 0.3]),
            (700.0, [4.0, 5.0, 6.0], [0.4, 0.5, 0.6]),
            (1300.0, [7.0, 8.0, 9.0], [0.7, 0.8, 0.9]),
        ];
        let bytes = write_bin(&records);
        let parsed = parse_bin(&bytes).unwrap();
        assert_eq!(parsed.len(), 3);
        assert_eq!(parsed[1].0, 700.0);
        assert_eq!(parsed[1].1, [4.0, 5.0, 6.0]);
    }

    #[test]
    fn rejects_foreign_bytes() {
        assert!(parse_bin(b"X").is_none());
        assert!(parse_bin(b"WOB1abc").is_none());
    }

    #[test]
    fn interpolates_between_samples() {
        let records = vec![
            (0.0, [0.0, 0.0, 0.0], [1.0, 1.0, 1.0]),
            (600.0, [600.0, 0.0, 0.0], [2.0, 2.0, 2.0]),
            (1200.0, [1200.0, 0.0, 0.0], [3.0, 3.0, 3.0]),
        ];
        let rec = orbit_rec(&records);
        let (p, v) = position_at(&rec, 300.0).unwrap();
        assert_eq!(p, [300.0, 0.0, 0.0]);
        assert_eq!(v, [1.5, 1.5, 1.5]);
        assert!(position_at(&rec, 900.0).is_some());
        assert!(position_at(&rec, 5000.0).is_none());
        assert!(position_at(&rec, -10.0).is_none());
    }

    #[test]
    fn refuses_across_gaps() {
        let records = vec![
            (0.0, [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]),
            (600.0, [1.0, 0.0, 0.0], [0.0, 0.0, 0.0]),
            (100000.0, [2.0, 0.0, 0.0], [0.0, 0.0, 0.0]),
            (100600.0, [3.0, 0.0, 0.0], [0.0, 0.0, 0.0]),
        ];
        let rec = orbit_rec(&records);
        assert!(position_at(&rec, 300.0).is_some());
        assert!(position_at(&rec, 100300.0).is_some());
        assert!(position_at(&rec, 50300.0).is_none());
    }

    #[test]
    fn honors_exact_sample_times() {
        let records = vec![
            (0.0, [0.0, 0.0, 0.0], [1.0, 0.0, 0.0]),
            (600.0, [1.0, 0.0, 0.0], [2.0, 0.0, 0.0]),
        ];
        let rec = orbit_rec(&records);
        let (p, v) = position_at(&rec, 600.0).unwrap();
        assert_eq!(p, [1.0, 0.0, 0.0]);
        assert_eq!(v, [2.0, 0.0, 0.0]);
    }
}
