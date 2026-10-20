use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::inflate::{gunzip, tar_members};
use omegaflow::cdn::upload_release;
use omegaflow::lsk::days_from_civil;
use omegaflow::soho_sem::{Record, parse_bin, write_bin};

const DEFAULT_URL: &str =
    "https://spdf.gsfc.nasa.gov/pub/data/soho/celias/ascii/CELIAS_SEM_daily_avg.tar.gz";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn has_flag(args: &[String], name: &str) -> bool {
    args.iter().any(|a| a == name)
}

fn parse_row(line: &str) -> Option<(i64, Record)> {
    if line.trim_start().starts_with(';') || line.trim().is_empty() {
        return None;
    }
    let f: Vec<&str> = line.split_whitespace().collect();
    if f.len() < 16 {
        return None;
    }
    let year: i64 = f[1].parse().ok()?;
    let doy: i64 = f[2].parse().ok()?;
    if !(1..=366).contains(&doy) {
        return None;
    }
    let v = |i: usize| {
        f[i].parse::<f64>()
            .ok()
            .filter(|x| x.is_finite() && *x > 0.0)
    };
    let ch1 = v(3)?;
    let ch2 = v(5)?;
    let ch3 = v(7)?;
    let flux_26_34 = v(14)?;
    let flux_0_1_50 = v(15)?;
    let day = days_from_civil(year, 1, 1)? + doy - 1;
    Some((day, (day, ch1, ch2, ch3, flux_26_34, flux_0_1_50)))
}

fn parse_day_file(text: &str) -> Vec<(i64, Record)> {
    let mut out = Vec::new();
    for line in text.lines() {
        if let Some(rec) = parse_row(line) {
            out.push(rec);
        }
    }
    out
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = has_flag(&args, "--ci-mode");
    let out = match arg_value(&args, "--out") {
        Some(v) => v,
        None => "soho_sem_daily.bin".to_string(),
    };
    let url = match arg_value(&args, "--url") {
        Some(v) => v,
        None => DEFAULT_URL.to_string(),
    };

    let Some(tar_gz) = fetch_raw_bytes(&url) else {
        eprintln!("{url}: fetch void — the series stays unwritten (0 honored)");
        std::process::exit(1);
    };
    let Some(tar) = gunzip(&tar_gz) else {
        eprintln!("{url}: gzip void — the series stays unwritten");
        std::process::exit(1);
    };
    let Some(members) = tar_members(&tar) else {
        eprintln!("{url}: tar void — the series stays unwritten");
        std::process::exit(1);
    };
    eprintln!("SEM archive: {} members", members.len());

    let mut records: Vec<(i64, Record)> = Vec::new();
    let mut files = 0usize;
    for m in &members {
        if !m.name.ends_with("_v4.day") {
            continue;
        }
        let Some(slice) = tar.get(m.start..m.end) else {
            continue;
        };
        let Ok(text) = std::str::from_utf8(slice) else {
            eprintln!("{}: not utf8 — skipped", m.name);
            continue;
        };
        let rows = parse_day_file(text);
        if rows.is_empty() {
            eprintln!("{}: no usable row — skipped", m.name);
            continue;
        }
        files += 1;
        records.extend(rows);
    }
    records.sort_by_key(|(d, _)| *d);
    records.dedup_by_key(|(d, _)| *d);
    eprintln!("SEM daily: {} files, {} records", files, records.len());
    if records.is_empty() {
        eprintln!("no records — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
    let flat: Vec<Record> = records.iter().map(|(_, r)| *r).collect();
    let bytes = write_bin(&flat);
    if std::fs::write(&out, &bytes).is_err() {
        eprintln!("write {out} returned void");
        std::process::exit(1);
    }
    match parse_bin(&bytes) {
        Some(parsed) => eprintln!(
            "{out}: {} records, roundtrip parses ({} B)",
            parsed.len(),
            bytes.len()
        ),
        None => {
            eprintln!("{out}: roundtrip parse void");
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_release("spdf.gsfc.nasa.gov", &out) {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXCERPT: &str = "\
;SEM DAILY AVERAGE FULL SOLAR DISK FLUX at 1 AU
;Column 0-\tJulian date (mid point of sample)
;-----------------------------------------------
2461042.000000 2026   1 162.07 2.03632e+00 11309.39 1.05623e+03 130.91 1.95856e+00 -2.65023e+07 1.43360e+08 5.97408e+03 1.45798e+08 9.74601e-01 1.43652e+10 2.92961e+10
2461043.000000 2026   2 160.91 7.78464e-01 10584.87 5.65563e+02 130.25 6.32968e-01 -2.90399e+07 1.42859e+08 2.04842e+03 1.45774e+08 9.74573e-01 1.42298e+10 2.88664e+10
2461044.000000 2026   3 0.00 0.00000e+00 0.00 0.00000e+00 0.00 0.00000e+00 0.0 0.0 0.0 0.0 0.0 0.0 0.0
";

    #[test]
    fn excerpt_keeps_the_two_valid_days_and_drops_the_zero_row() {
        let rows = parse_day_file(EXCERPT);
        assert_eq!(rows.len(), 2);
        let (day, rec) = rows[0];
        assert_eq!(day, days_from_civil(2026, 1, 1).unwrap());
        assert_eq!(rec.0, day);
        assert!((rec.1 - 162.07).abs() < 1e-9);
        assert!((rec.2 - 11309.39).abs() < 1e-6);
        assert!((rec.3 - 130.91).abs() < 1e-9);
        assert!((rec.4 - 1.43652e10).abs() < 1e4);
        assert!((rec.5 - 2.92961e10).abs() < 1e5);
        assert_eq!(rows[1].0, days_from_civil(2026, 1, 2).unwrap());
    }

    #[test]
    fn parse_row_rejects_a_short_line() {
        assert!(parse_row("2461042.0 2026 1 162.07").is_none());
        assert!(parse_row("; a comment").is_none());
        assert!(parse_row("").is_none());
    }

    #[test]
    fn bin_roundtrip_matches_the_register_fields() {
        let records: Vec<Record> = vec![
            (20454, 162.07, 11309.39, 130.91, 1.43652e10, 2.92961e10),
            (20455, 160.91, 10584.87, 130.25, 1.42298e10, 2.88664e10),
        ];
        let bytes = write_bin(&records);
        assert_eq!(parse_bin(&bytes).unwrap(), records);
    }
}
