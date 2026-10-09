use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdf::{CdfFile, value_present};
use omegaflow::cdn::upload_release;

const MAGIC: &[u8; 4] = b"CTEC";
const FIELDS: usize = 4;
const NETLOC: &str = "cdaweb.gsfc.nasa.gov";
const OUT_PATH: &str = "cdaweb_tec.bin";
const COMPILER: &str = "tools/harvest/src/bin/cdaweb_tec_compiler.rs";
const TTL_S: f64 = 86400.0;

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
        "{}: CDF {}.{}.{} encoding {} majority {} {} zVariables {} attributes eof {}",
        path,
        file.version.0,
        file.version.1,
        file.version.2,
        file.encoding,
        file.majority,
        file.vars.len(),
        file.num_att,
        file.eof,
    );
    for var in &file.vars {
        let n_rec = file.var_records(&bytes, var).map(|r| r.len()).ok();
        eprintln!(
            "  var {} {} {} x{} dims {:?} vary {:?} record_vary {} records {:?}",
            var.var_num,
            var.name,
            type_name(var.data_type),
            var.num_elements,
            var.dim_sizes,
            var.dim_vary,
            var.record_vary,
            n_rec,
        );
    }
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

fn support_values(file: &CdfFile, bytes: &[u8], name: &str) -> Option<Vec<f64>> {
    let var = file.var(name)?;
    let mut records = file.var_records(bytes, var).ok()?;
    if records.len() != 1 {
        return None;
    }
    Some(records.swap_remove(0).1)
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
        eprintln!("{label}: Epoch absent — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    };
    let epoch = match file.epoch_map(bytes, epoch_var) {
        Ok(m) => m,
        Err(note) => {
            eprintln!("{label}: Epoch: {:?}", note);
            std::process::exit(1);
        }
    };
    let Some(lat_vals) = support_values(&file, bytes, "lat") else {
        eprintln!("{label}: lat absent — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    };
    let Some(lon_vals) = support_values(&file, bytes, "lon") else {
        eprintln!("{label}: lon absent — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    };
    let Some(tec_var) = file.var("tecUQR") else {
        eprintln!("{label}: tecUQR absent — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    };
    let tec_records = match file.var_records(bytes, tec_var) {
        Ok(r) => r,
        Err(note) => {
            eprintln!("{label}: tecUQR: {:?}", note);
            std::process::exit(1);
        }
    };

    let n_lon = lon_vals.len();
    if n_lon == 0 || lat_vals.is_empty() {
        eprintln!("{label}: empty lat/lon grid — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
    let per = lat_vals.len() * n_lon;

    let mut records: Vec<[f64; FIELDS]> = Vec::new();
    let mut skipped_epochs = 0usize;
    for (rec, vals) in &tec_records {
        let Some(&t) = epoch.get(rec) else {
            skipped_epochs += 1;
            continue;
        };
        if !value_present(t) || vals.len() != per {
            skipped_epochs += 1;
            continue;
        }
        for (i, &la) in lat_vals.iter().enumerate() {
            if !value_present(la) {
                continue;
            }
            for (j, &lo) in lon_vals.iter().enumerate() {
                if !value_present(lo) {
                    continue;
                }
                let v = vals[i * n_lon + j];
                if !value_present(v) {
                    continue;
                }
                records.push([la, lo, v, t]);
            }
        }
    }

    if records.is_empty() {
        eprintln!("no records — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
    records.sort_by(|a, b| {
        a[3].total_cmp(&b[3])
            .then(a[0].total_cmp(&b[0]))
            .then(a[1].total_cmp(&b[1]))
            .then(a[2].total_cmp(&b[2]))
    });

    let bytes_out = write_bin(&records);
    if let Some(parent) = std::path::Path::new(out).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if std::fs::write(out, &bytes_out).is_err() {
        eprintln!("write {out} returned void");
        std::process::exit(1);
    }
    let v_min = records.iter().map(|r| r[2]).fold(f64::INFINITY, f64::min);
    let v_max = records
        .iter()
        .map(|r| r[2])
        .fold(f64::NEG_INFINITY, f64::max);
    let t_min = records[0][3];
    let t_max = records[records.len() - 1][3];
    eprintln!(
        "{out}: {} records ({} epoch(s) skipped), TEC [{}, {}] TECU, t [{}, {}] unix",
        records.len(),
        skipped_epochs,
        v_min,
        v_max,
        t_min,
        t_max
    );
    eprintln!(
        "  {} grid points ({} lat x {} lon) x {} epochs",
        per,
        lat_vals.len(),
        n_lon,
        tec_records.len() - skipped_epochs
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
    println!("format cdaweb_tec");
    println!("ttl {}", TTL_S as u64);
    println!("at earth");
    println!("cmap .");
    println!("lat lat");
    println!("lon lon");
    println!("field cdaweb_tec_tecu cdaweb_tec_tecu inverse-square em TECU 86400 0.0 0.0");
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
        eprintln!("{url}: the CDF stays unfetched");
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
    } else if let Some(path) = arg_value(&args, "--file") {
        compile_file(&path, &out);
    } else {
        eprintln!(
            "usage: cdaweb_tec_compiler --probe <cdf> | --file <cdf> | --url <cdf-url> [--out <bin>] [--ci-mode]"
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
            [87.5, -180.0, 3.9, 1_602_633_600.0],
            [85.0, -175.0, 12.5, 1_602_634_500.0],
        ];
        let bytes = write_bin(&records);
        assert_eq!(bytes.len(), 8 + records.len() * FIELDS * 8);
        assert_eq!(parse_bin(&bytes).as_deref(), Some(records.as_slice()));
    }

    #[test]
    fn parse_bin_rejects_foreign_magic_and_short_body() {
        assert!(parse_bin(b"XXXX").is_none());
        assert!(parse_bin(b"TECU").is_none());
        let good = write_bin(&[[0.0; FIELDS]]);
        let short = good[..good.len() - 1].to_vec();
        assert!(parse_bin(&short).is_none());
    }

    #[test]
    fn parses_cdaweb_tec_when_present() {
        let path = "/tmp/opencode/tec.cdf";
        let bytes = match std::fs::read(path) {
            Ok(b) => b,
            Err(_) => return,
        };
        let file = CdfFile::parse(&bytes).unwrap();
        let tec = file.var("tecUQR").unwrap();
        assert_eq!(tec.dim_sizes, vec![71, 73]);
        assert!(file.var("lat").is_some());
        assert!(file.var("lon").is_some());
        assert!(file.var("Epoch").is_some());
        let records = file.var_records(&bytes, tec).unwrap();
        assert_eq!(records.len(), 96);
        assert_eq!(records[0].1.len(), 71 * 73);
    }
}
