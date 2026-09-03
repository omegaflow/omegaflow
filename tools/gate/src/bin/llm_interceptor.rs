use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use omegaflow::axioms;
use omegaflow::friction::{parse_model_register, Advice, Friction, ModelProps};
use omegaflow::handover::{consent_word, handover_markdown, spawn_command, write_handover};
use omegaflow::json::{parse_json, JsonVal};
use omegaflow::llm_gate::{json_write, Gate, Severity};
use omegaflow::state::{self, State};
use omegaflow::tool_perm::{ToolPerm, ToolState};

const MAX_REWRITES: u32 = 3;

struct DeltaInfo {
    content: Option<String>,
    finish: Option<String>,
    tools: Vec<(u32, Option<String>, String)>,
    usage_prompt: Option<u64>,
}

enum CheckOutcome {
    Clean,
    Soft,
    Hard,
}

fn env_str(key: &str, fallback: &str) -> String {
    std::env::var(key)
        .ok()
        .filter(|v| !v.is_empty())
        .unwrap_or_else(|| fallback.to_string())
}

fn env_u64(key: &str, fallback: u64) -> u64 {
    std::env::var(key)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(fallback)
}

fn normalize_model(raw: &str) -> String {
    let t = raw.trim();
    for prefix in ["omegaflow/", "deepseek/"] {
        if let Some(rest) = t.strip_prefix(prefix) {
            return rest.to_string();
        }
    }
    t.to_string()
}

fn delta_parts(data: &JsonVal) -> DeltaInfo {
    let mut info = DeltaInfo {
        content: None,
        finish: None,
        tools: Vec::new(),
        usage_prompt: None,
    };
    let JsonVal::Obj(map) = data else {
        return info;
    };
    if let Some(JsonVal::Obj(usage)) = map.get("usage") {
        if let Some(JsonVal::Num(n)) = usage.get("prompt_tokens") {
            if *n > 0.0 && n.is_finite() {
                info.usage_prompt = Some(*n as u64);
            }
        }
    }
    let Some(JsonVal::Arr(choices)) = map.get("choices") else {
        return info;
    };
    let Some(JsonVal::Obj(first)) = choices.first() else {
        return info;
    };
    if let Some(JsonVal::Obj(delta)) = first.get("delta") {
        if let Some(JsonVal::Str(c)) = delta.get("content") {
            info.content = Some(c.clone());
        }
        if let Some(JsonVal::Arr(tcs)) = delta.get("tool_calls") {
            for tc in tcs {
                let JsonVal::Obj(tc) = tc else { continue };
                let index = match tc.get("index") {
                    Some(JsonVal::Num(n)) => *n as u32,
                    _ => 0,
                };
                let mut name = None;
                let mut args = String::new();
                if let Some(JsonVal::Obj(f)) = tc.get("function") {
                    if let Some(JsonVal::Str(n)) = f.get("name") {
                        name = Some(n.clone());
                    }
                    if let Some(JsonVal::Str(a)) = f.get("arguments") {
                        args = a.clone();
                    }
                }
                info.tools.push((index, name, args));
            }
        }
    }
    if let Some(JsonVal::Str(f)) = first.get("finish_reason") {
        info.finish = Some(f.clone());
    }
    info
}

fn json_bool(v: &JsonVal, key: &str) -> bool {
    match v {
        JsonVal::Obj(map) => matches!(map.get(key), Some(JsonVal::Bool(true))),
        _ => false,
    }
}

fn json_str_field(v: &JsonVal, key: &str) -> Option<String> {
    match v {
        JsonVal::Obj(map) => match map.get(key) {
            Some(JsonVal::Str(s)) => Some(s.clone()),
            _ => None,
        },
        _ => None,
    }
}

fn resolve_key() -> String {
    if let Ok(k) = std::env::var("OMEGAFLOW_LLM_KEY") {
        if !k.is_empty() {
            return k;
        }
    }
    let home = std::env::var("HOME").unwrap_or_default();
    let path = format!("{}/.local/share/opencode/auth.json", home);
    if let Ok(text) = std::fs::read_to_string(&path) {
        if let Some(JsonVal::Obj(root)) = parse_json(&text) {
            for name in ["deepseek", "tokenrouter", "openai"] {
                if let Some(JsonVal::Obj(prov)) = root.get(name) {
                    if let Some(JsonVal::Str(k)) = prov.get("key") {
                        return k.clone();
                    }
                }
            }
        }
    }
    String::new()
}

fn marker_delta(text: &str) -> String {
    let obj = JsonVal::Obj({
        let mut d = HashMap::new();
        d.insert("content".to_string(), JsonVal::Str(text.to_string()));
        let mut first = HashMap::new();
        first.insert("delta".to_string(), JsonVal::Obj(d));
        let mut root = HashMap::new();
        root.insert(
            "choices".to_string(),
            JsonVal::Arr(vec![JsonVal::Obj(first)]),
        );
        root
    });
    format!("data: {}\n\n", json_write(&obj))
}

fn sse_done() -> String {
    "data: [DONE]\n\n".to_string()
}

fn write_chunk(stream: &mut TcpStream, data: &[u8]) -> bool {
    if stream
        .write_all(format!("{:x}\r\n", data.len()).as_bytes())
        .is_err()
        || stream.write_all(data).is_err()
        || stream.write_all(b"\r\n").is_err()
    {
        return false;
    }
    let _ = stream.flush();
    true
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
    let content_length: Option<usize> = headers
        .iter()
        .find(|(k, _)| k == "content-length")
        .and_then(|(_, v)| v.parse().ok());
    let mut body = buf[header_end + 4..].to_vec();
    if let Some(len) = content_length {
        while body.len() < len {
            match stream.read(&mut tmp) {
                Ok(0) => break,
                Ok(n) => body.extend_from_slice(&tmp[..n]),
                Err(_) => break,
            }
        }
        body.truncate(len);
    }
    let mut parts = request_line.split_whitespace();
    let method = parts.next().unwrap_or("").to_string();
    let path = parts.next().unwrap_or("").to_string();
    Some((method, path, headers, body))
}

fn find_header_end(buf: &[u8]) -> Option<usize> {
    buf.windows(4).position(|w| w == b"\r\n\r\n")
}

fn spawn_curl(upstream: &str, key: &str, body: &[u8], auth_header: Option<&str>) -> Option<Child> {
    let url = format!("{}/chat/completions", upstream.trim_end_matches('/'));
    let mut cmd = Command::new("curl");
    cmd.arg("-sS")
        .arg("-N")
        .arg("--max-time")
        .arg("900")
        .arg(&url)
        .arg("-H")
        .arg("Content-Type: application/json")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit());
    let auth = if !key.is_empty() {
        format!("Authorization: Bearer {}", key)
    } else {
        auth_header
            .map(|a| format!("Authorization: {}", a))
            .unwrap_or_default()
    };
    cmd.arg("-H").arg(&auth);
    cmd.arg("--data-binary").arg("@-");
    let mut child = cmd.spawn().ok()?;
    if let Some(mut stdin) = child.stdin.take() {
        let _ = stdin.write_all(body);
    }
    Some(child)
}

fn run_curl_full(
    upstream: &str,
    key: &str,
    body: &[u8],
    auth_header: Option<&str>,
) -> Option<String> {
    let url = format!("{}/chat/completions", upstream.trim_end_matches('/'));
    let mut cmd = Command::new("curl");
    cmd.arg("-sS")
        .arg("--max-time")
        .arg("300")
        .arg(&url)
        .arg("-H")
        .arg("Content-Type: application/json")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit());
    let auth = if !key.is_empty() {
        format!("Authorization: Bearer {}", key)
    } else {
        auth_header
            .map(|a| format!("Authorization: {}", a))
            .unwrap_or_default()
    };
    cmd.arg("-H").arg(&auth);
    cmd.arg("--data-binary").arg("@-");
    let mut child = cmd.spawn().ok()?;
    if let Some(mut stdin) = child.stdin.take() {
        let _ = stdin.write_all(body);
    }
    let mut out = String::new();
    if let Some(mut stdout) = child.stdout.take() {
        let _ = stdout.read_to_string(&mut out);
    }
    let _ = child.wait();
    Some(out)
}

fn intervention_body(messages: &JsonVal, model: &str, partial: &str) -> String {
    let JsonVal::Obj(root) = messages else {
        return String::new();
    };
    let msg_list = root
        .get("messages")
        .cloned()
        .unwrap_or(JsonVal::Arr(Vec::new()));
    let assistant = JsonVal::Obj({
        let mut m = HashMap::new();
        m.insert("role".to_string(), JsonVal::Str("assistant".to_string()));
        m.insert("content".to_string(), JsonVal::Str(partial.to_string()));
        m
    });
    let system = JsonVal::Obj({
        let mut m = HashMap::new();
        m.insert("role".to_string(), JsonVal::Str("system".to_string()));
        m.insert(
            "content".to_string(),
            JsonVal::Str(format!(
                "OMEGAFLOW GATE INTERVENTION. The text just emitted was not delivered to the reader. Rewrite the intended statement silently and continue from there — emit only the corrected continuation. Never explain the correction, never name a rule, never quote the offending text, never say 'Zurückgenommen' or 'taken back', never apologize.",
            )),
        );
        m
    });
    let messages2 = match msg_list {
        JsonVal::Arr(mut items) => {
            items.push(assistant);
            items.push(system);
            JsonVal::Arr(items)
        }
        other => JsonVal::Arr(vec![other, assistant, system]),
    };
    let body = JsonVal::Obj({
        let mut m = HashMap::new();
        if !model.is_empty() {
            m.insert("model".to_string(), JsonVal::Str(model.to_string()));
        }
        m.insert("stream".to_string(), JsonVal::Bool(true));
        m.insert("messages".to_string(), messages2);
        m
    });
    json_write(&body)
}

struct Shared {
    gate: Mutex<Gate>,
    friction: Mutex<Friction>,
    models: HashMap<String, ModelProps>,
    reported_pending: Mutex<HashSet<String>>,
    handover_pending: Mutex<Option<String>>,
    handover_written: Mutex<bool>,
    state: Mutex<State>,
    perm: Mutex<ToolPerm>,
    seen_tool_ids: Mutex<HashSet<String>>,
}

fn with_include_usage(mut parsed: JsonVal, streaming: bool) -> JsonVal {
    if streaming {
        if let JsonVal::Obj(map) = &mut parsed {
            let mut so = HashMap::new();
            so.insert("include_usage".to_string(), JsonVal::Bool(true));
            map.insert("stream_options".to_string(), JsonVal::Obj(so));
        }
    }
    parsed
}

fn git_changes(root: &str) -> Vec<String> {
    let out = Command::new("git")
        .arg("-C")
        .arg(root)
        .arg("status")
        .arg("--porcelain")
        .output();
    match out {
        Ok(o) if o.status.success() => String::from_utf8_lossy(&o.stdout)
            .lines()
            .map(|l| l[3..].trim().to_string())
            .filter(|s| !s.is_empty())
            .collect(),
        _ => Vec::new(),
    }
}

fn git_log(root: &str) -> String {
    match Command::new("git")
        .arg("-C")
        .arg(root)
        .arg("log")
        .arg("--oneline")
        .arg("-5")
        .output()
    {
        Ok(o) if o.status.success() => String::from_utf8_lossy(&o.stdout).to_string(),
        _ => String::new(),
    }
}

fn git_ref(root: &str, flag: &str) -> String {
    match Command::new("git")
        .arg("-C")
        .arg(root)
        .arg("rev-parse")
        .arg(flag)
        .output()
    {
        Ok(o) if o.status.success() => String::from_utf8_lossy(&o.stdout).trim().to_string(),
        _ => String::new(),
    }
}

fn finish_state(shared: &Arc<Shared>, root: &str, state_path: &str, fill: Option<f64>) {
    let violations = { shared.gate.lock().unwrap().violations };
    let mut st = shared.state.lock().unwrap();
    st.head = git_ref(root, "--short");
    st.violations = violations;
    st.fill = fill;
    let _ = state::write(state_path, &st);
}

fn do_handover(
    shared: &Arc<Shared>,
    stream: &mut TcpStream,
    root: &str,
    reason: &str,
    state: &str,
) {
    let mut written = shared.handover_written.lock().unwrap();
    if *written {
        return;
    }
    *written = true;
    drop(written);
    let log = git_log(root);
    let status = git_changes(root).join("\n");
    let md = handover_markdown(reason, state, &log, &status);
    match write_handover(root, &md) {
        Ok(path) => {
            let mut pending = shared.handover_pending.lock().unwrap();
            *pending = Some(path.clone());
            drop(pending);
            let marker = marker_delta(&format!(
                "\n⚠ handover: drift measured ({}). handover written: {}. Reply 'Go' to open the new session.\n",
                reason, path
            ));
            let _ = write_chunk(stream, marker.as_bytes());
        }
        Err(_) => {
            let marker = marker_delta(&format!(
                "\n⚠ handover: drift measured ({}) — handover not written.\n",
                reason
            ));
            let _ = write_chunk(stream, marker.as_bytes());
        }
    }
}

fn check_window(gate: &mut Gate, text: &str) -> CheckOutcome {
    match gate.check_text(text) {
        Some(v) if v.severity == Severity::Hard => {
            gate.record_violation(&v.rule, &v.quote);
            CheckOutcome::Hard
        }
        Some(v) => {
            gate.record_violation(&v.rule, &v.quote);
            CheckOutcome::Soft
        }
        None => CheckOutcome::Clean,
    }
}

fn hard_mode_rewrites(hard_mode: &str) -> bool {
    hard_mode == "rewrite"
}

fn main() {
    let port: u16 = env_u64("OMEGAFLOW_LLM_PORT", 4100) as u16;
    let upstream = env_str("OMEGAFLOW_LLM_UPSTREAM", "https://api.deepseek.com");
    let key = resolve_key();
    let root = env_str("OMEGAFLOW_ROOT", ".");
    let session = env_str("OMEGAFLOW_SESSION", "operator");
    let ledger = env_str(
        "OMEGAFLOW_GATE_LEDGER",
        &format!("{}/phi/llm_gate_ledger.φ", root),
    );
    let hard_mode = env_str("OMEGAFLOW_HARD_MODE", "passthrough");
    let mut gate = Gate::new(&session, &ledger);
    if let Ok(text) = std::fs::read_to_string(format!("{}/phi/sources.φ", root)) {
        gate.learn_sources(&text);
    }
    gate.learn_register(&root);
    let model_path = env_str("OMEGAFLOW_MODELS", &format!("{}/phi/models.φ", root));
    let models = match std::fs::read_to_string(&model_path) {
        Ok(text) => parse_model_register(&text),
        Err(_) => HashMap::new(),
    };
    let state_path = format!("{}/phi/reports/gate_state.φ", root);
    let init_state = match state::read(&state_path) {
        Some(s) if s.session == session => s,
        _ => State::fresh(&session),
    };
    let _ = state::write(&state_path, &init_state);
    let perm_path = env_str(
        "OMEGAFLOW_TOOL_PERM",
        &format!("{}/phi/llm_tool_permission.φ", root),
    );
    let perm = ToolPerm::load(&perm_path);
    let shared = Arc::new(Shared {
        gate: Mutex::new(gate),
        friction: Mutex::new(Friction::new()),
        models,
        reported_pending: Mutex::new(HashSet::new()),
        handover_pending: Mutex::new(None),
        handover_written: Mutex::new(false),
        state: Mutex::new(init_state),
        perm: Mutex::new(perm),
        seen_tool_ids: Mutex::new(HashSet::new()),
    });
    let listener = match TcpListener::bind(format!("127.0.0.1:{}", port)) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("llm-interceptor: bind 127.0.0.1:{} refused: {}", port, e);
            return;
        }
    };
    let learned_rules = shared.gate.lock().unwrap().learned_rules.len();
    let register_len = shared.gate.lock().unwrap().register.len();
    let tool_entries = shared.perm.lock().unwrap().tools.len();
    eprintln!(
        "llm-interceptor: the gate listens on 127.0.0.1:{} (upstream {}, {} register values, {} learned rules)",
        port, upstream, register_len, learned_rules
    );
    eprintln!(
        "llm-interceptor: tool permission ledger at {} ({} entries)",
        perm_path, tool_entries
    );
    for conn in listener.incoming() {
        let Ok(mut stream) = conn else { continue };
        let shared = shared.clone();
        let upstream = upstream.clone();
        let key = key.clone();
        let root = root.clone();
        let hard_mode = hard_mode.clone();
        std::thread::spawn(move || {
            stream.set_read_timeout(Some(Duration::from_secs(120))).ok();
            let Some((method, path, headers, body)) = read_request(&mut stream) else {
                return;
            };
            let auth_header = headers
                .iter()
                .find(|(k, _)| k == "authorization")
                .map(|(_, v)| v.clone());
            if method == "GET" && path.starts_with("/health") {
                let ok = b"gate ok\n";
                let head = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                    ok.len()
                );
                let _ = stream.write_all(head.as_bytes());
                let _ = stream.write_all(ok);
                return;
            }
            if method != "POST" || !path.ends_with("/chat/completions") {
                let _ = stream.write_all(
                    b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
                );
                return;
            }
            handle_chat(
                &mut stream,
                &body,
                &shared,
                &upstream,
                &key,
                auth_header.as_deref(),
                &root,
                &hard_mode,
            );
        });
    }
}

#[allow(clippy::too_many_arguments)]
fn handle_chat(
    stream: &mut TcpStream,
    body: &[u8],
    shared: &Arc<Shared>,
    upstream: &str,
    key: &str,
    auth_header: Option<&str>,
    root: &str,
    hard_mode: &str,
) {
    let body_text = String::from_utf8_lossy(body).to_string();
    let Some(parsed) = parse_json(&body_text) else {
        let head = "HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nTransfer-Encoding: chunked\r\nCache-Control: no-store\r\nConnection: close\r\n\r\n";
        if stream.write_all(head.as_bytes()).is_err() {
            return;
        }
        if let Some(full) = run_curl_full(upstream, key, body, auth_header) {
            let _ = write_chunk(stream, full.as_bytes());
        }
        let _ = write_chunk(stream, b"");
        return;
    };
    let streaming = json_bool(&parsed, "stream");
    let model_raw = json_str_field(&parsed, "model").unwrap_or_default();
    let model = normalize_model(&model_raw);
    if !model.is_empty() && !shared.models.is_empty() && !shared.models.contains_key(&model) {
        let _ = stream.write_all(
            b"HTTP/1.1 400 Bad Request\r\nContent-Type: text/plain\r\nConnection: close\r\n\r\n",
        );
        let _ = stream.write_all(
            format!(
                "the gate serves only the models it knows — \"{}\" carries no entry in phi/models.φ\n",
                model
            )
            .as_bytes(),
        );
        let mut gate = shared.gate.lock().unwrap();
        gate.record_violation(
            "gate/model-refused",
            &format!("model \"{}\" carries no entry in phi/models.φ", model),
        );
        return;
    }
    if !model.is_empty() && shared.models.contains_key(&model) && model != "deepseek-v4-pro" {
        let mut reported = shared.reported_pending.lock().unwrap();
        if reported.insert(model.clone()) {
            eprintln!(
                "llm-interceptor: the session runs {} — the operator named omegaflow/deepseek-v4-pro as the gate-complete model",
                model
            );
        }
    }
    let head = "HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nTransfer-Encoding: chunked\r\nCache-Control: no-store\r\nConnection: close\r\n\r\n";
    if stream.write_all(head.as_bytes()).is_err() {
        return;
    }
    let state_path = format!("{}/phi/reports/gate_state.φ", root);
    let messages = parsed.clone();
    let last_user = last_user_content(&parsed);
    learn_tool_results(shared, &parsed);
    let handover_to_spawn = {
        if !consent_word(&last_user) {
            None
        } else {
            let mut pending = shared.handover_pending.lock().unwrap();
            pending.take()
        }
    };
    if let Some(path) = handover_to_spawn {
        if let Ok(content) = std::fs::read_to_string(&path) {
            let argv = spawn_command(&content, &model_raw, root);
            if !argv.is_empty() {
                let log = format!("{}/phi/reports/spawn-{}.log", root, std::process::id());
                let out = std::fs::File::create(&log)
                    .unwrap_or_else(|_| std::fs::File::open("/dev/null").expect("devnull"));
                let _ = Command::new(&argv[0])
                    .args(&argv[1..])
                    .stdin(Stdio::null())
                    .stdout(Stdio::from(out))
                    .stderr(Stdio::inherit())
                    .spawn();
            }
        }
    }
    let model_known = {
        let mut friction = shared.friction.lock().unwrap();
        friction.set_model(&model, &shared.models)
    };
    if !model_known && !model.is_empty() {
        let mut reported = shared.reported_pending.lock().unwrap();
        if reported.insert(model.clone()) {
            drop(reported);
            let mut gate = shared.gate.lock().unwrap();
            gate.record_violation(
                "gate/model-pending",
                &format!("model \"{}\" carries no entry in phi/models.φ", model),
            );
            drop(gate);
        }
    }
    let forward_bytes: Vec<u8> = {
        let mut gate = shared.gate.lock().unwrap();
        let mut findings: Vec<(String, String, String)> = Vec::new();
        for text in message_texts(&parsed) {
            for v in gate.check_input(&text) {
                gate.record_violation(&format!("input/{}", v.rule), &v.quote);
                findings.push((v.rule, v.feedback, v.quote));
                if findings.len() >= 6 {
                    break;
                }
            }
            if findings.len() >= 6 {
                break;
            }
        }
        drop(gate);
        let mut mutated = with_include_usage(parsed.clone(), streaming);
        axioms::inject(&mut mutated, &root);
        {
            let mut st = shared.state.lock().unwrap();
            st.turns += 1;
            if !model.is_empty() {
                st.model = model.clone();
            }
            state::inject_after_axioms(&mut mutated, &st);
        }
        {
            let perm = shared.perm.lock().unwrap();
            let msg = perm.system_message();
            drop(perm);
            if let Some(content) = msg {
                if let JsonVal::Obj(map) = &mut mutated {
                    if let Some(JsonVal::Arr(msgs)) = map.get_mut("messages") {
                        msgs.push(JsonVal::Obj({
                            let mut m = HashMap::new();
                            m.insert("role".to_string(), JsonVal::Str("system".to_string()));
                            m.insert("content".to_string(), JsonVal::Str(content));
                            m
                        }));
                    }
                }
            }
        }
        if !findings.is_empty() {
            if let JsonVal::Obj(map) = &mut mutated {
                if let Some(JsonVal::Arr(msgs)) = map.get_mut("messages") {
                    msgs.push(quarantine_system_message(&findings));
                }
            }
        }
        json_write(&mutated).into_bytes()
    };
    if !streaming {
        if let Some(full) = run_curl_full(upstream, key, &forward_bytes, auth_header) {
            let mut gate = shared.gate.lock().unwrap();
            if let Some(v) = gate.check_text(&full) {
                gate.record_violation(&v.rule, &v.quote);
            }
            drop(gate);
            let _ = write_chunk(stream, full.as_bytes());
            let _ = write_chunk(stream, b"");
        }
        finish_state(shared, root, &state_path, None);
        return;
    }
    let Some(mut child) = spawn_curl(upstream, key, &forward_bytes, auth_header) else {
        let _ = write_chunk(stream, sse_done().as_bytes());
        let _ = write_chunk(stream, b"");
        return;
    };
    let mut rewrites = 0u32;
    let mut text_acc = String::new();
    let mut partial = String::new();
    let mut tool_states: HashMap<u32, (Option<String>, String)> = HashMap::new();
    let mut tool_buf: Vec<Vec<u8>> = Vec::new();
    'stream: loop {
        let mut reader = BufReader::new(match child.stdout.take() {
            Some(s) => s,
            None => break,
        });
        let mut line = String::new();
        loop {
            line.clear();
            match reader.read_line(&mut line) {
                Ok(0) => break,
                Ok(_) => {}
                Err(_) => break,
            }
            let trimmed = line.trim_end();
            if !trimmed.starts_with("data:") {
                if !write_chunk(stream, line.as_bytes()) {
                    return;
                }
                continue;
            }
            let payload = trimmed[5..].trim();
            if payload == "[DONE]" {
                break;
            }
            let Some(data) = parse_json(payload) else {
                if !write_chunk(stream, line.as_bytes()) {
                    return;
                }

                continue;
            };
            let info = delta_parts(&data);
            if let Some(pt) = info.usage_prompt {
                let mut friction = shared.friction.lock().unwrap();
                friction.note_usage(pt);
                drop(friction);
            }
            if info.finish.as_deref() == Some("tool_calls") {
                let mut gate = shared.gate.lock().unwrap();
                let mut hard_blocked = false;
                for (name, args) in tool_states.values() {
                    let Some(name) = name.as_ref() else { continue };
                    if let Some(v) = gate.check_tool_call(name, args) {
                        gate.record_violation(&v.rule, &v.quote);
                        if v.severity == Severity::Hard {
                            hard_blocked = true;
                        }
                    }
                }
                if hard_mode_rewrites(hard_mode) {
                    if hard_blocked {
                        drop(gate);
                        tool_buf.clear();
                        if !respawn(
                            stream,
                            &mut child,
                            &mut rewrites,
                            &messages,
                            &model_raw,
                            &partial,
                            upstream,
                            key,
                            auth_header,
                            &last_user,
                        ) {
                            return;
                        }
                        tool_states.clear();
                        continue 'stream;
                    }
                    for buf in tool_buf.drain(..) {
                        if !write_chunk(stream, &buf) {
                            return;
                        }
                    }
                } else if hard_blocked {
                    gate.record_note(
                        "gate/hard-passthrough",
                        "hard tool-call rule matched; the tool call was forwarded to the reader",
                    );
                }
                if !write_chunk(stream, line.as_bytes()) {
                    return;
                }
                {
                    let perm = shared.perm.lock().unwrap();
                    for (name, _) in tool_states.values() {
                        let Some(name) = name.as_ref() else { continue };
                        if perm.state_of(name) == ToolState::Denied {
                            gate.record_violation(
                                "tool-perm/denied",
                                &format!(
                                    "the model called \"{}\" though the ledger denies it",
                                    name
                                ),
                            );
                        }
                    }
                }
                drop(gate);
                let mut friction = shared.friction.lock().unwrap();
                let mut advisories: Vec<Advice> = Vec::new();
                for (name, args) in tool_states.values() {
                    let Some(name) = name.as_ref() else { continue };
                    friction.note_tool(args);
                    advisories.extend(friction.tool_friction(name, args));
                }
                drop(friction);
                let mut gate = shared.gate.lock().unwrap();
                for a in advisories {
                    match a {
                        Advice::Loop => {
                            gate.record_violation("reib/tool-loop", "repeated identical tool call")
                        }
                        Advice::Handover => {
                            gate.record_violation("reib/handover", "context-fill signals")
                        }
                        _ => {}
                    }
                }
                drop(gate);
                tool_states.clear();
                continue;
            }
            if let Some(content) = info.content.clone() {
                text_acc.push_str(&content);
                partial.push_str(&content);
                let boundary = text_acc.ends_with(['.', '!', '?', '\n'])
                    || text_acc.chars().count() >= 300
                    || info.finish.as_deref() == Some("stop");
                if boundary && !text_acc.trim().is_empty() {
                    let mut gate = shared.gate.lock().unwrap();
                    match check_window(&mut gate, &text_acc) {
                        CheckOutcome::Hard => {
                            if hard_mode_rewrites(hard_mode) {
                                drop(gate);
                                if !respawn(
                                    stream,
                                    &mut child,
                                    &mut rewrites,
                                    &messages,
                                    &model_raw,
                                    &partial,
                                    upstream,
                                    key,
                                    auth_header,
                                    &last_user,
                                ) {
                                    return;
                                }
                                text_acc.clear();
                                continue 'stream;
                            }
                            gate.record_note(
                                "gate/hard-passthrough",
                                "hard rule matched; the text was forwarded to the reader",
                            );
                            drop(gate);
                            if !write_chunk(stream, line.as_bytes()) {
                                return;
                            }
                            text_acc.clear();
                            continue;
                        }
                        CheckOutcome::Soft => {
                            drop(gate);
                            if !write_chunk(stream, line.as_bytes()) {
                                return;
                            }
                            text_acc.clear();
                            continue;
                        }
                        CheckOutcome::Clean => {
                            drop(gate);
                            let mut friction = shared.friction.lock().unwrap();
                            let advisories: Vec<Advice> = friction.check(&text_acc);
                            drop(friction);
                            let mut handover_reason = None;
                            let mut gate = shared.gate.lock().unwrap();
                            for a in advisories {
                                match a {
                                    Advice::Loop => gate.record_violation(
                                        "reib/loop",
                                        "the same statement repeats",
                                    ),
                                    Advice::Handover => {
                                        gate.record_violation(
                                            "reib/handover",
                                            "context-fill signals",
                                        );
                                        handover_reason = Some("limit phrase in model output");
                                    }
                                    _ => {}
                                }
                            }
                            drop(gate);
                            if let Some(reason) = handover_reason {
                                do_handover(
                                    shared,
                                    stream,
                                    root,
                                    reason,
                                    "the model spoke its own context loss",
                                );
                            }
                            text_acc.clear();
                        }
                    }
                }
                if !write_chunk(stream, line.as_bytes()) {
                    return;
                }

                continue;
            }
            if !info.tools.is_empty() {
                for (index, name, args) in info.tools {
                    match tool_states.get_mut(&index) {
                        Some((n, a)) => {
                            if name.is_some() {
                                *n = name;
                            }
                            a.push_str(&args);
                        }
                        None => {
                            tool_states.insert(index, (name, args));
                        }
                    }
                }
                if hard_mode_rewrites(hard_mode) {
                    tool_buf.push(line.as_bytes().to_vec());
                    continue;
                }
                if !write_chunk(stream, line.as_bytes()) {
                    return;
                }
                continue;
            }
            if !write_chunk(stream, line.as_bytes()) {
                return;
            }
        }
        break;
    }
    if !text_acc.trim().is_empty() {
        let mut gate = shared.gate.lock().unwrap();
        let _ = check_window(&mut gate, &text_acc);
        drop(gate);
    }
    let _ = write_chunk(stream, sse_done().as_bytes());
    let mut friction = shared.friction.lock().unwrap();
    let changes = git_changes(root);
    let commit = friction.commit_advice(&changes);
    let fill = friction.context_fill();
    let threshold = friction.threshold();
    let fill_advice = friction.check_fill();
    friction.end_turn();
    drop(friction);
    let mut handover_state = None;
    let mut gate = shared.gate.lock().unwrap();
    for a in fill_advice {
        match a {
            Advice::Handover => {
                gate.record_violation("reib/handover", "context-fill threshold reached");
                handover_state = Some(format!(
                    "context_fill {} ≥ threshold {} (model {})",
                    fill.map(|f| format!("{:.3}", f)).unwrap_or_default(),
                    threshold.map(|t| format!("{:.3}", t)).unwrap_or_default(),
                    model
                ));
            }
            _ => {}
        }
    }
    if let Some(advice) = commit {
        gate.record_violation("gate/commit-advice", &advice);
    }
    drop(gate);
    if let Some(state) = handover_state {
        do_handover(
            shared,
            stream,
            root,
            "context-fill threshold reached",
            &state,
        );
    }
    finish_state(shared, root, &state_path, fill);
    let _ = write_chunk(stream, b"");
}

#[allow(clippy::too_many_arguments)]
fn respawn(
    stream: &mut TcpStream,
    child: &mut Child,
    rewrites: &mut u32,
    messages: &JsonVal,
    model: &str,
    partial: &str,
    upstream: &str,
    key: &str,
    auth_header: Option<&str>,
    last_user: &str,
) -> bool {
    if *rewrites >= MAX_REWRITES {
        let _ = write_chunk(stream, sse_done().as_bytes());
        let _ = write_chunk(stream, b"");
        return false;
    }
    *rewrites += 1;
    let _ = child.kill();
    let context = if partial.is_empty() {
        last_user.to_string()
    } else {
        partial.to_string()
    };
    let new_body = intervention_body(messages, model, &context);
    match spawn_curl(upstream, key, new_body.as_bytes(), auth_header) {
        Some(new_child) => {
            *child = new_child;
            true
        }
        None => {
            let _ = write_chunk(stream, sse_done().as_bytes());
            let _ = write_chunk(stream, b"");
            false
        }
    }
}

fn last_user_content(parsed: &JsonVal) -> String {
    let JsonVal::Obj(map) = parsed else {
        return String::new();
    };
    let Some(JsonVal::Arr(messages)) = map.get("messages") else {
        return String::new();
    };
    for msg in messages.iter().rev() {
        let JsonVal::Obj(m) = msg else { continue };
        let is_user = matches!(m.get("role"), Some(JsonVal::Str(r)) if r == "user");
        if !is_user {
            continue;
        }
        match m.get("content") {
            Some(JsonVal::Str(s)) => return s.clone(),
            _ => continue,
        }
    }
    String::new()
}

fn message_texts(parsed: &JsonVal) -> Vec<String> {
    let mut out = Vec::new();
    let JsonVal::Obj(map) = parsed else {
        return out;
    };
    let Some(JsonVal::Arr(msgs)) = map.get("messages") else {
        return out;
    };
    for m in msgs {
        let JsonVal::Obj(m) = m else { continue };
        let is_scanned = match m.get("role") {
            Some(JsonVal::Str(r)) => r == "user" || r == "tool",
            _ => false,
        };
        if !is_scanned {
            continue;
        }
        match m.get("content") {
            Some(JsonVal::Str(s)) => out.push(s.clone()),
            Some(JsonVal::Arr(parts)) => {
                let mut text = String::new();
                for p in parts {
                    if let JsonVal::Obj(p) = p {
                        if let Some(JsonVal::Str(t)) = p.get("text") {
                            text.push_str(t);
                        }
                    }
                }
                if !text.is_empty() {
                    out.push(text);
                }
            }
            _ => {}
        }
    }
    out
}

fn learn_tool_results(shared: &Arc<Shared>, parsed: &JsonVal) {
    let JsonVal::Obj(root) = parsed else {
        return;
    };
    let Some(JsonVal::Arr(msgs)) = root.get("messages") else {
        return;
    };
    let mut id_names: HashMap<String, String> = HashMap::new();
    for m in msgs {
        let JsonVal::Obj(m) = m else { continue };
        let role = match m.get("role") {
            Some(JsonVal::Str(r)) => r.as_str(),
            _ => "",
        };
        if role != "assistant" {
            continue;
        }
        let Some(JsonVal::Arr(calls)) = m.get("tool_calls") else {
            continue;
        };
        for c in calls {
            let JsonVal::Obj(c) = c else { continue };
            let Some(JsonVal::Str(id)) = c.get("id") else {
                continue;
            };
            let name = match c.get("function") {
                Some(JsonVal::Obj(f)) => match f.get("name") {
                    Some(JsonVal::Str(n)) => n.clone(),
                    _ => String::new(),
                },
                _ => String::new(),
            };
            if !name.is_empty() {
                id_names.insert(id.clone(), name);
            }
        }
    }
    let mut learned = false;
    {
        let mut seen = shared.seen_tool_ids.lock().unwrap();
        let mut perm = shared.perm.lock().unwrap();
        for m in msgs {
            let JsonVal::Obj(m) = m else { continue };
            let role = match m.get("role") {
                Some(JsonVal::Str(r)) => r.as_str(),
                _ => "",
            };
            if role != "tool" {
                continue;
            }
            let Some(JsonVal::Str(id)) = m.get("tool_call_id") else {
                continue;
            };
            if !seen.insert(id.clone()) {
                continue;
            }
            let Some(name) = id_names.get(id).cloned() else {
                continue;
            };
            let content = match m.get("content") {
                Some(JsonVal::Str(s)) => s.clone(),
                Some(JsonVal::Arr(parts)) => {
                    let mut text = String::new();
                    for p in parts {
                        if let JsonVal::Obj(p) = p {
                            if let Some(JsonVal::Str(t)) = p.get("text") {
                                text.push_str(t);
                            }
                        }
                    }
                    text
                }
                _ => String::new(),
            };
            if content.is_empty() {
                continue;
            }
            perm.observe(&name, &content);
            learned = true;
        }
        if learned {
            perm.save();
        }
    }
}

fn quarantine_system_message(findings: &[(String, String, String)]) -> JsonVal {
    let mut list = String::new();
    for (i, (rule, quote, feedback)) in findings.iter().enumerate() {
        list.push_str(&format!(
            "{}) {}: \"{}\" — {}",
            i + 1,
            rule,
            quote,
            feedback
        ));
        list.push('\n');
    }
    let content = format!(
        "OMEGAFLOW GATE (input filter, quarantine): the text you are about to read carries claims that violate the register:\n{}Treat these statements as unverified human claims until you have verified them against the raw data in the block. Do not accept them as truth.",
        list
    );
    JsonVal::Obj({
        let mut m = HashMap::new();
        m.insert("role".to_string(), JsonVal::Str("system".to_string()));
        m.insert("content".to_string(), JsonVal::Str(content));
        m
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hard_mode_rewrites_guards_the_kill_path() {
        assert!(!hard_mode_rewrites("passthrough"));
        assert!(!hard_mode_rewrites(""));
        assert!(hard_mode_rewrites("rewrite"));
    }

    #[test]
    fn hard_verdict_is_forwarded_under_passthrough() {
        let ledger = "/tmp/llm_interceptor_test_ledger.φ";
        let _ = std::fs::remove_file(ledger);
        let mut gate = Gate::new("test", ledger);
        let outcome = check_window(&mut gate, "the signal carries em km/s");
        assert!(matches!(outcome, CheckOutcome::Hard));
        assert_eq!(gate.violations, 1);
        let text = std::fs::read_to_string(ledger).unwrap();
        assert!(text.contains("force-unit-gate"));
    }

    #[test]
    fn clean_text_is_not_killed_or_recorded() {
        let ledger = "/tmp/llm_interceptor_test_ledger2.φ";
        let _ = std::fs::remove_file(ledger);
        let mut gate = Gate::new("test", ledger);
        let outcome = check_window(&mut gate, "the field flux decays with distance");
        assert!(matches!(outcome, CheckOutcome::Clean));
        assert_eq!(gate.violations, 0);
        assert!(!std::path::Path::new(ledger).exists());
    }

    #[test]
    fn hard_passthrough_note_does_not_inflate_the_counter() {
        let ledger = "/tmp/llm_interceptor_test_ledger3.φ";
        let _ = std::fs::remove_file(ledger);
        let mut gate = Gate::new("test", ledger);
        let before = gate.violations;
        gate.record_note(
            "gate/hard-passthrough",
            "hard rule matched; the text was forwarded to the reader",
        );
        assert_eq!(gate.violations, before);
        let text = std::fs::read_to_string(ledger).unwrap();
        assert!(text.contains("gate/hard-passthrough"));
    }
}
