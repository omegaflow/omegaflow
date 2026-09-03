use std::fs;
use std::io::Read;
use std::process::Command;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut to: Option<String> = None;
    let mut from: Option<String> = None;
    let mut subject: Option<String> = None;
    let mut body: Option<String> = None;
    let mut html: Option<String> = None;
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
            _ => {}
        }
        i += 1;
    }
    let Some(to) = to else {
        eprintln!(
            "usage: smail --to <addr> [--from <addr>] --subject <s> [--body <file>] [--html <file>]"
        );
        std::process::exit(2);
    };
    let from = from.unwrap_or_else(|| String::from("code@omegaflow.space"));
    let subject = subject.unwrap_or_default();
    let text = match body.as_deref() {
        Some("-") | None => read_stdin(),
        Some(path) => fs::read_to_string(path).unwrap_or_default(),
    };
    let html_body = match html.as_deref() {
        Some(path) => Some(fs::read_to_string(path).unwrap_or_default()),
        None => None,
    };
    let token = match secret_key("RESEND_API_KEY") {
        Some(t) => t,
        None => {
            eprintln!("smail: RESEND_API_KEY absent (.secrets.local or env)");
            std::process::exit(1);
        }
    };
    let payload = build_payload(&to, &from, &subject, &text, html_body.as_deref());
    let resp = send(&token, &payload);
    println!("{}", resp);
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
}
