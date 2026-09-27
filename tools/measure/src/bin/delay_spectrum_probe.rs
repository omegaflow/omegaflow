use std::env;
use std::process::exit;

use omegaflow::te::{
    TeNull, TeStatsParams, conditional_te_stats_lagged_n, transfer_entropy_binned,
};

const N: usize = 512;
const DELAY: usize = 5;
const MAX_LAGS: usize = 24;
const N_SURR: usize = 50;
const BINS: usize = 4;
const COUPLING: f64 = 0.95;
const SEED: u64 = 0x9E37_79B9_7F4A_7C15;
const LAG_SEED_MIX: u64 = 0x517C_C1B7_2722_0A95;
const MIN_N: usize = 32;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn usage() {
    eprintln!(
        "delay_spectrum_probe — the delay spectrum (lag-matrix) of a transfer-entropy signal:\n\
         sweeps the lag tau over a deterministic two-channel fixture (driver X, target Y =\n\
         delayed copy of X plus noise), measures TE(X->Y; tau) at every lag, and returns one\n\
         verdict: the lag at maximal TE, checked against the phase-randomized surrogate null\n\
         (mean + 2 sigma) at that lag.\n\
         \x20 delay_spectrum_probe [--n <samples>] [--delay <samples>] [--lags <max>]\n\
         \x20     [--coupling <0..1>] [--surrogates <n>] [--bins <n>] [--seed <n>]\n\
         an absent series or a peak that does not break the null reads absent/still,\n\
         never a fabricated 0 (0 honored)."
    );
}

fn next_rng(rng: &mut u64) -> f64 {
    *rng = rng
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);
    ((*rng >> 33) as f64) / ((u32::MAX >> 1) as f64)
}

fn white(n: usize, rng: &mut u64) -> Vec<f32> {
    (0..n).map(|_| (next_rng(rng) * 2.0 - 1.0) as f32).collect()
}

fn delayed_driver(n: usize, delay: usize, coupling: f64, rng: &mut u64) -> (Vec<f32>, Vec<f32>) {
    let x = white(n, rng);
    let mut y = vec![0.0f32; n];
    for t in 0..n {
        y[t] = if t >= delay {
            (coupling * x[t - delay] as f64 + (next_rng(rng) * 0.05 - 0.025)) as f32
        } else {
            (next_rng(rng) * 0.1 - 0.05) as f32
        };
    }
    (x, y)
}

struct LagPoint {
    lag: usize,
    te: f64,
    thr: Option<f64>,
}

fn delay_spectrum(
    driver: &[f32],
    target: &[f32],
    lags: usize,
    n_surr: usize,
    bins: usize,
    seed: u64,
) -> Option<Vec<LagPoint>> {
    let n = driver.len().min(target.len());
    if n < MIN_N {
        return None;
    }
    let lags = lags.min(n.saturating_sub(8));
    if lags == 0 {
        return None;
    }
    let mut points = Vec::with_capacity(lags);
    for lag in 1..=lags {
        let lag_seed = seed ^ (lag as u64).wrapping_mul(LAG_SEED_MIX);
        let Some(te) = transfer_entropy_binned(target, driver, lag, bins) else {
            continue;
        };
        if !te.is_finite() {
            continue;
        }
        let thr = conditional_te_stats_lagged_n(
            target,
            driver,
            &[],
            TeStatsParams {
                lag,
                max_lag: lag,
                bins,
                seed: lag_seed,
                n_surr,
                null: TeNull::Phase,
            },
        )
        .map(|(_, _, thr)| thr)
        .filter(|t| t.is_finite());
        points.push(LagPoint { lag, te, thr });
    }
    if points.is_empty() {
        None
    } else {
        Some(points)
    }
}

fn peak(points: &[LagPoint]) -> Option<&LagPoint> {
    points.iter().max_by(|a, b| a.te.total_cmp(&b.te))
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.iter().any(|a| a == "--help" || a == "-h") {
        usage();
        return;
    }
    let n: usize = arg_value(&args, "--n")
        .and_then(|v| v.parse().ok())
        .unwrap_or(N);
    let delay: usize = arg_value(&args, "--delay")
        .and_then(|v| v.parse().ok())
        .unwrap_or(DELAY);
    let lags: usize = arg_value(&args, "--lags")
        .and_then(|v| v.parse().ok())
        .unwrap_or(MAX_LAGS);
    let coupling: f64 = arg_value(&args, "--coupling")
        .and_then(|v| v.parse().ok())
        .unwrap_or(COUPLING);
    let n_surr: usize = arg_value(&args, "--surrogates")
        .and_then(|v| v.parse().ok())
        .unwrap_or(N_SURR);
    let bins: usize = arg_value(&args, "--bins")
        .and_then(|v| v.parse().ok())
        .unwrap_or(BINS);
    let seed: u64 = arg_value(&args, "--seed")
        .and_then(|v| v.parse().ok())
        .unwrap_or(SEED);

    println!("=== delay-spectrum probe — the lag-matrix of a transfer-entropy signal ===");
    println!(
        "fixture: driver X = white; target Y_t = {coupling} * X_{{t-{delay}}} + noise | n = {n} | lags 1..={lags} | surrogates {n_surr} (phase) | bins {bins}"
    );

    let mut rng = seed;
    let (x, y) = delayed_driver(n, delay, coupling, &mut rng);

    let fwd = match delay_spectrum(&x, &y, lags, n_surr, bins, seed) {
        Some(p) => p,
        None => {
            eprintln!(
                "X->Y delay spectrum: below the measurement floor (n < {MIN_N}) — no verdict (0 honored)"
            );
            exit(2);
        }
    };

    println!();
    println!("delay spectrum TE(X->Y; tau):");
    println!(
        "{:>4} | {:>12} | {:>12} | {:>6}",
        "lag", "TE", "thr (mean+2sd)", "mark"
    );
    for p in &fwd {
        match p.thr {
            Some(t) => println!(
                "{:>4} | {:>12.4e} | {:>12.4e} | {:>6}",
                p.lag,
                p.te,
                t,
                if p.te > t { "arrow" } else { "still" }
            ),
            None => println!(
                "{:>4} | {:>12.4e} | {:>12} | {:>6}",
                p.lag, p.te, "absent", "absent"
            ),
        }
    }

    let pk = match peak(&fwd) {
        Some(p) => p,
        None => {
            eprintln!("no finite TE over the lag sweep — no verdict (0 honored)");
            exit(2);
        }
    };

    println!();
    match pk.thr {
        Some(thr) if pk.te > thr => {
            println!(
                "verdict: lag at maximal TE = {} samples (planted delay {delay}) | TE({}) = {:.4e} | surrogate threshold = {:.4e} | arrow — the delay signature survives the null",
                pk.lag, pk.lag, pk.te, thr
            );
        }
        Some(thr) => {
            println!(
                "verdict: lag at maximal TE = {} samples (planted delay {delay}) | TE({}) = {:.4e} | surrogate threshold = {:.4e} | still — the peak does not break the null",
                pk.lag, pk.lag, pk.te, thr
            );
        }
        None => {
            println!(
                "verdict: lag at maximal TE = {} samples (planted delay {delay}) | TE({}) = {:.4e} | surrogate threshold absent (0 honored) | still",
                pk.lag, pk.lag, pk.te
            );
        }
    }

    match delay_spectrum(&y, &x, lags, n_surr, bins, seed) {
        Some(rev) => match peak(&rev) {
            Some(rp) => match rp.thr {
                Some(thr) => println!(
                    "reverse Y->X peak: lag {} | TE = {:.4e} | threshold = {:.4e} | {}",
                    rp.lag,
                    rp.te,
                    thr,
                    if rp.te > thr { "arrow" } else { "still" }
                ),
                None => println!("reverse Y->X peak: threshold absent (0 honored)"),
            },
            None => println!("reverse Y->X: no finite TE over the lag sweep (0 honored)"),
        },
        None => println!("reverse Y->X: below the measurement floor — absent (0 honored)"),
    }
    println!();
    println!(
        "note: the per-lag threshold is a single-comparison mean+2sd over phase-randomized surrogates; a lag sweep carries an uncorrected family-wise rate (the multiple-comparison form is the fam-Schwelle in te.rs)."
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn seeded_fixture(seed: u64) -> (Vec<f32>, Vec<f32>) {
        let mut rng = seed;
        delayed_driver(N, DELAY, COUPLING, &mut rng)
    }

    #[test]
    fn recovers_the_planted_delay() {
        let (x, y) = seeded_fixture(SEED);
        let spec =
            delay_spectrum(&x, &y, MAX_LAGS, N_SURR, BINS, SEED).expect("the pair is measurable");
        let pk = peak(&spec).expect("a peak exists");
        assert_eq!(
            pk.lag, DELAY,
            "the planted delay must be the lag of maximal TE"
        );
    }

    #[test]
    fn the_planted_delay_breaks_its_null() {
        let (x, y) = seeded_fixture(SEED);
        let spec =
            delay_spectrum(&x, &y, MAX_LAGS, N_SURR, BINS, SEED).expect("the pair is measurable");
        let pk = peak(&spec).expect("a peak exists");
        let thr = pk.thr.expect("the null is measurable");
        assert!(
            pk.te > thr,
            "the arrow breaks the null: TE {} vs thr {thr}",
            pk.te
        );
    }

    #[test]
    fn reverse_direction_stays_still() {
        let (x, y) = seeded_fixture(SEED);
        let spec = delay_spectrum(&y, &x, MAX_LAGS, N_SURR, BINS, SEED)
            .expect("the reverse pair is measurable");
        let pk = peak(&spec).expect("a peak exists");
        let thr = pk.thr.expect("the reverse null is measurable");
        assert!(
            pk.te <= thr,
            "Y->X stays under the null: TE {} vs thr {thr}",
            pk.te
        );
    }

    #[test]
    fn a_too_short_pair_reads_absent() {
        let x = vec![0.0f32; 10];
        let y = vec![0.0f32; 10];
        assert!(delay_spectrum(&x, &y, MAX_LAGS, N_SURR, BINS, SEED).is_none());
    }

    #[test]
    fn identical_inputs_measure_equal() {
        let (x, y) = seeded_fixture(SEED);
        let a = delay_spectrum(&x, &y, MAX_LAGS, N_SURR, BINS, SEED);
        let b = delay_spectrum(&x, &y, MAX_LAGS, N_SURR, BINS, SEED);
        match (a, b) {
            (Some(pa), Some(pb)) => {
                assert_eq!(
                    pa.len(),
                    pb.len(),
                    "symmetry gate: unequal spectrum lengths"
                );
                for (q, r) in pa.iter().zip(pb.iter()) {
                    assert_eq!(q.lag, r.lag);
                    assert_eq!(q.te, r.te);
                    match (q.thr, r.thr) {
                        (Some(xt), Some(yt)) => assert_eq!(xt, yt),
                        (None, None) => {}
                        _ => panic!("symmetry gate: one threshold present, the other not"),
                    }
                }
            }
            (None, None) => {}
            _ => panic!("symmetry gate: one run measurable, the other not"),
        }
    }
}
