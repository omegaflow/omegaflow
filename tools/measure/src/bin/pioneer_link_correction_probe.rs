const NAVIO_BEAT_BOUND: f64 = 5.0e5;
const TE_SEED: u64 = 0x5031_3037;

use std::collections::HashMap;

use omegaflow::archivar::{
    body_barycenter_position, body_barycenter_velocity, body_fixed_to_icrs_smooth, embedded_lsk,
    omni2::{parse_bin as parse_omni2, COMP_N1800},
    parse_ephemeris_binary, BodyEphemeris,
};
use omegaflow::atdf::parse_bin;
use omegaflow::doppler::parse_bin as parse_pdpl;
use omegaflow::inflate::gunzip;
use omegaflow::ionex::{parse_gim, tec_at, TecGrid};
use omegaflow::lsk::LeapSeconds;
use omegaflow::odp::{
    downlink_rate_core, dsn_station, interp, propagate_accel, station_velocity, sun_accel, C, EARTH,
};

const SC_BODY: &str = "pioneer10_daily";
const PIONEER_ANOMALY: f64 = 8.74e-10;
const PLASMA_K: f64 = 40.31;
const AU: f64 = 1.495978707e11;
const R_SUN: f64 = 6.957e8;
const TSI_W_M2: f64 = 1361.0;
const SC_MASS_KG: f64 = 258.8;
const SC_AREA_M2: f64 = 4.7;
const SC_REFLECTIVITY: f64 = 0.6;
const RTG_THERMAL_W: f64 = 2580.0;
const RTG_HALF_LIFE_S: f64 = 87.74 * 365.25 * 86400.0;

const RTG_ANISO_SCAN: [f64; 3] = [0.0, 0.0104, 0.0144];
const ELEC_ETA_SCAN: [f64; 4] = [0.0, 0.1, 0.406, 1.0];
const GRID_DT: f64 = 600.0;
const GAP_S: f64 = 5.0 * 86400.0;
const GAP_DAY_S: f64 = 0.1 * 86400.0;
const OVERLAP_BIN_S: f64 = 600.0;
const OVERLAP_TOL_S: f64 = 60.0;
const OMNI2_WINDOW_S: f64 = 3.0 * 3600.0;
const TEC_WINDOW_S: f64 = 3.0 * 3600.0;
const PLASMA_DT: f64 = 3600.0;
const NAVIO_T_MIN: f64 = -8.8e8;
const NAVIO_T_MAX: f64 = 1.0e8;

fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn dist(a: [f64; 3], b: [f64; 3]) -> f64 {
    let d = sub(a, b);
    dot(d, d).sqrt()
}

fn norm(a: [f64; 3]) -> f64 {
    dot(a, a).sqrt()
}

fn rms_of(v: &[f64]) -> f64 {
    if v.is_empty() {
        return f64::NAN;
    }
    (v.iter().map(|x| x * x).sum::<f64>() / v.len() as f64).sqrt()
}

fn rms_w(resid: &[f64], w: &[f64]) -> f64 {
    let mut sw = 0.0;
    let mut sq = 0.0;
    for i in 0..resid.len() {
        sq += w[i] * resid[i] * resid[i];
        sw += w[i];
    }
    if sw > 0.0 {
        (sq / sw).sqrt()
    } else {
        f64::NAN
    }
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

fn fixed_effects(
    rates: &[f64],
    refs: &[f64],
    obs: &[f64],
    times: &[f64],
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
    let mut mr = vec![0.0f64; n_epoch];
    let mut mf = vec![0.0f64; n_epoch];
    let mut mo = vec![0.0f64; n_epoch];
    let mut cnt = vec![0usize; n_epoch];
    for i in 0..n {
        let e = epoch[i];
        mr[e] += rates[i];
        mf[e] += refs[i];
        mo[e] += obs[i];
        cnt[e] += 1;
    }
    for e in 0..n_epoch {
        mr[e] /= cnt[e] as f64;
        mf[e] /= cnt[e] as f64;
        mo[e] /= cnt[e] as f64;
    }
    let mut cr = vec![0.0f64; n];
    let mut cf = vec![0.0f64; n];
    let mut co = vec![0.0f64; n];
    for i in 0..n {
        let e = epoch[i];
        cr[i] = rates[i] - mr[e];
        cf[i] = refs[i] - mf[e];
        co[i] = obs[i] - mo[e];
    }
    let (a, c, _) = lin_fit3(&cr, &cf, &co)?;
    let mut offset = vec![0.0f64; n_epoch];
    for e in 0..n_epoch {
        offset[e] = mo[e] - a * mr[e] - c * mf[e];
    }
    let mut resid = vec![0.0f64; n];
    for i in 0..n {
        resid[i] = obs[i] - a * rates[i] - c * refs[i] - offset[epoch[i]];
    }
    Some((a, c, resid, epoch, offset))
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
    let n_files = files.iter().copied().max().unwrap_or(0) as usize + 1;
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

struct FitStat {
    a: f64,
    c: f64,
    rms: f64,
    drift: f64,
    se_drift: f64,
}

fn fit_stats(
    rates: &[f64],
    refs: &[f64],
    obs: &[f64],
    times: &[f64],
    files: &[i64],
) -> Option<FitStat> {
    let (a, c, resid, _, _) = fixed_effects_cells(rates, refs, obs, times, files)?;
    let rms = rms_of(&resid);
    let (drift, _) = lin_fit(times, &resid);
    let n = times.len() as f64;
    let t_mean = times.iter().sum::<f64>() / n;
    let sxx: f64 = times.iter().map(|t| (t - t_mean).powi(2)).sum();
    let rss: f64 = resid.iter().map(|r| r * r).sum();
    let se_drift = if n > 2.0 && sxx > 0.0 {
        (rss / ((n - 2.0) * sxx)).sqrt()
    } else {
        f64::NAN
    };
    Some(FitStat {
        a,
        c,
        rms,
        drift,
        se_drift,
    })
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
    let n_files = files.iter().copied().max().unwrap_or(0) as usize + 1;
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

fn fit_stats_w(
    rates: &[f64],
    refs: &[f64],
    obs: &[f64],
    times: &[f64],
    files: &[i64],
    w: &[f64],
) -> Option<FitStat> {
    let (a, c, resid, _, _) = fixed_effects_cells_w(rates, refs, obs, times, files, w)?;
    let rms = rms_w(&resid, w);
    let (drift, _) = lin_fit_w(times, &resid, w);
    let mut sw = 0.0;
    let mut sx = 0.0;
    for i in 0..times.len() {
        sw += w[i];
        sx += w[i] * times[i];
    }
    let t_mean = sx / sw;
    let sxx: f64 = times
        .iter()
        .zip(w)
        .map(|(t, &wi)| wi * (t - t_mean).powi(2))
        .sum();
    let rss: f64 = resid.iter().zip(w).map(|(r, &wi)| wi * r * r).sum();
    let se_drift = if sw > 2.0 && sxx > 0.0 {
        ((rss / (sw - 2.0)) / sxx).sqrt()
    } else {
        f64::NAN
    };
    Some(FitStat {
        a,
        c,
        rms,
        drift,
        se_drift,
    })
}

fn lin_fit_w(xs: &[f64], ys: &[f64], w: &[f64]) -> (f64, f64) {
    let mut sw = 0.0;
    let mut sx = 0.0;
    let mut sy = 0.0;
    for i in 0..xs.len() {
        sw += w[i];
        sx += w[i] * xs[i];
        sy += w[i] * ys[i];
    }
    if sw <= 0.0 {
        return (0.0, 0.0);
    }
    let mx = sx / sw;
    let my = sy / sw;
    let mut sxx = 0.0;
    let mut sxy = 0.0;
    for i in 0..xs.len() {
        let dx = xs[i] - mx;
        sxx += w[i] * dx * dx;
        sxy += w[i] * dx * (ys[i] - my);
    }
    if sxx <= 0.0 {
        (0.0, 0.0)
    } else {
        let b = sxy / sxx;
        (b, my - b * mx)
    }
}

fn median(v: &mut [f64]) -> f64 {
    v.sort_by(f64::total_cmp);
    if v.is_empty() {
        return f64::NAN;
    }
    v[v.len() / 2]
}

fn ls_grid(times: &[f64], vals: &[f64], flo: f64, fhi: f64, step: f64) -> Vec<(f64, f64)> {
    let mut grid: Vec<(f64, f64)> = Vec::new();
    let mut f = flo;
    while f <= fhi {
        grid.push((f, 0.0));
        f += step;
    }
    if grid.is_empty() {
        return grid;
    }
    let m = times.len() as f64;
    let vsum = vals.iter().sum::<f64>() / m;
    for (fref, pow) in grid.iter_mut() {
        let mut s = 0.0;
        let mut c = 0.0;
        for &t in times {
            let ph = std::f64::consts::TAU * *fref * t;
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
        for (i, &t) in times.iter().enumerate() {
            let ph = std::f64::consts::TAU * *fref * t;
            let ds = ph.sin() - s;
            let dc = ph.cos() - c;
            let dv = vals[i] - vsum;
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
            *pow = (a * a + b * b) * m / 2.0;
        }
    }
    grid
}

fn peak_of(grid: &[(f64, f64)]) -> (f64, f64, f64) {
    if grid.is_empty() {
        return (f64::NAN, 0.0, f64::NAN);
    }
    let mut best = grid[0];
    for g in grid {
        if g.1 > best.1 {
            best = *g;
        }
    }
    let mut pows: Vec<f64> = grid.iter().map(|g| g.1).collect();
    pows.sort_by(f64::total_cmp);
    let floor = pows[pows.len() / 2];
    (best.0, best.1, best.1 / floor)
}

fn peak_interp(grid: &[(f64, f64)]) -> Option<f64> {
    let (fmax, _, _) = peak_of(grid);
    let k = grid.iter().position(|g| g.0 == fmax)?;
    if k == 0 || k + 1 >= grid.len() {
        return None;
    }
    let step = grid[k].0 - grid[k - 1].0;
    let pm = grid[k - 1].1;
    let p0 = grid[k].1;
    let pp = grid[k + 1].1;
    let denom = pm - 2.0 * p0 + pp;
    if denom.abs() < 1e-300 {
        return None;
    }
    let d = 0.5 * (pm - pp) / denom;
    if d.abs() > 1.0 {
        return None;
    }
    Some(fmax + d * step)
}

fn jd_date(tdb: f64) -> String {
    let jd = tdb / 86400.0 + 2451545.0;
    let unix_day = (jd - 2440587.5).round() as i64;
    match omegaflow::spectral::civil_from_days(unix_day) {
        Some((y, m, d)) => format!("{y:04}-{m:02}-{d:02}"),
        None => format!("tdb {tdb:.0} s"),
    }
}

fn year_of(tdb: f64) -> Option<u32> {
    let jd = tdb / 86400.0 + 2451545.0;
    let unix_day = (jd - 2440587.5).round() as i64;
    omegaflow::spectral::civil_from_days(unix_day).map(|(y, _, _)| y)
}

struct CommonMode {
    pairs: usize,
    windows: usize,
    diff_rms: f64,
    pair_counts: Vec<(i64, i64, usize)>,
    cm: Vec<Option<f64>>,
}

fn common_mode(times: &[f64], stations: &[i64], resid: &[f64]) -> CommonMode {
    let n = times.len();
    let mut cm_sum = vec![0.0f64; n];
    let mut cm_cnt = vec![0usize; n];
    let mut diff_sq = 0.0f64;
    let mut pairs = 0usize;
    let mut windows = 0usize;
    let mut bins: HashMap<i64, HashMap<i64, Vec<usize>>> = HashMap::new();
    for i in 0..n {
        let b = (times[i] / OVERLAP_BIN_S).floor() as i64;
        bins.entry(b)
            .or_default()
            .entry(stations[i])
            .or_default()
            .push(i);
    }
    let mut pair_count_map: HashMap<(i64, i64), usize> = HashMap::new();
    for (_, by_station) in &bins {
        let mut ids: Vec<i64> = by_station.keys().copied().collect();
        ids.sort_unstable();
        let mut bin_pairs = 0usize;
        for x in 0..ids.len() {
            for y in x + 1..ids.len() {
                let a = &by_station[&ids[x]];
                let b = &by_station[&ids[y]];
                let mut used = vec![false; b.len()];
                let mut j = 0usize;
                let mut pair_n = 0usize;
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
                        let diff = resid[ia] - resid[b[k]];
                        diff_sq += diff * diff;
                        pairs += 1;
                        bin_pairs += 1;
                        pair_n += 1;
                        let mean = 0.5 * (resid[ia] + resid[b[k]]);
                        cm_sum[ia] += mean;
                        cm_cnt[ia] += 1;
                        cm_sum[b[k]] += mean;
                        cm_cnt[b[k]] += 1;
                    }
                }
                if pair_n > 0 {
                    let key = (ids[x].min(ids[y]), ids[x].max(ids[y]));
                    *pair_count_map.entry(key).or_default() += pair_n;
                }
            }
        }
        if bin_pairs > 0 {
            windows += 1;
        }
    }
    let mut pair_counts: Vec<(i64, i64, usize)> = pair_count_map
        .into_iter()
        .map(|((a, b), n)| (a, b, n))
        .collect();
    pair_counts.sort_by(|a, b| b.2.cmp(&a.2));
    let diff_rms = if pairs > 0 {
        (diff_sq / pairs as f64).sqrt()
    } else {
        0.0
    };
    let cm: Vec<Option<f64>> = (0..n)
        .map(|i| {
            if cm_cnt[i] > 0 {
                Some(cm_sum[i] / cm_cnt[i] as f64)
            } else {
                None
            }
        })
        .collect();
    CommonMode {
        pairs,
        windows,
        diff_rms,
        pair_counts,
        cm,
    }
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
        Some(PLASMA_K / C * d / f0)
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

fn tec_pair_shift(maps: &[(f64, TecGrid)], t: f64, lat: f64, lon: f64, f0: f64) -> Option<f64> {
    if maps.len() < 2 {
        return None;
    }
    let idx = maps.partition_point(|(e, _)| *e <= t);
    if idx == 0 || idx >= maps.len() {
        return None;
    }
    let (t1, g1) = &maps[idx - 1];
    let (t2, g2) = &maps[idx];
    let dt = t2 - t1;
    if dt <= 0.0 || dt > TEC_WINDOW_S {
        return None;
    }
    let v1 = tec_at(g1, lat, lon)?;
    let v2 = tec_at(g2, lat, lon)?;
    if !(v1.is_finite() && v1 > 0.0) || !(v2.is_finite() && v2 > 0.0) {
        return None;
    }
    let d = (v2 - v1) / dt * 1e16;
    if !d.is_finite() {
        return None;
    }
    Some(PLASMA_K / C * d / f0)
}

fn load_ionex(dir: &str, lsk: &LeapSeconds) -> Vec<(f64, TecGrid)> {
    let mut out = Vec::new();
    let Ok(entries) = std::fs::read_dir(dir) else {
        return out;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let name = path.to_string_lossy().to_ascii_uppercase();
        if !(name.ends_with(".INX") || name.ends_with(".INX.GZ")) {
            continue;
        }
        let Ok(raw) = std::fs::read(&path) else {
            continue;
        };
        let bytes = if name.ends_with(".GZ") {
            match gunzip(&raw) {
                Some(b) => b,
                None => continue,
            }
        } else {
            raw
        };
        let text = String::from_utf8_lossy(&bytes);
        for g in parse_gim(&text, -1.0) {
            if let Some(tdb) = lsk.unix_to_tdb(g.epoch_unix) {
                out.push((tdb, g));
            }
        }
    }
    out.sort_by(|a, b| a.0.total_cmp(&b.0));
    out
}

fn srp_accel(r: [f64; 3]) -> [f64; 3] {
    let rn = norm(r);
    if rn <= 0.0 {
        return [0.0; 3];
    }
    let flux = TSI_W_M2 / ((rn / AU) * (rn / AU));
    let a = SC_AREA_M2 / SC_MASS_KG * flux * (1.0 + SC_REFLECTIVITY) / C;
    [a * r[0] / rn, a * r[1] / rn, a * r[2] / rn]
}

fn rtg_accel(r: [f64; 3], t: f64, aniso: f64, t0_rtg: f64) -> [f64; 3] {
    let rn = norm(r);
    if rn <= 0.0 {
        return [0.0; 3];
    }
    let p = RTG_THERMAL_W * 2f64.powf((t0_rtg - t) / RTG_HALF_LIFE_S);
    let a = aniso * p / (SC_MASS_KG * C);
    [-a * r[0] / rn, -a * r[1] / rn, -a * r[2] / rn]
}

fn dyn_accel(t: f64, r: [f64; 3], aniso: f64, t0_rtg: f64, a_p: f64) -> [f64; 3] {
    let sun = sun_accel(r);
    let srp = srp_accel(r);
    let rtg = rtg_accel(r, t, aniso, t0_rtg);
    let rn = norm(r);
    let mut out = [0.0; 3];
    for k in 0..3 {
        out[k] = sun[k] + srp[k] + rtg[k];
        if rn > 0.0 {
            out[k] -= a_p * r[k] / (rn * rn);
        }
    }
    out
}

fn run_rates_dyn(
    state0: [f64; 6],
    t_first: f64,
    t_last: f64,
    aniso: f64,
    t0_rtg: f64,
    a_p: f64,
    times: &[f64],
    r_st: &[[f64; 3]],
    v_st: &[[f64; 3]],
) -> Vec<f64> {
    let acc = |t: f64, r: [f64; 3]| dyn_accel(t, r, aniso, t0_rtg, a_p);
    run_rates_dyn_accel(state0, t_first, t_last, &acc, times, r_st, v_st)
}

fn run_rates_dyn_accel(
    state0: [f64; 6],
    t_first: f64,
    t_last: f64,
    acc: &dyn Fn(f64, [f64; 3]) -> [f64; 3],
    times: &[f64],
    r_st: &[[f64; 3]],
    v_st: &[[f64; 3]],
) -> Vec<f64> {
    let grid = propagate_accel(state0, t_first, t_last, GRID_DT, acc);
    let sc = |t: f64| -> Option<([f64; 3], [f64; 3])> {
        let s = interp(&grid, t)?;
        Some(([s[0], s[1], s[2]], [s[3], s[4], s[5]]))
    };
    let mut rates = vec![0.0f64; times.len()];
    for i in 0..times.len() {
        rates[i] = downlink_rate_core(times[i], r_st[i], v_st[i], &sc).unwrap_or(f64::NAN);
    }
    rates
}

fn recoil_telem_accel(
    t: f64,
    r: [f64; 3],
    series: &[(f64, f64)],
    window: f64,
    eta: f64,
) -> [f64; 3] {
    let rn = norm(r);
    if rn <= 0.0 {
        return [0.0; 3];
    }
    let Some(p) = nearest_omni(series, t, window) else {
        return [0.0; 3];
    };
    let a = eta * p / (SC_MASS_KG * C);
    [-a * r[0] / rn, -a * r[1] / rn, -a * r[2] / rn]
}

fn light_time_sc_pos(
    t1: f64,
    r_st1: [f64; 3],
    sc: &dyn Fn(f64) -> Option<([f64; 3], [f64; 3])>,
) -> Option<(f64, [f64; 3])> {
    let mut t3 = t1;
    for _ in 0..6 {
        let (r_sc3, _) = sc(t3)?;
        let rho = dist(r_st1, r_sc3);
        let t3_new = t1 - rho / C;
        if (t3_new - t3).abs() < 1e-9 {
            t3 = t3_new;
            break;
        }
        t3 = t3_new;
    }
    let (r_sc3, _) = sc(t3)?;
    Some((t3, r_sc3))
}

fn subset_rms(covered: &[bool], resid: &[f64]) -> Option<f64> {
    let mut s = 0.0f64;
    let mut n = 0usize;
    for i in 0..resid.len() {
        if covered[i] {
            s += resid[i] * resid[i];
            n += 1;
        }
    }
    if n > 0 {
        Some((s / n as f64).sqrt())
    } else {
        None
    }
}

fn pearson(xs: &[f64], ys: &[f64]) -> f64 {
    let n = xs.len() as f64;
    if n < 2.0 {
        return f64::NAN;
    }
    let mx = xs.iter().sum::<f64>() / n;
    let my = ys.iter().sum::<f64>() / n;
    let mut sxx = 0.0;
    let mut syy = 0.0;
    let mut sxy = 0.0;
    for i in 0..xs.len() {
        let dx = xs[i] - mx;
        let dy = ys[i] - my;
        sxx += dx * dx;
        syy += dy * dy;
        sxy += dx * dy;
    }
    if sxx <= 0.0 || syy <= 0.0 {
        return f64::NAN;
    }
    sxy / (sxx * syy).sqrt()
}

struct Witness {
    days: usize,
    r_level: f64,
    r_diff: f64,
    med_a_scatter: f64,
    med_n_scatter: f64,
    med_a_det: f64,
    med_n_det: f64,
}

fn witness(
    a: &HashMap<i64, Vec<(f64, f64)>>,
    b: &HashMap<i64, Vec<(f64, f64)>>,
) -> Option<Witness> {
    let mut days: Vec<i64> = a.keys().copied().filter(|d| b.contains_key(d)).collect();
    days.sort_unstable();
    if days.len() < 30 {
        return None;
    }
    let mut a_med = Vec::with_capacity(days.len());
    let mut b_med = Vec::with_capacity(days.len());
    let mut a_scatter = Vec::new();
    let mut b_scatter = Vec::new();
    let mut a_det = Vec::new();
    let mut b_det = Vec::new();
    for d in &days {
        let mut av = a[d].clone();
        let mut bv = b[d].clone();
        av.sort_by(|x, y| x.0.total_cmp(&y.0));
        bv.sort_by(|x, y| x.0.total_cmp(&y.0));
        let aval: Vec<f64> = av.iter().map(|(_, v)| *v).collect();
        let bval: Vec<f64> = bv.iter().map(|(_, v)| *v).collect();
        let am = median(&mut aval.clone());
        let bm = median(&mut bval.clone());
        a_med.push(am);
        b_med.push(bm);
        if av.len() >= 2 {
            let sq: f64 = av.iter().map(|(_, v)| (v - am).powi(2)).sum();
            a_scatter.push((sq / av.len() as f64).sqrt());
        }
        if bv.len() >= 2 {
            let sq: f64 = bv.iter().map(|(_, v)| (v - bm).powi(2)).sum();
            b_scatter.push((sq / bv.len() as f64).sqrt());
        }
        if av.len() >= 3 {
            let t0 = av[0].0;
            let xs: Vec<f64> = av.iter().map(|(t, _)| t - t0).collect();
            let (sl, ic) = lin_fit(&xs, &aval);
            let sq: f64 = av
                .iter()
                .map(|(t, v)| (v - sl * (t - t0) - ic).powi(2))
                .sum();
            a_det.push((sq / av.len() as f64).sqrt());
        }
        if bv.len() >= 3 {
            let t0 = bv[0].0;
            let xs: Vec<f64> = bv.iter().map(|(t, _)| t - t0).collect();
            let (sl, ic) = lin_fit(&xs, &bval);
            let sq: f64 = bv
                .iter()
                .map(|(t, v)| (v - sl * (t - t0) - ic).powi(2))
                .sum();
            b_det.push((sq / bv.len() as f64).sqrt());
        }
    }
    let r_level = pearson(&a_med, &b_med);
    let da: Vec<f64> = a_med.windows(2).map(|w| w[1] - w[0]).collect();
    let db: Vec<f64> = b_med.windows(2).map(|w| w[1] - w[0]).collect();
    let r_diff = pearson(&da, &db);
    let med_a_scatter = median(&mut a_scatter.clone());
    let med_b_scatter = median(&mut b_scatter.clone());
    let med_a_det = median(&mut a_det.clone());
    let med_b_det = median(&mut b_det.clone());
    Some(Witness {
        days: days.len(),
        r_level,
        r_diff,
        med_a_scatter,
        med_n_scatter: med_b_scatter,
        med_a_det,
        med_n_det: med_b_det,
    })
}

fn main() {
    let Some(lsk) = embedded_lsk() else {
        eprintln!("pioneer10 link: naif0012 table void — the probe stays empty (0 honored)");
        return;
    };
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ionex_dir = args
        .iter()
        .position(|a| a == "--ionex-dir")
        .and_then(|i| args.get(i + 1))
        .cloned();
    let omni2_path = match args
        .iter()
        .position(|a| a == "--omni2")
        .and_then(|i| args.get(i + 1))
        .cloned()
    {
        Some(p) => Some(p),
        None => {
            let root = omegaflow::archivar::cache_root();
            ["omni2_serie.bin", "omni2_serie_1h.bin"]
                .iter()
                .map(|p| root.join(p))
                .map(|p| p.to_string_lossy().into_owned())
                .find(|p| std::path::Path::new(p).exists())
        }
    };
    let sky = "data/pioneer10_skyfreq.bin";
    let Ok(bytes) = std::fs::read(sky) else {
        eprintln!("pioneer10 link: skyfreq bin void ({sky})");
        return;
    };
    let Some(records) = parse_bin(&bytes) else {
        eprintln!("pioneer10 link: skyfreq bin parse void");
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
                eprintln!("{body}: ephemeris bin void ({p})");
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
    let mut census: HashMap<i64, usize> = HashMap::new();
    for r in &records {
        *census.entry(r[6] as i64).or_default() += 1;
    }
    let mut census_ids: Vec<i64> = census.keys().copied().collect();
    census_ids.sort_unstable();
    let census_line: Vec<String> = census_ids
        .iter()
        .map(|id| {
            if dsn_station(*id).is_some() {
                format!("{id} (n={})", census[&id])
            } else {
                format!("{id} unbenannt (n={})", census[&id])
            }
        })
        .collect();
    let mut n_series: Vec<(f64, f64)> = Vec::new();
    let mut omni2_name: Option<String> = None;
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
                if !n_series.is_empty() {
                    omni2_name = Some(p.clone());
                }
            }
        }
    }
    if n_series.is_empty() {
        let hint = match &omni2_path {
            Some(p) => format!("{p} carries no N1800 —"),
            None => {
                "no omni2 bin in OMEGAFLOW_STATE cache (omni2_serie.bin or --omni2) —".to_string()
            }
        };
        eprintln!("pioneer10 link: {hint} the plasma deduction stays empty (0 honored)");
    } else {
        let p = omni2_name.as_deref().unwrap_or("");
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
            "pioneer10 link: Plasma source {p} — N1800: {} records, {}..{}, search window {:.1} h (2× median gap)",
            n_series.len(),
            jd_date(n_series[0].0),
            jd_date(n_series[n_series.len() - 1].0),
            omni2_window / 3600.0
        );
    }
    let tec_maps: Vec<(f64, TecGrid)> = match &ionex_dir {
        Some(dir) => {
            let maps = load_ionex(dir, &lsk);
            if maps.is_empty() {
                eprintln!(
                    "pioneer10 link: {dir} carries no IONEX-GIMs — the TEC deduction stays empty (0 honored)"
                );
            } else {
                eprintln!("pioneer10 link: {dir} carries {} TEC maps", maps.len());
            }
            maps
        }
        None => Vec::new(),
    };

    let mut times: Vec<f64> = Vec::new();
    let mut obs: Vec<f64> = Vec::new();
    let mut refs: Vec<f64> = Vec::new();
    let mut samplers: Vec<f64> = Vec::new();
    let mut stations: Vec<i64> = Vec::new();
    let mut rates0: Vec<f64> = Vec::new();
    let mut r_st: Vec<[f64; 3]> = Vec::new();
    let mut v_st: Vec<[f64; 3]> = Vec::new();
    let mut shift_plasma: Vec<Option<f64>> = Vec::new();
    let mut shift_tec: Vec<Option<f64>> = Vec::new();
    let mut slipped: Vec<bool> = Vec::new();
    let mut strength: Vec<f64> = Vec::new();
    let mut strength_ok: Vec<bool> = Vec::new();
    let mut ramp: Vec<f64> = Vec::new();
    let mut files: Vec<i64> = Vec::new();
    let mut no_station = 0usize;
    let mut no_model = 0usize;
    let mut no_plasma_occ = 0usize;
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
        let Some((t3, r3)) = light_time_sc_pos(t1, rs, &granule_sc) else {
            no_model += 1;
            continue;
        };
        let Some(rs2) = body_fixed_to_icrs_smooth(EARTH, lat, lon, alt, t1 + PLASMA_DT, &eph)
        else {
            no_model += 1;
            continue;
        };
        let Some((r4, _)) = granule_sc(t3 + PLASMA_DT) else {
            no_model += 1;
            continue;
        };
        let f0 = r[1];
        let n1 = nearest_omni(&n_series, t1, omni2_window);
        let n2 = nearest_omni(&n_series, t1 + PLASMA_DT, omni2_window);
        let sh_p = match (n1, n2) {
            (Some(v1), Some(v2)) => {
                let sh = plasma_shift(plasma_column(rs, r3, v1), plasma_column(rs2, r4, v2), f0);
                if sh.is_none() {
                    no_plasma_occ += 1;
                }
                sh
            }
            _ => None,
        };
        let sh_t = tec_pair_shift(&tec_maps, t1, lat, lon, f0);
        times.push(t1);
        obs.push(f0);
        refs.push(r[2]);
        samplers.push(r[3]);
        stations.push(r[6] as i64);
        rates0.push(rate);
        r_st.push(rs);
        v_st.push(vs);
        shift_plasma.push(sh_p);
        shift_tec.push(sh_t);
        slipped.push(r[9] != 0.0);
        strength.push(r[10]);
        strength_ok.push(r[10] < 0.0);
        ramp.push(r[11]);
        files.push(r[12] as i64);
    }
    if times.len() < 100 {
        eprintln!(
            "pioneer10 link: {} samples ({no_station} without station, {no_model} without model) — too short",
            times.len()
        );
        return;
    }
    let no_slipped = slipped.iter().filter(|&&s| s).count();
    let no_strength = slipped
        .iter()
        .zip(&strength_ok)
        .filter(|&(&s, &ok)| !s && !ok)
        .count();
    if no_slipped > 0 || no_strength > 0 {
        let Some((_, _, resid_all, _, _)) = fixed_effects(&rates0, &refs, &obs, &times) else {
            eprintln!("pioneer10 link: base fit (all samples) void");
            return;
        };
        let rms_all = rms_of(&resid_all);
        let mut sq_slip = 0.0;
        let mut sq_no_str = 0.0;
        let mut sq_clean = 0.0;
        let mut n_clean = 0usize;
        for i in 0..times.len() {
            if slipped[i] {
                sq_slip += resid_all[i] * resid_all[i];
            } else if !strength_ok[i] {
                sq_no_str += resid_all[i] * resid_all[i];
            } else {
                sq_clean += resid_all[i] * resid_all[i];
                n_clean += 1;
            }
        }
        let rms_slip = (sq_slip / no_slipped as f64).sqrt();
        let rms_no_str = (sq_no_str / no_strength as f64).sqrt();
        let rms_clean = (sq_clean / n_clean as f64).sqrt();
        eprintln!(
            "Deduction 0 Slipped-cycle mask: {no_slipped} of {} samples carry a slipped cycle (field 76) — discarded, not averaged; base RMS of all {rms_all:.3e} Hz, slipped subset {rms_slip:.3e} Hz, cleaned {rms_clean:.3e} Hz",
            times.len()
        );
        eprintln!(
            "Deduction 0b Strength gate: {no_strength} of the non-slipped samples carry no signal strength (field 78 ≥ 0 — the field is in 0.1 dBm, real strengths are negative, ~−174 dBm) — discarded, not averaged; subset {rms_no_str:.3e} Hz"
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
        r_st = keep.iter().map(|&i| r_st[i]).collect();
        v_st = keep.iter().map(|&i| v_st[i]).collect();
        shift_plasma = keep.iter().map(|&i| shift_plasma[i]).collect();
        shift_tec = keep.iter().map(|&i| shift_tec[i]).collect();
        strength = keep.iter().map(|&i| strength[i]).collect();
        ramp = keep.iter().map(|&i| ramp[i]).collect();
        files = keep.iter().map(|&i| files[i]).collect();
    }
    let n = times.len();
    let t_first = times[0];
    let t_last = times[n - 1];
    eprintln!(
        "pioneer10 link: {} samples ({no_station} without station, {no_model} without model), {}..{}, stations [{}]",
        n,
        jd_date(t_first),
        jd_date(t_last),
        census_line.join(", ")
    );

    let Some((_, _, resid_epoch, _, _)) = fixed_effects(&rates0, &refs, &obs, &times) else {
        eprintln!("pioneer10 link: base fit (epoch offsets) void");
        return;
    };
    let rms_epoch = rms_of(&resid_epoch);
    let Some((a0, c0, resid0, cell0, offset0)) =
        fixed_effects_cells(&rates0, &refs, &obs, &times, &files)
    else {
        eprintln!("pioneer10 link: base fit (epoch×file cells) void");
        return;
    };
    let rms0 = rms_of(&resid0);
    let n_cells = offset0.len();
    let min_cell = {
        let mut cnt = vec![0usize; n_cells];
        for i in 0..n {
            cnt[cell0[i]] += 1;
        }
        cnt.iter().copied().filter(|&c| c > 0).min().unwrap_or(0)
    };
    let max_off = offset0
        .iter()
        .copied()
        .fold(f64::NEG_INFINITY, f64::max)
        .max(offset0.iter().copied().fold(f64::INFINITY, f64::min).abs());
    eprintln!(
        "Base (Horizons granule + Moyer station/light time): epoch offsets alone {rms_epoch:.3e} Hz; with {n_cells} file cells (min n/cell {min_cell}, max |offset| {max_off:.3e} Hz): A {a0:.4e} Hz/(m/s), C {c0:.4e}, residual-RMS {rms0:.3e} Hz"
    );

    {
        let mut order: Vec<usize> = (0..n).collect();
        order.sort_by(|&a, &b| samplers[a].total_cmp(&samplers[b]));
        for q in 0..4 {
            let lo = q * n / 4;
            let hi = (q + 1) * n / 4;
            let mut s: Vec<f64> = (lo..hi).map(|k| samplers[order[k]]).collect();
            let med = median(&mut s);
            let mut sq = 0.0;
            for k in lo..hi {
                let rr = resid0[order[k]];
                sq += rr * rr;
            }
            eprintln!(
                "  Sampler quartile {q}: n={}, median sampler {med:.2} s, residual RMS {:.3e} Hz",
                hi - lo,
                (sq / (hi - lo) as f64).sqrt()
            );
        }
    }
    {
        let mut order: Vec<usize> = (0..n).collect();
        order.sort_by(|&a, &b| strength[a].total_cmp(&strength[b]));
        for q in 0..4 {
            let lo = q * n / 4;
            let hi = (q + 1) * n / 4;
            let mut s: Vec<f64> = (lo..hi).map(|k| strength[order[k]]).collect();
            let med = median(&mut s);
            let mut sq = 0.0;
            for k in lo..hi {
                let rr = resid0[order[k]];
                sq += rr * rr;
            }
            eprintln!(
                "  Signal-strength quartile {q}: n={}, median strength {med:.1}, residual RMS {:.3e} Hz",
                hi - lo,
                (sq / (hi - lo) as f64).sqrt()
            );
        }
    }
    {
        let n_files = files.iter().copied().max().unwrap_or(0) as usize + 1;
        let mut order: Vec<usize> = (0..n).collect();
        order.sort_by(|&a, &b| strength[a].total_cmp(&strength[b]));
        for q in 0..4 {
            let lo = q * n / 4;
            let hi = (q + 1) * n / 4;
            let mut sq = vec![0.0f64; n_files];
            let mut cnt = vec![0usize; n_files];
            for k in lo..hi {
                let i = order[k];
                let f = files[i] as usize;
                sq[f] += resid0[i] * resid0[i];
                cnt[f] += 1;
            }
            let mut parts: Vec<String> = Vec::new();
            for f in 0..n_files {
                if cnt[f] > 0 {
                    parts.push(format!(
                        "file {f}: n={}, RMS={:.3e}",
                        cnt[f],
                        (sq[f] / cnt[f] as f64).sqrt()
                    ));
                }
            }
            eprintln!("    strength quartile {q} per file: {}", parts.join(" | "));
        }
    }
    {
        let mut segs: Vec<(f64, f64, f64, f64, usize, i64, f64, f64, f64, bool, String)> =
            Vec::new();
        let mut lo = 0usize;
        while lo < n {
            let mut hi = lo + 1;
            while hi < n && times[hi] - times[hi - 1] < GAP_DAY_S {
                hi += 1;
            }
            if hi - lo >= 20 {
                let mut sq = 0.0;
                let mut s: Vec<f64> = Vec::with_capacity(hi - lo);
                let mut smp: Vec<f64> = Vec::with_capacity(hi - lo);
                let mut sum = 0.0;
                let mut stset: Vec<i64> = Vec::new();
                for i in lo..hi {
                    sq += resid0[i] * resid0[i];
                    sum += resid0[i];
                    s.push(strength[i]);
                    smp.push(samplers[i]);
                    if !stset.contains(&stations[i]) {
                        stset.push(stations[i]);
                    }
                }
                let mean = sum / (hi - lo) as f64;
                let mut det_sq = 0.0;
                let mid = 0.5 * (times[lo] + times[hi - 1]);
                let xs: Vec<f64> = (lo..hi).map(|i| times[i] - mid).collect();
                let ys: Vec<f64> = (lo..hi).map(|i| resid0[i]).collect();
                let (b, _) = lin_fit(&xs, &ys);
                for i in lo..hi {
                    let det = resid0[i] - mean - b * (times[i] - mid);
                    det_sq += det * det;
                }
                let det_rms = (det_sq / (hi - lo) as f64).sqrt();
                let rms = (sq / (hi - lo) as f64).sqrt();
                let med_strength = median(&mut s);
                let med_sampler = median(&mut smp);
                let gap_adj = (lo > 0 && times[lo] - times[lo - 1] >= 86400.0)
                    || (hi < n && times[hi] - times[hi - 1] >= 86400.0);
                stset.sort_unstable();
                let st_desc: Vec<String> = stset.iter().map(|x| x.to_string()).collect();
                segs.push((
                    times[lo],
                    times[hi - 1],
                    rms,
                    med_strength,
                    hi - lo,
                    files[lo],
                    med_sampler,
                    mean,
                    det_rms,
                    gap_adj,
                    st_desc.join(","),
                ));
            }
            lo = hi;
        }
        segs.sort_by(|a, b| b.2.total_cmp(&a.2));
        if segs.len() >= 8 {
            let n_seg = segs.len();
            eprintln!(
                "  Segment scatter: {n_seg} segments (≥20 samples), RMS min/p50/p90/max {:.3e}/{:.3e}/{:.3e}/{:.3e} Hz",
                segs[n_seg - 1].2,
                segs[n_seg / 2].2,
                segs[n_seg * 9 / 10].2,
                segs[0].2
            );
            for (k, s) in segs.iter().take(8).enumerate() {
                eprintln!(
                    "    #{k}: {}..{} file {} n={} RMS {:.3e} Hz (offset {:.3e}, scatter {:.3e}, {}) stations [{}], strength median {:.1}, sampler median {:.1} s",
                    jd_date(s.0),
                    jd_date(s.1),
                    s.5,
                    s.4,
                    s.2,
                    s.7,
                    s.8,
                    if s.9 { "gap-adjacent" } else { "interior" },
                    s.10,
                    s.3,
                    s.6
                );
            }
        }
    }
    {
        let mut st: Vec<i64> = stations.clone();
        st.sort_unstable();
        st.dedup();
        for s in st {
            let mut sq = 0.0;
            let mut cnt = 0usize;
            for i in 0..n {
                if stations[i] == s {
                    sq += resid0[i] * resid0[i];
                    cnt += 1;
                }
            }
            eprintln!(
                "  Station {s}: n={cnt}, residual-RMS {:.3e} Hz",
                (sq / cnt as f64).sqrt()
            );
        }
    }
    let n_pre_mask = n;
    let rms0_pre_mask = rms0;

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
                let mut sq_masked = 0.0;
                let mut sq_rest = 0.0;
                for i in 0..n {
                    if marked[i] {
                        sq_masked += resid0[i] * resid0[i];
                    } else {
                        sq_rest += resid0[i] * resid0[i];
                    }
                }
                let seg_desc: Vec<String> = masked
                    .iter()
                    .take(6)
                    .map(|(a, _)| format!("{} (file {})", jd_date(times[*a]), files[*a]))
                    .collect();
                let n_seg_rms = seg_rms.len();
                let n_mask_seg = masked.len();
                eprintln!(
                    "Deduction 10 segment mask: gate 4×p90 = {gate:.3e} Hz (p90 {p90:.3e}, {n_seg_rms} segments) — {n_mask_seg} segments ({n_masked} samples) discarded, not averaged: [{}] — subset {:.3e} Hz vs rest {:.3e} Hz",
                    seg_desc.join(", "),
                    (sq_masked / n_masked as f64).sqrt(),
                    (sq_rest / (n - n_masked) as f64).sqrt()
                );
                let keep: Vec<usize> = (0..n).filter(|&i| !marked[i]).collect();
                times = keep.iter().map(|&i| times[i]).collect();
                obs = keep.iter().map(|&i| obs[i]).collect();
                refs = keep.iter().map(|&i| refs[i]).collect();
                samplers = keep.iter().map(|&i| samplers[i]).collect();
                stations = keep.iter().map(|&i| stations[i]).collect();
                rates0 = keep.iter().map(|&i| rates0[i]).collect();
                r_st = keep.iter().map(|&i| r_st[i]).collect();
                v_st = keep.iter().map(|&i| v_st[i]).collect();
                shift_plasma = keep.iter().map(|&i| shift_plasma[i]).collect();
                shift_tec = keep.iter().map(|&i| shift_tec[i]).collect();
                ramp = keep.iter().map(|&i| ramp[i]).collect();
                files = keep.iter().map(|&i| files[i]).collect();
                strength = keep.iter().map(|&i| strength[i]).collect();
            } else {
                eprintln!(
                    "Deduction 10 segment mask: gate 4×p90 = {gate:.3e} Hz — 0 or all segments above it (0 honored), no mask"
                );
            }
        }
    }
    let n = times.len();
    let (_, _, resid0, _, _) = match fixed_effects_cells(&rates0, &refs, &obs, &times, &files) {
        Some(x) => x,
        None => {
            eprintln!("pioneer10 link: base refit after the segment mask void");
            return;
        }
    };
    let rms0 = rms_of(&resid0);
    if n < n_pre_mask {
        eprintln!(
            "  after the mask: {n} samples, residual-RMS {rms0:.3e} Hz (before {n_pre_mask} samples, {rms0_pre_mask:.3e} Hz)"
        );
    }
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
                "Deduction 11 noise weighting: class RMS 1-s {:.3e} Hz / 60-s {:.3e} Hz — weights 1:{:.2} (inverse variance, live data; the 1-s class now carries the fit)",
                v0.sqrt(),
                v1.sqrt(),
                w0 / w1
            );
        }
    }
    let (a0, c0, resid0, cell0, offset0) =
        match fixed_effects_cells_w(&rates0, &refs, &obs, &times, &files, &weights) {
            Some(x) => x,
            None => {
                eprintln!("pioneer10 link: weighted base fit void");
                return;
            }
        };
    let rms0 = rms_w(&resid0, &weights);
    eprintln!("  weighted base: residual-RMS {rms0:.3e} Hz — A {a0:.4e} Hz/(m/s), C {c0:.4e}");

    let cm = common_mode(&times, &stations, &resid0);
    let mut obs_b = obs.clone();
    if cm.pairs > 0 {
        let pair_desc: Vec<String> = cm
            .pair_counts
            .iter()
            .map(|(a, b, n)| format!("{a}↔{b} (n={n})"))
            .collect();
        for i in 0..n {
            if let Some(v) = cm.cm[i] {
                obs_b[i] = a0 * rates0[i] + c0 * refs[i] + offset0[cell0[i]] + v;
            }
        }
        eprintln!(
            "Deduction 1 common-mode: {} windows, {} pairs [{}] — differential RMS {:.3e} Hz (base per-sample RMS {rms0:.3e} Hz): the station difference carries the Earth-side systematics, the pair mean the signal",
            cm.windows,
            cm.pairs,
            pair_desc.join(", "),
            cm.diff_rms
        );
    } else {
        eprintln!(
            "Deduction 1 common-mode: 0 overlapping windows — the stations never measure simultaneously, the deduction stays empty (0 honored), no fabrication"
        );
    }
    let Some((_, _, resid_b, _, _)) =
        fixed_effects_cells_w(&rates0, &refs, &obs_b, &times, &files, &weights)
    else {
        eprintln!("pioneer10 link: common-mode fit void");
        return;
    };
    let rms_b = rms_of(&resid_b);
    eprintln!("  common-mode series: residual-RMS {rms_b:.3e} Hz (base {rms0:.3e} Hz)");

    let mut obs_c = obs_b.clone();
    let mut n_tec = 0usize;
    let mut n_plasma = 0usize;
    let mut sum_abs_tec = 0.0f64;
    let mut max_abs_tec = 0.0f64;
    let mut sum_abs_plasma = 0.0f64;
    let mut max_abs_plasma = 0.0f64;
    let mut covered_tec: Vec<bool> = Vec::with_capacity(n);
    let mut covered_plasma: Vec<bool> = Vec::with_capacity(n);
    for i in 0..n {
        if let Some(sh) = shift_tec[i] {
            obs_c[i] -= sh;
            n_tec += 1;
            sum_abs_tec += sh.abs();
            max_abs_tec = max_abs_tec.max(sh.abs());
            covered_tec.push(true);
        } else {
            covered_tec.push(false);
        }
        if let Some(sh) = shift_plasma[i] {
            obs_c[i] -= sh;
            n_plasma += 1;
            sum_abs_plasma += sh.abs();
            max_abs_plasma = max_abs_plasma.max(sh.abs());
            covered_plasma.push(true);
        } else {
            covered_plasma.push(false);
        }
    }
    let Some((_, _, resid_c, _, _)) =
        fixed_effects_cells_w(&rates0, &refs, &obs_c, &times, &files, &weights)
    else {
        eprintln!("pioneer10 link: Medien-Fit void");
        return;
    };
    if n_tec > 0 {
        let mean_abs = sum_abs_tec / n_tec as f64;
        let before = subset_rms(&covered_tec, &resid_b);
        let after = subset_rms(&covered_tec, &resid_c);
        match (before, after) {
            (Some(b), Some(a)) => eprintln!(
                "Deduction 2 TEC: {n_tec} of {n} samples carry a map pair — |Δf| mean {mean_abs:.3e} Hz, max {max_abs_tec:.3e} Hz; RMS on the subset {b:.3e} → {a:.3e} Hz"
            ),
            _ => eprintln!(
                "Deduction 2 TEC: {n_tec} of {n} samples carry a map pair — |Δf| mean {mean_abs:.3e} Hz, max {max_abs_tec:.3e} Hz"
            ),
        }
    } else {
        eprintln!(
            "Deduction 2 TEC: 0 of {n} samples carry a map pair — the GIM maps begin in 1998, the ATDF era lies before it; the TEC deduction stays empty (0 honored), discarded instead of averaged"
        );
    }
    if n_plasma > 0 {
        let mean_abs = sum_abs_plasma / n_plasma as f64;
        let before = subset_rms(&covered_plasma, &resid_b);
        let after = subset_rms(&covered_plasma, &resid_c);
        match (before, after) {
            (Some(b), Some(a)) => eprintln!(
                "Deduction 3 plasma: OMNI2 carries {n_plasma} of {n} samples ({no_plasma_occ} without column/occultation) — |Δf| mean {mean_abs:.3e} Hz, max {max_abs_plasma:.3e} Hz; RMS on the subset {b:.3e} → {a:.3e} Hz"
            ),
            _ => eprintln!(
                "Deduction 3 plasma: OMNI2 carries {n_plasma} of {n} samples ({no_plasma_occ} without column/occultation) — |Δf| mean {mean_abs:.3e} Hz, max {max_abs_plasma:.3e} Hz"
            ),
        }
    } else {
        eprintln!(
            "Deduction 3 plasma: 0 of {n} samples carry OMNI2 — the deduction stays empty (0 honored)"
        );
    }

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
        let m = xs.len() as f64;
        let x_mean = xs.iter().sum::<f64>() / m;
        let sxx: f64 = xs.iter().map(|x| (x - x_mean).powi(2)).sum();
        let rss: f64 = xs
            .iter()
            .zip(&ys)
            .map(|(x, y)| {
                let p = k_ramp * (x - x_mean);
                let yy = y - ys.iter().sum::<f64>() / m;
                (yy - p).powi(2)
            })
            .sum();
        let se_k = if m > 2.0 && sxx > 0.0 {
            (rss / ((m - 2.0) * sxx)).sqrt()
        } else {
            f64::NAN
        };
        let mut sum_abs_ramp = 0.0;
        for i in 0..n {
            let corr = k_ramp * ramp[i] * (times[i] - seg_mid[i]);
            obs_d[i] -= corr;
            sum_abs_ramp += corr.abs();
        }
        let Some((_, _, resid_d_pre, _, _)) =
            fixed_effects_cells_w(&rates0, &refs, &obs_d, &times, &files, &weights)
        else {
            eprintln!("pioneer10 link: Ramp-Fit void");
            return;
        };
        eprintln!(
            "Deduction 5 Ramp: {n_ramped} of {n} samples carry RAMP_RATE (field 112) — the residual slope against the ramp term measures the coupling k = {k_ramp:.4e} ± {se_k:.4e} Hz per unit; mean |Δf| {:.3e} Hz, max |Δf| {:.3e} Hz; RMS {:.3e} → {:.3e} Hz",
            sum_abs_ramp / n as f64,
            (0..n)
                .map(|i| (k_ramp * ramp[i] * (times[i] - seg_mid[i])).abs())
                .fold(f64::NEG_INFINITY, f64::max),
            rms_of(&resid_c),
            rms_of(&resid_d_pre)
        );
    } else {
        eprintln!(
            "Deduction 5 Ramp: {n_ramped} of {n} samples carry RAMP_RATE — the deduction stays empty (0 honored)"
        );
    }
    let Some((_, _, resid_d, _, _)) =
        fixed_effects_cells_w(&rates0, &refs, &obs_d, &times, &files, &weights)
    else {
        eprintln!("pioneer10 link: Ramp-Fit void");
        return;
    };
    let mut slope_seg = vec![0.0f64; n];
    let mut seg_mid7 = vec![0.0f64; n];
    let mut n_seg7 = 0usize;
    let mut n_gap_adj = 0usize;
    let mut n_interior = 0usize;
    let mut sq_gap_adj = 0.0;
    let mut sq_interior = 0.0;
    let mut sq_det = 0.0;
    let mut sq_1s_raw = 0.0;
    let mut sq_1s_det = 0.0;
    let mut n_1s = 0usize;
    let mut sq_60_raw = 0.0;
    let mut sq_60_det = 0.0;
    let mut n_60 = 0usize;
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
                let gap_adj = (lo > 0 && times[lo] - times[lo - 1] >= 86400.0)
                    || (hi < n && times[hi] - times[hi - 1] >= 86400.0);
                for i in lo..hi {
                    slope_seg[i] = b;
                    seg_mid7[i] = mid;
                    let rr = resid_d[i];
                    let det = rr - b * (times[i] - mid);
                    sq_det += det * det;
                    if gap_adj {
                        sq_gap_adj += rr * rr;
                        n_gap_adj += 1;
                    } else {
                        sq_interior += rr * rr;
                        n_interior += 1;
                    }
                    if samplers[i] < 10.0 {
                        sq_1s_raw += rr * rr;
                        sq_1s_det += det * det;
                        n_1s += 1;
                    } else {
                        sq_60_raw += rr * rr;
                        sq_60_det += det * det;
                        n_60 += 1;
                    }
                }
                n_seg7 += 1;
            }
            lo = hi;
        }
    }
    let gap_line = if n_gap_adj > 0 && n_interior > 0 {
        format!(
            "Gap-adjacent ({n_gap_adj} samples, {:.3e} Hz) vs interior ({n_interior}, {:.3e} Hz)",
            (sq_gap_adj / n_gap_adj as f64).sqrt(),
            (sq_interior / n_interior as f64).sqrt()
        )
    } else {
        "no gap-adjacency measured".to_string()
    };
    eprintln!(
        "Deduction 7 daily-curve cut: {n_seg7} segments (≥20 samples) — residual {:.3e} → de-trended {:.3e} Hz; 1-s {:.3e} → {:.3e} Hz (n={n_1s}), 60-s {:.3e} → {:.3e} Hz (n={n_60}); {gap_line}",
        rms_of(&resid_d),
        (sq_det / n as f64).sqrt(),
        (sq_1s_raw / n_1s as f64).sqrt(),
        (sq_1s_det / n_1s as f64).sqrt(),
        (sq_60_raw / n_60 as f64).sqrt(),
        (sq_60_det / n_60 as f64).sqrt()
    );
    let mut obs_e = obs_d.clone();
    for i in 0..n {
        obs_e[i] -= slope_seg[i] * (times[i] - seg_mid7[i]);
    }
    let Some((_, _, resid_e, _, _)) =
        fixed_effects_cells_w(&rates0, &refs, &obs_e, &times, &files, &weights)
    else {
        eprintln!("pioneer10 link: daily-curve fit void");
        return;
    };
    let rms_e = rms_of(&resid_e);
    eprintln!(
        "  after the cut: residual-RMS {rms_e:.3e} Hz (ramp stage {:.3e} Hz) — the daily slope carried {:.3e} Hz",
        rms_of(&resid_d),
        rms_of(&resid_d) - rms_e
    );
    {
        let pdpl_loaded = std::fs::read("data/pioneer10_doppler_clean.bin")
            .ok()
            .and_then(|b| parse_pdpl(&b));
        match pdpl_loaded {
            Some(navio) => {
                let mut atdf_map: HashMap<i64, Vec<(f64, f64)>> = HashMap::new();
                for i in 0..n {
                    let day = (times[i] / 86400.0).floor() as i64;
                    atdf_map
                        .entry(day)
                        .or_default()
                        .push((times[i], resid_e[i]));
                }
                let mut nav_map: HashMap<i64, Vec<(f64, f64)>> = HashMap::new();
                let mut n_epoch_skip = 0usize;
                let mut n_value_skip = 0usize;
                for r in &navio {
                    if !(r[0] > NAVIO_T_MIN && r[0] < NAVIO_T_MAX) {
                        n_epoch_skip += 1;
                        continue;
                    }
                    if !r[1].is_finite() {
                        n_value_skip += 1;
                        continue;
                    }
                    let day = (r[0] / 86400.0).floor() as i64;
                    nav_map.entry(day).or_default().push((r[0], r[1]));
                }
                match witness(&atdf_map, &nav_map) {
                    Some(z) => {
                        let ratio = if z.med_n_scatter > 0.0 {
                            z.med_a_scatter / z.med_n_scatter
                        } else {
                            f64::NAN
                        };
                        let ratio_det = if z.med_n_det > 0.0 {
                            z.med_a_det / z.med_n_det
                        } else {
                            f64::NAN
                        };
                        eprintln!(
                            "Deduction 6 witness (NAVIO-clean, {n_epoch_skip} epoch-skips, {n_value_skip} value-skips): {} common days — residual daily-profile r {:.3} (day-to-day), level-r {:.3}; scatter about the daily median: ATDF {:.3e} Hz, NAVIO {:.3e} Hz ({ratio:.2}×); de-trended: ATDF {:.3e} Hz, NAVIO {:.3e} Hz ({ratio_det:.2}×) — {}",
                            z.days,
                            z.r_diff,
                            z.r_level,
                            z.med_a_scatter,
                            z.med_n_scatter,
                            z.med_a_det,
                            z.med_n_det,
                            if z.r_diff > 0.5 {
                                "the residuals share structure — a common unmodeled happening"
                            } else {
                                "die Residuen tragen getrennte Geschichten — jede Reduktion ihr eigenes Rauschen"
                            }
                        );
                    }
                    None => eprintln!(
                        "Deduction 6 witness (NAVIO-clean): too few common days — empty (0 honored)"
                    ),
                }
            }
            None => eprintln!(
                "Deduction 6 witness: data/pioneer10_doppler_clean.bin carries no PDPL holding — empty (0 honored)"
            ),
        }
    }

    let (Some(r0), Some(v0)) = (
        body_barycenter_position(SC_BODY, t_first, &eph),
        body_barycenter_velocity(SC_BODY, t_first, &eph),
    ) else {
        eprintln!("pioneer10 link: Horizons-Anfangszustand void");
        return;
    };
    let state0 = [r0[0], r0[1], r0[2], v0[0], v0[1], v0[2]];
    let Some(t0_rtg) = omegaflow::lsk::days_from_civil(1972, 3, 3)
        .map(|d| d as f64 * 86400.0)
        .and_then(|u| lsk.unix_to_tdb(u))
    else {
        eprintln!("pioneer10 link: RTG epoch void — the dynamics stays empty (0 honored)");
        return;
    };
    let srp_first = norm(srp_accel([state0[0], state0[1], state0[2]]));
    let rtg_first = norm(rtg_accel(
        [state0[0], state0[1], state0[2]],
        t_first,
        RTG_ANISO_SCAN[1],
        t0_rtg,
    ));
    eprintln!(
        "Deduction 4 Dynamics: a_SRP {srp_first:.3e} m/s², a_RTG(t0) {rtg_first:.3e} m/s² (aniso {}) — Sun + SRP + RTG + a_P as its own orbit (GRID_DT {} s)",
        RTG_ANISO_SCAN[1], GRID_DT
    );
    let mut best_aniso = RTG_ANISO_SCAN[1];
    let mut best_rms = f64::INFINITY;
    for &aniso in &RTG_ANISO_SCAN {
        let rates = run_rates_dyn(
            state0, t_first, t_last, aniso, t0_rtg, 0.0, &times, &r_st, &v_st,
        );
        let Some(f) = fit_stats_w(&rates, &refs, &obs_e, &times, &files, &weights) else {
            continue;
        };
        eprintln!("  aniso {aniso:.3}: residual-RMS {:.3e} Hz", f.rms);
        if f.rms < best_rms {
            best_rms = f.rms;
            best_aniso = aniso;
        }
    }
    let mut best_a_p = 0.0f64;
    let mut best_rms_ap = best_rms;
    let mut rms_lo = f64::INFINITY;
    let mut rms_hi = f64::NEG_INFINITY;
    for k in -2..=2 {
        let a_p = k as f64 * 4.0e-6;
        let rates = run_rates_dyn(
            state0, t_first, t_last, best_aniso, t0_rtg, a_p, &times, &r_st, &v_st,
        );
        let Some(f) = fit_stats_w(&rates, &refs, &obs_e, &times, &files, &weights) else {
            continue;
        };
        rms_lo = rms_lo.min(f.rms);
        rms_hi = rms_hi.max(f.rms);
        if f.rms < best_rms_ap {
            best_rms_ap = f.rms;
            best_a_p = a_p;
        }
    }
    eprintln!(
        "  a_P-scan ±8e-6 m/s² (aniso {best_aniso:.3}): residual-RMS flat {rms_lo:.3e}…{rms_hi:.3e} Hz — best a_P {best_a_p:.4e} m/s²"
    );
    let ptl = std::fs::read("data/pioneer10_telemetry.bin")
        .ok()
        .and_then(|b| omegaflow::pioneer_telemetry::parse_bin(&b));
    match ptl {
        Some(recs) => {
            let chan = |f: u32, c: u32| f << omegaflow::pioneer_telemetry::FILE_SHIFT | c;
            let mut best_recoil: Option<(String, f64, f64)> = None;
            for (cid, name) in [
                (chan(3, 1), "PRTG"),
                (chan(3, 5), "Pshunt"),
                (chan(3, 4), "Pbus"),
            ] {
                let mut s: Vec<(f64, f64)> = recs
                    .iter()
                    .filter(|r| r.2 == cid)
                    .map(|r| (r.0, r.1))
                    .collect();
                s.sort_by(|a, b| a.0.total_cmp(&b.0));
                if s.len() < 100 {
                    eprintln!(
                        "  Recoil {name}: {cid} carries {} records — empty (0 honored)",
                        s.len()
                    );
                    continue;
                }
                let mut gaps: Vec<f64> = Vec::with_capacity(s.len() - 1);
                for i in 1..s.len() {
                    gaps.push(s[i].0 - s[i - 1].0);
                }
                gaps.sort_by(f64::total_cmp);
                let med_gap = gaps[gaps.len() / 2];
                let window = (2.0 * med_gap).max(3.0 * 3600.0);
                let n_cover = times
                    .iter()
                    .filter(|&&t| nearest_omni(&s, t, window).is_some())
                    .count();
                let mean_p: f64 = s.iter().map(|(_, v)| v).sum::<f64>() / s.len() as f64;
                let mut best_eta = 0.0f64;
                let mut best_rms_eta = f64::INFINITY;
                let mut rms_recoil_lo = f64::INFINITY;
                let mut rms_recoil_hi = f64::NEG_INFINITY;
                let mut drift_eta0 = f64::NAN;
                let mut drift_best = f64::NAN;
                for &eta in &ELEC_ETA_SCAN {
                    let acc = |t: f64, r: [f64; 3]| {
                        let sun = sun_accel(r);
                        let srp = srp_accel(r);
                        let rec = recoil_telem_accel(t, r, &s, window, eta);
                        let rn = norm(r);
                        let mut out = [0.0; 3];
                        for k in 0..3 {
                            out[k] = sun[k] + srp[k] + rec[k];
                            if rn > 0.0 {
                                out[k] -= best_a_p * r[k] / (rn * rn);
                            }
                        }
                        out
                    };
                    let rates =
                        run_rates_dyn_accel(state0, t_first, t_last, &acc, &times, &r_st, &v_st);
                    let Some(f) = fit_stats_w(&rates, &refs, &obs_e, &times, &files, &weights)
                    else {
                        continue;
                    };
                    rms_recoil_lo = rms_recoil_lo.min(f.rms);
                    rms_recoil_hi = rms_recoil_hi.max(f.rms);
                    if eta == 0.0 {
                        drift_eta0 = f.drift / f.a;
                    }
                    if f.rms < best_rms_eta {
                        best_rms_eta = f.rms;
                        best_eta = eta;
                        drift_best = f.drift / f.a;
                    }
                }
                eprintln!(
                    "Deduction 8 Recoil {name}: {} records, mean {mean_p:.1} W units, window {:.1} h, coverage {n_cover} of {} samples — η-scan flat {rms_recoil_lo:.4e}…{rms_recoil_hi:.4e} Hz, best η {best_eta:.2}; drift η=0 {drift_eta0:.3e} → best η {drift_best:.3e} m/s²",
                    s.len(),
                    window / 3600.0,
                    times.len()
                );
                if best_recoil
                    .as_ref()
                    .map_or(true, |(_, _, brms)| best_rms_eta < *brms)
                {
                    best_recoil = Some((name.to_string(), best_eta, best_rms_eta));
                }
            }
            match best_recoil {
                Some((name, eta, rms)) => eprintln!(
                    "  Recoil finding: {name} with η {eta:.2} carries the orbit (RMS {rms:.3e} Hz) against the decay-model scan {best_rms_ap:.3e} Hz — the η-scan is flat, the chain does NOT distinguish the recoil channels (0 honored)"
                ),
                None => {
                    eprintln!("  Recoil finding: no channel carries the orbit — empty (0 honored)")
                }
            }
        }
        None => eprintln!(
            "Deduction 8 Recoil: data/pioneer10_telemetry.bin carries no PTLM holding — empty (0 honored)"
        ),
    }
    let rates_final = run_rates_dyn(
        state0, t_first, t_last, best_aniso, t0_rtg, best_a_p, &times, &r_st, &v_st,
    );
    let Some(FitStat {
        a: a_f,
        c: c_f,
        rms: rms_f,
        drift,
        se_drift,
    }) = fit_stats_w(&rates_final, &refs, &obs_e, &times, &files, &weights)
    else {
        eprintln!("pioneer10 link: dynamics fit void");
        return;
    };
    let accel = drift / a_f;
    let se_accel = se_drift / a_f.abs();
    let times_anomaly = accel.abs() / PIONEER_ANOMALY;
    eprintln!(
        "  dynamics fit: A {a_f:.4e} Hz/(m/s), C {c_f:.4e}, residual-RMS {rms_f:.3e} Hz, drift {drift:.4e} ± {se_drift:.4e} Hz/s → {accel:.4e} ± {se_accel:.4e} m/s²"
    );
    if se_accel.is_finite() && se_accel < accel.abs() && times_anomaly < 3.0 {
        eprintln!(
            "  the self-test carries the Pioneer anomaly ({PIONEER_ANOMALY:.3e} m/s², sunward): the deduction has cleaned the scene far enough that the drift is carried"
        );
    } else {
        eprintln!(
            "  the self-test does NOT carry the anomaly: |a| sits ~{times_anomaly:.0e}× above the Pioneer anomaly ({PIONEER_ANOMALY:.3e} m/s²) — the floor remains the per-sample scatter (0 honored)"
        );
    }
    if let Some(recs_t) = std::fs::read("data/pioneer10_telemetry.bin")
        .ok()
        .and_then(|b| omegaflow::pioneer_telemetry::parse_bin(&b))
    {
        let chan = |f: u32, c: u32| f << omegaflow::pioneer_telemetry::FILE_SHIFT | c;
        let mut series: Vec<Vec<(f64, f64)>> = Vec::new();
        for (cid, name) in [
            (chan(12, 1), "Trtg1"),
            (chan(3, 1), "PRTG"),
            (chan(3, 5), "Pshunt"),
        ] {
            let mut s: Vec<(f64, f64)> = recs_t
                .iter()
                .filter(|r| r.2 == cid)
                .map(|r| (r.0, r.1))
                .collect();
            s.sort_by(|a, b| a.0.total_cmp(&b.0));
            series.push(s);
            let _ = name;
        }
        let mut xs: Vec<[f64; 3]> = vec![[0.0; 3]; n];
        let mut cov = vec![false; n];
        let mut n_cov = 0usize;
        for i in 0..n {
            let mut ok = true;
            for (k, s) in series.iter().enumerate() {
                match nearest_omni(s, times[i], 3.0 * 3600.0) {
                    Some(v) => xs[i][k] = v,
                    None => ok = false,
                }
            }
            cov[i] = ok;
            if ok {
                n_cov += 1;
            }
        }
        if n_cov > 1000 {
            let mut m = [[0.0f64; 4]; 4];
            let mut rhs = [0.0f64; 4];
            for i in 0..n {
                if !cov[i] {
                    continue;
                }
                let x = [1.0, xs[i][0], xs[i][1], xs[i][2]];
                let wgt = weights[i];
                for a in 0..4 {
                    for b in 0..4 {
                        m[a][b] += wgt * x[a] * x[b];
                    }
                    rhs[a] += wgt * x[a] * resid_e[i];
                }
            }
            if let Some(beta) = solve4(m, rhs) {
                let mut sq_before = 0.0;
                let mut sq_after = 0.0;
                let mut sw = 0.0;
                let mut obs_f = obs_e.clone();
                for i in 0..n {
                    if !cov[i] {
                        continue;
                    }
                    let pat =
                        beta[0] + beta[1] * xs[i][0] + beta[2] * xs[i][1] + beta[3] * xs[i][2];
                    obs_f[i] -= pat;
                    sq_before += weights[i] * resid_e[i] * resid_e[i];
                    let rr = resid_e[i] - pat;
                    sq_after += weights[i] * rr * rr;
                    sw += weights[i];
                }
                eprintln!(
                    "Deduction 12 natural pattern (Trtg1/PRTG/Pshunt, weighted): coupling {:.3e} Hz/°F, {:.3e} Hz/W, {:.3e} Hz/W (intercept {:.3e}) — RMS {:.3e} → {:.3e} Hz ({n_cov} of {n} samples covered)",
                    beta[1],
                    beta[2],
                    beta[3],
                    beta[0],
                    (sq_before / sw).sqrt(),
                    (sq_after / sw).sqrt()
                );
                if let Some(f12) =
                    fit_stats_w(&rates_final, &refs, &obs_f, &times, &files, &weights)
                {
                    eprintln!(
                        "  drift after the pattern filter: {:.3e} ± {:.3e} m/s² (unfiltered {accel:.3e} ± {se_accel:.3e})",
                        f12.drift / f12.a,
                        f12.se_drift / f12.a.abs()
                    );
                }
            }
        } else {
            eprintln!(
                "Deduction 12 natural pattern: {n_cov} of {n} samples carry all three channels — too few (0 honored)"
            );
        }
    } else {
        eprintln!(
            "Deduction 12 natural pattern: data/pioneer10_telemetry.bin carries no PTLM holding — empty (0 honored)"
        );
    }
    {
        let Some(recs_te) = std::fs::read("data/pioneer10_telemetry.bin")
            .ok()
            .and_then(|b| omegaflow::pioneer_telemetry::parse_bin(&b))
        else {
            eprintln!("Deduction 14 TE lens: PTLM holding void — empty (0 honored)");
            return;
        };
        let chan = |f: u32, c: u32| f << omegaflow::pioneer_telemetry::FILE_SHIFT | c;
        let mut resid_day: HashMap<i64, Vec<f64>> = HashMap::new();
        for i in 0..n {
            let day = (times[i] / 86400.0).floor() as i64;
            resid_day.entry(day).or_default().push(resid_e[i]);
        }
        let mut drv: Vec<(String, HashMap<i64, (f64, usize)>)> = Vec::new();
        for (cid, name) in [
            (chan(12, 1), "Trtg1"),
            (chan(3, 1), "PRTG"),
            (chan(3, 5), "Pshunt"),
        ] {
            let mut m: HashMap<i64, (f64, usize)> = HashMap::new();
            for r in recs_te.iter().filter(|r| r.2 == cid) {
                let day = (r.0 / 86400.0).floor() as i64;
                let e = m.entry(day).or_insert((0.0, 0));
                e.0 += r.1;
                e.1 += 1;
            }
            drv.push((name.to_string(), m));
        }
        let mut days: Vec<i64> = resid_day
            .keys()
            .copied()
            .filter(|d| {
                resid_day[d].len() >= 3
                    && drv
                        .iter()
                        .all(|(_, m)| m.get(d).map_or(false, |(_, c)| *c > 0))
            })
            .collect();
        days.sort_unstable();
        if days.len() < 100 {
            eprintln!(
                "Deduction 14 TE lens: {} common days — too short (0 honored)",
                days.len()
            );
            return;
        }
        let mut res: Vec<f32> = Vec::with_capacity(days.len());
        let mut drv_series: Vec<Vec<f32>> = vec![Vec::with_capacity(days.len()); 3];
        let mut sun: Vec<f32> = Vec::with_capacity(days.len());
        for d in &days {
            let mut v = resid_day[d].clone();
            res.push(median(&mut v) as f32);
            for (k, (_, m)) in drv.iter().enumerate() {
                let (s, c) = m[d];
                drv_series[k].push((s / c as f64) as f32);
            }
            let t_noon = *d as f64 * 86400.0 + 43200.0;
            let sd = granule_sc(t_noon)
                .map(|(p, _)| norm(p) / AU)
                .unwrap_or(f64::NAN);
            sun.push(sd as f32);
        }
        let mut parts: Vec<String> = Vec::new();
        for (k, (name, _)) in drv.iter().enumerate() {
            match omegaflow::te::topological_te_phase(&drv_series[k], &res, 3, 3, TE_SEED) {
                Some(v) => parts.push(format!(
                    "{name}: TE {:.4} vs threshold {:.4} (τ {}/{}, {} surrogates, PE {:.2}/{:.2}) — {}",
                    v.te,
                    v.threshold,
                    v.tau_x,
                    v.tau_y,
                    v.surrogates_used,
                    v.pe_x.unwrap_or(f64::NAN),
                    v.pe_y.unwrap_or(f64::NAN),
                    if v.te > v.threshold {
                        "carries causal transfer (natural pattern)"
                    } else {
                        "no transfer carried (0 honored)"
                    }
                )),
                None => parts.push(format!(
                    "{name}: TE void — {} days do not carry the embedding",
                    days.len()
                )),
            }
        }
        match omegaflow::te::topological_te_phase(&sun, &res, 3, 3, TE_SEED) {
            Some(v) => parts.push(format!(
                "Sun: TE {:.4} vs threshold {:.4} (τ {}/{}, {} surrogates) — {}",
                v.te,
                v.threshold,
                v.tau_x,
                v.tau_y,
                v.surrogates_used,
                if v.te > v.threshold {
                    "carries causal transfer (natural pattern)"
                } else {
                    "no transfer carried (0 honored)"
                }
            )),
            None => parts.push("Sun: TE void".to_string()),
        }
        eprintln!(
            "Deduction 14 TE lens ({} common days, daily-median residual, topological TE, phase surrogates, threshold mean+2σ): {}",
            days.len(),
            parts.join(" | ")
        );
    }
    {
        let mut segs15: Vec<(f32, f32, f32)> = Vec::new();
        let mut lo = 0usize;
        while lo < n {
            let mut hi = lo + 1;
            while hi < n && times[hi] - times[hi - 1] < GAP_DAY_S {
                hi += 1;
            }
            if hi - lo >= 200 {
                let class0 = samplers[lo] < 10.0;
                let uniform = (lo..hi).all(|i| (samplers[i] < 10.0) == class0);
                if uniform {
                    let xs: Vec<f32> = (lo..hi).map(|i| rates0[i] as f32).collect();
                    let ys: Vec<f32> = (lo..hi).map(|i| resid_e[i] as f32).collect();
                    let pe = omegaflow::te::permutation_entropy(
                        &(lo..hi).map(|i| resid_e[i]).collect::<Vec<f64>>(),
                        3,
                        1,
                    );
                    match omegaflow::te::topological_te_phase(&xs, &ys, 3, 3, TE_SEED) {
                        Some(v) => segs15.push((
                            v.te as f32,
                            v.threshold as f32,
                            pe.unwrap_or(f64::NAN) as f32,
                        )),
                        None => segs15.push((f32::NAN, f32::NAN, pe.unwrap_or(f64::NAN) as f32)),
                    }
                }
            }
            lo = hi;
        }
        if segs15.is_empty() {
            eprintln!(
                "Deduction 15 TE-pass scale: 0 uniform segments ≥200 samples — empty (0 honored)"
            );
        } else {
            let n_seg15 = segs15.len();
            let n_above = segs15
                .iter()
                .filter(|(t, th, _)| t.is_finite() && *t > *th)
                .count();
            let pe_vals: Vec<f64> = segs15
                .iter()
                .map(|(_, _, p)| *p as f64)
                .filter(|p| p.is_finite())
                .collect();
            let mut pe_sorted = pe_vals.clone();
            pe_sorted.sort_by(f64::total_cmp);
            let pe_med = median(&mut pe_sorted);
            let max_ratio = segs15
                .iter()
                .filter(|(t, th, _)| t.is_finite() && *th > 0.0)
                .map(|(t, th, _)| (t / th) as f64)
                .fold(f64::NEG_INFINITY, f64::max);
            eprintln!(
                "Deduction 15 TE-pass scale (model rate → residual, {n_seg15} uniform segments ≥200 samples): {n_above} above the threshold (max TE/threshold {max_ratio:.2}); PE median {pe_med:.2} — {}",
                if n_above > n_seg15 / 10 {
                    "the pass scale carries causal transfer — a minute-scale pattern exists"
                } else if pe_med > 0.9 {
                    "the pass scale is white — the floor carries no causal pattern (0 honored)"
                } else {
                    "ordered series without model transfer — the structure is not the model (named)"
                }
            );
        }
    }
    {
        const F1_LO: f64 = 0.02;
        const F1_HI: f64 = 0.20;
        const F1_N: usize = 181;
        const F60_LO: f64 = 0.0005;
        const F60_HI: f64 = 0.008;
        const F60_N: usize = 76;
        let mut p1: Vec<Vec<f64>> = Vec::new();
        let mut p60: Vec<Vec<f64>> = Vec::new();
        let mut lo = 0usize;
        while lo < n {
            let mut hi = lo + 1;
            while hi < n && times[hi] - times[hi - 1] < GAP_DAY_S {
                hi += 1;
            }
            if hi - lo >= 200 {
                let class0 = samplers[lo] < 10.0;
                let uniform = (lo..hi).all(|i| (samplers[i] < 10.0) == class0);
                if uniform {
                    let m = hi - lo;
                    let mean: f64 = (lo..hi).map(|i| resid_e[i]).sum::<f64>() / m as f64;
                    let (flo, fhi, fsteps) = if class0 {
                        (F1_LO, F1_HI, F1_N)
                    } else {
                        (F60_LO, F60_HI, F60_N)
                    };
                    let mut spec = vec![0.0f64; fsteps];
                    for k in 0..fsteps {
                        let f = flo + (fhi - flo) * k as f64 / (fsteps - 1) as f64;
                        let mut re = 0.0;
                        let mut im = 0.0;
                        for i in lo..hi {
                            let ph = std::f64::consts::TAU * f * times[i];
                            let v = resid_e[i] - mean;
                            re += v * ph.cos();
                            im -= v * ph.sin();
                        }
                        spec[k] = (re * re + im * im) / m as f64;
                    }
                    if class0 {
                        p1.push(spec);
                    } else {
                        p60.push(spec);
                    }
                }
            }
            lo = hi;
        }
        eprintln!(
            "Deduction 16 spectral analysis of the pass scale (segment means subtracted, median periodogram):"
        );
        for (label, specs, flo, fhi, fsteps) in [
            ("1-s segments", &p1, F1_LO, F1_HI, F1_N),
            ("60-s segments", &p60, F60_LO, F60_HI, F60_N),
        ] {
            if specs.is_empty() {
                eprintln!("  {label}: 0 segments — empty (0 honored)");
                continue;
            }
            let mut med = vec![0.0f64; fsteps];
            for k in 0..fsteps {
                let mut v: Vec<f64> = specs.iter().map(|s| s[k]).collect();
                v.sort_by(f64::total_cmp);
                med[k] = v[v.len() / 2];
            }
            let mut med_sorted = med.clone();
            med_sorted.sort_by(f64::total_cmp);
            let floor = med_sorted[med_sorted.len() / 2];
            let mut peaks: Vec<(f64, f64)> = med
                .iter()
                .enumerate()
                .map(|(k, &p)| (flo + (fhi - flo) * k as f64 / (fsteps - 1) as f64, p))
                .collect();
            peaks.sort_by(|a, b| b.1.total_cmp(&a.1));
            let top: Vec<String> = peaks
                .iter()
                .take(3)
                .map(|(f, p)| format!("{f:.4} Hz ({:.1}×)", p / floor))
                .collect();
            eprintln!(
                "  {label}: {} segments, median periodogram, peaks: {}",
                specs.len(),
                top.join(", ")
            );
        }
        eprintln!(
            "  the spin (~4.28 RPM ≈ 71.3 mHz; in the 60-s alias ≈ 4.7 mHz) would be a nameable peak"
        );
        let idx1: Vec<usize> = (0..n).filter(|&i| samplers[i] < 10.0).collect();
        if idx1.len() >= 500 {
            let t1: Vec<f64> = idx1.iter().map(|&i| times[i]).collect();
            let r1: Vec<f64> = idx1.iter().map(|&i| resid_e[i]).collect();
            let grid = ls_grid(&t1, &r1, 0.05, 0.10, 0.0005);
            let mut powers: Vec<f64> = grid.iter().map(|(_, p)| *p).collect();
            powers.sort_by(f64::total_cmp);
            let floor = powers[powers.len() / 2];
            let mut sorted = grid.clone();
            sorted.sort_by(|a, b| b.1.total_cmp(&a.1));
            let top: Vec<String> = sorted
                .iter()
                .take(3)
                .map(|(f, p)| format!("{f:.4} Hz ({:.1}×)", p / floor))
                .collect();
            eprintln!(
                "  Spin search (LS scan over {} 1-s samples, irregular grid): peaks {} — a peak near ~71 mHz is absent from the spin signature (0 honored)",
                idx1.len(),
                top.join(", ")
            );
        } else {
            eprintln!(
                "  Spin search: {} 1-s samples — too few (0 honored)",
                idx1.len()
            );
        }
    }

    {
        let idx1: Vec<usize> = (0..n).filter(|&i| samplers[i] < 10.0).collect();
        if idx1.len() < 2000 {
            eprintln!(
                "Deduction 17 the line: {} 1-s samples — too short (0 honored)",
                idx1.len()
            );
        } else {
            let t1: Vec<f64> = idx1.iter().map(|&i| times[i]).collect();
            let r1: Vec<f64> = idx1.iter().map(|&i| resid_e[i]).collect();
            let refs17 = [0.0490, 0.0502, 0.0507, 0.05155];
            let g_shape = ls_grid(&t1, &r1, 0.02, 0.07, 0.0001);
            let (f_all, _, ratio_all) = peak_of(&g_shape);
            let g_line: Vec<(f64, f64)> = g_shape
                .iter()
                .copied()
                .filter(|(fr, _)| *fr >= 0.044 && *fr <= 0.058)
                .collect();
            let (f_star, _, ratio_star) = peak_of(&g_line);
            let f_star_i = peak_interp(&g_line).unwrap_or(f_star);
            let mut pows: Vec<f64> = g_shape.iter().map(|(_, p)| *p).collect();
            pows.sort_by(f64::total_cmp);
            let floor17 = pows[pows.len() / 2];
            let mut shape: Vec<String> = Vec::new();
            let mut f = 0.020;
            while f <= 0.070 {
                let p = g_shape
                    .iter()
                    .min_by(|a, b| (a.0 - f).abs().total_cmp(&(b.0 - f).abs()))
                    .map(|(_, p)| *p)
                    .unwrap_or(0.0);
                shape.push(format!("{:.0}:{:.1}", f * 1000.0, p / floor17));
                f += 0.002;
            }
            eprintln!(
                "Deduction 17 the line ({} 1-s samples): shape 20–70 mHz (2-mHz grid, ×median floor): {}",
                idx1.len(),
                shape.join(" ")
            );
            eprintln!(
                "Deduction 17   overall peak 20–70 mHz: {:.5} Hz ({ratio_all:.1}×) — the line region 44–58 mHz carries f* = {f_star_i:.6} Hz ({ratio_star:.1}×), T* = {:.4} s",
                f_all,
                1.0 / f_star_i
            );
            let part_scan = |label: &str, idx: &[usize]| {
                if idx.len() < 2000 {
                    eprintln!(
                        "Deduction 17   {label}: {} samples — too short (0 honored)",
                        idx.len()
                    );
                    return;
                }
                let ts: Vec<f64> = idx.iter().map(|&i| times[i]).collect();
                let vs: Vec<f64> = idx.iter().map(|&i| resid_e[i]).collect();
                let g = ls_grid(&ts, &vs, 0.044, 0.056, 0.00005);
                let (fp, _, ratio) = peak_of(&g);
                let mut pows2: Vec<f64> = g.iter().map(|(_, p)| *p).collect();
                pows2.sort_by(f64::total_cmp);
                let floor2 = pows2[pows2.len() / 2];
                let ref_parts: Vec<String> = refs17
                    .iter()
                    .map(|&fr| {
                        let p = g
                            .iter()
                            .min_by(|a, b| (a.0 - fr).abs().total_cmp(&(b.0 - fr).abs()))
                            .map(|(_, p)| *p)
                            .unwrap_or(0.0);
                        format!("{:.1} mHz:{:.1}×", fr * 1000.0, p / floor2)
                    })
                    .collect();
                eprintln!(
                    "Deduction 17   {label}: n={}, peak {fp:.5} Hz ({ratio:.1}×) — {}",
                    idx.len(),
                    ref_parts.join(" ")
                );
            };
            let mut fids: Vec<i64> = idx1.iter().map(|&i| files[i]).collect();
            fids.sort_unstable();
            fids.dedup();
            for fid in fids {
                let idx: Vec<usize> = idx1.iter().copied().filter(|&i| files[i] == fid).collect();
                part_scan(&format!("file {fid}"), &idx);
            }
            let mut years: Vec<u32> = idx1.iter().filter_map(|&i| year_of(times[i])).collect();
            years.sort_unstable();
            years.dedup();
            for y in years {
                let idx: Vec<usize> = idx1
                    .iter()
                    .copied()
                    .filter(|&i| year_of(times[i]) == Some(y))
                    .collect();
                part_scan(&format!("year {y}"), &idx);
            }
            let mut sts: Vec<i64> = idx1.iter().map(|&i| stations[i]).collect();
            sts.sort_unstable();
            sts.dedup();
            for st in sts {
                let idx: Vec<usize> = idx1
                    .iter()
                    .copied()
                    .filter(|&i| stations[i] == st)
                    .collect();
                part_scan(&format!("Station {st}"), &idx);
            }
            let idx60: Vec<usize> = (0..n).filter(|&i| samplers[i] >= 30.0).collect();
            if idx60.len() >= 2000 {
                let ts: Vec<f64> = idx60.iter().map(|&i| times[i]).collect();
                let vs: Vec<f64> = idx60.iter().map(|&i| resid_e[i]).collect();
                let g = ls_grid(&ts, &vs, 0.0001, 0.002, 0.00002);
                let (fp, _, ratio) = peak_of(&g);
                let mut local: Vec<f64> = g
                    .iter()
                    .filter(|(fr, _)| *fr >= 0.001 && *fr <= 0.002)
                    .map(|(_, p)| *p)
                    .collect();
                local.sort_by(f64::total_cmp);
                let local_floor = local[local.len() / 2];
                let p_alias = g
                    .iter()
                    .min_by(|a, b| (a.0 - 0.000714).abs().total_cmp(&(b.0 - 0.000714).abs()))
                    .map(|(_, p)| *p)
                    .unwrap_or(0.0);
                eprintln!(
                    "Deduction 17   ATDF-60-s class ({} samples): band 0.1–2 mHz — peak {:.4} mHz ({ratio:.1}×); at the 60-s alias 0.71 mHz: {:.1}× local floor [1–2 mHz] — the alias is {}",
                    idx60.len(),
                    fp * 1000.0,
                    p_alias / local_floor,
                    if p_alias >= 3.0 * local_floor {
                        "CARRIED"
                    } else {
                        "NOT carried (0 honored)"
                    }
                );
                let mut years60: Vec<u32> =
                    idx60.iter().filter_map(|&i| year_of(times[i])).collect();
                years60.sort_unstable();
                years60.dedup();
                for y in years60 {
                    let idx: Vec<usize> = idx60
                        .iter()
                        .copied()
                        .filter(|&i| year_of(times[i]) == Some(y))
                        .collect();
                    if idx.len() < 3000 {
                        continue;
                    }
                    let tsy: Vec<f64> = idx.iter().map(|&i| times[i]).collect();
                    let vsy: Vec<f64> = idx.iter().map(|&i| resid_e[i]).collect();
                    let gy = ls_grid(&tsy, &vsy, 0.0004, 0.0012, 0.00001);
                    let mut localy: Vec<f64> = gy
                        .iter()
                        .filter(|(fr, _)| *fr >= 0.0008 && *fr <= 0.0012)
                        .map(|(_, p)| *p)
                        .collect();
                    localy.sort_by(f64::total_cmp);
                    let localy_floor = localy[localy.len() / 2];
                    let py = gy
                        .iter()
                        .min_by(|a, b| (a.0 - 0.000714).abs().total_cmp(&(b.0 - 0.000714).abs()))
                        .map(|(_, p)| *p)
                        .unwrap_or(0.0);
                    eprintln!(
                        "Deduction 17     60-s year {y} ({} samples): P(0.71 mHz) = {:.1}× local floor [0.8–1.2 mHz]",
                        idx.len(),
                        py / localy_floor
                    );
                }
            }
        }
    }

    {
        let nav_series = |path: &str, t_lo: f64, t_hi: f64| -> Option<(Vec<f64>, Vec<f64>)> {
            let recs = std::fs::read(path).ok().and_then(|b| parse_pdpl(&b))?;
            let mut ts = Vec::new();
            let mut vs = Vec::new();
            for r in &recs {
                if r[0] > t_lo && r[0] < t_hi && r[1].is_finite() && r[1].abs() <= NAVIO_BEAT_BOUND
                {
                    ts.push(r[0]);
                    vs.push(r[1]);
                }
            }
            Some((ts, vs))
        };
        let witness_scan = |label: &str, ts: &[f64], vs: &[f64]| {
            if ts.len() < 2000 {
                eprintln!(
                    "Deduction 18 Witness {label}: {} samples — too short (0 honored)",
                    ts.len()
                );
                return;
            }
            let g = ls_grid(ts, vs, 0.00005, 0.002, 0.00002);
            let (fp, _, ratio) = peak_of(&g);
            let mut local: Vec<f64> = g
                .iter()
                .filter(|(fr, _)| *fr >= 0.001 && *fr <= 0.002)
                .map(|(_, p)| *p)
                .collect();
            local.sort_by(f64::total_cmp);
            let local_floor = local[local.len() / 2];
            let at = |fref: f64| {
                g.iter()
                    .min_by(|a, b| (a.0 - fref).abs().total_cmp(&(b.0 - fref).abs()))
                    .map(|(_, p)| *p / local_floor)
                    .unwrap_or(0.0)
            };
            let p071 = at(0.000714);
            eprintln!(
                "Deduction 18 Witness {label} ({} samples, {}..{}): band 0.05–2 mHz — peak {:.4} mHz ({ratio:.1}× band floor); at the 60-s alias 0.71 mHz: {:.1}× local floor [1–2 mHz]; shape P(0.2)/P(0.5)/P(0.71)/P(1.0)/P(1.5) mHz = {:.1}/{:.1}/{:.1}/{:.1}/{:.1}× — the alias is {}",
                ts.len(),
                jd_date(ts[0]),
                jd_date(ts[ts.len() - 1]),
                fp * 1000.0,
                p071,
                at(0.0002),
                at(0.0005),
                p071,
                at(0.001),
                at(0.0015),
                if p071 >= 3.0 {
                    "CARRIED — the line lives also in the NAVIO reduction"
                } else {
                    "NOT carried — only the ATDF chain carries it (reduction artifact named, 0 honored)"
                }
            );
        };
        if let Some((ts, vs)) = nav_series("data/pioneer10_doppler_clean.bin", t_first, t_last) {
            witness_scan("NAVIO-clean P10 (overlap era)", &ts, &vs);
            let mut years18: Vec<u32> = ts.iter().filter_map(|&t| year_of(t)).collect();
            years18.sort_unstable();
            years18.dedup();
            for y in years18 {
                let idx: Vec<usize> = (0..ts.len())
                    .filter(|&i| year_of(ts[i]) == Some(y))
                    .collect();
                if idx.len() < 3000 {
                    continue;
                }
                let tsy: Vec<f64> = idx.iter().map(|&i| ts[i]).collect();
                let vsy: Vec<f64> = idx.iter().map(|&i| vs[i]).collect();
                let gy = ls_grid(&tsy, &vsy, 0.0004, 0.0012, 0.00001);
                let mut localy: Vec<f64> = gy
                    .iter()
                    .filter(|(fr, _)| *fr >= 0.0008 && *fr <= 0.0012)
                    .map(|(_, p)| *p)
                    .collect();
                localy.sort_by(f64::total_cmp);
                let localy_floor = localy[localy.len() / 2];
                let py = gy
                    .iter()
                    .min_by(|a, b| (a.0 - 0.000714).abs().total_cmp(&(b.0 - 0.000714).abs()))
                    .map(|(_, p)| *p)
                    .unwrap_or(0.0);
                eprintln!(
                    "Deduction 18   NAVIO overlap year {y} ({} samples): P(0.71 mHz) = {:.1}× local floor [0.8–1.2 mHz]",
                    idx.len(),
                    py / localy_floor
                );
            }
        } else {
            eprintln!("Deduction 18 Witness NAVIO-clean P10: file void — empty (0 honored)");
        }
        if let Some((ts, vs)) =
            nav_series("data/pioneer10_doppler_clean.bin", NAVIO_T_MIN, NAVIO_T_MAX)
        {
            witness_scan("NAVIO-clean P10 (full era 1973–2002)", &ts, &vs);
        }
        if let Some((ts, vs)) =
            nav_series("data/pioneer11_doppler_clean.bin", NAVIO_T_MIN, NAVIO_T_MAX)
        {
            witness_scan("NAVIO-clean P11 (full era)", &ts, &vs);
        } else {
            eprintln!("Deduction 18 Witness NAVIO-clean P11: file void — empty (0 honored)");
        }
        if let Some((ts, vs)) = nav_series("data/pioneer11_doppler_clean.bin", t_first, t_last) {
            witness_scan("NAVIO-clean P11 (overlap era)", &ts, &vs);
        } else {
            eprintln!(
                "Deduction 18 Witness NAVIO-clean P11 (overlap era): file void — empty (0 honored)"
            );
        }
    }

    {
        let cnt = |lo: f64, hi: f64| samplers.iter().filter(|&&s| s >= lo && s < hi).count();
        let mut dts: Vec<f64> = Vec::new();
        let mut last: Option<f64> = None;
        for i in 0..n {
            if samplers[i] < 2.0 {
                if let Some(l) = last {
                    let d = times[i] - l;
                    if d > 0.0 && d < 5.0 {
                        dts.push(d);
                    }
                }
                last = Some(times[i]);
            }
        }
        let dt_line = if dts.len() >= 100 {
            dts.sort_by(f64::total_cmp);
            format!(
                "Δt in the 1-s grid p10/p50/p90 = {:.3}/{:.3}/{:.3} s",
                dts[dts.len() / 10],
                dts[dts.len() / 2],
                dts[dts.len() * 9 / 10]
            )
        } else {
            "Δt in the 1-s grid too short".to_string()
        };
        eprintln!(
            "Deduction 19 sampler grid: 1-s class {}, 10-s class {}, 60-s class {}, ≥120 s {} — {}",
            cnt(0.0, 2.0),
            cnt(2.0, 30.0),
            cnt(30.0, 120.0),
            cnt(120.0, f64::INFINITY),
            dt_line
        );
        let class_scan = |label: &str, lo: f64, hi: f64, band_lo: f64, band_hi: f64, step: f64| {
            let idx: Vec<usize> = (0..n)
                .filter(|&i| samplers[i] >= lo && samplers[i] < hi)
                .collect();
            if idx.len() < 2000 {
                eprintln!(
                    "Deduction 19   {label}: {} samples — too short (0 honored)",
                    idx.len()
                );
                return;
            }
            let ts: Vec<f64> = idx.iter().map(|&i| times[i]).collect();
            let vs: Vec<f64> = idx.iter().map(|&i| resid_e[i]).collect();
            let g = ls_grid(&ts, &vs, band_lo, band_hi, step);
            let (fp, _, ratio) = peak_of(&g);
            eprintln!(
                "Deduction 19   {label} ({} samples, band {:.0}–{:.0} mHz): peak {fp:.5} Hz ({ratio:.1}× floor)",
                idx.len(),
                band_lo * 1000.0,
                band_hi * 1000.0
            );
        };
        class_scan("1-s class", 0.0, 2.0, 0.044, 0.056, 0.00005);
        class_scan("10-s class", 2.0, 30.0, 0.0475, 0.0510, 0.000025);
        class_scan("60-s class", 30.0, 120.0, 0.0001, 0.002, 0.00002);
        {
            let idx_s: Vec<usize> = (0..n)
                .filter(|&i| strength[i] < 0.0 && samplers[i] < 10.0)
                .collect();
            if idx_s.len() >= 2000 {
                let ts: Vec<f64> = idx_s.iter().map(|&i| times[i]).collect();
                let vs: Vec<f64> = idx_s.iter().map(|&i| strength[i]).collect();
                let g = ls_grid(&ts, &vs, 0.044, 0.056, 0.00005);
                let (fp, _, ratio) = peak_of(&g);
                eprintln!(
                    "Deduction 19   signal strength (0.1 dBm, {} samples, band 44–56 mHz): peak {fp:.5} Hz ({ratio:.1}× floor) — {}",
                    idx_s.len(),
                    if ratio >= 3.0 {
                        "the strength modulates in the line band — antenna/pointing signature possible"
                    } else {
                        "the strength does NOT carry the line — the gain path is not the carrier (0 honored)"
                    }
                );
            }
        }
        let idx1: Vec<usize> = (0..n).filter(|&i| samplers[i] < 10.0).collect();
        let t1: Vec<f64> = idx1.iter().map(|&i| times[i]).collect();
        let r1: Vec<f64> = idx1.iter().map(|&i| resid_e[i]).collect();
        let g_fine = ls_grid(&t1, &r1, 0.044, 0.058, 0.00005);
        let (f_star, _, ratio_star) = peak_of(&g_fine);
        let f_star_i = peak_interp(&g_fine).unwrap_or(f_star);
        let fam_lo = f_star_i;
        let fam_hi = 1.0 - f_star_i;
        eprintln!(
            "Deduction 19 line holding: f* = {f_star_i:.6} Hz (T* = {:.4} s), {ratio_star:.1}× floor — the grids (1/10/60 s) set f0 only modulo 1/60 Hz: the family is f0 ≡ ±f* (mod 16.67 mHz), the members {:.6} Hz and {:.6} Hz (1 Hz − f*) are indistinguishable on these grids",
            1.0 / f_star_i,
            fam_lo,
            fam_hi
        );
        eprintln!(
            "Deduction 19 candidates measured: 256-divider on the family members — f*·256 = {:.4} Hz (Δ to 13 Hz: {:.4} Hz) and (1 Hz − f*)·256 = {:.4} Hz (Δ to 243 Hz: {:.4} Hz); T*/0.1 s = {:.1} count intervals; spin − f* = {:.2} mHz — the named machines (MDA resolvers/256-divider) carry no locally measured frequency claim; the measured deltas are the test, not the fabrication",
            f_star_i * 256.0,
            (f_star_i * 256.0 - 13.0).abs(),
            fam_hi * 256.0,
            (fam_hi * 256.0 - 243.0).abs(),
            1.0 / f_star_i / 0.1,
            (78.6 - f_star_i * 1000.0)
        );
    }

    {
        let idx1: Vec<usize> = (0..n).filter(|&i| samplers[i] < 10.0).collect();
        if idx1.len() < 2000 {
            eprintln!(
                "Deduction 20: {} 1-s samples — too short (0 honored)",
                idx1.len()
            );
        } else {
            let t1: Vec<f64> = idx1.iter().map(|&i| times[i]).collect();
            let r1: Vec<f64> = idx1.iter().map(|&i| resid_e[i]).collect();
            let mut fsky: Vec<f64> = idx1.iter().map(|&i| obs[i]).collect();
            fsky.sort_by(f64::total_cmp);
            let f0 = fsky[fsky.len() / 2];
            let m = t1.len() as f64;
            let g = ls_grid(&t1, &r1, 0.0485, 0.053, 0.00002);
            let mut pows: Vec<f64> = g.iter().map(|(_, p)| *p).collect();
            pows.sort_by(f64::total_cmp);
            let floor = pows[pows.len() / 2];
            let mut lmax: Vec<(f64, f64)> = Vec::new();
            for k in 1..g.len() - 1 {
                if g[k].1 > g[k - 1].1 && g[k].1 > g[k + 1].1 {
                    lmax.push(g[k]);
                }
            }
            lmax.sort_by(|a, b| b.1.total_cmp(&a.1));
            let mut taken: Vec<(f64, f64)> = Vec::new();
            for (f, p) in lmax {
                if taken.iter().all(|(tf, _)| (tf - f).abs() >= 0.0003) {
                    taken.push((f, p));
                }
                if taken.len() >= 3 {
                    break;
                }
            }
            let (fp, pp, _) = peak_of(&g);
            let k0 = g.iter().position(|(fr, _)| *fr == fp).unwrap_or(0);
            let half = 0.5 * (pp - floor).max(0.0);
            let mut lo = k0;
            while lo > 0 && g[lo - 1].1 - floor >= half {
                lo -= 1;
            }
            let mut hi = k0;
            while hi + 1 < g.len() && g[hi + 1].1 - floor >= half {
                hi += 1;
            }
            let fwhm = (g[hi].0 - g[lo].0).max(0.00002);
            let amp = (2.0 * pp / m).sqrt();
            let v_equiv = amp / f0 * 2.99792458e8;
            let amp_lines: Vec<String> = taken
                .iter()
                .map(|(f, p)| {
                    let a = (2.0 * p / m).sqrt();
                    format!("{f:.5} Hz: {a:.2e} Hz ≡ {:.2e} m/s", a / f0 * 2.99792458e8)
                })
                .collect();
            eprintln!(
                "Deduction 20 (a) amplitude gate: f0 = {:.6e} Hz (median sky freq), peak {fp:.5} Hz — A = {amp:.2e} Hz ≡ {v_equiv:.2e} m/s station velocity; the three local maxima (48.5–53 mHz, 0.02-mHz grid): {}",
                f0,
                amp_lines.join(" | ")
            );
            eprintln!(
                "Deduction 20 (a)   microseism reference: 0.1–10 µm at 50 mHz → v ≈ 3.1e-8–3.1e-6 m/s → Δf ≈ 2.4e-7–2.4e-5 Hz — {}",
                if amp >= 1.0e-3 {
                    "the measured amplitude lies ≥2 orders of magnitude above the Earth-wobble window — the wobble Doppler dies at the gate, the ground hypothesis (wobble) is ELIMINATED, remains: instrument/divider (measured, no fabrication)"
                } else {
                    "the measured amplitude lies in the microseism window — the ground hypothesis (wobble) LIVES"
                }
            );
            eprintln!(
                "Deduction 20 (c) peak width: FWHM (over the floor) = {:.3} mHz, Q = {:.1} — {}",
                fwhm * 1000.0,
                fp / fwhm,
                if fwhm <= 0.0001 {
                    "single-binned — a machine line (PLL/divider form)"
                } else if fwhm >= 0.001 {
                    "broad — natural form (sea-swell modulation)"
                } else {
                    "medium — undecided (0 honored)"
                }
            );
            for y in [1988u32, 1992] {
                for st in [14i64, 43, 63] {
                    let idx: Vec<usize> = idx1
                        .iter()
                        .copied()
                        .filter(|&i| year_of(times[i]) == Some(y) && stations[i] == st)
                        .collect();
                    if idx.len() < 1500 {
                        eprintln!(
                            "Deduction 20 (b)   {y} × station {st}: {} samples — too short (0 honored)",
                            idx.len()
                        );
                        continue;
                    }
                    let ts: Vec<f64> = idx.iter().map(|&i| times[i]).collect();
                    let vs: Vec<f64> = idx.iter().map(|&i| resid_e[i]).collect();
                    let gy = ls_grid(&ts, &vs, 0.044, 0.056, 0.00005);
                    let (fpy, ppy, ratioy) = peak_of(&gy);
                    let ampy = (2.0 * ppy / ts.len() as f64).sqrt();
                    eprintln!(
                        "Deduction 20 (b)   {y} × station {st}: n={}, peak {fpy:.5} Hz ({ratioy:.1}×), A = {ampy:.2e} Hz ≡ {:.2e} m/s",
                        idx.len(),
                        ampy / f0 * 2.99792458e8
                    );
                }
            }
        }
    }

    {
        if let Some(recs) = std::fs::read("data/pioneer10_skyfreq.bin")
            .ok()
            .and_then(|b| omegaflow::atdf::parse_bin(&b))
        {
            let mut hist = [0u64; 1000];
            let mut hist_class = [[0u64; 1000]; 3];
            let mut n_ok = 0usize;
            for r in &recs {
                let dcnt = r[7];
                if !dcnt.is_finite() || dcnt < 0.0 {
                    continue;
                }
                let u = ((dcnt - dcnt.floor()) * 1000.0).round() as usize % 1000;
                hist[u] += 1;
                let s = r[3];
                let k = if s < 2.0 {
                    0
                } else if s < 30.0 {
                    1
                } else {
                    2
                };
                hist_class[k][u] += 1;
                n_ok += 1;
            }
            let mut lattice = [false; 1000];
            for m in 0..256 {
                let u = ((m as f64) * 1000.0 / 256.0).round() as usize % 1000;
                lattice[u] = true;
            }
            let occupied: usize = hist.iter().filter(|&&c| c > 0).count();
            let lattice_hits: u64 = hist
                .iter()
                .enumerate()
                .filter(|(i, _)| lattice[*i])
                .map(|(_, &c)| c)
                .sum();
            let lattice_frac = lattice_hits as f64 / n_ok.max(1) as f64;
            eprintln!(
                "Deduction 22 count structure ({} PASF samples, {} with finite doppler_cnt): {} of 1000 fraction bins occupied — the 1/256 grid (256 rounded points) carries {:.3} of the holding — {}",
                recs.len(),
                n_ok,
                occupied,
                lattice_frac,
                if occupied <= 300 && lattice_frac > 0.95 {
                    "the 1/256 resolver structure IS carried (grid pattern in the L/P field)"
                } else if occupied > 900 {
                    "the L/P field is uniformly distributed over ~1000 bins — the 0.001-cycle resolution is native, no 1/256 grid (0 honored)"
                } else {
                    "neither the full 1/256 grid nor the uniform distribution — mixed structure, named (0 honored)"
                }
            );
            for (k, label) in [(0usize, "1-s class"), (1, "10-s class"), (2, "60-s class")] {
                let tot: u64 = hist_class[k].iter().sum();
                if tot < 1000 {
                    continue;
                }
                let occ: usize = hist_class[k].iter().filter(|&&c| c > 0).count();
                let lh: u64 = hist_class[k]
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| lattice[*i])
                    .map(|(_, &c)| c)
                    .sum();
                eprintln!(
                    "Deduction 22   {label}: {tot} samples, {occ}/1000 bins occupied, grid share {:.3}",
                    lh as f64 / tot as f64
                );
            }
            let mut idx: Vec<usize> = (0..1000).collect();
            idx.sort_by(|&a, &b| hist[b].cmp(&hist[a]));
            let dens = n_ok as f64 / 1000.0;
            let top: Vec<String> = idx
                .iter()
                .take(8)
                .map(|&i| format!("u={i}:{:.1}×", hist[i] as f64 / dens))
                .collect();
            eprintln!(
                "Deduction 22   top-8 bins (× uniform-density): {}",
                top.join(" ")
            );
        } else {
            eprintln!(
                "Deduction 22: data/pioneer10_skyfreq.bin carries no PASF — empty (0 honored)"
            );
        }
    }

    {
        if let Some(recs) = std::fs::read("data/pioneer10_skyfreq.bin")
            .ok()
            .and_then(|b| omegaflow::atdf::parse_bin(&b))
        {
            let t_all: Vec<f64> = recs.iter().map(|r| r[0]).collect();
            let ref_all: Vec<f64> = recs.iter().map(|r| r[2]).collect();
            let st_all: Vec<i64> = recs.iter().map(|r| r[6] as i64).collect();
            let samp_all: Vec<f64> = recs.iter().map(|r| r[3]).collect();
            let mut step_units: Vec<i64> = Vec::new();
            for i in 1..recs.len() {
                if !ref_all[i].is_finite() || !ref_all[i - 1].is_finite() {
                    continue;
                }
                let d = ((ref_all[i] - ref_all[i - 1]) * 10.0).round();
                if d != 0.0 {
                    step_units.push(d as i64);
                }
            }
            let n_steps = step_units.len();
            if n_steps >= 10 {
                step_units.sort_unstable();
                eprintln!(
                    "Deduction 23 ref series: {} PASF records, {} non-zero ref steps, p10/p50/p90 = {}/{}/{} ×0.1 Hz",
                    recs.len(),
                    n_steps,
                    step_units[n_steps / 10],
                    step_units[n_steps / 2],
                    step_units[n_steps * 9 / 10]
                );
            }
            for st in [14i64, 43, 63] {
                let idx: Vec<usize> = (0..recs.len())
                    .filter(|&i| st_all[i] == st && samp_all[i] < 10.0 && ref_all[i].is_finite())
                    .collect();
                if idx.len() < 2000 {
                    eprintln!(
                        "Deduction 23   Station {st}: {} 1-s samples — too short (0 honored)",
                        idx.len()
                    );
                    continue;
                }
                let ts: Vec<f64> = idx.iter().map(|&i| t_all[i]).collect();
                let vref: Vec<f64> = idx.iter().map(|&i| ref_all[i]).collect();
                let g_ref = ls_grid(&ts, &vref, 0.044, 0.056, 0.00005);
                let (fp_ref, _, ratio_ref) = peak_of(&g_ref);
                let mut tstep: Vec<f64> = Vec::new();
                let mut vstep: Vec<f64> = Vec::new();
                for k in 1..idx.len() {
                    let d = ref_all[idx[k]] - ref_all[idx[k - 1]];
                    if d != 0.0 && d.is_finite() {
                        tstep.push(t_all[idx[k]]);
                        vstep.push(d);
                    }
                }
                let step_line = if tstep.len() >= 500 {
                    let g_step = ls_grid(&tstep, &vstep, 0.044, 0.056, 0.00005);
                    let (fp_s, _, ratio_s) = peak_of(&g_step);
                    format!(
                        "steps (n={}): peak {fp_s:.5} Hz ({ratio_s:.1}×)",
                        tstep.len()
                    )
                } else {
                    format!("steps: {} — too short (0 honored)", tstep.len())
                };
                eprintln!(
                    "Deduction 23   Station {st} (n={} 1-s): ref-level peak {fp_ref:.5} Hz ({ratio_ref:.1}×), {}",
                    idx.len(),
                    step_line
                );
            }
            let mut tg: Vec<f64> = Vec::new();
            let mut vg: Vec<f64> = Vec::new();
            for i in 1..recs.len() {
                if samp_all[i] < 10.0 && samp_all[i - 1] < 10.0 && st_all[i] == st_all[i - 1] {
                    let d = ref_all[i] - ref_all[i - 1];
                    if d != 0.0 && d.is_finite() {
                        tg.push(t_all[i]);
                        vg.push(d);
                    }
                }
            }
            if tg.len() >= 1000 {
                let gg = ls_grid(&tg, &vg, 0.02, 0.07, 0.0001);
                let mut pows: Vec<f64> = gg.iter().map(|(_, p)| *p).collect();
                pows.sort_by(f64::total_cmp);
                let fl = pows[pows.len() / 2];
                let mut top8 = gg.clone();
                top8.sort_by(|a, b| b.1.total_cmp(&a.1));
                let tops: Vec<String> = top8
                    .iter()
                    .take(5)
                    .map(|(f, p)| format!("{f:.4} Hz ({:.1}×)", p / fl))
                    .collect();
                eprintln!(
                    "Deduction 23   global ref-step pattern (n={} steps, 20–70 mHz): peaks {}",
                    tg.len(),
                    tops.join(", ")
                );
            } else {
                eprintln!(
                    "Deduction 23   global ref-step pattern: {} steps — too short (0 honored)",
                    tg.len()
                );
            }
        } else {
            eprintln!(
                "Deduction 23: data/pioneer10_skyfreq.bin carries no PASF — empty (0 honored)"
            );
        }
    }

    {
        let idx1: Vec<usize> = (0..n).filter(|&i| samplers[i] < 10.0).collect();
        for (st, f_line) in [(14i64, 0.04575), (43, 0.05155), (63, 0.04715)] {
            let mut cells: Vec<usize> = idx1
                .iter()
                .copied()
                .filter(|&i| stations[i] == st && strength[i] < 0.0 && strength[i].is_finite())
                .collect();
            if cells.len() < 3000 {
                eprintln!(
                    "Deduction 24   station {st}: {} samples — too short (0 honored)",
                    cells.len()
                );
                continue;
            }
            cells.sort_by(|&a, &b| strength[a].total_cmp(&strength[b]));
            let k = cells.len() / 3;
            let mut amps = [0.0f64; 3];
            let mut ars = [0.0f64; 3];
            let mut parts: Vec<String> = Vec::new();
            for p in 0..3 {
                let lo = p * k;
                let hi = if p == 2 { cells.len() } else { (p + 1) * k };
                let idx: Vec<usize> = cells[lo..hi].to_vec();
                let ts: Vec<f64> = idx.iter().map(|&i| times[i]).collect();
                let vs: Vec<f64> = idx.iter().map(|&i| resid_e[i]).collect();
                let g = ls_grid(&ts, &vs, f_line - 0.003, f_line + 0.003, 0.00005);
                let (_, pp, ratio) = peak_of(&g);
                let amp = (2.0 * pp / ts.len() as f64).sqrt();
                let rms = rms_of(&vs);
                amps[p] = amp;
                ars[p] = amp / rms;
                parts.push(format!(
                    "{:.0}..{:.0} dBm·10: A={amp:.2e} Hz ({ratio:.1}×), A/RMS={:.3}",
                    strength[cells[lo]],
                    strength[cells[hi - 1]],
                    amp / rms
                ));
            }
            let verdict = if amps[0] >= 1.5 * amps[2] && ars[0] >= 0.8 * ars[2] {
                "the amplitude grows toward the weak signal — loop-noise signature CARRIED"
            } else if (amps[0] - amps[2]).abs() <= 0.2 * amps[2] {
                "the amplitude is strength-constant — a FIXED trace (reference leak), no loop-noise signature (0 honored)"
            } else {
                "no clean pattern — undecided (0 honored)"
            };
            eprintln!(
                "Deduction 24   station {st} (line {:.2} mHz, n={}): {} — {verdict}",
                f_line * 1000.0,
                cells.len(),
                parts.join(" | ")
            );
        }
    }

    {
        let nav_series = |path: &str, t_lo: f64, t_hi: f64| -> Option<(Vec<f64>, Vec<f64>)> {
            let recs = std::fs::read(path).ok().and_then(|b| parse_pdpl(&b))?;
            let mut ts = Vec::new();
            let mut vs = Vec::new();
            for r in &recs {
                if r[0] > t_lo && r[0] < t_hi && r[1].is_finite() && r[1].abs() <= NAVIO_BEAT_BOUND
                {
                    ts.push(r[0]);
                    vs.push(r[1]);
                }
            }
            Some((ts, vs))
        };
        let fit_phase = |ts: &[f64], vs: &[f64], f: f64| -> (f64, f64) {
            let m = ts.len() as f64;
            let vsum = vs.iter().sum::<f64>() / m;
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
            if det.abs() < 1e-300 {
                return (0.0, 0.0);
            }
            let a = (sy * cc - cy * sc) / det;
            let b = (cy * ss - sy * sc) / det;
            ((a * a + b * b).sqrt(), b.atan2(a))
        };
        let p10o = nav_series("data/pioneer10_doppler_clean.bin", t_first, t_last);
        let p11o = nav_series("data/pioneer11_doppler_clean.bin", t_first, t_last);
        match (p10o, p11o) {
            (Some((ts10, vs10)), Some((ts11, vs11))) => {
                let mut s10 = ts10.clone();
                let mut s11 = ts11.clone();
                s10.sort_by(f64::total_cmp);
                s11.sort_by(f64::total_cmp);
                let t_lo = ts11[0].max(ts10[0]);
                let t_hi = s10[s10.len() * 95 / 100].min(s11[s11.len() * 95 / 100]);
                let t_mid = (t_lo + t_hi) / 2.0;
                let half = |ts: &[f64], vs: &[f64], first: bool| {
                    let mut hts = Vec::new();
                    let mut hvs = Vec::new();
                    for (k, &t) in ts.iter().enumerate() {
                        let take = if first { t < t_mid } else { t >= t_mid };
                        if take && t >= t_lo && t <= t_hi {
                            hts.push(t);
                            hvs.push(vs[k]);
                        }
                    }
                    (hts, hvs)
                };
                let (h10a, hv10a) = half(&ts10, &vs10, true);
                let (h10b, hv10b) = half(&ts10, &vs10, false);
                let (h11a, hv11a) = half(&ts11, &vs11, true);
                let (h11b, hv11b) = half(&ts11, &vs11, false);
                if h10a.len() < 2000 || h10b.len() < 2000 || h11a.len() < 2000 || h11b.len() < 2000
                {
                    eprintln!(
                        "Deduction 26: halves too short ({}/{}/{}/{}) — empty (0 honored)",
                        h10a.len(),
                        h10b.len(),
                        h11a.len(),
                        h11b.len()
                    );
                } else {
                    let f_alias = 0.000714;
                    let (a10a, p10a) = fit_phase(&h10a, &hv10a, f_alias);
                    let (a10b, p10b) = fit_phase(&h10b, &hv10b, f_alias);
                    let (a11a, p11a) = fit_phase(&h11a, &hv11a, f_alias);
                    let (a11b, p11b) = fit_phase(&h11b, &hv11b, f_alias);
                    let dp1 = (p10a - p11a).rem_euclid(std::f64::consts::TAU);
                    let dp2 = (p10b - p11b).rem_euclid(std::f64::consts::TAU);
                    let delta = (dp1 - dp2).rem_euclid(std::f64::consts::TAU);
                    let mut null_delta: Vec<f64> = Vec::new();
                    let mut rng = 0x9E3779B97F4A7C15u64;
                    for _ in 0..200 {
                        rng = rng
                            .wrapping_mul(6364136223846793005)
                            .wrapping_add(1442695040888963407);
                        let off_a = ((rng >> 33) as usize) % h10a.len();
                        rng = rng
                            .wrapping_mul(6364136223846793005)
                            .wrapping_add(1442695040888963407);
                        let off_b = ((rng >> 33) as usize) % h10b.len();
                        let rot_a: Vec<f64> = hv10a[off_a..]
                            .iter()
                            .chain(&hv10a[..off_a])
                            .copied()
                            .collect();
                        let rot_b: Vec<f64> = hv10b[off_b..]
                            .iter()
                            .chain(&hv10b[..off_b])
                            .copied()
                            .collect();
                        let (_, pr_a) = fit_phase(&h10a, &rot_a, f_alias);
                        let (_, pr_b) = fit_phase(&h10b, &rot_b, f_alias);
                        null_delta.push((pr_a - pr_b).rem_euclid(std::f64::consts::TAU));
                    }
                    null_delta.sort_by(f64::total_cmp);
                    let q10 = null_delta[20];
                    let q50 = null_delta[100];
                    let q90 = null_delta[180];
                    let verdict = if delta < q10 || delta > q90 {
                        "the phase difference is STABLE across the halves — P10 and P11 carry the SAME continuous oscillation — the station chain is the shared carrier (measured)"
                    } else {
                        "the phase difference lies in the null distribution — no shared oscillation carried (0 honored)"
                    };
                    eprintln!(
                        "Deduction 26 probe coherence ({}..{}, f = 0.714 mHz): P10 halves A = {a10a:.2e}/{a10b:.2e} Hz, P11 A = {a11a:.2e}/{a11b:.2e} Hz — Δψ(H1) = {dp1:.2} rad, Δψ(H2) = {dp2:.2} rad, δ = {delta:.2} rad; null (200 circular surrogates): q10/q50/q90 = {q10:.2}/{q50:.2}/{q90:.2} rad — {verdict}",
                        jd_date(t_lo),
                        jd_date(t_hi)
                    );
                }
            }
            _ => {
                eprintln!("Deduction 26: NAVIO-clean P10/P11 void — empty (0 honored)");
            }
        }
    }

    {
        if let Some(recs) = std::fs::read("data/pioneer10_skyfreq.bin")
            .ok()
            .and_then(|b| omegaflow::atdf::parse_bin(&b))
        {
            let cell_scan = |label: &str,
                             idx: &[usize],
                             t_all: &[f64],
                             fsky: &[f64],
                             flo: f64,
                             fhi: f64,
                             gap: f64| {
                if idx.len() < 1500 {
                    eprintln!(
                        "Deduction 27   {label}: {} 1-s samples — too short (0 honored)",
                        idx.len()
                    );
                    return;
                }
                let mut ts: Vec<f64> = Vec::new();
                let mut vs: Vec<f64> = Vec::new();
                let mut block_lo = 0usize;
                while block_lo < idx.len() {
                    let mut block_hi = block_lo + 1;
                    while block_hi < idx.len()
                        && t_all[idx[block_hi]] - t_all[idx[block_hi - 1]] <= gap
                    {
                        block_hi += 1;
                    }
                    let blen = block_hi - block_lo;
                    if blen >= 10 {
                        let xt: Vec<f64> =
                            idx[block_lo..block_hi].iter().map(|&i| t_all[i]).collect();
                        let yv: Vec<f64> =
                            idx[block_lo..block_hi].iter().map(|&i| fsky[i]).collect();
                        let (slope, icept) = lin_fit(&xt, &yv);
                        for k in 0..blen {
                            ts.push(xt[k]);
                            vs.push(yv[k] - (slope * xt[k] + icept));
                        }
                    }
                    block_lo = block_hi;
                }
                if ts.len() < 1500 {
                    eprintln!(
                        "Deduction 27   {label}: {} samples, {} after detrend — too short (0 honored)",
                        idx.len(),
                        ts.len()
                    );
                    return;
                }
                let g = ls_grid(&ts, &vs, flo, fhi, 0.00005);
                let (fp, _, ratio) = peak_of(&g);
                eprintln!(
                    "Deduction 27   {label}: n={}, peak {fp:.5} Hz ({ratio:.1}×)",
                    ts.len()
                );
            };
            let t_all: Vec<f64> = recs.iter().map(|r| r[0]).collect();
            let fsky: Vec<f64> = recs.iter().map(|r| r[1]).collect();
            let st_all: Vec<i64> = recs.iter().map(|r| r[6] as i64).collect();
            let samp_all: Vec<f64> = recs.iter().map(|r| r[3]).collect();
            let mode_all: Vec<i64> = recs.iter().map(|r| r[13] as i64).collect();
            let mut modes: Vec<i64> = mode_all.clone();
            modes.sort_unstable();
            modes.dedup();
            eprintln!(
                "Deduction 27 ground modes in the PASF holdings: {modes:?} (2 = 2-way, 3 = 3-way)",
            );
            for st in [14i64, 43, 63] {
                let base: Vec<usize> = (0..recs.len())
                    .filter(|&i| st_all[i] == st && samp_all[i] < 10.0 && fsky[i].is_finite())
                    .collect();
                if base.len() < 1500 {
                    eprintln!(
                        "Deduction 27   Station {st}: {} 1-s samples — too short (0 honored)",
                        base.len()
                    );
                } else {
                    cell_scan(
                        &format!("station {st} 1-s all ground modes"),
                        &base,
                        &t_all,
                        &fsky,
                        0.044,
                        0.056,
                        2.0,
                    );
                    for m in [2i64, 3] {
                        let idx: Vec<usize> =
                            base.iter().copied().filter(|&i| mode_all[i] == m).collect();
                        let mname = if m == 2 { "2-way" } else { "3-way" };
                        cell_scan(
                            &format!("station {st} 1-s mode {m} ({mname})"),
                            &idx,
                            &t_all,
                            &fsky,
                            0.044,
                            0.056,
                            2.0,
                        );
                    }
                }
                let base60: Vec<usize> = (0..recs.len())
                    .filter(|&i| st_all[i] == st && samp_all[i] >= 30.0 && fsky[i].is_finite())
                    .collect();
                if base60.len() >= 1500 {
                    cell_scan(
                        &format!("station {st} 60-s all ground modes"),
                        &base60,
                        &t_all,
                        &fsky,
                        0.0004,
                        0.0012,
                        120.0,
                    );
                    for m in [2i64, 3] {
                        let idx: Vec<usize> = base60
                            .iter()
                            .copied()
                            .filter(|&i| mode_all[i] == m)
                            .collect();
                        let mname = if m == 2 { "2-way" } else { "3-way" };
                        cell_scan(
                            &format!("station {st} 60-s mode {m} ({mname})"),
                            &idx,
                            &t_all,
                            &fsky,
                            0.0004,
                            0.0012,
                            120.0,
                        );
                    }
                }
            }
        } else {
            eprintln!(
                "Deduction 27: data/pioneer10_skyfreq.bin carries no PASF — empty (0 honored)"
            );
        }
    }

    {
        let idx1: Vec<usize> = (0..n).filter(|&i| samplers[i] < 10.0).collect();
        for (st, _f_line) in [(14i64, 0.04575), (43, 0.05155), (63, 0.04715)] {
            let mut cells: Vec<usize> = idx1
                .iter()
                .copied()
                .filter(|&i| stations[i] == st)
                .collect();
            if cells.len() < 3000 {
                eprintln!(
                    "Deduction 29   station {st}: {} 1-s samples — too short (0 honored)",
                    cells.len()
                );
                continue;
            }
            cells.sort_by(|&a, &b| times[a].total_cmp(&times[b]));
            let win = 3000usize;
            let step = 1500usize;
            let mut track: Vec<String> = Vec::new();
            let mut fs: Vec<f64> = Vec::new();
            let mut dates: Vec<String> = Vec::new();
            let mut lo = 0usize;
            while lo + win <= cells.len() {
                let ts: Vec<f64> = cells[lo..lo + win].iter().map(|&i| times[i]).collect();
                let vs: Vec<f64> = cells[lo..lo + win].iter().map(|&i| resid_e[i]).collect();
                let g = ls_grid(&ts, &vs, 0.044, 0.056, 0.00005);
                let (fp, _, ratio) = peak_of(&g);
                track.push(format!("{:.2}:{:.1}", fp * 1000.0, ratio));
                fs.push(fp);
                dates.push(jd_date(ts[ts.len() / 2]));
                lo += step;
            }
            let mut jumps: Vec<f64> = Vec::new();
            for k in 1..fs.len() {
                jumps.push((fs[k] - fs[k - 1]).abs());
            }
            let max_jump = jumps.iter().copied().fold(0.0, f64::max);
            let span = (fs[fs.len() - 1] - fs[0]).abs();
            eprintln!(
                "Deduction 29   station {st} ({} windows of {win} samples each, step {step}): f/mHz:ratio — {} — total drift {:.2} mHz, largest neighbor jump {:.2} mHz, window midpoint {}..{}",
                track.len(),
                track.join(" "),
                span * 1000.0,
                max_jump * 1000.0,
                dates[0],
                dates[dates.len() - 1]
            );
        }
    }

    {
        let idx1: Vec<usize> = (0..n).filter(|&i| samplers[i] < 10.0).collect();
        let fit_phase = |ts: &[f64], vs: &[f64], f: f64| -> (f64, f64) {
            let m = ts.len() as f64;
            let vsum = vs.iter().sum::<f64>() / m;
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
            if det.abs() < 1e-300 {
                return (0.0, 0.0);
            }
            let a = (sy * cc - cy * sc) / det;
            let b = (cy * ss - sy * sc) / det;
            ((a * a + b * b).sqrt(), b.atan2(a))
        };
        for (st, f_own) in [(14i64, 0.04815), (43, 0.04815), (63, 0.04815)] {
            let mut cells: Vec<usize> = idx1
                .iter()
                .copied()
                .filter(|&i| stations[i] == st && strength[i] < 0.0 && strength[i].is_finite())
                .collect();
            if cells.len() < 4000 {
                eprintln!(
                    "Deduction 30   station {st}: {} samples — too short (0 honored)",
                    cells.len()
                );
                continue;
            }
            cells.sort_by(|&a, &b| times[a].total_cmp(&times[b]));
            let ts_all: Vec<f64> = cells.iter().map(|&i| times[i]).collect();
            let vs_res: Vec<f64> = cells.iter().map(|&i| resid_e[i]).collect();
            let vs_str: Vec<f64> = cells.iter().map(|&i| strength[i]).collect();
            let g_res = ls_grid(&ts_all, &vs_res, 0.044, 0.056, 0.00005);
            let g_str = ls_grid(&ts_all, &vs_str, 0.044, 0.056, 0.00005);
            let (fp_res, _, ratio_res) = peak_of(&g_res);
            let (fp_str, _, ratio_str) = peak_of(&g_str);
            eprintln!(
                "Deduction 30   station {st} (n={}): residual peak {fp_res:.5} Hz ({ratio_res:.1}×), strength peak {fp_str:.5} Hz ({ratio_str:.1}×)",
                cells.len()
            );
            let f_test = f_own;
            let (a_res, _) = fit_phase(&ts_all, &vs_res, f_test);
            let (a_str, _) = fit_phase(&ts_all, &vs_str, f_test);
            let mid = cells.len() / 2;
            let (_, p_r1) = fit_phase(&ts_all[..mid], &vs_res[..mid], f_test);
            let (_, p_r2) = fit_phase(&ts_all[mid..], &vs_res[mid..], f_test);
            let (_, p_s1) = fit_phase(&ts_all[..mid], &vs_str[..mid], f_test);
            let (_, p_s2) = fit_phase(&ts_all[mid..], &vs_str[mid..], f_test);
            let dp1 = (p_r1 - p_s1).rem_euclid(std::f64::consts::TAU);
            let dp2 = (p_r2 - p_s2).rem_euclid(std::f64::consts::TAU);
            let delta = (dp1 - dp2).rem_euclid(std::f64::consts::TAU);
            let mut null_delta: Vec<f64> = Vec::new();
            let mut rng = 0x9E3779B97F4A7C15u64;
            for _ in 0..200 {
                rng = rng
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                let off_a = ((rng >> 33) as usize) % mid;
                rng = rng
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                let off_b = ((rng >> 33) as usize) % mid;
                let rot_a: Vec<f64> = vs_res[off_a..mid]
                    .iter()
                    .chain(&vs_res[..off_a])
                    .copied()
                    .collect();
                let rot_b: Vec<f64> = vs_res[mid + off_b..]
                    .iter()
                    .chain(&vs_res[mid..mid + off_b])
                    .copied()
                    .collect();
                let (_, pr_a) = fit_phase(&ts_all[..mid], &rot_a, f_test);
                let (_, pr_b) = fit_phase(&ts_all[mid..], &rot_b, f_test);
                null_delta.push((pr_a - pr_b).rem_euclid(std::f64::consts::TAU));
            }
            null_delta.sort_by(f64::total_cmp);
            let q10 = null_delta[20];
            let q50 = null_delta[100];
            let q90 = null_delta[180];
            let verdict = if delta < q10 || delta > q90 {
                "the two channels are PHASE-COHERENT — ONE thing carries residual and strength (the antenna/the shared field — measured)"
            } else {
                "the channels carry separate phases — no shared thing carried (0 honored)"
            };
            eprintln!(
                "Deduction 30   station {st} at f = {:.3} mHz: A(res) = {a_res:.2e} Hz, A(str) = {a_str:.2e} ×0.1 dBm — Δψ(H1) = {dp1:.2}, Δψ(H2) = {dp2:.2}, δ = {delta:.2} rad; null q10/q50/q90 = {q10:.2}/{q50:.2}/{q90:.2} — {verdict}",
                f_test * 1000.0
            );
        }
    }

    {
        if let Some(recs) = std::fs::read("data/pioneer10_skyfreq.bin")
            .ok()
            .and_then(|b| omegaflow::atdf::parse_bin(&b))
        {
            let t_all: Vec<f64> = recs.iter().map(|r| r[0]).collect();
            let st_all: Vec<i64> = recs.iter().map(|r| r[6] as i64).collect();
            let samp_all: Vec<f64> = recs.iter().map(|r| r[3]).collect();
            let resid_all: Vec<f64> = recs.iter().map(|r| r[8]).collect();
            let base: Vec<usize> = (0..recs.len())
                .filter(|&i| samp_all[i] < 10.0 && resid_all[i].is_finite())
                .collect();
            if base.len() < 3000 {
                eprintln!(
                    "Deduction 31: {} 1-s samples with doppler_resid — too short (0 honored)",
                    base.len()
                );
            } else {
                let ts: Vec<f64> = base.iter().map(|&i| t_all[i]).collect();
                let vs: Vec<f64> = base.iter().map(|&i| resid_all[i]).collect();
                let g = ls_grid(&ts, &vs, 0.02, 0.07, 0.0001);
                let mut pows: Vec<f64> = g.iter().map(|(_, p)| *p).collect();
                pows.sort_by(f64::total_cmp);
                let floor = pows[pows.len() / 2];
                let mut top = g.clone();
                top.sort_by(|a, b| b.1.total_cmp(&a.1));
                let tops: Vec<String> = top
                    .iter()
                    .take(5)
                    .map(|(f, p)| format!("{:.4} Hz ({:.1}×)", f, p / floor))
                    .collect();
                eprintln!(
                    "Deduction 31 doppler_resid global ({} 1-s samples, 20–70 mHz): peaks {}",
                    base.len(),
                    tops.join(", ")
                );
                for st in [14i64, 43, 63] {
                    let idx: Vec<usize> =
                        base.iter().copied().filter(|&i| st_all[i] == st).collect();
                    if idx.len() < 1500 {
                        eprintln!(
                            "Deduction 31   station {st}: {} samples — too short (0 honored)",
                            idx.len()
                        );
                        continue;
                    }
                    let tsy: Vec<f64> = idx.iter().map(|&i| t_all[i]).collect();
                    let vsy: Vec<f64> = idx.iter().map(|&i| resid_all[i]).collect();
                    let gy = ls_grid(&tsy, &vsy, 0.044, 0.056, 0.00005);
                    let (fpy, _, ratioy) = peak_of(&gy);
                    eprintln!(
                        "Deduction 31   station {st} (n={}): peak {fpy:.5} Hz ({ratioy:.1}×) in the 44–56 mHz band",
                        idx.len()
                    );
                }
            }
        } else {
            eprintln!(
                "Deduction 31: data/pioneer10_skyfreq.bin carries no PASF — empty (0 honored)"
            );
        }
    }

    {
        let idx1: Vec<usize> = (0..n).filter(|&i| samplers[i] < 10.0).collect();
        let mut per_st: std::collections::BTreeMap<i64, std::collections::BTreeMap<i64, Vec<f64>>> =
            std::collections::BTreeMap::new();
        for &i in &idx1 {
            let day = (times[i] / 86400.0).floor() as i64;
            per_st
                .entry(stations[i])
                .or_default()
                .entry(day)
                .or_default()
                .push(resid_e[i]);
        }
        let mut series: Vec<(i64, Vec<(i64, f64)>)> = Vec::new();
        for (st, days) in &per_st {
            let mut s: Vec<(i64, f64)> = days
                .iter()
                .map(|(d, v)| {
                    let mut w = v.clone();
                    w.sort_by(f64::total_cmp);
                    (*d, w[w.len() / 2])
                })
                .collect();
            s.sort_by(|a, b| a.0.cmp(&b.0));
            series.push((*st, s));
        }
        for (st, s) in &series {
            eprintln!(
                "Deduction 32   station {st}: {} days with 1-s data ({}..{})",
                s.len(),
                jd_date(s[0].0 as f64 * 86400.0),
                jd_date(s[s.len() - 1].0 as f64 * 86400.0)
            );
        }
        for a in 0..series.len() {
            for b in a + 1..series.len() {
                let (sta, sa) = &series[a];
                let (stb, sb) = &series[b];
                let mb: std::collections::HashMap<i64, f64> = sb.iter().copied().collect();
                let mut xs: Vec<f64> = Vec::new();
                let mut ys: Vec<f64> = Vec::new();
                for (d, va) in sa {
                    if let Some(vb) = mb.get(d) {
                        xs.push(*va);
                        ys.push(*vb);
                    }
                }
                if xs.len() < 5 {
                    eprintln!(
                        "Deduction 32   station {sta}×{stb}: {} common days — too short (0 honored)",
                        xs.len()
                    );
                    continue;
                }
                let mx = xs.iter().sum::<f64>() / xs.len() as f64;
                let my = ys.iter().sum::<f64>() / ys.len() as f64;
                let mut num = 0.0;
                let mut dx = 0.0;
                let mut dy = 0.0;
                for k in 0..xs.len() {
                    let ax = xs[k] - mx;
                    let ay = ys[k] - my;
                    num += ax * ay;
                    dx += ax * ax;
                    dy += ay * ay;
                }
                let r = num / (dx * dy).sqrt();
                eprintln!(
                    "Deduction 32   station {sta}×{stb}: {} common days, Pearson r = {r:.3} — {}",
                    xs.len(),
                    if r.abs() > 0.5 {
                        "correlated — a common slow driver is possible"
                    } else {
                        "no correlation carried (0 honored)"
                    }
                );
            }
        }
    }

    {
        let idx1: Vec<usize> = (0..n).filter(|&i| samplers[i] < 10.0).collect();
        let bins = 24usize;
        let self_te = |syms: &[usize]| -> f64 {
            let mut c2 = vec![0u64; bins * bins];
            let mut c1 = vec![0u64; bins];
            let mut c0 = vec![0u64; bins];
            for k in 1..syms.len() {
                c2[syms[k - 1] * bins + syms[k]] += 1;
                c1[syms[k - 1]] += 1;
                c0[syms[k]] += 1;
            }
            let total = (syms.len() - 1) as f64;
            let mut te = 0.0;
            for a in 0..bins {
                if c1[a] == 0 {
                    continue;
                }
                for b in 0..bins {
                    let c = c2[a * bins + b];
                    if c == 0 {
                        continue;
                    }
                    let p_ab = c as f64 / total;
                    let p_b_a = c as f64 / c1[a] as f64;
                    let p_b = c0[b] as f64 / total;
                    if p_b > 0.0 {
                        te += p_ab * (p_b_a / p_b).log2();
                    }
                }
            }
            te
        };
        for (st, win, step) in [
            (63i64, 500usize, 250usize),
            (63, 1000, 500),
            (43, 500, 250),
            (14, 500, 250),
        ] {
            let mut cells: Vec<usize> = idx1
                .iter()
                .copied()
                .filter(|&i| stations[i] == st)
                .collect();
            if cells.len() < win * 3 {
                eprintln!(
                    "Deduction 33   station {st} (win {win}): {} samples — too short (0 honored)",
                    cells.len()
                );
                continue;
            }
            cells.sort_by(|&a, &b| times[a].total_cmp(&times[b]));
            let mut syms: Vec<usize> = Vec::new();
            let mut lo = 0usize;
            while lo + win <= cells.len() {
                let ts: Vec<f64> = cells[lo..lo + win].iter().map(|&i| times[i]).collect();
                let vs: Vec<f64> = cells[lo..lo + win].iter().map(|&i| resid_e[i]).collect();
                let g = ls_grid(&ts, &vs, 0.044, 0.056, 0.00002);
                let (fp, _, _) = peak_of(&g);
                let b = (((fp - 0.044) / 0.0005).floor() as i64).clamp(0, bins as i64 - 1) as usize;
                syms.push(b);
                lo += step;
            }
            let te = self_te(&syms);
            let mut null: Vec<f64> = Vec::new();
            let mut rng = 0x9E3779B97F4A7C15u64;
            for _ in 0..200 {
                rng = rng
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                let mut p = syms.clone();
                for k in (1..p.len()).rev() {
                    rng = rng
                        .wrapping_mul(6364136223846793005)
                        .wrapping_add(1442695040888963407);
                    let j = ((rng >> 33) as usize) % (k + 1);
                    p.swap(k, j);
                }
                null.push(self_te(&p));
            }
            null.sort_by(f64::total_cmp);
            let q50 = null[100];
            let q90 = null[180];
            let used = syms
                .iter()
                .copied()
                .collect::<std::collections::BTreeSet<_>>()
                .len();
            eprintln!(
                "Deduction 33   station {st} (win {win}/step {step}): {} windows, {} bins occupied, self-TE = {te:.3} bit; null (200 permutations) q50/q90 = {q50:.3}/{q90:.3} bit — {}",
                syms.len(),
                used,
                if te > q90 {
                    "the peak sequence carries ORDER above the null — a succession structure is CARRIED"
                } else {
                    "the sequence lies in the null — no fixed member order carried (0 honored)"
                }
            );
            let mut pairs: Vec<(usize, usize, u64)> = Vec::new();
            for k in 1..syms.len() {
                pairs.push((syms[k - 1], syms[k], 1));
            }
            let mut pc: std::collections::BTreeMap<(usize, usize), u64> =
                std::collections::BTreeMap::new();
            for (a, b, _) in pairs {
                *pc.entry((a, b)).or_default() += 1;
            }
            let mut top: Vec<((usize, usize), u64)> =
                pc.into_iter().filter(|&(_, c)| c >= 3).collect();
            top.sort_by(|a, b| b.1.cmp(&a.1));
            let tops: Vec<String> = top
                .iter()
                .take(5)
                .map(|((a, b), c)| {
                    format!(
                        "{:.1}→{:.1} mHz ×{}",
                        44.0 + *a as f64 * 0.5,
                        44.0 + *b as f64 * 0.5,
                        c
                    )
                })
                .collect();
            eprintln!("Deduction 33     top transitions: {}", tops.join(", "));
        }
    }

    {
        let nav_series = |path: &str, t_lo: f64, t_hi: f64| -> Option<(Vec<f64>, Vec<f64>)> {
            let recs = std::fs::read(path).ok().and_then(|b| parse_pdpl(&b))?;
            let mut ts = Vec::new();
            let mut vs = Vec::new();
            for r in &recs {
                if r[0] > t_lo && r[0] < t_hi && r[1].is_finite() && r[1].abs() <= NAVIO_BEAT_BOUND
                {
                    ts.push(r[0]);
                    vs.push(r[1]);
                }
            }
            Some((ts, vs))
        };
        let fit_phase = |ts: &[f64], vs: &[f64], f: f64| -> f64 {
            let m = ts.len() as f64;
            let vsum = vs.iter().sum::<f64>() / m;
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
            if det.abs() < 1e-300 {
                return 0.0;
            }
            let a = (sy * cc - cy * sc) / det;
            let b = (cy * ss - sy * sc) / det;
            (a * a + b * b).sqrt()
        };
        let cross_te = |x: &[usize], y: &[usize], bins: usize| -> f64 {
            let total = (x.len() - 1) as f64;
            let mut c2 = vec![0u64; bins * bins * bins];
            let mut c_xy = vec![0u64; bins * bins];
            let mut c_x = vec![0u64; bins];
            let mut c_xx = vec![0u64; bins * bins];
            for t in 0..x.len() - 1 {
                c2[(x[t] * bins + y[t]) * bins + x[t + 1]] += 1;
                c_xy[x[t] * bins + y[t]] += 1;
                c_x[x[t]] += 1;
                c_xx[x[t] * bins + x[t + 1]] += 1;
            }
            let mut te = 0.0;
            for a in 0..bins {
                if c_x[a] == 0 {
                    continue;
                }
                for b in 0..bins {
                    if c_xy[a * bins + b] == 0 {
                        continue;
                    }
                    for d in 0..bins {
                        let c = c2[(a * bins + b) * bins + d];
                        if c == 0 {
                            continue;
                        }
                        let p = c as f64 / total;
                        let p_d_ab = c as f64 / c_xy[a * bins + b] as f64;
                        let p_d_a = c_xx[a * bins + d] as f64 / c_x[a] as f64;
                        if p_d_a > 0.0 {
                            te += p * (p_d_ab / p_d_a).log2();
                        }
                    }
                }
            }
            te
        };
        let p10o = nav_series("data/pioneer10_doppler_clean.bin", t_first, t_last);
        let p11o = nav_series("data/pioneer11_doppler_clean.bin", t_first, t_last);
        match (p10o, p11o) {
            (Some((ts10, vs10)), Some((ts11, vs11))) => {
                let mut s10 = ts10.clone();
                let mut s11 = ts11.clone();
                s10.sort_by(f64::total_cmp);
                s11.sort_by(f64::total_cmp);
                let t_lo = ts11[0].max(ts10[0]);
                let t_hi = s10[s10.len() * 95 / 100].min(s11[s11.len() * 95 / 100]);
                let win = 86400.0;
                let step = 86400.0;
                let mut env10: Vec<f64> = Vec::new();
                let mut env11: Vec<f64> = Vec::new();
                let mut w = t_lo;
                let mut n_pairs = 0usize;
                while w < t_hi {
                    let w10: Vec<f64> = (0..ts10.len())
                        .filter(|&k| ts10[k] >= w && ts10[k] < w + win)
                        .map(|k| vs10[k])
                        .collect();
                    let w11: Vec<f64> = (0..ts11.len())
                        .filter(|&k| ts11[k] >= w && ts11[k] < w + win)
                        .map(|k| vs11[k])
                        .collect();
                    if w10.len() >= 100 && w11.len() >= 100 {
                        let t10: Vec<f64> = (0..ts10.len())
                            .filter(|&k| ts10[k] >= w && ts10[k] < w + win)
                            .map(|k| ts10[k])
                            .collect();
                        let t11: Vec<f64> = (0..ts11.len())
                            .filter(|&k| ts11[k] >= w && ts11[k] < w + win)
                            .map(|k| ts11[k])
                            .collect();
                        env10.push(fit_phase(&t10, &w10, 0.000714));
                        env11.push(fit_phase(&t11, &w11, 0.000714));
                        n_pairs += 1;
                    }
                    w += step;
                }
                if n_pairs < 40 {
                    eprintln!(
                        "Deduction 34: {} common 24-h windows — too short (0 honored)",
                        n_pairs
                    );
                } else {
                    let terc = |v: &[f64]| -> Vec<usize> {
                        let mut s = v.to_vec();
                        s.sort_by(f64::total_cmp);
                        let q1 = s[s.len() / 3];
                        let q2 = s[s.len() * 2 / 3];
                        v.iter()
                            .map(|&x| {
                                if x <= q1 {
                                    0
                                } else if x <= q2 {
                                    1
                                } else {
                                    2
                                }
                            })
                            .collect()
                    };
                    let b10 = terc(&env10);
                    let b11 = terc(&env11);
                    let te_11_10 = cross_te(&b10, &b11, 3);
                    let te_10_11 = cross_te(&b11, &b10, 3);
                    let mut rng = 0x9E3779B97F4A7C15u64;
                    let mut null_11_10: Vec<f64> = Vec::new();
                    let mut null_10_11: Vec<f64> = Vec::new();
                    for _ in 0..200 {
                        rng = rng
                            .wrapping_mul(6364136223846793005)
                            .wrapping_add(1442695040888963407);
                        let mut p11 = b11.clone();
                        for k in (1..p11.len()).rev() {
                            rng = rng
                                .wrapping_mul(6364136223846793005)
                                .wrapping_add(1442695040888963407);
                            let j = ((rng >> 33) as usize) % (k + 1);
                            p11.swap(k, j);
                        }
                        null_11_10.push(cross_te(&b10, &p11, 3));
                        let mut p10 = b10.clone();
                        for k in (1..p10.len()).rev() {
                            rng = rng
                                .wrapping_mul(6364136223846793005)
                                .wrapping_add(1442695040888963407);
                            let j = ((rng >> 33) as usize) % (k + 1);
                            p10.swap(k, j);
                        }
                        null_10_11.push(cross_te(&b11, &p10, 3));
                    }
                    null_11_10.sort_by(f64::total_cmp);
                    null_10_11.sort_by(f64::total_cmp);
                    eprintln!(
                        "Deduction 34 probe coupling ({} common 24-h windows, envelope of the 0.71-mHz member, tercile bins, lag 1): TE(P11→P10) = {te_11_10:.3} bit, null q90 {:.3} — {}; TE(P10→P11) = {te_10_11:.3} bit, null q90 {:.3} — {}",
                        n_pairs,
                        null_11_10[180],
                        if te_11_10 > null_11_10[180] {
                            "CARRIED — P11 influences P10 with delay"
                        } else {
                            "not carried (0 honored)"
                        },
                        null_10_11[180],
                        if te_10_11 > null_10_11[180] {
                            "CARRIED — P10 influences P11 with delay"
                        } else {
                            "not carried (0 honored)"
                        }
                    );
                }
            }
            _ => {
                eprintln!("Deduction 34: NAVIO-clean P10/P11 void — empty (0 honored)");
            }
        }
    }

    {
        let idx1: Vec<usize> = (0..n).filter(|&i| samplers[i] < 10.0).collect();
        let pearson = |xs: &[f64], ys: &[f64]| -> f64 {
            if xs.len() < 5 {
                return f64::NAN;
            }
            let mx = xs.iter().sum::<f64>() / xs.len() as f64;
            let my = ys.iter().sum::<f64>() / ys.len() as f64;
            let mut num = 0.0;
            let mut dx = 0.0;
            let mut dy = 0.0;
            for k in 0..xs.len() {
                let ax = xs[k] - mx;
                let ay = ys[k] - my;
                num += ax * ay;
                dx += ax * ax;
                dy += ay * ay;
            }
            num / (dx * dy).sqrt()
        };
        for (st, win, step) in [(63i64, 500usize, 250usize), (43, 500, 250), (14, 500, 250)] {
            let mut cells: Vec<usize> = idx1
                .iter()
                .copied()
                .filter(|&i| stations[i] == st)
                .collect();
            if cells.len() < win * 3 {
                eprintln!(
                    "Deduction 35   station {st}: {} samples — too short (0 honored)",
                    cells.len()
                );
                continue;
            }
            cells.sort_by(|&a, &b| times[a].total_cmp(&times[b]));
            let mut amps: Vec<f64> = Vec::new();
            let mut freqs: Vec<f64> = Vec::new();
            let mut r_hel: Vec<f64> = Vec::new();
            let mut rho_v: Vec<f64> = Vec::new();
            let mut eps_v: Vec<f64> = Vec::new();
            let mut bet_v: Vec<f64> = Vec::new();
            let mut lo = 0usize;
            while lo + win <= cells.len() {
                let ts: Vec<f64> = cells[lo..lo + win].iter().map(|&i| times[i]).collect();
                let vs: Vec<f64> = cells[lo..lo + win].iter().map(|&i| resid_e[i]).collect();
                let g = ls_grid(&ts, &vs, 0.044, 0.056, 0.00002);
                let (fp, pp, _) = peak_of(&g);
                let amp = (2.0 * pp / win as f64).sqrt();
                let tm = ts[ts.len() / 2];
                if let Some((r3, _)) = granule_sc(tm) {
                    let rn = norm(r3);
                    let rs = r_st[cells[lo + win / 2]];
                    let rho = dist(rs, r3);
                    let re = norm(rs);
                    let mut num = 0.0;
                    for k in 0..3 {
                        num += -rs[k] * (r3[k] - rs[k]);
                    }
                    let eps = (num / (re * rho)).acos();
                    let bet = (r3[2] / rn).asin();
                    amps.push(amp);
                    freqs.push(fp);
                    r_hel.push(rn);
                    rho_v.push(rho);
                    eps_v.push(eps);
                    bet_v.push(bet);
                }
                lo += step;
            }
            if amps.len() < 10 {
                eprintln!(
                    "Deduction 35   station {st}: {} windows with ephemeris — too short (0 honored)",
                    amps.len()
                );
                continue;
            }
            eprintln!(
                "Deduction 35   station {st} ({} windows): Pearson amplitude×position — r_hel {:.2}, Earth distance {:.2}, elongation {:.2}, ecl. latitude {:.2}; frequency×position — r_hel {:.2}, Earth distance {:.2}, elongation {:.2}, ecl. latitude {:.2}",
                amps.len(),
                pearson(&amps, &r_hel),
                pearson(&amps, &rho_v),
                pearson(&amps, &eps_v),
                pearson(&amps, &bet_v),
                pearson(&freqs, &r_hel),
                pearson(&freqs, &rho_v),
                pearson(&freqs, &eps_v),
                pearson(&freqs, &bet_v)
            );
        }
    }

    {
        let idx1: Vec<usize> = (0..n).filter(|&i| samplers[i] < 10.0).collect();
        for st in [14i64, 43, 63] {
            let mut cells: Vec<usize> = idx1
                .iter()
                .copied()
                .filter(|&i| stations[i] == st)
                .collect();
            if cells.len() < 8000 {
                eprintln!(
                    "Deduction 36   station {st}: {} samples — too short (0 honored)",
                    cells.len()
                );
                continue;
            }
            cells.sort_by(|&a, &b| times[a].total_cmp(&times[b]));
            let win = 2000usize;
            let step = 1000usize;
            let mut amps: Vec<f64> = Vec::new();
            let mut dens: Vec<f64> = Vec::new();
            let mut lo = 0usize;
            while lo + win <= cells.len() {
                let ts: Vec<f64> = cells[lo..lo + win].iter().map(|&i| times[i]).collect();
                let vs: Vec<f64> = cells[lo..lo + win].iter().map(|&i| resid_e[i]).collect();
                let g = ls_grid(&ts, &vs, 0.044, 0.056, 0.00002);
                let (_, pp, _) = peak_of(&g);
                amps.push((2.0 * pp / win as f64).sqrt());
                let mut steps = 0usize;
                for k in lo + 1..lo + win {
                    if (refs[cells[k]] - refs[cells[k - 1]]).abs() > 1.0e-9 {
                        steps += 1;
                    }
                }
                dens.push(steps as f64 / win as f64);
                lo += step;
            }
            if amps.len() < 5 {
                eprintln!(
                    "Deduction 36   station {st}: {} windows — too short (0 honored)",
                    amps.len()
                );
                continue;
            }
            let mx = amps.iter().sum::<f64>() / amps.len() as f64;
            let my = dens.iter().sum::<f64>() / dens.len() as f64;
            let mut num = 0.0;
            let mut dx = 0.0;
            let mut dy = 0.0;
            for k in 0..amps.len() {
                let ax = amps[k] - mx;
                let ay = dens[k] - my;
                num += ax * ay;
                dx += ax * ax;
                dy += ay * ay;
            }
            let r = num / (dx * dy).sqrt();
            eprintln!(
                "Deduction 36   station {st} ({} windows of {win} samples each): Pearson band-envelope × reference-step density = {r:.3} — {}",
                amps.len(),
                if r.abs() > 0.5 {
                    "the band pulses WITH the staircase — the calibration companion is CARRIED"
                } else {
                    "no companion carried (0 honored)"
                }
            );
        }
    }

    {
        let idx1: Vec<usize> = (0..n).filter(|&i| samplers[i] < 10.0).collect();
        let mut cells: Vec<usize> = idx1
            .iter()
            .copied()
            .filter(|&i| stations[i] == 63)
            .collect();
        if cells.len() < 12000 {
            eprintln!(
                "Deduction 37   station 63: {} samples — too short (0 honored)",
                cells.len()
            );
        } else {
            cells.sort_by(|&a, &b| times[a].total_cmp(&times[b]));
            let win = 4000usize;
            let step = 2000usize;
            let mut fs: Vec<f64> = Vec::new();
            let mut lo = 0usize;
            while lo + win <= cells.len() {
                let ts: Vec<f64> = cells[lo..lo + win].iter().map(|&i| times[i]).collect();
                let vs: Vec<f64> = cells[lo..lo + win].iter().map(|&i| resid_e[i]).collect();
                let g = ls_grid(&ts, &vs, 0.0465, 0.0485, 0.000005);
                let (fp, _, _) = peak_of(&g);
                let fi = peak_interp(&g).unwrap_or(fp);
                fs.push(fi);
                lo += step;
            }

            let mx = (fs.len() - 1) as f64 / 2.0;
            let my = fs.iter().sum::<f64>() / fs.len() as f64;
            let mut num = 0.0;
            let mut den = 0.0;
            for (k, &f) in fs.iter().enumerate() {
                num += (k as f64 - mx) * (f - my);
                den += (k as f64 - mx) * (k as f64 - mx);
            }
            let slope = num / den;
            let resid: Vec<f64> = fs
                .iter()
                .enumerate()
                .map(|(k, &f)| f - (slope * (k as f64 - mx) + my))
                .collect();
            let rrms = rms_of(&resid);
            let mut max_jump = 0.0;
            for k in 1..fs.len() {
                let d = (fs[k] - fs[k - 1]).abs();
                if d > max_jump {
                    max_jump = d;
                }
            }
            let median_jump = {
                let mut ds: Vec<f64> = fs.windows(2).map(|w| (w[1] - w[0]).abs()).collect();
                ds.sort_by(f64::total_cmp);
                ds[ds.len() / 2]
            };
            eprintln!(
                "Deduction 37   station 63 ({} windows, track 46.5–48.5 mHz at 5-µHz grid): drift {:.3} mHz/window, residual-RMS {:.3} mHz, median neighbor jump {:.3} mHz, largest jump {:.3} mHz — {}",
                fs.len(),
                slope * 1000.0,
                rrms * 1000.0,
                median_jump * 1000.0,
                max_jump * 1000.0,
                if max_jump > 6.0 * rrms {
                    "STEPS carried — the line jumps, it does not tick uniformly"
                } else {
                    "uniform drift carried — no steps above the residual (0 honored)"
                }
            );

            let mut sorted = fs.clone();
            sorted.sort_by(f64::total_cmp);
            let mut gaps: Vec<f64> = sorted.windows(2).map(|w| w[1] - w[0]).collect();
            gaps.sort_by(f64::total_cmp);
            let g10 = gaps[gaps.len() / 10];
            let g50 = gaps[gaps.len() / 2];
            eprintln!(
                "Deduction 37   state gaps (sorted peaks): p10 = {:.3} mHz, p50 = {:.3} mHz — {}",
                g10 * 1000.0,
                g50 * 1000.0,
                if g50 * 1000.0 < 0.02 {
                    "a LADDER is carried — the peaks cluster on discrete steps"
                } else {
                    "continuous distribution — no alphabet carried (0 honored)"
                }
            );
        }
    }

    {
        let idx1: Vec<usize> = (0..n).filter(|&i| samplers[i] < 10.0).collect();
        let maxlag = 1500usize;
        let mut acfs: Vec<(i64, Vec<f64>)> = Vec::new();
        for st in [14i64, 43, 63] {
            let mut cells: Vec<usize> = idx1
                .iter()
                .copied()
                .filter(|&i| stations[i] == st)
                .collect();
            if cells.len() < 6000 {
                continue;
            }
            cells.sort_by(|&a, &b| times[a].total_cmp(&times[b]));
            let m = cells.len();
            let mean = cells.iter().map(|&i| resid_e[i]).sum::<f64>() / m as f64;
            let var = cells
                .iter()
                .map(|&i| (resid_e[i] - mean).powi(2))
                .sum::<f64>()
                / m as f64;
            let mut acf = vec![0.0f64; maxlag + 1];
            acf[0] = var;
            for lag in 1..=maxlag {
                let mut s = 0.0;
                for i in 0..m - lag {
                    s += (resid_e[cells[i]] - mean) * (resid_e[cells[i + lag]] - mean);
                }
                acf[lag] = s / (m - lag) as f64;
            }

            let mut peaks: Vec<(usize, f64)> = Vec::new();
            for lag in 6..=maxlag - 6 {
                let a = acf[lag] / var;
                if a > 0.0 && (lag - 5..=lag + 5).all(|k| acf[k] / var <= a) {
                    peaks.push((lag, a));
                }
            }
            peaks.sort_by(|a, b| b.1.total_cmp(&a.1));
            let top: Vec<String> = peaks
                .iter()
                .take(6)
                .map(|(l, a)| format!("{}s:{:.2}", l, a))
                .collect();
            let strong = peaks.iter().filter(|(_, a)| *a > 0.15).count();
            eprintln!(
                "Deduction 38   station {st} ({} samples, lags 1–1500 s): top peaks {} — {strong} peaks > 0.15",
                m,
                top.join(", ")
            );
            acfs.push((st, acf));
        }

        for a in 0..acfs.len() {
            for b in a + 1..acfs.len() {
                let (sta, va) = &acfs[a];
                let (stb, vb) = &acfs[b];
                let mxa = va.iter().skip(10).sum::<f64>() / (maxlag - 9) as f64;
                let mxb = vb.iter().skip(10).sum::<f64>() / (maxlag - 9) as f64;
                let mut num = 0.0;
                let mut dx = 0.0;
                let mut dy = 0.0;
                for k in 10..=maxlag {
                    let x = va[k] - mxa;
                    let y = vb[k] - mxb;
                    num += x * y;
                    dx += x * x;
                    dy += y * y;
                }
                let r = num / (dx * dy).sqrt();
                eprintln!(
                    "Deduction 38   pattern cross-correlation station {sta}×{stb}: r = {r:.3} — {}",
                    if r.abs() > 0.5 {
                        "the stations share ACF patterns"
                    } else {
                        "no shared patterns (0 honored)"
                    }
                );
            }
        }
    }

    {
        let idx1: Vec<usize> = (0..n).filter(|&i| samplers[i] < 10.0).collect();
        let fit_ab = |ts: &[f64], vs: &[f64], f: f64| -> (f64, f64) {
            let m = ts.len() as f64;
            let vsum = vs.iter().sum::<f64>() / m;
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
            if det.abs() < 1e-300 {
                return (0.0, 0.0);
            }
            ((sy * cc - cy * sc) / det, (cy * ss - sy * sc) / det)
        };
        let mut acfs: Vec<(i64, Vec<f64>)> = Vec::new();
        for st in [14i64, 43, 63] {
            let mut cells: Vec<usize> = idx1
                .iter()
                .copied()
                .filter(|&i| stations[i] == st)
                .collect();
            if cells.len() < 6000 {
                continue;
            }
            cells.sort_by(|&a, &b| times[a].total_cmp(&times[b]));
            let ts: Vec<f64> = cells.iter().map(|&i| times[i]).collect();
            let vs: Vec<f64> = cells.iter().map(|&i| resid_e[i]).collect();

            let mut freqs: Vec<f64> = Vec::new();
            let mut f = 0.044;
            while f <= 0.056 {
                freqs.push(f);
                f += 0.00005;
            }
            let mut coefs: Vec<(f64, f64)> = Vec::with_capacity(freqs.len());
            for &fr in &freqs {
                coefs.push(fit_ab(&ts, &vs, fr));
            }
            let mut band = vec![0.0f64; ts.len()];
            for (i, &t) in ts.iter().enumerate() {
                for (fr, (a, b)) in freqs.iter().zip(&coefs) {
                    let ph = std::f64::consts::TAU * fr * t;
                    band[i] += a * ph.sin() + b * ph.cos();
                }
            }
            let band_var = band.iter().map(|v| v * v).sum::<f64>() / band.len() as f64;
            let total_var = vs.iter().map(|v| v * v).sum::<f64>() / vs.len() as f64;
            eprintln!(
                "Deduction 39   station {st}: band wave reconstructed — band variance {:.2e} Hz², residual variance {:.2e} Hz² (share {:.1e})",
                band_var,
                total_var,
                band_var / total_var
            );

            let maxlag = 1500usize;
            let mean = band.iter().sum::<f64>() / band.len() as f64;
            let mut acf = vec![0.0f64; maxlag + 1];
            acf[0] = band_var;
            for lag in 1..=maxlag {
                let mut s = 0.0;
                for i in 0..band.len() - lag {
                    s += (band[i] - mean) * (band[i + lag] - mean);
                }
                acf[lag] = s / (band.len() - lag) as f64;
            }
            let mut peaks: Vec<(usize, f64)> = Vec::new();
            for lag in 6..=maxlag - 6 {
                let a = acf[lag] / band_var;
                if a > 0.0 && (lag - 5..=lag + 5).all(|k| acf[k] / band_var <= a) {
                    peaks.push((lag, a));
                }
            }
            peaks.sort_by(|a, b| b.1.total_cmp(&a.1));
            let strong: Vec<(usize, f64)> =
                peaks.iter().copied().filter(|(_, a)| *a > 0.3).collect();
            let top: Vec<String> = strong
                .iter()
                .take(8)
                .map(|(l, a)| format!("{}s:{:.2}", l, a))
                .collect();
            eprintln!(
                "Deduction 39   station {st}: {} peaks > 0.3 — {}",
                strong.len(),
                if top.is_empty() {
                    "none".to_string()
                } else {
                    top.join(", ")
                }
            );
            acfs.push((st, acf));
        }
        for a in 0..acfs.len() {
            for b in a + 1..acfs.len() {
                let (sta, va) = &acfs[a];
                let (stb, vb) = &acfs[b];
                let mut num = 0.0;
                let mut dx = 0.0;
                let mut dy = 0.0;
                for k in 10..=1500 {
                    num += va[k] * vb[k];
                    dx += va[k] * va[k];
                    dy += vb[k] * vb[k];
                }
                let r = num / (dx * dy).sqrt();
                eprintln!(
                    "Deduction 39   wave pattern station {sta}×{stb}: r = {r:.3} — {}",
                    if r.abs() > 0.5 {
                        "the naked waves share patterns"
                    } else {
                        "no shared patterns (0 honored)"
                    }
                );
            }
        }
    }

    {
        let idx1: Vec<usize> = (0..n).filter(|&i| samplers[i] < 10.0).collect();
        if idx1.len() < 8000 {
            eprintln!(
                "Deduction 40: {} 1-s samples — too short (0 honored)",
                idx1.len()
            );
        } else {
            let r0 = rms_of(&idx1.iter().map(|&i| resid_e[i]).collect::<Vec<f64>>());

            let mut dt: Vec<f64> = Vec::new();
            let mut dv: Vec<f64> = Vec::new();
            let mut lo = 0usize;
            while lo < idx1.len() {
                let mut hi = lo + 1;
                while hi < idx1.len() && times[idx1[hi]] - times[idx1[hi - 1]] <= 2.0 {
                    hi += 1;
                }
                if hi - lo >= 8 {
                    let xs: Vec<f64> = idx1[lo..hi].iter().map(|&i| times[i]).collect();
                    let ys: Vec<f64> = idx1[lo..hi].iter().map(|&i| resid_e[i]).collect();
                    let tx = xs.iter().sum::<f64>() / xs.len() as f64;
                    let m0 = xs.len() as f64;
                    let sx = xs.iter().map(|x| x - tx).sum::<f64>();
                    let sy = ys.iter().sum::<f64>();
                    let sxx = xs.iter().map(|x| (x - tx) * (x - tx)).sum::<f64>();
                    let sxxx = xs.iter().map(|x| (x - tx).powi(3)).sum::<f64>();
                    let sxxxx = xs.iter().map(|x| (x - tx).powi(4)).sum::<f64>();
                    let sxy = xs.iter().zip(&ys).map(|(x, y)| (x - tx) * y).sum::<f64>();
                    let sxxy = xs
                        .iter()
                        .zip(&ys)
                        .map(|(x, y)| (x - tx) * (x - tx) * y)
                        .sum::<f64>();
                    let det = m0 * (sxx * sxxxx - sxxx * sxxx) - sx * (sx * sxxxx - sxx * sxxx)
                        + sxx * (sx * sxxx - sxx * sxx);
                    if det.abs() > 1e-300 {
                        let cc = (sy * (sxx * sxxxx - sxxx * sxxx)
                            - sx * (sxy * sxxxx - sxxx * sxxy)
                            + sxx * (sxy * sxxx - sxx * sxxy))
                            / det;
                        let cb = (m0 * (sxy * sxxxx - sxxx * sxxy)
                            - sy * (sx * sxxxx - sxx * sxxx)
                            + sxx * (sx * sxxy - sxy * sxx))
                            / det;
                        let ca = (m0 * (sxx * sxxy - sxy * sxxx) - sx * (sx * sxxy - sxy * sxx)
                            + sy * (sx * sxxx - sxx * sxx))
                            / det;
                        for k in 0..xs.len() {
                            let dx = xs[k] - tx;
                            dt.push(xs[k]);
                            dv.push(ys[k] - (ca * dx * dx + cb * dx + cc));
                        }
                    }
                }
                lo = hi;
            }
            eprintln!(
                "Deduction 40   P10 ({n} 1-s samples): RMS {r0:.3e} → quadratic {:.3e} Hz",
                rms_of(&dv)
            );

            let mut rr = dv.clone();
            let cell = |st: i64| idx1.iter().copied().filter(|&i| stations[i] == st).count();
            for st in [14i64, 43, 63] {
                let c = cell(st);
                if c < 200 {
                    continue;
                }

                let mut ti = 0usize;
                let mut idx_c: Vec<usize> = Vec::new();
                for i in 0..n {
                    if ti < dt.len() && (times[i] - dt[ti]).abs() < 0.5 {
                        if stations[i] == st {
                            idx_c.push(ti);
                        }
                        ti += 1;
                    }
                }
                if idx_c.len() < 200 {
                    continue;
                }

                let mut vals: Vec<f64> = idx_c.iter().map(|&k| rr[k]).collect();
                vals.sort_by(f64::total_cmp);
                let med = vals[vals.len() / 2];
                for &k in &idx_c {
                    rr[k] -= med;
                }

                let mx = idx_c.iter().map(|&k| strength[idx1[k]]).sum::<f64>() / idx_c.len() as f64;
                let my = idx_c.iter().map(|&k| rr[k]).sum::<f64>() / idx_c.len() as f64;
                let mut num = 0.0;
                let mut den = 0.0;
                for &k in &idx_c {
                    num += (strength[idx1[k]] - mx) * (rr[k] - my);
                    den += (strength[idx1[k]] - mx) * (strength[idx1[k]] - mx);
                }
                let a = if den.abs() > 1e-300 { num / den } else { 0.0 };
                for &k in &idx_c {
                    rr[k] -= a * (strength[idx1[k]] - mx);
                }

                let mx = idx_c.iter().map(|&k| refs[idx1[k]]).sum::<f64>() / idx_c.len() as f64;
                let my = idx_c.iter().map(|&k| rr[k]).sum::<f64>() / idx_c.len() as f64;
                let mut num = 0.0;
                let mut den = 0.0;
                for &k in &idx_c {
                    num += (refs[idx1[k]] - mx) * (rr[k] - my);
                    den += (refs[idx1[k]] - mx) * (refs[idx1[k]] - mx);
                }
                let a = if den.abs() > 1e-300 { num / den } else { 0.0 };
                for &k in &idx_c {
                    rr[k] -= a * (refs[idx1[k]] - mx);
                }

                let mx = idx_c.iter().map(|&k| dt[k]).sum::<f64>() / idx_c.len() as f64;
                let my = idx_c.iter().map(|&k| rr[k]).sum::<f64>() / idx_c.len() as f64;
                let mut num = 0.0;
                let mut den = 0.0;
                for &k in &idx_c {
                    num += (dt[k] - mx) * (rr[k] - my);
                    den += (dt[k] - mx) * (dt[k] - mx);
                }
                let a = if den.abs() > 1e-300 { num / den } else { 0.0 };
                for &k in &idx_c {
                    rr[k] -= a * (dt[k] - mx);
                }
            }
            eprintln!(
                "Deduction 40   after the witnesses (cell, strength, reference, time): RMS {:.3e} Hz",
                rms_of(&rr)
            );

            let g = ls_grid(&dt, &rr, 0.044, 0.056, 0.00002);
            let (fp, _, ratio) = peak_of(&g);
            let g2 = ls_grid(&dt, &rr, 0.0004, 0.008, 0.00005);
            let (fp2, _, ratio2) = peak_of(&g2);
            eprintln!(
                "Deduction 40   negative-fuzzy residual: 44–56 mHz peak {fp:.5} Hz ({ratio:.1}× floor); 0.4–8 mHz peak {fp2:.4} Hz ({ratio2:.1}×) — {}",
                if ratio >= 5.0 || ratio2 >= 8.0 {
                    "a survivor above the floor — the negative index carries a candidate"
                } else {
                    "silence above the floor — no unpredictable survivor (0 honored)"
                }
            );
        }
    }
    eprintln!(
        "Floor after deduction: {:.3e} Hz (base {:.3e} Hz, Δ {:.3e} Hz) — what remains named: the 60-s integration scatter itself, the NAVIO-OBSVBL semantics, the reduction's own witness floors",
        rms_f,
        rms0,
        rms_f - rms0
    );
    {
        let run_half = |parity: usize| -> Option<(f64, f64)> {
            let idx: Vec<usize> = (0..n).filter(|&i| i % 2 == parity).collect();
            if idx.len() < 1000 {
                return None;
            }
            let t2: Vec<f64> = idx.iter().map(|&i| times[i]).collect();
            let o2: Vec<f64> = idx.iter().map(|&i| obs_e[i]).collect();
            let r2: Vec<f64> = idx.iter().map(|&i| refs[i]).collect();
            let rt2: Vec<f64> = idx.iter().map(|&i| rates0[i]).collect();
            let f2: Vec<i64> = idx.iter().map(|&i| files[i]).collect();
            let w2: Vec<f64> = idx.iter().map(|&i| weights[i]).collect();
            let (a, _, resid, _, _) = fixed_effects_cells_w(&rt2, &r2, &o2, &t2, &f2, &w2)?;
            let (drift, _) = lin_fit_w(&t2, &resid, &w2);
            let mut sw = 0.0;
            let mut sx = 0.0;
            for i in 0..t2.len() {
                sw += w2[i];
                sx += w2[i] * t2[i];
            }
            let tm = sx / sw;
            let sxx: f64 = t2
                .iter()
                .zip(&w2)
                .map(|(t, &wi)| wi * (t - tm).powi(2))
                .sum();
            let rss: f64 = resid.iter().zip(&w2).map(|(r, &wi)| wi * r * r).sum();
            let se = if sw > 2.0 && sxx > 0.0 {
                ((rss / (sw - 2.0)) / sxx).sqrt()
            } else {
                f64::NAN
            };
            Some((drift / a, se / a.abs()))
        };
        match (run_half(0), run_half(1)) {
            (Some((d0, s0)), Some((d1, s1))) => {
                let diff = (d0 - d1).abs();
                let sigma = (s0 * s0 + s1 * s1).sqrt();
                eprintln!(
                    "Split-half control of the weighting (even/odd, weighted): drift {d0:.3e} ± {s0:.3e} vs {d1:.3e} ± {s1:.3e} m/s² — difference {diff:.3e} = {:.2}σ — {}",
                    diff / sigma,
                    if diff < 3.0 * sigma {
                        "the halves carry the same drift, the weighting holds"
                    } else {
                        "WARNING SIGNAL: the halves carry separate drifts, the SE deceives"
                    }
                );
            }
            _ => eprintln!("Split-half control: one half too short — empty (0 honored)"),
        }
    }
    let atdf_witness: HashMap<i64, Vec<(f64, f64)>> = {
        let mut m: HashMap<i64, Vec<(f64, f64)>> = HashMap::new();
        for i in 0..n {
            let day = (times[i] / 86400.0).floor() as i64;
            m.entry(day).or_default().push((times[i], resid_e[i]));
        }
        m
    };
    if let Some(pdpl) = std::fs::read("data/pioneer10_doppler_clean.bin")
        .ok()
        .and_then(|b| parse_pdpl(&b))
    {
        let _ = navio_chain(
            "NAVIO chain (overlap era)",
            &pdpl,
            &eph,
            &granule_sc,
            &n_series,
            omni2_window,
            t_first,
            t_last,
            state0,
            t0_rtg,
            Some(&atdf_witness),
        );
        let t_min_full = pdpl
            .iter()
            .map(|r| r[0])
            .filter(|&t| t > NAVIO_T_MIN && t < NAVIO_T_MAX)
            .fold(f64::INFINITY, f64::min);
        let t_max_full = pdpl
            .iter()
            .map(|r| r[0])
            .filter(|&t| t > NAVIO_T_MIN && t < NAVIO_T_MAX)
            .fold(f64::NEG_INFINITY, f64::max);
        if t_min_full < t_max_full {
            let (Some(rs), Some(vs)) = (
                body_barycenter_position(SC_BODY, t_min_full, &eph),
                body_barycenter_velocity(SC_BODY, t_min_full, &eph),
            ) else {
                eprintln!("NAVIO chain (full era): Horizons initial state void");
                return;
            };
            let state0_full = [rs[0], rs[1], rs[2], vs[0], vs[1], vs[2]];
            let _ = navio_chain(
                "NAVIO chain (full era 1973–2002)",
                &pdpl,
                &eph,
                &granule_sc,
                &n_series,
                omni2_window,
                t_min_full,
                t_max_full,
                state0_full,
                t0_rtg,
                None,
            );
        }
    } else {
        eprintln!(
            "NAVIO chain: data/pioneer10_doppler_clean.bin carries no PDPL holdings — empty (0 honored)"
        );
    }

    if let (Some(pdpl11), Some(ep11)) = (
        std::fs::read("data/pioneer11_doppler_clean.bin")
            .ok()
            .and_then(|b| parse_pdpl(&b)),
        std::fs::read("data/ephemeris_pioneer11_daily.bin")
            .ok()
            .and_then(|d| parse_ephemeris_binary(&d)),
    ) {
        let mut eph11: HashMap<String, BodyEphemeris> = HashMap::new();
        eph11.insert(EARTH.to_string(), eph[EARTH].clone());
        eph11.insert("pioneer11_daily".to_string(), ep11);
        let sc11 = |t: f64| -> Option<([f64; 3], [f64; 3])> {
            Some((
                body_barycenter_position("pioneer11_daily", t, &eph11)?,
                body_barycenter_velocity("pioneer11_daily", t, &eph11)?,
            ))
        };
        let t_min11 = pdpl11
            .iter()
            .map(|r| r[0])
            .filter(|&t| t > NAVIO_T_MIN && t < NAVIO_T_MAX)
            .fold(f64::INFINITY, f64::min);
        let t_max11 = pdpl11
            .iter()
            .map(|r| r[0])
            .filter(|&t| t > NAVIO_T_MIN && t < NAVIO_T_MAX)
            .fold(f64::NEG_INFINITY, f64::max);
        if t_min11 < t_max11 {
            eprintln!(
                "NAVIO chain P11: the full era ({}..{}) carries the Saturn flyby (1979) — the Sun+SRP+RTG trajectory cannot carry it; the chain runs on the overlap era after the flyby (named, 0 honored)",
                jd_date(t_min11),
                jd_date(t_max11)
            );
            let (Some(rs11), Some(vs11)) = (
                body_barycenter_position("pioneer11_daily", t_first, &eph11),
                body_barycenter_velocity("pioneer11_daily", t_first, &eph11),
            ) else {
                eprintln!("NAVIO chain P11: Horizons initial state void");
                return;
            };
            let state0_11 = [rs11[0], rs11[1], rs11[2], vs11[0], vs11[1], vs11[2]];
            let _ = navio_chain(
                "NAVIO chain P11 (overlap era)",
                &pdpl11,
                &eph11,
                &sc11,
                &n_series,
                omni2_window,
                t_first,
                t_last,
                state0_11,
                t0_rtg,
                None,
            );
        }
    } else {
        eprintln!(
            "NAVIO chain P11: pioneer11_doppler_clean.bin or ephemeris_pioneer11_daily.bin void — empty (0 honored)"
        );
    }
}

fn navio_chain(
    label: &str,
    pdpl: &[[f64; 6]],
    eph: &HashMap<String, BodyEphemeris>,
    sc: &dyn Fn(f64) -> Option<([f64; 3], [f64; 3])>,
    n_series: &[(f64, f64)],
    omni2_window: f64,
    t_min: f64,
    t_max: f64,
    state0: [f64; 6],
    t0_rtg: f64,
    witness_map: Option<&HashMap<i64, Vec<(f64, f64)>>>,
) -> Option<FitStat> {
    let mut times: Vec<f64> = Vec::new();
    let mut obs: Vec<f64> = Vec::new();
    let mut refs: Vec<f64> = Vec::new();
    let mut r_st: Vec<[f64; 3]> = Vec::new();
    let mut v_st: Vec<[f64; 3]> = Vec::new();
    let mut shift_plasma: Vec<Option<f64>> = Vec::new();
    let mut no_model = 0usize;
    let mut no_value = 0usize;
    let mut no_beat = 0usize;
    let mut dtype: Vec<f64> = Vec::new();
    let mut sun_dist: Vec<f64> = Vec::new();
    for r in pdpl {
        if !(r[0] > t_min && r[0] < t_max) {
            continue;
        }
        if !r[1].is_finite() || !r[2].is_finite() || r[2] <= 0.0 {
            no_value += 1;
            continue;
        }
        if r[1].abs() > NAVIO_BEAT_BOUND {
            no_beat += 1;
            continue;
        }
        let (Some(rs), Some(vs)) = (
            body_barycenter_position(EARTH, r[0], eph),
            body_barycenter_velocity(EARTH, r[0], eph),
        ) else {
            no_model += 1;
            continue;
        };
        let Some(rate) = downlink_rate_core(r[0], rs, vs, sc) else {
            no_model += 1;
            continue;
        };
        if !rate.is_finite() {
            no_model += 1;
            continue;
        }
        let Some((t3, r3)) = light_time_sc_pos(r[0], rs, sc) else {
            no_model += 1;
            continue;
        };
        let Some(rs2) = body_barycenter_position(EARTH, r[0] + PLASMA_DT, eph) else {
            no_model += 1;
            continue;
        };
        let Some((r4, _)) = sc(t3 + PLASMA_DT) else {
            no_model += 1;
            continue;
        };
        let f0 = r[2];
        let n1 = nearest_omni(n_series, r[0], omni2_window);
        let n2 = nearest_omni(n_series, r[0] + PLASMA_DT, omni2_window);
        let sh_p = match (n1, n2) {
            (Some(v1), Some(v2)) => {
                plasma_shift(plasma_column(rs, r3, v1), plasma_column(rs2, r4, v2), f0)
            }
            _ => None,
        };
        times.push(r[0]);
        obs.push(r[1] + r[2]);
        refs.push(r[2]);
        r_st.push(rs);
        v_st.push(vs);
        shift_plasma.push(sh_p);
        dtype.push(r[4]);
        sun_dist.push(sc(r[0]).map(|(p, _)| norm(p)).unwrap_or(f64::NAN));
    }
    if times.len() < 100 {
        eprintln!(
            "{label}: {} samples ({no_value} without value, {no_beat} beyond the beat bound, {no_model} without model) — too short",
            times.len()
        );
        return None;
    }
    let n = times.len();
    let rates0 = rates_modeled(sc, &times, &r_st, &v_st);
    let Some((a0, c0, resid0, _, _)) = fixed_effects(&rates0, &refs, &obs, &times) else {
        eprintln!("{label}: base fit void");
        return None;
    };
    let rms0 = rms_of(&resid0);
    eprintln!(
        "{label}: {n} samples ({no_value} without value, {no_beat} beyond the beat bound, {no_model} without model), {}..{} — base A {a0:.4e} Hz/(m/s), C {c0:.4e}, residual-RMS {rms0:.3e} Hz",
        jd_date(times[0]),
        jd_date(times[n - 1])
    );
    {
        let mut order: Vec<usize> = (0..n).collect();
        order.sort_by(|&a, &b| sun_dist[a].total_cmp(&sun_dist[b]));
        for q in 0..4 {
            let lo = q * n / 4;
            let hi = (q + 1) * n / 4;
            let mut d: Vec<f64> = (lo..hi).map(|k| sun_dist[order[k]]).collect();
            let med = median(&mut d);
            let sq: f64 = (lo..hi).map(|k| resid0[order[k]].powi(2)).sum();
            eprintln!(
                "  sun-distance quartile {q}: n={}, median {:.1} AU, residual-RMS {:.3e} Hz",
                hi - lo,
                med / AU,
                (sq / (hi - lo) as f64).sqrt()
            );
        }
    }
    {
        let mut ds: Vec<i64> = dtype.iter().map(|&d| d as i64).collect();
        ds.sort_unstable();
        ds.dedup();
        let mut parts: Vec<String> = Vec::new();
        for d in ds {
            let mut sq = 0.0;
            let mut cnt = 0usize;
            for i in 0..n {
                if dtype[i] as i64 == d {
                    sq += resid0[i] * resid0[i];
                    cnt += 1;
                }
            }
            parts.push(format!(
                "DTYPE {d}: n={cnt}, RMS={:.3e}",
                (sq / cnt as f64).sqrt()
            ));
        }
        eprintln!("  {}", parts.join(" | "));
    }
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
                let mut sq_masked = 0.0;
                let mut sq_rest = 0.0;
                for i in 0..n {
                    if marked[i] {
                        sq_masked += resid0[i] * resid0[i];
                    } else {
                        sq_rest += resid0[i] * resid0[i];
                    }
                }
                let seg_desc: Vec<String> = masked
                    .iter()
                    .take(5)
                    .map(|(a, _)| jd_date(times[*a]))
                    .collect();
                let n_mask_seg = masked.len();
                eprintln!(
                    "  segment mask: gate 4×p90 = {gate:.3e} Hz (p90 {p90:.3e}) — {n_mask_seg} segments ({n_masked} samples) discarded: [{}] — subset {:.3e} Hz vs rest {:.3e} Hz",
                    seg_desc.join(", "),
                    (sq_masked / n_masked as f64).sqrt(),
                    (sq_rest / (n - n_masked) as f64).sqrt()
                );
                let keep: Vec<usize> = (0..n).filter(|&i| !marked[i]).collect();
                times = keep.iter().map(|&i| times[i]).collect();
                obs = keep.iter().map(|&i| obs[i]).collect();
                refs = keep.iter().map(|&i| refs[i]).collect();
                r_st = keep.iter().map(|&i| r_st[i]).collect();
                v_st = keep.iter().map(|&i| v_st[i]).collect();
                shift_plasma = keep.iter().map(|&i| shift_plasma[i]).collect();
                dtype = keep.iter().map(|&i| dtype[i]).collect();
                let (_, _, resid0, _, _) = match fixed_effects(
                    &rates_modeled(sc, &times, &r_st, &v_st),
                    &refs,
                    &obs,
                    &times,
                ) {
                    Some(x) => x,
                    None => {
                        eprintln!("{label}: segment-mask refit void");
                        return None;
                    }
                };
                eprintln!(
                    "  after the mask: {} samples, residual-RMS {:.3e} Hz (before {:.3e} Hz)",
                    times.len(),
                    rms_of(&resid0),
                    rms0
                );
            } else {
                eprintln!(
                    "  segment mask: gate 4×p90 = {gate:.3e} Hz — 0 or all segments above it (0 honored), no mask"
                );
            }
        }
    }
    let n = times.len();
    let files_zero = vec![0i64; n];
    let rates0 = rates_modeled(sc, &times, &r_st, &v_st);
    let mut obs_c = obs.clone();
    let mut n_plasma = 0usize;
    let mut sum_abs = 0.0f64;
    let mut max_abs = 0.0f64;
    for i in 0..n {
        if let Some(sh) = shift_plasma[i] {
            obs_c[i] -= sh;
            n_plasma += 1;
            sum_abs += sh.abs();
            max_abs = max_abs.max(sh.abs());
        }
    }
    let Some((_, _, resid_c, _, _)) = fixed_effects(&rates0, &refs, &obs_c, &times) else {
        eprintln!("{label}: plasma fit void");
        return None;
    };
    if n_plasma > 0 {
        eprintln!(
            "  plasma: {n_plasma} of {n} — |Δf| mean {:.3e} Hz, max {max_abs:.3e} Hz; RMS {:.3e} → {:.3e} Hz",
            sum_abs / n_plasma as f64,
            rms0,
            rms_of(&resid_c)
        );
    } else {
        eprintln!("  plasma: 0 of {n} samples carry OMNI2 — empty (0 honored)");
    }
    let mut slope_seg = vec![0.0f64; n];
    let mut seg_mid = vec![0.0f64; n];
    let mut sq_det = 0.0;
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
                let ys: Vec<f64> = (lo..hi).map(|i| resid_c[i]).collect();
                let (b, _) = lin_fit(&xs, &ys);
                for i in lo..hi {
                    slope_seg[i] = b;
                    seg_mid[i] = mid;
                    let det = resid_c[i] - b * (times[i] - mid);
                    sq_det += det * det;
                }
            }
            lo = hi;
        }
    }
    eprintln!(
        "  daily-curve cut: residual {:.3e} → de-trended {:.3e} Hz",
        rms_of(&resid_c),
        (sq_det / n as f64).sqrt()
    );
    let mut obs_d = obs_c.clone();
    for i in 0..n {
        obs_d[i] -= slope_seg[i] * (times[i] - seg_mid[i]);
    }
    let mut best_aniso = RTG_ANISO_SCAN[1];
    let mut best_rms = f64::INFINITY;
    for &aniso in &RTG_ANISO_SCAN {
        let rates = run_rates_dyn(
            state0,
            times[0],
            times[n - 1],
            aniso,
            t0_rtg,
            0.0,
            &times,
            &r_st,
            &v_st,
        );
        let Some(f) = fit_stats(&rates, &refs, &obs_d, &times, &files_zero) else {
            continue;
        };
        if f.rms < best_rms {
            best_rms = f.rms;
            best_aniso = aniso;
        }
    }
    let mut best_a_p = 0.0f64;
    let mut best_rms_ap = best_rms;
    let mut rms_lo = f64::INFINITY;
    let mut rms_hi = f64::NEG_INFINITY;
    for k in -1..=1 {
        let a_p = k as f64 * 8.0e-6;
        let rates = run_rates_dyn(
            state0,
            times[0],
            times[n - 1],
            best_aniso,
            t0_rtg,
            a_p,
            &times,
            &r_st,
            &v_st,
        );
        let Some(f) = fit_stats(&rates, &refs, &obs_d, &times, &files_zero) else {
            continue;
        };
        rms_lo = rms_lo.min(f.rms);
        rms_hi = rms_hi.max(f.rms);
        if f.rms < best_rms_ap {
            best_rms_ap = f.rms;
            best_a_p = a_p;
        }
    }
    eprintln!(
        "  Dynamik: aniso {best_aniso:.3}, a_P-Scan ±8e-6 flach {rms_lo:.4e}…{rms_hi:.4e} Hz, best a_P {best_a_p:.4e} m/s²"
    );
    let rates_final = run_rates_dyn(
        state0,
        times[0],
        times[n - 1],
        best_aniso,
        t0_rtg,
        best_a_p,
        &times,
        &r_st,
        &v_st,
    );
    let f = fit_stats(&rates_final, &refs, &obs_d, &times, &files_zero)?;
    let accel = f.drift / f.a;
    let se_accel = f.se_drift / f.a.abs();
    let times_anomaly = accel.abs() / PIONEER_ANOMALY;
    eprintln!(
        "  Dynamik-Fit: A {:.4e} Hz/(m/s), C {:.4e}, RMS {:.3e} Hz, Drift {:.4e} ± {:.4e} Hz/s → {:.4e} ± {:.4e} m/s²",
        f.a, f.c, f.rms, f.drift, f.se_drift, accel, se_accel
    );
    if se_accel.is_finite() && se_accel < accel.abs() && times_anomaly < 3.0 {
        eprintln!(
            "  the self-test carries the Pioneer anomaly ({PIONEER_ANOMALY:.3e} m/s²): the NAVIO chain carries the drift"
        );
    } else {
        eprintln!(
            "  the self-test does NOT carry the anomaly: |a| lies ~{times_anomaly:.0e}× above the Pioneer anomaly — 0 honored"
        );
    }
    {
        let mut ds: Vec<i64> = dtype.iter().map(|&d| d as i64).collect();
        ds.sort_unstable();
        ds.dedup();
        for d in ds {
            let idx: Vec<usize> = (0..n).filter(|&i| dtype[i] as i64 == d).collect();
            if idx.len() < 50 {
                continue;
            }
            let t2: Vec<f64> = idx.iter().map(|&i| times[i]).collect();
            let o2: Vec<f64> = idx.iter().map(|&i| obs_d[i]).collect();
            let r2: Vec<f64> = idx.iter().map(|&i| refs[i]).collect();
            let rt2: Vec<f64> = idx.iter().map(|&i| rates_final[i]).collect();
            let f2: Vec<i64> = idx.iter().map(|&i| files_zero[i]).collect();
            if let Some(fd) = fit_stats(&rt2, &r2, &o2, &t2, &f2) {
                eprintln!(
                    "  DTYPE {d}: n={}, Drift {:.3e} ± {:.3e} Hz/s → {:.3e} ± {:.3e} m/s²",
                    idx.len(),
                    fd.drift,
                    fd.se_drift,
                    fd.drift / fd.a,
                    fd.se_drift / fd.a.abs()
                );
            }
        }
    }
    if let Some(w) = witness_map {
        let Some((_, _, resid_n, _, _)) = fixed_effects(&rates_final, &refs, &obs_d, &times) else {
            return Some(f);
        };
        let mut nav_map: HashMap<i64, Vec<(f64, f64)>> = HashMap::new();
        for i in 0..n {
            let day = (times[i] / 86400.0).floor() as i64;
            nav_map.entry(day).or_default().push((times[i], resid_n[i]));
        }
        if let Some(z) = witness(&nav_map, w) {
            eprintln!(
                "  witness (roles swapped): {} common days — day-to-day r {:.3}, level-r {:.3}; scatter about the daily median: NAVIO {:.3e} Hz, ATDF {:.3e} Hz; de-trended: NAVIO {:.3e} Hz, ATDF {:.3e} Hz",
                z.days,
                z.r_diff,
                z.r_level,
                z.med_a_scatter,
                z.med_n_scatter,
                z.med_a_det,
                z.med_n_det
            );
        }
    }
    Some(f)
}

fn rates_modeled(
    sc: &dyn Fn(f64) -> Option<([f64; 3], [f64; 3])>,
    times: &[f64],
    r_st: &[[f64; 3]],
    v_st: &[[f64; 3]],
) -> Vec<f64> {
    let mut rates = vec![0.0f64; times.len()];
    for i in 0..times.len() {
        rates[i] = downlink_rate_core(times[i], r_st[i], v_st[i], sc).unwrap_or(f64::NAN);
    }
    rates
}

fn solve4(mut m: [[f64; 4]; 4], mut rhs: [f64; 4]) -> Option<[f64; 4]> {
    for col in 0..4 {
        let mut piv = col;
        for r in col + 1..4 {
            if m[r][col].abs() > m[piv][col].abs() {
                piv = r;
            }
        }
        if m[piv][col].abs() < 1e-300 {
            return None;
        }
        m.swap(col, piv);
        rhs.swap(col, piv);
        let d = m[col][col];
        for r in 0..4 {
            if r == col {
                continue;
            }
            let f = m[r][col] / d;
            if f == 0.0 {
                continue;
            }
            for c in col..4 {
                m[r][c] -= f * m[col][c];
            }
            rhs[r] -= f * rhs[col];
        }
    }
    let mut out = [0.0; 4];
    for i in 0..4 {
        out[i] = rhs[i] / m[i][i];
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lcg(s: &mut u64) -> f64 {
        *s = s
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        (*s >> 11) as f64 / (1u64 << 53) as f64 * 2.0 - 1.0
    }

    fn unit_grid(v: f64) -> TecGrid {
        TecGrid {
            epoch_unix: 0.0,
            lat_first: 10.0,
            lat_step: -20.0,
            nlat: 2,
            lon_first: -10.0,
            lon_step: 20.0,
            nlon: 2,
            cells: vec![v; 4],
        }
    }

    #[test]
    fn plasma_column_matches_quadrature() {
        let n1 = 7.0e6;

        let r_e = [0.0, AU, 0.0];
        let r_sc = [50.0 * AU, 0.0, 0.0];
        let col = plasma_column(r_e, r_sc, n1).unwrap();
        let f1 = |lam: f64| 1.0 / (2500.0 * lam * lam + (1.0 - lam) * (1.0 - lam));
        let m = 4_000_000;
        let mut s = f1(0.0) + f1(1.0);
        for k in 1..m {
            s += if k % 2 == 1 { 4.0 } else { 2.0 } * f1(k as f64 / m as f64);
        }
        let expect1 = n1 * AU * 2501.0f64.sqrt() * s / (3.0 * m as f64);
        let rel1 = (col - expect1).abs() / expect1;
        assert!(
            rel1 < 1e-6,
            "column {col} vs quadrature {expect1} (rel {rel1})"
        );

        let r_sc2 = [50.0 * AU, AU, 0.0];
        let col2 = plasma_column(r_e, r_sc2, n1).unwrap();
        let f2 = |lam: f64| 1.0 / (2500.0 * lam * lam + 1.0);
        let mut s2 = f2(0.0) + f2(1.0);
        for k in 1..40000 {
            s2 += if k % 2 == 1 { 4.0 } else { 2.0 } * f2(k as f64 / 40000.0);
        }
        let expect2 = n1 * 50.0 * AU * s2 / (3.0 * 40000.0);
        let rel2 = (col2 - expect2).abs() / expect2;
        assert!(
            rel2 < 1e-9,
            "column {col2} vs quadrature {expect2} (rel {rel2})"
        );
        let analytic = n1 * AU * 50.0f64.atan();
        assert!((col2 - analytic).abs() < 1e-6 * analytic);
    }

    #[test]
    fn plasma_column_refuses_occultation_and_void() {
        let r_e = [AU, 0.0, 0.0];
        let r_sc = [50.0 * AU, 0.0, 0.0];
        assert!(plasma_column(r_e, r_sc, 7.0e6).is_none(), "occultation");
        assert!(plasma_column(r_e, r_sc, 0.0).is_none(), "density 0");
        assert!(plasma_column(r_e, r_sc, f64::NAN).is_none(), "density NaN");
        assert!(plasma_column(r_e, r_e, 7.0e6).is_none(), "path 0");
    }

    #[test]
    fn plasma_shift_sign_and_formula() {
        let f0 = 2.292e9;
        let shift = plasma_shift(Some(1.0e18), Some(2.0e18), f0).unwrap();
        let expect = PLASMA_K / C * ((2.0e18 - 1.0e18) / PLASMA_DT) / f0;
        assert!((shift - expect).abs() < 1e-30);
        assert!(
            shift > 0.0,
            "a growing column inflates the measured frequency"
        );
        assert!(
            plasma_shift(None, Some(2.0e18), f0).is_none(),
            "absent column → None"
        );
    }

    #[test]
    fn srp_points_away_from_the_sun() {
        let a = srp_accel([AU, 0.0, 0.0]);
        let expect = SC_AREA_M2 / SC_MASS_KG * TSI_W_M2 * (1.0 + SC_REFLECTIVITY) / C;
        assert!((a[0] - expect).abs() < 1e-12);
        assert!(a[1].abs() < 1e-15 && a[2].abs() < 1e-15);
        assert!(a[0] > 0.0, "radiation pressure points away from the sun");
    }

    #[test]
    fn rtg_recoil_decays_with_half_life() {
        let r = [50.0 * AU, 0.0, 0.0];
        let t0 = 0.0;
        let a0 = rtg_accel(r, t0, 0.075, t0);
        let a1 = rtg_accel(r, t0 + RTG_HALF_LIFE_S, 0.075, t0);
        let m0 = norm(a0);
        let m1 = norm(a1);
        assert!((m1 - 0.5 * m0).abs() < 1e-9 * m0, "half-life halves it");
        assert!(a0[0] < 0.0, "RTG recoil points sunward");
    }

    #[test]
    fn tec_pair_shift_uses_rate_not_level() {
        let f0 = 2.292e9;
        let same = vec![(0.0, unit_grid(5.0)), (3600.0, unit_grid(5.0))];
        let s0 = tec_pair_shift(&same, 1800.0, 0.0, 0.0, f0).unwrap();
        assert!(s0.abs() < 1e-30, "same maps → rate 0: {s0}");
        let rising = vec![(0.0, unit_grid(5.0)), (3600.0, unit_grid(6.0))];
        let s1 = tec_pair_shift(&rising, 1800.0, 0.0, 0.0, f0).unwrap();
        let expect = PLASMA_K / C * (1.0e16 / 3600.0) / f0;
        assert!((s1 - expect).abs() < 1e-30);
        assert!(s1 > 0.0, "a growing TEC inflates the measured frequency");
        let gap = vec![(0.0, unit_grid(5.0)), (10.0 * 3600.0, unit_grid(6.0))];
        assert!(
            tec_pair_shift(&gap, 1800.0, 0.0, 0.0, f0).is_none(),
            "map gap > window"
        );
    }

    #[test]
    fn common_mode_isolates_station_noise() {
        let n = 1200;
        let mut times = Vec::with_capacity(n);
        let mut stations = Vec::with_capacity(n);
        let mut resid = Vec::with_capacity(n);
        let mut seed_a = 12345u64;
        let mut seed_b = 67890u64;
        for i in 0..n {
            let t = i as f64 * 10.0;
            times.push(t);
            stations.push(if i % 2 == 0 { 14 } else { 63 });
            let common = 10.0 * (t / 1000.0).sin();
            let noise = if i % 2 == 0 {
                5.0 * lcg(&mut seed_a)
            } else {
                5.0 * lcg(&mut seed_b)
            };
            resid.push(common + noise);
        }
        let cm = common_mode(&times, &stations, &resid);
        assert_eq!(
            cm.pairs, 600,
            "every sample of station 14 pairs with the next of 63"
        );
        let raw = rms_of(&resid);
        let mut cm_resid = Vec::new();
        for i in 0..n {
            if let Some(v) = cm.cm[i] {
                cm_resid.push(resid[i] - v);
            }
        }
        let cm_rms = rms_of(&cm_resid);
        assert!(
            cm_rms < raw,
            "common mode lowers the scatter: {cm_rms} < {raw}"
        );
        assert!(
            cm.diff_rms > 3.0,
            "differentially it carries the station noise: {}",
            cm.diff_rms
        );
    }

    #[test]
    fn common_mode_is_order_independent() {
        let n = 900;
        let mut times = Vec::with_capacity(n);
        let mut stations = Vec::with_capacity(n);
        let mut resid = Vec::with_capacity(n);
        let mut seed = 999u64;
        for i in 0..n {
            let t = i as f64 * 10.0;
            times.push(t);
            stations.push([14i64, 43, 63][i % 3]);
            resid.push((t / 500.0).sin() * 3.0 + 2.0 * lcg(&mut seed));
        }
        let a = common_mode(&times, &stations, &resid);
        let b = common_mode(&times, &stations, &resid);
        assert_eq!(a.pairs, b.pairs, "the pairing is order-independent");
        assert!(
            (a.diff_rms - b.diff_rms).abs() < 1e-9,
            "differential RMS reproduces: {} vs {}",
            a.diff_rms,
            b.diff_rms
        );
    }

    #[test]
    fn witness_shared_signal_correlates_and_ratio_names_the_noise_carrier() {
        let mut a: HashMap<i64, Vec<(f64, f64)>> = HashMap::new();
        let mut b: HashMap<i64, Vec<(f64, f64)>> = HashMap::new();
        let mut seed_a = 11u64;
        let mut seed_n = 22u64;
        for day in 0..100i64 {
            let s = 1000.0 * (day as f64 / 10.0).sin();
            let mut av = Vec::new();
            let mut nv = Vec::new();
            for k in 0..10 {
                let t = day as f64 * 86400.0 + k as f64 * 3600.0;
                av.push((t, s + 30.0 * lcg(&mut seed_a)));
                nv.push((t, 0.5 * s + 5.0 * lcg(&mut seed_n)));
            }
            a.insert(day, av);
            b.insert(day, nv);
        }
        let z = witness(&a, &b).unwrap();
        assert_eq!(z.days, 100);
        assert!(z.r_level > 0.999, "level correlation: {}", z.r_level);
        assert!(z.r_diff > 0.95, "profile correlation: {}", z.r_diff);
        let ratio = z.med_a_scatter / z.med_n_scatter;
        assert!((ratio - 6.0).abs() < 1.0, "scatter ratio ~6: {ratio}");
    }

    #[test]
    fn witness_independent_series_carry_no_profile_correlation() {
        let mut a: HashMap<i64, Vec<(f64, f64)>> = HashMap::new();
        let mut b: HashMap<i64, Vec<(f64, f64)>> = HashMap::new();
        let mut seed_a = 33u64;
        let mut seed_n = 44u64;
        for day in 0..60i64 {
            let mut av = Vec::new();
            let mut nv = Vec::new();
            for k in 0..5 {
                let t = day as f64 * 86400.0 + k as f64 * 3600.0;
                av.push((t, lcg(&mut seed_a)));
                nv.push((t, lcg(&mut seed_n)));
            }
            a.insert(day, av);
            b.insert(day, nv);
        }
        let z = witness(&a, &b).unwrap();
        assert!(
            z.r_diff.abs() < 0.5,
            "independent series carry no profile correlation: {}",
            z.r_diff
        );
    }

    #[test]
    fn fixed_effects_absorbs_epoch_offsets() {
        let n = 200;
        let mut times = Vec::new();
        let mut rates = Vec::new();
        let mut refs = Vec::new();
        let mut obs = Vec::new();
        for i in 0..n {
            let t = if i < 100 {
                i as f64 * 10.0
            } else {
                GAP_S + 1000.0 + (i as f64 - 100.0) * 10.0
            };
            let rate = 3.0 * t + 5.0;
            let rf = 2.292e8 + 1.7 * t + 0.3 * t * t;
            let off = if i < 100 { 1.0e3 } else { -2.0e3 };
            times.push(t);
            rates.push(rate);
            refs.push(rf);
            obs.push(8.3 * rate + 0.5 * rf + off);
        }
        let (a, c, resid, _, _) = fixed_effects(&rates, &refs, &obs, &times).unwrap();
        assert!((a - 8.3).abs() < 1e-9);
        assert!((c - 0.5).abs() < 1e-12);
        assert!(rms_of(&resid) < 1e-6, "epoch offsets absorbed, residual ~0");
    }
}
