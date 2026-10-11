use omegaflow::archivar::{RetryPolicy, TRANSFER_BOUND_S, fetch_raw_with};
use omegaflow::cdn::{CDN_BASE, upload_release};
use std::env;
use std::fs;
use std::path::Path;

const MANIFEST_NAME: &str = "ckan_coverage.manifest";
const COMPILER: &str = "tools/harvest/src/bin/ckan_coverage.rs";
const DEFAULT_PORTAL: &str = "https://data.cnra.ca.gov";
const DEFAULT_QUERY: &str = "title:lidar";
const PAGE_SIZE: usize = 100;
const MAX_PAGES: usize = 200;

#[derive(Debug)]
enum Json {
    Null,
    Bool(bool),
    Num(f64),
    Str(String),
    Arr(Vec<Json>),
    Obj(Vec<(String, Json)>),
}

struct JsonParser<'a> {
    b: &'a [u8],
    i: usize,
}

impl<'a> JsonParser<'a> {
    fn new(s: &'a str) -> Self {
        JsonParser {
            b: s.as_bytes(),
            i: 0,
        }
    }

    fn ws(&mut self) {
        while self.i < self.b.len() && matches!(self.b[self.i], b' ' | b'\t' | b'\n' | b'\r') {
            self.i += 1;
        }
    }

    fn parse(&mut self) -> Result<Json, String> {
        self.ws();
        match self.b.get(self.i) {
            Some(b'{') => self.parse_obj(),
            Some(b'[') => self.parse_arr(),
            Some(b'"') => Ok(Json::Str(self.parse_str()?)),
            Some(b't') => {
                self.lit("true")?;
                Ok(Json::Bool(true))
            }
            Some(b'f') => {
                self.lit("false")?;
                Ok(Json::Bool(false))
            }
            Some(b'n') => {
                self.lit("null")?;
                Ok(Json::Null)
            }
            Some(_) => self.parse_num(),
            None => Err("json: end of input where a value stands".to_string()),
        }
    }

    fn lit(&mut self, s: &str) -> Result<(), String> {
        match self.b.get(self.i..self.i + s.len()) {
            Some(slice) if slice == s.as_bytes() => {
                self.i += s.len();
                Ok(())
            }
            _ => Err(format!("json: expected {s} at byte {}", self.i)),
        }
    }

    fn parse_num(&mut self) -> Result<Json, String> {
        let start = self.i;
        while self.i < self.b.len()
            && matches!(
                self.b[self.i],
                b'0'..=b'9' | b'-' | b'+' | b'.' | b'e' | b'E'
            )
        {
            self.i += 1;
        }
        let raw = std::str::from_utf8(&self.b[start..self.i])
            .map_err(|_| "json: number not utf8".to_string())?;
        raw.parse::<f64>()
            .map(Json::Num)
            .map_err(|_| format!("json: number '{raw}' unreadable"))
    }

    fn parse_str(&mut self) -> Result<String, String> {
        self.i += 1;
        let mut out = String::new();
        while self.i < self.b.len() {
            match self.b[self.i] {
                b'"' => {
                    self.i += 1;
                    return Ok(out);
                }
                b'\\' => {
                    self.i += 1;
                    match self.b.get(self.i) {
                        Some(b'"') => {
                            out.push('"');
                            self.i += 1;
                        }
                        Some(b'\\') => {
                            out.push('\\');
                            self.i += 1;
                        }
                        Some(b'/') => {
                            out.push('/');
                            self.i += 1;
                        }
                        Some(b'b') => {
                            out.push('\u{8}');
                            self.i += 1;
                        }
                        Some(b'f') => {
                            out.push('\u{c}');
                            self.i += 1;
                        }
                        Some(b'n') => {
                            out.push('\n');
                            self.i += 1;
                        }
                        Some(b'r') => {
                            out.push('\r');
                            self.i += 1;
                        }
                        Some(b't') => {
                            out.push('\t');
                            self.i += 1;
                        }
                        Some(b'u') => {
                            self.i += 1;
                            let hex = self.b.get(self.i..self.i + 4).ok_or("json: \\u cut")?;
                            let hex = std::str::from_utf8(hex).map_err(|_| "json: \\u not utf8")?;
                            let cp =
                                u32::from_str_radix(hex, 16).map_err(|_| "json: \\u not hex")?;
                            self.i += 4;
                            if (0xD800..0xDC00).contains(&cp) {
                                if self.b.get(self.i..self.i + 2) == Some(b"\\u") {
                                    let low = self
                                        .b
                                        .get(self.i + 2..self.i + 6)
                                        .ok_or("json: \\u low cut")?;
                                    let low = std::str::from_utf8(low)
                                        .map_err(|_| "json: \\u low not utf8")?;
                                    let cp2 = u32::from_str_radix(low, 16)
                                        .map_err(|_| "json: \\u low not hex")?;
                                    if (0xDC00..0xE000).contains(&cp2) {
                                        let c = 0x10000 + ((cp - 0xD800) << 10) + (cp2 - 0xDC00);
                                        out.push(
                                            char::from_u32(c)
                                                .ok_or("json: surrogate not a code point")?,
                                        );
                                        self.i += 6;
                                        continue;
                                    }
                                }
                            }
                            out.push(char::from_u32(cp).ok_or("json: code point unreadable")?);
                        }
                        _ => return Err("json: bad escape".to_string()),
                    }
                }
                _ => {
                    let start = self.i;
                    self.i += 1;
                    while self.i < self.b.len() && (self.b[self.i] & 0xC0) == 0x80 {
                        self.i += 1;
                    }
                    out.push_str(
                        std::str::from_utf8(&self.b[start..self.i])
                            .map_err(|_| "json: text not utf8".to_string())?,
                    );
                }
            }
        }
        Err("json: string not closed".to_string())
    }

    fn parse_arr(&mut self) -> Result<Json, String> {
        self.i += 1;
        let mut out = Vec::new();
        self.ws();
        if self.b.get(self.i) == Some(&b']') {
            self.i += 1;
            return Ok(Json::Arr(out));
        }
        loop {
            out.push(self.parse()?);
            self.ws();
            match self.b.get(self.i) {
                Some(b',') => self.i += 1,
                Some(b']') => {
                    self.i += 1;
                    return Ok(Json::Arr(out));
                }
                _ => return Err("json: array separator expected".to_string()),
            }
        }
    }

    fn parse_obj(&mut self) -> Result<Json, String> {
        self.i += 1;
        let mut out = Vec::new();
        self.ws();
        if self.b.get(self.i) == Some(&b'}') {
            self.i += 1;
            return Ok(Json::Obj(out));
        }
        loop {
            self.ws();
            if self.b.get(self.i) != Some(&b'"') {
                return Err("json: object key expected".to_string());
            }
            let key = self.parse_str()?;
            self.ws();
            if self.b.get(self.i) != Some(&b':') {
                return Err("json: colon expected".to_string());
            }
            self.i += 1;
            let val = self.parse()?;
            out.push((key, val));
            self.ws();
            match self.b.get(self.i) {
                Some(b',') => self.i += 1,
                Some(b'}') => {
                    self.i += 1;
                    return Ok(Json::Obj(out));
                }
                _ => return Err("json: object separator expected".to_string()),
            }
        }
    }
}

fn parse_json(s: &str) -> Result<Json, String> {
    let mut p = JsonParser::new(s);
    p.parse()
}

impl Json {
    fn get(&self, key: &str) -> Option<&Json> {
        match self {
            Json::Obj(map) => map.iter().find(|(k, _)| k == key).map(|(_, v)| v),
            _ => None,
        }
    }

    fn as_str(&self) -> Option<&str> {
        match self {
            Json::Str(s) => Some(s),
            _ => None,
        }
    }

    fn as_f64(&self) -> Option<f64> {
        match self {
            Json::Num(n) => Some(*n),
            _ => None,
        }
    }

    fn as_bool(&self) -> Option<bool> {
        match self {
            Json::Bool(b) => Some(*b),
            _ => None,
        }
    }

    fn as_arr(&self) -> Option<&[Json]> {
        match self {
            Json::Arr(a) => Some(a),
            _ => None,
        }
    }
}

struct ShardResource {
    url: String,
    id: Option<String>,
    epoch: Option<String>,
}

struct CkanPage {
    resources: Vec<ShardResource>,
    packages: usize,
    count: Option<usize>,
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn listing_fetch(url: &str) -> Option<String> {
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

fn encode_query(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for b in value.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(b as char)
            }
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}

fn search_url(portal: &str, query: &str, rows: usize, start: usize) -> String {
    format!(
        "{}/api/3/action/package_search?q={}&rows={rows}&start={start}",
        portal.trim_end_matches('/'),
        encode_query(query)
    )
}

fn is_shard_url(url: &str) -> bool {
    let lower = url.to_ascii_lowercase();
    let path = lower.split(['?', '#']).next().unwrap_or(&lower);
    path.ends_with(".laz") || path.ends_with(".las") || path.ends_with(".zip")
}

fn parse_page(body: &str) -> Result<CkanPage, String> {
    let root = parse_json(body)?;
    if root.get("success").and_then(Json::as_bool) == Some(false) {
        return Err(
            "CKAN package_search answered success:false — the walk stays incomplete".to_string(),
        );
    }
    let result = root
        .get("result")
        .ok_or_else(|| "CKAN page carries no result".to_string())?;
    let count = result.get("count").and_then(Json::as_f64).and_then(|n| {
        if n.is_finite() && n >= 0.0 {
            Some(n as usize)
        } else {
            None
        }
    });
    let mut resources = Vec::new();
    let mut packages = 0usize;
    if let Some(page) = result.get("results").and_then(Json::as_arr) {
        packages = page.len();
        for pkg in page {
            let pkg_name = pkg.get("name").and_then(Json::as_str);
            if let Some(list) = pkg.get("resources").and_then(Json::as_arr) {
                for r in list {
                    if let Some(url) = r
                        .get("url")
                        .and_then(Json::as_str)
                        .filter(|u| !u.is_empty())
                    {
                        let id = r
                            .get("name")
                            .and_then(Json::as_str)
                            .or(pkg_name)
                            .map(|s| s.to_string());
                        let epoch = r
                            .get("last_modified")
                            .and_then(Json::as_str)
                            .or_else(|| r.get("created").and_then(Json::as_str))
                            .map(|s| s.to_string());
                        resources.push(ShardResource {
                            url: url.to_string(),
                            id,
                            epoch,
                        });
                    }
                }
            }
        }
    }
    Ok(CkanPage {
        resources,
        packages,
        count,
    })
}

fn enumerate(
    portal: &str,
    query: &str,
    rows: usize,
    max_pages: usize,
) -> Result<(Vec<ShardResource>, usize, Option<usize>), String> {
    let mut resources = Vec::new();
    let mut pages = 0usize;
    let mut start = 0usize;
    let mut count = None;
    loop {
        let url = search_url(portal, query, rows, start);
        let body = listing_fetch(&url).ok_or_else(|| {
            format!(
                "CKAN page {} at start {start} returned void ({url}) — the walk stays incomplete",
                pages + 1
            )
        })?;
        let page = parse_page(&body)?;
        pages += 1;
        if pages == 1 {
            count = page.count;
        }
        resources.extend(page.resources);
        start += page.packages;
        if page.packages == 0 || page.packages < rows {
            break;
        }
        if let Some(c) = count {
            if start >= c {
                break;
            }
        }
        if pages >= max_pages {
            return Err(format!(
                "CKAN walk reached the page bound {max_pages} — the walk stays incomplete"
            ));
        }
    }
    Ok((resources, pages, count))
}

fn manifest_lines(resources: &[ShardResource]) -> Vec<String> {
    let mut out = Vec::new();
    for r in resources {
        if is_shard_url(&r.url) {
            out.push(format!("shard {}", r.url));
        }
    }
    out
}

fn id_lines(resources: &[ShardResource]) -> Vec<String> {
    let mut out = Vec::new();
    for r in resources {
        if !is_shard_url(&r.url) {
            continue;
        }
        let id = r.id.as_deref().unwrap_or("-");
        let epoch = r.epoch.as_deref().unwrap_or("-");
        out.push(format!("{id}\t{epoch}\t{}", r.url));
    }
    out
}

fn run(args: &[String]) -> Result<(), String> {
    if args.iter().any(|a| a == "--help") {
        eprintln!(
            "usage: ckan_coverage [--portal <root>] [--q <query>] [--rows <n>] [--max-pages <n>] [--out <manifest>] [--emit-ids <path>] [--ci-mode]  (default: {DEFAULT_PORTAL} q={DEFAULT_QUERY}, a CKAN package_search offset walk ({PAGE_SIZE} packages/page, page bound {MAX_PAGES}), url-only shard lines; --emit-ids writes a resource-id sidecar)"
        );
        return Ok(());
    }
    let portal = match arg_value(args, "--portal").filter(|p| !p.is_empty()) {
        Some(p) => p,
        None => DEFAULT_PORTAL.to_string(),
    };
    if !portal.starts_with("http://") && !portal.starts_with("https://") {
        return Err(format!("--portal {portal}: not an http root — refused"));
    }
    let netloc =
        netloc_of(&portal).ok_or_else(|| format!("--portal {portal}: no host readable"))?;
    let query = match arg_value(args, "--q").filter(|q| !q.is_empty()) {
        Some(q) => q,
        None => DEFAULT_QUERY.to_string(),
    };
    let out = match arg_value(args, "--out").filter(|o| !o.is_empty()) {
        Some(o) => o,
        None => format!("data/{netloc}/{MANIFEST_NAME}"),
    };
    let out_name = match Path::new(&out).file_name().and_then(|n| n.to_str()) {
        Some(n) => n.to_string(),
        None => MANIFEST_NAME.to_string(),
    };
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let emit_ids = arg_value(args, "--emit-ids").filter(|p| !p.is_empty());
    let rows = match arg_value(args, "--rows").and_then(|v| v.parse::<usize>().ok()) {
        Some(n) if n > 0 => n,
        _ => PAGE_SIZE,
    };
    let max_pages = match arg_value(args, "--max-pages").and_then(|v| v.parse::<usize>().ok()) {
        Some(p) if p > 0 => p,
        _ => MAX_PAGES,
    };
    let origin = search_url(&portal, &query, rows, 0);

    let (mut resources, pages, count) = enumerate(&portal, &query, rows, max_pages)?;
    if pages == 0 || resources.is_empty() {
        return Err(format!(
            "{portal}: the CKAN walk carries no resource — nothing manifestiert (0 honored)"
        ));
    }
    resources.sort_by(|a, b| a.url.cmp(&b.url));
    resources.dedup_by(|a, b| a.url == b.url);
    let lines = manifest_lines(&resources);
    if lines.is_empty() {
        return Err(format!(
            "{portal} q='{query}': {} resources over {pages} page(s), none a point-cloud/archive url — nothing manifestiert (0 honored)",
            resources.len()
        ));
    }

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
    let count_note = match count {
        Some(c) => c.to_string(),
        None => "unmeasured".to_string(),
    };
    eprintln!(
        "ckan_coverage: {portal} q='{query}': {pages} page(s), {} resources, {} shards of {count_note} package(s) -> {out}",
        resources.len(),
        lines.len()
    );

    if let Some(path) = emit_ids.as_deref() {
        let ids = id_lines(&resources);
        let mut id_text = String::with_capacity(ids.len() * 128);
        for line in &ids {
            id_text.push_str(line);
            id_text.push('\n');
        }
        if let Some(parent) = Path::new(path).parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent)
                    .map_err(|e| format!("create {} returned void: {e}", parent.display()))?;
            }
        }
        fs::write(path, &id_text).map_err(|e| format!("write {path} returned void: {e}"))?;
        eprintln!("ckan_coverage: {} resource ids -> {path}", ids.len());
    }
    if ci_mode {
        if let Some(path) = emit_ids.as_deref() {
            if !upload_release(&netloc, path) {
                return Err(format!("{path} did not reach the CDN"));
            }
        }
    }

    println!("url {CDN_BASE}/{netloc}/{out_name}");
    println!("origin {origin}");
    println!("compiler {COMPILER}");
    println!("format ckan_coverage");

    if ci_mode && !upload_release(&netloc, &out) {
        return Err(format!("{out} did not reach the CDN"));
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("ckan_coverage: {msg}");
        std::process::exit(2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PAGE: &str = r#"{"help":"https://data.cnra.ca.gov/api/3/action/help_show?name=package_search","success":true,"result":{"count":65,"results":[
{"name":"dmp-000723","title":"Clear Lake Lidar","resources":[
{"format":"XLSX","name":"DMP-000723 Metadata Worksheet Clear Lake Lidar","url":"https://data.cnra.ca.gov/dataset/x/resource/y/download/dmp-000723_metadata-worksheet_clear_lake_lidar.xlsx","created":"2026-05-22T16:42:36.731226","last_modified":"2026-05-22T16:46:47.451603"},
{"format":"ZIP","name":"DMP-000723 Clear Lake Lidar","url":"https://data.cnra.ca.gov/dataset/x/resource/z/download/dmp-000723_clear_lake_lidar.zip","created":"2026-05-22T16:48:49.966846","last_modified":null}
]},
{"name":"empty-package","resources":[]}
]}}"#;

    #[test]
    fn measured_page_keeps_the_zip_and_leaves_out_the_worksheet() {
        let page = parse_page(PAGE).expect("measured page parses");
        assert_eq!(page.count, Some(65));
        assert_eq!(page.packages, 2);
        assert_eq!(page.resources.len(), 2);
        let lines = manifest_lines(&page.resources);
        assert_eq!(lines.len(), 1);
        assert_eq!(
            lines[0],
            "shard https://data.cnra.ca.gov/dataset/x/resource/z/download/dmp-000723_clear_lake_lidar.zip"
        );
    }

    #[test]
    fn shard_url_reads_point_cloud_and_archive_extensions_only() {
        assert!(is_shard_url("https://x/a.copc.laz"));
        assert!(is_shard_url("https://x/a.laz"));
        assert!(is_shard_url("https://x/a.las"));
        assert!(is_shard_url("https://x/a.las.zip"));
        assert!(is_shard_url("https://x/lidar-vergara.zip"));
        assert!(!is_shard_url("https://x/report.pdf"));
        assert!(!is_shard_url("https://x/a.laz.json"));
        assert!(!is_shard_url("https://x/index.html"));
        assert!(!is_shard_url(
            "http://qldspatial.information.qld.gov.au/catalogue/custom/search.page?q=%22LiDAR%22"
        ));
    }

    #[test]
    fn search_url_carries_the_page_and_an_encoded_query() {
        let url = search_url("https://data.cnra.ca.gov", "lidar point cloud", 100, 200);
        assert!(url.contains("/api/3/action/package_search?q=lidar%20point%20cloud"));
        assert!(url.contains("rows=100"));
        assert!(url.contains("start=200"));
    }

    #[test]
    fn a_success_false_answer_stays_incomplete() {
        let body = r#"{"success":false,"error":{"__type":"NotFound"}}"#;
        assert!(parse_page(body).is_err());
    }

    #[test]
    fn emit_ids_writes_resource_identity_and_absent_fields_render_as_dash() {
        let page = parse_page(PAGE).expect("page parses");
        let lines = id_lines(&page.resources);
        assert_eq!(lines.len(), 1);
        assert_eq!(
            lines[0],
            "DMP-000723 Clear Lake Lidar\t2026-05-22T16:48:49.966846\thttps://data.cnra.ca.gov/dataset/x/resource/z/download/dmp-000723_clear_lake_lidar.zip"
        );
        let bare = parse_page(
            r#"{"success":true,"result":{"count":1,"results":[{"resources":[{"url":"https://x/a.laz"}]}]}}"#,
        )
        .expect("bare page parses");
        assert_eq!(
            id_lines(&bare.resources),
            vec!["-\t-\thttps://x/a.laz".to_string()]
        );
    }

    #[test]
    fn default_portal_names_the_measured_cnra_ckan_home() {
        assert_eq!(DEFAULT_PORTAL, "https://data.cnra.ca.gov");
        assert_eq!(
            netloc_of(DEFAULT_PORTAL).as_deref(),
            Some("data.cnra.ca.gov")
        );
    }
}
