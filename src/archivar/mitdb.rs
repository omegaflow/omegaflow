pub const MAGIC: [u8; 4] = *b"MITB";

pub const COMP_MLII: u32 = 1;
pub const COMP_V1: u32 = 2;
pub const COMP_V2: u32 = 3;
pub const COMP_V4: u32 = 4;
pub const COMP_V5: u32 = 5;

pub struct Lead {
    pub format: u16,
    pub gain: f64,
    pub adc_zero: i64,
    pub name: String,
}

pub struct HeaInfo {
    pub nchan: usize,
    pub sample_rate: u32,
    pub nsamp: usize,
    pub leads: Vec<Lead>,
    pub comment: Option<String>,
}

pub fn parse_hea(text: &str) -> Option<HeaInfo> {
    let mut lines = text.lines().map(|l| l.trim_end_matches('\r'));
    let tokens: Vec<&str> = lines.next()?.split_whitespace().collect();
    if tokens.len() < 4 {
        return None;
    }
    let nchan: usize = tokens[1].parse().ok()?;
    let sample_rate: u32 = tokens[2].parse().ok()?;
    let nsamp: usize = tokens[3].parse().ok()?;
    if nchan == 0 || nsamp == 0 || sample_rate == 0 {
        return None;
    }
    let mut leads = Vec::new();
    let mut comment = None;
    for line in lines {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(c) = line.strip_prefix('#') {
            if comment.is_none() {
                comment = Some(c.trim().to_string());
            }
            continue;
        }
        let toks: Vec<&str> = line.split_whitespace().collect();
        if toks.len() < 5 {
            continue;
        }
        let format: u16 = toks[1].parse().ok()?;
        let gain: f64 = toks[2].parse().ok()?;
        if gain == 0.0 || !gain.is_finite() {
            return None;
        }
        let adc_zero: i64 = toks[4].parse().ok()?;
        let name = toks.last()?.to_string();
        leads.push(Lead {
            format,
            gain,
            adc_zero,
            name,
        });
    }
    if leads.len() != nchan {
        return None;
    }
    Some(HeaInfo {
        nchan,
        sample_rate,
        nsamp,
        leads,
        comment,
    })
}

fn twos12(v: u16) -> i32 {
    if v & 0x800 != 0 {
        v as i32 - 0x1000
    } else {
        v as i32
    }
}

pub fn decode_212(data: &[u8], nsamp: usize) -> Option<(Vec<i32>, Vec<i32>)> {
    let total = nsamp.checked_mul(2)?;
    let triples = total / 2;
    if data.len() != triples * 3 {
        return None;
    }
    let mut ch0 = Vec::with_capacity(nsamp);
    let mut ch1 = Vec::with_capacity(nsamp);
    for t in 0..triples {
        let b0 = data[t * 3] as u16;
        let b1 = data[t * 3 + 1] as u16;
        let b2 = data[t * 3 + 2] as u16;
        let hi = b1 & 0x0F;
        ch0.push(twos12((hi << 8) | b0));
        ch1.push(twos12((hi << 8) | b2));
    }
    Some((ch0, ch1))
}

pub fn comp_of_lead(name: &str) -> Option<u32> {
    match name.to_uppercase().as_str() {
        "MLII" => Some(COMP_MLII),
        "V1" => Some(COMP_V1),
        "V2" => Some(COMP_V2),
        "V4" => Some(COMP_V4),
        "V5" => Some(COMP_V5),
        _ => None,
    }
}

pub const BEAT_TYPES: [u16; 13] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];

pub fn parse_atr(bytes: &[u8], sample_rate: u32) -> Option<Vec<(f64, f64)>> {
    let mut t: u64 = 0;
    let mut i = 0usize;
    let mut beats = Vec::new();
    while i + 1 < bytes.len() {
        let v = bytes[i] as u16 | ((bytes[i + 1] as u16) << 8);
        let a = v >> 10;
        let mut interval = (v & 0x3FF) as u64;
        i += 2;
        if a == 59 && interval == 0 && i + 3 < bytes.len() {
            let hi = bytes[i] as u64 | ((bytes[i + 1] as u64) << 8);
            let lo = bytes[i + 2] as u64 | ((bytes[i + 3] as u64) << 8);
            interval = (hi << 16) | lo;
            i += 4;
        } else if a == 63 {
            i += interval as usize + (interval as usize & 1);
            continue;
        } else if a == 60 || a == 61 || a == 62 {
            continue;
        }
        t += interval;
        if BEAT_TYPES.contains(&a) {
            let epoch = t as f64 / sample_rate as f64;
            if let Some(&(prev_epoch, _)) = beats.last() {
                let rr: f64 = epoch - prev_epoch;
                if rr.is_finite() && rr > 0.0 && rr < 5.0 {
                    beats.push((epoch, rr));
                }
            } else {
                beats.push((epoch, f64::NAN));
            }
        }
    }
    if beats.is_empty() {
        return None;
    }
    let first_epoch = beats[0].0;
    Some(
        beats
            .into_iter()
            .filter(|(_, rr)| rr.is_finite())
            .map(|(e, rr)| (e - first_epoch, rr))
            .collect(),
    )
}

pub fn median(vals: &mut Vec<f64>) -> f64 {
    vals.sort_by(|a, b| a.total_cmp(b));
    let n = vals.len();
    if n % 2 == 0 {
        (vals[n / 2 - 1] + vals[n / 2]) * 0.5
    } else {
        vals[n / 2]
    }
}

pub fn decimate(v: &[f32], target: usize) -> Vec<f32> {
    if v.len() <= target {
        return v.to_vec();
    }
    let stride = (v.len() as f64 / target as f64).ceil() as usize;
    v.chunks(stride)
        .map(|c| {
            let mut m: Vec<f64> = c.iter().map(|&x| x as f64).collect();
            median(&mut m) as f32
        })
        .collect()
}

pub fn envelope(
    channel: &[i32],
    gain: f64,
    adc_zero: i64,
    sample_rate: u32,
    bucket_s: f64,
) -> Vec<f32> {
    let mut buckets: Vec<Vec<f64>> = Vec::new();
    for (i, raw) in channel.iter().enumerate() {
        let mv = (*raw as f64 - adc_zero as f64) / gain;
        let epoch = i as f64 / sample_rate as f64;
        let b = (epoch / bucket_s).floor() as usize;
        if b >= buckets.len() {
            buckets.resize(b + 1, Vec::new());
        }
        buckets[b].push(mv.abs());
    }
    buckets
        .into_iter()
        .map(|mut v| median(&mut v) as f32)
        .collect()
}

pub fn write_bin(records: &[(f64, f64, u32)]) -> Vec<u8> {
    let mut buf = Vec::with_capacity(8 + records.len() * 20);
    buf.extend_from_slice(&MAGIC);
    buf.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for (t, val, comp) in records {
        buf.extend_from_slice(&t.to_le_bytes());
        buf.extend_from_slice(&val.to_le_bytes());
        buf.extend_from_slice(&comp.to_le_bytes());
    }
    buf
}

pub fn parse_bin(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    if bytes.len() < 8 || bytes[0..4] != MAGIC {
        return None;
    }
    let n = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    if n > (bytes.len() - 8) / 20 {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    let mut off = 8usize;
    for _ in 0..n {
        let t = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        off += 8;
        let val = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        off += 8;
        let comp = u32::from_le_bytes(bytes.get(off..off + 4)?.try_into().ok()?);
        off += 4;
        if !(COMP_MLII..=COMP_V5).contains(&comp) {
            return None;
        }
        out.push((t, val, comp));
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let records = vec![
            (0.5, 1.02, COMP_MLII),
            (1.0, 2.31, COMP_V1),
            (2.0, 0.98, COMP_V2),
            (3.0, 1.77, COMP_V4),
            (4.0, 3.05, COMP_V5),
        ];
        let bytes = write_bin(&records);
        let parsed = parse_bin(&bytes).unwrap();
        assert_eq!(parsed, records);
    }

    #[test]
    fn rejects_foreign_bytes() {
        assert!(parse_bin(b"X").is_none());
        assert!(parse_bin(b"MITB\x00").is_none());
    }

    #[test]
    fn rejects_unknown_component() {
        let bytes = write_bin(&[(10.0, 1.0, 7)]);
        assert!(parse_bin(&bytes).is_none());
    }
}
