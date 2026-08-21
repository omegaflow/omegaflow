use omegaflow::archivar::omni2::{parse_bin, COMP_BZ, COMP_N1800, COMP_V1800};
use omegaflow::archivar::{fetch_raw, parse_json, scalar_of, JsonVal};
use omegaflow::te::{phase_randomized_surrogate, surrogate_stats_phase, transfer_entropy_lag};
use std::time::{SystemTime, UNIX_EPOCH};

const SURROGATE_SEED: u64 = 0x9E37_79B9_7F4A_7C15;
const ABK_HAPI: &str = "https://imag-data.bgs.ac.uk/GIN_V1/hapi/data?id=ABK/best-avail/PT1M/xyzf";
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

fn load_omni2() -> Vec<(f64, f64, u32)> {
    match std::fs::read(OMNI2_BIN) {
        Ok(bytes) => match parse_bin(&bytes) {
            Some(recs) => recs,
            None => {
                eprintln!("{OMNI2_BIN} parses void — the top series stays unmeasured");
                Vec::new()
            }
        },
        Err(_) => {
            eprintln!("{OMNI2_BIN} reads void — the top series stays unmeasured");
            Vec::new()
        }
    }
}

fn harvest_abk_year(year: i64) -> Vec<(f64, f64)> {
    let now = now_unix();
    let mut daily_max: Vec<(f64, f64)> = Vec::new();
    let mut day_epoch = 0.0f64;
    let mut day_peak = f64::NEG_INFINITY;
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
        let url = format!("{ABK_HAPI}&start={start}&stop={stop}&format=json");
        let body = match fetch_raw(&url, None, &[], 600) {
            Some(b) => b,
            None => {
                eprintln!("ABK {year}-{month:02}: fetch void");
                continue;
            }
        };
        let j = match parse_json(&body) {
            Some(j) => j,
            None => {
                eprintln!("ABK {year}-{month:02}: body parses void");
                continue;
            }
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
                    let this_day = (t / DAY).floor() * DAY;
                    if this_day != day_epoch {
                        if day_peak.is_finite() {
                            daily_max.push((day_epoch, day_peak));
                        }
                        day_epoch = this_day;
                        day_peak = f64::NEG_INFINITY;
                    }
                    if dbdt > day_peak {
                        day_peak = dbdt;
                    }
                }
            }
            prev = Some((t, [vx, vy, vz]));
        }
    }
    if day_peak.is_finite() {
        daily_max.push((day_epoch, day_peak));
    }
    daily_max
}

fn bin_day(series: &[(f64, f64)], t0: f64, n: usize) -> Vec<Option<f32>> {
    let mut sums = vec![0.0f64; n];
    let mut counts = vec![0u32; n];
    for &(t, v) in series {
        let idx = ((t - t0) / DAY).floor();
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

fn row(from: &str, to: &str, to_s: &[f32], from_s: &[f32], lags: &[usize], fam: f64) {
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
                "{from:>12} → {to:<12} | lag {lag} d | TE {te:>10.4e} | thr {thr:>10.4e} (mean {mean:.3e}, σ {sd:.3e}) | fam {fam:.4e} | {fam_v}"
            );
        }
        None => {
            println!(
                "{from:>12} → {to:<12} | lag {lag} d | TE {te:>10.4e} | fam {fam:.4e} | threshold missing"
            );
        }
    }
}

fn main() {
    let now = now_unix();
    println!("=== Bz-Retro-Probe — der Treiber über 60 Jahre (Sturm-Ensemble) ===");
    println!("system time: {}", iso_utc(now));
    println!("Estimator: KDE-TE (Silverman), Familien-Schwelle = max Surrogat-TE der Runde (Mehrfachvergleichskorrektur), phasenrandomisierte Surrogate (f64 FFT, 10 Realisierungen).");

    let omni = load_omni2();
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
        "omni2_serie.bin: Bz {:<6} | Speed {:<6} | Density {:<6} (Tages-Decimation 1440 min, 1963→2026)",
        omni_bz.len(),
        omni_speed.len(),
        omni_density.len()
    );

    let mut dbdt_daily: Vec<(f64, f64)> = Vec::new();
    let this_year = 2026i64;
    for y in FIRST_YEAR..=this_year {
        let mut days = harvest_abk_year(y);
        if days.is_empty() {
            eprintln!("ABK {y}: no daily cells — das Jahr bleibt leer");
            continue;
        }
        eprintln!("ABK {y}: {} Tage geerntet (daily max |dB/dt|)", days.len());
        dbdt_daily.append(&mut days);
    }
    dbdt_daily.sort_by(|a, b| a.0.total_cmp(&b.0));
    println!(
        "ABK daily max |dB/dt|: {} Tage ({} → {})",
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

    let bz = bin_day(&omni_bz, t0, n_days);
    let speed = bin_day(&omni_speed, t0, n_days);
    let density = bin_day(&omni_density, t0, n_days);
    let dbdt = bin_day(&dbdt_daily, t0, n_days);

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
    println!("=== Paar-Urteile (Schwelle am besten Lag, fam = Mehrfachvergleichskorrektur) ===");
    let fam = family_bound(&pairs, &lags);
    println!("fam = {fam:.4e}");
    for (from, to, to_s, from_s) in pairs.iter() {
        row(from, to, to_s, from_s, &lags, fam);
    }

    println!();
    println!("=== DAS BLATT (Retro-Zeile) ===");
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
                let fam_v = if te > fam { "Pfeil" } else { "family bound" };
                format!(
                    "TE({from} → dB/dt) = {te:.4e} | fam {fam:.4e} | Lag {lag} d | n {} | {fam_v}",
                    to_s.len()
                )
            }
            None => format!("TE({from} → dB/dt) = pending"),
        }
    };
    println!("{}", headline("Bz", &dbdt_bz, &bz_dbdt));
    println!("{}", headline("Speed", &dbdt_speed, &speed_dbdt));
    println!(
        "Fenster: {} → {} | Tages-Raster | ABK daily max |dB/dt| × OMNI2-Tagesmittel | ABK = Auroral-Zone (68.36°N)",
        iso_utc(t0),
        iso_utc(t0 + n_days as f64 * DAY)
    );
    println!("Urteil: was die Maschine misst — still ist ein Befund (0 honored).");
    println!("Grenze benannt: das Tages-Raster identifiziert den TREIBER, nicht den Minuten-Lag; der Minuten-Pfeil (live, 60 min) ist eine eigene Zeile.");
}
