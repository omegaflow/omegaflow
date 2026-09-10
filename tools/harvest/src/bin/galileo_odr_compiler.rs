use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::upload_release;

const BASE: &str = "https://pds-ppi.igpp.ucla.edu/annex/";
const GOJ: &str = "GO-J-RSS-1-ODR-V1.0";
const GOJS: &str = "GO-JS-RSS-1-ODR-V1.0";
const REC: usize = 2666;
const ENTRY: usize = 96;

struct Source {
    name: &'static str,
    volume: &'static str,
    sha256: &'static str,
}

const SOURCES: &[Source] = &[
    Source {
        name: "63131033.ODR",
        volume: GOJ,
        sha256: "4f976d45f5184a3394e7ec863efdf48b1f0747538a8bdec663aa6723318e7aa1",
    },
    Source {
        name: "63131742.ODR",
        volume: GOJ,
        sha256: "1edb63f1d5b39b7a5a80982742d382bfbcd6413d8e549eebd9ab4f7e458ad975",
    },
    Source {
        name: "63561707.ODR",
        volume: GOJ,
        sha256: "72904a159e53265d7c1db7e8c196ceb5066ce89262c8aea0f4508843ca7f91eb",
    },
    Source {
        name: "63570045.ODR",
        volume: GOJ,
        sha256: "7aad817520a1f83416925b3ae4c7712b02d7bec2148602da43d7a3952a793857",
    },
    Source {
        name: "70571807.ODR",
        volume: GOJ,
        sha256: "8b91928291df989c2ab7d48cd202d43d955710c5c1911b40ff6ca2e526e34433",
    },
    Source {
        name: "70571825.ODR",
        volume: GOJ,
        sha256: "fd319f30898dcffe0d2e61270cf298bb9adba6f4b695bf1928ea66703a64255c",
    },
    Source {
        name: "70580900.ODR",
        volume: GOJ,
        sha256: "362d3d00d614f5c5e748d870f5a69846ed03b3e32df046976d5d8f8f08150dcd",
    },
    Source {
        name: "JS_63540659.ODR",
        volume: GOJS,
        sha256: "aed589e47d1078296ea4f36373fb577608320722516917aa56713c513ef330c7",
    },
    Source {
        name: "JS_70561433.ODR",
        volume: GOJS,
        sha256: "a16fa7b38d060f4b0dc53327bbf1824ab2a4cef198f09e4f57dc78d5b21772a9",
    },
    Source {
        name: "JS_70571407.ODR",
        volume: GOJS,
        sha256: "d35f50efeada76741335c784d0f9ab417be23e91f95a9bd5959f3ccbdab531be",
    },
];

struct FileBytes {
    name: String,
    bytes: Vec<u8>,
    record_count: u32,
    sample_rate: u16,
    sha256: [u8; 32],
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
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

fn sample_rate(bytes: &[u8]) -> Option<u16> {
    if bytes.len() < REC {
        return None;
    }
    Some((bytes[158] as u16) << 8 | bytes[159] as u16)
}

fn write_odr_bin(files: &[FileBytes]) -> Vec<u8> {
    let count = files.len();
    let data_start = 8 + count * ENTRY;
    let mut bin = vec![0u8; data_start];
    bin[0..4].copy_from_slice(b"GODR");
    bin[4..8].copy_from_slice(&(count as u32).to_le_bytes());
    let mut offset = data_start as u64;
    for (i, f) in files.iter().enumerate() {
        let base = 8 + i * ENTRY;
        let nameb = f.name.as_bytes();
        let n = nameb.len().min(32);
        bin[base..base + n].copy_from_slice(&nameb[..n]);
        bin[base + 32..base + 64].copy_from_slice(&f.sha256);
        bin[base + 64..base + 68].copy_from_slice(&f.record_count.to_le_bytes());
        bin[base + 68..base + 70].copy_from_slice(&f.sample_rate.to_le_bytes());
        bin[base + 72..base + 80].copy_from_slice(&offset.to_le_bytes());
        bin[base + 80..base + 88].copy_from_slice(&(f.bytes.len() as u64).to_le_bytes());
        offset += f.bytes.len() as u64;
    }
    for f in files {
        bin.extend_from_slice(&f.bytes);
    }
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

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = match arg_value(&args, "--out") {
        Some(path) => path,
        None => "data/pds-ppi.igpp.ucla.edu/galileo_odr.bin".to_string(),
    };
    let dir = arg_value(&args, "--dir");

    let mut files: Vec<FileBytes> = Vec::new();
    for src in SOURCES {
        let bytes = match &dir {
            Some(d) => std::fs::read(format!("{d}/{}", src.name)).ok(),
            None => fetch_raw_bytes(&format!("{BASE}{}/ODR/{}", src.volume, src.name), 604800),
        };
        let Some(bytes) = bytes else {
            eprintln!("{}: {} read void", src.name, src.volume);
            continue;
        };
        if bytes.len() < REC {
            eprintln!(
                "{}: {} bytes — shorter than one ODR record",
                src.name,
                bytes.len()
            );
            continue;
        }
        let hex = sha256_hex(&bytes);
        if hex != src.sha256 {
            eprintln!("{}: sha256 {} — provenance mismatch", src.name, hex);
            continue;
        }
        let Some(sr) = sample_rate(&bytes) else {
            eprintln!("{}: sample-rate field void", src.name);
            continue;
        };
        let record_count = (bytes.len() / REC) as u32;
        let trailing = bytes.len() % REC;
        let Some(sha256) = hex32(src.sha256) else {
            eprintln!("{}: provenance hex void", src.name);
            continue;
        };
        if trailing == 0 {
            eprintln!(
                "{}: {} records, {} sps, {} bytes, provenance holds",
                src.name,
                record_count,
                sr,
                bytes.len()
            );
        } else {
            eprintln!(
                "{}: {} records + {} trailing bytes, {} sps, {} bytes, provenance holds",
                src.name,
                record_count,
                trailing,
                sr,
                bytes.len()
            );
        }
        files.push(FileBytes {
            name: src.name.to_string(),
            bytes,
            record_count,
            sample_rate: sr,
            sha256,
        });
    }
    if files.is_empty() {
        eprintln!("no galileo ODR files passed the provenance gate — the asset stays unwritten (0 honored)");
        return;
    }
    if let Some(p) = std::path::Path::new(&out).parent() {
        let _ = std::fs::create_dir_all(p);
    }
    let bin = write_odr_bin(&files);
    if std::fs::write(&out, &bin).is_err() {
        eprintln!("write {out} void");
        return;
    }
    match parse_odr_bin(&bin) {
        Some(parsed) => {
            let mut all_hold = true;
            let mut total_bytes = 0usize;
            for f in &parsed {
                total_bytes += f.bytes.len();
                if f.record_count as usize != f.bytes.len() / REC {
                    all_hold = false;
                }
                if sha256_hex(&f.bytes) != hex_string(&f.sha256) {
                    all_hold = false;
                }
            }
            eprintln!(
                "{out}: {} ODR files packaged ({} bytes), roundtrip parses, fidelity {}",
                parsed.len(),
                total_bytes,
                if all_hold { "holds" } else { "void" }
            );
        }
        None => {
            eprintln!("{out}: roundtrip parse void — the asset stays unverified");
        }
    }
    if ci_mode && !upload_release("pds-ppi.igpp.ucla.edu", &out) {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_file(name: &str, nrec: usize) -> FileBytes {
        let mut bytes = vec![0u8; nrec * REC];
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

    #[test]
    fn hex32_roundtrips_via_hex_string() {
        let h = sha256_hex(b"galileo ODR provenance");
        let raw = hex32(&h).unwrap();
        assert_eq!(hex_string(&raw), h);
        assert!(hex32("zz").is_none());
    }

    #[test]
    fn sample_rate_reads_big_endian_word() {
        let bytes = sample_file("x.ODR", 1).bytes;
        assert_eq!(sample_rate(&bytes), Some(1250));
        assert_eq!(sample_rate(&[0u8; 100]), None);
    }

    #[test]
    fn odr_bin_roundtrips() {
        let a = sample_file("63131033.ODR", 3);
        let b = sample_file("JS_70571407.ODR", 5);
        let bin = write_odr_bin(&[a, b]);
        let parsed = parse_odr_bin(&bin).unwrap();
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0].name, "63131033.ODR");
        assert_eq!(parsed[0].record_count, 3);
        assert_eq!(parsed[0].sample_rate, 1250);
        assert_eq!(parsed[0].bytes.len(), 3 * REC);
        assert_eq!(sha256_hex(&parsed[0].bytes), hex_string(&parsed[0].sha256));
        assert_eq!(parsed[1].name, "JS_70571407.ODR");
        assert_eq!(parsed[1].bytes.len(), 5 * REC);
        assert!(parse_odr_bin(b"X").is_none());
    }
}
