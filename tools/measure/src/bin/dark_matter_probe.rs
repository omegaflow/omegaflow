use omegaflow::archivar::{
    BodyEphemeris, J2000_EPOCH, SourceConfig, body_barycenter_position, download_ephemeris_batch,
    fetch_raw_bytes, load_sources, parse_ephemeris_binary,
};
use omegaflow::cdn::CDN_BASE;
use omegaflow::te::{phase_randomized_surrogate, transfer_entropy_lag};
use std::collections::HashMap;

const MODEL_BODIES: [&str; 9] = [
    "mercury", "venus", "earth", "moon", "mars", "jupiter", "saturn", "uranus", "neptune",
];
const PROBES: [&str; 5] = [
    "pioneer10",
    "pioneer11",
    "voyager1",
    "voyager2",
    "new_horizons",
];
const GM_SUN: f64 = omegaflow::kepler::GM_SUN_M3_S2;
const AU_M: f64 = omegaflow::kepler::AU_M;
const DAY: f64 = 86400.0;
const JD_UNIX_EPOCH: f64 = 2440587.5;
const WINDOW_DAYS: i64 = 180;
const LAG_MAX: usize = 2;
const SEED: u64 = 0x9E37_79B9_7F4A_7C15;

const POSITIVE: &[(&str, &str, i64, i64, i64)] = &[
    ("voyager1", "jupiter", 1979, 3, 5),
    ("voyager1", "saturn", 1980, 11, 12),
    ("voyager2", "jupiter", 1979, 7, 9),
    ("voyager2", "saturn", 1981, 8, 26),
    ("voyager2", "uranus", 1986, 1, 24),
    ("voyager2", "neptune", 1989, 8, 25),
];

const NULLS: &[(&str, [f64; 3])] = &[
    ("leer A", [-100.0 * AU_M, -60.0 * AU_M, 0.0]),
    ("leer B", [40.0 * AU_M, 120.0 * AU_M, 0.0]),
    ("leer C", [150.0 * AU_M, 150.0 * AU_M, 80.0 * AU_M]),
    ("leer D", [-250.0 * AU_M, 100.0 * AU_M, -50.0 * AU_M]),
];

struct Planet {
    name: String,
    gm: f64,
    j2: f64,
    radius_m: f64,
}

struct Residue {
    days: Vec<i64>,
    pos: Vec<[f64; 3]>,
    a_res: Vec<[f64; 3]>,
    r: Vec<f32>,
    ruck: Vec<f32>,
}

struct Arrow {
    label: String,
    probe: String,
    te: f64,
    thr: f64,
    desc: String,
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

fn acc(
    r: [f64; 3],
    tdb: f64,
    planets: &[Planet],
    eph: &HashMap<String, BodyEphemeris>,
    exclude: Option<&str>,
) -> Option<[f64; 3]> {
    let r2 = r[0] * r[0] + r[1] * r[1] + r[2] * r[2];
    let r3 = r2 * r2.sqrt();
    let mut a = [
        -GM_SUN * r[0] / r3,
        -GM_SUN * r[1] / r3,
        -GM_SUN * r[2] / r3,
    ];
    for p in planets {
        if exclude == Some(p.name.as_str()) {
            continue;
        }
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

fn load_probe_daily(name: &str, eph: &mut HashMap<String, BodyEphemeris>) -> bool {
    let key = format!("{name}_daily");
    let path = format!("data/ephemeris_{key}.bin");
    if !std::path::Path::new(&path).exists() {
        std::fs::create_dir_all("data").ok();
        let url = format!("{}/ssd.jpl.nasa.gov/ephemeris_{key}.bin", CDN_BASE);
        match fetch_raw_bytes(&url, 604800) {
            Some(bytes) => {
                if std::fs::write(&path, &bytes).is_err() {
                    eprintln!("{name}: daily-bin write void");
                    return false;
                }
            }
            None => {
                eprintln!("{name}: daily-bin fetch void (local and CDN)");
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
                "  {key}: span {lo:.2}..{hi:.2} ({} granules)",
                e.granules.len()
            );
            eph.insert(key, e);
            true
        }
        None => {
            eprintln!("{name}: daily-bin parse void");
            false
        }
    }
}

fn compute_residue(
    name: &str,
    planets: &[Planet],
    eph: &HashMap<String, BodyEphemeris>,
    exclude: Option<&str>,
) -> Option<Residue> {
    let key = format!("{name}_daily");
    let probe = eph.get(&key)?;
    let (lo_jd, hi_jd) = coverage(probe);
    let first = (lo_jd - JD_UNIX_EPOCH).ceil() as i64 + 1;
    let last = (hi_jd - JD_UNIX_EPOCH).floor() as i64 - 1;
    if last - first < 3 {
        return None;
    }
    let mut days: Vec<i64> = Vec::new();
    let mut pos: Vec<[f64; 3]> = Vec::new();
    for d in first..=last {
        let jd = d as f64 + JD_UNIX_EPOCH;
        let tdb = (jd - J2000_EPOCH) * DAY;
        match body_barycenter_position(&key, tdb, eph) {
            Some(r) => {
                days.push(d);
                pos.push(r);
            }
            None => {}
        }
    }
    let mut out = Residue {
        days: Vec::new(),
        pos: Vec::new(),
        a_res: Vec::new(),
        r: Vec::new(),
        ruck: Vec::new(),
    };
    let mut prev_a_res: Option<[f64; 3]> = None;
    for i in 1..pos.len() - 1 {
        if days[i] - days[i - 1] != 1 || days[i + 1] - days[i] != 1 {
            prev_a_res = None;
            continue;
        }
        let tdb = (days[i] as f64 + JD_UNIX_EPOCH - J2000_EPOCH) * DAY;
        let a_obs = [
            (pos[i + 1][0] - 2.0 * pos[i][0] + pos[i - 1][0]) / (DAY * DAY),
            (pos[i + 1][1] - 2.0 * pos[i][1] + pos[i - 1][1]) / (DAY * DAY),
            (pos[i + 1][2] - 2.0 * pos[i][2] + pos[i - 1][2]) / (DAY * DAY),
        ];
        let a_bek = match acc(pos[i], tdb, planets, eph, exclude) {
            Some(a) => a,
            None => {
                prev_a_res = None;
                continue;
            }
        };
        let a_res = [
            a_obs[0] - a_bek[0],
            a_obs[1] - a_bek[1],
            a_obs[2] - a_bek[2],
        ];
        let r_mag = (a_res[0] * a_res[0] + a_res[1] * a_res[1] + a_res[2] * a_res[2]).sqrt();
        if let Some(prev) = prev_a_res {
            let dx = a_res[0] - prev[0];
            let dy = a_res[1] - prev[1];
            let dz = a_res[2] - prev[2];
            let ruck = (dx * dx + dy * dy + dz * dz).sqrt();
            out.days.push(days[i]);
            out.pos.push(pos[i]);
            out.a_res.push(a_res);
            out.r.push(r_mag as f32);
            out.ruck.push(ruck as f32);
        }
        prev_a_res = Some(a_res);
    }
    if out.days.is_empty() { None } else { Some(out) }
}

fn print_summary(name: &str, res: &Residue) {
    let n = res.r.len();
    let mut rs: Vec<f64> = res.r.iter().map(|&v| v as f64).collect();
    rs.sort_by(f64::total_cmp);
    let mean = rs.iter().sum::<f64>() / n as f64;
    let max_r = rs[n - 1];
    let mut max_r_idx = 0usize;
    for i in 1..n {
        if res.r[i] > res.r[max_r_idx] {
            max_r_idx = i;
        }
    }
    let max_r_day = res.days[max_r_idx];
    let max_ruck = res.ruck.iter().cloned().fold(0.0f32, f32::max);
    println!(
        "  {name}: {n} steps (daily grid), {}..{}, |a_res| mean {mean:.4e} m/s2, median {:.4e}, max {max_r:.4e} @ {}, jerk max {max_ruck:.4e} m/s2/d",
        date_of(res.days[0]),
        date_of(res.days[n - 1]),
        rs[n / 2],
        date_of(max_r_day),
    );
}

fn moving_distance(
    pos: &[[f64; 3]],
    days: &[i64],
    planet: &str,
    eph: &HashMap<String, BodyEphemeris>,
) -> Vec<f32> {
    let mut out = Vec::with_capacity(days.len());
    for (&dd, &p) in days.iter().zip(pos) {
        let tdb = (dd as f64 + JD_UNIX_EPOCH - J2000_EPOCH) * DAY;
        match body_barycenter_position(planet, tdb, eph) {
            Some(q) => {
                let dx = q[0] - p[0];
                let dy = q[1] - p[1];
                let dz = q[2] - p[2];
                out.push((dx * dx + dy * dy + dz * dz).sqrt() as f32);
            }
            None => {
                eprintln!("  {planet}: Position void — driver stays empty");
                return Vec::new();
            }
        }
    }
    out
}

fn fixed_distance(pos: &[[f64; 3]], x: &[f64; 3]) -> Vec<f32> {
    pos.iter()
        .map(|p| {
            let dx = p[0] - x[0];
            let dy = p[1] - x[1];
            let dz = p[2] - x[2];
            (dx * dx + dy * dy + dz * dz).sqrt() as f32
        })
        .collect()
}

fn surrogate_te_values(to: &[f32], from: &[f32], lag: usize, seed: u64) -> Vec<f64> {
    let mut rng = seed.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut vals = Vec::new();
    for _ in 0..10 {
        let ys = phase_randomized_surrogate(from, &mut rng);
        if let Some(te) = transfer_entropy_lag(to, &ys, lag) {
            vals.push(te);
        }
    }
    vals
}

fn threshold_of(surrogates: &[f64]) -> f64 {
    if surrogates.len() < 2 {
        return 0.0;
    }
    let n = surrogates.len() as f64;
    let mean = surrogates.iter().sum::<f64>() / n;
    let var = surrogates
        .iter()
        .map(|&v| (v - mean) * (v - mean))
        .sum::<f64>()
        / n;
    mean + 2.0 * var.sqrt()
}

fn arrow_windowed(
    label: &str,
    probe: &str,
    res: &Residue,
    d: &[f32],
    fam: &mut f64,
) -> Option<Arrow> {
    if d.len() != res.days.len() || d.is_empty() {
        return None;
    }
    let mut imin = 0usize;
    for i in 1..d.len() {
        if d[i] < d[imin] {
            imin = i;
        }
    }
    let cday = res.days[imin];
    let idx: Vec<usize> = (0..d.len())
        .filter(|&i| (res.days[i] - cday).abs() <= WINDOW_DAYS)
        .collect();
    if idx.len() < 24 {
        return None;
    }
    let d_w: Vec<f32> = idx.iter().map(|&i| d[i]).collect();
    let r_w: Vec<f32> = idx.iter().map(|&i| res.r[i]).collect();
    let ruck_w: Vec<f32> = idx.iter().map(|&i| res.ruck[i]).collect();
    let mut best_te = f64::NEG_INFINITY;
    let mut best_thr = 0.0f64;
    let mut best_desc = String::new();
    for (tname, target) in [("R", &r_w), ("jerk", &ruck_w)] {
        for lag in 0..=LAG_MAX {
            let Some(te) = transfer_entropy_lag(target, &d_w, lag) else {
                continue;
            };
            let surrogates = surrogate_te_values(target, &d_w, lag, SEED);
            let thr = threshold_of(&surrogates);
            for &s in &surrogates {
                if s > *fam {
                    *fam = s;
                }
            }
            if te > best_te {
                best_te = te;
                best_thr = thr;
                best_desc = format!("d→{tname} @ lag {lag}");
            }
        }
    }
    if best_te.is_finite() {
        Some(Arrow {
            label: label.to_string(),
            probe: probe.to_string(),
            te: best_te,
            thr: best_thr,
            desc: best_desc,
        })
    } else {
        None
    }
}

fn mean_sd(v: &[f32]) -> (f64, f64) {
    let n = v.len() as f64;
    let m = v.iter().map(|&x| x as f64).sum::<f64>() / n;
    let var = v
        .iter()
        .map(|&x| (x as f64 - m) * (x as f64 - m))
        .sum::<f64>()
        / n;
    (m, var.sqrt())
}

fn transform(v: &[f32], mode: u8) -> Vec<f32> {
    match mode {
        1 => v.iter().map(|&x| x.ln() as f32).collect(),
        2 => {
            let (m, s) = mean_sd(v);
            let sd = if s > 0.0 { s } else { 1.0 };
            v.iter().map(|&x| ((x as f64 - m) / sd) as f32).collect()
        }
        _ => v.to_vec(),
    }
}

fn selfcheck(probe: &str, planet: &str, res: &Residue, eph: &HashMap<String, BodyEphemeris>) {
    let drv = moving_distance(&res.pos, &res.days, planet, eph);
    if drv.is_empty() {
        return;
    }
    let synth: Vec<f32> = drv
        .iter()
        .map(|&d| (1.0 / ((d as f64) * (d as f64))) as f32)
        .collect();
    let mut imin = 0usize;
    for i in 1..drv.len() {
        if drv[i] < drv[imin] {
            imin = i;
        }
    }
    let cday = res.days[imin];
    let idx: Vec<usize> = (0..drv.len())
        .filter(|&i| (res.days[i] - cday).abs() <= WINDOW_DAYS)
        .collect();
    let d_w: Vec<f32> = idx.iter().map(|&i| drv[i]).collect();
    let s_w: Vec<f32> = idx.iter().map(|&i| synth[i]).collect();
    for mode in 0..=2u8 {
        let dd = transform(&d_w, mode);
        let ss = transform(&s_w, mode);
        if let Some(te) = transfer_entropy_lag(&ss, &dd, 0) {
            let surr = surrogate_te_values(&ss, &dd, 0, SEED);
            let thr = threshold_of(&surr);
            println!(
                "  selfcheck {probe}/{planet} mode {mode}: TE(d→1/d²) @ lag 0 = {te:.4e} (thr {thr:.4e})"
            );
        }
    }
}

fn log_pair(a: &[f32], b: &[f32]) -> Option<(Vec<f32>, Vec<f32>)> {
    let mut la = Vec::new();
    let mut lb = Vec::new();
    for (&x, &y) in a.iter().zip(b) {
        if x > 0.0 && y > 0.0 {
            la.push(x.ln() as f32);
            lb.push(y.ln() as f32);
        }
    }
    if la.len() < 24 { None } else { Some((la, lb)) }
}

fn load_arc(name: &str, eph: &mut HashMap<String, BodyEphemeris>) -> bool {
    let path = format!("data/ephemeris_{name}.bin");
    if !std::path::Path::new(&path).exists() {
        std::fs::create_dir_all("data").ok();
        let url = format!("{}/ssd.jpl.nasa.gov/ephemeris_{name}.bin", CDN_BASE);
        match fetch_raw_bytes(&url, 604800) {
            Some(bytes) => {
                if std::fs::write(&path, &bytes).is_err() {
                    eprintln!("{name}: arc write void");
                    return false;
                }
            }
            None => {
                eprintln!("{name}: arc fetch void (local and CDN)");
                return false;
            }
        }
    }
    match std::fs::read(&path)
        .ok()
        .and_then(|d| parse_ephemeris_binary(&d))
    {
        Some(e) => {
            eph.insert(name.to_string(), e);
            true
        }
        None => {
            eprintln!("{name}: arc parse void");
            false
        }
    }
}

fn fine_positive(
    probe: &str,
    planet: &str,
    planets: &[Planet],
    eph: &HashMap<String, BodyEphemeris>,
    fam: &mut f64,
) -> Option<Arrow> {
    let arc_name = format!("{probe}_{planet}");
    let arc = eph.get(&arc_name)?;
    let (lo_jd, hi_jd) = coverage(arc);
    let step_jd = 1.0 / 24.0;
    let h = step_jd * DAY;
    let mut t: Vec<f64> = Vec::new();
    let mut pos: Vec<[f64; 3]> = Vec::new();
    let mut jd = lo_jd;
    while jd <= hi_jd {
        let tdb = (jd - J2000_EPOCH) * DAY;
        if let Some(r) = body_barycenter_position(&arc_name, tdb, eph) {
            t.push(tdb);
            pos.push(r);
        }
        jd += step_jd;
    }
    if pos.len() < 3 {
        return None;
    }
    let known_gm = planets
        .iter()
        .find(|p| p.name == planet)
        .map(|p| p.gm)
        .unwrap_or(f64::NAN);
    let mut d: Vec<f32> = Vec::new();
    let mut r: Vec<f32> = Vec::new();
    let mut ruck: Vec<f32> = Vec::new();
    let mut prev: Option<[f64; 3]> = None;
    for i in 1..pos.len() - 1 {
        let a_obs = [
            (pos[i + 1][0] - 2.0 * pos[i][0] + pos[i - 1][0]) / (h * h),
            (pos[i + 1][1] - 2.0 * pos[i][1] + pos[i - 1][1]) / (h * h),
            (pos[i + 1][2] - 2.0 * pos[i][2] + pos[i - 1][2]) / (h * h),
        ];
        let a_bek = acc(pos[i], t[i], planets, eph, Some(planet))?;
        let a_res = [
            a_obs[0] - a_bek[0],
            a_obs[1] - a_bek[1],
            a_obs[2] - a_bek[2],
        ];
        let r_mag = (a_res[0] * a_res[0] + a_res[1] * a_res[1] + a_res[2] * a_res[2]).sqrt();
        let q = body_barycenter_position(planet, t[i], eph)?;
        let dx = q[0] - pos[i][0];
        let dy = q[1] - pos[i][1];
        let dz = q[2] - pos[i][2];
        let dd = (dx * dx + dy * dy + dz * dz).sqrt();
        if let Some(pv) = prev {
            let jx = a_res[0] - pv[0];
            let jy = a_res[1] - pv[1];
            let jz = a_res[2] - pv[2];
            let ruck_mag = (jx * jx + jy * jy + jz * jz).sqrt();
            d.push(dd as f32);
            r.push(r_mag as f32);
            ruck.push(ruck_mag as f32);
        }
        prev = Some(a_res);
    }
    if d.len() < 24 {
        return None;
    }
    let mut imin = 0usize;
    for i in 1..d.len() {
        if d[i] < d[imin] {
            imin = i;
        }
    }
    let mut gm_ests: Vec<f64> = Vec::new();
    for (i, &di) in d.iter().enumerate() {
        if (i as i64 - imin as i64).abs() <= 24 {
            gm_ests.push(r[i] as f64 * (di as f64) * (di as f64));
        }
    }
    gm_ests.sort_by(f64::total_cmp);
    let gm_est = if gm_ests.is_empty() {
        f64::NAN
    } else {
        gm_ests[gm_ests.len() / 2]
    };
    let mut best_te = f64::NEG_INFINITY;
    let mut best_thr = 0.0f64;
    let mut best_desc = String::new();
    for (tname, target) in [("R", &r), ("jerk", &ruck)] {
        if let Some((ld, lt)) = log_pair(&d, target) {
            for lag in 0..=1 {
                let Some(te) = transfer_entropy_lag(&lt, &ld, lag) else {
                    continue;
                };
                let surrogates = surrogate_te_values(&lt, &ld, lag, SEED);
                let thr = threshold_of(&surrogates);
                for &s in &surrogates {
                    if s > *fam {
                        *fam = s;
                    }
                }
                if te > best_te {
                    best_te = te;
                    best_thr = thr;
                    best_desc = format!("d→{tname} @ lag {lag}");
                }
            }
        }
    }
    Some(Arrow {
        label: format!("{planet} [GM {gm_est:.3e} vs {known_gm:.3e}]"),
        probe: probe.to_string(),
        te: best_te,
        thr: best_thr,
        desc: best_desc,
    })
}

fn fibonacci_point(n: usize, j: usize) -> [f64; 3] {
    let phi = std::f64::consts::PI * (3.0 - 5.0_f64.sqrt());
    let y = 1.0 - 2.0 * (j as f64 + 0.5) / n as f64;
    let r = (1.0 - y * y).max(0.0).sqrt();
    let theta = phi * j as f64;
    [r * theta.cos(), y, r * theta.sin()]
}

fn run_grid(residues: &HashMap<String, Residue>) {
    const SHELLS: [f64; 9] = [40.0, 60.0, 80.0, 120.0, 160.0, 200.0, 300.0, 450.0, 600.0];
    const PER_SHELL: usize = 112;
    const WINDOW: i64 = 90;
    let n_points = SHELLS.len() * PER_SHELL;
    let mut points: Vec<(f64, [f64; 3])> = Vec::with_capacity(n_points);
    for &radius_au in &SHELLS {
        for j in 0..PER_SHELL {
            let dir = fibonacci_point(PER_SHELL, j);
            points.push((radius_au, dir));
        }
    }
    let mut flags: Vec<(usize, usize, f64, f64)> = Vec::new();
    for (pi, probe) in PROBES.iter().enumerate() {
        let Some(res) = residues.get(*probe) else {
            continue;
        };
        let mut rcopy: Vec<f64> = res.r.iter().map(|&v| v as f64).collect();
        rcopy.sort_by(f64::total_cmp);
        let median_r = rcopy[rcopy.len() / 2];
        for (idx, &(radius_au, dir)) in points.iter().enumerate() {
            let x = [
                dir[0] * radius_au * AU_M,
                dir[1] * radius_au * AU_M,
                dir[2] * radius_au * AU_M,
            ];
            let mut dmin2 = f64::INFINITY;
            let mut imin = 0usize;
            for i in 0..res.pos.len() {
                let dx = res.pos[i][0] - x[0];
                let dy = res.pos[i][1] - x[1];
                let dz = res.pos[i][2] - x[2];
                let dd = dx * dx + dy * dy + dz * dz;
                if dd < dmin2 {
                    dmin2 = dd;
                    imin = i;
                }
            }
            let cday = res.days[imin];
            let mut gms: Vec<f64> = Vec::new();
            let mut a_rad_max = 0.0f64;
            for i in 0..res.pos.len() {
                if (res.days[i] - cday).abs() > WINDOW {
                    continue;
                }
                if res.r[i] as f64 > 100.0 * median_r {
                    continue;
                }
                let dx = x[0] - res.pos[i][0];
                let dy = x[1] - res.pos[i][1];
                let dz = x[2] - res.pos[i][2];
                let d = (dx * dx + dy * dy + dz * dz).sqrt();
                if d <= 0.0 {
                    continue;
                }
                let a_rad =
                    (res.a_res[i][0] * dx + res.a_res[i][1] * dy + res.a_res[i][2] * dz) / d;
                if a_rad.abs() > a_rad_max {
                    a_rad_max = a_rad.abs();
                }
                gms.push(a_rad * d * d);
            }
            if gms.len() < 20 {
                continue;
            }
            gms.sort_by(f64::total_cmp);
            let gm_med = gms[gms.len() / 2];
            let mut devs: Vec<f64> = gms.iter().map(|&g| (g - gm_med).abs()).collect();
            devs.sort_by(f64::total_cmp);
            let mad = devs[devs.len() / 2];
            if gm_med > 0.0 && mad < 0.5 * gm_med && a_rad_max > 5.0 * median_r {
                flags.push((idx, pi, gm_med, mad));
            }
        }
    }
    let mut per_point: Vec<Vec<(usize, f64, f64)>> = vec![Vec::new(); n_points];
    for &(idx, pi, gm, mad) in &flags {
        per_point[idx].push((pi, gm, mad));
    }
    let multi: Vec<usize> = per_point
        .iter()
        .enumerate()
        .filter(|(_, v)| v.len() >= 2)
        .map(|(i, _)| i)
        .collect();
    println!(
        "\n=== Triangulations-Gitter ({n_points} Punkte × {} Sonden, Direkt-Test a_rad·d² = GM, Konstanz) ===",
        PROBES.len()
    );
    println!(
        "  Flaggen gesamt {}/{n_points} — Punkte bei ≥2 Sonden: {}",
        flags.len(),
        multi.len()
    );
    let mut sorted = flags.clone();
    sorted.sort_by(|a, b| b.2.total_cmp(&a.2));
    for &(idx, pi, gm, mad) in sorted.iter().take(10) {
        let (radius_au, dir) = points[idx];
        let probe_name = PROBES[pi];
        println!(
            "  top  {:11} r {:>3.0} AU dir [{:+.2} {:+.2} {:+.2}] GM {:.3e} MAD {:.3e}",
            probe_name, radius_au, dir[0], dir[1], dir[2], gm, mad
        );
    }
    for &idx in &multi {
        let (radius_au, dir) = points[idx];
        let row = per_point[idx]
            .iter()
            .map(|&(pi, gm, _)| format!("{}({:.2e})", PROBES[pi], gm))
            .collect::<Vec<_>>()
            .join(" ");
        println!(
            "  TRIANGULIERT r {:>3.0} AU dir [{:+.2} {:+.2} {:+.2}] — {}",
            radius_au, dir[0], dir[1], dir[2], row
        );
    }
}

fn main() {
    std::fs::create_dir_all("data").ok();
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
        "model: Sun+8+Moon point masses (moon {moon_in}) + Earth-J2 {earth_j2}, residue a_obs − a_known per daily step"
    );

    let mut residues: HashMap<String, Residue> = HashMap::new();
    for name in PROBES {
        if !load_probe_daily(name, &mut eph) {
            continue;
        }
        if let Some(res) = compute_residue(name, &planets, &eph, None) {
            print_summary(name, &res);
            residues.insert(name.to_string(), res);
        }
    }

    let mut fam = f64::NEG_INFINITY;
    let mut results: Vec<Arrow> = Vec::new();

    println!(
        "\n=== positive control (fine arcs, leave-one-out) — the machine must recover the omitted planet and measure its mass ==="
    );
    for (probe, planet, ..) in POSITIVE {
        let arc_name = format!("{probe}_{planet}");
        if !load_arc(&arc_name, &mut eph) {
            continue;
        }
        if let Some(arrow) = fine_positive(probe, planet, &planets, &eph, &mut fam) {
            results.push(arrow);
        }
    }

    println!("\n=== null control (full model, fixed empty points) — must stay silent ===");
    for probe in ["voyager1", "voyager2"] {
        if let Some(res) = residues.get(probe) {
            for (label, x) in NULLS {
                let drv = fixed_distance(&res.pos, x);
                if let Some(arrow) = arrow_windowed(label, probe, res, &drv, &mut fam) {
                    results.push(arrow);
                }
            }
        }
    }

    println!();
    for r in &results {
        let word = if r.te > fam {
            "fam-tragend"
        } else if r.te > r.thr {
            "Pfeil ueber eigener Schwelle"
        } else {
            "still"
        };
        println!(
            "  {:<10} {:<28} {:<16} TE {:.4e} (thr {:.4e}, fam {:.4e}) | {}",
            r.probe, r.label, r.desc, r.te, r.thr, fam, word
        );
    }
    println!("\nfam (Mehrfachvergleich) = {fam:.4e}");

    run_grid(&residues);

    println!("\n=== self-test — does the TE machine carry an artificial lump (R = 1/d²)? ===");
    for (probe, planet) in [("voyager1", "saturn"), ("voyager2", "neptune")] {
        if let Some(res) = residues.get(probe) {
            selfcheck(probe, planet, res, &eph);
        }
    }
}
