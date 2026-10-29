pub struct GeoidModel {
    pub radius_m: f64,
    pub degree: usize,
    c: Vec<Vec<f64>>,
    s: Vec<Vec<f64>>,
}

pub const WGS84_GM: f64 = 3.986_004_418e14;
pub const WGS84_A_M: f64 = 6_378_137.0;
pub const WGS84_F: f64 = 1.0 / 298.257_223_563;

pub fn somigliana_gravity(lat_deg: f64) -> f64 {
    let phi = lat_deg.to_radians();
    let s2 = phi.sin() * phi.sin();
    9.780_325_335_9 * (1.0 + 0.001_931_852_652_41 * s2) / (1.0 - 0.006_694_379_990_13 * s2).sqrt()
}

pub const WGS84_OMEGA: f64 = 7.292_115e-5;
pub const WGS84_GM_EARTH: f64 = 3.986_004_418e14;
pub const W0_IAG: f64 = 62_636_853.4;
pub const W0_EGM2008: f64 = 62_636_855.7294;

pub fn exact_radius(lat_deg: f64) -> f64 {
    let phi = lat_deg.to_radians();
    let e2 = 2.0 * WGS84_F - WGS84_F * WGS84_F;
    let n_rad = WGS84_A_M / (1.0 - e2 * phi.sin() * phi.sin()).sqrt();
    ((n_rad * phi.cos()).powi(2) + (n_rad * (1.0 - e2) * phi.sin()).powi(2)).sqrt()
}

pub const NORMAL_C20: f64 = -4.841_651_437_908_15e-4;
pub const NORMAL_C40: f64 = 7.905_370_510_517_63e-7;

pub fn normal_zonal(n: usize) -> f64 {
    match n {
        2 => NORMAL_C20,
        4 => NORMAL_C40,
        _ => 0.0,
    }
}

impl GeoidModel {
    pub fn parse(text: &str, cap: usize) -> Option<GeoidModel> {
        let mut radius_m = 0.0f64;
        let mut degree = 0usize;
        let mut rows: Vec<(usize, usize, f64, f64)> = Vec::new();
        for line in text.lines() {
            if line.starts_with("gfc") {
                let cols: Vec<&str> = line.split_whitespace().collect();
                if cols.len() < 5 {
                    continue;
                }
                let Ok(n) = cols[1].parse::<usize>() else {
                    continue;
                };
                let Ok(m) = cols[2].parse::<usize>() else {
                    continue;
                };
                if n > cap || m > n {
                    continue;
                }
                let Ok(coefficient_c) = cols[3].parse::<f64>() else {
                    continue;
                };
                let Ok(coefficient_s) = cols[4].parse::<f64>() else {
                    continue;
                };
                rows.push((n, m, coefficient_c, coefficient_s));
                if n > degree {
                    degree = n;
                }
                continue;
            }
            let cols: Vec<&str> = line.split_whitespace().collect();
            if cols.len() == 2 && cols[0] == "radius" {
                let parsed: f64 = cols[1].parse().ok()?;
                radius_m = parsed;
            }
        }
        if radius_m <= 0.0 || degree == 0 {
            return None;
        }
        let mut c = vec![vec![0.0f64; degree + 1]; degree + 1];
        let mut s = vec![vec![0.0f64; degree + 1]; degree + 1];
        for (n, m, cc, ss) in rows {
            c[n][m] = cc;
            s[n][m] = ss;
        }
        Some(GeoidModel {
            radius_m,
            degree,
            c,
            s,
        })
    }

    fn potential_at(&self, r_m: f64, lat_deg: f64, lon_deg: f64) -> Option<f64> {
        if !r_m.is_finite() || r_m <= 0.0 {
            return None;
        }
        let phi = lat_deg.to_radians();
        let psi = ((1.0 - WGS84_F).powi(2) * phi.tan()).atan();
        let theta = std::f64::consts::FRAC_PI_2 - psi;
        let x = theta.cos();
        let sin_theta = theta.sin();
        let lam = lon_deg.to_radians();
        let nmax = self.degree;
        let mut p = vec![vec![0.0f64; nmax + 1]; nmax + 1];
        p[0][0] = 1.0;
        let mut sum = 1.0f64;
        let mut r_l = 1.0f64;
        let ratio = self.radius_m / r_m;
        for n in 1..=nmax {
            if n == 1 {
                p[1][0] = 3.0f64.sqrt() * x;
                p[1][1] = 3.0f64.sqrt() * sin_theta;
            } else {
                p[n][n] = ((2.0 * n as f64 + 1.0) / (2.0 * n as f64)).sqrt()
                    * sin_theta
                    * p[n - 1][n - 1];
                let mut row = vec![0.0f64; n];
                for (m, &pm) in p[n - 1].iter().enumerate().take(n) {
                    let a = ((2.0 * n as f64 + 1.0) * (2.0 * n as f64 - 1.0)
                        / ((n as f64 - m as f64) * (n as f64 + m as f64)))
                        .sqrt();
                    let first = a * x * pm;
                    let second = if n >= 2 && m + 1 < n {
                        let b = ((2.0 * n as f64 + 1.0)
                            * (n as f64 + m as f64 - 1.0)
                            * (n as f64 - m as f64 - 1.0)
                            / ((2.0 * n as f64 - 3.0)
                                * (n as f64 - m as f64)
                                * (n as f64 + m as f64)))
                            .sqrt();
                        b * p[n - 2][m]
                    } else {
                        0.0
                    };
                    row[m] = first - second;
                }
                p[n][..n].copy_from_slice(&row);
            }
            r_l *= ratio;
            let mut inner = self.c[n][0] * p[n][0];
            for (m, &pnm) in p[n].iter().enumerate().take(n + 1).skip(1) {
                inner += pnm
                    * (self.c[n][m] * (m as f64 * lam).cos()
                        + self.s[n][m] * (m as f64 * lam).sin());
            }
            sum += r_l * inner;
        }
        let gravitational = WGS84_GM_EARTH / r_m * sum;
        let axial = r_m * psi.cos();
        let centrifugal = 0.5 * WGS84_OMEGA * WGS84_OMEGA * axial * axial;
        Some(gravitational + centrifugal)
    }

    pub fn potential_parts(&self, r_m: f64, lat_deg: f64, lon_deg: f64) -> Option<(f64, f64, f64)> {
        let w = self.potential_at(r_m, lat_deg, lon_deg)?;
        let phi = lat_deg.to_radians();
        let psi = ((1.0 - WGS84_F).powi(2) * phi.tan()).atan();
        let axial = r_m * psi.cos();
        let cent = 0.5 * WGS84_OMEGA * WGS84_OMEGA * axial * axial;
        Some((w, w - cent, cent))
    }

    pub fn geoid_m_bruns(&self, lat_deg: f64, lon_deg: f64, w0: f64) -> Option<f64> {
        let re = exact_radius(lat_deg);
        let gamma = somigliana_gravity(lat_deg);
        let mut n = 0.0f64;
        for _ in 0..4 {
            let w = self.potential_at(re + n, lat_deg, lon_deg)?;
            n += (w - w0) / gamma;
        }
        if n.is_finite() { Some(n) } else { None }
    }

    pub fn geoid_m(&self, lat_deg: f64, lon_deg: f64) -> Option<f64> {
        if !lat_deg.is_finite() || !lon_deg.is_finite() || lat_deg.abs() > 90.0 {
            return None;
        }
        let theta = (90.0 - lat_deg).to_radians();
        let lam = lon_deg.to_radians();
        let x = theta.cos();
        let sin_theta = theta.sin();
        let nmax = self.degree;
        let mut p = vec![vec![0.0f64; nmax + 1]; nmax + 1];
        p[0][0] = 1.0;
        let mut sum = 0.0f64;
        for n in 1..=nmax {
            if n == 1 {
                p[1][0] = 3.0f64.sqrt() * x;
                p[1][1] = 3.0f64.sqrt() * sin_theta;
            } else {
                p[n][n] = ((2.0 * n as f64 + 1.0) / (2.0 * n as f64)).sqrt()
                    * sin_theta
                    * p[n - 1][n - 1];
                let mut row = vec![0.0f64; n];
                for (m, &pm) in p[n - 1].iter().enumerate().take(n) {
                    let a = ((2.0 * n as f64 + 1.0) * (2.0 * n as f64 - 1.0)
                        / ((n as f64 - m as f64) * (n as f64 + m as f64)))
                        .sqrt();
                    let first = a * x * pm;
                    let second = if n >= 2 && m + 1 < n {
                        let b = ((2.0 * n as f64 + 1.0)
                            * (n as f64 + m as f64 - 1.0)
                            * (n as f64 - m as f64 - 1.0)
                            / ((2.0 * n as f64 - 3.0)
                                * (n as f64 - m as f64)
                                * (n as f64 + m as f64)))
                            .sqrt();
                        b * p[n - 2][m]
                    } else {
                        0.0
                    };
                    row[m] = first - second;
                }
                p[n][..n].copy_from_slice(&row);
            }
            if n < 2 {
                continue;
            }
            let mut inner = (self.c[n][0] - normal_zonal(n)) * p[n][0];
            for (m, &pnm) in p[n].iter().enumerate().take(n + 1).skip(1) {
                inner += pnm
                    * (self.c[n][m] * (m as f64 * lam).cos()
                        + self.s[n][m] * (m as f64 * lam).sin());
            }
            sum += inner;
        }
        let re = WGS84_A_M * (1.0 - WGS84_F * lat_deg.to_radians().sin().powi(2));
        let gamma = somigliana_gravity(lat_deg);
        let n = WGS84_GM / (re * gamma) * sum;
        if n.is_finite() { Some(n) } else { None }
    }

    pub fn coefficient(&self, n: usize, m: usize) -> Option<(f64, f64)> {
        if n > self.degree || m > n {
            return None;
        }
        Some((self.c[n][m], self.s[n][m]))
    }
}

pub struct StationValue {
    pub station: String,
    pub lat_deg: f64,
    pub lon_deg: f64,
    pub source: String,
    pub cap: usize,
    pub quantity: String,
    pub value: f64,
}

const TABLE: &str = include_str!("kernels/geoid_stations.tsv");

pub fn parse_table(text: &str) -> Option<Vec<StationValue>> {
    let mut out = Vec::new();
    for line in text.lines() {
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }
        let cols: Vec<&str> = line.split('\t').collect();
        if cols.len() < 7 {
            return None;
        }
        let lat_deg: f64 = cols[1].trim().parse().ok()?;
        let lon_deg: f64 = cols[2].trim().parse().ok()?;
        let cap: usize = cols[4].trim().parse().ok()?;
        let quantity = cols[5].trim().to_string();
        if quantity != "n" && quantity != "w" && quantity != "g" {
            return None;
        }
        let value: f64 = cols[6].trim().parse().ok()?;
        out.push(StationValue {
            station: cols[0].trim().to_string(),
            lat_deg,
            lon_deg,
            source: cols[3].trim().to_string(),
            cap,
            quantity,
            value,
        });
    }
    if out.is_empty() { None } else { Some(out) }
}

pub fn fixture() -> Option<Vec<StationValue>> {
    parse_table(TABLE)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn synthetic(c20: f64) -> GeoidModel {
        let mut text = String::from("radius 6378136.3\nmax_degree 2\n");
        text.push_str(&format!("gfc 0 0 1.0 0.0\ngfc 2 0 {c20} 0.0\n"));
        GeoidModel::parse(&text, 2).expect("a two degree model")
    }

    #[test]
    fn a_model_with_only_the_normal_field_has_no_undulation() {
        let model = synthetic(NORMAL_C20);
        for lat in [-60.0, 0.0, 45.0, 90.0] {
            let n = model.geoid_m(lat, 12.0).expect("a geoid value");
            assert!(n.abs() < 1e-6, "lat {lat}: {n} m");
        }
    }

    #[test]
    fn a_one_micro_unit_of_c20_is_the_analytic_zonal_term() {
        let delta = 1e-6;
        let model = synthetic(NORMAL_C20 + delta);
        for lat in [-45.0f64, 0.0, 30.0] {
            let theta = (90.0 - lat).to_radians();
            let p20 = 5.0f64.sqrt() * (3.0 * theta.cos().powi(2) - 1.0) / 2.0;
            let want = model.radius_m * delta * p20;
            let got = model.geoid_m(lat, 0.0).expect("a geoid value");
            assert!(
                (got - want).abs() < 1e-9,
                "lat {lat}: got {got}, want {want}"
            );
        }
    }

    #[test]
    fn the_bruns_iteration_lands_on_the_potential_it_was_given() {
        let model = synthetic(NORMAL_C20);
        let lat = 52.52;
        for target in [0.0f64, 10.0, -7.5] {
            let r = exact_radius(lat) + target;
            let w = model.potential_parts(r, lat, 13.4).expect("a potential").0;
            let n = model.geoid_m_bruns(lat, 13.4, w).expect("a geoid value");
            assert!(
                (n - target).abs() < 1e-4,
                "target {target}: iteration returned {n}"
            );
        }
    }

    #[test]
    fn a_lower_reference_potential_lifts_the_geoid() {
        let model = synthetic(NORMAL_C20);
        let base = model
            .potential_parts(exact_radius(30.0), 30.0, 0.0)
            .expect("a potential")
            .0;
        let low = model
            .geoid_m_bruns(30.0, 0.0, base - 98.1)
            .expect("a geoid value");
        let high = model
            .geoid_m_bruns(30.0, 0.0, base + 98.1)
            .expect("a geoid value");
        assert!(
            low > 9.0 && low < 11.0,
            "98.1 lower is about ten metres: {low}"
        );
        assert!(
            high < -9.0 && high > -11.0,
            "98.1 higher is about minus ten: {high}"
        );
    }

    #[test]
    fn an_empty_or_degree_free_model_is_absent() {
        assert!(GeoidModel::parse("", 10).is_none());
        assert!(GeoidModel::parse("radius 6371000.0\n", 10).is_none());
        assert!(GeoidModel::parse("gfc 2 0 -4.8e-4 0.0\n", 10).is_none());
    }

    #[test]
    fn the_fixture_carries_three_models_two_grids_and_the_service_potentials() {
        let rows = fixture().expect("geoid fixture");
        assert_eq!(rows.len(), 47);
        let counts = [
            ("Berlin", 13usize),
            ("Honolulu", 13),
            ("Cape Town", 11),
            ("Equator", 2),
            ("Hawaii North", 2),
            ("Hawaii East", 2),
            ("Hawaii South", 2),
            ("Hawaii West", 2),
        ];
        for (station, want) in counts {
            let set: Vec<&StationValue> = rows.iter().filter(|r| r.station == station).collect();
            assert_eq!(set.len(), want, "{station} carries {want} values");
        }
        for station in ["Berlin", "Honolulu", "Cape Town"] {
            let set: Vec<&StationValue> = rows.iter().filter(|r| r.station == station).collect();
            for source in [
                "ours EGM96",
                "ours EGM2008",
                "ours XGM2019e",
                "ours EGM2008 bruns",
                "EGM2008 grid",
                "PROJ EGM96 grid",
                "ICGEM EGM96",
                "ICGEM EGM2008",
            ] {
                assert!(
                    set.iter().any(|r| r.source == source),
                    "{station} lacks {source}"
                );
            }
            assert_eq!(
                set.iter().filter(|r| r.source == "ours EGM2008").count(),
                3,
                "{station} carries EGM2008 at two degrees as height and potential"
            );
        }
        assert_eq!(
            rows.iter()
                .filter(|r| r.station == "Cape Town" && r.quantity == "w")
                .count(),
            4,
            "Cape Town carries four potentials"
        );
        for station in ["Berlin", "Honolulu"] {
            assert_eq!(
                rows.iter()
                    .filter(|r| r.station == station && r.quantity == "w")
                    .count(),
                6,
                "{station} carries six potentials"
            );
            let degrees: Vec<usize> = rows
                .iter()
                .filter(|r| r.station == station && r.source == "ICGEM EGM96")
                .map(|r| r.cap)
                .collect();
            assert_eq!(degrees, vec![360, 180, 90], "{station}: three degrees");
        }
        for station in [
            "Equator",
            "Hawaii North",
            "Hawaii East",
            "Hawaii South",
            "Hawaii West",
        ] {
            let set: Vec<&StationValue> = rows.iter().filter(|r| r.station == station).collect();
            assert_eq!(set.len(), 2, "{station} carries the two EGM96 potentials");
            assert!(
                set.iter()
                    .all(|r| r.quantity == "w" && r.source.contains("EGM96"))
            );
        }
    }

    fn find<'a>(
        rows: &'a [StationValue],
        station: &str,
        source: &str,
        quantity: &str,
    ) -> &'a StationValue {
        rows.iter()
            .find(|r| r.station == station && r.source == source && r.quantity == quantity)
            .expect("the table carries this row")
    }

    #[test]
    fn the_three_models_agree_within_the_measured_spread() {
        let rows = fixture().expect("geoid fixture");
        for station in ["Berlin", "Honolulu", "Cape Town"] {
            let set: Vec<&StationValue> = rows
                .iter()
                .filter(|r| {
                    r.station == station
                        && r.quantity == "n"
                        && r.cap == 360
                        && ["ours EGM96", "ours EGM2008", "ours XGM2019e"]
                            .contains(&r.source.as_str())
                })
                .collect();
            assert_eq!(set.len(), 3, "{station}: three synthesis values");
            let spread = set
                .iter()
                .map(|a| {
                    set.iter()
                        .map(|b| (a.value - b.value).abs())
                        .fold(0.0, f64::max)
                })
                .fold(0.0, f64::max);
            assert!(
                spread <= 0.6,
                "{station}: the three models span {spread} m, beyond the 0.6 m allowance"
            );
        }
    }

    #[test]
    fn our_egm96_value_holds_against_the_proj_grid() {
        let rows = fixture().expect("geoid fixture");
        for station in ["Berlin", "Honolulu", "Cape Town"] {
            let ours = find(&rows, station, "ours EGM96", "n");
            let proj = find(&rows, station, "PROJ EGM96 grid", "n");
            let diff = (ours.value - proj.value).abs();
            assert!(
                diff <= 0.5,
                "{station}: our synthesis differs from PROJ's grid by {diff} m"
            );
        }
    }

    #[test]
    fn the_bruns_value_holds_against_the_official_egm2008_grid() {
        let rows = fixture().expect("geoid fixture");
        for station in ["Berlin", "Honolulu", "Cape Town"] {
            let bruns = find(&rows, station, "ours EGM2008 bruns", "n");
            let grid = find(&rows, station, "EGM2008 grid", "n");
            assert_eq!(bruns.cap, 2190);
            assert_eq!(grid.cap, 2190);
            let diff = (bruns.value - grid.value).abs();
            assert!(
                diff <= 0.02,
                "{station}: the ellipsoidal path differs from the grid by {diff} m"
            );
        }
    }

    #[test]
    fn our_egm2008_potential_matches_the_service_within_the_gm_term() {
        let rows = fixture().expect("geoid fixture");
        for station in ["Berlin", "Honolulu", "Cape Town"] {
            let ours = find(&rows, station, "ours EGM2008", "w");
            let service = find(&rows, station, "ICGEM EGM2008", "w");
            let diff = (ours.value - service.value).abs();
            assert!(
                diff <= 0.05,
                "{station}: our potential differs from the service by {diff} m2/s2"
            );
        }
    }
}
