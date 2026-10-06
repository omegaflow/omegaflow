use omegaflow::archivar::LeapSeconds;
use omegaflow::archivar::embedded_lsk;
use omegaflow::archivar::geo::{COMP_EPA_AQS_PM25, GeoRec, magic_of, parse_bin, write_bin};
use omegaflow::archivar::quaoar_occlt::{ZipEntry, zip_entries, zip_extract};
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::upload_release;
use omegaflow::lsk::days_from_civil;
use std::process::Command;

const NETLOC: &str = "aqs.epa.gov";
const ZIP_URL: &str = "https://aqs.epa.gov/aqsweb/airdata/daily_88101_2024.zip";
const FORMAT: &str = "epa_aqs_pm25";
const LAT_FIELD: &str = "Latitude";
const LON_FIELD: &str = "Longitude";
const DATE_FIELD: &str = "Date Local";
const VALUE_FIELD: &str = "Arithmetic Mean";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn fetch(url: &str) -> Result<Vec<u8>, String> {
    let out = Command::new("curl")
        .arg("-sSf")
        .arg("--retry")
        .arg("2")
        .arg("--max-time")
        .arg("900")
        .arg(url)
        .output()
        .map_err(|e| format!("curl {url} returned void: {e}"))?;
    if !out.status.success() {
        return Err(format!(
            "{url}: reads no bytes — {}",
            String::from_utf8_lossy(&out.stderr).trim()
        ));
    }
    if out.stdout.is_empty() {
        return Err(format!("{url}: carries no bytes"));
    }
    Ok(out.stdout)
}

fn read_bytes(source: &str) -> Result<Vec<u8>, String> {
    if source.starts_with("http://") || source.starts_with("https://") {
        fetch(source)
    } else {
        std::fs::read(source).map_err(|e| format!("read {source} returned void: {e}"))
    }
}

fn split_csv(line: &str) -> Vec<String> {
    let mut fields: Vec<String> = Vec::new();
    let mut cur = String::new();
    let mut in_quotes = false;
    for c in line.chars() {
        match c {
            '"' if in_quotes => in_quotes = false,
            '"' => in_quotes = true,
            ',' if !in_quotes => {
                fields.push(cur.trim().to_string());
                cur.clear();
            }
            _ => cur.push(c),
        }
    }
    fields.push(cur.trim().to_string());
    fields
}

fn header_fields(header: &str) -> Vec<String> {
    let line = match header.strip_prefix('\u{feff}') {
        Some(s) => s,
        None => header,
    };
    split_csv(line)
}

fn field_index(fields: &[String], name: &str) -> Option<usize> {
    fields.iter().position(|f| f == name)
}

fn find_data_member(entries: &[ZipEntry]) -> Option<&ZipEntry> {
    entries
        .iter()
        .filter(|e| e.name.to_ascii_lowercase().ends_with(".csv"))
        .max_by_key(|e| e.comp_size)
}

fn parse_step(args: &[String], name: &str, default: usize) -> Result<usize, String> {
    match arg_value(args, name) {
        Some(v) => {
            let n = v
                .parse::<usize>()
                .map_err(|_| format!("{name} {v} carries no step"))?;
            if n == 0 {
                return Err(format!("{name} carries no positive step"));
            }
            Ok(n)
        }
        None => Ok(default),
    }
}

fn parse_coord(s: &str) -> Option<f64> {
    let v = s.trim().parse::<f64>().ok()?;
    if v.is_finite() { Some(v) } else { None }
}

fn finite_pm25(s: &str) -> Option<f64> {
    let t = s.trim();
    if t.is_empty() || t.eq_ignore_ascii_case("null") || t.eq_ignore_ascii_case("na") {
        return None;
    }
    let v = t.parse::<f64>().ok()?;
    if v.is_finite() && v >= 0.0 {
        Some(v)
    } else {
        None
    }
}

fn epoch_of_date(s: &str, lsk: &LeapSeconds) -> Option<f64> {
    let mut parts = s.trim().split('-');
    let year: i64 = parts.next()?.parse().ok()?;
    let month: i64 = parts.next()?.parse().ok()?;
    let day: i64 = parts.next()?.parse().ok()?;
    let days = days_from_civil(year, month, day)?;
    lsk.unix_to_tdb(days as f64 * 86400.0)
}

fn parse_aqs(
    text: &str,
    lsk: &LeapSeconds,
    stride: usize,
    limit: usize,
) -> Result<Vec<GeoRec>, String> {
    let mut lines = text.lines().filter(|l| !l.trim().is_empty());
    let header = lines
        .next()
        .ok_or_else(|| "the AQS table carries no header".to_string())?;
    let fields = header_fields(header);
    let lat_idx = field_index(&fields, LAT_FIELD)
        .ok_or_else(|| format!("the AQS table carries no {LAT_FIELD} column"))?;
    let lon_idx = field_index(&fields, LON_FIELD)
        .ok_or_else(|| format!("the AQS table carries no {LON_FIELD} column"))?;
    let date_idx = field_index(&fields, DATE_FIELD)
        .ok_or_else(|| format!("the AQS table carries no {DATE_FIELD} column"))?;
    let value_idx = field_index(&fields, VALUE_FIELD)
        .ok_or_else(|| format!("the AQS table carries no {VALUE_FIELD} column"))?;
    let mut records: Vec<GeoRec> = Vec::new();
    let mut row = 0usize;
    for line in lines {
        let take = row % stride == 0;
        row += 1;
        if !take {
            continue;
        }
        let cells = split_csv(line);
        let Some(lat) = cells.get(lat_idx).and_then(|s| parse_coord(s)) else {
            continue;
        };
        let Some(lon) = cells.get(lon_idx).and_then(|s| parse_coord(s)) else {
            continue;
        };
        if !(-90.0..=90.0).contains(&lat) || !(-180.0..=180.0).contains(&lon) {
            continue;
        }
        let Some(t) = cells.get(date_idx).and_then(|s| epoch_of_date(s, lsk)) else {
            continue;
        };
        let Some(val) = cells.get(value_idx).and_then(|s| finite_pm25(s)) else {
            continue;
        };
        records.push(GeoRec {
            t,
            lat,
            lon,
            alt: 0.0,
            freq: 0.0,
            bin_width: 0.0,
            val,
            comp: COMP_EPA_AQS_PM25,
            station: 0,
        });
        if records.len() >= limit {
            break;
        }
    }
    Ok(records)
}

fn out_path(args: &[String], netloc: &str) -> String {
    match arg_value(args, "--out") {
        Some(p) if !p.is_empty() => p,
        _ => format!("data/{netloc}/{FORMAT}.bin"),
    }
}

fn netloc_of(args: &[String]) -> String {
    match arg_value(args, "--netloc") {
        Some(n) if !n.is_empty() => n,
        _ => NETLOC.to_string(),
    }
}

fn ensure_parent(out: &str) -> Result<(), String> {
    if let Some(parent) = std::path::Path::new(out).parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("create {} void: {e}", parent.display()))?;
        }
    }
    Ok(())
}

fn last_seg(path: &str) -> &str {
    match path.rsplit('/').next() {
        Some(s) => s,
        None => path,
    }
}

fn run(args: &[String]) -> Result<(), String> {
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let stride = parse_step(args, "--stride", 1)?;
    let limit = parse_step(args, "--limit", usize::MAX)?;
    let zip_source = match arg_value(args, "--input") {
        Some(p) => p,
        None => match arg_value(args, "--url") {
            Some(u) => u,
            None => ZIP_URL.to_string(),
        },
    };
    let zip = read_bytes(&zip_source)?;
    let entries = zip_entries(&zip)
        .ok_or_else(|| format!("{zip_source}: central directory void — the zip stays unread"))?;
    let member = find_data_member(&entries)
        .ok_or_else(|| format!("{zip_source}: no .csv member — the table stays unread"))?;
    let raw =
        zip_extract(&zip, member).ok_or_else(|| format!("{}: entry extract void", member.name))?;
    let table = String::from_utf8_lossy(&raw);
    let lsk: LeapSeconds = embedded_lsk()
        .ok_or_else(|| "the embedded naif0012.tls stays unread — no epoch".to_string())?;
    let records = parse_aqs(&table, &lsk, stride, limit)?;
    if records.is_empty() {
        return Err(format!(
            "{zip_source}: no station-day with a measured lat, lon, {DATE_FIELD} and {VALUE_FIELD} — the bin stays unwritten (0 honored)"
        ));
    }
    let magic = magic_of(FORMAT).ok_or_else(|| {
        format!("{FORMAT} carries no geo magic — the per-cell arm stays unwritten")
    })?;
    let netloc = netloc_of(args);
    let out = out_path(args, &netloc);
    ensure_parent(&out)?;
    let bytes = write_bin(magic, &records);
    std::fs::write(&out, &bytes).map_err(|e| format!("write {out} void: {e}"))?;
    match parse_bin(magic, &bytes) {
        Some(parsed) if parsed.len() == records.len() => {
            println!(
                "url https://github.com/omegaflow/sources/releases/download/{netloc}/{}",
                last_seg(&out)
            );
            println!("origin {zip_source}");
            println!("compiler tools/harvest/src/bin/epa_aqs_compiler.rs");
            println!("format {FORMAT}");
            println!("sha256 {}", sha256_hex(&bytes));
            eprintln!(
                "{out}: {} station-days, {} B, roundtrip parses",
                parsed.len(),
                bytes.len()
            );
        }
        Some(parsed) => {
            return Err(format!(
                "{out}: {} parsed vs {} written — the asset stays unverified",
                parsed.len(),
                records.len()
            ));
        }
        None => {
            return Err(format!(
                "{out}: roundtrip parse void — the asset stays unverified"
            ));
        }
    }
    if ci_mode && !upload_release(&netloc, &out) {
        return Err(format!(
            "{out}: CDN upload did not reach the {netloc} release"
        ));
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() || args.iter().any(|a| a == "--help" || a == "-h") {
        eprintln!(
            "usage: epa_aqs_compiler [--input <zip-path>] [--url <zip-url>] [--netloc <netloc>] [--stride N] [--limit N] [--out <path>] [--ci-mode]"
        );
        eprintln!(
            "  reads the US EPA AQS/AirData daily PM2.5 FRM/FEM mass table (parameter 88101)"
        );
        eprintln!("  emits one geo bin record per station-day");
        eprintln!("  unit: ug/m3 (Arithmetic Mean)");
        eprintln!("  epoch: the row's own Date Local, to TDB via the embedded leap table");
        eprintln!("  --limit bounds the emitted station-days; --stride samples every Nth data row");
        eprintln!("  --ci-mode uploads the verified asset to the <netloc> CDN release");
        std::process::exit(1);
    }
    if let Err(msg) = run(&args) {
        eprintln!("epa_aqs_compiler: {msg}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_lsk() -> LeapSeconds {
        embedded_lsk().expect("the embedded leap table reads")
    }

    #[test]
    fn split_csv_keeps_quoted_commas() {
        let fields = split_csv("a,\"b,c\",d");
        assert_eq!(fields, vec!["a", "b,c", "d"]);
    }

    #[test]
    fn header_fields_strip_a_bom() {
        let fields = header_fields("\u{feff}Latitude,Longitude");
        assert_eq!(fields, vec!["Latitude", "Longitude"]);
    }

    #[test]
    fn finite_pm25_reads_measured_and_refuses_absent() {
        assert_eq!(finite_pm25("3.625"), Some(3.625));
        assert_eq!(finite_pm25("0"), Some(0.0));
        assert_eq!(finite_pm25("NULL"), None);
        assert_eq!(finite_pm25("NA"), None);
        assert_eq!(finite_pm25(""), None);
        assert_eq!(finite_pm25("-1"), None);
    }

    #[test]
    fn epoch_of_date_uses_the_row_date() {
        let lsk = test_lsk();
        let d0 = epoch_of_date("2024-01-01", &lsk).unwrap();
        let d1 = epoch_of_date("2024-01-02", &lsk).unwrap();
        assert!((d1 - d0 - 86400.0).abs() < 1.0);
        assert!(epoch_of_date("", &lsk).is_none());
        assert!(epoch_of_date("2024-13-40", &lsk).is_none());
    }

    fn entry(name: &str, comp_size: u64) -> ZipEntry {
        ZipEntry {
            name: name.to_string(),
            method: 0,
            comp_size,
            local_offset: 0,
        }
    }

    #[test]
    fn find_data_member_picks_the_largest_csv() {
        let entries = vec![
            entry("ReadMe.csv", 512),
            entry("daily_88101_2024.csv", 47_053_488),
        ];
        let member = find_data_member(&entries).unwrap();
        assert_eq!(member.name, "daily_88101_2024.csv");
    }

    #[test]
    fn find_data_member_refuses_a_zip_without_a_csv() {
        let entries = vec![entry("data.xlsx", 1_000_000), entry("ReadMe.txt", 10)];
        assert!(find_data_member(&entries).is_none());
    }

    #[test]
    fn parse_aqs_emits_one_record_per_station_day() {
        let lsk = test_lsk();
        let text = "State Code,County Code,Site Num,Parameter Code,POC,Latitude,Longitude,Datum,Parameter Name,Date Local,Units of Measure,Arithmetic Mean\n\
01,003,0010,88101,3,30.497478,-87.880258,NAD83,PM2.5,2024-01-01,ug/m3,3.625\n\
01,003,0010,88101,3,30.497478,-87.880258,NAD83,PM2.5,2024-01-02,ug/m3,6.791667\n";
        let records = parse_aqs(text, &lsk, 1, usize::MAX).unwrap();
        assert_eq!(records.len(), 2);
        assert_eq!(records[0].lat, 30.497478);
        assert_eq!(records[0].lon, -87.880258);
        assert_eq!(records[0].val, 3.625);
        assert_eq!(records[0].comp, COMP_EPA_AQS_PM25);
        assert!(records[1].t > records[0].t);
    }

    #[test]
    fn parse_aqs_skips_rows_missing_lat_lon_date_or_value() {
        let lsk = test_lsk();
        let text = "State Code,County Code,Site Num,Parameter Code,POC,Latitude,Longitude,Datum,Parameter Name,Date Local,Units of Measure,Arithmetic Mean\n\
01,003,0010,88101,3,,-87.880258,NAD83,PM2.5,2024-01-01,ug/m3,3.625\n\
01,003,0010,88101,3,30.5,,NAD83,PM2.5,2024-01-01,ug/m3,3.625\n\
01,003,0010,88101,3,30.5,-87.8,NAD83,PM2.5,,ug/m3,3.625\n\
01,003,0010,88101,3,30.5,-87.8,NAD83,PM2.5,2024-01-01,ug/m3,\n\
01,003,0010,88101,3,30.5,-87.8,NAD83,PM2.5,2024-01-01,ug/m3,1.5\n";
        let records = parse_aqs(text, &lsk, 1, usize::MAX).unwrap();
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].val, 1.5);
    }

    #[test]
    fn parse_aqs_honours_stride_and_limit() {
        let lsk = test_lsk();
        let text = "Latitude,Longitude,Date Local,Arithmetic Mean\n\
30.5,-87.8,2024-01-01,1.0\n\
30.5,-87.8,2024-01-02,2.0\n\
30.5,-87.8,2024-01-03,3.0\n";
        let strided = parse_aqs(text, &lsk, 2, usize::MAX).unwrap();
        assert_eq!(strided.len(), 2);
        let limited = parse_aqs(text, &lsk, 1, 1).unwrap();
        assert_eq!(limited.len(), 1);
    }
}
