use omegaflow::archivar::{
    convert_to_si, extract_series, fetch_raw, is_time_key, load_sources, parse_json, scalar_of,
    ymd_to_days, Extract, JsonVal, SourceConfig,
};
use omegaflow::te::{surrogate_stats, surrogate_threshold_lag, transfer_entropy_lag};
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
                | Extract::First(fc)
                | Extract::Last(fc)
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

fn harvest_last(
    url: &str,
    ttl: u64,
    key: &str,
    unit: &str,
    filter: Option<(&str, &str)>,
) -> Vec<(f64, f64)> {
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
        if let Some((fk, fv)) = filter {
            match map.get(fk) {
                Some(JsonVal::Str(s)) if s == fv => {}
                _ => continue,
            }
        }
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
            "{:>14} → {:<14} | keine Aussage möglich (n = {})",
            label_a,
            label_b,
            xs.len()
        );
        return;
    }
    for &lag in lags {
        let seed = SURROGATE_SEED ^ (lag as u64).wrapping_mul(0x517C_C1B7_2722_0A95);
        let te = transfer_entropy_lag(xs, ys, lag);
        let thr = surrogate_threshold_lag(xs, ys, lag, seed);
        match (te, thr) {
            (Some(t), Some(h)) => {
                let arrow = if t > h { "Pfeil" } else { "still" };
                println!(
                    "{:>14} → {:<14} | lag {:>2} | TE {:>10.4e} | Schwelle {:>10.4e} | Überschuss {:>10.4e} | {}",
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
                    "{:>14} → {:<14} | lag {:>2} | TE fehlt (n < 8) | Schwelle fehlt | Überschuss fehlt | still",
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

struct Pfeil {
    von: String,
    nach: String,
    n: usize,
    te: f64,
    schwelle: f64,
    mean: f64,
    sd: f64,
}

fn main() {
    let sources = load_sources();
    let now = now_unix();

    println!("=== Nobel-Probe: Das Messprotokoll der Korona-Heizung (Nadel Ⅲ) ===");
    println!("Systemzeit: {}", now_iso());
    println!(
        "TE-Lib: TE(Y→X; τ) = Σ_t ln[ p(x_{{t+τ}}, x_t, y_t) · p(x_t) / (p(x_t, y_t) · p(x_{{t+τ}}, x_t)) ] / m, m = n − τ;"
    );
    println!("KDE-Bandbreite: Silverman h = 1.06·σ·n^(−0.2) je Reihe; lag 0 → kanonischer 1-Schritt-Schätzer.");
    println!(
        "Surrogate: 10 Fisher-Yates-Shuffles (LCG), Schwelle = mean + 2σ. Pfeil ⇔ TE > Schwelle."
    );
    println!("Zeitbasis: Unix-Sekunden relativ — TE ist verschiebungsinvariant; die TDB-Konstante kürzt sich.");

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
    println!("=== Kanal-Tafel ===");
    let block_of = |name: &str| {
        find_block(&sources, name)
            .map(|s| s.url)
            .unwrap_or_else(|| "Block fehlt im Register".into())
    };
    println!(
        "{:<14} | Block {} | {} | Filter energy == 0.05-0.4nm | Sync t_sun = t − {:.3} s",
        "X-Ray",
        block_of("noaa_goes_xray_flux_w_m2"),
        xray_url,
        GOES_SYNC_S
    );
    println!(
        "{:<14} | Block {} | {} | Filter line == 304 | Sync t_sun = t − {:.3} s",
        "EUV-304",
        block_of("solar_euv_flux_wm2"),
        euv_url,
        GOES_SYNC_S
    );
    println!(
        "{:<14} | Block {} | {} | Filter line == 284 | Sync t_sun = t − {:.3} s",
        "EUV-284",
        block_of("solar_euv_flux_wm2"),
        euv_url,
        GOES_SYNC_S
    );
    println!(
        "{:<14} | Block {} | {} | details-Einträge frequency == 2695 | Sync t_sun = t − {:.3} s",
        "Radio-2695",
        block_of("solar_radio_flux_sfu"),
        radio_url,
        GOES_SYNC_S
    );
    println!(
        "{:<14} | Block {} | {} | Key bz_gsm | Sync t_sun = t − 1.481e11 / (v·1000), v aus rtsw_wind (Join ±1 min)",
        "Bz-RTSW", block_of("magnetosphere_imf_bz_nt"), mag_url
    );
    println!(
        "{:<14} | Block {} | {} | Key proton_density | Sync wie Bz-RTSW",
        "Dichte-RTSW",
        block_of("solar_wind_density_cm3"),
        wind_url
    );
    println!(
        "{:<14} | Block {} | {} | Spalte BZ_GSM1800, Fill 999.9 | Sync t_sun = t − 1.481e11 / (V1800·1000)",
        "Bz-OMNI", block_of("omni_imf_bz_gsm_nt"), omni_url
    );
    println!(
        "{:<14} | Block {} | {} | Spalte N1800, Fill 999.9 | Sync wie Bz-OMNI",
        "Dichte-OMNI",
        block_of("omni_solarwind_density_percc"),
        omni_url
    );

    let xray = harvest_last(
        &xray_url,
        300,
        "flux",
        "W/m2",
        Some(("energy", "0.05-0.4nm")),
    );
    let euv304 = harvest_last(&euv_url, 300, "value", "W/m2", Some(("line", "304")));
    let euv284 = harvest_last(&euv_url, 300, "value", "W/m2", Some(("line", "284")));
    let radio_raw = harvest_radio(&radio_url, 300);
    let bz_rtsw_raw = harvest_last(&mag_url, 60, "bz_gsm", "nT", None);
    let dens_rtsw_raw = harvest_last(&wind_url, 60, "proton_density", "1/cm3", None);
    let wind_rtsw = harvest_last(&wind_url, 60, "proton_speed", "km/s", None);

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

    let fenster = |name: &str, s: &[(f64, f64)]| match (s.first(), s.last()) {
        (Some(&(a, _)), Some(&(b, _))) => {
            let days = (b - a) / 86400.0;
            println!(
                "{name:<14} | n = {:<6} | Fenster {:.2} d | Kadenz {:.2} s",
                s.len(),
                days,
                if s.len() > 1 {
                    (b - a) / (s.len() as f64 - 1.0)
                } else {
                    0.0
                }
            );
        }
        _ => println!("{name:<14} | keine Samples — der Kanal erntet Null",),
    };
    println!();
    fenster("X-Ray", &xray);
    fenster("EUV-304", &euv304);
    fenster("EUV-284", &euv284);
    fenster("Radio-2695", &radio);
    fenster("Bz-RTSW", &bz_rtsw);
    fenster("Dichte-RTSW", &dens_rtsw);
    fenster("Bz-OMNI", &bz_omni);
    fenster("Dichte-OMNI", &dens_omni);

    println!();
    println!("=== fehlt-Register ===");
    println!("OMNI ↔ GOES: Schnittmenge leer — der OMNI-Ingest endet am 06.08. (stopDate), die GOES-Reihen beginnen am 12.08. (7-Tage-Fenster).");
    println!("30 d @ 1 min wird von den APIs nicht bedient: xrays-30-day.json trägt 404.");
    println!("GOES-30d-Kandidat NGDC netCDF: 404 auf vier Stichproben (18./12./05.08., 25.07.) — kein Block eingetragen.");
    println!("Radio-Kadenz irregulär (~4–8 Samples/Tag); der Block-Pfad (details.0.flux) trägt nur den ersten Eintrag — die Reihe erntet alle details-Einträge.");
    println!(
        "EUV-Linien 256/1175/1216/1335/1405 + mgii_index bleiben ungeerntet (nicht deklariert)."
    );
    println!("LSK-Datei fehlt lokal — die Reihen laufen in Unix-Sekunden; die HAPI-Zeiten gehen durch die Identitäts-LSK (delta_t_a = J2000-Offset, 0-s-Pad): Unix rein, Unix raus — kein Leap-Offset, TE ist verschiebungsinvariant.");
    println!("Sekunden-Matrix: lag 0 (kanonisch) und lag 1 fallen zusammen — auf dem 1-min-Gitter ist der 1-Schritt-Schätzer der 60-s-Lag.");
    println!("Surrogat-Methodik: naive Fisher-Yates-Shuffles brechen die Autokorrelation nicht — die gebrochene Nullkontrolle (Dichte → GOES) ist der Beleg; phasenrandomisierte Surrogate / Block-Bootstrap stehen aus.");
    println!("Kleine n tragen keine Aussage: unter n = 30 wird kein Pfeil gemessen — Unterbestimmtheit, keine Stille.");

    let matrix_fenster = |channels: &[(&str, &[(f64, f64)])]| -> Option<(f64, f64, f64)> {
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

    let sekunden_channels = [
        ("X-Ray", xray.as_slice()),
        ("EUV-304", euv304.as_slice()),
        ("EUV-284", euv284.as_slice()),
        ("Bz-RTSW", bz_rtsw.as_slice()),
        ("Dichte-RTSW", dens_rtsw.as_slice()),
    ];
    println!();
    println!("=== Matrix 1 — Sekunden (lag ∈ {{0, 1, 2}} @ 1 min, gemeinsames Fenster) ===");
    match matrix_fenster(&sekunden_channels) {
        Some((t0, dt, n)) => {
            let binned: Vec<Vec<Option<f32>>> = sekunden_channels
                .iter()
                .map(|(_, s)| bin_mean(s, t0, dt, n as usize))
                .collect();
            for i in 0..sekunden_channels.len() {
                for j in 0..sekunden_channels.len() {
                    if i == j {
                        continue;
                    }
                    let (xs, ys) = pair_cells(&binned[j], &binned[i]);
                    te_row(
                        sekunden_channels[i].0,
                        sekunden_channels[j].0,
                        &xs,
                        &ys,
                        &[0, 1, 2],
                    );
                }
            }
        }
        None => println!("gemeinsames Fenster leer — Matrix fehlt"),
    }

    println!();
    println!("=== Matrix 2 — 7-Tage (lag ∈ {{0, 1, 2}} @ 1 min, n = 10 078) ===");
    {
        let channels = [
            ("X-Ray", xray.as_slice()),
            ("EUV-304", euv304.as_slice()),
            ("EUV-284", euv284.as_slice()),
        ];
        match matrix_fenster(&channels) {
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
            None => println!("gemeinsames Fenster leer — Matrix fehlt"),
        }
    }

    println!();
    println!("=== Matrix 3 — Stunden (Bz-OMNI ↔ Radio auf Radio-Gitter, Toleranz 1800 s; Bz-OMNI ↔ Dichte-OMNI stündlich) ===");
    {
        let (xs, ys) = join_nearest(&radio, &bz_omni, 1800.0);
        te_row("Bz-OMNI", "Radio-2695", &xs, &ys, &[0, 1, 2, 3]);
        let (xs, ys) = join_nearest(&radio, &bz_omni, 1800.0);
        te_row("Radio-2695", "Bz-OMNI", &ys, &xs, &[0, 1, 2, 3]);
        println!("  (Radio-Gitter-Index-lag ist irregular — benannt, nicht verschwiegen)");
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
            te_row("Bz-OMNI", "Dichte-OMNI", &xs, &ys, &[0, 1, 2, 3]);
            let (xs, ys) = pair_cells(&bb, &bd);
            te_row("Dichte-OMNI", "Bz-OMNI", &xs, &ys, &[0, 1, 2, 3]);
        } else {
            println!("Schnittmenge Bz-OMNI ↔ Dichte-OMNI leer — Matrix fehlt");
        }
    }

    println!();
    println!(
        "=== Matrix 4 — Grobe Radio-Matrix (Radio-Gitter, Toleranz 1800 s, schwache Statistik) ==="
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

    let mut pfeile: Vec<Pfeil> = Vec::new();
    {
        match matrix_fenster(&sekunden_channels) {
            Some((t0, dt, n)) => {
                let binned: Vec<Vec<Option<f32>>> = sekunden_channels
                    .iter()
                    .map(|(_, s)| bin_mean(s, t0, dt, n as usize))
                    .collect();
                for i in 0..sekunden_channels.len() {
                    for j in 0..sekunden_channels.len() {
                        if i == j {
                            continue;
                        }
                        let (xs, ys) = pair_cells(&binned[j], &binned[i]);
                        let te = transfer_entropy_lag(&xs, &ys, 0);
                        let stats = surrogate_stats(&xs, &ys, 0, SURROGATE_SEED);
                        if let (Some(t), Some((mean, sd, thr))) = (te, stats) {
                            pfeile.push(Pfeil {
                                von: sekunden_channels[i].0.into(),
                                nach: sekunden_channels[j].0.into(),
                                n: xs.len(),
                                te: t,
                                schwelle: thr,
                                mean,
                                sd,
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
        let stats = surrogate_stats(&xs, &ys, 0, SURROGATE_SEED);
        if let (Some(t), Some((mean, sd, thr))) = (te, stats) {
            pfeile.push(Pfeil {
                von: "Bz-OMNI".into(),
                nach: "Radio-2695".into(),
                n: xs.len(),
                te: t,
                schwelle: thr,
                mean,
                sd,
            });
        }
        let te = transfer_entropy_lag(&ys, &xs, 0);
        let stats = surrogate_stats(&ys, &xs, 0, SURROGATE_SEED);
        if let (Some(t), Some((mean, sd, thr))) = (te, stats) {
            pfeile.push(Pfeil {
                von: "Radio-2695".into(),
                nach: "Bz-OMNI".into(),
                n: xs.len(),
                te: t,
                schwelle: thr,
                mean,
                sd,
            });
        }
    }

    println!();
    println!("=== Pfeile (signifikant ⇔ TE > Schwelle = mean + 2σ der Surrogate) ===");
    for p in &pfeile {
        println!(
            "{:>14} → {:<14} | n {:>5} | TE {:.4e} | Schwelle {:.4e} | Surrogat (mean {:.3e}, σ {:.3e}) | {}",
            p.von,
            p.nach,
            p.n,
            p.te,
            p.schwelle,
            p.mean,
            p.sd,
            if p.te > p.schwelle { "Pfeil" } else { "still" }
        );
    }
    let sig = |a: &str, b: &str| {
        pfeile
            .iter()
            .any(|p| p.von == a && p.nach == b && p.te > p.schwelle)
    };
    let dag: Vec<String> = pfeile
        .iter()
        .filter(|p| p.te > p.schwelle)
        .map(|p| format!("{}→{}", p.von, p.nach))
        .collect();
    println!("DAG: {:?}", dag);

    println!();
    println!("=== Urteil ===");
    let bz_304 = sig("Bz-RTSW", "EUV-304");
    let s304_284 = sig("EUV-304", "EUV-284");
    let euv_xr_arrow = sig("EUV-304", "X-Ray") || sig("EUV-284", "X-Ray");
    let xr_euv_arrow = sig("X-Ray", "EUV-304") || sig("X-Ray", "EUV-284");
    let best_ue = |von: &[&str], nach: &str| {
        pfeile
            .iter()
            .filter(|p| p.te > p.schwelle && von.contains(&p.von.as_str()) && p.nach == nach)
            .max_by(|a, b| (a.te - a.schwelle).total_cmp(&(b.te - b.schwelle)))
    };
    let euv_xr_best = best_ue(&["EUV-304", "EUV-284"], "X-Ray");
    let xr_euv_best = match (
        best_ue(&["X-Ray"], "EUV-304"),
        best_ue(&["X-Ray"], "EUV-284"),
    ) {
        (Some(x), Some(y)) => {
            if x.te - x.schwelle >= y.te - y.schwelle {
                Some(x)
            } else {
                Some(y)
            }
        }
        (x, y) => x.or(y),
    };
    let euv_xr_ue = euv_xr_best
        .map(|p| p.te - p.schwelle)
        .unwrap_or(f64::NEG_INFINITY);
    let xr_euv_ue = xr_euv_best
        .map(|p| p.te - p.schwelle)
        .unwrap_or(f64::NEG_INFINITY);
    let radio_x_n = join_nearest(&radio, &xray, 1800.0).0.len();
    let radio_goes = radio_x_n >= 30
        && (sig("Radio-2695", "X-Ray")
            || sig("Radio-2695", "EUV-304")
            || sig("Radio-2695", "EUV-284"));
    let dens_violation = sig("Dichte-RTSW", "X-Ray")
        || sig("Dichte-RTSW", "EUV-304")
        || sig("Dichte-RTSW", "EUV-284")
        || sig("Dichte-OMNI", "Bz-OMNI")
        || sig("Dichte-OMNI", "Radio-2695");
    println!(
        "Nullkontrolle-Fenster: Dichte-RTSW ↔ GOES lief auf dem Sekunden-Fenster (gemeinsames Fenster aller Sekunden-Kanäle, ~2 d) — nicht auf OMNI (Schnittmenge leer, stopDate 06.08.)."
    );
    if bz_304 && s304_284 {
        if dens_violation {
            println!("TE(Bz → EUV-304) und TE(EUV-304 → EUV-284) signifikant → magnetischer Energiefluss durch die Übergangsregion in die heiße Korona (der Alfvén-Kanal) — VORLÄUFIG: die Surrogat-Schwelle steht unter dem Vorbehalt der gebrochenen Nullkontrolle.");
        } else {
            println!("TE(Bz → EUV-304) und TE(EUV-304 → EUV-284) signifikant → magnetischer Energiefluss durch die Übergangsregion in die heiße Korona (der Alfvén-Kanal).");
        }
    } else if bz_304 {
        println!("TE(Bz → EUV-304) signifikant, TE(EUV-304 → EUV-284) still → der Pfeil endet in der Übergangsregion — die Kette 304 → 284 fehlt.");
    } else if s304_284 {
        println!("TE(EUV-304 → EUV-284) signifikant, TE(Bz → EUV-304) still → die Übergangsregion heizt die Korona, der magnetische Antrieb bleibt ungemessen.");
    } else {
        println!("Kein Pfeil auf Bz → 304 → 284 — der magnetische Kanal ist still (0 honored: Stille ist die Antwort, kein Bug).");
    }
    if euv_xr_arrow && xr_euv_arrow {
        let (ux, uy) = (euv_xr_ue, xr_euv_ue);
        let (sx, sy) = (
            euv_xr_best.map(|p| p.sd).unwrap_or(f64::NAN),
            xr_euv_best.map(|p| p.sd).unwrap_or(f64::NAN),
        );
        if ux > uy {
            println!(
                "X-Ray ↔ EUV bidirektional — EUV → X-Ray trägt den größeren Überschuss ({:.2e} vs {:.2e}, Surrogat-σ {:.2e} vs {:.2e}) → Wellenheizung (Alfvén) überwiegt, Nanoflare-Anteil gegenläufig präsent.",
                ux, uy, sx, sy
            );
        } else {
            println!(
                "X-Ray ↔ EUV bidirektional — X-Ray → EUV trägt den größeren Überschuss ({:.2e} vs {:.2e}, Surrogat-σ {:.2e} vs {:.2e}) → Nanoflares überwiegen.",
                uy, ux, sy, sx
            );
        }
    } else if euv_xr_arrow {
        println!(
            "EUV → X-Ray signifikant bei stiller Rückrichtung → Wellenheizung (Alfvén-Kohärenz)."
        );
    } else if xr_euv_arrow {
        println!("X-Ray → EUV signifikant bei stiller Rückrichtung → Nanoflares.");
    } else {
        println!("X-Ray ↔ EUV beidseitig still → kein kausaler Pfeil auf der Sekunden-Skala.");
    }
    if radio_x_n < 30 {
        println!(
            "Radio ↔ GOES: keine Aussage möglich (n = {}) — Unterbestimmtheit, keine physikalische Stille.",
            radio_x_n
        );
    } else if radio_goes {
        println!("Radio → GOES signifikant bei stiller Rückrichtung → die Chromosphäre treibt die Korona.");
    } else {
        println!("Radio → GOES still — die Chromosphären-Kopplung trägt keinen Pfeil.");
    }
    if dens_violation {
        println!("Dichte-Paare über der Schwelle → NULLKONTROLLE BRICHT — das Urteil ist VORLÄUFIG: die naiven Fisher-Yates-Surrogate brechen die Autokorrelation nicht; gemeinsamer Trend oder Tagesgang könnten TE(Bz→304) und TE(304→284) mitdichten. Klärung pending: phasenrandomisierte Surrogate / Block-Bootstrap.");
    } else {
        println!("Nullkontrolle hält: Dichte-Paare unter der Schwelle.");
    }
    println!("Stille Zeilen sind Befunde. Exit 0.");
}
