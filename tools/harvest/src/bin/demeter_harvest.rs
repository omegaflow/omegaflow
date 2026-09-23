use std::collections::BTreeSet;
use std::fs;
use std::io::{BufReader, Read, Write};
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
const TOKEN_REFRESH_SECS: u64 = 2700;
const RETRY_PAUSE_SECS: u64 = 90;

fn now() -> u64 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(d) => d.as_secs(),
        Err(e) => {
            eprintln!("the system clock lies before the UNIX epoch: {e}");
            std::process::exit(1);
        }
    }
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

struct HttpReply {
    status: u16,
    body: String,
}

fn fetch_http(
    url: &str,
    method: &str,
    body: Option<&str>,
    headers: &[(String, String)],
) -> Option<HttpReply> {
    let mut cmd = Command::new("curl");
    cmd.arg("-s")
        .arg("-S")
        .arg("-g")
        .arg("-m")
        .arg("45")
        .arg("--connect-timeout")
        .arg("20")
        .arg("-w")
        .arg("\n%{http_code}");
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
    if !output.status.success() {
        eprintln!(
            "http {} transport void ({}): {} {}",
            method,
            output.status,
            url,
            String::from_utf8_lossy(&output.stderr).trim()
        );
        return None;
    }
    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    let (body, code) = stdout.rsplit_once('\n')?;
    let status: u16 = code.trim().parse().ok()?;
    Some(HttpReply {
        status,
        body: body.to_string(),
    })
}

fn req_json(token: &str, url: &str) -> Option<JsonVal> {
    let reply = fetch_http(url, "GET", None, &auth_header(token))?;
    if !(200..300).contains(&reply.status) {
        return None;
    }
    parse_json(&reply.body)
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
            .map(|t| t as usize);
        println!(
            "harvest: catalog page {page} ({len} URNs)",
            len = urns.len()
        );
        if let Some(tp) = total_pages {
            if page + 1 >= tp {
                break;
            }
        }
        page += 1;
        std::thread::sleep(std::time::Duration::from_millis(250));
    }
    urns
}
fn load_urns(token: &str) -> Vec<String> {
    let path = match std::env::var("DEMETER_URNS") {
        Ok(p) => p,
        Err(_) => "tmp/demeter_urns.txt".into(),
    };
    let existing = match fs::read_to_string(&path) {
        Ok(body) => body
            .lines()
            .map(|l| l.trim().to_string())
            .filter(|l| !l.is_empty())
            .collect::<Vec<String>>(),
        Err(_) => Vec::new(),
    };
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

fn clear_basket(token: &str) -> Option<u16> {
    let url = format!("{ORDER}/order/basket");
    fetch_http(&url, "DELETE", None, &auth_header(token)).map(|r| r.status)
}

#[derive(Debug)]
enum CreateError {
    WafBlocked,
    Unauthorized,
    HttpStatus(u16),
    UnparsableOk,
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
    match clear_basket(token) {
        Some(s) if (200..300).contains(&s) => {}
        Some(401) => {
            eprintln!("create {label}: basket clear 401 — token stale");
            return Err(CreateError::Unauthorized);
        }
        Some(s) => {
            eprintln!("create {label}: basket clear status {s}");
            return Err(CreateError::HttpStatus(s));
        }
        None => {
            eprintln!("create {label}: basket clear transport void");
            return Err(CreateError::CreateFailed);
        }
    }
    let mut sel_headers = auth_header(token);
    sel_headers.push(("Content-Type".into(), "application/json".into()));
    let sel_reply = match fetch_http(&basket_url, "POST", Some(&sel), &sel_headers) {
        Some(r) => r,
        None => {
            eprintln!("create {label}: selection transport void");
            return Err(CreateError::CreateFailed);
        }
    };
    if sel_reply.status == 401 {
        eprintln!("create {label}: selection 401 — token stale");
        return Err(CreateError::Unauthorized);
    }
    if !(200..300).contains(&sel_reply.status) {
        eprintln!("create {label}: selection status {}", sel_reply.status);
        return Err(CreateError::HttpStatus(sel_reply.status));
    }
    let Some(basket) = parse_json(&sel_reply.body) else {
        eprintln!("create {label}: selection unparsable");
        return Err(CreateError::UnparsableOk);
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
    let reply = match fetch_http(&order_url, "POST", Some(&order_body), &headers) {
        Some(r) => r,
        None => {
            eprintln!("create {label}: order transport void");
            return Err(CreateError::CreateFailed);
        }
    };
    if reply.status == 401 {
        eprintln!("create {label}: order 401 — token stale");
        return Err(CreateError::Unauthorized);
    }
    if reply.body.contains("Request Rejected") {
        eprintln!("create {label}: F5 ASM block (Request Rejected)");
        return Err(CreateError::WafBlocked);
    }
    if !(200..300).contains(&reply.status) {
        eprintln!("create {label}: order status {}", reply.status);
        return Err(CreateError::HttpStatus(reply.status));
    }
    let v = match parse_json(&reply.body) {
        Some(v) => v,
        None => {
            if let Some(oid) = find_order_by_label(token, label) {
                return Ok(oid);
            }
            eprintln!(
                "create {label}: order unparsable 200: {}",
                reply.body.chars().take(80).collect::<String>()
            );
            return Err(CreateError::UnparsableOk);
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
        let Some(lab) = jstr(item, "label") else {
            continue;
        };
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
    let st = match jstr(c, "status") {
        Some(s) => s,
        None => "UNKNOWN".to_string(),
    };
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

fn status_is_final(st: &str) -> bool {
    st == "DONE" || st == "DONE_WITH_WARNING"
}

fn status_has_live_files(st: &str) -> bool {
    st == "RUNNING"
}

fn status_is_dead(st: &str) -> bool {
    st == "ERROR" || st == "FAILED" || st == "EXPIRED"
}

enum DownloadOutcome {
    Complete(PathBuf),
    Incomplete(PathBuf),
    Void,
}

fn content_length(headers: &Path) -> Option<u64> {
    let body = fs::read_to_string(headers).ok()?;
    let mut last = None;
    for line in body.lines() {
        if let Some(rest) = line.to_ascii_lowercase().strip_prefix("content-length:") {
            if let Ok(n) = rest.trim().parse::<u64>() {
                last = Some(n);
            }
        }
    }
    last
}

fn download_zip(token: &str, oid: i64, dest: &Path) -> DownloadOutcome {
    let part = dest.with_extension("zip.part");
    let headers = dest.with_extension("zip.hdr");
    let url = format!("{ORDER}/user/orders/{oid}/download");
    let out = Command::new("curl")
        .arg("-s")
        .arg("-S")
        .arg("-L")
        .arg("--connect-timeout")
        .arg("20")
        .arg("--speed-time")
        .arg("120")
        .arg("--speed-limit")
        .arg("1")
        .arg("-o")
        .arg(&part)
        .arg("-D")
        .arg(&headers)
        .arg("-w")
        .arg("%{http_code}")
        .arg("-H")
        .arg(format!("Authorization: Bearer {token}"))
        .arg(url)
        .output();
    let (exit_ok, code) = match out {
        Ok(o) => {
            if !o.status.success() {
                eprintln!(
                    "download {oid} transport void ({}): {}",
                    o.status,
                    String::from_utf8_lossy(&o.stderr).trim()
                );
            }
            (
                o.status.success(),
                String::from_utf8_lossy(&o.stdout).trim().to_string(),
            )
        }
        Err(e) => {
            eprintln!("download {oid} curl absent: {e}");
            return DownloadOutcome::Void;
        }
    };
    let got = match fs::metadata(&part) {
        Ok(m) => m.len(),
        Err(e) => {
            eprintln!("download {oid} part {} unreadable: {e}", part.display());
            let _ = fs::remove_file(&part);
            return DownloadOutcome::Void;
        }
    };
    let declared = content_length(&headers);
    let _ = fs::remove_file(&headers);
    let http_ok = exit_ok && (code.is_empty() || code.starts_with('2'));
    let complete = http_ok
        && got > 0
        && match declared {
            Some(n) => got == n,
            None => true,
        };
    if complete {
        return match fs::rename(&part, dest) {
            Ok(()) => DownloadOutcome::Complete(dest.to_path_buf()),
            Err(e) => {
                eprintln!("download {oid} rename {} void: {e}", part.display());
                DownloadOutcome::Void
            }
        };
    }
    if got > 0 {
        eprintln!(
            "download {oid} incomplete stream — {got} bytes of {} at {}",
            match declared {
                Some(n) => n.to_string(),
                None => "an undeclared length".to_string(),
            },
            part.display()
        );
        DownloadOutcome::Incomplete(part)
    } else {
        let _ = fs::remove_file(&part);
        eprintln!("download {oid} no bytes (transport_ok={exit_ok}, http={code})");
        DownloadOutcome::Void
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

struct ExtractReport {
    extracted: usize,
    truncated: bool,
}

fn consume_descriptor(reader: &mut BufReader<fs::File>) -> bool {
    let mut first = [0u8; 4];
    if reader.read_exact(&mut first).is_err() {
        return false;
    }
    if first == *b"PK\x07\x08" {
        let mut rest = [0u8; 12];
        reader.read_exact(&mut rest).is_ok()
    } else {
        let mut rest = [0u8; 8];
        reader.read_exact(&mut rest).is_ok()
    }
}

fn skip_bytes(reader: &mut BufReader<fs::File>, len: usize) -> bool {
    let mut remaining = len as u64;
    let mut buf = [0u8; 8192];
    while remaining > 0 {
        let want = (buf.len() as u64).min(remaining) as usize;
        match reader.read_exact(&mut buf[..want]) {
            Ok(()) => remaining -= want as u64,
            Err(_) => return false,
        }
    }
    true
}

fn copy_stored(reader: &mut BufReader<fs::File>, comp_size: usize, out: &Path) -> Option<bool> {
    let mut outfile = match fs::File::create(out) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("extract {}: create void: {e}", out.display());
            return Some(false);
        }
    };
    let mut remaining = comp_size as u64;
    let mut buf = [0u8; 65536];
    while remaining > 0 {
        let want = (buf.len() as u64).min(remaining) as usize;
        if reader.read_exact(&mut buf[..want]).is_err() {
            return None;
        }
        if let Err(e) = outfile.write_all(&buf[..want]) {
            eprintln!("extract {}: write void: {e}", out.display());
            return Some(false);
        }
        remaining -= want as u64;
    }
    Some(true)
}

fn read_and_inflate(
    reader: &mut BufReader<fs::File>,
    comp_size: usize,
    out: &Path,
) -> Option<bool> {
    let mut body = vec![0u8; comp_size];
    if reader.read_exact(&mut body).is_err() {
        return None;
    }
    let Some(uncompressed) = omegaflow::inflate::inflate(&body) else {
        eprintln!("extract {}: deflate payload unreadable", out.display());
        return Some(false);
    };
    match fs::write(out, &uncompressed) {
        Ok(()) => Some(true),
        Err(e) => {
            eprintln!("extract {}: write void: {e}", out.display());
            Some(false)
        }
    }
}

fn extract_dat(workdir: &Path, zip_path: &Path) -> ExtractReport {
    let mut report = ExtractReport {
        extracted: 0,
        truncated: false,
    };
    let file = match fs::File::open(zip_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("extract {}: open void: {e}", zip_path.display());
            return report;
        }
    };
    let mut reader = BufReader::new(file);
    loop {
        let mut sig = [0u8; 4];
        match reader.read_exact(&mut sig) {
            Ok(()) => {}
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => break,
            Err(e) => {
                eprintln!("extract {}: read void: {e}", zip_path.display());
                report.truncated = true;
                break;
            }
        }
        if sig != *b"PK\x03\x04" {
            eprintln!(
                "extract {}: lost the local-file signature {:02x?} — partial named",
                zip_path.display(),
                sig
            );
            report.truncated = true;
            break;
        }
        let mut fixed = [0u8; 26];
        if reader.read_exact(&mut fixed).is_err() {
            report.truncated = true;
            break;
        }
        let flags = le16(&fixed, 2);
        let method = le16(&fixed, 4);
        let comp_size = le32(&fixed, 14);
        let name_len = le16(&fixed, 22);
        let extra_len = le16(&fixed, 24);
        let mut name_buf = vec![0u8; name_len];
        if reader.read_exact(&mut name_buf).is_err() {
            report.truncated = true;
            break;
        }
        let name = String::from_utf8_lossy(&name_buf).into_owned();
        if extra_len > 0 {
            let mut extra = vec![0u8; extra_len];
            if reader.read_exact(&mut extra).is_err() {
                report.truncated = true;
                break;
            }
        }
        let has_descriptor = flags & 0x08 != 0;
        if has_descriptor && comp_size == 0 {
            eprintln!(
                "extract: {name} carries a data descriptor without a local size — partial named"
            );
            report.truncated = true;
            break;
        }
        if !name.ends_with(".DAT") {
            if !skip_bytes(&mut reader, comp_size) {
                report.truncated = true;
                break;
            }
            if has_descriptor && !consume_descriptor(&mut reader) {
                report.truncated = true;
                break;
            }
            continue;
        }
        let base = name.rsplit('/').next().unwrap_or(&name).to_string();
        let out = workdir.join(&base);
        let written = match method {
            0 => copy_stored(&mut reader, comp_size, &out),
            8 => read_and_inflate(&mut reader, comp_size, &out),
            _ => {
                eprintln!("extract: {name} uses compression method {method} — unported, skipped");
                if !skip_bytes(&mut reader, comp_size) {
                    report.truncated = true;
                    break;
                }
                if has_descriptor && !consume_descriptor(&mut reader) {
                    report.truncated = true;
                    break;
                }
                continue;
            }
        };
        match written {
            Some(true) => report.extracted += 1,
            Some(false) => {}
            None => {
                report.truncated = true;
                break;
            }
        }
        if has_descriptor && !consume_descriptor(&mut reader) {
            report.truncated = true;
            break;
        }
    }
    report
}

#[derive(Clone)]
struct Slot {
    label: String,
    oid: i64,
    pulled_partial: bool,
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
            return Err(std::io::Error::other(
                "login void — the series stays unharvested",
            ));
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
        return Err(std::io::Error::other("catalog void — nothing to order"));
    }
    let urns: Vec<String> = urns.into_iter().take(urn_limit).collect();

    let batches: Vec<Vec<String>> = urns.chunks(BATCH).map(|c| c.to_vec()).collect();
    let mut slots: Vec<Slot> = Vec::new();
    let mut idx = 0usize;
    let mut harvested_files = 0usize;
    let mut last_token_refresh = start;

    while idx < batches.len() || !slots.is_empty() {
        if now().saturating_sub(start) >= budget {
            println!("harvest: budget {budget}s reached — stopping (resume via ledger)");
            break;
        }
        if now().saturating_sub(last_token_refresh) >= TOKEN_REFRESH_SECS {
            if let Some(t) = login(&user, &pass) {
                token = t;
                last_token_refresh = now();
                println!("harvest: token refreshed");
            }
        }
        while slots.len() < SLOTS && idx < batches.len() {
            if now().saturating_sub(start) >= budget {
                break;
            }
            let label = format!("demeter_{idx:04}");
            if done.contains(&label) {
                idx += 1;
                continue;
            }
            if now().saturating_sub(last_token_refresh) >= TOKEN_REFRESH_SECS {
                if let Some(t) = login(&user, &pass) {
                    token = t;
                    last_token_refresh = now();
                    println!("harvest: token refreshed before create");
                }
            }
            match create_order(&token, &batches[idx], &label) {
                Ok(oid) => {
                    slots.push(Slot {
                        label: label.clone(),
                        oid,
                        pulled_partial: false,
                    });
                    println!("harvest: created {oid} {label} (slots {})", slots.len());
                    std::thread::sleep(std::time::Duration::from_secs(CREATE_PAUSE_SECS));
                }
                Err(CreateError::WafBlocked) => {
                    eprintln!(
                        "harvest: WAF blocked {label} — waiting {}s",
                        WAF_BACKOFF_SECS
                    );
                    std::thread::sleep(std::time::Duration::from_secs(WAF_BACKOFF_SECS));
                }
                Err(CreateError::Unauthorized) => match login(&user, &pass) {
                    Some(t) => {
                        token = t;
                        last_token_refresh = now();
                        println!("harvest: token refreshed after 401");
                        std::thread::sleep(std::time::Duration::from_secs(5));
                        continue;
                    }
                    None => {
                        eprintln!("harvest: login void after 401 — stopping");
                        break;
                    }
                },
                Err(CreateError::HttpStatus(s)) => {
                    eprintln!("harvest: create {label} http {s} — retry in {RETRY_PAUSE_SECS}s");
                    std::thread::sleep(std::time::Duration::from_secs(RETRY_PAUSE_SECS));
                    break;
                }
                Err(CreateError::UnparsableOk) => {
                    eprintln!(
                        "harvest: create {label} unparsable 200 — retry in {RETRY_PAUSE_SECS}s"
                    );
                    std::thread::sleep(std::time::Duration::from_secs(RETRY_PAUSE_SECS));
                    break;
                }
                Err(CreateError::CreateFailed) => {
                    eprintln!("harvest: create {label} void — retry in {RETRY_PAUSE_SECS}s");
                    std::thread::sleep(std::time::Duration::from_secs(RETRY_PAUSE_SECS));
                    break;
                }
            }
            idx += 1;
        }
        if slots.is_empty() && idx >= batches.len() {
            break;
        }
        if now().saturating_sub(last_token_refresh) >= TOKEN_REFRESH_SECS {
            if let Some(t) = login(&user, &pass) {
                token = t;
                last_token_refresh = now();
                println!("harvest: token refreshed");
            }
        }
        let mut progressed = false;
        let snapshot = slots.clone();
        let mut remove: Vec<String> = Vec::new();
        let mut mark_partial: Vec<i64> = Vec::new();
        for slot in &snapshot {
            let (st, avail) = order_status(&token, slot.oid);
            if status_is_dead(&st) {
                eprintln!("harvest: {} {st} — removed from slots", slot.label);
                remove.push(slot.label.clone());
                progressed = true;
                continue;
            }
            let final_order = status_is_final(&st);
            let live_with_files = status_has_live_files(&st) && avail > 0;
            let should_download = (final_order) || (live_with_files && !slot.pulled_partial);
            if !should_download {
                continue;
            }
            let zip = Path::new(&workdir).join(format!("{}.zip", slot.oid));
            match download_zip(&token, slot.oid, &zip) {
                DownloadOutcome::Complete(path) => {
                    let report = extract_dat(Path::new(&workdir), &path);
                    harvested_files += report.extracted;
                    let _ = fs::remove_file(&path);
                    if final_order {
                        if report.truncated {
                            eprintln!(
                                "harvest: {} {st} entry stream truncated after {} files — kept for the next run",
                                slot.label, report.extracted
                            );
                        } else {
                            println!(
                                "harvest: {} {st} +{} files (total {harvested_files})",
                                slot.label, report.extracted
                            );
                            done.insert(slot.label.clone());
                            if let Ok(mut f) = fs::OpenOptions::new()
                                .create(true)
                                .append(true)
                                .open(&ledger_path)
                            {
                                let _ = writeln!(f, "{}", slot.label);
                            }
                        }
                        remove.push(slot.label.clone());
                    } else {
                        mark_partial.push(slot.oid);
                        println!(
                            "harvest: {} RUNNING +{} files (partial, total {harvested_files})",
                            slot.label, report.extracted
                        );
                    }
                    progressed = true;
                }
                DownloadOutcome::Incomplete(path) => {
                    let report = extract_dat(Path::new(&workdir), &path);
                    harvested_files += report.extracted;
                    let _ = fs::remove_file(&path);
                    if final_order {
                        remove.push(slot.label.clone());
                    } else {
                        mark_partial.push(slot.oid);
                    }
                    eprintln!(
                        "harvest: {} {st} incomplete stream +{} files named partial (total {harvested_files})",
                        slot.label, report.extracted
                    );
                    progressed = true;
                }
                DownloadOutcome::Void => {
                    eprintln!(
                        "harvest: {} {st} download void — retry next cycle",
                        slot.label
                    );
                }
            }
        }
        if !remove.is_empty() {
            slots.retain(|s| !remove.contains(&s.label));
        }
        for s in slots.iter_mut() {
            if mark_partial.contains(&s.oid) {
                s.pulled_partial = true;
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

    if harvested_files == 0 && done.len() < batches.len() {
        eprintln!(
            "harvest: 0 files harvested, {}/{} orders done — the series stays unwritten",
            done.len(),
            batches.len()
        );
        return Err(std::io::Error::other(
            "0 files harvested with the ledger incomplete",
        ));
    }

    if ci_mode {
        let compiler = match std::env::current_exe() {
            Ok(exe) => match exe.parent() {
                Some(par) => par.join("demeter_compiler"),
                None => PathBuf::from("demeter_compiler"),
            },
            Err(_) => PathBuf::from("demeter_compiler"),
        };
        let status = Command::new(&compiler)
            .arg("--aggregate")
            .arg(&workdir)
            .arg("--ci-mode")
            .status();
        match status {
            Ok(st) if st.success() => {}
            Ok(st) => {
                return Err(std::io::Error::other(format!(
                    "aggregate+upload exited {st} — the series stays unmanifested"
                )));
            }
            Err(e) => {
                return Err(std::io::Error::other(format!(
                    "aggregate+upload spawn void: {e}"
                )));
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
