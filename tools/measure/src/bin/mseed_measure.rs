use omegaflow_measure::miniseed::decode_body;
use std::env;
use std::fs;

fn ymd_to_days(y: i64, m: i64, d: i64) -> Option<f64> {
    let (m, y) = if m <= 2 { (m + 12, y - 1) } else { (m, y) };
    let a = y / 100;
    let b = 2 - a + a / 4;
    Some(
        (365.25 * (y + 4716) as f64).floor()
            + (30.6001 * (m + 1) as f64).floor()
            + d as f64
            + b as f64
            - 1524.5,
    )
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: mseed_messen <file.mseed> [start_epoch] [end_epoch]");
        return;
    }
    let bytes = match fs::read(&args[1]) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("lesen {}: {e}", args[1]);
            return;
        }
    };
    let Some((samples, _)) = decode_body(&bytes) else {
        println!("no samples");
        return;
    };
    let start = args.get(2).and_then(|s| s.parse::<f64>().ok());
    let end = args.get(3).and_then(|s| s.parse::<f64>().ok());
    println!("samples: {}", samples.len());
    let (t0, t1) = (
        samples.first().map(|x| x.0).unwrap(),
        samples.last().map(|x| x.0).unwrap(),
    );
    println!("zeitbereich: {} .. {}  (UTC)", t0, t1);

    let mut mn = f64::INFINITY;
    let mut mx = f64::NEG_INFINITY;
    for (_, v) in &samples {
        if *v < mn {
            mn = *v;
        }
        if *v > mx {
            mx = *v;
        }
    }
    println!("global: min {mn:.1} max {mx:.1}");

    let lo = start.unwrap_or(t0);
    let hi = end.unwrap_or(t1);
    let mut cell = (lo / 60.0).floor() * 60.0;
    let mut acc: Vec<(f64, f64, f64, f64, usize)> = Vec::new();
    let mut cur: Option<(f64, f64, f64, f64, usize)> = None;
    for (t, v) in &samples {
        if *t < lo || *t > hi {
            continue;
        }
        let c = (t / 60.0).floor() * 60.0;
        if cur.is_none() {
            cur = Some((c, 0.0, *v, *v, 1));
            cell = c;
        }
        if c != cell {
            if let Some((cb, s, cmx, cmn, n)) = cur.take() {
                let rms = (s / n as f64).sqrt();
                acc.push((cb, rms, cmx, cmn, n));
            }
            cell = c;
            cur = Some((c, 0.0, *v, *v, 1));
        }
        let (_, s, cmx, cmn, n) = cur.as_mut().unwrap();
        *s += v * v;
        *n += 1;
        if *v > *cmx {
            *cmx = *v;
        }
        if *v < *cmn {
            *cmn = *v;
        }
    }
    if let Some((cb, s, cmx, cmn, n)) = cur {
        let rms = (s / n as f64).sqrt();
        acc.push((cb, rms, cmx, cmn, n));
    }
    println!("== 1-min cells (start_utc | n | rms | max | min) ==");
    for (cb, rms, cmx, cmn, n) in acc {
        println!("{:.0} | {n} | {rms:.1} | {cmx:.1} | {cmn:.1}", cb);
    }

    let ymd = ymd_to_days(2026, 8, 26).unwrap();
    let kollab = (ymd + 0.0) * 86400.0;
    println!("kollab_epoch_referenz: {kollab:.0}");
}
