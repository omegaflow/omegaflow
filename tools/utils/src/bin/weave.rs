use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn split_line(line: &str, delim: char) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut quoted = false;
    for ch in line.chars() {
        match ch {
            '"' => quoted = !quoted,
            c if c == delim && !quoted => {
                out.push(cur.trim().to_string());
                cur.clear();
            }
            c => cur.push(c),
        }
    }
    out.push(cur.trim().to_string());
    out
}

fn norm(h: &str) -> String {
    let lower = h.to_lowercase();
    let base = match (lower.find('['), lower.find('(')) {
        (Some(i), Some(j)) => &lower[..i.min(j)],
        (Some(i), None) => &lower[..i],
        (None, Some(j)) => &lower[..j],
        (None, None) => &lower[..],
    };
    base.chars()
        .filter(|c| c.is_alphanumeric() || *c == '_')
        .collect()
}

fn admitted(token: &str) -> bool {
    !token.is_empty()
        && !matches!(
            token,
            "no-license-stated"
                | "all-rights-reserved"
                | "auth-required"
                | "registration-required"
                | "no-license"
                | "no-redistribution"
                | "unclear"
                | "unknown"
        )
        && !token.ends_with("-permission-required")
}

struct Table {
    file: String,
    header: Vec<String>,
    rows: Vec<Vec<String>>,
    anchor: usize,
    terms: String,
}

fn read_table(path: &str, anchor: &str, terms: &BTreeMap<String, String>) -> Result<Table, String> {
    let text = fs::read_to_string(path).map_err(|e| format!("{path}: {e}"))?;
    let mut lines = text
        .lines()
        .filter(|l| !l.trim().is_empty() && !l.trim_start().starts_with('#'));
    let first = lines.next().ok_or_else(|| format!("{path}: leer"))?;
    let delim = if first.contains('\t') {
        '\t'
    } else if first.contains(';') {
        ';'
    } else {
        ','
    };
    let header = split_line(first, delim);
    let want = norm(anchor);
    let at = header.iter().position(|h| norm(h) == want).ok_or_else(|| {
        format!(
            "{path}: kein Anker '{anchor}' im Kopf ({})",
            header.join(",")
        )
    })?;
    let mut rows = Vec::new();
    for line in lines {
        let row = split_line(line, delim);
        if row.len() != header.len() {
            continue;
        }
        rows.push(row);
    }
    let term = match terms.get(path) {
        Some(t) => t.clone(),
        None => "unset".to_string(),
    };
    Ok(Table {
        file: path.to_string(),
        header,
        rows,
        anchor: at,
        terms: term,
    })
}

fn csv_field(v: &str) -> String {
    if v.contains(',') || v.contains('"') || v.contains('\n') {
        format!("\"{}\"", v.replace('"', "\"\""))
    } else {
        v.to_string()
    }
}

fn run(args: &[String]) -> Result<(), String> {
    let anchor = arg_value(args, "--on").ok_or("--on <spalte> fehlt")?;
    let out = arg_value(args, "--out").ok_or("--out <datei> fehlt")?;
    let terms_spec = arg_value(args, "--terms");
    let files: Vec<String> = args
        .iter()
        .enumerate()
        .filter(|(i, a)| {
            !a.starts_with("--")
                && args
                    .get(i.wrapping_sub(1))
                    .map(|p| p != "--on" && p != "--out" && p != "--terms")
                    != Some(false)
        })
        .map(|(_, a)| a.clone())
        .collect();
    if files.len() < 2 {
        return Err("weave braucht mindestens zwei Tabellen".to_string());
    }

    let mut terms: BTreeMap<String, String> = BTreeMap::new();
    if let Some(spec) = &terms_spec {
        for pair in spec.split(',').filter(|p| !p.trim().is_empty()) {
            let Some((f, t)) = pair.split_once('=') else {
                return Err(format!("--terms: '{pair}' ist nicht datei=token"));
            };
            terms.insert(f.trim().to_string(), t.trim().to_string());
        }
    }

    let mut tables: Vec<Table> = Vec::new();
    let mut excluded: Vec<(String, String)> = Vec::new();
    for f in &files {
        let t = read_table(f, &anchor, &terms)?;
        if terms_spec.is_some() && !admitted(&t.terms) {
            excluded.push((t.file.clone(), t.terms.clone()));
        }
        tables.push(t);
    }
    if !excluded.is_empty() {
        for (f, t) in &excluded {
            eprintln!(
                "weave: WARNUNG {f} — terms '{t}' ist nicht admitiert (gemergt, aber markiert)"
            );
        }
    }
    if tables.len() < 2 {
        return Err("weave braucht mindestens zwei Tabellen".to_string());
    }

    let mut cols: Vec<(String, String, String, String)> = Vec::new();
    for t in &tables {
        let stem = match Path::new(&t.file).file_stem() {
            Some(s) => s.to_string_lossy().to_string(),
            None => t.file.clone(),
        };
        for (i, h) in t.header.iter().enumerate() {
            if i == t.anchor {
                continue;
            }
            let mut name = h.clone();
            if cols.iter().any(|(n, _, _, _)| n == &name) {
                name = format!("{h}@{stem}");
            }
            let mut k = 1;
            while cols.iter().any(|(n, _, _, _)| n == &name) {
                name = format!("{h}@{stem}#{k}");
                k += 1;
            }
            cols.push((name, t.file.clone(), h.clone(), norm(h)));
        }
    }

    let mut rows: BTreeMap<String, Vec<Option<String>>> = BTreeMap::new();
    let mut order: Vec<String> = Vec::new();
    for t in &tables {
        for r in &t.rows {
            let key = r[t.anchor].clone();
            if !rows.contains_key(&key) {
                rows.insert(key.clone(), vec![None; cols.len()]);
                order.push(key.clone());
            }
            let slot = rows.get_mut(&key).expect("key present");
            let mut ci = 0;
            for (i, h) in t.header.iter().enumerate() {
                if i == t.anchor {
                    continue;
                }
                while ci < cols.len() && !(cols[ci].1 == t.file && cols[ci].2 == *h) {
                    ci += 1;
                }
                if ci >= cols.len() {
                    break;
                }
                slot[ci] = Some(r[i].clone());
                ci += 1;
            }
        }
    }

    let mut groups: BTreeMap<String, Vec<usize>> = BTreeMap::new();
    for (i, c) in cols.iter().enumerate() {
        groups.entry(c.3.clone()).or_default().push(i);
    }

    let mut text = String::new();
    let head: Vec<String> = std::iter::once(anchor.clone())
        .chain(cols.iter().map(|(n, _, _, _)| n.clone()))
        .chain(std::iter::once("__conflict".to_string()))
        .collect();
    text.push_str(
        &head
            .iter()
            .map(|h| csv_field(h))
            .collect::<Vec<_>>()
            .join(","),
    );
    text.push('\n');
    let mut conflict_rows = 0usize;
    for key in &order {
        let values = rows.get(key).expect("row present");
        let mut notes: Vec<String> = Vec::new();
        for (norm_name, idxs) in &groups {
            if idxs.len() < 2 {
                continue;
            }
            let distinct: BTreeSet<&str> = idxs
                .iter()
                .filter_map(|i| values[*i].as_deref())
                .filter(|v| !v.is_empty())
                .collect();
            if distinct.len() > 1 {
                let names: Vec<&str> = idxs.iter().map(|i| cols[*i].0.as_str()).collect();
                notes.push(format!("{}:[{}]", norm_name, names.join("|")));
            }
        }
        if !notes.is_empty() {
            conflict_rows += 1;
        }
        let mut fields = vec![csv_field(key)];
        for v in values {
            fields.push(csv_field(v.as_deref().unwrap_or("")));
        }
        fields.push(csv_field(&notes.join(";")));
        text.push_str(&fields.join(","));
        text.push('\n');
    }
    fs::write(&out, text.as_bytes()).map_err(|e| format!("{out}: {e}"))?;

    let mut map = String::from("out_column\tsource_file\tsource_column\tterms\n");
    for (n, f, c, _) in &cols {
        let t = tables
            .iter()
            .find(|t| &t.file == f)
            .map(|t| t.terms.as_str())
            .unwrap_or("unset");
        map.push_str(&format!("{n}\t{f}\t{c}\t{t}\n"));
    }
    fs::write(format!("{out}.map"), map.as_bytes()).map_err(|e| format!("{out}.map: {e}"))?;

    let mut edges = String::from("from\tto\trelation\n");
    for i in 0..cols.len() {
        for j in (i + 1)..cols.len() {
            if cols[i].3 != cols[j].3 || cols[i].1 == cols[j].1 {
                continue;
            }
            let mut seen = 0usize;
            let mut same = true;
            for values in rows.values() {
                match (values[i].as_deref(), values[j].as_deref()) {
                    (Some(a), Some(b)) if !a.is_empty() && !b.is_empty() => {
                        seen += 1;
                        if a != b {
                            same = false;
                            break;
                        }
                    }
                    _ => {}
                }
            }
            if same && seen > 0 {
                edges.push_str(&format!(
                    "{}#{}\tmirror-of\t{}#{}\n",
                    cols[i].1, cols[i].2, cols[j].1, cols[j].2
                ));
            }
        }
    }
    for (n, f, c, _) in &cols {
        edges.push_str(&format!("{f}#{c}\tmerge:{out}\tcontributes\n"));
        edges.push_str(&format!("merge:{out}\t{n}\tmaterializes\n"));
    }
    fs::write(format!("{out}.edges"), edges.as_bytes()).map_err(|e| format!("{out}.edges: {e}"))?;

    println!(
        "weave: {} Quellen, {} Zeilen, {} Spalten + Anker -> {out} ({} Konfliktzeilen; Karte {out}.map, Kanten {out}.edges)",
        tables.len(),
        order.len(),
        cols.len(),
        conflict_rows
    );
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("weave: {msg}");
        std::process::exit(2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn norm_strips_units_and_case() {
        assert_eq!(norm("Discharge [m3/s]"), "discharge");
        assert_eq!(norm("Discharge (m3/s)"), "discharge");
        assert_eq!(norm(" station_id "), "station_id");
    }

    #[test]
    fn admitted_refuses_no_redistribution() {
        assert!(admitted("CC0-1.0"));
        assert!(!admitted("no-redistribution"));
        assert!(!admitted("auth-required"));
    }

    #[test]
    fn weave_marks_conflicts_instead_of_overwriting() {
        let dir = std::env::temp_dir().join(format!("weave-c-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        let a = dir.join("a.csv");
        let b = dir.join("b.csv");
        fs::write(&a, "time,flow\n1,5\n2,6\n").unwrap();
        fs::write(&b, "time,flow\n1,7\n3,8\n").unwrap();
        let out = dir.join("out.csv");
        let args = vec![
            "--on".to_string(),
            "time".to_string(),
            "--out".to_string(),
            out.to_string_lossy().to_string(),
            a.to_string_lossy().to_string(),
            b.to_string_lossy().to_string(),
        ];
        run(&args).unwrap();
        let text = fs::read_to_string(&out).unwrap();
        let mut lines = text.lines();
        assert!(lines.next().unwrap().contains("__conflict"));
        let row1 = lines.next().unwrap();
        assert!(row1.contains("1"), "{row1}");
        assert!(row1.contains("flow:"), "{row1}");
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn weave_marks_non_admitted_terms_but_merges() {
        let dir = std::env::temp_dir().join(format!("weave-t-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        let a = dir.join("a.csv");
        let b = dir.join("b.csv");
        fs::write(&a, "time,v\n1,5\n").unwrap();
        fs::write(&b, "time,v\n1,6\n").unwrap();
        let out = dir.join("out.csv");
        let spec = format!("{}=CC0-1.0,{}=no-redistribution", a.display(), b.display());
        let args = vec![
            "--on".to_string(),
            "time".to_string(),
            "--out".to_string(),
            out.to_string_lossy().to_string(),
            "--terms".to_string(),
            spec,
            a.to_string_lossy().to_string(),
            b.to_string_lossy().to_string(),
        ];
        run(&args).unwrap();
        let map = fs::read_to_string(format!("{}.map", out.display())).unwrap();
        assert!(map.contains("no-redistribution"), "{map}");
        assert!(map.contains("CC0-1.0"), "{map}");
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn weave_marks_identical_columns_as_mirror() {
        let dir = std::env::temp_dir().join(format!("weave-m-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        let a = dir.join("a.csv");
        let b = dir.join("b.csv");
        fs::write(&a, "time,flow\n1,5\n2,6\n").unwrap();
        fs::write(&b, "time,flow\n1,5\n2,6\n").unwrap();
        let out = dir.join("out.csv");
        let args = vec![
            "--on".to_string(),
            "time".to_string(),
            "--out".to_string(),
            out.to_string_lossy().to_string(),
            a.to_string_lossy().to_string(),
            b.to_string_lossy().to_string(),
        ];
        run(&args).unwrap();
        let edges = fs::read_to_string(format!("{}.edges", out.display())).unwrap();
        assert!(edges.contains("mirror-of"), "{edges}");
        let _ = fs::remove_dir_all(&dir);
    }
}
