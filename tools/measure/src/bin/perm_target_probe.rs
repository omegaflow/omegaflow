use omegaflow::archivar::{
    Extract, JsonVal, SourceConfig, cache_path_for, convert_to_si, extract_series, is_time_key,
    load_sources, parse_json, scalar_of, ymd_to_days,
};
use omegaflow::lsk::LeapSeconds;
use omegaflow::mathematikerin::omega::perm_target;
use omegaflow::sha256::sha256_hex;
use omegaflow::te::phase_randomized_surrogate;
use std::collections::HashMap;

const CACHE_NETLOC: &str = "services.swpc.noaa.gov";
const PERM_GROUND: f32 = f32::EPSILON;
const N_SURR: usize = 10;
const CAL_N: usize = 8192;

fn next_rng(rng: &mut u64) -> f64 {
    *rng = rng
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);
    ((*rng >> 33) as f64) / ((u32::MAX >> 1) as f64)
}

fn gauss(rng: &mut u64) -> f64 {
    let u1 = next_rng(rng).max(1e-12);
    let u2 = next_rng(rng);
    (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos()
}

fn cached_body(name: &str) -> Option<String> {
    std::fs::read_to_string(cache_path_for(CACHE_NETLOC, name)).ok()
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

fn series_last_rows(body: &str, key: &str, unit: &str) -> Vec<(f64, f64)> {
    let j = match parse_json(body) {
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

fn series_block(
    body: &str,
    block_name: &str,
    sources: &[SourceConfig],
    lsk: &LeapSeconds,
) -> Vec<(f64, f64)> {
    let Some(src) = find_block(sources, block_name) else {
        return Vec::new();
    };
    let mut series_src = src;
    series_src.url = format!("{CACHE_NETLOC}/{block_name}");
    series_src.extracts.retain(|e| {
        matches!(
            e,
            Extract::First(fc, _) | Extract::Last(fc, _) | Extract::Path(fc)
                if fc.name == block_name
        )
    });
    extract_series(&series_src, body, lsk)
}

fn map_values(values: &[f32]) -> Option<(Vec<f32>, Vec<f32>, usize)> {
    if values.len() < 2 {
        return None;
    }
    let mut xs = Vec::with_capacity(values.len() - 1);
    let mut ts = Vec::with_capacity(values.len() - 1);
    let mut n_eps = 0usize;
    for t in 1..values.len() {
        let g = values[t].abs();
        let v_c = (values[t] - values[t - 1]).abs();
        xs.push(v_c / (g + PERM_GROUND));
        ts.push(perm_target(g, v_c));
        if g == 0.0 {
            n_eps += 1;
        }
    }
    Some((xs, ts, n_eps))
}

fn quantile(sorted: &[f32], q: f64) -> f32 {
    let n = sorted.len();
    if n == 0 {
        return f32::NAN;
    }
    if n == 1 {
        return sorted[0];
    }
    let pos = q * (n - 1) as f64;
    let lo = pos.floor() as usize;
    let hi = (pos.ceil() as usize).min(n - 1);
    let frac = (pos - lo as f64) as f32;
    sorted[lo] + (sorted[hi] - sorted[lo]) * frac
}

struct MapRow {
    n: usize,
    n_eps: usize,
    x_q: [f32; 5],
    px1: f32,
    px3: f32,
    t_q: [f32; 3],
}

fn row_from(xs: Vec<f32>, ts: Vec<f32>, n_eps: usize) -> MapRow {
    let mut x_sorted = xs;
    x_sorted.sort_by(|a, b| a.total_cmp(b));
    let mut t_sorted = ts;
    t_sorted.sort_by(|a, b| a.total_cmp(b));
    let n = x_sorted.len();
    MapRow {
        n,
        n_eps,
        x_q: [
            quantile(&x_sorted, 0.10),
            quantile(&x_sorted, 0.25),
            quantile(&x_sorted, 0.50),
            quantile(&x_sorted, 0.75),
            quantile(&x_sorted, 0.90),
        ],
        px1: x_sorted.iter().filter(|&&x| x > 1.0).count() as f32 / n as f32,
        px3: x_sorted.iter().filter(|&&x| x > 3.0).count() as f32 / n as f32,
        t_q: [
            quantile(&t_sorted, 0.10),
            quantile(&t_sorted, 0.50),
            quantile(&t_sorted, 0.90),
        ],
    }
}

fn gate_x_finite(name: &str, xs: &[f32]) {
    let non_finite = xs.iter().filter(|x| !x.is_finite()).count();
    if non_finite > 0 {
        eprintln!(
            "g1: {name} carries {non_finite} non-finite x sample(s) of {}",
            xs.len()
        );
        std::process::exit(2);
    }
}

fn print_row(name: &str, r: &MapRow) {
    println!(
        " {:<26} | {:>6} | {:>6} | {:>6.4} {:>6.4} {:>6.4} {:>6.4} {:>6.4} | {:>6.4} {:>6.4} | {:>6.4} {:>6.4} {:>6.4}",
        name,
        r.n,
        r.n_eps,
        r.x_q[0],
        r.x_q[1],
        r.x_q[2],
        r.x_q[3],
        r.x_q[4],
        r.px1,
        r.px3,
        r.t_q[0],
        r.t_q[1],
        r.t_q[2],
    );
}

fn row_from_quantity(values: &[f32], n_eps: usize) -> MapRow {
    let mut sorted = values.to_vec();
    sorted.sort_by(|a, b| a.total_cmp(b));
    let n = sorted.len();
    MapRow {
        n,
        n_eps,
        x_q: [
            quantile(&sorted, 0.10),
            quantile(&sorted, 0.25),
            quantile(&sorted, 0.50),
            quantile(&sorted, 0.75),
            quantile(&sorted, 0.90),
        ],
        px1: sorted.iter().filter(|&&x| x > 1.0).count() as f32 / n as f32,
        px3: sorted.iter().filter(|&&x| x > 3.0).count() as f32 / n as f32,
        t_q: [
            quantile(&sorted, 0.10),
            quantile(&sorted, 0.50),
            quantile(&sorted, 0.90),
        ],
    }
}

fn live_mode(path: &str) {
    let bytes = match std::fs::read(path) {
        Ok(b) => b,
        Err(_) => {
            println!("live dump absent");
            std::process::exit(2);
        }
    };
    let text = String::from_utf8_lossy(&bytes);

    let mut sensors: Option<&str> = None;
    for line in text.lines() {
        let t = line.trim();
        if let Some(rest) = t.strip_prefix('#') {
            if let Some(v) = rest.trim().strip_prefix("sensors=") {
                sensors = Some(v.trim());
            }
        }
    }
    if sensors == Some("no") {
        println!("refused: self-driven dump is not the sensor field");
        std::process::exit(2);
    }
    let sensors_label = match sensors {
        Some("yes") => "yes",
        _ => "unknown",
    };

    let mut gs: Vec<f32> = Vec::new();
    let mut vcs: Vec<f32> = Vec::new();
    let mut targets: Vec<f32> = Vec::new();
    let mut perms: Vec<f32> = Vec::new();
    for line in text.lines() {
        let t = line.trim();
        if t.is_empty() || t.starts_with('#') {
            continue;
        }
        let cols: Vec<&str> = t.split(',').collect();
        if cols.len() < 6 {
            continue;
        }
        if cols[0].trim().parse::<u64>().is_err() {
            continue;
        }
        let (Ok(g), Ok(v_c), Ok(target), Ok(fp)) = (
            cols[2].trim().parse::<f32>(),
            cols[3].trim().parse::<f32>(),
            cols[4].trim().parse::<f32>(),
            cols[cols.len() - 1].trim().parse::<f32>(),
        ) else {
            continue;
        };
        if !g.is_finite() || !v_c.is_finite() || !target.is_finite() || !fp.is_finite() {
            continue;
        }
        gs.push(g);
        vcs.push(v_c);
        targets.push(target);
        perms.push(fp);
    }

    if gs.is_empty() {
        println!("live dump absent");
        std::process::exit(2);
    }

    println!("=== perm_target_probe --live: the realized live-field input distribution ===");
    println!("sha256={} sensors={sensors_label}", sha256_hex(&bytes));
    println!();

    let n = gs.len();
    let n_eps = gs.iter().filter(|&&g| g == 0.0).count();
    let xs: Vec<f32> = gs
        .iter()
        .zip(&vcs)
        .map(|(&g, &v_c)| v_c / (g + PERM_GROUND))
        .collect();

    println!(
        " {:<26} | {:>6} | {:>6} | {:>34} | {:>13} | {:>21}",
        "quantity", "n", "n_eps", "q10 q25 q50 q75 q90", "P(>1) P(>3)", "q10 q50 q90"
    );
    print_row("g", &row_from_quantity(&gs, n_eps));
    print_row("v_c", &row_from_quantity(&vcs, 0));
    print_row("x=v_c/(g+PERM_GROUND)", &row_from_quantity(&xs, 0));
    print_row("target", &row_from_quantity(&targets, 0));
    print_row("field_permeability", &row_from_quantity(&perms, 0));
    println!();
    println!(
        "epsilon-floor fraction: {:.6} ({n_eps}/{n})",
        n_eps as f32 / n as f32
    );
    println!("TE-branch live distribution: pending");
}

fn null_line(name: &str, values: &[f32], real_px1: f32, rng: &mut u64) {
    let n = values.len().saturating_sub(1);
    if n < 64 {
        println!(" null {name:<22} | surrogates skipped (n < 64)");
        return;
    }
    let mut masses = Vec::with_capacity(N_SURR);
    for _ in 0..N_SURR {
        let surr = phase_randomized_surrogate(values, rng);
        let Some((xs, ts, n_eps)) = map_values(&surr) else {
            continue;
        };
        masses.push(row_from(xs, ts, n_eps).px1);
    }
    if masses.len() < 2 {
        println!(" null {name:<22} | no surrogate mass");
        return;
    }
    let m = masses.iter().sum::<f32>() / masses.len() as f32;
    let var = masses.iter().map(|v| (v - m) * (v - m)).sum::<f32>() / masses.len() as f32;
    let envelope = m + 2.0 * var.sqrt();
    let verdict = if real_px1 > envelope {
        "outside envelope"
    } else {
        "within envelope"
    };
    println!(" null {name:<22} | mean {m:.4} | +2σ {envelope:.4} | real {real_px1:.4} | {verdict}");
}

fn ar1(n: usize, phi: f64, rng: &mut u64) -> Vec<f32> {
    let mut v = Vec::with_capacity(n);
    let mut x = 0.0f64;
    for _ in 0..n {
        x = phi * x + next_rng(rng) * 2.0 - 1.0;
        v.push(x as f32);
    }
    v
}

fn lag1_autocorr(v: &[f32]) -> f64 {
    let n = v.len();
    if n < 2 {
        return f64::NAN;
    }
    let mean = v.iter().sum::<f32>() as f64 / n as f64;
    let mut num = 0.0f64;
    for t in 1..n {
        num += (v[t] as f64 - mean) * (v[t - 1] as f64 - mean);
    }
    let mut den = 0.0f64;
    for &x in v {
        let d = x as f64 - mean;
        den += d * d;
    }
    num / den
}

fn two_cluster(n: usize, sep: f64, rng: &mut u64) -> (Vec<f32>, usize, usize) {
    let sigma = 1.0f64;
    let half = n / 2;
    let a_mean = -sep * sigma / 2.0;
    let b_mean = sep * sigma / 2.0;
    let mut v = Vec::with_capacity(n);
    for _ in 0..half {
        v.push((a_mean + sigma * gauss(rng)) as f32);
    }
    for _ in half..n {
        v.push((b_mean + sigma * gauss(rng)) as f32);
    }
    (v, half, n - half)
}

fn main() {
    let mut args = std::env::args().skip(1);
    if let Some(first) = args.next() {
        if first == "--live" {
            match args.next() {
                Some(p) => live_mode(&p),
                None => {
                    println!("live dump absent");
                    std::process::exit(2);
                }
            }
            return;
        }
    }

    println!(
        "=== perm_target_probe: the realized input distribution of the permeability map, measured ==="
    );
    println!(
        "map: x = |Δv| / (|v| + EPSILON), target = perm_target(|v|, |Δv|); harvested driver series stand where the live ω_sum cannot stand in CI"
    );
    println!();

    let wind_body = cached_body("json-rtsw-rtsw_wind_1m");
    let mag_body = cached_body("json-rtsw-rtsw_mag_1m");
    let xray_body = cached_body("json-goes-primary-xrays-7-day");
    let euv_body = cached_body("json-goes-primary-euvs-7-day");
    for (name, present) in [
        ("rtsw_wind_1m", wind_body.is_some()),
        ("rtsw_mag_1m", mag_body.is_some()),
        ("xrays-7-day", xray_body.is_some()),
        ("euvs-7-day", euv_body.is_some()),
    ] {
        println!("{name}: {}", if present { "cached" } else { "absent" });
    }
    println!();

    let sources = load_sources();
    let lsk = LeapSeconds {
        delta_t_a: 946_728_000.0,
        deltas: vec![(0.0, 0.0)],
    };
    let proton_speed = match &wind_body {
        Some(b) => series_last_rows(b, "proton_speed", "km/s"),
        None => Vec::new(),
    };
    let bz_gsm = match &mag_body {
        Some(b) => series_last_rows(b, "bz_gsm", "nT"),
        None => Vec::new(),
    };
    let proton_density = match &wind_body {
        Some(b) => series_last_rows(b, "proton_density", "1/cm3"),
        None => Vec::new(),
    };
    let xray = match &xray_body {
        Some(b) => series_block(b, "noaa_goes_xray_flux_w_m2", &sources, &lsk),
        None => Vec::new(),
    };
    let euv304 = match &euv_body {
        Some(b) => series_block(b, "solar_euv_flux_304_wm2", &sources, &lsk),
        None => Vec::new(),
    };
    let euv284 = match &euv_body {
        Some(b) => series_block(b, "solar_euv_flux_284_wm2", &sources, &lsk),
        None => Vec::new(),
    };

    println!("=== series map ===");
    println!(
        " {:<26} | {:>6} | {:>6} | {:>34} | {:>13} | {:>21}",
        "series", "n", "n_eps", "x q10 q25 q50 q75 q90", "P(x>1) P(x>3)", "target q10 q50 q90"
    );
    let series: [(&str, &[(f64, f64)]); 6] = [
        ("proton_speed", &proton_speed),
        ("bz_gsm", &bz_gsm),
        ("proton_density", &proton_density),
        ("noaa_goes_xray_flux_w_m2", &xray),
        ("solar_euv_flux_304_wm2", &euv304),
        ("solar_euv_flux_284_wm2", &euv284),
    ];
    let mut rows: Vec<(String, Vec<f32>, f32)> = Vec::new();
    for (name, s) in series {
        let values: Vec<f32> = s.iter().map(|&(_, v)| v as f32).collect();
        let Some((xs, ts, n_eps)) = map_values(&values) else {
            println!(" {name:<26} | absent");
            continue;
        };
        gate_x_finite(name, &xs);
        let row = row_from(xs, ts, n_eps);
        print_row(name, &row);
        rows.push((name.to_string(), values, row.px1));
    }
    println!();

    println!("=== nulls ({N_SURR} phase-randomized surrogates, P(x > 1) envelope) ===");
    let mut rng = 0x0A95_517C_C1B7_2722u64;
    for (name, values, real_px1) in &rows {
        null_line(name, values, *real_px1, &mut rng);
    }
    println!();

    println!("=== calibration (AR(1) φ ∈ {{0.0, 0.5, 0.9}}; two-cluster 2σ/6σ) ===");
    for phi in [0.0f64, 0.5, 0.9] {
        let v = ar1(CAL_N, phi, &mut rng);
        let r1 = lag1_autocorr(&v);
        let Some((xs, ts, n_eps)) = map_values(&v) else {
            println!(" AR(1) φ={phi:.1} | absent");
            continue;
        };
        gate_x_finite(&format!("AR(1) φ={phi:.1}"), &xs);
        let row = row_from(xs, ts, n_eps);
        print_row(&format!("AR(1) φ={phi:.1}"), &row);
        if (r1 - phi).abs() >= 0.05 {
            eprintln!("g3: AR(1) φ={phi:.1} lag-1 autocorr {r1:.4} outside 0.05 of φ");
            std::process::exit(2);
        }
        println!("   lag-1 autocorr {r1:.4} (gate g3: within 0.05 of φ={phi:.1})");
    }
    for sep in [2.0f64, 6.0] {
        let (v, a_count, b_count) = two_cluster(CAL_N, sep, &mut rng);
        let Some((xs, ts, n_eps)) = map_values(&v) else {
            println!(" two-cluster {sep:.0}σ | absent");
            continue;
        };
        gate_x_finite(&format!("two-cluster {sep:.0}σ"), &xs);
        let row = row_from(xs, ts, n_eps);
        print_row(&format!("two-cluster {sep:.0}σ"), &row);
        if a_count * 10 < 3 * v.len() || b_count * 10 < 3 * v.len() {
            eprintln!("g4: two-cluster {sep:.0}σ mode balance void (a={a_count}, b={b_count})");
            std::process::exit(2);
        }
    }
    println!();
    println!(
        "gates: g1 x finite — measured; g3 AR(1) lag-1 — measured; g4 two-cluster balance — measured"
    );
}
