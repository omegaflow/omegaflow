use omegaflow::archivar::{
    BodyEphemeris, J2000_EPOCH, body_barycenter_position, body_barycenter_velocity,
    parse_ephemeris_binary,
};
use omegaflow::atdf::parse_bin;
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

fn daily_median(records: &[[f64; 14]]) -> HashMap<i64, f64> {
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

fn span_date(d: i64) -> String {
    match omegaflow::spectral::civil_from_days(d) {
        Some((y, m, dd)) => format!("{y:04}-{m:02}-{dd:02}"),
        None => format!("day {d}"),
    }
}

fn run(eph: &HashMap<String, BodyEphemeris>) {
    let path = "data/pioneer10_skyfreq.bin";
    let Ok(bytes) = std::fs::read(path) else {
        eprintln!("pioneer10: skyfreq bin void ({path})");
        return;
    };
    let Some(records) = parse_bin(&bytes) else {
        eprintln!("pioneer10: skyfreq bin parse void");
        return;
    };
    if !eph.contains_key("pioneer10_daily") || !eph.contains_key("earth") {
        eprintln!("pioneer10: daily/earth bin absent");
        return;
    }
    let obs = daily_median(&records);
    let mut rows: Vec<(i64, f64, f64)> = Vec::new();
    for (&day, &o) in &obs {
        let tdb = (day as f64 + JD_UNIX_EPOCH - J2000_EPOCH) * DAY;
        let (Some(rs), Some(re), Some(vs), Some(ve)) = (
            body_barycenter_position("pioneer10_daily", tdb, eph),
            body_barycenter_position("earth", tdb, eph),
            body_barycenter_velocity("pioneer10_daily", tdb, eph),
            body_barycenter_velocity("earth", tdb, eph),
        ) else {
            continue;
        };
        let mut dx = 0.0f64;
        let mut dr = 0.0f64;
        for k in 0..3 {
            let d = rs[k] - re[k];
            dx += d * d;
            let dv = vs[k] - ve[k];
            dr += d * dv;
        }
        let range = dx.sqrt();
        if range <= 0.0 {
            continue;
        }
        rows.push((day, o, dr / range));
    }
    rows.sort_by_key(|r| r.0);
    if rows.len() < 30 {
        eprintln!("pioneer10: {} shared days — too short", rows.len());
        return;
    }
    let mut rdot: Vec<f64> = Vec::new();
    let mut obs_v: Vec<f64> = Vec::new();
    for (_, o, rd) in &rows {
        rdot.push(*rd);
        obs_v.push(*o);
    }
    let (a, b) = lin_fit(&rdot, &obs_v);
    let mut resid: Vec<f64> = Vec::new();
    let mut resid_rms = 0.0f64;
    for i in 0..rdot.len() {
        let r = obs_v[i] - a * rdot[i] - b;
        resid.push(r);
        resid_rms += r * r;
    }
    resid_rms = (resid_rms / rdot.len() as f64).sqrt();
    let day_f: Vec<f64> = rows.iter().map(|r| r.0 as f64).collect();
    let (drift_per_day, _) = lin_fit(&day_f, &resid);
    let n = day_f.len() as f64;
    let day_mean = day_f.iter().sum::<f64>() / n;
    let sxx_days: f64 = day_f.iter().map(|d| (d - day_mean).powi(2)).sum();
    let rss: f64 = resid.iter().map(|r| r * r).sum();
    let se_drift_per_day = if n > 2.0 && sxx_days > 0.0 {
        (rss / ((n - 2.0) * sxx_days)).sqrt()
    } else {
        f64::NAN
    };
    let drift_hz_s = drift_per_day / DAY;
    let se_hz_s = se_drift_per_day / DAY;
    let accel = drift_hz_s / a;
    let se_accel = se_hz_s / a.abs();
    let rdot_min = rdot.iter().cloned().fold(f64::INFINITY, f64::min);
    let rdot_max = rdot.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let signal_span = (a * rdot_max - a * rdot_min).abs();
    let fits = resid_rms < 0.1 * signal_span;
    eprintln!(
        "pioneer10: {} days ({}..{}) — fsky = A·ṙ + B, A {:.4e} Hz/(m/s), B {:.4e} Hz, signal span {:.3e} Hz, residual RMS {:.3e} Hz ({:.1}% of the span)",
        rows.len(),
        span_date(rows[0].0),
        span_date(rows[rows.len() - 1].0),
        a,
        b,
        signal_span,
        resid_rms,
        100.0 * resid_rms / signal_span
    );
    if !fits {
        eprintln!(
            "  the barycentric model does not carry the series (residual RMS ≥ 0,1·signal) — the reduction needs the full observation model (DSN station, light time, Moyer); the drift would be an artifact"
        );
        return;
    }
    let times_anomaly = accel.abs() / PIONEER_ANOMALY;
    let drift_significant = se_accel.is_finite() && se_accel < accel.abs();
    if drift_significant && times_anomaly < 3.0 {
        eprintln!(
            "  residual drift {drift_hz_s:.4e} ± {se_hz_s:.4e} Hz/s → acceleration {accel:.4e} ± {se_accel:.4e} m/s² — the Pioneer anomaly ({PIONEER_ANOMALY:.3e} m/s², sunward = negative) is carried"
        );
    } else {
        eprintln!(
            "  residual drift {drift_hz_s:.4e} ± {se_hz_s:.4e} Hz/s → acceleration {accel:.4e} ± {se_accel:.4e} m/s² — the self-test does NOT carry the anomaly: the magnitude lies ~{times_anomaly:.0e}× over the Pioneer anomaly ({PIONEER_ANOMALY:.3e} m/s²) and is compatible with the residual scale — the barycentric daily motion does not isolate the 8,74e-10 m/s²; the rest is model residual (daily Earth rotation, DSN station, light time). The full observation model (Moyer 00-07) is the next atom (0 honored)"
        );
    }
}

fn main() {
    let mut eph: HashMap<String, BodyEphemeris> = HashMap::new();
    for body in ["earth", "pioneer10_daily"] {
        load_eph(body, &mut eph);
    }
    run(&eph);
}
