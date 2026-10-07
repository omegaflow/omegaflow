use omegaflow::archivar::netcdf::NetcdfFile;
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::upload_release;
use omegaflow::inflate::inflate;
use omegaflow::lsk::days_from_civil;
use std::process::Command;

const MAGIC: &[u8; 4] = b"AMPF";
const FIELDS: usize = 5;
const NETLOC: &str = "zenodo.org";
const OUT_PATH: &str = "ampere_fac.bin";
const COMPILER: &str = "tools/harvest/src/bin/ampere_fac_compiler.rs";
const TTL: f64 = 86400.0;
const DEFAULT_URL: &str =
    "https://zenodo.org/api/records/22345897/files/ampere.202309.v2.k060_m08.north.grd.zip/content";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn le16(b: &[u8], off: usize) -> Option<u16> {
    Some(u16::from_le_bytes(b.get(off..off + 2)?.try_into().ok()?))
}

fn le32(b: &[u8], off: usize) -> Option<u32> {
    Some(u32::from_le_bytes(b.get(off..off + 4)?.try_into().ok()?))
}

fn curl_range(url: &str, from: u64, to: u64) -> Option<Vec<u8>> {
    let out = Command::new("curl")
        .arg("-sSL")
        .arg("-m")
        .arg("600")
        .arg("-r")
        .arg(format!("{from}-{to}"))
        .arg(url)
        .output()
        .ok()?;
    if out.status.success() {
        Some(out.stdout)
    } else {
        None
    }
}

fn http_size(url: &str) -> Option<u64> {
    let out = Command::new("curl")
        .arg("-sSI")
        .arg("-L")
        .arg("-m")
        .arg("60")
        .arg(url)
        .output()
        .ok()?;
    let head = String::from_utf8_lossy(&out.stdout);
    let mut size = None;
    for line in head.lines() {
        let lower = line.to_ascii_lowercase();
        if let Some(v) = lower.strip_prefix("content-length:") {
            if let Ok(n) = v.trim().parse::<u64>() {
                size = Some(n);
            }
        }
    }
    size
}

struct ZipEntry {
    name: String,
    method: u16,
    comp_size: u64,
    local_offset: u64,
}

fn remote_zip_entries(url: &str) -> Option<Vec<ZipEntry>> {
    let size = http_size(url)?;
    let tail_n = size.min(200_000);
    let tail = curl_range(url, size - tail_n, size - 1)?;
    let mut eocd = None;
    let mut i = tail.len();
    while i >= 4 {
        if tail[i - 4..i] == [0x50, 0x4b, 0x05, 0x06] {
            eocd = Some(i - 4);
            break;
        }
        i -= 1;
    }
    let eo = eocd?;
    let cd_size = le32(&tail, eo + 12)? as u64;
    let cd_off = le32(&tail, eo + 16)? as u64;
    let cd = curl_range(url, cd_off, cd_off + cd_size.saturating_sub(1))?;
    let mut entries = Vec::new();
    let mut p = 0usize;
    while p + 46 <= cd.len() {
        if cd[p..p + 4] != [0x50, 0x4b, 0x01, 0x02] {
            p += 1;
            continue;
        }
        let method = le16(&cd, p + 10)?;
        let comp = le32(&cd, p + 20)? as u64;
        let nlen = le16(&cd, p + 28)? as usize;
        let xlen = le16(&cd, p + 30)? as usize;
        let clen = le16(&cd, p + 32)? as usize;
        let lho = le32(&cd, p + 42)? as u64;
        let name_bytes = cd.get(p + 46..p + 46 + nlen)?;
        entries.push(ZipEntry {
            name: String::from_utf8_lossy(name_bytes).into_owned(),
            method,
            comp_size: comp,
            local_offset: lho,
        });
        p += 46 + nlen + xlen + clen;
    }
    if entries.is_empty() {
        None
    } else {
        Some(entries)
    }
}

fn remote_zip_extract(url: &str, e: &ZipEntry) -> Option<Vec<u8>> {
    let lh = curl_range(url, e.local_offset, e.local_offset + 30)?;
    let nlen = le16(&lh, 26)? as u64;
    let xlen = le16(&lh, 28)? as u64;
    let start = e.local_offset + 30 + nlen + xlen;
    let raw = curl_range(url, start, start + e.comp_size.saturating_sub(1))?;
    if e.method == 0 {
        Some(raw)
    } else {
        inflate(&raw)
    }
}

fn unix_seconds(year: f64, doy: f64, hours: f64) -> Option<f64> {
    if !year.is_finite() || !doy.is_finite() || !hours.is_finite() {
        return None;
    }
    if year < 1900.0 || doy < 1.0 {
        return None;
    }
    let days = days_from_civil(year as i64, 1, 1)?;
    Some(days as f64 * 86400.0 + (doy - 1.0) * 86400.0 + hours * 3600.0)
}

fn grid_record(
    geo_clat_deg: f64,
    geo_lon_deg: f64,
    radius_km: f64,
    epoch: f64,
    jpar: f64,
    fill: Option<f64>,
) -> Option<[f64; FIELDS]> {
    if !geo_clat_deg.is_finite()
        || !geo_lon_deg.is_finite()
        || !radius_km.is_finite()
        || !epoch.is_finite()
        || !jpar.is_finite()
    {
        return None;
    }
    if radius_km <= 0.0 {
        return None;
    }
    if let Some(f) = fill {
        if jpar == f {
            return None;
        }
    }
    Some([90.0 - geo_clat_deg, geo_lon_deg, radius_km, epoch, jpar])
}

fn build_records(
    nrec: usize,
    nobs: usize,
    npnt: &[f64],
    year: &[f64],
    doy: &[f64],
    hours: &[f32],
    geo_clat_deg: &[f32],
    geo_lon_deg: &[f32],
    radius_km: &[f32],
    jpar: &[f64],
    fill: Option<f64>,
) -> Vec<[f64; FIELDS]> {
    let mut records = Vec::new();
    for rec in 0..nrec {
        let Some(&np_raw) = npnt.get(rec) else {
            continue;
        };
        if !np_raw.is_finite() || np_raw < 0.0 {
            continue;
        }
        let np = (np_raw as usize).min(nobs);
        let (Some(&y), Some(&d), Some(&h)) = (year.get(rec), doy.get(rec), hours.get(rec)) else {
            continue;
        };
        let Some(epoch) = unix_seconds(y, d, h as f64) else {
            continue;
        };
        for ob in 0..np {
            let base = rec * nobs + ob;
            let (Some(&cl), Some(&lo), Some(&ra), Some(&jp)) = (
                geo_clat_deg.get(base),
                geo_lon_deg.get(base),
                radius_km.get(base),
                jpar.get(base),
            ) else {
                continue;
            };
            if let Some(r) = grid_record(cl as f64, lo as f64, ra as f64, epoch, jp, fill) {
                records.push(r);
            }
        }
    }
    records
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

fn emit(nc: &[u8], out: &str, source: Option<&str>) {
    let label = match source {
        Some(s) => s,
        None => out,
    };
    let file = match NetcdfFile::parse(nc) {
        Ok(f) => f,
        Err(note) => {
            eprintln!("{label}: {note:?}");
            std::process::exit(1);
        }
    };
    let shape = match file.var("jPar").map(|v| file.var_shape(v)) {
        Some(Ok(s)) => s,
        Some(Err(note)) => {
            eprintln!("{label}: jPar shape void: {note:?}");
            std::process::exit(1);
        }
        None => {
            eprintln!("{label}: jPar absent — the bin stays unwritten (0 honored)");
            std::process::exit(1);
        }
    };
    if shape.len() != 2 {
        eprintln!(
            "{label}: jPar rank {} — the bin stays unwritten (0 honored)",
            shape.len()
        );
        std::process::exit(1);
    }
    let nrec = shape[0] as usize;
    let nobs = shape[1] as usize;
    let Some(jpar) = file.values_f64(nc, "jPar") else {
        eprintln!("{label}: jPar values unread — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    };
    let (Some(npnt), Some(year), Some(doy), Some(hours), Some(clat), Some(lon), Some(radius)) = (
        file.values_numeric(nc, "npnt"),
        file.values_numeric(nc, "year"),
        file.values_numeric(nc, "doy"),
        file.values_f32(nc, "time"),
        file.values_f32(nc, "geo_cLat_deg"),
        file.values_f32(nc, "geo_lon_deg"),
        file.values_f32(nc, "R"),
    ) else {
        eprintln!("{label}: a grid coordinate is absent — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    };
    let fill = file.fill_value("jPar");
    let mut records = build_records(
        nrec, nobs, &npnt, &year, &doy, &hours, &clat, &lon, &radius, &jpar, fill,
    );
    if records.is_empty() {
        eprintln!("no records — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
    records.sort_by(|a, b| {
        a[3].total_cmp(&b[3])
            .then(a[0].total_cmp(&b[0]))
            .then(a[1].total_cmp(&b[1]))
            .then(a[4].total_cmp(&b[4]))
    });
    let bytes_out = write_bin(&records);
    if std::fs::write(out, &bytes_out).is_err() {
        eprintln!("write {out} returned void");
        std::process::exit(1);
    }
    let fac_min = records.iter().map(|r| r[4]).fold(f64::INFINITY, f64::min);
    let fac_max = records
        .iter()
        .map(|r| r[4])
        .fold(f64::NEG_INFINITY, f64::max);
    eprintln!(
        "{out}: {} records over {} grid rows ({} cells/row), jPar [{fac_min}, {fac_max}] uA/m2",
        records.len(),
        nrec,
        nobs,
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
    println!("format ampere_fac");
    println!("ttl {}", TTL as u64);
    println!("at earth");
    println!("cmap .");
    println!("lat geo_cLat_deg");
    println!("lon geo_lon_deg");
    println!(
        "field ampere_fac_uam2 ampere_fac_uam2 inverse-square electric uA/m2 {} 0.0 0.0",
        TTL as u64
    );
}

fn probe(path: &str) {
    let Ok(bytes) = std::fs::read(path) else {
        eprintln!("{path}: the file stays unread");
        std::process::exit(1);
    };
    let file = match NetcdfFile::parse(&bytes) {
        Ok(f) => f,
        Err(note) => {
            eprintln!("{path}: {note:?}");
            std::process::exit(1);
        }
    };
    eprintln!("{path}: {} dims, {} vars", file.dims.len(), file.vars.len());
    for v in &file.vars {
        let shape = match file.var_shape(v) {
            Ok(s) => s,
            Err(_) => continue,
        };
        eprintln!("  {} {:?} {:?}", v.name, v.nc_type.name(), shape);
    }
}

fn compile_file(path: &str, out: &str) {
    let Ok(bytes) = std::fs::read(path) else {
        eprintln!("{path}: the file stays unread");
        std::process::exit(1);
    };
    emit(&bytes, out, None);
}

fn compile_url(url: &str, out: &str) {
    let Some(entries) = remote_zip_entries(url) else {
        eprintln!("{url}: no zip central directory — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    };
    let Some(entry) = entries.iter().find(|e| e.name.ends_with(".grd.nc")) else {
        eprintln!("{url}: no .grd.nc member — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    };
    let Some(member) = remote_zip_extract(url, entry) else {
        eprintln!(
            "{url}: {} extract void — the bin stays unwritten (0 honored)",
            entry.name
        );
        std::process::exit(1);
    };
    emit(&member, out, Some(url));
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
    match arg_value(&args, "--url") {
        Some(url) => compile_url(&url, &out),
        None => match arg_value(&args, "--file").or_else(|| arg_value(&args, "--input")) {
            Some(path) => compile_file(&path, &out),
            None => compile_url(DEFAULT_URL, &out),
        },
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
            [80.0, 100.0, 6800.0, 1_672_531_200.0, 1.25],
            [75.0, 250.0, 6900.0, 1_672_531_320.0, -0.5],
        ];
        let bytes = write_bin(&records);
        assert_eq!(bytes.len(), 8 + records.len() * FIELDS * 8);
        assert_eq!(parse_bin(&bytes).as_deref(), Some(records.as_slice()));
    }

    #[test]
    fn parse_bin_rejects_foreign_magic_and_short_body() {
        assert!(parse_bin(b"AMPF").is_none());
        assert!(parse_bin(b"XXXX").is_none());
        let good = write_bin(&[[0.0; FIELDS]]);
        let short = good[..good.len() - 1].to_vec();
        assert!(parse_bin(&short).is_none());
    }

    #[test]
    fn grid_record_absent_is_not_a_record() {
        assert_eq!(
            grid_record(20.0, 100.0, 6800.0, 1_700_000_000.0, 1.0, None),
            Some([70.0, 100.0, 6800.0, 1_700_000_000.0, 1.0])
        );
        assert!(grid_record(f64::NAN, 100.0, 6800.0, 1.0, 1.0, None).is_none());
        assert!(grid_record(20.0, 100.0, 0.0, 1.0, 1.0, None).is_none());
        assert!(grid_record(20.0, 100.0, 6800.0, 1.0, f64::NAN, None).is_none());
        assert!(grid_record(20.0, 100.0, 6800.0, 1.0, 1.0, Some(1.0)).is_none());
    }

    #[test]
    fn unix_seconds_reads_the_civil_day() {
        assert_eq!(unix_seconds(2023.0, 1.0, 0.0), Some(1_672_531_200.0));
        assert_eq!(unix_seconds(2023.0, 1.0, 6.0), Some(1_672_552_800.0));
        assert!(unix_seconds(2023.0, 0.0, 0.0).is_none());
        assert!(unix_seconds(f64::NAN, 1.0, 0.0).is_none());
    }

    #[test]
    fn build_records_honors_npnt_per_grid_row() {
        let npnt = vec![2.0, 1.0];
        let year = vec![2023.0, 2023.0];
        let doy = vec![1.0, 1.0];
        let hours = vec![0.0f32, 6.0f32];
        let clat = vec![10.0f32, 20.0, 30.0, 40.0];
        let lon = vec![100.0f32, 101.0, 102.0, 103.0];
        let radius = vec![6800.0f32, 6800.0, 6800.0, 6800.0];
        let jpar = vec![1.0, 2.0, 3.0, 4.0];
        let records = build_records(
            2, 2, &npnt, &year, &doy, &hours, &clat, &lon, &radius, &jpar, None,
        );
        assert_eq!(records.len(), 3);
        assert_eq!(records[0], [80.0, 100.0, 6800.0, 1_672_531_200.0, 1.0]);
        assert_eq!(records[1], [70.0, 101.0, 6800.0, 1_672_531_200.0, 2.0]);
        assert_eq!(records[2], [60.0, 102.0, 6800.0, 1_672_552_800.0, 3.0]);
    }
}
