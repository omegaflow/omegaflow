use omegaflow::te::{phase_randomized_surrogate, topological_te_estimate, transfer_entropy_lag};

const N: usize = 10000;
const TRANSIENT: usize = 1000;
const COUPLING: f64 = 0.2;
const SEED: u64 = 0x9E37_79B9_7F4A_7C15;
const N_SURR: usize = 10;
const DIM: usize = 3;

fn coupled_henon(n: usize, transient: usize, c: f64) -> (Vec<f32>, Vec<f32>) {
    let total = n + transient;
    let mut xs = vec![0.0f64; total];
    let mut ys = vec![0.0f64; total];
    xs[0] = 0.1;
    xs[1] = 0.0;
    ys[0] = 0.2;
    ys[1] = 0.1;
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

fn mean_plus_2sigma(vals: &[f64]) -> f64 {
    let m = vals.iter().sum::<f64>() / vals.len() as f64;
    let var = vals.iter().map(|v| (v - m) * (v - m)).sum::<f64>() / vals.len() as f64;
    m + 2.0 * var.sqrt()
}

fn embedded_threshold(target: &[f32], source: &[f32], fam_pool: &mut Vec<f64>) -> Option<f64> {
    let mut rng = SEED.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut surr = Vec::with_capacity(N_SURR);
    for _ in 0..N_SURR {
        let ys = phase_randomized_surrogate(source, &mut rng);
        if let Some(t) = topological_te_estimate(target, &ys, DIM) {
            surr.push(t.te);
            fam_pool.push(t.te);
        }
    }
    if surr.is_empty() {
        None
    } else {
        Some(mean_plus_2sigma(&surr))
    }
}

fn embedded_direction(
    target: &[f32],
    source: &[f32],
    label: &str,
    fam_pool: &mut Vec<f64>,
) -> Option<(f64, f64)> {
    let te = topological_te_estimate(target, source, DIM)?.te;
    let thr = embedded_threshold(target, source, fam_pool)?;
    println!(
        "  {:<12} | TE {:>10.4e} | thr {:>10.4e} | {} ({} surrogate, dim {})",
        label,
        te,
        thr,
        if te > thr { "arrow" } else { "still" },
        N_SURR,
        DIM
    );
    Some((te, thr))
}

fn scalar_direction(target: &[f32], source: &[f32], label: &str) -> Option<(f64, f64)> {
    let te = transfer_entropy_lag(target, source, 1)?;
    let mut rng = SEED.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut surr = Vec::with_capacity(N_SURR);
    for _ in 0..N_SURR {
        let ys = phase_randomized_surrogate(source, &mut rng);
        if let Some(t) = transfer_entropy_lag(target, &ys, 1) {
            surr.push(t);
        }
    }
    if surr.is_empty() {
        println!("  {label:<12} | pending (no surrogate)");
        return None;
    }
    let thr = mean_plus_2sigma(&surr);
    println!(
        "  {:<12} | TE {:>10.4e} | thr {:>10.4e} | {}",
        label,
        te,
        thr,
        if te > thr { "arrow" } else { "still" }
    );
    Some((te, thr))
}

fn main() {
    println!("=== TE ground truth — Schreiber 2000 (unidirectionally coupled Hénon maps) ===");
    println!(
        "System: x_{{n+1}} = 1.4 − x_n² + 0.3 x_{{n−1}};  y_{{n+1}} = 1.4 − (c·x_n·y_n + (1−c)·y_n²) + 0.3 y_{{n−1}}"
    );
    println!(
        "n = {} (transient {} discarded), lag = 1, threshold = phase-randomized surrogates (10, mean + 2σ) + family threshold fam over the whole round",
        N, TRANSIENT
    );

    let (xc, yc) = coupled_henon(N, TRANSIENT, COUPLING);
    println!();
    println!(
        "Production estimator — Takens-embedded KSG (dim {DIM}), the arm the flux runs (omega.rs:489):"
    );
    println!("Coupling c = {:.2} (known direction: X → Y):", COUPLING);
    let mut fam_pool: Vec<f64> = Vec::new();
    let xy = embedded_direction(&yc, &xc, "TE(X→Y)", &mut fam_pool);
    let yx = embedded_direction(&xc, &yc, "TE(Y→X)", &mut fam_pool);

    let (xi, yi) = coupled_henon(N, TRANSIENT, 0.0);
    println!();
    println!("Control c = 0.00 (independent):");
    let cxy = embedded_direction(&yi, &xi, "TE(X→Y)", &mut fam_pool);
    let cyx = embedded_direction(&xi, &yi, "TE(Y→X)", &mut fam_pool);

    println!();
    match (xy, yx, cxy, cyx) {
        (Some((te_xy, thr_xy)), Some((te_yx, thr_yx)), Some((te_xy0, _)), Some((te_yx0, _))) => {
            let fam = fam_pool.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            println!(
                "fam = {:.4e} — the strongest surrogate TE of the whole round (multiple-comparison correction over all four directions × cases).",
                fam
            );
            println!("Asymmetry: TE(X→Y) / TE(Y→X) = {:.2}", te_xy / te_yx);
            let pass = te_xy > fam && te_yx <= fam && te_xy0 <= fam && te_yx0 <= fam;
            println!(
                "Verdict: {}",
                if pass {
                    "PASS — the production estimator reconstructs the known direction: only TE(X→Y) survives the family threshold, the opposite direction and the control stay still."
                } else {
                    "NOT PASS — the direction is not cleanly reconstructed; the values above name which series deviates."
                }
            );
            println!(
                "TE(X→Y) = {:.4e} (threshold {:.4e}, fam {:.4e}); TE(Y→X) = {:.4e} (threshold {:.4e}); c=0: TE(X→Y) {:.4e}, TE(Y→X) {:.4e}.",
                te_xy, thr_xy, fam, te_yx, thr_yx, te_xy0, te_yx0
            );
        }
        _ => println!(
            "Verdict: pending — a direction carries no MI-τ or no measurable surrogate (0 honored); absent values name no direction."
        ),
    }

    println!();
    println!(
        "Scalar Silverman-KDE reference (transfer_entropy_lag, lag 1), the canonical arm the paper names at docs/paper/gic-causal-driver.md:655-660:"
    );
    let _ = scalar_direction(&yc, &xc, "TE(X→Y)");
    let _ = scalar_direction(&xc, &yc, "TE(Y→X)");
    println!(
        "  the scalar arm does not null the reverse channel at strong coupling (finite-sample bias, te_bias_n_probe) — a riss against the production arm, carried, not smoothed."
    );
}
