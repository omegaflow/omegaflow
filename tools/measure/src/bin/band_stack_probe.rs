
use omegaflow::archivar::wdc_ae;
use omegaflow::archivar::{JsonVal, jpath, parse_json};
use std::collections::BTreeMap;
use std::path::Path;

const DAY: f64 = 86400.0;
const MIN: f64 = 60.0;

struct Station {
    code: &'static str,
    mlat: f64,
    mlon: f64,
}

const STATIONS: &[Station] = &[
    Station {
        code: "ABK",
        mlat: 65.180603,
        mlon: 101.815880,
    },
    Station {
        code: "KIL",
        mlat: 65.827354,
        mlon: 103.869637,
    },
    Station {
        code: "MUO",
        mlat: 64.612961,
        mlon: 105.258545,
    },
    Station {
        code: "SOD",
        mlat: 63.814796,
        mlon: 107.288300,
    },
    Station {
        code: "PEL",
        mlat: 63.417301,
        mlon: 104.951561,
    },
    Station {
        code: "IVA",
        mlat: 65.028931,
        mlon: 108.609955,
    },
    Station {
        code: "ALT",
        mlat: 66.556702,
        mlon: 106.203331,
    },
    Station {
        code: "NOR",
        mlat: 67.697388,
        mlon: 109.434196,
    },
    Station {
        code: "YKC",
        mlat: 69.530128,
        mlon: -59.640087,
    },
    Station {
        code: "C06",
        mlat: 60.913975,
        mlon: -53.473446,
    },
    Station {
        code: "C09",
        mlat: 65.334389,
        mlon: -29.075134,
    },
    Station {
        code: "GIM",
        mlat: 66.777817,
        mlon: -28.330137,
    },
    Station {
        code: "MEA",
        mlat: 62.098042,
        mlon: -54.395199,
    },
    Station {
        code: "CMO",
        mlat: 65.294014,
        mlon: -96.026627,
    },
    Station {
        code: "BET",
        mlat: 66.685280,
        mlon: -100.637184,
    },
    Station {
        code: "KAV",
        mlat: 71.224609,
        mlon: -97.334442,
    },
    Station {
        code: "SKT",
        mlat: 71.894539,
        mlon: 37.628761,
    },
    Station {
        code: "STF",
        mlat: 73.064102,
        mlon: 41.463329,
    },
    Station {
        code: "TIK",
        mlat: 66.054489,
        mlon: -162.723358,
    },
    Station {
        code: "CHD",
        mlat: 65.132568,
        mlon: -147.385544,
    },
];

struct Event {
    label: String,
    onset: f64,
}

struct Series {
    t: Vec<f64>,
    dbdt: Vec<f64>,
}

fn arg_value(args: &[String], key: &str) -> Option<String> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn iso_to_unix(s: &str) -> Option<f64> {
    let s = s.trim();
    let (date, time) = s.split_once('T').or_else(|| s.split_once(' '))?;
    let mut dp = date.split('-');
    let y: i64 = dp.next()?.parse().ok()?;
    let m: i64 = dp.next()?.parse().ok()?;
    let d: i64 = dp.next()?.parse().ok()?;
    let t = time
        .split(|c: char| c == '.' || c == 'Z' || c == 'z')
        .next()?;
    let mut tp = t.split(':');
    let hh: i64 = tp.next()?.parse().ok()?;
    let mm: i64 = tp.next().unwrap_or("0").parse().ok()?;
    let ss: i64 = tp.next().unwrap_or("0").parse().ok()?;
    let a = (14 - m) / 12;
    let yy = y + 4800 - a;
    let jdn =
        d + (153 * (m + 12 * a - 3) + 2) / 5 + 365 * yy + yy / 4 - yy / 100 + yy / 400 - 32045;
    Some((jdn - 2440588) as f64 * DAY + hh as f64 * 3600.0 + mm as f64 * 60.0 + ss as f64)
}

fn civil_from_days(z: i64) -> (i64, i64, i64) {
    let z = z + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    (if m <= 2 { y + 1 } else { y }, m, d)
}

fn unix_to_iso(t: f64) -> String {
    let days = (t / DAY).floor() as i64;
    let secs = t - days as f64 * DAY;
    let (y, m, d) = civil_from_days(days);
    let hh = (secs / 3600.0) as i64;
    let mm = ((secs - hh as f64 * 3600.0) / 60.0) as i64;
    format!("{y:04}-{m:02}-{d:02}T{hh:02}:{mm:02}Z")
}

fn frac(num: usize, den: usize) -> Option<f64> {
    if den == 0 {
        None
    } else {
        Some(num as f64 / den as f64)
    }
}

fn mean(total: f64, den: usize) -> Option<f64> {
    if den == 0 {
        None
    } else {
        Some(total / den as f64)
    }
}

fn show(v: Option<f64>, prec: usize) -> String {
    match v {
        Some(x) if x.is_finite() => format!("{x:.prec$}"),
        _ => "n/a".to_string(),
    }
}

fn median(values: &mut [f64]) -> Option<f64> {
    if values.is_empty() {
        return None;
    }
    values.sort_by(|a, b| a.total_cmp(b));
    let n = values.len();
    Some(if n % 2 == 1 {
        values[n / 2]
    } else {
        0.5 * (values[n / 2 - 1] + values[n / 2])
    })
}

fn robust_sigma(values: &[f64]) -> Option<f64> {
    if values.len() < 10 {
        return None;
    }
    let mut med = values.to_vec();
    let m = median(&mut med)?;
    let mut dev: Vec<f64> = values.iter().map(|v| (v - m).abs()).collect();
    let mad = median(&mut dev)?;
    let s = 1.4826 * mad;
    if s.is_finite() && s > 0.0 {
        Some(s)
    } else {
        None
    }
}

fn read_events(path: &str) -> Option<Vec<Event>> {
    let text = std::fs::read_to_string(path).ok()?;
    let mut out = Vec::new();
    for line in text.lines() {
        let label = line.trim();
        if label.is_empty() || label.starts_with('#') {
            continue;
        }
        let onset = iso_to_unix(label)?;
        out.push(Event {
            label: label.to_string(),
            onset,
        });
    }
    if out.is_empty() { None } else { Some(out) }
}

fn load_series(path: &Path) -> Option<Series> {
    let text = std::fs::read_to_string(path).ok()?;
    let Some(JsonVal::Arr(records)) = parse_json(&text) else {
        return None;
    };
    let mut rows: Vec<(f64, f64, f64)> = Vec::new();
    for rec in &records {
        let (Some(t), Some(n), Some(e)) =
            (jpath(rec, "tval"), jpath(rec, "N.nez"), jpath(rec, "E.nez"))
        else {
            continue;
        };
        if t.is_finite() && n.is_finite() && e.is_finite() {
            rows.push((t, n, e));
        }
    }
    if rows.len() < 30 {
        return None;
    }
    rows.sort_by(|a, b| a.0.total_cmp(&b.0));
    let mut t = Vec::with_capacity(rows.len());
    let mut dbdt = Vec::with_capacity(rows.len());
    for w in rows.windows(2) {
        let dt = w[1].0 - w[0].0;
        if !(dt > 0.0 && dt.is_finite()) || dt > 180.0 {
            continue;
        }
        let dn = w[1].1 - w[0].1;
        let de = w[1].2 - w[0].2;
        t.push(w[1].0);
        dbdt.push((dn * dn + de * de).sqrt() / (dt / MIN));
    }
    if t.len() < 20 {
        None
    } else {
        Some(Series { t, dbdt })
    }
}

fn mlt_at(mlon: f64, unix: f64) -> f64 {
    let ut = (unix.rem_euclid(DAY)) / 3600.0;
    (mlon / 15.0 + ut).rem_euclid(24.0)
}

fn oval(mlt: f64, al: f64) -> (f64, f64) {
    let a = al.abs().min(2000.0);
    let phase = 2.0 * std::f64::consts::PI * (mlt - 12.0) / 24.0;
    let center = 67.0 + 5.0 * phase.cos() - 0.006 * a;
    let half = 4.0 + 0.004 * a;
    (center, half)
}

fn in_band(mlat: f64, mlon: f64, t: f64, al: f64) -> bool {
    let (c, h) = oval(mlt_at(mlon, t), al);
    (mlat - c).abs() <= h
}

struct Wdc {
    ae: BTreeMap<i64, f64>,
    al: BTreeMap<i64, f64>,
    dst: BTreeMap<i64, f64>,
}

fn load_wdc(path: &str) -> Option<Wdc> {
    let bytes = std::fs::read(path).ok()?;
    let recs = wdc_ae::parse_bin(&bytes)?;
    let mut ae = BTreeMap::new();
    let mut al = BTreeMap::new();
    let mut dst = BTreeMap::new();
    for r in recs {
        let k = r.t_unix.round() as i64;
        if let Some(v) = r.ae_nt {
            ae.insert(k, v);
        }
        if let Some(v) = r.al_nt {
            al.insert(k, v);
        }
        if let Some(v) = r.dst_nt {
            dst.insert(k, v);
        }
    }
    eprintln!("WDC: AE={} AL={} Dst={}", ae.len(), al.len(), dst.len());
    if ae.is_empty() && al.is_empty() {
        None
    } else {
        Some(Wdc { ae, al, dst })
    }
}

fn nearest(map: &BTreeMap<i64, f64>, t: f64) -> Option<f64> {
    let k = t.round() as i64;
    if let Some(v) = map.get(&k) {
        return Some(*v);
    }
    let mut best: Option<(i64, f64)> = None;
    for (&kt, &v) in map.range((k - 1800)..=(k + 1800)) {
        let dist = (kt - k).abs();
        if best.map(|(bd, _)| dist < bd).unwrap_or(true) {
            best = Some((dist, v));
        }
    }
    best.map(|(_, v)| v)
}

fn al_at(wdc: &Wdc, t: f64) -> Option<f64> {
    nearest(&wdc.al, t).or_else(|| nearest(&wdc.ae, t).map(|ae| -0.5 * ae.abs()))
}

fn al_min_time(wdc: &Wdc, start: f64, end: f64) -> Option<f64> {
    let mut best: Option<(f64, f64)> = None;
    let mut t = start;
    while t <= end {
        if let Some(al) = nearest(&wdc.al, t) {
            if best.map(|(bv, _)| al < bv).unwrap_or(true) {
                best = Some((al, t));
            }
        }
        t += 60.0;
    }
    best.map(|(_, t)| t)
}

fn front_time(t_al: f64, mlon: f64, t: f64) -> f64 {
    let mlt = mlt_at(mlon, t);
    let dist = mlt.min(24.0 - mlt).min(12.0);
    t_al + 0.25 * dist * 3600.0
}

struct Stack {
    peak: f64,
    noise: f64,
    snr: f64,
    n_bins: usize,
    n_samples: usize,
    stations_used: usize,
    in_band: f64,
}

fn cached_series<'a>(
    cache: &'a mut BTreeMap<String, Option<Series>>,
    path: &Path,
) -> Option<&'a Series> {
    let key = path.to_string_lossy().into_owned();
    cache
        .entry(key)
        .or_insert_with(|| load_series(path))
        .as_ref()
}

fn stack_event(
    cache: &mut BTreeMap<String, Option<Series>>,
    raw: &Path,
    event: &Event,
    wdc: &Wdc,
    stations: &[&Station],
    mask: bool,
    align: bool,
) -> Option<Stack> {
    let t_end = event.onset + 4.0 * 3600.0;
    let t_start = event.onset - 2.0 * 3600.0;
    let Some(t_al) = al_min_time(wdc, event.onset - 3600.0, event.onset + 7200.0) else {
        return None;
    };
    let mut bins: BTreeMap<i32, (f64, usize)> = BTreeMap::new();
    let mut in_band_sum = 0.0;
    let mut in_band_n = 0usize;
    let mut stations_used = 0usize;
    for st in stations {
        let path = raw.join(format!("{}_{}.json", event.label, st.code));
        let Some(series) = cached_series(cache, &path) else {
            continue;
        };
        stations_used += 1;
        for (i, &t) in series.t.iter().enumerate() {
            if t < t_start || t > t_end {
                continue;
            }
            let Some(al) = al_at(wdc, t) else { continue };
            let band = in_band(st.mlat, st.mlon, t, al);
            if mask {
                in_band_n += 1;
                if band {
                    in_band_sum += 1.0;
                }
                if !band {
                    continue;
                }
            }
            let tref = if align {
                front_time(t_al, st.mlon, t)
            } else {
                event.onset
            };
            let rel = ((t - tref) / MIN).round() as i32;
            if rel.abs() > 90 {
                continue;
            }
            let e = bins.entry(rel).or_insert((0.0, 0));
            e.0 += series.dbdt[i];
            e.1 += 1;
        }
    }
    if stations_used == 0 {
        return None;
    }
    let mut peak = 0.0f64;
    let mut noise_vals = Vec::new();
    let mut n_bins = 0usize;
    let mut n_samples = 0usize;
    for (&rel, &(sum, count)) in &bins {
        if count < 2 {
            continue;
        }
        let v = sum / count as f64;
        n_bins += 1;
        n_samples += count;
        if rel.abs() <= 15 {
            peak = peak.max(v);
        }
        if rel.abs() >= 25 && rel.abs() <= 75 {
            noise_vals.push(v);
        }
    }
    let noise = robust_sigma(&noise_vals)?;
    if n_bins < 10 {
        return None;
    }
    Some(Stack {
        peak,
        noise,
        snr: peak / noise,
        n_bins,
        n_samples,
        stations_used,
        in_band: if mask && in_band_n > 0 {
            in_band_sum / in_band_n as f64
        } else {
            0.0
        },
    })
}

fn single_event(
    cache: &mut BTreeMap<String, Option<Series>>,
    raw: &Path,
    event: &Event,
    wdc: &Wdc,
    code: &str,
) -> Option<Stack> {
    let st = STATIONS.iter().find(|s| s.code == code)?;
    stack_event(cache, raw, event, wdc, &[st], false, true)
}

fn percentile(values: &mut [f64], p: f64) -> Option<f64> {
    if values.is_empty() {
        return None;
    }
    values.sort_by(|a, b| a.total_cmp(b));
    let idx = ((values.len() - 1) as f64 * p).round() as usize;
    Some(values[idx])
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--help" || a == "-h") {
        eprintln!(
            "band_stack_probe --raw DIR --wdc FILE --events FILE [--quiet-events FILE] [--list-quiet]"
        );
        return;
    }
    let Some(wdc_path) = arg_value(&args, "--wdc") else {
        eprintln!("--wdc FILE required");
        std::process::exit(1);
    };
    let Some(events_path) = arg_value(&args, "--events") else {
        eprintln!("--events FILE required");
        std::process::exit(1);
    };
    let Some(raw_arg) = arg_value(&args, "--raw") else {
        eprintln!("--raw DIR required");
        std::process::exit(1);
    };
    let raw = std::path::PathBuf::from(raw_arg);
    let wdc = match load_wdc(&wdc_path) {
        Some(w) => w,
        None => {
            eprintln!("{wdc_path}: WDC-Bin parses void — AE/AL/Dst absent");
            std::process::exit(1);
        }
    };
    let events = match read_events(&events_path) {
        Some(e) => e,
        None => {
            eprintln!("{events_path}: no events");
            std::process::exit(1);
        }
    };
    let quiet_path = arg_value(&args, "--quiet-events");

    if args.iter().any(|a| a == "--list-quiet") {
        let mut event_starts: Vec<f64> = events.iter().map(|e| e.onset).collect();
        event_starts.sort_by(|a, b| a.total_cmp(b));
        let mut out = Vec::new();
        let start = iso_to_unix("2014-01-01T00:00:00Z").unwrap();
        let end = iso_to_unix("2023-12-31T00:00:00Z").unwrap();
        let mut day = start;
        while day <= end {
            let mut quiet = true;
            for h in 0..24 {
                let t = day + h as f64 * 3600.0;
                if let Some(d) = nearest(&wdc.dst, t) {
                    if d <= -20.0 {
                        quiet = false;
                        break;
                    }
                } else {
                    quiet = false;
                    break;
                }
                if let Some(al) = nearest(&wdc.al, t) {
                    if al < -50.0 {
                        quiet = false;
                        break;
                    }
                }
                if let Some(ae) = nearest(&wdc.ae, t) {
                    if ae > 200.0 {
                        quiet = false;
                        break;
                    }
                }
            }
            if quiet {
                let near = event_starts.iter().any(|&e| (e - day).abs() < 5.0 * DAY);
                if !near {
                    out.push(day);
                }
            }
            day += DAY;
        }
        let want = 50usize.min(out.len());
        let mut chosen = Vec::new();
        for i in 0..want {
            let idx = if want == 1 {
                0
            } else {
                i * (out.len() - 1) / (want - 1)
            };
            chosen.push(out[idx]);
        }
        if let Some(path) = arg_value(&args, "--quiet-out") {
            let body: Vec<String> = chosen.iter().map(|&t| unix_to_iso(t)).collect();
            let _ = std::fs::write(&path, body.join("\n") + "\n");
        }
        for t in &chosen {
            println!("{}", unix_to_iso(*t));
        }
        eprintln!("{} quiet days selected from {}", chosen.len(), out.len());
        return;
    }

    let mut cache: BTreeMap<String, Option<Series>> = BTreeMap::new();
    let mut event_rows: Vec<(String, f64, Stack)> = Vec::new();
    for e in &events {
        let stations: Vec<&Station> = STATIONS.iter().collect();
        if let Some(s) = stack_event(&mut cache, &raw, e, &wdc, &stations, true, true) {
            event_rows.push((e.label.clone(), e.onset, s));
        }
    }
    let n_events = events.len();
    let n_measured = event_rows.len();
    let snrs: Vec<f64> = event_rows.iter().map(|(_, _, s)| s.snr).collect();
    let detected = snrs.iter().filter(|&&s| s >= 3.0).count();
    let in_band_mean = mean(
        event_rows.iter().map(|(_, _, s)| s.in_band).sum::<f64>(),
        n_measured,
    );
    let band_counts: Vec<f64> = event_rows
        .iter()
        .map(|(_, _, s)| s.in_band * s.stations_used as f64)
        .collect();
    let band_med = median(&mut band_counts.clone());
    let band_min = band_counts.iter().cloned().fold(f64::INFINITY, f64::min);
    let band_max = band_counts
        .iter()
        .cloned()
        .fold(f64::NEG_INFINITY, f64::max);
    let events_ge3 = band_counts.iter().filter(|&&v| v >= 3.0).count();
    let stations_mean = mean(
        event_rows
            .iter()
            .map(|(_, _, s)| s.stations_used as f64)
            .sum::<f64>(),
        n_measured,
    );
    let samples_mean = mean(
        event_rows
            .iter()
            .map(|(_, _, s)| s.n_samples as f64)
            .sum::<f64>(),
        n_measured,
    );
    let bins_mean = mean(
        event_rows
            .iter()
            .map(|(_, _, s)| s.n_bins as f64)
            .sum::<f64>(),
        n_measured,
    );

    let mut single_stats = Vec::new();
    for code in ["ABK", "SOD"] {
        let mut snrs = Vec::new();
        let mut noises = Vec::new();
        let mut det = 0usize;
        let mut n = 0usize;
        for e in &events {
            if let Some(s) = single_event(&mut cache, &raw, e, &wdc, code) {
                n += 1;
                if s.snr >= 3.0 {
                    det += 1;
                }
                snrs.push(s.snr);
                noises.push(s.noise);
            }
        }
        let med = median(&mut snrs);
        let nmed = median(&mut noises);
        single_stats.push((code, n, det, med, snrs, nmed));
    }

    let mut quiet_snrs = Vec::new();
    let mut quiet_noises = Vec::new();
    let mut quiet_peaks = Vec::new();
    let mut quiet_det = 0usize;
    let mut quiet_n = 0usize;
    let mut qevents_vec: Vec<Event> = Vec::new();
    if let Some(qp) = quiet_path {
        if let Some(qevents) = read_events(&qp) {
            for e in &qevents {
                let stations: Vec<&Station> = STATIONS.iter().collect();
                if let Some(s) = stack_event(&mut cache, &raw, e, &wdc, &stations, true, true) {
                    quiet_n += 1;
                    if s.snr >= 3.0 {
                        quiet_det += 1;
                    }
                    quiet_snrs.push(s.snr);
                    quiet_noises.push(s.noise);
                    quiet_peaks.push(s.peak);
                }
            }
            qevents_vec = qevents;
        }
    }
    let mut quiet_draw = Vec::new();
    let mut seed = 0x9e3779b97f4a7c15u64;
    for _ in 0..500 {
        seed = seed
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        if quiet_snrs.is_empty() {
            break;
        }
        let i = (seed >> 33) as usize % quiet_snrs.len();
        quiet_draw.push(quiet_snrs[i]);
    }
    let quiet_q95 = percentile(&mut quiet_draw.clone(), 0.95);
    let quiet_fa = quiet_draw.iter().filter(|&&s| s >= 3.0).count();
    let quiet_noise_med = median(&mut quiet_noises);
    let quiet_peak_med = median(&mut quiet_peaks);
    let mut quiet_peak_draw = Vec::new();
    let mut qseed = 0x9e3779b97f4a7c15u64;
    for _ in 0..500 {
        qseed = qseed
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        if quiet_peaks.is_empty() {
            break;
        }
        let i = (qseed >> 33) as usize % quiet_peaks.len();
        quiet_peak_draw.push(quiet_peaks[i]);
    }
    let quiet_peak_q95 = percentile(&mut quiet_peak_draw, 0.95);
    let det_peak = quiet_peak_q95
        .map(|q| event_rows.iter().filter(|(_, _, s)| s.peak >= q).count());
    let det_both = quiet_peak_q95.zip(quiet_q95).map(|(qp, qs)| {
        event_rows
            .iter()
            .filter(|(_, _, s)| s.peak >= qp && s.snr >= qs)
            .count()
    });
    let stack_q95 = percentile(&mut snrs.clone(), 0.95);
    let det95 = quiet_q95.map(|q| snrs.iter().filter(|&&s| s >= q).count());

    let mut single_q95: BTreeMap<&str, f64> = BTreeMap::new();
    for code in ["ABK", "SOD"] {
        let mut qs = Vec::new();
        for e in &qevents_vec {
            if let Some(s) = single_event(&mut cache, &raw, e, &wdc, code) {
                qs.push(s.snr);
            }
        }
        if let Some(q) = percentile(&mut qs, 0.95) {
            single_q95.insert(code, q);
        }
    }

    let noise_med = median(
        &mut event_rows
            .iter()
            .map(|(_, _, s)| s.noise)
            .collect::<Vec<_>>(),
    )
    ;
    let peak_med = median(
        &mut event_rows
            .iter()
            .map(|(_, _, s)| s.peak)
            .collect::<Vec<_>>(),
    )
    ;

    let mut rand_snr = Vec::new();
    let mut rand_noise = Vec::new();
    if !event_rows.is_empty() {
        for _ in 0..500 {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            let e = &events[(seed >> 33) as usize % events.len()];
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            let k = 3 + ((seed >> 33) as usize % 5);
            let mut chosen: Vec<&Station> = Vec::new();
            let mut used = [false; 20];
            while chosen.len() < k {
                seed = seed
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                let i = (seed >> 33) as usize % STATIONS.len();
                if used[i] {
                    continue;
                }
                used[i] = true;
                chosen.push(&STATIONS[i]);
            }
            if let Some(s) = stack_event(&mut cache, &raw, e, &wdc, &chosen, false, true) {
                rand_snr.push(s.snr);
                rand_noise.push(s.noise);
            }
        }
    }
    let rand_q95 = percentile(&mut rand_snr.clone(), 0.95);
    let rand_med = median(&mut rand_snr.clone());
    let rand_noise_med = median(&mut rand_noise.clone());
    let rand_fa = rand_snr.iter().filter(|&&s| s >= 3.0).count();

    let mut rand_quiet_snr = Vec::new();
    let mut rand_quiet_peak = Vec::new();
    if !qevents_vec.is_empty() {
        for _ in 0..500 {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            let e = &qevents_vec[(seed >> 33) as usize % qevents_vec.len()];
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            let k = 3 + ((seed >> 33) as usize % 5);
            let mut chosen: Vec<&Station> = Vec::new();
            let mut used = [false; 20];
            while chosen.len() < k {
                seed = seed
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                let i = (seed >> 33) as usize % STATIONS.len();
                if used[i] {
                    continue;
                }
                used[i] = true;
                chosen.push(&STATIONS[i]);
            }
            if let Some(s) = stack_event(&mut cache, &raw, e, &wdc, &chosen, false, true) {
                rand_quiet_snr.push(s.snr);
                rand_quiet_peak.push(s.peak);
            }
        }
    }
    let rand_quiet_fa3 = rand_quiet_snr.iter().filter(|&&s| s >= 3.0).count();
    let rand_quiet_peak_q95 = percentile(&mut rand_quiet_peak.clone(), 0.95);
    let rand_quiet_fa_amp = quiet_peak_q95
        .map(|q| rand_quiet_peak.iter().filter(|&&p| p >= q).count());

    println!("# Band stack — measurements");
    println!();
    println!("Events read: {n_events}");
    println!("Events with stack: {n_measured}");
    println!("Stations in catalogue: {}", STATIONS.len());
    println!();
    println!("## Stack (oval mask, front alignment)");
    println!(
        "SNR median: {}",
        show(median(&mut snrs.clone()), 2)
    );
    println!("SNR q95: {}", show(stack_q95, 2));
    println!(
        "Peak |dB/dt| median: {} nT/min, noise sigma median: {} nT/min",
        show(peak_med, 2),
        show(noise_med, 2)
    );
    println!(
        "Detections (SNR >= 3): {detected}/{n_measured} = {}",
        show(frac(detected, n_measured), 3)
    );
    println!(
        "Detections at 5% null threshold ({}): {}/{n_measured} = {}",
        show(quiet_q95, 2),
        show(det95.map(|v| v as f64), 0),
        show(det95.and_then(|v| frac(v, n_measured)), 3)
    );
    println!(
        "Detections at 5% amplitude threshold ({} nT/min): {}/{n_measured} = {}",
        show(quiet_peak_q95, 2),
        show(det_peak.map(|v| v as f64), 0),
        show(det_peak.and_then(|v| frac(v, n_measured)), 3)
    );
    println!(
        "Detections at both SNR and amplitude: {}/{n_measured} = {}",
        show(det_both.map(|v| v as f64), 0),
        show(det_both.and_then(|v| frac(v, n_measured)), 3)
    );
    println!("Stations available (mean): {}", show(stations_mean, 2));
    println!(
        "Fraction of station samples in band (mean): {}",
        show(in_band_mean, 3)
    );
    println!(
        "Stations in band per event: median {}, min {}, max {}; >=3 stations in {events_ge3}/{n_measured}",
        show(band_med, 1),
        show(Some(band_min), 1),
        show(Some(band_max), 1)
    );
    println!("Samples per stack (mean): {}", show(samples_mean, 0));
    println!("occupied minute bins (mean): {}", show(bins_mean, 0));
    println!();
    println!("## Single station");
    for (code, n, det, med, ssnr, nmed) in &single_stats {
        let q = single_q95.get(code).copied();
        let d95 = q.map(|q| ssnr.iter().filter(|&&s| s >= q).count());
        println!(
            "{code}: n={n}, Detections@3sigma={det} ({}), Detections@5%({})={}, SNR median={}, noise sigma median={} nT/min",
            show(frac(*det, *n), 3),
            show(q, 2),
            show(d95.map(|v| v as f64), 0),
            show(*med, 2),
            show(*nmed, 2)
        );
    }
    println!();
    println!("## Null: quiet days");
    println!("Windows with stack: {quiet_n}");
    println!(
        "Peak |dB/dt| median: {} nT/min (q95={}), noise sigma median: {} nT/min",
        show(quiet_peak_med, 2),
        show(quiet_peak_q95, 2),
        show(quiet_noise_med, 2)
    );
    println!(
        "Raw detections: {quiet_det}/{quiet_n} = {}",
        show(frac(quiet_det, quiet_n), 3)
    );
    println!(
        "500 draws: false alarms={quiet_fa}/500 = {}, SNR q95={}",
        show(frac(quiet_fa, 500), 3),
        show(quiet_q95, 2)
    );
    println!();
    println!("## Random station groups on storms (detection)");
    println!("Draws with stack: {}", rand_snr.len());
    println!(
        "Detections@3sigma={rand_fa}/{} = {}, SNR median={}, q95={}, noise sigma median={} nT/min",
        rand_snr.len(),
        show(frac(rand_fa, rand_snr.len()), 3),
        show(rand_med, 2),
        show(rand_q95, 2),
        show(rand_noise_med, 2)
    );
    println!();
    println!("## Null: random station groups on quiet days");
    println!("Draws with stack: {}", rand_quiet_snr.len());
    println!(
        "False alarms@3sigma={rand_quiet_fa3}/{} = {}",
        rand_quiet_snr.len(),
        show(frac(rand_quiet_fa3, rand_quiet_snr.len()), 3)
    );
    println!(
        "False alarms@amplitude({} nT/min)={}/{} = {}, Peak-q95={} nT/min",
        show(quiet_peak_q95, 2),
        show(rand_quiet_fa_amp.map(|v| v as f64), 0),
        rand_quiet_snr.len(),
        show(rand_quiet_fa_amp.and_then(|v| frac(v, rand_quiet_snr.len())), 3),
        show(rand_quiet_peak_q95, 2)
    );
    println!();
    println!("## Events (first 20)");
    for (label, _, s) in event_rows.iter().take(20) {
        println!(
            "{label}\t{:.2}\t{:.2}\t{:.2}\t{}",
            s.snr, s.peak, s.noise, s.n_samples
        );
    }
}
