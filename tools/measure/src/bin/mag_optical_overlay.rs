use omegaflow::archivar::{JsonVal, fetch_raw, parse_json, scalar_of};
use std::path::Path;

const HAPI: &str = "https://imag-data.bgs.ac.uk/GIN_V1/hapi/data?id={station}/best-avail/PT1M/xyzf";
const FILL_NT: f64 = 99999.0;
const DAY: f64 = 86400.0;
const HOUR: f64 = 3600.0;
const MINUTE_SLOTS: usize = 1440;

const ORIENTATION: &str = "magnetometer = INTERMAGNET HAPI 1-min xyzf; the reported value is |dH/dt| in nT/min (forward difference of the horizontal X/Y components); optical = per-minute mean of the THEMIS-ASI meridian-brightness profile (TKDG)";

fn arg_value(args: &[String], key: &str) -> Option<String> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn iso_to_unix(s: &str) -> Option<f64> {
    let s = s.trim();
    let (date, time) = match s.split_once('T') {
        Some((d, t)) => (d, t),
        None => s.split_once(' ')?,
    };
    let mut dp = date.split('-');
    let y: i64 = dp.next()?.parse().ok()?;
    let m: i64 = dp.next()?.parse().ok()?;
    let d: i64 = dp.next()?.parse().ok()?;
    let t = time.split(['.', 'Z', 'z']).next()?;
    let mut tp = t.split(':');
    let hh: i64 = tp.next()?.parse().ok()?;
    let mm: i64 = tp.next().unwrap_or("0").parse().ok()?;
    let ss: i64 = tp.next().unwrap_or("0").parse().ok()?;
    let a = (14 - m) / 12;
    let yy = y + 4800 - a;
    let jdn =
        d + (153 * (m + 12 * a - 3) + 2) / 5 + 365 * yy + yy / 4 - yy / 100 + yy / 400 - 32045;
    Some((jdn - 2440588) as f64 * DAY + hh as f64 * HOUR + mm as f64 * 60.0 + ss as f64)
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
    let mm = ((day_secs - hh as f64 * HOUR) / 60.0) as i64;
    let ss = (day_secs - hh as f64 * HOUR - mm as f64 * 60.0) as i64;
    format!("{y:04}-{m:02}-{d:02}T{hh:02}:{mm:02}:{ss:02}Z")
}

fn read_tkdg_profiles(path: &str) -> Result<Vec<(f64, Vec<f64>)>, String> {
    let data = std::fs::read(Path::new(path)).map_err(|e| format!("{path}: {e}"))?;
    if data.len() < 8 || &data[0..4] != b"TKDG" {
        return Err(format!("{path}: not a TKDG file"));
    }
    let count = u32::from_le_bytes([data[4], data[5], data[6], data[7]]) as usize;
    let frame_bytes = 8 + 256 * 2;
    if data.len() < 8 + count * frame_bytes {
        return Err(format!(
            "{path}: truncated — {} bytes for {count} frames",
            data.len()
        ));
    }
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        let base = 8 + i * frame_bytes;
        let epoch = f64::from_le_bytes(data[base..base + 8].try_into().unwrap());
        let mut rows = Vec::with_capacity(256);
        for r in 0..256 {
            let o = base + 8 + r * 2;
            rows.push(u16::from_le_bytes([data[o], data[o + 1]]) as f64);
        }
        out.push((epoch, rows));
    }
    Ok(out)
}

fn read_tkdg(path: &str) -> Result<Vec<(f64, f64)>, String> {
    Ok(read_tkdg_profiles(path)?
        .into_iter()
        .map(|(epoch, rows)| {
            let mean = rows.iter().sum::<f64>() / rows.len() as f64;
            (epoch, mean)
        })
        .collect())
}

fn read_fodg(path: &str) -> Result<Vec<(f64, Vec<f32>)>, String> {
    let data = std::fs::read(Path::new(path)).map_err(|e| format!("{path}: {e}"))?;
    if data.len() < 16 || &data[0..4] != b"FODG" {
        return Err(format!("{path}: not a FODG full-frame cube"));
    }
    let az_bins = u32::from_le_bytes([data[4], data[5], data[6], data[7]]) as usize;
    let el_bins = u32::from_le_bytes([data[8], data[9], data[10], data[11]]) as usize;
    let count = u32::from_le_bytes([data[12], data[13], data[14], data[15]]) as usize;
    let cells = az_bins * el_bins;
    let frame_bytes = 8 + cells * 4;
    if cells == 0 || data.len() < 16 + count * frame_bytes {
        return Err(format!(
            "{path}: truncated — {} bytes for {count}×{cells}-cell frames",
            data.len()
        ));
    }
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        let base = 16 + i * frame_bytes;
        let epoch = f64::from_le_bytes(data[base..base + 8].try_into().unwrap());
        let mut rows = Vec::with_capacity(cells);
        for c in 0..cells {
            let o = base + 8 + c * 4;
            rows.push(f32::from_le_bytes([
                data[o],
                data[o + 1],
                data[o + 2],
                data[o + 3],
            ]));
        }
        out.push((epoch, rows));
    }
    Ok(out)
}

fn median(values: &mut [f64]) -> f64 {
    values.sort_by(|a, b| a.total_cmp(b));
    let n = values.len();
    if n == 0 {
        return f64::NAN;
    }
    if n % 2 == 1 {
        values[n / 2]
    } else {
        (values[n / 2 - 1] + values[n / 2]) / 2.0
    }
}

fn cube_cell_onset(
    series: &[(f64, f64)],
    window: usize,
    k: f64,
    persist: usize,
    search_from: f64,
) -> Option<f64> {
    if series.len() <= window {
        return None;
    }
    let mut run = 0usize;
    for i in window..series.len() {
        let (epoch, value) = series[i];
        if epoch < search_from || !value.is_finite() {
            run = 0;
            continue;
        }
        let mut base: Vec<f64> = series[i - window..i]
            .iter()
            .map(|(_, v)| *v)
            .filter(|v| v.is_finite())
            .collect();
        if base.len() < 5 {
            run = 0;
            continue;
        }
        let med = median(&mut base);
        let mut dev: Vec<f64> = base.iter().map(|v| (v - med).abs()).collect();
        let mad = median(&mut dev);
        let threshold = med + k * ROBUST_SIGMA * mad;
        if value > threshold {
            run += 1;
            if run >= persist {
                return Some(series[i + 1 - run].0);
            }
        } else {
            run = 0;
        }
    }
    None
}

fn cube_onset(
    frames: &[(f64, Vec<f32>)],
    window: usize,
    k: f64,
    persist: usize,
    search_from: f64,
) -> Option<f64> {
    let cells = match frames.first() {
        Some(f) => f.1.len(),
        None => return None,
    };
    if cells == 0 {
        return None;
    }
    let bg: Vec<f64> = frames
        .iter()
        .map(|(_, c)| {
            let mut v: Vec<f64> = c
                .iter()
                .map(|x| *x as f64)
                .filter(|x| x.is_finite())
                .collect();
            median(&mut v)
        })
        .collect();
    let mut earliest: Option<f64> = None;
    for cell in 0..cells {
        let series: Vec<(f64, f64)> = frames
            .iter()
            .enumerate()
            .map(|(i, f)| (f.0, f.1[cell] as f64 - bg[i]))
            .collect();
        if let Some(epoch) = cube_cell_onset(&series, window, k, persist, search_from) {
            earliest = Some(match earliest {
                Some(e) => e.min(epoch),
                None => epoch,
            });
        }
    }
    earliest
}

fn fetch_day(station: &str, date: &str) -> Result<Vec<(f64, f64, f64, f64)>, String> {
    let day_start = iso_to_unix(&format!("{date}T00:00:00Z"))
        .ok_or_else(|| format!("{date} is not an ISO day"))?;
    let day_end = day_start + DAY;
    let url = format!(
        "{HAPI}&time.min={}&time.max={}&format=json",
        iso_utc(day_start),
        iso_utc(day_end)
    );
    let body = fetch_raw(&url.replace("{station}", station), None, &[])
        .ok_or_else(|| format!("{station}: fetch void"))?;
    let root = parse_json(&body).ok_or_else(|| format!("{station}: json parse void"))?;
    let JsonVal::Obj(root) = root else {
        return Err(format!("{station}: root is not an object"));
    };
    let JsonVal::Arr(data) = root.get("data").ok_or("no data")? else {
        return Err(format!("{station}: no data array"));
    };
    let mut rows: Vec<(f64, f64, f64, f64)> = Vec::new();
    for row in data {
        let JsonVal::Arr(cells) = row else { continue };
        let Some(t) = cells.first().and_then(|c| match c {
            JsonVal::Str(s) => iso_to_unix(s),
            _ => None,
        }) else {
            continue;
        };
        let JsonVal::Arr(vec) = &cells[1] else {
            continue;
        };
        let x = vec.first().and_then(scalar_of);
        let y = vec.get(1).and_then(scalar_of);
        let z = vec.get(2).and_then(scalar_of);
        if let (Some(x), Some(y), Some(z)) = (x, y, z)
            && x != FILL_NT
            && y != FILL_NT
            && z != FILL_NT
        {
            rows.push((t, x, y, z));
        }
    }
    rows.sort_by(|a, b| a.0.total_cmp(&b.0));
    let mut out = Vec::with_capacity(rows.len());
    for i in 1..rows.len() {
        let (t1, x1, y1, _) = rows[i - 1];
        let (t2, x2, y2, _) = rows[i];
        let dt = t2 - t1;
        if dt <= 0.0 {
            continue;
        }
        let dhdt = ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt() / (dt / 60.0);
        out.push((t2, dhdt, x2, y2));
    }
    Ok(out)
}

fn minute_slot(epoch: f64, day_start: f64) -> Option<usize> {
    let off = epoch - day_start;
    if !(0.0..DAY).contains(&off) {
        return None;
    }
    Some((off / 60.0) as usize)
}

const ROBUST_SIGMA: f64 = 1.4826;

fn median_mad(values: &[f64]) -> (f64, f64) {
    let mut v: Vec<f64> = values.to_vec();
    v.sort_by(|a, b| a.total_cmp(b));
    if v.is_empty() {
        return (f64::NAN, f64::NAN);
    }
    let med = v[(v.len() - 1) / 2];
    let mut dev: Vec<f64> = v.iter().map(|x| (x - med).abs()).collect();
    dev.sort_by(|a, b| a.total_cmp(b));
    let mad = dev[(dev.len() - 1) / 2];
    (med, mad)
}

fn onset_slot(slots: &[Option<f64>], lo: usize, hi: usize, k: f64) -> Option<usize> {
    let vals: Vec<f64> = (lo..hi).filter_map(|i| slots[i]).collect();
    let (med, mad) = median_mad(&vals);
    if !(mad > 0.0) {
        return None;
    }
    let thr = med + k * ROBUST_SIGMA * mad;
    (lo..hi).find(|&i| slots[i].is_some_and(|v| v >= thr))
}

fn band_strength(profile: &[f64], run: usize) -> Option<f64> {
    let n = profile.len();
    if run == 0 || run > n {
        return None;
    }
    let win = (n / 4).max(run);
    let half = win / 2;
    let mut pref = vec![0.0f64; n + 1];
    for i in 0..n {
        pref[i + 1] = pref[i] + profile[i];
    }
    let mut resid = vec![0.0f64; n];
    for i in 0..n {
        let lo = i.saturating_sub(half);
        let hi = (i + half + 1).min(n);
        let base = (pref[hi] - pref[lo]) / (hi - lo) as f64;
        resid[i] = profile[i] - base;
    }
    let mut acc: f64 = resid[..run].iter().sum();
    let mut best = acc / run as f64;
    for j in run..n {
        acc += resid[j] - resid[j - run];
        best = best.max(acc / run as f64);
    }
    let level = profile.iter().sum::<f64>() / n as f64;
    if !(level.is_finite() && level > 0.0) {
        return None;
    }
    (best / level).is_finite().then_some(best / level)
}

fn band_onset_slot(
    profiles: &[Option<Vec<f64>>],
    lo: usize,
    hi: usize,
    k: f64,
    run: usize,
) -> Option<usize> {
    let mut strength: Vec<Option<f64>> = vec![None; profiles.len()];
    for i in lo..hi {
        if let Some(p) = &profiles[i] {
            strength[i] = band_strength(p, run);
        }
    }
    let vals: Vec<f64> = (lo..hi).filter_map(|i| strength[i]).collect();
    let (med, mad) = median_mad(&vals);
    if !(mad > 0.0) {
        return None;
    }
    let thr = med + k * ROBUST_SIGMA * mad;
    (lo..hi).find(|&i| strength[i].is_some_and(|v| v >= thr))
}

fn mean_profiles(frames: &[(f64, Vec<f64>)], day_start: f64) -> Vec<Option<Vec<f64>>> {
    let mut acc: Vec<Option<Vec<f64>>> = vec![None; MINUTE_SLOTS];
    let mut counts = vec![0u32; MINUTE_SLOTS];
    for (epoch, rows) in frames {
        if let Some(i) = minute_slot(*epoch, day_start) {
            match &mut acc[i] {
                Some(a) if a.len() == rows.len() => {
                    for (x, r) in a.iter_mut().zip(rows.iter()) {
                        *x += *r;
                    }
                    counts[i] += 1;
                }
                _ => {
                    acc[i] = Some(rows.clone());
                    counts[i] = 1;
                }
            }
        }
    }
    for i in 0..MINUTE_SLOTS {
        if let Some(a) = &mut acc[i] {
            let c = counts[i] as f64;
            for x in a.iter_mut() {
                *x /= c;
            }
        }
    }
    acc
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let date = match arg_value(&args, "--date") {
        Some(d) => d,
        None => {
            eprintln!("mag_optical_overlay needs --date <YYYY-MM-DD>");
            std::process::exit(2);
        }
    };
    let day_start = match iso_to_unix(&format!("{date}T00:00:00Z")) {
        Some(t) => t,
        None => {
            eprintln!("mag_optical_overlay: --date {date} is not an ISO day");
            std::process::exit(2);
        }
    };
    let stations_spec = match arg_value(&args, "--stations") {
        Some(v) => v,
        None => {
            eprintln!(
                "mag_optical_overlay needs --stations <code,...> — the station set is named, never defaulted"
            );
            std::process::exit(2);
        }
    };
    let stations: Vec<String> = stations_spec
        .split(',')
        .map(|s| s.trim().to_lowercase())
        .filter(|s| !s.is_empty())
        .collect();
    let optical_path = arg_value(&args, "--optical");
    let out_path = arg_value(&args, "--out");
    let onset_mode = match arg_value(&args, "--onset") {
        Some(mode) => mode,
        None => String::from("mean"),
    };
    if onset_mode != "mean" && onset_mode != "band" {
        eprintln!(
            "mag_optical_overlay: --onset {onset_mode} is neither mean (the per-minute column mean) nor band (the trend-removed, brightness-normalized latitudinal band)"
        );
        std::process::exit(2);
    }
    let band_run = match arg_value(&args, "--band-run")
        .and_then(|s| s.parse::<usize>().ok())
        .filter(|v| *v > 0)
    {
        Some(run) => run,
        None => 16,
    };
    let optical_onset_arg = match arg_value(&args, "--optical-onset") {
        Some(s) => match iso_to_unix(&s) {
            Some(t) => Some(t),
            None => {
                eprintln!("mag_optical_overlay: --optical-onset {s} is not an ISO UTC stamp");
                std::process::exit(2);
            }
        },
        None => None,
    };
    let optical_cube_path = arg_value(&args, "--optical-cube");
    let cube_window = arg_value(&args, "--cube-window")
        .and_then(|s| s.parse::<usize>().ok())
        .filter(|v| *v >= 5)
        .unwrap_or(120);
    let cube_k = arg_value(&args, "--cube-k")
        .and_then(|s| s.parse::<f64>().ok())
        .filter(|v| *v > 0.0)
        .unwrap_or(8.0);
    let cube_persist = arg_value(&args, "--cube-persist")
        .and_then(|s| s.parse::<usize>().ok())
        .filter(|v| *v > 0)
        .unwrap_or(3);

    let mut mag: Vec<(String, Vec<Option<f64>>)> = Vec::new();
    for st in &stations {
        match fetch_day(st, &date) {
            Ok(rows) => {
                let mut slots = vec![None; MINUTE_SLOTS];
                for (t, v, _, _) in &rows {
                    if let Some(i) = minute_slot(*t, day_start) {
                        slots[i] = Some(*v);
                    }
                }
                let peak = rows
                    .iter()
                    .max_by(|a, b| a.1.total_cmp(&b.1))
                    .map(|(t, v, _, _)| (iso_utc(*t), *v));
                if let Some((pt, pv)) = peak {
                    println!(
                        "# {st}: {} samples, peak |dH/dt| {pv:.2} nT/min at {pt}",
                        rows.len()
                    );
                }
                mag.push((st.clone(), slots));
            }
            Err(e) => {
                eprintln!("mag_optical_overlay: {st}: {e}");
                std::process::exit(1);
            }
        }
    }

    let (optical, optical_profiles): (Option<Vec<Option<f64>>>, Option<Vec<Option<Vec<f64>>>>) =
        match &optical_path {
            Some(p) => match read_tkdg(p) {
                Ok(frames) => {
                    let mut slots: Vec<Option<f64>> = vec![None; MINUTE_SLOTS];
                    let mut counts = vec![0u32; MINUTE_SLOTS];
                    for (epoch, bright) in &frames {
                        if let Some(i) = minute_slot(*epoch, day_start) {
                            let s = match slots[i] {
                                Some(v) => v + *bright,
                                None => *bright,
                            };
                            slots[i] = Some(s);
                            counts[i] += 1;
                        }
                    }
                    for i in 0..MINUTE_SLOTS {
                        if let Some(s) = slots[i] {
                            slots[i] = Some(s / counts[i] as f64);
                        }
                    }
                    let peak = frames
                        .iter()
                        .max_by(|a, b| a.1.total_cmp(&b.1))
                        .map(|(t, v)| (iso_utc(*t), *v));
                    if let Some((pt, pv)) = peak {
                        println!(
                            "# optical: {} frames, peak brightness {pv:.1} at {pt}",
                            frames.len()
                        );
                    }
                    let profiles = if onset_mode == "band" {
                        match read_tkdg_profiles(p) {
                            Ok(rows) => Some(mean_profiles(&rows, day_start)),
                            Err(e) => {
                                eprintln!("mag_optical_overlay: {e}");
                                std::process::exit(1);
                            }
                        }
                    } else {
                        None
                    };
                    (Some(slots), profiles)
                }
                Err(e) => {
                    eprintln!("mag_optical_overlay: {e}");
                    std::process::exit(1);
                }
            },
            None => (None, None),
        };

    println!("# orientation: {ORIENTATION}");
    let mut header = String::from("hour");
    for (st, _) in &mag {
        header.push_str(&format!(" {st}_mean {st}_max"));
    }
    if optical.is_some() {
        header.push_str(" opt_mean opt_max opt_min");
    }
    println!("# {header}");
    for h in 0..24usize {
        let lo = h * 60;
        let hi = lo + 60;
        let mut line = format!("{h:02}");
        for (_, slots) in &mag {
            let vals: Vec<f64> = (lo..hi).filter_map(|i| slots[i]).collect();
            let mean = if vals.is_empty() {
                f64::NAN
            } else {
                vals.iter().sum::<f64>() / vals.len() as f64
            };
            let max = vals.iter().copied().fold(f64::NAN, f64::max);
            line.push_str(&format!(" {mean:8.2} {max:8.2}"));
        }
        if let Some(slots) = &optical {
            let vals: Vec<f64> = (lo..hi).filter_map(|i| slots[i]).collect();
            let mean = if vals.is_empty() {
                f64::NAN
            } else {
                vals.iter().sum::<f64>() / vals.len() as f64
            };
            let max = vals.iter().copied().fold(f64::NAN, f64::max);
            let min = vals.iter().copied().fold(f64::NAN, f64::min);
            line.push_str(&format!(" {mean:8.1} {max:8.1} {min:8.1}"));
        }
        println!("{line}");
    }

    if let Some(win) = arg_value(&args, "--window") {
        let k = arg_value(&args, "--k")
            .and_then(|s| s.parse::<f64>().ok())
            .filter(|v| *v > 0.0)
            .unwrap_or(6.0);
        let (ws, we) = match win.split_once(',') {
            Some((a, b)) => (
                iso_to_unix(a).unwrap_or(day_start),
                iso_to_unix(b).unwrap_or(day_start + DAY),
            ),
            None => (day_start, day_start + DAY),
        };
        let slot_of = |t: f64| {
            if t <= day_start {
                0usize
            } else if t >= day_start + DAY {
                MINUTE_SLOTS
            } else {
                ((t - day_start) / 60.0) as usize
            }
        };
        let (lo, hi) = (slot_of(ws), slot_of(we));
        println!(
            "# onset window {}..{} k {k}",
            iso_utc(day_start + lo as f64 * 60.0),
            iso_utc(day_start + hi as f64 * 60.0)
        );
        println!(
            "# onset optical mode {onset_mode}{}",
            if onset_mode == "band" {
                format!(" run {band_run}")
            } else {
                String::new()
            }
        );
        let (optical_onset, optical_onset_note) = if let Some(t) = optical_onset_arg {
            (
                Some(slot_of(t)),
                " (--optical-onset, the 2D full-frame detector)",
            )
        } else if let Some(cube) = &optical_cube_path {
            let onset = match read_fodg(cube) {
                Ok(frames) => cube_onset(&frames, cube_window, cube_k, cube_persist, ws),
                Err(e) => {
                    eprintln!("mag_optical_overlay: {e}");
                    std::process::exit(1);
                }
            };
            (
                onset.map(slot_of),
                " (--optical-cube, the FODG full-frame cube)",
            )
        } else {
            (
                match (&optical, &optical_profiles, onset_mode.as_str()) {
                    (_, Some(profs), "band") => band_onset_slot(profs, lo, hi, k, band_run),
                    (Some(s), _, _) => onset_slot(s, lo, hi, k),
                    _ => None,
                },
                "",
            )
        };
        match optical_onset {
            Some(i) => {
                println!(
                    "onset optical {}{optical_onset_note}",
                    iso_utc(day_start + i as f64 * 60.0)
                )
            }
            None => println!("onset optical pending (no crossing above the robust threshold)"),
        }
        for (st, slots) in &mag {
            match onset_slot(slots, lo, hi, k) {
                Some(i) => {
                    let lag = match optical_onset {
                        Some(o) => (i as i64 - o as i64) * 60,
                        None => 0,
                    };
                    println!(
                        "onset {st} {} lag_s {lag}",
                        iso_utc(day_start + i as f64 * 60.0)
                    );
                }
                None => {
                    let peak = (lo..hi).filter_map(|i| slots[i]).fold(f64::NAN, f64::max);
                    println!(
                        "onset {st} riss (no crossing above the robust threshold; window peak |dH/dt| {peak:.2} nT/min)"
                    );
                }
            }
        }
    }

    if let Some(out) = out_path {
        let mut csv = String::from("iso");
        for (st, _) in &mag {
            csv.push_str(&format!(",{st}_dhdt_nt_min"));
        }
        csv.push_str(",optical_brightness\n");
        for i in 0..MINUTE_SLOTS {
            let epoch = day_start + i as f64 * 60.0;
            csv.push_str(&iso_utc(epoch));
            for (_, slots) in &mag {
                match slots[i] {
                    Some(v) => csv.push_str(&format!(",{v:.4}")),
                    None => csv.push(','),
                }
            }
            match optical.as_ref().and_then(|s| s[i]) {
                Some(v) => csv.push_str(&format!(",{v:.4}\n")),
                None => csv.push_str(",\n"),
            }
        }
        if let Err(e) = std::fs::write(&out, &csv) {
            eprintln!("mag_optical_overlay: {out} write void: {e}");
            std::process::exit(1);
        }
        println!("wrote per-minute overlay -> {out}");
    }

    if let Some(svg_path) = arg_value(&args, "--svg") {
        let svg = render_svg(day_start, &mag, optical.as_ref());
        if let Err(e) = std::fs::write(&svg_path, &svg) {
            eprintln!("mag_optical_overlay: {svg_path} write void: {e}");
            std::process::exit(1);
        }
        println!("wrote overlay svg -> {svg_path}");
    }
}

fn segments(slots: &[Option<f64>]) -> Vec<Vec<(usize, f64)>> {
    let mut out: Vec<Vec<(usize, f64)>> = Vec::new();
    let mut cur: Vec<(usize, f64)> = Vec::new();
    for (i, slot) in slots.iter().enumerate() {
        match slot {
            Some(v) => cur.push((i, *v)),
            None => {
                if !cur.is_empty() {
                    out.push(std::mem::take(&mut cur));
                }
            }
        }
    }
    if !cur.is_empty() {
        out.push(cur);
    }
    out
}

fn render_svg(
    day_start: f64,
    mag: &[(String, Vec<Option<f64>>)],
    optical: Option<&Vec<Option<f64>>>,
) -> String {
    let plot_left = 70.0;
    let plot_right = 1360.0;
    let plot_w = plot_right - plot_left;
    let panel_a_top = 40.0;
    let panel_a_h = 240.0;
    let panel_b_top = 330.0;
    let panel_b_h = 240.0;
    let svg_h = 610;
    let colors = ["#c0392b", "#e67e22", "#7f8c8d"];
    let x = |min: usize| plot_left + (min as f64 / MINUTE_SLOTS as f64) * plot_w;
    let logp = |v: f64| {
        let lo = -1.0;
        let hi = 3.0;
        let c = v.max(0.1).min(1000.0).log10();
        (c - lo) / (hi - lo)
    };
    let mut s = String::new();
    s.push_str(&format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"1400\" height=\"{svg_h}\" font-family=\"sans-serif\">\n"
    ));
    s.push_str("<rect width=\"1400\" height=\"610\" fill=\"white\"/>\n");
    s.push_str(&format!(
        "<text x=\"70\" y=\"20\" font-size=\"14\">INTERMAGNET |dH/dt| (nT/min, log) vs THEMIS-ASI kapu meridian brightness — {}</text>\n",
        iso_utc(day_start)
    ));
    s.push_str(&format!(
        "<text x=\"70\" y=\"{}\" font-size=\"11\">hour (UT)</text>\n",
        panel_b_top + panel_b_h + 28.0
    ));
    for h in 0..=24usize {
        let xx = x(h * 60);
        s.push_str(&format!(
            "<line x1=\"{xx:.1}\" y1=\"{panel_a_top}\" x2=\"{xx:.1}\" y2=\"{:.1}\" stroke=\"#eee\"/>\n",
            panel_b_top + panel_b_h
        ));
        if h % 2 == 0 {
            s.push_str(&format!(
                "<text x=\"{xx:.1}\" y=\"{:.1}\" font-size=\"10\" text-anchor=\"middle\">{:02}</text>\n",
                panel_b_top + panel_b_h + 16.0,
                h
            ));
        }
    }
    for d in 0..4 {
        for panel in [(panel_a_top, panel_a_h), (panel_b_top, panel_b_h)] {
            let yy = panel.0 + panel.1 * (1.0 - d as f64 / 3.0);
            s.push_str(&format!(
                "<line x1=\"{plot_left}\" y1=\"{yy:.1}\" x2=\"{plot_right}\" y2=\"{yy:.1}\" stroke=\"#eee\"/>\n"
            ));
        }
    }
    for (i, (_, slots)) in mag.iter().enumerate() {
        let color = colors[i % colors.len()];
        for seg in segments(slots) {
            let pts: Vec<String> = seg
                .iter()
                .map(|(m, v)| {
                    format!(
                        "{:.1},{:.1}",
                        x(*m),
                        panel_a_top + (1.0 - logp(*v)) * panel_a_h
                    )
                })
                .collect();
            s.push_str(&format!(
                "<polyline fill=\"none\" stroke=\"{color}\" stroke-width=\"1\" points=\"{}\"/>\n",
                pts.join(" ")
            ));
        }
    }
    if let Some(slots) = optical {
        let vals: Vec<f64> = slots.iter().filter_map(|s| *s).collect();
        let vmin = vals.iter().copied().fold(f64::INFINITY, f64::min);
        let vmax = vals.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        let span = (vmax - vmin).max(1.0);
        for seg in segments(slots) {
            let pts: Vec<String> = seg
                .iter()
                .map(|(m, v)| {
                    let frac = ((*v - vmin) / span).clamp(0.0, 1.0);
                    format!("{:.1},{:.1}", x(*m), panel_b_top + (1.0 - frac) * panel_b_h)
                })
                .collect();
            s.push_str(&format!(
                "<polyline fill=\"none\" stroke=\"#2471a3\" stroke-width=\"1.5\" points=\"{}\"/>\n",
                pts.join(" ")
            ));
        }
        s.push_str(&format!(
            "<text x=\"{plot_left}\" y=\"{:.1}\" font-size=\"10\" fill=\"#2471a3\">optical min {vmin:.0} / max {vmax:.0}</text>\n",
            panel_b_top - 6.0
        ));
    }
    for (label, hh) in [("07:41", 7.0 + 41.0 / 60.0), ("07:55", 7.0 + 55.0 / 60.0)] {
        let xx = x((hh * 60.0) as usize);
        s.push_str(&format!(
            "<line x1=\"{xx:.1}\" y1=\"{panel_a_top}\" x2=\"{xx:.1}\" y2=\"{:.1}\" stroke=\"#8e44ad\" stroke-dasharray=\"4,3\"/>\n",
            panel_b_top + panel_b_h
        ));
        s.push_str(&format!(
            "<text x=\"{xx:.1}\" y=\"{:.1}\" font-size=\"10\" fill=\"#8e44ad\" text-anchor=\"middle\">{label}</text>\n",
            panel_a_top - 6.0
        ));
    }
    let mut lx = plot_left + 400.0;
    for (i, (st, _)) in mag.iter().enumerate() {
        let color = colors[i % colors.len()];
        s.push_str(&format!(
            "<text x=\"{lx:.0}\" y=\"20\" font-size=\"12\" fill=\"{color}\">{st}</text>\n"
        ));
        lx += 70.0;
    }
    s.push_str("</svg>\n");
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_short_hapi_stamp_yields_unix_utc() {
        let t = iso_to_unix("2024-05-10T07:41Z").expect("stamp");
        assert_eq!(iso_utc(t), "2024-05-10T07:41:00Z");
    }

    #[test]
    fn the_minute_slot_is_the_utc_minute_of_the_day() {
        let d = iso_to_unix("2024-05-10T00:00:00Z").unwrap();
        let t = iso_to_unix("2024-05-10T07:41Z").unwrap();
        assert_eq!(minute_slot(t, d), Some(461));
        assert_eq!(minute_slot(d - 60.0, d), None);
    }

    #[test]
    fn the_tkdg_reader_averages_the_256_row_profile() {
        let dir = std::env::temp_dir().join("mag_optical_overlay_test");
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join("probe.bin");
        let mut bytes = Vec::new();
        bytes.extend_from_slice(b"TKDG");
        bytes.extend_from_slice(&1u32.to_le_bytes());
        bytes.extend_from_slice(&1715320800.0f64.to_le_bytes());
        for _ in 0..256 {
            bytes.extend_from_slice(&1400u16.to_le_bytes());
        }
        std::fs::write(&path, &bytes).unwrap();
        let frames = read_tkdg(path.to_str().unwrap()).expect("read");
        assert_eq!(frames.len(), 1);
        assert!((frames[0].1 - 1400.0).abs() < 1e-6);
    }

    fn flat_profile(level: f64) -> Vec<f64> {
        vec![level; 256]
    }

    #[test]
    fn band_strength_is_zero_for_a_uniform_profile_and_positive_for_a_band() {
        assert_eq!(band_strength(&flat_profile(1000.0), 16), Some(0.0));
        let mut banded = flat_profile(1000.0);
        for r in 100..116 {
            banded[r] += 800.0;
        }
        assert!(band_strength(&banded, 16).unwrap() > 0.4);
    }

    #[test]
    fn band_onset_ignores_a_pure_common_mode_rise() {
        let mut profiles: Vec<Option<Vec<f64>>> = vec![None; 12];
        for (i, p) in profiles.iter_mut().enumerate() {
            *p = Some(flat_profile(1000.0 + i as f64 * 50.0));
        }
        assert_eq!(band_onset_slot(&profiles, 0, 12, 6.0, 16), None);
    }

    #[test]
    fn band_onset_fires_on_a_localized_latitudinal_band() {
        let mut profiles: Vec<Option<Vec<f64>>> = vec![None; 12];
        for (i, p) in profiles.iter_mut().enumerate() {
            let mut rows = flat_profile(1000.0 + i as f64 * 50.0);
            if i < 7 {
                if i % 2 == 0 {
                    for r in 100..116 {
                        rows[r] += 1.0;
                    }
                }
            } else {
                for r in 100..116 {
                    rows[r] += 800.0;
                }
            }
            *p = Some(rows);
        }
        assert_eq!(band_onset_slot(&profiles, 0, 12, 6.0, 16), Some(7));
    }

    #[test]
    fn the_tkdg_profile_reader_carries_the_256_rows() {
        let dir = std::env::temp_dir().join("mag_optical_overlay_profile_test");
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join("probe.bin");
        let mut bytes = Vec::new();
        bytes.extend_from_slice(b"TKDG");
        bytes.extend_from_slice(&1u32.to_le_bytes());
        bytes.extend_from_slice(&1715320800.0f64.to_le_bytes());
        for r in 0..256u16 {
            bytes.extend_from_slice(&(1000 + r).to_le_bytes());
        }
        std::fs::write(&path, &bytes).unwrap();
        let frames = read_tkdg_profiles(path.to_str().unwrap()).expect("read");
        assert_eq!(frames.len(), 1);
        assert_eq!(frames[0].1.len(), 256);
        assert!((frames[0].1[255] - 1255.0).abs() < 1e-9);
    }

    #[test]
    fn the_fodg_cube_reader_derives_the_earliest_onset() {
        let mut frames: Vec<(f64, Vec<f32>)> = Vec::new();
        for i in 0..10usize {
            let v = if i >= 6 { 500.0f32 } else { 10.0 };
            frames.push((1000.0 + i as f64 * 60.0, vec![10.0, v]));
        }
        let mut bytes = Vec::new();
        bytes.extend_from_slice(b"FODG");
        bytes.extend_from_slice(&2u32.to_le_bytes());
        bytes.extend_from_slice(&1u32.to_le_bytes());
        bytes.extend_from_slice(&(frames.len() as u32).to_le_bytes());
        for (e, cells) in &frames {
            bytes.extend_from_slice(&e.to_le_bytes());
            for c in cells {
                bytes.extend_from_slice(&c.to_le_bytes());
            }
        }
        let dir = std::env::temp_dir().join("mag_optical_overlay_fodg_test");
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join("probe.bin");
        std::fs::write(&path, &bytes).unwrap();
        let read = read_fodg(path.to_str().unwrap()).expect("read");
        assert_eq!(read.len(), 10);
        let onset = cube_onset(&read, 5, 8.0, 3, f64::NEG_INFINITY).expect("onset");
        assert!((onset - (1000.0 + 6.0 * 60.0)).abs() < 1e-6);
    }
}
