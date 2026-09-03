use std::collections::BTreeSet;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use omegaflow::json::{JsonVal, jpath_val, jstr, parse_json};

const AUTH: &str = "https://regards.cnes.fr/api/v1/rs-authentication/oauth/token";
const ORDER: &str = "https://regards.cnes.fr/api/v1/rs-order";
const CAT: &str = "https://regards.cnes.fr/api/v1/rs-catalog";
const DATASET_QUERY: &str = "DatasetName:DMT_N1_1144";
const BATCH: usize = 100;
const SLOTS: usize = 1;
const DEFAULT_BUDGET_SECS: u64 = 5 * 3600;
const CREATE_PAUSE_SECS: u64 = 45;
const WAF_BACKOFF_SECS: u64 = 1800;

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

fn auth_header(token: &str) -> Vec<(String, String)> {
    vec![("Authorization".into(), format!("Bearer {token}"))]
}

fn login(user: &str, pass: &str) -> Option<String> {
    let url = format!("{AUTH}?grant_type=password&scope=cdpp&username={user}&password={pass}");
    let out = Command::new("curl")
        .arg("-s")
        .arg("-S")
        .arg("-f")
        .arg("-X")
        .arg("POST")
        .arg("-m")
        .arg("45")
        .arg("-H")
        .arg("Authorization: Basic Y2xpZW50OnNlY3JldA==")
        .arg(url)
        .output()
        .ok()?;
    if !out.status.success() {
        eprintln!(
            "login returned {}: {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        return None;
    }
    let body = String::from_utf8_lossy(&out.stdout).into_owned();
    let v = parse_json(&body)?;
    jstr(&v, "access_token")
}

fn fetch_http(
    url: &str,
    method: &str,
    body: Option<&str>,
    headers: &[(String, String)],
) -> Option<String> {
    let mut cmd = Command::new("curl");
    cmd.arg("-s")
        .arg("-S")
        .arg("-f")
        .arg("-g")
        .arg("-m")
        .arg("45")
        .arg("--connect-timeout")
        .arg("20");
    if method != "GET" {
        cmd.arg("-X").arg(method);
    }
    if let Some(b) = body {
        cmd.arg("-d").arg(b);
    }
    for (k, v) in headers {
        cmd.arg("-H").arg(format!("{}: {}", k, v));
    }
    cmd.arg(url);
    let output = cmd.output().ok()?;
    if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).into_owned())
    } else {
        eprintln!(
            "http {} returned ({}): {} {}",
            method,
            output.status,
            url,
            String::from_utf8_lossy(&output.stderr).trim()
        );
        None
    }
}

fn req_json(token: &str, url: &str) -> Option<JsonVal> {
    let body = fetch_http(url, "GET", None, &auth_header(token))?;
    parse_json(&body)
}

fn req_json_post(token: &str, url: &str, json: &str) -> Option<JsonVal> {
    let mut headers = auth_header(token);
    headers.push(("Content-Type".into(), "application/json".into()));
    let body = fetch_http(url, "POST", Some(json), &headers)?;
    parse_json(&body)
}

fn scan_urns(token: &str) -> Vec<String> {
    let mut urns = Vec::new();
    let mut page = 0usize;
    loop {
        let url = format!(
            "{CAT}/engines/legacy/dataobjects/search?q={DATASET_QUERY}&size=1000&page={page}"
        );
        let Some(v) = req_json(token, &url) else {
            break;
        };
        let Some(content) = jpath_val(&v, "content") else {
            break;
        };
        let JsonVal::Arr(arr) = content else { break };
        if arr.is_empty() {
            break;
        }
        for item in arr {
            if let Some(id) = jstr(item, "content.id") {
                urns.push(id);
            }
        }
        let total_pages = jpath_val(&v, "metadata.totalPages")
            .and_then(|t| match t {
                JsonVal::Num(n) => Some(*n as i64),
                JsonVal::Str(s) => s.parse::<i64>().ok(),
                _ => None,
            })
            .unwrap_or(0);
        println!(
            "harvest: catalog page {page} ({len} URNs)",
            len = urns.len()
        );
        if page + 1 >= total_pages as usize {
            break;
        }
        page += 1;
        std::thread::sleep(std::time::Duration::from_millis(250));
    }
    urns
}

fn load_urns(token: &str) -> Vec<String> {
    let path =
        std::env::var("DEMETER_URNS").unwrap_or_else(|_| "/tmp/opencode/demeter_urns.txt".into());
    let existing = fs::read_to_string(&path)
        .map(|body| {
            body.lines()
                .map(|l| l.trim().to_string())
                .filter(|l| !l.is_empty())
                .collect::<Vec<String>>()
        })
        .unwrap_or_default();
    if !existing.is_empty() {
        return existing;
    }
    let urns = scan_urns(token);
    if !urns.is_empty() {
        let mut out = String::new();
        for u in &urns {
            out.push_str(u);
            out.push('\n');
        }
        let _ = fs::write(&path, out);
        println!("harvest: wrote {path} ({len} URNs)", len = urns.len());
    }
    urns
}

fn clear_basket(token: &str) -> bool {
    let url = format!("{ORDER}/order/basket");
    fetch_http(&url, "DELETE", None, &auth_header(token)).is_some()
}

#[derive(Debug)]
enum CreateError {
    WafBlocked,
    CreateFailed,
}

fn create_order(token: &str, batch: &[String], label: &str) -> Result<i64, CreateError> {
    let sel = format!(
        r#"{{"engineType":"legacy","datasetUrn":null,"entityIdsToInclude":[{}],"entityIdsToExclude":null,"searchParameters":{{"q":[""]}}}}"#,
        batch
            .iter()
            .map(|u| format!("\"{u}\""))
            .collect::<Vec<_>>()
            .join(",")
    );
    let basket_url = format!("{ORDER}/order/basket/selection");
    if !clear_basket(token) {
        eprintln!("create {label}: basket clear void");
        return Err(CreateError::CreateFailed);
    }
    let Some(basket) = req_json_post(token, &basket_url, &sel) else {
        eprintln!("create {label}: basket void ({} URNs)", batch.len());
        return Err(CreateError::CreateFailed);
    };
    let quota = jpath_val(&basket, "quota").and_then(|q| match q {
        JsonVal::Num(n) => Some(*n as i64),
        _ => None,
    });
    if let Some(q) = quota {
        if q < 1 {
            eprintln!("create {label}: basket quota {q} — nothing selected");
            return Err(CreateError::CreateFailed);
        }
    }
    let order_body = format!(r#"{{"label":"{label}","onSuccessUrl":null}}"#);
    let order_url = format!("{ORDER}/user/orders");
    let mut headers = auth_header(token);
    headers.push(("Content-Type".into(), "application/json".into()));
    let raw = fetch_http(&order_url, "POST", Some(&order_body), &headers);
    let body = match raw {
        Some(b) => b,
        None => {
            eprintln!("create {label}: order POST void");
            return Err(CreateError::CreateFailed);
        }
    };
    let parsed = parse_json(&body);
    let v = match parsed {
        Some(v) => v,
        None => {
            if let Some(oid) = find_order_by_label(token, label) {
                return Ok(oid);
            }
            eprintln!(
                "create {label}: order parse void: {}",
                body.chars().take(80).collect::<String>()
            );
            return Err(CreateError::WafBlocked);
        }
    };
    let id = jpath_val(&v, "content.id")
        .and_then(|x| match x {
            JsonVal::Num(n) => Some(*n as i64),
            _ => None,
        })
        .or_else(|| jstr(&v, "content.id").and_then(|s| s.parse::<i64>().ok()));
    if let Some(oid) = id {
        return Ok(oid);
    }
    eprintln!("create {label}: no id in order response: {v:?}");
    Err(CreateError::CreateFailed)
}

fn find_order_by_label(token: &str, label: &str) -> Option<i64> {
    let url = format!("{ORDER}/user/orders?label={label}&size=5&sort=id,desc");
    let v = req_json(token, &url)?;
    let content = jpath_val(&v, "content")?;
    let JsonVal::Arr(arr) = content else {
        return None;
    };
    for item in arr {
        let lab = jstr(item, "label").unwrap_or_default();
        if lab == label {
            return jpath_val(item, "id").and_then(|x| match x {
                JsonVal::Num(n) => Some(*n as i64),
                _ => None,
            });
        }
    }
    None
}

fn order_status(token: &str, oid: i64) -> (String, i64) {
    let url = format!("{ORDER}/user/orders/{oid}");
    let default = ("UNKNOWN".to_string(), -1);
    let Some(v) = req_json(token, &url) else {
        return default;
    };
    let Some(c) = jpath_val(&v, "content") else {
        return default;
    };
    let st = jstr(c, "status").unwrap_or_default();
    let avail = jnum_i64(c, "availableFilesCount");
    (st, avail)
}

fn jnum_i64(v: &JsonVal, key: &str) -> i64 {
    jpath_val(v, key)
        .and_then(|x| match x {
            JsonVal::Num(n) => Some(*n as i64),
            _ => None,
        })
        .unwrap_or(-1)
}

fn download_zip(token: &str, oid: i64, dest: &Path) -> bool {
    let url = format!("{ORDER}/user/orders/{oid}/download");
    let out = Command::new("curl")
        .arg("-s")
        .arg("-S")
        .arg("-f")
        .arg("-L")
        .arg("-m")
        .arg("600")
        .arg("-H")
        .arg(format!("Authorization: Bearer {token}"))
        .arg(url)
        .output();
    match out {
        Ok(o) if o.status.success() && !o.stdout.is_empty() => fs::write(dest, &o.stdout).is_ok(),
        Ok(o) => {
            eprintln!(
                "download {oid} void ({}): {}",
                o.status,
                String::from_utf8_lossy(&o.stderr).trim()
            );
            false
        }
        Err(e) => {
            eprintln!("download {oid} curl absent: {e}");
            false
        }
    }
}

fn le16(d: &[u8], off: usize) -> usize {
    d[off] as usize | (d[off + 1] as usize) << 8
}

fn le32(d: &[u8], off: usize) -> usize {
    d[off] as usize
        | (d[off + 1] as usize) << 8
        | (d[off + 2] as usize) << 16
        | (d[off + 3] as usize) << 24
}

fn zip_entries(data: &[u8]) -> Vec<(String, usize, usize, usize)> {
    let mut out = Vec::new();
    let mut i = 0usize;
    while i + 46 <= data.len() {
        if &data[i..i + 4] == b"PK\x01\x02" {
            let method = le16(data, i + 10);
            let comp_size = le32(data, i + 20);
            let name_len = le16(data, i + 28);
            let extra_len = le16(data, i + 30);
            let comment_len = le16(data, i + 32);
            let local_off = le32(data, i + 42);
            if i + 46 + name_len <= data.len() {
                let name = String::from_utf8_lossy(&data[i + 46..i + 46 + name_len]).into_owned();
                out.push((name, method, comp_size, local_off));
            }
            i += 46 + name_len + extra_len + comment_len;
            continue;
        }
        i += 1;
    }
    out
}

fn extract_dat(workdir: &Path, zip_path: &Path) -> usize {
    let data = match fs::read(zip_path) {
        Ok(d) => d,
        Err(_) => return 0,
    };
    let entries = zip_entries(&data);
    let mut count = 0usize;
    for (name, method, comp_size, local_off) in entries {
        if !name.ends_with(".DAT") {
            continue;
        }
        if local_off + 30 > data.len() {
            continue;
        }
        let name_len = le16(&data, local_off + 26);
        let extra_len = le16(&data, local_off + 28);
        let start = local_off + 30 + name_len + extra_len;
        if start + comp_size > data.len() {
            continue;
        }
        let body = &data[start..start + comp_size];
        let uncompressed = match method {
            0 => Some(body.to_vec()),
            8 => omegaflow::inflate::inflate(body),
            _ => None,
        };
        let Some(uncompressed) = uncompressed else {
            continue;
        };
        let base = name.rsplit('/').next().unwrap_or(&name);
        let out = workdir.join(base);
        if fs::write(&out, &uncompressed).is_ok() {
            count += 1;
        }
    }
    count
}

fn run() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut budget = DEFAULT_BUDGET_SECS;
    let mut workdir = "demeter_work".to_string();
    let mut ledger_path = "demeter_ledger.txt".to_string();
    let mut ci_mode = false;
    let mut urn_limit: usize = usize::MAX;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--budget" => {
                i += 1;
                if i < args.len() {
                    budget = args[i].parse().unwrap_or(DEFAULT_BUDGET_SECS);
                }
            }
            "--workdir" => {
                i += 1;
                if i < args.len() {
                    workdir = args[i].clone();
                }
            }
            "--ledger" => {
                i += 1;
                if i < args.len() {
                    ledger_path = args[i].clone();
                }
            }
            "--urn-limit" => {
                i += 1;
                if i < args.len() {
                    urn_limit = args[i].parse().unwrap_or(usize::MAX);
                }
            }
            "--ci-mode" => ci_mode = true,
            _ => {}
        }
        i += 1;
    }

    let user = std::env::var("CDPP_USER")
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::NotFound, "CDPP_USER env absent"))?;
    let pass = std::env::var("CDPP_PASS")
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::NotFound, "CDPP_PASS env absent"))?;

    fs::create_dir_all(&workdir)?;
    let start = now();

    let mut token = match login(&user, &pass) {
        Some(t) => t,
        None => {
            eprintln!("demeter_harvest: login void — the series stays unharvested (0 honored)");
            return Ok(());
        }
    };
    println!("harvest: token acquired ({})", token.len());

    let mut done: BTreeSet<String> = BTreeSet::new();
    if let Ok(body) = fs::read_to_string(&ledger_path) {
        for l in body.lines() {
            if !l.trim().is_empty() {
                done.insert(l.trim().to_string());
            }
        }
    }
    println!("harvest: {} orders already done in ledger", done.len());

    let urns = load_urns(&token);
    println!("harvest: {} URNs", urns.len());
    if urns.is_empty() {
        eprintln!("harvest: catalog void — nothing to order (0 honored)");
        return Ok(());
    }
    let urns: Vec<String> = urns.into_iter().take(urn_limit).collect();

    let batches: Vec<Vec<String>> = urns.chunks(BATCH).map(|c| c.to_vec()).collect();
    let mut slots: Vec<(String, i64)> = Vec::new();
    let mut idx = 0usize;
    let mut harvested_files = 0usize;
    let mut last_token_refresh = start;

    while idx < batches.len() || !slots.is_empty() {
        if now().saturating_sub(start) >= budget {
            println!("harvest: budget {budget}s reached — stopping (resume via ledger)");
            break;
        }
        if now().saturating_sub(last_token_refresh) >= 2700 {
            if let Some(t) = login(&user, &pass) {
                token = t;
                last_token_refresh = now();
                println!("harvest: token refreshed");
            }
        }
        while slots.len() < SLOTS && idx < batches.len() {
            let label = format!("demeter_{idx:04}");
            if done.contains(&label) {
                idx += 1;
                continue;
            }
            match create_order(&token, &batches[idx], &label) {
                Ok(oid) => {
                    slots.push((label.clone(), oid));
                    println!("harvest: created {oid} {label} (slots {}/3)", slots.len());
                    std::thread::sleep(std::time::Duration::from_secs(CREATE_PAUSE_SECS));
                }
                Err(CreateError::WafBlocked) => {
                    eprintln!(
                        "harvest: WAF blocked {label} — waiting {}s",
                        WAF_BACKOFF_SECS
                    );
                    std::thread::sleep(std::time::Duration::from_secs(WAF_BACKOFF_SECS));
                }
                Err(CreateError::CreateFailed) => {
                    eprintln!("harvest: create {label} void — retry in 90s");
                    std::thread::sleep(std::time::Duration::from_secs(90));
                    break;
                }
            }
            idx += 1;
        }
        if slots.is_empty() && idx >= batches.len() {
            break;
        }
        let mut progressed = false;
        for (label, oid) in slots.clone() {
            let (st, _avail) = order_status(&token, oid);
            if st == "DONE" || st == "DELIVERED" {
                let zip = Path::new(&workdir).join(format!("{oid}.zip"));
                if download_zip(&token, oid, &zip) {
                    let n = extract_dat(Path::new(&workdir), &zip);
                    harvested_files += n;
                    println!("harvest: {label} DONE +{n} files (total {harvested_files})");
                    let _ = fs::remove_file(&zip);
                    done.insert(label.clone());
                    if let Ok(mut f) = fs::OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open(&ledger_path)
                    {
                        let _ = writeln!(f, "{label}");
                    }
                } else {
                    eprintln!("harvest: {label} DONE but zip void — retry");
                }
                slots.retain(|(l, _)| l != &label);
                progressed = true;
            } else if st == "ERROR" || st == "FAILED" || st == "EXPIRED" {
                eprintln!("harvest: {label} {st} — removed from slots");
                slots.retain(|(l, _)| l != &label);
                progressed = true;
            }
        }
        if !progressed {
            std::thread::sleep(std::time::Duration::from_secs(20));
        }
    }

    println!(
        "harvest: {} files on disk, {} orders done",
        harvested_files,
        done.len()
    );

    if ci_mode {
        let compiler = std::env::current_exe()?
            .parent()
            .map(|p| p.join("demeter_compiler"))
            .unwrap_or_else(|| PathBuf::from("demeter_compiler"));
        let st = Command::new(&compiler)
            .arg("--aggregate")
            .arg(&workdir)
            .arg("--ci-mode")
            .status();
        if let Ok(st) = st {
            if !st.success() {
                eprintln!("harvest: aggregate+upload void");
            }
        }
    }
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("demeter_harvest: {}", e);
        std::process::exit(1);
    }
}
