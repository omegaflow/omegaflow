use omegaflow::odf;

fn ls_scan(ts: &[f64], vs: &[f64], flo: f64, fhi: f64, step: f64) -> (Vec<(f64, f64)>, f64) {
    let m = ts.len() as f64;
    let vsum = vs.iter().sum::<f64>() / m;
    let mut grid: Vec<(f64, f64)> = Vec::new();
    let mut f = flo;
    while f <= fhi {
        grid.push((f, 0.0));
        f += step;
    }
    for (fr, p) in grid.iter_mut() {
        let mut s = 0.0;
        let mut c = 0.0;
        for &t in ts {
            let ph = std::f64::consts::TAU * *fr * t;
            s += ph.sin();
            c += ph.cos();
        }
        s /= m;
        c /= m;
        let mut ss = 0.0;
        let mut cc = 0.0;
        let mut sc = 0.0;
        let mut sy = 0.0;
        let mut cy = 0.0;
        for (i, &t) in ts.iter().enumerate() {
            let ph = std::f64::consts::TAU * *fr * t;
            let ds = ph.sin() - s;
            let dc = ph.cos() - c;
            let dv = vs[i] - vsum;
            ss += ds * ds;
            cc += dc * dc;
            sc += ds * dc;
            sy += ds * dv;
            cy += dc * dv;
        }
        let det = ss * cc - sc * sc;
        if det.abs() > 1e-300 {
            let a = (sy * cc - cy * sc) / det;
            let b = (cy * ss - sy * sc) / det;
            *p = (a * a + b * b) * m / 2.0;
        }
    }
    let mut pows: Vec<f64> = grid.iter().map(|(_, p)| *p).collect();
    pows.sort_by(f64::total_cmp);
    let floor = pows[pows.len() / 2];
    (grid, floor)
}

fn at(grid: &[(f64, f64)], f: f64, floor: f64) -> f64 {
    grid.iter()
        .min_by(|a, b| (a.0 - f).abs().total_cmp(&(b.0 - f).abs()))
        .map(|(_, p)| *p / floor)
        .unwrap_or(0.0)
}

fn rms(v: &[f64]) -> f64 {
    let m = v.iter().sum::<f64>() / v.len() as f64;
    (v.iter().map(|x| (x - m) * (x - m)).sum::<f64>() / v.len() as f64).sqrt()
}

fn quad_detrend(ts: &[f64], vs: &[f64], gap: f64, min_len: usize) -> (Vec<f64>, Vec<f64>) {
    let mut dts: Vec<f64> = Vec::new();
    let mut dvs: Vec<f64> = Vec::new();
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
                }
            }
        }
        lo = hi;
    }
    (dts, dvs)
}

fn main() {
    let Some(bytes) = std::fs::read("data/pioneer11_residuum.bin").ok() else {
        eprintln!("data/pioneer11_residuum.bin void — leer (0 honored)");
        return;
    };
    let Some(recs) = odf::parse_p11r_bin(&bytes) else {
        eprintln!("P11R parse void");
        return;
    };
    let n = recs.len();
    let mut ts = vec![0.0f64; n];
    let mut resid = vec![0.0f64; n];
    let mut pairs = vec![(0i64, 0i64); n];
    for (i, r) in recs.iter().enumerate() {
        ts[i] = r[0];
        resid[i] = r[1];
        pairs[i] = (r[5] as i64, r[6] as i64);
    }

    let mut per_pair: std::collections::BTreeMap<(i64, i64), Vec<usize>> =
        std::collections::BTreeMap::new();
    for i in 0..n {
        per_pair.entry(pairs[i]).or_default().push(i);
    }
    let mut keys: Vec<(i64, i64)> = per_pair.keys().copied().collect();
    keys.sort();
    for (rx, tx) in keys {
        let idx = &per_pair[&(rx, tx)];
        if idx.len() < 500 {
            continue;
        }
        let p_ts: Vec<f64> = idx.iter().map(|&i| ts[i]).collect();
        let p_vs: Vec<f64> = idx.iter().map(|&i| resid[i]).collect();
        let (g, fl) = ls_scan(&p_ts, &p_vs, 0.0004, 0.0015, 0.00002);
        let mut top = g.clone();
        top.sort_by(|a, b| b.1.total_cmp(&a.1));
        eprintln!(
            "Stufe A  Paar rx={rx} tx={tx}: n={}, Residuum-Spitze {:.5} Hz ({:.1}×)",
            idx.len(),
            top[0].0,
            top[0].1 / fl
        );
    }

    let (dq_ts, dq_vs) = quad_detrend(&ts, &resid, 600.0, 8);
    eprintln!(
        "Stage B  quadratische Detrendung: {n} → {} Samples, RMS {:.3e} → {:.3e} Hz",
        dq_ts.len(),
        rms(&resid),
        rms(&dq_vs)
    );

    let r1 = dq_vs.clone();

    let mut cells2: std::collections::BTreeMap<(i64, i64), Vec<usize>> =
        std::collections::BTreeMap::new();

    let mut dq_idx: Vec<usize> = Vec::new();
    let mut j = 0usize;
    for i in 0..n {
        if j < dq_ts.len() && (ts[i] - dq_ts[j]).abs() < 0.5 {
            dq_idx.push(i);
            j += 1;
        }
    }
    for (k, &orig) in dq_idx.iter().enumerate() {
        cells2.entry(pairs[orig]).or_default().push(k);
    }
    let mut r2 = r1.clone();
    for idx in cells2.values() {
        let mut vals: Vec<f64> = idx.iter().map(|&k| r1[k]).collect();
        vals.sort_by(f64::total_cmp);
        let med = vals[vals.len() / 2];
        for &k in idx {
            r2[k] -= med;
        }
    }
    eprintln!("Stage B  nach pair-cell: RMS {:.3e} Hz", rms(&r2));

    if dq_ts.len() < 500 {
        eprintln!("Stufe C: zu kurz (0 honored)");
        return;
    }
    let (g1, fl1) = ls_scan(&dq_ts, &r2, 0.0004, 0.0015, 0.00002);
    let mut top1 = g1.clone();
    top1.sort_by(|a, b| b.1.total_cmp(&a.1));
    eprintln!(
        "Stufe C  Rest (0,4–1,5 mHz): Spitzen {:.5} ({:.1}×), {:.5} ({:.1}×), {:.5} ({:.1}×) — Alias 0,71 mHz: {:.1}×, Gap-Raster 1/600: {:.1}×",
        top1[0].0,
        top1[0].1 / fl1,
        top1[1].0,
        top1[1].1 / fl1,
        top1[2].0,
        top1[2].1 / fl1,
        at(&g1, 0.000714, fl1),
        at(&g1, 1.0 / 600.0, fl1)
    );
    let (g2, fl2) = ls_scan(&dq_ts, &r2, 0.0002, 0.008, 0.00005);
    let mut top2 = g2.clone();
    top2.sort_by(|a, b| b.1.total_cmp(&a.1));
    let tops2: Vec<String> = top2
        .iter()
        .take(4)
        .map(|(f, p)| format!("{:.4} Hz ({:.1}×)", f, p / fl2))
        .collect();
    eprintln!(
        "Stufe C  Rest (0,2–8 mHz): Spitzen {} — am 1/600-Raster (1,67/3,33 mHz): {:.1}×/{:.1}×",
        tops2.join(", "),
        at(&g2, 1.0 / 600.0, fl2),
        at(&g2, 2.0 / 600.0, fl2)
    );
}
