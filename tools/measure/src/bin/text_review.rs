use std::env;
use std::io::Write;
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

const MODELS_TSV: &str = include_str!("../../free_models.tsv");

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Disposition {
    Eligible,
    Blocked,
    Struck,
}

impl Disposition {
    fn code(self) -> &'static str {
        match self {
            Disposition::Eligible => "eligible",
            Disposition::Blocked => "blocked",
            Disposition::Struck => "struck",
        }
    }
}

fn parse_disposition(s: &str) -> Option<Disposition> {
    match s {
        "eligible" => Some(Disposition::Eligible),
        "blocked" => Some(Disposition::Blocked),
        "struck" => Some(Disposition::Struck),
        _ => None,
    }
}

#[derive(Clone)]
struct Model {
    provider: String,
    id: String,
    base: String,
    env_var: String,
    channel: String,
    disposition: Disposition,
}

struct Resp {
    code: Option<u16>,
    body: String,
    ms: u128,
    timed_out: bool,
}

struct Review {
    provider: String,
    model: String,
    status: String,
    ms: u128,
    text: Option<String>,
}

fn json_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 16);
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out
}

fn extract_string_after(hay: &str, key: &str) -> Option<String> {
    let marker = format!("\"{}\"", key);
    let pos = hay.find(&marker)?;
    let rest = &hay[pos + marker.len()..];
    let colon = rest.find(':')?;
    let after = rest[colon + 1..].trim_start();
    if !after.starts_with('"') {
        return None;
    }
    let mut out = String::new();
    let mut chars = after[1..].chars();
    while let Some(c) = chars.next() {
        match c {
            '"' => return Some(out),
            '\\' => {
                if let Some(n) = chars.next() {
                    out.push(n);
                }
            }
            c => out.push(c),
        }
    }
    None
}

fn home() -> String {
    match env::var("HOME") {
        Ok(v) => v,
        Err(_) => String::new(),
    }
}

fn secrets_value(contents: &str, name: &str) -> Option<String> {
    for line in contents.lines() {
        let line = line.trim_end_matches('\r');
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((k, v)) = line.split_once('=') {
            if k == name {
                let v = v.trim();
                if !v.is_empty() {
                    return Some(v.to_string());
                }
            }
        }
    }
    None
}

fn balanced_object(s: &str) -> Option<&str> {
    if !s.starts_with('{') {
        return None;
    }
    let mut depth = 0i32;
    let mut in_str = false;
    let mut escaped = false;
    for (i, c) in s.char_indices() {
        if in_str {
            if escaped {
                escaped = false;
            } else if c == '\\' {
                escaped = true;
            } else if c == '"' {
                in_str = false;
            }
            continue;
        }
        match c {
            '"' => in_str = true,
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(&s[..i + 1]);
                }
            }
            _ => {}
        }
    }
    None
}

fn json_object_after<'a>(hay: &'a str, key: &str) -> Option<&'a str> {
    let marker = format!("\"{}\"", key);
    let mut search = 0;
    while let Some(rel) = hay[search..].find(&marker) {
        let pos = search + rel;
        let rest = &hay[pos + marker.len()..];
        let after = rest.trim_start();
        if let Some(colon) = after.strip_prefix(':') {
            if let Some(block) = balanced_object(colon.trim_start()) {
                return Some(block);
            }
        }
        search = pos + marker.len();
    }
    None
}

fn provider_field(contents: &str, provider: &str, field: &str) -> Option<String> {
    let block = json_object_after(contents, provider)?;
    let value = extract_string_after(block, field)?;
    if value.is_empty() { None } else { Some(value) }
}

fn key_for(m: &Model) -> Option<String> {
    if let Ok(v) = env::var(&m.env_var) {
        if !v.is_empty() {
            return Some(v);
        }
    }
    if let Ok(secrets) = std::fs::read_to_string(".secrets.local") {
        if let Some(v) = secrets_value(&secrets, &m.env_var) {
            return Some(v);
        }
    }
    if let Ok(auth) = std::fs::read_to_string(format!("{}/.local/share/opencode/auth.json", home()))
    {
        if let Some(v) = provider_field(&auth, &m.provider, "key") {
            return Some(v);
        }
    }
    let cfg =
        std::fs::read_to_string(format!("{}/.config/opencode/opencode.jsonc", home())).ok()?;
    provider_field(&cfg, &m.provider, "apiKey")
}

fn call(url: &str, key: &str, body: &str, timeout: u64) -> Resp {
    let start = Instant::now();
    let timeout_s = timeout.to_string();
    let child = Command::new("curl")
        .args([
            "-sS",
            "-m",
            &timeout_s,
            "-w",
            "\n%{http_code}",
            "-H",
            &format!("Authorization: Bearer {}", key),
            "-H",
            "Content-Type: application/json",
            "--data-binary",
            "@-",
            url,
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn();
    let mut child = match child {
        Ok(c) => c,
        Err(_) => {
            return Resp {
                code: None,
                body: String::new(),
                ms: start.elapsed().as_millis(),
                timed_out: false,
            };
        }
    };
    if let Some(mut si) = child.stdin.take() {
        let _ = si.write_all(body.as_bytes());
    }
    let out = match child.wait_with_output() {
        Ok(o) => o,
        Err(_) => {
            return Resp {
                code: None,
                body: String::new(),
                ms: start.elapsed().as_millis(),
                timed_out: false,
            };
        }
    };
    let ms = start.elapsed().as_millis();
    let text = String::from_utf8_lossy(&out.stdout).to_string();
    let (body_text, code) = match text.rfind('\n') {
        Some(i) => (
            text[..i].to_string(),
            text[i + 1..].trim().parse::<u16>().ok(),
        ),
        None => (text, None),
    };
    Resp {
        code,
        body: body_text,
        ms,
        timed_out: out.status.code() == Some(28),
    }
}

fn retry_hint(body: &str) -> u64 {
    if let Some(pos) = body.find("retry in ") {
        let rest = &body[pos + 9..];
        let n: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
        if let Ok(v) = n.parse::<u64>() {
            return v.min(120);
        }
    }
    30
}

fn parse_model(line: &str) -> Option<Model> {
    let mut it = line.split('\t');
    let provider = it.next()?.to_string();
    let id = it.next()?.to_string();
    let base = it.next()?.to_string();
    let env_var = it.next()?.to_string();
    let channel = it.next()?.to_string();
    let disposition = parse_disposition(it.next()?)?;
    if provider.is_empty()
        || id.is_empty()
        || base.is_empty()
        || env_var.is_empty()
        || channel.is_empty()
    {
        return None;
    }
    Some(Model {
        provider,
        id,
        base,
        env_var,
        channel,
        disposition,
    })
}

fn is_http(m: &Model) -> bool {
    m.channel != "client"
}

fn is_eligible(m: &Model) -> bool {
    m.disposition == Disposition::Eligible
}

fn report_excluded(models: &[Model]) {
    for m in models
        .iter()
        .filter(|m| m.disposition != Disposition::Eligible)
    {
        eprintln!("{}\t{}\t{}", m.disposition.code(), m.provider, m.id);
    }
}

fn build_prompt(draft: &str) -> String {
    format!(
        "You are a careful editor reviewing a draft application letter. Critique ONLY: clarity, tone, structure, grammar, and whether the argument is convincing to a reader who does not know the project. Do NOT rewrite the letter. Reply in at most 200 words as a numbered list of concrete issues. When a sentence is unclear or overclaiming, quote it verbatim. If the letter is fine, say so plainly. The draft follows between the markers.\n\n<<<DRAFT\n{}\nDRAFT>>>",
        draft
    )
}

fn build_body(model: &str, prompt: &str, max_tokens: u32) -> String {
    let mut b = String::new();
    b.push_str("{\"model\":\"");
    b.push_str(&json_escape(model));
    b.push_str("\",\"messages\":[{\"role\":\"user\",\"content\":\"");
    b.push_str(&json_escape(prompt));
    b.push_str("\"}],\"max_tokens\":");
    b.push_str(&max_tokens.to_string());
    b.push('}');
    b
}

fn extract_review(body: &str) -> Option<String> {
    let cpos = body.find("\"choices\"")?;
    let rest = &body[cpos..];
    let mpos = rest.find("\"message\"")?;
    let mrest = &rest[mpos..];
    if let Some(text) = extract_string_after(mrest, "content") {
        if !text.is_empty() {
            return Some(text);
        }
    }
    for field in ["reasoning", "reasoning_content"] {
        if let Some(text) = extract_string_after(mrest, field) {
            if !text.is_empty() {
                return Some(format!("[reasoning] {}", text));
            }
        }
    }
    None
}

fn classify(code: Option<u16>, timed_out: bool, body: &str) -> String {
    if timed_out {
        return "timeout".into();
    }
    let c = match code {
        Some(c) => c,
        None => return "timeout".into(),
    };
    if c == 429 {
        return "pending_rate_limited".into();
    }
    if (500..600).contains(&c) {
        return "http_5xx".into();
    }
    if c >= 400 {
        return format!("http_{}", c);
    }
    if extract_review(body).is_some() {
        "ok".into()
    } else {
        "empty".into()
    }
}

fn matches(m: &Model, model_filters: &[String], provider_filters: &[String]) -> bool {
    let m_ok = model_filters.is_empty() || model_filters.iter().any(|f| m.id.contains(f.as_str()));
    let p_ok = provider_filters.is_empty() || provider_filters.iter().any(|f| m.provider == *f);
    m_ok && p_ok
}

fn select(
    models: &[Model],
    model_filters: &[String],
    provider_filters: &[String],
    limit: Option<usize>,
) -> Vec<Model> {
    let mut v: Vec<Model> = models
        .iter()
        .filter(|m| is_eligible(m) && matches(m, model_filters, provider_filters))
        .cloned()
        .collect();
    if let Some(n) = limit {
        v.truncate(n);
    }
    v
}

fn run_model(
    m: &Model,
    key: &str,
    prompt: &str,
    max_tokens: u32,
    timeout: u64,
) -> (String, u128, Option<String>) {
    let url = format!("{}/chat/completions", m.base.trim_end_matches('/'));
    let body = build_body(&m.id, prompt, max_tokens);
    let mut total_ms = 0u128;
    let mut attempt = 0;
    loop {
        attempt += 1;
        let r = call(&url, key, &body, timeout);
        total_ms += r.ms;
        if r.timed_out {
            return ("timeout".into(), total_ms, None);
        }
        let code = match r.code {
            Some(c) => c,
            None => return ("timeout".into(), total_ms, None),
        };
        if code == 429 {
            if attempt >= 3 {
                return (classify(Some(code), false, &r.body), total_ms, None);
            }
            let wait = retry_hint(&r.body);
            thread::sleep(Duration::from_secs(wait));
            continue;
        }
        if (500..600).contains(&code) {
            if attempt >= 2 {
                return (classify(Some(code), false, &r.body), total_ms, None);
            }
            continue;
        }
        if code >= 400 {
            return (classify(Some(code), false, &r.body), total_ms, None);
        }
        let text = extract_review(&r.body);
        let status = if text.is_some() { "ok" } else { "empty" };
        return (status.into(), total_ms, text);
    }
}

fn build_report(draft_path: &str, rows: &[Review], ts: Option<u64>) -> String {
    let mut r = String::new();
    r.push_str(&format!("# text_review: {}\n\n", draft_path));
    r.push_str(&format!("- draft: {}\n", draft_path));
    r.push_str(&format!("- models: {}\n", rows.len()));
    match ts {
        Some(t) => r.push_str(&format!("- measured_at: {}\n\n", t)),
        None => r.push_str("- measured_at: pending\n\n"),
    }
    r.push_str("| provider | model | status | ms | chars |\n");
    r.push_str("|---|---|---|---|---|\n");
    for row in rows {
        let chars = match &row.text {
            Some(t) => t.chars().count().to_string(),
            None => "\u{2014}".to_string(),
        };
        r.push_str(&format!(
            "| {} | {} | {} | {} | {} |\n",
            row.provider, row.model, row.status, row.ms, chars
        ));
    }
    r.push('\n');
    for row in rows {
        match &row.text {
            Some(t) => {
                r.push_str(&format!("## {} {}\n\n{}\n\n", row.provider, row.model, t));
            }
            None => {
                r.push_str(&format!(
                    "- {} {}: {}\n",
                    row.provider, row.model, row.status
                ));
            }
        }
    }
    r
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut draft_path: Option<String> = None;
    let mut model_filters: Vec<String> = Vec::new();
    let mut provider_filters: Vec<String> = Vec::new();
    let mut out: Option<String> = None;
    let mut max_tokens: u32 = 2000;
    let mut timeout: u64 = 60;
    let mut limit: Option<usize> = None;
    let mut dry_run = false;
    let mut list_models = false;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--model" => {
                if i + 1 < args.len() {
                    model_filters.push(args[i + 1].clone());
                    i += 1;
                }
            }
            "--provider" => {
                if i + 1 < args.len() {
                    provider_filters.push(args[i + 1].clone());
                    i += 1;
                }
            }
            "--out" => {
                if i + 1 < args.len() {
                    out = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            "--max-tokens" => {
                if i + 1 < args.len() {
                    if let Ok(v) = args[i + 1].parse::<u32>() {
                        max_tokens = v;
                    }
                    i += 1;
                }
            }
            "--timeout" => {
                if i + 1 < args.len() {
                    if let Ok(v) = args[i + 1].parse::<u64>() {
                        timeout = v;
                    }
                    i += 1;
                }
            }
            "--limit" => {
                if i + 1 < args.len() {
                    if let Ok(v) = args[i + 1].parse::<usize>() {
                        limit = Some(v);
                    }
                    i += 1;
                }
            }
            "--dry-run" => dry_run = true,
            "--list-models" => list_models = true,
            s if !s.starts_with("--") && draft_path.is_none() => {
                draft_path = Some(s.to_string());
            }
            _ => {}
        }
        i += 1;
    }

    let all_models: Vec<Model> = MODELS_TSV
        .lines()
        .filter_map(parse_model)
        .filter(is_http)
        .collect();

    if list_models {
        for m in &all_models {
            println!("{}\t{}\t{}", m.provider, m.id, m.disposition.code());
        }
        return;
    }

    report_excluded(&all_models);

    let path = match draft_path {
        Some(p) => p,
        None => {
            eprintln!("text_review: a draft file argument is required");
            std::process::exit(2);
        }
    };

    let draft = match std::fs::read_to_string(&path) {
        Ok(t) => t,
        Err(_) => {
            eprintln!("text_review: draft absent or unreadable: {}", path);
            std::process::exit(2);
        }
    };

    let selected = select(&all_models, &model_filters, &provider_filters, limit);

    if dry_run {
        for m in &selected {
            println!("{}\t{}", m.provider, m.id);
        }
        return;
    }

    let out_path = match out {
        Some(o) => o,
        None => format!("{}.review.md", path),
    };
    let prompt = build_prompt(&draft);
    let mut rows: Vec<Review> = Vec::new();

    for m in &selected {
        let key = match key_for(m) {
            Some(k) => k,
            None => {
                rows.push(Review {
                    provider: m.provider.clone(),
                    model: m.id.clone(),
                    status: "pending_no_key".into(),
                    ms: 0,
                    text: None,
                });
                continue;
            }
        };
        eprintln!("== {} {} ==", m.provider, m.id);
        let (status, ms, text) = run_model(m, &key, &prompt, max_tokens, timeout);
        eprintln!("  {}", status);
        rows.push(Review {
            provider: m.provider.clone(),
            model: m.id.clone(),
            status,
            ms,
            text,
        });
    }

    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .ok();
    let report = build_report(&path, &rows, ts);
    if std::fs::write(&out_path, report).is_err() {
        eprintln!("text_review: report not writable: {}", out_path);
        std::process::exit(2);
    }

    let ok = rows.iter().filter(|r| r.status == "ok").count();
    let pending = rows
        .iter()
        .filter(|r| r.status.starts_with("pending"))
        .count();
    let http = rows.iter().filter(|r| r.status.starts_with("http")).count();
    let timeout_n = rows.iter().filter(|r| r.status == "timeout").count();
    println!(
        "text_review: {}/{} ok, {} pending, {} http, {} timeout — report {}",
        ok,
        rows.len(),
        pending,
        http,
        timeout_n,
        out_path
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prompt_carries_markers_and_draft() {
        let p = build_prompt("Dear committee, I apply.");
        assert!(p.contains("<<<DRAFT"));
        assert!(p.contains("DRAFT>>>"));
        assert!(p.contains("Dear committee, I apply."));
        assert!(p.contains("numbered list"));
    }

    #[test]
    fn parse_model_splits_tsv_line() {
        let m = match parse_model(
            "zai\tglm-4.7-flash\thttps://api.z.ai/api/paas/v4\tZAI_API_KEY\thttp\teligible",
        ) {
            Some(m) => m,
            None => panic!("tsv line carries no model"),
        };
        assert_eq!(m.provider, "zai");
        assert_eq!(m.id, "glm-4.7-flash");
        assert_eq!(m.base, "https://api.z.ai/api/paas/v4");
        assert_eq!(m.env_var, "ZAI_API_KEY");
        assert_eq!(m.channel, "http");
        assert_eq!(m.disposition, Disposition::Eligible);
        assert!(parse_model("bad\tline").is_none());
    }

    #[test]
    fn http_path_skips_client_channel_rows() {
        let client = match parse_model(
            "opencode\tbig-pickle\thttps://opencode.ai/zen/v1\tOPENCODE_API_KEY\tclient\teligible",
        ) {
            Some(m) => m,
            None => panic!("client tsv line carries no model"),
        };
        let http = match parse_model(
            "zai\tglm-5.3-flash\thttps://api.z.ai/api/paas/v4\tZAI_API_KEY\thttp\teligible",
        ) {
            Some(m) => m,
            None => panic!("http tsv line carries no model"),
        };
        assert!(!is_http(&client));
        assert!(is_http(&http));
    }

    #[test]
    fn disposition_gates_eligibility_without_a_default() {
        assert_eq!(parse_disposition("eligible"), Some(Disposition::Eligible));
        assert_eq!(parse_disposition("blocked"), Some(Disposition::Blocked));
        assert_eq!(parse_disposition("struck"), Some(Disposition::Struck));
        assert_eq!(parse_disposition(""), None);
        assert_eq!(parse_disposition("unknown"), None);
        assert!(
            parse_model("zai\tglm-5.3-flash\thttps://api.z.ai/api/paas/v4\tZAI_API_KEY\thttp")
                .is_none()
        );
        let blocked = match parse_model(
            "zai\tglm-5.3-flash\thttps://api.z.ai/api/paas/v4\tZAI_API_KEY\thttp\tblocked",
        ) {
            Some(m) => m,
            None => panic!("blocked tsv line carries no model"),
        };
        assert!(!is_eligible(&blocked));
    }

    #[test]
    fn model_filter_matches_substring() {
        let m = Model {
            provider: "kilo".into(),
            id: "nvidia/nemotron-3-nano:free".into(),
            base: "b".into(),
            env_var: "K".into(),
            channel: "http".into(),
            disposition: Disposition::Eligible,
        };
        assert!(matches(&m, &["nemotron".to_string()], &[]));
        assert!(!matches(&m, &["gemini".to_string()], &[]));
        assert!(matches(&m, &[], &[]));
    }

    #[test]
    fn provider_filter_selects_exact() {
        let m = Model {
            provider: "groq".into(),
            id: "openai/gpt-oss-120b".into(),
            base: "b".into(),
            env_var: "K".into(),
            channel: "http".into(),
            disposition: Disposition::Eligible,
        };
        assert!(matches(&m, &[], &["groq".to_string()]));
        assert!(!matches(&m, &[], &["kilo".to_string()]));
    }

    #[test]
    fn extract_review_reads_choices_message_content() {
        let body = r#"{"choices":[{"message":{"content":"1. Quote it."}}]}"#;
        assert_eq!(extract_review(body).as_deref(), Some("1. Quote it."));
        assert!(extract_review("{}").is_none());
        assert!(extract_review(r#"{"choices":[{"message":{"content":""}}]}"#).is_none());
    }

    #[test]
    fn secrets_value_matches_exact_env_name() {
        let secrets = "CLOUDFLARE_WORKERS_TOKEN=tok-abc\nOPENCODE_API_KEY=\nGOOGLE_API_KEY=g-key\n";
        assert_eq!(
            secrets_value(secrets, "CLOUDFLARE_WORKERS_TOKEN").as_deref(),
            Some("tok-abc")
        );
        assert_eq!(
            secrets_value(secrets, "GOOGLE_API_KEY").as_deref(),
            Some("g-key")
        );
        assert_eq!(secrets_value(secrets, "OPENCODE_API_KEY"), None);
        assert_eq!(secrets_value(secrets, "CLOUDFLARE_WORKERS"), None);
    }

    #[test]
    fn provider_field_matches_exact_provider_block() {
        let cfg = r#"{
  "provider": {
    "google": { "options": { "apiKey": "google-key" } },
    "opencode": { "whitelist": ["big-pickle"] },
    "cloudflare-workers-ai": { "options": { "apiKey": "cf-key" } }
  }
}"#;
        assert_eq!(
            provider_field(cfg, "google", "apiKey").as_deref(),
            Some("google-key")
        );
        assert_eq!(
            provider_field(cfg, "cloudflare-workers-ai", "apiKey").as_deref(),
            Some("cf-key")
        );
        assert_eq!(provider_field(cfg, "opencode", "apiKey"), None);
        assert_eq!(provider_field(cfg, "absent", "apiKey"), None);
    }

    #[test]
    fn provider_field_reads_auth_key_block() {
        let auth =
            r#"{"kilo":{"type":"api","key":"kilo-key"},"zai":{"type":"api","key":"zai-key"}}"#;
        assert_eq!(
            provider_field(auth, "kilo", "key").as_deref(),
            Some("kilo-key")
        );
        assert_eq!(
            provider_field(auth, "zai", "key").as_deref(),
            Some("zai-key")
        );
        assert_eq!(provider_field(auth, "opencode", "key"), None);
    }

    #[test]
    fn extract_review_falls_back_to_reasoning() {
        let body = r#"{"choices":[{"message":{"content":"","reasoning":"check the claim"},"finish_reason":"length"}]}"#;
        match extract_review(body) {
            Some(t) => {
                assert!(t.starts_with("[reasoning]"));
                assert!(t.contains("check the claim"));
            }
            None => panic!("reasoning fallback absent"),
        }
        assert_eq!(classify(Some(200), false, body), "ok");
        let body2 =
            r#"{"choices":[{"message":{"content":"","reasoning_content":"deep thought"}}]}"#;
        match extract_review(body2) {
            Some(t) => assert!(t.contains("deep thought")),
            None => panic!("reasoning_content fallback absent"),
        }
        assert!(extract_review(r#"{"choices":[{"message":{"content":""}}]}"#).is_none());
    }

    #[test]
    fn classify_terminal_status() {
        let ok_body = r#"{"choices":[{"message":{"content":"fine"}}]}"#;
        assert_eq!(classify(Some(200), false, ok_body), "ok");
        assert_eq!(classify(Some(200), false, "{}"), "empty");
        assert_eq!(classify(Some(429), false, "{}"), "pending_rate_limited");
        assert_eq!(classify(Some(503), false, "{}"), "http_5xx");
        assert_eq!(classify(Some(404), false, "{}"), "http_404");
        assert_eq!(classify(None, false, "{}"), "timeout");
    }
}
