pub mod json;

use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

pub enum Format {
    Csv,
    Json,
    Text,
    Votable,
    VotableTd,
}

impl Format {
    pub fn as_str(&self) -> &'static str {
        match self {
            Format::Csv => "csv",
            Format::Json => "json",
            Format::Text => "text",
            Format::Votable => "votable",
            Format::VotableTd => "votable/td",
        }
    }

    pub fn parse(s: &str) -> Option<Format> {
        match s {
            "csv" => Some(Format::Csv),
            "json" => Some(Format::Json),
            "text" => Some(Format::Text),
            "votable" => Some(Format::Votable),
            "votable/td" => Some(Format::VotableTd),
            _ => None,
        }
    }
}

pub fn query_sync(root: &str, adql: &str, format: Format) -> Option<String> {
    query_sync_timeout(root, adql, format, 300)
}

pub fn query_sync_timeout(root: &str, adql: &str, format: Format, timeout: u64) -> Option<String> {
    let out = Command::new("curl")
        .arg("-sSf")
        .arg("-m")
        .arg(timeout.to_string())
        .arg("-G")
        .arg("--data-urlencode")
        .arg("REQUEST=doQuery")
        .arg("--data-urlencode")
        .arg("LANG=ADQL")
        .arg("--data-urlencode")
        .arg(format!("FORMAT={}", format.as_str()))
        .arg("--data-urlencode")
        .arg(format!("QUERY={}", adql))
        .arg(root)
        .output()
        .ok()?;
    if out.status.success() {
        String::from_utf8(out.stdout).ok()
    } else {
        eprintln!(
            "tap sync http {}: {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        None
    }
}

pub struct Job {
    pub url: String,
}

pub fn submit_async(root: &str, adql: &str, format: Format) -> Option<Job> {
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
        .arg(format!("FORMAT={}", format.as_str()))
        .arg("--data-urlencode")
        .arg(format!("QUERY={}", adql))
        .arg(&base)
        .output()
        .ok()?;
    let headers = String::from_utf8_lossy(&out.stdout);
    let url = headers
        .lines()
        .find(|l| l.to_lowercase().starts_with("location:"))
        .map(|l| l["location:".len()..].trim().to_string())?;
    Some(Job { url })
}

impl Job {
    pub fn phase(&self) -> Option<String> {
        Command::new("curl")
            .arg("-sS")
            .arg(format!("{}/phase", self.url))
            .output()
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .map(|s| s.trim().to_string())
    }

    pub fn run(&self) -> Option<()> {
        let out = Command::new("curl")
            .arg("-sS")
            .arg("-o")
            .arg("/dev/null")
            .arg("-X")
            .arg("POST")
            .arg("--data-urlencode")
            .arg("PHASE=RUN")
            .arg(format!("{}/phase", self.url))
            .output()
            .ok()?;
        out.status.success().then_some(())
    }

    pub fn wait(&self, timeout_secs: u64, poll_secs: u64) -> Option<String> {
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(timeout_secs);
        let mut phase = self.phase()?;
        if phase == "PENDING" {
            self.run()?;
        }
        loop {
            if phase == "COMPLETED" || phase == "ERROR" || phase == "ABORTED" {
                return Some(phase);
            }
            if std::time::Instant::now() >= deadline {
                return Some(phase);
            }
            std::thread::sleep(std::time::Duration::from_secs(poll_secs));
            phase = self.phase()?;
        }
    }

    pub fn result(&self) -> Option<String> {
        Command::new("curl")
            .arg("-sS")
            .arg("-m")
            .arg("3600")
            .arg(format!("{}/results/result", self.url))
            .output()
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
    }

    pub fn delete(&self) -> bool {
        match Command::new("curl")
            .arg("-sS")
            .arg("-X")
            .arg("DELETE")
            .arg("-o")
            .arg("/dev/null")
            .arg(&self.url)
            .output()
        {
            Ok(o) => o.status.success(),
            Err(_) => false,
        }
    }
}

pub fn csv_line(line: &str) -> Option<Vec<String>> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut in_q = false;
    let mut chars = line.chars().peekable();
    while let Some(c) = chars.next() {
        if in_q {
            if c == '"' {
                if chars.peek() == Some(&'"') {
                    chars.next();
                    cur.push('"');
                } else {
                    in_q = false;
                }
            } else {
                cur.push(c);
            }
        } else if c == '"' {
            in_q = true;
        } else if c == ',' {
            out.push(std::mem::take(&mut cur));
        } else if c != '\r' {
            cur.push(c);
        }
    }
    out.push(cur);
    Some(out)
}

pub fn parse_csv(body: &str) -> Option<(Vec<String>, Vec<Vec<String>>)> {
    let mut lines = body.split('\n');
    let fields = csv_line(lines.next()?)?;
    let mut rows = Vec::new();
    for line in lines {
        if line.trim().is_empty() {
            continue;
        }
        if let Some(cells) = csv_line(line) {
            rows.push(cells);
        }
    }
    Some((fields, rows))
}

pub fn parse_text(body: &str) -> Option<(Vec<String>, Vec<Vec<String>>)> {
    let mut lines = body.split('\n');
    let fields: Vec<String> = lines
        .next()?
        .split('|')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    let mut rows = Vec::new();
    for line in lines {
        let t = line.trim();
        if t.is_empty() || t.starts_with("Number of") {
            continue;
        }
        let cells: Vec<String> = t
            .split('|')
            .map(|s| {
                let c = s.trim().to_string();
                if c == "null" {
                    String::new()
                } else {
                    c
                }
            })
            .collect();
        rows.push(cells);
    }
    Some((fields, rows))
}

pub fn parse_votable(body: &str) -> Option<(Vec<String>, Vec<Vec<String>>)> {
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
    let data = body.split("<DATA>").nth(1)?;
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
                let content = match c.strip_suffix("]]>") {
                    Some(s) => s,
                    None => c,
                };
                content.trim().to_string()
            } else {
                raw.to_string()
            };
            cells.push(v);
        }
        rows.push(cells);
    }
    Some((fields, rows))
}

pub fn parse_json_rows(body: &str) -> Option<(Vec<String>, Vec<Vec<String>>)> {
    let parsed = json::parse_json(body)?;
    let (meta_opt, data) = match &parsed {
        json::JsonVal::Obj(m) => (
            m.get("metadata")
                .or_else(|| m.get("columns"))
                .and_then(json::as_arr),
            m.get("data").and_then(json::as_arr),
        ),
        json::JsonVal::Arr(a) => (None, Some(a)),
        _ => return None,
    };
    let mut col_names = Vec::new();
    if let Some(meta) = meta_opt {
        for md in meta {
            if let Some(o) = json::as_obj(md) {
                if let Some(nm) = json::get_str(o, "name") {
                    col_names.push(nm);
                }
            }
        }
    } else if let Some(json::JsonVal::Obj(o)) = data.as_ref().and_then(|d| d.first()) {
        for k in o.keys() {
            col_names.push(k.clone());
        }
    }
    let data = data?;
    let mut rows = Vec::new();
    for row in data {
        let mut cells = Vec::new();
        if let Some(r) = json::as_arr(row) {
            for c in r {
                cells.push(match c {
                    json::JsonVal::Str(s) => s.clone(),
                    json::JsonVal::Num(v) => format!("{}", v),
                    _ => String::new(),
                });
            }
        } else if let Some(o) = json::as_obj(row) {
            for nm in &col_names {
                cells.push(match o.get(nm) {
                    Some(json::JsonVal::Str(s)) => s.clone(),
                    Some(json::JsonVal::Num(v)) => format!("{}", v),
                    _ => String::new(),
                });
            }
        } else {
            continue;
        }
        rows.push(cells);
    }
    Some((col_names, rows))
}

pub fn tables(root: &str) -> Option<Vec<(String, Option<String>, Option<String>)>> {
    let adql = "SELECT table_name, table_type, schema_name FROM tap_schema.tables".to_string();
    for c in sync_candidates(root) {
        let Some(body) = query_sync(&c, &adql, Format::Json) else {
            continue;
        };
        let Some((cols, rows)) = parse_json_rows(&body) else {
            continue;
        };
        let idx = |name: &str| cols.iter().position(|x| x.eq_ignore_ascii_case(name));
        let (Some(i_name), Some(i_type), Some(i_schema)) =
            (idx("table_name"), idx("table_type"), idx("schema_name"))
        else {
            continue;
        };
        let mut out = Vec::new();
        for r in rows {
            let cell = |i: usize| {
                r.get(i)
                    .and_then(|s| if s.is_empty() { None } else { Some(s.clone()) })
            };
            let Some(name) = cell(i_name) else {
                continue;
            };
            out.push((name, cell(i_type), cell(i_schema)));
        }
        if !out.is_empty() {
            return Some(out);
        }
    }
    None
}

pub struct CensusLine {
    pub url: String,
    pub http_code: String,
    pub time_s: String,
    pub final_url: String,
    pub probe: String,
    pub date: String,
}

pub fn today_ymd() -> String {
    let unix = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(d) => d.as_secs() as i64,
        Err(_) => return "absent".to_string(),
    };
    let (y, m, d) = civil_from_unix(unix);
    format!("{y:04}-{m:02}-{d:02}")
}

fn civil_from_unix(unix: i64) -> (i64, i64, i64) {
    let days = unix.div_euclid(86400);
    let z = days + 719468;
    let era = z.div_euclid(146097);
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}

pub fn census(url: &str) -> CensusLine {
    census_with(url, 300)
}

pub fn census_with(url: &str, probe_timeout: u64) -> CensusLine {
    let date = today_ymd();
    let out = Command::new("curl")
        .arg("-sS")
        .arg("-L")
        .arg("-m")
        .arg("30")
        .arg("-o")
        .arg("/dev/null")
        .arg("-w")
        .arg("%{http_code}\t%{time_total}\t%{url_effective}")
        .arg(url)
        .output();
    let (http_code, time_s, final_url) = match out {
        Ok(o) => {
            let s = String::from_utf8_lossy(&o.stdout).to_string();
            let mut it = s.split('\t');
            (
                it.next().unwrap_or("000").trim().to_string(),
                it.next().unwrap_or("absent").trim().to_string(),
                it.next().unwrap_or(url).trim().to_string(),
            )
        }
        Err(_) => ("000".to_string(), "absent".to_string(), url.to_string()),
    };
    let probe = if http_code == "000" {
        "kein-http".to_string()
    } else if tap_speaks(url, probe_timeout) {
        "tap".to_string()
    } else {
        "http".to_string()
    };
    CensusLine {
        url: url.to_string(),
        http_code,
        time_s,
        final_url,
        probe,
        date,
    }
}

fn sync_candidates(url: &str) -> Vec<String> {
    let u = url.trim_end_matches('/');
    if u.ends_with("/sync") {
        vec![u.to_string()]
    } else if u.ends_with("/tap") {
        vec![format!("{}/sync", u), u.to_string()]
    } else {
        vec![format!("{}/sync", u), u.to_string()]
    }
}

fn tap_shape(body: &str) -> bool {
    match json::parse_json(body) {
        Some(json::JsonVal::Obj(m)) => {
            m.contains_key("data")
                || m.contains_key("metadata")
                || m.contains_key("columns")
                || m.contains_key("error")
                || m.contains_key("message")
        }
        Some(json::JsonVal::Arr(_)) => true,
        _ => false,
    }
}

fn tap_speaks(url: &str, timeout: u64) -> bool {
    let adql = "SELECT TOP 1 * FROM tap_schema.tables";
    for c in sync_candidates(url) {
        if let Some(body) = query_sync_timeout(&c, adql, Format::Json, timeout) {
            if tap_shape(&body) {
                return true;
            }
        }
    }
    false
}
