use omegaflow::archivar::{
    BodyEphemeris, J2000_EPOCH, body_barycenter_position, parse_ephemeris_binary,
};
use omegaflow::doppler::parse_bin;
use std::collections::HashMap;

const JD_UNIX_EPOCH: f64 = 2440587.5;
const DAY: f64 = 86400.0;
const PIONEER_ANOMALY: f64 = 8.74e-10;

fn load_eph(name: &str, eph: &mut HashMap<String, BodyEphemeris>) -> bool {
    let path = format!("data/ephemeris_{name}.bin");
    match std::fs::read(&path)
        .ok()
        .and_then(|d| parse_ephemeris_binary(&d))
    {
        Some(e) => {
            eph.insert(name.to_string(), e);
            true
        }
        None => {
            eprintln!("{name}: ephemeris bin void ({path})");
            false
        }
    }
}

fn daily_median(records: &[[f64; 6]]) -> HashMap<i64, f64> {
    let mut groups: HashMap<i64, Vec<f64>> = HashMap::new();
    for r in records {
        let jd = J2000_EPOCH + r[0] / DAY;
        let day = (jd - JD_UNIX_EPOCH).floor() as i64;
        if r[1].is_finite() && r[1] > 0.0 {
            groups.entry(day).or_default().push(r[1]);
        }
    }
    let mut out = HashMap::new();
    for (day, mut vals) in groups {
        vals.sort_by(f64::total_cmp);
        out.insert(day, vals[vals.len() / 2]);
    }
    out
}

fn lin_fit(xs: &[f64], ys: &[f64]) -> (f64, f64) {
    let n = xs.len() as f64;
    let sx: f64 = xs.iter().sum();
    let sy: f64 = ys.iter().sum();
    let sxx: f64 = xs.iter().map(|x| x * x).sum();
    let sxy: f64 = xs.iter().zip(ys).map(|(x, y)| x * y).sum();
    let denom = n * sxx - sx * sx;
    if denom.abs() < 1e-300 {
        return (0.0, 0.0);
    }
    let a = (n * sxy - sx * sy) / denom;
    let b = (sy - a * sx) / n;
    (a, b)
}

fn run(name: &str, eph: &HashMap<String, BodyEphemeris>, clean: bool) {
    let doppler_path = if clean {
        format!("data/{name}_doppler_clean.bin")
    } else {
        format!("data/{name}_doppler.bin")
    };
    let Ok(bytes) = std::fs::read(&doppler_path) else {
        eprintln!("{name}: doppler bin void ({doppler_path})");
        return;
    };
    let Some(records) = parse_bin(&bytes) else {
        eprintln!("{name}: doppler bin parse void");
        return;
    };
    let sc_key = format!("{name}_daily");
    if !eph.contains_key(&sc_key) || !eph.contains_key("earth") {
        eprintln!("{name}: daily/earth bin absent");
        return;
    }
    let obs = daily_median(&records);
    let mut rows: Vec<(i64, f64, f64)> = Vec::new();
    for (&day, &o) in &obs {
        let tdb = (day as f64 + JD_UNIX_EPOCH - J2000_EPOCH) * DAY;
        let (Some(rs), Some(re)) = (
            body_barycenter_position(&sc_key, tdb, eph),
            body_barycenter_position("earth", tdb, eph),
        ) else {
            continue;
        };
        let dx = rs[0] - re[0];
        let dy = rs[1] - re[1];
        let dz = rs[2] - re[2];
        let range = (dx * dx + dy * dy + dz * dz).sqrt();
        rows.push((day, o, range));
    }
    rows.sort_by_key(|r| r.0);
    if rows.len() < 30 {
        eprintln!("{name}: {} shared days — too short", rows.len());
        return;
    }
    let mut days: Vec<i64> = Vec::new();
    let mut rdot: Vec<f64> = Vec::new();
    let mut obs_v: Vec<f64> = Vec::new();
    for i in 1..rows.len() - 1 {
        if rows[i].0 - rows[i - 1].0 != 1 || rows[i + 1].0 - rows[i].0 != 1 {
            continue;
        }
        let rdot_i = (rows[i + 1].2 - rows[i - 1].2) / (2.0 * DAY);
        days.push(rows[i].0);
        rdot.push(rdot_i);
        obs_v.push(rows[i].1);
    }
    if days.len() < 30 {
        eprintln!("{name}: {} dense days — too short", days.len());
        return;
    }
    let (a, b) = lin_fit(&rdot, &obs_v);
    let mut resid: Vec<f64> = Vec::new();
    let mut resid_rms = 0.0f64;
    for i in 0..days.len() {
        let r = obs_v[i] - a * rdot[i] - b;
        resid.push(r);
        resid_rms += r * r;
    }
    resid_rms = (resid_rms / days.len() as f64).sqrt();
    let day_f: Vec<f64> = days.iter().map(|&d| d as f64).collect();
    let (drift_per_day, _) = lin_fit(&day_f, &resid);
    let drift_hz_s = drift_per_day / DAY;
    let accel = drift_hz_s / a;
    let rdot_min = rdot.iter().cloned().fold(f64::INFINITY, f64::min);
    let rdot_max = rdot.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let signal_span = (a * rdot_max - a * rdot_min).abs();
    let fits = resid_rms < 0.1 * signal_span;
    let span = |d: i64| -> String {
        match omegaflow::spectral::civil_from_days(d) {
            Some((y, m, dd)) => format!("{y:04}-{m:02}-{dd:02}"),
            None => format!("day {d}"),
        }
    };
    eprintln!(
        "{}: {} dense days ({}..{}) — OBSVBL = A·ṙ + B, A {:.4e} Hz/(m/s), B {:.4e} Hz, signal span {:.3e} Hz, residual RMS {:.3e} Hz",
        name,
        days.len(),
        span(days[0]),
        span(days[days.len() - 1]),
        a,
        b,
        signal_span,
        resid_rms
    );
    if fits {
        eprintln!(
            "  residual drift {drift_hz_s:.4e} Hz/s → acceleration {accel:.4e} m/s² (Pioneer anomaly {PIONEER_ANOMALY:.3e} m/s², sunward = negative)"
        );
    } else {
        eprintln!(
            "  the barycentric first model does not carry the series (residual RMS {:.1e} Hz ≥ 0,1·signal {:.1e} Hz) — the reduction needs the full observation model (uplink ramp, station, Moyer); the drift would be an artifact",
            resid_rms, signal_span
        );
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let clean = args.iter().any(|a| a == "--clean");
    let mut eph: HashMap<String, BodyEphemeris> = HashMap::new();
    for body in ["earth", "pioneer10_daily", "pioneer11_daily"] {
        load_eph(body, &mut eph);
    }
    for name in ["pioneer10", "pioneer11"] {
        run(name, &eph, clean);
    }
}
