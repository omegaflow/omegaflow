use omegaflow::archivar::{
    BodyEphemeris, J2000_EPOCH, NAIF_LSK_EMBEDDED, SourceConfig, VerdictWord,
    body_barycenter_position, download_ephemeris_batch, fetch_raw_bytes, load_sources,
    parse_ephemeris_binary,
};
use omegaflow::cdn::body_url;
use omegaflow::json::{JsonVal, parse_json};
use omegaflow::lsk::{LeapSeconds, days_from_civil, parse as parse_lsk};
use omegaflow::sha256::sha256_hex;
use std::collections::HashMap;

const HOUR: f64 = 3600.0;
const DAY: f64 = 86400.0;
const TUBE_H: f64 = 12.0;
const PERIGEE_YEAR: i64 = 2026;
const PERIGEE_MONTH: i64 = 9;
const PERIGEE_DAY: i64 = 28;
const SEAL_JUICE_SHA256: &str = "aeb3c82ff3de672116ff7f8c28592d97ea05c5e78b8f652521d2cf3cae57488a";
const SEAL_SITE: &str = "docs/paper/flyby-path-2-preregistration.md:21";
const L1_DISTANCE_M: f64 = 1.5e9;
const C_M_S: f64 = 2.99792458e8;
const PROTON_MASS_KG: f64 = 1.67262192369e-27;
const B_MAX_NT: f64 = 1000.0;
const V_MAX_KM_S: f64 = 5000.0;
const T_MAX_K: f64 = 1.0e8;
const F_MAX_NT: f64 = 1.0e5;
const P_MAX_NPA: f64 = 1000.0;
const KP_MAX: f64 = 9.0;
const KP_INTERVAL_H: f64 = 3.0;
const SWARM_LATENCY_S: f64 = 86400.0;
const FILL_B: f64 = 999.9;
const FILL_P: f64 = 99.99;
const WIND_SPEED_WINDOW_S: f64 = 300.0;

const RTSW_MAG_URL: &str = "https://services.swpc.noaa.gov/json/rtsw/rtsw_mag_1m.json";
const RTSW_WIND_URL: &str = "https://services.swpc.noaa.gov/json/rtsw/rtsw_wind_1m.json";
const KP_GFZ_URL: &str = "https://kp.gfz.de/app/json/?start={start}T00:00:00Z&end={end}T00:00:00Z&index=Kp&status=def,nowcast&format=JSON";
const SWARM_HAPI_URL: &str = "https://vires.services/hapi/data?id=SW_FAST_MAGA_LR_1B&start={start}&stop={stop}&parameters=F&format=csv";
const OMNI2_HAPI_URL: &str = "https://cdaweb.gsfc.nasa.gov/hapi/data?id=OMNI2_H0_MRG1HR&time.min={start}T00:00:00Z&time.max={end}T23:59:59Z&parameters=BX_GSE1800,BY_GSM1800,BZ_GSM1800,T1800,N1800,V1800,Pressure1800&format=csv";
const ACE_MAG_URL: &str = "https://services.swpc.noaa.gov/json/ace/mag/ace_mag_1h.json";
const ACE_SWEPAM_URL: &str = "https://services.swpc.noaa.gov/json/ace/swepam/ace_swepam_1h.json";
const TUBE_JSON_PATH: &str = "data/flyby2/tube-juice-2026-09-28.json";

struct RtswMag {
    t: f64,
    bt: Option<f64>,
    bz: Option<f64>,
    source: String,
    active: bool,
}

struct RtswWind {
    t: f64,
    speed: f64,
    density: Option<f64>,
    temperature: Option<f64>,
    source: String,
    active: bool,
}

#[derive(Clone)]
struct Agg {
    sum: f64,
    count: u32,
    sources: Vec<(String, bool)>,
}

impl Agg {
    fn fresh() -> Self {
        Agg {
            sum: 0.0,
            count: 0,
            sources: Vec::new(),
        }
    }

    fn add(&mut self, v: f64, source: &str, active: bool) {
        self.sum += v;
        self.count += 1;
        if !self
            .sources
            .iter()
            .any(|(s, a)| *s == source && *a == active)
        {
            self.sources.push((source.to_string(), active));
        }
    }

    fn value(&self) -> Option<f64> {
        if self.count > 0 {
            Some(self.sum / self.count as f64)
        } else {
            None
        }
    }
}

#[derive(Clone)]
struct PlainAgg {
    sum: f64,
    count: u32,
}

impl PlainAgg {
    fn fresh() -> Self {
        PlainAgg { sum: 0.0, count: 0 }
    }

    fn add(&mut self, v: f64) {
        self.sum += v;
        self.count += 1;
    }

    fn value(&self) -> Option<f64> {
        if self.count > 0 {
            Some(self.sum / self.count as f64)
        } else {
            None
        }
    }
}

#[derive(Clone)]
struct KpCell {
    value: f64,
    status: Option<String>,
}

fn julian_day_utc(y: i64, m: i64, d: i64) -> f64 {
    let a = (14 - m) / 12;
    let yy = y + 4800 - a;
    let mm = m + 12 * a - 3;
    let jdn = d + (153 * mm + 2) / 5 + 365 * yy + yy / 4 - yy / 100 + yy / 400 - 32045;
    jdn as f64 - 0.5
}

fn civil_from_days(z: i64) -> (i64, i64, i64) {
    let z = z + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}

fn date_of(days: i64) -> String {
    let (y, m, d) = civil_from_days(days);
    format!("{y}-{m:02}-{d:02}")
}

fn iso_utc(unix: f64) -> String {
    let total = (unix.max(0.0) / DAY).floor() as i64;
    let day_secs = unix.max(0.0) - total as f64 * DAY;
    let z = total + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    let hh = (day_secs / HOUR) as i64;
    let mm = ((day_secs - hh as f64 * HOUR) / 60.0) as i64;
    let ss = (day_secs - hh as f64 * HOUR - mm as f64 * 60.0) as i64;
    format!("{y:04}-{m:02}-{d:02}T{hh:02}:{mm:02}:{ss:02}Z")
}

fn parse_iso(s: &str) -> Option<f64> {
    let b = s.as_bytes();
    if b.len() < 19 {
        return None;
    }
    let year: i64 = s.get(0..4)?.parse().ok()?;
    let month: i64 = s.get(5..7)?.parse().ok()?;
    let day: i64 = s.get(8..10)?.parse().ok()?;
    let h: i64 = s.get(11..13)?.parse().ok()?;
    let mi: i64 = s.get(14..16)?.parse().ok()?;
    let sec: i64 = s.get(17..19)?.parse().ok()?;
    let days = days_from_civil(year, month, day)?;
    Some(days as f64 * DAY + h as f64 * HOUR + mi as f64 * 60.0 + sec as f64)
}

fn keep_b(v: f64) -> bool {
    v.is_finite() && v.abs() <= B_MAX_NT
}

fn keep_b_fill(v: f64) -> bool {
    v.is_finite() && v != FILL_B && v.abs() <= B_MAX_NT
}

fn keep_positive(v: f64, range: f64) -> bool {
    v.is_finite() && v > 0.0 && v <= range
}

fn keep_positive_fill(v: f64, fill: f64, range: f64) -> bool {
    v.is_finite() && v != fill && v > 0.0 && v <= range
}

fn keep_density(v: f64) -> bool {
    v.is_finite() && v > 0.0
}

fn keep_kp(v: f64) -> bool {
    v.is_finite() && v >= 0.0 && v <= KP_MAX
}

fn obj_str<'a>(v: &'a JsonVal, key: &str) -> Option<&'a str> {
    match v {
        JsonVal::Obj(m) => match m.get(key) {
            Some(JsonVal::Str(s)) => Some(s.as_str()),
            _ => None,
        },
        _ => None,
    }
}

fn obj_num(v: &JsonVal, key: &str) -> Option<f64> {
    match v {
        JsonVal::Obj(m) => match m.get(key) {
            Some(JsonVal::Num(n)) => Some(*n),
            _ => None,
        },
        _ => None,
    }
}

fn obj_bool(v: &JsonVal, key: &str) -> Option<bool> {
    match v {
        JsonVal::Obj(m) => match m.get(key) {
            Some(JsonVal::Bool(b)) => Some(*b),
            _ => None,
        },
        _ => None,
    }
}

fn parse_rtsw_mag(text: &str) -> Vec<RtswMag> {
    let mut out = Vec::new();
    let Some(json) = parse_json(text) else {
        return out;
    };
    let JsonVal::Arr(items) = json else {
        return out;
    };
    for item in &items {
        let Some(t) = obj_str(item, "time_tag").and_then(parse_iso) else {
            continue;
        };
        let bt = obj_num(item, "bt").filter(|v| keep_b(*v));
        let bz = obj_num(item, "bz_gsm").filter(|v| keep_b(*v));
        if bt.is_none() && bz.is_none() {
            continue;
        }
        let source = obj_str(item, "source").unwrap_or("").to_string();
        let active = obj_bool(item, "active").unwrap_or(false);
        out.push(RtswMag {
            t,
            bt,
            bz,
            source,
            active,
        });
    }
    out
}

fn parse_rtsw_wind(text: &str) -> Vec<RtswWind> {
    let mut out = Vec::new();
    let Some(json) = parse_json(text) else {
        return out;
    };
    let JsonVal::Arr(items) = json else {
        return out;
    };
    for item in &items {
        let Some(t) = obj_str(item, "time_tag").and_then(parse_iso) else {
            continue;
        };
        let Some(speed) = obj_num(item, "proton_speed").filter(|v| keep_positive(*v, V_MAX_KM_S))
        else {
            continue;
        };
        let density = obj_num(item, "proton_density").filter(|v| keep_density(*v));
        let temperature =
            obj_num(item, "proton_temperature").filter(|v| keep_positive(*v, T_MAX_K));
        let source = obj_str(item, "source").unwrap_or("").to_string();
        let active = obj_bool(item, "active").unwrap_or(false);
        out.push(RtswWind {
            t,
            speed,
            density,
            temperature,
            source,
            active,
        });
    }
    out
}

fn parse_kp(text: &str) -> Vec<(f64, f64, Option<String>)> {
    let mut out = Vec::new();
    let Some(json) = parse_json(text) else {
        return out;
    };
    let JsonVal::Obj(root) = json else {
        return out;
    };
    let kps: &Vec<JsonVal> = match root.get("Kp") {
        Some(JsonVal::Arr(a)) => a,
        _ => return out,
    };
    let dts: &Vec<JsonVal> = match root.get("datetime") {
        Some(JsonVal::Arr(a)) => a,
        _ => return out,
    };
    let statuses: Option<&Vec<JsonVal>> = match root.get("status") {
        Some(JsonVal::Arr(a)) => Some(a),
        _ => None,
    };
    for (i, k) in kps.iter().enumerate() {
        let Some(JsonVal::Str(ds)) = dts.get(i) else {
            continue;
        };
        let Some(t) = parse_iso(ds) else {
            continue;
        };
        let Some(kp) = match k {
            JsonVal::Num(n) => Some(*n),
            _ => None,
        }
        .filter(|v| keep_kp(*v)) else {
            continue;
        };
        let status = statuses.and_then(|s| s.get(i)).and_then(|v| match v {
            JsonVal::Str(s) => Some(s.clone()),
            _ => None,
        });
        out.push((t, kp, status));
    }
    out
}

fn parse_swarm_csv(text: &str) -> Vec<(f64, f64)> {
    let mut out = Vec::new();
    for line in text.lines() {
        if line.is_empty() || !line.as_bytes()[0].is_ascii_digit() {
            continue;
        }
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() != 2 {
            continue;
        }
        let Some(t) = parse_iso(parts[0]) else {
            continue;
        };
        let Some(f) = parts[1]
            .parse::<f64>()
            .ok()
            .filter(|v| keep_positive(*v, F_MAX_NT))
        else {
            continue;
        };
        out.push((t, f));
    }
    out
}

fn parse_omni2_csv(text: &str) -> (Vec<(f64, f64)>, Vec<(f64, f64)>) {
    let mut bz = Vec::new();
    let mut pressure = Vec::new();
    for line in text.lines() {
        if line.is_empty() || !line.as_bytes()[0].is_ascii_digit() {
            continue;
        }
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() != 8 {
            continue;
        }
        let Some(t) = parse_iso(parts[0]) else {
            continue;
        };
        if let Ok(v) = parts[3].parse::<f64>() {
            if keep_b_fill(v) {
                bz.push((t, v));
            }
        }
        if let Ok(v) = parts[7].parse::<f64>() {
            if keep_positive_fill(v, FILL_P, P_MAX_NPA) {
                pressure.push((t, v));
            }
        }
    }
    (bz, pressure)
}

fn parse_ace_mag(text: &str) -> Vec<(f64, f64)> {
    let mut out = Vec::new();
    let Some(json) = parse_json(text) else {
        return out;
    };
    let JsonVal::Arr(items) = json else {
        return out;
    };
    for item in &items {
        let Some(t) = obj_str(item, "time_tag").and_then(parse_iso) else {
            continue;
        };
        let Some(bt) = obj_num(item, "bt").filter(|v| keep_b(*v)) else {
            continue;
        };
        out.push((t, bt));
    }
    out
}

fn parse_ace_swepam(text: &str) -> (Vec<(f64, f64)>, Vec<(f64, f64)>) {
    let mut speed = Vec::new();
    let mut dens = Vec::new();
    let Some(json) = parse_json(text) else {
        return (speed, dens);
    };
    let JsonVal::Arr(items) = json else {
        return (speed, dens);
    };
    for item in &items {
        let Some(t) = obj_str(item, "time_tag").and_then(parse_iso) else {
            continue;
        };
        if let Some(v) = obj_num(item, "speed").filter(|v| keep_positive(*v, V_MAX_KM_S)) {
            speed.push((t, v));
        }
        if let Some(v) = obj_num(item, "dens").filter(|v| keep_density(*v)) {
            dens.push((t, v));
        }
    }
    (speed, dens)
}

fn transit_lead_s(speed_km_s: f64) -> f64 {
    let v_m_s = speed_km_s * 1000.0;
    L1_DISTANCE_M / v_m_s - L1_DISTANCE_M / C_M_S
}

fn wind_speed_at(wind: &[RtswWind], t: f64) -> Option<f64> {
    let i = wind.partition_point(|w| w.t < t);
    let mut best: Option<(f64, f64)> = None;
    if i < wind.len() {
        let dt = (wind[i].t - t).abs();
        if dt <= WIND_SPEED_WINDOW_S {
            best = Some((dt, wind[i].speed));
        }
    }
    if i > 0 {
        let dt = (wind[i - 1].t - t).abs();
        if dt <= WIND_SPEED_WINDOW_S && best.map_or(true, |(bd, _)| dt < bd) {
            best = Some((dt, wind[i - 1].speed));
        }
    }
    best.map(|(_, v)| v)
}

fn cell_idx(t_arr_unix: f64, t0: f64, n: usize, lsk: &LeapSeconds) -> Option<usize> {
    let t_tdb = lsk.unix_to_tdb(t_arr_unix)?;
    let idx = ((t_tdb - t0) / HOUR).floor();
    if idx < 0.0 || idx >= n as f64 {
        None
    } else {
        Some(idx as usize)
    }
}

fn fill_mag(
    mag: &[RtswMag],
    wind: &[RtswWind],
    t0: f64,
    n: usize,
    lsk: &LeapSeconds,
) -> (Vec<Agg>, Vec<Agg>) {
    let mut bt = vec![Agg::fresh(); n];
    let mut bz = vec![Agg::fresh(); n];
    for m in mag {
        let Some(speed) = wind_speed_at(wind, m.t) else {
            continue;
        };
        let lead = transit_lead_s(speed);
        if !lead.is_finite() || lead < 0.0 {
            continue;
        }
        let Some(i) = cell_idx(m.t + lead, t0, n, lsk) else {
            continue;
        };
        if let Some(v) = m.bt {
            bt[i].add(v, &m.source, m.active);
        }
        if let Some(v) = m.bz {
            bz[i].add(v, &m.source, m.active);
        }
    }
    (bt, bz)
}

fn fill_wind(
    wind: &[RtswWind],
    t0: f64,
    n: usize,
    lsk: &LeapSeconds,
) -> (Vec<Agg>, Vec<Agg>, Vec<Agg>, Vec<Agg>) {
    let mut speed = vec![Agg::fresh(); n];
    let mut density = vec![Agg::fresh(); n];
    let mut temperature = vec![Agg::fresh(); n];
    let mut pressure = vec![Agg::fresh(); n];
    for w in wind {
        let lead = transit_lead_s(w.speed);
        if !lead.is_finite() || lead < 0.0 {
            continue;
        }
        let Some(i) = cell_idx(w.t + lead, t0, n, lsk) else {
            continue;
        };
        speed[i].add(w.speed, &w.source, w.active);
        if let Some(v) = w.density {
            density[i].add(v, &w.source, w.active);
        }
        if let Some(v) = w.temperature {
            temperature[i].add(v, &w.source, w.active);
        }
        if let Some(d) = w.density {
            let v_m_s = w.speed * 1000.0;
            let p_npa = d * 1.0e6 * PROTON_MASS_KG * v_m_s * v_m_s * 1.0e9;
            if p_npa.is_finite() && p_npa > 0.0 {
                pressure[i].add(p_npa, &w.source, w.active);
            }
        }
    }
    (speed, density, temperature, pressure)
}

fn fill_kp(
    rows: &[(f64, f64, Option<String>)],
    t0: f64,
    n: usize,
    lsk: &LeapSeconds,
) -> Vec<Option<KpCell>> {
    let mut cells: Vec<Option<KpCell>> = Vec::with_capacity(n);
    for i in 0..n {
        let Some(t_cell) = lsk.tdb_to_unix(t0 + i as f64 * HOUR) else {
            cells.push(None);
            continue;
        };
        let mut found = None;
        for (t, v, status) in rows {
            if *t > t_cell {
                break;
            }
            if t_cell < *t + KP_INTERVAL_H * HOUR {
                found = Some(KpCell {
                    value: *v,
                    status: status.clone(),
                });
            }
        }
        cells.push(found);
    }
    cells
}

fn fill_plain(rows: &[(f64, f64)], t0: f64, n: usize, lsk: &LeapSeconds) -> Vec<PlainAgg> {
    let mut cells = vec![PlainAgg::fresh(); n];
    for &(t, v) in rows {
        let Some(i) = cell_idx(t, t0, n, lsk) else {
            continue;
        };
        cells[i].add(v);
    }
    cells
}

fn snapshot_files(dir: &str, prefix: &str) -> Vec<String> {
    let mut out = Vec::new();
    let Ok(entries) = std::fs::read_dir(dir) else {
        return out;
    };
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().to_string();
        if name.starts_with(prefix) && name.ends_with(".json") {
            out.push(entry.path().to_string_lossy().to_string());
        }
    }
    out.sort();
    out
}

fn utc_compact(unix: f64) -> String {
    iso_utc(unix)
        .chars()
        .filter(|c| *c != '-' && *c != ':')
        .collect()
}

fn load_rtsw_mag(dir: &Option<String>) -> (Vec<RtswMag>, bool) {
    let mut readings = Vec::new();
    if let Some(d) = dir {
        for path in snapshot_files(d, "rtsw_mag_1m-") {
            if let Ok(bytes) = std::fs::read(&path) {
                readings.extend(parse_rtsw_mag(&String::from_utf8_lossy(&bytes)));
            }
        }
    }
    let mut live_ok = false;
    if let Some(bytes) = fetch_raw_bytes(RTSW_MAG_URL) {
        readings.extend(parse_rtsw_mag(&String::from_utf8_lossy(&bytes)));
        live_ok = true;
        if let Some(d) = dir {
            let _ = std::fs::create_dir_all(d);
            if let Ok(now) = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
                let name = format!("rtsw_mag_1m-{}Z.json", utc_compact(now.as_secs_f64()));
                let _ = std::fs::write(format!("{d}/{name}"), &bytes);
            }
        }
    }
    readings.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap_or(std::cmp::Ordering::Equal));
    readings.dedup_by(|a, b| a.t == b.t);
    (readings, live_ok)
}

fn load_rtsw_wind(dir: &Option<String>) -> (Vec<RtswWind>, bool) {
    let mut readings = Vec::new();
    if let Some(d) = dir {
        for path in snapshot_files(d, "rtsw_wind_1m-") {
            if let Ok(bytes) = std::fs::read(&path) {
                readings.extend(parse_rtsw_wind(&String::from_utf8_lossy(&bytes)));
            }
        }
    }
    let mut live_ok = false;
    if let Some(bytes) = fetch_raw_bytes(RTSW_WIND_URL) {
        readings.extend(parse_rtsw_wind(&String::from_utf8_lossy(&bytes)));
        live_ok = true;
        if let Some(d) = dir {
            let _ = std::fs::create_dir_all(d);
            if let Ok(now) = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
                let name = format!("rtsw_wind_1m-{}Z.json", utc_compact(now.as_secs_f64()));
                let _ = std::fs::write(format!("{d}/{name}"), &bytes);
            }
        }
    }
    readings.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap_or(std::cmp::Ordering::Equal));
    readings.dedup_by(|a, b| a.t == b.t);
    (readings, live_ok)
}

fn coverage(eph: &BodyEphemeris) -> (f64, f64) {
    let mut lo = f64::INFINITY;
    let mut hi = f64::NEG_INFINITY;
    for g in &eph.granules {
        lo = lo.min(g.t0_jd - g.dt_jd);
        hi = hi.max(g.t0_jd + g.dt_jd);
    }
    (lo, hi)
}

fn find_perigee(name: &str, jd0: f64, eph: &HashMap<String, BodyEphemeris>) -> Option<(f64, f64)> {
    let arc = eph.get(name)?;
    let earth = eph.get("earth")?;
    let scan_lo = jd0 - 1.5;
    let scan_hi = jd0 + 1.5;
    let (arc_lo, arc_hi) = coverage(arc);
    let (e_lo, e_hi) = coverage(earth);
    if scan_lo < arc_lo.max(e_lo) || scan_hi > arc_hi.min(e_hi) {
        eprintln!("  {name}: perigee scan window outside arc or earth coverage");
        return None;
    }
    let mut best_jd = 0.0f64;
    let mut best_d = f64::INFINITY;
    let step_jd = 5.0 / 1440.0;
    let mut jd = scan_lo;
    while jd <= scan_hi {
        let tdb = (jd - J2000_EPOCH) * DAY;
        let Some(p) = body_barycenter_position(name, tdb, eph) else {
            jd += step_jd;
            continue;
        };
        let Some(q) = body_barycenter_position("earth", tdb, eph) else {
            jd += step_jd;
            continue;
        };
        let dx = p[0] - q[0];
        let dy = p[1] - q[1];
        let dz = p[2] - q[2];
        let d = (dx * dx + dy * dy + dz * dz).sqrt();
        if d < best_d {
            best_d = d;
            best_jd = jd;
        }
        jd += step_jd;
    }
    if best_d.is_finite() {
        Some((best_jd, best_d))
    } else {
        None
    }
}

fn j_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            c if c.is_control() => {
                for b in c.escape_default() {
                    out.push(b as char);
                }
            }
            c => out.push(c),
        }
    }
    out
}

fn f64_json(v: f64) -> String {
    format!("{v}")
}

fn agg_json(a: &Agg) -> String {
    match a.value() {
        None => "\"pending\"".to_string(),
        Some(v) => {
            let mut s = format!("{{\"value\": {}, \"count\": {}", f64_json(v), a.count);
            if !a.sources.is_empty() {
                s.push_str(", \"sources\": [");
                for (i, (src, act)) in a.sources.iter().enumerate() {
                    if i > 0 {
                        s.push_str(", ");
                    }
                    s.push_str(&format!(
                        "{{\"source\": \"{}\", \"active\": {}}}",
                        j_escape(src),
                        act
                    ));
                }
                s.push(']');
            }
            s.push('}');
            s
        }
    }
}

fn plain_json(a: &PlainAgg) -> String {
    match a.value() {
        None => "\"pending\"".to_string(),
        Some(v) => format!("{{\"value\": {}}}", f64_json(v)),
    }
}

fn kp_json(c: &Option<KpCell>) -> String {
    match c {
        None => "\"pending\"".to_string(),
        Some(k) => match &k.status {
            None => format!("{{\"value\": {}}}", f64_json(k.value)),
            Some(s) => format!(
                "{{\"value\": {}, \"status\": \"{}\"}}",
                f64_json(k.value),
                j_escape(s)
            ),
        },
    }
}

fn cell_utc(t_tdb: f64, lsk: &LeapSeconds) -> Option<String> {
    lsk.tdb_to_unix(t_tdb).map(iso_utc)
}

fn cell_utc_label(t_tdb: f64, lsk: &LeapSeconds) -> String {
    match cell_utc(t_tdb, lsk) {
        Some(s) => s,
        None => "pending".to_string(),
    }
}

fn riss_json(measured_sha: &str, measured_bytes: usize) -> String {
    let mut s = String::new();
    s.push_str("{\n");
    s.push_str("  \"flyby\": \"juice\",\n");
    s.push_str("  \"perigee_date\": \"2026-09-28\",\n");
    s.push_str(&format!(
        "  \"trajectory\": {{\"verdict\": \"{}\", \"seal_sha256\": \"{}\", \"measured_sha256\": \"{}\", \"measured_bytes\": {}}},\n",
        VerdictWord::Riss.word(),
        SEAL_JUICE_SHA256,
        measured_sha,
        measured_bytes
    ));
    s.push_str(&format!("  \"seal_site\": \"{}\",\n", SEAL_SITE));
    s.push_str("  \"tube\": \"no tube built from the unsealed arc\"\n");
    s.push_str("}\n");
    s
}

fn tube_json(
    verdict: VerdictWord,
    measured_sha: &str,
    measured_bytes: usize,
    t0: f64,
    n_hours: usize,
    t_p: f64,
    perigee_utc: &str,
    perigee_km: f64,
    channels: &[Vec<String>],
    lsk: &LeapSeconds,
) -> String {
    let mut s = String::new();
    s.push_str("{\n");
    s.push_str("  \"flyby\": \"juice\",\n");
    s.push_str("  \"perigee_date\": \"2026-09-28\",\n");
    s.push_str(&format!(
        "  \"trajectory\": {{\"verdict\": \"{}\", \"seal_sha256\": \"{}\", \"measured_sha256\": \"{}\", \"measured_bytes\": {}}},\n",
        verdict.word(),
        SEAL_JUICE_SHA256,
        measured_sha,
        measured_bytes
    ));
    s.push_str(&format!("  \"seal_site\": \"{}\",\n", SEAL_SITE));
    s.push_str("  \"tube\": {\n");
    s.push_str(&format!("    \"t0_tdb\": {},\n", f64_json(t0)));
    s.push_str(&format!("    \"n_hours\": {},\n", n_hours));
    s.push_str(&format!("    \"perigee_tdb\": {},\n", f64_json(t_p)));
    s.push_str(&format!(
        "    \"perigee_utc\": \"{}\",\n",
        j_escape(perigee_utc)
    ));
    s.push_str(&format!(
        "    \"perigee_distance_km\": {},\n",
        f64_json(perigee_km)
    ));
    s.push_str(&format!(
        "    \"transit\": {{\"l1_distance_m\": {}, \"c_m_s\": {}}},\n",
        f64_json(L1_DISTANCE_M),
        f64_json(C_M_S)
    ));
    s.push_str("    \"cells\": [\n");
    for (i, cell_channels) in channels.iter().enumerate() {
        let t_i = t0 + i as f64 * HOUR;
        s.push_str("      {\n");
        s.push_str(&format!("        \"index\": {},\n", i));
        s.push_str(&format!("        \"t_tdb\": {},\n", f64_json(t_i)));
        s.push_str(&format!(
            "        \"t_utc\": \"{}\",\n",
            j_escape(&cell_utc_label(t_i, lsk))
        ));
        s.push_str("        \"channels\": {\n");
        for (ci, fragment) in cell_channels.iter().enumerate() {
            let name = channel_name(ci);
            s.push_str(&format!(
                "          \"{}\": {}{}\n",
                name,
                fragment,
                if ci + 1 < cell_channels.len() {
                    ","
                } else {
                    ""
                }
            ));
        }
        s.push_str("        }\n");
        s.push_str(&format!(
            "      }}{}\n",
            if i + 1 < channels.len() { "," } else { "" }
        ));
    }
    s.push_str("    ]\n");
    s.push_str("  }\n");
    s.push_str("}\n");
    s
}

fn channel_name(ci: usize) -> &'static str {
    const NAMES: [&str; 13] = [
        "rtsw_bt",
        "rtsw_bz",
        "rtsw_speed",
        "rtsw_density",
        "rtsw_temperature",
        "rtsw_pressure",
        "kp",
        "swarm_f",
        "omni2_pressure",
        "omni2_bz",
        "ace_bt",
        "ace_speed",
        "ace_density",
    ];
    NAMES[ci]
}

fn table_value(v: Option<f64>, width: usize, precision: usize) -> String {
    match v {
        None => "pending".to_string(),
        Some(x) => format!(
            "{x:>width$.precision$}",
            width = width,
            precision = precision
        ),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let flyby = args
        .iter()
        .position(|a| a == "--flyby")
        .and_then(|i| args.get(i + 1))
        .cloned();
    let snapshots = args
        .iter()
        .position(|a| a == "--snapshots")
        .and_then(|i| args.get(i + 1))
        .cloned();
    if flyby.as_deref() != Some("juice") {
        eprintln!(
            "flyby_path2_fill: --flyby juice names the only sealed window (JUICE Earth flyby 2026-09-28)"
        );
        std::process::exit(2);
    }
    let lsk = match parse_lsk(NAIF_LSK_EMBEDDED) {
        Some(l) => l,
        None => {
            eprintln!("embedded leap table parses void");
            std::process::exit(2);
        }
    };

    let arc_path = "data/ssd.jpl.nasa.gov/ephemeris_juice.bin".to_string();
    let mut bytes = std::fs::read(&arc_path).ok();
    if bytes.is_none() {
        std::fs::create_dir_all("data/ssd.jpl.nasa.gov").ok();
        bytes = fetch_raw_bytes(&body_url("juice"));
        if let Some(b) = &bytes {
            std::fs::write(&arc_path, b).ok();
        }
    }
    let Some(bytes) = bytes else {
        eprintln!(
            "juice arc: local and CDN carry no bin ({} and {})",
            arc_path,
            body_url("juice")
        );
        std::process::exit(2);
    };
    let measured_sha = sha256_hex(&bytes);
    let verdict: VerdictWord = if measured_sha == SEAL_JUICE_SHA256 {
        VerdictWord::Placed
    } else {
        VerdictWord::Riss
    };
    println!(
        "trajectory: {} sha256 {} ({} B)",
        verdict.word(),
        measured_sha,
        bytes.len()
    );
    if verdict == VerdictWord::Riss {
        println!(
            "trajectory riss: measured sha256 {} ({} B) != seal sha256 {} ({})",
            measured_sha,
            bytes.len(),
            SEAL_JUICE_SHA256,
            SEAL_SITE
        );
        println!(
            "no tube built from the unsealed arc — the riss stands with both witnesses (verdict {})",
            verdict.word()
        );
        std::fs::create_dir_all("data/flyby2").ok();
        if std::fs::write(TUBE_JSON_PATH, riss_json(&measured_sha, bytes.len())).is_ok() {
            println!("riss register: {TUBE_JSON_PATH}");
        }
        return;
    }

    let Some(juice) = parse_ephemeris_binary(&bytes) else {
        eprintln!("juice arc: bin parse void");
        std::process::exit(2);
    };
    let sources = load_sources();
    let items: Vec<(usize, SourceConfig, String)> = sources
        .iter()
        .enumerate()
        .filter(|(_, s)| s.format == "ephemeris_binary" && s.body.as_deref() == Some("earth"))
        .map(|(idx, s)| {
            (
                idx,
                s.clone(),
                "data/ssd.jpl.nasa.gov/ephemeris_earth.bin".to_string(),
            )
        })
        .collect();
    download_ephemeris_batch(&items);
    let Some(earth) = items.iter().find_map(|(_, _, p)| {
        std::fs::read(p)
            .ok()
            .and_then(|d| parse_ephemeris_binary(&d))
    }) else {
        eprintln!("earth arc: bin parse void");
        std::process::exit(2);
    };
    let mut eph: HashMap<String, BodyEphemeris> = HashMap::new();
    eph.insert("juice".to_string(), juice);
    eph.insert("earth".to_string(), earth);
    let jd0 = julian_day_utc(PERIGEE_YEAR, PERIGEE_MONTH, PERIGEE_DAY);
    let Some((t_p_jd, perigee_m)) = find_perigee("juice", jd0, &eph) else {
        eprintln!("perigee scan void — the tube stays unbuilt");
        std::process::exit(2);
    };
    let t_p = (t_p_jd - J2000_EPOCH) * DAY;
    let perigee_utc = cell_utc_label(t_p, &lsk);
    println!(
        "perigee {} geocentric {:.0} km",
        perigee_utc,
        perigee_m / 1000.0
    );

    let t_lo = t_p - TUBE_H * HOUR;
    let t_hi = t_p + TUBE_H * HOUR;
    let t0 = (t_lo / HOUR).floor() * HOUR;
    let n_hours = ((t_hi - t0) / HOUR).round() as usize + 1;
    let i_p = ((t_p - t0) / HOUR).round() as usize;
    println!("tube ±{TUBE_H:.0} h: {n_hours} hourly cells, perigee at cell {i_p}");

    let (mag, mag_live) = load_rtsw_mag(&snapshots);
    let (wind, wind_live) = load_rtsw_wind(&snapshots);
    println!(
        "rtsw: {} mag / {} wind readings (live top-up ok: {mag_live}/{wind_live})",
        mag.len(),
        wind.len()
    );
    let (bt_cells, bz_cells) = fill_mag(&mag, &wind, t0, n_hours, &lsk);
    let (speed_cells, density_cells, temp_cells, pressure_cells) =
        fill_wind(&wind, t0, n_hours, &lsk);

    let kp_cells = match (
        lsk.tdb_to_unix(t0),
        lsk.tdb_to_unix(t0 + (n_hours - 1) as f64 * HOUR),
    ) {
        (Some(u_lo), Some(u_hi)) => {
            let start = date_of((u_lo / DAY).floor() as i64 - 1);
            let end = date_of((u_hi / DAY).ceil() as i64 + 1);
            let url = KP_GFZ_URL.replace("{start}", &start).replace("{end}", &end);
            let mut rows = match fetch_raw_bytes(&url) {
                Some(bytes) => parse_kp(&String::from_utf8_lossy(&bytes)),
                None => Vec::new(),
            };
            rows.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
            fill_kp(&rows, t0, n_hours, &lsk)
        }
        _ => {
            eprintln!("kp: tube window maps void in the leap table — kp stays pending");
            vec![None; n_hours]
        }
    };

    let swarm_cells = match (
        lsk.tdb_to_unix(t0),
        lsk.tdb_to_unix(t0 + n_hours as f64 * HOUR),
    ) {
        (Some(u_lo), Some(u_hi)) => {
            let rows = match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
                Ok(now) => {
                    let u_stop = u_hi.min(now.as_secs_f64() - SWARM_LATENCY_S);
                    if u_lo <= u_stop {
                        let url = SWARM_HAPI_URL
                            .replace("{start}", &iso_utc(u_lo))
                            .replace("{stop}", &iso_utc(u_stop));
                        match fetch_raw_bytes(&url) {
                            Some(bytes) => parse_swarm_csv(&String::from_utf8_lossy(&bytes)),
                            None => Vec::new(),
                        }
                    } else {
                        Vec::new()
                    }
                }
                Err(_) => Vec::new(),
            };
            fill_plain(&rows, t0, n_hours, &lsk)
        }
        _ => vec![PlainAgg::fresh(); n_hours],
    };

    let (omni_p_cells, omni_bz_cells) = match (
        lsk.tdb_to_unix(t0),
        lsk.tdb_to_unix(t0 + (n_hours - 1) as f64 * HOUR),
    ) {
        (Some(u_lo), Some(u_hi)) => {
            let start = date_of((u_lo / DAY).floor() as i64 - 1);
            let end = date_of((u_hi / DAY).ceil() as i64 + 1);
            let url = OMNI2_HAPI_URL
                .replace("{start}", &start)
                .replace("{end}", &end);
            let (bz_rows, p_rows) = match fetch_raw_bytes(&url) {
                Some(bytes) => parse_omni2_csv(&String::from_utf8_lossy(&bytes)),
                None => (Vec::new(), Vec::new()),
            };
            (
                fill_plain(&p_rows, t0, n_hours, &lsk),
                fill_plain(&bz_rows, t0, n_hours, &lsk),
            )
        }
        _ => (
            vec![PlainAgg::fresh(); n_hours],
            vec![PlainAgg::fresh(); n_hours],
        ),
    };

    let ace_bt_cells = match fetch_raw_bytes(ACE_MAG_URL) {
        Some(bytes) => fill_plain(
            &parse_ace_mag(&String::from_utf8_lossy(&bytes)),
            t0,
            n_hours,
            &lsk,
        ),
        None => vec![PlainAgg::fresh(); n_hours],
    };
    let (ace_speed_cells, ace_dens_cells) = match fetch_raw_bytes(ACE_SWEPAM_URL) {
        Some(bytes) => {
            let (speed_rows, dens_rows) = parse_ace_swepam(&String::from_utf8_lossy(&bytes));
            (
                fill_plain(&speed_rows, t0, n_hours, &lsk),
                fill_plain(&dens_rows, t0, n_hours, &lsk),
            )
        }
        None => (
            vec![PlainAgg::fresh(); n_hours],
            vec![PlainAgg::fresh(); n_hours],
        ),
    };

    let mut cell_channels: Vec<Vec<String>> = Vec::with_capacity(n_hours);
    for i in 0..n_hours {
        let fragments = vec![
            agg_json(&bt_cells[i]),
            agg_json(&bz_cells[i]),
            agg_json(&speed_cells[i]),
            agg_json(&density_cells[i]),
            agg_json(&temp_cells[i]),
            agg_json(&pressure_cells[i]),
            kp_json(&kp_cells[i]),
            plain_json(&swarm_cells[i]),
            plain_json(&omni_p_cells[i]),
            plain_json(&omni_bz_cells[i]),
            plain_json(&ace_bt_cells[i]),
            plain_json(&ace_speed_cells[i]),
            plain_json(&ace_dens_cells[i]),
        ];
        cell_channels.push(fragments);
    }

    std::fs::create_dir_all("data/flyby2").ok();
    let json = tube_json(
        verdict,
        &measured_sha,
        bytes.len(),
        t0,
        n_hours,
        t_p,
        &perigee_utc,
        perigee_m / 1000.0,
        &cell_channels,
        &lsk,
    );
    if std::fs::write(TUBE_JSON_PATH, json).is_ok() {
        println!("tube register: {TUBE_JSON_PATH}");
    } else {
        eprintln!("tube register write void: {TUBE_JSON_PATH}");
        std::process::exit(2);
    }

    println!(
        "cell t_utc (perigee cell {i_p}) | bt nT | bz nT | v km/s | n 1/cm3 | T K | p nPa | kp | swarm F nT | omni2 p nPa | omni2 bz nT | ace bt nT | ace v km/s | ace n 1/cm3"
    );
    for i in 0..n_hours {
        let t_utc = cell_utc_label(t0 + i as f64 * HOUR, &lsk);
        println!(
            "{:>2} {}{} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {}",
            i,
            t_utc,
            if i == i_p { " (perigee)" } else { "" },
            table_value(bt_cells[i].value(), 7, 2),
            table_value(bz_cells[i].value(), 7, 2),
            table_value(speed_cells[i].value(), 7, 1),
            table_value(density_cells[i].value(), 7, 2),
            table_value(temp_cells[i].value(), 7, 0),
            table_value(pressure_cells[i].value(), 7, 2),
            table_value(kp_cells[i].as_ref().map(|k| k.value), 7, 2),
            table_value(swarm_cells[i].value(), 7, 0),
            table_value(omni_p_cells[i].value(), 7, 2),
            table_value(omni_bz_cells[i].value(), 7, 2),
            table_value(ace_bt_cells[i].value(), 7, 2),
            table_value(ace_speed_cells[i].value(), 7, 0),
            table_value(ace_dens_cells[i].value(), 7, 3),
        );
    }
}
