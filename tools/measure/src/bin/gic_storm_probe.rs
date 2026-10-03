use omegaflow::archivar::omni2::{COMP_BZ, COMP_N1800, COMP_V1800, parse_bin};
use omegaflow::archivar::{JsonVal, fetch_raw, fetch_raw_bytes, parse_json, scalar_of};
use omegaflow::te::{phase_randomized_surrogate, transfer_entropy_lag};
use std::fmt::Write as _;
use std::time::{SystemTime, UNIX_EPOCH};

const SURROGATE_SEED: u64 = 0x9E37_79B9_7F4A_7C15;
const DEFAULT_N_SURR: usize = 100;
const LAGS: [usize; 2] = [0, 1];
const MINUTE: f64 = 60.0;
const HOUR: f64 = 3600.0;
const DAY: f64 = 86400.0;
const KP_INTERVAL_H: f64 = 3.0;
const J2000_UNIX_OFFSET: f64 = 946728000.0;
const OMNI2_BIN: &str = "omni2_serie_1h.bin";
const OMNI2_CDN_BASE: &str =
    "https://github.com/omegaflow/sources/releases/download/ssd.jpl.nasa.gov";
const IMAG_CDN_BASE: &str =
    "https://github.com/omegaflow/sources/releases/download/imag-data.bgs.ac.uk";
const KP_URL: &str = "https://services.swpc.noaa.gov/products/noaa-planetary-k-index.json";
const DEFAULT_STATION: &str = "ABK";
const DEFAULT_HOUR_START: &str = "2024-01-01";
const DEFAULT_HOUR_END: &str = "2024-12-31";
const DEFAULT_KP_THRESHOLD: f64 = 5.0;

fn now_unix() -> Option<f64> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .ok()
        .map(|d| d.as_secs_f64())
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

fn arg_after<'a>(args: &'a [String], flag: &str) -> Option<&'a str> {
    args.iter()
        .position(|a| a == flag)
        .and_then(|i| args.get(i + 1))
        .map(|s| s.as_str())
}

fn disk_cache(name: &str) -> String {
    omegaflow::archivar::cache_root()
        .join("bz_retro")
        .join(name)
        .to_string_lossy()
        .into_owned()
}

fn load_omni2(path: &str) -> Vec<(f64, f64, u32)> {
    if let Ok(bytes) = std::fs::read(path) {
        if let Some(recs) = parse_bin(&bytes) {
            return recs;
        }
        eprintln!("{path} parses void — fetching the CDN asset");
    } else {
        eprintln!("{path} reads void — fetching the CDN asset");
    }
    let name = match std::path::Path::new(path).file_name() {
        Some(n) => n.to_string_lossy().into_owned(),
        None => {
            eprintln!("{path} carries no asset name — the CDN fetch stays pending");
            return Vec::new();
        }
    };
    let url = format!("{OMNI2_CDN_BASE}/{name}");
    let Some(bytes) = fetch_raw_bytes(&url) else {
        eprintln!("{url} fetch stays pending — the top series stays unmeasured");
        return Vec::new();
    };
    let _ = std::fs::write(path, &bytes);
    match parse_bin(&bytes) {
        Some(recs) => recs,
        None => {
            eprintln!("{path} parses void after fetch — the top series stays unmeasured");
            Vec::new()
        }
    }
}

fn load_cdn_station_dbdt(station: &str) -> Option<Vec<(f64, f64)>> {
    let asset = format!("{}_dbdt_1h.bin", station.to_lowercase());
    let local = disk_cache(&asset);
    let bytes = std::fs::read(&local).ok().or_else(|| {
        let url = format!("{IMAG_CDN_BASE}/{asset}");
        match fetch_raw_bytes(&url) {
            Some(b) => {
                let _ = std::fs::write(&local, &b);
                Some(b)
            }
            None => None,
        }
    })?;
    let recs = omegaflow::intermagnet::parse_bin(&bytes)?;
    Some(recs.into_iter().map(|(t, v, _)| (t, v)).collect())
}

fn harvest_kp() -> Vec<(f64, f64)> {
    let body = match fetch_raw(KP_URL, None, &[]) {
        Some(b) => b,
        None => return Vec::new(),
    };
    let j = match parse_json(&body) {
        Some(j) => j,
        None => return Vec::new(),
    };
    let JsonVal::Arr(elements) = j else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for el in elements {
        let JsonVal::Obj(map) = el else {
            continue;
        };
        let Some(epoch) = map.get("time_tag").and_then(|v| match v {
            JsonVal::Str(s) => iso_to_unix(s),
            _ => None,
        }) else {
            continue;
        };
        let Some(raw) = map.get("Kp").and_then(scalar_of) else {
            continue;
        };
        if raw.is_finite() && (0.0..=9.0).contains(&raw) {
            out.push((epoch, raw));
        }
    }
    out.sort_by(|a, b| a.0.total_cmp(&b.0));
    out
}

fn shared_window(
    a: &[(f64, f64)],
    b: &[(f64, f64)],
    lo_clamp: Option<f64>,
    hi_clamp: Option<f64>,
) -> Option<(f64, f64)> {
    let (Some(a0), Some(a1)) = (a.first().map(|&(t, _)| t), a.last().map(|&(t, _)| t)) else {
        return None;
    };
    let (Some(b0), Some(b1)) = (b.first().map(|&(t, _)| t), b.last().map(|&(t, _)| t)) else {
        return None;
    };
    let lo = match lo_clamp {
        Some(c) => a0.max(b0).max(c),
        None => a0.max(b0),
    };
    let hi = match hi_clamp {
        Some(c) => a1.min(b1).min(c),
        None => a1.min(b1),
    };
    if hi <= lo { None } else { Some((lo, hi)) }
}

fn bin_cells(series: &[(f64, f64)], t0: f64, dt: f64, n: usize) -> Vec<Option<f32>> {
    let mut sums = vec![0.0f64; n];
    let mut counts = vec![0u32; n];
    for &(t, v) in series {
        let idx = ((t - t0) / dt).floor();
        if idx < 0.0 || idx >= n as f64 {
            continue;
        }
        let i = idx as usize;
        sums[i] += v;
        counts[i] += 1;
    }
    (0..n)
        .map(|i| {
            if counts[i] > 0 {
                Some((sums[i] / counts[i] as f64) as f32)
            } else {
                None
            }
        })
        .collect()
}

fn storm_at(times: &[f64], kp: &[(f64, f64)], threshold: f64) -> Vec<bool> {
    times
        .iter()
        .map(|&t| {
            kp.iter()
                .any(|&(kt, kv)| kv >= threshold && t >= kt && t < kt + KP_INTERVAL_H * HOUR)
        })
        .collect()
}

fn in_window(t: f64, lo: Option<f64>, hi: Option<f64>) -> bool {
    if let Some(lo) = lo {
        if t < lo {
            return false;
        }
    }
    if let Some(hi) = hi {
        if t >= hi {
            return false;
        }
    }
    true
}

fn selected_pairs(channels: &[&[Option<f32>]], select: impl Fn(usize) -> bool) -> Vec<Vec<f32>> {
    let Some(n) = channels.iter().map(|c| c.len()).min() else {
        return vec![Vec::new(); channels.len()];
    };
    let mut out: Vec<Vec<f32>> = vec![Vec::new(); channels.len()];
    for i in 0..n {
        if !select(i) {
            continue;
        }
        if !channels.iter().all(|c| c[i].is_some()) {
            continue;
        }
        for (ci, c) in channels.iter().enumerate() {
            if let Some(v) = c[i] {
                out[ci].push(v);
            }
        }
    }
    out
}

fn family_pairs(cols: &[Vec<f32>]) -> Vec<(&'static str, &'static str, &[f32], &[f32])> {
    if cols.len() < 4 {
        return Vec::new();
    }
    let dbdt = cols[0].as_slice();
    let bz = cols[1].as_slice();
    let speed = cols[2].as_slice();
    let density = cols[3].as_slice();
    vec![
        ("Bz", "dB/dt", dbdt, bz),
        ("dB/dt", "Bz", bz, dbdt),
        ("Speed", "dB/dt", dbdt, speed),
        ("dB/dt", "Speed", speed, dbdt),
        ("Density", "dB/dt", dbdt, density),
        ("dB/dt", "Density", density, dbdt),
    ]
}

fn surrogate_te_values(to: &[f32], from: &[f32], lag: usize, n_surr: usize) -> Vec<f64> {
    let mut rng = SURROGATE_SEED.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut vals = Vec::with_capacity(n_surr);
    for _ in 0..n_surr {
        let ys = phase_randomized_surrogate(from, &mut rng);
        if let Some(te) = transfer_entropy_lag(to, &ys, lag) {
            vals.push(te);
        }
    }
    vals
}

fn mean_plus_2sigma(vals: &[f64]) -> Option<f64> {
    if vals.len() < 2 {
        return None;
    }
    let m = vals.iter().sum::<f64>() / vals.len() as f64;
    let var = vals.iter().map(|v| (v - m) * (v - m)).sum::<f64>() / vals.len() as f64;
    Some(m + 2.0 * var.sqrt())
}

struct Row {
    from: &'static str,
    to: &'static str,
    lag: usize,
    te: Option<f64>,
    thr: Option<f64>,
    verdict: &'static str,
}

struct Round {
    label: &'static str,
    n: usize,
    fam: Option<f64>,
    rows: Vec<Row>,
}

fn measure_round(label: &'static str, cols: &[Vec<f32>], n_surr: usize, threads: usize) -> Round {
    let n = cols.first().map_or(0, Vec::len);
    let pairs = family_pairs(cols);
    let mut job_pair: Vec<usize> = Vec::new();
    let mut job_lag: Vec<usize> = Vec::new();
    for (pi, _) in pairs.iter().enumerate() {
        for &lag in LAGS.iter() {
            job_pair.push(pi);
            job_lag.push(lag);
        }
    }
    let mut results: Vec<Option<(f64, Option<f64>, f64)>> = vec![None; job_pair.len()];
    if threads > 1 && job_pair.len() > 1 {
        use std::sync::atomic::{AtomicUsize, Ordering};
        let next = std::sync::Arc::new(AtomicUsize::new(0));
        let partials = std::thread::scope(|s| {
            let mut handles = Vec::new();
            for _ in 0..threads.min(job_pair.len()) {
                let next = std::sync::Arc::clone(&next);
                let job_pair = &job_pair;
                let job_lag = &job_lag;
                let pairs = &pairs;
                handles.push(s.spawn(move || {
                    let mut out = Vec::new();
                    loop {
                        let i = next.fetch_add(1, Ordering::Relaxed);
                        if i >= job_pair.len() {
                            break;
                        }
                        let (_, _, to_s, from_s) = pairs[job_pair[i]];
                        let lag = job_lag[i];
                        let Some(te) = transfer_entropy_lag(to_s, from_s, lag) else {
                            continue;
                        };
                        let vals = surrogate_te_values(to_s, from_s, lag, n_surr);
                        let fam_partial = vals.iter().copied().fold(f64::NEG_INFINITY, f64::max);
                        let thr = mean_plus_2sigma(&vals);
                        out.push((i, (te, thr, fam_partial)));
                    }
                    out
                }));
            }
            handles
                .into_iter()
                .map(|h| h.join().expect("worker thread joins"))
                .collect::<Vec<_>>()
        });
        for p in partials {
            for (i, v) in p {
                results[i] = Some(v);
            }
        }
    } else {
        for i in 0..job_pair.len() {
            let (_, _, to_s, from_s) = pairs[job_pair[i]];
            let lag = job_lag[i];
            let Some(te) = transfer_entropy_lag(to_s, from_s, lag) else {
                continue;
            };
            let vals = surrogate_te_values(to_s, from_s, lag, n_surr);
            let fam_partial = vals.iter().copied().fold(f64::NEG_INFINITY, f64::max);
            let thr = mean_plus_2sigma(&vals);
            results[i] = Some((te, thr, fam_partial));
        }
    }

    let mut fam: Option<f64> = None;
    for r in results.iter().flatten() {
        let fp = r.2;
        if fp.is_finite() {
            fam = Some(match fam {
                Some(m) => m.max(fp),
                None => fp,
            });
        }
    }

    let lag_n = LAGS.len();
    let mut rows = Vec::with_capacity(pairs.len());
    for (pi, (from, to, _, _)) in pairs.iter().enumerate() {
        let mut best: Option<(usize, f64)> = None;
        for (li, _) in LAGS.iter().enumerate() {
            let idx = pi * lag_n + li;
            if let Some((te, _, _)) = results[idx] {
                if best.map_or(true, |(_, b)| te > b) {
                    best = Some((li, te));
                }
            }
        }
        let (lag, te, thr) = match best {
            Some((li, te)) => {
                let idx = pi * lag_n + li;
                let thr = results[idx].and_then(|(_, t, _)| t);
                (LAGS[li], Some(te), thr)
            }
            None => (LAGS[0], None, None),
        };
        let verdict = match (te, fam, thr) {
            (None, _, _) => "pending",
            (Some(te), Some(f), _) if te > f => "arrow",
            (Some(te), _, Some(t)) if te > t => "family bound",
            (Some(_), _, Some(_)) => "silent",
            (Some(_), _, None) => "threshold absent",
        };
        rows.push(Row {
            from,
            to,
            lag,
            te,
            thr,
            verdict,
        });
    }

    Round {
        label,
        n,
        fam,
        rows,
    }
}

fn opt_te(v: Option<f64>) -> String {
    match v {
        Some(x) => format!("{x:.4e}"),
        None => "pending".to_string(),
    }
}

fn push_round(report: &mut Vec<String>, round: &Round) {
    report.push(format!("=== {} ===", round.label));
    report.push(format!(
        "paired hours: n = {} | fam = {}",
        round.n,
        match round.fam {
            Some(f) => format!("{f:.4e}"),
            None => "pending".to_string(),
        }
    ));
    report.push(format!(
        "{:<12} | {:>5} | {:>12} | {:>12} | verdict",
        "pair", "lag h", "TE", "thr"
    ));
    for r in &round.rows {
        report.push(format!(
            "{:<12} | {:>5} | {:>12} | {:>12} | {}",
            format!("{} -> {}", r.from, r.to),
            r.lag,
            opt_te(r.te),
            opt_te(r.thr),
            r.verdict
        ));
    }
}

fn bz_row(round: &Round) -> Option<&Row> {
    round
        .rows
        .iter()
        .find(|r| r.from == "Bz" && r.to == "dB/dt")
}

fn flush(report: &[String], out_path: Option<&str>) {
    for line in report {
        println!("{line}");
    }
    if let Some(path) = out_path {
        let mut body = String::new();
        for line in report {
            let _ = writeln!(body, "{line}");
        }
        match std::fs::write(path, &body) {
            Ok(()) => println!("out: {path} written ({} lines)", report.len()),
            Err(e) => eprintln!("out: {path} writes void — {e}"),
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let station = arg_after(&args, "--station")
        .unwrap_or(DEFAULT_STATION)
        .to_string();
    let hour_start = arg_after(&args, "--hour-start")
        .unwrap_or(DEFAULT_HOUR_START)
        .to_string();
    let hour_end = arg_after(&args, "--hour-end")
        .unwrap_or(DEFAULT_HOUR_END)
        .to_string();
    let kp_threshold = arg_after(&args, "--kp-threshold")
        .and_then(|v| v.parse::<f64>().ok())
        .filter(|v| v.is_finite() && *v >= 0.0)
        .unwrap_or(DEFAULT_KP_THRESHOLD);
    let n_surr = arg_after(&args, "--n-surreps")
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|&v| v > 0)
        .unwrap_or(DEFAULT_N_SURR);
    let out_path = arg_after(&args, "--out").map(|s| s.to_string());
    let threads = arg_after(&args, "--threads")
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|&t| t > 0)
        .unwrap_or(4);

    let mut report: Vec<String> = Vec::new();
    report
        .push("=== GIC storm-only sub-analysis — Bz -> dB/dt on the storm subset ===".to_string());
    report.push(format!(
        "station = {station} | window = {hour_start} -> {hour_end} | Kp threshold = {kp_threshold} | n_surr = {n_surr} | seed = 0x{SURROGATE_SEED:016X}"
    ));
    if let Some(now) = now_unix() {
        report.push(format!("system time: {}", iso_utc(now)));
    }
    report.push(
        "chain: Bz -> dB/dt transfer entropy, phase-randomized surrogate null, family bound over the 6-pair round; storm hours selected by Kp from the wired SWPC channel."
            .to_string(),
    );

    let h_start = iso_to_unix(&format!("{hour_start}T00:00:00Z"));
    let h_end = iso_to_unix(&format!("{hour_end}T00:00:00Z"));

    let omni = load_omni2(&disk_cache(OMNI2_BIN));
    let omni_bz: Vec<(f64, f64)> = omni
        .iter()
        .filter(|(_, _, c)| *c == COMP_BZ)
        .map(|&(t, v, _)| (t + J2000_UNIX_OFFSET, v))
        .collect();
    let omni_speed: Vec<(f64, f64)> = omni
        .iter()
        .filter(|(_, _, c)| *c == COMP_V1800)
        .map(|&(t, v, _)| (t + J2000_UNIX_OFFSET, v))
        .collect();
    let omni_density: Vec<(f64, f64)> = omni
        .iter()
        .filter(|(_, _, c)| *c == COMP_N1800)
        .map(|&(t, v, _)| (t + J2000_UNIX_OFFSET, v))
        .collect();
    let Some(dbdt_1h) = load_cdn_station_dbdt(&station) else {
        report.push(format!(
            "{station} hourly |dB/dt| absent — the CDN asset stays pending; no measurement"
        ));
        flush(&report, out_path.as_deref());
        return;
    };
    report.push(format!(
        "omni2_serie_1h.bin: Bz {} | Speed {} | Density {} | {station} dB/dt {}",
        omni_bz.len(),
        omni_speed.len(),
        omni_density.len(),
        dbdt_1h.len()
    ));

    let Some((lo, hi)) = shared_window(&omni_bz, &dbdt_1h, h_start, h_end) else {
        report.push(format!(
            "common hourly window absent — no hour shared by OMNI2 and {station} |dB/dt|"
        ));
        flush(&report, out_path.as_deref());
        return;
    };
    let t0 = (lo / HOUR).floor() * HOUR;
    let n_cells = ((hi - t0) / HOUR).floor().max(1.0) as usize;
    let times_all: Vec<f64> = (0..n_cells).map(|i| t0 + i as f64 * HOUR).collect();
    report.push(format!(
        "common hourly window: {} -> {} | {} hours",
        iso_utc(t0),
        iso_utc(t0 + n_cells as f64 * HOUR),
        n_cells
    ));

    let kp = harvest_kp();
    if kp.is_empty() {
        report.push(
            "Kp harvest absent — the storm subset stays pending (never 0.0); the yearly round stands"
                .to_string(),
        );
    } else {
        report.push(format!(
            "Kp channel: {} records (3-h grid, SWPC noaa-planetary-k-index)",
            kp.len()
        ));
    }
    let storm = storm_at(&times_all, &kp, kp_threshold);
    let flagged = storm.iter().filter(|&&b| b).count();

    let bz = bin_cells(&omni_bz, t0, HOUR, n_cells);
    let speed = bin_cells(&omni_speed, t0, HOUR, n_cells);
    let density = bin_cells(&omni_density, t0, HOUR, n_cells);
    let dbdt = bin_cells(&dbdt_1h, t0, HOUR, n_cells);

    let year_cols = selected_pairs(&[&dbdt, &bz, &speed, &density], |_| true);
    let storm_cols = selected_pairs(&[&dbdt, &bz, &speed, &density], |i| {
        storm.get(i).copied().unwrap_or(false) && in_window(times_all[i], h_start, h_end)
    });

    let storm_round = measure_round("Storm-only round", &storm_cols, n_surr, threads);
    let year_round = measure_round(
        "Yearly round (all paired hours)",
        &year_cols,
        n_surr,
        threads,
    );

    report.push(String::new());
    report.push(format!(
        "storm hours flagged (Kp >= {kp_threshold}, hourly grid): {flagged}"
    ));
    push_round(&mut report, &storm_round);
    report.push(String::new());
    push_round(&mut report, &year_round);

    report.push(String::new());
    report.push(format!(
        "Vergleich Jahres-Rund vs storm-only | {station} | {hour_start}..{hour_end} | Kp >= {kp_threshold}"
    ));
    let storm_bz = bz_row(&storm_round);
    let year_bz = bz_row(&year_round);
    match (storm_bz, year_bz) {
        (Some(s), Some(y)) => report.push(format!(
            "Bz -> dB/dt | storm n = {} lag {} TE {} | year n = {} lag {} TE {} | storm {} | year {}",
            storm_round.n,
            s.lag,
            opt_te(s.te),
            year_round.n,
            y.lag,
            opt_te(y.te),
            s.verdict,
            y.verdict
        )),
        (Some(s), None) => report.push(format!(
            "Bz -> dB/dt | storm n = {} lag {} TE {} | year round absent | storm {}",
            storm_round.n,
            s.lag,
            opt_te(s.te),
            s.verdict
        )),
        (None, Some(y)) => report.push(format!(
            "Bz -> dB/dt | storm subset pending (n = {}) | year n = {} lag {} TE {} | year {}",
            storm_round.n,
            year_round.n,
            y.lag,
            opt_te(y.te),
            y.verdict
        )),
        (None, None) => report.push(format!(
            "Bz -> dB/dt | both rounds pending (storm n = {}, year n = {}) — the cells carry too few samples (n < 8), never 0.0",
            storm_round.n, year_round.n
        )),
    }
    report.push(
        "Verdikt: what the machine measures — pending stays pending (0 honored), never a fabricated value."
            .to_string(),
    );

    flush(&report, out_path.as_deref());
}
