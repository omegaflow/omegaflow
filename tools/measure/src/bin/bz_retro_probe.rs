use omegaflow::archivar::omni2::{parse_bin, COMP_BZ, COMP_N1800, COMP_V1800};
use omegaflow::archivar::{fetch_raw, parse_json, scalar_of, JsonVal};
use omegaflow::te::{phase_randomized_surrogate, surrogate_stats_phase, transfer_entropy_lag};
use std::time::{SystemTime, UNIX_EPOCH};

const SURROGATE_SEED: u64 = 0x9E37_79B9_7F4A_7C15;
const STATION_HAPI: &str =
    "https://imag-data.bgs.ac.uk/GIN_V1/hapi/data?id={station}/best-avail/PT1M/xyzf";
const FILL_NT: f64 = 99999.0;
const MINUTE: f64 = 60.0;
const HOUR: f64 = 3600.0;
const DAY: f64 = 86400.0;
const OMNI2_BIN: &str = "omni2_serie.bin";
const FIRST_YEAR: i64 = 1994;

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

fn load_omni2(path: &str) -> Vec<(f64, f64, u32)> {
    match std::fs::read(path) {
        Ok(bytes) => match parse_bin(&bytes) {
            Some(recs) => recs,
            None => {
                eprintln!("{path} parses void — the top series stays unmeasured");
                Vec::new()
            }
        },
        Err(_) => {
            eprintln!("{path} reads void — the top series stays unmeasured");
            Vec::new()
        }
    }
}

const CACHE_BASE: &str = "abk_dbdt_daily";

fn disk_cache(name: &str) -> String {
    omegaflow::archivar::cache_root()
        .join(name)
        .to_string_lossy()
        .into_owned()
}

fn load_cache(path: &str) -> Option<Vec<(f64, f64)>> {
    let body = std::fs::read_to_string(path).ok()?;
    let mut out = Vec::new();
    for line in body.lines() {
        let mut it = line.split_whitespace();
        let t: f64 = it.next()?.parse().ok()?;
        let v: f64 = it.next()?.parse().ok()?;
        out.push((t, v));
    }
    Some(out)
}

fn write_cache(path: &str, series: &[(f64, f64)]) {
    let mut s = String::new();
    for &(t, v) in series {
        s.push_str(&format!("{t} {v}\n"));
    }
    let _ = std::fs::write(path, s);
}

fn harvest_station_year_buckets(station: &str, year: i64, bucket_s: f64) -> Vec<(f64, f64)> {
    let now = now_unix();
    let mut peaks: Vec<(f64, f64)> = Vec::new();
    let mut bucket_epoch = 0.0f64;
    let mut bucket_peak = f64::NEG_INFINITY;
    let mut prev: Option<(f64, [f64; 3])> = None;
    for month in 1..=12i64 {
        let (ny, nm) = if month == 12 {
            (year + 1, 1)
        } else {
            (year, month + 1)
        };
        let start = format!("{year:04}-{month:02}-01T00:00:00Z");
        let mut stop = format!("{ny:04}-{nm:02}-01T00:00:00Z");
        if iso_to_unix(&stop).unwrap_or(0.0) > now - 2.0 * HOUR {
            stop = iso_utc(now - 2.0 * HOUR);
        }
        if iso_to_unix(&stop).unwrap_or(0.0) <= iso_to_unix(&start).unwrap_or(0.0) {
            continue;
        }
        let url = format!(
            "{}&start={start}&stop={stop}&format=json",
            STATION_HAPI.replace("{station}", station)
        );
        let mut root_json: Option<omegaflow::archivar::JsonVal> = None;
        for attempt in 0..3 {
            if attempt > 0 {
                std::thread::sleep(std::time::Duration::from_secs(20));
            }
            match fetch_raw(&url, None, &[], 600).and_then(|b| parse_json(&b)) {
                Some(j) => {
                    root_json = Some(j);
                    break;
                }
                None => {
                    eprintln!(
                        "{station} {year}-{month:02}: fetch void (attempt {})",
                        attempt + 1
                    );
                }
            }
        }
        let Some(j) = root_json else {
            continue;
        };
        let JsonVal::Obj(root) = j else {
            continue;
        };
        let Some(JsonVal::Arr(data)) = root.get("data") else {
            continue;
        };
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
            let comp = match (cx, cy, cz) {
                (Some(a), Some(b), Some(c))
                    if a.is_finite()
                        && b.is_finite()
                        && c.is_finite()
                        && a != FILL_NT
                        && b != FILL_NT
                        && c != FILL_NT =>
                {
                    Some((a, b, c))
                }
                _ => None,
            };
            let Some((vx, vy, vz)) = comp else {
                prev = None;
                continue;
            };
            if let Some((pt, [px, py, pz])) = prev {
                let dt = t - pt;
                if (MINUTE - 2.0..=MINUTE + 2.0).contains(&dt) {
                    let dx = vx - px;
                    let dy = vy - py;
                    let dz = vz - pz;
                    let dbdt = (dx * dx + dy * dy + dz * dz).sqrt();
                    let this_bucket = (t / bucket_s).floor() * bucket_s;
                    if this_bucket != bucket_epoch {
                        if bucket_peak.is_finite() {
                            peaks.push((bucket_epoch, bucket_peak));
                        }
                        bucket_epoch = this_bucket;
                        bucket_peak = f64::NEG_INFINITY;
                    }
                    if dbdt > bucket_peak {
                        bucket_peak = dbdt;
                    }
                }
            }
            prev = Some((t, [vx, vy, vz]));
        }
    }
    if bucket_peak.is_finite() {
        peaks.push((bucket_epoch, bucket_peak));
    }
    peaks
}

fn bin_cells(series: &[(f64, f64)], t0: f64, dt: f64, n: usize) -> Vec<Option<f32>> {
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

fn row(from: &str, to: &str, to_s: &[f32], from_s: &[f32], lags: &[usize], fam: f64, unit: &str) {
    let mut best: Option<(usize, f64)> = None;
    for &lag in lags {
        if let Some(te) = transfer_entropy_lag(to_s, from_s, lag) {
            if best.map_or(true, |(_, b)| te > b) {
                best = Some((lag, te));
            }
        }
    }
    let Some((lag, te)) = best else {
        println!("{from:>12} → {to:<12} | no TE (n < 8)");
        return;
    };
    match surrogate_stats_phase(to_s, from_s, lag, SURROGATE_SEED) {
        Some((mean, sd, thr)) => {
            let fam_v = if te > fam { "arrow" } else { "family bound" };
            println!(
                "{from:>12} → {to:<12} | lag {lag} {unit} | TE {te:>10.4e} | thr {thr:>10.4e} (mean {mean:.3e}, σ {sd:.3e}) | fam {fam:.4e} | {fam_v}"
            );
        }
        None => {
            println!(
                "{from:>12} → {to:<12} | lag {lag} {unit} | TE {te:>10.4e} | fam {fam:.4e} | threshold absent"
            );
        }
    }
}

fn family_bound_resumable(
    pairs: &[(&str, &str, &[f32], &[f32])],
    lags: &[usize],
    state_path: &str,
) -> f64 {
    let mut chunks_done = 0usize;
    let mut fam = f64::NEG_INFINITY;
    if let Ok(s) = std::fs::read_to_string(state_path) {
        let mut it = s.split_whitespace();
        if let (Some(c), Some(f)) = (it.next(), it.next()) {
            if let (Ok(cd), Ok(fv)) = (c.parse::<usize>(), f.parse::<f64>()) {
                chunks_done = cd.min(12);
                fam = fv;
            }
        }
    }
    let mut idx = 0usize;
    for (_, _, to, from) in pairs {
        for &lag in lags {
            if idx >= chunks_done {
                for te in surrogate_te_values(to, from, lag, SURROGATE_SEED) {
                    if te > fam {
                        fam = te;
                    }
                }
                let _ = std::fs::write(state_path, format!("{} {:.17e}", idx + 1, fam));
                eprintln!("fam chunk {}/12: running fam {fam:.4e}", idx + 1);
            }
            idx += 1;
        }
    }
    fam
}

fn run_hourly(
    station: &str,
    hour_start: &str,
    hour_end: &str,
    harvest_only: bool,
    force_harvest: bool,
    now: f64,
) {
    let sy: i64 = hour_start
        .get(..4)
        .and_then(|v| v.parse().ok())
        .unwrap_or(2024);
    let ey: i64 = hour_end
        .get(..4)
        .and_then(|v| v.parse().ok())
        .unwrap_or(2026);
    let cache_1h = disk_cache(&format!("abk_dbdt_1h_{station}_{sy}.tsv"));
    let omni2_1h = disk_cache("omni2_serie_1h.bin").to_string();
    let h_start = iso_to_unix(&format!("{hour_start}T00:00:00Z")).unwrap_or(0.0);
    let h_end = iso_to_unix(&format!("{hour_end}T00:00:00Z")).unwrap_or(now - 2.0 * HOUR);

    let omni = load_omni2(&omni2_1h);
    let omni_bz: Vec<(f64, f64)> = omni
        .iter()
        .filter(|(_, _, c)| *c == COMP_BZ)
        .map(|&(t, v, _)| (t, v))
        .collect();
    let omni_speed: Vec<(f64, f64)> = omni
        .iter()
        .filter(|(_, _, c)| *c == COMP_V1800)
        .map(|&(t, v, _)| (t, v))
        .collect();
    let omni_density: Vec<(f64, f64)> = omni
        .iter()
        .filter(|(_, _, c)| *c == COMP_N1800)
        .map(|&(t, v, _)| (t, v))
        .collect();
    println!(
        "omni2_serie_1h.bin: Bz {:<6} | Speed {:<6} | Density {:<6} (60-min-Buckets, 1994→2026)",
        omni_bz.len(),
        omni_speed.len(),
        omni_density.len()
    );

    let mut dbdt_1h: Vec<(f64, f64)> = if !force_harvest {
        match load_cache(&cache_1h) {
            Some(c) if !c.is_empty() => {
                eprintln!(
                    "cache {cache_1h}: {} hours loaded — the harvest is skipped",
                    c.len()
                );
                c
            }
            _ => Vec::new(),
        }
    } else {
        Vec::new()
    };
    if dbdt_1h.is_empty() {
        for y in sy..=ey {
            let mut hours = harvest_station_year_buckets(station, y, HOUR);
            if hours.is_empty() {
                eprintln!("ABK {y}: no hourly cells — the year stays empty");
                continue;
            }
            eprintln!(
                "ABK {y}: {} hours harvested (hourly max |dB/dt|)",
                hours.len()
            );
            dbdt_1h.append(&mut hours);
        }
        dbdt_1h.sort_by(|a, b| a.0.total_cmp(&b.0));
        write_cache(&cache_1h, &dbdt_1h);
        eprintln!("cache {cache_1h}: {} hours written", dbdt_1h.len());
    }
    if harvest_only {
        println!(
            "Harvest complete — cache {cache_1h} stands; the measurement run without --harvest-only."
        );
        return;
    }
    println!(
        "ABK hourly max |dB/dt|: {} hours ({} → {})",
        dbdt_1h.len(),
        iso_utc(dbdt_1h.first().map(|&(t, _)| t).unwrap_or(0.0)),
        iso_utc(dbdt_1h.last().map(|&(t, _)| t).unwrap_or(0.0))
    );

    let lo = omni_bz
        .first()
        .map(|&(t, _)| t)
        .unwrap_or(0.0)
        .max(dbdt_1h.first().map(|&(t, _)| t).unwrap_or(0.0))
        .max(h_start);
    let hi = omni_bz
        .last()
        .map(|&(t, _)| t)
        .unwrap_or(0.0)
        .min(dbdt_1h.last().map(|&(t, _)| t).unwrap_or(0.0))
        .min(h_end);
    let t0 = (lo / HOUR).floor() * HOUR;
    let n_cells = ((hi - t0) / HOUR).floor().max(1.0) as usize;
    println!(
        "common hourly window: {} → {} | {} hours",
        iso_utc(t0),
        iso_utc(t0 + n_cells as f64 * HOUR),
        n_cells
    );

    let bz = bin_cells(&omni_bz, t0, HOUR, n_cells);
    let speed = bin_cells(&omni_speed, t0, HOUR, n_cells);
    let density = bin_cells(&omni_density, t0, HOUR, n_cells);
    let dbdt = bin_cells(&dbdt_1h, t0, HOUR, n_cells);

    let (dbdt_bz, bz_dbdt) = pair_cells(&dbdt, &bz);
    let (dbdt_speed, speed_dbdt) = pair_cells(&dbdt, &speed);
    let (dbdt_density, density_dbdt) = pair_cells(&dbdt, &density);
    println!(
        "paired hours: Bz {:<6} | Speed {:<6} | Density {:<6}",
        bz_dbdt.len(),
        speed_dbdt.len(),
        density_dbdt.len()
    );

    let lags: [usize; 2] = [0, 1];
    let pairs: [(&str, &str, &[f32], &[f32]); 6] = [
        ("Bz", "dB/dt", &dbdt_bz, &bz_dbdt),
        ("dB/dt", "Bz", &bz_dbdt, &dbdt_bz),
        ("Speed", "dB/dt", &dbdt_speed, &speed_dbdt),
        ("dB/dt", "Speed", &speed_dbdt, &dbdt_speed),
        ("Density", "dB/dt", &dbdt_density, &density_dbdt),
        ("dB/dt", "Density", &density_dbdt, &dbdt_density),
    ];

    println!();
    println!(
        "=== Pair verdicts (threshold at the best lag, fam = multiple-comparison correction) ==="
    );
    let fam_state = format!("fam_state_{station}_{sy}.txt");
    let fam = family_bound_resumable(&pairs, &lags, &fam_state);
    println!("fam = {fam:.4e}");
    for (from, to, to_s, from_s) in pairs.iter() {
        row(from, to, to_s, from_s, &lags, fam, "h");
    }

    println!();
    println!("=== THE BLATT (1-h row) ===");
    let headline = |from: &str, to_s: &[f32], from_s: &[f32]| -> String {
        let mut best: Option<(usize, f64)> = None;
        for &lag in &lags {
            if let Some(te) = transfer_entropy_lag(to_s, from_s, lag) {
                if best.map_or(true, |(_, b)| te > b) {
                    best = Some((lag, te));
                }
            }
        }
        match best {
            Some((lag, te)) => {
                let fam_v = if te > fam { "arrow" } else { "family bound" };
                format!(
                    "TE({from} → dB/dt) = {te:.4e} | fam {fam:.4e} | lag {lag} h | n {} | {fam_v}",
                    to_s.len()
                )
            }
            None => format!("TE({from} → dB/dt) = pending"),
        }
    };
    println!("{}", headline("Bz", &dbdt_bz, &bz_dbdt));
    println!("{}", headline("Speed", &dbdt_speed, &speed_dbdt));
    println!(
        "Window: {} → {} | 1-h grid | ABK hourly max |dB/dt| × OMNI2 hourly (60-min buckets)",
        iso_utc(t0),
        iso_utc(t0 + n_cells as f64 * HOUR)
    );
    println!("Verdict: what the machine measures — still is a finding (0 honored).");
    println!(
        "Boundary named: lag 1 h straddles the L1 travel time (30–60 min); the hourly arrow is the fam test over the storm year."
    );
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let harvest_only = args.iter().any(|a| a == "--harvest-only");
    let force_harvest = args.iter().any(|a| a == "--force-harvest");
    let hourly = args.iter().any(|a| a == "--hourly");
    let station = args
        .iter()
        .position(|a| a == "--station")
        .and_then(|i| args.get(i + 1))
        .cloned()
        .unwrap_or_else(|| "ABK".to_string());
    let hour_start = args
        .iter()
        .position(|a| a == "--hour-start")
        .and_then(|i| args.get(i + 1))
        .cloned()
        .unwrap_or_else(|| "2024-01-01".to_string());
    let hour_end = args
        .iter()
        .position(|a| a == "--hour-end")
        .and_then(|i| args.get(i + 1))
        .cloned()
        .unwrap_or_else(|| "2024-12-31".to_string());
    let stride: usize = args
        .iter()
        .position(|a| a == "--stride")
        .and_then(|i| args.get(i + 1))
        .and_then(|v| v.parse().ok())
        .unwrap_or(1)
        .max(1);
    let now = now_unix();
    println!("=== Bz retro probe — the driver over 60 years (storm ensemble) ===");
    println!("system time: {}", iso_utc(now));
    println!(
        "Estimator: KDE-TE (Silverman), family threshold = max surrogate TE of the round (multiple-comparison correction), phase-randomized surrogates (f64 FFT, 10 realizations)."
    );

    if hourly {
        run_hourly(
            &station,
            &hour_start,
            &hour_end,
            harvest_only,
            force_harvest,
            now,
        );
        return;
    }

    let omni = load_omni2(&disk_cache(OMNI2_BIN));
    let omni_bz: Vec<(f64, f64)> = omni
        .iter()
        .filter(|(_, _, c)| *c == COMP_BZ)
        .map(|&(t, v, _)| (t, v))
        .collect();
    let omni_speed: Vec<(f64, f64)> = omni
        .iter()
        .filter(|(_, _, c)| *c == COMP_V1800)
        .map(|&(t, v, _)| (t, v))
        .collect();
    let omni_density: Vec<(f64, f64)> = omni
        .iter()
        .filter(|(_, _, c)| *c == COMP_N1800)
        .map(|&(t, v, _)| (t, v))
        .collect();
    println!(
        "omni2_serie.bin: Bz {:<6} | Speed {:<6} | Density {:<6} (daily decimation 1440 min, 1963→2026)",
        omni_bz.len(),
        omni_speed.len(),
        omni_density.len()
    );

    let mut dbdt_daily: Vec<(f64, f64)> = if !force_harvest {
        match load_cache(&disk_cache(&format!("{CACHE_BASE}_{station}.tsv"))) {
            Some(c) if !c.is_empty() => {
                eprintln!(
                    "cache {CACHE_BASE}_{station}: {} days loaded — the harvest is skipped",
                    c.len()
                );
                c
            }
            _ => Vec::new(),
        }
    } else {
        Vec::new()
    };
    if dbdt_daily.is_empty() {
        let this_year = 2026i64;
        for y in FIRST_YEAR..=this_year {
            let mut days = harvest_station_year_buckets(&station, y, DAY);
            if days.is_empty() {
                eprintln!("ABK {y}: no daily cells — the year stays empty");
                continue;
            }
            eprintln!("ABK {y}: {} days harvested (daily max |dB/dt|)", days.len());
            dbdt_daily.append(&mut days);
        }
        dbdt_daily.sort_by(|a, b| a.0.total_cmp(&b.0));
        write_cache(
            &disk_cache(&format!("{CACHE_BASE}_{station}.tsv")),
            &dbdt_daily,
        );
        eprintln!(
            "cache {CACHE_BASE}_{station}: {} days written",
            dbdt_daily.len()
        );
    }
    if harvest_only {
        println!(
            "Harvest complete — cache {CACHE_BASE}_{station} stands; the measurement run without --harvest-only."
        );
        return;
    }
    println!(
        "ABK daily max |dB/dt|: {} days ({} → {})",
        dbdt_daily.len(),
        iso_utc(dbdt_daily.first().map(|&(t, _)| t).unwrap_or(0.0)),
        iso_utc(dbdt_daily.last().map(|&(t, _)| t).unwrap_or(0.0))
    );

    let lo = omni_bz
        .first()
        .map(|&(t, _)| t)
        .unwrap_or(0.0)
        .max(dbdt_daily.first().map(|&(t, _)| t).unwrap_or(0.0));
    let hi = omni_bz
        .last()
        .map(|&(t, _)| t)
        .unwrap_or(0.0)
        .min(dbdt_daily.last().map(|&(t, _)| t).unwrap_or(0.0));
    let t0 = (lo / DAY).floor() * DAY;
    let n_days = ((hi - t0) / DAY).floor().max(1.0) as usize;
    println!(
        "common daily window: {} → {} | {} days",
        iso_utc(t0),
        iso_utc(t0 + n_days as f64 * DAY),
        n_days
    );

    let mut bz = bin_cells(&omni_bz, t0, DAY, n_days);
    let mut speed = bin_cells(&omni_speed, t0, DAY, n_days);
    let mut density = bin_cells(&omni_density, t0, DAY, n_days);
    let mut dbdt = bin_cells(&dbdt_daily, t0, DAY, n_days);
    if stride > 1 {
        bz = bz.into_iter().step_by(stride).collect();
        speed = speed.into_iter().step_by(stride).collect();
        density = density.into_iter().step_by(stride).collect();
        dbdt = dbdt.into_iter().step_by(stride).collect();
        println!(
            "named subgrid: every {stride}. day (lag 1 = {stride} days; lag 0 stays the storm day)"
        );
    }

    let (dbdt_bz, bz_dbdt) = pair_cells(&dbdt, &bz);
    let (dbdt_speed, speed_dbdt) = pair_cells(&dbdt, &speed);
    let (dbdt_density, density_dbdt) = pair_cells(&dbdt, &density);
    println!(
        "paired days: Bz {:<6} | Speed {:<6} | Density {:<6}",
        bz_dbdt.len(),
        speed_dbdt.len(),
        density_dbdt.len()
    );

    let lags: [usize; 2] = [0, 1];
    let pairs: [(&str, &str, &[f32], &[f32]); 6] = [
        ("Bz", "dB/dt", &dbdt_bz, &bz_dbdt),
        ("dB/dt", "Bz", &bz_dbdt, &dbdt_bz),
        ("Speed", "dB/dt", &dbdt_speed, &speed_dbdt),
        ("dB/dt", "Speed", &speed_dbdt, &dbdt_speed),
        ("Density", "dB/dt", &dbdt_density, &density_dbdt),
        ("dB/dt", "Density", &density_dbdt, &dbdt_density),
    ];

    println!();
    println!(
        "=== Pair verdicts (threshold at the best lag, fam = multiple-comparison correction) ==="
    );
    let fam = family_bound(&pairs, &lags);
    println!("fam = {fam:.4e}");
    for (from, to, to_s, from_s) in pairs.iter() {
        row(from, to, to_s, from_s, &lags, fam, "d");
    }

    println!();
    println!("=== THE BLATT (retro row) ===");
    let headline = |from: &str, to_s: &[f32], from_s: &[f32]| -> String {
        let mut best: Option<(usize, f64)> = None;
        for &lag in &lags {
            if let Some(te) = transfer_entropy_lag(to_s, from_s, lag) {
                if best.map_or(true, |(_, b)| te > b) {
                    best = Some((lag, te));
                }
            }
        }
        match best {
            Some((lag, te)) => {
                let fam_v = if te > fam { "arrow" } else { "family bound" };
                format!(
                    "TE({from} → dB/dt) = {te:.4e} | fam {fam:.4e} | lag {lag} d | n {} | {fam_v}",
                    to_s.len()
                )
            }
            None => format!("TE({from} → dB/dt) = pending"),
        }
    };
    println!("{}", headline("Bz", &dbdt_bz, &bz_dbdt));
    println!("{}", headline("Speed", &dbdt_speed, &speed_dbdt));
    println!(
        "Window: {} → {} | day grid | ABK daily max |dB/dt| × OMNI2 daily mean | ABK = auroral zone (68.36°N)",
        iso_utc(t0),
        iso_utc(t0 + n_days as f64 * DAY)
    );
    println!("Verdict: what the machine measures — still is a finding (0 honored).");
    println!(
        "Boundary named: the day grid identifies the DRIVER, not the minute lag; the minute arrow (live, 60 min) is its own row."
    );
}
