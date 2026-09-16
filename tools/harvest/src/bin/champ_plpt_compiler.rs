use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::geo::{COMP_CHAMP_DENS, GeoRec, MAGIC_CHAMP, verify_bin, write_bin};
use omegaflow::archivar::units::days_to_ymd;
use omegaflow::cdn::upload_release;
use omegaflow::inflate::unzip;
use omegaflow::lsk::days_from_civil;
use omegaflow::rinex::ecef_to_geodetic;
use omegaflow::spectral::SPECTRAL_NO_BAND;

const NETLOC: &str = "isdc-data.gfz.de";
const TEMPLATE: &str =
    "https://isdc-data.gfz.de/champ/ME/Level2/PLPT/{year}/CH-ME-2-PLPT+{date}_1.zip";
const CADENCE_S: f64 = 15.0;
const DAY_S: f64 = 86400.0;
const TTL: u64 = 86400;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn parse_ymd(s: &str) -> Option<(i64, i64, i64)> {
    let b = s.as_bytes();
    if b.len() != 10 || b[4] != b'-' || b[7] != b'-' {
        return None;
    }
    let y = s.get(0..4)?.parse::<i64>().ok()?;
    let m = s.get(5..7)?.parse::<i64>().ok()?;
    let d = s.get(8..10)?.parse::<i64>().ok()?;
    Some((y, m, d))
}

fn parse_plpt_dat(bytes: &[u8]) -> Vec<GeoRec> {
    let text = String::from_utf8_lossy(bytes);
    let mut out = Vec::new();
    for line in text.lines() {
        let t = line.trim();
        if t.is_empty() || t.starts_with('#') {
            continue;
        }
        let cols: Vec<&str> = t.split_whitespace().collect();
        if cols.len() < 11 {
            continue;
        }
        let (Ok(y), Ok(m), Ok(d)) = (
            cols[1].parse::<i64>(),
            cols[2].parse::<i64>(),
            cols[3].parse::<i64>(),
        ) else {
            continue;
        };
        let (Ok(hh), Ok(mm), Ok(ss), Ok(radius_km), Ok(lat_gc), Ok(lon), Ok(density)) = (
            cols[4].parse::<f64>(),
            cols[5].parse::<f64>(),
            cols[6].parse::<f64>(),
            cols[7].parse::<f64>(),
            cols[8].parse::<f64>(),
            cols[9].parse::<f64>(),
            cols[10].parse::<f64>(),
        ) else {
            continue;
        };
        let Some(days) = days_from_civil(y, m, d) else {
            continue;
        };
        if !density.is_finite() || density <= 0.0 {
            continue;
        }
        let t_epoch = days as f64 * DAY_S + hh * 3600.0 + mm * 60.0 + ss;
        let r = radius_km * 1000.0;
        let lat_rad = lat_gc.to_radians();
        let lon_rad = lon.to_radians();
        let (lat, lon, alt) = match ecef_to_geodetic(
            r * lat_rad.cos() * lon_rad.cos(),
            r * lat_rad.cos() * lon_rad.sin(),
            r * lat_rad.sin(),
        ) {
            Some(g) => g,
            None => continue,
        };
        out.push(GeoRec {
            t: t_epoch,
            lat,
            lon,
            alt,
            freq: SPECTRAL_NO_BAND,
            bin_width: CADENCE_S,
            val: density,
            comp: COMP_CHAMP_DENS,
            station: 0,
        });
    }
    out
}

fn ingest_zip(bytes: &[u8]) -> Vec<GeoRec> {
    match unzip(bytes) {
        Some(inner) => parse_plpt_dat(&inner),
        None => Vec::new(),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = match arg_value(&args, "--out") {
        Some(v) => v,
        None => {
            eprintln!("champ_plpt_compiler: --out <path> absent — the output path is never silent");
            std::process::exit(1);
        }
    };
    let input = arg_value(&args, "--input");
    let url = arg_value(&args, "--url");
    let year = arg_value(&args, "--year");
    let start = arg_value(&args, "--start");
    let end = arg_value(&args, "--end");

    let mut records: Vec<GeoRec> = Vec::new();
    let mut fetched = 0usize;
    let mut skipped = 0usize;
    let range_mode = input.is_none() && url.is_none();

    if let Some(path) = input {
        let bytes = match std::fs::read(&path) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("champ_plpt_compiler: read {path}: {e} — the zip stays unread");
                std::process::exit(1);
            }
        };
        records = ingest_zip(&bytes);
        if records.is_empty() {
            eprintln!(
                "champ_plpt_compiler: {path} carries no PLPT rows — the bin stays unwritten (0 honored)"
            );
            std::process::exit(1);
        }
        fetched = 1;
    } else if let Some(route) = url {
        let bytes = match fetch_raw_bytes(&route, TTL) {
            Some(b) => b,
            None => {
                eprintln!("champ_plpt_compiler: fetch void ({route})");
                std::process::exit(1);
            }
        };
        records = ingest_zip(&bytes);
        if records.is_empty() {
            eprintln!(
                "champ_plpt_compiler: {route} carries no PLPT rows — the bin stays unwritten (0 honored)"
            );
            std::process::exit(1);
        }
        fetched = 1;
    } else {
        let (sy, sm, sd, ey, em, ed) = match (&start, &end, &year) {
            (Some(s), Some(e), _) => {
                let (sy, sm, sd) = match parse_ymd(s) {
                    Some(v) => v,
                    None => {
                        eprintln!("champ_plpt_compiler: --start {s} reads void — want YYYY-MM-DD");
                        std::process::exit(1);
                    }
                };
                let (ey, em, ed) = match parse_ymd(e) {
                    Some(v) => v,
                    None => {
                        eprintln!("champ_plpt_compiler: --end {e} reads void — want YYYY-MM-DD");
                        std::process::exit(1);
                    }
                };
                (sy, sm, sd, ey, em, ed)
            }
            (None, None, Some(y)) => {
                let yv = match y.parse::<i64>() {
                    Ok(v) => v,
                    Err(_) => {
                        eprintln!("champ_plpt_compiler: --year {y} reads void — want YYYY");
                        std::process::exit(1);
                    }
                };
                (yv, 1, 1, yv, 12, 31)
            }
            (Some(_), None, _) | (None, Some(_), _) => {
                eprintln!("champ_plpt_compiler: --start and --end belong together — one alone is refused");
                std::process::exit(1);
            }
            _ => {
                eprintln!(
                    "champ_plpt_compiler: --year <YYYY> or --start/--end <YYYY-MM-DD> or --input/--url absent — refused"
                );
                std::process::exit(1);
            }
        };
        let start_days = match days_from_civil(sy, sm, sd) {
            Some(v) => v,
            None => {
                eprintln!("champ_plpt_compiler: --start {sy:04}-{sm:02}-{sd:02} reads void");
                std::process::exit(1);
            }
        };
        let end_days = match days_from_civil(ey, em, ed) {
            Some(v) => v,
            None => {
                eprintln!("champ_plpt_compiler: --end {ey:04}-{em:02}-{ed:02} reads void");
                std::process::exit(1);
            }
        };
        if end_days < start_days {
            eprintln!("champ_plpt_compiler: --end precedes --start — refused");
            std::process::exit(1);
        }
        let mut day = start_days;
        while day <= end_days {
            let (y, m, d) = days_to_ymd(day as u64);
            let date = format!("{y:04}-{m:02}-{d:02}");
            let route = TEMPLATE
                .replace("{year}", &format!("{y:04}"))
                .replace("{date}", &date);
            match fetch_raw_bytes(&route, TTL) {
                Some(bytes) => {
                    fetched += 1;
                    records.extend(ingest_zip(&bytes));
                }
                None => skipped += 1,
            }
            day += 1;
        }
    }

    if records.is_empty() {
        eprintln!(
            "champ_plpt_compiler: {} days fetched, {} absent, no PLPT rows — the bin stays unwritten (0 honored)",
            fetched, skipped
        );
        std::process::exit(1);
    }

    records.sort_by(|a, b| a.t.total_cmp(&b.t));
    let bin = write_bin(MAGIC_CHAMP, &records);
    if let Some(parent) = std::path::Path::new(&out).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if std::fs::write(&out, &bin).is_err() {
        eprintln!("champ_plpt_compiler: write {out} void");
        std::process::exit(1);
    }
    match verify_bin(MAGIC_CHAMP, &bin) {
        Some(n) if range_mode => eprintln!(
            "champ_plpt: {} records, {} B -> {out} ({} days fetched, {} absent, verified {})",
            records.len(),
            bin.len(),
            fetched,
            skipped,
            n
        ),
        Some(n) => eprintln!(
            "champ_plpt: {} records, {} B -> {out} (verified {})",
            records.len(),
            bin.len(),
            n
        ),
        None => {
            eprintln!("champ_plpt_compiler: {out}: verify void — the asset stays unverified");
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_release(NETLOC, &out) {
        eprintln!("champ_plpt_compiler: upload {out} did not reach the CDN");
        std::process::exit(1);
    }
}
