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

pub fn cartesian_from_cylindrical(
    longitude_deg: f64,
    spin_axis_km: f64,
    equator_km: f64,
) -> Option<[f64; 3]> {
    if !spin_axis_km.is_finite() || spin_axis_km < 0.0 || !equator_km.is_finite() {
        return None;
    }
    let longitude = longitude_deg.to_radians();
    Some([
        spin_axis_km * longitude.cos(),
        spin_axis_km * longitude.sin(),
        equator_km,
    ])
}

pub fn enu_basis(latitude_deg: f64, longitude_deg: f64) -> Option<[[f64; 3]; 3]> {
    if !latitude_deg.is_finite() || !(-90.0..=90.0).contains(&latitude_deg) {
        return None;
    }
    let lat = latitude_deg.to_radians();
    let lon = longitude_deg.to_radians();
    let east = [-lon.sin(), lon.cos(), 0.0];
    let north = [-lat.sin() * lon.cos(), -lat.sin() * lon.sin(), lat.cos()];
    let up = [lat.cos() * lon.cos(), lat.cos() * lon.sin(), lat.sin()];
    Some([east, north, up])
}

pub fn project_enu(basis: [[f64; 3]; 3], displacement_km: [f64; 3]) -> [f64; 3] {
    let mut out = [0.0; 3];
    for axis in 0..3 {
        out[axis] = basis[axis][0] * displacement_km[0]
            + basis[axis][1] * displacement_km[1]
            + basis[axis][2] * displacement_km[2];
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_cylindrical_point_round_trips() {
        let cartesian = cartesian_from_cylindrical(243.125207105, 5209.4825187, 3669.2423067)
            .expect("a station");
        let (lon, rho, z) =
            cylindrical_from_itrf(cartesian[0], cartesian[1], cartesian[2]).expect("a station");
        assert!((lon - 243.125207105).abs() < 1e-9);
        assert!((rho - 5209.4825187).abs() < 1e-9);
        assert!((z - 3669.2423067).abs() < 1e-9);
    }

    #[test]
    fn the_enu_basis_is_orthonormal_and_points_up() {
        let basis = enu_basis(0.0, 0.0).expect("a basis");
        assert!((basis[0][1] - 1.0).abs() < 1e-12);
        assert!((basis[1][2] - 1.0).abs() < 1e-12);
        assert!((basis[2][0] - 1.0).abs() < 1e-12);
        let basis = enu_basis(35.34, -116.87).expect("a basis");
        for a in 0..3 {
            let norm =
                (basis[a][0] * basis[a][0] + basis[a][1] * basis[a][1] + basis[a][2] * basis[a][2])
                    .sqrt();
            assert!((norm - 1.0).abs() < 1e-12);
            for b in (a + 1)..3 {
                let dot = basis[a][0] * basis[b][0]
                    + basis[a][1] * basis[b][1]
                    + basis[a][2] * basis[b][2];
                assert!(dot.abs() < 1e-12);
            }
        }
    }

    #[test]
    fn a_vertical_displacement_reads_as_up() {
        let basis = enu_basis(35.0, 243.0).expect("a basis");
        let up = project_enu(basis, [basis[2][0], basis[2][1], basis[2][2]]);
        assert!((up[2] - 1.0).abs() < 1e-12);
        assert!((up[0]).abs() < 1e-12);
    }

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
