use crate::geo::{
    COMP_LAS_CLASSIFICATION, COMP_LAS_INTENSITY, COMP_LAS_X, COMP_LAS_Y, COMP_LAS_Z, MAGIC_LAS,
};

pub const HEADER_BYTES: usize = 8;
pub const REC_BYTES: usize = 35;

pub struct LasSample {
    pub t_tdb: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub intensity: u16,
    pub classification: u8,
}

pub fn write_bin(records: &[LasSample]) -> Vec<u8> {
    let mut out = Vec::with_capacity(HEADER_BYTES + records.len() * REC_BYTES);
    out.extend_from_slice(&MAGIC_LAS);
    out.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for r in records {
        out.extend_from_slice(&r.t_tdb.to_le_bytes());
        out.extend_from_slice(&r.x.to_le_bytes());
        out.extend_from_slice(&r.y.to_le_bytes());
        out.extend_from_slice(&r.z.to_le_bytes());
        out.extend_from_slice(&r.intensity.to_le_bytes());
        out.push(r.classification);
    }
    out
}

pub fn parse_series(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    if bytes.len() < HEADER_BYTES || bytes[0..4] != MAGIC_LAS {
        return None;
    }
    let count = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    if bytes.len() != HEADER_BYTES + count * REC_BYTES {
        return None;
    }
    let mut out = Vec::with_capacity(count * 5);
    for i in 0..count {
        let base = HEADER_BYTES + i * REC_BYTES;
        let rec = bytes.get(base..base + REC_BYTES)?;
        let f64_of = |r: std::ops::Range<usize>| {
            rec.get(r)
                .and_then(|x| x.try_into().ok())
                .map(f64::from_le_bytes)
        };
        let t = f64_of(0..8)?;
        let x = f64_of(8..16)?;
        let y = f64_of(16..24)?;
        let z = f64_of(24..32)?;
        if !t.is_finite() || !x.is_finite() || !y.is_finite() || !z.is_finite() {
            return None;
        }
        let intensity = u16::from_le_bytes(rec[32..34].try_into().ok()?);
        let classification = rec[34];
        out.push((t, x, COMP_LAS_X));
        out.push((t, y, COMP_LAS_Y));
        out.push((t, z, COMP_LAS_Z));
        out.push((t, intensity as f64, COMP_LAS_INTENSITY));
        out.push((t, classification as f64, COMP_LAS_CLASSIFICATION));
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample(t: f64, x: f64, y: f64, z: f64, i: u16, c: u8) -> LasSample {
        LasSample {
            t_tdb: t,
            x,
            y,
            z,
            intensity: i,
            classification: c,
        }
    }

    #[test]
    fn bin_roundtrip_expands_each_sample_into_five_components() {
        let records = vec![
            sample(3.9e8, 1.4e11, -2.1e10, 6.4e6, 42, 2),
            sample(3.9e8 + 1.0, 1.4e11 + 1.0, -2.1e10, 6.4e6, 0, 0),
        ];
        let bytes = write_bin(&records);
        let series = parse_series(&bytes).unwrap();
        assert_eq!(series.len(), 10);
        assert_eq!(series[0], (3.9e8, 1.4e11, COMP_LAS_X));
        assert_eq!(series[1], (3.9e8, -2.1e10, COMP_LAS_Y));
        assert_eq!(series[2], (3.9e8, 6.4e6, COMP_LAS_Z));
        assert_eq!(series[3], (3.9e8, 42.0, COMP_LAS_INTENSITY));
        assert_eq!(series[4], (3.9e8, 2.0, COMP_LAS_CLASSIFICATION));
        assert_eq!(series[5], (3.9e8 + 1.0, 1.4e11 + 1.0, COMP_LAS_X));
        assert_eq!(series[9], (3.9e8 + 1.0, 0.0, COMP_LAS_CLASSIFICATION));
    }

    #[test]
    fn zero_intensity_and_zero_classification_roundtrip_as_measured() {
        let bytes = write_bin(&[sample(3.9e8, 0.0, -1.0, 2.0, 0, 0)]);
        let series = parse_series(&bytes).unwrap();
        assert_eq!(series[3], (3.9e8, 0.0, COMP_LAS_INTENSITY));
        assert_eq!(series[4], (3.9e8, 0.0, COMP_LAS_CLASSIFICATION));
    }

    #[test]
    fn negative_pre_j2000_epoch_roundtrips() {
        let bytes = write_bin(&[sample(-3.1e8, 1.0, 2.0, 3.0, 7, 1)]);
        let series = parse_series(&bytes).unwrap();
        assert_eq!(series[0], (-3.1e8, 1.0, COMP_LAS_X));
    }

    #[test]
    fn parse_rejects_truncated_header_and_unknown_magic() {
        assert!(parse_series(b"X").is_none());
        assert!(parse_series(b"LAS1").is_none());
        let bytes = write_bin(&[sample(1.0, 1.0, 2.0, 3.0, 7, 1)]);
        assert!(parse_series(&bytes[..bytes.len() - 1]).is_none());
    }

    #[test]
    fn parse_rejects_non_finite_sample_fields() {
        let mut bytes = write_bin(&[sample(1.0, 1.0, 2.0, 3.0, 7, 1)]);
        bytes[HEADER_BYTES + 8..HEADER_BYTES + 16].copy_from_slice(&f64::NAN.to_le_bytes());
        assert!(parse_series(&bytes).is_none());
        let mut bytes = write_bin(&[sample(1.0, 1.0, 2.0, 3.0, 7, 1)]);
        bytes[HEADER_BYTES..HEADER_BYTES + 8].copy_from_slice(&f64::INFINITY.to_le_bytes());
        assert!(parse_series(&bytes).is_none());
    }
}
