use std::collections::BTreeMap;
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

struct Table {
    file: String,
    header: Vec<String>,
    rows: Vec<Vec<String>>,
    anchor: usize,
}

fn read_table(path: &str, anchor: &str) -> Result<Table, String> {
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
    let at = header
        .iter()
        .position(|h| norm(h) == want)
        .ok_or_else(|| format!("{path}: kein Anker '{anchor}' im Kopf ({})", header.join(",")))?;
    let mut rows = Vec::new();
    for line in lines {
        let row = split_line(line, delim);
        if row.len() != header.len() {
            continue;
        }
        rows.push(row);
    }
    Ok(Table {
        file: path.to_string(),
        header,
        rows,
        anchor: at,
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
    let files: Vec<String> = args
        .iter()
        .enumerate()
        .filter(|(i, a)| {
            !a.starts_with("--")
                && args.get(i.wrapping_sub(1)).map(|p| p != "--on" && p != "--out") != Some(false)
        })
        .map(|(_, a)| a.clone())
        .collect();
    if files.len() < 2 {
        return Err("weave braucht mindestens zwei Tabellen".to_string());
    }

    let tables: Vec<Table> = files
        .iter()
        .map(|f| read_table(f, &anchor))
        .collect::<Result<_, _>>()?;

    let mut cols: Vec<(String, String, String)> = Vec::new();
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
            if cols.iter().any(|(n, _, _)| n == &name) {
                name = format!("{h}@{stem}");
            }
            let mut k = 1;
            while cols.iter().any(|(n, _, _)| n == &name) {
                name = format!("{h}@{stem}#{k}");
                k += 1;
            }
            cols.push((name, t.file.clone(), h.clone()));
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

    let mut text = String::new();
    let head: Vec<String> = std::iter::once(anchor.clone())
        .chain(cols.iter().map(|(n, _, _)| n.clone()))
        .collect();
    text.push_str(&head.iter().map(|h| csv_field(h)).collect::<Vec<_>>().join(","));
    text.push('\n');
    for key in &order {
        let mut fields = vec![csv_field(key)];
        for v in rows.get(key).expect("row present") {
            fields.push(csv_field(v.as_deref().unwrap_or("")));
        }
        text.push_str(&fields.join(","));
        text.push('\n');
    }
    fs::write(&out, text.as_bytes()).map_err(|e| format!("{out}: {e}"))?;

    let mut map = String::from("out_column\tsource_file\tsource_column\n");
    for (n, f, c) in &cols {
        map.push_str(&format!("{n}\t{f}\t{c}\n"));
    }
    let map_path = format!("{out}.map");
    fs::write(&map_path, map.as_bytes()).map_err(|e| format!("{map_path}: {e}"))?;

    println!(
        "weave: {} Tabellen, {} Zeilen, {} Spalten + Anker -> {out} (Karte: {map_path})",
        tables.len(),
        order.len(),
        cols.len()
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
    fn split_handles_quotes() {
        let v = split_line("a,\"b,c\",d", ',');
        assert_eq!(v, vec!["a", "b,c", "d"]);
    }

    #[test]
    fn weave_merges_out_of_order_subsets() {
        let dir = std::env::temp_dir().join(format!("weave-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        let a = dir.join("a.csv");
        let b = dir.join("b.csv");
        fs::write(&a, "time,temp\n1,10\n2,11\n").unwrap();
        fs::write(&b, "flow,time\n5,1\n6,3\n").unwrap();
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
        assert_eq!(lines.next().unwrap(), "time,temp,flow");
        assert_eq!(lines.next().unwrap(), "1,10,5");
        assert_eq!(lines.next().unwrap(), "2,11,");
        assert_eq!(lines.next().unwrap(), "3,,6");
        let _ = fs::remove_dir_all(&dir);
    }
}
