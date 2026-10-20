use omegaflow::archivar::json::{JsonVal, parse_json};
use omegaflow::archivar::{RetryPolicy, TRANSFER_BOUND_S, fetch_raw_with};
use omegaflow::cdn::{CDN_BASE, upload_release};
use std::env;
use std::fs;
use std::path::Path;

const COMPILER: &str = "tools/harvest/src/bin/nist_ncnr_nse_compiler.rs";
const NSE_BASE: &str = "https://ncnr.nist.gov/pub/ncnrdata/nse/";
const CATALOG_URL: &str = "https://ncnr.nist.gov/pub/ncnrdata/nse/?format=jsonld";
const MANIFEST_NAME: &str = "nist_ncnr_nse_coverage.manifest";
const MAX_ENTRIES: usize = 10_000;

#[derive(Debug)]
struct NseSet {
    name: String,
    url: String,
    date_modified: Option<String>,
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

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn parse_sets(body: &str) -> Result<(Vec<NseSet>, usize), String> {
    let root = parse_json(body).ok_or_else(|| "catalog: the body carries no JSON".to_string())?;
    let parts = match &root {
        JsonVal::Obj(map) => match map.get("hasPart") {
            Some(JsonVal::Arr(a)) => a,
            _ => return Err("catalog: the hasPart array is absent".to_string()),
        },
        _ => return Err("catalog: the top level is no object".to_string()),
    };
    let mut sets = Vec::new();
    let mut raw = 0usize;
    for part in parts {
        raw += 1;
        let name = match str_field(part, "name") {
            Some(n) => n,
            None => continue,
        };
        let url = match str_field(part, "url").filter(|u| u.starts_with("http")) {
            Some(u) => u,
            None => continue,
        };
        let date_modified = str_field(part, "dateModified");
        sets.push(NseSet {
            name,
            url,
            date_modified,
        });
    }
    Ok((sets, raw))
}

fn manifest_lines(sets: &[NseSet]) -> Vec<String> {
    sets.iter().map(|s| format!("shard {}", s.url)).collect()
}

fn run(args: &[String]) -> Result<(), String> {
    if args.iter().any(|a| a == "--help") {
        eprintln!(
            "usage: nist_ncnr_nse_compiler [--url <catalog>] [--out <manifest>] [--limit <n>] [--ci-mode]  (shard `<url>` lines). The NIST NCNR NSE JSON-LD directory ({CATALOG_URL}) carries one `hasPart` set per monthly directory (name, url, dateModified); each set is emitted as `shard <url>`. entry bound {MAX_ENTRIES}"
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

    let body = catalog_fetch(&root)
        .ok_or_else(|| format!("catalog {root} returned void — the walk stays incomplete"))?;
    let (mut sets, raw) = parse_sets(&body)?;
    if raw == 0 {
        return Err("catalog: no set stands in the response".to_string());
    }
    sets.truncate(limit);
    sets.sort_by(|a, b| a.name.cmp(&b.name));
    sets.dedup_by(|a, b| a.name == b.name || a.url == b.url);
    if sets.is_empty() {
        return Err(format!(
            "catalog: {raw} entries carry no name+url — nothing manifestiert (0 honored)"
        ));
    }
    let lines = manifest_lines(&sets);

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
    let first = sets.first().map(|s| s.name.as_str()).unwrap_or("?");
    let last = sets.last().map(|s| s.name.as_str()).unwrap_or("?");
    let newest = sets
        .iter()
        .filter_map(|s| s.date_modified.as_deref())
        .max()
        .unwrap_or("absent");
    eprintln!(
        "nist_ncnr_nse: {raw} sets, {} with name+url, span {first}..{last}, newest {newest} -> {out}",
        sets.len()
    );

    println!("url {CDN_BASE}/{netloc}/{out_name}");
    println!("origin {NSE_BASE}");
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
        eprintln!("nist_ncnr_nse_compiler: {msg}");
        std::process::exit(2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CATALOG: &str = r#"{"@context":"https://schema.org/","@type":"Dataset","hasPart":[
{"name":"202608","url":"https://ncnr.nist.gov/pub/ncnrdata/nse/202608/?format=jsonld","dateModified":"2026-08-31T12:00:00Z"},
{"name":"202102","url":"https://ncnr.nist.gov/pub/ncnrdata/nse/202102/?format=jsonld","dateModified":"2021-02-28T12:00:00Z"},
{"name":"202009","url":"https://ncnr.nist.gov/pub/ncnrdata/nse/202009/?format=jsonld","dateModified":"2020-09-30T12:00:00Z"},
{"name":"202101","url":"ftp://ncnr.nist.gov/pub/ncnrdata/nse/202101/?format=jsonld"}
]}"#;

    #[test]
    fn catalog_keeps_the_declared_set_and_skips_one_without_an_http_url() {
        let (sets, raw) = parse_sets(CATALOG).expect("catalog parses");
        assert_eq!(raw, 4);
        assert_eq!(sets.len(), 3);
        assert_eq!(sets[0].name, "202608");
        assert_eq!(
            manifest_lines(&sets),
            vec![
                "shard https://ncnr.nist.gov/pub/ncnrdata/nse/202608/?format=jsonld".to_string(),
                "shard https://ncnr.nist.gov/pub/ncnrdata/nse/202102/?format=jsonld".to_string(),
                "shard https://ncnr.nist.gov/pub/ncnrdata/nse/202009/?format=jsonld".to_string(),
            ]
        );
    }

    #[test]
    fn a_page_without_has_part_names_itself() {
        let err = parse_sets(r#"{"@type":"Dataset"}"#).unwrap_err();
        assert!(err.contains("hasPart"));
    }

    #[test]
    fn netloc_reads_the_host_from_the_catalog_url() {
        assert_eq!(netloc_of(CATALOG_URL).as_deref(), Some("ncnr.nist.gov"));
        assert_eq!(netloc_of("ftp://x/y"), None);
    }
}
