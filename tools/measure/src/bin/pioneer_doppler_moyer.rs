use omegaflow::archivar::{
    BodyEphemeris, body_barycenter_position, body_barycenter_velocity, parse_ephemeris_binary,
};
use omegaflow::atdf::parse_bin;
use omegaflow::odp::{EARTH, downlink_rate, dsn_station};
use std::collections::HashMap;

const PIONEER_ANOMALY: f64 = 8.74e-10;
const SC_BODY: &str = "pioneer10_daily";

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
    (
        (n * sxy - sx * sy) / denom,
        (sy - (n * sxy - sx * sy) / denom * sx) / n,
    )
}

fn lin_fit3(x1: &[f64], x2: &[f64], y: &[f64]) -> Option<(f64, f64, f64)> {
    let n = y.len() as f64;
    if n < 3.0 {
        return None;
    }
    let mx1 = x1.iter().sum::<f64>() / n;
    let mx2 = x2.iter().sum::<f64>() / n;
    let my = y.iter().sum::<f64>() / n;
    let mut s11 = 0.0;
    let mut s12 = 0.0;
    let mut s22 = 0.0;
    let mut sy1 = 0.0;
    let mut sy2 = 0.0;
    for i in 0..y.len() {
        let d1 = x1[i] - mx1;
        let d2 = x2[i] - mx2;
        let dy = y[i] - my;
        s11 += d1 * d1;
        s12 += d1 * d2;
        s22 += d2 * d2;
        sy1 += d1 * dy;
        sy2 += d2 * dy;
    }
    let det = s11 * s22 - s12 * s12;
    if det.abs() < 1e-300 {
        return None;
    }
    let a = (sy1 * s22 - sy2 * s12) / det;
    let c = (sy2 * s11 - sy1 * s12) / det;
    let b = my - a * mx1 - c * mx2;
    Some((a, c, b))
}

fn main() {
    let path = "data/pioneer10_skyfreq.bin";
    let Ok(bytes) = std::fs::read(path) else {
        eprintln!("pioneer10: skyfreq bin void ({path})");
        return;
    };
    let Some(records) = parse_bin(&bytes) else {
        eprintln!("pioneer10: skyfreq bin parse void");
        return;
    };
    let mut eph: HashMap<String, BodyEphemeris> = HashMap::new();
    for body in [EARTH, SC_BODY] {
        let p = format!("data/ephemeris_{body}.bin");
        match std::fs::read(&p)
            .ok()
            .and_then(|d| parse_ephemeris_binary(&d))
        {
            Some(e) => {
                eph.insert(body.to_string(), e);
            }
            None => {
                eprintln!("{body}: ephemeris bin void ({p})");
                return;
            }
        }
    }
    let mut rates: Vec<f64> = Vec::with_capacity(records.len());
    let mut refs: Vec<f64> = Vec::with_capacity(records.len());
    let mut obs: Vec<f64> = Vec::with_capacity(records.len());
    let mut times: Vec<f64> = Vec::with_capacity(records.len());
    let mut no_station = 0usize;
    let mut no_model = 0usize;
    let sc = &|t| {
        Some((
            body_barycenter_position(SC_BODY, t, &eph)?,
            body_barycenter_velocity(SC_BODY, t, &eph)?,
        ))
    };
    for r in &records {
        let Some((lat, lon, alt)) = dsn_station(r[6] as i64) else {
            no_station += 1;
            continue;
        };
        let Some(rate) = downlink_rate(r[0], lat, lon, alt, &eph, sc) else {
            no_model += 1;
            continue;
        };
        if !rate.is_finite() {
            no_model += 1;
            continue;
        }
        rates.push(rate);
        refs.push(r[2]);
        obs.push(r[1]);
        times.push(r[0]);
    }
    if rates.len() < 100 {
        eprintln!(
            "pioneer10: {} modeled samples — too short (no station {no_station}, no model {no_model})",
            rates.len()
        );
        return;
    }
    let gap_threshold = 5.0 * 86400.0;
    let mut epoch = vec![0usize; rates.len()];
    let mut eid = 0usize;
    for i in 0..rates.len() {
        if i > 0 && times[i] - times[i - 1] > gap_threshold {
            eid += 1;
        }
        epoch[i] = eid;
    }
    let n_epoch = eid + 1;
    let mut mean_rate = vec![0.0f64; n_epoch];
    let mut mean_ref = vec![0.0f64; n_epoch];
    let mut mean_obs = vec![0.0f64; n_epoch];
    let mut cnt = vec![0usize; n_epoch];
    let mut first_t = vec![0.0f64; n_epoch];
    for i in 0..rates.len() {
        let e = epoch[i];
        mean_rate[e] += rates[i];
        mean_ref[e] += refs[i];
        mean_obs[e] += obs[i];
        cnt[e] += 1;
        if cnt[e] == 1 {
            first_t[e] = times[i];
        }
    }
    for e in 0..n_epoch {
        mean_rate[e] /= cnt[e] as f64;
        mean_ref[e] /= cnt[e] as f64;
        mean_obs[e] /= cnt[e] as f64;
    }
    let mut cr = vec![0.0f64; rates.len()];
    let mut cf = vec![0.0f64; rates.len()];
    let mut co = vec![0.0f64; rates.len()];
    for i in 0..rates.len() {
        let e = epoch[i];
        cr[i] = rates[i] - mean_rate[e];
        cf[i] = refs[i] - mean_ref[e];
        co[i] = obs[i] - mean_obs[e];
    }
    let Some((a, c, _)) = lin_fit3(&cr, &cf, &co) else {
        eprintln!("pioneer10: 3-parameter fit void");
        return;
    };
    let mut offset = vec![0.0f64; n_epoch];
    for e in 0..n_epoch {
        offset[e] = mean_obs[e] - a * mean_rate[e] - c * mean_ref[e];
    }
    let offset_mean = offset.iter().sum::<f64>() / n_epoch as f64;
    for e in 0..n_epoch {
        let t0_epoch = first_t[e];
        let jd = t0_epoch / 86400.0 + 2451545.0;
        let day = (jd - 2440587.5).round() as i64;
        let date = match omegaflow::spectral::civil_from_days(day) {
            Some((y, m, d)) => format!("{y:04}-{m:02}-{d:02}"),
            None => format!("t {t0_epoch:.0} s"),
        };
        eprintln!(
            "Epoche {e} ({date}, n={}): Relativ-Offset {:.3e} Hz",
            cnt[e],
            offset[e] - offset_mean
        );
    }
    let mut resid: Vec<f64> = Vec::with_capacity(rates.len());
    let mut rss = 0.0f64;
    for i in 0..rates.len() {
        let rr = obs[i] - a * rates[i] - c * refs[i] - offset[epoch[i]];
        resid.push(rr);
        rss += rr * rr;
    }
    let rms = (rss / rates.len() as f64).sqrt();
    let rmin = rates.iter().cloned().fold(f64::INFINITY, f64::min);
    let rmax = rates.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let signal_span = (a * rmax - a * rmin).abs();
    let (slope, _) = lin_fit(&times, &resid);
    let n = times.len() as f64;
    let t_mean = times.iter().sum::<f64>() / n;
    let sxx: f64 = times.iter().map(|t| (t - t_mean).powi(2)).sum();
    let se_slope = if n > 2.0 && sxx > 0.0 {
        (rss / ((n - 2.0) * sxx)).sqrt()
    } else {
        f64::NAN
    };
    let accel = slope / a;
    let se_accel = se_slope / a.abs();
    let times_anomaly = accel.abs() / PIONEER_ANOMALY;
    let rms_kms = rms / a.abs() / 1000.0;
    eprintln!(
        "pioneer10: {} samples ({no_station} without station, {no_model} without model), {} epochs — fsky = A·ṙ_down + C·ref + B_epoch, A {:.4e} Hz/(m/s), C {:.4e}, signal span {:.3e} Hz, calibrated residual RMS {:.3e} Hz ({:.1}% of the span)",
        rates.len(),
        n_epoch,
        a,
        c,
        signal_span,
        rms,
        100.0 * rms / signal_span
    );
    let significant = se_accel.is_finite() && se_accel < accel.abs();
    if significant && times_anomaly < 3.0 {
        eprintln!(
            "  residual drift {slope:.4e} ± {se_slope:.4e} Hz/s → spacecraft acceleration {accel:.4e} ± {se_accel:.4e} m/s² — the Pioneer anomaly ({PIONEER_ANOMALY:.3e} m/s², sunward = negative) is carried"
        );
    } else {
        eprintln!(
            "  residual drift {slope:.4e} ± {se_slope:.4e} Hz/s → spacecraft acceleration {accel:.4e} ± {se_accel:.4e} m/s² — the self-test does NOT carry the anomaly: the magnitude lies ~{times_anomaly:.0e}× over the Pioneer anomaly ({PIONEER_ANOMALY:.3e} m/s²). The epoch offsets (kHz) are calibrated, but the intra-epoch residual (~{rms:.0e} Hz ≈ {rms_kms:.1} km/s) remains — the Horizons daily motion does not reproduce the raw ATDF Doppler better; the anomaly (~1 Hz over the mission) lies below it. The anomaly needs its own orbit solution from the raw Doppler (ODP), not the public daily motion (0 honored)"
        );
    }
}
