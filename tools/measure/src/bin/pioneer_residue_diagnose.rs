use std::collections::HashMap;

use omegaflow::archivar::{
    BodyEphemeris, body_barycenter_position, body_barycenter_velocity, parse_ephemeris_binary,
};
use omegaflow::atdf::parse_bin;
use omegaflow::odp::{EARTH, downlink_rate, dsn_station};
use omegaflow::te::topological_te_phase;

const SC_BODY: &str = "pioneer10_daily";
const YEAR: f64 = 365.25 * 86400.0;

fn lin_fit3(x1: &[f64], x2: &[f64], y: &[f64]) -> Option<(f64, f64, f64)> {
    let n = y.len() as f64;
    if n < 3.0 {
        return None;
    }
    let mx1 = x1.iter().sum::<f64>() / n;
    let mx2 = x2.iter().sum::<f64>() / n;
    let my = y.iter().sum::<f64>() / n;
    let mut s11 = 0.0;
    let mut s12 = 0.0;
    let mut s22 = 0.0;
    let mut sy1 = 0.0;
    let mut sy2 = 0.0;
    for i in 0..y.len() {
        let d1 = x1[i] - mx1;
        let d2 = x2[i] - mx2;
        let dy = y[i] - my;
        s11 += d1 * d1;
        s12 += d1 * d2;
        s22 += d2 * d2;
        sy1 += d1 * dy;
        sy2 += d2 * dy;
    }
    let det = s11 * s22 - s12 * s12;
    if det.abs() < 1e-300 {
        return None;
    }
    let a = (sy1 * s22 - sy2 * s12) / det;
    let c = (sy2 * s11 - sy1 * s12) / det;
    let b = my - a * mx1 - c * mx2;
    Some((a, c, b))
}

fn fixed_effects(
    rates: &[f64],
    refs: &[f64],
    obs: &[f64],
    times: &[f64],
) -> (f64, f64, Vec<f64>, Vec<usize>, Vec<f64>) {
    let gap_threshold = 5.0 * 86400.0;
    let n = rates.len();
    let mut epoch = vec![0usize; n];
    let mut eid = 0usize;
    for i in 0..n {
        if i > 0 && times[i] - times[i - 1] > gap_threshold {
            eid += 1;
        }
        epoch[i] = eid;
    }
    let n_epoch = eid + 1;
    let mut mr = vec![0.0f64; n_epoch];
    let mut mf = vec![0.0f64; n_epoch];
    let mut mo = vec![0.0f64; n_epoch];
    let mut cnt = vec![0usize; n_epoch];
    for i in 0..n {
        let e = epoch[i];
        mr[e] += rates[i];
        mf[e] += refs[i];
        mo[e] += obs[i];
        cnt[e] += 1;
    }
    for e in 0..n_epoch {
        mr[e] /= cnt[e] as f64;
        mf[e] /= cnt[e] as f64;
        mo[e] /= cnt[e] as f64;
    }
    let mut cr = vec![0.0f64; n];
    let mut cf = vec![0.0f64; n];
    let mut co = vec![0.0f64; n];
    for i in 0..n {
        cr[i] = rates[i] - mr[epoch[i]];
        cf[i] = refs[i] - mf[epoch[i]];
        co[i] = obs[i] - mo[epoch[i]];
    }
    let (a, c, _) = lin_fit3(&cr, &cf, &co).unwrap();
    let mut offset = vec![0.0f64; n_epoch];
    for e in 0..n_epoch {
        offset[e] = mo[e] - a * mr[e] - c * mf[e];
    }
    let mut resid = vec![0.0f64; n];
    for i in 0..n {
        resid[i] = obs[i] - a * rates[i] - c * refs[i] - offset[epoch[i]];
    }
    (a, c, resid, epoch, offset)
}

fn annual_phase(times: &[f64], target: &[f64]) -> (f64, f64) {
    let w = 2.0 * std::f64::consts::PI / YEAR;
    let mut m = [[0.0f64; 3]; 3];
    let mut v = [0.0f64; 3];
    for i in 0..target.len() {
        let basis = [1.0, (w * times[i]).sin(), (w * times[i]).cos()];
        for p in 0..3 {
            v[p] += basis[p] * target[i];
            for q in 0..3 {
                m[p][q] += basis[p] * basis[q];
            }
        }
    }
    let mut coef = [0.0f64; 3];
    let mut a = m;
    let mut rhs = v;
    for col in 0..3 {
        let mut pivot = col;
        for row in col + 1..3 {
            if a[row][col].abs() > a[pivot][col].abs() {
                pivot = row;
            }
        }
        a.swap(col, pivot);
        rhs.swap(col, pivot);
        for row in col + 1..3 {
            let f = a[row][col] / a[col][col];
            for k in col..3 {
                a[row][k] -= f * a[col][k];
            }
            rhs[row] -= f * rhs[col];
        }
    }
    for row in (0..3).rev() {
        let mut s = rhs[row];
        for k in row + 1..3 {
            s -= a[row][k] * coef[k];
        }
        coef[row] = s / a[row][row];
    }
    let amp = (coef[1] * coef[1] + coef[2] * coef[2]).sqrt();
    (amp, coef[2].atan2(coef[1]))
}

fn main() {
    let path = "data/pioneer10_skyfreq.bin";
    let Ok(bytes) = std::fs::read(path) else {
        eprintln!("pioneer10: skyfreq bin void ({path})");
        return;
    };
    let Some(records) = parse_bin(&bytes) else {
        eprintln!("pioneer10: skyfreq bin parse void");
        return;
    };
    let mut eph: HashMap<String, BodyEphemeris> = HashMap::new();
    for body in [EARTH, SC_BODY] {
        let p = format!("data/ephemeris_{body}.bin");
        match std::fs::read(&p)
            .ok()
            .and_then(|d| parse_ephemeris_binary(&d))
        {
            Some(e) => {
                eph.insert(body.to_string(), e);
            }
            None => {
                eprintln!("{body}: ephemeris bin void ({p})");
                return;
            }
        }
    }
    let sc = |t: f64| -> Option<([f64; 3], [f64; 3])> {
        Some((
            body_barycenter_position(SC_BODY, t, &eph)?,
            body_barycenter_velocity(SC_BODY, t, &eph)?,
        ))
    };
    let mut rates = Vec::with_capacity(records.len());
    let mut refs = Vec::with_capacity(records.len());
    let mut obs = Vec::with_capacity(records.len());
    let mut times = Vec::with_capacity(records.len());
    let mut dtypes = Vec::with_capacity(records.len());
    let mut stids = Vec::with_capacity(records.len());
    let mut modes = Vec::with_capacity(records.len());
    let mut r_earth_norm = Vec::with_capacity(records.len());
    let mut dsres = Vec::with_capacity(records.len());
    let mut samplers = Vec::with_capacity(records.len());
    let mut cnts = Vec::with_capacity(records.len());
    for r in &records {
        let Some((lat, lon, alt)) = dsn_station(r[6] as i64) else {
            continue;
        };
        let Some(rate) = downlink_rate(r[0], lat, lon, alt, &eph, &sc) else {
            continue;
        };
        if !rate.is_finite() {
            continue;
        }
        rates.push(rate);
        refs.push(r[2]);
        obs.push(r[1]);
        times.push(r[0]);
        dtypes.push(r[5] as i64);
        stids.push(r[6] as i64);
        modes.push(r[13] as i64);
        dsres.push(r[8]);
        samplers.push(r[3]);
        cnts.push(r[7]);
        let re = body_barycenter_position(EARTH, r[0], &eph).unwrap_or([0.0; 3]);
        r_earth_norm.push((re[0] * re[0] + re[1] * re[1] + re[2] * re[2]).sqrt());
    }
    let (a, c, resid, epoch, offset) = fixed_effects(&rates, &refs, &obs, &times);
    eprintln!(
        "fsky = A·ṙ + C·ref + B_epoch, A {a:.4e}, C {c:.4e}, Innen-Epochen-Residuum-RMS {:.3e} Hz",
        (resid.iter().map(|x| x * x).sum::<f64>() / resid.len() as f64).sqrt()
    );
    {
        let mut med_ref = refs.clone();
        med_ref.sort_by(f64::total_cmp);
        let f0 = 96.0 * (240.0 / 221.0) * med_ref[med_ref.len() / 2];
        let f0c = f0 / 2.99792458e8;
        let mut drate = Vec::new();
        let mut drate_rates = Vec::new();
        for i in 0..rates.len() - 1 {
            let dt = times[i + 1] - times[i];
            if dt > 0.0 && (dt - samplers[i]).abs() < 10.0 {
                let d = (cnts[i + 1] - cnts[i]) / dt;
                if d.is_finite() {
                    drate.push(d);
                    drate_rates.push(rates[i]);
                }
            }
        }
        let n = drate.len() as f64;
        let mr = drate_rates.iter().sum::<f64>() / n;
        let md = drate.iter().sum::<f64>() / n;
        let mut cov = 0.0;
        let mut var = 0.0;
        for i in 0..drate.len() {
            cov += (drate_rates[i] - mr) * (drate[i] - md);
            var += (drate_rates[i] - mr) * (drate_rates[i] - mr);
        }
        let m = cov / var;
        let b = md - m * mr;
        let n_drate = drate.len();
        let ratio_oneway = m / f0c;
        let ratio_tp = m / (240.0 / 221.0 * f0c);
        eprintln!(
            "Roh-Count-Beat (Δt) = m·ṙ_down + b: m = {m:.4e} Hz/(m/s), b = {b:.4e} Hz (n={n_drate}) — f0 = {f0:.4e} Hz, f0/c = {f0c:.4e}, (240/221)·f0/c = {:.4e}, m/(f0/c) = {ratio_oneway:.6}, m/((240/221)·f0/c) = {ratio_tp:.6}",
            240.0 / 221.0 * f0c,
        );
    }
    {
        let mean = dsres.iter().sum::<f64>() / dsres.len() as f64;
        let rms = (dsres.iter().map(|x| x * x).sum::<f64>() / dsres.len() as f64).sqrt();
        eprintln!("DSN DOPPLER_RESID (field 60/1000): mean {mean:.3e} Hz, RMS {rms:.3e} Hz");
    }

    let mut dtype_count: HashMap<i64, usize> = HashMap::new();
    let mut dtype_sum: HashMap<i64, f64> = HashMap::new();
    let mut dtype_sq: HashMap<i64, f64> = HashMap::new();
    for i in 0..rates.len() {
        let d = dtypes[i];
        *dtype_count.entry(d).or_insert(0) += 1;
        *dtype_sum.entry(d).or_insert(0.0) += resid[i];
        *dtype_sq.entry(d).or_insert(0.0) += resid[i] * resid[i];
    }
    let mut ds: Vec<i64> = dtype_count.keys().copied().collect();
    ds.sort_unstable();
    for d in ds {
        let n = dtype_count[&d] as f64;
        let cnt = dtype_count[&d];
        let mean = dtype_sum[&d] / n;
        let rms = (dtype_sq[&d] / n).sqrt();
        eprintln!("data_type {d}: n={cnt}, residual mean {mean:.3e} Hz, RMS {rms:.3e} Hz");
    }

    let w = 2.0 * std::f64::consts::PI / YEAR;
    let fit = |target: &[f64]| -> [f64; 6] {
        let mut m = [[0.0f64; 6]; 6];
        let mut v = [0.0f64; 6];
        for i in 0..target.len() {
            let basis = [
                1.0,
                (w * times[i]).sin(),
                (w * times[i]).cos(),
                (2.0 * w * times[i]).sin(),
                (2.0 * w * times[i]).cos(),
                r_earth_norm[i],
            ];
            for p in 0..6 {
                v[p] += basis[p] * target[i];
                for q in 0..6 {
                    m[p][q] += basis[p] * basis[q];
                }
            }
        }
        let mut coef = [0.0f64; 6];
        let mut a = m;
        let mut rhs = v;
        for col in 0..6 {
            let mut pivot = col;
            for row in col + 1..6 {
                if a[row][col].abs() > a[pivot][col].abs() {
                    pivot = row;
                }
            }
            a.swap(col, pivot);
            rhs.swap(col, pivot);
            for row in col + 1..6 {
                let f = a[row][col] / a[col][col];
                for k in col..6 {
                    a[row][k] -= f * a[col][k];
                }
                rhs[row] -= f * rhs[col];
            }
        }
        for row in (0..6).rev() {
            let mut s = rhs[row];
            for k in row + 1..6 {
                s -= a[row][k] * coef[k];
            }
            coef[row] = s / a[row][row];
        }
        coef
    };
    let coef = fit(&resid);
    let rc = fit(&rates);
    let dc = fit(&dsres);
    {
        let amp = (dc[1] * dc[1] + dc[2] * dc[2]).sqrt();
        let amp6 = (dc[3] * dc[3] + dc[4] * dc[4]).sqrt();
        eprintln!("DSN DOPPLER_RESID yearly share: {amp:.3e} Hz, 6-month {amp6:.3e} Hz");
    }
    {
        let n = resid.len() as f64;
        let mr = resid.iter().sum::<f64>() / n;
        let md = dsres.iter().sum::<f64>() / n;
        let mut cov = 0.0;
        let mut vr = 0.0;
        let mut vd = 0.0;
        for i in 0..resid.len() {
            let dr = resid[i] - mr;
            let dd = dsres[i] - md;
            cov += dr * dd;
            vr += dr * dr;
            vd += dd * dd;
        }
        let r = cov / (vr * vd).sqrt();
        eprintln!("Korrelation fsky-Residuum ↔ DSN-DOPPLER_RESID: Pearson r = {r:.4}");
    }
    {
        let (data_amp, data_phase) = annual_phase(&times, &obs);
        let model_pred: Vec<f64> = (0..rates.len())
            .map(|i| a * rates[i] + c * refs[i] + offset[epoch[i]])
            .collect();
        let (model_amp, model_phase) = annual_phase(&times, &model_pred);
        let (resid_amp_3, resid_phase_3) = annual_phase(&times, &resid);
        let diff = data_phase - model_phase;
        eprintln!(
            "yearly phase data {data_amp:.3e} Hz @ {data_phase:.3} rad vs model {model_amp:.3e} Hz @ {model_phase:.3} rad — difference {diff:.3} rad ({:.1}°); residual yearly share (3-basis) {resid_amp_3:.3e} Hz @ {resid_phase_3:.3} rad",
            diff.to_degrees()
        );
        let n_epoch = *epoch.iter().max().unwrap() + 1;
        for e in 0..n_epoch {
            let mut t = Vec::new();
            let mut r = Vec::new();
            for i in 0..rates.len() {
                if epoch[i] == e {
                    t.push(times[i]);
                    r.push(resid[i]);
                }
            }
            if !t.is_empty() {
                let span = t[t.len() - 1] - t[0];
                if span > 200.0 * 86400.0 {
                    let (amp, ph) = annual_phase(&t, &r);
                    eprintln!(
                        "  epoch {e}: n={}, span {:.0} d, residual yearly share {amp:.3e} Hz @ {ph:.3} rad",
                        t.len(),
                        span / 86400.0
                    );
                }
            }
        }
    }
    let amp_1yr = (coef[1] * coef[1] + coef[2] * coef[2]).sqrt();
    let phase_1yr = coef[2].atan2(coef[1]);
    let amp_6mo = (coef[3] * coef[3] + coef[4] * coef[4]).sqrt();
    let model_amp = (rc[1] * rc[1] + rc[2] * rc[2]).sqrt();
    let model_phase = rc[2].atan2(rc[1]);
    let phase_diff = phase_1yr - model_phase;
    let pd_deg = phase_diff.to_degrees();
    eprintln!(
        "yearly share residual: {amp_1yr:.3e} Hz (phase {phase_1yr:.2} rad), 6-month {amp_6mo:.3e} Hz; model-ṙ_down yearly share {model_amp:.3e} m/s (phase {model_phase:.2} rad) — phase difference {phase_diff:.3} rad ({pd_deg:.1}°), Earth-Sun distance coefficient {:.3e} Hz/m",
        coef[5]
    );
    {
        const JD_UNIX_EPOCH: f64 = 2440587.5;
        const DAY: f64 = 86400.0;
        const J2000_EPOCH: f64 = 2451545.0;
        const MIN_SAMPLES_PER_MONTH: usize = 3;
        const MIN_MONTHS: usize = 24;
        const STATIONS: [i64; 3] = [14, 43, 63];
        let month_of = |t: f64| -> Option<i64> {
            let jd = J2000_EPOCH + t / DAY;
            let days = (jd - JD_UNIX_EPOCH).floor() as i64;
            let (y, m, _) = omegaflow::spectral::civil_from_days(days)?;
            Some(y as i64 * 12 + m as i64)
        };
        let classes = [(1.0, 3i64, "1-s"), (60.0, 3i64, "60-s")];
        for (sampler, mode, cname) in classes {
            let mut groups: HashMap<(i64, i64), Vec<f64>> = HashMap::new();
            let mut class_counts: HashMap<(i64, i64), usize> = HashMap::new();
            for i in 0..rates.len() {
                if !STATIONS.contains(&stids[i]) {
                    continue;
                }
                if modes[i] != mode {
                    continue;
                }
                if (samplers[i] - sampler).abs() > 0.1 {
                    continue;
                }
                let Some(mo) = month_of(times[i]) else {
                    continue;
                };
                *class_counts.entry((stids[i], mo)).or_default() += 1;
                groups.entry((stids[i], mo)).or_default().push(resid[i]);
            }
            for st in STATIONS {
                let months = class_counts.iter().filter(|((s, _), _)| *s == st).count();
                let samples: usize = class_counts
                    .iter()
                    .filter(|((s, _), _)| *s == st)
                    .map(|(_, n)| n)
                    .sum();
                println!(
                    "  Sta {st}: {samples} {cname}-mode-3 residual samples over {months} months (≥ 1 sample)"
                );
            }
            let mut series: HashMap<i64, Vec<(i64, f64)>> = HashMap::new();
            for ((st, mo), vals) in groups {
                if vals.len() < MIN_SAMPLES_PER_MONTH {
                    continue;
                }
                let mut v = vals;
                v.sort_by(f64::total_cmp);
                series.entry(st).or_default().push((mo, v[v.len() / 2]));
            }
            for st in STATIONS {
                series.entry(st).or_default().sort_by_key(|x| x.0);
            }
            println!();
            println!(
                "Per-station residual (full model, Moyer path), {cname} class mode 3, monthly median (≥ {} samples):",
                MIN_SAMPLES_PER_MONTH
            );
            println!(
                "{:>4} | {:>5} | {:>20} | {:>14} | {:>14}",
                "Sta", "n", "span", "drift", "se"
            );
            for st in STATIONS {
                let m = &series[&st];
                if m.is_empty() {
                    println!("{st:>4} | no samples in the class (0 honored)");
                    continue;
                }
                let xs: Vec<f64> = m.iter().map(|x| x.0 as f64).collect();
                let ys: Vec<f64> = m.iter().map(|x| x.1).collect();
                let n = xs.len() as f64;
                let xm = xs.iter().sum::<f64>() / n;
                let ym = ys.iter().sum::<f64>() / n;
                let mut sxx = 0.0;
                let mut sxy = 0.0;
                for i in 0..xs.len() {
                    sxx += (xs[i] - xm) * (xs[i] - xm);
                    sxy += (xs[i] - xm) * (ys[i] - ym);
                }
                let a = if sxx > 0.0 { sxy / sxx } else { f64::NAN };
                let b = ym - a * xm;
                let mut rss = 0.0;
                for i in 0..xs.len() {
                    rss += (ys[i] - a * xs[i] - b).powi(2);
                }
                let se = if n > 2.0 && sxx > 0.0 {
                    (rss / ((n - 2.0) * sxx)).sqrt()
                } else {
                    f64::NAN
                };
                let f = m[0].0;
                let l = m[m.len() - 1].0;
                println!(
                    "{:>4} | {:>5} | {:>4}-{:02}..{:>4}-{:02} | {:>10.3} mHz/mo | {:>10.3}",
                    st,
                    m.len(),
                    f / 12,
                    f % 12 + 1,
                    l / 12,
                    l % 12 + 1,
                    1e3 * a,
                    1e3 * se
                );
            }
            for (i, a) in STATIONS.iter().enumerate() {
                for b in &STATIONS[i + 1..] {
                    let ma = &series[a];
                    let mb = &series[b];
                    let va: HashMap<i64, f64> = ma.iter().copied().collect();
                    let vb: HashMap<i64, f64> = mb.iter().copied().collect();
                    let mut xs: Vec<f64> = Vec::new();
                    let mut ys: Vec<f64> = Vec::new();
                    for (mo, v) in &va {
                        if let Some(w) = vb.get(mo) {
                            xs.push(*v);
                            ys.push(*w);
                        }
                    }
                    if xs.len() < MIN_MONTHS {
                        println!(
                            "Sta {a}↔{b}: only {} shared months (< {MIN_MONTHS}) — still (0 honored)",
                            xs.len()
                        );
                        continue;
                    }
                    let xf: Vec<f32> = xs.iter().map(|v| *v as f32).collect();
                    let yf: Vec<f32> = ys.iter().map(|v| *v as f32).collect();
                    let ab =
                        topological_te_phase(&xf, &yf, 3, 3, 0x9E37_79B9_7F4A_7C15 ^ (*a as u64));
                    let ba =
                        topological_te_phase(&yf, &xf, 3, 3, 0x9E37_79B9_7F4A_7C15 ^ (*b as u64));
                    println!("Sta {a}↔{b}: {} shared months", xs.len());
                    for (label, v) in [(format!("TE({a}→{b})"), &ab), (format!("TE({b}→{a})"), &ba)]
                    {
                        match v {
                            Some(t) => {
                                let word = if t.te > t.threshold {
                                    "ARROW (over thr)"
                                } else {
                                    "still"
                                };
                                println!(
                                    "  {label}: te {:.4e} vs thr {:.4e} ({} Surrogate, τ_x {} τ_y {}) — {}",
                                    t.te, t.threshold, t.surrogates_used, t.tau_x, t.tau_y, word
                                );
                            }
                            None => println!(
                                "  {label}: no MI-τ — the phase space carries no coupling (still, 0 honored)"
                            ),
                        }
                    }
                }
            }
            println!(
                "Shared-driver question: TE over the surrogate threshold (mean+2σ, phase-randomized). n = monthly medians of the full model residual, no interpolation."
            );
        }
    }
}
