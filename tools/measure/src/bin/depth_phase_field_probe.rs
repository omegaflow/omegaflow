use omegaflow::ak135::{p_p_travel, p_travel_depth, s_p_travel};
use omegaflow::archivar::{fetch_raw, fetch_raw_bytes, parse_json, scalar_of, JsonVal};
use omegaflow::json::jpath;
use omegaflow_measure::miniseed::decode_body;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

const DATASELECT_ROUTE: &str = "https://service.earthscope.org/fdsnws/dataselect/1/query";
const CATALOG_URL: &str = "https://earthquake.usgs.gov/fdsnws/event/1/query";
const STATION_URL: &str = "https://service.iris.edu/fdsnws/station/1/query";
const GEBCO_URL: &str = "https://api.opentopodata.org/v1/gebco2020?locations=";

const MIN_DEPTH_KM: f64 = 35.0;
const MIN_MAG: f64 = 6.0;
const REGION: [f64; 4] = [34.0, 38.0, 68.0, 74.0];
const SEARCH_START: &str = "2000-01-01T00:00:00";
const MIN_DIST_DEG: f64 = 30.0;
const MAX_DIST_DEG: f64 = 90.0;
const MAX_STATIONS: usize = 12;
const P_WINDOW_AFTER_ORIGIN_S: f64 = 1500.0;
const SNR_GATE: f64 = 3.0;
const P_WAVELET_S: f64 = 4.0;
const WINDOW_LO: f64 = 0.7;
const WINDOW_HI: f64 = 1.3;
const EDGE_FRAC: f64 = 0.15;
const SECONDARY_CORR_MIN: f64 = 0.30;
const DEPTH_MATCH_GATE_KM: f64 = 10.0;
const PI: f64 = std::f64::consts::PI;
const STA_WINDOW_S: f64 = 1.0;
const LTA_WINDOW_S: f64 = 30.0;
const STA_LTA_RATIO: f64 = 4.0;

const DEPTHS_FINE: [f64; 21] = [
    0.0, 5.0, 10.0, 15.0, 20.0, 25.0, 30.0, 35.0, 40.0, 45.0, 50.0, 60.0, 70.0, 80.0, 90.0, 100.0,
    150.0, 200.0, 250.0, 300.0, 400.0,
];

#[derive(Clone)]
struct Event {
    id: String,
    t0: f64,
    mag: f64,
    lat: f64,
    lon: f64,
    depth_km: f64,
}

#[derive(Clone)]
struct Station {
    net: String,
    sta: String,
    lat: f64,
    lon: f64,
}

struct SecondaryPick {
    lag_s: f64,
    corr: f64,
    inverted: bool,
    at_edge: bool,
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

fn haversine_km(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let r = 6371.0;
    let dlat = (lat2 - lat1).to_radians();
    let dlon = (lon2 - lon1).to_radians();
    let a = (dlat / 2.0).sin().powi(2)
        + lat1.to_radians().cos() * lat2.to_radians().cos() * (dlon / 2.0).sin().powi(2);
    2.0 * r * a.sqrt().atan2((1.0 - a).sqrt())
}

fn arc_deg(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    haversine_km(lat1, lon1, lat2, lon2) / 111.195
}

fn catalog_events(body: &str) -> Vec<Event> {
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

fn parse_stations_text(body: &str) -> Vec<Station> {
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

fn gebco_elevation(lat: f64, lon: f64) -> Option<f64> {
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

fn bandpass(samples: &[(f64, f64)], rate: f64) -> Vec<f64> {
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

fn p_onset(samples: &[(f64, f64)], rate: f64) -> Option<f64> {
    first_break_arrival(samples, rate).or_else(|| sta_lta_arrival(samples, rate))
}

fn onset_index(samples: &[(f64, f64)], rate: f64, t_p: f64) -> usize {
    let Some(&(t0, _)) = samples.first() else {
        return 0;
    };
    if t_p <= t0 {
        return 0;
    }
    ((t_p - t0) * rate).round() as usize
}

fn onset_snr(bp: &[f64], rate: f64, i_p: usize) -> Option<f64> {
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

fn correlate_window(
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

fn invert_depth_single(delta_deg: f64, lag: f64) -> Option<f64> {
    let mut best = (f64::INFINITY, 0.0f64);
    for &h in DEPTHS_FINE.iter() {
        if let Some(pred) =
            p_p_travel(delta_deg, h).and_then(|pp| p_travel_depth(delta_deg, h).map(|p| pp - p))
        {
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

fn median(xs: &mut [f64]) -> f64 {
    xs.sort_by(|a, b| a.total_cmp(b));
    xs[xs.len() / 2]
}

fn fetch_station_body(station: &Station, start: &str, end: &str) -> Option<(Vec<(f64, f64)>, f64)> {
    let url = format!(
        "{DATASELECT_ROUTE}?network={}&station={}&channel=BHZ&starttime={}&endtime={}&format=miniseed",
        station.net, station.sta, start, end
    );
    let body = fetch_raw_bytes(&url, 3600)?;
    decode_body(&body)
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let search_end = match arg_value(&args, "--end") {
        Some(v) => v,
        None => match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(d) => unix_to_iso(d.as_secs_f64()),
            Err(_) => {
                eprintln!("depth-phase field probe: the system clock precedes the epoch — no end time, no fabricated zero");
                return;
            }
        },
    };

    let lag_5km = p_p_travel(45.0, 15.0).and_then(|pp| p_travel_depth(45.0, 10.0).map(|p| pp - p));
    let depth_precision_km = lag_5km.map(|l| 5.0 / l * 2.0);

    println!(
        "=== depth-phase field pilot — the pP/sP lag reads the depth from a real deep event ==="
    );
    println!("selection rule (registered before the first fetch):");
    println!(
        "  depth >= {MIN_DEPTH_KM} km, magnitude >= {MIN_MAG}, land epicenter (Hindu Kush box lat {}..{} lon {}..{}),",
        REGION[0], REGION[1], REGION[2], REGION[3]
    );
    println!(
        "  station distance band {MIN_DIST_DEG}..{MAX_DIST_DEG} deg, SNR gate >= {SNR_GATE} at the P onset BEFORE the pP window"
    );
    println!();
    match (lag_5km, depth_precision_km) {
        (Some(l), Some(d)) => println!(
            "error budget (before the run): the lag carries the onset pick (~2 s scatter); {l:.2} s of lag per 5 km -> expected depth precision ~ +/- {d:.1} km at a single event"
        ),
        _ => println!(
            "error budget: the ak135 45-deg depth leg stays uncomputed — no fabricated precision"
        ),
    }
    println!(
        "match gate: catalog depth uncertainty ~ +/- {DEPTH_MATCH_GATE_KM} km; outside the gate = two readings (our scatter or the catalog)"
    );
    println!(
        "polarity witness: at the free surface pP arrives inverted; a negative correlation peak is the pP signature, a positive-only peak is suspicious (coda)"
    );
    println!();

    let cat_url = format!(
        "{CATALOG_URL}?format=geojson&starttime={SEARCH_START}&endtime={search_end}&minmagnitude={MIN_MAG}&mindepth={MIN_DEPTH_KM}&minlatitude={}&maxlatitude={}&minlongitude={}&maxlongitude={}&orderby=magnitude&limit=100",
        REGION[0], REGION[1], REGION[2], REGION[3]
    );
    let Some(body) = fetch_raw(&cat_url, None, &[], 86400) else {
        eprintln!("catalog carries no body — the channel stays unmeasured (0 honored)");
        return;
    };
    let events = catalog_events(&body);
    if events.is_empty() {
        eprintln!("no registered deep event in the region box — nothing measured (0 honored)");
        return;
    }
    let event = &events[0];
    println!(
        "event {}: M{:.1} at {:.4} N {:.4} E, catalog depth {:.1} km, origin {}",
        event.id,
        event.mag,
        event.lat,
        event.lon,
        event.depth_km,
        unix_to_iso(event.t0)
    );
    match gebco_elevation(event.lat, event.lon) {
        Some(e) => println!("  land witness: GEBCO surface elevation {e:.1} m"),
        None => println!("  land witness: GEBCO returned no elevation (the box is the land rule)"),
    }
    println!();

    let start = unix_to_iso(event.t0);
    let end = unix_to_iso(event.t0 + P_WINDOW_AFTER_ORIGIN_S);
    let st_url = format!(
        "{STATION_URL}?format=text&level=channel&latitude={:.4}&longitude={:.4}&minradius={MIN_DIST_DEG}&maxradius={MAX_DIST_DEG}&channel=BHZ&starttime={start}&endtime={end}&includerestricted=false",
        event.lat, event.lon
    );
    let Some(st_body) = fetch_raw(&st_url, None, &[], 86400) else {
        eprintln!("station query carries no body — the channel stays unmeasured (0 honored)");
        return;
    };
    let mut stations = parse_stations_text(&st_body);
    let mut seen = std::collections::HashSet::new();
    stations.retain(|s| seen.insert(format!("{}.{}", s.net, s.sta)));
    stations.sort_by(|a, b| {
        let da = arc_deg(event.lat, event.lon, a.lat, a.lon);
        let db = arc_deg(event.lat, event.lon, b.lat, b.lon);
        da.total_cmp(&db)
    });
    stations.truncate(MAX_STATIONS);
    println!(
        "{} BHZ stations in the {MIN_DIST_DEG}..{MAX_DIST_DEG} deg band",
        stations.len()
    );

    let mut picked = 0usize;
    let mut depths: Vec<f64> = Vec::new();
    let mut rows: Vec<String> = Vec::new();
    for st in &stations {
        let key = format!("{}.{}", st.net, st.sta);
        let delta = arc_deg(event.lat, event.lon, st.lat, st.lon);
        let Some((samples, rate)) = fetch_station_body(st, &start, &end) else {
            eprintln!("{key} carries no decodable record — skipped");
            continue;
        };
        let Some(t_p) = p_onset(&samples, rate) else {
            eprintln!("{key} carries no P pick — skipped");
            continue;
        };
        let bp = bandpass(&samples, rate);
        let i_p = onset_index(&samples, rate, t_p);
        let Some(snr) = onset_snr(&bp, rate, i_p) else {
            eprintln!("{key} carries no noise floor — skipped");
            continue;
        };
        if snr < SNR_GATE {
            eprintln!(
                "{key} P-onset SNR {snr:.1} below the {SNR_GATE} gate — skipped before the pP window"
            );
            continue;
        }
        let nw = (P_WAVELET_S * rate).round() as usize;
        if nw == 0 {
            eprintln!("{key} carries a degenerate rate — skipped");
            continue;
        }
        let p_p_lag_pred = match p_p_travel(delta, event.depth_km)
            .and_then(|pp| p_travel_depth(delta, event.depth_km).map(|p| pp - p))
        {
            Some(l) => l,
            None => {
                eprintln!("{key} delta {delta:.1} carries no ak135 pP prediction — skipped");
                continue;
            }
        };
        let pp = correlate_window(&bp, i_p, nw, rate, p_p_lag_pred);
        let pp_txt = match &pp {
            Some(p) => {
                let word = if p.corr.abs() >= SECONDARY_CORR_MIN {
                    "found"
                } else {
                    "weak"
                };
                format!(
                    "lag {:.1} s (pred {:.1} s) corr {:+.2} {} edge={}",
                    p.lag_s, p_p_lag_pred, p.corr, word, p.at_edge
                )
            }
            None => "window out of trace".to_string(),
        };
        let sp_txt = match s_p_travel(delta, event.depth_km)
            .and_then(|sp| p_travel_depth(delta, event.depth_km).map(|p| sp - p))
            .and_then(|sp_lag| correlate_window(&bp, i_p, nw, rate, sp_lag))
        {
            Some(p) if p.corr.abs() >= SECONDARY_CORR_MIN => {
                format!(
                    "lag {:.1} s corr {:+.2} inverted={}",
                    p.lag_s, p.corr, p.inverted
                )
            }
            Some(p) => format!("weak (corr {:+.2})", p.corr),
            None => "absent".to_string(),
        };
        let depth_txt = match &pp {
            Some(p) if p.corr.abs() >= SECONDARY_CORR_MIN => {
                match invert_depth_single(delta, p.lag_s) {
                    Some(h) => {
                        depths.push(h);
                        format!("{h:.0} km")
                    }
                    None => "inversion void".to_string(),
                }
            }
            _ => "-".to_string(),
        };
        picked += 1;
        rows.push(format!(
            "{key:>12}  delta {delta:>5.1}  snr {snr:>5.1}  pP {pp_txt}  sP {sp_txt}  depth {depth_txt}"
        ));
    }

    println!();
    println!("{picked} stations passed the SNR gate");
    for r in &rows {
        println!("{r}");
    }
    println!();
    if depths.is_empty() {
        println!("no station carried a pP above the correlation gate — no depth inverted (0 honored, two readings: phase too weak or the catalog depth is off by > 30%)");
        return;
    }
    let median_depth = median(&mut depths);
    let offset = median_depth - event.depth_km;
    let verdict = if offset.abs() <= DEPTH_MATCH_GATE_KM {
        "inside the match gate"
    } else {
        "outside the match gate (two readings: our scatter or the catalog)"
    };
    println!(
        "inverted depth: median {median_depth:.0} km ({} stations), catalog {:.1} km, offset {:+.1} km -> {verdict}",
        depths.len(),
        event.depth_km,
        offset
    );
    let edge_flags = rows.iter().filter(|r| r.contains("edge=true")).count();
    if edge_flags > 0 {
        println!("edge flag (pP pick at the window edge — the catalog depth may itself be off): {edge_flags} stations");
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

    #[test]
    fn invert_depth_single_recovers_the_catalog_depth() {
        for true_h in [35.0, 50.0, 100.0, 200.0] {
            let delta = 45.0;
            let lag = p_p_travel(delta, true_h).unwrap() - p_travel_depth(delta, true_h).unwrap();
            let h = invert_depth_single(delta, lag).unwrap();
            assert!(
                (h - true_h).abs() <= 10.0,
                "inverted {h} km vs true {true_h} km"
            );
        }
    }
}
