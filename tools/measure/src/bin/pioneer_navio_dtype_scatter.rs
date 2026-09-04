use omegaflow::doppler::parse_pnav_bin;

const DAY_S: f64 = 86400.0;

fn run(name: &str) {
    let path = format!("data/{name}_navio.bin");
    let Some(recs) = std::fs::read(&path).ok().and_then(|d| parse_pnav_bin(&d)) else {
        eprintln!("{name}: pnav bin void ({path})");
        return;
    };
    // record: [timtag, obs, freq, cmptime, dtype, sc, trans, rcvr1, linkmode]
    let mut by12: std::collections::BTreeMap<i64, Vec<f64>> = std::collections::BTreeMap::new();
    let mut by13: std::collections::BTreeMap<i64, Vec<f64>> = std::collections::BTreeMap::new();
    let mut n12 = 0usize;
    let mut n13 = 0usize;
    for r in &recs {
        let obs = r[1];
        if !obs.is_finite() {
            continue;
        }
        let day = (r[0] / DAY_S).floor() as i64;
        match r[4] as i64 {
            12 => {
                by12.entry(day).or_default().push(obs);
                n12 += 1;
            }
            13 => {
                by13.entry(day).or_default().push(obs);
                n13 += 1;
            }
            _ => {}
        }
    }
    eprintln!(
        "{name}: DTYPE-12 (two-way) {n12} samples over {} days; DTYPE-13 (three-way) {n13} samples over {} days",
        by12.len(),
        by13.len()
    );

    // per-day de-trended (subtract day median) per-sample RMS for each DTYPE
    let scatter = |map: &std::collections::BTreeMap<i64, Vec<f64>>| -> (usize, f64, f64) {
        let mut rms_vals: Vec<f64> = Vec::new();
        let mut n_used = 0usize;
        for (_, v) in map {
            if v.len() < 5 {
                continue;
            }
            let mut s = v.clone();
            s.sort_by(f64::total_cmp);
            let med = s[s.len() / 2];
            let d: Vec<f64> = v.iter().map(|x| x - med).collect();
            let r = (d.iter().map(|x| x * x).sum::<f64>() / d.len() as f64).sqrt();
            rms_vals.push(r);
            n_used += v.len();
        }
        if rms_vals.is_empty() {
            return (0, f64::NAN, f64::NAN);
        }
        rms_vals.sort_by(f64::total_cmp);
        let med_rms = rms_vals[rms_vals.len() / 2];
        (n_used, med_rms, rms_vals[rms_vals.len() - 1])
    };
    let (u12, m12, mx12) = scatter(&by12);
    let (u13, m13, mx13) = scatter(&by13);
    eprintln!(
        "{name}: per-day median-de-trended per-sample RMS — DTYPE-12: n={u12} median {m12:.3e} Hz (max {mx12:.3e}); DTYPE-13: n={u13} median {m13:.3e} Hz (max {mx13:.3e})"
    );
    if m12.is_finite() && m13.is_finite() {
        let ratio = m13 / m12;
        eprintln!(
            "{name}: three-way/two-way per-sample RMS ratio = {ratio:.2} — {verdict}",
            verdict = if ratio <= 1.05 {
                "three-way scatter ≤ two-way (√N gain would survive; full rx×tx build reopens)"
            } else {
                "three-way scatter > two-way (three-way is the noisier second reference; DTYPE-13 is N-only, floor-bound — the two-way gate at linkmode 12 is the clean self-referenced channel, not a bug)"
            }
        );
    }
}

fn main() {
    for name in ["pioneer10", "pioneer11"] {
        run(name);
    }
}
