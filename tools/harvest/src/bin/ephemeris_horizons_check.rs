use std::collections::HashMap;

use omegaflow::archivar::J2000_EPOCH;
use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::motion::{
    BodyEphemeris, body_barycenter_position, parse_ephemeris_binary,
};

const CDN_BASE: &str = "https://github.com/omegaflow/sources/releases/download/ssd.jpl.nasa.gov";
const HORIZONS_API: &str = "https://ssd.jpl.nasa.gov/api/horizons.api";

const BODIES: [(&str, &str); 8] = [
    ("mercury", "199"),
    ("venus", "299"),
    ("earth", "399"),
    ("mars", "499"),
    ("jupiter", "599"),
    ("saturn", "699"),
    ("uranus", "799"),
    ("neptune", "899"),
];

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn load_ephemeris(name: &str, args: &[String]) -> Option<BodyEphemeris> {
    let bytes = match arg_value(args, "--ephem") {
        Some(p) => std::fs::read(&p).ok(),
        None => {
            let url = format!("{}/ephemeris_{}.bin", CDN_BASE, name);
            fetch_raw_bytes(&url, 3600)
                .or_else(|| std::fs::read(format!("ephemeris_{}.bin", name)).ok())
        }
    };
    bytes.and_then(|b| parse_ephemeris_binary(&b))
}

fn horizons_vectors(
    command: &str,
    start_jd: f64,
    stop_jd: f64,
    step_days: f64,
) -> Option<Vec<(f64, [f64; 3])>> {
    let url = format!(
        "{}?format=text&COMMAND='{}'&CENTER='500@0'&MAKE_EPHEM='YES'&EPHEM_TYPE='VECTORS'\
         &START_TIME='JD{}'&STOP_TIME='JD{}'&STEP_SIZE='{}d'\
         &REF_PLANE='FRAME'&VEC_TABLE='2'&OUT_UNITS='KM-S'&CSV_FORMAT='YES'",
        HORIZONS_API, command, start_jd, stop_jd, step_days
    );
    let bytes = fetch_raw_bytes(&url, 1800)?;
    let text = String::from_utf8_lossy(&bytes);
    let mut out = Vec::new();
    let mut in_data = false;
    for line in text.lines() {
        if line.starts_with("$$SOE") {
            in_data = true;
            continue;
        }
        if line.starts_with("$$EOE") {
            break;
        }
        if !in_data {
            continue;
        }
        let cols: Vec<&str> = line.split(',').collect();
        if cols.len() < 8 {
            continue;
        }
        let jd: f64 = cols[0].trim().parse().ok().unwrap_or(f64::NAN);
        let x: f64 = cols[2].trim().parse().ok().unwrap_or(f64::NAN);
        let y: f64 = cols[3].trim().parse().ok().unwrap_or(f64::NAN);
        let z: f64 = cols[4].trim().parse().ok().unwrap_or(f64::NAN);
        if ![jd, x, y, z].iter().all(|v| v.is_finite()) {
            continue;
        }
        out.push((jd, [x * 1000.0, y * 1000.0, z * 1000.0]));
    }
    if out.is_empty() { None } else { Some(out) }
}

fn check(name: &str, command: &str, args: &[String]) {
    let Some(eph) = load_ephemeris(name, args) else {
        eprintln!("{name}: ephemeris_{name}.bin absent or carries no CF-86 contract");
        return;
    };
    let mut map: HashMap<String, BodyEphemeris> = HashMap::new();
    map.insert(name.to_string(), eph);

    let start_jd: f64 = arg_value(args, "--window-start")
        .and_then(|v| v.parse().ok())
        .unwrap_or(J2000_EPOCH);
    let years: f64 = arg_value(args, "--window-years")
        .and_then(|v| v.parse().ok())
        .unwrap_or(30.0);
    let stop_jd = start_jd + years * 365.25;
    let step_days: f64 = arg_value(args, "--step-days")
        .and_then(|v| v.parse().ok())
        .unwrap_or(10.0);

    let Some(horizons) = horizons_vectors(command, start_jd, stop_jd, step_days) else {
        eprintln!("{name}: Horizons request void");
        return;
    };

    let mut uncovered = 0usize;
    let mut res: Vec<f64> = Vec::with_capacity(horizons.len());
    let mut ang: Vec<f64> = Vec::with_capacity(horizons.len());
    for (jd, hp) in &horizons {
        let tdb = (*jd - J2000_EPOCH) * 86400.0;
        let Some(sp) = body_barycenter_position(name, tdb, &map) else {
            uncovered += 1;
            continue;
        };
        let dx = sp[0] - hp[0];
        let dy = sp[1] - hp[1];
        let dz = sp[2] - hp[2];
        let dist = (dx * dx + dy * dy + dz * dz).sqrt();
        let r = (hp[0] * hp[0] + hp[1] * hp[1] + hp[2] * hp[2]).sqrt();
        res.push(dist);
        if r > 0.0 {
            ang.push(dist / r * 180.0 / std::f64::consts::PI * 3600.0);
        }
    }
    if res.is_empty() {
        eprintln!(
            "{name}: {} Horizon epochs, {} uncovered — no residual (0 honored)",
            horizons.len(),
            uncovered
        );
        return;
    }
    let n = res.len();
    let mean = res.iter().sum::<f64>() / n as f64;
    let rms = (res.iter().map(|r| r * r).sum::<f64>() / n as f64).sqrt();
    let max = res.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let min = res.iter().cloned().fold(f64::INFINITY, f64::min);
    let mut sorted = res.clone();
    sorted.sort_by(|a, b| a.total_cmp(b));
    let median = sorted[n / 2];
    let ang_mean = ang.iter().sum::<f64>() / ang.len() as f64;
    let ang_max = ang.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    println!(
        "{name}: {} epochs (JD {:.1}..{:.1}, {} d step), {} uncovered — residual |Chebyshev − Horizons|: {:.3}..{:.3} m (median {:.3}, mean {:.3}, RMS {:.3}); angle {:.4}″ mean, {:.4}″ max",
        n, start_jd, stop_jd, step_days, uncovered, min, max, median, mean, rms, ang_mean, ang_max
    );
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let bodies: Vec<(&str, &str)> = match arg_value(&args, "--body") {
        Some(name) => BODIES.iter().filter(|(n, _)| *n == name).copied().collect(),
        None => BODIES.to_vec(),
    };
    if bodies.is_empty() {
        eprintln!(
            "unknown body — know {}",
            BODIES
                .iter()
                .map(|(n, _)| *n)
                .collect::<Vec<_>>()
                .join(", ")
        );
        return;
    }
    for (name, command) in bodies {
        check(name, command, &args);
    }
}
