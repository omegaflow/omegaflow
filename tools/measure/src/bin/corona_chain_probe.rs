use omegaflow::archivar::hdf5::{Endian, Hdf5File, decode_f32, decode_f64};
use omegaflow::archivar::radio_burst;
use omegaflow::archivar::soho_lasco;
use omegaflow::te::{phase_randomized_surrogate, transfer_entropy_lag, transfer_entropy_lag_h};

const AIA_MAGIC: [u8; 4] = *b"AIA1";
const DT: f64 = 24.0;
const WINDOW: usize = 100;
const REFRACTORY: usize = 75;
const FLARE_THRESH: f64 = 5e-6;
const FILL: f64 = -9999.0;
const LAG96: usize = 4;
const N_SURR_EVENT: usize = 5;
const N_SURR_DAILY: usize = 20;
const N_SHIFT: usize = 500;

const BAND_304: u32 = 5;
const BAND_131: u32 = 1;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn arg_values(args: &[String], name: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < args.len() {
        if args[i] == name {
            if let Some(v) = args.get(i + 1) {
                out.push(v.clone());
                i += 1;
            }
        }
        i += 1;
    }
    out
}

fn read_aia_lines(path: &str) -> Vec<(f64, f64, u32)> {
    let Ok(bytes) = std::fs::read(path) else {
        eprintln!("{path} reads void");
        return Vec::new();
    };
    if bytes.len() < 8 || bytes[0..4] != AIA_MAGIC {
        eprintln!("{path} carries no AIA1 contract");
        return Vec::new();
    }
    let n = u32::from_le_bytes(bytes[4..8].try_into().unwrap()) as usize;
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let o = 8 + i * 20;
        let Some(t) = bytes
            .get(o..o + 8)
            .and_then(|b| b.try_into().ok())
            .map(f64::from_le_bytes)
        else {
            continue;
        };
        let Some(v) = bytes
            .get(o + 8..o + 16)
            .and_then(|b| b.try_into().ok())
            .map(f64::from_le_bytes)
        else {
            continue;
        };
        let Some(idx) = bytes
            .get(o + 16..o + 20)
            .and_then(|b| b.try_into().ok())
            .map(u32::from_le_bytes)
        else {
            continue;
        };
        out.push((t, v, idx));
    }
    out
}

fn goes_series(path: &str, band: &str) -> Vec<(f64, f64)> {
    let Ok(bytes) = std::fs::read(path) else {
        return Vec::new();
    };
    let Ok(file) = Hdf5File::parse(&bytes) else {
        return Vec::new();
    };
    let (Ok(t_raw), Ok(v_raw), Ok(f_raw)) = (
        file.read_dataset("time"),
        file.read_dataset(band),
        file.read_dataset(&format!("{}_flags", band.trim_end_matches("_flux"))),
    ) else {
        return Vec::new();
    };
    let n = t_raw.len() / 8;
    if v_raw.len() != n * 4 || f_raw.len() != n * 2 {
        return Vec::new();
    }
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let Some(t) = decode_f64(&t_raw, i * 8, Endian::Le) else {
            continue;
        };
        let flag = u16::from_le_bytes([f_raw[i * 2], f_raw[i * 2 + 1]]);
        if flag != 0 {
            continue;
        }
        let Some(v) = decode_f32(&v_raw, i * 4, Endian::Le) else {
            continue;
        };
        let v = v as f64;
        if !v.is_finite() || v == FILL || v <= 0.0 {
            continue;
        }
        out.push((t, v));
    }
    out.sort_by(|a, b| a.0.total_cmp(&b.0));
    out
}

fn bin_median(series: &[(f64, f64)], t0: f64, bins: usize) -> Vec<Option<f32>> {
    let mut acc: Vec<Vec<f64>> = vec![Vec::new(); bins];
    for &(t, v) in series {
        let idx = ((t - t0) / DT).floor();
        if idx < 0.0 || idx >= bins as f64 {
            continue;
        }
        acc[idx as usize].push(v);
    }
    acc.into_iter()
        .map(|mut v| {
            if v.is_empty() {
                return None;
            }
            v.sort_by(|a, b| a.total_cmp(b));
            let m = if v.len() % 2 == 0 {
                (v[v.len() / 2 - 1] + v[v.len() / 2]) * 0.5
            } else {
                v[v.len() / 2]
            };
            Some(m as f32)
        })
        .collect()
}

struct Event {
    t: f64,
    d96: f64,
}

fn te_pair(cool: &[f32], hot: &[f32], lag: usize) -> Option<f64> {
    let f = transfer_entropy_lag_h(hot, cool, lag, 1.0)?;
    let r = transfer_entropy_lag_h(cool, hot, lag, 1.0)?;
    let d = f - r;
    Some(d / (f.abs() + r.abs()).max(1e-12))
}

fn pearson(x: &[f64], y: &[f64]) -> Option<f64> {
    let n = x.len().min(y.len());
    if n < 3 {
        return None;
    }
    let mx = x[..n].iter().sum::<f64>() / n as f64;
    let my = y[..n].iter().sum::<f64>() / n as f64;
    let mut sxy = 0.0;
    let mut sxx = 0.0;
    let mut syy = 0.0;
    for i in 0..n {
        let dx = x[i] - mx;
        let dy = y[i] - my;
        sxy += dx * dy;
        sxx += dx * dx;
        syy += dy * dy;
    }
    if sxx <= 0.0 || syy <= 0.0 {
        return None;
    }
    Some(sxy / (sxx * syy).sqrt())
}

fn best_lead_r(
    driver: &[Option<f64>],
    target: &[Option<f64>],
    leads: &[i32],
) -> Option<(f64, i32, usize)> {
    let n = driver.len().min(target.len());
    let mut best: Option<(f64, i32, usize)> = None;
    for &lead in leads {
        let mut x = Vec::new();
        let mut y = Vec::new();
        for d in 0..n {
            let j = d as i32 + lead;
            if j < 0 || j >= n as i32 {
                continue;
            }
            let (Some(a), Some(b)) = (driver[d], target[j as usize]) else {
                continue;
            };
            x.push(a);
            y.push(b);
        }
        if let Some(r) = pearson(&x, &y) {
            if best.map_or(true, |(b, _, _)| r.abs() > b.abs()) {
                best = Some((r, lead, x.len()));
            }
        }
    }
    best
}

fn shift_series(v: &[Option<f64>], k: usize) -> Vec<Option<f64>> {
    let n = v.len();
    (0..n).map(|i| v[(i + n - k % n) % n]).collect()
}

struct DriverResult {
    name: &'static str,
    n_days: usize,
    r: f64,
    lead: i32,
    null95: f64,
    p: f64,
    te0: Option<f64>,
    te_fam: Option<f64>,
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some(aia_path) = arg_value(&args, "--aia") else {
        eprintln!("--aia <aia_lines.bin> absent");
        return;
    };
    let Some(goes_dir) = arg_value(&args, "--goes-dir") else {
        eprintln!("--goes-dir absent");
        return;
    };
    let lasco_dirs = arg_values(&args, "--lasco-dir");
    let radio_path = arg_value(&args, "--radio");
    let lsk = omegaflow::archivar::embedded_lsk();

    let records = read_aia_lines(&aia_path);
    let s304: Vec<(f64, f64)> = records
        .iter()
        .filter(|(_, _, i)| *i == BAND_304)
        .map(|&(t, v, _)| (t, v))
        .collect();
    let s131: Vec<(f64, f64)> = records
        .iter()
        .filter(|(_, _, i)| *i == BAND_131)
        .map(|&(t, v, _)| (t, v))
        .collect();
    if s304.is_empty() || s131.is_empty() {
        eprintln!("AIA 304 or 131 absent in {aia_path} — the chain stays unmeasured");
        return;
    }
    let t0 = s304
        .iter()
        .chain(s131.iter())
        .map(|&(t, _)| t)
        .fold(f64::INFINITY, f64::min);
    let t1 = s304
        .iter()
        .chain(s131.iter())
        .map(|&(t, _)| t)
        .fold(f64::NEG_INFINITY, f64::max);
    let bins = ((t1 - t0) / DT).floor() as usize;
    let g304 = bin_median(&s304, t0, bins);
    let g131 = bin_median(&s131, t0, bins);

    let mut b_flux: Vec<(f64, f64)> = Vec::new();
    let mut a_flux: Vec<(f64, f64)> = Vec::new();
    let Ok(entries) = std::fs::read_dir(&goes_dir) else {
        eprintln!("{goes_dir} reads void");
        return;
    };
    let mut goes_files = 0usize;
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().to_string();
        if name.starts_with("xr_") && name.ends_with(".nc") {
            goes_files += 1;
            let raw_b = goes_series(&entry.path().to_string_lossy(), "b_flux");
            let raw_a = goes_series(&entry.path().to_string_lossy(), "a_flux");
            for (t, v) in raw_b {
                if let Some(t2) = lsk.as_ref().and_then(|l| l.unix_to_tdb(t)) {
                    b_flux.push((t2, v));
                }
            }
            for (t, v) in raw_a {
                if let Some(t2) = lsk.as_ref().and_then(|l| l.unix_to_tdb(t)) {
                    a_flux.push((t2, v));
                }
            }
        }
    }
    b_flux.sort_by(|a, b| a.0.total_cmp(&b.0));
    a_flux.sort_by(|a, b| a.0.total_cmp(&b.0));
    if b_flux.is_empty() {
        eprintln!("{goes_dir}: no GOES b_flux found — the chain stays unmeasured");
        return;
    }
    let trig = bin_median(&b_flux, t0, bins);

    let mut events: Vec<Event> = Vec::new();
    let mut null_pool: Vec<f64> = Vec::new();
    let mut i = 0usize;
    while i < bins {
        let Some(tv) = trig[i] else {
            i += 1;
            continue;
        };
        if (tv as f64) < FLARE_THRESH {
            i += 1;
            continue;
        }
        let mut j = i;
        let mut peak = i;
        while j < bins && j - i < REFRACTORY {
            if let Some(v) = trig[j] {
                if (v as f64) < FLARE_THRESH && j > i + 3 {
                    break;
                }
                if let Some(pv) = trig[peak] {
                    if v > pv {
                        peak = j;
                    }
                }
            }
            j += 1;
        }
        let lo = peak.saturating_sub(WINDOW);
        let hi = (peak + WINDOW).min(bins);
        let mut best_cool: Vec<f32> = Vec::new();
        let mut best_hot: Vec<f32> = Vec::new();
        let mut run_cool: Vec<f32> = Vec::new();
        let mut run_hot: Vec<f32> = Vec::new();
        for k in lo..hi {
            match (g304[k], g131[k]) {
                (Some(c), Some(h)) => {
                    run_cool.push(c);
                    run_hot.push(h);
                }
                _ => {
                    if run_cool.len() > best_cool.len() {
                        std::mem::swap(&mut best_cool, &mut run_cool);
                        std::mem::swap(&mut best_hot, &mut run_hot);
                    }
                    run_cool.clear();
                    run_hot.clear();
                }
            }
        }
        if run_cool.len() > best_cool.len() {
            best_cool = run_cool;
            best_hot = run_hot;
        }
        if best_cool.len() >= 100 {
            if let Some(d) = te_pair(&best_cool, &best_hot, LAG96) {
                events.push(Event {
                    t: t0 + (peak as f64) * DT,
                    d96: d,
                });
                let mut rng = (events.len() as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15);
                for _ in 0..N_SURR_EVENT {
                    let cs = phase_randomized_surrogate(&best_cool, &mut rng);
                    if let Some(dn) = te_pair(&cs, &best_hot, LAG96) {
                        null_pool.push(dn);
                    }
                }
            }
        }
        i = j.max(i + REFRACTORY);
    }
    if events.is_empty() {
        eprintln!("no flare events with a complete 304/131 window — the chain stays unmeasured");
        return;
    }

    null_pool.sort_by(|a, b| a.total_cmp(b));
    let null95_event = if null_pool.is_empty() {
        f64::NAN
    } else {
        null_pool[(null_pool.len() * 95 / 100).min(null_pool.len() - 1)]
    };
    let event_mean = events.iter().map(|e| e.d96).sum::<f64>() / events.len() as f64;
    let event_pos = events.iter().filter(|e| e.d96 > 0.0).count();

    let day_of = |t: f64| -> usize {
        let unix = lsk.as_ref().and_then(|l| l.tdb_to_unix(t)).unwrap_or(t);
        (unix / 86_400.0).floor() as usize
    };
    let day_min = day_of(t0);
    let day_max = day_of(t1);
    let n_days = day_max.saturating_sub(day_min) + 1;
    let mut d96_by_day: Vec<Vec<f64>> = vec![Vec::new(); n_days];
    for ev in &events {
        let d = day_of(ev.t).saturating_sub(day_min);
        if d < n_days {
            d96_by_day[d].push(ev.d96);
        }
    }
    let target: Vec<Option<f64>> = d96_by_day
        .iter()
        .map(|v| {
            if v.is_empty() {
                None
            } else {
                Some(v.iter().sum::<f64>() / v.len() as f64)
            }
        })
        .collect();

    let mut goes_max = vec![f64::NAN; n_days];
    let mut goes_a_max = vec![f64::NAN; n_days];
    let mut flare_count = vec![0.0f64; n_days];
    for &(t, v) in &b_flux {
        let d = day_of(t).saturating_sub(day_min);
        if d >= n_days {
            continue;
        }
        if !goes_max[d].is_finite() || v > goes_max[d] {
            goes_max[d] = v;
        }
        if v >= FLARE_THRESH {
            flare_count[d] += 1.0;
        }
    }
    for &(t, v) in &a_flux {
        let d = day_of(t).saturating_sub(day_min);
        if d < n_days && (!goes_a_max[d].is_finite() || v > goes_a_max[d]) {
            goes_a_max[d] = v;
        }
    }
    let daily_max = |raw: &[f64]| -> Vec<Option<f64>> {
        raw.iter()
            .map(|&v| if v.is_finite() { Some(v) } else { None })
            .collect()
    };
    let goes_b_daily = daily_max(&goes_max);
    let goes_a_daily = daily_max(&goes_a_max);
    let flare_daily: Vec<Option<f64>> = flare_count.iter().map(|&v| Some(v)).collect();

    let mut cme_count = vec![0.0f64; n_days];
    let mut cme_speed = vec![f64::NAN; n_days];
    let mut cme_files = 0usize;
    for dir in &lasco_dirs {
        let Ok(entries) = std::fs::read_dir(dir) else {
            eprintln!("{dir} reads void — LASCO stays absent");
            continue;
        };
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if !name.ends_with(".txt") {
                continue;
            }
            let Ok(text) = std::fs::read_to_string(entry.path()) else {
                continue;
            };
            cme_files += 1;
            for r in soho_lasco::parse_text(&text) {
                let td = lsk
                    .as_ref()
                    .and_then(|l| l.unix_to_tdb(r.t_unix))
                    .unwrap_or(r.t_unix);
                let d = day_of(td).saturating_sub(day_min);
                if d >= n_days {
                    continue;
                }
                cme_count[d] += 1.0;
                if let Some(s) = r.speed_kms {
                    if !cme_speed[d].is_finite() || s > cme_speed[d] {
                        cme_speed[d] = s;
                    }
                }
            }
        }
    }
    let cme_daily: Vec<Option<f64>> = cme_count.iter().map(|&v| Some(v)).collect();
    let cme_speed_daily = daily_max(&cme_speed);

    let mut radio_count = vec![0.0f64; n_days];
    let mut radio_records = 0usize;
    let mut radio_common_days = 0usize;
    if let Some(path) = &radio_path {
        if let Ok(bytes) = std::fs::read(path) {
            if let Some(records) = radio_burst::parse_bin(&bytes) {
                radio_records = records.len();
                for r in records {
                    let td = lsk
                        .as_ref()
                        .and_then(|l| l.unix_to_tdb(r.t_unix))
                        .unwrap_or(r.t_unix);
                    let d = day_of(td).saturating_sub(day_min);
                    if d < n_days {
                        radio_common_days += 1;
                        radio_count[d] += 1.0;
                    }
                }
            }
        }
    }
    let radio_daily: Option<Vec<Option<f64>>> = if radio_common_days > 0 {
        Some(radio_count.iter().map(|&v| Some(v)).collect())
    } else {
        None
    };

    let leads: Vec<i32> = (-3..=3).collect();
    let mut results: Vec<DriverResult> = Vec::new();
    let mut drivers: Vec<(&'static str, Vec<Option<f64>>)> = vec![
        ("GOES-XRS b max", goes_b_daily.clone()),
        ("GOES-XRS a max", goes_a_daily.clone()),
        ("flare count (>=C5)", flare_daily.clone()),
    ];
    if cme_files > 0 {
        drivers.push(("LASCO CME count", cme_daily.clone()));
        drivers.push(("LASCO CME max speed", cme_speed_daily.clone()));
    }
    if let Some(r) = &radio_daily {
        drivers.push(("SWPC radio sweeps", r.clone()));
    }
    for (name, driver) in &drivers {
        let Some((r, lead, n)) = best_lead_r(driver, &target, &leads) else {
            continue;
        };
        let obs = r.abs();
        let mut null_best: Vec<f64> = Vec::with_capacity(N_SHIFT);
        for k in 7..=7 + N_SHIFT {
            let shifted = shift_series(driver, k);
            if let Some((rn, _, _)) = best_lead_r(&shifted, &target, &leads) {
                null_best.push(rn.abs());
            }
        }
        null_best.sort_by(|a, b| a.total_cmp(b));
        let null95 = if null_best.is_empty() {
            f64::NAN
        } else {
            null_best[(null_best.len() * 95 / 100).min(null_best.len() - 1)]
        };
        let exceed = null_best.iter().filter(|&&v| v >= obs).count();
        let p = if null_best.is_empty() {
            f64::NAN
        } else {
            (1 + exceed) as f64 / (1 + null_best.len()) as f64
        };

        let mut xs: Vec<f32> = Vec::new();
        let mut ys: Vec<f32> = Vec::new();
        for d in 0..n_days {
            if let (Some(a), Some(b)) = (driver[d], target[d]) {
                xs.push(a as f32);
                ys.push(b as f32);
            }
        }
        let mut te_fam: Option<f64> = None;
        let mut te0: Option<f64> = None;
        if xs.len() >= 30 {
            let mut pool: Vec<f64> = Vec::new();
            for lag in 0..=3 {
                let real = transfer_entropy_lag(&ys, &xs, lag);
                if lag == 0 {
                    te0 = real;
                }
                let mut rng = 0x51ED_2701u64 ^ (lag as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15);
                for _ in 0..N_SURR_DAILY {
                    let sur = phase_randomized_surrogate(&xs, &mut rng);
                    if let Some(t) = transfer_entropy_lag(&ys, &sur, lag) {
                        pool.push(t);
                    }
                }
            }
            if !pool.is_empty() {
                pool.sort_by(|a, b| a.total_cmp(b));
                te_fam = Some(pool[(pool.len() * 95 / 100).min(pool.len() - 1)]);
            }
        }
        results.push(DriverResult {
            name,
            n_days: n,
            r,
            lead,
            null95,
            p,
            te0,
            te_fam,
        });
    }

    let daily_grid_max = |g: &[Option<f32>]| -> Vec<Option<f64>> {
        let mut out: Vec<Option<f64>> = vec![None; n_days];
        for (k, v) in g.iter().enumerate() {
            let Some(x) = v else {
                continue;
            };
            let d = day_of(t0 + k as f64 * DT).saturating_sub(day_min);
            if d >= n_days {
                continue;
            }
            let x = *x as f64;
            if out[d].map_or(true, |b| x > b) {
                out[d] = Some(x);
            }
        }
        out
    };
    let hot_daily = daily_grid_max(&g131);
    let cool_daily = daily_grid_max(&g304);
    let mut controls: Vec<(&'static str, f64, i32, usize, f64, f64)> = Vec::new();
    for (name, target_c) in [("AIA 131 max", &hot_daily), ("AIA 304 max", &cool_daily)] {
        if let Some((r, lead, n)) = best_lead_r(&goes_b_daily, target_c, &leads) {
            let mut null_best: Vec<f64> = Vec::with_capacity(N_SHIFT);
            for k in 7..=7 + N_SHIFT {
                let shifted = shift_series(&goes_b_daily, k);
                if let Some((rn, _, _)) = best_lead_r(&shifted, target_c, &leads) {
                    null_best.push(rn.abs());
                }
            }
            null_best.sort_by(|a, b| a.total_cmp(b));
            let null95 = if null_best.is_empty() {
                f64::NAN
            } else {
                null_best[(null_best.len() * 95 / 100).min(null_best.len() - 1)]
            };
            let exceed = null_best.iter().filter(|&&v| v >= r.abs()).count();
            let p = (1 + exceed) as f64 / (1 + null_best.len()) as f64;
            controls.push((name, r, lead, n, null95, p));
        }
    }

    println!(
        "AIA ladder asset: {} 304-Å and {} 131-Å records, {} 24-s cells, {} days",
        s304.len(),
        s131.len(),
        bins,
        n_days
    );
    println!(
        "GOES-15 XRS: {} files, {} b_flux and {} a_flux good samples; events b_flux > {:.0e} W/m², ±40 min, refractory 30 min",
        goes_files,
        b_flux.len(),
        a_flux.len(),
        FLARE_THRESH
    );
    println!(
        "events: {} with a complete 304/131 window; D96 mean {:+.4e}, positive {}/{}; phase null 95 % {:+.4e} ({} surrogates, {} per event)",
        events.len(),
        event_mean,
        event_pos,
        events.len(),
        null95_event,
        null_pool.len(),
        N_SURR_EVENT
    );
    println!(
        "LASCO: {} monthly files, {} CMEs; SWPC radio: {} records, {} in the AIA era",
        cme_files,
        cme_count.iter().sum::<f64>(),
        radio_records,
        radio_common_days
    );
    if radio_daily.is_none() {
        println!(
            "SWPC radio: absent on the common days (rolling event list does not overlap 2014) — not a zero"
        );
    }
    println!();
    println!(
        "common days and daily coupling (target = daily mean 304→131 D at 96 s; lead > 0 = driver leads)"
    );
    println!(
        "{:<24} {:>6} {:>10} {:>6} {:>10} {:>8} {:>10} {:>10}",
        "driver", "n", "r(best)", "lead", "null95", "p", "TE(0 d)", "TE-null95"
    );
    for r in &results {
        println!(
            "{:<24} {:>6} {:>+10.4} {:>+6} {:>10.4} {:>8.4} {:>10} {:>10}",
            r.name,
            r.n_days,
            r.r,
            r.lead,
            r.null95,
            r.p,
            r.te0
                .map_or("absent".to_string(), |v| format!("{:+.3e}", v)),
            r.te_fam
                .map_or("absent".to_string(), |v| format!("{:+.3e}", v))
        );
    }
    println!();
    println!("positive control: same daily machinery, GOES-XRS b max → AIA daily maximum");
    println!(
        "{:<16} {:>6} {:>10} {:>6} {:>10} {:>8}",
        "target", "n", "r(best)", "lead", "null95", "p"
    );
    for (name, r, lead, n, null95, p) in &controls {
        println!(
            "{:<16} {:>6} {:>+10.4} {:>+6} {:>10.4} {:>8.4}",
            name, n, r, lead, null95, p
        );
    }
    println!();
    println!("A correlation is not a cause; the lead is a measured shift, not a mechanism.");
    println!("absences are named, not filled: radio/MRO without a common day stays absent.");
}
