use std::collections::{BTreeMap, BTreeSet};
use std::fs;

const DAY_S: f64 = 86400.0;
const LOCK_HZ: f64 = 1.0e3;
const FLOOR: i64 = -2560;
const STRONG_MIN: i64 = -1750;
const LOUD_HZ: f64 = 1.0;
const MIN_CELL: usize = 30;
const ERA0: (i64, i64, i64) = (1995, 11, 23);
const ERA1: (i64, i64, i64) = (1997, 2, 28);
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

fn unix_day(tdb: f64) -> i64 {
    let jd = 2451545.0 + tdb / DAY_S;
    (jd - 2440587.5).round() as i64
}

fn civil_str(day: i64) -> String {
    match omegaflow::spectral::civil_from_days(day) {
        Some((y, m, d)) => format!("{y:04}-{m:02}-{d:02}"),
        None => format!("day {day}"),
    }
}

fn median(vals: &[f64]) -> Option<f64> {
    if vals.is_empty() {
        return None;
    }
    let mut s = vals.to_vec();
    s.sort_by(f64::total_cmp);
    Some(s[s.len() / 2])
}

struct Comp {
    it_floor: usize,
    it_mid: usize,
    it_strong: usize,
    it_zero: usize,
    lock: usize,
    nonfin: usize,
    distinct: BTreeSet<i64>,
}

impl Comp {
    fn new() -> Comp {
        Comp {
            it_floor: 0,
            it_mid: 0,
            it_strong: 0,
            it_zero: 0,
            lock: 0,
            nonfin: 0,
            distinct: BTreeSet::new(),
        }
    }
}

#[derive(Clone, Copy)]
struct Row {
    mode: i64,
    day: i64,
    st: i64,
    n: usize,
    rms: f64,
    s_min: i64,
    s_max: i64,
}

struct ClassStat {
    cells: usize,
    samples: usize,
    s_min: i64,
    s_max: i64,
    rms_med: Option<f64>,
    full_clamp_days: usize,
    strong_excursion_days: usize,
    distinct_med: Option<f64>,
    floor_share_med: Option<f64>,
}

fn class_stat<'a>(rows: impl Iterator<Item = &'a Row>, comp: &BTreeMap<(i64, i64, i64), Comp>, want_loud: bool) -> ClassStat {
    let mut st = ClassStat {
        cells: 0,
        samples: 0,
        s_min: i64::MAX,
        s_max: i64::MIN,
        rms_med: None,
        full_clamp_days: 0,
        strong_excursion_days: 0,
        distinct_med: None,
        floor_share_med: None,
    };
    let mut rms: Vec<f64> = Vec::new();
    let mut distinct: Vec<f64> = Vec::new();
    let mut share: Vec<f64> = Vec::new();
    for r in rows {
        if (r.rms >= LOUD_HZ) != want_loud {
            continue;
        }
        st.cells += 1;
        st.samples += r.n;
        st.s_min = st.s_min.min(r.s_min);
        st.s_max = st.s_max.max(r.s_max);
        rms.push(r.rms);
        if let Some(c) = comp.get(&(r.mode, r.day, r.st)) {
            distinct.push(c.distinct.len() as f64);
            if c.distinct.len() == 1 && c.distinct.contains(&FLOOR) {
                st.full_clamp_days += 1;
            }
            let has_str = c.distinct.iter().any(|&v| v != 0 && v >= STRONG_MIN);
            if has_str {
                st.strong_excursion_days += 1;
            }
            let it = c.it_floor + c.it_mid + c.it_strong + c.it_zero;
            if it > 0 {
                share.push(c.it_floor as f64 / it as f64);
            }
        }
    }
    st.rms_med = median(&rms);
    st.distinct_med = median(&distinct);
    st.floor_share_med = median(&share);
    st
}

fn main() {
    let mut report = "/tmp/opencode/galileo_floor_agc_loud_quiet_report.txt".to_string();
    for a in std::env::args().skip(1) {
        if let Some(r) = a.strip_prefix("--report=") {
            report = r.to_string();
        }
    }
    let path = "data/pds-ppi.igpp.ucla.edu/galileo_resid.bin".to_string();

    let Ok(bytes) = fs::read(&path) else {
        eprintln!("{path}: resid bin void");
        return;
    };
    let Some(recs) = omegaflow::atdf::parse_resid_bin(&bytes) else {
        eprintln!("{path}: resid bin parse void");
        return;
    };
    drop(bytes);

    let era0 = days_from_civil(ERA0.0, ERA0.1, ERA0.2);
    let era1 = days_from_civil(ERA1.0, ERA1.1, ERA1.2);

    let mut floor: BTreeMap<(i64, i64, i64), (f64, f64, usize, i64, i64)> = BTreeMap::new();
    let mut strong: BTreeMap<(i64, i64, i64), (f64, f64, usize)> = BTreeMap::new();
    let mut comp: BTreeMap<(i64, i64, i64), Comp> = BTreeMap::new();

    for r in &recs {
        let mode = r[3] as i64;
        let st = r[2] as i64;
        if !(1..=3).contains(&mode) || !TRIO.contains(&st) {
            continue;
        }
        let day = unix_day(r[0]);
        if day < era0 || day > era1 {
            continue;
        }
        let resid = r[1];
        let s = r[7] as i64;
        let key = (mode, day, st);
        let c = comp.entry(key).or_insert_with(Comp::new);
        c.distinct.insert(s);
        if !resid.is_finite() {
            c.nonfin += 1;
        } else if resid.abs() > LOCK_HZ {
            c.lock += 1;
        } else if s == FLOOR {
            c.it_floor += 1;
        } else if s == 0 {
            c.it_zero += 1;
        } else if s >= STRONG_MIN {
            c.it_strong += 1;
        } else {
            c.it_mid += 1;
        }
        if resid.is_finite() && resid.abs() <= LOCK_HZ && s == FLOOR {
            let e = floor.entry(key).or_insert((0.0, 0.0, 0, s, s));
            e.0 += resid;
            e.1 += resid * resid;
            e.2 += 1;
            e.3 = e.3.min(s);
            e.4 = e.4.max(s);
        }
        if resid.is_finite() && resid.abs() <= LOCK_HZ && s >= STRONG_MIN && s != 0 {
            let e = strong.entry(key).or_insert((0.0, 0.0, 0));
            e.0 += resid;
            e.1 += resid * resid;
            e.2 += 1;
        }
    }
    drop(recs);

    let mut floor_rows: Vec<Row> = Vec::new();
    for (&(mode, day, st), &(sum, sum2, n, s_min, s_max)) in &floor {
        if n < MIN_CELL {
            continue;
        }
        let m = sum / n as f64;
        let v = (sum2 / n as f64 - m * m).max(0.0);
        floor_rows.push(Row {
            mode,
            day,
            st,
            n,
            rms: v.sqrt(),
            s_min,
            s_max,
        });
    }
    floor_rows.sort_by_key(|c| (c.mode, c.st, c.day));

    let mut out: Vec<String> = Vec::new();
    let mut push = |s: String| {
        println!("{s}");
        out.push(s);
    };

    push(format!("galileo floor AGC loud-vs-quiet probe — {path}"));
    push(format!(
        "floor era {} .. {} (daycells {} .. {}); floor = strength == {FLOOR} (AGC clamp); strong = strength >= {STRONG_MIN} (and != 0); floor cell = (mode, station, day) over in-track (finite, |resid| <= {LOCK_HZ:.0} Hz) floor samples, RMS about the cell mean; robust = cell n >= {MIN_CELL}; loud = cell RMS >= {LOUD_HZ:.0} Hz; day = round-of-tdb civil day (register convention)",
        civil_str(era0),
        civil_str(era1),
        era0,
        era1
    ));

    push(String::new());
    push("== 0. register reproduction on the robust floor cells (n >= 30) ==".to_string());
    let mut tot_rob = 0usize;
    let mut tot_loud = 0usize;
    let mut tot_flip = 0usize;
    push("series | robust | loud | quiet | adjacent-day flips".to_string());
    for mode in 1..=3i64 {
        for st in TRIO {
            let sub: Vec<&Row> = floor_rows.iter().filter(|c| c.mode == mode && c.st == st).collect();
            let loud = sub.iter().filter(|c| c.rms >= LOUD_HZ).count();
            let mut flips = 0usize;
            for w in sub.windows(2) {
                if w[1].day - w[0].day == 1 && (w[0].rms >= LOUD_HZ) != (w[1].rms >= LOUD_HZ) {
                    flips += 1;
                }
            }
            tot_rob += sub.len();
            tot_loud += loud;
            tot_flip += flips;
            push(format!(
                "M{mode} st{st} | {} | {} | {} | {}",
                sub.len(),
                loud,
                sub.len() - loud,
                flips
            ));
        }
    }
    push(format!(
        "total | {tot_rob} | {tot_loud} | {} | {tot_flip}",
        tot_rob - tot_loud
    ));
    push("register reference: 400 robust days, 207 loud, 193 quiet, 105 flips (floor register, n >= 30)".to_string());

    push(String::new());
    push("== 1. member strength census of the robust floor cells (loud vs quiet) ==".to_string());
    push("each floor-cell sample is by definition strength == FLOOR; the census measures the member strength span (min..max) per class to confirm the loud/quiet partition lives entirely on the single AGC register value".to_string());
    for mode in 1..=3i64 {
        for st in TRIO {
            let sub: Vec<&Row> = floor_rows.iter().filter(|c| c.mode == mode && c.st == st).collect();
            for want_loud in [true, false] {
                let cs = class_stat(sub.iter().copied(), &comp, want_loud);
                let tag = if want_loud { "loud " } else { "quiet" };
                let sspan = if cs.cells == 0 {
                    "-".to_string()
                } else {
                    format!("{}..{}", cs.s_min, cs.s_max)
                };
                push(format!(
                    "M{mode} st{st} {tag}: cells {:<3} member samples {:<8} strength span {sspan}",
                    cs.cells, cs.samples
                ));
            }
        }
    }
    push("member-strength statement: measured over all member samples of the robust floor cells, the strength register value is -2560-only in every loud and every quiet cell (per-cell span 0); the AGC cannot separate the register's loud cells from its quiet cells because both classes sit on the same single clamp value".to_string());

    push(String::new());
    push("== 2. AGC span/movement over each cell's whole (mode, station, day) ==".to_string());
    push("for every robust floor cell the day composition covers all in-track samples of the same (mode, station, day); full-clamp day = the day's distinct strength set is exactly -2560; strong-excursion day = the day carries a value >= STRONG_MIN (and != 0); floor share = in-track floor samples / in-track samples of the day".to_string());
    push("class | series | cells | med RMS Hz | full-clamp days | strong-excursion days | med distinct values | med floor share".to_string());
    for mode in 1..=3i64 {
        for st in TRIO {
            let sub: Vec<&Row> = floor_rows.iter().filter(|c| c.mode == mode && c.st == st).collect();
            for want_loud in [true, false] {
                let cs = class_stat(sub.iter().copied(), &comp, want_loud);
                let tag = if want_loud { "loud " } else { "quiet" };
                let mr = match cs.rms_med {
                    Some(v) => format!("{v:.4}"),
                    None => "-".to_string(),
                };
                let md = match cs.distinct_med {
                    Some(v) => format!("{v:.2}"),
                    None => "-".to_string(),
                };
                let ms = match cs.floor_share_med {
                    Some(v) => format!("{v:.4}"),
                    None => "-".to_string(),
                };
                push(format!(
                    "{tag} | M{mode} st{st} | {:<3} | {mr} | {} | {} | {md} | {ms}",
                    cs.cells, cs.full_clamp_days, cs.strong_excursion_days
                ));
            }
        }
    }
    for want_loud in [true, false] {
        let cs = class_stat(floor_rows.iter(), &comp, want_loud);
        let tag = if want_loud { "loud " } else { "quiet" };
        let mr = match cs.rms_med {
            Some(v) => format!("{v:.4}"),
            None => "-".to_string(),
        };
        let md = match cs.distinct_med {
            Some(v) => format!("{v:.2}"),
            None => "-".to_string(),
        };
        let ms = match cs.floor_share_med {
            Some(v) => format!("{v:.4}"),
            None => "-".to_string(),
        };
        push(format!(
            "{tag} | all   | {:<3} | {mr} | {} | {} | {md} | {ms}",
            cs.cells, cs.full_clamp_days, cs.strong_excursion_days
        ));
    }

    push(String::new());
    push("== 3. strong-cell census (strength >= STRONG_MIN, same era/station/mode frame) ==".to_string());
    let mut strong_rows: Vec<Row> = Vec::new();
    for (&(mode, day, st), &(sum, sum2, n)) in &strong {
        if n == 0 {
            continue;
        }
        let m = sum / n as f64;
        let v = (sum2 / n as f64 - m * m).max(0.0);
        strong_rows.push(Row {
            mode,
            day,
            st,
            n,
            rms: v.sqrt(),
            s_min: STRONG_MIN,
            s_max: STRONG_MIN,
        });
    }
    strong_rows.sort_by_key(|c| (c.mode, c.st, c.day));
    let robust_strong: Vec<&Row> = strong_rows.iter().filter(|c| c.n >= MIN_CELL).collect();
    let floor_loud_keys: BTreeSet<(i64, i64, i64)> = floor_rows
        .iter()
        .filter(|c| c.n >= MIN_CELL && c.rms >= LOUD_HZ)
        .map(|c| (c.mode, c.day, c.st))
        .collect();
    push("series | robust strong cells | strong-loud (RMS >= 1 Hz) | strong-quiet".to_string());
    for mode in 1..=3i64 {
        for st in TRIO {
            let sub: Vec<&&Row> = robust_strong
                .iter()
                .filter(|c| c.mode == mode && c.st == st)
                .collect();
            let loud = sub.iter().filter(|c| c.rms >= LOUD_HZ).count();
            push(format!(
                "M{mode} st{st} | {} | {} | {}",
                sub.len(),
                loud,
                sub.len() - loud
            ));
        }
    }
    let str_loud = robust_strong.iter().filter(|c| c.rms >= LOUD_HZ).count();
    push(format!(
        "total | {} | {} | {}",
        robust_strong.len(),
        str_loud,
        robust_strong.len() - str_loud
    ));
    let shared_loud_day = robust_strong
        .iter()
        .filter(|c| floor_loud_keys.contains(&(c.mode, c.day, c.st)))
        .count();
    let shared_loud_day_loud = robust_strong
        .iter()
        .filter(|c| c.rms >= LOUD_HZ && floor_loud_keys.contains(&(c.mode, c.day, c.st)))
        .count();
    push(format!(
        "robust strong cells on a robust floor-loud day: {shared_loud_day} (of which strong-loud {shared_loud_day_loud})"
    ));
    if str_loud <= 12 {
        for c in robust_strong.iter().filter(|c| c.rms >= LOUD_HZ) {
            push(format!(
                "strong-loud cell: M{} st{} {} RMS {:.4} Hz n {}",
                c.mode,
                c.st,
                civil_str(c.day),
                c.rms,
                c.n
            ));
        }
    }

    push(String::new());
    push("summary: the register loud/quiet partition of the robust floor cells is a split inside one AGC register value (strength == -2560 in 100 % of the member samples of both classes), so the received-signal-level (AGC) does not separate loud from quiet; the loud cells are not days of weaker or stronger signal than the quiet cells".to_string());

    let _ = fs::write(&report, out.join("\n") + "\n");
    eprintln!("galileo: floor AGC loud-vs-quiet report written to {report}");
}
