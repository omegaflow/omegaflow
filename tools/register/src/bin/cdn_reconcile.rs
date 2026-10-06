use omegaflow::archivar::{
    JsonVal, SourceConfig, cdn_manifest_map, extract_netloc, jstr, load_sources_from, parse_json,
    reference_name_from_url, source_name_from_url,
};
use omegaflow::cdn::{CAPPED_RELEASE, MODIS_LST_CMG_FAMILY};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::env;
use std::process::Command;

const CDN_REPO: &str = "omegaflow/sources";

const COMPILER_NETLOCS: &[&str] = &[
    "ssd.jpl.nasa.gov",
    "spdf.gsfc.nasa.gov",
    "physionet.org",
    "sentinel1euwest.blob.core.windows.net",
    "archive-api.open-meteo.com",
    "irsa.ipac.caltech.edu",
    "data.pmel.noaa.gov",
    "fermi.gsfc.nasa.gov",
    "service.iris.edu",
    "vizier.cfa.harvard.edu",
    "gsaweb.ast.cam.ac.uk",
    "ws.cadc-ccda.hia-iha.nrc-cnrc.gc.ca",
];

fn release_tag_netloc(tag: &str) -> (&str, bool) {
    match tag.strip_prefix("www.") {
        Some(bare) => (bare, true),
        None => (tag, false),
    }
}

fn cdn_tag_from_url(url: &str) -> Option<&str> {
    let rest = url.strip_prefix("https://github.com/omegaflow/sources/releases/download/")?;
    let tag = rest.split('/').next()?;
    if tag.is_empty() { None } else { Some(tag) }
}

fn origin_netlocs(raw: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut seen: BTreeSet<String> = BTreeSet::new();
    let b = raw.as_bytes();
    let mut i = 0usize;
    while i < b.len() {
        let scheme_len = if b[i..].starts_with(b"https://") {
            8
        } else if b[i..].starts_with(b"http://") {
            7
        } else {
            i += 1;
            continue;
        };
        let start = i + scheme_len;
        let mut end = start;
        while end < b.len()
            && !matches!(
                b[end],
                b'/' | b' ' | b'\t' | b'\r' | b'\n' | b';' | b'?' | b'#' | b','
            )
        {
            end += 1;
        }
        if end > start {
            let host = &raw[start..end];
            let bare = host.strip_prefix("www.").unwrap_or(host);
            if !bare.is_empty() && seen.insert(bare.to_string()) {
                out.push(bare.to_string());
            }
        }
        i = start;
    }
    out
}

fn register_hosts_raw(content: &str) -> BTreeSet<String> {
    let mut hosts: BTreeSet<String> = BTreeSet::new();
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some(rest) = line.strip_prefix("url ") {
            let url = rest.trim();
            if let Some(tag) = cdn_tag_from_url(url) {
                hosts.insert(tag.to_string());
            }
            if let Some(nl) = extract_netloc(url) {
                hosts.insert(nl.to_string());
            }
        }
        if let Some(rest) = line.strip_prefix("origin ") {
            for nl in origin_netlocs(rest.trim()) {
                hosts.insert(nl);
            }
        }
    }
    hosts
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

fn release_assets(release: &JsonVal) -> Vec<(String, Option<String>, u64)> {
    let mut out = Vec::new();
    let Some(JsonVal::Arr(assets)) = omegaflow::archivar::jpath_val(release, "assets") else {
        return out;
    };
    for a in assets {
        let name = match jstr(a, "name") {
            Some(n) if !n.is_empty() => n,
            _ => continue,
        };
        let digest = jstr(a, "digest");
        let size = match jstr(a, "size").and_then(|s| s.parse::<u64>().ok()) {
            Some(s) => s,
            None => 0u64,
        };
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
        match jstr(&it, "tag_name") {
            Some(tag) if !tag.is_empty() => out.push((tag, release_assets(&it))),
            _ => {}
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

fn shard_base(name: &str) -> Option<String> {
    if let Some((base, ordinal)) = name.rsplit_once('.') {
        if !ordinal.is_empty() && ordinal.bytes().all(|b| b.is_ascii_digit()) {
            return Some(base.to_string());
        }
    }
    if let Some((stem, ext)) = name.rsplit_once('.') {
        if let Some((base, shard)) = stem.rsplit_once('_') {
            if !shard.is_empty() && shard.bytes().all(|b| b.is_ascii_digit()) {
                return Some(format!("{}.{}", base, ext));
            }
        }
    }
    None
}

fn is_manifest(name: &str) -> bool {
    name.ends_with(".manifest")
}

fn is_shard_of(name: &str, stems: &[String]) -> bool {
    stems.iter().any(|stem| {
        name.strip_prefix(stem.as_str())
            .is_some_and(|rest| rest.starts_with('_'))
    })
}

fn literal_release_tags(root: &str) -> Vec<(String, usize, String, String)> {
    let dir = format!("{}/.github/workflows", root);
    let mut files: Vec<String> = Vec::new();
    if let Ok(rd) = std::fs::read_dir(&dir) {
        for e in rd.flatten() {
            let p = e.path();
            if p.extension().and_then(|x| x.to_str()) == Some("yml") {
                files.push(p.to_string_lossy().into_owned());
            }
        }
    }
    files.sort();
    let mut out = Vec::new();
    for f in &files {
        let Ok(text) = std::fs::read_to_string(f) else {
            continue;
        };
        let short = f.rsplit('/').next().unwrap_or(f).to_string();
        for (i, line) in text.lines().enumerate() {
            for (needle, kind) in [
                ("gh release upload ", "upload"),
                ("gh release create ", "create"),
                ("--release-tag ", "release-tag"),
            ] {
                let mut from = 0usize;
                while let Some(pos) = line[from..].find(needle) {
                    let start = from + pos + needle.len();
                    let raw = line[start..].split_whitespace().next().unwrap_or("");
                    let tag = raw.trim_matches(|c| c == '\'' || c == '"');
                    if is_literal_tag(tag) {
                        out.push((short.clone(), i + 1, kind.to_string(), tag.to_string()));
                    }
                    from = start;
                }
            }
        }
    }
    out
}

fn is_literal_tag(tag: &str) -> bool {
    !tag.is_empty()
        && !tag.contains('$')
        && !tag.contains('{')
        && !tag.contains('(')
        && !tag.contains('/')
        && tag != "tools-latest"
}

fn host_known(tag: &str, hosts: &BTreeSet<String>) -> bool {
    hosts.contains(tag)
        || hosts.iter().any(|h| {
            tag.strip_prefix(h.as_str())
                .is_some_and(|rest| rest.starts_with('-'))
        })
        || COMPILER_NETLOCS.iter().any(|h| {
            tag == *h
                || tag
                    .strip_prefix(h)
                    .is_some_and(|rest| rest.starts_with('-'))
        })
}

fn tag_baseline(root: &str) -> BTreeSet<String> {
    let path = format!("{}/docs/specs/cdn-tag-baseline.txt", root);
    let mut set = BTreeSet::new();
    if let Ok(text) = std::fs::read_to_string(&path) {
        for l in text.lines() {
            let l = l.trim();
            if !l.is_empty() && !l.starts_with('#') {
                set.insert(l.to_string());
            }
        }
    }
    set
}

fn contract(root: &str, hosts: &BTreeSet<String>, exempt: &BTreeSet<String>) -> Vec<String> {
    let mut out = Vec::new();
    for (file, line, kind, tag) in literal_release_tags(root) {
        if (kind == "upload" || kind == "release-tag") && tag == CAPPED_RELEASE {
            out.push(format!(
                "{file}:{line}: {kind} writes the capped release {CAPPED_RELEASE} — use the family tag \"<host>-<family>\" (cdn.rs CAPPED_RELEASE)"
            ));
        } else if !host_known(&tag, hosts) && !exempt.contains(&tag) {
            out.push(format!(
                "{file}:{line}: {kind} writes \"{tag}\", not bound in phi/sources.φ"
            ));
        }
    }
    out.sort();
    out.dedup();
    out
}

fn main() {
    let mut root = String::from(".");
    let mut out_path = String::from("docs/specs/cdn_reconciliation.json");
    let mut source_path = String::from("phi/sources.φ");
    let mut fail = false;
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
            "--fail" => fail = true,
            _ => {}
        }
        i += 1;
    }

    if fail {
        let full = format!("{}/{}", root, source_path);
        let content = match std::fs::read_to_string(&full) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("cdn_reconcile: read {} void: {}", full, e);
                std::process::exit(1);
            }
        };
        let hosts = register_hosts_raw(&content);
        let drift = contract(&root, &hosts, &tag_baseline(&root));
        if drift.is_empty() {
            eprintln!(
                "cdn_reconcile: cap+tag contract clean ({} registry hosts)",
                hosts.len()
            );
            return;
        }
        for d in &drift {
            eprintln!("cdn_reconcile: {d}");
        }
        std::process::exit(2);
    }

    let full_sources = format!("{}/{}", root, source_path);
    let content = match std::fs::read_to_string(&full_sources) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("cdn_reconcile: read {} void: {}", full_sources, e);
            std::process::exit(1);
        }
    };
    let sources: Vec<SourceConfig> = load_sources_from(&content);

    let manifest = cdn_manifest_map();
    let mut canonical_map: HashMap<String, String> = HashMap::new();
    for s in &sources {
        let is_cdn = s
            .url
            .starts_with("https://github.com/omegaflow/sources/releases/download/");
        let name = if s.format == "reference" || is_cdn {
            reference_name_from_url(&s.url)
        } else {
            match manifest.get(&s.url) {
                Some(n) => n.clone(),
                None => source_name_from_url(&s.url),
            }
        };
        canonical_map.insert(s.url.clone(), name);
    }
    let canonical_of = |u: &str| -> String {
        match canonical_map.get(u) {
            Some(n) => n.clone(),
            None => source_name_from_url(u),
        }
    };

    let mut netloc_of_source: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for s in &sources {
        match cdn_tag_from_url(&s.url) {
            Some(tag) => {
                let (netloc, _) = release_tag_netloc(tag);
                netloc_of_source
                    .entry(netloc.to_string())
                    .or_default()
                    .insert(s.url.clone());
            }
            None => {
                if let Some(netloc) = extract_netloc(&s.url) {
                    netloc_of_source
                        .entry(netloc.to_string())
                        .or_default()
                        .insert(s.url.clone());
                }
                if let Some(origin) = &s.origin {
                    for netloc in origin_netlocs(origin) {
                        netloc_of_source
                            .entry(netloc)
                            .or_default()
                            .insert(s.url.clone());
                    }
                }
            }
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
    let mut www_prefixed_tags: Vec<String> = Vec::new();
    for (tag, _) in &releases {
        let (netloc, www_prefixed) = release_tag_netloc(tag);
        if www_prefixed {
            www_prefixed_tags.push(tag.clone());
        }
        tag_netloc.insert(tag.clone(), netloc.to_string());
    }
    www_prefixed_tags.sort();

    let source_netlocs: BTreeSet<String> = netloc_of_source.keys().cloned().collect();
    let release_netlocs: BTreeSet<String> = tag_netloc.values().cloned().collect();

    let dataset_hosts: BTreeSet<String> = COMPILER_NETLOCS.iter().map(|s| s.to_string()).collect();

    let non_source_tags: BTreeSet<&str> =
        ["srdata.nist.gov", "rave-survey.org"].into_iter().collect();

    let modis_year_prefix = format!("{MODIS_LST_CMG_FAMILY}-");
    let classify = |nl: &str| -> &'static str {
        if dataset_hosts.contains(nl) {
            "dataset_host"
        } else if non_source_tags.contains(nl) {
            "non_source"
        } else if nl.starts_with("ps1-dr2-")
            || nl.starts_with("ssd.jpl.nasa.gov-")
            || nl.starts_with(modis_year_prefix.as_str())
        {
            "internal"
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
        let stems: Vec<String> = canonical
            .iter()
            .filter_map(|n| n.strip_suffix(".manifest").map(|s| s.to_string()))
            .collect();
        let sharded: BTreeSet<String> = canonical
            .iter()
            .filter(|exp| {
                if actual.contains(exp.as_str()) {
                    return false;
                }
                let shard_stems: Vec<String> = match exp.strip_suffix(".manifest") {
                    Some(s) => vec![s.to_string()],
                    None => Vec::new(),
                };
                actual.iter().any(|a| {
                    shard_base(a).as_deref() == Some(exp.as_str()) || is_shard_of(a, &shard_stems)
                })
            })
            .cloned()
            .collect();
        let manifest_present = actual.iter().any(|a| is_manifest(a));
        for exp in &canonical {
            let proven = actual.contains(exp) || (sharded.contains(exp) && manifest_present);
            if !proven {
                let mut row = BTreeMap::new();
                row.insert("netloc", nl.clone());
                row.insert("expected", exp.clone());
                missing_assets.push(row);
            }
        }
        for act in &actual {
            let explained = canonical.contains(act)
                || shard_base(act).is_some_and(|b| canonical.contains(b.as_str()))
                || is_shard_of(act, &stems)
                || (is_manifest(act) && !sharded.is_empty());
            if !explained {
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
    report.push((
        "www_prefixed_release_tags".into(),
        str_list(&www_prefixed_tags),
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
            "cdn_reconcile: {} written (orphan {} unmanifest {} divergence {} missing {} dupegroups {} www {})",
            out_full,
            orphan_releases.len(),
            unmanifested_sources.len(),
            divergence.len(),
            missing_assets.len(),
            byte_dupes.len(),
            www_prefixed_tags.len()
        ),
        Err(e) => {
            eprintln!("cdn_reconcile: write {} void: {}", out_full, e);
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn release_tag_netloc_flags_www_prefix() {
        assert_eq!(release_tag_netloc("www.gmrt.org"), ("gmrt.org", true));
        assert_eq!(release_tag_netloc("gmrt.org"), ("gmrt.org", false));
        assert_eq!(
            release_tag_netloc("ssd.jpl.nasa.gov"),
            ("ssd.jpl.nasa.gov", false)
        );
    }

    #[test]
    fn cdn_tag_from_url_extracts_the_release_tag() {
        assert_eq!(
            cdn_tag_from_url(
                "https://github.com/omegaflow/sources/releases/download/ssd.jpl.nasa.gov-de/ephemeris_de440_earth.bin"
            ),
            Some("ssd.jpl.nasa.gov-de")
        );
        assert_eq!(
            cdn_tag_from_url(
                "https://github.com/omegaflow/sources/releases/download/noaa-goes18/glm_l2.bin"
            ),
            Some("noaa-goes18")
        );
    }

    #[test]
    fn cdn_tag_from_url_ignores_direct_urls() {
        assert_eq!(
            cdn_tag_from_url("https://api.open-meteo.com/v1/forecast"),
            None
        );
        assert_eq!(
            cdn_tag_from_url("https://github.com/omegaflow/sources/releases/download//x.bin"),
            None
        );
    }

    #[test]
    fn register_hosts_raw_binds_ttl_less_cdn_blocks() {
        let content = "\
url https://github.com/omegaflow/sources/releases/download/www2.census.gov/zcta_gazetteer.bin
origin https://www2.census.gov/geo/docs/maps-data/data/gazetteer/2020_Gazetteer/2020_Gaz_zcta_national.zip
compiler tools/harvest/src/bin/zcta_gazetteer_compiler.rs
format zcta_gazetteer
no-cadence
";
        let hosts = register_hosts_raw(content);
        assert!(host_known("www2.census.gov", &hosts), "{hosts:?}");
    }

    #[test]
    fn no_workflow_writes_the_capped_release() {
        let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");
        let bad: Vec<String> = literal_release_tags(root)
            .into_iter()
            .filter(|(_, _, kind, tag)| {
                (kind == "upload" || kind == "release-tag") && tag == CAPPED_RELEASE
            })
            .map(|(f, l, k, t)| format!("{f}:{l}: {k} {t}"))
            .collect();
        assert!(
            bad.is_empty(),
            "capped-release writers in .github/workflows:\n{}",
            bad.join("\n")
        );
    }
}
