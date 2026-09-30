use omegaflow::archivar::load_env;
use omegaflow::archivar::sha256::sha256_hex;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;

const HOST: &str = "https://pradan.issdc.gov.in";
const AUTH: &str = "login-actions/authenticate";
const PAYLOAD_SUBDIR: &str = "protected/downloadFile/class_holder";
const DEFAULT_TIMEOUT_S: &str = "3600";

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

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let env = secrets();
    let user = credential(&env, "PRADAN_USER");
    let pass = credential(&env, "PRADAN_PASS");

    let jar = std::env::temp_dir().join(format!("pradan_ch2_{}.jar", std::process::id()));
    let jar_s = jar.to_string_lossy().to_string();

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
                        "pradan_ch2_compiler: --payload <token> or --url <https> absent — refused"
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

    let head = curl(&[
        "-s".into(),
        "-D".into(),
        "-".into(),
        "-o".into(),
        "/dev/null".into(),
        "-c".into(),
        jar_s.clone(),
        "-b".into(),
        jar_s.clone(),
        "--connect-timeout".into(),
        "30".into(),
        target.clone(),
    ])
    .map(|o| String::from_utf8_lossy(&o.stdout).to_string());
    let auth_url = match head.as_deref().and_then(location_header) {
        Some(u) if u.contains("protocol/openid-connect/auth") => u,
        _ => {
            eprintln!(
                "pradan_ch2_compiler: {target}: no OIDC redirect — the path is not behind the login (0 honored)"
            );
            std::process::exit(1);
        }
    };

    let page = curl(&[
        "-s".into(),
        "-c".into(),
        jar_s.clone(),
        "-b".into(),
        jar_s.clone(),
        "--connect-timeout".into(),
        "30".into(),
        auth_url,
    ])
    .map(|o| String::from_utf8_lossy(&o.stdout).to_string());
    let action = match page.as_deref().and_then(login_form_action) {
        Some(a) => a,
        None => {
            eprintln!(
                "pradan_ch2_compiler: Keycloak carried no login form (AUTH action absent) — the flow stops at the IdP page"
            );
            let _ = std::fs::remove_file(&jar);
            std::process::exit(1);
        }
    };

    if let Some(parent) = Path::new(&out).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let posted = curl(&[
        "-s".into(),
        "-L".into(),
        "-c".into(),
        jar_s.clone(),
        "-b".into(),
        jar_s.clone(),
        "--connect-timeout".into(),
        "30".into(),
        "--max-time".into(),
        match arg_value(&args, "--timeout-s") {
            Some(t) => t,
            None => DEFAULT_TIMEOUT_S.to_string(),
        },
        "-o".into(),
        out.clone(),
        "-w".into(),
        "final=%{url_effective} code=%{http_code} redirects=%{num_redirects} type=%{content_type}"
            .into(),
        "--data-urlencode".into(),
        format!("username={user}"),
        "--data-urlencode".into(),
        format!("password={pass}"),
        "-d".into(),
        "credentialId=".into(),
        "-d".into(),
        "login=Sign In".into(),
        action,
    ]);
    let _ = std::fs::remove_file(&jar);
    match posted {
        Some(o) => eprintln!(
            "pradan_ch2_compiler: {}",
            String::from_utf8_lossy(&o.stdout)
        ),
        None => {
            eprintln!(
                "pradan_ch2_compiler: the login POST did not complete — {out} stays unwritten"
            );
            std::process::exit(1);
        }
    }

    let bytes = match std::fs::read(&out) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("pradan_ch2_compiler: read {out}: {e}");
            std::process::exit(1);
        }
    };
    if bytes.windows(7).any(|w| w == b"<html") || bytes.windows(14).any(|w| w == b"kc-form-login") {
        eprintln!(
            "pradan_ch2_compiler: {out} carries the login page ({}) — the credentials did not open the session",
            bytes.len()
        );
        let _ = std::fs::remove_file(&out);
        std::process::exit(1);
    }
    if !magic_ok(&bytes, expected) {
        eprintln!(
            "pradan_ch2_compiler: {out}: {} bytes with unexpected magic ({:?}) — the artifact stays unverified",
            bytes.len(),
            expected
        );
        let _ = std::fs::remove_file(&out);
        std::process::exit(1);
    }

    let sha = sha256_hex(&bytes);
    println!("pradan_ch2: {} bytes, sha256 {sha} -> {out}", bytes.len());
}
