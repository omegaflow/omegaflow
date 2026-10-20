use omegaflow::archivar::{geo, omni2};
use omegaflow::te::{
    phase_randomized_surrogate, transfer_entropy_conditional, transfer_entropy_lag,
};
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicUsize, Ordering};

const SURROGATE_SEED: u64 = 0x9E37_79B9_7F4A_7C15;
const DEFAULT_N_SURR: usize = 10;
const J2000_UNIX_OFFSET: f64 = 946_728_000.0;
const MINUTE: f64 = 60.0;
const HOUR: f64 = 3600.0;
const DAY: f64 = 86_400.0;

fn arg_after<'a>(args: &'a [String], flag: &str) -> Option<&'a str> {
    args.iter()
        .position(|a| a == flag)
        .and_then(|i| args.get(i + 1))
        .map(|s| s.as_str())
}

fn day_from_iso(s: &str) -> Option<i64> {
    let mut p = s.split('-');
    let y: i64 = p.next()?.parse().ok()?;
    let m: i64 = p.next()?.parse().ok()?;
    let d: i64 = p.next()?.parse().ok()?;
    omegaflow::lsk::days_from_civil(y, m, d)
}

fn iso_from_seconds(unix: f64) -> String {
    let days = (unix / DAY).floor() as i64;
    let rem = unix - days as f64 * DAY;
    let hh = (rem / HOUR).floor() as i64;
    let mm = ((rem - hh as f64 * HOUR) / MINUTE).floor() as i64;
    let z = days + 719468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    format!("{y:04}-{m:02}-{d:02}T{hh:02}:{mm:02}Z")
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

fn peak_buckets(recs: &[geo::GeoRec], width_s: f64, take_abs: bool) -> BTreeMap<i64, f64> {
    let mut out: BTreeMap<i64, f64> = BTreeMap::new();
    for r in recs {
        if !r.val.is_finite() {
            continue;
        }
        let unix = r.t + J2000_UNIX_OFFSET;
        let k = (unix / width_s).floor() as i64;
        let signed = r.val;
        let a = signed.abs();
        let e = out.entry(k).or_insert(0.0);
        let keep = if take_abs { a > e.abs() } else { a > e.abs() };
        if keep {
            *e = if take_abs { a } else { signed };
        }
    }
    out
}

fn omni_buckets(recs: &[(f64, f64, u32)], comp: u32, width_s: f64) -> BTreeMap<i64, f64> {
    let mut buckets: BTreeMap<i64, Vec<f64>> = BTreeMap::new();
    for &(t, v, c) in recs {
        if c != comp || !v.is_finite() {
            continue;
        }
        let unix = t + J2000_UNIX_OFFSET;
        let k = (unix / width_s).floor() as i64;
        buckets.entry(k).or_default().push(v);
    }
    let mut out = BTreeMap::new();
    for (k, mut vals) in buckets {
        if let Some(m) = median(&mut vals) {
            out.insert(k, m);
        }
    }
    out
}

fn surrogate_tes(to: &[f32], from: &[f32], lag: usize, n_surr: usize) -> Vec<f64> {
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

fn conditional_surrogate_tes(
    to: &[f32],
    from: &[f32],
    cond: &[f32],
    lag: usize,
    n_surr: usize,
) -> Vec<f64> {
    let mut rng = SURROGATE_SEED.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut vals = Vec::with_capacity(n_surr);
    for _ in 0..n_surr {
        let ys = phase_randomized_surrogate(from, &mut rng);
        if let Some(te) = transfer_entropy_conditional(to, &ys, cond, lag) {
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

fn max_of(vals: &[f64]) -> Option<f64> {
    vals.iter()
        .copied()
        .filter(|v| v.is_finite())
        .fold(None, |acc, v| Some(acc.map_or(v, |m: f64| m.max(v))))
}

fn run_jobs<F>(threads: usize, njobs: usize, f: F) -> Vec<Option<(f64, Option<f64>, f64)>>
where
    F: Fn(usize) -> Option<(f64, Option<f64>, f64)> + Sync,
{
    let mut results: Vec<Option<(f64, Option<f64>, f64)>> = vec![None; njobs];
    if threads > 1 && njobs > 1 {
        let next = std::sync::Arc::new(AtomicUsize::new(0));
        let partials = std::thread::scope(|s| {
            let mut handles = Vec::new();
            for _ in 0..threads.min(njobs) {
                let next = std::sync::Arc::clone(&next);
                let f = &f;
                handles.push(s.spawn(move || {
                    let mut out = Vec::new();
                    loop {
                        let i = next.fetch_add(1, Ordering::Relaxed);
                        if i >= njobs {
                            break;
                        }
                        if let Some(v) = f(i) {
                            out.push((i, v));
                        }
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
        for (i, slot) in results.iter_mut().enumerate() {
            *slot = f(i);
        }
    }
    results
}

struct Row {
    from: String,
    to: String,
    lag: usize,
    te: f64,
    thr: Option<f64>,
    verdict: &'static str,
}

fn verdict(te: f64, thr: Option<f64>, fam: Option<f64>) -> &'static str {
    match (thr, fam) {
        (Some(_), Some(f)) if te > f => "arrow",
        (Some(t), Some(_)) if te > t => "family bound",
        (Some(_), _) => "silent",
        (None, _) => "threshold absent",
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let nur_path = omegaflow::or_temp(
        arg_after(&args, "--nur"),
        "gic_ground/fmi_image_mag_nur.bin",
    );
    let gic_path = omegaflow::or_temp(arg_after(&args, "--gic"), "gic_ground/fmi_gic_1min.bin");
    let omni_path = omegaflow::or_temp(arg_after(&args, "--omni"), "gic_te/omni2_serie_1h.bin");
    let start = arg_after(&args, "--start")
        .and_then(day_from_iso)
        .unwrap_or(12_354);
    let end = arg_after(&args, "--end")
        .and_then(day_from_iso)
        .unwrap_or(12_357);
    let n_surr = arg_after(&args, "--n-surreps")
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|&v| v > 0)
        .unwrap_or(DEFAULT_N_SURR);
    let grain = arg_after(&args, "--grain").unwrap_or("minute");
    let max_lag: usize = arg_after(&args, "--max-lag")
        .and_then(|v| v.parse().ok())
        .unwrap_or(if grain == "hour" { 6 } else { 10 });
    let threads = match arg_after(&args, "--threads") {
        Some(v) => v.parse::<usize>().ok().filter(|&t| t > 0).unwrap_or(1),
        None => std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1),
    };

    let nur_bytes = match std::fs::read(&nur_path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("NUR asset unreadable at {nur_path}: {e} — no measurement");
            std::process::exit(1);
        }
    };
    let gic_bytes = match std::fs::read(&gic_path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("GIC asset unreadable at {gic_path}: {e} — no measurement");
            std::process::exit(1);
        }
    };
    let Some(nur_recs) = geo::parse_bin(geo::MAGIC_IMAGE, &nur_bytes) else {
        eprintln!("NUR asset parses void — no measurement");
        std::process::exit(1);
    };
    let Some(gic_recs) = geo::parse_bin(geo::MAGIC_GIC, &gic_bytes) else {
        eprintln!("GIC asset parses void — no measurement");
        std::process::exit(1);
    };

    let hour_mode = grain == "hour" || grain == "hourly";
    let width = if hour_mode { HOUR } else { MINUTE };
    let take_abs = !args.iter().any(|a| a == "--signed");
    let ground = peak_buckets(&nur_recs, width, take_abs);
    let gic = peak_buckets(&gic_recs, width, take_abs);
    let lo = start as f64 * DAY;
    let hi = end as f64 * DAY;

    let mut keys: Vec<i64> = ground
        .keys()
        .filter(|&&k| {
            let t = k as f64 * width;
            t >= lo && t < hi && gic.contains_key(&k)
        })
        .copied()
        .collect();
    keys.sort_unstable();

    println!("=== Boden -> Strom edge — NUR dB/dt -> FMI GIC ===");
    println!(
        "NUR: {} IMG records, {} {} buckets | GIC: {} records, {} {} buckets",
        nur_recs.len(),
        ground.len(),
        if hour_mode { "h" } else { "min" },
        gic_recs.len(),
        gic.len(),
        if hour_mode { "h" } else { "min" }
    );
    let n = keys.len();
    if n < 16 {
        eprintln!("only {n} common buckets — no measurement");
        std::process::exit(1);
    }
    let first = *keys.first().unwrap();
    let last = *keys.last().unwrap();
    println!(
        "window: {} -> {} | common {} samples -> n {}",
        iso_from_seconds(first as f64 * width),
        iso_from_seconds(last as f64 * width),
        if hour_mode { "h" } else { "min" },
        n
    );
    println!(
        "grain = {} | mode = {} | seed = 0x{SURROGATE_SEED:016X} | n_surr = {n_surr} | lags = 0..={max_lag} {} | threads = {threads}",
        if hour_mode { "hour" } else { "minute" },
        if take_abs { "abs" } else { "signed" },
        if hour_mode { "h" } else { "min" }
    );

    let col = |m: &BTreeMap<i64, f64>| -> Vec<f32> {
        keys.iter()
            .map(|k| *m.get(k).unwrap_or(&f64::NAN) as f32)
            .collect()
    };
    let gic_s = col(&gic);
    let ground_s = col(&ground);

    let pairs: [(&str, &str, &[f32], &[f32]); 2] = [
        ("ground", "GIC", &gic_s, &ground_s),
        ("GIC", "ground", &ground_s, &gic_s),
    ];
    let n_jobs = pairs.len() * (max_lag + 1);
    let results = run_jobs(threads, n_jobs, |i| {
        let (pi, lag) = (i / (max_lag + 1), i % (max_lag + 1));
        let (_, _, to_s, from_s) = pairs[pi];
        let te = transfer_entropy_lag(to_s, from_s, lag)?;
        let vals = surrogate_tes(to_s, from_s, lag, n_surr);
        let thr = mean_plus_2sigma(&vals);
        let partial = max_of(&vals)?;
        Some((te, thr, partial))
    });
    let fam = results
        .iter()
        .flatten()
        .filter_map(|r| Some(r.2))
        .fold(None, |acc: Option<f64>, v| {
            Some(acc.map_or(v, |m| m.max(v)))
        });
    let mut rows: Vec<Row> = Vec::new();
    for i in 0..n_jobs {
        let (pi, lag) = (i / (max_lag + 1), i % (max_lag + 1));
        let Some((te, thr, _)) = results[i] else {
            continue;
        };
        let (from, to, _, _) = pairs[pi];
        rows.push(Row {
            from: from.to_string(),
            to: to.to_string(),
            lag: lag * if hour_mode { 3600 } else { 60 },
            te,
            thr,
            verdict: verdict(te, thr, fam),
        });
    }
    println!(
        "fam = {} (maximum surrogate TE over {} rows)",
        fam.map_or("pending".to_string(), |f| format!("{f:.4e}")),
        n_jobs
    );
    println!(
        "{:<8} -> {:<8} | {:>8} | {:>12} | {:>12} | verdict",
        "from",
        "to",
        if hour_mode { "lag s" } else { "lag s" },
        "TE",
        "thr"
    );
    for r in &rows {
        println!(
            "{:<8} -> {:<8} | {:>8} | {:>12} | {:>12} | {}",
            r.from,
            r.to,
            r.lag,
            format!("{:.4e}", r.te),
            r.thr.map_or("pending".to_string(), |t| format!("{t:.4e}")),
            r.verdict
        );
    }

    if !hour_mode {
        return;
    }

    let omni_bytes = match std::fs::read(&omni_path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("OMNI asset unreadable at {omni_path}: {e} — incremental arm omitted");
            return;
        }
    };
    let Some(omni_recs) = omni2::parse_bin(&omni_bytes) else {
        eprintln!("OMNI asset parses void — incremental arm omitted");
        return;
    };
    let bz = omni_buckets(&omni_recs, omni2::COMP_BZ, HOUR);
    let speed = omni_buckets(&omni_recs, omni2::COMP_V1800, HOUR);
    println!();
    println!("=== incremental arm (hourly): cTE(GIC | ground, L1) vs TE(GIC | L1) ===");
    let per_l1 = max_lag + 1;
    let mut carried = 0usize;
    let l1s: [(&str, &BTreeMap<i64, f64>); 2] = [("Bz", &bz), ("Speed", &speed)];
    for (l1_name, l1map) in l1s {
        let mut ckeys: Vec<i64> = keys
            .iter()
            .copied()
            .filter(|k| l1map.contains_key(k))
            .collect();
        ckeys.sort_unstable();
        let cn = ckeys.len();
        if cn < 16 {
            println!("{l1_name}: only {cn} common hourly samples with L1 — arm omitted (n < 16)");
            continue;
        }
        let ccol = |m: &BTreeMap<i64, f64>| -> Vec<f32> {
            ckeys
                .iter()
                .map(|k| *m.get(k).unwrap_or(&f64::NAN) as f32)
                .collect()
        };
        let cgic = ccol(&gic);
        let cground = ccol(&ground);
        let l1 = ccol(l1map);
        println!();
        println!(
            "L1 = {l1_name} | window {} -> {} | n {cn} | lags 0..={max_lag} h",
            iso_from_seconds(ckeys[0] as f64 * HOUR),
            iso_from_seconds(ckeys[cn - 1] as f64 * HOUR)
        );
        let c_jobs = per_l1 * 2;
        let c_results = run_jobs(threads, c_jobs, |i| {
            let arm = i % 2;
            let lag = i / 2;
            if arm == 0 {
                let te = transfer_entropy_conditional(&cgic, &cground, &l1, lag)?;
                let vals = conditional_surrogate_tes(&cgic, &cground, &l1, lag, n_surr);
                Some((te, mean_plus_2sigma(&vals), max_of(&vals)?))
            } else {
                let te = transfer_entropy_lag(&cgic, &l1, lag)?;
                let vals = surrogate_tes(&cgic, &l1, lag, n_surr);
                Some((te, mean_plus_2sigma(&vals), max_of(&vals)?))
            }
        });
        let cfam = c_results
            .iter()
            .flatten()
            .filter_map(|r| Some(r.2))
            .fold(None, |acc: Option<f64>, v| {
                Some(acc.map_or(v, |m| m.max(v)))
            });
        println!(
            "fam_cond = {} (maximum surrogate TE over the {} conditional/round rows)",
            cfam.map_or("pending".to_string(), |f| format!("{f:.4e}")),
            c_jobs
        );
        println!(
            "{:<5} | {:>4} | {:>12} | {:>12} | {:>12} | {:>12} | {:>12} | verdict",
            "L1", "lag h", "cTE", "cTE thr", "TE(L1)", "TE(L1) thr", "delta"
        );
        for lag in 0..per_l1 {
            let base_i = lag * 2;
            let Some((cte, cthr, _)) = c_results[base_i] else {
                continue;
            };
            let Some((bte, bthr, _)) = c_results[base_i + 1] else {
                continue;
            };
            let delta = cte - bte;
            let v = match (cthr, cfam) {
                (Some(_), Some(f)) if cte > f && delta > 0.0 => "traegt",
                (Some(_), Some(f)) if cte > f => "arrow, kein Zuwachs",
                (Some(t), _) if cte > t => "family bound",
                (Some(_), _) => "silent",
                (None, _) => "threshold absent",
            };
            if v == "traegt" {
                carried += 1;
            }
            println!(
                "{:<5} | {:>4} | {:>12} | {:>12} | {:>12} | {:>12} | {:>12} | {}",
                l1_name,
                lag,
                format!("{cte:.4e}"),
                cthr.map_or("pending".to_string(), |t| format!("{t:.4e}")),
                format!("{bte:.4e}"),
                bthr.map_or("pending".to_string(), |t| format!("{t:.4e}")),
                format!("{delta:.4e}"),
                v
            );
        }
    }
    println!();
    println!("incremental verdict: {carried} rows carry (cTE > fam_cond and cTE > TE(L1))");
}
