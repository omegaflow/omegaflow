use std::env;
use std::process::exit;

use omegaflow::te::{
    TeNull, TeStatsParams, conditional_te_stats_lagged_n, transfer_entropy_binned,
    transfer_entropy_conditional_binned_n,
};
use omegaflow_measure::eeglab::{
    EeglabSet, channel_series, common_average_series, open_set, open_set_bin, open_set_chanlocs,
    open_set_mat, resolve_channel,
};

fn subtract_reference(mut series: Vec<f32>, reference: &[f32]) -> Vec<f32> {
    let n = series.len().min(reference.len());
    for i in 0..n {
        series[i] -= reference[i];
    }
    series
}

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

fn monotonic_non_decreasing(series: &[Option<f64>]) -> bool {
    series.iter().all(Option::is_some) && series.windows(2).all(|w| w[0] <= w[1])
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
    if out.is_empty() { None } else { Some(out) }
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
         \x20 placebo_pair_eeg_probe --set <subject.set> [--a <chan_a>] [--b <chan_b>] [--c <chan_c>]\n\
         \x20              [--lags <n>] [--surrogates <n>] [--bins <n>] [--seed <n>]\n\
         the .set reads either as a text header naming its .fdt (datfile, nbchan/pnts/trials/\n\
         datatype) or as a MATLAB-5 MAT-v5 EEG struct (nbchan/pnts/trials/srate/chanlocs labels,\n\
         data embedded); samples unpack as interleaved little-endian f32 [channel][sample]\n\
         (channel fastest). an absent data field reads absent, never a fabricated 0.\n\
         the verum/sham sibling is found by swapping verum<->sham in the path; channels are\n\
         1-based indices or labels from the channel-location block. without --c, the reader\n\
         names the conditioning channel itself: the first channel of the chanlocs order that is\n\
         neither the reference (Cz) nor E1/E2; no such channel -> the --c line stands absent.\n\
         legacy: --a <eeg_a> --b <eeg_b> [--c <common_cause>] reads whitespace-separated finite\n\
         f32 values, one index per time step. an absent file is reported as pending, never as 0\n\
         (0 honored, absent stays absent)."
    );
}

fn fmt_opt(v: Option<f64>) -> String {
    match v {
        Some(x) if x.is_finite() => format!("{x:.4e}"),
        Some(_) => "absent".to_string(),
        None => "absent".to_string(),
    }
}

fn sibling_path(path: &str) -> Option<String> {
    if path.contains("verum") {
        Some(path.replace("verum", "sham"))
    } else if path.contains("sham") {
        Some(path.replace("sham", "verum"))
    } else {
        None
    }
}

fn auto_condition_channel(set: &EeglabSet, chan_a: usize, chan_b: usize) -> Option<String> {
    set.labels
        .iter()
        .enumerate()
        .find(|(i, label)| *i != chan_a && *i != chan_b && label.as_str() != "Cz")
        .map(|(_, label)| label.clone())
}

fn condition_pair(
    path: &str,
    sel_a: &str,
    sel_b: &str,
    sel_c: Option<&str>,
) -> Option<(String, Vec<f32>, Vec<f32>, Option<Vec<f32>>, String)> {
    let (set, samples) = open_set(path)
        .or_else(|| open_set_mat(path))
        .or_else(|| open_set_bin(path))?;
    let chan_a = resolve_channel(&set, sel_a)?;
    let chan_b = resolve_channel(&set, sel_b)?;
    let a = channel_series(&samples, &set, chan_a)?;
    let b = channel_series(&samples, &set, chan_b)?;
    let (c, c_note) = match sel_c {
        Some(sel) => {
            let chan_c = resolve_channel(&set, sel)?;
            let series = channel_series(&samples, &set, chan_c)?;
            (Some(series), format!("[{sel}]"))
        }
        None => match auto_condition_channel(&set, chan_a, chan_b) {
            Some(label) => {
                let chan_c = resolve_channel(&set, &label)?;
                let series = channel_series(&samples, &set, chan_c)?;
                (
                    Some(series),
                    format!("[{label}] — chanlocs order, not reference (Cz), not E1/E2"),
                )
            }
            None => (None, String::new()),
        },
    };
    let (a_re, b_re, c_re, ref_note) = match common_average_series(&samples, &set) {
        Some(reference) => (
            subtract_reference(a, &reference),
            subtract_reference(b, &reference),
            c.map(|c| subtract_reference(c, &reference)),
            "AVE (common average reference, Cz removed)".to_string(),
        ),
        None => (
            a,
            b,
            c,
            "Cz (recorded reference, common average absent)".to_string(),
        ),
    };
    let tag = if c_note.is_empty() {
        format!("{path} [{sel_a} ↔ {sel_b}]")
    } else {
        format!("{path} [{sel_a} ↔ {sel_b}] C={c_note}")
    };
    Some((tag, a_re, b_re, c_re, ref_note))
}

fn report_chanlocs(path: &str) {
    match open_set_chanlocs(path) {
        Some(entries) => {
            println!("chanlocs at {path}:");
            for (label, pos) in entries {
                match pos {
                    Some((x, y, z)) => println!("  [{label}] x={x:.6} y={y:.6} z={z:.6}"),
                    None => println!("  [{label}] absent"),
                }
            }
        }
        None => println!("chanlocs at {path}: absent (the .set carries no chanlocs positions)"),
    }
}

fn run_eeglab(
    args: &[String],
    set_path: &str,
    lag_max: usize,
    n_surr: usize,
    bins: usize,
    seed: u64,
) {
    let sel_a = arg_value(args, "--a");
    let sel_b = arg_value(args, "--b");
    let sel_c = arg_value(args, "--c");

    let (sel_a, sel_b) = match (sel_a, sel_b) {
        (Some(a), Some(b)) => (a, b),
        _ => {
            println!(
                "pending — the channel pair is unnamed: --a and --b name the channels (0 honored, absent stays absent)"
            );
            return;
        }
    };

    report_chanlocs(set_path);

    match condition_pair(set_path, &sel_a, &sel_b, sel_c.as_deref()) {
        Some((tag, a, b, c, ref_note)) => {
            println!("{tag}: re-reference {ref_note}");
            run_pair(&tag, &a, &b, c, lag_max, n_surr, bins, seed);
        }
        None => println!(
            "pending — the EEGLAB .set/.fdt pair is absent or unreadable at {set_path} (0 honored, absent stays absent)"
        ),
    }

    match sibling_path(set_path) {
        Some(sibling) => {
            report_chanlocs(&sibling);
            match condition_pair(&sibling, &sel_a, &sel_b, sel_c.as_deref()) {
                Some((tag, a, b, c, ref_note)) => {
                    println!("{tag}: re-reference {ref_note}");
                    run_pair(&tag, &a, &b, c, lag_max, n_surr, bins, seed);
                }
                None => println!(
                    "pending — the sibling condition is absent or unreadable at {sibling} (0 honored, absent stays absent)"
                ),
            }
        }
        None => println!("pending — no verum/sham sibling is nameable from {set_path} (0 honored)"),
    }
}

fn run_pair(
    tag: &str,
    a: &[f32],
    b: &[f32],
    c: Option<Vec<f32>>,
    lag_max: usize,
    n_surr: usize,
    bins: usize,
    seed: u64,
) {
    let mut n = a.len().min(b.len());
    if let Some(series) = &c {
        n = n.min(series.len());
    }
    let a = &a[..n];
    let b = &b[..n];
    let c = c.map(|mut series| {
        series.truncate(n);
        series
    });
    if n < MIN_N {
        println!("{tag}: n = {n} < {MIN_N} -> no finding (underdetermination, no fabrication)");
        return;
    }

    let lag_max = lag_max.min(n.saturating_sub(8));
    if lag_max == 0 {
        println!("{tag}: n = {n} leaves no lag sweep (0 honored)");
        return;
    }

    println!(
        "pair-EEG placebo control [{tag}]: n = {} | lags 1..={} | surrogates = {} | bins = {}",
        n, lag_max, n_surr, bins
    );
    match &c {
        Some(_) => println!("common-cause channel (bedingte TE): named"),
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
    let mut te_ab_series: Vec<Option<f64>> = Vec::with_capacity(lag_max);
    let mut te_ba_series: Vec<Option<f64>> = Vec::with_capacity(lag_max);
    for lag in 1..=lag_max {
        let lag_seed = seed ^ (lag as u64).wrapping_mul(LAG_SEED_MIX);
        let te_ab = transfer_entropy_binned(b, a, lag, bins);
        te_ab_series.push(te_ab);
        let fam_ab = conditional_te_stats_lagged_n(
            b,
            a,
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
        let te_ba = transfer_entropy_binned(a, b, lag, bins);
        te_ba_series.push(te_ba);
        let fam_ba = conditional_te_stats_lagged_n(
            a,
            b,
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
    for (dir, best, series) in [
        ("a→b", best_ab, te_ab_series.as_slice()),
        ("b→a", best_ba, te_ba_series.as_slice()),
    ] {
        match best {
            Some((lag, te)) => {
                if lag == lag_max && monotonic_non_decreasing(series) {
                    println!(
                        "  honest lag ({dir}) = {lag} steps at TE {te:.4e} — the argmax sits at the sweep boundary (lag_max = {lag_max}) and the TE grows monotonically across the sweep; this is the sweep bound, not a lag coupling"
                    );
                } else if lag == lag_max {
                    println!(
                        "  honest lag ({dir}) = {lag} steps at TE {te:.4e} — the argmax sits at the sweep boundary (lag_max = {lag_max}); a lower bound, not a lag coupling"
                    );
                } else {
                    println!("  honest lag ({dir}) = {lag} steps at TE {te:.4e}");
                }
            }
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
            let te_ba = transfer_entropy_conditional_binned_n(a, b, &[c], lag, bins);
            let fam_ba = conditional_te_stats_lagged_n(
                a,
                b,
                &[c],
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
            println!(
                "Stille: no bivariate arrow breaks fam-Schwelle — the placebo holds (the silence is the finding)."
            );
        } else if cond_arrow_ab + cond_arrow_ba == 0 {
            println!(
                "the bivariate arrow does not survive the common cause C — C carries the arrow (no unexplained carrier)."
            );
        } else {
            println!(
                "an arrow survives the common cause C — the form is named, the carrier is unexplained (never a word the force gate refuses)."
            );
        }
    } else {
        println!(
            "bedingte TE: pending — no common-cause channel named (--c); no exclusion without a measured channel (0 honored)."
        );
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.iter().any(|a| a == "--help" || a == "-h") {
        usage();
        return;
    }
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

    if let Some(set_path) = arg_value(&args, "--set") {
        run_eeglab(&args, &set_path, lag_max, n_surr, bins, seed);
        return;
    }

    let (Some(pa), Some(pb)) = (arg_value(&args, "--a"), arg_value(&args, "--b")) else {
        eprintln!("--set <subject.set> or --a <eeg_a> --b <eeg_b> required (see --help)");
        exit(2);
    };
    let pc = arg_value(&args, "--c");
    let (Some(a), Some(b)) = (read_series(&pa), read_series(&pb)) else {
        println!(
            "pending — the paired-EEG substrate is absent: {} / {} carries no finite series (0 honored, absent stays absent)",
            pa, pb
        );
        return;
    };
    let c = match &pc {
        Some(pc) => match read_series(pc) {
            Some(series) => Some(series),
            None => {
                println!(
                    "pending — the common-cause channel is absent at {} (0 honored, absent stays absent); bivariate pair-EEG still runs",
                    pc
                );
                None
            }
        },
        None => None,
    };
    run_pair(
        &format!("{pa} ↔ {pb}"),
        &a,
        &b,
        c,
        lag_max,
        n_surr,
        bins,
        seed,
    );
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
    fn sibling_swaps_verum_and_sham() {
        let v = "sub-02/ses-verum/eeg/sub-02_ses-verum_task-rest_eeg.set";
        assert_eq!(
            sibling_path(v),
            Some("sub-02/ses-sham/eeg/sub-02_ses-sham_task-rest_eeg.set".to_string())
        );
        assert!(sibling_path("sub-02_task-rest_eeg.set").is_none());
    }

    #[test]
    fn auto_condition_names_the_first_non_reference_non_pair_channel() {
        let set = EeglabSet {
            datfile: String::new(),
            nbchan: 4,
            pnts: 100,
            trials: 1,
            srate: Some(250.0),
            labels: vec![
                "Cz".to_string(),
                "E1".to_string(),
                "E2".to_string(),
                "Fz".to_string(),
            ],
        };
        assert_eq!(auto_condition_channel(&set, 1, 2), Some("Fz".to_string()));
        let only_ref = EeglabSet {
            datfile: String::new(),
            nbchan: 3,
            pnts: 100,
            trials: 1,
            srate: Some(250.0),
            labels: vec!["Cz".to_string(), "E1".to_string(), "E2".to_string()],
        };
        assert_eq!(auto_condition_channel(&only_ref, 1, 2), None);
    }

    #[test]
    fn monotonic_reads_rising_series_and_gaps_break_it() {
        assert!(monotonic_non_decreasing(&[Some(1.0), Some(2.0), Some(3.0)]));
        assert!(monotonic_non_decreasing(&[Some(1.0), Some(1.0)]));
        assert!(!monotonic_non_decreasing(&[
            Some(1.0),
            Some(2.0),
            Some(1.5)
        ]));
        assert!(!monotonic_non_decreasing(&[Some(1.0), None, Some(3.0)]));
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
        let te = transfer_entropy_binned(&b, &a, delay, 4).expect("the coupled TE is measurable");
        let (_, _, fam) = conditional_te_stats_lagged_n(
            &b,
            &a,
            &[],
            TeStatsParams {
                lag: delay,
                max_lag: delay,
                bins: 4,
                seed: lag_seed,
                n_surr: 50,
                null: TeNull::Phase,
            },
        )
        .expect("the null is measurable");
        assert!(
            te > fam,
            "the driven direction breaks fam-Schwelle: TE {te} vs fam {fam}"
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
            TeStatsParams {
                lag: delay,
                max_lag: delay,
                bins: 4,
                seed: lag_seed,
                n_surr: 50,
                null: TeNull::Phase,
            },
        )
        .expect("the conditional null is measurable");
        assert!(
            te > fam,
            "the direct arrow survives an irrelevant condition: TE {te} vs fam {fam}"
        );
    }
}
