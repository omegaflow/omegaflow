use crate::mathematikerin::receiver::EopSample;
use std::sync::OnceLock;

const EOP_TSV: &str = include_str!("kernels/eop.tsv");

type Row = (f64, f64, f64, f64, f64, f64, f64);

pub struct EopSeries {
    rows: Vec<Row>,
}

pub fn kernel() -> Option<&'static EopSeries> {
    static SERIES: OnceLock<Option<EopSeries>> = OnceLock::new();
    SERIES.get_or_init(|| EopSeries::parse(EOP_TSV)).as_ref()
}

impl EopSeries {
    pub fn parse(text: &str) -> Option<Self> {
        let mut rows: Vec<Row> = Vec::new();
        for line in text.lines() {
            if line.starts_with('#') || line.trim().is_empty() {
                continue;
            }
            let mut p = line.split('\t');
            let mut v = [0.0f64; 7];
            for slot in &mut v {
                *slot = p.next()?.trim().parse::<f64>().ok()?;
            }
            if !v.iter().all(|x| x.is_finite()) {
                return None;
            }
            rows.push((v[0], v[1], v[2], v[3], v[4], v[5], v[6]));
        }
        if rows.is_empty() {
            return None;
        }
        rows.sort_by(|a, b| a.0.total_cmp(&b.0));
        Some(Self { rows })
    }

    pub fn rows(&self) -> usize {
        self.rows.len()
    }

    fn bracketing(&self, mjd: f64) -> Option<(Row, Option<(Row, f64)>)> {
        if !mjd.is_finite() {
            return None;
        }
        let idx = self.rows.partition_point(|r| r.0 < mjd);
        if idx == 0 {
            let r = *self.rows.first()?;
            if r.0 == mjd {
                return Some((r, None));
            }
            return None;
        }
        if idx >= self.rows.len() {
            let r = *self.rows.last()?;
            if r.0 == mjd {
                return Some((r, None));
            }
            return None;
        }
        let a = self.rows[idx - 1];
        let b = self.rows[idx];
        if a.0 == mjd {
            return Some((a, None));
        }
        if b.0 == mjd {
            return Some((b, None));
        }
        let t = (mjd - a.0) / (b.0 - a.0);
        Some((a, Some((b, t))))
    }

    pub fn at(&self, mjd: f64) -> Option<EopSample> {
        let (a, next) = self.bracketing(mjd)?;
        let Some((b, t)) = next else {
            return Some(sample(a));
        };
        Some(EopSample {
            ut1_utc_s: ut1_utc_between(a, b, mjd, t)?,
            pm_x_arcsec: a.2 + (b.2 - a.2) * t,
            pm_y_arcsec: a.3 + (b.3 - a.3) * t,
        })
    }

    pub fn lod_dx_dy(&self, mjd: f64) -> Option<(f64, f64, f64)> {
        let (a, next) = self.bracketing(mjd)?;
        let Some((b, t)) = next else {
            return Some((a.4, a.5, a.6));
        };
        let lod = a.4 + (b.4 - a.4) * t;
        let dx = a.5 + (b.5 - a.5) * t;
        let dy = a.6 + (b.6 - a.6) * t;
        if [lod, dx, dy].iter().all(|v| v.is_finite()) {
            Some((lod, dx, dy))
        } else {
            None
        }
    }
}

fn sample(r: Row) -> EopSample {
    EopSample {
        ut1_utc_s: r.1,
        pm_x_arcsec: r.2,
        pm_y_arcsec: r.3,
    }
}

pub const EOP_INTERPOLATION_CONVENTION: &str = "UT1-UTC carries the 27 one-second steps of the series, so it is not interpolable across one; the reader interpolates UT1-TAI = UT1-UTC - (TAI-UTC) linearly and adds TAI-UTC back (SOFA eraUtcut1, notes 2 and 5); before 1972-01-01 UTC had no one-second steps and the plain interpolation is the exact operation there; a row pair that spans that boundary has no common scale and stays absent";

pub const EOP_FIVE_QUANTITIES_CONVENTION: &str = "the kernel carries the five IERS C04 quantities per day: UT1-UTC (s), x_p and y_p (arcsec), LOD (s) and the celestial pole offsets dX and dY (arcsec); LOD, dX and dY are interpolated linearly between adjacent rows, like the polar motion, because neither carries a step; the source column layout is named in the kernel header (tokens 0-based YJ MD DH MJD x y UT1 dX dY xrt yrt LOD), so a different C04 mirror with another order is a different kernel and must not be read with this one";

pub const DELTA_T_CONVENTION: &str = "Delta T = TT - UT1 = 32.184 s + (TAI-UTC) - (UT1-UTC); TAI-UTC from the embedded leap-second table, UT1-UTC from the EOP kernel; absent where either has no entry (before 1972-01-01, or outside the series), so the caller names the class instead of inheriting a polynomial";

pub fn delta_t_unix(unix: f64) -> Option<f64> {
    if !unix.is_finite() {
        return None;
    }
    let lsk = crate::archivar::membrane::embedded_lsk()?;
    let tai_utc = lsk.leap_at(unix)?;
    let mjd = unix / 86400.0 + 40587.0;
    let eop = kernel()?.at(mjd)?;
    let dt = 32.184 + tai_utc - eop.ut1_utc_s;
    if dt.is_finite() { Some(dt) } else { None }
}

fn ut1_utc_between(a: Row, b: Row, mjd: f64, t: f64) -> Option<f64> {
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
        assert_eq!(s.rows(), 23592);
    }

    #[test]
    fn an_exact_row_returns_its_own_value() {
        let s = kernel().expect("eop kernel");
        let e = s.at(37665.0).expect("first row");
        assert!((e.ut1_utc_s - 0.0326338).abs() < 1e-12);
        assert!((e.pm_x_arcsec - (-0.0127)).abs() < 1e-9);
        assert!((e.pm_y_arcsec - 0.2130).abs() < 1e-9);
        let (lod, dx, dy) = s.lod_dx_dy(37665.0).expect("first row extras");
        assert!((lod - 0.0017230).abs() < 1e-10, "{lod}");
        assert!(
            dx.abs() < 1e-12 && dy.abs() < 1e-12,
            "1962 has no pole offset"
        );
    }

    #[test]
    fn the_last_row_carries_the_measured_pole_offsets() {
        let s = kernel().expect("eop kernel");
        let (lod, dx, dy) = s.lod_dx_dy(61256.0).expect("last row extras");
        assert!((lod - 0.0004774).abs() < 1e-10, "{lod}");
        assert!((dx - 0.000406).abs() < 1e-9, "{dx}");
        assert!((dy - (-0.000270)).abs() < 1e-9, "{dy}");
    }

    #[test]
    fn between_two_rows_the_value_is_interpolated() {
        let s = kernel().expect("eop kernel");
        let e = s.at(37665.5).expect("midpoint");
        assert!((e.ut1_utc_s - (0.0326338 + 0.0320547) / 2.0).abs() < 1e-9);
        let (lod, dx, dy) = s.lod_dx_dy(37665.5).expect("midpoint extras");
        assert!((lod - (0.0017230 + 0.0016690) / 2.0).abs() < 1e-10, "{lod}");
        assert!(dx.abs() < 1e-12 && dy.abs() < 1e-12);
    }

    #[test]
    fn outside_the_series_there_is_no_value() {
        let s = kernel().expect("eop kernel");
        assert!(s.at(37664.0).is_none());
        assert!(s.at(70000.0).is_none());
        assert!(s.lod_dx_dy(37664.0).is_none());
        assert!(s.lod_dx_dy(70000.0).is_none());
    }

    #[test]
    fn delta_t_matches_the_usno_series() {
        let cases = [
            (946684800.0, 63.8285),
            (1501545600.0, 68.8373),
            (1577836800.0, 69.3612),
            (1785542400.0, 69.1713),
        ];
        for (unix, want) in cases {
            let got = delta_t_unix(unix).expect("delta t");
            assert!((got - want).abs() < 2.0e-3, "{unix}: {got} vs {want}");
        }
        assert!(delta_t_unix(0.0).is_none());
    }

    #[test]
    fn the_leap_second_edge_is_not_ramped() {
        let s = kernel().expect("eop kernel");
        let e = s.at(57753.5).expect("midpoint of the leap day");
        let a_tai = -0.4077697 - 36.0;
        let b_tai = 0.5912870 - 37.0;
        let want = (a_tai + b_tai) / 2.0 + 36.0;
        assert!(
            (e.ut1_utc_s - want).abs() < 1.0e-9,
            "{} vs {want}",
            e.ut1_utc_s
        );
        assert!(e.ut1_utc_s < 0.0, "the ramp came back: {}", e.ut1_utc_s);
    }

    const BULLETIN_A: &str = include_str!("kernels/eop_bulletina.tsv");

    #[test]
    fn the_c04_kernel_agrees_with_bulletin_a() {
        let s = kernel().expect("eop kernel");
        let mut n = 0usize;
        for line in BULLETIN_A.lines() {
            if line.starts_with('#') || line.trim().is_empty() {
                continue;
            }
            let c: Vec<&str> = line.split('\t').collect();
            assert_eq!(c.len(), 4, "a bulletin row carries four columns");
            let mjd: f64 = c[0].trim().parse().expect("mjd");
            let x: f64 = c[1].trim().parse().expect("x");
            let y: f64 = c[2].trim().parse().expect("y");
            let ut1: f64 = c[3].trim().parse().expect("ut1");
            let e = s.at(mjd).expect("an EOP value at a bulletin day");
            let dx_mas = (e.pm_x_arcsec - x).abs() * 1000.0;
            let dy_mas = (e.pm_y_arcsec - y).abs() * 1000.0;
            let dut1_us = (e.ut1_utc_s - ut1).abs() * 1.0e6;
            assert!(dx_mas <= 0.1, "mjd {mjd}: x differs by {dx_mas} mas");
            assert!(dy_mas <= 0.1, "mjd {mjd}: y differs by {dy_mas} mas");
            assert!(dut1_us <= 1.0, "mjd {mjd}: UT1-UTC differs by {dut1_us} us");
            n += 1;
        }
        assert_eq!(n, 4, "four bulletin days are compared");
    }

    const CODE_ERP: &str = include_str!("kernels/eop_code_erp.tsv");

    #[test]
    fn the_c04_kernel_agrees_with_the_code_mgex_erp() {
        let s = kernel().expect("eop kernel");
        let mut n = 0usize;
        for line in CODE_ERP.lines() {
            if line.starts_with('#') || line.trim().is_empty() {
                continue;
            }
            let c: Vec<&str> = line.split('\t').collect();
            assert_eq!(c.len(), 4, "an ERP row carries four columns");
            let mjd: f64 = c[0].trim().parse().expect("mjd");
            let x: f64 = c[1].trim().parse().expect("x");
            let y: f64 = c[2].trim().parse().expect("y");
            let ut1: f64 = c[3].trim().parse().expect("ut1");
            let e = s.at(mjd).expect("an EOP value at an ERP day");
            let dx_mas = (e.pm_x_arcsec - x).abs() * 1000.0;
            let dy_mas = (e.pm_y_arcsec - y).abs() * 1000.0;
            let dut1_us = (e.ut1_utc_s - ut1).abs() * 1.0e6;
            assert!(dx_mas <= 0.3, "mjd {mjd}: x differs by {dx_mas} mas");
            assert!(dy_mas <= 0.3, "mjd {mjd}: y differs by {dy_mas} mas");
            assert!(
                dut1_us <= 20.0,
                "mjd {mjd}: UT1-UTC differs by {dut1_us} us"
            );
            n += 1;
        }
        assert_eq!(n, 4, "four ERP days are compared");
    }
}
