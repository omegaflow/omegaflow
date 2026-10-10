const WGS84_A2_KM2: f64 = 40_680_631.6;
const WGS84_B2_KM2: f64 = 40_408_296.0;

const IGRF_MAX_DEGREE: usize = 13;

fn geodetic_to_geocentric_term() -> f64 {
    WGS84_B2_KM2 / WGS84_A2_KM2
}

#[derive(Clone, Debug)]
struct ShRow {
    h: bool,
    n: usize,
    m: usize,
    values: Vec<f64>,
    sv: f64,
}

#[derive(Clone, Debug)]
pub struct IgrfCoeffs {
    epochs: Vec<f64>,
    rows: Vec<ShRow>,
}

impl IgrfCoeffs {
    pub fn parse(text: &str) -> Option<IgrfCoeffs> {
        let mut epochs: Vec<f64> = Vec::new();
        let mut rows: Vec<ShRow> = Vec::new();
        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let tokens: Vec<&str> = line.split_whitespace().collect();
            if tokens[0] == "g/h" {
                for t in &tokens[3..tokens.len().saturating_sub(1)] {
                    epochs.push(t.parse::<f64>().ok()?);
                }
                continue;
            }
            if tokens[0] != "g" && tokens[0] != "h" {
                continue;
            }
            let n: usize = tokens.get(1)?.parse().ok()?;
            let m: usize = tokens.get(2)?.parse().ok()?;
            let values: Vec<f64> = tokens[3..]
                .iter()
                .map(|t| t.parse::<f64>().ok())
                .collect::<Option<Vec<f64>>>()?;
            if epochs.is_empty() || values.len() != epochs.len() + 1 {
                return None;
            }
            rows.push(ShRow {
                h: tokens[0] == "h",
                n,
                m,
                sv: values[epochs.len()],
                values,
            });
        }
        if epochs.is_empty() || rows.is_empty() {
            return None;
        }
        Some(IgrfCoeffs { epochs, rows })
    }

    fn coefficient(&self, h: bool, n: usize, m: usize, date: f64) -> Option<f64> {
        let row = self
            .rows
            .iter()
            .find(|r| r.h == h && r.n == n && r.m == m)?;
        let first = *self.epochs.first()?;
        let last = *self.epochs.last()?;
        if !date.is_finite() || date < first || date > last + 5.0 {
            return None;
        }
        if date >= last {
            return Some(row.values[self.epochs.len() - 1] + (date - last) * row.sv);
        }
        let mut i = 0;
        while i + 1 < self.epochs.len() && self.epochs[i + 1] <= date {
            i += 1;
        }
        let t0 = self.epochs[i];
        let t1 = self.epochs[i + 1];
        let dt = (date - t0) / (t1 - t0);
        Some(row.values[i] * (1.0 - dt) + row.values[i + 1] * dt)
    }

    fn north_geomagnetic_pole_geocentric(&self, date: f64) -> Option<(f64, f64)> {
        let g10 = self.coefficient(false, 1, 0, date)?;
        let g11 = self.coefficient(false, 1, 1, date)?;
        let h11 = self.coefficient(true, 1, 1, date)?;
        let vx = -g11;
        let vy = -h11;
        let vz = -g10;
        let r = (vx * vx + vy * vy + vz * vz).sqrt();
        if !r.is_finite() || r == 0.0 {
            return None;
        }
        let lat = (vz / r).clamp(-1.0, 1.0).asin().to_degrees();
        let lon = vy.atan2(vx).to_degrees();
        Some((lat, lon))
    }

    pub fn north_geomagnetic_pole_geodetic(&self, date: f64) -> Option<(f64, f64)> {
        let (lat_c, lon) = self.north_geomagnetic_pole_geocentric(date)?;
        let term = geodetic_to_geocentric_term();
        let lat = (lat_c.to_radians().tan() / term).atan().to_degrees();
        Some((lat, lon))
    }

    pub fn geomagnetic_latitude(
        &self,
        geodetic_lat_deg: f64,
        lon_deg: f64,
        date: f64,
    ) -> Option<f64> {
        if !geodetic_lat_deg.is_finite() || !lon_deg.is_finite() {
            return None;
        }
        let (pole_lat_c, pole_lon) = self.north_geomagnetic_pole_geocentric(date)?;
        let term = geodetic_to_geocentric_term();
        let lat_c = (geodetic_lat_deg.to_radians().tan() * term).atan();
        let pole_lat_c = pole_lat_c.to_radians();
        let dlon = (lon_deg - pole_lon).to_radians();
        let s = lat_c.sin() * pole_lat_c.sin() + lat_c.cos() * pole_lat_c.cos() * dlon.cos();
        Some(s.clamp(-1.0, 1.0).asin().to_degrees())
    }

    pub fn synthesize(
        &self,
        date: f64,
        radius_km: f64,
        reference_radius_km: f64,
        colat_deg: f64,
        lon_deg: f64,
    ) -> Option<[f64; 3]> {
        if !radius_km.is_finite() || radius_km <= 0.0 {
            return None;
        }
        if !reference_radius_km.is_finite() || reference_radius_km <= 0.0 {
            return None;
        }
        if !colat_deg.is_finite() || !(0.0..=180.0).contains(&colat_deg) {
            return None;
        }
        if !lon_deg.is_finite() {
            return None;
        }
        let pnm = schmidt_legendre(IGRF_MAX_DEGREE, colat_deg);
        let theta = colat_deg.to_radians();
        let costh = theta.cos();
        let sinth = (1.0 - costh * costh).sqrt();
        let phi = lon_deg.to_radians();
        let radius = radius_km / reference_radius_km;
        let mut b_r = 0.0;
        let mut b_theta = 0.0;
        let mut b_phi = 0.0;
        let mut r_n = radius.powi(-3);
        for n in 1..=IGRF_MAX_DEGREE {
            let g = self.coefficient(false, n, 0, date)?;
            b_r += (n as f64 + 1.0) * pnm[n][0] * r_n * g;
            b_theta += -pnm[0][n + 1] * r_n * g;
            for (m, _) in pnm.iter().enumerate().take(n + 1).skip(1) {
                let g = self.coefficient(false, n, m, date)?;
                let h = self.coefficient(true, n, m, date)?;
                let cmp = (m as f64 * phi).cos();
                let smp = (m as f64 * phi).sin();
                let gh = g * cmp + h * smp;
                b_r += (n as f64 + 1.0) * pnm[n][m] * r_n * gh;
                b_theta += -pnm[m][n + 1] * r_n * gh;
                let div_pnm = if sinth == 0.0 {
                    if costh > 0.0 {
                        pnm[m][n + 1]
                    } else {
                        -pnm[m][n + 1]
                    }
                } else {
                    pnm[n][m] / sinth
                };
                b_phi += m as f64 * div_pnm * r_n * (g * smp - h * cmp);
            }
            r_n /= radius;
        }
        Some([-b_theta, b_phi, -b_r])
    }
}

fn schmidt_legendre(nmax: usize, colat_deg: f64) -> Vec<Vec<f64>> {
    let theta = colat_deg.to_radians();
    let costh = theta.cos();
    let sinth = (1.0 - costh * costh).sqrt();
    let mut pnm = vec![vec![0.0f64; nmax + 2]; nmax + 1];
    pnm[0][0] = 1.0;
    pnm[1][1] = sinth;
    let rootn: Vec<f64> = (0..=(2 * nmax * nmax)).map(|k| (k as f64).sqrt()).collect();
    for m in 0..nmax {
        let pnm_tmp = rootn[2 * m + 1] * pnm[m][m];
        pnm[m + 1][m] = costh * pnm_tmp;
        if m > 0 {
            pnm[m + 1][m + 1] = sinth * pnm_tmp / rootn[2 * m + 2];
        }
        for n in (m + 2)..=nmax {
            let d = n * n - m * m;
            let e = 2 * n - 1;
            pnm[n][m] =
                (e as f64 * costh * pnm[n - 1][m] - rootn[d - e] * pnm[n - 2][m]) / rootn[d];
        }
    }
    pnm[0][2] = -pnm[1][1];
    pnm[1][2] = pnm[1][0];
    for n in 2..=nmax {
        pnm[0][n + 1] = -((n * n + n) as f64 / 2.0).sqrt() * pnm[n][1];
        pnm[1][n + 1] = ((2.0 * (n * n + n) as f64).sqrt() * pnm[n][0]
            - ((n * n + n - 2) as f64).sqrt() * pnm[n][2])
            / 2.0;
        for m in 2..n {
            pnm[m][n + 1] = 0.5 * (((n + m) * (n - m + 1)) as f64).sqrt() * pnm[n][m - 1]
                - 0.5 * (((n + m + 1) * (n - m)) as f64).sqrt() * pnm[n][m + 1];
        }
        pnm[n][n + 1] = ((2 * n) as f64).sqrt() * pnm[n][n - 1] / 2.0;
    }
    pnm
}

#[cfg(test)]
mod tests {
    use super::*;

    const MEASURED_COEFFS: &str = "\
g/h n m 1900.0 1905.0 1910.0 1915.0 1920.0 1925.0 1930.0 1935.0 1940.0 1945.0 1950.0 1955.0 1960.0 1965.0 1970.0 1975.0 1980.0 1985.0 1990.0 1995.0 2000.0 2005.0 2010.0 2015.0 2020.0 2025.0 2025-30
g  1  0 -31543 -31464 -31354 -31212 -31060 -30926 -30805 -30715 -30654 -30594 -30554 -30500 -30421 -30334 -30220 -30100 -29992 -29873 -29775 -29692 -29619.4 -29554.63 -29496.57 -29441.46 -29403.41 -29350.0    12.6
g  1  1  -2298  -2298  -2297  -2306  -2317  -2318  -2316  -2306  -2292  -2285  -2250  -2215  -2169  -2119  -2068  -2013  -1956  -1905  -1848  -1784  -1728.2  -1669.05  -1586.42  -1501.77  -1451.37  -1410.3    10.0
h  1  1   5922   5909   5898   5875   5845   5817   5808   5812   5821   5810   5815   5820   5791   5776   5737   5675   5604   5500   5406   5306   5186.1   5077.99   4944.26   4795.99   4653.35   4545.5   -21.5
";

    fn measured() -> IgrfCoeffs {
        IgrfCoeffs::parse(MEASURED_COEFFS).expect("the measured IGRF-14 excerpt parses")
    }

    #[test]
    fn north_geomagnetic_pole_matches_igrf14_table3() {
        let c = measured();
        let (lat, lon) = c.north_geomagnetic_pole_geodetic(1900.0).unwrap();
        assert!((lat - 78.68).abs() < 0.05, "1900.0 latitude {lat}");
        assert!((lon + 68.79).abs() < 0.05, "1900.0 longitude {lon}");
        let (lat, lon) = c.north_geomagnetic_pole_geodetic(2020.0).unwrap();
        assert!((lat - 80.65).abs() < 0.05, "2020.0 latitude {lat}");
        assert!((lon + 72.68).abs() < 0.05, "2020.0 longitude {lon}");
        let (lat, lon) = c.north_geomagnetic_pole_geodetic(2025.0).unwrap();
        assert!((lat - 80.85).abs() < 0.05, "2025.0 latitude {lat}");
        assert!((lon + 72.76).abs() < 0.05, "2025.0 longitude {lon}");
    }

    #[test]
    fn geomagnetic_latitude_is_ninety_at_the_pole() {
        let c = measured();
        let (lat, lon) = c.north_geomagnetic_pole_geodetic(2020.0).unwrap();
        let m = c.geomagnetic_latitude(lat, lon, 2020.0).unwrap();
        assert!(
            (m - 90.0).abs() < 1e-6,
            "geomagnetic latitude at the pole {m}"
        );
    }

    #[test]
    fn parse_refuses_a_row_that_does_not_match_the_epoch_count() {
        assert!(
            IgrfCoeffs::parse("g/h n m 1900.0 1905.0 2025-30\ng 1 0 -31543 -31464\n").is_none()
        );
        assert!(IgrfCoeffs::parse("").is_none());
    }

    const IGRF14_COEFFICIENTS: &str = include_str!("igrf14coeffs.txt");

    fn igrf14() -> IgrfCoeffs {
        IgrfCoeffs::parse(IGRF14_COEFFICIENTS).expect("the IGRF-14 coefficient table parses")
    }

    #[test]
    fn synthesis_matches_pyigrf14_witness_points() {
        let c = igrf14();
        let cases: [(f64, f64, f64, f64, [f64; 3]); 3] = [
            (
                1900.0,
                6300.0,
                175.0,
                -150.0,
                [-5072.93, 10620.34, -67233.55],
            ),
            (2020.0, 6700.0, 15.0, 90.0, [3734.07, 1294.17, 50833.13]),
            (2025.0, 6375.0, 56.0, -3.0, [28927.56, 261.98, 30910.08]),
        ];
        for (date, radius, colat, lon, expected) in cases {
            let xyz = c
                .synthesize(date, radius, 6371.2, colat, lon)
                .expect("the IGRF-14 table covers the date");
            for (k, &e) in expected.iter().enumerate() {
                let got = xyz[k];
                let tol = 1e-2 + 1e-2 * e.abs();
                assert!(
                    (got - e).abs() <= tol,
                    "date {date} colat {colat} lon {lon} component {k}: got {got}, witness {e}"
                );
            }
        }
    }
}
