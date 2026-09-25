use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::json::{JsonVal, parse_json};
use std::collections::HashMap;
use std::io::Read;
use std::process::Command;

const RECORD_API: &str = "https://zenodo.org/api/records";
const DEFAULT_RECORD: &str = "21132339";
const DEFAULT_OUT_DIR: &str = "data/zenodo.org";
const FETCH_TTL: u64 = 3600;
const WHOLE_BUFFER_CROSSCHECK_BYTES: u64 = 16 * 1024 * 1024;

const SHA256_H: [u32; 8] = [
    0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
];

const SHA256_K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

struct Sha256 {
    h: [u32; 8],
    buf: [u8; 64],
    buf_len: usize,
    total_len: u64,
}

impl Sha256 {
    fn new() -> Self {
        Self {
            h: SHA256_H,
            buf: [0u8; 64],
            buf_len: 0,
            total_len: 0,
        }
    }

    fn update(&mut self, mut data: &[u8]) {
        self.total_len = self.total_len.wrapping_add(data.len() as u64);
        if self.buf_len > 0 {
            let take = (64 - self.buf_len).min(data.len());
            self.buf[self.buf_len..self.buf_len + take].copy_from_slice(&data[..take]);
            self.buf_len += take;
            data = &data[take..];
            if self.buf_len == 64 {
                let block = self.buf;
                self.compress(&block);
                self.buf_len = 0;
            }
        }
        while data.len() >= 64 {
            let mut block = [0u8; 64];
            block.copy_from_slice(&data[..64]);
            self.compress(&block);
            data = &data[64..];
        }
        if !data.is_empty() {
            self.buf[..data.len()].copy_from_slice(data);
            self.buf_len = data.len();
        }
    }

    fn compress(&mut self, block: &[u8; 64]) {
        let mut w = [0u32; 64];
        for i in 0..16 {
            w[i] = (u32::from(block[i * 4]) << 24)
                | (u32::from(block[i * 4 + 1]) << 16)
                | (u32::from(block[i * 4 + 2]) << 8)
                | u32::from(block[i * 4 + 3]);
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16]
                .wrapping_add(s0)
                .wrapping_add(w[i - 7])
                .wrapping_add(s1);
        }
        let mut a = self.h[0];
        let mut b = self.h[1];
        let mut c = self.h[2];
        let mut d = self.h[3];
        let mut e = self.h[4];
        let mut f = self.h[5];
        let mut g = self.h[6];
        let mut h = self.h[7];
        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ ((!e) & g);
            let temp1 = h
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(SHA256_K[i])
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
        self.h[0] = self.h[0].wrapping_add(a);
        self.h[1] = self.h[1].wrapping_add(b);
        self.h[2] = self.h[2].wrapping_add(c);
        self.h[3] = self.h[3].wrapping_add(d);
        self.h[4] = self.h[4].wrapping_add(e);
        self.h[5] = self.h[5].wrapping_add(f);
        self.h[6] = self.h[6].wrapping_add(g);
        self.h[7] = self.h[7].wrapping_add(h);
    }

    fn finalize(mut self) -> [u8; 32] {
        let bitlen = self.total_len.wrapping_mul(8);
        self.update(&[0x80]);
        while self.buf_len != 56 {
            self.update(&[0]);
        }
        let mut len_bytes = [0u8; 8];
        for i in 0..8 {
            len_bytes[i] = ((bitlen >> ((7 - i) * 8)) & 0xff) as u8;
        }
        self.update(&len_bytes);
        let mut out = [0u8; 32];
        for (i, v) in self.h.iter().enumerate() {
            out[i * 4..i * 4 + 4].copy_from_slice(&v.to_be_bytes());
        }
        out
    }
}

fn hex_of(digest: &[u8; 32]) -> String {
    let mut out = String::with_capacity(64);
    for b in digest {
        out.push_str(&format!("{b:02x}"));
    }
    out
}

fn obj_of(v: &JsonVal) -> Option<&HashMap<String, JsonVal>> {
    match v {
        JsonVal::Obj(m) => Some(m),
        _ => None,
    }
}

fn str_at(m: &HashMap<String, JsonVal>, key: &str) -> Option<String> {
    match m.get(key) {
        Some(JsonVal::Str(s)) => Some(s.clone()),
        _ => None,
    }
}

fn num_at(m: &HashMap<String, JsonVal>, key: &str) -> Option<f64> {
    match m.get(key) {
        Some(JsonVal::Num(n)) => Some(*n),
        _ => None,
    }
}

struct ZenodoFile {
    key: String,
    size: u64,
    checksum: String,
    content_url: String,
}

fn parse_record(json: &JsonVal) -> Option<(String, String, Vec<ZenodoFile>)> {
    let root = obj_of(json)?;
    let meta = obj_of(root.get("metadata")?)?;
    let title = str_at(meta, "title")?;
    let access = str_at(meta, "access_right")?;
    let files = match root.get("files") {
        Some(JsonVal::Arr(arr)) => arr,
        _ => return None,
    };
    let mut out = Vec::with_capacity(files.len());
    for f in files {
        let m = obj_of(f)?;
        let key = str_at(m, "key")?;
        let size = num_at(m, "size")? as u64;
        let checksum = str_at(m, "checksum")?;
        let content_url = obj_of(m.get("links")?).and_then(|l| str_at(l, "self"))?;
        if content_url.is_empty() {
            return None;
        }
        out.push(ZenodoFile {
            key,
            size,
            checksum,
            content_url,
        });
    }
    Some((title, access, out))
}

fn download(url: &str, dest: &str) -> bool {
    let out = Command::new("curl")
        .arg("-sSL")
        .arg("-g")
        .arg("--retry")
        .arg("3")
        .arg("--retry-delay")
        .arg("2")
        .arg("--connect-timeout")
        .arg("30")
        .arg("-m")
        .arg("7200")
        .arg("-o")
        .arg(dest)
        .arg(url)
        .output();
    match out {
        Ok(o) if o.status.success() => true,
        Ok(o) => {
            eprintln!(
                "download {url}: curl returned {} — {}",
                o.status,
                String::from_utf8_lossy(&o.stderr).trim()
            );
            false
        }
        Err(e) => {
            eprintln!("download {url}: curl void — {e}");
            false
        }
    }
}

fn sha256_of_file(path: &str) -> Option<(u64, [u8; 32])> {
    let mut file = std::fs::File::open(path).ok()?;
    let mut hasher = Sha256::new();
    let mut total = 0u64;
    let mut chunk = [0u8; 1 << 20];
    loop {
        match file.read(&mut chunk) {
            Ok(0) => break,
            Ok(n) => {
                hasher.update(&chunk[..n]);
                total += n as u64;
            }
            Err(_) => return None,
        }
    }
    Some((total, hasher.finalize()))
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let record = match arg_value(&args, "--record") {
        Some(v) => v,
        None => DEFAULT_RECORD.to_string(),
    };
    let out_dir = match arg_value(&args, "--out") {
        Some(v) => v,
        None => DEFAULT_OUT_DIR.to_string(),
    };
    let download_key = arg_value(&args, "--download");

    let api_url = format!("{RECORD_API}/{record}");
    let Some(bytes) = fetch_raw_bytes(&api_url, FETCH_TTL) else {
        eprintln!("{api_url}: fetch void");
        std::process::exit(1);
    };
    let Some(json) = String::from_utf8(bytes).ok().and_then(|t| parse_json(&t)) else {
        eprintln!("{api_url}: body parses void");
        std::process::exit(1);
    };
    let Some((title, access, files)) = parse_record(&json) else {
        eprintln!("{api_url}: record shape carries no files array");
        std::process::exit(1);
    };
    eprintln!(
        "record {record} — \"{title}\" (access {access}, {} files)",
        files.len()
    );
    for f in &files {
        println!("{} {} {} {}", f.key, f.size, f.checksum, f.content_url);
    }

    let Some(key) = download_key else {
        return;
    };
    let Some(target) = files.iter().find(|f| f.key == key) else {
        eprintln!("record {record}: no file named {key}");
        std::process::exit(1);
    };
    std::fs::create_dir_all(&out_dir).ok();
    let dest = format!("{out_dir}/{key}");
    if !download(&target.content_url, &dest) {
        std::process::exit(1);
    }
    let Some((total, digest)) = sha256_of_file(&dest) else {
        eprintln!("{dest}: streamed hash void");
        std::process::exit(1);
    };
    if total != target.size {
        eprintln!(
            "{}: {} B on disk, record carries {} B — the stream is incomplete",
            key, total, target.size
        );
        std::process::exit(1);
    }
    if total <= WHOLE_BUFFER_CROSSCHECK_BYTES
        && let Ok(whole) = std::fs::read(&dest)
        && omegaflow::archivar::sha256::sha256_hex(&whole) != hex_of(&digest)
    {
        eprintln!("{key}: streamed digest diverges from whole-buffer digest");
        std::process::exit(1);
    }
    println!("{key} {} B sha256 {}", total, hex_of(&digest));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sha256_hex_of(bytes: &[u8]) -> String {
        let mut h = Sha256::new();
        h.update(bytes);
        hex_of(&h.finalize())
    }

    #[test]
    fn empty_stream_is_the_known_digest() {
        assert_eq!(
            sha256_hex_of(b""),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn abc_is_the_known_digest() {
        assert_eq!(
            sha256_hex_of(b"abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    #[test]
    fn multi_block_stream_stays_correct() {
        assert_eq!(
            sha256_hex_of(&b"a".repeat(64)),
            "ffe054fe7ae0cb6dc65c3af9b61d5209f439851db43d0ba5997337df154668eb"
        );
    }

    #[test]
    fn chunked_update_matches_single_update() {
        let data = b"x".repeat(100_000);
        let mut whole = Sha256::new();
        whole.update(&data);
        let a = whole.finalize();
        let mut chunked = Sha256::new();
        for c in data.chunks(777) {
            chunked.update(c);
        }
        assert_eq!(chunked.finalize(), a);
    }
}
