use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn sha256_hex(data: &[u8]) -> String {
    let mut digest = [0u32; 8];
    digest[0] = 0x6a09e667;
    digest[1] = 0xbb67ae85;
    digest[2] = 0x3c6ef372;
    digest[3] = 0xa54ff53a;
    digest[4] = 0x510e527f;
    digest[5] = 0x9b05688c;
    digest[6] = 0x1f83d9ab;
    digest[7] = 0x5be0cd19;
    let mut msg: Vec<u8> = Vec::with_capacity(data.len() + 64);
    msg.extend_from_slice(data);
    let bitlen = (data.len() as u64).wrapping_mul(8);
    msg.push(0x80);
    while (msg.len() % 64) != 56 {
        msg.push(0);
    }
    for i in 0..8 {
        let shift = (7 - i) * 8;
        msg.push(((bitlen >> shift) & 0xff) as u8);
    }
    let k: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4,
        0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe,
        0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f,
        0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc,
        0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
        0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116,
        0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
        0xc67178f2,
    ];
    for chunk in msg.chunks(64) {
        let mut w = [0u32; 64];
        for i in 0..16 {
            w[i] = ((chunk[i * 4] as u32) << 24)
                | ((chunk[i * 4 + 1] as u32) << 16)
                | ((chunk[i * 4 + 2] as u32) << 8)
                | (chunk[i * 4 + 3] as u32);
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16]
                .wrapping_add(s0)
                .wrapping_add(w[i - 7])
                .wrapping_add(s1);
        }
        let mut a = digest[0];
        let mut b = digest[1];
        let mut c = digest[2];
        let mut d = digest[3];
        let mut e = digest[4];
        let mut f = digest[5];
        let mut g = digest[6];
        let mut h = digest[7];
        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ ((!e) & g);
            let temp1 = h
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(k[i])
                .wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(maj);
            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }
        digest[0] = digest[0].wrapping_add(a);
        digest[1] = digest[1].wrapping_add(b);
        digest[2] = digest[2].wrapping_add(c);
        digest[3] = digest[3].wrapping_add(d);
        digest[4] = digest[4].wrapping_add(e);
        digest[5] = digest[5].wrapping_add(f);
        digest[6] = digest[6].wrapping_add(g);
        digest[7] = digest[7].wrapping_add(h);
    }
    let mut out = String::with_capacity(64);
    for v in digest {
        out.push_str(&format!("{:08x}", v));
    }
    out
}

fn strip_header(src: &str) -> (String, String) {
    if let Some(start) = src.find("<!--") {
        if let Some(end) = src[start + 4..].find("-->") {
            let block = &src[start + 4..start + 4 + end];
            let mut rest = start + 4 + end + 3;
            if src[rest..].starts_with("\r\n") {
                rest += 2;
            } else if src[rest..].starts_with('\n') {
                rest += 1;
            }
            return (block.to_string(), src[rest..].to_string());
        }
    }
    (String::new(), src.to_string())
}

fn header_field(block: &str, key: &str) -> String {
    for line in block.lines() {
        let line = line.trim();
        if let Some((k, v)) = line.split_once(':') {
            if k.trim() == key {
                return v.trim().to_string();
            }
        }
    }
    String::new()
}

fn abstract_text(body: &str) -> String {
    let mut lines: Vec<String> = Vec::new();
    let mut in_abstract = false;
    for line in body.lines() {
        let t = line.trim();
        if t == "## Abstract" || t == "# Abstract" {
            in_abstract = true;
            continue;
        }
        if in_abstract {
            if t.starts_with('#') {
                break;
            }
            if t.is_empty() {
                continue;
            }
            lines.push(t.to_string());
        }
    }
    lines.join(" ")
}

fn read_token() -> Option<String> {
    if let Ok(t) = env::var("ARXIV_TOKEN") {
        if !t.is_empty() {
            return Some(t);
        }
    }
    let body = fs::read_to_string(".secrets.local").ok()?;
    for line in body.lines() {
        if let Some((k, v)) = line.split_once('=') {
            if k.trim() == "ARXIV_TOKEN" && !v.trim().is_empty() {
                return Some(v.trim().to_string());
            }
        }
    }
    None
}

fn main() {
    let slug = match env::args().nth(1) {
        Some(s) if !s.is_empty() => s,
        _ => {
            eprintln!("usage: arxiv_submit <slug>");
            std::process::exit(2);
        }
    };
    let md = format!("docs/paper/{}.md", slug);
    if !Path::new(&md).exists() {
        eprintln!("no such paper: {}", md);
        std::process::exit(2);
    }

    let status = Command::new("cargo")
        .args([
            "run",
            "-q",
            "-p",
            "omegaflow-tools",
            "--bin",
            "export_latex",
            "--",
            "--check",
            &md,
        ])
        .status()
        .expect("run gate");
    if !status.success() {
        eprintln!("the paper carries a named difference — it does not leave the field");
        std::process::exit(1);
    }

    let src = fs::read_to_string(&md).expect("read paper");
    let (block, body) = strip_header(&src);
    let title = header_field(&block, "title");
    let abstract_words = abstract_text(&body).split_whitespace().count();
    let body_sha = sha256_hex(body.as_bytes());

    match read_token() {
        None => {
            eprintln!("pending — no ARXIV_TOKEN (env or .secrets.local)");
            eprintln!(
                "the arXiv account, the endorsement and the submission token are Leitstelle work; no token is fabricated."
            );
            println!("ready payload (not sent):");
            println!("  slug:     {}", slug);
            println!("  title:    {}", title);
            println!("  abstract: {} words", abstract_words);
            println!("  body sha: {}", body_sha);
            std::process::exit(2);
        }
        Some(_token) => {
            println!("ready payload (not sent):");
            println!("  slug:     {}", slug);
            println!("  title:    {}", title);
            println!("  abstract: {} words", abstract_words);
            println!("  tex:      docs/paper/export/{}.tex", slug);
            println!("  body sha: {}", body_sha);
            eprintln!(
                "pending — the submission endpoint is not carried by the public arXiv API docs (info.arxiv.org/help/api/submission.html carries no entry); the named path is the web UI (Submit TeX/LaTeX) or arXiv third-party submission. The endpoint is named by the Leitstelle when the account exists."
            );
            std::process::exit(1);
        }
    }
}
