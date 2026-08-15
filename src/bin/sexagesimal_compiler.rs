// TeVCat2 JSON array + McGill magnetar TabO1.csv → flat cmap catalog.
// Sexagesimal RA (hours)/Decl (degrees) → decimal degrees via omegaflow::sexagesimal.
// Rows: {"name","ra","dec"[,"dist"<kpc>][,"period"][,"flux"][,"index"]}.
// upload via --ci-mode (tag ssd.jpl.nasa.gov).

use omegaflow::cdn::upload_asset;
use omegaflow::sexagesimal::{sexagesimal_dec_to_deg, sexagesimal_ra_to_deg};
use std::io::Write;

fn json_escape(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

fn parse_kpc(s: &str) -> Option<f64> {
    let t = s.trim();
    if t.is_empty() || t == "null" {
        return None;
    }
    let num_end = t
        .char_indices()
        .find(|(_, c)| {
            !(c.is_ascii_digit() || *c == '.' || *c == '-' || *c == '+' || *c == 'e' || *c == 'E')
        })
        .map(|(i, _)| i)
        .unwrap_or(t.len());
    let val: f64 = t[..num_end].trim().parse().ok()?;
    let unit = t[num_end..].trim();
    match unit {
        "" | "kpc" => Some(val),
        "pc" => Some(val / 1000.0),
        _ => None,
    }
}

fn get_field(element: &str, key: &str) -> Option<String> {
    let needle = format!("\"{}\":", key);
    let i = element.find(&needle)?;
    let rest = element[i + needle.len()..].trim_start();
    if rest.starts_with('"') {
        let end = rest[1..].find('"')?;
        Some(rest[1..1 + end].to_string())
    } else if rest.starts_with("null") {
        None
    } else {
        let end = rest
            .find(|c: char| c == ',' || c == '}' || c == ']')
            .unwrap_or(rest.len());
        Some(rest[..end].trim().to_string())
    }
}

fn split_array(body: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut depth = 0usize;
    let mut in_str = false;
    let mut esc = false;
    let mut start = 0usize;
    let b = body.as_bytes();
    let mut i = 0usize;
    while i < b.len() {
        let c = b[i] as char;
        if in_str {
            if esc {
                esc = false;
            } else if c == '\\' {
                esc = true;
            } else if c == '"' {
                in_str = false;
            }
        } else {
            match c {
                '"' => in_str = true,
                '[' | '{' => depth += 1,
                ']' | '}' => depth -= 1,
                ',' if depth == 1 => {
                    out.push(body[start..i].trim().to_string());
                    start = i + 1;
                }
                _ => {}
            }
        }
        i += 1;
    }
    if depth == 0 && start < body.len() {
        let tail = body[start..].trim();
        if tail.starts_with('{') {
            out.push(tail.to_string());
        }
    }
    out
}

fn tevcat(input: &str, out_path: &str, probe: Option<&str>) -> usize {
    let body = match std::fs::read_to_string(input) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("read {}: {}", input, e);
            std::process::exit(1);
        }
    };
    let elements = split_array(&body);
    let mut rows = Vec::new();
    let mut skipped = 0usize;
    for el in &elements {
        let name = match get_field(el, "name") {
            Some(n) => n,
            None => {
                skipped += 1;
                continue;
            }
        };
        let ra = match get_field(el, "ra").and_then(|s| sexagesimal_ra_to_deg(&s)) {
            Some(v) => v,
            None => {
                skipped += 1;
                continue;
            }
        };
        let dec = match get_field(el, "dec").and_then(|s| sexagesimal_dec_to_deg(&s)) {
            Some(v) => v,
            None => {
                skipped += 1;
                continue;
            }
        };
        let dist = get_field(el, "distance").and_then(|s| parse_kpc(&s));
        let flux = get_field(el, "flux").and_then(|s| s.parse::<f64>().ok());
        let index = get_field(el, "index").and_then(|s| s.parse::<f64>().ok());
        let mut row = format!(
            "{{\"name\":\"{}\",\"ra\":{},\"dec\":{}",
            json_escape(&name),
            ra,
            dec
        );
        if let Some(d) = dist {
            row.push_str(&format!(",\"dist\":{}", d));
        }
        if let Some(f) = flux {
            row.push_str(&format!(",\"flux\":{}", f));
        }
        if let Some(ix) = index {
            row.push_str(&format!(",\"index\":{}", ix));
        }
        row.push('}');
        rows.push(row);
    }
    if let Some(p) = probe {
        for r in &rows {
            if r.contains(p) {
                eprintln!("probe: {}", r);
                return rows.len();
            }
        }
        eprintln!("probe: {} not present", p);
        return rows.len();
    }
    write_rows(&rows, out_path, "tevcat", skipped)
}

fn split_csv(line: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut in_quotes = false;
    for c in line.chars() {
        match c {
            '"' => in_quotes = !in_quotes,
            ',' if !in_quotes => {
                out.push(cur.trim().to_string());
                cur.clear();
            }
            _ => cur.push(c),
        }
    }
    out.push(cur.trim().to_string());
    out
}

fn magnetar(input: &str, out_path: &str, probe: Option<&str>) -> usize {
    let body = match std::fs::read_to_string(input) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("read {}: {}", input, e);
            std::process::exit(1);
        }
    };
    let mut lines = body.lines();
    let header = match lines.next() {
        Some(h) => h,
        None => {
            eprintln!("header absent");
            std::process::exit(1);
        }
    };
    let cols: Vec<String> = split_csv(header);
    let idx = |name: &str| -> Option<usize> { cols.iter().position(|c| c == name) };
    let (i_name, i_ra, i_dec, i_dist, i_period, i_flux) = match (
        idx("Name"),
        idx("RA"),
        idx("Decl"),
        idx("Dist"),
        idx("Period"),
        idx("Flux"),
    ) {
        (Some(a), Some(b), Some(c), Some(d), Some(e), Some(f)) => (a, b, c, d, e, f),
        _ => {
            eprintln!("column layout unrecognized");
            std::process::exit(1);
        }
    };
    let mut rows = Vec::new();
    let mut skipped = 0usize;
    for line in lines {
        let fields = split_csv(line);
        if fields.len() <= i_dec {
            skipped += 1;
            continue;
        }
        let name = fields[i_name].trim().to_string();
        let ra = match sexagesimal_ra_to_deg(&fields[i_ra]) {
            Some(v) => v,
            None => {
                skipped += 1;
                continue;
            }
        };
        let dec = match sexagesimal_dec_to_deg(&fields[i_dec]) {
            Some(v) => v,
            None => {
                skipped += 1;
                continue;
            }
        };
        let dist = fields[i_dist].trim().parse::<f64>().ok();
        let period = fields[i_period].trim().parse::<f64>().ok();
        let flux = fields[i_flux].trim().parse::<f64>().ok();
        let mut row = format!(
            "{{\"name\":\"{}\",\"ra\":{},\"dec\":{}",
            json_escape(&name),
            ra,
            dec
        );
        if let Some(d) = dist {
            row.push_str(&format!(",\"dist\":{}", d));
        }
        if let Some(p) = period {
            row.push_str(&format!(",\"period\":{}", p));
        }
        if let Some(f) = flux {
            row.push_str(&format!(",\"flux\":{}", f));
        }
        row.push('}');
        rows.push(row);
    }
    if let Some(p) = probe {
        for r in &rows {
            if r.contains(p) {
                eprintln!("probe: {}", r);
                return rows.len();
            }
        }
        eprintln!("probe: {} not present", p);
        return rows.len();
    }
    write_rows(&rows, out_path, "magnetar", skipped)
}

fn write_rows(rows: &[String], out_path: &str, label: &str, skipped: usize) -> usize {
    let mut buf = String::from("[");
    for (k, r) in rows.iter().enumerate() {
        if k > 0 {
            buf.push(',');
        }
        buf.push_str(r);
    }
    buf.push_str("]\n");
    match std::fs::File::create(out_path).and_then(|mut f| f.write_all(buf.as_bytes())) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("write {}: {}", out_path, e);
            std::process::exit(1);
        }
    }
    eprintln!(
        "{}: {} records written, {} skipped, {} B → {}",
        label,
        rows.len(),
        skipped,
        buf.len(),
        out_path
    );
    rows.len()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!(
            "usage: sexagesimal_compiler --source tevcat|magnetar --input <file> --out <flat.json> [--ci-mode] [--probe <name>]"
        );
        std::process::exit(1);
    }
    let mut source: Option<String> = None;
    let mut input: Option<String> = None;
    let mut out: Option<String> = None;
    let mut ci_mode = false;
    let mut probe: Option<String> = None;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--source" => {
                source = args.get(i + 1).cloned();
                i += 1;
            }
            "--input" => {
                input = args.get(i + 1).cloned();
                i += 1;
            }
            "--out" => {
                out = args.get(i + 1).cloned();
                i += 1;
            }
            "--ci-mode" => ci_mode = true,
            "--probe" => {
                probe = args.get(i + 1).cloned();
                i += 1;
            }
            _ => {}
        }
        i += 1;
    }
    let input = match input {
        Some(p) => p,
        None => {
            eprintln!("--input absent");
            std::process::exit(1);
        }
    };
    let out_path = match out {
        Some(p) => p,
        None => {
            eprintln!("--out absent");
            std::process::exit(1);
        }
    };
    match source.as_deref() {
        Some("tevcat") => {
            tevcat(&input, &out_path, probe.as_deref());
        }
        Some("magnetar") => {
            magnetar(&input, &out_path, probe.as_deref());
        }
        other => {
            eprintln!("--source {:?} absent from {{tevcat, magnetar}}", other);
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_asset(&out_path) {
        eprintln!("upload: {} did not reach the CDN", out_path);
        std::process::exit(1);
    }
}
