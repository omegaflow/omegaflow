use omegaflow::archivar::bc_mpo_mag::{FIELDS_PER_RECORD, parse_bin, write_bin};
use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::upload_release;
use omegaflow::inflate::unzip;
use omegaflow::lsk::days_from_civil;

const NETLOC: &str = "psa.esa.int";
const DAY_S: f64 = 86400.0;

fn arg_value(args: &[String], key: &str) -> Option<String> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn parse_utc(s: &str) -> Option<f64> {
    let b = s.as_bytes();
    if b.len() < 19 || b[4] != b'-' || b[7] != b'-' || b[10] != b'T' {
        return None;
    }
    let year = s.get(0..4)?.parse::<i64>().ok()?;
    let month = s.get(5..7)?.parse::<i64>().ok()?;
    let day = s.get(8..10)?.parse::<i64>().ok()?;
    let hour = s.get(11..13)?.parse::<i64>().ok()?;
    let minute = s.get(14..16)?.parse::<i64>().ok()?;
    let second = s.get(17..19)?.parse::<i64>().ok()?;
    let days = days_from_civil(year, month, day)?;
    let frac = if b.get(19) == Some(&b'.') {
        let frac_str = s.get(20..)?.trim_end_matches('Z');
        if frac_str.is_empty() {
            0.0
        } else {
            let digits = frac_str.parse::<f64>().ok()?;
            digits / 10f64.powi(frac_str.len() as i32)
        }
    } else {
        0.0
    };
    Some(days as f64 * DAY_S + hour as f64 * 3600.0 + minute as f64 * 60.0 + second as f64 + frac)
}

fn parse_tab(bytes: &[u8]) -> Vec<[f64; FIELDS_PER_RECORD]> {
    let text = std::str::from_utf8(bytes).unwrap_or("");
    let mut out = Vec::new();
    for line in text.lines() {
        let t = line.trim();
        if t.is_empty() {
            continue;
        }
        let cols: Vec<&str> = t.split(',').map(|f| f.trim()).collect();
        if cols.len() < 8 {
            continue;
        }
        let Some(t_unix) = parse_utc(cols[0]) else {
            continue;
        };
        let (Ok(pos_x), Ok(pos_y), Ok(pos_z), Ok(bx), Ok(by), Ok(bz)) = (
            cols[2].parse::<f64>(),
            cols[3].parse::<f64>(),
            cols[4].parse::<f64>(),
            cols[5].parse::<f64>(),
            cols[6].parse::<f64>(),
            cols[7].parse::<f64>(),
        ) else {
            continue;
        };
        if !t_unix.is_finite()
            || !pos_x.is_finite()
            || !pos_y.is_finite()
            || !pos_z.is_finite()
            || !bx.is_finite()
            || !by.is_finite()
            || !bz.is_finite()
        {
            continue;
        }
        out.push([t_unix, pos_x, pos_y, pos_z, bx, by, bz]);
    }
    out
}

fn is_zip(bytes: &[u8]) -> bool {
    bytes.len() >= 4 && &bytes[0..4] == b"PK\x03\x04"
}

fn ingest(bytes: &[u8]) -> Vec<[f64; FIELDS_PER_RECORD]> {
    if is_zip(bytes) {
        match unzip(bytes) {
            Some(inner) => parse_tab(&inner),
            None => Vec::new(),
        }
    } else {
        parse_tab(bytes)
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = match arg_value(&args, "--out") {
        Some(v) => v,
        None => "data/psa.esa.int/bc_mpo_mag.bin".to_string(),
    };
    let (bytes, source) = match arg_value(&args, "--input") {
        Some(path) => match std::fs::read(&path) {
            Ok(b) => (b, path),
            Err(e) => {
                eprintln!("read {path} returned void: {e}");
                std::process::exit(1);
            }
        },
        None => match arg_value(&args, "--url") {
            Some(url) => match fetch_raw_bytes(&url) {
                Some(b) => (b, url),
                None => {
                    eprintln!("{url}: fetch void");
                    std::process::exit(1);
                }
            },
            None => {
                eprintln!("--input <path.tab> or --url <url> required");
                std::process::exit(1);
            }
        },
    };
    let mut records = ingest(&bytes);
    if records.is_empty() {
        eprintln!("{source}: no MAG rows — the series stays unwritten (0 honored)");
        std::process::exit(1);
    }
    records.sort_by(|a, b| a[0].total_cmp(&b[0]));
    let bin = write_bin(&records);
    let Some(parsed) = parse_bin(&bin) else {
        eprintln!("{out}: packed read void — the series stays unverified (0 honored)");
        std::process::exit(1);
    };
    if parsed != records {
        eprintln!("{out}: roundtrip void — the series stays unverified (0 honored)");
        std::process::exit(1);
    }
    if let Some(parent) = std::path::Path::new(&out).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if std::fs::write(&out, &bin).is_err() {
        eprintln!("write {out} returned void");
        std::process::exit(1);
    }
    let first = records[0][0];
    let last = records[records.len() - 1][0];
    eprintln!(
        "{out}: {} MAG record(s) packed (unix {first:.0}..{last:.0}), {} B, sha256 {}, roundtrip holds",
        records.len(),
        bin.len(),
        sha256_hex(&bin)
    );
    if ci_mode && !upload_release(NETLOC, &out) {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_utc_reads_the_measured_timestamp() {
        let t = parse_utc("2018-10-24T15:21:21.000000Z").unwrap();
        let expected = 1540394481.0;
        assert!((t - expected).abs() < 1.0);
        assert!(parse_utc("2018-10-24T15:21:21.abcZ").is_none());
        assert!(parse_utc("no time here").is_none());
    }

    #[test]
    fn parse_tab_reads_the_measured_columns() {
        let tab = "2018-10-24T15:21:21.000000Z,1/0605114480:00000,  126305792.00,   75839900.00,     144517.43,     69.229,   -589.718,   1579.287,  -65.87,  -65.67,    4.02\n\
                   2018-10-24T15:21:22.000000Z,1/0605114481:00000,  126305774.40,   75839921.60,     144517.76,     69.310,   -589.687,   1579.239,  -65.87,  -65.67,    4.02\n";
        let rows = parse_tab(tab.as_bytes());
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0][4], 69.229);
        assert_eq!(rows[0][5], -589.718);
        assert_eq!(rows[0][6], 1579.287);
        assert_eq!(rows[1][0] - rows[0][0], 1.0);
    }

    #[test]
    fn parse_tab_skips_void_rows() {
        let tab = "2018-10-24T15:21:21.000000Z,1/0605114480:00000,  126305792.00,   75839900.00,     144517.43,     69.229,   -589.718,   1579.287,  -65.87,  -65.67,    4.02\n\
                   2018-10-24T15:21:22.000000Z,1/0605114481:00000,             BAD,   75839921.60,     144517.76,     69.310,   -589.687,   1579.239,  -65.87,  -65.67,    4.02\n";
        assert_eq!(parse_tab(tab.as_bytes()).len(), 1);
    }

    #[test]
    fn ingest_reads_a_raw_tab() {
        let tab = "2018-10-24T15:21:21.000000Z,1/0605114480:00000,  126305792.00,   75839900.00,     144517.43,     69.229,   -589.718,   1579.287,  -65.87,  -65.67,    4.02\n";
        let rows = ingest(tab.as_bytes());
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0][4], 69.229);
        assert_eq!(rows[0][5], -589.718);
        assert_eq!(rows[0][6], 1579.287);
    }

    #[test]
    fn ingest_reads_a_stored_zip() {
        let tab = b"2018-10-24T15:21:21.000000Z,1/0605114480:00000,  126305792.00,   75839900.00,     144517.43,     69.229,   -589.718,   1579.287,  -65.87,  -65.67,    4.02\n";
        let mut zip = Vec::new();
        zip.extend_from_slice(b"PK\x03\x04");
        zip.extend_from_slice(&20u16.to_le_bytes());
        zip.extend_from_slice(&0u16.to_le_bytes());
        zip.extend_from_slice(&0u16.to_le_bytes());
        zip.extend_from_slice(&0u16.to_le_bytes());
        zip.extend_from_slice(&0u16.to_le_bytes());
        zip.extend_from_slice(&0u32.to_le_bytes());
        zip.extend_from_slice(&(tab.len() as u32).to_le_bytes());
        zip.extend_from_slice(&(tab.len() as u32).to_le_bytes());
        zip.extend_from_slice(&0u16.to_le_bytes());
        zip.extend_from_slice(&0u16.to_le_bytes());
        zip.extend_from_slice(tab);
        let rows = ingest(&zip);
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0][4], 69.229);
        assert_eq!(rows[0][5], -589.718);
        assert_eq!(rows[0][6], 1579.287);
    }

    #[test]
    fn ingest_refuses_a_zip_without_a_tab_member() {
        assert_eq!(ingest(b"PK\x03\x04\x14\x00").len(), 0);
        assert_eq!(ingest(b"not a zip, not a tab").len(), 0);
    }
}
