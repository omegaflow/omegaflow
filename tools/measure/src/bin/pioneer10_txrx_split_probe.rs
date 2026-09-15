use std::collections::BTreeMap;

use omegaflow::atdf::parse_bin as parse_pasf;
use omegaflow::doppler::parse_pnav_bin;

const DAY_S: f64 = 86400.0;
const BAND_LO: f64 = 0.044;
const BAND_HI: f64 = 0.056;
const STEP: f64 = 0.00002;
const STATIONS: [i64; 3] = [14, 43, 63];
const GAP_RUN_S: f64 = 600.0;
const MIN_RUN: usize = 4;
const JOIN_TOL_S: f64 = 120.0;

fn ls_grid(times: &[f64], vals: &[f64], flo: f64, fhi: f64, step: f64) -> Vec<(f64, f64)> {
    let mut grid: Vec<(f64, f64)> = Vec::new();
    let mut f = flo;
    while f <= fhi {
        grid.push((f, 0.0));
        f += step;
    }
    if grid.is_empty() {
        return grid;
    }
    let m = times.len() as f64;
    let vsum = vals.iter().sum::<f64>() / m;
    for (fref, pow) in grid.iter_mut() {
        let mut s = 0.0;
        let mut c = 0.0;
        for &t in times {
            let ph = std::f64::consts::TAU * *fref * t;
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
        for (i, &t) in times.iter().enumerate() {
            let ph = std::f64::consts::TAU * *fref * t;
            let ds = ph.sin() - s;
            let dc = ph.cos() - c;
            let dv = vals[i] - vsum;
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
            *pow = (a * a + b * b) * m / 2.0;
        }
    }
    grid
}

fn peak_of(grid: &[(f64, f64)]) -> (f64, f64, f64) {
    if grid.is_empty() {
        return (f64::NAN, 0.0, f64::NAN);
    }
    let mut best = grid[0];
    for g in grid {
        if g.1 > best.1 {
            best = *g;
        }
    }
    let mut pows: Vec<f64> = grid.iter().map(|g| g.1).collect();
    pows.sort_by(f64::total_cmp);
    let floor = pows[pows.len() / 2];
    (best.0, best.1, best.1 / floor)
}

fn peak_interp(grid: &[(f64, f64)]) -> Option<f64> {
    let (fmax, _, _) = peak_of(grid);
    let k = grid.iter().position(|g| g.0 == fmax)?;
    if k == 0 || k + 1 >= grid.len() {
        return None;
    }
    let step = grid[k].0 - grid[k - 1].0;
    let pm = grid[k - 1].1;
    let p0 = grid[k].1;
    let pp = grid[k + 1].1;
    let denom = pm - 2.0 * p0 + pp;
    if denom.abs() < 1e-300 {
        return None;
    }
    let d = 0.5 * (pm - pp) / denom;
    if d.abs() > 1.0 {
        return None;
    }
    Some(fmax + d * step)
}

fn jd_date(tdb: f64) -> String {
    let jd = 2451545.0 + tdb / DAY_S;
    let unix_day = (jd - 2440587.5).round() as i64;
    match omegaflow::spectral::civil_from_days(unix_day) {
        Some((y, m, d)) => format!("{y:04}-{m:02}-{d:02}"),
        None => format!("tdb {tdb:.0} s"),
    }
}

fn detrend_runs(ts: &[f64], vs: &[f64]) -> (Vec<f64>, Vec<f64>) {
    let mut dts: Vec<f64> = Vec::new();
    let mut dvs: Vec<f64> = Vec::new();
    let mut lo = 0usize;
    while lo < ts.len() {
        let mut hi = lo + 1;
        while hi < ts.len() && ts[hi] - ts[hi - 1] <= GAP_RUN_S {
            hi += 1;
        }
        if hi - lo >= MIN_RUN {
            let n = (hi - lo) as f64;
            let mx = ts[lo..hi].iter().sum::<f64>() / n;
            let my = vs[lo..hi].iter().sum::<f64>() / n;
            let mut num = 0.0;
            let mut den = 0.0;
            for k in lo..hi {
                num += (ts[k] - mx) * (vs[k] - my);
                den += (ts[k] - mx) * (ts[k] - mx);
            }
            let slope = if den.abs() > 1e-300 { num / den } else { 0.0 };
            for k in lo..hi {
                dts.push(ts[k]);
                dvs.push(vs[k] - (slope * (ts[k] - mx) + my));
            }
        }
        lo = hi;
    }
    (dts, dvs)
}

fn top_peaks(ts: &[f64], vs: &[f64], flo: f64, fhi: f64, step: f64, k: usize) -> Vec<(f64, f64)> {
    let grid = ls_grid(ts, vs, flo, fhi, step);
    let mut lmax: Vec<(f64, f64)> = Vec::new();
    for i in 1..grid.len() - 1 {
        if grid[i].1 > grid[i - 1].1 && grid[i].1 > grid[i + 1].1 {
            lmax.push(grid[i]);
        }
    }
    lmax.sort_by(|a, b| b.1.total_cmp(&a.1));
    let mut taken: Vec<(f64, f64)> = Vec::new();
    for (f, p) in lmax {
        if taken.iter().all(|(tf, _)| (tf - f).abs() >= 0.0003) {
            taken.push((f, p));
        }
        if taken.len() >= k {
            break;
        }
    }
    taken
}

fn ls_amp(ts: &[f64], vs: &[f64], f: f64) -> Option<f64> {
    let g = ls_grid(ts, vs, f, f, 1e-12);
    let (_, p) = g.first().copied()?;
    Some((2.0 * p / ts.len() as f64).sqrt())
}

fn report_peak(label: &str, ts: &[f64], vs: &[f64]) {
    if ts.len() < 200 {
        eprintln!("  {label}: n={} — too short (0 honored)", ts.len());
        return;
    }
    let (dts, dvs) = detrend_runs(ts, vs);
    if dts.len() < 200 {
        eprintln!(
            "  {label}: n={} ({} after run-detrend) — too short (0 honored)",
            ts.len(),
            dts.len()
        );
        return;
    }
    let grid = ls_grid(&dts, &dvs, BAND_LO, BAND_HI, STEP);
    let (fp, pp, ratio) = peak_of(&grid);
    let fp_i = peak_interp(&grid).unwrap_or(fp);
    let amp = (2.0 * pp / dts.len() as f64).sqrt();
    eprintln!(
        "  {label}: n={}, peak {fp_i:.5} Hz ({ratio:.1}× floor), A = {amp:.2e} Hz",
        dts.len()
    );
}

fn main() {
    let pasf_bytes = match std::fs::read("data/spdf.gsfc.nasa.gov/pioneer10_skyfreq.bin") {
        Ok(b) => b,
        Err(_) => {
            eprintln!("PASF void — empty (0 honored)");
            return;
        }
    };
    let Some(pasf) = parse_pasf(&pasf_bytes) else {
        eprintln!("PASF parse void");
        return;
    };
    let pnav_bytes = match std::fs::read("data/spdf.gsfc.nasa.gov/pioneer10_navio.bin") {
        Ok(b) => b,
        Err(_) => {
            eprintln!("PNAV void — empty (0 honored)");
            return;
        }
    };
    let Some(pnav) = parse_pnav_bin(&pnav_bytes) else {
        eprintln!("PNAV parse void");
        return;
    };

    eprintln!(
        "PASF: {} records, {}..{}",
        pasf.len(),
        jd_date(pasf[0][0]),
        jd_date(pasf[pasf.len() - 1][0])
    );
    eprintln!(
        "PNAV: {} records, {}..{}",
        pnav.len(),
        jd_date(pnav[0][0]),
        jd_date(pnav[pnav.len() - 1][0])
    );

    let mut pasf_mode_st: BTreeMap<(i64, i64, i64), usize> = BTreeMap::new();
    for r in &pasf {
        let st = r[6] as i64;
        let mode = r[13] as i64;
        let cls = if r[3] < 10.0 {
            1
        } else if r[3] < 30.0 {
            10
        } else {
            60
        };
        *pasf_mode_st.entry((st, mode, cls)).or_insert(0) += 1;
    }
    eprintln!("PASF (station, ground_mode, sampler_class) census:");
    for ((st, mode, cls), n) in &pasf_mode_st {
        eprintln!("  station {st} mode {mode} class {cls}s: n={n}");
    }

    let mut pnav_dtype: BTreeMap<i64, usize> = BTreeMap::new();
    let mut pnav_mode: BTreeMap<i64, usize> = BTreeMap::new();
    let mut pnav_pair: BTreeMap<(i64, i64), usize> = BTreeMap::new();
    for r in &pnav {
        *pnav_dtype.entry(r[4] as i64).or_insert(0) += 1;
        *pnav_mode.entry(r[8] as i64).or_insert(0) += 1;
        if r[6] > 0.0 && r[7] > 0.0 {
            *pnav_pair.entry((r[7] as i64, r[6] as i64)).or_insert(0) += 1;
        }
    }
    eprintln!("PNAV dtype census: {:?}", pnav_dtype);
    eprintln!("PNAV mode census: {:?}", pnav_mode);
    eprintln!("PNAV (rx, tx) census (rx = slot 7, tx = slot 6):");
    for ((rx, tx), n) in &pnav_pair {
        eprintln!("  rx {rx} tx {tx}: n={n}");
    }

    eprintln!("--- reproduce the band per rx (PASF resid slot 8, 1-s class, mode 3) ---");
    for st in STATIONS {
        let mut ts: Vec<f64> = Vec::new();
        let mut vs: Vec<f64> = Vec::new();
        for r in &pasf {
            if r[6] as i64 == st && r[13] as i64 == 3 && r[3] < 10.0 && r[8].is_finite() {
                ts.push(r[0]);
                vs.push(r[8]);
            }
        }
        report_peak(&format!("rx {st} (1-s, mode 3)"), &ts, &vs);
    }

    eprintln!("--- does the PNAV obs (slot 1) carry the band? per rx ---");
    for st in STATIONS {
        let mut ts: Vec<f64> = Vec::new();
        let mut vs: Vec<f64> = Vec::new();
        for r in &pnav {
            if r[7] as i64 == st && r[8] as i64 == 13 && r[1].is_finite() {
                ts.push(r[0]);
                vs.push(r[1]);
            }
        }
        report_peak(&format!("PNAV obs rx {st} (mode 13)"), &ts, &vs);
    }

    eprintln!("--- the split: PASF 1-s mode-3 samples joined against PNAV for tx ---");
    let mut by_rx: BTreeMap<i64, Vec<(f64, f64, i64)>> = BTreeMap::new();
    for r in &pnav {
        let rx = r[7] as i64;
        let tx = r[6] as i64;
        if rx <= 0 || tx <= 0 {
            continue;
        }
        by_rx.entry(rx).or_default().push((r[0], r[6], tx));
    }
    for seq in by_rx.values_mut() {
        seq.sort_by(|a, b| a.0.total_cmp(&b.0));
    }
    let mut per_rx: BTreeMap<i64, Vec<(f64, f64)>> = BTreeMap::new();
    let mut per_pair: BTreeMap<(i64, i64), Vec<(f64, f64)>> = BTreeMap::new();
    let mut matched = 0usize;
    let mut unmatched = 0usize;
    let mut dt_min = f64::INFINITY;
    let mut dt_max = 0.0f64;
    for r in &pasf {
        let st = r[6] as i64;
        if r[13] as i64 != 3 || r[3] >= 10.0 || !r[8].is_finite() {
            continue;
        }
        let t = r[0];
        per_rx.entry(st).or_default().push((t, r[8]));
        let seq = match by_rx.get(&st) {
            Some(s) => s,
            None => {
                unmatched += 1;
                continue;
            }
        };
        let idx = seq.partition_point(|x| x.0 < t);
        let mut best: Option<(f64, i64)> = None;
        if idx < seq.len() {
            let cand = &seq[idx];
            let d = (cand.0 - t).abs();
            if d <= JOIN_TOL_S {
                best = Some((d, cand.2));
            }
        }
        if idx > 0 {
            let cand = &seq[idx - 1];
            let d = (cand.0 - t).abs();
            if d <= JOIN_TOL_S && best.map_or(true, |(bd, _)| d < bd) {
                best = Some((d, cand.2));
            }
        }
        match best {
            Some((d, tx)) => {
                matched += 1;
                if d < dt_min {
                    dt_min = d;
                }
                if d > dt_max {
                    dt_max = d;
                }
                per_pair.entry((st, tx)).or_default().push((t, r[8]));
            }
            None => unmatched += 1,
        }
    }
    eprintln!("  join: {matched} matched (|Δt| {dt_min:.0}..{dt_max:.0} s), {unmatched} unmatched");

    eprintln!("--- per-rx top-5 peaks (band from PASF resid, mode 3 1-s) ---");
    let mut rx_top: BTreeMap<i64, Vec<(f64, f64)>> = BTreeMap::new();
    for st in STATIONS {
        let Some(seq) = per_rx.get(&st) else {
            continue;
        };
        let ts: Vec<f64> = seq.iter().map(|x| x.0).collect();
        let vs: Vec<f64> = seq.iter().map(|x| x.1).collect();
        let tops = top_peaks(&ts, &vs, BAND_LO, BAND_HI, STEP, 5);
        let names: Vec<String> = tops
            .iter()
            .map(|(f, p)| {
                let a = (2.0 * p / ts.len() as f64).sqrt();
                format!("{:.5} Hz ({:.2e})", f, a)
            })
            .collect();
        eprintln!("  rx {st} (n={}): top-5 {}", ts.len(), names.join(" | "));
        rx_top.insert(st, tops);
    }

    eprintln!("--- per-(rx,tx) peaks + amplitude at the three rx anchors ---");
    for st in STATIONS {
        let anchors = &rx_top[&st];
        let mut pairs: Vec<i64> = per_pair
            .iter()
            .filter(|((rx, _), _)| *rx == st)
            .map(|((_, tx), _)| *tx)
            .collect();
        pairs.sort_unstable();
        pairs.dedup();
        if pairs.is_empty() {
            eprintln!("  rx {st}: no joined tx (0 honored)");
            continue;
        }
        for tx in pairs {
            let Some(seq) = per_pair.get(&(st, tx)) else {
                continue;
            };
            let ts: Vec<f64> = seq.iter().map(|x| x.0).collect();
            let vs: Vec<f64> = seq.iter().map(|x| x.1).collect();
            let tops = top_peaks(&ts, &vs, BAND_LO, BAND_HI, STEP, 3);
            let top_names: Vec<String> = tops.iter().map(|(f, _)| format!("{:.5} Hz", f)).collect();
            let mut anchor_amps: Vec<String> = Vec::new();
            for (fa, _) in anchors.iter().take(3) {
                match ls_amp(&ts, &vs, *fa) {
                    Some(a) => anchor_amps.push(format!("{:.5}→{:.2e}", fa, a)),
                    None => anchor_amps.push(format!("{:.5}→absent", fa)),
                }
            }
            let mname = if st == tx { "2-way" } else { "3-way" };
            eprintln!(
                "  rx {st} tx {tx} ({mname}): n={}, top-3 [{}], amp@rx-anchors [{}]",
                ts.len(),
                top_names.join(" "),
                anchor_amps.join(" ")
            );
        }
    }
}
