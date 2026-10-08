use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::hapi_csv::parse_iso_seconds;
use omegaflow::json::{JsonVal, jnum, jpath_val, jstr, parse_json};

const MAGIC: &[u8; 4] = b"CRTI";
const FIELDS: usize = 4;
const NETLOC: &str = "data.impc.dlr.de";
const SOURCE_URL: &str = "https://data.impc.dlr.de/roti/DLR_GNSS_GCG_L3_ROTI-MAX_NC_GLOBAL/latest/DLR_GNSS_GCG_L3_ROTI-MAX_NC_GLOBAL_latest_D.json";
const COMPILER: &str = "tools/harvest/src/bin/impc_roti_compiler.rs";

const LAT_INDEX_RISES_NORTHWARD: bool = true;
const LON_INDEX_RISES_EASTWARD: bool = true;

struct GridMeta {
    min_lat: f64,
    max_lat: f64,
    delta_lat: f64,
    min_lon: f64,
    max_lon: f64,
    delta_lon: f64,
}

fn coverage(root: &JsonVal) -> Option<GridMeta> {
    Some(GridMeta {
        min_lat: jnum(root, "metadata.spatial_coverage.min_lat")?,
        max_lat: jnum(root, "metadata.spatial_coverage.max_lat")?,
        delta_lat: jnum(root, "metadata.spatial_coverage.delta_lat")?,
        min_lon: jnum(root, "metadata.spatial_coverage.min_lon")?,
        max_lon: jnum(root, "metadata.spatial_coverage.max_lon")?,
        delta_lon: jnum(root, "metadata.spatial_coverage.delta_lon")?,
    })
}

fn bin_count(min: f64, max: f64, delta: f64) -> Option<usize> {
    if !delta.is_finite() || delta <= 0.0 {
        return None;
    }
    let n = ((max - min) / delta).round();
    if n.is_finite() && n >= 1.0 {
        Some(n as usize)
    } else {
        None
    }
}

fn latitude(j: usize, m: &GridMeta) -> f64 {
    if LAT_INDEX_RISES_NORTHWARD {
        m.min_lat + (j as f64 + 0.5) * m.delta_lat
    } else {
        m.max_lat - (j as f64 + 0.5) * m.delta_lat
    }
}

fn longitude(i: usize, m: &GridMeta) -> f64 {
    if LON_INDEX_RISES_EASTWARD {
        m.min_lon + (i as f64 + 0.5) * m.delta_lon
    } else {
        m.max_lon - (i as f64 + 0.5) * m.delta_lon
    }
}

fn parse_records(root: &JsonVal) -> Option<Vec<[f64; FIELDS]>> {
    let meta = coverage(root)?;
    let n_lat = bin_count(meta.min_lat, meta.max_lat, meta.delta_lat)?;
    let n_lon = bin_count(meta.min_lon, meta.max_lon, meta.delta_lon)?;
    let start = jstr(root, "metadata.temporal_coverage.start_time")?;
    let t_unix = parse_iso_seconds(&start)?;
    let rows = match jpath_val(root, "data")? {
        JsonVal::Arr(a) => a,
        _ => return None,
    };
    let cols = match rows.first() {
        Some(JsonVal::Arr(a)) => a.len(),
        _ => return None,
    };
    let lon_first = rows.len() == n_lon && cols == n_lat;
    let lat_first = rows.len() == n_lat && cols == n_lon;
    if !lon_first && !lat_first {
        return None;
    }
    let mut out: Vec<[f64; FIELDS]> = Vec::new();
    for (i, row) in rows.iter().enumerate() {
        let JsonVal::Arr(cells) = row else {
            continue;
        };
        for (j, cell) in cells.iter().enumerate() {
            let JsonVal::Num(value) = cell else {
                continue;
            };
            if !value.is_finite() {
                continue;
            }
            let (lat_idx, lon_idx) = if lon_first { (j, i) } else { (i, j) };
            if lat_idx >= n_lat || lon_idx >= n_lon {
                continue;
            }
            out.push([
                latitude(lat_idx, &meta),
                longitude(lon_idx, &meta),
                t_unix,
                *value,
            ]);
        }
    }
    if out.is_empty() {
        return None;
    }
    out.sort_by(|a, b| {
        a[2].total_cmp(&b[2])
            .then(a[0].total_cmp(&b[0]))
            .then(a[1].total_cmp(&b[1]))
    });
    Some(out)
}

fn write_bin(records: &[[f64; FIELDS]]) -> Vec<u8> {
    let mut out = Vec::with_capacity(8 + records.len() * FIELDS * 8);
    out.extend_from_slice(MAGIC);
    out.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for record in records {
        for v in record {
            out.extend_from_slice(&v.to_le_bytes());
        }
    }
    out
}

fn parse_bin(bytes: &[u8]) -> Option<Vec<[f64; FIELDS]>> {
    if bytes.get(0..4)? != MAGIC {
        return None;
    }
    let n = u32::from_le_bytes(bytes.get(4..8)?.try_into().ok()?) as usize;
    if bytes.len() != 8 + n * FIELDS * 8 {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let mut record = [0.0f64; FIELDS];
        for (j, slot) in record.iter_mut().enumerate() {
            let off = 8 + i * FIELDS * 8 + j * 8;
            *slot = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        }
        out.push(record);
    }
    Some(out)
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn emit_records(records: &[[f64; FIELDS]], out: &str, origin: &str) {
    if records.is_empty() {
        eprintln!("no records — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
    let bytes_out = write_bin(records);
    if std::fs::write(out, &bytes_out).is_err() {
        eprintln!("write {out} returned void");
        std::process::exit(1);
    }
    let t_min = match records.first() {
        Some(r) => r[2],
        None => f64::NAN,
    };
    let t_max = match records.last() {
        Some(r) => r[2],
        None => f64::NAN,
    };
    let v_min = records.iter().map(|r| r[3]).fold(f64::INFINITY, f64::min);
    let v_max = records
        .iter()
        .map(|r| r[3])
        .fold(f64::NEG_INFINITY, f64::max);
    eprintln!(
        "{out}: {} records, ROTI [{}, {}] TECU/min, t [{}, {}] unix",
        records.len(),
        v_min,
        v_max,
        t_min,
        t_max
    );
    eprintln!(
        "  records {} bytes, sha256 {}",
        bytes_out.len(),
        sha256_hex(&bytes_out)
    );
    match parse_bin(&bytes_out) {
        Some(parsed) if parsed == records => {
            eprintln!("  roundtrip: {} records parse, identical", parsed.len());
        }
        Some(parsed) => {
            eprintln!(
                "  roundtrip: {} records parse but differ from the emitted set",
                parsed.len()
            );
            std::process::exit(1);
        }
        None => {
            eprintln!("  roundtrip parse void — the bin stays unverified");
            std::process::exit(1);
        }
    }
    let out_name = match std::path::Path::new(out).file_name() {
        Some(n) => n.to_string_lossy().into_owned(),
        None => out.to_string(),
    };
    println!("url https://github.com/omegaflow/sources/releases/download/{NETLOC}/{out_name}");
    println!("origin {origin}");
    println!("compiler {COMPILER}");
    println!("sha256 {}", sha256_hex(&bytes_out));
    println!("format impc_roti");
    println!("ttl 3600");
    println!("at earth");
    println!(
        "quantity impc_roti_tecu_min impc_roti_tecu_min inverse-square scale TECU/min 60 0.0 0.0"
    );
}

fn compile_url(url: &str, out: &str) {
    let Some(bytes) = fetch_raw_bytes(url) else {
        eprintln!("{url}: the file stays unfetched");
        std::process::exit(1);
    };
    let text = String::from_utf8_lossy(&bytes);
    let Some(root) = parse_json(&text) else {
        eprintln!("{url}: the json stays unparsed");
        std::process::exit(1);
    };
    let Some(records) = parse_records(&root) else {
        eprintln!("{url}: metadata or grid absent — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    };
    emit_records(&records, out, url);
}

fn compile_file(path: &str, out: &str) {
    let Ok(bytes) = std::fs::read(path) else {
        eprintln!("{path}: the file stays unread");
        std::process::exit(1);
    };
    let text = String::from_utf8_lossy(&bytes);
    let Some(root) = parse_json(&text) else {
        eprintln!("{path}: the json stays unparsed");
        std::process::exit(1);
    };
    let Some(records) = parse_records(&root) else {
        eprintln!("{path}: metadata or grid absent — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    };
    emit_records(&records, out, path);
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = match arg_value(&args, "--out") {
        Some(p) => p,
        None => "impc_roti.bin".to_string(),
    };
    if let Some(path) = arg_value(&args, "--file") {
        compile_file(&path, &out);
    } else if let Some(url) = arg_value(&args, "--url") {
        compile_url(&url, &out);
    } else {
        compile_url(SOURCE_URL, &out);
    }
    if ci_mode && !omegaflow::cdn::upload_release(NETLOC, &out) {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bin_roundtrip_preserves_records() {
        let records = vec![
            [-89.0, -179.0, 1_354_416_000.0, 1.5],
            [89.0, 179.0, 1_354_416_000.0, 0.0],
        ];
        let bytes = write_bin(&records);
        assert_eq!(bytes.len(), 8 + records.len() * FIELDS * 8);
        assert_eq!(parse_bin(&bytes).as_deref(), Some(records.as_slice()));
    }

    #[test]
    fn parse_bin_rejects_foreign_magic_and_short_body() {
        assert!(parse_bin(b"CRTI").is_none());
        assert!(parse_bin(b"XXXX").is_none());
        let good = write_bin(&[[0.0; FIELDS]]);
        let short = good[..good.len() - 1].to_vec();
        assert!(parse_bin(&short).is_none());
    }

    #[test]
    fn bin_centres_follow_the_coverage_corners() {
        let meta = GridMeta {
            min_lat: -90.0,
            max_lat: 90.0,
            delta_lat: 2.0,
            min_lon: -180.0,
            max_lon: 180.0,
            delta_lon: 2.0,
        };
        assert_eq!(latitude(0, &meta), -89.0);
        assert_eq!(latitude(89, &meta), 89.0);
        assert_eq!(longitude(0, &meta), -179.0);
        assert_eq!(longitude(179, &meta), 179.0);
        assert_eq!(
            bin_count(meta.min_lat, meta.max_lat, meta.delta_lat),
            Some(90)
        );
        assert_eq!(
            bin_count(meta.min_lon, meta.max_lon, meta.delta_lon),
            Some(180)
        );
    }
}
