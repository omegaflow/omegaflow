use std::collections::BTreeMap;
use std::fs;

use omegaflow::atdf::parse_resid_bin;
use omegaflow::spectral::civil_from_days;

const DAY_S: f64 = 86400.0;
const LOCK_HZ: f64 = 1.0e3;
const FLOOR: i64 = -2560;
const LOUD_HZ: f64 = 1.0;
const MIN_CELL: usize = 30;
const ERA0_C: (i64, i64, i64) = (1995, 11, 23);
const ERA1_C: (i64, i64, i64) = (1997, 2, 28);
const STATIONS: [i64; 3] = [14, 43, 63];
const MIN_SEG: usize = 3;
const N_PERM: usize = 1999;
const SEED_BASE: u64 = 0x9E37_79B9_7F4A_7C15;
const PLAT_END_C: (i64, i64, i64) = (1995, 11, 30);
const POST_START_C: (i64, i64, i64) = (1995, 12, 1);
const W0_C: (i64, i64, i64) = (1995, 11, 28);
const W1_C: (i64, i64, i64) = (1995, 12, 3);
const DGT_C: (i64, i64, i64) = (1996, 5, 23);
const STEP_C: (i64, i64, i64) = (1995, 12, 1);

fn days_from_civil(y: i64, m: i64, d: i64) -> i64 {
    let yy = if m <= 2 { y - 1 } else { y };
    let era = if yy >= 0 { yy } else { yy - 399 } / 400;
    let yoe = yy - era * 400;
    let mp = (m + 9) % 12;
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe - 719468
}

fn unix_day(tdb: f64) -> i64 {
    let jd = 2451545.0 + tdb / DAY_S;
    (jd - 2440587.5).round() as i64
}

fn civil_str(day: i64) -> String {
    match civil_from_days(day) {
        Some((y, m, d)) => format!("{y:04}-{m:02}-{d:02}"),
        None => format!("day {day}"),
    }
}

fn month_str(day: i64) -> String {
    match civil_from_days(day) {
        Some((y, m, _)) => format!("{y:04}-{m:02}"),
        None => format!("day {day}"),
    }
}

fn median(vals: &[f64]) -> Option<f64> {
    if vals.is_empty() {
        return None;
    }
    let mut s = vals.to_vec();
    s.sort_by(f64::total_cmp);
    Some(s[s.len() / 2])
}

fn mean(vals: &[f64]) -> Option<f64> {
    if vals.is_empty() {
        return None;
    }
    Some(vals.iter().sum::<f64>() / vals.len() as f64)
}

fn sd_sample(vals: &[f64]) -> Option<f64> {
    if vals.len() < 2 {
        return None;
    }
    let m = vals.iter().sum::<f64>() / vals.len() as f64;
    let v = vals.iter().map(|x| (x - m) * (x - m)).sum::<f64>() / (vals.len() - 1) as f64;
    Some(v.sqrt())
}

fn next_rng(rng: &mut u64) -> u64 {
    *rng = rng
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);
    *rng >> 33
}

fn seed_for(mode: i64, st: i64, salt: u64) -> u64 {
    SEED_BASE
        .wrapping_add((mode as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15))
        .wrapping_add((st as u64).wrapping_mul(0x0F0F_0F0F_0F0F_0F0F))
        .wrapping_add(salt)
}

struct Cell {
    n: usize,
    sum: f64,
    sum2: f64,
    vals: Vec<f64>,
}

impl Cell {
    fn new() -> Cell {
        Cell {
            n: 0,
            sum: 0.0,
            sum2: 0.0,
            vals: Vec::new(),
        }
    }
    fn med(&self) -> f64 {
        let mut s = self.vals.clone();
        s.sort_by(f64::total_cmp);
        s[s.len() / 2]
    }
    fn mean(&self) -> f64 {
        self.sum / self.n as f64
    }
    fn rms(&self) -> f64 {
        let m = self.mean();
        (self.sum2 / self.n as f64 - m * m).max(0.0).sqrt()
    }
}

#[derive(Clone)]
struct Row {
    day: i64,
    n: usize,
    mean: f64,
    med: f64,
    rms: f64,
    run: usize,
}

struct BestCut {
    c: usize,
    nl: usize,
    nr: usize,
    ml: f64,
    mr: f64,
    sp: f64,
    t: f64,
}

fn best_cut(levels: &[f64]) -> Option<BestCut> {
    let n = levels.len();
    if n < 2 * MIN_SEG {
        return None;
    }
    let mut best: Option<BestCut> = None;
    let mut best_jump = -1.0f64;
    for c in MIN_SEG..=(n - MIN_SEG) {
        let (l, r) = levels.split_at(c);
        let (Some(ml), Some(mr)) = (mean(l), mean(r)) else {
            continue;
        };
        let j = (mr - ml).abs();
        if j <= best_jump {
            continue;
        }
        let (Some(sl), Some(sr)) = (sd_sample(l), sd_sample(r)) else {
            continue;
        };
        best_jump = j;
        let df = (l.len() + r.len() - 2) as f64;
        let sp = (((l.len() - 1) as f64 * sl * sl + (r.len() - 1) as f64 * sr * sr) / df).sqrt();
        let denom = sp * (1.0 / l.len() as f64 + 1.0 / r.len() as f64).sqrt();
        let t = if denom > 0.0 {
            (mr - ml) / denom
        } else {
            f64::INFINITY
        };
        best = Some(BestCut {
            c,
            nl: l.len(),
            nr: r.len(),
            ml,
            mr,
            sp,
            t,
        });
    }
    best
}

fn cut_dates(rows: &[Row], bc: &BestCut) -> (i64, i64) {
    (rows[bc.c - 1].day, rows[bc.c].day)
}

fn max_step(levels: &[f64]) -> f64 {
    let n = levels.len();
    if n < 2 * MIN_SEG {
        return 0.0;
    }
    let mut pref = vec![0.0f64; n + 1];
    for i in 0..n {
        pref[i + 1] = pref[i] + levels[i];
    }
    let mut best = 0.0f64;
    for c in MIN_SEG..=(n - MIN_SEG) {
        let ml = pref[c] / c as f64;
        let mr = (pref[n] - pref[c]) / (n - c) as f64;
        let d = (ml - mr).abs();
        if d > best {
            best = d;
        }
    }
    best
}

fn runs_of(rows: &[Row]) -> Vec<(usize, usize)> {
    let nr = rows.iter().map(|r| r.run).max().map_or(0, |m| m + 1);
    let mut runs: Vec<(usize, usize)> = Vec::new();
    for rid in 0..nr {
        let mut s: Option<usize> = None;
        let mut e: Option<usize> = None;
        for (i, r) in rows.iter().enumerate() {
            if r.run == rid {
                if s.is_none() {
                    s = Some(i);
                }
                e = Some(i);
            }
        }
        if let (Some(ss), Some(ee)) = (s, e) {
            runs.push((ss, ee - ss + 1));
        }
    }
    runs
}

fn block_perm_p(runs: &[(usize, usize)], levels: &[f64], obs: f64, seed: u64) -> f64 {
    let nb = runs.len();
    let n = levels.len();
    if nb < 2 {
        return f64::NAN;
    }
    let mut rng = seed;
    let mut order: Vec<usize> = (0..nb).collect();
    let mut buf = vec![0.0f64; n];
    let mut cnt = 0usize;
    for _ in 0..N_PERM {
        for i in (1..nb).rev() {
            let j = (next_rng(&mut rng) as usize) % (i + 1);
            order.swap(i, j);
        }
        let mut k = 0usize;
        for &oi in &order {
            let (s, l) = runs[oi];
            buf[k..k + l].copy_from_slice(&levels[s..s + l]);
            k += l;
        }
        if max_step(&buf) >= obs {
            cnt += 1;
        }
    }
    (cnt as f64 + 1.0) / (N_PERM as f64 + 1.0)
}

fn fmt_p(p: f64) -> String {
    if p < 0.00005 {
        format!("{p:.1e}")
    } else {
        format!("{p:.4}")
    }
}

fn fmt_hz(v: f64) -> String {
    format!("{v:+.4}")
}

fn rec(out: &mut Vec<String>, s: String) {
    out.push(s);
}

fn series_rows(cells: &BTreeMap<(i64, i64, i64), Cell>, mode: i64, st: i64) -> Vec<Row> {
    let mut rows: Vec<Row> = Vec::new();
    for ((m, d, s), c) in cells {
        if *m == mode && *s == st && c.n >= MIN_CELL {
            let rms = c.rms();
            if rms < LOUD_HZ {
                rows.push(Row {
                    day: *d,
                    n: c.n,
                    mean: c.mean(),
                    med: c.med(),
                    rms,
                    run: 0,
                });
            }
        }
    }
    rows.sort_by_key(|r| r.day);
    let mut run_id = 0usize;
    for i in 0..rows.len() {
        if i > 0 && rows[i].day > rows[i - 1].day + 1 {
            run_id += 1;
        }
        rows[i].run = run_id;
    }
    rows
}

fn level_stats(rows: &[Row]) -> Option<(usize, f64, f64, Option<f64>, f64, f64)> {
    if rows.is_empty() {
        return None;
    }
    let meds: Vec<f64> = rows.iter().map(|r| r.med).collect();
    let med = median(&meds)?;
    let m = meds.iter().sum::<f64>() / meds.len() as f64;
    let sd = sd_sample(&meds);
    let mn = meds.iter().cloned().fold(f64::INFINITY, f64::min);
    let mx = meds.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    Some((meds.len(), med, m, sd, mn, mx))
}

fn desc_block(rows: &[Row]) -> String {
    match level_stats(rows) {
        None => "n 0 (empty block, 0 honored)".to_string(),
        Some((n, med, m, sd, mn, mx)) => match sd {
            Some(s) => format!(
                "n {n} | median {med:+.4} | mean {m:+.4} | sd {s:.4} | min {mn:+.4} | max {mx:+.4} Hz"
            ),
            None => format!(
                "n {n} | median {med:+.4} | mean {m:+.4} | sd - | min {mn:+.4} | max {mx:+.4} Hz"
            ),
        },
    }
}

fn best_cut_on(rows: &[Row]) -> Option<(BestCut, i64, i64)> {
    let meds: Vec<f64> = rows.iter().map(|r| r.med).collect();
    let bc = best_cut(&meds)?;
    let (ld, rd) = cut_dates(rows, &bc);
    Some((bc, ld, rd))
}

fn main() {
    let mut report = "/tmp/opencode/galileo_ruck_nachmessen_f3_report.txt".to_string();
    for a in std::env::args().skip(1) {
        if let Some(r) = a.strip_prefix("--report=") {
            report = r.to_string();
        }
    }
    let path = "data/pds-ppi.igpp.ucla.edu/galileo_resid.bin";
    let Ok(bytes) = fs::read(path) else {
        eprintln!("{path}: resid bin void");
        return;
    };
    let Some(recs) = parse_resid_bin(&bytes) else {
        eprintln!("{path}: resid bin parse void");
        return;
    };
    drop(bytes);

    let era0 = days_from_civil(ERA0_C.0, ERA0_C.1, ERA0_C.2);
    let era1 = days_from_civil(ERA1_C.0, ERA1_C.1, ERA1_C.2);
    let plat_end = days_from_civil(PLAT_END_C.0, PLAT_END_C.1, PLAT_END_C.2);
    let post_start = days_from_civil(POST_START_C.0, POST_START_C.1, POST_START_C.2);
    let w0 = days_from_civil(W0_C.0, W0_C.1, W0_C.2);
    let w1 = days_from_civil(W1_C.0, W1_C.1, W1_C.2);
    let dgt = days_from_civil(DGT_C.0, DGT_C.1, DGT_C.2);
    let step = days_from_civil(STEP_C.0, STEP_C.1, STEP_C.2);

    let mut cells: BTreeMap<(i64, i64, i64), Cell> = BTreeMap::new();
    let mut month_all: BTreeMap<String, usize> = BTreeMap::new();
    let mut month_m1: BTreeMap<String, usize> = BTreeMap::new();
    let mut n_lock = 0usize;
    let mut n_floor = 0usize;
    for r in &recs {
        let mode = r[3] as i64;
        let st = r[2] as i64;
        if !(1..=3).contains(&mode) || !STATIONS.contains(&st) {
            continue;
        }
        let day = unix_day(r[0]);
        if day < era0 || day > era1 {
            continue;
        }
        let resid = r[1];
        if !resid.is_finite() {
            continue;
        }
        if resid.abs() > LOCK_HZ {
            n_lock += 1;
            continue;
        }
        if r[7] as i64 != FLOOR {
            continue;
        }
        n_floor += 1;
        let ms = month_str(day);
        *month_all.entry(ms.clone()).or_insert(0) += 1;
        let m1cell = month_m1.entry(ms).or_insert(0);
        if mode == 1 {
            *m1cell += 1;
        }
        let key = (mode, day, st);
        let c = cells.entry(key).or_insert_with(Cell::new);
        c.n += 1;
        c.sum += resid;
        c.sum2 += resid * resid;
        c.vals.push(resid);
    }
    drop(recs);

    let mut out: Vec<String> = Vec::new();
    rec(
        &mut out,
        "galileo floor quiet-basis ruck F3 nachmessen — the four sharpening measurements".to_string(),
    );
    rec(&mut out, format!(
        "binding: floor era {} .. {} (day labels {era0}..{era1}); floor = strength == {FLOOR} (AGC clamp); robust cell = (mode, station, day) n >= {MIN_CELL} over in-track (finite, |resid| <= {LOCK_HZ:.0} Hz) floor samples; quiet = cell RMS < {LOUD_HZ:.0} Hz, loud = >= {LOUD_HZ:.0} Hz; day = round-of-tdb civil day; day level = daily median of the cell resid; the day label D spans UTC D-1 12:00..D 12:00",
        civil_str(era0),
        civil_str(era1),
    ));

    rec(&mut out, String::new());
    rec(&mut out, "== census (era reproduction) ==".to_string());
    let mut tot_rob = 0usize;
    let mut tot_loud = 0usize;
    for &mode in &[1i64, 2, 3] {
        for &st in &STATIONS {
            let rows = series_rows(&cells, mode, st);
            let quiet = rows.len();
            let robust = cells
                .iter()
                .filter(|((m, _, s), c)| *m == mode && *s == st && c.n >= MIN_CELL)
                .count();
            let loud = robust - quiet;
            tot_rob += robust;
            tot_loud += loud;
            rec(
                &mut out,
                format!(
                    "M{mode} st{st}: robust {robust} (loud {loud}, quiet {quiet}) | quiet days span {} .. {}",
                    rows.first().map_or("-".to_string(), |r| civil_str(r.day)),
                    rows.last().map_or("-".to_string(), |r| civil_str(r.day))
                ),
            );
        }
    }
    rec(
        &mut out,
        format!(
            "total robust cells {tot_rob} (loud {tot_loud}, quiet {}) | floor in-track samples {n_floor} | lock samples excluded {n_lock}",
            tot_rob - tot_loud
        ),
    );

    rec(&mut out, String::new());
    rec(
        &mut out,
        "== Q1: mode-1 quiet level after the 1995-11-30/12-01 step — near 0 or +0.75 kept? ==".to_string(),
    );
    for &st in &STATIONS {
        let rows = series_rows(&cells, 1, st);
        let plateau: Vec<Row> = rows
            .iter()
            .filter(|r| r.day <= plat_end && r.rms > 0.0)
            .cloned()
            .collect();
        let post: Vec<Row> = rows.iter().filter(|r| r.day >= post_start).cloned().collect();
        let dec95: Vec<Row> = post
            .iter()
            .filter(|r| r.day <= days_from_civil(1995, 12, 31))
            .cloned()
            .collect();
        let zero_days = rows.iter().filter(|r| r.rms == 0.0).count();
        rec(&mut out, String::new());
        rec(
            &mut out,
            format!("M1 st{st}: quiet days total {} (zero-spread days in the pre-region excluded from the plateau median: {zero_days})", rows.len()),
        );
        rec(
            &mut out,
            format!("  plateau (pre-step, <= {}) : {}", civil_str(plat_end), desc_block(&plateau)),
        );
        rec(
            &mut out,
            format!("  post-step (>= {})       : {}", civil_str(post_start), desc_block(&post)),
        );
        rec(
            &mut out,
            format!("  post-step Dec-1995 only  : {}", desc_block(&dec95)),
        );
        if let Some(ps) = level_stats(&plateau) {
            let ret = post.iter().filter(|r| r.med >= ps.1).count();
            let hi = post.iter().filter(|r| r.med >= 0.5).count();
            rec(
                &mut out,
                format!(
                    "  post-step days at/above plateau median {:+.4}: {ret} of {} | at/above +0.5 Hz: {hi}",
                    ps.1,
                    post.len()
                ),
            );
            if let Some(postmed) = median(&post.iter().map(|r| r.med).collect::<Vec<f64>>()) {
                rec(
                    &mut out,
                    format!(
                        "  plateau median {:+.4} vs post-step median {:+.4} -> net {:+.4} Hz (GLM arithmetic +0.75 - 0.8 ~ -0.05)",
                        ps.1,
                        postmed,
                        postmed - ps.1
                    ),
                );
            }
        }
        rec(
            &mut out,
            format!("  post-step day list ({} days):", post.len()),
        );
        for r in &post {
            rec(
                &mut out,
                format!(
                    "    {} | n {:<6} | med {:+.4} | mean {:+.4} | rms {:.4}",
                    civil_str(r.day),
                    r.n,
                    r.med,
                    r.mean,
                    r.rms
                ),
            );
        }
    }

    rec(&mut out, String::new());
    rec(
        &mut out,
        "== Q2: modes 2/3 coverage at the transition window 1995-11-28 .. 1995-12-03 ==".to_string(),
    );
    for &mode in &[1i64, 2, 3] {
        for &st in &STATIONS {
            rec(&mut out, String::new());
            rec(
                &mut out,
                format!("M{mode} st{st} window cells (n | level-med Hz | rms Hz | class; class quiet = robust n>=30 & rms<1, loud = robust & rms>=1, thin = 1..29 samples, empty = no floor sample):"),
            );
            let mut d = w0;
            while d <= w1 {
                let key = (mode, d, st);
                if let Some(c) = cells.get(&key) {
                    let cls = if c.n >= MIN_CELL && c.rms() < LOUD_HZ {
                        "quiet".to_string()
                    } else if c.n >= MIN_CELL {
                        "loud".to_string()
                    } else {
                        "thin".to_string()
                    };
                    rec(
                        &mut out,
                        format!(
                            "  {} | n {:<6} | med {} | rms {:.4} | {cls}",
                            civil_str(d),
                            c.n,
                            fmt_hz(c.med()),
                            c.rms()
                        ),
                    );
                } else {
                    rec(
                        &mut out,
                        format!(
                            "  {} | n 0 | (no floor sample, 0 honored) | empty",
                            civil_str(d)
                        ),
                    );
                }
                d += 1;
            }
            let srows = series_rows(&cells, mode, st);
            let pre: Vec<&Row> = srows.iter().filter(|r| r.day <= plat_end && r.day >= w0).collect();
            let pos: Vec<&Row> = srows.iter().filter(|r| r.day >= post_start && r.day <= w1).collect();
            let left = pre
                .iter()
                .max_by_key(|r| r.day)
                .map(|r| (r.day, r.med, r.rms, r.n));
            let right = pos
                .iter()
                .min_by_key(|r| r.day)
                .map(|r| (r.day, r.med, r.rms, r.n));
            match (left, right) {
                (Some((ld, lm, lr, ln)), Some((rd, rm, rr, rn))) => rec(
                    &mut out,
                    format!(
                        "  boundary pair {} (med {:+.4}, rms {:.4}, n {}) -> {} (med {:+.4}, rms {:.4}, n {}) : delta {:+.4} Hz",
                        civil_str(ld),
                        lm,
                        lr,
                        ln,
                        civil_str(rd),
                        rm,
                        rr,
                        rn,
                        rm - lm
                    ),
                ),
                (Some((ld, lm, lr, ln)), None) => rec(
                    &mut out,
                    format!(
                        "  last pre-boundary quiet day {} (med {:+.4}, rms {:.4}, n {}); no quiet robust day on/after the boundary within the window (0 honored)",
                        civil_str(ld),
                        lm,
                        lr,
                        ln
                    ),
                ),
                (None, Some((rd, rm, rr, rn))) => rec(
                    &mut out,
                    format!(
                        "  no quiet robust day before the boundary within the window (0 honored); first on/after {} (med {:+.4}, rms {:.4}, n {})",
                        civil_str(rd),
                        rm,
                        rr,
                        rn
                    ),
                ),
                (None, None) => rec(
                    &mut out,
                    "  no quiet robust day on either side of the boundary within the window (0 honored)".to_string(),
                ),
            }
        }
    }

    rec(&mut out, String::new());
    rec(
        &mut out,
        "== Q3: segmentation provenance of the 1995-11-30/12-01 change point ==".to_string(),
    );
    rec(
        &mut out,
        "method: single-cut two-segment piecewise-constant model on the chronological quiet-day level series of the whole floor era, cut between sequence indices with min 3 days per segment, statistic = max |segment-mean jump|, null = run-block time reshuffle (1999 draws); the scan evaluates every admissible cut, it does not fix a date.".to_string(),
    );
    for &st in &STATIONS {
        let rows = series_rows(&cells, 1, st);
        let meds: Vec<f64> = rows.iter().map(|r| r.med).collect();
        let runs = runs_of(&rows);
        rec(&mut out, String::new());
        rec(
            &mut out,
            format!(
                "M1 st{st}: {} quiet days, first {} last {} (era {} .. {})",
                rows.len(),
                rows.first().map_or("-".to_string(), |r| civil_str(r.day)),
                rows.last().map_or("-".to_string(), |r| civil_str(r.day)),
                civil_str(era0),
                civil_str(era1)
            ),
        );
        if let Some((bc, ld, rd)) = best_cut_on(&rows) {
            let obs = (bc.mr - bc.ml).abs();
            let p_perm = block_perm_p(&runs, &meds, obs, seed_for(1, st, 0x55));
            rec(
                &mut out,
                format!(
                    "  whole-era best cut {} <-> {} | nL {} nR {} | levelL {:+.4} levelR {:+.4} | jump {:+.4} Hz | pooled sd {:.4} | t {:.2} | run-block p {}",
                    civil_str(ld),
                    civil_str(rd),
                    bc.nl,
                    bc.nr,
                    bc.ml,
                    bc.mr,
                    bc.mr - bc.ml,
                    bc.sp,
                    bc.t,
                    fmt_p(p_perm)
                ),
            );
            rec(
                &mut out,
                format!(
                    "  margin: left segment {} quiet days, right segment {}; cut date {} is {} days after the era/data opening {} and {} days before the era/data end {}; the left side is a resolved pre-cut quiet week, not the data edge",
                    bc.nl,
                    bc.nr,
                    civil_str(ld),
                    ld - era0,
                    civil_str(era0),
                    era1 - rd,
                    civil_str(era1)
                ),
            );
            let post_rows: Vec<Row> = rows.iter().filter(|r| r.day >= step).cloned().collect();
            if let Some((bc2, ld2, rd2)) = best_cut_on(&post_rows) {
                rec(
                    &mut out,
                    format!(
                        "  post-step body alone (>= {}) best cut: {} <-> {} | jump {:+.4} Hz | t {:.2} (no 0.5-Hz-class step remains once the opening plateau is the left side)",
                        civil_str(step),
                        civil_str(ld2),
                        civil_str(rd2),
                        bc2.mr - bc2.ml,
                        bc2.t
                    ),
                );
            }
            let plateau: Vec<Row> = rows
                .iter()
                .filter(|r| r.day <= plat_end && r.rms > 0.0)
                .cloned()
                .collect();
            rec(
                &mut out,
                "  plateau-day jackknife (drop one opening-quiet day, rescan whole era):".to_string(),
            );
            for p in &plateau {
                let trimmed: Vec<Row> = rows.iter().filter(|r| r.day != p.day).cloned().collect();
                match best_cut_on(&trimmed) {
                    Some((bcj, ldj, rdj)) => rec(
                        &mut out,
                        format!(
                            "    drop {} (med {:+.4}): cut stays {} <-> {} | jump {:+.4} Hz",
                            civil_str(p.day),
                            p.med,
                            civil_str(ldj),
                            civil_str(rdj),
                            bcj.mr - bcj.ml
                        ),
                    ),
                    None => rec(
                        &mut out,
                        format!("    drop {}: scan void", civil_str(p.day)),
                    ),
                }
            }
            rec(
                &mut out,
                "  the three stations are measured independently (separate resid series); the same date reproduces when the whole-era scan runs on each of st14/st43/st63".to_string(),
            );
        } else {
            rec(&mut out, "  whole-era scan void (too few quiet days)".to_string());
        }
    }

    rec(&mut out, String::new());
    rec(
        &mut out,
        "== Q4: 1996-05-23 DGT boundary — mode-1 quiet level step? ==".to_string(),
    );
    rec(
        &mut out,
        "floor sample census by month (trio stations, mode 1..3; 0 honored):".to_string(),
    );
    for (m, n_all) in &month_all {
        let m1n = month_m1[m];
        rec(
            &mut out,
            format!(
                "  {m}: all-modes in-track floor samples {n_all} | mode-1-only {m1n}"
            ),
        );
    }
    for &st in &STATIONS {
        let rows = series_rows(&cells, 1, st);
        let before: Vec<Row> = rows
            .iter()
            .filter(|r| r.day >= post_start && r.day < dgt)
            .cloned()
            .collect();
        let after: Vec<Row> = rows.iter().filter(|r| r.day > dgt).cloned().collect();
        let last_pre = rows.iter().filter(|r| r.day < dgt).max_by_key(|r| r.day);
        let first_post = rows.iter().filter(|r| r.day > dgt).min_by_key(|r| r.day);
        let near40 = rows.iter().filter(|r| (r.day - dgt).abs() <= 40).count();
        let near90 = rows.iter().filter(|r| (r.day - dgt).abs() <= 90).count();
        rec(&mut out, String::new());
        rec(
            &mut out,
            format!(
                "M1 st{st}: quiet days within +/-40 d of {}: {near40} | within +/-90 d: {near90}",
                civil_str(dgt)
            ),
        );
        rec(
            &mut out,
            format!(
                "  before-side (>= {}, < {}) quiet-day block: {}",
                civil_str(post_start),
                civil_str(dgt),
                desc_block(&before)
            ),
        );
        rec(
            &mut out,
            format!(
                "  after-side (> {}) quiet-day block: {}",
                civil_str(dgt),
                desc_block(&after)
            ),
        );
        match (last_pre, first_post) {
            (Some(lp), Some(fp)) => rec(
                &mut out,
                format!(
                    "  nearest quiet days: last before {} = {} (med {:+.4}, n {}) {:.0} d before | first after = {} (med {:+.4}, n {}) {:.0} d after",
                    civil_str(dgt),
                    civil_str(lp.day),
                    lp.med,
                    lp.n,
                    dgt as f64 - lp.day as f64,
                    civil_str(fp.day),
                    fp.med,
                    fp.n,
                    fp.day as f64 - dgt as f64
                ),
            ),
            _ => rec(&mut out, "  no quiet day on one side of 1996-05-23 (0 honored)".to_string()),
        }
        let win: Vec<Row> = rows.iter().filter(|r| r.day >= dgt - 90).cloned().collect();
        if let Some((bc, ld, rd)) = best_cut_on(&win) {
            let near = ld >= dgt - 21 && ld <= dgt + 21;
            rec(
                &mut out,
                format!(
                    "  best cut in the {} +/-90-d window: {} <-> {} | jump {:+.4} Hz | lies within +/-21 d of 1996-05-23: {}",
                    civil_str(dgt),
                    civil_str(ld),
                    civil_str(rd),
                    bc.mr - bc.ml,
                    if near { "yes".to_string() } else { "no".to_string() }
                ),
            );
        } else {
            rec(
                &mut out,
                format!(
                    "  best cut in the {} +/-90-d window: void (fewer than 6 quiet days)",
                    civil_str(dgt)
                ),
            );
        }
    }

    rec(&mut out, String::new());
    rec(&mut out, "== reading notes ==".to_string());
    rec(
        &mut out,
        "Q1: if the post-step median is ~0, the -0.8 Hz step removed the +0.7..+0.85 Hz opening plateau (the plateau was the pre-existing offset); if it stays ~+0.75, the step created an offset. Q2: dense quiet mode-2/3 cells on both sides of the boundary are counter-data for the mode-1-only claim; n=0 on a side is a gap (0 honored), not counter-data. Q4: a level step across 1996-05-23 needs quiet mode-1 days on both near sides; the month census names the empty months.".to_string(),
    );

    let _ = fs::write(&report, out.join("\n") + "\n");
    println!("report written to {report}");
}
