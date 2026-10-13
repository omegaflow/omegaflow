use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

struct Route {
    prefix: &'static str,
    provider: &'static str,
    base_url: &'static str,
    key_env: &'static str,
}

const ROUTES: &[Route] = &[
    Route { prefix: "groq", provider: "Groq", base_url: "https://api.groq.com/openai/v1", key_env: "GROQ_API_KEY" },
    Route { prefix: "ollama", provider: "Ollama Cloud", base_url: "https://ollama.com/v1", key_env: "OLLAMA_API_KEY" },
    Route { prefix: "llm7", provider: "LLM7", base_url: "https://api.llm7.io/v1", key_env: "LLM7_API_KEY" },
    Route { prefix: "deepinfra", provider: "DeepInfra", base_url: "https://api.deepinfra.com/v1/openai", key_env: "DEEPINFRA_API_KEY" },
    Route { prefix: "hf", provider: "HuggingFace", base_url: "https://router.huggingface.co/v1", key_env: "HF_TOKEN" },
    Route { prefix: "zai", provider: "Z.ai", base_url: "https://api.z.ai/api/paas/v4", key_env: "ZAI_API_KEY" },
    Route { prefix: "sambanova", provider: "SambaNova", base_url: "https://api.sambanova.ai/v1", key_env: "SAMBANOVA_API_KEY" },
    Route { prefix: "mistral", provider: "Mistral", base_url: "https://api.mistral.ai/v1", key_env: "MISTRAL_API_KEY" },
    Route { prefix: "cf", provider: "Cloudflare Workers AI", base_url: "https://api.cloudflare.com/client/v4/accounts", key_env: "CF_API_TOKEN" },
    Route { prefix: "nim", provider: "NVIDIA NIM", base_url: "https://integrate.api.nvidia.com/v1", key_env: "NVIDIA_API_KEY" },
    Route { prefix: "openrouter", provider: "OpenRouter", base_url: "https://openrouter.ai/api/v1", key_env: "OPENROUTER_API_KEY" },
    Route { prefix: "google", provider: "Google AI Studio", base_url: "https://generativelanguage.googleapis.com/v1beta/openai", key_env: "GEMINI_API_KEY" },
    Route { prefix: "pollinations", provider: "Pollinations", base_url: "https://text.pollinations.ai/openai", key_env: "POLLINATIONS_API_KEY" },
];

fn main() {
    let port: u16 = std::env::var("OMEGAFLOW_GATEWAY_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8181);
    let addr = format!("127.0.0.1:{port}");
    let listener = match TcpListener::bind(&addr) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("llm_gateway: bind {addr}: {e}");
            std::process::exit(2);
        }
    };
    eprintln!("llm_gateway: listening on http://{addr} ({} routes)", ROUTES.len());
    for r in ROUTES {
        let key = if std::env::var(r.key_env).is_ok() { "present" } else { "absent" };
        eprintln!("  /{} -> {} ({}) key {} [{}]", r.prefix, r.provider, r.base_url, r.key_env, key);
    }
    for stream in listener.incoming() {
        match stream {
            Ok(mut s) => handle(&mut s),
            Err(e) => eprintln!("llm_gateway: accept: {e}"),
        }
    }
}

fn handle(stream: &mut TcpStream) {
    let mut buf = [0u8; 4096];
    let n = match stream.read(&mut buf) {
        Ok(n) => n,
        Err(_) => return,
    };
    let req = String::from_utf8_lossy(&buf[..n]).to_string();
    let first = req.lines().next().unwrap_or("");
    let mut parts = first.split_whitespace();
    let method = parts.next().unwrap_or("");
    let path = parts.next().unwrap_or("");
    let (status, body) = match (method, path) {
        ("GET", "/v1/models") => ("200 OK", models_json()),
        ("GET", "/health") => ("200 OK", "{\"status\":\"ok\"}".to_string()),
        _ => ("404 Not Found", "{\"error\":{\"message\":\"not found\"}}".to_string()),
    };
    let resp = format!(
        "HTTP/1.1 {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    );
    let _ = stream.write_all(resp.as_bytes());
    let _ = stream.flush();
}

fn models_json() -> String {
    let items: Vec<String> = ROUTES
        .iter()
        .map(|r| {
            format!(
                "{{\"id\":\"{}/…\",\"object\":\"model\",\"owned_by\":\"{}\"}}",
                r.prefix, r.provider
            )
        })
        .collect();
    format!("{{\"object\":\"list\",\"data\":[{}]}}", items.join(","))
}
