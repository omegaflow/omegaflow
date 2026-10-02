use omegaflow::archivar::ephemeris::{
    CHEBYSHEV_DEGREE, GRANULE_DAYS, N_SAMPLES, chebyshev_fit, chebyshev_nodes,
};
use omegaflow::archivar::{
    BodyEphemeris, CHEBYSHEV_N, ChebyshevGranule, J2000_EPOCH, body_barycenter_position,
    chebyshev_evaluate, parse_ephemeris_binary,
};
use omegaflow::lsk::days_from_civil;
use std::collections::HashMap;

const DEFAULT_DE: &str = "data/ssd.jpl.nasa.gov-de/ephemeris_de441_earth.bin";
const DEFAULT_INPOP: &str = "data/ftp.imcce.fr/ephemeris_inpop_earth.bin";
const DEFAULT_EPM: &str = "data/ftp.iaaras.ru/ephemeris_epm_earth.bin";
const DAY: f64 = 86400.0;

fn dist_km(a: [f64; 3], b: [f64; 3]) -> f64 {
    ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2) + (a[2] - b[2]).powi(2)).sqrt() / 1000.0
}

fn num(v: Option<f64>) -> String {
    match v {
        Some(x) => format!("{x:.6}"),
        None => "pending".to_string(),
    }
}

fn sample(g: &ChebyshevGranule, tau: f64) -> [f64; 3] {
    [
        chebyshev_evaluate(&g.cx, tau),
        chebyshev_evaluate(&g.cy, tau),
        chebyshev_evaluate(&g.cz, tau),
    ]
}

fn load(path: &str) -> Option<HashMap<String, BodyEphemeris>> {
    let bytes = std::fs::read(path).ok()?;
    let e = parse_ephemeris_binary(&bytes)?;
    let mut m = HashMap::new();
    m.insert("earth".to_string(), e);
    Some(m)
}

fn report(word: &str, path: &str) {
    let bytes = match std::fs::read(path) {
        Ok(b) => b,
        Err(_) => {
            println!("{word}: absent ({path})");
            return;
        }
    };
    let e = match parse_ephemeris_binary(&bytes) {
        Some(e) => e,
        None => {
            println!("{word}: parses void ({path})");
            return;
        }
    };
    let mut gs: Vec<&ChebyshevGranule> = e.granules.iter().collect();
    gs.sort_by(|a, b| {
        a.t0_jd
            .partial_cmp(&b.t0_jd)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    let n = gs.len();
    let mut dts: Vec<f64> = gs.iter().map(|g| g.dt_jd).collect();
    dts.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let dt_min = dts.first().copied();
    let dt_med = dts.get(dts.len() / 2).copied();
    let dt_max = dts.last().copied();
    let lo = gs.first().map(|g| g.t0_jd - g.dt_jd);
    let hi = gs.last().map(|g| g.t0_jd + g.dt_jd);
    let span_yr = match (lo, hi) {
        (Some(l), Some(h)) => Some((h - l) / 365.25),
        _ => None,
    };
    let mut deg = 0usize;
    for g in &gs {
        for axis in [&g.cx, &g.cy, &g.cz] {
            for k in (0..CHEBYSHEV_N).rev() {
                if axis[k].abs() > 0.0 {
                    if k > deg {
                        deg = k;
                    }
                    break;
                }
            }
        }
    }
    let mut gap_max: Option<f64> = None;
    let mut gap_at_jd = 0.0;
    let mut mism: Vec<f64> = Vec::new();
    let mut mism_max: Option<(f64, f64)> = None;
    let mut overlaps = 0usize;
    for pair in gs.windows(2) {
        let a = pair[0];
        let b = pair[1];
        let end_a = a.t0_jd + a.dt_jd;
        let start_b = b.t0_jd - b.dt_jd;
        if start_b < end_a - 1.0e-6 {
            overlaps += 1;
            continue;
        }
        let gap = (start_b - end_a).abs();
        if match gap_max {
            Some(m) => gap > m,
            None => true,
        } {
            gap_max = Some(gap);
            gap_at_jd = start_b;
        }
        let d = dist_km(sample(a, 1.0), sample(b, -1.0));
        if d.is_finite() {
            mism.push(d);
            if match mism_max {
                Some((m, _)) => d > m,
                None => true,
            } {
                mism_max = Some((d, start_b));
            }
        }
    }
    mism.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mism_med = mism.get(mism.len() / 2).copied();
    println!("{word}: {path}");
    println!(
        "  granules {n}, half-width dt (d): min {} median {} max {}",
        num(dt_min),
        num(dt_med),
        num(dt_max)
    );
    println!(
        "  coverage JD {}..{} ({} yr), effective Chebyshev degree {deg} (of {CHEBYSHEV_N})",
        num(lo),
        num(hi),
        num(span_yr)
    );
    println!(
        "  boundary gap max {} d at JD {}, reconstruction mismatch (km): median {} max {} at JD {}, overlapping pairs {}",
        num(gap_max),
        num(Some(gap_at_jd)),
        num(mism_med),
        num(mism_max.map(|x| x.0)),
        num(mism_max.map(|x| x.1)),
        overlaps
    );
}

fn series(center_jd: f64, half_days: f64) {
    let de = load(DEFAULT_DE);
    let inpop = load(DEFAULT_INPOP);
    let epm = load(DEFAULT_EPM);
    let (Some(de), Some(inpop), Some(epm)) = (de, inpop, epm) else {
        println!("series: a bin is absent");
        return;
    };
    let step = 1.0 / 24.0;
    let n = (2.0 * half_days / step) as i64;
    let mut prev: Option<[[f64; 3]; 3]> = None;
    let mut max_step = [(0.0f64, 0.0f64); 3];
    for k in 0..=n {
        let jd = center_jd - half_days + k as f64 * step;
        let tdb = (jd - J2000_EPOCH) * DAY;
        let p = |m: &HashMap<String, BodyEphemeris>| body_barycenter_position("earth", tdb, m);
        let (Some(a), Some(b), Some(c)) = (p(&de), p(&inpop), p(&epm)) else {
            continue;
        };
        let sub = |u: [f64; 3], v: [f64; 3]| [u[0] - v[0], u[1] - v[1], u[2] - v[2]];
        let off = [sub(a, b), sub(a, c), sub(b, c)];
        let jd_prev = jd - step;
        let straddles = (0..3).any(|i| {
            let m = [&de, &inpop, &epm][i];
            let e = &m["earth"];
            e.granules.iter().any(|g| {
                let lo = g.t0_jd - g.dt_jd;
                let hi = g.t0_jd + g.dt_jd;
                (lo >= jd_prev && lo <= jd) || (hi >= jd_prev && hi <= jd)
            })
        });
        if let Some(po) = prev {
            if !straddles {
                for i in 0..3 {
                    let d = dist_km(po[i], off[i]);
                    if d > max_step[i].0 {
                        max_step[i] = (d, jd);
                    }
                }
            }
        }
        prev = Some(off);
    }
    println!("series around JD {center_jd:.4} ±{half_days} d, 1-h steps — largest 1-h change:");
    for (i, (v, jd)) in max_step.iter().enumerate() {
        let word = ["de-inpop", "de-epm", "inpop-epm"][i];
        println!(
            "  {word}: {v:.6} km/h at JD {jd:.4} ({:.1} mm/s)",
            v * 1.0e6 / 3600.0
        );
    }
    for (word, m) in [("de", &de), ("inpop", &inpop), ("epm", &epm)] {
        let e = &m["earth"];
        let mut bs: Vec<f64> = Vec::new();
        for g in &e.granules {
            for b in [g.t0_jd - g.dt_jd, g.t0_jd + g.dt_jd] {
                if (b - center_jd).abs() <= half_days {
                    bs.push(b);
                }
            }
        }
        bs.sort_by(|a, b| a.partial_cmp(b).unwrap());
        println!("  {word} boundaries in window: {} {:?}", bs.len(), bs);
    }
}

fn selftest() {
    fn curve(et: f64) -> (f64, f64, f64) {
        let base = 1.5e11;
        (
            base + 3.0e4 * et + 0.5 * 6.0e-3 * et * et,
            1.0e10 + 2.9e4 * et - 0.5 * 5.0e-3 * et * et,
            -4.0e9 + 1.0e4 * et + 0.5 * 2.0e-3 * et * et,
        )
    }
    let half = GRANULE_DAYS * 86400.0 / 2.0;
    let fit = |mid: f64| -> [f64; 3] {
        let samples: Vec<(f64, f64, f64)> = chebyshev_nodes(N_SAMPLES)
            .iter()
            .map(|tau| curve(mid + tau * half))
            .collect();
        let (cx, cy, cz) = chebyshev_fit(&samples, CHEBYSHEV_DEGREE).expect("fits");
        let arr = |v: &Vec<f64>| {
            let mut a = [0.0f64; CHEBYSHEV_N];
            for (i, x) in v.iter().enumerate().take(CHEBYSHEV_N) {
                a[i] = *x;
            }
            a
        };
        let (cxa, cya, cza) = (arr(&cx), arr(&cy), arr(&cz));
        [
            chebyshev_evaluate(&cxa, 1.0),
            chebyshev_evaluate(&cya, 1.0),
            chebyshev_evaluate(&cza, 1.0),
        ]
    };
    let a = fit(0.0);
    let b_mid = GRANULE_DAYS * 86400.0;
    let samples_b: Vec<(f64, f64, f64)> = chebyshev_nodes(N_SAMPLES)
        .iter()
        .map(|tau| curve(b_mid + tau * half))
        .collect();
    let (cx, cy, cz) = chebyshev_fit(&samples_b, CHEBYSHEV_DEGREE).expect("fits");
    let arr = |v: &Vec<f64>| {
        let mut x = [0.0f64; CHEBYSHEV_N];
        for (i, v) in v.iter().enumerate().take(CHEBYSHEV_N) {
            x[i] = *v;
        }
        x
    };
    let (cxa, cya, cza) = (arr(&cx), arr(&cy), arr(&cz));
    let b = [
        chebyshev_evaluate(&cxa, -1.0),
        chebyshev_evaluate(&cya, -1.0),
        chebyshev_evaluate(&cza, -1.0),
    ];
    let (ex, ey, ez) = curve(half);
    let expected = [ex, ey, ez];
    let mut max = 0.0f64;
    for k in 0..3 {
        let miss = (a[k] - expected[k]).abs();
        let gap = (a[k] - b[k]).abs();
        println!("axis {k}: fit miss {miss:.6} m, boundary gap {gap:.6} m");
        max = max.max(miss).max(gap);
    }
    println!("synthetic granule selftest: max deviation {max:.6} m");
}

fn arg(flag: &str, args: &[String]) -> Option<String> {
    args.iter()
        .position(|a| a == flag)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.first().map(|s| s.as_str()) == Some("--selftest") {
        selftest();
        return;
    }
    if args.first().map(|s| s.as_str()) == Some("--series") {
        let date = match arg("--date", &args) {
            Some(d) => d,
            None => {
                println!("series: --date carries no YYYY-MM-DD");
                return;
            }
        };
        let half: f64 = match arg("--half-days", &args).and_then(|s| s.parse::<f64>().ok()) {
            Some(v) if v.is_finite() && v > 0.0 => v,
            _ => {
                println!("series: --half-days carries no positive f64");
                return;
            }
        };
        let p: Vec<&str> = date.split('-').collect();
        let center_jd = p
            .get(0)
            .and_then(|y| y.parse::<i64>().ok())
            .zip(p.get(1).and_then(|m| m.parse::<i64>().ok()))
            .zip(p.get(2).and_then(|d| d.parse::<i64>().ok()))
            .and_then(|((y, m), d)| days_from_civil(y, m, d))
            .map(|days| days as f64 + 2440587.5);
        match center_jd {
            Some(jd) => series(jd, half),
            None => println!("series: --date carries no YYYY-MM-DD"),
        }
        return;
    }
    let path_or = |idx: usize, def: &str| -> String {
        match args.get(idx) {
            Some(v) => v.clone(),
            None => def.to_string(),
        }
    };
    let de = path_or(0, DEFAULT_DE);
    let inpop = path_or(1, DEFAULT_INPOP);
    let epm = path_or(2, DEFAULT_EPM);
    report("de", &de);
    report("inpop", &inpop);
    report("epm", &epm);
}
