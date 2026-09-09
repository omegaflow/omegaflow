use std::collections::HashMap;

use omegaflow::archivar::{
    body_barycenter_position, embedded_lsk, fetch_raw_bytes, parse_ephemeris_binary, BodyEphemeris,
};
use omegaflow::cdn::CDN_BASE;

const UNIX_JD_OFFSET: f64 = 2440587.5;
const BIN_TTL_S: u64 = 604800;
const TAU: f64 = std::f64::consts::TAU;

const MOON_PERIODS: [(&str, f64); 5] = [
    ("miranda", 1.413),
    ("ariel", 2.520),
    ("umbriel", 4.144),
    ("titania", 8.706),
    ("oberon", 13.463),
];

fn ensure_bin(path: &str, netloc: &str, asset: &str, ttl: u64) -> Option<Vec<u8>> {
    if let Ok(bytes) = std::fs::read(path) {
        return Some(bytes);
    }
    if !path.starts_with("data/") {
        return None;
    }
    let url = format!("{}/{}/{}", CDN_BASE, netloc, asset);
    let bytes = fetch_raw_bytes(&url, ttl)?;
    if let Some(parent) = std::path::Path::new(path).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if std::fs::write(path, &bytes).is_err() {
        return None;
    }
    Some(bytes)
}

fn load(
    netloc: &str,
    asset: &str,
    name: &str,
    eph_dir: &str,
) -> Option<HashMap<String, BodyEphemeris>> {
    let path = format!("{eph_dir}/{netloc}/{asset}");
    let bytes = ensure_bin(&path, netloc, asset, BIN_TTL_S)?;
    let eph = parse_ephemeris_binary(&bytes)?;
    let mut map = HashMap::new();
    map.insert(name.to_string(), eph);
    Some(map)
}

fn arg_token(args: &[String], key: &str) -> Option<String> {
    let pos = args.iter().position(|a| a == key)?;
    args.get(pos + 1).cloned()
}

fn parse_f64s(args: &[String], key: &str, count: usize) -> Option<Vec<f64>> {
    let pos = args.iter().position(|a| a == key)?;
    let mut v = Vec::new();
    for i in (pos + 1)..(pos + 1 + count) {
        match args.get(i).and_then(|s| s.parse::<f64>().ok()) {
            Some(x) if x.is_finite() => v.push(x),
            _ => return None,
        }
    }
    if v.len() == count {
        Some(v)
    } else {
        None
    }
}

fn vec_sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn peak_power(series: &[(f64, f64)], period: f64) -> Option<(f64, f64)> {
    if series.len() < 8 {
        return None;
    }
    let n = series.len() as f64;
    let mean = series.iter().map(|&(_, y)| y).sum::<f64>() / n;
    let omega = TAU / period;
    let (mut scc, mut sss, mut scs, mut syc, mut sys) = (0.0, 0.0, 0.0, 0.0, 0.0);
    for &(t, y) in series {
        let c = (omega * t).cos();
        let s = (omega * t).sin();
        let yp = y - mean;
        scc += c * c;
        sss += s * s;
        scs += c * s;
        syc += yp * c;
        sys += yp * s;
    }
    let det = scc * sss - scs * scs;
    if !(det.is_finite() && det.abs() > 1e-300) {
        return None;
    }
    let a = (sss * syc - scs * sys) / det;
    let b = (scc * sys - scs * syc) / det;
    let amp = (a * a + b * b).sqrt();
    if amp.is_finite() {
        Some((amp, mean))
    } else {
        None
    }
}

fn periodogram(series: &[(f64, f64)], p_min: f64, p_max: f64, n_freq: usize) -> Vec<(f64, f64)> {
    let f_min = 1.0 / p_max;
    let f_max = 1.0 / p_min;
    let df = (f_max - f_min) / (n_freq - 1) as f64;
    let mut amps: Vec<Option<f64>> = Vec::with_capacity(n_freq);
    for i in 0..n_freq {
        let f = f_min + df * i as f64;
        let period = 1.0 / f;
        amps.push(peak_power(series, period).map(|(a, _)| a));
    }
    let mut peaks: Vec<(f64, f64)> = Vec::new();
    for i in 1..(amps.len() - 1) {
        let (Some(lo), Some(mid), Some(hi)) = (amps[i - 1], amps[i], amps[i + 1]) else {
            continue;
        };
        if mid <= lo || mid <= hi {
            continue;
        }
        let l2 = lo * lo;
        let m2 = mid * mid;
        let h2 = hi * hi;
        let denom = l2 - 2.0 * m2 + h2;
        let (f_peak, amp2) = if denom.abs() > 1e-300 {
            let d = 0.5 * (l2 - h2) / denom;
            (f_min + df * (i as f64 + d), m2 - 0.25 * (l2 - h2) * d)
        } else {
            (f_min + df * i as f64, m2)
        };
        if f_peak > 0.0 && amp2 > 0.0 {
            peaks.push((1.0 / f_peak, amp2.sqrt()));
        }
    }
    peaks.sort_by(|a, b| b.1.total_cmp(&a.1));
    peaks
}

fn report_peaks(out: &mut String, label: &str, series: &[(f64, f64)], p_min: f64, p_max: f64) {
    let peaks = periodogram(series, p_min, p_max, 6000);
    out.push_str(&format!("  {label}:\n"));
    let mut shown = 0;
    for &(p, a) in &peaks {
        if a < 0.05 {
            continue;
        }
        if shown >= 8 {
            break;
        }
        let moon = match MOON_PERIODS
            .iter()
            .find(|&&(_, mp)| (p - mp).abs() / mp < 0.02)
        {
            Some(&(name, _)) => format!(" (~{name})"),
            None => String::new(),
        };
        out.push_str(&format!(
            "    period {:6.3} d, amplitude {:6.2} m{}\n",
            p, a, moon
        ));
        shown += 1;
    }
    if shown == 0 {
        out.push_str("    no peak above 0.05 m\n");
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    println!("Uranus wobble period — the periodicity of the planet-center − system-barycenter offset (DE441).");

    let eph_dir = arg_token(&args, "--eph-dir").unwrap_or("data".to_string());
    let report_dir = arg_token(&args, "--report-dir").unwrap_or("state/reports".to_string());
    let jd0 = arg_token(&args, "--jd0")
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(2451545.0);
    let window_d = arg_token(&args, "--window-d")
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(365.0);
    let step_d = arg_token(&args, "--step-d")
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(0.05);
    let p_min = arg_token(&args, "--p-min")
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(0.4);
    let p_max = arg_token(&args, "--p-max")
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(40.0);

    let Some(lsk) = embedded_lsk() else {
        eprintln!("uranus-wobble: the embedded LSK carries no naif0012 table — the TDB axis stays unconverted");
        return;
    };

    let center = load(
        "ssd.jpl.nasa.gov",
        "ephemeris_uranus_c.bin",
        "uranus_c",
        &eph_dir,
    );
    let de = load(
        "ssd.jpl.nasa.gov",
        "ephemeris_uranus.bin",
        "uranus",
        &eph_dir,
    );

    let mut report = String::new();
    report.push_str("Uranus wobble period — DE441 planet-center − system-barycenter offset\n\n");

    let synthetic: Option<Vec<(f64, f64)>> = {
        let vals = parse_f64s(&args, "--calibrate-synthetic", 4);
        vals.map(|v| {
            let (p1, a1, p2, a2) = (v[0], v[1], v[2], v[3]);
            let n = (window_d / step_d) as usize;
            let mut series = Vec::with_capacity(n);
            for i in 0..n {
                let t = jd0 + i as f64 * step_d;
                let y = a1 * (TAU * t / p1).sin() + a2 * (TAU * t / p2).sin();
                series.push((t, y));
            }
            report.push_str(&format!(
                "Synthetic series (--calibrate-synthetic): {:.3} d / {:.2} m + {:.3} d / {:.2} m\n\n",
                p1, a1, p2, a2
            ));
            series
        })
    };

    let (wob_x, wob_y, wob_z, wob_mag): (
        Vec<(f64, f64)>,
        Vec<(f64, f64)>,
        Vec<(f64, f64)>,
        Vec<(f64, f64)>,
    ) = match (&synthetic, &center, &de) {
        (Some(s), _, _) => {
            let x: Vec<(f64, f64)> = s.iter().map(|&(t, y)| (t, y)).collect();
            (x.clone(), x.clone(), x.clone(), x)
        }
        (None, Some(cm), Some(dm)) => {
            let mut x = Vec::new();
            let mut y = Vec::new();
            let mut z = Vec::new();
            let mut m = Vec::new();
            let n = (window_d / step_d) as usize;
            for i in 0..n {
                let jd = jd0 + i as f64 * step_d;
                let unix = (jd - UNIX_JD_OFFSET) * 86400.0;
                let Some(tdb) = lsk.unix_to_tdb(unix) else {
                    continue;
                };
                let (Some(c), Some(b)) = (
                    body_barycenter_position("uranus_c", tdb, cm),
                    body_barycenter_position("uranus", tdb, dm),
                ) else {
                    continue;
                };
                let w = vec_sub(c, b);
                let mag = (w[0] * w[0] + w[1] * w[1] + w[2] * w[2]).sqrt();
                x.push((jd, w[0]));
                y.push((jd, w[1]));
                z.push((jd, w[2]));
                m.push((jd, mag));
            }
            report.push_str(&format!(
                "Sampled {} epochs over JD {:.2}..{:.2} (step {:.2} d):\n\n",
                m.len(),
                jd0,
                jd0 + window_d,
                step_d
            ));
            let mean = m.iter().map(|&(_, v)| v).sum::<f64>() / m.len() as f64;
            let max = m.iter().map(|&(_, v)| v).fold(0.0, f64::max);
            report.push_str(&format!(
                "  wobble magnitude: mean {:.3} m, max {:.3} m\n\n",
                mean, max
            ));
            (x, y, z, m)
        }
        _ => {
            eprintln!("uranus-wobble: ephemeris_uranus_c.bin or ephemeris_uranus.bin absent");
            return;
        }
    };

    report.push_str("Dominant periods (least-squares periodogram):\n");
    report_peaks(&mut report, "magnitude |c−b|", &wob_mag, p_min, p_max);
    report_peaks(&mut report, "x component", &wob_x, p_min, p_max);
    report_peaks(&mut report, "y component", &wob_y, p_min, p_max);
    report_peaks(&mut report, "z component", &wob_z, p_min, p_max);

    report.push_str("\nReference (Uranus moon orbital periods, d):");
    for &(name, p) in &MOON_PERIODS {
        report.push_str(&format!(" {name} {p}"));
    }
    report.push('\n');

    print!("{report}");
    if let Err(e) = std::fs::create_dir_all(&report_dir) {
        eprintln!("uranus-wobble: report dir {report_dir} stays unbuilt — {e}");
    }
    let report_path = format!("{report_dir}/uranus_wobble_period.txt");
    if let Err(e) = std::fs::write(&report_path, &report) {
        eprintln!("uranus-wobble: report write void — {e}");
    }
    let series_path = format!("{report_dir}/uranus_wobble_period_series.tsv");
    let mut series = String::new();
    series.push_str("jd\twx_m\twy_m\twz_m\twmag_m\n");
    for i in 0..wob_mag.len() {
        series.push_str(&format!(
            "{:.8}\t{:.3}\t{:.3}\t{:.3}\t{:.3}\n",
            wob_mag[i].0, wob_x[i].1, wob_y[i].1, wob_z[i].1, wob_mag[i].1
        ));
    }
    if let Err(e) = std::fs::write(&series_path, &series) {
        eprintln!("uranus-wobble: series write void — {e}");
    }
    println!("series: {series_path}");
    println!("report: {report_path}");
}
