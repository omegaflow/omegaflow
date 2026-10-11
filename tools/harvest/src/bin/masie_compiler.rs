use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::archivar::{PRESENCE_FLAG_QUANTITY, embedded_lsk};
use omegaflow::cdn::upload_release;
use omegaflow::lsk::days_from_civil;
use std::path::Path;

const NETLOC: &str = "noaadata.apps.nsidc.org";
const DEFAULT_URL: &str =
    "https://noaadata.apps.nsidc.org/NOAA/G02186/masie_4km_allyears_extent_sqkm.csv";
const OUT_PATH: &str = "data/masie_ice_extent.bin";
const COMPILER: &str = "tools/harvest/src/bin/masie_compiler.rs";
const FORMAT: &str = "masie_ice_extent";
const TTL: f64 = 86400.0;
const REC_BYTES: usize = 26 * 8;
const FRAME_HEADER: usize = 19;
const KM2_TO_M2: f64 = 1.0e6;

const SLOT_VAL: usize = 3;
const SLOT_EPOCH: usize = 4;
const SLOT_TTL: usize = 5;
const SLOT_CHANNEL: usize = 9;
const SLOT_PRESENCE: usize = 25;

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

fn parse_yyyyddd(cell: &str) -> Option<(i64, i64)> {
    let t = cell.trim();
    if t.len() != 7 || !t.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    let year: i64 = t.get(0..4)?.parse().ok()?;
    let yday: i64 = t.get(4..7)?.parse().ok()?;
    if !(1..=366).contains(&yday) {
        return None;
    }
    Some((year, yday))
}

fn day_unix(year: i64, yday: i64) -> Option<f64> {
    let base = days_from_civil(year, 1, 1)?;
    Some(((base + yday - 1) * 86400) as f64)
}

fn parse_cell(cell: &str) -> Option<f64> {
    let t = cell.trim();
    if t.is_empty() {
        return None;
    }
    t.parse::<f64>().ok().filter(|v| v.is_finite())
}

fn region_name(cell: &str) -> String {
    let c = cell.trim();
    match c.find(')') {
        Some(i) => c[i + 1..].trim().to_string(),
        None => c.to_string(),
    }
}

fn slug(name: &str) -> String {
    name.chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() {
                ch.to_ascii_lowercase()
            } else {
                '_'
            }
        })
        .collect()
}

struct MasieRow {
    epoch_unix: f64,
    values: Vec<Option<f64>>,
}

fn parse(text: &str) -> Option<(Vec<String>, Vec<MasieRow>)> {
    let mut names: Option<Vec<String>> = None;
    let mut rows = Vec::new();
    for line in text.lines() {
        let line = line.trim_end();
        if line.trim().is_empty() {
            continue;
        }
        if names.is_none() {
            if line.trim_start().starts_with("yyyyddd") {
                let parsed: Vec<String> = line.split(',').skip(1).map(region_name).collect();
                if !parsed.is_empty() {
                    names = Some(parsed);
                }
            }
            continue;
        }
        let cells: Vec<&str> = line.split(',').collect();
        if cells.len() < 2 {
            continue;
        }
        let Some((year, yday)) = parse_yyyyddd(cells[0]) else {
            continue;
        };
        let Some(epoch_unix) = day_unix(year, yday) else {
            continue;
        };
        let values: Vec<Option<f64>> = cells[1..].iter().map(|c| parse_cell(c)).collect();
        if values.iter().any(|v| v.is_some()) {
            rows.push(MasieRow { epoch_unix, values });
        }
    }
    let names = names?;
    Some((names, rows))
}

fn record(epoch_tdb: f64, region: usize, extent_m2: f64) -> [f64; 26] {
    let mut r = [0.0f64; 26];
    r[SLOT_VAL] = extent_m2;
    r[SLOT_EPOCH] = epoch_tdb;
    r[SLOT_TTL] = TTL;
    r[SLOT_CHANNEL] = (region + 1) as f64;
    r[SLOT_PRESENCE] = PRESENCE_FLAG_QUANTITY;
    r
}

fn frame(records: &[[f64; 26]]) -> Option<Vec<u8>> {
    if records.is_empty() {
        return None;
    }
    let response_epoch = records
        .iter()
        .map(|r| r[SLOT_EPOCH])
        .fold(f64::NEG_INFINITY, f64::max);
    let mut out = Vec::with_capacity(FRAME_HEADER + records.len() * REC_BYTES);
    out.extend_from_slice(&[0xCF, 0x86, 0x09]);
    out.extend_from_slice(&response_epoch.to_le_bytes());
    out.extend_from_slice(&0u32.to_le_bytes());
    out.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for r in records {
        for v in r {
            out.extend_from_slice(&v.to_le_bytes());
        }
    }
    Some(out)
}

fn unframe(bytes: &[u8]) -> Option<Vec<[f64; 26]>> {
    if bytes.len() < FRAME_HEADER || bytes.get(0..3)? != [0xCF, 0x86, 0x09].as_slice() {
        return None;
    }
    let n = u32::from_le_bytes(bytes.get(15..19)?.try_into().ok()?) as usize;
    if bytes.len() != FRAME_HEADER + n * REC_BYTES {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    let mut off = FRAME_HEADER;
    for _ in 0..n {
        let mut r = [0.0f64; 26];
        for slot in r.iter_mut() {
            *slot = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
            off += 8;
        }
        out.push(r);
    }
    Some(out)
}

fn compile(text: &str, names_in: &mut Option<Vec<String>>) -> Vec<MasieRow> {
    let Some((names, rows)) = parse(text) else {
        return Vec::new();
    };
    match names_in {
        Some(existing) if *existing != names => {
            eprintln!(
                "masie: region header differs from the first file — the block stays unwritten (riss)"
            );
            std::process::exit(1);
        }
        Some(_) => {}
        None => *names_in = Some(names),
    }
    rows
}

fn run(args: &[String]) -> Result<(), String> {
    let lsk = embedded_lsk().ok_or("the embedded naif0012 table stays unread")?;
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = match arg_value(args, "--out") {
        Some(v) => v,
        None => OUT_PATH.to_string(),
    };
    let inputs = arg_values(args, "--input");

    let mut names: Option<Vec<String>> = None;
    let mut rows: Vec<MasieRow> = Vec::new();
    let mut origin = String::new();

    if inputs.is_empty() {
        let url = match arg_value(args, "--url") {
            Some(v) => v,
            None => DEFAULT_URL.to_string(),
        };
        let bytes = fetch_raw_bytes(&url).ok_or_else(|| format!("{url}: fetch returned void"))?;
        let text = String::from_utf8_lossy(&bytes).into_owned();
        origin = url;
        rows.extend(compile(&text, &mut names));
    } else {
        for path in &inputs {
            let text = std::fs::read_to_string(path)
                .map_err(|e| format!("read {path} returned void: {e}"))?;
            origin = path.clone();
            rows.extend(compile(&text, &mut names));
        }
    }

    let names = names.ok_or_else(|| {
        "no MASIE header line (yyyyddd…) found — the asset stays unwritten".to_string()
    })?;

    let mut records: Vec<[f64; 26]> = Vec::new();
    let mut skipped = 0usize;
    for row in &rows {
        for (region, value) in row.values.iter().enumerate() {
            let Some(v) = value else {
                skipped += 1;
                continue;
            };
            let Some(epoch_tdb) = lsk.unix_to_tdb(row.epoch_unix) else {
                skipped += 1;
                continue;
            };
            records.push(record(epoch_tdb, region, v * KM2_TO_M2));
        }
    }

    if records.is_empty() {
        return Err("no measured extent — the asset stays unwritten (0 honored)".to_string());
    }

    let bytes = frame(&records).ok_or("framing returned void")?;
    if let Some(parent) = Path::new(&out).parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("create parent returned void: {e}"))?;
        }
    }
    std::fs::write(&out, &bytes).map_err(|e| format!("write {out} returned void: {e}"))?;

    match unframe(&bytes) {
        Some(parsed) if parsed == records => {
            eprintln!("  roundtrip: {} records parse, identical", parsed.len());
        }
        Some(parsed) => {
            return Err(format!(
                "{out}: {} rows read back, {} written — the asset stays unverified",
                parsed.len(),
                records.len()
            ));
        }
        None => return Err(format!("{out}: roundtrip parse returned void")),
    }

    let sha = sha256_hex(&bytes);
    eprintln!(
        "{out}: {} records ({} absent cells skipped), {} regions, {} bytes, sha256 {}",
        records.len(),
        skipped,
        names.len(),
        bytes.len(),
        sha
    );

    let out_name = match Path::new(&out).file_name() {
        Some(n) => n.to_string_lossy().into_owned(),
        None => out.clone(),
    };
    println!("url https://github.com/omegaflow/sources/releases/download/{NETLOC}/{out_name}");
    println!("origin {origin}");
    println!("compiler {COMPILER}");
    println!("sha256 {sha}");
    println!("format {FORMAT}");
    println!("ttl {}", TTL as u64);
    println!("at earth");
    for name in &names {
        let s = slug(name);
        println!(
            "quantity masie_ice_extent_{s} masie_ice_extent_{s} inverse-square area m2 86400 0.0 0.0"
        );
    }

    if ci_mode && !upload_release(NETLOC, &out) {
        return Err(format!("{out}: CDN upload returned void"));
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("masie_compiler: {msg}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROBE: &str = "MASIE NSIDC/NIC Sea Ice Product G02186 - Daily Ice Extent by Region in Square Kilometers\n\
yyyyddd, (0) Northern_Hemisphere, (1) Beaufort_Sea, (2) Chukchi_Sea\n\
 2006001,13034723.95, 1069710.81,  966006.16\n\
 2006002,13034723.95,            ,  966006.16\n\
 2006003,       0.00,     11408.53,    6420.96\n";

    #[test]
    fn parse_reads_the_region_header_and_cells() {
        let (names, rows) = parse(PROBE).expect("the probe parses");
        assert_eq!(
            names,
            vec!["Northern_Hemisphere", "Beaufort_Sea", "Chukchi_Sea"]
        );
        assert_eq!(rows.len(), 3);
        assert_eq!(rows[0].values[0], Some(13034723.95));
        assert_eq!(rows[0].values[1], Some(1069710.81));
    }

    #[test]
    fn an_absent_cell_is_skipped_never_a_fabricated_zero() {
        let (_, rows) = parse(PROBE).expect("the probe parses");
        assert_eq!(rows[1].values[1], None);
    }

    #[test]
    fn a_measured_zero_is_kept() {
        let (_, rows) = parse(PROBE).expect("the probe parses");
        assert_eq!(rows[2].values[0], Some(0.0));
    }

    #[test]
    fn epoch_reads_from_yyyyddd() {
        let (year, yday) = parse_yyyyddd(" 2006001").expect("yyyyddd parses");
        assert_eq!((year, yday), (2006, 1));
        assert_eq!(day_unix(2006, 1), Some(1136073600.0));
        assert_eq!(parse_yyyyddd("2006000"), None);
        assert_eq!(parse_yyyyddd("2006367"), None);
    }

    #[test]
    fn the_frame_carries_the_wire_magic_and_roundtrips() {
        let records = vec![
            record(1136073600.0, 0, 1.3034e13),
            record(1136073600.0, 1, 1.0697e12),
        ];
        let bytes = frame(&records).expect("non-empty frames");
        assert_eq!(&bytes[0..3], [0xCF, 0x86, 0x09].as_slice());
        assert_eq!(bytes.len(), FRAME_HEADER + 2 * REC_BYTES);
        assert_eq!(unframe(&bytes), Some(records));
        assert!(frame(&[]).is_none());
    }

    #[test]
    fn a_foreign_or_truncated_frame_is_void() {
        assert!(unframe(b"XXXX").is_none());
        let good = frame(&[record(1.0, 0, 1.0)]).expect("frames");
        assert!(unframe(&good[..good.len() - 1]).is_none());
    }
}
