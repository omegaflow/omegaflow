use omegaflow::te::{phase_randomized_surrogate, transfer_entropy_lag};

const TRANSIENT: usize = 1000;
const N_SURR: usize = 10;
const SEED: u64 = 0x9E37_79B9_7F4A_7C15;

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

fn main() {
    println!(
        "=== TE directionality vs (c, n): does reverse TE carry a true arrow, and where does it cross? ==="
    );
    println!(
        "TE(X->Y) forward, TE(Y->X) reverse; surrogate thr = phase-randomized ({}), mean+2sigma.",
        N_SURR
    );
    println!("X drives Y via x_n in the y map (coupling c). Reverse arrow = a real reverse TE above its own null.");
    println!();
    println!(
        "{:>6} {:>7} | {:>12} {:>12} {:>6} | {:>11} {:>11} | {:>6} {:>6}",
        "c", "n", "TE(X->Y)", "TE(Y->X)", "ratio", "thr_fwd", "thr_rev", "fwd", "rev"
    );

    let cs: [f64; 9] = [0.0, 0.05, 0.10, 0.15, 0.20, 0.25, 0.30, 0.40, 0.50];
    let ns: [usize; 3] = [1000, 3000, 10000];

    let mut cell: u64 = 0;
    for &c in &cs {
        for &n in &ns {
            let (xc, yc) = coupled_henon(n, TRANSIENT, c);

            let mut fam_pool: Vec<f64> = Vec::new();
            let mut surr_f = Vec::with_capacity(N_SURR);
            let mut surr_r = Vec::with_capacity(N_SURR);

            let te_xy = transfer_entropy_lag(&yc, &xc, 1);
            let te_yx = transfer_entropy_lag(&xc, &yc, 1);

            let mut rng = SEED ^ cell.wrapping_mul(0x9E37_79B9_7F4A_7C15);
            cell += 1;
            if let (Some(f), Some(r)) = (te_xy, te_yx) {
                for _ in 0..N_SURR {
                    let sf = phase_randomized_surrogate(&xc, &mut rng);
                    if let Some(t) = transfer_entropy_lag(&yc, &sf, 1) {
                        surr_f.push(t);
                        fam_pool.push(t);
                    }
                    let sr = phase_randomized_surrogate(&yc, &mut rng);
                    if let Some(t) = transfer_entropy_lag(&xc, &sr, 1) {
                        surr_r.push(t);
                        fam_pool.push(t);
                    }
                }
                let thr_f = mean_plus_2sigma(&surr_f);
                let thr_r = mean_plus_2sigma(&surr_r);
                let fam = fam_pool.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
                println!(
                    "{:>6.2} {:>7} | {:>12.4e} {:>12.4e} {:>6.2} | {:>11.4e} {:>11.4e} | {:>6} {:>6}",
                    c,
                    n,
                    f,
                    r,
                    f / r.max(1e-300),
                    thr_f,
                    thr_r,
                    if f > fam { "arrow" } else { "-" },
                    if r > fam { "arrow" } else { "-" }
                );
            } else {
                println!("{:>6.2} {:>7} | NaN", c, n);
            }
        }
    }
    println!();
    println!("rev crosses its own thr/fam -> look at whether that crossing c moves with n (bias) or not (asymmetry).");
    println!("c = 0 anchors the uncoupled control: a reverse arrow there is a false-arrow bias that n scaling must not carry.");
}
