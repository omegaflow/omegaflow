use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdf::{CdfFile, is_epoch_type, value_present};

const MAGIC: &[u8; 4] = b"CRTI";
const FIELDS: usize = 4;
const NETLOC: &str = "cdaweb.gsfc.nasa.gov";
const BASE_URL: &str = "https://cdaweb.gsfc.nasa.gov/pub/data/gps/roti15min_jpl/";
const COMPILER: &str = "tools/harvest/src/bin/cdaweb_roti_compiler.rs";
const ROTI_VAR: &str = "rotimed";
const LAT_VAR: &str = "lat";
const LON_VAR: &str = "lon";

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
        eprintln!(
            "  var {} {} {} x{} dims {:?} vary {:?} record_vary {} max_rec {}",
            var.var_num,
            var.name,
            type_name(var.data_type),
            var.num_elements,
            var.dim_sizes,
            var.dim_vary,
            var.record_vary,
            var.max_rec,
        );
    }
}

fn epoch_var(file: &CdfFile) -> Option<&omegaflow::cdf::CdfVar> {
    file.var("Epoch")
        .or_else(|| file.vars.iter().find(|v| is_epoch_type(v.data_type)))
}

fn roti_var(file: &CdfFile) -> Option<&omegaflow::cdf::CdfVar> {
    file.var(ROTI_VAR)
}

fn axis(file: &CdfFile, bytes: &[u8], name: &str) -> Option<Vec<f64>> {
    let var = file.var(name)?;
    let records = file.var_records(bytes, var).ok()?;
    let mut out = Vec::new();
    for (_, values) in records {
        out.extend(values);
    }
    if out.is_empty() { None } else { Some(out) }
}

fn parse_records(file: &CdfFile, bytes: &[u8]) -> Option<Vec<[f64; FIELDS]>> {
    let epoch = epoch_var(file)?;
    let roti = roti_var(file)?;
    let lat = axis(file, bytes, LAT_VAR)?;
    let lon = axis(file, bytes, LON_VAR)?;
    let epoch_map = file.epoch_map(bytes, epoch).ok()?;
    let records = file.var_records(bytes, roti).ok()?;
    let mut out: Vec<[f64; FIELDS]> = Vec::new();
    for (rec, values) in records {
        let Some(&t) = epoch_map.get(&rec) else {
            continue;
        };
        if !value_present(t) {
            continue;
        }
        for (k, value) in values.iter().enumerate() {
            let (i, j) = (k / lon.len(), k % lon.len());
            let (Some(&la), Some(&lo)) = (lat.get(i), lon.get(j)) else {
                break;
            };
            if !value_present(*value) {
                continue;
            }
            out.push([la, lo, t, *value]);
        }
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

fn list_cdf_urls(html: &str, base: &str) -> Vec<String> {
    let mut out = Vec::new();
    for part in html.split("href=\"").skip(1) {
        let Some(end) = part.find('"') else {
            continue;
        };
        let href = &part[..end];
        if href.to_ascii_lowercase().ends_with(".cdf") {
            out.push(format!("{base}{href}"));
        }
    }
    out.sort();
    out.dedup();
    out
}

fn emit_records(records: &[[f64; FIELDS]], out: &str, origin: Option<&str>, void: usize) {
    if records.is_empty() {
        eprintln!("no records — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
    let bytes_out = write_bin(records);
    if std::fs::write(out, &bytes_out).is_err() {
        eprintln!("write {out} returned void");
        std::process::exit(1);
    }
    let t_min = records.first().map(|r| r[2]).unwrap_or(f64::NAN);
    let t_max = records.last().map(|r| r[2]).unwrap_or(f64::NAN);
    let v_min = records.iter().map(|r| r[3]).fold(f64::INFINITY, f64::min);
    let v_max = records
        .iter()
        .map(|r| r[3])
        .fold(f64::NEG_INFINITY, f64::max);
    eprintln!(
        "{out}: {} records ({} files void), ROTI [{}, {}] TECU/min, t [{}, {}] unix",
        records.len(),
        void,
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
    if let Some(src) = origin {
        println!("origin {src}");
    }
    println!("compiler {COMPILER}");
    println!("sha256 {}", sha256_hex(&bytes_out));
    println!("format cdaweb_roti");
    println!("ttl 86400");
    println!("at earth");
    println!("cmap .");
    println!("lat lat");
    println!("lon lon");
    println!(
        "quantity cdaweb_roti_tecu_min cdaweb_roti_tecu_min inverse-square scale TECU/min 900 0.0 0.0"
    );
}

fn emit_urls(urls: &[String], out: &str, origin: Option<&str>) {
    let mut records: Vec<[f64; FIELDS]> = Vec::new();
    let mut void = 0usize;
    for (i, url) in urls.iter().enumerate() {
        let Some(bytes) = fetch_raw_bytes(url) else {
            void += 1;
            eprintln!("{url}: the file stays unfetched");
            continue;
        };
        let file = match CdfFile::parse(&bytes) {
            Ok(f) => f,
            Err(note) => {
                void += 1;
                eprintln!("{url}: {:?}", note);
                continue;
            }
        };
        match parse_records(&file, &bytes) {
            Some(mut part) => records.append(&mut part),
            None => {
                void += 1;
                eprintln!("{url}: {ROTI_VAR} or Epoch absent — the file carries no map");
            }
        }
        if (i + 1) % 40 == 0 || i + 1 == urls.len() {
            eprintln!(
                "{}/{} files measured, {} records",
                i + 1,
                urls.len(),
                records.len()
            );
        }
    }
    emit_records(&records, out, origin, void);
}

fn compile_dir(url: &str, out: &str) {
    let base = if url.ends_with('/') {
        url.to_string()
    } else {
        format!("{url}/")
    };
    let Some(html) = fetch_raw_bytes(&base) else {
        eprintln!("{base}: the listing stays unfetched");
        std::process::exit(1);
    };
    let html = String::from_utf8_lossy(&html);
    let urls = list_cdf_urls(&html, &base);
    eprintln!("{base}: {} cdf files listed", urls.len());
    if urls.is_empty() {
        eprintln!("{base}: no cdf href — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
    emit_urls(&urls, out, Some(&base));
}

fn compile_url(url: &str, out: &str) {
    emit_urls(&[url.to_string()], out, Some(url));
}

fn compile_file(path: &str, out: &str) {
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
    let Some(records) = parse_records(&file, &bytes) else {
        eprintln!("{path}: {ROTI_VAR} or Epoch absent — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    };
    emit_records(&records, out, Some(path), 0);
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
        None => "cdaweb_roti.bin".to_string(),
    };
    if let Some(y) = arg_value(&args, "--year") {
        let url = format!("{BASE_URL}{y}/");
        compile_dir(&url, &out);
    } else if let Some(url) = arg_value(&args, "--dir") {
        compile_dir(&url, &out);
    } else if let Some(url) = arg_value(&args, "--url") {
        compile_url(&url, &out);
    } else if let Some(path) = arg_value(&args, "--file") {
        compile_file(&path, &out);
    } else {
        eprintln!(
            "usage: cdaweb_roti_compiler --probe <cdf> | --year <YYYY> | --dir <url> | --url <cdf> | --file <cdf> [--out <bin>] [--ci-mode]"
        );
        std::process::exit(2);
    }
    if ci_mode && !omegaflow::cdn::upload_release(NETLOC, &out) {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn empty_file() -> CdfFile {
        CdfFile {
            version: (3, 0, 0),
            encoding: 1,
            little_endian: false,
            majority: 2,
            vars: Vec::new(),
            num_att: 0,
            eof: 0,
        }
    }

    #[test]
    fn an_absent_roti_variable_is_none() {
        let file = empty_file();
        assert!(roti_var(&file).is_none());
        assert!(epoch_var(&file).is_none());
        assert!(axis(&file, &[], LAT_VAR).is_none());
        assert!(parse_records(&file, &[]).is_none());
    }

    #[test]
    fn bin_roundtrip_preserves_records() {
        let records = vec![
            [-90.0, -180.0, 1_354_416_000.0, 1.5],
            [87.5, 175.0, 1_354_416_900.0, 0.0],
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
    fn cdf_fill_is_absent_and_zero_is_a_measurement() {
        assert!(!value_present(omegaflow::cdf::FILL_F64));
        assert!(value_present(0.0));
    }

    #[test]
    fn listing_extracts_only_cdf_hrefs() {
        let html = r#"<a href="gps_roti15min_jpl_20121201_v01.cdf">a</a>
            <a href="SHA1SUM">b</a>
            <a href="gps_roti15min_jpl_20121202_v01.cdf">c</a>"#;
        let urls = list_cdf_urls(
            html,
            "https://cdaweb.gsfc.nasa.gov/pub/data/gps/roti15min_jpl/2012/",
        );
        assert_eq!(urls.len(), 2);
        assert!(urls[0].ends_with("20121201_v01.cdf"));
        assert!(urls[1].ends_with("20121202_v01.cdf"));
    }
}
