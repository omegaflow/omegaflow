use std::env;
use std::io::Write;
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

const MODELS_TSV: &str = include_str!("../../free_models.tsv");

const TOOLS1: &str = r#"[{"type":"function","function":{"name":"get_weather","description":"Get current weather","parameters":{"type":"object","properties":{"city":{"type":"string"}},"required":["city"]}}}]"#;
const TOOLS2: &str = r#"[{"type":"function","function":{"name":"get_weather","description":"Get current weather","parameters":{"type":"object","properties":{"city":{"type":"string"}},"required":["city"]}}},{"type":"function","function":{"name":"get_umbrella_advice","description":"Advise on umbrella given weather","parameters":{"type":"object","properties":{"weather":{"type":"string"}},"required":["weather"]}}}]"#;

const T7_EXPECT: [&str; 5] = [
    "MacArthur Fellowship|free",
    "Thiel Fellowship|duty",
    "Emergent Ventures|free",
    "Astera Residency|duty",
    "Long-Term Future Fund|closed",
];

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

struct Row {
    provider: String,
    model: String,
    task: String,
    trial: u32,
    status: String,
    ms: u128,
    note: String,
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

fn decode_json_string(after: &str) -> Option<String> {
    let mut chars = after.chars();
    if chars.next() != Some('"') {
        return None;
    }
    let mut out = String::new();
    while let Some(c) = chars.next() {
        match c {
            '"' => return Some(out),
            '\\' => match chars.next()? {
                'n' => out.push('\n'),
                'r' => out.push('\r'),
                't' => out.push('\t'),
                'b' => out.push('\u{8}'),
                'f' => out.push('\u{c}'),
                '"' => out.push('"'),
                '\\' => out.push('\\'),
                '/' => out.push('/'),
                'u' => {
                    let hex: String = chars.by_ref().take(4).collect();
                    if hex.len() != 4 {
                        return None;
                    }
                    if let Ok(code) = u32::from_str_radix(&hex, 16) {
                        if let Some(ch) = char::from_u32(code) {
                            out.push(ch);
                        }
                    }
                }
                other => {
                    out.push('\\');
                    out.push(other);
                }
            },
            c => out.push(c),
        }
    }
    None
}

fn assistant_content(body: &str) -> Option<String> {
    let scope = match body.find("\"message\"") {
        Some(p) => &body[p..],
        None => body,
    };
    let marker = "\"content\"";
    let pos = scope.find(marker)?;
    let after = scope[pos + marker.len()..].trim_start();
    let after = after.strip_prefix(':')?.trim_start();
    if after.starts_with("null") {
        return None;
    }
    if !after.starts_with('"') {
        return None;
    }
    decode_json_string(after)
}

fn home() -> String {
    match env::var("HOME") {
        Ok(v) => v,
        Err(_) => String::new(),
    }
}

fn key_for(m: &Model) -> Option<String> {
    if let Ok(v) = env::var(&m.env_var) {
        if !v.is_empty() {
            return Some(v);
        }
    }
    let auth = std::fs::read_to_string(format!("{}/.local/share/opencode/auth.json", home())).ok();
    if let Some(a) = auth {
        if let Some(pos) = a.find(&format!("\"{}\"", m.provider)) {
            if let Some(k) = extract_string_after(&a[pos..], "key") {
                if !k.is_empty() {
                    return Some(k);
                }
            }
        }
    }
    let cfg =
        std::fs::read_to_string(format!("{}/.config/opencode/opencode.jsonc", home())).ok()?;
    if let Some(pos) = cfg.find(&format!("\"{}\"", m.provider)) {
        if let Some(k) = extract_string_after(&cfg[pos..], "apiKey") {
            if !k.is_empty() {
                return Some(k);
            }
        }
    }
    None
}

fn call(url: &str, key: &str, body: &str) -> Resp {
    let start = Instant::now();
    let child = Command::new("curl")
        .args([
            "-sS",
            "-m",
            "60",
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

fn build_filler() -> String {
    let unit = "The quick brown fox jumps over the lazy dog near the river bank at dawn. ";
    let mut s = String::with_capacity(140_000);
    let mut i = 0;
    while s.len() < 130_000 {
        s.push_str(unit);
        i += 1;
        if i == 500 {
            s.push_str("The access code is QX7-4412. ");
        }
    }
    s
}

fn body_with(prompt: &str, tools: Option<&str>, max_tokens: u32, model: &str) -> String {
    let mut b = String::new();
    b.push_str("{\"model\":\"");
    b.push_str(&json_escape(model));
    b.push_str("\",\"messages\":[{\"role\":\"user\",\"content\":\"");
    b.push_str(&json_escape(prompt));
    b.push_str("\"}]");
    if let Some(t) = tools {
        b.push_str(",\"tools\":");
        b.push_str(t);
        b.push_str(",\"tool_choice\":\"auto\"");
    }
    b.push_str(",\"max_tokens\":");
    b.push_str(&max_tokens.to_string());
    b.push('}');
    b
}

fn build_body(task: &str, model: &str) -> String {
    match task {
        "T1" => body_with(
            "Call the get_weather tool for the city Berlin. You must use the tool.",
            Some(TOOLS1),
            256,
            model,
        ),
        "T2" => body_with(
            "Get the weather for Berlin, then advise on an umbrella. Use the tools.",
            Some(TOOLS2),
            256,
            model,
        ),
        "T2b" => body_with(
            "The get_weather result for Berlin is: rain. Now call the get_umbrella_advice tool with weather=rain.",
            Some(TOOLS2),
            256,
            model,
        ),
        "T3" => body_with(
            "Fix this Rust function so it sums 1 through n inclusive:\nfn sum_to(n: u32) -> u32 { let mut s = 0; for i in 1..n { s += i; } s }\nReturn only the corrected function.",
            None,
            512,
            model,
        ),
        "T4" => body_with(
            "Return ONLY a JSON object with fields: name (a short string), count (integer 1-10), mode (\"fast\" or \"safe\"), and meta (an object with boolean ok). Use mode \"fast\".",
            None,
            4096,
            model,
        ),
        "T5" => body_with(
            "A train travels 60 km/h for 2.5 hours, then 80 km/h for 1.25 hours. What is the total distance in km? End with: Answer: <number>",
            None,
            256,
            model,
        ),
        "T6" => {
            let p = format!(
                "{} At the very end: what is the access code mentioned in the document? Return only the code.",
                build_filler()
            );
            body_with(&p, None, 64, model)
        }
        "T7" => body_with(
            "Classify each funding program by the obligations it imposes on the recipient. Answer with exactly one line per program in the format NAME|LABEL, no spaces around the pipe, nothing else. LABEL is one of: free (no release, publication, or reporting obligations), duty (carries obligations), closed (not currently open). Programs: MacArthur Fellowship, Thiel Fellowship, Emergent Ventures, Astera Residency, Long-Term Future Fund.",
            None,
            8192,
            model,
        ),
        _ => String::new(),
    }
}

fn check(task: &str, body: &str, text: &str) -> bool {
    match task {
        "T1" => {
            body.contains("\"tool_calls\"")
                && body.contains("get_weather")
                && body.contains("Berlin")
        }
        "T3" => text.contains("..=n"),
        "T4" => [
            "\"name\"",
            "\"count\"",
            "\"mode\"",
            "\"fast\"",
            "\"meta\"",
            "\"ok\"",
        ]
        .iter()
        .all(|k| text.contains(k)),
        "T5" => text.contains("250"),
        "T6" => text.contains("QX7-4412"),
        "T7" => {
            let n = text.replace(" |", "|").replace("| ", "|");
            T7_EXPECT.iter().all(|k| n.contains(k)) && !n.contains("Thiel Fellowship|free")
        }
        _ => false,
    }
}

fn note_of(body: &str) -> String {
    body.chars()
        .map(|c| {
            if c == '\n' || c == '\r' || c == '\t' {
                ' '
            } else {
                c
            }
        })
        .take(160)
        .collect()
}

fn run_trial(task: &str, m: &Model, key: &str) -> (String, u128, String) {
    let url = format!("{}/chat/completions", m.base.trim_end_matches('/'));
    let mut total_ms = 0u128;
    let mut attempt = 0;
    loop {
        attempt += 1;
        let body = build_body(task, &m.id);
        let r = call(&url, key, &body);
        total_ms += r.ms;
        if r.timed_out {
            return ("timeout".into(), total_ms, String::new());
        }
        let code = match r.code {
            Some(c) => c,
            None => return ("timeout".into(), total_ms, note_of(&r.body)),
        };
        if code == 429 {
            if attempt >= 3 {
                return ("pending_rate_limited".into(), total_ms, note_of(&r.body));
            }
            let wait = retry_hint(&r.body);
            thread::sleep(Duration::from_secs(wait));
            continue;
        }
        if (500..600).contains(&code) {
            if attempt >= 2 {
                return ("http_5xx".into(), total_ms, note_of(&r.body));
            }
            continue;
        }
        if code >= 400 {
            if r.body.to_lowercase().contains("context") {
                return ("pending_context".into(), total_ms, note_of(&r.body));
            }
            return (format!("http_{}", code), total_ms, note_of(&r.body));
        }
        if task == "T2" {
            let first = r.body.contains("\"tool_calls\"") && r.body.contains("get_weather");
            if !first {
                return ("no_tool".into(), total_ms, note_of(&r.body));
            }
            let r2 = call(&url, key, &build_body("T2b", &m.id));
            total_ms += r2.ms;
            return (
                if r2.body.contains("get_umbrella_advice") {
                    "pass".into()
                } else {
                    "no_tool".into()
                },
                total_ms,
                note_of(&r2.body),
            );
        }
        let content = assistant_content(&r.body);
        let text = match &content {
            Some(t) => t.as_str(),
            None => "",
        };
        if check(task, &r.body, text) {
            return ("pass".into(), total_ms, note_of(&r.body));
        }
        if text.trim().is_empty() {
            return ("no_output".into(), total_ms, note_of(&r.body));
        }
        return ("wrong_answer".into(), total_ms, note_of(text));
    }
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

fn write_summary(f: &mut std::fs::File, models: &[Model], rows: &[Row]) {
    for m in models {
        let mrows: Vec<&Row> = rows
            .iter()
            .filter(|r| r.provider == m.provider && r.model == m.id && r.task != "-")
            .collect();
        if mrows.is_empty() {
            continue;
        }
        let count = |t: &str, s: &str| {
            mrows
                .iter()
                .filter(|r| r.task == t && r.status == s)
                .count()
        };
        let n_t = |t: &str| mrows.iter().filter(|r| r.task == t).count();
        let mut lats: Vec<u128> = mrows
            .iter()
            .filter(|r| r.status == "pass")
            .map(|r| r.ms)
            .collect();
        lats.sort();
        let pct = |q: f64| -> u128 {
            if lats.is_empty() {
                0
            } else {
                lats[((lats.len() as f64 * q) as usize).min(lats.len() - 1)]
            }
        };
        let n429 = mrows.iter().filter(|r| r.status == "http_429").count();
        let n5 = mrows.iter().filter(|r| r.status == "http_5xx").count();
        let nt = mrows.iter().filter(|r| r.status == "timeout").count();
        let np = mrows
            .iter()
            .filter(|r| r.status.starts_with("pending"))
            .count();
        let _ = writeln!(
            f,
            "{}\t{}\tSUMMARY\ttool_ok={}/{}\tT2={}/{}\tT3={}/{}\tT4={}/{}\tT5={}/{}\tT6={}/{}\tT7={}/{}\tp50={}\tp95={}\t429={}\t5xx={}\ttimeout={}\tpending={}",
            m.provider,
            m.id,
            count("T1", "pass"),
            n_t("T1"),
            count("T2", "pass"),
            n_t("T2"),
            count("T3", "pass"),
            n_t("T3"),
            count("T4", "pass"),
            n_t("T4"),
            count("T5", "pass"),
            n_t("T5"),
            count("T6", "pass"),
            n_t("T6"),
            count("T7", "pass"),
            n_t("T7"),
            pct(0.5),
            pct(0.95),
            n429,
            n5,
            nt,
            np
        );
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut out = "free_model_bench.tsv".to_string();
    let mut filter: Option<String> = None;
    let mut task_filter: Option<String> = None;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--out" => {
                if i + 1 < args.len() {
                    out = args[i + 1].clone();
                    i += 1;
                }
            }
            "--model" => {
                if i + 1 < args.len() {
                    filter = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            "--task" => {
                if i + 1 < args.len() {
                    task_filter = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            _ => {}
        }
        i += 1;
    }

    let models: Vec<Model> = MODELS_TSV
        .lines()
        .filter_map(parse_model)
        .filter(is_http)
        .collect();
    let tasks: [(&str, u32); 7] = [
        ("T1", 10),
        ("T2", 5),
        ("T3", 3),
        ("T4", 5),
        ("T5", 3),
        ("T6", 3),
        ("T7", 3),
    ];
    let selected: Vec<&Model> = models
        .iter()
        .filter(|m| filter.as_ref().map_or(true, |f| m.id.contains(f.as_str())))
        .collect();
    let workers: usize = env::var("BENCH_WORKERS")
        .ok()
        .and_then(|v| v.parse::<std::num::NonZeroUsize>().ok())
        .map(|v| v.get())
        .unwrap_or(6);
    let mut groups: Vec<Vec<&Model>> = vec![Vec::new(); workers];
    for (i, m) in selected.iter().enumerate() {
        groups[i % workers].push(m);
    }
    let tf_ref = &task_filter;

    let rows: Vec<Row> = std::thread::scope(|scope| {
        let mut handles = Vec::new();
        for g in groups {
            let path = out.clone();
            handles.push(scope.spawn(move || {
                let mut sink = std::fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&path)
                    .ok();
                let mut local: Vec<Row> = Vec::new();
                let mut emit = |r: Row| {
                    if let Some(f) = sink.as_mut() {
                        let line = format!(
                            "{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
                            r.provider, r.model, r.task, r.trial, r.status, r.ms, r.note
                        );
                        let _ = f.write_all(line.as_bytes());
                    }
                    local.push(r);
                };
                for m in g {
                    let key = match key_for(m) {
                        Some(k) => k,
                        None => {
                            emit(Row {
                                provider: m.provider.clone(),
                                model: m.id.clone(),
                                task: "-".into(),
                                trial: 0,
                                status: "pending_no_key".into(),
                                ms: 0,
                                note: String::new(),
                            });
                            continue;
                        }
                    };
                    eprintln!("== {} {} ==", m.provider, m.id);
                    for (t, n) in tasks {
                        if let Some(tf) = tf_ref {
                            if tf != t {
                                continue;
                            }
                        }
                        for trial in 1..=n {
                            let (status, ms, note) = run_trial(t, m, &key);
                            eprintln!("  {} {} {}/{} {}", m.provider, t, trial, n, status);
                            emit(Row {
                                provider: m.provider.clone(),
                                model: m.id.clone(),
                                task: t.into(),
                                trial,
                                status,
                                ms,
                                note,
                            });
                        }
                    }
                }
                local
            }));
        }
        let mut all: Vec<Row> = Vec::new();
        for h in handles {
            if let Ok(v) = h.join() {
                all.extend(v);
            }
        }
        all
    });

    if let Ok(mut f) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&out)
    {
        write_summary(&mut f, &models, &rows);
    }
    eprintln!("wrote {}", out);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_model_splits_tsv_line() {
        let m = match parse_model(
            "zai\tglm-5.3-flash\thttps://api.z.ai/api/paas/v4\tZAI_API_KEY\thttp",
        ) {
            Some(m) => m,
            None => panic!("tsv line carries no model"),
        };
        assert_eq!(m.provider, "zai");
        assert_eq!(m.id, "glm-5.3-flash");
        assert_eq!(m.base, "https://api.z.ai/api/paas/v4");
        assert_eq!(m.env_var, "ZAI_API_KEY");
        assert_eq!(m.channel, "http");
        assert!(parse_model("bad\tline").is_none());
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

    const ESCAPED_T4_BODY: &str = r#"{"choices":[{"finish_reason":"stop","index":0,"message":{"content":"```json\n{\n  \"name\": \"sample\",\n  \"count\": 7,\n  \"mode\": \"fast\",\n  \"meta\": {\n    \"ok\": true\n  }\n}\n```"}}]}"#;

    #[test]
    fn assistant_content_decodes_the_escaped_message_text() {
        let content = match assistant_content(ESCAPED_T4_BODY) {
            Some(c) => c,
            None => panic!("escaped envelope carries a message content string"),
        };
        assert!(content.contains("\"name\""));
        assert!(content.contains("\"fast\""));
        assert!(content.starts_with("```json"));
    }

    #[test]
    fn t4_scores_the_decoded_content_not_the_raw_envelope() {
        let content = match assistant_content(ESCAPED_T4_BODY) {
            Some(c) => c,
            None => panic!("escaped envelope carries a message content string"),
        };
        assert!(!check("T4", ESCAPED_T4_BODY, ""));
        assert!(check("T4", ESCAPED_T4_BODY, &content));
    }

    #[test]
    fn reasoning_only_response_carries_an_empty_content() {
        let body = r#"{"choices":[{"finish_reason":"length","index":0,"message":{"content":"","reasoning_content":"we might say \"name\""}}]}"#;
        match assistant_content(body) {
            Some(content) => assert!(content.trim().is_empty()),
            None => panic!("the empty content string is present, not absent"),
        }
    }

    #[test]
    fn null_content_is_absent() {
        let body = r#"{"choices":[{"finish_reason":"length","index":0,"message":{"content":null,"reasoning_content":"the answer would be ..."}}]}"#;
        assert!(assistant_content(body).is_none());
    }

    #[test]
    fn reasoning_content_does_not_score_t7() {
        let body = r#"{"choices":[{"finish_reason":"length","index":0,"message":{"content":null,"reasoning_content":"MacArthur Fellowship|free\nThiel Fellowship|duty\nEmergent Ventures|free\nAstera Residency|duty\nLong-Term Future Fund|closed"}}]}"#;
        assert!(assistant_content(body).is_none());
        assert!(!check("T7", body, ""));
    }

    #[test]
    fn t4_and_t7_carry_raised_token_budgets() {
        assert!(build_body("T4", "m").contains("\"max_tokens\":4096"));
        assert!(build_body("T7", "m").contains("\"max_tokens\":8192"));
    }

    #[test]
    fn decode_json_string_restores_escapes() {
        let decoded = match decode_json_string(r#""a\n\"b\"\t\u00e9""#) {
            Some(d) => d,
            None => panic!("escaped string decodes"),
        };
        assert_eq!(decoded, "a\n\"b\"\té");
    }
}
