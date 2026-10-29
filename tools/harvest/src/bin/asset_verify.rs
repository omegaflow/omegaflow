use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};

#[derive(Clone, Debug)]
struct Entry {
    asset: String,
    url: String,
    format: String,
    sha256: Option<String>,
    size: usize,
    api: String,
}

fn gh_token() -> Result<String, String> {
    let out = Command::new("gh")
        .args(["auth", "token"])
        .output()
        .map_err(|e| format!("gh auth token returned void: {e}"))?;
    if !out.status.success() {
        return Err("gh auth token reported a failure".to_string());
    }
    let token = String::from_utf8_lossy(&out.stdout).trim().to_string();
    if token.is_empty() {
        return Err("gh auth token carries nothing".to_string());
    }
    Ok(token)
}

fn catalogue() -> Result<BTreeMap<String, (usize, String)>, String> {
    let jq = r#".[].assets[] | "\(.browser_download_url) \(.size) \(.id)""#;
    let out = Command::new("gh")
        .args([
            "api",
            "--paginate",
            "repos/omegaflow/sources/releases?per_page=100",
            "--jq",
            jq,
        ])
        .output()
        .map_err(|e| format!("gh api catalogue returned void: {e}"))?;
    if !out.status.success() {
        return Err(format!("the release catalogue answers {}", out.status));
    }
    let text = String::from_utf8_lossy(&out.stdout);
    let mut map = BTreeMap::new();
    for line in text.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != 3 {
            continue;
        }
        let Ok(size) = parts[1].parse::<usize>() else {
            continue;
        };
        let url = parts[0].to_string();
        map.insert(
            url,
            (
                size,
                format!(
                    "https://api.github.com/repos/omegaflow/sources/releases/assets/{}",
                    parts[2]
                ),
            ),
        );
    }
    Ok(map)
}

fn entries(register: &str, only: Option<&str>) -> Result<Vec<Entry>, String> {
    let text =
        fs::read_to_string(register).map_err(|e| format!("read {register} returned void: {e}"))?;
    let mut blocks: Vec<BTreeMap<String, String>> = Vec::new();
    let mut current: BTreeMap<String, String> = BTreeMap::new();
    for line in text.lines() {
        if line.trim().is_empty() {
            if !current.is_empty() {
                blocks.push(std::mem::take(&mut current));
            }
            continue;
        }
        let mut parts = line.splitn(2, char::is_whitespace);
        let key = parts.next().unwrap_or("");
        let value = parts.next().unwrap_or("").trim();
        if !key.is_empty() {
            current.insert(key.to_string(), value.to_string());
        }
    }
    if !current.is_empty() {
        blocks.push(current);
    }
    let mut out = Vec::new();
    for block in blocks {
        let Some(url) = block.get("url") else {
            continue;
        };
        if !url.ends_with(".bin") {
            continue;
        }
        let format = match block.get("format") {
            Some(name) => name.clone(),
            None => continue,
        };
        if let Some(only) = only
            && format != only
        {
            continue;
        }
        out.push(Entry {
            asset: url.rsplit('/').next().unwrap_or("unnamed").to_string(),
            url: url.clone(),
            format,
            sha256: block.get("sha256").cloned(),
            size: 0,
            api: String::new(),
        });
    }
    Ok(out)
}

fn stream_hash(token: &str, api: &str) -> Result<String, String> {
    let script = format!(
        "curl -sSfL --max-time 3600 -H 'Authorization: Bearer {token}' -H 'Accept: application/octet-stream' '{api}' | sha256sum"
    );
    let mut child = Command::new("sh")
        .args(["-c", &script])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("sh returned void: {e}"))?;
    let out = child
        .wait_with_output()
        .map_err(|e| format!("the hash run returned void: {e}"))?;
    if !out.status.success() {
        return Err(format!(
            "the stream reported {}: {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        ));
    }
    let text = String::from_utf8_lossy(&out.stdout).trim().to_string();
    let hash = text.split_whitespace().next().unwrap_or("").to_string();
    if hash.len() != 64 {
        return Err(format!("sha256sum returned {text:?}"));
    }
    Ok(hash)
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().skip(1).collect();
    let usage = "usage: asset_verify --register <phi/sources.φ> --max-bytes <n> [--format <name>] [--limit <n>] [--out <tsv>] [--append]";
    let mut register = None;
    let mut only: Option<String> = None;
    let mut max_bytes: Option<usize> = None;
    let mut limit = usize::MAX;
    let mut out: Option<String> = None;
    let mut append = false;
    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--register" => {
                register = Some(
                    args.get(index + 1)
                        .ok_or_else(|| usage.to_string())?
                        .clone(),
                );
                index += 2;
            }
            "--format" => {
                only = Some(
                    args.get(index + 1)
                        .ok_or_else(|| usage.to_string())?
                        .clone(),
                );
                index += 2;
            }
            "--max-bytes" => {
                max_bytes = Some(
                    args.get(index + 1)
                        .ok_or_else(|| usage.to_string())?
                        .parse::<usize>()
                        .map_err(|_| "max-bytes carries no number".to_string())?,
                );
                index += 2;
            }
            "--limit" => {
                limit = args
                    .get(index + 1)
                    .ok_or_else(|| usage.to_string())?
                    .parse::<usize>()
                    .map_err(|_| "limit carries no number".to_string())?;
                index += 2;
            }
            "--out" => {
                out = Some(
                    args.get(index + 1)
                        .ok_or_else(|| usage.to_string())?
                        .clone(),
                );
                index += 2;
            }
            "--append" => {
                append = true;
                index += 1;
                continue;
            }
            other => return Err(format!("{other}: not a flag of this tool")),
        }
    }
    let register = register.ok_or_else(|| usage.to_string())?;
    let max_bytes = max_bytes.ok_or_else(|| usage.to_string())?;
    let token = gh_token()?;
    let catalogue = catalogue()?;
    let mut list = entries(&register, only.as_deref())?;
    let mut missing = 0usize;
    for entry in list.iter_mut() {
        match catalogue.get(&entry.url) {
            Some((size, api)) => {
                entry.size = *size;
                entry.api = api.clone();
            }
            None => missing += 1,
        }
    }
    list.retain(|entry| entry.size > 0 && entry.size <= max_bytes);
    list.sort_by_key(|entry| entry.size);
    println!(
        "candidates {}  (not in the catalogue {missing})  budget {} bytes",
        list.len(),
        max_bytes
    );

    let mut sink: Option<std::io::BufWriter<fs::File>> = match &out {
        None => None,
        Some(path) => {
            let file = if append {
                fs::OpenOptions::new()
                    .append(true)
                    .open(path)
                    .map_err(|e| format!("open {path} for append returned void: {e}"))?
            } else {
                fs::File::create(path).map_err(|e| format!("create {path} returned void: {e}"))?
            };
            let mut writer = std::io::BufWriter::new(file);
            if !append {
                writeln!(writer, "asset\tformat\tbytes\tsha256_head\tverdict")
                    .map_err(|e| format!("write {path} returned void: {e}"))?;
            }
            writer
                .flush()
                .map_err(|e| format!("flush {path} returned void: {e}"))?;
            Some(writer)
        }
    };

    let mut gated = 0usize;
    let mut absent = 0usize;
    let mut seen = 0usize;
    for entry in list.iter().take(limit) {
        seen += 1;
        let line = match &entry.sha256 {
            None => {
                absent += 1;
                format!(
                    "{}\t{}\t{}\tabsent\tthe register carries no sha256",
                    entry.asset, entry.format, entry.size
                )
            }
            Some(wanted) => match stream_hash(&token, &entry.api) {
                Err(reason) => {
                    absent += 1;
                    format!(
                        "{}\t{}\t{}\tabsent\t{reason}",
                        entry.asset, entry.format, entry.size
                    )
                }
                Ok(actual) => {
                    if actual == *wanted {
                        gated += 1;
                        format!(
                            "{}\t{}\t{}\t{}\tgated-copy",
                            entry.asset, entry.format, entry.size, actual
                        )
                    } else {
                        absent += 1;
                        format!(
                            "{}\t{}\t{}\t{}\tabsent: the stream hashes {actual}, the register {wanted}",
                            entry.asset, entry.format, entry.size, actual
                        )
                    }
                }
            },
        };
        eprintln!("{line}");
        if let Some(writer) = sink.as_mut() {
            writeln!(writer, "{line}").map_err(|e| format!("write returned void: {e}"))?;
            writer
                .flush()
                .map_err(|e| format!("flush returned void: {e}"))?;
        }
    }
    println!("gated-copy {gated}  absent {absent}  of {seen} streamed");
    Ok(())
}
