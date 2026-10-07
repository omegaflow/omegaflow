use omegaflow::archivar::fetch_raw;
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::upload_release;
use omegaflow::lsk::days_from_civil;

const MAGIC: &[u8; 4] = b"PCN1";
const FIELDS: usize = 4;
const NETLOC: &str = "ftp.space.dtu.dk";
const BASE: &str = "https://ftp.space.dtu.dk/WDC/indices/pcn/PCN_definitive";
const COMPILER: &str = "tools/harvest/src/bin/pcn_compiler.rs";
const TTL_S: f64 = 86400.0;
const SAMPLE_TAU_S: f64 = 60.0;

#[derive(Clone, Debug, PartialEq)]
struct PcnSample {
    t_unix: f64,
    val: Option<f64>,
}

#[derive(Clone, Debug, PartialEq)]
struct PcnFile {
    lat: f64,
    lon: f64,
    samples: Vec<PcnSample>,
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn header_kv(line: &str) -> Option<(&str, &str)> {
    let s = line.trim_end_matches(['|', ' ', '\r']);
    if s.len() < 24 {
        return None;
    }
    Some((s.get(0..24)?.trim(), s.get(24..)?.trim()))
}

fn header_f64(value: &str) -> Option<f64> {
    value.parse::<f64>().ok().filter(|v| v.is_finite())
}

fn wrap_lon(lon: f64) -> f64 {
    if lon > 180.0 { lon - 360.0 } else { lon }
}

fn parse_date_time(date: &str, time: &str) -> Option<f64> {
    let mut d = date.split('-');
    let y: i64 = d.next()?.parse().ok()?;
    let mo: i64 = d.next()?.parse().ok()?;
    let da: i64 = d.next()?.parse().ok()?;
    if d.next().is_some() {
        return None;
    }
    let mut t = time.split(':');
    let hh: i64 = t.next()?.parse().ok()?;
    let mm: i64 = t.next()?.parse().ok()?;
    let ss: f64 = t.next()?.parse().ok()?;
    if t.next().is_some() || !(0.0..=60.0).contains(&ss) {
        return None;
    }
    let days = days_from_civil(y, mo, da)?;
    Some(days as f64 * 86400.0 + hh as f64 * 3600.0 + mm as f64 * 60.0 + ss)
}

fn pcn_value(token: &str) -> Option<f64> {
    if token.split('.').next() == Some("99999") {
        return None;
    }
    token.parse::<f64>().ok().filter(|v| v.is_finite())
}

fn parse_pcn(text: &str) -> Option<PcnFile> {
    let lines: Vec<&str> = text.lines().collect();
    let first = lines.iter().find(|l| !l.trim().is_empty())?;
    let (format_key, format_value) = header_kv(first)?;
    if format_key != "Format" || format_value != "IAGA-2002" {
        return None;
    }
    let header_end = lines
        .iter()
        .position(|l| l.trim_start().starts_with("DATE"))?;
    let mut code: Option<&str> = None;
    let mut lat: Option<f64> = None;
    let mut lon: Option<f64> = None;
    for line in &lines[..header_end] {
        let Some((key, value)) = header_kv(line) else {
            continue;
        };
        match key {
            "IAGA Code" => code = Some(value),
            "Geodetic Latitude" => lat = header_f64(value),
            "Geodetic Longitude" => lon = header_f64(value),
            _ => {}
        }
    }
    if code != Some("PCN") {
        return None;
    }
    let (Some(lat), Some(lon)) = (lat, lon) else {
        return None;
    };
    if !(-90.0..=90.0).contains(&lat) {
        return None;
    }
    let lon = wrap_lon(lon);
    if !(-180.0..=180.0).contains(&lon) {
        return None;
    }
    let column_line = lines[header_end].trim().trim_end_matches('|').trim_end();
    let tokens: Vec<&str> = column_line.split_whitespace().collect();
    if tokens != ["DATE", "TIME", "DOY", "PCN"] {
        return None;
    }
    let mut samples: Vec<PcnSample> = Vec::new();
    for line in &lines[header_end + 1..] {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let cells: Vec<&str> = line.split_whitespace().collect();
        if cells.len() != 4 || cells[2].parse::<u32>().is_err() {
            continue;
        }
        let Some(t_unix) = parse_date_time(cells[0], cells[1]) else {
            continue;
        };
        samples.push(PcnSample {
            t_unix,
            val: pcn_value(cells[3]),
        });
    }
    if samples.is_empty() {
        return None;
    }
    Some(PcnFile { lat, lon, samples })
}

fn to_records(file: &PcnFile) -> Vec<[f64; FIELDS]> {
    let mut out = Vec::new();
    for s in &file.samples {
        let Some(v) = s.val else { continue };
        out.push([s.t_unix, file.lat, file.lon, v]);
    }
    out
}

fn write_bin(records: &[[f64; FIELDS]]) -> Vec<u8> {
    let mut out = Vec::with_capacity(8 + records.len() * FIELDS * 8);
    out.extend_from_slice(MAGIC);
    out.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for r in records {
        for v in r {
            out.extend_from_slice(&v.to_le_bytes());
        }
    }
    out
}

fn parse_bin(bytes: &[u8]) -> Option<Vec<[f64; FIELDS]>> {
    if bytes.get(0..4)? != MAGIC {
        return None;
    }
    let n = u32::from_le_bytes(bytes.get(4..8)?.try_into().ok()?) as usize;
    if bytes.len() != 8 + n * FIELDS * 8 {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let mut r = [0.0f64; FIELDS];
        for (j, slot) in r.iter_mut().enumerate() {
            let off = 8 + i * FIELDS * 8 + j * 8;
            *slot = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        }
        out.push(r);
    }
    Some(out)
}

fn emit(text: &str, out: &str, source: &str) {
    let Some(file) = parse_pcn(text) else {
        eprintln!("{source}: IAGA-2002 PCN text parses void — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    };
    let mut records = to_records(&file);
    records.sort_by(|a, b| a[0].total_cmp(&b[0]));
    if records.is_empty() {
        eprintln!(
            "{source}: {} samples carry no present PCN value — the bin stays unwritten (0 honored)",
            file.samples.len()
        );
        std::process::exit(1);
    }
    let bytes_out = write_bin(&records);
    if let Some(parent) = std::path::Path::new(out).parent()
        && !parent.as_os_str().is_empty()
    {
        std::fs::create_dir_all(parent).ok();
    }
    if std::fs::write(out, &bytes_out).is_err() {
        eprintln!("write {out} returned void");
        std::process::exit(1);
    }
    let v_min = records.iter().map(|r| r[3]).fold(f64::INFINITY, f64::min);
    let v_max = records
        .iter()
        .map(|r| r[3])
        .fold(f64::NEG_INFINITY, f64::max);
    let t_min = match records.first() {
        Some(r) => r[0],
        None => f64::NAN,
    };
    let t_max = match records.last() {
        Some(r) => r[0],
        None => f64::NAN,
    };
    eprintln!(
        "{out}: {} records ({} samples, {} absent skipped), PCN [{}, {}] mV/m, t [{}, {}] unix",
        records.len(),
        file.samples.len(),
        file.samples.len() - records.len(),
        v_min,
        v_max,
        t_min,
        t_max
    );
    eprintln!("  station PCN lat {} lon {}", file.lat, file.lon);
    eprintln!(
        "  {} bytes, sha256 {}",
        bytes_out.len(),
        sha256_hex(&bytes_out)
    );
    match parse_bin(&bytes_out) {
        Some(parsed) if parsed == records => {
            eprintln!("  roundtrip: {} records parse, identical", parsed.len());
        }
        Some(parsed) => {
            eprintln!(
                "  roundtrip: {} records parse but differ from the emitted set",
                parsed.len()
            );
            std::process::exit(1);
        }
        None => {
            eprintln!("  roundtrip parse void — the bin stays unverified");
            std::process::exit(1);
        }
    }
    let out_name = match std::path::Path::new(out).file_name() {
        Some(n) => n.to_string_lossy().into_owned(),
        None => out.to_string(),
    };
    println!("url https://github.com/omegaflow/sources/releases/download/{NETLOC}/{out_name}");
    println!("origin {source}");
    println!("compiler {COMPILER}");
    println!("sha256 {}", sha256_hex(&bytes_out));
    println!("format pcn");
    println!("ttl {}", TTL_S as u64);
    println!("at earth");
    println!(
        "field pcn_mvm pcn_mvm inverse-square electric mV/m {} 0.0 0.0",
        SAMPLE_TAU_S as u64
    );
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let year = match arg_value(&args, "--year") {
        Some(v) => v,
        None => {
            eprintln!("pcn_compiler: --year <YYYY> is required");
            std::process::exit(2);
        }
    };
    let out = match arg_value(&args, "--out") {
        Some(v) => v,
        None => format!("pcn_{year}.bin"),
    };
    let url = match arg_value(&args, "--url") {
        Some(v) => v,
        None => format!("{BASE}/pcn{year}d.dat"),
    };
    let text = match arg_value(&args, "--file") {
        Some(path) => match std::fs::read_to_string(&path) {
            Ok(t) => t,
            Err(_) => {
                eprintln!("{path} unreadable — the bin stays unwritten (0 honored)");
                std::process::exit(1);
            }
        },
        None => match fetch_raw(&url, None, &[]) {
            Some(t) => t,
            None => {
                eprintln!("{url} fetch void — the bin stays unwritten (0 honored)");
                std::process::exit(1);
            }
        },
    };
    let source = match arg_value(&args, "--file") {
        Some(v) => v,
        None => url,
    };
    emit(&text, &out, &source);
    if ci_mode && !upload_release(NETLOC, &out) {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hdr(key: &str, value: &str) -> String {
        format!(" {key:<23}{value}\n")
    }

    fn measured() -> String {
        let mut s = String::new();
        s.push_str(&hdr("Format", "IAGA-2002"));
        s.push_str(&hdr("Source of Data", "DTU"));
        s.push_str(&hdr(
            "Station Name",
            "Polar Cap North (PCN) index (from THL obs.)",
        ));
        s.push_str(&hdr("IAGA Code", "PCN"));
        s.push_str(&hdr("Geodetic Latitude", "77.467"));
        s.push_str(&hdr("Geodetic Longitude", "290.767"));
        s.push_str(&hdr("Data Interval Type", "1-min"));
        s.push_str(&hdr("Data type", "Definitive"));
        s.push_str(&hdr("# Missing values are indi", "cated by value 99999.00"));
        s.push_str(&hdr("# PCN: Polar Cap index N", "orth, unit 1 mV/m"));
        s.push_str(&hdr(
            "# License: CC BY https:/",
            "/creativecommons.org/licenses/by/4.0/",
        ));
        s.push_str("DATE       TIME         DOY       PCN                               |\n");
        s.push_str(" 2015-01-01 00:00:00.000 001       -0.23\n");
        s.push_str(" 2015-01-01 00:01:00.000 001       -0.28\n");
        s.push_str(" 2015-01-01 00:02:00.000 001       99999.00\n");
        s
    }

    #[test]
    fn parse_pcn_reads_the_measured_index() {
        let file = parse_pcn(&measured()).expect("the measured PCN text parses");
        assert_eq!(file.lat, 77.467);
        assert!((file.lon + 69.233).abs() < 1e-9, "lon {}", file.lon);
        assert_eq!(file.samples.len(), 3);
        assert_eq!(file.samples[0].val, Some(-0.23));
        assert_eq!(file.samples[1].val, Some(-0.28));
        assert_eq!(file.samples[2].val, None);
        let days = days_from_civil(2015, 1, 1).unwrap();
        assert_eq!(file.samples[0].t_unix, (days * 86400) as f64);
        assert_eq!(file.samples[1].t_unix - file.samples[0].t_unix, 60.0);
    }

    #[test]
    fn to_records_skips_absent_values() {
        let file = parse_pcn(&measured()).unwrap();
        let records = to_records(&file);
        assert_eq!(records.len(), 2);
        assert_eq!(
            records[0],
            [file.samples[0].t_unix, 77.467, file.lon, -0.23]
        );
        assert_eq!(
            records[1],
            [file.samples[1].t_unix, 77.467, file.lon, -0.28]
        );
    }

    #[test]
    fn parse_pcn_refuses_foreign_text() {
        assert!(parse_pcn("").is_none());
        assert!(parse_pcn("MSTK  53.351 247.026 20230423 GEODETIC nT  1Hz\n").is_none());
        assert!(parse_pcn(&measured().replace("IAGA-2002", "IAGA-2003")).is_none());
        assert!(parse_pcn(&measured().replace("PCN", "FCC")).is_none());
        assert!(parse_pcn(&measured().replace("77.467", "      ")).is_none());
    }

    #[test]
    fn bin_roundtrip_preserves_records() {
        let records = vec![
            [0.0, 77.467, -69.233, -0.23],
            [60.0, 77.467, -69.233, -0.28],
        ];
        let bytes = write_bin(&records);
        assert_eq!(bytes.len(), 8 + records.len() * FIELDS * 8);
        assert_eq!(parse_bin(&bytes).as_deref(), Some(records.as_slice()));
    }

    #[test]
    fn parse_bin_rejects_foreign_magic_and_short_body() {
        assert!(parse_bin(b"XXXX").is_none());
        assert!(parse_bin(b"PCN1").is_none());
        let good = write_bin(&[[0.0; FIELDS]]);
        let short = good[..good.len() - 1].to_vec();
        assert!(parse_bin(&short).is_none());
    }
}
