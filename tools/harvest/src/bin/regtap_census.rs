use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use omegaflow::archivar::json::{parse_json, JsonVal};

fn as_arr(v: &JsonVal) -> Option<&Vec<JsonVal>> {
    match v {
        JsonVal::Arr(a) => Some(a),
        _ => None,
    }
}

fn as_obj(v: &JsonVal) -> Option<&HashMap<String, JsonVal>> {
    match v {
        JsonVal::Obj(o) => Some(o),
        _ => None,
    }
}

fn get_str(o: &HashMap<String, JsonVal>, key: &str) -> Option<String> {
    match o.get(key) {
        Some(JsonVal::Str(s)) => Some(s.clone()),
        _ => None,
    }
}

fn arg(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let Some(cmd) = args.first().cloned() else {
        eprintln!("usage: regtap_census <wave|import> [--ledger <path>] [--regtap <root>]");
        std::process::exit(1);
    };
    match cmd.as_str() {
        "wave" => {
            let Some(ledger_path) = arg(&args, "--ledger") else {
                eprintln!("wave needs --ledger <path>");
                std::process::exit(1);
            };
            let probe_timeout: u64 = arg(&args, "--probe-timeout")
                .and_then(|s| s.parse().ok())
                .unwrap_or(60);
            let pause_s: u64 = arg(&args, "--pause")
                .and_then(|s| s.parse().ok())
                .unwrap_or(2);
            let entries: Vec<(String, String)> = read_kandidat(&ledger_path)
                .into_iter()
                .filter(|(_, n)| n.contains("ungewogen"))
                .collect();
            let ordered = order_fruchtfolge(entries);
            println!("url\thttp_code\ttime_s\tfinal_url\tprobe\tdate");
            let mut weighed: HashMap<String, String> = HashMap::new();
            let total = ordered.len();
            for (i, (url, note)) in ordered.iter().enumerate() {
                let l = census_with(url, probe_timeout);
                println!(
                    "{}\t{}\t{}\t{}\t{}\t{}",
                    l.url, l.http_code, l.time_s, l.final_url, l.probe, l.date
                );
                weighed.insert(url.clone(), gewogen_note(note, &l));
                if i + 1 < total {
                    std::thread::sleep(std::time::Duration::from_secs(pause_s));
                }
            }
            match rewrite_ledger_notes(&ledger_path, &weighed) {
                Ok(n) => eprintln!("wave: {} weighed, {} ledger notes rewritten", total, n),
                Err(_) => eprintln!("wave: {} weighed, ledger rewrite returned void", total),
            }
        }
        "import" => {
            let Some(root) = arg(&args, "--regtap") else {
                eprintln!("import needs --regtap <root>");
                std::process::exit(1);
            };
            let ledger_path = arg(&args, "--ledger");
            let mut paths = vec![
                "phi/sources.φ".to_string(),
                "phi/dead_sources.φ".to_string(),
                "phi/blocked_sources.φ".to_string(),
                "phi/witnesses.φ".to_string(),
                "phi/footprints.φ".to_string(),
                "phi/pipeline/ledger.φ".to_string(),
            ];
            if let Some(p) = &ledger_path {
                if !paths.iter().any(|x| x == p) {
                    paths.push(p.clone());
                }
            }
            let (hosts, urls) = known_hosts_and_urls(&paths);
            let (count, rows) = (regtap_count(&root), regtap_services(&root));
            match (count, rows) {
                (Some(n), Some(rows)) => {
                    eprintln!("regtap: {} rows, COUNT(*) = {}", rows.len(), n);
                    let mut block = String::new();
                    let mut emitted = 0usize;
                    let mut artifacts = 0usize;
                    let mut duplicates = 0usize;
                    let mut seen: HashSet<String> = HashSet::new();
                    for (ivoid, url) in &rows {
                        if urls.contains(url) {
                            continue;
                        }
                        let Some(h) = host_of(url) else {
                            artifacts += 1;
                            continue;
                        };
                        if hosts.contains(&h) {
                            continue;
                        }
                        if !seen.insert(url.clone()) {
                            duplicates += 1;
                            continue;
                        }
                        let note = if ivoid.is_empty() {
                            format!("RegTAP-entdeckt, ungewogen ({})", today_ymd())
                        } else {
                            format!("{} — RegTAP-entdeckt, ungewogen ({})", ivoid, today_ymd())
                        };
                        block.push_str(&format!("ausstehend\nkandidat {}\nnote {}\n\n", url, note));
                        emitted += 1;
                    }
                    print!("{}", block);
                    eprintln!(
                        "regtap: {} candidates after Bestand-Dedupe ({} relative access_url, {} batch duplicates skipped)",
                        emitted, artifacts, duplicates
                    );
                    if let Some(p) = ledger_path {
                        let needs_sep = fs::read_to_string(&p)
                            .map(|c| !c.is_empty() && !c.ends_with("\n\n"))
                            .unwrap_or(false);
                        match OpenOptions::new().create(true).append(true).open(&p) {
                            Ok(mut f) => {
                                let sep = if needs_sep { "\n" } else { "" };
                                if f.write_all(sep.as_bytes()).is_ok()
                                    && f.write_all(block.as_bytes()).is_ok()
                                {
                                    eprintln!("ledger: appended to {}", p);
                                } else {
                                    eprintln!("ledger: write to {} returned void", p);
                                }
                            }
                            Err(_) => eprintln!("ledger: {} not writable", p),
                        }
                    }
                }
                _ => {
                    eprintln!("regtap returned void");
                    std::process::exit(1);
                }
            }
        }
        _ => {
            eprintln!("usage: regtap_census <wave|import> [--ledger <path>] [--regtap <root>]");
            std::process::exit(1);
        }
    }
}

fn host_of(url: &str) -> Option<String> {
    let rest = url
        .strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"))?;
    let host = rest.split(['/', ':', '?']).next()?;
    if host.is_empty() {
        None
    } else {
        Some(host.to_lowercase())
    }
}

fn known_hosts_and_urls(paths: &[String]) -> (HashSet<String>, HashSet<String>) {
    let mut hosts = HashSet::new();
    let mut urls = HashSet::new();
    for p in paths {
        let Ok(content) = fs::read_to_string(p) else {
            continue;
        };
        for line in content.lines() {
            let t = line.trim();
            for pfx in ["url ", "kandidat "] {
                if let Some(rest) = t.strip_prefix(pfx) {
                    let u = rest.trim().to_string();
                    if u.starts_with("http") {
                        urls.insert(u.clone());
                        if let Some(h) = host_of(&u) {
                            hosts.insert(h);
                        }
                    }
                }
            }
        }
    }
    (hosts, urls)
}

fn read_kandidat(ledger: &str) -> Vec<(String, String)> {
    let Ok(content) = fs::read_to_string(ledger) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    let mut current: Option<String> = None;
    for line in content.lines() {
        let t = line.trim();
        if let Some(url) = t.strip_prefix("kandidat ") {
            current = Some(url.trim().to_string());
        } else if t.starts_with("note ") {
            if let Some(u) = current.take() {
                out.push((u, t["note ".len()..].trim().to_string()));
            }
        } else if t.is_empty() {
            current = None;
        }
    }
    out
}

fn order_fruchtfolge(entries: Vec<(String, String)>) -> Vec<(String, String)> {
    let mut keys: Vec<String> = Vec::new();
    let mut groups: Vec<Vec<(String, String)>> = Vec::new();
    for e in entries {
        let key = match host_of(&e.0) {
            Some(h) => h,
            None => e.0.clone(),
        };
        match keys.iter().position(|k| *k == key) {
            Some(i) => groups[i].push(e),
            None => {
                keys.push(key);
                groups.push(vec![e]);
            }
        }
    }
    let mut out = Vec::with_capacity(groups.iter().map(|g| g.len()).sum());
    let mut idx = 0usize;
    loop {
        let mut emitted = false;
        for g in &groups {
            if idx < g.len() {
                out.push(g[idx].clone());
                emitted = true;
            }
        }
        if !emitted {
            break;
        }
        idx += 1;
    }
    out
}

struct CensusLine {
    url: String,
    http_code: String,
    time_s: String,
    final_url: String,
    probe: String,
    date: String,
}

fn gewogen_note(note: &str, l: &CensusLine) -> String {
    let prefix = match note.split("ungewogen").next() {
        Some(p) => p.trim_end(),
        None => note,
    };
    format!(
        "{} gewogen {}: http {} probe {}",
        prefix, l.date, l.http_code, l.probe
    )
}

fn rewrite_ledger_notes(path: &str, notes: &HashMap<String, String>) -> std::io::Result<usize> {
    let content = fs::read_to_string(path)?;
    let mut out = String::with_capacity(content.len());
    let mut current: Option<String> = None;
    let mut rewritten = 0usize;
    for line in content.lines() {
        let t = line.trim();
        if let Some(url) = t.strip_prefix("kandidat ") {
            current = Some(url.trim().to_string());
            out.push_str(line);
            out.push('\n');
        } else if t.starts_with("note ") {
            if let Some(u) = &current {
                if let Some(new_note) = notes.get(u) {
                    out.push_str("note ");
                    out.push_str(new_note);
                    out.push('\n');
                    rewritten += 1;
                } else {
                    out.push_str(line);
                    out.push('\n');
                }
            } else {
                out.push_str(line);
                out.push('\n');
            }
            current = None;
        } else {
            if t.is_empty() {
                current = None;
            }
            out.push_str(line);
            out.push('\n');
        }
    }
    let tmp = format!("{}.tmp", path);
    fs::write(&tmp, &out)?;
    fs::rename(&tmp, path)?;
    Ok(rewritten)
}

fn today_ymd() -> String {
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

fn census_with(url: &str, probe_timeout: u64) -> CensusLine {
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
    match parse_json(body) {
        Some(JsonVal::Obj(m)) => {
            m.contains_key("data")
                || m.contains_key("metadata")
                || m.contains_key("columns")
                || m.contains_key("error")
                || m.contains_key("message")
        }
        Some(JsonVal::Arr(_)) => true,
        _ => false,
    }
}

fn tap_query(root: &str, adql: &str, timeout: u64) -> Option<String> {
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
        .arg("FORMAT=json")
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

fn tap_speaks(url: &str, timeout: u64) -> bool {
    let adql = "SELECT TOP 1 * FROM tap_schema.tables";
    for c in sync_candidates(url) {
        if let Some(body) = tap_query(&c, adql, timeout) {
            if tap_shape(&body) {
                return true;
            }
        }
    }
    false
}

fn parse_json_rows(body: &str) -> Option<(Vec<String>, Vec<Vec<String>>)> {
    let parsed = parse_json(body)?;
    let (meta_opt, data) = match &parsed {
        JsonVal::Obj(m) => (
            m.get("metadata")
                .or_else(|| m.get("columns"))
                .and_then(as_arr),
            m.get("data").and_then(as_arr),
        ),
        JsonVal::Arr(a) => (None, Some(a)),
        _ => return None,
    };
    let mut col_names = Vec::new();
    if let Some(meta) = meta_opt {
        for md in meta {
            if let Some(o) = as_obj(md) {
                if let Some(nm) = get_str(o, "name") {
                    col_names.push(nm);
                }
            }
        }
    } else if let Some(JsonVal::Obj(o)) = data.as_ref().and_then(|d| d.first()) {
        for k in o.keys() {
            col_names.push(k.clone());
        }
    }
    let data = data?;
    let mut rows = Vec::new();
    for row in data {
        let mut cells = Vec::new();
        if let Some(r) = as_arr(row) {
            for c in r {
                cells.push(match c {
                    JsonVal::Str(s) => s.clone(),
                    JsonVal::Num(v) => format!("{}", v),
                    _ => String::new(),
                });
            }
        } else if let Some(o) = as_obj(row) {
            for nm in &col_names {
                cells.push(match o.get(nm) {
                    Some(JsonVal::Str(s)) => s.clone(),
                    Some(JsonVal::Num(v)) => format!("{}", v),
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

const REGTAP_TAP_WHERE: &str =
    "c.standard_id = 'ivo://ivoa.net/std/tap' AND i.intf_type = 'vs:paramhttp'";

fn regtap_count_query() -> String {
    format!(
        "SELECT COUNT(*) AS n FROM rr.resource r JOIN rr.capability c ON r.ivoid = c.ivoid JOIN rr.interface i ON c.ivoid = i.ivoid AND c.cap_index = i.cap_index WHERE {REGTAP_TAP_WHERE}"
    )
}

fn regtap_services_query() -> String {
    format!(
        "SELECT r.ivoid, i.access_url FROM rr.resource r JOIN rr.capability c ON r.ivoid = c.ivoid JOIN rr.interface i ON c.ivoid = i.ivoid AND c.cap_index = i.cap_index WHERE {REGTAP_TAP_WHERE}"
    )
}

fn regtap_count(root: &str) -> Option<i64> {
    let body = tap_query(root, &regtap_count_query(), 300)?;
    let (_, rows) = parse_json_rows(&body)?;
    let first = rows.first()?;
    first.first()?.parse().ok()
}

fn regtap_services(root: &str) -> Option<Vec<(String, String)>> {
    let body = tap_query(root, &regtap_services_query(), 300)?;
    let parsed = parse_json(&body)?;
    let data = match &parsed {
        JsonVal::Obj(m) => m.get("data").and_then(as_arr)?,
        JsonVal::Arr(a) => a,
        _ => return None,
    };
    let mut out = Vec::new();
    for row in data {
        let (ivoid, url) = match row {
            JsonVal::Obj(o) => (get_str(o, "ivoid"), get_str(o, "access_url")),
            JsonVal::Arr(a) => (
                a.first().and_then(|v| match v {
                    JsonVal::Str(s) => Some(s.clone()),
                    _ => None,
                }),
                a.get(1).and_then(|v| match v {
                    JsonVal::Str(s) => Some(s.clone()),
                    _ => None,
                }),
            ),
            _ => continue,
        };
        if let (Some(ivoid), Some(url)) = (ivoid, url) {
            if !url.is_empty() {
                out.push((ivoid, url));
            }
        }
    }
    Some(out)
}
