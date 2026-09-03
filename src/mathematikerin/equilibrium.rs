pub const SUN_TEFF_K: f64 = 5772.0;
pub const SUN_RADIUS_M: f64 = 6.957e8;
pub const AU_M: f64 = 1.495_978_707e11;

pub fn teq(teff_k: f64, r_star_m: f64, a_m: f64, albedo: f64) -> Option<f64> {
    if !teff_k.is_finite()
        || teff_k <= 0.0
        || !r_star_m.is_finite()
        || r_star_m <= 0.0
        || !a_m.is_finite()
        || a_m <= 0.0
        || !albedo.is_finite()
        || albedo < 0.0
        || albedo >= 1.0
    {
        return None;
    }
    let t = teff_k * (r_star_m / (2.0 * a_m)).sqrt() * (1.0 - albedo).powf(0.25);
    if t.is_finite() {
        Some(t)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn earth_albedo_30_is_the_canonical_255k() {
        let t = teq(SUN_TEFF_K, SUN_RADIUS_M, AU_M, 0.30).unwrap();
        assert!(
            (t - 254.6).abs() < 0.5,
            "earth equilibrium ~255 K, got {}",
            t
        );
    }

    #[test]
    fn earth_zero_albedo_is_278k() {
        let t = teq(SUN_TEFF_K, SUN_RADIUS_M, AU_M, 0.0).unwrap();
        assert!(
            (t - 278.3).abs() < 0.5,
            "zero-albedo earth ~278 K, got {}",
            t
        );
    }

    #[test]
    fn hot_jupiter_scales_with_star_and_orbit() {
        let t = teq(SUN_TEFF_K, SUN_RADIUS_M, 0.05 * AU_M, 0.0).unwrap();
        assert!((t - 1244.0).abs() < 5.0, "0.05 AU ~1244 K, got {}", t);
    }

    #[test]
    fn refuses_unphysical_inputs() {
        assert!(teq(0.0, SUN_RADIUS_M, AU_M, 0.0).is_none());
        assert!(teq(SUN_TEFF_K, 0.0, AU_M, 0.0).is_none());
        assert!(teq(SUN_TEFF_K, SUN_RADIUS_M, 0.0, 0.0).is_none());
        assert!(teq(SUN_TEFF_K, SUN_RADIUS_M, AU_M, 1.0).is_none());
        assert!(teq(SUN_TEFF_K, SUN_RADIUS_M, AU_M, -0.1).is_none());
        assert!(teq(f64::NAN, SUN_RADIUS_M, AU_M, 0.0).is_none());
    }
}
