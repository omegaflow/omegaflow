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
    let cfg = std::fs::read_to_string(format!("{}/.config/opencode/opencode.jsonc", home())).ok()?;
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
            return Resp { code: None, body: String::new(), ms: start.elapsed().as_millis(), timed_out: false };
        }
    };
    if let Some(mut si) = child.stdin.take() {
        let _ = si.write_all(body.as_bytes());
    }
    let out = match child.wait_with_output() {
        Ok(o) => o,
        Err(_) => {
            return Resp { code: None, body: String::new(), ms: start.elapsed().as_millis(), timed_out: false };
        }
    };
    let ms = start.elapsed().as_millis();
    let text = String::from_utf8_lossy(&out.stdout).to_string();
    let (body_text, code) = match text.rfind('\n') {
        Some(i) => (text[..i].to_string(), text[i + 1..].trim().parse::<u16>().ok()),
        None => (text, None),
    };
    Resp { code, body: body_text, ms, timed_out: out.status.code() == Some(28) }
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

fn body_with(prompt: &str, tools: Option<&str>, max_tokens: u32) -> String {
    let mut b = String::new();
    b.push_str("{\"messages\":[{\"role\":\"user\",\"content\":\"");
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

fn build_body(task: &str) -> String {
    match task {
        "T1" => body_with("Call the get_weather tool for the city Berlin. You must use the tool.", Some(TOOLS1), 256),
        "T2" => body_with("Get the weather for Berlin, then advise on an umbrella. Use the tools.", Some(TOOLS2), 256),
        "T2b" => body_with("The get_weather result for Berlin is: rain. Now call the get_umbrella_advice tool with weather=rain.", Some(TOOLS2), 256),
        "T3" => body_with("Fix this Rust function so it sums 1 through n inclusive:\nfn sum_to(n: u32) -> u32 { let mut s = 0; for i in 1..n { s += i; } s }\nReturn only the corrected function.", None, 512),
        "T4" => body_with("Return ONLY a JSON object with fields: name (a short string), count (integer 1-10), mode (\"fast\" or \"safe\"), and meta (an object with boolean ok). Use mode \"fast\".", None, 256),
        "T5" => body_with("A train travels 60 km/h for 2.5 hours, then 80 km/h for 1.25 hours. What is the total distance in km? End with: Answer: <number>", None, 256),
        "T6" => {
            let p = format!("{} At the very end: what is the access code mentioned in the document? Return only the code.", build_filler());
            body_with(&p, None, 64)
        }
        "T7" => body_with(
            "Classify each funding program by the obligations it imposes on the recipient. Answer with exactly one line per program in the format NAME|LABEL, no spaces around the pipe, nothing else. LABEL is one of: free (no release, publication, or reporting obligations), duty (carries obligations), closed (not currently open). Programs: MacArthur Fellowship, Thiel Fellowship, Emergent Ventures, Astera Residency, Long-Term Future Fund.",
            None,
            256,
        ),
        _ => String::new(),
    }
}

fn check(task: &str, body: &str) -> bool {
    match task {
        "T1" => body.contains("\"tool_calls\"") && body.contains("get_weather") && body.contains("Berlin"),
        "T3" => body.contains("..=n"),
        "T4" => ["\"name\"", "\"count\"", "\"mode\"", "\"fast\"", "\"meta\"", "\"ok\""].iter().all(|k| body.contains(k)),
        "T5" => body.contains("250"),
        "T6" => body.contains("QX7-4412"),
        "T7" => {
            let n = body.replace(" |", "|").replace("| ", "|");
            T7_EXPECT.iter().all(|k| n.contains(k)) && !n.contains("Thiel Fellowship|free")
        }
        _ => false,
    }
}

fn run_trial(task: &str, m: &Model, key: &str) -> (String, u128) {
    let url = format!("{}/chat/completions", m.base.trim_end_matches('/'));
    let mut total_ms = 0u128;
    let mut attempt = 0;
    loop {
        attempt += 1;
        let body = build_body(task);
        let r = call(&url, key, &body);
        total_ms += r.ms;
        if r.timed_out {
            return ("timeout".into(), total_ms);
        }
        let code = match r.code {
            Some(c) => c,
            None => return ("timeout".into(), total_ms),
        };
        if code == 429 {
            if attempt >= 3 {
                return ("pending_rate_limited".into(), total_ms);
            }
            let wait = retry_hint(&r.body);
            thread::sleep(Duration::from_secs(wait));
            continue;
        }
        if (500..600).contains(&code) {
            if attempt >= 2 {
                return ("http_5xx".into(), total_ms);
            }
            continue;
        }
        if code >= 400 {
            if r.body.to_lowercase().contains("context") {
                return ("pending_context".into(), total_ms);
            }
            return (format!("http_{}", code), total_ms);
        }
        if task == "T2" {
            let first = r.body.contains("\"tool_calls\"") && r.body.contains("get_weather");
            if !first {
                return ("no_tool".into(), total_ms);
            }
            let r2 = call(&url, key, &build_body("T2b"));
            total_ms += r2.ms;
            return (if r2.body.contains("get_umbrella_advice") { "pass".into() } else { "no_tool".into() }, total_ms);
        }
        return (if check(task, &r.body) { "pass".into() } else { "wrong_answer".into() }, total_ms);
    }
}

fn parse_model(line: &str) -> Option<Model> {
    let mut it = line.split('\t');
    let provider = it.next()?.to_string();
    let id = it.next()?.to_string();
    let base = it.next()?.to_string();
    let env_var = it.next()?.to_string();
    if provider.is_empty() || id.is_empty() || base.is_empty() || env_var.is_empty() {
        return None;
    }
    Some(Model { provider, id, base, env_var })
}

fn write_tsv(path: &str, models: &[Model], rows: &[Row]) {
    let mut f = match std::fs::File::create(path) {
        Ok(f) => f,
        Err(_) => return,
    };
    for r in rows {
        let _ = writeln!(f, "{}\t{}\t{}\t{}\t{}\t{}", r.provider, r.model, r.task, r.trial, r.status, r.ms);
    }
    for m in models {
        let mrows: Vec<&Row> = rows.iter().filter(|r| r.provider == m.provider && r.model == m.id && r.task != "-").collect();
        if mrows.is_empty() {
            continue;
        }
        let count = |t: &str, s: &str| mrows.iter().filter(|r| r.task == t && r.status == s).count();
        let n_t = |t: &str| mrows.iter().filter(|r| r.task == t).count();
        let mut lats: Vec<u128> = mrows.iter().filter(|r| r.status == "pass").map(|r| r.ms).collect();
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
        let np = mrows.iter().filter(|r| r.status.starts_with("pending")).count();
        let _ = writeln!(
            f,
            "{}\t{}\tSUMMARY\ttool_ok={}/{}\tT2={}/{}\tT3={}/{}\tT4={}/{}\tT5={}/{}\tT6={}/{}\tT7={}/{}\tp50={}\tp95={}\t429={}\t5xx={}\ttimeout={}\tpending={}",
            m.provider, m.id,
            count("T1", "pass"), n_t("T1"),
            count("T2", "pass"), n_t("T2"),
            count("T3", "pass"), n_t("T3"),
            count("T4", "pass"), n_t("T4"),
            count("T5", "pass"), n_t("T5"),
            count("T6", "pass"), n_t("T6"),
            count("T7", "pass"), n_t("T7"),
            pct(0.5), pct(0.95), n429, n5, nt, np
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

    let models: Vec<Model> = MODELS_TSV.lines().filter_map(parse_model).collect();
    let tasks: [(&str, u32); 7] = [("T1", 10), ("T2", 5), ("T3", 3), ("T4", 5), ("T5", 3), ("T6", 3), ("T7", 3)];
    let mut rows: Vec<Row> = Vec::new();

    for m in &models {
        if let Some(f) = &filter {
            if !m.id.contains(f.as_str()) {
                continue;
            }
        }
        let key = match key_for(m) {
            Some(k) => k,
            None => {
                rows.push(Row {
                    provider: m.provider.clone(),
                    model: m.id.clone(),
                    task: "-".into(),
                    trial: 0,
                    status: "pending_no_key".into(),
                    ms: 0,
                });
                continue;
            }
        };
        eprintln!("== {} {} ==", m.provider, m.id);
        for (t, n) in tasks {
            if let Some(tf) = &task_filter {
                if tf != t {
                    continue;
                }
            }
            for trial in 1..=n {
                let (status, ms) = run_trial(t, m, &key);
                eprintln!("  {} {}/{} {}", t, trial, n, status);
                rows.push(Row {
                    provider: m.provider.clone(),
                    model: m.id.clone(),
                    task: t.into(),
                    trial,
                    status,
                    ms,
                });
            }
        }
    }

    write_tsv(&out, &models, &rows);
    eprintln!("wrote {}", out);
}
