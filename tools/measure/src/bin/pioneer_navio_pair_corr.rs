use omegaflow::archivar::spectral::civil_from_days;
use omegaflow::te::phase_randomized_surrogate;

const DAY_S: f64 = 86400.0;
const N_SURR: usize = 200;

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

fn pearson(x: &[f64], y: &[f64]) -> f64 {
    let n = x.len() as f64;
    if n < 3.0 {
        return f64::NAN;
    }
    let mx = x.iter().sum::<f64>() / n;
    let my = y.iter().sum::<f64>() / n;
    let mut num = 0.0;
    let mut sx = 0.0;
    let mut sy = 0.0;
    for i in 0..x.len() {
        let dx = x[i] - mx;
        let dy = y[i] - my;
        num += dx * dy;
        sx += dx * dx;
        sy += dy * dy;
    }
    if sx <= 0.0 || sy <= 0.0 {
        return f64::NAN;
    }
    num / (sx * sy).sqrt()
}

fn spearman(x: &[f64], y: &[f64]) -> f64 {
    let rank = |v: &[f64]| -> Vec<f64> {
        let mut idx: Vec<usize> = (0..v.len()).collect();
        idx.sort_by(|&a, &b| v[a].total_cmp(&v[b]));
        let mut r = vec![0.0f64; v.len()];
        let mut i = 0usize;
        while i < idx.len() {
            let mut j = i;
            while j + 1 < idx.len() && v[idx[j + 1]] == v[idx[i]] {
                j += 1;
            }
            let avg = (i + j) as f64 / 2.0 + 1.0;
            for k in i..=j {
                r[idx[k]] = avg;
            }
            i = j + 1;
        }
        r
    };
    let rx = rank(x);
    let ry = rank(y);
    pearson(&rx, &ry)
}

fn jd_date(tdb: f64) -> String {
    let jd = tdb / DAY_S + 2451545.0;
    let unix_day = (jd - 2440587.5).round() as i64;
    match civil_from_days(unix_day) {
        Some((y, m, d)) => format!("{y:04}-{m:02}-{d:02}"),
        None => format!("tdb {tdb:.0} s"),
    }
}

fn main() {
    let Some(p10) = read_daily("data/pioneer10_navio_subkhz_daily.bin") else {
        eprintln!("p10 daily void");
        return;
    };
    let Some(p11) = read_daily("data/pioneer11_navio_subkhz_daily.bin") else {
        eprintln!("p11 daily void");
        return;
    };
    let m10: std::collections::BTreeMap<i64, f64> = p10
        .iter()
        .map(|(t, v)| ((t / DAY_S).floor() as i64, *v))
        .collect();
    let m11: std::collections::BTreeMap<i64, f64> = p11
        .iter()
        .map(|(t, v)| ((t / DAY_S).floor() as i64, *v))
        .collect();
    let mut shared: Vec<(i64, f64, f64)> = Vec::new();
    for (d, v10) in &m10 {
        if let Some(&v11) = m11.get(d) {
            shared.push((*d, *v10, v11));
        }
    }
    shared.sort_by_key(|s| s.0);
    if shared.len() < 30 {
        eprintln!("shared era too short ({})", shared.len());
        return;
    }
    let x: Vec<f64> = shared.iter().map(|s| s.1).collect();
    let y: Vec<f64> = shared.iter().map(|s| s.2).collect();
    let d0 = shared[0].0;
    let d1 = shared[shared.len() - 1].0;
    eprintln!(
        "Deduktion-42 pair correlation: {} shared days {}..{} (P10 vs P11 sub-kHz daily medians)",
        shared.len(),
        jd_date(d0 as f64 * DAY_S),
        jd_date(d1 as f64 * DAY_S)
    );
    let r_p = pearson(&x, &y);
    let r_s = spearman(&x, &y);
    eprintln!("Pearson r = {r_p:.4}, Spearman rho = {r_s:.4}");

    let yf: Vec<f32> = y.iter().map(|&v| v as f32).collect();
    let mut rng = 0x9E3779B97F4A7C15u64;
    let mut surr: Vec<f64> = Vec::with_capacity(N_SURR);
    for _ in 0..N_SURR {
        let ys = phase_randomized_surrogate(&yf, &mut rng);
        let yfd: Vec<f64> = ys.iter().map(|&v| v as f64).collect();
        let r = pearson(&x, &yfd);
        if r.is_finite() {
            surr.push(r);
        }
    }
    surr.sort_by(f64::total_cmp);
    let n = surr.len() as f64;
    let mean = surr.iter().sum::<f64>() / n;
    let var = surr.iter().map(|&v| (v - mean) * (v - mean)).sum::<f64>() / n;
    let sd = var.sqrt();
    let thr = mean + 2.0 * sd;
    let p50 = surr[surr.len() / 2];
    let p95 = surr[((surr.len() as f64 * 0.95) as usize).min(surr.len() - 1)];
    let p99 = surr[((surr.len() as f64 * 0.99) as usize).min(surr.len() - 1)];
    eprintln!(
        "surrogate null (phase-randomized, n={}): mean {mean:.4} sd {sd:.4}, threshold {thr:.4} (mean+2sd), p50/p95/p99 = {p50:.4}/{p95:.4}/{p99:.4}",
        surr.len()
    );
    let verdict = if r_p > thr {
        "correlation exceeds the phase-randomized null -> a shared common-mode structure in the overlap era (common systematics or a common physical driver)"
    } else if r_p.abs() < sd {
        "correlation within ±1 sd of the null -> no shared common-mode structure; the residual curves are independently floor-dominated (Deduktion 26 confirmed)"
    } else {
        "correlation between the null and threshold -> inconclusive against the phase null"
    };
    eprintln!("verdict: r = {r_p:.4} vs threshold {thr:.4} -> {verdict}");
    eprintln!("note: a ~1 Hz anomaly signal contributes negligibly to this correlation coefficient — a nonzero r here would mean common systematics, never the anomaly (0 honored)");
}
