use std::env;
use std::process::exit;

use omegaflow::te::{
    coherent_phase_surrogates, phase_randomized_surrogate, topological_te_estimate,
};
use omegaflow_measure::eeglab::{
    channel_series, labels_from_channels_tsv, open_set, open_set_bin, open_set_mat, resolve_channel,
};

const DIM: usize = 3;
const DEFAULT_SURROGATES: usize = 200;
const DEFAULT_PERCENTILE: f64 = 95.0;
const SEED: u64 = 0x9E37_79B9_7F4A_7C15;
const MIN_N: usize = 32;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn usage() {
    eprintln!(
        "hyperscanning_group_te — the family-wise TE screen over a hyperscanning cohort:\n\
         per task, one transfer-entropy family is tested (every triad x every ordered pair).\n\
         The one estimator is the Takens estimate (dim 3; per series the mutual-information\n\
         lag tau replaces the lag grid) — the observed path and the surrogate path run the\n\
         same estimator function. The null is the empirical distribution of the family\n\
         maximum over phase-randomized surrogates (the max-statistic carries the whole\n\
         family, FWER = 1 - percentile):\n\
         \x20 hyperscanning_group_te --manifest <file> [--channel <label>]\n\
         \x20     [--surrogates <n>] [--seed <n>] [--max-points <n>] [--percentile <p>]\n\
         \x20     [--null phase|coherent-phase]\n\
         \x20 --null phase (default) rotates each series alone; coherent-phase rotates every series\n\
         \x20 of a triad with one shared phase vector, preserving the linear cross-structure — the\n\
         \x20 pair null for transfer beyond the linear cross-correlation.\n\
         manifest lines: <task> <triad> <slot> <path> (blank and # lines skipped).\n\
         each path reads as a text .set, a MAT-v5 EEG struct, or an EEGB .bin; one series per\n\
         participant is taken (--channel, default Fz) — a named electrode, not the common average.\n\
         without --max-points each series is read in full (the physical truth); n and srate print.\n\
         an absent recording drops its triad from the family, never a fabricated 0 (0 honored)."
    );
}

fn parse_manifest(text: &str) -> Vec<(String, String, String, String)> {
    let mut out = Vec::new();
    for raw in text.lines() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut it = line.split_whitespace();
        let (Some(task), Some(triad), Some(slot), Some(path)) =
            (it.next(), it.next(), it.next(), it.next())
        else {
            continue;
        };
        out.push((
            task.to_string(),
            triad.to_string(),
            slot.to_string(),
            path.to_string(),
        ));
    }
    out
}

fn load_series(
    path: &str,
    channel: &str,
    max_points: Option<usize>,
) -> Option<(Vec<f32>, Option<f64>)> {
    let (mut set, samples) = open_set(path)
        .or_else(|| open_set_mat(path))
        .or_else(|| open_set_bin(path))?;
    if set.labels.is_empty() {
        if let Some(labels) = labels_from_channels_tsv(path) {
            set.labels = labels;
        }
    }
    let ch = resolve_channel(&set, channel)?;
    let mut series = channel_series(&samples, &set, ch)?;
    if let Some(cap) = max_points {
        if series.len() > cap {
            series.truncate(cap);
        }
    }
    if series.len() < MIN_N {
        return None;
    }
    Some((series, set.srate))
}

fn percentile(sorted: &[f64], p: f64) -> Option<f64> {
    if sorted.is_empty() {
        return None;
    }
    let rank = (p / 100.0) * (sorted.len() as f64 - 1.0);
    let lo = rank.floor() as usize;
    let hi = rank.ceil() as usize;
    if lo == hi {
        return Some(sorted[lo]);
    }
    let frac = rank - lo as f64;
    Some(sorted[lo] * (1.0 - frac) + sorted[hi] * frac)
}

fn family_max(series: &[Vec<f32>], dim: usize) -> Option<f64> {
    let mut best: Option<f64> = None;
    for i in 0..series.len() {
        for j in 0..series.len() {
            if i == j {
                continue;
            }
            if let Some(est) = topological_te_estimate(&series[j], &series[i], dim) {
                if best.map_or(true, |b| est.te > b) {
                    best = Some(est.te);
                }
            }
        }
    }
    best
}

struct Cell {
    triad: String,
    driver: String,
    target: String,
    tau_x: usize,
    tau_y: usize,
    te: f64,
}

fn observed_cells(triads: &[(String, Vec<(String, Vec<f32>)>)], dim: usize) -> Vec<Cell> {
    let mut out = Vec::new();
    for (triad, series) in triads {
        for i in 0..series.len() {
            for j in 0..series.len() {
                if i == j {
                    continue;
                }
                if let Some(est) = topological_te_estimate(&series[j].1, &series[i].1, dim) {
                    out.push(Cell {
                        triad: triad.clone(),
                        driver: series[i].0.clone(),
                        target: series[j].0.clone(),
                        tau_x: est.tau_x,
                        tau_y: est.tau_y,
                        te: est.te,
                    });
                }
            }
        }
    }
    out
}

fn surrogate_family_maxima(
    triads: &[(String, Vec<(String, Vec<f32>)>)],
    dim: usize,
    n_surr: usize,
    seed: u64,
    coherent: bool,
) -> Vec<f64> {
    let mut out = Vec::with_capacity(n_surr);
    for s in 0..n_surr {
        let mut family: Option<f64> = None;
        for (t, (_, series)) in triads.iter().enumerate() {
            let mut randomized: Vec<Vec<f32>> = Vec::with_capacity(series.len());
            if coherent {
                let mut rng = seed
                    ^ (s as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15)
                    ^ ((t as u64) << 20)
                    ^ 0xA5A5_5A5A;
                let refs: Vec<&[f32]> = series.iter().map(|(_, v)| v.as_slice()).collect();
                randomized = coherent_phase_surrogates(&refs, &mut rng);
            } else {
                for (k, (_, v)) in series.iter().enumerate() {
                    let mut rng = seed
                        ^ (s as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15)
                        ^ ((t as u64) << 20)
                        ^ ((k as u64) << 8)
                        ^ 0xA5A5_5A5A;
                    randomized.push(phase_randomized_surrogate(v, &mut rng));
                }
            }
            if let Some(m) = family_max(&randomized, dim) {
                if family.map_or(true, |f| m > f) {
                    family = Some(m);
                }
            }
        }
        if let Some(f) = family {
            out.push(f);
        }
    }
    out.sort_by(f64::total_cmp);
    out
}

fn screen_carries_a_measurement(triads_per_task: &[usize]) -> bool {
    triads_per_task.iter().any(|&n| n >= 1)
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.iter().any(|a| a == "--help" || a == "-h") {
        usage();
        return;
    }
    let Some(manifest_path) = arg_value(&args, "--manifest") else {
        eprintln!("hyperscanning_group_te: --manifest <file> required");
        exit(2);
    };
    let channel = match arg_value(&args, "--channel") {
        Some(c) => c,
        None => "Fz".to_string(),
    };
    let n_surr: usize = arg_value(&args, "--surrogates")
        .and_then(|v| v.parse().ok())
        .unwrap_or(DEFAULT_SURROGATES);
    let seed: u64 = arg_value(&args, "--seed")
        .and_then(|v| v.parse().ok())
        .unwrap_or(SEED);
    let max_points: Option<usize> = arg_value(&args, "--max-points").and_then(|v| v.parse().ok());
    let pct: f64 = arg_value(&args, "--percentile")
        .and_then(|v| v.parse().ok())
        .unwrap_or(DEFAULT_PERCENTILE);
    let coherent = match arg_value(&args, "--null").as_deref() {
        Some("coherent-phase") => true,
        Some("phase") | None => false,
        Some(other) => {
            eprintln!(
                "hyperscanning_group_te: --null {other} is not a null model (phase|coherent-phase)"
            );
            exit(2);
        }
    };

    let text = match std::fs::read_to_string(&manifest_path) {
        Ok(t) => t,
        Err(_) => {
            eprintln!("hyperscanning_group_te: the manifest at {manifest_path} is not readable");
            exit(2);
        }
    };
    let entries = parse_manifest(&text);
    if entries.is_empty() {
        eprintln!("hyperscanning_group_te: the manifest carries no entry");
        exit(2);
    }

    let mut tasks: Vec<String> = entries.iter().map(|e| e.0.clone()).collect();
    tasks.sort();
    tasks.dedup();
    let mut triads_per_task: Vec<usize> = Vec::new();

    println!(
        "hyperscanning group TE screen | channel [{channel}] | dim {DIM} | surrogates {n_surr} | percentile {pct} | null {}",
        if coherent { "coherent-phase" } else { "phase" }
    );

    for task in &tasks {
        let mut triads: Vec<(String, Vec<(String, Vec<f32>)>)> = Vec::new();
        let mut triad_ids: Vec<String> = entries
            .iter()
            .filter(|e| &e.0 == task)
            .map(|e| e.1.clone())
            .collect();
        triad_ids.sort();
        triad_ids.dedup();
        for triad in &triad_ids {
            let mut members: Vec<(String, Vec<f32>)> = Vec::new();
            for entry in entries.iter().filter(|e| &e.0 == task && &e.1 == triad) {
                match load_series(&entry.3, &channel, max_points) {
                    Some((series, srate)) => {
                        println!(
                            "  [{task}/{triad}/{slot}] n = {} srate = {}",
                            series.len(),
                            match srate {
                                Some(s) => format!("{s}"),
                                None => "absent".to_string(),
                            },
                            slot = entry.2
                        );
                        members.push((entry.2.clone(), series));
                    }
                    None => println!(
                        "  [{task}/{triad}/{slot}] absent — no readable [{channel}] series (0 honored)",
                        slot = entry.2
                    ),
                }
            }
            if members.len() >= 2 {
                triads.push((triad.clone(), members));
            }
        }

        triads_per_task.push(triads.len());
        if triads.is_empty() {
            println!("=== {task}: no complete triad carries a series — pending (0 honored)");
            continue;
        }

        let cells = observed_cells(&triads, DIM);
        let maxima = surrogate_family_maxima(&triads, DIM, n_surr, seed, coherent);
        let Some(threshold) = percentile(&maxima, pct) else {
            println!("=== {task}: the surrogate family carries no maximum — pending (0 honored)");
            continue;
        };
        let observed_max = cells.iter().map(|c| c.te).fold(f64::NEG_INFINITY, f64::max);
        let survivors: Vec<&Cell> = cells.iter().filter(|c| c.te > threshold).collect();

        println!(
            "=== {task}: {} triad(s) | {} cell(s) | fam-max p{pct} = {threshold:.4e} | observed max = {observed_max:.4e} | survivors = {}",
            triads.len(),
            cells.len(),
            survivors.len()
        );
        for c in &survivors {
            println!(
                "    SURVIVOR {} {}→{} tau {}/{} TE {:.4e}",
                c.triad, c.driver, c.target, c.tau_x, c.tau_y, c.te
            );
        }
        if survivors.is_empty() {
            println!("    no cell breaks the family maximum — the silence is the finding");
        }
    }

    if !screen_carries_a_measurement(&triads_per_task) {
        eprintln!(
            "hyperscanning_group_te: no task carried a complete triad — the screen ran on no readable series; the run carries no measurement"
        );
        exit(2);
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

    #[test]
    fn manifest_reads_four_columns_and_skips_noise() {
        let text = "# comment\n\npddecision G01 S01 a.set\npddecision G01 S02 b.set\n";
        let e = parse_manifest(text);
        assert_eq!(e.len(), 2);
        assert_eq!(
            e[0],
            (
                "pddecision".into(),
                "G01".into(),
                "S01".into(),
                "a.set".into()
            )
        );
    }

    #[test]
    fn a_screen_on_no_readable_series_carries_no_measurement() {
        assert!(!screen_carries_a_measurement(&[]));
        assert!(!screen_carries_a_measurement(&[0, 0, 0]));
        assert!(screen_carries_a_measurement(&[0, 1, 0]));
    }

    #[test]
    fn percentile_interpolates() {
        let v = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(percentile(&v, 0.0), Some(1.0));
        assert_eq!(percentile(&v, 100.0), Some(5.0));
        assert_eq!(percentile(&v, 50.0), Some(3.0));
        assert!(percentile(&[], 95.0).is_none());
    }

    fn white(n: usize, rng: &mut u64) -> Vec<f32> {
        (0..n).map(|_| (next_rng(rng) * 2.0 - 1.0) as f32).collect()
    }

    fn ar1_sine(n: usize, phi: f64, period: f64, phase: f64, noise: f64, rng: &mut u64) -> Vec<f32> {
        let mut v = Vec::with_capacity(n);
        let mut x = 0.0f64;
        for t in 0..n {
            x = phi * x + (2.0 * std::f64::consts::PI * t as f64 / period + phase).sin()
                + noise * (next_rng(rng) * 2.0 - 1.0);
            v.push(x as f32);
        }
        v
    }

    #[test]
    fn a_structured_driver_breaks_the_family_maximum() {
        let mut rng = SEED ^ 0xDEAD_BEEF;
        let n = 400usize;
        let delay = 8usize;
        let a = ar1_sine(n, 0.6, 36.0, 0.0, 0.0, &mut rng);
        let mut b = vec![0.0f32; n];
        let mut x = 0.0f64;
        for t in 0..n {
            x = 0.5 * x
                + if t >= delay { 0.8 * a[t - delay] as f64 } else { 0.0 }
                + (next_rng(&mut rng) * 0.04 - 0.02);
            b[t] = x as f32;
        }
        let triads = vec![(
            "G01".to_string(),
            vec![("S01".to_string(), a), ("S02".to_string(), b)],
        )];
        let cells = observed_cells(&triads, DIM);
        assert!(!cells.is_empty(), "the structured pair carries no cell");
        let maxima = surrogate_family_maxima(&triads, DIM, 50, SEED, false);
        let threshold = percentile(&maxima, 95.0).expect("the family maximum is measurable");
        let observed = cells.iter().map(|c| c.te).fold(f64::NEG_INFINITY, f64::max);
        assert!(
            observed > threshold,
            "the driven direction breaks the family maximum: {observed} vs {threshold}"
        );
        let top = cells.iter().max_by(|x, y| x.te.total_cmp(&y.te)).unwrap();
        assert_eq!(top.driver, "S01");
        assert_eq!(top.target, "S02");
    }

    #[test]
    fn independent_structured_series_stay_under_the_family_maximum() {
        let mut rng = SEED ^ 0x1234_5678;
        let trials = 20usize;
        let mut fp = 0usize;
        let mut meas = 0usize;
        for t in 0..trials {
            let phase_a = next_rng(&mut rng) * std::f64::consts::TAU;
            let phase_b = next_rng(&mut rng) * std::f64::consts::TAU;
            let a = ar1_sine(400, 0.6, 37.0, phase_a, 0.05, &mut rng);
            let b = ar1_sine(400, 0.6, 43.0, phase_b, 0.05, &mut rng);
            let members = vec![("S01".to_string(), a), ("S02".to_string(), b)];
            let triads = vec![("G01".to_string(), members)];
            let cells = observed_cells(&triads, DIM);
            if cells.is_empty() {
                continue;
            }
            let maxima = surrogate_family_maxima(&triads, DIM, 50, SEED ^ (t as u64), false);
            let Some(threshold) = percentile(&maxima, 95.0) else {
                continue;
            };
            meas += 1;
            if cells.iter().any(|c| c.te > threshold) {
                fp += 1;
            }
        }
        println!(
            "structured-FP gate: {fp} of {meas} measurable trials carried a survivor"
        );
        assert!(
            meas >= 16,
            "structured-FP gate: {} of {trials} measurable — the machine stays silent too often",
            meas
        );
        assert!(
            fp <= 4,
            "structured-FP gate: {fp} of {meas} above the family maximum — the null does not hold"
        );
    }

    #[test]
    fn white_noise_pair_carries_no_cell() {
        let mut rng = SEED ^ 0x0F0F_0F0F;
        let a = white(13, &mut rng);
        let b = white(13, &mut rng);
        let triads = vec![(
            "G01".to_string(),
            vec![("S01".to_string(), a.clone()), ("S02".to_string(), b.clone())],
        )];
        let cells = observed_cells(&triads, DIM);
        assert!(cells.is_empty(), "a white pair carries no cell");
        assert!(
            family_max(&[a, b], DIM).is_none(),
            "a white pair carries no family maximum — the screen reports pending, never a green idle"
        );
    }

    #[test]
    fn family_max_equals_observed_max() {
        let mut rng = SEED ^ 0x5A5A_5A5A;
        let s1 = ar1_sine(400, 0.6, 37.0, 0.1, 0.05, &mut rng);
        let s2 = ar1_sine(400, 0.6, 43.0, 1.2, 0.05, &mut rng);
        let s3 = ar1_sine(400, 0.6, 51.0, 2.3, 0.05, &mut rng);
        let series = vec![s1.clone(), s2.clone(), s3.clone()];
        let members = vec![
            ("S01".to_string(), s1),
            ("S02".to_string(), s2),
            ("S03".to_string(), s3),
        ];
        let triads = vec![("G01".to_string(), members)];
        let fm = family_max(&series, DIM).expect("the family carries a maximum");
        let cells = observed_cells(&triads, DIM);
        let om = cells.iter().map(|c| c.te).fold(f64::NEG_INFINITY, f64::max);
        assert_eq!(
            fm, om,
            "family_max and the observed cell maximum run the one estimator on the same pairs"
        );
    }
}
