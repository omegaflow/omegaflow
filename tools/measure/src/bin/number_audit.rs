
use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::path::PathBuf;

const TOL: f64 = 3.5e-4;

#[derive(Clone, Debug)]
struct Num {
    value: f64,
    raw: String,
    line: usize,
    table: bool,
    comma_decimal: bool,
    abstract_line: bool,
}

#[derive(Clone, Debug)]
struct Finding {
    rule: u8,
    text: String,
}

struct Report {
    nums: Vec<Num>,
    findings: Vec<Finding>,
    data_rows: usize,
    main_table_n: Option<usize>,
    table_count: usize,
}

impl Report {
    fn new() -> Self {
        Self {
            nums: Vec::new(),
            findings: Vec::new(),
            data_rows: 0,
            main_table_n: None,
            table_count: 0,
        }
    }
}

fn near(a: f64, b: f64) -> bool {
    if a == b {
        return true;
    }
    let scale = a.abs().max(b.abs());
    if scale == 0.0 {
        return false;
    }
    (a - b).abs() / scale <= TOL
}

fn expand_superscripts(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '⁰' => out.push('0'),
            '¹' => out.push('1'),
            '²' => out.push('2'),
            '³' => out.push('3'),
            '⁴' => out.push('4'),
            '⁵' => out.push('5'),
            '⁶' => out.push('6'),
            '⁷' => out.push('7'),
            '⁸' => out.push('8'),
            '⁹' => out.push('9'),
            '⁻' => out.push('-'),
            c => out.push(c),
        }
    }
    out
}

fn parse_number(tok: &str) -> Option<(f64, bool)> {
    let t = expand_superscripts(tok).replace('−', "-");
    let ch: Vec<char> = t.chars().collect();
    let mut i = 0usize;
    if i < ch.len() && (ch[i] == '+' || ch[i] == '-') {
        i += 1;
    }
    let neg = i > 0 && ch[0] == '-';
    let mut int = String::new();
    while i < ch.len() && ch[i].is_ascii_digit() {
        int.push(ch[i]);
        i += 1;
    }
    if int.is_empty() {
        return None;
    }

    loop {
        if i + 3 < ch.len()
            && (ch[i] == ' ' || ch[i] == '\u{a0}' || ch[i] == '\u{202f}')
            && ch[i + 1].is_ascii_digit()
            && ch[i + 2].is_ascii_digit()
            && ch[i + 3].is_ascii_digit()
        {
            int.push(ch[i + 1]);
            int.push(ch[i + 2]);
            int.push(ch[i + 3]);
            i += 4;
            continue;
        }
        break;
    }

    let mut groups: Vec<String> = Vec::new();
    let mut seps: Vec<char> = Vec::new();
    while i + 1 < ch.len() && (ch[i] == '.' || ch[i] == ',') && ch[i + 1].is_ascii_digit() {
        let sep = ch[i];
        i += 1;
        let mut g = String::new();
        while i < ch.len() && ch[i].is_ascii_digit() {
            g.push(ch[i]);
            i += 1;
        }
        seps.push(sep);
        groups.push(g);
    }

    let mut exp: i32 = 0;
    if i < ch.len() && (ch[i] == 'e' || ch[i] == 'E') {
        let save = i;
        let mut j = i + 1;
        let mut eneg = false;
        if j < ch.len() && (ch[j] == '+' || ch[j] == '-') {
            eneg = ch[j] == '-';
            j += 1;
        }
        let st = j;
        while j < ch.len() && ch[j].is_ascii_digit() {
            j += 1;
        }
        if j > st {
            let digits: String = ch[st..j].iter().collect();
            if let Ok(value) = digits.parse::<i32>() {
                exp = if eneg { -value } else { value };
                i = j;
            } else {
                i = save;
            }
        } else {
            i = save;
        }
    }
    let _ = i;

    let int_v: f64 = int.parse().ok()?;
    let mut value = int_v;
    let mut comma_decimal = false;

    if !groups.is_empty() {
        let comma_n = seps.iter().filter(|c| **c == ',').count();
        let dot_n = seps.len() - comma_n;

        if comma_n > 1 && !groups.iter().all(|g| g.len() == 3) {
            return None;
        }
        if comma_n > 0 && dot_n > 0 {
            return None;
        }
        if comma_n > 1 {
            let mut s = int.clone();
            for g in &groups {
                s.push_str(g);
            }
            value = s.parse().ok()?;
        } else if comma_n == 1 {
            let g = &groups[0];
            let thousands = g.len() == 3 && int.len() <= 3 && int != "0" && exp == 0;
            if thousands {
                let mut s = int.clone();
                s.push_str(g);
                value = s.parse().ok()?;
            } else {
                comma_decimal = true;
                let s = format!("{int}.{g}");
                value = s.parse().ok()?;
            }
        } else if dot_n == 1 {
            let s = format!("{}.{}", int, groups[0]);
            value = s.parse().ok()?;
        } else {
            return None;
        }
    }

    if exp != 0 {
        value *= 10f64.powi(exp);
    }
    if neg {
        value = -value;
    }
    if !value.is_finite() {
        return None;
    }
    Some((value, comma_decimal))
}

fn is_token_char(c: char) -> bool {
    c.is_alphanumeric()
        || matches!(
            c,
            '.' | ','
                | '-'
                | '+'
                | '/'
                | '%'
                | '°'
                | '×'
                | ':'
                | '⁰'
                | '¹'
                | '²'
                | '³'
                | '⁴'
                | '⁵'
                | '⁶'
                | '⁷'
                | '⁸'
                | '⁹'
                | '⁻'
        )
}

fn tokenize_spans(line: &str) -> Vec<(String, usize, usize)> {
    let chars: Vec<char> = line.chars().collect();
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < chars.len() {
        if !is_token_char(chars[i]) {
            i += 1;
            continue;
        }
        let start = i;
        while i < chars.len() && is_token_char(chars[i]) {
            i += 1;
        }
        let s: String = chars[start..i].iter().collect();
        if !s.is_empty() {
            out.push((s, start, i));
        }
    }
    out
}

fn is_table_line(line: &str) -> bool {
    let t = line.trim();
    t.starts_with('|') && t.matches('|').count() >= 2
}

fn is_separator_row(line: &str) -> bool {
    let t = line.trim();
    if !t.starts_with('|') {
        return false;
    }
    let cells: Vec<&str> = t.split('|').skip(1).collect();
    !cells.is_empty()
        && cells.iter().all(|c| {
            let c = c.trim();
            c.is_empty() || c.chars().all(|ch| ch == '-' || ch == ':' || ch == ' ')
        })
}

fn heading_text(line: &str) -> Option<String> {
    let t = line.trim_start();
    if !t.starts_with('#') {
        return None;
    }
    let level = t.chars().take_while(|c| *c == '#').count();
    if level == 0 || level > 6 {
        return None;
    }
    let rest = t[level..].trim();
    if rest.is_empty() {
        None
    } else {
        Some(rest.to_string())
    }
}

fn is_abstract_section(section: &str) -> bool {
    let s = section.to_lowercase();
    s.contains("zusammenfassung")
        || s.contains("abstract")
        || s.contains("kurzfassung")
        || s.contains("summary")
}

fn is_data_section(section: &str) -> bool {
    let s = section.to_lowercase();
    s.starts_with("2")
        && (s.starts_with("2.")
            || s.starts_with("2 ")
            || s.contains("daten")
            || s.contains("data"))
        || s.contains("daten")
}

fn is_identifier_line(line: &str) -> bool {
    let low = line.to_lowercase();
    [
        "arxiv", "docs/", "state/", "http", "sha256", "commit", ".md", ".rs", "doi",
        "jd ", "register:", "`",
    ]
    .iter()
    .any(|k| low.contains(k))
}

fn is_yearish(n: &Num) -> bool {
    n.value.fract() == 0.0
        && (1900.0..=2100.0).contains(&n.value)
        && !n.raw.contains('.')
        && !n.raw.contains(',')
}

fn analyze_text(text: &str) -> Report {
    let mut rep = Report::new();
    let mut nums: Vec<Num> = Vec::new();

    let lines: Vec<&str> = text.lines().collect();
    let mut table_flags = vec![false; lines.len()];
    let mut abstract_flags = vec![false; lines.len()];
    let mut table_groups: Vec<usize> = Vec::new(); // Datenzeilen je Tabelle
    let mut table_totals: Vec<usize> = Vec::new();

    let mut in_comment = false;
    let mut in_fence = false;
    let mut current_section = String::new();
    let mut seen_h1 = false;
    let mut lead = false;
    let mut in_table = false;
    let mut table_rows = 0usize;
    let mut table_has_header = false;
    let mut in_data_section = false;

    for (idx, raw_line) in lines.iter().enumerate() {
        let line = *raw_line;
        let trim = line.trim();

        if trim.contains("<!--") {
            in_comment = true;
        }
        if trim.ends_with("-->") || trim.contains("-->") {
            in_comment = false;
            continue;
        }
        if in_comment {
            continue;
        }
        if trim.starts_with("```") {
            in_fence = !in_fence;
            continue;
        }
        if in_fence {
            continue;
        }

        if let Some(h) = heading_text(line) {
            if in_table {
                table_groups.push(table_rows.saturating_sub(if table_has_header { 1 } else { 0 }));
                table_totals.push(table_rows);
                in_table = false;
            }
            let level = line.trim_start().chars().take_while(|c| *c == '#').count();
            current_section = h.clone();
            in_data_section = is_data_section(&current_section);
            if level == 1 {
                seen_h1 = true;
                lead = true;
            } else if seen_h1 {
                lead = false;
            }
            if is_abstract_section(&current_section) {
                abstract_flags[idx] = true;
            } else if lead {
                abstract_flags[idx] = true;
            }
            continue;
        }

        let table = is_table_line(line);
        if table {
            if !in_table {
                in_table = true;
                table_rows = 0;
                table_has_header = false;
            }
            table_flags[idx] = true;
            if !is_separator_row(line) {
                table_rows += 1;
                if table_rows == 1 {
                    table_has_header = true;
                }
            }
        } else if in_table {
            table_groups.push(table_rows.saturating_sub(if table_has_header { 1 } else { 0 }));
            table_totals.push(table_rows);
            in_table = false;
        }

        if lead || is_abstract_section(&current_section) {
            abstract_flags[idx] = true;
        }

        if trim.is_empty() {
            continue;
        }

        for (tok, _start, _end) in tokenize_spans(line) {
            if let Some((value, comma_decimal)) = parse_number(&tok) {
                nums.push(Num {
                    value,
                    raw: tok,
                    line: idx + 1,
                    table,
                    comma_decimal,
                    abstract_line: abstract_flags[idx] && !table,
                });
            }
        }
        let _ = in_data_section;
    }
    if in_table {
        table_groups.push(table_rows.saturating_sub(if table_has_header { 1 } else { 0 }));
        table_totals.push(table_rows);
    }

    let table_values: Vec<f64> = nums.iter().filter(|n| n.table).map(|n| n.value).collect();
    let data_rows: usize = table_groups.iter().sum();
    let main_table_n = table_groups.iter().copied().max();
    rep.data_rows = data_rows;
    rep.main_table_n = main_table_n;
    rep.table_count = table_groups.len();

    for n in nums.iter().filter(|n| n.abstract_line) {
        if n.value.abs() < 1e-12 || is_yearish(n) {
            continue;
        }
        if n.raw.contains('-') || n.raw.contains(':') || n.raw.contains('σ') {
            continue;
        }
        let core_letters = n.raw.chars().any(|c| c.is_alphabetic());
        if (n.raw.ends_with('.') || n.raw.ends_with(','))
            && n.value.fract() == 0.0
            && n.value.abs() <= 99.0
        {
            continue;
        }
        if core_letters && n.raw.matches('.').count() >= 2 {
            continue;
        }
        if n.value.abs() > 1e9 {
            continue;
        }
        let Some(line) = lines.get(n.line.saturating_sub(1)) else {
            continue;
        };
        let line_low = line.to_lowercase();
        if is_identifier_line(&line_low) {
            continue;
        }
        if line_low.contains('§') {
            continue;
        }
        if n.value.abs() <= 31.0 && line_low.contains("2026-") && !n.raw.contains('.') {
            continue;
        }
        let anchored_table = table_values.iter().any(|tv| near(*tv, n.value));
        let anchored_register = line_low.contains("register");
        if anchored_table || anchored_register {
            continue;
        }
        rep.findings.push(Finding {
            rule: 1,
            text: format!(
                "R1 l.{} '{}' — abstract/lead number without table/register mark",
                n.line, n.raw
            ),
        });
    }

    let mut r2_seen: BTreeSet<u64> = BTreeSet::new();
    for (idx, line) in lines.iter().enumerate() {
        if table_flags[idx] || is_table_line(line) || heading_text(line).is_some() {
            continue;
        }
        if is_separator_row(line) {
            continue;
        }
        let line_low = line.to_lowercase();
        if line_low.contains("min-n")
            || line_low.contains("min n")
            || line_low.contains("unter n")
            || line_low.contains("mindest")
            || line_low.contains("schwelle")
            || line_low.contains("threshold")
        {
            continue;
        }
        let toks = tokenize_spans(line);
        let section = {
            let mut s = String::new();
            for prior in lines[..=idx].iter().rev() {
                if let Some(h) = heading_text(prior) {
                    s = h;
                    break;
                }
            }
            s
        };
        let in_data = is_data_section(&section);
        for w in toks.windows(2) {
            let label = w[0].0.as_str();
            let upper = label == "N";
            let lower = label == "n" && in_data;
            if !upper && !lower {
                continue;
            }
            let Some((value, _)) = parse_number(&w[1].0) else {
                continue;
            };
            if value.fract() != 0.0 || value < 0.0 || value > 1e12 {
                continue;
            }
            let v = value as u64;
            if !r2_seen.insert(v) {
                continue;
            }
            if table_groups.iter().any(|g| *g as u64 == v) {
                continue;
            }
            let text = if table_groups.is_empty() {
                format!(
                    "R2 l.{} '{} = {}' — §2 count without table (no table n)",
                    idx + 1,
                    label,
                    v
                )
            } else {
                format!(
                    "R2 l.{} '{} = {}' — §2 count ≠ table n (main table {} data rows)",
                    idx + 1,
                    label,
                    v,
                    main_table_n.map_or_else(|| "pending".to_string(), |n| n.to_string())
                )
            };
            rep.findings.push(Finding {
                rule: 2,
                text,
            });
        }
    }

    #[derive(Clone)]
    struct Row {
        label: String,
        nums: Vec<f64>,
        line: usize,
    }
    let mut rows: Vec<Row> = Vec::new();
    for (idx, line) in lines.iter().enumerate() {
        if !table_flags[idx] || is_separator_row(line) {
            continue;
        }
        let cells: Vec<&str> = line.trim().split('|').map(|c| c.trim()).collect();
        let mut cells = cells.into_iter().filter(|c| !c.is_empty()).peekable();
        let Some(label_raw) = cells.next() else {
            continue;
        };
        if label_raw.is_empty() || label_raw.chars().any(|c| c.is_ascii_digit()) {
            continue;
        }
        let label = label_raw.to_lowercase();
        if label.len() < 4 {
            continue;
        }
        let mut vals = Vec::new();
        for c in cells {
            if let Some((v, _)) = parse_number(c) {
                vals.push(v);
            }
        }
        if vals.is_empty() {
            continue;
        }
        rows.push(Row {
            label,
            nums: vals,
            line: idx + 1,
        });
    }
    let mut by_label: BTreeMap<String, Vec<Row>> = BTreeMap::new();
    for r in rows {
        by_label.entry(r.label.clone()).or_insert_with(Vec::new).push(r);
    }
    let mut r3_reported: BTreeSet<String> = BTreeSet::new();
    for (label, group) in &by_label {
        if group.len() < 2 {
            continue;
        }
        'outer: for i in 0..group.len() {
            for j in (i + 1)..group.len() {
                let a = &group[i];
                let b = &group[j];
                for k in 0..a.nums.len().min(b.nums.len()) {
                    if !near(a.nums[k], b.nums[k]) {
                        if r3_reported.insert(label.clone()) {
                            rep.findings.push(Finding {
                                rule: 3,
                                text: format!(
                                    "R3 l.{}/{} '{}' — double count: {} vs {}",
                                    a.line, b.line, label, a.nums[k], b.nums[k]
                                ),
                            });
                        }
                        break 'outer;
                    }
                }
            }
        }
    }

    let comma: Vec<&Num> = nums
        .iter()
        .filter(|n| {
            if !n.comma_decimal {
                return false;
            }
            let Some(line) = lines.get(n.line.saturating_sub(1)) else {
                return false;
            };
            let line_low = line.to_lowercase();
            if line_low.contains("lag")
                && n.value.abs() < 100.0
                && n.raw
                    .split(',')
                    .all(|p| p.chars().filter(|c| c.is_ascii_digit()).count() <= 2)
            {
                return false;
            }
            true
        })
        .collect();
    if !comma.is_empty() {
        let dot = nums
            .iter()
            .filter(|n| !n.comma_decimal && n.raw.contains('.'))
            .count();
        let first = comma[0];
        let text = if dot > 0 {
            format!(
                "R4 l.{} '{}' — mixed comma locale: {} decimal commas vs {} point decimals",
                first.line,
                first.raw,
                comma.len(),
                dot
            )
        } else {
            format!(
                "R4 l.{} '{}' — comma locale: {} decimal commas, no point decimals",
                first.line,
                first.raw,
                comma.len()
            )
        };
        rep.findings.push(Finding {
            rule: 4,
            text,
        });
    }

    rep.nums = nums;
    rep
}

fn find_blatt_dir(args: &[String]) -> (PathBuf, String) {
    let explicit: Vec<PathBuf> = args.iter().filter(|a| !a.starts_with('-')).map(PathBuf::from).collect();
    if !explicit.is_empty() {
        let p = explicit[0].clone();
        return (p, "Argument".to_string());
    }
    let mut candidates = vec![
        PathBuf::from("docs/blatt"),
        PathBuf::from("../docs/blatt"),
        PathBuf::from("../omegaflow/docs/blatt"),
        PathBuf::from("../../../docs/blatt"),
    ];
    candidates.dedup();
    for c in candidates {
        if c.is_dir() {
            return (c, "Standardpfad".to_string());
        }
    }
    (PathBuf::from("docs/blatt"), "not found".to_string())
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let (dir, origin) = find_blatt_dir(&args);

    let mut files: Vec<PathBuf> = match fs::read_dir(&dir) {
        Ok(rd) => rd
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.extension().map(|x| x == "md").unwrap_or(false))
            .collect(),
        Err(_) => Vec::new(),
    };
    files.sort();

    println!("=== number audit — std-only (R1 anchor, R2 table n, R3 double count, R4 comma locale) ===");
    println!("sheet directory: {} ({origin})", dir.display());
    if files.is_empty() {
        println!("No sheet readable — the audit measures nothing (0 honored).");
        return;
    }

    let mut total = 0usize;
    let mut per_rule = [0usize; 5];
    let mut total_nums = 0usize;
    let mut total_tables = 0usize;

    for f in &files {
        let name = match f.file_name() {
            Some(s) => s.to_string_lossy().into_owned(),
            None => f.to_string_lossy().into_owned(),
        };
        let text = match fs::read_to_string(f) {
            Ok(t) => t,
            Err(_) => {
                println!("\n## {name}\n  not readable — 0 honored");
                continue;
            }
        };
        let rep = analyze_text(&text);
        let n = rep.findings.len();
        total += n;
        total_nums += rep.nums.len();
        total_tables += rep.table_count;
        for finding in &rep.findings {
            per_rule[finding.rule as usize] += 1;
        }

        println!("\n## {name}");
        println!(
            "  numbers: {} · tables: {} (main n = {}, total data rows = {})",
            rep.nums.len(),
            rep.table_count,
            rep.main_table_n.map_or_else(|| "pending".to_string(), |n| n.to_string()),
            rep.data_rows
        );
        if rep.nums.is_empty() {
            println!("  no number — the sheet does not measure");
        }
        for finding in &rep.findings {
            println!("  {}", finding.text);
        }
        println!("  [findings = {n}]");
    }

    println!("\n=== Total ===");
    println!("  sheets = {}", files.len());
    println!("  numbers read = {total_nums} · tables = {total_tables}");
    println!("  findings = {total}");
    println!(
        "  R1 = {} · R2 = {} · R3 = {} · R4 = {}",
        per_rule[1], per_rule[2], per_rule[3], per_rule[4]
    );
    println!(
        "  The number per sheet is the finding count; there is no hard threshold — only the rule."
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_scientific_and_comma() {
        assert!(near(parse_number("3.91e-2").unwrap().0, 0.0391));
        assert!(near(parse_number("1,5e-1").unwrap().0, 0.15));
        assert!(parse_number("1,5e-1").unwrap().1);
        assert!(near(parse_number("0,9771").unwrap().0, 0.9771));
        assert!(near(parse_number("5 862 322").unwrap().0, 5862322.0));
        assert!(near(parse_number("12,074").unwrap().0, 12074.0));
    }

    #[test]
    fn lag_list_is_not_a_number() {
        assert!(parse_number("1,6,12,24").is_none());
    }

    #[test]
    fn r1_fires_on_unanchored_lead_number() {
        let text = "# T\n\nDie Zahl 1392 steht ohne Tabelle.\n\n## Daten\n| a | b |\n|---|---|\n| x | 3.5 |\n";
        let rep = analyze_text(text);
        assert!(rep.findings.iter().any(|f| f.rule == 1));
    }

    #[test]
    fn r1_silent_when_table_carries_the_number() {
        let text = "# T\n\nDie Zahl 1392 steht hier.\n\n## Daten\n| n | wert |\n|---|---|\n| 1392 | 3.5 |\n";
        let rep = analyze_text(text);
        assert!(!rep.findings.iter().any(|f| f.rule == 1));
    }

    #[test]
    fn r2_fires_on_mismatch() {
        let text = "# T\n\n## 2. Daten\nN = 7\n\n## Tabelle\n| a | b |\n|---|---|\n| x | 1 |\n| y | 2 |\n";
        let rep = analyze_text(text);
        assert!(rep.findings.iter().any(|f| f.rule == 2));
    }

    #[test]
    fn r2_silent_on_match() {
        let text = "# T\n\n## 2. Daten\nN = 2\n\n## Tabelle\n| a | b |\n|---|---|\n| x | 1 |\n| y | 2 |\n";
        let rep = analyze_text(text);
        assert!(!rep.findings.iter().any(|f| f.rule == 2));
    }

    #[test]
    fn r3_fires_on_same_label_two_values() {
        let text = "# T\n\n## Tabelle\n| records | wert |\n|---|---|\n| records | 1663 |\n| records | 1666 |\n";
        let rep = analyze_text(text);
        assert!(rep.findings.iter().any(|f| f.rule == 3));
    }

    #[test]
    fn r4_fires_on_comma_locale() {
        let text = "# T\n\n## Daten\nDer Wert ist 492,0 s gegen 487.7 s.\n";
        let rep = analyze_text(text);
        assert!(rep.findings.iter().any(|f| f.rule == 4));
    }
}
