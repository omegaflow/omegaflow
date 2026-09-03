use std::collections::HashMap;

use omegaflow::archivar::{
    body_barycenter_position, parse_ephemeris_binary, BodyEphemeris, J2000_EPOCH,
};

const DAY_S: f64 = 86400.0;
const AU: f64 = 1.495978707e11;

fn jd_date(tdb: f64) -> String {
    let jd = tdb / DAY_S + J2000_EPOCH;
    let unix_day = (jd - 2440587.5).round() as i64;
    match omegaflow::spectral::civil_from_days(unix_day) {
        Some((y, m, d)) => format!("{y:04}-{m:02}-{d:02}"),
        None => format!("tdb {tdb:.0} s"),
    }
}

fn ymd_to_tdb(y: i32, m: u32, d: u32) -> Option<f64> {
    let days = omegaflow::archivar::lsk::days_from_civil(y as i64, m as i64, d as i64)?;
    Some((days as f64 + 2440587.5 - J2000_EPOCH) * DAY_S)
}

fn tdb_of_date(s: &str) -> Option<f64> {
    let p: Vec<&str> = s.split('-').collect();
    if p.len() != 3 {
        return None;
    }
    let y: i32 = p[0].parse().ok()?;
    let m: u32 = p[1].parse().ok()?;
    let d: u32 = p[2].parse().ok()?;
    ymd_to_tdb(y, m, d)
}

fn load_eph(name: &str, eph: &mut HashMap<String, BodyEphemeris>) -> bool {
    let path = format!("data/ephemeris_{name}.bin");
    match std::fs::read(&path)
        .ok()
        .and_then(|d| parse_ephemeris_binary(&d))
    {
        Some(e) => {
            eph.insert(name.to_string(), e);
            true
        }
        None => false,
    }
}

fn norm(v: [f64; 3]) -> f64 {
    (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
}

fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn main() {
    let p10_flags = [
        "1980-06-24",
        "1980-12-07",
        "1981-02-09",
        "1982-01-30",
        "1982-03-12",
        "1982-06-19",
        "1996-05-28",
        "1996-10-15",
        "1996-10-28",
        "1996-11-07",
        "1996-11-22",
    ];
    let p11_flags = [
        "1974-11-29",
        "1974-12-03",
        "1974-12-05",
        "1979-08-31",
        "1979-09-01",
        "1981-01-25",
        "1981-09-27",
        "1982-04-03",
        "1982-04-05",
        "1982-04-12",
        "1983-07-18",
    ];

    let mut eph: HashMap<String, BodyEphemeris> = HashMap::new();
    for b in ["pioneer10_daily", "pioneer11_daily"] {
        if !load_eph(b, &mut eph) {
            eprintln!("{b}: ephemeris bin void");
            return;
        }
    }

    eprintln!("P10 heliocentric positions at its Ruck-flag epochs:");
    let mut p10_pos: Vec<(f64, [f64; 3])> = Vec::new();
    for d in &p10_flags {
        let Some(t) = tdb_of_date(d) else { continue };
        let Some(pos) = body_barycenter_position("pioneer10_daily", t, &eph) else {
            eprintln!("  {d}: no position");
            continue;
        };
        let r = norm(pos) / AU;
        p10_pos.push((t, pos));
        eprintln!("  {d}: r = {r:.2} AU");
    }

    eprintln!("\nP11 heliocentric positions at its Ruck-flag epochs:");
    let mut p11_pos: Vec<(f64, [f64; 3])> = Vec::new();
    for d in &p11_flags {
        let Some(t) = tdb_of_date(d) else { continue };
        let Some(pos) = body_barycenter_position("pioneer11_daily", t, &eph) else {
            eprintln!("  {d}: no position");
            continue;
        };
        let r = norm(pos) / AU;
        p11_pos.push((t, pos));
        eprintln!("  {d}: r = {r:.2} AU");
    }

    eprintln!("\nCross-spacecraft separation at each P10 flag (P11 position that day):");
    for (t, p10) in &p10_pos {
        let d = jd_date(*t);
        let Some(p11) = body_barycenter_position("pioneer11_daily", *t, &eph) else {
            eprintln!("  {d}: no P11 position");
            continue;
        };
        let sep = norm(sub(*p10, p11)) / AU;
        let dt_days = (ymd_to_tdb(1981, 2, 9).unwrap_or(*t) - *t) / DAY_S;
        eprintln!(
            "  {d}: P10–P11 separation {sep:.2} AU (Δ from P10 1981-02-09 = {dt_days:+.0} d)"
        );
    }

    eprintln!("\nCross-spacecraft separation at each P11 flag (P10 position that day):");
    for (t, p11) in &p11_pos {
        let d = jd_date(*t);
        let Some(p10) = body_barycenter_position("pioneer10_daily", *t, &eph) else {
            eprintln!("  {d}: no P10 position");
            continue;
        };
        let sep = norm(sub(*p11, p10)) / AU;
        eprintln!("  {d}: P10–P11 separation {sep:.2} AU");
    }
}
