use crate::geo::{
    GeoRec, COMP_DCDB_DEPTH, COMP_GHCN_PRCP, COMP_GHCN_SNOW, COMP_GHCN_SNWD, COMP_GHCN_TMAX,
    COMP_GHCN_TMIN, COMP_GSOD_DEWP, COMP_GSOD_GUST, COMP_GSOD_PRCP, COMP_GSOD_SLP, COMP_GSOD_TEMP,
    COMP_GSOD_TMAX, COMP_GSOD_TMIN, COMP_GSOD_WDSP, COMP_ISD_DEWP, COMP_ISD_SLP, COMP_ISD_TEMP,
    COMP_ISD_WDIR, COMP_ISD_WSPD,
};
use crate::lsk::LeapSeconds;
use std::collections::HashMap;

const DAY_S: f64 = 86400.0;
const HOUR_S: f64 = 3600.0;

pub fn csv_fields(line: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut cur = String::new();
    let mut in_quotes = false;
    let mut chars = line.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '"' => {
                if in_quotes && chars.peek() == Some(&'"') {
                    cur.push('"');
                    chars.next();
                } else {
                    in_quotes = !in_quotes;
                }
            }
            ',' if !in_quotes => {
                fields.push(std::mem::take(&mut cur));
            }
            _ => cur.push(c),
        }
    }
    fields.push(cur);
    fields
}

fn num(field: &str) -> Option<f64> {
    field.trim().trim_matches('"').parse::<f64>().ok()
}

fn unix_of_civil(y: i64, m: i64, d: i64, secs: f64) -> Option<f64> {
    Some(crate::lsk::days_from_civil(y, m, d)? as f64 * DAY_S + secs)
}

fn tdb_of(unix: f64, lsk: &LeapSeconds) -> Option<f64> {
    lsk.unix_to_tdb(unix)
}

pub fn tdb_window(year: i64, month: Option<i64>, lsk: &LeapSeconds) -> Option<(f64, f64)> {
    let start_m = match month {
        Some(m) => m,
        None => 1,
    };
    let (end_y, end_m) = match month {
        Some(12) => (year + 1, 1),
        Some(m) => (year, m + 1),
        None => (year + 1, 1),
    };
    let start = unix_of_civil(year, start_m, 1, 0.0)?;
    let end = unix_of_civil(end_y, end_m, 1, 0.0)?;
    Some((lsk.unix_to_tdb(start)?, lsk.unix_to_tdb(end)?))
}

pub fn filter_window(recs: Vec<GeoRec>, start: f64, end: f64) -> Vec<GeoRec> {
    recs.into_iter()
        .filter(|r| r.t >= start && r.t < end)
        .collect()
}

fn rec(
    tdb: f64,
    lat: f64,
    lon: f64,
    alt: f64,
    bin_width: f64,
    val: f64,
    comp: u32,
) -> Option<GeoRec> {
    if !tdb.is_finite()
        || !lat.is_finite()
        || !lon.is_finite()
        || !alt.is_finite()
        || !val.is_finite()
    {
        return None;
    }
    Some(GeoRec {
        t: tdb,
        lat,
        lon,
        alt,
        freq: 0.0,
        bin_width,
        val,
        comp,
        station: 0,
    })
}

pub fn parse_ghcn_stations(text: &str) -> HashMap<String, (f64, f64, f64)> {
    let mut out = HashMap::new();
    for line in text.lines() {
        if line.len() < 37 {
            continue;
        }
        let id = line[..11].trim();
        let lat = line[12..20].trim().parse::<f64>();
        let lon = line[21..29].trim().parse::<f64>();
        let elev = line[31..37].trim().parse::<f64>();
        let (Ok(lat), Ok(lon), Ok(elev)) = (lat, lon, elev) else {
            continue;
        };
        if id.is_empty() || !lat.is_finite() || !lon.is_finite() || !elev.is_finite() {
            continue;
        }
        out.insert(id.to_string(), (lat, lon, elev));
    }
    out
}

pub fn parse_ghcn(text: &str, anchor: (f64, f64, f64), lsk: &LeapSeconds) -> Vec<GeoRec> {
    let (lat, lon, elev) = anchor;
    let mut out = Vec::new();
    for line in text.lines().skip(1) {
        let f: Vec<&str> = line.split(',').collect();
        if f.len() < 4 {
            continue;
        }
        let date = f[1].trim();
        if date.len() != 8 {
            continue;
        }
        let (Ok(y), Ok(m), Ok(d)) = (
            date[0..4].parse::<i64>(),
            date[4..6].parse::<i64>(),
            date[6..8].parse::<i64>(),
        ) else {
            continue;
        };
        let Some(unix) = unix_of_civil(y, m, d, 0.0) else {
            continue;
        };
        let Some(tdb) = tdb_of(unix, lsk) else {
            continue;
        };
        let Some(raw) = num(f[3]) else {
            continue;
        };
        if (raw + 9999.0).abs() < 0.5 {
            continue;
        }
        let val = raw / 10.0;
        let comp = match f[2].trim() {
            "TMAX" => COMP_GHCN_TMAX,
            "TMIN" => COMP_GHCN_TMIN,
            "PRCP" if val >= 0.0 => COMP_GHCN_PRCP,
            "SNOW" if val >= 0.0 => COMP_GHCN_SNOW,
            "SNWD" if val >= 0.0 => COMP_GHCN_SNWD,
            _ => continue,
        };
        if let Some(r) = rec(tdb, lat, lon, elev, DAY_S, val, comp) {
            out.push(r);
        }
    }
    out
}

fn f_to_c(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn knots_to_ms(k: f64) -> f64 {
    k * 0.514_444
}

fn inch_to_mm(v: f64) -> f64 {
    v * 25.4
}

fn ymd_of(date: &str) -> Option<(i64, i64, i64)> {
    let mut p = date.trim().trim_matches('"').split('-');
    let y = p.next()?.parse().ok()?;
    let m = p.next()?.parse().ok()?;
    let d = p.next()?.parse().ok()?;
    Some((y, m, d))
}

pub fn parse_gsod(text: &str, lsk: &LeapSeconds) -> Vec<GeoRec> {
    let mut lines = text.lines();
    let Some(header) = lines.next() else {
        return Vec::new();
    };
    let h = csv_fields(header);
    let idx = |name: &str| h.iter().position(|c| c.trim_matches('"') == name);
    let (Some(i_date), Some(i_lat), Some(i_lon), Some(i_elev)) = (
        idx("DATE"),
        idx("LATITUDE"),
        idx("LONGITUDE"),
        idx("ELEVATION"),
    ) else {
        return Vec::new();
    };
    let i_temp = idx("TEMP");
    let i_dewp = idx("DEWP");
    let i_slp = idx("SLP");
    let i_wdsp = idx("WDSP");
    let i_gust = idx("GUST");
    let i_max = idx("MAX");
    let i_min = idx("MIN");
    let i_prcp = idx("PRCP");
    let mut out = Vec::new();
    for line in lines {
        let f = csv_fields(line);
        let Some(date) = f.get(i_date).map(|s| s.trim_matches('"')) else {
            continue;
        };
        let (Some(lat), Some(lon), Some(elev)) = (
            f.get(i_lat).and_then(|s| num(s)),
            f.get(i_lon).and_then(|s| num(s)),
            f.get(i_elev).and_then(|s| num(s)),
        ) else {
            continue;
        };
        let Some((y, m, d)) = ymd_of(date) else {
            continue;
        };
        let Some(unix) = unix_of_civil(y, m, d, 0.0) else {
            continue;
        };
        let Some(tdb) = tdb_of(unix, lsk) else {
            continue;
        };
        let mut push = |i: Option<usize>,
                        sentinel: f64,
                        convert: fn(f64) -> f64,
                        comp: u32,
                        nonneg: bool|
         -> Option<()> {
            let i = i?;
            let raw = f.get(i).and_then(|s| num(s))?;
            if raw >= sentinel {
                return None;
            }
            let val = convert(raw);
            if nonneg && val < 0.0 {
                return None;
            }
            if let Some(r) = rec(tdb, lat, lon, elev, DAY_S, val, comp) {
                out.push(r);
            }
            Some(())
        };
        push(i_temp, 9999.0, f_to_c, COMP_GSOD_TEMP, false);
        push(i_dewp, 9999.0, f_to_c, COMP_GSOD_DEWP, false);
        push(i_slp, 9999.0, |v| v, COMP_GSOD_SLP, true);
        push(i_wdsp, 999.0, knots_to_ms, COMP_GSOD_WDSP, true);
        push(i_gust, 999.0, knots_to_ms, COMP_GSOD_GUST, true);
        push(i_max, 9999.0, f_to_c, COMP_GSOD_TMAX, false);
        push(i_min, 9999.0, f_to_c, COMP_GSOD_TMIN, false);
        push(i_prcp, 99.9, inch_to_mm, COMP_GSOD_PRCP, true);
    }
    out
}

fn coded(i: usize, f: &[String], bound: i64) -> Option<f64> {
    let s = f.get(i)?;
    let first = s.split(',').next()?;
    let code: i64 = first.trim().parse().ok()?;
    if code.abs() >= bound {
        return None;
    }
    Some(code as f64 / 10.0)
}

pub fn parse_isd(text: &str, lsk: &LeapSeconds) -> Vec<GeoRec> {
    let mut lines = text.lines();
    let Some(header) = lines.next() else {
        return Vec::new();
    };
    let h = csv_fields(header);
    let idx = |name: &str| h.iter().position(|c| c.trim_matches('"') == name);
    let (
        Some(i_date),
        Some(i_lat),
        Some(i_lon),
        Some(i_elev),
        Some(i_wnd),
        Some(i_tmp),
        Some(i_dew),
        Some(i_slp),
    ) = (
        idx("DATE"),
        idx("LATITUDE"),
        idx("LONGITUDE"),
        idx("ELEVATION"),
        idx("WND"),
        idx("TMP"),
        idx("DEW"),
        idx("SLP"),
    )
    else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for line in lines {
        let f = csv_fields(line);
        let Some(date) = f.get(i_date).map(|s| s.trim_matches('"')) else {
            continue;
        };
        let (Some(lat), Some(lon), Some(elev)) = (
            f.get(i_lat).and_then(|s| num(s)),
            f.get(i_lon).and_then(|s| num(s)),
            f.get(i_elev).and_then(|s| num(s)),
        ) else {
            continue;
        };
        let Some((dstr, tstr)) = date.split_once('T') else {
            continue;
        };
        let Some((y, m, d)) = ymd_of(dstr) else {
            continue;
        };
        let mut tp = tstr.split(':');
        let Some(hh) = tp.next().and_then(|v| v.parse::<f64>().ok()) else {
            continue;
        };
        let mm: f64 = match tp.next().and_then(|v| v.parse().ok()) {
            Some(v) => v,
            None => continue,
        };
        let ss: f64 = match tp.next().and_then(|v| v.parse().ok()) {
            Some(v) => v,
            None => continue,
        };
        let Some(unix) = unix_of_civil(y, m, d, hh * 3600.0 + mm * 60.0 + ss) else {
            continue;
        };
        let Some(tdb) = tdb_of(unix, lsk) else {
            continue;
        };
        if let Some(val) = coded(i_tmp, &f, 9000) {
            if let Some(r) = rec(tdb, lat, lon, elev, HOUR_S, val, COMP_ISD_TEMP) {
                out.push(r);
            }
        }
        if let Some(val) = coded(i_dew, &f, 9000) {
            if let Some(r) = rec(tdb, lat, lon, elev, HOUR_S, val, COMP_ISD_DEWP) {
                out.push(r);
            }
        }
        if let Some(val) = coded(i_slp, &f, 90000) {
            if val > 0.0 {
                if let Some(r) = rec(tdb, lat, lon, elev, HOUR_S, val, COMP_ISD_SLP) {
                    out.push(r);
                }
            }
        }
        if let Some(wnd) = f.get(i_wnd) {
            let parts: Vec<&str> = wnd.split(',').collect();
            let dir_s = match parts.first() {
                Some(s) => s.trim(),
                None => "",
            };
            let Ok(dir) = dir_s.parse::<i64>() else {
                continue;
            };
            if !(0..=360).contains(&dir) {
                continue;
            }
            if let Some(r) = rec(tdb, lat, lon, elev, HOUR_S, dir as f64, COMP_ISD_WDIR) {
                out.push(r);
            }
            if let Some(sp) = parts.get(3) {
                if let Ok(code) = sp.trim().parse::<i64>() {
                    if code.abs() < 9000 {
                        let val = code as f64 / 10.0;
                        if val >= 0.0 {
                            if let Some(r) = rec(tdb, lat, lon, elev, HOUR_S, val, COMP_ISD_WSPD) {
                                out.push(r);
                            }
                        }
                    }
                }
            }
        }
    }
    out
}

fn unix_of_iso(date: &str) -> Option<f64> {
    let s = date.trim().trim_matches('"');
    let s = s.strip_suffix('Z').unwrap_or(s);
    let (dstr, tstr) = s.split_once('T')?;
    let (y, m, d) = ymd_of(dstr)?;
    let mut tp = tstr.split(':');
    let hh: f64 = tp.next()?.parse().ok()?;
    let mm: f64 = tp.next()?.parse().ok()?;
    let ss: f64 = match tp.next() {
        Some(v) => v.parse().ok()?,
        None => 0.0,
    };
    unix_of_civil(y, m, d, hh * 3600.0 + mm * 60.0 + ss)
}

pub fn parse_dcdb(text: &str, lsk: &LeapSeconds) -> Vec<GeoRec> {
    let mut lines = text.lines();
    let Some(header) = lines.next() else {
        return Vec::new();
    };
    let h = csv_fields(header);
    let idx = |name: &str| h.iter().position(|c| c.trim_matches('"') == name);
    let (Some(i_lon), Some(i_lat), Some(i_depth), Some(i_time)) =
        (idx("LON"), idx("LAT"), idx("DEPTH"), idx("TIME"))
    else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for line in lines {
        if line.trim().is_empty() {
            continue;
        }
        let f = csv_fields(line);
        let (Some(lat), Some(lon), Some(depth)) = (
            f.get(i_lat).and_then(|s| num(s)),
            f.get(i_lon).and_then(|s| num(s)),
            f.get(i_depth).and_then(|s| num(s)),
        ) else {
            continue;
        };
        if !(depth.is_finite() && depth >= 0.0) {
            continue;
        }
        let Some(date) = f.get(i_time) else {
            continue;
        };
        let Some(unix) = unix_of_iso(date) else {
            continue;
        };
        let Some(tdb) = tdb_of(unix, lsk) else {
            continue;
        };
        if let Some(r) = rec(tdb, lat, lon, -depth, 0.0, depth, COMP_DCDB_DEPTH) {
            out.push(r);
        }
    }
    out
}
