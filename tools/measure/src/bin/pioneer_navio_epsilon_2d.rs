use std::collections::BTreeMap;
use std::collections::HashMap;

use omegaflow::archivar::odp::dsn_station;
use omegaflow::archivar::spectral::civil_from_days;
use omegaflow::archivar::{body_barycenter_position, parse_ephemeris_binary, BodyEphemeris};
use omegaflow::odf::parse_p11r_bin;

const DAY_S: f64 = 86400.0;
const AU: f64 = 1.495978707e11;
const J2000_JD: f64 = 2451545.0;
const MIN_DAY_SAMPLES: usize = 30;

fn load(name: &str, eph: &mut HashMap<String, BodyEphemeris>) -> bool {
    let p = format!("data/ephemeris_{name}.bin");
    std::fs::read(&p)
        .ok()
        .and_then(|d| parse_ephemeris_binary(&d))
        .map(|e| {
            eph.insert(name.to_string(), e);
        })
        .is_some()
}

fn norm(v: [f64; 3]) -> f64 {
    (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
}

fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn date_of(tdb: f64) -> String {
    let jd = tdb / DAY_S + J2000_JD;
    let unix_day = (jd - 2440587.5).round() as i64;
    match civil_from_days(unix_day) {
        Some((y, m, d)) => format!("{y:04}-{m:02}-{d:02}"),
        None => format!("tdb {tdb:.0} s"),
    }
}

fn year_of(tdb: f64) -> i64 {
    let jd = tdb / DAY_S + J2000_JD;
    let unix_day = (jd - 2440587.5).round() as i64;
    match civil_from_days(unix_day) {
        Some((y, _, _)) => y as i64,
        None => -1,
    }
}

fn median(v: &[f64]) -> f64 {
    assert!(!v.is_empty(), "median of an empty series is a fabricated value");
    let mut s = v.to_vec();
    s.sort_by(f64::total_cmp);
    s[s.len() / 2]
}

fn complex_letter(code: i64) -> String {
    match dsn_station(code) {
        Some((_, lon, _)) if lon < -50.0 => "G".to_string(),
        Some((_, lon, _)) if lon > 50.0 => "C".to_string(),
        Some((_, lon, _)) if lon < 0.0 => "M".to_string(),
        Some(_) | None => "?".to_string(),
    }
}

fn station_mix(st: &BTreeMap<i64, usize>) -> String {
    let mut parts: Vec<String> = Vec::new();
    for (code, n) in st {
        parts.push(format!("{}{code}:{n}", complex_letter(*code)));
    }
    parts.join(" ")
}

struct DayAccum {
    vals: Vec<f64>,
    st: BTreeMap<i64, usize>,
    tw3: usize,
}

struct DayRow {
    t: f64,
    au: f64,
    eps: f64,
    rms: f64,
    samp: usize,
    st: BTreeMap<i64, usize>,
    tw3: usize,
}

fn run(probe: &str, sc_body: &str) {
    let mut eph: HashMap<String, BodyEphemeris> = HashMap::new();
    for b in [sc_body, "earth"] {
        if !load(b, &mut eph) {
            eprintln!("{probe}: {b} ephemeris bin void");
            return;
        }
    }
    let p = format!("data/{probe}_navio_residuum.bin");
    let Ok(bytes) = std::fs::read(&p) else {
        eprintln!("{probe}: residuum bin void ({p})");
        return;
    };
    let Some(recs) = parse_p11r_bin(&bytes) else {
        eprintln!("{probe}: residuum parse void");
        return;
    };
    let mut acc: BTreeMap<i64, DayAccum> = BTreeMap::new();
    for r in &recs {
        if !r[1].is_finite() {
            continue;
        }
        let day = (r[0] / DAY_S).floor() as i64;
        let a = acc.entry(day).or_insert(DayAccum {
            vals: Vec::new(),
            st: BTreeMap::new(),
            tw3: 0,
        });
        a.vals.push(r[1]);
        *a.st.entry(r[5] as i64).or_insert(0usize) += 1;
        if (r[7] as i64) == 13 {
            a.tw3 += 1;
        }
    }
    let mut rows: Vec<DayRow> = Vec::new();
    for (day, a) in &acc {
        if a.vals.len() < MIN_DAY_SAMPLES {
            continue;
        }
        let t = *day as f64 * DAY_S;
        let (Some(p_pos), Some(e_pos)) = (
            body_barycenter_position(sc_body, t, &eph),
            body_barycenter_position("earth", t, &eph),
        ) else {
            continue;
        };
        let sun = [0.0, 0.0, 0.0];
        let r_probe = norm(sub(p_pos, sun));
        let r_earth = norm(sub(e_pos, sun));
        let e_to_p = sub(p_pos, e_pos);
        let r_e_p = norm(e_to_p);
        let elong_deg = (dot(sub(sun, e_pos), e_to_p) / (r_earth * r_e_p).max(1e-30))
            .clamp(-1.0, 1.0)
            .acos()
            .to_degrees();
        let m = a.vals.iter().sum::<f64>() / a.vals.len() as f64;
        let rms = (a
            .vals
            .iter()
            .map(|v| (v - m) * (v - m))
            .sum::<f64>()
            / a.vals.len() as f64)
            .sqrt();
        rows.push(DayRow {
            t,
            au: r_probe / AU,
            eps: elong_deg,
            rms,
            samp: a.vals.len(),
            st: a.st.clone(),
            tw3: a.tw3,
        });
    }
    rows.sort_by(|a, b| a.t.total_cmp(&b.t));
    if rows.is_empty() {
        eprintln!("{probe}: no qualifying days — silent (0 honored)");
        return;
    }
    eprintln!(
        "{probe}: {} days (>= {MIN_DAY_SAMPLES} resid/day), {}..{}, years {:?}..{:?}",
        rows.len(),
        date_of(rows[0].t),
        date_of(rows[rows.len() - 1].t),
        year_of(rows[0].t),
        year_of(rows[rows.len() - 1].t)
    );

    let band_med = |lo: f64, hi: f64, f: &dyn Fn(&DayRow) -> f64| -> Option<(usize, f64)> {
        let v: Vec<f64> = rows.iter().filter(|r| f(r) >= lo && f(r) < hi).map(|r| r.rms).collect();
        if v.is_empty() {
            None
        } else {
            Some((v.len(), median(&v)))
        }
    };
    for (lo, hi, axis) in [(0.0, 10.0, "eps"), (10.0, 20.0, "eps"), (0.0, 5.0, "dist"), (5.0, 10.0, "dist")] {
        let g = match axis {
            "eps" => |r: &DayRow| r.eps,
            _ => |r: &DayRow| r.au,
        };
        if let Some((n, med)) = band_med(lo, hi, &g) {
            eprintln!("  repro {axis} {lo:.0}-{hi:.0}: median resid-RMS {med:.0} Hz ({n} days)");
        }
    }

    let conj_idx: Vec<usize> = rows.iter().enumerate().filter(|(_, r)| r.eps < 10.0).map(|(i, _)| i).collect();
    eprintln!("{probe}: elongation 0-10 (conjunction) days n={}", conj_idx.len());
    let mut by_year: BTreeMap<i64, Vec<f64>> = BTreeMap::new();
    let mut by_au: BTreeMap<i64, Vec<f64>> = BTreeMap::new();
    for &i in &conj_idx {
        let r = &rows[i];
        by_year.entry(year_of(r.t)).or_default().push(r.rms);
        by_au.entry(r.au.floor() as i64).or_default().push(r.rms);
    }
    for (y, v) in &by_year {
        eprintln!("  era {y}: n={} rms med {:.0} ({:.0}..{:.0}) Hz", v.len(), median(v), v.iter().cloned().fold(f64::INFINITY, f64::min), v.iter().cloned().fold(f64::NEG_INFINITY, f64::max));
    }
    for (a, v) in &by_au {
        eprintln!("  dist {a}-{} AU: n={} rms med {:.0} ({:.0}..{:.0}) Hz", a + 1, v.len(), median(v), v.iter().cloned().fold(f64::INFINITY, f64::min), v.iter().cloned().fold(f64::NEG_INFINITY, f64::max));
    }
    if conj_idx.len() <= 70 {
        eprintln!("  ledger (date year distAU eps rmsHz nSamp stations threeWayShare):");
        for &i in &conj_idx {
            let r = &rows[i];
            eprintln!(
                "    {} y{} d{:.2}AU eps{:.1} rms{:.0}Hz n{} [{}] 3w{}/{}",
                date_of(r.t),
                year_of(r.t),
                r.au,
                r.eps,
                r.rms,
                r.samp,
                station_mix(&r.st),
                r.tw3,
                r.samp
            );
        }
    }
    for (tol, epsmin, label) in [(1.0, 10.0, "loose +-1AU eps>=10"), (0.4, 30.0, "strict +-0.4AU eps>=30")] {
        let mut pools = Vec::new();
        let mut diffs = Vec::new();
        let mut thin = 0usize;
        for &i in &conj_idx {
            let c = &rows[i];
            let y = year_of(c.t);
            let pool: Vec<f64> = rows
                .iter()
                .filter(|r| {
                    r.eps >= epsmin && year_of(r.t) == y && (r.au - c.au).abs() <= tol
                })
                .map(|r| r.rms)
                .collect();
            if pool.len() < 3 {
                thin += 1;
                continue;
            }
            let pm = median(&pool);
            pools.push(pm);
            diffs.push(c.rms - pm);
        }
        if !diffs.is_empty() {
            let louder = diffs.iter().filter(|d| **d > 0.0).count();
            let cmed =
                median(&conj_idx.iter().map(|&i| rows[i].rms).collect::<Vec<f64>>());
            eprintln!(
                "  2D control {label}: {}/{} conj days have >= 3 non-conj same-year days ({thin} thin); conj med {cmed:.0} Hz vs matched non-conj med {:.0} Hz; matched diff (conj - nonconj) med {:.0} Hz; conj louder than own pool {louder}/{}",
                diffs.len(),
                conj_idx.len(),
                median(&pools),
                median(&diffs),
                diffs.len()
            );
        } else {
            eprintln!("  2D control {label}: {thin} conj days, none with a >= 3-day non-conj same-year pool — the era x distance cell is data-thin (0 honored)");
        }
    }
        eprintln!("{probe}: annual geometry + era profile (all tracked days by year):");
        for y in 1971..=2003 {
            let yr: Vec<&DayRow> = rows.iter().filter(|r| year_of(r.t) == y).collect();
            if yr.is_empty() {
                continue;
            }
            let mine = yr.iter().map(|r| r.eps).fold(f64::INFINITY, f64::min);
            let nconj = yr.iter().filter(|r| r.eps < 10.0).count();
            let rmsv: Vec<f64> = yr.iter().map(|r| r.rms).collect();
            eprintln!("    {y}: n={} minEps {mine:.1} deg conj<10 {nconj} medRMS {:.0} (lo {:.0}..hi {:.0}) Hz", yr.len(), median(&rmsv), rmsv.iter().cloned().fold(f64::INFINITY, f64::min), rmsv.iter().cloned().fold(f64::NEG_INFINITY, f64::max));
        }
    if sc_body.contains("11") {
        eprintln!("{probe}: resid-RMS by 0.5-AU heliocentric band (whole mission, 0-13 AU):");
        let mut au5: BTreeMap<i64, Vec<f64>> = BTreeMap::new();
        for r in &rows {
            if r.au < 13.0 {
                au5.entry((r.au * 2.0).floor() as i64).or_default().push(r.rms);
            }
        }
        for (k, v) in &au5 {
            eprintln!(
                "    {:.1}-{:.1} AU: n={} med {:.0} ({:.0}..{:.0}) Hz",
                *k as f64 * 0.5,
                *k as f64 * 0.5 + 0.5,
                v.len(),
                median(v),
                v.iter().cloned().fold(f64::INFINITY, f64::min),
                v.iter().cloned().fold(f64::NEG_INFINITY, f64::max)
            );
        }
        let mut conj_years: BTreeMap<i64, Vec<f64>> = BTreeMap::new();
        for &i in &conj_idx {
            conj_years.entry(year_of(rows[i].t)).or_default().push(rows[i].au);
        }
        for (y, aus) in &conj_years {
            let lo = aus.iter().cloned().fold(f64::INFINITY, f64::min) - 0.7;
            let hi = aus.iter().cloned().fold(f64::NEG_INFINITY, f64::max) + 0.7;
            eprintln!(
                "{probe}: {y} days within {lo:.1}-{hi:.1} AU by elongation band (era x distance window of the conj days):"
            );
            let bands: [(f64, f64, &str); 7] = [
                (0.0, 10.0, "0-10 conj"),
                (10.0, 30.0, "10-30"),
                (30.0, 60.0, "30-60"),
                (60.0, 90.0, "60-90"),
                (90.0, 120.0, "90-120"),
                (120.0, 150.0, "120-150"),
                (150.0, 180.0, "150-180 opp"),
            ];
            for (blo, bhi, bname) in bands {
                let v: Vec<f64> = rows
                    .iter()
                    .filter(|r| {
                        year_of(r.t) == *y && r.au >= lo && r.au <= hi && r.eps >= blo && r.eps < bhi
                    })
                    .map(|r| r.rms)
                    .collect();
                if !v.is_empty() {
                    eprintln!(
                        "    eps {bname}: n={} med {:.0} ({:.0}..{:.0}) Hz",
                        v.len(),
                        median(&v),
                        v.iter().cloned().fold(f64::INFINITY, f64::min),
                        v.iter().cloned().fold(f64::NEG_INFINITY, f64::max)
                    );
                }
            }
        }
    }
}

fn main() {
    for (probe, sc) in [
        ("pioneer10", "pioneer10_daily"),
        ("pioneer11", "pioneer11_daily"),
    ] {
        run(probe, sc);
    }
}
