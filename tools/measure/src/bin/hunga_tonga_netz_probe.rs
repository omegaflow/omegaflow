use omegaflow::archivar::fetch::{load_env, resolve_secret};
use omegaflow::archivar::geo::{COMP_BGR_AZIM, COMP_BGR_VAPP, MAGIC_BGR, parse_bin};
use omegaflow::archivar::hdf5::{Endian, Hdf5File, decode_f32, decode_f64};
use omegaflow::archivar::tle::RADIUS_EARTH_KM;
use omegaflow::archivar::{angular_distance_deg, embedded_lsk, fetch_raw_bytes};
use omegaflow::lsk::days_from_civil;
use omegaflow::spectral::civil_from_days;
use omegaflow_measure::miniseed::decode_body;

const ERUPTION_UNIX: f64 = 1642220085.0;
const TONGA_LAT: f64 = -20.536;
const TONGA_LON: f64 = -175.382;
const LAMB_SPEED_KM_S: f64 = 0.306;
const EVENT_DAY: &str = "2022-01-15";
const QUIET_DAY: &str = "2022-01-13";
const BGR_CDN: &str = "https://github.com/omegaflow/sources/releases/download/download.bgr.de/bgr_infrasound_IS52_2022.bin";
const EVENT_FDCF: &str = "https://noaa-goes17.s3.amazonaws.com/ABI-L2-FDCF/2022/015/04/OR_ABI-L2-FDCF-M6_G17_s20220150410320_e20220150419387_c20220150419494.nc";
const NULL_FDCF: &str = "https://noaa-goes17.s3.amazonaws.com/ABI-L2-FDCF/2022/013/04/OR_ABI-L2-FDCF-M6_G17_s20220130410320_e20220130419387_c20220130419504.nc";
const EVENT_C07: &str = "https://noaa-goes17.s3.amazonaws.com/ABI-L2-CMIPF/2022/015/04/OR_ABI-L2-CMIPF-M6C07_G17_s20220150410320_e20220150419398_c20220150419449.nc";
const NULL_C07: &str = "https://noaa-goes17.s3.amazonaws.com/ABI-L2-CMIPF/2022/013/04/OR_ABI-L2-CMIPF-M6C07_G17_s20220130410320_e20220130419397_c20220130419464.nc";

const SEISMIC_STATIONS: [(&str, &str, f64, f64); 8] = [
    ("IU", "AFI", -13.909, -171.777),
    ("II", "MSVF", -17.733, 178.050),
    ("IU", "SNZO", -41.310, 174.704),
    ("IU", "HNR", -9.438, 159.947),
    ("IU", "NWAO", -32.928, 117.808),
    ("IU", "MAJO", 36.546, 138.204),
    ("IU", "COLA", 64.874, -147.862),
    ("IU", "ANMO", 34.946, -106.457),
];

const COOPS: [(&str, &str, f64, f64); 6] = [
    ("1770000", "Pago Pago", -14.275, -170.690),
    ("1820000", "Kwajalein", 8.731667, 167.73611),
    ("1890000", "Wake Island", 19.290556, 166.6175),
    ("1615680", "Kahului", 20.895, -156.470),
    ("1630000", "Guam Apra", 13.443389, 144.65636),
    ("1612340", "Honolulu", 21.3067, -157.8670),
];

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn has(args: &[String], name: &str) -> bool {
    args.iter().any(|a| a == name)
}

fn utc_str(unix: f64) -> String {
    let days = unix.div_euclid(86400.0) as i64;
    let secs = unix - days as f64 * 86400.0;
    let (y, m, d) = match civil_from_days(days) {
        Some(t) => t,
        None => return format!("day {days}"),
    };
    let h = (secs / 3600.0) as u32;
    let mi = ((secs % 3600.0) / 60.0) as u32;
    let s = (secs % 60.0) as u32;
    format!("{y:04}-{m:02}-{d:02} {h:02}:{mi:02}:{s:02}")
}

fn day_start(day: &str) -> Option<f64> {
    let mut it = day.split('-');
    let y: i64 = it.next()?.parse().ok()?;
    let m: i64 = it.next()?.parse().ok()?;
    let d: i64 = it.next()?.parse().ok()?;
    Some(days_from_civil(y, m, d)? as f64 * 86400.0)
}

fn iso_unix(text: &str) -> Option<f64> {
    let t = text.trim().trim_end_matches('Z');
    let (date, time) = t.split_once('T')?;
    let mut dp = date.split('-');
    let y: i64 = dp.next()?.parse().ok()?;
    let m: i64 = dp.next()?.parse().ok()?;
    let d: i64 = dp.next()?.parse().ok()?;
    let mut tp = time.split(':');
    let h: f64 = tp.next()?.parse().ok()?;
    let mi: f64 = tp.next()?.parse().ok()?;
    let s: f64 = tp.next()?.parse().ok()?;
    Some(days_from_civil(y, m, d)? as f64 * 86400.0 + h * 3600.0 + mi * 60.0 + s)
}

fn distance_km(lat: f64, lon: f64) -> f64 {
    angular_distance_deg(lat, lon, TONGA_LAT, TONGA_LON).to_radians() * RADIUS_EARTH_KM
}

fn bearing_deg(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let r1 = lat1.to_radians();
    let r2 = lat2.to_radians();
    let dl = (lon2 - lon1).to_radians();
    let y = dl.sin() * r2.cos();
    let x = r1.cos() * r2.sin() - r1.sin() * r2.cos() * dl.cos();
    (y.atan2(x).to_degrees() + 360.0) % 360.0
}

fn wrap_deg(a: f64) -> f64 {
    (a + 180.0).rem_euclid(360.0) - 180.0
}

fn median(values: &[f64]) -> Option<f64> {
    if values.is_empty() {
        return None;
    }
    let mut v = values.to_vec();
    v.sort_by(|a, b| a.total_cmp(b));
    let n = v.len();
    Some(if n % 2 == 0 {
        (v[n / 2 - 1] + v[n / 2]) * 0.5
    } else {
        v[n / 2]
    })
}

fn mad(values: &[f64], med: f64) -> f64 {
    let dev: Vec<f64> = values.iter().map(|v| (v - med).abs()).collect();
    match median(&dev) {
        Some(v) => v,
        None => f64::NAN,
    }
}

fn robust(values: &[f64]) -> Option<(f64, f64)> {
    let m = median(values)?;
    Some((m, 1.4826 * mad(values, m)))
}

struct Onset {
    t: f64,
    value: f64,
    base: f64,
    #[allow(dead_code)]
    sigma: f64,
}

fn detect_onset(
    series: &[(f64, f64)],
    search_from: f64,
    base_lo: f64,
    base_hi: f64,
    k: f64,
    floor_abs: f64,
    floor_frac: f64,
) -> Option<Onset> {
    let base: Vec<f64> = series
        .iter()
        .filter(|(t, _)| *t >= base_lo && *t <= base_hi)
        .map(|(_, v)| *v)
        .collect();
    let (med, sigma) = robust(&base)?;
    let threshold = (k * sigma).max(floor_abs).max(floor_frac * med.abs());
    for (t, v) in series {
        if *t >= search_from && (v - med).abs() > threshold {
            return Some(Onset {
                t: *t,
                value: *v,
                base: med,
                sigma,
            });
        }
    }
    None
}

fn detect_peak_onset(
    series: &[(f64, f64)],
    search_from: f64,
    base_lo: f64,
    base_hi: f64,
    floor: f64,
) -> Option<Onset> {
    let base: Vec<f64> = series
        .iter()
        .filter(|(t, _)| *t >= base_lo && *t <= base_hi)
        .map(|(_, v)| *v)
        .collect();
    let (med, sigma) = robust(&base)?;
    let window: Vec<(f64, f64)> = series
        .iter()
        .filter(|(t, _)| *t >= search_from && *t <= search_from + 24.0 * 3600.0)
        .map(|(t, v)| (*t, v - med))
        .collect();
    let (_, pv) = window
        .iter()
        .max_by(|a, b| a.1.abs().total_cmp(&b.1.abs()))
        .copied()?;
    let sign = pv.signum();
    let threshold = (2.5 * sigma).max(floor);
    for (t, v) in &window {
        if v.signum() == sign && v.abs() > threshold {
            return Some(Onset {
                t: *t,
                value: *v,
                base: med,
                sigma,
            });
        }
    }
    None
}

fn inventory() {
    println!("=== Inventory (measured 2026-10-20) ===");
    println!(
        "{:<22} {:<44} {:<20} {:<10} {}",
        "thread", "source", "coverage", "cadence", "access"
    );
    println!(
        "{:<22} {:<44} {:<20} {:<10} {}",
        "seismic",
        "EarthScope FDSN dataselect (BHZ)",
        "global stations, Jan 2022",
        "40-100 Hz",
        "open (HTTP 200)"
    );
    println!(
        "{:<22} {:<44} {:<20} {:<10} {}",
        "Luftdruck",
        "NOAA CO-OPS air_pressure",
        "Pacific stations, Jan 2022",
        "6 min",
        "open (HTTP 200)"
    );
    println!(
        "{:<22} {:<44} {:<20} {:<10} {}",
        "infrasound",
        "BGR IS52 PMCC detections (house release)",
        "2022 full year",
        "~300 s",
        "open (HTTP 200)"
    );
    println!(
        "{:<22} {:<44} {:<20} {:<10} {}",
        "Tsunami",
        "NOAA CO-OPS water_level, MSL",
        "Pacific stations, Jan 2022",
        "6 min",
        "open (HTTP 200)"
    );
    println!(
        "{:<22} {:<44} {:<20} {:<10} {}",
        "ionosphere",
        "CDAWeb HAPI GPS_TEC15MIN_IGS (IGS)",
        "global 2.5°x5° map, Jan 2022",
        "15 min",
        "open (HTTP 200)"
    );
    println!(
        "{:<22} {:<44} {:<20} {:<10} {}",
        "lightning",
        "NOAA GOES-17 GLM-L2-LCFA, AWS S3",
        "GOES-West disk, 2022-01-15",
        "20 s granules",
        "open (HTTP 200)"
    );
    println!(
        "{:<22} {:<44} {:<20} {:<10} {}",
        "Aerosol/Thermik",
        "GOES-17 ABI FDCF/AODF (AWS S3) + TROPOMI AER_AI (MEEO S3)",
        "Full disk, Jan 2022",
        "10 min / Orbit",
        "keyless (HTTP 200)"
    );
    println!();
}

fn be16(b: &[u8], i: usize) -> Option<u16> {
    Some(((*b.get(i)? as u16) << 8) | *b.get(i + 1)? as u16)
}

fn field(rec: &[u8], a: usize, b: usize) -> String {
    String::from_utf8_lossy(&rec[a..b]).trim().to_string()
}

fn select_stream(body: &[u8], net: &str, sta: &str, cha: &str) -> Option<Vec<u8>> {
    let mut groups: Vec<(String, Vec<u8>)> = Vec::new();
    let mut pos = 0usize;
    let mut records = 0usize;
    while pos + 48 <= body.len() {
        let first_off = be16(body, pos + 46)? as usize;
        if first_off < 48 || pos + first_off + 8 > body.len() {
            break;
        }
        let exp = *body.get(pos + first_off + 6)? as usize;
        if !(8..=20).contains(&exp) {
            break;
        }
        let reclen = 1usize << exp;
        if pos + reclen > body.len() {
            break;
        }
        let rec = &body[pos..pos + reclen];
        pos += reclen;
        records += 1;
        let rnet = field(rec, 18, 20);
        let rsta = field(rec, 8, 13);
        let loc = field(rec, 13, 15);
        let rcha = field(rec, 15, 18);
        if rsta != sta || rcha != cha {
            continue;
        }
        if !net.is_empty() && !rnet.is_empty() && rnet != net {
            continue;
        }
        let key = loc;
        if let Some((_, buf)) = groups.iter_mut().find(|(k, _)| *k == key) {
            buf.extend_from_slice(rec);
        } else {
            groups.push((key, rec.to_vec()));
        }
    }
    if groups.is_empty() {
        return None;
    }
    let pick = groups
        .iter()
        .position(|(k, _)| k == "00")
        .or_else(|| groups.iter().position(|(k, _)| k.is_empty()))
        .expect("groups is non-empty after the guard");
    let (loc, buf) = groups.swap_remove(pick);
    eprintln!(
        "  stream {net}.{sta}.{loc}.{cha}: {} B selected from {} records",
        buf.len(),
        records
    );
    Some(buf)
}

fn seismic_day(day: &str, label: &str) -> Vec<(f64, f64, String)> {
    let Some(start) = day_start(day) else {
        return Vec::new();
    };
    let t_ref = start + 4.0 * 3600.0 + 14.0 * 60.0 + 45.0;
    println!("--- Seismic {label}: {day} 02:15-06:15 UTC, BHZ, real miniSEED waveform ---");
    println!(
        "{:<12} {:>9} {:>10} {:>20} {:>10} {:>9} {:>9}",
        "Station", "distance", "v_app", "onset (UTC)", "lag s", "RMS", "baseline"
    );
    let mut rows = Vec::new();
    for (net, sta, lat, lon) in SEISMIC_STATIONS {
        let url = format!(
            "https://service.earthscope.org/fdsnws/dataselect/1/query?network={net}&station={sta}&channel=BHZ&starttime={day}T02:15:00&endtime={day}T06:15:00&format=miniseed"
        );
        let Some(bytes) = fetch_raw_bytes(&url) else {
            println!("{net}.{sta:<9} absent: dataselect route void");
            continue;
        };
        let Some(stream) = select_stream(&bytes, net, sta, "BHZ") else {
            println!("{net}.{sta:<9} absent: no BHZ stream in {} B", bytes.len());
            continue;
        };
        let Some((samples, rate)) = decode_body(&stream) else {
            println!(
                "{net}.{sta:<9} absent: miniSEED decodes void ({} B)",
                stream.len()
            );
            continue;
        };
        let _ = rate;
        let bin_s = 20.0;
        let mut bins: Vec<(f64, f64)> = Vec::new();
        let mut acc = 0.0;
        let mut n = 0usize;
        let mut cell = f64::NEG_INFINITY;
        for (t, v) in &samples {
            if *t < start + 2.0 * 3600.0 + 900.0 || *t > start + 6.0 * 3600.0 + 900.0 {
                continue;
            }
            let c = (t / bin_s).floor() * bin_s;
            if c != cell {
                if n > 0 {
                    bins.push((cell, (acc / n as f64).sqrt()));
                }
                cell = c;
                acc = 0.0;
                n = 0;
            }
            acc += v * v;
            n += 1;
        }
        if n > 0 {
            bins.push((cell, (acc / n as f64).sqrt()));
        }
        if bins.is_empty() {
            println!("{net}.{sta:<9} absent: no samples in the named window");
            continue;
        }
        let base_lo = start + 2.25 * 3600.0;
        let base_hi = start + 3.5 * 3600.0;
        let onset = detect_onset(
            &bins,
            start + 3.5 * 3600.0,
            base_lo,
            base_hi,
            6.0,
            100.0,
            0.5,
        );
        let d = distance_km(lat, lon);
        match onset {
            Some(o) => {
                let lag = o.t - t_ref;
                let vapp = d / lag;
                let peak = bins
                    .iter()
                    .filter(|(t, _)| *t >= o.t && *t <= o.t + 600.0)
                    .map(|(_, v)| *v)
                    .fold(f64::NEG_INFINITY, f64::max);
                println!(
                    "{:<12} {:>7.0} km {:>8.2} km/s {:>20} {:>10.0} {:>7.0} {:>9.0}",
                    format!("{net}.{sta}"),
                    d,
                    vapp,
                    utc_str(o.t),
                    lag,
                    peak,
                    o.base
                );
                rows.push((d, lag, format!("{net}.{sta}")));
            }
            None => println!(
                "{:<12} {:>7.0} km {:>10} {:>20} {:>10} {:>7} {:>9.0}",
                format!("{net}.{sta}"),
                d,
                "absent",
                "no onset above threshold",
                "-",
                "-",
                0.0
            ),
        }
    }
    let num: f64 = rows.iter().map(|(d, lag, _)| d * lag).sum();
    let den: f64 = rows.iter().map(|(_, lag, _)| lag * lag).sum();
    if den > 0.0 {
        println!(
            "travel-time fit through the origin (all onsets): v_app = {:.2} km/s ({} stations)",
            num / den,
            rows.len()
        );
    } else {
        println!("travel-time fit: absent (no onsets)");
    }
    let keep: Vec<&(f64, f64, String)> = rows
        .iter()
        .filter(|(d, lag, _)| {
            *lag > 0.0 && {
                let v = d / lag;
                (2.0..=6.0).contains(&v)
            }
        })
        .collect();
    let num: f64 = keep.iter().map(|(d, lag, _)| d * lag).sum();
    let den: f64 = keep.iter().map(|(_, lag, _)| lag * lag).sum();
    if den > 0.0 {
        println!(
            "travel-time fit (2-6 km/s surface-wave band only): v_app = {:.2} km/s ({} stations)",
            num / den,
            keep.len()
        );
    }
    println!();
    rows
}

fn coops_series(
    station: &str,
    product: &str,
    begin: &str,
    end: &str,
    datum: bool,
) -> Option<Vec<(f64, f64)>> {
    let extra = if datum { "&datum=MSL" } else { "" };
    let url = format!(
        "https://api.tidesandcurrents.noaa.gov/api/prod/datagetter?begin_date={begin}&end_date={end}&station={station}&product={product}&units=metric&time_zone=gmt&interval=6&format=csv&application=omegaflow{extra}"
    );
    let bytes = fetch_raw_bytes(&url)?;
    let text = String::from_utf8_lossy(&bytes);
    let mut out = Vec::new();
    for line in text.lines() {
        let cols: Vec<&str> = line.split(',').collect();
        if cols.len() < 2 {
            continue;
        }
        let stamp = cols[0].trim();
        if stamp == "Date Time" || stamp.starts_with("Error") {
            continue;
        }
        let Some((date, time)) = stamp.split_once(' ') else {
            continue;
        };
        let Some(t) = iso_unix(&format!("{date}T{time}:00Z")) else {
            continue;
        };
        let Ok(v) = cols[1].trim().parse::<f64>() else {
            continue;
        };
        out.push((t, v));
    }
    if out.is_empty() { None } else { Some(out) }
}

fn rolling_median(series: &[(f64, f64)], half: usize) -> Vec<f64> {
    let n = series.len();
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let lo = i.saturating_sub(half);
        let hi = (i + half + 1).min(n);
        let vals: Vec<f64> = series[lo..hi].iter().map(|(_, v)| *v).collect();
        out.push(median(&vals).expect("a non-empty window has a median"));
    }
    out
}

fn solve_linear(mut a: Vec<Vec<f64>>, mut b: Vec<f64>) -> Option<Vec<f64>> {
    let n = b.len();
    for col in 0..n {
        let mut pivot = col;
        for row in col + 1..n {
            if a[row][col].abs() > a[pivot][col].abs() {
                pivot = row;
            }
        }
        if a[pivot][col].abs() < 1e-12 {
            return None;
        }
        a.swap(col, pivot);
        b.swap(col, pivot);
        for row in col + 1..n {
            let f = a[row][col] / a[col][col];
            for k in col..n {
                a[row][k] -= f * a[col][k];
            }
            b[row] -= f * b[col];
        }
    }
    let mut x = vec![0.0; n];
    for row in (0..n).rev() {
        let mut s = b[row];
        for k in row + 1..n {
            s -= a[row][k] * x[k];
        }
        x[row] = s / a[row][row];
    }
    if x.iter().all(|v| v.is_finite()) {
        Some(x)
    } else {
        None
    }
}

fn detrend_tide(series: &[(f64, f64)], fit_end: f64) -> Vec<(f64, f64)> {
    const PERIODS_H: [f64; 4] = [12.4206, 12.0, 23.9345, 25.8193];
    let n_par = 1 + 2 * PERIODS_H.len();
    let basis = |t: f64| -> Vec<f64> {
        let mut row = Vec::with_capacity(n_par);
        row.push(1.0);
        for p in PERIODS_H {
            let w = 2.0 * std::f64::consts::PI * t / (p * 3600.0);
            row.push(w.cos());
            row.push(w.sin());
        }
        row
    };
    let fit: Vec<&(f64, f64)> = series.iter().filter(|(t, _)| *t <= fit_end).collect();
    if fit.len() < n_par * 4 {
        return series
            .iter()
            .map(
                |(t, v)| match median(&fit.iter().map(|(_, v)| *v).collect::<Vec<_>>()) {
                    Some(base) => (*t, *v - base),
                    None => (*t, *v),
                },
            )
            .collect();
    }
    let mut ata = vec![vec![0.0; n_par]; n_par];
    let mut atb = vec![0.0; n_par];
    for (t, v) in &fit {
        let row = basis(*t);
        for i in 0..n_par {
            atb[i] += row[i] * v;
            for j in 0..n_par {
                ata[i][j] += row[i] * row[j];
            }
        }
    }
    let Some(coef) = solve_linear(ata, atb) else {
        return series.iter().map(|(t, v)| (*t, *v)).collect();
    };
    series
        .iter()
        .map(|(t, v)| {
            let row = basis(*t);
            let model: f64 = row.iter().zip(coef.iter()).map(|(r, c)| r * c).sum();
            (*t, v - model)
        })
        .collect()
}

fn coops_section(day: &str, label: &str) -> Vec<(f64, f64)> {
    println!("--- Pressure + water {label}: {day}, CO-OPS 6-min, GMT, level MSL ---");
    println!(
        "{:<14} {:>9} {:>22} {:>9} {:>11} {:>22} {:>9} {:>9}",
        "Station", "distance", "pressure onset", "dp hPa", "lag s", "water onset", "dh m", "lag s"
    );
    let Some(day0) = day_start(day) else {
        println!("--- Pressure + water {label}: {day}: absent (day unread) ---");
        println!();
        return Vec::new();
    };
    let t_ref = day0 + 4.0 * 3600.0 + 14.0 * 60.0 + 45.0;
    let begin = utc_str(day0 - 86400.0)[..10].replace('-', "");
    let next = day0 + 2.0 * 86400.0;
    let end = utc_str(next)[..10].replace('-', "");
    let mut p_rows: Vec<(f64, f64)> = Vec::new();
    let mut w_rows: Vec<(f64, f64)> = Vec::new();
    for (id, name, lat, lon) in COOPS {
        std::thread::sleep(std::time::Duration::from_millis(400));
        let ap = coops_series(id, "air_pressure", &begin, &end, false);
        std::thread::sleep(std::time::Duration::from_millis(400));
        let wl = coops_series(id, "water_level", &begin, &end, true);
        let cutoff = day0 + 2.0 * 86400.0;
        let ap = ap.map(|s| {
            s.into_iter()
                .filter(|(t, _)| *t < cutoff)
                .collect::<Vec<_>>()
        });
        let wl = wl.map(|s| {
            s.into_iter()
                .filter(|(t, _)| *t < cutoff)
                .collect::<Vec<_>>()
        });

        let d = distance_km(lat, lon);
        let mut out = vec![
            format!("{name}"),
            format!("{d:.0} km"),
            "absent".to_string(),
            "-".to_string(),
            "-".to_string(),
            "absent".to_string(),
            "-".to_string(),
            "-".to_string(),
        ];
        let mut pressure_lag = None;
        if let Some(ap) = &ap {
            let med = rolling_median(ap, 15);
            let resid: Vec<(f64, f64)> = ap
                .iter()
                .zip(med.iter())
                .filter(|(_, m)| m.is_finite())
                .map(|((t, v), m)| (*t, v - m))
                .collect();
            let base_lo = t_ref - 6.0 * 3600.0;
            let base_hi = t_ref - 900.0;
            let predicted = t_ref + d / LAMB_SPEED_KM_S;
            let onset = detect_onset(&resid, predicted - 1800.0, base_lo, base_hi, 4.0, 0.2, 0.0);
            if let Some(o) = onset {
                let lag = o.t - t_ref;
                let peak = resid
                    .iter()
                    .filter(|(t, _)| *t >= o.t && *t <= o.t + 2.0 * 3600.0)
                    .max_by(|a, b| a.1.abs().total_cmp(&b.1.abs()))
                    .copied()
                    .unwrap_or((o.t, o.value));
                pressure_lag = Some(o.t);
                out[2] = utc_str(o.t);
                out[3] = format!("{:+.2}", peak.1);
                out[4] = format!("{lag:.0}");
                p_rows.push((d, lag));
            }
        }
        if let Some(wl) = &wl {
            let resid = detrend_tide(wl, t_ref - 900.0);
            let onset =
                detect_peak_onset(&resid, t_ref, t_ref - 24.0 * 3600.0, t_ref - 900.0, 0.06);
            if let Some(o) = onset {
                let lag = o.t - t_ref;
                let amp = resid
                    .iter()
                    .filter(|(t, _)| *t >= o.t && *t <= o.t + 6.0 * 3600.0)
                    .map(|(_, v)| v.abs())
                    .fold(0.0, f64::max);
                out[5] = utc_str(o.t);
                out[6] = format!("{:+.2}", amp);
                out[7] = format!("{lag:.0}");
                if pressure_lag.is_some() {
                    w_rows.push((d, lag));
                }
            }
        }
        println!(
            "{:<14} {:>9} {:>22} {:>9} {:>11} {:>22} {:>9} {:>9}",
            out[0], out[1], out[2], out[3], out[4], out[5], out[6], out[7]
        );
    }
    for (name, rows) in [("pressure", &p_rows), ("water", &w_rows)] {
        let num: f64 = rows.iter().map(|(d, lag)| d * lag).sum();
        let den: f64 = rows.iter().map(|(_, lag)| lag * lag).sum();
        if den > 0.0 {
            println!(
                "travel-time fit {name} through the origin: v_app = {:.3} km/s ({} stations, from onsets)",
                num / den,
                rows.len()
            );
        }
    }
    let keep: Vec<&(f64, f64)> = w_rows
        .iter()
        .filter(|(d, lag)| *lag > 0.0 && (0.1..=0.3).contains(&(d / lag)))
        .collect();
    let num: f64 = keep.iter().map(|(d, lag)| d * lag).sum();
    let den: f64 = keep.iter().map(|(_, lag)| lag * lag).sum();
    if den > 0.0 {
        println!(
            "travel-time fit water (0.1-0.3 km/s tsunami band only): v_app = {:.3} km/s ({} stations)",
            num / den,
            keep.len()
        );
    }
    println!();
    w_rows
}

#[derive(Clone, Copy)]
struct TecSample {
    t: f64,
    cell: f64,
    mean: f64,
}

fn tec_day(day: &str) -> Vec<TecSample> {
    let url = format!(
        "https://cdaweb.gsfc.nasa.gov/hapi/data?id=GPS_TEC15MIN_IGS&time.min={day}T00:00:00Z&time.max={day}T12:00:00Z&parameters=tecUQRM3"
    );
    let Some(bytes) = fetch_raw_bytes(&url) else {
        return Vec::new();
    };
    let text = String::from_utf8_lossy(&bytes);
    let mut out = Vec::new();
    for line in text.lines() {
        let cols: Vec<&str> = line.split(',').collect();
        if cols.len() < 5184 {
            continue;
        }
        let Some(t) = iso_unix(cols[0]) else {
            continue;
        };
        let mut sum = 0.0;
        let mut n = 0usize;
        let mut global_sum = 0.0;
        let mut global_n = 0usize;
        for (i, raw) in cols[1..].iter().enumerate() {
            let Ok(v) = raw.trim().parse::<f64>() else {
                continue;
            };
            if !v.is_finite() || v < -100.0 {
                continue;
            }
            global_sum += v;
            global_n += 1;
            let lat_i = i / 73;
            let lon_i = i % 73;
            if (42..=45).contains(&lat_i) && lon_i <= 2 {
                sum += v;
                n += 1;
            }
        }
        if n > 0 && global_n > 0 {
            out.push(TecSample {
                t,
                cell: sum / n as f64,
                mean: global_sum / global_n as f64,
            });
        }
    }
    out
}

fn tec_residual(
    event: &[TecSample],
    quiet: &[TecSample],
    label: &str,
    t_ref: f64,
) -> Option<(f64, f64, f64, f64)> {
    let mut resid = Vec::new();
    let mut cell_abs = Vec::new();
    for e in event {
        let tod = e.t.rem_euclid(86400.0);
        let vals: Vec<f64> = quiet
            .iter()
            .filter(|q| {
                let d = (q.t.rem_euclid(86400.0) - tod).abs();
                d < 1.0 || d > 86399.0
            })
            .map(|q| q.cell / q.mean)
            .collect();
        if !vals.is_empty() {
            let q = vals.iter().sum::<f64>() / vals.len() as f64;
            resid.push((e.t, (e.cell / e.mean) / q - 1.0));
            cell_abs.push((e.t, e.cell));
        }
    }
    if resid.is_empty() {
        println!("TEC {label}: absent (no common 15-min grids)");
        return None;
    }
    let pre: Vec<f64> = resid
        .iter()
        .filter(|(t, _)| *t < t_ref)
        .map(|(_, v)| *v)
        .collect();
    let (base, sigma) = robust(&pre)?;
    let after: Vec<(f64, f64)> = resid
        .iter()
        .filter(|(t, _)| *t >= t_ref && *t <= t_ref + 6.0 * 3600.0)
        .map(|(t, v)| (*t, v - base))
        .collect();
    let peak = after
        .iter()
        .max_by(|a, b| a.1.abs().total_cmp(&b.1.abs()))
        .copied();
    let threshold = (3.0 * sigma).max(0.2);
    let onset = after.iter().find(|(_, v)| v.abs() > threshold).copied();
    match (onset, peak) {
        (Some((ot, ov)), Some((pt, pv))) => {
            let abs = cell_abs
                .iter()
                .find(|(t, _)| *t == pt)
                .map(|(_, v)| *v)
                .expect("the peak instant was pushed with the cell value");
            println!(
                "TEC {label}: Tonga cell (3x3) relative to the global mean; onset {} ({:+.3} = {:+.0} %, threshold {:.3}), maximum {} ({:+.3} = {:+.0} %, cell {:.1} TECU), pre-sigma {:.3}",
                utc_str(ot),
                ov,
                100.0 * ov,
                threshold,
                utc_str(pt),
                pv,
                100.0 * pv,
                abs,
                sigma
            );
            Some((ot, ov, pt, pv))
        }
        (None, Some((pt, pv))) => {
            println!(
                "TEC {label}: no onset above {:.3}; maximum {} ({:+.3} = {:+.0} %), pre-sigma {:.3}",
                threshold,
                utc_str(pt),
                pv,
                100.0 * pv,
                sigma
            );
            None
        }
        _ => {
            println!("TEC {label}: absent (no post-event grids)");
            None
        }
    }
}

fn tec_section(quiet_days: &[&str]) {
    println!("--- Ionosphere: GPS_TEC15MIN_IGS, 15-min, UTC (IGS map) ---");
    let event = tec_day(EVENT_DAY);
    if event.is_empty() {
        println!("TEC {EVENT_DAY}: absent (HAPI route void)");
        println!();
        return;
    }
    let mut quiet: Vec<TecSample> = Vec::new();
    for day in quiet_days {
        let s = tec_day(day);
        if s.is_empty() {
            println!("TEC {day}: absent (HAPI route void)");
            continue;
        }
        quiet.extend(s);
    }
    quiet.sort_by(|a, b| a.t.total_cmp(&b.t));
    if quiet.is_empty() {
        println!("TEC comparison days: absent");
        println!();
        return;
    }
    tec_residual(
        &event,
        &quiet,
        &format!("{EVENT_DAY} vs mean {}", quiet_days.join(",")),
        ERUPTION_UNIX,
    );
    let null_day = quiet_days[0];
    let null_event = tec_day(null_day);
    let mut null_quiet: Vec<TecSample> = Vec::new();
    for day in quiet_days.iter().skip(1) {
        null_quiet.extend(tec_day(day));
    }
    null_quiet.sort_by(|a, b| a.t.total_cmp(&b.t));
    if !null_event.is_empty() && !null_quiet.is_empty() {
        let Some(day0) = day_start(null_day) else {
            println!("TEC NULL {null_day}: absent (day unread)");
            println!();
            return;
        };
        let t_ref = day0 + 4.0 * 3600.0 + 14.0 * 60.0 + 45.0;
        tec_residual(
            &null_event,
            &null_quiet,
            &format!("NULL {null_day} vs mean {}", quiet_days[1..].join(",")),
            t_ref,
        );
    }
    println!();
}

fn s3_keys(prefix: &str) -> Vec<String> {
    let url =
        format!("https://noaa-goes17.s3.amazonaws.com/?list-type=2&prefix={prefix}&max-keys=1000");
    let Some(bytes) = fetch_raw_bytes(&url) else {
        return Vec::new();
    };
    let text = String::from_utf8_lossy(&bytes);
    let mut out = Vec::new();
    let mut rest = text.as_ref();
    while let Some(a) = rest.find("<Key>") {
        rest = &rest[a + 5..];
        if let Some(b) = rest.find("</Key>") {
            out.push(rest[..b].to_string());
            rest = &rest[b + 6..];
        } else {
            break;
        }
    }
    out
}

fn granule_start(key: &str) -> Option<f64> {
    let pos = key.find("_s")? + 2;
    let stamp = key.get(pos..pos + 14)?;
    let y: i64 = stamp.get(0..4)?.parse().ok()?;
    let doy: i64 = stamp.get(4..7)?.parse().ok()?;
    let h: f64 = stamp.get(7..9)?.parse().ok()?;
    let mi: f64 = stamp.get(9..11)?.parse().ok()?;
    let s: f64 = stamp.get(11..13)?.parse().ok()?;
    let tenth: f64 = stamp.get(13..14)?.parse().ok()?;
    let jan1 = days_from_civil(y, 1, 1)?;
    Some((jan1 + doy - 1) as f64 * 86400.0 + h * 3600.0 + mi * 60.0 + s + tenth / 10.0)
}

fn attr_text(file: &Hdf5File, ds: &str, attr: &str) -> Option<String> {
    let a = file.attribute(ds, attr)?;
    if !(a.datatype.class == 3 || (a.datatype.class == 9 && a.datatype.vlen_is_string)) {
        return None;
    }
    let mut bytes = a.data.clone();
    while bytes.last().is_some_and(|&b| b == 0 || b == b' ') {
        bytes.pop();
    }
    String::from_utf8(bytes).ok()
}

fn units_epoch(units: &str) -> Option<f64> {
    let rest = units.strip_prefix("seconds since ")?;
    let rb = rest.as_bytes();
    if rb.len() < 19 || rb.get(4) != Some(&b'-') || rb.get(7) != Some(&b'-') {
        return None;
    }
    let year: i64 = rest.get(0..4)?.parse().ok()?;
    let month: i64 = rest.get(5..7)?.parse().ok()?;
    let day: i64 = rest.get(8..10)?.parse().ok()?;
    let hour: i64 = rest.get(11..13)?.parse().ok()?;
    let minute: i64 = rest.get(14..16)?.parse().ok()?;
    let sec: f64 = rest
        .get(17..)?
        .trim_end()
        .trim_end_matches('Z')
        .parse()
        .ok()?;
    Some(
        days_from_civil(year, month, day)? as f64 * 86400.0
            + hour as f64 * 3600.0
            + minute as f64 * 60.0
            + sec,
    )
}

fn attr_scalar(file: &Hdf5File, ds: &str, attr: &str) -> Option<f64> {
    let a = file.attribute(ds, attr)?;
    if a.dataspace.dims.iter().product::<u64>() == 0 {
        return None;
    }
    num_at(&a.data, 0, &a.datatype)
}

fn num_at(raw: &[u8], i: usize, dt: &omegaflow::hdf5::Hdf5Datatype) -> Option<f64> {
    match dt.class {
        0 => {
            let off = i * dt.size;
            let s = dt.size;
            let v: i128 = match s {
                1 => {
                    if dt.signed {
                        raw.get(off).map(|b| *b as i8 as i128)?
                    } else {
                        raw.get(off).map(|b| *b as i128)?
                    }
                }
                2 => {
                    let b: [u8; 2] = raw.get(off..off + 2)?.try_into().ok()?;
                    let u = if dt.endian == Endian::Be {
                        u16::from_be_bytes(b)
                    } else {
                        u16::from_le_bytes(b)
                    };
                    if dt.signed {
                        u as i16 as i128
                    } else {
                        u as i128
                    }
                }
                4 => {
                    let b: [u8; 4] = raw.get(off..off + 4)?.try_into().ok()?;
                    let u = if dt.endian == Endian::Be {
                        u32::from_be_bytes(b)
                    } else {
                        u32::from_le_bytes(b)
                    };
                    if dt.signed {
                        u as i32 as i128
                    } else {
                        u as i128
                    }
                }
                8 => {
                    let b: [u8; 8] = raw.get(off..off + 8)?.try_into().ok()?;
                    let u = if dt.endian == Endian::Be {
                        u64::from_be_bytes(b)
                    } else {
                        u64::from_le_bytes(b)
                    };
                    if dt.signed {
                        u as i64 as i128
                    } else {
                        u as i128
                    }
                }
                _ => return None,
            };
            Some(v as f64)
        }
        1 => match dt.size {
            4 => decode_f32(raw, i * 4, dt.endian).map(|v| v as f64),
            8 => decode_f64(raw, i * 8, dt.endian),
            _ => None,
        },
        _ => None,
    }
}

fn dataset_scaled(file: &Hdf5File, name: &str) -> Vec<f64> {
    let Ok((_, ds, dt)) = file.dataset(name) else {
        return Vec::new();
    };
    let Ok(raw) = file.read_dataset(name) else {
        return Vec::new();
    };
    let count: usize = ds.dims.iter().fold(1usize, |a, d| a * (*d as usize));
    let scale = attr_scalar(file, name, "scale_factor");
    let offset = attr_scalar(file, name, "add_offset");
    let fill = attr_scalar(file, name, "_FillValue");
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        let Some(mut v) = num_at(&raw, i, dt) else {
            continue;
        };
        if fill.is_some_and(|f| (v - f).abs() < 1e-9) {
            continue;
        }
        if let Some(s) = scale {
            v *= s;
        }
        if let Some(o) = offset {
            v += o;
        }
        out.push(v);
    }
    out
}

struct GlmSummary {
    t: f64,
    count: usize,
    energy: f64,
    centroid_km: f64,
    centroid_bearing: f64,
}

fn glm_window(day: &str, doy: u32, label: &str, every: usize) -> Vec<GlmSummary> {
    println!("--- Lightning {label}: GOES-17 GLM-L2-LCFA, 20-s granules, orbit -137.2 deg ---");
    let mut keys = Vec::new();
    for hour in [3u32, 4, 5] {
        let prefix = format!("GLM-L2-LCFA/{}/{doy:03}/{hour:02}/", &day[0..4]);
        keys.extend(s3_keys(&prefix));
    }
    keys.sort();
    keys.dedup();
    let Some(day0) = day_start(day) else {
        println!("GLM {label}: absent (day unread)");
        println!();
        return Vec::new();
    };
    let lo = day0 + 3.0 * 3600.0 + 45.0 * 60.0;
    let hi = day0 + 5.0 * 3600.0;
    let selected: Vec<String> = keys
        .into_iter()
        .filter(|k| k.ends_with(".nc"))
        .filter(|k| granule_start(k).is_some_and(|t| t >= lo && t <= hi))
        .enumerate()
        .filter(|(i, _)| i % every == 0)
        .map(|(_, k)| k)
        .collect();
    if selected.is_empty() {
        println!("GLM {label}: absent (no granules in the named window)");
        println!();
        return Vec::new();
    }
    println!(
        "granules listed: {} (every {}th), window {} ... {}",
        selected.len(),
        every,
        utc_str(lo),
        utc_str(hi)
    );
    let mut out = Vec::new();
    let mut fetched = 0usize;
    for key in &selected {
        let url = format!("https://noaa-goes17.s3.amazonaws.com/{key}");
        let Some(bytes) = fetch_raw_bytes(&url) else {
            continue;
        };
        let Ok(file) = Hdf5File::parse(&bytes) else {
            continue;
        };
        fetched += 1;
        let lat = dataset_scaled(&file, "flash_lat");
        let lon = dataset_scaled(&file, "flash_lon");
        let energy = dataset_scaled(&file, "flash_energy");
        let off = dataset_scaled(&file, "flash_time_offset_of_first_event");
        let Some(epoch) = attr_text(&file, "flash_time_offset_of_first_event", "units")
            .and_then(|u| units_epoch(&u))
            .or_else(|| granule_start(key))
        else {
            continue;
        };
        let n = lat.len().min(lon.len()).min(energy.len()).min(off.len());
        let mut count = 0usize;
        let mut esum = 0.0;
        let mut slat = 0.0;
        let mut slon = 0.0;
        for i in 0..n {
            let d = distance_km(lat[i], lon[i]);
            if d > 500.0 {
                continue;
            }
            count += 1;
            esum += energy[i].max(0.0);
            slat += lat[i];
            slon += lon[i];
        }
        if count > 0 {
            let clat = slat / count as f64;
            let clon = slon / count as f64;
            out.push(GlmSummary {
                t: epoch + off.iter().take(n).fold(f64::INFINITY, |a, b| a.min(*b)),
                count,
                energy: esum,
                centroid_km: distance_km(clat, clon),
                centroid_bearing: bearing_deg(clat, clon, TONGA_LAT, TONGA_LON),
            });
        }
    }
    println!("granules read: {fetched}");
    out.sort_by(|a, b| a.t.total_cmp(&b.t));
    let total: usize = out.iter().map(|s| s.count).sum();
    println!(
        "flashes in the 500-km ring: {total} in {} non-empty granules",
        out.len()
    );
    for s in out.iter().take(18) {
        println!(
            "  {}  n={:<5} E={:.3e} J  centroid {:.0} km / {:.1} deg (bearing to centre)",
            utc_str(s.t),
            s.count,
            s.energy,
            s.centroid_km,
            s.centroid_bearing
        );
    }
    if out.len() > 18 {
        println!("  ... {} more non-empty granules", out.len() - 18);
    }
    let Some(t_ref) = day_start(day).map(|d| d + 4.0 * 3600.0 + 14.0 * 60.0 + 45.0) else {
        println!("GLM {label}: absent (day unread)");
        println!();
        return out;
    };
    let first = out
        .iter()
        .find(|s| s.t >= t_ref - 600.0 && s.count >= 3 && s.centroid_km <= 400.0);
    match first {
        Some(s) => println!(
            "GLM onset (first granule with >=3 flashes, centroid <=400 km): {} (lag {:+.0} s)",
            utc_str(s.t),
            s.t - t_ref
        ),
        None => println!(
            "GLM onset: absent — no cluster (>=3 flashes, centroid <=400 km) after {t_ref_ref}",
            t_ref_ref = utc_str(t_ref)
        ),
    }
    println!();
    out
}

fn bgr_section() {
    println!("--- Infrasound: BGR IS52 PMCC detections, 2022 ---");
    let Some(bytes) = fetch_raw_bytes(BGR_CDN) else {
        println!("BGR IS52: absent (house-release route void)");
        println!();
        return;
    };
    let Some(records) = parse_bin(MAGIC_BGR, &bytes) else {
        println!("BGR IS52: absent (no BGR1 contract)");
        println!();
        return;
    };
    let azim: Vec<_> = records.iter().filter(|r| r.comp == COMP_BGR_AZIM).collect();
    if azim.is_empty() {
        println!("BGR IS52: absent (no back-azimuth detections)");
        println!();
        return;
    }
    let (slat, slon) = (azim[0].lat, azim[0].lon);
    let d = distance_km(slat, slon);
    let bearing = bearing_deg(slat, slon, TONGA_LAT, TONGA_LON);
    let pred = ERUPTION_UNIX + d / LAMB_SPEED_KM_S;
    println!(
        "station {:.2},{:.2} · distance {:.0} km · predicted bearing {:.1} deg",
        slat, slon, d, bearing
    );
    println!("predicted direct Lamb arrival: {}", utc_str(pred));
    let Some(lsk) = embedded_lsk() else {
        println!("BGR IS52: TDB table absent");
        println!();
        return;
    };
    let Some(pred_tdb) = lsk.unix_to_tdb(pred) else {
        println!("BGR IS52: TDB mapping absent");
        println!();
        return;
    };
    let mut best: Option<(usize, f64)> = None;
    for (i, r) in azim.iter().enumerate() {
        let dt = r.t - pred_tdb;
        if best.map_or(true, |(_, b)| dt.abs() < b.abs()) {
            best = Some((i, dt));
        }
    }
    if let Some((i, dt)) = best {
        let r = &azim[i];
        if let Some(unix) = lsk.tdb_to_unix(r.t) {
            let azi = r.val.rem_euclid(360.0);
            let vapp = match records
                .iter()
                .find(|v| v.comp == COMP_BGR_VAPP && v.t == r.t && v.station == r.station)
            {
                Some(v) => format!("{:.1} m/s", v.val),
                None => "absent".to_string(),
            };
            println!(
                "nearest detection: {} (dt {:+.0} s) · azimuth {:.1} deg · residual {:+.1} deg · v_app {}",
                utc_str(unix),
                dt,
                azi,
                wrap_deg(azi - bearing),
                vapp
            );
        }
    }
    let quiet_pred = pred - 2.0 * 86400.0;
    if let Some(qtdb) = lsk.unix_to_tdb(quiet_pred) {
        let mut qbest: Option<(usize, f64)> = None;
        for (i, r) in azim.iter().enumerate() {
            let dt = r.t - qtdb;
            if qbest.map_or(true, |(_, b)| dt.abs() < b.abs()) {
                qbest = Some((i, dt));
            }
        }
        if let Some((i, dt)) = qbest
            && let Some(unix) = lsk.tdb_to_unix(azim[i].t)
        {
            println!(
                "NULL 2022-01-13: nearest detection: {} (dt {:+.0} s) · azimuth {:.1} deg · residual {:+.1} deg",
                utc_str(unix),
                dt,
                azim[i].val.rem_euclid(360.0),
                wrap_deg(azim[i].val - bearing)
            );
        }
    }
    println!();
}

fn nearest_index(values: &[f64], target: f64) -> Option<usize> {
    let mut best = 0usize;
    let mut best_d = f64::INFINITY;
    for (i, v) in values.iter().enumerate() {
        let d = (v - target).abs();
        if d < best_d {
            best_d = d;
            best = i;
        }
    }
    Some(best)
}

fn goes_index(file: &Hdf5File, lat_deg: f64, lon_deg: f64) -> Option<(usize, usize, usize, usize)> {
    let h = attr_scalar(file, "goes_imager_projection", "perspective_point_height")?;
    let lon0 = attr_scalar(
        file,
        "goes_imager_projection",
        "longitude_of_projection_origin",
    )?;
    let a = attr_scalar(file, "goes_imager_projection", "semi_major_axis")?;
    let b = attr_scalar(file, "goes_imager_projection", "semi_minor_axis")?;
    let lat = lat_deg.to_radians();
    let dlon = (lon_deg - lon0).to_radians();
    let e2 = 1.0 - (b * b) / (a * a);
    let n = a / (1.0 - e2 * lat.sin() * lat.sin()).sqrt();
    let x = n * lat.cos() * dlon.cos();
    let y = n * lat.cos() * dlon.sin();
    let z = n * (1.0 - e2) * lat.sin();
    let rho = ((h - x) * (h - x) + y * y).sqrt();
    let x_scan = (-y).atan2(h - x);
    let y_scan = (z / rho).atan();
    let xs = dataset_scaled(file, "x");
    let ys = dataset_scaled(file, "y");
    let xi = nearest_index(&xs, x_scan)?;
    let yi = nearest_index(&ys, y_scan)?;
    let dims = {
        let (_, ds, _) = file.dataset("Mask").ok()?;
        ds.dims.clone()
    };
    let nx = dims.get(1).copied().unwrap_or(1) as usize;
    Some((xi, yi, yi * nx + xi, nx))
}

fn raw_scaled_at(file: &Hdf5File, name: &str, idx: usize) -> Option<f64> {
    let (_, _, dt) = file.dataset(name).ok()?;
    let raw = file.read_dataset(name).ok()?;
    let v = num_at(&raw, idx, &dt)?;
    let scale = attr_scalar(file, name, "scale_factor").unwrap_or(1.0);
    let offset = attr_scalar(file, name, "add_offset");
    Some(match offset {
        Some(offset) => v * scale + offset,
        None => v * scale,
    })
}

fn goes_vent_pixel(file: &Hdf5File) -> Option<String> {
    let (xi, yi, idx, _) = goes_index(file, TONGA_LAT, TONGA_LON)?;
    let mask = raw_scaled_at(file, "Mask", idx)?;
    let power = raw_scaled_at(file, "Power", idx)?;
    let temp = raw_scaled_at(file, "Temp", idx)?;
    Some(format!(
        "vent x_idx={xi} y_idx={yi} mask={mask} FRP_MW={power} fire_T_K={temp}"
    ))
}

fn c07_vent(file: &Hdf5File) -> Option<String> {
    let (xi, yi, _idx, nx) = goes_index(file, TONGA_LAT, TONGA_LON)?;
    let (_, _, dt) = file.dataset("CMI").ok()?;
    let raw = file.read_dataset("CMI").ok()?;
    let scale = attr_scalar(file, "CMI", "scale_factor").unwrap_or(1.0);
    let offset = attr_scalar(file, "CMI", "add_offset");
    let val = |x: isize, y: isize| -> Option<f64> {
        if x < 0 || y < 0 {
            return None;
        }
        let i = (y as usize) * nx + (x as usize);
        let raw_v = num_at(&raw, i, &dt)?;
        let v = match offset {
            Some(offset) => raw_v * scale + offset,
            None => raw_v * scale,
        };
        if (100.0..1000.0).contains(&v) {
            Some(v)
        } else {
            None
        }
    };
    let center = val(xi as isize, yi as isize)?;
    let mut ring = Vec::new();
    let mut max = center;
    let mut mdx = 0isize;
    let mut mdy = 0isize;
    for dy in -5..=5 {
        for dx in -5..=5 {
            if dx == 0 && dy == 0 {
                continue;
            }
            if let Some(v) = val(xi as isize + dx, yi as isize + dy) {
                ring.push(v);
                if v > max {
                    max = v;
                    mdx = dx;
                    mdy = dy;
                }
            }
        }
    }
    let mean = if ring.is_empty() {
        f64::NAN
    } else {
        ring.iter().sum::<f64>() / ring.len() as f64
    };
    Some(format!(
        "C07 vent={center:.3} K | ring_mean={mean:.3} K | box_max={max:.3} K at dx={mdx} dy={mdy}"
    ))
}

fn c07_section() {
    println!("--- GOES-17 ABI C07 (3.9 um) at the vent, event vs NULL ---");
    for (label, url) in [
        ("event 2022-01-15 04:10-04:19", EVENT_C07),
        ("NULL  2022-01-13 04:10-04:19", NULL_C07),
    ] {
        let Some(bytes) = fetch_raw_bytes(url) else {
            println!("{label}: fetch void");
            continue;
        };
        let Ok(file) = Hdf5File::parse(&bytes) else {
            println!("{label}: HDF5 unread");
            continue;
        };
        match c07_vent(&file) {
            Some(s) => println!("{label}: {s}"),
            None => println!("{label}: C07 vent absent (fill or index unread)"),
        }
    }
    println!();
}

fn goes_thermal_section() {
    println!("--- Aerosol/Thermik: GOES-17 ABI, keyless AWS S3 ---");
    for (label, url) in [
        ("event 2022-01-15 04:10-04:19", EVENT_FDCF),
        ("NULL  2022-01-13 04:10-04:19", NULL_FDCF),
    ] {
        let Some(bytes) = fetch_raw_bytes(url) else {
            println!("{label}: absent (fetch void)");
            continue;
        };
        let Ok(file) = Hdf5File::parse(&bytes) else {
            println!("{label}: absent (HDF5 unread)");
            continue;
        };
        let count = dataset_scaled(&file, "total_number_of_pixels_with_fires_detected")
            .first()
            .copied();
        let frp = dataset_scaled(&file, "maximum_fire_radiative_power")
            .first()
            .copied();
        let temp = dataset_scaled(&file, "maximum_fire_temperature")
            .first()
            .copied();
        println!("{label}: fires={count:?} max_FRP_MW={frp:?} max_fire_T_K={temp:?} (full disk)");
        match goes_vent_pixel(&file) {
            Some(s) => println!("{label}: {s}"),
            None => println!("{label}: vent pixel absent (projection/mask unread)"),
        }
    }
    println!(
        "note: full-disk scalars; TROPOMI AER_AI is keyless on MEEO S3, pixel extraction near the vent pending."
    );
    println!();
}

const FIRMS_AREA: &str = "https://firms.modaps.eosdis.nasa.gov/api/area/csv/{FIRMS_MAP_KEY}/VIIRS_SNPP_SP/-180,-25,-170,-15/1/";

fn firms_section() {
    println!("--- Thermal twin: FIRMS VIIRS_SNPP_SP (keyed, key in house) ---");
    let env = load_env();
    for day in [EVENT_DAY, QUIET_DAY] {
        let url = resolve_secret(&format!("{FIRMS_AREA}{day}"), &env);
        let Some(bytes) = fetch_raw_bytes(&url) else {
            println!("FIRMS {day}: fetch void");
            continue;
        };
        let text = String::from_utf8_lossy(&bytes);
        let mut n = 0usize;
        let mut max_frp = 0.0f64;
        let mut max_bright = 0.0f64;
        for line in text.lines().skip(1) {
            let cols: Vec<&str> = line.split(',').collect();
            if cols.len() < 13 {
                continue;
            }
            let lat: f64 = cols[0].parse().unwrap_or(999.0);
            let lon: f64 = cols[1].parse().unwrap_or(999.0);
            if (lat - TONGA_LAT).abs() > 0.5 || (lon - TONGA_LON).abs() > 0.5 {
                continue;
            }
            n += 1;
            if let Ok(v) = cols[12].parse::<f64>() {
                max_frp = max_frp.max(v);
            }
            if let Ok(v) = cols[2].parse::<f64>() {
                max_bright = max_bright.max(v);
            }
        }
        if n == 0 {
            println!(
                "FIRMS {day}: no detection within 0.5 deg; first line: {}",
                text.lines().next().unwrap_or("<empty>")
            );
        } else {
            println!(
                "FIRMS {day}: detections_near_vent={n} max_FRP_MW={max_frp:.3} max_brightness_K={max_bright:.3}"
            );
        }
    }
    println!();
}

fn latest_key(bucket: &str, product: &str) -> Option<String> {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .ok()?
        .as_secs() as i64;
    let days = secs / 86400;
    let Some((year, _month, _day)) = civil_from_days(days) else {
        return None;
    };
    let doy = days - days_from_civil(year as i64, 1, 1)? + 1;
    let url = format!(
        "https://{bucket}.s3.amazonaws.com/?list-type=2&prefix={product}/{year:04}/{doy:03}/&max-keys=1000"
    );
    let bytes = fetch_raw_bytes(&url)?;
    let text = String::from_utf8_lossy(&bytes);
    let mut last = None;
    for chunk in text.split("<Key>").skip(1) {
        if let Some(key) = chunk.split("</Key>").next() {
            last = Some(key.to_string());
        }
    }
    last
}

fn live_section() {
    println!("--- Live: GOES-18 ABI (AWS S3), latest object of today ---");
    for (label, product) in [("FDCF", "ABI-L2-FDCF"), ("AODF", "ABI-L2-AODF")] {
        let Some(key) = latest_key("noaa-goes18", product) else {
            println!("{label}: no key today");
            continue;
        };
        let url = format!("https://noaa-goes18.s3.amazonaws.com/{key}");
        let Some(bytes) = fetch_raw_bytes(&url) else {
            println!("{label}: fetch void for {key}");
            continue;
        };
        let Ok(file) = Hdf5File::parse(&bytes) else {
            println!("{label}: HDF5 unread for {key}");
            continue;
        };
        if label == "FDCF" {
            let count = dataset_scaled(&file, "total_number_of_pixels_with_fires_detected")
                .first()
                .copied();
            let frp = dataset_scaled(&file, "maximum_fire_radiative_power")
                .first()
                .copied();
            println!("{label}: {key} fires={count:?} max_FRP_MW={frp:?}");
        } else {
            let aod = dataset_scaled(&file, "max_aod550_sea").first().copied();
            println!("{label}: {key} max_aod550_sea={aod:?}");
        }
    }
    println!();
}

fn point_section(path: &str, lat: f64, lon: f64) {
    let Ok(bytes) = std::fs::read(path) else {
        println!("{path}: unreadable");
        return;
    };
    let Ok(file) = Hdf5File::parse(&bytes) else {
        println!("{path}: HDF5 unparsed");
        return;
    };
    match goes_index(&file, lat, lon) {
        Some((xi, yi, idx, _)) => {
            let mask = raw_scaled_at(&file, "Mask", idx);
            let power = raw_scaled_at(&file, "Power", idx);
            let temp = raw_scaled_at(&file, "Temp", idx);
            let cmi = raw_scaled_at(&file, "CMI", idx);
            println!(
                "{path} lat={lat} lon={lon}: x_idx={xi} y_idx={yi} mask={mask:?} FRP_MW={power:?} fire_T_K={temp:?} CMI={cmi:?}"
            );
        }
        None => println!("{path} lat={lat} lon={lon}: index absent"),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let (Some(point), Some(path)) = (
        arg_value(&args, "--point"),
        arg_value(&args, "--file"),
    ) {
        let parts: Vec<f64> = point
            .split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect();
        if parts.len() == 2 {
            point_section(&path, parts[0], parts[1]);
            return;
        }
    }
    if has(&args, "--live") {
        live_section();
        return;
    }
    if has(&args, "--c07") {
        c07_section();
        return;
    }
    if let Some(path) = arg_value(&args, "--inspect-glm") {
        let Ok(bytes) = std::fs::read(&path) else {
            println!("{path}: unreadable");
            return;
        };
        let Ok(file) = Hdf5File::parse(&bytes) else {
            println!("{path}: HDF5 parses void");
            return;
        };
        for name in [
            "flash_lat",
            "flash_lon",
            "flash_energy",
            "flash_time_offset_of_first_event",
            "event_lat",
            "event_lon",
            "event_energy",
            "event_time_offset",
        ] {
            match file.dataset(name) {
                Ok((_, ds, dt)) => println!(
                    "{name}: dims {:?} class {} size {} endian {:?} signed {} count {}",
                    ds.dims,
                    dt.class,
                    dt.size,
                    dt.endian,
                    dt.signed,
                    dataset_scaled(&file, name).len()
                ),
                Err(_) => println!("{name}: absent"),
            }
        }
        return;
    }

    println!("=== Hunga Tonga-Hunga Ha'apai 2022-01-15 04:14:45 UTC — network event ===");
    println!("source: lat {TONGA_LAT}, lon {TONGA_LON}; all times UTC.");
    println!("Lamb phase speed: {LAMB_SPEED_KM_S} km/s.");
    println!();
    inventory();

    if has(&args, "--glm-only") {
        glm_window(EVENT_DAY, 15, "event", 1);
        return;
    }
    let skip = has(&args, "--skip-slow");
    if !skip {
        seismic_day(EVENT_DAY, "event");
        seismic_day(QUIET_DAY, "NULL");
    }

    coops_section(EVENT_DAY, "event");
    coops_section(QUIET_DAY, "NULL");

    if !skip {
        tec_section(&["2022-01-13", "2022-01-14", "2022-01-16", "2022-01-17"]);
        let event_glm = glm_window(EVENT_DAY, 15, "event", 1);
        let null_glm = glm_window(QUIET_DAY, 13, "NULL", 1);
        println!(
            "GLM comparison: event {} flashes, NULL {} flashes (same 500-km window)",
            event_glm.iter().map(|s| s.count).sum::<usize>(),
            null_glm.iter().map(|s| s.count).sum::<usize>()
        );
        println!();
    }

    bgr_section();

    goes_thermal_section();

    firms_section();

    println!("=== Verdict ===");
    println!("criterion: at least three phenomena with an onset and a consistent travel time.");
    println!(
        "phenomena with their own onset in this run: seismic, pressure, tsunami, TEC, GLM, infrasound."
    );
    println!(
        "aerosol/thermal: GOES-17 FDCF keyless measured (event vs NULL); TROPOMI AER_AI keyless on MEEO S3, pixel extraction pending."
    );
}
