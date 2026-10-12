use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::process::Command;

fn curl(url: &str) -> Option<String> {
    let out = Command::new("curl")
        .arg("-sSL")
        .arg("-m")
        .arg("60")
        .arg("-H")
        .arg("User-Agent: omegaflow-licence-resolve")
        .arg(url)
        .output()
        .ok()?;
    if out.status.success() {
        Some(String::from_utf8_lossy(&out.stdout).into_owned())
    } else {
        None
    }
}

fn host_of(u: &str) -> String {
    let t = u.trim();
    let t = match t.find("://") {
        Some(p) => &t[p + 3..],
        None => t,
    };
    let t = t.split('/').next().unwrap_or("");
    let t = t.split('@').next_back().unwrap_or(t);
    let t = t.split(':').next().unwrap_or(t);
    t.trim_start_matches("www.").to_string()
}

fn is_hostname(h: &str) -> bool {
    h.contains('.') && !h.contains(' ') && !h.contains('?')
}

fn ends_with_any(h: &str, suffixes: &[&str]) -> bool {
    suffixes.iter().any(|s| h == *s || h.ends_with(&format!(".{s}")))
}

struct Block {
    url: String,
    origin: String,
    terms: String,
    terms_ref: String,
}

fn category(terms: &str) -> &'static str {
    match terms {
        "" => "NO_TERMS",
        "unknown" => "UNKNOWN",
        "free-open" => "FREE_OPEN",
        "no-license" => "NO_LICENSE",
        "own-work" => "OWN_WORK",
        "PD" => "PD",
        _ => "NAMED",
    }
}

const GOV_PD: &[&str] = &[
    "nasa.gov",
    "noaa.gov",
    "ncei.noaa.gov",
    "usgs.gov",
    "nist.gov",
    "census.gov",
    "epa.gov",
    "navy.mil",
    "sciencebase.gov",
    "waterqualitydata.us",
    "pds.nasa.gov",
];
const ESA_CCBY: &[&str] = &[
    "copernicus.eu",
    "esa.int",
    "dataspace.copernicus.eu",
    "scihub.copernicus.eu",
];
const PDS: &[&str] = &[
    "igpp.ucla.edu",
    "astro.umd.edu",
    "wustl.edu",
    "seti.org",
    "psi.edu",
    "nmsu.edu",
];
const VIZIER: &[&str] = &["cds.unistra.fr", "unistra.fr", "cfa.harvard.edu"];
const IOOS: &[&str] = &["ioos.us"];
const GITHUB: &[&str] = &["github.com", "raw.githubusercontent.com", "media.githubusercontent.com"];

fn json_str(hay: &str, key: &str) -> Option<String> {
    let needle = format!("\"{key}\":");
    let pos = hay.find(&needle)? + needle.len();
    let rest = hay[pos..].trim_start().strip_prefix('"')?;
    let end = rest.find('"')?;
    Some(rest[..end].to_string())
}

fn erddap_licence(url: &str) -> Option<String> {
    let h = host_of(url);
    let scheme = if url.starts_with("http://") {
        "http"
    } else {
        "https"
    };
    let after = url.split("/erddap/").nth(1)?;
    let mut segs = after.split('/');
    segs.next()?;
    let ds = segs.next()?.split('.').next()?;
    let info = format!("{scheme}://{h}/erddap/info/{ds}/index.json");
    let body = curl(&info)?;
    let pos = body.find("\"license\"")?;
    let after = &body[pos..];
    let s = after.find("\"String\"")? + 8;
    let rest = &after[s..];
    let q = rest.find('"')?;
    let rest2 = &rest[q + 1..];
    let end = rest2.find('"')?;
    Some(rest2[..end].to_string())
}

fn zenodo_licence(url: &str) -> Option<String> {
    let id = url.split("/records/").nth(1)?.split('/').next()?;
    let body = curl(&format!("https://zenodo.org/api/records/{id}"))?;
    let pos = body.find("\"license\"")?;
    let seg = &body[pos..];
    json_str(seg, "id")
}

fn github_licence(url: &str) -> Option<String> {
    let after = url.split("://").nth(1)?;
    let mut segs = after.split('/');
    segs.next()?;
    let owner = segs.next()?;
    let repo = segs.next()?;
    if owner.is_empty() || repo.is_empty() {
        return None;
    }
    let body = curl(&format!("https://api.github.com/repos/{owner}/{repo}/license"))?;
    json_str(&body, "spdx_id")
}

fn hapi_licence(url: &str) -> Option<String> {
    let base = url.split("/hapi/").next()?;
    let id = url.split("id=").nth(1)?.split('&').next()?;
    let body = curl(&format!("{base}/hapi/info?id={id}"))?;
    if let Some(v) = json_str(&body, "license") {
        return Some(v);
    }
    if let Some(v) = json_str(&body, "x_dataTerms") {
        if let Some(p) = v.find("http") {
            let tail = &v[p..];
            let end = tail.find(char::is_whitespace).unwrap_or(tail.len());
            let cand = tail[..end].trim_end_matches(['.', ',', ';', ')']);
            return Some(cand.to_string());
        }
        return Some(v);
    }
    None
}

fn memo<F: FnOnce() -> Option<String>>(
    cache: &mut BTreeMap<String, Option<String>>,
    key: &str,
    f: F,
) -> Option<String> {
    if let Some(v) = cache.get(key) {
        return v.clone();
    }
    let v = f();
    cache.insert(key.to_string(), v.clone());
    v
}

fn propose(
    block: &Block,
    host: &str,
    cache: &mut BTreeMap<String, Option<String>>,
) -> (String, String) {
    if ends_with_any(host, GOV_PD) {
        return ("PD-USGov".to_string(), "gov-blanket".to_string());
    }
    if ends_with_any(host, ESA_CCBY) {
        return ("CC-BY-4.0".to_string(), "esa-blanket".to_string());
    }
    if ends_with_any(host, PDS) {
        return ("PD-USGov (PDS)".to_string(), "pds-blanket".to_string());
    }
    if ends_with_any(host, VIZIER) {
        return (
            "vizier-scientific-free".to_string(),
            "vizier-rule".to_string(),
        );
    }
    if block.url.contains("/erddap/") {
        if let Some(v) = memo(cache, &format!("erddap:{host}"), || {
            erddap_licence(&block.url)
        }) {
            return (v, "erddap".to_string());
        }
    }
    if host == "zenodo.org" {
        let id = block.url.split("/records/").nth(1).and_then(|s| s.split('/').next());
        if let Some(id) = id {
            if let Some(v) = memo(cache, &format!("zenodo:{id}"), || zenodo_licence(&block.url)) {
                return (v, "zenodo".to_string());
            }
        }
    }
    if ends_with_any(host, GITHUB) {
        if let Some(rest) = block.url.split("://").nth(1) {
            if let Some(v) = memo(cache, &format!("github:{rest}"), || github_licence(&block.url)) {
                return (v, "github".to_string());
            }
        }
    }
    if block.url.contains("/hapi/") {
        let key = block.url.split("/hapi/").next().unwrap_or("").to_string();
        if let Some(v) = memo(cache, &format!("hapi:{key}"), || hapi_licence(&block.url)) {
            return (v, "hapi".to_string());
        }
    }
    if ends_with_any(host, IOOS) {
        return ("ioos-open".to_string(), "ioos-blanket".to_string());
    }
    let ev = if block.terms_ref.is_empty() {
        "none".to_string()
    } else {
        block.terms_ref.clone()
    };
    ("manual".to_string(), format!("manual:{ev}"))
}

fn parse_register(text: &str) -> Vec<Block> {
    let mut blocks: Vec<Block> = Vec::new();
    let mut cur = Block {
        url: String::new(),
        origin: String::new(),
        terms: String::new(),
        terms_ref: String::new(),
    };
    let mut any = false;
    let flush = |b: &mut Block, any: &mut bool, out: &mut Vec<Block>| {
        if *any {
            out.push(Block {
                url: b.url.clone(),
                origin: b.origin.clone(),
                terms: b.terms.clone(),
                terms_ref: b.terms_ref.clone(),
            });
        }
        b.url.clear();
        b.origin.clear();
        b.terms.clear();
        b.terms_ref.clear();
        *any = false;
    };
    for line in text.lines() {
        if line.trim().is_empty() {
            flush(&mut cur, &mut any, &mut blocks);
        } else {
            any = true;
            if let Some(v) = line.strip_prefix("url ") {
                if cur.url.is_empty() {
                    cur.url = v.trim().to_string();
                }
            } else if let Some(v) = line.strip_prefix("origin ") {
                if cur.origin.is_empty() {
                    cur.origin = v.trim().to_string();
                }
            } else if let Some(v) = line.strip_prefix("terms ") {
                let mut it = v.split_whitespace();
                cur.terms = it.next().unwrap_or("").to_string();
                cur.terms_ref = it.next().unwrap_or("").to_string();
            }
        }
    }
    flush(&mut cur, &mut any, &mut blocks);
    blocks
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut register = "phi/sources.φ".to_string();
    let mut out: Option<String> = None;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--register" => {
                if let Some(v) = args.get(i + 1) {
                    register = v.clone();
                }
                i += 1;
            }
            "--out" => {
                out = args.get(i + 1).cloned();
                i += 1;
            }
            _ => {}
        }
        i += 1;
    }

    let text = match fs::read_to_string(&register) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("read {register}: {e}");
            std::process::exit(1);
        }
    };
    let blocks = parse_register(&text);
    let mut rows: Vec<String> = Vec::new();
    let mut host_counts: BTreeMap<String, (usize, String)> = BTreeMap::new();
    let mut cache: BTreeMap<String, Option<String>> = BTreeMap::new();
    let mut resolved = 0usize;
    let mut manual = 0usize;

    for b in &blocks {
        let cat = category(&b.terms);
        if cat == "NAMED" {
            continue;
        }
        let host = {
            let oh = host_of(&b.origin);
            if is_hostname(&oh) {
                oh
            } else {
                host_of(&b.url)
            }
        };
        let (proposed, method) = propose(b, &host, &mut cache);
        let tag = if method.starts_with("manual") {
            manual += 1;
            "MANUAL"
        } else {
            resolved += 1;
            "RESOLVED"
        };
        let entry = host_counts
            .entry(host.clone())
            .or_insert((0, proposed.clone()));
        entry.0 += 1;
        rows.push(format!(
            "{tag}\t{cat}\t{host}\t{proposed}\t{method}\t{}\t{}",
            b.url, b.terms_ref
        ));
    }

    let mut report = String::new();
    report.push_str(&format!(
        "# licence-resolve candidates — register={register}\n# blocks(ambiguous+PD)={} resolved={resolved} manual={manual}\n",
        rows.len()
    ));
    report.push_str("TAG\tCAT\tHOST\tPROPOSED\tMETHOD\tURL\tTERMS_REF\n");
    for r in &rows {
        report.push_str(r);
        report.push('\n');
    }
    report.push_str("\n# per-host summary (count, proposed)\n");
    for (h, (n, p)) in &host_counts {
        report.push_str(&format!("{n}\t{h}\t{p}\n"));
    }

    match out {
        Some(path) => {
            if let Err(e) = fs::write(&path, &report) {
                eprintln!("write {path}: {e}");
                std::process::exit(1);
            }
            eprintln!("wrote {path} — {resolved} resolved, {manual} manual");
        }
        None => print!("{report}"),
    }
}
