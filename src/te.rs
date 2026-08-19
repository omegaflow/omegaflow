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
    let mut vals: Vec<f64> = Vec::with_capacity(10);
    let mut rng = seed.wrapping_add(0x9e3779b97f4a7c15);
    for _ in 0..10 {
        let ys = shuffle_series(y, &mut rng);
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
}
