use omegaflow::bsp_reader::daf::{DafFile, Summary};
use std::collections::HashMap;

const EGA_LO: f64 = -286_200_000.0;
const EGA_HI: f64 = -285_854_400.0;
const JD_J2000: f64 = 2451545.0;
const JD_1970: f64 = 2440587.5;

fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}

fn et_to_date(et: f64) -> String {
    let jd = JD_J2000 + et / 86_400.0;
    let dn = jd - JD_1970;
    let days = dn.floor() as i64;
    let frac = dn - days as f64;
    let (y, m, d) = civil_from_days(days);
    let secs = (frac * 86_400.0).round() as i64;
    let hh = secs / 3600;
    let mm = (secs % 3600) / 60;
    let ss = secs % 60;
    format!("{y:04}-{m:02}-{d:02} ~{hh:02}:{mm:02}:{ss:02}")
}

fn load_sclk_breaks(path: &str) -> Option<Vec<(f64, f64)>> {
    let text = std::fs::read_to_string(path).ok()?;
    let lines: Vec<&str> = text.lines().collect();
    let mut start = None;
    for (i, l) in lines.iter().enumerate() {
        if l.contains("SCLK01_COEFFICIENTS_77") {
            start = Some(i);
            break;
        }
    }
    let s = start?;
    let mut buf: Vec<f64> = Vec::new();
    for l in &lines[s + 1..] {
        let t = l.trim();
        if t == ")" {
            break;
        }
        for tok in l.split_whitespace() {
            if let Ok(v) = tok.parse::<f64>() {
                buf.push(v);
            }
        }
    }
    let mut brk: Vec<(f64, f64)> = Vec::new();
    for c in buf.chunks_exact(3) {
        brk.push((c[0], c[1]));
    }
    brk.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
    if brk.len() >= 2 {
        Some(brk)
    } else {
        None
    }
}

fn tick_to_et(t: f64, brk: &[(f64, f64)]) -> f64 {
    if t <= brk[0].0 {
        return brk[0].1;
    }
    let last = brk[brk.len() - 1];
    if t >= last.0 {
        return last.1;
    }
    let mut lo = 0usize;
    let mut hi = brk.len() - 1;
    while hi - lo > 1 {
        let mid = (lo + hi) / 2;
        if brk[mid].0 <= t {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    let (x0, y0) = brk[lo];
    let (x1, y1) = brk[hi];
    y0 + (y1 - y0) * (t - x0) / (x1 - x0)
}

fn dump(idx: usize, s: &Summary, brk: &[(f64, f64)]) {
    let dc: Vec<String> = s.doubles.iter().map(|v| format!("{v:.6e}")).collect();
    let ic: Vec<String> = s.integers.iter().map(|v| v.to_string()).collect();
    let date = if s.doubles.len() >= 1 {
        et_to_date(tick_to_et(s.doubles[0], brk))
    } else {
        String::from("-")
    };
    println!(
        "[{idx}] DC=[{}] IC=[{}] name='{}' dc0~{date}",
        dc.join(", "),
        ic.join(", "),
        s.name
    );
}

fn histo(summaries: &[Summary], slot: usize) {
    let mut m: HashMap<i32, usize> = HashMap::new();
    for s in summaries {
        if let Some(&v) = s.integers.get(slot) {
            *m.entry(v).or_insert(0) += 1;
        }
    }
    let mut v: Vec<(i32, usize)> = m.into_iter().collect();
    v.sort_by(|a, b| b.1.cmp(&a.1));
    let top: Vec<String> = v.iter().take(6).map(|(k, c)| format!("{k}:{c}")).collect();
    println!("IC[{slot}] histogram: {}", top.join("  "));
}

fn main() {
    let mut args = std::env::args().skip(1);
    let Some(path) = args.next() else {
        println!("usage: ck_daf_probe <ck.bc> [gll.tsc]");
        return;
    };
    let tsc_path = args.next();

    let daf = match DafFile::open(&path) {
        Ok(d) => d,
        Err(e) => {
            println!("DafFile::open error: {e}");
            return;
        }
    };

    println!("path: {path}");
    println!(
        "idword: {:?}  ascii='{}'",
        daf.idword(),
        String::from_utf8_lossy(&daf.idword()).trim_end()
    );
    println!(
        "nd={} ni={} summary_size_doubles={}",
        daf.nd(),
        daf.ni(),
        daf.summary_size_doubles()
    );

    let brk = tsc_path.as_deref().and_then(load_sclk_breaks);
    match &brk {
        Some(b) => println!("sclk breakpoints loaded: {} (tsc={})", b.len(), tsc_path.as_deref().unwrap_or("")),
        None => println!("sclk breakpoints: none (coverage dates not decodable)"),
    }
    if brk.is_none() {
        println!("usage for date decode: ck_daf_probe <ck.bc> <gll.tsc>");
    }

    let summaries = match daf.summaries() {
        Ok(s) => s,
        Err(e) => {
            println!("summaries() error: {e}");
            return;
        }
    };
    let total = summaries.len();
    println!("total summaries: {total}");

    if total > 0 {
        for slot in 0..4 {
            histo(&summaries, slot);
        }
    }

    let brk = match &brk {
        Some(b) => b,
        None => {
            for (i, s) in summaries.iter().enumerate() {
                dump(i, s, &[]);
            }
            return;
        }
    };

    let mut min_et = f64::INFINITY;
    let mut max_et = f64::NEG_INFINITY;
    for s in &summaries {
        if let Some(&v) = s.doubles.first() {
            let e = tick_to_et(v, brk);
            if e < min_et {
                min_et = e;
            }
        }
        if let Some(&v) = s.doubles.get(1) {
            let e = tick_to_et(v, brk);
            if e > max_et {
                max_et = e;
            }
        }
    }
    if min_et.is_finite() && max_et.is_finite() {
        println!("coverage first summary start ~{}", et_to_date(min_et));
        println!("coverage last summary end   ~{}", et_to_date(max_et));
    }

    if total <= 300 {
        println!("-- full dump ({total}) --");
        for (i, s) in summaries.iter().enumerate() {
            dump(i, s, brk);
        }
    } else {
        println!("-- first 3 --");
        for i in 0..3 {
            dump(i, &summaries[i], brk);
        }
        println!("-- last 2 --");
        for i in (total - 2)..total {
            dump(i, &summaries[i], brk);
        }
    }

    let mut ega: Vec<usize> = Vec::new();
    for (i, s) in summaries.iter().enumerate() {
        if s.doubles.len() < 2 {
            continue;
        }
        let e0 = tick_to_et(s.doubles[0], brk);
        let e1 = tick_to_et(s.doubles[1], brk);
        if e1 >= EGA_LO && e0 <= EGA_HI {
            ega.push(i);
        }
    }

    if ega.is_empty() {
        println!("verdict: NO summary overlaps EGA-1 window 1990-12-07 00:00 .. 1990-12-11 00:00 (et [{EGA_LO:.0},{EGA_HI:.0}])");
    } else {
        println!("verdict: {} summary/summaries overlap EGA-1 window:", ega.len());
        for &i in &ega {
            let s = &summaries[i];
            let e0 = tick_to_et(s.doubles[0], brk);
            let e1 = tick_to_et(s.doubles[1], brk);
            println!(
                "  segment [{i}] {} .. {}  (ticks {:.0} .. {:.0})  frame={} IC={:?}",
                et_to_date(e0),
                et_to_date(e1),
                s.doubles[0],
                s.doubles[1],
                s.integers[0],
                &s.integers[..4]
            );
            let a0 = s.integers[4] as u32;
            let a1 = s.integers[5] as u32;
            let n = (a1 - a0 + 1) as usize;
            let peek = daf.read_doubles(a0, a0 + (n.min(24) as u32) - 1);
            match peek {
                Ok(v) => {
                    let vals: Vec<String> = v.iter().map(|x| format!("{x:.6e}")).collect();
                    println!("     payload addr {a0}..{a1} ({n} doubles), first {}: [{}]", v.len(), vals.join(", "));
                }
                Err(e) => println!("     payload read error: {e}"),
            }
        }
    }
}
