use omegaflow::te::topological_te_phase;

const N_TRIALS: usize = 60;
const SEED: u64 = 0x9E37_79B9_7F4A_7C15;

fn next_rng(rng: &mut u64) -> f64 {
    *rng = rng
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);
    ((*rng >> 33) as f64) / (u32::MAX as f64)
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
    let mut fp = 0usize;
    let mut fn_miss = 0usize;
    let mut fn_measurable = 0usize;

    for t in 0..N_TRIALS {
        let seed = SEED ^ (t as u64).wrapping_mul(0x517C_C1B7_2722_0A95);
        let a = ar1(300, 0.7, &mut rng);
        let b = ar1(300, 0.7, &mut rng);
        let ab = topological_te_phase(&a, &b, 3, 3, seed);
        let ba = topological_te_phase(&b, &a, 3, 3, seed);
        if ab.as_ref().is_some_and(|v| v.te > v.threshold) {
            fp += 1;
        }
        if ba.as_ref().is_some_and(|v| v.te > v.threshold) {
            fp += 1;
        }
    }
    let fp_rate = fp as f64 / (N_TRIALS * 2) as f64;

    for t in 0..N_TRIALS {
        let seed = SEED ^ (t as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15);
        let a = ar1(400, 0.9, &mut rng);
        let b: Vec<f32> = (0..a.len())
            .map(|i| {
                if i == 0 {
                    next_rng(&mut rng) as f32
                } else {
                    a[i - 1] + (next_rng(&mut rng) * 0.2 - 0.1) as f32
                }
            })
            .collect();
        let ab = topological_te_phase(&a, &b, 3, 3, seed);
        if let Some(v) = ab {
            fn_measurable += 1;
            if !(v.te > v.threshold) {
                fn_miss += 1;
            }
        }
    }
    let fn_rate = fn_miss as f64 / fn_measurable.max(1) as f64;

    println!(
        "false-positive rate (a ⊥ b): {}/{} direction tests = {:.1} % (2σ expectation ≈ 2,3 %)",
        fp,
        N_TRIALS * 2,
        fp_rate * 100.0
    );
    println!(
        "false-negative rate (b follows a, direction a→b): {} missed of {} measurable couplings = {:.1} %",
        fn_miss,
        fn_measurable,
        fn_rate * 100.0
    );
    println!();
    let fp_sweep = fp_rate * 96.0;
    println!(
        "sweep placement: at {} direction tests the machine alone yields {:.1} arrows through its false-positive rate — the 24 arrows of the sweep are a finding only above this number.",
        96, fp_sweep
    );
}
