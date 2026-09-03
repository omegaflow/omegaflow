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

pub fn transfer_entropy_conditional(x: &[f32], y: &[f32], c: &[f32], lag: usize) -> Option<f64> {
    let n = x.len();
    if n < 8 || y.len() < n || c.len() < n {
        return None;
    }
    if lag == 0 {
        return transfer_entropy_conditional(x, y, c, 1);
    }
    let m = n - lag;
    if m < 8 {
        return None;
    }
    let hx = silverman(x)?;
    let hy = silverman(y)?;
    let hz = silverman(c)?;
    let mut te = 0.0;
    for t in 0..m {
        let xt = x[t] as f64;
        let xk = x[t + lag] as f64;
        let yt = y[t] as f64;
        let zt = c[t] as f64;
        let mut k4 = 0.0;
        for s in 0..m {
            k4 += gaussian(xk - x[s + lag] as f64, hx)
                * gaussian(xt - x[s] as f64, hx)
                * gaussian(yt - y[s] as f64, hy)
                * gaussian(zt - c[s] as f64, hz);
        }
        let p4 = k4 / m as f64;
        let mut k2 = 0.0;
        for s in 0..n {
            k2 += gaussian(xt - x[s] as f64, hx) * gaussian(zt - c[s] as f64, hz);
        }
        let p2 = k2 / n as f64;
        let mut k3a = 0.0;
        for s in 0..n {
            k3a += gaussian(xt - x[s] as f64, hx)
                * gaussian(yt - y[s] as f64, hy)
                * gaussian(zt - c[s] as f64, hz);
        }
        let p3a = k3a / n as f64;
        let mut k3b = 0.0;
        for s in 0..m {
            k3b += gaussian(xk - x[s + lag] as f64, hx)
                * gaussian(xt - x[s] as f64, hx)
                * gaussian(zt - c[s] as f64, hz);
        }
        let p3b = k3b / m as f64;
        te += ((p4 * p2) / (p3a * p3b).max(1e-300)).ln();
    }
    Some(te / m as f64)
}

fn residual_surrogate_conditional(y: &[f32], c: &[f32], rng: &mut u64) -> Vec<f32> {
    let n = y.len();
    let yf: Vec<f64> = y.iter().map(|&v| v as f64).collect();
    let cf: Vec<f64> = c.iter().map(|&v| v as f64).collect();
    let mut sx = 0.0;
    let mut sy = 0.0;
    for i in 0..n {
        sx += cf[i];
        sy += yf[i];
    }
    let xm = sx / n as f64;
    let ym = sy / n as f64;
    let mut num = 0.0;
    let mut den = 0.0;
    for i in 0..n {
        num += (cf[i] - xm) * (yf[i] - ym);
        den += (cf[i] - xm) * (cf[i] - xm);
    }
    let beta1 = if den > 1e-12 { num / den } else { 0.0 };
    let beta0 = ym - beta1 * xm;
    let mut resid: Vec<f64> = (0..n).map(|i| yf[i] - (beta0 + beta1 * cf[i])).collect();

    for i in (1..n).rev() {
        *rng = rng
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        let j = ((*rng >> 33) as usize) % (i + 1);
        resid.swap(i, j);
    }
    (0..n)
        .map(|i| (beta0 + beta1 * cf[i] + resid[i]) as f32)
        .collect()
}

pub fn conditional_te_stats(
    x: &[f32],
    y: &[f32],
    c: &[f32],
    lag: usize,
    seed: u64,
    n_surr: usize,
) -> Option<(f64, f64, f64)> {
    let mut vals: Vec<f64> = Vec::with_capacity(n_surr);
    let mut rng = seed.wrapping_add(0x9e3779b97f4a7c15);
    for _ in 0..n_surr {
        let ys = residual_surrogate_conditional(y, c, &mut rng);
        if let Some(te) = transfer_entropy_conditional(x, &ys, c, lag) {
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
    ((*rng >> 33) as f64) / ((u32::MAX >> 1) as f64)
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

pub fn hilbert_instantaneous_phase(v: &[f32]) -> Option<Vec<f32>> {
    let n = v.len();
    if n < 2 || v.iter().any(|x| !x.is_finite()) {
        return None;
    }
    let m = n.next_power_of_two();
    let mut re: Vec<f64> = vec![0.0; m];
    let mut im: Vec<f64> = vec![0.0; m];
    for (i, &x) in v.iter().enumerate() {
        re[i] = x as f64;
    }
    fft(&mut re, &mut im, false);
    for k in 1..m / 2 {
        re[k] *= 2.0;
        im[k] *= 2.0;
    }
    for k in m / 2 + 1..m {
        re[k] = 0.0;
        im[k] = 0.0;
    }
    fft(&mut re, &mut im, true);
    Some((0..n).map(|i| im[i].atan2(re[i]) as f32).collect())
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

const Φ: f64 = 1.618033988749895;

fn silverman_f64(v: &[f64]) -> Option<f64> {
    let n = v.len() as f64;
    if n < 2.0 {
        return None;
    }
    let mean = v.iter().sum::<f64>() / n;
    let var = v
        .iter()
        .map(|&x| {
            let d = x - mean;
            d * d
        })
        .sum::<f64>()
        / n;
    if var <= 0.0 {
        return None;
    }
    Some(1.06 * var.sqrt() * n.powf(-0.2))
}

pub fn find_mi_lag(series: &[f64]) -> Option<usize> {
    let n = series.len();
    let max_lag = (n as f64 / Φ) as usize;
    if max_lag < 3 || series.iter().any(|v| !v.is_finite()) {
        return None;
    }
    let eps = f64::EPSILON;
    let mut mi_prev2 = 0.0;
    let mut mi_prev = 0.0;
    for lag in 1..=max_lag {
        let w = n - lag;
        let mut mn = f64::INFINITY;
        let mut mx = f64::NEG_INFINITY;
        for i in 0..w {
            let v = series[i];
            if v < mn {
                mn = v;
            }
            if v > mx {
                mx = v;
            }
        }
        let range = mx - mn;
        if range <= 0.0 {
            return None;
        }
        let mid = mn + range * 0.5;
        let mut h00 = 0usize;
        let mut h01 = 0usize;
        let mut h10 = 0usize;
        let mut h11 = 0usize;
        for i in 0..w {
            let b1 = series[i] > mid;
            let b2 = series[i + lag] > mid;
            match (b1, b2) {
                (false, false) => h00 += 1,
                (false, true) => h01 += 1,
                (true, false) => h10 += 1,
                (true, true) => h11 += 1,
            }
        }
        let total = w as f64;
        let p0 = (h00 + h01) as f64 / total;
        let p1 = (h10 + h11) as f64 / total;
        let q0 = (h00 + h10) as f64 / total;
        let q1 = (h01 + h11) as f64 / total;
        let mut mi = 0.0;
        if h00 > 0 {
            let p = h00 as f64 / total;
            mi += p * (p / (p0 * q0 + eps) + eps).log2();
        }
        if h01 > 0 {
            let p = h01 as f64 / total;
            mi += p * (p / (p0 * q1 + eps) + eps).log2();
        }
        if h10 > 0 {
            let p = h10 as f64 / total;
            mi += p * (p / (p1 * q0 + eps) + eps).log2();
        }
        if h11 > 0 {
            let p = h11 as f64 / total;
            mi += p * (p / (p1 * q1 + eps) + eps).log2();
        }
        if lag >= 3 && mi_prev2 > mi_prev && mi_prev <= mi {
            return Some(lag - 1);
        }
        mi_prev2 = mi_prev;
        mi_prev = mi;
    }
    None
}

pub fn embed_series(series: &[f64], tau: usize, dim: usize) -> Vec<Vec<f64>> {
    if tau == 0 || dim == 0 {
        return Vec::new();
    }
    let span = (dim - 1).saturating_mul(tau);
    let m = match series.len().checked_sub(span) {
        Some(v) if v > 0 => v,
        _ => return Vec::new(),
    };
    let mut out = Vec::with_capacity(m);
    for t in 0..m {
        let mut state = Vec::with_capacity(dim);
        for k in 0..dim {
            state.push(series[t + k * tau]);
        }
        out.push(state);
    }
    out
}

fn state_distance(a: &[f64], b: &[f64]) -> f64 {
    a.iter()
        .zip(b.iter())
        .map(|(&u, &v)| {
            let d = u - v;
            d * d
        })
        .sum::<f64>()
        .sqrt()
}

fn embedded_silverman(emb: &[Vec<f64>]) -> Option<f64> {
    let n = emb.len() as f64;
    let dim = match emb.first() {
        Some(s) => s.len(),
        None => return None,
    };
    if n < 2.0 || dim == 0 {
        return None;
    }
    let mut mean = vec![0.0; dim];
    for state in emb.iter() {
        for (k, &v) in state.iter().enumerate() {
            mean[k] += v;
        }
    }
    for k in 0..dim {
        mean[k] /= n;
    }
    let mut var = 0.0;
    for state in emb.iter() {
        let mut d2 = 0.0;
        for (k, &v) in state.iter().enumerate() {
            let d = v - mean[k];
            d2 += d * d;
        }
        var += d2;
    }
    var /= n;
    if var <= 0.0 {
        return None;
    }
    Some(1.06 * var.sqrt() * n.powf(-0.2))
}

pub fn transfer_entropy_embedded(
    x: &[f64],
    emb_x: &[Vec<f64>],
    emb_y: &[Vec<f64>],
    tau_x: usize,
    tau_y: usize,
) -> Option<f64> {
    let n = x.len();
    if n < 8 || tau_x == 0 || tau_y == 0 || x.iter().any(|v| !v.is_finite()) {
        return None;
    }
    let dim = match emb_x.first() {
        Some(s) => s.len(),
        None => return None,
    };
    if dim < 2 {
        return None;
    }
    if emb_y.first().map_or(true, |s| s.len() != dim) {
        return None;
    }
    if emb_x.iter().flatten().any(|v| !v.is_finite())
        || emb_y.iter().flatten().any(|v| !v.is_finite())
    {
        return None;
    }
    let back_x = (dim - 1) * tau_x;
    let back_y = (dim - 1) * tau_y;
    if emb_x.len() != n - back_x || emb_y.len() != n - back_y {
        return None;
    }
    let t_low = back_x.max(back_y);
    let t_high = match n.checked_sub(tau_x + 1) {
        Some(v) => v,
        None => return None,
    };
    if t_low > t_high {
        return None;
    }
    let m = t_high - t_low + 1;
    if m < 8 {
        return None;
    }
    let h_f = silverman_f64(&x[t_low + tau_x..])?;
    let h_x = embedded_silverman(emb_x)?;
    let h_y = embedded_silverman(emb_y)?;
    let n_x = emb_x.len();
    let n_xy = n - t_low;
    let state_x = |t: usize| &emb_x[t - back_x];
    let state_y = |t: usize| &emb_y[t - back_y];
    let mut te = 0.0;
    for t in t_low..=t_high {
        let fut = x[t + tau_x];
        let sx = state_x(t);
        let sy = state_y(t);
        let mut k3 = 0.0;
        for s in t_low..=t_high {
            k3 += gaussian(fut - x[s + tau_x], h_f)
                * gaussian(state_distance(sx, state_x(s)), h_x)
                * gaussian(state_distance(sy, state_y(s)), h_y);
        }
        let p3 = k3 / m as f64;
        let mut k1 = 0.0;
        for s in back_x..n {
            k1 += gaussian(state_distance(sx, state_x(s)), h_x);
        }
        let p1 = k1 / n_x as f64;
        let mut k2xy = 0.0;
        for s in t_low..n {
            k2xy += gaussian(state_distance(sx, state_x(s)), h_x)
                * gaussian(state_distance(sy, state_y(s)), h_y);
        }
        let p2xy = k2xy / n_xy as f64;
        let mut k2x = 0.0;
        for s in t_low..=t_high {
            k2x +=
                gaussian(fut - x[s + tau_x], h_f) * gaussian(state_distance(sx, state_x(s)), h_x);
        }
        let p2x = k2x / m as f64;
        te += ((p3 * p1) / (p2xy * p2x).max(1e-300)).ln();
    }
    Some(te / m as f64)
}

fn permutation_entropy_counts(
    series: &[f64],
    order: usize,
    delay: usize,
) -> Option<(f64, usize, usize)> {
    if order < 2 || delay == 0 || series.iter().any(|v| !v.is_finite()) {
        return None;
    }
    let span = (order - 1).saturating_mul(delay);
    let total_windows = match series.len().checked_sub(span) {
        Some(v) if v > 0 => v,
        _ => return None,
    };
    let mut motifs: Vec<usize> = Vec::with_capacity(total_windows);
    for i in 0..total_windows {
        let mut tied = false;
        for a in 0..order {
            for b in (a + 1)..order {
                if series[i + a * delay] == series[i + b * delay] {
                    tied = true;
                }
            }
        }
        if tied {
            continue;
        }
        let mut idx: Vec<usize> = (0..order).collect();
        idx.sort_unstable_by(|&a, &b| {
            series[i + a * delay]
                .partial_cmp(&series[i + b * delay])
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        let mut rank = vec![0usize; order];
        for (pos, &k) in idx.iter().enumerate() {
            rank[k] = pos;
        }
        let mut key = 0usize;
        for a in 0..order {
            let mut l = 0usize;
            for b in (a + 1)..order {
                if rank[b] < rank[a] {
                    l += 1;
                }
            }
            key = key * (order - a) + l;
        }
        motifs.push(key);
    }
    if motifs.is_empty() {
        return None;
    }
    motifs.sort_unstable();
    let used = motifs.len();
    let mut entropy = 0.0;
    let mut run_start = 0usize;
    while run_start < used {
        let mut run_end = run_start + 1;
        while run_end < used && motifs[run_end] == motifs[run_start] {
            run_end += 1;
        }
        let p = (run_end - run_start) as f64 / used as f64;
        entropy -= p * p.log2();
        run_start = run_end;
    }
    let log2_factorial: f64 =
        (2..=order).map(|k| (k as f64).ln()).sum::<f64>() / std::f64::consts::LN_2;
    Some((entropy / log2_factorial, used, total_windows))
}

pub fn permutation_entropy(series: &[f64], order: usize, delay: usize) -> Option<f64> {
    permutation_entropy_counts(series, order, delay).map(|(pe, _, _)| pe)
}

pub struct TopologicalVerdict {
    pub tau_x: usize,
    pub tau_y: usize,
    pub te: f64,
    pub threshold: f64,
    pub surrogate_mean: f64,
    pub surrogate_sd: f64,
    pub surrogates_used: usize,
    pub pe_x: Option<f64>,
    pub pe_y: Option<f64>,
    pub pe_motifs_x: usize,
    pub pe_motifs_y: usize,
}

fn topological_te_with(
    x: &[f32],
    y: &[f32],
    dim: usize,
    order: usize,
    seed: u64,
    surrogate: &mut dyn FnMut(&[f32], &mut u64) -> Vec<f32>,
) -> Option<TopologicalVerdict> {
    let n = x.len();
    if n < 8 || y.len() != n || dim < 2 {
        return None;
    }
    let xf: Vec<f64> = x.iter().map(|&v| v as f64).collect();
    let yf: Vec<f64> = y.iter().map(|&v| v as f64).collect();
    if xf.iter().chain(yf.iter()).any(|v| !v.is_finite()) {
        return None;
    }
    let tau_x = find_mi_lag(&xf)?;
    let tau_y = find_mi_lag(&yf)?;
    let emb_x = embed_series(&xf, tau_x, dim);
    let emb_y = embed_series(&yf, tau_y, dim);
    if emb_x.is_empty() || emb_y.is_empty() {
        return None;
    }
    let te = transfer_entropy_embedded(&xf, &emb_x, &emb_y, tau_x, tau_y)?;
    let mut vals: Vec<f64> = Vec::with_capacity(10);
    let mut rng = seed.wrapping_add(0x9e3779b97f4a7c15);
    for _ in 0..10 {
        let ys = surrogate(y, &mut rng);
        if ys.len() != n {
            continue;
        }
        let ysf: Vec<f64> = ys.iter().map(|&v| v as f64).collect();
        if ysf.iter().any(|v| !v.is_finite()) {
            continue;
        }
        let tau_s = match find_mi_lag(&ysf) {
            Some(v) => v,
            None => continue,
        };
        let emb_s = embed_series(&ysf, tau_s, dim);
        if emb_s.is_empty() {
            continue;
        }
        if let Some(te_s) = transfer_entropy_embedded(&xf, &emb_x, &emb_s, tau_x, tau_s) {
            vals.push(te_s);
        }
    }
    if vals.len() < 2 {
        return None;
    }
    let mean = vals.iter().sum::<f64>() / vals.len() as f64;
    let var = vals
        .iter()
        .map(|&v| {
            let d = v - mean;
            d * d
        })
        .sum::<f64>()
        / vals.len() as f64;
    let sd = var.sqrt();
    let (pe_x, motifs_x) = match permutation_entropy_counts(&xf, order, 1) {
        Some((pe, used, _)) => (Some(pe), used),
        None => (None, 0),
    };
    let (pe_y, motifs_y) = match permutation_entropy_counts(&yf, order, 1) {
        Some((pe, used, _)) => (Some(pe), used),
        None => (None, 0),
    };
    Some(TopologicalVerdict {
        tau_x,
        tau_y,
        te,
        threshold: mean + 2.0 * sd,
        surrogate_mean: mean,
        surrogate_sd: sd,
        surrogates_used: vals.len(),
        pe_x,
        pe_y,
        pe_motifs_x: motifs_x,
        pe_motifs_y: motifs_y,
    })
}

pub fn topological_te_phase(
    x: &[f32],
    y: &[f32],
    dim: usize,
    order: usize,
    seed: u64,
) -> Option<TopologicalVerdict> {
    topological_te_with(x, y, dim, order, seed, &mut |v, rng| {
        phase_randomized_surrogate(v, rng)
    })
}

pub fn topological_te_block(
    x: &[f32],
    y: &[f32],
    dim: usize,
    order: usize,
    block: usize,
    seed: u64,
) -> Option<TopologicalVerdict> {
    topological_te_with(x, y, dim, order, seed, &mut move |v, rng| {
        block_bootstrap_surrogate(v, block, rng)
    })
}

pub fn topological_te_instantaneous_phase(
    x: &[f32],
    y: &[f32],
    dim: usize,
    order: usize,
    seed: u64,
) -> Option<TopologicalVerdict> {
    let n = x.len();
    if n < 8 || y.len() != n || dim < 2 {
        return None;
    }
    let px = hilbert_instantaneous_phase(x)?;
    let py = hilbert_instantaneous_phase(y)?;
    let xf: Vec<f64> = px.iter().map(|&v| v as f64).collect();
    let yf: Vec<f64> = py.iter().map(|&v| v as f64).collect();
    if xf.iter().chain(yf.iter()).any(|v| !v.is_finite()) {
        return None;
    }
    let tau_x = find_mi_lag(&xf)?;
    let tau_y = find_mi_lag(&yf)?;
    let emb_x = embed_series(&xf, tau_x, dim);
    let emb_y = embed_series(&yf, tau_y, dim);
    if emb_x.is_empty() || emb_y.is_empty() {
        return None;
    }
    let te = transfer_entropy_embedded(&xf, &emb_x, &emb_y, tau_x, tau_y)?;
    let mut vals: Vec<f64> = Vec::with_capacity(10);
    let mut rng = seed.wrapping_add(0x9e3779b97f4a7c15);
    for _ in 0..10 {
        let ys_amp = phase_randomized_surrogate(y, &mut rng);
        let Some(ys) = hilbert_instantaneous_phase(&ys_amp) else {
            continue;
        };
        if ys.len() != n {
            continue;
        }
        let ysf: Vec<f64> = ys.iter().map(|&v| v as f64).collect();
        if ysf.iter().any(|v| !v.is_finite()) {
            continue;
        }
        let tau_s = match find_mi_lag(&ysf) {
            Some(v) => v,
            None => continue,
        };
        let emb_s = embed_series(&ysf, tau_s, dim);
        if emb_s.is_empty() {
            continue;
        }
        if let Some(te_s) = transfer_entropy_embedded(&xf, &emb_x, &emb_s, tau_x, tau_s) {
            vals.push(te_s);
        }
    }
    if vals.len() < 2 {
        return None;
    }
    let mean = vals.iter().sum::<f64>() / vals.len() as f64;
    let var = vals
        .iter()
        .map(|&v| {
            let d = v - mean;
            d * d
        })
        .sum::<f64>()
        / vals.len() as f64;
    let sd = var.sqrt();
    let (pe_x, motifs_x) = match permutation_entropy_counts(&xf, order, 1) {
        Some((pe, used, _)) => (Some(pe), used),
        None => (None, 0),
    };
    let (pe_y, motifs_y) = match permutation_entropy_counts(&yf, order, 1) {
        Some((pe, used, _)) => (Some(pe), used),
        None => (None, 0),
    };
    Some(TopologicalVerdict {
        tau_x,
        tau_y,
        te,
        threshold: mean + 2.0 * sd,
        surrogate_mean: mean,
        surrogate_sd: sd,
        surrogates_used: vals.len(),
        pe_x,
        pe_y,
        pe_motifs_x: motifs_x,
        pe_motifs_y: motifs_y,
    })
}

pub fn topological_verdict_from_gpu(verdict: &[f32; 72]) -> Option<TopologicalVerdict> {
    let valid_real = verdict[10] == 1.0;
    if !valid_real {
        return None;
    }
    let tau_x = verdict[0] as usize;
    let tau_y = verdict[6] as usize;
    let te = verdict[7] as f64;
    let mut vals: Vec<f64> = Vec::with_capacity(10);
    for s in 2..12 {
        if verdict[s * 6 + 4] == 1.0 {
            vals.push(verdict[s * 6 + 1] as f64);
        }
    }
    if vals.len() < 2 {
        return None;
    }
    let n = vals.len() as f64;
    let mean = vals.iter().sum::<f64>() / n;
    let var = vals.iter().map(|&v| (v - mean) * (v - mean)).sum::<f64>() / n;
    let sd = var.sqrt();
    let pe_x = if verdict[5] == 1.0 {
        Some(verdict[2] as f64)
    } else {
        None
    };
    let pe_y = if verdict[11] == 1.0 {
        Some(verdict[8] as f64)
    } else {
        None
    };
    Some(TopologicalVerdict {
        tau_x,
        tau_y,
        te,
        threshold: mean + 2.0 * sd,
        surrogate_mean: mean,
        surrogate_sd: sd,
        surrogates_used: vals.len(),
        pe_x,
        pe_y,
        pe_motifs_x: verdict[3] as usize,
        pe_motifs_y: verdict[9] as usize,
    })
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

    #[test]
    fn mi_lag_periodic_finds_quarter_period() {
        let n = 512;
        let x: Vec<f64> = (0..n).map(|t| (t as f64 * 0.3).sin()).collect();
        let tau = find_mi_lag(&x).unwrap();
        assert!(
            (3..=8).contains(&tau),
            "quarter-period tau expected, got {}",
            tau
        );
    }

    #[test]
    fn mi_lag_constant_is_none() {
        let x = vec![2.5f64; 128];
        assert!(find_mi_lag(&x).is_none());
    }

    #[test]
    fn mi_lag_short_series_is_none() {
        assert!(find_mi_lag(&[1.0, 2.0, 3.0]).is_none());
    }

    #[test]
    fn embed_series_forward_states() {
        let x: Vec<f64> = (0..10).map(|t| t as f64).collect();
        let emb = embed_series(&x, 2, 3);
        assert_eq!(emb.len(), 6);
        assert_eq!(emb[0], vec![0.0, 2.0, 4.0]);
        assert_eq!(emb[5], vec![5.0, 7.0, 9.0]);
        assert!(embed_series(&x, 0, 3).is_empty());
        assert!(embed_series(&x, 2, 10).is_empty());
    }

    #[test]
    fn embedded_te_causal_positive() {
        let n = 400;
        let mut xf = vec![0f64; n];
        let mut yf = vec![0f64; n];
        for t in 0..n {
            yf[t] = (t as f64 * 0.5).sin();
        }
        for t in 0..n - 1 {
            xf[t + 1] = 0.5 * xf[t] + 0.6 * yf[t];
        }
        let tau_x = find_mi_lag(&xf).unwrap();
        let tau_y = find_mi_lag(&yf).unwrap();
        let emb_x = embed_series(&xf, tau_x, 3);
        let emb_y = embed_series(&yf, tau_y, 3);
        let te = transfer_entropy_embedded(&xf, &emb_x, &emb_y, tau_x, tau_y).unwrap();
        assert!(
            te > 0.0,
            "embedded causal TE should be positive, got {}",
            te
        );
    }

    #[test]
    fn embedded_te_independent_near_zero() {
        let n = 400;
        let xf: Vec<f64> = (0..n).map(|t| (t as f64 * 0.5).sin()).collect();
        let yf: Vec<f64> = (0..n).map(|t| (t as f64 * 0.7 + 1.0).sin()).collect();
        let tau_x = find_mi_lag(&xf).unwrap();
        let tau_y = find_mi_lag(&yf).unwrap();
        let emb_x = embed_series(&xf, tau_x, 3);
        let emb_y = embed_series(&yf, tau_y, 3);
        let te = transfer_entropy_embedded(&xf, &emb_x, &emb_y, tau_x, tau_y).unwrap();
        assert!(
            te.abs() < 0.15,
            "independent embedded TE should be near zero, got {}",
            te
        );
    }

    #[test]
    fn embedded_te_window_too_small_is_none() {
        let x: Vec<f64> = (0..16).map(|t| (t as f64 * 0.3).sin()).collect();
        let tau = 8;
        let emb_x = embed_series(&x, tau, 3);
        let emb_y = embed_series(&x, tau, 3);
        assert!(emb_x.is_empty());
        assert!(transfer_entropy_embedded(&x, &emb_x, &emb_y, tau, tau).is_none());
    }

    #[test]
    fn pe_ramp_is_zero() {
        let x: Vec<f64> = (0..64).map(|t| t as f64 * 0.1).collect();
        let pe = permutation_entropy(&x, 3, 1).unwrap();
        assert!(pe.abs() < 1e-12, "ordered ramp PE should be 0, got {}", pe);
    }

    #[test]
    fn pe_noise_is_high() {
        let n = 512;
        let mut rng = 99u64;
        let x: Vec<f64> = (0..n)
            .map(|_| {
                rng = rng
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                ((rng >> 33) as f64) / ((u32::MAX >> 1) as f64)
            })
            .collect();
        let pe = permutation_entropy(&x, 3, 1).unwrap();
        assert!(pe > 0.8, "PE of noise should be high, got {}", pe);
    }

    #[test]
    fn pe_constant_is_none() {
        let x = vec![2.0f64; 64];
        assert!(permutation_entropy(&x, 3, 1).is_none());
    }

    #[test]
    fn pe_ties_skip_windows() {
        let mut x: Vec<f64> = (0..48).map(|t| t as f64).collect();
        x[6] = 5.0;
        x[21] = 20.0;
        let (_, used, total) = permutation_entropy_counts(&x, 3, 1).unwrap();
        assert!(
            used < total,
            "tied windows must be skipped: {} of {}",
            used,
            total
        );
    }

    #[test]
    fn pe_short_series_is_none() {
        assert!(permutation_entropy(&[1.0, 2.0], 3, 1).is_none());
    }

    #[test]
    fn topological_pipeline_causal_arrow() {
        let n = 512;
        let mut x = vec![0f32; n];
        let mut y = vec![0f32; n];
        for t in 0..n {
            y[t] = (t as f32 * 0.5).sin();
        }
        for t in 0..n - 1 {
            x[t + 1] = 0.5 * x[t] + 0.6 * y[t];
        }
        let v = topological_te_phase(&x, &y, 3, 3, 42).unwrap();
        assert!(v.tau_x >= 1 && v.tau_y >= 1);
        assert!(
            (2..=10).contains(&v.surrogates_used),
            "surrogates used {}",
            v.surrogates_used
        );
        assert!(
            v.threshold < v.te,
            "threshold {} should be below causal TE {}",
            v.threshold,
            v.te
        );
    }

    #[test]
    fn topological_pipeline_block_variant_runs() {
        let n = 512;
        let mut x = vec![0f32; n];
        let mut y = vec![0f32; n];
        for t in 0..n {
            y[t] = (t as f32 * 0.5).sin();
        }
        for t in 0..n - 1 {
            x[t + 1] = 0.5 * x[t] + 0.6 * y[t];
        }
        let v = topological_te_block(&x, &y, 3, 3, 32, 42).unwrap();
        assert!(v.surrogates_used >= 2);
        assert!(v.threshold.is_finite());
        assert!(v.pe_y.is_some());
    }

    #[test]
    fn hilbert_phase_of_sinusoid_is_linear_ramp() {
        let n = 128;
        let w = 0.3f64;
        let v: Vec<f32> = (0..n).map(|t| (t as f64 * w).sin() as f32).collect();
        let phase = hilbert_instantaneous_phase(&v).unwrap();
        assert_eq!(phase.len(), n);
        let mut unwrapped = phase[0];
        let mut prev = phase[0];
        for &p in &phase[1..] {
            assert!(p.is_finite());
            assert!((-std::f32::consts::PI..=std::f32::consts::PI).contains(&p));
            let mut d = p - prev;
            while d > std::f32::consts::PI {
                d -= 2.0 * std::f32::consts::PI;
            }
            while d < -std::f32::consts::PI {
                d += 2.0 * std::f32::consts::PI;
            }
            unwrapped += d;
            prev = p;
        }
        let slope = (unwrapped - phase[0]) / (n as f32 - 1.0);
        assert!(
            (slope - w as f32).abs() < 0.05,
            "unwrapped phase slope {} should track the carrier {}",
            slope,
            w
        );
    }

    #[test]
    fn phase_te_kuramoto_recovers_direction() {
        let n = 512;
        let dt = 0.05f64;
        let w1 = 0.6f64;
        let w2 = 0.6f64;
        let k12 = 0.8f64;
        let mut t1 = 0.0f64;
        let mut t2 = 0.0f64;
        let mut x = vec![0f32; n];
        let mut y = vec![0f32; n];
        for i in 0..n {
            x[i] = t1.sin() as f32;
            y[i] = t2.sin() as f32;
            t1 += w1 * dt;
            t2 += (w2 + k12 * (t1 - t2).sin()) * dt;
        }
        let fwd = topological_te_instantaneous_phase(&x, &y, 3, 3, 42)
            .expect("phase TE forward must resolve");
        let rev = topological_te_instantaneous_phase(&y, &x, 3, 3, 42)
            .expect("phase TE reverse must resolve");
        assert!(
            fwd.te > rev.te,
            "driver → follower phase TE {} must exceed the reverse {}",
            fwd.te,
            rev.te
        );
    }

    #[test]
    fn topological_pipeline_constant_driver_is_none() {
        let x: Vec<f32> = (0..64).map(|t| (t as f32 * 0.1).sin()).collect();
        let y = vec![3.0f32; 64];
        assert!(topological_te_phase(&x, &y, 3, 3, 7).is_none());
    }

    #[test]
    fn topological_pipeline_surrogate_without_mi_is_skipped() {
        let n = 512;
        let mut x = vec![0f32; n];
        let mut y = vec![0f32; n];
        for t in 0..n {
            y[t] = (t as f32 * 0.5).sin();
        }
        for t in 0..n - 1 {
            x[t + 1] = 0.5 * x[t] + 0.6 * y[t];
        }
        let res = topological_te_with(&x, &y, 3, 3, 42, &mut |v: &[f32],
                                                              _rng: &mut u64|
         -> Vec<f32> {
            vec![1.0; v.len()]
        });
        assert!(res.is_none());
    }

    #[test]
    fn gpu_verdict_assembles_threshold_from_valid_surrogates() {
        let mut v = [0f32; 72];
        v[0] = 4.0;
        v[6] = 3.0;
        v[7] = 0.5;
        v[10] = 1.0;
        v[2] = 0.62;
        v[5] = 1.0;
        v[8] = 0.44;
        v[11] = 1.0;
        v[2 * 6 + 4] = 1.0;
        v[2 * 6 + 1] = 0.1;
        v[3 * 6 + 4] = 1.0;
        v[3 * 6 + 1] = 0.3;
        v[4 * 6 + 4] = 1.0;
        v[4 * 6 + 1] = 0.5;
        let r = topological_verdict_from_gpu(&v).unwrap();
        assert_eq!(r.tau_x, 4);
        assert_eq!(r.tau_y, 3);
        assert_eq!(r.surrogates_used, 3);
        assert!((r.te - 0.5).abs() < 1e-12);
        let a = 0.1f32 as f64;
        let b = 0.3f32 as f64;
        let c = 0.5f32 as f64;
        let expected_mean = (a + b + c) / 3.0;
        let expected_sd = (((a - expected_mean) * (a - expected_mean)
            + (b - expected_mean) * (b - expected_mean)
            + (c - expected_mean) * (c - expected_mean))
            / 3.0)
            .sqrt();
        assert!((r.surrogate_mean - expected_mean).abs() < 1e-12);
        assert!((r.surrogate_sd - expected_sd).abs() < 1e-12);
        assert!((r.threshold - (expected_mean + 2.0 * expected_sd)).abs() < 1e-12);
        assert_eq!(r.pe_x, Some(0.62f32 as f64));
        assert_eq!(r.pe_y, Some(0.44f32 as f64));
    }

    #[test]
    fn gpu_verdict_real_invalid_is_none() {
        let v = [0f32; 72];
        assert!(topological_verdict_from_gpu(&v).is_none());
    }

    #[test]
    fn gpu_verdict_fewer_than_two_surrogates_is_none() {
        let mut v = [0f32; 72];
        v[10] = 1.0;
        v[2 * 6 + 4] = 1.0;
        assert!(topological_verdict_from_gpu(&v).is_none());
    }

    #[test]
    fn gpu_verdict_pe_invalid_is_none_value() {
        let mut v = [0f32; 72];
        v[10] = 1.0;
        v[2 * 6 + 4] = 1.0;
        v[2 * 6 + 1] = 0.1;
        v[3 * 6 + 4] = 1.0;
        v[3 * 6 + 1] = 0.2;
        let r = topological_verdict_from_gpu(&v).unwrap();
        assert_eq!(r.pe_x, None);
        assert_eq!(r.pe_y, None);
    }

    fn gate_rng(rng: &mut u64) -> f64 {
        *rng = rng
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        ((*rng >> 33) as f64) / ((u32::MAX >> 1) as f64)
    }

    fn gate_ar1(n: usize, phi: f64, rng: &mut u64) -> Vec<f32> {
        let mut v = Vec::with_capacity(n);
        let mut x = 0.0f64;
        for _ in 0..n {
            x = phi * x + gate_rng(rng) * 2.0 - 1.0;
            v.push(x as f32);
        }
        v
    }

    #[test]
    fn calibration_fp_independent_ar1_stays_near_chance() {
        let mut rng = 0x9E37_79B9_7F4A_7C15u64;
        let mut fp = 0usize;
        let mut meas = 0usize;
        for t in 0..30 {
            let seed = 0x9E37_79B9_7F4A_7C15 ^ (t as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15);
            let a = gate_ar1(300, 0.7, &mut rng);
            let b = gate_ar1(300, 0.7, &mut rng);
            if let Some(v) = topological_te_phase(&a, &b, 3, 3, seed) {
                meas += 1;
                if v.te > v.threshold {
                    fp += 1;
                }
            }
        }
        assert!(
            meas >= 20,
            "Kalibrier-Gate: {} of 30 measurable — the machine stays silent too often",
            meas
        );
        assert!(
            fp <= 8,
            "Kalibrier-Gate FP: {} of {} above the threshold — the null does not hold (100 % before the RNG-Fix)",
            fp,
            meas
        );
    }

    #[test]
    fn calibration_fn_true_coupling_is_found() {
        let mut rng = 0x517C_C1B7_2722_0A95u64;
        let mut found = 0usize;
        let mut meas = 0usize;
        for t in 0..20 {
            let seed = 0x9E37_79B9_7F4A_7C15 ^ (t as u64).wrapping_mul(0x517C_C1B7_2722_0A95);
            let a: Vec<f32> = (0..300)
                .map(|_| (gate_rng(&mut rng) * 2.0 - 1.0) as f32)
                .collect();
            let b: Vec<f32> = (0..a.len())
                .map(|i| {
                    if i == 0 {
                        gate_rng(&mut rng) as f32
                    } else {
                        (0.9 * a[i - 1] as f64 + (gate_rng(&mut rng) * 0.2 - 0.1)) as f32
                    }
                })
                .collect();
            if let (Some(te), Some((_, _, thr))) = (
                transfer_entropy_lag(&b, &a, 0),
                surrogate_stats_phase(&b, &a, 0, seed),
            ) {
                meas += 1;
                if te > thr {
                    found += 1;
                }
            }
        }
        assert!(
            found as f64 / meas.max(1) as f64 > 0.5,
            "Kalibrier-Gate FN: {} of {} true couplings found — the machine overlooks the coupling",
            found,
            meas
        );
    }

    #[test]
    fn calibration_symmetry_identical_series_measure_equally() {
        let mut rng = 0x2722_0A95_517C_C1B7u64;
        let a = gate_ar1(300, 0.7, &mut rng);
        let b = a.clone();
        let ab = topological_te_phase(&a, &b, 3, 3, 0x9E37_79B9_7F4A_7C15);
        let ba = topological_te_phase(&b, &a, 3, 3, 0x9E37_79B9_7F4A_7C15);
        match (ab, ba) {
            (Some(x), Some(y)) => assert!(
                (x.te - y.te).abs() < 1e-12,
                "Kalibrier-Gate symmetry: a=b measures unequal, {} vs {}",
                x.te,
                y.te
            ),
            (None, None) => {}
            _ => panic!("Kalibrier-Gate symmetry: one direction measurable, the other not"),
        }
    }

    #[test]
    fn calibration_n_floor_no_statement_below_threshold() {
        let mut rng = 0x9E37_79B9_7F4A_7C15u64;
        let a = gate_ar1(16, 0.7, &mut rng);
        let b = gate_ar1(16, 0.7, &mut rng);
        let ab = topological_te_phase(&a, &b, 3, 3, 0x9E37_79B9_7F4A_7C15);
        let ba = topological_te_phase(&b, &a, 3, 3, 0x9E37_79B9_7F4A_7C15);
        assert!(
            ab.is_none() || ba.is_none(),
            "Kalibrier-Gate n-Floor: n=16 carries no verdict"
        );
    }

    #[test]
    fn conditional_te_suppresses_shared_driver() {
        let n = 400;
        let c: Vec<f32> = (0..n).map(|t| (t as f32 * 0.26).sin()).collect();
        let mut rng = 3u64;
        let noise = |rng: &mut u64| -> f32 {
            *rng = rng
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            (((*rng >> 33) as f64) / ((u32::MAX >> 1) as f64)) as f32
        };
        let x: Vec<f32> = c.iter().map(|&z| z + 0.4 * noise(&mut rng)).collect();
        let y: Vec<f32> = c.iter().map(|&z| z + 0.4 * noise(&mut rng)).collect();
        let te = transfer_entropy_lag(&x, &y, 1).expect("unconditional TE resolves");
        let te_c = transfer_entropy_conditional(&x, &y, &c, 1).expect("conditional TE resolves");
        assert!(
            te_c < te,
            "conditioning on the shared driver must suppress TE: {te_c} >= {te}"
        );
        assert!(
            te_c.abs() < 0.05,
            "conditional TE of common-driver-only pair should be near zero, got {te_c}"
        );
    }

    #[test]
    fn conditional_te_keeps_true_coupling() {
        let n = 400;
        let c: Vec<f32> = (0..n).map(|t| (t as f32 * 0.26).sin()).collect();
        let mut rng = 7u64;
        let noise = |rng: &mut u64| -> f32 {
            *rng = rng
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            (((*rng >> 33) as f64) / ((u32::MAX >> 1) as f64)) as f32
        };
        let y: Vec<f32> = c.iter().map(|&z| z + 0.3 * noise(&mut rng)).collect();
        let mut x = vec![0f32; n];
        for t in 0..n {
            x[t] = c[t] + 0.4 * noise(&mut rng);
        }
        for t in 0..n - 1 {
            x[t + 1] += 0.6 * y[t];
        }
        let te_c = transfer_entropy_conditional(&x, &y, &c, 1).expect("conditional TE resolves");
        assert!(
            te_c > 0.05,
            "true coupling beyond the shared driver must survive conditioning, got {te_c}"
        );
    }

    #[test]
    fn conditional_te_surrogate_stats_threshold_is_finite() {
        let n = 300;
        let c: Vec<f32> = (0..n).map(|t| (t as f32 * 0.26).sin()).collect();
        let mut rng = 11u64;
        let noise = |rng: &mut u64| -> f32 {
            *rng = rng
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            (((*rng >> 33) as f64) / ((u32::MAX >> 1) as f64)) as f32
        };
        let x: Vec<f32> = c.iter().map(|&z| z + 0.4 * noise(&mut rng)).collect();
        let y: Vec<f32> = c.iter().map(|&z| z + 0.4 * noise(&mut rng)).collect();
        let (mean, sd, threshold) =
            conditional_te_stats(&x, &y, &c, 1, 0x9E37_79B9_7F4A_7C15, 10).expect("stats resolve");
        assert!(mean.is_finite() && sd.is_finite() && threshold.is_finite());
        assert!(threshold >= mean);
    }
}
