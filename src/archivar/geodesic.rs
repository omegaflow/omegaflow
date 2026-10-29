pub struct Vincenty {
    pub a_m: f64,
    pub f: f64,
}

#[derive(Debug)]
pub struct Inverse {
    pub distance_m: f64,
    pub azimuth1_deg: f64,
    pub azimuth2_deg: f64,
}

impl Vincenty {
    pub fn wgs84() -> Vincenty {
        Vincenty {
            a_m: 6_378_137.0,
            f: 1.0 / 298.257_223_563,
        }
    }

    pub fn inverse(&self, lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> Option<Inverse> {
        if !lat1.is_finite()
            || !lon1.is_finite()
            || !lat2.is_finite()
            || !lon2.is_finite()
            || lat1.abs() > 90.0
            || lat2.abs() > 90.0
        {
            return None;
        }
        let b = self.a_m * (1.0 - self.f);
        let (phi1, phi2) = (lat1.to_radians(), lat2.to_radians());
        let big_l = (lon2 - lon1).to_radians();
        let u1 = ((1.0 - self.f) * phi1.tan()).atan();
        let u2 = ((1.0 - self.f) * phi2.tan()).atan();
        let (sin_u1, cos_u1) = u1.sin_cos();
        let (sin_u2, cos_u2) = u2.sin_cos();
        let mut lambda = big_l;
        let mut sin_sigma = 0.0;
        let mut cos_sigma = 1.0;
        let mut sigma = 0.0;
        let mut cos_sq_alpha = 1.0;
        let mut cos2_sigma_m = 0.0;
        let mut converged = false;
        for _ in 0..200 {
            let (sin_lambda, cos_lambda) = lambda.sin_cos();
            sin_sigma = ((cos_u2 * sin_lambda).powi(2)
                + (cos_u1 * sin_u2 - sin_u1 * cos_u2 * cos_lambda).powi(2))
            .sqrt();
            if sin_sigma == 0.0 {
                return Some(Inverse {
                    distance_m: 0.0,
                    azimuth1_deg: 0.0,
                    azimuth2_deg: 0.0,
                });
            }
            cos_sigma = sin_u1 * sin_u2 + cos_u1 * cos_u2 * cos_lambda;
            sigma = sin_sigma.atan2(cos_sigma);
            let sin_alpha = cos_u1 * cos_u2 * sin_lambda / sin_sigma;
            cos_sq_alpha = 1.0 - sin_alpha * sin_alpha;
            cos2_sigma_m = if cos_sq_alpha != 0.0 {
                cos_sigma - 2.0 * sin_u1 * sin_u2 / cos_sq_alpha
            } else {
                0.0
            };
            let c = self.f / 16.0 * cos_sq_alpha * (4.0 + self.f * (4.0 - 3.0 * cos_sq_alpha));
            let previous = lambda;
            lambda = big_l
                + (1.0 - c)
                    * self.f
                    * sin_alpha
                    * (sigma
                        + c * sin_sigma
                            * (cos2_sigma_m
                                + c * cos_sigma * (-1.0 + 2.0 * cos2_sigma_m * cos2_sigma_m)));
            if (lambda - previous).abs() < 1e-13 {
                converged = true;
                break;
            }
        }
        if !converged {
            return None;
        }
        let u_sq = cos_sq_alpha * (self.a_m * self.a_m - b * b) / (b * b);
        let big_a =
            1.0 + u_sq / 16384.0 * (4096.0 + u_sq * (-768.0 + u_sq * (320.0 - 175.0 * u_sq)));
        let big_b = u_sq / 1024.0 * (256.0 + u_sq * (-128.0 + u_sq * (74.0 - 47.0 * u_sq)));
        let delta_sigma = big_b
            * sin_sigma
            * (cos2_sigma_m
                + big_b / 4.0
                    * (cos_sigma * (-1.0 + 2.0 * cos2_sigma_m * cos2_sigma_m)
                        - big_b / 6.0
                            * cos2_sigma_m
                            * (-3.0 + 4.0 * sin_sigma * sin_sigma)
                            * (-3.0 + 4.0 * cos2_sigma_m * cos2_sigma_m)));
        let distance_m = b * big_a * (sigma - delta_sigma);
        let azimuth1 =
            (cos_u2 * lambda.sin()).atan2(cos_u1 * sin_u2 - sin_u1 * cos_u2 * lambda.cos());
        let azimuth2 =
            (cos_u1 * lambda.sin()).atan2(-sin_u1 * cos_u2 + cos_u1 * sin_u2 * lambda.cos());
        let out = Inverse {
            distance_m,
            azimuth1_deg: azimuth1.to_degrees().rem_euclid(360.0),
            azimuth2_deg: azimuth2.to_degrees().rem_euclid(360.0),
        };
        if out.distance_m.is_finite() {
            Some(out)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_equator_quarter_is_a_quarter_of_the_great_circle() {
        let v = Vincenty::wgs84();
        let r = v.inverse(0.0, 0.0, 0.0, 90.0).expect("a solution");
        let want = v.a_m * std::f64::consts::FRAC_PI_2;
        assert!(
            (r.distance_m - want).abs() < 1e-6,
            "equator quarter {} vs {want}",
            r.distance_m
        );
    }

    #[test]
    fn the_meridian_quarter_is_the_published_wgs84_value() {
        let v = Vincenty::wgs84();
        let r = v.inverse(0.0, 0.0, 90.0, 0.0).expect("a solution");
        assert!(
            (r.distance_m - 10_001_965.729).abs() < 0.01,
            "meridian quarter {} m",
            r.distance_m
        );
    }

    #[test]
    fn the_inverse_is_symmetric() {
        let v = Vincenty::wgs84();
        let pairs = [
            (52.52, 13.40, -33.93, 18.42),
            (40.6, -73.8, 51.6, -0.5),
            (0.0, 0.0, 10.0, 179.0),
        ];
        for (lat1, lon1, lat2, lon2) in pairs {
            let a = v.inverse(lat1, lon1, lat2, lon2).expect("forward");
            let b = v.inverse(lat2, lon2, lat1, lon1).expect("backward");
            assert!((a.distance_m - b.distance_m).abs() < 1e-6, "{a:?} vs {b:?}");
        }
    }

    #[test]
    fn a_near_antipodal_pair_has_no_solution() {
        let v = Vincenty::wgs84();
        assert!(v.inverse(0.0, 0.0, 0.5, 179.7).is_none());
    }

    #[test]
    fn a_bad_latitude_is_absent() {
        let v = Vincenty::wgs84();
        assert!(v.inverse(91.0, 0.0, 0.0, 0.0).is_none());
        assert!(v.inverse(f64::NAN, 0.0, 0.0, 0.0).is_none());
    }
}
