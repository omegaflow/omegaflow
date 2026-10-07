use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdf::{CdfFile, CdfVar, value_present};
use omegaflow::cdn::upload_release;
use std::collections::HashMap;

const MAGIC: &[u8; 4] = b"THGM";
const FIELDS: usize = 4;
const NETLOC: &str = "themis.ssl.berkeley.edu";
const OUT_PATH: &str = "themis_mag.bin";
const COMPILER: &str = "tools/harvest/src/bin/themis_mag_compiler.rs";
const TTL: f64 = 86400.0;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn type_name(t: u32) -> &'static str {
    match t {
        1 => "CDF_INT1",
        2 => "CDF_INT2",
        4 => "CDF_INT4",
        8 => "CDF_INT8",
        11 => "CDF_UINT1",
        12 => "CDF_UINT2",
        14 => "CDF_UINT4",
        21 => "CDF_REAL4",
        22 => "CDF_REAL8",
        31 => "CDF_EPOCH",
        32 => "CDF_EPOCH16",
        33 => "CDF_TIME_TT2000",
        41 => "CDF_BYTE",
        44 => "CDF_FLOAT",
        45 => "CDF_DOUBLE",
        51 => "CDF_CHAR",
        52 => "CDF_UCHAR",
        _ => "unknown",
    }
}

fn probe(path: &str) {
    let Ok(bytes) = std::fs::read(path) else {
        eprintln!("{path}: the file stays unread");
        std::process::exit(1);
    };
    let file = match CdfFile::parse(&bytes) {
        Ok(f) => f,
        Err(note) => {
            eprintln!("{path}: {:?}", note);
            std::process::exit(1);
        }
    };
    eprintln!(
        "{}: CDF {}.{}.{} encoding {} majority {} {} zVariables",
        path,
        file.version.0,
        file.version.1,
        file.version.2,
        file.encoding,
        file.majority,
        file.vars.len(),
    );
    for var in &file.vars {
        let n_rec = file.var_records(&bytes, var).map(|r| r.len()).ok();
        eprintln!(
            "  var {} {} {} x{} dims {:?} record_vary {} max_rec {} records {:?}",
            var.var_num,
            var.name,
            type_name(var.data_type),
            var.num_elements,
            var.dim_sizes,
            var.record_vary,
            var.max_rec,
            n_rec,
        );
        if let Ok(records) = file.var_records(&bytes, var)
            && let Some((rec, vals)) = records.first()
        {
            let real: Vec<f64> = vals
                .iter()
                .copied()
                .filter(|v| value_present(*v))
                .take(4)
                .collect();
            eprintln!("    rec {rec} n {} real-first {:?}", vals.len(), real);
        }
    }
}

fn mag_var(file: &CdfFile) -> Option<&CdfVar> {
    use omegaflow::cdf::{TYPE_DOUBLE, TYPE_FLOAT, TYPE_REAL4, TYPE_REAL8};
    file.vars.iter().find(|v| {
        v.name.starts_with("thg_mag_")
            && v.dim_sizes == [3]
            && matches!(
                v.data_type,
                TYPE_REAL4 | TYPE_REAL8 | TYPE_FLOAT | TYPE_DOUBLE
            )
    })
}

fn vector_map(file: &CdfFile, bytes: &[u8], name: &str) -> Option<HashMap<u32, Vec<f64>>> {
    let var = file.var(name)?;
    let records = file.var_records(bytes, var).ok()?;
    let mut map = HashMap::with_capacity(records.len());
    for (rec, vals) in records {
        map.insert(rec, vals);
    }
    Some(map)
}

fn scalar_map(file: &CdfFile, bytes: &[u8], name: &str) -> Option<HashMap<u32, f64>> {
    let var = file.var(name)?;
    let records = file.var_records(bytes, var).ok()?;
    let mut map = HashMap::with_capacity(records.len());
    for (rec, vals) in records {
        if let Some(v) = vals.first().copied() {
            map.insert(rec, v);
        }
    }
    Some(map)
}

fn write_bin(records: &[[f64; FIELDS]]) -> Vec<u8> {
    let mut out = Vec::with_capacity(8 + records.len() * FIELDS * 8);
    out.extend_from_slice(MAGIC);
    out.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for r in records {
        for v in r {
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
        let mut r = [0.0f64; FIELDS];
        for (j, slot) in r.iter_mut().enumerate() {
            let off = 8 + i * FIELDS * 8 + j * 8;
            *slot = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        }
        out.push(r);
    }
    Some(out)
}

fn emit(bytes: &[u8], out: &str, source: Option<&str>) {
    let label = match source {
        Some(s) => s,
        None => out,
    };
    let file = match CdfFile::parse(bytes) {
        Ok(f) => f,
        Err(note) => {
            eprintln!("{label}: {:?}", note);
            std::process::exit(1);
        }
    };
    let Some(mag) = mag_var(&file) else {
        eprintln!("thg_mag_<station> [3] absent — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    };
    let mag_name = mag.name.clone();
    let time_name = format!("{mag_name}_time");
    let Some(epoch) = scalar_map(&file, bytes, &time_name) else {
        eprintln!("{time_name} absent — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    };
    let Some(bvec) = vector_map(&file, bytes, &mag_name) else {
        eprintln!("{mag_name} absent — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    };

    let mut records: Vec<[f64; FIELDS]> = Vec::with_capacity(epoch.len());
    let mut skipped = 0usize;
    for (rec, t) in &epoch {
        let Some(v) = bvec.get(rec) else {
            skipped += 1;
            continue;
        };
        if v.len() < 3 {
            skipped += 1;
            continue;
        }
        let (bx, by, bz) = (v[0], v[1], v[2]);
        if !value_present(*t) || !value_present(bx) || !value_present(by) || !value_present(bz) {
            skipped += 1;
            continue;
        }
        records.push([*t, bx, by, bz]);
    }
    records.sort_by(|a, b| a[0].total_cmp(&b[0]));

    if records.is_empty() {
        eprintln!("no records — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
    let bytes_out = write_bin(&records);
    if std::fs::write(out, &bytes_out).is_err() {
        eprintln!("write {out} returned void");
        std::process::exit(1);
    }
    let span = |i: usize| {
        (
            records.iter().map(|r| r[i]).fold(f64::INFINITY, f64::min),
            records
                .iter()
                .map(|r| r[i])
                .fold(f64::NEG_INFINITY, f64::max),
        )
    };
    let (x0, x1) = span(1);
    let (y0, y1) = span(2);
    let (z0, z1) = span(3);
    eprintln!(
        "{out}: {} records ({} skipped) from {mag_name}, Bx [{x0}, {x1}] By [{y0}, {y1}] Bz [{z0}, {z1}] nT",
        records.len(),
        skipped,
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
    if let Some(src) = source {
        println!("origin {src}");
    }
    println!("compiler {COMPILER}");
    println!("sha256 {}", sha256_hex(&bytes_out));
    println!("format themis_mag");
    println!("ttl {}", TTL as u64);
    println!("at earth");
    println!("field themis_mag_x_nt themis_mag_x_nt inverse-square em nT 86400 0.0 0.0");
    println!("field themis_mag_y_nt themis_mag_y_nt inverse-square em nT 86400 0.0 0.0");
    println!("field themis_mag_z_nt themis_mag_z_nt inverse-square em nT 86400 0.0 0.0");
}

fn compile_file(path: &str, out: &str) {
    let Ok(bytes) = std::fs::read(path) else {
        eprintln!("{path}: the file stays unread");
        std::process::exit(1);
    };
    emit(&bytes, out, None);
}

fn compile_url(url: &str, out: &str) {
    let Some(bytes) = fetch_raw_bytes(url) else {
        eprintln!("{url}: fetch void — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    };
    emit(&bytes, out, Some(url));
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Some(path) = arg_value(&args, "--probe") {
        probe(&path);
        return;
    }
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = match arg_value(&args, "--out") {
        Some(v) => v,
        None => OUT_PATH.to_string(),
    };
    if let Some(url) = arg_value(&args, "--url") {
        compile_url(&url, &out);
    } else if let Some(path) = arg_value(&args, "--file").or_else(|| arg_value(&args, "--input")) {
        compile_file(&path, &out);
    } else {
        eprintln!(
            "usage: themis_mag_compiler --probe <cdf> | --file <cdf> | --url <cdf> [--out <bin>] [--ci-mode]"
        );
        std::process::exit(2);
    }
    if ci_mode && !upload_release(NETLOC, &out) {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bin_roundtrip_preserves_records() {
        let records = vec![
            [1_767_225_600.0, 11329.0, 932.0, 52463.0],
            [1_767_225_660.0, -120.0, 0.0, -5.5],
        ];
        let bytes = write_bin(&records);
        assert_eq!(bytes.len(), 8 + records.len() * FIELDS * 8);
        assert_eq!(parse_bin(&bytes).as_deref(), Some(records.as_slice()));
    }

    #[test]
    fn parse_bin_rejects_foreign_magic_and_short_body() {
        assert!(parse_bin(b"XXXX").is_none());
        assert!(parse_bin(b"THGM").is_none());
        let good = write_bin(&[[0.0; FIELDS]]);
        let short = good[..good.len() - 1].to_vec();
        assert!(parse_bin(&short).is_none());
    }

    #[test]
    fn absent_value_is_not_a_record() {
        assert!(!value_present(f64::NAN));
        assert!(!value_present(f64::INFINITY));
        assert!(!value_present(-1.0e31));
        assert!(value_present(0.0));
    }
}
