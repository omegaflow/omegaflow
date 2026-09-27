use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::cdn::upload_release;
use omegaflow::json::{JsonVal, jnum, jpath_val, jstr, parse_json};

const API: &str = "http://occultations.ct.utfpr.edu.br/api/events";
const NETLOC: &str = "occultations.ct.utfpr.edu.br";
const PAGE_SIZE: usize = 100;
const MAX_PAGES: usize = 64;
const DEFAULT_OUT: &str = "phi/occultation_harvest/occultation_utfr_events.json";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn json_str(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

fn sexa_to_deg(raw: &str, is_ra: bool) -> Option<f64> {
    let raw = raw.trim();
    if raw.is_empty() {
        return None;
    }
    let normalized = raw.replace('\u{2212}', "-");
    let (neg, body) = match normalized.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => match normalized.strip_prefix('+') {
            Some(rest) => (false, rest),
            None => (false, normalized.as_str()),
        },
    };
    let toks: Vec<&str> = body.split_whitespace().collect();
    if toks.len() < 2 || toks.len() > 3 {
        return None;
    }
    let d: f64 = toks[0].parse().ok()?;
    let m: f64 = toks[1].parse().ok()?;
    let s: f64 = match toks.get(2) {
        Some(t) => t.replace(',', ".").parse().ok()?,
        None => 0.0,
    };
    if !(d.is_finite() && m.is_finite() && s.is_finite()) {
        return None;
    }
    let mut v = d + m / 60.0 + s / 3600.0;
    if neg {
        v = -v;
    }
    Some(if is_ra { v * 15.0 } else { v })
}

fn num_part(parts: &mut Vec<(String, String)>, key: &str, val: Option<f64>, positive: bool) {
    if let Some(x) = val
        && x.is_finite()
        && (!positive || x > 0.0)
    {
        parts.push((key.to_string(), format!("{}", x)));
    }
}

fn local_str(v: &JsonVal) -> Option<String> {
    let Some(JsonVal::Arr(arr)) = jpath_val(v, "local") else {
        return None;
    };
    let items: Vec<String> = arr
        .iter()
        .filter_map(|e| match e {
            JsonVal::Str(s) if !s.trim().is_empty() => Some(s.clone()),
            _ => None,
        })
        .collect();
    if items.is_empty() {
        None
    } else {
        Some(items.join(","))
    }
}

struct Stats {
    rows_in: usize,
    rows_out: usize,
    ra_void: usize,
    dec_void: usize,
    delta_void: usize,
}

fn build_row(v: &JsonVal, stats: &mut Stats) -> Option<String> {
    stats.rows_in += 1;
    let ra = jstr(v, "ra_position").and_then(|s| sexa_to_deg(&s, true));
    let Some(ra) = ra.filter(|r| r.is_finite() && *r >= 0.0 && *r < 360.0) else {
        stats.ra_void += 1;
        return None;
    };
    let dec = jstr(v, "dec_position").and_then(|s| sexa_to_deg(&s, false));
    let Some(dec) = dec.filter(|d| d.is_finite() && *d >= -90.0 && *d <= 90.0) else {
        stats.dec_void += 1;
        return None;
    };
    let Some(delta_au) = jnum(v, "delta_au").filter(|d| d.is_finite() && *d > 0.0) else {
        stats.delta_void += 1;
        return None;
    };
    let mut parts: Vec<(String, String)> = Vec::new();
    if let Some(name) = jstr(v, "name").filter(|s| !s.trim().is_empty()) {
        parts.push(("name".into(), json_str(&name)));
    }
    parts.push(("ra".into(), format!("{}", ra)));
    parts.push(("dec".into(), format!("{}", dec)));
    parts.push(("delta_au".into(), format!("{}", delta_au)));
    num_part(&mut parts, "number", jnum(v, "number"), true);
    num_part(&mut parts, "chords", jnum(v, "chords"), true);
    num_part(
        &mut parts,
        "equivalent_radius",
        jnum(v, "equivalent_radius"),
        true,
    );
    num_part(
        &mut parts,
        "equatorial_radius",
        jnum(v, "equatorial_radius"),
        true,
    );
    num_part(&mut parts, "albedo", jnum(v, "albedo"), true);
    num_part(&mut parts, "density", jnum(v, "density"), true);
    num_part(&mut parts, "oblateness", jnum(v, "oblateness"), false);
    num_part(
        &mut parts,
        "semi_major_axis",
        jnum(v, "semi_major_axis"),
        true,
    );
    num_part(&mut parts, "eccentricity", jnum(v, "eccentricity"), false);
    num_part(&mut parts, "inclination", jnum(v, "inclination"), false);
    num_part(
        &mut parts,
        "object_g_magnitude",
        jnum(v, "object_g_magnitude"),
        false,
    );
    num_part(
        &mut parts,
        "star_g_magnitude",
        jnum(v, "star_g_magnitude"),
        false,
    );
    num_part(&mut parts, "position_jd", jnum(v, "position_date"), true);
    if let Some(date) = jstr(v, "date").filter(|s| !s.trim().is_empty()) {
        parts.push(("date".into(), json_str(&date)));
    }
    if let Some(dc) = jstr(v, "dynamic_class").filter(|s| !s.trim().is_empty()) {
        parts.push(("dynamic_class".into(), json_str(&dc)));
    }
    if let Some(cat) = jstr(v, "category").filter(|s| !s.trim().is_empty()) {
        parts.push(("category".into(), json_str(&cat)));
    }
    if let Some(link) = jstr(v, "link").filter(|s| !s.trim().is_empty()) {
        parts.push(("link".into(), json_str(&link)));
    }
    if let Some(local) = local_str(v) {
        parts.push(("local".into(), json_str(&local)));
    }
    stats.rows_out += 1;
    Some(emit(&parts))
}

fn emit(parts: &[(String, String)]) -> String {
    let mut out = String::from("{");
    for (k, (key, val)) in parts.iter().enumerate() {
        if k > 0 {
            out.push(',');
        }
        out.push_str(&json_str(key));
        out.push(':');
        out.push_str(val);
    }
    out.push('}');
    out
}

fn fetch_page(page: usize) -> Option<JsonVal> {
    let url = format!("{API}?page={page}&pageSize={PAGE_SIZE}");
    let body = fetch_raw_bytes(&url)?;
    let text = String::from_utf8_lossy(&body);
    parse_json(&text)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = match arg_value(&args, "--out") {
        Some(v) => v,
        None => DEFAULT_OUT.to_string(),
    };

    let mut stats = Stats {
        rows_in: 0,
        rows_out: 0,
        ra_void: 0,
        dec_void: 0,
        delta_void: 0,
    };
    let mut rows_out: Vec<String> = Vec::new();
    let mut page = 1usize;
    let mut seen = 0usize;
    let mut total: Option<usize> = None;
    loop {
        let Some(json) = fetch_page(page) else {
            eprintln!("{API} page {page}: json void — the page stays unread");
            break;
        };
        if page == 1 {
            total = jnum(&json, "total")
                .filter(|t| t.is_finite() && *t > 0.0)
                .map(|t| t as usize);
        }
        let Some(JsonVal::Arr(arr)) = jpath_val(&json, "rows") else {
            eprintln!("{API} page {page}: rows void — the page carries no array");
            break;
        };
        if arr.is_empty() {
            break;
        }
        let n = arr.len();
        seen += n;
        for v in arr {
            if let Some(row) = build_row(v, &mut stats) {
                rows_out.push(row);
            }
        }
        if n < PAGE_SIZE {
            break;
        }
        if let Some(t) = total
            && seen >= t
        {
            break;
        }
        page += 1;
        if page > MAX_PAGES {
            break;
        }
    }

    if rows_out.is_empty() {
        eprintln!(
            "{API}: {} rows carried no resolvable RA/Dec/delta_au — the harvest stays unwritten (0 honored)",
            stats.rows_in
        );
        std::process::exit(1);
    }

    let mut buf = String::from("[");
    for (k, row) in rows_out.iter().enumerate() {
        if k > 0 {
            buf.push_str(",\n");
        }
        buf.push_str(row);
    }
    buf.push_str("]\n");

    if let Some(parent) = std::path::Path::new(&out).parent()
        && !parent.as_os_str().is_empty()
        && let Err(err) = std::fs::create_dir_all(parent)
    {
        eprintln!("mkdir {} returned void: {}", parent.display(), err);
        std::process::exit(1);
    }
    if let Err(err) = std::fs::write(&out, buf.as_bytes()) {
        eprintln!("write {} returned void: {}", out, err);
        std::process::exit(1);
    }
    eprintln!(
        "occultation_compiler: {}/{} rows → {} ({} B) — ra void {}, dec void {}, delta void {}",
        stats.rows_out,
        stats.rows_in,
        out,
        buf.len(),
        stats.ra_void,
        stats.dec_void,
        stats.delta_void,
    );
    if ci_mode && !upload_release(NETLOC, &out) {
        eprintln!("upload: {} did not reach the CDN", out);
        std::process::exit(1);
    }
}
