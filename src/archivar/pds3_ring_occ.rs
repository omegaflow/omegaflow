pub const COMP_SIGNAL_RE: u32 = 1;
pub const COMP_SIGNAL_IM: u32 = 2;

pub const PACK_MAGIC: [u8; 4] = *b"PROC";
pub const PACK_ENTRY_BYTES: usize = 64 + 16 + 4 + 4 + 8 + 8;
pub const PACK_RECORD_BYTES: usize = 24;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RingOccSample {
    pub radius_km: f64,
    pub signal_re: f64,
    pub signal_im: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RingOccFile {
    pub name: String,
    pub direction: String,
    pub samples: Vec<RingOccSample>,
}

pub fn pack(samples: &[RingOccSample], name: &str, direction: &str) -> Vec<u8> {
    pack_many(&[(samples, name, direction)])
}

pub fn pack_many(files: &[(&[RingOccSample], &str, &str)]) -> Vec<u8> {
    let data_start = 8 + files.len() * PACK_ENTRY_BYTES;
    let data_bytes: usize = files
        .iter()
        .map(|(s, _, _)| s.len() * PACK_RECORD_BYTES)
        .sum();
    let mut bin = vec![0u8; data_start + data_bytes];
    bin[0..4].copy_from_slice(&PACK_MAGIC);
    bin[4..8].copy_from_slice(&(files.len() as u32).to_le_bytes());
    let mut offset = data_start;
    for (i, (samples, name, direction)) in files.iter().enumerate() {
        let base = 8 + i * PACK_ENTRY_BYTES;
        let nameb = name.as_bytes();
        let n = nameb.len().min(64);
        bin[base..base + n].copy_from_slice(&nameb[..n]);
        let dirb = direction.as_bytes();
        let d = dirb.len().min(16);
        bin[base + 64..base + 64 + d].copy_from_slice(&dirb[..d]);
        bin[base + 80..base + 84].copy_from_slice(&(samples.len() as u32).to_le_bytes());
        bin[base + 88..base + 96].copy_from_slice(&(offset as u64).to_le_bytes());
        bin[base + 96..base + 104]
            .copy_from_slice(&((samples.len() * PACK_RECORD_BYTES) as u64).to_le_bytes());
        for (j, s) in samples.iter().enumerate() {
            let at = offset + j * PACK_RECORD_BYTES;
            bin[at..at + 8].copy_from_slice(&s.radius_km.to_le_bytes());
            bin[at + 8..at + 16].copy_from_slice(&s.signal_re.to_le_bytes());
            bin[at + 16..at + 24].copy_from_slice(&s.signal_im.to_le_bytes());
        }
        offset += samples.len() * PACK_RECORD_BYTES;
    }
    bin
}

fn str_field(bytes: &[u8], from: usize, to: usize) -> Option<String> {
    let field = bytes.get(from..to)?;
    let end = field.iter().position(|b| *b == 0).unwrap_or(field.len());
    String::from_utf8(field[..end].to_vec()).ok()
}

pub fn parse_packed(bytes: &[u8]) -> Option<Vec<RingOccFile>> {
    if bytes.len() < 8 + PACK_ENTRY_BYTES || bytes[0..4] != PACK_MAGIC {
        return None;
    }
    let count = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    if count == 0 || bytes.len() < 8 + count * PACK_ENTRY_BYTES {
        return None;
    }
    let mut files = Vec::with_capacity(count);
    for i in 0..count {
        let base = 8 + i * PACK_ENTRY_BYTES;
        let entry = &bytes[base..base + PACK_ENTRY_BYTES];
        let name = str_field(entry, 0, 64)?;
        let direction = str_field(entry, 64, 80)?;
        let sample_count = u32::from_le_bytes(entry[80..84].try_into().ok()?) as usize;
        let data_offset = u64::from_le_bytes(entry[88..96].try_into().ok()?) as usize;
        let data_bytes = u64::from_le_bytes(entry[96..104].try_into().ok()?) as usize;
        if data_offset + data_bytes > bytes.len() || data_bytes != sample_count * PACK_RECORD_BYTES
        {
            return None;
        }
        let mut samples = Vec::with_capacity(sample_count);
        for j in 0..sample_count {
            let at = data_offset + j * PACK_RECORD_BYTES;
            let radius_km = f64::from_le_bytes(bytes[at..at + 8].try_into().ok()?);
            let signal_re = f64::from_le_bytes(bytes[at + 8..at + 16].try_into().ok()?);
            let signal_im = f64::from_le_bytes(bytes[at + 16..at + 24].try_into().ok()?);
            if !radius_km.is_finite() || !signal_re.is_finite() || !signal_im.is_finite() {
                return None;
            }
            samples.push(RingOccSample {
                radius_km,
                signal_re,
                signal_im,
            });
        }
        files.push(RingOccFile {
            name,
            direction,
            samples,
        });
    }
    Some(files)
}

pub fn parse_series(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    let files = parse_packed(bytes)?;
    let total: usize = files.iter().map(|f| f.samples.len()).sum();
    let mut out = Vec::with_capacity(total * 2);
    for file in &files {
        for s in &file.samples {
            out.push((s.radius_km, s.signal_re, COMP_SIGNAL_RE));
            out.push((s.radius_km, s.signal_im, COMP_SIGNAL_IM));
        }
    }
    if out.is_empty() { None } else { Some(out) }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample(i: usize) -> RingOccSample {
        RingOccSample {
            radius_km: 70000.0 + i as f64 * 0.2,
            signal_re: 0.5 + i as f64 * 1e-6,
            signal_im: -0.25 + i as f64 * 1e-6,
        }
    }

    #[test]
    fn pack_and_roundtrip_hold() {
        let samples: Vec<RingOccSample> = (0..8).map(sample).collect();
        let bin = pack(&samples, "RS1D1SCI.DAT", "EGRESS");
        let files = parse_packed(&bin).expect("packed parses");
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].name, "RS1D1SCI.DAT");
        assert_eq!(files[0].direction, "EGRESS");
        assert_eq!(files[0].samples, samples);
    }

    #[test]
    fn parse_series_emits_both_components_on_radius_axis() {
        let samples: Vec<RingOccSample> = (0..3).map(sample).collect();
        let bin = pack(&samples, "RS1D1SCI.DAT", "INGRESS");
        let series = parse_series(&bin).expect("series parses");
        assert_eq!(series.len(), 6);
        assert_eq!(series[0], (70000.0, samples[0].signal_re, COMP_SIGNAL_RE));
        assert_eq!(series[1], (70000.0, samples[0].signal_im, COMP_SIGNAL_IM));
        assert_eq!(series[2].0, 70000.2);
        assert_eq!(series[2].2, COMP_SIGNAL_RE);
        assert_eq!(series[5].2, COMP_SIGNAL_IM);
    }

    #[test]
    fn parse_packed_rejects_void_and_truncated() {
        assert!(parse_packed(b"").is_none());
        assert!(parse_packed(b"XXXX").is_none());
        let samples: Vec<RingOccSample> = (0..4).map(sample).collect();
        let mut bin = pack(&samples, "RS1D1SCI.DAT", "EGRESS");
        bin.truncate(bin.len() - 1);
        assert!(parse_packed(&bin).is_none());
    }
}
