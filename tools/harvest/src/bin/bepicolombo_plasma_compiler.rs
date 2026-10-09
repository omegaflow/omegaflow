use omegaflow::archivar::fetch_raw;
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::upload_release;

const MAGIC: &[u8; 4] = b"BCPL";
const CHANNELS: [&str; 5] = ["xx", "xka", "kaka", "up", "down"];
const DATA_TYPES: [u8; 2] = [2, 40];
const NETLOC: &str = "zenodo.org";
const COMPILER: &str = "tools/harvest/src/bin/bepicolombo_plasma_compiler.rs";
const DEFAULT_URL: &str = "https://zenodo.org/api/records/17813314/files/SCE1_2021_HGA_MEANR4ONR6_KaTSelfCal_DST1Cal_60s_XX_KK_MT_plasmacalib.txt/content";
const DEFAULT_OUT: &str = "data/zenodo.org/bepicolombo_plasma.bin";

#[derive(Clone, Debug, PartialEq)]
struct PlasmaRow {
    dtype: u8,
    epoch: f64,
    values: [f64; 5],
}

#[derive(Clone, Debug, PartialEq)]
struct Series {
    dtype: u8,
    channel: u8,
    pairs: Vec<(f64, f64)>,
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn unit_of(dtype: u8) -> Option<&'static str> {
    match dtype {
        2 => Some("Hz"),
        40 => Some("km"),
        _ => None,
    }
}

fn parse_text(text: &str) -> Option<Vec<PlasmaRow>> {
    let mut rows: Vec<PlasmaRow> = Vec::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let mut it = line.split_whitespace();
        let Some(dtype) = it.next().and_then(|t| t.parse::<u8>().ok()) else {
            continue;
        };
        if unit_of(dtype).is_none() {
            continue;
        }
        let Some(epoch) = it.next().and_then(|t| t.parse::<f64>().ok()) else {
            continue;
        };
        if !epoch.is_finite() {
            continue;
        }
        let mut values = [0.0f64; 5];
        let mut complete = true;
        for slot in values.iter_mut() {
            match it.next().and_then(|t| t.parse::<f64>().ok()) {
                Some(v) if v.is_finite() => *slot = v,
                _ => {
                    complete = false;
                    break;
                }
            }
        }
        if !complete || it.next().is_some() {
            continue;
        }
        rows.push(PlasmaRow {
            dtype,
            epoch,
            values,
        });
    }
    if rows.is_empty() { None } else { Some(rows) }
}

fn to_series(rows: &[PlasmaRow]) -> Vec<Series> {
    let mut out: Vec<Series> = Vec::new();
    for dtype in DATA_TYPES {
        for channel in 0..CHANNELS.len() {
            let pairs: Vec<(f64, f64)> = rows
                .iter()
                .filter(|r| r.dtype == dtype)
                .map(|r| (r.epoch, r.values[channel]))
                .collect();
            if !pairs.is_empty() {
                out.push(Series {
                    dtype,
                    channel: channel as u8,
                    pairs,
                });
            }
        }
    }
    out
}

fn write_bin(series: &[Series]) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::new();
    out.extend_from_slice(MAGIC);
    out.extend_from_slice(&(series.len() as u32).to_le_bytes());
    for s in series {
        out.push(s.dtype);
        out.push(s.channel);
        out.extend_from_slice(&0u16.to_le_bytes());
        out.extend_from_slice(&(s.pairs.len() as u32).to_le_bytes());
        for (t, v) in &s.pairs {
            out.extend_from_slice(&t.to_le_bytes());
            out.extend_from_slice(&v.to_le_bytes());
        }
    }
    out
}

fn parse_bin(bytes: &[u8]) -> Option<Vec<Series>> {
    if bytes.get(0..4)? != MAGIC {
        return None;
    }
    let mut pos = 4usize;
    let head = bytes.get(pos..pos + 4)?;
    let n = u32::from_le_bytes(head.try_into().ok()?) as usize;
    pos += 4;
    let mut out: Vec<Series> = Vec::with_capacity(n);
    for _ in 0..n {
        let dtype = *bytes.get(pos)?;
        let channel = *bytes.get(pos + 1)?;
        pos += 2;
        bytes.get(pos..pos + 2)?;
        pos += 2;
        let count = u32::from_le_bytes(bytes.get(pos..pos + 4)?.try_into().ok()?) as usize;
        pos += 4;
        let mut pairs: Vec<(f64, f64)> = Vec::with_capacity(count);
        for _ in 0..count {
            let t = f64::from_le_bytes(bytes.get(pos..pos + 8)?.try_into().ok()?);
            pos += 8;
            let v = f64::from_le_bytes(bytes.get(pos..pos + 8)?.try_into().ok()?);
            pos += 8;
            pairs.push((t, v));
        }
        out.push(Series {
            dtype,
            channel,
            pairs,
        });
    }
    if pos != bytes.len() {
        return None;
    }
    Some(out)
}

fn emit(text: &str, out: &str, source: &str) {
    let Some(rows) = parse_text(text) else {
        eprintln!("{source}: no 7-column plasma row parses — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    };
    let series = to_series(&rows);
    if series.is_empty() {
        eprintln!(
            "{source}: no channel carries a unit (data type 2|40) — the bin stays unwritten (0 honored)"
        );
        std::process::exit(1);
    }
    let bytes_out = write_bin(&series);
    if let Some(parent) = std::path::Path::new(out).parent()
        && !parent.as_os_str().is_empty()
    {
        std::fs::create_dir_all(parent).ok();
    }
    if std::fs::write(out, &bytes_out).is_err() {
        eprintln!("write {out} returned void");
        std::process::exit(1);
    }
    match parse_bin(&bytes_out) {
        Some(parsed) if parsed == series => {
            eprintln!(
                "{out}: {} rows, {} series, {} bytes, roundtrip parses identical",
                rows.len(),
                series.len(),
                bytes_out.len()
            );
        }
        Some(parsed) => {
            eprintln!(
                "{out}: roundtrip parses {} series but differs from the emitted set",
                parsed.len()
            );
            std::process::exit(1);
        }
        None => {
            eprintln!("{out}: roundtrip parse void — the bin stays unverified");
            std::process::exit(1);
        }
    }
    let out_name = match std::path::Path::new(out).file_name() {
        Some(n) => n.to_string_lossy().into_owned(),
        None => out.to_string(),
    };
    println!("url https://github.com/omegaflow/sources/releases/download/{NETLOC}/{out_name}");
    println!("origin {source}");
    println!("compiler {COMPILER}");
    println!("sha256 {}", sha256_hex(&bytes_out));
    println!("format bepicolombo_plasma");
    println!("epoch seconds past J2000 (as published)");
    for s in &series {
        let unit = match unit_of(s.dtype) {
            Some(u) => u,
            None => continue,
        };
        println!(
            "series dtype {} channel {} unit {} pairs {}",
            s.dtype,
            CHANNELS[s.channel as usize],
            unit,
            s.pairs.len()
        );
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = match arg_value(&args, "--out") {
        Some(v) => v,
        None => DEFAULT_OUT.to_string(),
    };
    let (text, source) = match arg_value(&args, "--file") {
        Some(path) => match std::fs::read_to_string(&path) {
            Ok(t) => (t, path),
            Err(_) => {
                eprintln!("{path} unreadable — the bin stays unwritten (0 honored)");
                std::process::exit(1);
            }
        },
        None => {
            let url = match arg_value(&args, "--url") {
                Some(u) => u,
                None => DEFAULT_URL.to_string(),
            };
            match fetch_raw(&url, None, &[]) {
                Some(t) => (t, url),
                None => {
                    eprintln!("{url} fetch void — the bin stays unwritten (0 honored)");
                    std::process::exit(1);
                }
            }
        }
    };
    emit(&text, &out, &source);
    if ci_mode && !upload_release(NETLOC, &out) {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn measured() -> &'static str {
        " 2   668655362.000   0.04883576   0.00000000   0.00809027   0.04156590   0.00000000\n\
         2   668655422.000   0.04346983   0.00000000   0.00719753   0.03699875   0.00000000\n\
         40  668655362.000   0.00100000   0.00000000   0.00020000   0.00080000   0.00000000\n\
         7   668655362.000   0.00000000   0.00000000   0.00000000   0.00000000   0.00000000\n\
         2   668655482.000   0.03073454   0.00000000   0.00508739   0.02615929\n"
    }

    #[test]
    fn unit_follows_the_data_type() {
        assert_eq!(unit_of(2), Some("Hz"));
        assert_eq!(unit_of(40), Some("km"));
        assert_eq!(unit_of(7), None);
    }

    #[test]
    fn parse_reads_the_measured_header_and_skips_unclassifiable_rows() {
        let rows = parse_text(measured()).expect("the measured rows parse");
        assert_eq!(rows.len(), 3);
        assert_eq!(rows[0].dtype, 2);
        assert_eq!(rows[0].epoch, 668655362.0);
        assert_eq!(
            rows[0].values,
            [0.04883576, 0.0, 0.00809027, 0.0415659, 0.0]
        );
        assert_eq!(rows[1].dtype, 2);
        assert_eq!(rows[2].dtype, 40);
        assert_eq!(rows[2].values, [0.001, 0.0, 0.0002, 0.0008, 0.0]);
    }

    #[test]
    fn to_series_groups_by_data_type_and_channel() {
        let rows = parse_text(measured()).unwrap();
        let series = to_series(&rows);
        assert_eq!(series.len(), 10);
        let xx2 = series
            .iter()
            .find(|s| s.dtype == 2 && s.channel == 0)
            .expect("the type 2 X/X series stands");
        assert_eq!(xx2.pairs.len(), 2);
        assert_eq!(xx2.pairs[0], (668655362.0, 0.04883576));
        let kaka40 = series
            .iter()
            .find(|s| s.dtype == 40 && s.channel == 2)
            .expect("the type 40 Ka/Ka series stands");
        assert_eq!(kaka40.pairs.len(), 1);
        assert_eq!(kaka40.pairs[0], (668655362.0, 0.0002));
    }

    #[test]
    fn bin_roundtrip_preserves_series() {
        let rows = parse_text(measured()).unwrap();
        let series = to_series(&rows);
        let bytes = write_bin(&series);
        assert_eq!(parse_bin(&bytes).as_deref(), Some(series.as_slice()));
    }

    #[test]
    fn parse_bin_rejects_foreign_magic_and_short_body() {
        assert!(parse_bin(b"XXXX").is_none());
        assert!(parse_bin(b"BCPL").is_none());
        let rows = parse_text(measured()).unwrap();
        let good = write_bin(&to_series(&rows));
        let short = good[..good.len() - 1].to_vec();
        assert!(parse_bin(&short).is_none());
    }
}
