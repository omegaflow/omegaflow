use crate::archivar::{
    Aperture, FieldConfig, J2000_EPOCH, J2000_UNIX_OFFSET, SECONDS_PER_DAY, force_id_of,
    kernel_id_for_force,
};

pub const MAGIC: [u8; 4] = *b"MCON";
pub const VERSION: u8 = 1;
pub const HEADER_LEN: usize = 13;
pub const REC_BYTES: usize = 32;

pub const PRES_TIME: u8 = 0x01;
pub const PRES_SML: u8 = 0x02;
pub const PRES_AMPLITUDE: u8 = 0x04;
pub const PRES_BAY_DECREASE: u8 = 0x08;
pub const PRES_BAY_DURATION: u8 = 0x10;
pub const PRES_BAY_WIDTH: u8 = 0x20;
pub const PRES_ALL: u8 = 0x3F;

pub const COMP_SML: u32 = 1;
pub const COMP_AMPLITUDE: u32 = 2;
pub const COMP_BAY_DECREASE: u32 = 3;
pub const COMP_BAY_DURATION: u32 = 4;
pub const COMP_BAY_WIDTH: u32 = 5;

const COLUMNS: &[(u32, &str, &str)] = &[
    (COMP_SML, "mcon_sml_nt", "nT"),
    (COMP_AMPLITUDE, "mcon_amplitude_nt", "nT"),
    (COMP_BAY_DECREASE, "mcon_bay_decrease_nt", "nT"),
    (COMP_BAY_DURATION, "mcon_bay_duration_min", "min"),
    (COMP_BAY_WIDTH, "mcon_bay_width_min", "min"),
];

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
        .map(|(_, name, unit)| FieldConfig {
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
            aperture: Aperture::None,
        })
        .collect()
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct MconEvent {
    pub present: u8,
    pub jd_utc: f64,
    pub sml: f32,
    pub amplitude: f32,
    pub bay_decrease: f32,
    pub bay_duration: f32,
    pub bay_width: f32,
}

pub fn jd_from_unix(unix_seconds: f64) -> Option<f64> {
    if !unix_seconds.is_finite() {
        return None;
    }
    let jd = J2000_EPOCH + (unix_seconds - J2000_UNIX_OFFSET) / SECONDS_PER_DAY;
    if !jd.is_finite() {
        return None;
    }
    Some(jd)
}

pub fn unix_from_jd(jd_utc: f64) -> Option<f64> {
    if !jd_utc.is_finite() {
        return None;
    }
    let unix = (jd_utc - J2000_EPOCH) * SECONDS_PER_DAY + J2000_UNIX_OFFSET;
    if !unix.is_finite() {
        return None;
    }
    Some(unix)
}

pub fn write_header(out: &mut Vec<u8>, n_rows: u64) {
    out.extend_from_slice(&MAGIC);
    out.push(VERSION);
    out.extend_from_slice(&n_rows.to_le_bytes());
}

pub fn parse_header(b: &[u8]) -> Option<u64> {
    if b.len() < HEADER_LEN || b[0..4] != MAGIC || b[4] != VERSION {
        return None;
    }
    Some(u64::from_le_bytes(b[5..13].try_into().ok()?))
}

pub fn encode_rec(out: &mut [u8; REC_BYTES], r: &MconEvent) {
    out.fill(0);
    out[0] = r.present;
    out[4..12].copy_from_slice(&r.jd_utc.to_le_bytes());
    out[12..16].copy_from_slice(&r.sml.to_le_bytes());
    out[16..20].copy_from_slice(&r.amplitude.to_le_bytes());
    out[20..24].copy_from_slice(&r.bay_decrease.to_le_bytes());
    out[24..28].copy_from_slice(&r.bay_duration.to_le_bytes());
    out[28..32].copy_from_slice(&r.bay_width.to_le_bytes());
}

pub fn decode_rec(b: &[u8]) -> Option<MconEvent> {
    if b.len() != REC_BYTES {
        return None;
    }
    let present = b[0];
    if present == 0 || present & !PRES_ALL != 0 {
        return None;
    }
    let jd_utc = f64::from_le_bytes(b[4..12].try_into().ok()?);
    if !jd_utc.is_finite() || jd_utc <= 0.0 {
        return None;
    }
    let sml = f32::from_le_bytes(b[12..16].try_into().ok()?);
    let amplitude = f32::from_le_bytes(b[16..20].try_into().ok()?);
    let bay_decrease = f32::from_le_bytes(b[20..24].try_into().ok()?);
    let bay_duration = f32::from_le_bytes(b[24..28].try_into().ok()?);
    let bay_width = f32::from_le_bytes(b[28..32].try_into().ok()?);
    if !(sml.is_finite()
        && amplitude.is_finite()
        && bay_decrease.is_finite()
        && bay_duration.is_finite()
        && bay_width.is_finite())
    {
        return None;
    }
    Some(MconEvent {
        present,
        jd_utc,
        sml,
        amplitude,
        bay_decrease,
        bay_duration,
        bay_width,
    })
}

pub fn parse_bin(bytes: &[u8]) -> Option<Vec<MconEvent>> {
    let n = parse_header(bytes)? as usize;
    let body = bytes.get(HEADER_LEN..)?;
    if body.len() < n * REC_BYTES {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let chunk = body.get(i * REC_BYTES..(i + 1) * REC_BYTES)?;
        let fixed: [u8; REC_BYTES] = chunk.try_into().ok()?;
        out.push(decode_rec(&fixed)?);
    }
    Some(out)
}

pub fn parse_series(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    let events = parse_bin(bytes)?;
    let mut out = Vec::with_capacity(events.len() * 5);
    for e in events {
        let unix = unix_from_jd(e.jd_utc)?;
        for (pres, v, comp) in [
            (PRES_SML, e.sml, COMP_SML),
            (PRES_AMPLITUDE, e.amplitude, COMP_AMPLITUDE),
            (PRES_BAY_DECREASE, e.bay_decrease, COMP_BAY_DECREASE),
            (PRES_BAY_DURATION, e.bay_duration, COMP_BAY_DURATION),
            (PRES_BAY_WIDTH, e.bay_width, COMP_BAY_WIDTH),
        ] {
            if e.present & pres != 0 {
                out.push((unix, v as f64, comp));
            }
        }
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> MconEvent {
        MconEvent {
            present: PRES_ALL,
            jd_utc: 2439314.5,
            sml: -178.0,
            amplitude: 163.0,
            bay_decrease: 134.0,
            bay_duration: 12.0,
            bay_width: 36.0,
        }
    }

    #[test]
    fn roundtrip_keeps_every_component() {
        let r = sample();
        let mut buf = [0u8; REC_BYTES];
        encode_rec(&mut buf, &r);
        assert_eq!(decode_rec(&buf), Some(r));
    }

    #[test]
    fn absent_components_are_not_emitted() {
        let r = MconEvent {
            present: PRES_TIME | PRES_SML,
            ..sample()
        };
        let mut bin = Vec::new();
        write_header(&mut bin, 1);
        let mut buf = [0u8; REC_BYTES];
        encode_rec(&mut buf, &r);
        bin.extend_from_slice(&buf);
        let series = parse_series(&bin).unwrap();
        assert_eq!(series.len(), 1);
        assert_eq!(series[0].2, COMP_SML);
    }

    #[test]
    fn a_present_bit_of_zero_is_refused() {
        let r = MconEvent {
            present: 0,
            ..sample()
        };
        let mut buf = [0u8; REC_BYTES];
        encode_rec(&mut buf, &r);
        assert_eq!(decode_rec(&buf), None);
    }

    #[test]
    fn jd_and_unix_roundtrip() {
        let unix = 1_000_000_000.0;
        let jd = jd_from_unix(unix).unwrap();
        assert!((unix_from_jd(jd).unwrap() - unix).abs() < 1.0);
    }

    #[test]
    fn declared_fields_carry_the_read_site_names_and_units() {
        let fields = declared_fields(86400.0);
        assert_eq!(fields.len(), COLUMNS.len());
        assert_eq!(fields[0].name, "mcon_sml_nt");
        assert_eq!(fields[0].unit, "nT");
        assert_eq!(fields[3].name, "mcon_bay_duration_min");
        assert_eq!(fields[3].unit, "min");
        assert_eq!(fields[0].channel_ref, Some(force_id_of("em").unwrap()));
        assert_eq!(component_name(COMP_BAY_WIDTH), Some("mcon_bay_width_min"));
        assert_eq!(component_name(9), None);
    }
}
