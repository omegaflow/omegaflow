use crate::archivar::hapi_csv::parse_iso_seconds;
use crate::archivar::{Aperture, FieldConfig, force_id_of, kernel_id_for_force};

pub const MAGIC: [u8; 4] = *b"WDCA";
pub const HEADER_BYTES: usize = 8;
pub const RECORD_BYTES: usize = 8 + 1 + 6 * 8;

pub const COMP_AL: u32 = 0;
pub const COMP_AU: u32 = 1;
pub const COMP_AE: u32 = 2;
pub const COMP_SYM_D: u32 = 3;
pub const COMP_SYM_H: u32 = 4;
pub const COMP_DST: u32 = 5;

const COLUMNS: &[(u32, &str, &str)] = &[
    (COMP_AL, "wdc_ae_al_nt", "nT"),
    (COMP_AU, "wdc_ae_au_nt", "nT"),
    (COMP_AE, "wdc_ae_ae_nt", "nT"),
    (COMP_SYM_D, "wdc_sym_d_nt", "nT"),
    (COMP_SYM_H, "wdc_sym_h_nt", "nT"),
    (COMP_DST, "wdc_dst_nt", "nT"),
];

#[derive(Clone, Debug, PartialEq)]
pub struct WdcRecord {
    pub t_unix: f64,
    pub al_nt: Option<f64>,
    pub au_nt: Option<f64>,
    pub ae_nt: Option<f64>,
    pub sym_d_nt: Option<f64>,
    pub sym_h_nt: Option<f64>,
    pub dst_nt: Option<f64>,
}

impl WdcRecord {
    fn values(&self) -> [Option<f64>; 6] {
        [
            self.al_nt,
            self.au_nt,
            self.ae_nt,
            self.sym_d_nt,
            self.sym_h_nt,
            self.dst_nt,
        ]
    }
}

fn parse_num(cell: Option<&&str>) -> Option<f64> {
    let t = cell?.trim();
    if t.is_empty() {
        return None;
    }
    t.parse::<f64>().ok().filter(|v| v.is_finite())
}

pub fn parse_text(text: &str, dataset: &str) -> Vec<WdcRecord> {
    let mut out = Vec::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let cells: Vec<&str> = line.split(',').map(str::trim).collect();
        if cells.len() < 2 {
            continue;
        }
        let Some(t_unix) = parse_iso_seconds(cells[0]) else {
            continue;
        };
        let mut rec = WdcRecord {
            t_unix,
            al_nt: None,
            au_nt: None,
            ae_nt: None,
            sym_d_nt: None,
            sym_h_nt: None,
            dst_nt: None,
        };
        match dataset {
            "min_ae" => {
                rec.ae_nt = parse_num(cells.get(1));
                rec.al_nt = parse_num(cells.get(2));
                rec.au_nt = parse_num(cells.get(3));
            }
            "min_asysym" => {
                rec.sym_d_nt = parse_num(cells.get(3));
                rec.sym_h_nt = parse_num(cells.get(4));
            }
            "hour_dst" => {
                rec.dst_nt = parse_num(cells.get(1));
            }
            _ => return Vec::new(),
        }
        if rec.values().iter().any(|v| v.is_some()) {
            out.push(rec);
        }
    }
    out
}

pub fn write_bin(records: &[WdcRecord]) -> Vec<u8> {
    let mut buf = Vec::with_capacity(HEADER_BYTES + records.len() * RECORD_BYTES);
    buf.extend_from_slice(&MAGIC);
    buf.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for r in records {
        buf.extend_from_slice(&r.t_unix.to_le_bytes());
        let mut present = 0u8;
        let mut vals = [0.0f64; 6];
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

pub fn parse_bin(bytes: &[u8]) -> Option<Vec<WdcRecord>> {
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
        let mut vals = [None; 6];
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
        out.push(WdcRecord {
            t_unix,
            al_nt: vals[0],
            au_nt: vals[1],
            ae_nt: vals[2],
            sym_d_nt: vals[3],
            sym_h_nt: vals[4],
            dst_nt: vals[5],
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
            (COMP_AL, r.al_nt),
            (COMP_AU, r.au_nt),
            (COMP_AE, r.ae_nt),
            (COMP_SYM_D, r.sym_d_nt),
            (COMP_SYM_H, r.sym_h_nt),
            (COMP_DST, r.dst_nt),
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

    const AE_CSV: &str = "2003-11-01T00:00:00Z,617,-471,146,-163,10\n\
2003-11-01T00:01:00Z,604,-419,185,-117,10\n";
    const ASY_CSV: &str = "1981-01-01T00:00:00Z,9,35,-4,-31,10\n";
    const DST_CSV: &str = "1957-07-01T00:29:30Z,-210,20\n1957-07-01T01:29:30Z,-206,20\n";

    #[test]
    fn ae_rows_carry_ae_al_au_and_ignore_ao() {
        let records = parse_text(AE_CSV, "min_ae");
        assert_eq!(records.len(), 2);
        assert_eq!(records[0].ae_nt, Some(617.0));
        assert_eq!(records[0].al_nt, Some(-471.0));
        assert_eq!(records[0].au_nt, Some(146.0));
        assert_eq!(records[0].sym_h_nt, None);
        assert_eq!(records[0].dst_nt, None);
    }

    #[test]
    fn asysym_rows_carry_sym_d_and_sym_h() {
        let records = parse_text(ASY_CSV, "min_asysym");
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].sym_d_nt, Some(-4.0));
        assert_eq!(records[0].sym_h_nt, Some(-31.0));
        assert_eq!(records[0].ae_nt, None);
    }

    #[test]
    fn dst_rows_carry_the_hourly_value_at_the_stamp() {
        let records = parse_text(DST_CSV, "hour_dst");
        assert_eq!(records.len(), 2);
        assert_eq!(records[0].dst_nt, Some(-210.0));
        assert_eq!(records[1].dst_nt, Some(-206.0));
    }

    #[test]
    fn an_absent_cell_is_never_a_fabricated_zero() {
        let text = "2003-11-01T00:00:00Z,617,,146,-163,10\n";
        let records = parse_text(text, "min_ae");
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].al_nt, None);
        assert_eq!(records[0].au_nt, Some(146.0));
    }

    #[test]
    fn a_row_with_no_measured_value_is_dropped() {
        let text = "2003-11-01T00:00:00Z,,,,-163,10\n";
        assert!(parse_text(text, "min_ae").is_empty());
    }

    #[test]
    fn roundtrip_keeps_present_and_absent_columns() {
        let records = parse_text(AE_CSV, "min_ae");
        let bytes = write_bin(&records);
        assert_eq!(bytes.len(), HEADER_BYTES + 2 * RECORD_BYTES);
        assert_eq!(parse_bin(&bytes), Some(records));
    }

    #[test]
    fn declared_fields_carry_the_read_site_names_and_units() {
        let fields = declared_fields(86400.0);
        assert_eq!(fields.len(), COLUMNS.len());
        assert_eq!(fields[0].name, "wdc_ae_al_nt");
        assert_eq!(fields[0].unit, "nT");
        assert_eq!(fields[5].name, "wdc_dst_nt");
        assert_eq!(component_name(COMP_SYM_H), Some("wdc_sym_h_nt"));
        assert_eq!(component_name(9), None);
    }

    #[test]
    fn a_truncated_or_foreign_asset_is_void() {
        assert!(parse_bin(b"XXXX").is_none());
        let bytes = write_bin(&parse_text(AE_CSV, "min_ae"));
        assert!(parse_bin(&bytes[..bytes.len() - 1]).is_none());
    }

    #[test]
    fn an_unknown_dataset_yields_no_records() {
        assert!(parse_text(AE_CSV, "min_unknown").is_empty());
    }
}
