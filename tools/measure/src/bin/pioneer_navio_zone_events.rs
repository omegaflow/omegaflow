use omegaflow::archivar::spectral::civil_from_days;

const DAY_S: f64 = 86400.0;
const QUIET_HZ: f64 = 5.0;
const STEP_GAP_DAYS: f64 = 3.0;

fn rms_about0(v: &[f64]) -> f64 {
    if v.is_empty() {
        return f64::NAN;
    }
    (v.iter().map(|x| x * x).sum::<f64>() / v.len() as f64).sqrt()
}

fn pct(sorted: &[f64], p: f64) -> f64 {
    if sorted.is_empty() {
        return f64::NAN;
    }
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
    Some((num / den, my - num / den * mx))
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

fn event_scan(name: &str, mts: &[f64], mvs: &[f64]) {
    const SEED: u64 = 0x9E37_79B9_7F4A_7C15;
    const N_SURR: usize = 500;
    const BLOCK: usize = 16;
    const MIN_EXC_DAYS: usize = 2;
    const GAP_D: f64 = 2.0;

    let n = mts.len();
    let t0 = mts[0];
    let tc: Vec<f64> = mts.iter().map(|t| (t - t0) / DAY_S).collect();
    let a = lin_fit(&tc, mvs).map(|(a, _)| a).unwrap_or(0.0);
    let mtc = tc.iter().sum::<f64>() / tc.len() as f64;
    let resid: Vec<f64> = tc.iter().zip(mvs).map(|(t, v)| v - a * (t - mtc)).collect();
    let mres = resid.iter().sum::<f64>() / n as f64;
    let resid_c: Vec<f64> = resid.iter().map(|x| x - mres).collect();

    let qidx: Vec<usize> = (0..n).filter(|&i| resid[i].abs() <= QUIET_HZ).collect();
    let qvals: Vec<f64> = qidx.iter().map(|&i| resid[i]).collect();
    let qn = qvals.len() as f64;
    if qidx.len() < 20 {
        eprintln!("{name}: event scan — too few quiet days, stays silent (0 honored)");
        return;
    }
    let qmean = qvals.iter().sum::<f64>() / qn;
    let qvar = qvals.iter().map(|x| (x - qmean) * (x - qmean)).sum::<f64>() / qn;
    let qsd = qvar.sqrt();
    let exc = 3.0 * qsd;
    eprintln!(
        "{name}: event scan (pre-registered) — quiet-day sd {qsd:.3e} Hz, excursion gate E = 3σ = {exc:.3e} Hz; a quiet-day |residual| > E is an excursion day, runs of ≥{MIN_EXC_DAYS} contiguous (gap ≤ {GAP_D} d) with return are transit candidates"
    );

    let elevated: Vec<bool> = resid
        .iter()
        .map(|r| r.abs() > exc && r.abs() <= QUIET_HZ)
        .collect();
    let count_elev = elevated.iter().filter(|b| **b).count();
    eprintln!(
        "{name}: elevated quiet-day residuals (|resid| > {exc:.2} Hz): {count_elev} of {n} masked days"
    );

    let mut runs: Vec<(usize, usize)> = Vec::new();
    let mut i = 0usize;
    while i < n {
        if !elevated[i] {
            i += 1;
            continue;
        }
        let mut j = i;
        while j + 1 < n && elevated[j + 1] && (mts[j + 1] - mts[j]) / DAY_S <= GAP_D {
            j += 1;
        }
        runs.push((i, j));
        i = j + 1;
    }
    let multi = runs
        .iter()
        .filter(|(i0, j0)| j0 - i0 + 1 >= MIN_EXC_DAYS)
        .count();
    for (i0, j0) in &runs {
        let len = j0 - i0 + 1;
        let peak = (*i0..=*j0).map(|k| resid[k].abs()).fold(0.0f64, f64::max);
        let returns = j0 + 1 < n && resid[j0 + 1].abs() <= exc;
        eprintln!(
            "{name}: excursion run {d0}..{d1} ({len} d, peak {peak:.2} Hz, returns-after {returns})",
            d0 = jd_date(mts[*i0]),
            d1 = jd_date(mts[*j0])
        );
    }
    let runs_total = runs.len();
    eprintln!(
        "{name}: excursion runs: {runs_total} total, {multi} multi-day (≥{MIN_EXC_DAYS} d) — transit candidates (away a few days and back)",
        multi = multi
    );

    let real_max_run = runs.iter().map(|(i0, j0)| j0 - i0 + 1).max().unwrap_or(0);
    let elev_bool = |res: &[f64], thr: f64| -> Vec<bool> {
        res.iter()
            .map(|r| r.abs() > thr && r.abs() <= QUIET_HZ)
            .collect()
    };
    let max_run_of = |res: &[f64], thr: f64| -> usize {
        let e = elev_bool(res, thr);
        let mut mx = 0usize;
        let mut cur = 0usize;
        for b in &e {
            if *b {
                cur += 1;
                mx = mx.max(cur);
            } else {
                cur = 0;
            }
        }
        mx
    };
    let mut null_max = Vec::with_capacity(N_SURR);
    let mut rng = SEED;
    for _ in 0..N_SURR {
        let surr = block_surrogate(&resid_c, BLOCK, &mut rng);
        null_max.push(max_run_of(&surr, exc) as f64);
    }
    null_max.sort_by(f64::total_cmp);
    let p95 = null_max[(N_SURR as f64 * 0.95) as usize - 1];
    let transit = runs
        .iter()
        .filter(|(i0, j0)| j0 - i0 + 1 >= MIN_EXC_DAYS)
        .count();
    eprintln!(
        "{name}: block null (block {BLOCK} d, {N_SURR} surrogates) — max consecutive elevated quiet days under noise: p95 {p95:.0}; real max run {real_max_run} ({transit} multi-day runs) — {sig}",
        sig = if real_max_run > p95 as usize && transit > 0 {
            "a sustained excursion exceeds the noise structure"
        } else {
            "no sustained (multi-day) excursion — no transit-form event resolved"
        }
    );
}

fn run(name: &str) {
    let path = format!("data/{name}_navio_subkhz_zone_daily.bin");
    let Some(daily) = read_zone_daily(&path) else {
        eprintln!("{name}: zone daily bin void ({path}) — 0 honored");
        return;
    };
    let ts: Vec<f64> = daily.iter().map(|d| d.0).collect();
    let vs: Vec<f64> = daily.iter().map(|d| d.1).collect();
    let rs: Vec<f64> = daily.iter().map(|d| d.2).collect();
    let n_in = daily.len();

    let abs: Vec<f64> = vs.iter().map(|x| x.abs()).collect();
    let mut abs_s = abs.clone();
    abs_s.sort_by(f64::total_cmp);
    eprintln!(
        "{name}: event-floor basis — {n_in} zone days ({}..{}), median |med| {m:.3e} Hz, RMS {rms:.3e} Hz",
        jd_date(ts[0]),
        jd_date(ts[ts.len() - 1]),
        m = pct(&abs_s, 0.5),
        rms = rms_about0(&vs)
    );

    let (gate_med, gate_r) = {
        let p90_abs = pct(&abs_s, 0.9);
        let mut rs_s = rs.clone();
        rs_s.sort_by(f64::total_cmp);
        let p90_r = pct(&rs_s, 0.9);
        (4.0 * p90_abs, 4.0 * p90_r)
    };
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
    eprintln!(
        "{name}: mask (Deduction-10, 4×p90: |med|>{gate_med:.0} Hz or day-RMS>{gate_r:.0} Hz) — {n_masked} of {n_in} discarded, {n_surv} survive",
        n_surv = mts.len()
    );

    let mabs: Vec<f64> = mvs.iter().map(|x| x.abs()).collect();
    let counts: Vec<usize> = [1.0, 2.0, 5.0, 10.0, 20.0, 50.0]
        .iter()
        .map(|th| mabs.iter().filter(|a| **a <= *th).count())
        .collect();
    eprintln!(
        "{name}: masked-day |med| residency — days ≤ 1 Hz {c1}/{n}, ≤ 2 Hz {c2}, ≤ 5 Hz {c5}, ≤ 10 Hz {c10}, ≤ 20 Hz {c20}, ≤ 50 Hz {c50}",
        c1 = counts[0],
        c2 = counts[1],
        c5 = counts[2],
        c10 = counts[3],
        c20 = counts[4],
        c50 = counts[5],
        n = mts.len()
    );

    let mut steps = Vec::new();
    let mut qsteps = Vec::new();
    for i in 0..mts.len() - 1 {
        let dt = (mts[i + 1] - mts[i]) / DAY_S;
        if dt > STEP_GAP_DAYS {
            continue;
        }
        let step = (mvs[i + 1] - mvs[i]).abs();
        steps.push(step);
        if mvs[i].abs() <= QUIET_HZ && mvs[i + 1].abs() <= QUIET_HZ {
            qsteps.push(step);
        }
    }
    let sd = |v: &[f64]| -> f64 {
        let m = v.iter().sum::<f64>() / v.len() as f64;
        (v.iter().map(|x| (x - m) * (x - m)).sum::<f64>() / v.len() as f64).sqrt()
    };
    let report_steps = |v: &[f64], label: &str| {
        if v.is_empty() {
            eprintln!("{name}: {label}: no steps");
            return;
        }
        let mut s = v.to_vec();
        s.sort_by(f64::total_cmp);
        eprintln!(
            "{name}: {label}: {n} steps, median {med:.3e}, p90 {p90:.3e}, p99 {p99:.3e}, max {max:.3e}, RMS {rms:.3e} Hz",
            n = s.len(),
            med = pct(&s, 0.5),
            p90 = pct(&s, 0.9),
            p99 = pct(&s, 0.99),
            max = *s.last().unwrap_or(&0.0),
            rms = sd(&s)
        );
    };
    report_steps(&steps, "day-to-day |step| (gap ≤3 d), all masked days");
    report_steps(&qsteps, "quiet→quiet |step| (both |med| ≤ 5 Hz)");

    let mut runs: Vec<(usize, f64)> = Vec::new();
    let mut i = 0usize;
    while i < mvs.len() {
        if mvs[i].abs() > QUIET_HZ {
            i += 1;
            continue;
        }
        let mut j = i;
        while j + 1 < mvs.len() {
            if mvs[j + 1].abs() > QUIET_HZ {
                break;
            }
            if (mts[j + 1] - mts[j]) / DAY_S > 2.0 {
                break;
            }
            j += 1;
        }
        runs.push((j - i + 1, (mts[j] - mts[i]) / DAY_S + 1.0));
        i = j + 1;
    }
    let mut lens: Vec<f64> = runs.iter().map(|r| r.0 as f64).collect();
    lens.sort_by(f64::total_cmp);
    if !lens.is_empty() {
        eprintln!(
            "{name}: quiet runs (consecutive |med| ≤ 5 Hz, gap ≤ 2 d): {n} runs, length days median {med:.0} / p90 {p90:.0} / max {max:.0}",
            n = runs.len(),
            med = pct(&lens, 0.5),
            p90 = pct(&lens, 0.9),
            max = *lens.last().unwrap_or(&0.0)
        );
    }

    let qidx: Vec<usize> = (0..mvs.len())
        .filter(|&i| mvs[i].abs() <= QUIET_HZ)
        .collect();
    let qvals: Vec<f64> = qidx.iter().map(|&i| mvs[i]).collect();
    let qsteps_max = qsteps.iter().cloned().fold(0.0f64, f64::max);
    eprintln!(
        "{name}: event floor — {q} of {n} masked days are quiet (|med| ≤ 5 Hz); their day-to-day steps (median {qm:.3e}, max {qx:.3e} Hz). A 1–5 Hz event needs to exceed this quiet-context step noise.",
        q = qidx.len(),
        n = mvs.len(),
        qm = {
            let mut s = qsteps.clone();
            s.sort_by(f64::total_cmp);
            if s.is_empty() { f64::NAN } else { pct(&s, 0.5) }
        },
        qx = qsteps_max
    );
    if qidx.len() >= 2 {
        let qn = qvals.len() as f64;
        let qmean = qvals.iter().sum::<f64>() / qn;
        let qsd = (qvals.iter().map(|x| (x - qmean) * (x - qmean)).sum::<f64>() / qn).sqrt();
        eprintln!(
            "{name}: quiet-day medians themselves: mean {qmean:.3e}, sd {qsd:.3e} Hz — {ratio:.0}× the ~1 Hz event scale",
            ratio = qsd / 1.0
        );
    }
    event_scan(name, &mts, &mvs);
}

fn main() {
    for name in ["pioneer10", "pioneer11"] {
        run(name);
    }
}
