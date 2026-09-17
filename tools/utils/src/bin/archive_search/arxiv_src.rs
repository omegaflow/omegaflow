use std::fs;

use omegaflow::inflate::inflate;

struct TexFile {
    name: String,
    content: Vec<u8>,
}

pub fn run_lines(input: &str, out_dir: Option<&str>) -> Vec<String> {
    let Some(id) = resolve_id(input) else {
        return vec![format!("pending — no LaTeX source in {input}")];
    };
    let url = format!("https://arxiv.org/e-print/{id}");
    let Some(fetch) = crate::net::get(&url, &[], "60") else {
        return vec![format!("pending — no answer for {url}")];
    };
    let payload = if is_gzip(&fetch.raw) {
        match gunzip(&fetch.raw) {
            Some(decoded) => decoded,
            None => return vec![format!("pending — no LaTeX source in {input}")],
        }
    } else {
        fetch.raw
    };
    render(&payload, input, out_dir)
}

fn resolve_id(input: &str) -> Option<String> {
    let t = input.trim();
    let t = t
        .strip_prefix("arXiv:")
        .or_else(|| t.strip_prefix("arxiv:"))
        .unwrap_or(t)
        .trim();
    let candidate = if let Some(pos) = t.find("arxiv.org") {
        let rest = &t[pos + "arxiv.org".len()..];
        let rest = rest.split(['?', '#']).next().unwrap_or(rest);
        let rest = rest.trim_start_matches('/');
        let rest = rest
            .strip_prefix("abs/")
            .or_else(|| rest.strip_prefix("pdf/"))
            .or_else(|| rest.strip_prefix("e-print/"))
            .or_else(|| rest.strip_prefix("format/"))
            .unwrap_or(rest);
        rest.strip_suffix(".pdf").unwrap_or(rest)
    } else {
        t
    };
    validate_id(candidate)
}

fn validate_id(s: &str) -> Option<String> {
    let s = s.trim().trim_end_matches('/');
    if s.is_empty() {
        return None;
    }
    let (base, ver) = split_version(s);
    if !(is_modern_id(base) || is_old_id(base)) {
        return None;
    }
    if let Some(v) = ver {
        let digits = &v[1..];
        if !v.starts_with('v') || digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
            return None;
        }
    }
    Some(s.to_string())
}

fn split_version(s: &str) -> (&str, Option<&str>) {
    if let Some(pos) = s.rfind('v') {
        let ver = &s[pos..];
        let digits = &ver[1..];
        if !digits.is_empty()
            && digits.bytes().all(|b| b.is_ascii_digit())
            && pos > 0
            && s.as_bytes()[pos - 1].is_ascii_digit()
        {
            return (&s[..pos], Some(ver));
        }
    }
    (s, None)
}

fn is_modern_id(s: &str) -> bool {
    let Some((a, b)) = s.split_once('.') else {
        return false;
    };
    a.len() == 4
        && a.bytes().all(|c| c.is_ascii_digit())
        && (b.len() == 4 || b.len() == 5)
        && b.bytes().all(|c| c.is_ascii_digit())
}

fn is_old_id(s: &str) -> bool {
    let Some((archive, num)) = s.split_once('/') else {
        return false;
    };
    if archive.is_empty() {
        return false;
    }
    let (head, tail) = match archive.split_once('.') {
        Some((h, t)) => (h, Some(t)),
        None => (archive, None),
    };
    if head.is_empty() || !head.bytes().all(|c| c.is_ascii_lowercase() || c == b'-') {
        return false;
    }
    if let Some(t) = tail
        && (t.len() != 2 || !t.bytes().all(|c| c.is_ascii_uppercase()))
    {
        return false;
    }
    num.len() == 7 && num.bytes().all(|c| c.is_ascii_digit())
}

fn is_gzip(body: &[u8]) -> bool {
    body.len() >= 2 && body[0] == 0x1f && body[1] == 0x8b
}

fn gunzip(body: &[u8]) -> Option<Vec<u8>> {
    if body.len() < 10 || body[0] != 0x1f || body[1] != 0x8b || body[2] != 0x08 {
        return None;
    }
    let flags = body[3];
    let mut i = 10usize;
    if flags & 0x04 != 0 {
        let xlen = u16::from_le_bytes([*body.get(i)?, *body.get(i + 1)?]) as usize;
        i += 2 + xlen;
    }
    if flags & 0x08 != 0 {
        i = skip_cstr(body, i)?;
    }
    if flags & 0x10 != 0 {
        i = skip_cstr(body, i)?;
    }
    if flags & 0x02 != 0 {
        i += 2;
    }
    let data = body.get(i..)?;
    inflate(data)
}

fn skip_cstr(body: &[u8], mut i: usize) -> Option<usize> {
    while *body.get(i)? != 0 {
        i += 1;
    }
    Some(i + 1)
}

fn parse_tar(payload: &[u8]) -> Option<Vec<TexFile>> {
    if payload.len() < 512 || &payload[257..262] != b"ustar" {
        return None;
    }
    let mut files = Vec::new();
    let mut off = 0usize;
    let mut long_name: Option<String> = None;
    while off + 512 <= payload.len() {
        let block = &payload[off..off + 512];
        if block.iter().all(|&b| b == 0) {
            break;
        }
        let size = parse_octal(&block[124..136])?;
        let typeflag = block[156];
        let name = match long_name.take() {
            Some(n) => n,
            None => cstr(&block[0..100]),
        };
        let data_start = off + 512;
        let data_end = data_start.checked_add(size)?;
        if data_end > payload.len() {
            return None;
        }
        match typeflag {
            b'0' | 0 => {
                if is_tex(&name) {
                    files.push(TexFile {
                        name,
                        content: payload[data_start..data_end].to_vec(),
                    });
                }
            }
            b'L' => {
                let text = String::from_utf8_lossy(&payload[data_start..data_end]);
                long_name = Some(text.trim_end_matches('\0').to_string());
            }
            _ => {}
        }
        let padded = (size + 511) & !511;
        off = data_start + padded;
    }
    Some(files)
}

fn parse_octal(b: &[u8]) -> Option<usize> {
    let digits: Vec<u8> = b
        .iter()
        .copied()
        .take_while(|&c| c != 0 && c != b' ')
        .collect();
    if digits.is_empty() {
        return Some(0);
    }
    if digits[0] & 0x80 != 0 {
        return None;
    }
    let mut value = 0usize;
    for c in digits {
        if !(b'0'..=b'7').contains(&c) {
            return None;
        }
        value = value * 8 + (c - b'0') as usize;
    }
    Some(value)
}

fn cstr(b: &[u8]) -> String {
    let end = b.iter().position(|&c| c == 0).unwrap_or(b.len());
    String::from_utf8_lossy(&b[..end]).to_string()
}

fn is_tex(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    lower.ends_with(".tex") || lower.ends_with(".ltx")
}

fn safe_name(name: &str) -> String {
    let cleaned = name.replace('\\', "/");
    let parts: Vec<&str> = cleaned
        .split('/')
        .filter(|p| !p.is_empty() && *p != "." && *p != "..")
        .collect();
    if parts.is_empty() {
        "source.tex".to_string()
    } else {
        parts.join("/")
    }
}

fn render(payload: &[u8], input: &str, out_dir: Option<&str>) -> Vec<String> {
    if payload.is_empty() {
        return vec![format!("pending — no LaTeX source in {input}")];
    }
    let files = match parse_tar(payload) {
        Some(files) => files,
        None => vec![TexFile {
            name: "source.tex".to_string(),
            content: payload.to_vec(),
        }],
    };
    if files.is_empty() {
        return vec![format!("pending — no LaTeX source in {input}")];
    }
    match out_dir {
        None => {
            if files.len() == 1 {
                text_lines(&files[0].content)
            } else {
                let mut out = Vec::new();
                for f in &files {
                    out.push(format!("% ==== {} ====", f.name));
                    out.extend(text_lines(&f.content));
                }
                out
            }
        }
        Some(dir) => {
            let mut paths = Vec::new();
            for f in &files {
                let name = safe_name(&f.name);
                let path = std::path::Path::new(dir).join(&name);
                if let Some(parent) = path.parent()
                    && fs::create_dir_all(parent).is_err()
                {
                    continue;
                }
                if fs::write(&path, &f.content).is_err() {
                    continue;
                }
                paths.push(path.to_string_lossy().to_string());
            }
            if paths.is_empty() {
                vec![format!("pending — no LaTeX source in {input}")]
            } else {
                paths
            }
        }
    }
}

fn text_lines(content: &[u8]) -> Vec<String> {
    String::from_utf8_lossy(content)
        .lines()
        .map(|l| l.to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tar_entry(name: &str, content: &[u8]) -> Vec<u8> {
        let mut out = vec![0u8; 512];
        out[..name.len()].copy_from_slice(name.as_bytes());
        let size = format!("{:07o}", content.len());
        out[124..131].copy_from_slice(size.as_bytes());
        out[156] = b'0';
        out[257..262].copy_from_slice(b"ustar");
        out.extend_from_slice(content);
        let pad = (512 - content.len() % 512) % 512;
        out.extend(std::iter::repeat_n(0u8, pad));
        out
    }

    #[test]
    fn tar_tex_returns_name_and_content() {
        let body = b"\\documentclass{article}\n";
        let archive = tar_entry("paper/main.tex", body);
        let files = parse_tar(&archive).expect("ustar archive");
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].name, "paper/main.tex");
        assert_eq!(files[0].content, body.to_vec());
    }

    #[test]
    fn gunzip_decodes_stored_deflate_block() {
        let fixture: Vec<u8> = vec![
            0x1f, 0x8b, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, // gzip header
            0x01, 0x0b, 0x00, 0xf4, 0xff, // stored block: BFINAL, LEN=11, NLEN
            b'h', b'e', b'l', b'l', b'o', b' ', b'a', b'r', b'x', b'i', b'v', // payload
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // trailer
        ];
        assert!(is_gzip(&fixture));
        assert_eq!(gunzip(&fixture), Some(b"hello arxiv".to_vec()));
    }

    #[test]
    fn archive_without_tex_is_pending() {
        let archive = tar_entry("README.txt", b"no source here\n");
        assert_eq!(
            render(&archive, "2208.03865", None),
            vec!["pending — no LaTeX source in 2208.03865".to_string()]
        );
        assert_eq!(
            run_lines("not an arxiv identifier", None),
            vec!["pending — no LaTeX source in not an arxiv identifier".to_string()]
        );
    }
}
