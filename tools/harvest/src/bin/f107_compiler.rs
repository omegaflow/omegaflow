use omegaflow::archivar::f107::{parse_bin, write_bin};
use omegaflow::archivar::fetch_raw;
use omegaflow::cdn::upload_asset;
use omegaflow::lsk::days_from_civil;
use omegaflow::spectral::civil_from_days;

const BASE: &str = "https://www.ngdc.noaa.gov/stp/space-weather/solar-data/solar-features/solar-radio/noontime-flux/penticton";
const FIRST_YEAR: i64 = 1947;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn parse_line(line: &str, year: i64) -> Option<(i64, f64)> {
    let mut it = line.split_whitespace();
    let date = it.next()?;
    let station = it.next()?;
    if station != "PENT" {
        return None;
    }
    let value: f64 = it.next()?.parse().ok()?;
    if !value.is_finite() || value <= 0.0 {
        return None;
    }
    if date.len() != 6 || !date.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    let month: i64 = date[2..4].parse().ok()?;
    let day: i64 = date[4..6].parse().ok()?;
    let days = days_from_civil(year, month, day)?;
    Some((days, value * 1e-22))
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = arg_value(&args, "--out").unwrap_or_else(|| "f107_penticton.bin".to_string());
    let now_unix = match std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
    {
        Ok(u) => u,
        Err(_) => {
            eprintln!("the clock stays unread — the harvest window is undeclared");
            std::process::exit(1);
        }
    };
    let current_year = match civil_from_days(now_unix / 86400) {
        Some((y, _, _)) => y as i64,
        None => {
            eprintln!("civil_from_days returned void — the current year stays unread");
            std::process::exit(1);
        }
    };
    let mut records: Vec<(i64, f64)> = Vec::new();
    let mut skipped_lines = 0usize;
    let mut void_years = 0usize;
    for year in FIRST_YEAR..=current_year {
        let url = format!("{}/pent_noontime-flux_{}.txt", BASE, year);
        let Some(body) = fetch_raw(&url, None, &[], 86400) else {
            eprintln!("pent_noontime-flux_{}.txt returned void", year);
            void_years += 1;
            continue;
        };
        for line in body.lines() {
            let Some(record) = parse_line(line, year) else {
                skipped_lines += 1;
                continue;
            };
            records.push(record);
        }
    }
    records.sort_by(|a, b| a.0.cmp(&b.0));
    records.dedup_by(|a, b| a.0 == b.0);
    if records.is_empty() {
        eprintln!("no valid records — the series stays unwritten (0 honored)");
        std::process::exit(1);
    }
    let bytes = write_bin(&records);
    if std::fs::write(&out, &bytes).is_err() {
        eprintln!("write {} returned void", out);
        std::process::exit(1);
    }
    match parse_bin(&bytes) {
        Some(parsed) => {
            let (d0, v0) = parsed[0];
            let (d1, v1) = parsed[parsed.len() - 1];
            let span = |d: i64| -> String {
                match civil_from_days(d) {
                    Some((y, m, dd)) => format!("{}-{:02}-{:02}", y, m, dd),
                    None => format!("day {}", d),
                }
            };
            eprintln!(
                "{}: {} records ({}..{}), flux {:.0}..{:.0} sfu, {} skipped lines, {} void years — roundtrip parses ({} B)",
                out,
                parsed.len(),
                span(d0),
                span(d1),
                v1 / 1e-22,
                v0 / 1e-22,
                skipped_lines,
                void_years,
                bytes.len()
            );
        }
        None => {
            eprintln!(
                "{}: roundtrip parse void — the series stays unverified",
                out
            );
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_asset(&out) {
        std::process::exit(1);
    }
}
