use omegaflow::archivar::{
    cdn_manifest_map, extract_netloc, json_num, jstr, load_sources_from, parse_json,
    source_name_from_url, JsonVal, SourceConfig,
};
use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::process::Command;

const CDN_REPO: &str = "omegaflow/sources";

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

fn release_assets(release: &JsonVal) -> Vec<(String, Option<String>, u64)> {
    let mut out = Vec::new();
    let Some(JsonVal::Arr(assets)) = omegaflow::archivar::jpath_val(release, "assets") else {
        return out;
    };
    for a in assets {
        let name = jstr(a, "name").unwrap_or_default();
        if name.is_empty() {
            continue;
        }
        let digest = jstr(a, "digest");
        let size = json_num(a).map(|_| 0u64).unwrap_or(0);
        let size = jstr(a, "size")
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(size);
        out.push((name, digest, size));
    }
    out
}

fn collect_releases(body: &str) -> Vec<(String, Vec<(String, Option<String>, u64)>)> {
    let mut out = Vec::new();
    let Some(JsonVal::Arr(items)) = parse_json(body) else {
        return out;
    };
    for it in items {
        let tag = jstr(&it, "tag_name").unwrap_or_default();
        if !tag.is_empty() {
            out.push((tag, release_assets(&it)));
        }
    }
    out
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

fn str_list(items: &[String]) -> String {
    let inner: Vec<String> = items.iter().map(|s| esc(s)).collect();
    format!("[{}]", inner.join(", "))
}

fn row_list(rows: &[BTreeMap<&'static str, String>]) -> String {
    let objs: Vec<String> = rows
        .iter()
        .map(|r| {
            let inner: Vec<String> = r
                .iter()
                .map(|(k, v)| format!("{}: {}", esc(k), esc(v)))
                .collect();
            format!("{{{}}}", inner.join(", "))
        })
        .collect();
    format!("[{}]", objs.join(", "))
}

fn dupe_list(dups: &BTreeMap<String, Vec<String>>) -> String {
    let objs: Vec<String> = dups
        .iter()
        .map(|(netloc, tags)| {
            format!(
                "{{{}: {}, {}: {}}}",
                esc("netloc"),
                esc(netloc),
                esc("tags"),
                str_list(tags)
            )
        })
        .collect();
    format!("[{}]", objs.join(", "))
}

fn dup_map_sorted(dups: &BTreeMap<String, Vec<String>>) -> String {
    let objs: Vec<String> = dups
        .iter()
        .map(|(class, items)| {
            format!(
                "{{{}: {}, {}: {}}}",
                esc("class"),
                esc(class),
                esc("netlocs"),
                str_list(items)
            )
        })
        .collect();
    format!("[{}]", objs.join(", "))
}

fn group_list(groups: &[Vec<String>]) -> String {
    let objs: Vec<String> = groups
        .iter()
        .map(|g| format!("{{{}: {}}}", esc("names"), str_list(g)))
        .collect();
    format!("[{}]", objs.join(", "))
}

fn main() {
    let mut root = String::from(".");
    let mut out_path = String::from("docs/specs/cdn_reconciliation.json");
    let mut source_path = String::from("phi/sources.φ");
    let args: Vec<String> = env::args().collect();
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--root" => {
                i += 1;
                root = args[i].clone();
            }
            "--out" => {
                i += 1;
                out_path = args[i].clone();
            }
            "--sources" => {
                i += 1;
                source_path = args[i].clone();
            }
            _ => {}
        }
        i += 1;
    }

    let full_sources = format!("{}/{}", root, source_path);
    let content = match std::fs::read_to_string(&full_sources) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("cdn_reconcile: cannot read {}: {}", full_sources, e);
            std::process::exit(1);
        }
    };
    let sources: Vec<SourceConfig> = load_sources_from(&content);

    let manifest = cdn_manifest_map();
    let canonical_of = |u: &str| -> String {
        manifest
            .get(u)
            .cloned()
            .unwrap_or_else(|| source_name_from_url(u))
    };

    let mut netloc_of_source: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for s in &sources {
        if let Some(netloc) = extract_netloc(&s.url) {
            netloc_of_source
                .entry(netloc.to_string())
                .or_default()
                .insert(s.url.clone());
        }
    }

    let body = match gh_api_releases() {
        Some(b) => b,
        None => {
            eprintln!("cdn_reconcile: gh api release list void");
            std::process::exit(1);
        }
    };
    let releases = collect_releases(&body);

    let mut tag_netloc: BTreeMap<String, String> = BTreeMap::new();
    for (tag, _) in &releases {
        tag_netloc.insert(
            tag.clone(),
            tag.strip_prefix("www.").unwrap_or(tag).to_string(),
        );
    }

    let source_netlocs: BTreeSet<String> = netloc_of_source.keys().cloned().collect();
    let release_netlocs: BTreeSet<String> = tag_netloc.values().cloned().collect();

    let dataset_hosts: BTreeSet<String> = [
        "ssd.jpl.nasa.gov",
        "spdf.gsfc.nasa.gov",
        "physionet.org",
        "sentinel1euwest.blob.core.windows.net",
        "archive-api.open-meteo.com",
        "irsa.ipac.caltech.edu",
        "data.pmel.noaa.gov",
        "fermi.gsfc.nasa.gov",
        "service.iris.edu",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();

    let classify = |nl: &str| -> &'static str {
        if dataset_hosts.contains(nl) {
            "dataset_host"
        } else if nl.starts_with("github.com")
            || nl.starts_with("raw.githubusercontent.com")
            || nl.starts_with("github.com-")
        {
            "repo_tag"
        } else {
            "stale_pending"
        }
    };

    let mut orphan_releases: Vec<String> = Vec::new();
    let mut orphan_by_class: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut duplicate_netloc_tags: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for (tag, nl) in &tag_netloc {
        if !source_netlocs.contains(nl) {
            orphan_releases.push(tag.clone());
            orphan_by_class
                .entry(classify(nl).to_string())
                .or_default()
                .push(tag.clone());
        } else {
            duplicate_netloc_tags
                .entry(nl.clone())
                .or_default()
                .push(tag.clone());
        }
    }
    orphan_releases.sort();
    for v in orphan_by_class.values_mut() {
        v.sort();
    }
    duplicate_netloc_tags.retain(|_, v| v.len() > 1);

    let mut unmanifested_sources: Vec<String> = Vec::new();
    for nl in &source_netlocs {
        if !release_netlocs.contains(nl) {
            unmanifested_sources.push(nl.clone());
        }
    }
    unmanifested_sources.sort();

    let mut divergence: Vec<BTreeMap<&'static str, String>> = Vec::new();
    let mut missing_assets: Vec<BTreeMap<&'static str, String>> = Vec::new();
    let mut digest_seen: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for (nl, urls) in &netloc_of_source {
        let canonical: BTreeSet<String> = urls.iter().map(|u| canonical_of(u)).collect();
        let mut actual: BTreeSet<String> = BTreeSet::new();
        for (tag, assets) in &releases {
            if tag_netloc.get(tag).map(|x| x == nl).unwrap_or(false) {
                for (name, digest, _size) in assets {
                    actual.insert(name.clone());
                    if let Some(d) = digest {
                        digest_seen.entry(d.clone()).or_default().push(name.clone());
                    }
                }
            }
        }
        for exp in &canonical {
            if !actual.contains(exp) {
                let mut row = BTreeMap::new();
                row.insert("netloc", nl.clone());
                row.insert("expected", exp.clone());
                missing_assets.push(row);
            }
        }
        for act in &actual {
            if !canonical.contains(act) {
                let mut row = BTreeMap::new();
                row.insert("netloc", nl.clone());
                row.insert("actual", act.clone());
                divergence.push(row);
            }
        }
    }
    divergence.sort_by(|a, b| a.get("netloc").cmp(&b.get("netloc")));
    missing_assets.sort_by(|a, b| a.get("netloc").cmp(&b.get("netloc")));

    let mut byte_dupes: Vec<Vec<String>> = Vec::new();
    for (_d, names) in &digest_seen {
        let uniq: BTreeSet<String> = names.iter().cloned().collect();
        if uniq.len() > 1 {
            let mut v: Vec<String> = uniq.into_iter().collect();
            v.sort();
            byte_dupes.push(v);
        }
    }
    byte_dupes.sort();

    let mut report: Vec<(String, String)> = Vec::new();
    report.push(("sources_parsed".into(), sources.len().to_string()));
    report.push(("releases_live".into(), releases.len().to_string()));
    report.push(("source_netlocs".into(), source_netlocs.len().to_string()));
    report.push(("release_netlocs".into(), release_netlocs.len().to_string()));
    report.push(("orphan_releases".into(), str_list(&orphan_releases)));
    report.push((
        "orphan_releases_by_class".into(),
        dup_map_sorted(&orphan_by_class),
    ));
    report.push((
        "unmanifested_source_netlocs".into(),
        str_list(&unmanifested_sources),
    ));
    report.push((
        "duplicate_netloc_tags".into(),
        dupe_list(&duplicate_netloc_tags),
    ));
    report.push(("asset_name_divergence".into(), row_list(&divergence)));
    report.push(("missing_assets".into(), row_list(&missing_assets)));
    report.push((
        "byte_identical_duplicate_groups".into(),
        group_list(&byte_dupes),
    ));

    let mut body = String::from("{");
    for (i, (k, v)) in report.iter().enumerate() {
        if i > 0 {
            body.push(',');
        }
        body.push('\n');
        body.push_str(&format!("  {}: {}", esc(k), v));
    }
    body.push_str("\n}\n");
    let out_text = body;
    let out_full = if out_path.starts_with('/') {
        out_path.clone()
    } else {
        format!("{}/{}", root, out_path)
    };
    if let Some(parent) = std::path::Path::new(&out_full).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    match std::fs::write(&out_full, &out_text) {
        Ok(()) => eprintln!(
            "cdn_reconcile: {} written (orphan {} unmanifest {} divergence {} missing {} dupegroups {})",
            out_full,
            orphan_releases.len(),
            unmanifested_sources.len(),
            divergence.len(),
            missing_assets.len(),
            byte_dupes.len()
        ),
        Err(e) => {
            eprintln!("cdn_reconcile: write {} void: {}", out_full, e);
            std::process::exit(1);
        }
    }
}
