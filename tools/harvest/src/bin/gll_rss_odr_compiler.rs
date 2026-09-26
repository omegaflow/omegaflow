use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::upload_release;
use omegaflow::galileo_odr::{header, record_stride};
use omegaflow::odf::{PODF_SHARD_BUDGET, PODF_SHARD_LIMIT};

const BASE: &str =
    "https://pds-rings.seti.org/pds4/bundles/gll.rss/gll.rss.raw/data_rsc_11_11_odr/";
const NETLOC: &str = "pds-rings.seti.org";
const PREFIX: &str = "gll_rss_odr";
const FORMAT: &str = "galileo_odr";
const DIR: &str = "data/pds-rings.seti.org";
const MANIFEST: &str = "gll_rss_odr.manifest";
const SUFFIX: &str = "_odr.dat";
const MAX_DEPTH: u32 = 3;
const ENTRY: usize = 96;

struct FileBytes {
    name: String,
    bytes: Vec<u8>,
    record_count: u32,
    sample_rate: u16,
    sha256: [u8; 32],
}

struct FileMeta {
    name: String,
    sha256: [u8; 32],
    record_count: u32,
    sample_rate: u16,
    data_len: u64,
}

fn listing_names(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    for token in text.split("href=\"") {
        let Some(end) = token.find('"') else {
            continue;
        };
        out.push(token[..end].to_string());
    }
    out
}

fn crawl(url: &str, depth: u32, files: &mut Vec<String>) {
    if depth > MAX_DEPTH {
        return;
    }
    let Some(bytes) = fetch_raw_bytes(url) else {
        eprintln!("{url}: listing fetch void");
        return;
    };
    let Ok(text) = std::str::from_utf8(&bytes) else {
        eprintln!("{url}: listing not utf8");
        return;
    };
    for name in listing_names(text) {
        if name.starts_with('?') || name.starts_with('/') || name == "../" {
            continue;
        }
        if name.ends_with(SUFFIX) {
            files.push(format!("{url}{name}"));
            continue;
        }
        if name.ends_with('/') {
            crawl(&format!("{url}{name}"), depth + 1, files);
        }
    }
}

fn hex_nibble(c: u8) -> Option<u8> {
    match c {
        b'0'..=b'9' => Some(c - b'0'),
        b'a'..=b'f' => Some(c - b'a' + 10),
        _ => None,
    }
}

fn hex32(hex: &str) -> Option<[u8; 32]> {
    let b = hex.as_bytes();
    if b.len() != 64 {
        return None;
    }
    let mut out = [0u8; 32];
    for i in 0..32 {
        out[i] = (hex_nibble(b[i * 2])? << 4) | hex_nibble(b[i * 2 + 1])?;
    }
    Some(out)
}

fn hex_string(b: &[u8; 32]) -> String {
    let mut s = String::with_capacity(64);
    for v in b {
        s.push_str(&format!("{v:02x}"));
    }
    s
}

fn write_odr_bin(metas: &[FileMeta], data: &[u8]) -> Vec<u8> {
    let count = metas.len();
    let data_start = 8 + count * ENTRY;
    let mut bin = vec![0u8; data_start];
    bin[0..4].copy_from_slice(b"GODR");
    bin[4..8].copy_from_slice(&(count as u32).to_le_bytes());
    let mut offset = data_start as u64;
    for (i, f) in metas.iter().enumerate() {
        let base = 8 + i * ENTRY;
        let nameb = f.name.as_bytes();
        let n = nameb.len().min(32);
        bin[base..base + n].copy_from_slice(&nameb[..n]);
        bin[base + 32..base + 64].copy_from_slice(&f.sha256);
        bin[base + 64..base + 68].copy_from_slice(&f.record_count.to_le_bytes());
        bin[base + 68..base + 70].copy_from_slice(&f.sample_rate.to_le_bytes());
        bin[base + 72..base + 80].copy_from_slice(&offset.to_le_bytes());
        bin[base + 80..base + 88].copy_from_slice(&f.data_len.to_le_bytes());
        offset += f.data_len;
    }
    bin.extend_from_slice(data);
    bin
}

fn parse_odr_bin(data: &[u8]) -> Option<Vec<FileBytes>> {
    if data.len() < 8 || &data[0..4] != b"GODR" {
        return None;
    }
    let count = u32::from_le_bytes(data[4..8].try_into().ok()?) as usize;
    if data.len() < 8 + count * ENTRY {
        return None;
    }
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        let base = 8 + i * ENTRY;
        let name_end = data[base..base + 32]
            .iter()
            .position(|b| *b == 0)
            .unwrap_or(32);
        let name = String::from_utf8(data[base..base + name_end].to_vec()).ok()?;
        let mut sha256 = [0u8; 32];
        sha256.copy_from_slice(&data[base + 32..base + 64]);
        let record_count = u32::from_le_bytes(data[base + 64..base + 68].try_into().ok()?);
        let sample_rate = u16::from_le_bytes(data[base + 68..base + 70].try_into().ok()?);
        let data_offset = u64::from_le_bytes(data[base + 72..base + 80].try_into().ok()?) as usize;
        let data_length = u64::from_le_bytes(data[base + 80..base + 88].try_into().ok()?) as usize;
        if data_offset + data_length > data.len() {
            return None;
        }
        let bytes = data[data_offset..data_offset + data_length].to_vec();
        out.push(FileBytes {
            name,
            bytes,
            record_count,
            sample_rate,
            sha256,
        });
    }
    Some(out)
}

fn gate(bytes: &[u8], name: &str) -> Option<FileBytes> {
    let stride = record_stride(bytes)?;
    if bytes.len() < stride {
        return None;
    }
    let Some(h) = header(bytes) else {
        return None;
    };
    if h.sample_rate == 0 {
        return None;
    }
    let sha = hex32(&sha256_hex(bytes))?;
    Some(FileBytes {
        name: name.to_string(),
        bytes: bytes.to_vec(),
        record_count: (bytes.len() / stride) as u32,
        sample_rate: h.sample_rate,
        sha256: sha,
    })
}

fn roundtrip_holds(bin: &[u8]) -> bool {
    let Some(parsed) = parse_odr_bin(bin) else {
        return false;
    };
    parsed.iter().all(|f| {
        let Some(stride) = record_stride(&f.bytes) else {
            return false;
        };
        f.record_count as usize == f.bytes.len() / stride
            && sha256_hex(&f.bytes) == hex_string(&f.sha256)
    })
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let list_mode = args.iter().any(|a| a == "--list");
    let files_limit: usize = args
        .windows(2)
        .find(|w| w[0] == "--files")
        .and_then(|w| w[1].parse().ok())
        .unwrap_or(usize::MAX);
    let mut files: Vec<String> = Vec::new();
    crawl(BASE, 0, &mut files);
    files.sort();
    files.dedup();
    if files.is_empty() {
        eprintln!("{BASE}: no ODR files in listing tree — the series stays unwritten (0 honored)");
        return;
    }
    if list_mode {
        eprintln!("{BASE}: {} ODR files", files.len());
        for name in &files {
            eprintln!("  {name}");
        }
        return;
    }
    if std::fs::create_dir_all(DIR).is_err() {
        eprintln!("create {DIR} void");
        std::process::exit(1);
    }
    let mut seen: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    let mut metas: Vec<FileMeta> = Vec::new();
    let mut data: Vec<u8> = Vec::new();
    let mut names: Vec<String> = Vec::new();
    let mut shas: Vec<String> = Vec::new();
    let mut ord = 0usize;
    let mut flushed = 0usize;
    for url in files.iter().take(files_limit) {
        let name = url.rsplit('/').next().unwrap_or("odr").to_string();
        let Some(bytes) = fetch_raw_bytes(url) else {
            eprintln!("{name}: fetch void ({url})");
            continue;
        };
        let digest = sha256_hex(&bytes);
        if let Some(first) = seen.get(&digest) {
            eprintln!("{name}: sha256 {digest} — alias of {first} (counted once)");
            continue;
        }
        let Some(f) = gate(&bytes, &name) else {
            eprintln!("{name}: ODR gate void — {} B", bytes.len());
            continue;
        };
        seen.insert(digest, name.clone());
        if !metas.is_empty() && data.len() + f.bytes.len() > PODF_SHARD_BUDGET {
            if !flush_shard(ord, true, &metas, &data, ci_mode, &mut names, &mut shas) {
                std::process::exit(1);
            }
            ord += 1;
            flushed += 1;
            metas.clear();
            data.clear();
        }
        eprintln!(
            "{}: {} records, {} sps, {} bytes",
            f.name,
            f.record_count,
            f.sample_rate,
            f.bytes.len()
        );
        data.extend_from_slice(&f.bytes);
        metas.push(FileMeta {
            name: f.name,
            sha256: f.sha256,
            record_count: f.record_count,
            sample_rate: f.sample_rate,
            data_len: f.bytes.len() as u64,
        });
    }
    if metas.is_empty() {
        eprintln!("no ODR files gated — the series stays unwritten (0 honored)");
        return;
    }
    if !flush_shard(
        ord,
        flushed > 0,
        &metas,
        &data,
        ci_mode,
        &mut names,
        &mut shas,
    ) {
        std::process::exit(1);
    }
    let mut manifest = String::new();
    for (i, name) in names.iter().enumerate() {
        if let Some(sha) = shas.get(i) {
            manifest.push_str(&format!("{name} {sha}\n"));
        }
    }
    let manifest_path = format!("{DIR}/{MANIFEST}");
    if std::fs::write(&manifest_path, &manifest).is_err() {
        eprintln!("write {manifest_path} void");
        std::process::exit(1);
    }
    if ci_mode {
        if !upload_release(NETLOC, &manifest_path) {
            std::process::exit(1);
        }
        if std::fs::remove_file(&manifest_path).is_err() {
            eprintln!("{manifest_path}: local manifest stays (remove void)");
        }
    }
    if names.len() > 1 {
        println!(
            "url https://github.com/omegaflow/sources/releases/download/{NETLOC}/{PREFIX}.bin"
        );
        println!("format {FORMAT}");
        println!("origin procedure: {BASE} (Live-Listing subdirectories)");
        println!("compiler tools/harvest/src/bin/gll_rss_odr_compiler.rs");
        println!("at earth");
        println!("ttl 604800");
        println!("field ad1 galileo_odr_ad1_count inverse-square em count 604800 0.0 0.0");
        println!("field ad2 galileo_odr_ad2_count inverse-square em count 604800 0.0 0.0");
        println!("field ad3 galileo_odr_ad3_count inverse-square em count 604800 0.0 0.0");
        println!("field ad4 galileo_odr_ad4_count inverse-square em count 604800 0.0 0.0");
        println!();
    }
}

fn flush_shard(
    ord: usize,
    use_shard: bool,
    metas: &[FileMeta],
    data: &[u8],
    ci_mode: bool,
    names: &mut Vec<String>,
    shas: &mut Vec<String>,
) -> bool {
    let name = if use_shard {
        format!("{PREFIX}.bin.{ord:03}")
    } else {
        format!("{PREFIX}.bin")
    };
    let bin = write_odr_bin(metas, data);
    if bin.len() > PODF_SHARD_LIMIT {
        eprintln!(
            "{name}: {} bytes exceed the {PODF_SHARD_LIMIT}-byte CDN asset limit — the asset stays unwritten (0 honored)",
            bin.len()
        );
        return false;
    }
    if !roundtrip_holds(&bin) {
        eprintln!("{name}: roundtrip void — the asset stays unwritten (0 honored)");
        return false;
    }
    let path = format!("{DIR}/{name}");
    if std::fs::write(&path, &bin).is_err() {
        eprintln!("write {path} void");
        return false;
    }
    let group_bytes: usize = metas.iter().map(|f| f.data_len as usize).sum();
    eprintln!(
        "{name}: {} ODR files packaged ({} bytes), roundtrip holds",
        metas.len(),
        group_bytes
    );
    shas.push(sha256_hex(&bin));
    names.push(name.clone());
    if ci_mode {
        if !upload_release(NETLOC, &path) {
            return false;
        }
        if std::fs::remove_file(&path).is_err() {
            eprintln!("{path}: local shard stays (remove void)");
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_file(name: &str, nrec: usize) -> FileBytes {
        let mut bytes = vec![0u8; nrec * 2666];
        bytes[0] = 0xD2;
        bytes[4..6].copy_from_slice(&1333u16.to_be_bytes());
        bytes[158] = 0x04;
        bytes[159] = 0xE2;
        let sha256 = hex32(&sha256_hex(&bytes)).unwrap();
        FileBytes {
            name: name.to_string(),
            bytes,
            record_count: nrec as u32,
            sample_rate: 1250,
            sha256,
        }
    }

    fn pack(files: &[FileBytes]) -> Vec<u8> {
        let metas: Vec<FileMeta> = files
            .iter()
            .map(|f| FileMeta {
                name: f.name.clone(),
                sha256: f.sha256,
                record_count: f.record_count,
                sample_rate: f.sample_rate,
                data_len: f.bytes.len() as u64,
            })
            .collect();
        let mut data = Vec::new();
        for f in files {
            data.extend_from_slice(&f.bytes);
        }
        write_odr_bin(&metas, &data)
    }

    #[test]
    fn hex32_roundtrips_via_hex_string() {
        let h = sha256_hex(b"gll rss odr provenance");
        let raw = hex32(&h).unwrap();
        assert_eq!(hex_string(&raw), h);
        assert!(hex32("zz").is_none());
    }

    #[test]
    fn odr_bin_roundtrips() {
        let a = sample_file("63131033.ODR", 3);
        let b = sample_file("70571407.ODR", 2);
        let bin = pack(&[a, b]);
        let parsed = parse_odr_bin(&bin).unwrap();
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0].name, "63131033.ODR");
        assert_eq!(parsed[0].record_count, 3);
        assert_eq!(parsed[0].sample_rate, 1250);
        assert_eq!(sha256_hex(&parsed[0].bytes), hex_string(&parsed[0].sha256));
        assert!(parse_odr_bin(b"X").is_none());
    }

    #[test]
    fn roundtrip_holds_for_valid_bin() {
        let a = sample_file("63131033.ODR", 3);
        let bin = pack(&[a]);
        assert!(roundtrip_holds(&bin));
    }
}
