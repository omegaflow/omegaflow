
use omegaflow::te::{phase_randomized_surrogate, transfer_entropy_lag};
use std::time::{SystemTime, UNIX_EPOCH};

const SURROGATE_SEED: u64 = 0x9E37_79B9_7F4A_7C15;
const BURN: usize = 200;

fn arg_after<'a>(args: &'a [String], flag: &str) -> Option<&'a str> {
    args.iter()
        .position(|a| a == flag)
        .and_then(|i| args.get(i + 1))
        .map(|s| s.as_str())
}

fn list_usize(s: &str) -> Vec<usize> {
    s.split(',')
        .filter_map(|p| p.trim().parse::<usize>().ok())
        .collect()
}

fn list_f32(s: &str) -> Vec<f32> {
    s.split(',')
        .filter_map(|p| p.trim().parse::<f32>().ok())
        .collect()
}

fn rng_next(rng: &mut u64) -> f64 {
    *rng = rng
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);
    ((*rng >> 11) as f64) / ((1u64 << 53) as f64)
}

fn gauss(rng: &mut u64) -> f32 {
    loop {
        let u1 = rng_next(rng) as f32 * 2.0 - 1.0;
        let u2 = rng_next(rng) as f32 * 2.0 - 1.0;
        let s = u1 * u1 + u2 * u2;
        if s >= 1.0 || s <= 0.0 {
            continue;
        }
        return u1 * (-2.0 * s.ln() / s).sqrt();
    }
}

fn null_channels(n: usize, a: f32, k: usize, rng: &mut u64) -> Vec<Vec<f32>> {
    let mut x = vec![vec![0f32; BURN + n]; k];
    for step in 1..BURN + n {
        for j in 0..k {
            x[j][step] = a * x[j][step - 1] + gauss(rng);
        }
    }
    (0..k).map(|j| x[j][BURN..].to_vec()).collect()
}

fn surrogate_tes(to: &[f32], from: &[f32], lag: usize, n_surr: usize, seed: u64) -> Vec<f64> {
    let mut rng = seed.wrapping_add(0x9e3779b97f4a7c15);
    let mut vals = Vec::with_capacity(n_surr);
    for _ in 0..n_surr {
        let ys = phase_randomized_surrogate(from, &mut rng);
        if let Some(te) = transfer_entropy_lag(to, &ys, lag) {
            vals.push(te);
        }
    }
    vals
}

fn measure_round(channels: &[Vec<f32>], n_surr: usize, lags: &[usize]) -> (usize, usize, f64) {
    let mut observed: Vec<f64> = Vec::new();
    let mut fam = f64::NEG_INFINITY;
    for drv in 1..channels.len() {
        for &lag in lags {
            for (to, from) in [(0usize, drv), (drv, 0usize)] {
                let Some(te) = transfer_entropy_lag(&channels[to], &channels[from], lag) else {
                    continue;
                };
                for v in surrogate_tes(&channels[to], &channels[from], lag, n_surr, SURROGATE_SEED)
                {
                    if v > fam {
                        fam = v;
                    }
                }
                observed.push(te);
            }
        }
    }
    if observed.is_empty() {
        return (0, 0, f64::NAN);
    }
    let above = observed.iter().filter(|&&te| te > fam).count();
    (above, observed.len(), fam)
}

fn wilson95(k: usize, n: usize) -> (f64, f64) {
    if n == 0 {
        return (f64::NAN, f64::NAN);
    }
    let z = 1.959963984540054;
    let kf = k as f64;
    let nf = n as f64;
    let p = kf / nf;
    let z2 = z * z;
    let denom = 1.0 + z2 / nf;
    let centre = (p + z2 / (2.0 * nf)) / denom;
    let half = z * (p * (1.0 - p) / nf + z2 / (4.0 * nf * nf)).sqrt() / denom;
    (
        100.0 * (centre - half).max(0.0),
        100.0 * (centre + half).min(100.0),
    )
}

fn run_cell(
    channel_sets: &[Vec<Vec<f32>>],
    n_surr: usize,
    lags: &[usize],
    threads: usize,
) -> (usize, usize, usize, usize, Vec<usize>) {
    use std::sync::atomic::{AtomicUsize, Ordering};
    let next = std::sync::Arc::new(AtomicUsize::new(0));
    let partials = std::thread::scope(|s| {
        let mut handles = Vec::new();
        for _ in 0..threads {
            let next = std::sync::Arc::clone(&next);
            handles.push(s.spawn(move || {
                let mut rounds = 0usize;
                let mut rounds_false = 0usize;
                let mut combos_false = 0usize;
                let mut combos_total = 0usize;
                let mut per_trial: Vec<(usize, usize)> = Vec::new();
                loop {
                    let t = next.fetch_add(1, Ordering::Relaxed);
                    if t >= channel_sets.len() {
                        break;
                    }
                    let (above, total, _fam) = measure_round(&channel_sets[t], n_surr, lags);
                    rounds += 1;
                    combos_total += total;
                    combos_false += above;
                    if above > 0 {
                        rounds_false += 1;
                    }
                    per_trial.push((t, above));
                }
                (rounds, rounds_false, combos_false, combos_total, per_trial)
            }));
        }
        handles
            .into_iter()
            .map(|h| h.join().expect("worker thread joins"))
            .collect::<Vec<_>>()
    });
    let mut rounds = 0usize;
    let mut rounds_false = 0usize;
    let mut combos_false = 0usize;
    let mut combos_total = 0usize;
    let mut per_trial: Vec<(usize, usize)> = Vec::new();
    for (r, rf, cf, ct, p) in partials {
        rounds += r;
        rounds_false += rf;
        combos_false += cf;
        combos_total += ct;
        per_trial.extend(p);
    }
    per_trial.sort_by_key(|&(t, _)| t);
    (
        rounds,
        rounds_false,
        combos_false,
        combos_total,
        per_trial.into_iter().map(|(_, a)| a).collect(),
    )
}

fn iso_utc(unix: f64) -> String {
    let total = (unix.max(0.0) / 86400.0).floor() as i64;
    let z = total + 719468;
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

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let n = arg_after(&args, "--n")
        .and_then(|v| v.parse().ok())
        .unwrap_or(1260usize);
    let trials = arg_after(&args, "--trials")
        .and_then(|v| v.parse().ok())
        .unwrap_or(20usize);
    let ns = match arg_after(&args, "--n-surr") {
        Some(s) => list_usize(s),
        None => vec![10],
    };
    let autos = match arg_after(&args, "--a") {
        Some(s) => list_f32(s),
        None => vec![0.0],
    };
    let lag_max = arg_after(&args, "--lag-max")
        .and_then(|v| v.parse().ok())
        .unwrap_or(1usize);
    let lags: Vec<usize> = (0..=lag_max).collect();
    let threads = arg_after(&args, "--threads")
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|&t| t > 0)
        .unwrap_or(1usize);
    let per_trial_print = args.iter().any(|a| a == "--per-trial");

    if args.iter().any(|a| a == "--lag-check") {
        let mut rng = 0x1234_5678_9abc_def0u64;
        let channels = null_channels(256, 0.5, 2, &mut rng);
        for lag in 0..=3 {
            let te = transfer_entropy_lag(&channels[0], &channels[1], lag);
            println!("lag {lag}: TE {te:?}");
        }
        return;
    }

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .ok()
        .map(|d| d.as_secs_f64());
    println!("=== Scalar round-max family-bound calibration ===");
    match now {
        Some(t) => println!("system time: {}", iso_utc(t)),
        None => println!("system time: absent"),
    }
    println!(
        "Estimator: transfer_entropy_lag / phase_randomized_surrogate (canonical scalar path); n={n}; trials={trials}; lags={lags:?}; family = 3 drivers × 2 directions × {} lag(s) = {} combinations",
        lags.len(),
        lags.len() * 6
    );
    println!(
        "Null: four independent AR(1) channels (unit innovation), one target + three drivers; autocorrelation a per cell."
    );
    println!(
        "Rule: arrow iff TE > fam = max surrogate TE over the whole round. Nominal FWER of an exchangeable max rule = 1/(n_surr+1)."
    );
    println!("workers: {threads} thread(s); per-trial print: {per_trial_print}");
    println!();

    for &a in &autos {
        for &n_surr in &ns {
            let mut rng = 0xC2B2_AE3D_85EB_CA6Bu64 ^ ((a.to_bits() as u64) << 32) ^ n_surr as u64;
            let channel_sets: Vec<Vec<Vec<f32>>> = (0..trials)
                .map(|_| null_channels(n, a, 4, &mut rng))
                .collect();
            let t0 = std::time::Instant::now();
            let (rounds, rounds_false, combos_false, combos_total, above_by_trial) =
                run_cell(&channel_sets, n_surr, &lags, threads);
            let wall = t0.elapsed().as_secs_f64();
            let nominal = 1.0 / (n_surr as f64 + 1.0);
            let fwer = 100.0 * rounds_false as f64 / rounds as f64;
            let fpr = 100.0 * combos_false as f64 / combos_total as f64;
            let (lo, hi) = wilson95(rounds_false, rounds);
            println!(
                "a={a} | n_surr={n_surr} | n={n} | trials={rounds} | FWER {rounds_false}/{rounds} = {fwer:.2}% (95% Wilson CI {lo:.2}–{hi:.2}%) | FPR {combos_false}/{combos_total} = {fpr:.2}% | nominal {:.2}% | wall {wall:.1} s | threads {threads}",
                100.0 * nominal
            );
            if per_trial_print {
                for (t, above) in above_by_trial.iter().enumerate() {
                    println!("  trial {t}: {above} combination(s) above fam");
                }
            }
        }
    }
}
