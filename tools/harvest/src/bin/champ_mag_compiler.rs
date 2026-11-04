use omegaflow::archivar::cdf2::Cdf2File;
use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::geo::{GeoRec, MAGIC_CHAMPM, write_bin};
use omegaflow::archivar::units::days_to_ymd;
use omegaflow::cdn::upload_release;
use omegaflow::lsk::days_from_civil;
use omegaflow::spectral::SPECTRAL_NO_BAND;

const NETLOC: &str = "isdc-data.gfz.de";
const TEMPLATE: &str =
    "https://isdc-data.gfz.de/champ/ME/Level3/MAG/V0101/{year}/CH-ME-3-MAG+{date}.cdf";

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

fn column(file: &Cdf2File, bytes: &[u8], name: &str, idx: usize) -> Vec<(usize, f64)> {
    let Some(var) = file.var(name) else {
        return Vec::new();
    };
    match file.var_records(bytes, var) {
        Ok(records) => records
            .into_iter()
            .filter_map(|(r, v)| v.get(idx).copied().map(|x| (r, x)))
            .collect(),
        Err(_) => Vec::new(),
    }
}

fn ingest_cdf(bytes: &[u8]) -> Vec<GeoRec> {
    let Ok(file) = Cdf2File::parse(bytes) else {
        return Vec::new();
    };
    let Ok(epochs) = file.epoch_seconds(bytes) else {
        return Vec::new();
    };
    let lat = column(&file, bytes, "GEO_LAT", 0);
    let lon = column(&file, bytes, "GEO_LON", 0);
    let alt = column(&file, bytes, "GEO_ALT", 0);
    let vec_var = match file.var("NEC_VEC") {
        Some(v) => v,
        None => return Vec::new(),
    };
    let Ok(vectors) = file.var_records(bytes, vec_var) else {
        return Vec::new();
    };
    let mut by_lat = std::collections::HashMap::with_capacity(lat.len());
    for (r, v) in lat {
        by_lat.insert(r, v);
    }
    let mut by_lon = std::collections::HashMap::with_capacity(lon.len());
    for (r, v) in lon {
        by_lon.insert(r, v);
    }
    let mut by_alt = std::collections::HashMap::with_capacity(alt.len());
    for (r, v) in alt {
        by_alt.insert(r, v);
    }
    let mut by_vec = std::collections::HashMap::with_capacity(vectors.len());
    for (r, v) in vectors {
        by_vec.insert(r, v);
    }
    let mut out = Vec::with_capacity(epochs.len());
    for (rec, t) in epochs {
        let Some(v) = by_vec.get(&rec) else {
            continue;
        };
        if v.len() < 3 {
            continue;
        }
        let (x, y, z) = (v[0], v[1], v[2]);
        if !(x.is_finite() && y.is_finite() && z.is_finite()) {
            continue;
        }
        let magnitude = (x * x + y * y + z * z).sqrt();
        if !magnitude.is_finite() || magnitude <= 0.0 {
            continue;
        }
        let (Some(&la), Some(&lo), Some(&al)) =
            (by_lat.get(&rec), by_lon.get(&rec), by_alt.get(&rec))
        else {
            continue;
        };
        if !(-90.0..=90.0).contains(&la) || !(-360.0..=360.0).contains(&lo) {
            continue;
        }
        out.push(GeoRec {
            t,
            lat: la,
            lon: lo,
            alt: al,
            freq: SPECTRAL_NO_BAND,
            bin_width: 1.0,
            val: magnitude,
            comp: 0,
            station: 0,
        });
    }
    out
}

fn selftest() {
    // A synthetic CDF 2.6 header is not a small fixture; the reader is checked
    // against its own magic rules here and against a real GFZ file by --input.
    assert!(Cdf2File::parse(b"CDF30001\x00\x00\xff\xff").is_err());
    assert_eq!(MAGIC_CHAMPM, *b"CHM1");
    println!("champ_mag_compiler selftest: foreign magic refused, bin magic CHM1");
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--selftest") {
        selftest();
        return;
    }
    omegaflow::archivar::cdn::require_ci();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = match arg_value(&args, "--out") {
        Some(v) => v,
        None => {
            eprintln!("champ_mag_compiler: --out <path> absent — the output path is never silent");
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

    if let Some(path) = input {
        let bytes = match std::fs::read(&path) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("champ_mag_compiler: read {path}: {e} — the file stays unread");
                std::process::exit(1);
            }
        };
        records = ingest_cdf(&bytes);
        fetched = 1;
    } else if let Some(route) = url {
        let Some(bytes) = fetch_raw_bytes(&route) else {
            eprintln!("champ_mag_compiler: fetch void ({route})");
            std::process::exit(1);
        };
        records = ingest_cdf(&bytes);
        fetched = 1;
    } else {
        let (sy, sm, sd, ey, em, ed) = match (&start, &end, &year) {
            (Some(s), Some(e), _) => {
                let Some(sp) = parse_ymd(s) else {
                    eprintln!("champ_mag_compiler: --start {s} reads void — want YYYY-MM-DD");
                    std::process::exit(1);
                };
                let Some(ep) = parse_ymd(e) else {
                    eprintln!("champ_mag_compiler: --end {e} reads void — want YYYY-MM-DD");
                    std::process::exit(1);
                };
                (sp.0, sp.1, sp.2, ep.0, ep.1, ep.2)
            }
            (None, None, Some(y)) => {
                let Ok(yv) = y.parse::<i64>() else {
                    eprintln!("champ_mag_compiler: --year {y} reads void — want YYYY");
                    std::process::exit(1);
                };
                (yv, 1, 1, yv, 12, 31)
            }
            (Some(_), None, _) | (None, Some(_), _) => {
                eprintln!("champ_mag_compiler: --start and --end belong together — one alone is refused");
                std::process::exit(1);
            }
            _ => {
                eprintln!("champ_mag_compiler: --year <YYYY> or --start/--end <YYYY-MM-DD> or --input/--url absent — refused");
                std::process::exit(1);
            }
        };
        let (Some(start_days), Some(end_days)) =
            (days_from_civil(sy, sm, sd), days_from_civil(ey, em, ed))
        else {
            eprintln!("champ_mag_compiler: the window reads void");
            std::process::exit(1);
        };
        if end_days < start_days {
            eprintln!("champ_mag_compiler: --end precedes --start — refused");
            std::process::exit(1);
        }
        let mut day = start_days;
        while day <= end_days {
            let (y, m, d) = days_to_ymd(day as u64);
            let date = format!("{y:04}-{m:02}-{d:02}");
            let route = TEMPLATE
                .replace("{year}", &format!("{y:04}"))
                .replace("{date}", &date);
            match fetch_raw_bytes(&route) {
                Some(bytes) => {
                    fetched += 1;
                    records.extend(ingest_cdf(&bytes));
                }
                None => skipped += 1,
            }
            day += 1;
        }
    }

    if records.is_empty() {
        eprintln!("champ_mag_compiler: no record carries a magnetic field — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
    records.sort_by(|a, b| a.t.total_cmp(&b.t));
    let bytes = write_bin(MAGIC_CHAMPM, &records);
    std::fs::write(&out, &bytes).unwrap_or_else(|e| {
        eprintln!("champ_mag_compiler: write {out}: {e}");
        std::process::exit(1);
    });
    eprintln!(
        "champ_mag_compiler: {fetched} file(s), {skipped} absent, {} records, {} bytes",
        records.len(),
        bytes.len()
    );
    if ci_mode {
        upload_release(NETLOC, &out);
    }
}
