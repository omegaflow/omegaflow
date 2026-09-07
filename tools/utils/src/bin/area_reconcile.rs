use omegaflow::archivar::{jpath_val, jstr, load_sources_from, parse_json, JsonVal};
use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const CDN_REPO: &str = "omegaflow/sources";
const CDN_PREFIX: &str = "https://github.com/omegaflow/sources/releases/download/";
const COMPILED_SUFFIXES: &[&str] = &[
    "bin", "json", "gbco", "sky1", "s2e1", "pao1", "amn1", "be19", "vlde",
];

struct CdnAsset {
    tag: String,
    name: String,
}

struct LocalAsset {
    rel: String,
    name: String,
}

struct Report {
    releases: usize,
    asset_rows: Vec<CdnAsset>,
    cdn_names: BTreeSet<String>,
    data_files: usize,
    data_dirs: BTreeSet<String>,
    compiled_files: Vec<LocalAsset>,
    compiled_names: BTreeSet<String>,
    cache_hosts: usize,
    cache_files: usize,
    registered: Vec<(String, String)>,
    registered_names: BTreeSet<String>,
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut root: Option<String> = None;
    let mut sources_rel = String::from("phi/sources.φ");
    let mut json_mode = false;
    let mut full = false;
    let mut limit = 30usize;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--root" => {
                i += 1;
                root = match args.get(i) {
                    Some(r) => Some(r.clone()),
                    None => usage(),
                };
            }
            "--sources" => {
                i += 1;
                sources_rel = match args.get(i) {
                    Some(s) => s.clone(),
                    None => usage(),
                };
            }
            "--json" => json_mode = true,
            "--full" => full = true,
            "--limit" => {
                i += 1;
                limit = match args.get(i).and_then(|s| s.parse().ok()) {
                    Some(n) => n,
                    None => usage(),
                };
            }
            "--help" | "-h" => usage(),
            _ => usage(),
        }
        i += 1;
    }

    let root_path = match root {
        Some(r) => PathBuf::from(r),
        None => match find_repo_root() {
            Some(root) => root,
            None => {
                eprintln!("area_reconcile: phi/sources.φ absent here and in the parents");
                std::process::exit(2);
            }
        },
    };

    let sources_path = if sources_rel.starts_with('/') {
        PathBuf::from(&sources_rel)
    } else {
        root_path.join(&sources_rel)
    };
    let sources_text = match fs::read_to_string(&sources_path) {
        Ok(text) => text,
        Err(e) => {
            eprintln!(
                "area_reconcile: {} unreadable: {}",
                sources_path.display(),
                e
            );
            std::process::exit(2);
        }
    };

    let report = build_report(&root_path, &sources_text);

    if json_mode {
        print_json(&report);
    } else {
        print_plain(&report, full, limit);
    }
    eprintln!(
        "area_reconcile: releases {} | data_files {} | compiled_files {} | cache_files {} | registered {} | a {} b {} c {} d {}",
        report.releases,
        report.data_files,
        report.compiled_files.len(),
        report.cache_files,
        report.registered.len(),
        local_unmanifested(&report).len(),
        registered_unmanifested(&report).len(),
        orphaned_cdn(&report).len(),
        cdn_only(&report).len(),
    );
}

fn usage() -> ! {
    eprintln!(
        "usage: area_reconcile [--root <dir>] [--sources <path>] [--json] [--full] [--limit <n>]"
    );
    eprintln!("       reconciles the four asset areas: CDN (omegaflow/sources), data/, cache/, phi/sources.φ");
    eprintln!("       --json emits the whole reconciliation as one JSON object");
    eprintln!(
        "       --full lists every name of the large classes; --limit caps the default lists"
    );
    std::process::exit(2);
}

fn find_repo_root() -> Option<PathBuf> {
    if let Ok(env_root) = env::var("OMEGAFLOW_REPO") {
        if !env_root.is_empty() {
            let root = PathBuf::from(env_root);
            if root.join("phi").join("sources.φ").is_file() {
                return Some(root);
            }
            return None;
        }
    }
    let mut dir = env::current_dir().ok()?;
    loop {
        if dir.join("phi").join("sources.φ").is_file() {
            return Some(dir);
        }
        match dir.parent() {
            Some(parent) => dir = parent.to_path_buf(),
            None => return None,
        }
    }
}

fn build_report(root: &Path, sources_text: &str) -> Report {
    let (data_files, data_dirs, compiled_files) = scan_data(root);
    let (cache_hosts, cache_files) = scan_cache(root);

    let sources = load_sources_from(sources_text);
    let mut registered: Vec<(String, String)> = Vec::new();
    let mut registered_names: BTreeSet<String> = BTreeSet::new();
    for s in &sources {
        if let Some((tag, asset)) = cdn_line_parts(&s.url) {
            if registered_names.insert(asset.clone()) {
                registered.push((tag, asset));
            }
        }
    }
    registered.sort();

    let (releases, asset_rows) = fetch_cdn();
    let cdn_names: BTreeSet<String> = asset_rows.iter().map(|a| a.name.clone()).collect();

    let compiled_names: BTreeSet<String> = compiled_files.iter().map(|a| a.name.clone()).collect();

    Report {
        releases,
        asset_rows,
        cdn_names,
        data_files,
        data_dirs,
        compiled_files,
        compiled_names,
        cache_hosts,
        cache_files,
        registered,
        registered_names,
    }
}

fn scan_data(root: &Path) -> (usize, BTreeSet<String>, Vec<LocalAsset>) {
    let data_root = root.join("data");
    let mut files = 0usize;
    let mut dirs: BTreeSet<String> = BTreeSet::new();
    let mut compiled: Vec<LocalAsset> = Vec::new();
    let mut stack: Vec<PathBuf> = vec![data_root.clone()];
    while let Some(dir) = stack.pop() {
        let entries = match fs::read_dir(&dir) {
            Ok(e) => e,
            Err(_) => continue,
        };
        let mut children: Vec<PathBuf> = Vec::new();
        for entry in entries.flatten() {
            children.push(entry.path());
        }
        children.sort();
        for path in children {
            let name = match path.file_name() {
                Some(n) => n.to_string_lossy().into_owned(),
                None => continue,
            };
            if name.starts_with('.') {
                continue;
            }
            if path.is_dir() {
                stack.push(path);
            } else {
                let rel = match path.strip_prefix(&data_root) {
                    Ok(r) => r.to_string_lossy().into_owned(),
                    Err(_) => continue,
                };
                if let Some(netloc) = rel.split('/').next() {
                    dirs.insert(netloc.to_string());
                }
                files += 1;
                if is_compiled_name(&name) {
                    compiled.push(LocalAsset { rel, name });
                }
            }
        }
    }
    compiled.sort_by(|a, b| a.rel.cmp(&b.rel));
    (files, dirs, compiled)
}

fn scan_cache(root: &Path) -> (usize, usize) {
    let cache_root = root.join("cache");
    let mut hosts = 0usize;
    let mut files = 0usize;
    let entries = match fs::read_dir(&cache_root) {
        Ok(e) => e,
        Err(_) => return (0, 0),
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let name = match path.file_name() {
            Some(n) => n.to_string_lossy().into_owned(),
            None => continue,
        };
        if name.starts_with('.') {
            continue;
        }
        if path.is_dir() {
            hosts += 1;
            files += walk_file_count(&path);
        }
    }
    (hosts, files)
}

fn walk_file_count(dir: &Path) -> usize {
    let mut count = 0usize;
    let mut stack: Vec<PathBuf> = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        let entries = match fs::read_dir(&d) {
            Ok(e) => e,
            Err(_) => continue,
        };
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_dir() {
                stack.push(p);
            } else {
                count += 1;
            }
        }
    }
    count
}

fn is_compiled_name(name: &str) -> bool {
    let ext = match name.rsplit_once('.') {
        Some((_, e)) => e.to_ascii_lowercase(),
        None => return false,
    };
    COMPILED_SUFFIXES.contains(&ext.as_str())
}

fn cdn_line_parts(url: &str) -> Option<(String, String)> {
    let rest = url.strip_prefix(CDN_PREFIX)?;
    let (tag, asset) = rest.split_once('/')?;
    if tag.is_empty() || asset.is_empty() || asset.contains('/') {
        return None;
    }
    Some((tag.to_string(), asset.to_string()))
}

fn fetch_cdn() -> (usize, Vec<CdnAsset>) {
    let body = gh_api_releases().or_else(curl_api_releases);
    let body = match body {
        Some(b) => b,
        None => {
            eprintln!("area_reconcile: CDN release list void (gh api and curl both failed)");
            std::process::exit(1);
        }
    };
    let Some(JsonVal::Arr(items)) = parse_json(&body) else {
        eprintln!("area_reconcile: CDN release list is not a JSON array");
        std::process::exit(1);
    };
    let mut releases = 0usize;
    let mut assets: Vec<CdnAsset> = Vec::new();
    for release in items {
        let Some(JsonVal::Arr(release_assets)) = jpath_val(&release, "assets") else {
            continue;
        };
        if release_assets.is_empty() {
            continue;
        }
        let tag = match jstr(&release, "tag_name") {
            Some(t) if !t.is_empty() => t,
            _ => continue,
        };
        releases += 1;
        for a in release_assets {
            let name = match jstr(a, "name") {
                Some(n) if !n.is_empty() => n,
                _ => continue,
            };
            assets.push(CdnAsset {
                tag: tag.clone(),
                name,
            });
        }
    }
    assets.sort_by(|a, b| a.name.cmp(&b.name).then_with(|| a.tag.cmp(&b.tag)));
    (releases, assets)
}

fn gh_api_releases() -> Option<String> {
    let out = Command::new("gh")
        .arg("api")
        .arg("--paginate")
        .arg(format!("repos/{}/releases?per_page=100", CDN_REPO))
        .output()
        .ok()?;
    if out.status.success() {
        Some(String::from_utf8_lossy(&out.stdout).into_owned())
    } else {
        None
    }
}

fn curl_api_releases() -> Option<String> {
    let token = env::var("GITHUB_TOKEN")
        .ok()
        .filter(|t| !t.is_empty())
        .or_else(|| env::var("GH_TOKEN").ok().filter(|t| !t.is_empty()))?;
    let mut chunks: Vec<String> = Vec::new();
    let mut page = 1usize;
    loop {
        let url = format!(
            "https://api.github.com/repos/{}/releases?per_page=100&page={}",
            CDN_REPO, page
        );
        let out = Command::new("curl")
            .arg("-sS")
            .arg("-H")
            .arg(format!("Authorization: Bearer {}", token))
            .arg(url)
            .output()
            .ok()?;
        if !out.status.success() {
            return None;
        }
        let text = String::from_utf8_lossy(&out.stdout).into_owned();
        let trimmed = text.trim();
        if trimmed == "[]" {
            break;
        }
        let inner = trimmed
            .strip_prefix('[')
            .and_then(|s| s.strip_suffix(']'))?;
        chunks.push(inner.trim().to_string());
        page += 1;
    }
    Some(format!("[{}]", chunks.join(",")))
}

fn local_unmanifested(report: &Report) -> Vec<&LocalAsset> {
    report
        .compiled_files
        .iter()
        .filter(|a| !report.cdn_names.contains(&a.name))
        .collect()
}

fn registered_unmanifested(report: &Report) -> Vec<(String, String)> {
    report
        .registered
        .iter()
        .filter(|(_, asset)| !report.cdn_names.contains(asset))
        .cloned()
        .collect()
}

fn orphaned_cdn(report: &Report) -> Vec<String> {
    report
        .cdn_names
        .iter()
        .filter(|name| !report.registered_names.contains(*name))
        .cloned()
        .collect()
}

fn cdn_only(report: &Report) -> Vec<String> {
    report
        .cdn_names
        .iter()
        .filter(|name| !report.compiled_names.contains(*name))
        .cloned()
        .collect()
}

fn tag_histogram(report: &Report, names: &BTreeSet<String>) -> Vec<(String, usize)> {
    let mut counts: BTreeMap<String, usize> = BTreeMap::new();
    for a in &report.asset_rows {
        if names.contains(&a.name) {
            *counts.entry(a.tag.clone()).or_insert(0) += 1;
        }
    }
    let mut list: Vec<(String, usize)> = counts.into_iter().collect();
    list.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    list
}

fn print_plain(report: &Report, full: bool, limit: usize) {
    println!("area_reconcile — four-area asset reconciliation");
    println!();
    println!("areas");
    println!(
        "  1 CDN (github.com/{}) — {} releases, {} asset rows, {} distinct names",
        CDN_REPO,
        report.releases,
        report.asset_rows.len(),
        report.cdn_names.len()
    );
    println!(
        "  2 data/ — {} files, {} netloc dirs, {} compiled assets ({} distinct names)",
        report.data_files,
        report.data_dirs.len(),
        report.compiled_files.len(),
        report.compiled_names.len()
    );
    println!(
        "  3 cache/ — {} hosts, {} raw fetches (context only, never a manifestable asset)",
        report.cache_hosts, report.cache_files
    );
    println!(
        "  4 phi/sources.φ — {} registered CDN url-lines, {} distinct assets",
        report.registered.len(),
        report.registered_names.len()
    );

    let a = local_unmanifested(report);
    let b = registered_unmanifested(report);
    let c = orphaned_cdn(report);
    let d = cdn_only(report);

    println!();
    println!("checks");
    println!(
        "  (a) {} local-unmanifested — data/ compiled asset with no CDN asset of the same name (register debt)",
        a.len()
    );
    for asset in &a {
        println!("      data/{}", asset.rel);
    }
    println!();
    println!(
        "  (b) {} registered-but-unmanifested — sources.φ CDN url whose asset is absent on the CDN (register debt)",
        b.len()
    );
    for (tag, asset) in &b {
        println!("      {}/{}", tag, asset);
    }

    let c: BTreeSet<String> = c.into_iter().collect();
    let d: BTreeSet<String> = d.into_iter().collect();
    println!();
    println!(
        "  (c) {} orphaned CDN assets — CDN asset not registered in sources.φ (by basename)",
        c.len()
    );
    print_tag_histogram("      ", report, &c, full);
    print_name_list("      ", &c, full, limit);

    println!();
    println!(
        "  (d) {} CDN-only assets — CDN asset with no local data/ copy (info: CI-generated assets)",
        d.len()
    );
    print_tag_histogram("      ", report, &d, full);
    print_name_list("      ", &d, full, limit);
}

fn print_tag_histogram(indent: &str, report: &Report, names: &BTreeSet<String>, full: bool) {
    let rows = tag_histogram(report, names);
    let total = rows.len();
    let shown = if full { total } else { total.min(25) };
    for (tag, count) in rows.iter().take(shown) {
        println!("{}release {}: {}", indent, tag, count);
    }
    if total > shown {
        println!(
            "{}… and {} more releases (--full lists every release)",
            indent,
            total - shown
        );
    }
}

fn print_name_list(indent: &str, names: &BTreeSet<String>, full: bool, limit: usize) {
    let total = names.len();
    if total == 0 || full || total <= limit {
        for name in names {
            println!("{}{}", indent, name);
        }
        return;
    }
    for name in names.iter().take(limit) {
        println!("{}{}", indent, name);
    }
    println!(
        "{}… and {} more (--full lists every name)",
        indent,
        total - limit
    );
}

fn print_json(report: &Report) {
    let a: Vec<String> = local_unmanifested(report)
        .iter()
        .map(|asset| format!("data/{}", asset.rel))
        .collect();
    let b: Vec<String> = registered_unmanifested(report)
        .iter()
        .map(|(tag, asset)| format!("{}/{}", tag, asset))
        .collect();
    let c: BTreeSet<String> = orphaned_cdn(report).into_iter().collect();
    let d: BTreeSet<String> = cdn_only(report).into_iter().collect();

    let mut body = String::from("{");
    push_json_field(&mut body, "cdn_releases", &report.releases.to_string());
    push_json_field(
        &mut body,
        "cdn_asset_rows",
        &report.asset_rows.len().to_string(),
    );
    push_json_field(
        &mut body,
        "cdn_distinct_names",
        &report.cdn_names.len().to_string(),
    );
    push_json_field(&mut body, "data_files", &report.data_files.to_string());
    push_json_field(
        &mut body,
        "data_netloc_dirs",
        &report.data_dirs.len().to_string(),
    );
    push_json_field(
        &mut body,
        "data_compiled_files",
        &report.compiled_files.len().to_string(),
    );
    push_json_field(
        &mut body,
        "data_compiled_distinct_names",
        &report.compiled_names.len().to_string(),
    );
    push_json_field(&mut body, "cache_hosts", &report.cache_hosts.to_string());
    push_json_field(&mut body, "cache_files", &report.cache_files.to_string());
    push_json_field(
        &mut body,
        "registered_cdn_url_lines",
        &report.registered.len().to_string(),
    );
    push_json_field(
        &mut body,
        "registered_distinct_assets",
        &report.registered_names.len().to_string(),
    );
    push_json_field(&mut body, "a_local_unmanifested", &json_str_list(&a));
    push_json_field(&mut body, "b_registered_unmanifested", &json_str_list(&b));
    push_json_field(&mut body, "c_orphaned_cdn_count", &c.len().to_string());
    push_json_field(&mut body, "c_orphaned_cdn_names", &json_str_list(&c));
    push_json_field(&mut body, "d_cdn_only_count", &d.len().to_string());
    push_json_field(&mut body, "d_cdn_only_names", &json_str_list(&d));
    body.push_str("\n}\n");
    print!("{}", body);
}

fn json_str_list<'a, I>(items: I) -> String
where
    I: IntoIterator<Item = &'a String>,
{
    let inner: Vec<String> = items.into_iter().map(|s| esc(s)).collect();
    format!("[{}]", inner.join(", "))
}

fn push_json_field(body: &mut String, key: &str, value: &str) {
    if body.len() > 1 {
        body.push(',');
    }
    body.push('\n');
    body.push_str(&format!("  {}: {}", esc(key), value));
}

fn esc(s: &str) -> String {
    let mut o = String::with_capacity(s.len() + 2);
    o.push('"');
    for c in s.chars() {
        match c {
            '"' => o.push_str("\\\""),
            '\\' => o.push_str("\\\\"),
            '\n' => o.push_str("\\n"),
            '\t' => o.push_str("\\t"),
            '\r' => o.push_str("\\r"),
            c if (c as u32) < 0x20 => o.push_str(&format!("\\u{:04x}", c as u32)),
            c => o.push(c),
        }
    }
    o.push('"');
    o
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compiled_names_recognize_field_and_witness_types() {
        for name in [
            "dr3_stars.bin",
            "cmb_planck_smica_n64.json",
            "gebco_bathymetry.gbco",
            "skydirections.sky1",
            "antares_events_2007_2017.s2e1",
            "auger_catalog.pao1",
            "icecube_alerts.amn1",
            "bayestar2019.be19",
            "gaia_dr3_vlies.vlde",
        ] {
            assert!(is_compiled_name(name), "{} should be compiled", name);
        }
    }

    #[test]
    fn raw_names_are_not_compiled() {
        for name in [
            "omni2_1984.csv",
            "xr_20140815.nc",
            "aia2015_log.txt",
            "spk_131005.bsp",
            "omni_raw.epoch",
            "noext",
            ".hidden",
        ] {
            assert!(!is_compiled_name(name), "{} should stay raw", name);
        }
    }

    #[test]
    fn case_and_upper_suffix_are_compiled() {
        assert!(is_compiled_name("MAP.BIN"));
        assert!(is_compiled_name("Deep_Map.SKY1"));
    }

    #[test]
    fn cdn_line_parts_extracts_tag_and_asset() {
        let url = "https://github.com/omegaflow/sources/releases/download/data-argo.ifremer.fr/argo_bgc.bin";
        let (tag, asset) = cdn_line_parts(url).expect("registered cdn line");
        assert_eq!(tag, "data-argo.ifremer.fr");
        assert_eq!(asset, "argo_bgc.bin");
    }

    #[test]
    fn non_cdn_urls_carry_no_line_parts() {
        assert!(cdn_line_parts("https://api.wolfx.jp/jma_eew.json").is_none());
        assert!(
            cdn_line_parts("https://github.com/omegaflow/sources/releases/download/").is_none()
        );
        assert!(cdn_line_parts(
            "https://github.com/omegaflow/sources/releases/download/x.y/a/b.bin"
        )
        .is_none());
    }
}
