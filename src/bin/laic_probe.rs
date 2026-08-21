use omegaflow::archivar::{fetch_raw, parse_json, scalar_of, ymd_to_days, JsonVal};
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
    Some(data)
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
    if arg_value(&args, "--analyze").is_some() {
        analyze_main(&args);
        return;
    }
    harvest_main(&args);
}

fn harvest_main(args: &[String]) {
    let dir = match arg_value(args, "--harvest") {
        Some(d) => d,
        None => {
            println!("usage: laic_probe --harvest DIR [--max-events N] [--null N] [--swarm-limit N] [--swarm-null N] [--mag M]");
            println!("       laic_probe --analyze DIR [--radius KM] [--cell-min MIN] [--kde-scale K] [--max-events N] [--null N]");
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
    let now_s = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs_f64())
        .unwrap_or(ERA_START_S + 4.0e8);

    println!("=== Nadel-IV harvest: window series to disk (no TE — analysis runs offline) ===");
    println!(
        "events M ≥ {mag_min} since {}, window 72 h before t0, harvest radius {HARVEST_RADIUS_KM:.0} km, swarm sats A+B+C ({}), null windows {}, swarm null {}, dir {dir}",
        unix_to_iso(ERA_START_S),
        SWARM_SATS.len(),
        n_null,
        swarm_null
    );

    let stations = match fetch_text(BGS_STATIONS_URL) {
        Some(body) => parse_stations_xml(&body),
        None => {
            println!("BGS GetCapabilities carries no response — the station list stays unmeasured (0 honored)");
            return;
        }
    };
    println!("station list: {} INTERMAGNET observatories", stations.len());
    let catalog_url = FDSN_CATALOG_TEMPLATE
        .replace("{start}", &unix_to_iso(ERA_START_S))
        .replace("{stop}", &unix_to_iso(now_s))
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
        let t0 = ERA_START_S + frac * (now_s - ERA_START_S - WINDOW_S - 86400.0);
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
    let cell_s = cell_min * 60.0;
    let n_cells = (WINDOW_S / cell_s) as usize;
    let cells_per_lag = (3600.0 / cell_s).max(1.0) as usize;

    println!("=== Nadel-IV analysis: the LAIC direction, event-centered 72-h windows against the random-window null ensemble ===");
    println!(
        "harvest {dir}; analysis knobs: radius {radius_km:.0} km, cells {cell_min:.0} min (n = {n_cells}), kde scale {kde_scale} (both series scaled → both Silverman bandwidths scaled);"
    );
    println!(
        "TE per window on the scalar path (transfer_entropy_lag), threshold per lag = mean + 2σ of ten phase-randomized surrogates per series; lag sweep 0…{MAX_LAG_H} h in 1-h steps, lags with m < {MIN_M} cells underdetermined;"
    );
    println!(
        "window statistic per direction = max excess over the sweep; stack = mean over event windows; arrow ⇔ stack > null mean + 2σ; the null windows carry the same max-over-lag statistic (structural multiple-comparison correction), a Bonferroni-adjusted threshold is printed per lag;"
    );
    println!("control: TE(Solar Bz → F) on 1-h cells, sweep 0…48 h; the LAIC arrow must carry while the control stays silent;");
    println!("registered alternative A — Ereignisrate — remains unbuilt (register).");

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
    let mut events: Vec<(f64, f64, f64, f64)> = Vec::new();
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
    let mut null_windows: Vec<(f64, f64, f64)> = Vec::new();
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
        let Some(data) = load("e", i) else {
            println!("{i:>4} | {:<19} | window file void", unix_to_iso(t0));
            continue;
        };
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
            "{i:>4} | {:<19} | M {:.1} | {:>7.2} {:>8.2} | {:<3} | cells {:<3} rate {} gaps {} | LI {:>+.3e} | IL {:>+.3e} | ctrl {:>+.3e} | fac {:>+.3e}/{:>+.3e} | swarm {}/{}",
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
            stat.swarm_samples.map_or("-".to_string(), |v| v.to_string()),
            stat.swarm_cells.map_or("-".to_string(), |v| v.to_string())
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
        let Some(data) = load("n", i) else {
            println!("null {i:>4} | {:<19} | window file void", unix_to_iso(t0));
            continue;
        };
        let stat = window_stat(&data, radius_km, cell_s, kde_scale);
        println!(
            "null {i:>4} | {:<19} | {:>7.2} {:>8.2} | {:<3} | cells {:<3} rate {} | LI {:>+.3e} | IL {:>+.3e} | ctrl {:>+.3e} | fac {:>+.3e}/{:>+.3e}",
            unix_to_iso(t0),
            lat,
            lon,
            data.station,
            stat.n_cells,
            stat.n_rate_events,
            stat.excess[0].map_or(f64::NAN, |v| v),
            stat.excess[1].map_or(f64::NAN, |v| v),
            stat.control_excess.map_or(f64::NAN, |v| v),
            stat.fac_excess[0].map_or(f64::NAN, |v| v),
            stat.fac_excess[1].map_or(f64::NAN, |v| v)
        );
        null_stats.push(stat);
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

    println!();
    println!("=== stack verdict ===");
    verdict_line("TE(Lithosphere → Ionosphere)", &s_li, &n_li, z_main);
    verdict_line("TE(Ionosphere → Lithosphere)", &s_il, &n_il, z_main);
    verdict_line("TE(Solar Bz → Ionosphere)", &s_ctrl, &n_ctrl, z_ctrl);
    verdict_line("TE(Lithosphere → FAC)", &s_fac_li, &n_fac_li, z_main);
    verdict_line("TE(FAC → Lithosphere)", &s_fac_il, &n_fac_il, z_main);
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
        s_li.mean, s_li.n, n_li.mean + 2.0 * n_li.sd
    );
    println!(
        "TE(Ionosphere → Lithosphere) = {:.4e}   (stack mean excess, n = {}, null mean + 2σ = {:.4e})",
        s_il.mean, s_il.n, n_il.mean + 2.0 * n_il.sd
    );
    println!(
        "control TE(Solar Bz → Ionosphere) = {:.4e}   (stack mean excess, n = {}, null mean + 2σ = {:.4e})",
        s_ctrl.mean, s_ctrl.n, n_ctrl.mean + 2.0 * n_ctrl.sd
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
        println!("verdict                      = arrow: Lithosphere → Ionosphere carries, reverse silent, solar control silent");
    } else if li_arrow && ctrl_arrow {
        println!("verdict                      = Lithosphere → Ionosphere carries but the solar control carries too — the common driver is not excluded");
    } else if il_arrow && !li_arrow {
        println!("verdict                      = reverse arrow: Ionosphere → Lithosphere carries");
    } else if li_arrow && il_arrow {
        println!("verdict                      = both directions carry — no dominant causal arrow on this stack");
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
