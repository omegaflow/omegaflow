use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::upload_release;
use omegaflow::hdf5::Hdf5File;
use omegaflow::lsk::days_from_civil;
use omegaflow::netcdf::nc4_group;

const MAGIC: &[u8; 4] = b"SSUI";
const FIELDS: usize = 3;
const NETLOC: &str = "cdaweb.gsfc.nasa.gov";
const OUT_PATH: &str = "ssusi_aurora.bin";
const COMPILER: &str = "tools/harvest/src/bin/ssusi_compiler.rs";
const ORIGIN: &str = "https://cdaweb.gsfc.nasa.gov/pub/data/dmsp/dmspf16/ssusi/data/edr-aurora/";
const TTL: f64 = 86400.0;

const YEAR_VAR: &str = "YEAR";
const DOY_VAR: &str = "DOY";
const TIME_VAR: &str = "TIME";
const POWER_NORTH: &str = "HEMISPHERE_POWER_NORTH";
const POWER_SOUTH: &str = "HEMISPHERE_POWER_SOUTH";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn arg_values(args: &[String], name: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut i = 0;
    while i < args.len() {
        if args[i] == name {
            if let Some(v) = args.get(i + 1) {
                out.push(v.clone());
            }
            i += 2;
        } else {
            i += 1;
        }
    }
    out
}

fn value_present(v: f64) -> bool {
    v.is_finite() && v > 0.0
}

fn scalar(file: &Hdf5File, name: &str) -> Option<f64> {
    file.read_f64_dataset(name).ok()?.first().copied()
}

fn epoch_unix(year: f64, doy: f64, seconds_of_day: f64) -> Option<f64> {
    if !year.is_finite() || !doy.is_finite() || !seconds_of_day.is_finite() {
        return None;
    }
    if doy < 1.0 || seconds_of_day < 0.0 {
        return None;
    }
    let jan_first = days_from_civil(year as i64, 1, 1)? as f64;
    Some((jan_first + doy - 1.0) * 86400.0 + seconds_of_day)
}

fn parse_record(bytes: &[u8]) -> Option<[f64; FIELDS]> {
    let mut file = Hdf5File::parse(bytes).ok()?;
    let group = nc4_group(&mut file, "").ok()?;
    for required in [YEAR_VAR, DOY_VAR, TIME_VAR, POWER_NORTH, POWER_SOUTH] {
        if !group.variables.iter().any(|v| v.name == required) {
            return None;
        }
    }
    let year = scalar(&file, YEAR_VAR)?;
    let doy = scalar(&file, DOY_VAR)?;
    let seconds = scalar(&file, TIME_VAR)?;
    let north = scalar(&file, POWER_NORTH)?;
    let south = scalar(&file, POWER_SOUTH)?;
    let epoch = epoch_unix(year, doy, seconds)?;
    if !value_present(epoch) || !value_present(north) || !value_present(south) {
        return None;
    }
    Some([epoch, north, south])
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

fn probe(path: &str) {
    let Ok(bytes) = std::fs::read(path) else {
        eprintln!("{path}: the file stays unread");
        std::process::exit(1);
    };
    let file = match Hdf5File::parse(&bytes) {
        Ok(f) => f,
        Err(note) => {
            eprintln!("{path}: {note:?}");
            std::process::exit(1);
        }
    };
    for link in file.links_of("") {
        let name = link.name.clone();
        match file.resolve(&name) {
            Ok(obj) => {
                let dims = match obj.dataspace.as_ref() {
                    Some(d) => d.dims.clone(),
                    None => Vec::new(),
                };
                let class = obj.datatype.as_ref().map(|d| d.class);
                let size = obj.datatype.as_ref().map(|d| d.size);
                let units = obj.attrs.iter().find(|a| a.name == "UNITS").map(|a| {
                    String::from_utf8_lossy(&a.data)
                        .trim_end_matches('\0')
                        .to_string()
                });
                eprintln!("var {name} dims {dims:?} type {class:?}/{size:?} units {units:?}");
            }
            Err(note) => eprintln!("var {name}: {note:?}"),
        }
    }
}

fn collect(records: &mut Vec<[f64; FIELDS]>, skipped: &mut usize, label: &str, bytes: &[u8]) {
    match parse_record(bytes) {
        Some(r) => records.push(r),
        None => {
            *skipped += 1;
            eprintln!("{label}: no valid record — skipped (0 honored)");
        }
    }
}

fn emit(records: &mut Vec<[f64; FIELDS]>, skipped: usize, out: &str) {
    if records.is_empty() {
        eprintln!("no records — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
    records.sort_by(|a, b| a[0].total_cmp(&b[0]));
    let bytes_out = write_bin(records);
    if std::fs::write(out, &bytes_out).is_err() {
        eprintln!("write {out} returned void");
        std::process::exit(1);
    }
    eprintln!(
        "{out}: {} records ({} skipped), hemisphere power north [{}, {}] GW, south [{}, {}] GW",
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
    println!("format ssusi_aurora");
    println!("ttl {}", TTL as u64);
    println!("at earth");
    println!(
        "field ssusi_hemisphere_power_north ssusi_hemisphere_power_north inverse-square em GW {TTL} 0.0 0.0"
    );
    println!(
        "field ssusi_hemisphere_power_south ssusi_hemisphere_power_south inverse-square em GW {TTL} 0.0 0.0"
    );
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
    let urls = arg_values(&args, "--url");
    let mut files = arg_values(&args, "--file");
    files.extend(arg_values(&args, "--input"));
    if urls.is_empty() && files.is_empty() {
        eprintln!(
            "usage: ssusi_compiler --probe <nc> | --url <nc> [--url ...] | --file <nc> [--file ...] [--out <bin>] [--ci-mode]"
        );
        std::process::exit(2);
    }
    let mut records: Vec<[f64; FIELDS]> = Vec::new();
    let mut skipped = 0usize;
    for url in &urls {
        match fetch_raw_bytes(url) {
            Some(bytes) => collect(&mut records, &mut skipped, url, &bytes),
            None => {
                eprintln!("{url}: fetch void — the bin stays unwritten (0 honored)");
                std::process::exit(1);
            }
        }
    }
    for path in &files {
        match std::fs::read(path) {
            Ok(bytes) => collect(&mut records, &mut skipped, path, &bytes),
            Err(_) => {
                eprintln!("{path}: the file stays unread");
                std::process::exit(1);
            }
        }
    }
    emit(&mut records, skipped, &out);
    if ci_mode && !upload_release(NETLOC, &out) {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bin_roundtrip_preserves_records() {
        let records = vec![[1_700_000_000.0, 12.5, 3.25], [1_700_000_002.0, 0.5, 1.0]];
        let bytes = write_bin(&records);
        assert_eq!(bytes.len(), 8 + records.len() * FIELDS * 8);
        assert_eq!(parse_bin(&bytes).as_deref(), Some(records.as_slice()));
    }

    #[test]
    fn parse_bin_rejects_foreign_magic_and_short_body() {
        assert!(parse_bin(b"XXXX").is_none());
        let good = write_bin(&[[0.0; FIELDS]]);
        let short = good[..good.len() - 1].to_vec();
        assert!(parse_bin(&short).is_none());
    }

    #[test]
    fn absent_value_is_not_a_record() {
        assert!(!value_present(f64::NAN));
        assert!(!value_present(f64::INFINITY));
        assert!(!value_present(0.0));
        assert!(value_present(1.0));
    }

    #[test]
    fn epoch_uses_year_doy_seconds() {
        let t = epoch_unix(2016.0, 236.0, 0.0).unwrap();
        assert!((t - 1_471_910_400.0).abs() < 1.0);
    }
}
