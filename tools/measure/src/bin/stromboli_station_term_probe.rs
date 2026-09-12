use omegaflow_measure::depthphase::{arc_deg, arg_value, bandpass, median, unix_to_iso};
use omegaflow_measure::miniseed::decode_body;
use std::env;
use std::fs;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

const INGV_STATION_ROUTE: &str = "https://webservices.ingv.it/fdsnws/station/1/query";
const INGV_DATASELECT_ROUTE: &str = "https://webservices.ingv.it/fdsnws/dataselect/1/query";
const NETWORK: &str = "IV";
const CHANNEL: &str = "HHZ";
const CRATER_BOX: [f64; 4] = [38.75, 38.85, 15.15, 15.25];
const SECTION_S: f64 = 120.0;
const REPEATS: usize = 6;
const SPACING_S: f64 = 1800.0;
const CORR_GATE: f64 = 0.30;
const MAX_LAG_S: f64 = 2.0;
const EVENT_S: f64 = 20.0;
const KM_PER_DEG: f64 = 111.195;

#[derive(Clone)]
struct Station {
    net: String,
    sta: String,
    lat: f64,
    lon: f64,
}

fn days_from_civil(y: i64, m: i64, d: i64) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let mp = (m + 9) % 12;
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe - 719468
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
    let t = time
        .split(|c: char| c == '.' || c == 'Z' || c == 'z')
        .next()?;
    let mut tp = t.split(':');
    let hh: i64 = tp.next()?.parse().ok()?;
    let mm: i64 = tp.next().unwrap_or("0").parse().ok()?;
    let ss: i64 = tp.next().unwrap_or("0").parse().ok()?;
    let days = days_from_civil(y, m, d) as f64;
    Some(days * 86400.0 + hh as f64 * 3600.0 + mm as f64 * 60.0 + ss as f64)
}

fn curl_bytes(url: &str) -> (Option<u16>, Vec<u8>) {
    let tmp = env::temp_dir().join(format!("stromboli_{}.mseed", std::process::id()));
    let out = Command::new("curl")
        .arg("-sS")
        .arg("-L")
        .arg("-g")
        .arg("-m")
        .arg("120")
        .arg("--connect-timeout")
        .arg("20")
        .arg("-o")
        .arg(&tmp)
        .arg("-w")
        .arg("%{http_code}")
        .arg(url)
        .output();
    let code = match out {
        Ok(o) => {
            let s = String::from_utf8_lossy(&o.stdout);
            s.trim().parse::<u16>().ok().filter(|c| *c > 0)
        }
        Err(_) => None,
    };
    let body = match fs::read(&tmp) {
        Ok(b) => b,
        Err(_) => Vec::new(),
    };
    let _ = fs::remove_file(&tmp);
    (code, body)
}

fn parse_stations(body: &str) -> Vec<Station> {
    let mut lines = body.lines();
    let Some(header) = lines.next() else {
        return Vec::new();
    };
    let cols: Vec<&str> = header.trim_start_matches('#').split('|').collect();
    let idx = |name: &str| cols.iter().position(|c| c.trim() == name);
    let (Some(i_net), Some(i_sta), Some(i_lat), Some(i_lon), Some(i_end)) = (
        idx("Network"),
        idx("Station"),
        idx("Latitude"),
        idx("Longitude"),
        idx("EndTime"),
    ) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for line in lines {
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let f: Vec<&str> = line.split('|').collect();
        let widest = i_end.max(i_lon).max(i_lat).max(i_net).max(i_sta);
        if f.len() <= widest {
            continue;
        }
        if !f[i_end].trim().is_empty() {
            continue;
        }
        let Some(lat) = f[i_lat].trim().parse::<f64>().ok() else {
            continue;
        };
        let Some(lon) = f[i_lon].trim().parse::<f64>().ok() else {
            continue;
        };
        if !lat.is_finite() || !lon.is_finite() {
            continue;
        }
        out.push(Station {
            net: f[i_net].trim().to_string(),
            sta: f[i_sta].trim().to_string(),
            lat,
            lon,
        });
    }
    out
}

fn lag_to_ref(a: &[f64], b: &[f64], rate: f64, max_lag_s: f64) -> Option<(f64, f64)> {
    let max_lag = (max_lag_s * rate).round() as isize;
    let n = a.len().min(b.len());
    if n < 64 || max_lag <= 0 {
        return None;
    }
    let n_i = n as isize;
    let mut best: Option<(f64, f64)> = None;
    for lag in -max_lag..=max_lag {
        let mut num = 0.0;
        let mut aa = 0.0;
        let mut bb = 0.0;
        let mut cnt = 0usize;
        for i in 0..n {
            let j = i as isize + lag;
            if j < 0 || j >= n_i {
                continue;
            }
            let x = a[i];
            let y = b[j as usize];
            num += x * y;
            aa += x * x;
            bb += y * y;
            cnt += 1;
        }
        if cnt < 64 {
            continue;
        }
        let denom = (aa * bb).sqrt();
        if denom <= 1e-12 {
            continue;
        }
        let corr = num / denom;
        match best {
            Some((_, bc)) if corr <= bc => {}
            _ => best = Some((lag as f64 / rate, corr)),
        }
    }
    best
}

fn mad_about_median(xs: &mut [f64]) -> Option<f64> {
    if xs.is_empty() {
        return None;
    }
    let m = median(xs);
    let mut devs: Vec<f64> = xs.iter().map(|v| (v - m).abs()).collect();
    Some(median(&mut devs))
}

fn envelope_peak_index(bp: &[f64], rate: f64) -> Option<usize> {
    let dt = 1.0 / rate;
    let a = dt / (1.0 + dt);
    let mut env = 0.0;
    let mut best_i = 0usize;
    let mut best_v = 0.0f64;
    for (i, &v) in bp.iter().enumerate() {
        env += a * (v.abs() - env);
        if env > best_v {
            best_v = env;
            best_i = i;
        }
    }
    if best_v <= 1e-12 {
        None
    } else {
        Some(best_i)
    }
}

fn main() {
    println!("=== Stromboli station term — the fixed crater section as the volcano teacher ===");
    println!("source: INGV FDSN {INGV_DATASELECT_ROUTE} (network {NETWORK}, channel {CHANNEL})");
    println!(
        "crater section: station box lat {}..{} lon {}..{} · section {SECTION_S:.0} s × {REPEATS} repeats, {SPACING_S:.0} s apart",
        CRATER_BOX[0], CRATER_BOX[1], CRATER_BOX[2], CRATER_BOX[3]
    );
    println!();

    let station_url = format!(
        "{INGV_STATION_ROUTE}?network={NETWORK}&channel={CHANNEL}&minlatitude={}&maxlatitude={}&minlongitude={}&maxlongitude={}&level=channel&format=text",
        CRATER_BOX[0], CRATER_BOX[1], CRATER_BOX[2], CRATER_BOX[3]
    );
    let (code, body) = curl_bytes(&station_url);
    if code != Some(200) {
        eprintln!(
            "station query carried HTTP {} — the crater section stays unread (0 honored)",
            match code {
                Some(c) => c.to_string(),
                None => "no-verdict".to_string(),
            }
        );
        return;
    }
    let stations = parse_stations(&String::from_utf8_lossy(&body));
    if stations.is_empty() {
        eprintln!("no open {CHANNEL} station in the crater box — nothing measured (0 honored)");
        return;
    }
    println!(
        "{} open {CHANNEL} stations in the crater section:",
        stations.len()
    );
    for s in &stations {
        println!("  {}.{} at {:.4} N {:.4} E", s.net, s.sta, s.lat, s.lon);
    }
    println!();

    let mean_lat = stations.iter().map(|s| s.lat).sum::<f64>() / stations.len() as f64;
    let mean_lon = stations.iter().map(|s| s.lon).sum::<f64>() / stations.len() as f64;
    let mut ref_idx = 0usize;
    let mut ref_dist = f64::INFINITY;
    for (i, s) in stations.iter().enumerate() {
        let d = arc_deg(mean_lat, mean_lon, s.lat, s.lon);
        if d < ref_dist {
            ref_dist = d;
            ref_idx = i;
        }
    }
    let ref_key = format!("{}.{}", stations[ref_idx].net, stations[ref_idx].sta);
    println!("reference station: {ref_key} (closest to the section centroid)");
    println!(
        "station term = median cross-correlation lag of each station vs the reference, over the repeats"
    );
    println!();

    let args: Vec<String> = env::args().skip(1).collect();
    let end_override = arg_value(&args, "--end");

    let now = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(d) => d.as_secs_f64(),
        Err(_) => {
            eprintln!("the system clock precedes the epoch — no end time, no fabricated section");
            return;
        }
    };
    let section_end = match end_override {
        Some(iso) => match iso_to_unix(&iso) {
            Some(t) => t,
            None => {
                eprintln!("--end {iso} parses void — the section stays unread");
                return;
            }
        },
        None => now,
    };
    println!(
        "section ends at {} (--end to fix another section)",
        unix_to_iso(section_end)
    );
    println!();

    let mut lags: Vec<Vec<f64>> = vec![Vec::new(); stations.len()];
    let mut absent = vec![0usize; stations.len()];
    for r in 0..REPEATS {
        let end = section_end - r as f64 * SPACING_S;
        let start = end - SECTION_S;
        let start_iso = unix_to_iso(start);
        let end_iso = unix_to_iso(end);
        let ref_trace = match curl_bytes(&format!(
            "{INGV_DATASELECT_ROUTE}?network={}&station={}&channel={CHANNEL}&starttime={start_iso}&endtime={end_iso}&format=miniseed",
            stations[ref_idx].net, stations[ref_idx].sta
        )) {
            (Some(200), b) => match decode_body(&b) {
                Some((samples, rate)) => Some((bandpass(&samples, rate), rate)),
                None => None,
            },
            _ => None,
        };
        let Some((ref_bp, rate)) = ref_trace else {
            println!(
                "repeat {r}: reference {ref_key} carried no decodable record — section skipped"
            );
            continue;
        };
        let Some(peak) = envelope_peak_index(&ref_bp, rate) else {
            println!(
                "repeat {r}: reference {ref_key} carried no signal envelope — section skipped"
            );
            continue;
        };
        let seg = (EVENT_S * rate).round() as usize;
        if seg == 0 {
            continue;
        }
        let lo = peak.saturating_sub(seg / 2);
        let hi = (peak + seg / 2).min(ref_bp.len());
        if hi - lo < 64 {
            println!(
                "repeat {r}: the event window carries fewer than 64 samples — section skipped"
            );
            continue;
        }
        let ref_seg = &ref_bp[lo..hi];
        for (i, s) in stations.iter().enumerate() {
            let (sc, sb) = curl_bytes(&format!(
                "{INGV_DATASELECT_ROUTE}?network={}&station={}&channel={CHANNEL}&starttime={start_iso}&endtime={end_iso}&format=miniseed",
                s.net, s.sta
            ));
            if sc != Some(200) {
                absent[i] += 1;
                continue;
            }
            let Some((samples, _)) = decode_body(&sb) else {
                absent[i] += 1;
                continue;
            };
            let bp = bandpass(&samples, rate);
            if bp.len() < hi {
                absent[i] += 1;
                continue;
            }
            let sta_seg = &bp[lo..hi];
            match lag_to_ref(sta_seg, ref_seg, rate, MAX_LAG_S) {
                Some((lag, corr)) if corr >= CORR_GATE => lags[i].push(lag),
                Some((_, corr)) => {
                    absent[i] += 1;
                    eprintln!(
                        "repeat {r} {}.{} corr {corr:.2} below the {CORR_GATE} gate — skipped",
                        s.net, s.sta
                    );
                }
                None => absent[i] += 1,
            }
        }
    }

    println!();
    println!("station terms (median lag vs {ref_key}, + = arrives later):");
    println!(
        "{:>12}  {:>7}  {:>8}  {:>8}  {:>5}  {:>5}",
        "station", "dist km", "term s", "mad s", "n", "miss"
    );
    let mut rows: Vec<(String, f64, f64, Option<f64>, usize)> = Vec::new();
    for (i, s) in stations.iter().enumerate() {
        let d_km = arc_deg(stations[ref_idx].lat, stations[ref_idx].lon, s.lat, s.lon) * KM_PER_DEG;
        if lags[i].is_empty() {
            println!(
                "{:>12}  {:>7.2}  {:>8}  {:>8}  {:>5}  {:>5}",
                format!("{}.{}", s.net, s.sta),
                d_km,
                "absent",
                "-",
                0,
                absent[i]
            );
            continue;
        }
        let mut lag_series = lags[i].clone();
        let term = median(&mut lag_series);
        let mad = mad_about_median(&mut lag_series);
        rows.push((
            format!("{}.{}", s.net, s.sta),
            d_km,
            term,
            mad,
            lags[i].len(),
        ));
        match mad {
            Some(m) => println!(
                "{:>12}  {:>7.2}  {:>8.2}  {:>8.2}  {:>5}  {:>5}",
                format!("{}.{}", s.net, s.sta),
                d_km,
                term,
                m,
                lags[i].len(),
                absent[i]
            ),
            None => println!(
                "{:>12}  {:>7.2}  {:>8.2}  {:>8}  {:>5}  {:>5}",
                format!("{}.{}", s.net, s.sta),
                d_km,
                term,
                "absent",
                lags[i].len(),
                absent[i]
            ),
        }
    }

    if rows.len() >= 2 {
        let num = rows.iter().map(|r| r.1 * r.2).sum::<f64>();
        let den = rows.iter().map(|r| r.1 * r.1).sum::<f64>();
        if den > 1e-12 {
            let slope = num / den;
            println!();
            if slope > 0.0 && slope.is_finite() {
                let v = 1.0 / slope;
                println!(
                    "apparent phase velocity across the section: {:.2} km/s (lag grows {:.3} s per km from the reference)",
                    v, slope
                );
            } else {
                println!(
                    "apparent velocity: the lag-vs-distance fit carries a non-positive slope ({slope:.3} s/km) — the section carries no coherent crater source above the gate, so no velocity is read (0 honored)"
                );
            }
        }
    }
    println!();
    println!(
        "wiring: pending — the terms read the site/path bias of a fixed crater source; the handoff to the depth-phase station correction stays unbuilt"
    );
}
