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

const INVERSION_DEPTH_MAX_KM: f64 = 250.0;
const INVERSION_DEPTH_STEP_KM: f64 = 1.0;

const PI: f64 = std::f64::consts::PI;

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
    let mut best: Option<(f64, f64, bool)> = None;
    for k in k_lo..=k_hi {
        let seg_lo = i_p + k;
        let seg_hi = seg_lo + nw;
        if seg_hi > bp.len() {
            break;
        }
        let seg = &bp[seg_lo..seg_hi];
        let snorm = seg.iter().map(|v| v * v).sum::<f64>().sqrt();
        if snorm <= 1e-12 {
            continue;
        }
        let dot = wavelet
            .iter()
            .zip(seg.iter())
            .map(|(a, b)| a * b)
            .sum::<f64>();
        let corr = dot / (wnorm * snorm);
        let keep = match best {
            None => true,
            Some((bc, _, _)) => corr.abs() > bc.abs(),
        };
        if keep {
            let at_edge = k <= k_lo + edge || k >= k_hi - edge;
            best = Some((corr, k as f64 / rate, at_edge));
        }
    }
    best.map(|(corr, lag, at_edge)| SecondaryPick {
        lag_s: lag,
        corr,
        inverted: corr < 0.0,
        at_edge,
    })
}

pub fn p_p_lag(delta_deg: f64, depth_km: f64) -> Option<f64> {
    Some(p_p_travel(delta_deg, depth_km)? - p_travel_depth(delta_deg, depth_km)?)
}

pub fn s_p_lag(delta_deg: f64, depth_km: f64) -> Option<f64> {
    Some(s_p_travel(delta_deg, depth_km)? - p_travel_depth(delta_deg, depth_km)?)
}

pub fn invert_depth_single(delta_deg: f64, lag: f64) -> Option<f64> {
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
    if best.0.is_finite() {
        Some(best.1)
    } else {
        None
    }
}

pub fn invert_depth_multi(deltas: &[f64], lags: &[f64]) -> Option<f64> {
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
    if best.0.is_finite() {
        Some(best.1)
    } else {
        None
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
            let h = invert_depth_multi(&deltas, &lags).unwrap();
            assert!(
                (h - true_h).abs() <= 1.0,
                "inverted {h} km vs true {true_h} km"
            );
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
        let h = invert_depth_multi(&deltas, &lags).unwrap();
        assert!(
            (h - true_h).abs() <= 10.0,
            "with +0.5 s bias, inverted {h} km"
        );
    }

    #[test]
    fn invert_single_recovers_the_catalog_depth() {
        for true_h in [35.0, 50.0, 100.0, 200.0] {
            let delta = 45.0;
            let lag = p_p_lag(delta, true_h).unwrap();
            let h = invert_depth_single(delta, lag).unwrap();
            assert!(
                (h - true_h).abs() <= 1.0,
                "inverted {h} km vs true {true_h} km"
            );
        }
    }

    #[test]
    fn invert_single_resolves_between_the_deep_grid_nodes() {
        let true_h = 231.0;
        let delta = 45.0;
        let lag = p_p_lag(delta, true_h).unwrap();
        let h = invert_depth_single(delta, lag).unwrap();
        assert!(
            (h - true_h).abs() <= 1.0,
            "a depth between the 200 and 250 km nodes must invert to {true_h} km, got {h}"
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
    }
}
