use omegaflow::archivar::spectral::civil_from_days;

const DAY_S: f64 = 86400.0;
const C: f64 = 299792458.0;
const TRANS_RATIO: f64 = 240.0 / 221.0;
const PIONEER_ANOMALY: f64 = 8.74e-10;
const F0: f64 = 2.292e9;
const TAU_Y: f64 = 126.52;
const SEED: u64 = 0x9E37_79B9_7F4A_7C15;
const N_SURR: usize = 500;
const BLOCK: usize = 16;

fn rms_about0(v: &[f64]) -> f64 {
    if v.is_empty() {
        return f64::NAN;
    }
    (v.iter().map(|x| x * x).sum::<f64>() / v.len() as f64).sqrt()
}

fn lin_fit(xs: &[f64], ys: &[f64]) -> Option<(f64, f64)> {
    let n = xs.len() as f64;
    if n < 5.0 {
        return None;
    }
    let mx = xs.iter().sum::<f64>() / n;
    let my = ys.iter().sum::<f64>() / n;
    let mut num = 0.0;
    let mut den = 0.0;
    for i in 0..xs.len() {
        num += (xs[i] - mx) * (ys[i] - my);
        den += (xs[i] - mx) * (xs[i] - mx);
    }
    if den.abs() < 1e-300 {
        return None;
    }
    let a = num / den;
    Some((a, my - a * mx))
}

fn next_rng(rng: &mut u64) -> f64 {
    *rng = rng
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);
    ((*rng >> 33) as f64) / ((u32::MAX >> 1) as f64)
}

fn block_surrogate(v: &[f64], block: usize, rng: &mut u64) -> Vec<f64> {
    let n = v.len();
    if n < 2 {
        return v.to_vec();
    }
    let block = block.clamp(1, n);
    let mut out = Vec::with_capacity(n);
    while out.len() < n {
        let start = (next_rng(rng) * n as f64) as usize % n;
        for i in 0..block {
            out.push(v[(start + i) % n]);
        }
    }
    out.truncate(n);
    out
}

fn pct(mut sorted: Vec<f64>, p: f64) -> f64 {
    if sorted.is_empty() {
        return f64::NAN;
    }
    sorted.sort_by(f64::total_cmp);
    let idx = ((sorted.len() as f64 - 1.0) * p).round() as usize;
    sorted[idx.min(sorted.len() - 1)]
}

fn jd_date(tdb: f64) -> String {
    let jd = tdb / DAY_S + 2451545.0;
    let unix_day = (jd - 2440587.5).round() as i64;
    match civil_from_days(unix_day) {
        Some((y, m, d)) => format!("{y:04}-{m:02}-{d:02}"),
        None => format!("tdb {tdb:.0} s"),
    }
}

fn read_zone_daily(path: &str) -> Option<Vec<(f64, f64, f64, f64)>> {
    let d = std::fs::read(path).ok()?;
    if d.len() < 8 || &d[0..4] != b"PNDM" {
        return None;
    }
    let cnt = u32::from_le_bytes(d[4..8].try_into().ok()?) as usize;
    if d.len() != 8 + cnt * 32 {
        return None;
    }
    let mut out = Vec::with_capacity(cnt);
    for i in 0..cnt {
        let o = 8 + i * 32;
        let t = f64::from_le_bytes(d[o..o + 8].try_into().ok()?);
        let med = f64::from_le_bytes(d[o + 8..o + 16].try_into().ok()?);
        let r = f64::from_le_bytes(d[o + 16..o + 24].try_into().ok()?);
        let nv = f64::from_le_bytes(d[o + 24..o + 32].try_into().ok()?);
        out.push((t, med, r, nv));
    }
    Some(out)
}

fn lag1_autocorr(v: &[f64]) -> f64 {
    let n = v.len();
    if n < 2 {
        return f64::NAN;
    }
    let m = v.iter().sum::<f64>() / n as f64;
    let den: f64 = v.iter().map(|x| (x - m) * (x - m)).sum();
    if den.abs() < 1e-300 {
        return f64::NAN;
    }
    let num: f64 = (0..n - 1).map(|i| (v[i] - m) * (v[i + 1] - m)).sum();
    num / den
}

fn fit_three(tc: &[f64], v: &[f64]) -> (f64, f64, f64, f64) {
    let n = v.len() as f64;
    let mv = v.iter().sum::<f64>() / n;
    let vc: Vec<f64> = v.iter().map(|x| x - mv).collect();
    let mt = tc.iter().sum::<f64>() / n;
    let tc_c: Vec<f64> = tc.iter().map(|x| x - mt).collect();

    let a_lin = lin_fit(&tc_c, &vc).map(|(a, _)| a).unwrap_or(0.0);
    let resid_lin: Vec<f64> = tc_c.iter().zip(&vc).map(|(t, x)| x - a_lin * t).collect();
    let rms_lin = rms_about0(&resid_lin);

    let x2: Vec<f64> = tc_c.iter().map(|t| t * t).collect();
    let a_q = lin_fit(&x2, &vc).map(|(a, _)| a).unwrap_or(0.0);
    let resid_q: Vec<f64> = x2.iter().zip(&vc).map(|(x, vv)| vv - a_q * x).collect();
    let rms_q = rms_about0(&resid_q);

    let tau_s = TAU_Y * 365.25 * DAY_S;
    let dec: Vec<f64> = tc.iter().map(|&t| 1.0 - (-t / tau_s).exp()).collect();
    let a_e = lin_fit(&dec, &vc).map(|(a, _)| a).unwrap_or(0.0);
    let resid_e: Vec<f64> = dec.iter().zip(&vc).map(|(d, vv)| vv - a_e * d).collect();
    let rms_e = rms_about0(&resid_e);

    (rms_lin, rms_q, rms_e, a_lin)
}

fn run(name: &str) {
    let path = format!("data/{name}_navio_subkhz_zone_daily.bin");
    let Some(daily) = read_zone_daily(&path) else {
        eprintln!("{name}: zone daily bin void/parse void ({path}) — 0 honored");
        return;
    };
    let n_in = daily.len();
    let ts: Vec<f64> = daily.iter().map(|d| d.0).collect();
    let vs: Vec<f64> = daily.iter().map(|d| d.1).collect();
    let rs: Vec<f64> = daily.iter().map(|d| d.2).collect();

    let abs: Vec<f64> = vs.iter().map(|x| x.abs()).collect();
    eprintln!(
        "{name}: quiet-zone basis — {n_in} daily medians ({d0}..{d1}), median |daily-med| {med:.3e} Hz, RMS {rms:.3e} Hz, |med| p50/p90/p95/p99 = {p50:.0}/{p90:.0}/{p95:.0}/{p99:.0} Hz",
        d0 = jd_date(ts[0]),
        d1 = jd_date(ts[ts.len() - 1]),
        med = pct(abs.clone(), 0.5),
        rms = rms_about0(&vs),
        p50 = pct(abs.clone(), 0.5),
        p90 = pct(abs.clone(), 0.9),
        p95 = pct(abs.clone(), 0.95),
        p99 = pct(abs.clone(), 0.99),
    );

    let p90_abs = pct(abs, 0.9);
    let p90_r = pct(rs.clone(), 0.9);
    let gate_med = 4.0 * p90_abs;
    let gate_r = 4.0 * p90_r;
    let mut mts = Vec::new();
    let mut mvs = Vec::new();
    for i in 0..n_in {
        if rs[i] > gate_r || vs[i].abs() > gate_med {
            continue;
        }
        mts.push(ts[i]);
        mvs.push(vs[i]);
    }
    let n_masked = n_in - mts.len();
    if n_masked == 0 || n_masked == n_in {
        eprintln!(
            "{name}: mask gate |med| 4×p90 = {gate_med:.3e} Hz, day-RMS 4×p90 = {gate_r:.3e} Hz — 0 or all days above it (0 honored), no mask"
        );
        return;
    }
    eprintln!(
        "{name}: mask (Deduction-10 discipline) — corrupt-day clusters (day-RMS > {gate_r:.0} Hz) and jitter-tail days (|med| > {gate_med:.0} Hz) discarded, not averaged: {n_masked} of {n_in} discarded, {n_surv} days survive",
        n_surv = mts.len()
    );

    if mts.len() < 50 {
        eprintln!(
            "{name}: masked series too thin ({}) — stays silent (0 honored)",
            mts.len()
        );
        return;
    }

    let t0 = mts[0];
    let tc: Vec<f64> = mts.iter().map(|t| (t - t0) / DAY_S).collect();
    let span_y = (tc[tc.len() - 1] - tc[0]) / 365.25;
    let ac1 = lag1_autocorr(&mvs);
    eprintln!(
        "{name}: masked series lag-1 autocorrelation {ac1:.3} over {span_y:.2} y (block null length {BLOCK} d = 2^4 covers it)"
    );

    let mv = mvs.iter().sum::<f64>() / mvs.len() as f64;
    let centered: Vec<f64> = mvs.iter().map(|x| x - mv).collect();

    let mut null_slope = Vec::with_capacity(N_SURR);
    let mut null_dq = Vec::with_capacity(N_SURR);
    let mut null_de = Vec::with_capacity(N_SURR);
    let mut rng = SEED;
    for _ in 0..N_SURR {
        let surr = block_surrogate(&centered, BLOCK, &mut rng);
        let (rl, rq, re, sl) = fit_three(&tc, &surr);
        null_slope.push(sl.abs());
        null_dq.push((rl - rq) / rl);
        null_de.push((rl - re) / rl);
    }
    let thr_slope = pct(null_slope.clone(), 0.95);
    let thr_dq = pct(null_dq.clone(), 0.95);
    let thr_de = pct(null_de.clone(), 0.95);
    eprintln!(
        "{name}: surrogate null (block bootstrap, block {BLOCK} d, {N_SURR} surrogates, seed fixed) — p95 thresholds: |linear slope| {thr_slope:.3e} Hz/d, Δ∝t² {thr_dq:.4}, Δexp {thr_de:.4}"
    );

    let (rms_lin, rms_q, rms_e, slope_lin) = fit_three(&tc, &mvs);
    let dq = (rms_lin - rms_q) / rms_lin;
    let de = (rms_lin - rms_e) / rms_lin;
    let k_phys = TRANS_RATIO / C;
    let accel = slope_lin / DAY_S / (k_phys * F0);
    let mtc = tc.iter().sum::<f64>() / tc.len() as f64;
    let sxx: f64 = tc.iter().map(|t| (t - mtc) * (t - mtc)).sum();
    let se_slope = if sxx > 0.0 {
        rms_lin / sxx.sqrt()
    } else {
        f64::NAN
    };
    eprintln!(
        "{name}: real fits — linear RMS {rms_lin:.3e}, ∝t² {rms_q:.3e}, RTG-exp τ={TAU_Y:.0}y {rms_e:.3e} Hz; Δ∝t² {dq:.4}, Δexp {de:.4}"
    );
    eprintln!(
        "{name}: linear slope {slope_lin:.3e} Hz/d (naive SE {se_slope:.3e}, |slope|/SE {sig:.2}σ) → {accel:.3e} m/s² = {ratio:.2}× anomaly (negative sunward)",
        sig = slope_lin.abs() / se_slope,
        ratio = accel / PIONEER_ANOMALY
    );

    let drift_resolved = slope_lin.abs() > thr_slope;
    let quad_sig = dq > thr_dq;
    let exp_sig = de > thr_de;
    let outcome = if !drift_resolved {
        "no preference (limit) — the drift is not resolved, so no form is characterized"
    } else if quad_sig && dq > de {
        "∝t² preferred (force signature)"
    } else if exp_sig && de > dq {
        "RTG-exp preferred (thermal decay, τ=126.5 y, telemetry-free)"
    } else {
        "no preference (limit)"
    };
    eprintln!(
        "{name}: verdict — drift resolved {drift_resolved} (|slope| {:.3e} vs threshold {:.3e} Hz/d); ∝t² {dq:.4} vs {thr_dq:.4}, exp {de:.4} vs {thr_de:.4} → {outcome}",
        slope_lin.abs(),
        thr_slope
    );
}

fn main() {
    for name in ["pioneer10", "pioneer11"] {
        run(name);
    }
}
