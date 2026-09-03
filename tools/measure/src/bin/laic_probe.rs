use omegaflow::archivar::{
    fetch_raw, fetch_raw_bytes, parse_json, scalar_of, ymd_to_days, JsonVal,
};
use omegaflow::cdn::{upload_asset, CDN_BASE, CDN_RELEASE};
use omegaflow::inflate::{gunzip, unzip};
use omegaflow::te::{surrogate_stats_phase, transfer_entropy_lag};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

const ERA_START_S: f64 = 1388534400.0;
const WINDOW_S: f64 = 259200.0;
const MAX_LAG_H: usize = 72;
const MIN_M: usize = 30;
const MIN_N_WINDOWS: usize = 30;
const M_MIN_EVENT: f64 = 6.0;
const M_MIN_REGION: f64 = 2.0;
const HARVEST_RADIUS_KM: f64 = 2000.0;
const STATION_MAX_KM: f64 = 3000.0;
const SURROGATE_SEED: u64 = 0x9E37_79B9_7F4A_7C15;
const FILL_BGS_F: f64 = 99999.0;
const FILL_OMNI_BZ: f64 = 999.9;
const F_MAX_NT: f64 = 100000.0;
const FDSN_LIMIT: usize = 20000;

const BGS_STATIONS_URL: &str =
    "https://imag-data.bgs.ac.uk/GIN_V1/GINServices?Request=GetCapabilities";
const BGS_HAPI_TEMPLATE: &str = "https://imag-data.bgs.ac.uk/GIN_V1/hapi/data?id={station}/best-avail/PT1M/xyzf&start={start}Z&stop={stop}Z&format=json";
const OMNI_HAPI_TEMPLATE: &str = "https://cdaweb.gsfc.nasa.gov/hapi/data?id=OMNI2_H0_MRG1HR&time.min={start}Z&time.max={stop}Z&parameters=BZ_GSM1800&format=json";
const FDSN_CATALOG_TEMPLATE: &str = "https://earthquake.usgs.gov/fdsnws/event/1/query?format=geojson&starttime={start}&endtime={stop}&minmagnitude={mag}&orderby=time&limit={limit}";
const FDSN_REGION_TEMPLATE: &str = "https://earthquake.usgs.gov/fdsnws/event/1/query?format=geojson&starttime={start}&endtime={stop}&latitude={lat}&longitude={lon}&maxradiuskm={radius}&minmagnitude={mag}&orderby=time&limit={limit}";
const SWARM_SATS: [&str; 3] = [
    "SW_OPER_FACATMS_2F",
    "SW_OPER_FACBTMS_2F",
    "SW_OPER_FACCTMS_2F",
];
const SWARM_HAPI_TEMPLATE: &str = "https://vires.services/hapi/data?id={sat}&start={start}Z&stop={stop}Z&parameters=Latitude,Longitude,FAC&format=json";
const GIM_TEMPLATE: &str = "ftp://gssc.esa.int/gnss/products/ionex/{year}/{doy}/COD0OPSRAP_{year}{doy}0000_01D_01H_GIM.INX.gz";
const GIM_EXPONENT: f64 = -1.0;
const TEC_CELL_S: f64 = 3600.0;
const TEC_N_CELLS: usize = 72;
const CHAMP_TEMPLATE: &str =
    "https://isdc-data.gfz.de/champ/ME/Level2/PLPT/{year}/CH-ME-2-PLPT+{date}_1.zip";
const FDSN_STATION_URL: &str = "https://service.iris.edu/fdsnws/station/1/query?format=text&level=station&latitude={lat}&longitude={lon}&maxradius=10&starttime={start}&endtime={end}&includerestricted=false";
const FDSN_DATASELECT_URL: &str = "https://service.iris.edu/fdsnws/dataselect/1/query?network={net}&station={sta}&location=*&channel={cha}&starttime={start}&endtime={end}";
const BFS_ODL_WFS: &str = "https://www.imis.bfs.de/ogc/opendata/ows";
const RADON_MAX_KM: f64 = 200.0;
const ISD_HISTORY_URL: &str = "https://www.ncei.noaa.gov/pub/data/noaa/isd-history.csv";
const ISD_ACCESS_TEMPLATE: &str =
    "https://www.ncei.noaa.gov/data/global-hourly/access/{year}/{station}.csv";
const WEATHER_MAX_KM: f64 = 500.0;
const NGL_STATIONS_URL: &str = "https://geodesy.unr.edu/NGLStationPages/DataHoldings.txt";
const NGL_TENV3_TEMPLATE: &str =
    "https://geodesy.unr.edu/gps_timeseries/IGS20/tenv3/IGS20/{station}.tenv3";
const GPS_MAX_KM: f64 = 200.0;
const MJD_UNIX_OFFSET: f64 = 40587.0;
const GPS_WINDOW_S: f64 = 7776000.0;
const GPS_CELL_S: f64 = 86400.0;
const GPS_N_CELLS: usize = 90;
const GPS_LAG_MAX_H: usize = 30;
const WGS84_A: f64 = 6378137.0;
const WGS84_B: f64 = 6356752.314245;

const A_A0: f64 = -3.969683028665376e+01;
const A_A1: f64 = 2.209460984245205e+02;
const A_A2: f64 = -2.759285104469687e+02;
const A_A3: f64 = 1.383577518672690e+02;
const A_A4: f64 = -3.066479806614716e+01;
const A_A5: f64 = 2.506628277459239e+00;
const A_B0: f64 = -5.447609879822406e+01;
const A_B1: f64 = 1.615858368580409e+02;
const A_B2: f64 = -1.556989798598866e+02;
const A_B3: f64 = 6.680131188771972e+01;
const A_B4: f64 = -1.328068155288572e+01;
const A_C0: f64 = -7.784894002430293e-03;
const A_C1: f64 = -3.223964580411365e-01;
const A_C2: f64 = -2.400758277161838e+00;
const A_C3: f64 = -2.549732539343734e+00;
const A_C4: f64 = 4.374664141464968e+00;
const A_C5: f64 = 2.938163982698783e+00;
const A_D0: f64 = 7.784695709041462e-03;
const A_D1: f64 = 3.224671290700398e-01;
const A_D2: f64 = 2.445134137142996e+00;
const A_D3: f64 = 3.754408661907416e+00;

#[derive(Clone)]
struct Station {
    id: String,
    lat: f64,
    lon: f64,
}

#[derive(Clone)]
struct Event {
    t0: f64,
    mag: f64,
    lat: f64,
    lon: f64,
}

#[derive(Clone)]
struct WindowData {
    kind: String,
    t0: f64,
    mag: f64,
    lat: f64,
    lon: f64,
    station: String,
    f: Vec<(f64, f64)>,
    bz: Vec<(f64, f64)>,
    region: Vec<(f64, f64, f64, f64)>,
    swarm: Vec<(f64, f64, f64, f64)>,
    tec: Vec<(f64, f64)>,
    champ: Vec<(f64, f64, f64, f64)>,
    env: Vec<(f64, f64)>,
    radon: Vec<(f64, f64)>,
    weather: Vec<(f64, f64)>,
    gps: Vec<(f64, f64)>,
}

struct WindowStat {
    n_cells: usize,
    n_rate_events: usize,
    excess: [Option<f64>; 2],
    control_excess: Option<f64>,
    curve: [Vec<Option<f64>>; 2],
    control_curve: Vec<Option<f64>>,
    f_gap_cells: usize,
    swarm_samples: Option<usize>,
    swarm_cells: Option<usize>,
    fac_excess: [Option<f64>; 2],
    tec_excess: [Option<f64>; 2],
    tec_control_excess: Option<f64>,
    tec_n_cells: usize,
    champ_excess: [Option<f64>; 2],
    champ_n_cells: usize,
    env_excess: [Option<f64>; 2],
    env_n_cells: usize,
    weather_excess: [Option<f64>; 2],
    weather_n_cells: usize,
    radon_excess: [Option<f64>; 2],
    radon_n_cells: usize,
    radon_weather_excess: [Option<f64>; 2],
    radon_weather_n_cells: usize,
    radon_env_excess: [Option<f64>; 2],
    radon_env_n_cells: usize,
    gps_radon_excess: [Option<f64>; 2],
    gps_radon_n_cells: usize,
}

struct StackStat {
    mean: f64,
    sd: f64,
    n: usize,
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn days_to_ymd(days: i64) -> (i64, u32, u32) {
    let z = days + 719468;
    let era = (if z >= 0 { z } else { z - 146096 }) / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    (if m <= 2 { y + 1 } else { y }, m as u32, d as u32)
}

fn unix_to_iso(u: f64) -> String {
    let days = u.div_euclid(86400.0) as i64;
    let rest = (u - days as f64 * 86400.0).floor();
    let (y, m, d) = days_to_ymd(days);
    let hh = (rest / 3600.0) as u32;
    let mm = ((rest - hh as f64 * 3600.0) / 60.0) as u32;
    let ss = (rest - hh as f64 * 3600.0 - mm as f64 * 60.0) as u32;
    format!("{y:04}-{m:02}-{d:02}T{hh:02}:{mm:02}:{ss:02}")
}

fn iso_to_unix(s: &str) -> Option<f64> {
    let (date, time) = s.trim_end_matches('Z').split_once('T')?;
    let mut dp = date.split('-');
    let y: i64 = dp.next()?.parse().ok()?;
    let m: u32 = dp.next()?.parse().ok()?;
    let d: u32 = dp.next()?.parse().ok()?;
    let days = ymd_to_days(y, m, d)? as f64;
    let mut tp = time.split(':');
    let hh: f64 = tp.next()?.parse().ok()?;
    let mm: f64 = tp
        .next()
        .map(|s| s.parse::<f64>().ok())
        .flatten()
        .unwrap_or(0.0);
    let ss: f64 = tp
        .next()
        .map(|s| s.parse::<f64>().ok())
        .flatten()
        .unwrap_or(0.0);
    Some(days * 86400.0 + hh * 3600.0 + mm * 60.0 + ss)
}

fn date_only_to_unix(s: &str) -> Option<f64> {
    let s = s.trim();
    let (y, m, d) = if s.len() == 8 && s.bytes().all(|b| b.is_ascii_digit()) {
        (
            s[0..4].parse::<i64>().ok()?,
            s[4..6].parse::<u32>().ok()?,
            s[6..8].parse::<u32>().ok()?,
        )
    } else {
        let mut dp = s.split('-');
        (
            dp.next()?.parse::<i64>().ok()?,
            dp.next()?.parse::<u32>().ok()?,
            dp.next()?.parse::<u32>().ok()?,
        )
    };
    ymd_to_days(y, m, d).map(|d| d as f64 * 86400.0)
}

fn haversine_km(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let r = 6371.0;
    let dlat = (lat2 - lat1).to_radians();
    let dlon = (lon2 - lon1).to_radians();
    let a = (dlat / 2.0).sin().powi(2)
        + lat1.to_radians().cos() * lat2.to_radians().cos() * (dlon / 2.0).sin().powi(2);
    2.0 * r * a.sqrt().atan2((1.0 - a).sqrt())
}

fn fetch_text(url: &str) -> Option<String> {
    fetch_raw(url, None, &[], 86400)
}

fn percent_encode(s: &str) -> String {
    let mut out = String::with_capacity(s.len() * 3);
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'.' | b'_' | b'-' | b'~' => {
                out.push(b as char)
            }
            _ => out.push_str(&format!("%{:02X}", b)),
        }
    }
    out
}

fn parse_stations_xml(body: &str) -> Vec<Station> {
    let mut out = Vec::new();
    for obs in body.split("<Observatory>").skip(1) {
        let tag = |name: &str| -> Option<String> {
            let open = format!("<{name}>");
            let start = obs.find(&open)? + open.len();
            let end = obs[start..].find(&format!("</{name}>"))? + start;
            Some(obs[start..end].trim().to_string())
        };
        let Some(code) = tag("Code") else { continue };
        let Some(lat) = tag("Latitude").and_then(|s| s.parse::<f64>().ok()) else {
            continue;
        };
        let Some(lon) = tag("Longitude").and_then(|s| s.parse::<f64>().ok()) else {
            continue;
        };
        out.push(Station { id: code, lat, lon });
    }
    out
}

fn nearest_station<'a>(lat: f64, lon: f64, stations: &'a [Station]) -> Option<&'a Station> {
    stations
        .iter()
        .filter_map(|s| {
            let d = haversine_km(lat, lon, s.lat, s.lon);
            (d <= STATION_MAX_KM).then_some((d, s))
        })
        .min_by(|a, b| a.0.total_cmp(&b.0))
        .map(|(_, s)| s)
}

fn hapi_series(body: &str, param: Option<&str>, skip: fn(f64) -> bool) -> Vec<(f64, f64)> {
    let j = match parse_json(body) {
        Some(v) => v,
        None => return Vec::new(),
    };
    let JsonVal::Obj(root) = j else {
        return Vec::new();
    };
    let Some(JsonVal::Arr(data)) = root.get("data") else {
        return Vec::new();
    };
    let mut col: HashMap<String, usize> = HashMap::new();
    if let Some(JsonVal::Arr(params)) = root.get("parameters") {
        for (i, p) in params.iter().enumerate() {
            if let JsonVal::Obj(po) = p {
                if let Some(JsonVal::Str(n)) = po.get("name") {
                    col.insert(n.clone(), i);
                }
            }
        }
    }
    let mut out = Vec::new();
    for row in data {
        let JsonVal::Arr(cells) = row else { continue };
        let Some(JsonVal::Str(ts)) = cells.first() else {
            continue;
        };
        let Some(t) = iso_to_unix(ts) else { continue };
        let idx = match param {
            Some(p) => match col.get(p) {
                Some(&i) => i,
                None => continue,
            },
            None => cells.len() - 1,
        };
        let Some(v) = cells.get(idx).and_then(scalar_of) else {
            continue;
        };
        if !v.is_finite() || skip(v) {
            continue;
        }
        out.push((t, v));
    }
    out.sort_by(|a, b| a.0.total_cmp(&b.0));
    out
}

fn swarm_pass_samples(body: &str) -> Vec<(f64, f64, f64, f64)> {
    let j = match parse_json(body) {
        Some(v) => v,
        None => return Vec::new(),
    };
    let JsonVal::Obj(root) = j else {
        return Vec::new();
    };
    let Some(JsonVal::Arr(data)) = root.get("data") else {
        return Vec::new();
    };
    let mut col: HashMap<String, usize> = HashMap::new();
    if let Some(JsonVal::Arr(params)) = root.get("parameters") {
        for (i, p) in params.iter().enumerate() {
            if let JsonVal::Obj(po) = p {
                if let Some(JsonVal::Str(n)) = po.get("name") {
                    col.insert(n.clone(), i);
                }
            }
        }
    }
    if col.is_empty() {
        col.insert("Latitude".to_string(), 1);
        col.insert("Longitude".to_string(), 2);
        col.insert("FAC".to_string(), 3);
    }
    let mut out = Vec::new();
    for row in data {
        let JsonVal::Arr(cells) = row else { continue };
        let Some(JsonVal::Str(ts)) = cells.first() else {
            continue;
        };
        let Some(t) = iso_to_unix(ts) else { continue };
        let lat = col
            .get("Latitude")
            .and_then(|&i| cells.get(i))
            .and_then(scalar_of);
        let lon = col
            .get("Longitude")
            .and_then(|&i| cells.get(i))
            .and_then(scalar_of);
        let fac = col
            .get("FAC")
            .and_then(|&i| cells.get(i))
            .and_then(scalar_of);
        let (Some(lat), Some(lon), Some(fac)) = (lat, lon, fac) else {
            continue;
        };
        if !fac.is_finite() {
            continue;
        }
        out.push((t, lat, lon, fac));
    }
    out
}

fn day_of_year(t: f64) -> (i64, u32) {
    let days = t.div_euclid(86400.0) as i64;
    let (y, _, _) = days_to_ymd(days);
    let y0 = ymd_to_days(y, 1, 1).unwrap_or(0) as i64;
    let doy = days - y0 + 1;
    (y, doy as u32)
}

fn gim_tec_series(body: &str, lat: f64, lon: f64) -> Vec<(f64, f64)> {
    let mut out = Vec::new();
    for g in omegaflow::ionex::parse_gim(body, GIM_EXPONENT) {
        if let Some(v) = omegaflow::ionex::tec_at(&g, lat, lon) {
            out.push((g.epoch_unix, v));
        }
    }
    out.sort_by(|a, b| a.0.total_cmp(&b.0));
    out
}

fn harvest_tec(dir: &str, t0: f64, lat: f64, lon: f64) -> Vec<(f64, f64)> {
    let t_start = t0 - WINDOW_S;
    let cache_dir = format!("{dir}/gim");
    let _ = std::fs::create_dir_all(&cache_dir);
    let mut out = Vec::new();
    let mut day = t_start.div_euclid(86400.0) as i64;
    let end_day = t0.div_euclid(86400.0) as i64;
    while day <= end_day {
        let (y, doy) = day_of_year(day as f64 * 86400.0);
        let stamp = format!("{y:04}{doy:03}");
        let path = format!("{cache_dir}/{stamp}.gz");
        let mut bytes: Option<Vec<u8>> = std::fs::read(&path).ok();
        if bytes.is_none() {
            let url = GIM_TEMPLATE
                .replace("{year}", &format!("{y:04}"))
                .replace("{doy}", &format!("{doy:03}"));
            bytes = fetch_raw_bytes(&url, 86400);
            if let Some(b) = &bytes {
                let _ = std::fs::write(&path, b);
            }
        }
        if let Some(b) = bytes {
            if let Some(text) = gunzip(&b) {
                let text = String::from_utf8_lossy(&text).to_string();
                for &(t, v) in &gim_tec_series(&text, lat, lon) {
                    if t >= t_start && t <= t0 {
                        out.push((t, v));
                    }
                }
            }
        }
        day += 1;
    }
    out.sort_by(|a, b| a.0.total_cmp(&b.0));
    out
}

fn geocentric_to_geodetic_lat(lat_geo: f64) -> f64 {
    let tan = lat_geo.to_radians().tan();
    (tan * (WGS84_A / WGS84_B) * (WGS84_A / WGS84_B))
        .atan()
        .to_degrees()
}

fn bcd(b: u8) -> Option<u32> {
    let hi = (b >> 4) & 0xF;
    let lo = b & 0xF;
    if hi > 9 || lo > 9 {
        None
    } else {
        Some(hi as u32 * 10 + lo as u32)
    }
}

fn mseed_time(h: &[u8]) -> Option<f64> {
    let year_binary = bcd(h[20]).is_none() || bcd(h[21]).is_none();
    let year = if year_binary {
        (h[20] as u32) << 8 | h[21] as u32
    } else {
        bcd(h[20]).unwrap() * 100 + bcd(h[21]).unwrap()
    };
    let doy = if year_binary {
        (h[22] as u32) << 8 | h[23] as u32
    } else {
        match (bcd(h[22]), bcd(h[23]), bcd(h[24])) {
            (Some(a), Some(b), Some(c)) => a * 100 + b * 10 + c,
            _ => (h[22] as u32) << 8 | h[23] as u32,
        }
    };
    let hour = bcd(h[25])?;
    let min = bcd(h[26])?;
    let sec = bcd(h[27])?;
    let frac = bcd(h[29]).unwrap_or(0) as f64 / 10000.0;
    let days = ymd_to_days(year as i64, 1, 1)? as f64;
    Some(
        (days + doy as f64 - 1.0) * 86400.0
            + hour as f64 * 3600.0
            + min as f64 * 60.0
            + sec as f64
            + frac,
    )
}

fn sign_extend(v: u32, bits: u32) -> i64 {
    let shift = 32 - bits;
    ((v << shift) as i32 as i64) >> shift
}

fn steim_decode(words: &[u32], encoding: u8, nsamp: usize) -> Vec<i32> {
    let nframes = words.len() / 16;
    let mut out: Vec<i32> = Vec::with_capacity(nsamp.min(words.len() * 8));
    let mut xn: i64 = 0;
    for fi in 0..nframes {
        if out.len() >= nsamp {
            break;
        }
        let base = fi * 16;
        let ctrl = words[base];
        let start: usize = if fi == 0 {
            out.push(words[base + 1] as i32);
            xn = words[base + 1] as i64;
            3
        } else {
            1
        };
        let mut diffs: Vec<i64> = Vec::new();
        for widx in start..16 {
            let w = words[base + widx];
            let nib = (ctrl >> (30 - 2 * widx)) & 3;
            match nib {
                0 => {}
                1 => {
                    for j in 0..4 {
                        diffs.push(((w >> (24 - 8 * j)) & 0xFF) as i8 as i64);
                    }
                }
                2 => {
                    if encoding == 10 {
                        diffs.push(sign_extend((w >> 16) & 0xFFFF, 16));
                        diffs.push(sign_extend(w & 0xFFFF, 16));
                    } else {
                        match (w >> 30) & 3 {
                            0 => return out,
                            1 => diffs.push(sign_extend(w & 0x3FFF_FFFF, 30)),
                            2 => {
                                diffs.push(sign_extend((w >> 15) & 0x7FFF, 15));
                                diffs.push(sign_extend(w & 0x7FFF, 15));
                            }
                            _ => {
                                diffs.push(sign_extend((w >> 20) & 0x3FF, 10));
                                diffs.push(sign_extend((w >> 10) & 0x3FF, 10));
                                diffs.push(sign_extend(w & 0x3FF, 10));
                            }
                        }
                    }
                }
                3 => {
                    if encoding == 10 {
                        diffs.push(sign_extend(w, 32));
                    } else {
                        match (w >> 30) & 3 {
                            0 => {
                                for j in 0..5 {
                                    diffs.push(sign_extend((w >> (24 - 6 * j)) & 0x3F, 6));
                                }
                            }
                            1 => {
                                for j in 0..6 {
                                    diffs.push(sign_extend((w >> (25 - 5 * j)) & 0x1F, 5));
                                }
                            }
                            2 => {
                                for j in 0..7 {
                                    diffs.push(sign_extend((w >> (24 - 4 * j)) & 0xF, 4));
                                }
                            }
                            _ => return out,
                        }
                    }
                }
                _ => {}
            }
        }
        let dstart = if fi == 0 { 1 } else { 0 };
        for idx in dstart..diffs.len() {
            if out.len() >= nsamp {
                break;
            }
            xn += diffs[idx];
            out.push(xn as i32);
        }
    }
    out.truncate(nsamp);
    out
}

fn mseed_samples(
    h: &[u8],
    data: &[u8],
    encoding: u8,
    byte_order: u8,
    nsamp: usize,
    rate: f64,
) -> Vec<(f64, f64)> {
    let big = byte_order == 1;
    let rd16 = |i: usize| -> u16 {
        let b = [data[i], data[i + 1]];
        if big {
            u16::from_be_bytes(b)
        } else {
            u16::from_le_bytes(b)
        }
    };
    let rd32 = |i: usize| -> u32 {
        let b = [data[i], data[i + 1], data[i + 2], data[i + 3]];
        if big {
            u32::from_be_bytes(b)
        } else {
            u32::from_le_bytes(b)
        }
    };
    let mut raw: Vec<i32> = Vec::with_capacity(nsamp);
    match encoding {
        1 => {
            for i in 0..nsamp {
                raw.push(rd16(i * 2) as i16 as i32);
            }
        }
        2 => {
            for i in 0..nsamp {
                let b = [data[i * 3], data[i * 3 + 1], data[i * 3 + 2]];
                let v = if big {
                    (b[0] as u32) << 16 | (b[1] as u32) << 8 | b[2] as u32
                } else {
                    (b[2] as u32) << 16 | (b[1] as u32) << 8 | b[0] as u32
                };
                raw.push(sign_extend(v, 24) as i32);
            }
        }
        3 => {
            for i in 0..nsamp {
                raw.push(rd32(i * 4) as i32);
            }
        }
        4 => {
            for i in 0..nsamp {
                let b = [
                    data[i * 4],
                    data[i * 4 + 1],
                    data[i * 4 + 2],
                    data[i * 4 + 3],
                ];
                let f = if big {
                    f32::from_be_bytes(b)
                } else {
                    f32::from_le_bytes(b)
                };
                raw.push(f as i32);
            }
        }
        5 => {
            for i in 0..nsamp {
                let b = [
                    data[i * 8],
                    data[i * 8 + 1],
                    data[i * 8 + 2],
                    data[i * 8 + 3],
                    data[i * 8 + 4],
                    data[i * 8 + 5],
                    data[i * 8 + 6],
                    data[i * 8 + 7],
                ];
                let f = if big {
                    f64::from_be_bytes(b)
                } else {
                    f64::from_le_bytes(b)
                };
                raw.push(f as i32);
            }
        }
        10 | 11 => {
            let mut words = Vec::with_capacity(data.len() / 4);
            let mut i = 0usize;
            while i + 4 <= data.len() {
                words.push(rd32(i));
                i += 4;
            }
            raw = steim_decode(&words, encoding, nsamp);
        }
        _ => return Vec::new(),
    }
    raw.truncate(nsamp);
    let t = mseed_time(h).unwrap_or(0.0);
    let mut out = Vec::with_capacity(raw.len());
    for (i, v) in raw.iter().enumerate() {
        out.push((t + i as f64 / rate, *v as f64));
    }
    out
}

fn parse_mseed(bytes: &[u8]) -> Vec<(f64, f64)> {
    let mut out = Vec::new();
    let mut pos = 0usize;
    while pos + 64 <= bytes.len() {
        let h = &bytes[pos..pos + 64];
        let data_start = ((h[44] as usize) << 8 | h[45] as usize).max(64);
        let nsamp = (h[30] as usize) << 8 | h[31] as usize;
        let factor = i16::from_be_bytes([h[32], h[33]]);
        let mult = i16::from_be_bytes([h[34], h[35]]);
        let rate = if factor > 0 {
            factor as f64 * mult as f64
        } else if factor < 0 {
            -(mult as f64) / (factor as f64)
        } else {
            0.0
        };
        let encoding = h[52];
        let byte_order = h[53];
        let rec_len = 1usize << h[54];
        if data_start >= rec_len || rate <= 0.0 {
            break;
        }
        let end = (pos + rec_len).min(bytes.len());
        let data = &bytes[pos + data_start..end];
        out.extend(mseed_samples(h, data, encoding, byte_order, nsamp, rate));
        pos += rec_len;
    }
    out
}

fn champ_samples_from_zip(bytes: &[u8]) -> Vec<(f64, f64, f64, f64)> {
    let Some(unzipped) = unzip(bytes) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    let mut pos = 0usize;
    while pos < unzipped.len() {
        let end = match unzipped[pos..]
            .iter()
            .position(|&b| b == b'\n')
            .map(|p| pos + p)
        {
            Some(e) => e,
            None => unzipped.len(),
        };
        let line = String::from_utf8_lossy(&unzipped[pos..end]);
        pos = end + 1;
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }
        let cols: Vec<&str> = line.split_whitespace().collect();
        if cols.len() < 12 {
            continue;
        }
        let (Ok(y), Ok(m), Ok(d)) = (
            cols[1].parse::<i64>(),
            cols[2].parse::<u32>(),
            cols[3].parse::<u32>(),
        ) else {
            continue;
        };
        let (Ok(hh), Ok(mm), Ok(ss), Ok(lat), Ok(lon), Ok(dens)) = (
            cols[4].parse::<f64>(),
            cols[5].parse::<f64>(),
            cols[6].parse::<f64>(),
            cols[8].parse::<f64>(),
            cols[9].parse::<f64>(),
            cols[10].parse::<f64>(),
        ) else {
            continue;
        };
        let Some(days) = ymd_to_days(y, m, d) else {
            continue;
        };
        let t = days as f64 * 86400.0 + hh * 3600.0 + mm * 60.0 + ss;
        let glat = geocentric_to_geodetic_lat(lat);
        out.push((t, glat, lon, dens));
    }
    out
}

fn harvest_champ(dir: &str, t0: f64, lat: f64, lon: f64) -> Vec<(f64, f64, f64, f64)> {
    let t_start = t0 - WINDOW_S;
    let cache_dir = format!("{dir}/champ");
    let _ = std::fs::create_dir_all(&cache_dir);
    let mut out = Vec::new();
    let mut day = t_start.div_euclid(86400.0) as i64;
    let end_day = t0.div_euclid(86400.0) as i64;
    while day <= end_day {
        let (y, m, d) = days_to_ymd(day);
        let date = format!("{y:04}-{m:02}-{d:02}");
        let stamp = format!("{y:04}{m:02}{d:02}");
        let path = format!("{cache_dir}/{stamp}.zip");
        let mut bytes: Option<Vec<u8>> = std::fs::read(&path).ok();
        if bytes.is_none() {
            let url = CHAMP_TEMPLATE
                .replace("{year}", &format!("{y:04}"))
                .replace("{date}", &date);
            bytes = fetch_raw_bytes(&url, 86400);
            if let Some(b) = &bytes {
                let _ = std::fs::write(&path, b);
            }
        }
        if let Some(b) = bytes {
            for &(t, glat, glon, dens) in &champ_samples_from_zip(&b) {
                if t >= t_start && t <= t0 {
                    out.push((t, glat, glon, dens));
                }
            }
        }
        day += 1;
    }
    out.sort_by(|a, b| a.0.total_cmp(&b.0));
    let _ = (lat, lon);
    out
}

fn nearest_stations(body: &str, clat: f64, clon: f64) -> Vec<(String, String)> {
    let mut all: Vec<(f64, String, String)> = Vec::new();
    for line in body.lines() {
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }
        let cols: Vec<&str> = line.split('|').collect();
        if cols.len() < 4 {
            continue;
        }
        let (Some(lat), Some(lon)) = (cols[2].parse::<f64>().ok(), cols[3].parse::<f64>().ok())
        else {
            continue;
        };
        let d = haversine_km(clat, clon, lat, lon);
        all.push((d, cols[0].to_string(), cols[1].to_string()));
    }
    all.sort_by(|a, b| a.0.total_cmp(&b.0));
    all.into_iter().map(|(_, n, s)| (n, s)).collect()
}

fn rms_envelope(samples: &[(f64, f64)], t_start: f64, cell_s: f64, n: usize) -> Vec<(f64, f64)> {
    let mut sums = vec![0.0f64; n];
    let mut counts = vec![0u32; n];
    for &(t, v) in samples {
        let idx = ((t - t_start) / cell_s).floor();
        if idx < 0.0 || idx >= n as f64 {
            continue;
        }
        sums[idx as usize] += v * v;
        counts[idx as usize] += 1;
    }
    (0..n)
        .filter(|&i| counts[i] > 0)
        .map(|i| {
            (
                t_start + i as f64 * cell_s + cell_s / 2.0,
                (sums[i] / counts[i] as f64).sqrt(),
            )
        })
        .collect()
}

fn harvest_mseed(dir: &str, t0: f64, lat: f64, lon: f64) -> (String, Vec<(f64, f64)>) {
    let _ = dir;
    let t_start = t0 - WINDOW_S;
    let start = unix_to_iso(t_start);
    let stop = unix_to_iso(t0);
    let station_url = FDSN_STATION_URL
        .replace("{lat}", &format!("{lat:.4}"))
        .replace("{lon}", &format!("{lon:.4}"))
        .replace("{start}", &start)
        .replace("{end}", &stop);
    let Some(station_body) = fetch_text(&station_url) else {
        return (String::new(), Vec::new());
    };
    let stations = nearest_stations(&station_body, lat, lon);
    for (net, sta) in &stations {
        for cha in ["LHZ", "BHZ"] {
            let ds_url = FDSN_DATASELECT_URL
                .replace("{net}", net)
                .replace("{sta}", sta)
                .replace("{cha}", cha)
                .replace("{start}", &start)
                .replace("{end}", &stop);
            let Some(bytes) = fetch_raw_bytes(&ds_url, 86400) else {
                continue;
            };
            let samples = parse_mseed(&bytes);
            if samples.len() < 100 {
                continue;
            }
            let envelope = rms_envelope(&samples, t_start, 1800.0, 144);
            if envelope.len() >= MIN_M {
                return (sta.clone(), envelope);
            }
        }
    }
    (String::new(), Vec::new())
}

fn bfs_odl_series(body: &str, t_start: f64, t0: f64) -> Vec<(f64, f64)> {
    let j = match parse_json(body) {
        Some(v) => v,
        None => return Vec::new(),
    };
    let JsonVal::Obj(root) = j else {
        return Vec::new();
    };
    let Some(JsonVal::Arr(features)) = root.get("features") else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for f in features {
        let JsonVal::Obj(fm) = f else { continue };
        let Some(JsonVal::Obj(props)) = fm.get("properties") else {
            continue;
        };
        let Some(JsonVal::Str(ts)) = props.get("start_measure") else {
            continue;
        };
        let Some(t) = iso_to_unix(ts) else { continue };
        let Some(v) = props.get("value").and_then(scalar_of) else {
            continue;
        };
        if !v.is_finite() || v <= 0.0 {
            continue;
        }
        if t < t_start || t > t0 {
            continue;
        }
        out.push((t, v));
    }
    out.sort_by(|a, b| a.0.total_cmp(&b.0));
    out
}

fn harvest_radon(dir: &str, t0: f64, lat: f64, lon: f64) -> Vec<(f64, f64)> {
    let _ = dir;
    let t_start = t0 - GPS_WINDOW_S;
    let dlat = RADON_MAX_KM / 111.0;
    let dlon = RADON_MAX_KM / (111.0 * lat.to_radians().cos().abs().max(0.05));
    let cql = format!(
        "BBOX(geom,{:.4},{:.4},{:.4},{:.4}) AND start_measure >= '{}' AND start_measure <= '{}'",
        lon - dlon,
        lat - dlat,
        lon + dlon,
        lat + dlat,
        unix_to_iso(t_start),
        unix_to_iso(t0)
    );
    let url = format!(
        "{}?service=WFS&version=1.1.0&request=GetFeature&typeName=opendata:odlinfo_timeseries_odl_1h&outputFormat=application/json&CQL_FILTER={}",
        BFS_ODL_WFS,
        percent_encode(&cql)
    );
    match fetch_text(&url) {
        Some(body) => bfs_odl_series(&body, t_start, t0),
        None => Vec::new(),
    }
}

struct IsdStation {
    usaf: String,
    wban: String,
    lat: f64,
    lon: f64,
    begin: Option<f64>,
    end: Option<f64>,
}

fn csv_fields(line: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut in_q = false;
    for c in line.chars() {
        match c {
            '"' => in_q = !in_q,
            ',' if !in_q => out.push(std::mem::take(&mut cur)),
            _ => cur.push(c),
        }
    }
    out.push(cur);
    out
}

fn parse_isd_history(body: &str) -> Vec<IsdStation> {
    let mut out = Vec::new();
    for (li, line) in body.lines().enumerate() {
        if li == 0 {
            continue;
        }
        let f = csv_fields(line);
        if f.len() < 8 {
            continue;
        }
        let lat = f[6].trim().parse::<f64>().ok();
        let lon = f[7].trim().parse::<f64>().ok();
        let (Some(lat), Some(lon)) = (lat, lon) else {
            continue;
        };
        if !lat.is_finite() || !lon.is_finite() || (lat == 0.0 && lon == 0.0) {
            continue;
        }
        let begin = f.get(9).and_then(|s| date_only_to_unix(s));
        let end = f.get(10).and_then(|s| date_only_to_unix(s));
        out.push(IsdStation {
            usaf: f[0].trim().to_string(),
            wban: f[1].trim().to_string(),
            lat,
            lon,
            begin,
            end,
        });
    }
    out
}

fn nearest_isd_station(
    stations: &[IsdStation],
    lat: f64,
    lon: f64,
    t_start: f64,
    t0: f64,
) -> Option<&IsdStation> {
    stations
        .iter()
        .filter_map(|s| {
            if let Some(end) = s.end {
                if end < t_start {
                    return None;
                }
            }
            if let Some(begin) = s.begin {
                if begin > t0 {
                    return None;
                }
            }
            let d = haversine_km(lat, lon, s.lat, s.lon);
            (d <= WEATHER_MAX_KM).then_some((d, s))
        })
        .min_by(|a, b| a.0.total_cmp(&b.0))
        .map(|(_, s)| s)
}

fn isd_slp_series(body: &str, t_start: f64, t0: f64) -> Vec<(f64, f64)> {
    let mut out = Vec::new();
    for (li, line) in body.lines().enumerate() {
        if li == 0 {
            continue;
        }
        let f = csv_fields(line);
        if f.len() < 16 {
            continue;
        }
        let Some(t) = iso_to_unix(f[1].trim()) else {
            continue;
        };
        if t < t_start || t > t0 {
            continue;
        }
        let raw = f[15].split(',').next().unwrap_or("").trim();
        let Some(v) = raw.parse::<f64>().ok() else {
            continue;
        };
        if !v.is_finite() || v <= 0.0 || v >= 11000.0 {
            continue;
        }
        out.push((t, v / 10.0));
    }
    out.sort_by(|a, b| a.0.total_cmp(&b.0));
    out
}

fn harvest_weather(dir: &str, t0: f64, lat: f64, lon: f64) -> (String, Vec<(f64, f64)>) {
    let t_start = t0 - WINDOW_S;
    let history_path = format!("{dir}/isd_history.csv");
    let history_body = match std::fs::read_to_string(&history_path) {
        Ok(b) => b,
        Err(_) => {
            let Some(b) = fetch_text(ISD_HISTORY_URL) else {
                return (String::new(), Vec::new());
            };
            let _ = std::fs::write(&history_path, &b);
            b
        }
    };
    let stations = parse_isd_history(&history_body);
    let Some(sta) = nearest_isd_station(&stations, lat, lon, t_start, t0) else {
        return (String::new(), Vec::new());
    };
    let id = format!("{}{}", sta.usaf, sta.wban);
    let cache_dir = format!("{dir}/isd");
    let _ = std::fs::create_dir_all(&cache_dir);
    let mut out = Vec::new();
    let mut day = t_start.div_euclid(86400.0) as i64;
    let end_day = t0.div_euclid(86400.0) as i64;
    while day <= end_day {
        let (y, _, _) = days_to_ymd(day);
        let path = format!("{cache_dir}/{y}_{id}.csv");
        let mut bytes: Option<String> = std::fs::read_to_string(&path).ok();
        if bytes.is_none() {
            let url = ISD_ACCESS_TEMPLATE
                .replace("{year}", &format!("{y:04}"))
                .replace("{station}", &id);
            bytes = fetch_text(&url);
            if let Some(b) = &bytes {
                let _ = std::fs::write(&path, b);
            }
        }
        if let Some(b) = bytes {
            out.extend(isd_slp_series(&b, t_start, t0));
        }
        day += 1;
    }
    out.sort_by(|a, b| a.0.total_cmp(&b.0));
    (id, out)
}

fn harvest_gps(dir: &str, t0: f64, lat: f64, lon: f64) -> (String, Vec<(f64, f64)>) {
    let t_start = t0 - GPS_WINDOW_S;
    let stations_path = format!("{dir}/ngl_stations.txt");
    let stations_body = match std::fs::read_to_string(&stations_path) {
        Ok(b) => b,
        Err(_) => {
            let Some(b) = fetch_text(NGL_STATIONS_URL) else {
                return (String::new(), Vec::new());
            };
            let _ = std::fs::write(&stations_path, &b);
            b
        }
    };
    let stations = parse_ngl_stations(&stations_body);
    let Some(sta) = nearest_gps_station(&stations, lat, lon, t_start, t0) else {
        return (String::new(), Vec::new());
    };
    let cache_dir = format!("{dir}/ngl");
    let _ = std::fs::create_dir_all(&cache_dir);
    let path = format!("{cache_dir}/{}.tenv3", sta.id);
    let body = match std::fs::read_to_string(&path) {
        Ok(b) => b,
        Err(_) => {
            let url = NGL_TENV3_TEMPLATE.replace("{station}", &sta.id);
            let Some(b) = fetch_text(&url) else {
                return (sta.id.clone(), Vec::new());
            };
            let _ = std::fs::write(&path, &b);
            b
        }
    };
    (sta.id.clone(), tenv3_series(&body, t_start, t0))
}

struct GpsStation {
    id: String,
    lat: f64,
    lon: f64,
    begin: Option<f64>,
    end: Option<f64>,
}

fn parse_ngl_stations(body: &str) -> Vec<GpsStation> {
    let mut out = Vec::new();
    for (li, line) in body.lines().enumerate() {
        if li == 0 {
            continue;
        }
        let cols: Vec<&str> = line.split_whitespace().collect();
        if cols.len() < 3 {
            continue;
        }
        let lat = cols[1].parse::<f64>().ok();
        let lon = cols[2].parse::<f64>().ok();
        let (Some(lat), Some(lon)) = (lat, lon) else {
            continue;
        };
        if !lat.is_finite() || !lon.is_finite() {
            continue;
        }
        let begin = cols.get(7).and_then(|s| date_only_to_unix(s));
        let end = cols.get(8).and_then(|s| date_only_to_unix(s));
        out.push(GpsStation {
            id: cols[0].to_string(),
            lat,
            lon,
            begin,
            end,
        });
    }
    out
}

fn nearest_gps_station(
    stations: &[GpsStation],
    lat: f64,
    lon: f64,
    t_start: f64,
    t0: f64,
) -> Option<&GpsStation> {
    stations
        .iter()
        .filter_map(|s| {
            if let Some(end) = s.end {
                if end < t_start {
                    return None;
                }
            }
            if let Some(begin) = s.begin {
                if begin > t0 {
                    return None;
                }
            }
            let d = haversine_km(lat, lon, s.lat, s.lon);
            (d <= GPS_MAX_KM).then_some((d, s))
        })
        .min_by(|a, b| a.0.total_cmp(&b.0))
        .map(|(_, s)| s)
}

fn tenv3_series(body: &str, t_start: f64, t0: f64) -> Vec<(f64, f64)> {
    let mut out = Vec::new();
    for (li, line) in body.lines().enumerate() {
        if li == 0 {
            continue;
        }
        let cols: Vec<&str> = line.split_whitespace().collect();
        if cols.len() < 13 {
            continue;
        }
        let Some(mjd) = cols[3].parse::<f64>().ok() else {
            continue;
        };
        let t = (mjd - MJD_UNIX_OFFSET) * 86400.0;
        if t < t_start || t > t0 {
            continue;
        }
        let Some(east) = cols[8].parse::<f64>().ok() else {
            continue;
        };
        if !east.is_finite() {
            continue;
        }
        out.push((t, east));
    }
    out.sort_by(|a, b| a.0.total_cmp(&b.0));
    out
}

fn catalog_events(body: &str, mag_min: f64) -> Vec<Event> {
    let j = match parse_json(body) {
        Some(v) => v,
        None => return Vec::new(),
    };
    let JsonVal::Obj(root) = j else {
        return Vec::new();
    };
    let Some(JsonVal::Arr(features)) = root.get("features") else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for f in features {
        let JsonVal::Obj(fm) = f else { continue };
        let prop = |key: &str| -> Option<f64> {
            match fm.get("properties") {
                Some(JsonVal::Obj(pm)) => pm.get(key).and_then(scalar_of),
                _ => None,
            }
        };
        let Some(t_ms) = prop("time") else { continue };
        let Some(mag) = prop("mag") else { continue };
        let Some(JsonVal::Arr(coords)) = (match fm.get("geometry") {
            Some(JsonVal::Obj(gm)) => gm.get("coordinates"),
            _ => None,
        }) else {
            continue;
        };
        let Some(lon) = coords.first().and_then(scalar_of) else {
            continue;
        };
        let Some(lat) = coords.get(1).and_then(scalar_of) else {
            continue;
        };
        if mag < mag_min {
            continue;
        }
        out.push(Event {
            t0: t_ms / 1000.0,
            mag,
            lat,
            lon,
        });
    }
    out
}

fn region_records(
    body: &str,
    clat: f64,
    clon: f64,
    radius_km: f64,
    t0: f64,
) -> Vec<(f64, f64, f64, f64)> {
    let j = match parse_json(body) {
        Some(v) => v,
        None => return Vec::new(),
    };
    let JsonVal::Obj(root) = j else {
        return Vec::new();
    };
    let Some(JsonVal::Arr(features)) = root.get("features") else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for f in features {
        let JsonVal::Obj(fm) = f else { continue };
        let Some(t_ms) = (match fm.get("properties") {
            Some(JsonVal::Obj(pm)) => pm.get("time").and_then(scalar_of),
            _ => None,
        }) else {
            continue;
        };
        let Some(mag) = (match fm.get("properties") {
            Some(JsonVal::Obj(pm)) => pm.get("mag").and_then(scalar_of),
            _ => None,
        }) else {
            continue;
        };
        let Some(JsonVal::Arr(coords)) = (match fm.get("geometry") {
            Some(JsonVal::Obj(gm)) => gm.get("coordinates"),
            _ => None,
        }) else {
            continue;
        };
        let Some(lon) = coords.first().and_then(scalar_of) else {
            continue;
        };
        let Some(lat) = coords.get(1).and_then(scalar_of) else {
            continue;
        };
        let epoch = t_ms / 1000.0;
        if epoch >= t0 || haversine_km(clat, clon, lat, lon) > radius_km {
            continue;
        }
        out.push((epoch, lat, lon, mag));
    }
    out.sort_by(|a, b| a.0.total_cmp(&b.0));
    out
}

fn bin_mean(series: &[(f64, f64)], t_start: f64, cell_s: f64, n: usize) -> Vec<Option<f32>> {
    let mut sums = vec![0.0f64; n];
    let mut counts = vec![0u32; n];
    for &(t, v) in series {
        let idx = ((t - t_start) / cell_s).floor();
        if idx < 0.0 || idx >= n as f64 {
            continue;
        }
        let i = idx as usize;
        sums[i] += v;
        counts[i] += 1;
    }
    (0..n)
        .map(|i| {
            if counts[i] > 0 {
                Some((sums[i] / counts[i] as f64) as f32)
            } else {
                None
            }
        })
        .collect()
}

fn bin_count(epochs: &[f64], t_start: f64, cell_s: f64, n: usize) -> Vec<Option<f32>> {
    let mut counts = vec![0u32; n];
    for &t in epochs {
        let idx = ((t - t_start) / cell_s).floor();
        if idx < 0.0 || idx >= n as f64 {
            continue;
        }
        counts[idx as usize] += 1;
    }
    counts.into_iter().map(|c| Some(c as f32)).collect()
}

fn bin_fac(
    swarm: &[(f64, f64, f64, f64)],
    clat: f64,
    clon: f64,
    radius_km: f64,
    t_start: f64,
    cell_s: f64,
    n: usize,
) -> (Vec<Option<f32>>, usize, usize) {
    let mut sums = vec![0.0f64; n];
    let mut counts = vec![0u32; n];
    let mut samples = 0usize;
    for &(t, lat, lon, fac) in swarm {
        if haversine_km(clat, clon, lat, lon) > radius_km {
            continue;
        }
        samples += 1;
        let idx = ((t - t_start) / cell_s).floor();
        if idx < 0.0 || idx >= n as f64 {
            continue;
        }
        let i = idx as usize;
        sums[i] += fac;
        counts[i] += 1;
    }
    let cells = (0..n)
        .map(|i| {
            if counts[i] > 0 {
                Some((sums[i] / counts[i] as f64) as f32)
            } else {
                None
            }
        })
        .collect();
    let covered = counts.iter().filter(|&&c| c > 0).count();
    (cells, samples, covered)
}

fn pair_cells(a: &[Option<f32>], b: &[Option<f32>]) -> (Vec<f32>, Vec<f32>) {
    let mut xs = Vec::new();
    let mut ys = Vec::new();
    for (ca, cb) in a.iter().zip(b.iter()) {
        if let (Some(x), Some(y)) = (ca, cb) {
            xs.push(*x);
            ys.push(*y);
        }
    }
    (xs, ys)
}

fn sweep_excess(
    xs: &[f32],
    ys: &[f32],
    lag_h_max: usize,
    cells_per_lag: usize,
    kde_scale: f32,
) -> (Vec<Option<f64>>, Option<f64>, usize) {
    let n = xs.len();
    let xs: Vec<f32> = xs.iter().map(|v| v * kde_scale).collect();
    let ys: Vec<f32> = ys.iter().map(|v| v * kde_scale).collect();
    let mut curve = vec![None; lag_h_max + 1];
    let mut best: Option<f64> = None;
    let mut computed = 0usize;
    for lag_h in 0..=lag_h_max {
        let cell_lag = lag_h * cells_per_lag;
        if n < MIN_M + cell_lag {
            continue;
        }
        let seed = SURROGATE_SEED ^ (lag_h as u64).wrapping_mul(0x517C_C1B7_2722_0A95);
        let te = match transfer_entropy_lag(&xs, &ys, cell_lag) {
            Some(t) => t,
            None => continue,
        };
        let thr = match surrogate_stats_phase(&xs, &ys, cell_lag, seed) {
            Some((_, _, t)) => t,
            None => continue,
        };
        let excess = te - thr;
        curve[lag_h] = Some(excess);
        best = Some(best.map_or(excess, |b: f64| b.max(excess)));
        computed += 1;
    }
    (curve, best, computed)
}

fn pair_excess(
    driver: &[(f64, f64)],
    target: &[(f64, f64)],
    t_start: f64,
    cell_s: f64,
    n_cells: usize,
    cells_per_lag: usize,
    lag_h_max: usize,
    kde_scale: f32,
) -> ([Option<f64>; 2], usize) {
    let d_cells = bin_mean(driver, t_start, cell_s, n_cells);
    let t_cells = bin_mean(target, t_start, cell_s, n_cells);
    let (d_fs, t_fs) = pair_cells(&d_cells, &t_cells);
    let n = d_fs.len();
    if n < MIN_M {
        return ([None, None], n);
    }
    let (_, d_to_t, _) = sweep_excess(&t_fs, &d_fs, lag_h_max, cells_per_lag, kde_scale);
    let (_, t_to_d, _) = sweep_excess(&d_fs, &t_fs, lag_h_max, cells_per_lag, kde_scale);
    ([d_to_t, t_to_d], n)
}

fn norm_quantile(p: f64) -> f64 {
    let p = p.clamp(1e-12, 1.0 - 1e-12);
    let plow = 0.02425;
    if p < plow {
        let q = (-2.0 * p.ln()).sqrt();
        (((((A_C0 * q + A_C1) * q + A_C2) * q + A_C3) * q + A_C4) * q + A_C5)
            / ((((A_D0 * q + A_D1) * q + A_D2) * q + A_D3) * q + 1.0)
    } else if p <= 1.0 - plow {
        let q = p - 0.5;
        let r = q * q;
        (((((A_A0 * r + A_A1) * r + A_A2) * r + A_A3) * r + A_A4) * r + A_A5) * q
            / (((((A_B0 * r + A_B1) * r + A_B2) * r + A_B3) * r + A_B4) * r + 1.0)
    } else {
        -norm_quantile(1.0 - p)
    }
}

fn window_stat(data: &WindowData, radius_km: f64, cell_s: f64, kde_scale: f32) -> WindowStat {
    let n_cells = (WINDOW_S / cell_s) as usize;
    let t_start = data.t0 - WINDOW_S;
    let mut stat = WindowStat {
        n_cells: 0,
        n_rate_events: 0,
        excess: [None, None],
        control_excess: None,
        curve: [Vec::new(), Vec::new()],
        control_curve: Vec::new(),
        f_gap_cells: 0,
        swarm_samples: None,
        swarm_cells: None,
        fac_excess: [None, None],
        tec_excess: [None, None],
        tec_control_excess: None,
        tec_n_cells: 0,
        champ_excess: [None, None],
        champ_n_cells: 0,
        env_excess: [None, None],
        env_n_cells: 0,
        weather_excess: [None, None],
        weather_n_cells: 0,
        radon_excess: [None, None],
        radon_n_cells: 0,
        radon_weather_excess: [None, None],
        radon_weather_n_cells: 0,
        radon_env_excess: [None, None],
        radon_env_n_cells: 0,
        gps_radon_excess: [None, None],
        gps_radon_n_cells: 0,
    };
    let rate_epochs: Vec<f64> = data
        .region
        .iter()
        .filter(|&&(_, lat, lon, _)| haversine_km(data.lat, data.lon, lat, lon) <= radius_km)
        .map(|&(t, _, _, _)| t)
        .collect();
    stat.n_rate_events = rate_epochs.len();

    let f_cells = bin_mean(&data.f, t_start, cell_s, n_cells);
    let rate_cells = bin_count(&rate_epochs, t_start, cell_s, n_cells);
    stat.f_gap_cells = f_cells.iter().filter(|c| c.is_none()).count();
    let (fs, rs) = pair_cells(&f_cells, &rate_cells);
    stat.n_cells = fs.len();

    let cells_per_lag = (3600.0 / cell_s).max(1.0) as usize;
    let (curve_li, best_li, _) = sweep_excess(&fs, &rs, MAX_LAG_H, cells_per_lag, kde_scale);
    let (curve_il, best_il, _) = sweep_excess(&rs, &fs, MAX_LAG_H, cells_per_lag, kde_scale);
    stat.curve = [curve_li, curve_il];
    stat.excess = [best_li, best_il];

    let f_h = bin_mean(&data.f, t_start, 3600.0, (WINDOW_S / 3600.0) as usize);
    let bz_h = bin_mean(&data.bz, t_start, 3600.0, (WINDOW_S / 3600.0) as usize);
    let (fh, bh) = pair_cells(&f_h, &bz_h);
    let (control_curve, control_best, _) = sweep_excess(&fh, &bh, 48, 1, kde_scale);
    stat.control_curve = control_curve;
    stat.control_excess = control_best;

    if !data.swarm.is_empty() {
        let (fac_cells, samples, covered) = bin_fac(
            &data.swarm,
            data.lat,
            data.lon,
            radius_km,
            t_start,
            cell_s,
            n_cells,
        );
        stat.swarm_samples = Some(samples);
        stat.swarm_cells = Some(covered);
        let (fac_fs, fac_rs) = pair_cells(&fac_cells, &rate_cells);
        if fac_fs.len() >= MIN_M {
            let (_, best_fac_li, _) =
                sweep_excess(&fac_fs, &fac_rs, MAX_LAG_H, cells_per_lag, kde_scale);
            let (_, best_fac_il, _) =
                sweep_excess(&fac_rs, &fac_fs, MAX_LAG_H, cells_per_lag, kde_scale);
            stat.fac_excess = [best_fac_li, best_fac_il];
        }
    }
    if !data.tec.is_empty() {
        let tec_cells = bin_mean(&data.tec, t_start, TEC_CELL_S, TEC_N_CELLS);
        let rate_cells_h = bin_count(&rate_epochs, t_start, TEC_CELL_S, TEC_N_CELLS);
        let (tec_fs, tec_rs) = pair_cells(&tec_cells, &rate_cells_h);
        stat.tec_n_cells = tec_fs.len();
        if tec_fs.len() >= MIN_M {
            let (_, best_tl, _) = sweep_excess(&tec_fs, &tec_rs, MAX_LAG_H, 1, kde_scale);
            let (_, best_lt, _) = sweep_excess(&tec_rs, &tec_fs, MAX_LAG_H, 1, kde_scale);
            stat.tec_excess = [best_tl, best_lt];
        }
        let (tec_fh, bz_th) = pair_cells(&tec_cells, &bz_h);
        if tec_fh.len() >= MIN_M {
            let (_, best_ctrl, _) = sweep_excess(&tec_fh, &bz_th, 48, 1, kde_scale);
            stat.tec_control_excess = best_ctrl;
        }
    }
    if !data.champ.is_empty() {
        let mut sums = vec![0.0f64; n_cells];
        let mut counts = vec![0u32; n_cells];
        let mut paired_cells = 0usize;
        for &(t, glat, glon, dens) in &data.champ {
            if haversine_km(data.lat, data.lon, glat, glon) > radius_km {
                continue;
            }
            let idx = ((t - t_start) / cell_s).floor();
            if idx < 0.0 || idx >= n_cells as f64 {
                continue;
            }
            let i = idx as usize;
            sums[i] += dens;
            counts[i] += 1;
        }
        for c in counts.iter() {
            if *c > 0 {
                paired_cells += 1;
            }
        }
        stat.champ_n_cells = paired_cells;
        let champ_cells: Vec<Option<f32>> = (0..n_cells)
            .map(|i| {
                if counts[i] > 0 {
                    Some((sums[i] / counts[i] as f64) as f32)
                } else {
                    None
                }
            })
            .collect();
        let (champ_fs, champ_rs) = pair_cells(&champ_cells, &rate_cells);
        if champ_fs.len() >= MIN_M {
            let (_, best_cl, _) =
                sweep_excess(&champ_fs, &champ_rs, MAX_LAG_H, cells_per_lag, kde_scale);
            let (_, best_lc, _) =
                sweep_excess(&champ_rs, &champ_fs, MAX_LAG_H, cells_per_lag, kde_scale);
            stat.champ_excess = [best_cl, best_lc];
        }
    }
    if !data.env.is_empty() {
        let env_cells = bin_mean(&data.env, t_start, cell_s, n_cells);
        let (env_fs, env_rs) = pair_cells(&env_cells, &f_cells);
        stat.env_n_cells = env_fs.len();
        if env_fs.len() >= MIN_M {
            let (_, best_ef, _) =
                sweep_excess(&env_rs, &env_fs, MAX_LAG_H, cells_per_lag, kde_scale);
            let (_, best_fe, _) =
                sweep_excess(&env_fs, &env_rs, MAX_LAG_H, cells_per_lag, kde_scale);
            stat.env_excess = [best_ef, best_fe];
        }
    }
    if !data.weather.is_empty() {
        let (ex, n) = pair_excess(
            &data.weather,
            &data.f,
            t_start,
            cell_s,
            n_cells,
            cells_per_lag,
            MAX_LAG_H,
            kde_scale,
        );
        stat.weather_excess = ex;
        stat.weather_n_cells = n;
    }
    if !data.radon.is_empty() {
        let (ex, n) = pair_excess(
            &data.radon,
            &data.f,
            t_start,
            cell_s,
            n_cells,
            cells_per_lag,
            MAX_LAG_H,
            kde_scale,
        );
        stat.radon_excess = ex;
        stat.radon_n_cells = n;
    }
    if !data.radon.is_empty() && !data.weather.is_empty() {
        let (ex, n) = pair_excess(
            &data.radon,
            &data.weather,
            t_start,
            cell_s,
            n_cells,
            cells_per_lag,
            MAX_LAG_H,
            kde_scale,
        );
        stat.radon_weather_excess = ex;
        stat.radon_weather_n_cells = n;
    }
    if !data.radon.is_empty() && !data.env.is_empty() {
        let (ex, n) = pair_excess(
            &data.radon,
            &data.env,
            t_start,
            cell_s,
            n_cells,
            cells_per_lag,
            MAX_LAG_H,
            kde_scale,
        );
        stat.radon_env_excess = ex;
        stat.radon_env_n_cells = n;
    }
    if !data.gps.is_empty() && !data.radon.is_empty() {
        let gps_t_start = data.t0 - GPS_WINDOW_S;
        let (ex, n) = pair_excess(
            &data.gps,
            &data.radon,
            gps_t_start,
            GPS_CELL_S,
            GPS_N_CELLS,
            1,
            GPS_LAG_MAX_H,
            kde_scale,
        );
        stat.gps_radon_excess = ex;
        stat.gps_radon_n_cells = n;
    }
    stat
}

fn stack_stat(vals: &[f64]) -> StackStat {
    let n = vals.len();
    if n == 0 {
        return StackStat {
            mean: f64::NAN,
            sd: f64::NAN,
            n: 0,
        };
    }
    let mean = vals.iter().sum::<f64>() / n as f64;
    let var = vals.iter().map(|&v| (v - mean) * (v - mean)).sum::<f64>() / n as f64;
    StackStat {
        mean,
        sd: var.sqrt(),
        n,
    }
}

fn verdict_line(label: &str, ev: &StackStat, nu: &StackStat, z: f64) {
    if nu.n < 2 {
        println!(
            "{label:<32} | null ensemble void (n = {}) — no threshold, the comparison stays unmeasured",
            nu.n
        );
        return;
    }
    let thr2 = nu.mean + 2.0 * nu.sd;
    let thrb = nu.mean + z * nu.sd;
    if ev.n < MIN_N_WINDOWS {
        println!(
            "{label:<32} | no statement possible (n = {} < {MIN_N_WINDOWS})",
            ev.n
        );
    } else if ev.mean > thr2 {
        println!(
            "{label:<32} | stack {:.4e} > threshold {:.4e} (2σ) | ARROW (bonferroni thr {thrb:.4e})",
            ev.mean, thr2
        );
    } else {
        println!(
            "{label:<32} | stack {:.4e} ≤ threshold {:.4e} (2σ) | silent (bonferroni thr {thrb:.4e})",
            ev.mean, thr2
        );
    }
}

fn series_json(pairs: &[(f64, f64)]) -> String {
    let mut s = String::from("[");
    for (i, &(t, v)) in pairs.iter().enumerate() {
        if i > 0 {
            s.push(',');
        }
        s.push('[');
        s.push_str(&t.to_string());
        s.push(',');
        s.push_str(&v.to_string());
        s.push(']');
    }
    s.push(']');
    s
}

fn quad_json(quads: &[(f64, f64, f64, f64)]) -> String {
    let mut s = String::from("[");
    for (i, &(t, lat, lon, v)) in quads.iter().enumerate() {
        if i > 0 {
            s.push(',');
        }
        s.push('[');
        s.push_str(&t.to_string());
        s.push(',');
        s.push_str(&lat.to_string());
        s.push(',');
        s.push_str(&lon.to_string());
        s.push(',');
        s.push_str(&v.to_string());
        s.push(']');
    }
    s.push(']');
    s
}

fn window_json(data: &WindowData) -> String {
    let mut s = String::from("{");
    s.push_str(&format!("\"kind\":\"{}\",", data.kind));
    s.push_str(&format!("\"t0\":{},", data.t0));
    s.push_str(&format!("\"mag\":{},", data.mag));
    s.push_str(&format!("\"lat\":{},", data.lat));
    s.push_str(&format!("\"lon\":{},", data.lon));
    s.push_str(&format!("\"station\":\"{}\",", data.station));
    s.push_str("\"f\":");
    s.push_str(&series_json(&data.f));
    s.push_str(",\"bz\":");
    s.push_str(&series_json(&data.bz));
    s.push_str(",\"region\":");
    s.push_str(&quad_json(&data.region));
    s.push_str(",\"swarm\":");
    s.push_str(&quad_json(&data.swarm));
    s.push('}');
    s
}

fn parse_window_file(body: &str) -> Option<WindowData> {
    let j = parse_json(body)?;
    let JsonVal::Obj(root) = j else { return None };
    let get_num = |key: &str| -> Option<f64> {
        match root.get(key) {
            Some(v) => scalar_of(v),
            None => None,
        }
    };
    let get_str = |key: &str| -> Option<String> {
        match root.get(key) {
            Some(JsonVal::Str(s)) => Some(s.clone()),
            _ => None,
        }
    };
    let mut data = WindowData {
        kind: get_str("kind")?,
        t0: get_num("t0")?,
        mag: get_num("mag")?,
        lat: get_num("lat")?,
        lon: get_num("lon")?,
        station: get_str("station").unwrap_or_default(),
        f: Vec::new(),
        bz: Vec::new(),
        region: Vec::new(),
        swarm: Vec::new(),
        tec: Vec::new(),
        champ: Vec::new(),
        env: Vec::new(),
        radon: Vec::new(),
        weather: Vec::new(),
        gps: Vec::new(),
    };
    if let Some(JsonVal::Arr(rows)) = root.get("f") {
        for row in rows {
            if let JsonVal::Arr(cells) = row {
                if let (Some(t), Some(v)) = (
                    cells.first().and_then(scalar_of),
                    cells.get(1).and_then(scalar_of),
                ) {
                    data.f.push((t, v));
                }
            }
        }
    }
    if let Some(JsonVal::Arr(rows)) = root.get("bz") {
        for row in rows {
            if let JsonVal::Arr(cells) = row {
                if let (Some(t), Some(v)) = (
                    cells.first().and_then(scalar_of),
                    cells.get(1).and_then(scalar_of),
                ) {
                    data.bz.push((t, v));
                }
            }
        }
    }
    if let Some(JsonVal::Arr(rows)) = root.get("region") {
        for row in rows {
            if let JsonVal::Arr(cells) = row {
                if let (Some(t), Some(lat), Some(lon), Some(v)) = (
                    cells.first().and_then(scalar_of),
                    cells.get(1).and_then(scalar_of),
                    cells.get(2).and_then(scalar_of),
                    cells.get(3).and_then(scalar_of),
                ) {
                    data.region.push((t, lat, lon, v));
                }
            }
        }
    }
    if let Some(JsonVal::Arr(rows)) = root.get("swarm") {
        for row in rows {
            if let JsonVal::Arr(cells) = row {
                if let (Some(t), Some(lat), Some(lon), Some(v)) = (
                    cells.first().and_then(scalar_of),
                    cells.get(1).and_then(scalar_of),
                    cells.get(2).and_then(scalar_of),
                    cells.get(3).and_then(scalar_of),
                ) {
                    data.swarm.push((t, lat, lon, v));
                }
            }
        }
    }
    if let Some(JsonVal::Arr(rows)) = root.get("tec") {
        for row in rows {
            if let JsonVal::Arr(cells) = row {
                if let (Some(t), Some(v)) = (
                    cells.first().and_then(scalar_of),
                    cells.get(1).and_then(scalar_of),
                ) {
                    data.tec.push((t, v));
                }
            }
        }
    }
    if let Some(JsonVal::Arr(rows)) = root.get("champ") {
        for row in rows {
            if let JsonVal::Arr(cells) = row {
                if let (Some(t), Some(lat), Some(lon), Some(v)) = (
                    cells.first().and_then(scalar_of),
                    cells.get(1).and_then(scalar_of),
                    cells.get(2).and_then(scalar_of),
                    cells.get(3).and_then(scalar_of),
                ) {
                    data.champ.push((t, lat, lon, v));
                }
            }
        }
    }
    if let Some(JsonVal::Arr(rows)) = root.get("env") {
        for row in rows {
            if let JsonVal::Arr(cells) = row {
                if let (Some(t), Some(v)) = (
                    cells.first().and_then(scalar_of),
                    cells.get(1).and_then(scalar_of),
                ) {
                    data.env.push((t, v));
                }
            }
        }
    }
    Some(data)
}

fn load_tec_sidecar(dir: &str, prefix: &str, i: usize) -> Vec<(f64, f64)> {
    let body = match std::fs::read_to_string(format!("{dir}/tec_{prefix}{i:04}.json")) {
        Ok(b) => b,
        Err(_) => return Vec::new(),
    };
    match parse_window_file(&body) {
        Some(d) => d.tec,
        None => Vec::new(),
    }
}

fn load_champ_sidecar(dir: &str, prefix: &str, i: usize) -> Vec<(f64, f64, f64, f64)> {
    let body = match std::fs::read_to_string(format!("{dir}/champ_{prefix}{i:04}.json")) {
        Ok(b) => b,
        Err(_) => return Vec::new(),
    };
    match parse_window_file(&body) {
        Some(d) => d.champ,
        None => Vec::new(),
    }
}

fn load_env_sidecar(dir: &str, prefix: &str, i: usize) -> Vec<(f64, f64)> {
    let body = match std::fs::read_to_string(format!("{dir}/mseed_{prefix}{i:04}.json")) {
        Ok(b) => b,
        Err(_) => return Vec::new(),
    };
    match parse_window_file(&body) {
        Some(d) => d.env,
        None => Vec::new(),
    }
}

fn parse_series_array(body: &str) -> Vec<(f64, f64)> {
    let j = match parse_json(body) {
        Some(v) => v,
        None => return Vec::new(),
    };
    let JsonVal::Arr(rows) = j else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for row in rows {
        if let JsonVal::Arr(cells) = row {
            if let (Some(t), Some(v)) = (
                cells.first().and_then(scalar_of),
                cells.get(1).and_then(scalar_of),
            ) {
                out.push((t, v));
            }
        }
    }
    out
}

fn load_series_sidecar(dir: &str, name: &str, prefix: &str, i: usize) -> Vec<(f64, f64)> {
    let body = match std::fs::read_to_string(format!("{dir}/{name}_{prefix}{i:04}.json")) {
        Ok(b) => b,
        Err(_) => return Vec::new(),
    };
    parse_series_array(&body)
}

fn harvest_window(
    kind: &str,
    t0: f64,
    mag: f64,
    lat: f64,
    lon: f64,
    stations: &[Station],
    do_swarm: bool,
    swarm_sats: &[&str],
) -> WindowData {
    let mut data = WindowData {
        kind: kind.to_string(),
        t0,
        mag,
        lat,
        lon,
        station: String::new(),
        f: Vec::new(),
        bz: Vec::new(),
        region: Vec::new(),
        swarm: Vec::new(),
        tec: Vec::new(),
        champ: Vec::new(),
        env: Vec::new(),
        radon: Vec::new(),
        weather: Vec::new(),
        gps: Vec::new(),
    };
    let t_start = t0 - WINDOW_S;
    let start_iso = unix_to_iso(t_start);
    let stop_iso = unix_to_iso(t0);
    if let Some(station) = nearest_station(lat, lon, stations) {
        data.station = station.id.clone();
        let bgs_url = BGS_HAPI_TEMPLATE
            .replace("{station}", &station.id)
            .replace("{start}", &start_iso)
            .replace("{stop}", &stop_iso);
        if let Some(body) = fetch_text(&bgs_url) {
            data.f = hapi_series(&body, None, |v| {
                v == FILL_BGS_F || v <= 0.0 || v >= F_MAX_NT
            });
        }
    }
    let region_url = FDSN_REGION_TEMPLATE
        .replace("{start}", &start_iso)
        .replace("{stop}", &stop_iso)
        .replace("{lat}", &format!("{lat:.4}"))
        .replace("{lon}", &format!("{lon:.4}"))
        .replace("{radius}", &format!("{HARVEST_RADIUS_KM:.0}"))
        .replace("{mag}", &format!("{M_MIN_REGION:.1}"))
        .replace("{limit}", &FDSN_LIMIT.to_string());
    if let Some(body) = fetch_text(&region_url) {
        data.region = region_records(&body, lat, lon, HARVEST_RADIUS_KM, t0);
    }
    let omni_url = OMNI_HAPI_TEMPLATE
        .replace("{start}", &start_iso)
        .replace("{stop}", &stop_iso);
    if let Some(body) = fetch_text(&omni_url) {
        data.bz = hapi_series(&body, Some("BZ_GSM1800"), |v| v == FILL_OMNI_BZ);
    }
    if do_swarm {
        for sat in swarm_sats {
            let swarm_url = SWARM_HAPI_TEMPLATE
                .replace("{sat}", sat)
                .replace("{start}", &start_iso)
                .replace("{stop}", &stop_iso);
            if let Some(body) = fetch_text(&swarm_url) {
                data.swarm.extend(swarm_pass_samples(&body));
            }
        }
    }
    data
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Some(dir) = arg_value(&args, "--rebuild-manifest") {
        rebuild_manifest(&dir);
        return;
    }
    if let Some(dir) = arg_value(&args, "--compile") {
        let asset = arg_value(&args, "--asset").unwrap_or_else(|| "laic".to_string());
        let ci_mode = args.iter().any(|a| a == "--ci-mode");
        compile_main(&dir, &asset, ci_mode);
        return;
    }
    if arg_value(&args, "--analyze").is_some() {
        analyze_main(&args);
        return;
    }
    harvest_main(&args);
}

const BIN_MAGIC: u32 = 0x4C41_4943;
const BIN_VERSION: u32 = 2;

fn w_u8(v: &mut Vec<u8>, x: u8) {
    v.push(x);
}
fn w_u32(v: &mut Vec<u8>, x: u32) {
    v.extend_from_slice(&x.to_le_bytes());
}
fn w_f64(v: &mut Vec<u8>, x: f64) {
    v.extend_from_slice(&x.to_le_bytes());
}

fn w_window(v: &mut Vec<u8>, group: u8, d: &WindowData) {
    w_u8(v, group);
    w_f64(v, d.t0);
    w_f64(v, d.mag);
    w_f64(v, d.lat);
    w_f64(v, d.lon);
    w_u32(v, d.f.len() as u32);
    for &(t, x) in &d.f {
        w_f64(v, t);
        w_f64(v, x);
    }
    w_u32(v, d.bz.len() as u32);
    for &(t, x) in &d.bz {
        w_f64(v, t);
        w_f64(v, x);
    }
    w_u32(v, d.region.len() as u32);
    for &(t, a, b, x) in &d.region {
        w_f64(v, t);
        w_f64(v, a);
        w_f64(v, b);
        w_f64(v, x);
    }
    w_u32(v, d.tec.len() as u32);
    for &(t, x) in &d.tec {
        w_f64(v, t);
        w_f64(v, x);
    }
    w_u32(v, d.champ.len() as u32);
    for &(t, a, b, x) in &d.champ {
        w_f64(v, t);
        w_f64(v, a);
        w_f64(v, b);
        w_f64(v, x);
    }
    w_u32(v, d.env.len() as u32);
    for &(t, x) in &d.env {
        w_f64(v, t);
        w_f64(v, x);
    }
}

struct Cursor<'a> {
    b: &'a [u8],
    p: usize,
}
impl<'a> Cursor<'a> {
    fn u8(&mut self) -> Option<u8> {
        let x = *self.b.get(self.p)?;
        self.p += 1;
        Some(x)
    }
    fn u32(&mut self) -> Option<u32> {
        let s = self.b.get(self.p..self.p + 4)?;
        self.p += 4;
        Some(u32::from_le_bytes(s.try_into().ok()?))
    }
    fn f64(&mut self) -> Option<f64> {
        let s = self.b.get(self.p..self.p + 8)?;
        self.p += 8;
        Some(f64::from_le_bytes(s.try_into().ok()?))
    }
}

fn r_window(c: &mut Cursor, has_env: bool) -> Option<(u8, WindowData)> {
    let group = c.u8()?;
    let t0 = c.f64()?;
    let mag = c.f64()?;
    let lat = c.f64()?;
    let lon = c.f64()?;
    let mut d = WindowData {
        kind: if group == 0 {
            "event".into()
        } else {
            "null".into()
        },
        t0,
        mag,
        lat,
        lon,
        station: String::new(),
        f: Vec::new(),
        bz: Vec::new(),
        region: Vec::new(),
        swarm: Vec::new(),
        tec: Vec::new(),
        champ: Vec::new(),
        env: Vec::new(),
        radon: Vec::new(),
        weather: Vec::new(),
        gps: Vec::new(),
    };
    let n = c.u32()? as usize;
    for _ in 0..n {
        let t = c.f64()?;
        let x = c.f64()?;
        d.f.push((t, x));
    }
    let n = c.u32()? as usize;
    for _ in 0..n {
        let t = c.f64()?;
        let x = c.f64()?;
        d.bz.push((t, x));
    }
    let n = c.u32()? as usize;
    for _ in 0..n {
        let t = c.f64()?;
        let a = c.f64()?;
        let b = c.f64()?;
        let x = c.f64()?;
        d.region.push((t, a, b, x));
    }
    let n = c.u32()? as usize;
    for _ in 0..n {
        let t = c.f64()?;
        let x = c.f64()?;
        d.tec.push((t, x));
    }
    let n = c.u32()? as usize;
    for _ in 0..n {
        let t = c.f64()?;
        let a = c.f64()?;
        let b = c.f64()?;
        let x = c.f64()?;
        d.champ.push((t, a, b, x));
    }
    if has_env {
        let n = c.u32()? as usize;
        for _ in 0..n {
            let t = c.f64()?;
            let x = c.f64()?;
            d.env.push((t, x));
        }
    }
    Some((group, d))
}

fn pack_bin(windows: &[(u8, WindowData)]) -> Vec<u8> {
    let mut v = Vec::new();
    w_u32(&mut v, BIN_MAGIC);
    w_u32(&mut v, BIN_VERSION);
    w_u32(&mut v, windows.len() as u32);
    for (g, d) in windows {
        w_window(&mut v, *g, d);
    }
    v
}

fn unpack_bin(b: &[u8]) -> Option<Vec<(u8, WindowData)>> {
    let mut c = Cursor { b, p: 0 };
    if c.u32()? != BIN_MAGIC {
        return None;
    }
    let ver = c.u32()?;
    if ver != 1 && ver != 2 {
        return None;
    }
    let n = c.u32()? as usize;
    let mut out = Vec::with_capacity(n.min(100000));
    for _ in 0..n {
        out.push(r_window(&mut c, ver >= 2)?);
    }
    Some(out)
}

fn compile_main(dir: &str, asset: &str, ci_mode: bool) {
    let load = |prefix: &str, i: usize| -> Option<WindowData> {
        let body = std::fs::read_to_string(format!("{dir}/{prefix}{i:04}.json")).ok()?;
        parse_window_file(&body)
    };
    let mut windows: Vec<(u8, WindowData)> = Vec::new();
    let mut ne = 0usize;
    let mut nn = 0usize;
    let mut nt = 0usize;
    let mut nc = 0usize;
    for i in 0..10000 {
        let Some(mut d) = load("e", i) else { break };
        d.tec = load_tec_sidecar(dir, "e", i);
        d.champ = load_champ_sidecar(dir, "e", i);
        windows.push((0, d));
        ne += 1;
    }
    for i in 0..10000 {
        let Some(d) = load("n", i) else { break };
        windows.push((1, d));
        nn += 1;
    }
    for i in 0..10000 {
        let Some(mut d) = load("tcn", i) else { break };
        d.tec = load_tec_sidecar(dir, "tcn", i);
        windows.push((2, d));
        nt += 1;
    }
    for i in 0..10000 {
        let Some(mut d) = load("chn", i) else { break };
        d.champ = load_champ_sidecar(dir, "chn", i);
        windows.push((3, d));
        nc += 1;
    }
    let bytes = pack_bin(&windows);
    let path = format!("{asset}.bin");
    std::fs::write(&path, &bytes).expect("bin write");
    println!(
        "compiled {ne} events, {nn} nulls, {nt} tec-nulls, {nc} champ-nulls → {path} ({} bytes)",
        bytes.len()
    );
    if ci_mode {
        upload_asset(&path);
    }
}

fn rebuild_manifest(dir: &str) {
    let mut events: Vec<(f64, f64, f64, f64)> = Vec::new();
    for i in 0..10000 {
        let body = match std::fs::read_to_string(format!("{dir}/e{i:04}.json")) {
            Ok(b) => b,
            Err(_) => break,
        };
        let Some(d) = parse_window_file(&body) else {
            break;
        };
        events.push((d.t0, d.mag, d.lat, d.lon));
    }
    let mut nulls: Vec<(f64, f64, f64)> = Vec::new();
    for i in 0..10000 {
        let body = match std::fs::read_to_string(format!("{dir}/n{i:04}.json")) {
            Ok(b) => b,
            Err(_) => break,
        };
        let Some(d) = parse_window_file(&body) else {
            break;
        };
        nulls.push((d.t0, d.lat, d.lon));
    }
    let mut manifest = String::from("{\"events\":[");
    for (i, &(t0, mag, lat, lon)) in events.iter().enumerate() {
        if i > 0 {
            manifest.push(',');
        }
        manifest.push_str(&format!("[{t0},{mag},{lat},{lon}]"));
    }
    manifest.push_str("],\"null\":[");
    for (i, &(t0, lat, lon)) in nulls.iter().enumerate() {
        if i > 0 {
            manifest.push(',');
        }
        manifest.push_str(&format!("[{t0},{lat},{lon}]"));
    }
    manifest.push_str("]}");
    std::fs::write(format!("{dir}/manifest.json"), manifest).expect("manifest");
    println!(
        "manifest rebuilt: {} events, {} null windows",
        events.len(),
        nulls.len()
    );
}

fn harvest_main(args: &[String]) {
    let dir = match arg_value(args, "--harvest") {
        Some(d) => d,
        None => {
            println!(
                "usage: laic_probe --harvest DIR [--max-events N] [--null N] [--swarm-limit N] [--swarm-null N] [--mag M] [--era-start YYYY-MM-DD] [--era-end YYYY-MM-DD] [--tec-events N] [--tec-null N] [--tec-era YYYY-MM-DD] [--champ-events N] [--champ-null N] [--mseed-events N] [--mseed-null N] [--radon-events N] [--radon-null N] [--weather-events N] [--weather-null N] [--gps-events N] [--gps-null N]"
            );
            println!(
                "       laic_probe --analyze DIR [--radius KM] [--cell-min MIN] [--kde-scale K] [--max-events N] [--null N] [--bin PATH] [--cdn NAME]"
            );
            println!("       laic_probe --compile DIR [--asset NAME] [--ci-mode]");
            return;
        }
    };
    let max_events = arg_value(args, "--max-events")
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(0);
    let n_null = arg_value(args, "--null")
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(60);
    let swarm_limit = arg_value(args, "--swarm-limit")
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(60);
    let swarm_null = arg_value(args, "--swarm-null")
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(30);
    let mag_min = arg_value(args, "--mag")
        .and_then(|v| v.parse::<f64>().ok())
        .unwrap_or(M_MIN_EVENT);
    let tec_events = arg_value(args, "--tec-events")
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(0);
    let tec_null = arg_value(args, "--tec-null")
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(0);
    let champ_events = arg_value(args, "--champ-events")
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(0);
    let champ_null = arg_value(args, "--champ-null")
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(0);
    let mseed_events = arg_value(args, "--mseed-events")
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(0);
    let mseed_null = arg_value(args, "--mseed-null")
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(0);
    let radon_events = arg_value(args, "--radon-events")
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(0);
    let radon_null = arg_value(args, "--radon-null")
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(0);
    let weather_events = arg_value(args, "--weather-events")
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(0);
    let weather_null = arg_value(args, "--weather-null")
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(0);
    let gps_events = arg_value(args, "--gps-events")
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(0);
    let gps_null = arg_value(args, "--gps-null")
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(0);
    let now_s = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs_f64())
        .unwrap_or(ERA_START_S + 4.0e8);
    let era_start = arg_value(args, "--era-start")
        .and_then(|v| iso_to_unix(&format!("{v}T00:00:00")))
        .unwrap_or(ERA_START_S);
    let era_end = arg_value(args, "--era-end")
        .and_then(|v| iso_to_unix(&format!("{v}T00:00:00")))
        .unwrap_or(now_s);

    println!("=== Nadel-IV harvest: window series to disk (no TE — analysis runs offline) ===");
    println!(
        "events M ≥ {mag_min} in {} … {}, window 72 h before t0, harvest radius {HARVEST_RADIUS_KM:.0} km, swarm sats A+B+C ({}), null windows {}, swarm null {}, dir {dir}",
        unix_to_iso(era_start),
        unix_to_iso(era_end),
        SWARM_SATS.len(),
        n_null,
        swarm_null
    );

    let stations = match fetch_text(BGS_STATIONS_URL) {
        Some(body) => parse_stations_xml(&body),
        None => {
            println!(
                "BGS GetCapabilities carries no response — the station list stays unmeasured (0 honored)"
            );
            return;
        }
    };
    println!("station list: {} INTERMAGNET observatories", stations.len());
    let catalog_url = FDSN_CATALOG_TEMPLATE
        .replace("{start}", &unix_to_iso(era_start))
        .replace("{stop}", &unix_to_iso(era_end))
        .replace("{mag}", &format!("{mag_min:.1}"))
        .replace("{limit}", &FDSN_LIMIT.to_string());
    let mut events = match fetch_text(&catalog_url) {
        Some(body) => catalog_events(&body, mag_min),
        None => Vec::new(),
    };
    events.sort_by(|a, b| b.t0.total_cmp(&a.t0));
    println!("catalog: {} events M ≥ {mag_min}", events.len());
    if max_events > 0 && events.len() > max_events {
        events.truncate(max_events);
        println!("capped: the most recent {max_events} events");
    }
    if events.is_empty() {
        println!("catalog void — nothing to harvest (0 honored)");
        return;
    }
    std::fs::create_dir_all(&dir).expect("harvest dir");

    let mut null_windows: Vec<(f64, f64, f64)> = Vec::new();
    let mut rng = SURROGATE_SEED ^ 0xA5A5_A5A5_A5A5_A5A5;
    for _ in 0..n_null {
        rng = rng
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        let frac = ((rng >> 33) as f64) / (u32::MAX as f64);
        let t0 = era_start + frac * (era_end - era_start - WINDOW_S - 86400.0).max(0.0);
        let center = &events[(rng % events.len() as u64) as usize];
        null_windows.push((t0, center.lat, center.lon));
    }

    let mut manifest = String::from("{\"events\":[");
    for (i, ev) in events.iter().enumerate() {
        let path = format!("{dir}/e{i:04}.json");
        if std::path::Path::new(&path).exists() {
            println!(
                "{i:>4} | {:<19} | M {:.1} | already harvested",
                unix_to_iso(ev.t0),
                ev.mag
            );
        } else {
            let data = harvest_window(
                "event",
                ev.t0,
                ev.mag,
                ev.lat,
                ev.lon,
                &stations,
                i < swarm_limit,
                &SWARM_SATS,
            );
            println!(
                "{i:>4} | {:<19} | M {:.1} | {:>7.2} {:>8.2} | {:<3} | f {} bz {} region {} swarm {}",
                unix_to_iso(ev.t0),
                ev.mag,
                ev.lat,
                ev.lon,
                data.station,
                data.f.len(),
                data.bz.len(),
                data.region.len(),
                data.swarm.len()
            );
            std::fs::write(&path, window_json(&data)).expect("window file");
        }
        if i > 0 {
            manifest.push(',');
        }
        manifest.push_str(&format!("[{},{},{},{}]", ev.t0, ev.mag, ev.lat, ev.lon));
    }
    manifest.push_str("],\"null\":[");
    for (i, &(t0, lat, lon)) in null_windows.iter().enumerate() {
        let path = format!("{dir}/n{i:04}.json");
        if std::path::Path::new(&path).exists() {
            println!("null {i:>4} | {:<19} | already harvested", unix_to_iso(t0));
        } else {
            let data = harvest_window(
                "null",
                t0,
                0.0,
                lat,
                lon,
                &stations,
                i < swarm_null,
                &SWARM_SATS,
            );
            println!(
                "null {i:>4} | {:<19} | {:>7.2} {:>8.2} | {:<3} | f {} bz {} region {} swarm {}",
                unix_to_iso(t0),
                lat,
                lon,
                data.station,
                data.f.len(),
                data.bz.len(),
                data.region.len(),
                data.swarm.len()
            );
            std::fs::write(&path, window_json(&data)).expect("window file");
        }
        if i > 0 {
            manifest.push(',');
        }
        manifest.push_str(&format!("[{},{},{}]", t0, lat, lon));
    }
    manifest.push_str("]}");
    std::fs::write(format!("{dir}/manifest.json"), manifest).expect("manifest");

    let tec_era = arg_value(args, "--tec-era")
        .and_then(|v| iso_to_unix(&format!("{v}T00:00:00")))
        .unwrap_or(1704067200.0);
    let tec_n_events = tec_events.min(events.len());
    if tec_n_events > 0 {
        println!();
        println!(
            "=== TEC-GIM harvest (COD 1-h rapid, ESA GSSC FTP, bilinear at the epicenter, era {}) ===",
            unix_to_iso(tec_era)
        );
        for i in 0..tec_n_events {
            let ev = &events[i];
            if ev.t0 < tec_era {
                break;
            }
            let path = format!("{dir}/tec_e{i:04}.json");
            if std::path::Path::new(&path).exists() {
                println!("tec e{i:>4} | already harvested");
                continue;
            }
            let series = harvest_tec(&dir, ev.t0, ev.lat, ev.lon);
            println!(
                "tec e{i:>4} | {:<19} | M {:.1} | {:>7.2} {:>8.2} | cells {}",
                unix_to_iso(ev.t0),
                ev.mag,
                ev.lat,
                ev.lon,
                series.len()
            );
            let body = format!(
                "{{\"kind\":\"event\",\"t0\":{},\"mag\":{},\"lat\":{},\"lon\":{},\"station\":\"\",\"f\":[],\"bz\":[],\"region\":[],\"swarm\":[],\"tec\":{}}}",
                ev.t0,
                ev.mag,
                ev.lat,
                ev.lon,
                series_json(&series)
            );
            std::fs::write(&path, body).expect("tec sidecar");
        }
    }
    let mut tec_null_windows: Vec<(f64, f64, f64)> = Vec::new();
    let tec_n_null = tec_null;
    if tec_n_null > 0 {
        let mut rng = SURROGATE_SEED ^ 0x7EC7_EC7E_C7EC_7EC7;
        for _ in 0..tec_n_null {
            rng = rng
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            let frac = ((rng >> 33) as f64) / (u32::MAX as f64);
            let t0 = tec_era + frac * (now_s - tec_era - WINDOW_S - 86400.0).max(0.0);
            let center = &events[(rng % events.len() as u64) as usize];
            tec_null_windows.push((t0, center.lat, center.lon));
        }
    }
    let tec_n_null = tec_null_windows.len();
    if tec_n_null > 0 {
        println!();
        println!("=== TEC null ensemble: {tec_n_null} windows drawn from the TEC era ===");
        for i in 0..tec_n_null {
            let (t0, lat, lon) = tec_null_windows[i];
            let wpath = format!("{dir}/tcn{i:04}.json");
            let tpath = format!("{dir}/tec_tcn{i:04}.json");
            if std::path::Path::new(&wpath).exists() && std::path::Path::new(&tpath).exists() {
                println!("tec null {i:>4} | already harvested");
                continue;
            }
            let data = harvest_window("null", t0, 0.0, lat, lon, &stations, false, &SWARM_SATS);
            let series = harvest_tec(&dir, t0, lat, lon);
            println!(
                "tec null {i:>4} | {:<19} | {:>7.2} {:>8.2} | f {} region {} tec {}",
                unix_to_iso(t0),
                lat,
                lon,
                data.f.len(),
                data.region.len(),
                series.len()
            );
            if !std::path::Path::new(&wpath).exists() {
                std::fs::write(&wpath, window_json(&data)).expect("tec null window");
            }
            let body = format!(
                "{{\"kind\":\"null\",\"t0\":{},\"mag\":0.0,\"lat\":{},\"lon\":{},\"station\":\"\",\"f\":[],\"bz\":[],\"region\":[],\"swarm\":[],\"tec\":{}}}",
                t0,
                lat,
                lon,
                series_json(&series)
            );
            std::fs::write(&tpath, body).expect("tec sidecar");
        }
    }
    let champ_n_events = champ_events.min(events.len());
    if champ_n_events > 0 {
        println!();
        println!("=== CHAMP harvest (GFZ-ISDC PLPT density, 15-s, daily zips, anonymous) ===");
        for i in 0..champ_n_events {
            let ev = &events[i];
            let path = format!("{dir}/champ_e{i:04}.json");
            if std::path::Path::new(&path).exists() {
                println!("champ e{i:>4} | already harvested");
                continue;
            }
            let samples = harvest_champ(&dir, ev.t0, ev.lat, ev.lon);
            println!(
                "champ e{i:>4} | {:<19} | M {:.1} | {:>7.2} {:>8.2} | samples {}",
                unix_to_iso(ev.t0),
                ev.mag,
                ev.lat,
                ev.lon,
                samples.len()
            );
            let body = format!(
                "{{\"kind\":\"event\",\"t0\":{},\"mag\":{},\"lat\":{},\"lon\":{},\"station\":\"\",\"f\":[],\"bz\":[],\"region\":[],\"swarm\":[],\"tec\":[],\"champ\":{}}}",
                ev.t0,
                ev.mag,
                ev.lat,
                ev.lon,
                quad_json(&samples)
            );
            std::fs::write(&path, body).expect("champ sidecar");
        }
    }
    let mut champ_null_windows: Vec<(f64, f64, f64)> = Vec::new();
    let champ_n_null = champ_null;
    if champ_n_null > 0 {
        let mut rng = SURROGATE_SEED ^ 0x11A7_11A7_11A7_11A7;
        for _ in 0..champ_n_null {
            rng = rng
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            let frac = ((rng >> 33) as f64) / (u32::MAX as f64);
            let t0 = era_start + frac * (era_end - era_start - WINDOW_S - 86400.0).max(0.0);
            let center = &events[(rng % events.len() as u64) as usize];
            champ_null_windows.push((t0, center.lat, center.lon));
        }
    }
    let champ_n_null = champ_null_windows.len();
    if champ_n_null > 0 {
        println!();
        println!("=== CHAMP null ensemble: {champ_n_null} windows drawn from the era ===");
        for i in 0..champ_n_null {
            let (t0, lat, lon) = champ_null_windows[i];
            let wpath = format!("{dir}/chn{i:04}.json");
            let spath = format!("{dir}/champ_chn{i:04}.json");
            if std::path::Path::new(&wpath).exists() && std::path::Path::new(&spath).exists() {
                println!("champ null {i:>4} | already harvested");
                continue;
            }
            let data = harvest_window("null", t0, 0.0, lat, lon, &stations, false, &SWARM_SATS);
            let samples = harvest_champ(&dir, t0, lat, lon);
            println!(
                "champ null {i:>4} | {:<19} | {:>7.2} {:>8.2} | f {} region {} champ {}",
                unix_to_iso(t0),
                lat,
                lon,
                data.f.len(),
                data.region.len(),
                samples.len()
            );
            if !std::path::Path::new(&wpath).exists() {
                std::fs::write(&wpath, window_json(&data)).expect("champ null window");
            }
            let body = format!(
                "{{\"kind\":\"null\",\"t0\":{},\"mag\":0.0,\"lat\":{},\"lon\":{},\"station\":\"\",\"f\":[],\"bz\":[],\"region\":[],\"swarm\":[],\"tec\":[],\"champ\":{}}}",
                t0,
                lat,
                lon,
                quad_json(&samples)
            );
            std::fs::write(&spath, body).expect("champ sidecar");
        }
    }
    let mseed_n_events = mseed_events.min(events.len());
    if mseed_n_events > 0 {
        println!();
        println!(
            "=== MiniSEED-Envelope harvest (FDSN dataselect, nearest station, RMS per 30-min cell) ==="
        );
        for i in 0..mseed_n_events {
            let ev = &events[i];
            let path = format!("{dir}/mseed_e{i:04}.json");
            if std::path::Path::new(&path).exists() {
                println!("mseed e{i:>4} | already harvested");
                continue;
            }
            let (sta, envelope) = harvest_mseed(&dir, ev.t0, ev.lat, ev.lon);
            println!(
                "mseed e{i:>4} | {:<19} | M {:.1} | {:>7.2} {:>8.2} | {:<3} | cells {}",
                unix_to_iso(ev.t0),
                ev.mag,
                ev.lat,
                ev.lon,
                sta,
                envelope.len()
            );
            let body = format!(
                "{{\"kind\":\"event\",\"t0\":{},\"mag\":{},\"lat\":{},\"lon\":{},\"station\":\"\",\"f\":[],\"bz\":[],\"region\":[],\"swarm\":[],\"tec\":[],\"champ\":[],\"env\":{}}}",
                ev.t0,
                ev.mag,
                ev.lat,
                ev.lon,
                series_json(&envelope)
            );
            std::fs::write(&path, body).expect("mseed sidecar");
        }
    }
    let mseed_n_null = mseed_null.min(null_windows.len());
    if mseed_n_null > 0 {
        for i in 0..mseed_n_null {
            let path = format!("{dir}/mseed_n{i:04}.json");
            if std::path::Path::new(&path).exists() {
                println!("mseed null {i:>4} | already harvested");
                continue;
            }
            let (t0, lat, lon) = null_windows[i];
            let (sta, envelope) = harvest_mseed(&dir, t0, lat, lon);
            println!(
                "mseed null {i:>4} | {:<19} | {:>7.2} {:>8.2} | {:<3} | cells {}",
                unix_to_iso(t0),
                lat,
                lon,
                sta,
                envelope.len()
            );
            let body = format!(
                "{{\"kind\":\"null\",\"t0\":{},\"mag\":0.0,\"lat\":{},\"lon\":{},\"station\":\"\",\"f\":[],\"bz\":[],\"region\":[],\"swarm\":[],\"tec\":[],\"champ\":[],\"env\":{}}}",
                t0,
                lat,
                lon,
                series_json(&envelope)
            );
            std::fs::write(&path, body).expect("mseed sidecar");
        }
    }
    let radon_n_events = radon_events.min(events.len());
    if radon_n_events > 0 {
        println!();
        println!(
            "=== Radon harvest (BfS-ODL gamma dose rate, em, 1-h WFS, 200-km BBOX, CQL time filter) ==="
        );
        for i in 0..radon_n_events {
            let ev = &events[i];
            let path = format!("{dir}/radon_e{i:04}.json");
            if std::path::Path::new(&path).exists() {
                println!("radon e{i:>4} | already harvested");
                continue;
            }
            let series = harvest_radon(&dir, ev.t0, ev.lat, ev.lon);
            println!(
                "radon e{i:>4} | {:<19} | M {:.1} | {:>7.2} {:>8.2} | cells {}",
                unix_to_iso(ev.t0),
                ev.mag,
                ev.lat,
                ev.lon,
                series.len()
            );
            std::fs::write(&path, series_json(&series)).expect("radon sidecar");
        }
    }
    let radon_n_null = radon_null.min(null_windows.len());
    if radon_n_null > 0 {
        for i in 0..radon_n_null {
            let path = format!("{dir}/radon_n{i:04}.json");
            if std::path::Path::new(&path).exists() {
                println!("radon null {i:>4} | already harvested");
                continue;
            }
            let (t0, lat, lon) = null_windows[i];
            let series = harvest_radon(&dir, t0, lat, lon);
            println!(
                "radon null {i:>4} | {:<19} | {:>7.2} {:>8.2} | cells {}",
                unix_to_iso(t0),
                lat,
                lon,
                series.len()
            );
            std::fs::write(&path, series_json(&series)).expect("radon sidecar");
        }
    }
    let weather_n_events = weather_events.min(events.len());
    if weather_n_events > 0 {
        println!();
        println!(
            "=== Weather harvest (NOAA ISD global-hourly SLP, advective, nearest station <= 500 km) ==="
        );
        for i in 0..weather_n_events {
            let ev = &events[i];
            let path = format!("{dir}/weather_e{i:04}.json");
            if std::path::Path::new(&path).exists() {
                println!("weather e{i:>4} | already harvested");
                continue;
            }
            let (id, series) = harvest_weather(&dir, ev.t0, ev.lat, ev.lon);
            println!(
                "weather e{i:>4} | {:<19} | M {:.1} | {:>7.2} {:>8.2} | {:<11} | cells {}",
                unix_to_iso(ev.t0),
                ev.mag,
                ev.lat,
                ev.lon,
                id,
                series.len()
            );
            std::fs::write(&path, series_json(&series)).expect("weather sidecar");
        }
    }
    let weather_n_null = weather_null.min(null_windows.len());
    if weather_n_null > 0 {
        for i in 0..weather_n_null {
            let path = format!("{dir}/weather_n{i:04}.json");
            if std::path::Path::new(&path).exists() {
                println!("weather null {i:>4} | already harvested");
                continue;
            }
            let (t0, lat, lon) = null_windows[i];
            let (id, series) = harvest_weather(&dir, t0, lat, lon);
            println!(
                "weather null {i:>4} | {:<19} | {:>7.2} {:>8.2} | {:<11} | cells {}",
                unix_to_iso(t0),
                lat,
                lon,
                id,
                series.len()
            );
            std::fs::write(&path, series_json(&series)).expect("weather sidecar");
        }
    }
    let gps_n_events = gps_events.min(events.len());
    if gps_n_events > 0 {
        println!();
        println!(
            "=== GPS harvest (NGL tenv3 IGS20 daily east displacement, advective, nearest station <= 200 km) ==="
        );
        for i in 0..gps_n_events {
            let ev = &events[i];
            let path = format!("{dir}/gps_e{i:04}.json");
            if std::path::Path::new(&path).exists() {
                println!("gps e{i:>4} | already harvested");
                continue;
            }
            let (sta, series) = harvest_gps(&dir, ev.t0, ev.lat, ev.lon);
            println!(
                "gps e{i:>4} | {:<19} | M {:.1} | {:>7.2} {:>8.2} | {:<3} | cells {}",
                unix_to_iso(ev.t0),
                ev.mag,
                ev.lat,
                ev.lon,
                sta,
                series.len()
            );
            std::fs::write(&path, series_json(&series)).expect("gps sidecar");
        }
    }
    let gps_n_null = gps_null.min(null_windows.len());
    if gps_n_null > 0 {
        for i in 0..gps_n_null {
            let path = format!("{dir}/gps_n{i:04}.json");
            if std::path::Path::new(&path).exists() {
                println!("gps null {i:>4} | already harvested");
                continue;
            }
            let (t0, lat, lon) = null_windows[i];
            let (sta, series) = harvest_gps(&dir, t0, lat, lon);
            println!(
                "gps null {i:>4} | {:<19} | {:>7.2} {:>8.2} | {:<3} | cells {}",
                unix_to_iso(t0),
                lat,
                lon,
                sta,
                series.len()
            );
            std::fs::write(&path, series_json(&series)).expect("gps sidecar");
        }
    }
    println!("harvest complete. Exit 0.");
}

fn analyze_main(args: &[String]) {
    let dir = match arg_value(args, "--analyze") {
        Some(d) => d,
        None => return,
    };
    let radius_km = arg_value(args, "--radius")
        .and_then(|v| v.parse::<f64>().ok())
        .unwrap_or(HARVEST_RADIUS_KM);
    let cell_min = arg_value(args, "--cell-min")
        .and_then(|v| v.parse::<f64>().ok())
        .unwrap_or(30.0);
    let kde_scale = arg_value(args, "--kde-scale")
        .and_then(|v| v.parse::<f32>().ok())
        .unwrap_or(1.0);
    let max_events = arg_value(args, "--max-events")
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(0);
    let max_null = arg_value(args, "--null")
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(0);
    let bin_path: Option<String> = match arg_value(args, "--cdn") {
        Some(name) => {
            let url = format!("{}/{}/{}.bin", CDN_BASE, CDN_RELEASE, name);
            match fetch_raw_bytes(&url, 86400) {
                Some(bytes) => {
                    let path = format!("/tmp/opencode/{name}.bin");
                    std::fs::write(&path, &bytes).ok();
                    Some(path)
                }
                None => {
                    println!(
                        "CDN asset {name}.bin void — the analysis stays unmeasured (0 honored)"
                    );
                    return;
                }
            }
        }
        None => arg_value(args, "--bin"),
    };
    let cell_s = cell_min * 60.0;
    let n_cells = (WINDOW_S / cell_s) as usize;
    let cells_per_lag = (3600.0 / cell_s).max(1.0) as usize;

    println!(
        "=== Nadel-IV analysis: the LAIC direction, event-centered 72-h windows against the random-window null ensemble ==="
    );
    println!(
        "harvest {dir}; analysis knobs: radius {radius_km:.0} km, cells {cell_min:.0} min (n = {n_cells}), kde scale {kde_scale} (both series scaled → both Silverman bandwidths scaled);"
    );
    println!(
        "TE per window on the scalar path (transfer_entropy_lag), threshold per lag = mean + 2σ of ten phase-randomized surrogates per series; lag sweep 0…{MAX_LAG_H} h in 1-h steps, lags with m < {MIN_M} cells underdetermined;"
    );
    println!(
        "window statistic per direction = max excess over the sweep; stack = mean over event windows; arrow ⇔ stack > null mean + 2σ; the null windows carry the same max-over-lag statistic (structural multiple-comparison correction), a Bonferroni-adjusted threshold is printed per lag;"
    );
    println!(
        "control: TE(Solar Bz → F) on 1-h cells, sweep 0…48 h; the LAIC arrow must carry while the control stays silent;"
    );
    println!(
        "TEC channel (where sidecars exist): COD 1-h rapid GIMs (ESA GSSC FTP, bilinear at the epicenter), TEC pair on 1-h cells, sweep 0…{MAX_LAG_H} h (m ≥ {MIN_M}), control TE(Solar Bz → TEC);"
    );
    println!("registered alternative A — Ereignisrate — remains unbuilt (register).");

    let mut events: Vec<(f64, f64, f64, f64)> = Vec::new();
    let mut null_windows: Vec<(f64, f64, f64)> = Vec::new();
    if bin_path.is_none() {
        let manifest_body = match std::fs::read_to_string(format!("{dir}/manifest.json")) {
            Ok(b) => b,
            Err(_) => {
                println!("manifest void — the harvest directory carries no manifest (0 honored)");
                return;
            }
        };
        let j = match parse_json(&manifest_body) {
            Some(v) => v,
            None => {
                println!("manifest unparsed — the harvest directory carries no readable manifest");
                return;
            }
        };
        let JsonVal::Obj(root) = j else { return };
        if let Some(JsonVal::Arr(rows)) = root.get("events") {
            for row in rows {
                if let JsonVal::Arr(cells) = row {
                    if let (Some(t0), Some(mag), Some(lat), Some(lon)) = (
                        cells.first().and_then(scalar_of),
                        cells.get(1).and_then(scalar_of),
                        cells.get(2).and_then(scalar_of),
                        cells.get(3).and_then(scalar_of),
                    ) {
                        events.push((t0, mag, lat, lon));
                    }
                }
            }
        }
        if let Some(JsonVal::Arr(rows)) = root.get("null") {
            for row in rows {
                if let JsonVal::Arr(cells) = row {
                    if let (Some(t0), Some(lat), Some(lon)) = (
                        cells.first().and_then(scalar_of),
                        cells.get(1).and_then(scalar_of),
                        cells.get(2).and_then(scalar_of),
                    ) {
                        null_windows.push((t0, lat, lon));
                    }
                }
            }
        }
    }
    let mut bin_map: HashMap<(String, usize), WindowData> = HashMap::new();
    if let Some(bp) = &bin_path {
        match std::fs::read(bp).ok().as_deref().and_then(unpack_bin) {
            Some(all) => {
                let mut ei = 0usize;
                let mut ni = 0usize;
                let mut ti = 0usize;
                let mut ci = 0usize;
                events.clear();
                null_windows.clear();
                for (g, d) in all {
                    match g {
                        0 => {
                            events.push((d.t0, d.mag, d.lat, d.lon));
                            bin_map.insert(("e".to_string(), ei), d);
                            ei += 1;
                        }
                        1 => {
                            null_windows.push((d.t0, d.lat, d.lon));
                            bin_map.insert(("n".to_string(), ni), d);
                            ni += 1;
                        }
                        2 => {
                            bin_map.insert(("tcn".to_string(), ti), d);
                            ti += 1;
                        }
                        _ => {
                            bin_map.insert(("chn".to_string(), ci), d);
                            ci += 1;
                        }
                    }
                }
                println!(
                    "bin {bp}: {} events, {} null windows, {} tec-nulls, {} champ-nulls",
                    ei, ni, ti, ci
                );
            }
            None => {
                println!("bin {bp} void or unparsed — the analysis stays unmeasured (0 honored)");
                return;
            }
        }
    }
    if max_events > 0 && events.len() > max_events {
        events.truncate(max_events);
        println!("analysis subset: the most recent {max_events} events");
    }
    if max_null > 0 && null_windows.len() > max_null {
        null_windows.truncate(max_null);
        println!("analysis subset: {max_null} null windows");
    }
    println!(
        "catalog: {} events, {} null windows",
        events.len(),
        null_windows.len()
    );

    let load = |prefix: &str, i: usize| -> Option<WindowData> {
        if bin_path.is_some() {
            return bin_map.get(&(prefix.to_string(), i)).cloned();
        }
        let body = std::fs::read_to_string(format!("{dir}/{prefix}{i:04}.json")).ok()?;
        parse_window_file(&body)
    };

    let mut event_stats: Vec<(usize, WindowStat)> = Vec::new();
    let mut bgs_void = 0usize;
    let mut region_void = 0usize;
    let mut zero_rate = 0usize;
    println!();
    println!("=== event windows (excess = TE − surrogate threshold, max over the lag sweep) ===");
    for (i, &(t0, mag, lat, lon)) in events.iter().enumerate() {
        let mut data = match load("e", i) {
            Some(d) => d,
            None => {
                println!("{i:>4} | {:<19} | window file void", unix_to_iso(t0));
                continue;
            }
        };
        if bin_path.is_none() {
            data.tec = load_tec_sidecar(&dir, "e", i);
            data.champ = load_champ_sidecar(&dir, "e", i);
            data.env = load_env_sidecar(&dir, "e", i);
            data.radon = load_series_sidecar(&dir, "radon", "e", i);
            data.weather = load_series_sidecar(&dir, "weather", "e", i);
            data.gps = load_series_sidecar(&dir, "gps", "e", i);
        }
        let stat = window_stat(&data, radius_km, cell_s, kde_scale);
        if data.station.is_empty() || data.f.is_empty() {
            bgs_void += 1;
        }
        if stat.n_rate_events == 0 {
            zero_rate += 1;
        }
        if stat.excess[0].is_none() && stat.excess[1].is_none() && stat.n_rate_events == 0 {
            region_void += 1;
        }
        println!(
            "{i:>4} | {:<19} | M {:.1} | {:>7.2} {:>8.2} | {:<3} | cells {:<3} rate {} gaps {} | LI {:>+.3e} | IL {:>+.3e} | ctrl {:>+.3e} | fac {:>+.3e}/{:>+.3e} | tec {} {}/{} {} | champ {} {}/{}",
            unix_to_iso(t0),
            mag,
            lat,
            lon,
            data.station,
            stat.n_cells,
            stat.n_rate_events,
            stat.f_gap_cells,
            stat.excess[0].map_or(f64::NAN, |v| v),
            stat.excess[1].map_or(f64::NAN, |v| v),
            stat.control_excess.map_or(f64::NAN, |v| v),
            stat.fac_excess[0].map_or(f64::NAN, |v| v),
            stat.fac_excess[1].map_or(f64::NAN, |v| v),
            stat.tec_n_cells,
            stat.tec_excess[0].map_or("-".to_string(), |v| format!("{:+.3e}", v)),
            stat.tec_excess[1].map_or("-".to_string(), |v| format!("{:+.3e}", v)),
            stat.tec_control_excess
                .map_or("-".to_string(), |v| format!("{:+.3e}", v)),
            stat.champ_n_cells,
            stat.champ_excess[0].map_or("-".to_string(), |v| format!("{:+.3e}", v)),
            stat.champ_excess[1].map_or("-".to_string(), |v| format!("{:+.3e}", v))
        );
        event_stats.push((i, stat));
    }

    println!();
    println!(
        "=== null ensemble: {} random windows ===",
        null_windows.len()
    );
    let mut null_stats: Vec<WindowStat> = Vec::new();
    for (i, &(t0, lat, lon)) in null_windows.iter().enumerate() {
        let mut data = match load("n", i) {
            Some(d) => d,
            None => {
                println!("null {i:>4} | {:<19} | window file void", unix_to_iso(t0));
                continue;
            }
        };
        data.tec = load_tec_sidecar(&dir, "n", i);
        if bin_path.is_none() {
            data.env = load_env_sidecar(&dir, "n", i);
            data.radon = load_series_sidecar(&dir, "radon", "n", i);
            data.weather = load_series_sidecar(&dir, "weather", "n", i);
            data.gps = load_series_sidecar(&dir, "gps", "n", i);
        }
        let stat = window_stat(&data, radius_km, cell_s, kde_scale);
        println!(
            "null {i:>4} | {:<19} | {:>7.2} {:>8.2} | {:<3} | cells {:<3} rate {} | LI {:>+.3e} | IL {:>+.3e} | ctrl {:>+.3e} | tec {} {}/{} {}",
            unix_to_iso(t0),
            lat,
            lon,
            data.station,
            stat.n_cells,
            stat.n_rate_events,
            stat.excess[0].map_or(f64::NAN, |v| v),
            stat.excess[1].map_or(f64::NAN, |v| v),
            stat.control_excess.map_or(f64::NAN, |v| v),
            stat.tec_n_cells,
            stat.tec_excess[0].map_or("-".to_string(), |v| format!("{:+.3e}", v)),
            stat.tec_excess[1].map_or("-".to_string(), |v| format!("{:+.3e}", v)),
            stat.tec_control_excess
                .map_or("-".to_string(), |v| format!("{:+.3e}", v))
        );
        null_stats.push(stat);
    }

    let mut null_stats_tec: Vec<WindowStat> = Vec::new();
    for i in 0..1000 {
        let mut data = match load("tcn", i) {
            Some(d) => d,
            None => break,
        };
        if bin_path.is_none() {
            data.tec = load_tec_sidecar(&dir, "tcn", i);
        }
        let stat = window_stat(&data, radius_km, cell_s, kde_scale);
        println!(
            "tec null {i:>4} | {:<19} | {:>7.2} {:>8.2} | rate {} | tec {} {}/{} {}",
            unix_to_iso(data.t0),
            data.lat,
            data.lon,
            stat.n_rate_events,
            stat.tec_n_cells,
            stat.tec_excess[0].map_or("-".to_string(), |v| format!("{:+.3e}", v)),
            stat.tec_excess[1].map_or("-".to_string(), |v| format!("{:+.3e}", v)),
            stat.tec_control_excess
                .map_or("-".to_string(), |v| format!("{:+.3e}", v))
        );
        null_stats_tec.push(stat);
    }

    let mut null_stats_champ: Vec<WindowStat> = Vec::new();
    for i in 0..1000 {
        let mut data = match load("chn", i) {
            Some(d) => d,
            None => break,
        };
        if bin_path.is_none() {
            data.champ = load_champ_sidecar(&dir, "chn", i);
        }
        let stat = window_stat(&data, radius_km, cell_s, kde_scale);
        println!(
            "champ null {i:>4} | {:<19} | {:>7.2} {:>8.2} | rate {} | champ {} {}/{}",
            unix_to_iso(data.t0),
            data.lat,
            data.lon,
            stat.n_rate_events,
            stat.champ_n_cells,
            stat.champ_excess[0].map_or("-".to_string(), |v| format!("{:+.3e}", v)),
            stat.champ_excess[1].map_or("-".to_string(), |v| format!("{:+.3e}", v))
        );
        null_stats_champ.push(stat);
    }
    let n_lags_main = ((n_cells.saturating_sub(MIN_M)) / cells_per_lag).min(MAX_LAG_H) + 1;
    let z_main = norm_quantile(1.0 - 0.05 / (2.0 * n_lags_main as f64));
    let n_lags_ctrl = (WINDOW_S / 3600.0) as usize - MIN_M + 1;
    let z_ctrl = norm_quantile(1.0 - 0.05 / n_lags_ctrl as f64);

    let ev_li: Vec<f64> = event_stats
        .iter()
        .filter_map(|(_, s)| s.excess[0])
        .collect();
    let ev_il: Vec<f64> = event_stats
        .iter()
        .filter_map(|(_, s)| s.excess[1])
        .collect();
    let ev_ctrl: Vec<f64> = event_stats
        .iter()
        .filter_map(|(_, s)| s.control_excess)
        .collect();
    let ev_fac_li: Vec<f64> = event_stats
        .iter()
        .filter_map(|(_, s)| s.fac_excess[0])
        .collect();
    let ev_fac_il: Vec<f64> = event_stats
        .iter()
        .filter_map(|(_, s)| s.fac_excess[1])
        .collect();
    let nu_li: Vec<f64> = null_stats.iter().filter_map(|s| s.excess[0]).collect();
    let nu_il: Vec<f64> = null_stats.iter().filter_map(|s| s.excess[1]).collect();
    let nu_ctrl: Vec<f64> = null_stats.iter().filter_map(|s| s.control_excess).collect();
    let nu_fac_li: Vec<f64> = null_stats.iter().filter_map(|s| s.fac_excess[0]).collect();
    let nu_fac_il: Vec<f64> = null_stats.iter().filter_map(|s| s.fac_excess[1]).collect();
    let ev_tec_li: Vec<f64> = event_stats
        .iter()
        .filter_map(|(_, s)| s.tec_excess[0])
        .collect();
    let ev_tec_il: Vec<f64> = event_stats
        .iter()
        .filter_map(|(_, s)| s.tec_excess[1])
        .collect();
    let ev_tec_ctrl: Vec<f64> = event_stats
        .iter()
        .filter_map(|(_, s)| s.tec_control_excess)
        .collect();
    let nu_tec_li: Vec<f64> = null_stats_tec
        .iter()
        .filter_map(|s| s.tec_excess[0])
        .collect();
    let nu_tec_il: Vec<f64> = null_stats_tec
        .iter()
        .filter_map(|s| s.tec_excess[1])
        .collect();
    let nu_tec_ctrl: Vec<f64> = null_stats_tec
        .iter()
        .filter_map(|s| s.tec_control_excess)
        .collect();
    let ev_champ_li: Vec<f64> = event_stats
        .iter()
        .filter_map(|(_, s)| s.champ_excess[0])
        .collect();
    let ev_champ_il: Vec<f64> = event_stats
        .iter()
        .filter_map(|(_, s)| s.champ_excess[1])
        .collect();
    let nu_champ_li: Vec<f64> = null_stats_champ
        .iter()
        .filter_map(|s| s.champ_excess[0])
        .collect();
    let nu_champ_il: Vec<f64> = null_stats_champ
        .iter()
        .filter_map(|s| s.champ_excess[1])
        .collect();
    let ev_env_ef: Vec<f64> = event_stats
        .iter()
        .filter_map(|(_, s)| s.env_excess[0])
        .collect();
    let ev_env_fe: Vec<f64> = event_stats
        .iter()
        .filter_map(|(_, s)| s.env_excess[1])
        .collect();
    let nu_env_ef: Vec<f64> = null_stats.iter().filter_map(|s| s.env_excess[0]).collect();
    let nu_env_fe: Vec<f64> = null_stats.iter().filter_map(|s| s.env_excess[1]).collect();
    let ev_weather_f: Vec<f64> = event_stats
        .iter()
        .filter_map(|(_, s)| s.weather_excess[0])
        .collect();
    let ev_f_weather: Vec<f64> = event_stats
        .iter()
        .filter_map(|(_, s)| s.weather_excess[1])
        .collect();
    let nu_weather_f: Vec<f64> = null_stats
        .iter()
        .filter_map(|s| s.weather_excess[0])
        .collect();
    let nu_f_weather: Vec<f64> = null_stats
        .iter()
        .filter_map(|s| s.weather_excess[1])
        .collect();
    let ev_radon_f: Vec<f64> = event_stats
        .iter()
        .filter_map(|(_, s)| s.radon_excess[0])
        .collect();
    let ev_f_radon: Vec<f64> = event_stats
        .iter()
        .filter_map(|(_, s)| s.radon_excess[1])
        .collect();
    let nu_radon_f: Vec<f64> = null_stats
        .iter()
        .filter_map(|s| s.radon_excess[0])
        .collect();
    let nu_f_radon: Vec<f64> = null_stats
        .iter()
        .filter_map(|s| s.radon_excess[1])
        .collect();
    let ev_radon_weather: Vec<f64> = event_stats
        .iter()
        .filter_map(|(_, s)| s.radon_weather_excess[0])
        .collect();
    let ev_weather_radon: Vec<f64> = event_stats
        .iter()
        .filter_map(|(_, s)| s.radon_weather_excess[1])
        .collect();
    let nu_radon_weather: Vec<f64> = null_stats
        .iter()
        .filter_map(|s| s.radon_weather_excess[0])
        .collect();
    let nu_weather_radon: Vec<f64> = null_stats
        .iter()
        .filter_map(|s| s.radon_weather_excess[1])
        .collect();
    let ev_radon_env: Vec<f64> = event_stats
        .iter()
        .filter_map(|(_, s)| s.radon_env_excess[0])
        .collect();
    let ev_env_radon: Vec<f64> = event_stats
        .iter()
        .filter_map(|(_, s)| s.radon_env_excess[1])
        .collect();
    let nu_radon_env: Vec<f64> = null_stats
        .iter()
        .filter_map(|s| s.radon_env_excess[0])
        .collect();
    let nu_env_radon: Vec<f64> = null_stats
        .iter()
        .filter_map(|s| s.radon_env_excess[1])
        .collect();
    let ev_gps_radon: Vec<f64> = event_stats
        .iter()
        .filter_map(|(_, s)| s.gps_radon_excess[0])
        .collect();
    let ev_radon_gps: Vec<f64> = event_stats
        .iter()
        .filter_map(|(_, s)| s.gps_radon_excess[1])
        .collect();
    let nu_gps_radon: Vec<f64> = null_stats
        .iter()
        .filter_map(|s| s.gps_radon_excess[0])
        .collect();
    let nu_radon_gps: Vec<f64> = null_stats
        .iter()
        .filter_map(|s| s.gps_radon_excess[1])
        .collect();

    let s_li = stack_stat(&ev_li);
    let n_li = stack_stat(&nu_li);
    let s_il = stack_stat(&ev_il);
    let n_il = stack_stat(&nu_il);
    let s_ctrl = stack_stat(&ev_ctrl);
    let n_ctrl = stack_stat(&nu_ctrl);
    let s_fac_li = stack_stat(&ev_fac_li);
    let n_fac_li = stack_stat(&nu_fac_li);
    let s_fac_il = stack_stat(&ev_fac_il);
    let n_fac_il = stack_stat(&nu_fac_il);
    let s_tec_li = stack_stat(&ev_tec_li);
    let n_tec_li = stack_stat(&nu_tec_li);
    let s_tec_il = stack_stat(&ev_tec_il);
    let n_tec_il = stack_stat(&nu_tec_il);
    let s_tec_ctrl = stack_stat(&ev_tec_ctrl);
    let n_tec_ctrl = stack_stat(&nu_tec_ctrl);
    let n_lags_tec = (TEC_N_CELLS.saturating_sub(MIN_M)).min(MAX_LAG_H) + 1;
    let z_tec = norm_quantile(1.0 - 0.05 / (2.0 * n_lags_tec as f64));
    let s_champ_li = stack_stat(&ev_champ_li);
    let n_champ_li = stack_stat(&nu_champ_li);
    let s_champ_il = stack_stat(&ev_champ_il);
    let n_champ_il = stack_stat(&nu_champ_il);
    let s_env_ef = stack_stat(&ev_env_ef);
    let n_env_ef = stack_stat(&nu_env_ef);
    let s_env_fe = stack_stat(&ev_env_fe);
    let n_env_fe = stack_stat(&nu_env_fe);
    let s_weather_f = stack_stat(&ev_weather_f);
    let n_weather_f = stack_stat(&nu_weather_f);
    let s_f_weather = stack_stat(&ev_f_weather);
    let n_f_weather = stack_stat(&nu_f_weather);
    let s_radon_f = stack_stat(&ev_radon_f);
    let n_radon_f = stack_stat(&nu_radon_f);
    let s_f_radon = stack_stat(&ev_f_radon);
    let n_f_radon = stack_stat(&nu_f_radon);
    let s_radon_weather = stack_stat(&ev_radon_weather);
    let n_radon_weather = stack_stat(&nu_radon_weather);
    let s_weather_radon = stack_stat(&ev_weather_radon);
    let n_weather_radon = stack_stat(&nu_weather_radon);
    let s_radon_env = stack_stat(&ev_radon_env);
    let n_radon_env = stack_stat(&nu_radon_env);
    let s_env_radon = stack_stat(&ev_env_radon);
    let n_env_radon = stack_stat(&nu_env_radon);
    let s_gps_radon = stack_stat(&ev_gps_radon);
    let n_gps_radon = stack_stat(&nu_gps_radon);
    let s_radon_gps = stack_stat(&ev_radon_gps);
    let n_radon_gps = stack_stat(&nu_radon_gps);

    println!();
    println!("=== stack verdict ===");
    verdict_line("TE(Lithosphere → Ionosphere)", &s_li, &n_li, z_main);
    verdict_line("TE(Ionosphere → Lithosphere)", &s_il, &n_il, z_main);
    verdict_line("TE(Solar Bz → Ionosphere)", &s_ctrl, &n_ctrl, z_ctrl);
    verdict_line("TE(Lithosphere → FAC)", &s_fac_li, &n_fac_li, z_main);
    verdict_line("TE(FAC → Lithosphere)", &s_fac_il, &n_fac_il, z_main);
    verdict_line("TE(Lithosphere → TEC-GIM)", &s_tec_li, &n_tec_li, z_tec);
    verdict_line("TE(TEC-GIM → Lithosphere)", &s_tec_il, &n_tec_il, z_tec);
    verdict_line("TE(Solar Bz → TEC-GIM)", &s_tec_ctrl, &n_tec_ctrl, z_ctrl);
    verdict_line("TE(Envelope → Ionosphere)", &s_env_ef, &n_env_ef, z_main);
    verdict_line("TE(Ionosphere → Envelope)", &s_env_fe, &n_env_fe, z_main);
    verdict_line(
        "TE(Air pressure → Ionosphere)",
        &s_weather_f,
        &n_weather_f,
        z_main,
    );
    verdict_line(
        "TE(Ionosphere → Air pressure)",
        &s_f_weather,
        &n_f_weather,
        z_main,
    );
    verdict_line("TE(Radon → Ionosphere)", &s_radon_f, &n_radon_f, z_main);
    verdict_line("TE(Ionosphere → Radon)", &s_f_radon, &n_f_radon, z_main);
    verdict_line(
        "TE(Radon → Air pressure)",
        &s_radon_weather,
        &n_radon_weather,
        z_main,
    );
    verdict_line(
        "TE(Air pressure → Radon)",
        &s_weather_radon,
        &n_weather_radon,
        z_main,
    );
    verdict_line("TE(Radon → Envelope)", &s_radon_env, &n_radon_env, z_main);
    verdict_line("TE(Envelope → Radon)", &s_env_radon, &n_env_radon, z_main);
    verdict_line("TE(GPS → Radon)", &s_gps_radon, &n_gps_radon, z_main);
    verdict_line("TE(Radon → GPS)", &s_radon_gps, &n_radon_gps, z_main);
    verdict_line(
        "TE(Lithosphere → CHAMP density)",
        &s_champ_li,
        &n_champ_li,
        z_main,
    );
    verdict_line(
        "TE(CHAMP density → Lithosphere)",
        &s_champ_il,
        &n_champ_il,
        z_main,
    );
    println!(
        "voids: {} windows without station or F, {} windows with zero rate events",
        bgs_void, zero_rate
    );
    let _ = region_void;

    let curve_table = |label: &str,
                       curve: &[Vec<Option<f64>>],
                       null_curve: &[Vec<Option<f64>>],
                       z: f64| {
        println!();
        println!("=== per-lag mean excess — {label} ===");
        let n_lags = curve.iter().map(|c| c.len()).max().unwrap_or(0);
        for lag_h in 0..n_lags {
            let evs: Vec<f64> = curve
                .iter()
                .filter_map(|c| c.get(lag_h).copied().flatten())
                .collect();
            let nus: Vec<f64> = null_curve
                .iter()
                .filter_map(|c| c.get(lag_h).copied().flatten())
                .collect();
            if evs.is_empty() && nus.is_empty() {
                continue;
            }
            let (ev_mean, ev_n) = if evs.is_empty() {
                (f64::NAN, 0)
            } else {
                (evs.iter().sum::<f64>() / evs.len() as f64, evs.len())
            };
            let nu = stack_stat(&nus);
            let line = if ev_n == 0 {
                format!(
                    "lag {lag_h:>2} h | event void | null μ {:.4e} σ {:.4e} 2σ {:.4e}",
                    nu.mean,
                    nu.sd,
                    nu.mean + 2.0 * nu.sd
                )
            } else {
                format!(
                    "lag {lag_h:>2} h | event mean {ev_mean:>+.4e} (n {ev_n}) | null μ {:.4e} σ {:.4e} | 2σ thr {:.4e} | bonf thr {:.4e}",
                    nu.mean,
                    nu.sd,
                    nu.mean + 2.0 * nu.sd,
                    nu.mean + z * nu.sd
                )
            };
            println!("{line}");
        }
    };
    let best_lag = |curve: &[Vec<Option<f64>>]| -> (Option<usize>, f64) {
        let mut best: (Option<usize>, f64) = (None, f64::NEG_INFINITY);
        let n_lags = curve.iter().map(|c| c.len()).max().unwrap_or(0);
        for lag_h in 0..n_lags {
            let evs: Vec<f64> = curve
                .iter()
                .filter_map(|c| c.get(lag_h).copied().flatten())
                .collect();
            if evs.is_empty() {
                continue;
            }
            let mean = evs.iter().sum::<f64>() / evs.len() as f64;
            if mean > best.1 {
                best = (Some(lag_h), mean);
            }
        }
        best
    };
    let curve_event: Vec<Vec<Option<f64>>> = event_stats
        .iter()
        .map(|(_, s)| s.curve[0].clone())
        .collect();
    let curve_event_il: Vec<Vec<Option<f64>>> = event_stats
        .iter()
        .map(|(_, s)| s.curve[1].clone())
        .collect();
    let curve_event_ctrl: Vec<Vec<Option<f64>>> = event_stats
        .iter()
        .map(|(_, s)| s.control_curve.clone())
        .collect();
    let curve_null: Vec<Vec<Option<f64>>> = null_stats.iter().map(|s| s.curve[0].clone()).collect();
    let curve_null_il: Vec<Vec<Option<f64>>> =
        null_stats.iter().map(|s| s.curve[1].clone()).collect();
    let curve_null_ctrl: Vec<Vec<Option<f64>>> =
        null_stats.iter().map(|s| s.control_curve.clone()).collect();
    curve_table(
        "Lithosphere → Ionosphere (rate drives F)",
        &curve_event,
        &curve_null,
        z_main,
    );
    curve_table(
        "Ionosphere → Lithosphere (F drives rate)",
        &curve_event_il,
        &curve_null_il,
        z_main,
    );
    curve_table(
        "Solar Bz → Ionosphere (control)",
        &curve_event_ctrl,
        &curve_null_ctrl,
        z_ctrl,
    );
    let lag_li = best_lag(&curve_event);
    let lag_il = best_lag(&curve_event_il);

    let li_arrow = s_li.n >= MIN_N_WINDOWS && s_li.mean > n_li.mean + 2.0 * n_li.sd;
    let il_arrow = s_il.n >= MIN_N_WINDOWS && s_il.mean > n_il.mean + 2.0 * n_il.sd;
    let ctrl_arrow = s_ctrl.n >= MIN_N_WINDOWS && s_ctrl.mean > n_ctrl.mean + 2.0 * n_ctrl.sd;

    println!();
    println!("=== das Blatt ===");
    println!(
        "TE(Lithosphere → Ionosphere) = {:.4e}   (stack mean excess, n = {}, null mean + 2σ = {:.4e})",
        s_li.mean,
        s_li.n,
        n_li.mean + 2.0 * n_li.sd
    );
    println!(
        "TE(Ionosphere → Lithosphere) = {:.4e}   (stack mean excess, n = {}, null mean + 2σ = {:.4e})",
        s_il.mean,
        s_il.n,
        n_il.mean + 2.0 * n_il.sd
    );
    println!(
        "control TE(Solar Bz → Ionosphere) = {:.4e}   (stack mean excess, n = {}, null mean + 2σ = {:.4e})",
        s_ctrl.mean,
        s_ctrl.n,
        n_ctrl.mean + 2.0 * n_ctrl.sd
    );
    println!(
        "TE(Lithosphere → TEC-GIM) = {:.4e}   (stack mean excess, n = {}, null mean + 2σ = {:.4e})",
        s_tec_li.mean,
        s_tec_li.n,
        n_tec_li.mean + 2.0 * n_tec_li.sd
    );
    println!(
        "TE(TEC-GIM → Lithosphere) = {:.4e}   (stack mean excess, n = {}, null mean + 2σ = {:.4e})",
        s_tec_il.mean,
        s_tec_il.n,
        n_tec_il.mean + 2.0 * n_tec_il.sd
    );
    println!(
        "control TE(Solar Bz → TEC-GIM) = {:.4e}   (stack mean excess, n = {}, null mean + 2σ = {:.4e})",
        s_tec_ctrl.mean,
        s_tec_ctrl.n,
        n_tec_ctrl.mean + 2.0 * n_tec_ctrl.sd
    );
    println!(
        "TE(Lithosphere → CHAMP density) = {:.4e}   (stack mean excess, n = {}, null mean + 2σ = {:.4e})",
        s_champ_li.mean,
        s_champ_li.n,
        n_champ_li.mean + 2.0 * n_champ_li.sd
    );
    println!(
        "TE(CHAMP density → Lithosphere) = {:.4e}   (stack mean excess, n = {}, null mean + 2σ = {:.4e})",
        s_champ_il.mean,
        s_champ_il.n,
        n_champ_il.mean + 2.0 * n_champ_il.sd
    );
    println!(
        "TE(Envelope → Ionosphere) = {:.4e}   (stack mean excess, n = {}, null mean + 2σ = {:.4e})",
        s_env_ef.mean,
        s_env_ef.n,
        n_env_ef.mean + 2.0 * n_env_ef.sd
    );
    println!(
        "TE(Ionosphere → Envelope) = {:.4e}   (stack mean excess, n = {}, null mean + 2σ = {:.4e})",
        s_env_fe.mean,
        s_env_fe.n,
        n_env_fe.mean + 2.0 * n_env_fe.sd
    );
    println!(
        "Lag                          = {} (largest mean excess, Litho → Iono; {} for the reverse direction) — sweep 0…{MAX_LAG_H} h in 1-h steps, m ≥ {MIN_M} cells",
        lag_li.0.map_or("pending".to_string(), |l| format!("{l} h")),
        lag_il.0.map_or("pending".to_string(), |l| format!("{l} h"))
    );
    println!(
        "n (events), threshold        = {}, null ensemble mean + 2σ over {} random windows",
        s_li.n,
        null_windows.len()
    );
    if s_li.n < MIN_N_WINDOWS {
        println!(
            "verdict                      = no statement (n = {} < {MIN_N_WINDOWS})",
            s_li.n
        );
    } else if li_arrow && !il_arrow && !ctrl_arrow {
        println!(
            "verdict                      = arrow: Lithosphere → Ionosphere carries, reverse silent, solar control silent"
        );
    } else if li_arrow && ctrl_arrow {
        println!(
            "verdict                      = Lithosphere → Ionosphere carries but the solar control carries too — the common driver is not excluded"
        );
    } else if il_arrow && !li_arrow {
        println!("verdict                      = reverse arrow: Ionosphere → Lithosphere carries");
    } else if li_arrow && il_arrow {
        println!(
            "verdict                      = both directions carry — no dominant causal arrow on this stack"
        );
    } else {
        println!(
            "verdict                      = silent in both directions — a full finding (0 honored)"
        );
    }
    println!(
        "swarm FAC coverage: {} event windows with FAC samples in the radius, {} null windows",
        event_stats
            .iter()
            .filter(|(_, s)| s.swarm_samples.is_some_and(|v| v > 0))
            .count(),
        null_stats
            .iter()
            .filter(|s| s.swarm_samples.is_some_and(|v| v > 0))
            .count()
    );
    println!("Silent lines are findings. Exit 0.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mseed_steim2_decodes_real_record() {
        let bytes = match std::fs::read("/tmp/opencode/mseed_test.mseed") {
            Ok(b) => b,
            Err(_) => {
                eprintln!("mseed test skipped: /tmp/opencode/mseed_test.mseed absent");
                return;
            }
        };
        let mut expected = 0usize;
        let mut pos = 0usize;
        while pos + 64 <= bytes.len() {
            let h = &bytes[pos..pos + 64];
            expected += (h[30] as usize) << 8 | h[31] as usize;
            let rec_len = 1usize << h[54];
            pos += rec_len;
        }
        let samples = parse_mseed(&bytes);
        assert_eq!(
            samples.len(),
            expected,
            "decoded sample count must equal the sum of the per-record header counts"
        );
        assert!(expected > 0, "the file must carry records");
        let t0 = samples[0].0;
        assert!(
            (t0 - 1704067200.0).abs() < 2.0,
            "first sample must start near 2024-01-01T00:00:00, got {t0}"
        );
        for &(_, v) in &samples {
            assert!(v.is_finite(), "decoded sample must be finite");
            assert!(
                v.abs() < 1.0e7,
                "decoded sample out of plausible range: {v}"
            );
        }
    }

    #[test]
    fn bfs_odl_series_reads_synthetic_geojson() {
        let body = r#"{"type":"FeatureCollection","features":[
            {"type":"Feature","properties":{"start_measure":"2024-01-01T00:00:00Z","value":0.12,"unit":"µSv/h"}},
            {"type":"Feature","properties":{"start_measure":"2024-01-01T01:00:00Z","value":0.11,"unit":"µSv/h"}},
            {"type":"Feature","properties":{"start_measure":"2024-01-01T02:00:00Z","value":-1.0,"unit":"µSv/h"}},
            {"type":"Feature","properties":{"start_measure":"2024-01-01T03:00:00Z","value":999.0,"unit":"µSv/h"}}
        ]}"#;
        let t0 = iso_to_unix("2024-01-01T04:00:00").unwrap();
        let t_start = t0 - 259200.0;
        let series = bfs_odl_series(body, t_start, t0);
        assert_eq!(
            series.len(),
            3,
            "the non-positive -1.0 sample is skipped (0 honored)"
        );
        assert!((series[0].1 - 0.12).abs() < 1e-12);
        assert!((series[2].1 - 999.0).abs() < 1e-12);
        assert_eq!(series[0].0, iso_to_unix("2024-01-01T00:00:00").unwrap());
    }

    #[test]
    fn isd_slp_series_parses_quoted_csv() {
        let body = "header\n\
            \"A\",\"2016-06-22T18:24:00\",\"x\",\"x\",\"x\",\"x\",\"x\",\"x\",\"x\",\"x\",\"x\",\"x\",\"x\",\"x\",\"x\",\"10173,1\"\n\
            \"A\",\"2016-06-22T19:24:00\",\"x\",\"x\",\"x\",\"x\",\"x\",\"x\",\"x\",\"x\",\"x\",\"x\",\"x\",\"x\",\"x\",\"99999,9\"\n";
        let t0 = iso_to_unix("2016-06-22T20:00:00").unwrap();
        let series = isd_slp_series(body, t0 - 259200.0, t0);
        assert_eq!(series.len(), 1, "the absent 99999 SLP is skipped (absent)");
        assert!((series[0].1 - 1017.3).abs() < 1e-9);
        assert_eq!(series[0].0, iso_to_unix("2016-06-22T18:24:00").unwrap());
    }

    #[test]
    fn rms_envelope_bins_constant_and_filters_out_of_range() {
        let mut samples = Vec::new();
        for i in 0..10 {
            samples.push((i as f64 * 0.1, 2.0));
        }
        samples.push((99.0, 999.0));
        let env = rms_envelope(&samples, 0.0, 0.1, 10);
        assert_eq!(
            env.len(),
            10,
            "the out-of-range sample is skipped (0 honored)"
        );
        for &(_, v) in &env {
            assert!((v - 2.0).abs() < 1e-12, "constant 2.0 carries RMS 2.0");
        }
    }

    #[test]
    fn tenv3_series_parses_east_and_filters_window() {
        let body = "header\n\
            00NA 08MAR27 2008.2355 54552 1472 4  130.8   4781  0.836816  -1378706 -0.286481   104  0.836530\n\
            00NA 08MAR28 2008.2382 54553 1472 5  130.8   4781  0.837533  -1378706 -0.282454   104  0.844980\n\
            00NA 08MAR29 2008.2409 54554 1472 6  130.8   4781  0.838250  -1378706 -0.278400   104  0.853430\n";
        let t0 = (54554.0 - MJD_UNIX_OFFSET) * 86400.0;
        let series = tenv3_series(body, t0 - 259200.0, t0);
        assert_eq!(series.len(), 3);
        assert!((series[0].1 - 0.836816).abs() < 1e-9);
        assert_eq!(series[0].0, (54552.0 - MJD_UNIX_OFFSET) * 86400.0);
    }

    #[test]
    fn parse_ngl_stations_reads_dataholdings() {
        let body = "header\n\
            00NA -12.4666   130.8440  104.851 -4073662.2759  4712064.7454 -1367874.5096 2008-03-27 2018-09-25 2025-09-25   3185\n\
            01NA -12.4782   130.9820  105.409 -4084823.4607  4702026.6696 -1369125.8893 2008-04-08 2019-09-29 2025-09-25   2360\n";
        let stations = parse_ngl_stations(body);
        assert_eq!(stations.len(), 2);
        assert_eq!(stations[0].id, "00NA");
        assert!((stations[0].lat + 12.4666).abs() < 1e-9);
        assert!((stations[0].lon - 130.8440).abs() < 1e-9);
        assert!(stations[0].end.is_some());
    }

    #[test]
    fn nearest_isd_station_filters_closed_stations() {
        let body = "header\n\
            \"474270\",\"99999\",\"ERIMO\",\"JA\",\"\",\"\",\"+41.830\",\"+142.830\",\"+0063.0\",\"19730101\",\"20020520\"\n\
            \"475800\",\"99999\",\"OPEN\",\"JA\",\"\",\"\",\"+41.917\",\"+143.250\",\"+0063.0\",\"19730101\",\"20261231\"\n";
        let stations = parse_isd_history(&body);
        let t0 = iso_to_unix("2026-08-23T00:00:00").unwrap();
        let sta = nearest_isd_station(&stations, 41.83, 142.83, t0 - 259200.0, t0);
        let Some(sta) = sta else {
            panic!("the open station stays eligible");
        };
        assert_eq!(sta.usaf, "475800", "the closed ERIMO station is skipped");
    }

    #[test]
    fn series_sidecar_round_trip() {
        let series = vec![(1.5, 2.5), (3.5, 4.5)];
        let body = series_json(&series);
        let back = parse_series_array(&body);
        assert_eq!(back.len(), 2);
        assert!((back[0].0 - 1.5).abs() < 1e-12);
        assert!((back[0].1 - 2.5).abs() < 1e-12);
        assert!((back[1].1 - 4.5).abs() < 1e-12);
    }

    #[test]
    fn gim_parser_reads_synthetic_tec_map() {
        let mut body = String::new();
        for map in 1..=2 {
            body.push_str(&format!(
                "     {map}                                                      START OF TEC MAP\n  2024     1     1     {map}     0     0                        EPOCH OF CURRENT MAP\n"
            ));
            for b in 0..71 {
                let lat = 87.5 - 2.5 * b as f64;
                body.push_str(&format!(
                    "  {lat:>6.1}-180.0 180.0   5.0 450.0                            LAT/LON1/LON2/DLON/H\n"
                ));
                let v = 10 * map;
                for line_in_band in 0..5 {
                    let n = if line_in_band < 4 { 16 } else { 9 };
                    for _ in 0..n {
                        body.push_str(&format!("{:>5}", v));
                    }
                    body.push('\n');
                }
            }
        }
        let series = gim_tec_series(&body, 0.0, 0.0);
        assert_eq!(series.len(), 2);
        assert!((series[0].1 - 1.0).abs() < 1e-9);
        assert!((series[1].1 - 2.0).abs() < 1e-9);
        assert_eq!(series[0].0, 1704067200.0 + 3600.0);
    }

    #[test]
    fn bin_round_trip() {
        let d = WindowData {
            kind: "event".to_string(),
            t0: 1704067200.0,
            mag: 6.7,
            lat: -14.64,
            lon: -73.52,
            station: String::new(),
            f: vec![(1.0, 2.5), (3.0, 4.5)],
            bz: vec![(1.0, -3.0)],
            region: vec![(2.0, -14.0, -73.0, 5.2)],
            swarm: Vec::new(),
            tec: vec![(1.0, 15.0), (2.0, 16.0)],
            champ: vec![(1.0, -14.0, -73.0, 153000.0)],
            env: vec![(1.0, 42.0)],
            radon: Vec::new(),
            weather: Vec::new(),
            gps: Vec::new(),
        };
        let windows = vec![
            (0u8, d),
            (
                1u8,
                WindowData {
                    kind: "null".to_string(),
                    t0: 1704070000.0,
                    mag: 0.0,
                    lat: 3.0,
                    lon: 99.0,
                    station: String::new(),
                    f: Vec::new(),
                    bz: Vec::new(),
                    region: Vec::new(),
                    swarm: Vec::new(),
                    tec: Vec::new(),
                    champ: Vec::new(),
                    env: Vec::new(),
                    radon: Vec::new(),
                    weather: Vec::new(),
                    gps: Vec::new(),
                },
            ),
        ];
        let bytes = pack_bin(&windows);
        let back = unpack_bin(&bytes).expect("unpack");
        assert_eq!(back.len(), 2);
        assert_eq!(back[0].0, 0);
        assert_eq!(back[1].0, 1);
        assert_eq!(back[0].1.t0, 1704067200.0);
        assert_eq!(back[0].1.f.len(), 2);
        assert_eq!(back[0].1.tec.len(), 2);
        assert_eq!(back[0].1.champ.len(), 1);
        assert!((back[0].1.champ[0].3 - 153000.0).abs() < 1e-9);
        assert_eq!(back[1].1.kind, "null");
    }

    #[test]
    fn gim_parser_reads_real_day_file_when_present() {
        let bytes = match std::fs::read("/tmp/opencode/gim229.gz") {
            Ok(b) => b,
            Err(_) => {
                eprintln!("gim real-file test skipped: /tmp/opencode/gim229.gz absent");
                return;
            }
        };
        let Some(text) = gunzip(&bytes) else {
            eprintln!("gim real-file test skipped: gunzip void");
            return;
        };
        let text = String::from_utf8_lossy(&text).to_string();
        let series = gim_tec_series(&text, -14.64, -73.52);
        assert!(
            !series.is_empty(),
            "gim_tec_series on the real COD file must yield maps"
        );
        let v = series[0].1;
        assert!(
            v > 0.0 && v < 300.0,
            "TECU value out of plausible range: {}",
            v
        );
    }
}
