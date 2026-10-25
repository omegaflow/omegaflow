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
            ut1_utc_s: ut1_utc_between(a, b, mjd, t)?,
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

pub const EOP_INTERPOLATION_CONVENTION: &str = "UT1-UTC carries the 27 one-second steps of the series, so it is not interpolable across one; the reader interpolates UT1-TAI = UT1-UTC - (TAI-UTC) linearly and adds TAI-UTC back (SOFA eraUtcut1, notes 2 and 5); before 1972-01-01 UTC had no one-second steps and the plain interpolation is the exact operation there; a row pair that spans that boundary has no common scale and stays absent";

fn ut1_utc_between(
    a: (f64, f64, f64, f64),
    b: (f64, f64, f64, f64),
    mjd: f64,
    t: f64,
) -> Option<f64> {
    let lsk = crate::archivar::membrane::embedded_lsk()?;
    let leap = |m: f64| lsk.leap_at((m - 40587.0) * 86400.0);
    match (leap(a.0), leap(b.0), leap(mjd)) {
        (Some(la), Some(lb), Some(le)) => {
            let a_tai = a.1 - la;
            let b_tai = b.1 - lb;
            Some(a_tai + (b_tai - a_tai) * t + le)
        }
        (None, None, None) => Some(a.1 + (b.1 - a.1) * t),
        _ => None,
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

    #[test]
    fn the_leap_second_edge_is_not_ramped() {
        let s = kernel().expect("eop kernel");
        let e = s.at(57753.5).expect("midpoint of the leap day");
        let a_tai = -0.4077697 - 36.0;
        let b_tai = 0.5912870 - 37.0;
        let want = (a_tai + b_tai) / 2.0 + 36.0;
        assert!((e.ut1_utc_s - want).abs() < 1.0e-9, "{} vs {want}", e.ut1_utc_s);
        assert!(e.ut1_utc_s < 0.0, "the ramp came back: {}", e.ut1_utc_s);
    }
}
