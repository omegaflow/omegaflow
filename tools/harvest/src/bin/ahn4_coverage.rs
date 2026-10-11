use omegaflow::archivar::{RetryPolicy, TRANSFER_BOUND_S, fetch_raw_with};
use omegaflow::cdn::{CDN_BASE, upload_release};
use std::env;
use std::fs;
use std::path::Path;

const MANIFEST_NAME: &str = "ahn4_coverage.manifest";
const COMPILER: &str = "tools/harvest/src/bin/ahn4_coverage.rs";
const NETLOC: &str = "ns_hwh.fundaments.nl";
const FEATURE_SERVER: &str = "https://services.arcgis.com/nSZVuSZjHpEZZbRo/arcgis/rest/services/Kaartbladen_AHN4/FeatureServer/0";
const LAZ_FIELD: &str = "AHN4_LAZ";
const ID_FIELD: &str = "Name";
const PAGE_LIMIT: usize = 2000;
const MAX_PAGES: usize = 100;

#[derive(Debug)]
enum Json {
    Null,
    Bool(bool),
    Num(String),
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
            .map(|_| Json::Num(raw.to_string()))
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

    fn as_num_str(&self) -> Option<&str> {
        match self {
            Json::Num(s) => Some(s),
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

struct Tile {
    url: String,
    id: Option<String>,
    epoch: Option<String>,
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

fn is_lidar_url(url: &str) -> bool {
    let lower = url.to_ascii_lowercase();
    let path = lower.split(['?', '#']).next().unwrap_or(&lower);
    path.ends_with(".laz") || path.ends_with(".las") || path.ends_with(".copc.laz")
}

fn tile_id(attributes: &Json) -> Option<String> {
    if let Some(name) = attributes.get(ID_FIELD).and_then(Json::as_str) {
        return Some(name.to_string());
    }
    attributes
        .get("OBJECTID")
        .and_then(Json::as_num_str)
        .map(|s| s.to_string())
}

fn parse_tiles(body: &str) -> Result<(Vec<Tile>, bool), String> {
    let root = parse_json(body)?;
    let features = root.get("features").and_then(Json::as_arr).ok_or_else(|| {
        "listing carries no features array — the walk stays incomplete".to_string()
    })?;
    let mut tiles = Vec::new();
    for feature in features {
        let Some(attributes) = feature.get("attributes") else {
            continue;
        };
        if let Some(url) = attributes.get(LAZ_FIELD).and_then(Json::as_str) {
            tiles.push(Tile {
                url: url.to_string(),
                id: tile_id(attributes),
                epoch: None,
            });
        }
    }
    let more = matches!(root.get("exceededTransferLimit"), Some(Json::Bool(true)));
    Ok((tiles, more))
}

fn query_url(base: &str, offset: usize) -> String {
    format!(
        "{base}/query?where=1%3D1&outFields={ID_FIELD},{LAZ_FIELD}&returnGeometry=false&resultRecordCount={PAGE_LIMIT}&resultOffset={offset}&f=json"
    )
}

fn enumerate(base: &str) -> Result<(Vec<Tile>, usize), String> {
    let mut tiles = Vec::new();
    let mut pages = 0usize;
    let mut offset = 0usize;
    loop {
        let url = query_url(base, offset);
        let body = listing_fetch(&url).ok_or_else(|| {
            format!(
                "listing page {} returned void ({url}) — the walk stays incomplete",
                pages + 1
            )
        })?;
        let (page_tiles, more) = parse_tiles(&body)?;
        pages += 1;
        if page_tiles.is_empty() && more {
            return Err(format!(
                "page {pages} carries no tile but names more — the walk stays incomplete"
            ));
        }
        tiles.extend(page_tiles);
        if !more {
            break;
        }
        offset += PAGE_LIMIT;
        if pages >= MAX_PAGES {
            return Err(format!(
                "listing walk exceeded {MAX_PAGES} pages — the walk stays incomplete"
            ));
        }
    }
    Ok((tiles, pages))
}

fn shard_url(base: &str, url: &str) -> String {
    if url.starts_with("http://") || url.starts_with("https://") {
        url.to_string()
    } else {
        format!(
            "{}/{}",
            base.trim_end_matches('/'),
            url.trim_start_matches('/')
        )
    }
}

fn manifest_lines(base: &str, tiles: &[Tile]) -> Vec<String> {
    let mut out = Vec::new();
    for tile in tiles {
        if !is_lidar_url(&tile.url) {
            continue;
        }
        out.push(format!("shard {}", shard_url(base, &tile.url)));
    }
    out
}

fn id_lines(base: &str, tiles: &[Tile]) -> Vec<String> {
    let mut out = Vec::new();
    for tile in tiles {
        if !is_lidar_url(&tile.url) {
            continue;
        }
        let id = tile.id.as_deref().unwrap_or("-");
        let epoch = tile.epoch.as_deref().unwrap_or("-");
        out.push(format!("{id}\t{epoch}\t{}", shard_url(base, &tile.url)));
    }
    out
}

fn run(args: &[String]) -> Result<(), String> {
    if args.iter().any(|a| a == "--help") {
        eprintln!(
            "usage: ahn4_coverage [--base <feature-server-layer>] [--out <manifest>] [--emit-ids <path>] [--ci-mode]  (default: url-only shard lines; --emit-ids writes a tile-id sidecar)"
        );
        return Ok(());
    }
    let base = match arg_value(args, "--base").filter(|b| !b.is_empty()) {
        Some(b) => b,
        None => FEATURE_SERVER.to_string(),
    };
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

    let (tiles, pages) = enumerate(&base)?;
    if pages == 0 || tiles.is_empty() {
        return Err(format!(
            "{base}: the listing carries no tile — nothing manifestiert (0 honored)"
        ));
    }
    let lines = manifest_lines(&format!("https://{NETLOC}"), &tiles);
    if lines.is_empty() {
        return Err(format!(
            "{base}: {} tiles over {pages} pages, none a lidar url — nothing manifestiert (0 honored)",
            tiles.len()
        ));
    }

    let mut text = String::with_capacity(lines.len() * 96);
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
        "ahn4_coverage: {pages} pages, {} tiles, {} lidar urls -> {out}",
        tiles.len(),
        lines.len()
    );

    if let Some(path) = emit_ids.as_deref() {
        let id_lines = id_lines(&format!("https://{NETLOC}"), &tiles);
        let mut id_text = String::with_capacity(id_lines.len() * 112);
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
        eprintln!("ahn4_coverage: {} tile ids -> {path}", id_lines.len());
    }
    if ci_mode {
        if let Some(path) = emit_ids.as_deref() {
            if !upload_release(NETLOC, path) {
                return Err(format!("{path} did not reach the CDN"));
            }
        }
    }

    println!("url {CDN_BASE}/{NETLOC}/{out_name}");
    println!("origin {base}/query?where=1%3D1&outFields={ID_FIELD},{LAZ_FIELD}");
    println!("compiler {COMPILER}");
    println!("format ahn4_coverage");

    if ci_mode && !upload_release(NETLOC, &out) {
        return Err(format!("{out} did not reach the CDN"));
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("ahn4_coverage: {msg}");
        std::process::exit(2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PAGE: &str = r#"{"exceededTransferLimit":true,"features":[{"attributes":{"OBJECTID":1,"Name":"01CZ1","AHN4_DTM_05m":"https://ns_hwh.fundaments.nl/hwh-ahn/ahn4/02a_DTM_0.5m/M_01CZ1.zip","AHN4_DTM_5m":"https://ns_hwh.fundaments.nl/hwh-ahn/ahn4/02b_DTM_5m/M5_01CZ1.zip","AHN4_DSM_05m":"https://ns_hwh.fundaments.nl/hwh-ahn/ahn4/03a_DSM_0.5m/R_01CZ1.zip","AHN4_DSM_5m":"https://ns_hwh.fundaments.nl/hwh-ahn/ahn4/03b_DSM_5m/R5_01CZ1.zip","AHN4_LAZ":"https://ns_hwh.fundaments.nl/hwh-ahn/ahn4/01_LAZ/C_01CZ1.LAZ","Shape__Area":31250000,"Shape__Length":22500}},{"attributes":{"OBJECTID":2,"Name":"01CZ2","AHN4_DTM_05m":"https://ns_hwh.fundaments.nl/hwh-ahn/ahn4/02a_DTM_0.5m/M_01CZ2.zip","AHN4_DTM_5m":"https://ns_hwh.fundaments.nl/hwh-ahn/ahn4/02b_DTM_5m/M5_01CZ2.zip","AHN4_DSM_05m":"https://ns_hwh.fundaments.nl/hwh-ahn/ahn4/03a_DSM_0.5m/R_01CZ2.zip","AHN4_DSM_5m":"https://ns_hwh.fundaments.nl/hwh-ahn/ahn4/03b_DSM_5m/R5_01CZ2.zip","AHN4_LAZ":"https://ns_hwh.fundaments.nl/hwh-ahn/ahn4/01_LAZ/C_01CZ2.LAZ","Shape__Area":31250000,"Shape__Length":22500}}]}"#;

    const LAST_PAGE: &str = r#"{"features":[{"attributes":{"OBJECTID":3,"Name":"01DZ1","AHN4_LAZ":"https://ns_hwh.fundaments.nl/hwh-ahn/ahn4/01_LAZ/C_01DZ1.LAZ"}}]}"#;

    #[test]
    fn measured_page_becomes_shard_lines_on_an_absolute_url() {
        let (tiles, more) = parse_tiles(PAGE).expect("measured page parses");
        assert!(more);
        assert_eq!(tiles.len(), 2);
        let lines = manifest_lines("https://ns_hwh.fundaments.nl", &tiles);
        assert_eq!(lines.len(), 2);
        assert_eq!(
            lines[0],
            "shard https://ns_hwh.fundaments.nl/hwh-ahn/ahn4/01_LAZ/C_01CZ1.LAZ"
        );
        assert_eq!(
            lines[1],
            "shard https://ns_hwh.fundaments.nl/hwh-ahn/ahn4/01_LAZ/C_01CZ2.LAZ"
        );
    }

    #[test]
    fn the_last_page_carries_no_transfer_overflow() {
        let (tiles, more) = parse_tiles(LAST_PAGE).expect("last page parses");
        assert!(!more);
        assert_eq!(tiles.len(), 1);
        assert_eq!(
            manifest_lines("https://ns_hwh.fundaments.nl", &tiles)[0],
            "shard https://ns_hwh.fundaments.nl/hwh-ahn/ahn4/01_LAZ/C_01DZ1.LAZ"
        );
    }

    #[test]
    fn non_lidar_urls_are_left_out() {
        let body = r#"{"features":[{"attributes":{"OBJECTID":9,"Name":"01CZ9","AHN4_DTM_05m":"https://ns_hwh.fundaments.nl/hwh-ahn/ahn4/02a_DTM_0.5m/M_01CZ9.zip","AHN4_DSM_05m":"https://ns_hwh.fundaments.nl/hwh-ahn/ahn4/03a_DSM_0.5m/R_01CZ9.zip"}}]}"#;
        let (tiles, more) = parse_tiles(body).expect("feature parses");
        assert!(!more);
        assert!(tiles.is_empty());
        assert!(manifest_lines("https://ns_hwh.fundaments.nl", &tiles).is_empty());
    }

    #[test]
    fn the_query_url_carries_the_offset_and_the_laz_field() {
        let url = query_url(FEATURE_SERVER, 2000);
        assert!(url.starts_with(FEATURE_SERVER));
        assert!(url.contains("outFields=Name,AHN4_LAZ"));
        assert!(url.contains("resultOffset=2000"));
        assert!(url.contains("returnGeometry=false"));
    }

    #[test]
    fn emit_ids_writes_the_tile_name_and_absent_fields_render_as_dash() {
        let (tiles, _) = parse_tiles(PAGE).expect("measured page parses");
        let lines = id_lines("https://ns_hwh.fundaments.nl", &tiles);
        assert_eq!(lines.len(), 2);
        assert_eq!(
            lines[0],
            "01CZ1\t-\thttps://ns_hwh.fundaments.nl/hwh-ahn/ahn4/01_LAZ/C_01CZ1.LAZ"
        );
        let body = r#"{"features":[{"attributes":{"AHN4_LAZ":"https://ns_hwh.fundaments.nl/hwh-ahn/ahn4/01_LAZ/C_01DZ9.LAZ"}}]}"#;
        let (tiles, _) = parse_tiles(body).expect("feature parses");
        assert_eq!(
            id_lines("https://ns_hwh.fundaments.nl", &tiles)[0],
            "-\t-\thttps://ns_hwh.fundaments.nl/hwh-ahn/ahn4/01_LAZ/C_01DZ9.LAZ"
        );
    }
}
