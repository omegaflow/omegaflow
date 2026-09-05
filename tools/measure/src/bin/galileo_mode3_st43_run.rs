use std::collections::BTreeMap;
use std::fs;
use std::thread::available_parallelism;

const LOCK_HZ: f64 = 1.0e3;
const DAY_S: f64 = 86400.0;
const ST: i64 = 43;
const D0: i64 = 9457;
const D1: i64 = 9471;
const GAP_S: f64 = 60.0;
const MIN_SEG: usize = 120;
const FLO: f64 = 0.030;
const FHI: f64 = 0.070;
const STEP: f64 = 0.00005;
const MIR_LO: f64 = 0.044;
const MIR_HI: f64 = 0.056;
const ST43_REF_HZ: f64 = 0.05155;

fn unix_day(tdb: f64) -> i64 {
    let jd = 2451545.0 + tdb / DAY_S;
    (jd - 2440587.5).round() as i64
}

fn median(v: &[f64]) -> Option<f64> {
    if v.is_empty() {
        return None;
    }
    let mut w = v.to_vec();
    w.sort_by(f64::total_cmp);
    Some(w[w.len() / 2])
}

fn rms(v: &[f64]) -> Option<f64> {
    if v.is_empty() {
        return None;
    }
    let n = v.len() as f64;
    Some((v.iter().map(|x| x * x).sum::<f64>() / n).sqrt())
}

fn fmt_opt(v: Option<f64>) -> String {
    match v {
        Some(x) => format!("{x:.4e}"),
        None => "-".to_string(),
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

fn block_stats(ts: &[f64]) -> (usize, usize, f64, usize) {
    if ts.is_empty() {
        return (0, 0, 0.0, 0);
    }
    let mut n_blocks = 0usize;
    let mut n_kept = 0usize;
    let mut longest = 0usize;
    let mut longest_span = 0.0f64;
    let mut lo = 0usize;
    while lo < ts.len() {
        let mut hi = lo + 1;
        while hi < ts.len() && ts[hi] - ts[hi - 1] <= GAP_S {
            hi += 1;
        }
        let len = hi - lo;
        n_blocks += 1;
        if len >= MIN_SEG {
            n_kept += len;
        }
        if len > longest {
            longest = len;
            longest_span = ts[hi - 1] - ts[lo];
        }
        lo = hi;
    }
    (n_blocks, longest, longest_span, n_kept)
}

fn peak_of(grid: &[(f64, f64)]) -> (f64, f64) {
    let mut best = grid[0];
    for g in grid {
        if g.1 > best.1 {
            best = *g;
        }
    }
    best
}

fn members(grid: &[(f64, f64)], floor: f64, gate: f64) -> Vec<(f64, f64)> {
    let mut out: Vec<(f64, f64)> = Vec::new();
    for i in 1..grid.len() - 1 {
        if grid[i].1 > grid[i - 1].1 && grid[i].1 >= grid[i + 1].1 && grid[i].1 >= gate * floor {
            let f = grid[i].0;
            if out.is_empty() || (f - out.last().unwrap().0).abs() > 0.00015 {
                out.push((f, grid[i].1 / floor));
            }
        }
    }
    out.sort_by(|a, b| b.1.total_cmp(&a.1));
    out
}

fn spec_scan(label: &str, ts: &[f64], vs: &[f64]) {
    let (dts, dvs) = detrend_blocks(ts, vs);
    if dts.len() < MIN_SEG {
        println!("spec {label}: no detrendable block >= {MIN_SEG} samples (kept {}) -> no LS scan (0 honored)", dts.len());
        return;
    }
    let grid = ls_grid(&dts, &dvs, FLO, FHI, STEP);
    let pows: Vec<f64> = grid.iter().map(|(_, p)| *p).collect();
    let floor = median(&pows).expect("nonempty grid");
    let (fp, pp) = peak_of(&grid);
    let mir: Vec<(f64, f64)> = grid.iter().copied().filter(|(f, _)| *f >= MIR_LO && *f <= MIR_HI).collect();
    let (fm, pm) = peak_of(&mir);
    let mir_pows: Vec<f64> = mir.iter().map(|(_, p)| *p).collect();
    let mir_floor = median(&mir_pows).expect("nonempty mirror grid");
    let p_ref = grid
        .iter()
        .min_by(|a, b| (a.0 - ST43_REF_HZ).abs().total_cmp(&(b.0 - ST43_REF_HZ).abs()))
        .map(|(_, p)| *p)
        .unwrap_or(0.0);
    let mems = members(&grid, floor, 3.0);
    let mem_txt: Vec<String> = mems.iter().map(|(f, r)| format!("{:.3} mHz {:.1}x", f * 1000.0, r)).collect();
    println!(
        "spec {label}: n_scan {} | LS 30-70 mHz @0.05 mHz | band floor {floor:.3e} | band peak {:.4} mHz {:.1}x floor | 44-56 subwindow peak {:.4} mHz {:.1}x band-floor / {:.1}x subwindow-median | ref 51.55 mHz {:.1}x floor",
        dts.len(),
        fp * 1000.0,
        pp / floor,
        fm * 1000.0,
        pm / floor,
        pm / mir_floor,
        p_ref / floor
    );
    println!("  members(>=3x floor): {}", if mem_txt.is_empty() { "none".to_string() } else { mem_txt.join(" ") });
}

fn main() {
    let path = std::env::args()
        .skip(1)
        .find(|a| !a.starts_with('-'))
        .unwrap_or_else(|| "data/galileo_resid.bin".to_string());
    let bytes = fs::read(&path).expect("resid bin read");
    if bytes.len() < 8 || &bytes[0..4] != b"GASR" {
        println!("no GASR header");
        return;
    }
    let count = u32::from_le_bytes(bytes[4..8].try_into().expect("count")) as usize;
    if bytes.len() != 8 + count * 64 {
        println!("length mismatch");
        return;
    }
    let mut n_nan = 0usize;
    let mut n_lock = 0usize;
    let mut n_zero = 0usize;
    let mut cleaned = 0usize;
    let mut pd2: BTreeMap<i64, Vec<f64>> = BTreeMap::new();
    let mut pd3: BTreeMap<i64, Vec<f64>> = BTreeMap::new();
    let mut ser2: Vec<(f64, f64)> = Vec::new();
    let mut ser3: Vec<(f64, f64)> = Vec::new();
    let mut samp3: BTreeMap<i64, usize> = BTreeMap::new();
    for i in 0..count {
        let base = 8 + i * 64;
        let rd = |k: usize| -> f64 {
            let mut b = [0u8; 8];
            b.copy_from_slice(&bytes[base + k * 8..base + k * 8 + 8]);
            f64::from_le_bytes(b)
        };
        let resid = rd(1);
        if !resid.is_finite() {
            n_nan += 1;
            continue;
        }
        if resid.abs() > LOCK_HZ {
            n_lock += 1;
            continue;
        }
        if rd(7) == 0.0 {
            n_zero += 1;
            continue;
        }
        cleaned += 1;
        if rd(2) as i64 != ST {
            continue;
        }
        let day = unix_day(rd(0));
        if !(D0..=D1).contains(&day) {
            continue;
        }
        let mode = rd(3) as i64;
        match mode {
            2 => {
                pd2.entry(day).or_default().push(resid.abs());
                ser2.push((rd(0), resid));
            }
            3 => {
                pd3.entry(day).or_default().push(resid.abs());
                ser3.push((rd(0), resid));
                *samp3.entry((rd(6) * 10.0).round() as i64).or_insert(0usize) += 1;
            }
            _ => {}
        }
    }
    let (y0, m0, d0) = omegaflow::spectral::civil_from_days(D0).expect("day range");
    let (y1, m1, d1) = omegaflow::spectral::civil_from_days(D1).expect("day range");
    println!(
        "GASR {path}: {count} records | cleaned {cleaned} (finite, |resid|<={LOCK_HZ:.0} Hz, strength != 0), non-finite {n_nan}, lock {n_lock}, zero-strength {n_zero}"
    );
    println!(
        "selection: st {ST}, days {D0}-{D1} = {y0:04}-{m0:02}-{d0:02} .. {y1:04}-{m1:02}-{d1:02} (unix_day = round(jd-2440587.5))"
    );
    println!(
        "selected cleaned: m2 {} records ({} days), m3 {} records ({} days)",
        ser2.len(),
        pd2.len(),
        ser3.len(),
        pd3.len()
    );

    let mut alldays: Vec<i64> = pd2.keys().chain(pd3.keys()).copied().collect();
    alldays.sort_unstable();
    alldays.dedup();
    println!();
    println!("per-day noise: med = median|resid|, rms = sqrt(mean resid^2), n = cleaned samples of that mode at st{ST}");
    println!(
        "{:>5} | {:>7} {:>10} {:>10} | {:>7} {:>10} {:>10} | {:>7} {:>7}",
        "day", "m2 n", "m2 med Hz", "m2 rms Hz", "m3 n", "m3 med Hz", "m3 rms Hz", "med 3/2", "rms 3/2"
    );
    for day in &alldays {
        let v2 = pd2.get(day);
        let v3 = pd3.get(day);
        let (m2n, m2m, m2r) = match v2 {
            Some(x) => (x.len(), median(x), rms(x)),
            None => (0, None, None),
        };
        let (m3n, m3m, m3r) = match v3 {
            Some(x) => (x.len(), median(x), rms(x)),
            None => (0, None, None),
        };
        let ratm = match (m2m, m3m) {
            (Some(a), Some(b)) if a > 0.0 => format!("{:.3}", b / a),
            _ => "-".to_string(),
        };
        let ratr = match (m2r, m3r) {
            (Some(a), Some(b)) if a > 0.0 => format!("{:.3}", b / a),
            _ => "-".to_string(),
        };
        println!(
            "{day:>5} | {m2n:>7} {:>10} {:>10} | {m3n:>7} {:>10} {:>10} | {:>7} {:>7}",
            fmt_opt(m2m),
            fmt_opt(m2r),
            fmt_opt(m3m),
            fmt_opt(m3r),
            ratm,
            ratr
        );
    }

    let pool2: Vec<f64> = pd2.values().flatten().copied().collect();
    let pool3: Vec<f64> = pd3.values().flatten().copied().collect();
    let dmed2: Vec<f64> = pd2.values().filter_map(|x| median(x)).collect();
    let dmed3: Vec<f64> = pd3.values().filter_map(|x| median(x)).collect();
    let d_rms2: Vec<f64> = pd2.values().filter_map(|x| rms(x)).collect();
    let d_rms3: Vec<f64> = pd3.values().filter_map(|x| rms(x)).collect();
    let ratio = |a: Option<f64>, b: Option<f64>| -> String {
        match (a, b) {
            (Some(x), Some(y)) if x > 0.0 => format!("{:.3}", y / x),
            _ => "-".to_string(),
        }
    };
    println!();
    println!("overall floor over the run (pooled samples):");
    println!(
        "  m2 n {} | pooled med {:.4e} Hz | pooled rms {:.4e} Hz",
        pool2.len(),
        median(&pool2).expect("m2 nonempty"),
        rms(&pool2).expect("m2 nonempty")
    );
    println!(
        "  m3 n {} | pooled med {:.4e} Hz | pooled rms {:.4e} Hz",
        pool3.len(),
        median(&pool3).expect("m3 nonempty"),
        rms(&pool3).expect("m3 nonempty")
    );
    println!(
        "  floor ratio m3/m2: pooled med {} | pooled rms {}",
        ratio(median(&pool2), median(&pool3)),
        ratio(rms(&pool2), rms(&pool3))
    );
    println!(
        "  day-level (median over days of per-day med): m2 {} | m3 {} | ratio {}",
        fmt_opt(median(&dmed2)),
        fmt_opt(median(&dmed3)),
        ratio(median(&dmed2), median(&dmed3))
    );
    println!(
        "  day-level (median over days of per-day rms): m2 {} | m3 {} | ratio {}",
        fmt_opt(median(&d_rms2)),
        fmt_opt(median(&d_rms3)),
        ratio(median(&d_rms2), median(&d_rms3))
    );

    ser2.sort_by(|a, b| a.0.total_cmp(&b.0));
    ser3.sort_by(|a, b| a.0.total_cmp(&b.0));
    let ts2: Vec<f64> = ser2.iter().map(|x| x.0).collect();
    let vs2: Vec<f64> = ser2.iter().map(|x| x.1).collect();
    let ts3: Vec<f64> = ser3.iter().map(|x| x.0).collect();
    let vs3: Vec<f64> = ser3.iter().map(|x| x.1).collect();
    println!();
    println!("cadence / block structure (block = samples with inter-sample gap <= {GAP_S:.0} s):");
    let (nb2, lg2, sp2, nk2) = block_stats(&ts2);
    let (nb3, lg3, sp3, nk3) = block_stats(&ts3);
    println!(
        "  m2: {} blocks | longest {} samples ({:.1} h) | samples in blocks >= {MIN_SEG}: {nk2}",
        nb2,
        lg2,
        sp2 / 3600.0
    );
    println!(
        "  m3: {} blocks | longest {} samples ({:.1} h) | samples in blocks >= {MIN_SEG}: {nk3}",
        nb3,
        lg3,
        sp3 / 3600.0
    );
    let sampler_txt: Vec<String> = samp3.iter().map(|(k, v)| format!("{:.1} s x{v}", *k as f64 / 10.0)).collect();
    println!("  m3 sampler_s histogram (x0.1 rounded): {}", sampler_txt.join(" "));

    println!();
    spec_scan("st43 m2 days 9457-9471 (context, same method)", &ts2, &vs2);
    spec_scan("st43 m3 days 9457-9471", &ts3, &vs3);
}
