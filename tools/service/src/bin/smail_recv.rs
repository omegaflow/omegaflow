use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;

use omegaflow::json::{JsonVal, parse_json};

fn main() {
    let port: u16 = env_u64("OMEGAFLOW_MAIL_PORT", 1619) as u16;
    let token = env_str("OMEGAFLOW_MAIL_TOKEN", "");
    let default_ledger = state_dir().join("mail/mail_ledger.φ");
    let ledger = env_str("OMEGAFLOW_MAIL_LEDGER", &default_ledger.to_string_lossy());
    let listener = match TcpListener::bind(format!("127.0.0.1:{}", port)) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("smail-recv: bind 127.0.0.1:{} refused: {}", port, e);
            return;
        }
    };
    eprintln!(
        "smail-recv: webhook listens on 127.0.0.1:{} (ledger {}, token {})",
        port,
        ledger,
        if token.is_empty() {
            "offen"
        } else {
            "gefordert"
        }
    );
    for conn in listener.incoming() {
        let Ok(mut stream) = conn else { continue };
        let ledger = ledger.clone();
        let token = token.clone();
        std::thread::spawn(move || {
            stream.set_read_timeout(Some(Duration::from_secs(10))).ok();
            let Some((method, path, headers, body)) = read_request(&mut stream) else {
                return;
            };
            if method == "GET" && path.starts_with("/health") {
                let ok = b"smail-recv ok\n";
                let head = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                    ok.len()
                );
                let _ = stream.write_all(head.as_bytes());
                let _ = stream.write_all(ok);
                return;
            }
            let authorized = token.is_empty()
                || headers
                    .iter()
                    .find(|(k, _)| k == "authorization")
                    .map(|(_, v)| v == &format!("Bearer {}", token))
                    .unwrap_or(false);
            if method != "POST" || path != "/mail" || !authorized {
                let _ = stream.write_all(
                    b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
                );
                return;
            }
            let text = String::from_utf8_lossy(&body).to_string();
            let line = match record_line(&text) {
                Some(l) => l,
                None => {
                    let _ = stream.write_all(
                        b"HTTP/1.1 400 Bad Request\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
                    );
                    return;
                }
            };
            let appended = append_ledger(&ledger, &line);
            match appended {
                true => {
                    let _ = stream.write_all(
                        b"HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 2\r\nConnection: close\r\n\r\nok",
                    );
                }
                false => {
                    let _ = stream.write_all(
                        b"HTTP/1.1 500 Internal Server Error\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
                    );
                }
            }
        });
    }
}

fn read_request(
    stream: &mut TcpStream,
) -> Option<(String, String, Vec<(String, String)>, Vec<u8>)> {
    let mut buf: Vec<u8> = Vec::new();
    let mut tmp = [0u8; 4096];
    let header_end;
    loop {
        match stream.read(&mut tmp) {
            Ok(0) => return None,
            Ok(n) => {
                buf.extend_from_slice(&tmp[..n]);
                if let Some(pos) = find_header_end(&buf) {
                    header_end = pos;
                    break;
                }
            }
            Err(_) => return None,
        }
    }
    let head = String::from_utf8_lossy(&buf[..header_end]).to_string();
    let mut lines = head.split("\r\n");
    let request_line = lines.next().unwrap_or("").to_string();
    let mut headers = Vec::new();
    for line in lines {
        if line.is_empty() {
            continue;
        }
        if let Some((k, v)) = line.split_once(':') {
            headers.push((k.trim().to_lowercase(), v.trim().to_string()));
        }
    }
    let content_length: usize = headers
        .iter()
        .find(|(k, _)| k == "content-length")
        .and_then(|(_, v)| v.parse().ok())
        .unwrap_or(0);
    let mut body = buf[header_end + 4..].to_vec();
    while body.len() < content_length {
        match stream.read(&mut tmp) {
            Ok(0) => break,
            Ok(n) => body.extend_from_slice(&tmp[..n]),
            Err(_) => break,
        }
    }
    body.truncate(content_length);
    let mut parts = request_line.split_whitespace();
    let method = parts.next().unwrap_or("").to_string();
    let path = parts.next().unwrap_or("").to_string();
    Some((method, path, headers, body))
}

fn find_header_end(buf: &[u8]) -> Option<usize> {
    buf.windows(4).position(|w| w == b"\r\n\r\n")
}

fn json_str(v: &JsonVal, key: &str) -> String {
    match v {
        JsonVal::Obj(map) => match map.get(key) {
            Some(JsonVal::Str(s)) => s.clone(),
            _ => String::new(),
        },
        _ => String::new(),
    }
}

fn record_line(text: &str) -> Option<String> {
    let parsed = parse_json(text)?;
    let from = json_str(&parsed, "from");
    let to = json_str(&parsed, "to");
    let subject = json_str(&parsed, "subject");
    let raw = json_str(&parsed, "text");
    if from.is_empty() || to.is_empty() {
        return None;
    }
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let subject_clean = subject
        .replace('\r', "")
        .replace('\t', " ")
        .replace('\n', " ");
    let body_clean = mime_plaintext(&raw)
        .replace('\r', "")
        .replace('\t', " ")
        .replace('\n', " ");
    Some(format!(
        "mail\t{}\t{}\t{}\t{}\t{}",
        ts, from, to, subject_clean, body_clean
    ))
}

fn mime_plaintext(raw: &str) -> String {
    let boundary = header_value(raw, "content-type").and_then(|ct| {
        ct.split(';')
            .map(|s| s.trim())
            .find(|s| s.to_lowercase().starts_with("boundary="))
            .map(|s| s["boundary=".len()..].trim_matches('"').to_string())
    });
    match boundary {
        Some(b) => {
            let mut out = String::new();
            for part in raw.split(&format!("--{}", b)) {
                if let Some(pl) = plaintext_of_part(part) {
                    if !out.is_empty() {
                        out.push('\n');
                    }
                    out.push_str(&pl);
                }
            }
            out
        }
        None => plaintext_of_part(raw).unwrap_or_default(),
    }
}

fn plaintext_of_part(part: &str) -> Option<String> {
    let part = part.trim_start_matches(['\r', '\n']);
    if let Some(sep) = part.find("\r\n\r\n") {
        let head = &part[..sep];
        let body = &part[sep + 4..];
        let is_plain = header_value(head, "content-type")
            .map(|ct| ct.to_lowercase().starts_with("text/plain"))
            .unwrap_or(false);
        if !is_plain {
            return None;
        }
        let enc = header_value(head, "content-transfer-encoding")
            .unwrap_or_default()
            .to_lowercase();
        if enc.contains("base64") {
            return Some(decode_base64(body));
        }
        if enc.contains("quoted-printable") {
            return Some(decode_quoted_printable(body));
        }
        return Some(body.trim().to_string());
    }
    None
}

fn header_value(block: &str, name: &str) -> Option<String> {
    let mut found: Option<String> = None;
    let mut current: Option<String> = None;
    for line in block.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            break;
        }
        if line.starts_with(' ') || line.starts_with('\t') {
            if let Some(v) = current.as_mut() {
                v.push(' ');
                v.push_str(trimmed);
            }
            continue;
        }
        if let Some((k, v)) = line.split_once(':') {
            if k.trim().to_lowercase() == name {
                current = Some(v.trim().to_string());
                found = Some(v.trim().to_string());
            } else {
                current = None;
            }
        } else {
            current = None;
        }
    }
    if current.is_some() {
        found = current;
    }
    found
}

fn decode_base64(s: &str) -> String {
    let table = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out: Vec<u8> = Vec::new();
    let mut buf: u32 = 0;
    let mut bits = 0;
    for c in s.bytes() {
        if c == b'=' || c == b'\r' || c == b'\n' || c == b' ' {
            continue;
        }
        let v = table.iter().position(|&t| t == c);
        let Some(v) = v else { continue };
        buf = (buf << 6) | v as u32;
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            out.push((buf >> bits) as u8);
        }
    }
    String::from_utf8_lossy(&out).to_string()
}

fn decode_quoted_printable(s: &str) -> String {
    let mut out: Vec<u8> = Vec::new();
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'=' && i + 1 < bytes.len() {
            if bytes[i + 1] == b'\r' && i + 2 < bytes.len() && bytes[i + 2] == b'\n' {
                i += 3;
                continue;
            }
            if i + 2 < bytes.len() {
                let hex = &s[i + 1..i + 3];
                if let Ok(v) = u8::from_str_radix(hex, 16) {
                    out.push(v);
                    i += 3;
                    continue;
                }
            }
        }
        out.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).to_string()
}

fn append_ledger(path: &str, line: &str) -> bool {
    match std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
    {
        Ok(mut f) => f.write_all(format!("{}\n", line).as_bytes()).is_ok(),
        Err(_) => false,
    }
}

fn env_u64(name: &str, default: u64) -> u64 {
    std::env::var(name)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

fn env_str(name: &str, default: &str) -> String {
    std::env::var(name).unwrap_or_else(|_| default.to_string())
}

fn state_dir() -> std::path::PathBuf {
    if let Ok(dir) = std::env::var("OMEGAFLOW_STATE") {
        return std::path::PathBuf::from(dir);
    }
    std::path::PathBuf::from(".")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn record_line_from_payload() {
        let text = r#"{"from":"a@x.io","to":"code@omegaflow.space","subject":"hi","text":"Content-Type: text/plain\r\n\r\nbody"}"#;
        let line = record_line(text).unwrap();
        let parts: Vec<&str> = line.split('\t').collect();
        assert_eq!(parts.len(), 6);
        assert_eq!(parts[0], "mail");
        assert_eq!(parts[2], "a@x.io");
        assert_eq!(parts[3], "code@omegaflow.space");
        assert_eq!(parts[4], "hi");
        assert_eq!(parts[5], "body");
    }

    #[test]
    fn record_line_missing_from_is_void() {
        let text = r#"{"to":"code@omegaflow.space"}"#;
        assert!(record_line(text).is_none());
    }

    #[test]
    fn record_line_subject_newlines_collapsed() {
        let text = r#"{"from":"a@x.io","to":"b@x.io","subject":"a\nb","text":"c"}"#;
        let line = record_line(text).unwrap();
        assert!(!line.contains('\n'));
    }

    #[test]
    fn record_line_carriage_returns_stripped() {
        let text = r#"{"from":"a@x.io","to":"b@x.io","subject":"s","text":"Content-Type: text/plain\r\n\r\nline1\r\nline2"}"#;
        let line = record_line(text).unwrap();
        assert!(!line.contains('\r'));
    }

    #[test]
    fn plaintext_from_simple_part() {
        let raw = "Content-Type: text/plain; charset=utf-8\r\n\r\nhello world";
        assert_eq!(mime_plaintext(raw), "hello world");
    }

    #[test]
    fn plaintext_base64_decoded() {
        let raw = "Content-Type: text/plain\r\nContent-Transfer-Encoding: base64\r\n\r\naGVsbG8=";
        assert_eq!(mime_plaintext(raw), "hello");
    }

    #[test]
    fn plaintext_multipart_extracts_text_part() {
        let raw = "Content-Type: multipart/alternative; boundary=b1\r\n\r\n--b1\r\nContent-Type: text/plain\r\n\r\nplain body\r\n--b1\r\nContent-Type: text/html\r\n\r\n<p>hi</p>\r\n--b1--";
        assert_eq!(mime_plaintext(raw), "plain body");
    }

    #[test]
    fn quoted_printable_decoded() {
        let raw = "Content-Type: text/plain\r\nContent-Transfer-Encoding: quoted-printable\r\n\r\nGr=C3=BC=C3=9F";
        assert!(mime_plaintext(raw).contains("Grüß"));
    }

    #[test]
    fn folded_content_type_boundary() {
        let raw = "Content-Type: multipart/alternative;\r\n\tboundary=\"b1=_X\"\r\n\r\n--b1=_X\r\nContent-Type: text/plain\r\n\r\nhello\r\n--b1=_X--";
        assert_eq!(mime_plaintext(raw), "hello");
    }
}
