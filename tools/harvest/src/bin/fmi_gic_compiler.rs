use omegaflow::archivar::geo::{parse_bin, write_bin, GeoRec, COMP_GIC_A, MAGIC_GIC};
use omegaflow::cdn::upload_release;
use omegaflow::inflate::inflate;
use omegaflow::lsk::{days_from_civil, parse as parse_lsk};
use std::collections::HashMap;
use std::process::Command;

const BASE: &str = "https://space.fmi.fi/gic/man_ascii";
const NETLOC: &str = "space.fmi.fi";
const LAT: f64 = 60.6;
const LON: f64 = 25.2;
const ALT: f64 = 0.0;
const FIRST_YEAR: i64 = 1999;
const LAST_YEAR: i64 = 2023;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn year_zip(year: i64) -> String {
    if year == 2023 {
        "man202301-09.zip".to_string()
    } else {
        format!("man{year}.zip")
    }
}

fn download(url: &str) -> Option<Vec<u8>> {
    let out = Command::new("curl")
        .arg("-sSLf")
        .arg("--retry")
        .arg("3")
        .arg("--max-time")
        .arg("300")
        .arg(url)
        .output()
        .ok()?;
    if out.status.success() {
        Some(out.stdout)
    } else {
        eprintln!(
            "fetch {}: {}",
            url,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        None
    }
}

fn le16(d: &[u8], off: usize) -> usize {
    d[off] as usize | (d[off + 1] as usize) << 8
}

fn le32(d: &[u8], off: usize) -> usize {
    d[off] as usize
        | (d[off + 1] as usize) << 8
        | (d[off + 2] as usize) << 16
        | (d[off + 3] as usize) << 24
}

struct ZipEntry {
    name: String,
    method: usize,
    comp_size: usize,
    local_off: usize,
}

fn zip_entries(data: &[u8]) -> Vec<ZipEntry> {
    let mut out = Vec::new();
    let mut i = 0usize;
    while i + 46 <= data.len() {
        if &data[i..i + 4] == b"PK\x01\x02" {
            let method = le16(data, i + 10);
            let comp_size = le32(data, i + 20);
            let name_len = le16(data, i + 28);
            let extra_len = le16(data, i + 30);
            let comment_len = le16(data, i + 32);
            let local_off = le32(data, i + 42);
            if i + 46 + name_len <= data.len() {
                let name = String::from_utf8_lossy(&data[i + 46..i + 46 + name_len]).into_owned();
                out.push(ZipEntry {
                    name,
                    method,
                    comp_size,
                    local_off,
                });
            }
            i += 46 + name_len + extra_len + comment_len;
            continue;
        }
        i += 1;
    }
    out
}

fn entry_body(data: &[u8], e: &ZipEntry) -> Option<Vec<u8>> {
    if e.local_off + 30 > data.len() {
        return None;
    }
    let name_len = le16(data, e.local_off + 26);
    let extra_len = le16(data, e.local_off + 28);
    let start = e.local_off + 30 + name_len + extra_len;
    if start + e.comp_size > data.len() {
        return None;
    }
    let body = &data[start..start + e.comp_size];
    match e.method {
        0 => Some(body.to_vec()),
        8 => inflate(body),
        _ => None,
    }
}

fn file_unix(text: &str) -> Option<(i64, i64, i64)> {
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('%') {
            continue;
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 3 {
            let year: i64 = parts[0].parse().ok()?;
            let month: i64 = parts[1].parse().ok()?;
            let day: i64 = parts[2].parse().ok()?;
            if (1990..=2030).contains(&year) && (1..=12).contains(&month) && (1..=31).contains(&day)
            {
                return Some((year, month, day));
            }
        }
        return None;
    }
    None
}

fn collect_day(text: &str, buckets: &mut HashMap<i64, (f64, f64, f64)>, bucket_s: f64) -> usize {
    let Some((year, month, day)) = file_unix(text) else {
        return 0;
    };
    let Some(base_days) = days_from_civil(year, month, day) else {
        return 0;
    };
    let base = base_days as f64 * 86400.0;
    let mut samples = 0usize;
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('%') {
            continue;
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != 4 {
            continue;
        }
        let Ok(hh) = parts[0].parse::<i64>() else {
            continue;
        };
        if !(0..=23).contains(&hh) {
            continue;
        }
        let Ok(mm) = parts[1].parse::<i64>() else {
            continue;
        };
        let Ok(ss) = parts[2].parse::<i64>() else {
            continue;
        };
        if !(0..=59).contains(&mm) || !(0..=59).contains(&ss) {
            continue;
        }
        let Ok(v) = parts[3].parse::<f64>() else {
            continue;
        };
        if !v.is_finite() {
            continue;
        }
        samples += 1;
        let t = base + hh as f64 * 3600.0 + mm as f64 * 60.0 + ss as f64;
        let bucket = (t / bucket_s).floor() as i64;
        let a = v.abs();
        let cur = buckets
            .entry(bucket)
            .or_insert_with(|| (f64::NEG_INFINITY, 0.0, 0.0));
        if a > cur.0 {
            *cur = (a, v, t);
        }
    }
    samples
}

fn compile_year(url: &str, buckets: &mut HashMap<i64, (f64, f64, f64)>, bucket_s: f64) -> usize {
    let data = match download(url) {
        Some(d) => d,
        None => {
            eprintln!("{url}: download void — the year stays unharvested");
            return 0;
        }
    };
    let entries = zip_entries(&data);
    let mut days = 0usize;
    let mut samples = 0usize;
    let mut files = 0usize;
    for e in &entries {
        if !e.name.ends_with(".txt") {
            continue;
        }
        files += 1;
        let Some(body) = entry_body(&data, e) else {
            continue;
        };
        let Some(text) = String::from_utf8(body).ok() else {
            continue;
        };
        let s = collect_day(&text, buckets, bucket_s);
        days += 1;
        samples += s;
    }
    eprintln!("{url}: {files} daily files, {days} parsed, {samples} measured Ampere samples");
    samples
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out_bin = match arg_value(&args, "--out-bin") {
        Some(v) => v,
        None => {
            eprintln!("--out-bin <path> required");
            std::process::exit(1);
        }
    };
    let lsk_text = match arg_value(&args, "--lsk").and_then(|p| std::fs::read_to_string(p).ok()) {
        Some(t) => t,
        None => {
            eprintln!("--lsk absent — the TDB conversion stays void (no fabricated epoch)");
            std::process::exit(1);
        }
    };
    let lsk = match parse_lsk(&lsk_text) {
        Some(l) => l,
        None => {
            eprintln!("--lsk parses void — the leap-second table stays unread");
            std::process::exit(1);
        }
    };
    let bucket_min: f64 = arg_value(&args, "--bucket-min")
        .and_then(|v| v.parse().ok())
        .unwrap_or(60.0);
    if !(bucket_min > 0.0) || !bucket_min.is_finite() {
        eprintln!(
            "--bucket-min {} carries no positive bucket width",
            bucket_min
        );
        std::process::exit(1);
    }
    let bucket_s = bucket_min * 60.0;
    let limit: Option<usize> = arg_value(&args, "--limit").and_then(|v| v.parse().ok());
    let year_arg = arg_value(&args, "--year");
    let url_arg = arg_value(&args, "--url");
    let (start, end) = match (&url_arg, &year_arg) {
        (Some(_), _) => (FIRST_YEAR, FIRST_YEAR),
        (None, Some(y)) => match y.parse::<i64>() {
            Ok(y) => {
                if !(FIRST_YEAR..=LAST_YEAR).contains(&y) {
                    eprintln!(
                        "--year {y} outside the published archive {FIRST_YEAR}..={LAST_YEAR}"
                    );
                    std::process::exit(1);
                }
                (y, y)
            }
            Err(_) => {
                eprintln!("--year {y} parses void");
                std::process::exit(1);
            }
        },
        (None, None) => (FIRST_YEAR, LAST_YEAR),
    };
    let urls: Vec<String> = match &url_arg {
        Some(u) => vec![u.clone()],
        None => (start..=end)
            .map(year_zip)
            .map(|f| format!("{BASE}/{f}"))
            .collect(),
    };
    let mut buckets: HashMap<i64, (f64, f64, f64)> = HashMap::new();
    let mut samples = 0usize;
    for u in &urls {
        samples += compile_year(u, &mut buckets, bucket_s);
    }
    if buckets.is_empty() {
        eprintln!("fmi_gic: no measured Ampere samples — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
    let mut records: Vec<GeoRec> = Vec::with_capacity(buckets.len());
    for (_, (_, val, t)) in buckets {
        let Some(tdb) = lsk.unix_to_tdb(t) else {
            continue;
        };
        records.push(GeoRec {
            t: tdb,
            lat: LAT,
            lon: LON,
            alt: ALT,
            freq: 0.0,
            bin_width: bucket_s,
            val,
            comp: COMP_GIC_A,
        });
    }
    records.sort_by(|a, b| a.t.total_cmp(&b.t));
    if let Some(cap) = limit {
        records.truncate(cap);
    }
    if records.is_empty() {
        eprintln!(
            "fmi_gic: no bucket survives the TDB conversion — the bin stays unwritten (0 honored)"
        );
        std::process::exit(1);
    }
    let bytes = write_bin(MAGIC_GIC, &records);
    if std::fs::write(&out_bin, &bytes).is_err() {
        eprintln!("write {} returned void", out_bin);
        std::process::exit(1);
    }
    match parse_bin(MAGIC_GIC, &bytes) {
        Some(parsed) => eprintln!(
            "{}: {} geo records ({} measured Ampere samples → {}-min peak-magnitude buckets, {:.1} B), roundtrip parses",
            out_bin,
            parsed.len(),
            samples,
            bucket_min,
            bytes.len() as f64
        ),
        None => {
            eprintln!("{}: roundtrip parse void — the bin stays unverified", out_bin);
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_release(NETLOC, &out_bin) {
        std::process::exit(1);
    }
}
