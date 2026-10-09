use crate::archivar::hapi_csv::parse_iso_seconds;
use crate::archivar::{Aperture, FieldConfig, force_id_of, kernel_id_for_force};

pub const MAGIC: [u8; 4] = *b"SBSM";
pub const HEADER_BYTES: usize = 8;
pub const RECORD_BYTES: usize = 8 + 1 + 4 * 8;

pub const COMP_MLT: u32 = 0;
pub const COMP_MLAT: u32 = 1;
pub const COMP_GLON: u32 = 2;
pub const COMP_GLAT: u32 = 3;

const COLUMNS: &[(u32, &str, &str)] = &[
    (COMP_MLT, "substorm_mlt_h", "h"),
    (COMP_MLAT, "substorm_mlat_deg", "deg"),
    (COMP_GLON, "substorm_glon_deg", "deg"),
    (COMP_GLAT, "substorm_glat_deg", "deg"),
];

#[derive(Clone, Debug, PartialEq)]
pub struct SubstormOnset {
    pub t_unix: f64,
    pub mlt: Option<f64>,
    pub mlat: Option<f64>,
    pub glon: Option<f64>,
    pub glat: Option<f64>,
}

impl SubstormOnset {
    fn values(&self) -> [Option<f64>; 4] {
        [self.mlt, self.mlat, self.glon, self.glat]
    }
}

fn parse_num(cell: Option<&&str>) -> Option<f64> {
    let t = cell?.trim();
    if t.is_empty() {
        return None;
    }
    t.parse::<f64>().ok().filter(|v| v.is_finite())
}

pub fn parse_csv(text: &str) -> Vec<SubstormOnset> {
    let mut out = Vec::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let cells: Vec<&str> = line.split(',').map(str::trim).collect();
        if cells.len() < 5 {
            continue;
        }
        let Some(t_unix) = parse_iso_seconds(cells[0]) else {
            continue;
        };
        let rec = SubstormOnset {
            t_unix,
            mlt: parse_num(cells.get(1)),
            mlat: parse_num(cells.get(2)),
            glon: parse_num(cells.get(3)),
            glat: parse_num(cells.get(4)),
        };
        if rec.values().iter().any(|v| v.is_some()) {
            out.push(rec);
        }
    }
    out
}

pub fn write_bin(records: &[SubstormOnset]) -> Vec<u8> {
    let mut buf = Vec::with_capacity(HEADER_BYTES + records.len() * RECORD_BYTES);
    buf.extend_from_slice(&MAGIC);
    buf.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for r in records {
        buf.extend_from_slice(&r.t_unix.to_le_bytes());
        let mut present = 0u8;
        let mut vals = [0.0f64; 4];
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

pub fn parse_bin(bytes: &[u8]) -> Option<Vec<SubstormOnset>> {
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
        let mut vals = [None; 4];
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
        out.push(SubstormOnset {
            t_unix,
            mlt: vals[0],
            mlat: vals[1],
            glon: vals[2],
            glat: vals[3],
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
            (COMP_MLT, r.mlt),
            (COMP_MLAT, r.mlat),
            (COMP_GLON, r.glon),
            (COMP_GLAT, r.glat),
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

    const CSV: &str = "Date_UTC,MLT,MLAT,GLON,GLAT\n\
2015-01-01 07:53:00,20.98,71.50,216.35,70.14\n\
2015-01-01 22:19:00,0.40,68.19,25.79,71.09\n";

    #[test]
    fn csv_rows_carry_the_onset_stamp_and_the_four_angles() {
        let records = parse_csv(CSV);
        assert_eq!(records.len(), 2);
        assert_eq!(records[0].mlt, Some(20.98));
        assert_eq!(records[0].mlat, Some(71.50));
        assert_eq!(records[0].glon, Some(216.35));
        assert_eq!(records[0].glat, Some(70.14));
    }

    #[test]
    fn a_short_row_is_skipped_and_an_empty_cell_stays_absent() {
        let text = "2015-01-01 07:53:00,20.98,71.50\n\
2015-01-01 08:00:00,,68.19,25.79,71.09\n";
        let records = parse_csv(text);
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].mlt, None);
        assert_eq!(records[0].mlat, Some(68.19));
    }

    #[test]
    fn the_header_row_is_no_onset() {
        assert!(parse_csv("Date_UTC,MLT,MLAT,GLON,GLAT\n").is_empty());
    }

    #[test]
    fn roundtrip_keeps_present_and_absent_columns() {
        let records = parse_csv(CSV);
        let bytes = write_bin(&records);
        assert_eq!(bytes.len(), HEADER_BYTES + 2 * RECORD_BYTES);
        assert_eq!(parse_bin(&bytes), Some(records));
    }

    #[test]
    fn declared_fields_carry_the_read_site_names_and_units() {
        let fields = declared_fields(86400.0);
        assert_eq!(fields.len(), COLUMNS.len());
        assert_eq!(fields[0].name, "substorm_mlt_h");
        assert_eq!(fields[0].unit, "h");
        assert_eq!(fields[1].unit, "deg");
        assert_eq!(component_name(COMP_GLAT), Some("substorm_glat_deg"));
        assert_eq!(component_name(9), None);
    }

    #[test]
    fn a_truncated_or_foreign_asset_is_void() {
        assert!(parse_bin(b"XXXX").is_none());
        let bytes = write_bin(&parse_csv(CSV));
        assert!(parse_bin(&bytes[..bytes.len() - 1]).is_none());
    }
}
