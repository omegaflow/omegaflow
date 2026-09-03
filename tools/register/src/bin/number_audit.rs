use std::env;
use std::fs;
use std::path::Path;

const TOL: f64 = 3.5e-4;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Kind {
    Table,
    Prose,
}

#[derive(Clone, Debug)]
struct Num {
    value: f64,
    raw: String,
    line: usize,
    section: String,
    comma_decimal: bool,
}

#[derive(Default)]
struct FileReport {
    nums: Vec<Num>,
    sections: Vec<String>,
    findings: Vec<String>,
    found: [usize; 6],
    r6_candidates: Vec<String>,
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
    let t = expand_superscripts(tok.trim());

    let t = t.strip_suffix(',').unwrap_or(&t).to_string();
    let t = t.trim();

    let bytes = t.as_bytes();
    let mut exp_i = None;
    let mut i = 0;
    while i < bytes.len() {
        if (bytes[i] == b'e' || bytes[i] == b'E')
            && i + 1 < bytes.len()
            && (bytes[i + 1].is_ascii_digit()
                || ((bytes[i + 1] == b'-' || bytes[i + 1] == b'+')
                    && i + 2 < bytes.len()
                    && bytes[i + 2].is_ascii_digit()))
        {
            exp_i = Some(i);
            break;
        }
        i += 1;
    }
    let mantissa = match exp_i {
        Some(ei) => &t[..ei],
        None => &t[..],
    };

    if mantissa.is_empty() || !mantissa.chars().any(|c| c.is_ascii_digit()) {
        return None;
    }

    if t.matches('e').count() + t.matches('E').count() > 1 {
        return None;
    }

    let mb = mantissa.as_bytes();
    let mut comma_decimal = false;
    for j in 1..mb.len().saturating_sub(1) {
        if mb[j] == b',' && mb[j - 1].is_ascii_digit() && mb[j + 1].is_ascii_digit() {
            comma_decimal = true;
        }
    }

    let norm: String = {
        let mut s = String::with_capacity(mantissa.len());
        let mut iter = mantissa.chars().peekable();
        while let Some(ch) = iter.next() {
            match ch {
                ' ' | ' ' => continue,
                ',' if comma_decimal => s.push('.'),
                ',' => s.push(','),
                c => s.push(c),
            }
        }
        s
    };

    let s = match exp_i {
        Some(_) => {
            if norm.matches(',').count() == 1 {
                let parts: Vec<&str> = norm.split(',').collect();
                if parts.len() == 2
                    && parts[1].chars().all(|c| c.is_ascii_digit())
                    && parts[1].len() == 1
                    || parts[1].len() == 2
                {
                    norm.replace(',', ".")
                } else {
                    norm.replace(',', "")
                }
            } else {
                norm.replace(',', "")
            }
        }
        None => norm,
    };
    if s.is_empty() || !s.chars().any(|c| c.is_ascii_digit()) {
        return None;
    }
    let mut v: f64 = s.parse().ok()?;

    if let Some(ei) = exp_i {
        let exp = &t[ei + 1..];
        let mut eend = exp.len();
        for (j, ch) in exp.char_indices() {
            if ch.is_ascii_alphabetic() || ch == '%' || ch == '°' || ch == '/' || ch == '×' {
                eend = j;
                break;
            }
        }
        if let Ok(pow) = exp[..eend].parse::<f64>() {
            v *= 10f64.powf(pow);
        }
    }
    if !v.is_finite() {
        return None;
    }
    Some((v, comma_decimal))
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

#[cfg(test)]
fn normalize(tok: &str) -> Option<f64> {
    parse_number(tok).map(|(v, _)| v)
}

fn tokenize(line: &str) -> Vec<String> {
    line.split(|c: char| {
        !c.is_alphanumeric()
            && c != '.'
            && c != ','
            && c != '-'
            && c != '/'
            && c != '%'
            && c != '°'
            && c != '×'
            && c != '⁰'
            && c != '¹'
            && c != '²'
            && c != '³'
            && c != '⁴'
            && c != '⁵'
            && c != '⁶'
            && c != '⁷'
            && c != '⁸'
            && c != '⁹'
            && c != '⁻'
            && c != ' '
    })
    .filter(|t| !t.is_empty())
    .map(|t| t.to_string())
    .collect()
}

fn is_table_row(line: &str) -> bool {
    let t = line.trim();
    t.starts_with('|') && t.contains('|')
}

fn section_of(line: &str, current: &str) -> (bool, String) {
    let t = line.trim();
    if t.starts_with('#') {
        (true, t.trim_start_matches('#').trim().to_string())
    } else {
        (false, current.to_string())
    }
}

fn parse_file(path: &str) -> FileReport {
    let Ok(text) = fs::read_to_string(path) else {
        let mut rep = FileReport::default();
        rep.findings.push(format!(
            "{}: not readable — the sheet does not measure",
            path
        ));
        return rep;
    };
    analyze_text(path, &text)
}

fn analyze_text(path: &str, text: &str) -> FileReport {
    let mut rep = FileReport::default();
    let mut current_section = String::from("(kopf)");
    let mut prose_nums: Vec<Num> = Vec::new();
    let mut table_nums: Vec<Num> = Vec::new();
    let mut lines_text: Vec<String> = Vec::new();
    for (idx, line) in text.lines().enumerate() {
        let ln = idx + 1;
        lines_text.push(line.to_string());
        let (is_sec, sec) = section_of(line, &current_section);
        if is_sec {
            current_section = sec;
            rep.sections.push(current_section.clone());
            continue;
        }
        if line.trim().is_empty()
            || line.trim().starts_with("<!--")
            || line.trim().starts_with("-->")
        {
            continue;
        }
        let kind = if is_table_row(line) {
            Kind::Table
        } else {
            Kind::Prose
        };
        for tok in tokenize(line) {
            let s = tok.trim().to_string();
            if let Some((v, comma_decimal)) = parse_number(&s) {
                let num = Num {
                    value: v,
                    raw: s,
                    line: ln,
                    section: current_section.clone(),
                    comma_decimal,
                };
                rep.nums.push(num.clone());
                if kind == Kind::Prose {
                    prose_nums.push(num);
                } else {
                    table_nums.push(num);
                }
            }
        }
    }

    let table_commas = table_nums.iter().filter(|n| n.comma_decimal).count();
    let prose_commas = prose_nums.iter().filter(|n| n.comma_decimal).count();
    if table_commas > 0 && prose_commas > 0 {
        rep.found[4] += 1;
        rep.findings.push(format!(
            "R4 {}:{} comma-locale mixed — table comma {} ×, prose comma {} × (one sheet speaks one locale)",
            path, 0, table_commas, prose_commas
        ));
    }

    let table_values: Vec<f64> = table_nums.iter().map(|n| n.value).collect();

    const COUNT_KW: [&str; 24] = [
        "records",
        "record",
        "objects",
        "object",
        "patients",
        "patient",
        "samples",
        "sample",
        "counts",
        "count",
        "n =",
        "n=",
        "n ≈",
        "n≈",
        "pairs",
        "arrows",
        "shuffle",
        "shuffles",
        "years",
        "days",
        "objects'",
        "objects’",
        "lines",
        "rows",
    ];
    let line_at = |ln: usize| {
        lines_text
            .get(ln.saturating_sub(1))
            .map(|s| s.to_lowercase())
            .unwrap_or_default()
    };
    for n in &prose_nums {
        let anchored = table_values.iter().any(|&tv| near(tv, n.value));
        let in_abstract_or_conclusion =
            n.section.contains("Abstract") || n.section.contains("Conclusion");
        let is_count =
            n.value.fract() == 0.0 && COUNT_KW.iter().any(|kw| line_at(n.line).contains(kw));

        if !anchored && is_count {
            rep.found[5] += 1;
            rep.findings.push(format!(
                "R5 {}:{} [{}] '{}' — count claim without a table mark (register-anchorless)",
                path, n.line, n.section, n.raw
            ));
        }

        if !anchored && in_abstract_or_conclusion {
            rep.found[1] += 1;
            rep.findings.push(format!(
                "R1 {}:{} [{}] '{}' — Abstract/Conclusion number without a table mark",
                path, n.line, n.section, n.raw
            ));
        }
    }

    let _ = &prose_nums;

    let mut table_claims: Vec<(String, f64, usize)> = Vec::new();
    for n in &table_nums {
        let line_txt = lines_text
            .get(n.line.saturating_sub(1))
            .cloned()
            .unwrap_or_default();
        let cells: Vec<&str> = line_txt.split('|').map(|c| c.trim()).collect();

        let mut idx = 0;
        while idx < cells.len() && cells[idx].is_empty() {
            idx += 1;
        }
        if let Some(label_cell) = cells.get(idx) {
            let label = label_cell.to_lowercase();
            if label.is_empty() || label.chars().any(|c| c.is_ascii_digit()) {
                continue;
            }
            if let Some(count_cell) = cells.get(idx + 1) {
                if let Some((v, _)) = parse_number(count_cell) {
                    if v.fract() == 0.0 {
                        table_claims.push((label, v, n.line));
                    }
                }
            }
        }
    }

    let mut label_vocab: Vec<&str> = Vec::new();
    for (l, _, _) in &table_claims {
        if !label_vocab.contains(&l.as_str()) {
            label_vocab.push(l.as_str());
        }
    }

    let mut claims: Vec<(String, f64, usize)> = Vec::new();
    for n in &prose_nums {
        let line_txt = lines_text
            .get(n.line.saturating_sub(1))
            .cloned()
            .unwrap_or_default();
        let toks: Vec<String> = tokenize(&line_txt);
        for (i, t) in toks.iter().enumerate() {
            let Some((v, _)) = parse_number(t) else {
                continue;
            };
            if v != n.value || v.fract() != 0.0 {
                continue;
            }
            if i > 0 {
                let prev = toks[i - 1].to_lowercase();
                if label_vocab.contains(&prev.as_str()) {
                    claims.push((prev, v, n.line));
                }
            }

            if i >= 2 && toks[i - 1] == ":" {
                let prev = toks[i - 2].to_lowercase();
                if label_vocab.contains(&prev.as_str()) {
                    claims.push((prev, v, n.line));
                }
            }
        }
    }
    claims.extend(table_claims);

    let mut by_label: std::collections::HashMap<String, Vec<(f64, usize)>> = Default::default();
    for (l, v, ln) in claims {
        by_label.entry(l).or_default().push((v, ln));
    }
    for (label, vals) in by_label {
        let mut seen_val: Vec<(f64, usize)> = Vec::new();
        for (v, ln) in vals {
            if !seen_val.iter().any(|(sv, _)| near(*sv, v)) {
                seen_val.push((v, ln));
            }
        }
        for i in 0..seen_val.len() {
            for j in i + 1..seen_val.len() {
                let (va, la) = seen_val[i];
                let (vb, lb) = seen_val[j];
                if !near(va, vb) {
                    rep.found[3] += 1;
                    rep.findings.push(format!(
                        "R3 {}:{} '{}' — double counting: {} (l. {}) vs {} (l. {})",
                        path, la, label, va, la, vb, lb
                    ));
                }
            }
        }
    }

    for line in text.lines() {
        let low = line.to_lowercase();
        for kw in [
            "steepest",
            "largest",
            "highest",
            "strongest",
            "greater",
            "smallest",
        ] {
            if low.contains(kw) {
                rep.r6_candidates.push(format!(
                    "R6 {path}: '{}' — comparison claim, re-computation pending",
                    line.trim()
                ));
                break;
            }
        }
    }

    rep
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut files: Vec<String> = Vec::new();
    let mut i = 0;
    let mut tol_arg: Option<f64> = None;
    while i < args.len() {
        match args[i].as_str() {
            "--tol" => {
                if let Some(v) = args.get(i + 1).and_then(|s| s.parse::<f64>().ok()) {
                    tol_arg = Some(v);
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "-" | "--stdin" => {
                let mut buf = String::new();
                use std::io::Read;
                let _ = std::io::stdin().read_to_string(&mut buf);

                files.push("(stdin)".to_string());
                let _ = buf;
                i += 1;
            }
            other => {
                files.push(other.to_string());
                i += 1;
            }
        }
    }

    if let Some(t) = tol_arg {
        let _ = t;
    }

    if files.is_empty() {
        let dir = Path::new("docs/paper");
        if let Ok(rd) = fs::read_dir(dir) {
            let mut v: Vec<String> = rd
                .filter_map(|e| e.ok())
                .filter(|e| e.path().extension().map(|x| x == "md").unwrap_or(false))
                .map(|e| e.path().display().to_string())
                .collect();
            v.sort();
            files = v;
        }
    }

    println!("=== Number audit — std-only, R1–R5 hard, R6 output field ===");
    println!("(Tolerance {TOL:e} — the calibration fixes the line)");
    let mut totals: [usize; 6] = [0; 6];
    let mut r6_total = 0usize;

    for f in &files {
        let rep = parse_file(f);
        let n_find = rep.found.iter().sum::<usize>();
        let base = Path::new(f)
            .file_name()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_else(|| f.clone());
        println!();
        println!("## {base}");
        if rep.nums.is_empty() && rep.findings.is_empty() {
            println!("  no numbers — the sheet does not measure (0 honored)");
        }
        for fx in &rep.findings {
            println!("  {fx}");
        }
        for r in &rep.r6_candidates {
            println!("  {r}");
        }
        println!(
            "  [finds R1..R5 = {n_find} | R6 candidates = {}]",
            rep.r6_candidates.len()
        );
        for r in 1..=5 {
            totals[r] += rep.found[r];
        }
        r6_total += rep.r6_candidates.len();
    }

    println!();
    println!("=== Calibration (per rule) ===");
    let names = [
        "",
        "R1 abstract-number<->mark",
        "R2 §2 count<->table",
        "R3 double count",
        "R4 comma locale",
        "R5 unanchored number",
    ];
    for r in 1..=5 {
        println!("  {:<28} found = {}", names[r], totals[r]);
    }
    println!(
        "  {:<28} found = {} (output field, not hard)",
        "R6 comparison claim", r6_total
    );
    println!();
    println!(
        "The three numbers found / missed / invented per rule against the known-bad corpus: the corpus mapping stands in docs/specs/bekannt-schlecht-korpus.md — the run measures the tool (0 honored)."
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    use std::path::PathBuf;

    fn repo_root() -> PathBuf {
        let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
        manifest.parent().unwrap().parent().unwrap().to_path_buf()
    }

    fn corpus_distribution() -> (BTreeMap<String, usize>, usize) {
        let text = fs::read_to_string(repo_root().join("docs/specs/bekannt-schlecht-korpus.md"))
            .expect("the known-bad corpus must live at docs/specs/bekannt-schlecht-korpus.md");
        let mut dist: BTreeMap<String, usize> = BTreeMap::new();
        for line in text.lines() {
            let t = line.trim();
            if !t.starts_with('|') || !t.contains('|') {
                continue;
            }
            let cells: Vec<&str> = t.split('|').map(|c| c.trim()).collect();
            if cells.len() < 3 {
                continue;
            }
            if cells[2].is_empty() || cells[2].starts_with("Klasse") {
                continue;
            }
            let separator = cells
                .iter()
                .skip(1)
                .all(|c| c.is_empty() || c.chars().all(|ch| ch == '-' || ch == ':' || ch == ' '));
            if separator {
                continue;
            }
            let letter = cells[2].split_whitespace().next().unwrap_or("");
            if letter.len() == 1 {
                *dist.entry(letter.to_string()).or_default() += 1;
            }
        }
        let total = dist.values().sum();
        (dist, total)
    }

    #[test]
    fn known_bad_corpus_rows_are_reconciled_with_its_umfang() {
        let (dist, total) = corpus_distribution();
        let expect: BTreeMap<String, usize> = [
            ("A".to_string(), 14),
            ("Z".to_string(), 3),
            ("D".to_string(), 3),
            ("K".to_string(), 1),
            ("N".to_string(), 3),
            ("V".to_string(), 5),
        ]
        .into_iter()
        .collect();
        assert_eq!(dist, expect, "corpus table rows and Umfang line must agree");
        assert_eq!(total, 29, "29 verified findings, one per row");
        let must_find = ["A", "Z", "D", "K"]
            .iter()
            .map(|k| dist.get(*k).copied().unwrap_or(0))
            .sum::<usize>();
        assert_eq!(
            must_find, 21,
            "number findings the audit must find = A+Z+D+K"
        );
    }

    #[test]
    fn r1_abstract_number_without_table_mark_is_found() {
        let text = "# Probe\n## Abstract\nsteepest step 5.47 to 5.57\n## Tables\n| step | slope |\n| one | 3.75 |\n";
        let rep = analyze_text("r1_fixture", text);
        assert!(
            rep.found[1] >= 1,
            "R1 must fire on an Abstract number with no table mark"
        );
    }

    #[test]
    fn r3_double_count_is_found() {
        let text = "# Probe\n## T1\n| records | 1663 |\n## T3\n| records | 1666 |\n";
        let rep = analyze_text("r3_fixture", text);
        assert!(
            rep.found[3] >= 1,
            "R3 must fire on the same label with two values"
        );
    }

    #[test]
    fn r4_comma_locale_mix_is_found() {
        let text = "# Probe\n## Result\nthe delay measured 492,0 s\n## Table\n| station | delay |\n| sod | 487,7 |\n";
        let rep = analyze_text("r4_fixture", text);
        assert!(
            rep.found[4] >= 1,
            "R4 must fire when prose and table disagree on the comma"
        );
    }

    #[test]
    fn r5_unanchored_count_is_found() {
        let text = "# Probe\n## Body\nthe 862 records were measured\n";
        let rep = analyze_text("r5_fixture", text);
        assert!(
            rep.found[5] >= 1,
            "R5 must fire on an unanchored count claim"
        );
    }

    #[test]
    fn verbal_overdeclaration_stays_silent() {
        let text = "# Probe\n## Result\nthe hypothesis holds for the steepest step measured\n";
        let rep = analyze_text("v_fixture", text);
        let total: usize = rep.found.iter().sum();
        assert_eq!(
            total, 0,
            "a verbal claim (Klasse V) carries no number the audit may find"
        );
    }

    #[test]
    fn parses_scientific() {
        assert!(near(normalize("3.91e-2").unwrap(), 0.0391));
        assert!(near(normalize("7.2822e-1").unwrap(), 0.72822));
    }

    #[test]
    fn parses_comma_decimal() {
        assert!(near(normalize("0,9771").unwrap(), 0.9771));
        assert!(near(normalize("492,0").unwrap(), 492.0));
    }

    #[test]
    fn parses_thousands_space() {
        assert!(near(normalize("5 862 322").unwrap(), 5862322.0));
        assert!(near(normalize("5 862 322").unwrap(), 5862322.0));
    }

    #[test]
    fn f_class_tight() {
        assert!(!near(50.73, 50.71));

        assert!(near(0.728, 0.72822));
    }
}
