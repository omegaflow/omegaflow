use std::fs;

const BASE: &str = "sources/nssdc/PSCM-00001/";
const FILES: [&str; 2] = ["dr004487_f00001.phys.1", "dr004487_f00002.phys.1"];
const UNIT_S: f64 = 8.0;

struct Point {
    w: Vec<u64>,
}

fn records(d: &[u8]) -> Vec<&[u8]> {
    let mut out = Vec::new();
    let mut i = 0usize;
    while i + 2 <= d.len() {
        let n = u16::from_be_bytes([d[i], d[i + 1]]) as usize;
        i += 2;
        let end = (i + n).min(d.len());
        out.push(&d[i..end]);
        i = end;
    }
    out
}

fn word(chars: &[u8]) -> Option<u64> {
    if chars.len() < 6 {
        return None;
    }
    let mut v = 0u64;
    for &c in chars {
        v = (v << 6) | u64::from(c & 0x3F);
    }
    Some(v)
}

fn points(d: &[u8]) -> Vec<Point> {
    let mut pts = Vec::new();
    for (ri, rec) in records(d).into_iter().enumerate().skip(1) {
        if rec.len() < 1206 {
            continue;
        }
        let mut block: Vec<u64> = Vec::with_capacity(25);
        let mut w8: Vec<[u64; 8]> = Vec::with_capacity(25);
        for blk in 0..25 {
            let mut w = Vec::with_capacity(8);
            for k in 0..8 {
                let off = blk * 48 + 6 + 6 * k;
                let Some(chars) = rec.get(off..off + 6) else {
                    return pts;
                };
                let Some(v) = word(chars) else {
                    return pts;
                };
                w.push(v);
            }
            block.push(w[0]);
            let mut a = [0u64; 8];
            a.copy_from_slice(&w);
            w8.push(a);
        }
        let rising = block.windows(2).all(|p| p[1] > p[0]);
        if !rising && ri < 3 {
            eprintln!("  Kopf pruefen: Satz {ri} nicht steigend, erster Zeitwert {}", block[0]);
        }
        for a in w8 {
            pts.push(Point { w: a.to_vec() });
        }
    }
    pts
}

fn median(mut v: Vec<f64>) -> Option<f64> {
    if v.is_empty() {
        return None;
    }
    v.sort_by(f64::total_cmp);
    let n = v.len();
    Some(if n % 2 == 1 {
        v[n / 2]
    } else {
        0.5 * (v[n / 2 - 1] + v[n / 2])
    })
}

fn run(name: &str, file: &str) {
    let path = format!("{BASE}{file}");
    let Some(d) = fs::read(&path).ok() else {
        eprintln!("{name}: Datei fehlt ({path})");
        return;
    };
    let pts = points(&d);
    if let Some(head) = records(&d).first()
        && head.len() >= 1206
    {
        let mut zeros = 0usize;
        let mut seen = 0usize;
        let mut other: Vec<u64> = Vec::new();
        for blk in 0..25 {
            let Some(chars) = head.get(blk * 48 + 6..blk * 48 + 12) else {
                break;
            };
            let Some(v) = word(chars) else {
                break;
            };
            seen += 1;
            if v == 0 {
                zeros += 1;
            } else {
                other.push(v);
            }
        }
        eprintln!("{name}: Kopf-Satz {zeros} von {seen} Zeitwoertern null, andere {other:?}");
    }
    if pts.len() < 3 {
        eprintln!("{name}: keine Punkte ({path})");
        return;
    }

    let t: Vec<u64> = pts.iter().map(|p| p.w[0]).collect();
    let span = t[t.len() - 1].saturating_sub(t[0]);

    let mut distinct = [0usize; 8];
    for (k, d) in distinct.iter_mut().enumerate() {
        let mut vals: Vec<u64> = pts.iter().map(|p| p.w[k]).collect();
        vals.sort_unstable();
        vals.dedup();
        *d = vals.len();
    }
    eprintln!(
        "{name}: {} Punkte, Zeitwort {} -> {} (Spanne {}, {:.1} Tage bei 8 s)",
        pts.len(),
        t[0],
        t[t.len() - 1],
        span,
        span as f64 * UNIT_S / 86400.0
    );
    eprintln!("{name}: erste Zeitwerte {:?}", &t[..5.min(t.len())]);
    eprintln!("{name}: verschiedene Werte je Wort: {distinct:?}");

    let mut tsteps: std::collections::BTreeMap<u64, usize> = std::collections::BTreeMap::new();
    let mut w6: Vec<u64> = Vec::with_capacity(pts.len());
    for p in &pts {
        w6.push(p.w[5]);
    }
    let mut up = 0usize;
    let mut down = 0usize;
    for i in 1..pts.len() {
        let dt = t[i].saturating_sub(t[i - 1]);
        *tsteps.entry(dt).or_default() += 1;
        if w6[i] > w6[i - 1] {
            up += 1;
        } else if w6[i] < w6[i - 1] {
            down += 1;
        }
    }
    let mut ts: Vec<(u64, usize)> = tsteps.into_iter().collect();
    ts.sort_by_key(|a| std::cmp::Reverse(a.1));
    let top: Vec<String> = ts
        .iter()
        .take(6)
        .map(|(s, n)| format!("{s}x{n}"))
        .collect();
    eprintln!("{name}: Zeitschritte haeufigste [{}]", top.join(" "));
    eprintln!(
        "{name}: Wort 6 erst {} letzt {}, Richtung auf {up} ab {down}",
        w6[0],
        w6[w6.len() - 1]
    );

    for i in 1..pts.len() {
        if t[i] < t[i - 1] {
            eprintln!(
                "{name}: Zeitsprung rueckwaerts Punkt {i}: {} -> {} ({})",
                t[i - 1],
                t[i],
                (t[i] as i64) - (t[i - 1] as i64)
            );
        }
    }

    let mut rate: Vec<(usize, f64, f64)> = Vec::new();
    for i in 1..pts.len() {
        if t[i] <= t[i - 1] {
            continue;
        }
        let dt_s = (t[i] - t[i - 1]) as f64 * UNIT_S;
        let dw = (w6[i] as i64 - w6[i - 1] as i64) as f64;
        rate.push((i, t[i] as f64, dw / dt_s));
    }
    if rate.is_empty() {
        eprintln!("{name}: keine vorwaerts laufende Zeit, keine Rate");
        return;
    }
    let mut abs: Vec<(usize, f64, f64)> = rate.iter().map(|&(i, tt, r)| (i, tt, r.abs())).collect();
    abs.sort_by(|a, b| b.2.total_cmp(&a.2));
    let peak = abs[0];
    let peak_frac = (t[peak.0] - t[0]) as f64 / span as f64;
    eprintln!(
        "{name}: groesste |dWort6/dt| {:.3e} je s bei Punkt {} (t {}, {:.1} % der Spanne, {:.2} Tage nach Start)",
        peak.2,
        peak.0,
        t[peak.0],
        100.0 * peak_frac,
        (t[peak.0] - t[0]) as f64 * UNIT_S / 86400.0
    );
    let top5: Vec<String> = abs
        .iter()
        .take(5)
        .map(|(i, _, a)| format!("P{}:{a:.2e}", i))
        .collect();
    eprintln!("{name}: fuenf groesste Raeten [{}]", top5.join(" "));

    let med = median(rate.iter().map(|&(_, _, r)| r.abs()).collect());
    if let Some(m) = med {
        eprintln!(
            "{name}: |dWort6/dt| Median {m:.3e}, Spitze/Median {:.1}",
            peak.2 / m
        );
    }

    let mut bins = [0usize; 100];
    for &(i, _, _) in &rate {
        let f = (t[i] - t[0]) as f64 / span as f64;
        let b = ((f * 100.0) as usize).min(99);
        bins[b] += 1;
    }
    let mut densest = (0usize, 0usize);
    for (b, &n) in bins.iter().enumerate() {
        if n > densest.1 {
            densest = (b, n);
        }
    }
    let (db, dn) = densest;
    eprintln!(
        "{name}: dichtestes Hundertstel bei {} % ({} Punkte im Zeitfenster)",
        db, dn
    );

    let mut flyby = 0usize;
    let mut best = f64::INFINITY;
    for (i, &tt) in t.iter().enumerate() {
        let d = ((tt - t[0]) as f64 / span as f64 - 0.79).abs();
        if d < best {
            best = d;
            flyby = i;
        }
    }
    for (label, center) in [("Spitze", peak.0), ("Vorbeiflug", flyby)] {
        let lo = center.saturating_sub(3);
        let hi = (center + 4).min(pts.len());
        let mut parts: Vec<String> = Vec::new();
        for i in lo..hi {
            let dt = if i == 0 { 0 } else { t[i] - t[i - 1] };
            parts.push(format!(
                "P{i} t={} dt={} w6={:.3e} hex={:09x}",
                t[i],
                dt,
                w6[i] as f64,
                w6[i]
            ));
        }
        eprintln!("{name}: {label} {}", parts.join(" | "));
    }
    let mut prof: Vec<Vec<f64>> = vec![Vec::new(); 20];
    for &(i, _, r) in &rate {
        let f = (t[i] - t[0]) as f64 / span as f64;
        let b = ((f * 20.0) as usize).min(19);
        prof[b].push(r);
    }
    let mut line = String::new();
    for (b, v) in prof.iter().enumerate() {
        let m = median(v.clone());
        match m {
            Some(x) => line.push_str(&format!("{b}:{x:.1e} ")),
            None => line.push_str(&format!("{b}:— ")),
        }
    }
    eprintln!("{name}: Median dWort6/dt je 5 % der Spanne [1/s]: {line}");

    let mut val: Vec<Vec<f64>> = vec![Vec::new(); 20];
    for (i, &tt) in t.iter().enumerate() {
        let f = (tt - t[0]) as f64 / span as f64;
        let b = ((f * 20.0) as usize).min(19);
        val[b].push(w6[i] as f64);
    }
    let mut vline = String::new();
    for v in val.iter() {
        if let Some(m) = median(v.clone()) {
            vline.push_str(&format!("{:.6e} ", m));
        }
    }
    eprintln!("{name}: Median Wort6 je 5 % der Spanne: {vline}");

    let hi: Vec<u64> = w6.iter().map(|v| v >> 18).collect();
    let lo: Vec<u64> = w6.iter().map(|v| v & 0x3FFFF).collect();
    let mut hi_up = 0usize;
    let mut hi_down = 0usize;
    for i in 1..hi.len() {
        if hi[i] > hi[i - 1] {
            hi_up += 1;
        } else if hi[i] < hi[i - 1] {
            hi_down += 1;
        }
    }
    let mut lo_min = u64::MAX;
    let mut lo_max = 0u64;
    for &v in &lo {
        if v < lo_min {
            lo_min = v;
        }
        if v > lo_max {
            lo_max = v;
        }
    }
    eprintln!(
        "{name}: Wort6 hohe 18 Bit {} -> {} (auf {hi_up} ab {hi_down}), niedere 18 Bit {lo_min} bis {lo_max}",
        hi[0],
        hi[hi.len() - 1],
    );
    let mut hi_rate: Vec<(usize, f64)> = Vec::new();
    for i in 1..pts.len() {
        if t[i] <= t[i - 1] {
            continue;
        }
        let dt_s = (t[i] - t[i - 1]) as f64 * UNIT_S;
        hi_rate.push((i, ((hi[i] as i64 - hi[i - 1] as i64) as f64 / dt_s).abs()));
    }
    if !hi_rate.is_empty() {
        let mut s: Vec<(usize, f64)> = hi_rate.clone();
        s.sort_by(|a, b| b.1.total_cmp(&a.1));
        let (pi, pv) = s[0];
        eprintln!(
            "{name}: hohe 18 Bit groesste |Rate| {pv:.3e} je s bei Punkt {pi} ({:.1} % der Spanne)",
            (t[pi] - t[0]) as f64 / span as f64 * 100.0
        );
    }
    let mut max_i = 0usize;
    for i in 1..w6.len() {
        if w6[i] > w6[max_i] {
            max_i = i;
        }
    }
    eprintln!(
        "{name}: Wort6 Maximum bei Punkt {max_i} (t {}, {:.1} % der Spanne)",
        t[max_i],
        (t[max_i] - t[0]) as f64 / span as f64 * 100.0
    );

    for target in [1u64, 75u64] {
        let mut d: Vec<f64> = Vec::new();
        for i in 1..pts.len() {
            if t[i].saturating_sub(t[i - 1]) == target {
                d.push((w6[i] as i64 - w6[i - 1] as i64) as f64);
            }
        }
        if let Some(m) = median(d.clone()) {
            let mean = d.iter().sum::<f64>() / d.len() as f64;
            eprintln!(
                "{name}: dWort6 bei dt={} ({} s): n={} median {:.3e} mittel {:.3e}",
                target,
                (target as f64 * UNIT_S) as u64,
                d.len(),
                m,
                mean
            );
        }
    }
}

fn main() {
    for (i, f) in FILES.iter().enumerate() {
        run(&format!("f0000{}", i + 1), f);
    }
}
