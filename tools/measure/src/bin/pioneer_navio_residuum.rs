use std::collections::HashMap;

use omegaflow::archivar::{
    body_barycenter_position, body_barycenter_velocity, body_fixed_to_icrs_smooth,
    parse_ephemeris_binary, BodyEphemeris,
};
use omegaflow::doppler::parse_pnav_bin;
use omegaflow::odp::{downlink_rate_core, dsn_station, station_velocity, C, EARTH};

const SC_BODIES: &[&str] = &["pioneer10_daily", "pioneer11_daily"];
const SC_KEY: [&str; 2] = ["pioneer10", "pioneer11"];
const GAP_PASS_S: f64 = 5.0 * 86400.0;
const GAP_BLOCK_S: f64 = 120.0;
const MIN_BLOCK: usize = 4;
const DAY_S: f64 = 86400.0;
const LS_FLO: f64 = 0.0004;
const LS_FHI: f64 = 0.0015;
const LS_STEP: f64 = 0.00002;
const FLOOR_FLO: f64 = 0.0008;
const FLOOR_FHI: f64 = 0.0012;
const MIN_CELL: usize = 500;
const BEAT_BOUND: f64 = 5.0e5;
const DISPLACED_HZ: f64 = 1.0e5;
const RUCK_GATE_SD: f64 = 4.0;

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
    let jd = tdb / DAY_S + 2451545.0;
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

fn daily_medians(times: &[f64], vals: &[f64]) -> Vec<(f64, f64, f64, usize)> {
    let mut groups: std::collections::BTreeMap<i64, Vec<f64>> = std::collections::BTreeMap::new();
    for (i, &t) in times.iter().enumerate() {
        let day = (t / DAY_S).floor() as i64;
        groups.entry(day).or_default().push(vals[i]);
    }
    let mut out = Vec::new();
    for (day, mut v) in groups {
        v.sort_by(f64::total_cmp);
        let med = v[v.len() / 2];
        let n = v.len();
        let mean = v.iter().sum::<f64>() / n as f64;
        let rms = (v.iter().map(|x| (x - mean) * (x - mean)).sum::<f64>() / n as f64).sqrt();
        out.push((day as f64 * DAY_S, med, rms, n));
    }
    out
}

fn ruck_scan(times: &[f64], vals: &[f64]) {
    if times.len() < 3 {
        eprintln!("  daily series too short for the Ruck scan");
        return;
    }
    let mut ruck: Vec<f64> = Vec::new();
    for i in 1..times.len() - 1 {
        if times[i] - times[i - 1] > 2.0 * DAY_S || times[i + 1] - times[i] > 2.0 * DAY_S {
            ruck.push(f64::NAN);
        } else {
            ruck.push((vals[i + 1] - vals[i - 1]) / (2.0 * DAY_S));
        }
    }
    let mut finite: Vec<f64> = ruck.iter().cloned().filter(|v| v.is_finite()).collect();
    if finite.len() < 20 {
        eprintln!(
            "  Ruck scan too few finite daily steps ({}), stays silent (0 honored)",
            finite.len()
        );
        return;
    }
    finite.sort_by(f64::total_cmp);
    let med = finite[finite.len() / 2];
    let devs: Vec<f64> = finite.iter().map(|x| (x - med).abs()).collect();
    let mut sdevs = devs.clone();
    sdevs.sort_by(f64::total_cmp);
    let mad = sdevs[sdevs.len() / 2];
    let sd = 1.4826 * mad;
    let gate = RUCK_GATE_SD * sd;
    let mut flags: Vec<(f64, f64, f64)> = Vec::new();
    for (k, &r) in ruck.iter().enumerate() {
        if r.is_finite() && (r - med).abs() > gate {
            flags.push((times[k], r, (r - med).abs() / sd.max(1e-300)));
        }
    }
    flags.sort_by(|a, b| a.1.total_cmp(&b.1));
    eprintln!(
        "  Ruck scan: {} daily steps, |Ruck| median {med:.3e} Hz/s, MAD {mad:.3e} (sd {sd:.3e}), gate {gate:.3e} Hz/s ({RUCK_GATE_SD}·sd) — {n} flagged steps",
        ruck.len(),
        n = flags.len()
    );
    if flags.is_empty() {
        eprintln!("  Ruck: no step exceeds the gate — the daily residuum is still over the transit band (0 honored, a limit is a verdict)");
        return;
    }
    for (t, r, sigma) in flags.iter().take(12) {
        eprintln!(
            "  Ruck: {date} |Ruck| {r:.3e} Hz/s ({sigma:.1}·sd) — candidate, held against the local floor",
            date = jd_date(*t)
        );
    }
}

fn run(name: &str, sc_body: &str) {
    let path = format!("data/{name}_navio.bin");
    let Ok(bytes) = std::fs::read(&path) else {
        eprintln!("{name}: pnav bin void ({path})");
        return;
    };
    let Some(records) = parse_pnav_bin(&bytes) else {
        eprintln!("{name}: pnav bin parse void");
        return;
    };
    let mut eph: HashMap<String, BodyEphemeris> = HashMap::new();
    for body in [EARTH, sc_body] {
        let p = format!("data/ephemeris_{body}.bin");
        match std::fs::read(&p)
            .ok()
            .and_then(|d| parse_ephemeris_binary(&d))
        {
            Some(e) => {
                eph.insert(body.to_string(), e);
            }
            None => {
                eprintln!("{name}: ephemeris bin void ({p})");
                return;
            }
        }
    }
    let sc = |t: f64| -> Option<([f64; 3], [f64; 3])> {
        Some((
            body_barycenter_position(sc_body, t, &eph)?,
            body_barycenter_velocity(sc_body, t, &eph)?,
        ))
    };

    let mut times = Vec::new();
    let mut obs = Vec::new();
    let mut rates = Vec::new();
    let mut mode_of = Vec::new();
    let mut rx_of = Vec::new();
    let mut tx_of = Vec::new();
    let mut no_station = 0usize;
    let mut not_two_way = 0usize;
    for r in &records {
        if r[4] as i64 != 12 {
            not_two_way += 1;
            continue;
        }
        let mode = r[8] as i64;
        let rx = r[6] as i64;
        let tx = r[7] as i64;
        if mode != 12 {
            not_two_way += 1;
            continue;
        }
        let Some((rx_lat, rx_lon, rx_alt)) = dsn_station(rx) else {
            no_station += 1;
            continue;
        };
        let t3 = r[0];
        let (Some(r_rx), Some(v_rx)) = (
            body_fixed_to_icrs_smooth(EARTH, rx_lat, rx_lon, rx_alt, t3, &eph),
            station_velocity(t3, rx_lat, rx_lon, rx_alt, &eph),
        ) else {
            continue;
        };
        let Some(rdown) = downlink_rate_core(t3, r_rx, v_rx, &sc) else {
            continue;
        };
        if !rdown.is_finite() {
            continue;
        }
        let mut rate = rdown;
        let Some((tx_lat, tx_lon, tx_alt)) = dsn_station(tx) else {
            continue;
        };
        let Some((t2, r_sc2)) = light_time_sc_pos(t3, r_rx, &sc) else {
            continue;
        };
        let Some((_, v_sc2)) = sc(t2) else {
            continue;
        };
        let Some(rup) = uplink_rate(t2, r_sc2, v_sc2, tx_lat, tx_lon, tx_alt, &eph) else {
            continue;
        };
        if !rup.is_finite() {
            continue;
        }
        rate += rup;
        times.push(t3);
        obs.push(r[1]);
        rates.push(rate);
        mode_of.push(mode as f64);
        rx_of.push(rx as f64);
        tx_of.push(tx as f64);
    }
    eprintln!(
        "{name}: {} records (DTYPE-12 two-way gate; {not_two_way} non-two-way excluded, {no_station} without station)",
        records.len()
    );
    if times.len() < 100 {
        eprintln!("{name}: too short — the series stays unwritten (0 honored)");
        return;
    }

    let mut rx_unique: Vec<i64> = rx_of.iter().map(|&x| x as i64).collect();
    rx_unique.sort_unstable();
    rx_unique.dedup();

    let mut resid_all: Vec<f64> = Vec::new();
    let mut resid_times: Vec<f64> = Vec::new();
    let mut resid_obs: Vec<f64> = Vec::new();
    let mut resid_rx: Vec<f64> = Vec::new();
    let mut resid_mode: Vec<f64> = Vec::new();
    for &st in &rx_unique {
        let mut st_times = Vec::new();
        let mut st_obs = Vec::new();
        let mut st_rates = Vec::new();
        for i in 0..times.len() {
            if rx_of[i] as i64 == st {
                st_times.push(times[i]);
                st_obs.push(obs[i]);
                st_rates.push(rates[i]);
            }
        }
        if st_times.len() < MIN_CELL {
            continue;
        }
        let Some((a0, resid0, _, _)) = fixed_effects_1(&st_rates, &st_obs, &st_times) else {
            eprintln!("{name}: station {st} basis fit void");
            continue;
        };
        let mut active = vec![true; st_times.len()];
        let mut displaced = 0usize;
        for (k, r) in resid0.iter().enumerate() {
            if r.abs() > DISPLACED_HZ {
                active[k] = false;
                displaced += 1;
            }
        }
        let mut f_rates = Vec::new();
        let mut f_obs = Vec::new();
        let mut f_times = Vec::new();
        for k in 0..st_times.len() {
            if active[k] {
                f_rates.push(st_rates[k]);
                f_obs.push(st_obs[k]);
                f_times.push(st_times[k]);
            }
        }
        if f_times.len() < MIN_CELL {
            eprintln!("{name}: station {st} too few clean samples after displaced-count mask ({displaced} discarded)");
            continue;
        }
        let Some((a, resid, _, _)) = fixed_effects_1(&f_rates, &f_obs, &f_times) else {
            eprintln!("{name}: station {st} refit void");
            continue;
        };
        let rms = (resid.iter().map(|v| v * v).sum::<f64>() / resid.len() as f64).sqrt();
        eprintln!(
            "{name}: station {st} — {} two-way samples ({displaced} displaced-count discarded, A {a0:.4e}→{a:.4e} Hz/(m/s)), residual RMS {rms:.3e} Hz",
            f_times.len()
        );
        for k in 0..f_times.len() {
            resid_all.push(resid[k]);
            resid_times.push(f_times[k]);
            resid_obs.push(f_obs[k]);
            resid_rx.push(st as f64);
            resid_mode.push(12.0);
        }
    }
    if resid_all.len() < MIN_CELL {
        eprintln!("{name}: too few modeled residuals — stays silent (0 honored)");
        return;
    }

    let p11 = sc_body.contains("11");
    let mut out_all = Vec::new();
    for i in 0..resid_all.len() {
        out_all.push([
            resid_times[i],
            resid_all[i],
            resid_obs[i],
            0.0,
            resid_all[i],
            resid_rx[i],
            resid_rx[i],
            resid_mode[i],
            0.0,
        ]);
    }
    out_all.sort_by(|a, b| a[0].total_cmp(&b[0]));

    let resid_bin = if p11 {
        "data/pioneer11_navio_residuum.bin"
    } else {
        "data/pioneer10_navio_residuum.bin"
    };
    let bin = omegaflow::odf::write_p11r_bin(&out_all);
    if std::fs::write(resid_bin, &bin).is_err() {
        eprintln!("{name}: write {resid_bin} void");
        return;
    }
    match omegaflow::odf::parse_p11r_bin(&bin) {
        Some(parsed) if parsed.len() == out_all.len() => {
            eprintln!(
                "{name}: {resid_bin} — {} two-way residual samples, {}..{}, {:.0} B — roundtrip parses",
                parsed.len(),
                jd_date(parsed[0][0]),
                jd_date(parsed[parsed.len() - 1][0]),
                bin.len()
            );
        }
        _ => eprintln!("{name}: {resid_bin} roundtrip parse void — series stays unverified"),
    }

    let mut resid_ts_sorted: Vec<(f64, f64)> = resid_times
        .iter()
        .copied()
        .zip(resid_all.iter().copied())
        .collect();
    resid_ts_sorted.sort_by(|a, b| a.0.total_cmp(&b.0));
    let rts: Vec<f64> = resid_ts_sorted.iter().map(|p| p.0).collect();
    let rvs: Vec<f64> = resid_ts_sorted.iter().map(|p| p.1).collect();

    let med = daily_medians(&rts, &rvs);
    let daily_bin = if p11 {
        "data/pioneer11_navio_daily.bin"
    } else {
        "data/pioneer10_navio_daily.bin"
    };
    let mut daily_out: Vec<[f64; 4]> = Vec::new();
    for (t, m, rms, n) in &med {
        daily_out.push([*t, *m, *rms, *n as f64]);
    }
    let mut db = Vec::with_capacity(8 + daily_out.len() * 32);
    db.extend_from_slice(b"PNDM");
    db.extend_from_slice(&(daily_out.len() as u32).to_le_bytes());
    for r in &daily_out {
        for v in r {
            db.extend_from_slice(&v.to_le_bytes());
        }
    }
    if std::fs::write(daily_bin, &db).is_err() {
        eprintln!("{name}: write {daily_bin} void");
        return;
    }
    eprintln!(
        "{name}: {daily_bin} — {} daily-median residua (median never travels without its scatter)",
        daily_out.len()
    );

    let (dts, dvs) = detrend_blocks(&rts, &rvs, GAP_BLOCK_S, MIN_BLOCK);
    let (fp, pp) = ls_peak(&dts, &dvs, LS_FLO, LS_FHI, LS_STEP);
    let mut floor: Vec<f64> = Vec::new();
    let mut f = FLOOR_FLO;
    while f <= FLOOR_FHI {
        let (_, p) = ls_peak(&dts, &dvs, f, f, 1e-12);
        floor.push(p);
        f += LS_STEP;
    }
    floor.sort_by(f64::total_cmp);
    let fmed = floor[floor.len() / 2];
    let amp = if dts.len() > 0 {
        (2.0 * pp / dts.len() as f64).sqrt()
    } else {
        0.0
    };
    eprintln!(
        "{name}: de-trended daily residuum (n={}) — LS peak {fp:.5} Hz (A {amp:.3e} Hz), {ratio:.1}× own local floor; beat gate |resid| ≤ {BEAT_BOUND:.0e} Hz",
        dts.len(),
        ratio = if fmed > 0.0 { pp / fmed } else { f64::NAN }
    );

    let med_t: Vec<f64> = med.iter().map(|m| m.0).collect();
    let med_v: Vec<f64> = med.iter().map(|m| m.1).collect();
    ruck_scan(&med_t, &med_v);
}

fn main() {
    for (i, key) in SC_KEY.iter().enumerate() {
        run(key, SC_BODIES[i]);
    }
}
