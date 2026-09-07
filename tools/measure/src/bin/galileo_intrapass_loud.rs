use std::fs;

use omegaflow::lsk::days_from_civil;
use omegaflow::spectral::civil_from_days;

const DAY_S: f64 = 86400.0;
const LOCK_HZ: f64 = 1.0e3;
const FLOOR_AGC: i64 = -2560;
const LOUD_HZ: f64 = 1.0;
const MIN_CELL: usize = 30;
const RUN_GAP_S: f64 = 600.0;
const N_WIN: usize = 12;

fn resid_path() -> String {
    "data/pds-ppi.igpp.ucla.edu/galileo_resid.bin".to_string()
}

fn unix_day(tdb: f64) -> i64 {
    let jd = 2451545.0 + tdb / DAY_S;
    (jd - 2440587.5).round() as i64
}

fn civil(day: i64) -> String {
    match civil_from_days(day) {
        Some((y, m, d)) => format!("{y:04}-{m:02}-{d:02}"),
        None => format!("day {day}"),
    }
}

fn fmt_utc(tdb: f64) -> String {
    let unix = tdb + 10957.5 * DAY_S;
    let rem = unix.rem_euclid(DAY_S);
    let h = (rem / 3600.0) as i64;
    let m = ((rem % 3600.0) / 60.0) as i64;
    format!("{h:02}:{m:02}")
}

#[derive(Clone, Copy)]
struct Anchor {
    mode: i64,
    station: i64,
    day: i64,
    name: &'static str,
}

fn anchors() -> Vec<Anchor> {
    vec![
        Anchor { mode: 1, station: 14, day: days_from_civil(1995, 11, 24).unwrap(), name: "M1 st14 1995-11-24 (25.85 Hz)" },
        Anchor { mode: 2, station: 14, day: days_from_civil(1995, 11, 24).unwrap(), name: "M2 st14 1995-11-24 (31.77 Hz)" },
        Anchor { mode: 1, station: 63, day: days_from_civil(1996, 6, 26).unwrap(), name: "M1 st63 1996-06-26 (20.64 Hz)" },
        Anchor { mode: 3, station: 43, day: days_from_civil(1995, 12, 4).unwrap(), name: "M3 st43 1995-12-04 (10.52 Hz)" },
        Anchor { mode: 3, station: 14, day: days_from_civil(1995, 12, 5).unwrap(), name: "M3 st14 1995-12-05 (52.92 Hz)" },
        Anchor { mode: 3, station: 63, day: days_from_civil(1995, 11, 27).unwrap(), name: "M3 st63 1995-11-27 (186.5 Hz)" },
        Anchor { mode: 1, station: 43, day: days_from_civil(1996, 11, 4).unwrap(), name: "M1 st43 1996-11-04 (23.1 Hz)" },
    ]
}

fn load_all() -> Option<Vec<(f64, f64, i64, i64)>> {
    let bytes = fs::read(resid_path()).ok()?;
    let recs = omegaflow::atdf::parse_resid_bin(&bytes)?;
    let mut outv: Vec<(f64, f64, i64, i64)> = Vec::new();
    for r in &recs {
        let resid = r[1];
        if !resid.is_finite() || resid.abs() > LOCK_HZ || r[7] as i64 != FLOOR_AGC {
            continue;
        }
        let station = r[2] as i64;
        let mode = r[3] as i64;
        if !(mode >= 1 && mode <= 4) || !(station == 14 || station == 43 || station == 63) {
            continue;
        }
        outv.push((r[0], resid, mode, station));
    }
    Some(outv)
}

fn load_floor(all: &[(f64, f64, i64, i64)], mode: i64, station: i64, day: i64) -> Option<Vec<(f64, f64)>> {
    let mut outv: Vec<(f64, f64)> = Vec::new();
    for (t, r, m, s) in all {
        if *m != mode || *s != station {
            continue;
        }
        if unix_day(*t) == day {
            outv.push((*t, *r));
        }
    }
    outv.sort_by(|a, b| a.0.total_cmp(&b.0));
    Some(outv)
}

fn stats(v: &[(f64, f64)], lo: usize, hi: usize) -> (usize, f64, f64) {
    let n = hi - lo;
    if n == 0 {
        return (0, 0.0, 0.0);
    }
    let mut sum = 0.0;
    let mut sumsq = 0.0;
    for s in &v[lo..hi] {
        sum += s.1;
        sumsq += s.1 * s.1;
    }
    let mean = sum / n as f64;
    (n, mean, (sumsq / n as f64 - mean * mean).max(0.0).sqrt())
}

fn main() {
    let report_path = match std::env::args().skip(1).find(|a| !a.starts_with('-')) {
        Some(p) => p,
        None => "tmp/galileo_intrapass_loud_report.txt".to_string(),
    };
    let mut out: Vec<String> = Vec::new();
    out.push("galileo intra-pass structure of loud floor runs over time (tdb)".to_string());
    out.push("binding: floor = strength -2560, |resid| <= 1000, run = gap <= 600 s, run RMS about run mean (reference loudness)".to_string());
    out.push("per-window over 12 equal-time windows: window mean (drift) and window RMS about the window mean (internal noise)".to_string());
    out.push("loud window = internal RMS >= 1 Hz; uniform = >= 6 loud windows and internal-RMS cv <= 1.0; front = first-20% energy share >= 0.5; burst = loud windows in >= 2 separated clusters; else mixed".to_string());
    let Some(all) = load_all() else {
        println!("resid parse void");
        return;
    };
    for a in anchors() {
        let Some(v) = load_floor(&all, a.mode, a.station, a.day) else {
            continue;
        };
        let (n_day, _, rms_day) = stats(&v, 0, v.len());
        out.push(format!(
            "ANCHOR {} mode {} st{} day {} ({}) day-floor n {n_day} day RMS {rms_day:.4} Hz",
            a.name,
            a.mode,
            a.station,
            a.day,
            civil(a.day)
        ));
        if n_day < MIN_CELL {
            out.push("  day n < MIN_CELL (0 honored)".to_string());
            continue;
        }
        let mut runs: Vec<(usize, usize)> = Vec::new();
        let mut s = 0usize;
        for i in 1..v.len() {
            if v[i].0 - v[i - 1].0 > RUN_GAP_S {
                runs.push((s, i));
                s = i;
            }
        }
        runs.push((s, v.len()));
        let loud_runs: Vec<(usize, usize)> = runs
            .iter()
            .filter(|(lo, hi)| hi - lo >= MIN_CELL)
            .filter(|(lo, hi)| stats(&v, *lo, *hi).2 >= LOUD_HZ)
            .copied()
            .collect();
        if loud_runs.is_empty() {
            out.push("  day RMS >= 1 Hz but no single run with n >= MIN_CELL and run RMS >= 1 Hz (0 honored)".to_string());
            continue;
        }
        for (lo, hi) in &loud_runs {
            let (n_r, m_r, rms_r) = stats(&v, *lo, *hi);
            let t0 = v[*lo].0;
            let t1 = v[*hi - 1].0;
            let span = (t1 - t0).max(1.0);
            let mut win: Vec<(usize, f64, f64)> = Vec::new();
            for w in 0..N_WIN {
                let w0 = t0 + span * (w as f64) / N_WIN as f64;
                let w1 = t0 + span * ((w + 1) as f64) / N_WIN as f64;
                let mut cnt = 0usize;
                let mut sum = 0.0;
                let mut sumsq = 0.0;
                for i in *lo..*hi {
                    let t = v[i].0;
                    if t >= w0 && t < w1 {
                        cnt += 1;
                        sum += v[i].1;
                        sumsq += v[i].1 * v[i].1;
                    }
                }
                if cnt >= 5 {
                    let wmean = sum / cnt as f64;
                    let wvar = (sumsq / cnt as f64 - wmean * wmean).max(0.0);
                    win.push((w, wmean, wvar.sqrt()));
                }
            }
            let mut dist: Vec<f64> = v[*lo..*hi].iter().map(|(_, r)| r.abs()).collect();
            dist.sort_by(f64::total_cmp);
            let n_d = dist.len();
            let p50 = dist[n_d / 2];
            let p90 = dist[(n_d * 9) / 10];
            let p99 = dist[(n_d * 99) / 100];
            let max_r = dist[n_d - 1];
            let gt1 = dist.iter().filter(|x| **x > 1.0).count();
            let gt10 = dist.iter().filter(|x| **x > 10.0).count();
            let gt100 = dist.iter().filter(|x| **x > 100.0).count();
            out.push(format!(
                "    |resid| distribution n {n_d} p50 {p50:.3} p90 {p90:.3} p99 {p99:.3} max {max_r:.1}; >1Hz {gt1} >10Hz {gt10} >100Hz {gt100}"
            ));
            let loud_win: Vec<usize> = win
                .iter()
                .filter(|(_, _, r)| *r >= LOUD_HZ)
                .map(|(w, _, _)| *w)
                .collect();
            let mut clusters = 0usize;
            let mut prev: Option<usize> = None;
            for w in &loud_win {
                match prev {
                    Some(p) if *w == p + 1 => {}
                    _ => clusters += 1,
                }
                prev = Some(*w);
            }
            let wr: Vec<f64> = win.iter().map(|(_, _, r)| *r).collect();
            let wmean = wr.iter().sum::<f64>() / wr.len() as f64;
            let wvar = wr.iter().map(|r| (r - wmean) * (r - wmean)).sum::<f64>() / wr.len() as f64;
            let cv = wvar.sqrt() / wmean.max(1e-9);
            let frac_loud = if win.is_empty() { 0.0 } else { loud_win.len() as f64 / win.len() as f64 };
            let t20 = t0 + 0.2 * span;
            let mut en = 0.0f64;
            let mut en20 = 0.0f64;
            for i in *lo..*hi {
                let d = (v[i].1 - m_r) * (v[i].1 - m_r);
                en += d;
                if v[i].0 <= t20 {
                    en20 += d;
                }
            }
            let front = if en > 0.0 { en20 / en } else { 0.0 };
            let uniform = frac_loud >= 0.5 && cv <= 1.0;
            let burst = clusters >= 2;
            let kind = if front >= 0.5 {
                "front (H3)"
            } else if uniform && !burst {
                "uniform (H2)"
            } else if burst {
                "burst (H3)"
            } else {
                "mixed"
            };
            out.push(format!(
                "  loud run {}..{} UTC n {n_r} run mean {m_r:.2} Hz run RMS {rms_r:.4} Hz",
                fmt_utc(t0),
                fmt_utc(t1)
            ));
            out.push(format!(
                "    per-window [w: internalRMS about window mean; windowMean]: {}",
                win.iter()
                    .map(|(w, m, r)| format!("{w}:{r:.2}({m:.1})"))
                    .collect::<Vec<_>>()
                    .join(" ")
            ));
            out.push(format!(
                "      loud {}/{} windows ({frac_loud:.2}) internal cv {cv:.2} front20 {front:.3} clusters {clusters} -> {kind}",
                loud_win.len(),
                win.len()
            ));
        }
    }
    let text = out.join("\n");
    println!("{text}");
    let _ = std::fs::write(&report_path, text);
}
