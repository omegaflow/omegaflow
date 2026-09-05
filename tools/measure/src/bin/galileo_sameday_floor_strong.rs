use std::collections::{BTreeMap, BTreeSet, HashMap};

use omegaflow::archivar::{BodyEphemeris, body_barycenter_position, parse_ephemeris_binary};
use omegaflow::spectral::civil_from_days;

const DAY_S: f64 = 86400.0;
const LOCK_HZ: f64 = 1.0e3;
const MIN_CLASS: usize = 30;
const FLOOR: i64 = -2560;
const STRONG_MIN: i64 = -1750;
const CONJ_ELONG_DEG: f64 = 30.0;
const OUT: &str = "reports/galileo_sameday_floor_strong.txt";

fn norm(v: [f64; 3]) -> f64 {
    (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
}

fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn ang(a: [f64; 3], b: [f64; 3]) -> f64 {
    let na = norm(a);
    let nb = norm(b);
    if na <= 0.0 || nb <= 0.0 {
        return f64::NAN;
    }
    (dot(a, b) / (na * nb)).clamp(-1.0, 1.0).acos().to_degrees()
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
    let p = format!("data/ephemeris_{name}.bin");
    std::fs::read(&p)
        .ok()
        .and_then(|d| parse_ephemeris_binary(&d))
        .map(|e| eph.insert(name.to_string(), e))
        .is_some()
}

fn jd_date(tdb_s: f64) -> String {
    let jd = 2451545.0 + tdb_s / DAY_S;
    let unix_day = (jd - 2440587.5).round() as i64;
    match civil_from_days(unix_day) {
        Some((y, m, d)) => format!("{y:04}-{m:02}-{d:02}"),
        None => format!("tdb {tdb_s:.0}"),
    }
}

fn fmt_o(v: Option<f64>) -> String {
    match v {
        Some(x) if x.is_finite() => format!("{x:.3}"),
        _ => "-".to_string(),
    }
}

fn sign_stats(diffs: &[f64]) -> (usize, usize) {
    let pos = diffs.iter().filter(|d| **d > 0.0).count();
    let neg = diffs.iter().filter(|d| **d < 0.0).count();
    (pos, neg)
}

fn main() {
    let mut eph: HashMap<String, BodyEphemeris> = HashMap::new();
    for b in ["galileo_daily", "earth"] {
        if !load(b, &mut eph) {
            eprintln!("galileo: {b} ephemeris bin void");
        }
    }
    let geom_ok = eph.contains_key("galileo_daily") && eph.contains_key("earth");

    let Ok(bytes) = std::fs::read("data/galileo_resid.bin") else {
        eprintln!("galileo: resid bin void");
        return;
    };
    let Some(recs) = omegaflow::atdf::parse_resid_bin(&bytes) else {
        eprintln!("galileo: resid bin parse void");
        return;
    };
    drop(bytes);

    let mut out: Vec<String> = Vec::new();
    out.push("galileo same-day floor-vs-strong paired comparison".to_string());
    out.push("binding: mode records only (mode 2 primary, mode 1 secondary); lock transitions (|resid| > 1000 Hz) excluded before noise".to_string());
    out.push("classes: floor = signal_strength == -2560 (AGC floor), strong = signal_strength >= -1750 (strong plateau tail); strength == 0 separated, never classed".to_string());
    out.push("pair unit: (calendar day, station) that carries BOTH >= 30 floor samples AND >= 30 strong samples".to_string());
    out.push("noise = per-class RMS about the class mean within the pair; diff = floor RMS - strong RMS (same day, same station)".to_string());
    out.push("geometry: elong = angle at Earth (Sun/probe); conjunction = elong < 30 deg; day constant by construction, era/geometry shared".to_string());
    out.push(format!(
        "pair class minimum sample count: {MIN_CLASS}; day held constant per pair"
    ));

    let mut all_pair_days: BTreeSet<i64> = BTreeSet::new();
    let mut report: BTreeMap<i64, Vec<String>> = BTreeMap::new();

    for mode in [2i64, 1i64] {
        let mut total = 0usize;
        let mut lock = 0usize;
        let mut zero = 0usize;
        let mut agg: BTreeMap<(i64, i64), [Vec<f64>; 2]> = BTreeMap::new();
        for r in &recs {
            if r[3] as i64 != mode {
                continue;
            }
            total += 1;
            let resid = r[1];
            if resid.abs() > LOCK_HZ {
                lock += 1;
                continue;
            }
            let s = r[7] as i64;
            let class = if s == FLOOR {
                0usize
            } else if s >= STRONG_MIN {
                1usize
            } else {
                continue;
            };
            if s == 0 {
                zero += 1;
            }
            let d = (r[0] / DAY_S).floor() as i64;
            let st = r[2] as i64;
            agg.entry((d, st))
                .or_insert_with(|| [Vec::new(), Vec::new()])[class]
                .push(resid);
        }

        let mut rows: Vec<(i64, i64, f64, usize, f64, usize, f64, bool)> = Vec::new();
        for (&(d, st), both) in &agg {
            let nf = both[0].len();
            let ns = both[1].len();
            if nf >= MIN_CLASS && ns >= MIN_CLASS {
                let rf = rms(&both[0]);
                let rs = rms(&both[1]);
                if rf.is_finite() && rs.is_finite() {
                    rows.push((d, st, rf, nf, rs, ns, rf - rs, false));
                }
            }
        }
        rows.sort_by_key(|r| (r.0, r.1));

        let n_class_floor: usize = agg.values().map(|b| b[0].len()).sum();
        let n_class_strong: usize = agg.values().map(|b| b[1].len()).sum();
        let floor_days: BTreeSet<i64> = agg
            .iter()
            .filter(|(_, b)| !b[0].is_empty())
            .map(|((d, _), _)| *d)
            .collect();
        let strong_days: BTreeSet<i64> = agg
            .iter()
            .filter(|(_, b)| !b[1].is_empty())
            .map(|((d, _), _)| *d)
            .collect();

        let day_set: BTreeSet<i64> = rows.iter().map(|r| r.0).collect();
        let st_set: BTreeSet<i64> = rows.iter().map(|r| r.1).collect();
        for d in &day_set {
            all_pair_days.insert(*d);
        }

        let geom_note = |d: i64| -> Option<bool> {
            if !geom_ok {
                return None;
            }
            let t = d as f64 * DAY_S;
            match (
                body_barycenter_position("galileo_daily", t, &eph),
                body_barycenter_position("earth", t, &eph),
            ) {
                (Some(p), Some(e)) => {
                    let el = ang(sub([0.0, 0.0, 0.0], e), sub(p, e));
                    if el.is_finite() {
                        Some(el < CONJ_ELONG_DEG)
                    } else {
                        None
                    }
                }
                _ => None,
            }
        };

        let mut sec = Vec::new();
        sec.push(String::new());
        sec.push(format!(
            "mode {mode} — same-day paired floor (-2560) vs strong (>= -1750)"
        ));
        sec.push(format!(
            "  samples: {total} incl lock, {lock} lock transitions, {zero} strength 0 (separated)"
        ));
        sec.push(format!(
            "  class samples after lock exclusion: floor {n_class_floor} ({} days), strong {n_class_strong} ({} days)",
            floor_days.len(),
            strong_days.len()
        ));
        sec.push(format!(
            "  paired (day, station) cells with both classes >= {MIN_CLASS}: {}  (stations {:?}, {} distinct days)",
            rows.len(),
            st_set.iter().collect::<Vec<_>>(),
            day_set.len()
        ));

        let rms_floor: Vec<f64> = rows.iter().map(|r| r.2).collect();
        let rms_strong: Vec<f64> = rows.iter().map(|r| r.4).collect();
        let diffs: Vec<f64> = rows.iter().map(|r| r.6).collect();
        let (pos, neg) = sign_stats(&diffs);
        let dl = day_set.iter().min().copied();
        let dh = day_set.iter().max().copied();
        match (dl, dh) {
            (Some(a), Some(b)) => sec.push(format!(
                "  span: {} .. {}",
                jd_date(a as f64 * DAY_S),
                jd_date(b as f64 * DAY_S)
            )),
            _ => {}
        }
        sec.push(format!(
            "  median cell floor RMS: {} Hz   median cell strong RMS: {} Hz",
            fmt_o(median(&rms_floor)),
            fmt_o(median(&rms_strong))
        ));
        sec.push(format!(
            "  median same-day diff (floor - strong): {} Hz   sign split: {} cells floor louder, {} cells strong louder",
            fmt_o(median(&diffs)),
            pos,
            neg
        ));

        let conj: Vec<f64> = rows
            .iter()
            .filter(|r| geom_note(r.0) == Some(true))
            .map(|r| r.6)
            .collect();
        if !conj.is_empty() {
            let (cp, cn) = sign_stats(&conj);
            sec.push(format!(
                "  conjunction subset (elong < 30 deg): {} cells, median diff {} Hz, sign {} / {}",
                conj.len(),
                fmt_o(median(&conj)),
                cp,
                cn
            ));
        }

        let mut by_st: BTreeMap<i64, Vec<f64>> = BTreeMap::new();
        for r in &rows {
            by_st.entry(r.1).or_default().push(r.6);
        }
        sec.push(
            "  per station: cells, distinct days, median diff, sign floor-louder/strong-louder"
                .to_string(),
        );
        for (st, v) in &by_st {
            let days: BTreeSet<i64> = rows.iter().filter(|r| r.1 == *st).map(|r| r.0).collect();
            let (p, n) = sign_stats(v);
            sec.push(format!(
                "    station {st}: {} cells, {} days, median diff {} Hz, sign {}/{}",
                v.len(),
                days.len(),
                fmt_o(median(v)),
                p,
                n
            ));
        }

        let mut day_floor: BTreeMap<i64, Vec<f64>> = BTreeMap::new();
        let mut day_strong: BTreeMap<i64, Vec<f64>> = BTreeMap::new();
        for r in &rows {
            if let Some(both) = agg.get(&(r.0, r.1)) {
                day_floor
                    .entry(r.0)
                    .or_default()
                    .extend(both[0].iter().copied());
                day_strong
                    .entry(r.0)
                    .or_default()
                    .extend(both[1].iter().copied());
            }
        }
        let day_diffs: Vec<f64> = day_floor
            .iter()
            .map(|(d, f)| rms(f) - rms(&day_strong[d]))
            .collect();
        if !day_diffs.is_empty() {
            let (dp, dn) = sign_stats(&day_diffs);
            sec.push("  day-level (paired-station samples pooled per day)".to_string());
            sec.push(format!(
                "    {} days, median same-day diff {} Hz, sign {} / {}",
                day_diffs.len(),
                fmt_o(median(&day_diffs)),
                dp,
                dn
            ));
        }

        sec.push("  paired cell table (date, station, floor n / RMS Hz, strong n / RMS Hz, diff Hz, conj?)".to_string());
        for (d, st, rf, nf, rs, ns, diff, _) in &rows {
            let cj = match geom_note(*d) {
                Some(true) => "conj",
                Some(false) => "-",
                None => "no-geom",
            };
            sec.push(format!(
                "    {} station {}  floor {nf}/{}  strong {ns}/{}  diff {}  {cj}",
                jd_date(*d as f64 * DAY_S),
                st,
                fmt_o(Some(*rf)),
                fmt_o(Some(*rs)),
                fmt_o(Some(*diff))
            ));
        }
        report.insert(mode, sec);
    }

    drop(recs);

    for (mode, sec) in &report {
        out.extend(sec.iter().cloned());
        if *mode == 2 && !report.contains_key(&1) {
            out.push(String::new());
            out.push("mode 1: no samples or no paired cells".to_string());
        }
    }

    std::fs::create_dir_all("reports").ok();
    let body = out.join("\n") + "\n";
    match std::fs::write(OUT, &body) {
        Ok(()) => eprintln!("galileo: same-day paired floor/strong report written to {OUT}"),
        Err(_) => eprintln!("galileo: write {OUT} void"),
    }
}
