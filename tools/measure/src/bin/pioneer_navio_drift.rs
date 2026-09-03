use omegaflow::archivar::spectral::civil_from_days;

const DAY_S: f64 = 86400.0;
const C: f64 = 299792458.0;
const TRANS_RATIO: f64 = 240.0 / 221.0;
const PIONEER_ANOMALY: f64 = 8.74e-10;
const F0: f64 = 2.292e9;

fn jd_date(tdb: f64) -> String {
    let jd = tdb / DAY_S + 2451545.0;
    let unix_day = (jd - 2440587.5).round() as i64;
    match civil_from_days(unix_day) {
        Some((y, m, d)) => format!("{y:04}-{m:02}-{d:02}"),
        None => format!("tdb {tdb:.0} s"),
    }
}

fn read_daily(path: &str) -> Option<Vec<(f64, f64)>> {
    let d = std::fs::read(path).ok()?;
    if d.len() < 8 || &d[0..4] != b"PNDM" {
        return None;
    }
    let cnt = u32::from_le_bytes(d[4..8].try_into().ok()?) as usize;
    if d.len() != 8 + cnt * 32 {
        return None;
    }
    let mut out = Vec::new();
    for i in 0..cnt {
        let o = 8 + i * 32;
        let t = f64::from_le_bytes(d[o..o + 8].try_into().ok()?);
        let med = f64::from_le_bytes(d[o + 8..o + 16].try_into().ok()?);
        out.push((t, med));
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
    let a = num / den;
    Some((a, my - a * mx))
}

fn run(name: &str) {
    let path = format!("data/{name}_navio_subkhz_daily.bin");
    let Some(daily) = read_daily(&path) else {
        eprintln!("{name}: sub-kHz daily bin void/parse void ({path})");
        return;
    };
    let ts: Vec<f64> = daily.iter().map(|x| x.0).collect();
    let vs: Vec<f64> = daily.iter().map(|x| x.1).collect();

    let mut absmed: Vec<f64> = vs.iter().map(|x| x.abs()).collect();
    absmed.sort_by(f64::total_cmp);
    let p90 = absmed[((absmed.len() as f64 * 0.90) as usize).min(absmed.len() - 1)];
    let p95 = absmed[((absmed.len() as f64 * 0.95) as usize).min(absmed.len() - 1)];

    let t0 = ts[0];
    let span_y = (ts[ts.len() - 1] - t0) / DAY_S / 365.25;
    eprintln!(
        "{name}: continuous drift of {} sub-kHz daily medians over {span_y:.2} y ({}..{})",
        daily.len(),
        jd_date(t0),
        jd_date(ts[ts.len() - 1])
    );

    let k_phys = TRANS_RATIO / C;
    for (label, cap, keep) in [
        ("all days", f64::INFINITY, (0..vs.len()).collect::<Vec<_>>()),
        (
            "quiet days (|med| ≤ p90)",
            p90,
            (0..vs.len())
                .filter(|&i| vs[i].abs() <= p90)
                .collect::<Vec<_>>(),
        ),
        (
            "quiet days (|med| ≤ p95)",
            p95,
            (0..vs.len())
                .filter(|&i| vs[i].abs() <= p95)
                .collect::<Vec<_>>(),
        ),
    ] {
        let _ = cap;
        if keep.len() < 30 {
            continue;
        }
        let kts: Vec<f64> = keep.iter().map(|&i| ts[i]).collect();
        let kvs: Vec<f64> = keep.iter().map(|&i| vs[i]).collect();
        let Some((slope, _)) = lin_fit(&kts, &kvs) else {
            continue;
        };
        let drift_hz_s = slope / DAY_S;
        let accel = drift_hz_s / (k_phys * F0);
        eprintln!(
            "{name}: [{label}] n={} — secular slope {slope:.3e} Hz/day → {drift_hz_s:.3e} Hz/s → acceleration {accel:.3e} m/s² = {ratio:.1e}× anomaly",
            keep.len(),
            ratio = accel / PIONEER_ANOMALY
        );
    }

    let t_rel: Vec<f64> = ts.iter().map(|t| (t - t0) / DAY_S).collect();
    let quiet: Vec<usize> = (0..vs.len()).filter(|&i| vs[i].abs() <= p95).collect();
    let q_t: Vec<f64> = quiet.iter().map(|&i| t_rel[i]).collect();
    let q_v: Vec<f64> = quiet.iter().map(|&i| vs[i]).collect();
    let (a_lin, _) = lin_fit(&q_t, &q_v).unwrap_or((0.0, 0.0));
    let resid_lin: Vec<f64> = q_t
        .iter()
        .zip(q_v.iter())
        .map(|(t, v)| v - a_lin * t)
        .collect();
    let rms_lin = (resid_lin.iter().map(|x| x * x).sum::<f64>() / resid_lin.len() as f64).sqrt();
    let n = q_t.len() as f64;
    let mt = q_t.iter().sum::<f64>() / n;
    let x2: Vec<f64> = q_t.iter().map(|t| (t - mt) * (t - mt)).collect();
    let a_q = lin_fit(&x2, &q_v).map(|(a, _)| a).unwrap_or(0.0);
    let resid_quad: Vec<f64> = x2
        .iter()
        .zip(q_v.iter())
        .map(|(x, v)| v - a_q * x)
        .collect();
    let rms_quad = (resid_quad.iter().map(|x| x * x).sum::<f64>() / resid_quad.len() as f64).sqrt();

    eprintln!(
        "{name}: [quiet ≤p95] form test — linear resid RMS {rms_lin:.3e} Hz vs quadratic (∝t²) resid RMS {rms_quad:.3e} Hz (quad coeff {a_q:.3e} Hz/day²; n={})",
        q_t.len()
    );
    if rms_quad < rms_lin * 0.98 {
        eprintln!(
            "{name}: the ∝t² form (constant acceleration) fits >2% better than linear on the quiet days — a curvature signal held against the floor, not claimed"
        );
    } else {
        eprintln!(
            "{name}: no >2% curvature advantage for ∝t² over linear on the quiet days — the constant-acceleration signature is not resolved above the quiet-day residuum floor"
        );
    }
    eprintln!(
        "{name}: drift verdict at the quiet-day floor — a true anomaly (~1 Hz over the mission) vs quiet resid RMS {rms_lin:.1e} Hz (0 honored, pending against the physical floor)",
    );
}

fn main() {
    for name in ["pioneer10", "pioneer11"] {
        run(name);
    }
}
