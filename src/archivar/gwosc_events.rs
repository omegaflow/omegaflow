use crate::archivar::{FieldConfig, force_id_of, kernel_id_for_force};

pub const MAGIC: [u8; 4] = *b"GWE1";
pub const HEADER_BYTES: usize = 8;

pub const COMP_FAR: u32 = 0;
pub const COMP_SNR: u32 = 1;
pub const COMP_DISTANCE: u32 = 2;

pub const BIT_FAR: u32 = 1 << 0;
pub const BIT_SNR: u32 = 1 << 1;
pub const BIT_DISTANCE: u32 = 1 << 2;

const COLUMNS: &[(u32, u32, &str, &str)] = &[
    (COMP_FAR, BIT_FAR, "gwosc_event_far_per_year", "a-1"),
    (COMP_SNR, BIT_SNR, "gwosc_event_snr", "1"),
    (COMP_DISTANCE, BIT_DISTANCE, "gwosc_event_distance_mpc", "Mpc"),
];

#[derive(Clone, Debug, PartialEq)]
pub struct GwEvent {
    pub gps: f64,
    pub far: Option<f64>,
    pub snr: Option<f64>,
    pub distance: Option<f64>,
}

pub fn write_bin(events: &[GwEvent]) -> Vec<u8> {
    let mut buf = Vec::with_capacity(HEADER_BYTES + events.len() * 36);
    buf.extend_from_slice(&MAGIC);
    buf.extend_from_slice(&(events.len() as u32).to_le_bytes());
    for e in events {
        buf.extend_from_slice(&e.gps.to_le_bytes());
        let mut mask = 0u32;
        if e.far.is_some() {
            mask |= BIT_FAR;
        }
        if e.snr.is_some() {
            mask |= BIT_SNR;
        }
        if e.distance.is_some() {
            mask |= BIT_DISTANCE;
        }
        buf.extend_from_slice(&mask.to_le_bytes());
        for value in [e.far, e.snr, e.distance].into_iter().flatten() {
            buf.extend_from_slice(&value.to_le_bytes());
        }
    }
    buf
}

pub fn parse_bin(bytes: &[u8]) -> Option<Vec<GwEvent>> {
    if bytes.len() < HEADER_BYTES || bytes[0..4] != MAGIC {
        return None;
    }
    let count = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    let mut off = HEADER_BYTES;
    let mut out = Vec::with_capacity(count);
    for _ in 0..count {
        let gps = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        if !gps.is_finite() || gps < 0.0 {
            return None;
        }
        off += 8;
        let mask = u32::from_le_bytes(bytes.get(off..off + 4)?.try_into().ok()?);
        if mask & !(BIT_FAR | BIT_SNR | BIT_DISTANCE) != 0 {
            return None;
        }
        off += 4;
        let mut read = |bit: u32| -> Option<Option<f64>> {
            if mask & bit == 0 {
                return Some(None);
            }
            let v = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
            off += 8;
            if !v.is_finite() {
                return None;
            }
            Some(Some(v))
        };
        let far = read(BIT_FAR)?;
        let snr = read(BIT_SNR)?;
        let distance = read(BIT_DISTANCE)?;
        out.push(GwEvent {
            gps,
            far,
            snr,
            distance,
        });
    }
    if off != bytes.len() {
        return None;
    }
    Some(out)
}

pub fn names() -> Vec<String> {
    COLUMNS.iter().map(|c| c.2.to_string()).collect()
}

pub fn component_name(comp: u32) -> Option<&'static str> {
    COLUMNS.iter().find(|c| c.0 == comp).map(|c| c.2)
}

pub fn parse_series(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    let events = parse_bin(bytes)?;
    let mut out = Vec::with_capacity(events.len() * COLUMNS.len());
    for e in events {
        let values = [
            (COMP_FAR, e.far),
            (COMP_SNR, e.snr),
            (COMP_DISTANCE, e.distance),
        ];
        for (comp, value) in values {
            if let Some(v) = value {
                out.push((e.gps, v, comp));
            }
        }
    }
    Some(out)
}

pub fn declared_fields(tau: f64) -> Vec<FieldConfig> {
    let Some(channel_ref) = force_id_of("gravity") else {
        return Vec::new();
    };
    let Some(kernel) = kernel_id_for_force(channel_ref) else {
        return Vec::new();
    };
    let mut out = Vec::with_capacity(COLUMNS.len());
    for (_, _, name, unit) in COLUMNS {
        out.push(FieldConfig {
            key: name.to_string(),
            name: name.to_string(),
            band_id: None,
            kernel,
            channel_ref: Some(channel_ref),
            tau,
            tau_src: crate::archivar::types::TauSource::Measured,
            absorption: 0.0,
            advection: 0.0,
            unit: (*unit).to_string(),
            freq: crate::archivar::spectral::SPECTRAL_NO_BAND,
            bin_width: crate::archivar::spectral::SPECTRAL_NO_BAND,
            fold: None,
            aperture: crate::archivar::Aperture::None,
        });
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Vec<GwEvent> {
        vec![
            GwEvent {
                gps: 1126259462.4,
                far: Some(1.0e-7),
                snr: Some(24.4),
                distance: Some(410.0),
            },
            GwEvent {
                gps: 1187008882.4,
                far: Some(1.0e-6),
                snr: None,
                distance: Some(40.0),
            },
            GwEvent {
                gps: 1421382272.1,
                far: None,
                snr: Some(8.7),
                distance: None,
            },
        ]
    }

    #[test]
    fn roundtrip_keeps_absence() {
        let bytes = write_bin(&sample());
        assert_eq!(parse_bin(&bytes).unwrap(), sample());
    }

    #[test]
    fn absent_values_are_not_zero() {
        let bytes = write_bin(&sample());
        let back = parse_bin(&bytes).unwrap();
        assert_eq!(back[1].snr, None);
        assert_eq!(back[2].distance, None);
        assert!(back.iter().all(|e| e.far != Some(0.0)));
    }

    #[test]
    fn series_carries_only_present_columns() {
        let rows = parse_series(&write_bin(&sample())).unwrap();
        assert_eq!(rows.len(), 6);
        assert_eq!(rows.iter().filter(|r| r.2 == COMP_SNR).count(), 2);
    }

    #[test]
    fn rejects_foreign_bytes() {
        assert!(parse_bin(b"").is_none());
        assert!(parse_bin(b"GWE1\x01\x00\x00\x00\x00").is_none());
        assert!(parse_bin(b"BSV1abcdefgh").is_none());
    }
}
