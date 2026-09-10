use omegaflow::archivar::geo::{
    parse_bin, write_bin, GeoRec, COMP_SMG_E_GEO, COMP_SMG_E_NEZ, COMP_SMG_N_GEO, COMP_SMG_N_NEZ,
    COMP_SMG_Z_GEO, COMP_SMG_Z_NEZ, MAGIC_SMG,
};
use omegaflow::archivar::{fetch_raw, jpath, parse_json, JsonVal};
use omegaflow::cdn::upload_release;
use omegaflow::lsk::{parse as parse_lsk, LeapSeconds};

const NETLOC: &str = "supermag.jhuapl.edu";
const DATA_API: &str = "https://supermag.jhuapl.edu/services/data-api.php";
const MAGSTID: &str = "https://supermag.jhuapl.edu/lib/php/magstid.php";
const LOGON: &str = "omegaflow";
const FILL_NT: f64 = 999999.0;
const CHUNK_S: f64 = 2419200.0;
const DAY: f64 = 86400.0;

fn arg_value(args: &[String], key: &str) -> Option<String> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn iso_to_unix(s: &str) -> Option<f64> {
    let s = s.trim();
    let (date, time) = if let Some((d, t)) = s.split_once('T') {
        (d, t)
    } else {
        s.split_once(' ')?
    };
    let mut dp = date.split('-');
    let y: i64 = dp.next()?.parse().ok()?;
    let m: i64 = dp.next()?.parse().ok()?;
    let d: i64 = dp.next()?.parse().ok()?;
    let t = time
        .split(|c: char| c == '.' || c == 'Z' || c == 'z')
        .next()?;
    let mut tp = t.split(':');
    let hh: i64 = tp.next()?.parse().ok()?;
    let mm: i64 = match tp.next() {
        Some(v) => v,
        None => "0",
    }
    .parse()
    .ok()?;
    let ss: i64 = match tp.next() {
        Some(v) => v,
        None => "0",
    }
    .parse()
    .ok()?;
    let a = (14 - m) / 12;
    let yy = y + 4800 - a;
    let jdn =
        d + (153 * (m + 12 * a - 3) + 2) / 5 + 365 * yy + yy / 4 - yy / 100 + yy / 400 - 32045;
    Some((jdn - 2440588) as f64 * DAY + hh as f64 * 3600.0 + mm as f64 * 60.0 + ss as f64)
}

fn iso_utc(unix: f64) -> String {
    let total = (unix.max(0.0) / DAY).floor() as i64;
    let day_secs = unix.max(0.0) - total as f64 * DAY;
    let z = total + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    let hh = (day_secs / 3600.0) as i64;
    let mm = ((day_secs - hh as f64 * 3600.0) / 60.0) as i64;
    let ss = (day_secs - hh as f64 * 3600.0 - mm as f64 * 60.0) as i64;
    format!("{y:04}-{m:02}-{d:02}T{hh:02}:{mm:02}:{ss:02}Z")
}

fn num_after(s: &str, key: &str) -> Option<f64> {
    let i = s.find(key)? + key.len();
    let tail = &s[i..];
    let j = tail.find(|c: char| c == ',' || c == '}' || c == '\n')?;
    tail[..j].trim().parse().ok()
}

fn station_position(code: &str) -> Option<(f64, f64)> {
    let text = fetch_raw(MAGSTID, None, &[], 3600)?;
    let needle = format!("id:\"{code}\"");
    let start = text.find(&needle)?;
    let region = &text[start..];
    let lat = num_after(region, "geolat:")?;
    let lon_raw = num_after(region, "geolon:")?;
    let lon = if lon_raw > 180.0 {
        lon_raw - 360.0
    } else {
        lon_raw
    };
    if lat.is_finite() && lon.is_finite() {
        Some((lat, lon))
    } else {
        None
    }
}

fn component_value(record: &JsonVal, path: &str) -> Option<f64> {
    match jpath(record, path) {
        Some(x) if x.is_finite() && (x - FILL_NT).abs() > 1.0 => Some(x),
        _ => None,
    }
}

fn fetch_chunk(start: &str, extent: f64, station: &str) -> Option<Vec<JsonVal>> {
    let url = format!(
        "{DATA_API}?fmt=json&logon={LOGON}&start={start}&extent={extent:.0}&all&station={station}"
    );
    for attempt in 0..3 {
        if attempt > 0 {
            std::thread::sleep(std::time::Duration::from_secs(15));
        }
        let Some(text) = fetch_raw(&url, None, &[], 600) else {
            continue;
        };
        let Some(JsonVal::Arr(records)) = parse_json(&text) else {
            return None;
        };
        if records.is_empty() {
            return None;
        }
        return Some(records);
    }
    None
}

fn compile_window(station: &str, start_unix: f64, days: f64, lsk: &LeapSeconds) -> Vec<GeoRec> {
    let (lat, lon) = match station_position(station) {
        Some(p) => p,
        None => {
            eprintln!("{station}: magstid.php carries no position — the bin stays unwritten");
            return Vec::new();
        }
    };
    let mut out: Vec<GeoRec> = Vec::new();
    let mut cursor = start_unix;
    let end = start_unix + days * DAY;
    while cursor < end {
        let extent = (end - cursor).min(CHUNK_S);
        let chunk_start = iso_utc(cursor);
        let Some(records) = fetch_chunk(&chunk_start, extent, station) else {
            eprintln!("{station} {chunk_start}: chunk void — no records flow from the API");
            cursor += extent;
            continue;
        };
        for rec in &records {
            let Some(tval) = jpath(rec, "tval") else {
                continue;
            };
            if !tval.is_finite() {
                continue;
            }
            let Some(bin_width) = jpath(rec, "ext") else {
                continue;
            };
            if !(bin_width > 0.0) || !bin_width.is_finite() {
                continue;
            }
            let Some(tdb) = lsk.unix_to_tdb(tval) else {
                continue;
            };
            let components = [
                (COMP_SMG_N_NEZ, "N.nez"),
                (COMP_SMG_E_NEZ, "E.nez"),
                (COMP_SMG_Z_NEZ, "Z.nez"),
                (COMP_SMG_N_GEO, "N.geo"),
                (COMP_SMG_E_GEO, "E.geo"),
                (COMP_SMG_Z_GEO, "Z.geo"),
            ];
            for (comp, path) in components {
                let Some(val) = component_value(rec, path) else {
                    continue;
                };
                out.push(GeoRec {
                    t: tdb,
                    lat,
                    lon,
                    alt: 0.0,
                    freq: 0.0,
                    bin_width,
                    val,
                    comp,
                });
            }
        }
        cursor += extent;
    }
    out.sort_by(|a, b| a.t.total_cmp(&b.t).then_with(|| a.comp.cmp(&b.comp)));
    out
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let station = match arg_value(&args, "--station") {
        Some(v) => v,
        None => {
            eprintln!("--station (IAGA code, e.g. TRO) required");
            std::process::exit(1);
        }
    };
    let start = match arg_value(&args, "--start") {
        Some(v) => v,
        None => {
            eprintln!("--start (ISO, e.g. 2025-03-01T00:00:00) required");
            std::process::exit(1);
        }
    };
    let start_unix = match iso_to_unix(&start) {
        Some(u) => u,
        None => {
            eprintln!("--start {start} parses void");
            std::process::exit(1);
        }
    };
    let days: f64 = arg_value(&args, "--days")
        .and_then(|v| v.parse().ok())
        .unwrap_or(28.0);
    if !(days > 0.0) || !days.is_finite() {
        eprintln!("--days {days} carries no positive window");
        std::process::exit(1);
    }
    let out = match arg_value(&args, "--out") {
        Some(v) => v,
        None => {
            eprintln!("--out <path> required");
            std::process::exit(1);
        }
    };
    let lsk = match arg_value(&args, "--lsk")
        .and_then(|p| std::fs::read_to_string(p).ok())
        .and_then(|t| parse_lsk(&t))
    {
        Some(l) => l,
        None => {
            eprintln!("--lsk absent — the TDB conversion stays void (no fabricated epoch)");
            std::process::exit(1);
        }
    };

    let records = compile_window(&station, start_unix, days, &lsk);
    if records.is_empty() {
        eprintln!("{station}: no measured nT records — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
    let bytes = write_bin(MAGIC_SMG, &records);
    if let Some(parent) = std::path::Path::new(&out).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if std::fs::write(&out, &bytes).is_err() {
        eprintln!("write {} returned void", out);
        std::process::exit(1);
    }
    match parse_bin(MAGIC_SMG, &bytes) {
        Some(parsed) => eprintln!(
            "{station}: {} geo records ({:.0} d window, {} B) written, roundtrip parses",
            parsed.len(),
            days,
            bytes.len()
        ),
        None => {
            eprintln!("{}: roundtrip parse void — the bin stays unverified", out);
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_release(NETLOC, &out) {
        std::process::exit(1);
    }
}
