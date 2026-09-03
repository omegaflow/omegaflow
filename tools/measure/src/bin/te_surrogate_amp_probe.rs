use omegaflow::te::phase_randomized_surrogate;

fn next_rng(rng: &mut u64) -> f64 {
    *rng = rng
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);
    ((*rng >> 33) as f64) / (u32::MAX as f64)
}

fn main() {
    let n = 256usize;
    let sine: Vec<f32> = (0..n)
        .map(|i| (2.0 * std::f64::consts::PI * 4.0 * i as f64 / n as f64).sin() as f32)
        .collect();
    let mut rng = 0x9E37_79B9_7F4A_7C15u64;
    let surr = phase_randomized_surrogate(&sine, &mut rng);
    let amp_in = sine.iter().map(|&x| (x * x) as f64).sum::<f64>() / n as f64;
    let amp_out = surr.iter().map(|&x| (x * x) as f64).sum::<f64>() / n as f64;
    println!(
        "4 Hz sine: true power {:.4e}, surrogate {:.4e}, ratio {:.3}",
        amp_in,
        amp_out,
        amp_out / amp_in
    );

    println!();
    println!("next_rng distribution (expectation [0,1), suspicion [0,0.5)):");
    let mut rng = 0x9E37_79B9_7F4A_7C15u64;
    let mut mx = 0.0f64;
    let mut mn = 1.0f64;
    let mut mean = 0.0;
    let n_s = 100_000usize;
    for _ in 0..n_s {
        let v = next_rng(&mut rng);
        if v > mx {
            mx = v;
        }
        if v < mn {
            mn = v;
        }
        mean += v;
    }
    println!(
        "min {:.4}, max {:.4}, mean {:.4} (true [0,1): max ~1, mean ~0.5)",
        mn,
        mx,
        mean / n_s as f64
    );
}
