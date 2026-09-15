use std::collections::BTreeMap;

use omegaflow::atdf::parse_bin;

const BAND_LO: f64 = 0.044;
const BAND_HI: f64 = 0.058;
const DAY_S: f64 = 86400.0;
const STATIONS: [i64; 3] = [14, 43, 63];
const MIN_RUN: usize = 4;
const MIN_N: usize = 200;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Detrend {
    None,
    RunLinear,
    RunQuadratic,
    GlobalLinear,
}

fn year_of(tdb: f64) -> Option<i64> {
    let jd = 2451545.0 + tdb / DAY_S;
    let unix_day = (jd - 2440587.5).round() as i64;
    omegaflow::spectral::civil_from_days(unix_day).map(|(y, _, _)| y as i64)
}

fn solve(a: &[f64], b: &[f64], k: usize) -> Option<Vec<f64>> {
    let mut m = vec![0.0f64; k * (k + 1)];
    for i in 0..k {
        for j in 0..k {
            m[i * (k + 1) + j] = a[i * k + j];
        }
        m[i * (k + 1) + k] = b[i];
    }
    for col in 0..k {
        let mut piv = col;
        for row in col + 1..k {
            if m[row * (k + 1) + col].abs() > m[piv * (k + 1) + col].abs() {
                piv = row;
            }
        }
        if m[piv * (k + 1) + col].abs() < 1e-300 {
            return None;
        }
        if piv != col {
            for j in 0..=k {
                m.swap(col * (k + 1) + j, piv * (k + 1) + j);
            }
        }
        let d = m[col * (k + 1) + col];
        for j in col..=k {
            m[col * (k + 1) + j] /= d;
        }
        for row in 0..k {
            if row == col {
                continue;
            }
            let f = m[row * (k + 1) + col];
            for j in col..=k {
                m[row * (k + 1) + j] -= f * m[col * (k + 1) + j];
            }
        }
    }
    Some((0..k).map(|i| m[i * (k + 1) + k]).collect())
}

fn fit_resid(ts: &[f64], vs: &[f64], deg: usize) -> Vec<f64> {
    let n = ts.len();
    if n < deg + 1 {
        return vs.to_vec();
    }
    let mt = ts.iter().sum::<f64>() / n as f64;
    let k = deg + 1;
    let mut ata = vec![0.0f64; k * k];
    let mut aty = vec![0.0f64; k];
    for i in 0..n {
        let x = ts[i] - mt;
        let mut basis = vec![1.0f64; k];
        let mut p = 1.0;
        for j in 1..k {
            p *= x;
            basis[j] = p;
        }
        for a in 0..k {
            aty[a] += basis[a] * vs[i];
            for b in 0..k {
                ata[a * k + b] += basis[a] * basis[b];
            }
        }
    }
    let Some(coef) = solve(&ata, &aty, k) else {
        return vs.to_vec();
    };
    vs.iter()
        .enumerate()
        .map(|(i, &y)| {
            let x = ts[i] - mt;
            let mut p = 1.0;
            let mut f = 0.0;
            for c in &coef {
                f += c * p;
                p *= x;
            }
            y - f
        })
        .collect()
}

fn split_runs(ts: &[f64], gap: f64) -> Vec<(usize, usize)> {
    let mut runs = Vec::new();
    let mut lo = 0usize;
    while lo < ts.len() {
        let mut hi = lo + 1;
        while hi < ts.len() && ts[hi] - ts[hi - 1] <= gap {
            hi += 1;
        }
        if hi - lo >= MIN_RUN {
            runs.push((lo, hi));
        }
        lo = hi;
    }
    runs
}

fn run_detrend(ts: &[f64], vs: &[f64], gap: f64, deg: usize) -> (Vec<f64>, Vec<f64>) {
    let mut dts = Vec::new();
    let mut dvs = Vec::new();
    for (lo, hi) in split_runs(ts, gap) {
        let rts = &ts[lo..hi];
        let res = fit_resid(rts, &vs[lo..hi], deg);
        dts.extend_from_slice(rts);
        dvs.extend_from_slice(&res);
    }
    (dts, dvs)
}

fn detrend(ts: &[f64], vs: &[f64], mode: Detrend, gap: f64) -> (Vec<f64>, Vec<f64>) {
    match mode {
        Detrend::None => (ts.to_vec(), vs.to_vec()),
        Detrend::GlobalLinear => (ts.to_vec(), fit_resid(ts, vs, 1)),
        Detrend::RunLinear => run_detrend(ts, vs, gap, 1),
        Detrend::RunQuadratic => run_detrend(ts, vs, gap, 2),
    }
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

fn band_peak(ts: &[f64], vs: &[f64], step: f64) -> Option<(f64, f64, f64, f64)> {
    if ts.len() < MIN_N {
        return None;
    }
    let grid = ls_grid(ts, vs, BAND_LO, BAND_HI, step);
    if grid.len() < 3 {
        return None;
    }
    let mut bi = 0usize;
    for i in 1..grid.len() {
        if grid[i].1 > grid[bi].1 {
            bi = i;
        }
    }
    let fgrid = grid[bi].0;
    let p = grid[bi].1;
    let mut pows: Vec<f64> = grid.iter().map(|g| g.1).collect();
    pows.sort_by(f64::total_cmp);
    let floor = pows[pows.len() / 2];
    if !floor.is_finite() || floor <= 0.0 {
        return None;
    }
    let ratio = p / floor;
    let finterp = if bi == 0 || bi + 1 >= grid.len() {
        fgrid
    } else {
        let pm = grid[bi - 1].1;
        let p0 = p;
        let pp = grid[bi + 1].1;
        let denom = pm - 2.0 * p0 + pp;
        if denom.abs() < 1e-300 {
            fgrid
        } else {
            let d = 0.5 * (pm - pp) / denom;
            if d.abs() > 1.0 {
                fgrid
            } else {
                fgrid + d * step
            }
        }
    };
    let amp = (2.0 * p / ts.len() as f64).sqrt();
    Some((fgrid, finterp, amp, ratio))
}

fn per_station(pasf: &[[f64; 14]], st: i64, year: Option<i64>) -> (Vec<f64>, Vec<f64>) {
    let mut ts = Vec::new();
    let mut vs = Vec::new();
    for r in pasf {
        if r[6] as i64 == st && r[13] as i64 == 3 && r[3] < 10.0 && r[8].is_finite() {
            if let Some(y) = year {
                if year_of(r[0]) != Some(y) {
                    continue;
                }
            }
            ts.push(r[0]);
            vs.push(r[8]);
        }
    }
    (ts, vs)
}

fn fmt_peak(p: Option<(f64, f64, f64, f64)>) -> String {
    match p {
        Some((fg, fi, a, r)) => format!(
            "{:.2}/{:.2} mHz (A={:.1e}, {:.1}x)",
            fg * 1e3,
            fi * 1e3,
            a,
            r
        ),
        None => "absent".to_string(),
    }
}

fn main() {
    let Some(pasf) = std::fs::read("data/spdf.gsfc.nasa.gov/pioneer10_skyfreq.bin")
        .ok()
        .and_then(|b| parse_bin(&b))
    else {
        eprintln!("PASF void — empty (0 honored)");
        return;
    };
    eprintln!("PASF: {} records", pasf.len());

    let mut census: BTreeMap<(i64, i64), usize> = BTreeMap::new();
    for r in &pasf {
        if r[3] < 10.0 {
            *census.entry((r[6] as i64, r[13] as i64)).or_insert(0) += 1;
        }
    }
    eprintln!("PASF 1-s class (station, ground_mode) census: {census:?}");

    let gaps = [300.0f64, 600.0, 1800.0, 3600.0];
    let steps = [0.00002f64, 0.00005, 0.0001];

    let mut cache: BTreeMap<(i64, i64, Detrend, i64), (Vec<f64>, Vec<f64>)> = BTreeMap::new();
    let mut get = |st: i64, year: Option<i64>, mode: Detrend, gap: f64| {
        let gkey = (gap * 1000.0).round() as i64;
        let key = (st, year.unwrap_or(-1), mode, gkey);
        if !cache.contains_key(&key) {
            let (ts, vs) = per_station(&pasf, st, year);
            let (dts, dvs) = detrend(&ts, &vs, mode, gap);
            cache.insert(key, (dts, dvs));
        }
        cache.get(&key).cloned().unwrap()
    };

    eprintln!("\n=== METHOD SWEEP (all years, per-rx interpolated peak) ===");
    for mode in [Detrend::None, Detrend::GlobalLinear] {
        let name = match mode {
            Detrend::None => "detrend=none",
            Detrend::GlobalLinear => "detrend=global-linear",
            _ => unreachable!(),
        };
        for step in steps {
            let mut line = format!("  {name} step={:.4} mHz:", step * 1e3);
            for st in STATIONS {
                let (dts, dvs) = get(st, None, mode, 600.0);
                line.push_str(&format!(
                    "  rx{st}={}",
                    fmt_peak(band_peak(&dts, &dvs, step))
                ));
            }
            eprintln!("{line}");
        }
    }
    for (mode, name) in [
        (Detrend::RunLinear, "run-linear"),
        (Detrend::RunQuadratic, "run-quadratic"),
    ] {
        for gap in gaps {
            for step in steps {
                let mut line =
                    format!("  detrend={name} gap={gap:.0}s step={:.4} mHz:", step * 1e3);
                for st in STATIONS {
                    let (dts, dvs) = get(st, None, mode, gap);
                    line.push_str(&format!(
                        "  rx{st}={}",
                        fmt_peak(band_peak(&dts, &dvs, step))
                    ));
                }
                eprintln!("{line}");
            }
        }
    }

    eprintln!("\n=== FAITHFUL PAPER METHOD (fine grid, 1988) ===");
    eprintln!("  paper (1988): rx14=45.75 rx43=51.55 rx63=47.35 mHz; (1992): rx63=46.95 mHz");
    for step in steps {
        let mut line = format!("  1988 step={:.4} mHz (grid/interp):", step * 1e3);
        for st in STATIONS {
            let (dts, dvs) = get(st, Some(1988), Detrend::RunLinear, 600.0);
            line.push_str(&format!(
                "  rx{st}={}",
                fmt_peak(band_peak(&dts, &dvs, step))
            ));
        }
        eprintln!("{line}");
    }

    eprintln!("\n=== PER-YEAR PEAKS (default method: run-linear gap 600, step 0.02) ===");
    for st in STATIONS {
        let mut line = format!("  rx{st}:");
        for y in [1988i64, 1989, 1990, 1991, 1992, 1993] {
            let (dts, dvs) = get(st, Some(y), Detrend::RunLinear, 600.0);
            let n = dts.len();
            line.push_str(&format!(
                "  {y}=n{n}:{}",
                fmt_peak(band_peak(&dts, &dvs, 0.00002))
            ));
        }
        eprintln!("{line}");
    }

    eprintln!("\n=== SLIDING WINDOWS (3000 samples, step 1500, 1988, step 0.05 mHz) ===");
    for st in STATIONS {
        let (dts, dvs) = get(st, Some(1988), Detrend::RunLinear, 600.0);
        let win = 3000usize;
        if dts.len() < win {
            eprintln!("  rx{st}: n={} — shorter than window", dts.len());
            continue;
        }
        let mut peaks: Vec<f64> = Vec::new();
        let mut lo = 0usize;
        while lo + win <= dts.len() {
            let hi = lo + win;
            if let Some((_, fi, _, _)) = band_peak(&dts[lo..hi], &dvs[lo..hi], 0.00005) {
                peaks.push(fi * 1e3);
            }
            lo += win / 2;
        }
        if peaks.is_empty() {
            eprintln!("  rx{st}: no window peaks (0 honored)");
            continue;
        }
        peaks.sort_by(f64::total_cmp);
        let pmin = peaks[0];
        let pmax = peaks[peaks.len() - 1];
        let pmed = peaks[peaks.len() / 2];
        eprintln!(
            "  rx{st}: {} windows, peak {pmin:.2}..{pmed:.2}..{pmax:.2} mHz",
            peaks.len()
        );
    }
}
