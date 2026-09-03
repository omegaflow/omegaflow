use omegaflow::te::{surrogate_stats_phase, topological_te_phase, transfer_entropy_lag};

const SEED: u64 = 0x9E37_79B9_7F4A_7C15;

fn next_rng(rng: &mut u64) -> f64 {
    *rng = rng
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);
    ((*rng >> 33) as f64) / ((u32::MAX >> 1) as f64)
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

fn main() {
    let mut rng = SEED;
    println!(
        "{:>6} {:>6} {:>6} {:>6} | {:>9} {:>9} {:>6} | {:>9} {:>9} {:>6}",
        "n", "phi", "c", "noise", "te_scal", "thr_scal", "scal", "te_topo", "thr_topo", "topo"
    );
    for &(n, phi, c, noise) in &[
        (300usize, 0.5f64, 0.9f64, 0.1f64),
        (300, 0.3, 0.9, 0.1),
        (300, 0.5, 0.9, 0.3),
        (500, 0.5, 0.9, 0.1),
        (500, 0.5, 0.7, 0.1),
        (300, 0.0, 0.9, 0.1),
        (500, 0.0, 0.9, 0.1),
        (500, 0.0, 0.7, 0.1),
    ] {
        let mut found_scal = 0;
        let mut found_topo = 0;
        let mut meas = 0usize;
        let mut s_te = 0.0;
        let mut t_te = 0.0;
        let mut s_thr = 0.0;
        let mut t_thr = 0.0;
        for t in 0..10 {
            let seed = SEED ^ (t as u64).wrapping_mul(0x517C_C1B7_2722_0A95);
            let a = if phi == 0.0 {
                (0..n)
                    .map(|_| (next_rng(&mut rng) * 2.0 - 1.0) as f32)
                    .collect()
            } else {
                ar1(n, phi, &mut rng)
            };
            let b: Vec<f32> = (0..a.len())
                .map(|i| {
                    if i == 0 {
                        next_rng(&mut rng) as f32
                    } else {
                        (c * a[i - 1] as f64 + (next_rng(&mut rng) * noise * 2.0 - noise)) as f32
                    }
                })
                .collect();
            if let (Some(te), Some((_, _, thr))) = (
                transfer_entropy_lag(&b, &a, 0),
                surrogate_stats_phase(&b, &a, 0, seed),
            ) {
                meas += 1;
                s_te += te;
                s_thr += thr;
                if te > thr {
                    found_scal += 1;
                }
            }
            if let Some(v) = topological_te_phase(&b, &a, 3, 3, seed) {
                t_te += v.te;
                t_thr += v.threshold;
                if v.te > v.threshold {
                    found_topo += 1;
                }
            }
        }
        let m = meas.max(1) as f64;
        println!(
            "{:>6} {:>6.2} {:>6.2} {:>6.2} | {:>9.4e} {:>9.4e} {:>5}/{} | {:>9.4e} {:>9.4e} {:>5}/{}",
            n,
            phi,
            c,
            noise,
            s_te / m,
            s_thr / m,
            found_scal,
            meas,
            t_te / 10.0,
            t_thr / 10.0,
            found_topo,
            10
        );
    }
}
