use crate::json::{self, Json};
use crate::net::json_escape;
use std::process::Command;

const MCP_URL: &str = "https://api.alphaxiv.org/mcp/v1";
const TAIL_MARK: &str = "\n__OMEGAFLOW_MCP__";

struct Call {
    status: String,
    body: String,
    session: Option<String>,
}

fn call(body: &str, key: &str, session: Option<&str>) -> Option<Call> {
    let auth = format!("Authorization: Bearer {key}");
    let mut cmd = Command::new("curl");
    cmd.arg("-sL")
        .arg("--max-time")
        .arg("60")
        .arg("-X")
        .arg("POST")
        .arg("-H")
        .arg("Content-Type: application/json")
        .arg("-H")
        .arg("Accept: application/json, text/event-stream")
        .arg("-H")
        .arg(&auth);
    if let Some(s) = session {
        if !s.is_empty() {
            cmd.arg("-H").arg(format!("Mcp-Session-Id: {s}"));
        }
    }
    cmd.arg("--data")
        .arg(body)
        .arg("-w")
        .arg(format!(
            "{TAIL_MARK}%{{http_code}}|%header{{mcp-session-id}}"
        ))
        .arg(MCP_URL);
    let out = cmd.output().ok()?;
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    let at = stdout.rfind(TAIL_MARK)?;
    let body = stdout[..at].to_string();
    let tail = &stdout[at + TAIL_MARK.len()..];
    let (status, session) = match tail.split_once('|') {
        Some((s, v)) => (s.trim().to_string(), v.trim().to_string()),
        None => (tail.trim().to_string(), String::new()),
    };
    Some(Call {
        status,
        body,
        session: if session.is_empty() {
            None
        } else {
            Some(session)
        },
    })
}

fn payloads(body: &str) -> Vec<Json> {
    let mut raw = Vec::new();
    let trimmed = body.trim();
    if trimmed.starts_with('{') || trimmed.starts_with('[') {
        if let Some(v) = json::parse(trimmed) {
            raw.push(v);
        }
    } else {
        let mut buf = String::new();
        for line in body.lines() {
            let line = line.trim_start();
            if let Some(rest) = line.strip_prefix("data:") {
                buf.push_str(rest.trim_start());
            } else if line.is_empty() && !buf.is_empty() {
                if let Some(v) = json::parse(&buf) {
                    raw.push(v);
                }
                buf.clear();
            }
        }
        if !buf.is_empty() {
            if let Some(v) = json::parse(&buf) {
                raw.push(v);
            }
        }
    }
    let mut out = Vec::new();
    for v in raw {
        match v {
            Json::Arr(items) => out.extend(items),
            other => out.push(other),
        }
    }
    out
}

fn error_text(v: &Json) -> Option<String> {
    if let Some(err) = v.get("error") {
        if let Some(s) = err.as_str() {
            return Some(s.to_string());
        }
        if let Some(m) = err.get("message").and_then(|x| x.as_str()) {
            return Some(m.to_string());
        }
        return Some("unnamed JSON-RPC error".to_string());
    }
    let result = v.get("result")?;
    if matches!(result.get("isError"), Some(Json::Bool(true))) {
        if let Some(items) = result.get("content").and_then(|c| c.as_arr()) {
            for item in items {
                if let Some(t) = item.get("text").and_then(|x| x.as_str()) {
                    return Some(t.to_string());
                }
            }
        }
        return Some("the tool reported an error without text".to_string());
    }
    None
}

fn flat(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn truncate(s: &str, n: usize) -> String {
    s.chars().take(n).collect()
}

fn str_field<'a>(o: &'a Json, keys: &[&str]) -> Option<&'a str> {
    for key in keys {
        if let Some(v) = o.get(key) {
            if let Some(s) = v.as_str() {
                if !s.is_empty() {
                    return Some(s);
                }
            }
            if let Some(a) = v.as_arr() {
                if let Some(s) = a.first().and_then(|x| x.as_str()) {
                    if !s.is_empty() {
                        return Some(s);
                    }
                }
            }
            if let Some(s) = v.get("value").and_then(|x| x.as_str()) {
                if !s.is_empty() {
                    return Some(s);
                }
            }
        }
    }
    None
}

fn entry_line(o: &Json) -> Option<String> {
    let url = str_field(
        o,
        &[
            "url",
            "link",
            "paper_url",
            "paperUrl",
            "abs_url",
            "absUrl",
            "html_url",
            "href",
        ],
    )?;
    let title = str_field(o, &["title", "name"]).unwrap_or("");
    let snippet = str_field(
        o,
        &[
            "abstract",
            "snippet",
            "summary",
            "description",
            "text",
            "content",
        ],
    )
    .unwrap_or("");
    Some(format!(
        "{} | {} | {}",
        url,
        flat(title),
        truncate(&flat(snippet), 300)
    ))
}

fn between<'a>(s: &'a str, open: &str, close: &str) -> Option<&'a str> {
    let start = s.find(open)? + open.len();
    let rest = &s[start..];
    let end = rest.find(close)?;
    Some(&rest[..end])
}

fn starts_entry(line: &str) -> bool {
    let t = line.trim_start();
    let digits: String = t.chars().take_while(|c| c.is_ascii_digit()).collect();
    if digits.is_empty() {
        return false;
    }
    t[digits.len()..].starts_with(". [")
}

fn parse_md_entry(entry: &str) -> Option<String> {
    let open = entry.find("(http")?;
    let rest = &entry[open + 1..];
    let close = rest.find(')')?;
    let url = &rest[..close];
    let after = &rest[close + 1..];
    let title = between(entry, "**", "**").unwrap_or("");
    let snippet = after.find(": ").map(|i| &after[i + 2..]).unwrap_or("");
    Some(format!(
        "{} | {} | {}",
        url,
        flat(title),
        truncate(&flat(snippet), 300)
    ))
}

fn markdown_entries(text: &str, max: usize) -> Vec<String> {
    let mut entries: Vec<String> = Vec::new();
    let mut current = String::new();
    for line in text.lines() {
        if starts_entry(line) {
            if !current.trim().is_empty() {
                entries.push(current.trim().to_string());
            }
            current = line.to_string();
        } else if !current.is_empty() {
            current.push(' ');
            current.push_str(line.trim());
        }
    }
    if !current.trim().is_empty() {
        entries.push(current.trim().to_string());
    }
    let mut out = Vec::new();
    for entry in entries.iter().take(max) {
        if let Some(line) = parse_md_entry(entry) {
            out.push(line);
        }
    }
    out
}

fn collect(v: &Json, out: &mut Vec<String>, max: usize) {
    if out.len() >= max {
        return;
    }
    match v {
        Json::Arr(items) => {
            for item in items {
                if out.len() >= max {
                    break;
                }
                collect(item, out, max);
            }
        }
        Json::Obj(_) => {
            if let Some(line) = entry_line(v) {
                out.push(line);
                return;
            }
            for key in [
                "results", "papers", "items", "entries", "hits", "data", "content", "text",
            ] {
                if let Some(child) = v.get(key) {
                    collect(child, out, max);
                    if out.len() >= max {
                        return;
                    }
                }
            }
        }
        Json::Str(s) => {
            if let Some(inner) = json::parse(s.trim()) {
                collect(&inner, out, max);
            } else {
                for line in markdown_entries(s, max - out.len()) {
                    out.push(line);
                }
            }
        }
        _ => {}
    }
}

fn tool_call_body(query: &str) -> String {
    let keywords: Vec<String> = query
        .split_whitespace()
        .map(|w| format!("\"{}\"", json_escape(w)))
        .collect();
    format!(
        "{{\"jsonrpc\":\"2.0\",\"id\":2,\"method\":\"tools/call\",\"params\":{{\"name\":\"discover_papers\",\"arguments\":{{\"keywords\":[{}],\"question\":\"{}\",\"difficulty\":1}}}}}}",
        keywords.join(","),
        json_escape(query)
    )
}

pub fn alphaxiv_lines(query: &str, key: &str, max: usize) -> Vec<String> {
    if key.is_empty() {
        return vec!["pending — ALPHAXIV_API_KEY absent from .secrets.local/.env".to_string()];
    }
    let init = "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"initialize\",\"params\":{\"protocolVersion\":\"2025-06-18\",\"capabilities\":{},\"clientInfo\":{\"name\":\"archive_search\",\"version\":\"1\"}}}";
    let Some(init_resp) = call(init, key, None) else {
        return vec!["pending — no network".to_string()];
    };
    if !init_resp.status.starts_with('2') {
        let detail = flat(&init_resp.body);
        return vec![format!(
            "pending — alphaxiv initialize HTTP {}: {}",
            init_resp.status,
            truncate(&detail, 300)
        )];
    }
    for p in &payloads(&init_resp.body) {
        if let Some(err) = error_text(p) {
            return vec![format!("alphaxiv error: {}", flat(&err))];
        }
    }
    let session = init_resp.session;
    let note = "{\"jsonrpc\":\"2.0\",\"method\":\"notifications/initialized\"}";
    let _ = call(note, key, session.as_deref());
    let body = tool_call_body(query);
    let Some(call_resp) = call(&body, key, session.as_deref()) else {
        return vec!["pending — no network".to_string()];
    };
    if !call_resp.status.starts_with('2') {
        let detail = flat(&call_resp.body);
        return vec![format!(
            "pending — alphaxiv HTTP {}: {}",
            call_resp.status,
            truncate(&detail, 300)
        )];
    }
    let parsed = payloads(&call_resp.body);
    for p in &parsed {
        if let Some(err) = error_text(p) {
            return vec![format!("alphaxiv error: {}", flat(&err))];
        }
    }
    let mut out = Vec::new();
    for p in &parsed {
        if let Some(result) = p.get("result") {
            collect(result, &mut out, max);
        }
        if out.len() >= max {
            break;
        }
    }
    if out.is_empty() {
        let raw = flat(&call_resp.body);
        return vec![format!(
            "pending — the alphaxiv response carries no parseable entry: {}",
            truncate(&raw, 400)
        )];
    }
    out.truncate(max);
    out
}

fn researchers_call_body(query: &str, limit: usize) -> String {
    let limit = limit.clamp(1, 100);
    format!(
        "{{\"jsonrpc\":\"2.0\",\"id\":2,\"method\":\"tools/call\",\"params\":{{\"name\":\"find_researchers\",\"arguments\":{{\"query\":\"{}\",\"limit\":{}}}}}}}",
        json_escape(query),
        limit
    )
}

fn researcher_line(o: &Json) -> Option<String> {
    let slug = str_field(o, &["slug", "handle", "researcher_slug", "researcherSlug"]);
    let url = str_field(
        o,
        &["url", "profile_url", "profileUrl", "alphaxiv_url", "href"],
    )
    .map(str::to_string)
    .or_else(|| slug.map(|s| format!("https://www.alphaxiv.org/@{s}")))?;
    let name = str_field(
        o,
        &[
            "name",
            "display_name",
            "displayName",
            "full_name",
            "fullName",
            "title",
        ],
    )?;
    let position = str_field(
        o,
        &[
            "current_position",
            "currentPosition",
            "position",
            "affiliation",
            "current_affiliation",
            "institution",
            "organization",
        ],
    );
    let citations = o
        .get("citations")
        .or_else(|| o.get("citation_count"))
        .or_else(|| o.get("citationCount"))
        .and_then(Json::as_scalar_string);
    let mut parts: Vec<String> = Vec::new();
    if let Some(p) = position {
        if !p.is_empty() {
            parts.push(flat(p));
        }
    }
    if let Some(c) = citations {
        parts.push(format!("citations {c}"));
    }
    Some(format!(
        "{} | {} | {}",
        url,
        flat(name),
        truncate(&parts.join(" · "), 300)
    ))
}

fn researcher_md_entries(text: &str, max: usize) -> Vec<String> {
    let mut out = Vec::new();
    for line in text.lines() {
        if out.len() >= max {
            break;
        }
        let t = line.trim();
        if t.is_empty() {
            continue;
        }
        let slug = between(t, "SLUG=", "]").or_else(|| between(t, "SLUG=", " "));
        let Some(slug) = slug else {
            out.push(truncate(&flat(t), 300));
            continue;
        };
        let url = format!("https://www.alphaxiv.org/@{slug}");
        let cleaned = match (t.find("[SLUG="), t.find(']')) {
            (Some(a), Some(b)) if b > a => format!("{}{}", &t[..a], &t[b + 1..]),
            _ => t.to_string(),
        };
        out.push(format!(
            "{} | {}",
            url,
            truncate(&flat(cleaned.trim()), 300)
        ));
    }
    out
}

fn collect_researchers(v: &Json, out: &mut Vec<String>, max: usize) {
    if out.len() >= max {
        return;
    }
    match v {
        Json::Arr(items) => {
            for item in items {
                if out.len() >= max {
                    break;
                }
                collect_researchers(item, out, max);
            }
        }
        Json::Obj(_) => {
            if let Some(line) = researcher_line(v) {
                out.push(line);
                return;
            }
            for key in [
                "results",
                "researchers",
                "items",
                "entries",
                "hits",
                "data",
                "content",
                "text",
            ] {
                if let Some(child) = v.get(key) {
                    collect_researchers(child, out, max);
                    if out.len() >= max {
                        return;
                    }
                }
            }
        }
        Json::Str(s) => {
            if let Some(inner) = json::parse(s.trim()) {
                collect_researchers(&inner, out, max);
            } else {
                for line in researcher_md_entries(s, max - out.len()) {
                    out.push(line);
                }
            }
        }
        _ => {}
    }
}

pub fn alphaxiv_researchers_lines(query: &str, key: &str, max: usize) -> Vec<String> {
    if key.is_empty() {
        return vec!["pending — ALPHAXIV_API_KEY absent from .secrets.local/.env".to_string()];
    }
    let init = "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"initialize\",\"params\":{\"protocolVersion\":\"2025-06-18\",\"capabilities\":{},\"clientInfo\":{\"name\":\"archive_search\",\"version\":\"1\"}}}";
    let Some(init_resp) = call(init, key, None) else {
        return vec!["pending — no network".to_string()];
    };
    if !init_resp.status.starts_with('2') {
        let detail = flat(&init_resp.body);
        return vec![format!(
            "pending — alphaxiv initialize HTTP {}: {}",
            init_resp.status,
            truncate(&detail, 300)
        )];
    }
    for p in &payloads(&init_resp.body) {
        if let Some(err) = error_text(p) {
            return vec![format!("alphaxiv error: {}", flat(&err))];
        }
    }
    let session = init_resp.session;
    let note = "{\"jsonrpc\":\"2.0\",\"method\":\"notifications/initialized\"}";
    let _ = call(note, key, session.as_deref());
    let body = researchers_call_body(query, max);
    let Some(call_resp) = call(&body, key, session.as_deref()) else {
        return vec!["pending — no network".to_string()];
    };
    if !call_resp.status.starts_with('2') {
        let detail = flat(&call_resp.body);
        return vec![format!(
            "pending — alphaxiv HTTP {}: {}",
            call_resp.status,
            truncate(&detail, 300)
        )];
    }
    let parsed = payloads(&call_resp.body);
    for p in &parsed {
        if let Some(err) = error_text(p) {
            return vec![format!("alphaxiv error: {}", flat(&err))];
        }
    }
    let mut out = Vec::new();
    for p in &parsed {
        if let Some(result) = p.get("result") {
            collect_researchers(result, &mut out, max);
        }
        if out.len() >= max {
            break;
        }
    }
    if out.is_empty() {
        let raw = flat(&call_resp.body);
        return vec![format!(
            "pending — the alphaxiv response carries no parseable researcher: {}",
            truncate(&raw, 400)
        )];
    }
    out.truncate(max);
    out
}

const WRITE_TOOLS: [&str; 9] = [
    "follow_researcher",
    "unfollow_researcher",
    "save_papers_to_folder",
    "remove_papers_from_folder",
    "move_papers_between_folders",
    "create_folder",
    "rename_folder",
    "delete_folder",
    "edit_private_paper_metadata",
];

fn handshake(key: &str) -> Result<Option<String>, Vec<String>> {
    if key.is_empty() {
        return Err(vec![
            "pending — ALPHAXIV_API_KEY absent from .secrets.local/.env".to_string(),
        ]);
    }
    let init = "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"initialize\",\"params\":{\"protocolVersion\":\"2025-06-18\",\"capabilities\":{},\"clientInfo\":{\"name\":\"archive_search\",\"version\":\"1\"}}}";
    let Some(init_resp) = call(init, key, None) else {
        return Err(vec!["pending — no network".to_string()]);
    };
    if !init_resp.status.starts_with('2') {
        let detail = flat(&init_resp.body);
        return Err(vec![format!(
            "pending — alphaxiv initialize HTTP {}: {}",
            init_resp.status,
            truncate(&detail, 300)
        )]);
    }
    for p in &payloads(&init_resp.body) {
        if let Some(err) = error_text(p) {
            return Err(vec![format!("alphaxiv error: {}", flat(&err))]);
        }
    }
    let session = init_resp.session;
    let note = "{\"jsonrpc\":\"2.0\",\"method\":\"notifications/initialized\"}";
    let _ = call(note, key, session.as_deref());
    Ok(session)
}

fn text_content_lines(v: &Json, out: &mut Vec<String>, max: usize) {
    if out.len() >= max {
        return;
    }
    match v {
        Json::Arr(items) => {
            for item in items {
                if out.len() >= max {
                    break;
                }
                text_content_lines(item, out, max);
            }
        }
        Json::Obj(_) => {
            if let Some(t) = v.get("text").and_then(Json::as_str) {
                for line in t.lines() {
                    if out.len() >= max {
                        break;
                    }
                    let l = line.trim();
                    if !l.is_empty() {
                        out.push(truncate(&flat(l), 300));
                    }
                }
                return;
            }
            for key in [
                "content",
                "result",
                "data",
                "items",
                "papers",
                "researchers",
                "folders",
                "tools",
            ] {
                if let Some(child) = v.get(key) {
                    text_content_lines(child, out, max);
                    if out.len() >= max {
                        return;
                    }
                }
            }
        }
        _ => {}
    }
}

fn annotation_flag(t: &Json) -> &'static str {
    let Some(a) = t.get("annotations") else {
        return "read";
    };
    let destructive = matches!(a.get("destructiveHint"), Some(Json::Bool(true)));
    if destructive {
        return "destructive";
    }
    match a.get("readOnlyHint") {
        Some(Json::Bool(true)) => "read",
        Some(Json::Bool(false)) => "write",
        _ => "read",
    }
}

fn tools_list(key: &str, session: Option<&str>) -> Vec<(String, &'static str)> {
    let body = "{\"jsonrpc\":\"2.0\",\"id\":2,\"method\":\"tools/list\"}";
    let Some(resp) = call(body, key, session) else {
        return Vec::new();
    };
    if !resp.status.starts_with('2') {
        return Vec::new();
    }
    let mut out = Vec::new();
    for p in &payloads(&resp.body) {
        if let Some(tools) = p
            .get("result")
            .and_then(|r| r.get("tools"))
            .and_then(Json::as_arr)
        {
            for t in tools {
                let name = t.get("name").and_then(Json::as_str).unwrap_or("");
                if !name.is_empty() {
                    out.push((name.to_string(), annotation_flag(t)));
                }
            }
        }
    }
    out
}

pub fn alphaxiv_tools_lines(key: &str) -> Vec<String> {
    let session = match handshake(key) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let body = "{\"jsonrpc\":\"2.0\",\"id\":2,\"method\":\"tools/list\"}";
    let Some(resp) = call(body, key, session.as_deref()) else {
        return vec!["pending — no network".to_string()];
    };
    if !resp.status.starts_with('2') {
        let detail = flat(&resp.body);
        return vec![format!(
            "pending — alphaxiv tools/list HTTP {}: {}",
            resp.status,
            truncate(&detail, 300)
        )];
    }
    let parsed = payloads(&resp.body);
    for p in &parsed {
        if let Some(err) = error_text(p) {
            return vec![format!("alphaxiv error: {}", flat(&err))];
        }
    }
    let mut out = Vec::new();
    for p in &parsed {
        if let Some(tools) = p
            .get("result")
            .and_then(|r| r.get("tools"))
            .and_then(Json::as_arr)
        {
            for t in tools {
                let name = t.get("name").and_then(Json::as_str).unwrap_or("");
                if name.is_empty() {
                    continue;
                }
                out.push(format!("{} | {}", name, annotation_flag(t)));
            }
        }
    }
    if out.is_empty() {
        let raw = flat(&resp.body);
        return vec![format!(
            "pending — the alphaxiv tools/list carried no tool: {}",
            truncate(&raw, 400)
        )];
    }
    out
}

fn call_tool(spec: &str, key: &str) -> Result<(String, String), Vec<String>> {
    let (tool, args) = match spec.split_once(char::is_whitespace) {
        Some((t, a)) => (t.trim(), a.trim()),
        None => (spec.trim(), ""),
    };
    if tool.is_empty() {
        return Err(vec![
            "pending — --alphaxiv-call carries no tool name (form: --alphaxiv-call <tool> '<json-arguments>')"
                .to_string(),
        ]);
    }
    if WRITE_TOOLS.contains(&tool) {
        return Err(vec![format!(
            "refused — '{tool}' writes at alphaXiv; archive_search is read-only (a write is a per-act operator act)"
        )]);
    }
    let session = handshake(key)?;
    let tool_list = tools_list(key, session.as_deref());
    if !tool_list.is_empty() {
        match tool_list.iter().find(|(n, _)| n.as_str() == tool) {
            Some((_, "read")) => {}
            Some((_, flag)) => {
                return Err(vec![format!(
                    "refused — '{tool}' is {flag} at alphaXiv; archive_search is read-only (a write is a per-act operator act)"
                )]);
            }
            None => {
                return Err(vec![format!(
                    "pending — '{tool}' is not an alphaXiv MCP tool (see --alphaxiv-tools)"
                )]);
            }
        }
    }
    let args = if args.is_empty() { "{}" } else { args };
    let body = format!(
        "{{\"jsonrpc\":\"2.0\",\"id\":2,\"method\":\"tools/call\",\"params\":{{\"name\":\"{}\",\"arguments\":{}}}}}",
        json_escape(tool),
        args
    );
    let Some(resp) = call(&body, key, session.as_deref()) else {
        return Err(vec!["pending — no network".to_string()]);
    };
    if !resp.status.starts_with('2') {
        let detail = flat(&resp.body);
        return Err(vec![format!(
            "pending — alphaxiv HTTP {}: {}",
            resp.status,
            truncate(&detail, 300)
        )]);
    }
    let parsed = payloads(&resp.body);
    for p in &parsed {
        if let Some(err) = error_text(p) {
            return Err(vec![format!("alphaxiv error: {}", flat(&err))]);
        }
    }
    Ok((tool.to_string(), resp.body))
}

pub fn alphaxiv_call_lines(spec: &str, key: &str, max: usize) -> Vec<String> {
    let body = match call_tool(spec, key) {
        Ok((_, body)) => body,
        Err(e) => return e,
    };
    let parsed = payloads(&body);
    let mut out = Vec::new();
    for p in &parsed {
        if let Some(result) = p.get("result") {
            collect(result, &mut out, max);
        }
        if out.len() >= max {
            break;
        }
    }
    if out.is_empty() {
        for p in &parsed {
            if let Some(result) = p.get("result") {
                text_content_lines(result, &mut out, max);
            }
            if out.len() >= max {
                break;
            }
        }
    }
    if out.is_empty() {
        let raw = flat(&body);
        return vec![format!(
            "pending — the alphaxiv call carries no parseable text: {}",
            truncate(&raw, 400)
        )];
    }
    out.truncate(max);
    out
}

pub fn alphaxiv_call_text(spec: &str, key: &str) -> Result<(String, String), Vec<String>> {
    let (tool, body) = call_tool(spec, key)?;
    let parsed = payloads(&body);
    let mut out = Vec::new();
    for p in &parsed {
        if let Some(result) = p.get("result") {
            text_content_lines(result, &mut out, usize::MAX);
        }
    }
    if out.is_empty() {
        return Err(vec![format!(
            "pending — the alphaxiv call carries no parseable text: {}",
            truncate(&flat(&body), 400)
        )]);
    }
    Ok((tool, out.join("\n")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn payloads_read_sse_data_lines() {
        let body = "event: message\ndata: {\"jsonrpc\":\"2.0\",\"id\":2,\"result\":{}}\n\n";
        let parsed = payloads(body);
        assert_eq!(parsed.len(), 1);
        assert!(parsed[0].get("result").is_some());
    }

    #[test]
    fn payloads_expand_json_rpc_batches() {
        let body = "[{\"id\":1},{\"id\":2}]";
        assert_eq!(payloads(body).len(), 2);
    }

    #[test]
    fn markdown_entries_read_the_measured_list_shape() {
        let text = "1. [ID=2206.10173] **Lead-lag TE** (https://www.alphaxiv.org/abs/2206.10173). Published 2022-06-21 · 9 votes · 32 views: Symbolic transfer entropy detects lead-lag.\n2. [ID=2506.16215] **Transfer entropy for finite data** (https://www.alphaxiv.org/abs/2506.16215). Published 2025-06-19 · 0 votes · 22 views: Transfer entropy is a widely used measure.";
        let lines = markdown_entries(text, 5);
        assert_eq!(lines.len(), 2);
        assert_eq!(
            lines[0],
            "https://www.alphaxiv.org/abs/2206.10173 | Lead-lag TE | Symbolic transfer entropy detects lead-lag."
        );
        assert_eq!(
            lines[1],
            "https://www.alphaxiv.org/abs/2506.16215 | Transfer entropy for finite data | Transfer entropy is a widely used measure."
        );
    }

    #[test]
    fn markdown_entries_join_wrapped_snippet_lines() {
        let text = "1. [ID=1205.6339] **TE as LLR** (https://www.alphaxiv.org/abs/1205.6339). Published 2012-05-29 · 3 votes · 20 views: Transfer entropy is a measure of\ntime-directed information transfer.\n";
        let lines = markdown_entries(text, 5);
        assert_eq!(
            lines,
            vec![
                "https://www.alphaxiv.org/abs/1205.6339 | TE as LLR | Transfer entropy is a measure of time-directed information transfer.".to_string()
            ]
        );
    }

    #[test]
    fn entry_line_requires_a_url_and_skips_blanks() {
        let with_url =
            json::parse(r#"{"title":"T","url":"https://example.org/p","abstract":"A"}"#).unwrap();
        assert_eq!(
            entry_line(&with_url).as_deref(),
            Some("https://example.org/p | T | A")
        );
        let without_url = json::parse(r#"{"title":"T","abstract":"A"}"#).unwrap();
        assert!(entry_line(&without_url).is_none());
        let blank_url = json::parse(r#"{"title":"T","url":""}"#).unwrap();
        assert!(entry_line(&blank_url).is_none());
    }

    #[test]
    fn collect_reads_mcp_content_text_json() {
        let body = r#"{"content":[{"type":"text","text":"[{\"title\":\"T\",\"url\":\"https://example.org/p\"}]"}]}"#;
        let v = json::parse(body).unwrap();
        let mut out = Vec::new();
        collect(&v, &mut out, 5);
        assert_eq!(out, vec!["https://example.org/p | T | ".to_string()]);
    }

    #[test]
    fn error_text_reads_json_rpc_error_and_is_error() {
        let err = json::parse(r#"{"error":{"code":-32000,"message":"bad request"}}"#).unwrap();
        assert_eq!(error_text(&err).as_deref(), Some("bad request"));
        let tool =
            json::parse(r#"{"result":{"isError":true,"content":[{"type":"text","text":"boom"}]}}"#)
                .unwrap();
        assert_eq!(error_text(&tool).as_deref(), Some("boom"));
        let ok = json::parse(r#"{"result":{"content":[]}}"#).unwrap();
        assert_eq!(error_text(&ok), None);
    }

    #[test]
    fn researchers_call_body_names_find_researchers() {
        let body = researchers_call_body("transfer entropy", 12);
        assert!(body.contains("\"name\":\"find_researchers\""));
        assert!(body.contains("\"query\":\"transfer entropy\""));
        assert!(body.contains("\"limit\":12"));
    }

    #[test]
    fn researcher_line_builds_a_profile_url_from_slug() {
        let o = json::parse(
            r#"{"slug":"jane-doe","name":"Jane Doe","current_position":"ETH Zurich","citations":42}"#,
        )
        .unwrap();
        assert_eq!(
            researcher_line(&o).as_deref(),
            Some("https://www.alphaxiv.org/@jane-doe | Jane Doe | ETH Zurich · citations 42")
        );
    }

    #[test]
    fn collect_researchers_reads_mcp_content_text_json() {
        let body = r#"{"content":[{"type":"text","text":"[{\"slug\":\"jane-doe\",\"name\":\"Jane Doe\"}]"}]}"#;
        let v = json::parse(body).unwrap();
        let mut out = Vec::new();
        collect_researchers(&v, &mut out, 5);
        assert_eq!(
            out,
            vec!["https://www.alphaxiv.org/@jane-doe | Jane Doe | ".to_string()]
        );
    }

    #[test]
    fn researcher_md_entries_read_the_slug_shape() {
        let text = "1. [SLUG=jane-doe] Jane Doe (ETH Zurich) · 42 citations";
        let lines = researcher_md_entries(text, 5);
        assert_eq!(lines.len(), 1);
        assert!(lines[0].starts_with("https://www.alphaxiv.org/@jane-doe | "));
        assert!(lines[0].contains("Jane Doe (ETH Zurich)"));
    }

    #[test]
    fn alphaxiv_call_refuses_write_tools() {
        let lines = alphaxiv_call_lines("follow_researcher {\"slug\":\"x\"}", "", 5);
        assert_eq!(lines.len(), 1);
        assert!(lines[0].starts_with("refused — 'follow_researcher'"));
    }

    #[test]
    fn alphaxiv_call_without_a_tool_names_the_form() {
        let lines = alphaxiv_call_lines("", "", 5);
        assert!(lines[0].starts_with("pending — --alphaxiv-call carries no tool name"));
    }

    #[test]
    fn annotation_flag_reads_hints() {
        let read = json::parse(r#"{"annotations":{"readOnlyHint":true}}"#).unwrap();
        assert_eq!(annotation_flag(&read), "read");
        let write = json::parse(r#"{"annotations":{"readOnlyHint":false}}"#).unwrap();
        assert_eq!(annotation_flag(&write), "write");
        let del = json::parse(r#"{"annotations":{"destructiveHint":true}}"#).unwrap();
        assert_eq!(annotation_flag(&del), "destructive");
        let none = json::parse("{}").unwrap();
        assert_eq!(annotation_flag(&none), "read");
    }
}
