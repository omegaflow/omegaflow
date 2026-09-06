use std::collections::BTreeMap;

use omegaflow::archivar::spectral::civil_from_days;
use omegaflow::atdf::parse_resid_bin;
use omegaflow::odf::parse_p11r_bin;

const DAY_S: f64 = 86400.0;
const LOCK_HZ: f64 = 1.0e3;
const LOUD_HZ: f64 = 1.0;
const QUIET_HZ: f64 = 5.0;
const TRIO: [i64; 3] = [14, 43, 63];
const FLOOR: i64 = -2560;
const MIN_CELL: usize = 30;
const GAP_PASS_S: f64 = 900.0;
const MIN_BLOCK: usize = 8;

fn days_from_civil(y: i64, m: i64, d: i64) -> i64 {
    let yy = if m <= 2 { y - 1 } else { y };
    let era = if yy >= 0 { yy } else { yy - 399 } / 400;
    let yoe = yy - era * 400;
    let mp = (m + 9) % 12;
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe - 719468
}

fn daycell_of(y: i64, m: i64, d: i64) -> i64 {
    days_from_civil(y, m, d) - 10958
}

fn date_of(dc: i64) -> String {
    match civil_from_days(dc + 10958) {
        Some((y, m, d)) => format!("{y:04}-{m:02}-{d:02}"),
        None => format!("daycell {dc}"),
    }
}

fn jd_date(tdb: f64) -> String {
    date_of((tdb / DAY_S).floor() as i64)
}

fn median(v: &[f64]) -> f64 {
    if v.is_empty() {
        return f64::NAN;
    }
    let mut s = v.to_vec();
    s.sort_by(f64::total_cmp);
    s[s.len() / 2]
}

fn rms_about_mean(v: &[f64]) -> f64 {
    if v.is_empty() {
        return f64::NAN;
    }
    let m = v.iter().sum::<f64>() / v.len() as f64;
    (v.iter().map(|x| (x - m) * (x - m)).sum::<f64>() / v.len() as f64).sqrt()
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
    let mx = rx.iter().sum::<f64>() / n as f64;
    let my = ry.iter().sum::<f64>() / n as f64;
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
    if dx2 > 0.0 && dy2 > 0.0 {
        Some(num / (dx2 * dy2).sqrt())
    } else {
        None
    }
}

#[derive(Clone)]
struct GC {
    mode: i64,
    day: i64,
    st: i64,
    n: usize,
    rms: f64,
    loud: bool,
    thin: bool,
}

#[derive(Clone)]
struct PC {
    rx: i64,
    day: i64,
    n: usize,
    med: f64,
    rms: f64,
}

fn quad_detrend_cells(
    ts: &[f64],
    vs: &[f64],
    rx: &[i64],
) -> (Vec<f64>, Vec<f64>, Vec<i64>) {
    let mut dts = Vec::new();
    let mut dvs = Vec::new();
    let mut drx = Vec::new();
    let mut lo = 0usize;
    while lo < ts.len() {
        let mut hi = lo + 1;
        while hi < ts.len() && ts[hi] - ts[hi - 1] <= GAP_PASS_S {
            hi += 1;
        }
        if hi - lo >= MIN_BLOCK {
            let xs = &ts[lo..hi];
            let ys = &vs[lo..hi];
            let tx = xs.iter().sum::<f64>() / xs.len() as f64;
            let m0 = xs.len() as f64;
            let sx = xs.iter().map(|x| x - tx).sum::<f64>();
            let sy = ys.iter().sum::<f64>();
            let sxx = xs.iter().map(|x| (x - tx) * (x - tx)).sum::<f64>();
            let sxxx = xs.iter().map(|x| (x - tx).powi(3)).sum::<f64>();
            let sxxxx = xs.iter().map(|x| (x - tx).powi(4)).sum::<f64>();
            let sxy = xs.iter().zip(ys).map(|(x, y)| (x - tx) * y).sum::<f64>();
            let sxxy = xs
                .iter()
                .zip(ys)
                .map(|(x, y)| (x - tx) * (x - tx) * y)
                .sum::<f64>();
            let det = m0 * (sxx * sxxxx - sxxx * sxxx) - sx * (sx * sxxxx - sxx * sxxx)
                + sxx * (sx * sxxx - sxx * sxx);
            if det.abs() > 1e-300 {
                let cc = (sy * (sxx * sxxxx - sxxx * sxxx) - sx * (sxy * sxxxx - sxxx * sxxy)
                    + sxx * (sxy * sxxx - sxx * sxxy))
                    / det;
                let cb = (m0 * (sxy * sxxxx - sxxx * sxxy) - sy * (sx * sxxxx - sxx * sxxx)
                    + sxx * (sx * sxxy - sxy * sxx))
                    / det;
                let ca = (m0 * (sxx * sxxy - sxy * sxxx) - sx * (sx * sxxy - sxy * sxx)
                    + sy * (sx * sxxx - sxx * sxx))
                    / det;
                for k in lo..hi {
                    let dx = xs[k - lo] - tx;
                    dts.push(xs[k - lo]);
                    dvs.push(ys[k - lo] - (ca * dx * dx + cb * dx + cc));
                    drx.push(rx[k]);
                }
            }
        }
        lo = hi;
    }
    (dts, dvs, drx)
}

fn pioneer_floor_cells(
    probe: &str,
    cells: &mut Vec<PC>,
    census: &mut (usize, usize, usize),
) -> Option<(usize, f64, f64, Vec<(i64, usize, usize)>)> {
    let path = format!("data/spdf.gsfc.nasa.gov/{probe}_navio_residuum.bin");
    let bytes = std::fs::read(&path).ok()?;
    let recs = parse_p11r_bin(&bytes)?;
    let n_rec = recs.len();
    let mut ts = Vec::with_capacity(n_rec);
    let mut vs = Vec::with_capacity(n_rec);
    let mut rs = Vec::with_capacity(n_rec);
    let mut t0 = f64::INFINITY;
    let mut t1 = f64::NEG_INFINITY;
    let mut n_lock = 0usize;
    let mut n_nonfin = 0usize;
    for r in &recs {
        let t = r[0];
        let resid = r[1];
        let rx = r[5] as i64;
        t0 = t0.min(t);
        t1 = t1.max(t);
        if !resid.is_finite() {
            n_nonfin += 1;
            continue;
        }
        if resid.abs() > LOCK_HZ {
            n_lock += 1;
            continue;
        }
        ts.push(t);
        vs.push(resid);
        rs.push(rx);
    }
    let (dts, dvs, drs) = quad_detrend_cells(&ts, &vs, &rs);
    let mut by_rx: BTreeMap<i64, Vec<f64>> = BTreeMap::new();
    for (i, &rx) in drs.iter().enumerate() {
        by_rx.entry(rx).or_default().push(dvs[i]);
    }
    let mut off: BTreeMap<i64, f64> = BTreeMap::new();
    for (rx, v) in &by_rx {
        off.insert(*rx, median(v));
    }
    let n_det = dts.len();
    census.0 += n_lock;
    census.1 += n_nonfin;
    census.2 += n_rec;
    let mut daymap: BTreeMap<(i64, i64), Vec<f64>> = BTreeMap::new();
    for i in 0..n_det {
        let centered = dvs[i] - off[&drs[i]];
        let day = (dts[i] / DAY_S).floor() as i64;
        daymap.entry((drs[i], day)).or_default().push(centered);
    }
    let mut st_tot: BTreeMap<i64, (usize, usize)> = BTreeMap::new();
    for ((rx, day), v) in &daymap {
        cells.push(PC {
            rx: *rx,
            day: *day,
            n: v.len(),
            med: median(v),
            rms: rms_about_mean(v),
        });
        let e = st_tot.entry(*rx).or_insert((0, 0));
        e.0 += v.len();
        e.1 += 1;
    }
    let mut stv: Vec<(i64, usize, usize)> = st_tot
        .iter()
        .map(|(rx, (n, nd))| (*rx, *n, *nd))
        .collect();
    stv.sort_by_key(|x| x.0);
    Some((n_det, t0, t1, stv))
}

fn fmt3(v: f64) -> String {
    format!("{v:.3}")
}


fn pioneer_at<'a>(
    p: &'a BTreeMap<i64, Vec<(String, i64, usize, f64, f64)>>,
    st: i64,
    day: i64,
) -> Vec<&'a (String, i64, usize, f64, f64)> {
    match p.get(&st) {
        Some(v) => v.iter().filter(|x| x.1 == day).collect(),
        None => Vec::new(),
    }
}
fn main() {
    let report_path = match std::env::args().skip(1).find(|a| !a.starts_with('-')) {
        Some(p) => p,
        None => "/tmp/opencode/galileo_pioneer_stationsfloor_kreuz_report.txt".to_string(),
    };

    let w0 = daycell_of(1995, 11, 23);
    let w1 = daycell_of(1997, 2, 28);

    let mut out: Vec<String> = Vec::new();
    let mut push = |s: String| {
        println!("{s}");
        out.push(s);
    };

    push(format!(
        "galileo-pioneer stationsfloor cross probe — window {} .. {} (daycell {w0}..{w1})",
        date_of(w0),
        date_of(w1)
    ));
    push(format!(
        "Galileo floor metric: cell = (mode, station, TDB day), strength == {FLOOR} (AGC clamp), lock |resid| > {LOCK_HZ:.0} Hz excluded before noise; loud = cell RMS >= {LOUD_HZ} Hz; robust cells n >= {MIN_CELL}"
    ));
    push(format!(
        "Pioneer floor metric (negative-fuzzy pipeline of the zone-daily fields): per-pass quadratic detrend (gap {GAP_PASS_S:.0} s, block >= {MIN_BLOCK}), per-station median removal, then cell = (station, TDB day); lock |resid| > {LOCK_HZ:.0} Hz excluded; quiet band |daily median| <= {QUIET_HZ} Hz (Ded-45)"
    ));

    push(String::new());
    push("== S1 Pioneer NAVIO residuum census (negative-fuzzy floor cells) ==".to_string());
    let mut pcells: BTreeMap<String, Vec<PC>> = BTreeMap::new();
    for probe in ["pioneer10", "pioneer11"] {
        let mut cells = Vec::new();
        let mut census = (0usize, 0usize, 0usize);
        let Some((n_det, t0, t1, stv)) = pioneer_floor_cells(probe, &mut cells, &mut census) else {
            push(format!("{probe}: residuum bin or parse void — 0 honored"));
            continue;
        };
        let stl: Vec<String> = stv
            .iter()
            .map(|(rx, n, nd)| format!("st{rx} n {n} ({nd} d)"))
            .collect();
        push(format!(
            "{probe}: {n_det} of {} residual samples survive the per-pass quad detrend ({} lock-excluded, {} non-finite before it); span {} .. {}; stations: {}",
            census.2,
            census.0,
            census.1,
            jd_date(t0),
            jd_date(t1),
            stl.join(", ")
        ));
        for c in &cells {
            if c.day >= w0 && c.day <= w1 {
                pcells.entry(probe.to_string()).or_default().push(c.clone());
            }
        }
    }

    let mut pdiary: BTreeMap<i64, Vec<(String, i64, usize, f64, f64)>> = BTreeMap::new();
    for (probe, v) in &pcells {
        let mut n_win = 0usize;
        for c in v {
            if c.rx != 14 && c.rx != 43 && c.rx != 63 {
                continue;
            }
            n_win += 1;
            pdiary.entry(c.rx).or_default().push((probe.clone(), c.day, c.n, c.med, c.rms));
        }
        let _ = n_win;
    }
    for st in TRIO {
        if let Some(e) = pdiary.get_mut(&st) {
            e.sort_by_key(|x| (x.1, x.0.clone()));
        }
    }

    push(String::new());
    push("== S1b Pioneer window (station, day) cells at the 70-m trio ==".to_string());
    for st in TRIO {
        if let Some(v) = pdiary.get(&st) {
            for (probe, day, n, med, r) in v {
                let mut band = if med.abs() > QUIET_HZ { "LOUD" } else { "quiet" };
                if *n < MIN_CELL {
                    band = "thin";
                }
                push(format!(
                    "  {probe} st{st} {} n {n:<6} day-med {} Hz [{}] day-RMS {} Hz",
                    date_of(*day),
                    fmt3(*med),
                    band,
                    fmt3(*r)
                ));
            }
        } else {
            push(format!("  st{st}: no Pioneer window cells (0 honored)"));
        }
    }
    push("  Pioneer trio window aggregate (per station: cells, robust n>=30, loud |med|>5 Hz, quiet, thin):".to_string());
    for st in TRIO {
        if let Some(v) = pdiary.get(&st) {
            let tot = v.len();
            let robust: Vec<&(String, i64, usize, f64, f64)> =
                v.iter().filter(|x| x.2 >= MIN_CELL).collect();
            let loud = robust.iter().filter(|x| x.3.abs() > QUIET_HZ).count();
            let quiet = robust.len() - loud;
            let thin = tot - robust.len();
            push(format!(
                "  st{st}: {tot} cells, {rob} robust ({loud} loud, {quiet} quiet), {thin} thin",
                rob = robust.len()
            ));
        }
    }

    push(String::new());
    push("== S2 Galileo floor loud-day reproduction (window) ==".to_string());

    push(String::new());
    push("== S2 Galileo floor loud-day reproduction (window) ==".to_string());
    let Ok(bytes) = std::fs::read("data/pds-ppi.igpp.ucla.edu/galileo_resid.bin") else {
        push("galileo: resid bin void — 0 honored".to_string());
        return;
    };
    let Some(recs) = parse_resid_bin(&bytes) else {
        push("galileo: resid parse void — 0 honored".to_string());
        return;
    };
    drop(bytes);
    let mut cell: BTreeMap<(i64, i64, i64), (f64, f64, usize)> = BTreeMap::new();
    let mut n_lock = 0usize;
    let mut n_mode = 0usize;
    let mut n_nonfin = 0usize;
    for r in &recs {
        let mode = r[3] as i64;
        if mode < 1 || mode > 3 {
            n_mode += 1;
            continue;
        }
        let resid = r[1];
        if !resid.is_finite() {
            n_nonfin += 1;
            continue;
        }
        if resid.abs() > LOCK_HZ {
            n_lock += 1;
            continue;
        }
        let s = r[7] as i64;
        if s != FLOOR {
            continue;
        }
        let st = r[2] as i64;
        let day = (r[0] / DAY_S).floor() as i64;
        let e = cell.entry((mode, st, day)).or_insert((0.0, 0.0, 0));
        e.0 += resid;
        e.1 += resid * resid;
        e.2 += 1;
    }
    drop(recs);
    push(format!(
        "galileo resid: floor cells (mode 1..3, any station, strength {FLOOR}); excluded lock {n_lock}, non-mode {n_mode}, non-finite {n_nonfin}"
    ));

    let mut gcells: Vec<GC> = Vec::new();
    for (&(mode, st, day), &(sum, sum2, n)) in &cell {
        if n == 0 || !TRIO.contains(&st) || day < w0 || day > w1 {
            continue;
        }
        let m = sum / n as f64;
        let v = (sum2 / n as f64 - m * m).max(0.0);
        let rms = v.sqrt();
        gcells.push(GC {
            mode,
            day,
            st,
            n,
            rms,
            loud: rms >= LOUD_HZ,
            thin: n < MIN_CELL,
        });
    }
    gcells.sort_by_key(|c| (c.day, c.st, c.mode));

    let mut loud_sum: BTreeMap<(i64, i64), (usize, usize)> = BTreeMap::new();
    for c in &gcells {
        let e = loud_sum.entry((c.st, c.mode)).or_insert((0, 0));
        e.0 += 1;
        if c.loud {
            e.1 += 1;
        }
    }
    push("window floor cells (station 14/43/63, modes 1..3), loud = RMS >= 1 Hz:".to_string());
    for st in TRIO {
        for mode in [1i64, 2, 3] {
            if let Some((nc, nl)) = loud_sum.get(&(st, mode)) {
                push(format!("  mode {mode} st{st}: {nc} cells, {nl} loud"));
            }
        }
    }

    push("chronological window floor rows (per day+station, cells of each mode):".to_string());
    let mut st_day: BTreeMap<(i64, i64), Vec<String>> = BTreeMap::new();
    for c in &gcells {
        st_day
            .entry((c.day, c.st))
            .or_default()
            .push(format!(
                "m{} {} Hz (n{}, {})",
                c.mode,
                fmt3(c.rms),
                c.n,
                if c.thin { "thin" } else if c.loud { "LOUD" } else { "quiet" }
            ));
    }
    let mut all_keys: Vec<(i64, i64)> = st_day.keys().copied().collect();
    all_keys.sort();
    push(format!(
        "Galileo floor (station, day) cells in window (any n, any mode, trio stations): {} distinct",
        all_keys.len()
    ));
    for (day, st) in &all_keys {
        push(format!(
            "  {} st{st}: {}",
            date_of(*day),
            st_day.get(&(*day, *st)).unwrap().join(" | ")
        ));
    }

    push(String::new());
    push("== S3 joint same-(station, day) cells in window ==".to_string());
    let mut joint_cells = 0usize;
    let mut joint_loud_both = 0usize;
    let mut g_rms_joint: Vec<f64> = Vec::new();
    let mut p_rms_joint: Vec<f64> = Vec::new();
    for (day, st) in &all_keys {
        let pv: Vec<&(String, i64, usize, f64, f64)> =
            pioneer_at(&pdiary, *st, *day);
        if pv.is_empty() {
            continue;
        }
        joint_cells += 1;
        let gld: Vec<&GC> = gcells.iter().filter(|c| c.day == *day && c.st == *st).collect();
        let any_g_loud = gld.iter().any(|c| c.loud && !c.thin);
        let any_p_loud = pv.iter().any(|x| x.2 >= MIN_CELL && x.3.abs() > QUIET_HZ);
        if any_g_loud && any_p_loud {
            joint_loud_both += 1;
        }
        let gs: Vec<String> = gld.iter().map(|c| {
            format!(
                "m{} {} n{} {}",
                c.mode,
                fmt3(c.rms),
                c.n,
                if c.thin { "thin" } else if c.loud { "LOUD" } else { "quiet" }
            )
        }).collect();
        let ps: Vec<String> = pv.iter().map(|x| {
            let band = if x.2 < MIN_CELL {
                "thin".to_string()
            } else if x.3.abs() > QUIET_HZ {
                "LOUD".to_string()
            } else {
                "quiet".to_string()
            };
            format!(
                "{} n{} med{} {} rms{}",
                x.0,
                x.2,
                fmt3(x.3),
                band,
                fmt3(x.4)
            )
        }).collect();
        push(format!(
            "  joint {} st{st}: Galileo [{}] | Pioneer [{}]",
            date_of(*day),
            gs.join(", "),
            ps.join(", ")
        ));
        if let Some(gc0) = gld.iter().find(|c| !c.thin) {
            if let Some(pp) = pv.iter().find(|x| x.2 >= MIN_CELL) {
                g_rms_joint.push(gc0.rms);
                p_rms_joint.push(pp.4);
            }
        }
    }
    push(format!(
        "joint (station, day) cells in window carrying both a Galileo floor cell (any n) and a Pioneer cell (any n): {joint_cells}; joint loud-both: {joint_loud_both}"
    ));
    if g_rms_joint.len() >= 5 {
        let rho = spearman(&p_rms_joint, &g_rms_joint);
        let rho_s = match rho {
            Some(v) => format!("{v:+.3}"),
            None => "-".to_string(),
        };
        push(format!(
            "joint spearman Pioneer day-RMS vs Galileo robust floor cell RMS, n {}: {rho_s}",
            g_rms_joint.len(),
        ));
    } else {
        push(format!(
            "joint spearman: n {} (< 5) — 0 honored",
            g_rms_joint.len()
        ));
    }

    push(String::new());
    push("== S4 verdict counts ==".to_string());
    let mut gal_loud: Vec<(i64, i64)> = Vec::new();
    for c in &gcells {
        if c.loud && !c.thin {
            gal_loud.push((c.day, c.st));
        }
    }
    gal_loud.sort_unstable();
    gal_loud.dedup();
    push("Galileo loud (station, day) cells in window (robust n >= 30):".to_string());
    for st in TRIO {
        let loud_days = gal_loud.iter().filter(|(_, s)| *s == st).count();
        push(format!(
            "  st{st}: {loud_days} loud days"
        ));
    }
    let mut g_loud_p_any = 0usize;
    let mut g_loud_p_present = 0usize;
    let mut g_loud_p_loud = 0usize;
    let mut g_loud_p_quiet = 0usize;
    for (day, st) in &gal_loud {
        let pv: Vec<&(String, i64, usize, f64, f64)> =
            pioneer_at(&pdiary, *st, *day);
        if pv.is_empty() {
            continue;
        }
        g_loud_p_any += 1;
        g_loud_p_present += 1;
        let loud = pv.iter().any(|x| x.2 >= MIN_CELL && x.3.abs() > QUIET_HZ);
        if loud {
            g_loud_p_loud += 1;
        } else if pv.iter().any(|x| x.2 >= MIN_CELL) {
            g_loud_p_quiet += 1;
        }
    }
    push(format!(
        "of Galileo loud (station, day), Pioneer same-cell present {g_loud_p_any} (robust {g_loud_p_present}: loud {g_loud_p_loud}, quiet {g_loud_p_quiet}); thin-only Pioneer cells not classed"
    ));

    let mut p_loud_st: BTreeMap<i64, Vec<(String, i64)>> = BTreeMap::new();
    for (probe, v) in &pcells {
        for c in v {
            if TRIO.contains(&c.rx) && c.n >= MIN_CELL && c.med.abs() > QUIET_HZ {
                p_loud_st.entry(c.rx).or_default().push((probe.clone(), c.day));
            }
        }
    }
    for st in TRIO {
        if let Some(list) = p_loud_st.get(&st) {
            for (probe, d) in list {
                let gl = gal_loud.iter().any(|(gd, gs)| *gd == *d && *gs == st);
                push(format!(
                    "  {probe} loud st{st} {} — Galileo same (station, day) loud: {}",
                    date_of(*d),
                    if gl { "yes" } else { "no (0 honored)" }
                ));
            }
        } else {
            push(format!("  Pioneer loud st{st}: none in window (0 honored)"));
        }
    }

    push(String::new());
    push("== S5 station diary: loud/quiet state per (station, day), both probes ==".to_string());
    for st in TRIO {
        push(format!("  --- st{st} ---"));
        let mut rows: BTreeMap<i64, Vec<String>> = BTreeMap::new();
        for c in gcells.iter().filter(|c| c.st == st) {
            rows.entry(c.day).or_default().push(format!(
                "g m{} {} Hz {} (n{})",
                c.mode,
                fmt3(c.rms),
                if c.thin {
                    "thin".to_string()
                } else if c.loud {
                    "LOUD".to_string()
                } else {
                    "quiet".to_string()
                },
                c.n
            ));
        }
        if let Some(v) = pdiary.get(&st) {
            for (probe, day, n, med, r) in v {
                rows.entry(*day).or_default().push(format!(
                    "{probe} med {} Hz {} rms {} Hz (n{n})",
                    fmt3(*med),
                    if *n < MIN_CELL {
                        "thin".to_string()
                    } else if med.abs() > QUIET_HZ {
                        "LOUD".to_string()
                    } else {
                        "quiet".to_string()
                    },
                    fmt3(*r)
                ));
            }
        }
        for (day, v) in rows {
            push(format!("  {} | {}", date_of(day), v.join(" | ")));
        }
    }

    let _ = std::fs::write(&report_path, out.join("\n") + "\n");
    eprintln!("report written to {report_path}");
}
