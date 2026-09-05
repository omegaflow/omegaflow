use std::collections::{BTreeMap, BTreeSet};

const LOCK_HZ: f64 = 1.0e3;
const MIN_SAMP: usize = 30;
const MIN_MODE_DAYS: usize = 4;
const MIN_BLK_DAYS: usize = 8;
const MIN_REF_LEVELS: usize = 4;

struct Bin {
    n: usize,
    n_non1: usize,
    vals: Vec<f32>,
}

struct Row {
    st: i64,
    day: i64,
    mi: i64,
    mode: i64,
    domfrac: f64,
    arms_l: f64,
    med_l: f64,
    ref_med: f64,
    frac_non1: f64,
}

struct Within {
    st: i64,
    mi: i64,
    amode: i64,
    bmode: i64,
    del_med: f64,
    del_arms: f64,
}

fn unix_day(tdb: f64) -> i64 {
    let jd = 2451545.0 + tdb / 86400.0;
    (jd - 2440587.5).round() as i64
}

fn month_index(day: i64) -> Option<i64> {
    let (y, m, _) = omegaflow::spectral::civil_from_days(day)?;
    Some(y as i64 * 12 + m as i64)
}

fn label_of(day: i64) -> Option<String> {
    let (y, m, _) = omegaflow::spectral::civil_from_days(day)?;
    Some(format!("{y:04}-{m:02}"))
}

fn median_f64_of(v: &[f64]) -> Option<f64> {
    if v.is_empty() {
        return None;
    }
    let mut w = v.to_vec();
    w.sort_by(f64::total_cmp);
    Some(w[w.len() / 2])
}

fn day_level(v: &mut Vec<f64>) -> Option<(f64, f64)> {
    if v.is_empty() {
        return None;
    }
    let n = v.len() as f64;
    let ss = v.iter().map(|x| x * x).sum::<f64>();
    let rms = (ss / n).sqrt();
    v.sort_by(f64::total_cmp);
    let med = v[v.len() / 2];
    if rms > 0.0 && med > 0.0 {
        Some((rms.log10(), med.log10()))
    } else {
        None
    }
}

fn bin_med(v: &[f32]) -> Option<f64> {
    if v.is_empty() {
        return None;
    }
    let mut w: Vec<f64> = v.iter().map(|&x| x as f64).collect();
    w.sort_by(f64::total_cmp);
    Some(w[w.len() / 2])
}

fn bin_arms(v: &[f32]) -> Option<f64> {
    if v.is_empty() {
        return None;
    }
    let n = v.len() as f64;
    let ss = v.iter().map(|&x| (x as f64) * (x as f64)).sum::<f64>();
    Some((ss / n).sqrt())
}

fn ld(x: f64) -> Option<f64> {
    if x > 0.0 {
        Some(x.log10())
    } else {
        None
    }
}

fn avg_ranks(v: &[f64]) -> Vec<f64> {
    let mut ix: Vec<usize> = (0..v.len()).collect();
    ix.sort_by(|&a, &b| v[a].total_cmp(&v[b]));
    let mut r = vec![0.0f64; v.len()];
    let mut i = 0usize;
    while i < ix.len() {
        let mut j = i;
        while j + 1 < ix.len() && v[ix[j + 1]] == v[ix[i]] {
            j += 1;
        }
        let mr = 1.0 + (i + j) as f64 / 2.0;
        for k in i..=j {
            r[ix[k]] = mr;
        }
        i = j + 1;
    }
    r
}

fn spearman(x: &[f64], y: &[f64]) -> Option<f64> {
    if x.len() != y.len() || x.len() < 3 {
        return None;
    }
    let rx = avg_ranks(x);
    let ry = avg_ranks(y);
    let n = rx.len() as f64;
    let mx = rx.iter().sum::<f64>() / n;
    let my = ry.iter().sum::<f64>() / n;
    let mut cov = 0.0f64;
    let mut vx = 0.0f64;
    let mut vy = 0.0f64;
    for i in 0..rx.len() {
        let dx = rx[i] - mx;
        let dy = ry[i] - my;
        cov += dx * dy;
        vx += dx * dx;
        vy += dy * dy;
    }
    if vx > 0.0 && vy > 0.0 {
        Some(cov / (vx * vy).sqrt())
    } else {
        None
    }
}

fn n_distinct_rounded(v: &[f64], tol: f64) -> usize {
    let mut w: Vec<f64> = v.iter().map(|&x| (x / tol).round()).collect();
    w.sort_by(f64::total_cmp);
    w.dedup();
    w.len()
}

fn binom_two_tail(n: usize, k: usize) -> Option<f64> {
    if n == 0 || k > n {
        return None;
    }
    let nf = n as f64;
    let ln2 = std::f64::consts::LN_2;
    let mut ln_fact = vec![0.0f64; n + 1];
    for i in 1..=n {
        ln_fact[i] = ln_fact[i - 1] + (i as f64).ln();
    }
    let p_eq = |t: usize| -> f64 {
        let lc = ln_fact[n] - ln_fact[t] - ln_fact[n - t] - nf * ln2;
        lc.exp()
    };
    let tail_sum = |from: usize, to: usize| -> f64 {
        let mut s = 0.0f64;
        for t in from..=to {
            s += p_eq(t);
        }
        s
    };
    let t = if k <= n / 2 {
        tail_sum(0, k)
    } else {
        tail_sum(0, n) - tail_sum(0, k - 1)
    };
    Some((2.0 * t).min(1.0))
}

fn sub_med(a: &[f64], b: &[f64]) -> Option<f64> {
    match (median_f64_of(a), median_f64_of(b)) {
        (Some(x), Some(y)) => Some(x - y),
        _ => None,
    }
}

fn process_day(
    day: i64,
    st: i64,
    dbins: &[(i64, &Bin)],
    refs: &BTreeMap<(i64, i64), Vec<f64>>,
    rows: &mut Vec<Row>,
    within: &mut Vec<Within>,
    dropped: &mut usize,
) {
    let qb: Vec<(i64, &Bin)> = dbins
        .iter()
        .copied()
        .filter(|(mo, b)| (1..=3).contains(mo) && b.n >= MIN_SAMP)
        .collect();
    if qb.is_empty() {
        return;
    }
    let tot: usize = qb.iter().map(|(_, b)| b.n).sum();
    let Some(mi) = month_index(day) else {
        return;
    };
    let (dm, db) = match qb.iter().max_by(|(_, a), (_, b)| a.n.cmp(&b.n)) {
        Some(x) => x,
        None => return,
    };
    let domfrac = db.n as f64 / tot as f64;
    let mut vals: Vec<f64> = Vec::with_capacity(tot);
    for (_, b) in &qb {
        for &x in &b.vals {
            vals.push(x as f64);
        }
    }
    let Some((arms_l, med_l)) = day_level(&mut vals) else {
        *dropped += 1;
        return;
    };
    let non1: usize = qb.iter().map(|(_, b)| b.n_non1).sum();
    let frac_non1 = non1 as f64 / tot as f64;
    let Some(rv) = refs.get(&(day, st)) else {
        return;
    };
    let Some(ref_med) = median_f64_of(rv) else {
        return;
    };
    rows.push(Row {
        st,
        day,
        mi,
        mode: *dm,
        domfrac,
        arms_l,
        med_l,
        ref_med,
        frac_non1,
    });
    for ai in 0..qb.len() {
        for bi in (ai + 1)..qb.len() {
            let (ma, ba) = qb[ai];
            let (mb, bb) = qb[bi];
            if ma == mb {
                continue;
            }
            let dmed = match (bin_med(&ba.vals).and_then(ld), bin_med(&bb.vals).and_then(ld)) {
                (Some(x), Some(y)) => x - y,
                _ => continue,
            };
            let darm = match (bin_arms(&ba.vals).and_then(ld), bin_arms(&bb.vals).and_then(ld)) {
                (Some(x), Some(y)) => x - y,
                _ => continue,
            };
            within.push(Within {
                st,
                mi,
                amode: ma,
                bmode: mb,
                del_med: dmed,
                del_arms: darm,
            });
        }
    }
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
    let mut bins: BTreeMap<(i64, i64, i64), Bin> = BTreeMap::new();
    let mut refs: BTreeMap<(i64, i64), Vec<f64>> = BTreeMap::new();
    let mut total = 0usize;
    let mut cleaned = 0usize;
    let mut n_lock = 0usize;
    let mut n_zero = 0usize;
    let mut n1s = 0usize;
    let mut n_non1 = 0usize;
    let mut n_mode_other = 0usize;
    for i in 0..count {
        let base = 8 + i * 64;
        let rd = |k: usize| {
            let mut buf = [0u8; 8];
            buf.copy_from_slice(&bytes[base + k * 8..base + k * 8 + 8]);
            f64::from_le_bytes(buf)
        };
        let rec: [f64; 8] = [rd(0), rd(1), rd(2), rd(3), rd(4), rd(5), rd(6), rd(7)];
        total += 1;
        if !rec[1].is_finite() {
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
        cleaned += 1;
        let day = unix_day(rec[0]);
        let st = rec[2] as i64;
        let mo = rec[3] as i64;
        let non1 = (rec[6] * 10.0).round() as i64 != 10;
        if non1 {
            n_non1 += 1;
        } else {
            n1s += 1;
        }
        if !(1..=3).contains(&mo) {
            n_mode_other += 1;
        }
        refs.entry((day, st)).or_insert_with(Vec::new).push(rec[5]);
        let b = bins
            .entry((day, st, mo))
            .or_insert_with(|| Bin { n: 0, n_non1: 0, vals: Vec::new() });
        b.n += 1;
        if non1 {
            b.n_non1 += 1;
        }
        b.vals.push(rec[1].abs() as f32);
    }
    println!(
        "GASR {path}: {total} records, cleaned {cleaned} (|resid|<={LOCK_HZ:.0} Hz, strength != 0, finite), lock {n_lock}, zero-strength {n_zero}"
    );
    println!(
        "cleaned cadence: {n1s} at 1 s, {n_non1} other (frac1s {:.6}); cleaned samples with mode outside 1..=3: {n_mode_other}",
        n1s as f64 / (n1s + n_non1).max(1) as f64
    );
    let mut rows: Vec<Row> = Vec::new();
    let mut within: Vec<Within> = Vec::new();
    let mut day_drop = 0usize;
    let mut cur_key: Option<(i64, i64)> = None;
    let mut day_bins: Vec<(i64, &Bin)> = Vec::new();
    for ((d, st, mo), b) in bins.iter() {
        let key = (*d, *st);
        if cur_key != Some(key) {
            if let Some((pd, ps)) = cur_key.take() {
                process_day(pd, ps, &day_bins, &refs, &mut rows, &mut within, &mut day_drop);
            }
            day_bins.clear();
            cur_key = Some(key);
        }
        day_bins.push((*mo, b));
    }
    if let Some((pd, ps)) = cur_key.take() {
        process_day(pd, ps, &day_bins, &refs, &mut rows, &mut within, &mut day_drop);
    }
    println!(
        "qualifying day-rows: {} (>=1 mode-bin with >= {MIN_SAMP} cleaned samples, dominant mode 1..=3); {day_drop} days dropped (level degenerate in log10)",
        rows.len()
    );
    println!(
        "within-day mode-bin pair records (>=2 modes with >= {MIN_SAMP} samples same day): {}",
        within.len()
    );
    rows.sort_by(|a, b| (a.st, a.mi, a.day).cmp(&(b.st, b.mi, b.day)));
    let mut within_map: BTreeMap<(i64, i64), Vec<&Within>> = BTreeMap::new();
    for w in &within {
        within_map
            .entry((w.st, w.mi))
            .or_insert_with(Vec::new)
            .push(w);
    }
    let mut d12_rows: Vec<(String, f64, f64, usize, usize)> = Vec::new();
    let mut d13_rows: Vec<(String, f64, f64, usize, usize)> = Vec::new();
    let mut d23_rows: Vec<(String, f64, f64, usize, usize)> = Vec::new();
    let mut ref_rows: Vec<(String, Option<f64>, Option<f64>, usize)> = Vec::new();
    let mut i = 0usize;
    while i < rows.len() {
        let st = rows[i].st;
        let mi = rows[i].mi;
        let mut j = i;
        while j < rows.len() && rows[j].st == st && rows[j].mi == mi {
            j += 1;
        }
        let blk = &rows[i..j];
        i = j;
        if blk.len() < MIN_BLK_DAYS {
            continue;
        }
        let Some(lab) = label_of(blk[0].day) else {
            continue;
        };
        let nd = blk.len();
        let lo = blk[0].day;
        let hi = blk[nd - 1].day;
        let df: f64 = blk.iter().map(|r| r.domfrac).sum::<f64>() / nd as f64;
        let mut mode_days: BTreeMap<i64, usize> = BTreeMap::new();
        let mut g_arms: BTreeMap<i64, Vec<f64>> = BTreeMap::new();
        let mut g_med: BTreeMap<i64, Vec<f64>> = BTreeMap::new();
        for r in blk {
            *mode_days.entry(r.mode).or_insert(0usize) += 1;
            g_arms.entry(r.mode).or_insert_with(Vec::new).push(r.arms_l);
            g_med.entry(r.mode).or_insert_with(Vec::new).push(r.med_l);
        }
        println!(
            "blk st{st} {lab} d{lo}-{hi} | {nd} days | domfrac {df:.2} | mode-day {:?}",
            mode_days
        );
        let mut mstr: Vec<String> = Vec::new();
        for m in 1..=3 {
            if let Some(v) = g_arms.get(&m) {
                let a = median_f64_of(v)
                    .map(|x| format!("{x:.2}"))
                    .unwrap_or_else(|| "-".to_string());
                let d = median_f64_of(&g_med[&m])
                    .map(|x| format!("{x:.2}"))
                    .unwrap_or_else(|| "-".to_string());
                mstr.push(format!("m{m} n{} arms {a} med {d}", v.len()));
            }
        }
        println!("  level {}", mstr.join(" | "));
        let cnon = blk.iter().filter(|r| r.frac_non1 > 0.0).count();
        if cnon > 0 {
            let mxnon = blk.iter().map(|r| r.frac_non1).fold(0.0f64, f64::max);
            println!("  cadence: {cnon}/{nd} days carry non-1-s samples (max frac {mxnon:.4})");
        } else {
            println!("  cadence: all days at 1 s");
        }
        for (a, b) in [(1i64, 2i64), (1, 3), (2, 3)] {
            let (Some(ga), Some(gb)) = (g_arms.get(&a), g_arms.get(&b)) else {
                continue;
            };
            if ga.len() < MIN_MODE_DAYS || gb.len() < MIN_MODE_DAYS {
                println!(
                    "  d{a}v{b}: n{a} {} n{b} {} below floor (< {MIN_MODE_DAYS} days)",
                    ga.len(),
                    gb.len()
                );
                continue;
            }
            let ma = &g_med[&a];
            let mb = &g_med[&b];
            let da = sub_med(ga, gb)
                .map(|x| format!("{x:.2}"))
                .unwrap_or_else(|| "-".to_string());
            let dm = sub_med(ma, mb)
                .map(|x| format!("{x:.2}"))
                .unwrap_or_else(|| "-".to_string());
            println!(
                "  d{a}v{b} n{a}={} n{b}={} armsDek {da} medDek {dm}",
                ga.len(),
                gb.len()
            );
            let tup = (
                format!("st{st} {lab}"),
                sub_med(ga, gb).expect("both nonempty"),
                sub_med(ma, mb).expect("both nonempty"),
                ga.len(),
                gb.len(),
            );
            match (a, b) {
                (1, 2) => d12_rows.push(tup),
                (1, 3) => d13_rows.push(tup),
                _ => d23_rows.push(tup),
            }
        }
        let refv: Vec<f64> = blk.iter().map(|r| r.ref_med).collect();
        let lv = n_distinct_rounded(&refv, 1.0);
        if lv >= MIN_REF_LEVELS {
            let av: Vec<f64> = blk.iter().map(|r| r.arms_l).collect();
            let mv: Vec<f64> = blk.iter().map(|r| r.med_l).collect();
            let sa = spearman(&refv, &av);
            let sm = spearman(&refv, &mv);
            let sa_s = sa
                .map(|x| format!("{x:.2}"))
                .unwrap_or_else(|| "-".to_string());
            let sm_s = sm
                .map(|x| format!("{x:.2}"))
                .unwrap_or_else(|| "-".to_string());
            println!(
                "  ref same-month: distinct levels {lv} | rho(arms) {sa_s} rho(med) {sm_s} n {nd}"
            );
            ref_rows.push((format!("st{st} {lab}"), sa, sm, nd));
        } else {
            println!(
                "  ref same-month: distinct levels {lv} n {nd} below floor ({MIN_REF_LEVELS}+ levels)"
            );
        }
        if let Some(wl) = within_map.get(&(st, mi)) {
            let m12: Vec<&Within> = wl
                .iter()
                .copied()
                .filter(|w| w.amode == 1 && w.bmode == 2)
                .collect();
            if !m12.is_empty() {
                let dmv: Vec<f64> = m12.iter().map(|w| w.del_med).collect();
                let dav: Vec<f64> = m12.iter().map(|w| w.del_arms).collect();
                let posm = dmv.iter().filter(|&&x| x > 0.0).count();
                let negm = dmv.len() - posm;
                let pm = binom_two_tail(dmv.len(), posm.min(negm))
                    .map(|x| format!("{x:.2}"))
                    .unwrap_or_else(|| "-".to_string());
                let md = median_f64_of(&dmv)
                    .map(|x| format!("{x:.2}"))
                    .unwrap_or_else(|| "-".to_string());
                let md_arms = median_f64_of(&dav)
                    .map(|x| format!("{x:.2}"))
                    .unwrap_or_else(|| "-".to_string());
                println!(
                    "  within-day m1v2: n {} days | medDek(med) {md} medDek(arms) {md_arms} | +{posm} -{negm} | p {pm}",
                    m12.len()
                );
            }
        }
    }
    println!("\n## d1v2 same-day level difference per (station, month) — era + station controlled");
    for (lab, da, dm, na, nb) in &d12_rows {
        println!("  d1v2 {lab}: n1 {na} n2 {nb} armsDek {da:.2} medDek {dm:.2}");
    }
    let pa = d12_rows.iter().filter(|(_, da, _, _, _)| *da > 0.0).count();
    let na = d12_rows.len() - pa;
    let pm = d12_rows.iter().filter(|(_, _, dm, _, _)| *dm > 0.0).count();
    let nm = d12_rows.len() - pm;
    println!(
        "d1v2 sign over blocks: arms +{pa}/-{na}, med +{pm}/-{nm} (n blocks {})",
        d12_rows.len()
    );
    for (lab, da, dm, na, nb) in &d13_rows {
        println!("  d1v3 {lab}: n1 {na} n3 {nb} armsDek {da:.2} medDek {dm:.2}");
    }
    for (lab, da, dm, na, nb) in &d23_rows {
        println!("  d2v3 {lab}: n2 {na} n3 {nb} armsDek {da:.2} medDek {dm:.2}");
    }
    println!("\n## ref same-month level association — rho(day med ref, day noise) per (station, month)");
    let mut rp = 0usize;
    let mut rn = 0usize;
    let mut rm_p = 0usize;
    let mut rm_n = 0usize;
    for (lab, a, m, nd) in &ref_rows {
        let sa = a
            .map(|x| format!("{x:.2}"))
            .unwrap_or_else(|| "-".to_string());
        let sm = m
            .map(|x| format!("{x:.2}"))
            .unwrap_or_else(|| "-".to_string());
        if let Some(x) = a {
            if *x > 0.0 {
                rp += 1;
            } else {
                rn += 1;
            }
        }
        if let Some(x) = m {
            if *x > 0.0 {
                rm_p += 1;
            } else {
                rm_n += 1;
            }
        }
        println!("  ref {lab}: rho(arms) {sa} rho(med) {sm} n {nd}");
    }
    println!(
        "ref rho sign over blocks: arms +{rp}/-{rn}, med +{rm_p}/-{rm_n} (n blocks {})",
        ref_rows.len()
    );
    let m12all: Vec<&Within> = within
        .iter()
        .filter(|w| w.amode == 1 && w.bmode == 2)
        .collect();
    if !m12all.is_empty() {
        let dmv: Vec<f64> = m12all.iter().map(|w| w.del_med).collect();
        let dav: Vec<f64> = m12all.iter().map(|w| w.del_arms).collect();
        let posm = dmv.iter().filter(|&&x| x > 0.0).count();
        let negm = dmv.len() - posm;
        let pm = binom_two_tail(dmv.len(), posm.min(negm))
            .map(|x| format!("{x:.3}"))
            .unwrap_or_else(|| "-".to_string());
        let posa = dav.iter().filter(|&&x| x > 0.0).count();
        let nega = dav.len() - posa;
        let pm_a = binom_two_tail(dav.len(), posa.min(nega))
            .map(|x| format!("{x:.3}"))
            .unwrap_or_else(|| "-".to_string());
        let md = median_f64_of(&dmv)
            .map(|x| format!("{x:.2}"))
            .unwrap_or_else(|| "-".to_string());
        let md_arms = median_f64_of(&dav)
            .map(|x| format!("{x:.2}"))
            .unwrap_or_else(|| "-".to_string());
        let nblk = m12all
            .iter()
            .map(|w| (w.st, w.mi))
            .collect::<BTreeSet<_>>()
            .len();
        println!("\n## within-day m1v2 pooled over all qualifying days (era + station cancelled per day)");
        println!(
            "  n days {n} from {nblk} station-month blocks | medDek(med) {md} | medDek(arms) {md_arms} | sign med +{posm}/-{negm} p {pm} | sign arms +{posa}/-{nega} p {pm_a}",
            n = m12all.len()
        );
    } else {
        println!("\n## within-day m1v2 pooled: no qualifying days");
    }
}
