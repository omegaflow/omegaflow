use omegaflow::archivar::f107::parse_bin as parse_f107;
use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::spectral::civil_from_days;
use omegaflow::te::{phase_randomized_surrogate, topological_te_phase, transfer_entropy_lag};
use omegaflow::wso_polar::parse_bin as parse_wso;

const WSO_CDN: &str =
    "https://github.com/omegaflow/sources/releases/download/ssd.jpl.nasa.gov/wso_polar.bin";
const F107_CDN: &str =
    "https://github.com/omegaflow/sources/releases/download/ssd.jpl.nasa.gov/f107_penticton.bin";
const STRIDE_DAYS: i64 = 30;
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
    let Some(wso_bytes) = load_bin(&args, "--wso-bin", WSO_CDN) else {
        eprintln!("wso_polar.bin absent or carries no WSP1 contract");
        return;
    };
    let Some(wso) = parse_wso(&wso_bytes) else {
        eprintln!("wso_polar.bin: WSP1-contract void");
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
    if wso.is_empty() || f107.is_empty() {
        eprintln!("empty series — the probe stays still (0 honored)");
        return;
    }

    let d0 = wso.first().map(|&(d, ..)| d).unwrap();
    let d1 = wso.last().map(|&(d, ..)| d).unwrap();
    let mut p_bin: Vec<Vec<f64>> = vec![Vec::new(); ((d1 - d0) / STRIDE_DAYS + 1) as usize];
    for &(d, _n, _s, avg) in &wso {
        let idx = ((d - d0) / STRIDE_DAYS) as usize;
        if idx < p_bin.len() {
            p_bin[idx].push(avg);
        }
    }
    let mut f_bin: Vec<Vec<f64>> = vec![Vec::new(); p_bin.len()];
    for &(d, v) in &f107 {
        if d < d0 || d > d1 {
            continue;
        }
        let idx = ((d - d0) / STRIDE_DAYS) as usize;
        if idx < f_bin.len() {
            f_bin[idx].push(v);
        }
    }
    let mut p_series: Vec<f32> = Vec::new();
    let mut f_series: Vec<f32> = Vec::new();
    let mut kept: Vec<i64> = Vec::new();
    for i in 0..p_bin.len() {
        if p_bin[i].is_empty() || f_bin[i].is_empty() {
            continue;
        }
        let pm = p_bin[i].iter().sum::<f64>() / p_bin[i].len() as f64;
        let fm = f_bin[i].iter().sum::<f64>() / f_bin[i].len() as f64;
        p_series.push(pm as f32);
        f_series.push(fm as f32);
        kept.push(d0 + i as i64 * STRIDE_DAYS);
    }
    if p_series.len() < 16 {
        eprintln!("too few months with coverage — the probe stays still (0 honored)");
        return;
    }

    println!(
        "WSO polar field: {} months ({}..{}), monthly (30-d bins), 4,5 cycles",
        p_series.len(),
        span(*kept.first().unwrap()),
        span(*kept.last().unwrap())
    );
    println!();
    println!(
        "TE lag sweep (polar field ↔ F10.7, lag 0..{} months):",
        LAG_MAX
    );
    println!(
        "{:>4} | {:>11} | {:>11} | {:>11}",
        "lag", "TE(P→F)", "TE(F→P)", "fam"
    );
    let mut fam = f64::NEG_INFINITY;
    let mut best: Option<(usize, f64)> = None;
    for lag in 0..=LAG_MAX {
        let seed = SEED ^ (lag as u64).wrapping_mul(0x517C_C1B7_2722_0A95);
        let pf = transfer_entropy_lag(&f_series, &p_series, lag).unwrap_or(f64::NAN);
        let fp = transfer_entropy_lag(&p_series, &f_series, lag).unwrap_or(f64::NAN);
        let mut rng = seed.wrapping_add(0x9E37_79B9_7F4A_7C15);
        for _ in 0..N_SURR {
            let ps = phase_randomized_surrogate(&p_series, &mut rng);
            if let Some(v) = transfer_entropy_lag(&f_series, &ps, lag) {
                if v > fam {
                    fam = v;
                }
            }
        }
        if pf > best.map_or(f64::NEG_INFINITY, |(_, b)| b) {
            best = Some((lag, pf));
        }
        println!(
            "{:>4} | {:>11.4e} | {:>11.4e} | {:>11.4e}",
            lag, pf, fp, fam
        );
    }
    println!();
    if let Some((l, v)) = best {
        let word = if v > fam { "ARROW (over fam)" } else { "still" };
        println!(
            "Peak TE(P→F) at lag {} months = {:.2} a, TE {:.4e} vs fam {:.4e} — {}",
            l,
            l as f64 / 12.0,
            v,
            fam,
            word
        );
    }
    println!(
        "fam = strongest surrogate TE of the round (multiple-comparison correction). The precursor relation places the arrow at ~5–6 a (lag 60–72), not at the edge artifact. 4,5 cycles are statistically thin — silence is honest (0 honored)."
    );
    println!();
    println!("Counter-probe — Takens-embedded TE (phase space, dim 3, order 3, auto-MI-τ):");
    let topo_pf = topological_te_phase(&f_series, &p_series, 3, 3, SEED);
    let topo_fp = topological_te_phase(&p_series, &f_series, 3, 3, SEED);
    for (name, v) in [("TE(P→F)", &topo_pf), ("TE(F→P)", &topo_fp)] {
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
