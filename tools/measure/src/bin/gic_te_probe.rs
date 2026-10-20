use omegaflow::archivar::{fetch_raw_bytes, geo, omni2};
use omegaflow::lsk::days_from_civil;
use omegaflow::te::{phase_randomized_surrogate, transfer_entropy_lag};
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicUsize, Ordering};

const SURROGATE_SEED: u64 = 0x9E37_79B9_7F4A_7C15;
const DEFAULT_N_SURR: usize = 10;
const STRIDE: usize = 3;
const DAY: f64 = 86400.0;
const HOUR: f64 = 3600.0;
const J2000_UNIX_OFFSET: f64 = 946728000.0;
const GIC_URL: &str =
    "https://github.com/omegaflow/sources/releases/download/space.fmi.fi/fmi_gic.bin";
const OMNI_URL: &str =
    "https://github.com/omegaflow/sources/releases/download/ssd.jpl.nasa.gov/omni2_serie_1h.bin";

fn arg_after<'a>(args: &'a [String], flag: &str) -> Option<&'a str> {
    args.iter()
        .position(|a| a == flag)
        .and_then(|i| args.get(i + 1))
        .map(|s| s.as_str())
}

fn read_asset(path: &str, url: &str) -> Option<Vec<u8>> {
    if let Ok(bytes) = std::fs::read(path) {
        return Some(bytes);
    }
    let bytes = fetch_raw_bytes(url)?;
    let _ = std::fs::write(path, &bytes);
    Some(bytes)
}

fn median(v: &mut [f64]) -> Option<f64> {
    if v.is_empty() {
        return None;
    }
    v.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let n = v.len();
    Some(if n % 2 == 1 {
        v[n / 2]
    } else {
        0.5 * (v[n / 2 - 1] + v[n / 2])
    })
}

fn daily_gic(recs: &[geo::GeoRec]) -> BTreeMap<i64, f64> {
    let mut out: BTreeMap<i64, f64> = BTreeMap::new();
    for r in recs {
        if r.comp != geo::COMP_GIC_A || !r.val.is_finite() {
            continue;
        }
        let unix = r.t + J2000_UNIX_OFFSET;
        let day = (unix / DAY).floor() as i64;
        let a = r.val.abs();
        let e = out.entry(day).or_insert(0.0);
        if a > *e {
            *e = a;
        }
    }
    out
}

fn hourly_gic(recs: &[geo::GeoRec]) -> BTreeMap<i64, f64> {
    let mut out: BTreeMap<i64, f64> = BTreeMap::new();
    for r in recs {
        if r.comp != geo::COMP_GIC_A || !r.val.is_finite() {
            continue;
        }
        let unix = r.t + J2000_UNIX_OFFSET;
        let hour = (unix / HOUR).floor() as i64;
        let a = r.val.abs();
        let e = out.entry(hour).or_insert(0.0);
        if a > *e {
            *e = a;
        }
    }
    out
}

fn daily_omni(recs: &[(f64, f64, u32)], comp: u32) -> BTreeMap<i64, f64> {
    let mut buckets: BTreeMap<i64, Vec<f64>> = BTreeMap::new();
    for &(t, v, c) in recs {
        if c != comp || !v.is_finite() {
            continue;
        }
        let unix = t + J2000_UNIX_OFFSET;
        let day = (unix / DAY).floor() as i64;
        buckets.entry(day).or_default().push(v);
    }
    let mut out = BTreeMap::new();
    for (day, mut vals) in buckets {
        if let Some(m) = median(&mut vals) {
            out.insert(day, m);
        }
    }
    out
}

fn hourly_omni(recs: &[(f64, f64, u32)], comp: u32) -> BTreeMap<i64, f64> {
    let mut buckets: BTreeMap<i64, Vec<f64>> = BTreeMap::new();
    for &(t, v, c) in recs {
        if c != comp || !v.is_finite() {
            continue;
        }
        let unix = t + J2000_UNIX_OFFSET;
        let hour = (unix / HOUR).floor() as i64;
        buckets.entry(hour).or_default().push(v);
    }
    let mut out = BTreeMap::new();
    for (hour, mut vals) in buckets {
        if let Some(m) = median(&mut vals) {
            out.insert(hour, m);
        }
    }
    out
}

fn day_from_iso(s: &str) -> Option<i64> {
    let mut p = s.split('-');
    let y: i64 = p.next()?.parse().ok()?;
    let m: i64 = p.next()?.parse().ok()?;
    let d: i64 = p.next()?.parse().ok()?;
    days_from_civil(y, m, d)
}

fn iso_from_day(day: i64) -> String {
    let z = day + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    format!("{y:04}-{m:02}-{d:02}")
}

fn iso_from_hour(hour: i64) -> String {
    let unix = hour as f64 * HOUR;
    let day = (unix / DAY).floor() as i64;
    let hh = ((unix - day as f64 * DAY) / HOUR).floor() as i64;
    format!("{}T{hh:02}:00Z", iso_from_day(day))
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

fn family_pairs(cols: &[Vec<f32>]) -> Vec<(&'static str, &'static str, &[f32], &[f32])> {
    if cols.len() < 4 {
        return Vec::new();
    }
    let gic = cols[0].as_slice();
    let bz = cols[1].as_slice();
    let speed = cols[2].as_slice();
    let density = cols[3].as_slice();
    vec![
        ("Bz", "GIC", gic, bz),
        ("GIC", "Bz", bz, gic),
        ("Speed", "GIC", gic, speed),
        ("GIC", "Speed", speed, gic),
        ("Density", "GIC", gic, density),
        ("GIC", "Density", density, gic),
    ]
}

fn parse_lags(args: &[String]) -> Vec<usize> {
    let Some(raw) = arg_after(args, "--lags") else {
        return vec![0, 1];
    };
    let mut out: Vec<usize> = raw
        .split(',')
        .filter_map(|s| s.trim().parse::<usize>().ok())
        .collect();
    out.sort_unstable();
    out.dedup();
    if out.is_empty() { vec![0, 1] } else { out }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let gic_path = omegaflow::or_temp(arg_after(&args, "--gic"), "gic_te/fmi_gic.bin");
    let omni_path = omegaflow::or_temp(arg_after(&args, "--omni"), "gic_te/omni2_serie_1h.bin");
    let start = arg_after(&args, "--start")
        .and_then(day_from_iso)
        .unwrap_or(10957);
    let end = arg_after(&args, "--end")
        .and_then(day_from_iso)
        .unwrap_or(45282);
    let n_surr = arg_after(&args, "--n-surreps")
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|&v| v > 0)
        .unwrap_or(DEFAULT_N_SURR);
    let grain = arg_after(&args, "--grain").unwrap_or("day");
    let lags = parse_lags(&args);
    let threads = match arg_after(&args, "--threads") {
        Some(v) => match v.parse::<usize>() {
            Ok(t) if t > 0 => t,
            _ => 1,
        },
        None => std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1),
    };

    let Some(gic_bytes) = read_asset(&gic_path, GIC_URL) else {
        eprintln!("fmi_gic.bin: fetch void — no measurement");
        std::process::exit(1);
    };
    let Some(gic_recs) = geo::parse_bin(geo::MAGIC_GIC, &gic_bytes) else {
        eprintln!("fmi_gic.bin: parses void — no measurement");
        std::process::exit(1);
    };
    let Some(omni_bytes) = read_asset(&omni_path, OMNI_URL) else {
        eprintln!("omni2_serie_1h.bin: fetch void — no measurement");
        std::process::exit(1);
    };
    let Some(omni_recs) = omni2::parse_bin(&omni_bytes) else {
        eprintln!("omni2_serie_1h.bin: parses void — no measurement");
        std::process::exit(1);
    };

    let hourly = grain == "hour" || grain == "hourly";
    let step = if hourly { 1 } else { STRIDE };
    let unit = if hourly { "h" } else { "d" };

    let (gic, bz, speed, density) = if hourly {
        (
            hourly_gic(&gic_recs),
            hourly_omni(&omni_recs, omni2::COMP_BZ),
            hourly_omni(&omni_recs, omni2::COMP_V1800),
            hourly_omni(&omni_recs, omni2::COMP_N1800),
        )
    } else {
        (
            daily_gic(&gic_recs),
            daily_omni(&omni_recs, omni2::COMP_BZ),
            daily_omni(&omni_recs, omni2::COMP_V1800),
            daily_omni(&omni_recs, omni2::COMP_N1800),
        )
    };

    println!("=== GIC causal-driver probe — measured GIC [A] ===");
    println!(
        "gic: {} records, {} {} buckets | omni2: {} records (Bz {}, Speed {}, Density {})",
        gic_recs.len(),
        gic.len(),
        unit,
        omni_recs.len(),
        bz.len(),
        speed.len(),
        density.len()
    );

    let (keys, n_common, first_label, last_label, span_days) = if hourly {
        let lo = start as f64 * DAY;
        let hi = end as f64 * DAY;
        let mut keys: Vec<i64> = gic
            .keys()
            .filter(|&&k| k as f64 * HOUR >= lo && (k as f64 * HOUR) < hi)
            .filter(|k| bz.contains_key(k) && speed.contains_key(k) && density.contains_key(k))
            .copied()
            .collect();
        keys.sort_unstable();
        let n_common = keys.len();
        let first = *keys.first().unwrap_or(&0);
        let last = *keys.last().unwrap_or(&0);
        (
            keys,
            n_common,
            iso_from_hour(first),
            iso_from_hour(last),
            (last - first + 1) as f64 * HOUR / DAY,
        )
    } else {
        let mut keys: Vec<i64> = gic
            .keys()
            .filter(|d| **d >= start && **d <= end)
            .filter(|d| bz.contains_key(d) && speed.contains_key(d) && density.contains_key(d))
            .copied()
            .collect();
        keys.sort_unstable();
        let n_common = keys.len();
        keys = keys.into_iter().step_by(STRIDE).collect();
        let first = *keys.first().unwrap_or(&0);
        let last = *keys.last().unwrap_or(&0);
        (
            keys,
            n_common,
            iso_from_day(first),
            iso_from_day(last),
            (last - first + 1) as f64,
        )
    };
    let n = keys.len();
    if n < 8 {
        eprintln!("only {n} common {unit} samples — no measurement");
        std::process::exit(1);
    }
    println!(
        "window: {} -> {} ({:.2} days, {:.2} years) | common {} samples {} -> n {}",
        first_label,
        last_label,
        span_days,
        span_days / 365.25,
        unit,
        n_common,
        n
    );
    println!(
        "grain = {} | seed = 0x{SURROGATE_SEED:016X} | n_surr = {n_surr} | lags = {:?} ({unit}) | threads = {threads}",
        if hourly { "hour" } else { "day" },
        lags
    );
    if args.iter().any(|a| a == "--dry-run") {
        return;
    }

    let col = |m: &BTreeMap<i64, f64>| -> Vec<f32> {
        keys.iter()
            .map(|k| *m.get(k).unwrap_or(&f64::NAN) as f32)
            .collect()
    };
    let cols = vec![col(&gic), col(&bz), col(&speed), col(&density)];
    let pairs = family_pairs(&cols);

    let mut jobs: Vec<(usize, usize)> = Vec::new();
    for (pi, _) in pairs.iter().enumerate() {
        for (li, _) in lags.iter().enumerate() {
            jobs.push((pi, li));
        }
    }
    let mut results: Vec<Option<(f64, Option<f64>, f64)>> = vec![None; jobs.len()];
    if threads > 1 && jobs.len() > 1 {
        let next = std::sync::Arc::new(AtomicUsize::new(0));
        let partials = std::thread::scope(|s| {
            let mut handles = Vec::new();
            for _ in 0..threads.min(jobs.len()) {
                let next = std::sync::Arc::clone(&next);
                let jobs = &jobs;
                let pairs = &pairs;
                let lags = &lags;
                handles.push(s.spawn(move || {
                    let mut out = Vec::new();
                    loop {
                        let i = next.fetch_add(1, Ordering::Relaxed);
                        if i >= jobs.len() {
                            break;
                        }
                        let (pi, li) = jobs[i];
                        let (_, _, to_s, from_s) = pairs[pi];
                        let lag = lags[li];
                        let Some(te) = transfer_entropy_lag(to_s, from_s, lag) else {
                            continue;
                        };
                        let vals = surrogate_te_values(to_s, from_s, lag, n_surr);
                        let partial = vals.iter().copied().fold(f64::NEG_INFINITY, f64::max);
                        let thr = mean_plus_2sigma(&vals);
                        out.push((i, (te, thr, partial)));
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
        for (i, &(pi, li)) in jobs.iter().enumerate() {
            let (_, _, to_s, from_s) = pairs[pi];
            let lag = lags[li];
            if let Some(te) = transfer_entropy_lag(to_s, from_s, lag) {
                let vals = surrogate_te_values(to_s, from_s, lag, n_surr);
                let partial = vals.iter().copied().fold(f64::NEG_INFINITY, f64::max);
                let thr = mean_plus_2sigma(&vals);
                results[i] = Some((te, thr, partial));
            }
        }
    }

    let fam = results
        .iter()
        .flatten()
        .map(|r| r.2)
        .filter(|v| v.is_finite())
        .fold(None, |acc: Option<f64>, v| {
            Some(acc.map_or(v, |m| m.max(v)))
        });

    let mut rows: Vec<Row> = Vec::new();
    for (i, &(pi, li)) in jobs.iter().enumerate() {
        let Some((te, thr, _)) = results[i] else {
            continue;
        };
        let (from, to, _, _) = pairs[pi];
        let verdict = match (thr, fam) {
            (Some(_), Some(f)) if te > f => "arrow",
            (Some(t), Some(_)) if te > t => "family bound",
            (Some(_), _) => "silent",
            (None, _) => "threshold absent",
        };
        rows.push(Row {
            from,
            to,
            lag: lags[li] * step,
            te: Some(te),
            thr,
            verdict,
        });
    }
    println!(
        "fam = {}",
        fam.map_or("pending".to_string(), |f| format!("{f:.4e}"))
    );
    println!(
        "{:<8} -> {:<8} | {:>5} | {:>12} | {:>12} | verdict",
        "from",
        "to",
        format!("lag {unit}"),
        "TE",
        "thr"
    );
    for r in &rows {
        println!(
            "{:<8} -> {:<8} | {:>5} | {:>12} | {:>12} | {}",
            r.from,
            r.to,
            r.lag,
            r.te.map_or("pending".to_string(), |v| format!("{v:.4e}")),
            r.thr.map_or("pending".to_string(), |v| format!("{v:.4e}")),
            r.verdict
        );
    }
}
