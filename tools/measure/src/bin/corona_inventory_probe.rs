fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = (if mp < 10 { mp + 3 } else { mp - 9 }) as u32;
    (if m <= 2 { y + 1 } else { y }, m, d)
}

fn utc_day(unix: f64) -> String {
    let days = (unix / 86_400.0).floor() as i64;
    let (y, m, d) = civil_from_days(days);
    format!("{y:04}-{m:02}-{d:02}")
}

fn median_step(times: &[(f64, u32)]) -> Option<f64> {
    if times.len() < 3 {
        return None;
    }
    let mut dt: Vec<f64> = times
        .windows(2)
        .map(|w| w[1].0 - w[0].0)
        .filter(|v| *v > 0.0 && v.is_finite())
        .collect();
    if dt.is_empty() {
        return None;
    }
    dt.sort_by(|a, b| a.total_cmp(b));
    Some(dt[dt.len() / 2])
}

fn read_records(path: &str, magic: &[u8; 4]) -> Vec<(f64, f64, u32)> {
    let Ok(bytes) = std::fs::read(path) else {
        eprintln!("{path} reads void");
        return Vec::new();
    };
    if bytes.len() < 8 || &bytes[0..4] != magic {
        eprintln!(
            "{path} carries no {} contract",
            String::from_utf8_lossy(magic)
        );
        return Vec::new();
    }
    let n = u32::from_le_bytes(bytes[4..8].try_into().unwrap()) as usize;
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let o = 8 + i * 20;
        let Some(t) = bytes
            .get(o..o + 8)
            .and_then(|b| b.try_into().ok())
            .map(f64::from_le_bytes)
        else {
            continue;
        };
        let Some(v) = bytes
            .get(o + 8..o + 16)
            .and_then(|b| b.try_into().ok())
            .map(f64::from_le_bytes)
        else {
            continue;
        };
        let Some(idx) = bytes
            .get(o + 16..o + 20)
            .and_then(|b| b.try_into().ok())
            .map(u32::from_le_bytes)
        else {
            continue;
        };
        out.push((t, v, idx));
    }
    out
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let lsk = omegaflow::archivar::embedded_lsk();
    for (flag, magic, labels) in [
        (
            "--eve",
            b"EVL1",
            &[
                (0u32, "94A"),
                (1, "131A"),
                (3, "171A"),
                (6, "195A"),
                (8, "211A"),
                (10, "284A"),
                (11, "304A"),
                (12, "335A"),
                (23, "584A"),
                (36, "977A"),
                (38, "1032A"),
            ][..],
        ),
        (
            "--aia",
            b"AIA1",
            &[
                (0u32, "94A"),
                (1, "131A"),
                (2, "171A"),
                (3, "193A"),
                (4, "211A"),
                (5, "304A"),
                (6, "335A"),
            ][..],
        ),
    ] {
        let Some(path) = arg_value(&args, flag) else {
            continue;
        };
        let records = read_records(&path, magic);
        if records.is_empty() {
            continue;
        }
        let size = match std::fs::metadata(&path) {
            Ok(m) => format!("{} B", m.len()),
            Err(_) => "size absent".to_string(),
        };
        println!("{flag} {path}: {} records, {size}", records.len());
        for (idx, name) in labels {
            let mut times: Vec<(f64, u32)> = records
                .iter()
                .filter(|(_, _, i)| i == idx)
                .map(|&(t, _, _)| (t, 0))
                .collect();
            times.sort_by(|a, b| a.0.total_cmp(&b.0));
            if times.is_empty() {
                println!("  {name:<6} absent");
                continue;
            }
            let first = times[0].0;
            let last = times[times.len() - 1].0;
            let to_unix = |t: f64| lsk.as_ref().and_then(|l| l.tdb_to_unix(t)).unwrap_or(t);
            let step = median_step(&times);
            println!(
                "  {name:<6} n={:<8} {} .. {}  median dt = {}",
                times.len(),
                utc_day(to_unix(first)),
                utc_day(to_unix(last)),
                step.map_or("absent".to_string(), |v| format!("{v:.3} s"))
            );
        }
    }
}
