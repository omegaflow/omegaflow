use omegaflow::archivar::{embedded_lsk, fetch_raw_bytes};
use omegaflow::cdn::upload_release;
use omegaflow::hdf5::Hdf5File;

const NETLOC: &str = "cdaweb.gsfc.nasa.gov";
const MAGIC: &[u8; 4] = b"G16M";
const VERSION: u8 = 1;
const HEADER_BYTES: usize = 12;
const REC_BYTES: usize = 20;
const J2K_UNIX: f64 = 946728000.0;
const FILL: f64 = -9999.0;
const FRAMES: [&str; 3] = ["gsm", "gse", "eci"];

fn arg_value(args: &[String], key: &str) -> Option<String> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn dataset_name(frame: &str) -> Option<String> {
    if FRAMES.contains(&frame) {
        Some(format!("b_{frame}"))
    } else {
        None
    }
}

fn component(b: &[f64], row: usize, comp: usize, fill: f64) -> Option<f64> {
    let v = *b.get(row * 3 + comp)?;
    if v.is_finite() && v != fill {
        Some(v)
    } else {
        None
    }
}

fn row(t: f64, t_fill: f64, b: &[f64], i: usize, b_fill: f64) -> Option<[f64; 4]> {
    if !t.is_finite() || t == t_fill {
        return None;
    }
    let bx = component(b, i, 0, b_fill)?;
    let by = component(b, i, 1, b_fill)?;
    let bz = component(b, i, 2, b_fill)?;
    Some([t, bx, by, bz])
}

fn pack(records: &[[f64; 4]]) -> Vec<u8> {
    let mut out = Vec::with_capacity(HEADER_BYTES + records.len() * REC_BYTES);
    out.extend_from_slice(MAGIC);
    out.push(VERSION);
    out.extend_from_slice(&[0u8; 3]);
    out.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for r in records {
        out.extend_from_slice(&r[0].to_le_bytes());
        out.extend_from_slice(&(r[1] as f32).to_le_bytes());
        out.extend_from_slice(&(r[2] as f32).to_le_bytes());
        out.extend_from_slice(&(r[3] as f32).to_le_bytes());
    }
    out
}

fn unpack(bytes: &[u8]) -> Option<Vec<[f64; 4]>> {
    if bytes.len() < HEADER_BYTES || &bytes[0..4] != MAGIC || bytes[4] != VERSION {
        return None;
    }
    let count = u32::from_le_bytes(bytes[8..12].try_into().ok()?) as usize;
    if bytes.len() != HEADER_BYTES + count * REC_BYTES {
        return None;
    }
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        let base = HEADER_BYTES + i * REC_BYTES;
        let t = f64::from_le_bytes(bytes[base..base + 8].try_into().ok()?);
        let bx = f32::from_le_bytes(bytes[base + 8..base + 12].try_into().ok()?);
        let by = f32::from_le_bytes(bytes[base + 12..base + 16].try_into().ok()?);
        let bz = f32::from_le_bytes(bytes[base + 16..base + 20].try_into().ok()?);
        out.push([t, bx as f64, by as f64, bz as f64]);
    }
    Some(out)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let frame = match arg_value(&args, "--frame") {
        Some(v) => v,
        None => "gsm".to_string(),
    };
    let Some(dataset) = dataset_name(&frame) else {
        eprintln!("--frame gsm|gse|eci (gsm is the geocentric solar magnetospheric frame)");
        std::process::exit(2);
    };
    let out = match arg_value(&args, "--out") {
        Some(v) => v,
        None => format!("data/{NETLOC}/goes16_mag.bin"),
    };
    let (bytes, source) = match arg_value(&args, "--input") {
        Some(path) => match std::fs::read(&path) {
            Ok(b) => (b, path),
            Err(e) => {
                eprintln!("read {path} returned void: {e}");
                std::process::exit(1);
            }
        },
        None => match arg_value(&args, "--url") {
            Some(url) => match fetch_raw_bytes(&url) {
                Some(b) => (b, url),
                None => {
                    eprintln!("{url}: fetch void — the series stays unwritten (0 honored)");
                    std::process::exit(1);
                }
            },
            None => {
                eprintln!("--input <file.nc> or --url <url> required");
                std::process::exit(2);
            }
        },
    };
    let file = match Hdf5File::parse(&bytes) {
        Ok(f) => f,
        Err(note) => {
            eprintln!("{source}: HDF5/NetCDF4 arm parses void: {note:?}");
            std::process::exit(1);
        }
    };
    let time = match file.read_f64_dataset("time") {
        Ok(v) => v,
        Err(note) => {
            eprintln!("{source}: time dataset unread: {note:?}");
            std::process::exit(1);
        }
    };
    let field = match file.read_f64_dataset(&dataset) {
        Ok(v) => v,
        Err(note) => {
            eprintln!("{source}: {dataset} dataset unread: {note:?}");
            std::process::exit(1);
        }
    };
    let t_fill = file.attr_f64("time", "_FillValue").unwrap_or(FILL);
    let b_fill = file.attr_f64(&dataset, "_FillValue").unwrap_or(FILL);
    let Some(lsk) = embedded_lsk() else {
        eprintln!("embedded naif0012 parses void — the epoch conversion stays unread");
        std::process::exit(1);
    };
    let mut records: Vec<[f64; 4]> = Vec::with_capacity(time.len());
    let mut absent = 0usize;
    for i in 0..time.len() {
        let Some(r) = row(time[i], t_fill, &field, i, b_fill) else {
            absent += 1;
            continue;
        };
        let Some(tdb) = lsk.unix_to_tdb(r[0] + J2K_UNIX) else {
            absent += 1;
            continue;
        };
        records.push([tdb, r[1], r[2], r[3]]);
    }
    eprintln!(
        "{source}: {} rows, {} kept in {dataset}, {} absent (fill/unreadable)",
        time.len(),
        records.len(),
        absent
    );
    if records.is_empty() {
        eprintln!("{out}: no valid {dataset} vector — the series stays unwritten (0 honored)");
        std::process::exit(1);
    }
    records.sort_by(|a, b| a[0].total_cmp(&b[0]));
    let bin = pack(&records);
    match unpack(&bin) {
        Some(parsed) if parsed.len() == records.len() => {}
        _ => {
            eprintln!("{out}: roundtrip parse void — the series stays unverified");
            std::process::exit(1);
        }
    }
    if let Some(parent) = std::path::Path::new(&out).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if std::fs::write(&out, &bin).is_err() {
        eprintln!("write {out} returned void");
        std::process::exit(1);
    }
    let first = records[0][0];
    let last = records[records.len() - 1][0];
    eprintln!(
        "{out}: {} {dataset} vectors packed (tdb {first:.1}..{last:.1}), {} B, roundtrip holds",
        records.len(),
        bin.len()
    );
    if ci_mode && !upload_release(NETLOC, &out) {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dataset_name_reads_the_measured_frames() {
        assert_eq!(dataset_name("gsm").as_deref(), Some("b_gsm"));
        assert_eq!(dataset_name("gse").as_deref(), Some("b_gse"));
        assert_eq!(dataset_name("eci").as_deref(), Some("b_eci"));
        assert_eq!(dataset_name("sensor"), None);
    }

    #[test]
    fn row_is_none_where_the_source_carries_no_vector() {
        let b = vec![1.0, 2.0, 3.0, FILL, 5.0, 6.0];
        assert_eq!(row(100.0, FILL, &b, 0, FILL), Some([100.0, 1.0, 2.0, 3.0]));
        assert!(row(100.0, FILL, &b, 1, FILL).is_none());
        assert!(row(FILL, FILL, &b, 0, FILL).is_none());
        assert!(row(f64::NAN, FILL, &b, 0, FILL).is_none());
        assert!(row(100.0, FILL, &b, 2, FILL).is_none());
    }

    #[test]
    fn pack_and_unpack_roundtrip() {
        let records = vec![[100.0, -1.5, 2.25, -3.0], [200.0, 4.0, 5.0, 6.0]];
        let bytes = pack(&records);
        assert_eq!(bytes.len(), HEADER_BYTES + 2 * REC_BYTES);
        let parsed = unpack(&bytes).expect("roundtrip parses");
        assert_eq!(parsed.len(), 2);
        for (a, b) in records.iter().zip(parsed.iter()) {
            assert!((a[0] - b[0]).abs() < 1e-9);
            for k in 1..4 {
                assert!((a[k] - b[k]).abs() < 1e-6);
            }
        }
    }

    #[test]
    fn unpack_refuses_a_foreign_bin() {
        assert!(unpack(b"G16M\x01\x00\x00\x00\x01\x00\x00\x00").is_none());
        assert!(unpack(b"XXXX\x01\x00\x00\x00\x00\x00\x00\x00").is_none());
        assert!(unpack(b"G16M\x02\x00\x00\x00\x00\x00\x00\x00").is_none());
    }
}
