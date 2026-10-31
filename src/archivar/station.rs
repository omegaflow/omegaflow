pub fn cylindrical_from_itrf(x_km: f64, y_km: f64, z_km: f64) -> Option<(f64, f64, f64)> {
    let rho = (x_km * x_km + y_km * y_km).sqrt();
    if !rho.is_finite() || rho <= 0.0 || !z_km.is_finite() {
        return None;
    }
    let mut longitude = y_km.atan2(x_km).to_degrees();
    if longitude < 0.0 {
        longitude += 360.0;
    }
    Some((longitude, rho, z_km))
}

pub fn propagate(
    x_km: f64,
    y_km: f64,
    z_km: f64,
    rate_km_per_year: [f64; 3],
    years: f64,
) -> [f64; 3] {
    [
        x_km + rate_km_per_year[0] * years,
        y_km + rate_km_per_year[1] * years,
        z_km + rate_km_per_year[2] * years,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_point_on_the_prime_meridian_has_zero_longitude() {
        let (longitude, rho, z) = cylindrical_from_itrf(6378.0, 0.0, 0.0).expect("a station");
        assert!((longitude).abs() < 1e-12);
        assert!((rho - 6378.0).abs() < 1e-12);
        assert!((z).abs() < 1e-12);
    }

    #[test]
    fn a_point_in_the_third_quadrant_carries_a_full_circle_longitude() {
        let (longitude, _, _) = cylindrical_from_itrf(-1.0, -1.0, 0.0).expect("a station");
        assert!((longitude - 225.0).abs() < 1e-12);
    }

    #[test]
    fn the_spin_axis_distance_ignores_the_height() {
        let (_, rho, z) = cylindrical_from_itrf(3000.0, 4000.0, 1234.0).expect("a station");
        assert!((rho - 5000.0).abs() < 1e-12);
        assert!((z - 1234.0).abs() < 1e-12);
    }

    #[test]
    fn propagation_moves_along_the_rate() {
        let moved = propagate(
            -2354.9067148,
            -4646.8400771,
            3669.2422852,
            [-0.0000156, 0.0000063, -0.0000030],
            -8.0,
        );
        assert!((moved[0] + 2354.9065900).abs() < 1e-9);
        assert!((moved[2] - 3669.2423092).abs() < 1e-9);
    }

    #[test]
    fn a_station_on_the_axis_is_absent() {
        assert!(cylindrical_from_itrf(0.0, 0.0, 1.0).is_none());
    }
}
