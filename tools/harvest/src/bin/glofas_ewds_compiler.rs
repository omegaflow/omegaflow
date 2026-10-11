use omegaflow::archivar::json::{JsonVal, parse_json};
use omegaflow::archivar::{RetryPolicy, TRANSFER_BOUND_S, fetch_raw_with};
use omegaflow::cdn::{CDN_BASE, upload_release};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const COMPILER: &str = "tools/harvest/src/bin/glofas_ewds_compiler.rs";
const CATALOG_URL: &str = "https://ewds.climate.copernicus.eu/api/catalogue/v1/collections";
const DEFAULT_PREFIX: &str = "cems-glofas";
const MANIFEST_NAME: &str = "glofas_ewds_coverage.manifest";
const MAX_ENTRIES: usize = 1_000;
const KEY_NAME: &str = "EWDS_API_TOKEN";
const EXECUTION_URL: &str =
    "https://ewds.climate.copernicus.eu/api/retrieve/v1/processes/cems-glofas-historical/execution";
const PROBE_BODY: &str = r#"{"inputs":{"system_version":"version_4_0","hydrological_model":"lisflood","product_type":"consolidated","timespan":"time_mean","variable":"average_river_discharge_in_the_last_24_hours","year":"2020","month":"01","day":"01","data_format":"grib2","download_format":"unarchived"}}"#;

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
            "usage: glofas_ewds_compiler [--url <catalog>] [--prefix <id-prefix>] [--out <manifest>] [--limit <n>] [--ci-mode] [--probe]  (shard `<url>` lines). The EWDS catalogue ({CATALOG_URL}) carries one Collection per dataset; the {DEFAULT_PREFIX}* family is emitted as `shard <collection-url>`. The catalogue is public; `--probe` POSTs the minimal valid request to {EXECUTION_URL} behind PRIVATE-TOKEN: {KEY_NAME} (read from .secrets.local; measured 2026-10-11: 401 without the key; the STAC items endpoint answers 501) and prints only the HTTP status and any jobID/detail. entry bound {MAX_ENTRIES}"
        );
        return Ok(());
    }
    if args.iter().any(|a| a == "--probe") {
        return probe();
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
}
