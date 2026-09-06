use std::env;

use omegaflow::archivar::{C_LIGHT, PARSEC_M};
use omegaflow::te::{phase_randomized_surrogate, transfer_entropy_lag};

const PHI: f64 = 1.618033988749895;
const LOW_BAND: usize = 3;
const DEFAULT_TOL_FRAC: f64 = 0.25;
const DEFAULT_ALPHA: f64 = 0.05;
const DEFAULT_SURROGATES: usize = 100;
const MIN_N: usize = 30;
const SEED: u64 = 0x9E37_79B9_7F4A_7C15;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Verdict {
    EchoCandidate,
    SignificantNotPhysical,
    Field,
    Absent,
    Pending,
}

impl Verdict {
    fn label(self) -> &'static str {
        match self {
            Verdict::EchoCandidate => "echo-candidate",
            Verdict::SignificantNotPhysical => "significant-but-not-physical",
            Verdict::Field => "field",
            Verdict::Absent => "absent",
            Verdict::Pending => "pending",
        }
    }
}

struct Source {
    name: String,
    ra_deg: f64,
    dec_deg: f64,
    dist_pc: Option<f64>,
    flux: Vec<f32>,
}

struct DirectedVerdict {
    driver: String,
    target: String,
    verdict: Verdict,
    separation_lt_s: Option<f64>,
    window: Option<(usize, usize)>,
    best_lag: Option<usize>,
    best_te: Option<f64>,
    best_threshold: Option<f64>,
}

struct MeasCell {
    driver: usize,
    target: usize,
    lag: usize,
    te: f64,
    mean: f64,
    sd: f64,
    z: f64,
    zs: Vec<f64>,
}

struct ScreenReport {
    directed: Vec<DirectedVerdict>,
    n_cells: usize,
    n_surr: usize,
    alpha: f64,
    z_threshold: f64,
}

fn icrs_pos_m(ra_deg: f64, dec_deg: f64, dist_pc: f64) -> [f64; 3] {
    let ra = ra_deg.to_radians();
    let dec = dec_deg.to_radians();
    let d = dist_pc * PARSEC_M;
    [
        d * dec.cos() * ra.cos(),
        d * dec.cos() * ra.sin(),
        d * dec.sin(),
    ]
}

fn separation_light_seconds(a: &Source, b: &Source) -> Option<f64> {
    let (da, db) = (a.dist_pc?, b.dist_pc?);
    let pa = icrs_pos_m(a.ra_deg, a.dec_deg, da);
    let pb = icrs_pos_m(b.ra_deg, b.dec_deg, db);
    let dx = pa[0] - pb[0];
    let dy = pa[1] - pb[1];
    let dz = pa[2] - pb[2];
    let chord_m = (dx * dx + dy * dy + dz * dz).sqrt();
    Some(chord_m / C_LIGHT)
}

fn lags_for_direction(
    n: usize,
    separation_lt_s: Option<f64>,
    cadence_s: Option<f64>,
    tol_frac: f64,
) -> (Vec<usize>, Option<(usize, usize)>) {
    let hi_max = ((n as f64 / PHI).floor() as usize).min(n.saturating_sub(8));
    if hi_max == 0 {
        return (Vec::new(), None);
    }
    let mut lags: Vec<usize> = (1..=LOW_BAND.min(hi_max)).collect();
    let window = match (separation_lt_s, cadence_s) {
        (Some(sep), Some(cad)) => {
            let lc = sep / cad;
            let lo_edge = lc * (1.0 - tol_frac);
            let hi_edge = lc * (1.0 + tol_frac);
            if hi_edge < 1.0 {
                None
            } else {
                let lo = match lo_edge.floor() {
                    b if b >= 1.0 => b as usize,
                    _ => 1,
                };
                let hi = hi_edge.ceil() as usize;
                if lo <= hi_max {
                    Some((lo, hi.min(hi_max)))
                } else {
                    None
                }
            }
        }
        _ => None,
    };
    if let Some((lo, hi)) = window {
        lags.extend(lo..=hi);
    }
    lags.sort_unstable();
    lags.dedup();
    (lags, window)
}

fn surrogate_zs(
    target: &[f32],
    driver: &[f32],
    lag: usize,
    seed: u64,
    n_surr: usize,
) -> Option<(f64, f64, f64, Vec<f64>)> {
    let te = transfer_entropy_lag(target, driver, lag)?;
    if !te.is_finite() {
        return None;
    }
    let mut rng = seed.wrapping_add(0x9e3779b97f4a7c15);
    let mut vals = Vec::with_capacity(n_surr);
    for _ in 0..n_surr {
        let s = phase_randomized_surrogate(driver, &mut rng);
        if let Some(v) = transfer_entropy_lag(target, &s, lag) {
            if v.is_finite() {
                vals.push(v);
            }
        }
    }
    if vals.len() != n_surr {
        return None;
    }
    let mean = vals.iter().sum::<f64>() / vals.len() as f64;
    let var = vals.iter().map(|&v| (v - mean) * (v - mean)).sum::<f64>() / vals.len() as f64;
    if var <= 0.0 || !var.is_finite() {
        return None;
    }
    let sd = var.sqrt();
    let zs: Vec<f64> = vals.iter().map(|&v| (v - mean) / sd).collect();
    Some((te, mean, sd, zs))
}

fn fabric_z_threshold(null_max: &[f64], alpha: f64) -> f64 {
    let mut sorted = null_max.to_vec();
    sorted.sort_by(|a, b| a.total_cmp(b));
    let n = sorted.len();
    let m = (alpha * n as f64).floor() as usize;
    let m = m.min(n.saturating_sub(1));
    sorted[n - 1 - m]
}

fn screen_set(
    sources: &[Source],
    cadence_s: Option<f64>,
    seed: u64,
    tol_frac: f64,
    n_surr: usize,
    alpha: f64,
) -> ScreenReport {
    let mut cells: Vec<MeasCell> = Vec::new();
    let mut directed_windows: Vec<(usize, usize, Option<(usize, usize)>, Option<f64>)> = Vec::new();
    for i in 0..sources.len() {
        for j in 0..sources.len() {
            if i == j {
                continue;
            }
            let n = sources[i].flux.len().min(sources[j].flux.len());
            let sep = separation_light_seconds(&sources[i], &sources[j]);
            let (lags, window) = lags_for_direction(n, sep, cadence_s, tol_frac);
            directed_windows.push((i, j, window, sep));
            let pair_seed = seed ^ (i as u64).rotate_left(16) ^ (j as u64);
            for &lag in &lags {
                let m = surrogate_zs(
                    &sources[j].flux,
                    &sources[i].flux,
                    lag,
                    pair_seed ^ lag as u64,
                    n_surr,
                );
                if let Some((te, mean, sd, zs)) = m {
                    let z = (te - mean) / sd;
                    if z.is_finite() {
                        cells.push(MeasCell {
                            driver: i,
                            target: j,
                            lag,
                            te,
                            mean,
                            sd,
                            z,
                            zs,
                        });
                    }
                }
            }
        }
    }

    let n_cells = cells.len();
    let z_threshold = if n_cells == 0 {
        0.0
    } else {
        let mut null_max = vec![f64::NEG_INFINITY; n_surr];
        for c in &cells {
            for (r, &val) in c.zs.iter().enumerate() {
                if val > null_max[r] {
                    null_max[r] = val;
                }
            }
        }
        fabric_z_threshold(&null_max, alpha)
    };

    let mut directed = Vec::new();
    for (driver, target, window, sep) in directed_windows {
        let mut best_lag = None;
        let mut best_z = f64::NEG_INFINITY;
        let mut best_te = None;
        let mut best_threshold = None;
        let mut any_significant = false;
        let mut window_significant = false;
        for c in cells
            .iter()
            .filter(|c| c.driver == driver && c.target == target)
        {
            if c.z > z_threshold {
                any_significant = true;
                if window.map_or(false, |(lo, hi)| c.lag >= lo && c.lag <= hi) {
                    window_significant = true;
                }
            }
            if c.z > best_z {
                best_z = c.z;
                best_lag = Some(c.lag);
                best_te = Some(c.te);
                best_threshold = Some(c.mean + z_threshold * c.sd);
            }
        }
        let verdict = if n_cells == 0 {
            Verdict::Absent
        } else if !any_significant {
            Verdict::Field
        } else if window.is_none() {
            Verdict::Pending
        } else if window_significant {
            Verdict::EchoCandidate
        } else {
            Verdict::SignificantNotPhysical
        };
        directed.push(DirectedVerdict {
            driver: sources[driver].name.clone(),
            target: sources[target].name.clone(),
            verdict,
            separation_lt_s: sep,
            window,
            best_lag,
            best_te,
            best_threshold,
        });
    }
    ScreenReport {
        directed,
        n_cells,
        n_surr,
        alpha,
        z_threshold,
    }
}

fn read_series(path: &str) -> Option<(Vec<f64>, Vec<f32>)> {
    let body = std::fs::read_to_string(path).ok()?;
    let mut times = Vec::new();
    let mut flux = Vec::new();
    for line in body.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut toks: Vec<&str> = line
            .split([',', ' ', '\t'])
            .filter(|t| !t.is_empty())
            .collect();
        if toks.len() == 1 {
            if let Ok(v) = toks.remove(0).parse::<f32>() {
                if v.is_finite() {
                    flux.push(v);
                }
            }
            continue;
        }
        let t = toks.remove(0);
        let v = toks.remove(0);
        if let (Ok(t), Ok(v)) = (t.parse::<f64>(), v.parse::<f32>()) {
            if t.is_finite() && v.is_finite() {
                times.push(t);
                flux.push(v);
            }
        }
    }
    if flux.is_empty() {
        return None;
    }
    Some((times, flux))
}

fn cadence_of(times: &[f64]) -> Option<f64> {
    if times.len() < 2 {
        return None;
    }
    let mut dts = Vec::with_capacity(times.len() - 1);
    for w in times.windows(2) {
        dts.push(w[1] - w[0]);
    }
    dts.sort_by(|a, b| a.total_cmp(b));
    let median = dts[dts.len() / 2];
    if median <= 0.0 {
        return None;
    }
    let uniform = dts.iter().all(|&dt| ((dt - median) / median).abs() < 1e-6);
    if !uniform {
        return None;
    }
    Some(median)
}

fn load_set(path: &str) -> Option<(Vec<Source>, Option<f64>)> {
    let body = std::fs::read_to_string(path).ok()?;
    let mut sources = Vec::new();
    let mut times_grid: Option<Vec<f64>> = None;
    let mut cadence = None;
    for line in body.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let toks: Vec<&str> = line.split_whitespace().collect();
        if toks.len() < 5 {
            eprintln!("manifest row without <path> <name> <ra> <dec> <dist|?> -> {line}");
            continue;
        }
        let (times, flux) = match read_series(toks[0]) {
            Some(v) => v,
            None => {
                eprintln!("series not readable or empty: {} -> skipped", toks[0]);
                continue;
            }
        };
        let name = toks[1].to_string();
        let Ok(ra_deg) = toks[2].parse::<f64>() else {
            eprintln!("ra not a number on row {line} -> skipped");
            continue;
        };
        let Ok(dec_deg) = toks[3].parse::<f64>() else {
            eprintln!("dec not a number on row {line} -> skipped");
            continue;
        };
        let dist_pc = if toks[4] == "?" {
            None
        } else {
            match toks[4].parse::<f64>() {
                Ok(d) => Some(d),
                Err(_) => {
                    eprintln!("dist not a number on row {line} -> skipped");
                    continue;
                }
            }
        };
        if flux.len() < MIN_N {
            eprintln!(
                "{}: n = {} < {MIN_N} -> absent, no screening",
                name,
                flux.len()
            );
            continue;
        }
        match (&times_grid, times.len()) {
            (None, 0) => times_grid = None,
            (None, _) => {
                times_grid = Some(times.clone());
                cadence = cadence_of(&times);
                if cadence.is_none() {
                    eprintln!("{}: time grid not uniform -> pending, no screening", name);
                    return None;
                }
            }
            (Some(_), 0) => {
                eprintln!(
                    "{}: time column absent while the set carries times -> pending",
                    name
                );
                return None;
            }
            (Some(grid), _) => {
                if grid.len() != times.len() {
                    eprintln!(
                        "{}: series length differs from the set grid -> pending",
                        name
                    );
                    return None;
                }
                for (g, t) in grid.iter().zip(times.iter()) {
                    if ((g - t).abs() / g.abs().max(1.0)) > 1e-6 {
                        eprintln!("{}: time axis differs from the set grid -> pending", name);
                        return None;
                    }
                }
            }
        }
        sources.push(Source {
            name,
            ra_deg,
            dec_deg,
            dist_pc,
            flux,
        });
    }
    if sources.len() < 2 {
        eprintln!("< 2 usable sources -> no screening");
        return None;
    }
    let n0 = sources[0].flux.len();
    if sources.iter().any(|s| s.flux.len() != n0) {
        eprintln!("series lengths differ within the set -> pending, no screening");
        return None;
    }
    Some((sources, cadence))
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let set_path = args
        .iter()
        .position(|a| a == "--set")
        .and_then(|i| args.get(i + 1));
    let cadence_override: Option<f64> = args
        .iter()
        .position(|a| a == "--cadence-s")
        .and_then(|i| args.get(i + 1))
        .and_then(|v| v.parse().ok());
    let tol_frac: f64 = args
        .iter()
        .position(|a| a == "--tol-frac")
        .and_then(|i| args.get(i + 1))
        .and_then(|v| v.parse().ok())
        .unwrap_or(DEFAULT_TOL_FRAC);
    let n_surr: usize = args
        .iter()
        .position(|a| a == "--surrogates")
        .and_then(|i| args.get(i + 1))
        .and_then(|v| v.parse().ok())
        .unwrap_or(DEFAULT_SURROGATES);
    let alpha: f64 = args
        .iter()
        .position(|a| a == "--alpha")
        .and_then(|i| args.get(i + 1))
        .and_then(|v| v.parse().ok())
        .unwrap_or(DEFAULT_ALPHA);
    let max_pairs: Option<usize> = args
        .iter()
        .position(|a| a == "--max-pairs")
        .and_then(|i| args.get(i + 1))
        .and_then(|v| v.parse().ok());
    let seed: u64 = args
        .iter()
        .position(|a| a == "--seed")
        .and_then(|i| args.get(i + 1))
        .and_then(|v| v.parse().ok())
        .unwrap_or(SEED);

    let Some(set_path) = set_path else {
        eprintln!("--set <manifest> absent");
        std::process::exit(2);
    };

    let Some((mut sources, cadence_measured)) = load_set(set_path) else {
        return;
    };
    let cadence_s = cadence_override.or(cadence_measured);
    if cadence_s.is_none() {
        eprintln!(
            "no cadence: the set carries no uniform time axis and --cadence-s is absent -> pending, no screening"
        );
        return;
    }
    if let Some(cap) = max_pairs {
        sources.truncate(cap);
    }

    let report = screen_set(&sources, cadence_s, seed, tol_frac, n_surr, alpha);

    let cadence_word = match cadence_s {
        Some(c) => format!("{c:.3}"),
        None => "series".to_string(),
    };
    println!(
        "=== pair screen: {} sources, {} measured cells, cadence = {} s, separation window tolerance = {:.0} % ===",
        sources.len(),
        report.n_cells,
        cadence_word,
        tol_frac * 100.0
    );
    println!(
        "surrogate fabric-max null: {} surrogates per cell, familywise alpha = {:.3}, corrected z threshold = {:.3}",
        report.n_surr, report.alpha, report.z_threshold
    );
    println!();
    println!(
        "{:>20} {:>20} | {:>28} | {:>16} | {:>10} | {:>12} | {:>12}",
        "driver", "target", "verdict", "sep lt s", "window", "best lag", "best TE"
    );
    for d in &report.directed {
        let window_str = match d.window {
            Some((lo, hi)) => format!("{lo}..{hi}"),
            None => "-".to_string(),
        };
        let sep_str = match d.separation_lt_s {
            Some(s) => format!("{s:.3e}"),
            None => "unknown".to_string(),
        };
        let best_str = match d.best_lag {
            Some(l) => l.to_string(),
            None => "-".to_string(),
        };
        let te_str = match d.best_te {
            Some(t) => format!("{t:.4e}"),
            None => "-".to_string(),
        };
        println!(
            "{:>20} {:>20} | {:>28} | {:>16} | {:>10} | {:>12} | {:>12}",
            d.driver,
            d.target,
            d.verdict.label(),
            sep_str,
            window_str,
            best_str,
            te_str
        );
    }

    println!();
    let mut candidates: Vec<&DirectedVerdict> = report.directed.iter().collect();
    candidates.sort_by(|x, y| {
        y.best_te
            .unwrap_or(f64::NEG_INFINITY)
            .total_cmp(&x.best_te.unwrap_or(f64::NEG_INFINITY))
    });
    let candidates: Vec<&DirectedVerdict> = candidates
        .into_iter()
        .filter(|d| d.verdict == Verdict::EchoCandidate)
        .collect();
    if candidates.is_empty() {
        println!("no echo-candidate pair (measured, not fabricated)");
    } else {
        println!("=== echo-candidates: suspects, not a physics verdict ===");
        for d in candidates {
            let win = d.window.unwrap();
            let lag = match d.best_lag {
                Some(v) => v.to_string(),
                None => "absent".to_string(),
            };
            let te = match d.best_te {
                Some(v) => format!("{v:.4e}"),
                None => "absent".to_string(),
            };
            let thr = match d.best_threshold {
                Some(v) => format!("{v:.4e}"),
                None => "absent".to_string(),
            };
            let lt = match d.separation_lt_s {
                Some(v) => format!("{v:.3e}"),
                None => "absent".to_string(),
            };
            println!(
                "{} -> {}  lag {} in window {}..{}, TE {} > threshold {}; separation light time {} s",
                d.driver,
                d.target,
                lag,
                win.0,
                win.1,
                te,
                thr,
                lt
            );
        }
    }

    let mut counts = [0usize; 5];
    for d in &report.directed {
        let idx = match d.verdict {
            Verdict::EchoCandidate => 0,
            Verdict::SignificantNotPhysical => 1,
            Verdict::Field => 2,
            Verdict::Absent => 3,
            Verdict::Pending => 4,
        };
        counts[idx] += 1;
    }
    println!(
        "verdict counts: echo-candidate {}, significant-but-not-physical {}, field {}, absent {}, pending {}",
        counts[0], counts[1], counts[2], counts[3], counts[4]
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

    fn ar1(n: usize, phi: f64, rng: &mut u64) -> Vec<f32> {
        let mut v = Vec::with_capacity(n);
        let mut x = 0.0f64;
        for _ in 0..n {
            x = phi * x + next_rng(rng) * 2.0 - 1.0;
            v.push(x as f32);
        }
        v
    }

    const CADENCE_S: f64 = 86400.0;
    const N_SURR: usize = 24;
    const ALPHA: f64 = 0.05;

    fn ra_offset_deg(separation_s: f64, dist_pc: f64) -> f64 {
        let chord_m = separation_s * C_LIGHT;
        let d_m = dist_pc * PARSEC_M;
        (2.0 * (chord_m / (2.0 * d_m)).min(1.0).asin()).to_degrees()
    }

    fn src(name: &str, ra_deg: f64, flux: Vec<f32>) -> Source {
        Source {
            name: name.to_string(),
            ra_deg,
            dec_deg: 0.0,
            dist_pc: Some(1000.0),
            flux,
        }
    }

    fn verdict_of(report: &ScreenReport, driver: &str, target: &str) -> Verdict {
        report
            .directed
            .iter()
            .find(|d| d.driver == driver && d.target == target)
            .map(|d| d.verdict)
            .expect("directed pair absent from the report")
    }

    #[test]
    fn separation_light_seconds_matches_chord_over_c() {
        let a = src("a", 0.0, vec![0.0f32; 2]);
        let off = ra_offset_deg(123456.0, 1000.0);
        let b = src("b", off, vec![0.0f32; 2]);
        let sep = separation_light_seconds(&a, &b).unwrap();
        let expect = 123456.0;
        assert!(
            (sep - expect).abs() / expect < 1e-6,
            "separation light time {} should match the constructed chord {}",
            sep,
            expect
        );
    }

    #[test]
    fn delayed_prediction_pair_is_echo_candidate() {
        let mut rng = 0x9E37_79B9_7F4A_7C15u64;
        let n = 192usize;
        let delay = 12usize;
        let a = white(n, &mut rng);
        let mut b = vec![0.0f32; n];
        for t in 0..n {
            if t >= delay {
                b[t] = (0.9 * a[t - delay] as f64 + (next_rng(&mut rng) * 0.1 - 0.05)) as f32;
            } else {
                b[t] = (next_rng(&mut rng) * 0.1 - 0.05) as f32;
            }
        }
        let sep_s = delay as f64 * CADENCE_S;
        let a_src = src("driver", 0.0, a);
        let b_src = src("target", ra_offset_deg(sep_s, 1000.0), b);
        let report = screen_set(
            &[a_src, b_src],
            Some(CADENCE_S),
            42,
            DEFAULT_TOL_FRAC,
            N_SURR,
            ALPHA,
        );
        assert_eq!(
            verdict_of(&report, "driver", "target"),
            Verdict::EchoCandidate
        );
        let d = report
            .directed
            .iter()
            .find(|d| d.driver == "driver" && d.target == "target")
            .unwrap();
        let (lo, hi) = d.window.unwrap();
        let lag = d.best_lag.unwrap();
        assert!(
            lag >= lo && lag <= hi,
            "best lag {} must sit in the separation window {lo}..{hi}",
            lag
        );
    }

    #[test]
    fn near_simultaneous_correlation_with_wide_separation_is_not_physical() {
        let mut rng = 0x2722_0A95_517C_C1B7u64;
        let n = 192usize;
        let s = ar1(n + 1, 0.6, &mut rng);
        let mut a = Vec::with_capacity(n);
        let mut b = Vec::with_capacity(n);
        for t in 0..n {
            a.push((s[t + 1] as f64 + (next_rng(&mut rng) * 0.1 - 0.05)) as f32);
            b.push((s[t] as f64 + (next_rng(&mut rng) * 0.1 - 0.05)) as f32);
        }
        let sep_s = 30.0 * CADENCE_S;
        let a_src = src("lead", 0.0, a);
        let b_src = src("follow", ra_offset_deg(sep_s, 1000.0), b);
        let report = screen_set(
            &[a_src, b_src],
            Some(CADENCE_S),
            7,
            DEFAULT_TOL_FRAC,
            N_SURR,
            ALPHA,
        );
        assert_eq!(
            verdict_of(&report, "lead", "follow"),
            Verdict::SignificantNotPhysical
        );
    }

    #[test]
    fn independent_pair_is_field() {
        let mut rng = 0x517C_C1B7_2722_0A95u64;
        let a = ar1(192, 0.7, &mut rng);
        let b = ar1(192, 0.7, &mut rng);
        let a_src = src("a", 0.0, a);
        let b_src = src("b", ra_offset_deg(600000.0, 1000.0), b);
        let report = screen_set(
            &[a_src, b_src],
            Some(CADENCE_S),
            3,
            DEFAULT_TOL_FRAC,
            N_SURR,
            ALPHA,
        );
        assert_eq!(verdict_of(&report, "a", "b"), Verdict::Field);
        assert_eq!(verdict_of(&report, "b", "a"), Verdict::Field);
    }

    #[test]
    fn many_independent_pairs_yield_no_echo_candidate() {
        let mut rng = 0x9E37_79B9_7F4A_7C15u64 ^ 0xDEAD_BEEF;
        let n = 64usize;
        let mut sources = Vec::new();
        for k in 0..4usize {
            let a = white(n, &mut rng);
            let b = white(n, &mut rng);
            let sep_s = (5 + k) as f64 * CADENCE_S;
            sources.push(src(&format!("a{k}"), 0.0, a));
            sources.push(src(&format!("b{k}"), ra_offset_deg(sep_s, 1000.0), b));
        }
        let report = screen_set(
            &sources,
            Some(CADENCE_S),
            11,
            DEFAULT_TOL_FRAC,
            N_SURR,
            ALPHA,
        );
        assert!(
            report.n_cells >= 100,
            "the screen must count many cells, n_cells = {}",
            report.n_cells
        );
        let echo = report
            .directed
            .iter()
            .filter(|d| d.verdict == Verdict::EchoCandidate)
            .count();
        let not_physical = report
            .directed
            .iter()
            .filter(|d| d.verdict == Verdict::SignificantNotPhysical)
            .count();
        assert_eq!(echo, 0, "independent pairs must not yield echo candidates");
        assert_eq!(
            not_physical, 0,
            "independent pairs must not yield significance"
        );
        let field = report
            .directed
            .iter()
            .filter(|d| d.verdict == Verdict::Field)
            .count();
        assert_eq!(field, report.directed.len());
    }
}
