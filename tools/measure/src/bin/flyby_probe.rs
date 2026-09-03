use omegaflow::archivar::{
    BodyEphemeris, NAIF_LSK_EMBEDDED, SourceConfig, body_barycenter_position,
    body_barycenter_velocity, download_ephemeris_batch, fetch_raw_bytes, load_sources,
    parse_ephemeris_binary,
};
use omegaflow::cdn::CDN_BASE;
use omegaflow::lsk::{days_from_civil, parse as parse_lsk};
use omegaflow::te::{surrogate_stats_phase, transfer_entropy_lag};
use std::collections::HashMap;
use std::process::Command;

const MODEL_BODIES: [&str; 9] = [
    "mercury", "venus", "earth", "moon", "mars", "jupiter", "saturn", "uranus", "neptune",
];
const GM_SUN: f64 = omegaflow::kepler::GM_SUN_M3_S2;
const J2000_EPOCH: f64 = omegaflow::archivar::J2000_EPOCH;
const SEED: u64 = 0x9E37_79B9_7F4A_7C15;
const HOUR: f64 = 3600.0;
const DAY: f64 = 86400.0;
const TE_WINDOW_D: f64 = 5.0;
const HAPI_MARGIN_D: f64 = 1.0;
const TUBE_H: f64 = 12.0;
const MODEL_DT_S: f64 = 4.0;
const LAG_MAX: usize = 6;

const HAPI_BASE: &str = "https://cdaweb.gsfc.nasa.gov/hapi";
const HAPI_DATASET: &str = "OMNI2_H0_MRG1HR";
const HAPI_PARAMS: &str = "BX_GSE1800,BY_GSM1800,BZ_GSM1800,T1800,N1800,V1800,Pressure1800,KP1800";
const FILL_B: f64 = 999.9;
const FILL_T: f64 = 9_999_999.0;
const FILL_V: f64 = 9999.0;
const FILL_P: f64 = 99.99;
const FILL_KP: f64 = 99.0;

const FLYBYS: &[(&str, i64, i64, i64)] = &[
    ("galileo_e1", 1990, 12, 8),
    ("galileo_e2", 1992, 12, 8),
    ("near", 1998, 1, 23),
    ("cassini", 1999, 8, 18),
    ("rosetta", 2005, 3, 4),
    ("messenger", 2005, 8, 2),
    ("juno", 2013, 10, 9),
];

struct Planet {
    name: String,
    gm: f64,
    j2: f64,
    radius_m: f64,
}

fn julian_day_utc(y: i64, m: i64, d: i64) -> f64 {
    let a = (14 - m) / 12;
    let yy = y + 4800 - a;
    let mm = m + 12 * a - 3;
    let jdn = d + (153 * mm + 2) / 5 + 365 * yy + yy / 4 - yy / 100 + yy / 400 - 32045;
    jdn as f64 - 0.5
}

fn civil_from_days(z: i64) -> (i64, i64, i64) {
    let z = z + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}

fn date_of(days: i64) -> String {
    let (y, m, d) = civil_from_days(days);
    format!("{y}-{m:02}-{d:02}")
}

fn iso_utc(unix: f64) -> String {
    let total = (unix.max(0.0) / DAY).floor() as i64;
    let day_secs = unix.max(0.0) - total as f64 * DAY;
    let z = total + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    let hh = (day_secs / HOUR) as i64;
    let mm = ((day_secs - hh as f64 * HOUR) / 60.0) as i64;
    let ss = (day_secs - hh as f64 * HOUR - mm as f64 * 60.0) as i64;
    format!("{y:04}-{m:02}-{d:02}T{hh:02}:{mm:02}:{ss:02}Z")
}

fn parse_iso(s: &str) -> Option<f64> {
    let b = s.as_bytes();
    if b.len() < 19 {
        return None;
    }
    let year: i64 = s.get(0..4)?.parse().ok()?;
    let month: i64 = s.get(5..7)?.parse().ok()?;
    let day: i64 = s.get(8..10)?.parse().ok()?;
    let h: i64 = s.get(11..13)?.parse().ok()?;
    let mi: i64 = s.get(14..16)?.parse().ok()?;
    let sec: i64 = s.get(17..19)?.parse().ok()?;
    let days = days_from_civil(year, month, day)?;
    Some(days as f64 * DAY + h as f64 * HOUR + mi as f64 * 60.0 + sec as f64)
}

fn fetch(url: &str) -> Option<String> {
    let out = Command::new("curl")
        .arg("-sSf")
        .arg("--retry")
        .arg("2")
        .arg("--max-time")
        .arg("180")
        .arg(url)
        .output()
        .ok()?;
    if out.status.success() {
        String::from_utf8(out.stdout).ok()
    } else {
        eprintln!(
            "fetch http {}: {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        None
    }
}

fn keep_b(v: f64) -> bool {
    v.is_finite() && v != FILL_B && v.abs() <= 1000.0
}

fn keep_positive(v: f64, fill: f64, range: f64) -> bool {
    v.is_finite() && v != fill && v > 0.0 && v <= range
}

fn keep_kp(v: f64) -> bool {
    v.is_finite() && v != FILL_KP && v >= 0.0 && v <= 90.0
}

fn acc(
    r: [f64; 3],
    tdb: f64,
    planets: &[Planet],
    eph: &HashMap<String, BodyEphemeris>,
) -> Option<[f64; 3]> {
    let r2 = r[0] * r[0] + r[1] * r[1] + r[2] * r[2];
    let r3 = r2 * r2.sqrt();
    let mut a = [
        -GM_SUN * r[0] / r3,
        -GM_SUN * r[1] / r3,
        -GM_SUN * r[2] / r3,
    ];
    for p in planets {
        let q = body_barycenter_position(&p.name, tdb, eph)?;
        let dx = q[0] - r[0];
        let dy = q[1] - r[1];
        let dz = q[2] - r[2];
        let d2 = dx * dx + dy * dy + dz * dz;
        let d3 = d2 * d2.sqrt();
        if d3 <= 0.0 {
            continue;
        }
        a[0] += p.gm * dx / d3;
        a[1] += p.gm * dy / d3;
        a[2] += p.gm * dz / d3;
        if p.j2 > 0.0 && p.radius_m > 0.0 {
            let k = 1.5 * p.gm * p.j2 * p.radius_m * p.radius_m / (d2 * d3);
            let zr = dz * dz / d2;
            let w = 5.0 * zr - 1.0;
            a[0] += k * w * dx;
            a[1] += k * w * dy;
            a[2] += k * (w - 2.0) * dz;
        }
    }
    Some(a)
}

fn coverage(eph: &BodyEphemeris) -> (f64, f64) {
    let mut lo = f64::INFINITY;
    let mut hi = f64::NEG_INFINITY;
    for g in &eph.granules {
        lo = lo.min(g.t0_jd - g.dt_jd);
        hi = hi.max(g.t0_jd + g.dt_jd);
    }
    (lo, hi)
}

fn load_flyby_arc(name: &str, eph: &mut HashMap<String, BodyEphemeris>) -> bool {
    let path = format!("data/ephemeris_{name}.bin");
    if !std::path::Path::new(&path).exists() {
        std::fs::create_dir_all("data").ok();
        let url = format!("{}/ssd.jpl.nasa.gov/ephemeris_{name}.bin", CDN_BASE);
        match fetch_raw_bytes(&url, 604800) {
            Some(bytes) => {
                if std::fs::write(&path, &bytes).is_err() {
                    eprintln!("{name}: arc bin write void");
                    return false;
                }
            }
            None => {
                eprintln!("{name}: arc bin fetch void (local and CDN)");
                return false;
            }
        }
    }
    match std::fs::read(&path)
        .ok()
        .and_then(|d| parse_ephemeris_binary(&d))
    {
        Some(e) => {
            let (lo, hi) = coverage(&e);
            eprintln!(
                "  {name}: arc span {lo:.2}..{hi:.2} ({} granules)",
                e.granules.len()
            );
            eph.insert(name.to_string(), e);
            true
        }
        None => {
            eprintln!("{name}: arc bin parse void");
            false
        }
    }
}

fn find_perigee(name: &str, jd0: f64, eph: &HashMap<String, BodyEphemeris>) -> Option<(f64, f64)> {
    let arc = eph.get(name)?;
    let earth = eph.get("earth")?;
    let scan_lo = jd0 - 1.5;
    let scan_hi = jd0 + 1.5;
    let (arc_lo, arc_hi) = coverage(arc);
    let (e_lo, e_hi) = coverage(earth);
    if scan_lo < arc_lo.max(e_lo) || scan_hi > arc_hi.min(e_hi) {
        eprintln!("  {name}: perigee scan window outside arc or earth coverage");
        return None;
    }
    let mut best_jd = 0.0f64;
    let mut best_d = f64::INFINITY;
    let step_jd = 5.0 / 1440.0;
    let mut jd = scan_lo;
    while jd <= scan_hi {
        let tdb = (jd - J2000_EPOCH) * DAY;
        let Some(p) = body_barycenter_position(name, tdb, eph) else {
            jd += step_jd;
            continue;
        };
        let Some(q) = body_barycenter_position("earth", tdb, eph) else {
            jd += step_jd;
            continue;
        };
        let dx = p[0] - q[0];
        let dy = p[1] - q[1];
        let dz = p[2] - q[2];
        let d = (dx * dx + dy * dy + dz * dz).sqrt();
        if d < best_d {
            best_d = d;
            best_jd = jd;
        }
        jd += step_jd;
    }
    if best_d.is_finite() {
        Some((best_jd, best_d))
    } else {
        None
    }
}

fn harvest_drivers(
    t_lo: f64,
    t_hi: f64,
    lsk: &omegaflow::lsk::LeapSeconds,
) -> Vec<(f64, f64, u32)> {
    let (Some(u_lo), Some(u_hi)) = (lsk.tdb_to_unix(t_lo), lsk.tdb_to_unix(t_hi)) else {
        eprintln!("  HAPI window: tdb→unix void — the driver stays empty");
        return Vec::new();
    };
    let start_day = ((u_lo / DAY).floor() as i64) - 1;
    let end_day = ((u_hi / DAY).ceil() as i64) + 1;
    let url = format!(
        "{}/data?id={}&time.min={}T00:00:00Z&time.max={}T23:59:59Z&parameters={}&format=csv",
        HAPI_BASE,
        HAPI_DATASET,
        date_of(start_day),
        date_of(end_day),
        HAPI_PARAMS
    );
    let Some(text) = fetch(&url) else {
        eprintln!(
            "  HAPI window {}-{}: fetch void — the driver stays empty",
            date_of(start_day),
            date_of(end_day)
        );
        return Vec::new();
    };
    let mut out: Vec<(f64, f64, u32)> = Vec::new();
    for line in text.lines() {
        if line.is_empty() || !line.as_bytes()[0].is_ascii_digit() {
            continue;
        }
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() != 9 {
            continue;
        }
        let Some(t_unix) = parse_iso(parts[0]) else {
            continue;
        };
        let Some(t_tdb) = lsk.unix_to_tdb(t_unix) else {
            continue;
        };
        let keepers: [(usize, u32, fn(f64) -> bool); 8] = [
            (1usize, 1u32, keep_b),
            (2usize, 2u32, keep_b),
            (3usize, 3u32, keep_b),
            (4usize, 4u32, |v| keep_positive(v, FILL_T, 1.0e8)),
            (5usize, 5u32, |v| keep_positive(v, FILL_B, 1000.0)),
            (6usize, 6u32, |v| keep_positive(v, FILL_V, 5000.0)),
            (7usize, 7u32, |v| keep_positive(v, FILL_P, 1000.0)),
            (8usize, 8u32, keep_kp),
        ];
        for (col, comp, keep) in keepers {
            if let Ok(v) = parts[col].parse::<f64>() {
                if keep(v) {
                    out.push((t_tdb, v, comp));
                }
            }
        }
    }
    out
}

fn hour_cells(rows: &[(f64, f64, u32)], comp: u32, t0: f64, n: usize) -> Vec<Option<f32>> {
    let mut sums = vec![0.0f64; n];
    let mut counts = vec![0u32; n];
    for &(t, v, c) in rows {
        if c != comp {
            continue;
        }
        let idx = ((t - t0) / HOUR).floor();
        if idx < 0.0 || idx >= n as f64 {
            continue;
        }
        let i = idx as usize;
        sums[i] += v;
        counts[i] += 1;
    }
    (0..n)
        .map(|i| {
            if counts[i] > 0 {
                Some((sums[i] / counts[i] as f64) as f32)
            } else {
                None
            }
        })
        .collect()
}

fn kp_cells(rows: &[(f64, f64, u32)], t0: f64, n: usize) -> Vec<Option<f32>> {
    let mut kp: Vec<(usize, f32)> = Vec::new();
    for &(t, v, c) in rows {
        if c != 8 {
            continue;
        }
        let idx = ((t - t0) / HOUR).floor();
        if idx < 0.0 || idx >= n as f64 {
            continue;
        }
        kp.push((idx as usize, (v / 10.0) as f32));
    }
    kp.sort_by(|a, b| a.0.cmp(&b.0));
    kp.dedup_by(|a, b| a.0 == b.0);
    (0..n)
        .map(|i| {
            let mut val: Option<f32> = None;
            for &(idx, v) in &kp {
                if idx <= i {
                    val = Some(v);
                } else {
                    break;
                }
            }
            if val.is_none() {
                val = kp.first().map(|&(_, v)| v);
            }
            val
        })
        .collect()
}

fn pair_cells(a: &[Option<f32>], b: &[Option<f32>]) -> (Vec<f32>, Vec<f32>) {
    let mut xs = Vec::new();
    let mut ys = Vec::new();
    for (ca, cb) in a.iter().zip(b.iter()) {
        if let (Some(x), Some(y)) = (ca, cb) {
            xs.push(*x);
            ys.push(*y);
        }
    }
    (xs, ys)
}

fn surrogate_te_values(to: &[f32], from: &[f32], lag: usize, seed: u64) -> Vec<f64> {
    let mut rng = seed.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut vals = Vec::new();
    for _ in 0..10 {
        let ys = omegaflow::te::phase_randomized_surrogate(from, &mut rng);
        if let Some(te) = transfer_entropy_lag(to, &ys, lag) {
            vals.push(te);
        }
    }
    vals
}

fn fam_round(pairs: &[(&str, &str, &[f32], &[f32])], lags: &[usize]) -> f64 {
    let mut fam = f64::NEG_INFINITY;
    for (_, _, to, from) in pairs {
        for &lag in lags {
            for te in surrogate_te_values(to, from, lag, SEED) {
                if te > fam {
                    fam = te;
                }
            }
        }
    }
    fam
}

fn verdict_row(
    from: &str,
    to: &str,
    to_s: &[f32],
    from_s: &[f32],
    lags: &[usize],
    fam: f64,
) -> (f64, String) {
    let mut best: Option<(usize, f64)> = None;
    for &lag in lags {
        if let Some(te) = transfer_entropy_lag(to_s, from_s, lag) {
            if best.map_or(true, |(_, b)| te > b) {
                best = Some((lag, te));
            }
        }
    }
    let Some((lag, te)) = best else {
        println!("{from:>10} → {to:<10} | no TE (n < 8)");
        return (0.0, "still".to_string());
    };
    let thr = surrogate_stats_phase(to_s, from_s, lag, SEED)
        .map(|(_, _, t)| t)
        .unwrap_or(0.0);
    let word = if te > fam {
        "fam-carrying"
    } else if te > thr {
        "arrow over own threshold"
    } else {
        "still"
    };
    println!(
        "{from:>10} → {to:<10} | lag {lag} h | TE {te:>10.4e} | thr {thr:>10.4e} | fam {fam:.4e} | {word}"
    );
    (te, word.to_string())
}

fn integrate_leg(
    r0: [f64; 3],
    v0: [f64; 3],
    t_start: f64,
    t_stop: f64,
    dt: f64,
    planets: &[Planet],
    eph: &HashMap<String, BodyEphemeris>,
) -> Option<Vec<[f64; 3]>> {
    let n_hours = ((t_stop - t_start) / HOUR).round().abs() as usize;
    if n_hours == 0 {
        return Some(vec![r0]);
    }
    let steps_per_hour = (HOUR / dt).round() as usize;
    let steps = n_hours * steps_per_hour;
    let sign = if t_stop >= t_start { 1.0 } else { -1.0 };
    let dts = dt * sign;
    let mut r = r0;
    let mut v = v0;
    let a0 = acc(r, t_start, planets, eph)?;
    v = [
        v[0] + 0.5 * dts * a0[0],
        v[1] + 0.5 * dts * a0[1],
        v[2] + 0.5 * dts * a0[2],
    ];
    let mut out = vec![[0.0f64; 3]; n_hours + 1];
    out[0] = r0;
    for k in 1..=steps {
        r = [r[0] + dts * v[0], r[1] + dts * v[1], r[2] + dts * v[2]];
        let a = acc(r, t_start + k as f64 * dts, planets, eph)?;
        v = [v[0] + dts * a[0], v[1] + dts * a[1], v[2] + dts * a[2]];
        if k % steps_per_hour == 0 {
            out[k / steps_per_hour] = r;
        }
    }
    Some(out)
}

fn model_selfcheck(
    r0: [f64; 3],
    v0: [f64; 3],
    t_p: f64,
    t_grid0: f64,
    t_end: f64,
    planets: &[Planet],
    eph: &HashMap<String, BodyEphemeris>,
) -> f64 {
    let build = |dt: f64| -> Option<Vec<[f64; 3]>> {
        let back = integrate_leg(r0, v0, t_p, t_grid0, dt, planets, eph)?;
        let fwd = integrate_leg(r0, v0, t_p, t_end, dt, planets, eph)?;
        let n_back = back.len();
        let mut out = vec![[0.0f64; 3]; n_back + fwd.len() - 1];
        for i in 0..n_back {
            out[i] = back[n_back - 1 - i];
        }
        for j in 1..fwd.len() {
            out[n_back - 1 + j] = fwd[j];
        }
        Some(out)
    };
    let Some(fine) = build(MODEL_DT_S) else {
        return f64::NAN;
    };
    let Some(coarse) = build(3.0 * MODEL_DT_S) else {
        return f64::NAN;
    };
    let mut max_d = 0.0f64;
    for i in 0..fine.len() {
        let dx = fine[i][0] - coarse[i][0];
        let dy = fine[i][1] - coarse[i][1];
        let dz = fine[i][2] - coarse[i][2];
        let d = (dx * dx + dy * dy + dz * dz).sqrt();
        if d > max_d {
            max_d = d;
        }
    }
    max_d
}

fn run_flyby(
    name: &str,
    y: i64,
    m: i64,
    d: i64,
    planets: &[Planet],
    eph: &HashMap<String, BodyEphemeris>,
    lsk: &omegaflow::lsk::LeapSeconds,
) {
    println!("=== {name} (flyby {y}-{m}-{d}) ===");
    let jd0 = julian_day_utc(y, m, d);
    let Some((t_p_jd, perigee_m)) = find_perigee(name, jd0, eph) else {
        println!("  perigee scan void — the flyby stays unmeasured");
        return;
    };
    let t_p = (t_p_jd - J2000_EPOCH) * DAY;
    let perigee_unix = lsk.tdb_to_unix(t_p).unwrap_or(t_p);
    println!(
        "  perigee {} geocentric {:.0} km",
        iso_utc(perigee_unix),
        perigee_m / 1000.0
    );
    let t_lo = t_p - TE_WINDOW_D * DAY;
    let t_hi = t_p + TE_WINDOW_D * DAY;
    let t0 = (t_lo / HOUR).floor() * HOUR;
    let n_hours = ((t_hi - t0) / HOUR).round() as usize + 1;
    let i_p = ((t_p - t0) / HOUR).round() as usize;
    let t_grid0 = t_p - i_p as f64 * HOUR;

    let r0 = match body_barycenter_position(name, t_p, eph) {
        Some(p) => p,
        None => {
            println!("  arc position at perigee void");
            return;
        }
    };
    let v0 = match body_barycenter_velocity(name, t_p, eph) {
        Some(v) => v,
        None => {
            println!("  arc velocity at perigee void");
            return;
        }
    };
    let back = match integrate_leg(r0, v0, t_p, t_grid0, MODEL_DT_S, planets, eph) {
        Some(l) => l,
        None => {
            println!("  model backward leg void");
            return;
        }
    };
    let fwd = match integrate_leg(
        r0,
        v0,
        t_p,
        t_grid0 + (n_hours - 1) as f64 * HOUR,
        MODEL_DT_S,
        planets,
        eph,
    ) {
        Some(l) => l,
        None => {
            println!("  model forward leg void");
            return;
        }
    };
    if back.len() != i_p + 1 || fwd.len() != n_hours - i_p {
        println!(
            "  model legs {}/{} vs grid {i_p}/{n_hours} — the grid stays unmeasured",
            back.len(),
            fwd.len()
        );
        return;
    }
    let mut r_cells = vec![None; n_hours];
    let mut r_max_tube = 0.0f64;
    for h in 0..n_hours {
        let t_i = t_grid0 + h as f64 * HOUR;
        let Some(p_arc) = body_barycenter_position(name, t_i, eph) else {
            println!("  arc position void at hour {h}");
            return;
        };
        let m_i = if h <= i_p {
            back[i_p - h]
        } else {
            fwd[h - i_p]
        };
        let dx = p_arc[0] - m_i[0];
        let dy = p_arc[1] - m_i[1];
        let dz = p_arc[2] - m_i[2];
        let rr = (dx * dx + dy * dy + dz * dz).sqrt();
        r_cells[h] = Some(rr as f32);
        if (t_i - t_p).abs() <= TUBE_H * HOUR && rr > r_max_tube {
            r_max_tube = rr;
        }
    }
    let selfcheck = model_selfcheck(
        r0,
        v0,
        t_p,
        t_grid0,
        t_grid0 + (n_hours - 1) as f64 * HOUR,
        planets,
        eph,
    );
    println!(
        "  tube ±{TUBE_H:.0} h: max R {:.1} m | model free-run ({MODEL_DT_S:.0} vs {:.0} s) max {:.1} m",
        r_max_tube,
        3.0 * MODEL_DT_S,
        selfcheck
    );
    let tube_r = |off: f64| -> String {
        let i = i_p as i64 + off as i64;
        if i < 0 || i >= n_hours as i64 {
            return "void".to_string();
        }
        match r_cells[i as usize] {
            Some(r) => format!("{r:.0} m"),
            None => "void".to_string(),
        }
    };
    println!(
        "  R-profil: −12 h {} | −6 h {} | −1 h {} | 0 h {} | +1 h {} | +6 h {} | +12 h {}",
        tube_r(-12.0),
        tube_r(-6.0),
        tube_r(-1.0),
        tube_r(0.0),
        tube_r(1.0),
        tube_r(6.0),
        tube_r(12.0)
    );

    let mut j_cells = vec![None; n_hours];
    for h in 1..n_hours - 1 {
        match (r_cells[h - 1], r_cells[h + 1]) {
            (Some(a), Some(b)) => j_cells[h] = Some(b - 2.0 * r_cells[h].unwrap_or(0.0) + a),
            _ => {}
        }
    }

    let rows = harvest_drivers(t_lo - HAPI_MARGIN_D * DAY, t_hi + HAPI_MARGIN_D * DAY, lsk);
    let bz = hour_cells(&rows, 3, t_grid0, n_hours);
    let pressure = hour_cells(&rows, 7, t_grid0, n_hours);
    let kp = kp_cells(&rows, t_grid0, n_hours);
    let mut pgrad = vec![None; n_hours];
    for h in 1..n_hours - 1 {
        match (pressure[h - 1], pressure[h + 1]) {
            (Some(a), Some(b)) => pgrad[h] = Some(0.5 * (b - a)),
            _ => {}
        }
    }
    let (j_pg, pg_j) = pair_cells(&j_cells, &pgrad);
    let (j_bz, bz_j) = pair_cells(&j_cells, &bz);
    let (j_kp, kp_j) = pair_cells(&j_cells, &kp);
    let (r_pg, pg_r) = pair_cells(&r_cells, &pgrad);
    let (r_bz, bz_r) = pair_cells(&r_cells, &bz);
    let (r_kp, kp_r) = pair_cells(&r_cells, &kp);
    println!(
        "  window ±{TE_WINDOW_D:.0} d: {n_hours} hours | paired: jerk {} | R {}",
        j_bz.len(),
        r_bz.len()
    );

    let lags: Vec<usize> = (0..=LAG_MAX).collect();
    let pairs: [(&str, &str, &[f32], &[f32]); 12] = [
        ("P-grad", "jerk", &j_pg, &pg_j),
        ("jerk", "P-grad", &pg_j, &j_pg),
        ("Bz", "jerk", &j_bz, &bz_j),
        ("jerk", "Bz", &bz_j, &j_bz),
        ("Kp", "jerk", &j_kp, &kp_j),
        ("jerk", "Kp", &kp_j, &j_kp),
        ("P-grad", "R", &r_pg, &pg_r),
        ("R", "P-grad", &pg_r, &r_pg),
        ("Bz", "R", &r_bz, &bz_r),
        ("R", "Bz", &bz_r, &r_bz),
        ("Kp", "R", &r_kp, &kp_r),
        ("R", "Kp", &kp_r, &r_kp),
    ];
    let fam = fam_round(&pairs, &lags);
    println!("  fam = {fam:.4e}");
    let mut strongest = 0.0f64;
    let mut strongest_word = "still".to_string();
    let mut strongest_label = String::new();
    for (idx, (from, to, to_s, from_s)) in pairs.iter().enumerate() {
        let (te, word) = verdict_row(from, to, to_s, from_s, &lags, fam);
        if idx % 2 == 0 && te > strongest {
            strongest = te;
            strongest_word = word;
            strongest_label = format!("{from} → {to}");
        }
    }
    let verdict = if strongest_word == "fam-carrying" {
        format!("Pfeil aus dem Plasmadruck ({strongest_label})")
    } else if strongest_word == "arrow over own threshold" {
        format!("arrow over own threshold, unter fam ({strongest_label})")
    } else {
        "silence".to_string()
    };
    println!("  verdict: {verdict} (0 honored — silence is a finding)");
    println!();
}

fn main() {
    std::fs::create_dir_all("data").ok();
    let args: Vec<String> = std::env::args().skip(1).collect();
    let only = args
        .iter()
        .position(|a| a == "--flyby")
        .and_then(|i| args.get(i + 1))
        .cloned();

    let lsk = match parse_lsk(NAIF_LSK_EMBEDDED) {
        Some(l) => l,
        None => {
            eprintln!("embedded leap table parses void");
            return;
        }
    };

    let sources = load_sources();
    let items: Vec<(usize, SourceConfig, String)> = sources
        .iter()
        .enumerate()
        .filter(|(_, s)| {
            s.format == "ephemeris_binary"
                && MODEL_BODIES.contains(&s.body.as_deref().unwrap_or(""))
        })
        .map(|(idx, s)| {
            (
                idx,
                s.clone(),
                format!("data/ephemeris_{}.bin", s.body.as_deref().unwrap_or("")),
            )
        })
        .collect();
    download_ephemeris_batch(&items);
    let mut eph: HashMap<String, BodyEphemeris> = HashMap::new();
    let mut planets: Vec<Planet> = Vec::new();
    for (_, s, path) in &items {
        let name = s.body.clone().unwrap_or_default();
        match std::fs::read(path)
            .ok()
            .and_then(|d| parse_ephemeris_binary(&d))
        {
            Some(e) => {
                let gm = e.props.as_ref().and_then(|p| p.gm);
                let j2 = e.props.as_ref().and_then(|p| p.j2);
                let radius_m = e.props.as_ref().map(|p| p.radius_m);
                eph.insert(name.clone(), e);
                match (gm, j2, radius_m) {
                    (Some(gm), Some(j2), Some(radius_m))
                        if gm.is_finite() && gm > 0.0 && j2.is_finite() && radius_m > 0.0 =>
                    {
                        planets.push(Planet {
                            name,
                            gm,
                            j2,
                            radius_m,
                        });
                    }
                    (Some(gm), _, _) if gm.is_finite() && gm > 0.0 => {
                        planets.push(Planet {
                            name,
                            gm,
                            j2: 0.0,
                            radius_m: 0.0,
                        });
                    }
                    _ => eprintln!("{name}: gm absent — the body does not carry the mass"),
                }
            }
            None => eprintln!("{name}: bin parse void"),
        }
    }
    if planets.len() < MODEL_BODIES.len() {
        eprintln!(
            "model incomplete: {} of {} bodies",
            planets.len(),
            MODEL_BODIES.len()
        );
    }
    let earth_j2 = planets
        .iter()
        .find(|p| p.name == "earth")
        .map(|p| p.j2)
        .unwrap_or(0.0);
    let moon_in = planets.iter().any(|p| p.name == "moon");
    println!(
        "model: Sun+8+Moon point masses (moon {moon_in}) + Earth-J2 {earth_j2} (from the ephemeris-bins), Leapfrog {MODEL_DT_S:.0} s"
    );

    for (name, y, m, d) in FLYBYS {
        if let Some(only) = &only {
            if only != name {
                continue;
            }
        }
        if !load_flyby_arc(name, &mut eph) {
            continue;
        }
        run_flyby(name, *y, *m, *d, &planets, &eph, &lsk);
    }
}
