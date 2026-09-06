use std::collections::BTreeMap;
use std::fs;

use omegaflow::atdf::parse_resid_bin;

const DAY_S: f64 = 86400.0;
const LOCK_HZ: f64 = 1.0e3;
const FLOOR_AGC: f64 = -2560.0;
const LOUD_HZ: f64 = 1.0;

fn unix_day_float(tdb: f64) -> f64 {
    tdb / DAY_S + 10957.5
}

fn day_key(tdb: f64) -> i64 {
    unix_day_float(tdb).round() as i64
}

fn hour_of_day(tdb: f64) -> f64 {
    unix_day_float(tdb).rem_euclid(1.0) * 24.0
}

fn days_from_civil(y: i64, m: i64, d: i64) -> i64 {
    let yy = if m <= 2 { y - 1 } else { y };
    let era = if yy >= 0 { yy } else { yy - 399 } / 400;
    let yoe = yy - era * 400;
    let mp = (m + 9) % 12;
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe - 719468
}

struct OdrWin {
    label: String,
    day: i64,
    station: i64,
    h0: f64,
    h1: f64,
}

fn parse_hhmm(s: &str) -> Option<f64> {
    let b: Vec<&str> = s.split(':').collect();
    let h: f64 = b.first()?.parse().ok()?;
    let m: f64 = b.get(1)?.parse().ok()?;
    Some(h + m / 60.0)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some(seg_path) = args.first() else {
        eprintln!("galileo ODR overlap probe: <segments table> <report path>");
        return;
    };
    let Some(report_path) = args.get(1) else {
        eprintln!("galileo ODR overlap probe: <segments table> <report path>");
        return;
    };

    let mut out: Vec<String> = Vec::new();
    let mut push = |s: String| {
        println!("{s}");
        out.push(s);
    };

    push("galileo GO-J/GO-JS ODR windows vs closed-loop floor register cells (canonical round day-key)".to_string());
    let seg = match fs::read_to_string(seg_path) {
        Ok(t) => t,
        Err(_) => {
            push(format!("segment table void at {seg_path}"));
            let _ = fs::write(&report_path, out.join("\n") + "\n");
            return;
        }
    };
    let mut wins: Vec<OdrWin> = Vec::new();
    for line in seg.lines() {
        let f: Vec<&str> = line.split_whitespace().collect();
        if f.len() < 5 {
            continue;
        }
        let ymd: Vec<i64> = f[1].split('-').filter_map(|x| x.parse().ok()).collect();
        if ymd.len() != 3 {
            continue;
        }
        let (y, mo, d) = (ymd[0], ymd[1], ymd[2]);
        let Ok(station) = f[2].parse::<i64>() else { continue };
        let (Some(h0), Some(h1)) = (parse_hhmm(f[3]), parse_hhmm(f[4])) else {
            continue;
        };
        wins.push(OdrWin { label: format!("{} {}", f[0], f[1]), day: days_from_civil(y, mo, d), station, h0, h1 });
    }
    wins.sort_by_key(|w| (w.day, w.station));

    let Ok(bytes) = fs::read("data/pds-ppi.igpp.ucla.edu/galileo_resid.bin") else {
        push("galileo resid bin void".to_string());
        let _ = fs::write(&report_path, out.join("\n") + "\n");
        return;
    };
    let Some(recs) = parse_resid_bin(&bytes) else {
        push("resid parse void".to_string());
        return;
    };
    drop(bytes);

    let mut cell: BTreeMap<(i64, i64, i64), (f64, f64, usize)> = BTreeMap::new();
    let mut cell_inwin: BTreeMap<(i64, i64, i64, u32, u32), (f64, f64, usize)> = BTreeMap::new();
    let mut hours: BTreeMap<(i64, i64, i64), Vec<f64>> = BTreeMap::new();
    for r in &recs {
        let tdb = r[0];
        let mode = r[3] as i64;
        if !(1..=3).contains(&mode) {
            continue;
        }
        let resid = r[1];
        if !resid.is_finite() || resid.abs() > LOCK_HZ {
            continue;
        }
        if r[7] != FLOOR_AGC {
            continue;
        }
        let st = r[2] as i64;
        if st != 14 && st != 43 && st != 63 {
            continue;
        }
        let day = day_key(tdb);
        let h = hour_of_day(tdb);
        let e = cell.entry((mode, st, day)).or_insert((0.0, 0.0, 0));
        e.0 += resid;
        e.1 += resid * resid;
        e.2 += 1;
        hours.entry((mode, st, day)).or_default().push(h);
        for w in &wins {
            if w.day == day && w.station == st && h >= w.h0 - 1e-6 && h <= w.h1 + 1e-6 {
                let k = (mode, st, day, (w.h0 * 100.0) as u32, (w.h1 * 100.0) as u32);
                let iw = cell_inwin.entry(k).or_insert((0.0, 0.0, 0));
                iw.0 += resid;
                iw.1 += resid * resid;
                iw.2 += 1;
            }
        }
    }

    push(String::new());
    push("odr-window | register floor cell (mode, day, station) | class | in-window floor samples".to_string());
    for w in &wins {
        for mode in 1..=3 {
            let key = (mode, w.station, w.day);
            let Some(&(su, su2, n)) = cell.get(&key) else {
                continue;
            };
            let rms = {
                let m = su / n as f64;
                ((su2 / n as f64 - m * m).max(0.0)).sqrt()
            };
            let class = if n >= 30 && rms >= LOUD_HZ { "LOUD" } else if n >= 30 { "quiet" } else { "thin" };
            let Some(hs) = hours.get(&key) else {
                continue;
            };
            let in_win = hs.iter().filter(|h| **h >= w.h0 - 1e-6 && **h <= w.h1 + 1e-6).count();
            let lo = hs.iter().cloned().fold(f64::INFINITY, f64::min);
            let hi = hs.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            let date = match omegaflow::spectral::civil_from_days(w.day) {
                Some((y, m, d)) => format!("{y:04}-{m:02}-{d:02}"),
                None => "-".to_string(),
            };
            let span = if hs.is_empty() { "-".into() } else { format!("{lo:.1}-{hi:.1}") };
            let iw_key = (mode, w.station, w.day, (w.h0 * 100.0) as u32, (w.h1 * 100.0) as u32);
            let iwr = match cell_inwin.get(&iw_key) {
                Some(&(su, su2, ni)) => {
                    let m = su / ni as f64;
                    format!("{:.3}", ((su2 / ni as f64 - m * m).max(0.0)).sqrt())
                }
                None => "-".to_string(),
            };
            push(format!(
                "{:>11} st{} m{mode} day n {:>6} rms {:>9.3} Hz {:>5} | floor-span {}h | in-window n {:>6} rms {} Hz | {}",
                w.label, w.station, n, rms, class, span, in_win, iwr, date
            ));
        }
    }
    let _ = fs::write(&report_path, out.join("\n") + "\n");
    eprintln!("report written to {report_path}");
}
