use std::collections::BTreeMap;
use std::env;

use omegaflow::archivar::hapi_csv::parse_iso_seconds;
use omegaflow::te::{
    LaggedCond, TeNull, TeStatsParams, conditional_te_stats_lagged_n,
    transfer_entropy_conditional_binned_n,
};
use omegaflow::wdc_ae::parse_bin;

const SEED: u64 = 0x5eed_de2;
const FILL_ABS: f64 = 1.0e30;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn parse_arg<T: std::str::FromStr>(args: &[String], name: &str, default: T) -> T {
    arg_value(args, name)
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

fn usable(v: f64) -> bool {
    v.is_finite() && v.abs() < FILL_ABS
}

struct Acc {
    sum: f64,
    n: usize,
}

impl Acc {
    fn empty() -> Self {
        Acc { sum: 0.0, n: 0 }
    }

    fn add(&mut self, v: f64) {
        self.sum += v;
        self.n += 1;
    }

    fn mean(&self) -> Option<f64> {
        (self.n > 0).then(|| self.sum / self.n as f64)
    }
}

fn bin_of(t: f64, bin_s: f64) -> i64 {
    (t / bin_s).floor() as i64
}

fn read_kp_csv(path: &str, bin_s: f64) -> BTreeMap<i64, f64> {
    let text = match std::fs::read_to_string(path) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("de2_conditional_te: Kp csv unread {path}: {e}");
            std::process::exit(2);
        }
    };
    let mut acc: BTreeMap<i64, Acc> = BTreeMap::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let cells: Vec<&str> = line.split(',').map(str::trim).collect();
        if cells.len() < 2 {
            continue;
        }
        let Some(t) = parse_iso_seconds(cells[0]) else {
            continue;
        };
        let Ok(v) = cells[1].parse::<f64>() else {
            continue;
        };
        if usable(v) {
            acc.entry(bin_of(t, bin_s))
                .or_insert(Acc::empty())
                .add(v);
        }
    }
    acc.into_iter()
        .filter_map(|(k, a)| a.mean().map(|m| (k, m)))
        .collect()
}

fn read_cond(path: &str, bin_s: f64, field: &str) -> BTreeMap<i64, f64> {
    let bytes = match std::fs::read(path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("de2_conditional_te: conditioner bin unread {path}: {e}");
            std::process::exit(2);
        }
    };
    let Some(records) = parse_bin(&bytes) else {
        eprintln!("de2_conditional_te: {path} is not a wdc_ae bin (magic/length)");
        std::process::exit(2);
    };
    let mut acc: BTreeMap<i64, Acc> = BTreeMap::new();
    for r in records {
        let v = match field {
            "dst" => r.dst_nt,
            "symd" => r.sym_d_nt,
            _ => r.ae_nt,
        };
        if let Some(v) = v
            && usable(v)
            && r.t_unix.is_finite()
        {
            acc.entry(bin_of(r.t_unix, bin_s))
                .or_insert(Acc::empty())
                .add(v);
        }
    }
    acc.into_iter()
        .filter_map(|(k, a)| a.mean().map(|m| (k, m)))
        .collect()
}

fn read_de2(path: &str, bin_s: f64) -> (BTreeMap<i64, f64>, BTreeMap<i64, f64>) {
    let text = match std::fs::read_to_string(path) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("de2_conditional_te: DE-2 CSV unread {path}: {e}");
            std::process::exit(2);
        }
    };
    let mut ion: BTreeMap<i64, Acc> = BTreeMap::new();
    let mut neutral: BTreeMap<i64, Acc> = BTreeMap::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let cells: Vec<&str> = line.split(',').map(str::trim).collect();
        if cells.len() != 7 {
            continue;
        }
        let Some(t) = parse_iso_seconds(cells[0]) else {
            continue;
        };
        let mut v = [0.0f64; 6];
        let mut ok = true;
        for (slot, cell) in v.iter_mut().zip(&cells[1..]) {
            match cell.parse::<f64>() {
                Ok(x) => *slot = x,
                Err(_) => {
                    ok = false;
                    break;
                }
            }
        }
        if !ok {
            continue;
        }
        let (nve, ive) = (v[1], v[4]);
        if !usable(nve) || !usable(ive) {
            continue;
        }
        let key = bin_of(t, bin_s);
        neutral.entry(key).or_insert(Acc::empty()).add(nve);
        ion.entry(key).or_insert(Acc::empty()).add(ive);
    }
    let means = |m: BTreeMap<i64, Acc>| {
        m.into_iter()
            .filter_map(|(k, a)| a.mean().map(|x| (k, x)))
            .collect()
    };
    (means(ion), means(neutral))
}

fn demean(v: &mut [f32]) {
    let n = v.len() as f64;
    if n == 0.0 {
        return;
    }
    let mean = v.iter().map(|&x| x as f64).sum::<f64>() / n;
    for x in v {
        *x = (*x as f64 - mean) as f32;
    }
}

fn iso(t: f64) -> String {
    let secs = t.floor() as i64;
    let days = secs.div_euclid(86400);
    let sod = secs.rem_euclid(86400);
    let (y, m, d) = civil_from_days(days);
    format!(
        "{y:04}-{m:02}-{d:02}T{:02}:{:02}:{:02}Z",
        sod / 3600,
        (sod % 3600) / 60,
        sod % 60
    )
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
    (if m <= 2 { y + 1 } else { y }, m, d)
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let csv = match arg_value(&args, "--csv") {
        Some(v) => v,
        None => "data/cdaweb.gsfc.nasa.gov/de2_ua16s_all.csv".to_string(),
    };
    let ae = match arg_value(&args, "--ae") {
        Some(v) => v,
        None => "data/wdc.kugi.kyoto-u.ac.jp/wdc_ae.bin".to_string(),
    };
    let bin_s: f64 = parse_arg(&args, "--bin-seconds", 300.0);
    if !(bin_s.is_finite() && bin_s > 0.0) {
        eprintln!("de2_conditional_te: --bin-seconds must be positive");
        std::process::exit(2);
    }
    let cond_field = match arg_value(&args, "--cond") {
        Some(v) => v,
        None => "ae".to_string(),
    };
    if !["ae", "dst", "symd", "kp"].contains(&cond_field.as_str()) {
        eprintln!("de2_conditional_te: --cond names no ae|dst|symd|kp");
        std::process::exit(2);
    }
    let cond_arg: f64 = parse_arg(&args, "--cond-seconds", 0.0);
    let cond_default = if cond_field == "kp" { 10800.0 } else { bin_s };
    let cond_s = if cond_arg == 0.0 { cond_default } else { cond_arg };
    if !(cond_s.is_finite() && cond_s >= bin_s) {
        eprintln!("de2_conditional_te: --cond-seconds must be finite and >= --bin-seconds");
        std::process::exit(2);
    }
    let max_lag: usize = parse_arg(&args, "--max-lag", 3);
    let bins: usize = parse_arg(&args, "--bins", 8);
    let n_surr: usize = parse_arg(&args, "--surr", 50);

    let (ion, neutral) = read_de2(&csv, bin_s);
    let ae_bins = if cond_field == "kp" {
        let kp = match arg_value(&args, "--kp-csv") {
            Some(v) => v,
            None => {
                eprintln!("de2_conditional_te: --cond kp needs --kp-csv <time,kp,...>");
                std::process::exit(2);
            }
        };
        read_kp_csv(&kp, cond_s)
    } else {
        read_cond(&ae, cond_s, &cond_field)
    };
    let ae_at = |k: i64| -> Option<f64> {
        let coarse = bin_of(k as f64 * bin_s, cond_s);
        ae_bins.get(&coarse).copied()
    };

    let mut keys: Vec<i64> = ion
        .keys()
        .filter(|k| neutral.contains_key(*k) && ae_at(**k).is_some())
        .copied()
        .collect();
    keys.sort_unstable();
    if keys.len() < 32 {
        println!(
            "de2_conditional_te: only {} joint bins (bin={}s) -> not measurable, no fabrication",
            keys.len(),
            bin_s
        );
        return;
    }

    let mut nve: Vec<f32> = keys.iter().map(|k| neutral[k] as f32).collect();
    let mut ive: Vec<f32> = keys.iter().map(|k| ion[k] as f32).collect();
    let ae_series: Vec<f32> = keys
        .iter()
        .map(|k| ae_at(*k).expect("keys were filtered on ae_at") as f32)
        .collect();
    let mut ae_c: Vec<f32> = ae_series.clone();
    demean(&mut nve);
    demean(&mut ive);
    demean(&mut ae_c);

    let mut gaps: Vec<i64> = keys.windows(2).map(|w| w[1] - w[0]).collect();
    gaps.sort_unstable();
    let segments = 1 + gaps.iter().filter(|&&g| g > 2).count();
    let gap_median = gaps[gaps.len() / 2];
    let gap_max = gaps[gaps.len() - 1];
    let span = format!(
        "{} .. {}",
        iso(keys[0] as f64 * bin_s),
        iso((keys[keys.len() - 1] + 1) as f64 * bin_s)
    );

    println!(
        "de2_conditional_te: n={} bin={}s cond={} cond_bin={}s span={} segments={} gap_median={}s gap_max={}s bins={} surr={} null=Arx est=Binned seed={:#x}",
        keys.len(),
        bin_s,
        cond_field,
        cond_s,
        span,
        segments,
        gap_median as f64 * bin_s,
        gap_max as f64 * bin_s,
        bins,
        n_surr,
        SEED
    );

    let cond = [LaggedCond {
        series: &ae_c,
        lag: 0,
    }];
    let empty: [LaggedCond; 0] = [];
    let mut any_cond_pass = false;

    for lag in 1..=max_lag {
        for (label, driver, target) in [("NVE->IVE", &nve, &ive), ("IVE->NVE", &ive, &nve)] {
            let obs_cond = transfer_entropy_conditional_binned_n(driver, target, &cond, lag, bins);
            let stats_cond = conditional_te_stats_lagged_n(
                driver,
                target,
                &cond,
                TeStatsParams {
                    lag,
                    max_lag,
                    bins,
                    seed: SEED,
                    n_surr,
                    null: TeNull::Arx,
                },
            );
            let obs_raw = transfer_entropy_conditional_binned_n(driver, target, &empty, lag, bins);
            let stats_raw = conditional_te_stats_lagged_n(
                driver,
                target,
                &empty,
                TeStatsParams {
                    lag,
                    max_lag,
                    bins,
                    seed: SEED,
                    n_surr,
                    null: TeNull::Arx,
                },
            );
            let fmt = |obs: Option<f64>, st: Option<(f64, f64, f64)>| -> (String, bool) {
                match (obs, st) {
                    (Some(o), Some((m, sd, thr))) => {
                        let pass = o > thr;
                        (
                            format!(
                                "te={o:+.6} null_mean={m:+.6} null_sd={sd:.6} thr={thr:+.6} pass={}",
                                if pass { "yes" } else { "no" }
                            ),
                            pass,
                        )
                    }
                    _ => ("te=absent".to_string(), false),
                }
            };
            let (cond_line, cond_pass) = fmt(obs_cond, stats_cond);
            any_cond_pass |= cond_pass;
            let (raw_line, _) = fmt(obs_raw, stats_raw);
            println!(
                "LAG {:>5}s {:>8} cond[{}]: {}",
                lag as f64 * bin_s,
                label,
                cond_field,
                cond_line
            );
            println!(
                "LAG {:>5}s {:>8} raw     : {}",
                lag as f64 * bin_s,
                label,
                raw_line
            );
        }
    }

    println!(
        "de2_conditional_te: any conditioned lag above the ARX null: {}",
        if any_cond_pass { "yes" } else { "no" }
    );
}
