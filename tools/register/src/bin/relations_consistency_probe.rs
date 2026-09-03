use std::env;
use std::fs;
use std::path::Path;

const TOL: f64 = 3.5e-4;

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

type Column = Vec<f64>;

fn table_columns(text: &str) -> Vec<Vec<Column>> {
    let lines: Vec<&str> = text.lines().collect();
    let mut tables: Vec<Vec<Column>> = Vec::new();
    let mut i = 0;
    while i < lines.len() {
        if !is_table_row(lines[i]) {
            i += 1;
            continue;
        }
        let mut rows: Vec<Vec<String>> = Vec::new();
        while i < lines.len() && is_table_row(lines[i]) {
            let cells: Vec<String> = lines[i].split('|').map(|c| c.trim().to_string()).collect();
            rows.push(cells);
            i += 1;
        }
        let ncols = rows.iter().map(|r| r.len()).max().unwrap_or(0);
        let mut cols: Vec<Column> = Vec::new();
        for j in 0..ncols {
            let mut col: Column = Vec::new();
            for r in &rows {
                if let Some(cell) = r.get(j) {
                    if let Some((v, _)) = parse_number(cell) {
                        col.push(v);
                    }
                }
            }
            cols.push(col);
        }
        tables.push(cols);
    }
    tables
}

fn line_values(line: &str) -> Vec<f64> {
    let mut out = Vec::new();
    for tok in tokenize(line) {
        if let Some((v, _)) = parse_number(&tok) {
            out.push(v);
        }
    }
    out
}

fn check_steepest(
    path: &str,
    line_no: usize,
    line: &str,
    next: Option<&str>,
    tables: &[Vec<Column>],
) -> Option<String> {
    let low = line.to_lowercase();
    let is_step_claim = ["steepest", "steep"].iter().any(|k| low.contains(k));
    if !is_step_claim {
        return None;
    }

    let mut vals = line_values(line);
    if let Some(n) = next {
        vals.extend(line_values(n));
    }
    if vals.len() < 2 {
        return None;
    }

    let mut best: Option<(usize, usize, &Column, usize)> = None;
    for (ti, t) in tables.iter().enumerate() {
        for (ci, col) in t.iter().enumerate() {
            let hits = vals
                .iter()
                .filter(|v| col.iter().any(|cv| near(*cv, **v)))
                .count();
            if hits >= 2 && best.map(|b| hits > b.3).unwrap_or(true) {
                best = Some((ti, ci, col, hits));
            }
        }
    }
    let (ti, ci, col, _hits) = best?;

    let mut max_step: Option<f64> = None;
    for w in col.windows(2) {
        let d = (w[1] - w[0]).abs();
        max_step = Some(max_step.map(|m| m.max(d)).unwrap_or(d));
    }
    let m = max_step?;
    let mut findings: Vec<String> = Vec::new();

    for pair in vals.windows(2) {
        let (a, b) = (pair[0], pair[1]);
        let Some(ra) = col.iter().position(|cv| near(*cv, a)) else {
            continue;
        };
        let Some(rb) = col.iter().position(|cv| near(*cv, b)) else {
            continue;
        };
        if ra.abs_diff(rb) != 1 {
            continue;
        }
        let step = (col[rb] - col[ra]).abs();
        if !near(step, m) {
            findings.push(format!(
                "{path}:{line_no} C-steepest — '{a} → {b}' (Δ={step}) as steepest step, table-max Δ={m} (table {ti}, column {ci})",
                path = path, line_no = line_no, a = a, b = b, step = step, m = m, ti = ti + 1, ci = ci + 1
            ));
        }
    }
    if findings.is_empty() {
        None
    } else {
        Some(findings.join("\n"))
    }
}

fn process(path: &str) -> Vec<String> {
    let mut out = Vec::new();
    let Ok(text) = fs::read_to_string(path) else {
        out.push(format!(
            "{path}: not readable — the sheet does not measure (0 honored)"
        ));
        return out;
    };
    let tables = table_columns(&text);
    let lines: Vec<&str> = text.lines().collect();
    for (idx, line) in lines.iter().enumerate() {
        let ln = idx + 1;
        let next = lines.get(idx + 1).copied();
        if let Some(f) = check_steepest(path, ln, line, next, &tables) {
            out.push(f);
        }
    }
    out
}

fn main() {
    let mut files: Vec<String> = env::args().skip(1).collect();
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
    println!("=== relations-driver — C-steepest (one relation class, one tolerance rule) ===");
    println!("(Toleranz {TOL:e} — dieselbe Kalibrationslinie wie das Nummern-Audit)");
    let mut total = 0usize;
    for f in &files {
        let findings = process(f);
        let base = Path::new(f)
            .file_name()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_else(|| f.clone());
        println!("\n## {base}");
        if findings.is_empty() {
            println!("  no steepest-step contradictions (0 honored)");
        }
        for fx in &findings {
            println!("  {fx}");
        }
        total += findings.len();
    }
    println!("\n[C-steepest contradictions total = {total}]");
    println!(
        "Second relations kind (value extremum, e.g. 'max = V') is explicitly outside this first class — separate measurement in a next run (0 honored)."
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_scientific() {
        assert!(near(parse_number("5.11e-1").unwrap().0, 0.511));
        assert!(near(parse_number("3.75e-2").unwrap().0, 0.0375));
    }

    #[test]
    fn corona_steepest_is_a_contradiction() {
        let col: Column = vec![
            4.16, 4.70, 4.84, 5.47, 5.57, 5.81, 6.13, 6.27, 6.30, 6.43, 6.81,
        ];
        let tables = vec![vec![col]];
        let line = "steepest (4.84 → 5.47 → 5.57 in log T)";
        let f = check_steepest("corona.md", 233, line, None, &tables);
        assert!(
            f.is_some(),
            "steepest claim must be a recomputation contradiction"
        );
        assert!(
            f.unwrap().contains("5.47 → 5.57"),
            "the failing step is the small one"
        );
    }

    #[test]
    fn single_max_step_is_not_a_contradiction() {
        let col: Column = vec![4.16, 4.70, 4.84, 5.47, 5.57, 5.81];
        let tables = vec![vec![col]];

        let f = check_steepest("clean.md", 1, "steepest (4.84 → 5.47)", None, &tables);
        assert!(
            f.is_none(),
            "the actual max step alone is not a contradiction"
        );
    }

    #[test]
    fn unanchored_claim_is_not_fired() {
        let tables: Vec<Vec<Column>> = Vec::new();
        let f = check_steepest("clean.md", 1, "steepest (0.1 → 0.2)", None, &tables);
        assert!(f.is_none(), "no table anchor → non-recomputable, not fired");
    }
}
