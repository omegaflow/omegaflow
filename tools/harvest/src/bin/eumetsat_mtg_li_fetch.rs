use omegaflow::archivar::{fetch_raw_bytes, jstr, parse_json};
use std::process::Command;

const SEARCH_URL: &str = "https://api.eumetsat.int/data/search-products/1.0.0/os";
const TOKEN_URL: &str = "https://api.eumetsat.int/token";
const DEFAULT_COLLECTION: &str = "EO:EUM:DAT:0691";
const DATA_ROOT: &str = "data/api.eumetsat.int";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn has_flag(args: &[String], name: &str) -> bool {
    args.iter().any(|a| a == name)
}

fn parse_secret(text: &str, key: &str) -> Option<String> {
    let mut found = None;
    for line in text.lines() {
        if let Some((k, v)) = line.split_once('=') {
            if k.trim() == key && !v.trim().is_empty() {
                found = Some(v.trim().to_string());
            }
        }
    }
    found
}

fn secret(name: &str) -> Option<String> {
    if let Ok(v) = std::env::var(name) {
        if !v.is_empty() {
            return Some(v);
        }
    }
    let body = std::fs::read_to_string(".secrets.local").ok()?;
    parse_secret(&body, name)
}

fn blocks<'a>(body: &'a str, tag: &str) -> Vec<&'a str> {
    let open = format!("<{tag}>");
    let close = format!("</{tag}>");
    let mut out = Vec::new();
    let mut from = 0usize;
    while let Some(s) = body[from..].find(&open) {
        let start = from + s + open.len();
        let Some(e) = body[start..].find(&close) else {
            break;
        };
        out.push(&body[start..start + e]);
        from = start + e + close.len();
    }
    out
}

fn tag_value(block: &str, tag: &str) -> Option<String> {
    let open = format!("<{tag}>");
    let close = format!("</{tag}>");
    let s = block.find(&open)? + open.len();
    let e = block[s..].find(&close)?;
    Some(block[s..s + e].to_string())
}

fn attr(tag: &str, name: &str) -> Option<String> {
    let needle = format!("{name}=\"");
    let s = tag.find(&needle)? + needle.len();
    let e = tag[s..].find('"')?;
    Some(tag[s..s + e].to_string())
}

fn percent_decode(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            let hex = std::str::from_utf8(&bytes[i + 1..i + 3]).unwrap_or("");
            if let Ok(b) = u8::from_str_radix(hex, 16) {
                out.push(b);
                i += 3;
                continue;
            }
        }
        out.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

struct Product {
    id: String,
    files: Vec<(String, String)>,
}

fn parse_products(body: &str) -> Vec<Product> {
    let mut products = Vec::new();
    for entry in blocks(body, "atom:entry") {
        let Some(id) = tag_value(entry, "dc:identifier") else {
            continue;
        };
        let mut files = Vec::new();
        for link in entry.split("<atom:link ").skip(1) {
            let tag = link.split('>').next().unwrap_or("");
            let Some(rel) = attr(tag, "rel") else {
                continue;
            };
            let Some(href) = attr(tag, "href") else {
                continue;
            };
            if rel == "sip-entries" && href.contains("name=") {
                let Some(name) = href.split("name=").nth(1) else {
                    continue;
                };
                if name.ends_with(".nc") {
                    files.push((percent_decode(name), href));
                }
            }
        }
        products.push(Product { id, files });
    }
    products
}

fn search_url(collection: &str, count: usize) -> String {
    format!("{SEARCH_URL}?pi={collection}&c={count}")
}

fn bearer_token() -> Option<String> {
    let key = secret("EUMETSAT_KEY")?;
    let secret = secret("EUMETSAT_SECRET")?;
    let auth = format!("{key}:{secret}");
    let out = Command::new("curl")
        .arg("-sS")
        .arg("-X")
        .arg("POST")
        .arg("-u")
        .arg(&auth)
        .arg("-d")
        .arg("grant_type=client_credentials")
        .arg(TOKEN_URL)
        .output()
        .ok()?;
    if !out.status.success() {
        eprintln!(
            "token: {} — the fetch stays unwritten (0 honored)",
            out.status
        );
        return None;
    }
    let text = String::from_utf8(out.stdout).ok()?;
    let json = parse_json(&text)?;
    jstr(&json, "access_token")
}

fn http_code(href: &str, token: &str) -> Option<String> {
    let out = Command::new("curl")
        .arg("-sS")
        .arg("-o")
        .arg("/dev/null")
        .arg("-w")
        .arg("%{http_code}")
        .arg("-H")
        .arg(format!("Authorization: Bearer {token}"))
        .arg(href)
        .output()
        .ok()?;
    Some(String::from_utf8_lossy(&out.stdout).trim().to_string())
}

fn download(href: &str, token: &str, out: &str) -> Option<u64> {
    let output = Command::new("curl")
        .arg("-sSLf")
        .arg("--retry")
        .arg("2")
        .arg("--max-time")
        .arg("900")
        .arg("-H")
        .arg(format!("Authorization: Bearer {token}"))
        .arg("-o")
        .arg(out)
        .arg(href)
        .output()
        .ok()?;
    if !output.status.success() {
        eprintln!(
            "download {href}: {} — the asset stays unwritten (0 honored)",
            output.status
        );
        return None;
    }
    std::fs::metadata(out).ok().map(|m| m.len())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let collection = match arg_value(&args, "--collection") {
        Some(c) => c,
        None => DEFAULT_COLLECTION.to_string(),
    };
    let count = arg_value(&args, "--count")
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(1);
    let check_only = has_flag(&args, "--check");

    let Some(body) = fetch_raw_bytes(&search_url(&collection, count)) else {
        eprintln!("{collection}: OpenSearch fetch void — the arm stays unwritten (0 honored)");
        std::process::exit(1);
    };
    let text = String::from_utf8_lossy(&body);
    let products = parse_products(&text);
    if products.is_empty() {
        eprintln!("{collection}: no product in the OpenSearch feed — the arm stays unwritten");
        std::process::exit(1);
    }
    eprintln!(
        "{collection}: {} product(s), newest {} with {} netCDF entr{}",
        products.len(),
        products[0].id,
        products[0].files.len(),
        if products[0].files.len() == 1 {
            "y"
        } else {
            "ies"
        }
    );
    if check_only {
        match bearer_token() {
            Some(token) => {
                eprintln!("token: obtained (Bearer flow resolves)");
                if let Some((_, href)) = products[0].files.first() {
                    let code = match http_code(href, &token) {
                        Some(c) => c,
                        None => "void".to_string(),
                    };
                    eprintln!("download entry (Bearer): HTTP {code}");
                }
            }
            None => eprintln!("token: void — the credential-bearing route stays pending"),
        }
        return;
    }
    let Some(token) = bearer_token() else {
        eprintln!("{collection}: no bearer token — the download stays unwritten (0 honored)");
        std::process::exit(1);
    };
    let url_collection = collection.replace(':', "%3A");
    let mut written = 0usize;
    for product in &products {
        let dir = format!("{DATA_ROOT}/{url_collection}");
        std::fs::create_dir_all(&dir).ok();
        for (name, href) in &product.files {
            let base = name.rsplit('/').next().unwrap_or(name);
            let out = format!("{dir}/{base}");
            if let Some(size) = download(href, &token, &out) {
                eprintln!("{out}: {size} B");
                written += 1;
            }
        }
    }
    eprintln!("{collection}: {written} file(s) written");
}
