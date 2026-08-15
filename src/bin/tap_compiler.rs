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
                        b'u' => {
                            let h = std::str::from_utf8(&self.b[self.i..self.i + 4]).ok()?;
                            self.i += 4;
                            s.push(char::from_u32(u32::from_str_radix(h, 16).ok()?)?);
                        }
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

fn tap_query_votable(root: &str, adql: &str) -> Option<String> {
    let out = Command::new("curl")
        .arg("-sSf")
        .arg("-G")
        .arg("--data-urlencode")
        .arg("REQUEST=doQuery")
        .arg("--data-urlencode")
        .arg("LANG=ADQL")
        .arg("--data-urlencode")
        .arg("FORMAT=votable/td")
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

fn tap_async(root: &str, adql: &str, poll_secs: u64) -> Option<String> {
    let base = root.replace("/tap/sync", "/tap/async");
    let out = Command::new("curl")
        .arg("-sS")
        .arg("-D")
        .arg("-")
        .arg("-o")
        .arg("/dev/null")
        .arg("-X")
        .arg("POST")
        .arg("--data-urlencode")
        .arg("REQUEST=doQuery")
        .arg("--data-urlencode")
        .arg("LANG=ADQL")
        .arg("--data-urlencode")
        .arg(format!("QUERY={}", adql))
        .arg(&base)
        .output()
        .ok()?;
    let headers = String::from_utf8_lossy(&out.stdout);
    let job = headers
        .lines()
        .find(|l| l.to_lowercase().starts_with("location:"))
        .map(|l| l["location:".len()..].trim().to_string())?;
    eprintln!("uws job: {}", job);
    let mut phase = String::new();
    for _ in 0..(poll_secs / 10 + 1) {
        phase = Command::new("curl")
            .arg("-sS")
            .arg(format!("{}/phase", job))
            .output()
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())?
            .trim()
            .to_string();
        if phase == "COMPLETED" {
            break;
        }
        if phase == "ERROR" {
            eprintln!("uws job ERROR");
            return None;
        }
        std::thread::sleep(std::time::Duration::from_secs(10));
    }
    if phase != "COMPLETED" {
        eprintln!("uws job phase: {} nach {} s", phase, poll_secs);
        return None;
    }
    Command::new("curl")
        .arg("-sS")
        .arg("-m")
        .arg("3600")
        .arg(format!("{}/results/result", job))
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
}

fn votable_rows(body: &str) -> Option<(Vec<String>, Vec<Vec<String>>)> {
    let mut fields: Vec<String> = Vec::new();
    for f in body.split("<FIELD").skip(1) {
        let seg = match f.split_once('>') {
            Some((s, _)) => s,
            None => continue,
        };
        let name = if let Some(attr) = seg.split("name=\"").nth(1) {
            attr.split_once('"')?.0.trim().to_string()
        } else {
            let inner = match f.split_once('>') {
                Some((_, rest)) => match rest.split_once('<') {
                    Some((i, _)) => i.trim().to_string(),
                    None => continue,
                },
                None => continue,
            };
            if inner.is_empty() {
                continue;
            }
            inner
        };
        fields.push(name);
    }
    let data = match body.split("<DATA>").nth(1) {
        Some(d) => d,
        None => {
            eprintln!(
                "votable: <DATA> absent, felder={} len={}",
                fields.len(),
                body.len()
            );
            return None;
        }
    };
    let mut rows = Vec::new();
    for tr in data.split("<TR>").skip(1) {
        let end = match tr.split_once("</TR>") {
            Some((e, _)) => e,
            None => continue,
        };
        let mut cells = Vec::new();
        for td in end.split("<TD>").skip(1) {
            let raw = match td.split_once("</TD>") {
                Some((r, _)) => r.trim(),
                None => continue,
            };
            let v = if let Some(c) = raw.strip_prefix("<![CDATA[") {
                c.strip_suffix("]]>").unwrap_or(c).trim().to_string()
            } else {
                raw.to_string()
            };
            cells.push(v);
        }
        rows.push(cells);
    }
    eprintln!("votable: {} fields, {} rows", fields.len(), rows.len());
    Some((fields, rows))
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
    let mut async_mode: Option<u64> = None;
    let mut epoch_prop: Option<f64> = None;
    let mut votable_flag = false;
    let mut order_by: Option<String> = None;
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
            "--async" => {
                async_mode = args.get(i + 1).and_then(|s| s.parse().ok()).or(Some(600));
                i += 1;
            }
            "--epoch" => {
                epoch_prop = args.get(i + 1).and_then(|s| s.parse().ok());
                i += 1;
            }
            "--votable" => votable_flag = true,
            "--order" => {
                order_by = args.get(i + 1).cloned();
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
    let table_ref = if table.contains('/') {
        format!("\"{}\"", table)
    } else {
        table.clone()
    };
    let adql = if async_mode.is_some() {
        format!("SELECT {} FROM {}", cols_sel, table_ref)
    } else {
        let mut q = format!("SELECT TOP {} {} FROM {}", limit, cols_sel, table_ref);
        if let Some(o) = &order_by {
            q.push_str(&format!(" ORDER BY \"{}\"", o));
        }
        q
    };
    let mut cells_rows: Vec<Vec<String>> = Vec::new();
    if async_mode.is_some() {
        let Some(poll) = async_mode else {
            unreachable!()
        };
        let Some(body) = tap_async(&root, &adql, poll) else {
            eprintln!("async returned void");
            std::process::exit(1);
        };
        let Some((fields, rows)) = votable_rows(&body) else {
            eprintln!("votable returned void");
            std::process::exit(1);
        };
        col_names = fields;
        cells_rows = rows;
    } else if votable_flag {
        let Some(body) = tap_query_votable(&root, &adql) else {
            eprintln!("query returned void");
            std::process::exit(1);
        };
        match votable_rows(&body) {
            Some((fields, rows)) => {
                col_names = fields;
                cells_rows = rows;
            }
            None => {
                eprintln!(
                    "votable returned void, body[:160] = {}",
                    &body[..body.len().min(160)]
                );
                std::process::exit(1);
            }
        }
    } else {
        let Some(body) = tap_query(&root, &adql) else {
            eprintln!("query returned void");
            std::process::exit(1);
        };
        let Some(Json::Obj(m)) = parse_json(&body) else {
            eprintln!("json returned void");
            std::process::exit(1);
        };
        let Some(meta) = m
            .get("metadata")
            .or_else(|| m.get("columns"))
            .and_then(as_arr)
        else {
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
        let Some(data) = m.get("data").and_then(as_arr) else {
            eprintln!("data absent");
            std::process::exit(1);
        };
        for row in data {
            let Some(r) = as_arr(row) else { continue };
            let mut cells = Vec::new();
            for c in r {
                cells.push(match c {
                    Json::Str(s) => s.clone(),
                    Json::Num(v) => format!("{}", v),
                    _ => String::new(),
                });
            }
            cells_rows.push(cells);
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
    let i_ra = col_idx.iter().find(|(k, _)| k == "ra").map(|(_, i)| *i);
    let i_dec = col_idx.iter().find(|(k, _)| k == "dec").map(|(_, i)| *i);
    let i_pmra = col_idx.iter().find(|(k, _)| k == "pmra").map(|(_, i)| *i);
    let i_pmdec = col_idx.iter().find(|(k, _)| k == "pmdec").map(|(_, i)| *i);
    let mut rows_out: Vec<String> = Vec::new();
    for cells in &cells_rows {
        let mut ra_v = i_ra
            .and_then(|i| cells.get(i))
            .and_then(|s| s.parse::<f64>().ok());
        let mut dec_v = i_dec
            .and_then(|i| cells.get(i))
            .and_then(|s| s.parse::<f64>().ok());
        if let (Some(epoch), Some(ra), Some(dec), Some(i_pmra), Some(i_pmdec)) =
            (epoch_prop, ra_v, dec_v, i_pmra, i_pmdec)
        {
            let pmra = cells
                .get(i_pmra)
                .and_then(|s| s.parse::<f64>().ok())
                .unwrap_or(0.0);
            let pmdec = cells
                .get(i_pmdec)
                .and_then(|s| s.parse::<f64>().ok())
                .unwrap_or(0.0);
            ra_v = Some(ra + pmra / (3.6e6 * dec.to_radians().cos().max(1e-6)) * (2000.0 - epoch));
            dec_v = Some(dec + pmdec / 3.6e6 * (2000.0 - epoch));
        }
        let mut obj = String::from("{");
        let mut pos = 0;
        for (k, i) in &col_idx {
            if epoch_prop.is_some() && (k == "pmra" || k == "pmdec") {
                continue;
            }
            if pos > 0 {
                obj.push(',');
            }
            pos += 1;
            let out = if k == "ra" {
                ra_v.map(|v| format!("{}", v))
                    .unwrap_or_else(|| "null".to_string())
            } else if k == "dec" {
                dec_v
                    .map(|v| format!("{}", v))
                    .unwrap_or_else(|| "null".to_string())
            } else {
                let raw = cells.get(*i).map(|s| s.as_str()).unwrap_or("");
                if raw.is_empty() {
                    "null".to_string()
                } else if raw.parse::<f64>().is_ok() {
                    raw.to_string()
                } else {
                    json_string(raw)
                }
            };
            obj.push_str(&format!("\"{}\":{}", k, out));
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
