use crate::archivar::{Aperture, FieldConfig, force_id_of, kernel_id_for_force};

pub const MAGIC: [u8; 3] = [0xCF, 0x86, 0x09];
pub const FRAME_HEADER: usize = 19;
pub const SLOTS: usize = 26;
pub const RECORD_BYTES: usize = SLOTS * 8;

pub const SLOT_VAL: usize = 3;
pub const SLOT_EPOCH: usize = 4;
pub const SLOT_TTL: usize = 5;
pub const SLOT_KERNEL: usize = 8;
pub const SLOT_CHANNEL: usize = 9;
pub const SLOT_FREQ: usize = 22;
pub const SLOT_BIN_WIDTH: usize = 23;
pub const SLOT_PRESENCE: usize = 25;

pub const FUNC_GFC: u32 = 0;
pub const FUNC_GFCT: u32 = 1;

pub struct Channel {
    pub comp: u32,
    pub name: &'static str,
    pub unit: &'static str,
}

const CHANNELS: &[Channel] = &[
    Channel {
        comp: 1,
        name: "icgem_gfc_c",
        unit: "1",
    },
    Channel {
        comp: 2,
        name: "icgem_gfc_s",
        unit: "1",
    },
    Channel {
        comp: 3,
        name: "icgem_gfct_c",
        unit: "1",
    },
    Channel {
        comp: 4,
        name: "icgem_gfct_s",
        unit: "1",
    },
];

pub fn channels() -> &'static [Channel] {
    CHANNELS
}

pub fn component_name(comp: u32) -> Option<&'static str> {
    CHANNELS.iter().find(|c| c.comp == comp).map(|c| c.name)
}

pub fn channel_of(functional: u32, sine: bool) -> Option<u32> {
    let base = match functional {
        FUNC_GFC => 1,
        FUNC_GFCT => 3,
        _ => return None,
    };
    Some(base + u32::from(sine))
}

pub fn declared_fields(tau: f64) -> Vec<FieldConfig> {
    let Some(channel_ref) = force_id_of("gravity") else {
        return Vec::new();
    };
    let Some(kernel) = kernel_id_for_force(channel_ref) else {
        return Vec::new();
    };
    let mut out = Vec::with_capacity(CHANNELS.len());
    for c in CHANNELS {
        if component_name(c.comp).is_none() {
            continue;
        }
        out.push(FieldConfig {
            key: c.name.to_string(),
            name: c.name.to_string(),
            band_id: None,
            kernel,
            channel_ref: Some(channel_ref),
            tau,
            absorption: 0.0,
            advection: 0.0,
            unit: c.unit.to_string(),
            freq: crate::archivar::spectral::SPECTRAL_NO_BAND,
            bin_width: crate::archivar::spectral::SPECTRAL_NO_BAND,
            fold: None,
            aperture: Aperture::None,
        });
    }
    out
}

pub fn write_bin(records: &[[f64; 26]]) -> Option<Vec<u8>> {
    if records.is_empty() {
        return None;
    }
    let response_epoch = records
        .iter()
        .map(|r| r[SLOT_EPOCH])
        .fold(f64::NEG_INFINITY, f64::max);
    let mut out = Vec::with_capacity(FRAME_HEADER + records.len() * RECORD_BYTES);
    out.extend_from_slice(&MAGIC);
    out.extend_from_slice(&response_epoch.to_le_bytes());
    out.extend_from_slice(&0u32.to_le_bytes());
    out.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for r in records {
        for v in r {
            out.extend_from_slice(&v.to_le_bytes());
        }
    }
    Some(out)
}

pub fn parse_records(bytes: &[u8]) -> Option<Vec<[f64; 26]>> {
    if bytes.len() < FRAME_HEADER || bytes.get(0..3)? != MAGIC.as_slice() {
        return None;
    }
    let n = u32::from_le_bytes(bytes.get(15..19)?.try_into().ok()?) as usize;
    if bytes.len() != FRAME_HEADER + n * RECORD_BYTES {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    let mut off = FRAME_HEADER;
    for _ in 0..n {
        let mut r = [0.0f64; 26];
        for slot in r.iter_mut() {
            *slot = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
            off += 8;
        }
        out.push(r);
    }
    Some(out)
}

pub fn parse_series(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    let records = parse_records(bytes)?;
    let mut out = Vec::with_capacity(records.len());
    for r in records {
        let val = r[SLOT_VAL];
        let epoch = r[SLOT_EPOCH];
        let channel = r[SLOT_CHANNEL];
        if !channel.is_finite() || channel < 1.0 || channel > CHANNELS.len() as f64 {
            continue;
        }
        let comp = channel as u32;
        if comp as f64 != channel {
            continue;
        }
        if !epoch.is_finite() || !val.is_finite() {
            continue;
        }
        out.push((epoch, val, comp));
    }
    if out.is_empty() { None } else { Some(out) }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn record(epoch: f64, val: f64, channel: f64, n: f64, m: f64) -> [f64; 26] {
        let mut r = [0.0f64; 26];
        r[SLOT_VAL] = val;
        r[SLOT_EPOCH] = epoch;
        r[SLOT_CHANNEL] = channel;
        r[SLOT_FREQ] = n;
        r[SLOT_BIN_WIDTH] = m;
        r[SLOT_PRESENCE] = 1.0;
        r
    }

    #[test]
    fn frame_roundtrips_and_carries_the_wire_magic() {
        let records = vec![
            record(0.0, 1.0, 1.0, 0.0, 0.0),
            record(0.0, -4.84e-4, 2.0, 2.0, 0.0),
        ];
        let bytes = write_bin(&records).expect("non-empty frames");
        assert_eq!(&bytes[0..3], MAGIC.as_slice());
        assert_eq!(bytes.len(), FRAME_HEADER + 2 * RECORD_BYTES);
        assert_eq!(parse_records(&bytes), Some(records));
        assert!(write_bin(&[]).is_none());
    }

    #[test]
    fn series_folds_each_coefficient_onto_its_channel() {
        let bytes = write_bin(&[record(0.0, 2.44e-6, 1.0, 2.0, 2.0)]).expect("frames");
        assert_eq!(parse_series(&bytes), Some(vec![(0.0, 2.44e-6, 1)]));
    }

    #[test]
    fn a_foreign_or_truncated_frame_is_void() {
        assert!(parse_records(b"XXXX").is_none());
        let good = write_bin(&[record(1.0, 1.0, 1.0, 0.0, 0.0)]).expect("frames");
        assert!(parse_records(&good[..good.len() - 1]).is_none());
    }

    #[test]
    fn channels_map_to_their_functional_and_letter() {
        assert_eq!(channel_of(FUNC_GFC, false), Some(1));
        assert_eq!(channel_of(FUNC_GFC, true), Some(2));
        assert_eq!(channel_of(FUNC_GFCT, false), Some(3));
        assert_eq!(channel_of(FUNC_GFCT, true), Some(4));
        assert_eq!(channel_of(9, false), None);
    }

    #[test]
    fn declared_fields_carry_gravity_and_the_read_site_names() {
        let fields = declared_fields(86400.0);
        assert_eq!(fields.len(), CHANNELS.len());
        assert_eq!(fields[0].name, "icgem_gfc_c");
        assert_eq!(fields[0].unit, "1");
        assert_eq!(fields[0].channel_ref, Some(force_id_of("gravity").unwrap()));
        assert_eq!(component_name(4), Some("icgem_gfct_s"));
        assert_eq!(component_name(9), None);
    }
}
