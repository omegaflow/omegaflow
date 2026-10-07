use omegaflow::archivar::geo::{GeoRec, MAGIC_GIC, parse_bin};
use omegaflow::archivar::{fetch_raw, fetch_raw_bytes};
use omegaflow::lsk::days_from_civil;
use std::collections::BTreeMap;

const NUR_BASE: &str = "https://space.fmi.fi/image/www/data_download.php";
const GIC_HOURLY_URL: &str =
    "https://github.com/omegaflow/sources/releases/download/space.fmi.fi/fmi_gic.bin";
const DAY_S: f64 = 86400.0;
const HOUR: f64 = 3600.0;
const J2000_UNIX_OFFSET: f64 = 946728000.0;

fn civil_from_days(z: i64) -> (i64, i64, i64) {
    let z = z + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}

fn iso(tdb: f64) -> String {
    let t = tdb + J2000_UNIX_OFFSET;
    let days = (t / DAY_S).floor() as i64;
    let secs = t - days as f64 * DAY_S;
    let (y, m, d) = civil_from_days(days);
    let hh = (secs / HOUR) as i64;
    let mm = ((secs - hh as f64 * HOUR) / 60.0) as i64;
    format!("{y:04}-{m:02}-{d:02}T{hh:02}:{mm:02}Z")
}

fn arg_after<'a>(args: &'a [String], flag: &str) -> Option<&'a str> {
    args.iter()
        .position(|a| a == flag)
        .and_then(|i| args.get(i + 1))
        .map(|s| s.as_str())
}

fn cache_path(name: &str) -> String {
    let dir = std::path::Path::new("/tmp/opencode/nur_gic_probe");
    let _ = std::fs::create_dir_all(dir);
    dir.join(name).to_string_lossy().into_owned()
}

fn cached_text(url: &str, name: &str) -> Option<String> {
    let p = cache_path(name);
    if let Ok(s) = std::fs::read_to_string(&p) {
        if !s.is_empty() {
            return Some(s);
        }
    }
    let body = fetch_raw(url, None, &[])?;
    let _ = std::fs::write(&p, &body);
    Some(body)
}

fn cached_bytes(url: &str, name: &str) -> Option<Vec<u8>> {
    let p = cache_path(name);
    if let Ok(b) = std::fs::read(&p) {
        if !b.is_empty() {
            return Some(b);
        }
    }
    let b = fetch_raw_bytes(url)?;
    let _ = std::fs::write(&p, &b);
    Some(b)
}

fn parse_text2(text: &str) -> Vec<(f64, f64)> {
    let mut out = Vec::new();
    for line in text.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 7 {
            continue;
        }
        let Ok(y) = parts[0].parse::<i64>() else {
            continue;
        };
        if !(1982..=2100).contains(&y) {
            continue;
        }
        let Ok(m) = parts[1].parse::<i64>() else {
            continue;
        };
        let Ok(d) = parts[2].parse::<i64>() else {
            continue;
        };
        let Ok(hh) = parts[3].parse::<i64>() else {
            continue;
        };
        let Ok(mm) = parts[4].parse::<i64>() else {
            continue;
        };
        let Ok(ss) = parts[5].parse::<i64>() else {
            continue;
        };
        if !(1..=12).contains(&m) || !(1..=31).contains(&d) {
            continue;
        }
        let Ok(x) = parts[6].parse::<f64>() else {
            continue;
        };
        if !x.is_finite() {
            continue;
        }
        let Some(days) = days_from_civil(y, m, d) else {
            continue;
        };
        let t = days as f64 * DAY_S + hh as f64 * HOUR + mm as f64 * 60.0 + ss as f64;
        out.push((t, x));
    }
    out
}

fn hourly_peak_from_samples(s: &[(f64, f64)]) -> BTreeMap<i64, f64> {
    let mut out: BTreeMap<i64, f64> = BTreeMap::new();
    for &(t, v) in s {
        if !v.is_finite() {
            continue;
        }
        let k = (t / HOUR).floor() as i64;
        let a = v.abs();
        let e = out.entry(k).or_insert(0.0);
        if a > *e {
            *e = a;
        }
    }
    out
}

fn hourly_peak_from_geo(recs: &[GeoRec]) -> BTreeMap<i64, f64> {
    let mut out: BTreeMap<i64, f64> = BTreeMap::new();
    for r in recs {
        if !r.val.is_finite() {
            continue;
        }
        let unix = r.t + J2000_UNIX_OFFSET;
        let k = (unix / HOUR).floor() as i64;
        let a = r.val.abs();
        let e = out.entry(k).or_insert(0.0);
        if a > *e {
            *e = a;
        }
    }
    out
}

fn pearson(x: &[f64], y: &[f64]) -> Option<f64> {
    let n = x.len();
    if n < 3 {
        return None;
    }
    let nf = n as f64;
    let mx = x.iter().sum::<f64>() / nf;
    let my = y.iter().sum::<f64>() / nf;
    let mut sxy = 0.0;
    let mut sxx = 0.0;
    let mut syy = 0.0;
    for i in 0..n {
        let dx = x[i] - mx;
        let dy = y[i] - my;
        sxy += dx * dy;
        sxx += dx * dx;
        syy += dy * dy;
    }
    if sxx <= 0.0 || syy <= 0.0 {
        return None;
    }
    Some(sxy / (sxx.sqrt() * syy.sqrt()))
}

fn ols(x: &[f64], y: &[f64]) -> Option<(f64, f64)> {
    let n = x.len();
    if n < 3 {
        return None;
    }
    let nf = n as f64;
    let mx = x.iter().sum::<f64>() / nf;
    let my = y.iter().sum::<f64>() / nf;
    let mut sxy = 0.0;
    let mut sxx = 0.0;
    for i in 0..n {
        let dx = x[i] - mx;
        sxy += dx * (y[i] - my);
        sxx += dx * dx;
    }
    if sxx <= 0.0 {
        return None;
    }
    let slope = sxy / sxx;
    Some((slope, my - slope * mx))
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let start = arg_after(&args, "--start").unwrap_or("20031029");
    let days: i64 = arg_after(&args, "--days")
        .and_then(|v| v.parse().ok())
        .unwrap_or(3);
    let station = arg_after(&args, "--station").unwrap_or("NUR");
    let sample_rate = arg_after(&args, "--sample-rate").unwrap_or("10");

    let nur_url = format!(
        "{NUR_BASE}?starttime={start}&length={}&format=text2&stations={station}&sample_rate={sample_rate}",
        days * 1440
    );
    let nur_text = cached_text(&nur_url, &format!("nur_{start}_{days}d.txt"));
    let nur_samples = match &nur_text {
        Some(t) => parse_text2(t),
        None => Vec::new(),
    };
    let mut dbdt: Vec<(f64, f64)> = Vec::new();
    for w in nur_samples.windows(2) {
        let (t0, x0) = w[0];
        let (t1, x1) = w[1];
        let dt = t1 - t0;
        if dt > 0.0 && dt <= 60.0 {
            dbdt.push((t1, -(x1 - x0)));
        }
    }
    let nur_hour = hourly_peak_from_samples(&dbdt);

    let gic_recs =
        match cached_bytes(GIC_HOURLY_URL, "fmi_gic.bin").and_then(|b| parse_bin(MAGIC_GIC, &b)) {
            Some(r) => r,
            None => Vec::new(),
        };
    let gic_hour = hourly_peak_from_geo(&gic_recs);

    let nur_lo = match dbdt.first() {
        Some(&(t, _)) => iso(t - J2000_UNIX_OFFSET),
        None => String::new(),
    };
    let nur_hi = match dbdt.last() {
        Some(&(t, _)) => iso(t - J2000_UNIX_OFFSET),
        None => String::new(),
    };
    println!(
        "NUR {station}: {} text samples, {} dB/dt samples, {} hours ({} .. {})",
        nur_samples.len(),
        dbdt.len(),
        nur_hour.len(),
        nur_lo,
        nur_hi
    );
    println!(
        "GIC hourly: {} records, {} hours",
        gic_recs.len(),
        gic_hour.len()
    );

    let mut xs: Vec<f64> = Vec::new();
    let mut ys: Vec<f64> = Vec::new();
    for (&k, &nv) in &nur_hour {
        if let Some(&gv) = gic_hour.get(&k) {
            xs.push(nv);
            ys.push(gv);
        }
    }
    println!("aligned hours: {}", xs.len());
    if xs.len() < 3 {
        eprintln!(
            "overlap carries <3 aligned hours — the relation stays pending (no fabricated value)"
        );
        std::process::exit(0);
    }
    let mut peak = (0.0f64, 0.0f64, 0i64);
    for (&k, &nv) in &nur_hour {
        if let Some(&gv) = gic_hour.get(&k) {
            if gv > peak.1 {
                peak = (nv, gv, k);
            }
        }
    }
    println!(
        "peak hour {}: NUR |dX/10s|={:.1} nT/10s, GIC |I|={:.2} A",
        iso(peak.2 as f64 * HOUR - J2000_UNIX_OFFSET),
        peak.0,
        peak.1
    );
    match pearson(&xs, &ys) {
        Some(r) => println!(
            "Pearson r(hourly |dB/dt|, hourly |GIC|) = {r:.6} (n={})",
            xs.len()
        ),
        None => println!("Pearson r void"),
    }
    match ols(&xs, &ys) {
        Some((s, b)) => println!("OLS hourly |GIC| = {s:.6}·|dX/10s| + {b:.6} (A per nT/10s)"),
        None => println!("OLS void"),
    }
}
