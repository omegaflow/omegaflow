use omegaflow::archivar::{JsonVal, parse_json, scalar_of};
use std::collections::BTreeMap;

const DAY: f64 = 86400.0;
const HOUR: f64 = 3600.0;
const MINUTE: f64 = 60.0;
const MAGIC: &[u8; 4] = b"IMDT";

fn arg_value(args: &[String], key: &str) -> Option<String> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn iso_to_unix(s: &str) -> Option<f64> {
    let s = s.trim();
    let (date, time) = match s.split_once('T') {
        Some((d, t)) => (d, t),
        None => s.split_once(' ')?,
    };
    let mut dp = date.split('-');
    let y: i64 = dp.next()?.parse().ok()?;
    let m: i64 = dp.next()?.parse().ok()?;
    let d: i64 = dp.next()?.parse().ok()?;
    let t = time
        .split(|c: char| c == '.' || c == 'Z' || c == 'z')
        .next()?;
    let mut tp = t.split(':');
    let hh: i64 = tp.next()?.parse().ok()?;
    let mm: i64 = match tp.next() {
        Some(v) => v,
        None => "0",
    }
    .parse()
    .ok()?;
    let ss: i64 = match tp.next() {
        Some(v) => v,
        None => "0",
    }
    .parse()
    .ok()?;
    let a = (14 - m) / 12;
    let yy = y + 4800 - a;
    let jdn =
        d + (153 * (m + 12 * a - 3) + 2) / 5 + 365 * yy + yy / 4 - yy / 100 + yy / 400 - 32045;
    Some((jdn - 2440588) as f64 * DAY + hh as f64 * HOUR + mm as f64 * MINUTE + ss as f64)
}

fn median(values: &mut [f64]) -> Option<f64> {
    if values.is_empty() {
        return None;
    }
    values.sort_by(|a, b| a.total_cmp(b));
    let n = values.len();
    Some(if n % 2 == 1 {
        values[n / 2]
    } else {
        0.5 * (values[n / 2 - 1] + values[n / 2])
    })
}

fn rotate_h(x: f64, y: f64, decl_deg: f64) -> f64 {
    let r = decl_deg.to_radians();
    x * r.cos() + y * r.sin()
}

fn parse_station_json(text: &str) -> Vec<(f64, f64, f64, f64)> {
    let Some(JsonVal::Obj(root)) = parse_json(text) else {
        return Vec::new();
    };
    let Some(JsonVal::Arr(data)) = root.get("data") else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for row in data {
        let JsonVal::Arr(cells) = row else {
            continue;
        };
        let Some(t) = cells.first().and_then(|c| match c {
            JsonVal::Str(s) => iso_to_unix(s),
            _ => None,
        }) else {
            continue;
        };
        let JsonVal::Arr(vec) = &cells[1] else {
            continue;
        };
        let x = vec.first().and_then(scalar_of);
        let y = vec.get(1).and_then(scalar_of);
        let z = vec.get(2).and_then(scalar_of);
        if let (Some(x), Some(y), Some(z)) = (x, y, z)
            && x.is_finite()
            && y.is_finite()
            && z.is_finite()
            && x != 99999.0
            && y != 99999.0
            && z != 99999.0
        {
            out.push((t, x, y, z));
        }
    }
    out
}

fn block_median_baseline(samples: &[(f64, f64)], block: usize) -> Vec<(f64, f64)> {
    let mut baseline = Vec::with_capacity(samples.len());
    let mut i = 0;
    while i < samples.len() {
        let end = (i + block).min(samples.len());
        let mut vals: Vec<f64> = samples[i..end].iter().map(|(_, v)| *v).collect();
        let Some(med) = median(&mut vals) else {
            i = end;
            continue;
        };
        for (t, _) in &samples[i..end] {
            baseline.push((*t, med));
        }
        i = end;
    }
    baseline
}

fn station_dh(samples: &[(f64, f64, f64, f64)], decl_deg: f64, block: usize) -> Vec<(f64, f64)> {
    let h: Vec<(f64, f64)> = samples
        .iter()
        .map(|(t, x, y, _)| (*t, rotate_h(*x, *y, decl_deg)))
        .collect();
    let baseline = block_median_baseline(&h, block);
    h.iter()
        .zip(baseline.iter())
        .map(|((t, hv), (_, b))| (*t, hv - b))
        .collect()
}

fn envelope(stations: &[Vec<(f64, f64)>], min_stations: usize) -> Vec<(f64, f64)> {
    let mut acc: BTreeMap<i64, Vec<f64>> = BTreeMap::new();
    for s in stations {
        for (t, v) in s {
            if v.is_finite() {
                acc.entry(*t as i64).or_default().push(*v);
            }
        }
    }
    let mut out = Vec::new();
    for (t, vals) in acc {
        if vals.len() >= min_stations {
            let mut sorted = vals;
            sorted.sort_by(|a, b| a.total_cmp(b));
            out.push((t as f64, sorted[0]));
        }
    }
    out
}

fn write_imdt(records: &[(f64, f64)]) -> Vec<u8> {
    let mut out = Vec::with_capacity(8 + records.len() * 16);
    out.extend_from_slice(MAGIC);
    out.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for (t, v) in records {
        out.extend_from_slice(&t.to_le_bytes());
        out.extend_from_slice(&v.to_le_bytes());
    }
    out
}

fn load_manifest(path: &str) -> Vec<(String, f64, String)> {
    let Ok(text) = std::fs::read_to_string(path) else {
        eprintln!("al_network_proxy: {path} read void");
        std::process::exit(1);
    };
    let mut out = Vec::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 3 {
            eprintln!("al_network_proxy: manifest line needs `CODE decl_deg path`: {line}");
            std::process::exit(1);
        }
        let Ok(decl) = parts[1].parse::<f64>() else {
            eprintln!("al_network_proxy: declination is no number: {}", parts[1]);
            std::process::exit(1);
        };
        out.push((parts[0].to_string(), decl, parts[2].to_string()));
    }
    out
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some(manifest_path) = arg_value(&args, "--manifest") else {
        eprintln!("al_network_proxy needs --manifest <file> (lines: CODE decl_deg hapi-json-path)");
        std::process::exit(2);
    };
    let Some(block) = arg_value(&args, "--baseline-block").and_then(|v| v.parse::<usize>().ok())
    else {
        eprintln!("al_network_proxy needs --baseline-block <samples> (block-median baseline)");
        std::process::exit(2);
    };
    if block == 0 {
        eprintln!("--baseline-block must be positive");
        std::process::exit(2);
    }
    let Some(min_stations) =
        arg_value(&args, "--min-stations").and_then(|v| v.parse::<usize>().ok())
    else {
        eprintln!("al_network_proxy needs --min-stations <n> (an envelope over 1 station is none)");
        std::process::exit(2);
    };
    if min_stations < 2 {
        eprintln!("--min-stations must be at least 2");
        std::process::exit(2);
    }
    let stations = load_manifest(&manifest_path);
    if stations.is_empty() {
        eprintln!("al_network_proxy: manifest carries no station");
        std::process::exit(1);
    }
    let mut series: Vec<Vec<(f64, f64)>> = Vec::new();
    for (code, decl, path) in &stations {
        let Ok(text) = std::fs::read_to_string(path) else {
            eprintln!("al_network_proxy: {code} {path} read void");
            std::process::exit(1);
        };
        let samples = parse_station_json(&text);
        if samples.is_empty() {
            eprintln!("al_network_proxy: {code} {path} carries no finite vector sample");
            std::process::exit(1);
        }
        println!(
            "station {code} decl {decl} samples {} baseline_block {block}",
            samples.len()
        );
        series.push(station_dh(&samples, *decl, block));
    }
    let env = envelope(&series, min_stations);
    if env.is_empty() {
        eprintln!(
            "al_network_proxy: envelope empty — no timestamp carries {min_stations} station(s)"
        );
        std::process::exit(1);
    }
    println!(
        "envelope samples {} min_stations {min_stations} stations {}",
        env.len(),
        series.len()
    );
    if let Some(out) = arg_value(&args, "--out") {
        match std::fs::write(&out, write_imdt(&env)) {
            Ok(()) => println!("wrote {} envelope sample(s) → {out}", env.len()),
            Err(e) => {
                eprintln!("al_network_proxy: {out} write void: {e}");
                std::process::exit(1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotation_at_zero_declination_is_x() {
        assert_eq!(rotate_h(3.0, 4.0, 0.0), 3.0);
    }

    #[test]
    fn rotation_at_ninety_degrees_is_y() {
        assert!((rotate_h(3.0, 4.0, 90.0) - 4.0).abs() < 1e-12);
    }

    #[test]
    fn the_block_baseline_is_the_block_median() {
        let samples = vec![(0.0, 10.0), (60.0, 12.0), (120.0, 14.0), (180.0, 2.0)];
        let b = block_median_baseline(&samples, 3);
        assert_eq!(b[0].1, 12.0);
        assert_eq!(b[2].1, 12.0);
        assert_eq!(b[3].1, 2.0);
    }

    #[test]
    fn the_lower_envelope_takes_the_minimum_and_gates_on_coverage() {
        let a = vec![(0.0, 5.0), (60.0, -3.0), (120.0, 7.0)];
        let b = vec![(0.0, 1.0), (60.0, -8.0)];
        let env = envelope(&[a, b], 2);
        assert_eq!(env, vec![(0.0, 1.0), (60.0, -8.0)]);
    }

    #[test]
    fn a_timestamp_with_one_station_is_not_an_envelope() {
        let a = vec![(0.0, 5.0), (60.0, -3.0)];
        let b = vec![(0.0, 1.0)];
        let env = envelope(&[a, b], 2);
        assert_eq!(env, vec![(0.0, 1.0)]);
    }

    #[test]
    fn the_hapi_json_parses_to_time_and_components() {
        let text = r#"{"data":[["2026-01-01T00:00:00Z",[1.0,2.0,3.0,4.0]],["2026-01-01T00:01:00Z",[5.0,6.0,7.0,8.0]]]}"#;
        let s = parse_station_json(text);
        assert_eq!(s.len(), 2);
        assert_eq!(s[0].1, 1.0);
        assert_eq!(s[1].3, 7.0);
    }

    #[test]
    fn the_fill_value_is_absent_not_a_sample() {
        let text = r#"{"data":[["2026-01-01T00:00:00Z",[99999.0,2.0,3.0,4.0]]]}"#;
        assert!(parse_station_json(text).is_empty());
    }

    #[test]
    fn the_imdt_series_round_trips_through_the_parser() {
        let env = vec![(0.0, -8.0), (60.0, -3.0)];
        let bytes = write_imdt(&env);
        let parsed = omegaflow::intermagnet::parse_bin(&bytes).expect("parse void");
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[1].1, -3.0);
    }
}
