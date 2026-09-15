use std::collections::BTreeMap;

use omegaflow::atdf::parse_bin as parse_pasf;

const DAY_S: f64 = 86400.0;
const BAND_LO: f64 = 0.044;
const BAND_HI: f64 = 0.058;
const STEP: f64 = 0.00002;
const STATIONS: [i64; 3] = [14, 43, 63];
const GAP_RUN_S: f64 = 600.0;
const MIN_RUN: usize = 4;

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

fn year_of(tdb: f64) -> Option<i64> {
    let jd = 2451545.0 + tdb / DAY_S;
    let unix_day = (jd - 2440587.5).round() as i64;
    omegaflow::spectral::civil_from_days(unix_day).map(|(y, _, _)| y as i64)
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

fn topk(ts: &[f64], vs: &[f64], k: usize) -> Option<Vec<(f64, f64)>> {
    if ts.len() < 200 {
        return None;
    }
    let (dts, dvs) = detrend_runs(ts, vs);
    if dts.len() < 200 {
        return None;
    }
    let tops = top_peaks(&dts, &dvs, BAND_LO, BAND_HI, STEP, k);
    let out: Vec<(f64, f64)> = tops
        .iter()
        .map(|(f, p)| (*f, (2.0 * p / dts.len() as f64).sqrt()))
        .collect();
    Some(out)
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
    let pasf_bytes = match std::fs::read("data/spdf.gsfc.nasa.gov/pioneer10_skyfreq_6file.bin") {
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
    eprintln!(
        "PASF 6-file: {} records, {}..{}",
        pasf.len(),
        jd_date(pasf[0][0]),
        jd_date(pasf[pasf.len() - 1][0])
    );

    let mut sampler_hist: BTreeMap<i64, usize> = BTreeMap::new();
    let mut n_sub10 = 0usize;
    let mut n_strict1 = 0usize;
    for r in &pasf {
        let s = r[3];
        if s <= 1.0 {
            n_strict1 += 1;
        }
        if s < 10.0 {
            n_sub10 += 1;
        }
        *sampler_hist.entry(s.round() as i64).or_insert(0) += 1;
    }
    eprintln!("  sampler census: sub-10-s n={n_sub10}, strict <=1-s n={n_strict1}");
    let mut hist: Vec<(i64, usize)> = sampler_hist.into_iter().collect();
    hist.sort_by_key(|(k, _)| *k);
    let hist_s: Vec<String> = hist.iter().map(|(k, n)| format!("{k}s:{n}")).collect();
    eprintln!("  sampler histogram: {}", hist_s.join(" "));

    let mut m3_hist: BTreeMap<i64, usize> = BTreeMap::new();
    let mut m3_sub10 = 0usize;
    let mut m3_strict1 = 0usize;
    let mut m3_sub10_3st = 0usize;
    let mut m3_strict1_3st = 0usize;
    let mut m3_three_class: BTreeMap<i64, usize> = BTreeMap::new();
    for r in &pasf {
        if r[13] as i64 != 3 {
            continue;
        }
        let s = r[3];
        *m3_hist.entry(s.round() as i64).or_insert(0) += 1;
        if s < 10.0 {
            m3_sub10 += 1;
        }
        if s <= 1.0 {
            m3_strict1 += 1;
        }
        let st = r[6] as i64;
        let in3 = st == 14 || st == 43 || st == 63;
        if in3 && s < 10.0 {
            m3_sub10_3st += 1;
        }
        if in3 && s <= 1.0 {
            m3_strict1_3st += 1;
        }
        let cls = if s <= 1.0 {
            1
        } else if s <= 11.0 {
            10
        } else {
            60
        };
        *m3_three_class.entry(cls).or_insert(0) += 1;
    }
    let mut m3h: Vec<(i64, usize)> = m3_hist.into_iter().collect();
    m3h.sort_by_key(|(k, _)| *k);
    let m3h_s: Vec<String> = m3h.iter().map(|(k, n)| format!("{k}s:{n}")).collect();
    eprintln!("  mode-3 sampler histogram: {}", m3h_s.join(" "));
    eprintln!(
        "  mode-3 sub-10-s n={m3_sub10}, strict <=1-s n={m3_strict1}, sub-10-s[14/43/63] n={m3_sub10_3st}, strict[14/43/63] n={m3_strict1_3st}"
    );
    let mut m3c: Vec<(i64, usize)> = m3_three_class.into_iter().collect();
    m3c.sort_by_key(|(k, _)| *k);
    let m3c_s: Vec<String> = m3c.iter().map(|(k, n)| format!("class{k}:{n}")).collect();
    eprintln!("  mode-3 class(1/10/60) census: {}", m3c_s.join(" "));

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

    let years: Vec<i64> = (1987..=1994).collect();
    eprintln!("--- per-rx band peak per year (44-58 mHz, 1-s mode 3) ---");
    for st in STATIONS {
        let mut by_year: BTreeMap<i64, (Vec<f64>, Vec<f64>)> = BTreeMap::new();
        for r in &pasf {
            if r[6] as i64 == st && r[13] as i64 == 3 && r[3] < 10.0 && r[8].is_finite() {
                if let Some(y) = year_of(r[0]) {
                    let e = by_year.entry(y).or_default();
                    e.0.push(r[0]);
                    e.1.push(r[8]);
                }
            }
        }
        eprintln!("  rx {st}:");
        for y in &years {
            match by_year.get(y) {
                Some((ts, vs)) => match topk(ts, vs, 3) {
                    Some(peaks) => {
                        let names: Vec<String> = peaks
                            .iter()
                            .map(|(f, a)| format!("{f:.5} (A={a:.2e})"))
                            .collect();
                        eprintln!("    {y}: n={} top-3 {}", ts.len(), names.join(" | "));
                    }
                    None => eprintln!("    {y}: n={} — short (0 honored)", ts.len()),
                },
                None => eprintln!("    {y}: no samples (0 honored)"),
            }
        }
    }
}
