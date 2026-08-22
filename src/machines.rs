//! Die Ernte-Maschinen: Solar + ENSO. Jede Maschine erntet ihre Serien
//! über die vorhandenen Archivar-Fetch-Pfade und sendet Zellen über einen
//! mpsc-Kanal an die Mathematikerin — ein Kanal je Maschine, kein
//! Sonderpfad im Kern.
use std::collections::{HashMap, HashSet};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

use crate::archivar::{
    extract_series, fetch_raw, fetch_raw_bytes, Extract, LeapSeconds, SourceConfig,
};
pub const SOLAR_FAST_GRID: u32 = 60;
pub const SOLAR_COARSE_GRID: u32 = 43200;
const SOLAR_GOES_SYNC_S: f64 = 149_597_870_700.0 / 299_792_458.0;
const SOLAR_L1_SUN_M: f64 = 1.481e11;
const SOLAR_RING_MAX: usize = 256;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum SolarChannel {
    Xray,
    Euv304,
    Euv284,
    BzGsm,
    Density,
    F107,
}

impl SolarChannel {
    pub fn name(self) -> &'static str {
        match self {
            SolarChannel::Xray => "xray",
            SolarChannel::Euv304 => "euv304",
            SolarChannel::Euv284 => "euv284",
            SolarChannel::BzGsm => "bz",
            SolarChannel::Density => "density",
            SolarChannel::F107 => "f107",
        }
    }

    pub fn idx(self) -> usize {
        match self {
            SolarChannel::Xray => 0,
            SolarChannel::Euv304 => 1,
            SolarChannel::Euv284 => 2,
            SolarChannel::BzGsm => 3,
            SolarChannel::Density => 4,
            SolarChannel::F107 => 5,
        }
    }
}

#[derive(Clone, Copy)]
pub struct SolarCell {
    pub grid: u32,
    pub channel: SolarChannel,
    pub bin: u64,
    pub value: f32,
}

pub const ENSO_GRID: u32 = 21600;
const ENSO_RING_MAX: usize = 1024;
const ENSO_FETCH_TTL: u64 = 3600;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum EnsoStation {
    St51000,
    St51001,
    St41001,
    St41002,
    St41043,
    St41010,
    St41049,
    St42001,
    St42002,
    St42036,
    St42055,
    St44009,
    St44013,
    St45001,
    St45161,
    St45178,
    St45186,
    St45207,
    St46001,
    St46002,
    St46005,
    St46012,
    St46022,
    St46025,
    St46026,
    St46029,
    St46035,
    St46047,
    St46053,
    St46054,
    St46059,
    St46069,
    St46071,
    St46075,
    St46086,
    St51004,
    St15006,
}

impl EnsoStation {
    pub const ALL: [EnsoStation; 37] = [
        EnsoStation::St51000,
        EnsoStation::St51001,
        EnsoStation::St41001,
        EnsoStation::St41002,
        EnsoStation::St41043,
        EnsoStation::St41010,
        EnsoStation::St41049,
        EnsoStation::St42001,
        EnsoStation::St42002,
        EnsoStation::St42036,
        EnsoStation::St42055,
        EnsoStation::St44009,
        EnsoStation::St44013,
        EnsoStation::St45001,
        EnsoStation::St45161,
        EnsoStation::St45178,
        EnsoStation::St45186,
        EnsoStation::St45207,
        EnsoStation::St46001,
        EnsoStation::St46002,
        EnsoStation::St46005,
        EnsoStation::St46012,
        EnsoStation::St46022,
        EnsoStation::St46025,
        EnsoStation::St46026,
        EnsoStation::St46029,
        EnsoStation::St46035,
        EnsoStation::St46047,
        EnsoStation::St46053,
        EnsoStation::St46054,
        EnsoStation::St46059,
        EnsoStation::St46069,
        EnsoStation::St46071,
        EnsoStation::St46075,
        EnsoStation::St46086,
        EnsoStation::St51004,
        EnsoStation::St15006,
    ];

    pub fn id(self) -> &'static str {
        match self {
            EnsoStation::St51000 => "51000",
            EnsoStation::St51001 => "51001",
            EnsoStation::St41001 => "41001",
            EnsoStation::St41002 => "41002",
            EnsoStation::St41043 => "41043",
            EnsoStation::St41010 => "41010",
            EnsoStation::St41049 => "41049",
            EnsoStation::St42001 => "42001",
            EnsoStation::St42002 => "42002",
            EnsoStation::St42036 => "42036",
            EnsoStation::St42055 => "42055",
            EnsoStation::St44009 => "44009",
            EnsoStation::St44013 => "44013",
            EnsoStation::St45001 => "45001",
            EnsoStation::St45161 => "45161",
            EnsoStation::St45178 => "45178",
            EnsoStation::St45186 => "45186",
            EnsoStation::St45207 => "45207",
            EnsoStation::St46001 => "46001",
            EnsoStation::St46002 => "46002",
            EnsoStation::St46005 => "46005",
            EnsoStation::St46012 => "46012",
            EnsoStation::St46022 => "46022",
            EnsoStation::St46025 => "46025",
            EnsoStation::St46026 => "46026",
            EnsoStation::St46029 => "46029",
            EnsoStation::St46035 => "46035",
            EnsoStation::St46047 => "46047",
            EnsoStation::St46053 => "46053",
            EnsoStation::St46054 => "46054",
            EnsoStation::St46059 => "46059",
            EnsoStation::St46069 => "46069",
            EnsoStation::St46071 => "46071",
            EnsoStation::St46075 => "46075",
            EnsoStation::St46086 => "46086",
            EnsoStation::St51004 => "51004",
            EnsoStation::St15006 => "15006",
        }
    }

    pub fn idx(self) -> usize {
        match self {
            EnsoStation::St51000 => 0,
            EnsoStation::St51001 => 1,
            EnsoStation::St41001 => 2,
            EnsoStation::St41002 => 3,
            EnsoStation::St41043 => 4,
            EnsoStation::St41010 => 5,
            EnsoStation::St41049 => 6,
            EnsoStation::St42001 => 7,
            EnsoStation::St42002 => 8,
            EnsoStation::St42036 => 9,
            EnsoStation::St42055 => 10,
            EnsoStation::St44009 => 11,
            EnsoStation::St44013 => 12,
            EnsoStation::St45001 => 13,
            EnsoStation::St45161 => 14,
            EnsoStation::St45178 => 15,
            EnsoStation::St45186 => 16,
            EnsoStation::St45207 => 17,
            EnsoStation::St46001 => 18,
            EnsoStation::St46002 => 19,
            EnsoStation::St46005 => 20,
            EnsoStation::St46012 => 21,
            EnsoStation::St46022 => 22,
            EnsoStation::St46025 => 23,
            EnsoStation::St46026 => 24,
            EnsoStation::St46029 => 25,
            EnsoStation::St46035 => 26,
            EnsoStation::St46047 => 27,
            EnsoStation::St46053 => 28,
            EnsoStation::St46054 => 29,
            EnsoStation::St46059 => 30,
            EnsoStation::St46069 => 31,
            EnsoStation::St46071 => 32,
            EnsoStation::St46075 => 33,
            EnsoStation::St46086 => 34,
            EnsoStation::St51004 => 35,
            EnsoStation::St15006 => 36,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum EnsoSeries {
    Wspd,
    Gst,
    Wvht,
    Dpd,
    Apd,
    Pres,
    Ptdy,
    Atmp,
    Wtmp,
    Dewp,
    Vis,
    Tide,
    WdirSin,
    WdirCos,
    MwdSin,
    MwdCos,
    Rain,
}

impl EnsoSeries {
    pub const ALL: [EnsoSeries; 17] = [
        EnsoSeries::Wspd,
        EnsoSeries::Gst,
        EnsoSeries::Wvht,
        EnsoSeries::Dpd,
        EnsoSeries::Apd,
        EnsoSeries::Pres,
        EnsoSeries::Ptdy,
        EnsoSeries::Atmp,
        EnsoSeries::Wtmp,
        EnsoSeries::Dewp,
        EnsoSeries::Vis,
        EnsoSeries::Tide,
        EnsoSeries::WdirSin,
        EnsoSeries::WdirCos,
        EnsoSeries::MwdSin,
        EnsoSeries::MwdCos,
        EnsoSeries::Rain,
    ];

    pub fn idx(self) -> usize {
        match self {
            EnsoSeries::Wspd => 0,
            EnsoSeries::Gst => 1,
            EnsoSeries::Wvht => 2,
            EnsoSeries::Dpd => 3,
            EnsoSeries::Apd => 4,
            EnsoSeries::Pres => 5,
            EnsoSeries::Ptdy => 6,
            EnsoSeries::Atmp => 7,
            EnsoSeries::Wtmp => 8,
            EnsoSeries::Dewp => 9,
            EnsoSeries::Vis => 10,
            EnsoSeries::Tide => 11,
            EnsoSeries::WdirSin => 12,
            EnsoSeries::WdirCos => 13,
            EnsoSeries::MwdSin => 14,
            EnsoSeries::MwdCos => 15,
            EnsoSeries::Rain => 16,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            EnsoSeries::Wspd => "wspd",
            EnsoSeries::Gst => "gst",
            EnsoSeries::Wvht => "wvht",
            EnsoSeries::Dpd => "dpd",
            EnsoSeries::Apd => "apd",
            EnsoSeries::Pres => "pres",
            EnsoSeries::Ptdy => "ptdy",
            EnsoSeries::Atmp => "atmp",
            EnsoSeries::Wtmp => "wtmp",
            EnsoSeries::Dewp => "dewp",
            EnsoSeries::Vis => "vis",
            EnsoSeries::Tide => "tide",
            EnsoSeries::WdirSin => "wdir_sin",
            EnsoSeries::WdirCos => "wdir_cos",
            EnsoSeries::MwdSin => "mwd_sin",
            EnsoSeries::MwdCos => "mwd_cos",
            EnsoSeries::Rain => "rain",
        }
    }

    pub fn column(self) -> &'static str {
        match self {
            EnsoSeries::Wspd => "WSPD",
            EnsoSeries::Gst => "GST",
            EnsoSeries::Wvht => "WVHT",
            EnsoSeries::Dpd => "DPD",
            EnsoSeries::Apd => "APD",
            EnsoSeries::Pres => "PRES",
            EnsoSeries::Ptdy => "PTDY",
            EnsoSeries::Atmp => "ATMP",
            EnsoSeries::Wtmp => "WTMP",
            EnsoSeries::Dewp => "DEWP",
            EnsoSeries::Vis => "VIS",
            EnsoSeries::Tide => "TIDE",
            EnsoSeries::WdirSin | EnsoSeries::WdirCos => "WDIR",
            EnsoSeries::MwdSin | EnsoSeries::MwdCos => "MWD",
            EnsoSeries::Rain => "RAIN",
        }
    }

    pub fn plausible(self, v: f64) -> bool {
        if !v.is_finite() {
            return false;
        }
        match self {
            EnsoSeries::Wspd
            | EnsoSeries::Gst
            | EnsoSeries::Wvht
            | EnsoSeries::Dpd
            | EnsoSeries::Apd
            | EnsoSeries::Vis
            | EnsoSeries::Rain => v > 0.0 && v < 99.0,
            EnsoSeries::Pres => v > 0.0 && v < 9999.0,
            EnsoSeries::Ptdy | EnsoSeries::Tide => v.abs() < 99.0,
            EnsoSeries::Atmp | EnsoSeries::Wtmp | EnsoSeries::Dewp => v > -100.0 && v < 900.0,
            EnsoSeries::WdirSin | EnsoSeries::WdirCos | EnsoSeries::MwdSin | EnsoSeries::MwdCos => {
                v >= 0.0 && v <= 360.0
            }
        }
    }

    pub fn transform(self, raw: f64) -> Option<f64> {
        if !self.plausible(raw) {
            return None;
        }
        match self {
            EnsoSeries::WdirSin | EnsoSeries::MwdSin => Some(raw.to_radians().sin()),
            EnsoSeries::WdirCos | EnsoSeries::MwdCos => Some(raw.to_radians().cos()),
            _ => Some(raw),
        }
    }
}

#[derive(Clone, Copy)]
pub struct EnsoCell {
    pub station: EnsoStation,
    pub series: EnsoSeries,
    pub bin: u64,
    pub value: f32,
}

fn solar_find_block<'a>(sources: &'a [SourceConfig], field_name: &str) -> Option<&'a SourceConfig> {
    sources.iter().find(|s| {
        s.extracts.iter().any(|e| match e {
            Extract::Field(fc)
            | Extract::First(fc, _)
            | Extract::Last(fc, _)
            | Extract::Path(fc) => fc.name == field_name,
            _ => false,
        })
    })
}

fn solar_series(
    block: &SourceConfig,
    body: &str,
    field_name: &str,
    lsk: &LeapSeconds,
) -> Vec<(f64, f64)> {
    let mut series_src = block.clone();
    series_src.extracts.retain(|e| {
        matches!(
            e,
            Extract::First(fc, _) | Extract::Last(fc, _) | Extract::Path(fc)
                if fc.name == field_name
        )
    });
    extract_series(&series_src, body, lsk)
}

fn solar_l1_sync(series: &[(f64, f64)], wind: &[(f64, f64)], tolerance_s: f64) -> Vec<(f64, f64)> {
    let mut out = Vec::new();
    for &(t, v) in series {
        let mut best: Option<(f64, f64)> = None;
        for &(tw, vw) in wind {
            let dt = (tw - t).abs();
            if dt <= tolerance_s && best.map_or(true, |(b, _)| dt < b) {
                best = Some((dt, vw));
            }
        }
        let Some((_, v_ms)) = best else {
            continue;
        };
        if v_ms <= 0.0 {
            continue;
        }
        out.push((t - SOLAR_L1_SUN_M / v_ms, v));
    }
    out.sort_by(|a, b| a.0.total_cmp(&b.0));
    out
}

fn solar_send_bins(
    series: &[(f64, f64)],
    grid: u32,
    channel: SolarChannel,
    last_sent: &mut std::collections::HashMap<(u32, SolarChannel), u64>,
    tx: &mpsc::Sender<SolarCell>,
) {
    let dt = grid as f64;
    let mut sorted: Vec<&(f64, f64)> = series
        .iter()
        .filter(|&&(t, _)| t.is_finite() && t > 0.0)
        .collect();
    sorted.sort_by(|a, b| a.0.total_cmp(&b.0));
    let gate = last_sent.get(&(grid, channel)).copied().unwrap_or(0);
    let mut cells: Vec<SolarCell> = Vec::new();
    let mut cur_bin: i64 = i64::MIN;
    let mut sum: f64 = 0.0;
    let mut cnt: u32 = 0;
    for &&(t, v) in &sorted {
        let bin = (t / dt).floor() as i64;
        if bin != cur_bin {
            if cur_bin != i64::MIN && cnt > 0 {
                let b = cur_bin as u64;
                if b > gate {
                    cells.push(SolarCell {
                        grid,
                        channel,
                        bin: b,
                        value: (sum / cnt as f64) as f32,
                    });
                }
            }
            cur_bin = bin;
            sum = 0.0;
            cnt = 0;
        }
        sum += v;
        cnt += 1;
    }
    if cur_bin != i64::MIN && cnt > 0 {
        let b = cur_bin as u64;
        if b > gate {
            cells.push(SolarCell {
                grid,
                channel,
                bin: b,
                value: (sum / cnt as f64) as f32,
            });
        }
    }
    if gate == 0 && cells.len() > SOLAR_RING_MAX {
        let drop = cells.len() - SOLAR_RING_MAX;
        cells.drain(..drop);
    }
    for cell in &cells {
        let _ = tx.send(*cell);
    }
    if let Some(last) = cells.last() {
        last_sent.insert((grid, channel), last.bin);
    }
}

pub fn solar_harvest(
    tx: mpsc::Sender<SolarCell>,
    sources: Vec<SourceConfig>,
    time: Arc<Mutex<Option<LeapSeconds>>>,
) {
    let mut last_sent: std::collections::HashMap<(u32, SolarChannel), u64> =
        std::collections::HashMap::new();
    loop {
        let lock = match time.lock() {
            Ok(l) => l,
            Err(_) => break,
        };
        let Some(lsk) = lock.as_ref() else {
            drop(lock);
            thread::sleep(std::time::Duration::from_secs(SOLAR_FAST_GRID as u64));
            continue;
        };
        let goes_shift = |series: Vec<(f64, f64)>| -> Vec<(f64, f64)> {
            series
                .into_iter()
                .map(|(t, v)| (t - SOLAR_GOES_SYNC_S, v))
                .collect()
        };
        if let Some(block) = solar_find_block(&sources, "noaa_goes_xray_flux_w_m2") {
            if let Some(body) = fetch_raw(&block.url, None, &block.headers, block.ttl) {
                let raw = solar_series(block, &body, "noaa_goes_xray_flux_w_m2", lsk);
                let shifted = goes_shift(raw);
                solar_send_bins(
                    &shifted,
                    SOLAR_FAST_GRID,
                    SolarChannel::Xray,
                    &mut last_sent,
                    &tx,
                );
                solar_send_bins(
                    &shifted,
                    SOLAR_COARSE_GRID,
                    SolarChannel::Xray,
                    &mut last_sent,
                    &tx,
                );
            }
        }
        if let Some(block) = solar_find_block(&sources, "solar_euv_flux_304_wm2") {
            if let Some(body) = fetch_raw(&block.url, None, &block.headers, block.ttl) {
                for (field, channel) in [
                    ("solar_euv_flux_304_wm2", SolarChannel::Euv304),
                    ("solar_euv_flux_284_wm2", SolarChannel::Euv284),
                ] {
                    let raw = solar_series(block, &body, field, lsk);
                    let shifted = goes_shift(raw);
                    solar_send_bins(&shifted, SOLAR_FAST_GRID, channel, &mut last_sent, &tx);
                    solar_send_bins(&shifted, SOLAR_COARSE_GRID, channel, &mut last_sent, &tx);
                }
            }
        }
        if let Some(block) = solar_find_block(&sources, "solar_f107_flux_sfu") {
            if let Some(body) = fetch_raw(&block.url, None, &block.headers, block.ttl) {
                let raw = solar_series(block, &body, "solar_f107_flux_sfu", lsk);
                let shifted = goes_shift(raw);
                solar_send_bins(
                    &shifted,
                    SOLAR_COARSE_GRID,
                    SolarChannel::F107,
                    &mut last_sent,
                    &tx,
                );
            }
        }
        let mut wind: Vec<(f64, f64)> = Vec::new();
        if let Some(block) = solar_find_block(&sources, "solar_wind_speed_km_s") {
            if let Some(body) = fetch_raw(&block.url, None, &block.headers, block.ttl) {
                wind = solar_series(block, &body, "solar_wind_speed_km_s", lsk);
                let dens = solar_series(block, &body, "solar_wind_density_cm3", lsk);
                let synced = solar_l1_sync(&dens, &wind, 60.0);
                solar_send_bins(
                    &synced,
                    SOLAR_FAST_GRID,
                    SolarChannel::Density,
                    &mut last_sent,
                    &tx,
                );
            }
        }
        if let Some(block) = solar_find_block(&sources, "magnetosphere_imf_bz_nt") {
            if let Some(body) = fetch_raw(&block.url, None, &block.headers, block.ttl) {
                let bz = solar_series(block, &body, "magnetosphere_imf_bz_nt", lsk);
                let synced = solar_l1_sync(&bz, &wind, 60.0);
                solar_send_bins(
                    &synced,
                    SOLAR_FAST_GRID,
                    SolarChannel::BzGsm,
                    &mut last_sent,
                    &tx,
                );
            }
        }
        drop(lock);
        thread::sleep(std::time::Duration::from_secs(SOLAR_FAST_GRID as u64));
    }
}

fn enso_ndbc_parse(body: &str, lsk: &LeapSeconds) -> Option<Vec<(EnsoSeries, Vec<(f64, f64)>)>> {
    let header = body.lines().find(|l| l.starts_with('#'))?;
    let cols: Vec<&str> = header.split_whitespace().collect();
    let mut present: Vec<(EnsoSeries, usize)> = Vec::new();
    for s in EnsoSeries::ALL {
        if let Some(i) = cols.iter().position(|&c| c == s.column()) {
            if !present.iter().any(|&(cs, _)| cs == s) {
                present.push((s, i));
            }
        }
    }
    if present.is_empty() {
        return None;
    }
    let mut series: Vec<(EnsoSeries, Vec<(f64, f64)>)> =
        present.iter().map(|&(s, _)| (s, Vec::new())).collect();
    for line in body
        .lines()
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
    {
        let p: Vec<&str> = line.split_whitespace().collect();
        let needed = present.iter().map(|&(_, i)| i).max().unwrap_or(0);
        if p.len() <= needed {
            continue;
        }
        let Ok(year) = p[0].parse::<i64>() else {
            continue;
        };
        let Ok(month) = p[1].parse::<i64>() else {
            continue;
        };
        let Ok(day) = p[2].parse::<i64>() else {
            continue;
        };
        let Ok(hour) = p[3].parse::<i64>() else {
            continue;
        };
        let Ok(minute) = p[4].parse::<i64>() else {
            continue;
        };
        let days = crate::lsk::days_from_civil(year, month, day)?;
        let unix = days as f64 * 86400.0 + hour as f64 * 3600.0 + minute as f64 * 60.0;
        let Some(t) = lsk.unix_to_tdb(unix) else {
            continue;
        };
        for (s, vec) in series.iter_mut() {
            let i = present.iter().find(|&&(cs, _)| cs == *s).map(|&(_, i)| i);
            let Some(i) = i else { continue };
            if let Ok(v) = p[i].parse::<f64>() {
                if let Some(tv) = s.transform(v) {
                    vec.push((t, tv));
                }
            }
        }
    }
    series.retain(|(_, vec)| !vec.is_empty());
    if series.is_empty() {
        None
    } else {
        Some(series)
    }
}

fn enso_send_bins(
    series: &[(f64, f64)],
    station: EnsoStation,
    kind: EnsoSeries,
    last_sent: &mut HashMap<(EnsoStation, EnsoSeries), u64>,
    tx: &mpsc::Sender<EnsoCell>,
) {
    let dt = ENSO_GRID as f64;
    let mut sorted: Vec<&(f64, f64)> = series
        .iter()
        .filter(|&&(t, _)| t.is_finite() && t > 0.0)
        .collect();
    sorted.sort_by(|a, b| a.0.total_cmp(&b.0));
    let gate = last_sent.get(&(station, kind)).copied().unwrap_or(0);
    let mut cells: Vec<EnsoCell> = Vec::new();
    let mut cur_bin: i64 = i64::MIN;
    let mut sum: f64 = 0.0;
    let mut cnt: u32 = 0;
    for &&(t, v) in &sorted {
        let bin = (t / dt).floor() as i64;
        if bin != cur_bin {
            if cur_bin != i64::MIN && cnt > 0 {
                let b = cur_bin as u64;
                if b > gate {
                    cells.push(EnsoCell {
                        station,
                        series: kind,
                        bin: b,
                        value: (sum / cnt as f64) as f32,
                    });
                }
            }
            cur_bin = bin;
            sum = 0.0;
            cnt = 0;
        }
        sum += v;
        cnt += 1;
    }
    if cur_bin != i64::MIN && cnt > 0 {
        let b = cur_bin as u64;
        if b > gate {
            cells.push(EnsoCell {
                station,
                series: kind,
                bin: b,
                value: (sum / cnt as f64) as f32,
            });
        }
    }
    if gate == 0 && cells.len() > ENSO_RING_MAX {
        let drop = cells.len() - ENSO_RING_MAX;
        cells.drain(..drop);
    }
    for cell in &cells {
        let _ = tx.send(*cell);
    }
    if let Some(last) = cells.last() {
        last_sent.insert((station, kind), last.bin);
    }
}

pub fn enso_harvest(tx: mpsc::Sender<EnsoCell>, time: Arc<Mutex<Option<LeapSeconds>>>) {
    let mut last_sent: HashMap<(EnsoStation, EnsoSeries), u64> = HashMap::new();
    let mut said: HashSet<String> = HashSet::new();
    let mut backfilled: HashSet<EnsoStation> = HashSet::new();
    loop {
        let lsk = {
            let lock = match time.lock() {
                Ok(l) => l,
                Err(_) => break,
            };
            let Some(lsk) = lock.as_ref() else {
                drop(lock);
                thread::sleep(std::time::Duration::from_secs(ENSO_FETCH_TTL));
                continue;
            };
            lsk.clone()
        };
        for station in EnsoStation::ALL {
            if backfilled.insert(station) {
                enso_backfill(station, &lsk, &mut last_sent, &tx);
            }
            let url = format!(
                "https://www.ndbc.noaa.gov/data/realtime2/{}.txt",
                station.id()
            );
            if let Some(body) = fetch_raw(&url, None, &[], ENSO_FETCH_TTL) {
                match enso_ndbc_parse(&body, &lsk) {
                    Some(series) => {
                        for (kind, values) in &series {
                            enso_send_bins(values, station, *kind, &mut last_sent, &tx);
                        }
                    }
                    None => {
                        if said.insert(format!("{} column absent", station.id())) {
                            eprintln!("enso pair {} column absent", station.id());
                        }
                    }
                }
            }
        }
        thread::sleep(std::time::Duration::from_secs(ENSO_FETCH_TTL));
    }
}

fn enso_backfill(
    station: EnsoStation,
    lsk: &LeapSeconds,
    last_sent: &mut HashMap<(EnsoStation, EnsoSeries), u64>,
    tx: &mpsc::Sender<EnsoCell>,
) {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let year = 1970 + (now / 31556952) as i64 - 1;
    if year < 1970 {
        return;
    }
    let url = format!(
        "https://www.ndbc.noaa.gov/view_text_file.php?filename={}h{}.txt.gz&dir=data/historical/stdmet/",
        station.id(),
        year
    );
    let cache = format!("/tmp/omegaflow_enso_cache/{}_{}.txt", station.id(), year);
    let body = match std::fs::read_to_string(&cache) {
        Ok(cached) => Some(cached),
        Err(_) => enso_cache_fetch(&url, &cache),
    };
    if let Some(body) = body {
        if let Some(series) = enso_ndbc_parse(&body, lsk) {
            for (kind, values) in &series {
                enso_send_bins(values, station, *kind, last_sent, tx);
            }
        }
    }
}

fn enso_cache_fetch(url: &str, cache: &str) -> Option<String> {
    let bytes = fetch_raw_bytes(url, 86400)?;
    let text = if bytes.starts_with(&[0x1f, 0x8b]) {
        crate::inflate::gunzip(&bytes).map(|b| String::from_utf8_lossy(&b).into_owned())?
    } else {
        String::from_utf8_lossy(&bytes).into_owned()
    };
    let _ = std::fs::create_dir_all("/tmp/omegaflow_enso_cache");
    let _ = std::fs::write(cache, &text);
    thread::sleep(std::time::Duration::from_secs(32));
    Some(text)
}

#[cfg(test)]
mod enso_parser_tests {
    use super::*;
    use crate::archivar::embedded_lsk;

    fn series_of<'a>(
        parsed: &'a [(EnsoSeries, Vec<(f64, f64)>)],
        s: EnsoSeries,
    ) -> &'a [(f64, f64)] {
        parsed
            .iter()
            .find(|(k, _)| *k == s)
            .map(|(_, v)| v.as_slice())
            .unwrap_or(&[])
    }

    const FIXTURE: &str = "#YY  MM DD hh mm WDIR WSPD GST  WVHT   DPD   APD MWD   PRES  ATMP  WTMP  DEWP  VIS PTDY  TIDE\n#yr  mo dy hr mn degT m/s  m/s     m   sec   sec degT   hPa  degC  degC  degC  nmi  hPa    ft\n2026 08 21 15 10 100  3.0  4.0    MM    MM    MM  MM 1013.5  26.0  26.7  23.1   MM   MM    MM\n2026 08 21 15 00 100   MM  4.0    MM    MM    MM  MM 1013.3  26.0  26.7  22.9   MM -0.7    MM\n2026 08 21 14 50 100  4.0  5.0   1.5     8   6.2 100 1013.2  26.1    MM  23.0   MM   MM    MM\n";

    #[test]
    fn enso_ndbc_parse_splits_wind_and_sst_with_mm_skips() {
        let lsk = embedded_lsk().expect("embedded lsk");
        let parsed = enso_ndbc_parse(FIXTURE, &lsk).expect("fixture parses");
        let wind = series_of(&parsed, EnsoSeries::Wspd);
        let sst = series_of(&parsed, EnsoSeries::Wtmp);
        assert_eq!(wind.len(), 2);
        assert_eq!(sst.len(), 2);
        assert_eq!(wind[0].1, 3.0);
        assert_eq!(wind[1].1, 4.0);
        assert_eq!(sst[0].1, 26.7);
        assert_eq!(sst[1].1, 26.7);
        assert_eq!(wind[0].0 - wind[1].0, 1200.0);
        assert_eq!(sst[0].0 - sst[1].0, 600.0);
    }

    #[test]
    fn enso_ndbc_parse_skips_stdmet_sentinel_values() {
        let lsk = embedded_lsk().expect("embedded lsk");
        let body = "#YY  MM DD hh mm WDIR WSPD GST  WVHT   DPD   APD MWD   PRES  ATMP  WTMP  DEWP  VIS  TIDE\n#yr  mo dy hr mn degT m/s  m/s     m   sec   sec degT   hPa  degC  degC  degC  nmi  hPa    ft\n2026 01 01 00 00 115  2.6  3.9 99.00 99.00 99.00 999 1019.3  25.2  25.8  21.3 99.0 99.00\n2026 01 01 00 10 112 99.0  4.0  1.81 11.43  8.43 301 1019.3  25.2 999.0  21.3 99.0 99.00\n2026 01 01 00 20 118  3.2  4.5 99.00 99.00 99.00 999 1019.3  25.2  25.9  21.4 99.0 99.00\n";
        let parsed = enso_ndbc_parse(body, &lsk).expect("stdmet parses");
        let wind = series_of(&parsed, EnsoSeries::Wspd);
        let sst = series_of(&parsed, EnsoSeries::Wtmp);
        assert_eq!(wind.len(), 2);
        assert_eq!(sst.len(), 2);
        assert_eq!(wind[0].1, 2.6);
        assert_eq!(wind[1].1, 3.2);
        assert_eq!(sst[0].1, 25.8);
        assert_eq!(sst[1].1, 25.9);
    }

    #[test]
    fn enso_ndbc_parse_carries_all_present_columns_with_gates() {
        let lsk = embedded_lsk().expect("embedded lsk");
        let body = "#YY  MM DD hh mm WDIR WSPD GST  WVHT   DPD   APD MWD   PRES  ATMP  WTMP  DEWP  VIS PTDY  TIDE\n2026 08 21 15 00 100  3.0  4.5   1.5   8.0   6.2 100 1013.2  26.1  25.8  23.0 10.0 -0.7  0.55\n";
        let parsed = enso_ndbc_parse(body, &lsk).expect("matrix parses");
        assert_eq!(series_of(&parsed, EnsoSeries::Wspd)[0].1, 3.0);
        assert_eq!(series_of(&parsed, EnsoSeries::Gst)[0].1, 4.5);
        assert_eq!(series_of(&parsed, EnsoSeries::Wvht)[0].1, 1.5);
        assert_eq!(series_of(&parsed, EnsoSeries::Dpd)[0].1, 8.0);
        assert_eq!(series_of(&parsed, EnsoSeries::Apd)[0].1, 6.2);
        assert_eq!(series_of(&parsed, EnsoSeries::Pres)[0].1, 1013.2);
        assert_eq!(series_of(&parsed, EnsoSeries::Ptdy)[0].1, -0.7);
        assert_eq!(series_of(&parsed, EnsoSeries::Atmp)[0].1, 26.1);
        assert_eq!(series_of(&parsed, EnsoSeries::Wtmp)[0].1, 25.8);
        assert_eq!(series_of(&parsed, EnsoSeries::Dewp)[0].1, 23.0);
        assert_eq!(series_of(&parsed, EnsoSeries::Vis)[0].1, 10.0);
        assert_eq!(series_of(&parsed, EnsoSeries::Tide)[0].1, 0.55);
    }

    #[test]
    fn enso_ndbc_parse_missing_column_leaves_series_absent() {
        let lsk = embedded_lsk().expect("embedded lsk");
        let body = "#YY  MM DD hh mm WDIR WSPD GST\n2026 08 21 15 10 100  3.0  4.0\n";
        let parsed = enso_ndbc_parse(body, &lsk).expect("wspd parses");
        assert_eq!(series_of(&parsed, EnsoSeries::Wspd).len(), 1);
        assert!(series_of(&parsed, EnsoSeries::Wtmp).is_empty());
        assert!(series_of(&parsed, EnsoSeries::Pres).is_empty());
    }

    #[test]
    fn enso_ndbc_parse_direction_columns_flow_as_sin_cos() {
        let lsk = embedded_lsk().expect("embedded lsk");
        let body = "#YY  MM DD hh mm WDIR WSPD GST WVHT DPD APD MWD PRES ATMP WTMP DEWP VIS TIDE\n2026 08 21 15 10   0  3.0 4.0  1.5 8.0 6.2  90 1013.2 26.1 25.8 23.0 10.0 0.5\n2026 08 21 15 00  90  3.0 4.0  1.5 8.0 6.2 999 1013.2 26.1 25.8 23.0 10.0 0.5\n";
        let parsed = enso_ndbc_parse(body, &lsk).expect("directions parse");
        let sin = series_of(&parsed, EnsoSeries::WdirSin);
        let cos = series_of(&parsed, EnsoSeries::WdirCos);
        let msin = series_of(&parsed, EnsoSeries::MwdSin);
        assert_eq!(sin.len(), 2);
        assert_eq!(cos.len(), 2);
        assert_eq!(sin[0].1, 0.0);
        assert_eq!(cos[0].1, 1.0);
        assert!((sin[1].1 - 1.0).abs() < 1e-9);
        assert!(cos[1].1.abs() < 1e-9);
        assert_eq!(msin.len(), 1);
        assert!((msin[0].1 - 1.0).abs() < 1e-9);
    }

    #[test]
    fn enso_ndbc_parse_rain_column_when_present() {
        let lsk = embedded_lsk().expect("embedded lsk");
        let body = "#YY MM DD hh mm WDIR WSPD GST WVHT DPD APD MWD PRES ATMP WTMP DEWP VIS TIDE RAIN\n2026 08 21 15 10 100 3.0 4.0 1.5 8.0 6.2 100 1013.2 26.1 25.8 23.0 10.0 0.5 0.12\n2026 08 21 15 00 100 3.0 4.0 1.5 8.0 6.2 100 1013.2 26.1 25.8 23.0 10.0 0.5 99.0\n";
        let parsed = enso_ndbc_parse(body, &lsk).expect("rain parses");
        let rain = series_of(&parsed, EnsoSeries::Rain);
        assert_eq!(rain.len(), 1);
        assert_eq!(rain[0].1, 0.12);
    }

    #[test]
    fn enso_ndbc_parse_all_missing_rows_is_none() {
        let lsk = embedded_lsk().expect("embedded lsk");
        let body = "#YY  MM DD hh mm WDIR WSPD GST  WVHT   DPD   APD MWD   PRES  ATMP  WTMP  DEWP  VIS PTDY  TIDE\n2026 08 21 15 10  MM   MM   MM    MM    MM    MM  MM    MM    MM    MM    MM   MM   MM    MM\n";
        assert!(enso_ndbc_parse(body, &lsk).is_none());
    }
}
