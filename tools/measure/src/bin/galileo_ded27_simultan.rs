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
const ANCHOR_DAY: i64 = 9458;

fn unix_day(tdb: f64) -> i64 {
    (tdb / DAY_S + 10957.5).round() as i64
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
    let s = (rem % 60.0) as i64;
    format!("{h:02}:{m:02}:{s:02}")
}

struct Acc {
    n: usize,
    sum: f64,
    sumsq: f64,
}

impl Acc {
    fn push(&mut self, v: f64) {
        self.n += 1;
        self.sum += v;
        self.sumsq += v * v;
    }
    fn rms(&self) -> f64 {
        if self.n == 0 {
            return 0.0;
        }
        let mean = self.sum / self.n as f64;
        (self.sumsq / self.n as f64 - mean * mean).max(0.0).sqrt()
    }
}

fn load() -> Option<(BTreeMap<(i64, i64), Vec<(f64, f64)>>, usize)> {
    let file = File::open("data/pds-ppi.igpp.ucla.edu/galileo_resid.bin").ok()?;
    let mut reader = BufReader::new(file);
    let mut magic = [0u8; 4];
    reader.read_exact(&mut magic).ok()?;
    if &magic != b"GASR" {
        return None;
    }
    let mut count_buf = [0u8; 4];
    reader.read_exact(&mut count_buf).ok()?;
    let mut per: BTreeMap<(i64, i64), Vec<(f64, f64)>> = BTreeMap::new();
    let mut rec = [0u8; 64];
    let mut kept = 0usize;
    loop {
        if reader.read_exact(&mut rec).is_err() {
            break;
        }
        let mut r = [0.0f64; 8];
        for (k, slot) in r.iter_mut().enumerate() {
            let mut buf = [0u8; 8];
            buf.copy_from_slice(&rec[k * 8..k * 8 + 8]);
            *slot = f64::from_le_bytes(buf);
        }
        let st = r[2] as i64;
        let mode = r[3] as i64;
        if !TRIO.contains(&st) || !(1..=4).contains(&mode) {
            continue;
        }
        let resid = r[1];
        if !resid.is_finite() || resid.abs() > LOCK_HZ {
            continue;
        }
        if r[7] as i64 != FLOOR_AGC {
            continue;
        }
        per.entry((mode, st))
            .or_insert_with(Vec::new)
            .push((r[0], resid));
        kept += 1;
    }
    for v in per.values_mut() {
        v.sort_by(|a, b| a.0.total_cmp(&b.0));
    }
    Some((per, kept))
}

fn runs(v: &[(f64, f64)]) -> Vec<(usize, usize)> {
    let mut outr: Vec<(usize, usize)> = Vec::new();
    if v.is_empty() {
        return outr;
    }
    let mut s = 0usize;
    for i in 1..v.len() {
        if v[i].0 - v[i - 1].0 > RUN_GAP_S {
            outr.push((s, i - 1));
            s = i;
        }
    }
    outr.push((s, v.len() - 1));
    outr
}

fn main() {
    let report_path = match std::env::args().skip(1).find(|a| !a.starts_with('-')) {
        Some(p) => p,
        None => "tmp/galileo_ded27_simultan_report.txt".to_string(),
    };
    let Some((per, kept)) = load() else {
        println!("galileo_ded27_simultan: resid parse void");
        return;
    };
    let mut out: Vec<String> = Vec::new();
    out.push("galileo simultaneity two-station test (Ded-27) over the floor-era resid".to_string());
    out.push(format!("floor trio non-lock AGC samples kept: {kept}"));
    out.push("floor sample = strength == -2560, |resid| <= 1000 Hz, station in {14,43,63}, mode 1..4".to_string());
    out.push("run = maximal sample run within one (mode, station) with tdb gap <= 600 s".to_string());
    out.push("simultaneous cell = tdb overlap of one run at station A and one run at station B in the SAME mode".to_string());
    out.push("cell n = samples of that station inside the overlap window; cell RMS about the cell mean (reference loudness, loud >= 1 Hz)".to_string());

    let mut cells: Vec<(i64, i64, i64, f64, f64, usize, usize, f64, f64)> = Vec::new();
    for mode in 1..=4i64 {
        for ia in 0..3 {
            for ib in (ia + 1)..3 {
                let a = TRIO[ia];
                let b = TRIO[ib];
                let (Some(va), Some(vb)) = (per.get(&(mode, a)), per.get(&(mode, b))) else {
                    continue;
                };
                let ra = runs(va);
                let rb = runs(vb);
                for &(sa, ea) in &ra {
                    for &(sb, eb) in &rb {
                        let lo = va[sa].0.max(vb[sb].0);
                        let hi = va[ea].0.min(vb[eb].0);
                        if hi <= lo {
                            continue;
                        }
                        let mut aa = Acc { n: 0, sum: 0.0, sumsq: 0.0 };
                        for i in sa..=ea {
                            if va[i].0 >= lo && va[i].0 <= hi {
                                aa.push(va[i].1);
                            }
                        }
                        let mut bb = Acc { n: 0, sum: 0.0, sumsq: 0.0 };
                        for i in sb..=eb {
                            if vb[i].0 >= lo && vb[i].0 <= hi {
                                bb.push(vb[i].1);
                            }
                        }
                        if aa.n == 0 || bb.n == 0 {
                            continue;
                        }
                        cells.push((mode, a, b, lo, hi, aa.n, bb.n, aa.rms(), bb.rms()));
                    }
                }
            }
        }
    }

    cells.sort_by(|x, y| {
        x.0.cmp(&y.0)
            .then(x.3.total_cmp(&y.3))
            .then(x.1.cmp(&y.1))
            .then(x.2.cmp(&y.2))
    });

    out.push(format!("\nsimultaneous cells total: {}", cells.len()));
    let robust: Vec<&(i64, i64, i64, f64, f64, usize, usize, f64, f64)> = cells
        .iter()
        .filter(|c| c.5 >= MIN_CELL && c.6 >= MIN_CELL)
        .collect();
    out.push(format!("robust simultaneous cells (nA >= {MIN_CELL} AND nB >= {MIN_CELL}): {}", robust.len()));
    for mode in 1..=4i64 {
        let cm: Vec<&(i64, i64, i64, f64, f64, usize, usize, f64, f64)> =
            cells.iter().filter(|c| c.0 == mode).collect();
        if cm.is_empty() {
            out.push(format!("  mode {mode}: no simultaneous cells (0 honored)"));
            continue;
        }
        out.push(format!("  mode {mode}: simultaneous cells {}", cm.len()));
        for c in &cm {
            let day = unix_day((c.3 + c.4) * 0.5);
            out.push(format!(
                "    {} st{} vs st{} [{} .. {} UTC] nA {} nB {} rmsA {:.4} rmsB {:.4} {}",
                civil(day),
                c.1,
                c.2,
                fmt_utc(c.3),
                fmt_utc(c.4),
                c.5,
                c.6,
                c.7,
                c.8,
                if c.7 >= LOUD_HZ && c.8 >= LOUD_HZ {
                    "both-loud"
                } else if c.7 >= LOUD_HZ {
                    "A-loud-B-quiet"
                } else if c.8 >= LOUD_HZ {
                    "A-quiet-B-loud"
                } else {
                    "both-quiet"
                }
            ));
        }
    }

    out.push("\nrobust cells by loudness pattern per mode (Ded-27 verdict input):".to_string());
    for mode in 1..=4i64 {
        let mut both = 0usize;
        let mut a_only = 0usize;
        let mut b_only = 0usize;
        let mut none = 0usize;
        for c in robust.iter().filter(|c| c.0 == mode) {
            match (c.7 >= LOUD_HZ, c.8 >= LOUD_HZ) {
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
    let both = robust.iter().filter(|c| c.7 >= LOUD_HZ && c.8 >= LOUD_HZ).count();
    let one = robust
        .iter()
        .filter(|c| (c.7 >= LOUD_HZ) != (c.8 >= LOUD_HZ))
        .count();
    let none = robust.iter().filter(|c| c.7 < LOUD_HZ && c.8 < LOUD_HZ).count();
    out.push(format!(
        "robust totals: both-loud {both} | one-loud {one} | both-quiet {none}"
    ));

    out.push(format!("\nanchor 1995-11-24 (day {ANCHOR_DAY}) simultaneous cells (all n):"));
    let anc: Vec<&(i64, i64, i64, f64, f64, usize, usize, f64, f64)> = cells
        .iter()
        .filter(|c| unix_day((c.3 + c.4) * 0.5) == ANCHOR_DAY)
        .collect();
    if anc.is_empty() {
        out.push("  none (0 honored)".to_string());
    } else {
        for c in &anc {
            out.push(format!(
                "  mode {} st{} vs st{} [{} .. {} UTC] nA {} nB {} rmsA {:.4} rmsB {:.4} loudA {} loudB {}",
                c.0,
                c.1,
                c.2,
                fmt_utc(c.3),
                fmt_utc(c.4),
                c.5,
                c.6,
                c.7,
                c.8,
                c.7 >= LOUD_HZ,
                c.8 >= LOUD_HZ
            ));
        }
    }

    let text = out.join("\n");
    println!("{text}");
    let _ = std::fs::write(&report_path, text);
}
