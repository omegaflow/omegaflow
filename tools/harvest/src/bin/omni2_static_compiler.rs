use omegaflow::archivar::omni2::{parse_bin, write_bin, COMP_AE, COMP_AL, COMP_AU, COMP_SYMH};
use omegaflow::cdn::upload_asset;
use omegaflow::lsk::days_from_civil;
use std::process::Command;

const BASE: &str = "https://spdf.gsfc.nasa.gov/pub/data/omni";
const FILL_AE_H: f64 = 9999.0;
const FILL_I6: f64 = 99999.0;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn fetch(url: &str) -> Option<String> {
    let out = Command::new("curl")
        .arg("-sSf")
        .arg("--retry")
        .arg("2")
        .arg("--max-time")
        .arg("300")
        .arg(url)
        .output()
        .ok()?;
    if out.status.success() {
        String::from_utf8(out.stdout).ok()
    } else {
        eprintln!(
            "fetch http {}: {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        None
    }
}

fn epoch_hourly(year: i64, doy: i64, hour: i64) -> Option<f64> {
    let days = days_from_civil(year, 1, 1)? + (doy - 1);
    Some(days as f64 * 86400.0 + hour as f64 * 3600.0)
}

fn epoch_minute(year: i64, doy: i64, hour: i64, minute: i64) -> Option<f64> {
    epoch_hourly(year, doy, hour).map(|e| e + minute as f64 * 60.0)
}

fn keep(v: f64, fill: f64) -> bool {
    v.is_finite() && v != fill
}

fn token_i64(tokens: &[&str], i: usize) -> Option<i64> {
    tokens.get(i)?.trim().parse().ok()
}

fn token_f64(tokens: &[&str], i: usize) -> Option<f64> {
    tokens.get(i)?.trim().parse().ok()
}

fn parse_hourly(line: &str) -> Vec<(f64, f64, u32)> {
    let tokens: Vec<&str> = line.split_whitespace().collect();
    if tokens.len() < 54 {
        return Vec::new();
    }
    let (Some(year), Some(doy), Some(hour)) = (
        token_i64(&tokens, 0),
        token_i64(&tokens, 1),
        token_i64(&tokens, 2),
    ) else {
        return Vec::new();
    };
    let Some(t) = epoch_hourly(year, doy, hour) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    if let Some(ae) = token_f64(&tokens, 41) {
        if keep(ae, FILL_AE_H) {
            out.push((t, ae, COMP_AE));
        }
    }
    if let Some(al) = token_f64(&tokens, 52) {
        if keep(al, FILL_I6) {
            out.push((t, al, COMP_AL));
        }
    }
    if let Some(au) = token_f64(&tokens, 53) {
        if keep(au, FILL_I6) {
            out.push((t, au, COMP_AU));
        }
    }
    out
}

fn parse_minute(line: &str) -> Vec<(f64, f64, u32)> {
    let tokens: Vec<&str> = line.split_whitespace().collect();
    if tokens.len() < 44 {
        return Vec::new();
    }
    let (Some(year), Some(doy), Some(hour), Some(minute)) = (
        token_i64(&tokens, 0),
        token_i64(&tokens, 1),
        token_i64(&tokens, 2),
        token_i64(&tokens, 3),
    ) else {
        return Vec::new();
    };
    let Some(t) = epoch_minute(year, doy, hour, minute) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    if let Some(symh) = token_f64(&tokens, 41) {
        if keep(symh, FILL_I6) {
            out.push((t, symh, COMP_SYMH));
        }
    }
    out
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = match arg_value(&args, "--out") {
        Some(p) => p,
        None => omegaflow::archivar::cache_root()
            .join("omni2_indices.bin")
            .to_string_lossy()
            .into_owned(),
    };
    let start_year: i64 = arg_value(&args, "--window-start")
        .and_then(|v| v.parse().ok())
        .unwrap_or(1995);
    let end_year: i64 = arg_value(&args, "--window-end")
        .and_then(|v| v.parse().ok())
        .unwrap_or(2026);
    let symh = args.iter().any(|a| a == "--symh");
    if start_year > end_year || start_year < 1963 {
        eprintln!("--window-start/--window-end carry no valid span");
        std::process::exit(1);
    }
    let mut raw: Vec<(f64, f64, u32)> = Vec::new();
    for year in start_year..=end_year {
        let url = format!("{BASE}/low_res_omni/omni2_{year}.dat");
        let Some(text) = fetch(&url) else {
            eprintln!("year {year}: fetch void — the year stays unharvested");
            continue;
        };
        let mut year_rows = 0usize;
        for line in text.lines() {
            let parsed = parse_hourly(line);
            if !parsed.is_empty() {
                year_rows += 1;
                raw.extend(parsed);
            }
        }
        eprintln!("year {year}: {} hourly rows", year_rows);
    }
    if symh {
        for year in start_year..=end_year {
            for month in 1..=12 {
                let url = format!("{BASE}/high_res_omni/monthly_1min/omni_min{year}{month:02}.asc");
                let Some(text) = fetch(&url) else {
                    continue;
                };
                let mut rows = 0usize;
                for line in text.lines() {
                    let parsed = parse_minute(line);
                    if !parsed.is_empty() {
                        rows += 1;
                        raw.extend(parsed);
                    }
                }
                eprintln!("{year}-{month:02}: {rows} minute rows");
            }
        }
    }
    if raw.is_empty() {
        eprintln!("no records — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
    raw.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.2.cmp(&b.2)));
    let bytes = write_bin(&raw);
    if let Some(parent) = std::path::Path::new(&out).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if std::fs::write(&out, &bytes).is_err() {
        eprintln!("write {} returned void", out);
        std::process::exit(1);
    }
    match parse_bin(&bytes) {
        Some(parsed) => eprintln!("{}: {} records, roundtrip parses", out, parsed.len()),
        None => {
            eprintln!("{}: roundtrip parse void — the bin stays unverified", out);
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_asset(&out) {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hourly_indices_match_verified_sample() {
        let line = "2025   1  0 2610 51 52  61  26  13.6  12.8 -15.3 129.0  -7.7   9.6  -3.4  10.1  -1.0   0.7   4.6   3.2   2.4   2.4  178561.  19.6  427.  -4.5   5.9 0.029  6.66   45580.   2.3    5.   1.0   1.5 0.002   0.43   1.35   7.0 40 198   -26  287 999999.99 99999.99 99999.99 99999.99 99999.99 99999.99  0  27 211.9 999.9  -187   100  4.7";
        let recs = parse_hourly(line);
        let ae = recs.iter().find(|r| r.2 == COMP_AE).expect("AE present");
        let al = recs.iter().find(|r| r.2 == COMP_AL).expect("AL present");
        let au = recs.iter().find(|r| r.2 == COMP_AU).expect("AU present");
        assert!((ae.1 - 287.0).abs() < 1e-6, "AE must be 287, got {}", ae.1);
        assert!(
            (al.1 - -187.0).abs() < 1e-6,
            "AL must be -187, got {}",
            al.1
        );
        assert!((au.1 - 100.0).abs() < 1e-6, "AU must be 100, got {}", au.1);
        let expect_t = epoch_hourly(2025, 1, 0).unwrap();
        assert!(
            (ae.0 - expect_t).abs() < 1e-6,
            "epoch must be 2025 day1 hour0"
        );
    }

    #[test]
    fn minute_symh_matches_verified_sample() {
        let line = "2026 213  0  0 51 99   4 999  75   1267    137  0.02     11    4.94   -3.18    2.79    2.54    3.04    2.23    0.09    0.24 99999.9 99999.9 99999.9 99999.9 999.99 9999999. 99.99 999.99 999.99 999.9 9999.99 9999.99 9999.99   14.45   -0.66    0.36    69   -51    18     3     0     4    20 999.99 99.9";
        let recs = parse_minute(line);
        let symh = recs
            .iter()
            .find(|r| r.2 == COMP_SYMH)
            .expect("SYM-H present");
        assert!(
            (symh.1 - 0.0).abs() < 1e-6,
            "SYM-H must be 0, got {}",
            symh.1
        );
        let expect_t = epoch_minute(2026, 213, 0, 0).unwrap();
        assert!(
            (symh.0 - expect_t).abs() < 1e-6,
            "epoch must be 2026 doy213 hour0 min0"
        );
    }

    #[test]
    fn fill_values_are_skipped() {
        let hourly = "2025   1  0 2610 51 52  61  26  13.6  12.8 -15.3 129.0  -7.7   9.6  -3.4  10.1  -1.0   0.7   4.6   3.2   2.4   2.4  178561.  19.6  427.  -4.5   5.9 0.029  6.66   45580.   2.3    5.   1.0   1.5 0.002   0.43   1.35   7.0 40 198   -26 9999 999999.99 99999.99 99999.99 99999.99 99999.99 99999.99  0  27 211.9 999.9 99999 99999  4.7";
        let recs = parse_hourly(hourly);
        assert!(
            recs.iter()
                .all(|r| r.2 != COMP_AE && r.2 != COMP_AL && r.2 != COMP_AU),
            "fill rows must carry no index records"
        );
    }
}
