use std::env;
use std::process::exit;

use omegaflow::te::{
    coherent_phase_surrogates, find_mi_lag, phase_randomized_surrogate, topological_te_estimate,
    topological_te_estimate_frozen,
};
use omegaflow_measure::eeglab::{
    channel_series, labels_from_channels_tsv, open_set, open_set_bin, open_set_mat, resolve_channel,
};

const DIM: usize = 3;
const DEFAULT_SURROGATES: usize = 200;
const DEFAULT_PERCENTILE: f64 = 95.0;
const CONFIRM_SURROGATES: usize = 1000;
const CONFIRM_PERCENTILE: f64 = 99.0;
const SEED: u64 = 0x9E37_79B9_7F4A_7C15;
const CONFIRM_SEED: u64 = 0x2545_F491_4F6C_DD1D;
const MIN_N: usize = 32;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn usage() {
    eprintln!(
        "hyperscanning_group_te — the two-level TE screen over a hyperscanning cohort:\n\
         per task, one transfer-entropy family is tested (every triad x every ordered pair).\n\
         The one estimator is the Takens estimate (dim 3; per series the mutual-information\n\
         lag tau replaces the lag grid) — the observed path and the surrogate path run the\n\
         same estimator function. The null is the empirical distribution over phase-randomized\n\
         surrogates, read at two levels: the family maximum (the max-statistic carries the whole\n\
         family, FWER = 1 - percentile, the stricter line) and the per-cell distribution (each\n\
         ordered pair against its own surrogate series, naming the weaker transfer the family\n\
         maximum masks):\n\
         \x20 hyperscanning_group_te --manifest <file> [--channel <label>]\n\
         \x20     [--surrogates <n>] [--seed <n>] [--max-points <n>] [--percentile <p>]\n\
         \x20     [--null phase|coherent-phase] [--nominees-out <file>] [--nominees <file>]\n\
         \x20 --nominees-out <file> writes the per-cell survivors of the screen (the nominees:\n\
         \x20 task, triad, driver, target, slot, TE, own per-cell threshold) as a TSV artifact.\n\
         \x20 --nominees <file> is the confirmation run: it reads that artifact and tests each\n\
         \x20 nominated cell against its own per-cell null with a fresh seed — p99 and 1000\n\
         \x20 surrogates by default — printing the confirmed nomination list. The family-maximum\n\
         \x20 path stays the FWER carrier; the confirmation replaces only the family re-test.\n\
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

struct Nominee {
    task: String,
    triad: String,
    driver: String,
    target: String,
    slot: usize,
    te: f64,
    threshold: f64,
}

fn parse_nominees(text: &str) -> Vec<Nominee> {
    let mut out = Vec::new();
    for raw in text.lines() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut it = line.split('\t');
        let (
            Some(task),
            Some(triad),
            Some(driver),
            Some(target),
            Some(slot),
            Some(te),
            Some(threshold),
        ) = (
            it.next(),
            it.next(),
            it.next(),
            it.next(),
            it.next(),
            it.next(),
            it.next(),
        )
        else {
            continue;
        };
        let (Ok(slot), Ok(te), Ok(threshold)) = (
            slot.parse::<usize>(),
            te.parse::<f64>(),
            threshold.parse::<f64>(),
        ) else {
            continue;
        };
        out.push(Nominee {
            task: task.to_string(),
            triad: triad.to_string(),
            driver: driver.to_string(),
            target: target.to_string(),
            slot,
            te,
            threshold,
        });
    }
    out
}

fn write_nominees(path: &str, nominees: &[Nominee]) -> bool {
    let mut text = String::from("# task\ttriad\tdriver\ttarget\tslot\tte\tthreshold\n");
    for n in nominees {
        text.push_str(&format!(
            "{}\t{}\t{}\t{}\t{}\t{:.17e}\t{:.17e}\n",
            n.task, n.triad, n.driver, n.target, n.slot, n.te, n.threshold
        ));
    }
    match std::fs::write(path, text) {
        Ok(()) => true,
        Err(_) => {
            eprintln!("hyperscanning_group_te: the nominee list at {path} is not writable");
            false
        }
    }
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

#[cfg(test)]
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
    slot: usize,
}

fn cell_slot(offset: usize, members: usize, i: usize, j: usize) -> usize {
    offset + i * (members - 1) + if j < i { j } else { j - 1 }
}

fn observed_cells(triads: &[(String, Vec<(String, Vec<f32>)>)], dim: usize) -> Vec<Cell> {
    let mut out = Vec::new();
    let mut offset = 0usize;
    for (triad, series) in triads {
        let members = series.len();
        for i in 0..members {
            for j in 0..members {
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
                        slot: cell_slot(offset, members, i, j),
                    });
                }
            }
        }
        offset += members * members.saturating_sub(1);
    }
    out
}

struct SurrogateFamily {
    maxima: Vec<f64>,
    cell_distributions: Vec<Vec<f64>>,
}

fn randomized_triad(
    series: &[(String, Vec<f32>)],
    seed: u64,
    s: usize,
    t: usize,
    coherent: bool,
) -> Vec<Vec<f32>> {
    if coherent {
        let mut rng = seed
            ^ (s as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15)
            ^ ((t as u64) << 20)
            ^ 0xA5A5_5A5A;
        let refs: Vec<&[f32]> = series.iter().map(|(_, v)| v.as_slice()).collect();
        coherent_phase_surrogates(&refs, &mut rng)
    } else {
        let mut randomized = Vec::with_capacity(series.len());
        for (k, (_, v)) in series.iter().enumerate() {
            let mut rng = seed
                ^ (s as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15)
                ^ ((t as u64) << 20)
                ^ ((k as u64) << 8)
                ^ 0xA5A5_5A5A;
            randomized.push(phase_randomized_surrogate(v, &mut rng));
        }
        randomized
    }
}

fn member_taus(series: &[(String, Vec<f32>)]) -> Vec<Option<usize>> {
    series
        .iter()
        .map(|(_, v)| {
            let vf: Vec<f64> = v.iter().map(|&x| x as f64).collect();
            find_mi_lag(&vf)
        })
        .collect()
}

fn frozen_estimate(
    x: &[f32],
    y: &[f32],
    dim: usize,
    tau_x: Option<usize>,
    tau_y: Option<usize>,
) -> Option<f64> {
    match (tau_x, tau_y) {
        (Some(tx), Some(ty)) => topological_te_estimate_frozen(x, y, dim, tx, ty).map(|e| e.te),
        _ => None,
    }
}

fn surrogate_family_maxima(
    triads: &[(String, Vec<(String, Vec<f32>)>)],
    dim: usize,
    n_surr: usize,
    seed: u64,
    coherent: bool,
) -> SurrogateFamily {
    let mut offsets = Vec::with_capacity(triads.len());
    let mut total_cells = 0usize;
    for (_, series) in triads {
        offsets.push(total_cells);
        total_cells += series.len() * series.len().saturating_sub(1);
    }
    let frozen: Vec<Vec<Option<usize>>> = triads.iter().map(|(_, s)| member_taus(s)).collect();
    let mut cell_distributions: Vec<Vec<f64>> = vec![Vec::new(); total_cells];
    let mut out = Vec::with_capacity(n_surr);
    for s in 0..n_surr {
        let mut family: Option<f64> = None;
        for (t, (_, series)) in triads.iter().enumerate() {
            let members = series.len();
            let randomized = randomized_triad(series, seed, s, t, coherent);
            for i in 0..members {
                for j in 0..members {
                    if i == j {
                        continue;
                    }
                    if let Some(te) = frozen_estimate(
                        &randomized[j],
                        &randomized[i],
                        dim,
                        frozen[t][j],
                        frozen[t][i],
                    ) {
                        if family.map_or(true, |f| te > f) {
                            family = Some(te);
                        }
                        let slot = cell_slot(offsets[t], members, i, j);
                        cell_distributions[slot].push(te);
                    }
                }
            }
        }
        if let Some(f) = family {
            out.push(f);
        }
    }
    out.sort_by(f64::total_cmp);
    for d in cell_distributions.iter_mut() {
        d.sort_by(f64::total_cmp);
    }
    SurrogateFamily {
        maxima: out,
        cell_distributions,
    }
}

fn per_cell_survivors<'a>(cells: &'a [Cell], family: &SurrogateFamily, pct: f64) -> Vec<&'a Cell> {
    cells
        .iter()
        .filter(|c| percentile(&family.cell_distributions[c.slot], pct).map_or(false, |t| c.te > t))
        .collect()
}

fn confirmation_cell_nulls(
    triads: &[(String, Vec<(String, Vec<f32>)>)],
    plan: &[(usize, usize, usize, &Nominee)],
    dim: usize,
    n_surr: usize,
    seed: u64,
    coherent: bool,
) -> Vec<Vec<f64>> {
    let mut dists: Vec<Vec<f64>> = vec![Vec::new(); plan.len()];
    let mut by_triad: Vec<Vec<usize>> = vec![Vec::new(); triads.len()];
    for (idx, p) in plan.iter().enumerate() {
        by_triad[p.0].push(idx);
    }
    let frozen: Vec<Vec<Option<usize>>> = triads.iter().map(|(_, s)| member_taus(s)).collect();
    for s in 0..n_surr {
        for (ti, members) in triads.iter().enumerate() {
            if by_triad[ti].is_empty() {
                continue;
            }
            let randomized = randomized_triad(&members.1, seed, s, ti, coherent);
            for &idx in &by_triad[ti] {
                let (_, i, j, _) = plan[idx];
                if let Some(te) = frozen_estimate(
                    &randomized[j],
                    &randomized[i],
                    dim,
                    frozen[ti][j],
                    frozen[ti][i],
                ) {
                    dists[idx].push(te);
                }
            }
        }
    }
    for d in dists.iter_mut() {
        d.sort_by(f64::total_cmp);
    }
    dists
}

fn fmt_value(v: Option<f64>) -> String {
    match v {
        Some(x) => format!("{x:.6e}"),
        None => "absent".to_string(),
    }
}

fn print_verdict(verdict: &str, n: &Nominee, te: Option<f64>, threshold: Option<f64>) {
    println!(
        "    {verdict}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{:.6e}\t{:.6e}",
        n.task,
        n.triad,
        n.driver,
        n.target,
        n.slot,
        fmt_value(te),
        fmt_value(threshold),
        n.te,
        n.threshold
    );
}

fn run_confirmation(
    entries: &[(String, String, String, String)],
    nominees: &[Nominee],
    channel: &str,
    max_points: Option<usize>,
    n_surr: usize,
    pct: f64,
    seed: u64,
    coherent: bool,
) {
    let mut tasks: Vec<String> = nominees.iter().map(|n| n.task.clone()).collect();
    tasks.sort();
    tasks.dedup();
    println!(
        "hyperscanning group TE confirmation | channel [{channel}] | dim {DIM} | surrogates {n_surr} | percentile {pct} | null {} | fresh seed",
        if coherent { "coherent-phase" } else { "phase" }
    );
    let mut confirmed_total = 0usize;
    let mut pending_total = 0usize;
    for task in &tasks {
        let mut triad_ids: Vec<String> = entries
            .iter()
            .filter(|e| &e.0 == task)
            .map(|e| e.1.clone())
            .collect();
        triad_ids.sort();
        triad_ids.dedup();
        let mut triads: Vec<(String, Vec<(String, Vec<f32>)>)> = Vec::new();
        for triad in &triad_ids {
            let mut members: Vec<(String, Vec<f32>)> = Vec::new();
            for entry in entries.iter().filter(|e| &e.0 == task && &e.1 == triad) {
                if let Some((series, _srate)) = load_series(&entry.3, channel, max_points) {
                    members.push((entry.2.clone(), series));
                }
            }
            if members.len() >= 2 {
                triads.push((triad.clone(), members));
            }
        }
        let task_nominees: Vec<&Nominee> = nominees.iter().filter(|n| &n.task == task).collect();
        let mut plan: Vec<(usize, usize, usize, &Nominee)> = Vec::new();
        for n in &task_nominees {
            let Some(ti) = triads.iter().position(|(id, _)| id == &n.triad) else {
                print_verdict("PENDING", n, None, None);
                pending_total += 1;
                continue;
            };
            let members = &triads[ti].1;
            let Some(i) = members.iter().position(|(s, _)| s == &n.driver) else {
                print_verdict("PENDING", n, None, None);
                pending_total += 1;
                continue;
            };
            let Some(j) = members.iter().position(|(s, _)| s == &n.target) else {
                print_verdict("PENDING", n, None, None);
                pending_total += 1;
                continue;
            };
            plan.push((ti, i, j, n));
        }
        let dists = confirmation_cell_nulls(&triads, &plan, DIM, n_surr, seed, coherent);
        let mut confirmed_here = 0usize;
        for (idx, (ti, i, j, n)) in plan.iter().enumerate() {
            let members = &triads[*ti].1;
            let observed = topological_te_estimate(&members[*j].1, &members[*i].1, DIM);
            let threshold = percentile(&dists[idx], pct);
            match (observed, threshold) {
                (Some(obs), Some(thr)) => {
                    if obs.te > thr {
                        confirmed_here += 1;
                        confirmed_total += 1;
                        print_verdict("CONFIRMED", n, Some(obs.te), Some(thr));
                    } else {
                        print_verdict("NOT-CONFIRMED", n, Some(obs.te), Some(thr));
                    }
                }
                (obs, thr) => {
                    pending_total += 1;
                    print_verdict("PENDING", n, obs.map(|o| o.te), thr);
                }
            }
        }
        println!(
            "=== confirmation {task}: {} nominee(s) | {confirmed_here} confirmed",
            task_nominees.len()
        );
    }
    println!(
        "=== confirmation total: {} nominee(s) | {confirmed_total} confirmed | {pending_total} pending",
        nominees.len()
    );
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
    let confirm_path = arg_value(&args, "--nominees");
    let nominees_out = arg_value(&args, "--nominees-out");
    let confirming = confirm_path.is_some();
    let n_surr: usize = match arg_value(&args, "--surrogates").and_then(|v| v.parse().ok()) {
        Some(v) => v,
        None => {
            if confirming {
                CONFIRM_SURROGATES
            } else {
                DEFAULT_SURROGATES
            }
        }
    };
    let seed: u64 = match arg_value(&args, "--seed").and_then(|v| v.parse().ok()) {
        Some(v) => v,
        None => {
            if confirming {
                CONFIRM_SEED
            } else {
                SEED
            }
        }
    };
    let max_points: Option<usize> = arg_value(&args, "--max-points").and_then(|v| v.parse().ok());
    let pct: f64 = match arg_value(&args, "--percentile").and_then(|v| v.parse().ok()) {
        Some(v) => v,
        None => {
            if confirming {
                CONFIRM_PERCENTILE
            } else {
                DEFAULT_PERCENTILE
            }
        }
    };
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

    if let Some(path) = &confirm_path {
        let nominee_text = match std::fs::read_to_string(path) {
            Ok(t) => t,
            Err(_) => {
                eprintln!("hyperscanning_group_te: the nominee list at {path} is not readable");
                exit(2);
            }
        };
        let nominees = parse_nominees(&nominee_text);
        if nominees.is_empty() {
            println!(
                "hyperscanning group TE confirmation: the nominee list at {path} carries no nomination — the per-cell silence is the finding (0 honored)"
            );
            return;
        }
        run_confirmation(
            &entries, &nominees, &channel, max_points, n_surr, pct, seed, coherent,
        );
        return;
    }

    let mut tasks: Vec<String> = entries.iter().map(|e| e.0.clone()).collect();
    tasks.sort();
    tasks.dedup();
    let mut triads_per_task: Vec<usize> = Vec::new();
    let mut nominees: Vec<Nominee> = Vec::new();

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
        let family = surrogate_family_maxima(&triads, DIM, n_surr, seed, coherent);
        let Some(threshold) = percentile(&family.maxima, pct) else {
            println!("=== {task}: the surrogate family carries no maximum — pending (0 honored)");
            continue;
        };
        let observed_max = cells.iter().map(|c| c.te).fold(f64::NEG_INFINITY, f64::max);
        let family_survivors: Vec<&Cell> = cells.iter().filter(|c| c.te > threshold).collect();
        let cell_survivors = per_cell_survivors(&cells, &family, pct);
        for c in &cell_survivors {
            if let Some(t) = percentile(&family.cell_distributions[c.slot], pct) {
                nominees.push(Nominee {
                    task: task.clone(),
                    triad: c.triad.clone(),
                    driver: c.driver.clone(),
                    target: c.target.clone(),
                    slot: c.slot,
                    te: c.te,
                    threshold: t,
                });
            }
        }

        println!(
            "=== {task}: {} triad(s) | {} cell(s) | fam-max p{pct} = {threshold:.4e} | observed max = {observed_max:.4e} | family-max survivors = {} | per-cell survivors = {}",
            triads.len(),
            cells.len(),
            family_survivors.len(),
            cell_survivors.len()
        );
        for c in &family_survivors {
            println!(
                "    FAMILY-MAX SURVIVOR {} {}→{} tau {}/{} TE {:.4e}",
                c.triad, c.driver, c.target, c.tau_x, c.tau_y, c.te
            );
        }
        for c in &cell_survivors {
            if family_survivors.iter().any(|f| f.slot == c.slot) {
                println!(
                    "    PER-CELL SURVIVOR {} {}→{} tau {}/{} TE {:.4e}",
                    c.triad, c.driver, c.target, c.tau_x, c.tau_y, c.te
                );
            } else {
                println!(
                    "    PER-CELL SURVIVOR (masked by the family maximum) {} {}→{} tau {}/{} TE {:.4e}",
                    c.triad, c.driver, c.target, c.tau_x, c.tau_y, c.te
                );
            }
        }
        if family_survivors.is_empty() {
            println!(
                "    no cell breaks the family maximum — the family-max silence is the finding"
            );
        }
        if cell_survivors.is_empty() {
            println!(
                "    no cell breaks its own surrogate distribution — the cell-level silence is the finding"
            );
        }
    }

    if !screen_carries_a_measurement(&triads_per_task) {
        eprintln!(
            "hyperscanning_group_te: no task carried a complete triad — the screen ran on no readable series; the run carries no measurement"
        );
        exit(2);
    }
    if let Some(path) = &nominees_out {
        if !write_nominees(path, &nominees) {
            exit(2);
        }
        println!(
            "hyperscanning group TE screen: {} nominee(s) written to {path}",
            nominees.len()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use omegaflow::te::{embed_series, transfer_entropy_embedded_kde};

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

    fn ar1_sine(
        n: usize,
        phi: f64,
        period: f64,
        phase: f64,
        noise: f64,
        rng: &mut u64,
    ) -> Vec<f32> {
        let mut v = Vec::with_capacity(n);
        let mut x = 0.0f64;
        for t in 0..n {
            x = phi * x
                + (2.0 * std::f64::consts::PI * t as f64 / period + phase).sin()
                + noise * (next_rng(rng) * 2.0 - 1.0);
            v.push(x as f32);
        }
        v
    }

    fn rich_series(n: usize, rng: &mut u64) -> Vec<f32> {
        let periods = [7.3f64, 13.1, 23.7, 41.9, 67.3];
        let mut v = Vec::with_capacity(n);
        let mut x = 0.0f64;
        for t in 0..n {
            let s: f64 = periods
                .iter()
                .enumerate()
                .map(|(k, &p)| {
                    let amp = k as f64 + 1.0;
                    let angle = 2.0 * std::f64::consts::PI * t as f64 / p + k as f64 * 0.9;
                    amp * angle.sin()
                })
                .sum();
            x = 0.6 * x + s + 0.3 * (next_rng(rng) * 2.0 - 1.0);
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
                + if t >= delay {
                    0.8 * a[t - delay] as f64
                } else {
                    0.0
                }
                + (next_rng(&mut rng) * 0.04 - 0.02);
            b[t] = x as f32;
        }
        let triads = vec![(
            "G01".to_string(),
            vec![("S01".to_string(), a), ("S02".to_string(), b)],
        )];
        let cells = observed_cells(&triads, DIM);
        assert!(!cells.is_empty(), "the structured pair carries no cell");
        let maxima = surrogate_family_maxima(&triads, DIM, 50, SEED, false).maxima;
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
            let maxima = surrogate_family_maxima(&triads, DIM, 50, SEED ^ (t as u64), false).maxima;
            let Some(threshold) = percentile(&maxima, 95.0) else {
                continue;
            };
            meas += 1;
            if cells.iter().any(|c| c.te > threshold) {
                fp += 1;
            }
        }
        println!("structured-FP gate: {fp} of {meas} measurable trials carried a survivor");
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
            vec![
                ("S01".to_string(), a.clone()),
                ("S02".to_string(), b.clone()),
            ],
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

    fn fn_gate_fixture(rng: &mut u64) -> (Vec<f32>, Vec<f32>, Vec<f32>, Vec<f32>) {
        let n = 600usize;
        let delay = 8usize;
        let a = ar1_sine(n, 0.6, 36.0, 0.0, 0.0, rng);
        let mut b = vec![0.0f32; n];
        let mut x = 0.0f64;
        for t in 0..n {
            x = 0.5 * x
                + if t >= delay {
                    0.8 * a[t - delay] as f64
                } else {
                    0.0
                }
                + (next_rng(rng) * 0.04 - 0.02);
            b[t] = x as f32;
        }
        let c = ar1_sine(n, 0.6, 29.0, 1.1, 0.02, rng);
        let mut d = vec![0.0f32; n];
        let mut y = 0.0f64;
        for t in 0..n {
            y = 0.5 * y
                + if t >= 2 {
                    0.35 * (c[t - 2] as f64) * (c[t - 2] as f64)
                } else {
                    0.0
                }
                + (next_rng(rng) * 0.04 - 0.02);
            d[t] = y as f32;
        }
        (a, b, c, d)
    }

    #[test]
    fn family_fn_gate() {
        let mut rng = SEED ^ 0xFACE_FEED;
        let (a, b, c, d) = fn_gate_fixture(&mut rng);
        let triads = vec![(
            "G01".to_string(),
            vec![
                ("A".to_string(), a),
                ("B".to_string(), b),
                ("C".to_string(), c),
                ("D".to_string(), d),
            ],
        )];
        let cells = observed_cells(&triads, DIM);
        let family = surrogate_family_maxima(&triads, DIM, 100, SEED, true);
        let family_threshold =
            percentile(&family.maxima, 95.0).expect("the family maximum is measurable");
        let family_survivors: Vec<&Cell> =
            cells.iter().filter(|c| c.te > family_threshold).collect();
        let cell_survivors = per_cell_survivors(&cells, &family, 95.0);

        let linear = cells
            .iter()
            .find(|c| c.driver == "A" && c.target == "B")
            .expect("the linear cell is estimated");
        let nonlinear = cells
            .iter()
            .find(|c| c.driver == "C" && c.target == "D")
            .expect("the nonlinear cell is estimated");

        let null_floor = &family.cell_distributions[nonlinear.slot];
        let null_mean = null_floor.iter().sum::<f64>() / null_floor.len() as f64;
        let null_sd = (null_floor
            .iter()
            .map(|v| (v - null_mean) * (v - null_mean))
            .sum::<f64>()
            / null_floor.len() as f64)
            .sqrt();
        let null_p95 = percentile(null_floor, 95.0).expect("the per-cell null is measurable");
        let floor_excess = nonlinear.te - null_mean;

        assert!(
            cell_survivors.iter().any(|c| c.slot == nonlinear.slot),
            "the per-cell rule finds the weaker nonlinear transfer: TE {:.4e} | null mean {:.4e} sd {:.4e} p95 {:.4e} | excess {:.4e} ({:.2} sd) | fam-max {:.4e}",
            nonlinear.te,
            null_mean,
            null_sd,
            null_p95,
            floor_excess,
            floor_excess / null_sd,
            family_threshold
        );
        assert!(
            !family_survivors.iter().any(|c| c.slot == nonlinear.slot),
            "the family maximum does not carry the weak nonlinear transfer (the mask the per-cell stage exists for): TE {:.4e} vs fam-max {:.4e}",
            nonlinear.te,
            family_threshold
        );
        assert!(
            !cell_survivors.iter().any(|c| c.slot == linear.slot),
            "the strong linear pair is not reported as transfer beyond its coherent null: TE {:.4e}",
            linear.te
        );
    }

    const SELF_NULL_SEEDS: usize = 100;

    fn white_like(v: &[f32], rng: &mut u64) -> Vec<f32> {
        let n = v.len() as f64;
        let mean = v.iter().map(|&x| x as f64).sum::<f64>() / n;
        let sd = (v
            .iter()
            .map(|&x| {
                let e = x as f64 - mean;
                e * e
            })
            .sum::<f64>()
            / n)
            .sqrt();
        (0..v.len())
            .map(|_| (mean + (next_rng(rng) - 0.5) * 12.0f64.sqrt() * sd) as f32)
            .collect()
    }

    fn ksg_te_frozen(target: &[f32], driver: &[f32], tau_t: usize, tau_d: usize) -> Option<f64> {
        topological_te_estimate_frozen(target, driver, DIM, tau_t, tau_d).map(|e| e.te)
    }

    fn kde_te_frozen(target: &[f32], driver: &[f32], tau_t: usize, tau_d: usize) -> Option<f64> {
        let xf: Vec<f64> = target.iter().map(|&v| v as f64).collect();
        let df: Vec<f64> = driver.iter().map(|&v| v as f64).collect();
        let emb_x = embed_series(&xf, tau_t, DIM);
        let emb_d = embed_series(&df, tau_d, DIM);
        if emb_x.is_empty() || emb_d.is_empty() {
            return None;
        }
        transfer_entropy_embedded_kde(&xf, &emb_x, &emb_d, tau_t, tau_d)
    }

    fn white_arm(
        target: &[f32],
        driver: &[f32],
        white_target: bool,
        white_driver: bool,
        tau_t: usize,
        tau_d: usize,
        seed_base: u64,
        est: fn(&[f32], &[f32], usize, usize) -> Option<f64>,
    ) -> Vec<f64> {
        let mut out = Vec::with_capacity(SELF_NULL_SEEDS);
        for s in 0..SELF_NULL_SEEDS {
            let mut rng = seed_base ^ (s as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15);
            let t: Vec<f32> = if white_target {
                white_like(target, &mut rng)
            } else {
                target.to_vec()
            };
            let d: Vec<f32> = if white_driver {
                white_like(driver, &mut rng)
            } else {
                driver.to_vec()
            };
            if let Some(te) = est(&t, &d, tau_t, tau_d) {
                out.push(te);
            }
        }
        out
    }

    struct ArmStats {
        mu: f64,
        sd: f64,
        p95: f64,
    }

    fn arm_stats(vals: &[f64]) -> Option<ArmStats> {
        if vals.len() < 2 {
            return None;
        }
        let n = vals.len() as f64;
        let mu = vals.iter().sum::<f64>() / n;
        let sd = (vals.iter().map(|v| (v - mu) * (v - mu)).sum::<f64>() / n).sqrt();
        let mut sorted = vals.to_vec();
        sorted.sort_by(f64::total_cmp);
        Some(ArmStats {
            mu,
            sd,
            p95: percentile(&sorted, 95.0)?,
        })
    }

    #[test]
    fn self_null_discriminator() {
        let mut rng = SEED ^ 0xFACE_FEED;
        let (_, _, c, d) = fn_gate_fixture(&mut rng);
        let members = vec![("C".to_string(), c.clone()), ("D".to_string(), d.clone())];
        let taus = member_taus(&members);
        let tau_c = taus[0].expect("self_null_discriminator: the driver c carries no MI lag");
        let tau_d = taus[1].expect("self_null_discriminator: the target d carries no MI lag");
        println!(
            "self_null_discriminator: tau_driver={tau_c} tau_target={tau_d} n=600 dim=3 seeds={SELF_NULL_SEEDS}"
        );

        let w1 = white_arm(&d, &c, false, true, tau_d, tau_c, SEED ^ 0x51A7_E11B, ksg_te_frozen);
        let w2 = white_arm(&d, &c, true, true, tau_d, tau_c, SEED ^ 0x51A7_E11C, ksg_te_frozen);
        let w3 = white_arm(&d, &c, true, false, tau_d, tau_c, SEED ^ 0x51A7_E11D, ksg_te_frozen);
        let k1 = white_arm(&d, &c, false, true, tau_d, tau_c, SEED ^ 0x51A7_E12B, kde_te_frozen);
        let k2 = white_arm(&d, &c, true, true, tau_d, tau_c, SEED ^ 0x51A7_E12C, kde_te_frozen);
        let k3 = white_arm(&d, &c, true, false, tau_d, tau_c, SEED ^ 0x51A7_E12D, kde_te_frozen);

        for (name, vals) in [
            ("KSG W1 driver-white", &w1),
            ("KSG W2 both-white", &w2),
            ("KSG W3 target-white", &w3),
            ("KDE K1 driver-white", &k1),
            ("KDE K2 both-white", &k2),
            ("KDE K3 target-white", &k3),
        ] {
            assert_eq!(
                vals.len(),
                SELF_NULL_SEEDS,
                "self_null_discriminator {name}: {} of {SELF_NULL_SEEDS} seeds measurable",
                vals.len()
            );
            let s = arm_stats(vals).expect("self_null_discriminator: the arm is degenerate");
            println!(
                "self_null_discriminator {name}: n={} mu={:.6e} sd={:.6e} p95={:.6e}",
                vals.len(),
                s.mu,
                s.sd,
                s.p95
            );
        }

        for &tau_driver in &[1usize, 2, 4, 8, tau_c] {
            let vals = white_arm(
                &d,
                &c,
                false,
                true,
                tau_d,
                tau_driver,
                SEED ^ 0x51A7_E13B ^ ((tau_driver as u64) << 16),
                ksg_te_frozen,
            );
            assert_eq!(
                vals.len(),
                SELF_NULL_SEEDS,
                "self_null_discriminator sweep tau_driver={tau_driver}: {} of {SELF_NULL_SEEDS} seeds measurable",
                vals.len()
            );
            let s = arm_stats(&vals).expect("self_null_discriminator: the sweep arm is degenerate");
            println!(
                "self_null_discriminator sweep W1 tau_driver={tau_driver}: n={} mu={:.6e} sd={:.6e} p95={:.6e}",
                vals.len(),
                s.mu,
                s.sd,
                s.p95
            );
        }

        let w1_again =
            white_arm(&d, &c, false, true, tau_d, tau_c, SEED ^ 0x51A7_E11B, ksg_te_frozen);
        let k1_again =
            white_arm(&d, &c, false, true, tau_d, tau_c, SEED ^ 0x51A7_E12B, kde_te_frozen);
        assert_eq!(
            w1, w1_again,
            "self_null_discriminator: the W1 arm is not seed-deterministic"
        );
        assert_eq!(
            k1, k1_again,
            "self_null_discriminator: the K1 arm is not seed-deterministic"
        );
    }

    #[test]
    fn family_fp_gate() {
        let mut rng = SEED ^ 0xC0FF_EE00;
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
            let family = surrogate_family_maxima(&triads, DIM, 50, SEED ^ (t as u64), false);
            meas += 1;
            if !per_cell_survivors(&cells, &family, 95.0).is_empty() {
                fp += 1;
            }
        }
        println!("per-cell-FP gate: {fp} of {meas} measurable trials carried a survivor");
        assert!(
            meas >= 16,
            "per-cell-FP gate: {} of {trials} measurable — the machine stays silent too often",
            meas
        );
        assert!(
            fp <= 6,
            "per-cell-FP gate: {fp} of {meas} above their own null — the per-cell rule exceeds chance"
        );
    }

    #[test]
    fn coherent_null_fp_gate() {
        let mut rng = SEED ^ 0xC0FF_1E11;
        let trials = 20usize;
        let mut fp = 0usize;
        let mut meas = 0usize;
        for t in 0..trials {
            let a = rich_series(400, &mut rng);
            let b = rich_series(400, &mut rng);
            let members = vec![("S01".to_string(), a), ("S02".to_string(), b)];
            let triads = vec![("G01".to_string(), members)];
            let cells = observed_cells(&triads, DIM);
            if cells.is_empty() {
                continue;
            }
            let family = surrogate_family_maxima(&triads, DIM, 50, SEED ^ (t as u64), true);
            meas += 1;
            if !per_cell_survivors(&cells, &family, 95.0).is_empty() {
                fp += 1;
            }
        }
        println!("coherent-per-cell-FP gate: {fp} of {meas} measurable trials carried a survivor");
        assert!(
            meas >= 16,
            "coherent-per-cell-FP gate: {} of {trials} measurable — the machine stays silent too often",
            meas
        );
        assert!(
            fp <= 6,
            "coherent-per-cell-FP gate: {fp} of {meas} above their own coherent null — the per-cell rule exceeds chance"
        );
    }

    #[test]
    fn nominees_round_trip() {
        let text = "# task\ttriad\tdriver\ttarget\tslot\tte\tthreshold\npddecision\tG01\tA\tB\t0\t1.5e0\t2.5e-1\n";
        let n = parse_nominees(text);
        assert_eq!(n.len(), 1);
        assert_eq!(n[0].task, "pddecision");
        assert_eq!(n[0].triad, "G01");
        assert_eq!(n[0].driver, "A");
        assert_eq!(n[0].target, "B");
        assert_eq!(n[0].slot, 0);
        assert_eq!(n[0].te, 1.5);
        assert_eq!(n[0].threshold, 0.25);
    }

    #[test]
    fn confirmation_confirms_the_strong_pair_against_its_own_null() {
        let mut rng = SEED ^ 0x0C0F_FEE1;
        let n = 800usize;
        let delay = 8usize;
        let a = ar1_sine(n, 0.6, 36.0, 0.0, 0.0, &mut rng);
        let mut b = vec![0.0f32; n];
        let mut x = 0.0f64;
        for t in 0..n {
            x = 0.5 * x
                + if t >= delay {
                    0.9 * a[t - delay] as f64
                } else {
                    0.0
                }
                + (next_rng(&mut rng) * 0.02 - 0.01);
            b[t] = x as f32;
        }
        let triads = vec![(
            "G01".to_string(),
            vec![("A".to_string(), a), ("B".to_string(), b)],
        )];
        let nominee = Nominee {
            task: "pddecision".to_string(),
            triad: "G01".to_string(),
            driver: "A".to_string(),
            target: "B".to_string(),
            slot: 0,
            te: 0.0,
            threshold: 0.0,
        };
        let plan = vec![(0usize, 0usize, 1usize, &nominee)];
        let dists = confirmation_cell_nulls(&triads, &plan, DIM, 200, CONFIRM_SEED, false);
        let threshold = percentile(&dists[0], 99.0).expect("the per-cell null is measurable");
        let observed = topological_te_estimate(&triads[0].1[1].1, &triads[0].1[0].1, DIM)
            .expect("the observed cell is estimated");
        assert!(
            observed.te > threshold,
            "the confirmation wiring confirms the strong pair against its own per-cell null: TE {:.4e} vs p99 {:.4e}",
            observed.te,
            threshold
        );
    }
}
