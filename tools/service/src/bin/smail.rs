use std::fs;
use std::io::{Read, Write};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut to: Option<String> = None;
    let mut from: Option<String> = None;
    let mut subject: Option<String> = None;
    let mut body: Option<String> = None;
    let mut html: Option<String> = None;
    let mut send_now = false;
    let mut dry_run = false;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--to" => {
                i += 1;
                if i < args.len() {
                    to = Some(args[i].clone());
                }
            }
            "--from" => {
                i += 1;
                if i < args.len() {
                    from = Some(args[i].clone());
                }
            }
            "--subject" => {
                i += 1;
                if i < args.len() {
                    subject = Some(args[i].clone());
                }
            }
            "--body" => {
                i += 1;
                if i < args.len() {
                    body = Some(args[i].clone());
                }
            }
            "--html" => {
                i += 1;
                if i < args.len() {
                    html = Some(args[i].clone());
                }
            }
            "--send" => {
                send_now = true;
            }
            "--dry-run" => {
                dry_run = true;
            }
            _ => {}
        }
        i += 1;
    }
    let Some(to) = to else {
        eprintln!(
            "usage: smail --to <addr> [--from <addr>] --subject <s> [--body <file>] [--html <file>] [--send] [--dry-run]"
        );
        eprintln!(
            "  default is dry-run (prints what would be sent); --send is the operator-consented act; --dry-run forces it"
        );
        std::process::exit(2);
    };
    let from = match from {
        Some(f) => f,
        None => String::from("code@omegaflow.space"),
    };
    let Some(subject) = subject else {
        eprintln!(
            "usage: smail --to <addr> [--from <addr>] --subject <s> [--body <file>] [--html <file>] [--send] [--dry-run]"
        );
        std::process::exit(2);
    };
    let text = match body.as_deref() {
        Some("-") | None => read_stdin(),
        Some(path) => match fs::read_to_string(path) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("smail: body {} unreadable: {}", path, e);
                std::process::exit(2);
            }
        },
    };
    let html_body = match html.as_deref() {
        Some(path) => match fs::read_to_string(path) {
            Ok(t) => Some(t),
            Err(e) => {
                eprintln!("smail: html {} unreadable: {}", path, e);
                std::process::exit(2);
            }
        },
        None => None,
    };
    let payload = build_payload(&to, &from, &subject, &text, html_body.as_deref());
    if !sends(send_now, dry_run) {
        println!("dry-run — nothing sent (add --send to send)");
        println!("to: {}", to);
        println!("from: {}", from);
        println!("subject: {}", subject);
        println!("text bytes: {}", text.len());
        println!("payload: {}", payload);
        return;
    }
    let token = match secret_key("RESEND_API_KEY") {
        Some(t) => t,
        None => {
            eprintln!("smail: RESEND_API_KEY absent (.secrets.local or env)");
            std::process::exit(1);
        }
    };
    let resp = send(&token, &payload);
    record_sent(&from, &to, &subject, &resp, text.len());
    println!("{}", resp);
}

fn state_dir() -> std::path::PathBuf {
    if let Ok(dir) = std::env::var("OMEGAFLOW_STATE") {
        return std::path::PathBuf::from(dir);
    }
    std::path::PathBuf::from("state")
}

fn extract_id(resp: &str) -> Option<String> {
    let key = "\"id\":\"";
    let start = resp.find(key)? + key.len();
    let rest = &resp[start..];
    let end = rest.find('"')?;
    Some(rest[..end].to_string())
}

fn sent_line(ts: u64, from: &str, to: &str, subject: &str, id: Option<&str>, bytes: usize) -> String {
    let subject_clean = subject
        .replace('\r', "")
        .replace('\t', " ")
        .replace('\n', " ");
    let id_field = match id {
        Some(v) => v,
        None => "",
    };
    format!(
        "sent\t{}\t{}\t{}\t{}\t{}\t{}\n",
        ts, from, to, subject_clean, id_field, bytes
    )
}

fn record_sent(from: &str, to: &str, subject: &str, resp: &str, bytes: usize) {
    let Ok(since) = SystemTime::now().duration_since(UNIX_EPOCH) else {
        return;
    };
    let id = extract_id(resp);
    let line = sent_line(since.as_secs(), from, to, subject, id.as_deref(), bytes);
    let path = state_dir().join("mail/sent_ledger.φ");
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(&path) {
        let _ = f.write_all(line.as_bytes());
    }
}

fn sends(send_now: bool, dry_run: bool) -> bool {
    send_now && !dry_run
}

fn secret_key(key: &str) -> Option<String> {
    if let Ok(v) = std::env::var(key) {
        if !v.is_empty() {
            return Some(v);
        }
    }
    let body = std::fs::read_to_string(".secrets.local").ok()?;
    for line in body.lines() {
        if let Some((k, v)) = line.split_once('=') {
            if k.trim() == key && !v.trim().is_empty() {
                return Some(v.trim().to_string());
            }
        }
    }
    None
}

fn read_stdin() -> String {
    let mut buf = String::new();
    match std::io::stdin().read_to_string(&mut buf) {
        Ok(_) => buf,
        Err(_) => String::new(),
    }
}

fn json_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c => out.push(c),
        }
    }
    out
}

fn build_payload(to: &str, from: &str, subject: &str, text: &str, html: Option<&str>) -> String {
    let mut p = format!(
        "{{\"to\":\"{}\",\"from\":\"{}\",\"subject\":\"{}\",\"text\":\"{}\"",
        json_escape(to),
        json_escape(from),
        json_escape(subject),
        json_escape(text)
    );
    if let Some(h) = html {
        p.push_str(&format!(",\"html\":\"{}\"", json_escape(h)));
    }
    p.push('}');
    p
}

fn send(token: &str, payload: &str) -> String {
    let url = "https://api.resend.com/emails";
    let auth = format!("Authorization: Bearer {}", token);
    match Command::new("curl")
        .arg("-s")
        .arg("-X")
        .arg("POST")
        .arg(url)
        .arg("-H")
        .arg(&auth)
        .arg("-H")
        .arg("Content-Type: application/json")
        .arg("-d")
        .arg(payload)
        .output()
    {
        Ok(o) => String::from_utf8_lossy(&o.stdout).to_string(),
        Err(_) => String::from("smail: curl no response"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escape_quotes_and_newlines() {
        assert_eq!(json_escape("a\"b\nc"), "a\\\"b\\nc");
    }

    #[test]
    fn payload_roundtrips_fields() {
        let p = build_payload("to@x.io", "from@x.io", "hi", "body", None);
        assert!(p.contains("\"to\":\"to@x.io\""));
        assert!(p.contains("\"from\":\"from@x.io\""));
        assert!(p.contains("\"subject\":\"hi\""));
        assert!(p.contains("\"text\":\"body\""));
        assert!(!p.contains("html"));
    }

    #[test]
    fn payload_with_html() {
        let p = build_payload("a", "b", "c", "d", Some("<p>hi</p>"));
        assert!(p.contains("\"html\":\"<p>hi</p>\""));
    }

    #[test]
    fn sending_needs_the_explicit_send_flag() {
        assert!(!sends(false, false));
        assert!(sends(true, false));
        assert!(!sends(true, true));
        assert!(!sends(false, true));
    }

    #[test]
    fn extract_id_reads_resend_response() {
        assert_eq!(extract_id("{\"id\":\"abc-123\"}"), Some("abc-123".to_string()));
        assert_eq!(extract_id("{\"statusCode\":403}"), None);
    }

    #[test]
    fn sent_line_has_seven_fields() {
        let l = sent_line(
            42,
            "code@omegaflow.space",
            "a@b.io",
            "hi\tthere",
            Some("id1"),
            7,
        );
        let parts: Vec<&str> = l.trim_end().split('\t').collect();
        assert_eq!(parts.len(), 7);
        assert_eq!(parts[0], "sent");
        assert_eq!(parts[1], "42");
        assert_eq!(parts[2], "code@omegaflow.space");
        assert_eq!(parts[3], "a@b.io");
        assert_eq!(parts[4], "hi there");
        assert_eq!(parts[5], "id1");
        assert_eq!(parts[6], "7");
    }

    #[test]
    fn sent_line_without_id_keeps_the_field_absent() {
        let l = sent_line(1, "a@x.io", "b@x.io", "s", None, 0);
        let parts: Vec<&str> = l.trim_end().split('\t').collect();
        assert_eq!(parts.len(), 7);
        assert_eq!(parts[5], "");
    }
}
