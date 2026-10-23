use crate::archivar::{Aperture, FieldConfig, force_id_of, kernel_id_for_force};

pub const MAGIC: [u8; 4] = *b"CALL";
pub const HEADER_BYTES: usize = 8;
pub const RECORD_BYTES: usize = 8 + 2 + 9 * 8;
pub const BAND_COUNT: usize = 9;
pub const BACKGROUND_HALF_MIN: i64 = 30;

pub const BAND_EDGES_MHZ: [f64; BAND_COUNT + 1] = [
    50.0, 70.0, 95.0, 130.0, 175.0, 235.0, 315.0, 420.0, 560.0, 846.0,
];

const COLUMNS: &[(u32, &str)] = &[
    (0, "callisto_band_01_50_70_mhz"),
    (1, "callisto_band_02_70_95_mhz"),
    (2, "callisto_band_03_95_130_mhz"),
    (3, "callisto_band_04_130_175_mhz"),
    (4, "callisto_band_05_175_235_mhz"),
    (5, "callisto_band_06_235_315_mhz"),
    (6, "callisto_band_07_315_420_mhz"),
    (7, "callisto_band_08_420_560_mhz"),
    (8, "callisto_band_09_560_846_mhz"),
];

pub const UNIT: &str = "digits";

#[derive(Clone, Debug, PartialEq)]
pub struct BandRecord {
    pub t_unix: f64,
    pub bands: [Option<f64>; BAND_COUNT],
}

pub fn write_bin(records: &[BandRecord]) -> Vec<u8> {
    let mut buf = Vec::with_capacity(HEADER_BYTES + records.len() * RECORD_BYTES);
    buf.extend_from_slice(&MAGIC);
    buf.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for r in records {
        buf.extend_from_slice(&r.t_unix.to_le_bytes());
        let mut present: u16 = 0;
        for (i, v) in r.bands.iter().enumerate() {
            if v.is_some() {
                present |= 1 << i;
            }
        }
        buf.extend_from_slice(&present.to_le_bytes());
        for v in r.bands {
            let encoded = match v {
                Some(x) => x,
                None => f64::NAN,
            };
            buf.extend_from_slice(&encoded.to_le_bytes());
        }
    }
    buf
}

pub fn parse_bin(bytes: &[u8]) -> Option<Vec<BandRecord>> {
    if bytes.len() < HEADER_BYTES || bytes[0..4] != MAGIC {
        return None;
    }
    let n = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    if bytes.len() != HEADER_BYTES + n * RECORD_BYTES {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    let mut off = HEADER_BYTES;
    for _ in 0..n {
        let t_unix = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        off += 8;
        let present = u16::from_le_bytes(bytes.get(off..off + 2)?.try_into().ok()?);
        off += 2;
        if !t_unix.is_finite() || present & !0x01ff != 0 {
            return None;
        }
        let mut bands = [None; BAND_COUNT];
        for (i, slot) in bands.iter_mut().enumerate() {
            let raw = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
            off += 8;
            if present & (1 << i) != 0 {
                if !raw.is_finite() {
                    return None;
                }
                *slot = Some(raw);
            }
        }
        out.push(BandRecord { t_unix, bands });
    }
    Some(out)
}

pub fn component_name(comp: u32) -> Option<&'static str> {
    COLUMNS.iter().find(|c| c.0 == comp).map(|c| c.1)
}

pub fn declared_fields(tau: f64) -> Vec<FieldConfig> {
    let Some(channel_ref) = force_id_of("em") else {
        return Vec::new();
    };
    let Some(kernel) = kernel_id_for_force(channel_ref) else {
        return Vec::new();
    };
    COLUMNS
        .iter()
        .map(|(_, name)| FieldConfig {
            key: name.to_string(),
            name: name.to_string(),
            band_id: None,
            kernel,
            channel_ref: Some(channel_ref),
            tau,
            tau_src: crate::archivar::types::TauSource::Measured,
            absorption: 0.0,
            advection: 0.0,
            unit: UNIT.to_string(),
            freq: crate::archivar::spectral::SPECTRAL_NO_BAND,
            bin_width: crate::archivar::spectral::SPECTRAL_NO_BAND,
            fold: None,
            aperture: Aperture::None,
        })
        .collect()
}

pub fn parse_series(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    let records = parse_bin(bytes)?;
    let lsk = crate::archivar::embedded_lsk()?;
    let mut out = Vec::new();
    for r in records {
        let Some(t) = lsk.unix_to_tdb(r.t_unix) else {
            continue;
        };
        for (comp, v) in r.bands.iter().enumerate() {
            if let Some(v) = v {
                out.push((t, *v, comp as u32));
            }
        }
    }
    Some(out)
}

pub fn band_index(freq_mhz: f64) -> Option<usize> {
    if !freq_mhz.is_finite() {
        return None;
    }
    for (i, w) in BAND_EDGES_MHZ.windows(2).enumerate() {
        if freq_mhz >= w[0] && freq_mhz < w[1] {
            return Some(i);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn band_edges_are_strictly_increasing_and_cover_nine_bands() {
        assert_eq!(BAND_EDGES_MHZ.len(), BAND_COUNT + 1);
        assert!(
            BAND_EDGES_MHZ
                .windows(2)
                .all(|w| w[1] > w[0] && w[0].is_finite())
        );
    }

    #[test]
    fn a_frequency_lands_in_exactly_one_band_or_none() {
        assert_eq!(band_index(50.0), Some(0));
        assert_eq!(band_index(69.99), Some(0));
        assert_eq!(band_index(70.0), Some(1));
        assert_eq!(band_index(846.0), None);
        assert_eq!(band_index(49.9), None);
    }

    #[test]
    fn roundtrip_keeps_present_and_absent_bands() {
        let records = vec![
            BandRecord {
                t_unix: 1_700_000_000.0,
                bands: [
                    Some(1.0),
                    None,
                    Some(3.0),
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(9.0),
                ],
            },
            BandRecord {
                t_unix: 1_700_000_060.0,
                bands: [None; BAND_COUNT],
            },
        ];
        let bytes = write_bin(&records);
        assert_eq!(bytes.len(), HEADER_BYTES + 2 * RECORD_BYTES);
        assert_eq!(parse_bin(&bytes), Some(records));
    }
}
