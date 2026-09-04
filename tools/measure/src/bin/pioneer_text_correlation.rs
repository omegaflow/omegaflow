use std::collections::{BTreeMap, HashSet};

fn tokenize(text: &str) -> Vec<f64> {
    let bytes: Vec<char> = text.chars().collect();
    let mut out: Vec<f64> = Vec::new();
    let mut i = 0usize;
    while i < bytes.len() {
        let c = bytes[i];
        if c.is_ascii_digit() || c == '.' {
            let mut j = i;
            let mut has_dot = false;
            while j < bytes.len() && (bytes[j].is_ascii_digit() || (bytes[j] == '.' && !has_dot)) {
                if bytes[j] == '.' {
                    has_dot = true;
                }
                j += 1;
            }
            let num: String = bytes[i..j].iter().collect();
            let mant = num.parse::<f64>().unwrap_or(f64::NAN);
            let mut exp = 0i32;
            let mut k = j;
            while k < bytes.len() && bytes[k].is_whitespace() {
                k += 1;
            }
            if k + 3 < bytes.len()
                && bytes[k] == '\u{00d7}'
                && bytes[k + 1] == '1'
                && bytes[k + 2] == '0'
            {
                let mut m = k + 3;
                let neg = m < bytes.len() && (bytes[m] == '-' || bytes[m] == '\u{2212}');
                let pos = m < bytes.len() && bytes[m] == '+';
                if neg || pos {
                    m += 1;
                }
                let mut es = 0i32;
                let mut ok = false;
                while m < bytes.len() && bytes[m].is_ascii_digit() {
                    es = es * 10 + (bytes[m] as i32 - '0' as i32);
                    ok = true;
                    m += 1;
                }
                if ok {
                    exp = if neg { -es } else { es };
                    j = m;
                }
            }
            if mant.is_finite() && mant > 0.0 {
                out.push(mant * 10f64.powi(exp));
            }
            i = j;
        } else {
            i += 1;
        }
    }
    out
}

fn mantissa_fraction(v: f64) -> f64 {
    let l = v.log10();
    l - l.floor()
}

fn corr(a: &[f64], b: &[f64], lag: usize) -> f64 {
    if b.len() <= lag {
        return 0.0;
    }
    let m = a.len().min(b.len() - lag).min(20_000);
    if m < 100 {
        return 0.0;
    }
    let ma = a[..m].iter().sum::<f64>() / m as f64;
    let mb = b[lag..lag + m].iter().sum::<f64>() / m as f64;
    let mut num = 0.0;
    let mut da = 0.0;
    let mut db = 0.0;
    for k in 0..m {
        let x = a[k] - ma;
        let y = b[lag + k] - mb;
        num += x * y;
        da += x * x;
        db += y * y;
    }
    let den = (da * db).sqrt();
    if den == 0.0 {
        0.0
    } else {
        num / den
    }
}

fn i1000_series(vals: &[f64]) -> Vec<f64> {
    let mut blocks: BTreeMap<i64, Vec<f64>> = BTreeMap::new();
    for (i, &v) in vals.iter().enumerate() {
        blocks.entry(i as i64 / 1000).or_default().push(v);
    }
    blocks
        .values()
        .map(|w| {
            let mut s = w.clone();
            s.sort_by(f64::total_cmp);
            s[s.len() / 2]
        })
        .collect()
}

fn j2000_series(tdb: &[f64], vals: &[f64]) -> Vec<f64> {
    let mut days: BTreeMap<i64, Vec<f64>> = BTreeMap::new();
    for (i, &v) in vals.iter().enumerate() {
        if v.is_finite() {
            days.entry((tdb[i] / 86400.0).floor() as i64)
                .or_default()
                .push(v);
        }
    }
    days.values()
        .map(|w| {
            let mut s = w.clone();
            s.sort_by(f64::total_cmp);
            s[s.len() / 2]
        })
        .collect()
}

fn j2000_map(tdb: &[f64], vals: &[f64]) -> BTreeMap<i64, f64> {
    let mut days: BTreeMap<i64, Vec<f64>> = BTreeMap::new();
    for (i, &v) in vals.iter().enumerate() {
        if v.is_finite() {
            days.entry((tdb[i] / 86400.0).floor() as i64)
                .or_default()
                .push(v);
        }
    }
    days.into_iter()
        .map(|(d, mut w)| {
            w.sort_by(f64::total_cmp);
            (d, w[w.len() / 2])
        })
        .collect()
}

fn xorshift(rng: &mut u64) -> u64 {
    *rng = rng
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);
    *rng
}

fn corr_report(name: &str, tokens: &[f64], series: &[f64], rng: &mut u64) {
    let mut best = 0.0f64;
    let mut best_lag = 0usize;
    for lag in 0..=1000 {
        let r = corr(tokens, series, lag).abs();
        if r > best {
            best = r;
            best_lag = lag;
        }
    }
    let n_surr = std::env::var("TE_SURR")
        .ok()
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(24);
    let mut null: Vec<f64> = Vec::new();
    for _ in 0..n_surr {
        let mut p = tokens.to_vec();
        for k in (1..p.len()).rev() {
            let j = ((xorshift(rng) >> 33) as usize) % (k + 1);
            p.swap(k, j);
        }
        let mut nb = 0.0f64;
        for lag in 0..=1000 {
            let r = corr(&p, series, lag).abs();
            if r > nb {
                nb = r;
            }
        }
        null.push(nb);
    }
    let null_max = null.iter().copied().fold(0.0, f64::max);
    let null_p95 = {
        let mut s = null.clone();
        s.sort_by(f64::total_cmp);
        s[((s.len() as f64 * 0.95) as usize).min(s.len() - 1)]
    };
    let exceed = null.iter().filter(|&&x| x >= best).count();
    eprintln!(
        "    {name}: max |r| = {best:.4} @lag {best_lag} vs Null max {null_max:.4} (p95 {null_p95:.4}, n={n_surr}) — p_emp = {exceed}/{n_surr}",
    );
}

fn main() {
    let text_path = std::env::var("TE_TEXT").unwrap_or_else(|_| {
        "docs/reference/pioneer-anomaly/pioneer-anomaly-lrr-2010-4.txt".to_string()
    });
    let Some(text) = std::fs::read_to_string(&text_path).ok() else {
        eprintln!("Text absent ({text_path}) — empty (0 honored)");
        return;
    };
    let tokens = tokenize(&text);
    let mut uniq: Vec<f64> = tokens.clone();
    uniq.sort_by(f64::total_cmp);
    uniq.dedup();
    eprintln!(
        "Text: {} characters, {} number tokens, {} distinct values",
        text.chars().count(),
        tokens.len(),
        uniq.len()
    );

    let mut tdb10: Vec<f64> = Vec::new();
    let mut obs10: Vec<f64> = Vec::new();
    let mut car10: Vec<f64> = Vec::new();
    let mut tdb11: Vec<f64> = Vec::new();
    let mut obs11: Vec<f64> = Vec::new();
    let mut car11: Vec<f64> = Vec::new();
    for (path, tdb, obs, car) in [
        (
            "data/pioneer10_doppler_clean.bin",
            &mut tdb10,
            &mut obs10,
            &mut car10,
        ),
        (
            "data/pioneer11_doppler_clean.bin",
            &mut tdb11,
            &mut obs11,
            &mut car11,
        ),
    ] {
        if let Ok(bytes) = std::fs::read(path) {
            if let Some(recs) = omegaflow::doppler::parse_bin(&bytes) {
                for r in &recs {
                    if r[1].is_finite() && r[2].is_finite() {
                        tdb.push(r[0]);
                        obs.push(r[1]);
                        car.push(r[2]);
                    }
                }
            }
        }
    }
    eprintln!(
        "Data: P10 {} records, P11 {} records (OBSVBL + carrier)",
        obs10.len(),
        obs11.len()
    );
    eprintln!(
        "P10 Doppler [{:.4}..{:.4}], carrier [{:.4}..{:.4}]",
        obs10.iter().copied().fold(f64::INFINITY, f64::min),
        obs10.iter().copied().fold(f64::NEG_INFINITY, f64::max),
        car10.iter().copied().fold(f64::INFINITY, f64::min),
        car10.iter().copied().fold(f64::NEG_INFINITY, f64::max),
    );
    eprintln!(
        "P11 Doppler [{:.4}..{:.4}], carrier [{:.4}..{:.4}]",
        obs11.iter().copied().fold(f64::INFINITY, f64::min),
        obs11.iter().copied().fold(f64::NEG_INFINITY, f64::max),
        car11.iter().copied().fold(f64::INFINITY, f64::min),
        car11.iter().copied().fold(f64::NEG_INFINITY, f64::max),
    );

    let mut set10: HashSet<i64> = HashSet::new();
    let mut set11: HashSet<i64> = HashSet::new();
    for &v in &obs10 {
        if v.fract().abs() < 1e-12 && v.abs() < 1e15 {
            set10.insert(v as i64);
        }
    }
    for &v in &obs11 {
        if v.fract().abs() < 1e-12 && v.abs() < 1e15 {
            set11.insert(v as i64);
        }
    }
    let mut hit10: Vec<i64> = Vec::new();
    let mut hit11: Vec<i64> = Vec::new();
    for &t in &uniq {
        if t.fract().abs() < 1e-12 && t.abs() < 1e15 {
            let v = t as i64;
            if set10.contains(&v) {
                hit10.push(v);
            }
            if set11.contains(&v) {
                hit11.push(v);
            }
        }
    }
    eprintln!(
        "Relation 1 exact hits: P10 {} {:?}, P11 {} {:?}",
        hit10.len(),
        hit10,
        hit11.len(),
        hit11
    );

    let mant_dist = |car_mant: &[f64], t: f64| -> f64 {
        let f = mantissa_fraction(t);
        let idx = car_mant.partition_point(|&m| m < f);
        let mut best = f64::INFINITY;
        for j in [idx.checked_sub(1), Some(idx), Some(idx + 1)]
            .into_iter()
            .flatten()
        {
            let jj = j % car_mant.len();
            let d = (car_mant[jj] - f).abs();
            best = best.min(d.min(1.0 - d));
        }
        best
    };
    let car_mant = |car: &[f64]| -> Vec<f64> {
        let mut m: Vec<f64> = car.iter().map(|&c| mantissa_fraction(c)).collect();
        m.sort_by(f64::total_cmp);
        m
    };
    let cm10 = car_mant(&car10);
    let cm11 = car_mant(&car11);
    let mut m10: Vec<(f64, f64)> = Vec::new();
    let mut m11: Vec<(f64, f64)> = Vec::new();
    for &t in &uniq {
        if t <= 1.0 || t > 1.0e12 {
            continue;
        }
        m10.push((t, mant_dist(&cm10, t)));
        m11.push((t, mant_dist(&cm11, t)));
    }
    m10.sort_by(|a, b| a.1.total_cmp(&b.1));
    m11.sort_by(|a, b| a.1.total_cmp(&b.1));
    eprintln!("Relation 2 mantissa (closest hits per probe):");
    for (t, d) in m10.iter().take(5) {
        eprintln!("  P10: {t} ↔ ΔMant {d:.3e}");
    }
    for (t, d) in m11.iter().take(5) {
        eprintln!("  P11: {t} ↔ ΔMant {d:.3e}");
    }
    let win = |m: &[(f64, f64)]| m.iter().filter(|&&(_, d)| d < 0.001).count();
    eprintln!(
        "  0.1%-window saturates: P10 {}/{}, P11 {}/{} text-numbers hit",
        win(&m10),
        m10.len(),
        win(&m11),
        m11.len()
    );

    let mut rng = 0x9E3779B97F4A7C15u64;
    eprintln!("Relation 3 — i/1000 blocks:");
    let s10d = i1000_series(&obs10);
    let s11d = i1000_series(&obs11);
    let s10c = i1000_series(&car10);
    let s11c = i1000_series(&car11);
    corr_report("P10×Doppler", &tokens, &s10d, &mut rng);
    corr_report("P11×Doppler", &tokens, &s11d, &mut rng);
    corr_report("P10×carrier", &tokens, &s10c, &mut rng);
    corr_report("P11×carrier", &tokens, &s11c, &mut rng);
    eprintln!("Relation 3 — J2000 daily medians:");
    let d10d = j2000_series(&tdb10, &obs10);
    let d11d = j2000_series(&tdb11, &obs11);
    let d10c = j2000_series(&tdb10, &car10);
    let d11c = j2000_series(&tdb11, &car11);
    corr_report("P10×Doppler", &tokens, &d10d, &mut rng);
    corr_report("P11×Doppler", &tokens, &d11d, &mut rng);
    corr_report("P10×carrier", &tokens, &d10c, &mut rng);
    corr_report("P11×carrier", &tokens, &d11c, &mut rng);

    let map10 = j2000_map(&tdb10, &obs10);
    let map11 = j2000_map(&tdb11, &obs11);
    let shared: Vec<i64> = map10
        .keys()
        .filter(|k| map11.contains_key(k))
        .copied()
        .collect();
    let a: Vec<f64> = shared.iter().map(|k| map10[k]).collect();
    let b: Vec<f64> = shared.iter().map(|k| map11[k]).collect();
    let r = corr(&a, &b, 0);
    eprintln!(
        "Cross-probe: {} shared days, Pearson r = {r:.4} (-> {:.3})",
        shared.len(),
        r
    );
}
