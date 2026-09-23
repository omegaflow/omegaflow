use std::collections::{HashMap, HashSet};
use std::env;
use std::io::{Read, Write};
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

const MODELS_TSV: &str = include_str!("../../free_models.tsv");

const TASK: &str = concat!(
    "Research the current obligations of five funding programs using your tools, then classify each.\n",
    "Use the bash tool to run web searches, for example:\n",
    "  archive_search --brave \"MacArthur Fellowship obligations\"\n",
    "and read a program's own page with:\n",
    "  archive_search --playwright <url-of-the-program-page>\n",
    "Classify each program by the obligations it imposes on the recipient.\n",
    "Answer with exactly one line per program, nothing else, in the format NAME|LABEL with no spaces around the pipe.\n",
    "LABEL is one of: free (no release, publication, or reporting obligations), duty (carries obligations), closed (not currently open).\n",
    "Programs: MacArthur Fellowship, Thiel Fellowship, Emergent Ventures, Astera Residency, Long-Term Future Fund.\n"
);

const T7_EXPECT: [&str; 5] = [
    "MacArthur Fellowship|free",
    "Thiel Fellowship|duty",
    "Emergent Ventures|free",
    "Astera Residency|duty",
    "Long-Term Future Fund|closed",
];

const FUNDING_RESEARCH_TASK: &str = concat!(
    "You are a researcher working for omegaflow, an independent open-source research system written in Rust: a WebGPU point cloud rendering an ICRS block universe (astronomy), plus a hardware companion device. The project has no institutional backing.\n",
    "Find concrete, currently open hardware-sponsorship and token/compute-funding routes that an unaffiliated individual open-source researcher can actually apply for. omegaflow needs single-board computers, dev kits, and sensors, and free API/GPU compute quotas to survive.\n",
    "Use your tools, for example:\n",
    "  archive_search --brave \"hardware sponsorship open source projects\"\n",
    "  archive_search --playwright <url-of-the-program-page>\n",
    "  sfetch <url>\n",
    "A route counts only if you have read its program page or another primary source and it names a way an individual without an institution can apply.\n",
    "Answer with exactly one line per route, nothing else, in the format\n",
    "ROUTE|WAS_ES_GIBT|ANTRAGSWEG_URL|FRIST|NAECHSTER_SCHRITT\n",
    "with no spaces around the pipes. ROUTE = program or provider name. WAS_ES_GIBT = what it gives (hardware, GPU credits, API tokens, compute). ANTRAGSWEG_URL = the exact application URL. FRIST = deadline, or rolling when there is none. NAECHSTER_SCHRITT = the next concrete step.\n",
    "After the route lines you may add a short justification naming the page where each route was verified.\n",
    "No generic advice, no vague programs, no fluff — only routes you verified on a page you actually read.\n"
);

enum Task {
    T7,
    FundingResearch,
    Custom { prompt: String },
}

impl Task {
    fn prompt(&self) -> String {
        match self {
            Task::T7 => TASK.to_string(),
            Task::FundingResearch => FUNDING_RESEARCH_TASK.to_string(),
            Task::Custom { prompt } => prompt.clone(),
        }
    }

    fn scoring(&self) -> bool {
        matches!(self, Task::T7)
    }
}

fn resolve_task(task_name: Option<String>, task_file: Option<String>) -> Task {
    if let Some(path) = task_file {
        if task_name.is_some() {
            eprintln!("free_model_agent_bench: --task and --task-file are mutually exclusive");
            std::process::exit(2);
        }
        let content = match std::fs::read_to_string(&path) {
            Ok(c) => c,
            Err(e) => {
                eprintln!(
                    "free_model_agent_bench: cannot read --task-file {}: {}",
                    path, e
                );
                std::process::exit(2);
            }
        };
        if content.trim().is_empty() {
            eprintln!("free_model_agent_bench: --task-file {} is empty", path);
            std::process::exit(2);
        }
        return Task::Custom {
            prompt: content.trim().to_string(),
        };
    }
    match task_name.as_deref() {
        None | Some("t7") => Task::T7,
        Some("funding-research") => Task::FundingResearch,
        Some(other) => {
            eprintln!(
                "free_model_agent_bench: unknown --task '{}' (known: t7, funding-research)",
                other
            );
            std::process::exit(2);
        }
    }
}

fn parse_models_shard(spec: &str) -> Result<(usize, usize), String> {
    let (index_str, count_str) = spec
        .split_once(':')
        .ok_or_else(|| format!("--models-shard expects i:N, got '{}'", spec))?;
    let index: usize = index_str.parse().map_err(|_| {
        format!(
            "--models-shard index is not a non-negative integer: '{}'",
            index_str
        )
    })?;
    let count: usize = count_str.parse().map_err(|_| {
        format!(
            "--models-shard count is not a non-negative integer: '{}'",
            count_str
        )
    })?;
    if count < 2 {
        return Err(format!("--models-shard count must be >= 2, got {}", count));
    }
    if index >= count {
        return Err(format!(
            "--models-shard index must satisfy 0 <= i < N, got {}:{}",
            index, count
        ));
    }
    Ok((index, count))
}

struct Model {
    provider: String,
    id: String,
}

struct ModelFull {
    provider: String,
    id: String,
    base: String,
    env_var: String,
}

struct Row {
    status: String,
    ms: u128,
    tool_calls: String,
    answer: String,
}

struct Accum {
    parts: HashMap<String, String>,
    part_order: Vec<String>,
    seen_any_event: bool,
    seen_tool_part: bool,
    archive_calls: HashSet<String>,
    archive_calls_anon: usize,
}

impl Accum {
    fn new() -> Accum {
        Accum {
            parts: HashMap::new(),
            part_order: Vec::new(),
            seen_any_event: false,
            seen_tool_part: false,
            archive_calls: HashSet::new(),
            archive_calls_anon: 0,
        }
    }
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

fn utf8_len(lead: u8) -> usize {
    match lead {
        0x00..=0x7F => 1,
        0xC0..=0xDF => 2,
        0xE0..=0xEF => 3,
        0xF0..=0xF7 => 4,
        _ => 1,
    }
}

fn json_string_at(hay: &str) -> Option<(String, usize)> {
    let b = hay.as_bytes();
    if b.first() != Some(&b'"') {
        return None;
    }
    let mut out = String::new();
    let mut i = 1;
    while i < b.len() {
        match b[i] {
            b'"' => return Some((out, i + 1)),
            b'\\' => {
                if i + 1 >= b.len() {
                    return None;
                }
                match b[i + 1] {
                    b'n' => {
                        out.push('\n');
                        i += 2;
                    }
                    b'r' => {
                        out.push('\r');
                        i += 2;
                    }
                    b't' => {
                        out.push('\t');
                        i += 2;
                    }
                    b'b' => {
                        out.push('\u{8}');
                        i += 2;
                    }
                    b'f' => {
                        out.push('\u{c}');
                        i += 2;
                    }
                    b'u' => {
                        if i + 6 > b.len() {
                            return None;
                        }
                        let hex = match std::str::from_utf8(&b[i + 2..i + 6]) {
                            Ok(h) => h,
                            Err(_) => return None,
                        };
                        let code = match u32::from_str_radix(hex, 16) {
                            Ok(c) => c,
                            Err(_) => return None,
                        };
                        match char::from_u32(code) {
                            Some(c) => {
                                out.push(c);
                                i += 6;
                            }
                            None => return None,
                        }
                    }
                    c => {
                        out.push(c as char);
                        i += 2;
                    }
                }
            }
            c => {
                let len = utf8_len(c);
                if i + len > b.len() {
                    return None;
                }
                match std::str::from_utf8(&b[i..i + len]) {
                    Ok(s) => {
                        out.push_str(s);
                        i += len;
                    }
                    Err(_) => return None,
                }
            }
        }
    }
    None
}

fn balanced_object(hay: &str) -> Option<usize> {
    let b = hay.as_bytes();
    if b.first() != Some(&b'{') {
        return None;
    }
    let mut depth = 0usize;
    let mut i = 0;
    while i < b.len() {
        match b[i] {
            b'"' => {
                i += 1;
                let mut closed = false;
                while i < b.len() {
                    match b[i] {
                        b'\\' => i += 2,
                        b'"' => {
                            i += 1;
                            closed = true;
                            break;
                        }
                        _ => i += 1,
                    }
                }
                if !closed {
                    return None;
                }
            }
            b'{' | b'[' => {
                depth += 1;
                i += 1;
            }
            b'}' | b']' => {
                if depth == 0 {
                    return None;
                }
                depth -= 1;
                i += 1;
                if depth == 0 {
                    return Some(i);
                }
            }
            _ => i += 1,
        }
    }
    None
}

fn object_after<'a>(hay: &'a str, key: &str) -> Option<&'a str> {
    let marker = format!("\"{}\"", key);
    let pos = hay.find(&marker)?;
    let rest = &hay[pos + marker.len()..];
    let colon = rest.find(':')?;
    if !rest[..colon].trim().is_empty() {
        return None;
    }
    let after = rest[colon + 1..].trim_start();
    if after.starts_with('{') {
        Some(after)
    } else {
        None
    }
}

fn string_value_after(hay: &str, key: &str) -> Option<String> {
    let marker = format!("\"{}\":", key);
    let pos = hay.find(&marker)?;
    let after = hay[pos + marker.len()..].trim_start();
    if !after.starts_with('"') {
        return None;
    }
    json_string_at(after).map(|(v, _)| v)
}

fn absorb(line: &str, acc: &mut Accum) {
    let etype = match string_value_after(line, "type") {
        Some(t) => t,
        None => return,
    };
    match etype.as_str() {
        "step_start" | "text" | "tool_use" | "step_finish" => {}
        _ => return,
    }
    acc.seen_any_event = true;
    let part = match object_after(line, "part") {
        Some(p) => p,
        None => return,
    };
    if part.contains("\"type\":\"tool\"") {
        acc.seen_tool_part = true;
        let key = string_value_after(part, "callID").or_else(|| string_value_after(part, "id"));
        let is_bash = match string_value_after(part, "tool") {
            Some(t) => t == "bash",
            None => true,
        };
        let has_archive = match object_after(part, "input") {
            Some(input) => input.contains("archive_search"),
            None => false,
        };
        if has_archive && is_bash {
            match key {
                Some(k) => {
                    acc.archive_calls.insert(k);
                }
                None => {
                    acc.archive_calls_anon += 1;
                }
            }
        }
    } else if part.contains("\"type\":\"text\"") {
        let pid = string_value_after(part, "id");
        let text = string_value_after(part, "text");
        match (pid, text) {
            (Some(id), Some(t)) => {
                if !acc.part_order.contains(&id) {
                    acc.part_order.push(id.clone());
                }
                acc.parts.insert(id, t);
            }
            _ => {}
        }
    }
}

fn absorb_stream(output: &str, acc: &mut Accum) {
    let mut rest = output;
    while !rest.is_empty() {
        match rest.find('{') {
            Some(start) => match balanced_object(&rest[start..]) {
                Some(end) => {
                    let end = start + end;
                    absorb(&rest[start..end], acc);
                    rest = &rest[end..];
                }
                None => match rest[start..].find('\n') {
                    Some(nl) => rest = &rest[start + nl + 1..],
                    None => break,
                },
            },
            None => break,
        }
    }
}

fn answer_text(acc: &Accum) -> String {
    let mut s = String::new();
    for id in &acc.part_order {
        if let Some(t) = acc.parts.get(id) {
            s.push_str(t);
        }
    }
    s
}

fn score(text: &str) -> bool {
    let n = text.replace(" |", "|").replace("| ", "|");
    T7_EXPECT.iter().all(|k| n.contains(k)) && !n.contains("Thiel Fellowship|free")
}

fn tool_calls_for(acc: &Accum) -> String {
    if acc.seen_tool_part {
        (acc.archive_calls.len() + acc.archive_calls_anon).to_string()
    } else if acc.seen_any_event {
        "0".to_string()
    } else {
        "pending".to_string()
    }
}

fn clip(s: &str, max: usize) -> String {
    let flat: String = s
        .chars()
        .map(|c| if c == '\n' || c == '\r' { ' ' } else { c })
        .collect();
    if flat.len() <= max {
        return flat;
    }
    let mut start = flat.len() - max;
    while !flat.is_char_boundary(start) {
        start += 1;
    }
    flat[start..].to_string()
}

fn safe_file_part(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.' {
                c
            } else {
                '_'
            }
        })
        .collect()
}

fn read_all<R: Read>(mut r: R) -> String {
    let mut b = String::new();
    let _ = r.read_to_string(&mut b);
    b
}

fn parse_model_full(line: &str) -> Option<ModelFull> {
    let mut it = line.split('\t');
    let provider = it.next()?.to_string();
    let id = it.next()?.to_string();
    let base = it.next()?.to_string();
    let env_var = it.next()?.to_string();
    if provider.is_empty() || id.is_empty() || base.is_empty() || env_var.is_empty() {
        return None;
    }
    Some(ModelFull {
        provider,
        id,
        base,
        env_var,
    })
}

fn parse_model(line: &str) -> Option<Model> {
    parse_model_full(line).map(|f| Model {
        provider: f.provider,
        id: f.id,
    })
}

fn key_for(keys_json: &str, env_var: &str) -> Option<String> {
    if let Ok(v) = env::var(env_var) {
        if !v.is_empty() {
            return Some(v);
        }
    }
    match string_value_after(keys_json, env_var) {
        Some(v) if !v.is_empty() => Some(v),
        _ => None,
    }
}

fn emit_opencode_config(path: &str) -> i32 {
    let keys = match env::var("FREE_MODEL_KEYS") {
        Ok(v) if !v.trim().is_empty() => v,
        _ => {
            eprintln!(
                "free_model_agent_bench: FREE_MODEL_KEYS env is absent or empty — expected a JSON object mapping free_models.tsv env_var names to api keys"
            );
            return 2;
        }
    };
    let models: Vec<ModelFull> = MODELS_TSV.lines().filter_map(parse_model_full).collect();
    let mut providers: Vec<&ModelFull> = Vec::new();
    for m in &models {
        if !providers.iter().any(|p| p.provider == m.provider) {
            providers.push(m);
        }
    }
    let mut body = String::from("{\n  \"provider\": {\n");
    for (i, head) in providers.iter().enumerate() {
        let prov_models: Vec<&ModelFull> = models
            .iter()
            .filter(|m| m.provider == head.provider)
            .collect();
        if prov_models.iter().any(|m| m.base != head.base) {
            eprintln!(
                "free_model_agent_bench: provider {} carries conflicting baseURLs in free_models.tsv",
                head.provider
            );
            return 2;
        }
        body.push_str("    \"");
        body.push_str(&json_escape(&head.provider));
        body.push_str("\": {\n      \"npm\": \"@ai-sdk/openai-compatible\",\n      \"options\": {\"baseURL\": \"");
        body.push_str(&json_escape(&head.base));
        body.push('"');
        match key_for(&keys, &head.env_var) {
            Some(k) => {
                body.push_str(", \"apiKey\": \"");
                body.push_str(&json_escape(&k));
                body.push('"');
            }
            None => {}
        }
        body.push_str("},\n      \"models\": {");
        let mut first = true;
        for m in &prov_models {
            if !first {
                body.push(',');
            }
            first = false;
            body.push_str(" \"");
            body.push_str(&json_escape(&m.id));
            body.push_str("\": {\"name\": \"");
            body.push_str(&json_escape(&m.id));
            body.push_str("\", \"tool_call\": true}");
        }
        body.push_str(" }\n    }");
        if i + 1 < providers.len() {
            body.push(',');
        }
        body.push('\n');
    }
    body.push_str("  }\n}\n");
    match std::fs::write(path, body.as_bytes()) {
        Ok(()) => {
            eprintln!(
                "free_model_agent_bench: wrote opencode provider config {}",
                path
            );
            0
        }
        Err(e) => {
            eprintln!("free_model_agent_bench: cannot write {}: {}", path, e);
            2
        }
    }
}

fn run_one(m: &Model, dir: Option<&str>, timeout: Duration, task: &Task) -> Row {
    let prompt = task.prompt();
    let start = Instant::now();
    let model_arg = format!("{}/{}", m.provider, m.id);
    let mut cmd = Command::new("opencode");
    cmd.args([
        "run", "--pure", "--format", "json", "--model", &model_arg, "--agent", "plan",
    ]);
    if let Some(d) = dir {
        cmd.args(["--dir", d]);
    }
    cmd.arg(&prompt)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = match cmd.spawn() {
        Ok(c) => c,
        Err(_) => {
            return Row {
                status: "spawn_failed".into(),
                ms: start.elapsed().as_millis(),
                tool_calls: "pending".into(),
                answer: String::new(),
            };
        }
    };
    let stdout = child.stdout.take();
    let stderr = child.stderr.take();
    let out_handle = stdout.map(|s| thread::spawn(move || read_all(s)));
    let err_handle = stderr.map(|s| thread::spawn(move || read_all(s)));
    let mut timed_out = false;
    let mut exit_code: Option<i32> = None;
    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                exit_code = status.code();
                break;
            }
            Ok(None) => {
                if Instant::now() >= start + timeout {
                    timed_out = true;
                    let _ = child.kill();
                    let _ = child.wait();
                    break;
                }
                thread::sleep(Duration::from_millis(100));
            }
            Err(_) => {
                timed_out = true;
                let _ = child.kill();
                let _ = child.wait();
                break;
            }
        }
    }
    let ms = start.elapsed().as_millis();
    let out = match out_handle {
        Some(h) => match h.join() {
            Ok(s) => s,
            Err(_) => String::new(),
        },
        None => String::new(),
    };
    let err = match err_handle {
        Some(h) => match h.join() {
            Ok(s) => s,
            Err(_) => String::new(),
        },
        None => String::new(),
    };
    let mut acc = Accum::new();
    absorb_stream(&out, &mut acc);
    let trimmed = answer_text(&acc).trim().to_string();
    if timed_out {
        let keep = !trimmed.is_empty() && trimmed != prompt.trim();
        return Row {
            status: "timeout".into(),
            ms,
            tool_calls: tool_calls_for(&acc),
            answer: if keep { trimmed } else { String::new() },
        };
    }
    let echoed = trimmed == prompt.trim();
    let no_answer = trimmed.is_empty() || (echoed && (!task.scoring() || acc.parts.is_empty()));
    if no_answer {
        if trimmed.is_empty() && exit_code != Some(0) {
            let code_str = match exit_code {
                Some(c) => c.to_string(),
                None => "-".into(),
            };
            eprintln!("  exit={} stderr_tail={}", code_str, clip(&err, 240));
        }
        let status = if task.scoring() { "no_output" } else { "empty" };
        return Row {
            status: status.into(),
            ms,
            tool_calls: tool_calls_for(&acc),
            answer: String::new(),
        };
    }
    if task.scoring() {
        if score(&trimmed) {
            Row {
                status: "pass".into(),
                ms,
                tool_calls: tool_calls_for(&acc),
                answer: trimmed,
            }
        } else {
            eprintln!("  answer_tail={}", clip(&trimmed, 240));
            Row {
                status: "wrong_answer".into(),
                ms,
                tool_calls: tool_calls_for(&acc),
                answer: trimmed,
            }
        }
    } else {
        Row {
            status: "ok".into(),
            ms,
            tool_calls: tool_calls_for(&acc),
            answer: trimmed,
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut out = "free_model_agent_bench.tsv".to_string();
    let mut filter: Option<String> = None;
    let mut timeout: u64 = 300;
    let mut emit_config: Option<String> = None;
    let mut task_name: Option<String> = None;
    let mut task_file: Option<String> = None;
    let mut shard: Option<(usize, usize)> = None;
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
            "--timeout" => {
                if i + 1 < args.len() {
                    match args[i + 1].parse::<u64>() {
                        Ok(v) if v > 0 => timeout = v,
                        _ => {
                            eprintln!("free_model_agent_bench: --timeout expects seconds > 0");
                            std::process::exit(2);
                        }
                    }
                    i += 1;
                }
            }
            "--emit-config" => {
                if i + 1 < args.len() {
                    emit_config = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            "--task" => {
                if i + 1 < args.len() {
                    task_name = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            "--task-file" => {
                if i + 1 < args.len() {
                    task_file = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            "--models-shard" => {
                if i + 1 < args.len() {
                    match parse_models_shard(&args[i + 1]) {
                        Ok(s) => shard = Some(s),
                        Err(msg) => {
                            eprintln!("free_model_agent_bench: {}", msg);
                            std::process::exit(2);
                        }
                    }
                    i += 1;
                } else {
                    eprintln!("free_model_agent_bench: --models-shard expects i:N");
                    std::process::exit(2);
                }
            }
            _ => {}
        }
        i += 1;
    }
    if let Some(path) = emit_config {
        std::process::exit(emit_opencode_config(&path));
    }
    let task = resolve_task(task_name, task_file);
    let models: Vec<Model> = MODELS_TSV.lines().filter_map(parse_model).collect();
    if let Some((shard_index, shard_count)) = shard {
        eprintln!(
            "free_model_agent_bench: models shard {}/{} ({} of {} model slots)",
            shard_index,
            shard_count,
            models
                .iter()
                .enumerate()
                .filter(|(idx, _)| idx % shard_count == shard_index)
                .count(),
            models.len()
        );
    }
    let dir = env::current_dir()
        .ok()
        .map(|p| p.to_string_lossy().into_owned());
    let file = match std::fs::File::create(&out) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("free_model_agent_bench: cannot create {}: {}", out, e);
            std::process::exit(2);
        }
    };
    let mut file = file;
    let answers_dir = format!("{}.answers", out);
    if let Err(e) = std::fs::create_dir_all(&answers_dir) {
        eprintln!(
            "free_model_agent_bench: cannot create answers dir {}: {}",
            answers_dir, e
        );
        std::process::exit(2);
    }
    let _ = writeln!(
        file,
        "provider\tmodel\tstatus\tms\ttool_calls\tanswer_chars"
    );
    for (idx, m) in models.iter().enumerate() {
        if let Some((shard_index, shard_count)) = shard {
            if idx % shard_count != shard_index {
                continue;
            }
        }
        if let Some(f) = &filter {
            if !m.id.contains(f.as_str()) {
                continue;
            }
        }
        let row = run_one(m, dir.as_deref(), Duration::from_secs(timeout), &task);
        let answer_chars = row.answer.chars().count();
        eprintln!(
            "{} {} {} {}ms tool_calls={} answer_chars={}",
            m.provider, m.id, row.status, row.ms, row.tool_calls, answer_chars
        );
        if !row.answer.is_empty() {
            let name = format!(
                "{}__{}.md",
                safe_file_part(&m.provider),
                safe_file_part(&m.id)
            );
            let path = format!("{}/{}", answers_dir, name);
            if let Err(e) = std::fs::write(&path, row.answer.as_bytes()) {
                eprintln!(
                    "free_model_agent_bench: cannot write answer {}: {}",
                    path, e
                );
            }
        }
        let _ = writeln!(
            file,
            "{}\t{}\t{}\t{}\t{}\t{}",
            m.provider, m.id, row.status, row.ms, row.tool_calls, answer_chars
        );
        let _ = file.flush();
    }
    eprintln!("wrote {}", out);
}

#[cfg(test)]
mod tests {
    use super::*;

    const STREAM_PURE: &str = include_str!("../../fixtures/opencode_pure_stream.jsonl");
    const STREAM_TOOL: &str = include_str!("../../fixtures/opencode_tool_stream.jsonl");

    #[test]
    fn parser_extracts_final_answer_from_pure_stream() {
        let mut acc = Accum::new();
        absorb_stream(STREAM_PURE, &mut acc);
        assert_eq!(answer_text(&acc), "OK");
        assert_eq!(tool_calls_for(&acc), "0");
    }

    #[test]
    fn parser_counts_archive_call_and_extracts_final_answer_from_tool_stream() {
        let mut acc = Accum::new();
        absorb_stream(STREAM_TOOL, &mut acc);
        assert_eq!(answer_text(&acc), "DONE");
        assert_eq!(tool_calls_for(&acc), "1");
    }

    #[test]
    fn score_accepts_all_expected_t7_lines() {
        assert!(score(&T7_EXPECT.join("\n")));
    }

    #[test]
    fn unknown_events_leave_the_accumulator_pending() {
        let mut acc = Accum::new();
        absorb_stream("{\"type\":\"model_provider_error\",\"error\":{}}", &mut acc);
        assert_eq!(answer_text(&acc), "");
        assert_eq!(tool_calls_for(&acc), "pending");
    }

    #[test]
    fn models_shard_accepts_indices_below_the_shard_count() {
        assert_eq!(parse_models_shard("0:16"), Ok((0, 16)));
        assert_eq!(parse_models_shard("15:16"), Ok((15, 16)));
    }

    #[test]
    fn models_shard_rejects_out_of_range_and_malformed_specs() {
        assert!(parse_models_shard("16:16").is_err());
        assert!(parse_models_shard("1:1").is_err());
        assert!(parse_models_shard("16").is_err());
        assert!(parse_models_shard("x:16").is_err());
        assert!(parse_models_shard("1:x").is_err());
    }
}
