use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::upload_release;
use omegaflow::force::force_id_of;
use omegaflow::hdf5::{Hdf5File, Hdf5Layout};

const MAGIC: [u8; 4] = *b"LOSC";
const REC_BYTES: usize = 26 * 8;
const HDF5_MAGIC: [u8; 4] = [0x89, b'H', b'D', b'F'];
const NETLOC: &str = "gwosc.org";
const COMPILER: &str = "tools/harvest/src/bin/losc_compiler.rs";
const ORIGIN: &str = "https://gwosc.org/eventapi/json/GWTC-1-confident/GW150914/v3/H-H1_GWOSC_4KHZ_R1-1126259447-32.hdf5";
const DATASET: &str = "strain/Strain";
const ATTR_PATHS: [&str; 3] = ["strain/Strain", "strain", ""];
const SLOT_VAL: usize = 3;
const SLOT_EPOCH: usize = 4;
const SLOT_TTL: usize = 5;
const SLOT_FORCE_TYPE: usize = 9;
const SLOT_PRESENCE: usize = 25;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn find_attr(file: &Hdf5File, name: &str) -> Option<f64> {
    ATTR_PATHS.iter().find_map(|p| file.attr_f64(p, name))
}

fn scalar(file: &Hdf5File, name: &str) -> Option<f64> {
    file.read_f64_dataset(name)
        .ok()
        .and_then(|v| v.first().copied())
}

fn gps_start(file: &Hdf5File) -> Option<f64> {
    find_attr(file, "Xstart")
        .or_else(|| find_attr(file, "GPSstart"))
        .or_else(|| find_attr(file, "GPS_start"))
        .or_else(|| scalar(file, "meta/GPSstart"))
}

fn sample_rate(file: &Hdf5File, npoints: f64) -> Option<f64> {
    if let Some(dt) = find_attr(file, "Xspacing").filter(|v| v.is_finite() && *v > 0.0) {
        return Some(1.0 / dt);
    }
    if let Some(dt) = find_attr(file, "DeltaT").filter(|v| v.is_finite() && *v > 0.0) {
        return Some(1.0 / dt);
    }
    if let Some(rate) = find_attr(file, "fs")
        .or_else(|| find_attr(file, "SampleRate"))
        .or_else(|| find_attr(file, "sample_rate"))
        .filter(|v| v.is_finite() && *v > 0.0)
    {
        return Some(rate);
    }
    let duration = find_attr(file, "Duration").or_else(|| scalar(file, "meta/Duration"));
    match duration {
        Some(d) if d.is_finite() && d > 0.0 && npoints.is_finite() && npoints > 0.0 => {
            Some(npoints / d)
        }
        _ => None,
    }
}

fn datatype_name(class: u8, size: usize) -> &'static str {
    match (class, size) {
        (1, 4) => "f32",
        (1, 8) => "f64",
        (0, 4) => "i32",
        (0, 8) => "i64",
        _ => "other",
    }
}

fn write_bin(records: &[[f64; 26]]) -> Vec<u8> {
    let mut out = Vec::with_capacity(8 + records.len() * REC_BYTES);
    out.extend_from_slice(&MAGIC);
    out.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for r in records {
        for v in r {
            out.extend_from_slice(&v.to_le_bytes());
        }
    }
    out
}

fn parse_bin(bytes: &[u8]) -> Option<Vec<[f64; 26]>> {
    if bytes.len() < 8 || bytes[0..4] != MAGIC {
        return None;
    }
    let n = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    if bytes.len() != 8 + n * REC_BYTES {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let base = 8 + i * REC_BYTES;
        let mut r = [0.0f64; 26];
        for (k, slot) in r.iter_mut().enumerate() {
            let o = base + k * 8;
            *slot = f64::from_le_bytes(bytes[o..o + 8].try_into().ok()?);
        }
        out.push(r);
    }
    Some(out)
}

fn emit_records(records: &mut Vec<[f64; 26]>, out: &str) {
    if records.is_empty() {
        eprintln!("no records — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
    records.sort_by(|a, b| a[SLOT_EPOCH].total_cmp(&b[SLOT_EPOCH]));
    let bytes_out = write_bin(records);
    if std::fs::write(out, &bytes_out).is_err() {
        eprintln!("write {out} returned void");
        std::process::exit(1);
    }
    let t_min = records
        .iter()
        .map(|r| r[SLOT_EPOCH])
        .fold(f64::INFINITY, f64::min);
    let t_max = records
        .iter()
        .map(|r| r[SLOT_EPOCH])
        .fold(f64::NEG_INFINITY, f64::max);
    let v_min = records
        .iter()
        .map(|r| r[SLOT_VAL])
        .fold(f64::INFINITY, f64::min);
    let v_max = records
        .iter()
        .map(|r| r[SLOT_VAL])
        .fold(f64::NEG_INFINITY, f64::max);
    eprintln!(
        "{out}: {} records, epoch [{t_min}, {t_max}], strain [{v_min}, {v_max}], {} B",
        records.len(),
        bytes_out.len()
    );
    match parse_bin(&bytes_out) {
        Some(parsed) if parsed == *records => {
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
    println!("origin {ORIGIN}");
    println!("compiler {COMPILER}");
    println!("sha256 {}", sha256_hex(&bytes_out));
    println!("format losc-strain");
}

fn compile(input: &str, out: &str, force_type: u8, ttl_s: f64) {
    let bytes = match std::fs::read(input) {
        Ok(b) => b,
        Err(_) => {
            eprintln!("{input}: the file stays unread");
            std::process::exit(1);
        }
    };
    if bytes.get(0..4) != Some(&HDF5_MAGIC[..]) {
        eprintln!("{input}: carries no HDF5 signature — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
    let file = match Hdf5File::parse(&bytes) {
        Ok(f) => f,
        Err(note) => {
            eprintln!("{input}: HDF5 arm parses void: {note:?}");
            std::process::exit(1);
        }
    };
    let (obj, ds, dt) = match file.dataset(DATASET) {
        Ok(t) => t,
        Err(note) => {
            eprintln!("{input}: {DATASET} does not resolve: {note:?}");
            std::process::exit(1);
        }
    };
    let layout = match obj.layout.as_ref() {
        Some(Hdf5Layout::Contiguous { .. }) => "contiguous",
        Some(Hdf5Layout::Chunked { .. }) => "chunked",
        Some(Hdf5Layout::Compact { .. }) => "compact",
        None => "absent",
    };
    let filters: Vec<String> = obj.filters.iter().map(|f| f.id.to_string()).collect();
    let npoints = ds.dims.iter().fold(1u64, |a, d| a * *d) as f64;
    let kind = datatype_name(dt.class, dt.size);
    let gps = match gps_start(&file) {
        Some(v) if v.is_finite() => v,
        _ => {
            eprintln!("{input}: the GPS start is absent — the bin stays unwritten");
            std::process::exit(1);
        }
    };
    let rate = match sample_rate(&file, npoints) {
        Some(v) if v.is_finite() && v > 0.0 => v,
        _ => {
            eprintln!("{input}: the sample rate is absent — the bin stays unwritten");
            std::process::exit(1);
        }
    };
    let values = match file.read_f64_dataset(DATASET) {
        Ok(v) => v,
        Err(note) => {
            eprintln!("{input}: {DATASET} stays unread: {note:?}");
            std::process::exit(1);
        }
    };
    let mut records: Vec<[f64; 26]> = Vec::with_capacity(values.len());
    let mut absent = 0usize;
    for (i, value) in values.iter().enumerate() {
        if !value.is_finite() {
            absent += 1;
            continue;
        }
        let mut r = [0.0f64; 26];
        r[SLOT_EPOCH] = gps + i as f64 / rate;
        r[SLOT_VAL] = *value;
        r[SLOT_TTL] = ttl_s;
        r[SLOT_FORCE_TYPE] = force_type as f64;
        r[SLOT_PRESENCE] = 1.0;
        records.push(r);
    }
    eprintln!(
        "{input}: {DATASET} {kind} {layout} filters[{}], {} samples, GPS start {gps}, fs {rate}, {absent} absent",
        filters.join(","),
        values.len()
    );
    emit_records(&mut records, out);
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = match arg_value(&args, "--out") {
        Some(p) => p,
        None => {
            eprintln!("usage: losc_compiler --input <hdf5> --out <file.bin> [--ci-mode]");
            std::process::exit(2);
        }
    };
    let input = match arg_value(&args, "--input") {
        Some(p) => p,
        None => {
            eprintln!("usage: losc_compiler --input <hdf5> --out <file.bin> --force <medium> --ttl <secs> [--ci-mode]");
            std::process::exit(2);
        }
    };
    let force_type = match arg_value(&args, "--force").and_then(|n| force_id_of(&n)) {
        Some(id) => id,
        None => {
            eprintln!("losc: the force admission is declared per source — pass --force <medium>; the record stays unwritten");
            std::process::exit(2);
        }
    };
    let ttl_s = match arg_value(&args, "--ttl").and_then(|v| v.parse::<f64>().ok()) {
        Some(t) if t.is_finite() && t > 0.0 => t,
        _ => {
            eprintln!("losc: the ttl is declared per source — pass --ttl <secs>; a ttl <= 0 drops every record");
            std::process::exit(2);
        }
    };
    compile(&input, &out, force_type, ttl_s);
    if ci_mode && !upload_release(NETLOC, &out) {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bin_roundtrip_preserves_records() {
        let mut a = [0.0f64; 26];
        a[SLOT_VAL] = 1.5e-19;
        a[SLOT_EPOCH] = 1_126_259_447.0;
        a[SLOT_PRESENCE] = 1.0;
        let bytes = write_bin(&[a]);
        assert_eq!(bytes.len(), 8 + REC_BYTES);
        assert_eq!(parse_bin(&bytes).as_deref(), Some([a].as_slice()));
    }

    #[test]
    fn parse_bin_rejects_foreign_magic_and_short_body() {
        assert!(parse_bin(b"XXXX\x00\x00\x00\x00").is_none());
        assert!(parse_bin(b"LOSC").is_none());
        let good = write_bin(&[[0.0f64; 26]]);
        assert!(parse_bin(&good[..good.len() - 1]).is_none());
    }
}
