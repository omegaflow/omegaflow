use omegaflow::archivar::json::{JsonVal, parse_json};
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::archivar::{RetryPolicy, TRANSFER_BOUND_S, fetch_raw_with};
use omegaflow::cdn::{CDN_BASE, upload_release};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

const COMPILER: &str = "tools/harvest/src/bin/glofas_ewds_compiler.rs";
const CATALOG_URL: &str = "https://ewds.climate.copernicus.eu/api/catalogue/v1/collections";
const DEFAULT_PREFIX: &str = "cems-glofas";
const MANIFEST_NAME: &str = "glofas_ewds_coverage.manifest";
const MAX_ENTRIES: usize = 1_000;
const KEY_NAME: &str = "EWDS_API_TOKEN";
const EXECUTION_URL: &str =
    "https://ewds.climate.copernicus.eu/api/retrieve/v1/processes/cems-glofas-historical/execution";
const PROBE_BODY: &str = r#"{"inputs":{"system_version":"version_4_0","hydrological_model":"lisflood","product_type":"consolidated","timespan":"time_mean","variable":"average_river_discharge_in_the_last_24_hours","year":"2020","month":"01","day":"01","data_format":"grib2","download_format":"unarchived"}}"#;
const JOBS_BASE: &str = "https://ewds.climate.copernicus.eu/api/retrieve/v1/jobs";
const EWDS_NETLOC: &str = "ewds.climate.copernicus.eu";
const POLL_SECS: u64 = 6;
const MAX_POLLS: u32 = 40;
const DEFAULT_SYSTEM_VERSION: &str = "version_4_0";
const DEFAULT_PRODUCT_TYPE: &str = "consolidated";
const DEFAULT_TIMESPAN: &str = "time_mean";
const DEFAULT_VARIABLE: &str = "average_river_discharge_in_the_last_24_hours";
const DEFAULT_YEAR: &str = "2020";
const DEFAULT_MONTH: &str = "01";
const DEFAULT_DAY: &str = "01";
const DEFAULT_DATA_FORMAT: &str = "grib2";
const DEFAULT_DOWNLOAD_FORMAT: &str = "unarchived";

fn find_repo_root() -> Option<PathBuf> {
    if let Ok(env_root) = env::var("OMEGAFLOW_REPO") {
        if !env_root.is_empty() {
            let root = PathBuf::from(env_root);
            if root.join("phi").join("sources.\u{3c6}").is_file() {
                return Some(root);
            }
            return None;
        }
    }
    let mut starts: Vec<PathBuf> = Vec::new();
    if let Ok(cwd) = env::current_dir() {
        starts.push(cwd);
    }
    if let Ok(exe) = env::current_exe() {
        if let Some(dir) = exe.parent() {
            starts.push(dir.to_path_buf());
        }
    }
    for start in starts {
        let mut dir = start;
        loop {
            if dir.join("phi").join("sources.\u{3c6}").is_file() {
                return Some(dir);
            }
            match dir.parent() {
                Some(parent) => dir = parent.to_path_buf(),
                None => break,
            }
        }
    }
    None
}

fn read_secret(name: &str) -> Option<String> {
    let root = find_repo_root()?;
    let text = std::fs::read_to_string(root.join(".secrets.local")).ok()?;
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some(eq) = line.find('=') {
            if line[..eq].trim() == name {
                let value = line[eq + 1..].trim().trim_matches(['"', '\'']);
                if !value.is_empty() {
                    return Some(value.to_string());
                }
            }
        }
    }
    None
}

fn scalar_text(value: &JsonVal) -> Option<String> {
    match value {
        JsonVal::Str(s) if !s.is_empty() => Some(s.clone()),
        JsonVal::Num(n) => Some(format!("{n}")),
        JsonVal::Bool(b) => Some(format!("{b}")),
        JsonVal::Obj(map) => match map.get("msg") {
            Some(JsonVal::Str(s)) if !s.is_empty() => Some(s.clone()),
            _ => None,
        },
        _ => None,
    }
}

fn scalar_field(value: &JsonVal, key: &str) -> Option<String> {
    match value {
        JsonVal::Obj(map) => match map.get(key) {
            Some(JsonVal::Arr(items)) => {
                let parts: Vec<String> = items.iter().filter_map(scalar_text).collect();
                if parts.is_empty() {
                    None
                } else {
                    Some(parts.join("; "))
                }
            }
            Some(other) => scalar_text(other),
            None => None,
        },
        _ => None,
    }
}

fn probe() -> Result<(), String> {
    let token = read_secret(KEY_NAME).ok_or_else(|| {
        format!("{KEY_NAME} absent from .secrets.local — the data arm stays unmeasured")
    })?;
    let out = Command::new("curl")
        .args([
            "-sS",
            "--max-time",
            "60",
            "--connect-timeout",
            "15",
            "-X",
            "POST",
            EXECUTION_URL,
            "-H",
            "Content-Type: application/json",
            "-H",
            &format!("PRIVATE-TOKEN: {token}"),
            "-d",
            PROBE_BODY,
            "-w",
            "\n@@STATUS@@%{http_code}",
        ])
        .output()
        .map_err(|e| format!("curl invocation void: {e}"))?;
    let stdout = String::from_utf8_lossy(&out.stdout);
    let (body, status_text) = stdout
        .rsplit_once("@@STATUS@@")
        .ok_or_else(|| "execution: the response carries no HTTP status".to_string())?;
    let status: u16 = status_text
        .trim()
        .parse()
        .map_err(|_| format!("execution: HTTP status '{status_text}' is no code"))?;
    println!("http {status}");
    if let Some(root) = parse_json(body) {
        if (200..300).contains(&status) {
            if let Some(job) = str_field(&root, "jobID") {
                println!("jobID {job}");
            }
        } else if let Some(detail) = scalar_field(&root, "detail") {
            println!("detail {detail}");
        } else if let Some(title) = scalar_field(&root, "title") {
            println!("title {title}");
        }
    }
    if status == 401 || status == 403 {
        return Err(format!(
            "execution returned HTTP {status} — the key does not open the data arm"
        ));
    }
    Ok(())
}

fn json_escape(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

fn json_string_field(json: &str, key: &str) -> Option<String> {
    let key_pat = format!("\"{key}\"");
    let after_key = json.find(&key_pat)? + key_pat.len();
    let rest = &json[after_key..];
    let after_colon = rest.find(':')? + 1;
    let rest = &rest[after_colon..];
    let after_open = rest.find('"')? + 1;
    let rest = &rest[after_open..];
    let end = rest.find('"')?;
    Some(rest[..end].to_string())
}

fn json_number_field(json: &str, key: &str) -> Option<u64> {
    let key_pat = format!("\"{key}\"");
    let after_key = json.find(&key_pat)? + key_pat.len();
    let rest = &json[after_key..];
    let after_colon = rest.find(':')? + 1;
    let rest = rest[after_colon..].trim_start();
    let end = rest
        .find(|c: char| !c.is_ascii_digit())
        .unwrap_or(rest.len());
    if end == 0 {
        return None;
    }
    rest[..end].parse().ok()
}

fn arg_or(args: &[String], name: &str, default: &str) -> String {
    match arg_value(args, name).filter(|v| !v.is_empty()) {
        Some(v) => v,
        None => default.to_string(),
    }
}

fn request_body(
    system_version: &str,
    product_type: &str,
    timespan: &str,
    variable: &str,
    year: &str,
    month: &str,
    day: &str,
    data_format: &str,
    download_format: &str,
) -> String {
    format!(
        "{{\"inputs\":{{\"system_version\":\"{}\",\"hydrological_model\":\"lisflood\",\"product_type\":\"{}\",\"timespan\":\"{}\",\"variable\":\"{}\",\"year\":\"{}\",\"month\":\"{}\",\"day\":\"{}\",\"data_format\":\"{}\",\"download_format\":\"{}\"}}}}",
        json_escape(system_version),
        json_escape(product_type),
        json_escape(timespan),
        json_escape(variable),
        json_escape(year),
        json_escape(month),
        json_escape(day),
        json_escape(data_format),
        json_escape(download_format),
    )
}

fn curl_body_status(args: &[&str]) -> Result<(u16, String), String> {
    let out = Command::new("curl")
        .args(args)
        .output()
        .map_err(|e| format!("curl invocation void: {e}"))?;
    let stdout = String::from_utf8_lossy(&out.stdout);
    let (body, status_text) = stdout
        .rsplit_once("@@STATUS@@")
        .ok_or_else(|| "response: the body carries no HTTP status".to_string())?;
    let status: u16 = status_text
        .trim()
        .parse()
        .map_err(|_| format!("response: HTTP status '{status_text}' is no code"))?;
    Ok((status, body.to_string()))
}

fn fetch(args: &[String]) -> Result<(), String> {
    let token = read_secret(KEY_NAME).ok_or_else(|| {
        format!("{KEY_NAME} absent from .secrets.local — the data arm stays unmeasured")
    })?;
    let year = arg_or(args, "--year", DEFAULT_YEAR);
    let month = arg_or(args, "--month", DEFAULT_MONTH);
    let day = arg_or(args, "--day", DEFAULT_DAY);
    let variable = arg_or(args, "--variable", DEFAULT_VARIABLE);
    let timespan = arg_or(args, "--timespan", DEFAULT_TIMESPAN);
    let system_version = arg_or(args, "--system-version", DEFAULT_SYSTEM_VERSION);
    let product_type = arg_or(args, "--product-type", DEFAULT_PRODUCT_TYPE);
    let data_format = arg_or(args, "--data-format", DEFAULT_DATA_FORMAT);
    let download_format = arg_or(args, "--download-format", DEFAULT_DOWNLOAD_FORMAT);
    let out = match arg_value(args, "--out").filter(|o| !o.is_empty()) {
        Some(o) => o,
        None => format!("data/{EWDS_NETLOC}/glofas_{year}{month}{day}_{variable}.grib"),
    };
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let header = format!("PRIVATE-TOKEN: {token}");
    let body = request_body(
        &system_version,
        &product_type,
        &timespan,
        &variable,
        &year,
        &month,
        &day,
        &data_format,
        &download_format,
    );

    let (status, resp_body) = curl_body_status(&[
        "-sS",
        "--max-time",
        "120",
        "--connect-timeout",
        "15",
        "-X",
        "POST",
        EXECUTION_URL,
        "-H",
        "Content-Type: application/json",
        "-H",
        &header,
        "-d",
        &body,
        "-w",
        "\n@@STATUS@@%{http_code}",
    ])?;
    println!("http {status}");
    if status == 401 || status == 403 {
        return Err(format!(
            "execution returned HTTP {status} — the key does not open the data arm"
        ));
    }
    if !(200..300).contains(&status) {
        if let Some(root) = parse_json(&resp_body) {
            if let Some(detail) = scalar_field(&root, "detail") {
                println!("detail {detail}");
            } else if let Some(title) = scalar_field(&root, "title") {
                println!("title {title}");
            }
        }
        return Err(format!("execution returned HTTP {status} — no job stands"));
    }
    let job = parse_json(&resp_body)
        .and_then(|root| str_field(&root, "jobID"))
        .ok_or_else(|| "execution: the accepted response carries no jobID".to_string())?;
    println!("jobID {job}");

    let job_url = format!("{JOBS_BASE}/{job}");
    let mut last = "accepted".to_string();
    let mut done = false;
    for _ in 0..MAX_POLLS {
        std::thread::sleep(Duration::from_secs(POLL_SECS));
        let (poll_status, poll_body) = curl_body_status(&[
            "-sS",
            "--max-time",
            "60",
            "--connect-timeout",
            "15",
            "-H",
            &header,
            "-w",
            "\n@@STATUS@@%{http_code}",
            &job_url,
        ])?;
        if poll_status == 401 || poll_status == 403 {
            return Err(format!(
                "jobs returned HTTP {poll_status} — the key does not open the data arm"
            ));
        }
        last = json_string_field(&poll_body, "status")
            .ok_or_else(|| format!("job {job}: the poll carries no status field"))?;
        println!("status {last}");
        if last == "successful" {
            done = true;
            break;
        }
        if last == "failed" {
            break;
        }
    }
    if !done {
        return Err(format!(
            "job {job} reached {last} — the result stays undownloaded"
        ));
    }

    retrieve_and_save(&token, &job, &out, ci_mode)
}

fn retrieve_and_save(token: &str, job: &str, out: &str, ci_mode: bool) -> Result<(), String> {
    let header = format!("PRIVATE-TOKEN: {token}");
    let results_url = format!("{JOBS_BASE}/{job}/results");
    let (status, resp_body) = curl_body_status(&[
        "-sS",
        "--max-time",
        "120",
        "--connect-timeout",
        "15",
        "-H",
        &header,
        "-w",
        "\n@@STATUS@@%{http_code}",
        &results_url,
    ])?;
    if status == 401 || status == 403 {
        return Err(format!(
            "results returned HTTP {status} — the key does not open the data arm"
        ));
    }
    if !(200..300).contains(&status) {
        return Err(format!(
            "results returned HTTP {status} — the asset stays unnamed"
        ));
    }
    let href = json_string_field(&resp_body, "href")
        .ok_or_else(|| format!("results: the response carries no href — {resp_body}"))?;
    let declared_size = json_number_field(&resp_body, "file:size");
    download_href(&href, out, declared_size)?;
    if ci_mode && !upload_release(EWDS_NETLOC, out) {
        return Err(format!("{out} did not reach the CDN"));
    }
    Ok(())
}

fn download_href(href: &str, out: &str, declared_size: Option<u64>) -> Result<(), String> {
    if let Some(parent) = Path::new(out).parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("create {} returned void: {e}", parent.display()))?;
        }
    }
    let downloaded = Command::new("curl")
        .args([
            "-sSfL",
            "--max-time",
            "600",
            "--connect-timeout",
            "15",
            href,
            "-o",
            out,
        ])
        .output()
        .map_err(|e| format!("curl invocation void: {e}"))?;
    if !downloaded.status.success() {
        return Err(format!(
            "asset GET caught flux: curl status {:?} — {out} stays unwritten",
            downloaded.status.code()
        ));
    }
    let bytes = fs::read(out).map_err(|e| format!("read {out} returned void: {e}"))?;
    if bytes.is_empty() {
        return Err(format!("{out}: the asset carries no bytes"));
    }
    if let Some(expected) = declared_size {
        if bytes.len() as u64 != expected {
            return Err(format!(
                "{out}: {} bytes measured, {expected} declared by file:size — the asset stays untrusted",
                bytes.len()
            ));
        }
    }
    println!(
        "saved {out} bytes={} sha256={}",
        bytes.len(),
        sha256_hex(&bytes)
    );
    Ok(())
}

fn download_job(args: &[String]) -> Result<(), String> {
    let token = read_secret(KEY_NAME).ok_or_else(|| {
        format!("{KEY_NAME} absent from .secrets.local — the data arm stays unmeasured")
    })?;
    let job = arg_value(args, "--download")
        .filter(|j| !j.is_empty())
        .ok_or_else(|| "--download carries no jobID".to_string())?;
    let year = arg_or(args, "--year", DEFAULT_YEAR);
    let month = arg_or(args, "--month", DEFAULT_MONTH);
    let day = arg_or(args, "--day", DEFAULT_DAY);
    let variable = arg_or(args, "--variable", DEFAULT_VARIABLE);
    let out = match arg_value(args, "--out").filter(|o| !o.is_empty()) {
        Some(o) => o,
        None => format!("data/{EWDS_NETLOC}/glofas_{year}{month}{day}_{variable}.grib"),
    };
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    retrieve_and_save(&token, &job, &out, ci_mode)
}

struct GlofasCollection {
    id: String,
    url: String,
}

fn catalog_fetch(url: &str) -> Option<String> {
    fetch_raw_with(url, None, &[], RetryPolicy::All, TRANSFER_BOUND_S)
}

fn netloc_of(url: &str) -> Option<String> {
    let rest = url
        .strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"))?;
    let authority = rest.split('/').next().unwrap_or(rest);
    let host = authority.rsplit('@').next().unwrap_or(authority);
    let host = host.split(':').next().unwrap_or(host);
    if host.is_empty() {
        None
    } else {
        Some(host.to_string())
    }
}

fn str_field(value: &JsonVal, key: &str) -> Option<String> {
    match value {
        JsonVal::Obj(map) => match map.get(key) {
            Some(JsonVal::Str(s)) if !s.is_empty() => Some(s.clone()),
            _ => None,
        },
        _ => None,
    }
}

fn self_link(value: &JsonVal) -> Option<String> {
    let links = match value {
        JsonVal::Obj(map) => map.get("links")?,
        _ => return None,
    };
    match links {
        JsonVal::Arr(entries) => entries.iter().find_map(|entry| {
            if str_field(entry, "rel").as_deref() == Some("self") {
                str_field(entry, "href").filter(|href| href.starts_with("http"))
            } else {
                None
            }
        }),
        _ => None,
    }
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn parse_collections(body: &str, prefix: &str) -> Result<(Vec<GlofasCollection>, usize), String> {
    let root = parse_json(body).ok_or_else(|| "catalog: the body carries no JSON".to_string())?;
    let parts = match &root {
        JsonVal::Obj(map) => match map.get("collections") {
            Some(JsonVal::Arr(a)) => a,
            _ => return Err("catalog: the collections array is absent".to_string()),
        },
        _ => return Err("catalog: the top level is no object".to_string()),
    };
    let mut collections = Vec::new();
    let mut raw = 0usize;
    for part in parts {
        raw += 1;
        let id = match str_field(part, "id") {
            Some(i) => i,
            None => continue,
        };
        if !id.starts_with(prefix) {
            continue;
        }
        let url = match self_link(part) {
            Some(u) => u,
            None => continue,
        };
        collections.push(GlofasCollection { id, url });
    }
    Ok((collections, raw))
}

fn manifest_lines(collections: &[GlofasCollection]) -> Vec<String> {
    collections
        .iter()
        .map(|c| format!("shard {}", c.url))
        .collect()
}

fn run(args: &[String]) -> Result<(), String> {
    if args.iter().any(|a| a == "--help") {
        eprintln!(
            "usage: glofas_ewds_compiler [--url <catalog>] [--prefix <id-prefix>] [--out <manifest>] [--limit <n>] [--ci-mode] [--probe] [--fetch [--year <y>] [--month <m>] [--day <d>] [--variable <v>] [--timespan <t>] [--system-version <s>] [--product-type <p>] [--data-format <f>] [--download-format <df>] [--out <file>]] [--download <jobID> [--year <y>] [--month <m>] [--day <d>] [--variable <v>] [--out <file>]]  (shard `<url>` lines). The EWDS catalogue ({CATALOG_URL}) carries one Collection per dataset; the {DEFAULT_PREFIX}* family is emitted as `shard <collection-url>`. The catalogue is public; `--probe` POSTs the minimal valid request to {EXECUTION_URL} behind PRIVATE-TOKEN: {KEY_NAME} (read from .secrets.local; measured 2026-10-11: 401 without the key; the STAC items endpoint answers 501) and prints only the HTTP status and any jobID/detail. `--fetch` submits the same process through {EXECUTION_URL}, polls {JOBS_BASE}/<jobID> every {POLL_SECS}s (at most {MAX_POLLS} times) until `status:\"successful\"`, reads the href+file:size from {JOBS_BASE}/<jobID>/results and downloads that asset to --out (default data/{EWDS_NETLOC}/glofas_<year><month><day>_<variable>.grib). `--download <jobID>` skips submit/poll and re-fetches an existing job's result the same way. entry bound {MAX_ENTRIES}"
        );
        return Ok(());
    }
    if args.iter().any(|a| a == "--probe") {
        return probe();
    }
    if args.iter().any(|a| a == "--fetch") {
        return fetch(args);
    }
    if args.iter().any(|a| a == "--download") {
        return download_job(args);
    }
    let root = match arg_value(args, "--url").filter(|u| !u.is_empty()) {
        Some(u) => u,
        None => CATALOG_URL.to_string(),
    };
    let prefix = match arg_value(args, "--prefix").filter(|p| !p.is_empty()) {
        Some(p) => p,
        None => DEFAULT_PREFIX.to_string(),
    };
    let netloc = netloc_of(&root).ok_or_else(|| format!("--url {root}: no host readable"))?;
    let out = match arg_value(args, "--out").filter(|o| !o.is_empty()) {
        Some(o) => o,
        None => format!("data/{netloc}/{MANIFEST_NAME}"),
    };
    let out_name = match Path::new(&out).file_name().and_then(|n| n.to_str()) {
        Some(n) => n.to_string(),
        None => MANIFEST_NAME.to_string(),
    };
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let limit = match arg_value(args, "--limit").and_then(|v| v.parse::<usize>().ok()) {
        Some(n) if n > 0 => n,
        _ => MAX_ENTRIES,
    };

    let body = catalog_fetch(&root)
        .ok_or_else(|| format!("catalog {root} returned void — the walk stays incomplete"))?;
    let (mut collections, raw) = parse_collections(&body, &prefix)?;
    if raw == 0 {
        return Err("catalog: no collection stands in the response".to_string());
    }
    collections.truncate(limit);
    collections.sort_by(|a, b| a.id.cmp(&b.id));
    collections.dedup_by(|a, b| a.id == b.id || a.url == b.url);
    if collections.is_empty() {
        return Err(format!(
            "catalog: {raw} collections carry no '{prefix}' id+url — nothing manifestiert (0 honored)"
        ));
    }
    let lines = manifest_lines(&collections);

    let mut text = String::with_capacity(lines.len() * 128);
    for line in &lines {
        text.push_str(line);
        text.push('\n');
    }
    if let Some(parent) = Path::new(&out).parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("create {} returned void: {e}", parent.display()))?;
        }
    }
    fs::write(&out, &text).map_err(|e| format!("write {out} returned void: {e}"))?;
    let back = fs::read_to_string(&out).map_err(|_| format!("{out} re-read returned void"))?;
    if back.lines().count() != lines.len() {
        return Err(format!(
            "{out} roundtrip lists {} of {} lines — the manifest stays unverified",
            back.lines().count(),
            lines.len()
        ));
    }
    let first = collections.first().map(|c| c.id.as_str()).unwrap_or("?");
    let last = collections.last().map(|c| c.id.as_str()).unwrap_or("?");
    eprintln!(
        "glofas_ewds: {raw} collections, {} with '{prefix}' id+url, span {first}..{last} -> {out}",
        collections.len()
    );

    println!("url {CDN_BASE}/{netloc}/{out_name}");
    println!("origin {CATALOG_URL}");
    println!("compiler {COMPILER}");
    println!("format coverage_manifest");

    if ci_mode && !upload_release(&netloc, &out) {
        return Err(format!("{out} did not reach the CDN"));
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("glofas_ewds_compiler: {msg}");
        std::process::exit(2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CATALOG: &str = r#"{"collections":[
{"id":"cems-glofas-historical","title":"Historical","links":[{"rel":"self","href":"https://ewds.climate.copernicus.eu/api/catalogue/v1/collections/cems-glofas-historical"},{"rel":"parent","href":"https://ewds.climate.copernicus.eu/api/catalogue/v1/"}]},
{"id":"cems-glofas-forecast","title":"Forecast","links":[{"rel":"self","href":"https://ewds.climate.copernicus.eu/api/catalogue/v1/collections/cems-glofas-forecast"}]},
{"id":"efas-historical","title":"EFAS","links":[{"rel":"self","href":"https://ewds.climate.copernicus.eu/api/catalogue/v1/collections/efas-historical"}]},
{"id":"cems-glofas-seasonal","title":"Seasonal","links":[{"rel":"parent","href":"https://ewds.climate.copernicus.eu/api/catalogue/v1/"}]}
],"numberMatched":4}"#;

    #[test]
    fn catalog_keeps_the_prefixed_collections_and_skips_one_without_a_self_link() {
        let (collections, raw) = parse_collections(CATALOG, "cems-glofas").expect("catalog parses");
        assert_eq!(raw, 4);
        assert_eq!(collections.len(), 2);
        assert_eq!(collections[0].id, "cems-glofas-historical");
        assert_eq!(collections[1].id, "cems-glofas-forecast");
        assert_eq!(
            manifest_lines(&collections),
            vec![
                "shard https://ewds.climate.copernicus.eu/api/catalogue/v1/collections/cems-glofas-historical".to_string(),
                "shard https://ewds.climate.copernicus.eu/api/catalogue/v1/collections/cems-glofas-forecast".to_string(),
            ]
        );
    }

    #[test]
    fn a_page_without_collections_names_itself() {
        let err = parse_collections("{}", "cems-glofas").unwrap_err();
        assert!(err.contains("collections"));
    }

    #[test]
    fn netloc_reads_the_host_from_the_catalog_url() {
        assert_eq!(
            netloc_of(CATALOG_URL).as_deref(),
            Some("ewds.climate.copernicus.eu")
        );
        assert_eq!(netloc_of("ftp://x/y"), None);
    }

    const RESULTS: &str = r#"{"asset":{"value":{"type":"application/x-grib","href":"https://object-store.os-api.cci2.ecmwf.int:443/cci2-prod-cache-2/2026-10-07/a9683b649255b37d598e7bcd5dc18a72.grib","file:checksum":"f3867364db4b6aef99990efb51895c82","file:size":21366009,"file:local_path":"s3://cci2-prod-cache-2/2026-10-07/a9683b649255b37d598e7bcd5dc18a72.grib"}}}"#;

    #[test]
    fn results_carry_the_asset_href_and_declared_size() {
        assert_eq!(
            json_string_field(RESULTS, "href").as_deref(),
            Some(
                "https://object-store.os-api.cci2.ecmwf.int:443/cci2-prod-cache-2/2026-10-07/a9683b649255b37d598e7bcd5dc18a72.grib"
            )
        );
        assert_eq!(json_number_field(RESULTS, "file:size"), Some(21366009));
        assert_eq!(
            json_string_field(RESULTS, "file:checksum").as_deref(),
            Some("f3867364db4b6aef99990efb51895c82")
        );
    }

    #[test]
    fn a_results_blob_without_href_names_its_absence() {
        assert_eq!(json_string_field("{\"asset\":{}}", "href"), None);
        assert_eq!(json_number_field("{\"asset\":{}}", "file:size"), None);
    }
}
