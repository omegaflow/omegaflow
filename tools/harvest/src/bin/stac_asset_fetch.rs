use omegaflow::archivar::fetch_raw;
use omegaflow::archivar::fetch_raw_bytes_headers;
use omegaflow::archivar::fetch_raw_bytes_headers_redirect;
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::archivar::stac::{parse_collection_ids, parse_items, select_asset};

const CDSE_TOKEN_URL: &str =
    "https://identity.dataspace.copernicus.eu/auth/realms/CDSE/protocol/openid-connect/token";

fn arg_value(args: &[String], key: &str) -> Option<String> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn auth_headers(token_env: &str) -> Vec<(String, String)> {
    match std::env::var(token_env) {
        Ok(t) if !t.is_empty() => vec![("Authorization".to_string(), format!("Bearer {t}"))],
        _ => Vec::new(),
    }
}

fn urlencode(s: &str) -> String {
    let mut out = String::new();
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(b as char)
            }
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}

fn json_string_field(body: &str, key: &str) -> Option<String> {
    let needle = format!("\"{key}\"");
    let start = body.find(&needle)? + needle.len();
    let after = &body[start..];
    let colon = after.find(':')?;
    let value = &after[colon + 1..];
    let open = value.find('"')?;
    let rest = &value[open + 1..];
    let close = rest.find('"')?;
    Some(rest[..close].to_string())
}

fn cdse_token(user_env: &str, pass_env: &str) -> Option<String> {
    let user = std::env::var(user_env).ok()?;
    let pass = std::env::var(pass_env).ok()?;
    if user.is_empty() || pass.is_empty() {
        return None;
    }
    let body = format!(
        "grant_type=password&client_id=cdse-public&username={}&password={}",
        urlencode(&user),
        urlencode(&pass)
    );
    let headers = vec![(
        "Content-Type".to_string(),
        "application/x-www-form-urlencoded".to_string(),
    )];
    let response = fetch_raw(CDSE_TOKEN_URL, Some(&body), &headers)?;
    json_string_field(&response, "access_token")
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let token_env = match arg_value(&args, "--token-env") {
        Some(v) => v,
        None => "CDSE_TOKEN".to_string(),
    };
    let mut headers = auth_headers(&token_env);
    if headers.is_empty() {
        if let Some(token) = cdse_token("CDSE_USER", "CDSE_PASS") {
            headers = vec![("Authorization".to_string(), format!("Bearer {token}"))];
        }
    }

    if let Some(url) = arg_value(&args, "--collections") {
        let Some(bytes) = fetch_raw_bytes_headers(&url, &headers) else {
            eprintln!("collections fetch returned void ({url})");
            std::process::exit(1);
        };
        let Some(ids) = parse_collection_ids(&bytes) else {
            eprintln!("no collection id in the response ({url})");
            std::process::exit(1);
        };
        for id in ids {
            println!("{id}");
        }
        return;
    }

    if let Some(url) = arg_value(&args, "--items") {
        let Some(bytes) = fetch_raw_bytes_headers(&url, &headers) else {
            eprintln!("items fetch returned void ({url})");
            std::process::exit(1);
        };
        let Some(items) = parse_items(&bytes) else {
            eprintln!("no STAC item in the response ({url})");
            std::process::exit(1);
        };
        let prefer_spec = match arg_value(&args, "--prefer") {
            Some(v) => v,
            None => "geotiff,netcdf".to_string(),
        };
        let prefer: Vec<&str> = prefer_spec.split(',').collect();
        for it in &items {
            match select_asset(it, &prefer) {
                Some(a) => println!(
                    "{} {} {} {}",
                    it.id,
                    a.key,
                    a.media_type.as_deref().unwrap_or("-"),
                    a.href
                ),
                None => println!("{} - - (no asset)", it.id),
            }
        }
        return;
    }

    if let Some(url) = arg_value(&args, "--asset") {
        let Some(bytes) = fetch_raw_bytes_headers_redirect(&url, &headers) else {
            eprintln!("asset fetch returned void ({url})");
            std::process::exit(1);
        };
        if bytes.is_empty() {
            eprintln!("asset returned empty (0 honored) ({url})");
            std::process::exit(1);
        }
        println!("{} {} {}", bytes.len(), sha256_hex(&bytes), url);
        return;
    }

    eprintln!(
        "stac_asset_fetch: --collections <url> | --items <url> [--prefer geotiff,netcdf] | --asset <url> [--token-env CDSE_TOKEN] (no token -> mint via CDSE_USER/CDSE_PASS)"
    );
    std::process::exit(2);
}
