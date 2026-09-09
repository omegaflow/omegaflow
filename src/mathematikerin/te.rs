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
    surrogate_threshold_with(x, y, seed, 10)
}

pub fn surrogate_threshold_with(x: &[f32], y: &[f32], seed: u64, n_surr: usize) -> Option<f64> {
    let mut vals: Vec<f64> = Vec::with_capacity(n_surr);
    let mut rng = seed.wrapping_add(0x9e3779b97f4a7c15);
    for _ in 0..n_surr {
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

pub fn transfer_entropy_lag_h(x: &[f32], y: &[f32], lag: usize, factor: f64) -> Option<f64> {
    let n = x.len();
    if n < 8 {
        return None;
    }
    let shift = if lag == 0 { 1usize } else { lag };
    let m = n - shift;
    if m < 8 {
        return None;
    }
    let hx = silverman(x)? * factor;
    let hy = silverman(y)? * factor;
    let mut te = 0.0;
    for t in 0..m {
        let xt = x[t] as f64;
        let xt1 = x[t + shift] as f64;
        let yt = y[t] as f64;
        let mut k3 = 0.0;
        for s in 0..m {
            k3 += gaussian(xt1 - x[s + shift] as f64, hx)
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
            k2x += gaussian(xt1 - x[s + shift] as f64, hx) * gaussian(xt - x[s] as f64, hx);
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

pub fn transfer_entropy_conditional_h(
    x: &[f32],
    y: &[f32],
    c: &[f32],
    lag: usize,
    factor: f64,
) -> Option<f64> {
    let n = x.len();
    if n < 8 || y.len() < n || c.len() < n {
        return None;
    }
    if lag == 0 {
        return transfer_entropy_conditional_h(x, y, c, 1, factor);
    }
    let m = n - lag;
    if m < 8 {
        return None;
    }
    let hx = silverman(x)? * factor;
    let hy = silverman(y)? * factor;
    let hz = silverman(c)? * factor;
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

pub fn transfer_entropy_conditional_2(
    x: &[f32],
    y: &[f32],
    c1: &[f32],
    c2: &[f32],
    lag: usize,
) -> Option<f64> {
    let n = x.len();
    if n < 8 || y.len() < n || c1.len() < n || c2.len() < n {
        return None;
    }
    if lag == 0 {
        return transfer_entropy_conditional_2(x, y, c1, c2, 1);
    }
    let m = n - lag;
    if m < 8 {
        return None;
    }
    let hx = silverman(x)?;
    let hy = silverman(y)?;
    let h1 = silverman(c1)?;
    let h2 = silverman(c2)?;
    let mut te = 0.0;
    for t in 0..m {
        let xt = x[t] as f64;
        let xk = x[t + lag] as f64;
        let yt = y[t] as f64;
        let z1 = c1[t] as f64;
        let z2 = c2[t] as f64;
        let mut k5 = 0.0;
        for s in 0..m {
            k5 += gaussian(xk - x[s + lag] as f64, hx)
                * gaussian(xt - x[s] as f64, hx)
                * gaussian(yt - y[s] as f64, hy)
                * gaussian(z1 - c1[s] as f64, h1)
                * gaussian(z2 - c2[s] as f64, h2);
        }
        let p5 = k5 / m as f64;
        let mut k3 = 0.0;
        for s in 0..n {
            k3 += gaussian(xt - x[s] as f64, hx)
                * gaussian(z1 - c1[s] as f64, h1)
                * gaussian(z2 - c2[s] as f64, h2);
        }
        let p3 = k3 / n as f64;
        let mut k4a = 0.0;
        for s in 0..n {
            k4a += gaussian(xt - x[s] as f64, hx)
                * gaussian(yt - y[s] as f64, hy)
                * gaussian(z1 - c1[s] as f64, h1)
                * gaussian(z2 - c2[s] as f64, h2);
        }
        let p4a = k4a / n as f64;
        let mut k4b = 0.0;
        for s in 0..m {
            k4b += gaussian(xk - x[s + lag] as f64, hx)
                * gaussian(xt - x[s] as f64, hx)
                * gaussian(z1 - c1[s] as f64, h1)
                * gaussian(z2 - c2[s] as f64, h2);
        }
        let p4b = k4b / m as f64;
        te += ((p5 * p3) / (p4a * p4b).max(1e-300)).ln();
    }
    Some(te / m as f64)
}

fn bin_edges(v: &[f32]) -> Option<(f32, f32)> {
    let mut mn = f32::INFINITY;
    let mut mx = f32::NEG_INFINITY;
    for &x in v {
        if !x.is_finite() {
            return None;
        }
        mn = mn.min(x);
        mx = mx.max(x);
    }
    if mx <= mn {
        return None;
    }
    Some((mn, mx))
}

fn bin_index(v: f32, mn: f32, range: f32, bins: usize) -> usize {
    let mut b = (((v - mn) / range) * bins as f32) as usize;
    if b >= bins {
        b = bins - 1;
    }
    b
}

fn joint_key(indices: &[usize], bins: usize) -> u64 {
    let mut k = 0u64;
    let mut r = 1u64;
    for &i in indices {
        k += (i as u64) * r;
        r *= bins as u64;
    }
    k
}

pub fn transfer_entropy_binned(x: &[f32], y: &[f32], lag: usize, bins: usize) -> Option<f64> {
    transfer_entropy_conditional_binned_n(x, y, &[], lag, bins)
}

fn digamma(x: f64) -> f64 {
    let mut v = x;
    let mut acc = 0.0;
    while v < 8.0 {
        acc -= 1.0 / v;
        v += 1.0;
    }
    let inv = 1.0 / v;
    let inv2 = inv * inv;
    acc + v.ln()
        - 0.5 * inv
        - inv2 * (1.0 / 12.0 - inv2 * (1.0 / 120.0 - inv2 * (1.0 / 252.0 - inv2 * (1.0 / 240.0))))
}

pub fn transfer_entropy_ksg_conditional_n(
    x: &[f32],
    y: &[f32],
    conds: &[&[f32]],
    lag: usize,
    k: usize,
) -> Option<f64> {
    let n = x.len();
    if n < 8 || y.len() < n || k == 0 {
        return None;
    }
    for c in conds {
        if c.len() < n {
            return None;
        }
    }
    if x.iter().chain(y.iter()).any(|v| !v.is_finite()) {
        return None;
    }
    for c in conds {
        if c.iter().any(|v| !v.is_finite()) {
            return None;
        }
    }
    let shift = if lag == 0 { 1usize } else { lag };
    let m = n.checked_sub(shift)?;
    if m < 8 {
        return None;
    }
    let dim = 3 + conds.len();
    let mut pts: Vec<f64> = Vec::with_capacity(m * dim);
    for t in 0..m {
        pts.push(x[t + shift] as f64);
        pts.push(x[t] as f64);
        pts.push(y[t] as f64);
        for c in conds {
            pts.push(c[t] as f64);
        }
    }
    let k_eff = k.min(m - 1);
    let mut dists: Vec<f64> = Vec::with_capacity(m - 1);
    let mut sum = 0.0f64;
    for i in 0..m {
        dists.clear();
        for j in 0..m {
            if j == i {
                continue;
            }
            let mut d = 0.0f64;
            for di in 0..dim {
                let dd = (pts[j * dim + di] - pts[i * dim + di]).abs();
                if dd > d {
                    d = dd;
                }
            }
            dists.push(d);
        }
        let eps = *dists
            .select_nth_unstable_by(k_eff - 1, |a, b| a.total_cmp(b))
            .1;
        let mut n_xc = 0usize;
        let mut n_xxc = 0usize;
        let mut n_xyc = 0usize;
        for j in 0..m {
            if j == i {
                continue;
            }
            let dx = (pts[j * dim + 1] - pts[i * dim + 1]).abs() < eps;
            let dxf = (pts[j * dim] - pts[i * dim]).abs() < eps;
            let dy = (pts[j * dim + 2] - pts[i * dim + 2]).abs() < eps;
            let dc = (3..dim).all(|di| (pts[j * dim + di] - pts[i * dim + di]).abs() < eps);
            let xc = dx && dc;
            if xc {
                n_xc += 1;
            }
            if dxf && xc {
                n_xxc += 1;
            }
            if dy && xc {
                n_xyc += 1;
            }
        }
        sum +=
            digamma((n_xc + 1) as f64) - digamma((n_xxc + 1) as f64) - digamma((n_xyc + 1) as f64);
    }
    Some(digamma(k_eff as f64) + sum / m as f64)
}

pub fn transfer_entropy_conditional_binned_n(
    x: &[f32],
    y: &[f32],
    conds: &[&[f32]],
    lag: usize,
    bins: usize,
) -> Option<f64> {
    let n = x.len();
    if n < 8 || bins < 2 || y.len() < n {
        return None;
    }
    for c in conds {
        if c.len() < n {
            return None;
        }
    }
    let shift = if lag == 0 { 1usize } else { lag };
    let m = n.checked_sub(shift)?;
    if m < 8 {
        return None;
    }
    let (mn_x, mx_x) = bin_edges(x)?;
    let range_x = mx_x - mn_x;
    let (mn_y, mx_y) = bin_edges(y)?;
    let range_y = mx_y - mn_y;
    let mut cond_edges: Vec<(f32, f32)> = Vec::with_capacity(conds.len());
    for c in conds {
        let (mn, mx) = bin_edges(c)?;
        cond_edges.push((mn, mx - mn));
    }
    let bx: Vec<usize> = x
        .iter()
        .map(|&v| bin_index(v, mn_x, range_x, bins))
        .collect();
    let by: Vec<usize> = y
        .iter()
        .map(|&v| bin_index(v, mn_y, range_y, bins))
        .collect();
    let bcond: Vec<Vec<usize>> = conds
        .iter()
        .zip(&cond_edges)
        .map(|(c, &(mn, range))| c.iter().map(|&v| bin_index(v, mn, range, bins)).collect())
        .collect();

    let mut keybuf: Vec<usize> = Vec::with_capacity(conds.len() + 3);
    let mut map_full: std::collections::HashMap<u64, usize> = std::collections::HashMap::new();
    let mut map_xz: std::collections::HashMap<u64, usize> = std::collections::HashMap::new();
    let mut map_xyz: std::collections::HashMap<u64, usize> = std::collections::HashMap::new();
    let mut map_fxz: std::collections::HashMap<u64, usize> = std::collections::HashMap::new();

    for s in 0..m {
        keybuf.clear();
        keybuf.push(bx[s + shift]);
        keybuf.push(bx[s]);
        keybuf.push(by[s]);
        for b in &bcond {
            keybuf.push(b[s]);
        }
        *map_full.entry(joint_key(&keybuf, bins)).or_insert(0) += 1;
    }
    for s in 0..n {
        keybuf.clear();
        keybuf.push(bx[s]);
        for b in &bcond {
            keybuf.push(b[s]);
        }
        *map_xz.entry(joint_key(&keybuf, bins)).or_insert(0) += 1;
    }
    for s in 0..n {
        keybuf.clear();
        keybuf.push(bx[s]);
        keybuf.push(by[s]);
        for b in &bcond {
            keybuf.push(b[s]);
        }
        *map_xyz.entry(joint_key(&keybuf, bins)).or_insert(0) += 1;
    }
    for s in 0..m {
        keybuf.clear();
        keybuf.push(bx[s + shift]);
        keybuf.push(bx[s]);
        for b in &bcond {
            keybuf.push(b[s]);
        }
        *map_fxz.entry(joint_key(&keybuf, bins)).or_insert(0) += 1;
    }

    let mf = m as f64;
    let nf = n as f64;
    let mut te = 0.0;
    for t in 0..m {
        keybuf.clear();
        keybuf.push(bx[t + shift]);
        keybuf.push(bx[t]);
        keybuf.push(by[t]);
        for b in &bcond {
            keybuf.push(b[t]);
        }
        let p5 = *map_full.get(&joint_key(&keybuf, bins)).unwrap_or(&0) as f64 / mf;
        keybuf.clear();
        keybuf.push(bx[t]);
        for b in &bcond {
            keybuf.push(b[t]);
        }
        let p3 = *map_xz.get(&joint_key(&keybuf, bins)).unwrap_or(&0) as f64 / nf;
        keybuf.clear();
        keybuf.push(bx[t]);
        keybuf.push(by[t]);
        for b in &bcond {
            keybuf.push(b[t]);
        }
        let p4a = *map_xyz.get(&joint_key(&keybuf, bins)).unwrap_or(&0) as f64 / nf;
        keybuf.clear();
        keybuf.push(bx[t + shift]);
        keybuf.push(bx[t]);
        for b in &bcond {
            keybuf.push(b[t]);
        }
        let p4b = *map_fxz.get(&joint_key(&keybuf, bins)).unwrap_or(&0) as f64 / mf;
        te += ((p5 * p3) / (p4a * p4b).max(1e-300)).ln();
    }
    Some(te / mf)
}

fn ols_fit(y: &[f32], c: &[f32]) -> Option<(f64, f64)> {
    if y.len() < 2 || y.len() != c.len() {
        return None;
    }
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
    let mut ssc = 0.0;
    for i in 0..n {
        num += (cf[i] - xm) * (yf[i] - ym);
        den += (cf[i] - xm) * (cf[i] - xm);
        ssc += cf[i] * cf[i];
    }
    if den <= 1e-12 * ssc {
        return None;
    }
    let beta1 = num / den;
    Some((beta1, ym - beta1 * xm))
}

pub fn ols_residual(y: &[f32], c: &[f32]) -> Option<Vec<f32>> {
    let n = y.len();
    if n < 2 || y.len() != c.len() {
        return None;
    }
    let (beta1, beta0) = ols_fit(y, c)?;
    let yf: Vec<f64> = y.iter().map(|&v| v as f64).collect();
    let cf: Vec<f64> = c.iter().map(|&v| v as f64).collect();
    Some(
        (0..n)
            .map(|i| (yf[i] - (beta0 + beta1 * cf[i])) as f32)
            .collect(),
    )
}

fn residual_surrogate_conditional(y: &[f32], c: &[f32], rng: &mut u64) -> Vec<f32> {
    let n = y.len();
    match ols_fit(y, c) {
        Some((beta1, beta0)) => {
            let yf: Vec<f64> = y.iter().map(|&v| v as f64).collect();
            let cf: Vec<f64> = c.iter().map(|&v| v as f64).collect();
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
        None => shuffle_series(y, rng),
    }
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

fn solve_linear(a: &[f64], b: &[f64], k: usize) -> Option<Vec<f64>> {
    let mut m = vec![0f64; k * (k + 1)];
    for i in 0..k {
        for j in 0..k {
            m[i * (k + 1) + j] = a[i * k + j];
        }
        m[i * (k + 1) + k] = b[i];
    }
    for col in 0..k {
        let mut pivot = col;
        let mut best = m[col * (k + 1) + col].abs();
        for r in (col + 1)..k {
            let v = m[r * (k + 1) + col].abs();
            if v > best {
                best = v;
                pivot = r;
            }
        }
        if best <= 1e-12 {
            return None;
        }
        if pivot != col {
            for j in 0..=k {
                m.swap(col * (k + 1) + j, pivot * (k + 1) + j);
            }
        }
        let d = m[col * (k + 1) + col];
        for j in col..=k {
            m[col * (k + 1) + j] /= d;
        }
        for r in 0..k {
            if r == col {
                continue;
            }
            let f = m[r * (k + 1) + col];
            if f == 0.0 {
                continue;
            }
            for j in col..=k {
                m[r * (k + 1) + j] -= f * m[col * (k + 1) + j];
            }
        }
    }
    Some((0..k).map(|i| m[i * (k + 1) + k]).collect())
}

fn ols_fit_lagged(y: &[f32], c: &[f32], max_lag: usize) -> Option<Vec<f64>> {
    let n = y.len();
    if n < max_lag + 4 || y.len() != c.len() {
        return None;
    }
    let k = 1 + max_lag + (max_lag + 1);
    let mut a = vec![0f64; k * k];
    let mut b = vec![0f64; k];
    for t in max_lag..n {
        let mut row = Vec::with_capacity(k);
        row.push(1.0);
        for l in 1..=max_lag {
            row.push(y[t - l] as f64);
        }
        for l in 0..=max_lag {
            row.push(c[t - l] as f64);
        }
        let yt = y[t] as f64;
        for i in 0..k {
            b[i] += row[i] * yt;
            for j in 0..k {
                a[i * k + j] += row[i] * row[j];
            }
        }
    }
    solve_linear(&a, &b, k)
}

fn lagged_predict(coeffs: &[f64], y: &[f32], c: &[f32], t: usize, max_lag: usize) -> f64 {
    let mut v = coeffs[0];
    for l in 1..=max_lag {
        v += coeffs[l] * y[t - l] as f64;
    }
    for l in 0..=max_lag {
        v += coeffs[1 + max_lag + l] * c[t - l] as f64;
    }
    v
}

fn residual_surrogate_conditional_lagged(
    y: &[f32],
    c: &[f32],
    max_lag: usize,
    rng: &mut u64,
) -> Vec<f32> {
    let n = y.len();
    match ols_fit_lagged(y, c, max_lag) {
        Some(coeffs) => {
            let mut resid: Vec<f64> = (max_lag..n)
                .map(|t| y[t] as f64 - lagged_predict(&coeffs, y, c, t, max_lag))
                .collect();
            for i in (1..resid.len()).rev() {
                *rng = rng
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                let j = ((*rng >> 33) as usize) % (i + 1);
                resid.swap(i, j);
            }
            let mut out = vec![0f32; n];
            for t in 0..n {
                out[t] = if t < max_lag {
                    y[t]
                } else {
                    (lagged_predict(&coeffs, y, c, t, max_lag) + resid[t - max_lag]) as f32
                };
            }
            out
        }
        None => shuffle_series(y, rng),
    }
}

pub fn conditional_te_stats_lagged(
    x: &[f32],
    y: &[f32],
    c: &[f32],
    lag: usize,
    max_lag: usize,
    seed: u64,
    n_surr: usize,
) -> Option<(f64, f64, f64)> {
    let mut vals: Vec<f64> = Vec::with_capacity(n_surr);
    let mut rng = seed.wrapping_add(0x9e3779b97f4a7c15);
    for _ in 0..n_surr {
        let ys = residual_surrogate_conditional_lagged(y, c, max_lag, &mut rng);
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

fn ols_fit_lagged_2(y: &[f32], c1: &[f32], c2: &[f32], max_lag: usize) -> Option<Vec<f64>> {
    let n = y.len();
    if n < max_lag + 4 || y.len() != c1.len() || y.len() != c2.len() {
        return None;
    }
    let k = 1 + max_lag + (max_lag + 1) + (max_lag + 1);
    let mut a = vec![0f64; k * k];
    let mut b = vec![0f64; k];
    for t in max_lag..n {
        let mut row = Vec::with_capacity(k);
        row.push(1.0);
        for l in 1..=max_lag {
            row.push(y[t - l] as f64);
        }
        for l in 0..=max_lag {
            row.push(c1[t - l] as f64);
        }
        for l in 0..=max_lag {
            row.push(c2[t - l] as f64);
        }
        let yt = y[t] as f64;
        for i in 0..k {
            b[i] += row[i] * yt;
            for j in 0..k {
                a[i * k + j] += row[i] * row[j];
            }
        }
    }
    solve_linear(&a, &b, k)
}

fn lagged_predict_2(
    coeffs: &[f64],
    y: &[f32],
    c1: &[f32],
    c2: &[f32],
    t: usize,
    max_lag: usize,
) -> f64 {
    let mut v = coeffs[0];
    for l in 1..=max_lag {
        v += coeffs[l] * y[t - l] as f64;
    }
    for l in 0..=max_lag {
        v += coeffs[1 + max_lag + l] * c1[t - l] as f64;
    }
    for l in 0..=max_lag {
        v += coeffs[2 + 2 * max_lag + l] * c2[t - l] as f64;
    }
    v
}

fn residual_surrogate_conditional_lagged_2(
    y: &[f32],
    c1: &[f32],
    c2: &[f32],
    max_lag: usize,
    rng: &mut u64,
) -> Vec<f32> {
    let n = y.len();
    match ols_fit_lagged_2(y, c1, c2, max_lag) {
        Some(coeffs) => {
            let mut resid: Vec<f64> = (max_lag..n)
                .map(|t| y[t] as f64 - lagged_predict_2(&coeffs, y, c1, c2, t, max_lag))
                .collect();
            for i in (1..resid.len()).rev() {
                *rng = rng
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                let j = ((*rng >> 33) as usize) % (i + 1);
                resid.swap(i, j);
            }
            let mut out = vec![0f32; n];
            for t in 0..n {
                out[t] = if t < max_lag {
                    y[t]
                } else {
                    (lagged_predict_2(&coeffs, y, c1, c2, t, max_lag) + resid[t - max_lag]) as f32
                };
            }
            out
        }
        None => shuffle_series(y, rng),
    }
}

pub fn conditional_te_stats_lagged_2(
    x: &[f32],
    y: &[f32],
    c1: &[f32],
    c2: &[f32],
    lag: usize,
    max_lag: usize,
    seed: u64,
    n_surr: usize,
) -> Option<(f64, f64, f64)> {
    let mut vals: Vec<f64> = Vec::with_capacity(n_surr);
    let mut rng = seed.wrapping_add(0x9e3779b97f4a7c15);
    for _ in 0..n_surr {
        let ys = residual_surrogate_conditional_lagged_2(y, c1, c2, max_lag, &mut rng);
        if let Some(te) = transfer_entropy_conditional_2(x, &ys, c1, c2, lag) {
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

fn ols_fit_lagged_n(y: &[f32], conds: &[&[f32]], max_lag: usize) -> Option<Vec<f64>> {
    let n = y.len();
    if n < max_lag + 4 {
        return None;
    }
    for c in conds {
        if c.len() != n {
            return None;
        }
    }
    let n_cond = conds.len();
    let k = 1 + max_lag + n_cond * (max_lag + 1);
    let mut a = vec![0f64; k * k];
    let mut b = vec![0f64; k];
    for t in max_lag..n {
        let mut row = Vec::with_capacity(k);
        row.push(1.0);
        for l in 1..=max_lag {
            row.push(y[t - l] as f64);
        }
        for c in conds {
            for l in 0..=max_lag {
                row.push(c[t - l] as f64);
            }
        }
        let yt = y[t] as f64;
        for i in 0..k {
            b[i] += row[i] * yt;
            for j in 0..k {
                a[i * k + j] += row[i] * row[j];
            }
        }
    }
    solve_linear(&a, &b, k)
}

fn lagged_predict_n(coeffs: &[f64], y: &[f32], conds: &[&[f32]], t: usize, max_lag: usize) -> f64 {
    let mut v = coeffs[0];
    for l in 1..=max_lag {
        v += coeffs[l] * y[t - l] as f64;
    }
    for (ci, c) in conds.iter().enumerate() {
        let base = 1 + max_lag + ci * (max_lag + 1);
        for l in 0..=max_lag {
            v += coeffs[base + l] * c[t - l] as f64;
        }
    }
    v
}

fn residual_surrogate_conditional_lagged_n(
    y: &[f32],
    conds: &[&[f32]],
    max_lag: usize,
    rng: &mut u64,
) -> Vec<f32> {
    let n = y.len();
    match ols_fit_lagged_n(y, conds, max_lag) {
        Some(coeffs) => {
            let mut resid: Vec<f64> = (max_lag..n)
                .map(|t| y[t] as f64 - lagged_predict_n(&coeffs, y, conds, t, max_lag))
                .collect();
            for i in (1..resid.len()).rev() {
                *rng = rng
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                let j = ((*rng >> 33) as usize) % (i + 1);
                resid.swap(i, j);
            }
            let mut out = vec![0f32; n];
            for t in 0..n {
                out[t] = if t < max_lag {
                    y[t]
                } else {
                    (lagged_predict_n(&coeffs, y, conds, t, max_lag) + resid[t - max_lag]) as f32
                };
            }
            out
        }
        None => shuffle_series(y, rng),
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TeNull {
    Residual,
    Block,
    Shift,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TeEstimator {
    Binned,
    Ksg,
}

pub fn block_len_from_n(n: usize) -> usize {
    (n as f64).powf(1.0 / 3.0).round() as usize
}

pub fn conditional_te_stats_lagged_n(
    x: &[f32],
    y: &[f32],
    conds: &[&[f32]],
    lag: usize,
    max_lag: usize,
    bins: usize,
    seed: u64,
    n_surr: usize,
    null: TeNull,
) -> Option<(f64, f64, f64)> {
    let vals = conditional_te_surrogates_n(
        x,
        y,
        conds,
        lag,
        max_lag,
        bins,
        seed,
        n_surr,
        null,
        0,
        TeEstimator::Binned,
        4,
    )?;
    let n = vals.len() as f64;
    let mean = vals.iter().sum::<f64>() / n;
    let var = vals.iter().map(|&v| (v - mean) * (v - mean)).sum::<f64>() / n;
    let sd = var.sqrt();
    Some((mean, sd, mean + 2.0 * sd))
}

pub fn conditional_te_surrogates_n(
    x: &[f32],
    y: &[f32],
    conds: &[&[f32]],
    lag: usize,
    max_lag: usize,
    bins: usize,
    seed: u64,
    n_surr: usize,
    null: TeNull,
    block: usize,
    est: TeEstimator,
    k: usize,
) -> Option<Vec<f64>> {
    let mut vals: Vec<f64> = Vec::with_capacity(n_surr);
    let mut rng = seed.wrapping_add(0x9e3779b97f4a7c15);
    let block_len = if block == 0 {
        block_len_from_n(y.len())
    } else {
        block
    };
    for _ in 0..n_surr {
        let ys = match null {
            TeNull::Residual => {
                residual_surrogate_conditional_lagged_n(y, conds, max_lag, &mut rng)
            }
            TeNull::Block => block_bootstrap_surrogate(y, block_len, &mut rng),
            TeNull::Shift => cycle_phase_shift_surrogate(y, y.len(), &mut rng),
        };
        let te = match est {
            TeEstimator::Binned => transfer_entropy_conditional_binned_n(x, &ys, conds, lag, bins),
            TeEstimator::Ksg => transfer_entropy_ksg_conditional_n(x, &ys, conds, lag, k),
        };
        if let Some(te) = te {
            vals.push(te);
        }
    }
    if vals.len() < 2 {
        return None;
    }
    Some(vals)
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

pub struct CausalLink {
    pub driver: usize,
    pub target: usize,
    pub lag: usize,
    pub te: f64,
    pub threshold: f64,
    pub p_value: f64,
    pub fdr_pass: bool,
}

fn subsets_of_size(v: &[(usize, usize)], p: usize) -> Vec<Vec<(usize, usize)>> {
    fn rec(
        v: &[(usize, usize)],
        p: usize,
        start: usize,
        cur: &mut Vec<(usize, usize)>,
        out: &mut Vec<Vec<(usize, usize)>>,
    ) {
        if cur.len() == p {
            out.push(cur.clone());
            return;
        }
        for idx in start..v.len() {
            cur.push(v[idx]);
            rec(v, p, idx + 1, cur, out);
            cur.pop();
        }
    }
    let mut out = Vec::new();
    rec(v, p, 0, &mut Vec::new(), &mut out);
    out
}

pub fn pcmci_links(
    series: &[&[f32]],
    max_lag: usize,
    null_lag: usize,
    bins: usize,
    seed: u64,
    n_surr: usize,
    null: TeNull,
    block: usize,
    est: TeEstimator,
    k: usize,
    p_max: usize,
    alpha: f64,
) -> Option<Vec<CausalLink>> {
    let n_chan = series.len();
    if n_chan < 2 {
        return None;
    }
    let n = series[0].len();
    for s in series {
        if s.len() != n {
            return None;
        }
    }
    if !(alpha > 0.0 && alpha <= 1.0) {
        return None;
    }

    let test = |j: usize,
                i: usize,
                lag: usize,
                conds: &[&[f32]],
                seed_t: u64|
     -> Option<(f64, f64, f64)> {
        let te = match est {
            TeEstimator::Binned => {
                transfer_entropy_conditional_binned_n(series[j], series[i], conds, lag, bins)?
            }
            TeEstimator::Ksg => {
                transfer_entropy_ksg_conditional_n(series[j], series[i], conds, lag, k)?
            }
        };
        let surr = conditional_te_surrogates_n(
            series[j], series[i], conds, lag, null_lag, bins, seed_t, n_surr, null, block, est, k,
        )?;
        let mean = surr.iter().sum::<f64>() / surr.len() as f64;
        let var = surr.iter().map(|&v| (v - mean) * (v - mean)).sum::<f64>() / surr.len() as f64;
        let sd = var.sqrt();
        let threshold = mean + 2.0 * sd;
        let p_value = if sd > 0.0 {
            1.0 - normal_cdf((te - mean) / sd)
        } else if te > mean {
            0.0
        } else {
            1.0
        };
        Some((te, threshold, p_value))
    };

    let mut parents: Vec<Vec<(usize, usize)>> = vec![Vec::new(); n_chan];
    for p in 0..=p_max {
        let snapshot: Vec<Vec<(usize, usize)>> = parents.clone();
        let mut removals: Vec<(usize, usize, usize)> = Vec::new();
        for j in 0..n_chan {
            if p == 0 {
                for i in 0..n_chan {
                    if i == j {
                        continue;
                    }
                    for lag in 1..=max_lag {
                        let seed_t = seed
                            ^ (j as u64).wrapping_mul(0x9E37_79B9)
                            ^ (i as u64).wrapping_mul(0x85EB_CA6B)
                            ^ (lag as u64).wrapping_mul(0xC2B2_AE3D);
                        if let Some((te, threshold, _pv)) = test(j, i, lag, &[], seed_t) {
                            if te > threshold {
                                parents[j].push((i, lag));
                            }
                        }
                    }
                }
                continue;
            }
            let pool: Vec<(usize, usize)> = snapshot[j].clone();
            for &(i, lag) in &pool {
                let rest: Vec<(usize, usize)> = pool
                    .iter()
                    .copied()
                    .filter(|&(d, l)| (d, l) != (i, lag))
                    .collect();
                if rest.len() < p {
                    continue;
                }
                let mut removed_here = false;
                for (ci, comb) in subsets_of_size(&rest, p).iter().enumerate() {
                    let conds: Vec<&[f32]> = comb.iter().map(|&(d, _)| series[d]).collect();
                    let seed_t = seed
                        ^ (j as u64).wrapping_mul(0x9E37_79B9)
                        ^ (i as u64).wrapping_mul(0x85EB_CA6B)
                        ^ (lag as u64).wrapping_mul(0xC2B2_AE3D)
                        ^ (p as u64).wrapping_mul(0xD1B5_4A32)
                        ^ (ci as u64).wrapping_mul(0x4A32_D1B5);
                    if let Some((te, threshold, _pv)) = test(j, i, lag, &conds, seed_t) {
                        if te <= threshold {
                            removed_here = true;
                            break;
                        }
                    }
                }
                if removed_here {
                    removals.push((j, i, lag));
                }
            }
        }
        for &(j, i, lag) in &removals {
            parents[j].retain(|&(d, l)| (d, l) != (i, lag));
        }
    }

    let mut links: Vec<CausalLink> = Vec::new();
    for j in 0..n_chan {
        for i in 0..n_chan {
            if i == j {
                continue;
            }
            for lag in 1..=max_lag {
                let mut cond_specs: Vec<(usize, usize)> = parents[j]
                    .iter()
                    .copied()
                    .filter(|&(d, l)| (d, l) != (i, lag))
                    .collect();
                for &(d, l) in &parents[i] {
                    if !cond_specs.contains(&(d, l)) {
                        cond_specs.push((d, l));
                    }
                }
                let conds: Vec<&[f32]> = cond_specs.iter().map(|&(d, _)| series[d]).collect();
                let seed_t = seed
                    ^ (j as u64).wrapping_mul(0x9E37_79B9)
                    ^ (i as u64).wrapping_mul(0x85EB_CA6B)
                    ^ (lag as u64).wrapping_mul(0xC2B2_AE3D);
                if let Some((te, threshold, p_value)) = test(j, i, lag, &conds, seed_t) {
                    links.push(CausalLink {
                        driver: i,
                        target: j,
                        lag,
                        te,
                        threshold,
                        p_value,
                        fdr_pass: false,
                    });
                }
            }
        }
    }
    let p_vals: Vec<f64> = links.iter().map(|l| l.p_value).collect();
    if let Some(cutoff) = benjamini_hochberg(&p_vals, alpha) {
        for l in &mut links {
            if l.p_value <= cutoff {
                l.fdr_pass = true;
            }
        }
    }
    Some(links)
}

pub fn benjamini_hochberg(p_values: &[f64], level: f64) -> Option<f64> {
    if p_values.is_empty() || !(level > 0.0 && level <= 1.0) {
        return None;
    }
    let m = p_values.len() as f64;
    let mut sorted: Vec<f64> = p_values.to_vec();
    sorted.sort_by(|a, b| a.total_cmp(b));
    let mut cutoff = 0.0f64;
    for (k, &p) in sorted.iter().enumerate() {
        let rank = (k + 1) as f64;
        if p <= rank / m * level {
            cutoff = p;
        }
    }
    Some(cutoff)
}

pub fn surrogate_threshold_lag(x: &[f32], y: &[f32], lag: usize, seed: u64) -> Option<f64> {
    surrogate_stats(x, y, lag, seed).map(|(_, _, threshold)| threshold)
}

pub fn surrogate_stats(x: &[f32], y: &[f32], lag: usize, seed: u64) -> Option<(f64, f64, f64)> {
    surrogate_stats_with(x, y, lag, seed, 10, &mut |v, rng| shuffle_series(v, rng))
}

pub fn surrogate_stats_phase(
    x: &[f32],
    y: &[f32],
    lag: usize,
    seed: u64,
) -> Option<(f64, f64, f64)> {
    surrogate_stats_with(x, y, lag, seed, 10, &mut |v, rng| {
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
    surrogate_stats_with(x, y, lag, seed, 10, &mut |v, rng| {
        block_bootstrap_surrogate(v, block, rng)
    })
}

fn surrogate_stats_with(
    x: &[f32],
    y: &[f32],
    lag: usize,
    seed: u64,
    n_surr: usize,
    surrogate: &mut dyn FnMut(&[f32], &mut u64) -> Vec<f32>,
) -> Option<(f64, f64, f64)> {
    let mut vals: Vec<f64> = Vec::with_capacity(n_surr);
    let mut rng = seed.wrapping_add(0x9e3779b97f4a7c15);
    for _ in 0..n_surr {
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

pub fn cycle_phase_shift_surrogate(v: &[f32], cycle_len: usize, rng: &mut u64) -> Vec<f32> {
    let n = v.len();
    if n < 2 || cycle_len < 2 {
        return v.to_vec();
    }
    let mut out = v.to_vec();
    let mut start = 0usize;
    while start + cycle_len <= n {
        let shift = (next_rng(rng) * cycle_len as f64) as usize % cycle_len;
        if shift != 0 {
            out[start..start + cycle_len].rotate_left(shift);
        }
        start += cycle_len;
    }
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
    n_surr: usize,
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
    let mut vals: Vec<f64> = Vec::with_capacity(n_surr);
    let mut rng = seed.wrapping_add(0x9e3779b97f4a7c15);
    for _ in 0..n_surr {
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
    topological_te_with(x, y, dim, order, seed, 10, &mut |v, rng| {
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
    topological_te_with(x, y, dim, order, seed, 10, &mut move |v, rng| {
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
    topological_te_instantaneous_phase_with(x, y, dim, order, seed, 10)
}

fn topological_te_instantaneous_phase_with(
    x: &[f32],
    y: &[f32],
    dim: usize,
    order: usize,
    seed: u64,
    n_surr: usize,
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
    let mut vals: Vec<f64> = Vec::with_capacity(n_surr);
    let mut rng = seed.wrapping_add(0x9e3779b97f4a7c15);
    for _ in 0..n_surr {
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
    fn cycle_phase_shift_preserves_each_cycles_amplitude_exactly() {
        let n = 256;
        let cycle = 16;
        let mut x = Vec::with_capacity(n);
        for c in 0..n / cycle {
            for s in 0..cycle {
                let phase = 2.0 * std::f64::consts::PI * s as f64 / cycle as f64;
                x.push((c as f64 * 0.05 + phase).sin() as f32 + 0.02 * (c % 5) as f32);
            }
        }
        for seed in 1u64..=12 {
            let mut rng = seed.wrapping_mul(0x9E37_79B9_7F4A_7C15);
            let s = cycle_phase_shift_surrogate(&x, cycle, &mut rng);
            assert_eq!(s.len(), n);
            let mut cx = x.chunks_exact(cycle).map(|ch| {
                let mut v = ch.to_vec();
                v.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
                v
            });
            for ch in s.chunks_exact(cycle) {
                let mut v = ch.to_vec();
                v.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
                let orig = cx.next().unwrap();
                for (a, b) in v.iter().zip(&orig) {
                    assert_eq!(a, b, "seed {seed}: cycle amplitude set changed");
                }
            }
        }
    }

    #[test]
    fn cycle_phase_shift_randomizes_the_in_cycle_phase_order() {
        let cycle = 16;
        let cycles = 12;
        let mut x = vec![0f32; cycle * cycles];
        for c in 0..cycles {
            x[c * cycle] = 1.0;
        }
        let mut any_moved = false;
        for seed in 1u64..=8 {
            let mut rng = seed.wrapping_mul(0x9E37_79B9_7F4A_7C15);
            let s = cycle_phase_shift_surrogate(&x, cycle, &mut rng);
            let markers_at_zero = s.chunks_exact(cycle).filter(|ch| ch[0] == 1.0).count();
            if markers_at_zero != cycles {
                any_moved = true;
                break;
            }
        }
        assert!(
            any_moved,
            "no seed shifted the in-cycle marker — the phase order was not randomized"
        );
    }

    #[test]
    fn cycle_phase_shift_leaves_partial_and_short_series_alone() {
        let x = vec![1.0f32, 2.0, 3.0];
        let mut rng = 7u64;
        assert_eq!(cycle_phase_shift_surrogate(&x, 5, &mut rng), x);
        let partial = vec![1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0];
        let mut rng = 7u64;
        let s = cycle_phase_shift_surrogate(&partial, 4, &mut rng);
        assert_eq!(
            s[4..],
            partial[4..],
            "trailing partial cycle stays in place"
        );
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
        let res = topological_te_with(&x, &y, 3, 3, 42, 10, &mut |v: &[f32],
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
        if meas == 0 {
            panic!("Kalibrier-Gate FN: no coupling measurement succeeded in 20 trials");
        }
        assert!(
            found as f64 / meas as f64 > 0.5,
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

    fn gate_gauss(rng: &mut u64) -> f32 {
        loop {
            let u1 = gate_rng(rng) * 2.0 - 1.0;
            let u2 = gate_rng(rng) * 2.0 - 1.0;
            let s = u1 * u1 + u2 * u2;
            if s >= 1.0 || s <= 0.0 {
                continue;
            }
            let m = (-2.0 * (s as f64).ln() / (s as f64)).sqrt() as f32;
            return (u1 as f32) * m;
        }
    }

    fn gate_common_driver(n: usize, a: f32, c: f32, d_z: usize, rng: &mut u64) -> Vec<Vec<f32>> {
        let burn = 200;
        let n_chan = 2 + d_z;
        let mut x = vec![vec![0f32; burn + n]; n_chan];
        for step in 1..burn + n {
            x[0][step] = a * x[0][step - 1] + gate_gauss(rng);
            let mut yv = a * x[1][step - 1] + c * x[0][step - 1] + gate_gauss(rng);
            for d in 0..d_z {
                x[2 + d][step] = a * x[2 + d][step - 1] + 0.25 * gate_gauss(rng);
                yv += 0.5 * x[2 + d][step - 1];
            }
            x[1][step] = yv;
        }
        (0..n_chan).map(|j| x[j][burn..].to_vec()).collect()
    }

    fn gate_fpr_cell(
        a: f32,
        d_z: usize,
        trials: usize,
        null: TeNull,
        n_surr: usize,
        est: TeEstimator,
        rng: &mut u64,
    ) -> (usize, usize) {
        let mut fp = 0usize;
        let mut neg = 0usize;
        for t in 0..trials {
            let seed = 0x9E37_79B9_7F4A_7C15 ^ (t as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15);
            let series = gate_common_driver(150, a, 0.0, d_z, rng);
            let refs: Vec<&[f32]> = series.iter().map(|s| s.as_slice()).collect();
            let Some(links) = pcmci_links(&refs, 2, 12, 4, seed, n_surr, null, 0, est, 4, 2, 0.05)
            else {
                continue;
            };
            let n_chan = refs.len();
            for drv in 0..n_chan {
                for tgt in 0..n_chan {
                    if drv == tgt {
                        continue;
                    }
                    for lag in 1..=2 {
                        neg += 1;
                        if links.iter().any(|k| {
                            k.driver == drv && k.target == tgt && k.lag == lag && k.te > k.threshold
                        }) {
                            fp += 1;
                        }
                    }
                }
            }
        }
        (fp, neg)
    }

    fn gate_fpr_autocorr(null: TeNull, est: TeEstimator) {
        let n_surr = 100;
        let mut rng = 0xC2B2_AE3D_85EB_CA6Bu64;
        let cells = [
            (0.0f32, 0usize, 100usize),
            (0.5f32, 0usize, 100usize),
            (0.9f32, 0usize, 100usize),
            (0.0f32, 4usize, 7usize),
            (0.5f32, 4usize, 7usize),
            (0.9f32, 4usize, 7usize),
        ];
        let mut rows: Vec<(f32, usize, f64)> = Vec::new();
        for &(a, d_z, trials) in &cells {
            let (fp, neg) = gate_fpr_cell(a, d_z, trials, null, n_surr, est, &mut rng);
            rows.push((a, d_z, 100.0 * fp as f64 / neg as f64));
        }
        let named: String = rows
            .iter()
            .map(|&(a, d_z, fpr)| format!("a={a} D_Z={d_z}: {fpr:.2}% "))
            .collect();
        for &(a, d_z, fpr) in &rows {
            assert!(
                fpr <= 8.0,
                "Zug 5: FPR {fpr:.2}% at a={a} D_Z={d_z} exceeds 8% — the null does not hold under autocorrelation ({named})"
            );
        }
        for d_z in [0usize, 4usize] {
            let f0 = rows
                .iter()
                .find(|&&(a, d, _)| a == 0.0 && d == d_z)
                .expect("Zug 5: a=0 cell measured")
                .2;
            let f9 = rows
                .iter()
                .find(|&&(a, d, _)| a == 0.9 && d == d_z)
                .expect("Zug 5: a=0.9 cell measured")
                .2;
            assert!(
                f9 - f0 <= 2.0,
                "Zug 5: FPR rise {:.2}pp over a at D_Z={d_z} exceeds 2pp — the null leaks autocorrelation into the FPR ({named})",
                f9 - f0
            );
        }
    }

    #[test]
    fn gate_fpr_autocorrelation_block_null_binned_n_surr_100() {
        gate_fpr_autocorr(TeNull::Block, TeEstimator::Binned);
    }

    #[test]
    fn gate_fpr_autocorrelation_block_null_ksg_n_surr_100() {
        gate_fpr_autocorr(TeNull::Block, TeEstimator::Ksg);
    }

    fn gate_s60_f(x: f32) -> f32 {
        x
    }

    fn gate_s60_topology(
        n_chan: usize,
        rng: &mut u64,
    ) -> (Vec<f32>, Vec<(usize, usize, usize, f32)>) {
        let a_set = [0.0f32, 0.2, 0.4, 0.6, 0.8, 0.9];
        let mut links: Vec<(usize, usize, usize, f32)> = Vec::new();
        let mut used: Vec<(usize, usize)> = Vec::new();
        while links.len() < n_chan {
            let driver = (gate_rng(rng) * n_chan as f64) as usize;
            let target = (gate_rng(rng) * n_chan as f64) as usize;
            if driver == target || used.contains(&(driver, target)) {
                continue;
            }
            used.push((driver, target));
            let lag = 1 + (gate_rng(rng) * 2.0) as usize;
            let sign = if gate_rng(rng) < 0.5 { -1.0 } else { 1.0 };
            links.push((driver, target, lag, sign));
        }
        let a: Vec<f32> = (0..n_chan)
            .map(|_| a_set[(gate_rng(rng) * a_set.len() as f64) as usize])
            .collect();
        (a, links)
    }

    fn gate_s60_series(
        n_chan: usize,
        t: usize,
        c: f32,
        a: &[f32],
        links: &[(usize, usize, usize, f32)],
        rng: &mut u64,
    ) -> Option<Vec<Vec<f32>>> {
        let burn = 200;
        let mut x = vec![vec![0f32; burn + t]; n_chan];
        for step in 1..burn + t {
            for j in 0..n_chan {
                let mut v = a[j] * x[j][step - 1];
                for &(driver, target, lag, sign) in links {
                    if target == j && step >= lag {
                        v += c * sign * gate_s60_f(x[driver][step - lag]);
                    }
                }
                v += gate_gauss(rng);
                x[j][step] = v;
            }
        }
        let mut out = Vec::with_capacity(n_chan);
        for j in 0..n_chan {
            let col: Vec<f32> = x[j][burn..].to_vec();
            for &v in &col {
                if !v.is_finite() || v.abs() > 100.0 {
                    return None;
                }
            }
            out.push(col);
        }
        Some(out)
    }

    #[test]
    fn gate_ksg_finds_anchor_links_floor() {
        let mut rng = 0x9E37_79B9_7F4A_7C15u64;
        let mut above = 0usize;
        let mut total = 0usize;
        for r in 0..3 {
            let mut drawn: Option<(Vec<Vec<f32>>, Vec<(usize, usize, usize, f32)>)> = None;
            for _ in 0..8 {
                let (a, links) = gate_s60_topology(10, &mut rng);
                if let Some(s) = gate_s60_series(10, 150, 0.287, &a, &links, &mut rng) {
                    drawn = Some((s, links));
                    break;
                }
            }
            let Some((series, links)) = drawn else {
                continue;
            };
            let refs: Vec<&[f32]> = series.iter().map(|s| s.as_slice()).collect();
            for &(driver, target, lag, _sign) in &links {
                let true_parents: Vec<&[f32]> = links
                    .iter()
                    .filter(|&&(d, t, l, _)| (d, t, l) != (driver, target, lag) && t == target)
                    .map(|&(d, _, _, _)| refs[d])
                    .collect();
                total += 1;
                let mut hits = 0usize;
                for s in 0..20 {
                    let seed = 0x9E37_79B9_7F4A_7C15
                        ^ (s as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15)
                        ^ (r as u64).wrapping_mul(0x85EB_CA6B)
                        ^ (driver as u64).wrapping_mul(0xC2B2_AE3D);
                    let Some(te) = transfer_entropy_ksg_conditional_n(
                        refs[target],
                        refs[driver],
                        &true_parents,
                        lag,
                        4,
                    ) else {
                        continue;
                    };
                    let Some(surr) = conditional_te_surrogates_n(
                        refs[target],
                        refs[driver],
                        &true_parents,
                        lag,
                        2,
                        4,
                        seed,
                        100,
                        TeNull::Block,
                        0,
                        TeEstimator::Ksg,
                        4,
                    ) else {
                        continue;
                    };
                    let mean = surr.iter().sum::<f64>() / surr.len() as f64;
                    let var = surr.iter().map(|&v| (v - mean) * (v - mean)).sum::<f64>()
                        / surr.len() as f64;
                    if te > mean + 2.0 * var.sqrt() {
                        hits += 1;
                    }
                }
                if hits as f64 / 20.0 > 0.7 {
                    above += 1;
                }
            }
        }
        assert!(
            above >= 3,
            "Zug 6: the ksg estimator finds {above}/{total} anchor links above 70% power — the estimator is broken where binned was blind"
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

    fn flare_envelope(n: usize, starts: &[usize], amp: f32, tau: f32) -> Vec<f32> {
        let mut c = vec![0f32; n];
        for &s in starts {
            for t in s..n {
                c[t] += amp * (-((t - s) as f32) / tau).exp();
            }
        }
        c
    }

    fn flare_pair(n: usize, seed: u64) -> (Vec<f32>, Vec<f32>, Vec<f32>) {
        let mut rng = seed;
        let noise = |rng: &mut u64| -> f32 {
            *rng = rng
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            (((*rng >> 33) as f64) / ((u32::MAX >> 1) as f64)) as f32
        };
        let c = flare_envelope(n, &[30usize, 150usize], 1.0, 12.0);
        let mut x = vec![0f32; n];
        let mut y = vec![0f32; n];
        let alpha = 0.90f32;
        for t in 0..n {
            x[t] = c[t] + 0.05 * noise(&mut rng);
            y[t] = if t == 0 {
                0.0
            } else {
                alpha * y[t - 1] + (1.0 - alpha) * c[t - 1] + 0.05 * noise(&mut rng)
            };
        }
        (c, x, y)
    }

    #[test]
    fn flare_envelope_phase_null_reports_time_constant_confound() {
        let (_, x, y) = flare_pair(240, 0x7A5B_3C1D_9E4F_6A2Bu64);
        let te_fwd = transfer_entropy_lag(&x, &y, 1).expect("forward TE resolves");
        let te_rev = transfer_entropy_lag(&y, &x, 1).expect("reverse TE resolves");
        let (_, _, thr_fwd) =
            surrogate_stats_phase(&x, &y, 1, 0x9E37_79B9_7F4A_7C15).expect("phase null resolves");
        assert!(
            te_fwd > te_rev,
            "flare-envelope gate blindness: phase-null D must carry x->y (fast->slow), got fwd {} rev {}",
            te_fwd,
            te_rev
        );
        assert!(
            te_fwd > thr_fwd,
            "flare-envelope gate blindness: the false arrow must clear the phase null, got fwd {} thr {}",
            te_fwd,
            thr_fwd
        );
    }

    #[test]
    fn flare_envelope_conditional_suppresses_confound() {
        let (c, x, y) = flare_pair(240, 0x3C1D_9E4F_6A2B_7A5Bu64);
        let te_u = transfer_entropy_lag(&x, &y, 1).expect("unconditional TE resolves");
        let te_c = transfer_entropy_conditional(&x, &y, &c, 1).expect("conditional TE resolves");
        let (_, _, thr_c) =
            conditional_te_stats_lagged(&x, &y, &c, 1, 1, 0x9E37_79B9_7F4A_7C15, 10)
                .expect("lagged conditional null resolves");
        assert!(
            te_c < te_u,
            "flare-envelope gate: conditioning on the shared envelope must lower the spurious TE, got cond {} uncond {}",
            te_c,
            te_u
        );
        assert!(
            te_c <= thr_c,
            "flare-envelope gate: lagged residual null must not leak on the impulsive envelope (cond {} over thr {})",
            te_c,
            thr_c
        );
    }

    #[test]
    fn flare_envelope_conditional_keeps_true_coupling() {
        let n = 240;
        let mut rng = 0x6A2B_7A5B_3C1D_9E4Fu64;
        let noise = |rng: &mut u64| -> f32 {
            *rng = rng
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            (((*rng >> 33) as f64) / ((u32::MAX >> 1) as f64)) as f32
        };
        let c = flare_envelope(n, &[30usize, 150usize], 1.0, 12.0);
        let mut x = vec![0f32; n];
        let mut y = vec![0f32; n];
        let mut y_ind = vec![0f32; n];
        let alpha = 0.90f32;
        for t in 0..n {
            let ny = noise(&mut rng);
            y_ind[t] = ny;
            x[t] = c[t] + 0.4 * noise(&mut rng);
            y[t] = if t == 0 {
                0.0
            } else {
                alpha * y[t - 1] + (1.0 - alpha) * c[t - 1] + 0.3 * ny
            };
        }
        for t in 0..n - 1 {
            x[t + 1] += 0.6 * y_ind[t];
        }
        let te_c = transfer_entropy_conditional(&x, &y, &c, 1).expect("conditional TE resolves");
        let (_, _, thr_c) =
            conditional_te_stats_lagged(&x, &y, &c, 1, 1, 0x9E37_79B9_7F4A_7C15, 10)
                .expect("lagged conditional null resolves");
        assert!(
            te_c > thr_c,
            "flare-envelope gate: true coupling beyond the shared envelope must survive conditioning, got cond {} thr {}",
            te_c,
            thr_c
        );
    }

    #[test]
    fn synthetic_dag_recovers_known_direction() {
        let n = 240;
        let mut rng = 0x9E4F_6A2B_7A5B_3C1Du64;
        let noise = |rng: &mut u64| -> f32 {
            *rng = rng
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            (((*rng >> 33) as f64) / ((u32::MAX >> 1) as f64)) as f32
        };
        let z = flare_envelope(n, &[30usize, 150usize], 1.0, 12.0);
        let mut a = vec![0f32; n];
        let mut b = vec![0f32; n];
        let mut a_ind = vec![0f32; n];
        let alpha = 0.90f32;
        for t in 0..n {
            a_ind[t] = noise(&mut rng);
            a[t] = z[t] + 0.4 * a_ind[t];
            b[t] = if t == 0 {
                0.0
            } else {
                alpha * b[t - 1] + (1.0 - alpha) * z[t - 1] + 0.3 * noise(&mut rng)
            };
        }
        for t in 0..n - 1 {
            b[t + 1] += 0.5 * a_ind[t];
        }
        let seed = 0x9E37_79B9_7F4A_7C15;
        let te_ab = transfer_entropy_conditional(&b, &a, &z, 1).expect("A->B resolves");
        let te_ba = transfer_entropy_conditional(&a, &b, &z, 1).expect("B->A resolves");
        let (_, _, thr_ab) =
            conditional_te_stats_lagged(&b, &a, &z, 1, 1, seed, 10).expect("null A->B resolves");
        let (_, _, thr_ba) =
            conditional_te_stats_lagged(&a, &b, &z, 1, 1, seed, 10).expect("null B->A resolves");
        assert!(
            te_ab > thr_ab,
            "synthetic DAG: recover the known true edge A->B, got cond {} thr {}",
            te_ab,
            thr_ab
        );
        assert!(
            te_ba <= thr_ba,
            "synthetic DAG: reject the false reverse edge B->A, got cond {} thr {}",
            te_ba,
            thr_ba
        );
    }

    #[test]
    fn conditional_te_2_suppresses_two_drivers() {
        let n = 400;
        let c1: Vec<f32> = (0..n).map(|t| (t as f32 * 0.23).sin()).collect();
        let c2: Vec<f32> = (0..n).map(|t| (t as f32 * 0.11).cos()).collect();
        let mut rng = 0x0FEB_11D1_B2D1_9C93u64;
        let noise = |rng: &mut u64| -> f32 {
            *rng = rng
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            (((*rng >> 33) as f64) / ((u32::MAX >> 1) as f64)) as f32
        };
        let x: Vec<f32> = c1
            .iter()
            .zip(&c2)
            .map(|(&a, &b)| a + b + 0.4 * noise(&mut rng))
            .collect();
        let y: Vec<f32> = c1
            .iter()
            .zip(&c2)
            .map(|(&a, &b)| a + b + 0.4 * noise(&mut rng))
            .collect();
        let te_1 =
            transfer_entropy_conditional(&x, &y, &c1, 1).expect("one-confounder TE resolves");
        let te_2 = transfer_entropy_conditional_2(&x, &y, &c1, &c2, 1)
            .expect("two-confounder TE resolves");
        assert!(
            te_2 < te_1,
            "two-confounder must remove more than one-confounder, got 2:{te_2} >= 1:{te_1}"
        );
        assert!(
            te_2.abs() < 0.05,
            "two-confounder should suppress the shared-driver pair to near zero, got {te_2}"
        );
    }

    #[test]
    fn conditional_te_2_surrogate_stats_threshold_is_finite() {
        let n = 300;
        let c1: Vec<f32> = (0..n).map(|t| (t as f32 * 0.26).sin()).collect();
        let c2: Vec<f32> = (0..n).map(|t| (t as f32 * 0.13).cos()).collect();
        let mut rng = 0x9E37_79B9_7F4A_7C15u64;
        let noise = |rng: &mut u64| -> f32 {
            *rng = rng
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            (((*rng >> 33) as f64) / ((u32::MAX >> 1) as f64)) as f32
        };
        let x: Vec<f32> = c1
            .iter()
            .zip(&c2)
            .map(|(&a, &b)| a + b + 0.4 * noise(&mut rng))
            .collect();
        let y: Vec<f32> = c1
            .iter()
            .zip(&c2)
            .map(|(&a, &b)| a + b + 0.4 * noise(&mut rng))
            .collect();
        let (mean, sd, threshold) =
            conditional_te_stats_lagged_2(&x, &y, &c1, &c2, 1, 1, 0x9E37_79B9_7F4A_7C15, 10)
                .expect("stats resolve");
        assert!(mean.is_finite() && sd.is_finite() && threshold.is_finite());
        assert!(threshold >= mean);
    }

    #[test]
    fn binned_te_causal_positive() {
        let n = 200;
        let mut x = vec![0f32; n];
        let mut y = vec![0f32; n];
        for t in 0..n {
            y[t] = (t as f32 * 0.7).sin();
        }
        for t in 0..n - 1 {
            x[t + 1] = 0.5 * x[t] + 0.6 * y[t];
        }
        let te = transfer_entropy_binned(&x, &y, 1, 4).unwrap();
        assert!(te > 0.02, "binned causal TE should be positive, got {}", te);
    }

    #[test]
    fn binned_te_independent_below_causal() {
        let n = 200;
        let mut rng = 0x2D6B_91A3_77C4_05F1u64;
        let noise = |rng: &mut u64| -> f32 {
            *rng = rng
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            (((*rng >> 33) as f64) / ((u32::MAX >> 1) as f64)) as f32
        };
        let mut xi = vec![0f32; n];
        let mut yi = vec![0f32; n];
        for t in 0..n {
            xi[t] = noise(&mut rng);
            yi[t] = noise(&mut rng);
        }
        let te_indep = transfer_entropy_binned(&xi, &yi, 1, 4).unwrap();
        let mut xc = vec![0f32; n];
        let mut yc = vec![0f32; n];
        for t in 0..n {
            yc[t] = (t as f32 * 0.7).sin();
        }
        for t in 0..n - 1 {
            xc[t + 1] = 0.5 * xc[t] + 0.6 * yc[t];
        }
        let te_causal = transfer_entropy_binned(&xc, &yc, 1, 4).unwrap();
        assert!(
            te_indep < te_causal,
            "independent binned TE must sit below the causal pair (the naive bias is a floor the null absorbs), got indep {te_indep} causal {te_causal}"
        );
    }

    #[test]
    fn binned_conditional_suppresses_two_drivers() {
        let n = 400;
        let c1: Vec<f32> = (0..n).map(|t| (t as f32 * 0.23).sin()).collect();
        let c2: Vec<f32> = (0..n).map(|t| (t as f32 * 0.11).cos()).collect();
        let mut rng = 0x0FEB_11D1_B2D1_9C93u64;
        let noise = |rng: &mut u64| -> f32 {
            *rng = rng
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            (((*rng >> 33) as f64) / ((u32::MAX >> 1) as f64)) as f32
        };
        let x: Vec<f32> = c1
            .iter()
            .zip(&c2)
            .map(|(&a, &b)| a + b + 0.4 * noise(&mut rng))
            .collect();
        let y: Vec<f32> = c1
            .iter()
            .zip(&c2)
            .map(|(&a, &b)| a + b + 0.4 * noise(&mut rng))
            .collect();
        let te_1 = transfer_entropy_conditional_binned_n(&x, &y, &[&c1], 1, 3)
            .expect("one-confounder binned TE resolves");
        let te_2 = transfer_entropy_conditional_binned_n(&x, &y, &[&c1, &c2], 1, 3)
            .expect("two-confounder binned TE resolves");
        assert!(
            te_2 < te_1,
            "two-confounder binned must remove more than one, got 2:{te_2} >= 1:{te_1}"
        );
        assert!(
            te_2.abs() < 0.06,
            "two-confounder binned should suppress the shared-driver pair, got {te_2}"
        );
    }

    #[test]
    fn binned_conditional_recovers_dag_direction() {
        let n = 300;
        let mut rng = 0x9E4F_6A2B_7A5B_3C1Du64;
        let noise = |rng: &mut u64| -> f32 {
            *rng = rng
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            (((*rng >> 33) as f64) / ((u32::MAX >> 1) as f64)) as f32
        };
        let z = flare_envelope(n, &[40usize, 200usize], 1.0, 12.0);
        let mut a = vec![0f32; n];
        let mut b = vec![0f32; n];
        let mut a_ind = vec![0f32; n];
        let alpha = 0.90f32;
        for t in 0..n {
            a_ind[t] = noise(&mut rng);
            a[t] = z[t] + 0.4 * a_ind[t];
            b[t] = if t == 0 {
                0.0
            } else {
                alpha * b[t - 1] + (1.0 - alpha) * z[t - 1] + 0.3 * noise(&mut rng)
            };
        }
        for t in 0..n - 1 {
            b[t + 1] += 0.5 * a_ind[t];
        }
        let te_ab = transfer_entropy_conditional_binned_n(&b, &a, &[&z], 1, 4)
            .expect("binned A->B resolves");
        let te_ba = transfer_entropy_conditional_binned_n(&a, &b, &[&z], 1, 4)
            .expect("binned B->A resolves");
        assert!(
            te_ab > te_ba,
            "binned DAG: recover A->B over B->A, got fwd {te_ab} rev {te_ba}"
        );
    }

    #[test]
    fn binned_n_dim_direction_matches_kde() {
        let n = 300;
        let mut rng = 0x3C1D_9E4F_6A2B_7A5Bu64;
        let noise = |rng: &mut u64| -> f32 {
            *rng = rng
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            (((*rng >> 33) as f64) / ((u32::MAX >> 1) as f64)) as f32
        };
        let z = flare_envelope(n, &[40usize, 200usize], 1.0, 12.0);
        let mut a = vec![0f32; n];
        let mut b = vec![0f32; n];
        let mut a_ind = vec![0f32; n];
        let alpha = 0.90f32;
        for t in 0..n {
            a_ind[t] = noise(&mut rng);
            a[t] = z[t] + 0.4 * a_ind[t];
            b[t] = if t == 0 {
                0.0
            } else {
                alpha * b[t - 1] + (1.0 - alpha) * z[t - 1] + 0.3 * noise(&mut rng)
            };
        }
        for t in 0..n - 1 {
            b[t + 1] += 0.5 * a_ind[t];
        }
        let kde_ab = transfer_entropy_conditional(&b, &a, &z, 1).expect("KDE A->B resolves");
        let kde_ba = transfer_entropy_conditional(&a, &b, &z, 1).expect("KDE B->A resolves");
        let bin_ab = transfer_entropy_conditional_binned_n(&b, &a, &[&z], 1, 4)
            .expect("binned A->B resolves");
        let bin_ba = transfer_entropy_conditional_binned_n(&a, &b, &[&z], 1, 4)
            .expect("binned B->A resolves");
        assert!(
            kde_ab > kde_ba,
            "KDE reference must recover the direction, got fwd {kde_ab} rev {kde_ba}"
        );
        assert!(
            bin_ab > bin_ba,
            "binned estimator must agree with the KDE direction, got fwd {bin_ab} rev {bin_ba}"
        );
    }

    #[test]
    fn binned_null_does_not_leak_on_multi_driver_confound() {
        let n = 300;
        let mut rng = 0x5A3C_1D9E_4F6A_2B7Cu64;
        let noise = |rng: &mut u64| -> f32 {
            *rng = rng
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            (((*rng >> 33) as f64) / ((u32::MAX >> 1) as f64)) as f32
        };
        let c1 = flare_envelope(n, &[30usize, 150usize], 1.0, 12.0);
        let c2 = flare_envelope(n, &[60usize, 180usize], 1.0, 30.0);
        let mut x = vec![0f32; n];
        let mut y = vec![0f32; n];
        let alpha = 0.90f32;
        for t in 0..n {
            x[t] = c1[t] + c2[t] + 0.05 * noise(&mut rng);
            y[t] = if t == 0 {
                0.0
            } else {
                alpha * y[t - 1] + (1.0 - alpha) * (c1[t - 1] + c2[t - 1]) + 0.05 * noise(&mut rng)
            };
        }
        let bins = 3;
        let te_c = transfer_entropy_conditional_binned_n(&x, &y, &[&c1, &c2], 1, bins)
            .expect("two-driver binned TE resolves");
        let (_, _, thr) = conditional_te_stats_lagged_n(
            &x,
            &y,
            &[&c1, &c2],
            1,
            1,
            bins,
            0x9E37_79B9_7F4A_7C15,
            10,
            TeNull::Residual,
        )
        .expect("lagged N-dim null resolves");
        assert!(
            te_c <= thr,
            "binned N-dim null must not leak on the multi-driver impulsive confound, got te {te_c} thr {thr}"
        );
    }

    #[test]
    fn binned_null_keeps_true_coupling_under_multi_driver() {
        let n = 300;
        let mut rng = 0x6A2B_7A5B_3C1D_9E4Fu64;
        let noise = |rng: &mut u64| -> f32 {
            *rng = rng
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            (((*rng >> 33) as f64) / ((u32::MAX >> 1) as f64)) as f32
        };
        let c1 = flare_envelope(n, &[30usize, 150usize], 1.0, 12.0);
        let c2 = flare_envelope(n, &[60usize, 180usize], 1.0, 30.0);
        let mut x = vec![0f32; n];
        let mut y = vec![0f32; n];
        let mut x_ind = vec![0f32; n];
        let alpha = 0.90f32;
        for t in 0..n {
            x_ind[t] = noise(&mut rng);
            x[t] = c1[t] + c2[t] + 0.4 * x_ind[t];
            y[t] = if t == 0 {
                0.0
            } else {
                alpha * y[t - 1] + (1.0 - alpha) * (c1[t - 1] + c2[t - 1]) + 0.3 * noise(&mut rng)
            };
        }
        for t in 0..n - 1 {
            y[t + 1] += 0.5 * x_ind[t];
        }
        let bins = 3;
        let te_c = transfer_entropy_conditional_binned_n(&y, &x, &[&c1, &c2], 1, bins)
            .expect("two-driver binned TE resolves");
        let (_, _, thr) = conditional_te_stats_lagged_n(
            &y,
            &x,
            &[&c1, &c2],
            1,
            1,
            bins,
            0x9E37_79B9_7F4A_7C15,
            10,
            TeNull::Residual,
        )
        .expect("lagged N-dim null resolves");
        assert!(
            te_c > thr,
            "binned N-dim null must keep the true coupling beyond the shared drivers, got te {te_c} thr {thr}"
        );
    }

    #[test]
    fn pcmci_recovers_known_dag() {
        let n = 400;
        let mut rng = 0x7A5B_3C1D_9E4F_6A2Bu64;
        let noise = |rng: &mut u64| -> f32 {
            *rng = rng
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            (((*rng >> 33) as f64) / ((u32::MAX >> 1) as f64)) as f32
        };
        let z = flare_envelope(n, &[40usize, 200usize], 1.0, 12.0);
        let mut a = vec![0f32; n];
        let mut b = vec![0f32; n];
        let mut a_ind = vec![0f32; n];
        for t in 0..n {
            a_ind[t] = noise(&mut rng);
        }
        for t in 0..n {
            a[t] = if t == 0 {
                0.4 * a_ind[t]
            } else {
                0.5 * a[t - 1] + 0.6 * z[t - 1] + 0.4 * a_ind[t]
            };
            b[t] = if t == 0 {
                0.3 * noise(&mut rng)
            } else {
                0.9 * b[t - 1] + 0.6 * z[t - 1] + 0.5 * a_ind[t - 1] + 0.3 * noise(&mut rng)
            };
        }
        let series: [&[f32]; 3] = [&z, &a, &b];
        let links = pcmci_links(
            &series,
            1,
            3,
            3,
            0x9E37_79B9_7F4A_7C15,
            10,
            TeNull::Residual,
            0,
            TeEstimator::Binned,
            4,
            2,
            0.05,
        )
        .expect("pcmci resolves");
        let ab = links
            .iter()
            .find(|l| l.driver == 1 && l.target == 2 && l.lag == 1)
            .expect("A->B link present");
        let ba = links
            .iter()
            .find(|l| l.driver == 2 && l.target == 1 && l.lag == 1)
            .expect("B->A link present");
        assert!(
            ab.te > ab.threshold,
            "pcmci must recover the true edge A->B, got te {} thr {}",
            ab.te,
            ab.threshold
        );
        assert!(
            ba.te <= ba.threshold,
            "pcmci must reject the false edge B->A, got te {} thr {}",
            ba.te,
            ba.threshold
        );
    }

    #[test]
    fn benjamini_hochberg_ranks_cutoff() {
        let cutoff = benjamini_hochberg(&[0.001, 0.02, 0.5, 0.8], 0.05).unwrap();
        assert!(
            (cutoff - 0.02).abs() < 1e-12,
            "BH cutoff must be the largest qualifying p (0.02), got {cutoff}"
        );
    }
}
