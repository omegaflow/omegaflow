use omegaflow::cdn::upload_release;
use omegaflow::hdf5::{Hdf5Attribute, Hdf5File};
use omegaflow::lsk::days_from_civil;
use std::path::{Path, PathBuf};
use std::process::Command;

const MAGIC: &[u8; 4] = b"CPCP";
const FIELDS: usize = 3;
const NETLOC: &str = "zenodo.org";
const OUT_PATH: &str = "superdarn_cpcp_nc.bin";
const COMPILER: &str = "tools/harvest/src/bin/superdarn_cpcp_nc_compiler.rs";
const ORIGIN: &str = "https://zenodo.org/api/records/10875060/files/superDARN.zip/content";
const TTL: f64 = 86400.0;
const POT_DROP_VAR: &str = "map.pot.drop";
const STIME_VAR: &str = "map.stime";
const ETIME_VAR: &str = "map.etime";
const POT_DROP_VOLTS_PER_KV: f64 = 1000.0;
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

fn curl_bytes(url: &str) -> Option<Vec<u8>> {
    let out = Command::new("curl")
        .arg("-sSL")
        .arg("-m")
        .arg("600")
        .arg(url)
        .output()
        .ok()?;
    if out.status.success() {
        Some(out.stdout)
    } else {
        None
    }
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

fn records_from_nc(bytes: &[u8]) -> Result<Vec<[f64; FIELDS]>, String> {
    let file = Hdf5File::parse(bytes).map_err(|note| format!("HDF5 parse void: {note:?}"))?;
    let pot = file
        .read_f64_dataset(POT_DROP_VAR)
        .map_err(|note| format!("{POT_DROP_VAR} stays unread: {note:?}"))?;
    let (stime, s_scale, s_epoch) =
        time_axis(&file, STIME_VAR).ok_or_else(|| format!("{STIME_VAR} axis absent"))?;
    let (etime, e_scale, e_epoch) =
        time_axis(&file, ETIME_VAR).ok_or_else(|| format!("{ETIME_VAR} axis absent"))?;
    if pot.len() != stime.len() || pot.len() != etime.len() {
        return Err(format!(
            "length mismatch: pot {} stime {} etime {}",
            pot.len(),
            stime.len(),
            etime.len()
        ));
    }
    let mut out = Vec::new();
    for i in 0..pot.len() {
        let value = pot[i];
        if !value.is_finite() {
            continue;
        }
        let start = s_epoch + stime[i] * s_scale;
        let end = e_epoch + etime[i] * e_scale;
        if !start.is_finite() || !end.is_finite() {
            continue;
        }
        out.push([start, value / POT_DROP_VOLTS_PER_KV, end]);
    }
    if out.is_empty() {
        return Err("no finite map.pot.drop sample — the bin stays unwritten".to_string());
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
                None => eprintln!(
                    "var {name} dataspace absent class {:?} size {:?}",
                    obj.datatype.as_ref().map(|d| d.class),
                    obj.datatype.as_ref().map(|d| d.size)
                ),
            }
            for a in &obj.attrs {
                eprintln!("    attr {} = {:?}", a.name, attr_text(a));
            }
        }
    }
    match records_from_nc(&bytes) {
        Ok(records) => {
            let t_min = records.iter().map(|r| r[0]).fold(f64::INFINITY, f64::min);
            let t_max = records
                .iter()
                .map(|r| r[0])
                .fold(f64::NEG_INFINITY, f64::max);
            let v_min = records.iter().map(|r| r[1]).fold(f64::INFINITY, f64::min);
            let v_max = records
                .iter()
                .map(|r| r[1])
                .fold(f64::NEG_INFINITY, f64::max);
            let e_min = records.iter().map(|r| r[2]).fold(f64::INFINITY, f64::min);
            let e_max = records
                .iter()
                .map(|r| r[2])
                .fold(f64::NEG_INFINITY, f64::max);
            eprintln!(
                "{path}: {} records, t_unix [{t_min}, {t_max}], pot_drop_kv [{v_min}, {v_max}], end_unix [{e_min}, {e_max}]",
                records.len()
            );
            for r in records.iter().take(3) {
                eprintln!("  sample t {} pot_kv {} end {}", r[0], r[1], r[2]);
            }
        }
        Err(note) => eprintln!("{path}: record extraction void: {note}"),
    }
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

fn unzip_into(zip_bytes: &[u8], dest: &Path) -> Option<Vec<PathBuf>> {
    let zip_path = dest.join("superdarn.zip");
    if std::fs::write(&zip_path, zip_bytes).is_err() {
        return None;
    }
    let out = Command::new("unzip")
        .arg("-oq")
        .arg(&zip_path)
        .arg("-d")
        .arg(dest)
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let mut files = Vec::new();
    walk_nc(dest, &mut files);
    files.sort();
    Some(files)
}

fn emit(records: &mut Vec<[f64; FIELDS]>, out: &str) {
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
        "{out}: {} records, {} B; pot_drop_kv [{}, {}]",
        records.len(),
        bytes_out.len(),
        records.iter().map(|r| r[1]).fold(f64::INFINITY, f64::min),
        records
            .iter()
            .map(|r| r[1])
            .fold(f64::NEG_INFINITY, f64::max),
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
    println!(
        "url https://github.com/omegaflow/sources/releases/download/{NETLOC}/superdarn_cpcp_nc.bin"
    );
    println!("origin {ORIGIN}");
    println!("compiler {COMPILER}");
    println!(
        "sha256 {}",
        omegaflow::archivar::sha256::sha256_hex(&bytes_out)
    );
    println!("format superdarn_cpcp_nc");
    println!("ttl {}", TTL as u64);
    println!("at earth");
    println!(
        "field superdarn_cpcp_pot_drop superdarn_cpcp_pot_drop inverse-square electric kV {TTL} 0.0 0.0"
    );
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
        match omegaflow::archivar::extract::series_parse_bin("superdarn_cpcp_nc", &bytes) {
            Some(series) if !series.is_empty() => {
                let t_min = series.iter().map(|s| s.0).fold(f64::INFINITY, f64::min);
                let t_max = series.iter().map(|s| s.0).fold(f64::NEG_INFINITY, f64::max);
                eprintln!(
                    "{path}: {} series rows parse under format superdarn_cpcp_nc, tdb [{t_min}, {t_max}]",
                    series.len()
                );
            }
            _ => {
                eprintln!("{path}: the format superdarn_cpcp_nc reader took not the bin");
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

    let mut bytes_list: Vec<(String, Vec<u8>)> = Vec::new();
    for path in arg_values(&args, "--file") {
        match std::fs::read(&path) {
            Ok(b) => bytes_list.push((path, b)),
            Err(_) => {
                eprintln!("{path}: the file stays unread");
                std::process::exit(1);
            }
        }
    }
    for dir in arg_values(&args, "--dir") {
        let mut files = Vec::new();
        walk_nc(Path::new(&dir), &mut files);
        files.sort();
        if files.is_empty() {
            eprintln!("{dir}: no .nc files under the directory");
            std::process::exit(1);
        }
        for path in files {
            match std::fs::read(&path) {
                Ok(b) => bytes_list.push((path.display().to_string(), b)),
                Err(_) => {
                    eprintln!("{}: the file stays unread", path.display());
                    std::process::exit(1);
                }
            }
        }
    }
    for url in arg_values(&args, "--url") {
        let bytes = match curl_bytes(&url) {
            Some(b) => b,
            None => {
                eprintln!("{url}: fetch void — the bin stays unwritten (0 honored)");
                std::process::exit(1);
            }
        };
        if bytes.starts_with(&ZIP_MAGIC) {
            let tmp = std::env::temp_dir().join(format!("superdarn_nc_{}", std::process::id()));
            if std::fs::create_dir_all(&tmp).is_err() {
                eprintln!("{url}: the zip scratch dir stays uncreated");
                std::process::exit(1);
            }
            let files = match unzip_into(&bytes, &tmp) {
                Some(f) if !f.is_empty() => f,
                _ => {
                    eprintln!("{url}: unzip returned void");
                    std::process::exit(1);
                }
            };
            for path in files {
                match std::fs::read(&path) {
                    Ok(b) => bytes_list.push((path.display().to_string(), b)),
                    Err(_) => {
                        eprintln!("{}: the file stays unread", path.display());
                        std::process::exit(1);
                    }
                }
            }
        } else {
            bytes_list.push((url, bytes));
        }
    }
    if bytes_list.is_empty() {
        eprintln!(
            "usage: superdarn_cpcp_nc_compiler --probe <nc> | --verify-read <bin> | --file <nc> [--file ...] | --dir <dir> | --url <nc|zip> [--out-bin <bin>] [--ci-mode]"
        );
        std::process::exit(2);
    }

    let mut records: Vec<[f64; FIELDS]> = Vec::new();
    let mut skipped = 0usize;
    for (label, bytes) in &bytes_list {
        match records_from_nc(bytes) {
            Ok(mut rs) => records.append(&mut rs),
            Err(note) => {
                skipped += 1;
                eprintln!("{label}: {note} — skipped (0 honored)");
            }
        }
    }
    if skipped == bytes_list.len() {
        eprintln!(
            "no .nc file carried a finite map.pot.drop — the bin stays unwritten (0 honored)"
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
    fn units_parse_minutes_since() {
        let (scale, epoch) = parse_units("minutes since 2018-05-01 00:02:00").unwrap();
        assert_eq!(scale, 60.0);
        assert_eq!(epoch, 1_525_145_720.0);
    }

    #[test]
    fn units_parse_days_since_and_reject_absent_time() {
        let (scale, epoch) = parse_units("days since 2000-01-01 00:00:00").unwrap();
        assert_eq!(scale, 86400.0);
        assert_eq!(epoch, 946_684_800.0);
        assert!(parse_units("days since 2000-01-01").is_none());
        assert!(parse_units("hPa").is_none());
    }

    #[test]
    fn bin_roundtrip_preserves_records() {
        let records = vec![
            [1_525_145_720.0, 42.5, 120.0],
            [1_525_145_840.0, 38.0, 120.0],
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
}
