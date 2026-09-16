use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;

use omegaflow::json::{JsonVal, parse_json};

fn main() {
    let port: u16 = env_u64("OMEGAFLOW_MAIL_PORT", 1619) as u16;
    let token = env_str("OMEGAFLOW_MAIL_TOKEN", "");
    let default_ledger = state_dir().join("mail/mail_ledger.φ");
    let ledger = env_str("OMEGAFLOW_MAIL_LEDGER", &default_ledger.to_string_lossy());
    let default_seen = state_dir().join("mail/seen_ids.φ");
    let seen = env_str("OMEGAFLOW_MAIL_SEEN", &default_seen.to_string_lossy());
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
        if token.is_empty() { "open" } else { "required" }
    );
    for conn in listener.incoming() {
        let Ok(mut stream) = conn else { continue };
        let ledger = ledger.clone();
        let seen = seen.clone();
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
            let Some((line, message_id)) = record_line(&text) else {
                let _ = stream.write_all(
                    b"HTTP/1.1 400 Bad Request\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
                );
                return;
            };
            if !message_id.is_empty() && seen_contains(&seen, &message_id) {
                let _ = stream.write_all(
                    b"HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 4\r\nConnection: close\r\n\r\nseen",
                );
                return;
            }
            if !append_ledger(&ledger, &line) {
                let _ = stream.write_all(
                    b"HTTP/1.1 500 Internal Server Error\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
                );
                return;
            }
            if !message_id.is_empty() {
                let _ = append_ledger(&seen, &message_id);
            }
            let _ = stream.write_all(
                b"HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 2\r\nConnection: close\r\n\r\nok",
            );
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
    let content_length: usize = match headers.iter().find(|(k, _)| k == "content-length") {
        Some((_, v)) => match v.trim().parse::<usize>() {
            Ok(n) => n,
            Err(_) => return None,
        },
        None => 0,
    };
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

fn record_line(text: &str) -> Option<(String, String)> {
    let parsed = parse_json(text)?;
    let from = json_str(&parsed, "from");
    let to = json_str(&parsed, "to");
    let subject = json_str(&parsed, "subject");
    let raw = json_str(&parsed, "text");
    let message_id = json_str(&parsed, "messageId");
    if from.is_empty() || to.is_empty() {
        return None;
    }
    let ts = match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
        Ok(d) => d.as_secs(),
        Err(_) => return None,
    };
    let subject_clean = subject
        .replace('\r', "")
        .replace('\t', " ")
        .replace('\n', " ");
    let body_clean = mime_plaintext(&raw)
        .replace('\r', "")
        .replace('\t', " ")
        .replace('\n', " ");
    Some((
        format!(
            "mail\t{}\t{}\t{}\t{}\t{}\t{}",
            ts, from, to, subject_clean, body_clean, message_id
        ),
        message_id,
    ))
}

fn mime_plaintext(raw: &str) -> String {
    let (plain, html) = collect_text(raw);
    if plain.is_empty() {
        strip_html(&html)
    } else {
        plain
    }
}

fn strip_html(s: &str) -> String {
    let mut out = String::new();
    let mut in_tag = false;
    for c in s.chars() {
        match c {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => out.push(c),
            _ => {}
        }
    }
    out
}

fn collect_text(part: &str) -> (String, String) {
    let part = part.trim_start_matches(['\r', '\n']);
    let Some((head, body)) = split_headers_body(part) else {
        return (String::new(), String::new());
    };
    let content_type = match header_value(head, "content-type") {
        Some(ct) => ct.to_lowercase(),
        None => String::new(),
    };
    if content_type.starts_with("multipart/") {
        let Some(boundary) = content_type
            .split(';')
            .map(|s| s.trim())
            .find(|s| s.starts_with("boundary="))
            .map(|s| s["boundary=".len()..].trim_matches('"').to_string())
        else {
            return (String::new(), String::new());
        };
        let mut plain = String::new();
        let mut html = String::new();
        for sub in body.split(&format!("--{}", boundary)) {
            let (p, h) = collect_text(sub);
            if !p.is_empty() {
                if !plain.is_empty() {
                    plain.push('\n');
                }
                plain.push_str(&p);
            }
            if !h.is_empty() {
                if !html.is_empty() {
                    html.push('\n');
                }
                html.push_str(&h);
            }
        }
        (plain, html)
    } else if content_type.starts_with("text/html") {
        (String::new(), decode_body(head, body))
    } else {
        (decode_body(head, body), String::new())
    }
}

fn split_headers_body(part: &str) -> Option<(&str, &str)> {
    if let Some(i) = part.find("\r\n\r\n") {
        return Some((&part[..i], &part[i + 4..]));
    }
    if let Some(i) = part.find("\n\n") {
        return Some((&part[..i], &part[i + 2..]));
    }
    None
}

fn decode_body(head: &str, body: &str) -> String {
    let encoding = match header_value(head, "content-transfer-encoding") {
        Some(enc) => enc.to_lowercase(),
        None => String::new(),
    };
    if encoding.contains("base64") {
        decode_base64(body)
    } else if encoding.contains("quoted-printable") {
        decode_quoted_printable(body)
    } else {
        body.trim().to_string()
    }
}

fn header_value(block: &str, name: &str) -> Option<String> {
    let mut found: Option<String> = None;
    let mut in_target = false;
    for line in block.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            break;
        }
        if line.starts_with(' ') || line.starts_with('\t') {
            if in_target {
                if let Some(v) = found.as_mut() {
                    v.push(' ');
                    v.push_str(trimmed);
                }
            }
            continue;
        }
        if let Some((k, v)) = line.split_once(':') {
            in_target = k.trim().to_lowercase() == name;
            if in_target {
                found = Some(v.trim().to_string());
            }
        } else {
            in_target = false;
        }
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

fn seen_contains(path: &str, id: &str) -> bool {
    match std::fs::read_to_string(path) {
        Ok(s) => s.lines().any(|l| l == id),
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
    match std::env::var(name) {
        Ok(v) => v,
        Err(_) => default.to_string(),
    }
}

fn state_dir() -> std::path::PathBuf {
    if let Ok(dir) = std::env::var("OMEGAFLOW_STATE") {
        return std::path::PathBuf::from(dir);
    }
    std::path::PathBuf::from("state")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn record_line_from_payload() {
        let text = r#"{"from":"a@x.io","to":"code@omegaflow.space","subject":"hi","text":"Content-Type: text/plain\r\n\r\nbody","messageId":"<id-1@x.io>"}"#;
        let (line, message_id) = record_line(text).unwrap();
        let parts: Vec<&str> = line.split('\t').collect();
        assert_eq!(parts.len(), 7);
        assert_eq!(parts[0], "mail");
        assert_eq!(parts[2], "a@x.io");
        assert_eq!(parts[3], "code@omegaflow.space");
        assert_eq!(parts[4], "hi");
        assert_eq!(parts[5], "body");
        assert_eq!(parts[6], "<id-1@x.io>");
        assert_eq!(message_id, "<id-1@x.io>");
    }

    #[test]
    fn record_line_missing_from_is_void() {
        let text = r#"{"to":"code@omegaflow.space"}"#;
        assert!(record_line(text).is_none());
    }

    #[test]
    fn record_line_subject_newlines_collapsed() {
        let text = r#"{"from":"a@x.io","to":"b@x.io","subject":"a\nb","text":"c"}"#;
        let (line, _) = record_line(text).unwrap();
        assert!(!line.contains('\n'));
    }

    #[test]
    fn record_line_carriage_returns_stripped() {
        let text = r#"{"from":"a@x.io","to":"b@x.io","subject":"s","text":"Content-Type: text/plain\r\n\r\nline1\r\nline2"}"#;
        let (line, _) = record_line(text).unwrap();
        assert!(!line.contains('\r'));
    }

    #[test]
    fn record_line_without_message_id_has_empty_slot() {
        let text = r#"{"from":"a@x.io","to":"b@x.io","subject":"s","text":"c"}"#;
        let (line, message_id) = record_line(text).unwrap();
        let parts: Vec<&str> = line.split('\t').collect();
        assert_eq!(parts.len(), 7);
        assert_eq!(parts[6], "");
        assert_eq!(message_id, "");
    }

    #[test]
    fn seen_contains_finds_id_and_misses_absent_file() {
        let path =
            std::env::temp_dir().join(format!("omegaflow_seen_test_{}.φ", std::process::id()));
        let _ = std::fs::write(&path, "<id-1@x.io>\n<id-2@x.io>\n");
        assert!(seen_contains(&path.to_string_lossy(), "<id-1@x.io>"));
        assert!(!seen_contains(&path.to_string_lossy(), "<id-3@x.io>"));
        assert!(!seen_contains(
            "/nonexistent/omegaflow_seen_absent.φ",
            "<id-1@x.io>"
        ));
        let _ = std::fs::remove_file(&path);
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
    fn plaintext_nested_multipart_descends() {
        let raw = "Content-Type: multipart/mixed; boundary=outer\r\n\r\n--outer\r\nContent-Type: multipart/alternative; boundary=inner\r\n\r\n--inner\r\nContent-Type: text/plain; charset=utf-8\r\nContent-Transfer-Encoding: quoted-printable\r\n\r\nGr=C3=BC=C3=9F\r\n--inner\r\nContent-Type: text/html\r\n\r\n<p>Gr&uuml;&szlig;</p>\r\n--inner--\r\n--outer\r\nContent-Type: application/pgp-signature\r\n\r\nsig\r\n--outer--";
        assert_eq!(mime_plaintext(raw), "Gr\u{fc}\u{df}");
    }

    #[test]
    fn quoted_printable_decoded() {
        let raw = "Content-Type: text/plain\r\nContent-Transfer-Encoding: quoted-printable\r\n\r\nGr=C3=BC=C3=9F";
        assert_eq!(mime_plaintext(raw), "Gr\u{fc}\u{df}");
    }

    #[test]
    fn folded_content_type_boundary() {
        let raw = "Content-Type: multipart/alternative;\r\n\tboundary=\"b1=_X\"\r\n\r\n--b1=_X\r\nContent-Type: text/plain\r\n\r\nhello\r\n--b1=_X--";
        assert_eq!(mime_plaintext(raw), "hello");
    }

    #[test]
    fn folded_content_type_boundary_survives_following_header() {
        let raw = "Content-Type: multipart/alternative;\r\n\tboundary=\"b1=_X\"\r\nContent-Transfer-Encoding: 7bit\r\n\r\n--b1=_X\r\nContent-Type: text/plain\r\n\r\nhello\r\n--b1=_X--";
        assert_eq!(mime_plaintext(raw), "hello");
    }
}
