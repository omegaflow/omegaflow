use std::sync::OnceLock;
use std::time::{SystemTime, UNIX_EPOCH};

use super::types::J2000_UNIX_OFFSET;

const SECONDS_PER_JULIAN_YEAR: f64 = 86400.0 * 365.25;

const DJ00: f64 = 2_451_545.0;
const DJM: f64 = 365_250.0;
const DD2R: f64 = std::f64::consts::PI / 180.0;

const TDB_SERIES_TSV: &str = include_str!("kernels/tdb_fairhead.tsv");

fn tdb_series() -> Option<&'static Vec<(f64, f64, f64)>> {
    static SERIES: std::sync::OnceLock<Option<Vec<(f64, f64, f64)>>> = std::sync::OnceLock::new();
    SERIES
        .get_or_init(|| {
            let mut v: Vec<(f64, f64, f64)> = Vec::with_capacity(787);
            for line in TDB_SERIES_TSV.lines() {
                if line.starts_with('#') || line.trim().is_empty() {
                    continue;
                }
                let mut p = line.split('\t');
                let amp = p.next()?.trim().parse::<f64>().ok()?;
                let freq = p.next()?.trim().parse::<f64>().ok()?;
                let phase = p.next()?.trim().parse::<f64>().ok()?;
                if ![amp, freq, phase].iter().all(|x| x.is_finite()) {
                    return None;
                }
                v.push((amp, freq, phase));
            }
            if v.len() == 787 { Some(v) } else { None }
        })
        .as_ref()
}

fn delta_t_at_unix(unix: f64) -> f64 {
    let year = 2000.0 + (unix - J2000_UNIX_OFFSET) / SECONDS_PER_JULIAN_YEAR;
    super::astrometry::delta_t_espenak_meeus(year)
}

#[derive(Clone, Debug)]
pub struct LeapSeconds {
    pub delta_t_a: f64,
    pub deltas: Vec<(f64, f64)>,
}

pub(crate) fn tdb_minus_tt_full(
    date1: f64,
    date2: f64,
    ut: f64,
    elong: f64,
    u: f64,
    v: f64,
) -> Option<f64> {
    if ![date1, date2, ut, elong, u, v]
        .iter()
        .all(|x| x.is_finite())
    {
        return None;
    }
    let s = tdb_series()?;
    let t = ((date1 - DJ00) + date2) / DJM;
    let tsol = (ut % 1.0) * std::f64::consts::TAU + elong;
    let w = t / 3600.0;
    let elsun = (280.46645683_f64 + 1_296_027_711.034_29 * w).rem_euclid(360.0) * DD2R;
    let emsun = (357.52910918_f64 + 1_295_965_810.481 * w).rem_euclid(360.0) * DD2R;
    let d = (297.85019547_f64 + 16_029_616_012.090 * w).rem_euclid(360.0) * DD2R;
    let elj = (34.35151874_f64 + 109_306_899.894_53 * w).rem_euclid(360.0) * DD2R;
    let els = (50.07744430_f64 + 44_046_398.470_38 * w).rem_euclid(360.0) * DD2R;
    let wt = 0.00029e-10 * u * (tsol + elsun - els).sin()
        + 0.00100e-10 * u * (tsol - 2.0 * emsun).sin()
        + 0.00133e-10 * u * (tsol - d).sin()
        + 0.00133e-10 * u * (tsol + elsun - elj).sin()
        - 0.00229e-10 * u * (tsol + 2.0 * elsun + emsun).sin()
        - 0.02200e-10 * v * (elsun + emsun).cos()
        + 0.05312e-10 * u * (tsol - emsun).sin()
        - 0.13677e-10 * u * (tsol + 2.0 * elsun).sin()
        - 1.31840e-10 * v * elsun.cos()
        + 3.17679e-10 * u * tsol.sin();
    let mut w0 = 0.0;
    for &(a, f, p) in &s[0..474] {
        w0 += a * (f * t + p).sin();
    }
    let mut w1 = 0.0;
    for &(a, f, p) in &s[474..679] {
        w1 += a * (f * t + p).sin();
    }
    let mut w2 = 0.0;
    for &(a, f, p) in &s[679..764] {
        w2 += a * (f * t + p).sin();
    }
    let mut w3 = 0.0;
    for &(a, f, p) in &s[764..784] {
        w3 += a * (f * t + p).sin();
    }
    let mut w4 = 0.0;
    for &(a, f, p) in &s[784..787] {
        w4 += a * (f * t + p).sin();
    }
    let wf = t * (t * (t * (t * w4 + w3) + w2) + w1) + w0;
    let wj = 0.00065e-6 * (6069.776754 * t + 4.021194).sin()
        + 0.00033e-6 * (213.299095 * t + 5.543132).sin()
        + (-0.00196e-6 * (6208.294251 * t + 5.696701).sin())
        + (-0.00173e-6 * (74.781599 * t + 2.435900).sin())
        + 0.03638e-6 * t * t;
    let out = wt + wf + wj;
    if out.is_finite() { Some(out) } else { None }
}

pub(crate) fn tdb_minus_tt_seconds(unix: f64) -> Option<f64> {
    if !unix.is_finite() {
        return None;
    }
    let jd = unix / 86400.0 + 2_440_587.5;
    tdb_minus_tt_full(jd, 0.0, 0.0, 0.0, 0.0, 0.0)
}

pub struct MoyerTerm {
    pub amplitude_seconds: f64,
    pub angle_a_deg: f64,
    pub rate_a_deg_day: f64,
    pub angle_b_deg: f64,
    pub rate_b_deg_day: f64,
}

const MOYER_TSV: &str = include_str!("kernels/moyer_tdb.tsv");

pub fn moyer_terms() -> Option<&'static Vec<MoyerTerm>> {
    static TERMS: OnceLock<Option<Vec<MoyerTerm>>> = OnceLock::new();
    TERMS
        .get_or_init(|| {
            let mut out = Vec::new();
            for line in MOYER_TSV.lines() {
                if line.starts_with('#') || line.trim().is_empty() {
                    continue;
                }
                let cols: Vec<&str> = line.split('\t').collect();
                if cols.len() < 6 {
                    continue;
                }
                let v: Vec<f64> = cols[1..6]
                    .iter()
                    .filter_map(|x| x.trim().parse().ok())
                    .collect();
                if v.len() == 5 {
                    out.push(MoyerTerm {
                        amplitude_seconds: v[0],
                        angle_a_deg: v[1],
                        rate_a_deg_day: v[2],
                        angle_b_deg: v[3],
                        rate_b_deg_day: v[4],
                    });
                }
            }
            if out.is_empty() { None } else { Some(out) }
        })
        .as_ref()
}

pub fn moyer_tdb_minus_tt_seconds(unix: f64) -> Option<f64> {
    if !unix.is_finite() {
        return None;
    }
    let d = (unix - J2000_UNIX_OFFSET) / 86_400.0;
    let mut sum = 0.0;
    for term in moyer_terms()? {
        let a = (term.angle_a_deg + term.rate_a_deg_day * d).to_radians();
        let b = (term.angle_b_deg + term.rate_b_deg_day * d).to_radians();
        sum += term.amplitude_seconds * (a - b).sin();
    }
    if sum.is_finite() { Some(sum) } else { None }
}

impl LeapSeconds {
    pub fn leap_at(&self, unix: f64) -> Option<f64> {
        self.deltas
            .iter()
            .rev()
            .find(|&&(_, effective)| effective <= unix)
            .map(|&(offset, _)| offset)
    }

    pub fn unix_to_tdb(&self, unix: f64) -> Option<f64> {
        let to_tt = match self.leap_at(unix) {
            Some(leap) => self.delta_t_a + leap,
            None => {
                let &(_, first_unix) = self.deltas.first()?;
                if unix.partial_cmp(&first_unix) != Some(std::cmp::Ordering::Less) {
                    return None;
                }
                delta_t_at_unix(unix)
            }
        };
        Some(unix + to_tt - J2000_UNIX_OFFSET + tdb_minus_tt_seconds(unix)?)
    }

    pub fn tdb_to_unix(&self, tdb: f64) -> Option<f64> {
        for &(offset, _) in self.deltas.iter().rev() {
            let mut unix = tdb - self.delta_t_a - offset + J2000_UNIX_OFFSET;
            for _ in 0..2 {
                unix =
                    tdb - self.delta_t_a - offset - tdb_minus_tt_seconds(unix)? + J2000_UNIX_OFFSET;
            }
            if self.leap_at(unix) == Some(offset) {
                return Some(unix);
            }
        }
        let &(_, first_unix) = self.deltas.first()?;
        let first_tdb = first_unix + self.delta_t_a + self.leap_at(first_unix)? - J2000_UNIX_OFFSET;
        if tdb.partial_cmp(&first_tdb) != Some(std::cmp::Ordering::Less) {
            return None;
        }
        let mut unix = tdb + J2000_UNIX_OFFSET;
        for _ in 0..2 {
            unix = tdb + J2000_UNIX_OFFSET - delta_t_at_unix(unix) - tdb_minus_tt_seconds(unix)?;
        }
        Some(unix)
    }

    pub fn system_now_tdb(&self) -> Option<f64> {
        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(d) => self.unix_to_tdb(d.as_secs_f64()),
            Err(_) => None,
        }
    }
}

pub fn parse(text: &str) -> Option<LeapSeconds> {
    let delta_t_a = scan_plain_value(text, "DELTET/DELTA_T_A")?;
    let (values, complete) = scan_paren_mixed(text, "DELTET/DELTA_AT")?;
    if !complete {
        return None;
    }
    let mut deltas = Vec::new();
    let mut it = values.iter();
    while let Some(v) = it.next() {
        let date = it.next()?;
        deltas.push((*v, *date));
    }
    if deltas.is_empty() {
        return None;
    }
    Some(LeapSeconds { delta_t_a, deltas })
}

fn scan_plain_value(text: &str, key: &str) -> Option<f64> {
    let pos = text.find(key)?;
    let rest = &text[pos + key.len()..];
    let eq = rest.find('=')?;
    let after_eq = &rest[eq + 1..];
    let line = after_eq.lines().next()?;
    line.split_whitespace().next()?.parse().ok()
}

fn scan_paren_mixed(text: &str, key: &str) -> Option<(Vec<f64>, bool)> {
    let pos = text.find(key)?;
    let rest = &text[pos + key.len()..];
    let eq = rest.find('=')?;
    let after_eq = &rest[eq + 1..];
    let open = after_eq.find('(')?;
    let body = &after_eq[open + 1..];
    let close = body.find(')')?;
    let mut values = Vec::new();
    let mut complete = true;
    for tok in body[..close].split_whitespace() {
        let clean = tok.trim_end_matches(',');
        if let Some(date) = clean.strip_prefix('@') {
            values.push(date_unix(date)?);
            continue;
        }
        match clean.parse::<f64>() {
            Ok(v) => values.push(v),
            Err(_) => {
                complete = false;
                break;
            }
        }
    }
    Some((values, complete))
}

fn date_unix(date: &str) -> Option<f64> {
    let (y, rest) = date.split_once('-')?;
    let (m, d) = rest.split_once('-')?;
    let year: i64 = y.parse().ok()?;
    let month = month_of(m)?;
    let day: i64 = d.parse().ok()?;
    let days = days_from_civil(year, month, day)?;
    Some(days as f64 * 86400.0)
}

fn month_of(name: &str) -> Option<i64> {
    match name {
        "JAN" => Some(1),
        "FEB" => Some(2),
        "MAR" => Some(3),
        "APR" => Some(4),
        "MAY" => Some(5),
        "JUN" => Some(6),
        "JUL" => Some(7),
        "AUG" => Some(8),
        "SEP" => Some(9),
        "OCT" => Some(10),
        "NOV" => Some(11),
        "DEC" => Some(12),
        _ => None,
    }
}

pub fn days_from_civil(year: i64, month: i64, day: i64) -> Option<i64> {
    if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
        return None;
    }
    let y = if month <= 2 { year - 1 } else { year };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let mp = if month > 2 { month - 3 } else { month + 9 };
    let doy = (153 * mp + 2) / 5 + day - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    Some(era * 146097 + doe - 719468)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::archivar::embedded_lsk;

    #[test]
    fn parses_naif_style_entries() {
        let text = "KPL/LSK\n\
[2]       DELTA_AT  =  TAI - UTC\n\
[3]       DELTA_ET  =  ET - (TAI - DELTA_AT)\n\
\\begindata\n\n\
DELTET/DELTA_T_A       =   32.184\n\
DELTET/K               =    1.657D-3\n\
DELTET/M               = (  6.239996D0   1.99096871D-7 )\n\n\
DELTET/DELTA_AT        = ( 10,   @1972-JAN-1,\n 37,   @2017-JAN-1 )\n";
        let lsk = parse(text).unwrap();
        assert!((lsk.delta_t_a - 32.184).abs() < 1e-12);
        let now = 1700000000.0;
        assert_eq!(lsk.leap_at(now), Some(37.0));
        let tdb = lsk
            .unix_to_tdb(now)
            .expect("a post-1972 unix carries a tdb");
        let tt = now + 69.184 - J2000_UNIX_OFFSET;
        assert!(
            (tdb - tt).abs() < 1.8e-3,
            "the tdb periodic term stays under 1.8 ms, was {}",
            tdb - tt
        );
        assert_eq!(lsk.tdb_to_unix(lsk.unix_to_tdb(now).unwrap()), Some(now));
    }

    #[test]
    fn tdb_minus_tt_matches_the_erfa_published_vector() {
        let got = tdb_minus_tt_full(2_448_939.5, 0.123, 0.76543, 5.0123, 5525.242, 3190.0)
            .expect("the frozen series parses");
        let want = -1.280_368_005_936_999e-3;
        assert!(
            (got - want).abs() <= 1e-15,
            "eraDtdb vector: got {got:.18e}, want {want:.18e}"
        );
    }

    #[test]
    fn the_frozen_series_carries_787_terms() {
        assert_eq!(tdb_series().expect("series").len(), 787);
        assert!(tdb_minus_tt_full(f64::NAN, 0.0, 0.0, 0.0, 0.0, 0.0).is_none());
    }

    #[test]
    fn absent_entries_leave_none() {
        assert!(parse("DELTET/DELTA_T_A = ( 32.184 )\n").is_none());
        assert!(parse("DELTA_AT = ( 37, @2017-JAN-1 )\n").is_none());
    }

    #[test]
    fn unix_to_tdb_carries_the_j2000_domain() {
        let text = "KPL/LSK\n\
[2]       DELTA_AT  =  TAI - UTC\n\
[3]       DELTA_ET  =  ET - (TAI - DELTA_AT)\n\
\\begindata\n\n\
DELTET/DELTA_T_A       =   32.184\n\
DELTET/K               =    1.657D-3\n\
DELTET/M               = (  6.239996D0   1.99096871D-7 )\n\n\
DELTET/DELTA_AT        = ( 10,   @1972-JAN-1,\n 37,   @2017-JAN-1 )\n";
        let lsk = parse(text).unwrap();
        let unix_2026 = 1_785_000_000.0;
        let tdb = lsk
            .unix_to_tdb(unix_2026)
            .expect("post-1972 unix carries a tdb");
        assert!(
            (tdb - 838_272_069.184).abs() < 1.0,
            "a 2026 unix epoch maps near J2000-relative tdb, was {tdb}"
        );
        let unix_1968 = -40_000_000.0;
        let tdb_1968 = lsk
            .unix_to_tdb(unix_1968)
            .expect("a 1968 UT1 epoch resolves through the Delta T polynomial");
        let expect_1968 = unix_1968 + delta_t_at_unix(unix_1968) - J2000_UNIX_OFFSET
            + tdb_minus_tt_seconds(unix_1968).expect("the frozen series parses");
        assert!(
            (tdb_1968 - expect_1968).abs() < 1e-9,
            "pre-1972 carries TDB = UT1 + Delta T + (TDB-TT), was {tdb_1968}"
        );
        let back = lsk
            .tdb_to_unix(tdb_1968)
            .expect("the pre-1972 tdb inverts to its ut1 epoch");
        assert!(
            (back - unix_1968).abs() < 1e-6,
            "the pre-1972 round trips through the Delta T fixed point, was {back}"
        );
        assert!(
            lsk.unix_to_tdb(f64::NAN).is_none(),
            "an epoch with no anchor stays void — no fabricated value"
        );
    }

    #[test]
    fn the_series_carries_the_periodic_term_at_its_physical_amplitude() {
        let lsk = embedded_lsk().expect("the embedded leap-second kernel");
        let epochs = [
            1_234_567_890.0,
            1_500_000_000.0,
            1_700_000_000.0,
            1_800_000_000.0,
        ];
        let mut largest = 0.0f64;
        for unix in epochs {
            let leap = lsk.leap_at(unix).expect("an epoch carries a leap");
            let tdb = lsk.unix_to_tdb(unix).expect("a TDB value");
            let naive = unix + 32.184 + leap - J2000_UNIX_OFFSET;
            let gap = (tdb - naive).abs();
            assert!(
                gap <= 1.8e-3,
                "unix {unix}: the TDB-TT periodic term never exceeds 1.8 ms, gap {gap}"
            );
            if gap > largest {
                largest = gap;
            }
        }
        assert!(
            largest >= 1.0e-3,
            "the periodic term must reach its 1.6 ms amplitude somewhere, largest {largest}"
        );
    }

    #[test]
    fn the_moyer_realisation_agrees_with_the_full_series() {
        for unix in [
            1_234_567_890.0,
            1_500_000_000.0,
            1_700_000_000.0,
            1_800_000_000.0,
            2_000_000_000.0,
        ] {
            let full = super::tdb_minus_tt_seconds(unix).expect("the Fairhead series");
            let moyer = moyer_tdb_minus_tt_seconds(unix).expect("the Moyer realisation");
            let diff = (full - moyer).abs();
            assert!(
                diff < 1.0e-4,
                "unix {unix}: Fairhead {full}, Moyer {moyer}, difference {diff} s"
            );
        }
    }

    #[test]
    fn the_moyer_realisation_is_absent_for_a_void_epoch() {
        assert!(moyer_tdb_minus_tt_seconds(f64::NAN).is_none());
    }
}
