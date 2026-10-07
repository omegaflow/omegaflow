use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdf::{CdfFile, value_present};
use omegaflow::cdn::upload_release;
use std::collections::HashMap;

const MAGIC: &[u8; 4] = b"POES";
const FIELDS: usize = 3;
const NETLOC: &str = "cdaweb.gsfc.nasa.gov";
const OUT_PATH: &str = "poes19_meped.bin";
const COMPILER: &str = "tools/harvest/src/bin/poes19_meped_compiler.rs";
const TTL: f64 = 86400.0;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn channel_map(file: &CdfFile, bytes: &[u8], name: &str, idx: usize) -> Option<HashMap<u32, f64>> {
    let var = file.var(name)?;
    let records = file.var_records(bytes, var).ok()?;
    let mut map = HashMap::with_capacity(records.len());
    for (rec, vals) in records {
        if let Some(v) = vals.get(idx).copied() {
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
    let label = source.unwrap_or(out);
    let file = match CdfFile::parse(bytes) {
        Ok(f) => f,
        Err(note) => {
            eprintln!("{label}: {:?}", note);
            std::process::exit(1);
        }
    };
    let Some(epoch_var) = file.var("Epoch") else {
        eprintln!("Epoch absent — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    };
    let epoch = match file.epoch_map(bytes, epoch_var) {
        Ok(m) => m,
        Err(note) => {
            eprintln!("Epoch: {:?}", note);
            std::process::exit(1);
        }
    };
    let Some(ele) = channel_map(&file, bytes, "mep_ele_flux", 0) else {
        eprintln!("mep_ele_flux absent — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    };
    let Some(pro) = channel_map(&file, bytes, "mep_pro_flux_p6", 0) else {
        eprintln!("mep_pro_flux_p6 absent — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    };

    let mut records: Vec<[f64; FIELDS]> = Vec::with_capacity(epoch.len());
    let mut skipped = 0usize;
    for (rec, t) in &epoch {
        let (Some(&e), Some(&p)) = (ele.get(rec), pro.get(rec)) else {
            skipped += 1;
            continue;
        };
        if !value_present(*t) || !value_present(e) || !value_present(p) {
            skipped += 1;
            continue;
        }
        records.push([*t, e, p]);
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
    eprintln!(
        "{out}: {} records ({} skipped), mep_ele_flux [{}, {}] pfu, mep_pro_flux_p6 [{}, {}] pfu",
        records.len(),
        skipped,
        records.iter().map(|r| r[1]).fold(f64::INFINITY, f64::min),
        records
            .iter()
            .map(|r| r[1])
            .fold(f64::NEG_INFINITY, f64::max),
        records.iter().map(|r| r[2]).fold(f64::INFINITY, f64::min),
        records
            .iter()
            .map(|r| r[2])
            .fold(f64::NEG_INFINITY, f64::max),
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
    println!("format poes19_meped");
    println!("ttl {}", TTL as u64);
    println!("at earth");
    println!(
        "field poes19_meped_ele_flux poes19_meped_ele_flux inverse-square em pfu {TTL} 0.0 0.0"
    );
    println!(
        "field poes19_meped_pro_p6_flux poes19_meped_pro_p6_flux inverse-square em pfu {TTL} 0.0 0.0"
    );
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
        let Ok(bytes) = std::fs::read(&path) else {
            eprintln!("{path}: the file stays unread");
            std::process::exit(1);
        };
        match CdfFile::parse(&bytes) {
            Ok(file) => {
                for name in ["Epoch", "mep_ele_flux", "mep_pro_flux_p6"] {
                    match file.var(name) {
                        Some(v) => eprintln!("{name}: dims {:?}", v.dim_sizes),
                        None => eprintln!("{name}: absent"),
                    }
                }
            }
            Err(note) => {
                eprintln!("{path}: {:?}", note);
                std::process::exit(1);
            }
        }
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
            "usage: poes19_meped_compiler --probe <cdf> | --file <cdf> | --url <cdf> [--out <bin>] [--ci-mode]"
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
        let records = vec![[1_700_000_000.0, 12.5, 3.25], [1_700_000_002.0, 0.0, 0.0]];
        let bytes = write_bin(&records);
        assert_eq!(bytes.len(), 8 + records.len() * FIELDS * 8);
        assert_eq!(parse_bin(&bytes).as_deref(), Some(records.as_slice()));
    }

    #[test]
    fn parse_bin_rejects_foreign_magic_and_short_body() {
        assert!(parse_bin(b"POES").is_none());
        assert!(parse_bin(b"XXXX").is_none());
        let good = write_bin(&[[0.0; FIELDS]]);
        let short = good[..good.len() - 1].to_vec();
        assert!(parse_bin(&short).is_none());
    }

    #[test]
    fn absent_value_is_not_a_record() {
        assert!(!value_present(f64::NAN));
        assert!(!value_present(f64::INFINITY));
        assert!(value_present(0.0));
    }
}
