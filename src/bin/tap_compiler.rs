// TAP-Kompilator: Katalog-Inventare (tap_schema.tables) und Bulk-Fetches
// (ADQL, FORMAT=json, Paging) → Flat-cmap-JSON für den cmap+dist_scale/z-Kanal.
// Muster: ephemeris_compiler --index/--fetch-from. Erster Einsatz: TAPVizieR.
// upload via --ci-mode (tag ssd.jpl.nasa.gov).

use omegaflow::cdn::upload_asset;
use std::io::Write;
use std::process::Command;

#[derive(Debug)]
enum Json {
    Null,
    Num(f64),
    Str(String),
    Arr(Vec<Json>),
    Obj(std::collections::HashMap<String, Json>),
}

struct Jp<'a> {
    b: &'a [u8],
    i: usize,
}

impl<'a> Jp<'a> {
    fn ws(&mut self) {
        while self.i < self.b.len() && (self.b[self.i] as char).is_ascii_whitespace() {
            self.i += 1;
        }
    }
    fn val(&mut self) -> Option<Json> {
        self.ws();
        match self.b.get(self.i).copied() {
            Some(b'n') => {
                if self.b[self.i..].starts_with(b"null") {
                    self.i += 4;
                    Some(Json::Null)
                } else {
                    None
                }
            }
            Some(b'"') => self.string().map(Json::Str),
            Some(b'[') => {
                self.i += 1;
                let mut v = Vec::new();
                self.ws();
                if self.b.get(self.i) == Some(&b']') {
                    self.i += 1;
                    return Some(Json::Arr(v));
                }
                loop {
                    v.push(self.val()?);
                    self.ws();
                    match self.b.get(self.i) {
                        Some(b',') => self.i += 1,
                        Some(b']') => {
                            self.i += 1;
                            return Some(Json::Arr(v));
                        }
                        _ => return None,
                    }
                }
            }
            Some(b'{') => {
                self.i += 1;
                let mut m = std::collections::HashMap::new();
                self.ws();
                if self.b.get(self.i) == Some(&b'}') {
                    self.i += 1;
                    return Some(Json::Obj(m));
                }
                loop {
                    self.ws();
                    let k = self.string()?;
                    self.ws();
                    if self.b.get(self.i) != Some(&b':') {
                        return None;
                    }
                    self.i += 1;
                    let v = self.val()?;
                    m.insert(k, v);
                    self.ws();
                    match self.b.get(self.i) {
                        Some(b',') => self.i += 1,
                        Some(b'}') => {
                            self.i += 1;
                            return Some(Json::Obj(m));
                        }
                        _ => return None,
                    }
                }
            }
            Some(_) => {
                let rest = &self.b[self.i..];
                let end = rest
                    .iter()
                    .position(|&c| c == b',' || c == b']' || c == b'}' || c.is_ascii_whitespace())
                    .unwrap_or(rest.len());
                let s = std::str::from_utf8(&rest[..end]).ok()?;
                if end == 0 {
                    return None;
                }
                self.i += end;
                s.trim().parse::<f64>().ok().map(Json::Num)
            }
            None => None,
        }
    }
    fn string(&mut self) -> Option<String> {
        if self.b.get(self.i) != Some(&b'"') {
            return None;
        }
        self.i += 1;
        let mut s = String::new();
        loop {
            let c = self.b.get(self.i).copied()?;
            self.i += 1;
            match c {
                b'"' => return Some(s),
                b'\\' => {
                    let e = self.b.get(self.i).copied()?;
                    self.i += 1;
                    match e {
                        b'n' => s.push('\n'),
                        b't' => s.push('\t'),
                        b'r' => s.push('\r'),
                        b'"' => s.push('"'),
                        b'\\' => s.push('\\'),
                        _ => return None,
                    }
                }
                _ => s.push(c as char),
            }
        }
    }
}

fn parse_json(s: &str) -> Option<Json> {
    let mut p = Jp {
        b: s.as_bytes(),
        i: 0,
    };
    p.val()
}

fn tap_query(root: &str, adql: &str) -> Option<String> {
    let out = Command::new("curl")
        .arg("-sSf")
        .arg("-G")
        .arg("--data-urlencode")
        .arg("REQUEST=doQuery")
        .arg("--data-urlencode")
        .arg("LANG=ADQL")
        .arg("--data-urlencode")
        .arg("FORMAT=json")
        .arg("--data-urlencode")
        .arg(format!("QUERY={}", adql))
        .arg(root)
        .output()
        .ok()?;
    if out.status.success() {
        String::from_utf8(out.stdout).ok()
    } else {
        None
    }
}

fn json_string(s: &str) -> String {
    let mut o = String::from("\"");
    for c in s.chars() {
        match c {
            '"' => o.push_str("\\\""),
            '\\' => o.push_str("\\\\"),
            _ => o.push(c),
        }
    }
    o.push('"');
    o
}

fn get_str(o: &std::collections::HashMap<String, Json>, k: &str) -> Option<String> {
    match o.get(k) {
        Some(Json::Str(s)) => Some(s.clone()),
        Some(Json::Num(v)) => Some(format!("{}", v)),
        _ => None,
    }
}

fn as_arr(j: &Json) -> Option<&Vec<Json>> {
    match j {
        Json::Arr(a) => Some(a),
        _ => None,
    }
}

fn as_obj(j: &Json) -> Option<&std::collections::HashMap<String, Json>> {
    match j {
        Json::Obj(m) => Some(m),
        _ => None,
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut root: Option<String> = None;
    let mut out: Option<String> = None;
    let mut table: Option<String> = None;
    let mut columns: Option<String> = None;
    let mut ci_mode = false;
    let mut probe: Option<String> = None;
    let mut limit: usize = 50_000;
    let mut index_mode = false;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--root" => {
                root = args.get(i + 1).cloned();
                i += 1;
            }
            "--out" => {
                out = args.get(i + 1).cloned();
                i += 1;
            }
            "--table" => {
                table = args.get(i + 1).cloned();
                i += 1;
            }
            "--columns" => {
                columns = args.get(i + 1).cloned();
                i += 1;
            }
            "--limit" => {
                limit = args
                    .get(i + 1)
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(50_000);
                i += 1;
            }
            "--ci-mode" => ci_mode = true,
            "--probe" => {
                probe = args.get(i + 1).cloned();
                i += 1;
            }
            "--index" => index_mode = true,
            _ => {}
        }
        i += 1;
    }
    let root = match root {
        Some(r) => r,
        None => {
            eprintln!("--root absent");
            std::process::exit(1);
        }
    };
    if index_mode {
        let adql = "SELECT table_name, table_type, schema_name, description FROM tap_schema.tables";
        let Some(body) = tap_query(&root, adql) else {
            eprintln!("index query returned void");
            std::process::exit(1);
        };
        let Some(Json::Obj(m)) = parse_json(&body) else {
            eprintln!("index json returned void");
            std::process::exit(1);
        };
        let Some(cols) = m.get("data").and_then(as_arr) else {
            eprintln!("index data absent");
            std::process::exit(1);
        };
        let out_path = out.unwrap_or_else(|| "tap_index.φ".to_string());
        let mut buf = String::new();
        buf.push_str(&format!("# tap-inventar {}\n", root));
        let mut n = 0usize;
        for row in cols {
            let Some(r) = as_arr(row) else { continue };
            if r.len() < 4 {
                continue;
            }
            buf.push_str(&format!(
                "catalog {} {} {}\n",
                row_str(&r[0]),
                row_str(&r[2]),
                row_str(&r[1])
            ));
            n += 1;
        }
        if let Ok(mut f) = std::fs::File::create(&out_path) {
            let _ = f.write_all(buf.as_bytes());
        }
        eprintln!("tap index: {} tables → {}", n, out_path);
        return;
    }
    let table = match table {
        Some(t) => t,
        None => {
            eprintln!("--table absent");
            std::process::exit(1);
        }
    };
    let columns = match columns {
        Some(c) => c,
        None => {
            eprintln!("--columns absent (mapping: name:ColA;ra:ColB;...)");
            std::process::exit(1);
        }
    };
    let mapping: Vec<(String, String)> = columns
        .split(';')
        .filter(|s| !s.is_empty())
        .filter_map(|s| {
            let mut it = s.splitn(2, ':');
            Some((it.next()?.trim().to_string(), it.next()?.trim().to_string()))
        })
        .collect();
    if mapping.is_empty() {
        eprintln!("--columns empty");
        std::process::exit(1);
    }
    let mut col_names: Vec<String> = Vec::new();
    let col_idx: Vec<(String, usize)>;
    let mut rows_out: Vec<String> = Vec::new();
    let any_positional = mapping.iter().any(|(_, c)| c.starts_with('@'));
    let cols_sel = if any_positional {
        "*".to_string()
    } else {
        mapping
            .iter()
            .map(|(_, c)| format!("\"{}\"", c))
            .collect::<Vec<_>>()
            .join(",")
    };
    let adql = format!("SELECT TOP {} {} FROM \"{}\"", limit, cols_sel, table);
    let Some(body) = tap_query(&root, &adql) else {
        eprintln!("query returned void");
        std::process::exit(1);
    };
    let Some(Json::Obj(m)) = parse_json(&body) else {
        eprintln!("json returned void");
        std::process::exit(1);
    };
    let Some(meta) = m.get("metadata").and_then(as_arr) else {
        eprintln!("metadata absent");
        std::process::exit(1);
    };
    for md in meta {
        if let Some(o) = as_obj(md) {
            if let Some(nm) = get_str(o, "name") {
                col_names.push(nm);
            }
        }
    }
    let idx_of = |col: &str| -> Option<usize> {
        if let Some(n) = col.strip_prefix('@') {
            n.parse::<usize>().ok()
        } else {
            col_names.iter().position(|c| c == col)
        }
    };
    col_idx = mapping
        .iter()
        .filter_map(|(k, c)| idx_of(c).map(|i| (k.clone(), i)))
        .collect();
    if col_idx.len() != mapping.len() {
        for (k, c) in &mapping {
            if idx_of(c).is_none() {
                eprintln!("column absent: {} ({}), available: {:?}", c, k, col_names);
            }
        }
        std::process::exit(1);
    }
    let Some(data) = m.get("data").and_then(as_arr) else {
        eprintln!("data absent");
        std::process::exit(1);
    };
    for row in data {
        let Some(r) = as_arr(row) else { continue };
        let mut obj = String::from("{");
        for (pos, (k, i)) in col_idx.iter().enumerate() {
            if pos > 0 {
                obj.push(',');
            }
            let cell = match r.get(*i) {
                Some(Json::Str(s)) => json_string(s),
                Some(Json::Num(v)) => format!("{}", v),
                _ => "null".to_string(),
            };
            obj.push_str(&format!("\"{}\":{}", k, cell));
        }
        obj.push('}');
        rows_out.push(obj);
    }
    eprintln!(
        "tap fetch: {} columns, {} rows",
        col_names.len(),
        rows_out.len()
    );
    if let Some(name) = &probe {
        for r in &rows_out {
            if r.contains(&format!("\"name\":\"{}", name)) || r.contains(name) {
                eprintln!("probe: {}", r);
                return;
            }
        }
        eprintln!("probe: {} not present", name);
        return;
    }
    let out_path = match out {
        Some(p) => p,
        None => {
            eprintln!("--out absent");
            std::process::exit(1);
        }
    };
    let mut buf = String::from("[");
    for (k, r) in rows_out.iter().enumerate() {
        if k > 0 {
            buf.push(',');
        }
        buf.push_str(r);
    }
    buf.push_str("]\n");
    if let Ok(mut f) = std::fs::File::create(&out_path) {
        let _ = f.write_all(buf.as_bytes());
    } else {
        eprintln!("write {} returned void", out_path);
        std::process::exit(1);
    }
    eprintln!(
        "tap: {} rows → {} ({} B)",
        rows_out.len(),
        out_path,
        buf.len()
    );
    if ci_mode {
        let _ = upload_asset(&out_path);
    }
}

fn row_str(j: &Json) -> String {
    match j {
        Json::Str(s) => s.trim_matches('"').to_string(),
        Json::Num(v) => format!("{}", v),
        _ => String::new(),
    }
}
