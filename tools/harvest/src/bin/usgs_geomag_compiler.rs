use omegaflow::archivar::json::{JsonVal, parse_json, scalar_of};
use omegaflow::cdn::upload_release;
use omegaflow::lsk::days_from_civil;
use std::process::Command;

const BASE: &str = "https://geomag.usgs.gov/ws/data/";
const SECONDS_PER_DAY: f64 = 86_400.0;

const MAGIC: [u8; 4] = *b"UGE1";
const ELEMENT_MAX: u32 = 1;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn fetch(url: &str) -> Option<String> {
    let out = Command::new("curl")
        .arg("-sSf")
        .arg("--retry")
        .arg("2")
        .arg("--max-time")
        .arg("300")
        .arg(url)
        .output()
        .ok()?;
    if out.status.success() {
        String::from_utf8(out.stdout).ok()
    } else {
        eprintln!(
            "fetch http {}: {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        None
    }
}

fn iso_to_unix(s: &str) -> Option<f64> {
    let (date, rest) = s.split_once('T')?;
    let (y, md) = date.split_once('-')?;
    let (m, d) = md.split_once('-')?;
    let days = days_from_civil(y.parse().ok()?, m.parse().ok()?, d.parse().ok()?)?;
    let time = rest.strip_suffix('Z').unwrap_or(rest);
    let (hms, frac) = match time.split_once('.') {
        Some((a, b)) => (a, b.parse::<f64>().ok()?),
        None => (time, 0.0),
    };
    let mut parts = hms.split(':');
    let h: i64 = parts.next()?.parse().ok()?;
    let mi: i64 = parts.next()?.parse().ok()?;
    let sec: i64 = parts.next()?.parse().ok()?;
    Some((days * SECONDS_PER_DAY as i64 + h * 3600 + mi * 60 + sec) as f64 + frac)
}

enum ParallelZip {
    Zipped(Vec<(f64, f64, u32)>),
    Riss {
        times_len: usize,
        values_len: usize,
        k: usize,
    },
}

fn zip_parallel_arrays(times: &[f64], series: &[(u32, Vec<Option<f64>>)]) -> ParallelZip {
    for (_, values) in series {
        if values.len() != times.len() {
            return ParallelZip::Riss {
                times_len: times.len(),
                values_len: values.len(),
                k: times.len().min(values.len()),
            };
        }
    }
    let mut out = Vec::new();
    for (element, values) in series {
        for (i, t) in times.iter().enumerate() {
            if let Some(Some(v)) = values.get(i) {
                if v.is_finite() {
                    out.push((*t, *v, *element));
                }
            }
        }
    }
    ParallelZip::Zipped(out)
}

fn write_bin(records: &[(f64, f64, u32)]) -> Vec<u8> {
    let mut buf = Vec::with_capacity(8 + records.len() * 20);
    buf.extend_from_slice(&MAGIC);
    buf.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for (t, val, element) in records {
        buf.extend_from_slice(&t.to_le_bytes());
        buf.extend_from_slice(&val.to_le_bytes());
        buf.extend_from_slice(&element.to_le_bytes());
    }
    buf
}

fn parse_bin(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    if bytes.len() < 8 || bytes[0..4] != MAGIC {
        return None;
    }
    let n = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    if n > (bytes.len() - 8) / 20 {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    let mut off = 8usize;
    for _ in 0..n {
        let t = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        off += 8;
        let val = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        off += 8;
        let element = u32::from_le_bytes(bytes.get(off..off + 4)?.try_into().ok()?);
        off += 4;
        if element > ELEMENT_MAX {
            return None;
        }
        out.push((t, val, element));
    }
    Some(out)
}

fn element_index(name: &str) -> Option<u32> {
    match name {
        "E-E" => Some(0),
        "E-N" => Some(1),
        _ => None,
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let station = match arg_value(&args, "--station") {
        Some(v) => v,
        None => {
            eprintln!("--station carries no IAGA code — the bin stays unwritten");
            std::process::exit(2);
        }
    };
    let out = match arg_value(&args, "--out") {
        Some(v) => v,
        None => omegaflow::archivar::cache_root()
            .join(format!("usgs_geomag_efield_{station}.bin"))
            .to_string_lossy()
            .into_owned(),
    };
    let start = match arg_value(&args, "--start") {
        Some(v) => v,
        None => {
            eprintln!("--start carries no window edge — the bin stays unwritten");
            std::process::exit(2);
        }
    };
    let stop = match arg_value(&args, "--stop") {
        Some(v) => v,
        None => {
            eprintln!("--stop carries no window edge — the bin stays unwritten");
            std::process::exit(2);
        }
    };
    let url = format!(
        "{BASE}?id={station}&elements=E-E,E-N&format=json&starttime={start}&endtime={stop}"
    );
    let Some(text) = fetch(&url) else {
        eprintln!("usgs_geomag {station}: fetch void — the bin stays unwritten");
        std::process::exit(1);
    };
    let Some(JsonVal::Obj(root)) = parse_json(&text) else {
        eprintln!("usgs_geomag {station}: body carries no JSON object — the bin stays unwritten");
        std::process::exit(1);
    };
    let times_arr = match root.get("times") {
        Some(JsonVal::Arr(a)) => a,
        _ => {
            eprintln!("usgs_geomag {station}: times carries no array — the bin stays unwritten");
            std::process::exit(1);
        }
    };
    let mut times: Vec<f64> = Vec::with_capacity(times_arr.len());
    for t in times_arr {
        match t {
            JsonVal::Str(s) => match iso_to_unix(s) {
                Some(u) => times.push(u),
                None => {
                    eprintln!(
                        "usgs_geomag {station}: time {s} carries no unix second — the bin stays unwritten"
                    );
                    std::process::exit(1);
                }
            },
            _ => {
                eprintln!(
                    "usgs_geomag {station}: times carries a non-string member — the bin stays unwritten"
                );
                std::process::exit(1);
            }
        }
    }
    let values_arr = match root.get("values") {
        Some(JsonVal::Arr(a)) => a,
        _ => {
            eprintln!("usgs_geomag {station}: values carries no array — the bin stays unwritten");
            std::process::exit(1);
        }
    };
    let mut series: Vec<(u32, Vec<Option<f64>>)> = Vec::new();
    for entry in values_arr {
        let JsonVal::Obj(o) = entry else {
            eprintln!("usgs_geomag {station}: values carries a non-object member — skipped");
            continue;
        };
        let element = match o
            .get("metadata")
            .and_then(|m| match m {
                JsonVal::Obj(mm) => mm.get("element"),
                _ => None,
            })
            .and_then(|e| match e {
                JsonVal::Str(s) => Some(s.as_str()),
                _ => None,
            }) {
            Some(name) => name,
            None => {
                eprintln!("usgs_geomag {station}: a series carries no element name — skipped");
                continue;
            }
        };
        let Some(idx) = element_index(element) else {
            eprintln!("usgs_geomag {station}: element {element} carries no index — skipped");
            continue;
        };
        let vals = match o.get("values") {
            Some(JsonVal::Arr(a)) => a,
            _ => {
                eprintln!(
                    "usgs_geomag {station}: element {element} carries no values array — skipped"
                );
                continue;
            }
        };
        let mut col: Vec<Option<f64>> = Vec::with_capacity(vals.len());
        for v in vals {
            col.push(scalar_of(v));
        }
        series.push((idx, col));
    }
    let raw = match zip_parallel_arrays(&times, &series) {
        ParallelZip::Zipped(raw) => raw,
        ParallelZip::Riss {
            times_len,
            values_len,
            k,
        } => {
            eprintln!(
                "usgs_geomag {station}: parallel arrays diverge (times {times_len}, values {values_len}, first divergent k {k}) — the whole set is a Riss, no pad/truncate, the bin stays unwritten"
            );
            std::process::exit(1);
        }
    };
    if raw.is_empty() {
        eprintln!(
            "usgs_geomag {station}: no value carries a finite east/north measurement — the bin stays unwritten (0 honored)"
        );
        std::process::exit(1);
    }
    eprintln!(
        "usgs_geomag {station}: {} records of {} times and {} series, epoch in UTC unix s, value in mV/km, element 0=E-E 1=E-N",
        raw.len(),
        times.len(),
        series.len()
    );
    let bytes = write_bin(&raw);
    if let Some(parent) = std::path::Path::new(&out).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if std::fs::write(&out, &bytes).is_err() {
        eprintln!("write {} returned void", out);
        std::process::exit(1);
    }
    match parse_bin(&bytes) {
        Some(parsed) => {
            eprintln!("{}: {} records, roundtrip parses", out, parsed.len());
        }
        None => {
            eprintln!("{}: roundtrip parse void — the bin stays unverified", out);
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_release("geomag.usgs.gov", &out) {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zip_refuses_divergent_parallel_arrays() {
        let times = vec![1.0, 2.0, 3.0];
        let series = vec![(0u32, vec![Some(1.0), Some(2.0)])];
        match zip_parallel_arrays(&times, &series) {
            ParallelZip::Riss {
                times_len,
                values_len,
                k,
            } => {
                assert_eq!(times_len, 3);
                assert_eq!(values_len, 2);
                assert_eq!(k, 2);
            }
            ParallelZip::Zipped(_) => panic!("divergent parallel arrays read as Zipped"),
        }
    }

    #[test]
    fn zip_keeps_equal_length_arrays() {
        let times = vec![1.0, 2.0];
        let series = vec![
            (0u32, vec![Some(1.0), None]),
            (1u32, vec![Some(3.0), Some(4.0)]),
        ];
        match zip_parallel_arrays(&times, &series) {
            ParallelZip::Zipped(records) => {
                assert_eq!(records, vec![(1.0, 1.0, 0), (1.0, 3.0, 1), (2.0, 4.0, 1)]);
            }
            ParallelZip::Riss { .. } => panic!("equal-length arrays read as Riss"),
        }
    }
}
