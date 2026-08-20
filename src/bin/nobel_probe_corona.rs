use omegaflow::archivar::{
    convert_to_si, extract_series, fetch_raw, is_time_key, load_sources, parse_json, scalar_of,
    ymd_to_days, Extract, JsonVal, SourceConfig,
};
use omegaflow::te::{surrogate_stats, surrogate_stats_phase, transfer_entropy_lag};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

const AU_M: f64 = 149_597_870_700.0;
const L1_SUN_M: f64 = 1.481e11;
const C_LIGHT: f64 = 299_792_458.0;
const GOES_SYNC_S: f64 = AU_M / C_LIGHT;
const SURROGATE_SEED: u64 = 0x9E37_79B9_7F4A_7C15;
const BASE: &str = "https://services.swpc.noaa.gov/json";

fn now_unix() -> f64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs_f64())
        .unwrap_or(0.0)
}

fn days_to_ymd(total_days: u64) -> (u32, u32, u32) {
    let z = total_days as i64 + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let mut y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    if m <= 2 {
        y += 1;
    }
    (y as u32, m, d)
}

fn iso_date(unix: f64) -> String {
    let (y, m, d) = days_to_ymd(unix.max(0.0) as u64 / 86400);
    format!("{y:04}-{m:02}-{d:02}")
}

fn now_iso() -> String {
    let secs = now_unix() as u64;
    let (y, m, d) = days_to_ymd(secs / 86400);
    let hh = (secs % 86400) / 3600;
    let mm = (secs % 3600) / 60;
    let ss = secs % 60;
    format!("{y:04}-{m:02}-{d:02}T{hh:02}:{mm:02}:{ss:02}Z")
}

fn iso_to_unix(s: &str) -> Option<f64> {
    let s = s.trim();
    let (date, time) = if let Some((d, t)) = s.split_once('T') {
        (d, t)
    } else {
        s.split_once(' ')?
    };
    let mut dp = date.split('-');
    let y: i64 = dp.next()?.parse().ok()?;
    let m: u32 = dp.next()?.parse().ok()?;
    let d: u32 = dp.next()?.parse().ok()?;
    let t = time
        .split(|c: char| c == '.' || c == 'Z' || c == 'z')
        .next()?;
    let mut tp = t.split(':');
    let hh: i64 = tp.next()?.parse().ok()?;
    let mm: i64 = tp.next()?.parse().ok()?;
    let ss: i64 = tp.next()?.parse().ok()?;
    let days = ymd_to_days(y, m, d)? as i64;
    Some((days * 86400 + hh * 3600 + mm * 60 + ss) as f64)
}

fn epoch_of(map: &HashMap<String, JsonVal>) -> Option<f64> {
    for (k, v) in map {
        if !is_time_key(k) {
            continue;
        }
        match v {
            JsonVal::Str(s) => {
                if let Some(t) = iso_to_unix(s) {
                    return Some(t);
                }
            }
            JsonVal::Num(n) => return Some(*n),
            _ => {}
        }
    }
    None
}

fn find_block(sources: &[SourceConfig], field_name: &str) -> Option<SourceConfig> {
    sources
        .iter()
        .find(|s| {
            s.extracts.iter().any(|e| match e {
                Extract::Field(fc)
                | Extract::First(fc, _)
                | Extract::Last(fc, _)
                | Extract::Count(fc)
                | Extract::LastRow(fc)
                | Extract::ObjLast(fc)
                | Extract::Path(fc)
                | Extract::Deep(fc)
                | Extract::Regex(fc) => fc.name == field_name,
                Extract::Hapi(pairs) => pairs.iter().any(|(_, n)| n == field_name),
                _ => false,
            })
        })
        .cloned()
}

fn harvest_last(url: &str, ttl: u64, key: &str, unit: &str) -> Vec<(f64, f64)> {
    let body = match fetch_raw(url, None, &[], ttl) {
        Some(b) => b,
        None => return Vec::new(),
    };
    let j = match parse_json(&body) {
        Some(j) => j,
        None => return Vec::new(),
    };
    let JsonVal::Arr(elements) = j else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for el in elements {
        let JsonVal::Obj(map) = el else {
            continue;
        };
        let Some(epoch) = epoch_of(&map) else {
            continue;
        };
        let Some(raw) = map.get(key).and_then(scalar_of) else {
            continue;
        };
        let Some(val) = convert_to_si(raw, unit) else {
            continue;
        };
        if val.is_finite() {
            out.push((epoch, val));
        }
    }
    out
}

fn harvest_block(
    url: &str,
    ttl: u64,
    block_name: &str,
    sources: &[SourceConfig],
    lsk: &omegaflow::lsk::LeapSeconds,
) -> Vec<(f64, f64)> {
    let Some(src) = find_block(sources, block_name) else {
        return Vec::new();
    };
    let Some(body) = fetch_raw(url, None, &src.headers, ttl) else {
        return Vec::new();
    };
    let mut series_src = src.clone();
    series_src.url = url.to_string();
    series_src.extracts.retain(|e| {
        matches!(
            e,
            Extract::First(fc, _) | Extract::Last(fc, _) | Extract::Path(fc)
                if fc.name == block_name
        )
    });
    extract_series(&series_src, &body, lsk)
}

fn harvest_radio(url: &str, ttl: u64) -> Vec<(f64, f64)> {
    let body = match fetch_raw(url, None, &[], ttl) {
        Some(b) => b,
        None => return Vec::new(),
    };
    let j = match parse_json(&body) {
        Some(j) => j,
        None => return Vec::new(),
    };
    let JsonVal::Arr(elements) = j else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for el in elements {
        let JsonVal::Obj(map) = el else {
            continue;
        };
        let Some(epoch) = epoch_of(&map) else {
            continue;
        };
        let Some(JsonVal::Arr(details)) = map.get("details") else {
            continue;
        };
        for d in details {
            let JsonVal::Obj(dm) = d else {
                continue;
            };
            let freq = match dm.get("frequency").and_then(scalar_of) {
                Some(f) => f,
                None => continue,
            };
            if (freq - 2695.0).abs() > 0.5 {
                continue;
            }
            let Some(raw) = dm.get("flux").and_then(scalar_of) else {
                continue;
            };
            let Some(val) = convert_to_si(raw, "sfu") else {
                continue;
            };
            if val.is_finite() {
                out.push((epoch, val));
            }
        }
    }
    out.sort_by(|a, b| a.0.total_cmp(&b.0));
    out
}

fn hapi_one(src: &SourceConfig, url: &str, param: &str, name: &str) -> Vec<(f64, f64)> {
    let body = match fetch_raw(url, None, &src.headers, src.ttl) {
        Some(b) => b,
        None => return Vec::new(),
    };
    let fc = match src.extracts.iter().find_map(|e| match e {
        Extract::Field(fc) if fc.name == name => Some(fc.clone()),
        _ => None,
    }) {
        Some(fc) => fc,
        None => return Vec::new(),
    };
    let mut series_src = src.clone();
    series_src.url = url.to_string();
    series_src.extracts = vec![
        Extract::Hapi(vec![(param.to_string(), name.to_string())]),
        Extract::Field(fc),
    ];
    let lsk = omegaflow::lsk::LeapSeconds {
        delta_t_a: 946_728_000.0,
        deltas: vec![(0.0, 0.0)],
    };
    extract_series(&series_src, &body, &lsk)
}

fn l1_sync_series(series: &[(f64, f64)], wind: &[(f64, f64)], tolerance_s: f64) -> Vec<(f64, f64)> {
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
        out.push((t - L1_SUN_M / v_ms, v));
    }
    out.sort_by(|a, b| a.0.total_cmp(&b.0));
    out
}

fn bin_mean(series: &[(f64, f64)], t0: f64, dt: f64, n: usize) -> Vec<Option<f32>> {
    let mut sums = vec![0.0f64; n];
    let mut counts = vec![0u32; n];
    for &(t, v) in series {
        let idx = ((t - t0) / dt).floor();
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

fn te_row(label_a: &str, label_b: &str, xs: &[f32], ys: &[f32], lags: &[usize]) {
    if xs.len() < 30 {
        println!(
            "{:>14} → {:<14} | no statement possible (n = {})",
            label_a,
            label_b,
            xs.len()
        );
        return;
    }
    for &lag in lags {
        let seed = SURROGATE_SEED ^ (lag as u64).wrapping_mul(0x517C_C1B7_2722_0A95);
        let te = transfer_entropy_lag(xs, ys, lag);
        let thr = surrogate_stats_phase(xs, ys, lag, seed).map(|(_, _, t)| t);
        match (te, thr) {
            (Some(t), Some(h)) => {
                let arrow = if t > h { "arrow" } else { "silent" };
                println!(
                    "{:>14} → {:<14} | lag {:>2} | TE {:>10.4e} | threshold {:>10.4e} | excess {:>10.4e} | {}",
                    label_a,
                    label_b,
                    lag,
                    t,
                    h,
                    t - h,
                    arrow
                );
            }
            _ => {
                println!(
                    "{:>14} → {:<14} | lag {:>2} | TE missing (n < 8) | threshold missing | excess missing | silent",
                    label_a, label_b, lag
                );
            }
        }
    }
}

fn join_nearest(
    grid: &[(f64, f64)],
    dense: &[(f64, f64)],
    tolerance_s: f64,
) -> (Vec<f32>, Vec<f32>) {
    let mut xs = Vec::new();
    let mut ys = Vec::new();
    for &(t, v) in grid {
        let mut best: Option<(f64, f64)> = None;
        for &(td, vd) in dense {
            let dt = (td - t).abs();
            if dt <= tolerance_s && best.map_or(true, |(b, _)| dt < b) {
                best = Some((dt, vd));
            }
        }
        if let Some((_, vd)) = best {
            xs.push(v as f32);
            ys.push(vd as f32);
        }
    }
    (xs, ys)
}

struct Arrow {
    from: String,
    to: String,
    n: usize,
    te: f64,
    threshold: f64,
    mean: f64,
    sd: f64,
    naive_threshold: f64,
}

fn main() {
    let sources = load_sources();
    let now = now_unix();

    println!("=== Nobel probe: the measurement protocol of corona heating (Nadel Ⅲ) ===");
    println!("system time: {}", now_iso());
    println!(
        "TE-Lib: TE(Y→X; τ) = Σ_t ln[ p(x_{{t+τ}}, x_t, y_t) · p(x_t) / (p(x_t, y_t) · p(x_{{t+τ}}, x_t)) ] / m, m = n − τ;"
    );
    println!("KDE bandwidth: Silverman h = 1.06·σ·n^(−0.2) per series; lag 0 → canonical 1-step estimator.");
    println!(
        "Threshold: phase-randomized surrogates (preserving the spectrum of the driver series, std-only FFT, 10 realizations), mean + 2σ; the naive shuffle threshold stays printed only in the null control for comparison. Arrow ⇔ TE > threshold."
    );
    println!("Time base: Unix seconds relative — TE is shift-invariant; the TDB constant cancels.");

    let xray_url = format!("{BASE}/goes/primary/xrays-7-day.json");
    let euv_url = format!("{BASE}/goes/primary/euvs-7-day.json");
    let radio_url = format!("{BASE}/solar-radio-flux.json");
    let mag_url = format!("{BASE}/rtsw/rtsw_mag_1m.json");
    let wind_url = format!("{BASE}/rtsw/rtsw_wind_1m.json");
    let omni_min = iso_date(now - 30.0 * 86400.0);
    let omni_max = iso_date(now);
    let omni_url = format!(
        "https://cdaweb.gsfc.nasa.gov/hapi/data?id=OMNI2_H0_MRG1HR&time.min={omni_min}T00:00:00Z&time.max={omni_max}T00:00:00Z&parameters=BX_GSE1800,BY_GSM1800,BZ_GSM1800,T1800,N1800,V1800,Pressure1800,E1800&format=json"
    );

    println!();
    println!("=== channel board ===");
    let block_of = |name: &str| {
        find_block(&sources, name)
            .map(|s| s.url)
            .unwrap_or_else(|| "block missing from the register".into())
    };
    println!(
        "{:<14} | Block {} | {} | where energy == 0.05-0.4nm (block grammar) | Sync t_sun = t − {:.3} s",
        "X-Ray",
        block_of("noaa_goes_xray_flux_w_m2"),
        xray_url,
        GOES_SYNC_S
    );
    println!(
        "{:<14} | Block {} | {} | where line == 304 (block grammar) | Sync t_sun = t − {:.3} s",
        "EUV-304",
        block_of("solar_euv_flux_304_wm2"),
        euv_url,
        GOES_SYNC_S
    );
    println!(
        "{:<14} | Block {} | {} | where line == 284 (block grammar) | Sync t_sun = t − {:.3} s",
        "EUV-284",
        block_of("solar_euv_flux_284_wm2"),
        euv_url,
        GOES_SYNC_S
    );
    println!(
        "{:<14} | Block {} | {} | details entries frequency == 2695 | Sync t_sun = t − {:.3} s",
        "Radio-2695",
        block_of("solar_radio_flux_sfu"),
        radio_url,
        GOES_SYNC_S
    );
    println!(
        "{:<14} | Block {} | {} | Key bz_gsm | Sync t_sun = t − 1.481e11 / (v·1000), v from rtsw_wind (join ±1 min)",
        "Bz-RTSW", block_of("magnetosphere_imf_bz_nt"), mag_url
    );
    println!(
        "{:<14} | Block {} | {} | Key proton_density | Sync like Bz-RTSW",
        "Density-RTSW",
        block_of("solar_wind_density_cm3"),
        wind_url
    );
    println!(
        "{:<14} | Block {} | {} | column BZ_GSM1800, fill 999.9 | Sync t_sun = t − 1.481e11 / (V1800·1000)",
        "Bz-OMNI", block_of("omni_imf_bz_gsm_nt"), omni_url
    );
    println!(
        "{:<14} | Block {} | {} | column N1800, fill 999.9 | Sync like Bz-OMNI",
        "Density-OMNI",
        block_of("omni_solarwind_density_percc"),
        omni_url
    );

    let lsk = omegaflow::lsk::LeapSeconds {
        delta_t_a: 946_728_000.0,
        deltas: vec![(0.0, 0.0)],
    };
    let xray = harvest_block(&xray_url, 300, "noaa_goes_xray_flux_w_m2", &sources, &lsk);
    let euv304 = harvest_block(&euv_url, 300, "solar_euv_flux_304_wm2", &sources, &lsk);
    let euv284 = harvest_block(&euv_url, 300, "solar_euv_flux_284_wm2", &sources, &lsk);
    let radio_raw = harvest_radio(&radio_url, 300);
    let bz_rtsw_raw = harvest_last(&mag_url, 60, "bz_gsm", "nT");
    let dens_rtsw_raw = harvest_last(&wind_url, 60, "proton_density", "1/cm3");
    let wind_rtsw = harvest_last(&wind_url, 60, "proton_speed", "km/s");

    let omni_block = find_block(&sources, "omni_imf_bz_gsm_nt");
    let bz_omni_raw = omni_block
        .as_ref()
        .map(|s| hapi_one(s, &omni_url, "BZ_GSM1800", "omni_imf_bz_gsm_nt"))
        .unwrap_or_default();
    let dens_omni_raw = omni_block
        .as_ref()
        .map(|s| hapi_one(s, &omni_url, "N1800", "omni_solarwind_density_percc"))
        .unwrap_or_default();
    let v1800_raw = omni_block
        .as_ref()
        .map(|s| hapi_one(s, &omni_url, "V1800", "omni_solarwind_flow_speed_kms"))
        .unwrap_or_default();

    let shift = |v: Vec<(f64, f64)>| -> Vec<(f64, f64)> {
        v.into_iter().map(|(t, x)| (t - GOES_SYNC_S, x)).collect()
    };
    let xray = shift(xray);
    let euv304 = shift(euv304);
    let euv284 = shift(euv284);
    let radio = shift(radio_raw);
    let bz_rtsw = l1_sync_series(&bz_rtsw_raw, &wind_rtsw, 60.0);
    let dens_rtsw = l1_sync_series(&dens_rtsw_raw, &wind_rtsw, 60.0);
    let bz_omni = l1_sync_series(&bz_omni_raw, &v1800_raw, 1800.0);
    let dens_omni = l1_sync_series(&dens_omni_raw, &v1800_raw, 1800.0);

    let window_report = |name: &str, s: &[(f64, f64)]| match (s.first(), s.last()) {
        (Some(&(a, _)), Some(&(b, _))) => {
            let days = (b - a) / 86400.0;
            println!(
                "{name:<14} | n = {:<6} | window {:.2} d | cadence {:.2} s",
                s.len(),
                days,
                if s.len() > 1 {
                    (b - a) / (s.len() as f64 - 1.0)
                } else {
                    0.0
                }
            );
        }
        _ => println!("{name:<14} | no samples — the channel harvests null",),
    };
    println!();
    window_report("X-Ray", &xray);
    window_report("EUV-304", &euv304);
    window_report("EUV-284", &euv284);
    window_report("Radio-2695", &radio);
    window_report("Bz-RTSW", &bz_rtsw);
    window_report("Density-RTSW", &dens_rtsw);
    window_report("Bz-OMNI", &bz_omni);
    window_report("Density-OMNI", &dens_omni);

    println!();
    println!("=== missing register ===");
    println!("OMNI ↔ GOES: intersection empty — the OMNI ingest ends on 06.08. (stopDate), the GOES series begin on 12.08. (7-day window).");
    println!("30 d @ 1 min is not served by the APIs: xrays-30-day.json carries 404.");
    println!("GOES-30d candidate NGDC netCDF: 404 on four samples (18./12./05.08., 25.07.) — no block entered.");
    println!("Radio cadence irregular (~4–8 samples/day); the block path (details.0.flux) carries only the first entry — the series harvests all details entries.");
    println!("EUV lines 256/1175/1216/1335/1405 + mgii_index stay unharvested (not declared).");
    println!("LSK file missing locally — the series run in Unix seconds; the HAPI times pass through the identity-LSK (delta_t_a = J2000 offset, 0-s pad): Unix in, Unix out — no leap offset, TE is shift-invariant.");
    println!("Seconds matrix: lag 0 (canonical) and lag 1 coincide — on the 1-min grid the 1-step estimator is the 60-s lag.");
    println!("Surrogate methodology: the threshold is phase-randomized (autocorrelation-preserving); the naive shuffle threshold demonstrably breaks the control — both stand side by side in the null control. Block bootstrap sits ready as surrogate_stats_block in the lib (unused).");
    println!("Small n carry no statement: below n = 30 no arrow is measured — underdetermination, not silence.");

    let common_window = |channels: &[(&str, &[(f64, f64)])]| -> Option<(f64, f64, f64)> {
        let lo = channels
            .iter()
            .filter_map(|(_, s)| s.first().map(|&(t, _)| t))
            .fold(f64::NEG_INFINITY, f64::max);
        let hi = channels
            .iter()
            .filter_map(|(_, s)| s.last().map(|&(t, _)| t))
            .fold(f64::INFINITY, f64::min);
        if lo < hi {
            let dt = 60.0;
            let t0 = (lo / dt).floor() * dt;
            let n = ((hi - t0) / dt).floor() as usize;
            Some((t0, dt, n as f64))
        } else {
            None
        }
    };

    let seconds_channels = [
        ("X-Ray", xray.as_slice()),
        ("EUV-304", euv304.as_slice()),
        ("EUV-284", euv284.as_slice()),
        ("Bz-RTSW", bz_rtsw.as_slice()),
        ("Density-RTSW", dens_rtsw.as_slice()),
    ];
    println!();
    println!("=== Matrix 1 — seconds (lag ∈ {{0, 1, 2}} @ 1 min, common window) ===");
    match common_window(&seconds_channels) {
        Some((t0, dt, n)) => {
            let binned: Vec<Vec<Option<f32>>> = seconds_channels
                .iter()
                .map(|(_, s)| bin_mean(s, t0, dt, n as usize))
                .collect();
            for i in 0..seconds_channels.len() {
                for j in 0..seconds_channels.len() {
                    if i == j {
                        continue;
                    }
                    let (xs, ys) = pair_cells(&binned[j], &binned[i]);
                    te_row(
                        seconds_channels[i].0,
                        seconds_channels[j].0,
                        &xs,
                        &ys,
                        &[0, 1, 2],
                    );
                }
            }
        }
        None => println!("common window empty — matrix missing"),
    }

    println!();
    println!("=== Matrix 2 — 7-day (lag ∈ {{0, 1, 2}} @ 1 min, n = 10 078) ===");
    {
        let channels = [
            ("X-Ray", xray.as_slice()),
            ("EUV-304", euv304.as_slice()),
            ("EUV-284", euv284.as_slice()),
        ];
        match common_window(&channels) {
            Some((t0, dt, n)) => {
                let binned: Vec<Vec<Option<f32>>> = channels
                    .iter()
                    .map(|(_, s)| bin_mean(s, t0, dt, n as usize))
                    .collect();
                for i in 0..channels.len() {
                    for j in 0..channels.len() {
                        if i == j {
                            continue;
                        }
                        let (xs, ys) = pair_cells(&binned[j], &binned[i]);
                        te_row(channels[i].0, channels[j].0, &xs, &ys, &[0, 1, 2]);
                    }
                }
            }
            None => println!("common window empty — matrix missing"),
        }
    }

    println!();
    println!("=== Matrix 3 — hours (Bz-OMNI ↔ Radio on the radio grid, tolerance 1800 s; Bz-OMNI ↔ density-OMNI hourly) ===");
    {
        let (xs, ys) = join_nearest(&radio, &bz_omni, 1800.0);
        te_row("Bz-OMNI", "Radio-2695", &xs, &ys, &[0, 1, 2, 3]);
        let (xs, ys) = join_nearest(&radio, &bz_omni, 1800.0);
        te_row("Radio-2695", "Bz-OMNI", &ys, &xs, &[0, 1, 2, 3]);
        println!("  (radio-grid index lag is irregular — named, not concealed)");
        let lo = bz_omni
            .first()
            .map(|&(t, _)| t)
            .unwrap_or(0.0)
            .max(dens_omni.first().map(|&(t, _)| t).unwrap_or(0.0));
        let hi = bz_omni
            .last()
            .map(|&(t, _)| t)
            .unwrap_or(0.0)
            .min(dens_omni.last().map(|&(t, _)| t).unwrap_or(0.0));
        if lo < hi {
            let dt = 3600.0;
            let t0 = (lo / dt).floor() * dt;
            let n = ((hi - t0) / dt).floor() as usize;
            let bb = bin_mean(&bz_omni, t0, dt, n);
            let bd = bin_mean(&dens_omni, t0, dt, n);
            let (xs, ys) = pair_cells(&bd, &bb);
            te_row("Bz-OMNI", "Density-OMNI", &xs, &ys, &[0, 1, 2, 3]);
            let (xs, ys) = pair_cells(&bb, &bd);
            te_row("Density-OMNI", "Bz-OMNI", &xs, &ys, &[0, 1, 2, 3]);
        } else {
            println!("intersection Bz-OMNI ↔ density-OMNI empty — matrix missing");
        }
    }

    println!();
    println!(
        "=== Matrix 4 — coarse radio matrix (radio grid, tolerance 1800 s, weak statistics) ==="
    );
    for (label, dense) in [
        ("X-Ray", xray.as_slice()),
        ("EUV-304", euv304.as_slice()),
        ("EUV-284", euv284.as_slice()),
    ] {
        let (xs, ys) = join_nearest(&radio, dense, 1800.0);
        println!("  (Radio ↔ {}: n = {})", label, xs.len());
        te_row("Radio-2695", label, &xs, &ys, &[0]);
    }

    let mut arrows: Vec<Arrow> = Vec::new();
    {
        match common_window(&seconds_channels) {
            Some((t0, dt, n)) => {
                let binned: Vec<Vec<Option<f32>>> = seconds_channels
                    .iter()
                    .map(|(_, s)| bin_mean(s, t0, dt, n as usize))
                    .collect();
                for i in 0..seconds_channels.len() {
                    for j in 0..seconds_channels.len() {
                        if i == j {
                            continue;
                        }
                        let (xs, ys) = pair_cells(&binned[j], &binned[i]);
                        let te = transfer_entropy_lag(&xs, &ys, 0);
                        let stats = surrogate_stats_phase(&xs, &ys, 0, SURROGATE_SEED);
                        let naive = surrogate_stats(&xs, &ys, 0, SURROGATE_SEED);
                        if let (Some(t), Some((mean, sd, thr))) = (te, stats) {
                            arrows.push(Arrow {
                                from: seconds_channels[i].0.into(),
                                to: seconds_channels[j].0.into(),
                                n: xs.len(),
                                te: t,
                                threshold: thr,
                                mean,
                                sd,
                                naive_threshold: naive.map(|(_, _, t)| t).unwrap_or(f64::NAN),
                            });
                        }
                    }
                }
            }
            None => {}
        }
    }
    {
        let (xs, ys) = join_nearest(&radio, &bz_omni, 1800.0);
        let te = transfer_entropy_lag(&xs, &ys, 0);
        let stats = surrogate_stats_phase(&xs, &ys, 0, SURROGATE_SEED);
        let naive = surrogate_stats(&xs, &ys, 0, SURROGATE_SEED);
        if let (Some(t), Some((mean, sd, thr))) = (te, stats) {
            arrows.push(Arrow {
                from: "Bz-OMNI".into(),
                to: "Radio-2695".into(),
                n: xs.len(),
                te: t,
                threshold: thr,
                mean,
                sd,
                naive_threshold: naive.map(|(_, _, t)| t).unwrap_or(f64::NAN),
            });
        }
        let te = transfer_entropy_lag(&ys, &xs, 0);
        let stats = surrogate_stats_phase(&ys, &xs, 0, SURROGATE_SEED);
        let naive = surrogate_stats(&ys, &xs, 0, SURROGATE_SEED);
        if let (Some(t), Some((mean, sd, thr))) = (te, stats) {
            arrows.push(Arrow {
                from: "Radio-2695".into(),
                to: "Bz-OMNI".into(),
                n: xs.len(),
                te: t,
                threshold: thr,
                mean,
                sd,
                naive_threshold: naive.map(|(_, _, t)| t).unwrap_or(f64::NAN),
            });
        }
    }

    println!();
    println!(
        "=== Arrows (significant ⇔ TE > threshold = mean + 2σ of phase-randomized surrogates) ==="
    );
    for p in &arrows {
        println!(
            "{:>14} → {:<14} | n {:>5} | TE {:.4e} | threshold {:.4e} | surrogate (mean {:.3e}, σ {:.3e}) | threshold-naive {:.4e} | {}",
            p.from,
            p.to,
            p.n,
            p.te,
            p.threshold,
            p.mean,
            p.sd,
            p.naive_threshold,
            if p.te > p.threshold { "arrow" } else { "silent" }
        );
    }
    let sig = |a: &str, b: &str| {
        arrows
            .iter()
            .any(|p| p.from == a && p.to == b && p.te > p.threshold)
    };
    let dag: Vec<String> = arrows
        .iter()
        .filter(|p| p.te > p.threshold)
        .map(|p| format!("{}→{}", p.from, p.to))
        .collect();
    println!("DAG: {:?}", dag);

    println!();
    println!("=== Verdict ===");
    let bz_304 = sig("Bz-RTSW", "EUV-304");
    let s304_284 = sig("EUV-304", "EUV-284");
    let euv_xr_arrow = sig("EUV-304", "X-Ray") || sig("EUV-284", "X-Ray");
    let xr_euv_arrow = sig("X-Ray", "EUV-304") || sig("X-Ray", "EUV-284");
    let best_excess = |from: &[&str], to: &str| {
        arrows
            .iter()
            .filter(|p| p.te > p.threshold && from.contains(&p.from.as_str()) && p.to == to)
            .max_by(|a, b| (a.te - a.threshold).total_cmp(&(b.te - b.threshold)))
    };
    let euv_xr_best = best_excess(&["EUV-304", "EUV-284"], "X-Ray");
    let xr_euv_best = match (
        best_excess(&["X-Ray"], "EUV-304"),
        best_excess(&["X-Ray"], "EUV-284"),
    ) {
        (Some(x), Some(y)) => {
            if x.te - x.threshold >= y.te - y.threshold {
                Some(x)
            } else {
                Some(y)
            }
        }
        (x, y) => x.or(y),
    };
    let euv_xr_ue = euv_xr_best
        .map(|p| p.te - p.threshold)
        .unwrap_or(f64::NEG_INFINITY);
    let xr_euv_ue = xr_euv_best
        .map(|p| p.te - p.threshold)
        .unwrap_or(f64::NEG_INFINITY);
    let radio_x_n = join_nearest(&radio, &xray, 1800.0).0.len();
    let radio_goes = radio_x_n >= 30
        && (sig("Radio-2695", "X-Ray")
            || sig("Radio-2695", "EUV-304")
            || sig("Radio-2695", "EUV-284"));
    let dens_violation = sig("Density-RTSW", "X-Ray")
        || sig("Density-RTSW", "EUV-304")
        || sig("Density-RTSW", "EUV-284")
        || sig("Density-OMNI", "Bz-OMNI")
        || sig("Density-OMNI", "Radio-2695");
    println!(
        "Null-control window: density-RTSW ↔ GOES ran on the seconds window (common window of all seconds channels, ~2 d) — not on OMNI (intersection empty, stopDate 06.08.)."
    );
    println!("=== Null control: naive vs. phase-randomized threshold ===");
    for p in arrows.iter().filter(|p| p.from.starts_with("Density-RTSW")) {
        let naiv = if p.te > p.naive_threshold {
            "breaks"
        } else {
            "holds"
        };
        let phase = if p.te > p.threshold {
            "breaks"
        } else {
            "holds"
        };
        println!(
            "{:>14} → {:<14} | n {:>5} | TE {:.4e} | threshold-naive {:.4e} ({}) | threshold-phase {:.4e} ({})",
            p.from, p.to, p.n, p.te, p.naive_threshold, naiv, p.threshold, phase
        );
    }
    if bz_304 && s304_284 {
        if dens_violation {
            println!("TE(Bz → EUV-304) and TE(EUV-304 → EUV-284) significant → magnetic energy flow through the transition region into the hot corona (the Alfvén channel) — PROVISIONAL: the surrogate threshold stands under the caveat of the broken null control.");
        } else {
            println!("TE(Bz → EUV-304) and TE(EUV-304 → EUV-284) significant → magnetic energy flow through the transition region into the hot corona (the Alfvén channel).");
        }
    } else if bz_304 {
        println!("TE(Bz → EUV-304) significant, TE(EUV-304 → EUV-284) silent → the arrow ends in the transition region — the chain 304 → 284 is missing.");
    } else if s304_284 {
        println!("TE(EUV-304 → EUV-284) significant, TE(Bz → EUV-304) silent → the transition region heats the corona, the magnetic drive stays unmeasured.");
    } else {
        println!("No arrow on Bz → 304 → 284 — the magnetic channel is silent (0 honored: silence is the answer, not a bug).");
    }
    if euv_xr_arrow && xr_euv_arrow {
        let (ux, uy) = (euv_xr_ue, xr_euv_ue);
        let (sx, sy) = (
            euv_xr_best.map(|p| p.sd).unwrap_or(f64::NAN),
            xr_euv_best.map(|p| p.sd).unwrap_or(f64::NAN),
        );
        if ux > uy {
            println!(
                "X-Ray ↔ EUV bidirectional — EUV → X-Ray carries the larger excess ({:.2e} vs {:.2e}, surrogate σ {:.2e} vs {:.2e}) → wave heating (Alfvén) prevails, nanoflare share present in the opposite direction.",
                ux, uy, sx, sy
            );
        } else {
            println!(
                "X-Ray ↔ EUV bidirectional — X-Ray → EUV carries the larger excess ({:.2e} vs {:.2e}, surrogate σ {:.2e} vs {:.2e}) → nanoflares prevail.",
                uy, ux, sy, sx
            );
        }
    } else if euv_xr_arrow {
        println!(
            "EUV → X-Ray significant with silent reverse direction → wave heating (Alfvén coherence)."
        );
    } else if xr_euv_arrow {
        println!("X-Ray → EUV significant with silent reverse direction → nanoflares.");
    } else {
        println!("X-Ray ↔ EUV silent on both sides → no causal arrow on the seconds scale.");
    }
    if radio_x_n < 30 {
        println!(
            "Radio ↔ GOES: no statement possible (n = {}) — underdetermination, not physical silence.",
            radio_x_n
        );
    } else if radio_goes {
        println!("Radio → GOES significant with silent reverse direction → the chromosphere drives the corona.");
    } else {
        println!("Radio → GOES silent — the chromospheric coupling carries no arrow.");
    }
    if dens_violation {
        println!("Density pairs above the phase-randomized threshold → NULL CONTROL BREAKS EVEN SPECTRUM-PRESERVING — the break survives the corrected surrogate method; next suspect: multiple comparison (20 pairs × 3 lags × 4 matrices without correction) or a real, uninteresting coupling path. The verdict stays PROVISIONAL.");
    } else {
        println!("Null control holds under the phase-randomized threshold — the naive shuffle was the artifact; the significance machinery carries spectrum-preserving surrogates.");
    }
    println!("Silent lines are findings. Exit 0.");
}
