use std::collections::BTreeMap;

use omegaflow::atdf::parse_resid_bin;
use omegaflow::spectral::civil_from_days;

const DAY_S: f64 = 86400.0;
const LOCK_HZ: f64 = 1.0e3;
const MIN_CELL: usize = 30;
const DEFAULT_GAP_S: f64 = 600.0;
const SUB_GAP_S: f64 = 120.0;
const CHUNK_N: usize = 60;
const FLOOR_CAP: f64 = -2560.0;
const PLATEAU_MIN: f64 = -1900.0;
const EDGE_S: f64 = 120.0;
const STATIONS: [i64; 3] = [14, 43, 63];

fn median(vals: &[f64]) -> Option<f64> {
    if vals.is_empty() {
        return None;
    }
    let mut s = vals.to_vec();
    s.sort_by(f64::total_cmp);
    Some(s[s.len() / 2])
}

fn pct(sorted: &[f64], p: usize) -> Option<f64> {
    if sorted.is_empty() {
        return None;
    }
    let idx = ((sorted.len() * p) / 100).min(sorted.len() - 1);
    Some(sorted[idx])
}

fn fmt_o(v: Option<f64>) -> String {
    match v {
        Some(x) if x.is_finite() => format!("{x:.3}"),
        _ => "-".to_string(),
    }
}

fn jd_date(tdb_s: f64) -> String {
    let jd = 2451545.0 + tdb_s / DAY_S;
    let unix_day = (jd - 2440587.5).round() as i64;
    match civil_from_days(unix_day) {
        Some((y, m, d)) => format!("{y:04}-{m:02}-{d:02}"),
        None => format!("tdb {tdb_s:.0}"),
    }
}

fn classify(s: f64) -> u8 {
    if s == 0.0 {
        return 0;
    }
    if s <= FLOOR_CAP {
        return 1;
    }
    if s >= PLATEAU_MIN {
        return 2;
    }
    0
}

struct Chunk {
    label: u8,
    t0: f64,
    last_t: f64,
    n: usize,
    mean: f64,
    m2: f64,
}

impl Chunk {
    fn start(label: u8, t: f64, resid: f64) -> Chunk {
        Chunk { label, t0: t, last_t: t, n: 1, mean: resid, m2: 0.0 }
    }
    fn add(&mut self, t: f64, resid: f64) {
        self.last_t = t;
        self.n += 1;
        let d = resid - self.mean;
        self.mean += d / self.n as f64;
        let d2 = resid - self.mean;
        self.m2 += d * d2;
    }
}

struct Done {
    label: u8,
    n: usize,
    m2: f64,
    t0: f64,
    t1: f64,
}

struct PassOpen {
    t0: f64,
    t_last: f64,
    floor_all: usize,
    plateau_all: usize,
    done: Vec<Done>,
    cur: Option<Chunk>,
}

impl PassOpen {
    fn start(t: f64) -> PassOpen {
        PassOpen {
            t0: t,
            t_last: t,
            floor_all: 0,
            plateau_all: 0,
            done: Vec::new(),
            cur: None,
        }
    }
}

struct PassStat {
    t0: f64,
    dur_s: f64,
    floor_all: usize,
    plateau_all: usize,
    f_n: usize,
    f_m2: f64,
    p_n: usize,
    p_m2: f64,
    fi_n: usize,
    fi_m2: f64,
    pi_n: usize,
    pi_m2: f64,
}

fn pool_of(pass: &PassOpen, label: u8, interior: bool) -> (usize, f64) {
    let lo = pass.t0 + EDGE_S;
    let hi = pass.t_last - EDGE_S;
    let mut n = 0usize;
    let mut m2 = 0.0f64;
    for d in &pass.done {
        if d.label != label {
            continue;
        }
        if interior && (d.t0 < lo || d.t1 > hi) {
            continue;
        }
        n += d.n;
        m2 += d.m2;
    }
    (n, m2)
}

fn close_pass(pass: Option<PassOpen>) -> Option<PassStat> {
    let mut pass = pass?;
    if let Some(c) = pass.cur.take() {
        if c.n >= MIN_CELL {
            pass.done.push(Done { label: c.label, n: c.n, m2: c.m2, t0: c.t0, t1: c.last_t });
        }
    }
    let (f_n, f_m2) = pool_of(&pass, 1, false);
    let (p_n, p_m2) = pool_of(&pass, 2, false);
    let (fi_n, fi_m2) = pool_of(&pass, 1, true);
    let (pi_n, pi_m2) = pool_of(&pass, 2, true);
    Some(PassStat {
        t0: pass.t0,
        dur_s: pass.t_last - pass.t0,
        floor_all: pass.floor_all,
        plateau_all: pass.plateau_all,
        f_n,
        f_m2,
        p_n,
        p_m2,
        fi_n,
        fi_m2,
        pi_n,
        pi_m2,
    })
}

struct KeyState {
    prev: Option<f64>,
    pass: Option<PassOpen>,
}

impl KeyState {
    fn new() -> KeyState {
        KeyState { prev: None, pass: None }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let gap_s = match args.iter().filter_map(|a| a.parse::<f64>().ok()).next() {
        Some(v) => v,
        None => DEFAULT_GAP_S,
    };

    let Ok(bytes) = std::fs::read("data/galileo_resid.bin") else {
        eprintln!("galileo: resid bin void");
        return;
    };
    let Some(recs) = parse_resid_bin(&bytes) else {
        eprintln!("galileo: resid bin parse void");
        return;
    };
    drop(bytes);

    let mut mode_samples: BTreeMap<i64, usize> = BTreeMap::new();
    let mut mode_lock: BTreeMap<i64, usize> = BTreeMap::new();
    let mut state: BTreeMap<(i64, i64), KeyState> = BTreeMap::new();
    let mut passmap: BTreeMap<(i64, i64), Vec<PassStat>> = BTreeMap::new();

    for r in &recs {
        let mode = r[3] as i64;
        if mode != 1 && mode != 2 {
            continue;
        }
        let station = r[2] as i64;
        if !STATIONS.contains(&station) {
            continue;
        }
        *mode_samples.entry(mode).or_insert(0) += 1;
        let t = r[0];
        let resid = r[1];
        let key = (mode, station);
        if resid.abs() > LOCK_HZ {
            *mode_lock.entry(mode).or_insert(0) += 1;
        }
        let ks = state.entry(key).or_insert_with(KeyState::new);
        let boundary = match ks.prev {
            None => true,
            Some(p) => t - p > gap_s,
        };
        if boundary {
            if let Some(ps) = close_pass(ks.pass.take()) {
                passmap.entry(key).or_default().push(ps);
            }
        }
        let pass = ks.pass.get_or_insert_with(|| PassOpen::start(t));
        pass.t_last = t;
        if resid.abs() <= LOCK_HZ {
            let s = r[7];
            let lbl = classify(s);
            if lbl == 0 {
                if let Some(c) = pass.cur.take() {
                    if c.n >= MIN_CELL {
                        pass.done.push(Done { label: c.label, n: c.n, m2: c.m2, t0: c.t0, t1: c.last_t });
                    }
                }
            } else {
                match lbl {
                    1 => {
                        pass.floor_all += 1;
                    }
                    2 => {
                        pass.plateau_all += 1;
                    }
                    _ => {}
                }
                let need_new = match &pass.cur {
                    None => true,
                    Some(c) => c.label != lbl || t - c.last_t > SUB_GAP_S || c.n >= CHUNK_N,
                };
                if need_new {
                    if let Some(c) = pass.cur.take() {
                        if c.n >= MIN_CELL {
                            pass.done.push(Done { label: c.label, n: c.n, m2: c.m2, t0: c.t0, t1: c.last_t });
                        }
                    }
                    pass.cur = Some(Chunk::start(lbl, t, resid));
                } else {
                    if let Some(c) = pass.cur.as_mut() {
                        c.add(t, resid);
                    }
                }
            }
        }
        ks.prev = Some(t);
    }
    for ((mode, station), ks) in &mut state {
        if let Some(ps) = close_pass(ks.pass.take()) {
            passmap.entry((*mode, *station)).or_default().push(ps);
        }
    }
    drop(state);

    let mut out: Vec<String> = Vec::new();
    out.push("galileo pass strength ramp — the within-pass AGC-floor noise test".to_string());
    out.push("binding: pass = contiguous tracking arc per (station, mode), boundary = time gap between consecutive samples > gap_s; lock transitions (|resid| > 1000 Hz) excluded; strength state floor = signal_strength <= -2560 (AGC clamp), plateau = signal_strength >= -1900, values between or 0 excluded as transition/pad; sub-arc = contiguous same-state run, split on state change, a > 120 s gap between non-lock samples, or 60 samples (local detrend scale); noise = resid RMS about the sub-arc (chunk) mean, pooled per state within the pass; a chunk enters the pool at >= 30 non-lock samples; a pass is dual when its floor pool and its plateau pool both hold >= 30 non-lock samples; interior variant keeps only chunks fully inside pass_t0 + 120 s .. pass_t_last - 120 s (acquisition-transient control)".to_string());
    out.push(format!("pass gap threshold: {gap_s:.0} s (default {DEFAULT_GAP_S:.0} s); stations 14/43/63; modes 1 and 2"));
    out.push(String::new());

    out.push("overview".to_string());
    for mode in [1i64, 2] {
        out.push(format!(
            "  mode {mode}: {} samples at 14/43/63, {} lock transitions",
            mode_samples.get(&mode).copied().unwrap_or(0),
            mode_lock.get(&mode).copied().unwrap_or(0)
        ));
    }
    out.push(String::new());

    out.push("pass structure and dual-pass counts (dual = floor pool >= 30 and plateau pool >= 30)".to_string());
    out.push("  st mode passes floor_pres plateau_pres dual_full dual_int floor_only plateau_only neither".to_string());
    let mut keys: Vec<(i64, i64)> = passmap.keys().copied().collect();
    keys.sort();
    for (mode, station) in &keys {
        let passes = passmap[&(*mode, *station)].len();
        let mut fp = 0usize;
        let mut pp = 0usize;
        let mut dual_f = 0usize;
        let mut dual_i = 0usize;
        let mut f_only = 0usize;
        let mut p_only = 0usize;
        let mut neither = 0usize;
        for ps in &passmap[&(*mode, *station)] {
            let hf = ps.floor_all >= MIN_CELL;
            let hp = ps.plateau_all >= MIN_CELL;
            let hfd = ps.f_n >= MIN_CELL && ps.p_n >= MIN_CELL;
            let hid = ps.fi_n >= MIN_CELL && ps.pi_n >= MIN_CELL;
            if hf {
                fp += 1;
            }
            if hp {
                pp += 1;
            }
            if hfd {
                dual_f += 1;
            }
            if hid {
                dual_i += 1;
            }
            if hf && hp {
            } else if hf {
                f_only += 1;
            } else if hp {
                p_only += 1;
            } else {
                neither += 1;
            }
        }
        out.push(format!(
            "  {station} {mode} {} {} {} {} {} {} {} {}",
            passes, fp, pp, dual_f, dual_i, f_only, p_only, neither
        ));
    }
    out.push(String::new());

    for (mode, interior) in [(1i64, false), (1, true), (2, false), (2, true)] {
        let tag = if interior { "interior" } else { "full" };
        out.push(format!("paired within-pass noise, mode {mode}, {tag} sub-arcs: floor vs plateau of the same pass", mode = mode));
        out.push("  st n_dual med_floor med_plateau med_diff mean_diff floor>plat floor<plat med_ratio p25_diff p75_diff".to_string());
        let mut pooled_diff: Vec<f64> = Vec::new();
        let mut pooled_ratio: Vec<f64> = Vec::new();
        for station in STATIONS {
            let key = (mode, station);
            let mut floor_v: Vec<f64> = Vec::new();
            let mut plat_v: Vec<f64> = Vec::new();
            let mut diff_v: Vec<f64> = Vec::new();
            let mut ratio_v: Vec<f64> = Vec::new();
            let v = passmap.get(&key);
            if let Some(list) = v {
                for ps in list {
                    let (fn_, fm2) = if interior { (ps.fi_n, ps.fi_m2) } else { (ps.f_n, ps.f_m2) };
                    let (pn_, pm2) = if interior { (ps.pi_n, ps.pi_m2) } else { (ps.p_n, ps.p_m2) };
                    if fn_ < MIN_CELL || pn_ < MIN_CELL {
                        continue;
                    }
                    let fr = (fm2 / fn_ as f64).sqrt();
                    let pr = (pm2 / pn_ as f64).sqrt();
                    if !fr.is_finite() || !pr.is_finite() {
                        continue;
                    }
                    floor_v.push(fr);
                    plat_v.push(pr);
                    diff_v.push(fr - pr);
                    ratio_v.push(fr / pr);
                }
            }
            let n_dual = diff_v.len();
            let above = diff_v.iter().filter(|d| **d > 0.0).count();
            let below = diff_v.iter().filter(|d| **d < 0.0).count();
            let mean_d: Option<f64> = if diff_v.is_empty() {
                None
            } else {
                Some(diff_v.iter().sum::<f64>() / diff_v.len() as f64)
            };
            let mut ds = diff_v.clone();
            ds.sort_by(f64::total_cmp);
            out.push(format!(
                "  {station} {} {} {} {} {} {} {} {} {} {}",
                n_dual,
                fmt_o(median(&floor_v)),
                fmt_o(median(&plat_v)),
                fmt_o(median(&diff_v)),
                fmt_o(mean_d),
                above,
                below,
                fmt_o(median(&ratio_v)),
                fmt_o(pct(&ds, 25)),
                fmt_o(pct(&ds, 75))
            ));
            if n_dual > 0 {
                pooled_diff.extend(diff_v);
                pooled_ratio.extend(ratio_v);
            }
        }
        let mut pds = pooled_diff.clone();
        pds.sort_by(f64::total_cmp);
        let above = pooled_diff.iter().filter(|d| **d > 0.0).count();
        let below = pooled_diff.iter().filter(|d| **d < 0.0).count();
        let mean_d: Option<f64> = if pooled_diff.is_empty() {
            None
        } else {
            Some(pooled_diff.iter().sum::<f64>() / pooled_diff.len() as f64)
        };
        out.push(format!(
            "  pooled {} | med_diff {} (p10 {} p25 {} p50 {} p75 {} p90 {}) mean_diff {} | floor>plateau {} floor<plateau {} | med_ratio {}",
            pooled_diff.len(),
            fmt_o(median(&pooled_diff)),
            fmt_o(pct(&pds, 10)),
            fmt_o(pct(&pds, 25)),
            fmt_o(pct(&pds, 50)),
            fmt_o(pct(&pds, 75)),
            fmt_o(pct(&pds, 90)),
            fmt_o(mean_d),
            above,
            below,
            fmt_o(median(&pooled_ratio))
        ));
        out.push(String::new());
    }

    out.push("per-pass pairs (full), mode 1 (dual passes): date dur_s floor_n floor_rms plat_n plat_rms diff".to_string());
    for station in STATIONS {
        let key = (1, station);
        if let Some(list) = passmap.get(&key) {
            for ps in list {
                if ps.f_n < MIN_CELL || ps.p_n < MIN_CELL {
                    continue;
                }
                let fr = (ps.f_m2 / ps.f_n as f64).sqrt();
                let pr = (ps.p_m2 / ps.p_n as f64).sqrt();
                if !fr.is_finite() || !pr.is_finite() {
                    continue;
                }
                out.push(format!(
                    "  st{station} {} {:.0} {} {:.3} {} {:.3} {:.3}",
                    jd_date(ps.t0),
                    ps.dur_s,
                    ps.f_n,
                    fr,
                    ps.p_n,
                    pr,
                    fr - pr
                ));
            }
        }
    }
    let body = out.join("\n") + "\n";
    println!("{body}");
}
