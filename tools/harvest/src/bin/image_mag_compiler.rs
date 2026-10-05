use omegaflow::archivar::geo::{COMP_IMAGE_DXDT, GeoRec, MAGIC_IMAGE, parse_bin, write_bin};
use omegaflow::cdn::upload_release;
use omegaflow::lsk::{days_from_civil, parse as parse_lsk};
use std::process::Command;

const BASE: &str = "https://space.fmi.fi/image/www/data_download.php";
const NETLOC: &str = "space.fmi.fi";
const LAT: f64 = 60.50;
const LON: f64 = 24.65;
const ALT: f64 = 0.0;
const MAX_WINDOW_MIN: i64 = 14400;
const DAY_S: f64 = 86400.0;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn civil_from_days(z: i64) -> (i64, i64, i64) {
    let z = z + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
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

fn data_line(parts: &[&str]) -> Option<(i64, i64, i64, i64, i64, i64)> {
    if parts.len() < 9 {
        return None;
    }
    let year: i64 = parts[0].parse().ok()?;
    if !(1982..=2100).contains(&year) {
        return None;
    }
    let month: i64 = parts[1].parse().ok()?;
    let day: i64 = parts[2].parse().ok()?;
    let hh: i64 = parts[3].parse().ok()?;
    let mm: i64 = parts[4].parse().ok()?;
    let ss: i64 = parts[5].parse().ok()?;
    if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
        return None;
    }
    if !(0..=23).contains(&hh) || !(0..=59).contains(&mm) || !(0..=60).contains(&ss) {
        return None;
    }
    Some((year, month, day, hh, mm, ss))
}

fn collect_window(
    text: &str,
    sample_rate: f64,
    out: &mut Vec<(f64, f64, f64)>,
    prev: &mut Option<(f64, f64)>,
) -> usize {
    let mut samples = 0usize;
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        let Some((year, month, day, hh, mm, ss)) = data_line(&parts) else {
            continue;
        };
        let Some(base_days) = days_from_civil(year, month, day) else {
            continue;
        };
        let Ok(x) = parts[6].parse::<f64>() else {
            continue;
        };
        if !x.is_finite() {
            continue;
        }
        let t = base_days as f64 * DAY_S + hh as f64 * 3600.0 + mm as f64 * 60.0 + ss as f64;
        if let Some((pt, px)) = *prev {
            let dt = t - pt;
            if (dt - sample_rate).abs() <= 1.0 {
                out.push((t, -(x - px), dt));
            }
        }
        *prev = Some((t, x));
        samples += 1;
    }
    samples
}

fn compile_window(
    url: &str,
    sample_rate: f64,
    out: &mut Vec<(f64, f64, f64)>,
    prev: &mut Option<(f64, f64)>,
) -> usize {
    let data = match download(url) {
        Some(d) => d,
        None => {
            eprintln!("{url}: download void — the window stays unharvested");
            return 0;
        }
    };
    let Some(text) = String::from_utf8(data).ok() else {
        eprintln!("{url}: body is not UTF-8 — the window stays unread");
        return 0;
    };
    collect_window(&text, sample_rate, out, prev)
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
    let start = match arg_value(&args, "--start") {
        Some(v) => v,
        None => {
            eprintln!(
                "--start <YYYYMMDD> required (the IMAGE archive carries no fabricated default)"
            );
            std::process::exit(1);
        }
    };
    if start.len() != 8 || !start.chars().all(|c| c.is_ascii_digit()) {
        eprintln!("--start {start} names no YYYYMMDD day");
        std::process::exit(1);
    }
    let sy: i64 = match start[0..4].parse() {
        Ok(v) => v,
        Err(_) => {
            eprintln!("--start {start} names no readable year");
            std::process::exit(1);
        }
    };
    let sm: i64 = match start[4..6].parse() {
        Ok(v) => v,
        Err(_) => {
            eprintln!("--start {start} names no readable month");
            std::process::exit(1);
        }
    };
    let sd: i64 = match start[6..8].parse() {
        Ok(v) => v,
        Err(_) => {
            eprintln!("--start {start} names no readable day");
            std::process::exit(1);
        }
    };
    let Some(base_days) = days_from_civil(sy, sm, sd) else {
        eprintln!("--start {start} is no civil day");
        std::process::exit(1);
    };
    let days: i64 = match arg_value(&args, "--days").and_then(|v| v.parse().ok()) {
        Some(v) if v > 0 => v,
        _ => 1,
    };
    let sample_rate: f64 = match arg_value(&args, "--sample-rate") {
        Some(v) => match v.parse::<f64>() {
            Ok(x) if x > 0.0 && x.is_finite() => x,
            _ => {
                eprintln!("--sample-rate carries no positive seconds");
                std::process::exit(1);
            }
        },
        None => 10.0,
    };
    let stations = match arg_value(&args, "--stations") {
        Some(v) => v,
        None => "NUR".to_string(),
    };
    let limit: Option<usize> = arg_value(&args, "--limit").and_then(|v| v.parse().ok());
    let url_arg = arg_value(&args, "--url");

    let urls: Vec<String> = match url_arg {
        Some(u) => vec![u],
        None => {
            let mut out = Vec::new();
            let mut off = 0i64;
            while off < days {
                let chunk_days = (days - off).min(MAX_WINDOW_MIN / 1440);
                let (y, m, d) = civil_from_days(base_days + off);
                let starttime = format!("{y:04}{m:02}{d:02}");
                let length = chunk_days * 1440;
                out.push(format!(
                    "{BASE}?starttime={starttime}&length={length}&format=text2&stations={stations}&sample_rate={sample_rate}"
                ));
                off += chunk_days;
            }
            out
        }
    };

    let mut records: Vec<(f64, f64, f64)> = Vec::new();
    let mut prev: Option<(f64, f64)> = None;
    let mut samples = 0usize;
    for u in &urls {
        samples += compile_window(u, sample_rate, &mut records, &mut prev);
    }
    if records.is_empty() {
        eprintln!("fmi_image_mag: no measured -dX/dt sample — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
    records.sort_by(|a, b| a.0.total_cmp(&b.0));
    records.dedup_by(|a, b| a.0 == b.0);
    let mut geo: Vec<GeoRec> = Vec::with_capacity(records.len());
    for (t, val, dt) in records {
        let Some(tdb) = lsk.unix_to_tdb(t) else {
            continue;
        };
        geo.push(GeoRec {
            t: tdb,
            lat: LAT,
            lon: LON,
            alt: ALT,
            freq: 0.0,
            bin_width: dt,
            val,
            comp: COMP_IMAGE_DXDT,
            station: 0,
        });
    }
    if let Some(cap) = limit {
        geo.truncate(cap);
    }
    if geo.is_empty() {
        eprintln!(
            "fmi_image_mag: no record survives the TDB conversion — the bin stays unwritten (0 honored)"
        );
        std::process::exit(1);
    }
    let bytes = write_bin(MAGIC_IMAGE, &geo);
    if std::fs::write(&out_bin, &bytes).is_err() {
        eprintln!("write {} returned void", out_bin);
        std::process::exit(1);
    }
    match parse_bin(MAGIC_IMAGE, &bytes) {
        Some(parsed) => eprintln!(
            "{}: {} geo records ({} measured X samples → -dX/dt at {:.0} s cadence, {:.1} B), roundtrip parses",
            out_bin,
            parsed.len(),
            samples,
            sample_rate,
            bytes.len() as f64
        ),
        None => {
            eprintln!(
                "{}: roundtrip parse void — the bin stays unverified",
                out_bin
            );
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_release(NETLOC, &out_bin) {
        std::process::exit(1);
    }
}
