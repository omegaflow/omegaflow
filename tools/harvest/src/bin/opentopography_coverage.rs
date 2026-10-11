use omegaflow::archivar::json::{JsonVal, parse_json};
use omegaflow::archivar::{RetryPolicy, TRANSFER_BOUND_S, fetch_raw_with};
use omegaflow::cdn::{CDN_BASE, upload_release};
use std::env;
use std::fs;
use std::path::Path;

const COMPILER: &str = "tools/harvest/src/bin/opentopography_coverage.rs";
const CATALOG_URL: &str = "https://portal.opentopography.org/API/otCatalog?outputFormat=json";
const MANIFEST_NAME: &str = "opentopography_coverage.manifest";
const MAX_ENTRIES: usize = 100_000;
const API_KEY_ENV: &str = "OPENTOPOGRAPHY_API_KEY";

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

fn dataset_of(value: &JsonVal) -> Option<&JsonVal> {
    match value {
        JsonVal::Obj(map) => map.get("Dataset"),
        _ => None,
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

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn keyed_href(href: &str, key: Option<&str>) -> String {
    let Some(key) = key.filter(|k| !k.is_empty()) else {
        return href.to_string();
    };
    if !href.contains("portal.opentopography.org/API/") || href.contains("API_Key=") {
        return href.to_string();
    }
    let sep = if href.contains('?') { '&' } else { '?' };
    format!("{href}{sep}API_Key={key}")
}

fn parse_catalog(body: &str) -> Result<(Vec<String>, usize), String> {
    let root = parse_json(body).ok_or_else(|| "catalog: the body carries no JSON".to_string())?;
    let datasets = match &root {
        JsonVal::Obj(map) => match map.get("Datasets") {
            Some(JsonVal::Arr(a)) => a,
            _ => return Err("catalog: the Datasets array is absent".to_string()),
        },
        _ => return Err("catalog: the top level is no object".to_string()),
    };
    let mut hrefs = Vec::new();
    let mut raw = 0usize;
    for value in datasets {
        let Some(dataset) = dataset_of(value) else {
            continue;
        };
        raw += 1;
        if let Some(href) = str_field(dataset, "url").filter(|u| u.starts_with("http")) {
            hrefs.push(href);
        }
    }
    Ok((hrefs, raw))
}

fn manifest_lines(hrefs: &[String], key: Option<&str>) -> Vec<String> {
    hrefs
        .iter()
        .map(|href| format!("shard {}", keyed_href(href, key)))
        .collect()
}

fn run(args: &[String]) -> Result<(), String> {
    if args.iter().any(|a| a == "--help") {
        eprintln!(
            "usage: opentopography_coverage [--url <catalog>] [--out <manifest>] [--limit <n>] [--ci-mode]  (url-only shard lines). The OpenTopography catalog (default {CATALOG_URL}) is keyless and carries one Dataset per entry with only a declared `url` (DOI/landing href) — no per-tile download URL. Each entry is emitted as `shard <href>`. {API_KEY_ENV} is read; it is appended as `API_Key` only on an href that is itself an OpenTopography API endpoint, never on the DOI hrefs. entry bound {MAX_ENTRIES}"
        );
        return Ok(());
    }
    let root = match arg_value(args, "--url").filter(|u| !u.is_empty()) {
        Some(u) => u,
        None => CATALOG_URL.to_string(),
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
    let key = env::var(API_KEY_ENV).ok().filter(|k| !k.is_empty());
    eprintln!(
        "opentopography: API key {} for keyed downloads",
        if key.is_some() { "present" } else { "absent" }
    );

    let body = catalog_fetch(&root)
        .ok_or_else(|| format!("catalog {root} returned void — the walk stays incomplete"))?;
    let (mut hrefs, raw) = parse_catalog(&body)?;
    if raw == 0 {
        return Err("catalog: no Dataset entry stands in the response".to_string());
    }
    hrefs.truncate(limit);
    hrefs.sort();
    hrefs.dedup();
    if hrefs.is_empty() {
        return Err(format!(
            "catalog: {raw} datasets carry no resolvable href — nothing manifestiert (0 honored)"
        ));
    }
    let lines = manifest_lines(&hrefs, key.as_deref());

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
    eprintln!(
        "opentopography: {raw} datasets, {} shard hrefs -> {out} (the catalog carries no per-tile download URL — each shard is the dataset's DOI/landing href)",
        hrefs.len()
    );

    println!("url {CDN_BASE}/{netloc}/{out_name}");
    println!("origin {root}");
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
        eprintln!("opentopography_coverage: {msg}");
        std::process::exit(2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CATALOG: &str = r#"{"Datasets":[
{"Dataset":{"name":"A","identifier":{"@type":"PropertyValue","propertyID":"opentopoID","value":"OTLAS.1.2.3"},"url":"https://doi.org/10.5069/AAA"}},
{"Dataset":{"name":"B","identifier":{"@type":"PropertyValue","propertyID":"opentopoID","value":"OTSDEM.1.2.3"}}},
{"Dataset":{"name":"C","url":"ftp://portal.opentopography.org/not-http"}}
]}"#;

    #[test]
    fn catalog_keeps_the_declared_href_and_skips_an_entry_without_one() {
        let (hrefs, raw) = parse_catalog(CATALOG).expect("catalog parses");
        assert_eq!(raw, 3);
        assert_eq!(hrefs, vec!["https://doi.org/10.5069/AAA".to_string()]);
        assert_eq!(
            manifest_lines(&hrefs, None),
            vec!["shard https://doi.org/10.5069/AAA".to_string()]
        );
    }

    #[test]
    fn the_key_lands_only_on_an_opentopography_api_href_never_on_a_doi() {
        let api = "https://portal.opentopography.org/API/globaldem?demtype=SRTMGL1";
        assert_eq!(
            keyed_href(api, Some("k1")),
            "https://portal.opentopography.org/API/globaldem?demtype=SRTMGL1&API_Key=k1"
        );
        assert_eq!(
            keyed_href("https://doi.org/10.5069/AAA", Some("k1")),
            "https://doi.org/10.5069/AAA"
        );
        assert_eq!(keyed_href(api, None), api);
    }

    #[test]
    fn netloc_reads_the_host_from_the_catalog_url() {
        assert_eq!(
            netloc_of(CATALOG_URL).as_deref(),
            Some("portal.opentopography.org")
        );
        assert_eq!(netloc_of("ftp://x/y"), None);
    }
}
