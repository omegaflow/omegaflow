use omegaflow::archivar::{JsonVal, cache_root, fetch_raw, parse_json, scalar_of};
use omegaflow::cdn::upload_release;
use omegaflow::intermagnet::{parse_xyz_bin, write_xyz_bin};
use std::time::{SystemTime, UNIX_EPOCH};

const STATION_HAPI: &str =
    "https://imag-data.bgs.ac.uk/GIN_V1/hapi/data?id={station}/best-avail/PT1M/xyzf";
const CDN_TAG: &str = "imag-data.bgs.ac.uk";
const FILL_NT: f64 = 99999.0;
const HOUR: f64 = 3600.0;
const DAY: f64 = 86400.0;

fn now_unix() -> f64 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(d) => d.as_secs_f64(),
        Err(e) => -e.duration().as_secs_f64(),
    }
}

fn iso_to_unix(s: &str) -> Option<f64> {
    let s = s.trim();
    let (date, time) = match s.split_once('T') {
        Some((d, t)) => (d, t),
        None => s.split_once(' ')?,
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
    Some((jdn - 2440588) as f64 * DAY + hh as f64 * HOUR + mm as f64 * 60.0 + ss as f64)
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
    let mm = ((day_secs - hh as f64 * HOUR) / 60.0) as i64;
    let ss = (day_secs - hh as f64 * HOUR - mm as f64 * 60.0) as i64;
    format!("{y:04}-{m:02}-{d:02}T{hh:02}:{mm:02}:{ss:02}Z")
}

fn year_of(unix: f64) -> i64 {
    let total = (unix.max(0.0) / DAY).floor() as i64;
    let z = total + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    if m <= 2 { y + 1 } else { y }
}

const STATION_INFO: &str =
    "https://imag-data.bgs.ac.uk/GIN_V1/hapi/info?id={station}/best-avail/PT1M/xyzf";

fn dataset_year_range(station: &str) -> Option<(i64, i64)> {
    let url = STATION_INFO.replace("{station}", &station.to_lowercase());
    let root = fetch_raw(&url, None, &[]).and_then(|b| parse_json(&b))?;
    let JsonVal::Obj(obj) = root else {
        return None;
    };
    let start = match obj.get("startDate") {
        Some(JsonVal::Str(s)) => iso_to_unix(s),
        _ => None,
    }?;
    let stop = match obj.get("stopDate") {
        Some(JsonVal::Str(s)) => iso_to_unix(s),
        _ => None,
    }?;
    Some((year_of(start), year_of(stop)))
}

fn arg_value(args: &[String], key: &str) -> Option<String> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn harvest_year(station: &str, year: i64) -> Vec<(f64, f64, f64, f64)> {
    let now = now_unix();
    let mut out: Vec<(f64, f64, f64, f64)> = Vec::new();
    for month in 1..=12i64 {
        let (ny, nm) = if month == 12 {
            (year + 1, 1)
        } else {
            (year, month + 1)
        };
        let start = format!("{year:04}-{month:02}-01T00:00:00Z");
        let mut stop = format!("{ny:04}-{nm:02}-01T00:00:00Z");
        if iso_to_unix(&stop).is_none_or(|s| s > now - 2.0 * HOUR) {
            stop = iso_utc(now - 2.0 * HOUR);
        }
        if let Some(s) = iso_to_unix(&stop)
            && iso_to_unix(&start).is_some_and(|a| s <= a)
        {
            continue;
        }
        let url = format!(
            "{}&time.min={start}&time.max={stop}&format=json",
            STATION_HAPI.replace("{station}", &station.to_lowercase())
        );
        let mut root: Option<JsonVal> = None;
        for attempt in 0..3 {
            if attempt > 0 {
                std::thread::sleep(std::time::Duration::from_secs(20));
            }
            if let Some(j) = fetch_raw(&url, None, &[]).and_then(|b| parse_json(&b)) {
                root = Some(j);
                break;
            }
            eprintln!(
                "{station} {year}-{month:02}: fetch void (attempt {})",
                attempt + 1
            );
        }
        let Some(JsonVal::Obj(root)) = root else {
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
            let x = vec.first().and_then(scalar_of);
            let y = vec.get(1).and_then(scalar_of);
            let z = vec.get(2).and_then(scalar_of);
            if let (Some(x), Some(y), Some(z)) = (x, y, z)
                && x.is_finite()
                && y.is_finite()
                && z.is_finite()
                && x != FILL_NT
                && y != FILL_NT
                && z != FILL_NT
            {
                out.push((t, x, y, z));
            }
        }
    }
    out
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let Some(station) = arg_value(&args, "--station") else {
        eprintln!("--station (INTERMAGNET code, e.g. ABK) required");
        std::process::exit(1);
    };
    let Some(sy) = arg_value(&args, "--start").and_then(|v| v.parse::<i64>().ok()) else {
        eprintln!("--start (first year) required — the window is named, never defaulted");
        std::process::exit(2);
    };
    let ey = match arg_value(&args, "--end").and_then(|v| v.parse::<i64>().ok()) {
        Some(v) => v,
        None => year_of(now_unix()),
    };
    let out = match arg_value(&args, "--out") {
        Some(v) => v,
        None => format!("{}_xyz_1m.bin", station.to_lowercase()),
    };
    let (lo, hi) = match dataset_year_range(&station) {
        Some(range) => range,
        None => (sy, ey),
    };
    let (start_year, end_year) = if sy.max(lo) > ey.min(hi) {
        eprintln!(
            "{station}: window {sy}..{ey} lies outside the dataset range {lo}..{hi} — harvesting {lo}..{hi}"
        );
        (lo, hi)
    } else {
        let s = sy.max(lo);
        let e = ey.min(hi);
        if (s, e) != (sy, ey) {
            eprintln!(
                "{station}: window {sy}..{ey} clamped to the dataset range {lo}..{hi} -> {s}..{e}"
            );
        }
        (s, e)
    };
    let mut all: Vec<(f64, f64, f64, f64)> = Vec::new();
    for year in start_year..=end_year {
        let rows = harvest_year(&station, year);
        eprintln!("{station} {year}: {} xyz samples", rows.len());
        all.extend(rows);
    }
    all.sort_by(|a, b| a.0.total_cmp(&b.0));
    all.dedup_by(|a, b| a.0 == b.0);
    if all.is_empty() {
        eprintln!("{station}: no records — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
    let bytes = write_xyz_bin(&all);
    if let Some(parent) = std::path::Path::new(&out).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if std::fs::write(&out, &bytes).is_err() {
        eprintln!("write {out} returned void");
        std::process::exit(1);
    }
    match parse_xyz_bin(&bytes) {
        Some(parsed) => eprintln!("{out}: {} records, roundtrip parses", parsed.len()),
        None => {
            eprintln!("{out}: roundtrip parse void — the bin stays unverified");
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
    eprintln!("{station}: {} xyz samples ({sy}..{ey}) written", all.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_year_of_an_epoch_is_the_civil_year() {
        assert_eq!(year_of(0.0), 1970);
        assert_eq!(year_of(1_700_000_000.0), 2023);
    }

    #[test]
    fn the_year_of_a_january_epoch_is_the_civil_year() {
        assert_eq!(
            year_of(iso_to_unix("1991-01-01T00:00:00Z").expect("parse")),
            1991
        );
        assert_eq!(
            year_of(iso_to_unix("2026-10-14T00:00:00Z").expect("parse")),
            2026
        );
    }

    #[test]
    fn iso_utc_round_trips_through_iso_to_unix() {
        let t = iso_to_unix("2026-01-01T00:00:00Z").expect("parse");
        assert_eq!(iso_utc(t), "2026-01-01T00:00:00Z");
    }
}
