pub const A_WGS84: f64 = 6_378_137.0;

pub const F_WGS84: f64 = 1.0 / 298.257_223_563;

pub const K0: f64 = 0.9996;

pub const FALSE_NORTHING_SOUTH: f64 = 10_000_000.0;

pub const FALSE_EASTING: f64 = 500_000.0;

struct Series {
    rectifying_radius: f64,
    alpha: [f64; 6],
    beta: [f64; 6],
    delta: [f64; 6],
}

fn series() -> Series {
    let n = F_WGS84 / (2.0 - F_WGS84);
    let n2 = n * n;
    let n3 = n2 * n;
    let n4 = n2 * n2;
    let n5 = n4 * n;
    let n6 = n3 * n3;

    let rectifying_radius = A_WGS84 / (1.0 + n) * (1.0 + n2 / 4.0 + n4 / 64.0 + n6 / 256.0);

    let alpha = [
        n / 2.0 - 2.0 * n2 / 3.0 + 5.0 * n3 / 16.0 + 41.0 * n4 / 180.0 - 127.0 * n5 / 288.0
            + 7891.0 * n6 / 37800.0,
        13.0 * n2 / 48.0 - 3.0 * n3 / 5.0 + 557.0 * n4 / 1440.0 + 281.0 * n5 / 630.0
            - 1983433.0 * n6 / 1935360.0,
        61.0 * n3 / 240.0 - 103.0 * n4 / 140.0 + 15061.0 * n5 / 26880.0 + 167603.0 * n6 / 181440.0,
        49561.0 * n4 / 161280.0 - 179.0 * n5 / 168.0 + 6601661.0 * n6 / 7257600.0,
        34729.0 * n5 / 80640.0 - 3418889.0 * n6 / 1995840.0,
        212378941.0 * n6 / 319334400.0,
    ];

    let beta = [
        n / 2.0 - 2.0 * n2 / 3.0 + 37.0 * n3 / 96.0 - n4 / 360.0 - 81.0 * n5 / 512.0
            + 96199.0 * n6 / 604800.0,
        n2 / 48.0 + n3 / 15.0 - 437.0 * n4 / 1440.0 + 46.0 * n5 / 105.0
            - 1118711.0 * n6 / 3870720.0,
        17.0 * n3 / 480.0 - 37.0 * n4 / 840.0 - 209.0 * n5 / 4480.0 + 5569.0 * n6 / 90720.0,
        4397.0 * n4 / 161280.0 - 11.0 * n5 / 504.0 - 830251.0 * n6 / 7257600.0,
        4583.0 * n5 / 161280.0 - 108847.0 * n6 / 3991680.0,
        20648693.0 * n6 / 638668800.0,
    ];

    let delta = [
        2.0 * n - 2.0 * n2 / 3.0 - 2.0 * n3 + 116.0 * n4 / 45.0 + 26.0 * n5 / 45.0
            - 2854.0 * n6 / 675.0,
        7.0 * n2 / 3.0 - 8.0 * n3 / 5.0 - 227.0 * n4 / 45.0
            + 2704.0 * n5 / 315.0
            + 2323.0 * n6 / 945.0,
        56.0 * n3 / 15.0 - 136.0 * n4 / 35.0 - 1262.0 * n5 / 105.0 + 73814.0 * n6 / 2835.0,
        4279.0 * n4 / 630.0 - 332.0 * n5 / 35.0 - 399572.0 * n6 / 14175.0,
        4174.0 * n5 / 315.0 - 144838.0 * n6 / 6237.0,
        601676.0 * n6 / 22275.0,
    ];

    Series {
        rectifying_radius,
        alpha,
        beta,
        delta,
    }
}

pub fn zone_number(latitude_deg: f64, longitude_deg: f64) -> u8 {
    if (56.0..64.0).contains(&latitude_deg) && (3.0..12.0).contains(&longitude_deg) {
        return 32;
    }
    if (72.0..=84.0).contains(&latitude_deg) && longitude_deg >= 0.0 {
        if longitude_deg < 9.0 {
            return 31;
        }
        if longitude_deg < 21.0 {
            return 33;
        }
        if longitude_deg < 33.0 {
            return 35;
        }
        if longitude_deg < 42.0 {
            return 37;
        }
    }
    (((longitude_deg + 180.0) / 6.0).floor() + 1.0) as u8
}

pub fn zone_letter(latitude_deg: f64) -> Option<char> {
    if (-80.0..=84.0).contains(&latitude_deg) {
        let index = ((latitude_deg + 80.0) / 8.0).floor() as usize;
        "CDEFGHJKLMNPQRSTUVWXX".chars().nth(index)
    } else {
        None
    }
}

pub fn to_utm_wgs84(latitude_deg: f64, longitude_deg: f64, zone: u8) -> (f64, f64, f64) {
    let lam0 = (zone as f64 * 6.0 - 183.0).to_radians();
    let (mut northing, mut easting, convergence) = tm_at(latitude_deg, longitude_deg, lam0);
    if latitude_deg < 0.0 {
        northing += FALSE_NORTHING_SOUTH;
    }
    easting += FALSE_EASTING;
    (northing, easting, convergence)
}

pub fn to_tm_wgs84_cm0(latitude_deg: f64, longitude_deg: f64) -> (f64, f64, f64) {
    tm_at(latitude_deg, longitude_deg, 0.0)
}

fn tm_at(latitude_deg: f64, longitude_deg: f64, lam0: f64) -> (f64, f64, f64) {
    let s = series();
    let phi = latitude_deg.to_radians();
    let lam = longitude_deg.to_radians();
    let dlam = lam - lam0;
    let e = (F_WGS84 * (2.0 - F_WGS84)).sqrt();

    let tau = phi.tan();
    let tau_p = (tau.asinh() - e * (e * phi.sin()).atanh()).sinh();
    let cosh_psi = (1.0 + tau_p * tau_p).sqrt();

    let xi_p = tau_p.atan2(dlam.cos());
    let eta_p = (dlam.sin() / (tau_p * tau_p + dlam.cos() * dlam.cos()).sqrt()).asinh();

    let mut xi = xi_p;
    let mut eta = eta_p;
    let mut df_re = 1.0;
    let mut df_im = 0.0;
    for (j, a) in s.alpha.iter().enumerate() {
        let k = 2.0 * (j + 1) as f64;
        xi += a * (k * xi_p).sin() * (k * eta_p).cosh();
        eta += a * (k * xi_p).cos() * (k * eta_p).sinh();
        df_re += k * a * (k * xi_p).cos() * (k * eta_p).cosh();
        df_im -= k * a * (k * xi_p).sin() * (k * eta_p).sinh();
    }

    let northing = K0 * s.rectifying_radius * xi;
    let easting = K0 * s.rectifying_radius * eta;

    let rot_series = df_im.atan2(df_re);
    let rot_u = (-(tau_p * dlam.sin())).atan2(cosh_psi * dlam.cos());
    let convergence = -(rot_series + rot_u);
    (northing, easting, convergence)
}

pub fn to_utm_wgs84_no_zone(latitude_deg: f64, longitude_deg: f64) -> (f64, f64, f64) {
    to_utm_wgs84(
        latitude_deg,
        longitude_deg,
        zone_number(latitude_deg, longitude_deg),
    )
}

pub fn to_wgs84(northing: f64, easting: f64, zone: u8, northern: bool) -> (f64, f64) {
    let s = series();
    let xi =
        (northing - if northern { 0.0 } else { FALSE_NORTHING_SOUTH }) / (K0 * s.rectifying_radius);
    let eta = (easting - FALSE_EASTING) / (K0 * s.rectifying_radius);

    let mut xi_p = xi;
    let mut eta_p = eta;
    for (j, b) in s.beta.iter().enumerate() {
        let k = 2.0 * (j + 1) as f64;
        xi_p -= b * (k * xi).sin() * (k * eta).cosh();
        eta_p -= b * (k * xi).cos() * (k * eta).sinh();
    }

    let tau_p = xi_p.sin() / (eta_p.sinh().powi(2) + xi_p.cos().powi(2)).sqrt();
    let dlam = eta_p.sinh().atan2(xi_p.cos());
    let chi = tau_p.atan();

    let mut phi = chi;
    for (j, d) in s.delta.iter().enumerate() {
        phi += d * (2.0 * (j + 1) as f64 * chi).sin();
    }

    let lam0 = zone as f64 * 6.0 - 183.0;
    (phi.to_degrees(), lam0 + dlam.to_degrees())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forward_matches_published_coordinates() {
        let (n, e, _) = to_utm_wgs84(60.967_987_549_7, -149.119_325_194, 6);
        assert!((n - 6_761_077.20).abs() < 1e-2, "northing {n}");
        assert!((e - 385_273.02).abs() < 1e-2, "easting {e}");

        let (n, e, _) = to_utm_wgs84(-23.809_749, 148.334_111, 55);
        assert!((n - 7_366_197.82).abs() < 1e-2, "northing {n}");
        assert!((e - 635_900.94).abs() < 1e-2, "easting {e}");
    }

    #[test]
    fn forward_agrees_with_the_inverse_reference() {
        let (lat, lon) = to_wgs84(6_243_186.0, 261_878.0, 34, false);
        assert!((lat - -33.924_87).abs() < 3e-5, "latitude {lat}");
        assert!((lon - 18.424_06).abs() < 3e-5, "longitude {lon}");
    }

    #[test]
    fn round_trip_stays_under_a_millimetre() {
        let points = [
            (0.0, 0.0, 31),
            (50.775_35, 6.083_89, 32),
            (60.967_987_549_7, -149.119_325_194, 6),
            (-23.809_749, 148.334_111, 55),
            (-41.286_46, 174.776_24, 60),
            (78.223_2, 15.646_9, 33),
            (83.9, 40.0, 37),
        ];
        for (lat, lon, zone) in points {
            let (n, e, _) = to_utm_wgs84(lat, lon, zone);
            let northern = lat >= 0.0;
            let (lat2, lon2) = to_wgs84(n, e, zone, northern);
            assert!((lat2 - lat).abs() < 1e-9, "latitude at {lat},{lon}");
            assert!((lon2 - lon).abs() < 1e-9, "longitude at {lat},{lon}");
        }
    }

    #[test]
    fn convergence_matches_the_rotation_of_the_northward_tangent() {
        let step = 1e-4_f64.to_radians();
        let cases = [
            (10.0, 12.3, 33),
            (50.0, 7.0, 32),
            (60.967_987_549_7, -149.119_325_194, 6),
            (-33.9, 18.4, 34),
            (78.2, 15.6, 33),
        ];
        for (lat, lon, zone) in cases {
            let (_, _, gamma) = to_utm_wgs84(lat, lon, zone);
            let (n_hi, e_hi, _) = to_utm_wgs84(lat + step, lon, zone);
            let (n_lo, e_lo, _) = to_utm_wgs84(lat - step, lon, zone);
            let from_mapping = -((e_hi - e_lo).atan2(n_hi - n_lo));
            assert!(
                (gamma - from_mapping).abs() < 1e-9,
                "gamma {gamma} vs mapping {from_mapping} at {lat},{lon}"
            );
        }
    }

    #[test]
    fn zone_rules_cover_the_norway_and_svalbard_bands() {
        assert_eq!(zone_number(60.0, 5.0), 32);
        assert_eq!(zone_number(60.0, 2.9), 31);
        assert_eq!(zone_number(78.0, 8.0), 31);
        assert_eq!(zone_number(78.0, 15.0), 33);
        assert_eq!(zone_number(78.0, 25.0), 35);
        assert_eq!(zone_number(78.0, 40.0), 37);
        assert_eq!(zone_number(-33.924_87, 18.424_06), 34);
        assert_eq!(zone_number(0.0, 0.0), 31);
        assert_eq!(zone_letter(-33.924_87), Some('H'));
        assert_eq!(zone_letter(50.775_35), Some('U'));
        assert_eq!(zone_letter(85.0), None);
    }

    #[test]
    fn convergence_is_positive_east_of_the_central_meridian() {
        let (_, _, west) = to_utm_wgs84(50.0, 7.0, 32);
        let (_, _, east) = to_utm_wgs84(50.0, 11.0, 32);
        assert!(west < 0.0, "west convergence {west}");
        assert!(east > 0.0, "east convergence {east}");
    }
}

#[cfg(test)]
mod tmcoords {
    use super::to_tm_wgs84_cm0;

    #[test]
    fn the_exact_reference_holds_to_a_micrometre_near_the_meridian() {
        let text = include_str!("../data/tmcoords_sample.tsv");
        let mut bands = [0.0_f64; 10];
        let mut near = 0.0_f64;
        let mut near_at = String::new();
        let mut n = 0usize;
        let mut skipped = 0usize;
        for line in text.lines() {
            if line.starts_with("#") || line.trim().is_empty() {
                continue;
            }
            let c: Vec<&str> = line.split_whitespace().collect();
            if c.len() < 4 {
                continue;
            }
            let Ok(lat) = c[0].parse::<f64>() else {
                continue;
            };
            let Ok(lon) = c[1].parse::<f64>() else {
                continue;
            };
            let Ok(e_ref) = c[2].parse::<f64>() else {
                continue;
            };
            let Ok(n_ref) = c[3].parse::<f64>() else {
                continue;
            };
            if lat < 0.0 {
                continue;
            }
            let (north, east, _) = to_tm_wgs84_cm0(lat, lon);
            let d = (east - e_ref).abs().max((north - n_ref).abs());
            if !d.is_finite() || d > 1e6 {
                skipped += 1;
                continue;
            }
            let b = ((lon.abs() / 10.0) as usize).min(9);
            if d > bands[b] {
                bands[b] = d;
            }
            if lon.abs() <= 40.0 && d > near {
                near = d;
                near_at = format!("{lat} {lon}");
            }
            n += 1;
        }
        eprintln!(
            "bands 0-90 deg: {bands:?}, near worst {near:.3e} at {near_at}, n {n}, skipped {skipped}"
        );
        assert!(n > 6000, "sample carried {n} points");
        assert!(
            near < 1e-6,
            "within 40 deg the worst deviation is {near:.3e} m at {near_at} over {n} points"
        );
    }
}
