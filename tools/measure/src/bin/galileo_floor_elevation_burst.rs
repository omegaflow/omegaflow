use std::collections::{BTreeMap, HashMap};

use omegaflow::archivar::{body_barycenter_position, parse_ephemeris_binary, BodyEphemeris};
use omegaflow::lsk::days_from_civil;
use omegaflow::odp::{dsn_station, EARTH};

const DAY_S: f64 = 86400.0;
const LOCK_HZ: f64 = 1.0e3;
const FLOOR_AGC: f64 = -2560.0;
const LOUD_HZ: f64 = 1.0;
const MIN_CELL: usize = 30;
const RUN_GAP_S: f64 = 600.0;
const EL_STEP_S: f64 = 90.0;
const N_PERM: usize = 9999;
const STATIONS: [i64; 3] = [14, 43, 63];
const MODES: [i64; 3] = [1, 2, 3];

fn norm(v: [f64; 3]) -> f64 {
    (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
}

fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn unix_day(tdb: f64) -> i64 {
    (2451545.0 + tdb / DAY_S - 2440587.5).round() as i64
}

fn tdb_day_lo(day: i64) -> f64 {
    (day as f64 - 10957.5) * DAY_S
}

fn elevation_at(t: f64, station: i64, eph: &HashMap<String, BodyEphemeris>) -> Option<f64> {
    let (lat_deg, lon_deg, _alt) = dsn_station(station)?;
    let p = body_barycenter_position("galileo_daily", t, eph)?;
    let e = body_barycenter_position(EARTH, t, eph)?;
    let v = sub(p, e);
    let r = norm(v);
    if r <= 0.0 || !r.is_finite() {
        return None;
    }
    let dec = (v[2] / r).clamp(-1.0, 1.0).asin();
    let ra = v[1].atan2(v[0]);
    let jd = t / DAY_S + 2451545.0;
    let gmst = (280.46061837 + 360.98564736629 * (jd - 2451545.0)).rem_euclid(360.0);
    let lst = (gmst + lon_deg).rem_euclid(360.0).to_radians();
    let ha = lst - ra;
    let phi = lat_deg.to_radians();
    let sin_el = phi.sin() * dec.sin() + phi.cos() * dec.cos() * ha.cos();
    Some(sin_el.clamp(-1.0, 1.0).asin().to_degrees())
}

#[derive(Clone, Copy)]
struct Acc {
    n: usize,
    sum: f64,
    sum2: f64,
}

impl Acc {
    fn rms(&self) -> f64 {
        let mean = self.sum / self.n as f64;
        let var = (self.sum2 / self.n as f64 - mean * mean).max(0.0);
        var.sqrt()
    }
}

#[derive(Clone, Copy)]
struct CellStat {
    station: i64,
    day: i64,
    rms: f64,
    loud: bool,
    el_mean: f64,
    ceil: f64,
}

fn sorted_copy(v: &[f64]) -> Vec<f64> {
    let mut s = v.to_vec();
    s.sort_by(f64::total_cmp);
    s
}

fn pct(sorted: &[f64], p: f64) -> f64 {
    if sorted.is_empty() {
        return f64::NAN;
    }
    let k = (((sorted.len() - 1) as f64) * p).round() as usize;
    sorted[k]
}

fn mean(v: &[f64]) -> f64 {
    v.iter().sum::<f64>() / v.len() as f64
}

fn spearman(x: &[f64], y: &[f64]) -> Option<f64> {
    let n = x.len();
    if n < 5 || n != y.len() {
        return None;
    }
    let mut xi: Vec<usize> = (0..n).collect();
    let mut yi: Vec<usize> = (0..n).collect();
    xi.sort_by(|a, b| x[*a].total_cmp(&x[*b]));
    yi.sort_by(|a, b| y[*a].total_cmp(&y[*b]));
    let mut rx = vec![0.0f64; n];
    let mut ry = vec![0.0f64; n];
    let mut i = 0usize;
    while i < n {
        let mut j = i + 1;
        while j < n && x[xi[j]] == x[xi[i]] {
            j += 1;
        }
        let avg = ((i + j - 1) as f64) / 2.0;
        for k in xi[i..j].iter() {
            rx[*k] = avg;
        }
        i = j;
    }
    i = 0;
    while i < n {
        let mut j = i + 1;
        while j < n && y[yi[j]] == y[yi[i]] {
            j += 1;
        }
        let avg = ((i + j - 1) as f64) / 2.0;
        for k in yi[i..j].iter() {
            ry[*k] = avg;
        }
        i = j;
    }
    let mx = mean(&rx);
    let my = mean(&ry);
    let mut num = 0.0;
    let mut dx2 = 0.0;
    let mut dy2 = 0.0;
    for k in 0..n {
        let a = rx[k] - mx;
        let b = ry[k] - my;
        num += a * b;
        dx2 += a * a;
        dy2 += b * b;
    }
    if dx2 <= 0.0 || dy2 <= 0.0 {
        return None;
    }
    Some(num / (dx2 * dy2).sqrt())
}

fn next_rng(rng: &mut u64) -> u64 {
    *rng = rng
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);
    *rng >> 33
}

fn perm_mean_diff_p(a: &[f64], b: &[f64], seed: u64) -> Option<(f64, f64, usize, usize)> {
    let (n1, n2) = (a.len(), b.len());
    if n1 == 0 || n2 == 0 {
        return None;
    }
    let mut v: Vec<f64> = Vec::with_capacity(n1 + n2);
    v.extend_from_slice(a);
    v.extend_from_slice(b);
    let obs = mean(&v[..n1]) - mean(&v[n1..]);
    let total = n1 + n2;
    let mut cnt = 0usize;
    let mut rng = seed;
    for _ in 0..N_PERM {
        for i in 0..n1 {
            let j = i + (next_rng(&mut rng) as usize) % (total - i);
            v.swap(i, j);
        }
        let m = mean(&v[..n1]) - mean(&v[n1..]);
        if m.abs() >= obs.abs() {
            cnt += 1;
        }
    }
    Some((obs, (cnt as f64 + 1.0) / (N_PERM as f64 + 1.0), n1, n2))
}

#[derive(Clone, Copy)]
struct Anchor {
    mode: i64,
    station: i64,
    day: i64,
    name: &'static str,
}

fn anchors() -> Vec<Anchor> {
    vec![
        Anchor {
            mode: 1,
            station: 14,
            day: days_from_civil(1995, 11, 24).unwrap(),
            name: "M1 st14 1995-11-24",
        },
        Anchor {
            mode: 2,
            station: 14,
            day: days_from_civil(1995, 11, 24).unwrap(),
            name: "M2 st14 1995-11-24",
        },
        Anchor {
            mode: 3,
            station: 14,
            day: days_from_civil(1995, 12, 5).unwrap(),
            name: "M3 st14 1995-12-05",
        },
        Anchor {
            mode: 3,
            station: 63,
            day: days_from_civil(1995, 11, 27).unwrap(),
            name: "M3 st63 1995-11-27",
        },
        Anchor {
            mode: 1,
            station: 63,
            day: days_from_civil(1996, 6, 26).unwrap(),
            name: "M1 st63 1996-06-26",
        },
        Anchor {
            mode: 1,
            station: 43,
            day: days_from_civil(1996, 11, 4).unwrap(),
            name: "M1 st43 1996-11-04",
        },
        Anchor {
            mode: 3,
            station: 43,
            day: days_from_civil(1995, 12, 4).unwrap(),
            name: "M3 st43 1995-12-04",
        },
    ]
}

fn dist_line(tag: &str, vals: &[f64]) -> String {
    if vals.is_empty() {
        return format!("{tag}: n 0");
    }
    let s = sorted_copy(vals);
    let n = s.len();
    format!(
        "{tag}: n {n} min {:.2} p25 {:.2} med {:.2} p75 {:.2} max {:.2}",
        s[0],
        pct(&s, 0.25),
        pct(&s, 0.5),
        pct(&s, 0.75),
        s[n - 1]
    )
}

fn run_splits(times: &[f64]) -> Vec<(usize, usize)> {
    let mut out: Vec<(usize, usize)> = Vec::new();
    if times.is_empty() {
        return out;
    }
    let mut s = 0usize;
    for i in 1..times.len() {
        if times[i] - times[i - 1] > RUN_GAP_S {
            out.push((s, i));
            s = i;
        }
    }
    out.push((s, times.len()));
    out
}

fn episodes(
    times: &[f64],
    resids: &[f64],
    thresh_hz: f64,
    gap_s: f64,
) -> Vec<(f64, f64, usize, f64)> {
    let mut out: Vec<(f64, f64, usize, f64)> = Vec::new();
    if times.len() != resids.len() || times.is_empty() {
        return out;
    }
    for (lo, hi) in run_splits(times) {
        let mut open: Option<(usize, usize, f64)> = None;
        for i in lo..hi {
            let loud = resids[i].abs() > thresh_hz;
            if loud {
                match open {
                    None => {
                        open = Some((i, i, resids[i].abs()));
                    }
                    Some((s, _, pk)) => {
                        open = Some((s, i, pk.max(resids[i].abs())));
                    }
                }
            } else if let Some((s, e, pk)) = open {
                if times[i] - times[e] > gap_s {
                    out.push((times[s], times[e], e - s + 1, pk));
                    open = None;
                }
            }
        }
        if let Some((s, e, pk)) = open {
            out.push((times[s], times[e], e - s + 1, pk));
        }
    }
    out
}

fn main() {
    let report_path = match std::env::args().nth(1) {
        Some(p) => p,
        None => "/tmp/opencode/galileo_floor_elevation_burst_report.txt".to_string(),
    };
    let mut eph: HashMap<String, BodyEphemeris> = HashMap::new();
    let mut complete = true;
    for b in ["galileo_daily", "earth"] {
        let p = format!("data/ssd.jpl.nasa.gov/ephemeris_{b}.bin");
        match std::fs::read(&p).ok().and_then(|d| parse_ephemeris_binary(&d)) {
            Some(e) => {
                eph.insert(b.to_string(), e);
            }
            None => {
                println!("galileo_floor_elevation_burst: ephemeris {b} bin absent");
                complete = false;
            }
        }
    }
    if !complete {
        return;
    }
    let bytes = match std::fs::read("data/pds-ppi.igpp.ucla.edu/galileo_resid.bin") {
        Ok(b) => b,
        Err(_) => {
            println!("galileo_floor_elevation_burst: resid bin absent");
            return;
        }
    };
    let Some(recs) = omegaflow::atdf::parse_resid_bin(&bytes) else {
        println!("galileo_floor_elevation_burst: resid bin parse absent");
        return;
    };
    drop(bytes);

    let mut out: Vec<String> = Vec::new();
    out.push("galileo floor elevation and burst-duration probe (Richtung E2)".to_string());
    out.push("binding: floor sample = signal_strength exactly -2560 (AGC clamp), station 14/43/63, ground mode 1..3, |resid| <= 1000 Hz (lock cut separated).".to_string());
    out.push("day cell = floor samples of one TDB day (round-jd register convention); loud = cell RMS about the cell mean >= 1 Hz; robust cell = n >= 30 samples.".to_string());
    out.push("elevation proxy = textbook-GMST topocentric elevation of the probe above the station horizon (dsn_station geodetics; IAU 1982 GMST, tdb~UT1, equinox drift <= ~0.5 deg); probe direction = galileo_daily - earth barycenter.".to_string());

    let mut mode_acc: BTreeMap<(i64, i64, i64), Acc> = BTreeMap::new();
    let mut day_acc: BTreeMap<(i64, i64), Acc> = BTreeMap::new();
    let mut day_samp: BTreeMap<(i64, i64), Vec<(f64, f64)>> = BTreeMap::new();
    let mut n_floor = 0usize;
    let mut n_lock = 0usize;
    let mut n_nonfinite = 0usize;
    for r in &recs {
        let resid = r[1];
        if !resid.is_finite() {
            n_nonfinite += 1;
            continue;
        }
        if resid.abs() > LOCK_HZ {
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
        n_floor += 1;
        let day = unix_day(r[0]);
        let ma = mode_acc
            .entry((st, mo, day))
            .or_insert(Acc {
                n: 0,
                sum: 0.0,
                sum2: 0.0,
            });
        ma.n += 1;
        ma.sum += resid;
        ma.sum2 += resid * resid;
        let da = day_acc
            .entry((st, day))
            .or_insert(Acc {
                n: 0,
                sum: 0.0,
                sum2: 0.0,
            });
        da.n += 1;
        da.sum += resid;
        da.sum2 += resid * resid;
        day_samp.entry((st, day)).or_default().push((r[0], resid));
    }
    drop(recs);
    out.push(format!(
        "resid.bin: {n_floor} floor samples at st14/43/63 mode1..3 non-lock; {n_lock} lock-cut samples excluded; {n_nonfinite} non-finite excluded"
    ));

    out.push(String::new());
    out.push("== cell grid (register reconciliation) ==".to_string());
    let mut mode_tot = 0usize;
    let mut mode_loud = 0usize;
    let mut mode_quiet = 0usize;
    out.push("per (mode, station): robust day cells n>=30 | loud / quiet".to_string());
    for &st in &STATIONS {
        for &mo in &MODES {
            let mut loud = 0usize;
            let mut quiet = 0usize;
            for ((s, m, _), a) in &mode_acc {
                if *s == st && *m == mo && a.n >= MIN_CELL {
                    if a.rms() >= LOUD_HZ {
                        loud += 1;
                    } else {
                        quiet += 1;
                    }
                }
            }
            mode_tot += loud + quiet;
            mode_loud += loud;
            mode_quiet += quiet;
            out.push(format!(
                "  mode {mo} st{st}: robust {} (loud {loud}, quiet {quiet})",
                loud + quiet
            ));
        }
    }
    out.push(format!(
        "  total (mode, station, day): robust {mode_tot} | loud {mode_loud} | quiet {mode_quiet}"
    ));

    let mut day_tot = 0usize;
    let mut day_loud = 0usize;
    let mut day_quiet = 0usize;
    out.push("per station, merged (station, day) cells (all floor modes combined): robust n>=30 | loud / quiet".to_string());
    for &st in &STATIONS {
        let mut loud = 0usize;
        let mut quiet = 0usize;
        for ((s, _), a) in &day_acc {
            if *s == st && a.n >= MIN_CELL {
                if a.rms() >= LOUD_HZ {
                    loud += 1;
                } else {
                    quiet += 1;
                }
            }
        }
        day_tot += loud + quiet;
        day_loud += loud;
        day_quiet += quiet;
        out.push(format!(
            "  st{st}: robust {} (loud {loud}, quiet {quiet})",
            loud + quiet
        ));
    }
    out.push(format!(
        "  total (station, day): robust {day_tot} | loud {day_loud} | quiet {day_quiet}"
    ));

    out.push(String::new());
    out.push("== per-cell elevation build (merged robust (station, day) cells) ==".to_string());
    let mut cells: Vec<CellStat> = Vec::new();
    let mut n_cell_el_void = 0usize;
    for &st in &STATIONS {
        for ((s, day), a) in &day_acc {
            if *s != st || a.n < MIN_CELL {
                continue;
            }
            let rms = a.rms();
            let Some(samp) = day_samp.get(&(st, *day)) else {
                continue;
            };
            let mut els: Vec<f64> = Vec::new();
            for (t, _) in samp {
                if let Some(el) = elevation_at(*t, st, &eph) {
                    els.push(el);
                }
            }
            if els.is_empty() {
                n_cell_el_void += 1;
                continue;
            }
            let el_mean = mean(&els);
            let lo = tdb_day_lo(*day) - 0.5 * DAY_S;
            let hi = tdb_day_lo(*day) + 0.5 * DAY_S;
            let mut ceil = f64::NEG_INFINITY;
            let mut t = lo;
            while t < hi {
                if let Some(el) = elevation_at(t, st, &eph) {
                    if el > ceil {
                        ceil = el;
                    }
                }
                t += EL_STEP_S;
            }
            cells.push(CellStat {
                station: st,
                day: *day,
                rms,
                loud: rms >= LOUD_HZ,
                el_mean,
                ceil,
            });
        }
    }
    out.push(format!(
        "merged robust cells with elevation: {n} (void {n_cell_el_void}, 0 honored)",
        n = cells.len()
    ));

    out.push(String::new());
    out.push("== Measurement 1 — elevation of loud vs quiet (station, day) cells ==".to_string());
    out.push("ceil = max topocentric elevation over the day interval at the station (90-s grid); el_mean = mean elevation over the cell samples.".to_string());
    for &st in &STATIONS {
        let loud: Vec<&CellStat> = cells.iter().filter(|c| c.station == st && c.loud).collect();
        let quiet: Vec<&CellStat> = cells.iter().filter(|c| c.station == st && !c.loud).collect();
        let lc: Vec<f64> = loud.iter().map(|c| c.ceil).collect();
        let qc: Vec<f64> = quiet.iter().map(|c| c.ceil).collect();
        let le: Vec<f64> = loud.iter().map(|c| c.el_mean).collect();
        let qe: Vec<f64> = quiet.iter().map(|c| c.el_mean).collect();
        let seed = 0x9E37_79B9_7F4A_7C15u64 ^ (st as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15);
        let perm = perm_mean_diff_p(&lc, &qc, seed);
        out.push(format!("-- st{st} (merged station-day cells) --"));
        out.push(format!("  loud  cells {}: {}", lc.len(), dist_line("ceil deg", &lc)));
        out.push(format!("  quiet cells {}: {}", qc.len(), dist_line("ceil deg", &qc)));
        out.push(format!("  loud  cells {}: {}", le.len(), dist_line("el_mean deg", &le)));
        out.push(format!("  quiet cells {}: {}", qe.len(), dist_line("el_mean deg", &qe)));
        if let Some((obs, p, n1, n2)) = perm {
            out.push(format!(
                "  permutation diff-of-means ceil (loud - quiet) {obs:+.2} deg, two-sided p {p:.4}, n {n1}/{n2}"
            ));
        }
        for th in [10.0f64, 20.0, 30.0] {
            let nl = lc.iter().filter(|v| **v < th).count();
            let nq = qc.iter().filter(|v| **v < th).count();
            out.push(format!(
                "  cells with ceil < {th:.0} deg: loud {nl}/{n} quiet {nq}/{m}",
                n = lc.len(),
                m = qc.len()
            ));
        }
        let lr: Vec<f64> = cells
            .iter()
            .filter(|c| c.station == st && c.rms > 0.0)
            .map(|c| c.rms.log10())
            .collect();
        let cr: Vec<f64> = cells
            .iter()
            .filter(|c| c.station == st && c.rms > 0.0)
            .map(|c| c.ceil)
            .collect();
        if let Some(rho) = spearman(&lr, &cr) {
            out.push(format!(
                "  spearman(log10 cell-rms, ceil) over {} robust cells: {rho:+.2}",
                lr.len()
            ));
        }
        let mut loud_els: Vec<f64> = Vec::new();
        let mut all_els: Vec<f64> = Vec::new();
        let mut ns = 0usize;
        for c in cells.iter().filter(|c| c.station == st && c.loud) {
            let Some(samp) = day_samp.get(&(st, c.day)) else {
                continue;
            };
            for (t, r) in samp {
                if let Some(el) = elevation_at(*t, st, &eph) {
                    all_els.push(el);
                    if r.abs() > LOUD_HZ {
                        loud_els.push(el);
                        ns += 1;
                    }
                }
            }
        }
        out.push(format!(
            "  within loud days: loud-sample (|resid| > 1 Hz) n {ns}: {}",
            dist_line("el deg", &loud_els)
        ));
        out.push(format!(
            "  within loud days: all-sample {}",
            dist_line("el deg", &all_els)
        ));
        if !loud_els.is_empty() && !all_els.is_empty() {
            let ls = sorted_copy(&loud_els);
            let as_ = sorted_copy(&all_els);
            let nlow_l = ls.iter().filter(|v| **v < 5.0).count();
            let nlow_a = as_.iter().filter(|v| **v < 5.0).count();
            out.push(format!(
                "  loud samples below 5 deg: {nlow_l}/{}; all samples below 5 deg: {nlow_a}/{}",
                ls.len(),
                as_.len()
            ));
        }
    }

    out.push(String::new());
    out.push(String::new());
    out.push("== within loud days - elevation of loud samples vs same-day quiet samples ==".to_string());
    for &st in &STATIONS {
        let mut deltas: Vec<f64> = Vec::new();
        let mut n_lower = 0usize;
        let mut n_upper = 0usize;
        let mut n_cell = 0usize;
        for c in cells.iter().filter(|c| c.station == st && c.loud) {
            let Some(samp) = day_samp.get(&(st, c.day)) else {
                continue;
            };
            let mut el_l: Vec<f64> = Vec::new();
            let mut el_q: Vec<f64> = Vec::new();
            for (t, r) in samp {
                if let Some(el) = elevation_at(*t, st, &eph) {
                    if r.abs() > LOUD_HZ {
                        el_l.push(el);
                    } else {
                        el_q.push(el);
                    }
                }
            }
            if el_l.is_empty() || el_q.is_empty() {
                continue;
            }
            let ml = pct(&sorted_copy(&el_l), 0.5);
            let mq = pct(&sorted_copy(&el_q), 0.5);
            let d = ml - mq;
            deltas.push(d);
            if d < 0.0 {
                n_lower += 1;
            } else if d > 0.0 {
                n_upper += 1;
            }
            n_cell += 1;
        }
        out.push(format!(
            "  st{st}: {n_cell} loud cells with both classes; loud-sample median el below same-day quiet median {n_lower}, above {n_upper}: {}",
            dist_line("med_loud - med_quiet deg", &deltas)
        ));
    }

    out.push("== day-neighbor loud/quiet flips at matched geometry (merged station-day cells) ==".to_string());
    for &st in &STATIONS {
        let mut dc: Vec<&CellStat> = cells.iter().filter(|c| c.station == st).collect();
        dc.sort_by(|a, b| a.day.cmp(&b.day));
        let mut n_flip = 0usize;
        let mut dceils: Vec<f64> = Vec::new();
        let mut near = 0usize;
        let mut loud_higher = 0usize;
        for w in dc.windows(2) {
            if w[1].day - w[0].day != 1 {
                continue;
            }
            if w[0].loud == w[1].loud {
                continue;
            }
            n_flip += 1;
            let (l, q) = if w[0].loud { (w[0], w[1]) } else { (w[1], w[0]) };
            let d = l.ceil - q.ceil;
            dceils.push(d.abs());
            if d.abs() <= 2.0 {
                near += 1;
            }
            if d > 0.0 {
                loud_higher += 1;
            }
        }
        if n_flip > 0 {
            let sd = sorted_copy(&dceils);
            out.push(format!(
                "  st{st}: {n_flip} day-neighbor flips; median |ceil_delta| {:.2} deg; |delta| <= 2 deg {near}; loud day higher ceiling {loud_higher}",
                pct(&sd, 0.5)
            ));
        } else {
            out.push(format!("  st{st}: no day-neighbor loud/quiet flip (0 honored)"));
        }
    }

    out.push(String::new());
    out.push("== Measurement 2 — burst episode durations on loud (station, day) cells ==".to_string());
    out.push("loud sample = |resid| > threshold; episode = connected above-threshold samples with inter-sample gap <= gap_s inside a continuous run (> 600 s splits).".to_string());
    out.push("episode duration = time span last - first sample; single-sample episodes carry span 0 s (no temporal extent on the sampled lattice).".to_string());
    for (th, gap) in [(10.0f64, 30.0f64), (1.0, 60.0), (100.0, 2.0)] {
        let mut all_dur: Vec<f64> = Vec::new();
        let mut n_single = 0usize;
        let mut n_ep = 0usize;
        let mut cell_count = 0usize;
        for c in cells.iter().filter(|c| c.loud) {
            let Some(samp) = day_samp.get(&(c.station, c.day)) else {
                continue;
            };
            let mut srt = samp.clone();
            srt.sort_by(|a, b| a.0.total_cmp(&b.0));
            let ts: Vec<f64> = srt.iter().map(|x| x.0).collect();
            let rs: Vec<f64> = srt.iter().map(|x| x.1).collect();
            let eps = episodes(&ts, &rs, th, gap);
            if eps.is_empty() {
                continue;
            }
            cell_count += 1;
            for (t0, t1, n, _) in &eps {
                n_ep += 1;
                if *n == 1 {
                    n_single += 1;
                } else {
                    all_dur.push(t1 - t0);
                }
            }
        }
        out.push(format!(
            "T {th:.0} Hz, gap {gap:.0} s: {cell_count} loud cells, {n_ep} episodes ({n_single} single-sample), {}",
            dist_line("dur s (span>0)", &all_dur)
        ));
        let bins: [(f64, f64); 6] = [
            (0.0, 2.0),
            (2.0, 10.0),
            (10.0, 60.0),
            (60.0, 600.0),
            (600.0, 3600.0),
            (3600.0, f64::INFINITY),
        ];
        let mut counts = [0usize; 6];
        for d in &all_dur {
            for (k, (blo, bhi)) in bins.iter().enumerate() {
                if *d >= *blo && *d < *bhi {
                    counts[k] += 1;
                    break;
                }
            }
        }
        out.push(format!(
            "  span histogram: [0-2s) {} [2-10s) {} [10-60s) {} [60-600s) {} [600-3600s) {} [>=3600s] {}",
            counts[0], counts[1], counts[2], counts[3], counts[4], counts[5]
        ));
        let sd = sorted_copy(&all_dur);
        if !sd.is_empty() {
            out.push(format!(
                "  span>0 episodes {}, median {:.1} s, p90 {:.1} s, p99 {:.1} s, max {:.1} s",
                sd.len(),
                pct(&sd, 0.5),
                pct(&sd, 0.9),
                pct(&sd, 0.99),
                sd[sd.len() - 1]
            ));
        }
    }

    out.push(String::new());
    out.push("== per anchor loud pass — burst episodes (T 10 Hz, gap 30 s) ==".to_string());
    for a in anchors() {
        let day_cell = cells
            .iter()
            .find(|c| c.station == a.station && c.day == a.day)
            .copied();
        let Some(samp) = day_samp.get(&(a.station, a.day)) else {
            out.push(format!("{} — station-day cell absent (0 honored)", a.name));
            continue;
        };
        match day_cell {
            Some(c) if c.loud => {
                let mut srt = samp.clone();
                srt.sort_by(|x, y| x.0.total_cmp(&y.0));
                let ts: Vec<f64> = srt.iter().map(|x| x.0).collect();
                let rs: Vec<f64> = srt.iter().map(|x| x.1).collect();
                let eps = episodes(&ts, &rs, 10.0, 30.0);
                let mut spans: Vec<f64> = Vec::new();
                let mut singles = 0usize;
                let mut tot_s = 0.0f64;
                for (t0, t1, n, _) in &eps {
                    if *n == 1 {
                        singles += 1;
                    } else {
                        spans.push(t1 - t0);
                        tot_s += t1 - t0;
                    }
                }
                out.push(format!(
                    "{} mode {} st{}: rms {:.2} Hz ceil {:.1} deg | episodes n {} (single {singles}) {} total span {tot_s:.0} s",
                    a.name,
                    a.mode,
                    a.station,
                    c.rms,
                    c.ceil,
                    eps.len(),
                    dist_line("span s", &spans),
                ));
            }
            Some(_) => {
                out.push(format!(
                    "{} — merged station-day cell not loud (0 honored)",
                    a.name
                ));
            }
            None => {
                out.push(format!(
                    "{} — merged station-day cell absent (0 honored)",
                    a.name
                ));
            }
        }
    }

    let text = out.join("\n");
    println!("{text}");
    let _ = std::fs::write(&report_path, text);
}
