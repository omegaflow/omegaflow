use omegaflow::archivar::fetch::fetch_raw_bytes;
use omegaflow::archivar::omni2::{COMP_BZ, COMP_V1800, parse_bin as parse_omni};
use omegaflow::callisto::{BAND_COUNT, component_name, parse_bin as parse_callisto};
use std::collections::BTreeMap;

const OMNI2_1H_CDN: &str =
    "https://github.com/omegaflow/sources/releases/download/cdaweb.gsfc.nasa.gov/omni2_serie_1h.bin";
const J2000_UNIX_OFFSET: f64 = 946_728_000.0;
const MIN_MINUTES: usize = 120;
const LAGS_DAYS: [i64; 4] = [0, 1, 2, 3];
const SURROGATES: usize = 500;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn mean(v: &[f64]) -> Option<f64> {
    if v.is_empty() {
        None
    } else {
        Some(v.iter().sum::<f64>() / v.len() as f64)
    }
}

fn pearson(x: &[f64], y: &[f64]) -> Option<f64> {
    let n = x.len().min(y.len());
    if n < 8 {
        return None;
    }
    let mx = mean(&x[..n])?;
    let my = mean(&y[..n])?;
    let mut sxy = 0.0;
    let mut sxx = 0.0;
    let mut syy = 0.0;
    for i in 0..n {
        let dx = x[i] - mx;
        let dy = y[i] - my;
        sxy += dx * dy;
        sxx += dx * dx;
        syy += dy * dy;
    }
    if sxx <= 0.0 || syy <= 0.0 {
        return None;
    }
    Some(sxy / (sxx * syy).sqrt())
}

fn day_of(t: f64) -> i64 {
    (t / 86_400.0).floor() as i64
}


struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
    fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let callisto_path = match arg_value(&args, "--callisto") {
        Some(p) => p,
        None => {
            eprintln!("callisto_edge_probe needs --callisto <bin>");
            std::process::exit(2);
        }
    };
    let Some(callisto_bytes) = std::fs::read(&callisto_path).ok() else {
        eprintln!("{callisto_path}: read void");
        std::process::exit(1);
    };
    let Some(records) = parse_callisto(&callisto_bytes) else {
        eprintln!("{callisto_path}: CALLISTO bin reads void");
        std::process::exit(1);
    };

    let mut per_day: BTreeMap<i64, Vec<[Option<f64>; BAND_COUNT]>> = BTreeMap::new();
    for r in &records {
        let day = day_of(r.t_unix);
        per_day.entry(day).or_default().push(r.bands);
    }
    let mut years: BTreeMap<i64, Vec<i64>> = BTreeMap::new();
    for &day in per_day.keys() {
        let (y, _, _) = civil_from_days(day);
        years.entry(y).or_default().push(day);
    }
    let keep_years = [2020i64, 2022, 2024];
    let mut days: Vec<i64> = Vec::new();
    for y in keep_years {
        if let Some(v) = years.get(&y) {
            days.extend(v.iter().copied());
        }
    }
    days.sort_unstable();
    days.dedup();
    if days.is_empty() {
        eprintln!("no 2020/2022/2024 January days in the asset — unmeasured");
        std::process::exit(1);
    }
    let mut activity: Vec<[Option<f64>; BAND_COUNT]> = Vec::with_capacity(days.len());
    let mut kept_days: Vec<i64> = Vec::with_capacity(days.len());
    for &day in &days {
        let Some(rows) = per_day.get(&day) else {
            continue;
        };
        if rows.len() < MIN_MINUTES {
            continue;
        }
        let mut act = [None; BAND_COUNT];
        for b in 0..BAND_COUNT {
            let vals: Vec<f64> = rows.iter().filter_map(|r| r[b]).collect();
            if let Some(m) = mean(&vals) {
                act[b] = Some(m);
            }
        }
        activity.push(act);
        kept_days.push(day);
    }
    if kept_days.len() < 20 {
        eprintln!("only {} usable days — the edge stays unmeasured", kept_days.len());
        std::process::exit(1);
    }

    let omni_bytes = match arg_value(&args, "--omni") {
        Some(path) => match std::fs::read(&path) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("{path}: read void ({e}) — the edge stays unmeasured");
                std::process::exit(1);
            }
        },
        None => match fetch_raw_bytes(OMNI2_1H_CDN) {
            Some(b) => b,
            None => {
                eprintln!("OMNI 1h CDN asset fetch void — the edge stays unmeasured");
                std::process::exit(1);
            }
        },
    };
    let Some(omni) = parse_omni(&omni_bytes) else {
        eprintln!("OMNI 1h CDN asset reads void — the edge stays unmeasured");
        std::process::exit(1);
    };
    let mut bz_day: BTreeMap<i64, f64> = BTreeMap::new();
    let mut v_day: BTreeMap<i64, f64> = BTreeMap::new();
    for (t, v, comp) in omni {
        let day = day_of(t + J2000_UNIX_OFFSET);
        if comp == COMP_BZ {
            let e = bz_day.entry(day).or_insert(v);
            if v < *e {
                *e = v;
            }
        } else if comp == COMP_V1800 {
            v_day.entry(day).or_insert(v);
        }
    }

    println!(
        "radio days {} ({}..{}), OMNI days {} / {}; lags {:?} d",
        kept_days.len(),
        kept_days[0],
        kept_days[kept_days.len() - 1],
        bz_day.len(),
        v_day.len(),
        LAGS_DAYS
    );
    if let (Some(a), Some(b)) = (bz_day.keys().next(), bz_day.keys().next_back()) {
        println!("OMNI Bz day range {a}..{b}");
    }
    let mut observed = 0.0f64;
    let mut best = String::new();
    for b in 0..BAND_COUNT {
        for lag in LAGS_DAYS {
            for metric in ["Bzmin", "Vmean"] {
                let mut xs = Vec::new();
                let mut ys = Vec::new();
                for (i, &day) in kept_days.iter().enumerate() {
                    let target = day + lag;
                    let y = if metric == "Bzmin" {
                        bz_day.get(&target).copied()
                    } else {
                        v_day.get(&target).copied()
                    };
                    if let (Some(y), Some(x)) = (y, activity[i][b]) {
                        xs.push(x);
                        ys.push(y);
                    }
                }
                if let Some(r) = pearson(&xs, &ys) {
                    if r.abs() > observed {
                        observed = r.abs();
                        best = format!(
                            "{} lag {} d {} (n={})",
                            component_name(b as u32).unwrap_or("?"),
                            lag,
                            metric,
                            xs.len()
                        );
                    }
                }
            }
        }
    }
    if best.is_empty() {
        eprintln!("no paired radio/OMNI days — the edge stays unmeasured");
        std::process::exit(1);
    }

    let mut rng = Rng(0x9E37_79B9_7F4A_7C15);
    let mut null = Vec::with_capacity(SURROGATES);
    for _ in 0..SURROGATES {
        let mut shifted = activity.clone();
        for y in keep_years {
            let idx: Vec<usize> = kept_days
                .iter()
                .enumerate()
                .filter(|(_, d)| {
                    let (yy, _, _) = civil_from_days(**d);
                    yy == y
                })
                .map(|(i, _)| i)
                .collect();
            if idx.is_empty() {
                continue;
            }
            let shift = rng.below(idx.len());
            for (k, &i) in idx.iter().enumerate() {
                shifted[i] = activity[idx[(k + shift) % idx.len()]];
            }
        }
        let mut mx = 0.0f64;
        for b in 0..BAND_COUNT {
            for lag in LAGS_DAYS {
                for metric in ["Bzmin", "Vmean"] {
                    let mut xs = Vec::new();
                    let mut ys = Vec::new();
                    for (i, &day) in kept_days.iter().enumerate() {
                        let target = day + lag;
                        let y = if metric == "Bzmin" {
                            bz_day.get(&target).copied()
                        } else {
                            v_day.get(&target).copied()
                        };
                        if let (Some(y), Some(x)) = (y, shifted[i][b]) {
                            xs.push(x);
                            ys.push(y);
                        }
                    }
                    if let Some(r) = pearson(&xs, &ys) {
                        mx = mx.max(r.abs());
                    }
                }
            }
        }
        null.push(mx);
    }
    null.sort_by(|a, b| a.total_cmp(b));
    let q95 = null[(null.len() as f64 * 0.95) as usize];
    let exceed = null.iter().filter(|v| **v >= observed).count();
    let p = (exceed as f64 + 1.0) / (null.len() as f64 + 1.0);
    println!(
        "observed max |r| = {observed:.4} at {best}; surrogate 95th = {q95:.4}, p = {p:.4} ({} surrogates)",
        null.len()
    );
    println!(
        "verdict: {}",
        if observed > q95 {
            "traegt (observed max clears the phase/block-shift null)"
        } else {
            "traegt nicht (observed max sits inside the surrogate null)"
        }
    );
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
