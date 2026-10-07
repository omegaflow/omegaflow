use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdf::{CdfFile, value_present};
use omegaflow::cdn::upload_release;
use std::collections::HashMap;

const MAGIC: &[u8; 4] = b"DS16";
const FIELDS: usize = 6;
const NETLOC: &str = "cdaweb.gsfc.nasa.gov";
const OUT_PATH: &str = "dmsp16_ssj.bin";
const COMPILER: &str = "tools/harvest/src/bin/dmsp16_ssj_compiler.rs";
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
        eprintln!(
            "  var {} {} {} x{} dims {:?} record_vary {} max_rec {}",
            var.var_num,
            var.name,
            type_name(var.data_type),
            var.num_elements,
            var.dim_sizes,
            var.record_vary,
            var.max_rec,
        );
    }
    for name in [
        "SC_GEOCENTRIC_LAT",
        "SC_GEOCENTRIC_LON",
        "SC_GEOCENTRIC_R",
        "ELE_TOTAL_ENERGY_FLUX",
        "ION_TOTAL_ENERGY_FLUX",
        "ELE_AVG_ENERGY",
        "ION_AVG_ENERGY",
    ] {
        match file.var(name) {
            Some(v) => match file.var_records(&bytes, v) {
                Ok(recs) => {
                    let total = recs.len();
                    let present = recs
                        .iter()
                        .filter(|(_, vals)| vals.first().is_some_and(|x| value_present(*x)))
                        .count();
                    eprintln!("  present {name}: {present}/{total}");
                }
                Err(note) => eprintln!("  present {name}: {:?}", note),
            },
            None => eprintln!("  present {name}: absent"),
        }
    }
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
    let named = [
        "SC_GEOCENTRIC_LAT",
        "SC_GEOCENTRIC_LON",
        "SC_GEOCENTRIC_R",
        "ELE_TOTAL_ENERGY_FLUX",
        "ION_TOTAL_ENERGY_FLUX",
    ];
    let mut maps: Vec<HashMap<u32, f64>> = Vec::with_capacity(named.len());
    for name in named {
        let Some(m) = scalar_map(&file, bytes, name) else {
            eprintln!("{name} absent — the bin stays unwritten (0 honored)");
            std::process::exit(1);
        };
        maps.push(m);
    }

    let mut records: Vec<[f64; FIELDS]> = Vec::with_capacity(epoch.len());
    let mut skipped = 0usize;
    for (rec, t) in &epoch {
        let Some(&lat) = maps[0].get(rec) else {
            skipped += 1;
            continue;
        };
        let Some(&lon) = maps[1].get(rec) else {
            skipped += 1;
            continue;
        };
        let Some(&r) = maps[2].get(rec) else {
            skipped += 1;
            continue;
        };
        let Some(&ele_flux) = maps[3].get(rec) else {
            skipped += 1;
            continue;
        };
        let Some(&ion_flux) = maps[4].get(rec) else {
            skipped += 1;
            continue;
        };
        let radius_km = r / 1000.0;
        let present = value_present(*t)
            && value_present(lat)
            && value_present(lon)
            && value_present(radius_km)
            && value_present(ele_flux)
            && value_present(ion_flux);
        if !present || radius_km <= 0.0 {
            skipped += 1;
            continue;
        }
        records.push([lat, lon, radius_km, *t, ele_flux, ion_flux]);
    }
    records.sort_by(|a, b| {
        a[3].total_cmp(&b[3])
            .then(a[0].total_cmp(&b[0]))
            .then(a[1].total_cmp(&b[1]))
    });

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
    let (e0, e1) = span(4);
    let (i0, i1) = span(5);
    eprintln!(
        "{out}: {} records ({} skipped), ele_flux [{e0}, {e1}] ion_flux [{i0}, {i1}]",
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
    println!("format dmsp16_ssj");
    println!("ttl {}", TTL as u64);
    println!("at earth");
    println!("cmap .");
    println!("lat SC_GEOCENTRIC_LAT");
    println!("lon SC_GEOCENTRIC_LON");
    println!(
        "field dmsp16_ssj_ele_total_energy_flux dmsp16_ssj_ele_total_energy_flux inverse-square em eV/cm2/ster/s {} 0.0 0.0",
        TTL as u64
    );
    println!(
        "field dmsp16_ssj_ion_total_energy_flux dmsp16_ssj_ion_total_energy_flux inverse-square em eV/cm2/ster/s {} 0.0 0.0",
        TTL as u64
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
            "usage: dmsp16_ssj_compiler --probe <cdf> | --file <cdf> | --url <cdf> [--out <bin>] [--ci-mode]"
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
            [65.4, 263.2, 7223.9, 1_285_286_400.0, 1.0e10, 2.0e9],
            [-70.1, 10.0, 7100.0, 1_285_286_460.0, 0.0, 0.0],
        ];
        let bytes = write_bin(&records);
        assert_eq!(bytes.len(), 8 + records.len() * FIELDS * 8);
        assert_eq!(parse_bin(&bytes).as_deref(), Some(records.as_slice()));
    }

    #[test]
    fn parse_bin_rejects_foreign_magic_and_short_body() {
        assert!(parse_bin(b"DS16").is_none());
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
