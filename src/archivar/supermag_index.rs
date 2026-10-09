use crate::archivar::{
    Aperture, FieldConfig, JsonVal, force_id_of, jpath, kernel_id_for_force, parse_json,
};

pub const MAGIC: [u8; 4] = *b"SMIX";
pub const HEADER_BYTES: usize = 8;
pub const RECORD_BYTES: usize = 8 + 1 + 3 * 8;

pub const COMP_SME_NT: u32 = 1;
pub const COMP_SML_NT: u32 = 2;
pub const COMP_SMU_NT: u32 = 3;

const COLUMNS: &[(u32, &str, &str)] = &[
    (COMP_SME_NT, "supermag_sme_nt", "nT"),
    (COMP_SML_NT, "supermag_sml_nt", "nT"),
    (COMP_SMU_NT, "supermag_smu_nt", "nT"),
];

#[derive(Clone, Debug, PartialEq)]
pub struct IndexRecord {
    pub t_unix: f64,
    pub sme_nt: Option<f64>,
    pub sml_nt: Option<f64>,
    pub smu_nt: Option<f64>,
}

impl IndexRecord {
    fn values(&self) -> [Option<f64>; 3] {
        [self.sme_nt, self.sml_nt, self.smu_nt]
    }
}

fn finite(v: Option<f64>) -> Option<f64> {
    v.filter(|x| x.is_finite())
}

pub fn parse_text(body: &str) -> Vec<IndexRecord> {
    let Some(start) = body.find('[') else {
        return Vec::new();
    };
    let Some(JsonVal::Arr(rows)) = parse_json(&body[start..]) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for row in &rows {
        let Some(t_unix) = jpath(row, "tval") else {
            continue;
        };
        if !t_unix.is_finite() {
            continue;
        }
        let rec = IndexRecord {
            t_unix,
            sme_nt: finite(jpath(row, "SME")),
            sml_nt: finite(jpath(row, "SML")),
            smu_nt: finite(jpath(row, "SMU")),
        };
        if rec.values().iter().any(|v| v.is_some()) {
            out.push(rec);
        }
    }
    out
}

pub fn write_bin(records: &[IndexRecord]) -> Vec<u8> {
    let mut buf = Vec::with_capacity(HEADER_BYTES + records.len() * RECORD_BYTES);
    buf.extend_from_slice(&MAGIC);
    buf.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for r in records {
        buf.extend_from_slice(&r.t_unix.to_le_bytes());
        let mut present = 0u8;
        let mut vals = [0.0f64; 3];
        for (i, v) in r.values().iter().enumerate() {
            if let Some(x) = v {
                present |= 1 << i;
                vals[i] = *x;
            }
        }
        buf.push(present);
        for x in vals {
            buf.extend_from_slice(&x.to_le_bytes());
        }
    }
    buf
}

pub fn parse_bin(bytes: &[u8]) -> Option<Vec<IndexRecord>> {
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
        let present = *bytes.get(off)?;
        off += 1;
        if !t_unix.is_finite() {
            return None;
        }
        let mut vals = [None; 3];
        for (i, slot) in vals.iter_mut().enumerate() {
            let raw = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
            off += 8;
            if present & (1 << i) != 0 {
                if !raw.is_finite() {
                    return None;
                }
                *slot = Some(raw);
            }
        }
        out.push(IndexRecord {
            t_unix,
            sme_nt: vals[0],
            sml_nt: vals[1],
            smu_nt: vals[2],
        });
    }
    Some(out)
}

pub fn component_name(comp: u32) -> Option<&'static str> {
    COLUMNS.iter().find(|c| c.0 == comp).map(|c| c.1)
}

pub fn declared_fields(tau: f64) -> Vec<FieldConfig> {
    let Some(force) = force_id_of("em") else {
        return Vec::new();
    };
    let Some(kernel) = kernel_id_for_force(force) else {
        return Vec::new();
    };
    COLUMNS
        .iter()
        .map(|(_, name, unit)| FieldConfig {
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

pub fn parse_series(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    let records = parse_bin(bytes)?;
    let lsk = crate::archivar::embedded_lsk()?;
    let mut out = Vec::new();
    for r in records {
        let Some(t) = lsk.unix_to_tdb(r.t_unix) else {
            continue;
        };
        for (comp, v) in [
            (COMP_SME_NT, r.sme_nt),
            (COMP_SML_NT, r.sml_nt),
            (COMP_SMU_NT, r.smu_nt),
        ] {
            if let Some(v) = v {
                out.push((t, v, comp));
            }
        }
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    const BODY: &str = "OK\n[{\"tval\": 1715305980.0, \"SME\": 517.263184, \"SML\": -296.796356, \"SMU\": 220.466827},{\"tval\": 1715306040.0, \"SME\": 514.162537, \"SML\": -295.492981, \"SMU\": 218.669556}]";

    #[test]
    fn index_rows_carry_sme_sml_smu_at_the_stamp() {
        let records = parse_text(BODY);
        assert_eq!(records.len(), 2);
        assert_eq!(records[0].t_unix, 1715305980.0);
        assert_eq!(records[0].sme_nt, Some(517.263184));
        assert_eq!(records[0].sml_nt, Some(-296.796356));
        assert_eq!(records[0].smu_nt, Some(220.466827));
    }

    #[test]
    fn an_absent_index_is_never_a_fabricated_zero() {
        let body = "OK\n[{\"tval\": 1715305980.0, \"SME\": 517.0}]";
        let records = parse_text(body);
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].sme_nt, Some(517.0));
        assert_eq!(records[0].sml_nt, None);
        assert_eq!(records[0].smu_nt, None);
    }

    #[test]
    fn a_row_with_no_measured_index_is_dropped() {
        let body = "OK\n[{\"tval\": 1715305980.0}]";
        assert!(parse_text(body).is_empty());
    }

    #[test]
    fn a_body_without_a_json_array_is_void() {
        assert!(parse_text("ERROR: No username").is_empty());
        assert!(parse_text("OK\n[]").is_empty());
    }

    #[test]
    fn roundtrip_keeps_present_and_absent_columns() {
        let records = parse_text(BODY);
        let bytes = write_bin(&records);
        assert_eq!(bytes.len(), HEADER_BYTES + 2 * RECORD_BYTES);
        assert_eq!(parse_bin(&bytes), Some(records));
    }

    #[test]
    fn declared_fields_carry_the_read_site_names_and_units() {
        let fields = declared_fields(86400.0);
        assert_eq!(fields.len(), COLUMNS.len());
        assert_eq!(fields[0].name, "supermag_sme_nt");
        assert_eq!(fields[2].name, "supermag_smu_nt");
        assert_eq!(fields[0].unit, "nT");
        assert_eq!(component_name(COMP_SML_NT), Some("supermag_sml_nt"));
        assert_eq!(component_name(9), None);
    }

    #[test]
    fn a_truncated_or_foreign_asset_is_void() {
        assert!(parse_bin(b"XXXX").is_none());
        let bytes = write_bin(&parse_text(BODY));
        assert!(parse_bin(&bytes[..bytes.len() - 1]).is_none());
    }
}
