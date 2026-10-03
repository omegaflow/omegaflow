use omegaflow::archivar::load_env;
use omegaflow::archivar::sha256::sha256_hex;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;

const HOST: &str = "https://pradan.issdc.gov.in";
const AUTH: &str = "login-actions/authenticate";
const PAYLOAD_SUBDIR: &str = "protected/downloadFile/class_holder";
const LIST_PAGE: &str = "protected/miscDownloads.xhtml";
const DEFAULT_TIMEOUT_S: &str = "3600";
const SUMMARY: &str =
    "final=%{url_effective} code=%{http_code} redirects=%{num_redirects} type=%{content_type}";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn secrets() -> HashMap<String, String> {
    let mut env = load_env();
    if let Ok(repo) = std::env::var("OMEGAFLOW_REPO") {
        let path = PathBuf::from(repo).join(".secrets.local");
        if let Ok(content) = std::fs::read_to_string(&path) {
            for line in content.lines() {
                let line = line.trim();
                if line.is_empty() || line.starts_with('#') {
                    continue;
                }
                if let Some(eq) = line.find('=') {
                    env.entry(line[..eq].trim().to_string())
                        .or_insert_with(|| line[eq + 1..].trim().to_string());
                }
            }
        }
    }
    env
}

fn credential(env: &HashMap<String, String>, key: &str) -> String {
    match env.get(key).filter(|v| !v.is_empty()) {
        Some(v) => v.clone(),
        None => {
            eprintln!("pradan_ch2_compiler: {key} absent in .secrets.local — the login stays void");
            std::process::exit(2);
        }
    }
}

fn curl(args: &[String]) -> Option<std::process::Output> {
    match Command::new("curl").args(args).output() {
        Ok(o) => {
            if o.status.success() {
                Some(o)
            } else {
                eprintln!(
                    "pradan_ch2_compiler: curl returned ({}): {}",
                    o.status,
                    String::from_utf8_lossy(&o.stderr).trim()
                );
                None
            }
        }
        Err(e) => {
            eprintln!("pradan_ch2_compiler: curl did not run: {e}");
            None
        }
    }
}

fn location_header(headers: &str) -> Option<String> {
    headers.lines().find_map(|l| {
        let (k, v) = l.split_once(':')?;
        if k.trim().eq_ignore_ascii_case("location") {
            Some(v.trim().to_string())
        } else {
            None
        }
    })
}

fn login_form_action(html: &str) -> Option<String> {
    let marker = html.find(AUTH)?;
    let open = html[..marker].rfind('"')?;
    let rest = &html[open + 1..];
    let close = rest.find('"')?;
    Some(rest[..close].replace("&amp;", "&"))
}

fn magic_ok(bytes: &[u8], expected: Option<&str>) -> bool {
    match expected {
        Some("zip") => bytes.starts_with(b"PK\x03\x04"),
        Some("fits") => bytes.starts_with(b"SIMPLE"),
        _ => !bytes.is_empty(),
    }
}

fn timeout_arg(args: &[String]) -> String {
    match arg_value(args, "--timeout-s") {
        Some(v) => v,
        None => DEFAULT_TIMEOUT_S.to_string(),
    }
}

fn login_fetch(
    target: &str,
    out: &str,
    jar_s: &str,
    args: &[String],
    require_login: bool,
    user: &str,
    pass: &str,
) -> Result<String, String> {
    let head = curl(&[
        "-s".into(),
        "-D".into(),
        "-".into(),
        "-o".into(),
        "/dev/null".into(),
        "-c".into(),
        jar_s.to_string(),
        "-b".into(),
        jar_s.to_string(),
        "--connect-timeout".into(),
        "30".into(),
        target.to_string(),
    ])
    .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
    .ok_or_else(|| format!("{target}: curl did not complete the first request"))?;

    let auth_url =
        match location_header(&head).filter(|u| u.contains("protocol/openid-connect/auth")) {
            Some(u) => u,
            None if !require_login => {
                let o = curl(&[
                    "-s".into(),
                    "-L".into(),
                    "-c".into(),
                    jar_s.to_string(),
                    "-b".into(),
                    jar_s.to_string(),
                    "--connect-timeout".into(),
                    "30".into(),
                    "--max-time".into(),
                    timeout_arg(args),
                    "-o".into(),
                    out.to_string(),
                    "-w".into(),
                    SUMMARY.into(),
                    target.to_string(),
                ])
                .ok_or_else(|| format!("{target}: curl did not complete the public fetch"))?;
                return Ok(String::from_utf8_lossy(&o.stdout).to_string());
            }
            None => {
                return Err(format!(
                    "{target}: no OIDC redirect — the path is not behind the login (0 honored)"
                ));
            }
        };

    let page = curl(&[
        "-s".into(),
        "-c".into(),
        jar_s.to_string(),
        "-b".into(),
        jar_s.to_string(),
        "--connect-timeout".into(),
        "30".into(),
        auth_url,
    ])
    .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
    .ok_or_else(|| "curl did not complete the IdP page request".to_string())?;
    let action = login_form_action(&page).ok_or_else(|| {
        "Keycloak carried no login form (AUTH action absent) — the flow stops at the IdP page"
            .to_string()
    })?;

    if let Some(parent) = Path::new(out).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let posted = curl(&[
        "-s".into(),
        "-L".into(),
        "-c".into(),
        jar_s.to_string(),
        "-b".into(),
        jar_s.to_string(),
        "--connect-timeout".into(),
        "30".into(),
        "--max-time".into(),
        timeout_arg(args),
        "-o".into(),
        out.to_string(),
        "-w".into(),
        SUMMARY.into(),
        "--data-urlencode".into(),
        format!("username={user}"),
        "--data-urlencode".into(),
        format!("password={pass}"),
        "-d".into(),
        "credentialId=".into(),
        "-d".into(),
        "login=Sign In".into(),
        action,
    ])
    .ok_or_else(|| "the login POST did not complete".to_string())?;
    Ok(String::from_utf8_lossy(&posted.stdout).to_string())
}

fn contains_bytes(hay: &[u8], needle: &[u8]) -> bool {
    !needle.is_empty() && hay.windows(needle.len()).any(|w| w == needle)
}

fn login_page(body: &[u8]) -> bool {
    contains_bytes(body, b"kc-form-login")
}

#[derive(Debug, Clone)]
struct Product {
    payload: String,
    year: String,
    month: String,
    filename: String,
}

impl Product {
    fn url(&self) -> String {
        format!("{HOST}/ch2/{PAYLOAD_SUBDIR}/{}", self.filename)
    }
}

fn parse_product(token: &str) -> Option<Product> {
    let stem = token.strip_suffix(".zip")?;
    let rest = stem.strip_prefix("ch2_")?;
    let idx = rest.rfind("_l1_")?;
    let payload = &rest[..idx];
    let (year, month) = rest[idx + 4..].split_once('_')?;
    if payload.is_empty()
        || year.len() != 4
        || month.len() != 2
        || !year.bytes().all(|b| b.is_ascii_digit())
        || !month.bytes().all(|b| b.is_ascii_digit())
    {
        return None;
    }
    Some(Product {
        payload: payload.to_string(),
        year: year.to_string(),
        month: month.to_string(),
        filename: token.to_string(),
    })
}

fn products_from_html(html: &str) -> Vec<Product> {
    let bytes = html.as_bytes();
    let needle = b".zip";
    let mut out: Vec<Product> = Vec::new();
    let mut i = 0;
    while i + needle.len() <= bytes.len() {
        if &bytes[i..i + needle.len()] == needle {
            let mut start = i;
            while start > 0 {
                let c = bytes[start - 1];
                if c.is_ascii_alphanumeric() || c == b'_' || c == b'-' || c == b'.' {
                    start -= 1;
                } else {
                    break;
                }
            }
            if let Some(p) = parse_product(&html[start..i + needle.len()]) {
                if !out.iter().any(|q| q.filename == p.filename) {
                    out.push(p);
                }
            }
            i += needle.len();
        } else {
            i += 1;
        }
    }
    out.sort_by(|a, b| (&a.year, &a.month, &a.filename).cmp(&(&b.year, &b.month, &b.filename)));
    out
}

fn verify_artifact(out: &str, expected: Option<&str>) -> Result<Vec<u8>, String> {
    let bytes = std::fs::read(out).map_err(|e| format!("read {out}: {e}"))?;
    if login_page(&bytes) {
        let _ = std::fs::remove_file(out);
        return Err(format!(
            "{out} carries the login page ({}) — the credentials did not open the session",
            bytes.len()
        ));
    }
    if !magic_ok(&bytes, expected) {
        let _ = std::fs::remove_file(out);
        return Err(format!(
            "{out}: {} bytes with unexpected magic ({expected:?}) — the artifact stays unverified",
            bytes.len()
        ));
    }
    Ok(bytes)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let env = secrets();
    let user = credential(&env, "PRADAN_USER");
    let pass = credential(&env, "PRADAN_PASS");

    let jar = std::env::temp_dir().join(format!("pradan_ch2_{}.jar", std::process::id()));
    let jar_s = jar.to_string_lossy().to_string();
    let list_url = format!("{HOST}/ch2/{LIST_PAGE}");

    let do_list = args.iter().any(|a| a == "--list");
    let do_latest = args.iter().any(|a| a == "--latest");

    if do_list {
        let list_path =
            std::env::temp_dir().join(format!("pradan_ch2_list_{}.html", std::process::id()));
        let list_s = list_path.to_string_lossy().to_string();
        let summary = match login_fetch(&list_url, &list_s, &jar_s, &args, false, &user, &pass) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("pradan_ch2_compiler: {e}");
                let _ = std::fs::remove_file(&jar);
                std::process::exit(1);
            }
        };
        eprintln!("pradan_ch2_compiler: catalog {list_url} — {summary}");
        let bytes = match std::fs::read(&list_path) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("pradan_ch2_compiler: read {}: {e}", list_path.display());
                let _ = std::fs::remove_file(&jar);
                std::process::exit(1);
            }
        };
        let _ = std::fs::remove_file(&list_path);
        let _ = std::fs::remove_file(&jar);
        if login_page(&bytes) {
            eprintln!(
                "pradan_ch2_compiler: the catalog carried the login page ({} bytes) — the credentials did not open the session",
                bytes.len()
            );
            std::process::exit(1);
        }
        let html = String::from_utf8_lossy(&bytes);
        let products = products_from_html(&html);
        for p in &products {
            println!(
                "file={} payload={} year={} month={}",
                p.filename, p.payload, p.year, p.month
            );
        }
        println!("count={}", products.len());
        if products.is_empty() {
            eprintln!(
                "pradan_ch2_compiler: the catalog ({} bytes) carried no ch2_*_l1_*_*.zip entry — no product listed (0 honored)",
                bytes.len()
            );
        }
        return;
    }

    if do_latest {
        let list_path =
            std::env::temp_dir().join(format!("pradan_ch2_list_{}.html", std::process::id()));
        let list_s = list_path.to_string_lossy().to_string();
        let summary = match login_fetch(&list_url, &list_s, &jar_s, &args, false, &user, &pass) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("pradan_ch2_compiler: {e}");
                let _ = std::fs::remove_file(&jar);
                std::process::exit(1);
            }
        };
        eprintln!("pradan_ch2_compiler: catalog {list_url} — {summary}");
        let bytes = match std::fs::read(&list_path) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("pradan_ch2_compiler: read {}: {e}", list_path.display());
                let _ = std::fs::remove_file(&jar);
                std::process::exit(1);
            }
        };
        let _ = std::fs::remove_file(&list_path);
        if login_page(&bytes) {
            eprintln!(
                "pradan_ch2_compiler: the catalog carried the login page ({} bytes) — the credentials did not open the session",
                bytes.len()
            );
            let _ = std::fs::remove_file(&jar);
            std::process::exit(1);
        }
        let html = String::from_utf8_lossy(&bytes);
        let products = products_from_html(&html);
        let latest = match products.last() {
            Some(p) => p.clone(),
            None => {
                eprintln!(
                    "pradan_ch2_compiler: the catalog ({} bytes) carried no ch2_*_l1_*_*.zip entry — no product to choose (0 honored)",
                    bytes.len()
                );
                let _ = std::fs::remove_file(&jar);
                std::process::exit(1);
            }
        };
        println!("selected={}", latest.filename);
        let out = match arg_value(&args, "--out") {
            Some(o) => o,
            None => format!("data/pradan.issdc.gov.in/{}", latest.filename),
        };
        if let Some(parent) = Path::new(&out).parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let got = curl(&[
            "-s".into(),
            "-L".into(),
            "-c".into(),
            jar_s.clone(),
            "-b".into(),
            jar_s.clone(),
            "--connect-timeout".into(),
            "30".into(),
            "--max-time".into(),
            timeout_arg(&args),
            "-o".into(),
            out.clone(),
            "-w".into(),
            SUMMARY.into(),
            latest.url(),
        ])
        .ok_or_else(|| "the product fetch did not complete".to_string());
        let _ = std::fs::remove_file(&jar);
        match got {
            Ok(o) => eprintln!(
                "pradan_ch2_compiler: {}",
                String::from_utf8_lossy(&o.stdout)
            ),
            Err(e) => {
                eprintln!("pradan_ch2_compiler: {e} — {out} stays unwritten");
                std::process::exit(1);
            }
        }
        let bytes = match verify_artifact(&out, Some("zip")) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("pradan_ch2_compiler: {e}");
                std::process::exit(1);
            }
        };
        let sha = sha256_hex(&bytes);
        println!("pradan_ch2: {} bytes, sha256 {sha} -> {out}", bytes.len());
        return;
    }

    let (target, expected) = match arg_value(&args, "--url") {
        Some(url) => {
            let kind = if url.ends_with(".zip") {
                Some("zip")
            } else if url.ends_with(".fits") {
                Some("fits")
            } else {
                None
            };
            (url, kind)
        }
        None => {
            let payload = match arg_value(&args, "--payload") {
                Some(p) => p.to_ascii_lowercase(),
                None => {
                    eprintln!(
                        "pradan_ch2_compiler: --url <https>, --payload <token> or --list/--latest absent — refused"
                    );
                    std::process::exit(2);
                }
            };
            let year = match arg_value(&args, "--year") {
                Some(y) => y,
                None => {
                    eprintln!(
                        "pradan_ch2_compiler: --year <YYYY> required with --payload — refused"
                    );
                    std::process::exit(2);
                }
            };
            let month = match arg_value(&args, "--month") {
                Some(m) => m,
                None => {
                    eprintln!(
                        "pradan_ch2_compiler: --month <MM> required with --payload — refused"
                    );
                    std::process::exit(2);
                }
            };
            if year.len() != 4 || month.len() != 2 {
                eprintln!(
                    "pradan_ch2_compiler: --year <YYYY> and --month <MM> required with --payload"
                );
                std::process::exit(2);
            }
            (
                format!("{HOST}/ch2/{PAYLOAD_SUBDIR}/ch2_{payload}_l1_{year}_{month}.zip"),
                Some("zip"),
            )
        }
    };

    let filename = match target.rsplit('/').next().filter(|f| !f.is_empty()) {
        Some(f) => f.to_string(),
        None => {
            eprintln!("pradan_ch2_compiler: {target}: no file name in the target path — refused");
            std::process::exit(1);
        }
    };
    let out = match arg_value(&args, "--out") {
        Some(o) => o,
        None => format!("data/pradan.issdc.gov.in/{filename}"),
    };

    let summary = match login_fetch(&target, &out, &jar_s, &args, true, &user, &pass) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("pradan_ch2_compiler: {e}");
            let _ = std::fs::remove_file(&jar);
            std::process::exit(1);
        }
    };
    let _ = std::fs::remove_file(&jar);
    eprintln!("pradan_ch2_compiler: {summary}");

    let bytes = match verify_artifact(&out, expected) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("pradan_ch2_compiler: {e}");
            std::process::exit(1);
        }
    };
    let sha = sha256_hex(&bytes);
    println!("pradan_ch2: {} bytes, sha256 {sha} -> {out}", bytes.len());
}
