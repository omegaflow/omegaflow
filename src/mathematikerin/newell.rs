pub fn newell_dphi_dt(
    by: &[Option<f64>],
    bz: &[Option<f64>],
    speed: &[Option<f64>],
) -> Vec<Option<f64>> {
    by.iter()
        .zip(bz.iter())
        .zip(speed.iter())
        .map(|((b, z), s)| match (b, z, s) {
            (Some(by), Some(bz), Some(v)) => {
                let bt = by.hypot(*bz);
                let theta = by.atan2(*bz);
                let val = v.powf(4.0 / 3.0)
                    * bt.powf(2.0 / 3.0)
                    * (theta / 2.0).sin().abs().powf(8.0 / 3.0);
                if val.is_finite() { Some(val) } else { None }
            }
            _ => None,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::newell_dphi_dt;

    #[test]
    fn newell_known_value_matches_f32_reference() {
        let by = [Some(1.0_f64)];
        let bz = [Some(0.0_f64)];
        let speed = [Some(1.0_f64)];
        let out = newell_dphi_dt(&by, &bz, &speed);
        let expected = 2.0_f64.powf(-4.0 / 3.0);
        let measured = out[0].expect("finite value");
        assert!(
            (measured - expected).abs() < 1e-9,
            "{measured} vs {expected}"
        );
    }

    #[test]
    fn newell_absence_stays_absent() {
        let out = newell_dphi_dt(&[None], &[Some(1.0)], &[Some(1.0)]);
        assert_eq!(out, vec![None]);
    }
}
