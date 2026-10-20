use omegaflow::archivar::fetch::{fetch_raw, fetch_raw_bytes};
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::callisto::{
    BAND_COUNT, BACKGROUND_HALF_MIN, BandRecord, band_index, parse_bin, write_bin,
};
use omegaflow::cdn::upload_release;
use omegaflow::fits::{FitsHeader, FitsImage, FitsTable};
use omegaflow::inflate::gunzip;
use omegaflow::lsk::days_from_civil;
use std::collections::{BTreeMap, HashMap};
use std::path::Path;

const BASE: &str = "https://soleil.i4ds.ch/solarradio/data/2002-20yy_Callisto";
const CDN_TAG: &str = "soleil.i4ds.ch";

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


fn parse_days(s: &str) -> Option<i64> {
    let (y, rest) = s.split_once('-')?;
    let (m, d) = rest.split_once('-')?;
    days_from_civil(y.parse().ok()?, m.parse().ok()?, d.parse().ok()?)
}

fn parse_ymd(s: &str) -> Option<i64> {
    let (y, rest) = s.split('/').next().zip(s.split('/').nth(1))?;
    let d = s.split('/').nth(2)?;
    days_from_civil(y.parse().ok()?, rest.parse().ok()?, d.parse().ok()?)
}

fn parse_hms(s: &str) -> Option<f64> {
    let mut p = s.split(':');
    let h: f64 = p.next()?.parse().ok()?;
    let m: f64 = p.next()?.parse().ok()?;
    let sec: f64 = p.next()?.parse().ok()?;
    Some(h * 3600.0 + m * 60.0 + sec)
}

fn selected_files(html: &str, station: &str, ymd: &str) -> Vec<(String, String)> {
    let prefix = format!("{station}_{ymd}_");
    let mut best: HashMap<String, String> = HashMap::new();
    let mut rest = html;
    while let Some(pos) = rest.find("href=\"") {
        let tail = &rest[pos + 6..];
        let end = tail.find('"').unwrap_or(tail.len());
        let name = &tail[..end];
        rest = &tail[end..];
        if !name.starts_with(&prefix) || !name.ends_with(".fit.gz") {
            continue;
        }
        let stem = name.trim_end_matches(".fit.gz");
        let Some((stamp, suffix)) = stem.rsplit_once('_') else {
            continue;
        };
        let entry = best.entry(stamp.to_string()).or_insert_with(|| name.to_string());
        let current_suffix = entry
            .trim_end_matches(".fit.gz")
            .rsplit_once('_')
            .map(|(_, s)| s)
            .unwrap_or("");
        if suffix < current_suffix {
            *entry = name.to_string();
        }
    }
    let mut out: Vec<(String, String)> = best.into_iter().collect();
    out.sort_by(|a, b| a.0.cmp(&b.0));
    out
}

struct Accum {
    sums: BTreeMap<i64, [f64; BAND_COUNT]>,
    counts: BTreeMap<i64, [u32; BAND_COUNT]>,
}

fn add_file(bytes: &[u8], acc: &mut Accum) -> Option<usize> {
    let (header, _) = FitsHeader::parse(bytes, 0)?;
    let date_obs = header.str_unescaped("DATE-OBS")?;
    let time_obs = header.str_unescaped("TIME-OBS")?;
    let day = parse_ymd(&date_obs)?;
    let obs = parse_hms(&time_obs)?;
    let (image, next) = FitsImage::parse(bytes, 0)?;
    let (table, _) = FitsTable::parse(bytes, next)?;
    let time_col = table.column("TIME")?;
    let freq_col = table.column("FREQUENCY")?;
    let times = table.cell_array_f64(bytes, 0, time_col)?;
    let freqs = table.cell_array_f64(bytes, 0, freq_col)?;
    let ncols = image.dims[0].min(times.len());
    let nrows = image.dims[1].min(freqs.len());
    let mut row_band = Vec::with_capacity(nrows);
    for f in freqs.iter().take(nrows) {
        row_band.push(band_index(*f));
    }
    for x in 0..ncols {
        let t_unix = day as f64 * 86_400.0 + obs + times[x];
        if !t_unix.is_finite() {
            continue;
        }
        let minute = (t_unix / 60.0).floor() as i64;
        for (y, band) in row_band.iter().enumerate() {
            let Some(band) = band else {
                continue;
            };
            let Some(v) = image.value_f64(bytes, [x, y, 0]) else {
                continue;
            };
            if !v.is_finite() || v <= 0.0 {
                continue;
            }
            let sums = acc.sums.entry(minute).or_insert([0.0; BAND_COUNT]);
            let counts = acc.counts.entry(minute).or_insert([0u32; BAND_COUNT]);
            sums[*band] += v;
            counts[*band] += 1;
        }
    }
    Some(ncols)
}

fn median(mut vals: Vec<f64>) -> Option<f64> {
    if vals.is_empty() {
        return None;
    }
    vals.sort_by(|a, b| a.total_cmp(b));
    let n = vals.len();
    if n % 2 == 0 {
        Some((vals[n / 2 - 1] + vals[n / 2]) * 0.5)
    } else {
        Some(vals[n / 2])
    }
}

fn reduce(acc: &Accum) -> Vec<BandRecord> {
    let minutes: Vec<i64> = acc.sums.keys().copied().collect();
    let mut values: BTreeMap<i64, [Option<f64>; BAND_COUNT]> = BTreeMap::new();
    for &m in &minutes {
        let sums = match acc.sums.get(&m) {
            Some(s) => *s,
            None => [0.0; BAND_COUNT],
        };
        let counts = match acc.counts.get(&m) {
            Some(c) => *c,
            None => [0; BAND_COUNT],
        };
        let mut row = [None; BAND_COUNT];
        for b in 0..BAND_COUNT {
            if counts[b] > 0 {
                row[b] = Some(sums[b] / counts[b] as f64);
            }
        }
        values.insert(m, row);
    }

    let mut day_median: HashMap<i64, [Option<f64>; BAND_COUNT]> = HashMap::new();
    let mut per_day: HashMap<i64, [Vec<f64>; BAND_COUNT]> = HashMap::new();
    for (m, row) in &values {
        let day = m.div_euclid(1440);
        let slot = per_day.entry(day).or_default();
        for b in 0..BAND_COUNT {
            if let Some(v) = row[b] {
                slot[b].push(v);
            }
        }
    }
    for (day, mut slot) in per_day {
        let mut med = [None; BAND_COUNT];
        for b in 0..BAND_COUNT {
            med[b] = median(std::mem::take(&mut slot[b]));
        }
        day_median.insert(day, med);
    }

    let mut out = Vec::with_capacity(minutes.len());
    for &m in &minutes {
        let row = match values.get(&m) {
            Some(r) => *r,
            None => [None; BAND_COUNT],
        };
        let day = m.div_euclid(1440);
        let mut bands = [None; BAND_COUNT];
        for b in 0..BAND_COUNT {
            let Some(v) = row[b] else {
                continue;
            };
            let mut local = Vec::new();
            for &n in &minutes {
                if (n - m).abs() > BACKGROUND_HALF_MIN {
                    continue;
                }
                if let Some(x) = values.get(&n).and_then(|r| r[b]) {
                    local.push(x);
                }
            }
            let background = if local.len() >= 10 {
                median(local)
            } else {
                day_median.get(&day).and_then(|d| d[b])
            };
            if let Some(bg) = background {
                bands[b] = Some(v - bg);
            }
        }
        out.push(BandRecord {
            t_unix: m as f64 * 60.0,
            bands,
        });
    }
    out
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let station = match arg_value(&args, "--station") {
        Some(s) if !s.is_empty() => s,
        _ => {
            eprintln!("callisto_compiler needs --station <e-CALLISTO station>");
            std::process::exit(2);
        }
    };
    let (start, stop) = match (arg_value(&args, "--start"), arg_value(&args, "--stop")) {
        (Some(s), Some(e)) => (s, e),
        _ => {
            eprintln!("callisto_compiler needs --start <YYYY-MM-DD> and --stop <YYYY-MM-DD>");
            std::process::exit(2);
        }
    };
    let (Some(start_day), Some(stop_day)) = (parse_days(&start), parse_days(&stop)) else {
        eprintln!("--start/--stop carry no YYYY-MM-DD date");
        std::process::exit(2);
    };
    if stop_day < start_day {
        eprintln!("--stop {stop} lies before --start {start}");
        std::process::exit(2);
    }
    let out = match arg_value(&args, "--out") {
        Some(o) => o,
        None => format!("callisto_{}.bin", station.to_ascii_lowercase()),
    };
    let cache = match arg_value(&args, "--cache") {
        Some(c) => c,
        None => "tmp/callisto".to_string(),
    };
    let cache_dir = Path::new(&cache);
    if std::fs::create_dir_all(cache_dir).is_err() {
        eprintln!("create {} void", cache_dir.display());
        std::process::exit(1);
    }

    let mut acc = Accum {
        sums: BTreeMap::new(),
        counts: BTreeMap::new(),
    };
    let mut files = 0usize;
    let mut raw_bytes = 0usize;
    let mut missing_days = 0usize;
    for day in start_day..=stop_day {
        let (y, m, d) = civil_from_days(day);
        let ymd = format!("{y:04}{m:02}{d:02}");
        let index_url = format!("{BASE}/{y:04}/{m:02}/{d:02}/");
        let Some(html) = fetch_raw(&index_url, None, &[]) else {
            eprintln!("{index_url}: index fetch void — the day stays absent");
            missing_days += 1;
            continue;
        };
        let selected = selected_files(&html, &station, &ymd);
        if selected.is_empty() {
            eprintln!("{index_url}: no {station} files — the day stays absent");
            missing_days += 1;
            continue;
        }
        let mut day_files = 0usize;
        for (stamp, name) in selected {
            let path = cache_dir.join(&name);
            let bytes = if path.exists() {
                match std::fs::read(&path) {
                    Ok(b) => b,
                    Err(e) => {
                        eprintln!("{}: read void ({e})", path.display());
                        continue;
                    }
                }
            } else {
                let url = format!("{index_url}{name}");
                let Some(b) = fetch_raw_bytes(&url) else {
                    eprintln!("{url}: fetch void — the file stays absent");
                    continue;
                };
                if std::fs::write(&path, &b).is_err() {
                    eprintln!("{}: write void", path.display());
                    continue;
                }
                b
            };
            raw_bytes += bytes.len();
            let Some(fits) = gunzip(&bytes) else {
                eprintln!("{name}: gunzip void — the file stays absent");
                continue;
            };
            let Some(cols) = add_file(&fits, &mut acc) else {
                eprintln!("{name}: FITS image/table reads void — the file stays absent");
                continue;
            };
            files += 1;
            day_files += 1;
            eprintln!("{station} {stamp}: {cols} columns, {} B gz", bytes.len());
        }
        eprintln!("{ymd}: {day_files} files");
    }

    let append = args.iter().any(|a| a == "--append");
    let mut records = reduce(&acc);
    if append && Path::new(&out).exists() {
        match std::fs::read(&out).ok().and_then(|b| parse_bin(&b)) {
            Some(old) => {
                let mut merged: BTreeMap<i64, BandRecord> = BTreeMap::new();
                for r in old {
                    merged.insert((r.t_unix / 60.0).round() as i64, r);
                }
                for r in records {
                    merged.insert((r.t_unix / 60.0).round() as i64, r);
                }
                records = merged.into_values().collect();
                eprintln!("{out}: merged with the existing asset");
            }
            None => eprintln!("{out}: existing asset reads void — append refused"),
        }
    }
    let with_band = records
        .iter()
        .filter(|r| r.bands.iter().any(|b| b.is_some()))
        .count();
    eprintln!(
        "{station} {start}..{stop}: {files} files, {raw_bytes} B gz, {with_band} minutes, {missing_days} missing days"
    );
    if with_band == 0 {
        eprintln!("no reduced minutes — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
    let bytes = write_bin(&records);
    if let Some(parent) = Path::new(&out).parent() {
        if !parent.as_os_str().is_empty() && std::fs::create_dir_all(parent).is_err() {
            eprintln!("create {} void", parent.display());
            std::process::exit(1);
        }
    }
    if std::fs::write(&out, &bytes).is_err() {
        eprintln!("write {out} returned void");
        std::process::exit(1);
    }
    match parse_bin(&bytes) {
        Some(parsed) => {
            let first = parsed.first();
            let last = parsed.last();
            match (first, last) {
                (Some(first), Some(last)) => eprintln!(
                    "{out}: {} minutes, unix {:.0}..{:.0}, {} B, sha256 {}",
                    parsed.len(),
                    first.t_unix,
                    last.t_unix,
                    bytes.len(),
                    sha256_hex(&bytes)
                ),
                _ => eprintln!("{out}: {} minutes, {} B, sha256 {}", parsed.len(), bytes.len(), sha256_hex(&bytes)),
            }
        }
        None => {
            eprintln!("{out}: roundtrip parse void");
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_release(CDN_TAG, &out) {
        std::process::exit(1);
    }
}
