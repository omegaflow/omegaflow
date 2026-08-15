// Zip-Range-Extraktor: liest ein Zip über HTTP-Range (EOCD → Central Directory →
// Member-Payload), ohne den Zip-Körper zu laden. Für grosse Zips (dastcom5.zip ~1,3 GB).
// usage:
//   zip_range_extract --url <zip-url> [--tail <bytes>] --list <substr>
//   zip_range_extract --url <zip-url> [--tail <bytes>] --get <substr> --out <file>

use omegaflow::inflate::inflate;
use std::io::Write;
use std::process::Command;

const EOCD_SIG: [u8; 4] = [0x50, 0x4b, 0x05, 0x06];
const CEN_SIG: [u8; 4] = [0x50, 0x4b, 0x01, 0x02];
const LOC_SIG: [u8; 4] = [0x50, 0x4b, 0x03, 0x04];

fn u16le(b: &[u8], off: usize) -> u16 {
    b[off] as u16 | ((b[off + 1] as u16) << 8)
}

fn u32le(b: &[u8], off: usize) -> u32 {
    b[off] as u32
        | ((b[off + 1] as u32) << 8)
        | ((b[off + 2] as u32) << 16)
        | ((b[off + 3] as u32) << 24)
}

struct Member {
    name: String,
    method: u16,
    comp_size: u32,
    uncomp_size: u32,
    local_offset: u32,
}

fn http_range(url: &str, range_spec: &str, cap: u64) -> Option<Vec<u8>> {
    let out = Command::new("curl")
        .arg("-sSf")
        .arg("--max-filesize")
        .arg(cap.to_string())
        .arg("-r")
        .arg(range_spec)
        .arg(url)
        .output()
        .ok()?;
    if out.status.success() {
        Some(out.stdout)
    } else {
        None
    }
}

fn find_eocd(tail: &[u8]) -> Option<usize> {
    if tail.len() < 22 {
        return None;
    }
    for i in (0..=tail.len() - 4).rev() {
        if &tail[i..i + 4] == EOCD_SIG {
            return Some(i);
        }
    }
    None
}

struct Eocd {
    entries: u16,
    cd_size: u32,
    cd_offset: u32,
}

fn parse_eocd(tail: &[u8], at: usize) -> Option<Eocd> {
    let e = &tail[at..];
    if e.len() < 22 {
        return None;
    }
    let entries = u16le(e, 10);
    let cd_size = u32le(e, 12);
    let cd_offset = u32le(e, 16);
    if entries == 0xFFFF || cd_size == 0xFFFF_FFFF || cd_offset == 0xFFFF_FFFF {
        eprintln!("zip64 markers present — not in this tool's domain");
        return None;
    }
    Some(Eocd {
        entries,
        cd_size,
        cd_offset,
    })
}

fn central_members(cd: &[u8]) -> Vec<Member> {
    let mut members = Vec::new();
    let mut i = 0usize;
    while i + 46 <= cd.len() {
        if &cd[i..i + 4] != CEN_SIG {
            let skip = cd[i..].windows(4).position(|w| w == CEN_SIG);
            match skip {
                Some(0) => {}
                Some(n) => {
                    eprintln!("central directory: {} padding bytes at {}", n, i);
                    i += n;
                    continue;
                }
                None => break,
            }
        }
        let method = u16le(cd, i + 10);
        let comp_size = u32le(cd, i + 20);
        let uncomp_size = u32le(cd, i + 24);
        let name_len = u16le(cd, i + 28) as usize;
        let extra_len = u16le(cd, i + 30) as usize;
        let comment_len = u16le(cd, i + 32) as usize;
        let local_offset = u32le(cd, i + 42);
        let name_start = i + 46;
        if name_start + name_len > cd.len() {
            break;
        }
        let name = String::from_utf8_lossy(&cd[name_start..name_start + name_len]).to_string();
        members.push(Member {
            name,
            method,
            comp_size,
            uncomp_size,
            local_offset,
        });
        i = name_start + name_len + extra_len + comment_len;
    }
    members
}

fn extract_member(url: &str, m: &Member) -> Option<Vec<u8>> {
    let lh = http_range(
        url,
        &format!("{}-{}", m.local_offset, m.local_offset as u64 + 29),
        1 << 20,
    )?;
    if lh.len() < 30 {
        eprintln!("local header for {} truncated ({} bytes)", m.name, lh.len());
        return None;
    }
    if &lh[0..4] != LOC_SIG {
        eprintln!("local header signature absent at offset {}", m.local_offset);
        return None;
    }
    let name_len = u16le(&lh, 26) as u64;
    let extra_len = u16le(&lh, 28) as u64;
    let data_start = m.local_offset as u64 + 30 + name_len + extra_len;
    if m.comp_size == 0 {
        return Some(Vec::new());
    }
    let payload = http_range(
        url,
        &format!("{}-{}", data_start, data_start + m.comp_size as u64 - 1),
        m.comp_size as u64 + (1 << 20),
    )?;
    let out = match m.method {
        0 => payload,
        8 => match inflate(&payload) {
            Some(v) => v,
            None => {
                eprintln!("inflate returned void for {}", m.name);
                return None;
            }
        },
        other => {
            eprintln!("method {} not in {{stored, deflate}} for {}", other, m.name);
            return None;
        }
    };
    if m.uncomp_size != 0xFFFF_FFFF && out.len() != m.uncomp_size as usize {
        eprintln!(
            "member {}: {} bytes, central directory declares {}",
            m.name,
            out.len(),
            m.uncomp_size
        );
    }
    Some(out)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut url: Option<String> = None;
    let mut out: Option<String> = None;
    let mut list: Option<String> = None;
    let mut get: Option<String> = None;
    let mut tail: u64 = 131072;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--url" => {
                url = args.get(i + 1).cloned();
                i += 1;
            }
            "--out" => {
                out = args.get(i + 1).cloned();
                i += 1;
            }
            "--tail" => {
                tail = args
                    .get(i + 1)
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(131072);
                i += 1;
            }
            "--list" => {
                list = args.get(i + 1).cloned();
                i += 1;
            }
            "--get" => {
                get = args.get(i + 1).cloned();
                i += 1;
            }
            _ => {}
        }
        i += 1;
    }
    let url = match url {
        Some(u) => u,
        None => {
            eprintln!("--url absent");
            std::process::exit(1);
        }
    };
    if http_range(&url, "0-0", 1 << 20).is_none() {
        eprintln!("range probe 0-0 returned void");
        std::process::exit(1);
    }
    eprintln!("server honors range header (probe 0-0 answered)");
    let Some(full_tail) = http_range(&url, &format!("-{}", tail), tail + (1 << 20)) else {
        eprintln!(
            "tail range -{} returned void (zip smaller than tail?)",
            tail
        );
        std::process::exit(1);
    };
    if full_tail.len() > tail as usize + 64 {
        eprintln!(
            "tail response {} bytes — server delivered beyond the range (no range honor)",
            full_tail.len()
        );
        std::process::exit(1);
    }
    let Some(eocd_at) = find_eocd(&full_tail) else {
        eprintln!("EOCD absent in the last {} bytes", tail);
        std::process::exit(1);
    };
    let Some(eocd) = parse_eocd(&full_tail, eocd_at) else {
        eprintln!("EOCD unreadable at tail offset {}", eocd_at);
        std::process::exit(1);
    };
    eprintln!(
        "zip: {} entries, central directory {} bytes at {}",
        eocd.entries, eocd.cd_size, eocd.cd_offset
    );
    let cd_start = eocd.cd_offset as u64;
    let cd_end = cd_start + eocd.cd_size as u64 - 1;
    let Some(cd) = http_range(
        &url,
        &format!("{}-{}", cd_start, cd_end),
        eocd.cd_size as u64 + (1 << 20),
    ) else {
        eprintln!("central directory range returned void");
        std::process::exit(1);
    };
    let members = central_members(&cd);
    eprintln!("central directory: {} members parsed", members.len());
    if let Some(pat) = &list {
        for m in &members {
            if m.name.contains(pat.as_str()) {
                println!(
                    "{} method={} comp={} uncomp={} local={}",
                    m.name, m.method, m.comp_size, m.uncomp_size, m.local_offset
                );
            }
        }
        return;
    }
    let Some(pat) = &get else {
        eprintln!("--list <substr> or --get <substr> absent");
        std::process::exit(1);
    };
    let Some(m) = members.iter().find(|m| m.name.contains(pat.as_str())) else {
        eprintln!(
            "member containing \"{}\" not present in central directory",
            pat
        );
        std::process::exit(1);
    };
    eprintln!(
        "member: {} method={} comp={} uncomp={} local={}",
        m.name, m.method, m.comp_size, m.uncomp_size, m.local_offset
    );
    let Some(data) = extract_member(&url, m) else {
        eprintln!("extract returned void for {}", m.name);
        std::process::exit(1);
    };
    let out_path = match out {
        Some(p) => p,
        None => {
            eprintln!("--out absent");
            std::process::exit(1);
        }
    };
    if let Ok(mut f) = std::fs::File::create(&out_path) {
        let _ = f.write_all(&data);
    } else {
        eprintln!("write {} returned void", out_path);
        std::process::exit(1);
    }
    eprintln!("{} bytes → {}", data.len(), out_path);
}

#[cfg(test)]
mod tests {
    use super::{central_members, find_eocd, parse_eocd};

    fn entry(name: &str, comp: u32, uncomp: u32, local: u32) -> Vec<u8> {
        let mut e = vec![0u8; 46];
        e[0..4].copy_from_slice(&super::CEN_SIG);
        e[10..12].copy_from_slice(&8u16.to_le_bytes());
        e[20..24].copy_from_slice(&comp.to_le_bytes());
        e[24..28].copy_from_slice(&uncomp.to_le_bytes());
        e[28..30].copy_from_slice(&(name.len() as u16).to_le_bytes());
        e[42..46].copy_from_slice(&local.to_le_bytes());
        e.extend_from_slice(name.as_bytes());
        e
    }

    #[test]
    fn central_directory_parses_entries() {
        let mut cd = Vec::new();
        cd.extend(entry("doc/README.txt", 1234, 5678, 0x100));
        cd.extend(entry("dxlook.for", 42, 43, 0x1000));
        let members = central_members(&cd);
        assert_eq!(members.len(), 2);
        assert_eq!(members[0].name, "doc/README.txt");
        assert_eq!(members[0].method, 8);
        assert_eq!(members[0].comp_size, 1234);
        assert_eq!(members[0].local_offset, 0x100);
        assert_eq!(members[1].name, "dxlook.for");
    }

    #[test]
    fn eocd_fields_read_from_tail() {
        let mut tail = vec![0u8; 60];
        let at = 38;
        tail[at..at + 4].copy_from_slice(&super::EOCD_SIG);
        tail[at + 10..at + 12].copy_from_slice(&7u16.to_le_bytes());
        tail[at + 12..at + 16].copy_from_slice(&9000u32.to_le_bytes());
        tail[at + 16..at + 20].copy_from_slice(&500_000u32.to_le_bytes());
        let i = find_eocd(&tail).unwrap();
        assert_eq!(i, at);
        let e = parse_eocd(&tail, i).unwrap();
        assert_eq!(e.entries, 7);
        assert_eq!(e.cd_size, 9000);
        assert_eq!(e.cd_offset, 500_000);
    }
}
