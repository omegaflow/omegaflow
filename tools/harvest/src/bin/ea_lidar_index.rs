use omegaflow::archivar::{RetryPolicy, TRANSFER_BOUND_S, fetch_raw_with};
use omegaflow::cdn::{CDN_BASE, upload_release};
use std::env;
use std::fs;
use std::path::Path;

const MANIFEST_NAME: &str = "ea_lidar_index.manifest";
const COMPILER: &str = "tools/harvest/src/bin/ea_lidar_index.rs";
const ORIGIN: &str = "https://environment.data.gov.uk";
const PATH: &str = "/geoservices/datasets/9f0fa3fc-a860-4729-adc9-47fe53f658d0/ogc/features/v1/collections/LIDAR_Point_Cloud_Index_Catalogue/items";
const DEFAULT_LIMIT: usize = 1000;

enum Json {
    Lit,
    Num(String),
    Str(String),
    Arr(Vec<Json>),
    Obj(Vec<(String, Json)>),
}

struct Parser<'a> {
    b: &'a [u8],
    i: usize,
}

impl<'a> Parser<'a> {
    fn peek(&self) -> Option<u8> {
        self.b.get(self.i).copied()
    }

    fn ws(&mut self) {
        while let Some(c) = self.peek() {
            match c {
                b' ' | b'\t' | b'\n' | b'\r' => self.i += 1,
                _ => break,
            }
        }
    }

    fn expect_lit(&mut self, lit: &str) -> Option<()> {
        if self.b.get(self.i..self.i + lit.len())? == lit.as_bytes() {
            self.i += lit.len();
            Some(())
        } else {
            None
        }
    }

    fn value(&mut self) -> Option<Json> {
        self.ws();
        match self.peek()? {
            b'{' => self.object(),
            b'[' => self.array(),
            b'"' => self.string().map(Json::Str),
            b't' => match self.expect_lit("true") {
                Some(()) => Some(Json::Lit),
                None => None,
            },
            b'f' => match self.expect_lit("false") {
                Some(()) => Some(Json::Lit),
                None => None,
            },
            b'n' => match self.expect_lit("null") {
                Some(()) => Some(Json::Lit),
                None => None,
            },
            _ => self.number().map(Json::Num),
        }
    }

    fn object(&mut self) -> Option<Json> {
        self.i += 1;
        let mut out = Vec::new();
        self.ws();
        if self.peek() == Some(b'}') {
            self.i += 1;
            return Some(Json::Obj(out));
        }
        loop {
            self.ws();
            let key = self.string()?;
            self.ws();
            if self.peek() != Some(b':') {
                return None;
            }
            self.i += 1;
            let val = self.value()?;
            out.push((key, val));
            self.ws();
            match self.peek() {
                Some(b',') => self.i += 1,
                Some(b'}') => {
                    self.i += 1;
                    break;
                }
                _ => return None,
            }
        }
        Some(Json::Obj(out))
    }

    fn array(&mut self) -> Option<Json> {
        self.i += 1;
        let mut out = Vec::new();
        self.ws();
        if self.peek() == Some(b']') {
            self.i += 1;
            return Some(Json::Arr(out));
        }
        loop {
            let val = self.value()?;
            out.push(val);
            self.ws();
            match self.peek() {
                Some(b',') => self.i += 1,
                Some(b']') => {
                    self.i += 1;
                    break;
                }
                _ => return None,
            }
        }
        Some(Json::Arr(out))
    }

    fn string(&mut self) -> Option<String> {
        if self.peek() != Some(b'"') {
            return None;
        }
        self.i += 1;
        let mut out = String::new();
        loop {
            let c = self.peek()?;
            if c == b'"' {
                self.i += 1;
                break;
            }
            if c == b'\\' {
                self.i += 1;
                let e = self.peek()?;
                self.i += 1;
                match e {
                    b'"' => out.push('"'),
                    b'\\' => out.push('\\'),
                    b'/' => out.push('/'),
                    b'b' => out.push('\u{0008}'),
                    b'f' => out.push('\u{000C}'),
                    b'n' => out.push('\n'),
                    b'r' => out.push('\r'),
                    b't' => out.push('\t'),
                    b'u' => {
                        let hex = self.b.get(self.i..self.i + 4)?;
                        let h = std::str::from_utf8(hex).ok()?;
                        let code = u32::from_str_radix(h, 16).ok()?;
                        self.i += 4;
                        if let Some(ch) = char::from_u32(code) {
                            out.push(ch);
                        }
                    }
                    _ => out.push(e as char),
                }
                continue;
            }
            if c < 0x80 {
                out.push(c as char);
                self.i += 1;
                continue;
            }
            let len = match c {
                0xC0..=0xDF => 2,
                0xE0..=0xEF => 3,
                0xF0..=0xF7 => 4,
                _ => 1,
            };
            let end = self.i + len;
            let slice = self.b.get(self.i..end)?;
            out.push_str(std::str::from_utf8(slice).ok()?);
            self.i = end;
        }
        Some(out)
    }

    fn number(&mut self) -> Option<String> {
        let start = self.i;
        while let Some(c) = self.peek() {
            match c {
                b'-' | b'+' | b'0'..=b'9' | b'.' | b'e' | b'E' => self.i += 1,
                _ => break,
            }
        }
        if self.i == start {
            return None;
        }
        Some(
            std::str::from_utf8(&self.b[start..self.i])
                .ok()?
                .to_string(),
        )
    }
}

fn parse_json(s: &str) -> Option<Json> {
    let mut p = Parser {
        b: s.as_bytes(),
        i: 0,
    };
    p.ws();
    p.value()
}

fn obj_get<'a>(j: &'a Json, key: &str) -> Option<&'a Json> {
    if let Json::Obj(m) = j {
        for (k, v) in m {
            if k.as_str() == key {
                return Some(v);
            }
        }
    }
    None
}

fn obj_str(j: &Json, key: &str) -> Option<String> {
    match obj_get(j, key) {
        Some(Json::Str(s)) => Some(s.clone()),
        _ => None,
    }
}

fn number_usize(j: &Json) -> Option<usize> {
    if let Json::Num(raw) = j {
        if let Ok(v) = raw.parse::<u64>() {
            return Some(v as usize);
        }
    }
    None
}

fn finite_token(j: &Json) -> Option<String> {
    if let Json::Num(raw) = j {
        match raw.parse::<f64>() {
            Ok(v) if v.is_finite() => return Some(raw.clone()),
            _ => return None,
        }
    }
    None
}

fn parse_bbox(j: &Json) -> Option<[String; 4]> {
    if let Json::Arr(items) = j {
        if items.len() == 4 {
            let mut out = [String::new(), String::new(), String::new(), String::new()];
            for (k, item) in items.iter().enumerate() {
                let token = finite_token(item)?;
                out[k] = token;
            }
            return Some(out);
        }
    }
    None
}

fn tile_line(f: &Json) -> Option<String> {
    let props = obj_get(f, "properties")?;
    let filename = obj_str(props, "filename")?;
    if filename.is_empty() {
        return None;
    }
    let bbox = parse_bbox(obj_get(f, "bbox")?)?;
    let pt = finite_token(obj_get(props, "pt_count")?)?;
    Some(format!(
        "tile {filename} {} {} {} {} {pt}",
        bbox[0], bbox[1], bbox[2], bbox[3]
    ))
}

fn manifest_lines(features: &[Json]) -> Vec<String> {
    let mut out = Vec::new();
    for f in features {
        if let Some(line) = tile_line(f) {
            out.push(line);
        }
    }
    out
}

struct Page {
    number_returned: usize,
    next_href: Option<String>,
    tiles: Vec<String>,
}

fn page_of(body: &str) -> Option<Page> {
    let root = parse_json(body)?;
    let mut number_returned = 0usize;
    if let Some(n) = obj_get(&root, "numberReturned").and_then(number_usize) {
        number_returned = n;
    }
    let tiles = match obj_get(&root, "features") {
        Some(Json::Arr(a)) => {
            if number_returned == 0 {
                number_returned = a.len();
            }
            manifest_lines(a)
        }
        _ => Vec::new(),
    };
    let mut next_href = None;
    if let Some(Json::Arr(links)) = obj_get(&root, "links") {
        for l in links {
            if obj_str(l, "rel").as_deref() == Some("next") {
                if let Some(h) = obj_str(l, "href") {
                    next_href = Some(h);
                    break;
                }
            }
        }
    }
    Some(Page {
        number_returned,
        next_href,
        tiles,
    })
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn fetch_page(url: &str) -> Option<String> {
    fetch_raw_with(url, None, &[], RetryPolicy::All, TRANSFER_BOUND_S)
}

fn page_url(limit: usize) -> String {
    format!("{ORIGIN}{PATH}?limit={limit}&f=json")
}

fn resolve_href(base: &str, href: &str) -> String {
    if href.starts_with("http://") || href.starts_with("https://") {
        return href.to_string();
    }
    if let Some(rest) = href.strip_prefix('/') {
        return format!("{ORIGIN}/{rest}");
    }
    let mut b = base.to_string();
    if let Some(q) = b.find('?') {
        b.truncate(q);
    }
    if let Some(p) = b.rfind('/') {
        b.truncate(p + 1);
    }
    b.push_str(href);
    b
}

fn run(args: &[String]) -> Result<(), String> {
    let limit = match arg_value(args, "--limit").and_then(|v| v.parse::<usize>().ok()) {
        Some(n) if n > 0 => n,
        _ => DEFAULT_LIMIT,
    };
    let max_pages = match arg_value(args, "--max-pages").and_then(|v| v.parse::<usize>().ok()) {
        Some(n) if n > 0 => Some(n),
        _ => None,
    };
    let out = match arg_value(args, "--out").filter(|o| !o.is_empty()) {
        Some(o) => o,
        None => format!("data/environment.data.gov.uk/{MANIFEST_NAME}"),
    };
    let out_name = match Path::new(&out).file_name().and_then(|n| n.to_str()) {
        Some(n) => n.to_string(),
        None => MANIFEST_NAME.to_string(),
    };
    let ci_mode = args.iter().any(|a| a == "--ci-mode");

    let mut url = page_url(limit);
    let mut pages = 0usize;
    let mut lines: Vec<String> = Vec::new();
    loop {
        if let Some(m) = max_pages {
            if pages >= m {
                break;
            }
        }
        let body = fetch_page(&url).ok_or_else(|| {
            format!(
                "page {} returned void ({url}) — the walk stays incomplete",
                pages + 1
            )
        })?;
        let page = page_of(&body).ok_or_else(|| {
            format!(
                "page {} did not read as a FeatureCollection ({url}) — the walk stays incomplete",
                pages + 1
            )
        })?;
        pages += 1;
        eprintln!(
            "ea_lidar_index: page {pages} carried {} feature(s), {} tile line(s) — the walk continues",
            page.number_returned,
            page.tiles.len()
        );
        lines.extend(page.tiles);
        match page.next_href {
            Some(h) if page.number_returned > 0 => {
                url = resolve_href(&url, &h);
            }
            _ => break,
        }
    }

    if lines.is_empty() {
        return Err(format!(
            "the catalogue walk over {pages} page(s) carried no tile — nothing manifestiert (0 honored)"
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
        "ea_lidar_index: {pages} page(s), {} tile line(s) -> {out}",
        lines.len()
    );

    println!("url {CDN_BASE}/environment.data.gov.uk/{out_name}");
    println!("origin {ORIGIN}{PATH}");
    println!("compiler {COMPILER}");
    println!("format ea_lidar_index");

    if ci_mode && !upload_release("environment.data.gov.uk", &out) {
        return Err(format!("{out} did not reach the CDN"));
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("ea_lidar_index: {msg}");
        std::process::exit(2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PAGE: &str = r#"{"type":"FeatureCollection","numberMatched":244324,"numberReturned":3,"features":[{"id":"LIDAR_Point_Cloud_Index_Catalogue.1","bbox":[0.529889,52.966211,0.541508,52.984397],"properties":{"filename":"TF7044_P_11243_20171107_20171107.laz","filepath":".\\LAZ_Point_Archive\\...laz","pt_count":1218591,"year":"2017","os_ref":"TF7044","polygon_id":"P_11243_11244"}},{"id":"LIDAR_Point_Cloud_Index_Catalogue.2","properties":{"filename":"NO_BBOX.laz","pt_count":5}},{"id":"LIDAR_Point_Cloud_Index_Catalogue.3","bbox":[1.0,2.0,3.0,4.0],"properties":{"pt_count":7}}],"links":[{"rel":"next","href":"https://environment.data.gov.uk/geoservices/datasets/9f0fa3fc-a860-4729-adc9-47fe53f658d0/ogc/features/v1/collections/LIDAR_Point_Cloud_Index_Catalogue/items?limit=1000&f=json&startIndex=1000"}]}"#;

    #[test]
    fn measured_page_becomes_tile_lines_and_skips_incomplete_features() {
        let page = match page_of(PAGE) {
            Some(p) => p,
            None => panic!("the measured page reads"),
        };
        assert_eq!(page.number_returned, 3);
        assert_eq!(page.tiles.len(), 1);
        assert_eq!(
            page.tiles[0],
            "tile TF7044_P_11243_20171107_20171107.laz 0.529889 52.966211 0.541508 52.984397 1218591"
        );
        assert_eq!(
            page.next_href.as_deref(),
            Some(
                "https://environment.data.gov.uk/geoservices/datasets/9f0fa3fc-a860-4729-adc9-47fe53f658d0/ogc/features/v1/collections/LIDAR_Point_Cloud_Index_Catalogue/items?limit=1000&f=json&startIndex=1000"
            )
        );
    }

    #[test]
    fn a_nameless_feature_stays_out() {
        let body = r#"{"numberReturned":1,"features":[{"id":"x","bbox":[1.0,2.0,3.0,4.0],"properties":{"pt_count":7}}]}"#;
        let page = match page_of(body) {
            Some(p) => p,
            None => panic!("the page reads"),
        };
        assert!(page.tiles.is_empty());
    }

    #[test]
    fn a_bbox_of_three_stays_out() {
        let body = r#"{"numberReturned":1,"features":[{"id":"x","bbox":[1.0,2.0,3.0],"properties":{"filename":"short.laz","pt_count":7}}]}"#;
        let page = match page_of(body) {
            Some(p) => p,
            None => panic!("the page reads"),
        };
        assert!(page.tiles.is_empty());
    }
}
