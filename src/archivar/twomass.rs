pub const MAGIC: [u8; 4] = *b"2MPS";
pub const RECORD_BYTES: usize = 64;
pub const PSC_FIELDS: usize = 60;
pub const PSC_FILE_COUNT: usize = 92;

pub struct PscRow {
    pub ra_deg: f64,
    pub dec_deg: f64,
    pub jmag: Option<f64>,
    pub e_jmag: Option<f64>,
    pub hmag: Option<f64>,
    pub e_hmag: Option<f64>,
    pub kmag: Option<f64>,
    pub e_kmag: Option<f64>,
}

fn coord_field(v: Option<&str>) -> Option<f64> {
    let x: f64 = v?.trim().parse().ok()?;
    if x.is_finite() {
        Some(x)
    } else {
        None
    }
}

fn mag_field(v: Option<&str>) -> Option<f64> {
    let x: f64 = v?.trim().parse().ok()?;
    if x.is_finite() && x > 0.0 {
        Some(x)
    } else {
        None
    }
}

fn err_field(v: Option<&str>) -> Option<f64> {
    let x = mag_field(v)?;
    if x <= 8.0 {
        Some(x)
    } else {
        None
    }
}

fn band_detected(rd_flg: &[u8], band: usize) -> bool {
    matches!(
        rd_flg.get(band),
        Some(b'1') | Some(b'2') | Some(b'3') | Some(b'4')
    )
}

pub fn parse_psc_row(line: &str) -> Option<PscRow> {
    let fields: Vec<&str> = line.trim_end_matches('\r').split('|').collect();
    if fields.len() != PSC_FIELDS {
        return None;
    }
    let ra = coord_field(fields.get(0).copied())?;
    let dec = coord_field(fields.get(1).copied())?;
    if !(0.0..360.0).contains(&ra) || !(-90.0..=90.0).contains(&dec) {
        return None;
    }
    let rd = fields.get(19).copied().unwrap_or("").as_bytes();
    let jmag = mag_field(fields.get(6).copied());
    let e_j = if band_detected(rd, 0) {
        err_field(fields.get(8).copied())
    } else {
        None
    };
    let hmag = if band_detected(rd, 1) {
        mag_field(fields.get(10).copied())
    } else {
        None
    };
    let e_h = if band_detected(rd, 1) {
        err_field(fields.get(12).copied())
    } else {
        None
    };
    let kmag = if band_detected(rd, 2) {
        mag_field(fields.get(14).copied())
    } else {
        None
    };
    let e_k = if band_detected(rd, 2) {
        err_field(fields.get(16).copied())
    } else {
        None
    };
    Some(PscRow {
        ra_deg: ra,
        dec_deg: dec,
        jmag,
        e_jmag: e_j,
        hmag,
        e_hmag: e_h,
        kmag,
        e_kmag: e_k,
    })
}

pub enum Selection {
    Jmag { limit: f64 },
    Decimation { factor: u64 },
    Declination { lo: f64, hi: f64 },
}

impl Selection {
    pub fn keep(&self, row: &PscRow, seen: &mut u64) -> bool {
        let Some(jmag) = row.jmag else {
            return false;
        };
        match self {
            Selection::Jmag { limit } => jmag < *limit,
            Selection::Decimation { factor } => {
                *seen += 1;
                *factor > 0 && *seen % *factor == 0
            }
            Selection::Declination { lo, hi } => row.dec_deg >= *lo && row.dec_deg < *hi,
        }
    }
}

pub fn row_record(row: &PscRow) -> [f64; 8] {
    [
        row.ra_deg,
        row.dec_deg,
        row.jmag.unwrap_or(0.0),
        row.e_jmag.unwrap_or(0.0),
        row.hmag.unwrap_or(0.0),
        row.e_hmag.unwrap_or(0.0),
        row.kmag.unwrap_or(0.0),
        row.e_kmag.unwrap_or(0.0),
    ]
}

pub fn write_bin(records: &[[f64; 8]]) -> Vec<u8> {
    let mut out = Vec::with_capacity(8 + records.len() * RECORD_BYTES);
    out.extend_from_slice(&MAGIC);
    out.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for r in records {
        for v in r {
            out.extend_from_slice(&v.to_le_bytes());
        }
    }
    out
}

pub fn read_bin(data: &[u8]) -> Option<Vec<[f64; 8]>> {
    if data.len() < 8 || &data[0..4] != &MAGIC {
        return None;
    }
    let count = u32::from_le_bytes(data[4..8].try_into().ok()?) as usize;
    if data.len() != 8 + count * RECORD_BYTES {
        return None;
    }
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        let base = 8 + i * RECORD_BYTES;
        let mut r = [0.0f64; 8];
        for k in 0..8 {
            let mut buf = [0u8; 8];
            buf.copy_from_slice(&data[base + k * 8..base + k * 8 + 8]);
            r[k] = f64::from_le_bytes(buf);
        }
        out.push(r);
    }
    Some(out)
}

pub fn file_list() -> Vec<String> {
    let mut names = Vec::with_capacity(PSC_FILE_COUNT);
    for p in ['a', 'b'] {
        for c2 in 'a'..='z' {
            for c3 in 'a'..='z' {
                if p == 'a' && ((c2 == 'c' && c3 > 'e') || c2 > 'c') {
                    continue;
                }
                if p == 'b' && ((c2 == 'b' && c3 > 'i') || c2 > 'b') {
                    continue;
                }
                names.push(format!("psc_{}{}{}.gz", p, c2, c3));
            }
        }
    }
    names
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE: &str = "phi/pipeline/catalog/twomass/test_psc";

    fn fixture_lines() -> Option<String> {
        std::fs::read_to_string(FIXTURE).ok()
    }

    #[test]
    fn test_psc_fixture_parses() {
        let Some(text) = fixture_lines() else {
            println!(
                "skip: fixture absent — fetch: curl https://irsa.ipac.caltech.edu/2MASS/download/allsky/practice/test_psc -o {} (gitignored)",
                FIXTURE
            );
            return;
        };
        let rows: Vec<PscRow> = text.lines().filter_map(parse_psc_row).collect();
        assert_eq!(rows.len(), 10);
        let first = &rows[0];
        assert!((first.ra_deg - 1.119851).abs() < 1e-6);
        assert!((first.dec_deg - (-89.91861)).abs() < 1e-5);
        let j = first.jmag.expect("jmag detected");
        assert!((j - 12.467).abs() < 1e-3);
        let e_j = first.e_jmag.expect("e_jmag detected");
        assert!((e_j - 0.021).abs() < 1e-3);
        let h = first.hmag.expect("hmag detected");
        assert!((h - 12.131).abs() < 1e-3);
        let k = first.kmag.expect("kmag detected");
        assert!((k - 11.963).abs() < 1e-3);
        for r in &rows {
            assert!(r.ra_deg > 0.0 && r.ra_deg < 360.0);
            assert!(r.dec_deg >= -90.0 && r.dec_deg <= 90.0);
            assert!(r.jmag.is_some());
        }
    }

    #[test]
    fn test_selection_gates() {
        let row = PscRow {
            ra_deg: 10.0,
            dec_deg: -30.0,
            jmag: Some(11.4),
            e_jmag: Some(0.03),
            hmag: Some(10.8),
            e_hmag: Some(0.04),
            kmag: Some(10.5),
            e_kmag: Some(0.04),
        };
        let mut seen = 0u64;
        let jmag = Selection::Jmag { limit: 11.5 };
        assert!(jmag.keep(&row, &mut seen));
        assert!(!Selection::Jmag { limit: 11.0 }.keep(&row, &mut seen));
        let decim = Selection::Decimation { factor: 3 };
        let mut s1 = 0u64;
        assert!(!decim.keep(&row, &mut s1));
        assert!(!decim.keep(&row, &mut s1));
        assert!(decim.keep(&row, &mut s1));
        assert_eq!(s1, 3);
        let band = Selection::Declination { lo: -40.0, hi: 0.0 };
        assert!(band.keep(&row, &mut seen));
        assert!(!Selection::Declination { lo: 0.0, hi: 90.0 }.keep(&row, &mut seen));
        let dark = PscRow {
            ra_deg: 10.0,
            dec_deg: -30.0,
            jmag: None,
            e_jmag: None,
            hmag: None,
            e_hmag: None,
            kmag: None,
            e_kmag: None,
        };
        assert!(!jmag.keep(&dark, &mut seen));
        assert!(!band.keep(&dark, &mut seen));
    }

    #[test]
    fn test_bin_roundtrip() {
        let records = vec![
            [1.0, -2.0, 11.4, 0.03, 0.0, 0.0, 10.5, 0.04],
            [300.0, 89.0, 9.9, 0.02, 9.1, 0.03, 8.9, 0.03],
        ];
        let bin = write_bin(&records);
        assert_eq!(bin.len(), 8 + 2 * RECORD_BYTES);
        let parsed = read_bin(&bin).expect("roundtrip parses");
        assert_eq!(parsed.len(), 2);
        for (a, b) in parsed.iter().zip(records.iter()) {
            for k in 0..8 {
                assert_eq!(a[k], b[k]);
            }
        }
        assert!(read_bin(&bin[..7]).is_none());
        assert!(read_bin(&bin[..9]).is_none());
        let mut wrong = bin.clone();
        wrong[0] = b'X';
        assert!(read_bin(&wrong).is_none());
    }

    #[test]
    fn test_file_list() {
        let names = file_list();
        assert_eq!(names.len(), PSC_FILE_COUNT);
        assert_eq!(names.first().map(String::as_str), Some("psc_aaa.gz"));
        assert_eq!(names.last().map(String::as_str), Some("psc_bbi.gz"));
        assert!(names.contains(&"psc_ace.gz".to_string()));
        assert!(names.contains(&"psc_aba.gz".to_string()));
        assert!(names.contains(&"psc_baa.gz".to_string()));
        let unique: std::collections::HashSet<&String> = names.iter().collect();
        assert_eq!(unique.len(), names.len());
    }

    #[test]
    fn test_psc_fixture_selection_jmag() {
        let Some(text) = fixture_lines() else {
            println!(
                "skip: fixture absent ({} — gitignored, see test_psc_fixture_parses)",
                FIXTURE
            );
            return;
        };
        let rows: Vec<PscRow> = text.lines().filter_map(parse_psc_row).collect();
        let limit = 13.0;
        let mut seen = 0u64;
        let sel = Selection::Jmag { limit };
        let picked: Vec<&PscRow> = rows.iter().filter(|r| sel.keep(r, &mut seen)).collect();
        let reference: Vec<&PscRow> = rows
            .iter()
            .filter(|r| r.jmag.map(|j| j < limit).unwrap_or(false))
            .collect();
        assert_eq!(picked.len(), reference.len());
        assert!(picked.len() >= 1);
        assert!(picked.len() < rows.len());
    }
}
