use crate::archivar::{FieldConfig, force_id_of, kernel_id_for_force};

pub const MAGIC: [u8; 4] = *b"AGRA";
pub const HEADER_BYTES: usize = 8;
pub const RECORD_BYTES: usize = 16;

pub const COMP_GRAVITY: u32 = 0;

const COLUMNS: &[(u32, &str, &str)] = &[(COMP_GRAVITY, "agrav_gravity_ms2", "m/s2")];

#[derive(Clone, Debug, PartialEq)]
pub struct AgravRecord {
    pub epoch_tdb: f64,
    pub gravity: f64,
}

pub fn plausible_gravity(v: f64) -> bool {
    v.is_finite() && v > 0.0
}

pub fn write_bin(records: &[AgravRecord]) -> Option<Vec<u8>> {
    let count = u32::try_from(records.len()).ok()?;
    let mut out = Vec::with_capacity(HEADER_BYTES + records.len() * RECORD_BYTES);
    out.extend_from_slice(&MAGIC);
    out.extend_from_slice(&count.to_le_bytes());
    for r in records {
        if !r.epoch_tdb.is_finite() || !plausible_gravity(r.gravity) {
            return None;
        }
        out.extend_from_slice(&r.epoch_tdb.to_le_bytes());
        out.extend_from_slice(&r.gravity.to_le_bytes());
    }
    Some(out)
}

pub fn parse_records(bytes: &[u8]) -> Option<Vec<AgravRecord>> {
    if bytes.len() < HEADER_BYTES || bytes[0..4] != MAGIC {
        return None;
    }
    let count = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    if bytes.len() != HEADER_BYTES + count * RECORD_BYTES {
        return None;
    }
    let mut off = HEADER_BYTES;
    let mut out = Vec::with_capacity(count);
    for _ in 0..count {
        let epoch_tdb = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        let gravity = f64::from_le_bytes(bytes.get(off + 8..off + 16)?.try_into().ok()?);
        if !epoch_tdb.is_finite() || !plausible_gravity(gravity) {
            return None;
        }
        off += RECORD_BYTES;
        out.push(AgravRecord { epoch_tdb, gravity });
    }
    Some(out)
}

pub fn component_name(comp: u32) -> Option<&'static str> {
    COLUMNS.iter().find(|c| c.0 == comp).map(|c| c.1)
}

pub fn declared_fields(tau: f64) -> Vec<FieldConfig> {
    let Some(force) = force_id_of("gravity") else {
        return Vec::new();
    };
    let Some(kernel) = kernel_id_for_force(force) else {
        return Vec::new();
    };
    let mut out = Vec::with_capacity(COLUMNS.len());
    for (comp, name, unit) in COLUMNS {
        if component_name(*comp).is_none() {
            continue;
        }
        out.push(FieldConfig {
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
            aperture: crate::archivar::Aperture::None,
        });
    }
    out
}

pub fn parse_series(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    let records = parse_records(bytes)?;
    Some(
        records
            .into_iter()
            .map(|r| (r.epoch_tdb, r.gravity, COMP_GRAVITY))
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn record(epoch_tdb: f64, gravity: f64) -> AgravRecord {
        AgravRecord { epoch_tdb, gravity }
    }

    #[test]
    fn roundtrip_keeps_the_epoch_and_the_gravity() {
        let records = vec![record(0.0, 9.7803359), record(123456789.0, 9.794341)];
        let bytes = write_bin(&records).expect("finite records encode");
        assert_eq!(bytes.len(), HEADER_BYTES + 2 * RECORD_BYTES);
        assert_eq!(parse_records(&bytes), Some(records));
    }

    #[test]
    fn series_folds_each_record_onto_its_own_epoch() {
        let bytes = write_bin(&[record(42.5, 9.78)]).expect("finite record encodes");
        let series = parse_series(&bytes).expect("the encoded asset reads back");
        assert_eq!(series, vec![(42.5, 9.78, COMP_GRAVITY)]);
    }

    #[test]
    fn an_implausible_gravity_is_refused_at_the_write_site() {
        assert!(write_bin(&[record(0.0, 0.0)]).is_none());
        assert!(write_bin(&[record(0.0, -1.0)]).is_none());
        assert!(write_bin(&[record(0.0, f64::NAN)]).is_none());
        assert!(write_bin(&[record(f64::NAN, 9.78)]).is_none());
    }

    #[test]
    fn a_truncated_or_foreign_asset_is_void() {
        assert!(parse_records(b"XXXX").is_none());
        let bytes = write_bin(&[record(0.0, 9.78)]).expect("finite record encodes");
        assert!(parse_records(&bytes[..bytes.len() - 1]).is_none());
    }

    #[test]
    fn declared_fields_carry_the_read_site_name_and_unit() {
        let fields = declared_fields(604800.0);
        assert_eq!(fields.len(), COLUMNS.len());
        assert_eq!(fields[0].name, "agrav_gravity_ms2");
        assert_eq!(fields[0].unit, "m/s2");
        assert_eq!(fields[0].force, force_id_of("gravity").unwrap());
        assert_eq!(component_name(COMP_GRAVITY), Some("agrav_gravity_ms2"));
        assert_eq!(component_name(9), None);
    }
}
