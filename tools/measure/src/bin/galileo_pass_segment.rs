use std::collections::{BTreeMap, BTreeSet, HashMap};

use omegaflow::archivar::{body_barycenter_position, parse_ephemeris_binary, BodyEphemeris};
use omegaflow::atdf::parse_resid_bin;
use omegaflow::spectral::civil_from_days;

const DAY_S: f64 = 86400.0;
const LOCK_HZ: f64 = 1.0e3;
const MIN_CELL: usize = 30;
const DEFAULT_GAP_S: f64 = 600.0;
const STATIONS: [i64; 3] = [14, 43, 63];

fn norm(v: [f64; 3]) -> f64 {
    (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
}

fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

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

fn pct_f(sorted: &[f64], frac: f64) -> Option<f64> {
    if sorted.is_empty() {
        return None;
    }
    let idx = ((sorted.len() as f64 * frac).floor() as usize).min(sorted.len() - 1);
    Some(sorted[idx])
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
        Cell { n: 0, mean: 0.0, m2: 0.0 }
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

fn load(name: &str, eph: &mut HashMap<String, BodyEphemeris>) -> bool {
    let p = format!("data/ephemeris_{name}.bin");
    std::fs::read(&p)
        .ok()
        .and_then(|d| parse_ephemeris_binary(&d))
        .map(|e| {
            eph.insert(name.to_string(), e);
        })
        .is_some()
}

fn jd_date(tdb_s: f64) -> String {
    let jd = 2451545.0 + tdb_s / DAY_S;
    let unix_day = (jd - 2440587.5).round() as i64;
    match civil_from_days(unix_day) {
        Some((y, m, d)) => format!("{y:04}-{m:02}-{d:02}"),
        None => format!("tdb {tdb_s:.0}"),
    }
}

fn elong_deg_at(t: f64, eph: &HashMap<String, BodyEphemeris>) -> Option<f64> {
    let p = body_barycenter_position("galileo_daily", t, eph)?;
    let e = body_barycenter_position("earth", t, eph)?;
    let sun = [0.0, 0.0, 0.0];
    let re = norm(e);
    let e_to_p = sub(p, e);
    let r_ep = norm(e_to_p);
    if re <= 0.0 || r_ep <= 0.0 {
        return None;
    }
    let cos_eps = dot(sub(sun, e), e_to_p) / (re * r_ep);
    Some(cos_eps.clamp(-1.0, 1.0).acos().to_degrees())
}

struct Grp {
    prev: Option<f64>,
    open: Option<Open>,
}

impl Grp {
    fn new() -> Grp {
        Grp { prev: None, open: None }
    }
}

struct Open {
    t0: f64,
    t_last: f64,
    xs: Vec<f64>,
    ss: Vec<f64>,
}

impl Open {
    fn start(t: f64) -> Open {
        Open { t0: t, t_last: t, xs: Vec::new(), ss: Vec::new() }
    }
}

struct PassStat {
    t0: f64,
    dur_s: f64,
    n_xs: usize,
    rms: Option<f64>,
    med_resid: Option<f64>,
    med_strength: Option<f64>,
}

fn finish_pass(o: Option<Open>, passmap: &mut BTreeMap<(i64, i64), Vec<PassStat>>, key: (i64, i64)) {
    if let Some(o) = o {
        let n_xs = o.xs.len();
        let mut c = Cell::new();
        for x in &o.xs {
            c.add(*x);
        }
        let rms = c.rms().filter(|r| r.is_finite());
        passmap.entry(key).or_default().push(PassStat {
            t0: o.t0,
            dur_s: o.t_last - o.t0,
            n_xs,
            rms,
            med_resid: median(&o.xs),
            med_strength: median(&o.ss),
        });
    }
}

fn count_passes(recs: &[[f64; 8]], gap_s: f64) -> BTreeMap<(i64, i64), usize> {
    let mut out: BTreeMap<(i64, i64), usize> = BTreeMap::new();
    let mut prev: BTreeMap<(i64, i64), f64> = BTreeMap::new();
    for r in recs {
        let key = (r[3] as i64, r[2] as i64);
        let t = r[0];
        match prev.get(&key) {
            None => {
                *out.entry(key).or_insert(0) += 1;
            }
            Some(p) => {
                if t - p > gap_s {
                    *out.entry(key).or_insert(0) += 1;
                }
            }
        }
        prev.insert(key, t);
    }
    out
}

fn quartile_boundaries(counts: &BTreeMap<i64, usize>) -> [Option<i64>; 3] {
    let total: usize = counts.values().sum();
    let mut out = [None; 3];
    if total == 0 {
        return out;
    }
    let mut cum = 0usize;
    for (v, &c) in counts {
        cum += c;
        for (k, slot) in out.iter_mut().enumerate() {
            if slot.is_none() && cum * 4 >= total * (k + 1) {
                *slot = Some(*v);
            }
        }
        if out.iter().all(|s| s.is_some()) {
            break;
        }
    }
    out
}

fn q_of(s: f64, b: &[Option<i64>; 3]) -> u8 {
    if s == 0.0 {
        return 0;
    }
    let sv = s as i64;
    let mut q = 4u8;
    for (k, cut) in b.iter().enumerate() {
        if let Some(c) = cut {
            if sv <= *c {
                q = (k + 1) as u8;
                break;
            }
        }
    }
    q
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let gap_s = match args.iter().filter_map(|a| a.parse::<f64>().ok()).next() {
        Some(v) => v,
        None => DEFAULT_GAP_S,
    };

    let mut eph: HashMap<String, BodyEphemeris> = HashMap::new();
    let mut geometry_ok = true;
    for b in ["galileo_daily", "earth"] {
        if !load(b, &mut eph) {
            eprintln!("galileo: {b} ephemeris bin void — geometry sections skipped");
            geometry_ok = false;
        }
    }

    let Ok(bytes) = std::fs::read("data/galileo_resid.bin") else {
        eprintln!("galileo: resid bin void");
        return;
    };
    let Some(recs) = parse_resid_bin(&bytes) else {
        eprintln!("galileo: resid bin parse void");
        return;
    };
    drop(bytes);

    let total = recs.len();
    let mut mode_samples: BTreeMap<i64, usize> = BTreeMap::new();
    let mut mode_lock: BTreeMap<i64, usize> = BTreeMap::new();
    let mut mode_days: BTreeMap<i64, BTreeSet<i64>> = BTreeMap::new();
    let mut sm_samples: BTreeMap<(i64, i64), usize> = BTreeMap::new();
    let mut sm_days: BTreeMap<(i64, i64), BTreeSet<i64>> = BTreeMap::new();
    let mut daycell: BTreeMap<(i64, i64), Cell> = BTreeMap::new();
    let mut stationday: BTreeMap<(i64, i64, i64), Cell> = BTreeMap::new();
    let mut strength_counts: BTreeMap<i64, usize> = BTreeMap::new();

    for r in &recs {
        let mode = r[3] as i64;
        let station = r[2] as i64;
        let day = (r[0] / DAY_S).floor() as i64;
        *mode_samples.entry(mode).or_insert(0) += 1;
        mode_days.entry(mode).or_default().insert(day);
        *sm_samples.entry((mode, station)).or_insert(0) += 1;
        sm_days.entry((mode, station)).or_default().insert(day);
        if r[1].abs() > LOCK_HZ {
            *mode_lock.entry(mode).or_insert(0) += 1;
        } else {
            daycell.entry((mode, day)).or_insert_with(Cell::new).add(r[1]);
            stationday
                .entry((mode, station, day))
                .or_insert_with(Cell::new)
                .add(r[1]);
            if mode == 1 {
                *strength_counts.entry(r[7] as i64).or_insert(0) += 1;
            }
        }
    }

    let bounds = quartile_boundaries(&strength_counts);
    let cut_str = |k: usize| -> String {
        match bounds[k] {
            Some(v) => v.to_string(),
            None => "-".to_string(),
        }
    };

    let mut passmap: BTreeMap<(i64, i64), Vec<PassStat>> = BTreeMap::new();
    let mut state: BTreeMap<(i64, i64), Grp> = BTreeMap::new();
    let mut dts: Vec<f64> = Vec::new();
    let dt_edges: [f64; 16] = [
        0.0, 1.0, 2.0, 5.0, 10.0, 30.0, 60.0, 120.0, 300.0, 600.0, 1800.0, 3600.0, 7200.0, 21600.0,
        86400.0, 3.0 * DAY_S,
    ];
    let mut dtbins = [0usize; 15];
    let dt_labels: [&str; 15] = [
        "<1s", "1-2s", "2-5s", "5-10s", "10-30s", "30-60s", "1-2m", "2-5m", "5-10m", "10-30m",
        "30-60m", "1-2h", "2-6h", "6-24h", "1-3d",
    ];

    for r in &recs {
        let t = r[0];
        let mode = r[3] as i64;
        let station = r[2] as i64;
        let key = (mode, station);
        let g = state.entry(key).or_insert_with(Grp::new);
        let boundary = match g.prev {
            None => true,
            Some(p) => t - p > gap_s,
        };
        if boundary {
            finish_pass(g.open.take(), &mut passmap, key);
        }
        match g.prev {
            Some(p) => {
                let d = t - p;
                let dv = if d >= 0.0 { d } else { 0.0 };
                dts.push(dv);
                let mut bin = 0usize;
                for (bi, ed) in dt_edges.iter().enumerate().skip(1) {
                    if dv >= *ed {
                        bin = bi - 1;
                    }
                }
                if dv >= 3.0 * DAY_S {
                    bin = 14;
                }
                dtbins[bin] += 1;
            }
            None => {}
        }
        let o = g.open.get_or_insert_with(|| Open::start(t));
        o.t_last = t;
        if r[1].abs() > LOCK_HZ {
            // lock transition: counted, excluded from the noise series
        } else {
            o.xs.push(r[1]);
            o.ss.push(r[7]);
        }
        g.prev = Some(t);
    }
    for ((mode, station), g) in &mut state {
        finish_pass(g.open.take(), &mut passmap, (*mode, *station));
    }
    drop(state);

    dts.sort_by(f64::total_cmp);
    let d50 = pct(&dts, 50);
    let d90 = pct(&dts, 90);
    let d99 = pct(&dts, 99);
    let d999 = pct_f(&dts, 0.999);
    let dmax = dts.last().copied();

    let mut out: Vec<String> = Vec::new();
    out.push("galileo pass segmentation — pass-level resid floor vs day-cell floor".to_string());
    out.push(format!("binding: pass = contiguous tracking arc per (station, mode); boundary = time gap between consecutive samples > {gap_s:.0} s; lock transitions (|resid| > {LOCK_HZ:.0} Hz) excluded from noise; noise = RMS about the cell/pass mean; RMS lists require >= {MIN_CELL} non-lock samples"));
    out.push(String::new());

    out.push("overview".to_string());
    out.push(format!("  total resid samples: {total}"));
    for (mode, s) in &mode_samples {
        out.push(format!(
            "  mode {mode}: {} samples, {} lock transitions, {} days",
            s,
            mode_lock.get(mode).copied().unwrap_or(0),
            mode_days.get(mode).map(|d| d.len()).unwrap_or(0)
        ));
    }
    out.push(String::new());

    out.push("sample gap (dt) between consecutive samples of the same (station, mode)".to_string());
    out.push(format!(
        "  dt deciles: p50 {} s, p90 {} s, p99 {} s, p99.9 {} s, max {} s",
        fmt_o(d50),
        fmt_o(d90),
        fmt_o(d99),
        fmt_o(d999),
        fmt_o(dmax)
    ));
    let mut hist = String::from("  dt histogram: ");
    for (bi, lb) in dt_labels.iter().enumerate() {
        hist.push_str(&format!("{lb} {}, ", dtbins[bi]));
    }
    out.push(hist.trim_end_matches(", ").to_string());
    out.push(format!(
        "  pass gap threshold: {gap_s:.0} s (default {DEFAULT_GAP_S:.0} s, argv override accepted)"
    ));
    out.push(String::new());

    out.push("pass-count sensitivity to the gap threshold (all stations)".to_string());
    for cand in [120.0f64, 300.0, 600.0, 900.0, 1800.0, 3600.0, 7200.0] {
        let cnt = count_passes(&recs, cand);
        let mut mode1 = 0usize;
        let mut mode2 = 0usize;
        let mut total_p = 0usize;
        for ((mode, station), c) in &cnt {
            total_p += c;
            if *mode == 1 && STATIONS.contains(station) {
                mode1 += c;
            }
            if *mode == 2 && STATIONS.contains(station) {
                mode2 += c;
            }
        }
        out.push(format!(
            "  gap {cand:.0} s: total passes {total_p}, passes mode 1 @14/43/63 {mode1}, passes mode 2 @14/43/63 {mode2}"
        ));
    }
    out.push(String::new());

    out.push("pass structure per station per mode".to_string());
    out.push("  st mode samples days passes pass_ge30 med_dur_min med_pass_n med_pass_resid_hz".to_string());
    let mut keys: Vec<(i64, i64)> = passmap.keys().copied().collect();
    keys.sort();
    for (mode, station) in keys {
        let passes = passmap[&(mode, station)].len();
        if passes == 0 {
            continue;
        }
        let dur: Vec<f64> = passmap[&(mode, station)].iter().map(|p| p.dur_s / 60.0).collect();
        let ge30: Vec<&PassStat> = passmap[&(mode, station)]
            .iter()
            .filter(|p| p.n_xs >= MIN_CELL)
            .collect();
        let pass_n: Vec<f64> = ge30.iter().map(|p| p.n_xs as f64).collect();
        let resid_med: Vec<f64> = ge30.iter().filter_map(|p| p.med_resid).collect();
        out.push(format!(
            "  {station} {mode} {} {} {} {} {} {} {}",
            sm_samples.get(&(mode, station)).copied().unwrap_or(0),
            sm_days.get(&(mode, station)).map(|d| d.len()).unwrap_or(0),
            passes,
            ge30.len(),
            fmt_o(median(&dur)),
            fmt_o(median(&pass_n)),
            fmt_o(median(&resid_med))
        ));
    }
    out.push(String::new());

    out.push("A. RMS floor: day-cell (mode,day, all stations pooled) — the rausch metric reference".to_string());
    for mode in [1i64, 2, 3] {
        let list: Vec<f64> = daycell
            .iter()
            .filter(|((m, _), _)| *m == mode)
            .filter_map(|((_, _), c)| c.rms())
            .filter(|r| r.is_finite())
            .collect();
        out.push(format!(
            "  mode {mode}: pooled day-cell RMS median {} Hz ({nd} day cells)",
            fmt_o(median(&list)),
            nd = list.len()
        ));
    }
    out.push(String::new());

    out.push("B. RMS floor: pass-level vs station-day-level, modes 1 and 2, stations 14/43/63".to_string());
    for mode in [1i64, 2] {
        for station in STATIONS {
            let samples = sm_samples.get(&(mode, station)).copied().unwrap_or(0);
            let days = sm_days.get(&(mode, station)).map(|d| d.len()).unwrap_or(0);
            let passes_all = passmap.get(&(mode, station)).map(|v| v.len()).unwrap_or(0);
            let day_rms: Vec<f64> = stationday
                .iter()
                .filter(|((m, s, _), _)| *m == mode && *s == station)
                .filter_map(|((_, _, _), c)| if c.n >= MIN_CELL { c.rms() } else { None })
                .filter(|r| r.is_finite())
                .collect();
            let pass_ge30: Vec<f64> = passmap
                .get(&(mode, station))
                .map(|v| {
                    v.iter()
                        .filter(|p| p.n_xs >= MIN_CELL)
                        .filter_map(|p| p.rms)
                        .collect()
                })
                .unwrap_or_default();
            let mut psorted = pass_ge30.clone();
            psorted.sort_by(f64::total_cmp);
            out.push(format!(
                "  mode {mode} station {station}: samples {samples}, days {days}, passes {passes_all}, passes_ge30 {}, station-day floor {} Hz ({} cells), pass floor {} Hz ({} passes, p10 {} / p90 {})",
                pass_ge30.len(),
                fmt_o(median(&day_rms)),
                day_rms.len(),
                fmt_o(median(&pass_ge30)),
                psorted.len(),
                fmt_o(pct(&psorted, 10)),
                fmt_o(pct(&psorted, 90))
            ));
        }
    }
    out.push(String::new());

    out.push("C. mode 1 pass floor by signal-strength quartile (fingerprint confirmation)".to_string());
    out.push(format!(
        "  strength quartile cuts: b1 = {}, b2 = {}, b3 = {}",
        cut_str(0),
        cut_str(1),
        cut_str(2)
    ));
    for station in STATIONS {
        for q in 1u8..=4 {
            let rms: Vec<f64> = passmap
                .get(&(1, station))
                .map(|v| {
                    v.iter()
                        .filter(|p| p.n_xs >= MIN_CELL)
                        .filter(|p| match p.med_strength {
                            Some(s) => q_of(s, &bounds) == q,
                            None => false,
                        })
                        .filter_map(|p| p.rms)
                        .collect()
                })
                .unwrap_or_default();
            out.push(format!(
                "  mode 1 station {station} Q{q}: pass floor {} Hz ({} passes)",
                fmt_o(median(&rms)),
                rms.len()
            ));
        }
    }
    out.push(String::new());

    if geometry_ok {
        out.push("D. geometry split of the day-cell floor (solar elongation eps at day start) — rausch curve reproduction".to_string());
        for mode in [1i64, 2] {
            let mut bands: BTreeMap<i64, Vec<f64>> = BTreeMap::new();
            let mut band_days: BTreeMap<i64, BTreeSet<i64>> = BTreeMap::new();
            for ((m, day), c) in &daycell {
                if *m != mode {
                    continue;
                }
                let Some(eps) = elong_deg_at(*day as f64 * DAY_S, &eph) else {
                    continue;
                };
                let band = (eps / 30.0).floor() as i64;
                if let Some(r) = c.rms() {
                    if r.is_finite() {
                        bands.entry(band).or_default().push(r);
                        band_days.entry(band).or_default().insert(*day);
                    }
                }
            }
            for (band, v) in &bands {
                out.push(format!(
                    "  mode {mode} eps {lo}-{hi} deg: pooled day-cell RMS {med} Hz ({n} days)",
                    lo = band * 30,
                    hi = (band + 1) * 30,
                    med = fmt_o(median(v)),
                    n = band_days.get(band).map(|d| d.len()).unwrap_or(0)
                ));
            }
        }
        out.push(String::new());

        out.push("E. pass-level floor by solar elongation window (pass mid-epoch), modes 1 and 2, stations 14/43/63".to_string());
        out.push("  windows: quiet = eps 0-30, loud = eps 150-180".to_string());
        for mode in [1i64, 2] {
            for station in STATIONS {
                let mut parts: Vec<String> = Vec::new();
                for (lo, hi, tag) in [(0.0f64, 30.0f64, "quiet"), (150.0, 180.0, "loud")] {
                    let pass_rms: Vec<f64> = passmap
                        .get(&(mode, station))
                        .map(|v| {
                            v.iter()
                                .filter(|p| p.n_xs >= MIN_CELL)
                                .filter(|p| {
                                    let tmid = p.t0 + p.dur_s * 0.5;
                                    match elong_deg_at(tmid, &eph) {
                                        Some(e) => e >= lo && e < hi,
                                        None => false,
                                    }
                                })
                                .filter_map(|p| p.rms)
                                .collect()
                        })
                        .unwrap_or_default();
                    let day_rms: Vec<f64> = stationday
                        .iter()
                        .filter(|((m, s, _), _)| *m == mode && *s == station)
                        .filter_map(|((_, _, day), c)| {
                            if c.n < MIN_CELL {
                                return None;
                            }
                            let eps = elong_deg_at(*day as f64 * DAY_S, &eph)?;
                            if eps >= lo && eps < hi {
                                c.rms()
                            } else {
                                None
                            }
                        })
                        .filter(|r| r.is_finite())
                        .collect();
                    parts.push(format!(
                        "{tag} eps {lo:.0}-{hi:.0}: pass floor {} Hz ({} passes), station-day floor {} Hz ({} cells)",
                        fmt_o(median(&pass_rms)),
                        pass_rms.len(),
                        fmt_o(median(&day_rms)),
                        day_rms.len()
                    ));
                }
                out.push(format!("  mode {mode} station {station}: {}", parts.join(" | ")));
            }
        }
        out.push(String::new());
    } else {
        out.push("D/E skipped — ephemeris bins absent (geometry unmeasured)".to_string());
        out.push(String::new());
    }

    out.push("registers".to_string());
    out.push("  per-pass median resid reported only on non-lock samples (|resid| <= 1000 Hz); floor lists require >= MIN_CELL non-lock samples".to_string());
    let first_day = recs.iter().map(|r| (r[0] / DAY_S).floor() as i64).min();
    let last_day = recs.iter().map(|r| (r[0] / DAY_S).floor() as i64).max();
    match (first_day, last_day) {
        (Some(a), Some(b)) => out.push(format!(
            "  span {} .. {}",
            jd_date(a as f64 * DAY_S),
            jd_date(b as f64 * DAY_S)
        )),
        _ => {}
    }

    let body = out.join("\n") + "\n";
    println!("{body}");
}
