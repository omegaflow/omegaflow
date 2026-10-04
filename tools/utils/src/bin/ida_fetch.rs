use std::env;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

const IDA_LOGIN: &str = "https://ida.loni.usc.edu/login.jsp?prompt=true";

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        None | Some("--login") => login(),
        Some("--get") => match args.get(1) {
            Some(url) => get(url),
            None => usage(),
        },
        _ => usage(),
    }
}

fn usage() -> ! {
    eprintln!("usage: ida_fetch --login | --get <url>");
    std::process::exit(2);
}

fn jar() -> PathBuf {
    if let Ok(p) = env::var("OMEGAFLOW_IDA_COOKIE_JAR") {
        if !p.is_empty() {
            return PathBuf::from(p);
        }
    }
    let Ok(home) = env::var("HOME") else {
        eprintln!("ida_fetch: HOME absent — set OMEGAFLOW_IDA_COOKIE_JAR");
        std::process::exit(2);
    };
    let dir = Path::new(&home).join(".cache").join("omegaflow");
    let _ = fs::create_dir_all(&dir);
    dir.join("ida-cookies.txt")
}

fn secrets_path() -> Option<PathBuf> {
    if let Ok(p) = env::var("OMEGAFLOW_SECRETS_FILE") {
        let p = PathBuf::from(p);
        if p.is_file() {
            return Some(p);
        }
    }
    let cwd = env::current_dir().ok()?;
    cwd.ancestors()
        .map(|d| d.join(".secrets.local"))
        .find(|f| f.is_file())
}

fn secret(name: &str) -> Option<String> {
    let text = fs::read_to_string(secrets_path()?).ok()?;
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((k, v)) = line.split_once('=') {
            if k.trim() == name {
                let v = v.trim();
                if !v.is_empty() {
                    return Some(v.to_string());
                }
            }
        }
    }
    None
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

fn login() {
    let Some(email) = secret("IDA_USER").or_else(|| secret("IDA_E-MAIL")) else {
        eprintln!("ida_fetch: IDA_USER/IDA_E-MAIL absent in .secrets.local");
        std::process::exit(2);
    };
    let Some(pass) = secret("IDA_PASS") else {
        eprintln!("ida_fetch: IDA_PASS absent in .secrets.local");
        std::process::exit(2);
    };
    let jar = jar();
    let jar_s = jar.to_string_lossy().to_string();
    let body = format!(
        "userEmail={}&userPassword={}&project=",
        urlencode(&email),
        urlencode(&pass)
    );
    let mut child = match Command::new("curl")
        .arg("-s")
        .arg("--max-time")
        .arg("60")
        .arg("-c")
        .arg(&jar_s)
        .arg("-b")
        .arg(&jar_s)
        .arg("-H")
        .arg("Content-Type: application/x-www-form-urlencoded")
        .arg("--data")
        .arg("@-")
        .arg("-o")
        .arg("/dev/null")
        .arg("-w")
        .arg("%{http_code} %{redirect_url}")
        .arg(IDA_LOGIN)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
    {
        Ok(c) => c,
        Err(e) => {
            eprintln!("ida_fetch: curl spawn void: {e}");
            std::process::exit(1);
        }
    };
    if let Some(mut stdin) = child.stdin.take() {
        let _ = stdin.write_all(body.as_bytes());
    }
    let out = match child.wait_with_output() {
        Ok(o) => o,
        Err(e) => {
            eprintln!("ida_fetch: curl void: {e}");
            std::process::exit(1);
        }
    };
    let meta = String::from_utf8_lossy(&out.stdout);
    let mut parts = meta.split_whitespace();
    let code = match parts.next() {
        Some(c) => c,
        None => {
            eprintln!("ida_fetch: curl returned no status");
            std::process::exit(1);
        }
    };
    let redirect: Option<&str> = parts.next();
    if !code.starts_with('3') && redirect.is_none() {
        eprintln!(
            "ida_fetch: no session (http {code}, no redirect) — the IDA gate requires a Cloudflare \
             Turnstile token, which a credential-only POST cannot carry"
        );
        std::process::exit(2);
    }
    let mut note = String::new();
    if let Some(r) = redirect {
        note = format!(" -> {r}");
    }
    println!("ida_fetch: login ok (http {code}{note}; jar {})", jar.display());
}

fn get(url: &str) {
    let jar = jar();
    let jar_s = jar.to_string_lossy().to_string();
    let out = match Command::new("curl")
        .arg("-sL")
        .arg("--max-time")
        .arg("60")
        .arg("-b")
        .arg(&jar_s)
        .arg("-w")
        .arg("\n__HTTP__%{http_code}")
        .arg(url)
        .output()
    {
        Ok(o) => o,
        Err(e) => {
            eprintln!("ida_fetch: curl void: {e}");
            std::process::exit(1);
        }
    };
    let text = String::from_utf8_lossy(&out.stdout);
    match text.rsplit_once("\n__HTTP__") {
        Some((body, code)) => {
            println!("ida_fetch: http {}", code.trim());
            print!("{body}");
        }
        None => print!("{text}"),
    }
}
