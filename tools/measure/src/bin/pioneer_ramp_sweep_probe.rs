use omegaflow::doppler::parse_pnav_bin;
use omegaflow::spectral::civil_from_days;

const DAY_S: f64 = 86400.0;
const GAP_PASS_S: f64 = 6.0 * 3600.0;
const MIN_BLOCK: usize = 16;
const COARSE_N: usize = 41;
const FINE_N: usize = 41;

fn jd_date(tdb: f64) -> String {
    let jd = tdb / DAY_S + 2451545.0;
    let unix_day = (jd - 2440587.5).round() as i64;
    match civil_from_days(unix_day) {
        Some((y, m, d)) => format!("{y:04}-{m:02}-{d:02}"),
        None => format!("tdb {tdb:.0} s"),
    }
}

fn median(vals: &[f64]) -> f64 {
    if vals.is_empty() {
        return f64::NAN;
    }
    let mut v = vals.to_vec();
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

fn percentile(vals: &[f64], p: f64) -> f64 {
    if vals.is_empty() {
        return f64::NAN;
    }
    let mut v = vals.to_vec();
    v.sort_by(f64::total_cmp);
    let idx = ((v.len() - 1) as f64 * p).round() as usize;
    v[idx.min(v.len() - 1)]
}

fn lin_slope(dt: &[f64], dc: &[f64]) -> Option<f64> {
    if dt.len() < 2 {
        return None;
    }
    let mut sxx = 0.0;
    let mut sxy = 0.0;
    for i in 0..dt.len() {
        sxx += dt[i] * dt[i];
        sxy += dt[i] * dc[i];
    }
    if !sxx.is_finite() || sxx.abs() < 1e-300 {
        return None;
    }
    Some(sxy / sxx)
}

fn residual_rms(dt: &[f64], dc: &[f64], s: f64) -> f64 {
    let n = dt.len() as f64;
    let mut acc = 0.0;
    for i in 0..dt.len() {
        let e = dc[i] - s * dt[i];
        acc += e * e;
    }
    (acc / n).sqrt()
}

fn split_passes(pts: &[(f64, f64)], gap: f64) -> Vec<&[(f64, f64)]> {
    let mut out = Vec::new();
    let mut lo = 0usize;
    for i in 1..pts.len() {
        if pts[i].0 - pts[i - 1].0 > gap {
            out.push(&pts[lo..i]);
            lo = i;
        }
    }
    out.push(&pts[lo..]);
    out
}

struct Sweep {
    t0: f64,
    t1: f64,
    a_ls: f64,
    rms_ls: f64,
    max_rate: f64,
    coarse_min: f64,
    best_slope: f64,
    best_rms: f64,
    fine_step: f64,
}

fn sweep_pass(pass: &[(f64, f64)]) -> Option<Sweep> {
    let n = pass.len();
    if n < MIN_BLOCK {
        return None;
    }
    let nf = n as f64;
    let t_mean = pass.iter().map(|p| p.0).sum::<f64>() / nf;
    let o_mean = pass.iter().map(|p| p.1).sum::<f64>() / nf;
    let dt: Vec<f64> = pass.iter().map(|p| p.0 - t_mean).collect();
    let dc: Vec<f64> = pass.iter().map(|p| p.1 - o_mean).collect();

    let a_ls = lin_slope(&dt, &dc)?;
    let rms_ls = residual_rms(&dt, &dc, a_ls);
    if !rms_ls.is_finite() {
        return None;
    }

    let mut rates: Vec<f64> = Vec::new();
    for i in 1..n {
        let d = pass[i].0 - pass[i - 1].0;
        if d.is_finite() && d > 0.0 {
            let r = (pass[i].1 - pass[i - 1].1) / d;
            if r.is_finite() {
                rates.push(r.abs());
            }
        }
    }
    let max_rate = percentile(&rates, 1.0);
    if !max_rate.is_finite() || max_rate <= 0.0 {
        return None;
    }

    let coarse_step = 2.0 * max_rate / (COARSE_N as f64 - 1.0);
    let mut c_min = f64::INFINITY;
    let mut s_c = 0.0f64;
    for k in 0..COARSE_N {
        let s = (k as f64 - (COARSE_N as f64 - 1.0) / 2.0) * coarse_step;
        let r = residual_rms(&dt, &dc, s);
        if r < c_min {
            c_min = r;
            s_c = s;
        }
    }

    let fine_step = 2.0 * coarse_step / (FINE_N as f64 - 1.0);
    let mut b_rms = f64::INFINITY;
    let mut s_b = s_c;
    for k in 0..FINE_N {
        let s = s_c + (k as f64 - (FINE_N as f64 - 1.0) / 2.0) * fine_step;
        let r = residual_rms(&dt, &dc, s);
        if r < b_rms {
            b_rms = r;
            s_b = s;
        }
    }

    Some(Sweep {
        t0: pass[0].0,
        t1: pass[n - 1].0,
        a_ls,
        rms_ls,
        max_rate,
        coarse_min: s_c,
        best_slope: s_b,
        best_rms: b_rms,
        fine_step,
    })
}

fn report_largest(name: &str, pass: &[(f64, f64)], sw: &Sweep) {
    let n = pass.len();
    let nf = n as f64;
    let t_mean = pass.iter().map(|p| p.0).sum::<f64>() / nf;
    let o_mean = pass.iter().map(|p| p.1).sum::<f64>() / nf;
    eprintln!(
        "{name}: largest pass (n={n}, {}..{}): LS slope {:.3e} Hz/s (resid RMS {:.3e} Hz); coarse sweep +/-{:.3e} Hz/s ({COARSE_N} points) -> {:.3e} Hz/s; fine sweep ({FINE_N} points, step {:.3e}) -> {:.3e} Hz/s, resid RMS {:.3e} Hz",
        jd_date(sw.t0),
        jd_date(sw.t1),
        sw.a_ls,
        sw.rms_ls,
        sw.max_rate,
        sw.coarse_min,
        sw.fine_step,
        sw.best_slope,
        sw.best_rms
    );

    let mut resid: Vec<f64> = Vec::new();
    for i in 0..n {
        let d = pass[i].0 - t_mean;
        let c = pass[i].1 - o_mean;
        resid.push(c - sw.best_slope * d);
    }
    let r = {
        let acc = resid.iter().map(|x| x * x).sum::<f64>();
        (acc / nf).sqrt()
    };
    let abs: Vec<f64> = resid.iter().map(|x| x.abs()).collect();
    let mabs = median(&abs);
    let maxabs = abs.iter().fold(0.0f64, |a, &b| a.max(b));
    eprintln!(
        "{name}: largest-pass residuum at best slope — RMS {r:.3e} Hz, median|r| {mabs:.3e} Hz, max|r| {maxabs:.3e} Hz"
    );

    let mut idx: Vec<usize> = (0..n).collect();
    idx.sort_by(|&a, &b| resid[b].abs().total_cmp(&resid[a].abs()));
    for &i in idx.iter().take(5) {
        eprintln!(
            "{name}:   {date} resid {resid:.3e} Hz (obs {obs:.3e} Hz)",
            date = jd_date(pass[i].0),
            resid = resid[i],
            obs = pass[i].1
        );
    }
}

fn run(name: &str) {
    let path = format!("data/spdf.gsfc.nasa.gov/{name}_navio.bin");
    let Ok(bytes) = std::fs::read(&path) else {
        eprintln!("{name}: pnav bin void ({path})");
        return;
    };
    let Some(records) = parse_pnav_bin(&bytes) else {
        eprintln!("{name}: pnav bin parse void");
        return;
    };

    let mut n12 = 0usize;
    let mut n13 = 0usize;
    let mut n_other = 0usize;
    for r in &records {
        match r[4] as i64 {
            12 => n12 += 1,
            13 => n13 += 1,
            _ => n_other += 1,
        }
    }

    let mut pts: Vec<(f64, f64)> = Vec::new();
    let mut skipped_nonfinite = 0usize;
    let mut skipped_nonpositive = 0usize;
    for r in &records {
        let t = r[0];
        let o = r[1];
        if !t.is_finite() || !o.is_finite() {
            skipped_nonfinite += 1;
            continue;
        }
        if o <= 0.0 {
            skipped_nonpositive += 1;
            continue;
        }
        pts.push((t, o));
    }
    if pts.is_empty() {
        eprintln!("{name}: no plausible OBSVBL samples — stays silent (0 honored)");
        return;
    }
    pts.sort_by(|a, b| a.0.total_cmp(&b.0));
    let t0 = pts[0].0;
    let t1 = pts[pts.len() - 1].0;
    let span_y = (t1 - t0) / DAY_S / 365.25;
    let passes = split_passes(&pts, GAP_PASS_S);
    let date0 = jd_date(t0);
    let date1 = jd_date(t1);
    eprintln!(
        "{name}: {recs} records (DTYPE two-way {n12}, three-way {n13}, other {n_other}) -> {n} plausible OBSVBL samples ({nf} non-finite skipped, {np} non-positive skipped), {date0}..{date1} ({span_y:.2} y), {p} passes",
        recs = records.len(),
        n = pts.len(),
        nf = skipped_nonfinite,
        np = skipped_nonpositive,
        p = passes.len()
    );

    let mut a_ls_all: Vec<f64> = Vec::new();
    let mut rms_ls_all: Vec<f64> = Vec::new();
    let mut best_rms_all: Vec<f64> = Vec::new();
    let mut agree = 0usize;
    let mut used = 0usize;
    let mut largest: Option<(&[(f64, f64)], Sweep)> = None;
    for p in &passes {
        let Some(sw) = sweep_pass(p) else {
            continue;
        };
        used += 1;
        if (sw.best_slope - sw.a_ls).abs() <= sw.fine_step {
            agree += 1;
        }
        a_ls_all.push(sw.a_ls);
        rms_ls_all.push(sw.rms_ls);
        best_rms_all.push(sw.best_rms);
        let larger = match &largest {
            None => true,
            Some((lp, _)) => p.len() > lp.len(),
        };
        if larger {
            largest = Some((p, sw));
        }
    }
    if used == 0 {
        eprintln!("{name}: no pass reached {MIN_BLOCK} samples — stays silent (0 honored)");
        return;
    }
    let med_a = median(&a_ls_all);
    let med_rl = median(&rms_ls_all);
    let med_rb = median(&best_rms_all);
    eprintln!(
        "{name}: ramp sweep over {used} passes (>= {MIN_BLOCK} samples) — LS slope median {med_a:.3e} Hz/s, LS resid RMS median {med_rl:.3e} Hz, sweep best resid RMS median {med_rb:.3e} Hz; sweep best within one fine step of LS slope in {agree}/{used} passes"
    );
    if let Some((p, sw)) = &largest {
        report_largest(name, p, sw);
    }
}

fn main() {
    for name in ["pioneer10", "pioneer11"] {
        run(name);
    }
}
