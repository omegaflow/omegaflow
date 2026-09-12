use std::env;
use std::process::exit;

use omegaflow::te::{
    conditional_te_stats_lagged_n, surrogate_stats_phase_n, transfer_entropy_conditional_binned_n,
    transfer_entropy_lag, TeNull,
};

const DEFAULT_LAG_MAX: usize = 24;
const DEFAULT_SURROGATES: usize = 100;
const DEFAULT_BINS: usize = 4;
const SEED: u64 = 0x9E37_79B9_7F4A_7C15;
const MIN_N: usize = 32;
const LAG_SEED_MIX: u64 = 0x517C_C1B7_2722_0A95;

enum Verdict {
    Arrow,
    Still,
}

impl Verdict {
    fn label(self) -> &'static str {
        match self {
            Verdict::Arrow => "ARROW",
            Verdict::Still => "still",
        }
    }
}

fn verdict(te: f64, threshold: f64) -> Verdict {
    if te > threshold {
        Verdict::Arrow
    } else {
        Verdict::Still
    }
}

fn parse_series(text: &str) -> Option<Vec<f32>> {
    let mut out = Vec::new();
    for tok in text.split_whitespace() {
        if let Ok(v) = tok.parse::<f32>() {
            if v.is_finite() {
                out.push(v);
            }
        }
    }
    if out.is_empty() {
        None
    } else {
        Some(out)
    }
}

fn read_series(path: &str) -> Option<Vec<f32>> {
    let text = std::fs::read_to_string(path).ok()?;
    parse_series(&text)
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn usage() {
    eprintln!(
        "placebo_pair_eeg_probe — the placebo control for needle XI (Placebo):\n\
         pair-EEG transfer entropy with a phase-randomized null (fam-Schwelle = mean + 2 sigma),\n\
         plus conditional TE against a named common-cause channel (bedingte TE):\n\
         \x20 placebo_pair_eeg_probe --a <eeg_a> --b <eeg_b> [--c <common_cause>] [--lags <n>]\n\
         \x20              [--surrogates <n>] [--bins <n>] [--seed <n>]\n\
         each series is whitespace-separated finite f32 values, one index per time step.\n\
         the paired-EEG substrate is not yet harvested; the probe reads named inputs, and an\n\
         absent file is reported as pending, never as 0 (0 honored, absent stays absent)."
    );
}

fn fmt_opt(v: Option<f64>) -> String {
    match v {
        Some(x) if x.is_finite() => format!("{x:.4e}"),
        Some(_) => "absent".to_string(),
        None => "absent".to_string(),
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.iter().any(|a| a == "--help" || a == "-h") {
        usage();
        return;
    }
    let (Some(pa), Some(pb)) = (arg_value(&args, "--a"), arg_value(&args, "--b")) else {
        eprintln!("--a <eeg_a> --b <eeg_b> required (see --help)");
        exit(2);
    };
    let pc = arg_value(&args, "--c");
    let lag_max: usize = arg_value(&args, "--lags")
        .and_then(|v| v.parse().ok())
        .unwrap_or(DEFAULT_LAG_MAX);
    let n_surr: usize = arg_value(&args, "--surrogates")
        .and_then(|v| v.parse().ok())
        .unwrap_or(DEFAULT_SURROGATES);
    let bins: usize = arg_value(&args, "--bins")
        .and_then(|v| v.parse().ok())
        .unwrap_or(DEFAULT_BINS);
    let seed: u64 = arg_value(&args, "--seed")
        .and_then(|v| v.parse().ok())
        .unwrap_or(SEED);

    let (Some(a), Some(b)) = (read_series(&pa), read_series(&pb)) else {
        println!(
            "pending — the paired-EEG substrate is absent: {} / {} carries no finite series (0 honored, absent stays absent)",
            pa, pb
        );
        return;
    };
    let mut n = a.len().min(b.len());
    let mut c: Option<Vec<f32>> = None;
    if let Some(pc) = &pc {
        match read_series(pc) {
            Some(series) => {
                n = n.min(series.len());
                c = Some(series);
            }
            None => {
                println!(
                    "pending — the common-cause channel is absent at {} (0 honored, absent stays absent); bivariate pair-EEG still runs",
                    pc
                );
            }
        }
    }
    let a = &a[..n];
    let b = &b[..n];
    let c = c.map(|mut s| {
        s.truncate(n);
        s
    });
    if n < MIN_N {
        println!("n = {n} < {MIN_N} -> no finding (underdetermination, no fabrication)");
        return;
    }

    let lag_max = lag_max.min(n.saturating_sub(8));
    if lag_max == 0 {
        println!("n = {n} leaves no lag sweep (0 honored)");
        return;
    }

    println!(
        "pair-EEG placebo control: {} ↔ {} | n = {} | lags 1..={} | surrogates = {} | bins = {}",
        pa, pb, n, lag_max, n_surr, bins
    );
    match &pc {
        Some(p) => println!("common-cause channel (bedingte TE): {p}"),
        None => println!("common-cause channel (bedingte TE): absent -> pending (0 honored)"),
    }
    println!();

    println!(
        "{:>4} | {:>11} | {:>11} | {:>5} | {:>11} | {:>11} | {:>5}",
        "lag", "TE(a→b)", "fam", "a→b", "TE(b→a)", "fam", "b→a"
    );
    let mut best_ab: Option<(usize, f64)> = None;
    let mut best_ba: Option<(usize, f64)> = None;
    let mut arrow_ab = 0usize;
    let mut arrow_ba = 0usize;
    for lag in 1..=lag_max {
        let lag_seed = seed ^ (lag as u64).wrapping_mul(LAG_SEED_MIX);
        let te_ab = transfer_entropy_lag(b, a, lag);
        let fam_ab = surrogate_stats_phase_n(b, a, lag, lag_seed, n_surr).map(|(_, _, thr)| thr);
        let te_ba = transfer_entropy_lag(a, b, lag);
        let fam_ba = surrogate_stats_phase_n(a, b, lag, lag_seed, n_surr).map(|(_, _, thr)| thr);
        let v_ab = match (te_ab, fam_ab) {
            (Some(t), Some(f)) => {
                if t > f {
                    arrow_ab += 1;
                }
                verdict(t, f)
            }
            _ => Verdict::Still,
        };
        let v_ba = match (te_ba, fam_ba) {
            (Some(t), Some(f)) => {
                if t > f {
                    arrow_ba += 1;
                }
                verdict(t, f)
            }
            _ => Verdict::Still,
        };
        if let Some(t) = te_ab {
            if t > best_ab.map_or(f64::NEG_INFINITY, |(_, v)| v) {
                best_ab = Some((lag, t));
            }
        }
        if let Some(t) = te_ba {
            if t > best_ba.map_or(f64::NEG_INFINITY, |(_, v)| v) {
                best_ba = Some((lag, t));
            }
        }
        println!(
            "{:>4} | {:>11} | {:>11} | {:>5} | {:>11} | {:>11} | {:>5}",
            lag,
            fmt_opt(te_ab),
            fmt_opt(fam_ab),
            v_ab.label(),
            fmt_opt(te_ba),
            fmt_opt(fam_ba),
            v_ba.label()
        );
    }
    println!();
    println!(
        "fam-Schwelle = mean + 2 sigma over {n_surr} phase-randomized surrogates (null control)."
    );
    println!(
        "bivariate: {} arrow(s) a→b, {} arrow(s) b→a over fam-Schwelle.",
        arrow_ab, arrow_ba
    );
    for (dir, best) in [("a→b", best_ab), ("b→a", best_ba)] {
        match best {
            Some((lag, te)) => println!("  honest lag ({dir}) = {lag} steps at TE {te:.4e}"),
            None => println!("  honest lag ({dir}) = absent (no TE carried by the sweep)"),
        }
    }

    println!();
    if let Some(c) = &c {
        println!("bedingte TE (TE(A→B | C), binned estimator, phase null):");
        println!(
            "{:>4} | {:>11} | {:>11} | {:>5} | {:>11} | {:>11} | {:>5}",
            "lag", "TE(a→b|c)", "fam", "a→b", "TE(b→a|c)", "fam", "b→a"
        );
        let mut cond_arrow_ab = 0usize;
        let mut cond_arrow_ba = 0usize;
        for lag in 1..=lag_max {
            let lag_seed = seed ^ (lag as u64).wrapping_mul(LAG_SEED_MIX);
            let te_ab = transfer_entropy_conditional_binned_n(b, a, &[c], lag, bins);
            let fam_ab = conditional_te_stats_lagged_n(
                b,
                a,
                &[c],
                lag,
                lag,
                bins,
                lag_seed,
                n_surr,
                TeNull::Phase,
            )
            .map(|(_, _, thr)| thr);
            let te_ba = transfer_entropy_conditional_binned_n(a, b, &[c], lag, bins);
            let fam_ba = conditional_te_stats_lagged_n(
                a,
                b,
                &[c],
                lag,
                lag,
                bins,
                lag_seed,
                n_surr,
                TeNull::Phase,
            )
            .map(|(_, _, thr)| thr);
            let v_ab = match (te_ab, fam_ab) {
                (Some(t), Some(f)) => {
                    if t > f {
                        cond_arrow_ab += 1;
                    }
                    verdict(t, f)
                }
                _ => Verdict::Still,
            };
            let v_ba = match (te_ba, fam_ba) {
                (Some(t), Some(f)) => {
                    if t > f {
                        cond_arrow_ba += 1;
                    }
                    verdict(t, f)
                }
                _ => Verdict::Still,
            };
            println!(
                "{:>4} | {:>11} | {:>11} | {:>5} | {:>11} | {:>11} | {:>5}",
                lag,
                fmt_opt(te_ab),
                fmt_opt(fam_ab),
                v_ab.label(),
                fmt_opt(te_ba),
                fmt_opt(fam_ba),
                v_ba.label()
            );
        }
        println!();
        println!(
            "bedingte TE: {} arrow(s) a→b|c, {} arrow(s) b→a|c over fam-Schwelle.",
            cond_arrow_ab, cond_arrow_ba
        );
        if arrow_ab + arrow_ba == 0 {
            println!("Stille: no bivariate arrow breaks fam-Schwelle — the placebo holds (the silence is the finding).");
        } else if cond_arrow_ab + cond_arrow_ba == 0 {
            println!("the bivariate arrow does not survive the common cause C — C carries the arrow (no unexplained carrier).");
        } else {
            println!("an arrow survives the common cause C — the form is named, the carrier is unexplained (never a word the force gate refuses).");
        }
    } else {
        println!("bedingte TE: pending — no common-cause channel named (--c); no exclusion without a measured channel (0 honored).");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn next_rng(rng: &mut u64) -> f64 {
        *rng = rng
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        ((*rng >> 33) as f64) / ((u32::MAX >> 1) as f64)
    }

    fn white(n: usize, rng: &mut u64) -> Vec<f32> {
        (0..n).map(|_| (next_rng(rng) * 2.0 - 1.0) as f32).collect()
    }

    #[test]
    fn parse_series_reads_finite_tokens_and_skips_the_rest() {
        let text = "1.0\n-2.5  3.25\nNaN\ninf\nnot_a_number\n4.0e1\n";
        let v = parse_series(text).expect("the finite tokens parse");
        assert_eq!(v, vec![1.0f32, -2.5, 3.25, 40.0]);
        assert!(parse_series("NaN\ninf\n").is_none());
        assert!(parse_series("").is_none());
    }

    #[test]
    fn delayed_coupling_breaks_fam_in_the_driven_direction() {
        let mut rng = SEED ^ 0xDEAD_BEEF;
        let n = 256usize;
        let delay = 5usize;
        let a = white(n, &mut rng);
        let mut b = vec![0.0f32; n];
        for t in 0..n {
            b[t] = if t >= delay {
                (0.95 * a[t - delay] as f64 + (next_rng(&mut rng) * 0.05 - 0.025)) as f32
            } else {
                (next_rng(&mut rng) * 0.1 - 0.05) as f32
            };
        }
        let lag_seed = SEED ^ (delay as u64).wrapping_mul(LAG_SEED_MIX);
        let te = transfer_entropy_lag(&b, &a, delay).expect("the coupled TE is measurable");
        let (_, _, fam) =
            surrogate_stats_phase_n(&b, &a, delay, lag_seed, 50).expect("the null is measurable");
        assert!(
            te > fam,
            "the driven direction must break fam-Schwelle: TE {te} vs fam {fam}"
        );
    }

    #[test]
    fn irrelevant_condition_leaves_the_direct_arrow() {
        let mut rng = SEED ^ 0xC1B7_2722_0A95_517C;
        let n = 512usize;
        let delay = 4usize;
        let a = white(n, &mut rng);
        let c = white(n, &mut rng);
        let mut b = vec![0.0f32; n];
        for t in 0..n {
            b[t] = if t >= delay {
                (0.95 * a[t - delay] as f64 + (next_rng(&mut rng) * 0.05 - 0.025)) as f32
            } else {
                (next_rng(&mut rng) * 0.1 - 0.05) as f32
            };
        }
        let lag_seed = SEED ^ (delay as u64).wrapping_mul(LAG_SEED_MIX);
        let te = transfer_entropy_conditional_binned_n(&b, &a, &[&c], delay, 4)
            .expect("the conditional TE is measurable");
        let (_, _, fam) = conditional_te_stats_lagged_n(
            &b,
            &a,
            &[&c],
            delay,
            delay,
            4,
            lag_seed,
            50,
            TeNull::Phase,
        )
        .expect("the conditional null is measurable");
        assert!(
            te > fam,
            "the direct arrow must survive an irrelevant condition: TE {te} vs fam {fam}"
        );
    }
}
