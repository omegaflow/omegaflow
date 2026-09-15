use std::fs;
use std::io::Read;
use std::process::Command;

use omegaflow::json::{JsonVal, parse_json};

const NIM_URL: &str = "https://integrate.api.nvidia.com/v1/chat/completions";
const DEFAULT_MODEL: &str = "nvidia/riva-translate-4b-instruct-v2";

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut from: Option<String> = None;
    let mut to: Option<String> = None;
    let mut model = DEFAULT_MODEL.to_string();
    let mut input: Option<String> = None;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--from" => {
                i += 1;
                if i < args.len() {
                    from = Some(args[i].clone());
                }
            }
            "--to" => {
                i += 1;
                if i < args.len() {
                    to = Some(args[i].clone());
                }
            }
            "--model" => {
                i += 1;
                if i < args.len() {
                    model = args[i].clone();
                }
            }
            "--in" => {
                i += 1;
                if i < args.len() {
                    input = Some(args[i].clone());
                }
            }
            _ => {}
        }
        i += 1;
    }
    let (Some(from), Some(to)) = (from, to) else {
        eprintln!("usage: riva-translate --from <lang> --to <lang> [--model <id>] [--in <file>|-]");
        std::process::exit(2);
    };
    let text = match input.as_deref() {
        Some("-") | None => match read_stdin() {
            Some(t) => t,
            None => {
                eprintln!("riva-translate: stdin absent");
                std::process::exit(2);
            }
        },
        Some(path) => match fs::read_to_string(path) {
            Ok(t) => t,
            Err(_) => {
                eprintln!("riva-translate: input file absent: {}", path);
                std::process::exit(2);
            }
        },
    };
    let token = match secret_key("NVIDIA_API_KEY") {
        Some(t) => t,
        None => {
            eprintln!("riva-translate: NVIDIA_API_KEY absent (.secrets.local or env)");
            std::process::exit(2);
        }
    };
    let payload = build_payload(&model, &pair_tag(&from, &to), &text);
    let Some(resp) = send(&token, &payload) else {
        eprintln!("riva-translate: curl no response");
        std::process::exit(2);
    };
    match translation(&resp) {
        Some(t) => println!("{}", t),
        None => {
            eprintln!("riva-translate: no translation in response: {}", resp);
            std::process::exit(2);
        }
    }
}

fn pair_tag(from: &str, to: &str) -> String {
    format!("{}-{}", from, to)
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

fn read_stdin() -> Option<String> {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).ok()?;
    Some(buf)
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

fn build_payload(model: &str, pair: &str, text: &str) -> String {
    format!(
        "{{\"model\":\"{}\",\"messages\":[{{\"role\":\"system\",\"content\":\"{}\"}},{{\"role\":\"user\",\"content\":\"{}\"}}],\"temperature\":0}}",
        json_escape(model),
        json_escape(pair),
        json_escape(text)
    )
}

fn translation(resp: &str) -> Option<String> {
    let parsed = parse_json(resp)?;
    let JsonVal::Obj(map) = parsed else {
        return None;
    };
    let JsonVal::Arr(choices) = map.get("choices")? else {
        return None;
    };
    let JsonVal::Obj(first) = choices.first()? else {
        return None;
    };
    let JsonVal::Obj(message) = first.get("message")? else {
        return None;
    };
    match message.get("content")? {
        JsonVal::Str(s) if !s.is_empty() => Some(s.clone()),
        _ => None,
    }
}

fn send(token: &str, payload: &str) -> Option<String> {
    let auth = format!("Authorization: Bearer {}", token);
    let output = Command::new("curl")
        .arg("-s")
        .arg("-X")
        .arg("POST")
        .arg(NIM_URL)
        .arg("-H")
        .arg(&auth)
        .arg("-H")
        .arg("Content-Type: application/json")
        .arg("-d")
        .arg(payload)
        .output()
        .ok()?;
    Some(String::from_utf8_lossy(&output.stdout).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pair_tag_joins_languages() {
        assert_eq!(pair_tag("de", "en"), "de-en");
        assert_eq!(pair_tag("zh", "en"), "zh-en");
    }

    #[test]
    fn payload_carries_model_pair_and_text() {
        let p = build_payload(
            "nvidia/riva-translate-4b-instruct-v2",
            "de-en",
            "Hallo Welt",
        );
        assert!(p.contains("\"model\":\"nvidia/riva-translate-4b-instruct-v2\""));
        assert!(p.contains("\"role\":\"system\",\"content\":\"de-en\""));
        assert!(p.contains("\"role\":\"user\",\"content\":\"Hallo Welt\""));
    }

    #[test]
    fn payload_escapes_quotes_and_newlines() {
        let p = build_payload("m", "de-en", "a\"b\nc");
        assert!(p.contains("a\\\"b\\nc"));
    }

    #[test]
    fn translation_reads_first_choice_content() {
        let resp = r#"{"choices":[{"message":{"role":"assistant","content":"Hello world"},"finish_reason":"stop"}]}"#;
        assert_eq!(translation(resp).as_deref(), Some("Hello world"));
    }

    #[test]
    fn translation_absent_without_choices() {
        assert!(translation(r#"{"error":"x"}"#).is_none());
        assert!(translation("not json").is_none());
    }

    #[test]
    fn translation_absent_on_empty_content() {
        let resp = r#"{"choices":[{"message":{"content":""},"finish_reason":"length"}]}"#;
        assert!(translation(resp).is_none());
    }
}
