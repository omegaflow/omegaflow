use std::collections::{BTreeMap, HashMap};

use omegaflow::archivar::{body_barycenter_position, parse_ephemeris_binary, BodyEphemeris};
use omegaflow::odf::parse_p11r_bin;

const DAY_S: f64 = 86400.0;
const AU: f64 = 1.495978707e11;

fn rms(v: &[f64]) -> f64 {
    if v.is_empty() {
        return f64::NAN;
    }
    let m = v.iter().sum::<f64>() / v.len() as f64;
    (v.iter().map(|x| (x - m) * (x - m)).sum::<f64>() / v.len() as f64).sqrt()
}

fn norm(v: [f64; 3]) -> f64 {
    (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
}

fn lin_fit(xs: &[f64], ys: &[f64]) -> Option<(f64, f64)> {
    let n = xs.len() as f64;
    if n < 5.0 {
        return None;
    }
    let mx = xs.iter().sum::<f64>() / n;
    let my = ys.iter().sum::<f64>() / n;
    let mut num = 0.0;
    let mut den = 0.0;
    for i in 0..xs.len() {
        num += (xs[i] - mx) * (ys[i] - my);
        den += (xs[i] - mx) * (xs[i] - mx);
    }
    if den.abs() < 1e-300 {
        return None;
    }
    let a = num / den;
    Some((a, my - a * mx))
}

fn subhz_drift(name: &str, ts: &[f64], r2: &[f64]) {
    const C: f64 = 299792458.0;
    const TR: f64 = 240.0 / 221.0;
    const AP: f64 = 8.74e-10;
    const F0: f64 = 2.292e9;
    const TAU_Y: f64 = 126.52;
    let day_s = 86400.0;
    let t0 = ts[0];
    let t: Vec<f64> = ts.iter().map(|x| (x - t0) / day_s).collect();
    let span_y = (ts[ts.len() - 1] - t0) / day_s / 365.25;
    eprintln!(
        "{name}: sub-Hz drift regression on {n} negative-fuzzy per-sample residua over {span_y:.2} y",
        n = ts.len()
    );
    let k = TR / C;
    let (a_lin, _) = lin_fit(&t, r2).unwrap_or((0.0, 0.0));
    let drift_hzday = a_lin;
    let accel = drift_hzday / day_s / (k * F0);
    let resid_lin: Vec<f64> = t
        .iter()
        .zip(r2.iter())
        .map(|(tt, v)| v - a_lin * (tt - t[0]))
        .collect();
    let rms_lin = rms(&resid_lin);
    let mt = t.iter().sum::<f64>() / t.len() as f64;
    let sxx: f64 = t.iter().map(|tt| (tt - mt) * (tt - mt)).sum();
    let se = if sxx > 0.0 {
        rms_lin / sxx.sqrt()
    } else {
        f64::NAN
    };
    let accel_se = se / day_s / (k * F0);
    let sigma = accel / accel_se.max(1e-300);
    eprintln!(
        "{name}: linear slope {drift_hzday:.4e} Hz/day (SE {se:.4e}) -> acceleration {accel:.3e} m/s² = {r:.1}× anomaly, {sigma:.1}σ (per-sample RMS {rms_lin:.3e} Hz; the slope SE is the honest uncertainty, not the naked point)",
        r = accel / AP
    );
    let x2: Vec<f64> = t.iter().map(|tt| (tt - mt) * (tt - mt)).collect();
    let a_q = lin_fit(&x2, r2).map(|(a, _)| a).unwrap_or(0.0);
    let resid_q: Vec<f64> = x2.iter().zip(r2.iter()).map(|(x, v)| v - a_q * x).collect();
    let rms_q = rms(&resid_q);
    let tau_s = TAU_Y * 365.25 * day_s;
    let dec: Vec<f64> = t
        .iter()
        .map(|tt| 1.0 - (-tt * day_s / tau_s).exp())
        .collect();
    let a_e = lin_fit(&dec, r2).map(|(a, _)| a).unwrap_or(0.0);
    let resid_e: Vec<f64> = dec
        .iter()
        .zip(r2.iter())
        .map(|(d, v)| v - a_e * d)
        .collect();
    let rms_e = rms(&resid_e);
    eprintln!(
        "{name}: model resid RMS — linear {rms_lin:.3e}, ∝t² {rms_q:.3e}, RTG-exp τ={TAU_Y:.0}y {rms_e:.3e} Hz"
    );
    let imp = |a: f64, b: f64| (a - b) / a * 100.0;
    eprintln!(
        "{name}: ∝t² improves on linear {iq:.2} %, RTG-exp {ie:.2} % (per-sample regression, √N over ~600k samples)",
        iq = imp(rms_lin, rms_q),
        ie = imp(rms_lin, rms_e)
    );
    eprintln!(
        "{name}: the anomaly (~1 Hz/mission) vs per-sample regression floor {rms_lin:.1e} Hz — held against this floor, not claimed (0 honored)"
    );
}

fn quad_detrend_cells(
    ts: &[f64],
    vs: &[f64],
    rx: &[i64],
    gap: f64,
    min_len: usize,
) -> (Vec<f64>, Vec<f64>, Vec<i64>) {
    let mut dts: Vec<f64> = Vec::new();
    let mut dvs: Vec<f64> = Vec::new();
    let mut drx: Vec<i64> = Vec::new();
    let mut lo = 0usize;
    while lo < ts.len() {
        let mut hi = lo + 1;
        while hi < ts.len() && ts[hi] - ts[hi - 1] <= gap {
            hi += 1;
        }
        if hi - lo >= min_len {
            let xs: Vec<f64> = ts[lo..hi].to_vec();
            let ys: Vec<f64> = vs[lo..hi].to_vec();
            let tx = xs.iter().sum::<f64>() / xs.len() as f64;
            let m0 = xs.len() as f64;
            let sx = xs.iter().map(|x| x - tx).sum::<f64>();
            let sy = ys.iter().sum::<f64>();
            let sxx = xs.iter().map(|x| (x - tx) * (x - tx)).sum::<f64>();
            let sxxx = xs.iter().map(|x| (x - tx).powi(3)).sum::<f64>();
            let sxxxx = xs.iter().map(|x| (x - tx).powi(4)).sum::<f64>();
            let sxy = xs.iter().zip(&ys).map(|(x, y)| (x - tx) * y).sum::<f64>();
            let sxxy = xs
                .iter()
                .zip(&ys)
                .map(|(x, y)| (x - tx) * (x - tx) * y)
                .sum::<f64>();
            let det = m0 * (sxx * sxxxx - sxxx * sxxx) - sx * (sx * sxxxx - sxx * sxxx)
                + sxx * (sx * sxxx - sxx * sxx);
            if det.abs() > 1e-300 {
                let cc = (sy * (sxx * sxxxx - sxxx * sxxx) - sx * (sxy * sxxxx - sxxx * sxxy)
                    + sxx * (sxy * sxxx - sxx * sxxy))
                    / det;
                let cb = (m0 * (sxy * sxxxx - sxxx * sxxy) - sy * (sx * sxxxx - sxx * sxxx)
                    + sxx * (sx * sxxy - sxy * sxx))
                    / det;
                let ca = (m0 * (sxx * sxxy - sxy * sxxx) - sx * (sx * sxxy - sxy * sxx)
                    + sy * (sx * sxxx - sxx * sxx))
                    / det;
                for k in 0..xs.len() {
                    let dx = xs[k] - tx;
                    dts.push(xs[k]);
                    dvs.push(ys[k] - (ca * dx * dx + cb * dx + cc));
                    drx.push(rx[lo + k]);
                }
            }
        }
        lo = hi;
    }
    (dts, dvs, drx)
}

fn station_cell_median(r2: &mut Vec<f64>, rx: &[i64]) {
    let mut by_rx: BTreeMap<i64, Vec<usize>> = BTreeMap::new();
    for (k, &st) in rx.iter().enumerate() {
        by_rx.entry(st).or_default().push(k);
    }
    for idx in by_rx.values() {
        let mut vals: Vec<f64> = idx.iter().map(|&k| r2[k]).collect();
        vals.sort_by(f64::total_cmp);
        let med = vals[vals.len() / 2];
        for &k in idx {
            r2[k] -= med;
        }
    }
}

fn report(name: &str, ts: &[f64], resid: &[f64], rx: &[i64], out_path: Option<&str>) {
    let n_in = ts.len();
    let gap_pass = 900.0;
    let (dq_ts, dq_vs, dq_rx) = quad_detrend_cells(ts, resid, rx, gap_pass, 8);
    eprintln!(
        "{name}: quadratic per-pass detrend (gap {gap_pass:.0} s, min 8): {n_in} → {} samples, RMS {rms0:.3e} → {rms1:.3e} Hz",
        dq_ts.len(),
        rms0 = rms(resid),
        rms1 = rms(&dq_vs)
    );
    if dq_ts.len() < 500 {
        eprintln!("{name}: too few detrended samples — stays silent (0 honored)");
        return;
    }

    let mut r2 = dq_vs.clone();
    station_cell_median(&mut r2, &dq_rx);
    eprintln!(
        "{name}: after station-cell median subtraction: RMS {:.3e} Hz",
        rms(&r2)
    );

    subhz_drift(name, &dq_ts, &r2);

    let mut daily: BTreeMap<i64, Vec<f64>> = BTreeMap::new();
    for (k, &t) in dq_ts.iter().enumerate() {
        let day = (t / DAY_S).floor() as i64;
        daily.entry(day).or_default().push(r2[k]);
    }
    let mut daily_med: Vec<(i64, f64, f64, usize)> = Vec::new();
    for (day, mut v) in daily {
        v.sort_by(f64::total_cmp);
        let med = v[v.len() / 2];
        let nv = v.len();
        let mean = v.iter().sum::<f64>() / nv as f64;
        let r = (v.iter().map(|x| (x - mean) * (x - mean)).sum::<f64>() / nv as f64).sqrt();
        daily_med.push((day, med, r, nv));
    }
    daily_med.sort_by_key(|d| d.0);
    let meds: Vec<f64> = daily_med.iter().map(|d| d.1).collect();
    let ms = rms(&meds);
    let mut absmed: Vec<f64> = meds.iter().map(|x| x.abs()).collect();
    absmed.sort_by(f64::total_cmp);
    let amed = absmed[absmed.len() / 2];
    eprintln!(
        "{name}: negative-fuzzy daily medians: {} days, median |daily-med| {amed:.3e} Hz, RMS of daily medians {ms:.3e} Hz",
        daily_med.len()
    );
    let subkhz = daily_med.iter().filter(|d| d.1.abs() < 1000.0).count();
    eprintln!(
        "{name}: daily medians below 1 kHz (sub-kHz): {subkhz} of {} ({:.1} %) — the sub-kHz residency of the daily median field",
        daily_med.len(),
        100.0 * subkhz as f64 / daily_med.len() as f64
    );
    let ms_m = if meds.is_empty() {
        f64::NAN
    } else {
        meds.iter().sum::<f64>() / meds.len() as f64
    };
    eprintln!(
        "{name}: daily-median scatter about 0: mean {ms_m:.3e}, RMS {ms:.3e} Hz — the floor the transit sweep would read",
    );
    let pct = |p: f64| -> f64 {
        if absmed.is_empty() {
            return f64::NAN;
        }
        let idx = ((absmed.len() as f64 - 1.0) * p).round() as usize;
        absmed[idx.min(absmed.len() - 1)]
    };
    eprintln!(
        "{name}: |daily-med| percentiles [p50,p90,p95,p99] = [{:.0}, {:.0}, {:.0}, {:.0}] Hz — the sub-kHz residency is median-driven, the tail is the jitter days",
        pct(0.5),
        pct(0.9),
        pct(0.95),
        pct(0.99)
    );

    if let Some(path) = out_path {
        let mut db = Vec::with_capacity(8 + daily_med.len() * 32);
        db.extend_from_slice(b"PNDM");
        db.extend_from_slice(&(daily_med.len() as u32).to_le_bytes());
        for (day, med, r, nv) in &daily_med {
            for v in [*day as f64 * DAY_S, *med, *r, *nv as f64] {
                db.extend_from_slice(&v.to_le_bytes());
            }
        }
        if std::fs::write(path, &db).is_err() {
            eprintln!("{name}: write {path} void");
        } else {
            eprintln!(
                "{name}: {path} — {} negative-fuzzy daily medians serialized (PNDM, sub-kHz basis for the Ruck scan)",
                daily_med.len()
            );
        }
    }
}

fn load_ephemeris(name: &str, eph: &mut HashMap<String, BodyEphemeris>) -> bool {
    let p = format!("data/ephemeris_{name}.bin");
    std::fs::read(&p)
        .ok()
        .and_then(|d| parse_ephemeris_binary(&d))
        .map(|e| {
            eph.insert(name.to_string(), e);
        })
        .is_some()
}

fn zone_predicate(probe: &str) -> Option<(f64, f64)> {
    match probe {
        // quiet-zone ranges from pioneer_navio_noise_geo (0932cae): the distance
        // band whose per-day resid-RMS fell to the floor is the drift-only zone.
        "pioneer10" => Some((50.0, f64::INFINITY)),
        "pioneer11" => Some((15.0, 30.0)),
        _ => None,
    }
}

fn run(probe: &str, sc_body: &str, zone: bool) {
    let path = format!("data/{probe}_navio_residuum.bin");
    let Some(bytes) = std::fs::read(&path).ok() else {
        eprintln!("{probe}: residuum bin void ({path}) — 0 honored");
        return;
    };
    let Some(recs) = parse_p11r_bin(&bytes) else {
        eprintln!("{probe}: residuum parse void");
        return;
    };
    let n = recs.len();
    let mut ts = vec![0.0f64; n];
    let mut resid = vec![0.0f64; n];
    let mut rx = vec![0i64; n];
    for (i, r) in recs.iter().enumerate() {
        ts[i] = r[0];
        resid[i] = r[1];
        rx[i] = r[5] as i64;
    }
    eprintln!(
        "{probe}: Deduktion-40 negative-fuzzy — {n} two-way residual samples, RMS {rms0:.3e} Hz",
        rms0 = rms(&resid)
    );

    // The global floor (existing path), always — byte-identical serialization.
    report(
        probe,
        &ts,
        &resid,
        &rx,
        Some(&format!("data/{probe}_navio_subkhz_daily.bin")),
    );

    if !zone {
        return;
    }
    let (au_lo, au_hi) = match zone_predicate(probe) {
        Some(z) => z,
        None => {
            eprintln!("{probe}: no quiet-zone range — 0 honored");
            return;
        }
    };
    let mut eph: HashMap<String, BodyEphemeris> = HashMap::new();
    if !load_ephemeris(sc_body, &mut eph) {
        eprintln!("{probe}: {sc_body} ephemeris bin void — zone isolation void (0 honored)");
        return;
    }
    // Per-day heliocentric distance (barycentric ≈ heliocentric for an outbound
    // probe; same AU convention as pioneer_navio_noise_geo 0932cae).
    let mut day_au: BTreeMap<i64, f64> = BTreeMap::new();
    for &t in ts.iter() {
        let day = (t / DAY_S).floor() as i64;
        if day_au.contains_key(&day) {
            continue;
        }
        if let Some(p) = body_barycenter_position(sc_body, t, &eph) {
            day_au.insert(day, norm(p) / AU);
        }
    }
    if day_au.is_empty() {
        eprintln!("{probe}: no ephemeris position resolved for any day — 0 honored");
        return;
    }
    let zone_name = if au_hi.is_finite() {
        format!("{probe}[zone {au_lo:.0}–{au_hi:.0} AU]")
    } else {
        format!("{probe}[zone >{au_lo:.0} AU]")
    };
    let mut zts: Vec<f64> = Vec::new();
    let mut zres: Vec<f64> = Vec::new();
    let mut zrx: Vec<i64> = Vec::new();
    let mut first_day = i64::MAX;
    let mut last_day = i64::MIN;
    let mut zdays: BTreeMap<i64, bool> = BTreeMap::new();
    for i in 0..n {
        let day = (ts[i] / DAY_S).floor() as i64;
        let au = match day_au.get(&day) {
            Some(a) => *a,
            None => continue,
        };
        if au > au_lo && au <= au_hi {
            zts.push(ts[i]);
            zres.push(resid[i]);
            zrx.push(rx[i]);
            zdays.insert(day, true);
            first_day = first_day.min(day);
            last_day = last_day.max(day);
        }
    }
    // timtag is TDB seconds since J2000 (the residuum compiler derives jd via
    // jd = tdb/DAY_S + 2451545.0), so the era year counts from 2000.
    let era_y0 = 2000.0 + first_day as f64 / 365.25;
    let era_y1 = 2000.0 + last_day as f64 / 365.25;
    eprintln!(
        "{probe}: quiet-zone isolation ({au_lo:.0}–{au_hi:.0} AU, era {era_y0:.0}–{era_y1:.0}) — {nz} of {n} samples across {nd} distinct days",
        nz = zts.len(),
        nd = zdays.len()
    );
    if zts.len() < 500 {
        eprintln!("{probe}: quiet zone too thin — stays silent (0 honored)");
        return;
    }
    report(
        &zone_name,
        &zts,
        &zres,
        &zrx,
        Some(&format!("data/{probe}_navio_subkhz_zone_daily.bin")),
    );
    eprintln!(
        "{probe}: quiet-zone reduction above — compare its |daily-med| floor against the global floor of the non-zone run"
    );
}

fn main() {
    let zone = std::env::args().any(|a| a == "--zone");
    for (probe, sc) in [
        ("pioneer10", "pioneer10_daily"),
        ("pioneer11", "pioneer11_daily"),
    ] {
        run(probe, sc, zone);
    }
}
