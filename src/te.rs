pub fn gaussian(u: f64, h: f64) -> f64 {
    (-(u * u) / (2.0 * h * h)).exp() / (h * (2.0 * std::f64::consts::PI).sqrt())
}

pub fn silverman(v: &[f32]) -> Option<f64> {
    let n = v.len() as f64;
    let mean = v.iter().map(|&x| x as f64).sum::<f64>() / n;
    let var = v
        .iter()
        .map(|&x| {
            let d = x as f64 - mean;
            d * d
        })
        .sum::<f64>()
        / n;
    if var <= 0.0 {
        return None;
    }
    Some(1.06 * var.sqrt() * n.powf(-0.2))
}

pub fn transfer_entropy(x: &[f32], y: &[f32]) -> Option<f64> {
    let n = x.len();
    if n < 8 {
        return None;
    }
    let hx = silverman(x)?;
    let hy = silverman(y)?;
    let m = n - 1;
    let mut te = 0.0;
    for t in 0..m {
        let xt = x[t] as f64;
        let xt1 = x[t + 1] as f64;
        let yt = y[t] as f64;
        let mut k3 = 0.0;
        for s in 0..m {
            k3 += gaussian(xt1 - x[s + 1] as f64, hx)
                * gaussian(xt - x[s] as f64, hx)
                * gaussian(yt - y[s] as f64, hy);
        }
        let p3 = k3 / m as f64;
        let mut k1 = 0.0;
        for s in 0..n {
            k1 += gaussian(xt - x[s] as f64, hx);
        }
        let p1 = k1 / n as f64;
        let mut k2xy = 0.0;
        for s in 0..n {
            k2xy += gaussian(xt - x[s] as f64, hx) * gaussian(yt - y[s] as f64, hy);
        }
        let p2xy = k2xy / n as f64;
        let mut k2x = 0.0;
        for s in 0..m {
            k2x += gaussian(xt1 - x[s + 1] as f64, hx) * gaussian(xt - x[s] as f64, hx);
        }
        let p2x = k2x / m as f64;
        te += ((p3 * p1) / (p2xy * p2x).max(1e-300)).ln();
    }
    Some(te / m as f64)
}

pub fn shuffle_series(v: &[f32], rng: &mut u64) -> Vec<f32> {
    let mut out = v.to_vec();
    for i in (1..out.len()).rev() {
        *rng = rng
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        let j = ((*rng >> 33) as usize) % (i + 1);
        out.swap(i, j);
    }
    out
}

pub fn surrogate_threshold(x: &[f32], y: &[f32], seed: u64) -> Option<f64> {
    let mut vals: Vec<f64> = Vec::with_capacity(10);
    let mut rng = seed.wrapping_add(0x9e3779b97f4a7c15);
    for _ in 0..10 {
        let ys = shuffle_series(y, &mut rng);
        if let Some(te) = transfer_entropy(x, &ys) {
            vals.push(te);
        }
    }
    if vals.len() < 2 {
        return None;
    }
    let n = vals.len() as f64;
    let mean = vals.iter().sum::<f64>() / n;
    let var = vals.iter().map(|&v| (v - mean) * (v - mean)).sum::<f64>() / n;
    Some(mean + 2.0 * var.sqrt())
}

pub fn transfer_entropy_lag(x: &[f32], y: &[f32], lag: usize) -> Option<f64> {
    if lag == 0 {
        return transfer_entropy(x, y);
    }
    let n = x.len();
    if n < 8 {
        return None;
    }
    let m = n - lag;
    if m < 8 {
        return None;
    }
    let hx = silverman(x)?;
    let hy = silverman(y)?;
    let mut te = 0.0;
    for t in 0..m {
        let xt = x[t] as f64;
        let xt1 = x[t + lag] as f64;
        let yt = y[t] as f64;
        let mut k3 = 0.0;
        for s in 0..m {
            k3 += gaussian(xt1 - x[s + lag] as f64, hx)
                * gaussian(xt - x[s] as f64, hx)
                * gaussian(yt - y[s] as f64, hy);
        }
        let p3 = k3 / m as f64;
        let mut k1 = 0.0;
        for s in 0..n {
            k1 += gaussian(xt - x[s] as f64, hx);
        }
        let p1 = k1 / n as f64;
        let mut k2xy = 0.0;
        for s in 0..n {
            k2xy += gaussian(xt - x[s] as f64, hx) * gaussian(yt - y[s] as f64, hy);
        }
        let p2xy = k2xy / n as f64;
        let mut k2x = 0.0;
        for s in 0..m {
            k2x += gaussian(xt1 - x[s + lag] as f64, hx) * gaussian(xt - x[s] as f64, hx);
        }
        let p2x = k2x / m as f64;
        te += ((p3 * p1) / (p2xy * p2x).max(1e-300)).ln();
    }
    Some(te / m as f64)
}

pub fn surrogate_threshold_lag(x: &[f32], y: &[f32], lag: usize, seed: u64) -> Option<f64> {
    surrogate_stats(x, y, lag, seed).map(|(_, _, threshold)| threshold)
}

pub fn surrogate_stats(x: &[f32], y: &[f32], lag: usize, seed: u64) -> Option<(f64, f64, f64)> {
    surrogate_stats_with(x, y, lag, seed, &mut |v, rng| shuffle_series(v, rng))
}

pub fn surrogate_stats_phase(
    x: &[f32],
    y: &[f32],
    lag: usize,
    seed: u64,
) -> Option<(f64, f64, f64)> {
    surrogate_stats_with(x, y, lag, seed, &mut |v, rng| {
        phase_randomized_surrogate(v, rng)
    })
}

pub fn surrogate_stats_block(
    x: &[f32],
    y: &[f32],
    lag: usize,
    block: usize,
    seed: u64,
) -> Option<(f64, f64, f64)> {
    surrogate_stats_with(x, y, lag, seed, &mut |v, rng| {
        block_bootstrap_surrogate(v, block, rng)
    })
}

fn surrogate_stats_with(
    x: &[f32],
    y: &[f32],
    lag: usize,
    seed: u64,
    surrogate: &mut dyn FnMut(&[f32], &mut u64) -> Vec<f32>,
) -> Option<(f64, f64, f64)> {
    let mut vals: Vec<f64> = Vec::with_capacity(10);
    let mut rng = seed.wrapping_add(0x9e3779b97f4a7c15);
    for _ in 0..10 {
        let ys = surrogate(y, &mut rng);
        if let Some(te) = transfer_entropy_lag(x, &ys, lag) {
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
    Some((mean, sd, mean + 2.0 * sd))
}

fn next_rng(rng: &mut u64) -> f64 {
    *rng = rng
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);
    ((*rng >> 33) as f64) / (u32::MAX as f64)
}

pub fn phase_randomized_surrogate(v: &[f32], rng: &mut u64) -> Vec<f32> {
    let n = v.len();
    if n < 2 {
        return v.to_vec();
    }
    let m = n.next_power_of_two();
    let mut re: Vec<f64> = vec![0.0; m];
    let mut im: Vec<f64> = vec![0.0; m];
    for (i, &x) in v.iter().enumerate() {
        re[i] = x as f64;
    }
    fft(&mut re, &mut im, false);
    for k in 1..m / 2 {
        let phi = next_rng(rng) * 2.0 * std::f64::consts::PI;
        let (s, c) = phi.sin_cos();
        let (ar, ai) = (re[k], im[k]);
        re[k] = ar * c - ai * s;
        im[k] = ar * s + ai * c;
        let j = m - k;
        re[j] = re[k];
        im[j] = -im[k];
    }
    fft(&mut re, &mut im, true);
    v.iter().enumerate().map(|(i, _)| re[i] as f32).collect()
}

pub fn block_bootstrap_surrogate(v: &[f32], block: usize, rng: &mut u64) -> Vec<f32> {
    let n = v.len();
    if n < 2 || block == 0 {
        return v.to_vec();
    }
    let block = block.min(n);
    let mut out = Vec::with_capacity(n);
    while out.len() < n {
        let start = (next_rng(rng) * n as f64) as usize % n;
        for i in 0..block {
            out.push(v[(start + i) % n]);
        }
    }
    out.truncate(n);
    out
}

fn fft(re: &mut [f64], im: &mut [f64], inverse: bool) {
    let n = re.len();
    let mut j = 0usize;
    for i in 1..n {
        let mut bit = n >> 1;
        while j & bit != 0 {
            j ^= bit;
            bit >>= 1;
        }
        j ^= bit;
        if i < j {
            re.swap(i, j);
            im.swap(i, j);
        }
    }
    let mut len = 2usize;
    while len <= n {
        let ang = if inverse {
            2.0 * std::f64::consts::PI / len as f64
        } else {
            -2.0 * std::f64::consts::PI / len as f64
        };
        let (s, c) = ang.sin_cos();
        let mut i = 0usize;
        while i < n {
            let mut w_re = 1.0f64;
            let mut w_im = 0.0f64;
            for k in 0..len / 2 {
                let u_re = re[i + k];
                let u_im = im[i + k];
                let v_re = re[i + k + len / 2] * w_re - im[i + k + len / 2] * w_im;
                let v_im = re[i + k + len / 2] * w_im + im[i + k + len / 2] * w_re;
                re[i + k] = u_re + v_re;
                im[i + k] = u_im + v_im;
                re[i + k + len / 2] = u_re - v_re;
                im[i + k + len / 2] = u_im - v_im;
                let w2_re = w_re * c - w_im * s;
                let w2_im = w_re * s + w_im * c;
                w_re = w2_re;
                w_im = w2_im;
            }
            i += len;
        }
        len <<= 1;
    }
    if inverse {
        let scale = 1.0 / n as f64;
        for (a, b) in re.iter_mut().zip(im.iter_mut()) {
            *a *= scale;
            *b *= scale;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transfer_entropy_causal_positive() {
        let n = 200;
        let mut x = vec![0f32; n];
        let mut y = vec![0f32; n];
        for t in 0..n {
            y[t] = (t as f32 * 0.7).sin();
        }
        for t in 0..n - 1 {
            x[t + 1] = 0.5 * x[t] + 0.6 * y[t];
        }
        let te = transfer_entropy(&x, &y).unwrap();
        assert!(te > 0.05, "causal TE should be positive, got {}", te);
    }

    #[test]
    fn transfer_entropy_independent_near_zero() {
        let n = 200;
        let mut x = vec![0f32; n];
        let mut y = vec![0f32; n];
        for t in 0..n {
            y[t] = (t as f32 * 0.7).sin();
            x[t] = ((t as u64).wrapping_mul(2654435761) >> 24) as f32 / 255.0;
        }
        let te = transfer_entropy(&x, &y).unwrap();
        assert!(
            te.abs() < 0.05,
            "independent TE should be near zero, got {}",
            te
        );
    }

    #[test]
    fn surrogate_threshold_below_causal_te() {
        let n = 200;
        let mut x = vec![0f32; n];
        let mut y = vec![0f32; n];
        for t in 0..n {
            y[t] = (t as f32 * 0.7).sin();
        }
        for t in 0..n - 1 {
            x[t + 1] = 0.5 * x[t] + 0.6 * y[t];
        }
        let te = transfer_entropy(&x, &y).unwrap();
        let thr = surrogate_threshold(&x, &y, 42).unwrap();
        assert!(
            thr < te,
            "surrogate threshold {} should be below causal TE {}",
            thr,
            te
        );
    }

    #[test]
    fn lag_zero_matches_canonical() {
        let n = 200;
        let mut x = vec![0f32; n];
        let mut y = vec![0f32; n];
        for t in 0..n {
            y[t] = (t as f32 * 0.7).sin();
        }
        for t in 0..n - 1 {
            x[t + 1] = 0.5 * x[t] + 0.6 * y[t];
        }
        let te0 = transfer_entropy(&x, &y).unwrap();
        let tel = transfer_entropy_lag(&x, &y, 0).unwrap();
        assert!((te0 - tel).abs() < 1e-9 * te0.abs());
    }

    #[test]
    fn surrogate_stats_carry_the_threshold() {
        let n = 200;
        let mut x = vec![0f32; n];
        let mut y = vec![0f32; n];
        for t in 0..n {
            y[t] = (t as f32 * 0.7).sin();
        }
        for t in 0..n - 1 {
            x[t + 1] = 0.5 * x[t] + 0.6 * y[t];
        }
        let (mean, sd, threshold) = surrogate_stats(&x, &y, 0, 42).unwrap();
        let direct = surrogate_threshold_lag(&x, &y, 0, 42).unwrap();
        assert!((mean + 2.0 * sd - threshold).abs() < 1e-12);
        assert!((threshold - direct).abs() < 1e-12);
        assert!(sd > 0.0);
    }

    fn autocorr(v: &[f32], lag: usize) -> f64 {
        let n = v.len();
        let mean = v.iter().map(|&x| x as f64).sum::<f64>() / n as f64;
        let var = v
            .iter()
            .map(|&x| {
                let d = x as f64 - mean;
                d * d
            })
            .sum::<f64>()
            / n as f64;
        let mut c = 0.0;
        for i in 0..n - lag {
            c += (v[i] as f64 - mean) * (v[i + lag] as f64 - mean);
        }
        c / ((n - lag) as f64) / var.max(1e-30)
    }

    #[test]
    fn phase_surrogate_preserves_autocorrelation() {
        let n = 512;
        let x: Vec<f32> = (0..n)
            .map(|t| (t as f32 * 0.13).sin() + 0.5 * (t as f32 * 0.037).sin())
            .collect();
        let mut rng = 42u64;
        let s = phase_randomized_surrogate(&x, &mut rng);
        assert_eq!(s.len(), n);
        for lag in 1..=8 {
            let a = autocorr(&x, lag);
            let b = autocorr(&s, lag);
            assert!((a - b).abs() < 0.3, "lag {}: autocorr {} vs {}", lag, a, b);
        }
    }

    #[test]
    fn phase_surrogate_preserves_variance() {
        let n = 512;
        let x: Vec<f32> = (0..n).map(|t| (t as f32 * 0.13).sin()).collect();
        let mut rng = 9u64;
        let s = phase_randomized_surrogate(&x, &mut rng);
        let var = |v: &[f32]| {
            let mean = v.iter().sum::<f32>() / v.len() as f32;
            v.iter().map(|&a| (a - mean) * (a - mean)).sum::<f32>() / v.len() as f32
        };
        assert!((var(&x) - var(&s)).abs() < 0.05 * var(&x));
    }

    #[test]
    fn block_bootstrap_preserves_short_lag_autocorrelation() {
        let n = 600;
        let mut x = vec![0f32; n];
        for t in 1..n {
            x[t] = 0.8 * x[t - 1] + 0.2 * (t as f32 * 0.9).sin();
        }
        let mut rng = 5u64;
        let s = block_bootstrap_surrogate(&x, 40, &mut rng);
        assert_eq!(s.len(), n);
        for lag in 1..=10 {
            let a = autocorr(&x, lag);
            let b = autocorr(&s, lag);
            assert!((a - b).abs() < 0.35, "lag {}: {} vs {}", lag, a, b);
        }
    }

    #[test]
    fn fft_roundtrip_is_identity() {
        let mut re: Vec<f64> = vec![0.0; 64];
        let mut im: Vec<f64> = vec![0.0; 64];
        for t in 0..64 {
            re[t] = (t as f64 * 0.3).sin() + 2.0 * (t as f64 * 0.05).cos();
        }
        let orig = re.clone();
        fft(&mut re, &mut im, false);
        fft(&mut re, &mut im, true);
        for (a, b) in re.iter().zip(orig.iter()) {
            assert!((a - b).abs() < 1e-9);
        }
    }
}
