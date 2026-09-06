use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;

use omegaflow::archivar::spectral::civil_from_days;
use omegaflow::odf::parse_p11r_bin;

const DAY_S: f64 = 86400.0;
const LOCK_HZ: f64 = 1.0e3;
const LOUD_HZ: f64 = 1.0;
const ROBUST_N: usize = 30;
const PASS_GAP_S: f64 = 600.0;
const FLOOR_AGC: i64 = -2560;
const QUIET_BAND_HZ: f64 = 5.0;
const TRIO: [i64; 3] = [14, 43, 63];
const MODES: [i64; 3] = [1, 2, 3];
const MIN_CELL: usize = 30;
const N_ANCHOR: usize = 5;
const GAP_DETREND_S: f64 = 900.0;
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

fn daycell(tdb: f64) -> i64 {
    (tdb / DAY_S).floor() as i64
}

fn hour_of_day(tdb: f64) -> f64 {
    (tdb / DAY_S + 10957.5).rem_euclid(1.0) * 24.0
}

fn median(v: &[f64]) -> Option<f64> {
    if v.is_empty() {
        return None;
    }
    let mut s = v.to_vec();
    s.sort_by(f64::total_cmp);
    Some(s[s.len() / 2])
}

fn rms_about_mean(v: &[f64]) -> Option<f64> {
    if v.is_empty() {
        return None;
    }
    let m = v.iter().sum::<f64>() / v.len() as f64;
    Some((v.iter().map(|x| (x - m) * (x - m)).sum::<f64>() / v.len() as f64).sqrt())
}

fn quad_detrend_cells(ts: &[f64], vs: &[f64]) -> Vec<f64> {
    let mut out = Vec::new();
    let mut lo = 0usize;
    while lo < ts.len() {
        let mut hi = lo + 1;
        while hi < ts.len() && ts[hi] - ts[hi - 1] <= GAP_DETREND_S {
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
                    let dx = ts[k] - tx;
                    out.push(ys[k - lo] - (ca * dx * dx + cb * dx + cc));
                }
            }
        }
        lo = hi;
    }
    out
}

#[derive(Clone)]
struct PSample {
    tdb: f64,
    val: f64,
}

fn pioneer_floor_samples() -> Option<BTreeMap<(i64, i64), Vec<PSample>>> {
    let path = "data/spdf.gsfc.nasa.gov/pioneer10_navio_residuum.bin";
    let bytes = std::fs::read(path).ok()?;
    let recs = parse_p11r_bin(&bytes)?;
    let mut ts = Vec::new();
    let mut vs = Vec::new();
    let mut rs = Vec::new();
    for r in &recs {
        let t = r[0];
        let resid = r[1];
        let rx = r[5] as i64;
        if !resid.is_finite() || resid.abs() > LOCK_HZ {
            continue;
        }
        ts.push(t);
        vs.push(resid);
        rs.push(rx);
    }
    let n_rec = ts.len();
    let mut dts = Vec::with_capacity(n_rec);
    let mut dvs = Vec::with_capacity(n_rec);
    let mut drs = Vec::with_capacity(n_rec);
    let mut lo = 0usize;
    while lo < ts.len() {
        let mut hi = lo + 1;
        while hi < ts.len() && ts[hi] - ts[hi - 1] <= GAP_DETREND_S {
            hi += 1;
        }
        if hi - lo >= MIN_BLOCK {
            let sub = quad_detrend_cells(&ts[lo..hi], &vs[lo..hi]);
            for (k, v) in sub.iter().enumerate() {
                dts.push(ts[lo + k]);
                dvs.push(*v);
                drs.push(rs[lo + k]);
            }
        }
        lo = hi;
    }
    let mut by_rx: BTreeMap<i64, Vec<f64>> = BTreeMap::new();
    for (i, &rx) in drs.iter().enumerate() {
        by_rx.entry(rx).or_default().push(dvs[i]);
    }
    let mut off: BTreeMap<i64, f64> = BTreeMap::new();
    for (rx, v) in &by_rx {
        if let Some(m) = median(v) {
            off.insert(*rx, m);
        }
    }
    let mut samp: BTreeMap<(i64, i64), Vec<PSample>> = BTreeMap::new();
    for i in 0..dts.len() {
        let dc = daycell(dts[i]);
        let centered = dvs[i] - off[&drs[i]];
        samp
            .entry((drs[i], dc))
            .or_default()
            .push(PSample { tdb: dts[i], val: centered });
    }
    for v in samp.values_mut() {
        v.sort_by(|a, b| a.tdb.total_cmp(&b.tdb));
    }
    Some(samp)
}

fn band_pioneer(n: usize, med: Option<f64>) -> String {
    if n < ROBUST_N {
        return "thin".to_string();
    }
    match med {
        Some(m) if m.abs() > QUIET_BAND_HZ => "loud".to_string(),
        Some(_) => "quiet".to_string(),
        None => "thin".to_string(),
    }
}

fn band_floor(n: usize, rms: Option<f64>) -> String {
    if n < ROBUST_N {
        return "thin".to_string();
    }
    match rms {
        Some(r) if r >= LOUD_HZ => "LOUD".to_string(),
        Some(_) => "quiet".to_string(),
        None => "thin".to_string(),
    }
}

fn fopt(v: Option<f64>) -> String {
    match v {
        Some(x) if x.is_finite() => format!("{x:+.3}"),
        _ => "-".to_string(),
    }
}

fn fopt_rms(v: Option<f64>) -> String {
    match v {
        Some(x) if x.is_finite() => format!("{x:.3}"),
        _ => "-".to_string(),
    }
}

fn main() {
    let report_path = match std::env::args().skip(1).find(|a| !a.starts_with('-')) {
        Some(p) => p,
        None => "/tmp/opencode/galileo_pioneer_pass_slices_report.txt".to_string(),
    };

    let mut out: Vec<String> = Vec::new();
    let mut push = |s: String| {
        println!("{s}");
        out.push(s);
    };

    push(format!(
        "galileo-pioneer pass-slice probe — sub-daily (station, day) cells, window 1995-11-23..1997-02-28"
    ));
    push(format!(
        "conventions: Galileo floor = resid strength {FLOOR_AGC} (AGC clamp), modes 1..3, trio stations; lock |resid| > {LOCK_HZ:.0} Hz excluded before noise; floor day cell = (mode, station, TDB day), day RMS about cell mean, loud >= {LOUD_HZ:.0} Hz robust n >= {ROBUST_N}; pass = contiguous samples gap <= {PASS_GAP_S:.0} s, window RMS/median about window mean"
    ));
    push(format!(
        "Pioneer residuum floor pipeline (negative-fuzzy, as the reference readers): lock |resid| > {LOCK_HZ:.0} Hz excluded; per-block quadratic detrend (gap {GAP_DETREND_S:.0} s, block >= {MIN_BLOCK}); per-station median removal; day cell = (station, TDB day); quiet band |median| <= {QUIET_BAND_HZ:.0} Hz; window RMS/median about window mean"
    ));

    let w0 = daycell_of(1995, 11, 23);
    let w1 = daycell_of(1997, 2, 28);
    push(format!(
        "window daycells {w0}..{w1} = {} .. {}",
        date_of(w0),
        date_of(w1)
    ));

    let p_samp = match pioneer_floor_samples() {
        Some(x) => x,
        None => {
            push("pioneer10: residuum bin or parse void — 0 honored".to_string());
            return;
        }
    };
    let p_trio: Vec<(i64, i64)> = p_samp
        .iter()
        .filter(|((st, dc), v)| TRIO.contains(st) && *dc >= w0 && *dc <= w1 && !v.is_empty())
        .map(|((st, dc), _)| (*st, *dc))
        .collect();
    push(format!(
        "pioneer10 (station, day) floor cells in window (trio, post-detrend): {}",
        p_trio.len()
    ));

    let mut p_win: BTreeMap<(i64, i64), Vec<PSample>> = BTreeMap::new();
    for (&(st, dc), v) in &p_samp {
        if TRIO.contains(&st) && dc >= w0 && dc <= w1 && !v.is_empty() {
            p_win.insert((st, dc), v.clone());
        }
    }

    let gal_path = "data/pds-ppi.igpp.ucla.edu/galileo_resid.bin";
    let Ok(f) = File::open(gal_path) else {
        push("galileo resid: file void".to_string());
        return;
    };
    let mut f = f;
    let mut head = [0u8; 8];
    if f.read_exact(&mut head).is_err() || &head[0..4] != b"GASR" {
        push("galileo resid: header void (0 honored)".to_string());
        return;
    }
    let gal_count = u32::from_le_bytes([head[4], head[5], head[6], head[7]]) as usize;

    let mut cell_acc: BTreeMap<(i64, i64, i64), (f64, f64, usize)> = BTreeMap::new();
    let mut g_win: BTreeMap<(i64, i64, i64), Vec<(f64, f64)>> = BTreeMap::new();
    let mut row = [0u8; 64];
    let mut n_read = 0usize;
    while n_read < gal_count {
        if f.read_exact(&mut row).is_err() {
            break;
        }
        n_read += 1;
        let mut r = [0.0f64; 8];
        for k in 0..8 {
            let mut b = [0u8; 8];
            b.copy_from_slice(&row[k * 8..k * 8 + 8]);
            r[k] = f64::from_le_bytes(b);
        }
        let resid = r[1];
        if !resid.is_finite() || resid.abs() > LOCK_HZ {
            continue;
        }
        let st = r[2] as i64;
        if !TRIO.contains(&st) {
            continue;
        }
        let mode = r[3] as i64;
        if !MODES.contains(&mode) {
            continue;
        }
        if (r[7] as i64) != FLOOR_AGC {
            continue;
        }
        let dc = daycell(r[0]);
        let e = cell_acc.entry((mode, st, dc)).or_insert((0.0, 0.0, 0));
        e.0 += resid;
        e.1 += resid * resid;
        e.2 += 1;
        if dc >= w0 && dc <= w1 {
            g_win.entry((mode, st, dc)).or_default().push((r[0], resid));
        }
    }
    drop(f);
    for v in g_win.values_mut() {
        v.sort_by(|a, b| a.0.total_cmp(&b.0));
    }
    push(format!("galileo resid: {n_read} rows read"));

    push(String::new());
    push("== S0 reproduction: Galileo floor day cells in window (robust loud, like the reference floor probes) ==".to_string());
    for mode in MODES {
        for st in TRIO {
            let mut n_cells = 0usize;
            let mut n_robust = 0usize;
            let mut n_loud = 0usize;
            for (&(m, s, dc), &(sum, sum2, n)) in &cell_acc {
                if m == mode && s == st && dc >= w0 && dc <= w1 {
                    n_cells += 1;
                    if n >= MIN_CELL {
                        n_robust += 1;
                        let mean = sum / n as f64;
                        let var = (sum2 / n as f64 - mean * mean).max(0.0);
                        if var.sqrt() >= LOUD_HZ {
                            n_loud += 1;
                        }
                    }
                }
            }
            push(format!(
                "  mode {mode} st{st}: {n_cells} floor day cells, {n_robust} robust, {n_loud} robust loud"
            ));
        }
    }

    push(String::new());
    push("== S0b reproduction: Pioneer10 floor (station, day) cells in window ==".to_string());
    for st in TRIO {
        let n = p_trio.iter().filter(|(s, _)| *s == st).count();
        push(format!("  st{st}: {n} floor day cells (post-detrend)"));
    }

    push(String::new());
    push("== S1 shared (station, day) cells in window ==".to_string());
    let mut shared: Vec<(i64, i64)> = Vec::new();
    {
        let mut set: BTreeMap<(i64, i64), bool> = BTreeMap::new();
        for (&(_, st, dc), v) in &g_win {
            if !v.is_empty() {
                set.insert((st, dc), true);
            }
        }
        for (&(st, dc), v) in &p_samp {
            if TRIO.contains(&st)
                && dc >= w0
                && dc <= w1
                && !v.is_empty()
                && set.contains_key(&(st, dc))
            {
                shared.push((st, dc));
            }
        }
    }
    shared.sort();
    shared.dedup();
    push(format!("shared (station, day) cells: {}", shared.len()));

    push(String::new());
    push("== S2 per shared cell: day-level state and floor pass windows ==".to_string());
    for (st, dc) in &shared {
        push(format!("  --- {} st{st} ---", date_of(*dc)));

        if let Some(v) = p_win.get(&(*st, *dc)) {
            let vals: Vec<f64> = v.iter().map(|s| s.val).collect();
            let med = median(&vals);
            let rms = rms_about_mean(&vals);
            push(format!(
                "    Pioneer day n{} med {} rms {} [{}]",
                v.len(),
                fopt(med),
                fopt_rms(rms),
                band_pioneer(v.len(), med)
            ));
            let mut wstart = 0usize;
            let mut wi = 0usize;
            while wi < v.len() {
                let mut j = wi + 1;
                while j < v.len() && v[j].tdb - v[j - 1].tdb <= PASS_GAP_S {
                    j += 1;
                }
                let n = j - wi;
                if n >= 4 {
                    let wvals: Vec<f64> = v[wi..j].iter().map(|s| s.val).collect();
                    let wmed = median(&wvals);
                    let wrms = rms_about_mean(&wvals);
                    push(format!(
                        "      Pioneer window {}: {}-{}h n{n} med {} rms {} [{}]",
                        wstart,
                        hour_of_day(v[wi].tdb),
                        hour_of_day(v[j - 1].tdb),
                        fopt(wmed),
                        fopt_rms(wrms),
                        band_pioneer(n, wmed)
                    ));
                }
                wstart += 1;
                wi = j;
            }
        } else {
            push("    Pioneer: no floor samples this day (0 honored)".to_string());
        }

        for mode in MODES {
            let key = (mode, *st, *dc);
            let gv = g_win.get(&key);
            let acc = cell_acc.get(&key).copied();
            match (gv, acc) {
                (Some(v), Some((sum, sum2, n))) => {
                    let mean = sum / n as f64;
                    let var = (sum2 / n as f64 - mean * mean).max(0.0);
                    let drms = var.sqrt();
                    let n_robust = n >= MIN_CELL;
                    push(format!(
                        "    Galileo floor m{mode}: day n{n} dayRMS {drms:.3} [{}]",
                        band_floor(if n_robust { n } else { 0 }, Some(drms))
                    ));
                    let mut wi = 0usize;
                    while wi < v.len() {
                        let mut j = wi + 1;
                        while j < v.len() && v[j].0 - v[j - 1].0 <= PASS_GAP_S {
                            j += 1;
                        }
                        let wn = j - wi;
                        if wn >= 4 {
                            let wvals: Vec<f64> = v[wi..j].iter().map(|x| x.1).collect();
                            let wmed = median(&wvals);
                            let wrms = rms_about_mean(&wvals);
                            push(format!(
                                "      m{mode} window: {}-{}h n{wn} med {} rms {} [{}]",
                                hour_of_day(v[wi].0),
                                hour_of_day(v[j - 1].0),
                                fopt(wmed),
                                fopt_rms(wrms),
                                band_floor(if wn >= MIN_CELL { wn } else { 0 }, wrms)
                            ));
                        }
                        wi = j;
                    }
                }
                _ => push(format!("    Galileo floor m{mode}: no floor samples (0 honored)")),
            }
        }
    }

    push(String::new());
    push("== S3 span and overlap of Pioneer vs Galileo floor on each shared cell ==".to_string());
    for (st, dc) in &shared {
        let pspan = p_win
            .get(&(*st, *dc))
            .and_then(|v| v.first().zip(v.last()))
            .map(|(a, b)| (a.tdb, b.tdb));
        let g_all: Vec<f64> = MODES
            .iter()
            .filter_map(|m| g_win.get(&(*m, *st, *dc)))
            .flat_map(|v| v.iter().map(|x| x.0))
            .collect();
        let gspan = g_all.iter().min_by(|a, b| a.total_cmp(b)).zip(g_all.iter().max_by(|a, b| a.total_cmp(b))).map(|(a, b)| (*a, *b));
        let p_str = match pspan {
            Some((a, b)) => format!("{:.2}-{:.2}h", hour_of_day(a), hour_of_day(b)),
            None => "absent".to_string(),
        };
        let g_str = match gspan {
            Some((a, b)) => format!("{:.2}-{:.2}h", hour_of_day(a), hour_of_day(b)),
            None => "absent".to_string(),
        };
        let relation = match (pspan, gspan) {
            (Some((pa, pb)), Some((ga, gb))) => {
                let ov = (pb.min(gb) - pa.max(ga)).max(0.0);
                if ov <= 1.0 {
                    "disjoint".to_string()
                } else {
                    format!("overlap {:.0} s", ov)
                }
            }
            _ => "absent".to_string(),
        };
        push(format!(
            "  {} st{st}: pioneer span {p_str} | galileo floor span {g_str} | {relation}",
            date_of(*dc)
        ));
    }

    push(String::new());
    push("== S4 loud anchor days: intra-day pass structure of the loudest robust floor cells ==".to_string());
    let mut loud: Vec<(i64, i64, i64, f64, usize)> = cell_acc
        .iter()
        .filter(|&(&(m, s, dc), &(_sum, _sum2, n))| {
            MODES.contains(&m) && TRIO.contains(&s) && dc >= w0 && dc <= w1 && n >= MIN_CELL
        })
        .map(|(&(m, s, dc), &(sum, sum2, n))| {
            let mean = sum / n as f64;
            let var = (sum2 / n as f64 - mean * mean).max(0.0);
            (m, s, dc, var.sqrt(), n)
        })
        .filter(|c| c.3 >= LOUD_HZ)
        .collect();
    loud.sort_by(|a, b| b.3.total_cmp(&a.3));
    let mut anchors: Vec<(i64, i64)> = Vec::new();
    for c in &loud {
        if !anchors.contains(&(c.1, c.2)) {
            anchors.push((c.1, c.2));
        }
        if anchors.len() >= N_ANCHOR {
            break;
        }
    }
    if anchors.is_empty() {
        push("no loud robust floor anchor (0 honored)".to_string());
    }
    for (st, dc) in &anchors {
        push(format!("  --- loud anchor {} st{st} ---", date_of(*dc)));
        for mode in MODES {
            if let Some(v) = g_win.get(&(mode, *st, *dc)) {
                let vals: Vec<f64> = v.iter().map(|x| x.1).collect();
                let n = vals.len();
                let med = median(&vals);
                let rms = rms_about_mean(&vals);
                push(format!(
                    "    m{mode}: day n{n} med {} rms {}",
                    fopt(med),
                    fopt_rms(rms)
                ));
                let mut wi = 0usize;
                while wi < v.len() {
                    let mut j = wi + 1;
                    while j < v.len() && v[j].0 - v[j - 1].0 <= PASS_GAP_S {
                        j += 1;
                    }
                    let wn = j - wi;
                    if wn >= 4 {
                        let wvals: Vec<f64> = v[wi..j].iter().map(|x| x.1).collect();
                        let wmed = median(&wvals);
                        let wrms = rms_about_mean(&wvals);
                        push(format!(
                            "      window {}-{}h n{wn} med {} rms {}",
                            hour_of_day(v[wi].0),
                            hour_of_day(v[j - 1].0),
                            fopt(wmed),
                            fopt_rms(wrms)
                        ));
                    }
                    wi = j;
                }
            }
        }
    }


    push(String::new());
    push("== S5 loud-day = loud-pass? aggregate over robust loud floor day-cells ==".to_string());
    push(format!(
        "loud floor cell = (mode, station, day) in window with n >= {MIN_CELL} and day RMS >= {LOUD_HZ:.0} Hz; floor windows split at gap <= {PASS_GAP_S:.0} s; robust window n >= {MIN_CELL}"
    ));
    let loud_cells: Vec<(i64, i64, i64)> = cell_acc
        .iter()
        .filter(|&(&(m, s, dc), &(_sum, _sum2, n))| {
            MODES.contains(&m) && TRIO.contains(&s) && dc >= w0 && dc <= w1 && n >= MIN_CELL
        })
        .filter(|&(_, &(sum, sum2, n))| {
            let mean = sum / n as f64;
            let var = (sum2 / n as f64 - mean * mean).max(0.0);
            var.sqrt() >= LOUD_HZ
        })
        .map(|(&(m, s, dc), _)| (m, s, dc))
        .collect();
    push(format!("robust loud floor day-cells in window: {}", loud_cells.len()));
    let mut n_single = 0usize;
    let mut n_multi_all_loud = 0usize;
    let mut n_multi_mixed = 0usize;
    let mut n_no_robust_win = 0usize;
    for (m, s, dc) in &loud_cells {
        if let Some(v) = g_win.get(&(*m, *s, *dc)) {
            let mut wins = Vec::new();
            let mut wi = 0usize;
            while wi < v.len() {
                let mut j = wi + 1;
                while j < v.len() && v[j].0 - v[j - 1].0 <= PASS_GAP_S {
                    j += 1;
                }
                if j - wi >= MIN_CELL {
                    let wvals: Vec<f64> = v[wi..j].iter().map(|x| x.1).collect();
                    let r = rms_about_mean(&wvals);
                    wins.push(match r { Some(x) => x >= LOUD_HZ, None => false });
                }
                wi = j;
            }
            if wins.len() == 1 {
                n_single += 1;
            } else if wins.is_empty() {
                n_no_robust_win += 1;
            } else if wins.iter().all(|&b| b) {
                n_multi_all_loud += 1;
            } else {
                n_multi_mixed += 1;
            }
        }
    }
    push(format!(
        "of {loud_n} loud day-cells: {n_single} single robust window (day loudness = one window); {n_multi_all_loud} multi-window all loud; {n_multi_mixed} multi-window with a quiet robust window; {n_no_robust_win} with floor windows all below robust n",
        loud_n = loud_cells.len()
    ));
    let mut loud_day_st: BTreeMap<(i64, i64), bool> = BTreeMap::new();
    for (_m, s, dc) in &loud_cells {
        loud_day_st.insert((*s, *dc), true);
    }
    for st in TRIO {
        let n = loud_day_st.iter().filter(|((s, _), _)| *s == st).count();
        push(format!("  loud (station, day) st{st}: {n}"));
    }
    push(format!(
        "loud (station, day) total: {}",
        loud_day_st.len()
    ));

    let _ = std::fs::write(&report_path, out.join("\n") + "\n");
    eprintln!("report written to {report_path}");
}
