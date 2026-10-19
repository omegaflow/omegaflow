
use omegaflow::archivar::{fetch_raw_bytes, geo, omni2};
use omegaflow::lsk::days_from_civil;
use omegaflow::te::{phase_randomized_surrogate, transfer_entropy_lag};
use std::collections::BTreeMap;

const SURROGATE_SEED: u64 = 0x9E37_79B9_7F4A_7C15;
const DEFAULT_N_SURR: usize = 10;
const LAGS: [usize; 2] = [0, 1];
const STRIDE: usize = 3;
const DAY: f64 = 86400.0;
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

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let gic_path = arg_after(&args, "--gic").unwrap_or("/tmp/opencode/gic_te/fmi_gic.bin");
    let omni_path = arg_after(&args, "--omni")
        .unwrap_or("/tmp/opencode/gic_te/omni2_serie_1h.bin");
    let start = arg_after(&args, "--start")
        .and_then(day_from_iso)
        .unwrap_or(10957); // 1999-12-31
    let end = arg_after(&args, "--end")
        .and_then(day_from_iso)
        .unwrap_or(45282); // 2023-12-31
    let n_surr = arg_after(&args, "--n-surreps")
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|&v| v > 0)
        .unwrap_or(DEFAULT_N_SURR);

    let Some(gic_bytes) = read_asset(gic_path, GIC_URL) else {
        eprintln!("fmi_gic.bin: fetch void — no measurement");
        std::process::exit(1);
    };
    let Some(gic_recs) = geo::parse_bin(geo::MAGIC_GIC, &gic_bytes) else {
        eprintln!("fmi_gic.bin: parses void — no measurement");
        std::process::exit(1);
    };
    let Some(omni_bytes) = read_asset(omni_path, OMNI_URL) else {
        eprintln!("omni2_serie_1h.bin: fetch void — no measurement");
        std::process::exit(1);
    };
    let Some(omni_recs) = omni2::parse_bin(&omni_bytes) else {
        eprintln!("omni2_serie_1h.bin: parses void — no measurement");
        std::process::exit(1);
    };

    let gic = daily_gic(&gic_recs);
    let bz = daily_omni(&omni_recs, omni2::COMP_BZ);
    let speed = daily_omni(&omni_recs, omni2::COMP_V1800);
    let density = daily_omni(&omni_recs, omni2::COMP_N1800);

    println!("=== GIC causal-driver probe — measured GIC [A] ===");
    println!(
        "gic: {} records, {} days | omni2: {} records (Bz {} d, Speed {} d, Density {} d)",
        gic_recs.len(),
        gic.len(),
        omni_recs.len(),
        bz.len(),
        speed.len(),
        density.len()
    );

    let mut days: Vec<i64> = gic
        .keys()
        .filter(|d| **d >= start && **d <= end)
        .filter(|d| bz.contains_key(d) && speed.contains_key(d) && density.contains_key(d))
        .copied()
        .collect();
    days.sort_unstable();
    let n_common = days.len();
    days = days.into_iter().step_by(STRIDE).collect();
    let n = days.len();
    if n < 8 {
        eprintln!("only {n} common days — no measurement");
        std::process::exit(1);
    }
    let first = days[0];
    let last = *days.last().unwrap();
    let span_days = last - first + 1;
    println!(
        "window: {} -> {} ({} days, {:.2} years) | common daily samples {} -> stride {} = n {}",
        iso_from_day(first),
        iso_from_day(last),
        span_days,
        span_days as f64 / 365.25,
        n_common,
        STRIDE,
        n
    );
    println!(
        "seed = 0x{SURROGATE_SEED:016X} | n_surr = {n_surr} | lags = {:?} (1 = {} days) | stride = {STRIDE}",
        LAGS, STRIDE
    );

    let col = |m: &BTreeMap<i64, f64>| -> Vec<f32> {
        days.iter()
            .map(|d| *m.get(d).unwrap_or(&f64::NAN) as f32)
            .collect()
    };
    let gic_s = col(&gic);
    let bz_s = col(&bz);
    let speed_s = col(&speed);
    let density_s = col(&density);
    let cols = vec![gic_s, bz_s, speed_s, density_s];

    let pairs = family_pairs(&cols);
    let mut rows: Vec<Row> = Vec::new();
    let mut fam: Option<f64> = None;
    let mut calls: Vec<(usize, usize, f64, Option<f64>, f64)> = Vec::new();
    for (pi, (_, _, to_s, from_s)) in pairs.iter().enumerate() {
        for (li, &lag) in LAGS.iter().enumerate() {
            let Some(te) = transfer_entropy_lag(to_s, from_s, lag) else {
                continue;
            };
            let vals = surrogate_te_values(to_s, from_s, lag, n_surr);
            let thr = mean_plus_2sigma(&vals);
            let partial = vals.iter().copied().fold(f64::NEG_INFINITY, f64::max);
            if partial.is_finite() {
                fam = Some(fam.map_or(partial, |m: f64| m.max(partial)));
            }
            calls.push((pi, li, te, thr, partial));
        }
    }
    for (pi, li, te, thr, _) in calls {
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
            lag: LAGS[li] * STRIDE,
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
        "from", "to", "lag d", "TE", "thr"
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
