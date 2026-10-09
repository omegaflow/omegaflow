use crate::archivar::json::{JsonVal, jnum, jpath_val, parse_json};
use crate::archivar::{Aperture, FieldConfig, force_id_of, kernel_id_for_force};
use std::collections::BTreeMap;

pub const COMP_EX: u32 = 0;
pub const COMP_EY: u32 = 1;

pub const MAGIC: [u8; 4] = *b"SWEF";
pub const HEADER_BYTES: usize = 8;
pub const RECORD_BYTES: usize = 8 + 1 + 2 * 8;

const COLUMNS: &[(u32, &str, &str, &str)] = &[
    (COMP_EX, "swpc_efield_ex", "mV/km", "Ex"),
    (COMP_EY, "swpc_efield_ey", "mV/km", "Ey"),
];

pub fn component_name(comp: u32) -> Option<&'static str> {
    COLUMNS.iter().find(|c| c.0 == comp).map(|c| c.1)
}

pub fn declared_fields(tau: f64) -> Vec<FieldConfig> {
    let Some(force) = force_id_of("electric") else {
        return Vec::new();
    };
    let Some(kernel) = kernel_id_for_force(force) else {
        return Vec::new();
    };
    COLUMNS
        .iter()
        .map(|(_, name, unit, _)| FieldConfig {
            key: name.to_string(),
            name: name.to_string(),
            band_id: None,
            kernel,
            force,
            tau,
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

fn mean_of(features: &[JsonVal], key: &str) -> Option<f64> {
    let mut sum = 0.0f64;
    let mut n = 0usize;
    for feature in features {
        let Some(props) = jpath_val(feature, "properties") else {
            continue;
        };
        let Some(v) = jnum(props, key) else {
            continue;
        };
        if !v.is_finite() {
            continue;
        }
        sum += v;
        n += 1;
    }
    if n == 0 { None } else { Some(sum / n as f64) }
}

pub fn parse_frame(bytes: &[u8]) -> Option<(Option<f64>, Option<f64>)> {
    let text = std::str::from_utf8(bytes).ok()?;
    let json = parse_json(text)?;
    let JsonVal::Arr(features) = jpath_val(&json, "features")? else {
        return None;
    };
    let ex = mean_of(features, "Ex");
    let ey = mean_of(features, "Ey");
    if ex.is_none() && ey.is_none() {
        None
    } else {
        Some((ex, ey))
    }
}

pub fn write_bin(rows: &[(f64, f64, u32)]) -> Vec<u8> {
    let mut grouped: BTreeMap<u64, (u8, [f64; 2])> = BTreeMap::new();
    for &(t, val, comp) in rows {
        if comp > COMP_EY {
            continue;
        }
        let slot = grouped.entry(t.to_bits()).or_insert((0u8, [0.0; 2]));
        slot.0 |= 1 << comp;
        slot.1[comp as usize] = val;
    }
    let mut buf = Vec::with_capacity(HEADER_BYTES + grouped.len() * RECORD_BYTES);
    buf.extend_from_slice(&MAGIC);
    buf.extend_from_slice(&(grouped.len() as u32).to_le_bytes());
    for (bits, (present, vals)) in grouped {
        buf.extend_from_slice(&f64::from_bits(bits).to_le_bytes());
        buf.push(present);
        for x in vals {
            buf.extend_from_slice(&x.to_le_bytes());
        }
    }
    buf
}

pub fn parse_bin(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    if bytes.len() < HEADER_BYTES || bytes[0..4] != MAGIC {
        return None;
    }
    let n = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    if bytes.len() != HEADER_BYTES + n * RECORD_BYTES {
        return None;
    }
    let mut out = Vec::with_capacity(n * 2);
    let mut off = HEADER_BYTES;
    for _ in 0..n {
        let t = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        off += 8;
        if !t.is_finite() {
            return None;
        }
        let present = *bytes.get(off)?;
        off += 1;
        if present & !0b11 != 0 {
            return None;
        }
        let ex = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        off += 8;
        let ey = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        off += 8;
        for (comp, bit, v) in [(COMP_EX, 1u8, ex), (COMP_EY, 2u8, ey)] {
            if present & bit == 0 {
                continue;
            }
            if !v.is_finite() {
                return None;
            }
            out.push((t, v, comp));
        }
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE: &str = concat!(
        r#"{"time_tag": "2026-10-08", "cadence": 60, "product_version": "US-Canada-1D", "#,
        r#""type": "FeatureCollection", "features": ["#,
        r#"{"type": "Feature", "geometry": {"type": "Point", "coordinates": [-139.0, 60.0]}, "#,
        r#""properties": {"Ex": 1.0, "Ey": -2.0, "quality_flag": 5}}, "#,
        r#"{"type": "Feature", "geometry": {"type": "Point", "coordinates": [-138.0, 61.0]}, "#,
        r#""properties": {"Ex": 3.0, "Ey": null, "quality_flag": 4}}, "#,
        r#"{"type": "Feature", "geometry": {"type": "Point", "coordinates": [-137.0, 62.0]}, "#,
        r#""properties": {"Ex": null, "Ey": -4.0, "quality_flag": 3}}]}"#,
    );

    #[test]
    fn a_frame_carries_the_signed_continental_grid_mean_of_ex_and_ey() {
        let (ex, ey) = parse_frame(FIXTURE.as_bytes()).expect("the frame parses");
        assert_eq!(ex, Some(2.0));
        assert_eq!(ey, Some(-3.0));
    }

    #[test]
    fn a_null_or_missing_component_stays_absent_from_the_mean() {
        let body = concat!(
            r#"{"features": ["#,
            r#"{"properties": {"Ex": 5.0, "Ey": null}},"#,
            r#"{"properties": {"Ex": null, "Ey": -1.0}}, "#,
            r#"{"properties": {"Ex": 7.0}}]}"#,
        );
        let (ex, ey) = parse_frame(body.as_bytes()).unwrap();
        assert_eq!(ex, Some(6.0));
        assert_eq!(ey, Some(-1.0));
    }

    #[test]
    fn a_frame_without_a_measurable_component_is_void() {
        assert!(parse_frame(b"").is_none());
        assert!(parse_frame(b"not json").is_none());
        assert!(parse_frame(br#"{"time_tag": "2026-10-08"}"#).is_none());
        assert!(parse_frame(br#"{"features": []}"#).is_none());
        assert!(
            parse_frame(br#"{"features": [{"properties": {"Ex": null, "Ey": null}}]}"#).is_none()
        );
    }

    #[test]
    fn a_zero_is_a_real_measurement() {
        let (ex, ey) = parse_frame(br#"{"features": [{"properties": {"Ex": 0.0, "Ey": 0.0}}]}"#)
            .expect("a zero field is measured");
        assert_eq!(ex, Some(0.0));
        assert_eq!(ey, Some(0.0));
    }

    #[test]
    fn roundtrip_keeps_present_and_absent_components() {
        let rows = vec![
            (100.0, 1.5, COMP_EX),
            (100.0, -2.0, COMP_EY),
            (200.0, 0.5, COMP_EX),
        ];
        let bytes = write_bin(&rows);
        assert_eq!(bytes.len(), HEADER_BYTES + 2 * RECORD_BYTES);
        assert_eq!(parse_bin(&bytes), Some(rows));
    }

    #[test]
    fn an_absent_component_stays_absent_through_the_roundtrip() {
        let bytes = write_bin(&[(100.0, 1.5, COMP_EX)]);
        let parsed = parse_bin(&bytes).unwrap();
        assert_eq!(parsed, vec![(100.0, 1.5, COMP_EX)]);
    }

    #[test]
    fn a_truncated_or_foreign_asset_is_void() {
        assert!(parse_bin(b"XXXX").is_none());
        let bytes = write_bin(&[(100.0, 1.5, COMP_EX)]);
        assert!(parse_bin(&bytes[..bytes.len() - 1]).is_none());
    }

    #[test]
    fn declared_fields_carry_the_read_site_names_and_units() {
        let fields = declared_fields(86400.0);
        assert_eq!(fields.len(), COLUMNS.len());
        assert_eq!(fields[0].name, "swpc_efield_ex");
        assert_eq!(fields[0].unit, "mV/km");
        assert_eq!(fields[1].name, "swpc_efield_ey");
        assert_eq!(fields[0].force, force_id_of("electric").unwrap());
        assert_eq!(component_name(COMP_EY), Some("swpc_efield_ey"));
        assert_eq!(component_name(9), None);
    }
}
