use std::collections::HashMap;

use omegaflow::archivar::{
    BodyEphemeris, body_barycenter_position, body_barycenter_velocity, parse_ephemeris_binary,
};
use omegaflow::doppler::parse_bin;
use omegaflow::odp::{EARTH, downlink_rate_core};

const PIONEER_ANOMALY: f64 = 8.74e-10;
const GAP_THRESHOLD: f64 = 5.0 * 86400.0;
const DISPLACED_HZ: f64 = 1.0e5;

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

fn fixed_effects(
    rate: &[f64],
    freq: &[f64],
    obs: &[f64],
    times: &[f64],
    active: &[bool],
) -> Option<(f64, f64, Vec<f64>, usize, Vec<f64>)> {
    let n = rate.len();
    let mut epoch = vec![0usize; n];
    let mut eid = 0usize;
    let mut prev = f64::NAN;
    for i in 0..n {
        if active[i] {
            if prev.is_finite() && times[i] - prev > GAP_THRESHOLD {
                eid += 1;
            }
            prev = times[i];
        }
        epoch[i] = eid;
    }
    let n_epoch = eid + 1;
    let mut mr = vec![0.0f64; n_epoch];
    let mut mf = vec![0.0f64; n_epoch];
    let mut mo = vec![0.0f64; n_epoch];
    let mut cnt = vec![0usize; n_epoch];
    for i in 0..n {
        if !active[i] {
            continue;
        }
        let e = epoch[i];
        mr[e] += rate[i];
        mf[e] += freq[i];
        mo[e] += obs[i];
        cnt[e] += 1;
    }
    for e in 0..n_epoch {
        if cnt[e] == 0 {
            continue;
        }
        mr[e] /= cnt[e] as f64;
        mf[e] /= cnt[e] as f64;
        mo[e] /= cnt[e] as f64;
    }
    let mut cr = Vec::new();
    let mut cf = Vec::new();
    let mut co = Vec::new();
    for i in 0..n {
        if !active[i] || cnt[epoch[i]] == 0 {
            continue;
        }
        cr.push(rate[i] - mr[epoch[i]]);
        cf.push(freq[i] - mf[epoch[i]]);
        co.push(obs[i] - mo[epoch[i]]);
    }
    let (a, c, _) = lin_fit3(&cr, &cf, &co)?;
    let mut offset = vec![0.0f64; n_epoch];
    for e in 0..n_epoch {
        offset[e] = mo[e] - a * mr[e] - c * mf[e];
    }
    let mut resid = vec![0.0f64; n];
    for i in 0..n {
        resid[i] = obs[i] - a * rate[i] - c * freq[i] - offset[epoch[i]];
    }
    Some((a, c, resid, n_epoch, offset))
}

fn fixed_effects_1(
    x: &[f64],
    obs: &[f64],
    times: &[f64],
    active: &[bool],
) -> Option<(f64, Vec<f64>, usize)> {
    let n = x.len();
    let mut epoch = vec![0usize; n];
    let mut eid = 0usize;
    let mut prev = f64::NAN;
    for i in 0..n {
        if active[i] {
            if prev.is_finite() && times[i] - prev > GAP_THRESHOLD {
                eid += 1;
            }
            prev = times[i];
        }
        epoch[i] = eid;
    }
    let n_epoch = eid + 1;
    let mut mx = vec![0.0f64; n_epoch];
    let mut mo = vec![0.0f64; n_epoch];
    let mut cnt = vec![0usize; n_epoch];
    for i in 0..n {
        if !active[i] {
            continue;
        }
        let e = epoch[i];
        mx[e] += x[i];
        mo[e] += obs[i];
        cnt[e] += 1;
    }
    for e in 0..n_epoch {
        if cnt[e] == 0 {
            continue;
        }
        mx[e] /= cnt[e] as f64;
        mo[e] /= cnt[e] as f64;
    }
    let mut sxx = 0.0;
    let mut sxy = 0.0;
    for i in 0..n {
        if !active[i] || cnt[epoch[i]] == 0 {
            continue;
        }
        let dx = x[i] - mx[epoch[i]];
        let dy = obs[i] - mo[epoch[i]];
        sxx += dx * dx;
        sxy += dx * dy;
    }
    if sxx.abs() < 1e-300 {
        return None;
    }
    let k = sxy / sxx;
    let mut offset = vec![0.0f64; n_epoch];
    for e in 0..n_epoch {
        offset[e] = mo[e] - k * mx[e];
    }
    let mut resid = vec![0.0f64; n];
    for i in 0..n {
        resid[i] = obs[i] - k * x[i] - offset[epoch[i]];
    }
    Some((k, resid, n_epoch))
}

fn run(name: &str, sc_body: &str) {
    let path = format!("data/{name}_doppler_clean.bin");
    let Ok(bytes) = std::fs::read(&path) else {
        eprintln!("{name}: clean bin void ({path})");
        return;
    };
    let Some(records) = parse_bin(&bytes) else {
        eprintln!("{name}: clean bin parse void");
        return;
    };
    let mut dtype_counts: HashMap<i64, usize> = HashMap::new();
    for r in &records {
        *dtype_counts.entry(r[4] as i64).or_insert(0) += 1;
    }
    eprintln!(
        "{name}: {} records, DTYPE {:?}",
        records.len(),
        dtype_counts
    );

    let mut eph: HashMap<String, BodyEphemeris> = HashMap::new();
    for body in [EARTH, sc_body] {
        let p = format!("data/ephemeris_{body}.bin");
        match std::fs::read(&p)
            .ok()
            .and_then(|d| parse_ephemeris_binary(&d))
        {
            Some(e) => {
                eph.insert(body.to_string(), e);
            }
            None => {
                eprintln!("{name}: ephemeris bin void ({p})");
                return;
            }
        }
    }
    let sc = |t: f64| -> Option<([f64; 3], [f64; 3])> {
        Some((
            body_barycenter_position(sc_body, t, &eph)?,
            body_barycenter_velocity(sc_body, t, &eph)?,
        ))
    };

    let mut rates = Vec::new();
    let mut freqs = Vec::new();
    let mut obs = Vec::new();
    let mut times = Vec::new();
    let mut cmptimes = Vec::new();
    let mut excluded_dtype = 0usize;
    for r in &records {
        if r[4] as i64 != 12 {
            excluded_dtype += 1;
            continue;
        }
        let (Some(re), Some(ve)) = (
            body_barycenter_position(EARTH, r[0], &eph),
            body_barycenter_velocity(EARTH, r[0], &eph),
        ) else {
            continue;
        };
        let Some(rate) = downlink_rate_core(r[0], re, ve, &sc) else {
            continue;
        };
        if !rate.is_finite() {
            continue;
        }
        rates.push(2.0 * rate);
        freqs.push(r[2]);
        obs.push(r[1]);
        times.push(r[0]);
        cmptimes.push(r[3]);
    }
    eprintln!(
        "{name}: {} two-way samples modeled ({excluded_dtype} non-DTYPE-12 excluded)",
        rates.len()
    );
    if rates.len() < 100 {
        eprintln!("{name}: too short");
        return;
    }

    let n = rates.len();
    let mut active = vec![true; n];

    let (_, _, resid0, _, _) = fixed_effects(&rates, &freqs, &obs, &times, &active).unwrap();
    let mut displaced = 0usize;
    for i in 0..n {
        if resid0[i].abs() > DISPLACED_HZ {
            active[i] = false;
            displaced += 1;
        }
    }

    let (a, c, resid, n_epoch, _offset) =
        fixed_effects(&rates, &freqs, &obs, &times, &active).unwrap();
    let used: Vec<usize> = (0..n).filter(|&i| active[i]).collect();
    let rms = (used.iter().map(|&i| resid[i] * resid[i]).sum::<f64>() / used.len() as f64).sqrt();

    let k_phys = (240.0 / 221.0) / 299792458.0;
    let x: Vec<f64> = (0..n).map(|i| freqs[i] * rates[i]).collect();
    let (k, resid_prod, n_epoch_prod) = fixed_effects_1(&x, &obs, &times, &active).unwrap();
    let rms_prod = (used
        .iter()
        .map(|&i| resid_prod[i] * resid_prod[i])
        .sum::<f64>()
        / used.len() as f64)
        .sqrt();

    let mut n500 = 0usize;
    let mut n1k = 0usize;
    for &i in &used {
        let r = resid_prod[i];
        let e500 = 500000.0 / cmptimes[i];
        let k500 = (r / e500).round();
        if k500 != 0.0 && (r - k500 * e500).abs() < 0.1 * e500 {
            n500 += 1;
            continue;
        }
        let k1k = (r / 1000.0).round();
        if k1k != 0.0 && (r - k1k * 1000.0).abs() < 100.0 {
            n1k += 1;
        }
    }

    let rmin = rates.iter().cloned().fold(f64::INFINITY, f64::min);
    let rmax = rates.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let signal_span = (k * freqs[0] * (rmax - rmin)).abs();
    let slope = {
        let mx = used.iter().map(|&i| times[i]).sum::<f64>() / used.len() as f64;
        let my = used.iter().map(|&i| resid_prod[i]).sum::<f64>() / used.len() as f64;
        let mut num = 0.0;
        let mut den = 0.0;
        for &i in &used {
            num += (times[i] - mx) * (resid_prod[i] - my);
            den += (times[i] - mx) * (times[i] - mx);
        }
        num / den
    };
    let accel = slope / (k * freqs[0]);
    eprintln!(
        "{name}: linear OBSVBL = A·ṙ₂w + C·freq + B_epoch — A {a:.4e} Hz/(m/s), C {c:.4e}, Residuum-RMS {rms:.3e} Hz ({n_epoch} Epochen)"
    );
    eprintln!(
        "{name}: product OBSVBL = K·freq·ṙ₂w + B_epoch — K {k:.6e} s/m (reference (240/221)/c = {k_phys:.6e}), residual RMS {rms_prod:.3e} Hz ({n_epoch_prod} epochs, {pct:.1}% of the span); {displaced} shifted counts (>100 kHz) discarded",
        pct = 100.0 * rms_prod / signal_span
    );
    eprintln!(
        "{name}: count-correction classes recognized — ±500k count ≈8,3 kHz: {n500}, 1000-Hz observable: {n1k} — marginal at a residual of {rms_prod:.0e} Hz, NOT blindly corrected (0 honored); they need a sub-kHz model (DSN station + orbit)"
    );
    let resid_khz = rms_prod / 1000.0;
    eprintln!(
        "{name}: self-test — the product residual ({rms_prod:.0e} Hz ≈ {resid_khz:.1} kHz) lies ~{ratio:.0e}× over the ~1-Hz signal of the Pioneer anomaly ({PIONEER_ANOMALY:.3e} m/s²): NOT carried; the residual drift {slope:.4e} Hz/s → {accel:.4e} m/s² is model artifact, not the anomaly (0 honored)",
        ratio = rms_prod / 1.0
    );
}

fn main() {
    for (name, sc_body) in [
        ("pioneer10", "pioneer10_daily"),
        ("pioneer11", "pioneer11_daily"),
    ] {
        run(name, sc_body);
    }
}
