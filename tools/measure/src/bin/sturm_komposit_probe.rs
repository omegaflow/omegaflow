use omegaflow::archivar::geo::{self, MAGIC_GIC, MAGIC_GOEV};
use omegaflow::archivar::grid_outage;
use omegaflow::archivar::intermagnet;
use omegaflow::archivar::omni2;
use omegaflow::archivar::wdc_ae;
use std::collections::{BTreeMap, BTreeSet};

const J2000_UNIX_OFFSET: f64 = 946_728_000.0;
const TT_MINUS_UTC: f64 = 69.0;
const HOUR: f64 = 3_600.0;
const DAY: f64 = 86_400.0;
const LAG_MIN: i32 = -24;
const LAG_MAX: i32 = 72;
const QUIET_DST: f64 = -20.0;
const STORM_DST: f64 = -50.0;
const EVENT_GAP_H: i64 = 3;
const MERGE_H: i64 = 48;
const EXCLUDE_DAYS: i64 = 10;
const BOOTSTRAP: usize = 1_000;
const NULL_DRAWS: usize = 500;
const SEED: u64 = 0x9E37_79B9_7F4A_7C15;
const MIN_EVENT_COVER: usize = 11;

const OMNI_PATH: &str = "cdaweb.gsfc.nasa.gov/compiled/omni2_1h_2014_2023.bin";
const WDC_PATH: &str = "wdc.kugi.kyoto-u.ac.jp/wdc_ae.bin";
const ABK_PATH: &str = "imag-data.bgs.ac.uk/abk_dbdt_1h.bin";
const SOD_PATH: &str = "imag-data.bgs.ac.uk/sod_dbdt_1h.bin";
const GIC_PATH: &str = "space.fmi.fi/compiled/fmi_gic.bin";
const ECOD_PATH: &str = "data.openei.org/grid_outage_events.bin";

fn arg_after<'a>(args: &'a [String], flag: &str) -> Option<&'a str> {
    args.iter()
        .position(|a| a == flag)
        .and_then(|i| args.get(i + 1))
        .map(|s| s.as_str())
}

fn civil_from_days(z: i64) -> (i64, i64, i64) {
    let z = z + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    (if m <= 2 { y + 1 } else { y }, m, d)
}

fn iso(unix: f64) -> String {
    let days = (unix / DAY).floor() as i64;
    let secs = unix - days as f64 * DAY;
    let (y, m, d) = civil_from_days(days);
    let hh = (secs / HOUR) as i64;
    let mm = ((secs - hh as f64 * HOUR) / 60.0) as i64;
    format!("{y:04}-{m:02}-{d:02}T{hh:02}:{mm:02}Z")
}

#[derive(Clone, Copy)]
struct Event {
    onset_h: i64,
    min_h: i64,
    dst_min: f64,
    end_h: i64,
}

struct Rng(u64);
impl Rng {
    fn new(seed: u64) -> Self {
        Rng(seed | 1)
    }
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

fn median(v: &mut [f64]) -> f64 {
    v.sort_by(|a, b| a.total_cmp(b));
    let n = v.len();
    if n % 2 == 1 {
        v[n / 2]
    } else {
        0.5 * (v[n / 2 - 1] + v[n / 2])
    }
}

fn percentile(sorted: &[f64], p: f64) -> f64 {
    if sorted.is_empty() {
        return f64::NAN;
    }
    let pos = p * (sorted.len() - 1) as f64;
    let lo = pos.floor() as usize;
    let hi = pos.ceil() as usize;
    let frac = pos - lo as f64;
    sorted[lo] * (1.0 - frac) + sorted[hi] * frac
}

fn hourly_mean(values: impl Iterator<Item = (f64, f64)>) -> BTreeMap<i64, f64> {
    let mut acc: BTreeMap<i64, (f64, usize)> = BTreeMap::new();
    for (t, v) in values {
        if !t.is_finite() || !v.is_finite() {
            continue;
        }
        let k = (t / HOUR).floor() as i64;
        let e = acc.entry(k).or_insert((0.0, 0));
        e.0 += v;
        e.1 += 1;
    }
    acc.into_iter()
        .map(|(k, (s, n))| (k, s / n as f64))
        .collect()
}

fn tdb_to_unix(t: f64) -> f64 {
    t + J2000_UNIX_OFFSET - TT_MINUS_UTC
}

fn detect_events(dst: &BTreeMap<i64, f64>) -> Vec<Event> {
    let keys: Vec<i64> = dst.keys().copied().collect();
    let mut runs: Vec<Event> = Vec::new();
    let mut i = 0usize;
    while i < keys.len() {
        let k = keys[i];
        if dst[&k] < STORM_DST {
            let mut j = i;
            let mut run: Vec<i64> = vec![k];
            while j + 1 < keys.len()
                && keys[j + 1] - keys[j] <= EVENT_GAP_H
                && dst[&keys[j + 1]] < STORM_DST
            {
                j += 1;
                run.push(keys[j]);
            }
            let mut min_h = run[0];
            for &h in &run {
                if dst[&h] < dst[&min_h] {
                    min_h = h;
                }
            }
            runs.push(Event {
                onset_h: k,
                min_h,
                dst_min: dst[&min_h],
                end_h: run[run.len() - 1],
            });
            i = j + 1;
        } else {
            i += 1;
        }
    }
    let mut merged: Vec<Event> = Vec::new();
    for e in runs {
        if let Some(last) = merged.last_mut()
            && e.onset_h - last.onset_h < MERGE_H
        {
            if e.dst_min < last.dst_min {
                last.min_h = e.min_h;
                last.dst_min = e.dst_min;
            }
            last.end_h = e.end_h;
            continue;
        }
        merged.push(e);
    }
    merged
}

fn quiet_hours(dst: &BTreeMap<i64, f64>, events: &[Event]) -> Vec<i64> {
    let mut out = Vec::new();
    for (&h, &v) in dst {
        if v <= QUIET_DST {
            continue;
        }
        let near = events.iter().any(|e| {
            h >= e.onset_h - EXCLUDE_DAYS * 24 && h <= e.end_h + EXCLUDE_DAYS * 24
        });
        if !near {
            out.push(h);
        }
    }
    out
}

struct Stats {
    mean: f64,
    sd: f64,
    n: usize,
}

fn baseline(series: &BTreeMap<i64, f64>, keys: &[i64]) -> Stats {
    let mut vals: Vec<f64> = keys.iter().filter_map(|k| series.get(k).copied()).collect();
    if vals.is_empty() {
        return Stats { mean: 0.0, sd: 0.0, n: 0 };
    }
    let n = vals.len();
    let mean = vals.iter().sum::<f64>() / n as f64;
    let var = vals.iter().map(|v| (v - mean) * (v - mean)).sum::<f64>() / n as f64;
    vals.clear();
    Stats {
        mean,
        sd: var.sqrt(),
        n,
    }
}

struct Row {
    values: Vec<Option<f64>>,
}

struct Result {
    name: &'static str,
    n: usize,
    peak: f64,
    lag: i32,
    area: f64,
    ci_lo: f64,
    ci_hi: f64,
    null95: f64,
}

fn curve_peak(rows: &[Row], idx: &[usize]) -> f64 {
    let mut best = 0.0f64;
    let mut lag = 0usize;
    while lag < (LAG_MAX - LAG_MIN + 1) as usize {
        let mut vals: Vec<f64> = idx
            .iter()
            .filter_map(|&i| rows[i].values[lag])
            .collect();
        if vals.is_empty() {
            lag += 1;
            continue;
        }
        let m = median(&mut vals).abs();
        if m > best {
            best = m;
        }
        lag += 1;
    }
    best
}

fn analyze(name: &'static str, series: &BTreeMap<i64, f64>, events: &[Event], quiet: &[i64], daily: bool) -> Option<Result> {
    let base_keys: Vec<i64> = if daily {
        let set: BTreeSet<i64> = quiet.iter().map(|h| h.div_euclid(24)).collect();
        set.into_iter().collect()
    } else {
        quiet.to_vec()
    };
    let stats = baseline(series, &base_keys);
    if stats.n < 30 || !(stats.sd > 0.0) {
        return None;
    }
    let mut rows: Vec<Row> = Vec::new();
    for e in events {
        let on = if daily { e.onset_h.div_euclid(24) } else { e.onset_h };
        let mut values = Vec::with_capacity((LAG_MAX - LAG_MIN + 1) as usize);
        let mut present = 0usize;
        for lag in LAG_MIN..=LAG_MAX {
            let key = on + if daily { (lag as i64).div_euclid(24) } else { lag as i64 };
            match series.get(&key) {
                Some(&v) => {
                    values.push(Some((v - stats.mean) / stats.sd));
                    present += 1;
                }
                None => values.push(None),
            }
        }
        if present >= MIN_EVENT_COVER {
            rows.push(Row { values });
        }
    }
    let n = rows.len();
    if n < 5 {
        return None;
    }
    let mut med = Vec::with_capacity((LAG_MAX - LAG_MIN + 1) as usize);
    for lag in 0..(LAG_MAX - LAG_MIN + 1) as usize {
        let mut vals: Vec<f64> = rows.iter().filter_map(|r| r.values[lag]).collect();
        med.push(if vals.is_empty() { f64::NAN } else { median(&mut vals) });
    }
    let mut best = 0usize;
    for (i, v) in med.iter().enumerate() {
        if v.abs() > med[best].abs() {
            best = i;
        }
    }
    let peak = med[best].abs();
    let lag = LAG_MIN + best as i32;
    let area: f64 = med.iter().filter(|v| v.is_finite()).map(|v| v.abs()).sum();

    let mut rng = Rng::new(SEED);
    let mut peaks = Vec::with_capacity(BOOTSTRAP);
    let mut idx = Vec::with_capacity(n);
    for _ in 0..BOOTSTRAP {
        idx.clear();
        for _ in 0..n {
            idx.push(rng.below(n));
        }
        peaks.push(curve_peak(&rows, &idx));
    }
    peaks.sort_by(|a, b| a.total_cmp(b));
    let ci_lo = percentile(&peaks, 0.025);
    let ci_hi = percentile(&peaks, 0.975);

    let mut pool = base_keys;
    if pool.len() < n + 5 {
        return None;
    }
    let mut null_peaks = Vec::with_capacity(NULL_DRAWS);
    for _ in 0..NULL_DRAWS {
        for i in 0..n {
            let j = i + rng.below(pool.len() - i);
            pool.swap(i, j);
        }
        let onsets: Vec<i64> = pool[..n].to_vec();
        let mut null_rows: Vec<Row> = Vec::with_capacity(n);
        for on in onsets {
            let mut values = Vec::with_capacity((LAG_MAX - LAG_MIN + 1) as usize);
            for lag in LAG_MIN..=LAG_MAX {
                let key = on + if daily { (lag as i64).div_euclid(24) } else { lag as i64 };
                values.push(series.get(&key).map(|&v| (v - stats.mean) / stats.sd));
            }
            null_rows.push(Row { values });
        }
        let all: Vec<usize> = (0..n).collect();
        null_peaks.push(curve_peak(&null_rows, &all));
    }
    null_peaks.sort_by(|a, b| a.total_cmp(b));
    let null95 = percentile(&null_peaks, 0.95);

    Some(Result {
        name,
        n,
        peak,
        lag,
        area,
        ci_lo,
        ci_hi,
        null95,
    })
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--help" || a == "-h") {
        eprintln!(
            "sturm_komposit_probe [--data-root DIR] [--omni F] [--wdc F] [--abk F] [--sod F] [--gic F] [--ecod F]"
        );
        return;
    }
    let root = arg_after(&args, "--data-root").unwrap_or("sources");
    let path = |flag: &str, rel: &str| -> String {
        match arg_after(&args, flag) {
            Some(p) => p.to_string(),
            None => format!("{root}/{rel}"),
        }
    };
    let read = |p: &str| -> Option<Vec<u8>> {
        match std::fs::read(p) {
            Ok(b) => Some(b),
            Err(e) => {
                eprintln!("{p}: read returned void ({e}) - thread absent");
                None
            }
        }
    };

    let mut wdc_dst: BTreeMap<i64, f64> = BTreeMap::new();
    let mut wdc_ae: BTreeMap<i64, f64> = BTreeMap::new();
    let mut wdc_al: BTreeMap<i64, f64> = BTreeMap::new();
    let mut wdc_au: BTreeMap<i64, f64> = BTreeMap::new();
    let mut wdc_symh: BTreeMap<i64, f64> = BTreeMap::new();
    let wdc_path = path("--wdc", WDC_PATH);
    if let Some(bytes) = read(&wdc_path) {
        if let Some(recs) = wdc_ae::parse_bin(&bytes) {
            let mut dst_vals = Vec::with_capacity(recs.len());
            let mut ae_vals = Vec::with_capacity(recs.len());
            let mut al_vals = Vec::with_capacity(recs.len());
            let mut au_vals = Vec::with_capacity(recs.len());
            let mut sym_vals = Vec::with_capacity(recs.len());
            for r in &recs {
                let keep = |v: Option<f64>| v.filter(|x| x.is_finite() && x.abs() < 99_999.0);
                if let Some(v) = keep(r.dst_nt) {
                    dst_vals.push((r.t_unix, v));
                }
                if let Some(v) = keep(r.ae_nt) {
                    ae_vals.push((r.t_unix, v));
                }
                if let Some(v) = keep(r.al_nt) {
                    al_vals.push((r.t_unix, v));
                }
                if let Some(v) = keep(r.au_nt) {
                    au_vals.push((r.t_unix, v));
                }
                if let Some(v) = keep(r.sym_h_nt) {
                    sym_vals.push((r.t_unix, v));
                }
            }
            wdc_dst = hourly_mean(dst_vals.into_iter());
            wdc_ae = hourly_mean(ae_vals.into_iter());
            wdc_al = hourly_mean(al_vals.into_iter());
            wdc_au = hourly_mean(au_vals.into_iter());
            wdc_symh = hourly_mean(sym_vals.into_iter());
        } else {
            eprintln!("{wdc_path}: WDC bin parses void - magnetic field absent");
        }
    }

    let events = detect_events(&wdc_dst);
    if events.is_empty() {
        eprintln!("no Dst events < -50 nT - the storm is absent, not the probe");
        std::process::exit(1);
    }
    let quiet = quiet_hours(&wdc_dst, &events);

    println!("# Storm composite");
    println!();
    println!("Window: 2014-11-02 .. 2023-12-02");
    println!("Preset threshold: Dst minimum < -50 nT (Kp substitute unused, Dst is present)");
    println!("Events: {}", events.len());
    println!("Quiet baseline hours (Dst > -20 nT, +/-10 d around events excluded): {}", quiet.len());
    println!();
    println!("## Event list");
    println!();
    println!("| # | Onset (UTC) | Dst-Min (UTC) | Dst-Min [nT] |");
    println!("|---:|---|---|---:|");
    for (i, e) in events.iter().enumerate() {
        println!(
            "| {} | {} | {} | {:.0} |",
            i + 1,
            iso(e.onset_h as f64 * HOUR),
            iso(e.min_h as f64 * HOUR),
            e.dst_min
        );
    }
    println!();

    let mut series: Vec<(&'static str, BTreeMap<i64, f64>, bool)> = Vec::new();
    series.push(("Dst", wdc_dst.clone(), false));
    series.push(("Sym-H", wdc_symh, false));
    series.push(("AE", wdc_ae, false));
    series.push(("AL", wdc_al, false));
    series.push(("AU", wdc_au, false));

    let omni_path = path("--omni", OMNI_PATH);
    if let Some(bytes) = read(&omni_path) {
        if let Some(recs) = omni2::parse_bin(&bytes) {
            let mut bz = Vec::new();
            let mut speed = Vec::new();
            let mut density = Vec::new();
            for &(t, v, comp) in &recs {
                let t = tdb_to_unix(t);
                match comp {
                    omni2::COMP_BZ => bz.push((t, v)),
                    omni2::COMP_V1800 => speed.push((t, v)),
                    omni2::COMP_N1800 => density.push((t, v)),
                    _ => {}
                }
            }
            series.push(("Bz", hourly_mean(bz.into_iter()), false));
            series.push(("Solar wind speed", hourly_mean(speed.into_iter()), false));
            series.push(("Solar wind density", hourly_mean(density.into_iter()), false));
        } else {
            eprintln!("{omni_path}: OMNI bin parses void - solar wind absent");
        }
    }

    for (name, flag, rel) in [
        ("ABK dB/dt", "--abk", ABK_PATH),
        ("SOD dB/dt", "--sod", SOD_PATH),
    ] {
        let p = path(flag, rel);
        if let Some(bytes) = read(&p) {
            if let Some(recs) = intermagnet::parse_bin(&bytes) {
                let vals = recs.into_iter().map(|(t, v, _)| (t, v));
                series.push((name, hourly_mean(vals), false));
            } else {
                eprintln!("{p}: dB/dt bin parses void - {name} absent");
            }
        }
    }

    let gic_path = path("--gic", GIC_PATH);
    if let Some(bytes) = read(&gic_path) {
        if let Some(recs) = geo::parse_bin(MAGIC_GIC, &bytes) {
            let vals = recs.into_iter().map(|r| (tdb_to_unix(r.t), r.val));
            series.push(("FMI GIC", hourly_mean(vals), false));
        } else {
            eprintln!("{gic_path}: GIC bin parses void - current absent");
        }
    }

    let ecod_path = path("--ecod", ECOD_PATH);
    if let Some(bytes) = read(&ecod_path) {
        if let Some(recs) = geo::parse_bin(MAGIC_GOEV, &bytes) {
            let mut uniq: BTreeMap<(i64, i64, i64), [Option<f64>; 3]> = BTreeMap::new();
            for r in &recs {
                let key = (
                    (r.t * 10.0).round() as i64,
                    (r.lat * 100_000.0).round() as i64,
                    (r.lon * 100_000.0).round() as i64,
                );
                let slot = uniq.entry(key).or_insert([None; 3]);
                match r.comp {
                    geo::COMP_GOEV_DURATION_H => slot[0] = Some(r.val),
                    geo::COMP_GOEV_MAX_CUSTOMERS => slot[1] = Some(r.val),
                    grid_outage::COMP_CAUSE_CLASS => slot[2] = Some(r.val),
                    _ => {}
                }
            }
            let mut events_day: BTreeMap<i64, f64> = BTreeMap::new();
            let mut customers_day: BTreeMap<i64, f64> = BTreeMap::new();
            for ((t_tdb, _lat, lon), comps) in &uniq {
                let unix = tdb_to_unix(*t_tdb as f64 / 10.0);
                let offset_h = (*lon as f64 / 1_500_000.0).round() as i64;
                let day = ((unix - offset_h as f64 * HOUR) / DAY).floor() as i64;
                *events_day.entry(day).or_insert(0.0) += 1.0;
                if let Some(c) = comps[1] {
                    *customers_day.entry(day).or_insert(0.0) += c;
                }
            }
            let ln_customers: BTreeMap<i64, f64> = customers_day
                .into_iter()
                .map(|(d, c)| (d, (1.0 + c).ln()))
                .collect();
            series.push(("ECOD events/day", events_day, true));
            series.push(("ECOD ln(1+customers)", ln_customers, true));
        } else {
            eprintln!("{ecod_path}: ECOD bin parses void - outages absent");
        }
    }

    println!("## Superposed Epoch (lag -24 h .. +72 h)");
    println!();
    println!("| Thread | n events | Peak \\|z\\| | Lag [h] | Area \\|z\\| | Bootstrap 95 % | Null 95 % | over/under null |");
    println!("|---|---:|---:|---:|---:|---|---:|---|");
    let mut results: Vec<Result> = Vec::new();
    for (name, s, daily) in &series {
        match analyze(name, s, &events, &quiet, *daily) {
            Some(r) => {
                let verdict = if r.peak > r.null95 { "over" } else { "under" };
                println!(
                    "| {} | {} | {:.2} | {:+} | {:.1} | {:.2} .. {:.2} | {:.2} | {} |",
                    r.name, r.n, r.peak, r.lag, r.area, r.ci_lo, r.ci_hi, r.null95, verdict
                );
                results.push(r);
            }
            None => {
                println!("| {} | - | - | - | - | - | - | absent/too short |", name);
            }
        }
    }
    results.sort_by(|a, b| b.peak.total_cmp(&a.peak));
    println!();
    println!("## Ranking by peak-\\|z\\|");
    println!();
    println!("| Rank | Thread | Peak \\|z\\| | Lag [h] | Null 95 % | Factor | n |");
    println!("|---:|---|---:|---:|---:|---:|---:|");
    for (i, r) in results.iter().enumerate() {
        println!(
            "| {} | {} | {:.2} | {:+} | {:.2} | {:.2}x | {} |",
            i + 1,
            r.name,
            r.peak,
            r.lag,
            r.null95,
            r.peak / r.null95,
            r.n
        );
    }
    println!();
    println!("## Absent");
    println!();
    println!("- **TEC**: in the storm set 2014-11-02 .. 2023-12-02 only the native May 2024 inventory exists; absent for the window, not filled with zero.");
    println!("- **Aurora (SSUSI/THEMIS)**: only May 2024 covered; absent in the storm set.");
    println!("- **Kp substitute**: not needed; Dst covers every hour of the window.");
}
