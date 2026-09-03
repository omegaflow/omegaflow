use omegaflow::archivar::f107::parse_bin as parse_f107;
use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::gong_series::parse_bin as parse_gong;
use omegaflow::spectral::civil_from_days;
use omegaflow::te::{phase_randomized_surrogate, topological_te_phase, transfer_entropy_lag};
use std::collections::HashMap;

const GONG_CDN: &str =
    "https://github.com/omegaflow/sources/releases/download/ssd.jpl.nasa.gov/gong_series.bin";
const F107_CDN: &str =
    "https://github.com/omegaflow/sources/releases/download/ssd.jpl.nasa.gov/f107_penticton.bin";
const RUN_HALF_WINDOW: i64 = 18;
const LAG_MAX: usize = 72;
const N_SURR: usize = 10;
const SEED: u64 = 0x9E37_79B9_7F4A_7C15;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn span(days: i64) -> String {
    match civil_from_days(days) {
        Some((y, m, d)) => format!("{y}-{m:02}-{d:02}"),
        None => format!("day {days}"),
    }
}

fn load_bin(args: &[String], arg: &str, cdn: &str) -> Option<Vec<u8>> {
    match arg_value(args, arg) {
        Some(p) => std::fs::read(&p).ok(),
        None => fetch_raw_bytes(cdn, 3600),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some(gong_bytes) = load_bin(&args, "--gong-bin", GONG_CDN) else {
        eprintln!("gong_series.bin absent or carries no GTS1 contract");
        return;
    };
    let Some(gong) = parse_gong(&gong_bytes) else {
        eprintln!("gong_series.bin: GTS1-contract void");
        return;
    };
    let Some(f107_bytes) = load_bin(&args, "--f107-bin", F107_CDN) else {
        eprintln!("f107_penticton.bin absent or carries no F107 contract");
        return;
    };
    let Some(f107) = parse_f107(&f107_bytes) else {
        eprintln!("f107_penticton.bin: F107-contract void");
        return;
    };
    if gong.is_empty() || f107.is_empty() {
        eprintln!("empty series — the probe stays still (0 honored)");
        return;
    }

    let mut mode_mean: HashMap<(u32, i32), (f64, usize)> = HashMap::new();
    for &(l, n, _, freq) in &gong {
        let e = mode_mean.entry((l, n)).or_insert((0.0, 0));
        e.0 += freq;
        e.1 += 1;
    }
    let mode_mean: HashMap<(u32, i32), f64> = mode_mean
        .into_iter()
        .map(|(k, (s, c))| (k, s / c as f64))
        .collect();

    let mut by_epoch: HashMap<i64, Vec<f64>> = HashMap::new();
    for &(l, n, days, freq) in &gong {
        if let Some(&m) = mode_mean.get(&(l, n)) {
            by_epoch.entry(days).or_default().push((freq - m) * 1e6);
        }
    }
    let mut epochs: Vec<i64> = by_epoch.keys().copied().collect();
    epochs.sort();
    let f_bin = |e: i64| -> Option<f32> {
        let mut s = 0.0;
        let mut n = 0usize;
        for &(d, v) in &f107 {
            if (d - e).abs() <= RUN_HALF_WINDOW {
                s += v;
                n += 1;
            }
        }
        if n == 0 {
            None
        } else {
            Some((s / n as f64) as f32)
        }
    };
    let mut g_series: Vec<f32> = Vec::new();
    let mut f_series: Vec<f32> = Vec::new();
    let mut mode_counts: Vec<usize> = Vec::new();
    let mut kept_epochs: Vec<i64> = Vec::new();
    for &e in &epochs {
        let Some(f) = f_bin(e) else {
            continue;
        };
        let v = &by_epoch[&e];
        g_series.push((v.iter().sum::<f64>() / v.len() as f64) as f32);
        f_series.push(f);
        mode_counts.push(v.len());
        kept_epochs.push(e);
    }
    if g_series.len() < 8 {
        eprintln!("too few runs with F10.7 coverage — the probe stays still (0 honored)");
        return;
    }

    let cadence_days: f64 = if kept_epochs.len() >= 2 {
        let mut spacings: Vec<i64> = kept_epochs.windows(2).map(|w| w[1] - w[0]).collect();
        spacings.sort();
        spacings[spacings.len() / 2] as f64
    } else {
        36.0
    };
    println!(
        "GONG runs: {} ({:?}..{:?}), {} modes (l 0..2), cadence ~{:.0} d (~{:.2} months), modes per run {}-{}",
        kept_epochs.len(),
        span(*kept_epochs.first().unwrap()),
        span(*kept_epochs.last().unwrap()),
        mode_mean.len(),
        cadence_days,
        cadence_days / 30.44,
        mode_counts.iter().min().unwrap_or(&0),
        mode_counts.iter().max().unwrap_or(&0)
    );

    println!();
    println!(
        "TE lag sweep (frequency shift ↔ F10.7, lag 0..{} runs):",
        LAG_MAX
    );
    println!(
        "{:>4} | {:>11} | {:>11} | {:>11}",
        "lag", "TE(G→F)", "TE(F→G)", "fam"
    );
    let mut fam = f64::NEG_INFINITY;
    let mut best: Option<(usize, f64)> = None;
    for lag in 0..=LAG_MAX {
        let seed = SEED ^ (lag as u64).wrapping_mul(0x517C_C1B7_2722_0A95);
        let gf = transfer_entropy_lag(&f_series, &g_series, lag).unwrap_or(f64::NAN);
        let fg = transfer_entropy_lag(&g_series, &f_series, lag).unwrap_or(f64::NAN);
        let mut rng = seed.wrapping_add(0x9E37_79B9_7F4A_7C15);
        for _ in 0..N_SURR {
            let gs = phase_randomized_surrogate(&g_series, &mut rng);
            if let Some(v) = transfer_entropy_lag(&f_series, &gs, lag) {
                if v > fam {
                    fam = v;
                }
            }
        }
        if gf > best.map_or(f64::NEG_INFINITY, |(_, b)| b) {
            best = Some((lag, gf));
        }
        println!(
            "{:>4} | {:>11.4e} | {:>11.4e} | {:>11.4e}",
            lag, gf, fg, fam
        );
    }
    println!();
    if let Some((l, v)) = best {
        let word = if v > fam { "ARROW (over fam)" } else { "still" };
        println!(
            "Peak TE(G→F) at lag {} runs = {:.2} a, TE {:.4e} vs fam {:.4e} — {}",
            l,
            l as f64 * cadence_days / 365.25,
            v,
            fam,
            word
        );
    }
    println!(
        "fam = strongest surrogate TE of the round (multiple-comparison correction). An edge \"arrow\" (high lag) is the KDE-sweep artifact, no finding. 3 cycles are statistically thin — silence is honest (0 honored)."
    );
    println!();
    println!("Counter-probe — Takens-embedded TE (phase space, dim 3, order 3, auto-MI-τ):");
    let topo_gf = topological_te_phase(&f_series, &g_series, 3, 3, SEED);
    let topo_fg = topological_te_phase(&g_series, &f_series, 3, 3, SEED);
    for (name, v) in [("TE(G→F)", &topo_gf), ("TE(F→G)", &topo_fg)] {
        match v {
            Some(t) => {
                let word = if t.te > t.threshold {
                    "ARROW (over thr)"
                } else {
                    "still"
                };
                println!(
                    "  {}: te {:.4e} vs thr {:.4e} ({} Surrogate, τ_x {} τ_y {}) — {}",
                    name, t.te, t.threshold, t.surrogates_used, t.tau_x, t.tau_y, word
                );
            }
            None => println!(
                "  {name}: no MI-τ — the phase space carries no coupling (still, 0 honored)"
            ),
        }
    }
}
