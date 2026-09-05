use std::collections::{BTreeMap, HashMap};

use omegaflow::archivar::lsk::days_from_civil;
use omegaflow::archivar::{body_barycenter_position, parse_ephemeris_binary, BodyEphemeris};
use omegaflow::spectral::civil_from_days;
use omegaflow::te::{
    conditional_te_stats, surrogate_stats_block, surrogate_stats_phase, transfer_entropy_conditional,
    transfer_entropy_lag,
};

const DAY_S: f64 = 86400.0;
const LOCK_HZ: f64 = 1.0e3;
const FLOOR_AGC: f64 = -2560.0;
const MIN_CELL: usize = 30;
const LOUD_HZ: f64 = 1.0;
const K_LEVEL: usize = 4;
const MIN_SIDE: usize = 2;
const MIN_RUN_SEG: usize = 8;
const MIN_RUN_TE: usize = 9;
const N_PERM: usize = 199;
const N_SURR: usize = 20;
const BLOCK: usize = 5;
const SEED: u64 = 0x9E37_79B9_7F4A_7C15;
const STATIONS: [i64; 3] = [14, 43, 63];
const MODES: [i64; 3] = [1, 2, 3];

fn unix_day(tdb: f64) -> i64 {
    (2451545.0 + tdb / DAY_S - 2440587.5).round() as i64
}

fn fmt_day(d: i64) -> String {
    match civil_from_days(d) {
        Some((y, m, dd)) => format!("{y:04}-{m:02}-{dd:02}"),
        None => format!("day{d}"),
    }
}

fn month_index(d: i64) -> Option<i64> {
    let (y, m, _) = civil_from_days(d)?;
    Some(y as i64 * 12 + m as i64)
}

fn median(v: &[f64]) -> Option<f64> {
    if v.is_empty() {
        return None;
    }
    let mut s = v.to_vec();
    s.sort_by(f64::total_cmp);
    Some(s[s.len() / 2])
}

fn fmt_opt(v: Option<f64>) -> String {
    match v {
        Some(x) if x.is_finite() => format!("{x:.4}"),
        _ => "-".to_string(),
    }
}

fn dot3(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
fn sub3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
fn norm3(a: [f64; 3]) -> f64 {
    dot3(a, a).sqrt()
}
fn elong_deg(probe: [f64; 3], earth: [f64; 3]) -> Option<f64> {
    let sun = sub3([0.0, 0.0, 0.0], earth);
    let prb = sub3(probe, earth);
    let ns = norm3(sun);
    let np = norm3(prb);
    if ns <= 0.0 || np <= 0.0 {
        return None;
    }
    Some((dot3(sun, prb) / (ns * np)).clamp(-1.0, 1.0).acos().to_degrees())
}

fn load_eph(name: &str, eph: &mut HashMap<String, BodyEphemeris>) -> bool {
    let p = format!("data/ephemeris_{name}.bin");
    std::fs::read(&p)
        .ok()
        .and_then(|d| parse_ephemeris_binary(&d))
        .map(|e| eph.insert(name.to_string(), e))
        .is_some()
}

#[derive(Clone, Copy)]
struct Acc {
    sum: f64,
    sum2: f64,
    tdb_sum: f64,
    n: u32,
}
fn acc() -> Acc {
    Acc {
        sum: 0.0,
        sum2: 0.0,
        tdb_sum: 0.0,
        n: 0,
    }
}

struct DayVal {
    day: i64,
    rms: f64,
    eps: Option<f64>,
}

struct Series {
    station: i64,
    mode: i64,
    vals: Vec<DayVal>,
    thin_days: usize,
    zero_spread_days: usize,
}

impl Series {
    fn day(&self, i: usize) -> i64 {
        self.vals[i].day
    }
    fn runs(&self) -> Vec<(usize, usize)> {
        let mut out: Vec<(usize, usize)> = Vec::new();
        if self.vals.is_empty() {
            return out;
        }
        let mut start = 0usize;
        for i in 1..self.vals.len() {
            if self.vals[i].day - self.vals[i - 1].day != 1 {
                out.push((start, i - 1));
                start = i;
            }
        }
        out.push((start, self.vals.len() - 1));
        out
    }
}

fn prefix_sums(y: &[f64]) -> Vec<f64> {
    let mut ps = vec![0.0; y.len() + 1];
    for (i, &v) in y.iter().enumerate() {
        ps[i + 1] = ps[i] + v;
    }
    ps
}

fn best_split(ps: &[f64], a: usize, b: usize) -> Option<(usize, f64)> {
    if b - a + 1 < 2 * MIN_SIDE {
        return None;
    }
    let mut best: Option<(usize, f64)> = None;
    for s in (a + MIN_SIDE - 1)..=(b - MIN_SIDE) {
        let nl = (s - a + 1) as f64;
        let nr = (b - s) as f64;
        let ml = (ps[s + 1] - ps[a]) / nl;
        let mr = (ps[b + 1] - ps[s + 1]) / nr;
        let g = (ml - mr).abs();
        if best.map_or(true, |(_, bg)| g > bg) {
            best = Some((s, g));
        }
    }
    best
}

fn shuffle(v: &mut [f64], rng: &mut u64) {
    for i in (1..v.len()).rev() {
        *rng = rng
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        let j = ((*rng >> 33) as usize) % (i + 1);
        v.swap(i, j);
    }
}

fn perm_max_gap(y: &[f64], a: usize, b: usize, rng: &mut u64) -> Option<f64> {
    let mut w: Vec<f64> = y[a..=b].to_vec();
    shuffle(&mut w, rng);
    let ps = prefix_sums(&w);
    best_split(&ps, 0, w.len() - 1).map(|(_, g)| g)
}

fn split_test(y: &[f64], a: usize, b: usize) -> Option<(usize, f64, f64)> {
    if b - a + 1 < 2 * MIN_SIDE {
        return None;
    }
    let ps = prefix_sums(y);
    let (s, gap) = best_split(&ps, a, b)?;
    let mut rng = SEED
        .wrapping_add((a as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15))
        .wrapping_add((b as u64).wrapping_mul(0x0F0F_0F0F_0F0F_0F0F));
    let mut cnt = 0usize;
    for _ in 0..N_PERM {
        if let Some(pg) = perm_max_gap(y, a, b, &mut rng) {
            if pg >= gap {
                cnt += 1;
            }
        }
    }
    let p = (cnt as f64 + 1.0) / (N_PERM as f64 + 1.0);
    Some((s, gap, p))
}

fn segment_run(y: &[f64], a: usize, b: usize, out: &mut Vec<(usize, usize)>) {
    let Some((s, _, p)) = split_test(y, a, b) else {
        out.push((a, b));
        return;
    };
    if p <= 0.05 {
        segment_run(y, a, s, out);
        segment_run(y, s + 1, b, out);
    } else {
        out.push((a, b));
    }
}

fn seg_level(y: &[f64], a: usize, b: usize) -> f64 {
    let m = (a..=b).map(|i| y[i]).sum::<f64>() / (b - a + 1) as f64;
    10f64.powf(m)
}

fn directed_row(
    src: &[f32],
    tgt: &[f32],
    era: &[f32],
    lag: usize,
) -> (Option<f64>, Option<f64>, Option<f64>, Option<f64>, Option<f64>) {
    let te = transfer_entropy_lag(tgt, src, lag);
    let ph = surrogate_stats_phase(tgt, src, lag, SEED).map(|t| t.2);
    let bl = surrogate_stats_block(tgt, src, lag, BLOCK, SEED).map(|t| t.2);
    let cte = transfer_entropy_conditional(tgt, src, era, lag);
    let cth = conditional_te_stats(tgt, src, era, lag, SEED, N_SURR).map(|t| t.2);
    (te, ph, bl, cte, cth)
}

fn row_marks(r: &(Option<f64>, Option<f64>, Option<f64>, Option<f64>, Option<f64>)) -> String {
    let (te, ph, bl, cte, cth) = r;
    let mut s = String::new();
    if let (Some(t), Some(h)) = (te, ph) {
        if t > h {
            s.push('P');
        }
    }
    if let (Some(t), Some(h)) = (te, bl) {
        if t > h {
            s.push('B');
        }
    }
    if let (Some(t), Some(h)) = (cte, cth) {
        if t > h {
            s.push('C');
        }
    }
    s
}

fn print_dir_row(src_name: &str, tgt_name: &str, src: &[f32], tgt: &[f32], era: &[f32]) {
    let n = src.len();
    for lag in 1usize..=3 {
        let r = directed_row(src, tgt, era, lag);
        let marks = row_marks(&r);
        println!(
            "        {src_name}->{tgt_name} n {n} lag {lag}  te {}  phNull {}  blNull {}  cteEra {}  cThr {}  {}",
            fmt_opt(r.0),
            fmt_opt(r.1),
            fmt_opt(r.2),
            fmt_opt(r.3),
            fmt_opt(r.4),
            marks
        );
    }
}

fn aligned(a: &Series, b: &Series) -> Option<(Vec<i64>, Vec<f32>, Vec<f32>)> {
    let mut days: Vec<i64> = Vec::new();
    let mut la: Vec<f32> = Vec::new();
    let mut lb: Vec<f32> = Vec::new();
    let (mut i, mut j) = (0usize, 0usize);
    while i < a.vals.len() && j < b.vals.len() {
        let da = a.vals[i].day;
        let db = b.vals[j].day;
        if da == db {
            days.push(da);
            la.push(a.vals[i].rms.log10() as f32);
            lb.push(b.vals[j].rms.log10() as f32);
            i += 1;
            j += 1;
        } else if da < db {
            i += 1;
        } else {
            j += 1;
        }
    }
    if days.is_empty() {
        return None;
    }
    Some((days, la, lb))
}

fn era_from_days(days: &[i64]) -> Option<Vec<f32>> {
    let mut out = Vec::with_capacity(days.len());
    for d in days {
        out.push(month_index(*d)? as f32);
    }
    Some(out)
}

fn main() {
    let mut eph: HashMap<String, BodyEphemeris> = HashMap::new();
    if !load_eph("galileo_daily", &mut eph) || !load_eph("earth", &mut eph) {
        println!("galileo_floor_stair_te: ephemeris binaries void");
        return;
    }
    let bytes = match std::fs::read("data/galileo_resid.bin") {
        Ok(b) => b,
        Err(_) => {
            println!("galileo_floor_stair_te: resid bin void");
            return;
        }
    };
    let Some(recs) = omegaflow::atdf::parse_resid_bin(&bytes) else {
        println!("galileo_floor_stair_te: resid parse void");
        return;
    };
    drop(bytes);

    let mut cells: BTreeMap<(i64, i64), BTreeMap<i64, Acc>> = BTreeMap::new();
    let mut n_lock = 0usize;
    let mut n_floor_70m = 0usize;
    for r in &recs {
        if r[1].abs() > LOCK_HZ {
            n_lock += 1;
            continue;
        }
        if r[7] != FLOOR_AGC {
            continue;
        }
        let st = r[2] as i64;
        let mo = r[3] as i64;
        if !STATIONS.contains(&st) || !MODES.contains(&mo) {
            continue;
        }
        n_floor_70m += 1;
        let d = unix_day(r[0]);
        let a = cells
            .entry((st, mo))
            .or_default()
            .entry(d)
            .or_insert_with(acc);
        a.sum += r[1];
        a.sum2 += r[1] * r[1];
        a.tdb_sum += r[0];
        a.n += 1;
    }
    drop(recs);
    println!(
        "resid: floor samples at 70m stations (st 14/43/63, mode 1..3, non-lock) {n_floor_70m}; lock samples excluded {n_lock}\n"
    );

    let mut series: Vec<Series> = Vec::new();
    for &station in &STATIONS {
        for &mode in &MODES {
            let Some(daymap) = cells.get(&(station, mode)) else {
                continue;
            };
            let mut vals: Vec<DayVal> = Vec::new();
            let mut thin = 0usize;
            let mut zero_spread = 0usize;
            for (day, c) in daymap {
                let n = c.n as usize;
                if n < MIN_CELL {
                    thin += 1;
                    continue;
                }
                let mean = c.sum / c.n as f64;
                let var = (c.sum2 / c.n as f64 - mean * mean).max(0.0);
                let rms = var.sqrt();
                if rms <= 0.0 {
                    zero_spread += 1;
                    continue;
                }
                let tmean = c.tdb_sum / c.n as f64;
                let eps = body_barycenter_position("galileo_daily", tmean, &eph)
                    .zip(body_barycenter_position("earth", tmean, &eph))
                    .and_then(|(p, e)| elong_deg(p, e));
                vals.push(DayVal { day: *day, rms, eps });
            }
            series.push(Series {
                station,
                mode,
                vals,
                thin_days: thin,
                zero_spread_days: zero_spread,
            });
        }
    }
    drop(eph);

    println!("## Runs per (station, mode) — floor day cells n >= {MIN_CELL} samples, consecutive TDB days\n");
    for s in &series {
        let runs = s.runs();
        let mut parts: Vec<String> = Vec::new();
        for (a, b) in &runs {
            parts.push(format!(
                "{}..{} ({} d)",
                fmt_day(s.day(*a)),
                fmt_day(s.day(*b)),
                b - a + 1
            ));
        }
        println!(
            "mode {} st{}: n_days {} ({} thin <{MIN_CELL} excluded, {} zero-spread excluded) | runs: {}",
            s.mode,
            s.station,
            s.vals.len(),
            s.thin_days,
            s.zero_spread_days,
            if parts.is_empty() {
                "none".to_string()
            } else {
                parts.join("; ")
            }
        );
    }

    println!();
    println!("############################################################");
    println!("## Measurement 1 — step vs spike structure of the daily RMS per (station, mode)");
    println!("############################################################\n");

    let mut summary_rows: Vec<String> = Vec::new();
    summary_rows.push(format!(
        "{:<22} {:>6} {:>6} {:>6} {:>7} {:>7} {:>7} {:>10}",
        "series", "days", "flips", "segs", "steps", "short", "loudRuns", "loudRuns>=K"
    ));

    for s in &series {
        println!("==== mode {} st{} — Tages-RMS (Hz), log10-Domaine", s.mode, s.station);
        let runs = s.runs();
        let loud = |i: usize| s.vals[i].rms >= LOUD_HZ;

        let mut flips = 0usize;
        let mut loud_runs: Vec<usize> = Vec::new();
        let mut quiet_runs: Vec<usize> = Vec::new();
        for &(a, b) in &runs {
            let mut cur = loud(a);
            let mut len = 1usize;
            for i in (a + 1)..=b {
                let st = loud(i);
                if st == cur {
                    len += 1;
                } else {
                    if cur {
                        loud_runs.push(len);
                    } else {
                        quiet_runs.push(len);
                    }
                    flips += 1;
                    cur = st;
                    len = 1;
                }
            }
            if cur {
                loud_runs.push(len);
            } else {
                quiet_runs.push(len);
            }
        }
        let (min_l, max_l) = loud_runs
            .iter()
            .fold((usize::MAX, 0usize), |(mn, mx), &v| (mn.min(v), mx.max(v)));
        let (min_q, max_q) = quiet_runs
            .iter()
            .fold((usize::MAX, 0usize), |(mn, mx), &v| (mn.min(v), mx.max(v)));
        let nl = loud_runs.len();
        let nq = quiet_runs.len();
        let loud_long = loud_runs.iter().filter(|&&v| v >= K_LEVEL).count();
        let loud_short = loud_runs.iter().filter(|&&v| v <= 3).count();
        let quiet_long = quiet_runs.iter().filter(|&&v| v >= K_LEVEL).count();
        println!(
            "  binary loud(>= {LOUD_HZ} Hz): flips {flips}; loud-runs n {nl} min {min_l} max {max_l} (>= {K_LEVEL} d: {loud_long}, <= 3 d: {loud_short}); quiet-runs n {nq} min {min_q} max {max_q} (>= {K_LEVEL} d: {quiet_long})"
        );

        let mut tot_seg = 0usize;
        let mut tot_steps = 0usize;
        let mut tot_short = 0usize;
        let mut levels: Vec<f64> = Vec::new();
        let mut levels_days: Vec<(f64, String)> = Vec::new();

        for &(a, b) in &runs {
            let nrun = b - a + 1;
            if nrun < MIN_RUN_SEG {
                println!(
                    "  run {}..{} ({} d) < {MIN_RUN_SEG} d: not segmented (named, data-thin)",
                    fmt_day(s.day(a)),
                    fmt_day(s.day(b)),
                    nrun
                );
                continue;
            }
            let y: Vec<f64> = (a..=b).map(|i| s.vals[i].rms.log10()).collect();
            let mut segs: Vec<(usize, usize)> = Vec::new();
            segment_run(&y, 0, nrun - 1, &mut segs);
            let dump: Vec<String> = (a..=b)
                .map(|i| format!("{}:{:.4}", fmt_day(s.day(i)).split_once('-').map_or(fmt_day(s.day(i)), |t| t.1.to_string()), s.vals[i].rms))
                .collect();
            println!(
                "  run {}..{} ({} d) | {} segments | days(HZ) {}",
                fmt_day(s.day(a)),
                fmt_day(s.day(b)),
                nrun,
                segs.len(),
                dump.join(" ")
            );
            for (i, (sa, sb)) in segs.iter().enumerate() {
                let len = sb - sa + 1;
                let lvl = seg_level(&y, *sa, *sb);
                let tag = if len >= K_LEVEL { "level" } else { "short" };
                println!(
                    "      [{}] {}..{} ({} d, {tag}) level {:>12.5} Hz",
                    i + 1,
                    fmt_day(s.day(a + sa)),
                    fmt_day(s.day(a + sb)),
                    len,
                    lvl
                );
                if len >= K_LEVEL {
                    levels.push(lvl);
                    levels_days.push((
                        lvl,
                        format!("{}..{}", fmt_day(s.day(a + sa)), fmt_day(s.day(a + sb))),
                    ));
                }
            }
            let mut steps = 0usize;
            for w in segs.windows(2) {
                let ll = w[0].1 - w[0].0 + 1;
                let rl = w[1].1 - w[1].0 + 1;
                if ll >= K_LEVEL && rl >= K_LEVEL {
                    steps += 1;
                }
            }
            let short = segs.iter().filter(|(sa, sb)| sb - sa + 1 < K_LEVEL).count();
            tot_seg += segs.len();
            tot_steps += steps;
            tot_short += short;
            if segs.len() == 1 {
                if let Some((sp, gap, p)) = split_test(&y, 0, nrun - 1) {
                    println!(
                        "      top-level split candidate: {} | gap {gap:.3} log10 | p {p:.3} (> 0.05: not a persistent step)",
                        fmt_day(s.day(a + sp))
                    );
                }
            }
            println!("      -> steps (persistent boundary, both sides >= {K_LEVEL} d) {steps}; short segments (< {K_LEVEL} d) {short}\n");
        }

        let mut all: Vec<f64> = s.vals.iter().map(|v| v.rms).collect();
        all.sort_by(f64::total_cmp);
        let q = |i: usize| -> f64 {
            let k = ((all.len() - 1) * i / 4).min(all.len() - 1);
            all[k]
        };
        let mut loudv: Vec<f64> = s.vals.iter().filter(|v| v.rms >= LOUD_HZ).map(|v| v.rms).collect();
        loudv.sort_by(f64::total_cmp);
        let quiet_med = median(
            &s.vals
                .iter()
                .filter(|v| v.rms < LOUD_HZ)
                .map(|v| v.rms)
                .collect::<Vec<f64>>(),
        );
        let loud_str = if loudv.is_empty() {
            "none".to_string()
        } else {
            let k = |i: usize| loudv[((loudv.len() - 1) * i / 4).min(loudv.len() - 1)];
            format!(
                "n {} p25/p50/p75 {}/{}/{} Hz max {:.4} Hz",
                loudv.len(),
                k(1),
                k(2),
                k(3),
                loudv[loudv.len() - 1]
            )
        };
        println!(
            "  day-rms (Hz): n {} p0/p25/p50/p75/p100 {:.4}/{:.4}/{:.4}/{:.4}/{:.4} | loud-day {loud_str} | quiet-day median {}",
            all.len(),
            q(0),
            q(1),
            q(2),
            q(3),
            q(4),
            fmt_opt(quiet_med)
        );

        levels.sort_by(f64::total_cmp);
        if levels.is_empty() {
            println!("  persistent level classes: none (no run reached {MIN_RUN_SEG} d or no segment >= {K_LEVEL} d)");
        } else {
            let mut classes: Vec<(String, Vec<(f64, String)>)> = Vec::new();
            for (lvl, when) in &levels_days {
                let k = (lvl.log10() * 2.0).floor() as i64;
                let lo = 10f64.powf(k as f64 / 2.0);
                let hi = 10f64.powf((k + 1) as f64 / 2.0);
                let label = format!("{lo:.3}..{hi:.3} Hz");
                match classes.iter_mut().find(|(c, _)| *c == label) {
                    Some((_, v)) => v.push((*lvl, when.clone())),
                    None => classes.push((label, vec![(*lvl, when.clone())])),
                }
            }
            classes.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
            let recurring: Vec<&(String, Vec<(f64, String)>)> =
                classes.iter().filter(|(_, v)| v.len() >= 2).collect();
            if recurring.is_empty() {
                println!(
                    "  occupied level classes (0.5 decade, segments >= {K_LEVEL} d): {} — no class recurs (no ladder rung occupied twice)",
                    classes.len()
                );
            } else {
                println!("  occupied level classes (0.5 decade, segments >= {K_LEVEL} d): {} | recurring classes (>= 2 segments):", classes.len());
                for (label, v) in recurring {
                    let ls: Vec<String> = v.iter().map(|(l, _)| format!("{l:.4}")).collect();
                    println!("      {label}: n {} levels [{}]", v.len(), ls.join(", "));
                }
            }
        }
        summary_rows.push(format!(
            "{:<22} {:>6} {:>6} {:>6} {:>7} {:>7} {:>7} {:>10}",
            format!("m{} st{}", s.mode, s.station),
            s.vals.len(),
            flips,
            tot_seg,
            tot_steps,
            tot_short,
            nl,
            loud_long
        ));
        println!();
    }

    println!("  M1 summary (series):");
    for row in &summary_rows {
        println!("    {row}");
    }

    println!();
    println!("############################################################");
    println!("## Measurement 2 — transfer entropy station -> station (Tages-RMS, log10)");
    println!("## common floor days (n >= {MIN_CELL}) at both stations of the same mode;");
    println!("## runs >= {MIN_RUN_TE} d analysed; source randomized for the phase/block nulls;");
    println!("## direction X->Y = state of X (source) on day t predicts Y on day t+lag past Y_t");
    println!("############################################################\n");

    let get_series =
        |station: i64, mode: i64| -> Option<&Series> {
            series.iter().find(|s| s.station == station && s.mode == mode)
        };

    for &mode in &MODES {
        let pairs = [(14i64, 43i64), (14, 63), (43, 63)];
        for &(sa, sb) in &pairs {
            let (Some(a), Some(b)) = (get_series(sa, mode), get_series(sb, mode)) else {
                continue;
            };
            let Some((days, la, lb)) = aligned(a, b) else {
                println!("mode {mode}: st{sa} x st{sb} — no common floor day");
                continue;
            };
            let Some(era) = era_from_days(&days) else {
                println!("mode {mode}: st{sa} x st{sb} — era month index void (named)");
                continue;
            };
            let runs = {
                let mut out: Vec<(usize, usize)> = Vec::new();
                if !days.is_empty() {
                    let mut start = 0usize;
                    for i in 1..days.len() {
                        if days[i] - days[i - 1] != 1 {
                            out.push((start, i - 1));
                            start = i;
                        }
                    }
                    out.push((start, days.len() - 1));
                }
                out
            };
            println!(
                "mode {mode}: st{sa} x st{sb} — common floor days n {}; runs {}",
                days.len(),
                runs.len()
            );
            for (p, q) in &runs {
                let nrun = q - p + 1;
                if nrun < MIN_RUN_TE {
                    println!(
                        "    common run {}..{} ({} d) < {MIN_RUN_TE} d: below TE minimum (named)",
                        fmt_day(days[*p]),
                        fmt_day(days[*q]),
                        nrun
                    );
                    continue;
                }
                println!(
                    "    common run {}..{} ({} d):",
                    fmt_day(days[*p]),
                    fmt_day(days[*q]),
                    nrun
                );
                let (sla, slb) = (&la[*p..=*q], &lb[*p..=*q]);
                print_dir_row(&format!("st{sa}"), &format!("st{sb}"), sla, slb, &era[*p..=*q]);
                print_dir_row(&format!("st{sb}"), &format!("st{sa}"), slb, sla, &era[*p..=*q]);
                println!();
            }
        }
    }

    println!("############################################################");
    println!("## Measurement 3 — candidate drivers -> floor (Tages-RMS, log10) on the TE runs");
    println!("## driver eps = solar elongation (deg) at the Earth per floor day cell (measured geometry);");
    println!("## constructed step drivers are marked 'constructed' (not a measured asset);");
    println!("## direction D->F = driver state on day t predicts floor on day t+lag past floor_t");
    println!("############################################################\n");

    let anchors = [
        (1995i64, 12i64, 5i64, "modswitch-1995-12-05"),
        (1996, 11, 1, "array-1996-11-01"),
    ];

    for s in &series {
        let runs = s.runs();
        for &(a, b) in &runs {
            let nrun = b - a + 1;
            if nrun < MIN_RUN_TE {
                continue;
            }
            let y: Vec<f32> = (a..=b).map(|i| s.vals[i].rms.log10() as f32).collect();
            let mut era: Vec<f32> = Vec::with_capacity(nrun);
            for i in a..=b {
                match month_index(s.day(i)) {
                    Some(m) => era.push(m as f32),
                    None => {
                        println!("mode {} st{} run: era month index void (named)", s.mode, s.station);
                        break;
                    }
                }
            }
            if era.len() != nrun {
                continue;
            }
            let epss: Vec<Option<f64>> = (a..=b).map(|i| s.vals[i].eps).collect();
            println!(
                "mode {} st{} — run {}..{} ({} d)",
                s.mode,
                s.station,
                fmt_day(s.day(a)),
                fmt_day(s.day(b)),
                nrun
            );
            let present: Vec<f64> = epss.iter().filter_map(|e| *e).collect();
            if present.len() == epss.len() {
                let lo = present.iter().cloned().fold(f64::INFINITY, f64::min);
                let hi = present.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
                let (nloud, nquiet) = (a..=b).fold((0usize, 0usize), |(nl, nq), i| {
                    if s.vals[i].rms >= LOUD_HZ {
                        (nl + 1, nq)
                    } else {
                        (nl, nq + 1)
                    }
                });
                let loud_eps: Vec<f64> = (a..=b)
                    .filter(|&i| s.vals[i].rms >= LOUD_HZ)
                    .filter_map(|i| s.vals[i].eps)
                    .collect();
                let quiet_eps: Vec<f64> = (a..=b)
                    .filter(|&i| s.vals[i].rms < LOUD_HZ)
                    .filter_map(|i| s.vals[i].eps)
                    .collect();
                println!(
                    "    eps driver: range {lo:.2}..{hi:.2} deg over the run; loud days n {nloud} (eps median {}), quiet n {nquiet} (eps median {})",
                    fmt_opt(median(&loud_eps)),
                    fmt_opt(median(&quiet_eps))
                );
                if hi - lo < 5.0 {
                    println!("    restriction: eps span < 5 deg in this window — the smooth-driver test is weak here (named)");
                }
                let mut e: Vec<f32> = Vec::with_capacity(nrun);
                for i in a..=b {
                    if let Some(v) = s.vals[i].eps {
                        e.push(v as f32);
                    }
                }
                if e.len() == nrun {
                    print_dir_row("eps", "floor", &e, &y, &era);
                    print_dir_row("floor", "eps", &y, &e, &era);
                } else {
                    println!("    eps driver: eps present on {}/{} days (named, driver skipped)", e.len(), nrun);
                }
            } else {
                println!(
                    "    eps driver: present on {}/{} days — geometry absent on the rest (named, driver skipped)",
                    present.len(),
                    epss.len()
                );
            }
            for (ay, am, ad, aname) in anchors {
                let Some(anchor_day) = days_from_civil(ay, am, ad) else {
                    continue;
                };
                let d0 = s.day(a);
                let d1 = s.day(b);
                if anchor_day <= d0 || anchor_day >= d1 {
                    continue;
                }
                let idx = (a..=b).find(|&i| s.day(i) >= anchor_day);
                let Some(first_after) = idx else {
                    continue;
                };
                let left = first_after - a;
                let right = b - first_after + 1;
                if left < 3 || right < 3 {
                    println!(
                        "    step driver {aname} (constructed): anchor inside run but side n {left}/{right} < 3 — not evaluated (named)"
                    );
                    continue;
                }
                let step: Vec<f32> = (a..=b)
                    .map(|i| if s.day(i) < anchor_day { 0.0f32 } else { 1.0 })
                    .collect();
                println!(
                    "    step driver {aname} (constructed, 0 before / 1 from anchor, side n {left}/{right}):"
                );
                print_dir_row(&format!("{aname}"), "floor", &step, &y, &era);
                print_dir_row("floor", &format!("{aname}"), &y, &step, &era);
            }
            println!();
        }
    }
    println!("finished");
}
