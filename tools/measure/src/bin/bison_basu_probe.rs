use omegaflow::archivar::f107::parse_bin as parse_f107;
use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::bison_basu::parse_bin as parse_basu;
use omegaflow::spectral::civil_from_days;
use omegaflow::te::{phase_randomized_surrogate, topological_te_phase, transfer_entropy_lag};

const BASU_CDN: &str =
    "https://github.com/omegaflow/sources/releases/download/ssd.jpl.nasa.gov/bison_basu.bin";
const F107_CDN: &str =
    "https://github.com/omegaflow/sources/releases/download/ssd.jpl.nasa.gov/f107_penticton.bin";
const HALF_WINDOW: i64 = 45;
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
    let band: u8 = arg_value(&args, "--band")
        .and_then(|v| v.parse().ok())
        .unwrap_or(1);
    let band_name = match band {
        0 => "low (1860–2400 µHz)",
        2 => "high (2920–3450 µHz)",
        _ => "mid (2400–2920 µHz)",
    };
    let Some(basu_bytes) = load_bin(&args, "--basu-bin", BASU_CDN) else {
        eprintln!("bison_basu.bin absent or carries no BSN2 contract");
        return;
    };
    let Some(basu) = parse_basu(&basu_bytes) else {
        eprintln!("bison_basu.bin: BSN2-contract void");
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
    let series: Vec<(i64, f64)> = basu
        .iter()
        .filter(|&&(b, ..)| b == band)
        .map(|&(_, d, s, _)| (d, s))
        .collect();
    if series.is_empty() || f107.is_empty() {
        eprintln!("empty series — the probe stays still (0 honored)");
        return;
    }

    let mut series = series;
    series.sort_by_key(|&(d, _)| d);
    let f_bin = |e: i64| -> Option<f32> {
        let mut s = 0.0;
        let mut n = 0usize;
        for &(d, v) in &f107 {
            if (d - e).abs() <= HALF_WINDOW {
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
    let mut s_series: Vec<f32> = Vec::new();
    let mut f_series: Vec<f32> = Vec::new();
    let mut kept: Vec<i64> = Vec::new();
    for &(e, shift) in &series {
        let Some(f) = f_bin(e) else {
            continue;
        };
        s_series.push(shift as f32);
        f_series.push(f);
        kept.push(e);
    }
    if s_series.len() < 16 {
        eprintln!("too few points with F10.7 coverage — the probe stays still (0 honored)");
        return;
    }

    let cadence_days: f64 = if kept.len() >= 2 {
        let mut spacings: Vec<i64> = kept.windows(2).map(|w| w[1] - w[0]).collect();
        spacings.sort();
        spacings[spacings.len() / 2] as f64
    } else {
        91.25
    };
    println!(
        "BiSON-Basu-2012 (Fig.-2 digitization): {} points ({}..{}), band {}, cadence ~{:.0} d (~{:.2} months)",
        s_series.len(),
        span(*kept.first().unwrap()),
        span(*kept.last().unwrap()),
        band_name,
        cadence_days,
        cadence_days / 30.44
    );
    println!();
    println!(
        "TE lag sweep (frequency shift ↔ F10.7, lag 0..{} runs):",
        LAG_MAX
    );
    println!(
        "{:>4} | {:>11} | {:>11} | {:>11}",
        "lag", "TE(S→F)", "TE(F→S)", "fam"
    );
    let mut fam = f64::NEG_INFINITY;
    let mut best: Option<(usize, f64)> = None;
    for lag in 0..=LAG_MAX {
        let seed = SEED ^ (lag as u64).wrapping_mul(0x517C_C1B7_2722_0A95);
        let sf = transfer_entropy_lag(&f_series, &s_series, lag).unwrap_or(f64::NAN);
        let fs = transfer_entropy_lag(&s_series, &f_series, lag).unwrap_or(f64::NAN);
        let mut rng = seed.wrapping_add(0x9E37_79B9_7F4A_7C15);
        for _ in 0..N_SURR {
            let ss = phase_randomized_surrogate(&s_series, &mut rng);
            if let Some(v) = transfer_entropy_lag(&f_series, &ss, lag) {
                if v > fam {
                    fam = v;
                }
            }
        }
        if sf > best.map_or(f64::NEG_INFINITY, |(_, b)| b) {
            best = Some((lag, sf));
        }
        println!(
            "{:>4} | {:>11.4e} | {:>11.4e} | {:>11.4e}",
            lag, sf, fs, fam
        );
    }
    println!();
    if let Some((l, v)) = best {
        let word = if v > fam { "ARROW (over fam)" } else { "still" };
        println!(
            "Peak TE(S→F) at lag {} runs = {:.2} a, TE {:.4e} vs fam {:.4e} — {}",
            l,
            l as f64 * cadence_days / 365.25,
            v,
            fam,
            word
        );
    }
    println!(
        "fam = strongest surrogate TE of the round (multiple-comparison correction). The series is a Fig. digitization (low confidence) with a sparse cycle 21 (~12 points 1978–1986) — silence is honest (0 honored)."
    );
    println!();
    println!("Counter-probe — Takens-embedded TE (phase space, dim 3, order 3, auto-MI-τ):");
    let topo_sf = topological_te_phase(&f_series, &s_series, 3, 3, SEED);
    let topo_fs = topological_te_phase(&s_series, &f_series, 3, 3, SEED);
    for (name, v) in [("TE(S→F)", &topo_sf), ("TE(F→S)", &topo_fs)] {
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
