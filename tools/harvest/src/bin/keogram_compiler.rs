use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::cdn::upload_release;
use omegaflow::keogram::{brightness_columns, parse_bin, write_bin};
use omegaflow::lsk::days_from_civil;
use omegaflow::spectral::civil_from_days;

const HOST: &str = "space.fmi.fi";
const BASE: &str = "https://space.fmi.fi/MIRACLE/ASC/ASC_keograms";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn parse_date(s: &str) -> Option<i64> {
    let year: i64 = s.get(0..4)?.parse().ok()?;
    let month: i64 = s.get(5..7)?.parse().ok()?;
    let day: i64 = s.get(8..10)?.parse().ok()?;
    let days = days_from_civil(year, month, day)?;
    let (cy, cm, cd) = civil_from_days(days)?;
    if cy as i64 != year || cm as i64 != month || cd as i64 != day {
        return None;
    }
    Some(days)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let station = match arg_value(&args, "--station") {
        Some(s) => s.to_uppercase(),
        None => {
            eprintln!("keogram_compiler needs --station <icao>");
            std::process::exit(2);
        }
    };
    let (start, stop) = match (arg_value(&args, "--start"), arg_value(&args, "--stop")) {
        (Some(s), Some(e)) => (s, e),
        _ => {
            eprintln!("keogram_compiler needs --start <YYYY-MM-DD> and --stop <YYYY-MM-DD>");
            std::process::exit(2);
        }
    };
    let (start_day, stop_day) = match (parse_date(&start), parse_date(&stop)) {
        (Some(s), Some(e)) => (s, e),
        _ => {
            eprintln!("--start/--stop must be YYYY-MM-DD");
            std::process::exit(2);
        }
    };
    if stop_day < start_day {
        eprintln!("--stop must not lie before --start");
        std::process::exit(2);
    }
    let out = match arg_value(&args, "--out") {
        Some(o) => o,
        None => format!("keogram_{station}.bin"),
    };

    let mut rows: Vec<(f64, u32, f64)> = Vec::new();
    for day in start_day..=stop_day {
        let Some((year, month, mday)) = civil_from_days(day) else {
            continue;
        };
        let yy = year % 100;
        let dir = format!("{station}.{yy:02}{month:02}");
        let file = format!("{station}_{yy:02}{month:02}{mday:02}.jpg");
        let url = format!("{BASE}/{dir}/{file}");
        let Some(jpeg) = fetch_raw_bytes(&url) else {
            eprintln!("{file} fetch void — night skipped (0 honored)");
            continue;
        };
        let Some(cols) = brightness_columns(&jpeg) else {
            eprintln!("{file} decodes void — night skipped (0 honored)");
            continue;
        };
        let t_unix = day as f64 * 86400.0;
        let mut kept = 0usize;
        for (i, &m) in cols.mean.iter().enumerate() {
            if !m.is_finite() {
                continue;
            }
            rows.push((t_unix, i as u32, m));
            kept += 1;
        }
        eprintln!("{file}: {}x{} -> {kept} columns", cols.width, cols.height);
    }
    rows.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.cmp(&b.1)));
    if rows.is_empty() {
        eprintln!("no columns across {start}..{stop} — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
    let bytes = write_bin(&rows);
    if std::fs::write(&out, &bytes).is_err() {
        eprintln!("write {out} returned void");
        std::process::exit(1);
    }
    match parse_bin(&bytes) {
        Some(parsed) => {
            eprintln!(
                "{out}: {} columns, roundtrip parses ({} B)",
                parsed.len(),
                bytes.len()
            );
        }
        None => {
            eprintln!("{out}: roundtrip parse void");
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_release(HOST, &out) {
        std::process::exit(1);
    }
}
