use omegaflow::odf;

fn ls_peak(ts: &[f64], vs: &[f64], flo: f64, fhi: f64, step: f64) -> (f64, f64, f64) {
    let m = ts.len() as f64;
    let vsum = vs.iter().sum::<f64>() / m;
    let mut best = (f64::NAN, 0.0);
    let mut all: Vec<f64> = Vec::new();
    let mut f = flo;
    while f <= fhi {
        let mut s = 0.0;
        let mut c = 0.0;
        for &t in ts {
            let ph = std::f64::consts::TAU * f * t;
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
            let ph = std::f64::consts::TAU * f * t;
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
            let p = (a * a + b * b) * m / 2.0;
            all.push(p);
            if p > best.1 {
                best = (f, p);
            }
        }
        f += step;
    }
    all.sort_by(f64::total_cmp);
    let floor = all[all.len() / 2];
    (best.0, best.1 / floor, (2.0 * best.1 / m).sqrt())
}

fn main() {
    let Some(bytes) = std::fs::read("data/pioneer11_odf.bin").ok() else {
        eprintln!("data/pioneer11_odf.bin void — leer (0 honored)");
        return;
    };
    let Some(recs) = odf::parse_podf_bin(&bytes) else {
        eprintln!("PODF parse void");
        return;
    };

    let mut per_rx: std::collections::BTreeMap<i64, Vec<(f64, i64)>> =
        std::collections::BTreeMap::new();
    for r in &recs {
        if r[4] > 0.0 {
            per_rx
                .entry(r[3] as i64)
                .or_default()
                .push((r[0], r[4] as i64));
        }
    }
    for (rx, seq) in &mut per_rx {
        seq.sort_by(|a, b| a.0.total_cmp(&b.0));

        let mut dedup: Vec<(f64, i64)> = Vec::new();
        for (t, tx) in seq.iter() {
            let day = (t / 86400.0).floor();
            match dedup.last() {
                Some((lt, ltx)) if (*lt / 86400.0).floor() == day && *ltx == *tx => {}
                _ => dedup.push((*t, *tx)),
            }
        }
        let order: Vec<String> = dedup
            .iter()
            .map(|(_t, tx)| format!("{tx}({})", if *tx == *rx { "2w" } else { "3w" }))
            .collect();
        let unique: std::collections::BTreeSet<i64> = dedup.iter().map(|(_, tx)| *tx).collect();
        eprintln!(
            "tx-sequence  rx={rx}: {} transitions, senders {:?} — {}",
            dedup.len(),
            unique,
            order.join(" ")
        );
    }

    let mut per_pair: std::collections::BTreeMap<(i64, i64), (Vec<f64>, Vec<f64>)> =
        std::collections::BTreeMap::new();
    for r in &recs {
        if r[4] > 0.0 {
            per_pair
                .entry((r[3] as i64, r[4] as i64))
                .or_default()
                .0
                .push(r[0]);
            per_pair
                .entry((r[3] as i64, r[4] as i64))
                .or_default()
                .1
                .push(r[1]);
        }
    }
    let mut keys: Vec<(i64, i64)> = per_pair.keys().copied().collect();
    keys.sort();
    for (rx, tx) in keys {
        let (ts, vs) = &per_pair[&(rx, tx)];
        if ts.len() < 500 {
            continue;
        }
        let mut dts: Vec<f64> = Vec::new();
        let mut dvs: Vec<f64> = Vec::new();
        let mut lo = 0usize;
        while lo < ts.len() {
            let mut hi = lo + 1;
            while hi < ts.len() && ts[hi] - ts[hi - 1] <= 600.0 {
                hi += 1;
            }
            if hi - lo >= 4 {
                let xt: Vec<f64> = ts[lo..hi].to_vec();
                let yv: Vec<f64> = vs[lo..hi].to_vec();
                let mx = xt.iter().sum::<f64>() / xt.len() as f64;
                let my = yv.iter().sum::<f64>() / yv.len() as f64;
                let mut num = 0.0;
                let mut den = 0.0;
                for k in 0..xt.len() {
                    num += (xt[k] - mx) * (yv[k] - my);
                    den += (xt[k] - mx) * (xt[k] - mx);
                }
                let slope = if den.abs() > 1e-300 { num / den } else { 0.0 };
                for k in 0..xt.len() {
                    dts.push(xt[k]);
                    dvs.push(yv[k] - (slope * (xt[k] - mx) + my));
                }
            }
            lo = hi;
        }
        if dts.len() < 300 {
            continue;
        }
        let (fp, ratio, amp) = ls_peak(&dts, &dvs, 0.0004, 0.0015, 0.00002);
        let mname = if rx == tx { "2-way" } else { "3-way" };
        eprintln!(
            "pair rx={rx} tx={tx} ({mname}): n={}, peak {:.5} Hz ({ratio:.1}× floor), A = {amp:.2e} Hz",
            dts.len(),
            fp
        );
    }
}
