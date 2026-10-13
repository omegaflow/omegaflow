use std::collections::HashMap;
use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process::Command;

struct Route {
    prefix: &'static str,
    provider: &'static str,
    base_url: &'static str,
    key_env: &'static str,
    auth_id: &'static str,
}

const ROUTES: &[Route] = &[
    Route { prefix: "groq", provider: "Groq", base_url: "https://api.groq.com/openai/v1", key_env: "GROQ_API_KEY", auth_id: "" },
    Route { prefix: "ollama", provider: "Ollama Cloud", base_url: "https://ollama.com/v1", key_env: "OLLAMA_API_KEY", auth_id: "" },
    Route { prefix: "llm7", provider: "LLM7", base_url: "https://api.llm7.io/v1", key_env: "LLM7_API_KEY", auth_id: "" },
    Route { prefix: "deepinfra", provider: "DeepInfra", base_url: "https://api.deepinfra.com/v1/openai", key_env: "DEEPINFRA_API_KEY", auth_id: "deepinfra" },
    Route { prefix: "hf", provider: "HuggingFace", base_url: "https://router.huggingface.co/v1", key_env: "HF_TOKEN", auth_id: "huggingface" },
    Route { prefix: "deepseek", provider: "DeepSeek", base_url: "https://api.deepseek.com/v1", key_env: "DEEPSEEK_API_KEY", auth_id: "deepseek" },
    Route { prefix: "zai", provider: "Z.ai", base_url: "https://api.z.ai/api/paas/v4", key_env: "ZAI_API_KEY", auth_id: "" },
    Route { prefix: "sambanova", provider: "SambaNova", base_url: "https://api.sambanova.ai/v1", key_env: "SAMBANOVA_API_KEY", auth_id: "sambanova" },
    Route { prefix: "mistral", provider: "Mistral", base_url: "https://api.mistral.ai/v1", key_env: "MISTRAL_API_KEY", auth_id: "mistral" },
    Route { prefix: "cf", provider: "Cloudflare Workers AI", base_url: "https://api.cloudflare.com/client/v4/accounts", key_env: "CLOUDFLARE_WORKERS_TOKEN", auth_id: "cloudflare-workers-ai" },
    Route { prefix: "nim", provider: "NVIDIA NIM", base_url: "https://integrate.api.nvidia.com/v1", key_env: "NVIDIA_API_KEY", auth_id: "nvidia" },
    Route { prefix: "openrouter", provider: "OpenRouter", base_url: "https://openrouter.ai/api/v1", key_env: "OPENROUTER_API_KEY", auth_id: "openrouter" },
    Route { prefix: "google", provider: "Google AI Studio", base_url: "https://generativelanguage.googleapis.com/v1beta/openai", key_env: "GEMINI_API_KEY", auth_id: "google" },
    Route { prefix: "pollinations", provider: "Pollinations", base_url: "https://text.pollinations.ai/openai", key_env: "POLLINATIONS_API_KEY", auth_id: "" },
];

fn load_secrets() -> HashMap<String, String> {
    let path = match std::env::var("OMEGAFLOW_SECRETS") {
        Ok(p) => p,
        Err(_) => return HashMap::new(),
    };
    let mut m = HashMap::new();
    let text = match fs::read_to_string(&path) {
        Ok(t) => t,
        Err(_) => return m,
    };
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((k, v)) = line.split_once('=') {
            m.insert(k.trim().to_string(), v.trim().to_string());
        }
    }
    m
}

fn load_auth_text() -> String {
    let path = match std::env::var("OMEGAFLOW_AUTH") {
        Ok(p) => p,
        Err(_) => match std::env::var("HOME") {
            Ok(h) => format!("{h}/.local/share/opencode/auth.json"),
            Err(_) => return String::new(),
        },
    };
    match fs::read_to_string(&path) {
        Ok(t) => t,
        Err(_) => String::new(),
    }
}

fn auth_key(auth: &str, id: &str) -> Option<String> {
    if id.is_empty() {
        return None;
    }
    let needle = format!("\"{id}\"");
    let i = auth.find(&needle)?;
    let rest = &auth[i + needle.len()..];
    let k = rest.find("\"key\"")?;
    let rest = &rest[k + 5..];
    let colon = rest.find(':')?;
    let rest = &rest[colon + 1..];
    let q1 = rest.find('"')?;
    let rest = &rest[q1 + 1..];
    let q2 = rest.find('"')?;
    Some(rest[..q2].to_string())
}

fn key_for(route: &Route, secrets: &HashMap<String, String>, auth: &str) -> Option<String> {
    if let Ok(v) = std::env::var(route.key_env) {
        if !v.is_empty() {
            return Some(v);
        }
    }
    match secrets.get(route.key_env) {
        Some(v) if !v.is_empty() => return Some(v.clone()),
        _ => {}
    }
    auth_key(auth, route.auth_id)
}

fn route_for(model: &str) -> Option<&'static Route> {
    for r in ROUTES {
        let plen = r.prefix.len();
        if model.len() > plen && model.as_bytes().get(plen) == Some(&b'/') && &model[..plen] == r.prefix {
            return Some(r);
        }
    }
    None
}

fn json_string_field(body: &str, key: &str) -> Option<String> {
    let needle = format!("\"{key}\"");
    let i = body.find(&needle)?;
    let rest = &body[i + needle.len()..];
    let colon = rest.find(':')?;
    let rest = &rest[colon + 1..];
    let q1 = rest.find('"')?;
    let rest = &rest[q1 + 1..];
    let q2 = rest.find('"')?;
    Some(rest[..q2].to_string())
}

fn find(hay: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.is_empty() || hay.len() < needle.len() {
        return None;
    }
    for i in 0..=(hay.len() - needle.len()) {
        if &hay[i..i + needle.len()] == needle {
            return Some(i);
        }
    }
    None
}

fn content_length(headers: &str) -> usize {
    for line in headers.lines() {
        let lower = line.to_ascii_lowercase();
        if let Some(rest) = lower.strip_prefix("content-length:") {
            let digits: String = rest.trim().chars().take_while(|c| c.is_ascii_digit()).collect();
            if let Ok(n) = digits.parse() {
                return n;
            }
        }
    }
    0
}

fn read_request(stream: &mut TcpStream) -> Option<(String, String, String)> {
    let mut data: Vec<u8> = Vec::new();
    let mut chunk = [0u8; 8192];
    let mut header_end: Option<usize> = None;
    loop {
        let n = match stream.read(&mut chunk) {
            Ok(0) => break,
            Ok(n) => n,
            Err(_) => break,
        };
        data.extend_from_slice(&chunk[..n]);
        if header_end.is_none() {
            if let Some(p) = find(&data, b"\r\n\r\n") {
                header_end = Some(p + 4);
            }
        }
        if let Some(he) = header_end {
            let headers = String::from_utf8_lossy(&data[..he]).to_string();
            let cl = content_length(&headers);
            if data.len() >= he + cl {
                break;
            }
        }
        if data.len() > 10_000_000 {
            break;
        }
    }
    let he = header_end?;
    let headers = String::from_utf8_lossy(&data[..he]).to_string();
    let first = headers.lines().next()?;
    let mut parts = first.split_whitespace();
    let method = match parts.next() {
        Some(s) => s.to_string(),
        None => return None,
    };
    let path = match parts.next() {
        Some(s) => s.to_string(),
        None => return None,
    };
    let body = String::from_utf8_lossy(&data[he..]).to_string();
    Some((method, path, body))
}

fn status_text(code: u16) -> &'static str {
    match code {
        200 => "OK",
        400 => "Bad Request",
        404 => "Not Found",
        429 => "Too Many Requests",
        500 => "Internal Server Error",
        502 => "Bad Gateway",
        503 => "Service Unavailable",
        504 => "Gateway Timeout",
        _ => "Status",
    }
}

fn reply(stream: &mut TcpStream, code: u16, body: &str) {
    let resp = format!(
        "HTTP/1.1 {code} {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        status_text(code),
        body.len()
    );
    let _ = stream.write_all(resp.as_bytes());
    let _ = stream.flush();
}

fn curl_post(url: &str, key: &str, body: &str) -> (u16, String, String) {
    let out = Command::new("curl")
        .args(["-s", "-S", "--max-time", "180", "-X", "POST", "-w", "\n%{http_code}\n%{header_json}"])
        .arg(url)
        .arg("-H")
        .arg(format!("Authorization: Bearer {key}"))
        .arg("-H")
        .arg("Content-Type: application/json")
        .arg("--data-binary")
        .arg(body)
        .output();
    match out {
        Ok(o) => {
            let text = String::from_utf8_lossy(&o.stdout).to_string();
            split_curl(&text)
        }
        Err(e) => (0, String::new(), format!("{{\"error\":{{\"message\":\"curl: {e}\"}}}}")),
    }
}

fn split_curl(text: &str) -> (u16, String, String) {
    let last = match text.rfind('\n') {
        Some(i) => i,
        None => return (0, String::new(), text.to_string()),
    };
    let headers = text[last + 1..].to_string();
    let rest = &text[..last];
    let prev = match rest.rfind('\n') {
        Some(i) => i,
        None => return (0, headers, rest.to_string()),
    };
    let code_text = rest[prev + 1..].trim();
    let code: u16 = match code_text.parse() {
        Ok(c) => c,
        Err(_) => 0,
    };
    let body = rest[..prev].to_string();
    (code, headers, body)
}

fn retry_after_secs(headers: &str) -> Option<u64> {
    let lower = headers.to_ascii_lowercase();
    let i = lower.find("retry-after")?;
    let rest = &lower[i + 11..];
    let digits: String = rest
        .chars()
        .skip_while(|c| !c.is_ascii_digit())
        .take_while(|c| c.is_ascii_digit())
        .collect();
    match digits.parse() {
        Ok(n) => Some(n),
        Err(_) => None,
    }
}

fn chat_completions(body: &str, secrets: &HashMap<String, String>, auth: &str) -> (u16, String) {
    let model = match json_string_field(body, "model") {
        Some(m) => m,
        None => return (400, "{\"error\":{\"message\":\"missing model\"}}".to_string()),
    };
    let mut last_code = 502u16;
    let mut last_body = format!("{{\"error\":{{\"message\":\"no route for {model}\"}}}}");
    for cand in model.split('|') {
        let cand = cand.trim();
        if cand.is_empty() {
            continue;
        }
        let route = match route_for(cand) {
            Some(r) => r,
            None => continue,
        };
        let provider_model = &cand[route.prefix.len() + 1..];
        let key = match key_for(route, secrets, auth) {
            Some(k) => k,
            None => {
                last_code = 502;
                last_body = format!("{{\"error\":{{\"message\":\"key {} absent\"}}}}", route.key_env);
                continue;
            }
        };
        let url = format!("{}/chat/completions", route.base_url);
        let forward = body.replacen(&format!("\"{model}\""), &format!("\"{provider_model}\""), 1);
        let mut attempt = 0;
        loop {
            let (code, headers, resp) = curl_post(&url, &key, &forward);
            if (200..300).contains(&code) {
                return (200, resp);
            }
            if code == 429 || (500..600).contains(&code) {
                last_code = code;
                last_body = resp;
                if attempt < 2 {
                    let wait = match retry_after_secs(&headers) {
                        Some(s) => s.min(30),
                        None => 1u64 << attempt,
                    };
                    std::thread::sleep(std::time::Duration::from_secs(wait));
                    attempt += 1;
                    continue;
                }
                break;
            }
            return (code, resp);
        }
    }
    (last_code, last_body)
}

fn models_json() -> String {
    let items: Vec<String> = ROUTES
        .iter()
        .map(|r| format!("{{\"id\":\"{}/…\",\"object\":\"model\",\"owned_by\":\"{}\"}}", r.prefix, r.provider))
        .collect();
    format!("{{\"object\":\"list\",\"data\":[{}]}}", items.join(","))
}

fn main() {
    let port: u16 = match std::env::var("OMEGAFLOW_GATEWAY_PORT") {
        Ok(v) => match v.parse() {
            Ok(p) => p,
            Err(_) => 8181,
        },
        Err(_) => 8181,
    };
    let addr = format!("127.0.0.1:{port}");
    let listener = match TcpListener::bind(&addr) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("llm_gateway: bind {addr}: {e}");
            std::process::exit(2);
        }
    };
    let secrets = load_secrets();
    let auth = load_auth_text();
    eprintln!("llm_gateway: listening on http://{addr} ({} routes)", ROUTES.len());
    for r in ROUTES {
        let key = if key_for(r, &secrets, &auth).is_some() { "present" } else { "absent" };
        eprintln!("  /{} -> {} ({}) key {} [{}]", r.prefix, r.provider, r.base_url, r.key_env, key);
    }
    for stream in listener.incoming() {
        match stream {
            Ok(mut s) => handle(&mut s, &secrets, &auth),
            Err(e) => eprintln!("llm_gateway: accept: {e}"),
        }
    }
}

fn handle(stream: &mut TcpStream, secrets: &HashMap<String, String>, auth: &str) {
    let (method, path, body) = match read_request(stream) {
        Some(t) => t,
        None => return,
    };
    let (code, out) = match (method.as_str(), path.as_str()) {
        ("GET", "/v1/models") => (200, models_json()),
        ("GET", "/health") => (200, "{\"status\":\"ok\"}".to_string()),
        ("POST", "/v1/chat/completions") => chat_completions(&body, secrets, auth),
        _ => (404, "{\"error\":{\"message\":\"not found\"}}".to_string()),
    };
    reply(stream, code, &out);
}
