pub fn erfc(x: f64) -> f64 {
    if x == f64::INFINITY {
        return 0.0;
    }
    if x == f64::NEG_INFINITY {
        return 2.0;
    }
    if x >= 0.0 {
        erfc_positive(x)
    } else {
        2.0 - erfc_positive(-x)
    }
}

fn erfc_positive(x: f64) -> f64 {
    let t = 1.0 / (1.0 + 0.3275911 * x);
    let poly = t
        * (0.254829592
            + t * (-0.284496736 + t * (1.421413741 + t * (-1.453152027 + t * 1.061405429))));
    poly * (-x * x).exp()
}

pub fn inv_normal(p: f64) -> f64 {
    if !(p > 0.0 && p < 1.0) {
        return f64::NAN;
    }

    const A1: f64 = -3.969683028665376e+01;
    const A2: f64 = 2.209460984245205e+02;
    const A3: f64 = -2.759285104469687e+02;
    const A4: f64 = 1.383577518672690e+02;
    const A5: f64 = -3.066479806614716e+01;
    const A6: f64 = 2.506628277459239e+00;

    const B1: f64 = -5.447609879822406e+01;
    const B2: f64 = 1.615858368580409e+02;
    const B3: f64 = -1.556989798598866e+02;
    const B4: f64 = 6.680131188771972e+01;
    const B5: f64 = -1.328068155288572e+01;

    const C1: f64 = -7.784894002430293e-03;
    const C2: f64 = -3.223964580411365e-01;
    const C3: f64 = -2.400758277161838e+00;
    const C4: f64 = -2.549732539343734e+00;
    const C5: f64 = 4.374664141464968e+00;
    const C6: f64 = 2.938163982698783e+00;

    const D1: f64 = 7.784695709041462e-03;
    const D2: f64 = 3.224671290700398e-01;
    const D3: f64 = 2.445134137142996e+00;
    const D4: f64 = 3.754408661907416e+00;

    const P_LOW: f64 = 0.02425;

    let mut x = if p < P_LOW {
        let q = (-2.0 * p.ln()).sqrt();
        (((((C1 * q + C2) * q + C3) * q + C4) * q + C5) * q + C6)
            / ((((D1 * q + D2) * q + D3) * q + D4) * q + 1.0)
    } else if p <= 1.0 - P_LOW {
        let q = p - 0.5;
        let r = q * q;
        (((((A1 * r + A2) * r + A3) * r + A4) * r + A5) * r + A6) * q
            / (((((B1 * r + B2) * r + B3) * r + B4) * r + B5) * r + 1.0)
    } else {
        let q = (-2.0 * (1.0 - p).ln()).sqrt();
        -(((((C1 * q + C2) * q + C3) * q + C4) * q + C5) * q + C6)
            / ((((D1 * q + D2) * q + D3) * q + D4) * q + 1.0)
    };

    let e = 0.5 * erfc(-x / std::f64::consts::SQRT_2) - p;
    let u = e * std::f64::consts::TAU.sqrt() * (x * x / 2.0).exp();
    x -= u / (1.0 + x * u / 2.0);

    x
}

pub fn normal_scores(xs: &[f64]) -> Vec<f64> {
    let n = xs.len();
    if n == 0 {
        return Vec::new();
    }
    if n == 1 {
        return vec![0.0];
    }

    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by(|&a, &b| xs[a].total_cmp(&xs[b]));

    let mut rank = vec![0.0_f64; n];
    let mut i = 0;
    while i < n {
        let mut j = i;
        while j + 1 < n && xs[order[j + 1]] == xs[order[i]] {
            j += 1;
        }
        let mid = ((i + 1) + (j + 1)) as f64 / 2.0;
        for k in i..=j {
            rank[order[k]] = mid;
        }
        i = j + 1;
    }

    let denom = n as f64 + 0.25;
    rank.iter()
        .map(|&r| inv_normal((r - 0.375) / denom))
        .collect()
}

#[derive(Clone, Copy, Debug)]
pub struct PartialCorrelation {
    pub r: f64,
    pub p: f64,
    pub n: usize,
    pub dim: usize,
}

pub fn partial_correlation(x: &[f64], y: &[f64], conds: &[&[f64]]) -> Option<PartialCorrelation> {
    let n = x.len();
    if y.len() != n {
        return None;
    }
    if conds.iter().any(|c| c.len() != n) {
        return None;
    }
    if x.iter().chain(y.iter()).any(|v| !v.is_finite()) {
        return None;
    }
    if conds.iter().any(|c| c.iter().any(|v| !v.is_finite())) {
        return None;
    }

    let dim = conds.len();
    if n <= dim + 3 {
        return None;
    }

    let m = dim + 2;
    let mut cols: Vec<&[f64]> = Vec::with_capacity(m);
    cols.push(x);
    cols.push(y);
    for c in conds {
        cols.push(c);
    }

    let nf = n as f64;
    let mut means = vec![0.0_f64; m];
    for (k, col) in cols.iter().enumerate() {
        means[k] = col.iter().sum::<f64>() / nf;
    }

    let mut sds = vec![0.0_f64; m];
    for (k, col) in cols.iter().enumerate() {
        let var = col
            .iter()
            .map(|&v| (v - means[k]) * (v - means[k]))
            .sum::<f64>()
            / nf;
        if !(var > 0.0) {
            return None;
        }
        sds[k] = var.sqrt();
    }

    let mut corr = vec![vec![0.0_f64; m]; m];
    for a in 0..m {
        corr[a][a] = 1.0;
        for b in (a + 1)..m {
            let cov = (0..n)
                .map(|t| (cols[a][t] - means[a]) * (cols[b][t] - means[b]))
                .sum::<f64>()
                / nf;
            let c = (cov / (sds[a] * sds[b])).clamp(-1.0, 1.0);
            corr[a][b] = c;
            corr[b][a] = c;
        }
    }

    let precision = invert(&corr, m)?;

    let denom = (precision[0][0] * precision[1][1]).sqrt();
    if !(denom > 0.0) {
        return None;
    }
    let r = (-precision[0][1] / denom).clamp(-1.0, 1.0);

    let dof = (n - dim - 3) as f64;
    let z = 0.5 * ((1.0 + r) / (1.0 - r)).ln() * dof.sqrt();
    let p = erfc(z.abs() / std::f64::consts::SQRT_2);

    Some(PartialCorrelation { r, p, n, dim })
}

fn invert(a: &[Vec<f64>], m: usize) -> Option<Vec<Vec<f64>>> {
    let mut aug = vec![vec![0.0_f64; 2 * m]; m];
    for i in 0..m {
        for j in 0..m {
            aug[i][j] = a[i][j];
        }
        aug[i][m + i] = 1.0;
    }

    for col in 0..m {
        let mut pivot = col;
        let mut best = aug[col][col].abs();
        for row in (col + 1)..m {
            let v = aug[row][col].abs();
            if v > best {
                best = v;
                pivot = row;
            }
        }
        if best < 1e-12 {
            return None;
        }
        if pivot != col {
            aug.swap(pivot, col);
        }

        let inv_p = 1.0 / aug[col][col];
        for j in 0..(2 * m) {
            aug[col][j] *= inv_p;
        }
        for row in 0..m {
            if row == col {
                continue;
            }
            let factor = aug[row][col];
            if factor == 0.0 {
                continue;
            }
            for j in 0..(2 * m) {
                aug[row][j] -= factor * aug[col][j];
            }
        }
    }

    let mut out = vec![vec![0.0_f64; m]; m];
    for i in 0..m {
        for j in 0..m {
            out[i][j] = aug[i][m + j];
        }
    }
    Some(out)
}

pub trait CiTest {
    fn p(&self, x: &[f64], y: &[f64], conds: &[&[f64]]) -> Option<f64>;
}

pub struct ParCorr;

impl CiTest for ParCorr {
    fn p(&self, x: &[f64], y: &[f64], conds: &[&[f64]]) -> Option<f64> {
        partial_correlation(x, y, conds).map(|pc| pc.p)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Rng(u64);

    impl Rng {
        fn new(seed: u64) -> Self {
            Rng(seed)
        }

        fn next_u64(&mut self) -> u64 {
            let mut x = self.0;
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            self.0 = x;
            x
        }

        fn uniform(&mut self) -> f64 {
            (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
        }

        fn normal(&mut self) -> f64 {
            let u1 = self.uniform().max(f64::MIN_POSITIVE);
            let u2 = self.uniform();
            (-2.0 * u1.ln()).sqrt() * (std::f64::consts::TAU * u2).cos()
        }
    }

    #[test]
    fn erfc_matches_known_values() {
        assert!((erfc(0.0) - 1.0).abs() < 1e-6);
        assert!((erfc(1.0) - 0.15729921).abs() < 1e-5);
        assert!((erfc(-1.0) - 1.84270079).abs() < 1e-5);
    }

    #[test]
    fn inv_normal_matches_known_values() {
        assert!((inv_normal(0.975) - 1.959964).abs() < 1e-4);
        assert!(inv_normal(0.5).abs() < 1e-6);
        assert!(inv_normal(0.0).is_nan());
    }

    #[test]
    fn normal_scores_are_centered() {
        let mut rng = Rng::new(42);
        let xs: Vec<f64> = (0..200).map(|_| rng.normal()).collect();
        let scores = normal_scores(&xs);
        assert_eq!(scores.len(), xs.len());
        let mean = scores.iter().sum::<f64>() / scores.len() as f64;
        assert!(mean.abs() < 1e-9, "mean {}", mean);
        assert!(scores.iter().all(|s| s.is_finite()));
    }

    #[test]
    fn chain_removes_the_middle() {
        let mut rng = Rng::new(7);
        let n = 300;
        let mut x = vec![0.0; n];
        let mut y = vec![0.0; n];
        let mut z = vec![0.0; n];
        for t in 0..n {
            x[t] = rng.normal();
            y[t] = 0.7 * x[t] + 0.3 * rng.normal();
            z[t] = 0.7 * y[t] + 0.3 * rng.normal();
        }
        let test = ParCorr;
        assert!(test.p(&x, &z, &[&y]).unwrap() > 0.05);
        assert!(test.p(&x, &z, &[]).unwrap() < 0.05);
    }

    #[test]
    fn confounder_is_removed() {
        let mut rng = Rng::new(11);
        let n = 300;
        let mut z = vec![0.0; n];
        let mut x = vec![0.0; n];
        let mut y = vec![0.0; n];
        for t in 0..n {
            z[t] = rng.normal();
            x[t] = 0.6 * z[t] + 0.3 * rng.normal();
            y[t] = 0.6 * z[t] + 0.3 * rng.normal();
        }
        let test = ParCorr;
        assert!(test.p(&x, &y, &[&z]).unwrap() > 0.05);
        assert!(test.p(&x, &y, &[]).unwrap() < 0.05);
    }

    #[test]
    fn direct_edge_is_kept() {
        let mut rng = Rng::new(23);
        let n = 300;
        let mut x = vec![0.0; n];
        let mut y = vec![0.0; n];
        for t in 0..n {
            x[t] = rng.normal();
            y[t] = 0.7 * x[t] + 0.3 * rng.normal();
        }
        let test = ParCorr;
        assert!(test.p(&x, &y, &[]).unwrap() < 0.01);
    }

    #[test]
    fn too_few_samples_returns_none() {
        assert!(partial_correlation(&[1.0, 2.0], &[1.0, 2.0], &[]).is_none());
    }
}
