use std::fs::File;
use std::io::{BufReader, Read};

fn civil_days(y: i64, m: i64, d: i64) -> i64 {
    let yy = if m <= 2 { y - 1 } else { y };
    let era = if yy >= 0 { yy } else { yy - 399 } / 400;
    let yoe = yy - era * 400;
    let mp = if m > 2 { m - 3 } else { m + 9 };
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe - 719468
}

use omegaflow::spectral::civil_from_days;

const DAY_S: f64 = 86400.0;
const LOCK_HZ: f64 = 1.0e3;
const FLOOR_AGC: i64 = -2560;
const LOUD_HZ: f64 = 1.0;
const MIN_CELL: usize = 30;
const RUN_GAP_S: f64 = 600.0;
const N_WIN: usize = 12;

#[derive(Clone, Copy)]
struct Anchor {
    mode: i64,
    station: i64,
    year: i64,
    month: i64,
    day: i64,
    name: &'static str,
}

fn anchors() -> Vec<Anchor> {
    vec![
        Anchor { mode: 1, station: 14, year: 1995, month: 11, day: 24, name: "M1 st14 1995-11-24 (ref 25.85 Hz)" },
        Anchor { mode: 2, station: 14, year: 1995, month: 11, day: 24, name: "M2 st14 1995-11-24 (ref 31.77 Hz)" },
        Anchor { mode: 3, station: 14, year: 1995, month: 12, day: 5, name: "M3 st14 1995-12-05 (ref 52.9 Hz)" },
        Anchor { mode: 3, station: 63, year: 1995, month: 11, day: 27, name: "M3 st63 1995-11-27 (ref 186.5 Hz)" },
        Anchor { mode: 1, station: 63, year: 1996, month: 6, day: 26, name: "M1 st63 1996-06-26 (ref 20.64 Hz)" },
        Anchor { mode: 1, station: 43, year: 1996, month: 11, day: 4, name: "M1 st43 1996-11-04 (ref 23.1 Hz)" },
        Anchor { mode: 3, station: 43, year: 1995, month: 12, day: 4, name: "M3 st43 1995-12-04 (ref 10.5 Hz)" },
    ]
}

fn civil_day(day: i64) -> String {
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
    let s = (rem % 60.0) as i64;
    format!("{h:02}:{m:02}:{s:02}")
}

fn stats_of(v: &[(f64, f64)], lo: usize, hi: usize) -> (usize, f64, f64) {
    let n = hi - lo;
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
        None => "tmp/galileo_ded27_intrapass_report.txt".to_string(),
    };
    let mut out: Vec<String> = Vec::new();
    out.push("galileo intra-pass structure of loud floor runs over tdb (Ded-27, H2 vs H3)".to_string());
    out.push("binding: floor = strength -2560, |resid| <= 1000 Hz, run = gap <= 600 s".to_string());
    out.push("reference day cell = samples whose floor(tdb/86400) == unix_day_of_date - 10958 (midday-to-midday window of the reference series)".to_string());
    out.push("run RMS and day RMS are about the respective mean (reference loudness threshold 1 Hz)".to_string());
    out.push("structure per loud run: 12 equal-time windows (window mean + internal RMS about the window mean), drift/noise split, front-20% energy share, |resid| quantiles".to_string());

    let anc = anchors();
    let file = match File::open("data/pds-ppi.igpp.ucla.edu/galileo_resid.bin") {
        Ok(f) => f,
        Err(_) => {
            println!("resid open void");
            return;
        }
    };
    let mut reader = BufReader::new(file);
    let mut magic = [0u8; 4];
    if reader.read_exact(&mut magic).is_err() || &magic != b"GASR" {
        println!("resid header void");
        return;
    }
    let mut count_buf = [0u8; 4];
    if reader.read_exact(&mut count_buf).is_err() {
        println!("resid count void");
        return;
    }
    let mut series: Vec<Vec<(f64, f64)>> = vec![Vec::new(); anc.len()];
    let mut rec = [0u8; 64];
    while reader.read_exact(&mut rec).is_ok() {
        let mut r = [0.0f64; 8];
        for (k, slot) in r.iter_mut().enumerate() {
            let mut buf = [0u8; 8];
            buf.copy_from_slice(&rec[k * 8..k * 8 + 8]);
            *slot = f64::from_le_bytes(buf);
        }
        let resid = r[1];
        if !resid.is_finite() || resid.abs() > LOCK_HZ {
            continue;
        }
        if r[7] as i64 != FLOOR_AGC {
            continue;
        }
        let mode = r[3] as i64;
        let station = r[2] as i64;
        let bucket = (r[0] / DAY_S).floor() as i64;
        for (i, a) in anc.iter().enumerate() {
            let want = civil_days(a.year, a.month, a.day) - 10958;
            if a.mode == mode && a.station == station && bucket == want {
                series[i].push((r[0], resid));
                break;
            }
        }
    }
    for v in series.iter_mut() {
        v.sort_by(|x, y| x.0.total_cmp(&y.0));
    }

    for (i, a) in anc.iter().enumerate() {
        let v = &series[i];
    if v.is_empty() {
        out.push("  0 floor samples in the reference day cell (0 honored)".to_string());
        continue;
    }
        out.push(format!("\nANCHOR {} mode {} st{} ({})", a.name, a.mode, a.station, civil_day(civil_days(a.year, a.month, a.day))));
        let (n_cell, m_cell, rms_cell) = stats_of(v, 0, v.len());
        out.push(format!("  reference day cell: n {n_cell} mean {m_cell:.3} Hz RMS {rms_cell:.4} Hz"));
        if n_cell < MIN_CELL {
            out.push("  day n < 30 (0 honored)".to_string());
            continue;
        }
        let mut run_segments: Vec<(usize, usize)> = Vec::new();
        if !v.is_empty() {
            let mut s = 0usize;
            for k in 1..v.len() {
                if v[k].0 - v[k - 1].0 > RUN_GAP_S {
                    run_segments.push((s, k));
                    s = k;
                }
            }
            run_segments.push((s, v.len()));
        }
        let loud: Vec<(usize, usize)> = run_segments
            .iter()
            .copied()
            .filter(|(lo, hi)| hi - lo >= MIN_CELL && stats_of(v, *lo, *hi).2 >= LOUD_HZ)
            .collect();
        if loud.is_empty() {
            out.push("  no run with n >= 30 and run RMS >= 1 Hz inside the day cell (0 honored)".to_string());
            continue;
        }
        out.push(format!("  loud runs inside the day cell: {}", loud.len()));
        for (lo, hi) in &loud {
            let (n_r, m_r, rms_r) = stats_of(v, *lo, *hi);
            let t0 = v[*lo].0;
            let t1 = v[*hi - 1].0;
            let span = (t1 - t0).max(1.0);
            out.push(format!(
                "    run [{} .. {} UTC] n {n_r} mean {m_r:.3} Hz run RMS {rms_r:.4} Hz span {:.0} s",
                fmt_utc(t0),
                fmt_utc(t1),
                span
            ));
            let mut wins: Vec<(usize, usize, f64, f64)> = Vec::new();
            let mut tot_n = 0usize;
            let mut noise_var_num = 0.0f64;
            for w in 0..N_WIN {
                let wa = t0 + span * (w as f64) / N_WIN as f64;
                let wb = t0 + span * ((w + 1) as f64) / N_WIN as f64;
                let mut cnt = 0usize;
                let mut sum = 0.0;
                let mut sumsq = 0.0;
                for s in &v[*lo..*hi] {
                    if s.0 >= wa && s.0 < wb {
                        cnt += 1;
                        sum += s.1;
                        sumsq += s.1 * s.1;
                    }
                }
                if cnt >= 5 {
                    let wmean = sum / cnt as f64;
                    let wvar = (sumsq / cnt as f64 - wmean * wmean).max(0.0);
                    wins.push((w, cnt, wmean, wvar.sqrt()));
                    tot_n += cnt;
                    noise_var_num += wvar * cnt as f64;
                }
            }
            let noise_rms = if tot_n > 0 {
                (noise_var_num / tot_n as f64).sqrt()
            } else {
                0.0
            };
            let drift_rms = (rms_r * rms_r - noise_rms * noise_rms).max(0.0).sqrt();
            out.push(format!(
                "    per-window [index: internal RMS about window mean; window mean; n]: {}",
                wins.iter()
                    .map(|(w, cnt, wm, wr)| format!("{w}:{wr:.3}({wm:.2},n{cnt})"))
                    .collect::<Vec<_>>()
                    .join("  ")
            ));
            out.push(format!(
                "    decomposition: total RMS {rms_r:.4} Hz = drift {drift_rms:.4} + noise {noise_rms:.4} (drift share {:.2})",
                if rms_r > 0.0 { drift_rms * drift_rms / (rms_r * rms_r) } else { 0.0 }
            ));
            let t20 = t0 + 0.2 * span;
            let mut en = 0.0f64;
            let mut en20 = 0.0f64;
            for s in &v[*lo..*hi] {
                let d = (s.1 - m_r) * (s.1 - m_r);
                en += d;
                if s.0 <= t20 {
                    en20 += d;
                }
            }
            let front = if en > 0.0 { en20 / en } else { 0.0 };
            let mut dist: Vec<f64> = v[*lo..*hi].iter().map(|s| s.1.abs()).collect();
            dist.sort_by(f64::total_cmp);
            let p50 = dist[dist.len() / 2];
            let p90 = dist[(dist.len() * 9) / 10];
            let p99 = dist[(dist.len() * 99) / 100];
            let gt1 = dist.iter().filter(|x| **x > 1.0).count();
            let gt10 = dist.iter().filter(|x| **x > 10.0).count();
            let gt100 = dist.iter().filter(|x| **x > 100.0).count();
            let mx = dist[dist.len() - 1];
            out.push(format!(
                "    |resid| p50 {p50:.3} p90 {p90:.3} p99 {p99:.3} max {mx:.3} Hz | >1Hz {gt1} >10Hz {gt10} >100Hz {gt100} | front20 share {front:.3}"
            ));
        }
    }

    let text = out.join("\n");
    println!("{text}");
    let _ = std::fs::write(&report_path, text);
}
