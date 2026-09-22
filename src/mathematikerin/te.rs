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
        for &xs in x.iter().take(n) {
            k1 += gaussian(xt - xs as f64, hx);
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
        for &xs in x.iter().take(n) {
            k1 += gaussian(xt - xs as f64, hx);
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
        for &xs in x.iter().take(n) {
            k1 += gaussian(xt - xs as f64, hx);
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
    conds: &[LaggedCond],
    lag: usize,
    k: usize,
) -> Option<f64> {
    let n = x.len();
    if n < 8 || y.len() < n || k == 0 {
        return None;
    }
    let lo = max_cond_lag(conds);
    if lo >= n {
        return None;
    }
    for c in conds {
        if c.series.len() < n {
            return None;
        }
    }
    if x.iter().chain(y.iter()).any(|v| !v.is_finite()) {
        return None;
    }
    for c in conds {
        if c.series.iter().any(|v| !v.is_finite()) {
            return None;
        }
    }
    let shift = if lag == 0 { 1usize } else { lag };
    let m = n.checked_sub(shift)?;
    let m_eff = m.checked_sub(lo)?;
    if m_eff < 8 {
        return None;
    }
    let dim = 3 + conds.len();
    let mut pts: Vec<f64> = Vec::with_capacity(m_eff * dim);
    for t in lo..m {
        pts.push(x[t + shift] as f64);
        pts.push(x[t] as f64);
        pts.push(y[t] as f64);
        for c in conds {
            pts.push(c.series[t - c.lag] as f64);
        }
    }
    let k_eff = k.min(m_eff - 1);
    let mut dists: Vec<f64> = Vec::with_capacity(m_eff - 1);
    let mut sum = 0.0f64;
    for i in 0..m_eff {
        dists.clear();
        for j in 0..m_eff {
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
        for j in 0..m_eff {
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
    Some(digamma(k_eff as f64) + sum / m_eff as f64)
}

#[derive(Clone, Copy)]
pub struct LaggedCond<'a> {
    pub series: &'a [f32],
    pub lag: usize,
}

fn max_cond_lag(conds: &[LaggedCond]) -> usize {
    conds.iter().map(|c| c.lag).fold(0, usize::max)
}

pub fn transfer_entropy_conditional_binned_n(
    x: &[f32],
    y: &[f32],
    conds: &[LaggedCond],
    lag: usize,
    bins: usize,
) -> Option<f64> {
    let n = x.len();
    if n < 8 || bins < 2 || y.len() < n {
        return None;
    }
    let lo = max_cond_lag(conds);
    if lo >= n {
        return None;
    }
    for c in conds {
        if c.series.len() < n {
            return None;
        }
    }
    let shift = if lag == 0 { 1usize } else { lag };
    let m = n.checked_sub(shift)?;
    if m.checked_sub(lo)? < 8 {
        return None;
    }
    let (mn_x, mx_x) = bin_edges(x)?;
    let range_x = mx_x - mn_x;
    let (mn_y, mx_y) = bin_edges(y)?;
    let range_y = mx_y - mn_y;
    let mut cond_edges: Vec<(f32, f32)> = Vec::with_capacity(conds.len());
    for c in conds {
        let (mn, mx) = bin_edges(c.series)?;
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
        .map(|(c, &(mn, range))| {
            c.series
                .iter()
                .map(|&v| bin_index(v, mn, range, bins))
                .collect()
        })
        .collect();

    let mut keybuf: Vec<usize> = Vec::with_capacity(conds.len() + 3);
    let mut map_full: std::collections::HashMap<u64, usize> = std::collections::HashMap::new();
    let mut map_xz: std::collections::HashMap<u64, usize> = std::collections::HashMap::new();
    let mut map_xyz: std::collections::HashMap<u64, usize> = std::collections::HashMap::new();
    let mut map_fxz: std::collections::HashMap<u64, usize> = std::collections::HashMap::new();

    for s in lo..m {
        keybuf.clear();
        keybuf.push(bx[s + shift]);
        keybuf.push(bx[s]);
        keybuf.push(by[s]);
        for (b, c) in bcond.iter().zip(conds) {
            keybuf.push(b[s - c.lag]);
        }
        *map_full.entry(joint_key(&keybuf, bins)).or_insert(0) += 1;
    }
    for s in lo..n {
        keybuf.clear();
        keybuf.push(bx[s]);
        for (b, c) in bcond.iter().zip(conds) {
            keybuf.push(b[s - c.lag]);
        }
        *map_xz.entry(joint_key(&keybuf, bins)).or_insert(0) += 1;
    }
    for s in lo..n {
        keybuf.clear();
        keybuf.push(bx[s]);
        keybuf.push(by[s]);
        for (b, c) in bcond.iter().zip(conds) {
            keybuf.push(b[s - c.lag]);
        }
        *map_xyz.entry(joint_key(&keybuf, bins)).or_insert(0) += 1;
    }
    for s in lo..m {
        keybuf.clear();
        keybuf.push(bx[s + shift]);
        keybuf.push(bx[s]);
        for (b, c) in bcond.iter().zip(conds) {
            keybuf.push(b[s - c.lag]);
        }
        *map_fxz.entry(joint_key(&keybuf, bins)).or_insert(0) += 1;
    }

    let mf = (m - lo) as f64;
    let nf = (n - lo) as f64;
    let mut te = 0.0;
    for t in lo..m {
        keybuf.clear();
        keybuf.push(bx[t + shift]);
        keybuf.push(bx[t]);
        keybuf.push(by[t]);
        for (b, c) in bcond.iter().zip(conds) {
            keybuf.push(b[t - c.lag]);
        }
        let p5 = *map_full.get(&joint_key(&keybuf, bins)).unwrap_or(&0) as f64 / mf;
        keybuf.clear();
        keybuf.push(bx[t]);
        for (b, c) in bcond.iter().zip(conds) {
            keybuf.push(b[t - c.lag]);
        }
        let p3 = *map_xz.get(&joint_key(&keybuf, bins)).unwrap_or(&0) as f64 / nf;
        keybuf.clear();
        keybuf.push(bx[t]);
        keybuf.push(by[t]);
        for (b, c) in bcond.iter().zip(conds) {
            keybuf.push(b[t - c.lag]);
        }
        let p4a = *map_xyz.get(&joint_key(&keybuf, bins)).unwrap_or(&0) as f64 / nf;
        keybuf.clear();
        keybuf.push(bx[t + shift]);
        keybuf.push(bx[t]);
        for (b, c) in bcond.iter().zip(conds) {
            keybuf.push(b[t - c.lag]);
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

pub fn conditional_te_stats_lagged(
    x: &[f32],
    y: &[f32],
    c: &[f32],
    lag: usize,
    max_lag: usize,
    seed: u64,
    n_surr: usize,
) -> Option<(f64, f64, f64)> {
    let conds = [LaggedCond { series: c, lag: 0 }];
    let mut vals: Vec<f64> = Vec::with_capacity(n_surr);
    let mut rng = seed.wrapping_add(0x9e3779b97f4a7c15);
    for _ in 0..n_surr {
        let Some(ys) = arx_conditional_surrogate(y, x, &conds, max_lag, &mut rng) else {
            continue;
        };
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

#[derive(Clone, Copy)]
pub struct TeStats2Params {
    pub lag: usize,
    pub max_lag: usize,
    pub seed: u64,
    pub n_surr: usize,
}

pub fn conditional_te_stats_lagged_2(
    x: &[f32],
    y: &[f32],
    c1: &[f32],
    c2: &[f32],
    p: TeStats2Params,
) -> Option<(f64, f64, f64)> {
    let TeStats2Params {
        lag,
        max_lag,
        seed,
        n_surr,
    } = p;
    let conds = [
        LaggedCond { series: c1, lag: 0 },
        LaggedCond { series: c2, lag: 0 },
    ];
    let mut vals: Vec<f64> = Vec::with_capacity(n_surr);
    let mut rng = seed.wrapping_add(0x9e3779b97f4a7c15);
    for _ in 0..n_surr {
        let Some(ys) = arx_conditional_surrogate(y, x, &conds, max_lag, &mut rng) else {
            continue;
        };
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

fn arx_null_cond_series<'a>(conds: &[LaggedCond<'a>]) -> Vec<&'a [f32]> {
    let mut groups: Vec<&'a [f32]> = Vec::with_capacity(conds.len());
    for c in conds {
        if groups
            .iter()
            .all(|g| !std::ptr::eq(g.as_ptr(), c.series.as_ptr()))
        {
            groups.push(c.series);
        }
    }
    groups
}

fn ols_fit_lagged_nx(
    y: &[f32],
    conds: &[LaggedCond],
    x: &[f32],
    max_lag: usize,
) -> Option<Vec<f64>> {
    let n = y.len();
    if x.len() != n {
        return None;
    }
    for c in conds {
        if c.series.len() != n {
            return None;
        }
    }
    assert!(
        max_cond_lag(conds) <= max_lag,
        "a cond column beyond the null window is outside the superset — the fit covers lags 0..=max_lag per distinct series"
    );
    let t0 = max_lag;
    if n < t0 + 4 {
        return None;
    }
    let groups = arx_null_cond_series(conds);
    let n_cond = groups.len();
    let k = 1 + max_lag + n_cond * (max_lag + 1) + max_lag;
    let mut a = vec![0f64; k * k];
    let mut b = vec![0f64; k];
    for t in t0..n {
        let mut row = Vec::with_capacity(k);
        row.push(1.0);
        for l in 1..=max_lag {
            row.push(y[t - l] as f64);
        }
        for c in &groups {
            for l in 0..=max_lag {
                row.push(c[t - l] as f64);
            }
        }
        for l in 1..=max_lag {
            row.push(x[t - l] as f64);
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

fn lagged_predict_nx(
    coeffs: &[f64],
    y: &[f32],
    groups: &[&[f32]],
    x: &[f32],
    t: usize,
    max_lag: usize,
) -> f64 {
    let mut v = coeffs[0];
    for l in 1..=max_lag {
        v += coeffs[l] * y[t - l] as f64;
    }
    let cbase = 1 + max_lag;
    for (ci, c) in groups.iter().enumerate() {
        let base = cbase + ci * (max_lag + 1);
        for l in 0..=max_lag {
            v += coeffs[base + l] * c[t - l] as f64;
        }
    }
    let xbase = cbase + groups.len() * (max_lag + 1);
    for l in 1..=max_lag {
        v += coeffs[xbase + l - 1] * x[t - l] as f64;
    }
    v
}

pub fn arx_restricted_surrogate(y: &[f32], order: usize, rng: &mut u64) -> Option<Vec<f32>> {
    let n = y.len();
    let p = order;
    match ols_fit_lagged_n(y, &[], p) {
        Some(coeffs) => {
            let resid: Vec<f64> = (p..n)
                .map(|t| y[t] as f64 - lagged_predict_n(&coeffs, y, &[], t, p))
                .collect();
            let resid_f32: Vec<f32> = resid.iter().map(|&v| v as f32).collect();
            let perm = restricted_permutation_surrogate(&resid_f32, rng);
            let mut out = vec![0f32; n];
            for t in 0..n {
                out[t] = if t < p {
                    y[t]
                } else {
                    let v = lagged_predict_n(&coeffs, &out, &[], t, p) + perm[t - p] as f64;
                    if !v.is_finite() {
                        return None;
                    }
                    v as f32
                };
            }
            Some(out)
        }
        None => None,
    }
}

pub fn arx_conditional_surrogate(
    y: &[f32],
    x: &[f32],
    conds: &[LaggedCond],
    max_lag: usize,
    rng: &mut u64,
) -> Option<Vec<f32>> {
    if conds.is_empty() {
        return arx_restricted_surrogate(y, max_lag, rng);
    }
    let n = y.len();
    if x.len() != n {
        return None;
    }
    assert!(
        max_cond_lag(conds) <= max_lag,
        "a cond column beyond the null window is outside the superset — the null re-simulation covers lags 0..=max_lag per distinct series"
    );
    let t0 = max_lag;
    let groups = arx_null_cond_series(conds);
    match ols_fit_lagged_nx(y, conds, x, max_lag) {
        Some(coeffs) => {
            let resid: Vec<f64> = (t0..n)
                .map(|t| y[t] as f64 - lagged_predict_nx(&coeffs, y, &groups, x, t, max_lag))
                .collect();
            let resid_f32: Vec<f32> = resid.iter().map(|&v| v as f32).collect();
            let perm = restricted_permutation_surrogate(&resid_f32, rng);
            let mut out = vec![0f32; n];
            for t in 0..n {
                out[t] = if t < t0 {
                    y[t]
                } else {
                    let v = lagged_predict_nx(&coeffs, &out, &groups, x, t, max_lag)
                        + perm[t - t0] as f64;
                    if !v.is_finite() {
                        return None;
                    }
                    v as f32
                };
            }
            Some(out)
        }
        None => None,
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TeNull {
    Block,
    Shift,
    Phase,
    RestrictedPermutation,
    XShift,
    Arx,
    CoherentPhase,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TeEstimator {
    Binned,
    Ksg,
}

pub fn block_len_from_n(n: usize) -> usize {
    (n as f64).powf(1.0 / 3.0).round() as usize
}

#[derive(Clone, Copy)]
pub struct TeStatsParams {
    pub lag: usize,
    pub max_lag: usize,
    pub bins: usize,
    pub seed: u64,
    pub n_surr: usize,
    pub null: TeNull,
}

pub fn conditional_te_stats_lagged_n(
    x: &[f32],
    y: &[f32],
    conds: &[LaggedCond],
    p: TeStatsParams,
) -> Option<(f64, f64, f64)> {
    let TeStatsParams {
        lag,
        max_lag,
        bins,
        seed,
        n_surr,
        null,
    } = p;
    let vals = conditional_te_surrogates_n(
        x,
        y,
        conds,
        TeSurrogateParams {
            lag,
            max_lag,
            bins,
            seed,
            n_surr,
            null,
            block: 0,
            est: TeEstimator::Binned,
            k: 4,
        },
    )?;
    let n = vals.len() as f64;
    let mean = vals.iter().sum::<f64>() / n;
    let var = vals.iter().map(|&v| (v - mean) * (v - mean)).sum::<f64>() / n;
    let sd = var.sqrt();
    Some((mean, sd, mean + 2.0 * sd))
}

#[derive(Clone, Copy)]
pub struct TeSurrogateParams {
    pub lag: usize,
    pub max_lag: usize,
    pub bins: usize,
    pub seed: u64,
    pub n_surr: usize,
    pub null: TeNull,
    pub block: usize,
    pub est: TeEstimator,
    pub k: usize,
}

pub fn conditional_te_surrogates_n(
    x: &[f32],
    y: &[f32],
    conds: &[LaggedCond],
    p: TeSurrogateParams,
) -> Option<Vec<f64>> {
    let TeSurrogateParams {
        lag,
        max_lag,
        bins,
        seed,
        n_surr,
        null,
        block,
        est,
        k,
    } = p;
    let mut vals: Vec<f64> = Vec::with_capacity(n_surr);
    let mut rng = seed.wrapping_add(0x9e3779b97f4a7c15);
    let block_len = if block == 0 {
        block_len_from_n(y.len())
    } else {
        block
    };
    for _ in 0..n_surr {
        let (xs_buf, ys) = match null {
            TeNull::CoherentPhase => {
                let mut both = coherent_phase_surrogates(&[x, y], &mut rng);
                let ys = both
                    .pop()
                    .expect("coherent phase yields one series per input");
                let xs = both
                    .pop()
                    .expect("coherent phase yields one series per input");
                (Some(xs), ys)
            }
            TeNull::XShift => (Some(x_shift_surrogate(x, &mut rng)), y.to_vec()),
            TeNull::Block => (None, block_bootstrap_surrogate(y, block_len, &mut rng)),
            TeNull::Shift => (None, cycle_phase_shift_surrogate(y, y.len(), &mut rng)),
            TeNull::Phase => (None, phase_randomized_surrogate(y, &mut rng)),
            TeNull::RestrictedPermutation => (None, restricted_permutation_surrogate(y, &mut rng)),
            TeNull::Arx => {
                let Some(s) = arx_conditional_surrogate(y, x, conds, max_lag, &mut rng) else {
                    continue;
                };
                (None, s)
            }
        };
        let xs: &[f32] = xs_buf.as_deref().unwrap_or(x);
        let te = match est {
            TeEstimator::Binned => transfer_entropy_conditional_binned_n(xs, &ys, conds, lag, bins),
            TeEstimator::Ksg => transfer_entropy_ksg_conditional_n(xs, &ys, conds, lag, k),
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

#[derive(Clone, Copy)]
pub struct PcmciParams {
    pub max_lag: usize,
    pub null_lag: usize,
    pub bins: usize,
    pub seed: u64,
    pub n_surr: usize,
    pub null: TeNull,
    pub block: usize,
    pub est: TeEstimator,
    pub k: usize,
    pub p_max: usize,
    pub alpha: f64,
}

pub fn pcmci_links(series: &[&[f32]], p: PcmciParams) -> Option<Vec<CausalLink>> {
    let PcmciParams {
        max_lag,
        null_lag,
        bins,
        seed,
        n_surr,
        null,
        block,
        est,
        k,
        p_max,
        alpha,
    } = p;
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
                conds: &[LaggedCond],
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
            series[j],
            series[i],
            conds,
            TeSurrogateParams {
                lag,
                max_lag: null_lag,
                bins,
                seed: seed_t,
                n_surr,
                null,
                block,
                est,
                k,
            },
        )?;
        let b = surr.len();
        let rank = 1 + surr.iter().filter(|&&s| s >= te).count();
        let p_value = rank as f64 / (b + 1) as f64;
        let mut all = surr.clone();
        all.push(te);
        all.sort_by(|a, b| a.total_cmp(b));
        let k = (alpha * (b + 1) as f64).floor() as usize;
        let idx = all.len().saturating_sub(k + 1);
        let threshold = all[idx];
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
                        if let Some((te, threshold, _pv)) = test(j, i, lag, &[], seed_t)
                            && te > threshold
                        {
                            parents[j].push((i, lag));
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
                    let conds: Vec<LaggedCond> = comb
                        .iter()
                        .filter(|&&(d, _)| d != i && d != j)
                        .map(|&(d, l)| LaggedCond {
                            series: series[d],
                            lag: l,
                        })
                        .collect();
                    let seed_t = seed
                        ^ (j as u64).wrapping_mul(0x9E37_79B9)
                        ^ (i as u64).wrapping_mul(0x85EB_CA6B)
                        ^ (lag as u64).wrapping_mul(0xC2B2_AE3D)
                        ^ (p as u64).wrapping_mul(0xD1B5_4A32)
                        ^ (ci as u64).wrapping_mul(0x4A32_D1B5);
                    if let Some((te, threshold, _pv)) = test(j, i, lag, &conds, seed_t)
                        && te <= threshold
                    {
                        removed_here = true;
                        break;
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
                let conds: Vec<LaggedCond> = cond_specs
                    .iter()
                    .filter(|&&(d, _)| d != i && d != j)
                    .map(|&(d, l)| LaggedCond {
                        series: series[d],
                        lag: l,
                    })
                    .collect();
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
    surrogate_stats_phase_n(x, y, lag, seed, 10)
}

pub fn surrogate_stats_phase_n(
    x: &[f32],
    y: &[f32],
    lag: usize,
    seed: u64,
    n_surr: usize,
) -> Option<(f64, f64, f64)> {
    surrogate_stats_with(x, y, lag, seed, n_surr, &mut |v, rng| {
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
    surrogate_stats_block_n(x, y, lag, block, seed, 10)
}

pub fn surrogate_stats_block_n(
    x: &[f32],
    y: &[f32],
    lag: usize,
    block: usize,
    seed: u64,
    n_surr: usize,
) -> Option<(f64, f64, f64)> {
    surrogate_stats_with(x, y, lag, seed, n_surr, &mut |v, rng| {
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

pub fn coherent_phase_surrogates(vs: &[&[f32]], rng: &mut u64) -> Vec<Vec<f32>> {
    let Some(n) = vs.iter().map(|v| v.len()).min() else {
        return Vec::new();
    };
    if n < 2 {
        return vs.iter().map(|v| v.to_vec()).collect();
    }
    let m = n.next_power_of_two();
    let mut phases: Vec<f64> = vec![0.0; m];
    for phase in phases.iter_mut().take(m / 2).skip(1) {
        *phase = next_rng(rng) * 2.0 * std::f64::consts::PI;
    }
    vs.iter()
        .map(|v| {
            let mut re: Vec<f64> = vec![0.0; m];
            let mut im: Vec<f64> = vec![0.0; m];
            for (i, &x) in v.iter().enumerate().take(n) {
                re[i] = x as f64;
            }
            fft(&mut re, &mut im, false);
            for k in 1..m / 2 {
                let (s, c) = phases[k].sin_cos();
                let (ar, ai) = (re[k], im[k]);
                re[k] = ar * c - ai * s;
                im[k] = ar * s + ai * c;
                let j = m - k;
                re[j] = re[k];
                im[j] = -im[k];
            }
            fft(&mut re, &mut im, true);
            (0..n).map(|i| re[i] as f32).collect()
        })
        .collect()
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

pub fn x_shift_surrogate(v: &[f32], rng: &mut u64) -> Vec<f32> {
    let n = v.len();
    if n < 2 {
        return v.to_vec();
    }
    let s = (next_rng(rng) * n as f64) as usize % n;
    let mut out = v.to_vec();
    out.rotate_right(s);
    out
}

const RESTRICTED_PERMUTATION_BINS: usize = 32;

pub fn restricted_permutation_surrogate(v: &[f32], rng: &mut u64) -> Vec<f32> {
    let n = v.len();
    if n < 2 {
        return v.to_vec();
    }
    let mut min = f32::INFINITY;
    let mut max = f32::NEG_INFINITY;
    for &x in v {
        if x < min {
            min = x;
        }
        if x > max {
            max = x;
        }
    }
    let k = RESTRICTED_PERMUTATION_BINS;
    let range = (max - min) as f64;
    let mut bins: Vec<Vec<usize>> = vec![Vec::new(); k];
    for t in 1..n {
        let b = if range <= 0.0 {
            0usize
        } else {
            (((v[t - 1] - min) as f64 / range) * k as f64) as usize
        };
        bins[b.min(k - 1)].push(t);
    }
    let mut out = v.to_vec();
    for pos in &bins {
        let mut vals: Vec<f32> = pos.iter().map(|&t| v[t]).collect();
        for i in (1..vals.len()).rev() {
            let j = (next_rng(rng) * (i as f64 + 1.0)) as usize;
            vals.swap(i, j);
        }
        for (i, &t) in pos.iter().enumerate() {
            out[t] = vals[i];
        }
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
        for &v in series.iter().take(w) {
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
    let dim = {
        let s = emb.first()?;
        s.len()
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
    for m in &mut mean {
        *m /= n;
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

pub struct Betti0Verdict {
    pub tau: usize,
    pub dim: usize,
    pub ladder: Vec<f64>,
    pub betti0: Vec<usize>,
    pub deaths: Vec<f64>,
    pub persistence: f64,
}

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    components: usize,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            rank: vec![0; n],
            components: n,
        }
    }

    fn find(&mut self, mut x: usize) -> usize {
        while self.parent[x] != x {
            self.parent[x] = self.parent[self.parent[x]];
            x = self.parent[x];
        }
        x
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let ra = self.find(a);
        let rb = self.find(b);
        if ra == rb {
            return false;
        }
        if self.rank[ra] < self.rank[rb] {
            self.parent[ra] = rb;
        } else if self.rank[ra] > self.rank[rb] {
            self.parent[rb] = ra;
        } else {
            self.parent[rb] = ra;
            self.rank[ra] += 1;
        }
        self.components -= 1;
        true
    }
}

pub fn betti0_persistence(series: &[f64], dim: usize) -> Option<Betti0Verdict> {
    if dim < 2 || series.iter().any(|v| !v.is_finite()) {
        return None;
    }
    let tau = find_mi_lag(series)?;
    let emb = embed_series(series, tau, dim);
    if emb.len() < 16 {
        return None;
    }
    let n = emb.len();
    let mut edges: Vec<(f64, usize, usize)> = Vec::with_capacity(n * (n - 1) / 2);
    for i in 0..n {
        for j in (i + 1)..n {
            let d = state_distance(&emb[i], &emb[j]);
            if d.is_finite() {
                edges.push((d, i, j));
            }
        }
    }
    if edges.is_empty() {
        return None;
    }
    edges.sort_unstable_by(|a, b| a.0.total_cmp(&b.0));
    let d_min = edges.iter().map(|e| e.0).find(|&d| d > 0.0)?;
    let d_max = edges[edges.len() - 1].0;
    let mut ladder = Vec::new();
    let mut k = 0u32;
    loop {
        let thr = d_min * 2.0f64.powi(k as i32);
        ladder.push(thr);
        if thr >= d_max || k >= 63 {
            break;
        }
        k += 1;
    }
    let mut uf = UnionFind::new(n);
    let mut deaths = Vec::with_capacity(n - 1);
    let mut betti0 = Vec::with_capacity(ladder.len());
    let mut ei = 0usize;
    for &thr in &ladder {
        while ei < edges.len() && edges[ei].0 <= thr {
            let (d, a, b) = edges[ei];
            if uf.union(a, b) {
                deaths.push(d);
            }
            ei += 1;
        }
        betti0.push(uf.components);
    }
    let persistence = match deaths.last() {
        Some(&d) => d / d_max,
        None => return None,
    };
    Some(Betti0Verdict {
        tau,
        dim,
        ladder,
        betti0,
        deaths,
        persistence,
    })
}

const TE_KSG_K: usize = 4;

pub fn transfer_entropy_embedded_kde(
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
    let dim = {
        let s = emb_x.first()?;
        s.len()
    };
    if dim < 2 {
        return None;
    }
    if emb_y.first().is_none_or(|s| s.len() != dim) {
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
    let t_high = n.checked_sub(tau_x + 1)?;
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

pub fn transfer_entropy_embedded_ksg(
    x: &[f64],
    emb_x: &[Vec<f64>],
    emb_y: &[Vec<f64>],
    tau_x: usize,
    tau_y: usize,
    k: usize,
) -> Option<f64> {
    let n = x.len();
    if n < 8 || tau_x == 0 || tau_y == 0 || k == 0 || x.iter().any(|v| !v.is_finite()) {
        return None;
    }
    let dim = {
        let s = emb_x.first()?;
        s.len()
    };
    if dim < 2 {
        return None;
    }
    if emb_y.first().is_none_or(|s| s.len() != dim) {
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
    let t_high = n.checked_sub(tau_x + 1)?;
    if t_low > t_high {
        return None;
    }
    let m = t_high - t_low + 1;
    if m < 8 {
        return None;
    }
    let jd = 1 + 2 * dim;
    let mut pts: Vec<f64> = Vec::with_capacity(m * jd);
    for t in t_low..=t_high {
        pts.push(x[t + tau_x]);
        pts.extend_from_slice(&emb_x[t - back_x]);
        pts.extend_from_slice(&emb_y[t - back_y]);
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
            for di in 0..jd {
                let dd = (pts[j * jd + di] - pts[i * jd + di]).abs();
                if dd > d {
                    d = dd;
                }
            }
            dists.push(d);
        }
        let eps = *dists
            .select_nth_unstable_by(k_eff - 1, |a, b| a.total_cmp(b))
            .1;
        let mut n_x = 0usize;
        let mut n_xx = 0usize;
        let mut n_xy = 0usize;
        for j in 0..m {
            if j == i {
                continue;
            }
            let fut = (pts[j * jd] - pts[i * jd]).abs() < eps;
            let sx = (0..dim).all(|di| (pts[j * jd + 1 + di] - pts[i * jd + 1 + di]).abs() < eps);
            let sy = (0..dim)
                .all(|di| (pts[j * jd + 1 + dim + di] - pts[i * jd + 1 + dim + di]).abs() < eps);
            if fut && sx {
                n_x += 1;
            }
            if sx {
                n_xx += 1;
            }
            if sx && sy {
                n_xy += 1;
            }
        }
        sum += digamma((n_xx + 1) as f64) - digamma((n_x + 1) as f64) - digamma((n_xy + 1) as f64);
    }
    Some(digamma(k_eff as f64) + sum / m as f64)
}

fn lgamma_lanczos(x: f64) -> f64 {
    if x < 0.5 {
        std::f64::consts::PI.ln() - (std::f64::consts::PI * x).sin().ln() - lgamma_lanczos(1.0 - x)
    } else {
        const COF: [f64; 6] = [
            76.18009172947146,
            -86.50532032941677,
            24.01409824083091,
            -1.231739572450155,
            0.1208650973866179e-2,
            -0.5395239384953e-5,
        ];
        let mut y = x;
        let mut tmp = x + 5.5;
        tmp -= (x + 0.5) * tmp.ln();
        let mut ser = 1.000000000190015;
        for &cof in &COF {
            y += 1.0;
            ser += cof / y;
        }
        -tmp + (2.5066282746310005 * ser / x).ln()
    }
}

fn log_factorial(n: u64) -> f64 {
    lgamma_lanczos(n as f64 + 1.0)
}

fn log_choose(n: u64, k: usize) -> f64 {
    let n = n as f64;
    let k = k as f64;
    lgamma_lanczos(n + 1.0) - lgamma_lanczos(k + 1.0) - lgamma_lanczos(n - k + 1.0)
}

fn sorted_log_sum(counts: &std::collections::HashMap<u64, u64>, f: impl Fn(u64) -> f64) -> f64 {
    let mut v: Vec<f64> = counts.values().map(|&n| f(n)).collect();
    v.sort_by(|a, b| a.total_cmp(b));
    v.iter().sum()
}

fn quantile_edges(v: &[f64], c: usize) -> Vec<f64> {
    let n = v.len();
    let mut s = v.to_vec();
    s.sort_by(|a, b| a.total_cmp(b));
    (1..c).map(|j| s[j * n / c]).collect()
}

fn bin_of(v: f64, edges: &[f64]) -> usize {
    edges.iter().filter(|&&e| e < v).count()
}

pub fn transfer_entropy_reduced_normalized(
    xs: &[f64],
    ys: &[f64],
    k: usize,
    l: usize,
    c: usize,
) -> Option<f64> {
    if c < 2 || k == 0 || l == 0 {
        return None;
    }
    let n = xs.len();
    if n == 0 || ys.len() != n {
        return None;
    }
    if xs.iter().chain(ys.iter()).any(|v| !v.is_finite()) {
        return None;
    }
    if xs.iter().all(|&v| v == xs[0]) || ys.iter().all(|&v| v == ys[0]) {
        return None;
    }
    let mint = k.max(l);
    if n <= mint {
        return None;
    }
    let n_pairs = n - mint;
    let c_l = c.checked_pow(l as u32)?;
    let c_k = c.checked_pow(k as u32)?;
    if n_pairs < c_l.checked_mul(c_k)? {
        return None;
    }
    let edges_x = quantile_edges(xs, c);
    let edges_y = quantile_edges(ys, c);
    let mut n123: std::collections::HashMap<u64, u64> = std::collections::HashMap::new();
    let mut n12: std::collections::HashMap<u64, u64> = std::collections::HashMap::new();
    let mut n23: std::collections::HashMap<u64, u64> = std::collections::HashMap::new();
    let mut n2: std::collections::HashMap<u64, u64> = std::collections::HashMap::new();
    for f in mint..n {
        let q = bin_of(ys[f], &edges_y);
        let mut r = 0usize;
        for j in 0..l {
            r = r * c + bin_of(ys[f - 1 - j], &edges_y);
        }
        let mut s = 0usize;
        for j in 0..k {
            s = s * c + bin_of(xs[f - 1 - j], &edges_x);
        }
        *n123.entry(((q * c_l + r) * c_k + s) as u64).or_insert(0) += 1;
        *n12.entry((q * c_l + r) as u64).or_insert(0) += 1;
        *n23.entry((r * c_k + s) as u64).or_insert(0) += 1;
        *n2.entry(r as u64).or_insert(0) += 1;
    }
    let s_full = sorted_log_sum(&n123, log_factorial);
    let s_y = sorted_log_sum(&n2, log_factorial);
    let s_fy = sorted_log_sum(&n12, log_factorial);
    let s_yx = sorted_log_sum(&n23, log_factorial);
    let multiset = |v: u64| log_choose(v + c as u64 - 1, c - 1);
    let corr = sorted_log_sum(&n2, multiset) - sorted_log_sum(&n23, multiset);
    let num = s_full + s_y - s_fy - s_yx + corr;
    let denom = if num > 0.0 { s_y - s_fy + corr } else { -corr };
    if !denom.is_finite() || denom <= 0.0 {
        return None;
    }
    Some(num / denom)
}

pub fn reduced_te_flow(xs: &[f64], ys: &[f64], k: usize, l: usize, c: usize) -> Option<f64> {
    let r = transfer_entropy_reduced_normalized(xs, ys, k, l, c)?;
    (r.is_finite() && r > 0.0).then_some(r)
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

pub struct TopologicalEstimate {
    pub te: f64,
    pub tau_x: usize,
    pub tau_y: usize,
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
    let estimate = topological_te_estimate(x, y, dim)?;
    let xf: Vec<f64> = x.iter().map(|&v| v as f64).collect();
    let yf: Vec<f64> = y.iter().map(|&v| v as f64).collect();
    let emb_x = embed_series(&xf, estimate.tau_x, dim);
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
        if let Some(te_s) =
            transfer_entropy_embedded_ksg(&xf, &emb_x, &emb_s, estimate.tau_x, tau_s, TE_KSG_K)
        {
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
        tau_x: estimate.tau_x,
        tau_y: estimate.tau_y,
        te: estimate.te,
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

pub fn topological_te_estimate(x: &[f32], y: &[f32], dim: usize) -> Option<TopologicalEstimate> {
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
    let te = transfer_entropy_embedded_ksg(&xf, &emb_x, &emb_y, tau_x, tau_y, TE_KSG_K)?;
    Some(TopologicalEstimate { te, tau_x, tau_y })
}

pub fn topological_te_estimate_frozen(
    x: &[f32],
    y: &[f32],
    dim: usize,
    tau_x: usize,
    tau_y: usize,
) -> Option<TopologicalEstimate> {
    let n = x.len();
    if n < 8 || y.len() != n || dim < 2 || tau_x == 0 || tau_y == 0 {
        return None;
    }
    let xf: Vec<f64> = x.iter().map(|&v| v as f64).collect();
    let yf: Vec<f64> = y.iter().map(|&v| v as f64).collect();
    if xf.iter().chain(yf.iter()).any(|v| !v.is_finite()) {
        return None;
    }
    let emb_x = embed_series(&xf, tau_x, dim);
    let emb_y = embed_series(&yf, tau_y, dim);
    if emb_x.is_empty() || emb_y.is_empty() {
        return None;
    }
    let te = transfer_entropy_embedded_ksg(&xf, &emb_x, &emb_y, tau_x, tau_y, TE_KSG_K)?;
    Some(TopologicalEstimate { te, tau_x, tau_y })
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
    let te = transfer_entropy_embedded_kde(&xf, &emb_x, &emb_y, tau_x, tau_y)?;
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
        if let Some(te_s) = transfer_entropy_embedded_kde(&xf, &emb_x, &emb_s, tau_x, tau_s) {
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

fn gate_rng(rng: &mut u64) -> f64 {
    *rng = rng
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);
    ((*rng >> 33) as f64) / ((u32::MAX >> 1) as f64)
}

fn gate_gauss(rng: &mut u64) -> f32 {
    loop {
        let u1 = gate_rng(rng) * 2.0 - 1.0;
        let u2 = gate_rng(rng) * 2.0 - 1.0;
        let s = u1 * u1 + u2 * u2;
        if s >= 1.0 || s <= 0.0 {
            continue;
        }
        let m = (-2.0 * s.ln() / s).sqrt() as f32;
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

pub struct GateCell {
    pub a: f32,
    pub d_z: usize,
    pub fp: usize,
    pub neg: usize,
    pub tp: usize,
}

#[derive(Clone, Copy)]
pub struct GateParams {
    pub null: TeNull,
    pub est: TeEstimator,
    pub max_lag: usize,
    pub null_lag: usize,
    pub bins: usize,
    pub block: usize,
    pub n_surr: usize,
}

fn gate_fpr_cells_from(n: usize, cells: &[(f32, usize, usize)], p: GateParams) -> Vec<GateCell> {
    let GateParams {
        null,
        est,
        max_lag,
        null_lag,
        bins,
        block,
        n_surr,
    } = p;
    let mut rng = 0xC2B2_AE3D_85EB_CA6Bu64;
    let mut out = Vec::with_capacity(cells.len());
    for &(a, d_z, trials) in cells {
        let mut fp = 0usize;
        let mut neg = 0usize;
        let mut tp = 0usize;
        for t in 0..trials {
            let seed = 0x9E37_79B9_7F4A_7C15 ^ (t as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15);
            let series = gate_common_driver(n, a, 0.0, d_z, &mut rng);
            let refs: Vec<&[f32]> = series.iter().map(|s| s.as_slice()).collect();
            let Some(links) = pcmci_links(
                &refs,
                PcmciParams {
                    max_lag,
                    null_lag,
                    bins,
                    seed,
                    n_surr,
                    null,
                    block,
                    est,
                    k: 4,
                    p_max: 2,
                    alpha: 0.05,
                },
            ) else {
                continue;
            };
            let n_chan = refs.len();
            for drv in 0..n_chan {
                for tgt in 0..n_chan {
                    if drv == tgt {
                        continue;
                    }
                    for lag in 1..=max_lag {
                        let z_to_y = tgt == 1 && drv >= 2 && drv < 2 + d_z;
                        if z_to_y {
                            if lag == 1 {
                                tp += 1;
                            }
                            continue;
                        }
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
        out.push(GateCell {
            a,
            d_z,
            fp,
            neg,
            tp,
        });
    }
    out
}

pub fn gate_fpr_cells(n: usize, p: GateParams) -> Vec<GateCell> {
    let cells = [
        (0.0f32, 0usize, 100usize),
        (0.5f32, 0usize, 100usize),
        (0.9f32, 0usize, 100usize),
        (0.0f32, 4usize, 7usize),
        (0.5f32, 4usize, 7usize),
        (0.9f32, 4usize, 7usize),
    ];
    gate_fpr_cells_from(n, &cells, p)
}

#[cfg(test)]
fn gate_fpr_coarse_cells(n: usize, p: GateParams) -> Vec<GateCell> {
    let cells = [
        (0.0f32, 4usize, 7usize),
        (0.5f32, 4usize, 7usize),
        (0.9f32, 4usize, 7usize),
    ];
    gate_fpr_cells_from(n, &cells, p)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transfer_entropy_causal_positive() {
        let n = 200;
        let mut x = vec![0f32; n];
        let mut y = vec![0f32; n];
        for (t, yt) in y.iter_mut().enumerate() {
            *yt = (t as f32 * 0.7).sin();
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
        for (t, yt) in y.iter_mut().enumerate() {
            *yt = (t as f32 * 0.7).sin();
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
        for (t, yt) in y.iter_mut().enumerate() {
            *yt = (t as f32 * 0.7).sin();
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
        for (t, yt) in y.iter_mut().enumerate() {
            *yt = (t as f32 * 0.7).sin();
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
    fn gate_phase_surrogate_padding_edge() {
        fn deviation(n: usize) -> f64 {
            let x: Vec<f32> = (0..n)
                .map(|t| 0.05 * t as f32 + (t as f32 * 0.13).sin())
                .collect();
            let mut rng = 42u64;
            let s = phase_randomized_surrogate(&x, &mut rng);
            (1..=8)
                .map(|lag| (autocorr(&x, lag) - autocorr(&s, lag)).abs())
                .fold(0.0f64, f64::max)
        }
        let dev_exact = deviation(1024);
        let dev_pad = deviation(1000);
        println!(
            "gate_phase_surrogate_padding_edge: n=1024 deviation {:.4e}, n=1000 deviation {:.4e}",
            dev_exact, dev_pad
        );
        assert!(
            dev_pad <= dev_exact + 0.15,
            "padding artifact measured: n=1000 lag-autocorr deviation {:.4e} vs n=1024 {:.4e}",
            dev_pad,
            dev_exact
        );
    }

    #[ignore = "n up to 4096 x 200-surrogate MI-lag sweep — heavy, runs in te-gate.yml"]
    #[test]
    fn gate_mi_lag_stability() {
        for &n in &[32usize, 64, 128, 512, 4096] {
            let x = gate_ar1_sine(n, 0.6, 11.0, &mut 7u64);
            let xf: Vec<f64> = x.iter().map(|&v| v as f64).collect();
            let base = find_mi_lag(&xf);
            let mut rng = 99u64;
            let reps = 200usize;
            let mut taus: Vec<usize> = Vec::new();
            let mut none = 0usize;
            for _ in 0..reps {
                let s = phase_randomized_surrogate(&x, &mut rng);
                let sf: Vec<f64> = s.iter().map(|&v| v as f64).collect();
                match find_mi_lag(&sf) {
                    Some(t) => taus.push(t),
                    None => none += 1,
                }
            }
            let mean = if taus.is_empty() {
                0.0
            } else {
                taus.iter().sum::<usize>() as f64 / taus.len() as f64
            };
            let sd = if taus.is_empty() {
                0.0
            } else {
                (taus
                    .iter()
                    .map(|&t| (t as f64 - mean) * (t as f64 - mean))
                    .sum::<f64>()
                    / taus.len() as f64)
                    .sqrt()
            };
            println!(
                "gate_mi_lag_stability: n={} base_tau={:?} none={}/{} tau_mean={:.2} tau_sd={:.2}",
                n, base, none, reps, mean, sd
            );
        }
    }

    #[test]
    fn conditional_arx_surrogate_differs_from_unconditional_2_cond_finite() {
        let n = 300;
        let max_lag = 3;
        let c: Vec<f32> = (0..n)
            .map(|t| (t as f32 * 0.21).sin() + 0.4 * (t as f32 * 0.071).cos())
            .collect();
        let c2: Vec<f32> = (0..n).map(|t| (t as f32 * 0.113).cos()).collect();
        let mut y = vec![0f32; n];
        for t in 0..n {
            let noise = ((t as u64).wrapping_mul(2654435761) >> 24) as f32 / 255.0 - 0.5;
            y[t] = if t < max_lag {
                0.5 * c[t] + noise
            } else {
                0.7 * y[t - 1] - 0.25 * y[t - 2] + 0.5 * c[t] + 0.05 * noise
            };
        }
        let mut rng_a = 42u64;
        let mut rng_b = 42u64;
        let x: Vec<f32> = (0..n)
            .map(|t| 0.4 * c2[t] + (t as f32 * 0.23).sin())
            .collect();
        let cond_surr = arx_conditional_surrogate(
            &y,
            &x,
            &[LaggedCond { series: &c, lag: 0 }],
            max_lag,
            &mut rng_a,
        )
        .expect("the conditional Arx fit resolves");
        let uncond_surr = arx_restricted_surrogate(&y, max_lag, &mut rng_b)
            .expect("the unconditional Arx fit resolves");
        assert_eq!(cond_surr.len(), n);
        assert_eq!(uncond_surr.len(), n);
        let max_diff = cond_surr
            .iter()
            .zip(uncond_surr.iter())
            .map(|(a, b)| (a - b).abs())
            .fold(0.0f32, f32::max);
        assert!(
            max_diff > 1e-4,
            "conditional and unconditional Arx surrogates coincide (max diff {})",
            max_diff
        );
        let mut rng_c = 42u64;
        let s2 = arx_conditional_surrogate(
            &y,
            &x,
            &[
                LaggedCond { series: &c, lag: 0 },
                LaggedCond {
                    series: &c2,
                    lag: 0,
                },
            ],
            max_lag,
            &mut rng_c,
        )
        .expect("the two-confounder Arx fit resolves");
        assert_eq!(s2.len(), n);
        assert!(
            s2.iter().all(|v| v.is_finite()),
            "2-cond Arx surrogate carries non-finite values"
        );
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
        for (t, r) in re.iter_mut().enumerate() {
            *r = (t as f64 * 0.3).sin() + 2.0 * (t as f64 * 0.05).cos();
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
        for (t, y) in yf.iter_mut().enumerate() {
            *y = (t as f64 * 0.5).sin();
        }
        for t in 0..n - 1 {
            xf[t + 1] = 0.5 * xf[t] + 0.6 * yf[t];
        }
        let tau_x = find_mi_lag(&xf).unwrap();
        let tau_y = find_mi_lag(&yf).unwrap();
        let emb_x = embed_series(&xf, tau_x, 3);
        let emb_y = embed_series(&yf, tau_y, 3);
        let te = transfer_entropy_embedded_kde(&xf, &emb_x, &emb_y, tau_x, tau_y).unwrap();
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
        let te = transfer_entropy_embedded_kde(&xf, &emb_x, &emb_y, tau_x, tau_y).unwrap();
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
        assert!(transfer_entropy_embedded_kde(&x, &emb_x, &emb_y, tau, tau).is_none());
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
        for (t, yt) in y.iter_mut().enumerate() {
            *yt = (t as f32 * 0.5).sin();
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
        for (t, yt) in y.iter_mut().enumerate() {
            *yt = (t as f32 * 0.5).sin();
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
        for (t, yt) in y.iter_mut().enumerate() {
            *yt = (t as f32 * 0.5).sin();
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
    fn split_recording_ksg_kde_reduced_te_all_resolve() {
        let seed = 0x9E37_79B9_7F4A_7C15;
        let mut rng = seed;
        let a = gate_ar1(300, 0.5, &mut rng);
        let b: Vec<f32> = (0..a.len())
            .map(|i| {
                if i == 0 {
                    gate_rng(&mut rng) as f32
                } else {
                    (0.9 * a[i - 1] as f64 + (gate_rng(&mut rng) * 0.2 - 0.1)) as f32
                }
            })
            .collect();
        let ksg_verdict = topological_te_phase(&b, &a, 3, 3, seed);
        assert!(
            ksg_verdict.is_some(),
            "KSG phase estimate must resolve for the AR(1) fixture"
        );
        let ksg = ksg_verdict.unwrap();
        let xf: Vec<f64> = b.iter().map(|&v| v as f64).collect();
        let yf: Vec<f64> = a.iter().map(|&v| v as f64).collect();
        let tau_x = find_mi_lag(&xf).expect("driver MI-lag resolves");
        let tau_y = find_mi_lag(&yf).expect("target MI-lag resolves");
        let emb_x = embed_series(&xf, tau_x, 3);
        let emb_y = embed_series(&yf, tau_y, 3);
        let kde_estimate = transfer_entropy_embedded_kde(&xf, &emb_x, &emb_y, tau_x, tau_y);
        assert!(
            kde_estimate.is_some(),
            "KDE estimate must resolve for the AR(1) fixture"
        );
        let kde = kde_estimate.unwrap();
        let reduced_estimate = transfer_entropy_reduced_normalized(&xf, &yf, 1, 1, 4);
        assert!(
            reduced_estimate.is_some(),
            "reduced-TE estimate must resolve for the AR(1) fixture"
        );
        let reduced = reduced_estimate.unwrap();
        println!(
            "split recording: ksg={:.6} kde={:.6} reduced_te_rhat={:.6}",
            ksg.te, kde, reduced
        );
    }

    #[test]
    fn gate_reduced_te_normalized_bounded() {
        let mut rng = 0x9E37_79B9_7F4A_7C15u64;
        for _ in 0..20 {
            let a = gate_ar1(300, 0.5, &mut rng);
            let mut b: Vec<f32> = Vec::with_capacity(a.len());
            b.push(gate_rng(&mut rng) as f32);
            for i in 1..a.len() {
                let v = 0.5 * b[i - 1] as f64
                    + 0.6 * a[i - 1] as f64
                    + (gate_rng(&mut rng) * 0.2 - 0.1);
                b.push(v as f32);
            }
            let xf: Vec<f64> = a.iter().map(|&v| v as f64).collect();
            let yf: Vec<f64> = b.iter().map(|&v| v as f64).collect();
            if let Some(r) = transfer_entropy_reduced_normalized(&xf, &yf, 1, 1, 4) {
                assert!(
                    r.abs() <= 1.0,
                    "Kalibrier-Gate Normierung reduced-TE: coupled pair yields |R̂|={} outside [-1,1]",
                    r
                );
            }
        }
        for _ in 0..20 {
            let a = gate_ar1(300, 0.7, &mut rng);
            let b = gate_ar1(300, 0.7, &mut rng);
            let xf: Vec<f64> = a.iter().map(|&v| v as f64).collect();
            let yf: Vec<f64> = b.iter().map(|&v| v as f64).collect();
            if let Some(r) = transfer_entropy_reduced_normalized(&xf, &yf, 1, 1, 4) {
                assert!(
                    r.abs() <= 1.0,
                    "Kalibrier-Gate Normierung reduced-TE: independent pair yields |R̂|={} outside [-1,1]",
                    r
                );
            }
        }
    }

    #[test]
    fn gate_fn_reduced_te_mdl_coupled_ar1_flows() {
        let mut rng = 0x517C_C1B7_2722_0A95u64;
        let mut found = 0usize;
        let mut meas = 0usize;
        for _ in 0..20 {
            let a = gate_ar1(300, 0.5, &mut rng);
            let mut b: Vec<f32> = Vec::with_capacity(a.len());
            b.push(gate_rng(&mut rng) as f32);
            for i in 1..a.len() {
                let v = 0.5 * b[i - 1] as f64
                    + 0.6 * a[i - 1] as f64
                    + (gate_rng(&mut rng) * 0.2 - 0.1);
                b.push(v as f32);
            }
            let xf: Vec<f64> = a.iter().map(|&v| v as f64).collect();
            let yf: Vec<f64> = b.iter().map(|&v| v as f64).collect();
            if let Some(r) = transfer_entropy_reduced_normalized(&xf, &yf, 1, 1, 4) {
                meas += 1;
                if r > 0.0 {
                    found += 1;
                }
            }
        }
        assert_eq!(
            meas, 20,
            "Kalibrier-Gate FN reduced-TE: {} of 20 coupled pairs measurable — the machine stays silent too often",
            meas
        );
        assert!(
            found as f64 / meas as f64 > 0.5,
            "Kalibrier-Gate FN reduced-TE MDL: {} of {} coupled pairs show R > 0 — the MDL null rejects the planted coupling",
            found,
            meas
        );
    }

    #[test]
    fn gate_fp_reduced_te_mdl_independent_ar1_stays_silent() {
        let mut rng = 0x9E37_79B9_7F4A_7C15u64;
        let mut fp = 0usize;
        let mut meas = 0usize;
        for _ in 0..30 {
            let a = gate_ar1(300, 0.7, &mut rng);
            let b = gate_ar1(300, 0.7, &mut rng);
            let xf: Vec<f64> = a.iter().map(|&v| v as f64).collect();
            let yf: Vec<f64> = b.iter().map(|&v| v as f64).collect();
            if let Some(r) = transfer_entropy_reduced_normalized(&xf, &yf, 1, 1, 4) {
                meas += 1;
                if r > 0.0 {
                    fp += 1;
                }
            }
        }
        assert_eq!(
            meas, 30,
            "Kalibrier-Gate FP reduced-TE: {} of 30 independent pairs measurable",
            meas
        );
        assert!(
            fp <= 8,
            "Kalibrier-Gate FP reduced-TE MDL: {} of {} independent pairs show R > 0 — the MDL null does not hold",
            fp,
            meas
        );
    }

    #[test]
    fn gate_n_floor_reduced_te_table_size() {
        let mut rng = 0x2722_0A95_517C_C1B7u64;
        let a = gate_ar1(300, 0.7, &mut rng);
        let b = gate_ar1(300, 0.7, &mut rng);
        let xf: Vec<f64> = a.iter().map(|&v| v as f64).collect();
        let yf: Vec<f64> = b.iter().map(|&v| v as f64).collect();
        let below = transfer_entropy_reduced_normalized(&xf[..255], &yf[..255], 2, 2, 4);
        assert!(
            below.is_none(),
            "Kalibrier-Gate n-Floor reduced-TE: n=255 below the C^l*C^k=256 contingency table carries no verdict"
        );
        let boundary = transfer_entropy_reduced_normalized(&xf[..258], &yf[..258], 2, 2, 4);
        assert!(
            boundary.is_some(),
            "Kalibrier-Gate n-Floor reduced-TE: n=258 at the C^l*C^k=256 table boundary carries a verdict"
        );
        let above = transfer_entropy_reduced_normalized(&xf, &yf, 2, 2, 4);
        assert!(
            above.is_some(),
            "Kalibrier-Gate n-Floor reduced-TE: n=300 above the table size carries a verdict"
        );
    }

    #[test]
    fn gate_symmetry_reduced_te_identical_series_measure_equally() {
        let mut rng = 0x2722_0A95_517C_C1B7u64;
        let a = gate_ar1(300, 0.7, &mut rng);
        let b = a.clone();
        let xf: Vec<f64> = a.iter().map(|&v| v as f64).collect();
        let yf: Vec<f64> = b.iter().map(|&v| v as f64).collect();
        let ab = transfer_entropy_reduced_normalized(&xf, &yf, 1, 1, 4);
        let ba = transfer_entropy_reduced_normalized(&yf, &xf, 1, 1, 4);
        match (ab, ba) {
            (Some(x), Some(y)) => assert!(
                (x - y).abs() < 1e-12,
                "Kalibrier-Gate symmetry reduced-TE: a=b measures unequal, {} vs {}",
                x,
                y
            ),
            (None, None) => {}
            _ => panic!(
                "Kalibrier-Gate symmetry reduced-TE: one direction measurable, the other not"
            ),
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
    fn calibration_self_null_discriminator_white_driver_stays_at_the_floor() {
        let measure = || -> (Vec<f64>, usize) {
            let trials = 20usize;
            let n = 600usize;
            let mut rng = 0x51A7_E11B_3C0D_9F02u64;
            let mut excess_sds: Vec<f64> = Vec::with_capacity(trials);
            let mut meas = 0usize;
            for t in 0..trials {
                let seed = 0x9E37_79B9_7F4A_7C15 ^ (t as u64).wrapping_mul(0x51A7_E11B_3C0D_9F02);
                let target: Vec<f32> = (0..n)
                    .map(|_| (gate_rng(&mut rng) * 2.0 - 1.0) as f32)
                    .collect();
                let driver: Vec<f32> = (0..n)
                    .map(|_| (gate_rng(&mut rng) * 2.0 - 1.0) as f32)
                    .collect();
                if let Some(v) = topological_te_phase(&target, &driver, 3, 3, seed) {
                    assert!(
                        v.surrogate_sd > 0.0,
                        "self-null floor: the white-noise surrogate null collapsed to zero variance"
                    );
                    meas += 1;
                    excess_sds.push((v.te - v.surrogate_mean) / v.surrogate_sd);
                }
            }
            (excess_sds, meas)
        };
        let (excess_sds, meas) = measure();
        let (again, meas_again) = measure();
        assert_eq!(
            meas, meas_again,
            "self-null floor: the measurement is not seed-deterministic"
        );
        assert_eq!(
            excess_sds, again,
            "self-null floor: the excess series is not seed-deterministic"
        );
        assert_eq!(
            meas, 20,
            "self-null floor: {} of 20 white-noise pairs measurable — the machine stays silent too often",
            meas
        );
        let mean_excess = excess_sds.iter().sum::<f64>() / excess_sds.len() as f64;
        let worst = excess_sds.iter().map(|e| e.abs()).fold(0.0f64, f64::max);
        println!(
            "self-null floor (canonical null): mean excess {mean_excess:+.3} sd over {meas} white-noise pairs, worst |excess| {worst:.3} sd"
        );
    }

    fn gate_ar1_sine(n: usize, phi: f64, period: f64, rng: &mut u64) -> Vec<f32> {
        let mut v = Vec::with_capacity(n);
        let mut x = 0.0f64;
        for t in 0..n {
            x = phi * x
                + (2.0 * std::f64::consts::PI * t as f64 / period).sin()
                + gate_rng(rng) * 0.02
                - 0.01;
            v.push(x as f32);
        }
        v
    }

    #[test]
    fn topological_estimate_deterministic() {
        let mut rng = 0xC0FF_EE11_1234_5678u64;
        let a = gate_ar1_sine(400, 0.6, 37.0, &mut rng);
        let b = gate_ar1_sine(400, 0.6, 43.0, &mut rng);
        let e1 = topological_te_estimate(&a, &b, 3).expect("the pair carries an estimate");
        let e2 = topological_te_estimate(&a, &b, 3).expect("the pair carries an estimate");
        assert_eq!(
            e1.te, e2.te,
            "the estimate is seed-free: the same pair measures the same TE"
        );
        assert_eq!(e1.tau_x, e2.tau_x);
        assert_eq!(e1.tau_y, e2.tau_y);
    }

    #[test]
    fn topological_estimate_white_pair_is_none() {
        let mut rng = 0x0F0F_0F0F_DEAD_BEEFu64;
        let a: Vec<f32> = (0..13)
            .map(|_| (gate_rng(&mut rng) * 2.0 - 1.0) as f32)
            .collect();
        let b: Vec<f32> = (0..13)
            .map(|_| (gate_rng(&mut rng) * 2.0 - 1.0) as f32)
            .collect();
        assert!(
            topological_te_estimate(&a, &b, 3).is_none(),
            "a white pair carries no tau, no estimate"
        );
        assert!(topological_te_estimate(&b, &a, 3).is_none());
    }

    #[test]
    fn topological_estimate_symmetry_identical_series_measure_equally() {
        let mut rng = 0x2722_0A95_517C_C1B7u64;
        let a = gate_ar1_sine(400, 0.6, 37.0, &mut rng);
        let b = a.clone();
        match (
            topological_te_estimate(&a, &b, 3),
            topological_te_estimate(&b, &a, 3),
        ) {
            (Some(x), Some(y)) => assert!(
                (x.te - y.te).abs() < 1e-12,
                "estimate symmetry: a=b measures unequal, {} vs {}",
                x.te,
                y.te
            ),
            (None, None) => {}
            _ => panic!("estimate symmetry: one direction measurable, the other not"),
        }
    }

    #[test]
    fn topological_estimate_tau_fields_match_mi_lag() {
        let mut rng = 0x517C_C1B7_2722_0A95u64;
        let a = gate_ar1_sine(400, 0.6, 37.0, &mut rng);
        let b = gate_ar1_sine(400, 0.6, 43.0, &mut rng);
        let est = topological_te_estimate(&a, &b, 3).expect("the pair carries an estimate");
        let af: Vec<f64> = a.iter().map(|&v| v as f64).collect();
        let bf: Vec<f64> = b.iter().map(|&v| v as f64).collect();
        assert_eq!(est.tau_x, find_mi_lag(&af).expect("x carries a lag"));
        assert_eq!(est.tau_y, find_mi_lag(&bf).expect("y carries a lag"));
    }

    #[test]
    fn topological_estimate_frozen_matches_searched_tau() {
        let mut rng = 0x1357_9BDF_2468_ACE0u64;
        let a = gate_ar1_sine(400, 0.6, 37.0, &mut rng);
        let b = gate_ar1_sine(400, 0.6, 43.0, &mut rng);
        let searched = topological_te_estimate(&a, &b, 3).expect("the pair carries an estimate");
        let frozen = topological_te_estimate_frozen(&a, &b, 3, searched.tau_x, searched.tau_y)
            .expect("the frozen pair carries an estimate");
        assert_eq!(
            frozen.te, searched.te,
            "A = A: the frozen estimator equals the searched estimator at the searched lags"
        );
        assert_eq!(frozen.tau_x, searched.tau_x);
        assert_eq!(frozen.tau_y, searched.tau_y);
    }

    #[test]
    fn topological_estimate_n_floor_short_series_is_none() {
        let mut rng = 0x9E37_79B9_7F4A_7C15u64;
        for n in 8..=13usize {
            let a: Vec<f32> = (0..n)
                .map(|_| (gate_rng(&mut rng) * 2.0 - 1.0) as f32)
                .collect();
            let b: Vec<f32> = (0..n)
                .map(|_| (gate_rng(&mut rng) * 2.0 - 1.0) as f32)
                .collect();
            assert!(
                topological_te_estimate(&a, &b, 3).is_none(),
                "n={n}: the short white pair carries no estimate"
            );
            let mut r2 = 0xABCD_EF01_2345_6789u64 ^ (n as u64);
            let s = gate_ar1_sine(n, 0.6, 37.0, &mut r2);
            assert!(
                topological_te_estimate(&s, &s, 3).is_none(),
                "n={n}: the short structured series carries no estimate"
            );
        }
    }

    fn betti_ar1(n: usize, phi: f64, rng: &mut u64) -> Vec<f64> {
        let mut v = Vec::with_capacity(n);
        let mut x = 0.0f64;
        for _ in 0..n {
            x = phi * x + gate_rng(rng) * 2.0 - 1.0;
            v.push(x);
        }
        v
    }

    fn betti_two_cluster(n: usize, rng: &mut u64) -> Vec<f64> {
        let mut v = Vec::with_capacity(n);
        let mut level = 1.0f64;
        let mut since = 0usize;
        for _ in 0..n {
            if since >= 40 {
                level = -level;
                since = 0;
            }
            since += 1;
            v.push(level + gate_rng(rng) * 0.02 - 0.01);
        }
        v
    }

    #[test]
    fn betti0_fp_independent_ar1_has_no_persistent_component() {
        let mut rng = 0x9E37_79B9_7F4A_7C15u64;
        let mut meas = 0usize;
        for _ in 0..20 {
            let a = betti_ar1(300, 0.7, &mut rng);
            if let Some(v) = betti0_persistence(&a, 3) {
                meas += 1;
                assert!(
                    v.persistence < 0.5,
                    "betti0-FP: persistence {} for independent AR(1) carries a persistent component",
                    v.persistence
                );
            }
        }
        assert!(
            meas >= 15,
            "betti0-FP: {} of 20 measurable — the machine stays silent too often",
            meas
        );
    }

    #[test]
    fn betti0_fn_structured_series_has_persistent_component() {
        let mut rng = 0x517C_C1B7_2722_0A95u64;
        let mut meas = 0usize;
        for _ in 0..20 {
            let s = betti_two_cluster(320, &mut rng);
            if let Some(v) = betti0_persistence(&s, 2) {
                meas += 1;
                assert!(
                    v.persistence > 0.5,
                    "betti0-FN: persistence {} for the structured series misses the persistent component",
                    v.persistence
                );
            }
        }
        assert!(
            meas >= 15,
            "betti0-FN: {} of 20 measurable — the machine stays silent too often",
            meas
        );
    }

    #[test]
    fn betti0_symmetry_identical_series_measure_equally() {
        let mut rng = 0x2722_0A95_517C_C1B7u64;
        let a = betti_ar1(300, 0.7, &mut rng);
        let b = a.clone();
        let ab = betti0_persistence(&a, 3);
        let ba = betti0_persistence(&b, 3);
        match (ab, ba) {
            (Some(x), Some(y)) => {
                assert_eq!(x.ladder, y.ladder, "betti0-symmetry: ladder differs");
                assert_eq!(x.betti0, y.betti0, "betti0-symmetry: betti0 differs");
                assert_eq!(x.deaths, y.deaths, "betti0-symmetry: deaths differ");
                assert_eq!(
                    x.persistence, y.persistence,
                    "betti0-symmetry: persistence differs"
                );
            }
            (None, None) => {}
            _ => panic!("betti0-symmetry: one direction measurable, the other not"),
        }
    }

    #[test]
    fn betti0_n_floor_no_verdict_below_threshold() {
        let mut rng = 0x9E37_79B9_7F4A_7C15u64;
        let a = betti_ar1(16, 0.7, &mut rng);
        assert!(
            betti0_persistence(&a, 3).is_none(),
            "betti0-n-floor: n=16 carries a verdict"
        );
    }

    fn betti0_quantile(sorted: &[f64], p: f64) -> f64 {
        let i = (sorted.len() as f64 * p).ceil() as usize;
        sorted[i - 1]
    }

    fn betti0_bootstrap(
        series: &[f64],
        dim: usize,
        n_boot: usize,
        rng: &mut u64,
    ) -> (Option<f64>, Option<f64>, Option<f64>, usize) {
        let m = series.len();
        let point = betti0_persistence(series, dim).map(|v| v.persistence);
        let mut values = Vec::with_capacity(n_boot);
        let mut resampled = Vec::with_capacity(m);
        for _ in 0..n_boot {
            resampled.clear();
            for _ in 0..m {
                let idx = ((gate_rng(rng) * m as f64) as usize).min(m - 1);
                resampled.push(series[idx]);
            }
            if let Some(v) = betti0_persistence(&resampled, dim) {
                values.push(v.persistence);
            }
        }
        if values.is_empty() {
            return (point, None, None, 0);
        }
        values.sort_unstable_by(|a, b| a.total_cmp(b));
        let n_meas = values.len();
        (
            point,
            Some(betti0_quantile(&values, 0.025)),
            Some(betti0_quantile(&values, 0.975)),
            n_meas,
        )
    }

    #[test]
    fn betti0_calibration_fp_null_q95_stays_below_threshold() {
        let mut rng = 0xB377_0C4A_17E5_33D2u64;
        let mut values = Vec::with_capacity(100);
        for _ in 0..100 {
            let a = betti_ar1(300, 0.7, &mut rng);
            if let Some(v) = betti0_persistence(&a, 3) {
                values.push(v.persistence);
            }
        }
        assert!(
            values.len() >= 90,
            "betti0-calibration FP: {} of 100 AR(1) nulls measurable — the machine stays silent too often",
            values.len()
        );
        values.sort_unstable_by(|a, b| a.total_cmp(b));
        let q95 = betti0_quantile(&values, 0.95);
        let above = values.iter().filter(|&&p| p >= 0.5).count();
        assert!(
            q95 < 0.5,
            "betti0-calibration FP: q95 = {q95:.4} of the AR(1) null distribution, {} of {} at or above 0.5 — the 0.5 threshold sits inside the null spread",
            above,
            values.len()
        );
    }

    #[test]
    fn betti0_calibration_fn_structured_q05_stays_above_threshold() {
        let mut rng = 0x17E5_33D2_B377_0C4Au64;
        let mut values = Vec::with_capacity(100);
        for _ in 0..100 {
            let s = betti_two_cluster(320, &mut rng);
            if let Some(v) = betti0_persistence(&s, 2) {
                values.push(v.persistence);
            }
        }
        assert!(
            values.len() >= 90,
            "betti0-calibration FN: {} of 100 two-cluster series measurable — the machine stays silent too often",
            values.len()
        );
        values.sort_unstable_by(|a, b| a.total_cmp(b));
        let q05 = betti0_quantile(&values, 0.05);
        let below = values.iter().filter(|&&p| p <= 0.5).count();
        assert!(
            q05 > 0.5,
            "betti0-calibration FN: q05 = {q05:.4} of the two-cluster distribution, {} of {} at or below 0.5 — the 0.5 threshold swallows the persistent component",
            below,
            values.len()
        );
    }

    #[test]
    fn betti0_bootstrap_fp_ci_upper_stays_below_threshold() {
        let mut rng = 0x4A17_E533_D2B3_770Cu64;
        let a = betti_ar1(300, 0.7, &mut rng);
        let (point, ci_low, ci_high, n_meas) = betti0_bootstrap(&a, 3, 200, &mut rng);
        assert!(
            n_meas >= 180,
            "betti0-bootstrap FP: {} of 200 resamples measurable — the bootstrap stays silent too often",
            n_meas
        );
        let (ci_low, ci_high) = (ci_low.unwrap(), ci_high.unwrap());
        assert!(
            ci_high < 0.5,
            "betti0-bootstrap FP: the 95% percentile bootstrap CI [{ci_low:.4}, {ci_high:.4}] reaches the 0.5 threshold (point estimate {point:?})"
        );
    }

    #[test]
    fn betti0_bootstrap_fn_ci_lower_stays_above_threshold() {
        let mut rng = 0xE533_D2B3_770C_4A17u64;
        let s = betti_two_cluster(320, &mut rng);
        let (point, ci_low, ci_high, n_meas) = betti0_bootstrap(&s, 2, 200, &mut rng);
        assert!(
            n_meas >= 180,
            "betti0-bootstrap FN: {} of 200 resamples measurable — the bootstrap stays silent too often",
            n_meas
        );
        let (ci_low, ci_high) = (ci_low.unwrap(), ci_high.unwrap());
        assert!(
            ci_low > 0.5,
            "betti0-bootstrap FN: the 95% percentile bootstrap CI [{ci_low:.4}, {ci_high:.4}] reaches the 0.5 threshold (point estimate {point:?})"
        );
    }

    #[test]
    fn phase_block_null_stays_byte_identical_at_ten() {
        let mut rng = 0x0A95_517C_C1B7_2722u64;
        let a = gate_ar1(300, 0.7, &mut rng);
        let b = gate_ar1(300, 0.3, &mut rng);
        let seed = 0x9E37_79B9_7F4A_7C15;
        for lag in [0usize, 1, 2, 3, 5] {
            assert_eq!(
                surrogate_stats_phase(&b, &a, lag, seed),
                surrogate_stats_phase_n(&b, &a, lag, seed, 10),
                "phase null: the 10-path must stay byte-identical through the _n form"
            );
            assert_eq!(
                surrogate_stats_block(&b, &a, lag, 5, seed),
                surrogate_stats_block_n(&b, &a, lag, 5, seed, 10),
                "block null: the 10-path must stay byte-identical through the _n form"
            );
        }
    }

    const FPR_RISE_Z: f64 = 3.0;

    fn fpr_rise_sigma_test(f0: f64, f9: f64, neg0: usize, neg9: usize) -> bool {
        let p0 = f0 / 100.0;
        let p9 = f9 / 100.0;
        let var = p0 * (1.0 - p0) / neg0 as f64 + p9 * (1.0 - p9) / neg9 as f64;
        f9 - f0 <= FPR_RISE_Z * var.sqrt() * 100.0
    }

    fn gate_fpr_autocorr_assert(cells: &[GateCell]) {
        let rows: Vec<(f32, usize, Option<f64>)> = cells
            .iter()
            .map(|c| {
                (
                    c.a,
                    c.d_z,
                    (c.neg > 0).then(|| 100.0 * c.fp as f64 / c.neg as f64),
                )
            })
            .collect();
        let named: String = rows
            .iter()
            .map(|&(a, d_z, fpr)| match fpr {
                Some(fpr) => format!("a={a} D_Z={d_z}: {fpr:.2}% "),
                None => format!("a={a} D_Z={d_z}: unmeasured "),
            })
            .collect();
        let unmeasured: Vec<(f32, usize)> = rows
            .iter()
            .filter(|&&(_, _, fpr)| fpr.is_none())
            .map(|&(a, d_z, _)| (a, d_z))
            .collect();
        assert!(
            unmeasured.is_empty(),
            "Zug 5: cells {unmeasured:?} carry neg=0 — unmeasured, never a passing zero ({named})"
        );
        for &(a, d_z, fpr) in &rows {
            let fpr = fpr.expect("Zug 5: cell measured (neg=0 absent)");
            assert!(
                fpr <= 8.0,
                "Zug 5: FPR {fpr:.2}% at a={a} D_Z={d_z} exceeds 8% — the null does not hold under autocorrelation ({named})"
            );
        }
        let mut d_zs: Vec<usize> = cells.iter().map(|c| c.d_z).collect();
        d_zs.sort_unstable();
        d_zs.dedup();
        for d_z in d_zs {
            let c0 = cells
                .iter()
                .find(|c| c.a == 0.0 && c.d_z == d_z)
                .expect("Zug 5: a=0 cell measured");
            let c9 = cells
                .iter()
                .find(|c| c.a == 0.9 && c.d_z == d_z)
                .expect("Zug 5: a=0.9 cell measured");
            let f0 = 100.0 * c0.fp as f64 / c0.neg as f64;
            let f9 = 100.0 * c9.fp as f64 / c9.neg as f64;
            let rise = f9 - f0;
            assert!(
                fpr_rise_sigma_test(f0, f9, c0.neg, c9.neg),
                "Zug 5: FPR rise {:.2}pp over a at D_Z={d_z} exceeds 3σ of the binomial difference of the measured cells — the null leaks autocorrelation into the FPR ({named})",
                rise
            );
        }
    }

    fn fpr_rise_normal_tail(z: f64) -> f64 {
        let steps = 4000usize;
        let hi = 10.0;
        let h = (hi - z) / steps as f64;
        let mut sum = 0.0;
        for i in 0..steps {
            let x = z + (i as f64 + 0.5) * h;
            sum += (-0.5 * x * x).exp();
        }
        sum * h / (2.0 * std::f64::consts::PI).sqrt()
    }

    fn fpr_rise_trip_mc(p: f64, neg0: usize, neg9: usize, n: usize, rng: &mut u64) -> (f64, f64) {
        let mut one = 0usize;
        let mut two = 0usize;
        for _ in 0..n {
            let mut fp0 = 0usize;
            for _ in 0..neg0 {
                if next_rng(rng) < p {
                    fp0 += 1;
                }
            }
            let mut fp9 = 0usize;
            for _ in 0..neg9 {
                if next_rng(rng) < p {
                    fp9 += 1;
                }
            }
            let f0 = 100.0 * fp0 as f64 / neg0 as f64;
            let f9 = 100.0 * fp9 as f64 / neg9 as f64;
            if !fpr_rise_sigma_test(f0, f9, neg0, neg9) {
                one += 1;
            }
            let p0 = f0 / 100.0;
            let p9 = f9 / 100.0;
            let var = p0 * (1.0 - p0) / neg0 as f64 + p9 * (1.0 - p9) / neg9 as f64;
            if (f9 - f0).abs() > FPR_RISE_Z * var.sqrt() * 100.0 {
                two += 1;
            }
        }
        (one as f64 / n as f64, two as f64 / n as f64)
    }

    #[test]
    fn gate_fpr_rise_calibration() {
        let mc = 100_000usize;
        let mut rng = 0x517C_C1B7_2722_0A95u64;
        let configs = [
            (0.08f64, 400usize, 400usize),
            (0.08f64, 392usize, 392usize),
            (0.0475f64, 400usize, 400usize),
            (0.06f64, 400usize, 400usize),
            (0.0725f64, 400usize, 400usize),
            (0.0659f64, 392usize, 392usize),
            (0.0714f64, 392usize, 392usize),
        ];
        let exp_one = fpr_rise_normal_tail(FPR_RISE_Z);
        let exp_two = 2.0 * exp_one;
        let mc_sigma_one = (exp_one * (1.0 - exp_one) / mc as f64).sqrt();
        let mc_sigma_two = (exp_two * (1.0 - exp_two) / mc as f64).sqrt();
        let mut two_sum = 0.0;
        for (p, neg0, neg9) in configs {
            let (one, two) = fpr_rise_trip_mc(p, neg0, neg9, mc, &mut rng);
            two_sum += two;
            assert!(
                (one - exp_one).abs() <= 3.0 * mc_sigma_one,
                "one-sided trip rate {one:.5} at p={p} neg=({neg0},{neg9}) leaves the MC band around {exp_one:.5}"
            );
            assert!(
                (two - exp_two).abs() <= 3.0 * mc_sigma_two,
                "two-sided trip rate {two:.5} at p={p} neg=({neg0},{neg9}) leaves the MC band around {exp_two:.5}"
            );
        }
        let collective = 1.0 - (1.0 - two_sum / configs.len() as f64).powi(18);
        assert!(
            (collective - 0.05).abs() <= 0.01,
            "the collective two-sided false-trip rate over 18 rise cells is {collective:.3} — the 5% multiplicity holds"
        );
        assert!(
            fpr_rise_sigma_test(4.75, 7.25, 400, 400),
            "the measured restricted-null rise 4.75→7.25% over a (1.49σ) stays green under the 3σ test"
        );
        assert!(
            !fpr_rise_sigma_test(9.0, 18.0, 400, 400),
            "the folge86 leak profile 9→18% over a at rho=0.9 measures 3.76σ at the battery cell size and stays red"
        );
    }

    fn gate_fpr_autocorr(null: TeNull, est: TeEstimator) {
        let cells = gate_fpr_cells(
            150,
            GateParams {
                null,
                est,
                max_lag: 2,
                null_lag: 12,
                bins: 4,
                block: 0,
                n_surr: 200,
            },
        );
        gate_fpr_autocorr_assert(&cells);
    }

    #[test]
    fn gate_fpr_autocorrelation_block_null_binned_n_surr_200() {
        gate_fpr_autocorr(TeNull::Block, TeEstimator::Binned);
    }

    #[test]
    #[ignore = "n=1000 calibration gate — heavy, runs in te-gate.yml"]
    fn gate_fpr_autocorrelation_phase_null_binned_n_1000() {
        let cells = gate_fpr_coarse_cells(
            1000,
            GateParams {
                null: TeNull::Phase,
                est: TeEstimator::Binned,
                max_lag: 2,
                null_lag: 12,
                bins: 4,
                block: 0,
                n_surr: 200,
            },
        );
        gate_fpr_autocorr_assert(&cells);
    }

    #[test]
    fn gate_fpr_autocorrelation_block_null_ksg_n_surr_200() {
        gate_fpr_autocorr(TeNull::Block, TeEstimator::Ksg);
    }

    #[test]
    fn gate_fpr_autocorrelation_shift_null_binned_n_surr_200() {
        gate_fpr_autocorr(TeNull::Shift, TeEstimator::Binned);
    }

    #[test]
    fn gate_fpr_autocorrelation_shift_null_ksg_n_surr_200() {
        gate_fpr_autocorr(TeNull::Shift, TeEstimator::Ksg);
    }

    #[test]
    fn gate_fpr_autocorrelation_restricted_null_binned_n_surr_200() {
        gate_fpr_autocorr(TeNull::RestrictedPermutation, TeEstimator::Binned);
    }

    #[test]
    fn gate_fpr_autocorrelation_xshift_null_binned_n_surr_200() {
        gate_fpr_autocorr(TeNull::XShift, TeEstimator::Binned);
    }

    #[test]
    fn gate_fpr_autocorrelation_arx_null_binned_n_surr_200() {
        gate_fpr_autocorr(TeNull::Arx, TeEstimator::Binned);
    }

    #[test]
    fn gate_fpr_autocorrelation_coherent_phase_null_binned_n_surr_200() {
        gate_fpr_autocorr(TeNull::CoherentPhase, TeEstimator::Binned);
    }

    #[test]
    #[ignore = "n=1000 calibration gate — heavy, runs in te-gate.yml"]
    fn gate_fpr_autocorrelation_coherent_phase_null_binned_n_1000() {
        let cells = gate_fpr_coarse_cells(
            1000,
            GateParams {
                null: TeNull::CoherentPhase,
                est: TeEstimator::Binned,
                max_lag: 2,
                null_lag: 12,
                bins: 4,
                block: 0,
                n_surr: 200,
            },
        );
        gate_fpr_autocorr_assert(&cells);
    }

    #[test]
    fn coherent_phase_null_absorbs_linear_cross_coupling() {
        let mut rng = 0x1357_9BDF_2468_ACE0u64;
        let n = 400usize;
        let x = gate_ar1(n, 0.7, &mut rng);
        let mut y = vec![0f32; n];
        for t in 1..n {
            y[t] = 0.5 * y[t - 1] + 0.6 * x[t - 1] + 0.1 * gate_gauss(&mut rng);
        }
        let obs = transfer_entropy_conditional_binned_n(&y, &x, &[], 1, 4)
            .expect("the linear coupling is measurable");
        let params = |null| TeStatsParams {
            lag: 1,
            max_lag: 8,
            bins: 4,
            seed: 0x9E37_79B9_7F4A_7C15,
            n_surr: 200,
            null,
        };
        let (_, _, thr_coherent) =
            conditional_te_stats_lagged_n(&y, &x, &[], params(TeNull::CoherentPhase))
                .expect("the coherent null carries a threshold");
        let (_, _, thr_phase) = conditional_te_stats_lagged_n(&y, &x, &[], params(TeNull::Phase))
            .expect("the single-phase null carries a threshold");
        assert!(
            obs <= thr_coherent,
            "the coherent null preserves the linear cross-structure — the linear transfer stays under its threshold: obs {obs:.5} threshold {thr_coherent:.5}"
        );
        assert!(
            obs > thr_phase,
            "the single-phase null destroys the cross-phase — the linear transfer breaks its threshold: obs {obs:.5} threshold {thr_phase:.5}"
        );
    }

    #[test]
    fn coherent_phase_null_detects_nonlinear_transfer() {
        let mut rng = 0x2468_ACE0_1357_9BDFu64;
        let n = 600usize;
        let x = gate_ar1(n, 0.7, &mut rng);
        let mut y = vec![0f32; n];
        for t in 1..n {
            y[t] = 0.3 * y[t - 1] + 1.2 * x[t - 1] * x[t - 1] + 0.1 * gate_gauss(&mut rng);
        }
        let obs = transfer_entropy_conditional_binned_n(&y, &x, &[], 1, 4)
            .expect("the nonlinear transfer is measurable");
        let (_, _, thr) = conditional_te_stats_lagged_n(
            &y,
            &x,
            &[],
            TeStatsParams {
                lag: 1,
                max_lag: 8,
                bins: 4,
                seed: 0x9E37_79B9_7F4A_7C15,
                n_surr: 200,
                null: TeNull::CoherentPhase,
            },
        )
        .expect("the coherent null carries a threshold");
        assert!(
            obs > thr,
            "the bispectral transfer survives the coherent null: obs {obs:.5} threshold {thr:.5}"
        );
    }

    #[test]
    #[ignore = "n=1000 calibration gate — heavy, runs in te-gate.yml"]
    fn gate_fpr_autocorrelation_arx_null_binned_n_1000() {
        let cells = gate_fpr_coarse_cells(
            1000,
            GateParams {
                null: TeNull::Arx,
                est: TeEstimator::Binned,
                max_lag: 2,
                null_lag: 12,
                bins: 4,
                block: 0,
                n_surr: 200,
            },
        );
        gate_fpr_autocorr_assert(&cells);
    }

    #[test]
    #[ignore = "n=1000 calibration gate — heavy, runs in te-gate.yml"]
    fn gate_fpr_autocorrelation_xshift_null_binned_n_1000() {
        let cells = gate_fpr_coarse_cells(
            1000,
            GateParams {
                null: TeNull::XShift,
                est: TeEstimator::Binned,
                max_lag: 2,
                null_lag: 12,
                bins: 4,
                block: 0,
                n_surr: 200,
            },
        );
        gate_fpr_autocorr_assert(&cells);
    }

    #[test]
    #[ignore = "n=1000 calibration gate — heavy, runs in te-gate.yml"]
    fn gate_fpr_autocorrelation_shift_null_binned_n_1000() {
        let cells = gate_fpr_coarse_cells(
            1000,
            GateParams {
                null: TeNull::Shift,
                est: TeEstimator::Binned,
                max_lag: 2,
                null_lag: 12,
                bins: 4,
                block: 0,
                n_surr: 200,
            },
        );
        gate_fpr_autocorr_assert(&cells);
    }

    #[test]
    #[ignore = "n=1000 calibration gate — heavy, runs in te-gate.yml"]
    fn gate_fpr_autocorrelation_shift_null_ksg_n_1000() {
        let cells = gate_fpr_coarse_cells(
            1000,
            GateParams {
                null: TeNull::Shift,
                est: TeEstimator::Ksg,
                max_lag: 2,
                null_lag: 12,
                bins: 4,
                block: 0,
                n_surr: 200,
            },
        );
        gate_fpr_autocorr_assert(&cells);
    }

    #[test]
    #[ignore = "n=1000 calibration gate — heavy, runs in te-gate.yml"]
    fn gate_fpr_autocorrelation_arx_null_ksg_n_1000() {
        let cells = gate_fpr_coarse_cells(
            1000,
            GateParams {
                null: TeNull::Arx,
                est: TeEstimator::Ksg,
                max_lag: 2,
                null_lag: 12,
                bins: 4,
                block: 0,
                n_surr: 200,
            },
        );
        gate_fpr_autocorr_assert(&cells);
    }

    #[test]
    #[ignore = "block sweep for the n=1000 gate fix — runs in te-gate.yml"]
    fn block_sweep_n1000() {
        for block in [10usize, 16, 24, 32, 48] {
            let cells = gate_fpr_cells_from(
                1000,
                &[(0.0f32, 4usize, 7usize), (0.5f32, 4, 7), (0.9f32, 4, 7)],
                GateParams {
                    null: TeNull::Block,
                    est: TeEstimator::Binned,
                    max_lag: 2,
                    null_lag: 12,
                    bins: 4,
                    block,
                    n_surr: 100,
                },
            );
            for c in &cells {
                println!(
                    "block={block} a={} d_z={} fpr={:.2}%",
                    c.a,
                    c.d_z,
                    100.0 * c.fp as f64 / c.neg as f64
                );
            }
        }
    }

    #[test]
    #[ignore = "KSG sweep for the n=1000 gate — runs in te-gate.yml"]
    fn ksg_sweep_n1000() {
        for (null_name, null) in [
            ("block", TeNull::Block),
            ("phase", TeNull::Phase),
            ("arx", TeNull::Arx),
        ] {
            let cells = gate_fpr_cells_from(
                1000,
                &[(0.0f32, 4usize, 7usize), (0.5f32, 4, 7), (0.9f32, 4, 7)],
                GateParams {
                    null,
                    est: TeEstimator::Ksg,
                    max_lag: 2,
                    null_lag: 12,
                    bins: 4,
                    block: 0,
                    n_surr: 100,
                },
            );
            for c in &cells {
                println!(
                    "null={null_name} a={} d_z={} fpr={:.2}%",
                    c.a,
                    c.d_z,
                    100.0 * c.fp as f64 / c.neg as f64
                );
            }
        }
    }

    #[test]
    #[ignore = "shift sweep for the n=1000 gate — runs in te-gate.yml"]
    fn shift_sweep_n1000() {
        for (est_name, est) in [("binned", TeEstimator::Binned), ("ksg", TeEstimator::Ksg)] {
            let cells = gate_fpr_cells_from(
                1000,
                &[(0.0f32, 4usize, 7usize), (0.5f32, 4, 7), (0.9f32, 4, 7)],
                GateParams {
                    null: TeNull::Shift,
                    est,
                    max_lag: 2,
                    null_lag: 12,
                    bins: 4,
                    block: 0,
                    n_surr: 100,
                },
            );
            for c in &cells {
                println!(
                    "est={est_name} a={} d_z={} fpr={:.2}%",
                    c.a,
                    c.d_z,
                    100.0 * c.fp as f64 / c.neg as f64
                );
            }
        }
    }

    #[test]
    #[ignore = "arx sweep for the n=1000 gate — runs in te-gate.yml"]
    fn arx_sweep_n1000() {
        let mut ols_rng = 0xC2B2_AE3D_85EB_CA6Bu64;
        for a in [0.0f32, 0.5, 0.9] {
            let mut ols_resolved = 0usize;
            let mut ols_total = 0usize;
            for _ in 0..100 {
                let series = gate_common_driver(1000, a, 0.0, 4, &mut ols_rng);
                for j in 0..series.len() {
                    let conds: Vec<&[f32]> = (0..series.len())
                        .filter(|&i| i != j)
                        .map(|i| series[i].as_slice())
                        .collect();
                    ols_total += 1;
                    if ols_fit_lagged_n(&series[j], &conds, 12).is_some() {
                        ols_resolved += 1;
                    }
                }
            }
            for (est_name, est) in [("ksg", TeEstimator::Ksg)] {
                let cells = gate_fpr_cells_from(
                    1000,
                    &[(a, 4usize, 7usize)],
                    GateParams {
                        null: TeNull::Arx,
                        est,
                        max_lag: 2,
                        null_lag: 12,
                        bins: 4,
                        block: 0,
                        n_surr: 100,
                    },
                );
                for c in &cells {
                    println!(
                        "est={est_name} a={} d_z={} fpr={:.2}% ols_resolved={ols_resolved}/{ols_total}",
                        c.a,
                        c.d_z,
                        100.0 * c.fp as f64 / c.neg as f64
                    );
                }
            }
        }
    }

    #[test]
    #[ignore = "coupling scan for the per-lag gate — heavy, runs in te-gate.yml"]
    fn coupling_scan_te_over_thr_n1000() {
        let n = 800usize;
        let couplings = [0.3f32, 0.6, 0.9, 1.2, 2.0];
        let trials = 5usize;
        let n_surr = 40usize;
        let mut rng = 0x6A2B_7A5B_3C1D_9E4Fu64;
        let noise = |rng: &mut u64| -> f32 {
            *rng = rng
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            (((*rng >> 33) as f64) / ((u32::MAX >> 1) as f64)) as f32
        };
        let mut ratios: Vec<(f32, Option<f64>)> = Vec::new();
        for &coupling in &couplings {
            let mut te_sum = 0.0;
            let mut thr_sum = 0.0;
            let mut meas = 0usize;
            for t in 0..trials {
                let seed = 0x9E37_79B9_7F4A_7C15 ^ (t as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15);
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
                    x[t + 1] += coupling * y_ind[t];
                }
                let Some(te) = transfer_entropy_conditional(&x, &y, &c, 1) else {
                    continue;
                };
                let Some((_, _, thr)) = conditional_te_stats_lagged(&x, &y, &c, 1, 1, seed, n_surr)
                else {
                    continue;
                };
                te_sum += te;
                thr_sum += thr;
                meas += 1;
            }
            let ratio = (meas > 0).then(|| (te_sum / meas as f64) / (thr_sum / meas as f64));
            match ratio {
                Some(r) => println!(
                    "coupling={coupling} te={:.4} thr={:.4} te/thr={r:.3}",
                    te_sum / meas as f64,
                    thr_sum / meas as f64
                ),
                None => println!("coupling={coupling}: unmeasured"),
            }
            ratios.push((coupling, ratio));
        }
        let named: String = ratios
            .iter()
            .map(|&(c, r)| match r {
                Some(r) => format!("c={c}:{r:.3} "),
                None => format!("c={c}:unmeasured "),
            })
            .collect();
        for &(c, r) in &ratios {
            assert!(
                r.is_some(),
                "coupling scan: cell c={c} unmeasured — never a passing zero ({named})"
            );
        }
        let strongest = ratios
            .iter()
            .find(|&&(c, _)| c == 2.0)
            .expect("c=2.0 cell measured")
            .1
            .expect("c=2.0 cell carries a ratio");
        assert!(
            strongest > 1.0,
            "coupling scan: the strongest coupling does not clear its null (te/thr {strongest:.3}) — the null swallows the true coupling ({named})"
        );
    }

    #[test]
    #[ignore = "FPR-vs-bins sweep for the per-lag gate — heavy, runs in te-gate.yml"]
    fn fpr_bins_sweep_n2000() {
        let bins_set = [16usize, 32, 64, 128];
        let a_set = [0.0f32, 0.5, 0.9];
        let trials_per_cell = 336usize;
        for &bins in &bins_set {
            let cells: Vec<(f32, usize, usize)> = a_set
                .iter()
                .map(|&a| (a, 0usize, trials_per_cell))
                .collect();
            let out = gate_fpr_cells_from(
                150,
                &cells,
                GateParams {
                    null: TeNull::RestrictedPermutation,
                    est: TeEstimator::Binned,
                    max_lag: 2,
                    null_lag: 12,
                    bins,
                    block: 0,
                    n_surr: 200,
                },
            );
            let mut names = String::new();
            for (c, a) in out.iter().zip(&a_set) {
                let fpr = (c.neg > 0).then(|| 100.0 * c.fp as f64 / c.neg as f64);
                match fpr {
                    Some(f) => {
                        names.push_str(&format!("a={a}:{f:.2}% "));
                        println!("bins={bins} a={a} fpr={f:.2}%");
                    }
                    None => {
                        names.push_str(&format!("a={a}:unmeasured "));
                        println!("bins={bins} a={a} fpr=unmeasured");
                    }
                }
            }
            for (c, a) in out.iter().zip(&a_set) {
                assert!(
                    c.neg > 0,
                    "FPR-vs-bins sweep: bins={bins} a={a} cell unmeasured (neg=0) — never a passing zero ({names})"
                );
            }
            for (c, a) in out.iter().zip(&a_set) {
                let fpr = 100.0 * c.fp as f64 / c.neg as f64;
                assert!(
                    fpr <= 8.0,
                    "FPR-vs-bins sweep: FPR {fpr:.2}% at bins={bins} a={a} exceeds 8% ({names})"
                );
            }
            let f0 = out.iter().find(|c| c.a == 0.0).expect("a=0 cell measured");
            let f9 = out
                .iter()
                .find(|c| c.a == 0.9)
                .expect("a=0.9 cell measured");
            let fpr0 = 100.0 * f0.fp as f64 / f0.neg as f64;
            let fpr9 = 100.0 * f9.fp as f64 / f9.neg as f64;
            assert!(
                fpr_rise_sigma_test(fpr0, fpr9, f0.neg, f9.neg),
                "FPR-vs-bins sweep: FPR rise {:.2}pp over a at bins={bins} exceeds 3σ of the binomial difference of the measured cells ({names})",
                fpr9 - fpr0
            );
        }
    }

    fn gate_conditional_driver(
        n: usize,
        a: f32,
        rho: f32,
        coupling: f32,
        rng: &mut u64,
    ) -> (Vec<f32>, Vec<f32>, Vec<f32>) {
        let burn = 200;
        let rho = rho.clamp(0.0, 0.95);
        let rho_x = (1.0 - rho * rho).sqrt();
        let mut c = vec![0f32; burn + n];
        let mut x = vec![0f32; burn + n];
        let mut y = vec![0f32; burn + n];
        for t in 1..burn + n {
            c[t] = a * c[t - 1] + gate_gauss(rng);
            y[t] = a * y[t - 1] + 0.5 * c[t - 1] + gate_gauss(rng);
            x[t] = a * x[t - 1] + rho * c[t] + rho_x * gate_gauss(rng) + coupling * y[t - 1];
        }
        (x[burn..].to_vec(), y[burn..].to_vec(), c[burn..].to_vec())
    }

    fn gate_conditional_driver_2(
        n: usize,
        a: f32,
        rho: f32,
        coupling: f32,
        rng: &mut u64,
    ) -> (Vec<f32>, Vec<f32>, Vec<f32>, Vec<f32>) {
        let burn = 200;
        let rho = rho.clamp(0.0, 0.95);
        let rho_x = (1.0 - rho * rho).sqrt();
        let mut c1 = vec![0f32; burn + n];
        let mut c2 = vec![0f32; burn + n];
        let mut x = vec![0f32; burn + n];
        let mut y = vec![0f32; burn + n];
        for t in 1..burn + n {
            c1[t] = a * c1[t - 1] + gate_gauss(rng);
            c2[t] = a * c2[t - 1] + gate_gauss(rng);
            y[t] = a * y[t - 1] + 0.5 * c1[t - 1] + 0.5 * c2[t - 1] + gate_gauss(rng);
            x[t] = a * x[t - 1]
                + rho * c1[t]
                + rho * c2[t]
                + rho_x * gate_gauss(rng)
                + coupling * y[t - 1];
        }
        (
            x[burn..].to_vec(),
            y[burn..].to_vec(),
            c1[burn..].to_vec(),
            c2[burn..].to_vec(),
        )
    }

    fn gate_conditional_driver_reversed(
        n: usize,
        a: f32,
        rho: f32,
        coupling: f32,
        rng: &mut u64,
    ) -> (Vec<f32>, Vec<f32>, Vec<f32>) {
        let burn = 200;
        let rho = rho.clamp(0.0, 0.95);
        let rho_x = (1.0 - rho * rho).sqrt();
        let mut c = vec![0f32; burn + n];
        let mut x = vec![0f32; burn + n];
        let mut y = vec![0f32; burn + n];
        for t in 1..burn + n {
            c[t] = a * c[t - 1] + gate_gauss(rng);
            x[t] = a * x[t - 1] + 0.5 * c[t - 1] + gate_gauss(rng);
            y[t] = a * y[t - 1] + rho * c[t] + rho_x * gate_gauss(rng) + coupling * x[t - 1];
        }
        (x[burn..].to_vec(), y[burn..].to_vec(), c[burn..].to_vec())
    }

    fn gate_conditional_fpr(
        n: usize,
        a: f32,
        rho: f32,
        max_lag: usize,
        trials: usize,
        n_surr: usize,
        rng: &mut u64,
    ) -> (usize, usize) {
        let mut fp = 0usize;
        let mut neg = 0usize;
        for t in 0..trials {
            let seed = 0x9E37_79B9_7F4A_7C15 ^ (t as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15);
            let (x, y, c) = gate_conditional_driver(n, a, rho, 0.0, rng);
            let Some(te) = transfer_entropy_conditional(&x, &y, &c, 1) else {
                continue;
            };
            let Some((_, _, thr)) =
                conditional_te_stats_lagged(&x, &y, &c, 1, max_lag, seed, n_surr)
            else {
                continue;
            };
            neg += 1;
            if te > thr {
                fp += 1;
            }
        }
        (fp, neg)
    }

    fn gate_conditional_fpr_reversed(
        n: usize,
        a: f32,
        rho: f32,
        max_lag: usize,
        trials: usize,
        n_surr: usize,
        rng: &mut u64,
    ) -> (usize, usize) {
        let mut fp = 0usize;
        let mut neg = 0usize;
        for t in 0..trials {
            let seed = 0x8D4F_13A7_3C0E_92B1 ^ (t as u64).wrapping_mul(0x8D4F_13A7_3C0E_92B1);
            let (x, y, c) = gate_conditional_driver_reversed(n, a, rho, 0.0, rng);
            let Some(te) = transfer_entropy_conditional(&y, &x, &c, 1) else {
                continue;
            };
            let Some((_, _, thr)) =
                conditional_te_stats_lagged(&y, &x, &c, 1, max_lag, seed, n_surr)
            else {
                continue;
            };
            neg += 1;
            if te > thr {
                fp += 1;
            }
        }
        (fp, neg)
    }

    #[test]
    fn conditional_arx_fit_resolves_at_small_n() {
        let mut rng = 0x9E37_79B9_7F4A_7C15u64;
        for n in [64usize, 128, 256] {
            let max_lag = n / 8;
            for a in [0.0f32, 0.5, 0.9] {
                let c = gate_ar1(n, a as f64, &mut rng);
                let c2 = gate_ar1(n, a as f64, &mut rng);
                let y = gate_ar1(n, a as f64, &mut rng);
                let x = gate_ar1(n, a as f64, &mut rng);
                let conds = [LaggedCond {
                    series: c.as_slice(),
                    lag: 0,
                }];
                let conds2 = [
                    LaggedCond {
                        series: c.as_slice(),
                        lag: 0,
                    },
                    LaggedCond {
                        series: c2.as_slice(),
                        lag: 0,
                    },
                ];
                assert!(
                    ols_fit_lagged_n(&y, &[c.as_slice()], max_lag).is_some(),
                    "n={n} max_lag={max_lag} a={a}: the ARX fit resolves — the refusal arm (None) is the answer, never a silent shuffle"
                );
                assert!(
                    ols_fit_lagged_nx(&y, &conds, &x, max_lag).is_some(),
                    "n={n} max_lag={max_lag} a={a}: the full ARX fit (x-lags included) resolves — the refusal arm (None) is the answer, never a silent shuffle"
                );
                let s = arx_conditional_surrogate(&y, &x, &conds, max_lag, &mut rng)
                    .expect("the refusal is the None arm, never a silent shuffle");
                assert_eq!(s.len(), n, "n={n}: the surrogate carries the series length");
                assert!(
                    s.iter().all(|v| v.is_finite()),
                    "n={n} a={a}: the surrogate must stay finite"
                );
                assert!(
                    arx_conditional_surrogate(&y, &x[..n - 1], &conds, max_lag, &mut rng).is_none(),
                    "n={n}: an x length mismatch is the refusal arm (None), never a silent shuffle"
                );
                assert!(
                    ols_fit_lagged_nx(&y, &conds2, &x, max_lag).is_some(),
                    "n={n} max_lag={max_lag} a={a}: the two-confounder full ARX fit (x-lags included) resolves — the refusal arm (None) is the answer, never a silent shuffle"
                );
                let s2 = arx_conditional_surrogate(&y, &x, &conds2, max_lag, &mut rng)
                    .expect("the refusal is the None arm, never a silent shuffle");
                assert_eq!(
                    s2.len(),
                    n,
                    "n={n}: the two-confounder surrogate carries the series length"
                );
                assert!(
                    s2.iter().all(|v| v.is_finite()),
                    "n={n} a={a}: the two-confounder surrogate must stay finite"
                );
                assert!(
                    arx_conditional_surrogate(&y, &x[..n - 1], &conds2, max_lag, &mut rng)
                        .is_none(),
                    "n={n}: an x length mismatch is the refusal arm (None), never a silent shuffle"
                );
            }
        }
    }

    #[test]
    fn arx_restricted_surrogate_refuses_instead_of_shuffling() {
        let mut rng = 0x9E37_79B9_7F4A_7C15u64;
        let y = gate_ar1(32, 0.5, &mut rng);
        assert!(
            arx_restricted_surrogate(&y, 29, &mut rng).is_none(),
            "an order leaving fewer than 4 fit rows is the refusal arm (None), never a silent shuffle"
        );
        let mut ynan = gate_ar1(64, 0.5, &mut rng);
        ynan[9] = f32::NAN;
        assert!(
            arx_restricted_surrogate(&ynan, 2, &mut rng).is_none(),
            "a NaN-carrying series is the refusal arm (None), never a silent shuffle"
        );
    }

    #[test]
    fn arx_conditional_surrogate_refuses_when_condition_duplicates_the_driver() {
        let mut rng = 0x9E37_79B9_7F4A_7C15u64;
        let y = gate_ar1(128, 0.5, &mut rng);
        let x = gate_ar1(128, 0.5, &mut rng);
        let conds = [LaggedCond {
            series: x.as_slice(),
            lag: 0,
        }];
        assert!(
            arx_conditional_surrogate(&y, &x, &conds, 4, &mut rng).is_none(),
            "a condition series identical to the driver duplicates the x-lag columns — the refusal arm (None) is the answer, never a silent shuffle"
        );
    }

    #[test]
    fn arx_conditional_surrogate_dedups_one_series_across_lag_pairs() {
        let mut rng = 0x9E37_79B9_7F4A_7C15u64;
        let y = gate_ar1(128, 0.5, &mut rng);
        let x = gate_ar1(128, 0.5, &mut rng);
        let c = gate_ar1(128, 0.5, &mut rng);
        let conds = [
            LaggedCond {
                series: c.as_slice(),
                lag: 0,
            },
            LaggedCond {
                series: c.as_slice(),
                lag: 1,
            },
        ];
        let s = arx_conditional_surrogate(&y, &x, &conds, 4, &mut rng)
            .expect("one series across two lag pairs dedups to one history — the fit resolves");
        assert_eq!(s.len(), 128);
        assert!(s.iter().all(|v| v.is_finite()));
    }

    #[test]
    #[ignore = "conditional n=1000 FP/FN calibration gate — heavy, runs in te-gate.yml"]
    fn gate_conditional_arx_fpr_fn_n1000() {
        let n = 1000usize;
        let max_lag = 2usize;
        let n_surr = 20usize;
        let trials = 100usize;
        let a_set = [0.0f32, 0.5, 0.9];
        let rho_set = [0.0f32, 0.5, 0.9];
        let mut rng = 0xC2B2_AE3D_85EB_CA6Bu64;
        let mut rows: Vec<(f32, f32, usize, usize)> = Vec::new();
        for &rho in &rho_set {
            for &a in &a_set {
                let (fp, neg) = gate_conditional_fpr(n, a, rho, max_lag, trials, n_surr, &mut rng);
                rows.push((a, rho, fp, neg));
            }
        }
        let named: String = rows
            .iter()
            .map(|&(a, rho, fp, neg)| format!("a={a} rho={rho}: {fp}/{neg} "))
            .collect();
        for &(a, rho, fp, neg) in &rows {
            assert!(
                neg > 0,
                "conditional FP/FN gate: cell a={a} rho={rho} unmeasured (neg=0) — never a passing zero ({named})"
            );
            let fpr = 100.0 * fp as f64 / neg as f64;
            assert!(
                fpr <= 8.0,
                "conditional FP/FN gate: FPR {fpr:.2}% at a={a} rho={rho} exceeds 8% — the Arx null leaks the x-c cross-correlation ({named})"
            );
        }
        for &rho in &rho_set {
            let f0 = rows
                .iter()
                .find(|&&(a, r, _, _)| a == 0.0 && r == rho)
                .expect("a=0 cell measured");
            let f9 = rows
                .iter()
                .find(|&&(a, r, _, _)| a == 0.9 && r == rho)
                .expect("a=0.9 cell measured");
            let fpr0 = 100.0 * f0.2 as f64 / f0.3 as f64;
            let fpr9 = 100.0 * f9.2 as f64 / f9.3 as f64;
            assert!(
                fpr_rise_sigma_test(fpr0, fpr9, f0.3, f9.3),
                "conditional FP/FN gate: FPR rise {:.2}pp over a at rho={rho} exceeds 3σ of the binomial difference of the measured cells ({named})",
                fpr9 - fpr0
            );
        }
        let mut found = 0usize;
        let mut meas = 0usize;
        for t in 0..30usize {
            let seed = 0x517C_C1B7_2722_0A95u64 ^ (t as u64).wrapping_mul(0x517C_C1B7_2722_0A95);
            let (x, y, c) = gate_conditional_driver(n, 0.5, 0.5, 0.6, &mut rng);
            let Some(te) = transfer_entropy_conditional(&x, &y, &c, 1) else {
                continue;
            };
            let Some((_, _, thr)) =
                conditional_te_stats_lagged(&x, &y, &c, 1, max_lag, seed, n_surr)
            else {
                continue;
            };
            meas += 1;
            if te > thr {
                found += 1;
            }
        }
        assert!(
            meas > 0,
            "conditional FP/FN gate: no FN measurement resolved"
        );
        assert!(
            found as f64 / meas as f64 >= 0.5,
            "conditional FP/FN gate: FN arm found {found}/{meas} true couplings — below 50%"
        );
    }

    #[test]
    #[ignore = "conditional n=1000 FP/FN calibration gate (reversed x→y) — heavy, runs in te-gate.yml"]
    fn gate_conditional_arx_fpr_fn_reversed_n1000() {
        let n = 1000usize;
        let max_lag = 2usize;
        let n_surr = 20usize;
        let trials = 100usize;
        let a_set = [0.0f32, 0.5, 0.9];
        let rho_set = [0.0f32, 0.5, 0.9];
        let mut rng = 0x3A71_9C2E_85B4_D60Fu64;
        let mut rows: Vec<(f32, f32, usize, usize)> = Vec::new();
        for &rho in &rho_set {
            for &a in &a_set {
                let (fp, neg) =
                    gate_conditional_fpr_reversed(n, a, rho, max_lag, trials, n_surr, &mut rng);
                rows.push((a, rho, fp, neg));
            }
        }
        let named: String = rows
            .iter()
            .map(|&(a, rho, fp, neg)| format!("a={a} rho={rho}: {fp}/{neg} "))
            .collect();
        for &(a, rho, fp, neg) in &rows {
            assert!(
                neg > 0,
                "conditional FP/FN gate (reversed): cell a={a} rho={rho} unmeasured (neg=0) — never a passing zero ({named})"
            );
            let fpr = 100.0 * fp as f64 / neg as f64;
            assert!(
                fpr <= 8.0,
                "conditional FP/FN gate (reversed): FPR {fpr:.2}% at a={a} rho={rho} exceeds 8% — the Arx null leaks the x-c cross-correlation ({named})"
            );
        }
        for &rho in &rho_set {
            let f0 = rows
                .iter()
                .find(|&&(a, r, _, _)| a == 0.0 && r == rho)
                .expect("a=0 cell measured");
            let f9 = rows
                .iter()
                .find(|&&(a, r, _, _)| a == 0.9 && r == rho)
                .expect("a=0.9 cell measured");
            let fpr0 = 100.0 * f0.2 as f64 / f0.3 as f64;
            let fpr9 = 100.0 * f9.2 as f64 / f9.3 as f64;
            assert!(
                fpr_rise_sigma_test(fpr0, fpr9, f0.3, f9.3),
                "conditional FP/FN gate (reversed): FPR rise {:.2}pp over a at rho={rho} exceeds 3σ of the binomial difference of the measured cells ({named})",
                fpr9 - fpr0
            );
        }
        let mut found = 0usize;
        let mut meas = 0usize;
        for t in 0..30usize {
            let seed = 0x7C35_4E91_A28B_06D4u64 ^ (t as u64).wrapping_mul(0x7C35_4E91_A28B_06D4);
            let (x, y, c) = gate_conditional_driver_reversed(n, 0.5, 0.5, 0.6, &mut rng);
            let Some(te) = transfer_entropy_conditional(&y, &x, &c, 1) else {
                continue;
            };
            let Some((_, _, thr)) =
                conditional_te_stats_lagged(&y, &x, &c, 1, max_lag, seed, n_surr)
            else {
                continue;
            };
            meas += 1;
            if te > thr {
                found += 1;
            }
        }
        assert!(
            meas > 0,
            "conditional FP/FN gate (reversed): no FN measurement resolved"
        );
        assert!(
            found as f64 / meas as f64 >= 0.5,
            "conditional FP/FN gate (reversed): FN arm found {found}/{meas} true couplings — below 50%"
        );
    }

    #[test]
    #[ignore = "conditional n=1000 FP/FN calibration gate — heavy, runs in te-gate.yml"]
    fn gate_conditional_arx_2_fpr_n1000() {
        let n = 1000usize;
        let max_lag = 2usize;
        let n_surr = 20usize;
        let trials = 40usize;
        let rho = 0.9f32;
        let a_set = [0.0f32, 0.5, 0.9];
        let mut rng = 0x6A2B_7A5B_3C1D_9E4Fu64;
        let mut rows: Vec<(f32, usize, usize)> = Vec::new();
        for &a in &a_set {
            let mut fp = 0usize;
            let mut neg = 0usize;
            for t in 0..trials {
                let seed = 0x9E37_79B9_7F4A_7C15 ^ (t as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15);
                let (x, y, c1, c2) = gate_conditional_driver_2(n, a, rho, 0.0, &mut rng);
                let Some(te) = transfer_entropy_conditional_2(&x, &y, &c1, &c2, 1) else {
                    continue;
                };
                let Some((_, _, thr)) = conditional_te_stats_lagged_2(
                    &x,
                    &y,
                    &c1,
                    &c2,
                    TeStats2Params {
                        lag: 1,
                        max_lag,
                        seed,
                        n_surr,
                    },
                ) else {
                    continue;
                };
                neg += 1;
                if te > thr {
                    fp += 1;
                }
            }
            rows.push((a, fp, neg));
        }
        let named: String = rows
            .iter()
            .map(|&(a, fp, neg)| format!("a={a}: {fp}/{neg} "))
            .collect();
        for &(a, fp, neg) in &rows {
            assert!(
                neg > 0,
                "conditional FP/FN gate (2): cell a={a} unmeasured (neg=0) — never a passing zero ({named})"
            );
            let fpr = 100.0 * fp as f64 / neg as f64;
            assert!(
                fpr <= 8.0,
                "conditional FP/FN gate (2): FPR {fpr:.2}% at a={a} rho={rho} exceeds 8% — the two-confounder Arx null leaks ({named})"
            );
        }
        let f0 = rows
            .iter()
            .find(|&&(a, _, _)| a == 0.0)
            .expect("a=0 cell measured");
        let f9 = rows
            .iter()
            .find(|&&(a, _, _)| a == 0.9)
            .expect("a=0.9 cell measured");
        let fpr0 = 100.0 * f0.1 as f64 / f0.2 as f64;
        let fpr9 = 100.0 * f9.1 as f64 / f9.2 as f64;
        assert!(
            fpr_rise_sigma_test(fpr0, fpr9, f0.2, f9.2),
            "conditional FP/FN gate (2): FPR rise {:.2}pp over a at rho={rho} exceeds 3σ of the binomial difference of the measured cells ({named})",
            fpr9 - fpr0
        );
        let mut found = 0usize;
        let mut meas = 0usize;
        for t in 0..30usize {
            let seed = 0x517C_C1B7_2722_0A95u64 ^ (t as u64).wrapping_mul(0x517C_C1B7_2722_0A95);
            let (x, y, c1, c2) = gate_conditional_driver_2(n, 0.5, 0.5, 0.6, &mut rng);
            let Some(te) = transfer_entropy_conditional_2(&x, &y, &c1, &c2, 1) else {
                continue;
            };
            let Some((_, _, thr)) = conditional_te_stats_lagged_2(
                &x,
                &y,
                &c1,
                &c2,
                TeStats2Params {
                    lag: 1,
                    max_lag,
                    seed,
                    n_surr,
                },
            ) else {
                continue;
            };
            meas += 1;
            if te > thr {
                found += 1;
            }
        }
        assert!(
            meas > 0,
            "conditional FP/FN gate (2): no FN measurement resolved"
        );
        assert!(
            found as f64 / meas as f64 >= 0.5,
            "conditional FP/FN gate (2): FN arm found {found}/{meas} true couplings — below 50%"
        );
    }

    fn gate_s60_f(x: f32) -> f32 {
        x
    }

    type PeakList = Vec<(usize, usize, usize, f32)>;

    fn gate_s60_topology(n_chan: usize, rng: &mut u64) -> (Vec<f32>, PeakList) {
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
        for row in x.iter().take(n_chan) {
            let col: Vec<f32> = row[burn..].to_vec();
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
            let mut drawn: Option<(Vec<Vec<f32>>, PeakList)> = None;
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
                let true_parents: Vec<LaggedCond> = links
                    .iter()
                    .filter(|&&(d, t, l, _)| (d, t, l) != (driver, target, lag) && t == target)
                    .map(|&(d, _, l, _)| LaggedCond {
                        series: refs[d],
                        lag: l,
                    })
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
                        TeSurrogateParams {
                            lag,
                            max_lag: 2,
                            bins: 4,
                            seed,
                            n_surr: 100,
                            null: TeNull::Arx,
                            block: 0,
                            est: TeEstimator::Ksg,
                            k: 4,
                        },
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
    fn gate_wgsl_ksg_parity_real_and_surrogate_against_cpu_reference() {
        use std::sync::Arc;
        use std::sync::atomic::{AtomicBool, Ordering};
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());
        let adapter_options = wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::None,
            compatible_surface: None,
            force_fallback_adapter: false,
        };
        let adapter = match pollster::block_on(instance.request_adapter(&adapter_options)) {
            Some(a) => a,
            None => {
                eprintln!("compute-only device request returned void — wgsl ksg parity skipped");
                return;
            }
        };
        let (device, queue) = match pollster::block_on(
            adapter.request_device(&wgpu::DeviceDescriptor::default(), None),
        ) {
            Ok(dq) => dq,
            Err(e) => {
                eprintln!("device request returned: {}", e);
                return;
            }
        };
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(crate::mathematikerin::TE_WGSL.into()),
        });
        let te_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                {
                    let mut e =
                        crate::mathematikerin::storage_entry(true, wgpu::ShaderStages::COMPUTE);
                    e.binding = 0;
                    e
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                {
                    let mut e =
                        crate::mathematikerin::storage_entry(false, wgpu::ShaderStages::COMPUTE);
                    e.binding = 2;
                    e
                },
            ],
        });
        let te_pipe_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&te_layout],
            push_constant_ranges: &[],
        });
        let te_pipe = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: None,
            layout: Some(&te_pipe_layout),
            module: &module,
            entry_point: Some("te_compute"),
            compilation_options: Default::default(),
            cache: None,
        });
        let series_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: crate::mathematikerin::TE_SERIES_BYTES,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let param_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: 16,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let out_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: crate::mathematikerin::te_verdict_bytes(TE_KSG_K as u32),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });
        let read_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: crate::mathematikerin::te_verdict_bytes(TE_KSG_K as u32),
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let te_bind = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &te_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: series_buf.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: param_buf.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: out_buf.as_entire_binding(),
                },
            ],
        });
        let n = 200usize;
        let mut x = vec![0f32; n];
        let mut y = vec![0f32; n];
        for (t, slot) in y.iter_mut().enumerate() {
            *slot = (t as f32 * 0.5).sin();
        }
        for t in 0..n - 1 {
            x[t + 1] = 0.5 * x[t] + 0.6 * y[t];
        }
        let seed = 42u64;
        let mut data = vec![0f32; 12 * crate::mathematikerin::TE_SERIES_STRIDE];
        data[0..n].copy_from_slice(&x);
        data[crate::mathematikerin::TE_SERIES_STRIDE..crate::mathematikerin::TE_SERIES_STRIDE + n]
            .copy_from_slice(&y);
        let mut rng = seed.wrapping_add(0x9e3779b97f4a7c15);
        let mut surrogates: Vec<Vec<f32>> = Vec::with_capacity(10);
        for s in 0..10 {
            let surr = phase_randomized_surrogate(&y, &mut rng);
            let off = (2 + s) * crate::mathematikerin::TE_SERIES_STRIDE;
            data[off..off + n].copy_from_slice(&surr);
            surrogates.push(surr);
        }
        queue.write_buffer(&series_buf, 0, &crate::mathematikerin::le_bytes_f32(&data));
        let max_lag = (n as f64 / Φ) as u32;
        let param = [n as u32, max_lag, 1.0f32.to_bits(), TE_KSG_K as u32];
        let mut pb = [0u8; 16];
        for (i, p) in param.iter().enumerate() {
            pb[i * 4..i * 4 + 4].copy_from_slice(&p.to_le_bytes());
        }
        queue.write_buffer(&param_buf, 0, &pb);
        let mut enc = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
        {
            let mut pass = enc.begin_compute_pass(&wgpu::ComputePassDescriptor::default());
            pass.set_pipeline(&te_pipe);
            pass.set_bind_group(0, &te_bind, &[]);
            pass.dispatch_workgroups(1, 1, 1);
        }
        enc.copy_buffer_to_buffer(
            &out_buf,
            0,
            &read_buf,
            0,
            crate::mathematikerin::te_verdict_bytes(TE_KSG_K as u32),
        );
        queue.submit(std::iter::once(enc.finish()));
        let mapped = Arc::new(AtomicBool::new(false));
        let m2 = mapped.clone();
        let slice = read_buf.slice(..);
        slice.map_async(wgpu::MapMode::Read, move |r| {
            m2.store(r.is_ok(), Ordering::SeqCst);
        });
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
        while !mapped.load(Ordering::SeqCst) && std::time::Instant::now() < deadline {
            device.poll(wgpu::Maintain::Poll);
        }
        assert!(
            mapped.load(Ordering::SeqCst),
            "wgsl ksg parity readback returned void"
        );
        let mapped_data = slice.get_mapped_range();
        assert!(
            crate::mathematikerin::te_verdict_bytes(TE_KSG_K as u32) >= 384,
            "the K>0 verdict readback must carry the KSG columns (384 B)"
        );
        assert_eq!(
            mapped_data.len(),
            crate::mathematikerin::te_verdict_bytes(TE_KSG_K as u32) as usize,
            "the readback size must follow te_verdict_bytes"
        );
        let mut verdict =
            [0f32; crate::mathematikerin::te_verdict_bytes(TE_KSG_K as u32) as usize / 4];
        for k in 0..verdict.len() {
            let mut b = [0u8; 4];
            b.copy_from_slice(&mapped_data[k * 4..k * 4 + 4]);
            verdict[k] = f32::from_le_bytes(b);
        }
        drop(mapped_data);
        read_buf.unmap();
        assert_eq!(
            verdict[10], 1.0,
            "the kde real-pair slot must stay valid alongside the ksg mirror"
        );
        assert_eq!(
            verdict[75], 1.0,
            "the wgsl ksg real-pair slot is absent at k={}",
            TE_KSG_K
        );
        let tau_x = verdict[0] as usize;
        let tau_y = verdict[6] as usize;
        let gpu_te = verdict[74];
        let xf: Vec<f64> = x.iter().map(|&v| v as f64).collect();
        let yf: Vec<f64> = y.iter().map(|&v| v as f64).collect();
        let emb_x = embed_series(&xf, tau_x, 3);
        let emb_y = embed_series(&yf, tau_y, 3);
        let cpu_opt = transfer_entropy_embedded_ksg(&xf, &emb_x, &emb_y, tau_x, tau_y, TE_KSG_K);
        let cpu_te = cpu_opt.expect("the KSG reference carries a TE at the GPU lags");
        let gap = (gpu_te as f64 - cpu_te).abs();
        assert!(
            gap < 0.05 + 0.05 * cpu_te.abs(),
            "wgsl ksg parity: gpu {} cpu {} gap {}",
            gpu_te,
            cpu_te,
            gap
        );
        let mut ksg_surrogates = 0usize;
        for s in 2..12 {
            if verdict[73 + 2 * s] != 1.0 {
                continue;
            }
            let tau_s = verdict[s * 6] as usize;
            let gpu_s = verdict[72 + 2 * s];
            let ysf: Vec<f64> = surrogates[s - 2].iter().map(|&v| v as f64).collect();
            let emb_s = embed_series(&ysf, tau_s, 3);
            let cpu_s_opt =
                transfer_entropy_embedded_ksg(&xf, &emb_x, &emb_s, tau_x, tau_s, TE_KSG_K);
            let cpu_s = cpu_s_opt.expect("the KSG reference carries a surrogate TE at GPU lags");
            let gap_s = (gpu_s as f64 - cpu_s).abs();
            assert!(
                gap_s < 0.05 + 0.05 * cpu_s.abs(),
                "wgsl ksg surrogate {s} parity: gpu {} cpu {} gap {}",
                gpu_s,
                cpu_s,
                gap_s
            );
            ksg_surrogates += 1;
        }
        assert!(
            ksg_surrogates >= 2,
            "wgsl ksg surrogate parity holds on {ksg_surrogates} series — the null mirror needs at least two"
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
            for (t, ct) in c.iter_mut().enumerate().skip(s) {
                *ct += amp * (-((t - s) as f32) / tau).exp();
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

    #[ignore = "power study pending at n=240 (43% vs the 50% floor) — runs in te-gate.yml"]
    #[test]
    fn flare_envelope_conditional_keeps_true_coupling() {
        let n = 240;
        let alpha = 0.90f32;
        let trials = 30usize;
        let mut found = 0usize;
        let mut meas = 0usize;
        for trial in 0..trials {
            let seed =
                0x6A2B_7A5B_3C1D_9E4Fu64 ^ (trial as u64).wrapping_mul(0x517C_C1B7_2722_0A95);
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
            let mut y_ind = vec![0f32; n];
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
            let Some(te_c) = transfer_entropy_conditional(&x, &y, &c, 1) else {
                continue;
            };
            let Some((_, _, thr_c)) =
                conditional_te_stats_lagged(&x, &y, &c, 1, 1, seed ^ 0x9E37_79B9_7F4A_7C15, 256)
            else {
                continue;
            };
            meas += 1;
            if te_c > thr_c {
                found += 1;
            }
        }
        assert!(
            meas > 0,
            "flare-envelope gate: no true-coupling realization resolved"
        );
        assert!(
            found as f64 / meas as f64 >= 0.5,
            "flare-envelope gate: true coupling beyond the shared envelope survived conditioning in only {found}/{meas} realizations — below 50%"
        );
    }

    #[ignore = "print-only power probe for the flare-envelope gate — runs in te-gate.yml"]
    #[test]
    fn flare_envelope_power_probe() {
        let alpha = 0.90f32;
        let trials = 30usize;
        for &n in &[400usize, 600usize, 1000usize] {
            let mut found = 0usize;
            let mut meas = 0usize;
            for trial in 0..trials {
                let seed =
                    0x6A2B_7A5B_3C1D_9E4Fu64 ^ (trial as u64).wrapping_mul(0x517C_C1B7_2722_0A95);
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
                let mut y_ind = vec![0f32; n];
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
                let Some(te_c) = transfer_entropy_conditional(&x, &y, &c, 1) else {
                    continue;
                };
                let Some((_, _, thr_c)) = conditional_te_stats_lagged(
                    &x,
                    &y,
                    &c,
                    1,
                    1,
                    seed ^ 0x9E37_79B9_7F4A_7C15,
                    256,
                ) else {
                    continue;
                };
                meas += 1;
                if te_c > thr_c {
                    found += 1;
                }
            }
            if meas > 0 {
                println!(
                    "flare power probe: n={n} found={found} meas={meas} power={:.3}",
                    found as f64 / meas as f64
                );
            } else {
                println!("flare power probe: n={n} found=0 meas=0 (no realization resolved)");
            }
        }
    }

    #[test]
    fn synthetic_dag_recovers_known_direction() {
        let n = 240;
        let alpha = 0.90f32;
        let trials = 30usize;
        let mut found_ab = 0usize;
        let mut found_ba = 0usize;
        let mut meas = 0usize;
        for trial in 0..trials {
            let seed =
                0x9E4F_6A2B_7A5B_3C1Du64 ^ (trial as u64).wrapping_mul(0x517C_C1B7_2722_0A95);
            let mut rng = seed;
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
            let null_seed = seed ^ 0x9E37_79B9_7F4A_7C15;
            let Some(te_ab) = transfer_entropy_conditional(&b, &a, &z, 1) else {
                continue;
            };
            let Some(te_ba) = transfer_entropy_conditional(&a, &b, &z, 1) else {
                continue;
            };
            let Some((_, _, thr_ab)) =
                conditional_te_stats_lagged(&b, &a, &z, 1, 1, null_seed, 256)
            else {
                continue;
            };
            let Some((_, _, thr_ba)) =
                conditional_te_stats_lagged(&a, &b, &z, 1, 1, null_seed, 256)
            else {
                continue;
            };
            meas += 1;
            if te_ab > thr_ab {
                found_ab += 1;
            }
            if te_ba > thr_ba {
                found_ba += 1;
            }
        }
        assert!(meas > 0, "synthetic DAG: no direction realization resolved");
        assert!(
            found_ab as f64 / meas as f64 >= 0.5,
            "synthetic DAG: the known true edge A->B was recovered in only {found_ab}/{meas} realizations — below 50%"
        );
        assert!(
            found_ba as f64 / meas as f64 <= 0.5,
            "synthetic DAG: the false reverse edge B->A was detected in {found_ba}/{meas} realizations — above 50%"
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
        let (mean, sd, threshold) = conditional_te_stats_lagged_2(
            &x,
            &y,
            &c1,
            &c2,
            TeStats2Params {
                lag: 1,
                max_lag: 1,
                seed: 0x9E37_79B9_7F4A_7C15,
                n_surr: 10,
            },
        )
        .expect("stats resolve");
        assert!(mean.is_finite() && sd.is_finite() && threshold.is_finite());
        assert!(threshold >= mean);
    }

    #[test]
    fn binned_te_causal_positive() {
        let n = 200;
        let mut x = vec![0f32; n];
        let mut y = vec![0f32; n];
        for (t, yt) in y.iter_mut().enumerate() {
            *yt = (t as f32 * 0.7).sin();
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
        for (t, yt) in yc.iter_mut().enumerate() {
            *yt = (t as f32 * 0.7).sin();
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
        let te_1 = transfer_entropy_conditional_binned_n(
            &x,
            &y,
            &[LaggedCond {
                series: &c1,
                lag: 0,
            }],
            1,
            3,
        )
        .expect("one-confounder binned TE resolves");
        let te_2 = transfer_entropy_conditional_binned_n(
            &x,
            &y,
            &[
                LaggedCond {
                    series: &c1,
                    lag: 0,
                },
                LaggedCond {
                    series: &c2,
                    lag: 0,
                },
            ],
            1,
            3,
        )
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
        let te_ab = transfer_entropy_conditional_binned_n(
            &b,
            &a,
            &[LaggedCond { series: &z, lag: 0 }],
            1,
            4,
        )
        .expect("binned A->B resolves");
        let te_ba = transfer_entropy_conditional_binned_n(
            &a,
            &b,
            &[LaggedCond { series: &z, lag: 0 }],
            1,
            4,
        )
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
        let bin_ab = transfer_entropy_conditional_binned_n(
            &b,
            &a,
            &[LaggedCond { series: &z, lag: 0 }],
            1,
            4,
        )
        .expect("binned A->B resolves");
        let bin_ba = transfer_entropy_conditional_binned_n(
            &a,
            &b,
            &[LaggedCond { series: &z, lag: 0 }],
            1,
            4,
        )
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
        let conds = [
            LaggedCond {
                series: &c1,
                lag: 0,
            },
            LaggedCond {
                series: &c2,
                lag: 0,
            },
        ];
        let te_c = transfer_entropy_conditional_binned_n(&x, &y, &conds, 1, bins)
            .expect("two-driver binned TE resolves");
        let (_, _, thr) = conditional_te_stats_lagged_n(
            &x,
            &y,
            &conds,
            TeStatsParams {
                lag: 1,
                max_lag: 1,
                bins,
                seed: 0x9E37_79B9_7F4A_7C15,
                n_surr: 256,
                null: TeNull::Arx,
            },
        )
        .expect("lagged N-dim null resolves");
        assert!(
            te_c <= thr,
            "binned N-dim null does not leak on the multi-driver impulsive confound, got te {te_c} thr {thr}"
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
        let conds = [
            LaggedCond {
                series: &c1,
                lag: 0,
            },
            LaggedCond {
                series: &c2,
                lag: 0,
            },
        ];
        let te_c = transfer_entropy_conditional_binned_n(&y, &x, &conds, 1, bins)
            .expect("two-driver binned TE resolves");
        let (_, _, thr) = conditional_te_stats_lagged_n(
            &y,
            &x,
            &conds,
            TeStatsParams {
                lag: 1,
                max_lag: 1,
                bins,
                seed: 0x9E37_79B9_7F4A_7C15,
                n_surr: 10,
                null: TeNull::Arx,
            },
        )
        .expect("lagged N-dim null resolves");
        assert!(
            te_c > thr,
            "binned N-dim null keeps the true coupling beyond the shared drivers, got te {te_c} thr {thr}"
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
        for ai in &mut a_ind {
            *ai = noise(&mut rng);
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
                0.9 * b[t - 1] + 0.6 * z[t - 1] + 0.6 * a[t - 1] + 0.3 * noise(&mut rng)
            };
        }
        let series: [&[f32]; 3] = [&z, &a, &b];
        let links = pcmci_links(
            &series,
            PcmciParams {
                max_lag: 2,
                null_lag: 12,
                bins: 4,
                seed: 0x9E37_79B9_7F4A_7C15,
                n_surr: 100,
                null: TeNull::Arx,
                block: 0,
                est: TeEstimator::Ksg,
                k: 4,
                p_max: 2,
                alpha: 0.05,
            },
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
