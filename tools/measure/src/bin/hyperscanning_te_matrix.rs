use std::env;
use std::process::exit;

use omegaflow::te::{
    TeNull, TeStatsParams, conditional_te_stats_lagged_n, transfer_entropy_binned,
};
use omegaflow_measure::eeglab::{
    channel_series, common_average_series, open_set, open_set_bin, open_set_mat, resolve_channel,
};

const DEFAULT_LAGS: usize = 24;
const DEFAULT_SURROGATES: usize = 100;
const DEFAULT_BINS: usize = 4;
const SEED: u64 = 0x9E37_79B9_7F4A_7C15;
const LAG_SEED_MIX: u64 = 0x517C_C1B7_2722_0A95;
const MIN_N: usize = 32;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn arg_values(args: &[String], name: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut i = 0;
    while i + 1 < args.len() {
        if args[i] == name {
            out.push(args[i + 1].clone());
            i += 2;
        } else {
            i += 1;
        }
    }
    out
}

fn usage() {
    eprintln!(
        "hyperscanning_te_matrix — the pairwise transfer-entropy matrix for a hyperscanning group:\n\
         reads N EEG recordings (one per participant), extracts one series each (a named channel\n\
         or the common average), and measures the directional TE for every ordered pair with a\n\
         phase-randomized null (fam-Schwelle = mean + 2 sigma):\n\
         \x20 hyperscanning_te_matrix --eeg <a.set> --eeg <b.set> --eeg <c.set>\n\
         \x20     [--channel <n|label>] [--lags <n>] [--surrogates <n>] [--bins <n>] [--seed <n>]\n\
         \x20     [--max-points <n>]\n\
         each --eeg reads as a text .set naming its .fdt, a MAT-v5 EEG struct, or an EEGB .bin.\n\
         without --channel the common average over all channels is used (the group signal).\n\
         without --max-points each series is read in full; the used n is printed.\n\
         an absent recording reads absent, never a fabricated 0 (0 honored)."
    );
}

fn load_series(path: &str, channel: Option<&str>, max_points: Option<usize>) -> Option<Vec<f32>> {
    let (set, samples) = open_set(path)
        .or_else(|| open_set_mat(path))
        .or_else(|| open_set_bin(path))?;
    let mut series = match channel {
        Some(sel) => {
            let ch = resolve_channel(&set, sel)?;
            channel_series(&samples, &set, ch)?
        }
        None => common_average_series(&samples, &set)?,
    };
    if let Some(cap) = max_points {
        if series.len() > cap {
            series.truncate(cap);
        }
    }
    if series.len() < MIN_N {
        return None;
    }
    Some(series)
}

struct PairFinding {
    lag: usize,
    te: f64,
    fam: Option<f64>,
}

fn best_pair(driver: &[f32], target: &[f32], lags: usize, n_surr: usize, bins: usize, seed: u64) -> Option<PairFinding> {
    let n = driver.len().min(target.len());
    if n < MIN_N {
        return None;
    }
    let lags = lags.min(n.saturating_sub(8));
    if lags == 0 {
        return None;
    }
    let mut best: Option<PairFinding> = None;
    for lag in 1..=lags {
        let lag_seed = seed ^ (lag as u64).wrapping_mul(LAG_SEED_MIX);
        let Some(te) = transfer_entropy_binned(target, driver, lag, bins) else {
            continue;
        };
        let fam = conditional_te_stats_lagged_n(
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
        .map(|(_, _, thr)| thr);
        if best.as_ref().map_or(true, |b| te > b.te) {
            best = Some(PairFinding { lag, te, fam });
        }
    }
    best
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.iter().any(|a| a == "--help" || a == "-h") {
        usage();
        return;
    }
    let paths = arg_values(&args, "--eeg");
    if paths.len() < 2 {
        eprintln!(
            "hyperscanning_te_matrix: {} --eeg path(s) — at least two participants are needed for a pair matrix",
            paths.len()
        );
        exit(2);
    }
    let channel = arg_value(&args, "--channel");
    let lags: usize = arg_value(&args, "--lags")
        .and_then(|v| v.parse().ok())
        .unwrap_or(DEFAULT_LAGS);
    let n_surr: usize = arg_value(&args, "--surrogates")
        .and_then(|v| v.parse().ok())
        .unwrap_or(DEFAULT_SURROGATES);
    let bins: usize = arg_value(&args, "--bins")
        .and_then(|v| v.parse().ok())
        .unwrap_or(DEFAULT_BINS);
    let seed: u64 = arg_value(&args, "--seed")
        .and_then(|v| v.parse().ok())
        .unwrap_or(SEED);
    let max_points: Option<usize> = arg_value(&args, "--max-points")
        .and_then(|v| v.parse().ok());

    let chan_note = match &channel {
        Some(sel) => format!("channel [{sel}]"),
        None => "common average (group signal)".to_string(),
    };
    println!(
        "hyperscanning TE matrix: {} participant(s) | {chan_note} | lags 1..={lags} | surrogates {n_surr} | bins {bins}",
        paths.len()
    );

    let mut series: Vec<Option<Vec<f32>>> = Vec::with_capacity(paths.len());
    for path in &paths {
        match load_series(path, channel.as_deref(), max_points) {
            Some(s) => {
                println!("  [{path}] n = {}", s.len());
                series.push(Some(s));
            }
            None => {
                println!("  [{path}] absent — the recording carries no readable series (0 honored)");
                series.push(None);
            }
        }
    }

    let n = paths.len();
    println!();
    println!("TE(i→j) matrix (row i drives column j):");
    print!("{:>4}", "");
    for j in 0..n {
        print!("{:>12}", format!("j{j}"));
    }
    println!();
    let mut matrix: Vec<Vec<Option<f64>>> = vec![vec![None; n]; n];
    let mut lag_matrix: Vec<Vec<Option<usize>>> = vec![vec![None; n]; n];
    let mut verdict: Vec<Vec<Option<bool>>> = vec![vec![None; n]; n];
    for i in 0..n {
        print!("{:>4}", format!("i{i}"));
        for j in 0..n {
            if i == j {
                print!("{:>12}", "—");
                continue;
            }
            let finding = match (&series[i], &series[j]) {
                (Some(driver), Some(target)) => {
                    best_pair(driver, target, lags, n_surr, bins, seed)
                }
                _ => None,
            };
            match finding {
                Some(f) => {
                    matrix[i][j] = Some(f.te);
                    lag_matrix[i][j] = Some(f.lag);
                    verdict[i][j] = Some(f.fam.is_some_and(|fam| f.te > fam));
                    print!("{:>12}", format!("{:.3e}", f.te));
                }
                None => print!("{:>12}", "absent"),
            }
        }
        println!();
    }

    println!();
    println!("verdict over the phase-randomized fam-Schwelle (mean + 2 sigma):");
    for i in 0..n {
        let words: Vec<String> = (0..n)
            .map(|j| {
                if i == j {
                    "self".to_string()
                } else {
                    match verdict[i][j] {
                        Some(true) => match lag_matrix[i][j] {
                            Some(lag) => format!("i{i}→j{j} ARROW(lag {lag})"),
                            None => format!("i{i}→j{j} ARROW"),
                        },
                        Some(false) => format!("i{i}→j{j} still"),
                        None => format!("i{i}→j{j} absent"),
                    }
                }
            })
            .collect();
        println!("  {}", words.join(" | "));
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
    fn a_delayed_driver_breaks_the_null_in_the_driven_direction() {
        let mut rng = SEED ^ 0xDEAD_BEEF;
        let n = 512usize;
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
        let finding = best_pair(&a, &b, 24, 50, 4, SEED).expect("the pair is measurable");
        assert_eq!(finding.lag, delay);
        let fam = finding.fam.expect("the null is measurable");
        assert!(
            finding.te > fam,
            "a→b breaks the null: TE {} vs fam {fam}",
            finding.te
        );
        let reverse = best_pair(&b, &a, 24, 50, 4, SEED).expect("the reverse pair is measurable");
        let reverse_fam = reverse.fam.expect("the reverse null is measurable");
        assert!(
            reverse.te <= reverse_fam,
            "b→a stays under the null: TE {} vs fam {reverse_fam}",
            reverse.te
        );
    }

    #[test]
    fn a_too_short_pair_reads_absent() {
        let a = vec![0.0f32; 10];
        let b = vec![0.0f32; 10];
        assert!(best_pair(&a, &b, 24, 50, 4, SEED).is_none());
    }

    #[test]
    fn arg_values_collects_every_occurrence() {
        let args: Vec<String> = ["--eeg", "a.set", "--eeg", "b.set", "--lags", "8"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(arg_values(&args, "--eeg"), vec!["a.set", "b.set"]);
        assert_eq!(arg_value(&args, "--lags"), Some("8".to_string()));
    }
}
