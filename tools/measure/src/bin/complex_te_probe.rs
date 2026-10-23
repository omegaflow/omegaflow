use std::cmp::Ordering;

use omegaflow::archivar::trisp_nse;
use omegaflow::te::{
    block_bootstrap_surrogate, find_cross_mi_lag, phase_randomized_surrogate,
    surrogate_rank_p_value, transfer_entropy_conditional, transfer_entropy_lag, z_phase_surrogate,
};

const SEED: u64 = 0x9E37_79B9_7F4A_7C15;
const DEFAULT_N_PERM: usize = 200;
const MIN_N: usize = 8;
const MAX_LAG: usize = 3;

#[derive(Clone, Copy)]
enum NullKind {
    PhaseRandom,
    Block(usize),
}

impl NullKind {
    fn label(self) -> String {
        match self {
            NullKind::PhaseRandom => "phase-randomized".to_string(),
            NullKind::Block(block) => format!("block-bootstrap block {block}"),
        }
    }

    fn surrogate(self, v: &[f32], rng: &mut u64) -> Vec<f32> {
        match self {
            NullKind::PhaseRandom => phase_randomized_surrogate(v, rng),
            NullKind::Block(block) => block_bootstrap_surrogate(v, block, rng),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Arm {
    Raw,
    DetrendP,
    ConditionalP,
}

impl Arm {
    fn parse(s: &str) -> Option<Arm> {
        match s {
            "raw" => Some(Arm::Raw),
            "detrend-p" => Some(Arm::DetrendP),
            "conditional-p" => Some(Arm::ConditionalP),
            _ => None,
        }
    }

    fn label(self) -> &'static str {
        match self {
            Arm::Raw => "raw",
            Arm::DetrendP => "detrend-along-p",
            Arm::ConditionalP => "conditional-p (pTE | p)",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Estimator {
    Scalar,
    Conditional,
}

#[derive(Clone)]
struct Channel {
    name: String,
    values: Vec<Option<f32>>,
}

struct Cell {
    driver: usize,
    target: usize,
    n: usize,
    lag: usize,
    te: Option<f64>,
    mean: Option<f64>,
    sd: Option<f64>,
    thr: Option<f64>,
    null_max: Option<f64>,
    p: Option<f64>,
}

struct Analysis {
    cells: Vec<Cell>,
    family: Option<f64>,
    holds: bool,
    n_perm: usize,
    null_label: String,
    arm: Arm,
}

fn arg_after<'a>(args: &'a [String], flag: &str) -> Option<&'a str> {
    args.iter()
        .position(|a| a == flag)
        .and_then(|i| args.get(i + 1))
        .map(|s| s.as_str())
}

fn arg_usize(args: &[String], flag: &str) -> Option<usize> {
    arg_after(args, flag).and_then(|v| v.parse().ok())
}

fn cell_seed(driver: usize, target: usize) -> u64 {
    SEED.wrapping_add((driver as u64).wrapping_mul(0x0000_0100_0000_01b3))
        .wrapping_add(target as u64)
}

fn paired(target: &[Option<f32>], driver: &[Option<f32>]) -> Option<(Vec<f32>, Vec<f32>)> {
    let mut t = Vec::new();
    let mut d = Vec::new();
    for (tv, dv) in target.iter().zip(driver.iter()) {
        if let (Some(a), Some(b)) = (tv, dv)
            && a.is_finite()
            && b.is_finite()
        {
            t.push(*a);
            d.push(*b);
        }
    }
    if t.len() < MIN_N {
        return None;
    }
    Some((t, d))
}

fn aligned_p(
    target: &[Option<f32>],
    driver: &[Option<f32>],
    p: &[f64],
) -> Option<(Vec<f32>, Vec<f32>, Vec<f64>)> {
    if target.len() != driver.len() || target.len() != p.len() {
        return None;
    }
    let mut t = Vec::new();
    let mut d = Vec::new();
    let mut pv = Vec::new();
    for i in 0..target.len() {
        if let (Some(a), Some(b)) = (target[i], driver[i])
            && a.is_finite()
            && b.is_finite()
            && p[i].is_finite()
        {
            t.push(a);
            d.push(b);
            pv.push(p[i]);
        }
    }
    if t.len() < MIN_N {
        return None;
    }
    Some((t, d, pv))
}

fn detrend_along_p(v: &[f32], p: &[f64]) -> Option<Vec<f32>> {
    let n = v.len();
    if n != p.len() || n < 3 {
        return None;
    }
    if v.iter().any(|x| !x.is_finite()) || p.iter().any(|x| !x.is_finite()) {
        return None;
    }
    let pmean = p.iter().sum::<f64>() / n as f64;
    let vmean = v.iter().map(|&x| x as f64).sum::<f64>() / n as f64;
    let mut sxx = 0.0;
    let mut sxy = 0.0;
    for i in 0..n {
        let dp = p[i] - pmean;
        sxx += dp * dp;
        sxy += dp * (v[i] as f64 - vmean);
    }
    if !(sxx > 0.0) {
        return None;
    }
    let b = sxy / sxx;
    let a = vmean - b * pmean;
    Some(
        (0..n)
            .map(|i| (v[i] as f64 - (a + b * p[i])) as f32)
            .collect(),
    )
}

fn null_stats(
    target: &[f32],
    driver: &[f32],
    covar: Option<&[f32]>,
    lag: usize,
    seed: u64,
    n_perm: usize,
    null: NullKind,
    est: Estimator,
    observed: f64,
) -> Option<(f64, f64, f64, f64, f64)> {
    let c = match (est, covar) {
        (Estimator::Conditional, Some(c)) => c,
        (Estimator::Conditional, None) => return None,
        (Estimator::Scalar, _) => &[],
    };
    let mut rng = seed.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut vals: Vec<f64> = Vec::with_capacity(n_perm);
    for _ in 0..n_perm {
        let te = match est {
            Estimator::Scalar => {
                let surrogate = null.surrogate(driver, &mut rng);
                transfer_entropy_lag(target, &surrogate, lag)
            }
            Estimator::Conditional => {
                let Some(surrogate) = z_phase_surrogate(driver, c, &mut rng) else {
                    continue;
                };
                transfer_entropy_conditional(target, &surrogate, c, lag)
            }
        };
        if let Some(te) = te {
            vals.push(te);
        }
    }
    if vals.len() < 2 {
        return None;
    }
    let n = vals.len() as f64;
    let mean = vals.iter().sum::<f64>() / n;
    let var = vals.iter().map(|&v| (v - mean) * (v - mean)).sum::<f64>() / n;
    let sd = var.sqrt();
    let null_max = vals.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let p = surrogate_rank_p_value(observed, &vals)?;
    Some((mean, sd, mean + 2.0 * sd, null_max, p))
}

fn cell_from(
    driver: usize,
    target: usize,
    n: usize,
    lag: usize,
    te: Option<f64>,
    stats: Option<(f64, f64, f64, f64, f64)>,
) -> Cell {
    match stats {
        Some((mean, sd, thr, null_max, p)) => Cell {
            driver,
            target,
            n,
            lag,
            te,
            mean: Some(mean),
            sd: Some(sd),
            thr: Some(thr),
            null_max: Some(null_max),
            p: Some(p),
        },
        None => Cell {
            driver,
            target,
            n,
            lag,
            te,
            mean: None,
            sd: None,
            thr: None,
            null_max: None,
            p: None,
        },
    }
}

fn eval_cell(
    driver: usize,
    target: usize,
    channels: &[Channel],
    p: Option<&[f64]>,
    n_perm: usize,
    null: NullKind,
    arm: Arm,
) -> Cell {
    let seed = cell_seed(driver, target);

    if arm == Arm::Raw {
        let Some((t, d)) = paired(&channels[target].values, &channels[driver].values) else {
            return cell_from(driver, target, 0, 0, None, None);
        };
        let n = t.len();
        if n <= MIN_N {
            return cell_from(driver, target, n, 0, None, None);
        }
        let max_lag = n.saturating_sub(MIN_N).min(MAX_LAG);
        let tf: Vec<f64> = t.iter().map(|&v| v as f64).collect();
        let df: Vec<f64> = d.iter().map(|&v| v as f64).collect();
        let Some(lag) = find_cross_mi_lag(&tf, &df, max_lag) else {
            return cell_from(driver, target, n, 0, None, None);
        };
        let Some(te) = transfer_entropy_lag(&t, &d, lag) else {
            return cell_from(driver, target, n, lag, None, None);
        };
        let stats = null_stats(&t, &d, None, lag, seed, n_perm, null, Estimator::Scalar, te);
        return cell_from(driver, target, n, lag, Some(te), stats);
    }

    let Some(pv) = p else {
        return cell_from(driver, target, 0, 0, None, None);
    };
    let Some((t, d, pv)) = aligned_p(&channels[target].values, &channels[driver].values, pv) else {
        return cell_from(driver, target, 0, 0, None, None);
    };
    let n = t.len();
    if n <= MIN_N {
        return cell_from(driver, target, n, 0, None, None);
    }
    let max_lag = n.saturating_sub(MIN_N).min(MAX_LAG);
    let tf: Vec<f64> = t.iter().map(|&v| v as f64).collect();
    let df: Vec<f64> = d.iter().map(|&v| v as f64).collect();
    let Some(lag) = find_cross_mi_lag(&tf, &df, max_lag) else {
        return cell_from(driver, target, n, 0, None, None);
    };

    match arm {
        Arm::DetrendP => {
            let Some(tr) = detrend_along_p(&t, &pv) else {
                return cell_from(driver, target, n, 0, None, None);
            };
            let Some(dr) = detrend_along_p(&d, &pv) else {
                return cell_from(driver, target, n, 0, None, None);
            };
            let Some(te) = transfer_entropy_lag(&tr, &dr, lag) else {
                return cell_from(driver, target, n, lag, None, None);
            };
            let stats = null_stats(
                &tr,
                &dr,
                None,
                lag,
                seed,
                n_perm,
                null,
                Estimator::Scalar,
                te,
            );
            cell_from(driver, target, n, lag, Some(te), stats)
        }
        Arm::ConditionalP => {
            let covar: Vec<f32> = pv.iter().map(|&v| v as f32).collect();
            let Some(te) = transfer_entropy_conditional(&t, &d, &covar, lag) else {
                return cell_from(driver, target, n, lag, None, None);
            };
            let stats = null_stats(
                &t,
                &d,
                Some(&covar),
                lag,
                seed,
                n_perm,
                null,
                Estimator::Conditional,
                te,
            );
            cell_from(driver, target, n, lag, Some(te), stats)
        }
        Arm::Raw => cell_from(driver, target, n, lag, None, None),
    }
}

fn analyse(
    channels: &[Channel],
    p: Option<&[f64]>,
    n_perm: usize,
    null: NullKind,
    arm: Arm,
    threads: usize,
) -> Analysis {
    let count = channels.len();
    let mut ordered: Vec<(usize, usize)> = Vec::new();
    for driver in 0..count {
        for target in 0..count {
            if driver != target {
                ordered.push((driver, target));
            }
        }
    }
    let workers = threads.min(ordered.len());
    let chunk = if workers == 0 {
        1
    } else {
        ordered.len().div_ceil(workers)
    };
    let mut cells: Vec<Cell> = Vec::with_capacity(ordered.len());
    std::thread::scope(|scope| {
        let mut handles = Vec::new();
        for part in ordered.chunks(chunk) {
            handles.push(scope.spawn(move || {
                part.iter()
                    .map(|&(driver, target)| {
                        eval_cell(driver, target, channels, p, n_perm, null, arm)
                    })
                    .collect::<Vec<Cell>>()
            }));
        }
        for handle in handles {
            if let Ok(part) = handle.join() {
                cells.extend(part);
            }
        }
    });
    let family = cells.iter().filter_map(|c| c.null_max).reduce(f64::max);
    let holds = family.is_some_and(|fam| cells.iter().any(|c| c.te.is_some_and(|te| te > fam)));
    let null_label = match arm {
        Arm::ConditionalP => "z-phase (p-preserving)".to_string(),
        Arm::Raw | Arm::DetrendP => null.label(),
    };
    Analysis {
        cells,
        family,
        holds,
        n_perm,
        null_label,
        arm,
    }
}

fn fmt_opt(v: Option<f64>) -> String {
    match v {
        Some(x) => format!("{x:.4e}"),
        None => "absent".to_string(),
    }
}

fn print_analysis(a: &Analysis, channels: &[Channel]) {
    println!(
        "  arm: {}; channels: {} ({}); null {} with {} permutation(s); family bound = max over directed pairs",
        a.arm.label(),
        channels.len(),
        channels
            .iter()
            .map(|c| c.name.as_str())
            .collect::<Vec<_>>()
            .join(", "),
        a.null_label,
        a.n_perm,
    );
    println!("  TE matrix (driver → follower):");
    for cell in &a.cells {
        let driver = &channels[cell.driver].name;
        let target = &channels[cell.target].name;
        let arrow = match (cell.te, a.family) {
            (Some(te), Some(fam)) if te > fam => "arrow",
            (Some(_), Some(_)) => "still",
            _ => "pending",
        };
        if cell.te.is_none() {
            println!(
                "    {driver:>10} → {target:<10} | no TE (n {} < the lagged floor)",
                cell.n
            );
            continue;
        }
        println!(
            "    {driver:>10} → {target:<10} | n {} lag {} | TE {} | thr {} (mean {}, σ {}) | p {} | fam {} | {}",
            cell.n,
            cell.lag,
            fmt_opt(cell.te),
            fmt_opt(cell.thr),
            fmt_opt(cell.mean),
            fmt_opt(cell.sd),
            fmt_opt(cell.p),
            fmt_opt(a.family),
            arrow,
        );
    }
    match a.family {
        Some(fam) => println!("  family bound: {fam:.4e}"),
        None => println!("  family bound: absent (null carries fewer than two surrogates)"),
    }
    println!(
        "  verdict: {}",
        if a.holds { "holds" } else { "no statement" }
    );
}

fn channel_from_column(name: &str, values: Vec<Option<f64>>) -> Channel {
    Channel {
        name: name.to_string(),
        values: values.into_iter().map(|v| v.map(|x| x as f32)).collect(),
    }
}

fn channels_from_bin(table: &trisp_nse::TrispNseTable) -> Vec<Channel> {
    let nrows = table.rows.len();
    let order: Vec<usize> = match table.column_index("tau") {
        Some(ti)
            if table
                .rows
                .iter()
                .all(|r| r.get(ti).copied().flatten().is_some()) =>
        {
            let mut idx: Vec<usize> = (0..nrows).collect();
            idx.sort_by(|&a, &b| {
                let ta = table.rows[a][ti];
                let tb = table.rows[b][ti];
                ta.partial_cmp(&tb)
                    .unwrap_or(Ordering::Equal)
                    .then(a.cmp(&b))
            });
            idx
        }
        _ => (0..nrows).collect(),
    };
    let mut channels = Vec::new();
    for (j, name) in table.columns.iter().enumerate() {
        if name == "tau" || name == "file" {
            continue;
        }
        let values: Vec<Option<f64>> = order
            .iter()
            .map(|&r| table.rows[r].get(j).copied().flatten())
            .collect();
        if values.iter().filter(|v| v.is_some()).count() >= MIN_N {
            channels.push(channel_from_column(name, values));
        }
    }
    channels
}

fn run_bin(path: &str, n_perm: usize, null: NullKind, arm: Arm, threads: usize) -> bool {
    if arm != Arm::Raw {
        println!(
            "arm {} needs a p-ordered cross-run series (--multi); a single bin carries no p — no statement",
            arm.label(),
        );
        return false;
    }
    let bytes = match std::fs::read(path) {
        Ok(b) => b,
        Err(_) => {
            println!("{path} reads void — no statement (the bin is the measurement)");
            return false;
        }
    };
    let Some(table) = trisp_nse::parse_bin(&bytes) else {
        println!("{path} parses void — no statement (TNSE expected)");
        return false;
    };
    let channels = channels_from_bin(&table);
    println!(
        "complex_te_probe — {path}: {} row(s), {} column(s), {} channel(s) carry the floor",
        table.rows.len(),
        table.columns.len(),
        channels.len(),
    );
    if channels.len() < 2 {
        println!("  verdict: no statement (fewer than two channels carry the floor)");
        return false;
    }
    let analysis = analyse(&channels, None, n_perm, null, Arm::Raw, threads);
    print_analysis(&analysis, &channels);
    analysis.holds
}

#[derive(Clone, Copy, PartialEq)]
enum RunStat {
    First,
    Mean,
}

impl RunStat {
    fn parse(s: &str) -> Option<RunStat> {
        match s {
            "first" => Some(RunStat::First),
            "mean" => Some(RunStat::Mean),
            _ => None,
        }
    }

    fn label(self) -> &'static str {
        match self {
            RunStat::First => "first (smallest tau)",
            RunStat::Mean => "mean over the run's tau rows",
        }
    }
}

struct Run {
    temperature: f64,
    table: trisp_nse::TrispNseTable,
}

fn flag_values(args: &[String], flag: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut i = 0;
    while i < args.len() {
        if args[i] == flag {
            if let Some(v) = args.get(i + 1) {
                out.push(v.clone());
            }
            i += 2;
        } else {
            i += 1;
        }
    }
    out
}

fn parse_temperature(path: &str) -> Option<f64> {
    let stem = std::path::Path::new(path)
        .file_stem()?
        .to_string_lossy()
        .into_owned();
    let mut last = None;
    for token in stem.split('_') {
        if let Ok(v) = token.parse::<f64>() {
            if v.is_finite() {
                last = Some(v);
            }
        }
    }
    last
}

fn smallest_tau_row(table: &trisp_nse::TrispNseTable) -> Option<&Vec<Option<f64>>> {
    match table.column_index("tau") {
        Some(ti) => {
            let mut best: Option<(f64, &Vec<Option<f64>>)> = None;
            for row in &table.rows {
                if let Some(t) = row.get(ti).copied().flatten() {
                    if best.map_or(true, |(bt, _)| t < bt) {
                        best = Some((t, row));
                    }
                }
            }
            best.map(|(_, row)| row)
        }
        None => table.rows.first(),
    }
}

fn representative(table: &trisp_nse::TrispNseTable, column: usize, stat: RunStat) -> Option<f64> {
    match stat {
        RunStat::First => smallest_tau_row(table)?.get(column).copied().flatten(),
        RunStat::Mean => {
            let mut sum = 0.0f64;
            let mut count = 0usize;
            for row in &table.rows {
                if let Some(v) = row.get(column).copied().flatten() {
                    sum += v;
                    count += 1;
                }
            }
            if count == 0 {
                None
            } else {
                Some(sum / count as f64)
            }
        }
    }
}

fn channel_columns(runs: &[Run], wanted: &[String]) -> Vec<String> {
    if !wanted.is_empty() {
        return wanted.to_vec();
    }
    let mut cols: Vec<String> = Vec::new();
    for run in runs {
        for name in &run.table.columns {
            if name == "tau" || name == "file" {
                continue;
            }
            if !cols.iter().any(|c| c == name) {
                cols.push(name.clone());
            }
        }
    }
    cols
}

fn multi_channels(runs: &[Run], wanted: &[String], stat: RunStat) -> (Vec<Channel>, Vec<String>) {
    let names = channel_columns(runs, wanted);
    let mut channels = Vec::new();
    let mut absent = Vec::new();
    for name in names {
        let values: Vec<Option<f32>> = runs
            .iter()
            .map(|run| {
                run.table
                    .column_index(&name)
                    .and_then(|c| representative(&run.table, c, stat))
                    .map(|v| v as f32)
            })
            .collect();
        let carried = values.iter().filter(|v| v.is_some()).count();
        if carried >= MIN_N {
            channels.push(Channel { name, values });
        } else {
            absent.push(format!(
                "{name} ({carried}/{} runs carry the scalar)",
                runs.len()
            ));
        }
    }
    (channels, absent)
}

fn order_runs(runs: &mut [Run]) {
    runs.sort_by(|a, b| {
        a.temperature
            .partial_cmp(&b.temperature)
            .unwrap_or(Ordering::Equal)
    });
}

fn collect_bins(bins: &[String], dirs: &[String]) -> Vec<String> {
    let mut out: Vec<String> = bins.to_vec();
    for dir in dirs {
        let mut found: Vec<String> = Vec::new();
        match std::fs::read_dir(dir) {
            Ok(entries) => {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.extension().is_some_and(|x| x == "bin") {
                        found.push(path.to_string_lossy().into_owned());
                    }
                }
            }
            Err(_) => {
                println!("{dir} reads void — the directory is the measurement");
            }
        }
        found.sort();
        out.extend(found);
    }
    out
}

fn load_runs(paths: &[String]) -> Vec<Run> {
    let mut runs = Vec::new();
    for path in paths {
        let bytes = match std::fs::read(path) {
            Ok(b) => b,
            Err(_) => {
                println!("{path} reads void — run skipped (the bin is the measurement)");
                continue;
            }
        };
        let Some(table) = trisp_nse::parse_bin(&bytes) else {
            println!("{path} parses void — run skipped (TNSE expected)");
            continue;
        };
        let Some(temperature) = parse_temperature(path) else {
            println!("{path}: the stem carries no numeric temperature — run skipped (no order)");
            continue;
        };
        runs.push(Run { temperature, table });
    }
    order_runs(&mut runs);
    runs
}

fn run_multi(
    runs: &[Run],
    wanted: &[String],
    stat: RunStat,
    n_perm: usize,
    null: NullKind,
    arm: Arm,
    threads: usize,
) -> bool {
    if runs.len() < 2 {
        println!(
            "  verdict: no statement ({} run(s) — a cross-run series needs at least two)",
            runs.len()
        );
        return false;
    }
    println!(
        "complex_te_probe --multi: {} run(s), scalar {} , arm {}, ordered by temperature:",
        runs.len(),
        stat.label(),
        arm.label(),
    );
    for run in runs {
        println!("    T = {:.6}", run.temperature);
    }
    let (channels, absent) = multi_channels(runs, wanted, stat);
    for name in &absent {
        println!("  absent: {name}");
    }
    println!(
        "  cross-run channels: {} of {} column(s)",
        channels.len(),
        channels.len() + absent.len()
    );
    if channels.len() < 2 {
        println!("  verdict: no statement (fewer than two columns carry the floor over the runs)");
        return false;
    }
    let p: Vec<f64> = runs.iter().map(|r| r.temperature).collect();
    let analysis = analyse(&channels, Some(&p), n_perm, null, arm, threads);
    print_analysis(&analysis, &channels);
    analysis.holds
}

const SELFTEST_N: usize = 600;
const SELFTEST_TRANSIENT: usize = 100;
const SELFTEST_COUPLING: f64 = 0.30;
const SELFTEST_N_PERM: usize = 60;
const SELFTEST_ARM_N: usize = 400;
const SELFTEST_ARM_N_PERM: usize = 30;

fn selftest_bin_reader() -> bool {
    let (x, y) = coupled_henon(SELFTEST_N, SELFTEST_TRANSIENT, SELFTEST_COUPLING, 0.1, 0.2);
    let mut table = trisp_nse::TrispNseTable {
        columns: vec![
            "tau".to_string(),
            "driver_x".to_string(),
            "follower_y".to_string(),
            "absent_fit".to_string(),
        ],
        rows: Vec::new(),
        log: Vec::new(),
    };
    for i in (0..SELFTEST_N).rev() {
        table.rows.push(vec![
            Some(i as f64),
            Some(x[i] as f64),
            Some(y[i] as f64),
            None,
        ]);
    }
    let bin = trisp_nse::pack(&table);
    let Some(parsed) = trisp_nse::parse_bin(&bin) else {
        println!("  TNSE round-trip: the packed bin parses void");
        return false;
    };
    if parsed != table {
        println!("  TNSE round-trip: the parsed table diverges from the packed one");
        return false;
    }
    let channels = channels_from_bin(&parsed);
    if channels.len() != 2 {
        println!(
            "  TNSE round-trip: {} channel(s) carry the floor (expected 2)",
            channels.len()
        );
        return false;
    }
    let a = analyse(
        &channels,
        None,
        SELFTEST_N_PERM,
        NullKind::PhaseRandom,
        Arm::Raw,
        2,
    );
    let forward = a
        .cells
        .iter()
        .find(|c| c.driver == 0 && c.target == 1)
        .and_then(|c| c.te);
    println!(
        "  TNSE round-trip: channels {} / TE(x→y) {} / family {} / holds {}",
        channels
            .iter()
            .map(|c| c.name.as_str())
            .collect::<Vec<_>>()
            .join(", "),
        fmt_opt(forward),
        fmt_opt(a.family),
        a.holds
    );
    if forward.is_none() || !a.holds {
        println!("  TNSE round-trip: the restored order does not carry the measured direction");
        return false;
    }
    true
}

fn coupled_henon(n: usize, transient: usize, c: f64, x0: f64, y0: f64) -> (Vec<f32>, Vec<f32>) {
    let total = n + transient;
    let mut xs = vec![0.0f64; total];
    let mut ys = vec![0.0f64; total];
    xs[0] = x0;
    ys[0] = y0;
    xs[1] = 0.0;
    ys[1] = 0.0;
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

fn selftest_temperature_parse() -> bool {
    let cases: [(&str, Option<f64>); 5] = [
        ("sample_a.bin", Some(10.0)),
        ("run_1.25_abc.bin", Some(1.25)),
        ("no_number_here.bin", None),
        ("series_2_3.bin", Some(3.0)),
        ("cold_-25.5_sc9.bin", Some(-25.5)),
    ];
    for (stem, want) in cases {
        let got = parse_temperature(stem);
        let ok = match (got, want) {
            (Some(g), Some(w)) => (g - w).abs() < 1.0e-12,
            (None, None) => true,
            _ => false,
        };
        if !ok {
            println!("  temperature parse: {stem} -> {got:?} (expected {want:?})");
            return false;
        }
    }
    true
}

fn selftest_run_scalar() -> bool {
    let table = trisp_nse::TrispNseTable {
        columns: vec!["tau".to_string(), "c".to_string()],
        rows: vec![
            vec![Some(3.0), Some(30.0)],
            vec![Some(1.0), Some(10.0)],
            vec![Some(2.0), Some(20.0)],
        ],
        log: Vec::new(),
    };
    let first = representative(&table, 1, RunStat::First);
    let mean = representative(&table, 1, RunStat::Mean);
    if first != Some(10.0) {
        println!("  run scalar: first = {first:?} (expected 10.0 at the smallest tau)");
        return false;
    }
    if mean != Some(20.0) {
        println!("  run scalar: mean = {mean:?} (expected 20.0)");
        return false;
    }
    true
}

fn selftest_multi_run() -> bool {
    let (x, y) = coupled_henon(SELFTEST_N, SELFTEST_TRANSIENT, SELFTEST_COUPLING, 0.1, 0.2);
    let (u, w) = coupled_henon(SELFTEST_N, SELFTEST_TRANSIENT, 0.0, 0.3, -0.1);
    let mut runs = Vec::with_capacity(SELFTEST_N);
    for i in 0..SELFTEST_N {
        let temperature = (i + 1) as f64;
        let table = trisp_nse::TrispNseTable {
            columns: vec![
                "tau".to_string(),
                "driver_x".to_string(),
                "follower_y".to_string(),
                "null_a".to_string(),
                "null_b".to_string(),
            ],
            rows: vec![vec![
                Some(0.0),
                Some(x[i] as f64),
                Some(y[i] as f64),
                Some(u[i] as f64),
                Some(w[i] as f64),
            ]],
            log: Vec::new(),
        };
        runs.push(Run { temperature, table });
    }
    runs.reverse();
    order_runs(&mut runs);
    if runs
        .windows(2)
        .any(|pair| pair[0].temperature > pair[1].temperature)
    {
        println!("  multi-run: the temperature order is not monotone after ordering");
        return false;
    }

    let mut parsed_runs = Vec::with_capacity(runs.len());
    for run in &runs {
        let bin = trisp_nse::pack(&run.table);
        let Some(table) = trisp_nse::parse_bin(&bin) else {
            println!("  multi-run: a packed run parses void");
            return false;
        };
        parsed_runs.push(Run {
            temperature: run.temperature,
            table,
        });
    }

    let (channels, absent) = multi_channels(&parsed_runs, &[], RunStat::First);
    if !absent.is_empty() {
        println!("  multi-run: unexpected absent column(s) {absent:?}");
        return false;
    }
    let analysis = analyse(
        &channels,
        None,
        SELFTEST_N_PERM,
        NullKind::PhaseRandom,
        Arm::Raw,
        2,
    );
    let fwd = analysis
        .cells
        .iter()
        .find(|c| channels[c.driver].name == "driver_x" && channels[c.target].name == "follower_y")
        .and_then(|c| c.te);
    let rev = analysis
        .cells
        .iter()
        .find(|c| channels[c.driver].name == "follower_y" && channels[c.target].name == "driver_x")
        .and_then(|c| c.te);
    println!(
        "  multi-run: TE(x→y) {} / TE(y→x) {} / family {} / holds {}",
        fmt_opt(fwd),
        fmt_opt(rev),
        fmt_opt(analysis.family),
        analysis.holds
    );
    if fwd.is_none() || rev.is_none() {
        println!("  multi-run: the estimator carries no statement (floor unmet)");
        return false;
    }
    if !analysis.holds || fwd <= rev {
        println!("  multi-run: the measured cross-run direction does not stand (expected x → y)");
        return false;
    }

    let null_channels: Vec<Channel> = channels
        .iter()
        .filter(|c| c.name == "null_a" || c.name == "null_b")
        .cloned()
        .collect();
    let null_analysis = analyse(
        &null_channels,
        None,
        SELFTEST_N_PERM,
        NullKind::PhaseRandom,
        Arm::Raw,
        2,
    );
    println!(
        "  multi-run null pair: family {} / holds {}",
        fmt_opt(null_analysis.family),
        null_analysis.holds
    );
    if null_analysis.holds {
        println!("  multi-run null pair: an arrow stands where none is coupled");
        return false;
    }
    true
}

fn trended_pair(c: f64, x0: f64, y0: f64, slope: f64) -> (Vec<f32>, Vec<f32>, Vec<f64>) {
    let (x, y) = coupled_henon(SELFTEST_ARM_N, SELFTEST_TRANSIENT, c, x0, y0);
    let p: Vec<f64> = (0..SELFTEST_ARM_N).map(|i| i as f64).collect();
    let trend = |v: &[f32]| -> Vec<f32> {
        v.iter()
            .enumerate()
            .map(|(i, &x)| (x as f64 + slope * i as f64) as f32)
            .collect()
    };
    (trend(&x), trend(&y), p)
}

fn arm_channel(name: &str, v: &[f32]) -> Channel {
    Channel {
        name: name.to_string(),
        values: v.iter().map(|&x| Some(x)).collect(),
    }
}

fn arm_te(a: &Analysis, driver: usize, target: usize) -> Option<f64> {
    a.cells
        .iter()
        .find(|c| c.driver == driver && c.target == target)
        .and_then(|c| c.te)
}

fn selftest_detrend_arm() -> bool {
    let (cx, cy, p) = trended_pair(SELFTEST_COUPLING, 0.1, 0.2, 0.5);
    let coupled = vec![arm_channel("driver_x", &cx), arm_channel("follower_y", &cy)];
    let a = analyse(
        &coupled,
        Some(&p),
        SELFTEST_ARM_N_PERM,
        NullKind::PhaseRandom,
        Arm::DetrendP,
        2,
    );
    let fwd = arm_te(&a, 0, 1);
    println!(
        "  detrend-along-p coupled: TE(x→y) {} / family {} / holds {}",
        fmt_opt(fwd),
        fmt_opt(a.family),
        a.holds
    );
    if fwd.is_none() || !a.holds {
        println!("  detrend-along-p: the trended coupled direction does not stand after detrend");
        return false;
    }
    let (ix, iy, ip) = trended_pair(0.0, 0.3, -0.1, 0.5);
    let independent = vec![arm_channel("null_a", &ix), arm_channel("null_b", &iy)];
    let b = analyse(
        &independent,
        Some(&ip),
        SELFTEST_ARM_N_PERM,
        NullKind::PhaseRandom,
        Arm::DetrendP,
        2,
    );
    println!(
        "  detrend-along-p null pair: family {} / holds {}",
        fmt_opt(b.family),
        b.holds
    );
    if b.holds {
        println!("  detrend-along-p: a common trend survives the detrend as an arrow");
        return false;
    }
    true
}

fn lcg_step(state: &mut u64) -> f64 {
    *state = state
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);
    ((*state >> 40) as f64 / (1u64 << 24) as f64) - 0.5
}

fn stochastic_pair(slope: f64, coupled: bool) -> (Vec<f32>, Vec<f32>, Vec<f64>) {
    let n = SELFTEST_ARM_N;
    let mut state = 0x243f_6a88_85a3_08d3u64;
    let mut d = vec![0.0f64; n];
    let mut x = vec![0.0f64; n];
    let mut p = vec![0.0f64; n];
    for i in 0..n {
        let e_d = lcg_step(&mut state);
        let e_x = lcg_step(&mut state);
        let prev = if i > 0 { d[i - 1] } else { 0.0 };
        p[i] = i as f64;
        d[i] = 0.5 * prev + 0.5 * e_d + slope * p[i];
        let coupling = if coupled { 0.7 * prev } else { 0.0 };
        x[i] = coupling + 0.8 * e_x + slope * p[i];
    }
    let to_f32 = |v: &[f64]| v.iter().map(|&z| z as f32).collect();
    (to_f32(&d), to_f32(&x), p)
}

fn selftest_conditional_arm() -> bool {
    let (cx, cy, p) = stochastic_pair(0.02, true);
    let coupled = vec![arm_channel("driver_x", &cx), arm_channel("follower_y", &cy)];
    let a = analyse(
        &coupled,
        Some(&p),
        SELFTEST_ARM_N_PERM,
        NullKind::PhaseRandom,
        Arm::ConditionalP,
        2,
    );
    let fwd = arm_te(&a, 0, 1);
    println!(
        "  conditional-p coupled: pTE(x→y|p) {} / family {} / holds {}",
        fmt_opt(fwd),
        fmt_opt(a.family),
        a.holds
    );
    let (ix, iy, ip) = stochastic_pair(0.02, false);
    let independent = vec![arm_channel("null_a", &ix), arm_channel("null_b", &iy)];
    let b = analyse(
        &independent,
        Some(&ip),
        SELFTEST_ARM_N_PERM,
        NullKind::PhaseRandom,
        Arm::ConditionalP,
        2,
    );
    let null_fwd = arm_te(&b, 0, 1);
    println!(
        "  conditional-p null pair: pTE(x→y|p) {} / family {} / holds {}",
        fmt_opt(null_fwd),
        fmt_opt(b.family),
        b.holds
    );
    let Some(fwd) = fwd else {
        println!("  conditional-p: the estimator carries no statement on the coupled pair");
        return false;
    };
    if b.holds {
        println!("  conditional-p: a shared trend stands as a conditional arrow");
        return false;
    }
    if let Some(nf) = null_fwd
        && fwd <= nf
    {
        println!("  conditional-p: the coupled pTE does not exceed the confounded null pTE");
        return false;
    }
    if a.holds {
        println!("  conditional-p: coupled x → y clears its family bound");
    } else {
        println!(
            "  conditional-p: coupled pTE exceeds the confounded null; the surrogate family bound stays (weak coupling at n {SELFTEST_ARM_N})"
        );
    }
    true
}

fn selftest() -> bool {
    let mut ok = true;
    println!(
        "complex_te_probe --selftest — synthetic only (n {SELFTEST_N}, cold {SELFTEST_N_PERM} permutations)"
    );

    let (x, y) = coupled_henon(SELFTEST_N, SELFTEST_TRANSIENT, SELFTEST_COUPLING, 0.1, 0.2);
    let coupled = vec![
        Channel {
            name: "driver_x".to_string(),
            values: x.iter().map(|&v| Some(v)).collect(),
        },
        Channel {
            name: "follower_y".to_string(),
            values: y.iter().map(|&v| Some(v)).collect(),
        },
    ];
    let a = analyse(
        &coupled,
        None,
        SELFTEST_N_PERM,
        NullKind::PhaseRandom,
        Arm::Raw,
        2,
    );
    let forward = a
        .cells
        .iter()
        .find(|c| c.driver == 0 && c.target == 1)
        .and_then(|c| c.te);
    let reverse = a
        .cells
        .iter()
        .find(|c| c.driver == 1 && c.target == 0)
        .and_then(|c| c.te);
    println!(
        "  coupled pair: TE(x→y) {} / TE(y→x) {} / family {} / holds {}",
        fmt_opt(forward),
        fmt_opt(reverse),
        fmt_opt(a.family),
        a.holds
    );
    if forward.is_none() || reverse.is_none() {
        println!("  known pair: the estimator carries no statement (floor unmet)");
        ok = false;
    } else if !a.holds || forward <= reverse {
        println!("  known pair: the measured direction does not stand (expected x → y)");
        ok = false;
    } else {
        println!("  known pair: x → y holds, y → x stays");
    }

    let (x2, y2) = coupled_henon(SELFTEST_N, SELFTEST_TRANSIENT, 0.0, 0.3, -0.1);
    let independent = vec![
        Channel {
            name: "null_a".to_string(),
            values: x2.iter().map(|&v| Some(v)).collect(),
        },
        Channel {
            name: "null_b".to_string(),
            values: y2.iter().map(|&v| Some(v)).collect(),
        },
    ];
    let b = analyse(
        &independent,
        None,
        SELFTEST_N_PERM,
        NullKind::PhaseRandom,
        Arm::Raw,
        2,
    );
    println!(
        "  null pair: family {} / holds {}",
        fmt_opt(b.family),
        b.holds
    );
    if b.family.is_none() {
        println!(
            "  null pair: the surrogate null carries no statement (fewer than two surrogates)"
        );
        ok = false;
    } else if b.holds {
        println!("  null pair: an arrow stands where none is coupled");
        ok = false;
    } else {
        println!("  null pair: no arrow — the null is not rejected");
    }

    if !selftest_bin_reader() {
        ok = false;
    }
    if !selftest_temperature_parse() {
        ok = false;
    }
    if !selftest_run_scalar() {
        ok = false;
    }
    if !selftest_multi_run() {
        ok = false;
    }
    if !selftest_detrend_arm() {
        ok = false;
    }
    if !selftest_conditional_arm() {
        ok = false;
    }

    println!("  selftest: {}", if ok { "PASS" } else { "FAIL" });
    ok
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.iter().any(|a| a == "--selftest") {
        if !selftest() {
            std::process::exit(1);
        }
        return;
    }

    let n_perm = arg_usize(&args, "--n-perm")
        .filter(|&n| n >= 2)
        .unwrap_or(DEFAULT_N_PERM);
    let block = arg_usize(&args, "--block");
    let threads = arg_usize(&args, "--threads")
        .filter(|&n| n > 0)
        .or_else(|| std::thread::available_parallelism().ok().map(|p| p.get()))
        .unwrap_or(1);
    let null = match block {
        Some(b) if b > 0 => NullKind::Block(b),
        _ => NullKind::PhaseRandom,
    };

    let stat = match arg_after(&args, "--run-stat") {
        None => RunStat::First,
        Some(s) => match RunStat::parse(s) {
            Some(stat) => stat,
            None => {
                eprintln!("--run-stat expects first|mean");
                std::process::exit(2);
            }
        },
    };
    let arm = match arg_after(&args, "--arm") {
        None => Arm::Raw,
        Some(s) => match Arm::parse(s) {
            Some(arm) => arm,
            None => {
                eprintln!("--arm expects raw|detrend-p|conditional-p");
                std::process::exit(2);
            }
        },
    };
    let wanted: Vec<String> = flag_values(&args, "--run-scalar")
        .into_iter()
        .filter(|s| s != "all")
        .collect();
    let bins = flag_values(&args, "--bin");
    let dirs = flag_values(&args, "--dir");
    let multi = args.iter().any(|a| a == "--multi") || !dirs.is_empty() || bins.len() > 1;

    if bins.is_empty() && dirs.is_empty() {
        eprintln!(
            "complex_te_probe --bin <path> [--n-perm N] [--block N] [--threads N] | \
             --dir <path> [--bin <path> ...] [--run-scalar <col> ...] \
             [--run-stat first|mean] [--arm raw|detrend-p|conditional-p] [--multi] \
             [--n-perm N] [--block N] [--threads N] | --selftest"
        );
        std::process::exit(2);
    }

    if !multi && bins.len() == 1 {
        run_bin(&bins[0], n_perm, null, arm, threads);
        return;
    }

    let paths = collect_bins(&bins, &dirs);
    if paths.is_empty() {
        println!("no bin found — no statement (0 honored)");
        std::process::exit(1);
    }
    let runs = load_runs(&paths);
    if runs.is_empty() {
        println!("no run carries a numeric temperature — no statement (0 honored)");
        std::process::exit(1);
    }
    run_multi(&runs, &wanted, stat, n_perm, null, arm, threads);
}
