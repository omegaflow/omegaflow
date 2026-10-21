use crate::archivar::{Aperture, FieldConfig, force_id_of, kernel_id_for_force};

pub const MAGIC: [u8; 4] = *b"4LAC";
pub const VERSION: u32 = 1;
pub const HEADER_BYTES: usize = 16;

const CATALOG_EPOCH_ISO: &str = "2008-08-04T15:43:36";

const COLUMNS: &[(u32, &str, &str)] = &[
    (0, "ra", "deg"),
    (1, "dec", "deg"),
    (2, "glon", "deg"),
    (3, "glat", "deg"),
    (4, "signif_avg", "1"),
    (5, "flux1000", "ph/s/cm2"),
    (6, "unc_flux1000", "ph/s/cm2"),
    (7, "energy_flux100", "erg/cm2/s"),
    (8, "unc_energy_flux100", "erg/cm2/s"),
    (9, "pl_index", "1"),
    (10, "unc_pl_index", "1"),
    (11, "pivot_energy", "MeV"),
    (12, "lp_index", "1"),
    (13, "lp_beta", "1"),
    (14, "variability_index", "1"),
    (15, "redshift", "1"),
    (16, "nu_syn", "Hz"),
    (17, "nufnu_syn", "erg/cm2/s"),
    (18, "highest_energy", "GeV"),
];

#[derive(Clone, PartialEq, Debug)]
pub struct Table {
    pub names: Vec<String>,
    pub rows: Vec<Vec<Option<f64>>>,
}

pub fn component_name(comp: u32) -> Option<&'static str> {
    COLUMNS.iter().find(|c| c.0 == comp).map(|c| c.1)
}

fn component_of(name: &str) -> Option<u32> {
    COLUMNS.iter().find(|c| c.1 == name).map(|c| c.0)
}

pub fn declared_fields(tau: f64) -> Vec<FieldConfig> {
    let Some(channel_ref) = force_id_of("em") else {
        return Vec::new();
    };
    let Some(kernel) = kernel_id_for_force(channel_ref) else {
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
        });
    }
    out
}

pub fn write_bin(table: &Table) -> Vec<u8> {
    let n_cols = table.names.len();
    let words = n_cols.div_ceil(64);
    let mut out = Vec::new();
    out.extend_from_slice(&MAGIC);
    out.extend_from_slice(&VERSION.to_le_bytes());
    out.extend_from_slice(&(n_cols as u32).to_le_bytes());
    out.extend_from_slice(&(table.rows.len() as u32).to_le_bytes());
    for name in &table.names {
        let b = name.as_bytes();
        out.extend_from_slice(&(b.len() as u16).to_le_bytes());
        out.extend_from_slice(b);
    }
    for row in &table.rows {
        let mut mask = vec![0u64; words];
        for (j, cell) in row.iter().enumerate() {
            if cell.is_some() {
                mask[j / 64] |= 1u64 << (j % 64);
            }
        }
        for w in &mask {
            out.extend_from_slice(&w.to_le_bytes());
        }
        for v in row.iter().flatten() {
            out.extend_from_slice(&v.to_le_bytes());
        }
    }
    out
}

pub fn parse_bin(bytes: &[u8]) -> Option<Table> {
    if bytes.len() < HEADER_BYTES || bytes[0..4] != MAGIC {
        return None;
    }
    let version = u32::from_le_bytes(bytes[4..8].try_into().ok()?);
    if version != VERSION {
        return None;
    }
    let n_cols = u32::from_le_bytes(bytes[8..12].try_into().ok()?) as usize;
    let n_rows = u32::from_le_bytes(bytes[12..16].try_into().ok()?) as usize;
    if n_cols == 0 || n_cols > bytes.len() / 2 {
        return None;
    }
    let mut pos = HEADER_BYTES;
    let mut names = Vec::with_capacity(n_cols);
    for _ in 0..n_cols {
        let len = u16::from_le_bytes(bytes.get(pos..pos + 2)?.try_into().ok()?) as usize;
        pos += 2;
        let raw = bytes.get(pos..pos + len)?;
        names.push(String::from_utf8(raw.to_vec()).ok()?);
        pos += len;
    }
    let words = n_cols.div_ceil(64);
    let min_row = words.checked_mul(8)?;
    if bytes.len().checked_sub(pos)? < n_rows.checked_mul(min_row)? {
        return None;
    }
    let mut rows = Vec::with_capacity(n_rows);
    for _ in 0..n_rows {
        let mut mask = vec![0u64; words];
        for w in mask.iter_mut() {
            *w = u64::from_le_bytes(bytes.get(pos..pos + 8)?.try_into().ok()?);
            pos += 8;
        }
        let mut row = vec![None; n_cols];
        for (j, slot) in row.iter_mut().enumerate() {
            if mask[j / 64] & (1u64 << (j % 64)) != 0 {
                let v = f64::from_le_bytes(bytes.get(pos..pos + 8)?.try_into().ok()?);
                pos += 8;
                if !v.is_finite() {
                    return None;
                }
                *slot = Some(v);
            }
        }
        rows.push(row);
    }
    if pos != bytes.len() {
        return None;
    }
    Some(Table { names, rows })
}

pub fn parse_series(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    let table = parse_bin(bytes)?;
    let lsk = crate::archivar::embedded_lsk()?;
    let tdb = crate::archivar::parse_iso_tdb(CATALOG_EPOCH_ISO, &lsk)?;
    let comps: Vec<Option<u32>> = table.names.iter().map(|n| component_of(n)).collect();
    let mut out = Vec::new();
    for row in &table.rows {
        for (j, comp) in comps.iter().enumerate() {
            let Some(comp) = comp else {
                continue;
            };
            let Some(v) = row.get(j).copied().flatten() else {
                continue;
            };
            out.push((tdb, v, *comp));
        }
    }
    if out.is_empty() { None } else { Some(out) }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn table() -> Table {
        Table {
            names: vec![
                "ra".to_string(),
                "dec".to_string(),
                "flux1000".to_string(),
                "pl_index".to_string(),
            ],
            rows: vec![
                vec![Some(10.0), Some(-5.0), Some(2.5e-8), Some(2.1)],
                vec![Some(20.0), None, Some(3.5e-8), Some(1.9)],
            ],
        }
    }

    #[test]
    fn roundtrip_preserves_absence() {
        let t = table();
        assert_eq!(parse_bin(&write_bin(&t)), Some(t));
    }

    #[test]
    fn parse_bin_rejects_foreign_magic_and_truncation() {
        assert!(parse_bin(b"").is_none());
        assert!(parse_bin(b"XXXX").is_none());
        let good = write_bin(&table());
        assert!(parse_bin(&good[..good.len() - 1]).is_none());
    }

    #[test]
    fn parse_bin_rejects_bad_version_and_leftover() {
        let mut bad_version = write_bin(&table());
        bad_version[4..8].copy_from_slice(&2u32.to_le_bytes());
        assert!(parse_bin(&bad_version).is_none());

        let mut leftover = write_bin(&table());
        leftover.push(0u8);
        assert!(parse_bin(&leftover).is_none());
    }

    #[test]
    fn parse_bin_refuses_non_finite_values() {
        let nan = Table {
            names: vec!["ra".to_string()],
            rows: vec![vec![Some(f64::NAN)]],
        };
        assert!(parse_bin(&write_bin(&nan)).is_none());
    }

    #[test]
    fn series_folds_each_present_column_onto_the_catalog_epoch() {
        let bytes = write_bin(&table());
        let series = parse_series(&bytes).expect("the encoded asset reads back");
        assert_eq!(series.len(), 7);
        let lsk = crate::archivar::embedded_lsk().unwrap();
        let tdb = crate::archivar::parse_iso_tdb(CATALOG_EPOCH_ISO, &lsk).unwrap();
        assert_eq!(series[0], (tdb, 10.0, component_of("ra").unwrap()));
        assert_eq!(series[2], (tdb, 2.5e-8, component_of("flux1000").unwrap()));
        assert_eq!(series[3], (tdb, 2.1, component_of("pl_index").unwrap()));
        assert_eq!(series[4], (tdb, 20.0, component_of("ra").unwrap()));
    }

    #[test]
    fn an_unknown_column_stays_absent_and_an_empty_frame_is_void() {
        let foreign = Table {
            names: vec!["other".to_string()],
            rows: vec![vec![Some(1.0)]],
        };
        assert!(parse_series(&write_bin(&foreign)).is_none());
        let held = Table {
            names: vec!["ra".to_string()],
            rows: vec![vec![None]],
        };
        assert!(parse_series(&write_bin(&held)).is_none());
    }

    #[test]
    fn declared_fields_carry_the_read_site_names_and_units() {
        let fields = declared_fields(33554432.0);
        assert_eq!(fields.len(), COLUMNS.len());
        assert_eq!(fields[0].name, "ra");
        assert_eq!(fields[0].unit, "deg");
        assert_eq!(fields[5].name, "flux1000");
        assert_eq!(fields[5].unit, "ph/s/cm2");
        assert_eq!(fields[11].name, "pivot_energy");
        assert_eq!(fields[11].unit, "MeV");
        assert_eq!(component_name(18), Some("highest_energy"));
        assert_eq!(component_name(19), None);
    }
}
