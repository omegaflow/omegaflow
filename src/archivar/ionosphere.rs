pub struct Klobuchar {
    pub alpha: [f64; 4],
    pub beta: [f64; 4],
}

const C_M_S: f64 = 299_792_458.0;
const L1_HZ: f64 = 1_575_420_000.0;

impl Klobuchar {
    pub fn vertical_tec(&self, lat_deg: f64, lon_deg: f64, utc_s: f64) -> Option<f64> {
        if !lat_deg.is_finite()
            || !lon_deg.is_finite()
            || !utc_s.is_finite()
            || lat_deg.abs() > 90.0
            || !(0.0..86_400.0).contains(&utc_s)
        {
            return None;
        }
        let phi_u = lat_deg / 180.0;
        let lam_u = lon_deg / 180.0;
        let elevation = 0.25;
        let psi = 0.0137 / (elevation + 0.11) - 0.022;
        let phi_i = (phi_u + psi).clamp(-0.416, 0.416);
        let lam_i = lam_u;
        let phi_m = phi_i + 0.064 * (lam_i - 1.617).cos();
        let mut t = 4.32e4 * lam_i + utc_s;
        if t >= 86_400.0 {
            t -= 86_400.0;
        }
        if t < 0.0 {
            t += 86_400.0;
        }
        let amplitude = poly(&self.alpha, phi_m).max(0.0);
        let period = poly(&self.beta, phi_m).max(72_000.0);
        let x = 2.0 * std::f64::consts::PI * (t - 50_400.0) / period;
        let delay_s =
            5.0e-9 + amplitude * (1.0 - x * x / 2.0 + x * x * x * x / 24.0);
        let tec = delay_s * C_M_S * L1_HZ * L1_HZ / 40.3 / 1.0e16;
        if tec.is_finite() { Some(tec) } else { None }
    }
}

fn poly(c: &[f64; 4], x: f64) -> f64 {
    c[0] + c[1] * x + c[2] * x * x + c[3] * x * x * x
}

pub struct TecSample {
    pub time_utc: String,
    pub lat_deg: f64,
    pub lon_deg: f64,
    pub champ_vtec: f64,
    pub iri_tec: f64,
    pub klobuchar_tec: f64,
}

const TABLE: &str = include_str!("kernels/ionosphere_tec.tsv");

pub fn parse_table(text: &str) -> Option<Vec<TecSample>> {
    let mut out = Vec::new();
    for line in text.lines() {
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }
        let cols: Vec<&str> = line.split('\t').collect();
        if cols.len() < 6 {
            return None;
        }
        out.push(TecSample {
            time_utc: cols[0].trim().to_string(),
            lat_deg: cols[1].trim().parse().ok()?,
            lon_deg: cols[2].trim().parse().ok()?,
            champ_vtec: cols[3].trim().parse().ok()?,
            iri_tec: cols[4].trim().parse().ok()?,
            klobuchar_tec: cols[5].trim().parse().ok()?,
        });
    }
    if out.is_empty() { None } else { Some(out) }
}

pub fn fixture() -> Option<Vec<TecSample>> {
    parse_table(TABLE)
}

pub fn coefficient_set() -> Klobuchar {
    Klobuchar {
        alpha: [0.6519e-8, 0.1490e-7, -0.5960e-7, -0.1192e-6],
        beta: [0.7782e5, 0.3277e5, -0.6554e5, -0.1966e6],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_broadcast_model_has_its_night_floor() {
        let quiet = Klobuchar {
            alpha: [0.0; 4],
            beta: [0.0; 4],
        };
        let tec = quiet.vertical_tec(45.0, 10.0, 20_000.0).expect("a value");
        assert!(
            (9.0..9.6).contains(&tec),
            "the 5 ns floor is about 9.2 TECU, was {tec}"
        );
    }

    #[test]
    fn the_fixture_carries_four_samples_with_three_sources() {
        let rows = fixture().expect("ionosphere fixture");
        assert_eq!(rows.len(), 4);
        for r in &rows {
            assert!(r.lat_deg.abs() <= 90.0);
            assert!(r.champ_vtec > 0.0 && r.iri_tec > 0.0 && r.klobuchar_tec > 0.0);
        }
    }

    #[test]
    fn iri_holds_against_the_champ_observation() {
        let rows = fixture().expect("ionosphere fixture");
        for r in &rows {
            let diff = (r.iri_tec - r.champ_vtec).abs();
            assert!(
                diff <= 2.0,
                "{}: IRI {:.2} vs CHAMP {:.2} differ by {:.2} TECU",
                r.time_utc,
                r.iri_tec,
                r.champ_vtec,
                diff
            );
        }
    }

    #[test]
    fn the_broadcast_model_sits_above_the_observation_by_the_measured_factor() {
        let rows = fixture().expect("ionosphere fixture");
        for r in &rows {
            let ratio = r.klobuchar_tec / r.champ_vtec;
            assert!(
                (1.2..2.0).contains(&ratio),
                "{}: Klobuchar {:.2} vs CHAMP {:.2} is a factor {ratio:.2}",
                r.time_utc,
                r.klobuchar_tec,
                r.champ_vtec
            );
        }
    }

    #[test]
    fn a_point_outside_the_day_or_the_latitude_is_absent() {
        let k = coefficient_set();
        assert!(k.vertical_tec(45.0, 10.0, 90_000.0).is_none());
        assert!(k.vertical_tec(95.0, 10.0, 1000.0).is_none());
        assert!(k.vertical_tec(f64::NAN, 10.0, 1000.0).is_none());
    }

    #[test]
    fn the_measured_coefficients_reproduce_the_fixture() {
        let k = coefficient_set();
        let rows = fixture().expect("ionosphere fixture");
        for r in &rows {
            let minute: f64 = r.time_utc[14..16].parse().expect("minute");
            let second: f64 = r.time_utc[17..19].parse().expect("second");
            let seconds = 20.0 * 3600.0 + minute * 60.0 + second;
            let tec = k
                .vertical_tec(r.lat_deg, r.lon_deg, seconds)
                .expect("a value");
            assert!(
                (tec - r.klobuchar_tec).abs() < 0.05,
                "{}: recomputed {tec:.3} vs fixture {:.3}",
                r.time_utc,
                r.klobuchar_tec
            );
        }
    }
}
