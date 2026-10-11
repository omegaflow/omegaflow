use omegaflow::archivar::{RetryPolicy, TRANSFER_BOUND_S, fetch_raw_with};
use omegaflow::cdn::{CDN_BASE, upload_release};
use std::env;
use std::fs;
use std::path::Path;

const MANIFEST_NAME: &str = "swisstopo_coverage.manifest";
const COMPILER: &str = "tools/harvest/src/bin/swisstopo_coverage.rs";
const NETLOC: &str = "data.geo.admin.ch";
const STAC_BASE: &str = "https://data.geo.admin.ch/api/stac/v0.9";
const DEFAULT_COLLECTION: &str = "ch.swisstopo.swisssurface3d";
const PAGE_LIMIT: usize = 1000;
const MAX_PAGES: usize = 5000;

#[derive(Debug)]
enum Json {
    Null,
    Bool,
    Num,
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
            .map(|_| Json::Num)
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

fn is_lidar_href(href: &str) -> bool {
    let lower = href.to_ascii_lowercase();
    let path = lower.split(['?', '#']).next().unwrap_or(&lower);
    path.ends_with(".laz")
        || path.ends_with(".las")
        || path.ends_with(".copc.laz")
        || path.ends_with(".las.zip")
        || path.ends_with(".laz.zip")
}

struct Asset {
    href: String,
    id: Option<String>,
    epoch: Option<String>,
}

struct StacPage {
    hrefs: Vec<Asset>,
    next: Option<String>,
}

fn collect_assets(node: &Json, out: &mut Vec<Asset>) {
    let id = node.get("id").and_then(Json::as_str).map(|s| s.to_string());
    let epoch = node
        .get("properties")
        .and_then(|p| p.get("datetime"))
        .and_then(Json::as_str)
        .map(|s| s.to_string());
    if let Some(Json::Obj(assets)) = node.get("assets") {
        for (_, asset) in assets {
            if let Some(href) = asset.get("href").and_then(Json::as_str) {
                out.push(Asset {
                    href: href.to_string(),
                    id: id.clone(),
                    epoch: epoch.clone(),
                });
            }
        }
    }
}

fn parse_items(body: &str) -> Result<StacPage, String> {
    let root = parse_json(body)?;
    let mut hrefs = Vec::new();
    match root.get("features").and_then(Json::as_arr) {
        Some(features) => {
            for feature in features {
                collect_assets(feature, &mut hrefs);
            }
        }
        None => collect_assets(&root, &mut hrefs),
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
    Ok(StacPage { hrefs, next })
}

fn items_url(base: &str, collection: &str, filter: &str) -> String {
    format!("{base}/collections/{collection}/items?limit={PAGE_LIMIT}{filter}")
}

fn enumerate(
    base: &str,
    collection: &str,
    max_pages: usize,
    filter: &str,
) -> Result<(Vec<Asset>, usize), String> {
    let mut hrefs = Vec::new();
    let mut pages = 0usize;
    let mut url = items_url(base, collection, filter);
    loop {
        let body = stac_fetch(&url).ok_or_else(|| {
            format!(
                "STAC page {} returned void ({url}) — the walk stays incomplete",
                pages + 1
            )
        })?;
        let page = parse_items(&body)?;
        pages += 1;
        hrefs.extend(page.hrefs);
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
    Ok((hrefs, pages))
}

fn shard_href(base: &str, href: &str) -> String {
    if href.starts_with("http://") || href.starts_with("https://") {
        href.to_string()
    } else {
        format!(
            "{}/{}",
            base.trim_end_matches('/'),
            href.trim_start_matches('/')
        )
    }
}

fn manifest_lines(base: &str, hrefs: &[Asset]) -> Vec<String> {
    let mut out = Vec::new();
    for asset in hrefs {
        if !is_lidar_href(&asset.href) {
            continue;
        }
        out.push(format!("shard {}", shard_href(base, &asset.href)));
    }
    out
}

fn id_lines(base: &str, hrefs: &[Asset]) -> Vec<String> {
    let mut out = Vec::new();
    for asset in hrefs {
        if !is_lidar_href(&asset.href) {
            continue;
        }
        let id = asset.id.as_deref().unwrap_or("-");
        let epoch = asset.epoch.as_deref().unwrap_or("-");
        out.push(format!("{id}\t{epoch}\t{}", shard_href(base, &asset.href)));
    }
    out
}

fn run(args: &[String]) -> Result<(), String> {
    if args.iter().any(|a| a == "--help") {
        eprintln!(
            "usage: swisstopo_coverage [--base <stac-root>] [--collection <id>] [--out <manifest>] [--bbox <w,s,e,n>] [--datetime <range>] [--max-pages <n>] [--emit-ids <path>] [--ci-mode]  (default: url-only shard lines; --emit-ids writes a tile-id sidecar)"
        );
        return Ok(());
    }
    let base = match arg_value(args, "--base").filter(|b| !b.is_empty()) {
        Some(b) => b,
        None => STAC_BASE.to_string(),
    };
    let collection = match arg_value(args, "--collection").filter(|c| !c.is_empty()) {
        Some(c) => c,
        None => DEFAULT_COLLECTION.to_string(),
    };
    if collection.contains('/')
        || collection.contains('?')
        || collection.contains(char::is_whitespace)
    {
        return Err(format!(
            "--collection {collection}: not a collection id — refused"
        ));
    }
    let out = match arg_value(args, "--out").filter(|o| !o.is_empty()) {
        Some(o) => o,
        None => format!("data/{NETLOC}/{MANIFEST_NAME}"),
    };
    let out_name = match Path::new(&out).file_name().and_then(|n| n.to_str()) {
        Some(n) => n.to_string(),
        None => MANIFEST_NAME.to_string(),
    };
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let emit_ids = arg_value(args, "--emit-ids").filter(|p| !p.is_empty());
    let max_pages = match arg_value(args, "--max-pages").and_then(|v| v.parse::<usize>().ok()) {
        Some(p) if p > 0 => p,
        _ => MAX_PAGES,
    };
    let mut filter = String::new();
    if let Some(bbox) = arg_value(args, "--bbox").filter(|b| !b.is_empty()) {
        filter.push_str("&bbox=");
        filter.push_str(&bbox);
    }
    if let Some(dt) = arg_value(args, "--datetime").filter(|d| !d.is_empty()) {
        filter.push_str("&datetime=");
        filter.push_str(&dt);
    }

    let (hrefs, pages) = enumerate(&base, &collection, max_pages, &filter)?;
    if pages == 0 || hrefs.is_empty() {
        return Err(format!(
            "{collection}: the STAC walk carries no asset — nothing manifestiert (0 honored)"
        ));
    }
    let lines = manifest_lines(&base, &hrefs);
    if lines.is_empty() {
        return Err(format!(
            "{collection}: {} assets over {pages} pages, none a lidar tile — nothing manifestiert (0 honored)",
            hrefs.len()
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
    eprintln!(
        "swisstopo_coverage: {collection}: {pages} pages, {} assets, {} lidar tiles -> {out}",
        hrefs.len(),
        lines.len()
    );

    if let Some(path) = emit_ids.as_deref() {
        let id_lines = id_lines(&base, &hrefs);
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
        eprintln!("swisstopo_coverage: {} tile ids -> {path}", id_lines.len());
    }

    println!("url {CDN_BASE}/{NETLOC}/{out_name}");
    println!("origin {base}/collections/{collection}/items");
    println!("compiler {COMPILER}");
    println!("format swisstopo_coverage");

    if ci_mode && !upload_release(NETLOC, &out) {
        return Err(format!("{out} did not reach the CDN"));
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("swisstopo_coverage: {msg}");
        std::process::exit(2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ITEM: &str = r#"{"id":"swisssurface3d_2015_2494-1140","collection":"ch.swisstopo.swisssurface3d","type":"Feature","stac_version":"0.9.0","geometry":{"type":"Polygon","coordinates":[[[6.0726649,46.4120458],[6.0729899,46.4120498],[6.0729884,46.4121127],[6.0726633,46.4121088],[6.0726649,46.4120458]]]},"bbox":[6.0726633,46.4120458,6.0729899,46.4121127],"properties":{"datetime":"2015-01-01T00:00:00Z","created":"2021-02-23T13:20:01.041996Z","updated":"2025-01-17T08:23:08.549422Z"},"links":[{"rel":"self","href":"https://data.geo.admin.ch/api/stac/v0.9/collections/ch.swisstopo.swisssurface3d/items/swisssurface3d_2015_2494-1140"},{"rel":"root","href":"https://data.geo.admin.ch/api/stac/v0.9/"},{"rel":"parent","href":"https://data.geo.admin.ch/api/stac/v0.9/collections/ch.swisstopo.swisssurface3d"},{"rel":"collection","href":"https://data.geo.admin.ch/api/stac/v0.9/collections/ch.swisstopo.swisssurface3d"},{"rel":"alternate","title":"STAC Browser","type":"text/html","href":"https://data.geo.admin.ch/browser/index.html#/collections/ch.swisstopo.swisssurface3d/items/swisssurface3d_2015_2494-1140"}],"assets":{"swisssurface3d_2015_2494-1140_2056_5728.las.zip":{"type":"application/vnd.laszip","href":"https://data.geo.admin.ch/ch.swisstopo.swisssurface3d/swisssurface3d_2015_2494-1140/swisssurface3d_2015_2494-1140_2056_5728.las.zip","created":"2021-02-23T13:20:02.844784Z","updated":"2025-01-17T08:23:08.549422Z","proj:epsg":2056,"checksum:multihash":"12208AC3BC5EBD82ADE0CCE09F5367B57E1AA9A91AEFB425B29F383DDC3809B8D288"}}}"#;

    const PAGE: &str = r#"{"type":"FeatureCollection","timeStamp":"2026-10-11T07:23:10.953938Z","features":[{"id":"swisssurface3d_2015_2494-1140","collection":"ch.swisstopo.swisssurface3d","type":"Feature","assets":{"swisssurface3d_2015_2494-1140_2056_5728.las.zip":{"type":"application/vnd.laszip","href":"https://data.geo.admin.ch/ch.swisstopo.swisssurface3d/swisssurface3d_2015_2494-1140/swisssurface3d_2015_2494-1140_2056_5728.las.zip"}}},{"id":"swisssurface3d_2017_2682-1206","collection":"ch.swisstopo.swisssurface3d","type":"Feature","assets":{"swisssurface3d_2017_2682-1206_2056_5728.copc.laz":{"type":"application/vnd.laszip","href":"https://data.geo.admin.ch/ch.swisstopo.swisssurface3d/swisssurface3d_2017_2682-1206/swisssurface3d_2017_2682-1206_2056_5728.copc.laz"},"swisssurface3d_2017_2682-1206_2056_5728.ept.json":{"type":"application/json","href":"https://data.geo.admin.ch/ch.swisstopo.swisssurface3d/swisssurface3d_2017_2682-1206/swisssurface3d_2017_2682-1206_2056_5728.ept.json"}}}],"links":[{"rel":"self","href":"https://data.geo.admin.ch/api/stac/v0.9/collections/ch.swisstopo.swisssurface3d/items"},{"rel":"next","href":"https://data.geo.admin.ch/api/stac/v0.9/collections/ch.swisstopo.swisssurface3d/items?cursor=cD1zd2lzc3VyZmFjZTNkXzIwMTdfMjY4Mi0xMjA2%3D%3D&limit=1000"}]}"#;

    #[test]
    fn measured_item_asset_becomes_a_shard_line() {
        let page = parse_items(ITEM).expect("measured item parses");
        assert_eq!(page.hrefs.len(), 1);
        assert!(page.next.is_none());
        let lines = manifest_lines(STAC_BASE, &page.hrefs);
        assert_eq!(lines.len(), 1);
        assert_eq!(
            lines[0],
            "shard https://data.geo.admin.ch/ch.swisstopo.swisssurface3d/swisssurface3d_2015_2494-1140/swisssurface3d_2015_2494-1140_2056_5728.las.zip"
        );
    }

    #[test]
    fn collection_page_carries_lidar_assets_and_the_next_cursor() {
        let page = parse_items(PAGE).expect("measured page parses");
        assert_eq!(page.hrefs.len(), 3);
        assert_eq!(
            page.next.as_deref(),
            Some(
                "https://data.geo.admin.ch/api/stac/v0.9/collections/ch.swisstopo.swisssurface3d/items?cursor=cD1zd2lzc3VyZmFjZTNkXzIwMTdfMjY4Mi0xMjA2%3D%3D&limit=1000"
            )
        );
        let lines = manifest_lines(STAC_BASE, &page.hrefs);
        assert_eq!(lines.len(), 2);
        assert_eq!(
            lines[1],
            "shard https://data.geo.admin.ch/ch.swisstopo.swisssurface3d/swisssurface3d_2017_2682-1206/swisssurface3d_2017_2682-1206_2056_5728.copc.laz"
        );
    }

    #[test]
    fn non_lidar_assets_are_left_out() {
        let body = r#"{"type":"Feature","assets":{"a_2056_1.ept.json":{"type":"application/json","href":"https://data.geo.admin.ch/x/a_2056_1.ept.json"},"b_2056_1.copc.json":{"type":"application/json","href":"https://data.geo.admin.ch/x/b_2056_1.copc.json"}}}"#;
        let page = parse_items(body).expect("feature parses");
        assert_eq!(page.hrefs.len(), 2);
        let lines = manifest_lines(STAC_BASE, &page.hrefs);
        assert!(lines.is_empty());
    }

    #[test]
    fn emit_ids_writes_item_identity_and_absent_fields_render_as_dash() {
        let body = r#"{"type":"FeatureCollection","features":[
{"id":"swisssurface3d_2015_2494-1140","properties":{"datetime":"2015-01-01T00:00:00Z"},"assets":{"data":{"href":"https://data.geo.admin.ch/ch.swisstopo.swisssurface3d/swisssurface3d_2015_2494-1140/swisssurface3d_2015_2494-1140_2056_5728.copc.laz"}}},
{"assets":{"data":{"href":"https://data.geo.admin.ch/ch.swisstopo.swisssurface3d/x/x_2056_1.copc.laz"}}}
]}"#;
        let page = parse_items(body).expect("page parses");
        let lines = id_lines(STAC_BASE, &page.hrefs);
        assert_eq!(lines.len(), 2);
        assert_eq!(
            lines[0],
            "swisssurface3d_2015_2494-1140\t2015-01-01T00:00:00Z\thttps://data.geo.admin.ch/ch.swisstopo.swisssurface3d/swisssurface3d_2015_2494-1140/swisssurface3d_2015_2494-1140_2056_5728.copc.laz"
        );
        assert_eq!(
            lines[1],
            "-\t-\thttps://data.geo.admin.ch/ch.swisstopo.swisssurface3d/x/x_2056_1.copc.laz"
        );
    }
}
