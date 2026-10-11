use omegaflow::archivar::json::{JsonVal, parse_json};
use omegaflow::archivar::{RetryPolicy, TRANSFER_BOUND_S, fetch_raw_with};
use omegaflow::cdn::{CDN_BASE, upload_release};
use std::env;
use std::fs;
use std::path::Path;

const COMPILER: &str = "tools/harvest/src/bin/glofas_ewds_compiler.rs";
const CATALOG_URL: &str = "https://ewds.climate.copernicus.eu/api/catalogue/v1/collections";
const DEFAULT_PREFIX: &str = "cems-glofas";
const MANIFEST_NAME: &str = "glofas_ewds_coverage.manifest";
const MAX_ENTRIES: usize = 1_000;

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
            "usage: glofas_ewds_compiler [--url <catalog>] [--prefix <id-prefix>] [--out <manifest>] [--limit <n>] [--ci-mode]  (shard `<url>` lines). The EWDS catalogue ({CATALOG_URL}) carries one Collection per dataset; the {DEFAULT_PREFIX}* family is emitted as `shard <collection-url>`. The catalogue is public; the retrieve execution endpoint (.../processes/<id>/execution) carries the GloFAS data behind PRIVATE-TOKEN: CDS_API_KEY (measured 2026-10-11: 401 without the key; the STAC items endpoint answers 501). entry bound {MAX_ENTRIES}"
        );
        return Ok(());
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
