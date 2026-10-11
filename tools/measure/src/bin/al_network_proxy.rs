use omegaflow::intermagnet::parse_xyz_bin;
use std::collections::BTreeMap;

const MAGIC: &[u8; 4] = b"IMXZ";

fn write_imdt(records: &[(f64, f64, u32)]) -> Vec<u8> {
    let mut out = Vec::with_capacity(8 + records.len() * 16);
    out.extend_from_slice(b"IMDT");
    out.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for (t, v, _) in records {
        out.extend_from_slice(&t.to_le_bytes());
        out.extend_from_slice(&v.to_le_bytes());
    }
    out
}

fn arg_value(args: &[String], key: &str) -> Option<String> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .cloned()
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

fn block_median_baseline(values: &[f64], block: usize) -> Vec<f64> {
    let mut out = Vec::with_capacity(values.len());
    let mut i = 0;
    while i < values.len() {
        let end = (i + block).min(values.len());
        let mut vals: Vec<f64> = values[i..end].to_vec();
        let Some(med) = median(&mut vals) else {
            i = end;
            continue;
        };
        for _ in i..end {
            out.push(med);
        }
        i = end;
    }
    out
}

fn station_dh(samples: &[(f64, f64, f64, f64)], decl_deg: f64, block: usize) -> Vec<(f64, f64)> {
    let h: Vec<f64> = samples
        .iter()
        .map(|(_, x, y, _)| rotate_h(*x, *y, decl_deg))
        .collect();
    let baseline = block_median_baseline(&h, block);
    samples
        .iter()
        .zip(h.iter())
        .zip(baseline.iter())
        .map(|((s, hv), b)| (s.0, hv - b))
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
            eprintln!("al_network_proxy: manifest line needs `CODE decl_deg bin-path`: {line}");
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
        eprintln!("al_network_proxy needs --manifest <file> (lines: CODE decl_deg xyz-bin-path)");
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
        let Ok(bytes) = std::fs::read(path) else {
            eprintln!("al_network_proxy: {code} {path} read void");
            std::process::exit(1);
        };
        let Some(samples) = parse_xyz_bin(&bytes) else {
            eprintln!(
                "al_network_proxy: {code} {path} is not an IMXZ bin (magic {})",
                String::from_utf8_lossy(MAGIC)
            );
            std::process::exit(1);
        };
        if samples.is_empty() {
            eprintln!("al_network_proxy: {code} {path} carries no sample");
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
        let records: Vec<(f64, f64, u32)> = env.iter().map(|(t, v)| (*t, *v, 0)).collect();
        match std::fs::write(&out, write_imdt(&records)) {
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
        let b = block_median_baseline(&[10.0, 12.0, 14.0, 2.0], 3);
        assert_eq!(b, vec![12.0, 12.0, 12.0, 2.0]);
    }

    #[test]
    fn the_dh_series_removes_the_block_baseline() {
        let samples = vec![
            (0.0, 10.0, 0.0, 0.0),
            (60.0, 12.0, 0.0, 0.0),
            (120.0, 14.0, 0.0, 0.0),
        ];
        let dh = station_dh(&samples, 0.0, 3);
        assert_eq!(dh, vec![(0.0, -2.0), (60.0, 0.0), (120.0, 2.0)]);
    }

    #[test]
    fn the_lower_envelope_takes_the_minimum_and_gates_on_coverage() {
        let a = vec![(0.0, 5.0), (60.0, -3.0), (120.0, 7.0)];
        let b = vec![(0.0, 1.0), (60.0, -8.0)];
        assert_eq!(envelope(&[a, b], 2), vec![(0.0, 1.0), (60.0, -8.0)]);
    }

    #[test]
    fn a_timestamp_with_one_station_is_not_an_envelope() {
        let a = vec![(0.0, 5.0), (60.0, -3.0)];
        let b = vec![(0.0, 1.0)];
        assert_eq!(envelope(&[a, b], 2), vec![(0.0, 1.0)]);
    }

    #[test]
    fn the_envelope_imdt_series_round_trips() {
        let env = vec![(0.0, -8.0), (60.0, -3.0)];
        let records: Vec<(f64, f64, u32)> = env.iter().map(|(t, v)| (*t, *v, 0)).collect();
        let bytes = write_imdt(&records);
        let parsed = omegaflow::intermagnet::parse_bin(&bytes).expect("parse void");
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[1].1, -3.0);
    }
}
