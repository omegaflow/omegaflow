use std::collections::{BTreeMap, HashMap};

use omegaflow::archivar::{
    BodyEphemeris, body_barycenter_position, body_barycenter_velocity, body_fixed_to_icrs_smooth,
    parse_ephemeris_binary,
};
use omegaflow::odf::{parse_p11r_bin, parse_podf_bin, write_p11r_bin};
use omegaflow::odp::{C, EARTH, downlink_rate_core, dsn_station, station_velocity};

const SC_BODY: &str = "pioneer11_daily";
const GAP_PASS_S: f64 = 5.0 * 86400.0;
const GAP_BLOCK_S: f64 = 120.0;
const MIN_BLOCK: usize = 4;
const F_ALIAS: f64 = 0.714e-3;
const LS_FLO: f64 = 0.0004;
const LS_FHI: f64 = 0.0015;
const LS_STEP: f64 = 0.00002;
const FLOOR_FLO: f64 = 0.0008;
const FLOOR_FHI: f64 = 0.0012;
const MIN_CELL: usize = 500;

const BEAT_BOUND: f64 = 5.0e5;

fn dist(a: [f64; 3], b: [f64; 3]) -> f64 {
    let dx = a[0] - b[0];
    let dy = a[1] - b[1];
    let dz = a[2] - b[2];
    (dx * dx + dy * dy + dz * dz).sqrt()
}

fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn jd_date(tdb: f64) -> String {
    let jd = tdb / 86400.0 + 2451545.0;
    let unix_day = (jd - 2440587.5).round() as i64;
    match omegaflow::spectral::civil_from_days(unix_day) {
        Some((y, m, d)) => format!("{y:04}-{m:02}-{d:02}"),
        None => format!("tdb {tdb:.0} s"),
    }
}

fn fixed_effects_1(
    x: &[f64],
    obs: &[f64],
    times: &[f64],
) -> Option<(f64, Vec<f64>, Vec<usize>, Vec<f64>)> {
    let n = x.len();
    let mut epoch = vec![0usize; n];
    let mut eid = 0usize;
    for i in 0..n {
        if i > 0 && times[i] - times[i - 1] > GAP_PASS_S {
            eid += 1;
        }
        epoch[i] = eid;
    }
    let n_epoch = eid + 1;
    let mut mx = vec![0.0f64; n_epoch];
    let mut mo = vec![0.0f64; n_epoch];
    let mut cnt = vec![0usize; n_epoch];
    for i in 0..n {
        let e = epoch[i];
        mx[e] += x[i];
        mo[e] += obs[i];
        cnt[e] += 1;
    }
    for e in 0..n_epoch {
        mx[e] /= cnt[e] as f64;
        mo[e] /= cnt[e] as f64;
    }
    let mut sxx = 0.0;
    let mut sxy = 0.0;
    for i in 0..n {
        let dx = x[i] - mx[epoch[i]];
        let dy = obs[i] - mo[epoch[i]];
        sxx += dx * dx;
        sxy += dx * dy;
    }
    if sxx.abs() < 1e-300 {
        return None;
    }
    let a = sxy / sxx;
    let mut offset = vec![0.0f64; n_epoch];
    for e in 0..n_epoch {
        offset[e] = mo[e] - a * mx[e];
    }
    let mut resid = vec![0.0f64; n];
    for i in 0..n {
        resid[i] = obs[i] - a * x[i] - offset[epoch[i]];
    }
    Some((a, resid, epoch, offset))
}

fn light_time_sc_pos(
    t3: f64,
    r_rx: [f64; 3],
    sc: &dyn Fn(f64) -> Option<([f64; 3], [f64; 3])>,
) -> Option<(f64, [f64; 3])> {
    let mut t2 = t3;
    for _ in 0..6 {
        let (r_sc2, _) = sc(t2)?;
        let rho = dist(r_rx, r_sc2);
        let t2_new = t3 - rho / C;
        if (t2_new - t2).abs() < 1e-9 {
            t2 = t2_new;
            break;
        }
        t2 = t2_new;
    }
    let (r_sc2, _) = sc(t2)?;
    Some((t2, r_sc2))
}

fn uplink_rate(
    t2: f64,
    r_sc2: [f64; 3],
    v_sc2: [f64; 3],
    lat: f64,
    lon: f64,
    alt: f64,
    eph: &HashMap<String, BodyEphemeris>,
) -> Option<f64> {
    let mut t1 = t2;
    for _ in 0..6 {
        let r_tx1 = body_fixed_to_icrs_smooth(EARTH, lat, lon, alt, t1, eph)?;
        let rho = dist(r_sc2, r_tx1);
        if rho <= 0.0 {
            return None;
        }
        let t1_new = t2 - rho / C;
        if (t1_new - t1).abs() < 1e-9 {
            t1 = t1_new;
            break;
        }
        t1 = t1_new;
    }
    let r_tx = body_fixed_to_icrs_smooth(EARTH, lat, lon, alt, t1, eph)?;
    let v_tx = station_velocity(t1, lat, lon, alt, eph)?;
    let rho = dist(r_sc2, r_tx);
    if rho <= 0.0 {
        return None;
    }
    let mut du = [0.0; 3];
    let mut vu = [0.0; 3];
    for k in 0..3 {
        du[k] = r_sc2[k] - r_tx[k];
        vu[k] = v_sc2[k] - v_tx[k];
    }
    Some(dot(du, vu) / rho)
}

fn detrend_blocks(ts: &[f64], vs: &[f64], gap: f64, min_len: usize) -> (Vec<f64>, Vec<f64>) {
    let mut dts: Vec<f64> = Vec::new();
    let mut dvs: Vec<f64> = Vec::new();
    let mut lo = 0usize;
    while lo < ts.len() {
        let mut hi = lo + 1;
        while hi < ts.len() && ts[hi] - ts[hi - 1] <= gap {
            hi += 1;
        }
        if hi - lo >= min_len {
            let xt: Vec<f64> = ts[lo..hi].to_vec();
            let yv: Vec<f64> = vs[lo..hi].to_vec();
            let mx = xt.iter().sum::<f64>() / xt.len() as f64;
            let my = yv.iter().sum::<f64>() / yv.len() as f64;
            let mut num = 0.0;
            let mut den = 0.0;
            for k in 0..xt.len() {
                num += (xt[k] - mx) * (yv[k] - my);
                den += (xt[k] - mx) * (xt[k] - mx);
            }
            let slope = if den.abs() > 1e-300 { num / den } else { 0.0 };
            for k in 0..xt.len() {
                dts.push(xt[k]);
                dvs.push(yv[k] - (slope * (xt[k] - mx) + my));
            }
        }
        lo = hi;
    }
    (dts, dvs)
}

fn ls_peak(ts: &[f64], vs: &[f64], flo: f64, fhi: f64, step: f64) -> (f64, f64) {
    let m = ts.len() as f64;
    let vsum = vs.iter().sum::<f64>() / m;
    let mut best = (f64::NAN, 0.0);
    let mut f = flo;
    while f <= fhi {
        let mut s = 0.0;
        let mut c = 0.0;
        for &t in ts {
            let ph = std::f64::consts::TAU * f * t;
            s += ph.sin();
            c += ph.cos();
        }
        s /= m;
        c /= m;
        let mut ss = 0.0;
        let mut cc = 0.0;
        let mut sc = 0.0;
        let mut sy = 0.0;
        let mut cy = 0.0;
        for (i, &t) in ts.iter().enumerate() {
            let ph = std::f64::consts::TAU * f * t;
            let ds = ph.sin() - s;
            let dc = ph.cos() - c;
            let dv = vs[i] - vsum;
            ss += ds * ds;
            cc += dc * dc;
            sc += ds * dc;
            sy += ds * dv;
            cy += dc * dv;
        }
        let det = ss * cc - sc * sc;
        if det.abs() > 1e-300 {
            let a = (sy * cc - cy * sc) / det;
            let b = (cy * ss - sy * sc) / det;
            let p = (a * a + b * b) * m / 2.0;
            if p > best.1 {
                best = (f, p);
            }
        }
        f += step;
    }
    best
}

fn run_stats(ts: &[f64], gap: f64, min_len: usize) -> (usize, usize, usize) {
    let mut runs = 0usize;
    let mut max_len = 0usize;
    let mut covered = 0usize;
    let mut lo = 0usize;
    while lo < ts.len() {
        let mut hi = lo + 1;
        while hi < ts.len() && ts[hi] - ts[hi - 1] <= gap {
            hi += 1;
        }
        let len = hi - lo;
        if len >= min_len {
            runs += 1;
            covered += len;
            if len > max_len {
                max_len = len;
            }
        }
        lo = hi;
    }
    (runs, max_len, covered)
}

fn band_scan(label: &str, ts: &[f64], vs: &[f64], gap: f64) {
    let mut fts: Vec<f64> = Vec::new();
    let mut fvs: Vec<f64> = Vec::new();
    let mut n_beat = 0usize;
    for (i, &t) in ts.iter().enumerate() {
        if vs[i].is_finite() && vs[i].abs() <= BEAT_BOUND {
            fts.push(t);
            fvs.push(vs[i]);
        } else {
            n_beat += 1;
        }
    }
    let (dts, dvs) = detrend_blocks(&fts, &fvs, gap, MIN_BLOCK);
    if dts.len() < MIN_CELL {
        eprintln!(
            "p11-resid {label}: {dts} detrended samples ({n_beat} beyond the beat bound discarded) — too short (0 honored)",
            dts = dts.len()
        );
        return;
    }
    let (fp, pp) = ls_peak(&dts, &dvs, LS_FLO, LS_FHI, LS_STEP);
    let (_, p_alias) = ls_peak(&dts, &dvs, F_ALIAS, F_ALIAS, 1e-12);
    let mut floor: Vec<f64> = Vec::new();
    let mut f = FLOOR_FLO;
    while f <= FLOOR_FHI {
        let (_, p) = ls_peak(&dts, &dvs, f, f, 1e-12);
        floor.push(p);
        f += LS_STEP;
    }
    floor.sort_by(f64::total_cmp);
    let fmed = floor[floor.len() / 2];
    let shape = [0.0004, 0.0006, F_ALIAS, 0.0008, 0.0010, 0.0012];
    let shape_line: Vec<String> = shape
        .iter()
        .map(|&sf| {
            let (_, p) = ls_peak(&dts, &dvs, sf, sf, 1e-12);
            format!("{:.1}×", p / fmed)
        })
        .collect();
    eprintln!(
        "p11-resid {label}: n={}, peak {:.5} Hz ({:.1}× local floor [0,8–1,2 mHz]), A = {:.2e} Hz; at the 0,714 mHz alias: {:.1}× local floor ({n_beat} beyond the beat bound); form P(0,4/0,6/0,714/0,8/1,0/1,2 mHz) = {}",
        dts.len(),
        fp,
        pp / fmed,
        (2.0 * pp / dts.len() as f64).sqrt(),
        p_alias / fmed,
        shape_line.join("/")
    );
}

fn main() {
    let podf = "data/pioneer11_odf.bin";
    let Ok(bytes) = std::fs::read(podf) else {
        eprintln!("p11-resid PODF bin void ({podf})");
        return;
    };
    let Some(records) = parse_podf_bin(&bytes) else {
        eprintln!("p11-resid PODF bin parse void");
        return;
    };
    let mut eph: HashMap<String, BodyEphemeris> = HashMap::new();
    for body in [EARTH, SC_BODY] {
        let p = format!("data/ephemeris_{body}.bin");
        match std::fs::read(&p)
            .ok()
            .and_then(|d| parse_ephemeris_binary(&d))
        {
            Some(e) => {
                eph.insert(body.to_string(), e);
            }
            None => {
                eprintln!("p11-resid {body}: ephemeris bin void ({p})");
                return;
            }
        }
    }
    let sc = |t: f64| -> Option<([f64; 3], [f64; 3])> {
        Some((
            body_barycenter_position(SC_BODY, t, &eph)?,
            body_barycenter_velocity(SC_BODY, t, &eph)?,
        ))
    };

    let mut cells: BTreeMap<(i64, i64, i64), (usize, f64, f64)> = BTreeMap::new();
    let mut station_span: BTreeMap<i64, (f64, f64, usize)> = BTreeMap::new();
    for r in &records {
        if r[8] < 2.0 {
            continue;
        }
        let key = (r[3] as i64, r[4] as i64, r[5] as i64);
        let e = cells
            .entry(key)
            .or_insert((0, f64::INFINITY, f64::NEG_INFINITY));
        e.0 += 1;
        if r[0] < e.1 {
            e.1 = r[0];
        }
        if r[0] > e.2 {
            e.2 = r[0];
        }
        let s = station_span
            .entry(r[3] as i64)
            .or_insert((f64::INFINITY, f64::NEG_INFINITY, 0));
        if r[0] < s.0 {
            s.0 = r[0];
        }
        if r[0] > s.1 {
            s.1 = r[0];
        }
        s.2 += 1;
    }
    eprintln!(
        "p11-resid census: {} samples; stations (receive): {}",
        records.len(),
        station_span
            .iter()
            .map(|(st, (t0, t1, n))| format!("{st} n={n} {}..{}", jd_date(*t0), jd_date(*t1)))
            .collect::<Vec<_>>()
            .join(" | ")
    );
    for ((rx, tx, mode), (n, _, _)) in &cells {
        let mname = match mode {
            11 => "one-way",
            12 => "two-way",
            13 => "three-way",
            14 => "three-way-coh",
            _ => "other",
        };
        if *n >= MIN_CELL || *mode == 13 || *mode == 14 {
            eprintln!("p11-resid   {rx}×{tx} mode {mode} ({mname}): {n} samples");
        }
    }

    let mut modeled: Vec<[f64; 9]> = Vec::new();
    let mut resid_slot: Vec<Option<f64>> = Vec::new();
    let mut rdown_of: Vec<f64> = Vec::new();
    let mut no_station = 0usize;
    let mut no_model = 0usize;
    let mut no_comp = 0usize;
    for r in &records {
        if r[8] < 2.0 {
            no_comp += 1;
            continue;
        }
        let mode = r[5] as i64;
        if !(11..=14).contains(&mode) {
            continue;
        }
        let rx = r[3] as i64;
        let tx = r[4] as i64;
        let Some((rx_lat, rx_lon, rx_alt)) = dsn_station(rx) else {
            no_station += 1;
            continue;
        };
        let need_tx = mode >= 12;
        let tx_ll = if need_tx {
            match dsn_station(tx) {
                Some(v) => Some(v),
                None => {
                    no_station += 1;
                    continue;
                }
            }
        } else {
            None
        };
        let t3 = r[0];
        let (Some(r_rx), Some(v_rx)) = (
            body_fixed_to_icrs_smooth(EARTH, rx_lat, rx_lon, rx_alt, t3, &eph),
            station_velocity(t3, rx_lat, rx_lon, rx_alt, &eph),
        ) else {
            no_model += 1;
            continue;
        };
        let Some(rdown) = downlink_rate_core(t3, r_rx, v_rx, &sc) else {
            no_model += 1;
            continue;
        };
        if !rdown.is_finite() {
            no_model += 1;
            continue;
        }
        let mut rate = rdown;
        if let Some((tx_lat, tx_lon, tx_alt)) = tx_ll {
            let Some((t2, r_sc2)) = light_time_sc_pos(t3, r_rx, &sc) else {
                no_model += 1;
                continue;
            };
            let Some((_, v_sc2)) = sc(t2) else {
                no_model += 1;
                continue;
            };
            let Some(rup) = uplink_rate(t2, r_sc2, v_sc2, tx_lat, tx_lon, tx_alt, &eph) else {
                no_model += 1;
                continue;
            };
            if !rup.is_finite() {
                no_model += 1;
                continue;
            }
            rate += rup;
        }
        modeled.push([
            t3,
            f64::NAN,
            r[1],
            r[2],
            rate,
            rx as f64,
            tx as f64,
            mode as f64,
            r[8],
        ]);
        resid_slot.push(None);
        rdown_of.push(rdown);
    }
    eprintln!(
        "p11-resid model: {} samples modeled ({} without station coordinates, {} without model, {} without compression) — skipped ones stay unwritten (0 honored)",
        modeled.len(),
        no_station,
        no_model,
        no_comp
    );
    if modeled.len() < 100 {
        eprintln!("p11-resid too short — the series stays unwritten (0 honored)");
        return;
    }

    let rx_of: Vec<i64> = modeled.iter().map(|m| m[5] as i64).collect();
    let mut stations: Vec<i64> = rx_of.clone();
    stations.sort_unstable();
    stations.dedup();
    for st in &stations {
        let mut rates: Vec<f64> = Vec::new();
        let mut refs: Vec<f64> = Vec::new();
        let mut obs: Vec<f64> = Vec::new();
        let mut times: Vec<f64> = Vec::new();
        let mut idx: Vec<usize> = Vec::new();
        for (i, m) in modeled.iter().enumerate() {
            if m[5] as i64 == *st {
                rates.push(m[4]);
                refs.push(m[3]);
                obs.push(m[2]);
                times.push(m[0]);
                idx.push(i);
            }
        }
        let Some((a_full, resid, _, _)) = fixed_effects_1(&rates, &obs, &times) else {
            eprintln!("p11-resid station {st}: basis fit void");
            continue;
        };
        let rms_full = (resid.iter().map(|v| v * v).sum::<f64>() / resid.len() as f64).sqrt();
        let rates_d: Vec<f64> = idx.iter().map(|&i| rdown_of[i]).collect();
        let a_down = fixed_effects_1(&rates_d, &obs, &times).map(|(ad, _, _, _)| ad);
        for (k, &i) in idx.iter().enumerate() {
            resid_slot[i] = Some(resid[k]);
            modeled[i][1] = resid[k];
        }
        let ref_med = {
            let mut rs: Vec<f64> = refs.clone();
            rs.sort_by(f64::total_cmp);
            rs[rs.len() / 2]
        };
        let ref_min = refs.iter().cloned().fold(f64::INFINITY, f64::min);
        let ref_max = refs.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let obs_min = obs.iter().cloned().fold(f64::INFINITY, f64::min);
        let obs_max = obs.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        eprintln!(
            "p11-resid station {st}: {n_st} samples, obs {obs_min:.3e}..{obs_max:.3e} Hz, ref {ref_min:.3e}..{ref_med:.3e}..{ref_max:.3e} Hz — obs = A·ṙ₂w + B_Pass: A {a_full:.4e} Hz/(m/s) (≈ +f/c), residual RMS {rms_full:.3e} Hz; downlink-only A {a_down:.4e} Hz/(m/s) (≈ 2× — the uplink leg is carried)",
            n_st = times.len(),
            a_down = a_down.unwrap_or(f64::NAN),
        );
    }
    let unset = resid_slot.iter().filter(|s| s.is_none()).count();
    if unset > 0 {
        eprintln!("p11-resid {unset} residuals without fit — discarded (0 honored)");
        let mut keep = Vec::new();
        for (i, m) in modeled.iter().enumerate() {
            if resid_slot[i].is_some() {
                keep.push(*m);
            }
        }
        modeled = keep;
    }
    let out = "data/pioneer11_residuum.bin";
    let bin = write_p11r_bin(&modeled);
    if std::fs::write(out, &bin).is_err() {
        eprintln!("p11-resid write {out} void");
        return;
    }
    match parse_p11r_bin(&bin) {
        Some(parsed) if parsed.len() == modeled.len() => {
            eprintln!(
                "p11-resid {out}: {} residual samples, {}..{}, {:.0} B — roundtrip parses",
                parsed.len(),
                jd_date(parsed[0][0]),
                jd_date(parsed[parsed.len() - 1][0]),
                bin.len()
            );
        }
        _ => eprintln!("p11-resid {out}: roundtrip parse void — the series stays unverified"),
    }

    for st in &stations {
        let mut tw: Vec<(f64, f64)> = Vec::new();
        let mut th: Vec<(f64, f64)> = Vec::new();
        let mut th_cells: Vec<(i64, usize)> = Vec::new();
        let mut cell_map: BTreeMap<i64, usize> = BTreeMap::new();
        for m in &modeled {
            if m[5] as i64 != *st {
                continue;
            }
            if !m[1].is_finite() || m[1].abs() > BEAT_BOUND {
                continue;
            }
            let mode = m[7] as i64;
            if mode == 12 && m[5] as i64 == m[6] as i64 {
                tw.push((m[0], m[1]));
            } else if (mode == 13 || mode == 14) && m[5] as i64 != m[6] as i64 {
                th.push((m[0], m[1]));
                *cell_map.entry(m[6] as i64).or_default() += 1;
            }
        }
        for (txx, c) in cell_map {
            th_cells.push((txx, c));
        }
        th_cells.sort_unstable();
        if th.is_empty() {
            continue;
        }
        th.sort_by(|a, b| a.0.total_cmp(&b.0));
        let tsv: Vec<f64> = th.iter().map(|p| p.0).collect();
        let gaps = [120.0, 600.0, 3600.0];
        let run_line: Vec<String> = gaps
            .iter()
            .map(|g| {
                let (runs, max_len, covered) = run_stats(&tsv, *g, MIN_BLOCK);
                format!("{g:.0}-s gap: {runs} runs, max {max_len}, {covered} samples in runs")
            })
            .collect();
        eprintln!(
            "p11-resid station {st} split: two-way n={}, three-way n={} ({}) — {}",
            tw.len(),
            th.len(),
            th_cells
                .iter()
                .map(|(txx, c)| format!("×{txx} {c}"))
                .collect::<Vec<_>>()
                .join(", "),
            run_line.join("; ")
        );
        let tw_ts: Vec<f64> = tw.iter().map(|p| p.0).collect();
        let tw_vs: Vec<f64> = tw.iter().map(|p| p.1).collect();
        let th_ts: Vec<f64> = th.iter().map(|p| p.0).collect();
        let th_vs: Vec<f64> = th.iter().map(|p| p.1).collect();
        let (tw_dts, tw_dvs) = detrend_blocks(&tw_ts, &tw_vs, GAP_BLOCK_S, MIN_BLOCK);

        let th_gap = 600.0;
        let (th_dts, th_dvs) = detrend_blocks(&th_ts, &th_vs, th_gap, MIN_BLOCK);
        if tw_dts.len() < MIN_CELL {
            eprintln!(
                "p11-resid station {st} split: two-way scan too short ({n_tw} detrended samples, 0 honored) — three-way scan {n_th} detrended samples",
                n_tw = tw_dts.len(),
                n_th = th_dts.len()
            );
            continue;
        }
        let (tw_fp, tw_pp) = ls_peak(&tw_dts, &tw_dvs, LS_FLO, LS_FHI, LS_STEP);
        let (_, tw_alias) = ls_peak(&tw_dts, &tw_dvs, F_ALIAS, F_ALIAS, 1e-12);
        let mut floor: Vec<f64> = Vec::new();
        let mut f = FLOOR_FLO;
        while f <= FLOOR_FHI {
            let (_, p) = ls_peak(&tw_dts, &tw_dvs, f, f, 1e-12);
            floor.push(p);
            f += LS_STEP;
        }
        floor.sort_by(f64::total_cmp);
        let fmed = floor[floor.len() / 2];
        let mut floor_th: Vec<f64> = Vec::new();
        let mut f = FLOOR_FLO;
        while f <= FLOOR_FHI {
            let (_, p) = ls_peak(&th_dts, &th_dvs, f, f, 1e-12);
            floor_th.push(p);
            f += LS_STEP;
        }
        floor_th.sort_by(f64::total_cmp);
        let fmed_th = floor_th[floor_th.len() / 2];
        if th_dts.len() >= MIN_CELL {
            let (th_fp, th_pp) = ls_peak(&th_dts, &th_dvs, LS_FLO, LS_FHI, LS_STEP);
            let (_, th_alias) = ls_peak(&th_dts, &th_dvs, F_ALIAS, F_ALIAS, 1e-12);
            eprintln!(
                "p11-resid station {st} split scan: two-way (n={n_tw}) peak {tw_fp:.5} Hz (A {tw_a:.2e} Hz, alias {tw_alias:.1}× own local floor), three-way 600-s blocks (n={n_th}) peak {th_fp:.5} Hz (A {th_a:.2e} Hz, alias {th_alias:.1}× own local floor)",
                n_tw = tw_dts.len(),
                n_th = th_dts.len(),
                tw_a = (2.0 * tw_pp / tw_dts.len() as f64).sqrt(),
                th_a = (2.0 * th_pp / th_dts.len() as f64).sqrt(),
                tw_alias = tw_alias / fmed,
                th_alias = th_alias / fmed_th,
            );
        } else {
            let th_rms = (th.iter().map(|p| p.1 * p.1).sum::<f64>() / th.len() as f64).sqrt();
            let tw_rms = (tw.iter().map(|p| p.1 * p.1).sum::<f64>() / tw.len() as f64).sqrt();
            eprintln!(
                "p11-resid station {st} split: three-way 600-s blocks too short ({n_th} detrended samples) — singles as consistency points: residual RMS two-way {tw_rms:.3e} Hz vs three-way {th_rms:.3e} Hz; two-way scan (n={n_tw}): peak {tw_fp:.5} Hz, alias {tw_alias:.1}× local floor",
                n_th = th_dts.len(),
                n_tw = tw_dts.len(),
                tw_alias = tw_alias / fmed,
            );
        }
    }

    let mut keys: Vec<(i64, i64, i64)> = cells.keys().copied().collect();
    keys.sort_unstable();
    for (rx, tx, mode) in keys {
        if !(11..=14).contains(&mode) || dsn_station(rx).is_none() {
            continue;
        }
        if mode >= 12 && dsn_station(tx).is_none() {
            continue;
        }
        let n_cell = cells[&(rx, tx, mode)].0;
        if n_cell < MIN_CELL {
            continue;
        }
        let ts: Vec<f64> = modeled
            .iter()
            .filter(|m| m[5] as i64 == rx && m[6] as i64 == tx && m[7] as i64 == mode)
            .map(|m| m[0])
            .collect();
        let vs: Vec<f64> = modeled
            .iter()
            .filter(|m| m[5] as i64 == rx && m[6] as i64 == tx && m[7] as i64 == mode)
            .map(|m| m[1])
            .collect();
        if ts.len() != n_cell {
            continue;
        }
        let mname = match mode {
            11 => "Einweg",
            12 => "Zweiweg",
            13 => "Dreiweg",
            14 => "Dreiweg-koh",
            _ => "sonst",
        };
        let gap = if mode == 13 || mode == 14 {
            600.0
        } else {
            GAP_BLOCK_S
        };
        let gname = if mode == 13 || mode == 14 {
            " (600-s blocks)"
        } else {
            ""
        };
        band_scan(
            &format!("{rx}×{tx} Mode {mode} ({mname}){gname}"),
            &ts,
            &vs,
            gap,
        );
    }
}
