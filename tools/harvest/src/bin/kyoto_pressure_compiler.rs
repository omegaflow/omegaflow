use omegaflow::archivar::{embedded_lsk, fetch_raw_bytes};
use omegaflow::cdn::upload_release;
use omegaflow::geo::{COMP_KYOTO_PRESSURE, GeoRec, MAGIC_KYOTO, parse_bin, write_bin};
use omegaflow::inflate::inflate;
use omegaflow::lsk::days_from_civil;
use std::env;
use std::fs;

const NETLOC: &str = "zenodo.org";
const LIVE_ZIP: &str = "https://zenodo.org/records/8098323/files/data.zip";
const CDN_ZIP: &str = "https://github.com/omegaflow/sources/releases/download/zenodo.org/data.zip";
const KYOTO_LAT: f64 = 35.02938;
const KYOTO_LON: f64 = 135.78347;
const KYOTO_ALT: f64 = 60.82;
const JST_OFFSET_S: f64 = 32400.0;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn le16(b: &[u8]) -> u16 {
    u16::from_le_bytes([b[0], b[1]])
}

fn le32(b: &[u8]) -> u32 {
    u32::from_le_bytes([b[0], b[1], b[2], b[3]])
}

struct ZipEntry {
    name: String,
    method: u16,
    comp_size: usize,
    lho: usize,
}

fn zip_entries(data: &[u8]) -> Option<Vec<ZipEntry>> {
    let mut eocd: Option<usize> = None;
    let lo = data.len().saturating_sub(22 + 65535);
    for i in (lo..data.len()).rev() {
        if i + 4 <= data.len() && data[i..i + 4] == *b"PK\x05\x06" {
            eocd = Some(i);
            break;
        }
    }
    let eo = eocd?;
    let cd_size = le32(data.get(eo + 12..eo + 16)?) as usize;
    let cd_off = le32(data.get(eo + 16..eo + 20)?) as usize;
    let cd = data.get(cd_off..cd_off + cd_size)?;
    let mut entries = Vec::new();
    let mut p = 0usize;
    while p + 46 <= cd.len() {
        if cd[p..p + 4] != *b"PK\x01\x02" {
            break;
        }
        let method = le16(&cd[p + 10..p + 12]);
        let comp_size = le32(&cd[p + 20..p + 24]) as usize;
        let nlen = le16(&cd[p + 28..p + 30]) as usize;
        let xlen = le16(&cd[p + 30..p + 32]) as usize;
        let clen = le16(&cd[p + 32..p + 34]) as usize;
        let lho = le32(&cd[p + 42..p + 46]) as usize;
        let name = String::from_utf8_lossy(&cd[p + 46..p + 46 + nlen]).into_owned();
        entries.push(ZipEntry {
            name,
            method,
            comp_size,
            lho,
        });
        p += 46 + nlen + xlen + clen;
    }
    Some(entries)
}

fn entry_bytes(data: &[u8], e: &ZipEntry) -> Option<Vec<u8>> {
    let lh = data.get(e.lho..e.lho + 30)?;
    let nlen = le16(&lh[26..28]) as usize;
    let xlen = le16(&lh[28..30]) as usize;
    let start = e.lho + 30 + nlen + xlen;
    let raw = data.get(start..start + e.comp_size)?;
    if e.method == 0 {
        Some(raw.to_vec())
    } else {
        inflate(raw)
    }
}

fn parse_pressure(text: &str) -> Vec<(f64, f64)> {
    let mut samples = Vec::new();
    for line in text.lines() {
        let cols: Vec<&str> = line.split_whitespace().collect();
        if cols.len() < 8 {
            continue;
        }
        let (Ok(year), Ok(month), Ok(day), Ok(hour), Ok(minute), Ok(second), Ok(hpa)) = (
            cols[0].parse::<i64>(),
            cols[1].parse::<i64>(),
            cols[2].parse::<i64>(),
            cols[3].parse::<i64>(),
            cols[4].parse::<i64>(),
            cols[5].parse::<f64>(),
            cols[7].parse::<f64>(),
        ) else {
            continue;
        };
        let Some(days) = days_from_civil(year, month, day) else {
            continue;
        };
        let unix = days as f64 * 86400.0 + hour as f64 * 3600.0 + minute as f64 * 60.0 + second
            - JST_OFFSET_S;
        samples.push((unix, hpa));
    }
    samples
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let out_bin = match arg_value(&args, "--out-bin") {
        Some(v) => v,
        None => "data/zenodo.org/kyoto_pressure.bin".to_string(),
    };
    let ci_mode = args.iter().any(|a| a == "--ci-mode");

    let archive = match arg_value(&args, "--url") {
        Some(u) => fetch_raw_bytes(&u),
        None => fetch_raw_bytes(CDN_ZIP).or_else(|| fetch_raw_bytes(LIVE_ZIP)),
    };
    let Some(archive) = archive else {
        eprintln!(
            "kyoto pressure: the Zenodo data.zip stayed unreadable (CDN mirror + live both void) — pending"
        );
        std::process::exit(1);
    };

    let Some(entries) = zip_entries(&archive) else {
        eprintln!(
            "kyoto pressure: the Zenodo data.zip carries no readable central directory — pending"
        );
        std::process::exit(1);
    };
    let members: Vec<&ZipEntry> = entries
        .iter()
        .filter(|e| e.name.starts_with("data/") && e.name.ends_with(".txt"))
        .collect();
    if members.is_empty() {
        eprintln!("kyoto pressure: no data/22MMDD.txt members in the archive (0 honored)");
        std::process::exit(1);
    }

    let Some(lsk) = embedded_lsk() else {
        eprintln!(
            "kyoto pressure: naif0012 table void — the TDB epoch stays void (no fabricated epoch)"
        );
        std::process::exit(1);
    };

    let mut records: Vec<GeoRec> = Vec::new();
    for e in &members {
        let Some(bytes) = entry_bytes(&archive, e) else {
            eprintln!("kyoto pressure: {} stays unreadable — pending", e.name);
            continue;
        };
        let text = String::from_utf8_lossy(&bytes);
        for (unix, hpa) in parse_pressure(&text) {
            if !hpa.is_finite() || hpa <= 0.0 {
                continue;
            }
            let Some(t) = lsk.unix_to_tdb(unix) else {
                continue;
            };
            records.push(GeoRec {
                t,
                lat: KYOTO_LAT,
                lon: KYOTO_LON,
                alt: KYOTO_ALT,
                freq: 0.0,
                bin_width: 0.0,
                val: hpa,
                comp: COMP_KYOTO_PRESSURE,
                station: 0,
            });
        }
    }
    if records.is_empty() {
        eprintln!(
            "kyoto pressure: no pressure rows from {} members — the bin stays unwritten (0 honored)",
            members.len()
        );
        std::process::exit(1);
    }
    records.sort_by(|a, b| a.t.total_cmp(&b.t));

    let bytes = write_bin(MAGIC_KYOTO, &records);
    if let Some(parent) = std::path::Path::new(&out_bin).parent() {
        if !parent.as_os_str().is_empty() {
            let _ = std::fs::create_dir_all(parent);
        }
    }
    if fs::write(&out_bin, &bytes).is_err() {
        eprintln!("write {} returned void", out_bin);
        std::process::exit(1);
    }
    match parse_bin(MAGIC_KYOTO, &bytes) {
        Some(parsed) => eprintln!(
            "{}: {} surface-pressure records, {} B, roundtrip parses",
            out_bin,
            parsed.len(),
            bytes.len()
        ),
        None => {
            eprintln!("{}: roundtrip parse void", out_bin);
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_release(NETLOC, &out_bin) {
        std::process::exit(1);
    }
}
