use crate::mathematikerin::receiver::EopSample;
use std::sync::OnceLock;

const EOP_TSV: &str = include_str!("kernels/eop.tsv");

pub struct EopSeries {
    rows: Vec<(f64, f64, f64, f64)>,
}

pub fn kernel() -> Option<&'static EopSeries> {
    static SERIES: OnceLock<Option<EopSeries>> = OnceLock::new();
    SERIES.get_or_init(|| EopSeries::parse(EOP_TSV)).as_ref()
}

impl EopSeries {
    pub fn parse(text: &str) -> Option<Self> {
        let mut rows: Vec<(f64, f64, f64, f64)> = Vec::new();
        for line in text.lines() {
            if line.starts_with('#') || line.trim().is_empty() {
                continue;
            }
            let mut p = line.split('\t');
            let mjd = p.next()?.trim().parse::<f64>().ok()?;
            let ut1 = p.next()?.trim().parse::<f64>().ok()?;
            let xp = p.next()?.trim().parse::<f64>().ok()?;
            let yp = p.next()?.trim().parse::<f64>().ok()?;
            if ![mjd, ut1, xp, yp].iter().all(|v| v.is_finite()) {
                return None;
            }
            rows.push((mjd, ut1, xp, yp));
        }
        if rows.is_empty() {
            return None;
        }
        rows.sort_by(|a, b| a.0.total_cmp(&b.0));
        Some(Self { rows })
    }

    pub fn at(&self, mjd: f64) -> Option<EopSample> {
        if !mjd.is_finite() {
            return None;
        }
        let idx = self.rows.partition_point(|r| r.0 < mjd);
        if idx == 0 {
            let r = self.rows.first()?;
            if r.0 == mjd {
                return Some(sample(*r));
            }
            return None;
        }
        if idx >= self.rows.len() {
            let r = self.rows.last()?;
            if r.0 == mjd {
                return Some(sample(*r));
            }
            return None;
        }
        let a = self.rows[idx - 1];
        let b = self.rows[idx];
        if a.0 == mjd {
            return Some(sample(a));
        }
        if b.0 == mjd {
            return Some(sample(b));
        }
        let t = (mjd - a.0) / (b.0 - a.0);
        Some(EopSample {
            ut1_utc_s: a.1 + (b.1 - a.1) * t,
            pm_x_arcsec: a.2 + (b.2 - a.2) * t,
            pm_y_arcsec: a.3 + (b.3 - a.3) * t,
        })
    }
}

fn sample(r: (f64, f64, f64, f64)) -> EopSample {
    EopSample {
        ut1_utc_s: r.1,
        pm_x_arcsec: r.2,
        pm_y_arcsec: r.3,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_kernel_parses_and_covers_the_measured_era() {
        let s = kernel().expect("eop kernel");
        assert!(s.at(37665.0).is_some(), "1962-01-01");
        assert!(s.at(58000.0).is_some(), "2017");
        assert!(s.at(61000.0).is_some(), "2025");
    }

    #[test]
    fn an_exact_row_returns_its_own_value() {
        let s = kernel().expect("eop kernel");
        let e = s.at(37665.0).expect("first row");
        assert!((e.ut1_utc_s - 0.0326338).abs() < 1e-12);
        assert!((e.pm_x_arcsec - (-0.0127)).abs() < 1e-9);
        assert!((e.pm_y_arcsec - 0.2130).abs() < 1e-9);
    }

    #[test]
    fn between_two_rows_the_value_is_interpolated() {
        let s = kernel().expect("eop kernel");
        let e = s.at(37665.5).expect("midpoint");
        assert!((e.ut1_utc_s - (0.0326338 + 0.0320547) / 2.0).abs() < 1e-9);
    }

    #[test]
    fn outside_the_series_there_is_no_value() {
        let s = kernel().expect("eop kernel");
        assert!(s.at(37664.0).is_none());
        assert!(s.at(70000.0).is_none());
    }
}
