use omegaflow::archivar::LeapSeconds;
use omegaflow::archivar::embedded_lsk;
use omegaflow::archivar::geo::{COMP_USDA_FARA_SHARE, GeoRec, magic_of, parse_bin, write_bin};
use omegaflow::archivar::quaoar_occlt::{ZipEntry, zip_entries, zip_extract};
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::upload_release;
use omegaflow::lsk::days_from_civil;
use std::collections::HashMap;
use std::process::Command;

const NETLOC: &str = "ers.usda.gov";
const ZIP_URL: &str = "https://www.ers.usda.gov/media/5627/2019-large-retailer-access-map-lram-formerly-known-as-the-food-access-research-atlas-fara-data.zip";
const CENTERS_URL: &str =
    "https://www2.census.gov/geo/docs/reference/cenpop2020/tract/CenPop2020_Mean_TR.txt";
const FORMAT: &str = "usda_fara_low_access";
const FIPS_FIELD: &str = "CensusTract";
const VALUE_FIELD: &str = "lapophalfshare";
const EPOCH_YEAR: i64 = 2019;
const EPOCH_MONTH: i64 = 1;
const EPOCH_DAY: i64 = 1;

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

fn finite_share(s: &str) -> Option<f64> {
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

fn pad_fips(s: &str, width: usize) -> String {
    let t = s.trim();
    if t.len() >= width {
        return t.to_string();
    }
    let mut out = String::with_capacity(width);
    for _ in t.len()..width {
        out.push('0');
    }
    out.push_str(t);
    out
}

fn parse_centers(text: &str) -> Result<HashMap<String, (f64, f64)>, String> {
    let mut lines = text.lines().filter(|l| !l.trim().is_empty());
    let header = lines
        .next()
        .ok_or_else(|| "the census centre file carries no header".to_string())?;
    let fields = header_fields(header);
    let state_idx = field_index(&fields, "STATEFP")
        .ok_or_else(|| "the census centre file carries no STATEFP".to_string())?;
    let county_idx = field_index(&fields, "COUNTYFP")
        .ok_or_else(|| "the census centre file carries no COUNTYFP".to_string())?;
    let tract_idx = field_index(&fields, "TRACTCE")
        .ok_or_else(|| "the census centre file carries no TRACTCE".to_string())?;
    let lat_idx = field_index(&fields, "LATITUDE")
        .ok_or_else(|| "the census centre file carries no LATITUDE".to_string())?;
    let lon_idx = field_index(&fields, "LONGITUDE")
        .ok_or_else(|| "the census centre file carries no LONGITUDE".to_string())?;
    let mut out: HashMap<String, (f64, f64)> = HashMap::new();
    for line in lines {
        let cells = split_csv(line);
        let Some(state) = cells.get(state_idx) else {
            continue;
        };
        let Some(county) = cells.get(county_idx) else {
            continue;
        };
        let Some(tract) = cells.get(tract_idx) else {
            continue;
        };
        let Some(lat) = cells.get(lat_idx).and_then(|s| parse_coord(s)) else {
            continue;
        };
        let Some(lon) = cells.get(lon_idx).and_then(|s| parse_coord(s)) else {
            continue;
        };
        if !(-90.0..=90.0).contains(&lat) || !(-180.0..=180.0).contains(&lon) {
            continue;
        }
        let key = format!(
            "{}{}{}",
            pad_fips(state, 2),
            pad_fips(county, 3),
            pad_fips(tract, 6)
        );
        out.insert(key, (lat, lon));
    }
    if out.is_empty() {
        return Err("the census centre file carries no usable tract centre".to_string());
    }
    Ok(out)
}

fn epoch_tdb() -> Result<f64, String> {
    let lsk: LeapSeconds = embedded_lsk()
        .ok_or_else(|| "the embedded naif0012.tls stays unread — no epoch".to_string())?;
    let days = days_from_civil(EPOCH_YEAR, EPOCH_MONTH, EPOCH_DAY)
        .ok_or_else(|| "the 2019-01-01 epoch stays uncompiled".to_string())?;
    let unix = days as f64 * 86400.0;
    lsk.unix_to_tdb(unix)
        .ok_or_else(|| "the 2019-01-01 epoch stays untranslated".to_string())
}

fn parse_fara(
    text: &str,
    centers: &HashMap<String, (f64, f64)>,
    t: f64,
    stride: usize,
    limit: usize,
) -> Result<Vec<GeoRec>, String> {
    let mut lines = text.lines().filter(|l| !l.trim().is_empty());
    let header = lines
        .next()
        .ok_or_else(|| "the FARA table carries no header".to_string())?;
    let fields = header_fields(header);
    let fips_idx = field_index(&fields, FIPS_FIELD)
        .ok_or_else(|| format!("the FARA table carries no {FIPS_FIELD} column"))?;
    let value_idx = field_index(&fields, VALUE_FIELD)
        .ok_or_else(|| format!("the FARA table carries no {VALUE_FIELD} column"))?;
    let mut records: Vec<GeoRec> = Vec::new();
    let mut row = 0usize;
    for line in lines {
        let take = row % stride == 0;
        row += 1;
        if !take {
            continue;
        }
        let cells = split_csv(line);
        let Some(fips) = cells.get(fips_idx) else {
            continue;
        };
        let Some(val) = cells.get(value_idx).and_then(|s| finite_share(s)) else {
            continue;
        };
        let Some(&(lat, lon)) = centers.get(fips.trim()) else {
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
            comp: COMP_USDA_FARA_SHARE,
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
    let centers_source = match arg_value(args, "--centers") {
        Some(c) => c,
        None => CENTERS_URL.to_string(),
    };
    let zip = read_bytes(&zip_source)?;
    let entries = zip_entries(&zip)
        .ok_or_else(|| format!("{zip_source}: central directory void — the zip stays unread"))?;
    let member = find_data_member(&entries)
        .ok_or_else(|| format!("{zip_source}: no .csv member — the table stays unread"))?;
    let raw =
        zip_extract(&zip, member).ok_or_else(|| format!("{}: entry extract void", member.name))?;
    let fara = String::from_utf8_lossy(&raw);
    let center_bytes = read_bytes(&centers_source)?;
    let center_text = String::from_utf8_lossy(&center_bytes);
    let centers = parse_centers(&center_text)?;
    let t = epoch_tdb()?;
    let records = parse_fara(&fara, &centers, t, stride, limit)?;
    if records.is_empty() {
        return Err(format!(
            "{zip_source}: no matched tract centre with a measured {VALUE_FIELD} — the bin stays unwritten (0 honored)"
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
            println!("origin {centers_source}");
            println!("compiler tools/harvest/src/bin/usda_fara_compiler.rs");
            println!("format {FORMAT}");
            println!("sha256 {}", sha256_hex(&bytes));
            eprintln!(
                "{out}: {} matched tracts, {} B, roundtrip parses",
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
            "usage: usda_fara_compiler [--input <zip-path>] [--url <zip-url>] [--centers <path|url>] [--netloc <netloc>] [--stride N] [--limit N] [--out <path>] [--ci-mode]"
        );
        eprintln!("  joins the USDA FARA tract table with the 2020 census tract centres");
        eprintln!("  unit: low-access population share at 1/2 mile (lapophalfshare)");
        eprintln!("  epoch: the dataset's own year, 2019-01-01 TDB");
        eprintln!("  --limit bounds the emitted tracts; --stride samples every Nth tract row");
        eprintln!("  --ci-mode uploads the verified asset to the <netloc> CDN release");
        std::process::exit(1);
    }
    if let Err(msg) = run(&args) {
        eprintln!("usda_fara_compiler: {msg}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_csv_keeps_quoted_commas() {
        let fields = split_csv("a,\"b,c\",d");
        assert_eq!(fields, vec!["a", "b,c", "d"]);
    }

    #[test]
    fn header_fields_strip_a_bom() {
        let fields = header_fields("\u{feff}CensusTract,lapophalfshare");
        assert_eq!(fields, vec!["CensusTract", "lapophalfshare"]);
    }

    #[test]
    fn finite_share_reads_measured_and_refuses_absent() {
        assert_eq!(finite_share("0.1442"), Some(0.1442));
        assert_eq!(finite_share("0"), Some(0.0));
        assert_eq!(finite_share("NULL"), None);
        assert_eq!(finite_share("NA"), None);
        assert_eq!(finite_share(""), None);
        assert_eq!(finite_share("-1"), None);
    }

    #[test]
    fn pad_fips_keeps_leading_zeros() {
        assert_eq!(pad_fips("1", 2), "01");
        assert_eq!(pad_fips("100", 6), "000100");
        assert_eq!(pad_fips("010", 2), "010");
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
            entry("VariableLookup.csv", 21_531),
            entry("Food Access Research Atlas.csv", 47_053_488),
        ];
        let member = find_data_member(&entries).unwrap();
        assert_eq!(member.name, "Food Access Research Atlas.csv");
    }

    #[test]
    fn find_data_member_refuses_a_zip_without_a_csv() {
        let entries = vec![entry("data.xlsx", 1_000_000), entry("ReadMe.txt", 10)];
        assert!(find_data_member(&entries).is_none());
    }

    #[test]
    fn parse_centers_keys_the_11_digit_fips() {
        let text = "STATEFP,COUNTYFP,TRACTCE,POPULATION,LATITUDE,LONGITUDE\n\
01,001,000100,4820,32.318,-86.902\n\
02,020,000200,1000,61.37,-152.4\n";
        let centers = parse_centers(text).unwrap();
        assert_eq!(centers.len(), 2);
        assert_eq!(centers.get("01001000100"), Some(&(32.318, -86.902)));
        assert_eq!(centers.get("02020000200"), Some(&(61.37, -152.4)));
    }

    #[test]
    fn parse_fara_joins_centres_and_skips_absent() {
        let text = "CensusTract,State,lapophalfshare\n\
01001000100,Alabama,0.1442\n\
01001000200,Alabama,NULL\n\
99999999999,Nowhere,0.5\n";
        let mut centers: HashMap<String, (f64, f64)> = HashMap::new();
        centers.insert("01001000100".to_string(), (32.318, -86.902));
        let records = parse_fara(text, &centers, 1546300800.0, 1, usize::MAX).unwrap();
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].lat, 32.318);
        assert_eq!(records[0].lon, -86.902);
        assert_eq!(records[0].val, 0.1442);
        assert_eq!(records[0].t, 1546300800.0);
        assert_eq!(records[0].comp, COMP_USDA_FARA_SHARE);
    }

    #[test]
    fn parse_fara_honours_stride_and_limit() {
        let text = "CensusTract,State,lapophalfshare\n\
01001000100,A,0.1\n\
01001000100,A,0.2\n\
01001000100,A,0.3\n";
        let mut centers: HashMap<String, (f64, f64)> = HashMap::new();
        centers.insert("01001000100".to_string(), (32.318, -86.902));
        let strided = parse_fara(text, &centers, 1.0, 2, usize::MAX).unwrap();
        assert_eq!(strided.len(), 2);
        let limited = parse_fara(text, &centers, 1.0, 1, 1).unwrap();
        assert_eq!(limited.len(), 1);
    }

    #[test]
    fn epoch_is_the_dataset_year() {
        let t = epoch_tdb().unwrap();
        assert!(t > 1546300800.0);
        assert!(t < 1546300800.0 + 86400.0);
    }
}
