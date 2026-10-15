use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process::{Command, Stdio};
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use std::sync::OnceLock;

struct Route {
    prefix: &'static str,
    provider: &'static str,
    base_url: &'static str,
    key_env: &'static str,
    auth_id: &'static str,
}

const ROUTES: &[Route] = &[
    Route {
        prefix: "groq",
        provider: "Groq",
        base_url: "https://api.groq.com/openai/v1",
        key_env: "GROQ_API_KEY",
        auth_id: "",
    },
    Route {
        prefix: "ollama",
        provider: "Ollama Cloud",
        base_url: "https://ollama.com/v1",
        key_env: "OLLAMA_API_KEY",
        auth_id: "",
    },
    Route {
        prefix: "llm7",
        provider: "LLM7",
        base_url: "https://api.llm7.io/v1",
        key_env: "LLM7_API_KEY",
        auth_id: "",
    },
    Route {
        prefix: "deepinfra",
        provider: "DeepInfra",
        base_url: "https://api.deepinfra.com/v1/openai",
        key_env: "DEEPINFRA_API_KEY",
        auth_id: "deepinfra",
    },
    Route {
        prefix: "hf",
        provider: "HuggingFace",
        base_url: "https://router.huggingface.co/v1",
        key_env: "HF_TOKEN",
        auth_id: "huggingface",
    },
    Route {
        prefix: "deepseek",
        provider: "DeepSeek",
        base_url: "https://api.deepseek.com/v1",
        key_env: "DEEPSEEK_API_KEY",
        auth_id: "deepseek",
    },
    Route {
        prefix: "zai",
        provider: "Z.ai",
        base_url: "https://api.z.ai/api/paas/v4",
        key_env: "ZAI_API_KEY",
        auth_id: "",
    },
    Route {
        prefix: "sambanova",
        provider: "SambaNova",
        base_url: "https://api.sambanova.ai/v1",
        key_env: "SAMBANOVA_API_KEY",
        auth_id: "sambanova",
    },
    Route {
        prefix: "mistral",
        provider: "Mistral (EU)",
        base_url: "https://api.eu.mistral.ai/v1",
        key_env: "MISTRAL_API_KEY",
        auth_id: "mistral",
    },
    Route {
        prefix: "cf",
        provider: "Cloudflare Workers AI",
        base_url: "https://api.cloudflare.com/client/v4/accounts",
        key_env: "CLOUDFLARE_WORKERS_TOKEN",
        auth_id: "cloudflare-workers-ai",
    },
    Route {
        prefix: "nim",
        provider: "NVIDIA NIM",
        base_url: "https://integrate.api.nvidia.com/v1",
        key_env: "NVIDIA_API_KEY",
        auth_id: "nvidia",
    },
    Route {
        prefix: "openrouter",
        provider: "OpenRouter",
        base_url: "https://openrouter.ai/api/v1",
        key_env: "OPENROUTER_API_KEY",
        auth_id: "openrouter",
    },
    Route {
        prefix: "google",
        provider: "Google AI Studio",
        base_url: "https://generativelanguage.googleapis.com/v1beta/openai",
        key_env: "GEMINI_API_KEY",
        auth_id: "google",
    },
    Route {
        prefix: "pollinations",
        provider: "Pollinations",
        base_url: "https://text.pollinations.ai/openai",
        key_env: "POLLINATIONS_API_KEY",
        auth_id: "",
    },
    Route {
        prefix: "zen",
        provider: "OpenCode Zen",
        base_url: "https://opencode.ai/zen/v1",
        key_env: "OPENCODE_API_KEY",
        auth_id: "opencode",
    },
    Route {
        prefix: "inception",
        provider: "Inception",
        base_url: "https://api.inceptionlabs.ai/v1",
        key_env: "INCEPTION_API_KEY",
        auth_id: "inception",
    },
    Route {
        prefix: "kenari",
        provider: "Kenari",
        base_url: "https://kenari.id/v1",
        key_env: "KENARI_API_KEY",
        auth_id: "kenari",
    },
    Route {
        prefix: "unorouter",
        provider: "UnoRouter",
        base_url: "https://api.unorouter.com/v1",
        key_env: "UNOROUTER_API_KEY",
        auth_id: "unorouter",
    },
    Route {
        prefix: "horde",
        provider: "AI Horde",
        base_url: "https://aihorde.net/api/v2",
        key_env: "AIHORDE_API_KEY",
        auth_id: "",
    },
    Route {
        prefix: "ovh",
        provider: "OVHcloud AI Endpoints",
        base_url: "https://oai.endpoints.kepler.ai.cloud.ovh.net/v1",
        key_env: "",
        auth_id: "",
    },
    Route {
        prefix: "kilo",
        provider: "Kilo",
        base_url: "https://api.kilo.ai/api/gateway",
        key_env: "KILO_API_KEY",
        auth_id: "kilo",
    },
    Route {
        prefix: "orcarouter",
        provider: "OrcaRouter",
        base_url: "https://api.orcarouter.ai/v1",
        key_env: "ORCAROUTER_API_KEY",
        auth_id: "orcarouter",
    },
];

fn load_secrets() -> HashMap<String, String> {
    let path = match std::env::var("OMEGAFLOW_SECRETS") {
        Ok(p) => p,
        Err(_) => match std::env::current_dir() {
            Ok(d) => format!("{}/.secrets.local", d.display()),
            Err(_) => return HashMap::new(),
        },
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
    if route.key_env.is_empty() {
        return Some(String::new());
    }
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
        if model.len() > plen
            && model.as_bytes().get(plen) == Some(&b'/')
            && &model[..plen] == r.prefix
        {
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

fn json_number_field(body: &str, key: &str) -> Option<u64> {
    let needle = format!("\"{key}\"");
    let i = body.find(&needle)?;
    let rest = &body[i + needle.len()..];
    let colon = rest.find(':')?;
    let rest = rest[colon + 1..].trim_start();
    let digits: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
    digits.parse().ok()
}

fn json_bool_field(body: &str, key: &str) -> bool {
    let needle = format!("\"{key}\"");
    let Some(i) = body.find(&needle) else {
        return false;
    };
    let rest = &body[i + needle.len()..];
    let rest = match rest.find(':') {
        Some(c) => &rest[c + 1..],
        None => return false,
    };
    rest.trim_start().starts_with("true")
}

fn json_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            _ => out.push(c),
        }
    }
    out.push('"');
    out
}
static REDACTOR: OnceLock<Option<Redactor>> = OnceLock::new();

struct Redactor {
    forward: Vec<(String, String)>,
    reverse: Vec<(String, String)>,
    names: HashSet<String>,
    stop: HashSet<String>,
}

fn filter_enabled() -> bool {
    match std::env::var("OMEGAFLOW_FILTER") {
        Ok(v) => v == "1" || v.eq_ignore_ascii_case("on"),
        Err(_) => false,
    }
}

fn rss_kib() -> Option<u64> {
    let text = fs::read_to_string("/proc/self/status").ok()?;
    for line in text.lines() {
        if let Some(rest) = line.strip_prefix("VmRSS:") {
            return rest.split_whitespace().next()?.parse().ok();
        }
    }
    None
}

fn read_redact_file(name: &str) -> Option<String> {
    let rel = format!("data/redact/{name}");
    if let Ok(text) = fs::read_to_string(&rel) {
        return Some(text);
    }
    if let Ok(repo) = std::env::var("OMEGAFLOW_REPO") {
        if !repo.is_empty() {
            if let Ok(text) = fs::read_to_string(format!("{repo}/data/redact/{name}")) {
                return Some(text);
            }
        }
    }
    if let Ok(home) = std::env::var("HOME") {
        if let Ok(text) = fs::read_to_string(format!("{home}/projects/omegaflow/data/redact/{name}")) {
            return Some(text);
        }
    }
    None
}

fn load_names() -> (HashSet<String>, HashSet<String>) {
    let mut names: HashSet<String> = HashSet::new();
    for file in ["vornamen.txt", "nachnamen.txt"] {
        match read_redact_file(file) {
            Some(text) => {
                for line in text.lines() {
                    let label = line.trim();
                    if !label.is_empty() && !label.starts_with('#') {
                        names.insert(label.to_lowercase());
                    }
                }
            }
            None => eprintln!("llm_gateway: data/redact/{file} absent - that name list stays inert"),
        }
    }
    let mut stop: HashSet<String> = HashSet::new();
    match read_redact_file("stopwoerter.txt") {
        Some(text) => {
            for line in text.lines() {
                let label = line.trim();
                if !label.is_empty() && !label.starts_with('#') {
                    stop.insert(label.to_lowercase());
                }
            }
        }
        None => eprintln!("llm_gateway: data/redact/stopwoerter.txt absent - that stop list stays inert"),
    }
    (names, stop)
}

fn active_redactor() -> Option<&'static Redactor> {
    REDACTOR
        .get_or_init(|| {
            let started = Instant::now();
            let rss_before = rss_kib();
            let dir = match std::env::var("OMEGAFLOW_PRIVATE") {
                Ok(p) if !p.is_empty() => p,
                _ => match std::env::var("HOME") {
                    Ok(h) => format!("{h}/projects/{}", concat!("omegaflow", "-personal")),
                    Err(_) => String::new(),
                },
            };
            let path = format!("{dir}/redact.map");
            let map_text = fs::read_to_string(&path).ok();
            if map_text.is_none() {
                eprintln!("llm_gateway: redact.map absent at {path} - map layer inert");
            }
            let (names, stop) = load_names();
            if map_text.is_none() && names.is_empty() {
                eprintln!("llm_gateway: redact layer inert - no map and no name lists");
                return None;
            }
            let redactor = Redactor::from_parts(map_text.as_deref(), names, stop);
            let elapsed_ms = started.elapsed().as_secs_f64() * 1000.0;
            let rss_after = rss_kib();
            let rss_note = match (rss_before, rss_after) {
                (Some(a), Some(b)) if b >= a => format!("+{} KiB", b - a),
                (Some(a), Some(b)) => format!("{} KiB", b as i64 - a as i64),
                _ => "unread".to_string(),
            };
            eprintln!(
                "llm_gateway: redact layer loaded in {elapsed_ms:.1} ms - {} names - rss {rss_note}",
                redactor.names.len()
            );
            Some(redactor)
        })
        .as_ref()
}

impl Redactor {
    fn from_parts(map_text: Option<&str>, names: HashSet<String>, stop: HashSet<String>) -> Redactor {
        let mut forward: Vec<(String, String)> = Vec::new();
        if let Some(text) = map_text {
            for line in text.lines() {
                let line = line.trim();
                if line.is_empty() || line.starts_with('#') {
                    continue;
                }
                if let Some((real, ph)) = line.split_once('=') {
                    let real = real.trim();
                    let ph = ph.trim();
                    if !real.is_empty() && !ph.is_empty() {
                        forward.push((real.to_string(), ph.to_string()));
                    }
                }
            }
        }
        forward.sort_by(|a, b| b.0.len().cmp(&a.0.len()));
        let mut reverse: Vec<(String, String)> =
            forward.iter().map(|(r, p)| (p.clone(), r.clone())).collect();
        reverse.sort_by(|a, b| b.0.len().cmp(&a.0.len()));
        Redactor { forward, reverse, names, stop }
    }

    fn redact(&self, s: &str) -> String {
        let mut out = apply_patterns(s, &self.names, &self.stop);
        for (real, ph) in &self.forward {
            if out.contains(real.as_str()) {
                out = out.replace(real.as_str(), ph.as_str());
            }
        }
        out
    }

    fn restore(&self, s: &str) -> String {
        let mut out = s.to_string();
        for (ph, real) in &self.reverse {
            if out.contains(ph.as_str()) {
                out = out.replace(ph.as_str(), real.as_str());
            }
        }
        out
    }
}

fn starts(ch: &[char], i: usize, s: &str) -> bool {
    let mut j = i;
    for c in s.chars() {
        if j >= ch.len() || ch[j] != c {
            return false;
        }
        j += 1;
    }
    true
}

fn run_while<F: Fn(char) -> bool>(ch: &[char], mut i: usize, p: F) -> usize {
    while i < ch.len() && p(ch[i]) {
        i += 1;
    }
    i
}

fn c_key(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_' || c == '-'
}

fn c_upnum(c: char) -> bool {
    c.is_ascii_uppercase() || c.is_ascii_digit()
}

fn c_tok(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '.' || c == '_' || c == '-'
}

fn c_mail(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '.' || c == '_' || c == '%' || c == '+' || c == '-'
}

fn c_dom(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '.' || c == '-'
}

fn c_word(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

fn c_de_lower(c: char) -> bool {
    c.is_ascii_lowercase()
        || c == '-'
        || c == '\u{e4}'
        || c == '\u{f6}'
        || c == '\u{fc}'
        || c == '\u{df}'
}

fn c_de_upper(c: char) -> bool {
    c.is_ascii_uppercase() || c == '\u{c4}' || c == '\u{d6}' || c == '\u{dc}'
}

fn subst<F: FnMut(&[char], usize) -> Option<usize>>(s: &str, repl: &str, mut m: F) -> String {
    let ch: Vec<char> = s.chars().collect();
    let mut out = String::with_capacity(s.len());
    let mut i = 0;
    while i < ch.len() {
        match m(&ch, i) {
            Some(e) if e > i => {
                out.push_str(repl);
                i = e;
            }
            _ => {
                out.push(ch[i]);
                i += 1;
            }
        }
    }
    out
}

fn p_sk(ch: &[char], i: usize) -> Option<usize> {
    if starts(ch, i, "sk-") {
        let e = run_while(ch, i + 3, c_key);
        if e >= i + 19 {
            return Some(e);
        }
    }
    None
}

fn p_github(ch: &[char], i: usize) -> Option<usize> {
    if starts(ch, i, "github_pat_") {
        let e = run_while(ch, i + 11, |c| c.is_ascii_alphanumeric() || c == '_');
        if e >= i + 31 {
            return Some(e);
        }
    }
    None
}

fn p_gh(ch: &[char], i: usize) -> Option<usize> {
    match ch.get(i + 2).copied() {
        Some(c) if starts(ch, i, "gh") && "pousr".contains(c) && ch.get(i + 3) == Some(&'_') => {}
        _ => return None,
    }
    let e = run_while(ch, i + 4, |c| c.is_ascii_alphanumeric());
    if e >= i + 24 {
        Some(e)
    } else {
        None
    }
}

fn p_akia(ch: &[char], i: usize) -> Option<usize> {
    if starts(ch, i, "AKIA") {
        let e = i + 20;
        if e <= ch.len() && ch[i + 4..e].iter().copied().all(c_upnum) {
            return Some(e);
        }
    }
    None
}

fn p_aiza(ch: &[char], i: usize) -> Option<usize> {
    if starts(ch, i, "AIza") {
        let e = run_while(ch, i + 4, c_key);
        if e >= i + 34 {
            return Some(e);
        }
    }
    None
}

fn p_mac(ch: &[char], i: usize) -> Option<usize> {
    let mut j = i;
    for k in 0..6 {
        if k > 0 {
            if ch.get(j) != Some(&':') {
                return None;
            }
            j += 1;
        }
        if j + 2 > ch.len() || !ch[j].is_ascii_hexdigit() || !ch[j + 1].is_ascii_hexdigit() {
            return None;
        }
        j += 2;
    }
    Some(j)
}

fn p_uuid(ch: &[char], i: usize) -> Option<usize> {
    let groups = [8usize, 4, 4, 4, 12];
    let mut j = i;
    for (k, g) in groups.iter().enumerate() {
        if k > 0 {
            if ch.get(j) != Some(&'-') {
                return None;
            }
            j += 1;
        }
        if j + g > ch.len() || !ch[j..j + g].iter().all(|c| c.is_ascii_hexdigit()) {
            return None;
        }
        j += g;
    }
    Some(j)
}

fn p_xox(ch: &[char], i: usize) -> Option<usize> {
    match ch.get(i + 3).copied() {
        Some(c) if starts(ch, i, "xox") && "baprs".contains(c) && ch.get(i + 4) == Some(&'-') => {}
        _ => return None,
    }
    let e = run_while(ch, i + 5, |c| c.is_ascii_alphanumeric() || c == '-');
    if e >= i + 15 {
        Some(e)
    } else {
        None
    }
}

fn p_hf(ch: &[char], i: usize) -> Option<usize> {
    if starts(ch, i, "hf_") {
        let e = run_while(ch, i + 3, |c| c.is_ascii_alphanumeric());
        if e >= i + 23 {
            return Some(e);
        }
    }
    None
}

fn p_bearer(ch: &[char], i: usize) -> Option<usize> {
    if starts(ch, i, "Bearer") {
        let ws = run_while(ch, i + 6, |c| c.is_whitespace());
        if ws > i + 6 {
            let e = run_while(ch, ws, c_tok);
            if e >= ws + 16 {
                return Some(e);
            }
        }
    }
    None
}

fn ci_first(ch: &[char], i: usize, w: &str) -> bool {
    let mut it = w.chars();
    let first = match it.next() {
        Some(c) => c,
        None => return true,
    };
    if i >= ch.len() {
        return false;
    }
    let cur = ch[i];
    if cur != first.to_ascii_lowercase() && cur != first.to_ascii_uppercase() {
        return false;
    }
    let mut j = i + 1;
    for c in it {
        if j >= ch.len() || ch[j] != c {
            return false;
        }
        j += 1;
    }
    true
}

fn kw_at(ch: &[char], i: usize) -> Option<(usize, usize)> {
    let mut ke: Option<usize> = None;
    for w in ["password", "passwd", "secret", "token", "key"] {
        if ci_first(ch, i, w) {
            ke = Some(i + w.chars().count());
            break;
        }
    }
    if ke.is_none()
        && i + 3 <= ch.len()
        && (ch[i] == 'A' || ch[i] == 'a')
        && starts(ch, i + 1, "pi")
    {
        let mut j = i + 3;
        if j < ch.len() && (ch[j] == '-' || ch[j] == '_') {
            j += 1;
        }
        if ci_first(ch, j, "key") {
            ke = Some(j + 3);
        }
    }
    let ke = ke?;
    let mut j = run_while(ch, ke, |c| c.is_whitespace());
    if ch.get(j) != Some(&':') && ch.get(j) != Some(&'=') {
        return None;
    }
    j += 1;
    j = run_while(ch, j, |c| c.is_whitespace());
    let v = run_while(ch, j, |c| !c.is_whitespace());
    if v == j {
        return None;
    }
    Some((ke, v))
}

fn rep_kw(s: &str) -> String {
    let ch: Vec<char> = s.chars().collect();
    let mut out = String::with_capacity(s.len());
    let mut i = 0;
    while i < ch.len() {
        if let Some((ke, end)) = kw_at(&ch, i) {
            let kw: String = ch[i..ke].iter().collect();
            out.push_str(&kw);
            out.push_str("=<SECRET>");
            i = end;
        } else {
            out.push(ch[i]);
            i += 1;
        }
    }
    out
}

fn p_email(ch: &[char], i: usize) -> Option<usize> {
    let j = run_while(ch, i, c_mail);
    if j == i || ch.get(j) != Some(&'@') {
        return None;
    }
    let d0 = j + 1;
    let d1 = run_while(ch, d0, c_dom);
    if d1 == d0 {
        return None;
    }
    let mut p = d1;
    while p > d0 {
        p -= 1;
        if ch[p] == '.' {
            let mut t = p + 1;
            while t < d1 && ch[t].is_ascii_alphabetic() {
                t += 1;
            }
            if t >= p + 3 {
                return Some(t);
            }
        }
    }
    None
}

fn p_iban(ch: &[char], i: usize) -> Option<usize> {
    if i + 4 <= ch.len()
        && ch[i].is_ascii_uppercase()
        && ch[i + 1].is_ascii_uppercase()
        && ch[i + 2].is_ascii_digit()
        && ch[i + 3].is_ascii_digit()
    {
        let e = run_while(ch, i + 4, c_upnum);
        if e >= i + 14 {
            return Some(e);
        }
    }
    None
}

fn street_tail(ch: &[char], start: usize) -> Option<usize> {
    let mut j = start;
    let sp = j;
    while j < ch.len() && j - sp < 2 && ch[j] == ' ' {
        j += 1;
    }
    let d = j;
    while j < ch.len() && j - d < 4 && ch[j].is_ascii_digit() {
        j += 1;
    }
    if j == d {
        return None;
    }
    if j < ch.len() && ch[j].is_ascii_lowercase() {
        j += 1;
    }
    if j < ch.len() && c_word(ch[j]) {
        return None;
    }
    Some(j)
}

fn p_street(ch: &[char], i: usize) -> Option<usize> {
    if !c_de_upper(ch[i]) {
        return None;
    }
    let suffixes = [
        "stra\u{df}e",
        "str.",
        "weg",
        "gasse",
        "platz",
        "allee",
        "ring",
        "damm",
        "ufer",
    ];
    let mut best: Option<usize> = None;
    let mut s = i + 3;
    while s < ch.len() {
        if ch[i + 1..s].iter().copied().all(c_de_lower) {
            for suf in suffixes {
                if starts(ch, s, suf) {
                    if let Some(e) = street_tail(ch, s + suf.chars().count()) {
                        if best.map_or(true, |b| e > b) {
                            best = Some(e);
                        }
                    }
                }
            }
        }
        s += 1;
    }
    best
}

fn p_plz(ch: &[char], i: usize) -> Option<usize> {
    let mut j = i;
    for _ in 0..5 {
        if j < ch.len() && ch[j].is_ascii_digit() {
            j += 1;
        } else {
            return None;
        }
    }
    let sp = j;
    while j < ch.len() && j - sp < 2 && ch[j] == ' ' {
        j += 1;
    }
    if j == sp || !c_de_upper(ch.get(j).copied()?) {
        return None;
    }
    j += 1;
    let b = j;
    while j < ch.len() && c_de_lower(ch[j]) {
        j += 1;
    }
    if j >= b + 2 {
        Some(j)
    } else {
        None
    }
}

fn p_tel(ch: &[char], i: usize) -> Option<usize> {
    if i > 0 && c_word(ch[i - 1]) {
        return None;
    }
    let mut ke: Option<usize> = None;
    for w in ["Telefon", "Mobil", "Handy", "Tel", "Fax"] {
        if starts(ch, i, w) {
            ke = Some(i + w.chars().count());
            break;
        }
    }
    let ke = ke?;
    let mut j = run_while(ch, ke, |c| c.is_whitespace());
    while j < ch.len() && (ch[j] == ':' || ch[j] == '.') {
        j += 1;
    }
    j = run_while(ch, j, |c| c.is_whitespace());
    if j >= ch.len() || !(ch[j] == '+' || ch[j].is_ascii_digit()) {
        return None;
    }
    let first = j;
    let e = run_while(ch, first + 1, |c| {
        c.is_ascii_digit() || c == ' ' || c == '(' || c == ')' || c == '/' || c == '.' || c == '-'
    });
    if e < first + 7 {
        return None;
    }
    let mut p = e;
    while p > first + 1 {
        p -= 1;
        if ch[p].is_ascii_digit() && p >= first + 6 {
            let end = p + 1;
            if end >= ch.len() || !c_word(ch[end]) {
                return Some(end);
            }
        }
    }
    None
}

fn p_card(ch: &[char], i: usize) -> Option<usize> {
    if i > 0 && (ch[i - 1].is_ascii_digit() || ch[i - 1] == '-') {
        return None;
    }
    let mut j = i;
    let mut digits = 0usize;
    while j < ch.len() {
        let c = ch[j];
        if c.is_ascii_digit() {
            digits += 1;
            j += 1;
        } else if (c == ' ' || c == '-') && j + 1 < ch.len() && ch[j + 1].is_ascii_digit() {
            j += 1;
        } else {
            break;
        }
    }
    if (13..=19).contains(&digits) && (j >= ch.len() || !c_word(ch[j])) {
        Some(j)
    } else {
        None
    }
}

fn p_bic(ch: &[char], i: usize) -> Option<usize> {
    if i > 0 && c_word(ch[i - 1]) {
        return None;
    }
    let mut j = i;
    let mut k = 0;
    while k < 6 {
        match ch.get(j) {
            Some(c) if c.is_ascii_uppercase() => {
                j += 1;
                k += 1;
            }
            _ => return None,
        }
    }
    let mut k2 = 0;
    while k2 < 2 {
        match ch.get(j) {
            Some(c) if c.is_ascii_alphanumeric() => {
                j += 1;
                k2 += 1;
            }
            _ => return None,
        }
    }
    let base = j;
    let extended = (0..3).all(|d| ch.get(base + d).map_or(false, |c| c.is_ascii_alphanumeric()));
    if extended {
        j = base + 3;
    }
    if j >= ch.len() || !c_word(ch[j]) {
        Some(j)
    } else {
        None
    }
}

const VAT_PREFIX: &[&str] = &[
    "AT", "BE", "BG", "CH", "CY", "CZ", "DE", "DK", "EE", "EL", "ES", "FI", "FR", "GB", "HR",
    "HU", "IE", "IT", "LT", "LU", "LV", "MT", "NL", "NO", "PL", "PT", "RO", "SE", "SI", "SK",
    "XI",
];

fn p_vat(ch: &[char], i: usize) -> Option<usize> {
    if i > 0 && c_word(ch[i - 1]) {
        return None;
    }
    if i + 2 > ch.len() {
        return None;
    }
    let cc: String = ch[i..i + 2].iter().collect();
    if !VAT_PREFIX.contains(&cc.as_str()) {
        return None;
    }
    let mut j = i + 2;
    while j < ch.len() && (ch[j] == ' ' || ch[j] == '.' || ch[j] == '-') {
        j += 1;
    }
    let start = j;
    while j < ch.len() && ch[j].is_ascii_alphanumeric() {
        j += 1;
    }
    let n = j - start;
    if (8..=12).contains(&n) && (j >= ch.len() || !c_word(ch[j])) {
        Some(j)
    } else {
        None
    }
}

fn p_phone(ch: &[char], i: usize) -> Option<usize> {
    if i > 0 && (c_word(ch[i - 1]) || ch[i - 1] == '+' || ch[i - 1] == '.') {
        return None;
    }
    let mut j = i;
    if ch[j] == '+' {
        j += 1;
    }
    if j >= ch.len() || !ch[j].is_ascii_digit() {
        return None;
    }
    let mut digits = 0usize;
    while j < ch.len() {
        let c = ch[j];
        if c.is_ascii_digit() {
            digits += 1;
            j += 1;
        } else if c == '(' || c == ')' {
            j += 1;
        } else if (c == ' ' || c == '-' || c == '/' || c == '.')
            && j + 1 < ch.len()
            && (ch[j + 1].is_ascii_digit() || ch[j + 1] == '(')
        {
            j += 1;
        } else {
            break;
        }
    }
    if (7..=15).contains(&digits) && (j >= ch.len() || !c_word(ch[j])) {
        Some(j)
    } else {
        None
    }
}

fn postal_nl(ch: &[char], i: usize) -> Option<usize> {
    let mut j = i;
    let mut k = 0;
    while k < 4 {
        match ch.get(j) {
            Some(c) if c.is_ascii_digit() => {
                j += 1;
                k += 1;
            }
            _ => return None,
        }
    }
    if ch.get(j) == Some(&' ') {
        j += 1;
    }
    let mut k2 = 0;
    while k2 < 2 {
        match ch.get(j) {
            Some(c) if c.is_ascii_alphabetic() => {
                j += 1;
                k2 += 1;
            }
            _ => return None,
        }
    }
    if j >= ch.len() || !c_word(ch[j]) {
        Some(j)
    } else {
        None
    }
}

fn postal_ca(ch: &[char], i: usize) -> Option<usize> {
    let mut j = i;
    let mut step = 0;
    while step < 6 {
        let c = ch.get(j)?;
        if step == 3 && (*c == ' ' || *c == '-') {
            j += 1;
            continue;
        }
        let ok = if step % 2 == 0 {
            c.is_ascii_alphabetic()
        } else {
            c.is_ascii_digit()
        };
        if !ok {
            return None;
        }
        j += 1;
        step += 1;
    }
    if j >= ch.len() || !c_word(ch[j]) {
        Some(j)
    } else {
        None
    }
}

fn c_name(c: char) -> bool {
    c.is_alphabetic() || c == '-' || c == '\'' || c == '\u{2019}'
}

fn p_name(ch: &[char], i: usize, names: &HashSet<String>, stop: &HashSet<String>) -> Option<usize> {
    if !ch[i].is_alphabetic() {
        return None;
    }
    if i > 0 && c_name(ch[i - 1]) {
        return None;
    }
    let mut j = i;
    while j < ch.len() && j - i < 40 && c_name(ch[j]) {
        j += 1;
    }
    if j - i < 2 {
        return None;
    }
    let token: String = ch[i..j].iter().collect::<String>().to_lowercase();
    if stop.contains(&token) {
        return None;
    }
    if names.contains(&token) {
        Some(j)
    } else {
        None
    }
}

fn postal_pairs(ch: &[char], i: usize) -> Option<usize> {
    if !ch.get(i).map_or(false, |c| c.is_ascii_digit()) {
        return None;
    }
    let mut j = i;
    let mut a = 0usize;
    while j < ch.len() && ch[j].is_ascii_digit() && a < 6 {
        j += 1;
        a += 1;
    }
    let sep = ch.get(j).copied();
    if sep != Some('-') && sep != Some(' ') {
        return None;
    }
    j += 1;
    let mut b = 0usize;
    while j < ch.len() && ch[j].is_ascii_digit() {
        j += 1;
        b += 1;
    }
    let pair = (a, b);
    if matches!(pair, (2, 3) | (3, 2) | (3, 4) | (4, 3) | (5, 3) | (5, 4))
        && (j >= ch.len() || !c_word(ch[j]))
    {
        Some(j)
    } else {
        None
    }
}

fn p_postal(ch: &[char], i: usize) -> Option<usize> {
    if i > 0 && (ch[i - 1].is_ascii_alphanumeric() || ch[i - 1] == '-') {
        return None;
    }
    if let Some(e) = postal_nl(ch, i) {
        return Some(e);
    }
    if let Some(e) = postal_ca(ch, i) {
        return Some(e);
    }
    if let Some(e) = postal_pairs(ch, i) {
        return Some(e);
    }
    if !ch[i].is_ascii_digit() {
        return None;
    }
    let mut j = i;
    while j < ch.len() && ch[j].is_ascii_digit() {
        j += 1;
    }
    let n = j - i;
    if (n == 4 || n == 5 || n == 6)
        && (j >= ch.len() || (!ch[j].is_ascii_alphanumeric() && ch[j] != '-'))
    {
        Some(j)
    } else {
        None
    }
}

fn apply_patterns(s: &str, names: &HashSet<String>, stop: &HashSet<String>) -> String {
    let s = subst(s, "<KEY>", p_sk);
    let s = subst(&s, "<KEY>", p_github);
    let s = subst(&s, "<KEY>", p_gh);
    let s = subst(&s, "<KEY>", p_akia);
    let s = subst(&s, "<KEY>", p_aiza);
    let s = subst(&s, "<MAC>", p_mac);
    let s = subst(&s, "<UUID>", p_uuid);
    let s = subst(&s, "<KEY>", p_xox);
    let s = subst(&s, "<KEY>", p_hf);
    let s = subst(&s, "Bearer <KEY>", p_bearer);
    let s = rep_kw(&s);
    let s = subst(&s, "<EMAIL>", p_email);
    let s = subst(&s, "<IBAN>", p_iban);
    let s = subst(&s, "<ADDR>", p_street);
    let s = subst(&s, "<POSTAL>", p_plz);
    let s = subst(&s, "<TEL>", p_tel);
    let s = subst(&s, "<CARD>", p_card);
    let s = subst(&s, "<BIC>", p_bic);
    let s = subst(&s, "<VAT>", p_vat);
    let s = subst(&s, "<TEL>", p_phone);
    let s = subst(&s, "<POSTAL>", p_postal);
    subst(&s, "<NAME>", |ch, i| p_name(ch, i, names, stop))
}

fn json_ws(b: u8) -> bool {
    matches!(b, b' ' | b'\t' | b'\n' | b'\r')
}

fn json_string_end(s: &[u8], i: usize) -> Option<usize> {
    if s.get(i) != Some(&b'"') {
        return None;
    }
    let mut j = i + 1;
    while j < s.len() {
        match s[j] {
            b'\\' => j += 2,
            b'"' => return Some(j + 1),
            _ => j += 1,
        }
    }
    None
}

fn scan_json(
    s: &[u8],
    i: &mut usize,
    path: &mut Vec<String>,
    spans: &mut Vec<(Vec<String>, usize, usize)>,
) {
    while *i < s.len() && json_ws(s[*i]) {
        *i += 1;
    }
    match s.get(*i) {
        Some(b'{') => {
            *i += 1;
            loop {
                while *i < s.len() && json_ws(s[*i]) {
                    *i += 1;
                }
                if s.get(*i) == Some(&b'}') {
                    *i += 1;
                    return;
                }
                let ke = match json_string_end(s, *i) {
                    Some(e) => e,
                    None => return,
                };
                let key = String::from_utf8_lossy(&s[*i + 1..ke - 1]).into_owned();
                *i = ke;
                while *i < s.len() && json_ws(s[*i]) {
                    *i += 1;
                }
                if s.get(*i) != Some(&b':') {
                    return;
                }
                *i += 1;
                path.push(key);
                scan_json(s, i, path, spans);
                path.pop();
                while *i < s.len() && json_ws(s[*i]) {
                    *i += 1;
                }
                match s.get(*i) {
                    Some(b',') => {
                        *i += 1;
                    }
                    Some(b'}') => {
                        *i += 1;
                        return;
                    }
                    _ => return,
                }
            }
        }
        Some(b'[') => {
            *i += 1;
            loop {
                while *i < s.len() && json_ws(s[*i]) {
                    *i += 1;
                }
                if s.get(*i) == Some(&b']') {
                    *i += 1;
                    return;
                }
                scan_json(s, i, path, spans);
                while *i < s.len() && json_ws(s[*i]) {
                    *i += 1;
                }
                match s.get(*i) {
                    Some(b',') => {
                        *i += 1;
                    }
                    Some(b']') => {
                        *i += 1;
                        return;
                    }
                    _ => return,
                }
            }
        }
        Some(b'"') => {
            let e = match json_string_end(s, *i) {
                Some(e) => e,
                None => return,
            };
            spans.push((path.clone(), *i, e));
            *i = e;
        }
        Some(_) => {
            while *i < s.len() && !json_ws(s[*i]) && !matches!(s[*i], b',' | b'}' | b']') {
                *i += 1;
            }
        }
        None => {}
    }
}

fn transform_content(body: &str, container: &str, f: &dyn Fn(&str) -> String) -> String {
    let mut i = 0usize;
    let mut path: Vec<String> = Vec::new();
    let mut spans: Vec<(Vec<String>, usize, usize)> = Vec::new();
    scan_json(body.as_bytes(), &mut i, &mut path, &mut spans);
    let mut out = String::with_capacity(body.len());
    let mut last = 0usize;
    for (p, s, e) in spans {
        if p.last().map(String::as_str) != Some("content") {
            continue;
        }
        if !p.iter().any(|k| k.as_str() == container) {
            continue;
        }
        out.push_str(&body[last..s]);
        out.push('"');
        out.push_str(&f(&body[s + 1..e - 1]));
        out.push('"');
        last = e;
    }
    out.push_str(&body[last..]);
    out
}

fn restore_sse_event(event: &str, r: &Redactor) -> String {
    let mut out = String::with_capacity(event.len());
    for line in event.split_inclusive('\n') {
        let (text, nl) = match line.strip_suffix('\n') {
            Some(c) => (c, "\n"),
            None => (line, ""),
        };
        match text.strip_prefix("data:") {
            Some(rest) if rest.trim() != "[DONE]" && rest.trim_start().starts_with('{') => {
                out.push_str("data:");
                out.push_str(&transform_content(rest, "choices", &|s| r.restore(s)));
                out.push_str(nl);
            }
            _ => out.push_str(line),
        }
    }
    out
}
fn provenance_sink() -> String {
    match std::env::var("OMEGAFLOW_GATEWAY_LOG") {
        Ok(p) if !p.is_empty() => p,
        _ => "state/zustand/llm-gateway-provenance.jsonl".to_string(),
    }
}

fn write_provenance(line: &str) {
    let path = provenance_sink();
    if let Some(parent) = std::path::Path::new(&path).parent() {
        let _ = fs::create_dir_all(parent);
    }
    match fs::OpenOptions::new().create(true).append(true).open(&path) {
        Ok(mut f) => {
            let _ = writeln!(f, "{line}");
        }
        Err(e) => eprintln!("llm_gateway: provenance sink {path} ({:?})", e.kind()),
    }
}

fn emit_provenance(
    alias: &str,
    attempted: &[String],
    start: Instant,
    code: u16,
    provider: Option<&str>,
    model_id: Option<&str>,
    resp: &str,
) {
    let ts = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(d) => d.as_secs(),
        Err(_) => 0,
    };
    let mut s = format!("{{\"ts\":{ts},\"alias\":{}", json_escape(alias));
    if let Some(p) = provider {
        s.push_str(&format!(",\"provider\":{}", json_escape(p)));
    }
    if let Some(m) = model_id {
        s.push_str(&format!(",\"model_id\":{}", json_escape(m)));
    }
    if let Some(id) = json_string_field(resp, "id") {
        s.push_str(&format!(",\"request_id\":{}", json_escape(&id)));
    }
    if let Some(p) = json_number_field(resp, "prompt_tokens") {
        s.push_str(&format!(",\"prompt_tokens\":{p}"));
    }
    if let Some(c) = json_number_field(resp, "completion_tokens") {
        s.push_str(&format!(",\"completion_tokens\":{c}"));
    }
    if let Some(t) = json_number_field(resp, "total_tokens") {
        s.push_str(&format!(",\"total_tokens\":{t}"));
    }
    s.push_str(&format!(",\"latency_ms\":{}", start.elapsed().as_millis()));
    s.push_str(&format!(",\"status\":{code}"));
    s.push_str(&format!(",\"ok\":{}", (200..300).contains(&code)));
    let chain: Vec<String> = attempted.iter().map(|a| json_escape(a)).collect();
    s.push_str(&format!(",\"fallback_chain\":[{}]", chain.join(",")));
    s.push('}');
    write_provenance(&s);
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
            let digits: String = rest
                .trim()
                .chars()
                .take_while(|c| c.is_ascii_digit())
                .collect();
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
    curl_post_timeout(url, key, body, 180)
}

fn curl_post_timeout(url: &str, key: &str, body: &str, secs: u64) -> (u16, String, String) {
    let dir = std::env::temp_dir();
    let body_path = dir.join("omegaflow_gw_body.txt");
    let head_path = dir.join("omegaflow_gw_headers.txt");
    let max_time = secs.to_string();
    let out = Command::new("curl")
        .args(["-s", "-S", "--max-time"])
        .arg(&max_time)
        .args(["-X", "POST"])
        .arg(url)
        .arg("-H")
        .arg(format!("Authorization: Bearer {key}"))
        .arg("-H")
        .arg("Content-Type: application/json")
        .arg("--data-binary")
        .arg(body)
        .arg("-o")
        .arg(&body_path)
        .arg("-D")
        .arg(&head_path)
        .arg("-w")
        .arg("%{http_code}")
        .output();
    let code: u16 = match out {
        Ok(o) => {
            let t = String::from_utf8_lossy(&o.stdout).trim().to_string();
            match t.parse() {
                Ok(c) => c,
                Err(_) => 0,
            }
        }
        Err(e) => {
            return (
                0,
                String::new(),
                format!("{{\"error\":{{\"message\":\"curl: {e}\"}}}}"),
            );
        }
    };
    let resp = match fs::read_to_string(&body_path) {
        Ok(s) => s,
        Err(_) => String::new(),
    };
    let headers = match fs::read_to_string(&head_path) {
        Ok(s) => s,
        Err(_) => String::new(),
    };
    let _ = fs::remove_file(&body_path);
    let _ = fs::remove_file(&head_path);
    (code, headers, resp)
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

fn is_paid(prefix: &str) -> bool {
    matches!(
        prefix,
        "deepinfra"
            | "deepseek"
            | "mistral"
            | "sambanova"
            | "hf"
            | "google"
            | "zen"
            | "inception"
            | "unorouter"
            | "orcarouter"
            | "nim"
            | "openrouter"
    )
}

fn paid_allowed(prefix: &str) -> bool {
    if let Ok(v) = std::env::var("OMEGAFLOW_PAID") {
        let v = v.trim();
        if v == "1" || v.eq_ignore_ascii_case("all") {
            return true;
        }
        if v.split(|c: char| c == ',' || c.is_whitespace())
            .any(|p| p == prefix)
        {
            return true;
        }
    }
    let work = match std::env::var("OMEGAFLOW_WORK") {
        Ok(w) => w,
        Err(_) => match std::env::var("HOME") {
            Ok(h) => format!("{h}/projects/omegaflow-work"),
            Err(_) => return false,
        },
    };
    if let Ok(s) = std::fs::read_to_string(format!("{work}/post/paid-open")) {
        let s = s.trim();
        if s == "1" || s.eq_ignore_ascii_case("all") {
            return true;
        }
        return s
            .split(|c: char| c == ',' || c.is_whitespace())
            .any(|p| p == prefix);
    }
    false
}

fn chat_completions(body: &str, secrets: &HashMap<String, String>, auth: &str, red: Option<&Redactor>) -> (u16, String) {
    let start = Instant::now();
    let model = match json_string_field(body, "model") {
        Some(m) => m,
        None => {
            return (
                400,
                "{\"error\":{\"message\":\"missing model\"}}".to_string(),
            );
        }
    };
    let disabled = disabled_set();
    let mut attempted: Vec<String> = Vec::new();
    let mut answered_provider: Option<String> = None;
    let mut answered_model: Option<String> = None;
    let mut last_code = 502u16;
    let mut last_body = format!("{{\"error\":{{\"message\":\"no route for {model}\"}}}}");
    let result: (u16, String) = 'chain: {
        for cand in model.split('|') {
            let cand = cand.trim();
            if cand.is_empty() {
                continue;
            }
            let route = match route_for(cand) {
                Some(r) => r,
                None => continue,
            };
            if is_paid(route.prefix) && !paid_allowed(route.prefix) {
                last_code = 402;
                last_body = format!(
                    "{{\"error\":{{\"message\":\"{} is a billed route — the operator's word is required (name the route in $OMEGAFLOW_WORK/post/paid-open, e.g. `mistral`, or set OMEGAFLOW_PAID=mistral)\"}}}}",
                    route.prefix
                );
                continue;
            }
            let provider_model = &cand[route.prefix.len() + 1..];
            if disabled.contains(&format!("{}/{}", route.prefix, provider_model)) {
                last_code = 503;
                last_body = format!(
                    "{{\"error\":{{\"message\":\"{} {} disabled for this session\"}}}}",
                    route.prefix, provider_model
                );
                continue;
            }
            attempted.push(format!("{}/{}", route.provider, provider_model));
            if route.prefix == "horde" {
                let key = match key_for(route, secrets, auth) {
                    Some(k) => k,
                    None => "0000000000".to_string(),
                };
                answered_provider = Some(route.provider.to_string());
                answered_model = Some(provider_model.to_string());
                let (code, resp) = horde_chat(body, provider_model, &key);
                break 'chain (code, resp);
            }
            let key = match key_for(route, secrets, auth) {
                Some(k) => k,
                None => {
                    last_code = 502;
                    last_body = format!(
                        "{{\"error\":{{\"message\":\"key {} absent\"}}}}",
                        route.key_env
                    );
                    continue;
                }
            };
            let base = match effective_base(route, secrets) {
                Some(b) => b,
                None => {
                    last_code = 502;
                    last_body = format!(
                        "{{\"error\":{{\"message\":\"base {} absent\"}}}}",
                        route.prefix
                    );
                    continue;
                }
            };
            let url = format!("{}/chat/completions", base);
            let forward =
                body.replacen(&format!("\"{model}\""), &format!("\"{provider_model}\""), 1);
            let mut attempt = 0;
            loop {
                let (code, headers, resp) = curl_post(&url, &key, &forward);
                if (200..300).contains(&code) {
                    answered_provider = Some(route.provider.to_string());
                    answered_model = Some(provider_model.to_string());
                    break 'chain (200, resp);
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
                break 'chain (code, resp);
            }
        }
        (last_code, last_body)
    };
    emit_provenance(
        &model,
        &attempted,
        start,
        result.0,
        answered_provider.as_deref(),
        answered_model.as_deref(),
        &result.1,
    );
    let (code, raw) = result;
    match red {
        Some(r) => (code, transform_content(&raw, "choices", &|s| r.restore(s))),
        None => (code, raw),
    }
}

fn stream_chat(stream: &mut TcpStream, body: &str, secrets: &HashMap<String, String>, auth: &str, red: Option<&Redactor>) {
    let start = Instant::now();
    let model = match json_string_field(body, "model") {
        Some(m) => m,
        None => {
            reply(stream, 400, "{\"error\":{\"message\":\"missing model\"}}");
            return;
        }
    };
    let disabled = disabled_set();
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
        if disabled.contains(&format!("{}/{}", route.prefix, provider_model)) {
            continue;
        }
        if route.prefix == "horde" {
            let key = match key_for(route, secrets, auth) {
                Some(k) => k,
                None => "0000000000".to_string(),
            };
            let (code, resp) = horde_chat(body, provider_model, &key);
            let out = match red {
                Some(r) => transform_content(&resp, "choices", &|s| r.restore(s)),
                None => resp,
            };
            reply(stream, code, &out);
            return;
        }
        let key = match key_for(route, secrets, auth) {
            Some(k) => k,
            None => continue,
        };
        let base = match effective_base(route, secrets) {
            Some(b) => b,
            None => continue,
        };
        let url = format!("{}/chat/completions", base);
        let forward = body.replacen(&format!("\"{model}\""), &format!("\"{provider_model}\""), 1);
        let head = "HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nCache-Control: no-cache\r\nConnection: close\r\n\r\n";
        if stream.write_all(head.as_bytes()).is_err() {
            return;
        }
        let _ = stream.flush();
        match Command::new("curl")
            .args(["-s", "-S", "-N", "--max-time", "300", "-X", "POST"])
            .arg(&url)
            .arg("-H")
            .arg(format!("Authorization: Bearer {key}"))
            .arg("-H")
            .arg("Content-Type: application/json")
            .arg("--data-binary")
            .arg(&forward)
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
        {
            Ok(mut c) => {
                if let Some(mut out) = c.stdout.take() {
                    let mut buf = [0u8; 8192];
                    match red {
                        Some(r) => {
                            let mut carry: Vec<u8> = Vec::new();
                            let mut events = String::new();
                            'stream: loop {
                                match out.read(&mut buf) {
                                    Ok(0) => break,
                                    Ok(n) => {
                                        let mut joined = std::mem::take(&mut carry);
                                        joined.extend_from_slice(&buf[..n]);
                                        let valid = match std::str::from_utf8(&joined) {
                                            Ok(s) => s.len(),
                                            Err(e) => e.valid_up_to(),
                                        };
                                        events.push_str(&String::from_utf8_lossy(&joined[..valid]));
                                        carry = joined[valid..].to_vec();
                                        while let Some(pos) = events.find("\n\n") {
                                            let event: String = events.drain(..pos + 2).collect();
                                            let emit = restore_sse_event(&event, r);
                                            if stream.write_all(emit.as_bytes()).is_err() {
                                                break 'stream;
                                            }
                                            let _ = stream.flush();
                                        }
                                    }
                                    Err(_) => break,
                                }
                            }
                            if !carry.is_empty() {
                                events.push_str(&String::from_utf8_lossy(&carry));
                            }
                            if !events.is_empty() {
                                let emit = restore_sse_event(&events, r);
                                let _ = stream.write_all(emit.as_bytes());
                                let _ = stream.flush();
                            }
                        }
                        None => loop {
                            match out.read(&mut buf) {
                                Ok(0) => break,
                                Ok(n) => {
                                    if stream.write_all(&buf[..n]).is_err() {
                                        break;
                                    }
                                    let _ = stream.flush();
                                }
                                Err(_) => break,
                            }
                        },
                    }
                }
                let _ = c.wait();
            }
            Err(e) => {
                let err = format!("data: {{\"error\":{{\"message\":\"curl spawn: {e}\"}}}}\n\n");
                let _ = stream.write_all(err.as_bytes());
                let _ = stream.flush();
            }
        }
        emit_provenance(
            &model,
            &[format!("{}/{}", route.provider, provider_model)],
            start,
            200,
            Some(route.provider),
            Some(provider_model),
            "",
        );
        return;
    }
    reply(
        stream,
        502,
        &format!("{{\"error\":{{\"message\":\"no route for {model}\"}}}}"),
    );
}

fn curl_get(url: &str, key: &str) -> Option<(u16, String)> {
    let out = Command::new("curl")
        .args(["-s", "-S", "--max-time", "30", "-w", "\n%{http_code}"])
        .arg(url)
        .arg("-H")
        .arg(format!("Authorization: Bearer {key}"))
        .output()
        .ok()?;
    let raw = String::from_utf8_lossy(&out.stdout).to_string();
    let (body, code) = match raw.rfind('\n') {
        Some(i) => {
            let code = raw[i + 1..].trim().parse::<u16>().ok()?;
            (raw[..i].to_string(), code)
        }
        None => (raw, 0),
    };
    Some((code, body))
}

const CF_ACCOUNT_ID_DEFAULT: &str = "b2891b7064691de2ef3891cfaf17190b";

fn cf_account(secrets: &HashMap<String, String>) -> String {
    match std::env::var("CLOUDFLARE_ACCOUNT_ID") {
        Ok(v) if !v.is_empty() => v,
        _ => match secrets.get("CLOUDFLARE_ACCOUNT_ID") {
            Some(v) if !v.is_empty() => v.clone(),
            _ => CF_ACCOUNT_ID_DEFAULT.to_string(),
        },
    }
}

fn effective_base(r: &Route, secrets: &HashMap<String, String>) -> Option<String> {
    if r.prefix != "cf" {
        return Some(r.base_url.to_string());
    }
    Some(format!("{}/{}/ai/v1", r.base_url, cf_account(secrets)))
}

fn curl_capture(args: &[String]) -> (u16, String) {
    let out = Command::new("curl").args(args).output();
    match out {
        Ok(o) => {
            let raw = String::from_utf8_lossy(&o.stdout).to_string();
            match raw.rfind('\n') {
                Some(i) => {
                    let code = match raw[i + 1..].trim().parse::<u16>() {
                        Ok(c) => c,
                        Err(_) => 0,
                    };
                    (code, raw[..i].to_string())
                }
                None => (0, raw),
            }
        }
        Err(_) => (0, String::new()),
    }
}

fn horde_chat(body: &str, provider_model: &str, key: &str) -> (u16, String) {
    let base = "https://aihorde.net/api/v2";
    let prompt = match json_string_field(body, "content") {
        Some(p) => p,
        None => {
            return (
                400,
                "{\"error\":{\"message\":\"horde: no prompt\"}}".to_string(),
            );
        }
    };
    let submit_body = format!(
        "{{\"prompt\":{},\"params\":{{\"max_length\":512,\"max_context_length\":4096}},\"models\":[{}]}}",
        json_escape(&prompt),
        json_escape(provider_model)
    );
    let submit_args = vec![
        "-s".to_string(),
        "-S".to_string(),
        "--max-time".to_string(),
        "60".to_string(),
        "-X".to_string(),
        "POST".to_string(),
        format!("{base}/generate/text/async"),
        "-H".to_string(),
        format!("apikey: {key}"),
        "-H".to_string(),
        "Client-Agent: omegaflow:1.0:https://omegaflow.space".to_string(),
        "-H".to_string(),
        "Content-Type: application/json".to_string(),
        "--data-binary".to_string(),
        submit_body,
        "-w".to_string(),
        "\n%{http_code}".to_string(),
    ];
    let (code, resp) = curl_capture(&submit_args);
    if !(200..300).contains(&code) {
        return (
            502,
            format!("{{\"error\":{{\"message\":\"horde submit http {code}\"}}}}"),
        );
    }
    let id = match json_string_field(&resp, "id") {
        Some(i) => i,
        None => {
            return (
                502,
                "{\"error\":{\"message\":\"horde: no job id\"}}".to_string(),
            );
        }
    };
    for _ in 0..40 {
        std::thread::sleep(std::time::Duration::from_secs(3));
        let status_args = vec![
            "-s".to_string(),
            "-S".to_string(),
            "--max-time".to_string(),
            "30".to_string(),
            format!("{base}/generate/text/status/{id}"),
            "-H".to_string(),
            format!("apikey: {key}"),
            "-H".to_string(),
            "Client-Agent: omegaflow:1.0:https://omegaflow.space".to_string(),
            "-w".to_string(),
            "\n%{http_code}".to_string(),
        ];
        let (scode, sresp) = curl_capture(&status_args);
        if !(200..300).contains(&scode) {
            continue;
        }
        if sresp.contains("\"done\":true") || sresp.contains("\"done\": true") {
            let text = match json_string_field(&sresp, "text") {
                Some(t) => t,
                None => String::new(),
            };
            let out = format!(
                "{{\"id\":\"horde-{id}\",\"object\":\"chat.completion\",\"choices\":[{{\"index\":0,\"message\":{{\"role\":\"assistant\",\"content\":{}}},\"finish_reason\":\"stop\"}}]}}",
                json_escape(&text)
            );
            return (200, out);
        }
    }
    (
        504,
        "{\"error\":{\"message\":\"horde: job not finished in time\"}}".to_string(),
    )
}

fn json_string_values(body: &str, key: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut rest = body;
    let needle = format!("\"{key}\"");
    loop {
        let Some(i) = rest.find(&needle) else { break };
        let after = &rest[i + needle.len()..];
        let Some(colon) = after.find(':') else { break };
        let after2 = &after[colon + 1..];
        let Some(q1) = after2.find('"') else { break };
        let after3 = &after2[q1 + 1..];
        let Some(q2) = after3.find('"') else { break };
        out.push(after3[..q2].to_string());
        rest = &after3[q2 + 1..];
    }
    out
}

fn models_url(r: &Route, secrets: &HashMap<String, String>) -> Option<(String, &'static str)> {
    if r.prefix == "cf" {
        return Some((
            format!(
                "{}/{}/ai/models/search?per_page=200",
                r.base_url,
                cf_account(secrets)
            ),
            "name",
        ));
    }
    let base = effective_base(r, secrets)?;
    Some((format!("{}/models", base), "id"))
}

fn catalog(secrets: &HashMap<String, String>, auth: &str) -> Vec<(&'static Route, String)> {
    let mut out: Vec<(&'static Route, String)> = Vec::new();
    for r in ROUTES {
        let key = match key_for(r, secrets, auth) {
            Some(k) => k,
            None => continue,
        };
        let (url, id_key) = match models_url(r, secrets) {
            Some(t) => t,
            None => continue,
        };
        let (code, body) = match curl_get(&url, &key) {
            Some(t) => t,
            None => {
                eprintln!("llm_gateway: models {} — transport unread", r.prefix);
                continue;
            }
        };
        let ids: Vec<String> = json_string_values(&body, id_key)
            .into_iter()
            .filter(|s| r.prefix != "cf" || s.starts_with("@cf/"))
            .collect();
        eprintln!(
            "llm_gateway: models {} — http {code} ({} ids)",
            r.prefix,
            ids.len()
        );
        for id in ids {
            out.push((r, id));
        }
    }
    out
}

fn arg_value(args: &[String], flag: &str) -> Option<String> {
    let i = args.iter().position(|a| a == flag)?;
    args.get(i + 1).cloned()
}

const PROBE_TIMEOUT_SECS: u64 = 12;
const SLOW_MS: u128 = 8000;
const DISABLED_SESSION: &str = "state/zustand/llm-gateway-disabled.session";
const DISABLED_PERSIST: &str = "state/zustand/llm-gateway-disabled.persist";

fn append_disabled(path: &str, entry: &str) {
    if let Some(parent) = std::path::Path::new(path).parent() {
        let _ = fs::create_dir_all(parent);
    }
    if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(path) {
        let _ = writeln!(f, "{entry}");
    }
}

fn disabled_set() -> HashSet<String> {
    let mut set = HashSet::new();
    for p in [DISABLED_SESSION, DISABLED_PERSIST] {
        if let Ok(t) = fs::read_to_string(p) {
            for line in t.lines() {
                let l = line.trim();
                if !l.is_empty() && !l.starts_with('#') {
                    set.insert(l.to_string());
                }
            }
        }
    }
    set
}

fn probe(
    secrets: &HashMap<String, String>,
    auth: &str,
    provider_filter: &Option<String>,
    model_filter: &Option<String>,
) {
    println!("provider\tmodel\thttp\tms\tverdict");
    let mut reachable = 0u32;
    let mut probed = 0u32;
    let mut session: Vec<String> = Vec::new();
    let mut persist: Vec<String> = Vec::new();
    for (r, id) in catalog(secrets, auth) {
        if let Some(pf) = provider_filter {
            if !pf.split(',').any(|p| p.trim() == r.prefix) {
                continue;
            }
        }
        if let Some(mf) = model_filter {
            if !id.contains(mf.as_str()) {
                continue;
            }
        }
        let key = match key_for(r, secrets, auth) {
            Some(k) => k,
            None => continue,
        };
        let base = match effective_base(r, secrets) {
            Some(b) => b,
            None => continue,
        };
        let entry = format!("{}/{}", r.prefix, id);
        let url = format!("{}/chat/completions", base);
        let body = format!(
            "{{\"model\":\"{id}\",\"messages\":[{{\"role\":\"user\",\"content\":\"ping\"}}],\"max_tokens\":4}}"
        );
        let t0 = Instant::now();
        let (code, _headers, resp) = curl_post_timeout(&url, &key, &body, PROBE_TIMEOUT_SECS);
        let ms = t0.elapsed().as_millis();
        let verdict = if code == 0 {
            session.push(entry.clone());
            "timeout"
        } else if code == 402 || resp.to_lowercase().contains("insufficient") {
            persist.push(entry.clone());
            "quota"
        } else if code == 429 {
            session.push(entry.clone());
            "ratelimit"
        } else if (200..300).contains(&code) && ms >= SLOW_MS {
            session.push(entry.clone());
            "slow"
        } else if [400u16, 403, 404, 405, 410].contains(&code) {
            session.push(entry.clone());
            "unsupported"
        } else if (200..300).contains(&code) {
            reachable += 1;
            "ok"
        } else {
            "error"
        };
        probed += 1;
        println!("{}\t{}\t{}\t{}\t{}", r.prefix, id, code, ms, verdict);
    }
    for e in &session {
        append_disabled(DISABLED_SESSION, e);
    }
    for e in &persist {
        append_disabled(DISABLED_PERSIST, e);
    }
    eprintln!(
        "llm_gateway --probe: {reachable}/{probed} reachable · {} session-disabled · {} quota-disabled (persist)",
        session.len(),
        persist.len()
    );
}

fn list_models(secrets: &HashMap<String, String>, auth: &str) -> String {
    let items: Vec<String> = catalog(secrets, auth)
        .into_iter()
        .map(|(r, id)| {
            format!(
                "{{\"id\":\"{}/{}\",\"object\":\"model\",\"owned_by\":\"{}\"}}",
                r.prefix, id, r.provider
            )
        })
        .collect();
    format!("{{\"object\":\"list\",\"data\":[{}]}}", items.join(","))
}

fn raw_catalog(secrets: &HashMap<String, String>, auth: &str, prefix: &str) -> Option<String> {
    for r in ROUTES {
        if r.prefix != prefix {
            continue;
        }
        let key = key_for(r, secrets, auth)?;
        let (url, _) = models_url(r, secrets)?;
        let (_code, body) = curl_get(&url, &key)?;
        return Some(body);
    }
    None
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--probe") {
        let secrets = load_secrets();
        let auth = load_auth_text();
        let provider = arg_value(&args, "--provider");
        let model = arg_value(&args, "--model");
        probe(&secrets, &auth, &provider, &model);
        return;
    }
    if args.iter().any(|a| a == "--catalog") {
        let secrets = load_secrets();
        let auth = load_auth_text();
        let dir = match arg_value(&args, "--out") {
            Some(d) => d,
            None => "state/zustand/llm-gateway-catalog".to_string(),
        };
        let _ = fs::create_dir_all(&dir);
        let provider = arg_value(&args, "--provider");
        for r in ROUTES {
            if let Some(pf) = &provider {
                if !pf.split(',').any(|p| p.trim() == r.prefix) {
                    continue;
                }
            }
            match raw_catalog(&secrets, &auth, r.prefix) {
                Some(body) => {
                    let _ = fs::write(format!("{dir}/{}.json", r.prefix), &body);
                    eprintln!(
                        "catalog {}: {} B -> {dir}/{}.json",
                        r.prefix,
                        body.len(),
                        r.prefix
                    );
                }
                None => eprintln!("catalog {}: unread", r.prefix),
            }
        }
        return;
    }
    let port: u16 = match std::env::var("OMEGAFLOW_GATEWAY_PORT") {
        Ok(v) => match v.parse() {
            Ok(p) => p,
            Err(_) => 1621,
        },
        Err(_) => 1621,
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
    if filter_enabled() {
        active_redactor();
    }
    eprintln!(
        "llm_gateway: listening on http://{addr} ({} routes)",
        ROUTES.len()
    );
    for r in ROUTES {
        let key = if key_for(r, &secrets, &auth).is_some() {
            "present"
        } else {
            "absent"
        };
        eprintln!(
            "  /{} -> {} ({}) key {} [{}]",
            r.prefix, r.provider, r.base_url, r.key_env, key
        );
    }
    let secrets = std::sync::Arc::new(secrets);
    let auth = std::sync::Arc::new(auth);
    for stream in listener.incoming() {
        match stream {
            Ok(mut s) => {
                let secrets = secrets.clone();
                let auth = auth.clone();
                std::thread::spawn(move || handle(&mut s, &secrets, auth.as_str()));
            }
            Err(e) => eprintln!("llm_gateway: accept: {e}"),
        }
    }
}

fn handle(stream: &mut TcpStream, secrets: &HashMap<String, String>, auth: &str) {
    let (method, path, body) = match read_request(stream) {
        Some(t) => t,
        None => return,
    };
    let red = if filter_enabled() { active_redactor() } else { None };
    let outgoing = match red {
        Some(r) => transform_content(&body, "messages", &|s| r.redact(s)),
        None => body.clone(),
    };
    if method == "POST" && path == "/v1/chat/completions" && json_bool_field(&body, "stream") {
        stream_chat(stream, &outgoing, secrets, auth, red);
        return;
    }
    if method == "GET" && path.starts_with("/private/read") {
        let rel = path
            .split_once('?')
            .map(|(_, q)| q)
            .unwrap_or("")
            .split('&')
            .find_map(|kv| kv.strip_prefix("path="))
            .unwrap_or("");
        if rel.is_empty() {
            reply(stream, 400, "path= missing");
            return;
        }
        let repo = match std::env::var("OMEGAFLOW_REPO") {
            Ok(r) => r,
            Err(_) => match std::env::var("HOME") {
                Ok(h) => format!("{h}/projects/omegaflow"),
                Err(_) => {
                    reply(stream, 500, "HOME and OMEGAFLOW_REPO absent - cannot locate the redactor");
                    return;
                }
            },
        };
        let out = std::process::Command::new(format!("{repo}/bin/private_read"))
            .arg(rel)
            .output();
        match out {
            Ok(o) if o.status.success() => {
                let text = String::from_utf8_lossy(&o.stdout).into_owned();
                reply(stream, 200, &text);
            }
            Ok(o) => {
                let text = String::from_utf8_lossy(&o.stderr).trim().to_string();
                reply(stream, 422, &text);
            }
            Err(e) => {
                let text = e.to_string();
                reply(stream, 500, &text);
            }
        }
        return;
    }
    let (code, out) = match (method.as_str(), path.as_str()) {
        ("GET", "/v1/models") => (200, list_models(secrets, auth)),
        ("GET", "/health") => (200, "{\"status\":\"ok\"}".to_string()),
        ("POST", "/v1/chat/completions") => chat_completions(&outgoing, secrets, auth, red),
        _ => (404, "{\"error\":{\"message\":\"not found\"}}".to_string()),
    };
    reply(stream, code, &out);
}
