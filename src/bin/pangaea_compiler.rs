use omegaflow::cdn::upload_asset;
use std::env;
use std::fs;
use std::process::Command;

fn curl(url: &str) -> Option<String> {
    let out = Command::new("curl")
        .arg("-sSL")
        .arg("-m")
        .arg("120")
        .arg(url)
        .output()
        .ok()?;
    if out.status.success() {
        Some(String::from_utf8_lossy(&out.stdout).into_owned())
    } else {
        None
    }
}

fn collection_members(id: &str) -> Vec<String> {
    let url = format!("https://doi.pangaea.de/10.1594/{}", id);
    let Some(html) = curl(&url) else {
        return vec![];
    };
    let mut members = Vec::new();
    let bytes = html.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i..].starts_with(b"PANGAEA.") {
            let mut k = i + 8;
            while k < bytes.len() && bytes[k].is_ascii_digit() {
                k += 1;
            }
            if k > i + 8 {
                let m = String::from_utf8_lossy(&bytes[i..k]).to_string();
                if m != id && !members.contains(&m) {
                    members.push(m);
                }
                i = k;
                continue;
            }
        }
        i += 1;
    }
    members
}

struct Tab {
    lat: Option<f64>,
    lon: Option<f64>,
    cols: Vec<String>,
    rows: Vec<Vec<String>>,
}

fn parse_tab(body: &str) -> Option<Tab> {
    let mut lat: Option<f64> = None;
    let mut lon: Option<f64> = None;
    for line in body.lines() {
        if let Some(pos) = line.find("LATITUDE:") {
            let rest = &line[pos..];
            if let Some(v) = rest
                .trim_start_matches("LATITUDE:")
                .trim()
                .split_whitespace()
                .next()
                .and_then(|s| s.parse::<f64>().ok())
            {
                lat = Some(v);
            }
            if let Some(lp) = rest.find("LONGITUDE:") {
                if let Some(v) = rest[lp..]
                    .trim_start_matches("LONGITUDE:")
                    .trim()
                    .split_whitespace()
                    .next()
                    .and_then(|s| s.parse::<f64>().ok())
                {
                    lon = Some(v);
                }
            }
        }
    }
    let mut after = false;
    let mut cols: Vec<String> = Vec::new();
    let mut rows: Vec<Vec<String>> = Vec::new();
    for line in body.lines() {
        let t = line.trim_end();
        if t == "*/" {
            after = true;
            continue;
        }
        if !after {
            continue;
        }
        if t.is_empty() {
            continue;
        }
        let cells: Vec<String> = t.split('\t').map(|s| s.trim().to_string()).collect();
        if cols.is_empty() {
            cols = cells;
        } else if cells.len() >= cols.len() {
            rows.push(cells);
        }
    }
    if cols.is_empty() || rows.is_empty() {
        return None;
    }
    Some(Tab {
        lat,
        lon,
        cols,
        rows,
    })
}

fn jkey(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

fn clean_key(s: &str) -> String {
    let mut out = String::new();
    let mut prev_us = false;
    for c in s.chars() {
        let cl = c.to_lowercase().next().unwrap_or(c);
        if cl.is_ascii_alphanumeric() {
            out.push(cl);
            prev_us = false;
        } else if !prev_us && !out.is_empty() {
            out.push('_');
            prev_us = true;
        }
    }
    out.trim_matches('_').to_string()
}

fn jval(cell: &str) -> String {
    if cell.is_empty() {
        return "null".to_string();
    }
    if let Ok(n) = cell.parse::<f64>() {
        return format!("{}", n);
    }
    format!("\"{}\"", jkey(cell))
}

fn emit_json(tab: &Tab, total: &mut usize) -> String {
    let mut out = String::from("[");
    for (ri, row) in tab.rows.iter().enumerate() {
        if ri > 0 {
            out.push(',');
        }
        let lat_s = match tab.lat {
            Some(v) => v.to_string(),
            None => "null".to_string(),
        };
        let lon_s = match tab.lon {
            Some(v) => v.to_string(),
            None => "null".to_string(),
        };
        out.push_str(&format!("\"lat\":{}, \"lon\":{}", lat_s, lon_s));
        for (ci, col) in tab.cols.iter().enumerate() {
            let val = row.get(ci).map(|s| s.as_str()).unwrap_or("");
            out.push_str(&format!(", \"{}\": {}", clean_key(col), jval(val)));
        }
        *total += 1;
    }
    out.push_str("]\n");
    out
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut id: Option<String> = None;
    let mut out: Option<String> = None;
    let mut ci_mode = false;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--id" => {
                id = args.get(i + 1).cloned();
                i += 1;
            }
            "--out" => {
                out = args.get(i + 1).cloned();
                i += 1;
            }
            "--ci-mode" => ci_mode = true,
            _ => {}
        }
        i += 1;
    }
    let Some(id) = id else {
        eprintln!("--id absent");
        std::process::exit(1);
    };
    let mut buf = String::new();
    let mut total = 0usize;
    let mut meta = String::new();
    let direct = format!(
        "https://doi.pangaea.de/10.1594/{id}?format=textfile",
        id = id
    );
    if let Some(body) = curl(&direct) {
        if !body.trim_start().starts_with('<') {
            if let Some(tab) = parse_tab(&body) {
                meta = format!(
                    "{}: lat {:?} lon {:?} cols {} rows {}",
                    id,
                    tab.lat,
                    tab.lon,
                    tab.cols.len(),
                    tab.rows.len()
                );
                buf = emit_json(&tab, &mut total);
            }
        }
    }
    if buf.is_empty() {
        let members = collection_members(&id);
        eprintln!("collection {}: {} members", id, members.len());
        for m in &members {
            let murl = format!("https://doi.pangaea.de/10.1594/{}?format=textfile", m);
            if let Some(body) = curl(&murl) {
                if body.trim_start().starts_with('<') {
                    continue;
                }
                if let Some(tab) = parse_tab(&body) {
                    buf = emit_json(&tab, &mut total);
                    if meta.is_empty() {
                        meta = format!(
                            "{}: lat {:?} lon {:?} cols {} rows {}",
                            m,
                            tab.lat,
                            tab.lon,
                            tab.cols.len(),
                            tab.rows.len()
                        );
                    }
                }
            }
        }
    }
    if buf.is_empty() {
        eprintln!("pangaea: {} returned void ({} members)", id, total);
        std::process::exit(1);
    }
    if let Some(path) = out {
        let _ = fs::write(&path, buf);
        if ci_mode && !upload_asset(&path) {
            eprintln!("upload: {} did not reach the CDN", path);
            std::process::exit(1);
        }
        eprintln!("pangaea: {} rows → {} · {}", total, path, meta);
    } else {
        eprintln!("pangaea: {} rows · {} (--out absent)", total, meta);
    }
}
