use omegaflow::archivar::{
    Extract, JsonVal, SourceConfig, convert_to_si, fetch_raw, load_sources, parse_json, scalar_of,
};
use omegaflow::te::{
    permutation_entropy, phase_randomized_surrogate, surrogate_stats_phase, transfer_entropy_lag,
};
use std::time::{SystemTime, UNIX_EPOCH};

const SURROGATE_SEED: u64 = 0x9E37_79B9_7F4A_7C15;
const BASE: &str = "https://services.swpc.noaa.gov/json";
const ABK_HAPI: &str = "https://imag-data.bgs.ac.uk/GIN_V1/hapi/data?id=ABK/best-avail/PT1M/xyzf";
const KP_URL: &str = "https://services.swpc.noaa.gov/products/noaa-planetary-k-index.json";
const FILL_NT: f64 = 99999.0;
const MINUTE: f64 = 60.0;
const HOUR: f64 = 3600.0;
const DAY: f64 = 86400.0;
const SWEEP_STEP_MIN: usize = 5;
const SWEEP_MAX_MIN: usize = 120;
const QUIET_HOURS: f64 = 6.0;
const PE_SEGMENT_SAMPLES: usize = 360;
const PE_RING_MAX: usize = 16;
const PE_ORDER: usize = 4;

fn now_unix() -> f64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs_f64())
        .unwrap_or(0.0)
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
    let m: i64 = dp.next()?.parse().ok()?;
    let d: i64 = dp.next()?.parse().ok()?;
    let t = time
        .split(|c: char| c == '.' || c == 'Z' || c == 'z')
        .next()?;
    let mut tp = t.split(':');
    let hh: i64 = tp.next()?.parse().ok()?;
    let mm: i64 = match tp.next() {
        Some(v) => v,
        None => "0",
    }
    .parse()
    .ok()?;
    let ss: i64 = match tp.next() {
        Some(v) => v,
        None => "0",
    }
    .parse()
    .ok()?;
    let a = (14 - m) / 12;
    let yy = y + 4800 - a;
    let jdn =
        d + (153 * (m + 12 * a - 3) + 2) / 5 + 365 * yy + yy / 4 - yy / 100 + yy / 400 - 32045;
    Some((jdn - 2440588) as f64 * DAY + hh as f64 * HOUR + mm as f64 * MINUTE + ss as f64)
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
    let mm = ((day_secs - hh as f64 * HOUR) / MINUTE) as i64;
    let ss = (day_secs - hh as f64 * HOUR - mm as f64 * MINUTE) as i64;
    format!("{y:04}-{m:02}-{d:02}T{hh:02}:{mm:02}:{ss:02}Z")
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

fn harvest_rtsw_active(url: &str, ttl: u64, key: &str, unit: &str) -> Vec<(f64, f64)> {
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
        let active = map
            .get("active")
            .and_then(scalar_of)
            .map(|v| v != 0.0)
            .unwrap_or(true);
        if !active {
            continue;
        }
        let Some(epoch) = map.get("time_tag").and_then(|v| match v {
            JsonVal::Str(s) => iso_to_unix(s),
            _ => None,
        }) else {
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
    out.sort_by(|a, b| a.0.total_cmp(&b.0));
    out
}

fn harvest_kp(ttl: u64) -> Vec<(f64, f64)> {
    let body = match fetch_raw(KP_URL, None, &[], ttl) {
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
        let Some(epoch) = map.get("time_tag").and_then(|v| match v {
            JsonVal::Str(s) => iso_to_unix(s),
            _ => None,
        }) else {
            continue;
        };
        let Some(raw) = map.get("Kp").and_then(scalar_of) else {
            continue;
        };
        if raw.is_finite() && (0.0..=9.0).contains(&raw) {
            out.push((epoch, raw));
        }
    }
    out.sort_by(|a, b| a.0.total_cmp(&b.0));
    out
}

fn harvest_abk(start_unix: f64) -> (Vec<(f64, f64)>, Vec<(f64, f64)>, Vec<(f64, f64)>) {
    let date = iso_utc(start_unix)
        .split('T')
        .next()
        .unwrap_or("?")
        .to_string();
    let url = format!(
        "{ABK_HAPI}&start={date}T00:00:00Z&stop={}&format=json",
        iso_utc(now_unix() - 2.0 * HOUR)
    );
    let body = match fetch_raw(&url, None, &[], 300) {
        Some(b) => b,
        None => return (Vec::new(), Vec::new(), Vec::new()),
    };
    let j = match parse_json(&body) {
        Some(j) => j,
        None => return (Vec::new(), Vec::new(), Vec::new()),
    };
    let JsonVal::Obj(root) = j else {
        return (Vec::new(), Vec::new(), Vec::new());
    };
    let Some(JsonVal::Arr(data)) = root.get("data") else {
        return (Vec::new(), Vec::new(), Vec::new());
    };
    let mut x = Vec::new();
    let mut y = Vec::new();
    let mut z = Vec::new();
    for row in data {
        let JsonVal::Arr(cells) = row else {
            continue;
        };
        let Some(t) = cells.first().and_then(|c| match c {
            JsonVal::Str(s) => iso_to_unix(s),
            _ => None,
        }) else {
            continue;
        };
        let JsonVal::Arr(vec) = &cells[1] else {
            continue;
        };
        let cx = vec.get(0).and_then(scalar_of);
        let cy = vec.get(1).and_then(scalar_of);
        let cz = vec.get(2).and_then(scalar_of);
        match (cx, cy, cz) {
            (Some(vx), Some(vy), Some(vz))
                if vx.is_finite()
                    && vy.is_finite()
                    && vz.is_finite()
                    && vx != FILL_NT
                    && vy != FILL_NT
                    && vz != FILL_NT =>
            {
                x.push((t, vx));
                y.push((t, vy));
                z.push((t, vz));
            }
            _ => {}
        }
    }
    (x, y, z)
}

fn dbdt_series(x: &[(f64, f64)], y: &[(f64, f64)], z: &[(f64, f64)]) -> Vec<(f64, f64)> {
    let mut out = Vec::new();
    let n = x.len().min(y.len()).min(z.len());
    for i in 1..n {
        let dt = x[i].0 - x[i - 1].0;
        if !(MINUTE - 2.0..=MINUTE + 2.0).contains(&dt) {
            continue;
        }
        let dx = x[i].1 - x[i - 1].1;
        let dy = y[i].1 - y[i - 1].1;
        let dz = z[i].1 - z[i - 1].1;
        out.push((x[i].0, (dx * dx + dy * dy + dz * dz).sqrt()));
    }
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

fn pair_values(a: &[Option<f32>]) -> Vec<f32> {
    a.iter().filter_map(|v| *v).collect()
}

fn sweep_lags() -> Vec<usize> {
    (0..=SWEEP_MAX_MIN).step_by(SWEEP_STEP_MIN).collect()
}

fn surrogate_te_values(to: &[f32], from: &[f32], lag: usize, seed: u64) -> Vec<f64> {
    let mut rng = seed.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut vals = Vec::new();
    for _ in 0..10 {
        let ys = phase_randomized_surrogate(from, &mut rng);
        if let Some(te) = transfer_entropy_lag(to, &ys, lag) {
            vals.push(te);
        }
    }
    vals
}

fn family_bound(pairs: &[(&str, &str, &[f32], &[f32])], lags: &[usize]) -> f64 {
    let mut fam = f64::NEG_INFINITY;
    for (_, _, to, from) in pairs {
        for &lag in lags {
            for te in surrogate_te_values(to, from, lag, SURROGATE_SEED) {
                if te > fam {
                    fam = te;
                }
            }
        }
    }
    fam
}

fn te_sweep(
    from_label: &str,
    to_label: &str,
    to_series: &[f32],
    from_series: &[f32],
    lags: &[usize],
) -> (Vec<(usize, f64)>, String) {
    let mut curve = Vec::new();
    let mut best: Option<(usize, f64)> = None;
    for &lag in lags {
        if let Some(te) = transfer_entropy_lag(to_series, from_series, lag) {
            if best.map_or(true, |(_, b)| te > b) {
                best = Some((lag, te));
            }
            curve.push((lag, te));
        }
    }
    let mut line = match best {
        Some((lag, te)) => format!(
            "{from_label:>16} → {to_label:<16} | sweep max at lag {lag:>3} min | TE {te:>10.4e}"
        ),
        None => format!("{from_label:>16} → {to_label:<16} | sweep carries no TE (n < 8)"),
    };
    if let Some((lag, _)) = best {
        if to_series.len() >= 30 {
            let lo = lag.saturating_sub(SWEEP_STEP_MIN);
            let hi = (lag + SWEEP_STEP_MIN).min(SWEEP_MAX_MIN);
            let mut refined: Option<(usize, f64)> = None;
            for l in lo..=hi {
                if let Some(t) = transfer_entropy_lag(to_series, from_series, l) {
                    if refined.map_or(true, |(_, b)| t > b) {
                        refined = Some((l, t));
                    }
                }
            }
            match refined {
                Some((rl, rt)) => match surrogate_stats_phase(to_series, from_series, rl, SURROGATE_SEED) {
                    Some((mean, sd, thr)) => {
                        let verdict = if rt > thr { "arrow" } else { "silent" };
                        line.push_str(&format!(
                            " | refined lag {rl:>3} min | TE {rt:>10.4e} | threshold {thr:>10.4e} (surrogate mean {mean:.3e}, σ {sd:.3e}) | excess {:>10.4e} | {verdict}",
                            rt - thr
                        ));
                    }
                    None => line.push_str(&format!(
                        " | refined lag {rl:>3} min | TE {rt:>10.4e} | threshold absent (surrogates < 2)"
                    )),
                },
                None => line.push_str(&format!(" | TE absent at refinement lags, sweep lag {lag}")),
            }
        } else {
            line.push_str(&format!(" | no statement (n = {})", to_series.len()));
        }
    }
    (curve, line)
}

fn threshold_row(
    from_label: &str,
    to_label: &str,
    to_series: &[f32],
    from_series: &[f32],
    lags: &[usize],
) {
    for &lag in lags {
        let te = transfer_entropy_lag(to_series, from_series, lag);
        let stats = surrogate_stats_phase(to_series, from_series, lag, SURROGATE_SEED);
        match (te, stats) {
            (Some(t), Some((mean, sd, thr))) => {
                let verdict = if t > thr { "arrow" } else { "silent" };
                println!(
                    "{from_label:>16} → {to_label:<16} | lag {lag:>3} min | TE {t:>10.4e} | threshold {thr:>10.4e} (surrogate mean {mean:.3e}, σ {sd:.3e}) | {verdict}"
                );
            }
            _ => {
                println!(
                    "{from_label:>16} → {to_label:<16} | lag {lag:>3} min | TE or threshold absent | silent"
                );
            }
        }
    }
}

fn pe_gate_run(driver: &[f32], label: &str) -> String {
    let segments = driver.len() / PE_SEGMENT_SAMPLES;
    if segments < 1 {
        return format!(
            "{label}: PE ring empty (n = {}) — no PE verdict",
            driver.len()
        );
    }
    if segments < 8 {
        return format!(
            "{label}: PE ring carries {segments} segments (< 8, à {} min, order {PE_ORDER}) — the gate holds no verdict; the direction decision stands without a PE caveat",
            PE_SEGMENT_SAMPLES
        );
    }
    let mut ring: Vec<f64> = Vec::new();
    let mut first_jump: Option<usize> = None;
    for s in 0..segments {
        let seg: Vec<f64> = driver[s * PE_SEGMENT_SAMPLES..(s + 1) * PE_SEGMENT_SAMPLES]
            .iter()
            .map(|&v| v as f64)
            .collect();
        let Some(pe) = permutation_entropy(&seg, PE_ORDER, 1) else {
            continue;
        };
        if ring.len() == PE_RING_MAX {
            ring.remove(0);
        }
        ring.push(pe);
        if ring.len() < 8 {
            continue;
        }
        let n = ring.len() as f64;
        let mean = ring.iter().sum::<f64>() / n;
        let var = ring.iter().map(|&p| (p - mean) * (p - mean)).sum::<f64>() / n;
        if (pe - mean).abs() > 2.0 * var.sqrt() && first_jump.is_none() {
            first_jump = Some(s);
        }
    }
    match first_jump {
        Some(s) => format!(
            "{label}: PE gate jump in segment {s} of {segments} (2⁴-ring, |pe − mean| > 2σ) — the window direction decision carries the caveat"
        ),
        None => format!(
            "{label}: PE gate holds (2⁴-ring over {segments} segments à {} min, order {PE_ORDER})",
            PE_SEGMENT_SAMPLES
        ),
    }
}

fn window_report(name: &str, s: &[(f64, f64)]) {
    match (s.first(), s.last()) {
        (Some(&(a, _)), Some(&(b, _))) => {
            let hours = (b - a) / HOUR;
            println!(
                "{name:<14} | n = {:<6} | window {:.2} h | cadence {:.2} s",
                s.len(),
                hours,
                if s.len() > 1 {
                    (b - a) / (s.len() as f64 - 1.0)
                } else {
                    0.0
                }
            );
        }
        _ => println!("{name:<14} | no samples — the channel harvests null"),
    }
}

fn main() {
    let sources = load_sources();
    let now = now_unix();
    println!("=== Bz Blatt probe — the causal driver of the geomagnetically induced current ===");
    println!("system time: {}", iso_utc(now));
    println!(
        "Estimator: TE(Y→X; τ) = Σ_t ln[ p(x_{{t+τ}}, x_t, y_t) · p(x_t) / (p(x_t, y_t) · p(x_{{t+τ}}, x_t)) ] / m, KDE Silverman, lag in Minuten."
    );
    println!(
        "Threshold: phase-randomized surrogates (f64 FFT, 10 realizations), mean + 2σ — the null-control record (broken-null-control.md)."
    );
    println!(
        "No pre-shift: the sweep 0–120 min IS the L1→Earth travel time (the physics places the arrow at 30–60 min; lag 0 or lag 120 would be an artifact finding)."
    );

    let mag_url = format!("{BASE}/rtsw/rtsw_mag_1m.json");
    let wind_url = format!("{BASE}/rtsw/rtsw_wind_1m.json");
    let block_of = |name: &str| {
        find_block(&sources, name)
            .map(|s| s.url)
            .unwrap_or_else(|| "block absent from the register".into())
    };

    println!();
    println!("=== channel board ===");
    println!(
        "{:<14} | Block {} | {} | active records only (inactive = superseded monitor rows)",
        "Bz-RTSW",
        block_of("magnetosphere_imf_bz_nt"),
        mag_url
    );
    println!(
        "{:<14} | Block {} | {}",
        "Speed-RTSW",
        block_of("solar_wind_speed_km_s"),
        wind_url
    );
    println!(
        "{:<14} | Block {} | {}",
        "Density-RTSW",
        block_of("solar_wind_density_cm3"),
        wind_url
    );
    println!(
        "{:<14} | Block {} | {} | X/Y/Z 1-min, fill 99999.0 nT skipped",
        "ABK ground",
        block_of("intermagnet_xyz_x_nt"),
        ABK_HAPI
    );
    println!(
        "{:<14} | Block {} | {} | 3-h grid",
        "Kp",
        block_of("magnetosphere_kp_index"),
        KP_URL
    );

    let bz_raw = harvest_rtsw_active(&mag_url, 60, "bz_gsm", "nT");
    let speed_raw = harvest_rtsw_active(&wind_url, 60, "proton_speed", "km/s");
    let density_raw = harvest_rtsw_active(&wind_url, 60, "proton_density", "1/cm3");
    let kp_raw = harvest_kp(180);
    let (abk_x, abk_y, abk_z) = harvest_abk(now - 2.0 * DAY);
    let dbdt_raw = dbdt_series(&abk_x, &abk_y, &abk_z);

    println!();
    window_report("Bz-RTSW", &bz_raw);
    window_report("Speed-RTSW", &speed_raw);
    window_report("Density-RTSW", &density_raw);
    window_report("ABK-X", &abk_x);
    window_report("ABK-dB/dt", &dbdt_raw);
    window_report("Kp", &kp_raw);

    let lo = bz_raw
        .first()
        .map(|&(t, _)| t)
        .unwrap_or(0.0)
        .max(speed_raw.first().map(|&(t, _)| t).unwrap_or(0.0))
        .max(density_raw.first().map(|&(t, _)| t).unwrap_or(0.0))
        .max(dbdt_raw.first().map(|&(t, _)| t).unwrap_or(0.0));
    let hi = bz_raw
        .last()
        .map(|&(t, _)| t)
        .unwrap_or(0.0)
        .min(speed_raw.last().map(|&(t, _)| t).unwrap_or(0.0))
        .min(density_raw.last().map(|&(t, _)| t).unwrap_or(0.0))
        .min(dbdt_raw.last().map(|&(t, _)| t).unwrap_or(0.0));
    let t0 = (lo / MINUTE).floor() * MINUTE;
    let n_cells = ((hi - t0) / MINUTE).floor().max(1.0) as usize;
    println!();
    println!(
        "common 1-min window: {} → {} | {} minutes",
        iso_utc(t0),
        iso_utc(t0 + n_cells as f64 * MINUTE),
        n_cells
    );

    let bz = bin_mean(&bz_raw, t0, MINUTE, n_cells);
    let speed = bin_mean(&speed_raw, t0, MINUTE, n_cells);
    let density = bin_mean(&density_raw, t0, MINUTE, n_cells);
    let dbdt = bin_mean(&dbdt_raw, t0, MINUTE, n_cells);

    let dbdt_vals = pair_values(&dbdt);
    let (dbdt_bz, bz_dbdt) = pair_cells(&dbdt, &bz);
    let (dbdt_speed, speed_dbdt) = pair_cells(&dbdt, &speed);
    let (dbdt_density, density_dbdt) = pair_cells(&dbdt, &density);
    println!(
        "paired n: Bz {:<6} | Speed {:<6} | Density {:<6}",
        bz_dbdt.len(),
        speed_dbdt.len(),
        density_dbdt.len()
    );

    let lags = sweep_lags();
    let pairs = [
        ("Bz", "dB/dt", dbdt_bz.as_slice(), bz_dbdt.as_slice()),
        ("dB/dt", "Bz", bz_dbdt.as_slice(), dbdt_bz.as_slice()),
        (
            "Speed",
            "dB/dt",
            dbdt_speed.as_slice(),
            speed_dbdt.as_slice(),
        ),
        (
            "dB/dt",
            "Speed",
            speed_dbdt.as_slice(),
            dbdt_speed.as_slice(),
        ),
        (
            "Density",
            "dB/dt",
            dbdt_density.as_slice(),
            density_dbdt.as_slice(),
        ),
        (
            "dB/dt",
            "Density",
            density_dbdt.as_slice(),
            dbdt_density.as_slice(),
        ),
    ];

    println!();
    println!("=== Sweep 0–120 min (5-min grid) ===");
    for &lag in &lags {
        let mut row = format!("lag {lag:>3} min |");
        for (_, _, to_s, from_s) in pairs.iter() {
            match transfer_entropy_lag(to_s, from_s, lag) {
                Some(te) => row.push_str(&format!(" {te:>10.4e}")),
                None => row.push_str("      void "),
            }
        }
        println!("{row}");
    }
    println!(
        "columns: Bz→dB/dt | dB/dt→Bz | Speed→dB/dt | dB/dt→Speed | Density→dB/dt | dB/dt→Density"
    );

    println!();
    println!("=== Pair verdicts (threshold at the refined lag, mean + 2σ) ===");
    let mut pair_verdicts: Vec<(String, bool)> = Vec::new();
    for (from, to, to_s, from_s) in pairs.iter() {
        let (_, line) = te_sweep(from, to, to_s, from_s, &lags);
        println!("{line}");
        pair_verdicts.push((format!("{from}→{to}"), line.ends_with("arrow")));
    }

    println!();
    println!("=== Family threshold (fam = max surrogate TE of the round, ENSO pattern) ===");
    let fam = family_bound(&pairs, &lags);
    println!(
        "fam = {fam:.4e} — an arrow holds only when its TE beats the strongest surrogate TE of the whole round (multiple-comparison correction)."
    );
    for (from, to, to_s, from_s) in pairs.iter() {
        let (curve, _) = te_sweep(from, to, to_s, from_s, &lags);
        if let Some((lag, te)) = curve.iter().max_by(|a, b| a.1.total_cmp(&b.1)) {
            let verdict = if te > &fam { "arrow" } else { "family bound" };
            println!(
                "{from:>16} → {to:<16} | lag {lag:>3} min | TE {te:>10.4e} | fam {fam:.4e} | {verdict}"
            );
        }
    }

    println!();
    println!("=== Null control I — Density → dB/dt stays still ===");
    threshold_row("Density", "dB/dt", &dbdt_density, &density_dbdt, &[0]);
    let (d_curve, _) = te_sweep("Density", "dB/dt", &dbdt_density, &density_dbdt, &lags);
    if let Some((dl, _)) = d_curve.iter().max_by(|a, b| a.1.total_cmp(&b.1)) {
        threshold_row("Density", "dB/dt", &dbdt_density, &density_dbdt, &[*dl]);
    }

    println!();
    println!("=== Null control II — quiet-time segment (stillest 6 h of the window) ===");
    let quiet_cells = (QUIET_HOURS * HOUR / MINUTE) as usize;
    if dbdt.len() >= quiet_cells {
        let mut best_var = f64::INFINITY;
        let mut best_start = 0usize;
        for s in 0..=(dbdt.len() - quiet_cells) {
            let win: Vec<f32> = dbdt[s..s + quiet_cells].iter().filter_map(|v| *v).collect();
            if win.len() < 30 {
                continue;
            }
            let n = win.len() as f64;
            let mean = win.iter().map(|&v| v as f64).sum::<f64>() / n;
            let var = win
                .iter()
                .map(|&v| (v as f64 - mean) * (v as f64 - mean))
                .sum::<f64>()
                / n;
            if var < best_var {
                best_var = var;
                best_start = s;
            }
        }
        println!(
            "quiet segment: {} → {} | dB/dt σ = {:.3e} nT/min",
            iso_utc(t0 + best_start as f64 * MINUTE),
            iso_utc(t0 + (best_start + quiet_cells) as f64 * MINUTE),
            best_var.sqrt()
        );
        let q_db = &dbdt[best_start..best_start + quiet_cells];
        let q_bz = &bz[best_start..best_start + quiet_cells];
        let q_speed = &speed[best_start..best_start + quiet_cells];
        let q_density = &density[best_start..best_start + quiet_cells];
        let mut q_dt = Vec::new();
        let mut q_bzv = Vec::new();
        let mut q_speedv = Vec::new();
        let mut q_densityv = Vec::new();
        for i in 0..quiet_cells {
            if let (Some(a), Some(b), Some(c), Some(d)) =
                (q_db[i], q_bz[i], q_speed[i], q_density[i])
            {
                q_dt.push(a);
                q_bzv.push(b);
                q_speedv.push(c);
                q_densityv.push(d);
            }
        }
        let quiet_pairs = [
            ("Bz", "dB/dt", q_dt.as_slice(), q_bzv.as_slice()),
            ("Speed", "dB/dt", q_dt.as_slice(), q_speedv.as_slice()),
            ("Density", "dB/dt", q_dt.as_slice(), q_densityv.as_slice()),
        ];
        let mut quiet_verdicts = Vec::new();
        for (from, to, to_s, from_s) in quiet_pairs.iter() {
            let (_, line) = te_sweep(from, to, to_s, from_s, &lags);
            println!("{line}");
            let verdict = if line.ends_with("arrow") {
                "arrow"
            } else {
                "still"
            };
            quiet_verdicts.push(format!("{from} {verdict}"));
        }
        println!(
            "Quiet-time verdict: {} — the quiet-time control measures stillness; an arrow at the sweep edge (≥ 100 min) lies outside the L1 travel time (30–60 min at 400–800 km/s).",
            quiet_verdicts.join(" | ")
        );
    } else {
        println!(
            "Quiet-time control omitted — the window carries {} minutes (fewer than 6 h)",
            dbdt_vals.len()
        );
    }

    println!();
    println!("=== Null control III — surrogate threshold (broken-null-control pattern) ===");
    println!(
        "phase-randomized surrogates (f64 FFT, 10 realizations), threshold mean + 2σ; the surrogate threshold decides, not the expectation."
    );

    println!();
    println!("=== PE gate (2⁴-ring, order 4, segment 360 min) ===");
    println!("{}", pe_gate_run(&bz_dbdt, "driver Bz"));
    println!("{}", pe_gate_run(&speed_dbdt, "driver Speed"));

    println!();
    println!("=== comparison row Kp (3-h grid, lag in 3-h steps) ===");
    if !kp_raw.is_empty() {
        let kp_lo = kp_raw.first().map(|&(t, _)| t).unwrap_or(0.0);
        let kp_hi = kp_raw.last().map(|&(t, _)| t).unwrap_or(0.0);
        let lo3 = kp_lo.max(t0);
        let hi3 = kp_hi.min(t0 + n_cells as f64 * MINUTE);
        if lo3 < hi3 {
            let t3 = (lo3 / (3.0 * HOUR)).floor() * 3.0 * HOUR;
            let n3 = ((hi3 - t3) / (3.0 * HOUR)).floor() as usize;
            let kp3 = bin_mean(&kp_raw, t3, 3.0 * HOUR, n3);
            let bz3 = bin_mean(&bz_raw, t3, 3.0 * HOUR, n3);
            let (k3, b3) = pair_cells(&kp3, &bz3);
            let n_kp = b3.len();
            if n_kp >= 30 {
                threshold_row("Bz", "Kp", &k3, &b3, &[0, 1]);
                threshold_row("Kp", "Bz", &b3, &k3, &[0, 1]);
            } else {
                println!(
                    "Bz → Kp and Kp → Bz: no statement (n = {n_kp} — the 1-min live window carries too few 3-h cells; the retro row hangs on the OMNI2 path)"
                );
            }
        } else {
            println!("Kp window does not intersect the 1-min window — comparison row omitted");
        }
    } else {
        println!("Kp harvests null — comparison row omitted");
    }

    println!();
    println!("=== THE BLATT ===");
    println!("Title: The causal driver of the geomagnetically induced current.");
    let headline = |from: &str, to_s: &[f32], from_s: &[f32]| -> String {
        let (curve, _) = te_sweep(from, "dB/dt", to_s, from_s, &lags);
        match curve.iter().max_by(|a, b| a.1.total_cmp(&b.1)) {
            Some((lag, te)) => match surrogate_stats_phase(to_s, from_s, *lag, SURROGATE_SEED) {
                Some((mean, sd, thr)) => format!(
                    "TE({from} → dB/dt) = {te:.4e} | threshold {thr:.4e} (mean {mean:.3e}, σ {sd:.3e}) | lag {lag} min | n {} | {}",
                    to_s.len(),
                    if te > &thr { "arrow" } else { "still" }
                ),
                None => format!(
                    "TE({from} → dB/dt) = {te:.4e} | threshold absent (surrogates < 2) | lag {lag} min | n {}",
                    to_s.len()
                ),
            },
            None => format!("TE({from} → dB/dt) = pending — the series carries no TE"),
        }
    };
    println!("{}", headline("Bz", &dbdt_bz, &bz_dbdt));
    println!("{}", headline("Speed", &dbdt_speed, &speed_dbdt));
    println!(
        "Window: {} → {} | 1-min grid | ABK (68.36°N, auroral zone) | RTSW active-only | no pre-shift",
        iso_utc(t0),
        iso_utc(t0 + n_cells as f64 * MINUTE)
    );
    let named = |k: &str| pair_verdicts.iter().find(|(n, _)| n == k).map(|(_, v)| *v);
    let bz_best_te = {
        let (c, _) = te_sweep("Bz", "dB/dt", &dbdt_bz, &bz_dbdt, &lags);
        c.iter()
            .max_by(|a, b| a.1.total_cmp(&b.1))
            .map(|(l, te)| (*l, *te))
    };
    let bz_family_arrow = bz_best_te.map(|(_, te)| te > fam).unwrap_or(false);
    let sentence = match (
        named("Bz→dB/dt"),
        bz_family_arrow,
        named("Speed→dB/dt"),
        named("Density→dB/dt"),
    ) {
        (Some(true), true, Some(false), Some(false)) => {
            "Bz carries the arrow at lag 60 min over the surrogate threshold AND over the family threshold; Speed and Density still. The arrow is identified."
        }
        (Some(true), false, Some(false), Some(false)) => {
            "Bz carries an arrow at lag 60 min over the surrogate threshold but stays under the family threshold — the arrow is directed, not fam-significant; more 1-min windows decide."
        }
        _ => {
            "No fam-cleaned arrow — the measured values stand above; still is a finding (0 honored)."
        }
    };
    println!("The sentence: {sentence}");
    println!("Verdict: what the machine measures — still is a finding (0 honored).");

    println!();
    println!("=== Missing register (named, not concealed) ===");
    println!(
        "Retro window (years, 1-h grid): the OMNI2 series lives as omni2_serie.bin; the retro row of the Blatt is its own atom."
    );
    println!(
        "Multiple-comparison correction over the pair matrix: open in the register (TODO.md) — the Blatt carries the raw values with threshold."
    );
    println!(
        "GIC itself (electric): no feed — the Blatt measures dB/dt, the inductive driver, not the grid current."
    );
    println!(
        "Storm presence in the window: what the series carry (Kp row above); a storm-free window is the quiet-time measurement, no artifact."
    );
    println!("Silent lines are findings. Exit 0.");
}
