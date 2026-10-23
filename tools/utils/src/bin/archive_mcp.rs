use omegaflow_utils::json;


use std::env;
use std::io::{self, BufRead, Write};
use std::path::PathBuf;
use std::process::Command;
use std::sync::OnceLock;

const PROTOCOL: &str = "2025-03-26";
const VERSION: &str = "0.1.0";

const SEARCH_SCHEMA: &str = r#"{"type":"object","properties":{"query":{"type":"string","description":"Search terms or an archive identifier"},"source":{"type":"string","description":"One arm, e.g. simbad, vizier, mast, gaia, ned, ads, arxiv, openalex. Use 'all' for fan-out."},"group":{"type":"string","enum":["astro","literature","web","data"],"description":"Arm group; only used with source=all"},"limit":{"type":"integer","minimum":1,"maximum":50,"default":5},"since":{"type":"string","description":"Start date, YYYY or YYYY-MM-DD"},"until":{"type":"string","description":"End date, YYYY or YYYY-MM-DD"},"jobs":{"type":"integer","minimum":1,"maximum":16,"default":4},"sniff":{"type":"boolean","default":false,"description":"Check the top result URLs with archive_search --sniff (status, format, magic, header). Adds one HTTP request per checked URL."},"sniff_limit":{"type":"integer","minimum":1,"maximum":5,"default":3,"description":"How many top URLs to sniff when sniff=true"}},"required":["query"]}"#;

const SOURCES_SCHEMA: &str = r#"{"type":"object","properties":{"group":{"type":"string","enum":["astro","literature","web","data"],"description":"Only list arms in this group"}}}"#;

const TOOLS: &[(&str, &str, &str)] = &[
    (
        "archive_search",
        SEARCH_SCHEMA,
        "Search archive_search's keyless and keyed arms. Pass source for a single archive or source=all with group for a routed fan-out. Use archive_sources to discover arm names; keyless arms spend no API tokens.",
    ),
    (
        "archive_sources",
        SOURCES_SCHEMA,
        "List the available archive_search arms and their groups (astro, literature, web, data).",
    ),
];

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout().lock();
    for line in stdin.lock().lines() {
        let Ok(line) = line else {
            break;
        };
        if line.trim().is_empty() {
            continue;
        }
        let Some(request) = json::parse(&line) else {
            let response = error_response("null", -32700, "parse error");
            let _ = writeln!(stdout, "{response}");
            let _ = stdout.flush();
            continue;
        };
        let id = request.get("id");
        let Some(method) = request.get("method").and_then(|m| m.as_str()) else {
            if id.is_some() {
                let response = error_response(&id_text(id), -32600, "invalid request");
                let _ = writeln!(stdout, "{response}");
                let _ = stdout.flush();
            }
            continue;
        };
        match method {
            "initialize" => {
                let result = format!(
                    r#"{{"protocolVersion":"{PROTOCOL}","capabilities":{{"tools":{{"listChanged":false}}}},"serverInfo":{{"name":"archive-mcp","version":"{VERSION}"}}}}"#
                );
                respond(&mut stdout, id, &result);
            }
            "notifications/initialized" => {}
            "ping" => respond(&mut stdout, id, "{}"),
            "tools/list" => respond(&mut stdout, id, &tools_list()),
            "tools/call" => respond(&mut stdout, id, &tools_call(&request)),
            _ => {
                let response = error_response(&id_text(id), -32601, "method not found");
                let _ = writeln!(stdout, "{response}");
                let _ = stdout.flush();
            }
        }
    }
}

fn respond(stdout: &mut impl Write, id: Option<&json::Json>, result: &str) {
    let response = format!(
        r#"{{"jsonrpc":"2.0","id":{},"result":{result}}}"#,
        id_text(id)
    );
    let _ = writeln!(stdout, "{response}");
    let _ = stdout.flush();
}

fn error_response(id: &str, code: i32, message: &str) -> String {
    format!(
        r#"{{"jsonrpc":"2.0","id":{id},"error":{{"code":{code},"message":"{}"}}}}"#,
        escape(message)
    )
}

fn tools_list() -> String {
    let tools = TOOLS
        .iter()
        .map(|(name, schema, description)| {
            format!(
                r#"{{"name":"{name}","description":"{}","inputSchema":{schema}}}"#,
                escape(description)
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    format!(r#"{{"tools":[{tools}]}}"#)
}

fn tools_call(request: &json::Json) -> String {
    let Some(params) = request.get("params") else {
        return error_result("missing params");
    };
    let Some(name) = params.get("name").and_then(|n| n.as_str()) else {
        return error_result("missing tool name");
    };
    match name {
        "archive_search" => search_call(params),
        "archive_sources" => sources_call(params),
        _ => error_result(&format!("unknown tool: {name}")),
    }
}

fn search_call(params: &json::Json) -> String {
    let Some(arguments) = params.get("arguments") else {
        return error_result("missing arguments");
    };
    let Some(query) = arguments.get("query").and_then(|q| q.as_str()) else {
        return error_result("missing query");
    };
    if query.trim().is_empty() {
        return error_result("empty query");
    }
    let source = arguments
        .get("source")
        .and_then(|s| s.as_str())
        .unwrap_or("")
        .trim();
    let group = arguments
        .get("group")
        .and_then(|g| g.as_str())
        .unwrap_or("")
        .trim();
    let limit = arguments
        .get("limit")
        .and_then(as_usize)
        .unwrap_or(5)
        .clamp(1, 50);
    let jobs = arguments
        .get("jobs")
        .and_then(as_usize)
        .unwrap_or(4)
        .clamp(1, 16);
    let sniff = matches!(arguments.get("sniff"), Some(json::Json::Bool(true)));
    let sniff_limit = arguments
        .get("sniff_limit")
        .and_then(as_usize)
        .unwrap_or(3)
        .clamp(1, 5);
    let since = arguments.get("since").and_then(|s| s.as_str());
    let until = arguments.get("until").and_then(|s| s.as_str());
    if !source.is_empty() && source != "all" {
        if let Ok(rows) = source_rows() {
            if !rows.iter().any(|(name, _)| name == source) {
                return error_result(&format!("unknown source: {source}"));
            }
        }
    }
    let args = match build_args(source, group, query, limit, since, until, jobs) {
        Ok(args) => args,
        Err(message) => return error_result(&message),
    };
    match run_archive(&args) {
        Ok(text) if text.trim().is_empty() => content_result("pending — no result lines", false),
        Ok(text) => {
            let text = if sniff {
                add_sniffs(&text, sniff_limit)
            } else {
                text
            };
            content_result(&text, false)
        }
        Err(message) => content_result(&message, true),
    }
}

fn build_args(
    source: &str,
    group: &str,
    query: &str,
    limit: usize,
    since: Option<&str>,
    until: Option<&str>,
    jobs: usize,
) -> Result<Vec<String>, String> {
    if source.is_empty() && group.is_empty() {
        return Err("choose source (single arm) or source=all with group".to_string());
    }
    if !group.is_empty() && !matches!(group, "astro" | "literature" | "web" | "data") {
        return Err(format!("unknown group: {group}"));
    }
    let mut args: Vec<String> = Vec::new();
    if source.is_empty() || source == "all" {
        args.push("--all".to_string());
        if !group.is_empty() {
            args.push("--group".to_string());
            args.push(group.to_string());
        }
        args.push("--jobs".to_string());
        args.push(jobs.to_string());
        args.push("--stop".to_string());
        args.push(limit.to_string());
    } else {
        args.push(format!("--{source}"));
    }
    args.push(query.to_string());
    args.push("--max".to_string());
    args.push(limit.to_string());
    if let Some(since) = since.filter(|s| !s.trim().is_empty()) {
        args.push("--since".to_string());
        args.push(since.to_string());
    }
    if let Some(until) = until.filter(|s| !s.trim().is_empty()) {
        args.push("--until".to_string());
        args.push(until.to_string());
    }
    Ok(args)
}

fn sources_call(params: &json::Json) -> String {
    let group = params
        .get("arguments")
        .and_then(|a| a.get("group"))
        .and_then(|g| g.as_str())
        .unwrap_or("")
        .trim();
    if !group.is_empty() && !matches!(group, "astro" | "literature" | "web" | "data") {
        return error_result(&format!("unknown group: {group}"));
    }
    let rows = match source_rows() {
        Ok(rows) => rows,
        Err(message) => return content_result(&message, true),
    };
    let mut lines = vec!["all\tastro,literature,web,data".to_string()];
    for (source, groups) in rows {
        if group.is_empty() || groups.split(',').any(|g| g == group) {
            lines.push(format!("{source}\t{groups}"));
        }
    }
    content_result(&lines.join("\n"), false)
}

fn source_rows() -> Result<Vec<(String, String)>, String> {
    static ROWS: OnceLock<Result<Vec<(String, String)>, String>> = OnceLock::new();
    ROWS.get_or_init(|| {
        let text = run_archive(&["--list-sources".to_string()])?;
        let rows = text
            .lines()
            .filter_map(|line| {
                let (source, groups) = line.split_once('\t')?;
                Some((source.to_string(), groups.to_string()))
            })
            .collect::<Vec<_>>();
        if rows.is_empty() {
            return Err("pending — archive_search listed no sources".to_string());
        }
        Ok(rows)
    })
    .clone()
}

fn add_sniffs(text: &str, limit: usize) -> String {
    let urls = result_urls(text);
    if urls.is_empty() {
        return text.to_string();
    }
    let mut out = text.to_string();
    out.push_str("\n--- sniff ---");
    for url in urls.into_iter().take(limit) {
        match run_sniff(&url) {
            Ok(lines) => {
                out.push('\n');
                out.push_str(lines.trim());
            }
            Err(message) => {
                out.push_str("\n[sniff] pending — ");
                out.push_str(&message);
            }
        }
    }
    out
}

fn result_urls(text: &str) -> Vec<String> {
    let mut urls: Vec<String> = Vec::new();
    for line in text.lines() {
        let Some(rest) = line.strip_prefix("url ") else {
            continue;
        };
        let url = rest.split('\t').next().unwrap_or(rest).trim();
        if url.starts_with("http") && !urls.iter().any(|seen| seen == url) {
            urls.push(url.to_string());
        }
    }
    urls
}

fn run_sniff(url: &str) -> Result<String, String> {
    let bin = archive_search_bin();
    let out = Command::new(&bin)
        .arg("--sniff")
        .arg(url)
        .output()
        .map_err(|e| format!("cannot run {}: {e}", bin.display()))?;
    let stdout = String::from_utf8_lossy(&out.stdout).trim().to_string();
    if stdout.is_empty() {
        let stderr = String::from_utf8_lossy(&out.stderr);
        return Err(first_line(stderr.trim()).to_string());
    }
    Ok(stdout)
}

fn run_archive(args: &[String]) -> Result<String, String> {
    let bin = archive_search_bin();
    let out = Command::new(&bin)
        .args(args)
        .output()
        .map_err(|e| format!("pending — cannot run {}: {e}", bin.display()))?;
    let stdout = String::from_utf8_lossy(&out.stdout);
    let text = strip_prefix(stdout.trim());
    if text.is_empty() {
        let stderr = String::from_utf8_lossy(&out.stderr);
        let stderr = stderr.trim();
        if stderr.is_empty() {
            return Ok(String::new());
        }
        return Err(format!("pending — archive_search: {}", first_line(stderr)));
    }
    Ok(text)
}

fn archive_search_bin() -> PathBuf {
    if let Ok(path) = env::var("OMEGAFLOW_ARCHIVE_SEARCH") {
        if !path.trim().is_empty() {
            return PathBuf::from(path);
        }
    }
    if let Ok(exe) = env::current_exe() {
        let sibling = exe.with_file_name("archive_search");
        if sibling.is_file() {
            return sibling;
        }
    }
    PathBuf::from("archive_search")
}

fn strip_prefix(text: &str) -> String {
    text.lines()
        .map(|line| {
            line.strip_prefix('[')
                .and_then(|rest| rest.split_once("] "))
                .map(|(_, body)| body)
                .unwrap_or(line)
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn first_line(text: &str) -> &str {
    text.lines().next().unwrap_or(text)
}

fn content_result(text: &str, is_error: bool) -> String {
    format!(
        r#"{{"content":[{{"type":"text","text":"{}"}}],"isError":{is_error}}}"#,
        escape(text)
    )
}

fn error_result(message: &str) -> String {
    content_result(&format!("error: {message}"), true)
}

fn as_usize(value: &json::Json) -> Option<usize> {
    match value {
        json::Json::Num(n) if n.is_finite() && *n >= 1.0 => Some(*n as usize),
        _ => None,
    }
}

fn id_text(id: Option<&json::Json>) -> String {
    match id {
        Some(json::Json::Str(s)) => format!("\"{}\"", escape(s)),
        Some(json::Json::Num(n)) if n.is_finite() && n.fract() == 0.0 => {
            format!("{}", *n as i64)
        }
        Some(json::Json::Num(n)) if n.is_finite() => format!("{n}"),
        Some(json::Json::Null) | None => "null".to_string(),
        _ => "null".to_string(),
    }
}

fn escape(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for c in text.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if c < ' ' => out.push(' '),
            _ => out.push(c),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lists_two_meta_tools() {
        let listed = tools_list();
        assert!(listed.contains("archive_search"));
        assert!(listed.contains("archive_sources"));
        assert!(!listed.contains("simbad_search"));
    }

    #[test]
    fn routes_a_single_source() {
        assert_eq!(
            build_args("simbad", "", "Sirius", 3, None, None, 4).unwrap(),
            vec!["--simbad", "Sirius", "--max", "3"]
        );
    }

    #[test]
    fn routes_a_group_fan_out() {
        assert_eq!(
            build_args("all", "astro", "Sirius", 5, Some("2020"), None, 2).unwrap(),
            vec![
                "--all", "--group", "astro", "--jobs", "2", "--stop", "5", "Sirius", "--max", "5",
                "--since", "2020"
            ]
        );
    }

    #[test]
    fn refuses_a_call_without_route() {
        assert!(build_args("", "", "x", 5, None, None, 4).is_err());
    }

    #[test]
    fn rejects_unknown_groups() {
        assert!(build_args("all", "nope", "x", 5, None, None, 4).is_err());
    }

    #[test]
    fn collects_unique_result_urls() {
        let text = "url https://example.org/a\ttitle: A\nurl https://example.org/a\ttitle: A\nurl https://example.org/b\ttitle: B";
        assert_eq!(
            result_urls(text),
            vec!["https://example.org/a", "https://example.org/b"]
        );
    }

    #[test]
    fn strips_the_archive_prefix() {
        assert_eq!(
            strip_prefix("[simbad] url https://example.org/a\n[simbad] pending — x"),
            "url https://example.org/a\npending — x"
        );
    }
}
