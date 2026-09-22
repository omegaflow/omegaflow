use std::collections::HashSet;
use std::env;
use std::io::Write;
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

const MODELS_TSV: &str = include_str!("../../free_models.tsv");

const TERMS_TXT: &str = include_str!("../../text_probe_terms.txt");

const EN_FUNCTION_WORDS: &[&str] = &[
    "the", "and", "of", "to", "in", "is", "was", "that", "for", "with", "on", "at", "by", "from",
    "it", "not", "are", "as", "this", "be", "been", "will", "would", "can", "there", "has", "have",
    "which", "but", "about",
];

const FORBIDDEN_TERMS: &[&str] = &[
    "failed", "error", "cannot", "crash", "secret", "fallback", "expected", "must", "should",
    "default",
];

struct Task {
    name: String,
    prompt: String,
}

struct Terms {
    de_function: Vec<String>,
    speculation: Vec<String>,
    slop: Vec<String>,
    slop_pairs: Vec<(String, String)>,
    tasks: Vec<Task>,
    judge_prompt: String,
}

fn load_terms() -> Terms {
    let mut terms = Terms {
        de_function: Vec::new(),
        speculation: Vec::new(),
        slop: Vec::new(),
        slop_pairs: Vec::new(),
        tasks: Vec::new(),
        judge_prompt: String::new(),
    };
    let mut section = String::new();
    for raw in TERMS_TXT.lines() {
        let line = raw.trim_end_matches('\r');
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if let Some(rest) = trimmed.strip_prefix("[task]") {
            terms.tasks.push(Task {
                name: rest.trim().to_string(),
                prompt: String::new(),
            });
            section = "task".to_string();
            continue;
        }
        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            section = trimmed[1..trimmed.len() - 1].to_string();
            continue;
        }
        match section.as_str() {
            "de_function" => terms.de_function.push(trimmed.to_string()),
            "speculation" => terms.speculation.push(trimmed.to_string()),
            "slop" => terms.slop.push(trimmed.to_string()),
            "slop_pair" => {
                if let Some((a, b)) = trimmed.split_once('\t') {
                    terms.slop_pairs.push((a.to_string(), b.to_string()));
                }
            }
            "task" => {
                if let Some(t) = terms.tasks.last_mut() {
                    if t.prompt.is_empty() {
                        t.prompt = trimmed.to_string();
                    } else {
                        t.prompt.push(' ');
                        t.prompt.push_str(trimmed);
                    }
                }
            }
            "judge" => {
                if !terms.judge_prompt.is_empty() {
                    terms.judge_prompt.push('\n');
                }
                terms.judge_prompt.push_str(line);
            }
            _ => {}
        }
    }
    terms
}

#[derive(Clone)]
struct Model {
    provider: String,
    id: String,
    base: String,
    env_var: String,
    channel: String,
}

struct Resp {
    code: Option<u16>,
    body: String,
    ms: u128,
    timed_out: bool,
}

struct Answer {
    status: String,
    ms: u128,
    text: Option<String>,
    finish_reason: Option<String>,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Lang {
    De,
    En,
    Unknown,
}

struct Metrics {
    speculation: usize,
    forbidden: usize,
    slop: usize,
    repetition: f64,
}

struct JudgeScore {
    truth: Option<i64>,
    depth: Option<i64>,
    language: Option<i64>,
    cliche: Option<i64>,
    house: Option<i64>,
}

struct Row {
    provider: String,
    model: String,
    task: String,
    status: String,
    lang: Lang,
    chars: Option<usize>,
    finish_reason: Option<String>,
    ms: u128,
    metrics: Option<Metrics>,
    judge: Option<JudgeScore>,
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

fn extract_int_after(hay: &str, key: &str) -> Option<i64> {
    let marker = format!("\"{}\"", key);
    let pos = hay.find(&marker)?;
    let rest = &hay[pos + marker.len()..];
    let colon = rest.find(':')?;
    let after = rest[colon + 1..].trim_start();
    let digits: String = after.chars().take_while(|c| c.is_ascii_digit()).collect();
    if digits.is_empty() {
        return None;
    }
    digits.parse::<i64>().ok()
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
    })
}

fn is_http(m: &Model) -> bool {
    m.channel != "client"
}

fn load_models() -> Vec<Model> {
    let mut raw: Vec<Model> = MODELS_TSV
        .lines()
        .filter_map(parse_model)
        .filter(is_http)
        .collect();
    let extra_path = format!("{}/free_text_models.tsv", env!("CARGO_MANIFEST_DIR"));
    if let Ok(extra) = std::fs::read_to_string(&extra_path) {
        for m in extra.lines().filter_map(parse_model).filter(is_http) {
            raw.push(m);
        }
    }
    let mut seen: HashSet<(String, String)> = HashSet::new();
    let mut out: Vec<Model> = Vec::new();
    for m in raw {
        if seen.insert((m.provider.clone(), m.id.clone())) {
            out.push(m);
        }
    }
    out
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

fn extract_answer(body: &str) -> Option<String> {
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
    if extract_answer(body).is_some() {
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
        .filter(|m| matches(m, model_filters, provider_filters))
        .cloned()
        .collect();
    if let Some(n) = limit {
        v.truncate(n);
    }
    v
}

fn resolve_judge(models: &[Model], id: &str) -> Option<Model> {
    models
        .iter()
        .find(|m| m.id == id)
        .cloned()
        .or_else(|| models.iter().find(|m| m.id.contains(id)).cloned())
}

fn run_model(m: &Model, key: &str, prompt: &str, max_tokens: u32, timeout: u64) -> Answer {
    let url = format!("{}/chat/completions", m.base.trim_end_matches('/'));
    let body = build_body(&m.id, prompt, max_tokens);
    let mut total_ms = 0u128;
    let mut attempt = 0;
    loop {
        attempt += 1;
        let r = call(&url, key, &body, timeout);
        total_ms += r.ms;
        if r.timed_out {
            return Answer {
                status: "timeout".into(),
                ms: total_ms,
                text: None,
                finish_reason: None,
            };
        }
        let code = match r.code {
            Some(c) => c,
            None => {
                return Answer {
                    status: "timeout".into(),
                    ms: total_ms,
                    text: None,
                    finish_reason: None,
                };
            }
        };
        if code == 429 {
            if attempt >= 3 {
                return Answer {
                    status: classify(Some(code), false, &r.body),
                    ms: total_ms,
                    text: None,
                    finish_reason: None,
                };
            }
            let wait = retry_hint(&r.body);
            thread::sleep(Duration::from_secs(wait));
            continue;
        }
        if (500..600).contains(&code) {
            if attempt >= 2 {
                return Answer {
                    status: classify(Some(code), false, &r.body),
                    ms: total_ms,
                    text: None,
                    finish_reason: None,
                };
            }
            continue;
        }
        if code >= 400 {
            return Answer {
                status: classify(Some(code), false, &r.body),
                ms: total_ms,
                text: None,
                finish_reason: None,
            };
        }
        let text = extract_answer(&r.body);
        let finish_reason = extract_string_after(&r.body, "finish_reason");
        let status = if text.is_some() { "ok" } else { "empty" };
        return Answer {
            status: status.into(),
            ms: total_ms,
            text,
            finish_reason,
        };
    }
}

fn count_occurrences(hay: &str, needle: &str) -> usize {
    if needle.is_empty() {
        return 0;
    }
    hay.match_indices(needle).count()
}

fn scan_terms<T: AsRef<str>>(text: &str, terms: &[T]) -> usize {
    let lower = text.to_lowercase();
    terms
        .iter()
        .map(|t| count_occurrences(&lower, t.as_ref()))
        .sum()
}

fn slop_hits(text: &str, terms: &Terms) -> usize {
    let lower = text.to_lowercase();
    let mut n = scan_terms(&lower, &terms.slop);
    for (a, b) in &terms.slop_pairs {
        if lower.contains(&format!("{} ", a)) && lower.contains(&format!(" {} ", b)) {
            n += 1;
        }
    }
    n
}

fn repetition_rate(text: &str) -> f64 {
    let lower = text.to_lowercase();
    let tokens: Vec<&str> = lower
        .split(|c: char| !c.is_alphanumeric())
        .filter(|s| !s.is_empty())
        .collect();
    if tokens.len() < 3 {
        return 0.0;
    }
    let total = tokens.len() - 2;
    let mut seen: HashSet<(&str, &str, &str)> = HashSet::new();
    for w in tokens.windows(3) {
        seen.insert((w[0], w[1], w[2]));
    }
    let repeated = total.saturating_sub(seen.len());
    repeated as f64 / total as f64
}

fn detect_language(text: &str, terms: &Terms) -> Lang {
    let lower = text.to_lowercase();
    let tokens: Vec<&str> = lower
        .split(|c: char| !c.is_alphabetic() && c != '\u{df}')
        .filter(|s| !s.is_empty())
        .collect();
    let mut de = 0i32;
    let mut en = 0i32;
    for t in &tokens {
        if terms.de_function.iter().any(|w| w.as_str() == *t) {
            de += 1;
        }
        if EN_FUNCTION_WORDS.contains(t) {
            en += 1;
        }
    }
    for ch in lower.chars() {
        if ch == '\u{e4}' || ch == '\u{f6}' || ch == '\u{fc}' || ch == '\u{df}' {
            de += 2;
        }
    }
    if de > en {
        Lang::De
    } else if en > de {
        Lang::En
    } else {
        Lang::Unknown
    }
}

fn lang_code(l: Lang) -> &'static str {
    match l {
        Lang::De => "de",
        Lang::En => "en",
        Lang::Unknown => "?",
    }
}

fn metrics_for(text: &str, terms: &Terms) -> Metrics {
    Metrics {
        speculation: scan_terms(text, &terms.speculation),
        forbidden: scan_terms(text, FORBIDDEN_TERMS),
        slop: slop_hits(text, terms),
        repetition: repetition_rate(text),
    }
}

fn build_judge_prompt(template: &str, task: &str, answer: &str) -> String {
    template.replace("{task}", task).replace("{answer}", answer)
}

fn run_judge(
    jm: &Model,
    key: &str,
    template: &str,
    task: &str,
    answer: &str,
    timeout: u64,
) -> Option<JudgeScore> {
    let prompt = build_judge_prompt(template, task, answer);
    let a = run_model(jm, key, &prompt, 256, timeout);
    let text = a.text?;
    let s = JudgeScore {
        truth: extract_int_after(&text, "truth"),
        depth: extract_int_after(&text, "depth"),
        language: extract_int_after(&text, "language"),
        cliche: extract_int_after(&text, "cliche"),
        house: extract_int_after(&text, "house"),
    };
    if s.truth.is_none()
        && s.depth.is_none()
        && s.language.is_none()
        && s.cliche.is_none()
        && s.house.is_none()
    {
        None
    } else {
        Some(s)
    }
}

fn opt_int(v: Option<i64>) -> String {
    match v {
        Some(n) => n.to_string(),
        None => String::new(),
    }
}

fn row_line(r: &Row) -> String {
    let chars = match r.chars {
        Some(n) => n.to_string(),
        None => String::new(),
    };
    let finish = match &r.finish_reason {
        Some(s) => s.clone(),
        None => String::new(),
    };
    let (spec, forb, slop, rep) = match &r.metrics {
        Some(m) => (
            m.speculation.to_string(),
            m.forbidden.to_string(),
            m.slop.to_string(),
            format!("{:.3}", m.repetition),
        ),
        None => (String::new(), String::new(), String::new(), String::new()),
    };
    let (jt, jd, jl, jc, jh) = match &r.judge {
        Some(s) => (
            opt_int(s.truth),
            opt_int(s.depth),
            opt_int(s.language),
            opt_int(s.cliche),
            opt_int(s.house),
        ),
        None => (
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
        ),
    };
    format!(
        "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
        r.provider,
        r.model,
        r.task,
        r.status,
        lang_code(r.lang),
        chars,
        finish,
        r.ms,
        spec,
        forb,
        slop,
        rep,
        jt,
        jd,
        jl,
        jc,
        jh
    )
}

fn build_tsv(rows: &[Row], ts: Option<u64>) -> String {
    let mut r = String::new();
    match ts {
        Some(t) => r.push_str(&format!("# text_probe measured_at {}\n", t)),
        None => r.push_str("# text_probe measured_at pending\n"),
    }
    r.push_str("provider\tmodel\ttask\tstatus\tlang\tchars\tfinish_reason\tms\tspeculation\tforbidden\tslop\trepetition\tjudge_truth\tjudge_depth\tjudge_language\tjudge_cliche\tjudge_house\n");
    for row in rows {
        r.push_str(&row_line(row));
        r.push('\n');
    }
    r
}

fn safe_name(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c == '/' || c == '\\' || c == ':' || c.is_whitespace() {
                '_'
            } else {
                c
            }
        })
        .collect()
}

fn flush_answers(dir: &str, current: &Option<(String, String)>, buf: &str) -> std::io::Result<()> {
    match current {
        Some((p, m)) => {
            let path = format!("{}/{}__{}.md", dir, safe_name(p), safe_name(m));
            std::fs::write(path, buf)
        }
        None => Ok(()),
    }
}

fn write_answers(dir: &str, rows: &[Row]) -> std::io::Result<()> {
    std::fs::create_dir_all(dir)?;
    let mut current: Option<(String, String)> = None;
    let mut buf = String::new();
    for row in rows {
        let key = (row.provider.clone(), row.model.clone());
        if current.as_ref() != Some(&key) {
            flush_answers(dir, &current, &buf)?;
            current = Some(key);
            buf = format!("# {} {}\n", row.provider, row.model);
        }
        buf.push_str(&format!("\n## {}\n\n", row.task));
        match &row.text {
            Some(t) => {
                buf.push_str(t);
                buf.push('\n');
            }
            None => {
                buf.push_str(&format!("_({})_\n", row.status));
            }
        }
    }
    flush_answers(dir, &current, &buf)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut model_filters: Vec<String> = Vec::new();
    let mut provider_filters: Vec<String> = Vec::new();
    let mut out: Option<String> = None;
    let mut max_tokens: u32 = 2000;
    let mut timeout: u64 = 60;
    let mut limit: Option<usize> = None;
    let mut dry_run = false;
    let mut list_models = false;
    let mut judge_id: Option<String> = None;

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
            "--judge" => {
                if i + 1 < args.len() {
                    judge_id = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            "--dry-run" => dry_run = true,
            "--list-models" => list_models = true,
            _ => {}
        }
        i += 1;
    }

    let all_models = load_models();
    let terms = load_terms();

    if list_models {
        for m in &all_models {
            println!("{}\t{}", m.provider, m.id);
        }
        return;
    }

    let selected = select(&all_models, &model_filters, &provider_filters, limit);

    let judge_model = match &judge_id {
        Some(id) => match resolve_judge(&all_models, id) {
            Some(m) => Some(m),
            None => {
                eprintln!("text_probe: judge model absent from the model list: {}", id);
                std::process::exit(2);
            }
        },
        None => None,
    };
    let judge_key = match &judge_model {
        Some(jm) => key_for(jm),
        None => None,
    };

    if dry_run {
        for m in &selected {
            println!("{}\t{}", m.provider, m.id);
        }
        return;
    }

    let out_path = match out {
        Some(o) => o,
        None => "text-probe.tsv".to_string(),
    };

    let mut rows: Vec<Row> = Vec::new();
    for m in &selected {
        let key = key_for(m);
        for task in &terms.tasks {
            eprintln!("== {} {} / {} ==", m.provider, m.id, task.name);
            let answer = match &key {
                Some(k) => run_model(m, k, &task.prompt, max_tokens, timeout),
                None => Answer {
                    status: "pending_no_key".into(),
                    ms: 0,
                    text: None,
                    finish_reason: None,
                },
            };
            eprintln!("  {}", answer.status);
            let metrics = match &answer.text {
                Some(t) => Some(metrics_for(t, &terms)),
                None => None,
            };
            let judge = match (&answer.text, &judge_model, &judge_key) {
                (Some(t), Some(jm), Some(jk)) => {
                    run_judge(jm, jk, &terms.judge_prompt, &task.prompt, t, timeout)
                }
                _ => None,
            };
            rows.push(Row {
                provider: m.provider.clone(),
                model: m.id.clone(),
                task: task.name.clone(),
                status: answer.status,
                lang: match &answer.text {
                    Some(t) => detect_language(t, &terms),
                    None => Lang::Unknown,
                },
                chars: answer.text.as_ref().map(|t| t.chars().count()),
                finish_reason: answer.finish_reason,
                ms: answer.ms,
                metrics,
                judge,
                text: answer.text,
            });
        }
    }

    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .ok();
    let report = build_tsv(&rows, ts);
    if std::fs::write(&out_path, report).is_err() {
        eprintln!("text_probe: report not writable: {}", out_path);
        std::process::exit(2);
    }

    let answers_dir = format!("{}.answers", out_path);
    if write_answers(&answers_dir, &rows).is_err() {
        eprintln!("text_probe: answers not writable: {}", answers_dir);
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
        "text_probe: {}/{} ok, {} pending, {} http, {} timeout — report {}",
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
    fn language_heuristic_separates_de_and_en() {
        let terms = load_terms();
        let de = terms.de_function.join(" ");
        let en = EN_FUNCTION_WORDS.join(" ");
        assert_eq!(detect_language(&de, &terms), Lang::De);
        assert_eq!(detect_language(&en, &terms), Lang::En);
    }

    #[test]
    fn gate_vocabulary_scan_finds_speculation() {
        let terms = load_terms();
        assert_eq!(scan_terms("This is probably so.", &terms.speculation), 1);
        assert_eq!(
            scan_terms("A clean sentence without markers.", &terms.speculation),
            0
        );
        assert_eq!(scan_terms("the fallback is absent", FORBIDDEN_TERMS), 1);
    }

    #[test]
    fn slop_marker_finds_pair() {
        let terms = load_terms();
        assert_eq!(slop_hits("This is not truth but measurement.", &terms), 1);
        assert_eq!(slop_hits("A sentence without cliche.", &terms), 0);
    }

    #[test]
    fn load_terms_fills_every_section() {
        let terms = load_terms();
        assert!(!terms.de_function.is_empty());
        assert!(!terms.speculation.is_empty());
        assert!(!terms.slop.is_empty());
        assert_eq!(terms.slop_pairs.len(), 2);
        assert_eq!(terms.tasks.len(), 3);
        assert!(terms.tasks.iter().all(|t| !t.prompt.is_empty()));
        assert!(terms.judge_prompt.contains("{task}"));
        assert!(terms.judge_prompt.contains("{answer}"));
    }

    #[test]
    fn key_for_matches_env_name_exactly() {
        let m = Model {
            provider: "textprobe-test-provider".into(),
            id: "x".into(),
            base: "b".into(),
            env_var: "TEXT_PROBE_TEST_KEY".into(),
            channel: "http".into(),
        };
        unsafe { env::set_var("TEXT_PROBE_TEST_KEY", "exact-key") };
        assert_eq!(key_for(&m).as_deref(), Some("exact-key"));
        let other = Model {
            env_var: "TEXT_PROBE_TEST_KEY_EXT".into(),
            ..m.clone()
        };
        assert_eq!(key_for(&other), None);
    }

    #[test]
    fn http_path_skips_client_channel_rows() {
        let client = match parse_model(
            "opencode\tbig-pickle\thttps://opencode.ai/zen/v1\tOPENCODE_API_KEY\tclient",
        ) {
            Some(m) => m,
            None => panic!("client tsv line carries no model"),
        };
        let http = match parse_model(
            "zai\tglm-5.3-flash\thttps://api.z.ai/api/paas/v4\tZAI_API_KEY\thttp",
        ) {
            Some(m) => m,
            None => panic!("http tsv line carries no model"),
        };
        assert!(!is_http(&client));
        assert!(is_http(&http));
    }
}
