use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::mariner4_odg;
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::upload_release;
use omegaflow::inflate::{gunzip, tar_members};
use omegaflow::lsk::days_from_civil;

const NETLOC: &str = "spdf.gsfc.nasa.gov";
const FORMAT: &str = "mariner4_odg";
const URLS: &[&str] = &[
    "https://spdf.gsfc.nasa.gov/pub/data/mariner/mariner4/celestial_mechanics/PSCM-00021_DD001528_05-DEC-64.tar",
    "https://spdf.gsfc.nasa.gov/pub/data/mariner/mariner4/celestial_mechanics/PSCM-00021_DD001529_28-NOV-64.tar",
];

const HEADER_RECORDS: usize = 3;
const WORDS_PER_DATA_RECORD: usize = 200;
const LOGICAL_RECORD_WORDS: usize = 8;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn words_of(payload: &[u8]) -> Vec<u64> {
    let mut chars: Vec<u64> = Vec::with_capacity(payload.len() / 3 * 4);
    for group in payload.chunks_exact(3) {
        let value = ((group[0] as u64) << 16) | ((group[1] as u64) << 8) | group[2] as u64;
        chars.push((value >> 18) & 0x3F);
        chars.push((value >> 12) & 0x3F);
        chars.push((value >> 6) & 0x3F);
        chars.push(value & 0x3F);
    }
    chars
        .chunks_exact(6)
        .map(|c| c.iter().fold(0u64, |w, &ch| (w << 6) | ch))
        .collect()
}

fn unix_from_words(w1: u64, w2: u64) -> Option<f64> {
    let v: u128 = ((w1 as u128) << 36) | (w2 as u128);
    let exp = ((v >> 63) & 0xFF) as i32;
    let frac = (v & ((1u128 << 63) - 1)) as f64;
    let sign = if (v >> 71) & 1 == 1 { -1.0 } else { 1.0 };
    let seconds = sign * (frac / (2f64.powi(63))) * 2f64.powi(exp - 128);
    if !seconds.is_finite() {
        return None;
    }
    let epoch1950 = (days_from_civil(1950, 1, 1)? as f64) * 86400.0;
    Some(epoch1950 + seconds)
}

fn hex_single(w: u64) -> f64 {
    let sign = if (w >> 35) & 1 == 1 { -1.0 } else { 1.0 };
    let exp = ((w >> 28) & 0x7F) as i32;
    let frac = (w & 0x0FFF_FFFF) as f64 / (2f64.powi(28));
    sign * frac * 16f64.powi(exp - 64)
}

fn datatype_of(code: u64) -> Option<u32> {
    let mut found = None;
    for i in 1..=11u32 {
        if (code >> (35 - i)) & 1 == 1 {
            if found.is_some() {
                return None;
            }
            found = Some(i);
        }
    }
    found
}

fn parse_tar(raw: &[u8], rows: &mut Vec<(f64, f64, u32)>, skipped: &mut usize) {
    let owned;
    let dats: Vec<&[u8]> = if let Some(members) = tar_members(raw) {
        members
            .iter()
            .filter(|m| m.name.ends_with("_F1.DAT"))
            .filter_map(|m| raw.get(m.start..m.end))
            .collect()
    } else if let Some(t) = gunzip(raw) {
        owned = t;
        match tar_members(&owned) {
            Some(members) => members
                .iter()
                .filter(|m| m.name.ends_with("_F1.DAT"))
                .filter_map(|m| owned.get(m.start..m.end))
                .collect(),
            None => Vec::new(),
        }
    } else {
        vec![raw]
    };

    for data in dats {
        let mut off = 0usize;
        let mut index = 0usize;
        while off + 2 <= data.len() {
            let len = u16::from_be_bytes([data[off], data[off + 1]]) as usize;
            let payload = off + 2;
            let end = payload + len;
            if len == 0 || end > data.len() {
                break;
            }
            if index >= HEADER_RECORDS {
                let words = words_of(&data[payload..end]);
                for base in (0..WORDS_PER_DATA_RECORD).step_by(LOGICAL_RECORD_WORDS) {
                    let Some(code) = words.get(base + 5).copied() else {
                        break;
                    };
                    let Some(dtype) = datatype_of(code) else {
                        *skipped += 1;
                        continue;
                    };
                    let (Some(&w1), Some(&w2)) = (words.get(base + 1), words.get(base + 2)) else {
                        break;
                    };
                    let Some(t) = unix_from_words(w1, w2) else {
                        *skipped += 1;
                        continue;
                    };
                    if !(-252_460_800.0..=189_345_600.0).contains(&t) {
                        *skipped += 1;
                        continue;
                    }
                    let Some(&obs) = words.get(base + 6) else {
                        break;
                    };
                    rows.push((t, hex_single(obs), dtype));
                }
            }
            off = end;
            index += 1;
        }
    }
}

fn run(args: &[String]) -> Result<(), String> {
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = match arg_value(args, "--out-bin") {
        Some(v) => v,
        None => "mariner4_odg.bin".to_string(),
    };

    let raws: Vec<Vec<u8>> = match arg_value(args, "--in") {
        Some(path) => vec![std::fs::read(&path).map_err(|e| format!("read {path} void: {e}"))?],
        None => {
            let mut all = Vec::new();
            for url in URLS {
                all.push(fetch_raw_bytes(url).ok_or_else(|| format!("{url}: fetch void"))?);
            }
            all
        }
    };

    let mut rows: Vec<(f64, f64, u32)> = Vec::new();
    let mut skipped = 0usize;
    for raw in &raws {
        parse_tar(raw, &mut rows, &mut skipped);
    }
    if rows.is_empty() {
        return Err(format!(
            "no observables parsed ({skipped} records without a measured code word) — the bin stays unwritten (0 honored)"
        ));
    }
    let bin = mariner4_odg::write_bin(&rows);
    if mariner4_odg::parse_bin(&bin).is_none() {
        return Err(format!("{out}: roundtrip parse void"));
    }
    std::fs::write(&out, &bin).map_err(|e| format!("write {out} void: {e}"))?;
    let t_min = rows.iter().map(|r| r.0).fold(f64::INFINITY, f64::min);
    let t_max = rows.iter().map(|r| r.0).fold(f64::NEG_INFINITY, f64::max);
    eprintln!(
        "{out}: {} observables, {} records skipped, t [{t_min:.0}..{t_max:.0}] unix, format {FORMAT}, sha256 {}",
        rows.len(),
        skipped,
        sha256_hex(&bin)
    );
    if ci_mode && !upload_release(NETLOC, &out) {
        return Err(format!("{out}: CDN upload returned void"));
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--help" || a == "-h") {
        eprintln!(
            "usage: mariner4_odg_compiler [--in <F1.DAT|tar>] [--out-bin <name>] [--ci-mode]"
        );
        std::process::exit(2);
    }
    if let Err(msg) = run(&args) {
        eprintln!("mariner4_odg_compiler: {msg}");
        std::process::exit(1);
    }
}
