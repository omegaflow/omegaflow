use crate::archivar::hapi_csv::parse_iso_seconds;

pub const MAGIC: [u8; 4] = *b"GIFC";
pub const HEADER_BYTES: usize = 8;
pub const RECORD_BYTES: usize = 8 + 1 + 8 + 8;

#[derive(Clone, Debug, PartialEq)]
pub struct GiroRecord {
    pub t_unix: f64,
    pub fof2_mhz: Option<f64>,
    pub cs: Option<i64>,
}

pub fn parse_text(text: &str) -> Vec<GiroRecord> {
    let mut out = Vec::new();
    for line in text.lines() {
        let line = line.trim();
        if line.starts_with("# STATUS: ERROR") {
            return Vec::new();
        }
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let cells: Vec<&str> = line.split_whitespace().collect();
        if cells.len() < 3 {
            continue;
        }
        let Some(t_unix) = parse_iso_seconds(cells[0]) else {
            continue;
        };
        let cs = cells[1].parse::<i64>().ok();
        let fof2_mhz = cells[2]
            .parse::<f64>()
            .ok()
            .filter(|v| v.is_finite() && *v > 0.0);
        if fof2_mhz.is_some() || cs.is_some() {
            out.push(GiroRecord {
                t_unix,
                fof2_mhz,
                cs,
            });
        }
    }
    out
}

pub fn write_bin(records: &[GiroRecord]) -> Vec<u8> {
    let mut buf = Vec::with_capacity(HEADER_BYTES + records.len() * RECORD_BYTES);
    buf.extend_from_slice(&MAGIC);
    buf.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for r in records {
        let (fof2, has_fof2) = match r.fof2_mhz {
            Some(v) => (v, true),
            None => (0.0, false),
        };
        let (cs, has_cs) = match r.cs {
            Some(v) => (v, true),
            None => (0, false),
        };
        let mut present = 0u8;
        if has_fof2 {
            present |= 1;
        }
        if has_cs {
            present |= 2;
        }
        buf.extend_from_slice(&r.t_unix.to_le_bytes());
        buf.push(present);
        buf.extend_from_slice(&fof2.to_le_bytes());
        buf.extend_from_slice(&cs.to_le_bytes());
    }
    buf
}

pub fn parse_bin(bytes: &[u8]) -> Option<Vec<GiroRecord>> {
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
        if !t_unix.is_finite() {
            return None;
        }
        let present = *bytes.get(off)?;
        off += 1;
        let fof2_raw = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        off += 8;
        let cs_raw = i64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        off += 8;
        let fof2_mhz = if present & 1 != 0 {
            if !fof2_raw.is_finite() {
                return None;
            }
            Some(fof2_raw)
        } else {
            None
        };
        let cs = if present & 2 != 0 { Some(cs_raw) } else { None };
        out.push(GiroRecord {
            t_unix,
            fof2_mhz,
            cs,
        });
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    const FASTCHAR: &str = "# Time CS foF2 QD\n\
2024-01-01T00:00:00  100  7.2  //\n\
2024-01-01T00:15:00  100  6.8  //\n\
2024-01-01T00:30:00  999  0.0  //\n";
    const ERROR: &str = "# STATUS: ERROR\n# no best value for the requested window\n";

    #[test]
    fn fastchar_rows_carry_time_and_fof2() {
        let records = parse_text(FASTCHAR);
        assert_eq!(records.len(), 3);
        assert_eq!(
            records[0].t_unix,
            parse_iso_seconds("2024-01-01T00:00:00").expect("stamp parses")
        );
        assert_eq!(records[0].fof2_mhz, Some(7.2));
        assert_eq!(records[1].fof2_mhz, Some(6.8));
        assert_eq!(records[0].cs, Some(100));
        assert_eq!(records[2].cs, Some(999));
    }

    #[test]
    fn a_nonphysical_fof2_is_absent_not_zero() {
        let records = parse_text(FASTCHAR);
        assert_eq!(records[2].fof2_mhz, None);
    }

    #[test]
    fn an_error_body_yields_no_records() {
        assert!(parse_text(ERROR).is_empty());
    }

    #[test]
    fn roundtrip_keeps_present_and_absent_values() {
        let records = parse_text(FASTCHAR);
        let bytes = write_bin(&records);
        assert_eq!(bytes.len(), HEADER_BYTES + 3 * RECORD_BYTES);
        assert_eq!(parse_bin(&bytes), Some(records));
    }

    #[test]
    fn a_truncated_or_foreign_asset_is_void() {
        assert!(parse_bin(b"XXXX").is_none());
        let bytes = write_bin(&parse_text(FASTCHAR));
        assert!(parse_bin(&bytes[..bytes.len() - 1]).is_none());
    }
}
