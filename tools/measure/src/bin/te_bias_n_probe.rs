use omegaflow::te::{
    LaggedCond, binned_n_eff, kde_n_eff, topological_te_estimate, transfer_entropy_binned,
    transfer_entropy_ksg_conditional_n, transfer_entropy_lag,
};

const TRANSIENT: usize = 1000;
const COUPLING: f64 = 0.2;
const BASE_SEED: u64 = 0x9E37_79B9_7F4A_7C15;
const SEED_STEP: u64 = 0x517C_C1B7_2722_0A95;
const COND_SEED_XOR: u64 = 0xD1B5_4A32_D192_ED03;
const COND_PHI: f64 = 0.5;
const COND_LAG: usize = 1;
const COND_K: usize = 4;
const REF_N: usize = 10000;
const DEFAULT_REPLICATES: usize = 5;
const DEFAULT_NS: &str = "800,1260,1600,2200,4000,10000";

struct Rep {
    x: Vec<f32>,
    y: Vec<f32>,
    c: Vec<f32>,
}

fn splitmix64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

fn uniform01(state: &mut u64) -> f64 {
    ((splitmix64(state) >> 11) as f64) / ((1u64 << 53) as f64)
}

fn coupled_henon_seeded(n: usize, transient: usize, c: f64, seed: u64) -> (Vec<f32>, Vec<f32>) {
    let total = n + transient;
    let mut st = seed;
    let mut xs = vec![0.0f64; total];
    let mut ys = vec![0.0f64; total];
    xs[0] = 0.1 + 0.02 * (uniform01(&mut st) - 0.5);
    xs[1] = 0.0 + 0.02 * (uniform01(&mut st) - 0.5);
    ys[0] = 0.2 + 0.02 * (uniform01(&mut st) - 0.5);
    ys[1] = 0.1 + 0.02 * (uniform01(&mut st) - 0.5);
    for t in 1..total - 1 {
        xs[t + 1] = 1.4 - xs[t] * xs[t] + 0.3 * xs[t - 1];
        ys[t + 1] = 1.4 - (c * xs[t] * ys[t] + (1.0 - c) * ys[t] * ys[t]) + 0.3 * ys[t - 1];
    }
    let to_f32 = |v: &[f64]| {
        v[transient..]
            .iter()
            .map(|&x| x as f32)
            .collect::<Vec<f32>>()
    };
    (to_f32(&xs), to_f32(&ys))
}

fn ar1_seeded(n: usize, phi: f64, seed: u64) -> Vec<f32> {
    let mut st = seed;
    let mut v = vec![0.0f64; n];
    v[0] = 0.3 * (uniform01(&mut st) - 0.5);
    for t in 1..n {
        v[t] = phi * v[t - 1] + 0.3 * (uniform01(&mut st) - 0.5);
    }
    v.iter().map(|&x| x as f32).collect()
}

fn arg_after<'a>(args: &'a [String], flag: &str) -> Option<&'a str> {
    args.iter()
        .position(|a| a == flag)
        .and_then(|i| args.get(i + 1))
        .map(|s| s.as_str())
}

fn parse_ns(s: &str) -> Vec<usize> {
    let mut ns: Vec<usize> = s
        .split(',')
        .filter_map(|p| p.trim().parse::<usize>().ok())
        .filter(|&n| n >= 9)
        .collect();
    ns.sort_unstable();
    ns.dedup();
    ns
}

fn mean(xs: &[f64]) -> Option<f64> {
    if xs.is_empty() {
        None
    } else {
        Some(xs.iter().sum::<f64>() / xs.len() as f64)
    }
}

fn sample_std(xs: &[f64]) -> Option<f64> {
    if xs.len() < 2 {
        return None;
    }
    let m = mean(xs)?;
    let var = xs.iter().map(|&x| (x - m) * (x - m)).sum::<f64>() / (xs.len() as f64 - 1.0);
    Some(var.sqrt())
}

fn usage() {
    println!(
        "usage: te_bias_n_probe [--replicates N] [--ns a,b,c] [--estimator scalar|embedded|binned|conditional] [--dim N] [--bins N] [--out <path>]"
    );
    println!(
        "runs the scalar Silverman-KDE TE estimator (transfer_entropy_lag, lag 1) by default, the embedded KSG estimator (topological_te_estimate, Takens dim N) with --estimator embedded, the binned histogram estimator (transfer_entropy_binned, lag 1, --bins N, default 4 = the matrix cell width) with --estimator binned, or the conditional embedded KSG estimator (transfer_entropy_ksg_conditional_n, dim 3 + 1 conditioning series, lag 1, k 4) with --estimator conditional, on coupled Hénon maps (Schreiber 2000, true direction X→Y) at several n with fixed-seed replicates."
    );
    println!(
        "the conditional arm conditions on an independent AR(1) series c (φ = {COND_PHI}, innovation 0.3·(u−½), splitmix64 from the replicate seed xored with {COND_SEED_XOR:#018x}), a real covariate, not a phase surrogate."
    );
    println!(
        "reports mean TE(X→Y), its bias against the n = {REF_N} reference, its dispersion across replicates, the reverse mean and the direction accuracy; writes the sheet to --out when given (stdout always)."
    );
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--help" || a == "-h") {
        usage();
        return;
    }
    let reps = arg_after(&args, "--replicates")
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|&v| v > 0)
        .unwrap_or(DEFAULT_REPLICATES);
    let ns = match arg_after(&args, "--ns") {
        Some(s) => parse_ns(s),
        None => parse_ns(DEFAULT_NS),
    };
    if ns.is_empty() {
        eprintln!("--ns names no measurable n (need n >= 9) — no measurement");
        return;
    }
    let out = arg_after(&args, "--out").map(|s| s.to_string());
    let estimator = arg_after(&args, "--estimator").unwrap_or("scalar");
    if estimator != "scalar"
        && estimator != "embedded"
        && estimator != "binned"
        && estimator != "conditional"
    {
        eprintln!(
            "--estimator names no measurable arm (scalar | embedded | binned | conditional) — no measurement"
        );
        return;
    }
    let embedded = estimator == "embedded";
    let binned = estimator == "binned";
    let conditional = estimator == "conditional";
    let dim = arg_after(&args, "--dim")
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|&v| v >= 2)
        .unwrap_or(3);
    let bins = arg_after(&args, "--bins")
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|&v| v >= 2)
        .unwrap_or(4);
    let Some(max_n) = ns.iter().copied().max() else {
        return;
    };
    let n_traj = max_n.max(REF_N);

    let mut sheet = String::new();
    let mut line = |t: &str| {
        println!("{t}");
        sheet.push_str(t);
        sheet.push('\n');
    };

    let estimator_label = if embedded {
        format!("embedded KSG (topological_te_estimate, Takens dim {dim})")
    } else if binned {
        format!("binned histogram (transfer_entropy_binned, lag 1, {bins} bins)")
    } else if conditional {
        format!(
            "conditional embedded KSG (transfer_entropy_ksg_conditional_n, dim 3 + 1 cond [independent AR(1) φ = {COND_PHI}], lag {COND_LAG}, k {COND_K})"
        )
    } else {
        "scalar Silverman-KDE (transfer_entropy_lag, lag 1)".to_string()
    };
    line(&format!(
        "=== TE estimator bias vs n — coupled Hénon (Schreiber 2000), {estimator_label} ==="
    ));
    line(
        "system: x_{n+1} = 1.4 − x_n² + 0.3 x_{n−1};  y_{n+1} = 1.4 − (c·x_n·y_n + (1−c)·y_n²) + 0.3 y_{n−1}",
    );
    line(&format!(
        "coupling c = {COUPLING:.2} (known direction X → Y); transient {TRANSIENT} steps discarded per replicate; estimator = {estimator_label}; the scalar arm is the KDE/Silverman estimator the paper names at docs/paper/gic-causal-driver.md:655-660, the embedded arm the KSG estimator the production flux runs (omega.rs:489)"
    ));
    line(&format!(
        "replicates = {reps} (fixed splitmix64 seeds, base {BASE_SEED:#018x}); reference n = {REF_N} (the validated benchmark size)"
    ));

    let reps_data: Vec<Rep> = (0..reps)
        .map(|r| {
            let seed = BASE_SEED ^ (r as u64).wrapping_mul(SEED_STEP);
            let (x, y) = coupled_henon_seeded(n_traj, TRANSIENT, COUPLING, seed);
            let c = ar1_seeded(n_traj, COND_PHI, seed ^ COND_SEED_XOR);
            Rep { x, y, c }
        })
        .collect();

    let te_fwd = |d: &Rep, n: usize| {
        if conditional {
            transfer_entropy_ksg_conditional_n(
                &d.y[..n],
                &d.x[..n],
                &[LaggedCond {
                    series: &d.c[..n],
                    lag: COND_LAG,
                }],
                COND_LAG,
                COND_K,
            )
        } else if embedded {
            topological_te_estimate(&d.y[..n], &d.x[..n], dim).map(|e| e.te)
        } else if binned {
            transfer_entropy_binned(&d.y[..n], &d.x[..n], 1, bins)
        } else {
            transfer_entropy_lag(&d.y[..n], &d.x[..n], 1)
        }
    };
    let te_rev = |d: &Rep, n: usize| {
        if conditional {
            transfer_entropy_ksg_conditional_n(
                &d.x[..n],
                &d.y[..n],
                &[LaggedCond {
                    series: &d.c[..n],
                    lag: COND_LAG,
                }],
                COND_LAG,
                COND_K,
            )
        } else if embedded {
            topological_te_estimate(&d.x[..n], &d.y[..n], dim).map(|e| e.te)
        } else if binned {
            transfer_entropy_binned(&d.x[..n], &d.y[..n], 1, bins)
        } else {
            transfer_entropy_lag(&d.x[..n], &d.y[..n], 1)
        }
    };

    let ref_fwd: Vec<f64> = reps_data.iter().filter_map(|d| te_fwd(d, REF_N)).collect();
    let ref_rev: Vec<f64> = reps_data.iter().filter_map(|d| te_rev(d, REF_N)).collect();
    let ref_fwd_mean = mean(&ref_fwd).unwrap_or(f64::NAN);
    let ref_rev_mean = mean(&ref_rev).unwrap_or(f64::NAN);
    line(&format!(
        "reference (n = {REF_N}, {} of {reps} replicate(s) measurable): TE(X→Y) mean = {ref_fwd_mean:.4e}, TE(Y→X) mean = {ref_rev_mean:.4e}",
        ref_fwd.len()
    ));
    line("");

    line(&format!(
        "{:>7} | {:>13} | {:>13} | {:>13} | {:>13} | {:>8} | {:>9}",
        "n", "mean_fwd", "bias_fwd", "sd_fwd", "mean_rev", "ratio", "dir_acc"
    ));
    line(&"-".repeat(96));

    for &n in &ns {
        let mut fwd: Vec<f64> = Vec::with_capacity(reps);
        let mut rev: Vec<f64> = Vec::with_capacity(reps);
        let mut neffs: Vec<f64> = Vec::with_capacity(reps);
        let mut binned_neffs: Vec<f64> = Vec::with_capacity(reps);
        let mut dir_ok = 0usize;
        for d in &reps_data {
            if let (Some(f), Some(r)) = (te_fwd(d, n), te_rev(d, n)) {
                if f > r {
                    dir_ok += 1;
                }
                fwd.push(f);
                rev.push(r);
            }
            if let Some(ne) = kde_n_eff(&d.y[..n], &d.x[..n], 1) {
                neffs.push(ne);
            }
            if binned {
                if let Some(ne) = binned_n_eff(&d.y[..n], &d.x[..n], 1, bins) {
                    binned_neffs.push(ne);
                }
            }
        }
        if fwd.is_empty() {
            line(&format!(
                "{:>7} | {:>13} | {:>13} | {:>13} | {:>13} | {:>8} | {:>9}",
                n, "pending", "pending", "pending", "pending", "pending", "pending"
            ));
            continue;
        }
        let mf = mean(&fwd).unwrap_or(f64::NAN);
        let bias = mf - ref_fwd_mean;
        let sd = sample_std(&fwd).unwrap_or(f64::NAN);
        let mr = mean(&rev).unwrap_or(f64::NAN);
        let ratio = if mr.is_finite() && mr > 0.0 {
            mf / mr
        } else {
            f64::NAN
        };
        line(&format!(
            "{:>7} | {:>13.4e} | {:>13.3e} | {:>13.3e} | {:>13.4e} | {:>8.3} | {:>9}",
            n,
            mf,
            bias,
            sd,
            mr,
            ratio,
            format!("{dir_ok}/{}", fwd.len())
        ));
        let neff_mean = mean(&neffs).unwrap_or(f64::NAN);
        line(&format!(
            "{:>7} | kde_n_eff mean = {:.4e} over {} replicate(s)",
            n,
            neff_mean,
            neffs.len()
        ));
        if binned {
            let binned_neff_mean = mean(&binned_neffs).unwrap_or(f64::NAN);
            line(&format!(
                "{:>7} | binned_n_eff mean = {:.4e} over {} replicate(s)",
                n,
                binned_neff_mean,
                binned_neffs.len()
            ));
        }
    }

    line("");
    line(&format!(
        "reading: bias_fwd = mean TE(X→Y) at n minus the n = {REF_N} reference; the direction is correct when TE(X→Y) > TE(Y→X) (dir_acc). A bias whose magnitude grows toward the operating sizes n ≈ 1260–2200 marks the un-corrected small-sample bias the paper leaves open."
    ));

    if let Some(path) = out {
        let p = std::path::Path::new(&path);
        if let Some(parent) = p.parent() {
            if !parent.as_os_str().is_empty() {
                let _ = std::fs::create_dir_all(parent);
            }
        }
        match std::fs::write(p, sheet.as_bytes()) {
            Ok(()) => println!("sheet written: {path} ({} byte(s))", sheet.len()),
            Err(_) => eprintln!(
                "{path} did not take the sheet — the measurement stands in the stdout only"
            ),
        }
    }
}
