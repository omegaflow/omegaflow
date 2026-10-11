use omegaflow::archivar::fetch::{RetryPolicy, fetch_raw_to_file_with};
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::upload_release;
use omegaflow::hdf5::{Hdf5Attribute, Hdf5File};
use omegaflow::lsk::days_from_civil;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;

const MAGIC: &[u8; 4] = b"GRDC";
const FIELDS: usize = 4;
const NETLOC: &str = "zenodo.org";
const OUT_PATH: &str = "grdc_caravan.bin";
const COMPILER: &str = "tools/harvest/src/bin/grdc_caravan_compiler.rs";
const ORIGIN: &str =
    "https://zenodo.org/api/records/15349031/files/GRDC_Caravan_extension_nc.zip/content";
const TTL: f64 = 86400.0;
const GRDC_TRANSFER_BOUND_S: u64 = 3 * 3600;

const STREAMFLOW_VAR: &str = "streamflow";
const TIME_VAR: &str = "time";
const NC_DIR: &str = "GRDC_Caravan_extension_nc/timeseries/netcdf/grdc";
const ATTRS_MEMBER: &str =
    "GRDC_Caravan_extension_nc/attributes/grdc/attributes_additional_grdc.csv";
const ATTRS_FILE: &str = "attributes_additional_grdc.csv";

const HDF5_MAGIC: [u8; 4] = [0x89, b'H', b'D', b'F'];
const ZIP_MAGIC: [u8; 4] = *b"PK\x03\x04";

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

fn fetch_to_file(url: &str, path: &Path) -> bool {
    fetch_raw_to_file_with(url, RetryPolicy::All, GRDC_TRANSFER_BOUND_S, path).is_some()
}

fn attr_text(attr: &Hdf5Attribute) -> String {
    String::from_utf8_lossy(&attr.data)
        .trim_end_matches('\0')
        .trim()
        .to_string()
}

fn parse_units(units: &str) -> Option<(f64, f64)> {
    let idx = units.find(" since ")?;
    let scale: f64 = match units[..idx].trim() {
        "second" | "seconds" => 1.0,
        "minute" | "minutes" => 60.0,
        "hour" | "hours" => 3600.0,
        "day" | "days" => 86400.0,
        _ => return None,
    };
    let rest = units[idx + " since ".len()..].trim();
    let mut parts = rest.split_whitespace();
    let date = parts.next()?;
    let time = parts.next()?;
    let mut d = date.split('-');
    let year: i64 = d.next()?.parse().ok()?;
    let month: i64 = d.next()?.parse().ok()?;
    let day: i64 = d.next()?.parse().ok()?;
    let mut hms = time.split(':');
    let hour: i64 = hms.next()?.parse().ok()?;
    let minute: i64 = hms.next()?.parse().ok()?;
    let second: f64 = hms.next()?.parse().ok()?;
    let days = days_from_civil(year, month, day)?;
    let epoch = days as f64 * 86400.0 + hour as f64 * 3600.0 + minute as f64 * 60.0 + second;
    (scale.is_finite() && epoch.is_finite()).then_some((scale, epoch))
}

fn time_axis(file: &Hdf5File, name: &str) -> Option<(Vec<f64>, f64, f64)> {
    let units = attr_text(file.attribute(name, "units")?);
    let (scale, epoch) = parse_units(&units)?;
    let values = file.read_f64_dataset(name).ok()?;
    Some((values, scale, epoch))
}

fn records_from_nc(bytes: &[u8], lat: f64, lon: f64) -> Result<Vec<[f64; FIELDS]>, String> {
    let file = Hdf5File::parse(bytes).map_err(|note| format!("HDF5 parse void: {note:?}"))?;
    let flow = file
        .read_f64_dataset(STREAMFLOW_VAR)
        .map_err(|note| format!("{STREAMFLOW_VAR} stays unread: {note:?}"))?;
    let (time, scale, epoch) =
        time_axis(&file, TIME_VAR).ok_or_else(|| format!("{TIME_VAR} axis absent"))?;
    if flow.len() != time.len() {
        return Err(format!(
            "length mismatch: streamflow {} time {}",
            flow.len(),
            time.len()
        ));
    }
    let fill = file.attr_f64(STREAMFLOW_VAR, "_FillValue");
    let mut out = Vec::new();
    for i in 0..flow.len() {
        let value = flow[i];
        if !value.is_finite() {
            continue;
        }
        if let Some(f) = fill
            && value == f
        {
            continue;
        }
        let t = epoch + time[i] * scale;
        if !t.is_finite() {
            continue;
        }
        out.push([t, lat, lon, value]);
    }
    if out.is_empty() {
        return Err("no finite streamflow sample — the bin stays unwritten".to_string());
    }
    Ok(out)
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

fn csv_fields(line: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut in_quotes = false;
    let mut chars = line.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '"' => {
                if in_quotes && chars.peek() == Some(&'"') {
                    cur.push('"');
                    chars.next();
                } else {
                    in_quotes = !in_quotes;
                }
            }
            ',' if !in_quotes => out.push(std::mem::take(&mut cur)),
            _ => cur.push(c),
        }
    }
    out.push(cur);
    out
}

fn read_attrs(path: &Path) -> HashMap<String, (f64, f64)> {
    let mut map = HashMap::new();
    let Ok(text) = std::fs::read_to_string(path) else {
        return map;
    };
    let mut lines = text.lines();
    let Some(header) = lines.next() else {
        return map;
    };
    let cols = csv_fields(header);
    let id_i = cols.iter().position(|c| c.trim() == "gauge_id");
    let lat_i = cols.iter().position(|c| c.trim() == "lat_pp");
    let lon_i = cols.iter().position(|c| c.trim() == "long_pp");
    let (Some(id_i), Some(lat_i), Some(lon_i)) = (id_i, lat_i, lon_i) else {
        return map;
    };
    for line in lines {
        if line.trim().is_empty() {
            continue;
        }
        let f = csv_fields(line);
        let (Some(id), Some(lat), Some(lon)) = (f.get(id_i), f.get(lat_i), f.get(lon_i)) else {
            continue;
        };
        if let (Ok(lat), Ok(lon)) = (lat.trim().parse::<f64>(), lon.trim().parse::<f64>())
            && lat.is_finite()
            && lon.is_finite()
        {
            map.insert(id.trim().to_string(), (lat, lon));
        }
    }
    map
}

fn walk_nc(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            walk_nc(&path, out);
        } else if path.extension().and_then(|e| e.to_str()) == Some("nc") {
            out.push(path);
        }
    }
}

fn find_file(dir: &Path, name: &str) -> Option<PathBuf> {
    let entries = std::fs::read_dir(dir).ok()?;
    let mut dirs = Vec::new();
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            dirs.push(path);
        } else if path.file_name().and_then(|n| n.to_str()) == Some(name) {
            return Some(path);
        }
    }
    for d in dirs {
        if let Some(p) = find_file(&d, name) {
            return Some(p);
        }
    }
    None
}

fn unzip_members(zip: &Path, dest: &Path) {
    let pattern = format!("{NC_DIR}/*.nc");
    let _ = Command::new("unzip")
        .arg("-oq")
        .arg(zip)
        .arg(pattern)
        .arg(ATTRS_MEMBER)
        .arg("-d")
        .arg(dest)
        .status();
}

fn probe_path(path: &str) {
    let Ok(bytes) = std::fs::read(path) else {
        eprintln!("{path}: the file stays unread");
        std::process::exit(1);
    };
    if !bytes.starts_with(&HDF5_MAGIC) {
        eprintln!(
            "{path}: first bytes {:02x?} — no HDF5 signature",
            &bytes[..bytes.len().min(8)]
        );
        std::process::exit(1);
    }
    let file = match Hdf5File::parse(&bytes) {
        Ok(f) => f,
        Err(note) => {
            eprintln!("{path}: HDF5 parse returned void: {note:?}");
            std::process::exit(1);
        }
    };
    for link in file.links_of("") {
        let name = link.name.clone();
        if let Ok(obj) = file.resolve(&name) {
            match obj.dataspace.as_ref() {
                Some(ds) => eprintln!(
                    "var {name} dims {:?} class {:?} size {:?}",
                    ds.dims,
                    obj.datatype.as_ref().map(|d| d.class),
                    obj.datatype.as_ref().map(|d| d.size)
                ),
                None => eprintln!("var {name} dataspace absent"),
            }
            for a in &obj.attrs {
                eprintln!("    attr {} = {:?}", a.name, attr_text(a));
            }
        }
    }
    match records_from_nc(&bytes, 0.0, 0.0) {
        Ok(records) => {
            let v_min = records.iter().map(|r| r[3]).fold(f64::INFINITY, f64::min);
            let v_max = records
                .iter()
                .map(|r| r[3])
                .fold(f64::NEG_INFINITY, f64::max);
            let t_min = records.iter().map(|r| r[0]).fold(f64::INFINITY, f64::min);
            let t_max = records
                .iter()
                .map(|r| r[0])
                .fold(f64::NEG_INFINITY, f64::max);
            eprintln!(
                "{path}: {} records, streamflow [{v_min}, {v_max}] mm/d, t_unix [{t_min}, {t_max}]",
                records.len()
            );
        }
        Err(note) => eprintln!("{path}: record extraction void: {note}"),
    }
}

fn emit(records: &mut Vec<[f64; FIELDS]>, out: &str) {
    if records.is_empty() {
        eprintln!("no records — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
    records.sort_by(|a, b| a[0].total_cmp(&b[0]));
    let bytes_out = write_bin(records);
    if let Some(parent) = Path::new(out).parent()
        && !parent.as_os_str().is_empty()
    {
        std::fs::create_dir_all(parent).ok();
    }
    if std::fs::write(out, &bytes_out).is_err() {
        eprintln!("write {out} returned void");
        std::process::exit(1);
    }
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
    let v_min = records.iter().map(|r| r[3]).fold(f64::INFINITY, f64::min);
    let v_max = records
        .iter()
        .map(|r| r[3])
        .fold(f64::NEG_INFINITY, f64::max);
    eprintln!(
        "{out}: {} records, streamflow [{v_min}, {v_max}] mm/d",
        records.len()
    );
    let out_name = Path::new(out)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(OUT_PATH);
    println!("url https://github.com/omegaflow/sources/releases/download/{NETLOC}/{out_name}");
    println!("origin {ORIGIN}");
    println!("compiler {COMPILER}");
    println!("sha256 {}", sha256_hex(&bytes_out));
    println!("format grdc_caravan");
    println!("ttl {}", TTL as u64);
    println!("at earth");
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Some(path) = arg_value(&args, "--probe") {
        probe_path(&path);
        return;
    }
    if let Some(path) = arg_value(&args, "--verify-read") {
        let Ok(bytes) = std::fs::read(&path) else {
            eprintln!("{path}: the file stays unread");
            std::process::exit(1);
        };
        match parse_bin(&bytes) {
            Some(parsed) if !parsed.is_empty() => {
                eprintln!(
                    "{path}: {} records parse under format grdc_caravan",
                    parsed.len()
                );
            }
            _ => {
                eprintln!("{path}: the format grdc_caravan reader took not the bin");
                std::process::exit(1);
            }
        }
        return;
    }
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = match arg_value(&args, "--out-bin") {
        Some(v) => v,
        None => match arg_value(&args, "--out") {
            Some(v) => v,
            None => OUT_PATH.to_string(),
        },
    };

    let mut attrs = match arg_value(&args, "--attrs") {
        Some(p) => read_attrs(Path::new(&p)),
        None => HashMap::new(),
    };
    let mut nc_files: Vec<PathBuf> = Vec::new();

    for dir in arg_values(&args, "--dir") {
        walk_nc(Path::new(&dir), &mut nc_files);
        if attrs.is_empty()
            && let Some(p) = find_file(Path::new(&dir), ATTRS_FILE)
        {
            attrs = read_attrs(&p);
        }
    }

    let mut zips: Vec<PathBuf> = Vec::new();
    for path in arg_values(&args, "--file") {
        let p = PathBuf::from(&path);
        match std::fs::read(&p) {
            Ok(bytes) if bytes.starts_with(&ZIP_MAGIC) => zips.push(p),
            Ok(bytes) if bytes.starts_with(&HDF5_MAGIC) => nc_files.push(p),
            Ok(_) => eprintln!("{path}: neither zip nor HDF5 — skipped (0 honored)"),
            Err(_) => {
                eprintln!("{path}: the file stays unread");
                std::process::exit(1);
            }
        }
    }
    for url in arg_values(&args, "--url") {
        let tmp = std::env::temp_dir().join(format!("grdc_{}.zip", std::process::id()));
        if !fetch_to_file(&url, &tmp) {
            eprintln!("{url}: fetch void — the bin stays unwritten (0 honored)");
            std::process::exit(1);
        }
        zips.push(tmp);
    }

    if !zips.is_empty() {
        let scratch = std::env::temp_dir().join(format!("grdc_nc_{}", std::process::id()));
        if std::fs::create_dir_all(&scratch).is_err() {
            eprintln!("the zip scratch dir stays uncreated");
            std::process::exit(1);
        }
        for z in &zips {
            unzip_members(z, &scratch);
        }
        walk_nc(&scratch, &mut nc_files);
        if attrs.is_empty()
            && let Some(p) = find_file(&scratch, ATTRS_FILE)
        {
            attrs = read_attrs(&p);
        }
    }

    if nc_files.is_empty() {
        eprintln!(
            "usage: grdc_caravan_compiler --url <zip> | --file <zip|nc> [--file ...] | --dir <dir> [--attrs <csv>] [--out <bin>] [--ci-mode] | --probe <nc> | --verify-read <bin>"
        );
        std::process::exit(2);
    }

    nc_files.sort();
    let mut records: Vec<[f64; FIELDS]> = Vec::new();
    let mut skipped = 0usize;
    for path in &nc_files {
        let gauge = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_string();
        let Some((lat, lon)) = attrs.get(&gauge).copied() else {
            skipped += 1;
            eprintln!(
                "{}: no lat_pp/long_pp for {gauge} in the attributes — skipped (0 honored)",
                path.display()
            );
            continue;
        };
        let Ok(bytes) = std::fs::read(path) else {
            skipped += 1;
            eprintln!("{}: the file stays unread", path.display());
            continue;
        };
        match records_from_nc(&bytes, lat, lon) {
            Ok(mut rs) => records.append(&mut rs),
            Err(note) => {
                skipped += 1;
                eprintln!("{}: {note} — skipped (0 honored)", path.display());
            }
        }
    }
    if records.is_empty() {
        eprintln!(
            "no .nc file carried a finite streamflow sample — the bin stays unwritten (0 honored; {skipped} skipped)"
        );
        std::process::exit(1);
    }
    emit(&mut records, &out);
    if ci_mode && !upload_release(NETLOC, &out) {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn units_parse_days_since() {
        let (scale, epoch) = parse_units("days since 1950-01-02 00:00:00").unwrap();
        assert_eq!(scale, 86400.0);
        assert_eq!(epoch, -631_065_600.0);
        assert!(parse_units("days since 1950-01-02").is_none());
        assert!(parse_units("mm/d").is_none());
    }

    #[test]
    fn bin_roundtrip_preserves_records() {
        let records = vec![
            [0.0, -28.7563, 17.7188, 1.5],
            [86400.0, -28.7563, 17.7188, 2.0],
        ];
        let bytes = write_bin(&records);
        assert_eq!(bytes.len(), 8 + records.len() * FIELDS * 8);
        assert_eq!(parse_bin(&bytes).as_deref(), Some(records.as_slice()));
    }

    #[test]
    fn parse_bin_rejects_foreign_magic_and_short_body() {
        assert!(parse_bin(b"XXXX").is_none());
        let good = write_bin(&[[0.0; FIELDS]]);
        assert!(parse_bin(&good[..good.len() - 1]).is_none());
    }

    #[test]
    fn csv_fields_respects_quoted_commas() {
        let f = csv_fields("GRDC_1,1.0,\"a, b\",-28.7,17.7");
        assert_eq!(f, vec!["GRDC_1", "1.0", "a, b", "-28.7", "17.7"]);
    }

    #[test]
    fn read_attrs_reads_lat_lon_by_header() {
        let dir = std::env::temp_dir();
        let path = dir.join(format!("grdc_attrs_test_{}.csv", std::process::id()));
        let mut s = String::from("gauge_id,altitude,lat_pp,long_pp\n");
        s.push_str("GRDC_1159100,152.0,-28.7563,17.7188\n");
        std::fs::write(&path, s).unwrap();
        let map = read_attrs(&path);
        assert_eq!(map.get("GRDC_1159100").copied(), Some((-28.7563, 17.7188)));
        std::fs::remove_file(&path).ok();
    }
}
