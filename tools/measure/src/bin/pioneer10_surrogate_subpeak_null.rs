use std::collections::HashMap;

use omegaflow::archivar::{
    BodyEphemeris, body_barycenter_position, body_barycenter_velocity, body_fixed_to_icrs_smooth,
    light_time_worldline,
    omni2::{COMP_N1800, parse_bin as parse_omni2},
    parse_ephemeris_binary,
};
use omegaflow::atdf::parse_bin;
use omegaflow::odp::{EARTH, downlink_rate_core, dsn_station, station_velocity};

const SC_BODY: &str = "pioneer10_daily";
const GAP_S: f64 = 5.0 * 86400.0;
const GAP_DAY_S: f64 = 0.1 * 86400.0;
const OVERLAP_BIN_S: f64 = 600.0;
const OVERLAP_TOL_S: f64 = 60.0;
const BAND_LO: f64 = 0.044;
const BAND_HI: f64 = 0.056;
const GRID_STEP: f64 = 0.00005;
const HALF_STEP: f64 = GRID_STEP / 2.0;
const N_SURR: usize = 199;
const PLASMA_K: f64 = 40.31;
const AU: f64 = 1.495978707e11;
const R_SUN: f64 = 6.957e8;
const OMNI2_WINDOW_S: f64 = 3.0 * 3600.0;
const PLASMA_DT: f64 = 3600.0;

fn lin_fit3(x1: &[f64], x2: &[f64], y: &[f64]) -> Option<(f64, f64, f64)> {
    let n = y.len() as f64;
    if n < 3.0 {
        return None;
    }
    let mx1 = x1.iter().sum::<f64>() / n;
    let mx2 = x2.iter().sum::<f64>() / n;
    let my = y.iter().sum::<f64>() / n;
    let mut s11 = 0.0;
    let mut s12 = 0.0;
    let mut s22 = 0.0;
    let mut sy1 = 0.0;
    let mut sy2 = 0.0;
    for i in 0..y.len() {
        let d1 = x1[i] - mx1;
        let d2 = x2[i] - mx2;
        let dy = y[i] - my;
        s11 += d1 * d1;
        s12 += d1 * d2;
        s22 += d2 * d2;
        sy1 += d1 * dy;
        sy2 += d2 * dy;
    }
    let det = s11 * s22 - s12 * s12;
    if det.abs() < 1e-300 {
        return None;
    }
    let a = (sy1 * s22 - sy2 * s12) / det;
    let c = (sy2 * s11 - sy1 * s12) / det;
    let b = my - a * mx1 - c * mx2;
    Some((a, c, b))
}

fn lin_fit3_w(x1: &[f64], x2: &[f64], y: &[f64], w: &[f64]) -> Option<(f64, f64, f64)> {
    let n = y.len() as f64;
    if n < 3.0 {
        return None;
    }
    let mut sw = 0.0;
    for i in 0..y.len() {
        sw += w[i];
    }
    if sw <= 0.0 {
        return None;
    }
    let mx1 = x1.iter().zip(w).map(|(x, &wi)| wi * x).sum::<f64>() / sw;
    let mx2 = x2.iter().zip(w).map(|(x, &wi)| wi * x).sum::<f64>() / sw;
    let my = y.iter().zip(w).map(|(v, &wi)| wi * v).sum::<f64>() / sw;
    let mut s11 = 0.0;
    let mut s12 = 0.0;
    let mut s22 = 0.0;
    let mut sy1 = 0.0;
    let mut sy2 = 0.0;
    for i in 0..y.len() {
        let d1 = x1[i] - mx1;
        let d2 = x2[i] - mx2;
        let dy = y[i] - my;
        s11 += w[i] * d1 * d1;
        s12 += w[i] * d1 * d2;
        s22 += w[i] * d2 * d2;
        sy1 += w[i] * d1 * dy;
        sy2 += w[i] * d2 * dy;
    }
    let det = s11 * s22 - s12 * s12;
    if det.abs() < 1e-300 {
        return None;
    }
    let a = (sy1 * s22 - sy2 * s12) / det;
    let c = (sy2 * s11 - sy1 * s12) / det;
    let b = my - a * mx1 - c * mx2;
    Some((a, c, b))
}

fn rms_of(v: &[f64]) -> f64 {
    if v.is_empty() {
        return f64::NAN;
    }
    (v.iter().map(|x| x * x).sum::<f64>() / v.len() as f64).sqrt()
}

fn lin_fit(xs: &[f64], ys: &[f64]) -> (f64, f64) {
    let n = xs.len() as f64;
    let sx: f64 = xs.iter().sum();
    let sy: f64 = ys.iter().sum();
    let sxx: f64 = xs.iter().map(|x| x * x).sum();
    let sxy: f64 = xs.iter().zip(ys).map(|(x, y)| x * y).sum();
    let denom = n * sxx - sx * sx;
    if denom.abs() < 1e-300 {
        return (0.0, 0.0);
    }
    (
        (n * sxy - sx * sy) / denom,
        (sy - (n * sxy - sx * sy) / denom * sx) / n,
    )
}

fn fixed_effects_cells(
    rates: &[f64],
    refs: &[f64],
    obs: &[f64],
    times: &[f64],
    files: &[i64],
) -> Option<(f64, f64, Vec<f64>, Vec<usize>, Vec<f64>)> {
    let n = rates.len();
    let mut epoch = vec![0usize; n];
    let mut eid = 0usize;
    for i in 0..n {
        if i > 0 && times[i] - times[i - 1] > GAP_S {
            eid += 1;
        }
        epoch[i] = eid;
    }
    let n_epoch = eid + 1;
    let n_files = files.iter().copied().max().map_or(1, |m| m as usize + 1);
    let n_cells = n_epoch * n_files;
    let mut cell = vec![0usize; n];
    for i in 0..n {
        cell[i] = epoch[i] * n_files + files[i] as usize;
    }
    let mut mr = vec![0.0f64; n_cells];
    let mut mf = vec![0.0f64; n_cells];
    let mut mo = vec![0.0f64; n_cells];
    let mut cnt = vec![0usize; n_cells];
    for i in 0..n {
        let c = cell[i];
        mr[c] += rates[i];
        mf[c] += refs[i];
        mo[c] += obs[i];
        cnt[c] += 1;
    }
    for c in 0..n_cells {
        if cnt[c] > 0 {
            mr[c] /= cnt[c] as f64;
            mf[c] /= cnt[c] as f64;
            mo[c] /= cnt[c] as f64;
        }
    }
    let mut cr = vec![0.0f64; n];
    let mut cf = vec![0.0f64; n];
    let mut co = vec![0.0f64; n];
    for i in 0..n {
        let c = cell[i];
        cr[i] = rates[i] - mr[c];
        cf[i] = refs[i] - mf[c];
        co[i] = obs[i] - mo[c];
    }
    let (a, c_coef, _) = lin_fit3(&cr, &cf, &co)?;
    let mut offset = vec![0.0f64; n_cells];
    for c in 0..n_cells {
        offset[c] = mo[c] - a * mr[c] - c_coef * mf[c];
    }
    let mut resid = vec![0.0f64; n];
    for i in 0..n {
        resid[i] = obs[i] - a * rates[i] - c_coef * refs[i] - offset[cell[i]];
    }
    Some((a, c_coef, resid, cell, offset))
}

fn fixed_effects_cells_w(
    rates: &[f64],
    refs: &[f64],
    obs: &[f64],
    times: &[f64],
    files: &[i64],
    w: &[f64],
) -> Option<(f64, f64, Vec<f64>, Vec<usize>, Vec<f64>)> {
    let n = rates.len();
    let mut epoch = vec![0usize; n];
    let mut eid = 0usize;
    for i in 0..n {
        if i > 0 && times[i] - times[i - 1] > GAP_S {
            eid += 1;
        }
        epoch[i] = eid;
    }
    let n_epoch = eid + 1;
    let n_files = files.iter().copied().max().map_or(1, |m| m as usize + 1);
    let n_cells = n_epoch * n_files;
    let mut cell = vec![0usize; n];
    for i in 0..n {
        cell[i] = epoch[i] * n_files + files[i] as usize;
    }
    let mut mr = vec![0.0f64; n_cells];
    let mut mf = vec![0.0f64; n_cells];
    let mut mo = vec![0.0f64; n_cells];
    let mut mw = vec![0.0f64; n_cells];
    for i in 0..n {
        let c = cell[i];
        mr[c] += w[i] * rates[i];
        mf[c] += w[i] * refs[i];
        mo[c] += w[i] * obs[i];
        mw[c] += w[i];
    }
    for c in 0..n_cells {
        if mw[c] > 0.0 {
            mr[c] /= mw[c];
            mf[c] /= mw[c];
            mo[c] /= mw[c];
        }
    }
    let mut cr = vec![0.0f64; n];
    let mut cf = vec![0.0f64; n];
    let mut co = vec![0.0f64; n];
    for i in 0..n {
        let c = cell[i];
        cr[i] = rates[i] - mr[c];
        cf[i] = refs[i] - mf[c];
        co[i] = obs[i] - mo[c];
    }
    let (a, c_coef, _) = lin_fit3_w(&cr, &cf, &co, w)?;
    let mut offset = vec![0.0f64; n_cells];
    for c in 0..n_cells {
        offset[c] = mo[c] - a * mr[c] - c_coef * mf[c];
    }
    let mut resid = vec![0.0f64; n];
    for i in 0..n {
        resid[i] = obs[i] - a * rates[i] - c_coef * refs[i] - offset[cell[i]];
    }
    Some((a, c_coef, resid, cell, offset))
}

fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn plasma_column(r_e: [f64; 3], r_sc: [f64; 3], n1au: f64) -> Option<f64> {
    if !n1au.is_finite() || n1au <= 0.0 {
        return None;
    }
    let d = sub(r_sc, r_e);
    let len2 = dot(d, d);
    if len2 <= 0.0 {
        return None;
    }
    let len = len2.sqrt();
    let lam = -dot(r_e, d) / len2;
    let b2 = dot(r_e, r_e) - lam * lam * len2;
    if b2 < R_SUN * R_SUN {
        return None;
    }
    let b = b2.sqrt();
    let s1 = -lam * len;
    let s2 = (1.0 - lam) * len;
    let col = n1au * AU * AU / b * ((s2 / b).atan() - (s1 / b).atan());
    if col.is_finite() && col > 0.0 {
        Some(col)
    } else {
        None
    }
}

fn plasma_shift(col1: Option<f64>, col2: Option<f64>, f0: f64) -> Option<f64> {
    let (c1, c2) = (col1?, col2?);
    let d = (c2 - c1) / PLASMA_DT;
    if d.is_finite() {
        Some(PLASMA_K / 299792458.0 * d / f0)
    } else {
        None
    }
}

fn nearest_omni(series: &[(f64, f64)], t: f64, window: f64) -> Option<f64> {
    if series.is_empty() {
        return None;
    }
    let idx = series.partition_point(|(tt, _)| *tt < t);
    let mut best: Option<f64> = None;
    let mut best_dt = f64::INFINITY;
    for k in [idx.saturating_sub(1), idx.min(series.len() - 1)] {
        if k >= series.len() {
            continue;
        }
        let dt = (series[k].0 - t).abs();
        if dt <= window && dt < best_dt {
            best_dt = dt;
            best = Some(series[k].1);
        }
    }
    best
}

struct CommonMode {
    pairs: usize,
    cm: Vec<Option<f64>>,
}

fn common_mode(times: &[f64], stations: &[i64], resid: &[f64]) -> CommonMode {
    let n = times.len();
    let mut cm_sum = vec![0.0f64; n];
    let mut cm_cnt = vec![0usize; n];
    let mut pairs = 0usize;
    let mut bins: HashMap<i64, HashMap<i64, Vec<usize>>> = HashMap::new();
    for i in 0..n {
        let b = (times[i] / OVERLAP_BIN_S).floor() as i64;
        bins.entry(b)
            .or_default()
            .entry(stations[i])
            .or_default()
            .push(i);
    }
    for (_, by_station) in &bins {
        let mut ids: Vec<i64> = by_station.keys().copied().collect();
        ids.sort_unstable();
        for x in 0..ids.len() {
            for y in x + 1..ids.len() {
                let a = &by_station[&ids[x]];
                let b = &by_station[&ids[y]];
                let mut used = vec![false; b.len()];
                let mut j = 0usize;
                for &ia in a {
                    while j < b.len() && times[b[j]] < times[ia] - OVERLAP_TOL_S {
                        j += 1;
                    }
                    let mut best: Option<usize> = None;
                    let mut k = j;
                    while k < b.len() && times[b[k]] <= times[ia] + OVERLAP_TOL_S {
                        if !used[k] {
                            let dk = (times[b[k]] - times[ia]).abs();
                            if best.map_or(true, |bk| dk < (times[b[bk]] - times[ia]).abs()) {
                                best = Some(k);
                            }
                        }
                        k += 1;
                    }
                    if let Some(k) = best {
                        used[k] = true;
                        pairs += 1;
                        let mean = 0.5 * (resid[ia] + resid[b[k]]);
                        cm_sum[ia] += mean;
                        cm_cnt[ia] += 1;
                        cm_sum[b[k]] += mean;
                        cm_cnt[b[k]] += 1;
                    }
                }
            }
        }
    }
    let cm: Vec<Option<f64>> = (0..n)
        .map(|i| {
            if cm_cnt[i] > 0 {
                Some(cm_sum[i] / cm_cnt[i] as f64)
            } else {
                None
            }
        })
        .collect();
    CommonMode { pairs, cm }
}

struct LsBasis {
    m: f64,
    ns: usize,
    freqs: Vec<f64>,
    ds: Vec<f64>,
    dc: Vec<f64>,
    ss: Vec<f64>,
    cc: Vec<f64>,
    sc: Vec<f64>,
    det: Vec<f64>,
}

fn build_ls_basis(times: &[f64], flo: f64, fhi: f64, step: f64) -> LsBasis {
    let mut freqs: Vec<f64> = Vec::new();
    let mut f = flo;
    while f <= fhi {
        freqs.push(f);
        f += step;
    }
    let m = times.len() as f64;
    let ns = times.len();
    let nf = freqs.len();
    let mut ds = vec![0.0f64; nf * ns];
    let mut dc = vec![0.0f64; nf * ns];
    let mut ss = vec![0.0f64; nf];
    let mut cc = vec![0.0f64; nf];
    let mut sc = vec![0.0f64; nf];
    let mut det = vec![0.0f64; nf];
    for (fi, &fref) in freqs.iter().enumerate() {
        let mut s = 0.0;
        let mut c = 0.0;
        for &t in times {
            let ph = std::f64::consts::TAU * fref * t;
            s += ph.sin();
            c += ph.cos();
        }
        s /= m;
        c /= m;
        let mut lss = 0.0;
        let mut lcc = 0.0;
        let mut lsc = 0.0;
        let off = fi * ns;
        for (si, &t) in times.iter().enumerate() {
            let ph = std::f64::consts::TAU * fref * t;
            let dsi = ph.sin() - s;
            let dci = ph.cos() - c;
            ds[off + si] = dsi;
            dc[off + si] = dci;
            lss += dsi * dsi;
            lcc += dci * dci;
            lsc += dsi * dci;
        }
        ss[fi] = lss;
        cc[fi] = lcc;
        sc[fi] = lsc;
        det[fi] = lss * lcc - lsc * lsc;
    }
    LsBasis {
        m,
        ns,
        freqs,
        ds,
        dc,
        ss,
        cc,
        sc,
        det,
    }
}

fn ls_power_grid(basis: &LsBasis, vals: &[f64]) -> Vec<(f64, f64)> {
    let nf = basis.freqs.len();
    let ns = basis.ns;
    let mut grid: Vec<(f64, f64)> = Vec::with_capacity(nf);
    for fi in 0..nf {
        let off = fi * ns;
        let mut sy = 0.0;
        let mut cy = 0.0;
        for si in 0..ns {
            sy += basis.ds[off + si] * vals[si];
            cy += basis.dc[off + si] * vals[si];
        }
        let d = basis.det[fi];
        let pow = if d.abs() > 1e-300 {
            let a = (sy * basis.cc[fi] - cy * basis.sc[fi]) / d;
            let b = (cy * basis.ss[fi] - sy * basis.sc[fi]) / d;
            (a * a + b * b) * basis.m / 2.0
        } else {
            0.0
        };
        grid.push((basis.freqs[fi], pow));
    }
    grid
}

fn top_peaks(grid: &[(f64, f64)], k: usize) -> Vec<(f64, f64)> {
    let mut lmax: Vec<(f64, f64)> = Vec::new();
    for i in 1..grid.len() - 1 {
        if grid[i].1 > grid[i - 1].1 && grid[i].1 > grid[i + 1].1 {
            lmax.push(grid[i]);
        }
    }
    lmax.sort_by(|a, b| b.1.total_cmp(&a.1));
    let mut taken: Vec<(f64, f64)> = Vec::new();
    for (f, p) in lmax {
        if taken.iter().all(|(tf, _)| (tf - f).abs() >= 0.0003) {
            taken.push((f, p));
        }
        if taken.len() >= k {
            break;
        }
    }
    taken
}

fn jd_date(tdb: f64) -> String {
    let jd = tdb / 86400.0 + 2451545.0;
    let unix_day = (jd - 2440587.5).round() as i64;
    match omegaflow::spectral::civil_from_days(unix_day) {
        Some((y, m, d)) => format!("{y:04}-{m:02}-{d:02}"),
        None => format!("tdb {tdb:.0} s"),
    }
}

fn year_of(tdb: f64) -> Option<i64> {
    let jd = tdb / 86400.0 + 2451545.0;
    let unix_day = (jd - 2440587.5).round() as i64;
    omegaflow::spectral::civil_from_days(unix_day).map(|(y, _, _)| y as i64)
}

fn main() {
    let sky = "data/spdf.gsfc.nasa.gov/pioneer10_skyfreq_6file.bin";
    let Ok(bytes) = std::fs::read(sky) else {
        eprintln!("retrace: 6-file skyfreq bin void ({sky})");
        return;
    };
    let Some(records) = parse_bin(&bytes) else {
        eprintln!("retrace: 6-file skyfreq bin parse void");
        return;
    };
    let mut eph: HashMap<String, BodyEphemeris> = HashMap::new();
    for body in [EARTH, SC_BODY] {
        let p = format!("data/ssd.jpl.nasa.gov/ephemeris_{body}.bin");
        match std::fs::read(&p)
            .ok()
            .and_then(|d| parse_ephemeris_binary(&d))
        {
            Some(e) => {
                eph.insert(body.to_string(), e);
            }
            None => {
                eprintln!("retrace: {body} ephemeris bin void ({p})");
                return;
            }
        }
    }
    let granule_sc = |t: f64| -> Option<([f64; 3], [f64; 3])> {
        Some((
            body_barycenter_position(SC_BODY, t, &eph)?,
            body_barycenter_velocity(SC_BODY, t, &eph)?,
        ))
    };

    let omni2_path = ["omni2_serie.bin", "omni2_serie_1h.bin"]
        .iter()
        .map(|p| omegaflow::archivar::cache_root().join(p))
        .map(|p| p.to_string_lossy().into_owned())
        .find(|p| std::path::Path::new(p).exists());
    let mut n_series: Vec<(f64, f64)> = Vec::new();
    let mut omni2_window = OMNI2_WINDOW_S;
    if let Some(p) = &omni2_path {
        if let Ok(b) = std::fs::read(p) {
            if let Some(recs) = parse_omni2(&b) {
                n_series = recs
                    .iter()
                    .filter(|r| r.2 == COMP_N1800)
                    .map(|r| (r.0, r.1 * 1e6))
                    .collect();
                n_series.sort_by(|a, b| a.0.total_cmp(&b.0));
            }
        }
    }
    if n_series.len() > 2 {
        let mut gaps: Vec<f64> = Vec::with_capacity(n_series.len() - 1);
        for i in 1..n_series.len() {
            gaps.push(n_series[i].0 - n_series[i - 1].0);
        }
        gaps.sort_by(f64::total_cmp);
        let med_gap = gaps[gaps.len() / 2];
        omni2_window = (2.0 * med_gap).max(OMNI2_WINDOW_S);
    }
    eprintln!(
        "retrace plasma source: {} — N1800: {} records, {}..{}, search window {:.1} h",
        omni2_path.as_deref().unwrap_or("(absent)"),
        n_series.len(),
        if n_series.is_empty() {
            "-".to_string()
        } else {
            jd_date(n_series[0].0)
        },
        if n_series.is_empty() {
            "-".to_string()
        } else {
            jd_date(n_series[n_series.len() - 1].0)
        },
        omni2_window / 3600.0
    );

    let mut times: Vec<f64> = Vec::new();
    let mut obs: Vec<f64> = Vec::new();
    let mut refs: Vec<f64> = Vec::new();
    let mut samplers: Vec<f64> = Vec::new();
    let mut stations: Vec<i64> = Vec::new();
    let mut rates0: Vec<f64> = Vec::new();
    let mut shift_plasma: Vec<Option<f64>> = Vec::new();
    let mut slipped: Vec<bool> = Vec::new();
    let mut strength_ok: Vec<bool> = Vec::new();
    let mut ramp: Vec<f64> = Vec::new();
    let mut files: Vec<i64> = Vec::new();
    let mut no_station = 0usize;
    let mut no_model = 0usize;
    for r in &records {
        let Some((lat, lon, alt)) = dsn_station(r[6] as i64) else {
            no_station += 1;
            continue;
        };
        let t1 = r[0];
        let (Some(rs), Some(vs)) = (
            body_fixed_to_icrs_smooth(EARTH, lat, lon, alt, t1, &eph),
            station_velocity(t1, lat, lon, alt, &eph),
        ) else {
            no_model += 1;
            continue;
        };
        let Some(rate) = downlink_rate_core(t1, rs, vs, &granule_sc) else {
            no_model += 1;
            continue;
        };
        if !rate.is_finite() {
            no_model += 1;
            continue;
        }
        let mut sh_p: Option<f64> = None;
        if !n_series.is_empty() {
            if let Some((r3, t3)) = light_time_worldline(rs, t1, &|t| granule_sc(t).map(|(p, _)| p))
            {
                if let (Some(rs2), Some((r4, _))) = (
                    body_fixed_to_icrs_smooth(EARTH, lat, lon, alt, t1 + PLASMA_DT, &eph),
                    granule_sc(t3 + PLASMA_DT),
                ) {
                    let n1 = nearest_omni(&n_series, t1, omni2_window);
                    let n2 = nearest_omni(&n_series, t1 + PLASMA_DT, omni2_window);
                    if let (Some(v1), Some(v2)) = (n1, n2) {
                        sh_p = plasma_shift(
                            plasma_column(rs, r3, v1),
                            plasma_column(rs2, r4, v2),
                            r[1],
                        );
                    }
                }
            }
        }
        times.push(t1);
        obs.push(r[1]);
        refs.push(r[2]);
        samplers.push(r[3]);
        stations.push(r[6] as i64);
        rates0.push(rate);
        shift_plasma.push(sh_p);
        slipped.push(r[9] != 0.0);
        strength_ok.push(r[10] < 0.0);
        ramp.push(r[11]);
        files.push(r[12] as i64);
    }
    let n_raw = records.len();
    let no_slipped = slipped.iter().filter(|&&s| s).count();
    let no_strength = slipped
        .iter()
        .zip(&strength_ok)
        .filter(|&(&s, &ok)| !s && !ok)
        .count();
    eprintln!(
        "retrace 6-file: {} records, {} modelable ({} without station, {} without model), {} slipped (field 76), {} without strength (field 78 >= 0)",
        n_raw,
        times.len(),
        no_station,
        no_model,
        no_slipped,
        no_strength
    );

    let keep: Vec<usize> = (0..times.len())
        .filter(|&i| !slipped[i] && strength_ok[i])
        .collect();
    times = keep.iter().map(|&i| times[i]).collect();
    obs = keep.iter().map(|&i| obs[i]).collect();
    refs = keep.iter().map(|&i| refs[i]).collect();
    samplers = keep.iter().map(|&i| samplers[i]).collect();
    stations = keep.iter().map(|&i| stations[i]).collect();
    rates0 = keep.iter().map(|&i| rates0[i]).collect();
    ramp = keep.iter().map(|&i| ramp[i]).collect();
    files = keep.iter().map(|&i| files[i]).collect();

    let mut n = times.len();
    eprintln!(
        "retrace 6-file: after gates 0/0b {} samples, {}..{}, stations {:?}",
        n,
        jd_date(times[0]),
        jd_date(times[n - 1]),
        {
            let mut s: Vec<i64> = stations.iter().copied().collect();
            s.sort_unstable();
            s.dedup();
            s
        }
    );

    let Some((_, _, resid0, _, _)) = fixed_effects_cells(&rates0, &refs, &obs, &times, &files)
    else {
        eprintln!("retrace: base fit void");
        return;
    };
    let n_pre_mask = n;

    {
        let mut seg_rms: Vec<(usize, usize, f64)> = Vec::new();
        let mut lo = 0usize;
        while lo < n {
            let mut hi = lo + 1;
            while hi < n && times[hi] - times[hi - 1] < GAP_DAY_S {
                hi += 1;
            }
            if hi - lo >= 20 {
                let sq: f64 = (lo..hi).map(|i| resid0[i] * resid0[i]).sum();
                seg_rms.push((lo, hi, (sq / (hi - lo) as f64).sqrt()));
            }
            lo = hi;
        }
        if seg_rms.len() >= 10 {
            let mut r: Vec<f64> = seg_rms.iter().map(|s| s.2).collect();
            r.sort_by(f64::total_cmp);
            let p90 = r[r.len() * 9 / 10];
            let gate = 4.0 * p90;
            let masked: Vec<(usize, usize)> = seg_rms
                .iter()
                .filter(|s| s.2 > gate)
                .map(|s| (s.0, s.1))
                .collect();
            let n_masked: usize = masked.iter().map(|(a, b)| b - a).sum();
            let mut marked = vec![false; n];
            for (a, b) in &masked {
                for i in *a..*b {
                    marked[i] = true;
                }
            }
            if n_masked > 0 && n_masked < n {
                eprintln!(
                    "retrace Deduction 10 segment mask: gate 4xp90 = {gate:.3e} Hz (p90 {p90:.3e}, {} segments) — {} segments ({} samples) discarded",
                    seg_rms.len(),
                    masked.len(),
                    n_masked
                );
                let keep: Vec<usize> = (0..n).filter(|&i| !marked[i]).collect();
                times = keep.iter().map(|&i| times[i]).collect();
                obs = keep.iter().map(|&i| obs[i]).collect();
                refs = keep.iter().map(|&i| refs[i]).collect();
                samplers = keep.iter().map(|&i| samplers[i]).collect();
                stations = keep.iter().map(|&i| stations[i]).collect();
                rates0 = keep.iter().map(|&i| rates0[i]).collect();
                ramp = keep.iter().map(|&i| ramp[i]).collect();
                files = keep.iter().map(|&i| files[i]).collect();
            }
        }
    }
    n = times.len();
    let Some((_, _, resid0, _, _)) = fixed_effects_cells(&rates0, &refs, &obs, &times, &files)
    else {
        eprintln!("retrace: base refit void");
        return;
    };
    let rms0 = rms_of(&resid0);
    eprintln!(
        "retrace: {} samples ({} before segment mask), base residual-RMS {rms0:.3e} Hz",
        n, n_pre_mask
    );

    let mut weights: Vec<f64> = vec![1.0; n];
    {
        let mut v_class = [0.0f64; 2];
        let mut n_class = [0usize; 2];
        for i in 0..n {
            let c = if samplers[i] < 10.0 { 0 } else { 1 };
            v_class[c] += resid0[i] * resid0[i];
            n_class[c] += 1;
        }
        if n_class[0] > 3 && n_class[1] > 3 && v_class[0] > 0.0 && v_class[1] > 0.0 {
            let v0 = v_class[0] / n_class[0] as f64;
            let v1 = v_class[1] / n_class[1] as f64;
            let w0 = 1.0 / v0;
            let w1 = 1.0 / v1;
            for i in 0..n {
                weights[i] = if samplers[i] < 10.0 { w0 } else { w1 };
            }
            let wsum: f64 = weights.iter().sum();
            if wsum > 0.0 {
                let norm = n as f64 / wsum;
                for wgt in &mut weights {
                    *wgt *= norm;
                }
            }
            eprintln!(
                "retrace Deduction 11 noise weighting: class RMS 1-s {:.3e} / 60-s {:.3e} Hz — weights 1:{:.2}",
                v0.sqrt(),
                v1.sqrt(),
                w0 / w1
            );
        }
    }
    let Some((a0, c0, resid0, cell0, offset0)) =
        fixed_effects_cells_w(&rates0, &refs, &obs, &times, &files, &weights)
    else {
        eprintln!("retrace: weighted base fit void");
        return;
    };
    let rms0 = {
        let mut sw = 0.0;
        let mut sq = 0.0;
        for i in 0..n {
            sw += weights[i];
            sq += weights[i] * resid0[i] * resid0[i];
        }
        (sq / sw).sqrt()
    };
    eprintln!(
        "retrace: weighted base residual-RMS {rms0:.3e} Hz — A {a0:.4e} Hz/(m/s), C {c0:.4e}"
    );

    let cm = common_mode(&times, &stations, &resid0);
    let mut obs_b = obs.clone();
    if cm.pairs > 0 {
        for i in 0..n {
            if let Some(v) = cm.cm[i] {
                obs_b[i] = a0 * rates0[i] + c0 * refs[i] + offset0[cell0[i]] + v;
            }
        }
        eprintln!(
            "retrace Deduction 1 common-mode: {} pairs isolated",
            cm.pairs
        );
    } else {
        eprintln!("retrace Deduction 1 common-mode: 0 overlapping windows (0 honored)");
    }
    {
        let mut bins: HashMap<i64, std::collections::HashSet<i64>> = HashMap::new();
        for i in 0..n {
            bins.entry((times[i] / OVERLAP_BIN_S).floor() as i64)
                .or_default()
                .insert(stations[i]);
        }
        let multi = bins.values().filter(|s| s.len() >= 2).count();
        let multi_stations: Vec<String> = bins
            .values()
            .filter(|s| s.len() >= 2)
            .map(|s| {
                let mut v: Vec<i64> = s.iter().copied().collect();
                v.sort_unstable();
                format!("{v:?}")
            })
            .collect();
        let mut idx: Vec<usize> = (0..n).collect();
        idx.sort_by(|&a, &b| times[a].total_cmp(&times[b]));
        let mut min_diff_st = f64::INFINITY;
        let mut min_pair = (0i64, 0i64);
        let mut prev: HashMap<i64, f64> = HashMap::new();
        for &i in &idx {
            let st = stations[i];
            let t = times[i];
            for (&ost, &ot) in &prev {
                if ost == st {
                    continue;
                }
                let d = (t - ot).abs();
                if d < min_diff_st {
                    min_diff_st = d;
                    min_pair = (ost, st);
                }
            }
            prev.insert(st, t);
        }
        eprintln!(
            "retrace common-mode diagnosis: {multi} of {} 600-s bins hold 2+ stations (sets {multi_stations:?}); min inter-station |Δt| = {min_diff_st:.3e} s ({min_pair:?})",
            bins.len()
        );
    }

    let mut obs_c = obs_b.clone();
    let mut n_plasma = 0usize;
    let mut sum_abs_plasma = 0.0f64;
    let mut max_abs_plasma = 0.0f64;
    for i in 0..n {
        if let Some(sh) = shift_plasma[i] {
            obs_c[i] -= sh;
            n_plasma += 1;
            sum_abs_plasma += sh.abs();
            max_abs_plasma = max_abs_plasma.max(sh.abs());
        }
    }
    eprintln!(
        "retrace Deduction 2 TEC: 0 of {n} samples carry a map pair — the GIM maps begin in 1998, the ATDF era lies before it; the TEC deduction stays empty (0 honored), discarded instead of averaged"
    );
    if n_plasma > 0 {
        let mean_abs = sum_abs_plasma / n_plasma as f64;
        eprintln!(
            "retrace Deduction 3 plasma: OMNI2 carries {n_plasma} of {n} samples — |Δf| mean {mean_abs:.3e} Hz, max {max_abs_plasma:.3e} Hz"
        );
    } else {
        eprintln!(
            "retrace Deduction 3 plasma: 0 of {n} samples carry OMNI2 — the deduction stays empty (0 honored)"
        );
    }
    let Some((_, _, resid_c, _, _)) =
        fixed_effects_cells_w(&rates0, &refs, &obs_c, &times, &files, &weights)
    else {
        eprintln!("retrace: media fit void");
        return;
    };

    let mut obs_d = obs_c.clone();
    let n_ramped = ramp.iter().filter(|&&r| r != 0.0).count();
    if n_ramped > 100 {
        let mut seg_mid = vec![0.0f64; n];
        {
            let mut lo = 0usize;
            while lo < n {
                let mut hi = lo + 1;
                while hi < n && times[hi] - times[hi - 1] < GAP_DAY_S {
                    hi += 1;
                }
                let mid = 0.5 * (times[lo] + times[hi - 1]);
                for i in lo..hi {
                    seg_mid[i] = mid;
                }
                lo = hi;
            }
        }
        let xs: Vec<f64> = (0..n)
            .filter(|&i| ramp[i] != 0.0)
            .map(|i| ramp[i] * (times[i] - seg_mid[i]))
            .collect();
        let ys: Vec<f64> = (0..n)
            .filter(|&i| ramp[i] != 0.0)
            .map(|i| resid_c[i])
            .collect();
        let (k_ramp, _) = lin_fit(&xs, &ys);
        for i in 0..n {
            obs_d[i] -= k_ramp * ramp[i] * (times[i] - seg_mid[i]);
        }
        eprintln!(
            "retrace Deduction 5 Ramp: {} of {} samples carry RAMP_RATE (field 112), k = {k_ramp:.4e}",
            n_ramped, n
        );
    } else {
        eprintln!(
            "retrace Deduction 5 Ramp: {} of {} samples carry RAMP_RATE (0 honored)",
            n_ramped, n
        );
    }
    let Some((_, _, resid_d, _, _)) =
        fixed_effects_cells_w(&rates0, &refs, &obs_d, &times, &files, &weights)
    else {
        eprintln!("retrace: ramp fit void");
        return;
    };

    let mut slope_seg = vec![0.0f64; n];
    let mut seg_mid7 = vec![0.0f64; n];
    {
        let mut lo = 0usize;
        while lo < n {
            let mut hi = lo + 1;
            while hi < n && times[hi] - times[hi - 1] < GAP_DAY_S {
                hi += 1;
            }
            if hi - lo >= 20 {
                let mid = 0.5 * (times[lo] + times[hi - 1]);
                let xs: Vec<f64> = (lo..hi).map(|i| times[i] - mid).collect();
                let ys: Vec<f64> = (lo..hi).map(|i| resid_d[i]).collect();
                let (b, _) = lin_fit(&xs, &ys);
                for i in lo..hi {
                    slope_seg[i] = b;
                    seg_mid7[i] = mid;
                }
            }
            lo = hi;
        }
    }
    let mut obs_e = obs_d.clone();
    for i in 0..n {
        obs_e[i] -= slope_seg[i] * (times[i] - seg_mid7[i]);
    }
    let Some((_, _, resid_e, _, _)) =
        fixed_effects_cells_w(&rates0, &refs, &obs_e, &times, &files, &weights)
    else {
        eprintln!("retrace: daily-curve fit void");
        return;
    };
    let rms_e = rms_of(&resid_e);
    eprintln!(
        "retrace Deduction 7 daily-curve: corrected residual-RMS {rms_e:.3e} Hz (ramp stage {:.3e} Hz)",
        rms_of(&resid_d)
    );

    eprintln!(
        "retrace chain applied: 0 slipped, 0b strength, 10 segment mask, 11 weighting, 1 common-mode, 2 TEC empty (no GIM before 1998), 3 plasma (OMNI2 N1800), 5 ramp, 7 daily-curve -> resid_e"
    );

    let claims: [(i64, i64, f64, &str); 4] = [
        (14, 1988, 0.04575, "st14 45.75 (1988)"),
        (43, 1988, 0.05155, "st43 51.55 (1988)"),
        (63, 1988, 0.04735, "st63 47.35 (1988)"),
        (63, 1992, 0.04695, "st63 46.95 (1992)"),
    ];

    struct Cell {
        st: i64,
        year: i64,
        ts: Vec<f64>,
        vs: Vec<f64>,
    }
    let mut cells: Vec<Cell> = Vec::new();
    for st in [14i64, 43, 63] {
        let mut by_year: HashMap<i64, (Vec<f64>, Vec<f64>)> = HashMap::new();
        for i in 0..n {
            if stations[i] != st || samplers[i] >= 10.0 {
                continue;
            }
            if let Some(y) = year_of(times[i]) {
                let e = by_year.entry(y).or_insert_with(|| (Vec::new(), Vec::new()));
                e.0.push(times[i]);
                e.1.push(resid_e[i]);
            }
        }
        let mut yrs: Vec<i64> = by_year.keys().copied().collect();
        yrs.sort_unstable();
        for y in yrs {
            let (ts, vs) = by_year.remove(&y).unwrap();
            if ts.len() >= 200 {
                cells.push(Cell {
                    st,
                    year: y,
                    ts,
                    vs,
                });
            }
        }
    }

    let mut rng: u64 = 0x9e37_79b9_7f4a_7c15;
    let mut real_hits = 0usize;
    let mut expected_hits = 0.0f64;
    let mut expected_var = 0.0f64;
    let mut seen: [bool; 4] = [false; 4];
    for cell in &cells {
        let basis = build_ls_basis(&cell.ts, BAND_LO, BAND_HI, GRID_STEP);
        let real_grid = ls_power_grid(&basis, &cell.vs);
        let real_top5 = top_peaks(&real_grid, 5);
        let floor = {
            let mut pows: Vec<f64> = real_grid.iter().map(|(_, p)| *p).collect();
            pows.sort_by(f64::total_cmp);
            pows[pows.len() / 2]
        };
        let tops_str: Vec<String> = real_top5
            .iter()
            .map(|(f, p)| format!("{:.2} mHz ({:.1}x)", f * 1000.0, p / floor))
            .collect();
        let mut cell_hits = 0usize;
        let mut has_claim = false;
        let mut hit_labels: Vec<&str> = Vec::new();
        for (ci, (cst, cy, cf, label)) in claims.iter().enumerate() {
            if *cst != cell.st || *cy != cell.year {
                continue;
            }
            seen[ci] = true;
            has_claim = true;
            if real_top5.iter().any(|(f, _)| (f - cf).abs() <= HALF_STEP) {
                cell_hits += 1;
                real_hits += 1;
                hit_labels.push(label);
            }
        }
        let f32v: Vec<f32> = cell.vs.iter().map(|&x| x as f32).collect();
        let mut rates: Vec<f64> = Vec::with_capacity(N_SURR);
        for _ in 0..N_SURR {
            let surr = omegaflow::te::phase_randomized_surrogate(&f32v, &mut rng);
            let surr64: Vec<f64> = surr.iter().map(|&x| x as f64).collect();
            let grid = ls_power_grid(&basis, &surr64);
            let t5 = top_peaks(&grid, 5);
            let covered = t5.len() as f64 * 2.0 * HALF_STEP;
            rates.push(covered / (BAND_HI - BAND_LO));
        }
        let n_r = rates.len() as f64;
        let mean = rates.iter().sum::<f64>() / n_r;
        let var = rates.iter().map(|&r| (r - mean) * (r - mean)).sum::<f64>() / n_r;
        let sd = var.sqrt();
        if has_claim {
            expected_hits += mean;
            expected_var += mean * (1.0 - mean);
        }
        eprintln!(
            "null st{} {} n={}: top-5 [{}]; claims hit {} {:?}; null hit rate {mean:.4} ± {sd:.4}",
            cell.st,
            cell.year,
            cell.ts.len(),
            tops_str.join(" | "),
            cell_hits,
            hit_labels
        );
    }
    for (ci, (_, _, _, label)) in claims.iter().enumerate() {
        if !seen[ci] {
            eprintln!("null {label}: cell absent or n < 200 (0 honored)");
        }
    }
    let expected_sd = expected_var.sqrt();
    let threshold = expected_hits + 2.0 * expected_sd;
    let verdict = if real_hits as f64 > threshold {
        "SELECTION"
    } else {
        "CHANCE"
    };
    eprintln!(
        "null aggregate: paper hits {real_hits}/4 within ±0.025 mHz of real top-5; null expected {expected_hits:.4} ± {expected_sd:.4} (mean+2σ = {threshold:.4}); verdict = {verdict}"
    );
}
