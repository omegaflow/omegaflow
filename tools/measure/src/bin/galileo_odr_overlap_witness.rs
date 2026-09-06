use std::collections::{BTreeMap, BTreeSet};
use std::fs;

use omegaflow::atdf::parse_resid_bin;
use omegaflow::spectral::civil_from_days;

const DAY_S: f64 = 86400.0;
const LOCK_HZ: f64 = 1.0e3;
const FLOOR: i64 = -2560;
const STRONG_MIN: i64 = -1750;
const LOUD_HZ: f64 = 1.0;
const J2000_UNIX_D: i64 = 10958;
const TRIO: [i64; 3] = [14, 43, 63];

fn days_from_civil(y: i64, m: i64, d: i64) -> i64 {
    let yy = if m <= 2 { y - 1 } else { y };
    let era = if yy >= 0 { yy } else { yy - 399 } / 400;
    let yoe = yy - era * 400;
    let mp = (m + 9) % 12;
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe - 719468
}

fn day_label(day: i64) -> String {
    match civil_from_days(day + J2000_UNIX_D) {
        Some((y, m, d)) => format!("{y:04}-{m:02}-{d:02}"),
        None => "-".to_string(),
    }
}

fn opt_day_label(day: Option<i64>) -> String {
    match day {
        Some(d) => day_label(d),
        None => "-".to_string(),
    }
}

struct Time {
    y: i64,
    mo: i64,
    d: i64,
    h: i64,
    mi: i64,
    s: i64,
}

impl Time {
    fn parse(s: &str) -> Option<Time> {
        let b = s.as_bytes();
        if b.len() < 19 || b[4] != b'-' || b[7] != b'-' || b[10] != b'T' {
            return None;
        }
        let num = |a: usize, bb: usize| s[a..bb].parse::<i64>().ok();
        let y = num(0, 4)?;
        let mo = num(5, 7)?;
        let d = num(8, 10)?;
        let h = num(11, 13)?;
        let mi = num(14, 16)?;
        let sec = num(17, 19)?;
        Some(Time { y, mo, d, h, mi, s: sec })
    }

    fn daycount(&self) -> i64 {
        days_from_civil(self.y, self.mo, self.d)
    }

    fn epoch(&self) -> i64 {
        self.daycount() * DAY_S as i64 + self.h * 3600 + self.mi * 60 + self.s
    }
}

fn fmt_time(t: &Time) -> String {
    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
        t.y, t.mo, t.d, t.h, t.mi, t.s
    )
}

struct Seg {
    start: Time,
    stop: Time,
}

fn parse_index(path: &str) -> Option<Vec<Seg>> {
    let text = fs::read_to_string(path).ok()?;
    let mut segs = Vec::new();
    for line in text.lines() {
        let t = line.trim();
        if t.is_empty() {
            continue;
        }
        let f: Vec<&str> = t.split(',').map(|x| x.trim_matches('"')).collect();
        if f.len() < 5 || !f[2].ends_with(".ODR") {
            continue;
        }
        let (Some(st), Some(sp)) = (Time::parse(f[3]), Time::parse(f[4])) else {
            continue;
        };
        if sp.epoch() < st.epoch() {
            continue;
        }
        segs.push(Seg { start: st, stop: sp });
    }
    Some(segs)
}

fn floor_rms(sum: f64, sum2: f64, n: usize) -> f64 {
    let m = sum / n as f64;
    let v = (sum2 / n as f64 - m * m).max(0.0);
    v.sqrt()
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let index_path = match args.first() {
        Some(a) => a.clone(),
        None => "/tmp/opencode/odr_sample/INDEX.TAB".to_string(),
    };
    let report_path = match args.get(1) {
        Some(a) => a.clone(),
        None => "/tmp/opencode/galileo_odr_overlap_witness_report.txt".to_string(),
    };

    let mut out: Vec<String> = Vec::new();
    let mut push = |s: String| {
        println!("{s}");
        out.push(s);
    };

    push("galileo GWE-ODR vs closed-loop floor overlap witness".to_string());
    let Some(segs) = parse_index(&index_path) else {
        push(format!("index void at {index_path}"));
        let _ = fs::write(&report_path, out.join("\n") + "\n");
        return;
    };
    if segs.is_empty() {
        push(format!("index at {index_path}: no usable segments"));
        let _ = fs::write(&report_path, out.join("\n") + "\n");
        return;
    }
    let mut covered: BTreeSet<i64> = BTreeSet::new();
    for sg in &segs {
        let mut d = sg.start.daycount();
        while d <= sg.stop.daycount() {
            covered.insert(d);
            d += 1;
        }
    }
    let first_t = segs.iter().map(|s| &s.start).min_by_key(|t| t.epoch());
    let last_t = segs.iter().map(|s| &s.stop).max_by_key(|t| t.epoch());
    let first_s = match first_t {
        Some(t) => fmt_time(t),
        None => "-".to_string(),
    };
    let last_s = match last_t {
        Some(t) => fmt_time(t),
        None => "-".to_string(),
    };
    push(format!(
        "odr INDEX (GO-X-RSS-1-ODR-V1.0): {} segments, coverage {} .. {}, {} distinct UTC days",
        segs.len(),
        first_s,
        last_s,
        covered.len()
    ));

    let win0 = days_from_civil(1994, 4, 28);
    let win1 = days_from_civil(1995, 6, 28);
    let floor_era_start = days_from_civil(1995, 11, 23);
    let odr_days_in_win = covered
        .iter()
        .filter(|d| **d >= win0 && **d <= win1)
        .count();
    let odr_days_in_floor_era = covered.iter().filter(|d| **d >= floor_era_start).count();
    push(format!(
        "odr UTC days inside [1994-04-28..1995-06-28]: {odr_days_in_win}; odr UTC days on/after floor-era start 1995-11-23: {odr_days_in_floor_era}"
    ));

    let Ok(bytes) = fs::read("data/galileo_resid.bin") else {
        push("galileo resid bin void".to_string());
        let _ = fs::write(&report_path, out.join("\n") + "\n");
        return;
    };
    let Some(recs) = parse_resid_bin(&bytes) else {
        push("galileo resid bin parse void".to_string());
        let _ = fs::write(&report_path, out.join("\n") + "\n");
        return;
    };
    drop(bytes);

    let mut floor_cell: BTreeMap<(i64, i64, i64), (f64, f64, usize, f64)> = BTreeMap::new();
    let mut win_activity: BTreeMap<i64, usize> = BTreeMap::new();
    let mut n_mode = 0usize;
    let mut n_lock = 0usize;
    let mut n_zero = 0usize;
    let mut n_uncl = 0usize;
    let mut n_inf = 0usize;
    let mut t0_all = f64::INFINITY;
    let mut t1_all = f64::NEG_INFINITY;
    for r in &recs {
        let tdb = r[0];
        t0_all = t0_all.min(tdb);
        t1_all = t1_all.max(tdb);
        let mode = r[3] as i64;
        if mode < 1 || mode > 3 {
            n_mode += 1;
            continue;
        }
        let resid = r[1];
        if !resid.is_finite() {
            n_inf += 1;
            continue;
        }
        if resid.abs() > LOCK_HZ {
            n_lock += 1;
            continue;
        }
        let s = r[7] as i64;
        if s == 0 {
            n_zero += 1;
            continue;
        }
        let is_floor = s == FLOOR;
        if !is_floor && s < STRONG_MIN {
            n_uncl += 1;
            continue;
        }
        let st = r[2] as i64;
        let day = (tdb / DAY_S).floor() as i64;
        let civil = day + J2000_UNIX_D;
        if civil >= win0 && civil <= win1 {
            let e = win_activity.entry(st).or_insert(0);
            *e += 1;
        }
        if is_floor {
            let e = floor_cell
                .entry((mode, st, day))
                .or_insert_with(|| (0.0, 0.0, 0, tdb));
            e.0 += resid;
            e.1 += resid * resid;
            e.2 += 1;
        }
    }
    drop(recs);

    let mut cells: Vec<(i64, i64, i64, usize, f64)> = floor_cell
        .iter()
        .map(|(&(m, st, d), &(su, su2, n, _))| (m, st, d, n, floor_rms(su, su2, n)))
        .collect();
    cells.sort_by_key(|c| (c.0, c.2, c.1));

    push(format!(
        "resid span {} .. {}; modes 1..=3; lock (|resid| > {:.0} Hz) excluded; excluded: lock {n_lock}, mode-out-of-range {n_mode}, strength-0 {n_zero}, non-finite {n_inf}, unclassed {n_uncl}",
        day_label((t0_all / DAY_S).floor() as i64),
        day_label((t1_all / DAY_S).floor() as i64),
        LOCK_HZ
    ));
    push(format!(
        "floor day cells (mode, station, day) across the whole record, any n: {}",
        cells.len()
    ));
    push(format!(
        "earliest floor cell day (any station): {}",
        opt_day_label(cells.iter().map(|c| c.2).min())
    ));

    let win_cells: Vec<&(i64, i64, i64, usize, f64)> = cells
        .iter()
        .filter(|c| c.2 + J2000_UNIX_D >= win0 && c.2 + J2000_UNIX_D <= win1)
        .collect();
    push(format!(
        "floor day cells with day inside ODR window [1994-04-28..1995-06-28]: {}",
        win_cells.len()
    ));
    for c in &win_cells {
        let tag = if TRIO.contains(&c.1) {
            ""
        } else {
            " (non-trio station)"
        };
        push(format!(
            "  mode {} st{} day {} n {} rms {:.4} Hz{}",
            c.0,
            c.1,
            day_label(c.2),
            c.3,
            c.4,
            tag
        ));
    }

    let trio_days: BTreeSet<i64> = cells
        .iter()
        .filter(|c| TRIO.contains(&c.1))
        .map(|c| c.2)
        .collect();
    let et = trio_days.iter().min().copied();
    push(format!(
        "earliest trio (14/43/63) floor day: {}",
        opt_day_label(et)
    ));
    let last_day = match last_t {
        Some(t) => t.daycount(),
        None => 0,
    };
    match et {
        Some(d) => {
            let gap = d + J2000_UNIX_D - last_day;
            push(format!("calendar gap from last ODR day to earliest trio floor day: {gap} days"));
        }
        None => push("trio floor days: none".to_string()),
    }

    let trio_cells: Vec<&(i64, i64, i64, usize, f64)> =
        cells.iter().filter(|c| TRIO.contains(&c.1)).collect();
    let robust_trio: Vec<&(i64, i64, i64, usize, f64)> =
        trio_cells.iter().copied().filter(|c| c.3 >= 30).collect();
    push(format!(
        "trio floor cells: {} total (any n), {} robust (n >= 30)",
        trio_cells.len(),
        robust_trio.len()
    ));

    push(String::new());
    push("robust trio floor census (n >= 30): cells and loud (rms >= 1 Hz) per (mode, station)".to_string());
    let mut sum_robust = 0usize;
    let mut sum_loud = 0usize;
    for mode in 1..=3 {
        for st in TRIO {
            let sub: Vec<&(i64, i64, i64, usize, f64)> =
                robust_trio.iter().filter(|c| c.0 == mode && c.1 == st).copied().collect();
            let loud = sub.iter().filter(|c| c.4 >= LOUD_HZ).count();
            sum_robust += sub.len();
            sum_loud += loud;
            push(format!("  mode {mode} st{st}: {} robust cells, {loud} loud", sub.len()));
        }
    }
    push(format!(
        "trio robust floor total: {sum_robust} cells, {sum_loud} loud (closed-loop floor register basis)"
    ));

    push(String::new());
    push("closed-loop resid samples inside ODR window per station (modes 1..=3, locked excluded):".to_string());
    let mut act: Vec<(i64, usize)> = win_activity.iter().map(|(k, v)| (*k, *v)).collect();
    act.sort();
    let mut trio_act = 0usize;
    for (st, n) in &act {
        if TRIO.contains(st) {
            trio_act += n;
        }
        push(format!("  st{st}: {n} samples"));
    }
    push(format!("  on ODR stations 14/43/63: {trio_act} samples"));

    let _ = fs::write(&report_path, out.join("\n") + "\n");
    eprintln!("galileo: ODR-overlap witness report written to {report_path}");
}
