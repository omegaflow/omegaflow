use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufReader, Read};

use omegaflow::spectral::civil_from_days;

const DAY_S: f64 = 86400.0;
const LOCK_HZ: f64 = 1.0e3;
const FLOOR_AGC: i64 = -2560;
const LOUD_HZ: f64 = 1.0;
const MIN_CELL: usize = 30;
const RUN_GAP_S: f64 = 600.0;
const TRIO: [i64; 3] = [14, 43, 63];
const LAG_SCAN: i64 = 5;
const QUART: usize = 4;
const TOP_K: usize = 8;

#[derive(Clone, Copy)]
struct Sample {
    t: f64,
    resid: f64,
}

#[derive(Clone, Copy)]
struct Cell {
    mode: i64,
    a: i64,
    b: i64,
    lo: f64,
    hi: f64,
    na: usize,
    nb: usize,
    rmsa: f64,
    rmsb: f64,
}

fn unix_day(tdb: f64) -> i64 {
    (tdb / DAY_S + 10957.5).round() as i64
}

fn civil(day: i64) -> String {
    match civil_from_days(day) {
        Some((y, m, d)) => format!("{y:04}-{m:02}-{d:02}"),
        None => format!("day {day}"),
    }
}

fn fmt_utc(tdb: f64) -> String {
    let unix = tdb + 10957.5 * DAY_S;
    let rem = unix.rem_euclid(DAY_S);
    let h = (rem / 3600.0) as i64;
    let m = ((rem % 3600.0) / 60.0) as i64;
    let s = (rem % 60.0) as i64;
    format!("{h:02}:{m:02}:{s:02}")
}

fn load() -> Option<(BTreeMap<(i64, i64), Vec<Sample>>, usize)> {
    let file = File::open("data/pds-ppi.igpp.ucla.edu/galileo_resid.bin").ok()?;
    let mut reader = BufReader::new(file);
    let mut magic = [0u8; 4];
    reader.read_exact(&mut magic).ok()?;
    if &magic != b"GASR" {
        return None;
    }
    let mut count_buf = [0u8; 4];
    reader.read_exact(&mut count_buf).ok()?;
    let mut per: BTreeMap<(i64, i64), Vec<Sample>> = BTreeMap::new();
    let mut rec = [0u8; 64];
    let mut kept = 0usize;
    loop {
        if reader.read_exact(&mut rec).is_err() {
            break;
        }
        let mut r = [0.0f64; 8];
        for (k, slot) in r.iter_mut().enumerate() {
            let mut buf = [0u8; 8];
            buf.copy_from_slice(&rec[k * 8..k * 8 + 8]);
            *slot = f64::from_le_bytes(buf);
        }
        let st = r[2] as i64;
        let mode = r[3] as i64;
        if !TRIO.contains(&st) || !(1..=4).contains(&mode) {
            continue;
        }
        let resid = r[1];
        if !resid.is_finite() || resid.abs() > LOCK_HZ {
            continue;
        }
        if r[7] as i64 != FLOOR_AGC {
            continue;
        }
        per.entry((mode, st)).or_insert_with(Vec::new).push(Sample { t: r[0], resid });
        kept += 1;
    }
    for v in per.values_mut() {
        v.sort_by(|x, y| x.t.total_cmp(&y.t));
    }
    Some((per, kept))
}

fn runs(v: &[Sample]) -> Vec<(usize, usize)> {
    let mut outr: Vec<(usize, usize)> = Vec::new();
    if v.is_empty() {
        return outr;
    }
    let mut s = 0usize;
    for i in 1..v.len() {
        if v[i].t - v[i - 1].t > RUN_GAP_S {
            outr.push((s, i - 1));
            s = i;
        }
    }
    outr.push((s, v.len() - 1));
    outr
}

fn mean_of(v: &[Sample]) -> f64 {
    if v.is_empty() {
        return 0.0;
    }
    v.iter().map(|x| x.resid).sum::<f64>() / v.len() as f64
}

fn rms_about_mean(v: &[Sample], mean: f64) -> f64 {
    if v.is_empty() {
        return 0.0;
    }
    let ss: f64 = v.iter().map(|x| (x.resid - mean) * (x.resid - mean)).sum();
    (ss / v.len() as f64).sqrt()
}

fn med_dt(v: &[Sample]) -> Option<f64> {
    if v.len() < 2 {
        return None;
    }
    let mut ds: Vec<f64> = v.windows(2).map(|w| w[1].t - w[0].t).collect();
    ds.sort_by(f64::total_cmp);
    Some(ds[ds.len() / 2])
}

#[derive(Clone, Copy)]
struct Pair {
    ta: f64,
    tb: f64,
    ra: f64,
    rb: f64,
}

fn match_pairs(a: &[Sample], b: &[Sample], tol: f64) -> Vec<Pair> {
    let mut out: Vec<Pair> = Vec::new();
    let mut used = vec![false; a.len()];
    for &sb in b {
        let start = a.partition_point(|x| x.t < sb.t - tol);
        let mut j = start;
        let mut best: Option<(usize, f64)> = None;
        while j < a.len() && a[j].t <= sb.t + tol {
            if !used[j] {
                let d = (a[j].t - sb.t).abs();
                if best.map_or(true, |(_, bd)| d < bd) {
                    best = Some((j, d));
                }
            }
            j += 1;
        }
        if let Some((j, _)) = best {
            used[j] = true;
            out.push(Pair { ta: a[j].t, tb: sb.t, ra: a[j].resid, rb: sb.resid });
        }
    }
    out.sort_by(|x, y| x.ta.total_cmp(&y.ta));
    out
}

fn median_pair_dt(pairs: &[Pair]) -> f64 {
    if pairs.is_empty() {
        return f64::NAN;
    }
    let mut ds: Vec<f64> = pairs.iter().map(|p| (p.ta - p.tb).abs()).collect();
    ds.sort_by(f64::total_cmp);
    ds[ds.len() / 2]
}

fn pearson(x: &[f64], y: &[f64]) -> f64 {
    let n = x.len();
    if n < 2 {
        return f64::NAN;
    }
    let mx = x.iter().sum::<f64>() / n as f64;
    let my = y.iter().sum::<f64>() / n as f64;
    let mut sxx = 0.0;
    let mut syy = 0.0;
    let mut sxy = 0.0;
    for i in 0..n {
        let dx = x[i] - mx;
        let dy = y[i] - my;
        sxx += dx * dx;
        syy += dy * dy;
        sxy += dx * dy;
    }
    if sxx == 0.0 || syy == 0.0 {
        return f64::NAN;
    }
    sxy / (sxx.sqrt() * syy.sqrt())
}

fn quantile(v: &[f64], q: f64) -> f64 {
    if v.is_empty() {
        return f64::NAN;
    }
    let mut s: Vec<f64> = v.to_vec();
    s.sort_by(f64::total_cmp);
    let idx = ((s.len() - 1) as f64 * q).round() as usize;
    s[idx]
}

fn top_abs(v: &[Sample], k: usize) -> Vec<Sample> {
    let mut s: Vec<Sample> = v.to_vec();
    s.sort_by(|x, y| y.resid.abs().total_cmp(&x.resid.abs()));
    s.truncate(k);
    s
}

fn quart_loud(v: &[Sample], mean: f64, thr: f64, lo: f64, hi: f64) -> [usize; QUART] {
    let mut q = [0usize; QUART];
    for s in v {
        if (s.resid - mean).abs() > thr {
            let qn = (((s.t - lo) / (hi - lo)) * QUART as f64).floor() as usize;
            if qn < QUART {
                q[qn] += 1;
            }
        }
    }
    q
}

fn main() {
    let report_path = match std::env::args().skip(1).find(|a| !a.starts_with('-')) {
        Some(p) => p,
        None => "tmp/galileo_beide_laut_simultan_report.txt".to_string(),
    };
    let Some((per, kept)) = load() else {
        println!("galileo_beide_laut_simultan: resid parse void");
        return;
    };
    let mut out: Vec<String> = Vec::new();
    out.push("galileo both-loud simultaneous (station,station) cells: common-cause or independent test".to_string());
    out.push(format!("floor trio non-lock AGC samples kept: {kept}"));
    out.push("binding: floor strength == -2560, |resid| <= 1000 Hz, station in {14,43,63}, mode 1..4".to_string());
    out.push("run = maximal sample run within one (mode, station) with tdb gap <= 600 s".to_string());
    out.push("simultaneous cell = tdb overlap of one run at station A and one run at station B in the SAME mode".to_string());
    out.push("cell RMS about the cell mean (reference loudness threshold 1 Hz); robust = nA >= 30 AND nB >= 30".to_string());
    out.push("alignment: cadence 1.000 s at both stations; grid + matched pairs tol 0.75 s".to_string());

    let mut cells: Vec<Cell> = Vec::new();
    for mode in 1..=4i64 {
        for ia in 0..3 {
            for ib in (ia + 1)..3 {
                let a = TRIO[ia];
                let b = TRIO[ib];
                let (Some(va), Some(vb)) = (per.get(&(mode, a)), per.get(&(mode, b))) else {
                    continue;
                };
                let ra = runs(va);
                let rb = runs(vb);
                for &(sa, ea) in &ra {
                    for &(sb, eb) in &rb {
                        let lo = va[sa].t.max(vb[sb].t);
                        let hi = va[ea].t.min(vb[eb].t);
                        if hi <= lo {
                            continue;
                        }
                        let wa: Vec<Sample> = va[sa..=ea].iter().copied().filter(|x| x.t >= lo && x.t <= hi).collect();
                        let wb: Vec<Sample> = vb[sb..=eb].iter().copied().filter(|x| x.t >= lo && x.t <= hi).collect();
                        if wa.is_empty() || wb.is_empty() {
                            continue;
                        }
                        let na = wa.len();
                        let nb = wb.len();
                        let rmsa = rms_about_mean(&wa, mean_of(&wa));
                        let rmsb = rms_about_mean(&wb, mean_of(&wb));
                        cells.push(Cell { mode, a, b, lo, hi, na, nb, rmsa, rmsb });
                    }
                }
            }
        }
    }
    cells.sort_by(|x, y| {
        x.mode
            .cmp(&y.mode)
            .then(x.lo.total_cmp(&y.lo))
            .then(x.a.cmp(&y.a))
            .then(x.b.cmp(&y.b))
    });

    out.push(format!("\nsimultaneous cells total: {}", cells.len()));
    let robust: Vec<Cell> = cells
        .iter()
        .copied()
        .filter(|c| c.na >= MIN_CELL && c.nb >= MIN_CELL)
        .collect();
    out.push(format!(
        "robust simultaneous cells (nA >= {MIN_CELL} AND nB >= {MIN_CELL}): {}",
        robust.len()
    ));
    let both_loud: Vec<Cell> = robust
        .iter()
        .copied()
        .filter(|c| c.rmsa >= LOUD_HZ && c.rmsb >= LOUD_HZ)
        .collect();
    out.push(format!(
        "robust both-loud cells (cell RMS >= {LOUD_HZ} Hz at BOTH stations): {}",
        both_loud.len()
    ));
    for (i, c) in both_loud.iter().enumerate() {
        let day = unix_day((c.lo + c.hi) * 0.5);
        out.push(format!(
            "  [{}] {} mode {} st{} vs st{} {} .. {} UTC | nA {} nB {} | rmsA {:.4} rmsB {:.4} Hz",
            i + 1,
            civil(day),
            c.mode,
            c.a,
            c.b,
            fmt_utc(c.lo),
            fmt_utc(c.hi),
            c.na,
            c.nb,
            c.rmsa,
            c.rmsb
        ));
    }

    let both_all: Vec<Cell> = cells
        .iter()
        .copied()
        .filter(|c| c.rmsa >= LOUD_HZ && c.rmsb >= LOUD_HZ)
        .collect();
    out.push(format!(
        "all both-loud cells incl. sub-robust (nA>=1 AND nB>=1): {}",
        both_all.len()
    ));
    for (i, c) in both_all.iter().enumerate() {
        let day = unix_day((c.lo + c.hi) * 0.5);
        out.push(format!(
            "  [{}] {} mode {} st{} vs st{} {} .. {} UTC | nA {} nB {} | rmsA {:.4} rmsB {:.4} Hz | robust {}",
            i + 1,
            civil(day),
            c.mode,
            c.a,
            c.b,
            fmt_utc(c.lo),
            fmt_utc(c.hi),
            c.na,
            c.nb,
            c.rmsa,
            c.rmsb,
            c.na >= MIN_CELL && c.nb >= MIN_CELL
        ));
    }

    for (i, c) in both_all.iter().enumerate() {
        let day = unix_day((c.lo + c.hi) * 0.5);
        let (Some(pa), Some(pb)) = (per.get(&(c.mode, c.a)), per.get(&(c.mode, c.b))) else {
            continue;
        };
        let va: Vec<Sample> = pa.iter().copied().filter(|x| x.t >= c.lo && x.t <= c.hi).collect();
        let vb: Vec<Sample> = pb.iter().copied().filter(|x| x.t >= c.lo && x.t <= c.hi).collect();
        out.push(format!(
            "\n=== both-loud cell {}: mode {} st{} vs st{} civil {} | {} .. {} UTC | robust {}",
            i + 1,
            c.mode,
            c.a,
            c.b,
            civil(day),
            fmt_utc(c.lo),
            fmt_utc(c.hi),
            c.na >= MIN_CELL && c.nb >= MIN_CELL
        ));
        let ma = mean_of(&va);
        let mb = mean_of(&vb);
        let ra_min = va.iter().fold(f64::INFINITY, |acc, x| acc.min(x.resid));
        let ra_max = va.iter().fold(f64::NEG_INFINITY, |acc, x| acc.max(x.resid));
        let rb_min = vb.iter().fold(f64::INFINITY, |acc, x| acc.min(x.resid));
        let rb_max = vb.iter().fold(f64::NEG_INFINITY, |acc, x| acc.max(x.resid));
        let aa: Vec<f64> = va.iter().map(|x| x.resid.abs()).collect();
        let ab: Vec<f64> = vb.iter().map(|x| x.resid.abs()).collect();
        let c1 = aa.iter().filter(|x| **x > 1.0).count();
        let c10 = aa.iter().filter(|x| **x > 10.0).count();
        let c100 = aa.iter().filter(|x| **x > 100.0).count();
        let d1 = ab.iter().filter(|x| **x > 1.0).count();
        let d10 = ab.iter().filter(|x| **x > 10.0).count();
        let d100 = ab.iter().filter(|x| **x > 100.0).count();
        out.push(format!(
            "  st{}: n {} mean {ma:.3} Hz | resid {ra_min:.1}..{ra_max:.1} Hz | |resid| med {:.3} p90 {:.3} max {:.3} Hz | >1 Hz {c1} >10 Hz {c10} >100 Hz {c100} | cadence med {:.3} s",
            c.a,
            va.len(),
            quantile(&aa, 0.5),
            quantile(&aa, 0.9),
            aa.iter().fold(f64::NEG_INFINITY, |m, x| m.max(*x)),
            med_dt(&va).unwrap_or(f64::NAN)
        ));
        out.push(format!(
            "  st{}: n {} mean {mb:.3} Hz | resid {rb_min:.1}..{rb_max:.1} Hz | |resid| med {:.3} p90 {:.3} max {:.3} Hz | >1 Hz {d1} >10 Hz {d10} >100 Hz {d100} | cadence med {:.3} s",
            c.b,
            vb.len(),
            quantile(&ab, 0.5),
            quantile(&ab, 0.9),
            ab.iter().fold(f64::NEG_INFINITY, |m, x| m.max(*x)),
            med_dt(&vb).unwrap_or(f64::NAN)
        ));

        for qn in 0..QUART {
            let qlo = c.lo + (c.hi - c.lo) * qn as f64 / QUART as f64;
            let qhi = c.lo + (c.hi - c.lo) * (qn + 1) as f64 / QUART as f64;
            let qa: Vec<Sample> = va.iter().copied().filter(|x| x.t >= qlo && x.t < qhi).collect();
            let qb: Vec<Sample> = vb.iter().copied().filter(|x| x.t >= qlo && x.t < qhi).collect();
            out.push(format!(
                "  quart {qn}: t {}..{} | st{} n{} rmsAboutCell {:.3} | st{} n{} rmsAboutCell {:.3}",
                fmt_utc(qlo),
                fmt_utc(qhi),
                c.a,
                qa.len(),
                rms_about_mean(&qa, ma),
                c.b,
                qb.len(),
                rms_about_mean(&qb, mb)
            ));
        }

        out.push("  top |resid| samples per station (t, resid Hz):".to_string());
        let ta = top_abs(&va, TOP_K);
        let tb = top_abs(&vb, TOP_K);
        out.push(format!(
            "    st{}: {}",
            c.a,
            ta.iter()
                .map(|s| format!("{}({:.1})", fmt_utc(s.t), s.resid))
                .collect::<Vec<_>>()
                .join("  ")
        ));
        out.push(format!(
            "    st{}: {}",
            c.b,
            tb.iter()
                .map(|s| format!("{}({:.1})", fmt_utc(s.t), s.resid))
                .collect::<Vec<_>>()
                .join("  ")
        ));

        let step = match (med_dt(&va), med_dt(&vb)) {
            (Some(x), Some(y)) => x.max(y),
            (Some(x), None) => x,
            (None, Some(y)) => y,
            (None, None) => 1.0,
        };
        let step = if step > 0.0 && step.is_finite() { step } else { 1.0 };
        let nc = ((c.hi - c.lo) / step).ceil() as usize + 1;

        let mut grid_a: Vec<Option<(f64, f64)>> = vec![None; nc];
        let mut grid_b: Vec<Option<(f64, f64)>> = vec![None; nc];
        for (g, v) in [(&mut grid_a, &va), (&mut grid_b, &vb)] {
            for &s in v {
                let k = ((s.t - c.lo) / step).floor() as usize;
                if k >= nc {
                    continue;
                }
                let cc = c.lo + (k as f64 + 0.5) * step;
                let d = (s.t - cc).abs();
                if g[k].map_or(true, |x| d < x.0) {
                    g[k] = Some((d, s.resid));
                }
            }
        }
        let grid_a: Vec<Option<f64>> = grid_a.into_iter().map(|x| x.map(|v| v.1)).collect();
        let grid_b: Vec<Option<f64>> = grid_b.into_iter().map(|x| x.map(|v| v.1)).collect();

        let mut x0 = Vec::new();
        let mut y0 = Vec::new();
        for k in 0..nc {
            if let (Some(x), Some(y)) = (grid_a[k], grid_b[k]) {
                x0.push(x);
                y0.push(y);
            }
        }
        let n0 = x0.len();
        let r0 = if n0 > 0 { pearson(&x0, &y0) } else { f64::NAN };
        out.push(format!(
            "  grid step {step:.3} s, ncell {nc}, common-grid pairs {n0}, pearson r lag0 {r0:.4}"
        ));
        let mut scan: Vec<(i64, f64, usize)> = Vec::new();
        for lag in -LAG_SCAN..=LAG_SCAN {
            let mut xs = Vec::new();
            let mut ys = Vec::new();
            for k in 0..nc {
                let i = k as i64;
                let j = i + lag;
                if j < 0 || j >= nc as i64 {
                    continue;
                }
                if let (Some(x), Some(y)) = (grid_a[i as usize], grid_b[j as usize]) {
                    xs.push(x);
                    ys.push(y);
                }
            }
            let rl = if xs.len() > 1 { pearson(&xs, &ys) } else { f64::NAN };
            scan.push((lag, rl, xs.len()));
        }
        out.push(format!(
            "  lag scan r (grid shift): {}",
            scan.iter()
                .map(|(l, r, n)| format!("{l}:{r:.3}(n{n})"))
                .collect::<Vec<_>>()
                .join("  ")
        ));

        let tol = 0.75 * step;
        let pairs = match_pairs(&va, &vb, tol);
        let np = pairs.len();
        out.push(format!(
            "  matched pairs (tol {tol:.3} s): {np}, median |ta-tb| {:.4} s",
            median_pair_dt(&pairs)
        ));
        if np > 1 {
            let xa: Vec<f64> = pairs.iter().map(|p| p.ra).collect();
            let xb: Vec<f64> = pairs.iter().map(|p| p.rb).collect();
            let rp = pearson(&xa, &xb);
            let mut dx = Vec::new();
            let mut dy = Vec::new();
            for k in 1..np {
                dx.push(pairs[k].ra - pairs[k - 1].ra);
                dy.push(pairs[k].rb - pairs[k - 1].rb);
            }
            let rd = pearson(&dx, &dy);
            out.push(format!("  matched pair pearson raw {rp:.4} | first-difference pearson {rd:.4}"));

            for q in [0.9f64, 0.99f64] {
                let mabsa: Vec<f64> = va.iter().map(|x| (x.resid - ma).abs()).collect();
                let mabsb: Vec<f64> = vb.iter().map(|x| (x.resid - mb).abs()).collect();
                let thr_a = quantile(&mabsa, q);
                let thr_b = quantile(&mabsb, q);
                let la_all: Vec<bool> = va.iter().map(|x| (x.resid - ma).abs() > thr_a).collect();
                let lb_all: Vec<bool> = vb.iter().map(|x| (x.resid - mb).abs() > thr_b).collect();
                let n_la_all = la_all.iter().filter(|x| **x).count();
                let n_lb_all = lb_all.iter().filter(|x| **x).count();
                let la: Vec<bool> = pairs.iter().map(|p| (p.ra - ma).abs() > thr_a).collect();
                let lb: Vec<bool> = pairs.iter().map(|p| (p.rb - mb).abs() > thr_b).collect();
                let both = la.iter().zip(&lb).filter(|(x, y)| **x && **y).count();
                let a_only = la.iter().zip(&lb).filter(|(x, y)| **x && !**y).count();
                let b_only = la.iter().zip(&lb).filter(|(x, y)| !**x && **y).count();
                let n_la = la.iter().filter(|x| **x).count();
                let n_lb = lb.iter().filter(|x| **x).count();
                let expected = np as f64 * (n_la_all as f64 / va.len() as f64) * (n_lb_all as f64 / vb.len() as f64);
                out.push(format!(
                    "  q{q:.2}: loudA {n_la} loudB {n_lb} in {np} matched pairs | same-sample both-loud {both} (independence expected {expected:.1}) | A-only {a_only} B-only {b_only}"
                ));
                let qa_loud = quart_loud(&va, ma, thr_a, c.lo, c.hi);
                let qb_loud = quart_loud(&vb, mb, thr_b, c.lo, c.hi);
                out.push(format!("    loud-in-quart A {:?} B {:?}", qa_loud, qb_loud));
            }
        }
    }

    let text = out.join("\n");
    println!("{text}");
    let _ = std::fs::write(&report_path, text);
}
