use std::collections::BTreeMap;

use omegaflow::archivar::spectral::civil_from_days;
use omegaflow::odf::parse_p11r_bin;

const DAY_S: f64 = 86400.0;

fn jd_date(tdb: f64) -> String {
    let jd = tdb / DAY_S + 2451545.0;
    let unix_day = (jd - 2440587.5).round() as i64;
    match civil_from_days(unix_day) {
        Some((y, m, d)) => format!("{y:04}-{m:02}-{d:02}"),
        None => format!("tdb {tdb:.0} s"),
    }
}

fn year_of(tdb: f64) -> i32 {
    let jd = tdb / DAY_S + 2451545.0;
    let unix_day = (jd - 2440587.5).round() as i64;
    match civil_from_days(unix_day) {
        Some((y, _, _)) => y as i32,
        None => -1,
    }
}

fn daily_median_by_station(recs: &[[f64; 9]]) -> BTreeMap<i64, BTreeMap<i64, (f64, usize)>> {
    let mut out: BTreeMap<i64, BTreeMap<i64, Vec<f64>>> = BTreeMap::new();
    for r in recs {
        let day = (r[0] / DAY_S).floor() as i64;
        let rx = r[5] as i64;
        let resid = r[1];
        if resid.is_finite() && resid.abs() < 5.0e5 {
            out.entry(day)
                .or_default()
                .entry(rx)
                .or_default()
                .push(resid);
        }
    }
    let mut res = BTreeMap::new();
    for (day, by_rx) in out {
        let mut m = BTreeMap::new();
        for (rx, mut v) in by_rx {
            v.sort_by(f64::total_cmp);
            let med = v[v.len() / 2];
            m.insert(rx, (med, v.len()));
        }
        res.insert(day, m);
    }
    res
}

fn era_report(name: &str, recs: &[[f64; 9]], years: &[i32], window: i64) {
    let by_day = daily_median_by_station(recs);
    let mut focus = BTreeMap::new();
    for (day, m) in &by_day {
        let t = *day as f64 * DAY_S;
        if years.contains(&year_of(t)) {
            focus.insert(*day, m);
        }
    }
    for (day, m) in &focus {
        let t = *day as f64 * DAY_S;
        let mut line: Vec<String> = Vec::new();
        let mut samples = 0usize;
        for (rx, (med, n)) in m.iter() {
            line.push(format!("{rx}:{med:.1}Hz({n})"));
            samples += n;
        }
        let dmed: Vec<f64> = m.values().map(|v| v.0).collect();
        let spread = if dmed.len() >= 2 {
            let lo = dmed.iter().cloned().fold(f64::INFINITY, f64::min);
            let hi = dmed.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            hi - lo
        } else {
            0.0
        };
        let pre = if by_day.contains_key(&(day - window)) {
            let p = &by_day[&(day - window)];
            let v: Vec<f64> = p.values().map(|x| x.0).collect();
            Some(median(&v))
        } else {
            None
        };
        let post = if by_day.contains_key(&(day + window)) {
            let p = &by_day[&(day + window)];
            let v: Vec<f64> = p.values().map(|x| x.0).collect();
            Some(median(&v))
        } else {
            None
        };
        let ctx = match (pre, post) {
            (Some(a), Some(b)) => format!("pre {a:.1} Hz .. post {b:.1} Hz (step {:.1} Hz)", b - a),
            (Some(a), None) => format!("pre {a:.1} Hz, no +{window}-d follow"),
            (None, Some(b)) => format!("no -{window}-d, post {b:.1} Hz"),
            _ => String::from("no context"),
        };
        eprintln!(
            "{name} {date} [{samples} samp, station spread {spread:.1} Hz] {ctx} — stations: {line}",
            name = name,
            date = jd_date(t),
            ctx = ctx,
            line = line.join(" | ")
        );
    }
}

fn median(v: &[f64]) -> f64 {
    let mut s = v.to_vec();
    s.sort_by(f64::total_cmp);
    s[s.len() / 2]
}

fn run(name: &str, years: &[i32]) {
    let path = format!("data/{name}_navio_residuum.bin");
    let Ok(bytes) = std::fs::read(&path) else {
        eprintln!("{name}: residuum bin void ({path})");
        return;
    };
    let Some(recs) = parse_p11r_bin(&bytes) else {
        eprintln!("{name}: residuum parse void");
        return;
    };
    let n_stations: usize = {
        let mut s: Vec<i64> = recs.iter().map(|r| r[5] as i64).collect();
        s.sort_unstable();
        s.dedup();
        s.len()
    };
    eprintln!(
        "{name}: {} two-way residual samples, {} stations — era report over {:?}",
        recs.len(),
        n_stations,
        years
    );
    era_report(name, &recs, years, 1);
    era_report(name, &recs, years, 7);
}

fn main() {
    run("pioneer10", &[1996]);
    run("pioneer11", &[1981, 1982]);
}
