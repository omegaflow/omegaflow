use crate::lsk::days_from_civil;

pub const WATS_FILL_SCALAR: f64 = -9.0;
pub const WATS_FILL_VELOCITY: f64 = 9999.0;
pub const IDM_FILL: f64 = 9_999_999.0;
pub const IDM_PAIRS_PER_LINE: usize = 6;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WatsRecord {
    pub date_yyddd: i64,
    pub ut_ms: i64,
    pub mode: i64,
    pub slot: i64,
    pub outin: i64,
    pub mass_amu: i64,
    pub density_cm3: f64,
    pub tn_k: f64,
    pub tn_corr_k: f64,
    pub v_sc_ms: f64,
    pub c1: i64,
    pub c2: i64,
    pub t1: i64,
    pub t2: i64,
    pub v_geo_ms: f64,
    pub v_geo_cor_ms: f64,
    pub orbit: i64,
    pub altitude_km: f64,
    pub latitude_deg: f64,
    pub longitude_deg: f64,
    pub lst_h: f64,
    pub lmt_h: f64,
    pub l_shell: f64,
    pub invariant_latitude_deg: f64,
    pub sza_deg: f64,
}

impl WatsRecord {
    pub fn unix_ms(&self) -> Option<i64> {
        yyddd_unix_ms(self.date_yyddd, self.ut_ms)
    }

    pub fn corrected_wind_ms(&self) -> Option<f64> {
        usable_velocity(self.v_geo_cor_ms)
    }

    pub fn corrected_wind_sc_ms(&self) -> Option<f64> {
        usable_velocity(self.v_sc_ms)
    }
}

fn usable_velocity(v: f64) -> Option<f64> {
    (v.is_finite() && v.abs() < WATS_FILL_VELOCITY).then_some(v)
}

pub fn yyddd_unix_ms(date_yyddd: i64, ut_ms: i64) -> Option<i64> {
    let yy = date_yyddd.div_euclid(1000);
    let ddd = date_yyddd.rem_euclid(1000);
    if !(1..=366).contains(&ddd) || !(0..=86_400_000).contains(&ut_ms) {
        return None;
    }
    let year = 1900 + yy;
    let day0 = days_from_civil(year, 1, 1)?;
    Some((day0 + ddd - 1) * 86_400_000 + ut_ms)
}

fn cell(line: &str, start: usize, end: usize) -> Option<&str> {
    let s = line.get(start..end)?;
    if s.trim().is_empty() { None } else { Some(s) }
}

fn num<T: std::str::FromStr>(line: &str, start: usize, end: usize) -> Option<T> {
    cell(line, start, end)?.trim().parse().ok()
}

fn is_wats_header(line: &str) -> bool {
    let Some(date) = cell(line, 1, 6) else {
        return false;
    };
    date.trim().len() == 5 && date.trim().bytes().all(|b| b.is_ascii_digit())
}

pub fn parse_wats_record(head: &str, data: &str) -> Option<WatsRecord> {
    Some(WatsRecord {
        date_yyddd: num(head, 1, 6)?,
        ut_ms: num(head, 6, 15)?,
        mode: num(head, 15, 17)?,
        slot: num(head, 17, 18)?,
        outin: num(head, 18, 19)?,
        mass_amu: num(head, 19, 22)?,
        density_cm3: num(head, 22, 34)?,
        tn_k: num(head, 34, 41)?,
        tn_corr_k: num(head, 41, 48)?,
        v_sc_ms: num(head, 48, 56)?,
        c1: num(head, 57, 62)?,
        c2: num(head, 62, 67)?,
        t1: num(head, 68, 71)?,
        t2: num(head, 71, 74)?,
        v_geo_ms: num(data, 1, 9)?,
        v_geo_cor_ms: num(data, 9, 17)?,
        orbit: num(data, 17, 23)?,
        altitude_km: num(data, 23, 30)?,
        latitude_deg: num(data, 30, 36)?,
        longitude_deg: num(data, 36, 43)?,
        lst_h: num(data, 43, 49)?,
        lmt_h: num(data, 49, 55)?,
        l_shell: num(data, 55, 60)?,
        invariant_latitude_deg: num(data, 60, 66)?,
        sza_deg: num(data, 66, 73)?,
    })
}

pub fn parse_wats_2s(text: &str) -> Vec<WatsRecord> {
    let lines: Vec<&str> = text.lines().map(|l| l.trim_end_matches('\r')).collect();
    let mut out = Vec::new();
    let mut i = 0usize;
    while i + 1 < lines.len() {
        if !is_wats_header(lines[i]) {
            i += 1;
            continue;
        }
        if let Some(rec) = parse_wats_record(lines[i], lines[i + 1]) {
            out.push(rec);
        }
        i += 2;
    }
    out
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdmAxis {
    HorizontalZ,
    VerticalY,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IdmSample {
    pub ut_ms: i64,
    pub velocity_ms: f64,
    pub quality: u8,
    pub axis: IdmAxis,
}

impl IdmSample {
    pub fn quality_good(&self) -> bool {
        self.quality <= 1
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct IdmRecord {
    pub date_yyddd: i64,
    pub ut_ms: i64,
    pub latitude_deg: f64,
    pub longitude_deg: f64,
    pub invariant_latitude_deg: f64,
    pub mlt_h: f64,
    pub altitude_km: f64,
    pub sc_velocity_ms: f64,
    pub samples: Vec<IdmSample>,
}

impl IdmRecord {
    pub fn unix_ms(&self) -> Option<i64> {
        yyddd_unix_ms(self.date_yyddd, self.ut_ms)
    }

    pub fn good_horizontal(&self) -> impl Iterator<Item = &IdmSample> {
        self.samples
            .iter()
            .filter(|s| s.axis == IdmAxis::HorizontalZ && s.quality_good())
    }

    pub fn good_vertical(&self) -> impl Iterator<Item = &IdmSample> {
        self.samples
            .iter()
            .filter(|s| s.axis == IdmAxis::VerticalY && s.quality_good())
    }
}

fn is_idm_header(line: &str) -> bool {
    let Some(date) = cell(line, 1, 6) else {
        return false;
    };
    let Some(count) = cell(line, 81, 85) else {
        return false;
    };
    let date = date.trim();
    date.len() == 5
        && date.bytes().all(|b| b.is_ascii_digit())
        && count.trim().bytes().all(|b| b.is_ascii_digit())
}

pub fn parse_idm_velocity(token: &str) -> Option<(f64, u8, IdmAxis)> {
    let token = token.trim();
    let (int_part, frac) = token.split_once('.')?;
    let value: f64 = int_part.parse().ok()?;
    let mut digits = frac.bytes();
    let quality = digits.next()?.checked_sub(b'0')?;
    let direction = digits.next()?.checked_sub(b'0')?;
    if quality > 9 || direction > 9 {
        return None;
    }
    let axis = if direction % 2 == 0 {
        IdmAxis::HorizontalZ
    } else {
        IdmAxis::VerticalY
    };
    Some((value, quality, axis))
}

pub fn parse_idm_record(head: &str, pair_lines: &[&str]) -> Option<IdmRecord> {
    let date_yyddd = num(head, 1, 6)?;
    let ut_ms = num(head, 6, 15)?;
    let latitude_deg = num(head, 15, 26)?;
    let longitude_deg = num(head, 26, 37)?;
    let invariant_latitude_deg = num(head, 37, 48)?;
    let mlt_h = num(head, 48, 59)?;
    let altitude_km = num(head, 59, 70)?;
    let sc_velocity_ms = num(head, 70, 81)?;
    let count: usize = num(head, 81, 85)?;
    let mut samples = Vec::with_capacity(count);
    for line in pair_lines {
        let mut k = 0usize;
        while k < line.len() && samples.len() < count {
            let time = cell(line, k, k + 9)?;
            let velocity = cell(line, k + 9, k + 20)?;
            k += 20;
            let ut_ms = time.trim().parse().ok()?;
            let (value, quality, axis) = parse_idm_velocity(velocity)?;
            samples.push(IdmSample {
                ut_ms,
                velocity_ms: value,
                quality,
                axis,
            });
        }
        if samples.len() == count {
            break;
        }
    }
    if samples.len() != count {
        return None;
    }
    Some(IdmRecord {
        date_yyddd,
        ut_ms,
        latitude_deg,
        longitude_deg,
        invariant_latitude_deg,
        mlt_h,
        altitude_km,
        sc_velocity_ms,
        samples,
    })
}

pub fn parse_idm_250ms(text: &str) -> Vec<IdmRecord> {
    let lines: Vec<&str> = text.lines().map(|l| l.trim_end_matches('\r')).collect();
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < lines.len() {
        if !is_idm_header(lines[i]) {
            i += 1;
            continue;
        }
        let count: usize = match num(lines[i], 81, 85) {
            Some(n) => n,
            None => {
                i += 1;
                continue;
            }
        };
        if count == 0 {
            i += 1;
            continue;
        }
        let need_lines = count.div_ceil(IDM_PAIRS_PER_LINE);
        let start = i + 1;
        let end = (start + need_lines).min(lines.len());
        let pairs = &lines[start..end];
        if let Some(rec) = parse_idm_record(lines[i], pairs) {
            out.push(rec);
        }
        i = end.max(start);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    const WATS: &str = include_str!("de2_fixtures/wats_2s_1981230.asc");
    const IDM: &str = include_str!("de2_fixtures/idm_250ms_1981227T0344.asc");

    #[test]
    fn wats_fixed_width_fields_that_touch_are_separated_by_column() {
        let recs = parse_wats_2s(WATS);
        assert_eq!(recs.len(), 8);
        let r = &recs[0];
        assert_eq!(r.date_yyddd, 81230);
        assert_eq!(r.ut_ms, 36_415_461);
        assert_eq!((r.mode, r.slot, r.outin, r.mass_amu), (3, 1, 1, 32));
        assert!((r.density_cm3 - 314_820.0).abs() < 1e-6);
        assert_eq!(r.tn_k, -9.0);
        assert_eq!(r.v_sc_ms, 9999.0);
        assert_eq!(r.v_geo_ms, 9999.0);
        assert_eq!(r.v_geo_cor_ms, 9999.0);
        assert_eq!(r.orbit, 221);
        assert!((r.altitude_km - 573.3).abs() < 1e-9);
        assert!((r.latitude_deg - 75.6).abs() < 1e-9);
        assert!((r.longitude_deg - 42.4).abs() < 1e-9);
        assert!((r.lst_h - 12.90).abs() < 1e-9);
        assert!((r.lmt_h - 14.78).abs() < 1e-9);
        assert!((r.l_shell - 2.88).abs() < 1e-9);
        assert!((r.invariant_latitude_deg - 71.5).abs() < 1e-9);
        assert!((r.sza_deg - 62.7).abs() < 1e-9);
        assert_eq!(r.corrected_wind_ms(), None);

        let r2 = &recs[1];
        assert_eq!((r2.mode, r2.slot, r2.outin, r2.mass_amu), (3, 2, 0, 32));
        assert!((r2.density_cm3 + 2_695_900.0).abs() < 1e-6);
        assert!((r2.tn_k - 261.7).abs() < 1e-9);

        let r4 = &recs[3];
        assert_eq!(r4.mode, 5);
        assert_eq!(r4.slot, 4);
        assert_eq!(r4.v_geo_cor_ms, 9999.0);
    }

    #[test]
    fn wats_yyddd_maps_to_unix_milliseconds() {
        let recs = parse_wats_2s(WATS);
        assert_eq!(recs[0].unix_ms(), Some(366_977_215_461));
    }

    #[test]
    fn idm_multiline_records_carry_six_pairs_per_line() {
        let recs = parse_idm_250ms(IDM);
        assert_eq!(recs.len(), 3);
        let r = &recs[0];
        assert_eq!(r.date_yyddd, 81227);
        assert_eq!(r.ut_ms, 13_486_369);
        assert!((r.latitude_deg - 52.26).abs() < 1e-9);
        assert!((r.longitude_deg - 141.12).abs() < 1e-9);
        assert!((r.invariant_latitude_deg - 47.26).abs() < 1e-9);
        assert!((r.mlt_h - 12.73).abs() < 1e-9);
        assert!((r.altitude_km - 780.76).abs() < 1e-9);
        assert!((r.sc_velocity_ms - 7404.52).abs() < 1e-9);
        assert_eq!(r.samples.len(), 32);
        assert_eq!(r.samples[0].ut_ms, 13_486_444);
        assert_eq!(r.samples[0].velocity_ms, -68.0);
        assert_eq!(r.samples[0].quality, 0);
        assert_eq!(r.samples[0].axis, IdmAxis::HorizontalZ);
        assert_eq!(r.samples[5].velocity_ms, -411.0);
        assert_eq!(r.samples.last().unwrap().ut_ms, 13_494_069);

        let r2 = &recs[1];
        assert_eq!(r2.samples.len(), 1);
        assert_eq!(r2.samples[0].velocity_ms, -17.0);
        assert_eq!(r2.samples[0].axis, IdmAxis::VerticalY);
        assert_eq!(r2.samples[0].quality, 0);

        let r3 = &recs[2];
        assert_eq!(r3.samples.len(), 32);
        assert_eq!(r3.samples[1].velocity_ms, -50.0);
        assert_eq!(r3.samples[1].quality, 0);
        assert_eq!(r3.samples[1].axis, IdmAxis::HorizontalZ);
    }

    #[test]
    fn idm_velocity_flags_are_not_decimals() {
        let (v, q, axis) = parse_idm_velocity("-414.00").unwrap();
        assert_eq!((v, q, axis), (-414.0, 0, IdmAxis::HorizontalZ));
        let (v, q, axis) = parse_idm_velocity(" 123.41").unwrap();
        assert_eq!((v, q, axis), (123.0, 4, IdmAxis::VerticalY));
        assert!(parse_idm_velocity("9999999.0").is_none());
    }
}
