use omegaflow::archivar::{RetryPolicy, TRANSFER_BOUND_S, fetch_raw_with};
use omegaflow::cdn::{CDN_BASE, upload_release};
use std::env;
use std::fs;
use std::path::Path;

const COMPILER: &str = "tools/harvest/src/bin/coverage.rs";
const DEFAULT_COLLECTION: &str = "3dep-lidar-copc";
const DEFAULT_ROOT: &str = "https://planetarycomputer.microsoft.com/api/stac/v1";
const TNM_ROOT: &str = "https://tnmaccess.nationalmap.gov/api/v1/products";
const TNM_DATASET: &str = "Lidar Point Cloud (LPC)";
const MANIFEST_NAME: &str = "coverage.manifest";
const PAGE_LIMIT: usize = 1000;
const MAX_PAGES: usize = 5000;

#[derive(Debug)]
enum Json {
    Null,
    Bool,
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
                Ok(Json::Bool)
            }
            Some(b'f') => {
                self.lit("false")?;
                Ok(Json::Bool)
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

    fn as_arr(&self) -> Option<&[Json]> {
        match self {
            Json::Arr(a) => Some(a),
            _ => None,
        }
    }
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn stac_fetch(url: &str) -> Option<String> {
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

fn is_copc_href(href: &str) -> bool {
    let lower = href.to_ascii_lowercase();
    let path = lower.split(['?', '#']).next().unwrap_or(&lower);
    path.ends_with(".copc.laz")
}

fn copc_href(feature: &Json) -> Option<String> {
    let assets = match feature.get("assets") {
        Some(Json::Obj(a)) => a,
        _ => return None,
    };
    for (key, asset) in assets {
        if let Some(href) = asset.get("href").and_then(Json::as_str) {
            if is_copc_href(href) || is_copc_href(key) {
                return Some(href.to_string());
            }
        }
    }
    None
}

fn bbox_of(feature: &Json) -> Option<[f64; 6]> {
    let arr = feature.get("bbox")?.as_arr()?;
    if arr.len() != 6 {
        return None;
    }
    let mut out = [0f64; 6];
    for (slot, value) in out.iter_mut().zip(arr.iter()) {
        let n = value.as_f64()?;
        if !n.is_finite() {
            return None;
        }
        *slot = n;
    }
    Some(out)
}

struct Tile {
    href: String,
    bbox: Option<[f64; 6]>,
    id: Option<String>,
    epoch: Option<String>,
}

struct StacPage {
    tiles: Vec<Tile>,
    next: Option<String>,
}

fn parse_items(body: &str) -> Result<StacPage, String> {
    let root = parse_json(body)?;
    let mut tiles = Vec::new();
    if let Some(features) = root.get("features").and_then(Json::as_arr) {
        for feature in features {
            if let Some(href) = copc_href(feature) {
                let id = feature
                    .get("id")
                    .and_then(Json::as_str)
                    .map(|s| s.to_string());
                let epoch = feature
                    .get("properties")
                    .and_then(|p| p.get("datetime"))
                    .and_then(Json::as_str)
                    .map(|s| s.to_string());
                tiles.push(Tile {
                    href,
                    bbox: bbox_of(feature),
                    id,
                    epoch,
                });
            }
        }
    }
    let next = root
        .get("links")
        .and_then(Json::as_arr)
        .and_then(|links| {
            links
                .iter()
                .find(|l| l.get("rel").and_then(Json::as_str) == Some("next"))
        })
        .and_then(|l| l.get("href").and_then(Json::as_str))
        .map(|s| s.to_string());
    Ok(StacPage { tiles, next })
}

fn items_url(root: &str, collection: &str, limit: usize) -> String {
    format!(
        "{}/collections/{collection}/items?limit={limit}",
        root.trim_end_matches('/')
    )
}

fn enumerate(
    root: &str,
    collection: &str,
    limit: usize,
    max_pages: usize,
) -> Result<(Vec<Tile>, usize), String> {
    let mut tiles = Vec::new();
    let mut pages = 0usize;
    let mut url = items_url(root, collection, limit);
    loop {
        let body = stac_fetch(&url).ok_or_else(|| {
            format!(
                "STAC page {} returned void ({url}) — the walk stays incomplete",
                pages + 1
            )
        })?;
        let page = parse_items(&body)?;
        pages += 1;
        tiles.extend(page.tiles);
        match page.next {
            Some(next) if next.is_empty() => break,
            Some(next) if next == url => {
                return Err(format!(
                    "STAC page {pages} names its own url as next ({url}) — the walk stays incomplete"
                ));
            }
            Some(next) => url = next,
            None => break,
        }
        if pages >= max_pages {
            return Err(format!(
                "STAC walk reached the page bound {max_pages} — the walk stays incomplete"
            ));
        }
    }
    Ok((tiles, pages))
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

fn tnm_items_url(root: &str, dataset: &str, max: usize, offset: usize) -> String {
    format!(
        "{}?datasets={}&outputFormat=JSON&max={max}&offset={offset}",
        root.trim_end_matches('/'),
        encode_query(dataset)
    )
}

fn tnm_id(item: &Json) -> Option<String> {
    for key in ["id", "sourceId", "title"] {
        if let Some(value) = item
            .get(key)
            .and_then(Json::as_str)
            .filter(|s| !s.is_empty())
        {
            return Some(value.to_string());
        }
    }
    None
}

fn tnm_laz_href(item: &Json) -> Option<String> {
    for key in ["downloadURL", "downloadLazURL"] {
        if let Some(href) = item
            .get(key)
            .and_then(Json::as_str)
            .filter(|h| h.starts_with("http"))
        {
            return Some(href.to_string());
        }
    }
    if let Some(urls) = item.get("urls").and_then(Json::as_arr) {
        for entry in urls {
            let href = entry
                .as_str()
                .or_else(|| entry.get("url").and_then(Json::as_str));
            if let Some(href) = href.filter(|h| h.starts_with("http")) {
                return Some(href.to_string());
            }
        }
    }
    None
}

fn parse_tnm_page(body: &str) -> Result<(Vec<Tile>, usize, Option<usize>), String> {
    let root = parse_json(body)?;
    let mut tiles = Vec::new();
    let mut raw = 0usize;
    if let Some(items) = root.get("items").and_then(Json::as_arr) {
        raw = items.len();
        for item in items {
            if let Some(href) = tnm_laz_href(item) {
                tiles.push(Tile {
                    href,
                    bbox: None,
                    id: tnm_id(item),
                    epoch: item
                        .get("publicationDate")
                        .and_then(Json::as_str)
                        .map(|s| s.to_string()),
                });
            }
        }
    }
    let total = root.get("total").and_then(Json::as_f64).and_then(|t| {
        if t.is_finite() && t >= 0.0 {
            Some(t as usize)
        } else {
            None
        }
    });
    Ok((tiles, raw, total))
}

fn tnm_enumerate(
    root: &str,
    dataset: &str,
    max: usize,
    max_pages: usize,
) -> Result<(Vec<Tile>, usize, String), String> {
    let first = tnm_items_url(root, dataset, max, 0);
    let mut tiles = Vec::new();
    let mut pages = 0usize;
    let mut offset = 0usize;
    loop {
        let url = tnm_items_url(root, dataset, max, offset);
        let body = stac_fetch(&url).ok_or_else(|| {
            format!(
                "TNM page {} returned void ({url}) — the walk stays incomplete",
                pages + 1
            )
        })?;
        let (page, raw, total) = parse_tnm_page(&body)?;
        pages += 1;
        tiles.extend(page);
        offset += raw;
        if raw < max {
            break;
        }
        if let Some(total) = total {
            if offset >= total {
                break;
            }
        }
        if pages >= max_pages {
            return Err(format!(
                "TNM walk reached the page bound {max_pages} — the walk stays incomplete"
            ));
        }
    }
    Ok((tiles, pages, first))
}

fn manifest_lines(tiles: &[Tile], emit_bbox: bool) -> Vec<String> {
    let mut out = Vec::with_capacity(tiles.len());
    for t in tiles {
        match t.bbox {
            Some(b) if emit_bbox && b.iter().all(|v| v.is_finite()) => out.push(format!(
                "shard {} {} {} {} {} {} {} EPSG:4326",
                t.href, b[0], b[1], b[2], b[3], b[4], b[5]
            )),
            _ => out.push(format!("shard {}", t.href)),
        }
    }
    out
}

fn id_lines(tiles: &[Tile]) -> Vec<String> {
    let mut out = Vec::with_capacity(tiles.len());
    for t in tiles {
        let id = t.id.as_deref().unwrap_or("-");
        let epoch = t.epoch.as_deref().unwrap_or("-");
        out.push(format!("{id}\t{epoch}\t{}", t.href));
    }
    out
}

fn run(args: &[String]) -> Result<(), String> {
    if args.iter().any(|a| a == "--help") {
        eprintln!(
            "usage: coverage --kind <stac|tnm> [--collection <id>] [--url <root>] [--out <manifest>] [--limit <n>] [--jobs <n>] [--bbox|--bbox-off] [--emit-ids <path>] [--ci-mode]  (default: url-only shard lines). stac: <stac-root>/collections/<id>/items, a next-link chain, page bound {MAX_PAGES}; tnm: https://tnmaccess.nationalmap.gov/api/v1/products?datasets=Lidar%20Point%20Cloud%20(LPC), an offset walk (max {PAGE_LIMIT}), page bound {MAX_PAGES}. --bbox emits the STAC bbox; the TNM bbox stays absent (lon/lat degrees, no ICRS transform), so tnm is url-only; --emit-ids writes a tile-id sidecar"
        );
        return Ok(());
    }
    let kind = match arg_value(args, "--kind").filter(|k| !k.is_empty()) {
        Some(k) => k,
        None => return Err("--kind absent — no arm selected".to_string()),
    };
    let tnm = match kind.as_str() {
        "stac" => false,
        "tnm" => true,
        other => {
            return Err(format!(
                "--kind {other}: this build carries the stac and tnm arms"
            ));
        }
    };
    let collection = if tnm {
        String::new()
    } else {
        match arg_value(args, "--collection").filter(|c| !c.is_empty()) {
            Some(c) => c,
            None => DEFAULT_COLLECTION.to_string(),
        }
    };
    if !tnm
        && (collection.contains('/')
            || collection.contains('?')
            || collection.contains(char::is_whitespace))
    {
        return Err(format!(
            "--collection {collection}: not a collection id — refused"
        ));
    }
    let root = match arg_value(args, "--url").filter(|u| !u.is_empty()) {
        Some(u) => u,
        None if tnm => TNM_ROOT.to_string(),
        None => DEFAULT_ROOT.to_string(),
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
    let emit_ids = arg_value(args, "--emit-ids").filter(|p| !p.is_empty());
    let emit_bbox = args.iter().any(|a| a == "--bbox") && !args.iter().any(|a| a == "--bbox-off");
    let limit = match arg_value(args, "--limit").and_then(|v| v.parse::<usize>().ok()) {
        Some(n) if n > 0 => n,
        _ => PAGE_LIMIT,
    };
    let jobs = match arg_value(args, "--jobs").and_then(|v| v.parse::<usize>().ok()) {
        Some(n) if n > 0 => n,
        _ => 1,
    };
    if jobs > 1 {
        eprintln!(
            "coverage: the {kind} walk is a single-cursor chain — {jobs} workers collapse to one"
        );
    }

    let (mut tiles, pages, origin_url) = if tnm {
        tnm_enumerate(&root, TNM_DATASET, limit, MAX_PAGES)?
    } else {
        let origin = items_url(&root, &collection, limit);
        let (tiles, pages) = enumerate(&root, &collection, limit, MAX_PAGES)?;
        (tiles, pages, origin)
    };
    if pages == 0 || tiles.is_empty() {
        return Err(format!(
            "{kind}: the walk carries no shard — nothing manifestiert (0 honored)"
        ));
    }
    tiles.sort_by(|a, b| a.href.cmp(&b.href));
    tiles.dedup_by(|a, b| a.href == b.href);
    let lines = manifest_lines(&tiles, emit_bbox);

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
        "coverage: {kind}: {pages} pages, {} tiles -> {out}",
        tiles.len()
    );

    if let Some(path) = emit_ids.as_deref() {
        let id_lines = id_lines(&tiles);
        let mut id_text = String::with_capacity(id_lines.len() * 128);
        for line in &id_lines {
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
        eprintln!("coverage: {} tile ids -> {path}", id_lines.len());
    }
    if ci_mode {
        if let Some(path) = emit_ids.as_deref() {
            if !upload_release(&netloc, path) {
                return Err(format!("{path} did not reach the CDN"));
            }
        }
    }

    println!("url {CDN_BASE}/{netloc}/{out_name}");
    println!("origin {origin_url}");
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
        eprintln!("coverage: {msg}");
        std::process::exit(2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PAGE: &str = r#"{"type":"FeatureCollection","features":[
{"type":"Feature","id":"tile-a","bbox":[-105.1,39.9,1600.0,-105.0,40.0,1700.0],"assets":{"data":{"href":"https://storage.example/tile-a.copc.laz","type":"application/vnd.laz"},"thumbnail":{"href":"https://storage.example/tile-a.png"}}},
{"type":"Feature","id":"tile-b","assets":{"data":{"href":"https://storage.example/tile-b.copc.laz"}}},
{"type":"Feature","id":"tile-c","bbox":[-106.0,39.0,-105.0,40.0],"assets":{"data":{"href":"https://storage.example/tile-c.copc.laz"}}}
],"links":[{"rel":"self","href":"https://pc/api/stac/v1/collections/3dep-lidar-copc/items"},{"rel":"next","href":"https://pc/api/stac/v1/collections/3dep-lidar-copc/items?token=abc"}]}"#;

    #[test]
    fn bbox_tile_emits_six_floats_absent_bbox_emits_the_bare_url() {
        let page = parse_items(PAGE).expect("page parses");
        assert_eq!(
            page.next.as_deref(),
            Some("https://pc/api/stac/v1/collections/3dep-lidar-copc/items?token=abc")
        );
        let lines = manifest_lines(&page.tiles, true);
        assert_eq!(lines.len(), 3);
        assert_eq!(
            lines[0],
            "shard https://storage.example/tile-a.copc.laz -105.1 39.9 1600 -105 40 1700"
        );
        assert_eq!(lines[1], "shard https://storage.example/tile-b.copc.laz");
        assert_eq!(lines[2], "shard https://storage.example/tile-c.copc.laz");
    }

    #[test]
    fn default_manifest_is_url_only_even_with_a_bbox() {
        let page = parse_items(PAGE).expect("page parses");
        let lines = manifest_lines(&page.tiles, false);
        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0], "shard https://storage.example/tile-a.copc.laz");
        assert_eq!(lines[1], "shard https://storage.example/tile-b.copc.laz");
        assert_eq!(lines[2], "shard https://storage.example/tile-c.copc.laz");
    }

    #[test]
    fn a_four_float_bbox_is_absent_never_fabricated() {
        let feature = parse_json(r#"{"bbox":[-106.0,39.0,-105.0,40.0]}"#).expect("parses");
        assert!(bbox_of(&feature).is_none());
    }

    #[test]
    fn netloc_reads_the_host_from_a_stac_root() {
        assert_eq!(
            netloc_of("https://planetarycomputer.microsoft.com/api/stac/v1").as_deref(),
            Some("planetarycomputer.microsoft.com")
        );
        assert_eq!(netloc_of("ftp://x/y"), None);
    }

    #[test]
    fn emit_ids_writes_tile_identity_and_absent_fields_render_as_dash() {
        let body = r#"{"type":"FeatureCollection","features":[
{"type":"Feature","id":"tile-a","properties":{"datetime":"2021-05-01T00:00:00Z"},"assets":{"data":{"href":"https://storage.example/tile-a.copc.laz"}}},
{"type":"Feature","assets":{"data":{"href":"https://storage.example/tile-b.copc.laz"}}}
]}"#;
        let page = parse_items(body).expect("page parses");
        let lines = id_lines(&page.tiles);
        assert_eq!(lines.len(), 2);
        assert_eq!(
            lines[0],
            "tile-a\t2021-05-01T00:00:00Z\thttps://storage.example/tile-a.copc.laz"
        );
        assert_eq!(lines[1], "-\t-\thttps://storage.example/tile-b.copc.laz");
    }

    #[test]
    fn tnm_page_keeps_the_laz_url_and_skips_an_item_without_one() {
        let body = r#"{"total":9755359,"items":[
{"title":"MD_AnneArundel_2018","publicationDate":"2018-05-01","downloadURL":"https://rockyweb.usgs.gov/vdelivery/Datasets/Staged/Elevation/LPC/Projects/MD_AnneArundel_2018/tile.laz","boundingBox":{"minX":-76.6,"maxX":-76.4,"minY":38.9,"maxY":39.1}},
{"title":"no-laz-here","publicationDate":"2019-01-01","boundingBox":{"minX":-77.0,"maxX":-76.9,"minY":39.0,"maxY":39.1}}
]}"#;
        let (tiles, raw, total) = parse_tnm_page(body).expect("page parses");
        assert_eq!(raw, 2);
        assert_eq!(total, Some(9755359));
        assert_eq!(tiles.len(), 1);
        assert_eq!(
            tiles[0].href,
            "https://rockyweb.usgs.gov/vdelivery/Datasets/Staged/Elevation/LPC/Projects/MD_AnneArundel_2018/tile.laz"
        );
        assert_eq!(tiles[0].id.as_deref(), Some("MD_AnneArundel_2018"));
        assert_eq!(tiles[0].epoch.as_deref(), Some("2018-05-01"));
        assert_eq!(
            manifest_lines(&tiles, true),
            vec![
                "shard https://rockyweb.usgs.gov/vdelivery/Datasets/Staged/Elevation/LPC/Projects/MD_AnneArundel_2018/tile.laz".to_string()
            ]
        );
    }
}
