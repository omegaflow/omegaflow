use omegaflow::archivar::{cache_root, fetch_raw, parse_json, scalar_of, JsonVal};
use omegaflow::cdn::upload_release;
use std::time::{SystemTime, UNIX_EPOCH};

const STATION_HAPI: &str =
    "https://imag-data.bgs.ac.uk/GIN_V1/hapi/data?id={station}/best-avail/PT1M/xyzf";
const CDN_TAG: &str = "imag-data.bgs.ac.uk";
const FILL_NT: f64 = 99999.0;
const MINUTE: f64 = 60.0;
const HOUR: f64 = 3600.0;
const DAY: f64 = 86400.0;
const MAGIC: &[u8; 4] = b"IMDT";

fn now_unix() -> f64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs_f64())
        .unwrap_or(0.0)
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
    Some((jdn - 2440588) as f64 * DAY + hh as f64 * HOUR + mm as f64 * MINUTE + ss as f64)
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
    let hh = (day_secs / HOUR) as i64;
    let mm = ((day_secs - hh as f64 * HOUR) / MINUTE) as i64;
    let ss = (day_secs - hh as f64 * HOUR - mm as f64 * MINUTE) as i64;
    format!("{y:04}-{m:02}-{d:02}T{hh:02}:{mm:02}:{ss:02}Z")
}

fn arg_value(args: &[String], key: &str) -> Option<String> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn harvest_year_buckets(station: &str, year: i64, bucket_s: f64) -> Vec<(f64, f64)> {
    let now = now_unix();
    let mut peaks: Vec<(f64, f64)> = Vec::new();
    let mut bucket_epoch = 0.0f64;
    let mut bucket_peak = f64::NEG_INFINITY;
    let mut prev: Option<(f64, [f64; 3])> = None;
    for month in 1..=12i64 {
        let (ny, nm) = if month == 12 {
            (year + 1, 1)
        } else {
            (year, month + 1)
        };
        let start = format!("{year:04}-{month:02}-01T00:00:00Z");
        let mut stop = format!("{ny:04}-{nm:02}-01T00:00:00Z");
        if iso_to_unix(&stop).unwrap_or(0.0) > now - 2.0 * HOUR {
            stop = iso_utc(now - 2.0 * HOUR);
        }
        if iso_to_unix(&stop).unwrap_or(0.0) <= iso_to_unix(&start).unwrap_or(0.0) {
            continue;
        }
        let url = format!(
            "{}&start={start}&stop={stop}&format=json",
            STATION_HAPI.replace("{station}", station)
        );
        let mut root_json: Option<JsonVal> = None;
        for attempt in 0..3 {
            if attempt > 0 {
                std::thread::sleep(std::time::Duration::from_secs(20));
            }
            match fetch_raw(&url, None, &[], 600).and_then(|b| parse_json(&b)) {
                Some(j) => {
                    root_json = Some(j);
                    break;
                }
                None => eprintln!(
                    "{station} {year}-{month:02}: fetch void (attempt {})",
                    attempt + 1
                ),
            }
        }
        let Some(j) = root_json else {
            continue;
        };
        let JsonVal::Obj(root) = j else {
            continue;
        };
        let Some(JsonVal::Arr(data)) = root.get("data") else {
            continue;
        };
        for row in data {
            let JsonVal::Arr(cells) = row else {
                continue;
            };
            let Some(t) = cells.first().and_then(|c| match c {
                JsonVal::Str(s) => iso_to_unix(s),
                _ => None,
            }) else {
                continue;
            };
            let JsonVal::Arr(vec) = &cells[1] else {
                continue;
            };
            let cx = vec.get(0).and_then(scalar_of);
            let cy = vec.get(1).and_then(scalar_of);
            let cz = vec.get(2).and_then(scalar_of);
            let comp = match (cx, cy, cz) {
                (Some(a), Some(b), Some(c))
                    if a.is_finite()
                        && b.is_finite()
                        && c.is_finite()
                        && a != FILL_NT
                        && b != FILL_NT
                        && c != FILL_NT =>
                {
                    Some((a, b, c))
                }
                _ => None,
            };
            let Some((vx, vy, vz)) = comp else {
                prev = None;
                continue;
            };
            if let Some((pt, [px, py, pz])) = prev {
                let dt = t - pt;
                if (MINUTE - 2.0..=MINUTE + 2.0).contains(&dt) {
                    let dx = vx - px;
                    let dy = vy - py;
                    let dz = vz - pz;
                    let dbdt = (dx * dx + dy * dy + dz * dz).sqrt();
                    let this_bucket = (t / bucket_s).floor() * bucket_s;
                    if this_bucket != bucket_epoch {
                        if bucket_peak.is_finite() {
                            peaks.push((bucket_epoch, bucket_peak));
                        }
                        bucket_epoch = this_bucket;
                        bucket_peak = f64::NEG_INFINITY;
                    }
                    if dbdt > bucket_peak {
                        bucket_peak = dbdt;
                    }
                }
            }
            prev = Some((t, [vx, vy, vz]));
        }
    }
    if bucket_peak.is_finite() {
        peaks.push((bucket_epoch, bucket_peak));
    }
    peaks
}

fn write_bin(records: &[(f64, f64)]) -> Vec<u8> {
    let mut out = Vec::with_capacity(8 + records.len() * 16);
    out.extend_from_slice(MAGIC);
    out.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for (t, v) in records {
        out.extend_from_slice(&t.to_le_bytes());
        out.extend_from_slice(&v.to_le_bytes());
    }
    out
}

fn parse_bin(data: &[u8]) -> Option<Vec<(f64, f64)>> {
    if data.len() < 8 || &data[0..4] != MAGIC {
        return None;
    }
    let count = u32::from_le_bytes(data[4..8].try_into().ok()?) as usize;
    if data.len() != 8 + count * 16 {
        return None;
    }
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        let base = 8 + i * 16;
        let t = f64::from_le_bytes(data[base..base + 8].try_into().ok()?);
        let v = f64::from_le_bytes(data[base + 8..base + 16].try_into().ok()?);
        out.push((t, v));
    }
    Some(out)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let station = arg_value(&args, "--station").unwrap_or_else(|| {
        eprintln!("--station (INTERMAGNET code, e.g. ABK) required");
        std::process::exit(1);
    });
    let grain = arg_value(&args, "--grain").unwrap_or_else(|| "hourly".to_string());
    let bucket_s = match grain.as_str() {
        "hourly" => HOUR,
        "daily" => DAY,
        _ => {
            eprintln!("--grain hourly|daily");
            std::process::exit(1);
        }
    };
    let sy: i64 = arg_value(&args, "--start")
        .and_then(|v| v.parse().ok())
        .unwrap_or(1994);
    let ey: i64 = arg_value(&args, "--end")
        .and_then(|v| v.parse().ok())
        .unwrap_or_else(|| {
            let n = now_unix();
            let total = (n / DAY).floor() as i64;
            let z = total + 719468;
            let era = if z >= 0 { z } else { z - 146096 } / 146097;
            let doe = z - era * 146097;
            let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
            yoe + era * 400
        });
    let out = arg_value(&args, "--out")
        .unwrap_or_else(|| format!("{}_dbdt_{grain}.bin", station.to_lowercase()));

    let mut all: Vec<(f64, f64)> = Vec::new();
    for year in sy..=ey {
        let rows = harvest_year_buckets(&station, year, bucket_s);
        eprintln!("{station} {year}: {} {grain} cells", rows.len());
        all.extend(rows);
    }
    all.sort_by(|a, b| a.0.total_cmp(&b.0));
    all.dedup_by(|a, b| a.0 == b.0);
    if all.is_empty() {
        eprintln!(
            "{}: no records — the bin stays unwritten (0 honored)",
            station
        );
        std::process::exit(1);
    }
    let bytes = write_bin(&all);
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
    let cache = cache_root().join(&out);
    if cache != std::path::Path::new(&out) {
        let _ = std::fs::copy(&out, &cache);
    }
    if ci_mode && !upload_release(CDN_TAG, &out) {
        std::process::exit(1);
    }
    eprintln!(
        "{station}: {} {grain} dB/dt cells ({sy}..{ey}) written — the measurement series is flat, not a field channel (Rat 2026-09-04)",
        all.len()
    );
}
