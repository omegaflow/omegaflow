use std::collections::{BTreeMap, HashMap};

use omegaflow::archivar::{BodyEphemeris, body_barycenter_position, parse_ephemeris_binary};
use omegaflow::spectral::civil_from_days;

const DAY_S: f64 = 86400.0;
const LOCK_HZ: f64 = 1.0e3;
const AU_M: f64 = 1.495978707e11;
const MIN_RANK_N: usize = 8;

fn norm(v: [f64; 3]) -> f64 {
    (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
}
fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
fn rms(vals: &[f64]) -> f64 {
    if vals.is_empty() {
        return f64::NAN;
    }
    let m = vals.iter().sum::<f64>() / vals.len() as f64;
    (vals.iter().map(|v| (v - m) * (v - m)).sum::<f64>() / vals.len() as f64).sqrt()
}
fn median(vals: &[f64]) -> Option<f64> {
    if vals.is_empty() {
        return None;
    }
    let mut s = vals.to_vec();
    s.sort_by(f64::total_cmp);
    Some(s[s.len() / 2])
}
fn load(name: &str, eph: &mut HashMap<String, BodyEphemeris>) -> bool {
    std::fs::read(format!("data/ephemeris_{name}.bin"))
        .ok()
        .and_then(|d| parse_ephemeris_binary(&d))
        .map(|e| {
            eph.insert(name.to_string(), e);
        })
        .is_some()
}
fn ymd(tdb_day: i64) -> (i32, u32, u32) {
    let jd = 2451545.0 + tdb_day as f64;
    let unix_day = (jd - 2440587.5).round() as i64;
    match civil_from_days(unix_day) {
        Some((y, m, d)) => (y as i32, m, d),
        None => (0, 0, 0),
    }
}
fn year_q(tdb_day: i64) -> (i32, u32) {
    let (y, m, _) = ymd(tdb_day);
    (y, (m.saturating_sub(1)) / 3)
}
fn spearman(x: &[f64], y: &[f64]) -> Option<f64> {
    let n = x.len();
    if n < MIN_RANK_N || n != y.len() {
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
    if dx2 <= 0.0 || dy2 <= 0.0 {
        return None;
    }
    Some(num / (dx2 * dy2).sqrt())
}

fn main() {
    let mut eph: HashMap<String, BodyEphemeris> = HashMap::new();
    for b in ["galileo_daily", "earth"] {
        if !load(b, &mut eph) {
            eprintln!("galileo: {b} ephemeris bin void");
            return;
        }
    }
    let Ok(bytes) = std::fs::read("data/galileo_resid.bin") else {
        eprintln!("galileo: resid bin void");
        return;
    };
    let Some(recs) = omegaflow::atdf::parse_resid_bin(&bytes) else {
        eprintln!("galileo: resid bin parse void");
        return;
    };
    let n_lock_total: usize = recs.iter().filter(|r| r[1].abs() > LOCK_HZ).count();
    println!(
        "galileo era x cycle probe: {} resid samples, {} lock-transition samples excluded",
        recs.len(),
        n_lock_total
    );
    println!(
        "era proxy = calendar date (no offline monthly SSN series in data/; measured absent). geometry = epsilon at the Earth, alpha at the Sun, heliocentric AU at the TDB day start."
    );

    let mut day_samp: BTreeMap<(i64, i64), (Vec<f64>, usize)> = BTreeMap::new();
    for r in &recs {
        let mode = r[3] as i64;
        let day = (r[0] / DAY_S).floor() as i64;
        let e = day_samp
            .entry((mode, day))
            .or_insert_with(|| (Vec::new(), 0));
        e.1 += 1;
        if r[1].abs() <= LOCK_HZ {
            e.0.push(r[1]);
        }
    }
    drop(recs);

    let mut geo: BTreeMap<i64, (f64, f64, f64)> = BTreeMap::new();
    for &(_, day) in day_samp.keys() {
        let t = day as f64 * DAY_S;
        if let (Some(p), Some(e)) = (
            body_barycenter_position("galileo_daily", t, &eph),
            body_barycenter_position("earth", t, &eph),
        ) {
            let r_probe = norm(p);
            let r_earth = norm(e);
            let e_to_p = sub(p, e);
            let r_e_p = norm(e_to_p);
            let alpha = (dot(e, p) / (r_earth * r_probe).max(1e-30))
                .clamp(-1.0, 1.0)
                .acos()
                .to_degrees();
            let eps = (dot(sub([0.0; 3], e), e_to_p) / (r_earth * r_e_p).max(1e-30))
                .clamp(-1.0, 1.0)
                .acos()
                .to_degrees();
            geo.insert(day, (eps, alpha, r_probe / AU_M));
        }
    }

    let reg = |eps: f64| -> &'static str {
        if eps <= 30.0 {
            "CONJ eps0-30"
        } else if eps >= 150.0 {
            "OPP eps150-180"
        } else {
            "MID eps30-150"
        }
    };

    let mut cells: Vec<(i64, i64, f64, f64, i64)> = Vec::new();
    for ((mode, day), (v, _)) in &day_samp {
        if v.is_empty() {
            continue;
        }
        let rr = rms(v);
        if !rr.is_finite() {
            continue;
        }
        let Some((eps, _, _)) = geo.get(day).copied() else {
            continue;
        };
        cells.push((*mode, *day, rr, eps, v.len() as i64));
    }

    let modes = [1i64, 2, 3];
    for m in modes {
        let rows: Vec<&(i64, i64, f64, f64, i64)> = cells.iter().filter(|c| c.0 == m).collect();
        let (mut yr, mut qr): (
            BTreeMap<(i64, u32, &str), Vec<f64>>,
            BTreeMap<(i64, &str), Vec<f64>>,
        ) = (BTreeMap::new(), BTreeMap::new());
        for c in &rows {
            let (y, q) = year_q(c.1);
            let rl = reg(c.3);
            yr.entry((y as i64, q, rl)).or_default().push(c.2);
            qr.entry((y as i64, rl)).or_default().push(c.2);
        }
        println!("\n== mode {m}: {} day cells ==", rows.len());
        let mut spe: Vec<(String, f64, f64, usize)> = Vec::new();
        for rl in ["CONJ eps0-30", "MID eps30-150", "OPP eps150-180"] {
            let days: Vec<f64> = cells
                .iter()
                .filter(|c| c.0 == m && reg(c.3) == rl)
                .map(|c| c.1 as f64)
                .collect();
            let rms_v: Vec<f64> = cells
                .iter()
                .filter(|c| c.0 == m && reg(c.3) == rl)
                .map(|c| c.2)
                .collect();
            let rho = spearman(&days, &rms_v);
            let nd = days.len();
            if nd > 0 {
                spe.push((
                    rl.to_string(),
                    rho.unwrap_or(f64::NAN),
                    median(&rms_v).unwrap_or(f64::NAN),
                    nd,
                ));
            }
        }
        for (rl, rho, med, nd) in &spe {
            println!(
                "  {rl}: n_days {nd}  pooled-day median {med:.1} Hz  spearman(dayRMS vs day) {rho:+.2}"
            );
        }
        println!("  per (year, quarter, regime): n days, median day RMS Hz");
        for ((y, q, rl), v) in &yr {
            println!(
                "    {y}-q{q} {rl}: n {:>2}  med {:6.2} Hz",
                v.len(),
                median(v).unwrap_or(f64::NAN)
            );
        }
        println!("  per (year, regime) pooled across quarters:");
        for ((y, rl), v) in &qr {
            println!(
                "    {y} {rl}: n_days {:>2}  med {:6.2} Hz",
                v.len(),
                median(v).unwrap_or(f64::NAN)
            );
        }
    }

    println!("\n== decisive day rows ==");
    for m in [2i64, 3] {
        let rows: Vec<&(i64, i64, f64, f64, i64)> = cells.iter().filter(|c| c.0 == m).collect();
        println!("mode {m}: all OPP (eps>=150) day cells, individually:");
        for c in rows.iter().filter(|c| reg(c.3) == "OPP eps150-180") {
            let (eps, alpha, au) = geo
                .get(&c.1)
                .copied()
                .unwrap_or((f64::NAN, f64::NAN, f64::NAN));
            let (y, mo, d) = ymd(c.1);
            println!(
                "  {y}-{mo:02}-{d:02}  eps {eps:5.1}  alpha {alpha:5.1}  {au:4.2} AU  dayRMS {:.2} Hz  n_samp {ns}",
                c.2,
                ns = c.4
            );
        }
        println!("mode {m}: far-side opposition (eps>=150 AND AU>=4.5) = the late-era loud test:");
        for c in rows.iter().filter(|c| {
            reg(c.3) == "OPP eps150-180" && geo.get(&c.1).map(|g| g.2 >= 4.5).unwrap_or(false)
        }) {
            let (eps, alpha, au) = geo
                .get(&c.1)
                .copied()
                .unwrap_or((f64::NAN, f64::NAN, f64::NAN));
            let (y, mo, d) = ymd(c.1);
            println!(
                "  {y}-{mo:02}-{d:02}  eps {eps:5.1}  alpha {alpha:5.1}  {au:4.2} AU  dayRMS {:.2} Hz  n_samp {ns}",
                c.2,
                ns = c.4
            );
        }
        println!("mode {m}: near opposition cruise (eps>=150 AND AU<2.5), per-year median:");
        let mut near: BTreeMap<i64, Vec<f64>> = BTreeMap::new();
        for c in rows.iter().filter(|c| {
            reg(c.3) == "OPP eps150-180" && geo.get(&c.1).map(|g| g.2 < 2.5).unwrap_or(false)
        }) {
            let (y, _, _) = ymd(c.1);
            near.entry(y as i64).or_default().push(c.2);
        }
        for (y, v) in &near {
            println!(
                "    {y}: n {} med {:.2} Hz",
                v.len(),
                median(v).unwrap_or(f64::NAN)
            );
        }
    }
    println!("\n== same-era geometry contrast: years with BOTH OPP and CONJ day cells ==");
    for m in modes {
        let rows: Vec<&(i64, i64, f64, f64, i64)> = cells.iter().filter(|c| c.0 == m).collect();
        let mut by_y: BTreeMap<i64, (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>)> = BTreeMap::new();
        for c in &rows {
            let (y, _, _) = ymd(c.1);
            let e = by_y
                .entry(y as i64)
                .or_insert_with(|| (Vec::new(), Vec::new(), Vec::new(), Vec::new()));
            if reg(c.3) == "OPP eps150-180" {
                e.0.push(c.2);
                e.3.push(c.1 as f64);
            } else if reg(c.3) == "CONJ eps0-30" {
                e.1.push(c.2);
                e.2.push(c.1 as f64);
            }
        }
        for (y, (opp, conj, _, _)) in &by_y {
            if opp.len() >= 2 && conj.len() >= 2 {
                let rho_all = spearman(
                    &{
                        let mut v: Vec<f64> = Vec::new();
                        for c in rows.iter().filter(|c| ymd(c.1).0 as i64 == *y) {
                            v.push(c.1 as f64);
                        }
                        v
                    },
                    &{
                        let mut v: Vec<f64> = Vec::new();
                        for c in rows.iter().filter(|c| ymd(c.1).0 as i64 == *y) {
                            v.push(c.2);
                        }
                        v
                    },
                );
                let med_o = median(opp).unwrap_or(f64::NAN);
                let med_c = median(conj).unwrap_or(f64::NAN);
                println!(
                    "  mode {m} {y}: OPP n {:>2} med {:.2} Hz | CONJ n {:>2} med {:.2} Hz | ratio {:.1}x | same-year spearman(all eps) {rho_all:+.2}",
                    opp.len(),
                    med_o,
                    conj.len(),
                    med_c,
                    med_o / med_c,
                    rho_all = rho_all.unwrap_or(f64::NAN)
                );
            }
        }
    }
}
