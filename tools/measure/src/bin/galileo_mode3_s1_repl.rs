use std::collections::BTreeMap;

use omegaflow::te::{
    conditional_te_stats, surrogate_stats_block, surrogate_stats_phase, transfer_entropy_conditional,
    transfer_entropy_lag,
};

const LOCK_HZ: f64 = 1.0e3;
const MIN_SAMP: usize = 30;
const N_SURR: usize = 20;
const BLOCK: usize = 5;
const SEED0: u64 = 0x9E37_79B9_7F4A_7C15;
const SEEDS: [u64; 10] = [
    SEED0,
    0x9E37_79B9_7F4A_7C16,
    1,
    2,
    7,
    42,
    0xDEAD_BEEF,
    0x0BAD_5EED,
    0x1234_5678_9ABC_DEF0,
    u64::MAX,
];

fn unix_day(tdb: f64) -> i64 {
    let jd = 2451545.0 + tdb / 86400.0;
    (jd - 2440587.5).round() as i64
}

fn month_index(day: i64) -> Option<i64> {
    let (y, m, _) = omegaflow::spectral::civil_from_days(day)?;
    Some(y as i64 * 12 + m as i64)
}

fn median(v: &mut [f64]) -> Option<f64> {
    if v.is_empty() {
        return None;
    }
    v.sort_by(f64::total_cmp);
    Some(v[v.len() / 2])
}

fn rms(v: &[f64]) -> Option<f64> {
    if v.is_empty() {
        return None;
    }
    let n = v.len() as f64;
    let mean = v.iter().sum::<f64>() / n;
    Some((v.iter().map(|x| (x - mean) * (x - mean)).sum::<f64>() / n).sqrt())
}

fn fmt(v: Option<f64>) -> String {
    match v {
        Some(x) => format!("{x:.4e}"),
        None => "-".to_string(),
    }
}

fn series_var(v: &[f32]) -> Option<f64> {
    if v.is_empty() {
        return None;
    }
    let n = v.len() as f64;
    let mean = v.iter().map(|&x| x as f64).sum::<f64>() / n;
    Some(v.iter().map(|&x| (x as f64 - mean) * (x as f64 - mean)).sum::<f64>() / n)
}

fn distinct_count(v: &[f32], tol: f64) -> usize {
    let mut w: Vec<f64> = v.iter().map(|&x| (x as f64 / tol).round()).collect();
    w.sort_by(f64::total_cmp);
    w.dedup();
    w.len()
}

#[derive(Default)]
struct Bin63 {
    vals: Vec<f32>,
    refs: Vec<f64>,
    strn: Vec<f64>,
}

struct RDay {
    day: i64,
    dom: i64,
    cap3: bool,
    m1: usize,
    m2: usize,
    m3: usize,
}

fn segments(days: &[i64]) -> Vec<(i64, i64, usize)> {
    let mut out: Vec<(i64, i64, usize)> = Vec::new();
    let mut i = 0usize;
    while i < days.len() {
        let mut j = i;
        while j + 1 < days.len() && days[j + 1] == days[j] + 1 {
            j += 1;
        }
        out.push((days[i], days[j], j - i + 1));
        i = j + 1;
    }
    out
}

fn histogram(lens: &[usize]) -> BTreeMap<usize, usize> {
    let mut h: BTreeMap<usize, usize> = BTreeMap::new();
    for &l in lens {
        *h.entry(l).or_insert(0) += 1;
    }
    h
}

fn table(label: &str, drv: &[f32], tgt: &[f32], era: &[f32]) {
    let n = drv.len();
    println!(
        "== {label} | n = {n} | drvVar {} tgtVar {} | drvLvl {} | eraLv {}",
        fmt(series_var(drv)),
        fmt(series_var(tgt)),
        distinct_count(drv, 1.0),
        distinct_count(era, 1.0)
    );
    println!("    dir | lag |      TE |   thrPh |   thrBl |     cTE |    cThr");
    for &lag in &[1usize, 2, 3, 5] {
        let te = transfer_entropy_lag(tgt, drv, lag);
        let ph = surrogate_stats_phase(tgt, drv, lag, SEED0).map(|(_, _, t)| t);
        let bl = surrogate_stats_block(tgt, drv, lag, BLOCK, SEED0).map(|(_, _, t)| t);
        let ct = transfer_entropy_conditional(tgt, drv, era, lag);
        let cthr = conditional_te_stats(tgt, drv, era, lag, SEED0, N_SURR).map(|(_, _, t)| t);
        println!(
            "    D->T | {lag:>2} | {} | {} | {} | {} | {}",
            fmt(te),
            fmt(ph),
            fmt(bl),
            fmt(ct),
            fmt(cthr)
        );
        let rte = transfer_entropy_lag(drv, tgt, lag);
        let rph = surrogate_stats_phase(drv, tgt, lag, SEED0).map(|(_, _, t)| t);
        let rbl = surrogate_stats_block(drv, tgt, lag, BLOCK, SEED0).map(|(_, _, t)| t);
        let rct = transfer_entropy_conditional(drv, tgt, era, lag);
        let rcthr = conditional_te_stats(drv, tgt, era, lag, SEED0, N_SURR).map(|(_, _, t)| t);
        println!(
            "    T->D | {lag:>2} | {} | {} | {} | {} | {}",
            fmt(rte),
            fmt(rph),
            fmt(rbl),
            fmt(rct),
            fmt(rcthr)
        );
    }
    println!();
}

fn row3(label: &str, drv: &[f32], tgt: &[f32], era: &[f32]) {
    let n = drv.len();
    println!(
        "-- {label} lag3 | n {n} | drvVar {} tgtVar {} drvLvl {} eraLv {} | TE {} thrPh {} thrBl {} cTE {} cThr0 {}",
        fmt(series_var(drv)),
        fmt(series_var(tgt)),
        distinct_count(drv, 1.0),
        distinct_count(era, 1.0),
        fmt(transfer_entropy_lag(tgt, drv, 3)),
        fmt(surrogate_stats_phase(tgt, drv, 3, SEED0).map(|(_, _, t)| t)),
        fmt(surrogate_stats_block(tgt, drv, 3, BLOCK, SEED0).map(|(_, _, t)| t)),
        fmt(transfer_entropy_conditional(tgt, drv, era, 3)),
        fmt(conditional_te_stats(tgt, drv, era, 3, SEED0, N_SURR).map(|(_, _, t)| t)),
    );
}

fn seed_repl3(label: &str, drv: &[f32], tgt: &[f32], era: &[f32]) {
    let n = drv.len();
    if n < 8 {
        println!("-- {label} seed-repl lag3 | n {n} < 8 -> no verdict (measured limit)");
        return;
    }
    let Some(cte) = transfer_entropy_conditional(tgt, drv, era, 3) else {
        println!("-- {label} seed-repl lag3 | conditional TE not defined (degenerate series)");
        return;
    };
    let mut cross: Vec<u64> = Vec::new();
    for &s in &SEEDS {
        if let Some((_, _, t)) = conditional_te_stats(tgt, drv, era, 3, s, N_SURR) {
            if cte > t {
                cross.push(s);
            }
        }
    }
    println!(
        "-- {label} seed-repl lag3 | cTE {cte:.4e} | cThr(20-surr) crossed in {}/{} seeds | crossed seeds: {:x?}",
        cross.len(),
        SEEDS.len(),
        cross
    );
    if let Some((mu, sd, thr)) = conditional_te_stats(tgt, drv, era, 3, SEED0, 600) {
        let sd = sd.max(1e-300);
        let z = (cte - mu) / sd;
        println!(
            "   pooled null N=600 seed0: mean {mu:.4e} sd {sd:.4e} thr {thr:.4e} | z {z:.2} | cTE-thr {:.4e}",
            cte - thr
        );
    }
}

fn chain_in(rows: &[RDay], lo: i64, hi: i64) -> Option<(i64, i64)> {
    let inr: Vec<i64> = rows.iter().map(|r| r.day).filter(|&d| d >= lo && d <= hi).collect();
    if inr.is_empty() {
        return None;
    }
    let segs = segments(&inr);
    let best = segs.iter().max_by_key(|x| x.2)?;
    Some((best.0, best.1))
}

fn main() {
    let path = std::env::args()
        .skip(1)
        .find(|a| !a.starts_with('-'))
        .unwrap_or_else(|| "data/galileo_resid.bin".to_string());
    let bytes = std::fs::read(&path).expect("resid bin read");
    if bytes.len() < 8 || &bytes[0..4] != b"GASR" {
        println!("no GASR header");
        return;
    }
    let count = u32::from_le_bytes(bytes[4..8].try_into().expect("count")) as usize;
    if bytes.len() != 8 + count * 64 {
        println!("length mismatch");
        return;
    }
    let mut cnt: BTreeMap<(i64, i64, i64), usize> = BTreeMap::new();
    let mut bins63: BTreeMap<i64, Bin63> = BTreeMap::new();
    let mut mode_tot = [0usize; 3];
    let mut n_lock = 0usize;
    let mut n_zero = 0usize;
    let mut n_nan = 0usize;
    let mut cleaned = 0usize;
    for i in 0..count {
        let base = 8 + i * 64;
        let rd = |k: usize| -> f64 {
            let mut b = [0u8; 8];
            b.copy_from_slice(&bytes[base + k * 8..base + k * 8 + 8]);
            f64::from_le_bytes(b)
        };
        let rec: [f64; 8] = [rd(0), rd(1), rd(2), rd(3), rd(4), rd(5), rd(6), rd(7)];
        if !rec[1].is_finite() {
            n_nan += 1;
            continue;
        }
        if rec[1].abs() > LOCK_HZ {
            n_lock += 1;
            continue;
        }
        if rec[7] == 0.0 {
            n_zero += 1;
            continue;
        }
        let mode = rec[3] as i64;
        if (1..=3).contains(&mode) {
            mode_tot[(mode - 1) as usize] += 1;
        }
        let day = unix_day(rec[0]);
        let st = rec[2] as i64;
        *cnt.entry((day, st, mode)).or_insert(0) += 1;
        if st == 63 && mode == 1 && (9830..=9930).contains(&day) {
            let b = bins63.entry(day).or_default();
            b.vals.push(rec[1].abs() as f32);
            b.refs.push(rec[5]);
            b.strn.push(rec[7]);
        }
        cleaned += 1;
    }
    println!(
        "GASR {path}: {count} records, cleaned {cleaned}, lock {n_lock}, zero-strength {n_zero}, non-finite {n_nan}"
    );
    println!("mode_tot(cleaned) m1/m2/m3 = {mode_tot:?}");

    let mut strows: BTreeMap<i64, Vec<RDay>> = BTreeMap::new();
    {
        let mut stday: BTreeMap<i64, BTreeMap<i64, Vec<(i64, usize)>>> = BTreeMap::new();
        for (&(day, st, mode), &n) in &cnt {
            if n >= MIN_SAMP {
                stday.entry(st).or_default().entry(day).or_default().push((mode, n));
            }
        }
        for (st, dm) in stday {
            let mut v = Vec::new();
            for (day, bins) in dm {
                let mut dom = bins[0].0;
                let mut best = bins[0].1;
                let mut cap3 = false;
                let mut m1 = 0usize;
                let mut m2 = 0usize;
                let mut m3 = 0usize;
                for &(m, n) in &bins {
                    match m {
                        1 => m1 = n,
                        2 => m2 = n,
                        _ => m3 = n,
                    }
                    if m == 3 {
                        cap3 = true;
                    }
                    if n >= best {
                        dom = m;
                        best = n;
                    }
                }
                v.push(RDay { day, dom, cap3, m1, m2, m3 });
            }
            strows.insert(st, v);
        }
    }

    println!();
    println!("=== (A) Mission Mode-3 structure (qualifying day: some (day,station,mode) bin has >= {MIN_SAMP} cleaned samples; dom = dominant qualifying mode-bin of the day)");
    let mut all_dom3 = 0usize;
    let mut best_dom3: Option<(i64, i64, usize, i64)> = None;
    let mut best_cap3: Option<(i64, i64, usize, i64)> = None;
    for (st, rows) in &strows {
        let q = rows.len();
        let mut d1 = 0usize;
        let mut d2 = 0usize;
        let mut d3 = 0usize;
        let mut cap = 0usize;
        let mut dom3: Vec<i64> = Vec::new();
        let mut cap3d: Vec<i64> = Vec::new();
        for r in rows {
            match r.dom {
                1 => d1 += 1,
                2 => d2 += 1,
                _ => d3 += 1,
            }
            if r.cap3 {
                cap += 1;
                cap3d.push(r.day);
            }
            if r.dom == 3 {
                dom3.push(r.day);
            }
        }
        all_dom3 += d3;
        let s3 = segments(&dom3);
        let s3c = segments(&cap3d);
        let longest3 = s3.iter().max_by_key(|x| x.2);
        let longest3c = s3c.iter().max_by_key(|x| x.2);
        if let Some(x) = longest3 {
            let (s, e, l) = (x.0, x.1, x.2);
            if best_dom3.map_or(true, |(_, _, bl, _)| l > bl) {
                best_dom3 = Some((s, e, l, *st));
            }
        }
        if let Some(x) = longest3c {
            let (s, e, l) = (x.0, x.1, x.2);
            if best_cap3.map_or(true, |(_, _, bl, _)| l > bl) {
                best_cap3 = Some((s, e, l, *st));
            }
        }
        let mut hl: Vec<usize> = s3.iter().map(|x| x.2).collect();
        hl.sort_by(|a, b| b.cmp(a));
        let mut hlc: Vec<usize> = s3c.iter().map(|x| x.2).collect();
        hlc.sort_by(|a, b| b.cmp(a));
        let h3 = histogram(&hl);
        let h3c = histogram(&hlc);
        let h3s: Vec<String> = h3.iter().map(|(k, v)| format!("len{k}:{v}")).collect();
        let h3cs: Vec<String> = h3c.iter().map(|(k, v)| format!("len{k}:{v}")).collect();
        println!("st {st}: qualDays {q} | dom1 {d1} dom2 {d2} dom3 {d3} | mode3SampleDays {cap}");
        println!(
            "   m3dom {d3} days in {} segments | seg-len-dist {{{}}}",
            s3.len(),
            h3s.join(" ")
        );
        if let Some(x) = longest3 {
            println!("   longest m3dom segment {}-{} ({} d)", x.0, x.1, x.2);
        }
        let long3: Vec<String> = s3.iter().filter(|x| x.2 >= 3).map(|(a, b, l)| format!("{a}-{b}({l})")).collect();
        if !long3.is_empty() {
            println!("   m3dom segs >=3d: {}", long3.join(" "));
        }
        println!(
            "   mode3Sample {cap} days in {} segments | seg-len-dist {{{}}}",
            s3c.len(),
            h3cs.join(" ")
        );
        if let Some(x) = longest3c {
            println!("   longest mode3Sample segment {}-{} ({} d)", x.0, x.1, x.2);
        }
        let long3c: Vec<String> = s3c.iter().filter(|x| x.2 >= 3).map(|(a, b, l)| format!("{a}-{b}({l})")).collect();
        if !long3c.is_empty() {
            println!("   mode3Sample segs >=3d: {}", long3c.join(" "));
        }
    }
    println!("mission: mode3-dominant days total {all_dom3}");
    match best_dom3 {
        Some((s, e, l, st)) => println!("longest mode3-dominant window: st{st} {s}-{e} = {l} contiguous days"),
        None => println!("longest mode3-dominant window: none"),
    }
    match best_cap3 {
        Some((s, e, l, st)) => println!("longest mode3-sample window: st{st} {s}-{e} = {l} contiguous days"),
        None => println!("longest mode3-sample window: none"),
    }

    let Some(r63) = strows.get(&63) else {
        println!("station 63 has no qualifying days");
        return;
    };
    println!();
    println!("=== (B) station 63 qualifying-day structure around the isolate (day: dom | m1/m2/m3 cleaned counts; '-' = not a qualifying day)");
    for day in 9856..=9914 {
        match r63.iter().find(|r| r.day == day) {
            Some(r) => println!("  {day}: dom{} m1 {} m2 {} m3 {} mode3Sample {}", r.dom, r.m1, r.m2, r.m3, r.cap3),
            None => println!("  {day}: -"),
        }
    }

    let wdefs: [(&str, i64, i64); 4] = [
        ("prev-run 9858-9877", 9858, 9877),
        ("shift-left 9878-9903", 9878, 9903),
        ("isolate 9883-9908", 9883, 9908),
        ("shift-right 9888-9913", 9888, 9913),
    ];
    for (label, lo, hi) in wdefs {
        let Some((d0, d1)) = chain_in(r63, lo, hi) else {
            println!("\n### {label}: no qualifying day in {lo}-{hi}");
            continue;
        };
        let mut dd = [0usize; 4];
        let mut sel: Vec<&RDay> = Vec::new();
        for r in r63 {
            if r.day >= d0 && r.day <= d1 {
                if (1..=3).contains(&r.dom) {
                    dd[r.dom as usize] += 1;
                }
                if r.dom == 1 {
                    sel.push(r);
                }
            }
        }
        let seln = sel.len();
        let mut refd: Vec<f32> = Vec::new();
        let mut strd: Vec<f32> = Vec::new();
        let mut medt: Vec<f32> = Vec::new();
        let mut rmst: Vec<f32> = Vec::new();
        let mut era: Vec<f32> = Vec::new();
        let mut missing = 0usize;
        for r in &sel {
            let Some(b) = bins63.get(&r.day) else {
                missing += 1;
                continue;
            };
            let mut av: Vec<f64> = b.vals.iter().map(|&x| x as f64).collect();
            let Some(md) = median(&mut av) else {
                missing += 1;
                continue;
            };
            let Some(rm) = rms(&av) else {
                missing += 1;
                continue;
            };
            let mut rv = b.refs.clone();
            let Some(rf) = median(&mut rv) else {
                missing += 1;
                continue;
            };
            let mut sv = b.strn.clone();
            let Some(st) = median(&mut sv) else {
                missing += 1;
                continue;
            };
            let Some(mi) = month_index(r.day) else {
                missing += 1;
                continue;
            };
            refd.push(rf as f32);
            strd.push(st as f32);
            medt.push(md as f32);
            rmst.push(rm as f32);
            era.push(mi as f32);
        }
        let n = refd.len();
        println!();
        println!(
            "### {label}: block {d0}-{d1} ({} d) | dom-day counts 1:{},2:{},3:{} | mode1-controlled n {seln}, series n {n}, missing {missing}",
            d1 - d0 + 1,
            dd[1],
            dd[2],
            dd[3]
        );
        if n < 8 {
            println!("    n < 8 -> no TE verdict (measured limit)");
            continue;
        }
        let l_s1 = format!("S1 ref->med | st63 {d0}-{d1} mode1");
        let l_rms = format!("S1 ref->rms | st63 {d0}-{d1} mode1");
        let l_str = format!("S1 strength->med | st63 {d0}-{d1} mode1");
        table(&l_s1, &refd, &medt, &era);
        row3(&l_rms, &refd, &rmst, &era);
        row3(&l_str, &strd, &medt, &era);
        seed_repl3(&l_s1, &refd, &medt, &era);
        seed_repl3(&l_str, &strd, &medt, &era);
    }
}
