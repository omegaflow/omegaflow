use crate::miniseed::decode_body;
use omegaflow::ak135::{p_p_travel, p_travel_depth, s_p_travel};
use omegaflow::archivar::{fetch_raw, fetch_raw_bytes, parse_json, scalar_of, JsonVal};
use omegaflow::json::jpath;

pub const DATASELECT_ROUTE: &str = "https://service.earthscope.org/fdsnws/dataselect/1/query";
pub const CATALOG_URL: &str = "https://earthquake.usgs.gov/fdsnws/event/1/query";
pub const STATION_URL: &str = "https://service.iris.edu/fdsnws/station/1/query";
pub const GEBCO_URL: &str = "https://api.opentopodata.org/v1/gebco2020?locations=";

pub const MIN_DEPTH_KM: f64 = 35.0;
pub const MIN_MAG: f64 = 6.0;
pub const REGION: [f64; 4] = [34.0, 38.0, 68.0, 74.0];
pub const SEARCH_START: &str = "2000-01-01T00:00:00";
pub const MIN_DIST_DEG: f64 = 30.0;
pub const MAX_DIST_DEG: f64 = 90.0;
pub const MAX_STATIONS: usize = 12;
pub const P_WINDOW_AFTER_ORIGIN_S: f64 = 1500.0;
pub const SNR_GATE: f64 = 3.0;
pub const P_WAVELET_S: f64 = 4.0;
pub const WINDOW_LO: f64 = 0.7;
pub const WINDOW_HI: f64 = 1.3;
pub const EDGE_FRAC: f64 = 0.15;
pub const SECONDARY_CORR_MIN: f64 = 0.30;
pub const DEPTH_MATCH_GATE_KM: f64 = 10.0;
pub const STA_WINDOW_S: f64 = 1.0;
pub const LTA_WINDOW_S: f64 = 30.0;
pub const STA_LTA_RATIO: f64 = 4.0;

const INVERSION_DEPTH_MAX_KM: f64 = 700.0;
const INVERSION_DEPTH_EDGE_KM: f64 = 660.0;
const INVERSION_DEPTH_STEP_KM: f64 = 1.0;

const PI: f64 = std::f64::consts::PI;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DepthInversion {
    Depth(f64),
    EdgeDiscontinuity,
    SaturatedBound,
    Absent,
}

#[derive(Clone)]
pub struct Event {
    pub id: String,
    pub t0: f64,
    pub mag: f64,
    pub lat: f64,
    pub lon: f64,
    pub depth_km: f64,
}

#[derive(Clone)]
pub struct Station {
    pub net: String,
    pub sta: String,
    pub lat: f64,
    pub lon: f64,
}

pub struct SecondaryPick {
    pub lag_s: f64,
    pub corr: f64,
    pub inverted: bool,
    pub at_edge: bool,
    pub peak_half_width_s: Option<f64>,
    pub runner_up_ratio: Option<f64>,
}

pub fn arg_value(args: &[String], name: &str) -> Option<String> {
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

pub fn unix_to_iso(u: f64) -> String {
    let days = u.div_euclid(86400.0) as i64;
    let rest = (u - days as f64 * 86400.0).floor();
    let (y, m, d) = days_to_ymd(days);
    let hh = (rest / 3600.0) as u32;
    let mm = ((rest - hh as f64 * 3600.0) / 60.0) as u32;
    let ss = (rest - hh as f64 * 3600.0 - mm as f64 * 60.0) as u32;
    format!("{y:04}-{m:02}-{d:02}T{hh:02}:{mm:02}:{ss:02}")
}

fn haversine_km(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let r = 6371.0;
    let dlat = (lat2 - lat1).to_radians();
    let dlon = (lon2 - lon1).to_radians();
    let a = (dlat / 2.0).sin().powi(2)
        + lat1.to_radians().cos() * lat2.to_radians().cos() * (dlon / 2.0).sin().powi(2);
    2.0 * r * a.sqrt().atan2((1.0 - a).sqrt())
}

pub fn arc_deg(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    haversine_km(lat1, lon1, lat2, lon2) / 111.195
}

pub fn catalog_events(body: &str) -> Vec<Event> {
    let Some(j) = parse_json(body) else {
        return Vec::new();
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
        let Some(depth) = coords.get(2).and_then(scalar_of) else {
            continue;
        };
        if mag < MIN_MAG || depth < MIN_DEPTH_KM {
            continue;
        }
        let id = match fm.get("id") {
            Some(JsonVal::Str(s)) => s.clone(),
            _ => String::new(),
        };
        out.push(Event {
            id,
            t0: t_ms / 1000.0,
            mag,
            lat,
            lon,
            depth_km: depth,
        });
    }
    out
}

pub fn parse_stations_text(body: &str) -> Vec<Station> {
    let mut lines = body.lines();
    let Some(header) = lines.next() else {
        return Vec::new();
    };
    let cols: Vec<&str> = header.trim_start_matches('#').split('|').collect();
    let idx = |name: &str| cols.iter().position(|c| c.trim() == name);
    let (Some(i_net), Some(i_sta), Some(i_lat), Some(i_lon)) = (
        idx("Network"),
        idx("Station"),
        idx("Latitude"),
        idx("Longitude"),
    ) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for line in lines {
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let f: Vec<&str> = line.split('|').collect();
        if f.len() <= i_lon.max(i_lat).max(i_net).max(i_sta) {
            continue;
        }
        let Some(lat) = f[i_lat].trim().parse::<f64>().ok() else {
            continue;
        };
        let Some(lon) = f[i_lon].trim().parse::<f64>().ok() else {
            continue;
        };
        if !lat.is_finite() || !lon.is_finite() {
            continue;
        }
        out.push(Station {
            net: f[i_net].trim().to_string(),
            sta: f[i_sta].trim().to_string(),
            lat,
            lon,
        });
    }
    out
}

pub fn gebco_elevation(lat: f64, lon: f64) -> Option<f64> {
    let url = format!("{GEBCO_URL}{lat},{lon}");
    let body = fetch_raw(&url, None, &[], 86400)?;
    let json = parse_json(&body)?;
    let v = jpath(&json, "results.0.elevation")?;
    if v.is_finite() {
        Some(v)
    } else {
        None
    }
}

fn median_abs(xs: &[f64]) -> f64 {
    let mut sorted: Vec<f64> = xs.iter().map(|v| v.abs()).collect();
    sorted.sort_by(|a, b| a.total_cmp(b));
    sorted[sorted.len() / 2]
}

pub fn bandpass(samples: &[(f64, f64)], rate: f64) -> Vec<f64> {
    let Some(&(_, first)) = samples.first() else {
        return Vec::new();
    };
    let dt = 1.0 / rate;
    let rc_hp = 1.0 / (2.0 * PI * 0.5);
    let a_hp = rc_hp / (rc_hp + dt);
    let rc_lp = 1.0 / (2.0 * PI * 2.0);
    let a_lp = dt / (rc_lp + dt);
    let mut hp = 0.0;
    let mut lp = 0.0;
    let mut prev_x = first;
    let mut out = Vec::with_capacity(samples.len());
    for &(_, x) in samples {
        hp = a_hp * (hp + x - prev_x);
        prev_x = x;
        lp += a_lp * (hp - lp);
        out.push(lp);
    }
    out
}

fn sta_lta_arrival(samples: &[(f64, f64)], rate: f64) -> Option<f64> {
    let n_sta = (STA_WINDOW_S * rate).round() as usize;
    let n_lta = (LTA_WINDOW_S * rate).round() as usize;
    if n_sta == 0 || n_lta == 0 || samples.len() < n_lta + 1 {
        return None;
    }
    let n = samples.len();
    let mut prefix = Vec::with_capacity(n + 1);
    let mut acc = 0.0;
    prefix.push(0.0);
    for (_, v) in samples.iter() {
        acc += v.abs();
        prefix.push(acc);
    }
    for i in (n_lta - 1)..n {
        let sta = (prefix[i + 1] - prefix[i + 1 - n_sta]) / n_sta as f64;
        let lta = (prefix[i + 1] - prefix[i + 1 - n_lta]) / n_lta as f64;
        if lta > 1e-12 && sta / lta >= STA_LTA_RATIO {
            return Some(samples[i].0);
        }
    }
    None
}

fn first_break_arrival(samples: &[(f64, f64)], rate: f64) -> Option<f64> {
    let vals = bandpass(samples, rate);
    let noise_len = ((20.0 * rate).round() as usize).min(vals.len() / 2);
    if noise_len == 0 {
        return None;
    }
    let floor = median_abs(&vals[..noise_len]);
    if floor <= 1e-12 {
        return None;
    }
    let threshold = 5.0 * floor;
    let sustain = (1.0 * rate).round() as usize;
    if sustain == 0 {
        return None;
    }
    let mut count = 0usize;
    for i in noise_len..vals.len() {
        if vals[i].abs() > threshold {
            count += 1;
            if count >= sustain {
                return Some(samples[i - sustain + 1].0);
            }
        } else {
            count = 0;
        }
    }
    None
}

pub fn p_onset(samples: &[(f64, f64)], rate: f64) -> Option<f64> {
    first_break_arrival(samples, rate).or_else(|| sta_lta_arrival(samples, rate))
}

pub fn onset_index(samples: &[(f64, f64)], rate: f64, t_p: f64) -> usize {
    let Some(&(t0, _)) = samples.first() else {
        return 0;
    };
    if t_p <= t0 {
        return 0;
    }
    ((t_p - t0) * rate).round() as usize
}

pub fn onset_snr(bp: &[f64], rate: f64, i_p: usize) -> Option<f64> {
    let noise_len = ((20.0 * rate).round() as usize).min(bp.len() / 2);
    if noise_len == 0 || noise_len >= bp.len() || i_p >= bp.len() {
        return None;
    }
    let floor = median_abs(&bp[..noise_len]);
    if floor <= 1e-12 {
        return None;
    }
    let win = (P_WAVELET_S * rate).round() as usize;
    if win == 0 {
        return None;
    }
    let hi = (i_p + win).min(bp.len());
    let signal = bp[i_p..hi].iter().fold(0.0f64, |m, &v| m.max(v.abs()));
    Some(signal / floor)
}

pub fn correlate_window(
    bp: &[f64],
    i_p: usize,
    nw: usize,
    rate: f64,
    lag_target: f64,
) -> Option<SecondaryPick> {
    if i_p + nw > bp.len() {
        return None;
    }
    let wavelet = &bp[i_p..i_p + nw];
    let wnorm = wavelet.iter().map(|v| v * v).sum::<f64>().sqrt();
    if wnorm <= 1e-12 {
        return None;
    }
    let k_lo = (lag_target * WINDOW_LO * rate).floor() as usize;
    let k_hi = (lag_target * WINDOW_HI * rate).ceil() as usize;
    if k_hi <= k_lo {
        return None;
    }
    let edge = ((k_hi - k_lo) as f64 * EDGE_FRAC).round() as usize;
    let mut surf: Vec<f64> = Vec::with_capacity(k_hi - k_lo + 1);
    for k in k_lo..=k_hi {
        let seg_lo = i_p + k;
        let seg_hi = seg_lo + nw;
        if seg_hi > bp.len() {
            break;
        }
        let seg = &bp[seg_lo..seg_hi];
        let snorm = seg.iter().map(|v| v * v).sum::<f64>().sqrt();
        let dot = wavelet
            .iter()
            .zip(seg.iter())
            .map(|(a, b)| a * b)
            .sum::<f64>();
        surf.push(dot / (wnorm * snorm));
    }
    if surf.is_empty() {
        return None;
    }
    let mut i_star = 0usize;
    for i in 1..surf.len() {
        if surf[i].abs() > surf[i_star].abs() {
            i_star = i;
        }
    }
    let k_star = k_lo + i_star;
    let at_edge = k_star <= k_lo + edge || k_star >= k_hi - edge;
    let corr = surf[i_star];
    let c0 = corr.abs();
    let lag_s = if !at_edge && i_star > 0 && i_star + 1 < surf.len() {
        let cm = surf[i_star - 1].abs();
        let cp = surf[i_star + 1].abs();
        let denom = cm + cp - 2.0 * c0;
        if denom.abs() > 1e-12 {
            let delta = ((cm - cp) / (2.0 * denom)).clamp(-0.5, 0.5);
            (k_star as f64 + delta) / rate
        } else {
            k_star as f64 / rate
        }
    } else {
        k_star as f64 / rate
    };
    let peak_half_width_s = if at_edge {
        None
    } else {
        let threshold = c0 / 2.0;
        let mut x_left = None;
        for j in (k_lo..k_star).rev() {
            let sj = surf[j - k_lo].abs();
            let sj1 = surf[j + 1 - k_lo].abs();
            if sj >= threshold && sj1 < threshold {
                let denom = sj - sj1;
                x_left = Some(if denom.abs() <= 1e-12 {
                    j as f64 + 0.5
                } else {
                    j as f64 + (sj - threshold) / denom
                });
                break;
            }
        }
        let k_end = k_lo + surf.len() - 1;
        let mut x_right = None;
        for j in (k_star + 1)..=k_hi.min(k_end) {
            let sj = surf[j - k_lo].abs();
            let sj1 = surf[j - 1 - k_lo].abs();
            if sj >= threshold && sj1 < threshold {
                let denom = sj - sj1;
                x_right = Some(if denom.abs() <= 1e-12 {
                    j as f64 - 0.5
                } else {
                    j as f64 - (sj - threshold) / denom
                });
                break;
            }
        }
        match (x_left, x_right) {
            (Some(xl), Some(xr)) => {
                let width_samples = xr - xl;
                if width_samples >= 1.0 {
                    Some(width_samples / rate)
                } else {
                    None
                }
            }
            _ => None,
        }
    };
    let runner_up_ratio = {
        let mut best = None;
        for i in 0..surf.len() {
            if i.abs_diff(i_star) <= 1 {
                continue;
            }
            let above_left = i == 0 || surf[i].abs() >= surf[i - 1].abs();
            let above_right = i + 1 == surf.len() || surf[i].abs() >= surf[i + 1].abs();
            if above_left && above_right {
                let v = surf[i].abs();
                if best.map_or(true, |b| v > b) {
                    best = Some(v);
                }
            }
        }
        best.map(|b| b / c0)
    };
    Some(SecondaryPick {
        lag_s,
        corr,
        inverted: corr < 0.0,
        at_edge,
        peak_half_width_s,
        runner_up_ratio,
    })
}

pub fn peak_sigma_s(pick: &SecondaryPick, rate: f64) -> Option<f64> {
    if pick.at_edge {
        return None;
    }
    match pick.peak_half_width_s {
        Some(w) if w.is_finite() && w > 0.0 => Some(w),
        Some(_) => None,
        None => {
            if rate.is_finite() && rate > 0.0 {
                Some(1.0 / rate)
            } else {
                None
            }
        }
    }
}

pub fn p_p_lag(delta_deg: f64, depth_km: f64) -> Option<f64> {
    Some(p_p_travel(delta_deg, depth_km)? - p_travel_depth(delta_deg, depth_km)?)
}

pub fn s_p_lag(delta_deg: f64, depth_km: f64) -> Option<f64> {
    Some(s_p_travel(delta_deg, depth_km)? - p_travel_depth(delta_deg, depth_km)?)
}

fn inversion_state(best_sq: f64, best_h: f64) -> DepthInversion {
    if !best_sq.is_finite() {
        DepthInversion::Absent
    } else if best_h == INVERSION_DEPTH_EDGE_KM {
        DepthInversion::EdgeDiscontinuity
    } else if best_h == INVERSION_DEPTH_MAX_KM {
        DepthInversion::SaturatedBound
    } else {
        DepthInversion::Depth(best_h)
    }
}

pub fn invert_depth_single(delta_deg: f64, lag: f64) -> DepthInversion {
    let mut best = (f64::INFINITY, 0.0f64);
    let n = (INVERSION_DEPTH_MAX_KM / INVERSION_DEPTH_STEP_KM) as usize;
    for i in 0..=n {
        let h = i as f64 * INVERSION_DEPTH_STEP_KM;
        if let Some(pred) = p_p_lag(delta_deg, h) {
            let r = lag - pred;
            let s = r * r;
            if s < best.0 {
                best = (s, h);
            }
        }
    }
    inversion_state(best.0, best.1)
}

pub fn invert_depth_weighted(deltas: &[f64], lags: &[f64], weights: &[f64]) -> DepthInversion {
    if deltas.is_empty() || deltas.len() != lags.len() || lags.len() != weights.len() {
        return DepthInversion::Absent;
    }
    let mut best = (f64::INFINITY, 0.0f64);
    let n = (INVERSION_DEPTH_MAX_KM / INVERSION_DEPTH_STEP_KM) as usize;
    for i in 0..=n {
        let h = i as f64 * INVERSION_DEPTH_STEP_KM;
        let mut s = 0.0;
        let mut ok = true;
        for ((&d, &lag), &w) in deltas.iter().zip(lags.iter()).zip(weights.iter()) {
            if !(w.is_finite() && w >= 0.0) {
                ok = false;
                break;
            }
            match p_p_lag(d, h) {
                Some(pred) => {
                    let r = lag - pred;
                    s += r * r * w;
                }
                None => {
                    ok = false;
                    break;
                }
            }
        }
        if ok && s < best.0 {
            best = (s, h);
        }
    }
    inversion_state(best.0, best.1)
}

pub fn invert_depth_multi(deltas: &[f64], lags: &[f64]) -> DepthInversion {
    let mut best = (f64::INFINITY, 0.0f64);
    let n = (INVERSION_DEPTH_MAX_KM / INVERSION_DEPTH_STEP_KM) as usize;
    for i in 0..=n {
        let h = i as f64 * INVERSION_DEPTH_STEP_KM;
        let mut s = 0.0;
        let mut ok = true;
        for (&d, &lag) in deltas.iter().zip(lags.iter()) {
            match p_p_lag(d, h) {
                Some(pred) => {
                    let r = lag - pred;
                    s += r * r;
                }
                None => {
                    ok = false;
                    break;
                }
            }
        }
        if ok && s < best.0 {
            best = (s, h);
        }
    }
    inversion_state(best.0, best.1)
}

pub const BRANCH_UNSTABLE_SKIP: &str = "pP Δ-branch fold (branch-unstable)";
const FOLD_BAND_HALF_WIDTHS_DEG: [f64; 4] = [0.25, 0.5, 0.75, 1.0];
const SMOOTH_REFERENCE_DELTA_LO_DEG: f64 = 40.0;
const SMOOTH_REFERENCE_DELTA_HI_DEG: f64 = 90.0;
const SMOOTH_REFERENCE_STEP_DEG: f64 = 1.0;
pub const FOLD_GATE_OVER_SMOOTH_FACTOR: f64 = 3.0;

pub struct DeltaBranch {
    pub fold_metric_s_per_deg: Option<f64>,
    pub smooth_s_per_deg: Option<f64>,
    pub unstable: bool,
}

fn p_p_lag_gradient_across(delta_deg: f64, depth_km: f64, half_width_deg: f64) -> Option<f64> {
    let hi = p_p_lag(delta_deg + half_width_deg, depth_km)?;
    let lo = p_p_lag(delta_deg - half_width_deg, depth_km)?;
    Some((hi - lo).abs() / (2.0 * half_width_deg))
}

pub fn p_p_branch_fold_metric(delta_deg: f64, depth_km: f64) -> Option<f64> {
    FOLD_BAND_HALF_WIDTHS_DEG
        .iter()
        .filter_map(|&d| p_p_lag_gradient_across(delta_deg, depth_km, d))
        .fold(None, |best, g| Some(best.map_or(g, |b: f64| b.max(g))))
}

pub fn smooth_p_p_lag_gradient_s_per_deg(depth_km: f64) -> Option<f64> {
    let mut grads = Vec::new();
    let mut delta = SMOOTH_REFERENCE_DELTA_LO_DEG;
    while delta <= SMOOTH_REFERENCE_DELTA_HI_DEG {
        if let Some(g) = p_p_branch_fold_metric(delta, depth_km) {
            grads.push(g);
        }
        delta += SMOOTH_REFERENCE_STEP_DEG;
    }
    if grads.is_empty() {
        return None;
    }
    grads.sort_by(|a, b| a.total_cmp(b));
    Some(grads[grads.len() / 2])
}

pub fn delta_branch(delta_deg: f64, depth_km: f64) -> DeltaBranch {
    let metric = p_p_branch_fold_metric(delta_deg, depth_km);
    let smooth = smooth_p_p_lag_gradient_s_per_deg(depth_km);
    let unstable = match (metric, smooth) {
        (Some(m), Some(s)) => m > s * FOLD_GATE_OVER_SMOOTH_FACTOR,
        _ => false,
    };
    DeltaBranch {
        fold_metric_s_per_deg: metric,
        smooth_s_per_deg: smooth,
        unstable,
    }
}

pub fn median(xs: &mut [f64]) -> f64 {
    xs.sort_by(|a, b| a.total_cmp(b));
    xs[xs.len() / 2]
}

pub fn fetch_station_body(
    station: &Station,
    start: &str,
    end: &str,
) -> Option<(Vec<(f64, f64)>, f64)> {
    let url = format!(
        "{DATASELECT_ROUTE}?network={}&station={}&channel=BHZ&starttime={}&endtime={}&format=miniseed",
        station.net, station.sta, start, end
    );
    let body = fetch_raw_bytes(&url, 3600)?;
    decode_body(&body)
}

pub struct StationMeasure {
    pub key: String,
    pub delta_deg: f64,
    pub snr: f64,
    pub p_p: Option<SecondaryPick>,
    pub s_p: Option<SecondaryPick>,
    pub inversion: Option<DepthInversion>,
    pub p_p_sigma_s: Option<f64>,
    pub p_p_lag_pred: f64,
    pub s_p_lag_pred: Option<f64>,
    pub skip: Option<String>,
    pub branch_unstable: bool,
}

fn skipped(key: String, delta_deg: f64, reason: &str) -> StationMeasure {
    StationMeasure {
        key,
        delta_deg,
        snr: 0.0,
        p_p: None,
        s_p: None,
        inversion: None,
        p_p_sigma_s: None,
        p_p_lag_pred: 0.0,
        s_p_lag_pred: None,
        skip: Some(reason.to_string()),
        branch_unstable: false,
    }
}

pub fn measure_station(event: &Event, station: &Station, start: &str, end: &str) -> StationMeasure {
    let key = format!("{}.{}", station.net, station.sta);
    let delta = arc_deg(event.lat, event.lon, station.lat, station.lon);
    if delta_branch(delta, event.depth_km).unstable {
        let mut m = skipped(key, delta, BRANCH_UNSTABLE_SKIP);
        m.branch_unstable = true;
        return m;
    }
    let Some((samples, rate)) = fetch_station_body(station, start, end) else {
        return skipped(key, delta, "no decodable record");
    };
    let Some(t_p) = p_onset(&samples, rate) else {
        return skipped(key, delta, "no P pick");
    };
    let bp = bandpass(&samples, rate);
    let i_p = onset_index(&samples, rate, t_p);
    let Some(snr) = onset_snr(&bp, rate, i_p) else {
        return skipped(key, delta, "no noise floor");
    };
    if snr < SNR_GATE {
        return skipped(key, delta, "SNR below gate");
    }
    let nw = (P_WAVELET_S * rate).round() as usize;
    if nw == 0 {
        return skipped(key, delta, "degenerate rate");
    }
    let p_p_lag_pred = match p_p_lag(delta, event.depth_km) {
        Some(l) => l,
        None => return skipped(key, delta, "no ak135 pP prediction"),
    };
    let pp = correlate_window(&bp, i_p, nw, rate, p_p_lag_pred);
    let p_p_sigma_s = pp.as_ref().and_then(|p| peak_sigma_s(p, rate));
    let s_p_lag_pred = s_p_lag(delta, event.depth_km);
    let sp = s_p_lag_pred.and_then(|l| correlate_window(&bp, i_p, nw, rate, l));
    let inversion = match &pp {
        Some(p) if p.corr.abs() >= SECONDARY_CORR_MIN => Some(invert_depth_single(delta, p.lag_s)),
        _ => None,
    };
    StationMeasure {
        key,
        delta_deg: delta,
        snr,
        p_p: pp,
        s_p: sp,
        inversion,
        p_p_sigma_s,
        p_p_lag_pred,
        s_p_lag_pred,
        skip: None,
        branch_unstable: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lcg(seed: u64, idx: usize) -> f64 {
        let mut x = seed
            .wrapping_add(idx as u64)
            .wrapping_mul(0x9E37_79B9_7F4A_7C15);
        x = (x ^ (x >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        x = (x ^ (x >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        x ^= x >> 31;
        (x as f64) / (u64::MAX as f64) * 2.0 - 1.0
    }

    fn synth_trace(rate: f64, dur_s: f64, t_p: f64, p_amp: f64) -> Vec<(f64, f64)> {
        let n = (dur_s * rate) as usize;
        let mut out = Vec::with_capacity(n);
        for i in 0..n {
            let t = 1000.0 + i as f64 / rate;
            let mut v = 0.01;
            if t >= t_p {
                let ph = (t - t_p) * 2.0 * PI * 1.0;
                let env = (-((t - t_p) / 1.5)).exp();
                v += p_amp * env * ph.sin();
            }
            if t >= t_p + 4.0 {
                let coda_env = 0.1 * p_amp * (-((t - t_p - 4.0) / 12.0)).exp();
                v += coda_env * lcg(0xC0DA, i);
            }
            out.push((t, v));
        }
        out
    }

    fn inject(samples: &[(f64, f64)], t_inj: f64, amp: f64, polarity: f64) -> Vec<(f64, f64)> {
        let mut out = samples.to_vec();
        for (t, v) in out.iter_mut() {
            if *t >= t_inj {
                let ph = (*t - t_inj) * 2.0 * PI * 1.0;
                let env = (-((*t - t_inj) / 1.5)).exp();
                *v += polarity * amp * env * ph.sin();
            }
        }
        out
    }

    fn assert_depth(state: DepthInversion, true_h: f64) {
        match state {
            DepthInversion::Depth(v) => assert!(
                (v - true_h).abs() <= 1.0,
                "inverted {v} km vs true {true_h} km"
            ),
            other => panic!("depth {true_h} km inverted to {other:?}"),
        }
    }

    #[test]
    fn p_p_lag_vanishes_at_zero_depth() {
        for d in [20.0, 40.0, 60.0, 80.0] {
            let lag = p_p_lag(d, 0.0).unwrap();
            assert!(lag.abs() < 1e-6, "pP-P at h=0 should vanish, got {lag}");
        }
    }

    #[test]
    fn lag_grows_across_the_shallow_band() {
        for d in [20.0, 30.0, 60.0] {
            let l0 = p_p_lag(d, 0.0).unwrap();
            let l10 = p_p_lag(d, 10.0).unwrap();
            let l20 = p_p_lag(d, 20.0).unwrap();
            let l100 = p_p_lag(d, 100.0).unwrap();
            assert!(l10 > l0 && l20 > l10, "shallow lag must grow");
            assert!(l100 > l20, "deep lag above shallow");
        }
    }

    #[test]
    fn s_p_lag_exceeds_p_p_lag() {
        for d in [20.0, 40.0, 60.0] {
            for h in [10.0, 20.0, 35.0, 50.0] {
                assert!(s_p_lag(d, h).unwrap() > p_p_lag(d, h).unwrap());
            }
        }
    }

    #[test]
    fn invert_multi_recovers_the_source_depth() {
        for true_h in [10.0, 20.0, 35.0] {
            let deltas = [20.0, 35.0, 50.0, 65.0, 80.0];
            let lags: Vec<f64> = deltas
                .iter()
                .map(|&d| p_p_lag(d, true_h).unwrap())
                .collect();
            let h = invert_depth_multi(&deltas, &lags);
            assert_depth(h, true_h);
        }
    }

    #[test]
    fn inversion_survives_a_second_of_pick_scatter() {
        let true_h = 20.0;
        let deltas = [20.0, 35.0, 50.0, 65.0, 80.0];
        let lags: Vec<f64> = deltas
            .iter()
            .map(|&d| p_p_lag(d, true_h).unwrap() + 0.5)
            .collect();
        let h = invert_depth_multi(&deltas, &lags);
        match h {
            DepthInversion::Depth(v) => assert!(
                (v - true_h).abs() <= 10.0,
                "with +0.5 s bias, inverted {v} km"
            ),
            other => panic!("depth {true_h} km with scatter must stay Depth, got {other:?}"),
        }
    }

    #[test]
    fn invert_single_recovers_the_catalog_depth() {
        for true_h in [35.0, 50.0, 100.0, 200.0] {
            let delta = 45.0;
            let lag = p_p_lag(delta, true_h).unwrap();
            let h = invert_depth_single(delta, lag);
            assert_depth(h, true_h);
        }
    }

    #[test]
    fn invert_single_recovers_a_deep_catalog_depth() {
        for true_h in [300.0, 410.0, 500.0, 600.0] {
            let delta = 45.0;
            let lag = p_p_lag(delta, true_h).unwrap();
            let h = invert_depth_single(delta, lag);
            assert_depth(h, true_h);
        }
        let lag_660 = p_p_lag(45.0, 660.0).unwrap();
        let state = invert_depth_single(45.0, lag_660);
        assert_eq!(
            state,
            DepthInversion::EdgeDiscontinuity,
            "a synthetic 660 km lag lands on the 660 wall — named, not asserted as a depth"
        );
        let lag_700 = p_p_lag(45.0, 700.0).unwrap();
        let state = invert_depth_single(45.0, lag_700);
        assert_eq!(
            state,
            DepthInversion::SaturatedBound,
            "a synthetic 700 km lag saturates the search ceiling — named, not asserted as a depth"
        );
    }

    #[test]
    fn invert_single_resolves_between_the_deep_grid_nodes() {
        let true_h = 231.0;
        let delta = 45.0;
        let lag = p_p_lag(delta, true_h).unwrap();
        let h = invert_depth_single(delta, lag);
        match h {
            DepthInversion::Depth(v) => assert!(
                (v - true_h).abs() <= 1.0,
                "a depth between the 200 and 250 km nodes must invert to {true_h} km, got {v}"
            ),
            other => panic!("depth 231 km must invert to Depth, got {other:?}"),
        }
    }

    #[test]
    fn kalibrier_gate_660_wall_700_ceiling_400_depth_and_absent() {
        let delta = 45.0;
        let on_660 = invert_depth_single(delta, p_p_lag(delta, 660.0).unwrap());
        assert_eq!(
            on_660,
            DepthInversion::EdgeDiscontinuity,
            "a lag whose grid minimum lands on 660 is the wall, never Depth(660)"
        );
        let on_700 = invert_depth_single(delta, p_p_lag(delta, 700.0).unwrap());
        assert_eq!(
            on_700,
            DepthInversion::SaturatedBound,
            "a lag whose grid minimum lands on 700 is the ceiling, never Depth(700)"
        );
        let mid = invert_depth_single(delta, p_p_lag(delta, 400.0).unwrap());
        assert_depth(mid, 400.0);
        let absent = invert_depth_single(200.0, p_p_lag(delta, 400.0).unwrap());
        assert_eq!(
            absent,
            DepthInversion::Absent,
            "a delta beyond the ak135 range carries no finite residual — Absent, never a depth"
        );
    }

    #[test]
    fn injected_p_p_in_real_coda_is_recovered_at_its_lag() {
        let rate = 40.0;
        let t_p = 1030.0;
        let mut samples = synth_trace(rate, 90.0, t_p, 10.0);
        samples = inject(&samples, t_p + 6.0, 6.0, -1.0);
        let t_onset = p_onset(&samples, rate).unwrap();
        assert!((t_onset - t_p).abs() < 1.0, "onset {t_onset} vs {t_p}");
        let bp = bandpass(&samples, rate);
        let i_p = onset_index(&samples, rate, t_onset);
        let nw = (P_WAVELET_S * rate).round() as usize;
        let pick = correlate_window(&bp, i_p, nw, rate, 6.0).unwrap();
        assert!(
            (pick.lag_s - 6.0).abs() < 0.5,
            "recovered lag {} vs 6.0",
            pick.lag_s
        );
        assert!(pick.corr.abs() >= 0.4, "corr {} too weak", pick.corr);
        assert!(
            pick.inverted,
            "injected negative polarity must read inverted"
        );
        assert!(
            !pick.at_edge,
            "a 6 s lag in a [0.7,1.3] window is not at the edge"
        );
    }

    #[test]
    fn positive_only_peak_reads_as_positive() {
        let rate = 40.0;
        let t_p = 1030.0;
        let mut samples = synth_trace(rate, 90.0, t_p, 10.0);
        samples = inject(&samples, t_p + 6.0, 6.0, 1.0);
        let t_onset = p_onset(&samples, rate).unwrap();
        let bp = bandpass(&samples, rate);
        let i_p = onset_index(&samples, rate, t_onset);
        let nw = (P_WAVELET_S * rate).round() as usize;
        let pick = correlate_window(&bp, i_p, nw, rate, 6.0).unwrap();
        assert!(
            !pick.inverted,
            "a positive-only injection reads positive (the suspicious signature)"
        );
    }

    #[test]
    fn an_absent_s_p_peaks_below_the_injected_phase() {
        let rate = 40.0;
        let t_p = 1030.0;
        let base = synth_trace(rate, 90.0, t_p, 10.0);
        let with_s_p = inject(&base, t_p + 8.0, 6.0, -1.0);
        let absent_peak = {
            let t_onset = p_onset(&base, rate).unwrap();
            let bp = bandpass(&base, rate);
            let i_p = onset_index(&base, rate, t_onset);
            let nw = (P_WAVELET_S * rate).round() as usize;
            correlate_window(&bp, i_p, nw, rate, 8.0)
                .unwrap()
                .corr
                .abs()
        };
        let present = {
            let t_onset = p_onset(&with_s_p, rate).unwrap();
            let bp = bandpass(&with_s_p, rate);
            let i_p = onset_index(&with_s_p, rate, t_onset);
            let nw = (P_WAVELET_S * rate).round() as usize;
            correlate_window(&bp, i_p, nw, rate, 8.0).unwrap()
        };
        assert!(
            absent_peak < present.corr.abs(),
            "the injected sP peak {} must stand above the absent floor {}",
            present.corr.abs(),
            absent_peak
        );
        assert!(
            (present.lag_s - 8.0).abs() < 0.5,
            "the injected sP is recovered at lag {} vs 8.0",
            present.lag_s
        );
        assert!(present.inverted, "the injected negative sP reads inverted");
    }

    #[test]
    fn kalibrier_gate_delta_fold_flags_the_fold_band() {
        for (delta, h) in [
            (31.0, 410.0),
            (32.0, 410.0),
            (33.0, 410.0),
            (32.0, 450.0),
            (33.0, 450.0),
            (34.0, 500.0),
            (35.0, 550.0),
            (37.0, 600.0),
        ] {
            let b = delta_branch(delta, h);
            let metric = b.fold_metric_s_per_deg.unwrap();
            let smooth = b.smooth_s_per_deg.unwrap();
            assert!(
                b.unstable,
                "a station at Δ={delta}° for {h} km sits in the folded pP family — branch-unstable (metric {metric:.2} vs smooth {smooth:.2} s/deg)"
            );
            assert!(
                metric > smooth * FOLD_GATE_OVER_SMOOTH_FACTOR,
                "the fold metric {metric:.2} must clear the smooth threshold {}",
                smooth * FOLD_GATE_OVER_SMOOTH_FACTOR
            );
        }
    }

    #[test]
    fn kalibrier_gate_delta_fold_passes_the_single_branch_band() {
        for (delta, h) in [
            (45.0, 410.0),
            (45.0, 450.0),
            (45.0, 500.0),
            (45.0, 550.0),
            (45.0, 600.0),
            (60.0, 450.0),
            (60.0, 600.0),
        ] {
            let b = delta_branch(delta, h);
            let metric = b.fold_metric_s_per_deg.unwrap();
            let smooth = b.smooth_s_per_deg.unwrap();
            assert!(
                !b.unstable,
                "a station at Δ={delta}° for {h} km sits on the single-branch band — stable (metric {metric:.2} vs smooth {smooth:.2} s/deg)"
            );
        }
    }

    #[test]
    fn kalibrier_gate_delta_fold_660_geometry_stays_clear() {
        for delta in [30.0, 35.0, 40.0, 44.0, 46.0] {
            let b = delta_branch(delta, 660.0);
            assert!(
                !b.unstable,
                "the 660 km pP geometry carries no fold up to Δ≈46° — Δ={delta}° must stay stable (metric {:.2} s/deg)",
                b.fold_metric_s_per_deg.unwrap()
            );
        }
    }

    #[test]
    fn kalibrier_gate_delta_fold_depth_direction_stays_injective() {
        for delta in [40.0, 45.0, 50.0, 60.0] {
            let mut prev = p_p_lag(delta, 250.0).unwrap();
            for h in [300.0, 410.0, 450.0, 500.0, 550.0, 600.0, 660.0] {
                let lag = p_p_lag(delta, h).unwrap();
                assert!(
                    lag > prev,
                    "at fixed Δ={delta}° the pP lag must stay strictly injective in depth: {h} km = {lag} not above {prev}"
                );
                prev = lag;
                assert!(
                    !delta_branch(delta, h).unstable,
                    "a mid-band Δ={delta}° at {h} km is smooth in depth — never flagged"
                );
            }
        }
    }

    #[test]
    fn sub_sample_lag_beats_the_nearest_sample() {
        let rate = 40.0;
        let t_p = 1030.0;
        let mut samples = synth_trace(rate, 90.0, t_p, 10.0);
        samples = inject(&samples, t_p + 6.137, 6.0, -1.0);
        let t_onset = p_onset(&samples, rate).unwrap();
        let bp = bandpass(&samples, rate);
        let i_p = onset_index(&samples, rate, t_onset);
        let nw = (P_WAVELET_S * rate).round() as usize;
        let pick = correlate_window(&bp, i_p, nw, rate, 6.0).unwrap();
        assert!(
            (pick.lag_s - 6.137).abs() < 0.0125,
            "sub-sample lag {} must beat the half-sample grid",
            pick.lag_s
        );
        assert!(
            (pick.lag_s - 6.137).abs() < (6.15f64 - 6.137).abs(),
            "sub-sample lag {} must beat the nearest sample 6.15",
            pick.lag_s
        );
    }

    #[test]
    fn sharp_and_broad_peaks_of_equal_height_carry_different_sigmas() {
        let sharp = SecondaryPick {
            lag_s: 6.0,
            corr: 0.8,
            inverted: false,
            at_edge: false,
            peak_half_width_s: Some(2.0),
            runner_up_ratio: None,
        };
        let broad = SecondaryPick {
            lag_s: 6.0,
            corr: 0.8,
            inverted: false,
            at_edge: false,
            peak_half_width_s: Some(5.0),
            runner_up_ratio: None,
        };
        assert_eq!(peak_sigma_s(&sharp, 40.0), Some(2.0));
        assert_eq!(peak_sigma_s(&broad, 40.0), Some(5.0));
        assert!(
            1.0 / (2.0 * 2.0) > 1.0 / (5.0 * 5.0),
            "the sharp peak carries the larger weight"
        );
    }

    #[test]
    fn weighted_fit_with_uniform_weights_reproduces_the_equal_weight_fit() {
        for true_h in [10.0, 20.0, 35.0] {
            let deltas = [20.0, 35.0, 50.0, 65.0, 80.0];
            let lags: Vec<f64> = deltas
                .iter()
                .map(|&d| p_p_lag(d, true_h).unwrap())
                .collect();
            let weights = vec![1.0; deltas.len()];
            let h_w = invert_depth_weighted(&deltas, &lags, &weights);
            let h_eq = invert_depth_multi(&deltas, &lags);
            assert_eq!(
                h_w, h_eq,
                "uniform weights reproduce the equal-weight fit at {true_h} km"
            );
            assert_depth(h_w, true_h);
            assert_depth(h_eq, true_h);
        }
    }

    #[test]
    fn weighted_fit_lets_the_sharp_station_dominate() {
        let true_h = 20.0;
        let deltas = [45.0, 60.0];
        let lag1 = p_p_lag(45.0, true_h).unwrap();
        let lag2 = p_p_lag(60.0, true_h).unwrap() + 1.0;
        let lags = [lag1, lag2];
        let weights = [1.0 / (0.05 * 0.05), 1.0 / (0.5 * 0.5)];
        let h_w = invert_depth_weighted(&deltas, &lags, &weights);
        let h_eq = invert_depth_multi(&deltas, &lags);
        let (DepthInversion::Depth(hw), DepthInversion::Depth(heq)) = (h_w, h_eq) else {
            panic!("both fits must carry Depth");
        };
        assert!(
            (hw - true_h).abs() < (heq - true_h).abs(),
            "weighted {hw} km must sit closer to {true_h} than the equal-weight {heq} km"
        );
    }

    #[test]
    fn runner_up_ratio_measures_the_distant_peak_not_the_flank() {
        let rate = 40.0;
        let t_p = 1030.0;
        let mut samples = synth_trace(rate, 90.0, t_p, 10.0);
        samples = inject(&samples, t_p + 6.0, 6.0, -1.0);
        samples = inject(&samples, t_p + 8.0, 2.0, 1.0);
        let t_onset = p_onset(&samples, rate).unwrap();
        let bp = bandpass(&samples, rate);
        let i_p = onset_index(&samples, rate, t_onset);
        let nw = (P_WAVELET_S * rate).round() as usize;
        let pick = correlate_window(&bp, i_p, nw, rate, 6.0).unwrap();
        let ratio = pick
            .runner_up_ratio
            .expect("a distant peak must be measured as the runner-up");
        assert!(
            ratio > 0.0 && ratio < 1.0,
            "runner-up ratio {ratio} must sit between the flank (0) and the pick (1)"
        );
        assert!(
            pick.peak_half_width_s.is_some(),
            "the main peak carries a width"
        );
        assert!(
            (pick.lag_s - 6.0).abs() < 0.5,
            "the main peak lag {} vs 6.0",
            pick.lag_s
        );
    }

    #[test]
    fn single_sample_peak_carries_no_width_but_the_sample_floor() {
        let pick = SecondaryPick {
            lag_s: 6.0,
            corr: 0.8,
            inverted: false,
            at_edge: false,
            peak_half_width_s: None,
            runner_up_ratio: None,
        };
        assert_eq!(peak_sigma_s(&pick, 20.0), Some(1.0 / 20.0));
    }

    #[test]
    fn polarity_inversion_keeps_the_surface() {
        let rate = 40.0;
        let t_p = 1030.0;
        let base = synth_trace(rate, 90.0, t_p, 10.0);
        let trace_a = inject(&base, t_p + 6.137, 6.0, -1.0);
        let trace_b = inject(&base, t_p + 6.137, 6.0, 1.0);
        let measure = |trace: &[(f64, f64)]| {
            let t_onset = p_onset(trace, rate).unwrap();
            let bp = bandpass(trace, rate);
            let i_p = onset_index(trace, rate, t_onset);
            let nw = (P_WAVELET_S * rate).round() as usize;
            correlate_window(&bp, i_p, nw, rate, 6.0).unwrap()
        };
        let pa = measure(&trace_a);
        let pb = measure(&trace_b);
        assert!(
            (pa.lag_s - pb.lag_s).abs() < 1.0 / rate,
            "the |corr| surface peak is polarity-blind to within a sample: lag {} vs {}",
            pa.lag_s,
            pb.lag_s
        );
        match (pa.peak_half_width_s, pb.peak_half_width_s) {
            (Some(a), Some(b)) => assert!(
                (a - b).abs() < 1.0 / rate,
                "the peak width is polarity-blind: {a} s vs {b} s"
            ),
            (a, b) => assert_eq!(a, b),
        }
        match (pa.runner_up_ratio, pb.runner_up_ratio) {
            (Some(a), Some(b)) => assert!(
                (a - b).abs() < 0.02,
                "the runner-up ratio is polarity-blind: {a} vs {b}"
            ),
            (a, b) => assert_eq!(a, b),
        }
        assert_eq!(pa.inverted, !pb.inverted);
    }

    #[test]
    fn a_lag_at_the_window_edge_is_flagged() {
        let rate = 40.0;
        let t_p = 1030.0;
        let mut samples = synth_trace(rate, 90.0, t_p, 10.0);
        samples = inject(&samples, t_p + 10.2, 6.0, -1.0);
        let t_onset = p_onset(&samples, rate).unwrap();
        let bp = bandpass(&samples, rate);
        let i_p = onset_index(&samples, rate, t_onset);
        let nw = (P_WAVELET_S * rate).round() as usize;
        let pick = correlate_window(&bp, i_p, nw, rate, 8.0).unwrap();
        assert!(
            pick.at_edge,
            "a lag at the edge of the [0.7,1.3] window must be flagged"
        );
        assert!(
            pick.peak_half_width_s.is_none(),
            "an edge pick carries no peak width"
        );
        assert!(
            peak_sigma_s(&pick, 40.0).is_none(),
            "an edge pick carries no sigma"
        );
    }
}
