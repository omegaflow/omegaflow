use omegaflow::te::{coherent_phase_surrogates, phase_randomized_surrogate};

const SEED: u64 = 0x9E37_79B9_7F4A_7C15;
const CONFIRM_SEED: u64 = 0x2545_F491_4F6C_DD1D;
const NULL_SURROGATES: usize = 200;

fn next_rng(rng: &mut u64) -> f64 {
    *rng = rng
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);
    ((*rng >> 33) as f64) / ((u32::MAX >> 1) as f64)
}

fn ar1_sine(n: usize, phi: f64, period: f64, phase: f64, noise: f64, rng: &mut u64) -> Vec<f32> {
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

fn strong_pair_fixture(
    n: usize,
    delay: usize,
    driver_noise: f64,
    rng: &mut u64,
) -> (Vec<f32>, Vec<f32>) {
    let a = ar1_sine(n, 0.6, 36.0, 0.0, driver_noise, rng);
    let mut b = vec![0.0f32; n];
    let mut x = 0.0f64;
    for t in 0..n {
        x = 0.5 * x
            + if t >= delay {
                0.9 * a[t - delay] as f64
            } else {
                0.0
            }
            + (next_rng(rng) * 0.02 - 0.01);
        b[t] = x as f32;
    }
    (a, b)
}

fn phase_null_pair(series: &[&[f32]], seed: u64, s: usize, t: usize) -> Vec<Vec<f32>> {
    series
        .iter()
        .enumerate()
        .map(|(k, v)| {
            let mut rng = seed
                ^ (s as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15)
                ^ ((t as u64) << 20)
                ^ ((k as u64) << 8)
                ^ 0xA5A5_5A5A;
            phase_randomized_surrogate(v, &mut rng)
        })
        .collect()
}

fn coherent_null_pair(series: &[&[f32]], seed: u64, s: usize, t: usize) -> Vec<Vec<f32>> {
    let mut rng = seed
        ^ (s as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15)
        ^ ((t as u64) << 20)
        ^ 0xA5A5_5A5A;
    coherent_phase_surrogates(series, &mut rng)
}

fn n_point_spectrum(v: &[f32]) -> Vec<f64> {
    let n = v.len();
    let mut tw_re = Vec::with_capacity(n);
    let mut tw_im = Vec::with_capacity(n);
    for t in 0..n {
        let ang = -2.0 * std::f64::consts::PI * t as f64 / n as f64;
        let (s, c) = ang.sin_cos();
        tw_re.push(c);
        tw_im.push(s);
    }
    let half = n / 2 + 1;
    let mut out = Vec::with_capacity(half);
    for k in 0..half {
        let mut re = 0.0f64;
        let mut im = 0.0f64;
        let mut idx = 0usize;
        for &x in v {
            re += x as f64 * tw_re[idx];
            im += x as f64 * tw_im[idx];
            idx += k;
            if idx >= n {
                idx -= n;
            }
        }
        out.push(re * re + im * im);
    }
    out
}

fn peak_line(label: &str, obs: &[f64], n: usize) {
    let (k, p) = obs
        .iter()
        .enumerate()
        .max_by(|a, b| a.1.total_cmp(b.1))
        .expect("the spectrum carries a peak");
    println!(
        "blind-band probe: {label} observed peak: bin {k} = {:.6} cycles/sample (power {:.3e})",
        k as f64 / n as f64,
        p
    );
}

fn coverage_report(label: &str, obs: &[f64], envelope: &[(f64, f64)], n: usize) {
    let mut bands: Vec<(usize, usize, usize, usize, f64)> = Vec::new();
    let mut failed = 0usize;
    for (k, &p) in obs.iter().enumerate() {
        let above = p > envelope[k].1;
        let below = p < envelope[k].0;
        if !above && !below {
            continue;
        }
        failed += 1;
        let gap = if above {
            if envelope[k].1 > 0.0 {
                p / envelope[k].1
            } else {
                f64::INFINITY
            }
        } else if p > 0.0 {
            envelope[k].0 / p
        } else {
            f64::INFINITY
        };
        match bands.last_mut() {
            Some((_, last, a, b, max_gap)) if k == *last + 1 => {
                *last = k;
                if above {
                    *a += 1;
                } else {
                    *b += 1;
                }
                if gap > *max_gap {
                    *max_gap = gap;
                }
            }
            _ => bands.push((k, k, usize::from(above), usize::from(below), gap)),
        }
    }
    if bands.is_empty() {
        println!(
            "blind-band probe: {label}: no bin of {} lies outside the surrogate envelope",
            obs.len()
        );
        return;
    }
    println!(
        "blind-band probe: {label}: {} of {} bins outside the surrogate envelope in {} band(s)",
        failed,
        obs.len(),
        bands.len()
    );
    for (i, &(lo, hi, above, below, gap)) in bands.iter().enumerate() {
        println!(
            "blind-band probe: {label} band {}: bins {}..={} = {:.6}..{:.6} cycles/sample ({} above max, {} below min, max gap {:.3}x)",
            i,
            lo,
            hi,
            lo as f64 / n as f64,
            hi as f64 / n as f64,
            above,
            below,
            gap
        );
    }
}

#[ignore = "print-only blind-band probe for the phase-null seam — runs in hyperscanning-te.yml"]
#[test]
fn phase_null_blind_band_stochastic_pair() {
    let mut rng = SEED ^ 0x0C0F_FEE1;
    let (driver, target) = strong_pair_fixture(800, 8, 0.05, &mut rng);
    let series: [&[f32]; 2] = [driver.as_slice(), target.as_slice()];
    let n = driver.len();
    let obs_driver = n_point_spectrum(&driver);
    let obs_target = n_point_spectrum(&target);
    println!(
        "blind-band probe: n={} bins={} null surrogates={} seed={:#x}",
        n,
        obs_driver.len(),
        NULL_SURROGATES,
        CONFIRM_SEED
    );
    peak_line("driver A", &obs_driver, n);
    peak_line("target B", &obs_target, n);
    for (label, coherent) in [("phase", false), ("coherent-phase", true)] {
        let mut env_driver: Vec<(f64, f64)> = vec![(f64::INFINITY, 0.0); obs_driver.len()];
        let mut env_target: Vec<(f64, f64)> = vec![(f64::INFINITY, 0.0); obs_target.len()];
        for s in 0..NULL_SURROGATES {
            let pair = if coherent {
                coherent_null_pair(&series, CONFIRM_SEED, s, 0)
            } else {
                phase_null_pair(&series, CONFIRM_SEED, s, 0)
            };
            let spec_driver = n_point_spectrum(&pair[0]);
            let spec_target = n_point_spectrum(&pair[1]);
            for (e, p) in env_driver.iter_mut().zip(spec_driver.iter()) {
                e.0 = e.0.min(*p);
                e.1 = e.1.max(*p);
            }
            for (e, p) in env_target.iter_mut().zip(spec_target.iter()) {
                e.0 = e.0.min(*p);
                e.1 = e.1.max(*p);
            }
        }
        coverage_report(&format!("{label} null driver A"), &obs_driver, &env_driver, n);
        coverage_report(&format!("{label} null target B"), &obs_target, &env_target, n);
    }
}
