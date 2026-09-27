use std::collections::HashMap;
use std::process::Command;

use omegaflow::cdn::upload_release;
use omegaflow::json::{JsonVal, parse_json};

const BYPARAMS_ENDPOINT: &str = "https://ned.ipac.caltech.edu/byparams";
const TICKET_CALLBACK: &str = "https://ned.ipac.caltech.edu/ticket/refresh/callback";
const CDN_TAG: &str = "ned.ipac.caltech.edu-byparams";

fn splitmix64(mut x: u64) -> u64 {
    x = x.wrapping_add(0x9E37_79B9_7F4A_7C15);
    x = (x ^ (x >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    x ^ (x >> 31)
}

fn hash64(s: &str) -> u64 {
    let mut h: u64 = 0xCBF2_9CE4_8422_2325;
    for b in s.as_bytes() {
        h = (h ^ u64::from(*b)).wrapping_mul(0x1000_0000_01B3);
    }
    splitmix64(h)
}

fn identity_rank(identity: &str) -> f64 {
    let h = hash64(identity);
    (h >> 11) as f64 / (1u64 << 53) as f64
}

fn latlon_cell(ra: f64, dec: f64, n: i64) -> Option<i64> {
    if !(ra.is_finite() && dec.is_finite() && (-90.0..=90.0).contains(&dec)) {
        return None;
    }
    let dec_idx = (((dec + 90.0) / 180.0 * n as f64).floor() as i64).clamp(0, n - 1);
    let ra_idx = ((ra.rem_euclid(360.0) / 360.0 * n as f64).floor() as i64).clamp(0, n - 1);
    Some(dec_idx * n + ra_idx)
}

fn band_count(step_deg: f64) -> usize {
    (180.0 / step_deg).ceil().max(1.0) as usize
}

fn band_bounds(index: usize, step_deg: f64) -> Option<(f64, f64)> {
    let total = band_count(step_deg);
    if index >= total {
        return None;
    }
    let lo = -90.0 + index as f64 * step_deg;
    let hi = (lo + step_deg).min(90.0);
    Some((lo, hi))
}

fn fmt_num(v: f64) -> String {
    let s = format!("{}", v);
    if s.contains('.') {
        s
    } else {
        format!("{}.0", s)
    }
}

fn cookie_jar() -> std::path::PathBuf {
    std::env::temp_dir().join(format!("ned_byparams_{}.cookies", std::process::id()))
}

fn http_get(url: &str) -> Option<String> {
    let jar = cookie_jar();
    let out = Command::new("curl")
        .arg("-sS")
        .arg("-m")
        .arg("60")
        .arg("-b")
        .arg(&jar)
        .arg("-c")
        .arg(&jar)
        .arg(url)
        .output()
        .ok()?;
    if out.status.success() {
        String::from_utf8(out.stdout).ok()
    } else {
        eprintln!(
            "ned_byparams: curl http {}: {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        None
    }
}

fn http_post_form(url: &str, fields: &[(String, String)]) -> Option<String> {
    let jar = cookie_jar();
    let mut cmd = Command::new("curl");
    cmd.arg("-sS")
        .arg("-m")
        .arg("120")
        .arg("-b")
        .arg(&jar)
        .arg("-c")
        .arg(&jar)
        .arg("-X")
        .arg("POST");
    for (k, v) in fields {
        cmd.arg("--data-urlencode").arg(format!("{}={}", k, v));
    }
    let out = cmd.arg(url).output().ok()?;
    if out.status.success() {
        String::from_utf8(out.stdout).ok()
    } else {
        eprintln!(
            "ned_byparams: curl post {}: {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        None
    }
}

fn attr_map(tag: &str) -> HashMap<String, String> {
    let mut out = HashMap::new();
    let chars: Vec<char> = tag.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] != '=' {
            i += 1;
            continue;
        }
        let mut j = i;
        while j > 0 && (chars[j - 1].is_ascii_alphanumeric() || chars[j - 1] == '_') {
            j -= 1;
        }
        let key: String = chars[j..i].iter().collect();
        if key.is_empty() {
            i += 1;
            continue;
        }
        let mut k = i + 1;
        while k < chars.len() && chars[k].is_whitespace() {
            k += 1;
        }
        if k >= chars.len() || (chars[k] != '"' && chars[k] != '\'') {
            i += 1;
            continue;
        }
        let quote = chars[k];
        let mut v = String::new();
        k += 1;
        while k < chars.len() && chars[k] != quote {
            v.push(chars[k]);
            k += 1;
        }
        out.insert(key, v);
        i = k + 1;
    }
    out
}

fn hidden_value(body: &str, name: &str) -> Option<String> {
    let needle = format!("name=\"{}\"", name);
    let pos = body.find(&needle)?;
    let rest = &body[pos + needle.len()..];
    let seg = rest.split_once('>')?.0;
    attr_map(seg).get("value").cloned()
}

fn captcha_answer(body: &str) -> Option<(String, String)> {
    let cap = hidden_value(body, "cortnedwic")?;
    let qmark = body.find("What does ")?;
    let qrest = &body[qmark + "What does ".len()..];
    let q = qrest.split("equal").next()?.trim();
    let mut parts = q.split('+');
    let a: i64 = parts.next()?.trim().parse().ok()?;
    let b: i64 = parts.next()?.trim().parse().ok()?;
    Some((format!("{}", a + b), cap))
}

fn submit_query(dec1: f64, dec2: f64, output_options: &str) -> Option<String> {
    let form = http_get(BYPARAMS_ENDPOINT)?;
    let build_id = hidden_value(&form, "form_build_id")?;
    let (cap_answer, cap_nedwic) = captcha_answer(&form)?;
    let fields: Vec<(String, String)> = vec![
        ("search_region_type".into(), "Box".into()),
        ("coordsys".into(), "Equatorial J2000".into()),
        ("rarange".into(), "Unconstrained".into()),
        ("decrange".into(), "Between".into()),
        ("dec1".into(), format!("{}d", fmt_num(dec1))),
        ("dec2".into(), format!("{}d", fmt_num(dec2))),
        ("rsrange".into(), "Available".into()),
        ("rsin".into(), String::new()),
        ("rsunit".into(), "z".into()),
        ("obj_sort".into(), "RA - ascending".into()),
        ("output_options".into(), output_options.to_string()),
        ("cortcap_t".into(), cap_answer),
        ("cortnedwic".into(), cap_nedwic),
        ("op".into(), "Go".into()),
        ("form_build_id".into(), build_id),
        ("form_id".into(), "byparams".into()),
    ];
    http_post_form(BYPARAMS_ENDPOINT, &fields).map(|b| b.trim().to_string())
}

fn poll_status(body: &str) -> Option<(i64, String)> {
    let parsed = parse_json(body)?;
    let obj = match &parsed {
        JsonVal::Obj(m) => m,
        _ => return None,
    };
    let code = match obj.get("code") {
        Some(JsonVal::Num(v)) => *v as i64,
        Some(JsonVal::Str(s)) => s.parse().ok()?,
        _ => return None,
    };
    let html = match obj.get("html") {
        Some(JsonVal::Str(s)) => s.clone(),
        _ => return None,
    };
    Some((code, html))
}

fn poll_ticket(ticket: &str, max_poll_secs: u64, poll_interval_secs: u64) -> Option<(i64, String)> {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(max_poll_secs);
    let mut code: i64 = 0;
    loop {
        let url = format!(
            "{}?ticket={}&code={}&refreshtime={}",
            TICKET_CALLBACK, ticket, code, poll_interval_secs
        );
        let body = http_get(&url)?;
        let (new_code, html) = match poll_status(&body) {
            Some(v) => v,
            None => {
                eprintln!(
                    "ned_byparams: ticket status parse void ({} bytes)",
                    body.len()
                );
                return None;
            }
        };
        code = new_code;
        if code != 0 {
            eprintln!("ned_byparams: ticket {} done (code {})", ticket, code);
            return Some((code, html));
        }
        if std::time::Instant::now() >= deadline {
            eprintln!(
                "ned_byparams: ticket {} still running after {} s",
                ticket, max_poll_secs
            );
            return None;
        }
        std::thread::sleep(std::time::Duration::from_secs(poll_interval_secs));
    }
}

fn result_url(html: &str) -> Option<String> {
    if let Some(p) = html.find("view_results") {
        let seg = &html[p..];
        if let Some(dv) = seg.find("data-value=") {
            let rest = &seg[dv + "data-value=".len()..];
            let rest = rest.trim_start_matches('"').trim_start_matches('\'');
            let quote = rest.chars().next()?;
            let end = rest[quote.len_utf8()..].find(quote)?;
            return Some(rest[quote.len_utf8()..quote.len_utf8() + end].to_string());
        }
    }
    let mut best: Option<String> = None;
    for attr in ["href=", "data-value="] {
        let mut cursor = 0;
        while let Some(p) = html[cursor..].find(attr) {
            let start = cursor + p + attr.len();
            let rest = html[start..]
                .trim_start_matches('"')
                .trim_start_matches('\'');
            let quote = match rest.chars().next() {
                Some(q) if q == '"' || q == '\'' => q,
                _ => {
                    cursor = start + 1;
                    continue;
                }
            };
            let inner = &rest[quote.len_utf8()..];
            let Some(end) = inner.find(quote) else {
                cursor = start + 1;
                continue;
            };
            let u = &inner[..end];
            if u.starts_with('/') {
                best = Some(format!("https://ned.ipac.caltech.edu{}", u));
            } else if u.starts_with("http") {
                best = Some(u.to_string());
            }
            cursor = start + end + 1;
        }
    }
    best
}

fn fetch_body(url: &str) -> Option<String> {
    let out = Command::new("curl")
        .arg("-sS")
        .arg("-m")
        .arg("3600")
        .arg("--retry")
        .arg("1")
        .arg("--retry-connrefused")
        .arg(url)
        .output()
        .ok()?;
    if out.status.success() {
        String::from_utf8(out.stdout).ok()
    } else {
        eprintln!(
            "ned_byparams: curl fetch {}: {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        None
    }
}

fn votable_fields(body: &str) -> Vec<HashMap<String, String>> {
    let mut out = Vec::new();
    for f in body.split("<FIELD").skip(1) {
        let seg = match f.split_once('>') {
            Some((s, _)) => s,
            None => continue,
        };
        if seg.contains('=') {
            out.push(attr_map(seg));
        }
    }
    out
}

fn votable_rows(data: &str) -> Vec<Vec<String>> {
    let mut rows = Vec::new();
    for tr in data.split("<TR>").skip(1) {
        let end = match tr.split_once("</TR>") {
            Some((e, _)) => e,
            None => continue,
        };
        let mut cells: Vec<String> = Vec::new();
        let mut pos = 0;
        loop {
            let rel = match end[pos..].find("<TD") {
                Some(i) => i,
                None => break,
            };
            let start = pos + rel;
            let tag_end = match end[start..].find('>') {
                Some(i) => start + i,
                None => break,
            };
            let tag = &end[start..tag_end];
            let self_closed = tag.trim_end().ends_with('/');
            let content_start = tag_end + 1;
            if self_closed {
                cells.push(String::new());
                pos = content_start;
                continue;
            }
            let close = match end[content_start..].find("</TD>") {
                Some(i) => content_start + i,
                None => break,
            };
            let raw = &end[content_start..close];
            let v = if let Some(c) = raw.strip_prefix("<![CDATA[") {
                c.strip_suffix("]]>").unwrap_or(c).trim().to_string()
            } else if let Some(st) = raw.find('<') {
                raw[..st].trim().to_string()
            } else {
                raw.trim().to_string()
            };
            cells.push(v);
            pos = close + "</TD>".len();
        }
        rows.push(cells);
    }
    rows
}

fn csv_line(line: &str) -> Vec<String> {
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
    out
}

fn csv_rows(body: &str) -> Option<(Vec<String>, Vec<Vec<String>>)> {
    let mut lines = body.split('\n');
    let fields = csv_line(lines.next()?);
    if fields.is_empty() {
        return None;
    }
    let mut rows = Vec::new();
    for line in lines {
        if line.trim().is_empty() {
            continue;
        }
        rows.push(csv_line(line));
    }
    Some((fields, rows))
}

struct Object {
    identity: String,
    name: Option<String>,
    ra: f64,
    dec: f64,
    z: f64,
    cz: Option<f64>,
    ptype: Option<String>,
    n_spectra: Option<i64>,
    cell: i64,
    rank: f64,
}

fn parse_result(body: &str, lattice: i64, votable: bool) -> Option<(Vec<Object>, usize)> {
    let (names, rows) = if votable {
        let data = body.split("<DATA>").nth(1)?;
        let data = data.split("</DATA>").next().unwrap_or(data);
        let fields = votable_fields(body);
        let names: Vec<String> = fields
            .iter()
            .map(|m| match m.get("ID").or_else(|| m.get("name")) {
                Some(s) => s.clone(),
                None => String::new(),
            })
            .collect();
        (names, votable_rows(data))
    } else {
        csv_rows(body)?
    };
    let col = |keys: &[&str]| -> Option<usize> {
        names
            .iter()
            .position(|n| keys.iter().any(|k| n.eq_ignore_ascii_case(k)))
    };
    let name_col = col(&["Object Name", "prefname"]);
    let ra_col = col(&["RA", "ra"]);
    let dec_col = col(&["DEC", "Dec", "dec"]);
    let z_col = col(&["Redshift", "z", "Redshift (z)"]);
    let cz_col = col(&["Velocity", "cz", "velocity"]);
    let ptype_col = col(&["Type", "Phys Type", "ptype"]);
    let n_spectra_col = col(&["Spectra", "n_spectra"]);
    let (Some(ra_col), Some(dec_col), Some(z_col)) = (ra_col, dec_col, z_col) else {
        eprintln!(
            "ned_byparams: ra/dec/z column absent (ra {:?} dec {:?} z {:?}) of {:?}",
            ra_col, dec_col, z_col, names
        );
        return None;
    };
    let mut seen: HashMap<String, ()> = HashMap::new();
    let mut out = Vec::new();
    let mut skipped = 0usize;
    for row in rows {
        let c = |i: usize| row.get(i).map(|s| s.as_str()).unwrap_or("");
        let fcell = |i: usize| c(i).parse::<f64>().ok();
        let (Some(ra), Some(dec), Some(z)) = (fcell(ra_col), fcell(dec_col), fcell(z_col)) else {
            skipped += 1;
            continue;
        };
        if !(ra.is_finite() && dec.is_finite() && z.is_finite()) {
            skipped += 1;
            continue;
        }
        if !(-90.0..=90.0).contains(&dec) {
            skipped += 1;
            continue;
        }
        let name_raw = name_col.map(c).unwrap_or("").trim();
        let name = if name_raw.is_empty() {
            None
        } else {
            Some(name_raw.to_string())
        };
        let identity = match &name {
            Some(n) => n.clone(),
            None => format!("{:x}.{:x}", ra.to_bits(), dec.to_bits()),
        };
        if seen.contains_key(&identity) {
            skipped += 1;
            continue;
        }
        seen.insert(identity.clone(), ());
        let cz = cz_col.and_then(fcell).filter(|v| v.is_finite());
        let ptype = ptype_col
            .map(c)
            .filter(|s| !s.trim().is_empty())
            .map(|s| s.trim().to_string());
        let n_spectra = n_spectra_col
            .and_then(fcell)
            .filter(|v| v.is_finite())
            .map(|v| v as i64);
        let cell = latlon_cell(ra, dec, lattice)?;
        let rank = identity_rank(&identity);
        out.push(Object {
            identity,
            name,
            ra,
            dec,
            z,
            cz,
            ptype,
            n_spectra,
            cell,
            rank,
        });
    }
    let key = |o: &Object| (o.cell, (o.rank * 1e12) as i64, o.identity.clone());
    out.sort_by_key(key);
    Some((out, skipped))
}

fn json_string(s: &str) -> String {
    let mut o = String::from("\"");
    for ch in s.chars() {
        match ch {
            '"' => o.push_str("\\\""),
            '\\' => o.push_str("\\\\"),
            '\n' => o.push_str("\\n"),
            '\r' => o.push_str("\\r"),
            '\t' => o.push_str("\\t"),
            c if (c as u32) < 0x20 => o.push_str(&format!("\\u{:04x}", c as u32)),
            c => o.push(c),
        }
    }
    o.push('"');
    o
}

fn write_array(path: &str, objs: &[Object]) {
    let mut buf = String::from("[\n");
    for (i, o) in objs.iter().enumerate() {
        if i > 0 {
            buf.push_str(",\n");
        }
        buf.push_str("  {");
        let mut first = true;
        let mut field = |k: &str, v: String| {
            if !first {
                buf.push(',');
            }
            first = false;
            buf.push_str(&format!("\"{}\":{}", k, v));
        };
        if let Some(n) = &o.name {
            field("name", json_string(n));
        }
        field("ra", format!("{}", o.ra));
        field("dec", format!("{}", o.dec));
        field("z", format!("{}", o.z));
        if let Some(cz) = o.cz {
            field("cz", format!("{}", cz));
        }
        if let Some(p) = &o.ptype {
            field("ptype", json_string(p));
        }
        if let Some(n) = o.n_spectra {
            field("n_spectra", format!("{}", n));
        }
        field("cell", format!("{}", o.cell));
        field("rank", format!("{}", o.rank));
        buf.push('}');
    }
    buf.push_str("\n]\n");
    let tmp = format!("{}.tmp", path);
    match std::fs::write(&tmp, buf.as_bytes()) {
        Ok(_) => match std::fs::rename(&tmp, path) {
            Ok(_) => {}
            Err(err) => {
                eprintln!("write {}: {}", path, err);
                std::process::exit(1);
            }
        },
        Err(err) => {
            eprintln!("write {}: {}", tmp, err);
            std::process::exit(1);
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut index: Option<usize> = None;
    let mut dec1_dir: Option<f64> = None;
    let mut dec2_dir: Option<f64> = None;
    let mut step_deg: f64 = 1.0;
    let mut lattice: i64 = 1024;
    let mut out: Option<String> = None;
    let mut output_options = String::from("VOTABLE/TD");
    let mut max_poll_secs: u64 = 90 * 60;
    let mut poll_interval_secs: u64 = 30;
    let mut count_only = false;
    let mut ci_mode = false;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--index" => {
                index = args.get(i + 1).and_then(|s| s.parse().ok());
                i += 1;
            }
            "--dec1" => {
                dec1_dir = args.get(i + 1).and_then(|s| s.parse().ok());
                i += 1;
            }
            "--dec2" => {
                dec2_dir = args.get(i + 1).and_then(|s| s.parse().ok());
                i += 1;
            }
            "--dec-step" => {
                step_deg = args.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(1.0);
                i += 1;
            }
            "--lattice" => {
                lattice = args.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(1024);
                i += 1;
            }
            "--out" => {
                out = args.get(i + 1).cloned();
                i += 1;
            }
            "--output-format" => {
                output_options = args.get(i + 1).cloned().unwrap_or("VOTABLE/TD".to_string());
                i += 1;
            }
            "--max-poll" => {
                max_poll_secs = args.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(5400);
                i += 1;
            }
            "--poll-interval" => {
                poll_interval_secs = args.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(30);
                i += 1;
            }
            "--count" => count_only = true,
            "--ci-mode" => ci_mode = true,
            _ => {}
        }
        i += 1;
    }
    if count_only {
        println!("{}", band_count(step_deg));
        return;
    }
    let (dec1, dec2) = match (dec1_dir, dec2_dir) {
        (Some(a), Some(b)) => (a, b),
        _ => {
            let idx = match index {
                Some(v) => v,
                None => {
                    eprintln!("ned_byparams: --index absent (or give --dec1 and --dec2)");
                    std::process::exit(1);
                }
            };
            match band_bounds(idx, step_deg) {
                Some(b) => b,
                None => {
                    eprintln!(
                        "ned_byparams: index {} outside the bands (total {})",
                        idx,
                        band_count(step_deg)
                    );
                    std::process::exit(1);
                }
            }
        }
    };
    let out_path = match out {
        Some(p) => p,
        None => {
            eprintln!("ned_byparams: --out absent");
            std::process::exit(1);
        }
    };
    let ticket = match submit_query(dec1, dec2, &output_options) {
        Some(t) if !t.is_empty() => t,
        other => {
            eprintln!(
                "ned_byparams: submit void at dec [{}, {}): {:?}",
                dec1, dec2, other
            );
            std::process::exit(1);
        }
    };
    eprintln!(
        "ned_byparams: ticket {} dec [{}, {}) fmt {}",
        ticket, dec1, dec2, output_options
    );
    let (code, html) = match poll_ticket(&ticket, max_poll_secs, poll_interval_secs) {
        Some(v) => v,
        None => {
            eprintln!(
                "ned_byparams: ticket {} poll void — the band [{}, {}) stays unharvested",
                ticket, dec1, dec2
            );
            std::process::exit(1);
        }
    };
    if code == 0 {
        eprintln!(
            "ned_byparams: ticket {} still running after poll — the band [{}, {}) stays unharvested",
            ticket, dec1, dec2
        );
        std::process::exit(1);
    }
    let url = match result_url(&html) {
        Some(u) => u,
        None => {
            eprintln!(
                "ned_byparams: ticket {} result URL absent from {} bytes of html",
                ticket,
                html.len()
            );
            std::process::exit(1);
        }
    };
    eprintln!("ned_byparams: ticket {} result url {}", ticket, url);
    let body = match fetch_body(&url) {
        Some(b) => b,
        None => {
            eprintln!("ned_byparams: result fetch void at {}", url);
            std::process::exit(1);
        }
    };
    let votable = output_options.starts_with("VOTABLE");
    match parse_result(&body, lattice, votable) {
        Some((objs, skipped)) => {
            write_array(&out_path, &objs);
            eprintln!(
                "ned_byparams: dec [{}, {}) : {} objects written ({} skipped) → {}",
                dec1,
                dec2,
                objs.len(),
                skipped,
                out_path
            );
            if ci_mode && !upload_release(CDN_TAG, &out_path) {
                std::process::exit(1);
            }
        }
        None => {
            eprintln!(
                "ned_byparams: parse void at dec [{}, {}) ({} bytes)",
                dec1,
                dec2,
                body.len()
            );
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn band_bounds_cover_full_sky() {
        assert_eq!(band_count(1.0), 180);
        assert_eq!(band_bounds(0, 1.0), Some((-90.0, -89.0)));
        assert_eq!(band_bounds(179, 1.0), Some((89.0, 90.0)));
        assert_eq!(band_bounds(180, 1.0), None);
        assert_eq!(band_bounds(0, 90.0), Some((-90.0, 0.0)));
        assert_eq!(band_bounds(1, 90.0), Some((0.0, 90.0)));
    }

    #[test]
    fn captcha_parses_arithmetic() {
        let form = r#"<label for="edit-cortcap-t">What does 1 + 6 equal? </label>
<input type="text" name="cortcap_t" value="" />
<input type="hidden" name="cortnedwic" value="11" />"#;
        assert_eq!(
            captcha_answer(form),
            Some(("7".to_string(), "11".to_string()))
        );
    }

    #[test]
    fn hidden_value_reads_form_build_id() {
        let form = r#"<input type="hidden" name="form_build_id" value="form-9_CkB8PwrLTSl6RsexTOKpI7lMJ6KT9E1pCRocLHQ8A" />"#;
        assert_eq!(
            hidden_value(form, "form_build_id").as_deref(),
            Some("form-9_CkB8PwrLTSl6RsexTOKpI7lMJ6KT9E1pCRocLHQ8A")
        );
    }

    #[test]
    fn poll_status_reads_code_and_html() {
        let body = r#"{"ticket":"b08cfc32","refreshtime":"15","code":0,"html":"<table></table>"}"#;
        let (code, html) = poll_status(body).unwrap();
        assert_eq!(code, 0);
        assert_eq!(html, "<table></table>");
    }

    #[test]
    fn cells_are_lattice_unique() {
        assert_eq!(latlon_cell(0.0, 0.0, 1024), Some(512 * 1024));
        assert!(latlon_cell(f64::NAN, 0.0, 1024).is_none());
        assert!(latlon_cell(0.0, 100.0, 1024).is_none());
    }

    const CSV: &str = "No.,Object Name,RA,DEC,Type,Velocity,Redshift,Spectra\n\
1,NGC 0001,0.0,27.7,G,4512,0.0150,3\n\
2,WISEA J000101.00+275030.1,0.0,27.7,G,,,1\n";

    #[test]
    fn parse_csv_rows_and_skip_absent_z() {
        let (fields, rows) = csv_rows(CSV).unwrap();
        assert_eq!(fields.len(), 8);
        assert_eq!(rows.len(), 2);
        let (objs, skipped) = parse_result(CSV, 1024, false).unwrap();
        assert_eq!(objs.len(), 1);
        assert_eq!(skipped, 1);
        assert_eq!(objs[0].name.as_deref(), Some("NGC 0001"));
        assert!((objs[0].z - 0.015).abs() < 1e-9);
        assert_eq!(objs[0].n_spectra, Some(3));
    }
}
