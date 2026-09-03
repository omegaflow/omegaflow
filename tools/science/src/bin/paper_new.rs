use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

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

fn civil_from_days(z: i64) -> (i64, i64, i64) {
    let z = z + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    (if m <= 2 { y + 1 } else { y }, m, d)
}

fn today() -> String {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);
    let (y, m, d) = civil_from_days(secs / 86_400);
    format!("{:04}-{:02}-{:02}", y, m, d)
}

fn valid_slug(s: &str) -> bool {
    !s.is_empty()
        && s.chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
        && !s.starts_with('-')
        && !s.ends_with('-')
        && !s.contains("--")
}

fn run_gate(file: &str) -> i32 {
    let status = if Path::new("target/debug/export_latex").exists() {
        Command::new("target/debug/export_latex")
            .args(["--check", file])
            .status()
    } else {
        Command::new("cargo")
            .args([
                "run",
                "-p",
                "omegaflow-tools",
                "--bin",
                "export_latex",
                "--",
                "--check",
                file,
            ])
            .status()
    };
    status.expect("run gate").code().unwrap_or(1)
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut slug: Option<String> = None;
    let mut title: Option<String> = None;
    let mut date: Option<String> = None;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--title" => {
                title = Some(args.get(i + 1).cloned().unwrap_or_default());
                i += 2;
            }
            "--date" => {
                date = Some(args.get(i + 1).cloned().unwrap_or_default());
                i += 2;
            }
            other if slug.is_none() && !other.starts_with("--") => {
                slug = Some(other.to_string());
                i += 1;
            }
            other => {
                eprintln!("unknown arg: {}", other);
                std::process::exit(2);
            }
        }
    }

    let slug = match slug {
        Some(s) if valid_slug(&s) => s,
        Some(s) => {
            eprintln!("slug is not kebab-case (a-z 0-9 -): {}", s);
            std::process::exit(2);
        }
        None => {
            eprintln!("usage: paper_new <slug> [--title \"…\"] [--date YYYY-MM-DD]");
            std::process::exit(2);
        }
    };

    let title = title.unwrap_or_else(|| "Pending title".to_string());
    if title.chars().count() > 75 {
        eprintln!(
            "title is long: {} chars (gate: <= 75) — the scaffold is not written",
            title.chars().count()
        );
        std::process::exit(2);
    }

    let date = date.unwrap_or_else(today);

    let body = format!("# {}\n\n## Abstract\n\n## 1. The measurement\n\n", title);
    let sha = sha256_hex(body.as_bytes());
    let header = format!(
        "<!--\n  title: {}\n  class: paper\n  date: {}\n  sha256: {}\n  status: live\n  see-also: \n-->\n",
        title, date, sha
    );

    let path = format!("docs/paper/{}.md", slug);
    if Path::new(&path).exists() {
        eprintln!("paper already exists: {}", path);
        std::process::exit(2);
    }

    fs::write(&path, format!("{}{}", header, body)).expect("write paper");
    println!("scaffold: {} (sha {})", path, sha);

    let code = run_gate(&path);
    if code != 0 {
        fs::remove_file(&path).expect("remove violating scaffold");
        eprintln!("gate named a difference — scaffold removed");
        std::process::exit(1);
    }
    println!("gate: born conform");
}
