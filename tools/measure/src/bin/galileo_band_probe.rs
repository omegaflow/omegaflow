use std::fs::File;
use std::io::{BufReader, Read};
use std::thread::available_parallelism;

const DAY_S: f64 = 86400.0;
const LOCK_HZ: f64 = 1.0e3;
const GAP_S: f64 = 60.0;
const MIN_SEG: usize = 120;
const FLO: f64 = 0.030;
const FHI: f64 = 0.070;
const STEP: f64 = 0.00005;
const MIR_LO: f64 = 0.044;
const MIR_HI: f64 = 0.056;
const REFS: [f64; 3] = [0.04575, 0.05155, 0.04735];

fn load_records(path: &str) -> Option<Vec<[f64; 8]>> {
    let mut f = BufReader::new(File::open(path).ok()?);
    let mut head = [0u8; 8];
    f.read_exact(&mut head).ok()?;
    if &head[0..4] != b"GASR" {
        return None;
    }
    let count = u32::from_le_bytes([head[4], head[5], head[6], head[7]]) as usize;
    let mut out: Vec<[f64; 8]> = Vec::with_capacity(count);
    let mut buf = [0u8; 64];
    for _ in 0..count {
        f.read_exact(&mut buf).ok()?;
        let mut r = [0.0f64; 8];
        for k in 0..8 {
            let mut b = [0u8; 8];
            b.copy_from_slice(&buf[k * 8..k * 8 + 8]);
            r[k] = f64::from_le_bytes(b);
        }
        out.push(r);
    }
    Some(out)
}

fn median(v: &mut [f64]) -> f64 {
    v.sort_by(f64::total_cmp);
    if v.is_empty() {
        return f64::NAN;
    }
    v[v.len() / 2]
}

fn jd_date(tdb: f64) -> String {
    let jd = tdb / DAY_S + 2451545.0;
    let unix_day = (jd - 2440587.5).round() as i64;
    match omegaflow::spectral::civil_from_days(unix_day) {
        Some((y, m, d)) => format!("{y:04}-{m:02}-{d:02}"),
        None => format!("tdb {tdb:.0} s"),
    }
}

fn ls_power(times: &[f64], vals: &[f64], fref: f64) -> f64 {
    let m = times.len() as f64;
    let vsum = vals.iter().sum::<f64>() / m;
    let mut s = 0.0;
    let mut c = 0.0;
    for &t in times {
        let ph = std::f64::consts::TAU * fref * t;
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
        let ph = std::f64::consts::TAU * fref * t;
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
        (a * a + b * b) * m / 2.0
    } else {
        0.0
    }
}

fn ls_grid(times: &[f64], vals: &[f64], flo: f64, fhi: f64, step: f64) -> Vec<(f64, f64)> {
    let mut freqs: Vec<f64> = Vec::new();
    let mut f = flo;
    while f <= fhi + step * 0.5 {
        freqs.push(f);
        f += step;
    }
    let threads = available_parallelism().map(|n| n.get()).unwrap_or(1).min(freqs.len());
    let mut out: Vec<(f64, f64)> = freqs.iter().map(|&x| (x, 0.0)).collect();
    std::thread::scope(|scope| {
        let mut handles = Vec::new();
        let per = freqs.len().div_ceil(threads);
        for ch in 0..threads {
            let lo = ch * per;
            let hi = ((ch + 1) * per).min(freqs.len());
            if lo >= hi {
                continue;
            }
            let sub: Vec<f64> = freqs[lo..hi].to_vec();
            handles.push(scope.spawn(move || {
                let mut local: Vec<(usize, f64)> = Vec::with_capacity(hi - lo);
                for (k, fk) in sub.iter().enumerate() {
                    local.push((lo + k, ls_power(times, vals, *fk)));
                }
                local
            }));
        }
        for h in handles {
            for (idx, p) in h.join().expect("thread join") {
                out[idx] = (out[idx].0, p);
            }
        }
    });
    out
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
    let floor = median(&mut pows);
    (best.0, best.1, best.1 / floor)
}

fn peak_interp(grid: &[(f64, f64)], fi: usize) -> Option<f64> {
    if fi == 0 || fi + 1 >= grid.len() {
        return None;
    }
    let step = grid[fi].0 - grid[fi - 1].0;
    let pm = grid[fi - 1].1;
    let p0 = grid[fi].1;
    let pp = grid[fi + 1].1;
    let denom = pm - 2.0 * p0 + pp;
    if denom.abs() < 1e-300 {
        return None;
    }
    let d = 0.5 * (pm - pp) / denom;
    if d.abs() > 1.0 {
        return None;
    }
    Some(grid[fi].0 + d * step)
}

fn members(grid: &[(f64, f64)], floor: f64, gate: f64) -> Vec<(f64, f64)> {
    let mut out: Vec<(f64, f64)> = Vec::new();
    for i in 1..grid.len() - 1 {
        if grid[i].1 > grid[i - 1].1 && grid[i].1 >= grid[i + 1].1 && grid[i].1 >= gate * floor {
            let fi = peak_interp(grid, i).unwrap_or(grid[i].0);
            if out.is_empty() || (fi - out.last().unwrap().0).abs() > 0.00015 {
                out.push((fi, grid[i].1 / floor));
            }
        }
    }
    out.sort_by(|a, b| b.1.total_cmp(&a.1));
    out
}

fn detrend_blocks(ts: &[f64], vs: &[f64]) -> (Vec<f64>, Vec<f64>) {
    let mut dts: Vec<f64> = Vec::new();
    let mut dvs: Vec<f64> = Vec::new();
    let mut lo = 0usize;
    while lo < ts.len() {
        let mut hi = lo + 1;
        while hi < ts.len() && ts[hi] - ts[hi - 1] <= GAP_S {
            hi += 1;
        }
        if hi - lo >= MIN_SEG {
            let xt = &ts[lo..hi];
            let yv = &vs[lo..hi];
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
    (dts, dvs)
}

fn shape_string(grid: &[(f64, f64)], floor: f64) -> String {
    let mut parts: Vec<String> = Vec::new();
    let mut f = 0.030;
    while f <= 0.070 + 1e-9 {
        let p = grid
            .iter()
            .min_by(|a, b| (a.0 - f).abs().total_cmp(&(b.0 - f).abs()))
            .map(|(_, p)| *p)
            .unwrap_or(0.0);
        parts.push(format!("{:.0}:{:.1}", f * 1000.0, p / floor));
        f += 0.002;
    }
    parts.join(" ")
}


fn civil_year(tdb: f64) -> Option<i64> {
    let jd = tdb / DAY_S + 2451545.0;
    let unix_day = (jd - 2440587.5).round() as i64;
    omegaflow::spectral::civil_from_days(unix_day).map(|(y, _, _)| y as i64)
}

fn mirror_cell_grid(ts: &[f64], vs: &[f64]) -> (Vec<(f64, f64)>, f64) {
    let grid = ls_grid(ts, vs, MIR_LO, MIR_HI, STEP);
    let mut pows: Vec<f64> = grid.iter().map(|(_, p)| *p).collect();
    let floor = median(&mut pows);
    (grid, floor)
}

fn run_persist(recs: &[[f64; 8]]) {
    let obs: [(i64, f64, &str); 4] = [
        (14, 0.04575, "45.75"),
        (43, 0.05155, "51.55"),
        (63, 0.04735, "47.35"),
        (42, 0.05240, "52.40"),
    ];
    for &(st, rf, rname) in &obs {
        for &mode in &[2i64, 3, 1] {
            let cell: Vec<&[f64; 8]> = recs.iter().filter(|r| (r[2] as i64) == st && (r[3] as i64) == mode).collect();
            if cell.is_empty() {
                continue;
            }
            let t0 = cell[0][0];
            let t1 = cell[cell.len() - 1][0];
            let mut years: Vec<i64> = cell.iter().filter_map(|r| civil_year(r[0])).collect();
            years.sort_unstable();
            years.dedup();
            println!("persist st {st} mode {mode} ({rname} mHz) | {} records {}..{} | years {:?}", cell.len(), jd_date(t0), jd_date(t1), years);
            for y in &years {
                let yrecs: Vec<&[f64; 8]> = cell.iter().copied().filter(|r| civil_year(r[0]) == Some(*y)).collect();
                if yrecs.is_empty() {
                    continue;
                }
                let mut ts: Vec<f64> = Vec::new();
                let mut vs: Vec<f64> = Vec::new();
                for r in &yrecs {
                    if r[1].abs() <= LOCK_HZ {
                        ts.push(r[0]);
                        vs.push(r[1]);
                    }
                }
                let (dts, dvs) = detrend_blocks(&ts, &vs);
                if dts.len() < MIN_SEG {
                    continue;
                }
                let mut days: Vec<i64> = yrecs.iter().map(|r| (r[0] / DAY_S).floor() as i64).collect();
                days.sort_unstable();
                days.dedup();
                let (grid, floor) = mirror_cell_grid(&dts, &dvs);
                let (fp, _, ratio) = peak_of(&grid);
                let fpi = peak_interp(&grid, grid.iter().position(|g| g.0 == fp).unwrap_or(0)).unwrap_or(fp);
                let p_ref = grid
                    .iter()
                    .min_by(|a, b| (a.0 - rf).abs().total_cmp(&(b.0 - rf).abs()))
                    .map(|(_, p)| *p)
                    .unwrap_or(0.0);
                let win: Vec<(f64, f64)> = grid.iter().copied().filter(|(f, _)| (f - rf).abs() <= 0.0005).collect();
                let wb = members(&win, floor, 0.0).into_iter().next();
                let (fb, rb) = match wb {
                    Some((fb, rb)) => (fb, rb),
                    None => (rf, p_ref / floor.max(1e-300)),
                };
                println!("  {y}: n_scan {} days {} | peak {:.4} mHz (interp {:.4}) {:.1}x | ref {rname} {:.1}x | best in +-0.5 mHz {:.3} mHz {:.1}x", dts.len(), days.len(), fp * 1000.0, fpi * 1000.0, ratio, p_ref / floor.max(1e-300), fb * 1000.0, rb.max(0.0));
            }
        }
    }
}


fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let persist = args.iter().any(|a| a == "--persist");
    let path = args.iter().find(|a| !a.starts_with('-')).cloned().unwrap_or_else(|| "data/galileo_resid.bin".to_string());
    let Some(recs) = load_records(&path) else {
        eprintln!("galileo_band_probe: {path} unreadable or not a GASR resid bin");
        return;
    };
    if recs.is_empty() {
        eprintln!("galileo_band_probe: {path} carries no records");
        return;
    }
    if persist {
        run_persist(&recs);
        return;
    }
    println!("galileo_band_probe: {} GASR records {}..{}", recs.len(), jd_date(recs[0][0]), jd_date(recs[recs.len() - 1][0]));
    println!("method: per (station, mode): lock cut |resid|>1000 Hz, per-segment linear detrend (gap 60 s, min 120), LS 30-70 mHz @ 0.05 mHz; floor = band median; members = local maxima >= 3x floor, parabolic interp; mirror numbers from the 44-56 mHz subset");

    let mut stations_present: Vec<i64> = recs.iter().map(|r| r[2] as i64).collect();
    stations_present.sort_unstable();
    stations_present.dedup();
    for &st in &[14i64, 43, 63, 42] {
        if !stations_present.contains(&st) {
            continue;
        }
        for &mode in &[2i64, 3, 1] {
            let cell: Vec<&[f64; 8]> = recs.iter().filter(|r| (r[2] as i64) == st && (r[3] as i64) == mode).collect();
            if cell.is_empty() {
                continue;
            }
            let mut lock = 0usize;
            let mut kept_ts: Vec<f64> = Vec::new();
            let mut kept_vs: Vec<f64> = Vec::new();
            for r in &cell {
                if r[1].abs() > LOCK_HZ {
                    lock += 1;
                } else {
                    kept_ts.push(r[0]);
                    kept_vs.push(r[1]);
                }
            }
            let mut days: Vec<i64> = cell.iter().map(|r| (r[0] / DAY_S).floor() as i64).collect();
            days.sort_unstable();
            days.dedup();
            let t0 = cell[0][0];
            let t1 = cell[cell.len() - 1][0];
            let (dts, dvs) = detrend_blocks(&kept_ts, &kept_vs);
            let mut seg_lens: Vec<usize> = Vec::new();
            let mut lo = 0usize;
            while lo < kept_ts.len() {
                let mut hi = lo + 1;
                while hi < kept_ts.len() && kept_ts[hi] - kept_ts[hi - 1] <= GAP_S {
                    hi += 1;
                }
                if hi - lo >= MIN_SEG {
                    seg_lens.push(hi - lo);
                }
                lo = hi;
            }
            seg_lens.sort_unstable();
            let n_seg = seg_lens.len();
            let med_seg = if seg_lens.is_empty() { 0.0 } else { seg_lens[seg_lens.len() / 2] as f64 };
            let seg_span = if n_seg > 0 {
                let mut spans: Vec<f64> = Vec::new();
                let mut lo2 = 0usize;
                while lo2 < kept_ts.len() {
                    let mut hi2 = lo2 + 1;
                    while hi2 < kept_ts.len() && kept_ts[hi2] - kept_ts[hi2 - 1] <= GAP_S {
                        hi2 += 1;
                    }
                    if hi2 - lo2 >= MIN_SEG {
                        spans.push(kept_ts[hi2 - 1] - kept_ts[lo2]);
                    }
                    lo2 = hi2;
                }
                spans.sort_by(f64::total_cmp);
                if spans.is_empty() { 0.0 } else { spans[spans.len() / 2] }
            } else {
                0.0
            };
            if dts.len() < MIN_SEG {
                println!("cell st {st} mode {mode} | n_raw {} n_lock {} days {} span {:.0} d | {} detrended samples — no scan (0 honored)", cell.len(), lock, days.len(), (t1 - t0) / DAY_S, dts.len());
                continue;
            }
            let grid = ls_grid(&dts, &dvs, FLO, FHI, STEP);
            let mut pows: Vec<f64> = grid.iter().map(|(_, p)| *p).collect();
            let floor = median(&mut pows);
            let mir: Vec<(f64, f64)> = grid.iter().copied().filter(|(f, _)| *f >= MIR_LO && *f <= MIR_HI).collect();
            let (fm, pm, rm) = peak_of(&mir);
            let mems = members(&grid, floor, 3.0);
            let mem_txt: Vec<String> = mems
                .iter()
                .map(|(f, r)| format!("{:.3} mHz {:.1}x", f * 1000.0, r))
                .collect();
            let mut refs_txt: Vec<String> = Vec::new();
            for rf in &REFS {
                let p_at = grid
                    .iter()
                    .min_by(|a, b| (a.0 - rf).abs().total_cmp(&(b.0 - rf).abs()))
                    .map(|(_, p)| *p)
                    .unwrap_or(0.0);
                let win: Vec<(f64, f64)> = grid.iter().copied().filter(|(f, _)| (f - rf).abs() <= 0.0005).collect();
                let wbest = members(&win, floor, 0.0).into_iter().next();
                let (fb, rb) = match wbest {
                    Some((fb, rb)) => (fb, rb),
                    None => (*rf, p_at / floor),
                };
                refs_txt.push(format!("{:.2} mHz: {:.1}x floor; best in +-0.5 mHz {:.3} mHz {:.1}x", rf * 1000.0, p_at / floor.max(1e-300), fb * 1000.0, rb.max(0.0)));
            }
            println!("cell st {st} mode {mode} | n_raw {} n_lock {} n_scan {} days {} span {:.0} d seg {} med_len {:.0} med_span {:.1} h | P10-mirror 44-56 mHz peak {:.4} mHz (interp {:.4}) {:.1}x 44-56-median {:.1}x 30-70-floor", cell.len(), lock, dts.len(), days.len(), (t1 - t0) / DAY_S, n_seg, med_seg, seg_span / 3600.0, fm * 1000.0, peak_interp(&mir, mir.iter().position(|g| g.0 == fm).unwrap_or(0)).unwrap_or(fm) * 1000.0, rm, pm / floor.max(1e-300));
            println!("  refs: {}", refs_txt.join(" | "));
            println!("  members(>=3x): {}", if mem_txt.is_empty() { "none".to_string() } else { mem_txt.join(" ") });
            println!("  shape 30-70 (2-mHz grid, x30-70 floor): {}", shape_string(&grid, floor));
        }
    }
}
