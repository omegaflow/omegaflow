pub const MAGIC: [u8; 4] = *b"EMTF";
pub const HEADER_BYTES: usize = 16;
pub const FIELDS: usize = 9;
pub const COMPONENT_COUNT: usize = 8;
pub const RECORD_BYTES: usize = FIELDS * 8;

pub const COMPONENTS: [(u32, &str, &str); COMPONENT_COUNT] = [
    (0, "emtf_zxx_re", "ohm"),
    (1, "emtf_zxx_im", "ohm"),
    (2, "emtf_zxy_re", "ohm"),
    (3, "emtf_zxy_im", "ohm"),
    (4, "emtf_zyx_re", "ohm"),
    (5, "emtf_zyx_im", "ohm"),
    (6, "emtf_zyy_re", "ohm"),
    (7, "emtf_zyy_im", "ohm"),
];

#[derive(Clone, Debug, PartialEq)]
pub struct EmtfBin {
    pub epoch_unix: f64,
    pub rows: Vec<[f64; FIELDS]>,
}

pub fn parse_bin(bytes: &[u8]) -> Option<EmtfBin> {
    if bytes.len() < HEADER_BYTES || bytes[0..4] != MAGIC {
        return None;
    }
    let n = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    let epoch_unix = f64::from_le_bytes(bytes[8..16].try_into().ok()?);
    if bytes.len() != HEADER_BYTES + n * RECORD_BYTES {
        return None;
    }
    if !epoch_unix.is_finite() {
        return None;
    }
    let mut rows = Vec::with_capacity(n);
    let mut off = HEADER_BYTES;
    for _ in 0..n {
        let mut r = [0.0f64; FIELDS];
        for slot in r.iter_mut() {
            *slot = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
            off += 8;
        }
        if !r.iter().all(|v| v.is_finite()) || !(r[0] > 0.0) {
            return None;
        }
        rows.push(r);
    }
    Some(EmtfBin { epoch_unix, rows })
}

pub fn component_name(comp: u32) -> Option<&'static str> {
    COMPONENTS.iter().find(|c| c.0 == comp).map(|c| c.1)
}

pub fn component_bins(bin: &EmtfBin, comp: usize) -> Vec<(f64, f64, f64)> {
    if comp >= COMPONENT_COUNT {
        return Vec::new();
    }
    let mut points: Vec<(f64, f64)> = Vec::new();
    for row in &bin.rows {
        let period = row[0];
        let value = row[1 + comp];
        if !period.is_finite() || !value.is_finite() || !(period > 0.0) {
            continue;
        }
        let freq = 1.0 / period;
        if !freq.is_finite() || !(freq > 0.0) {
            continue;
        }
        points.push((freq, value));
    }
    points.sort_by(|a, b| a.0.total_cmp(&b.0));
    let freqs: Vec<f64> = points.iter().map(|p| p.0).collect();
    let mut out = Vec::with_capacity(points.len());
    for (i, &(freq, value)) in points.iter().enumerate() {
        let prev = if i > 0 {
            freqs.get(i - 1).copied()
        } else {
            None
        };
        let next = freqs.get(i + 1).copied();
        let bin_width = match (prev, next) {
            (Some(lo), Some(hi)) => (hi - lo) * 0.5,
            (Some(lo), None) => (freq - lo).abs(),
            (None, Some(hi)) => (hi - freq).abs(),
            (None, None) => 0.0,
        };
        out.push((freq, bin_width, value));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pack(epoch_unix: f64, records: &[[f64; FIELDS]]) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(&MAGIC);
        out.extend_from_slice(&(records.len() as u32).to_le_bytes());
        out.extend_from_slice(&epoch_unix.to_le_bytes());
        for r in records {
            for v in r {
                out.extend_from_slice(&v.to_le_bytes());
            }
        }
        out
    }

    fn row(period: f64, zxx_re: f64) -> [f64; FIELDS] {
        [period, zxx_re, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]
    }

    #[test]
    fn roundtrip_holds_for_the_compiler_layout() {
        let records = [row(10.0, 1.5), row(20.0, 2.5)];
        let parsed = parse_bin(&pack(1_275_444_092.0, &records)).expect("bin parses");
        assert_eq!(parsed.epoch_unix, 1_275_444_092.0);
        assert_eq!(parsed.rows, records);
    }

    #[test]
    fn foreign_magic_and_short_body_are_void() {
        assert!(parse_bin(b"XXXX").is_none());
        assert!(parse_bin(b"EMTF").is_none());
        let good = pack(1.0, &[row(10.0, 1.0)]);
        assert!(parse_bin(&good[..good.len() - 1]).is_none());
    }

    #[test]
    fn an_absent_value_is_never_a_fabricated_zero() {
        let mut bytes = pack(1.0, &[row(10.0, 1.0)]);
        let z = HEADER_BYTES + 8;
        bytes[z..z + 8].copy_from_slice(&f64::NAN.to_le_bytes());
        assert!(parse_bin(&bytes).is_none());
    }

    #[test]
    fn a_non_finite_epoch_is_void() {
        assert!(parse_bin(&pack(f64::NAN, &[row(10.0, 1.0)])).is_none());
    }

    #[test]
    fn component_name_maps_the_read_site() {
        assert_eq!(component_name(0), Some("emtf_zxx_re"));
        assert_eq!(component_name(7), Some("emtf_zyy_im"));
        assert_eq!(component_name(8), None);
    }

    #[test]
    fn component_bins_convert_period_to_freq_and_compute_widths() {
        let bin = EmtfBin {
            epoch_unix: 1.0,
            rows: vec![row(10.0, 1.0), row(20.0, 2.0), row(40.0, 3.0)],
        };
        let bins = component_bins(&bin, 0);
        assert_eq!(bins.len(), 3);
        assert!((bins[0].0 - 0.025).abs() < 1e-12);
        assert!((bins[0].1 - 0.025).abs() < 1e-12);
        assert_eq!(bins[0].2, 3.0);
        assert!((bins[1].0 - 0.05).abs() < 1e-12);
        assert!((bins[1].1 - 0.0375).abs() < 1e-12);
        assert_eq!(bins[1].2, 2.0);
        assert!((bins[2].0 - 0.1).abs() < 1e-12);
        assert!((bins[2].1 - 0.05).abs() < 1e-12);
        assert_eq!(bins[2].2, 1.0);
    }

    #[test]
    fn component_bins_refuse_a_non_positive_period() {
        let bin = EmtfBin {
            epoch_unix: 1.0,
            rows: vec![row(0.0, 1.0), row(10.0, 2.0)],
        };
        let bins = component_bins(&bin, 0);
        assert_eq!(bins.len(), 1);
        assert!((bins[0].0 - 0.1).abs() < 1e-12);
        assert_eq!(bins[0].1, 0.0);
    }
}
