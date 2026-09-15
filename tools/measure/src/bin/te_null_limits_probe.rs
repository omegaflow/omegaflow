use omegaflow::archivar::{
    cache_path_for, convert_to_si, extract_series, is_time_key, load_sources, parse_json,
    scalar_of, ymd_to_days, Extract, JsonVal, SourceConfig,
};
use omegaflow::lsk::LeapSeconds;
use omegaflow::te::{
    phase_randomized_surrogate, surrogate_stats_phase, transfer_entropy_lag, transfer_entropy_lag_h,
};
use std::collections::HashMap;

const SEED: u64 = 0x9E37_79B9_7F4A_7C15;
const SEED_MUL: u64 = 0x517C_C1B7_2722_0A95;
const N_SURR: usize = 10;
const TRANSIENT: usize = 1000;
const L1_SUN_M: f64 = 1.481e11;
const CACHE_NETLOC: &str = "services.swpc.noaa.gov";
const FACTORS: [f64; 6] = [0.5, 0.75, 1.0, 1.5, 2.0, 3.0];

fn next_rng(rng: &mut u64) -> f64 {
    *rng = rng
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);
    ((*rng >> 33) as f64) / ((u32::MAX >> 1) as f64)
}

fn coupled_henon(n: usize, transient: usize, c: f64) -> (Vec<f32>, Vec<f32>) {
    let total = n + transient;
    let mut xs = vec![0.0f64; total];
    let mut ys = vec![0.0f64; total];
    xs[0] = 0.1;
    xs[1] = 0.0;
    ys[0] = 0.2;
    ys[1] = 0.1;
    for t in 1..total - 1 {
        xs[t + 1] = 1.4 - xs[t] * xs[t] + 0.3 * xs[t - 1];
        ys[t + 1] = 1.4 - (c * xs[t] * ys[t] + (1.0 - c) * ys[t] * ys[t]) + 0.3 * ys[t - 1];
    }
    let to_f32 = |v: &[f64]| {
        v[transient..]
            .iter()
            .map(|&x| x as f32)
            .collect::<Vec<f32>>()
    };
    (to_f32(&xs), to_f32(&ys))
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

fn mean_plus_2sigma(vals: &[f64]) -> Option<f64> {
    if vals.len() < 2 {
        return None;
    }
    let m = vals.iter().sum::<f64>() / vals.len() as f64;
    let var = vals.iter().map(|v| (v - m) * (v - m)).sum::<f64>() / vals.len() as f64;
    Some(m + 2.0 * var.sqrt())
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

fn decimate(pair: (Vec<f32>, Vec<f32>), stride: usize) -> (Vec<f32>, Vec<f32>) {
    (
        pair.0.into_iter().step_by(stride).collect(),
        pair.1.into_iter().step_by(stride).collect(),
    )
}

fn te_h_null(x: &[f32], y: &[f32], lag: usize, factor: f64, seed: u64) -> Option<(f64, f64)> {
    let te = transfer_entropy_lag_h(x, y, lag, factor)?;
    let mut rng = seed.wrapping_add(0x9e3779b97f4a7c15);
    let mut vals = Vec::with_capacity(N_SURR);
    for _ in 0..N_SURR {
        let ys = phase_randomized_surrogate(y, &mut rng);
        if let Some(t) = transfer_entropy_lag_h(x, &ys, lag, factor) {
            vals.push(t);
        }
    }
    let thr = mean_plus_2sigma(&vals)?;
    Some((te, thr))
}

fn te_null_vals(x: &[f32], y: &[f32], lag: usize, seed: u64) -> Option<(f64, f64, Vec<f64>)> {
    let te = transfer_entropy_lag(x, y, lag)?;
    let mut rng = seed.wrapping_add(0x9e3779b97f4a7c15);
    let mut vals = Vec::with_capacity(N_SURR);
    for _ in 0..N_SURR {
        let ys = phase_randomized_surrogate(y, &mut rng);
        if let Some(t) = transfer_entropy_lag(x, &ys, lag) {
            vals.push(t);
        }
    }
    let thr = mean_plus_2sigma(&vals)?;
    Some((te, thr, vals))
}

fn seed_next(cell: &mut u64) -> u64 {
    let s = SEED ^ cell.wrapping_mul(SEED_MUL);
    *cell += 1;
    s
}

fn window_report(name: &str, s: &[(f64, f64)]) {
    match (s.first(), s.last()) {
        (Some(&(a, _)), Some(&(b, _))) => println!(
            "{name:<14} | n = {:<6} | window {:.2} d | cadence {:.2} s",
            s.len(),
            (b - a) / 86400.0,
            if s.len() > 1 {
                (b - a) / (s.len() as f64 - 1.0)
            } else {
                0.0
            }
        ),
        _ => println!("{name:<14} | no samples"),
    }
}

fn m1_table(name: &str, x: &[f32], y: &[f32], lag: usize, cell: &mut u64) {
    if x.len() < 30 {
        println!("{name}: n = {} < 30 -> no finding", x.len());
        return;
    }
    println!("{name} (n = {}, lag {})", x.len(), lag);
    println!(
        " {:>7} | {:>10} | {:>12} | {:>12} | {}",
        "factor", "TE(h)", "thr(h)", "excess", "verdict"
    );
    for f in FACTORS {
        let seed = seed_next(cell);
        match te_h_null(x, y, lag, f, seed) {
            Some((te, thr)) => println!(
                " {:>7.2} | {:>10.4e} | {:>12.4e} | {:>+12.4e} | {}",
                f,
                te,
                thr,
                te - thr,
                if te > thr { "arrow" } else { "silent" }
            ),
            None => println!(" {:>7.2} | absent", f),
        }
    }
}

fn m2_table(name: &str, x: &[f32], y: &[f32], lags: &[usize], cell: &mut u64) {
    if x.len() < 30 {
        println!("{name}: n = {} < 30 -> no finding", x.len());
        return;
    }
    println!("{name} (n = {})", x.len());
    println!(
        " {:>5} | {:>10} | {:>12} | {:>12} | {}",
        "lag", "TE", "thr", "excess", "verdict"
    );
    let mut best: Option<(usize, f64)> = None;
    let mut verdict0: Option<bool> = None;
    let mut flips: Vec<usize> = Vec::new();
    for &lag in lags {
        let seed = seed_next(cell);
        let te = transfer_entropy_lag(x, y, lag);
        let thr = surrogate_stats_phase(x, y, lag, seed).map(|(_, _, t)| t);
        match (te, thr) {
            (Some(t), Some(h)) => {
                let verdict = t > h;
                if let Some(v0) = verdict0 {
                    if verdict != v0 {
                        flips.push(lag);
                    }
                } else {
                    verdict0 = Some(verdict);
                }
                if best.map_or(true, |(_, e)| t - h > e) {
                    best = Some((lag, t - h));
                }
                println!(
                    " {:>5} | {:>10.4e} | {:>12.4e} | {:>+12.4e} | {}",
                    lag,
                    t,
                    h,
                    t - h,
                    if verdict { "arrow" } else { "silent" }
                );
            }
            _ => println!(" {:>5} | absent", lag),
        }
    }
    match best {
        Some((lag, e)) => println!("optimum lag (max excess) = {lag}, excess {e:+.4e}"),
        None => println!("optimum lag: no measurable cell"),
    }
    if flips.is_empty() {
        println!("verdict set stable across the sweep");
    } else {
        println!("verdict flips at lags: {flips:?}");
    }
}

fn matrix_fam(
    cells: &[(String, Vec<f32>, Vec<f32>)],
    lags: &[usize],
    cell: &mut u64,
) -> (usize, usize, f64, Vec<(String, usize, f64, f64)>) {
    let mut per_cell = 0usize;
    let mut fam = f64::NEG_INFINITY;
    let mut rows: Vec<(String, usize, f64, f64)> = Vec::new();
    for (name, x, y) in cells {
        for &lag in lags {
            let seed = seed_next(cell);
            match te_null_vals(x, y, lag, seed) {
                Some((te, thr, vals)) => {
                    for &v in &vals {
                        if v > fam {
                            fam = v;
                        }
                    }
                    if te > thr {
                        per_cell += 1;
                    }
                    rows.push((name.clone(), lag, te, thr));
                }
                None => rows.push((name.clone(), lag, f64::NAN, f64::NAN)),
            }
        }
    }
    println!(
        " {:>20} | {:>4} | {:>10} | {:>12} | {:>7} | {:>7}",
        "pair", "lag", "TE", "thr(μ+2σ)", "cell", "fam"
    );
    let mut fam_surv = 0usize;
    for (name, lag, te, thr) in &rows {
        if te.is_finite() {
            let s = *te > fam;
            if s {
                fam_surv += 1;
            }
            println!(
                " {:>20} | {:>4} | {:>10.4e} | {:>12.4e} | {:>7} | {:>7}",
                name,
                lag,
                te,
                thr,
                if te > thr { "arrow" } else { "-" },
                if s { "arrow" } else { "-" }
            );
        } else {
            println!(" {:>20} | {:>4} | absent", name, lag);
        }
    }
    (per_cell, fam_surv, fam, rows)
}

fn main() {
    println!("=== te_null_limits_probe: the three named open limits of the broken-null paper, measured ===");
    println!(
        "estimator: KDE TE, Gaussian kernel, Silverman h = 1.06 σ n^(-1/5) per series; null: {} phase-randomized surrogates, threshold μ+2σ.",
        N_SURR
    );
    println!("M1: bandwidth sensitivity (Silverman factor); M2: lag sweep; M3: max-T (fam) over the pair matrix.");
    println!("fam = max of the surrogate TE values over all pairs x lags of the round (a single TE number, never a threshold).");
    println!();

    let (xh, yh) = coupled_henon(1000, TRANSIENT, 0.2);

    println!("=== live channels (cached SWPC JSON, cache/{CACHE_NETLOC}) ===");
    let wind_body = cached_body("json-rtsw-rtsw_wind_1m.json");
    let mag_body = cached_body("json-rtsw-rtsw_mag_1m.json");
    let xray_body = cached_body("json-goes-primary-xrays-7-day.json");
    let euv_body = cached_body("json-goes-primary-euvs-7-day.json");
    for (name, present) in [
        ("rtsw_wind_1m", wind_body.is_some()),
        ("rtsw_mag_1m", mag_body.is_some()),
        ("xrays-7-day", xray_body.is_some()),
        ("euvs-7-day", euv_body.is_some()),
    ] {
        println!("{name}: {}", if present { "cached" } else { "absent" });
    }

    let sources = load_sources();
    let lsk = LeapSeconds {
        delta_t_a: 946_728_000.0,
        deltas: vec![(0.0, 0.0)],
    };
    let wind_rtsw = match &wind_body {
        Some(b) => series_last_rows(b, "proton_speed", "km/s"),
        None => Vec::new(),
    };
    let bz_raw = match &mag_body {
        Some(b) => series_last_rows(b, "bz_gsm", "nT"),
        None => Vec::new(),
    };
    let dens_raw = match &wind_body {
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
    let bz_rtsw = l1_sync_series(&bz_raw, &wind_rtsw, 60.0);
    let dens_rtsw = l1_sync_series(&dens_raw, &wind_rtsw, 60.0);

    window_report("X-Ray", &xray);
    window_report("EUV-304", &euv304);
    window_report("EUV-284", &euv284);
    window_report("Bz-RTSW", &bz_rtsw);
    window_report("Density-RTSW", &dens_rtsw);
    println!();

    let channels: [(&str, &[(f64, f64)]); 5] = [
        ("X-Ray", &xray),
        ("EUV-304", &euv304),
        ("EUV-284", &euv284),
        ("Bz-RTSW", &bz_rtsw),
        ("Density-RTSW", &dens_rtsw),
    ];
    let lo = channels
        .iter()
        .filter_map(|(_, s)| s.first().map(|&(t, _)| t))
        .fold(f64::NEG_INFINITY, f64::max);
    let hi = channels
        .iter()
        .filter_map(|(_, s)| s.last().map(|&(t, _)| t))
        .fold(f64::INFINITY, f64::min);
    let dt = 60.0;
    let t0 = (lo / dt).floor() * dt;
    let n_cells = if lo < hi {
        ((hi - t0) / dt).floor() as usize
    } else {
        0
    };
    let binned: Vec<Vec<Option<f32>>> = channels
        .iter()
        .map(|(_, s)| bin_mean(s, t0, dt, n_cells))
        .collect();
    let cell_of =
        |target: usize, driver: usize| decimate(pair_cells(&binned[target], &binned[driver]), 2);
    let live_grid = lo < hi;
    if !live_grid {
        println!("live common window empty — live measurements fall back to synthetic");
    }
    println!("live 60 s grid: n_cells = {n_cells} (lag unit = 60 s)");
    println!("live cells decimated 2:1 (120 s cadence) to keep the local run modest; the full 60 s grid is a CI job");
    println!();

    let mut cell = 0u64;

    println!("=== M1 — KDE bandwidth sensitivity (Silverman factor on h; threshold recomputed under the same h) ===");
    if live_grid {
        let (x, y) = cell_of(0, 4);
        m1_table("Density-RTSW -> X-Ray", &x, &y, 1, &mut cell);
        let (x, y) = cell_of(1, 4);
        m1_table("Density-RTSW -> EUV-304", &x, &y, 1, &mut cell);
        let (x, y) = cell_of(2, 4);
        m1_table("Density-RTSW -> EUV-284", &x, &y, 1, &mut cell);
        let (x, y) = cell_of(3, 4);
        m1_table("Density-RTSW -> Bz-RTSW", &x, &y, 1, &mut cell);
        let (x, y) = cell_of(0, 4);
        let seed = seed_next(&mut cell);
        let lib = surrogate_stats_phase(&x, &y, 1, seed).map(|(_, _, t)| t);
        let mine = te_h_null(&x, &y, 1, 1.0, seed).map(|(_, t)| t);
        println!(
            "h = 1.0 path check (library surrogate_stats_phase == inline te_h_null, same seed): {}",
            lib == mine
        );
    }
    m1_table("Henon c=0.2 X->Y (fwd)", &yh, &xh, 1, &mut cell);
    m1_table("Henon c=0.2 Y->X (rev)", &xh, &yh, 1, &mut cell);
    println!();

    println!("=== M2 — lag sweep ===");
    if live_grid {
        let (x, y) = cell_of(0, 4);
        m2_table(
            "Density-RTSW -> X-Ray, tau 0..360 s (120 s grid)",
            &x,
            &y,
            &[0, 1, 2, 3],
            &mut cell,
        );
        let (x, y) = cell_of(3, 4);
        m2_table(
            "Density-RTSW -> Bz-RTSW, tau 0..360 s (120 s grid)",
            &x,
            &y,
            &[0, 1, 2, 3],
            &mut cell,
        );
    }
    let hen_lags = [0usize, 1, 5, 10, 20, 30, 60, 120, 180, 240, 300, 360];
    m2_table(
        "Henon c=0.2 X->Y (fwd), tau 0..360",
        &yh,
        &xh,
        &hen_lags,
        &mut cell,
    );
    m2_table(
        "Henon c=0.2 Y->X (rev), tau 0..360",
        &xh,
        &yh,
        &hen_lags,
        &mut cell,
    );
    println!();

    println!("=== M3 — max-T (fam) over the pair matrix ===");
    if live_grid {
        let names = ["X-Ray", "EUV-304", "EUV-284", "Bz-RTSW", "Density-RTSW"];
        let mut cells: Vec<(String, Vec<f32>, Vec<f32>)> = Vec::new();
        for i in 0..names.len() {
            for j in 0..names.len() {
                if i == j {
                    continue;
                }
                let (x, y) = cell_of(i, j);
                cells.push((format!("{} -> {}", names[j], names[i]), x, y));
            }
        }
        println!("live 20-pair matrix, lags {{0, 1, 2}} (120 s grid):");
        let (per_cell, fam_surv, fam, _) = matrix_fam(&cells, &[0, 1, 2], &mut cell);
        println!(
            "fam = {:.4e}; per-cell (μ+2σ) arrows: {} of {} cells; fam survivors (TE > fam): {}",
            fam,
            per_cell,
            cells.len() * 3,
            fam_surv
        );
    }

    let mut rng = 0x0A95_517C_C1B7_2722u64;
    let mut synth: Vec<(String, Vec<f32>, Vec<f32>)> = Vec::new();
    for i in 0..10 {
        let a = ar1(300, 0.7, &mut rng);
        let b = ar1(300, 0.7, &mut rng);
        synth.push((format!("indep {} -> {}", i, i + 10), a.clone(), b.clone()));
        synth.push((format!("indep {} -> {}", i + 10, i), b, a));
    }
    for i in 0..4 {
        let a: Vec<f32> = (0..300)
            .map(|_| (next_rng(&mut rng) * 2.0 - 1.0) as f32)
            .collect();
        let b: Vec<f32> = (0..a.len())
            .map(|k| {
                if k == 0 {
                    next_rng(&mut rng) as f32
                } else {
                    (0.9 * a[k - 1] as f64 + (next_rng(&mut rng) * 0.2 - 0.1)) as f32
                }
            })
            .collect();
        synth.push((format!("coupled {} a->b (true)", i), b.clone(), a.clone()));
        synth.push((format!("coupled {} b->a (silent)", i), a, b));
    }
    println!("synthetic matrix: 10 independent AR(1) pairs (20 directed cells) + 4 coupled pairs (8 directed cells), n = 300, lags {{0, 1}}:");
    let (per_cell, fam_surv, fam, rows) = matrix_fam(&synth, &[0, 1], &mut cell);
    let indep_per_cell = rows
        .iter()
        .filter(|(n, _, te, thr)| n.starts_with("indep") && te.is_finite() && te > thr)
        .count();
    let coupled_fwd_per_cell = rows
        .iter()
        .filter(|(n, _, te, thr)| n.contains("a->b") && te.is_finite() && te > thr)
        .count();
    let coupled_fwd_fam = rows
        .iter()
        .filter(|(n, _, te, _)| n.contains("a->b") && te.is_finite() && *te > fam)
        .count();
    println!(
        "fam = {:.4e}; per-cell (μ+2σ) arrows: {} of {} cells (independent cells: {} of 40 = false positives; true couplings: {} of 8); fam survivors (TE > fam): {} (true couplings: {})",
        fam,
        per_cell,
        synth.len() * 2,
        indep_per_cell,
        coupled_fwd_per_cell,
        fam_surv,
        coupled_fwd_fam
    );
}
