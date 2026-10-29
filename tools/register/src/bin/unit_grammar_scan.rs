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

fn field_words(rest: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut cur = String::new();
    let mut in_quote = false;
    for ch in rest.chars() {
        if ch == '"' {
            in_quote = !in_quote;
            cur.push(ch);
        } else if ch.is_whitespace() && !in_quote {
            if !cur.is_empty() {
                out.push(std::mem::take(&mut cur));
            }
        } else {
            cur.push(ch);
        }
    }
    if !cur.is_empty() {
        out.push(cur);
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
        if c == '.' || c == '/' || c == '(' || c == ')' {
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
    let mut numbers: Vec<(String, usize)> = Vec::new();
    let mut total = 0usize;
    for (n, line) in text.lines().enumerate() {
        let Some(rest) = line.trim().strip_prefix("field") else {
            continue;
        };
        if rest.is_empty() || !rest.starts_with(char::is_whitespace) {
            continue;
        }
        let cols = field_words(rest.trim_start());
        if cols.len() < 3 {
            continue;
        }
        total += 1;
        let mut unit = cols[2].as_str();
        for i in 2..cols.len().saturating_sub(1) {
            if omegaflow::mathematikerin::force::force_id_of(&cols[i]).is_some()
                || omegaflow::mathematikerin::force::quantity_kind_of(&cols[i]).is_some()
            {
                unit = cols[i + 1].as_str();
                break;
            }
        }
        if !seen.insert(unit.to_string()) {
            continue;
        }
        let clean = strip_annotations(unit);
        let prefix: String = clean
            .chars()
            .take_while(|c| {
                c.is_ascii_digit() || *c == '.' || *c == 'e' || *c == 'E' || *c == '-' || *c == '+'
            })
            .collect();
        let rest = &clean[prefix.len()..];
        let number_then_unit = !prefix.is_empty()
            && prefix != "1"
            && prefix.parse::<f64>().is_ok()
            && rest
                .chars()
                .next()
                .map(|c| c.is_ascii_alphabetic())
                .unwrap_or(false);
        if number_then_unit {
            numbers.push((unit.to_string(), n + 1));
            continue;
        }
        let mut ok = !clean.is_empty();
        if clean.is_empty() && unit.trim_start().starts_with('{') {
            ok = true;
        }
        for term in split_terms(&clean) {
            if term == "1" {
                continue;
            }
            if let Some(rest) = term.strip_prefix("10*") {
                if rest.parse::<i32>().is_ok() {
                    continue;
                }
                ok = false;
                break;
            }
            let (base, has_exp) = base_and_exp(&term);
            if base.is_empty()
                || (has_exp
                    && !term.contains('(')
                    && !term[base.len()..]
                        .chars()
                        .all(|c| c.is_ascii_digit() || c == '-' || c == '+'))
            {
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
    for (u, line) in &numbers {
        println!("  Zahl in der Einheit: {u} (line {line})");
    }
    println!(
        "{} of {} distinct units parse against UCUM 2.2",
        seen.len() - bad.len() - numbers.len(),
        seen.len()
    );
    std::process::exit(if bad.is_empty() && numbers.is_empty() {
        0
    } else {
        1
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    fn classify(unit: &str, atoms: &BTreeSet<String>, prefixes: &BTreeSet<String>) -> &'static str {
        let clean = strip_annotations(unit);
        let prefix: String = clean
            .chars()
            .take_while(|c| {
                c.is_ascii_digit() || *c == '.' || *c == 'e' || *c == 'E' || *c == '-' || *c == '+'
            })
            .collect();
        let rest = &clean[prefix.len()..];
        let number_then_unit = !prefix.is_empty()
            && prefix != "1"
            && prefix.parse::<f64>().is_ok()
            && rest
                .chars()
                .next()
                .map(|c| c.is_ascii_alphabetic())
                .unwrap_or(false);
        if number_then_unit {
            return "number";
        }
        let mut ok = !clean.is_empty();
        if clean.is_empty() && unit.trim_start().starts_with('{') {
            ok = true;
        }
        for term in split_terms(&clean) {
            if term == "1" {
                continue;
            }
            if let Some(rest) = term.strip_prefix("10*") {
                if rest.parse::<i32>().is_ok() {
                    continue;
                }
                ok = false;
                break;
            }
            let (base, has_exp) = base_and_exp(&term);
            if base.is_empty()
                || (has_exp
                    && !term.contains('(')
                    && !term[base.len()..]
                        .chars()
                        .all(|c| c.is_ascii_digit() || c == '-' || c == '+'))
            {
                ok = false;
                break;
            }
            if !unit_known(&base, atoms, prefixes) {
                ok = false;
                break;
            }
        }
        if ok { "valid" } else { "invalid" }
    }

    #[test]
    fn the_scanner_agrees_with_the_nlm_validator() {
        let dir = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");
        let xml = std::fs::read_to_string(format!("{dir}/src/archivar/kernels/ucum-essence.xml"))
            .expect("the frozen UCUM essence table");
        let tsv = std::fs::read_to_string(format!("{dir}/src/archivar/kernels/ucum_verdicts.tsv"))
            .expect("the NLM verdicts");
        let mut atoms: BTreeSet<String> = attr(&xml, "unit", "Code").into_iter().collect();
        atoms.extend(attr(&xml, "base-unit", "Code"));
        let prefixes: BTreeSet<String> = attr(&xml, "prefix", "Code").into_iter().collect();
        assert!(!atoms.is_empty() && !prefixes.is_empty());
        let mut n = 0usize;
        for line in tsv.lines() {
            if line.starts_with('#') || line.trim().is_empty() {
                continue;
            }
            let cols: Vec<&str> = line.split('\t').collect();
            assert!(cols.len() >= 2, "a verdict row carries code and status");
            let code = cols[0].trim();
            let nlm = cols[1].trim();
            let ours = classify(code, &atoms, &prefixes);
            let agree = (ours == "valid" && nlm == "valid")
                || (ours == "invalid" && nlm == "invalid")
                || (ours == "number" && nlm == "invalid");
            assert!(agree, "{code}: ours {ours}, NLM {nlm}");
            n += 1;
        }
        assert_eq!(n, 85, "85 distinct unit strings are compared");
    }
}
