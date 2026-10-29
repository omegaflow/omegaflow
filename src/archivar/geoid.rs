pub struct GeoidModel {
    pub radius_m: f64,
    pub degree: usize,
    c: Vec<Vec<f64>>,
    s: Vec<Vec<f64>>,
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
                for m in 0..n {
                    let a = ((2.0 * n as f64 + 1.0) * (2.0 * n as f64 - 1.0)
                        / ((n as f64 - m as f64) * (n as f64 + m as f64)))
                        .sqrt();
                    let first = a * x * p[n - 1][m];
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
                    p[n][m] = first - second;
                }
            }
            if n < 2 {
                continue;
            }
            let mut inner = (self.c[n][0] - normal_zonal(n)) * p[n][0];
            for m in 1..=n {
                inner += p[n][m]
                    * (self.c[n][m] * (m as f64 * lam).cos()
                        + self.s[n][m] * (m as f64 * lam).sin());
            }
            sum += inner;
        }
        let n = self.radius_m * sum;
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
    pub model: String,
    pub geoid_m: f64,
}

const TABLE: &str = include_str!("kernels/geoid_stations.tsv");

pub fn parse_table(text: &str) -> Option<Vec<StationValue>> {
    let mut out = Vec::new();
    for line in text.lines() {
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }
        let cols: Vec<&str> = line.split('\t').collect();
        if cols.len() < 5 {
            return None;
        }
        let lat_deg: f64 = cols[1].trim().parse().ok()?;
        let lon_deg: f64 = cols[2].trim().parse().ok()?;
        let geoid_m: f64 = cols[4].trim().parse().ok()?;
        out.push(StationValue {
            station: cols[0].trim().to_string(),
            lat_deg,
            lon_deg,
            model: cols[3].trim().to_string(),
            geoid_m,
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
    fn an_empty_or_degree_free_model_is_absent() {
        assert!(GeoidModel::parse("", 10).is_none());
        assert!(GeoidModel::parse("radius 6371000.0\n", 10).is_none());
        assert!(GeoidModel::parse("gfc 2 0 -4.8e-4 0.0\n", 10).is_none());
    }

    #[test]
    fn the_fixture_carries_three_models_and_one_independent_grid() {
        let rows = fixture().expect("geoid fixture");
        assert_eq!(rows.len(), 12);
        for station in ["Berlin", "Honolulu", "Kapstadt"] {
            let set: Vec<&StationValue> = rows.iter().filter(|r| r.station == station).collect();
            assert_eq!(set.len(), 4, "{station} carries four values");
            for model in ["EGM96", "EGM2008", "XGM2019e", "PROJ EGM96 grid"] {
                assert!(
                    set.iter().any(|r| r.model == model),
                    "{station} lacks {model}"
                );
            }
        }
    }

    #[test]
    fn the_three_models_agree_within_the_measured_spread() {
        let rows = fixture().expect("geoid fixture");
        for station in ["Berlin", "Honolulu", "Kapstadt"] {
            let set: Vec<&StationValue> = rows
                .iter()
                .filter(|r| r.station == station && r.model != "PROJ EGM96 grid")
                .collect();
            let spread = set
                .iter()
                .map(|a| {
                    set.iter()
                        .map(|b| (a.geoid_m - b.geoid_m).abs())
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
        for station in ["Berlin", "Honolulu", "Kapstadt"] {
            let ours = rows
                .iter()
                .find(|r| r.station == station && r.model == "EGM96")
                .expect("our EGM96 value");
            let proj = rows
                .iter()
                .find(|r| r.station == station && r.model == "PROJ EGM96 grid")
                .expect("the PROJ grid value");
            let diff = (ours.geoid_m - proj.geoid_m).abs();
            assert!(
                diff <= 0.5,
                "{station}: our synthesis differs from PROJ's grid by {diff} m"
            );
        }
    }
}
