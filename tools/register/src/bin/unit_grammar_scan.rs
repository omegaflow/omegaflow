use std::collections::BTreeSet;

fn attr(xml: &str, tag: &str, attr: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut rest = xml;
    let needle = format!("<{tag}");
    while let Some(p) = rest.find(&needle) {
        let after = &rest[p..];
        let end = match after.find('>') {
            Some(e) => e,
            None => break,
        };
        let head = &after[..end];
        if let Some(k) = head.find(&format!("{attr}=\"")) {
            let v = &head[k + attr.len() + 2..];
            if let Some(q) = v.find('"') {
                out.push(v[..q].to_string());
            }
        }
        rest = &after[end..];
    }
    out
}

fn strip_annotations(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut depth = 0i32;
    for c in s.chars() {
        match c {
            '{' => depth += 1,
            '}' => depth -= 1,
            _ if depth == 0 => out.push(c),
            _ => {}
        }
    }
    out
}

fn split_terms(s: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    for c in s.chars() {
        if c == '.' || c == '/' {
            if !cur.is_empty() {
                out.push(std::mem::take(&mut cur));
            }
        } else {
            cur.push(c);
        }
    }
    if !cur.is_empty() {
        out.push(cur);
    }
    out
}

fn base_and_exp(term: &str) -> (String, bool) {
    if term.starts_with('[') {
        match term.find(']') {
            Some(e) => return (term[..=e].to_string(), e + 1 < term.len()),
            None => return (term.to_string(), false),
        }
    }
    let mut cut = term.len();
    for (i, c) in term.char_indices() {
        if c.is_ascii_digit() || c == '-' || c == '+' || c == '(' {
            cut = i;
            break;
        }
    }
    (term[..cut].to_string(), cut < term.len())
}

fn unit_known(base: &str, atoms: &BTreeSet<String>, prefixes: &BTreeSet<String>) -> bool {
    if base == "1" {
        return true;
    }
    if atoms.contains(base) {
        return true;
    }
    for p in prefixes {
        if let Some(rest) = base.strip_prefix(p.as_str())
            && !rest.is_empty()
            && atoms.contains(rest)
        {
            return true;
        }
    }
    false
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let essence = match args.first() {
        Some(a) => a.clone(),
        None => "src/archivar/kernels/ucum-essence.xml".to_string(),
    };
    let register = match args.get(1) {
        Some(a) => a.clone(),
        None => "phi/sources.φ".to_string(),
    };
    let xml = match std::fs::read_to_string(&essence) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{essence}: {e}");
            std::process::exit(2);
        }
    };
    let mut atoms: BTreeSet<String> = attr(&xml, "unit", "Code").into_iter().collect();
    atoms.extend(attr(&xml, "base-unit", "Code"));
    let prefixes: BTreeSet<String> = attr(&xml, "prefix", "Code").into_iter().collect();
    if atoms.is_empty() || prefixes.is_empty() {
        eprintln!("{essence}: no units or prefixes found");
        std::process::exit(2);
    }
    let text = match std::fs::read_to_string(&register) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{register}: {e}");
            std::process::exit(2);
        }
    };
    let mut seen: BTreeSet<String> = BTreeSet::new();
    let mut bad: Vec<(String, usize)> = Vec::new();
    let mut total = 0usize;
    for (n, line) in text.lines().enumerate() {
        let Some(rest) = line.strip_prefix("field ") else {
            continue;
        };
        let cols: Vec<&str> = rest.split_whitespace().collect();
        if cols.len() < 3 {
            continue;
        }
        total += 1;
        let unit = cols[2];
        if !seen.insert(unit.to_string()) {
            continue;
        }
        let clean = strip_annotations(unit);
        let mut ok = !clean.is_empty();
        for term in split_terms(&clean) {
            if term == "1" {
                continue;
            }
            let (base, has_exp) = base_and_exp(&term);
            if base.is_empty() || (has_exp && !term.contains('(') && !term[base.len()..].chars().all(|c| c.is_ascii_digit() || c == '-' || c == '+')) {
                ok = false;
                break;
            }
            if !unit_known(&base, &atoms, &prefixes) {
                ok = false;
                break;
            }
        }
        if !ok {
            bad.push((unit.to_string(), n + 1));
        }
    }
    println!(
        "{register}: {total} field lines, {} distinct units, {} atoms, {} prefixes",
        seen.len(),
        atoms.len(),
        prefixes.len()
    );
    for (u, line) in &bad {
        println!("  not UCUM: {u} (line {line})");
    }
    println!(
        "{} of {} distinct units parse against UCUM 2.2",
        seen.len() - bad.len(),
        seen.len()
    );
    std::process::exit(if bad.is_empty() { 0 } else { 1 });
}
