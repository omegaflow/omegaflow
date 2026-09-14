use crate::stats::{mean, sample_sd};

pub const MIN_N: usize = 4;

pub struct NullBand {
    pub n_perms: usize,
    pub mean: f64,
    pub sd: f64,
    pub band_lo: f64,
    pub band_hi: f64,
}

pub struct ScatterResult {
    pub n: usize,
    pub sigma0: f64,
    pub sigma1: f64,
    pub rho: f64,
    pub null_mean: f64,
    pub null_sd: f64,
    pub null_band_lo: f64,
    pub null_band_hi: f64,
    pub beyond_null: bool,
}

impl ScatterResult {
    pub fn verdict_word(&self) -> &'static str {
        if self.rho < self.null_band_lo {
            "beyond null, reduction"
        } else if self.rho > self.null_band_hi {
            "beyond null, inflation"
        } else {
            "within null"
        }
    }
}

fn finite_pair(p: &(f64, f64)) -> bool {
    p.0.is_finite() && p.1.is_finite()
}

pub fn sigma0(pairs: &[(f64, f64)]) -> Option<f64> {
    if pairs.len() < 2 {
        return None;
    }
    if !pairs.iter().all(finite_pair) {
        return None;
    }
    let offsets: Vec<f64> = pairs.iter().map(|p| p.1).collect();
    sample_sd(&offsets)
}

pub fn ols_residual_sd(pairs: &[(f64, f64)]) -> Option<f64> {
    if pairs.len() < MIN_N {
        return None;
    }
    if !pairs.iter().all(finite_pair) {
        return None;
    }
    let n = pairs.len() as f64;
    let sx: f64 = pairs.iter().map(|p| p.0).sum();
    let sy: f64 = pairs.iter().map(|p| p.1).sum();
    let sxx: f64 = pairs.iter().map(|p| p.0 * p.0).sum();
    let sxy: f64 = pairs.iter().map(|p| p.0 * p.1).sum();
    let denom = n * sxx - sx * sx;
    if !denom.is_finite() || denom.abs() < f64::EPSILON {
        return None;
    }
    let slope = (n * sxy - sx * sy) / denom;
    let intercept = (sy - slope * sx) / n;
    let residuals: Vec<f64> = pairs
        .iter()
        .map(|p| p.1 - (intercept + slope * p.0))
        .collect();
    sample_sd(&residuals)
}

pub fn rho(pairs: &[(f64, f64)]) -> Option<f64> {
    let s0 = sigma0(pairs)?;
    let s1 = ols_residual_sd(pairs)?;
    if !(s0.is_finite() && s0 > 0.0) {
        return None;
    }
    Some(s1 / s0)
}

fn shuffle(v: &mut [f64], rng: &mut u64) {
    for i in (1..v.len()).rev() {
        *rng = rng
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        let j = ((*rng >> 33) as usize) % (i + 1);
        v.swap(i, j);
    }
}

pub fn permutation_null(pairs: &[(f64, f64)], n_perms: usize, seed: u64) -> Option<NullBand> {
    rho(pairs)?;
    if n_perms < 2 {
        return None;
    }
    let mut drivers: Vec<f64> = pairs.iter().map(|p| p.0).collect();
    let offsets: Vec<f64> = pairs.iter().map(|p| p.1).collect();
    let mut rng = seed.wrapping_add(0x9e3779b97f4a7c15);
    let mut vals: Vec<f64> = Vec::with_capacity(n_perms);
    for _ in 0..n_perms {
        shuffle(&mut drivers, &mut rng);
        let perm: Vec<(f64, f64)> = drivers
            .iter()
            .copied()
            .zip(offsets.iter().copied())
            .collect();
        if let Some(r) = rho(&perm) {
            vals.push(r);
        }
    }
    if vals.len() < 2 {
        return None;
    }
    let m = mean(&vals)?;
    let sd = sample_sd(&vals)?;
    Some(NullBand {
        n_perms: vals.len(),
        mean: m,
        sd,
        band_lo: m - 2.0 * sd,
        band_hi: m + 2.0 * sd,
    })
}

pub fn scatter(pairs: &[(f64, f64)], n_perms: usize, seed: u64) -> Option<ScatterResult> {
    let s0 = sigma0(pairs)?;
    let s1 = ols_residual_sd(pairs)?;
    let r = s1 / s0;
    let null = permutation_null(pairs, n_perms, seed)?;
    Some(ScatterResult {
        n: pairs.len(),
        sigma0: s0,
        sigma1: s1,
        rho: r,
        null_mean: null.mean,
        null_sd: null.sd,
        null_band_lo: null.band_lo,
        null_band_hi: null.band_hi,
        beyond_null: r < null.band_lo || r > null.band_hi,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_perfectly_correlated_driver_drives_rho_toward_zero_and_beyond_null() {
        let pairs: Vec<(f64, f64)> = (1..=8).map(|i| (i as f64, 2.0 * i as f64 + 1.0)).collect();
        let res = scatter(&pairs, 1000, 1).unwrap();
        assert!(res.rho < 0.05, "rho {}", res.rho);
        assert!(res.beyond_null);
        assert_eq!(res.verdict_word(), "beyond null, reduction");
    }

    #[test]
    fn an_uncorrelated_driver_leaves_rho_near_one_and_within_null() {
        let offsets = [
            3.1, -1.2, 0.5, 2.8, -0.9, 4.2, 1.1, -2.5, 3.7, -0.4, 2.2, -1.8, 0.9, 4.9, -3.0, 1.5,
        ];
        let pairs: Vec<(f64, f64)> = offsets
            .iter()
            .enumerate()
            .map(|(i, o)| ((i + 1) as f64, *o))
            .collect();
        let res = scatter(&pairs, 1000, 1).unwrap();
        assert!(res.rho > 0.5, "rho {}", res.rho);
        assert!(!res.beyond_null);
        assert_eq!(res.verdict_word(), "within null");
    }

    #[test]
    fn too_few_or_non_finite_pairs_are_absent() {
        assert!(scatter(&[], 1000, 1).is_none());
        assert!(scatter(&[(1.0, 2.0), (2.0, 3.0)], 1000, 1).is_none());
        assert!(
            scatter(
                &[(1.0, f64::NAN), (2.0, 3.0), (3.0, 4.0), (4.0, 5.0)],
                1000,
                1
            )
            .is_none()
        );
        assert!(
            scatter(
                &[(f64::INFINITY, 2.0), (2.0, 3.0), (3.0, 4.0), (4.0, 5.0)],
                1000,
                1
            )
            .is_none()
        );
    }

    #[test]
    fn a_constant_driver_carries_no_regression() {
        let pairs = vec![(5.0, 1.0), (5.0, 2.0), (5.0, 3.0), (5.0, 4.0)];
        assert!(scatter(&pairs, 1000, 1).is_none());
    }

    #[test]
    fn sigma0_reads_the_offset_series() {
        let pairs = vec![(1.0, 1.0), (2.0, 3.0), (3.0, 2.0), (4.0, 6.0)];
        let offsets = [1.0, 3.0, 2.0, 6.0];
        let m = offsets.iter().sum::<f64>() / 4.0;
        let var = offsets.iter().map(|o| (o - m) * (o - m)).sum::<f64>() / 3.0;
        assert!((sigma0(&pairs).unwrap() - var.sqrt()).abs() < 1e-12);
    }

    #[test]
    fn the_null_band_holds_two_standard_deviations() {
        let pairs: Vec<(f64, f64)> = (1..=12).map(|i| (i as f64, i as f64 * 0.5)).collect();
        let null = permutation_null(&pairs, 1000, 1).unwrap();
        assert_eq!(null.n_perms, 1000);
        assert!((null.band_hi - null.mean - 2.0 * null.sd).abs() < 1e-12);
        assert!((null.band_lo - (null.mean - 2.0 * null.sd)).abs() < 1e-12);
    }
}
