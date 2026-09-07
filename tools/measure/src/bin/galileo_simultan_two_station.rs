use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufReader, Read};

use omegaflow::spectral::civil_from_days;

const DAY_S: f64 = 86400.0;
const LOCK_HZ: f64 = 1.0e3;
const FLOOR_AGC: i64 = -2560;
const LOUD_HZ: f64 = 1.0;
const MIN_CELL: usize = 30;
const RUN_GAP_S: f64 = 600.0;
const TRIO: [i64; 3] = [14, 43, 63];

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

fn fmt_hhmm(tdb: f64) -> String {
    let unix = tdb + 10957.5 * DAY_S;
    let rem = unix.rem_euclid(DAY_S);
    let h = (rem / 3600.0) as i64;
    let m = ((rem % 3600.0) / 60.0) as i64;
    format!("{h:02}:{m:02}")
}

fn load_floor_trio() -> Option<(Vec<[f64; 8]>, usize, usize)> {
    let file = File::open(resid_path()).ok()?;
    let mut reader = BufReader::new(file);
    let mut magic = [0u8; 4];
    reader.read_exact(&mut magic).ok()?;
    if &magic != b"GASR" {
        return None;
    }
    let mut count_buf = [0u8; 4];
    reader.read_exact(&mut count_buf).ok()?;
    let _count = u32::from_le_bytes(count_buf) as usize;
    let mut all = Vec::new();
    let mut rec = [0u8; 64];
    let mut n_tot = 0usize;
    let mut n_kept = 0usize;
    loop {
        match reader.read_exact(&mut rec) {
            Ok(()) => {}
            Err(_) => break,
        }
        n_tot += 1;
        let mut r = [0.0f64; 8];
        for k in 0..8 {
            let mut buf = [0u8; 8];
            buf.copy_from_slice(&rec[k * 8..k * 8 + 8]);
            r[k] = f64::from_le_bytes(buf);
        }
        let st = r[2] as i64;
        let mode = r[3] as i64;
        if !(mode >= 1 && mode <= 4) || !TRIO.contains(&st) {
            continue;
        }
        let resid = r[1];
        if !resid.is_finite() || resid.abs() > LOCK_HZ {
            continue;
        }
        if r[7] as i64 != FLOOR_AGC {
            continue;
        }
        n_kept += 1;
        all.push(r);
    }
    Some((all, n_tot, n_kept))
}

struct RunStat {
    n: usize,
    sum: f64,
    sumsq: f64,
}

impl RunStat {
    fn new() -> RunStat {
        RunStat {
            n: 0,
            sum: 0.0,
            sumsq: 0.0,
        }
    }
    fn push(&mut self, v: f64) {
        self.n += 1;
        self.sum += v;
        self.sumsq += v * v;
    }
    fn rms(&self) -> Option<f64> {
        if self.n == 0 {
            return None;
        }
        let mean = self.sum / self.n as f64;
        Some((self.sumsq / self.n as f64 - mean * mean).max(0.0).sqrt())
    }
}

#[derive(Clone, Copy)]
struct CellOut {
    mode: i64,
    a: i64,
    b: i64,
    day: i64,
    t0: f64,
    t1: f64,
    n_a: usize,
    n_b: usize,
    rms_a: f64,
    rms_b: f64,
    loud_a: bool,
    loud_b: bool,
}

fn main() {
    let report_path = match std::env::args().skip(1).find(|a| !a.starts_with('-')) {
        Some(p) => p,
        None => "tmp/galileo_simultan_two_station_report.txt".to_string(),
    };
    let Some((all, n_tot, n_kept)) = load_floor_trio() else {
        println!("galileo_simultan_two_station: resid parse void");
        return;
    };
    let mut out: Vec<String> = Vec::new();
    out.push("galileo simultaneity two-station test over the floor-era resid".to_string());
    out.push(format!(
        "resid records read {n_tot}, kept floor trio non-lock samples {n_kept}"
    ));
    out.push("binding: floor sample = strength == -2560, |resid| <= 1000, station in {14,43,63}, mode 1..4".to_string());
    out.push("loud cell = resid RMS >= 1 Hz about the cell mean (reference threshold)".to_string());
    out.push(
        "run = maximal sample run with tdb gap <= 600 s within one (mode, station)".to_string(),
    );
    out.push("simultaneous cell = tdb overlap of one run at station A and one run at station B in the SAME mode".to_string());

    let mut per: BTreeMap<(i64, i64), Vec<(f64, f64)>> = BTreeMap::new();
    for r in &all {
        per.entry((r[3] as i64, r[2] as i64))
            .or_insert_with(Vec::new)
            .push((r[0], r[1]));
    }

    let mut cells: Vec<CellOut> = Vec::new();
    for mode in 1..=4i64 {
        for ia in 0..3 {
            for ib in (ia + 1)..3 {
                let a = TRIO[ia];
                let b = TRIO[ib];
                let Some(va) = per.get(&(mode, a)) else {
                    continue;
                };
                let Some(vb) = per.get(&(mode, b)) else {
                    continue;
                };
                let runs = |v: &Vec<(f64, f64)>| -> Vec<(usize, usize, f64, f64)> {
                    let mut outr: Vec<(usize, usize, f64, f64)> = Vec::new();
                    if v.is_empty() {
                        return outr;
                    }
                    let mut s = 0usize;
                    for i in 1..v.len() {
                        if v[i].0 - v[i - 1].0 > RUN_GAP_S {
                            outr.push((s, i - 1, v[s].0, v[i - 1].0));
                            s = i;
                        }
                    }
                    outr.push((s, v.len() - 1, v[s].0, v[v.len() - 1].0));
                    outr
                };
                let ra = runs(va);
                let rb = runs(vb);
                for (s_a, e_a, t0a, t1a) in &ra {
                    for (s_b, e_b, t0b, t1b) in &rb {
                        let lo = t0a.max(*t0b);
                        let hi = t1a.min(*t1b);
                        if hi <= lo {
                            continue;
                        }
                        let mut sa = RunStat::new();
                        for i in *s_a..=*e_a {
                            if va[i].0 >= lo && va[i].0 <= hi {
                                sa.push(va[i].1);
                            }
                        }
                        let mut sb = RunStat::new();
                        for i in *s_b..=*e_b {
                            if vb[i].0 >= lo && vb[i].0 <= hi {
                                sb.push(vb[i].1);
                            }
                        }
                        if sa.n == 0 || sb.n == 0 {
                            continue;
                        }
                        let (Some(rms_a), Some(rms_b)) = (sa.rms(), sb.rms()) else {
                            continue;
                        };
                        let day = unix_day((lo + hi) * 0.5);
                        cells.push(CellOut {
                            mode,
                            a,
                            b,
                            day,
                            t0: lo,
                            t1: hi,
                            n_a: sa.n,
                            n_b: sb.n,
                            rms_a,
                            rms_b,
                            loud_a: rms_a >= LOUD_HZ,
                            loud_b: rms_b >= LOUD_HZ,
                        });
                    }
                }
            }
        }
    }

    cells.sort_by(|x, y| {
        x.mode
            .cmp(&y.mode)
            .then(x.day.cmp(&y.day))
            .then(x.a.cmp(&y.a))
            .then(x.b.cmp(&y.b))
    });

    out.push(format!("simultaneous cells total: {}", cells.len()));
    for mode in 1..=4i64 {
        let cm: Vec<&CellOut> = cells.iter().filter(|c| c.mode == mode).collect();
        if cm.is_empty() {
            out.push(format!("mode {mode}: no simultaneous cells (0 honored)"));
            continue;
        }
        out.push(format!("mode {mode}: simultaneous cells {}", cm.len()));
        for c in &cm {
            let lbl = match (c.loud_a, c.loud_b) {
                (true, true) => "both-loud",
                (true, false) => "A-loud-B-quiet",
                (false, true) => "A-quiet-B-loud",
                (false, false) => "both-quiet",
            };
            out.push(format!(
                "  {} A st{} B st{} day {} [{}..{} UTC] nA {} nB {} rmsA {:.4} rmsB {:.4} {}",
                civil(c.day),
                c.a,
                c.b,
                c.day,
                fmt_hhmm(c.t0),
                fmt_hhmm(c.t1),
                c.n_a,
                c.n_b,
                c.rms_a,
                c.rms_b,
                lbl
            ));
        }
    }

    out.push("\nrobust simult cells (nA >= 30 AND nB >= 30), by loudness pattern:".to_string());
    let robust: Vec<&CellOut> = cells
        .iter()
        .filter(|c| c.n_a >= MIN_CELL && c.n_b >= MIN_CELL)
        .collect();
    out.push(format!("robust cells total: {}", robust.len()));
    for mode in 1..=4i64 {
        let mut both = 0usize;
        let mut a_only = 0usize;
        let mut b_only = 0usize;
        let mut none = 0usize;
        for c in robust.iter().filter(|c| c.mode == mode) {
            match (c.loud_a, c.loud_b) {
                (true, true) => both += 1,
                (true, false) => a_only += 1,
                (false, true) => b_only += 1,
                (false, false) => none += 1,
            }
        }
        out.push(format!(
            "  mode {mode}: both-loud {both} | A-loud-B-quiet {a_only} | A-quiet-B-loud {b_only} | both-quiet {none}"
        ));
    }

    let anchor_day = 9458;
    out.push("\nanchor 1995-11-24 (day 9458) simult cells:".to_string());
    let anc: Vec<&CellOut> = cells.iter().filter(|c| c.day == anchor_day).collect();
    if anc.is_empty() {
        out.push("  none (0 honored)".to_string());
    } else {
        for c in &anc {
            out.push(format!(
                "  mode {} A st{} B st{} day {} nA {} nB {} rmsA {:.4} rmsB {:.4} loudA {} loudB {}",
                c.mode, c.a, c.b, c.day, c.n_a, c.n_b, c.rms_a, c.rms_b, c.loud_a, c.loud_b
            ));
        }
    }

    let text = out.join("\n");
    println!("{text}");
    let _ = std::fs::write(&report_path, text);
}
