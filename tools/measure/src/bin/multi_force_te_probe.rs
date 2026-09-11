use std::env;
use std::process::exit;

use omegaflow::force::force_name_of;
use omegaflow::te::{
    benjamini_hochberg, conditional_te_stats_lagged_n, hilbert_instantaneous_phase,
    transfer_entropy_conditional_binned_n, TeNull,
};

const SEED: u64 = 0x9E37_79B9_7F4A_7C15;
const N_FORCE: usize = 9;
const BINS: usize = 3;
const PLANTED: [(usize, usize); 3] = [(8, 5), (0, 6), (1, 3)];

fn next_rng(rng: &mut u64) -> f64 {
    *rng = rng
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);
    ((*rng >> 33) as f64) / ((u32::MAX >> 1) as f64)
}

fn force_series(n: usize, seed: u64) -> Vec<Vec<f32>> {
    let mut rng = seed;
    let omega: [f64; N_FORCE] = [0.21, 0.17, 0.13, 0.29, 0.11, 0.23, 0.19, 0.31, 0.15];
    let mut phase: Vec<Vec<f64>> = vec![vec![0.0f64; n]; N_FORCE];
    for (i, series) in phase.iter_mut().enumerate() {
        let mut p = 0.0f64;
        for slot in series.iter_mut() {
            p += omega[i] + 0.05 * (next_rng(&mut rng) * 2.0 - 1.0);
            *slot = p;
        }
    }
    for (src, dst) in PLANTED {
        for t in 1..n {
            phase[dst][t] += 2.0 * phase[src][t - 1].sin();
        }
    }
    phase
        .iter()
        .map(|p| p.iter().map(|&x| x.sin() as f32).collect())
        .collect()
}

fn normal_cdf(x: f64) -> f64 {
    let t = 1.0 / (1.0 + 0.2316419 * x.abs());
    let d = 0.3989422804014327 * (-x * x * 0.5).exp();
    let poly = t
        * (0.319381530
            + t * (-0.356563782 + t * (1.781477937 + t * (-1.821255978 + t * 1.330274429))));
    if x >= 0.0 {
        1.0 - d * poly
    } else {
        d * poly
    }
}

fn conditional_link(
    target: &[f32],
    driver: &[f32],
    conds: &[&[f32]],
    lag: usize,
    max_lag: usize,
    bins: usize,
    seed: u64,
    n_surr: usize,
) -> Option<(f64, f64, f64)> {
    let te = transfer_entropy_conditional_binned_n(target, driver, conds, lag, bins)?;
    let (mean, sd, threshold) = conditional_te_stats_lagged_n(
        target,
        driver,
        conds,
        lag,
        max_lag,
        bins,
        seed,
        n_surr,
        TeNull::Block,
    )?;
    let p_value = if sd > 0.0 {
        1.0 - normal_cdf((te - mean) / sd)
    } else if te > mean {
        0.0
    } else {
        1.0
    };
    Some((te, threshold, p_value))
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let n_surr: usize = match args
        .iter()
        .position(|a| a == "--surrogat")
        .and_then(|i| args.get(i + 1))
    {
        Some(v) => match v.parse().ok() {
            Some(x) if x >= 2 => x,
            _ => 50,
        },
        None => 50,
    };
    let alpha: f64 = match args
        .iter()
        .position(|a| a == "--alpha")
        .and_then(|i| args.get(i + 1))
    {
        Some(v) => v
            .parse()
            .ok()
            .filter(|&x| x > 0.0 && x <= 1.0)
            .unwrap_or(0.05),
        None => 0.05,
    };
    let lags: Vec<usize> = match args
        .iter()
        .position(|a| a == "--lags")
        .and_then(|i| args.get(i + 1))
    {
        Some(v) => {
            let parsed: Vec<usize> = v
                .split(',')
                .filter_map(|s| s.parse().ok())
                .filter(|&l| l >= 1)
                .collect();
            if parsed.is_empty() {
                vec![1, 2, 3]
            } else {
                parsed
            }
        }
        None => vec![1, 2, 3],
    };

    let n = 512usize;
    let raw = force_series(n, SEED);
    let phases: Option<Vec<Vec<f32>>> =
        raw.iter().map(|v| hilbert_instantaneous_phase(v)).collect();
    let Some(phases) = phases else {
        eprintln!("phase-space unresolved: a force series carries no finite instantaneous phase");
        exit(2);
    };
    let max_lag = *lags.iter().max().expect("lag sweep non-empty");

    println!("=== conditional multi-force TE over the Hilbert phase space ===");
    let names: Vec<&str> = (0..N_FORCE)
        .filter_map(|i| force_name_of(i as u8))
        .collect();
    println!("forces = {:?}", names);
    println!(
        "n = {} | lags = {:?} | bins = {} | surrogates = {} | alpha = {}",
        n, lags, BINS, n_surr, alpha
    );
    println!(
        "every ordered pair (driver -> target) is conditioned on the other {} forces; threshold = mean+2sigma, p = Gaussian tail over {} block-bootstrap surrogates.",
        N_FORCE - 2,
        n_surr
    );
    println!();
    println!(
        "{:>6} {:>14} {:>4} | {:>10} {:>10} {:>10} | {}",
        "driver", "target", "lag", "te", "threshold", "p_value", "fdr"
    );

    let mut rows: Vec<(usize, usize, usize, f64, f64, f64)> = Vec::new();
    for drv in 0..N_FORCE {
        for tgt in 0..N_FORCE {
            if drv == tgt {
                continue;
            }
            let conds: Vec<&[f32]> = (0..N_FORCE)
                .filter(|&k| k != drv && k != tgt)
                .map(|k| phases[k].as_slice())
                .collect();
            for &lag in &lags {
                let seed_t = SEED
                    ^ (drv as u64).wrapping_mul(0x9E37_79B9)
                    ^ (tgt as u64).wrapping_mul(0x85EB_CA6B)
                    ^ (lag as u64).wrapping_mul(0xC2B2_AE3D);
                if let Some((te, thr, p)) = conditional_link(
                    &phases[tgt],
                    &phases[drv],
                    &conds,
                    lag,
                    max_lag,
                    BINS,
                    seed_t,
                    n_surr,
                ) {
                    rows.push((drv, tgt, lag, te, thr, p));
                }
            }
        }
    }

    let p_vals: Vec<f64> = rows.iter().map(|r| r.5).collect();
    let cutoff = benjamini_hochberg(&p_vals, alpha);
    let mut by_p = rows.clone();
    by_p.sort_by(|a, b| a.5.total_cmp(&b.5));
    let mut pass_count = 0usize;
    for (drv, tgt, lag, te, thr, p) in &by_p {
        let pass = cutoff.map(|c| *p <= c).unwrap_or(false);
        if pass {
            pass_count += 1;
        }
        println!(
            "{:>6} {:>14} {:>4} | {:>10.4e} {:>10.4e} {:>10.4e} | {}",
            force_name_of(*drv as u8).unwrap(),
            force_name_of(*tgt as u8).unwrap(),
            lag,
            te,
            thr,
            p,
            if pass { "pass" } else { "-" }
        );
    }
    println!();
    println!(
        "benjamini-hochberg cutoff = {:?} over {} tests",
        cutoff,
        rows.len()
    );
    println!("{} links pass FDR at alpha {}", pass_count, alpha);
    println!();
    println!("planted arrows and their recovered cells:");
    for (drv, tgt) in PLANTED {
        let cell = by_p
            .iter()
            .filter(|(d, t, _, _, _, _)| *d == drv && *t == tgt)
            .min_by(|a, b| a.5.total_cmp(&b.5));
        match cell {
            Some((_, _, lag, te, thr, p)) => {
                let pass = cutoff.map(|c| *p <= c).unwrap_or(false);
                println!(
                    "  {} -> {} : best lag {} | te {:.4e} | threshold {:.4e} | p {:.4e} | {}",
                    force_name_of(drv as u8).unwrap(),
                    force_name_of(tgt as u8).unwrap(),
                    lag,
                    te,
                    thr,
                    p,
                    if pass { "recovered" } else { "not recovered" }
                );
            }
            None => println!(
                "  {} -> {} : no resolved cell",
                force_name_of(drv as u8).unwrap(),
                force_name_of(tgt as u8).unwrap()
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn force_series_carries_nine_finite_series() {
        let s = force_series(128, SEED);
        assert_eq!(s.len(), N_FORCE);
        for v in &s {
            assert_eq!(v.len(), 128);
            assert!(v.iter().all(|&x| x.is_finite()));
        }
    }

    #[test]
    fn phase_lives_in_principal_branch() {
        let s = force_series(128, SEED);
        for v in &s {
            let ph = hilbert_instantaneous_phase(v).expect("phase resolves");
            assert_eq!(ph.len(), v.len());
            for &p in &ph {
                assert!(p.is_finite());
                assert!((-std::f32::consts::PI..=std::f32::consts::PI).contains(&p));
            }
        }
    }

    #[test]
    fn conditional_link_keeps_true_arrow_beyond_confound() {
        let n = 400;
        let mut rng = 0x1234_5678_9ABC_DEF0u64;
        let mut z = vec![0.0f32; n];
        let mut x = vec![0.0f32; n];
        let mut y = vec![0.0f32; n];
        for t in 0..n {
            let zp = if t == 0 { 0.0 } else { z[t - 1] as f64 };
            z[t] = (0.5 * zp + (next_rng(&mut rng) * 2.0 - 1.0)) as f32;
        }
        for t in 0..n {
            let xp = if t == 0 { 0.0 } else { x[t - 1] as f64 };
            x[t] = (0.6 * xp + 0.4 * (next_rng(&mut rng) * 2.0 - 1.0) + 0.6 * z[t] as f64) as f32;
        }
        for t in 0..n {
            let yp = if t == 0 { 0.0 } else { y[t - 1] as f64 };
            let xl = if t == 0 { 0.0 } else { x[t - 1] as f64 };
            y[t] =
                (0.4 * yp + 0.6 * xl + 0.5 * z[t] as f64 + 0.3 * (next_rng(&mut rng) * 2.0 - 1.0))
                    as f32;
        }
        let (te, thr, p) =
            conditional_link(&y, &x, &[&z], 1, 1, 3, SEED, 100).expect("conditional link resolves");
        assert!(
            te > thr,
            "true arrow must clear its own null: te {te} thr {thr}"
        );
        assert!(p < 0.1, "true arrow p-value must be small: {p}");
    }

    #[test]
    fn bh_cutoff_matches_rank_threshold() {
        let p = [0.01, 0.04, 0.03, 0.20, 0.50];
        let cutoff = benjamini_hochberg(&p, 0.05).expect("cutoff resolves");
        assert!((cutoff - 0.01).abs() < 1e-12);
    }

    #[test]
    fn phase_space_link_recovers_phase_coupling() {
        let n = 512;
        let mut rng = 0x0F1E_2D3C_4B5A_6978u64;
        let mut src_phase = vec![0.0f64; n];
        let mut tgt_phase = vec![0.0f64; n];
        let mut p = 0.0f64;
        for t in 0..n {
            p += 0.30 + 0.05 * (next_rng(&mut rng) * 2.0 - 1.0);
            src_phase[t] = p;
        }
        p = 0.0;
        for t in 0..n {
            let coupling = if t == 0 {
                0.0
            } else {
                2.0 * src_phase[t - 1].sin()
            };
            p += 0.21 + coupling + 0.1 * (next_rng(&mut rng) * 2.0 - 1.0);
            tgt_phase[t] = p;
        }
        let src: Vec<f32> = src_phase.iter().map(|&x| x.sin() as f32).collect();
        let tgt: Vec<f32> = tgt_phase.iter().map(|&x| x.sin() as f32).collect();
        let src_ph = hilbert_instantaneous_phase(&src).expect("source phase resolves");
        let tgt_ph = hilbert_instantaneous_phase(&tgt).expect("target phase resolves");
        let (te, thr, p_value) =
            conditional_link(&tgt_ph, &src_ph, &[], 1, 1, 3, 0x0F1E_2D3C_4B5A_6978, 100)
                .expect("phase link resolves");
        assert!(
            te > thr,
            "phase coupling must clear its own null: te {te} thr {thr}"
        );
        assert!(
            p_value < 0.05,
            "phase coupling p-value must be small: {p_value}"
        );
    }
}
