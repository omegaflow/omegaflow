use std::collections::BTreeMap;

use omegaflow::odf::parse_p11r_bin;

fn rms(v: &[f64]) -> f64 {
    if v.is_empty() {
        return f64::NAN;
    }
    let m = v.iter().sum::<f64>() / v.len() as f64;
    (v.iter().map(|x| (x - m) * (x - m)).sum::<f64>() / v.len() as f64).sqrt()
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

fn run(name: &str) {
    let path = format!("data/{name}_navio_residuum.bin");
    let Some(bytes) = std::fs::read(&path).ok() else {
        eprintln!("{name}: residuum bin void ({path}) — 0 honored");
        return;
    };
    let Some(recs) = parse_p11r_bin(&bytes) else {
        eprintln!("{name}: residuum parse void");
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
        "{name}: Deduktion-40 negative-fuzzy — {n} two-way residual samples, RMS {rms0:.3e} Hz",
        rms0 = rms(&resid)
    );

    let gap_pass = 900.0;
    let (dq_ts, dq_vs, dq_rx) = quad_detrend_cells(&ts, &resid, &rx, gap_pass, 8);
    eprintln!(
        "{name}: quadratic per-pass detrend (gap {gap_pass:.0} s, min 8): {n} → {} samples, RMS {rms0:.3e} → {rms1:.3e} Hz",
        dq_ts.len(),
        rms0 = rms(&resid),
        rms1 = rms(&dq_vs)
    );
    if dq_ts.len() < 500 {
        eprintln!("{name}: too few detrended samples — stays silent (0 honored)");
        return;
    }

    let mut r2 = dq_vs.clone();
    let mut by_rx: BTreeMap<i64, Vec<usize>> = BTreeMap::new();
    for (k, &st) in dq_rx.iter().enumerate() {
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
    eprintln!(
        "{name}: after station-cell median subtraction: RMS {:.3e} Hz",
        rms(&r2)
    );

    let day_s = 86400.0;
    let mut daily: BTreeMap<i64, Vec<f64>> = BTreeMap::new();
    for (k, &t) in dq_ts.iter().enumerate() {
        let day = (t / day_s).floor() as i64;
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

    let out = format!("data/{name}_navio_subkhz_daily.bin");
    let mut db = Vec::with_capacity(8 + daily_med.len() * 32);
    db.extend_from_slice(b"PNDM");
    db.extend_from_slice(&(daily_med.len() as u32).to_le_bytes());
    for (day, med, r, nv) in &daily_med {
        for v in [*day as f64 * day_s, *med, *r, *nv as f64] {
            db.extend_from_slice(&v.to_le_bytes());
        }
    }
    if std::fs::write(&out, &db).is_err() {
        eprintln!("{name}: write {out} void");
    } else {
        eprintln!(
            "{name}: {out} — {} negative-fuzzy daily medians serialized (PNDM, sub-kHz basis for the Ruck scan)",
            daily_med.len()
        );
    }
}

fn main() {
    for name in ["pioneer10", "pioneer11"] {
        run(name);
    }
}
