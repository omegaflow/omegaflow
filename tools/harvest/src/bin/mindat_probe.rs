use omegaflow::archivar::{fetch_raw_bytes_headers, http_code, load_env};
use omegaflow::json::{JsonVal, parse_json};

fn sorted_keys(map: &std::collections::HashMap<String, JsonVal>) -> Vec<String> {
    let mut keys: Vec<String> = map.keys().cloned().collect();
    keys.sort();
    keys
}

fn print_counts(prefix: &str, value: &JsonVal) {
    if let JsonVal::Obj(map) = value {
        for key in sorted_keys(map) {
            match &map[&key] {
                JsonVal::Num(n) if key.contains("count") || key.contains("total") => {
                    println!("count: {}{} = {}", prefix, key, n);
                }
                JsonVal::Obj(_) if key == "meta" || key == "pagination" => {
                    print_counts(&format!("{}{}.", prefix, key), &map[&key]);
                }
                _ => {}
            }
        }
    }
}

fn first_record_fields(value: &JsonVal) -> Option<Vec<String>> {
    if let JsonVal::Obj(map) = value {
        for key in sorted_keys(map) {
            if let JsonVal::Arr(items) = &map[&key] {
                if let Some(JsonVal::Obj(record)) = items.first() {
                    println!("array: {} ({} records)", key, items.len());
                    return Some(sorted_keys(record));
                }
                println!("array: {} ({} records, non-object first)", key, items.len());
                return Some(Vec::new());
            }
        }
    }
    None
}

fn shape(body: &str) {
    match parse_json(body) {
        None => println!("shape: body is not JSON ({} bytes)", body.len()),
        Some(value) => {
            if let JsonVal::Obj(map) = &value {
                let keys = sorted_keys(map);
                println!("top-level keys: {}", keys.join(", "));
                if let Some(fields) = first_record_fields(&value) {
                    println!("first record fields: {}", fields.join(", "));
                }
                print_counts("", &value);
            } else {
                println!("shape: top-level not an object");
            }
        }
    }
}

fn probe(label: &str, url: &str, headers: &[(String, String)]) {
    println!("== {} ==", label);
    match http_code(url, headers) {
        None => println!("pending — no HTTP status (route unread)"),
        Some(401) => println!("pending — HTTP 401"),
        Some(code) => {
            println!("HTTP {}", code);
            if (200..300).contains(&code) {
                match fetch_raw_bytes_headers(url, headers) {
                    None => println!("body unread"),
                    Some(bytes) => shape(&String::from_utf8_lossy(&bytes)),
                }
            }
        }
    }
}

fn body_diag(label: &str, url: &str, headers: &[(String, String)], secret: Option<&str>) {
    let mut cmd = std::process::Command::new("curl");
    cmd.arg("-s")
        .arg("-m")
        .arg("15")
        .arg("--connect-timeout")
        .arg("8");
    for (k, v) in headers {
        cmd.arg("-H").arg(format!("{}: {}", k, v));
    }
    cmd.arg("-w").arg("\n%{http_code}").arg(url);
    let output = match cmd.output() {
        Ok(out) => out,
        Err(_) => {
            println!("{}: curl did not run", label);
            return;
        }
    };
    let text = String::from_utf8_lossy(&output.stdout);
    let (body, code) = match text.rsplit_once('\n') {
        Some((b, c)) => (b.to_string(), c.trim().to_string()),
        None => (text.to_string(), String::new()),
    };
    let redacted = match secret {
        Some(s) if !s.is_empty() => body.replace(s, "[REDACTED]"),
        _ => body,
    };
    let shown: String = redacted.chars().take(200).collect();
    println!(
        "{}: HTTP {} body {} bytes [{}] — {}",
        label,
        code,
        redacted.len(),
        flags_of(&redacted.to_lowercase()),
        shown
    );
}

fn flags_of(lower: &str) -> String {
    let mut flags: Vec<&str> = Vec::new();
    for (needle, name) in [
        ("credential", "credentials"),
        ("invalid", "invalid"),
        ("token", "token-word"),
        ("expired", "expired"),
        ("active", "active"),
        ("member", "member"),
        ("subscription", "subscription"),
        ("plan", "plan"),
        ("permission", "permission"),
        ("forbidden", "forbidden"),
        ("denied", "denied"),
        ("not found", "not-found"),
    ] {
        if lower.contains(needle) {
            flags.push(name);
        }
    }
    flags.join(",")
}

fn main() {
    let env = load_env();
    let token = match env.get("MINDAT_TOKEN") {
        Some(value) if !value.trim().is_empty() => value.trim().to_string(),
        _ => {
            println!("pending — MINDAT_TOKEN absent");
            return;
        }
    };
    let listing = "https://api.mindat.org/v1/localities/?page=1";
    let ua = vec![(
        "User-Agent".to_string(),
        "omegaflow-mindat-probe/1.0".to_string(),
    )];
    let mut token_header = ua.clone();
    token_header.push(("Authorization".to_string(), format!("Token {}", token)));
    let mut bearer_header = ua.clone();
    bearer_header.push(("Authorization".to_string(), format!("Bearer {}", token)));
    let mut bogus = ua.clone();
    bogus.push((
        "Authorization".to_string(),
        "Token not-a-real-token".to_string(),
    ));
    probe("No auth", listing, &ua);
    probe("Bogus Token header", listing, &bogus);
    probe("Scheme Token header", listing, &token_header);
    probe("Scheme Bearer header", listing, &bearer_header);
    body_diag("No auth", listing, &ua, None);
    body_diag("Bogus Token header", listing, &bogus, None);
    body_diag("Scheme Token header", listing, &token_header, Some(&token));
    body_diag(
        "Scheme Bearer header",
        listing,
        &bearer_header,
        Some(&token),
    );
}
