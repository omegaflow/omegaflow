use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::upload_release;
use omegaflow::inflate::gunzip;
use omegaflow::lsk::days_from_civil;

const BASE: &str = "https://downloads.sws.bom.gov.au/wdc/iondata";
const MAGIC: [u8; 4] = *b"SWSI";
const HEADER_BYTES: usize = 8;
const RECORD_BYTES: usize = 8 + 1 + 8;

#[derive(Clone, Debug, PartialEq)]
pub struct SwsRecord {
    pub t_unix: f64,
    pub fof2_mhz: Option<f64>,
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

pub fn parse_text(text: &str, year: i64) -> Vec<SwsRecord> {
    let mut out = Vec::new();
    for line in text.lines() {
        if line.len() < 12 {
            continue;
        }
        let head = &line[..12];
        if !head.bytes().all(|b| b.is_ascii_digit()) {
            continue;
        }
        let month = head[8..10].parse::<i64>().ok();
        let day = head[10..12].parse::<i64>().ok();
        let (Some(month), Some(day)) = (month, day) else {
            continue;
        };
        let Some(days) = days_from_civil(year, month, day) else {
            continue;
        };
        for hour in 0..24usize {
            let start = 12 + hour * 5;
            let Some(token) = line.get(start..start + 5) else {
                break;
            };
            let b = token.as_bytes();
            let raw = token[1..4].trim();
            let Ok(value) = raw.parse::<f64>() else {
                continue;
            };
            let fof2_mhz = if value > 0.0 && value < 300.0 {
                Some(value / 10.0)
            } else {
                None
            };
            if fof2_mhz.is_none() {
                continue;
            }
            if b[4] == b'N' {
                continue;
            }
            out.push(SwsRecord {
                t_unix: (days * 86400 + hour as i64 * 3600) as f64,
                fof2_mhz,
            });
        }
    }
    out
}

pub fn write_bin(records: &[SwsRecord]) -> Vec<u8> {
    let mut buf = Vec::with_capacity(HEADER_BYTES + records.len() * RECORD_BYTES);
    buf.extend_from_slice(&MAGIC);
    buf.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for r in records {
        let (v, present) = match r.fof2_mhz {
            Some(v) => (v, 1u8),
            None => (0.0, 0u8),
        };
        buf.extend_from_slice(&r.t_unix.to_le_bytes());
        buf.push(present);
        buf.extend_from_slice(&v.to_le_bytes());
    }
    buf
}

pub fn parse_bin(bytes: &[u8]) -> Option<Vec<SwsRecord>> {
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
        let raw = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        off += 8;
        let fof2_mhz = if present & 1 != 0 {
            if !raw.is_finite() {
                return None;
            }
            Some(raw)
        } else {
            None
        };
        out.push(SwsRecord { t_unix, fof2_mhz });
    }
    Some(out)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let Some(station) = arg_value(&args, "--station") else {
        eprintln!("sws_ionosonde_compiler needs --station <SSSS> (e.g. 0318)");
        std::process::exit(2);
    };
    let Some(site) = arg_value(&args, "--site") else {
        eprintln!("sws_ionosonde_compiler needs --site <country/name> (e.g. de/juliusruh)");
        std::process::exit(2);
    };
    let Some(param) = arg_value(&args, "--param") else {
        eprintln!("sws_ionosonde_compiler needs --param <PP> (00 = foF2)");
        std::process::exit(2);
    };
    if param != "00" {
        eprintln!("--param {param}: only 00 (foF2) is parsed");
        std::process::exit(2);
    }
    let year_of =
        |name: &str| -> Option<i64> { arg_value(&args, name).and_then(|v| v.parse::<i64>().ok()) };
    let (Some(start_year), Some(stop_year)) = (year_of("--start-year"), year_of("--stop-year"))
    else {
        eprintln!("sws_ionosonde_compiler needs --start-year <YYYY> and --stop-year <YYYY>");
        std::process::exit(2);
    };
    if stop_year < start_year {
        eprintln!("--stop-year must not lie before --start-year");
        std::process::exit(2);
    }
    let Some(out) = arg_value(&args, "--out") else {
        eprintln!("sws_ionosonde_compiler needs --out <path.bin>");
        std::process::exit(2);
    };

    let mut records: Vec<SwsRecord> = Vec::new();
    let mut fetched = 0usize;
    for year in start_year..=stop_year {
        let yy = year.rem_euclid(100);
        let url = format!("{BASE}/{site}/{station}.{param}/{station}{param}.{yy:02}.gz");
        let Some(gz) = fetch_raw_bytes(&url) else {
            eprintln!("{url}: fetch void (year {year} skipped)");
            continue;
        };
        let Some(body) = gunzip(&gz) else {
            eprintln!("{url}: gzip stream stays unreadable (year {year} skipped)");
            continue;
        };
        let text = String::from_utf8_lossy(&body);
        let mut chunk = parse_text(&text, year);
        chunk.sort_by(|a, b| a.t_unix.total_cmp(&b.t_unix));
        chunk.dedup_by(|a, b| a.t_unix == b.t_unix);
        eprintln!("{year}: {} foF2 records", chunk.len());
        fetched += 1;
        records.extend(chunk);
    }
    records.sort_by(|a, b| a.t_unix.total_cmp(&b.t_unix));
    records.dedup_by(|a, b| a.t_unix == b.t_unix);
    eprintln!(
        "SWS {site} {station} {start_year}..{stop_year}: {} records from {fetched} years",
        records.len()
    );
    if records.is_empty() {
        eprintln!("no records — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
    let bytes = write_bin(&records);
    if let Some(parent) = std::path::Path::new(&out).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if std::fs::write(&out, &bytes).is_err() {
        eprintln!("write {out} returned void");
        std::process::exit(1);
    }
    match parse_bin(&bytes) {
        Some(parsed) => {
            eprintln!(
                "{out}: {} records, roundtrip parses ({} B, sha256 {})",
                parsed.len(),
                bytes.len(),
                sha256_hex(&bytes)
            );
        }
        None => {
            eprintln!("{out}: roundtrip parse void");
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_release("downloads.sws.bom.gov.au", &out) {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day() -> String {
        let mut line = String::from("031800250101");
        for hour in 0..24 {
            let token = match hour {
                0 => "  32 ".to_string(),
                1 => "U 24F".to_string(),
                2 => "   0F".to_string(),
                h => format!("  {:02} ", (h as i32 % 20) + 1),
            };
            assert_eq!(token.len(), 5);
            line.push_str(&token);
        }
        line.push('\n');
        line
    }

    #[test]
    fn tokens_are_qualifier_value_descriptor() {
        let r = parse_text(&day(), 2025);
        assert_eq!(r.len(), 23);
        assert_eq!(r[0].fof2_mhz, Some(3.2));
        assert_eq!(r[1].fof2_mhz, Some(2.4));
        assert_eq!(r[0].t_unix, 1735689600.0);
        assert_eq!(r[1].t_unix, 1735693200.0);
    }

    #[test]
    fn a_zero_value_is_absent_not_zero() {
        let r = parse_text(&day(), 2025);
        assert!(r.iter().all(|x| x.fof2_mhz != Some(0.0)));
        assert_eq!(r.len(), 23);
    }

    #[test]
    fn roundtrip_keeps_only_present_values() {
        let r = parse_text(&day(), 2025);
        let bytes = write_bin(&r);
        assert_eq!(bytes.len(), HEADER_BYTES + r.len() * RECORD_BYTES);
        assert_eq!(parse_bin(&bytes), Some(r));
    }

    #[test]
    fn a_foreign_asset_is_void() {
        assert!(parse_bin(b"XXXX").is_none());
    }
}
