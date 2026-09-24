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
    let mut cc: Vec<String> = Vec::new();
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
            "--cc" => {
                i += 1;
                if i < args.len() {
                    cc.push(args[i].clone());
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
            "usage: smail --to <addr> [--from <addr>] --subject <s> [--body <file>] [--html <file>] [--cc <addr>]... [--send] [--dry-run]"
        );
        eprintln!(
            "  default is dry-run (prints what would be sent); --send is the operator-consented act; --dry-run forces it; --cc is repeatable (one address per flag)"
        );
        std::process::exit(2);
    };
    let from = match from {
        Some(f) => f,
        None => String::from("code@omegaflow.space"),
    };
    let Some(subject) = subject else {
        eprintln!(
            "usage: smail --to <addr> [--from <addr>] --subject <s> [--body <file>] [--html <file>] [--cc <addr>]... [--send] [--dry-run]"
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
    let will_send = sends(send_now, dry_run);
    let verdict = gate(&text);
    if !will_send || verdict.refuse.is_some() {
        print_table(&verdict.rows);
    }
    if let Some(reason) = verdict.refuse {
        eprintln!("{}", reason);
        std::process::exit(2);
    }
    let payload = build_payload(&to, &from, &subject, &text, html_body.as_deref(), &cc);
    if !will_send {
        println!("dry-run — nothing sent (add --send to send)");
        println!("to: {}", to);
        println!("from: {}", from);
        println!("subject: {}", subject);
        println!("cc: {}", cc.join(", "));
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

fn sent_line(
    ts: u64,
    from: &str,
    to: &str,
    subject: &str,
    id: Option<&str>,
    bytes: usize,
) -> String {
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

fn build_payload(
    to: &str,
    from: &str,
    subject: &str,
    text: &str,
    html: Option<&str>,
    cc: &[String],
) -> String {
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
    if !cc.is_empty() {
        let joined: Vec<String> = cc
            .iter()
            .map(|a| format!("\"{}\"", json_escape(a)))
            .collect();
        p.push_str(&format!(",\"cc\":[{}]", joined.join(",")));
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

struct Claim {
    claim: String,
    source: String,
    resolves: bool,
}

enum Source {
    FileLine { file: String, line: u64 },
    RegisterKey { register: String, key: String },
    CommandArtifact { artifact: String },
}

enum Quellen {
    NoClaims,
    Claims(Vec<String>),
}

struct Verdict {
    rows: Vec<Claim>,
    refuse: Option<String>,
}

fn gate(body: &str) -> Verdict {
    match find_quellen(body) {
        None => Verdict {
            rows: vec![Claim {
                claim: "(no QUELLEN block)".to_string(),
                source: "absent".to_string(),
                resolves: false,
            }],
            refuse: Some(String::from(
                "smail: no QUELLEN block in draft — a state claim needs a measured source \
                 (claim → file:line | register#key | command@timestamp → artifact); \
                 a mail with no state claims carries QUELLEN: none",
            )),
        },
        Some(Quellen::NoClaims) => Verdict {
            rows: vec![Claim {
                claim: "(no state claims)".to_string(),
                source: "QUELLEN: none".to_string(),
                resolves: true,
            }],
            refuse: None,
        },
        Some(Quellen::Claims(lines)) => {
            let rows: Vec<Claim> = lines.iter().map(|l| resolve_claim(l)).collect();
            let unresolved: Vec<&str> = rows
                .iter()
                .filter(|c| !c.resolves)
                .map(|c| c.source.as_str())
                .collect();
            let refuse = if unresolved.is_empty() {
                None
            } else {
                Some(format!(
                    "smail: unresolved QUELLEN source: {} — measure before the word, \
                     or write QUELLEN: none when no state claim stands",
                    unresolved.join(", ")
                ))
            };
            Verdict { rows, refuse }
        }
    }
}

fn find_quellen(body: &str) -> Option<Quellen> {
    let mut lines = body.lines();
    let mut rest: Option<&str> = None;
    for line in lines.by_ref() {
        if let Some(r) = line.trim_start().strip_prefix("QUELLEN:") {
            rest = Some(r.trim());
            break;
        }
    }
    let rest = rest?;
    if rest == "none" {
        return Some(Quellen::NoClaims);
    }
    let mut claims: Vec<String> = Vec::new();
    if !rest.is_empty() {
        claims.push(rest.to_string());
    }
    for line in lines {
        let t = line.trim();
        if t.is_empty() {
            break;
        }
        claims.push(t.to_string());
    }
    Some(Quellen::Claims(claims))
}

fn resolve_claim(line: &str) -> Claim {
    match line.split_once('→') {
        Some((claim, source)) => {
            let source = source.trim().to_string();
            let resolves = match parse_source(&source) {
                Some(s) => resolve_source(&s),
                None => false,
            };
            Claim {
                claim: claim.trim().to_string(),
                source,
                resolves,
            }
        }
        None => Claim {
            claim: line.trim().to_string(),
            source: String::new(),
            resolves: false,
        },
    }
}

fn parse_source(source: &str) -> Option<Source> {
    if let Some((cmd_ts, artifact)) = source.split_once('→') {
        let cmd_ts = cmd_ts.trim();
        let artifact = artifact.trim();
        if cmd_ts.contains('@') && !artifact.is_empty() {
            return Some(Source::CommandArtifact {
                artifact: artifact.to_string(),
            });
        }
        return None;
    }
    if let Some((register, key)) = source.split_once('#') {
        let register = register.trim();
        let key = key.trim();
        if !register.is_empty() && !key.is_empty() {
            return Some(Source::RegisterKey {
                register: register.to_string(),
                key: key.to_string(),
            });
        }
        return None;
    }
    if let Some((file, line)) = source.rsplit_once(':') {
        let file = file.trim();
        let line = line.trim();
        if !file.is_empty() && !line.is_empty() && line.chars().all(|c| c.is_ascii_digit()) {
            if let Ok(n) = line.parse::<u64>() {
                if n >= 1 {
                    return Some(Source::FileLine {
                        file: file.to_string(),
                        line: n,
                    });
                }
            }
        }
        return None;
    }
    None
}

fn resolve_source(source: &Source) -> bool {
    match source {
        Source::FileLine { file, line } => match count_lines(file) {
            Some(n) => n as u64 >= *line,
            None => false,
        },
        Source::RegisterKey { register, key } => match fs::read_to_string(register) {
            Ok(content) => content.contains(key.as_str()),
            Err(_) => false,
        },
        Source::CommandArtifact { artifact } => std::path::Path::new(artifact).exists(),
    }
}

fn count_lines(path: &str) -> Option<usize> {
    let bytes = fs::read(path).ok()?;
    if bytes.is_empty() {
        return Some(0);
    }
    let newlines = bytes.iter().filter(|&&b| b == b'\n').count();
    Some(newlines + usize::from(bytes.last() != Some(&b'\n')))
}

fn print_table(rows: &[Claim]) {
    println!("claim | source | resolves");
    for r in rows {
        println!("{} | {} | {}", r.claim, r.source, r.resolves);
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
        let p = build_payload("to@x.io", "from@x.io", "hi", "body", None, &[]);
        assert!(p.contains("\"to\":\"to@x.io\""));
        assert!(p.contains("\"from\":\"from@x.io\""));
        assert!(p.contains("\"subject\":\"hi\""));
        assert!(p.contains("\"text\":\"body\""));
        assert!(!p.contains("html"));
    }

    #[test]
    fn payload_with_html() {
        let p = build_payload("a", "b", "c", "d", Some("<p>hi</p>"), &[]);
        assert!(p.contains("\"html\":\"<p>hi</p>\""));
    }

    #[test]
    fn payload_with_cc_carries_the_addresses() {
        let cc = vec!["c1@x.io".to_string(), "c2@x.io".to_string()];
        let p = build_payload("a", "b", "c", "d", None, &cc);
        assert!(p.contains("\"cc\":[\"c1@x.io\",\"c2@x.io\"]"));
    }

    #[test]
    fn payload_without_cc_omits_the_key() {
        let p = build_payload("a", "b", "c", "d", None, &[]);
        assert!(!p.contains("\"cc\""));
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
        assert_eq!(
            extract_id("{\"id\":\"abc-123\"}"),
            Some("abc-123".to_string())
        );
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

    #[test]
    fn gate_refuses_a_draft_without_a_quellen_block() {
        let v = gate("subject line\nbody text\n");
        assert!(v.refuse.is_some());
        assert_eq!(v.rows.len(), 1);
        assert!(!v.rows[0].resolves);
    }

    #[test]
    fn gate_passes_quellen_none() {
        let v = gate("draft\nQUELLEN: none\n");
        assert!(v.refuse.is_none());
        assert!(v.rows.iter().all(|r| r.resolves));
    }

    #[test]
    fn gate_refuses_an_unresolved_file_line_claim() {
        let v = gate("QUELLEN:\nThe membrane runs → /no/such/file.rs:1\n");
        assert!(v.refuse.is_some());
        assert_eq!(v.rows.len(), 1);
        assert_eq!(v.rows[0].source, "/no/such/file.rs:1");
        assert!(!v.rows[0].resolves);
    }

    #[test]
    fn parse_source_covers_all_three_forms() {
        assert!(matches!(
            parse_source("src/bin/smail.rs:42"),
            Some(Source::FileLine { .. })
        ));
        assert!(matches!(
            parse_source("phi/sources.φ#vires"),
            Some(Source::RegisterKey { .. })
        ));
        assert!(matches!(
            parse_source("curl -s -o out.bin https://x @ 1700000000 → /tmp/out.bin"),
            Some(Source::CommandArtifact { .. })
        ));
    }

    #[test]
    fn file_line_source_resolves_when_the_line_exists() {
        let resolves = match parse_source("src/bin/smail.rs:1") {
            Some(s) => resolve_source(&s),
            None => false,
        };
        assert!(resolves);
    }

    #[test]
    fn file_line_source_does_not_resolve_past_the_last_line() {
        let resolves = match parse_source("src/bin/smail.rs:999999") {
            Some(s) => resolve_source(&s),
            None => true,
        };
        assert!(!resolves);
    }
}
