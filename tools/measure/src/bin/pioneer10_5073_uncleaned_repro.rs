use omegaflow::atdf::parse_bin;

const DAY_S: f64 = 86400.0;
const BAND_LO: f64 = 0.044;
const BAND_HI: f64 = 0.058;
const COARSE_LO: f64 = 0.040;
const COARSE_HI: f64 = 0.060;
const STEP_FINE_01: f64 = 0.0001;
const STEP_FINE_005: f64 = 0.00005;
const STEP_COARSE: f64 = 0.001;
const MIN_N: usize = 200;
const GAP_RUN_S: f64 = 600.0;
const MIN_RUN: usize = 4;
const STATIONS: [i64; 3] = [14, 43, 63];

fn year_of(tdb: f64) -> Option<i64> {
    let jd = 2451545.0 + tdb / DAY_S;
    let unix_day = (jd - 2440587.5).round() as i64;
    omegaflow::spectral::civil_from_days(unix_day).map(|(y, _, _)| y as i64)
}

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

fn top_peaks(grid: &[(f64, f64)], k: usize) -> Vec<(f64, f64)> {
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

struct Scan {
    interp: f64,
    grid_f: f64,
    ratio: f64,
    amp: f64,
    floor: f64,
    top5: Vec<(f64, f64)>,
}

fn scan(ts: &[f64], vs: &[f64], flo: f64, fhi: f64, step: f64) -> Option<Scan> {
    if ts.len() < MIN_N {
        return None;
    }
    let grid = ls_grid(ts, vs, flo, fhi, step);
    if grid.len() < 3 {
        return None;
    }
    let (fgrid, p, ratio) = peak_of(&grid);
    let interp = match peak_interp(&grid) {
        Some(v) => v,
        None => fgrid,
    };
    let amp = (2.0 * p / ts.len() as f64).sqrt();
    let mut pows: Vec<f64> = grid.iter().map(|g| g.1).collect();
    pows.sort_by(f64::total_cmp);
    let floor = pows[pows.len() / 2];
    let top5 = top_peaks(&grid, 5);
    Some(Scan {
        interp,
        grid_f: fgrid,
        ratio,
        amp,
        floor,
        top5,
    })
}

fn fmt_scan(label: &str, s: Option<Scan>, grid_label: &str) {
    match s {
        Some(s) => {
            let tops: Vec<String> = s
                .top5
                .iter()
                .map(|(f, q)| format!("{:.2} mHz ({:.1}x)", f * 1e3, q / s.floor))
                .collect();
            eprintln!(
                "  {label} [{grid_label}]: peak {:.3} mHz (grid {:.2} mHz, {:.1}x floor, A={:.2e} Hz); top-5: {}",
                s.interp * 1e3,
                s.grid_f * 1e3,
                s.ratio,
                s.amp,
                tops.join(" | ")
            );
        }
        None => eprintln!("  {label} [{grid_label}]: absent"),
    }
}

fn report_set(label: &str, ts: &[f64], vs: &[f64]) {
    let n = ts.len();
    eprintln!("{label}: n={n}");
    if n < MIN_N {
        eprintln!("  n < {MIN_N} — absent (0 honored)");
        return;
    }
    let fine01 = scan(ts, vs, BAND_LO, BAND_HI, STEP_FINE_01);
    let fine005 = scan(ts, vs, BAND_LO, BAND_HI, STEP_FINE_005);
    let coarse = scan(ts, vs, COARSE_LO, COARSE_HI, STEP_COARSE);
    fmt_scan("fine 0.1 mHz (44-58)", fine01, "0.1 mHz");
    fmt_scan("fine 0.05 mHz (44-58)", fine005, "0.05 mHz");
    fmt_scan("coarse 1 mHz (40-60)", coarse, "1 mHz");
}

fn gather(
    times: &[f64],
    stations: &[i64],
    samplers: &[f64],
    resid: &[f64],
    sts: &[i64],
    class: fn(f64) -> bool,
) -> (Vec<f64>, Vec<f64>) {
    let mut ts = Vec::new();
    let mut vs = Vec::new();
    for i in 0..times.len() {
        if !sts.contains(&stations[i]) || !class(samplers[i]) {
            continue;
        }
        ts.push(times[i]);
        vs.push(resid[i]);
    }
    (ts, vs)
}

fn main() {
    let Some(pasf) = std::fs::read("data/spdf.gsfc.nasa.gov/pioneer10_skyfreq_6file.bin")
        .ok()
        .and_then(|b| parse_bin(&b))
    else {
        eprintln!("6-file skyfreq bin void");
        return;
    };
    eprintln!("6-file skyfreq: {} records", pasf.len());

    let mut times: Vec<f64> = Vec::new();
    let mut samplers: Vec<f64> = Vec::new();
    let mut stations: Vec<i64> = Vec::new();
    let mut resid: Vec<f64> = Vec::new();
    for r in &pasf {
        if !r[8].is_finite() {
            continue;
        }
        times.push(r[0]);
        samplers.push(r[3]);
        stations.push(r[6] as i64);
        resid.push(r[8]);
    }
    let n_fin = times.len();
    let n_1s = samplers.iter().filter(|&&s| s == 1.0).count();
    let n_sub10 = samplers.iter().filter(|&&s| s < 10.0).count();
    eprintln!(
        "uncleaned doppler_resid (slot 8): {n_fin} finite; strict-1.0-s={n_1s}, sub-10-s={n_sub10}"
    );

    let class_1s: fn(f64) -> bool = |s| s == 1.0;
    let class_sub10: fn(f64) -> bool = |s| s < 10.0;
    let classes: [(&str, fn(f64) -> bool); 2] =
        [("sub-10-s", class_sub10), ("strict-1.0-s", class_1s)];
    let all_stations: [i64; 4] = [14, 43, 61, 63];

    for (cname, cpred) in &classes {
        eprintln!("\n===== UNCLEANED, {cname}, all stations pooled =====");
        let (ts, vs) = gather(&times, &stations, &samplers, &resid, &all_stations, *cpred);
        report_set("  pooled (14,43,61,63)", &ts, &vs);
    }

    for (cname, cpred) in &classes {
        eprintln!("\n===== UNCLEANED, {cname}, per station =====");
        for st in STATIONS {
            let (ts, vs) = gather(&times, &stations, &samplers, &resid, &[st], *cpred);
            report_set(&format!("  station {st}"), &ts, &vs);
        }
    }

    for (cname, cpred) in &classes {
        eprintln!("\n===== UNCLEANED, {cname}, per year (all stations pooled) =====");
        for y in 1988..=1993 {
            let (ts_all, vs_all) =
                gather(&times, &stations, &samplers, &resid, &all_stations, *cpred);
            let mut ts = Vec::new();
            let mut vs = Vec::new();
            for i in 0..ts_all.len() {
                if year_of(ts_all[i]) == Some(y) {
                    ts.push(ts_all[i]);
                    vs.push(vs_all[i]);
                }
            }
            let n = ts.len();
            match scan(&ts, &vs, BAND_LO, BAND_HI, STEP_FINE_01) {
                Some(s) => eprintln!(
                    "  {y}: n={n} peak {:.3} mHz ({:.1}x, A={:.2e} Hz)",
                    s.interp * 1e3,
                    s.ratio,
                    s.amp
                ),
                None => eprintln!("  {y}: n={n} absent"),
            }
        }
    }

    eprintln!("\n===== DETREND SENSITIVITY (global pooled, fine 0.1 mHz) =====");
    for (cname, cpred) in &classes {
        let (ts, vs) = gather(&times, &stations, &samplers, &resid, &all_stations, *cpred);
        eprintln!("  {cname} n={}:", ts.len());
        match scan(&ts, &vs, BAND_LO, BAND_HI, STEP_FINE_01) {
            Some(s) => eprintln!(
                "    detrend=none:       peak {:.3} mHz ({:.1}x)",
                s.interp * 1e3,
                s.ratio
            ),
            None => eprintln!("    detrend=none: absent"),
        }
        let (dts, dvs) = detrend_runs(&ts, &vs);
        match scan(&dts, &dvs, BAND_LO, BAND_HI, STEP_FINE_01) {
            Some(s) => eprintln!(
                "    detrend=run-linear(600s): peak {:.3} mHz ({:.1}x)",
                s.interp * 1e3,
                s.ratio
            ),
            None => eprintln!("    detrend=run-linear(600s): absent"),
        }
    }
}
