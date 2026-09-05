use std::collections::{BTreeMap, BTreeSet};

use omegaflow::atdf::parse_resid_bin;

const DAY_S: f64 = 86400.0;
const LOCK_HZ: f64 = 1.0e3;
const MIN_CELL: usize = 30;
const GAP_PASS_S: f64 = 600.0;
const GAP_SEG_S: f64 = 60.0;
const MIN_SEG: usize = 120;
const STATIONS: [i64; 3] = [14, 43, 63];
const MODES: [i64; 2] = [1, 2];

fn median(vals: &[f64]) -> Option<f64> {
    if vals.is_empty() {
        return None;
    }
    let mut s = vals.to_vec();
    s.sort_by(f64::total_cmp);
    Some(s[s.len() / 2])
}

fn fmt_o(v: Option<f64>) -> String {
    match v {
        Some(x) if x.is_finite() => format!("{x:.2}"),
        _ => "-".to_string(),
    }
}

struct Cell {
    n: usize,
    mean: f64,
    m2: f64,
}

impl Cell {
    fn new() -> Cell {
        Cell {
            n: 0,
            mean: 0.0,
            m2: 0.0,
        }
    }
    fn add(&mut self, x: f64) {
        self.n += 1;
        let d = x - self.mean;
        self.mean += d / self.n as f64;
        let d2 = x - self.mean;
        self.m2 += d * d2;
    }
    fn rms(&self) -> Option<f64> {
        if self.n == 0 {
            return None;
        }
        Some((self.m2 / self.n as f64).sqrt())
    }
}

struct PassStat {
    n_all: usize,
    rms_all: Option<f64>,
    const_kept: Option<f64>,
    block_kept: Option<f64>,
    detr_kept: Option<f64>,
}

fn pass_stats(t: &[f64], x: &[f64]) -> PassStat {
    let n_all = x.len();
    let mut c = Cell::new();
    for v in x {
        c.add(*v);
    }
    let rms_all = c.rms().filter(|r| r.is_finite());
    let mut blocks: Vec<(usize, usize)> = Vec::new();
    let mut lo = 0usize;
    while lo < n_all {
        let mut hi = lo + 1;
        while hi < n_all && t[hi] - t[hi - 1] <= GAP_SEG_S {
            hi += 1;
        }
        if hi - lo >= MIN_SEG {
            blocks.push((lo, hi));
        }
        lo = hi;
    }
    let n_kept: usize = blocks.iter().map(|&(a, b)| b - a).sum();
    if n_kept == 0 {
        return PassStat {
            n_all,
            rms_all,
            const_kept: None,
            block_kept: None,
            detr_kept: None,
        };
    }
    let nkf = n_kept as f64;
    let mut mean_kept = 0.0;
    for &(a, b) in &blocks {
        for v in &x[a..b] {
            mean_kept += v;
        }
    }
    mean_kept /= nkf;
    let mut sse_const = 0.0;
    let mut sse_block = 0.0;
    let mut sse_detr = 0.0;
    for &(a, b) in &blocks {
        let seg = &x[a..b];
        for v in seg {
            let d = v - mean_kept;
            sse_const += d * d;
        }
        let seg_mean = seg.iter().sum::<f64>() / seg.len() as f64;
        for v in seg {
            let d = v - seg_mean;
            sse_block += d * d;
        }
        let nf = (b - a) as f64;
        let mt = t[a..b].iter().sum::<f64>() / nf;
        let mut num = 0.0;
        let mut den = 0.0;
        for k in a..b {
            let dt = t[k] - mt;
            num += dt * (x[k] - seg_mean);
            den += dt * dt;
        }
        let slope = if den > 0.0 { num / den } else { 0.0 };
        for k in a..b {
            let r = x[k] - (seg_mean + slope * (t[k] - mt));
            sse_detr += r * r;
        }
    }
    PassStat {
        n_all,
        rms_all,
        const_kept: Some((sse_const / nkf).sqrt()),
        block_kept: Some((sse_block / nkf).sqrt()),
        detr_kept: Some((sse_detr / nkf).sqrt()),
    }
}

struct Key {
    prev: Option<f64>,
    t: Vec<f64>,
    x: Vec<f64>,
    stats: Vec<PassStat>,
}

impl Key {
    fn new() -> Key {
        Key {
            prev: None,
            t: Vec::new(),
            x: Vec::new(),
            stats: Vec::new(),
        }
    }
}

fn main() {
    let Ok(bytes) = std::fs::read("data/galileo_resid.bin") else {
        eprintln!("galileo: resid bin void");
        return;
    };
    let Some(recs) = parse_resid_bin(&bytes) else {
        eprintln!("galileo: resid bin parse void");
        return;
    };
    drop(bytes);

    let mut want: BTreeSet<(i64, i64)> = BTreeSet::new();
    for m in MODES {
        for s in STATIONS {
            want.insert((m, s));
        }
    }

    let mut sm: BTreeMap<(i64, i64), usize> = BTreeMap::new();
    let mut sdays: BTreeMap<(i64, i64), BTreeSet<i64>> = BTreeMap::new();
    let mut stationday: BTreeMap<(i64, i64, i64), Cell> = BTreeMap::new();
    for r in &recs {
        let key = (r[3] as i64, r[2] as i64);
        if !want.contains(&key) {
            continue;
        }
        let day = (r[0] / DAY_S).floor() as i64;
        *sm.entry(key).or_insert(0) += 1;
        sdays.entry(key).or_default().insert(day);
        if r[1].abs() <= LOCK_HZ {
            stationday
                .entry((key.0, key.1, day))
                .or_insert_with(Cell::new)
                .add(r[1]);
        }
    }

    let mut state: BTreeMap<(i64, i64), Key> = BTreeMap::new();
    for r in &recs {
        let key = (r[3] as i64, r[2] as i64);
        if !want.contains(&key) {
            continue;
        }
        let t = r[0];
        let k = state.entry(key).or_insert_with(Key::new);
        let boundary = match k.prev {
            None => true,
            Some(p) => t - p > GAP_PASS_S,
        };
        if boundary && !k.t.is_empty() {
            k.stats.push(pass_stats(&k.t, &k.x));
            k.t.clear();
            k.x.clear();
        }
        if r[1].abs() <= LOCK_HZ {
            k.t.push(t);
            k.x.push(r[1]);
        }
        k.prev = Some(t);
    }
    for k in state.values_mut() {
        if !k.t.is_empty() {
            k.stats.push(pass_stats(&k.t, &k.x));
        }
    }

    let mut out: Vec<String> = Vec::new();
    out.push("galileo pass detrend floor — intra-pass linear detrend (60 s gap, min 120) vs pass mean floor".to_string());
    out.push("binding: pass = contiguous tracking arc per (station, mode), boundary = time gap > 600 s; lock |resid| > 1000 Hz excluded; rms_all = RMS about the pass mean (all non-lock samples); const/block/detr measured on kept samples = non-lock samples inside sub-segments >= 120 samples split at gaps > 60 s; const = RMS about the kept-pass mean, block = RMS about each sub-segment mean, detr = RMS about each sub-segment linear LS fit; floors = median over passes with >= 30 samples".to_string());
    out.push(String::new());

    out.push("mode st samples days passes_ge30 pass_floor_hz matched_pass_floor_hz const_kept block_kept detr_kept".to_string());
    for mode in MODES {
        for station in STATIONS {
            let key = (mode, station);
            let Some(k) = state.get(&key) else {
                continue;
            };
            let samples = sm.get(&key).copied().unwrap_or(0);
            let days = sdays.get(&key).map(|d| d.len()).unwrap_or(0);
            let ge30: Vec<&PassStat> = k
                .stats
                .iter()
                .filter(|p| p.n_all >= MIN_CELL && p.rms_all.is_some())
                .collect();
            let matched: Vec<&PassStat> = ge30
                .iter()
                .copied()
                .filter(|p| p.detr_kept.is_some())
                .collect();
            let pf: Vec<f64> = ge30.iter().filter_map(|p| p.rms_all).collect();
            let mpf: Vec<f64> = matched.iter().filter_map(|p| p.rms_all).collect();
            let ck: Vec<f64> = matched.iter().filter_map(|p| p.const_kept).collect();
            let bk: Vec<f64> = matched.iter().filter_map(|p| p.block_kept).collect();
            let dk: Vec<f64> = matched.iter().filter_map(|p| p.detr_kept).collect();
            out.push(format!(
                "{} {} {} {} {} {} {} {} {} {}",
                mode,
                station,
                samples,
                days,
                ge30.len(),
                fmt_o(median(&pf)),
                fmt_o(median(&mpf)),
                fmt_o(median(&ck)),
                fmt_o(median(&bk)),
                fmt_o(median(&dk))
            ));
        }
    }
    out.push(String::new());

    out.push("station-day floor (day cell RMS median, cells >= 30 non-lock samples)".to_string());
    for mode in MODES {
        for station in STATIONS {
            let list: Vec<f64> = stationday
                .iter()
                .filter(|((m, s, _), _)| *m == mode && *s == station)
                .filter_map(|((_, _, _), c)| if c.n >= MIN_CELL { c.rms() } else { None })
                .filter(|r| r.is_finite())
                .collect();
            out.push(format!(
                "  mode {mode} station {station}: {} Hz ({} cells)",
                fmt_o(median(&list)),
                list.len()
            ));
        }
    }
    out.push(String::new());

    out.push("drift decomposition on matched passes (ge30 with kept sub-segments): detr/const ratio p50, count ratio < 0.9, mean rms_all minus mean detr (drift carried by slow pass trend)".to_string());
    for mode in MODES {
        for station in STATIONS {
            let key = (mode, station);
            let Some(k) = state.get(&key) else {
                continue;
            };
            let mut ratios: Vec<f64> = Vec::new();
            let mut below = 0usize;
            let mut ar: Vec<f64> = Vec::new();
            let mut ad: Vec<f64> = Vec::new();
            for p in k.stats.iter() {
                if p.n_all < MIN_CELL {
                    continue;
                }
                if let (Some(c), Some(d)) = (p.const_kept, p.detr_kept) {
                    let r = if c > 0.0 { d / c } else { 1.0 };
                    ratios.push(r);
                    if r < 0.9 {
                        below += 1;
                    }
                    ar.push(c);
                    ad.push(d);
                }
            }
            let ma = median(&ar).unwrap_or(0.0);
            let md = median(&ad).unwrap_or(0.0);
            out.push(format!(
                "  mode {mode} station {station}: ratio p50 {}, ratio<0.9 {}/{} passes, med const {:.2} -> med detr {:.2} Hz (slow-trend share {:.2} Hz)",
                fmt_o(median(&ratios)),
                below,
                ratios.len(),
                ma,
                md,
                ma - md
            ));
        }
    }
    out.push(String::new());

    out.push(format!("registers: station-day floor is the day-cell RMS median (>= 30 non-lock); the pass floor rms_all and its matched count reproduce the pass befund segmentation (600 s pass gap)"));

    let body = out.join("\n") + "\n";
    println!("{body}");
}
